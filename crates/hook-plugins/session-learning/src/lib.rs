//! session-learning — Stop WASM hook plugin.
//!
//! At session end (Stop event), appends a timestamped learning stub to
//! `.factory/sidecar-learning.md`. Ensures a marker exists so nothing gets
//! silently lost when sessions end abruptly.
//!
//! Behavioral contracts:
//!   BC-7.03.076 — identity & registry binding: Stop, priority=910, on_error=continue
//!   BC-7.03.077 — appends timestamped marker to .factory/sidecar-learning.md;
//!                 creates file with header if absent
//!   BC-7.03.078 — skips (exit 0) when .factory/ directory does not exist
//!
//! Architecture compliance rules (from S-8.06 story spec):
//!   - File I/O: write via std::fs (host::write_file absent per D-172).
//!     host::read_file IS available but unused — session-learning is append-only.
//!   - No host::emit_event calls (session-learning.sh never called emit_event;
//!     parity-only per E-8 D-2).
//!   - No exec_subprocess calls (binary_allow removed post-migration per AC-001).
//!   - Target: wasm32-wasip1 (NOT deprecated wasm32-wasi) per D-172 universal finding #1.
//!   - Stdin MUST be read to EOF and discarded to prevent WASI SIGPIPE-equivalent
//!     failures when dispatcher writes a large Stop envelope (EC-005).

use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

use vsdd_hook_sdk::HookResult;

/// The header written to a newly-created sidecar-learning.md.
/// The trailing blank line (two newlines after the last sentence) is REQUIRED —
/// bats compares byte-identical content against the bash source output (AC-002).
pub const SIDECAR_HEADER: &str =
    "# Sidecar Learning\n\nSession-end markers for the VSDD factory. Run /session-review to synthesize.\n\n";

/// The marker line format. The timestamp placeholder is filled at runtime.
/// Format: `- Session ended at <ISO8601-UTC> (awaiting /session-review)\n`
pub const MARKER_FORMAT: &str = "- Session ended at {} (awaiting /session-review)\n";

/// Format a UTC timestamp as `%Y-%m-%dT%H:%M:%SZ`.
///
/// Used by the hook to produce the append marker. Extracted as a pure
/// function so unit tests can verify the format without WASM runtime.
///
/// Implementation note: uses `chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()`
/// (seconds precision — no millis; matches bash `date -u +"%Y-%m-%dT%H:%M:%SZ"`).
pub fn format_utc_now() -> String {
    chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
}

/// Core hook logic for session-learning.
///
/// Preconditions / postconditions per BCs:
///   BC-7.03.078 precondition: if `.factory/` absent → return Continue immediately (AC-003)
///   BC-7.03.077 postcondition: create sidecar-learning.md with header if absent,
///                              then append marker line; exit 0 (AC-002, AC-004)
///   BC-7.03.076 postcondition: always exit 0; on_error=continue
///
/// The `now_fn` parameter is injectable for unit testing (avoids real clock).
/// The `fs_root` parameter is the base directory for resolving `.factory/`
/// (pass `""` or `"."` in production; override in tests with a temp dir).
///
/// File I/O: write via std::fs (host::write_file absent per D-172).
/// host::read_file IS available but unused — session-learning is append-only.
pub fn session_learning_logic(
    now_fn: impl Fn() -> String,
    fs_root: &str,
) -> HookResult {
    // Step 1: Stdin drain is performed by the dispatcher's __internal::run trampoline
    // before calling on_hook (the payload is passed in via HookPayload). In the unit-test
    // path (calling session_learning_logic directly), there is no stdin to drain.
    // EC-005 stdin drain is handled at the main.rs boundary via the SDK run() trampoline.

    // Step 2: Check if .factory/ directory exists; if not, return Continue immediately
    // (AC-003, BC-7.03.078 postcondition 1).
    let factory_dir = if fs_root.is_empty() || fs_root == "." {
        Path::new(".factory").to_path_buf()
    } else {
        Path::new(fs_root).join(".factory")
    };

    if !factory_dir.is_dir() {
        return HookResult::Continue;
    }

    // Step 3: Open sidecar-learning.md. If file does not exist, create it and write
    // the header block first (AC-002, byte-identical to bash source output).
    // File I/O via std::fs (host::write_file absent per D-172).
    let sidecar_path = factory_dir.join("sidecar-learning.md");

    if !sidecar_path.exists() {
        // Create file and write header (byte-identical to bash source output, AC-002).
        match std::fs::write(&sidecar_path, SIDECAR_HEADER) {
            Ok(_) => {}
            Err(e) => {
                return HookResult::error(format!(
                    "session-learning: failed to create sidecar-learning.md: {e}"
                ));
            }
        }
    }

    // Step 4: Append the marker line using OpenOptions::new().append(true) (EC-004).
    // append(true) positions the cursor at EOF; no full-file buffering needed.
    let ts = now_fn();
    let marker = MARKER_FORMAT.replace("{}", &ts);

    match OpenOptions::new().append(true).open(&sidecar_path) {
        Ok(mut file) => match file.write_all(marker.as_bytes()) {
            Ok(_) => HookResult::Continue,
            Err(e) => HookResult::error(format!(
                "session-learning: failed to append marker to sidecar-learning.md: {e}"
            )),
        },
        Err(e) => HookResult::error(format!(
            "session-learning: failed to open sidecar-learning.md for append: {e}"
        )),
    }
}
