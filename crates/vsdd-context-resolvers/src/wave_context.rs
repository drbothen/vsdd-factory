//! Wave-context YAML parsing types.
//!
//! # Schema alignment
//! `WaveEntry` fields match `update-wave-state-on-merge/src/lib.rs` exactly:
//!   - `wave: String` — wave identifier (e.g., "F4")
//!   - `stories: Vec<String>` — stories planned for this wave
//!   - `stories_merged: Vec<String>` — stories already merged
//!   - `gate_status: Option<String>` — None / "not_started" / "pending" / "passed" / "deferred" / "failed"
//!   - `current_wave: Option<serde_norway::Value>` — optional extra field (preserved)
//!   - `next_gate_required: Option<serde_norway::Value>` — optional extra field (preserved)
//!
//! # Active wave determination
//! An active wave is the LAST entry in `waves` whose `gate_status` is not a
//! terminal value (see `TERMINAL_STATES`). See `find_active_wave` for the full
//! terminal-state enumeration per BC-8.14.009.
//!
//! # No panic guarantee
//! AC-010 / BC-4.12.004 INV1: no fallible unwrap or panic-on-error calls anywhere
//! in this module's production source. All error paths return `None`.

use serde::Deserialize;

/// Represents a single wave entry in `.factory/wave-state.yaml`.
///
/// Matches the canonical schema produced by `update-wave-state-on-merge`.
/// All list fields use `#[serde(default)]` to tolerate partial YAML entries.
/// `gate_status` uses `Option<String>` with `#[serde(default)]` to handle the
/// gate_status four-case truth table (per S-8.04 producer semantics):
///   Case 1: key absent → None (serde default) → wave is NOT terminal
///   Case 2: key present, YAML null/~ → None → wave is NOT terminal
///   Case 3: key present, "not_started" / "pending" → Some("...") → NOT terminal
///   Case 4: key present, terminal value
///           ("passed" | "deferred" | "failed" | "completed") → wave is terminal
///
/// TODO(TD-074): This struct is a sibling of `WaveEntry` in
/// `update-wave-state-on-merge/src/lib.rs` and
/// `warn-pending-wave-gate/src/lib.rs`. Three independent definitions can
/// drift. Future work: hoist into a shared `vsdd-wave-state` crate.
/// Tracked as TD-074 in `.factory/tech-debt-register.md`.
#[derive(Debug, Clone, Deserialize)]
pub struct WaveEntry {
    /// Wave identifier (e.g., "F4", "wave-3").
    pub wave: String,
    /// Story IDs planned for this wave.
    #[serde(default)]
    pub stories: Vec<String>,
    /// Story IDs already merged in this wave.
    #[serde(default)]
    pub stories_merged: Vec<String>,
    /// Gate status — None means not yet set (wave is not in a terminal state per BC-8.14.009).
    #[serde(default)]
    pub gate_status: Option<String>,
    /// Optional extra field from producer (round-tripped to preserve unknown data).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_wave: Option<serde_norway::Value>,
    /// Optional extra field from producer (round-tripped to preserve unknown data).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_gate_required: Option<serde_norway::Value>,
}

/// Top-level `.factory/wave-state.yaml` structure.
///
/// Mirrors the canonical schema from `update-wave-state-on-merge/src/lib.rs`.
/// `Default` yields an empty waves list — used as the empty-waves fallback
/// when YAML parse fails in `resolve_impl`.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct WaveState {
    /// All waves in the pipeline, in order from earliest to latest.
    #[serde(default)]
    pub waves: Vec<WaveEntry>,
}

/// Parse a YAML string into a `WaveState`.
///
/// Returns `Err(serde_norway::Error)` on malformed YAML; callers map errors to
/// `ResolverOutput { value: None }` (AC-002, EC-003). Does NOT panic.
/// BC-4.12.004 INV1: no fallible unwrap or panic-on-error calls.
///
/// Note: serde_norway handles CRLF line endings natively per YAML 1.2 spec,
/// so no explicit normalization is needed here. (Contrast with
/// `parse_cycle_id_from_state_md`, which does manual frontmatter parsing
/// and therefore must normalize CRLF before splitting on `\n---`.)
pub fn parse_wave_state(yaml: &str) -> Result<WaveState, serde_norway::Error> {
    serde_norway::from_str(yaml)
}

/// Terminal gate-status values that mark a wave as no longer active.
///
/// Per BC-8.14.009 canonical `gate_status` enum:
///
/// - `"passed"` — canonical terminal value; gate approved.
/// - `"deferred"` — canonical "skip" value; wave intentionally not run.
/// - `"failed"` — canonical failed-gate value; wave will not proceed.
/// - `"completed"` — non-canonical legacy alias. Retained for defensive parsing
///   of pre-BC-8.14.009 test fixtures from `update-wave-state-on-merge`.
///
/// Any other value (including `None`, `"not_started"`, `"pending"`, `"in_progress"`)
/// means the wave is still active.
const TERMINAL_STATES: &[&str] = &["passed", "deferred", "failed", "completed"];

/// Determine the active wave from a list of `WaveEntry`.
///
/// A wave is "terminal" if `gate_status` matches one of the canonical
/// values per BC-8.14.009:
///
///   - `"passed"` — canonical terminal value; gate approved.
///   - `"deferred"` — canonical "skip" value; wave intentionally not run.
///   - `"failed"` — canonical failed-gate value.
///   - `"completed"` — non-canonical legacy alias. Retained for defensive parsing
///     of pre-BC-8.14.009 test fixtures from `update-wave-state-on-merge`.
///
/// Returns the FIRST wave in the list (from the end, i.e., `.rev()`) whose
/// status is NOT terminal, or `None` if all waves are terminal or the list
/// is empty. A `None` gate_status (key absent or YAML null) is non-terminal.
pub fn find_active_wave(wave_state: &WaveState) -> Option<&WaveEntry> {
    wave_state.waves.iter().rev().find(|w| {
        let status = w.gate_status.as_deref().unwrap_or("");
        !TERMINAL_STATES.contains(&status)
    })
}

/// Parse the `current_cycle:` value from `.factory/STATE.md` YAML frontmatter.
///
/// STATE.md uses YAML front-matter delimited by `---` markers. This function
/// extracts the frontmatter block and parses `current_cycle` from it.
/// Returns `None` if the file has no frontmatter, the frontmatter is malformed,
/// or the `current_cycle` key is absent.
///
/// BC-4.12.004 INV1: does NOT panic.
pub fn parse_cycle_id_from_state_md(state_md: &str) -> Option<String> {
    // STATE.md starts with a YAML frontmatter block delimited by `---` markers.
    // Format:
    //   ---
    //   document_type: pipeline-state
    //   ...
    //   current_cycle: v1.0-feature-engine-discipline-pass-1
    //   ...
    //   ---
    //
    // Normalize CRLF → LF before frontmatter splitting on `\n---` to handle
    // STATE.md files edited on Windows or cloned with core.autocrlf = true.
    // The `find("\n---")` closing-marker search and
    // `trim_start_matches(['\n', '\r'])` would otherwise miss CRLF frontmatter.
    let normalized;
    let state_md = if state_md.contains('\r') {
        normalized = state_md.replace("\r\n", "\n");
        normalized.as_str()
    } else {
        state_md
    };
    let trimmed = state_md.trim_start();
    if !trimmed.starts_with("---") {
        return None;
    }

    // Find the closing `---` marker after the opening one.
    let after_open = trimmed.get(3..)?;
    // Skip past any immediate newline after the opening `---`
    let content_start = after_open.trim_start_matches(['\n', '\r']);
    let end_pos = content_start.find("\n---")?;
    let frontmatter = &content_start[..end_pos];

    // Parse the frontmatter as YAML.
    #[derive(Deserialize)]
    struct Frontmatter {
        #[serde(default)]
        current_cycle: Option<String>,
    }

    let fm: Frontmatter = serde_norway::from_str(frontmatter).ok()?;
    fm.current_cycle
}
