//! Durable unvalidated-mutation marker: atomic write, idempotent delete, predicate.
//!
//! This module owns the `.factory/unvalidated-mutation.marker` file lifecycle
//! for the INDETERMINATE outcome class (S-25.01 Layer-1, ADR-047):
//!
//! - [`write_indeterminate_marker`]: atomically writes the marker via temp+rename.
//! - [`delete_marker_if_pass`]: idempotently deletes the marker on PASS re-validation.
//! - [`should_write_marker`]: pure predicate — `true` iff outcome=Indeterminate AND policy=FailClosed.
//!
//! # PostToolUse-only invariant (BC-1.18.001 invariant 4)
//!
//! The marker write is PostToolUse only. Callers are responsible for enforcing this
//! constraint — `write_indeterminate_marker` itself does not check event type.
//! EC-002: PreToolUse INDETERMINATE events produce advisory events but no marker.
//!
//! # Single-marker policy (BC-1.18.001 invariant 3)
//!
//! A second INDETERMINATE event overwrites the existing marker (last-writer-wins).
//! The atomic temp+rename write achieves this without a read-modify-write cycle.
//!
//! # BC-5.38.001 Red Gate discipline
//!
//! All non-trivial function bodies use `todo!()`. Implementer fills in real logic.

use std::io;
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};

use chrono::{DateTime, Utc};

use crate::executor::DispatchOutcome;
use crate::registry::FailurePolicy;

/// TTL for the `.factory/unvalidated-mutation.marker` deadman timer (ADR-048 §Decision 2).
///
/// 86 400 seconds = 24 hours. The marker's `expires_at` field is written once at
/// creation time as `timestamp + UNVALIDATED_MUTATION_MARKER_TTL_SECONDS`. The dispatcher
/// native crash-path check (`block_if_marker_check`) and the gate WASM plugin
/// (`evaluate_gate`) both treat an expired marker as absent (fail-open on stale
/// quarantine). No renewal mechanism; the quarantine must be explicitly resolved or
/// will auto-expire after 24 h.
///
/// BC-1.18.003 PC4: expired marker → Allow + auto-delete (idempotent, gate plugin).
pub const UNVALIDATED_MUTATION_MARKER_TTL_SECONDS: u64 = 86_400;

/// The six required TOML fields for `.factory/unvalidated-mutation.marker`.
///
/// BC-1.18.001 postcondition 4: all fields MUST be present.
/// `artifact_path` MUST be empty string (not omitted) when no artifact context.
/// `cause` MUST be one of `"fuel"`, `"epoch"`, `"output-too-large"` (BC-3.08.001 Event 8).
/// `expires_at` is new in ADR-048 §Decision 2; legacy markers without it are treated
/// conservatively (non-expired = block) by both native and WASM checks.
#[derive(Debug, Clone)]
pub struct MarkerFields {
    /// RFC 3339 timestamp of the INDETERMINATE event (e.g. "2026-08-30T12:00:00Z").
    pub timestamp: String,
    /// Registry name of the plugin that produced the INDETERMINATE outcome.
    pub plugin_name: String,
    /// Path of the artifact being validated when the INDETERMINATE occurred.
    /// MUST be empty string (not omitted) when no artifact context is available.
    /// BC-1.18.001 postcondition 4 + BC-3.08.001 Event 8: field is mandatory, empty string allowed.
    pub artifact_path: String,
    /// Cause of the INDETERMINATE outcome.
    /// MUST be exactly one of: `"fuel"`, `"epoch"`, `"output-too-large"`.
    /// Matches the `IndeterminateCause` enum string representation in the event wire format.
    pub cause: String,
    /// Dispatcher trace ID (`dispatcher_trace_id`) for the invocation.
    pub trace_id: String,
    /// RFC 3339 expiry timestamp = `timestamp` + `UNVALIDATED_MUTATION_MARKER_TTL_SECONDS`.
    /// ADR-048 §Decision 2: 24-hour deadman. The gate plugin auto-deletes the marker when
    /// this timestamp is reached. The native crash-path check (`block_if_marker_check`) treats
    /// an expired marker as absent (Allow). Missing/unparseable on legacy markers → non-expired.
    pub expires_at: String,
}

/// Atomically write the unvalidated-mutation marker file via write-to-temp + rename.
///
/// Algorithm (BC-1.18.001 postcondition 4 + invariant 3):
/// 1. Compute temp path: `<marker_path>.tmp` in the same directory as `marker_path`.
/// 2. Serialize the six required TOML fields to the temp file (O_CREAT | O_WRONLY | O_TRUNC).
/// 3. Atomically rename the temp file to `marker_path`.
///
/// Single-marker policy: if a marker already exists, the rename overwrites it (last-writer-wins).
/// Serialization uses the `toml` crate (Library table mandate; AC-005).
/// All control characters including `\n` are correctly escaped (MEDIUM-4 fix).
///
/// # Errors
///
/// Returns `io::Error` if the temp-file write or rename fails (EC-008).
/// The caller emits the `plugin.indeterminate` event regardless of whether this write succeeds.
pub fn write_indeterminate_marker(fields: &MarkerFields, marker_path: &Path) -> io::Result<()> {
    // Compute the temp path in the same directory (atomic rename invariant).
    // L-1 fix (S-25.01 adversary): use a unique suffix so concurrent writers from
    // same-tier plugins do not collide on a shared `.tmp` name (ENOENT race on rename).
    // Uniqueness: plugin_name + process_id + monotonic nonce → last-writer-wins still holds
    // because each writer renames ITS OWN temp file to the shared final path; the final
    // rename is still atomic (BC-1.18.001 INV3 preserved).
    static WRITE_NONCE: AtomicU64 = AtomicU64::new(0);
    let nonce = WRITE_NONCE.fetch_add(1, Ordering::Relaxed);
    let pid = std::process::id();
    // Sanitize plugin_name for filename safety: keep alphanumeric, hyphen, underscore.
    let safe_plugin: String = fields
        .plugin_name
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect();
    let file_name = marker_path
        .file_name()
        .and_then(|f| f.to_str())
        .unwrap_or("unvalidated-mutation.marker");
    let tmp_name = format!("{file_name}.{safe_plugin}.{pid}.{nonce}.tmp");
    let tmp_path = marker_path
        .parent()
        .unwrap_or_else(|| std::path::Path::new("."))
        .join(&tmp_name);

    // Serialize 6 mandatory TOML fields via the `toml` crate (Library table mandate).
    // toml::to_string correctly escapes all control characters including \n, \r, \t, etc.
    // Field insertion order preserved via BTreeMap (deterministic output).
    let mut table = toml::Table::new();
    table.insert(
        "timestamp".to_string(),
        toml::Value::String(fields.timestamp.clone()),
    );
    table.insert(
        "plugin_name".to_string(),
        toml::Value::String(fields.plugin_name.clone()),
    );
    table.insert(
        "artifact_path".to_string(),
        toml::Value::String(fields.artifact_path.clone()),
    );
    table.insert(
        "cause".to_string(),
        toml::Value::String(fields.cause.clone()),
    );
    table.insert(
        "trace_id".to_string(),
        toml::Value::String(fields.trace_id.clone()),
    );
    // ADR-048 §Decision 2: 24-hour deadman TTL field (BC-1.18.001 postcondition 4 + 5).
    table.insert(
        "expires_at".to_string(),
        toml::Value::String(fields.expires_at.clone()),
    );

    let content = toml::to_string(&table)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;

    // Write to temp file (O_CREAT | O_WRONLY | O_TRUNC semantics via fs::write).
    std::fs::write(&tmp_path, content.as_bytes())?;

    // Atomic rename temp → final path (single-marker policy: overwrites existing).
    std::fs::rename(&tmp_path, marker_path)?;

    Ok(())
}

/// Read the `plugin_name` field from the unvalidated-mutation marker file.
///
/// Returns:
/// - `Ok(Some(name))` if the file exists and the `plugin_name` field is parseable.
/// - `Ok(None)` if the file does not exist (NotFound → no marker).
/// - `Ok(None)` if the file exists but cannot be parsed (corrupt marker → treat conservatively).
///
/// Used by `execute_tier` and `spawn_async_plugin` to enforce BC-1.18.003 INV2:
/// only the named plugin's PASS clears the marker (scoped clear).
/// The caller compares the returned name with the passing plugin's name before
/// calling `delete_marker_if_pass`.
pub fn read_marker_plugin_name(marker_path: &Path) -> io::Result<Option<String>> {
    match std::fs::read_to_string(marker_path) {
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
        // I/O errors other than NotFound: propagate them — caller decides.
        Err(e) => Err(e),
        Ok(content) => {
            // Parse via toml crate; treat parse errors as "unknown" plugin name (conservative).
            let table: toml::Table = match toml::from_str(&content) {
                Ok(t) => t,
                Err(_) => return Ok(None),
            };
            let name = table
                .get("plugin_name")
                .and_then(|v| v.as_str())
                .map(str::to_string);
            Ok(name)
        }
    }
}

/// Idempotently delete the unvalidated-mutation marker file with artifact-scope enforcement.
///
/// # Artifact-scoped clear (M-1 fix, BC-1.18.003 PC1 + INV2)
///
/// A marker records the artifact path under validation when the INDETERMINATE event
/// occurred. Clearing must be scoped: if plugin P went INDETERMINATE while validating
/// artifact A, a subsequent PASS by P on artifact B must NOT discharge the quarantine —
/// the original mutation (artifact A) is still unvalidated.
///
/// Clear predicate:
/// - If the marker's `artifact_path` field is EMPTY — this was a non-artifact-scoped
///   validator; the marker is cleared unconditionally on a PASS (vacuously satisfied).
/// - If the marker's `artifact_path` == `current_artifact_path` (normalized absolute path
///   comparison via `std::path::Path`) — same artifact; clear the marker.
/// - Otherwise — different artifact; preserve the marker (return `Ok(())` without deleting).
///
/// # Idempotency (AC-013; BC-1.18.003 PC2)
///
/// If `marker_path` does not exist (`io::ErrorKind::NotFound`), returns `Ok(())` (no-op).
/// All other `io::Error` variants are propagated as `Err`.
///
/// # Conservative parse-error posture
///
/// If the marker file exists but cannot be parsed as TOML, this function returns `Ok(())`
/// WITHOUT deleting the file — quarantine is preserved. The normal call path already checks
/// `read_marker_plugin_name` before calling this, so a corrupt marker prevents the call.
pub fn delete_marker_if_pass(marker_path: &Path, current_artifact_path: &str) -> io::Result<()> {
    // Read the marker to obtain its artifact_path for the scoped-clear predicate.
    let marker_artifact: String = match std::fs::read_to_string(marker_path) {
        // NotFound: no marker exists — idempotent Ok(()) (AC-013).
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(()),
        // Other I/O errors: propagate.
        Err(e) => return Err(e),
        Ok(content) => {
            match toml::from_str::<toml::Table>(&content) {
                Ok(table) => table
                    .get("artifact_path")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                // Corrupt marker: conservative posture — preserve quarantine, do NOT delete.
                Err(_) => return Ok(()),
            }
        }
    };

    // M-1 clear predicate: delete IFF marker's artifact_path is EMPTY (non-artifact
    // validator, vacuously satisfied) OR marker's artifact_path == current_artifact_path
    // (exact equality of normalized absolute paths via Path comparison).
    let should_clear = marker_artifact.is_empty()
        || Path::new(&marker_artifact) == Path::new(current_artifact_path);

    if !should_clear {
        // Artifact mismatch: this marker belongs to a different artifact; quarantine persists.
        // BC-1.18.003 INV2: marker{plugin=p, artifact=A} MUST NOT clear on p PASSing artifact B.
        return Ok(());
    }

    // Idempotent delete: NotFound silently swallowed (AC-013; BC-1.18.003 PC2).
    match std::fs::remove_file(marker_path) {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(e),
    }
}

/// Pure predicate: returns `true` iff `outcome` is `Indeterminate` AND `policy` is `FailClosed`.
///
/// This is the load-bearing gate before `write_indeterminate_marker` — fail-open plugins
/// MUST NOT write the marker or trigger the gate (BC-1.18.004 postcondition 1–3).
///
/// - `should_write_marker(Indeterminate, FailClosed) == true`
/// - `should_write_marker(Indeterminate, FailOpen) == false`
/// - `should_write_marker(Pass, FailClosed) == false`
/// - `should_write_marker(Fail, FailClosed) == false`
///
/// `FailurePolicy::default() == FailOpen` (S-21.10 canonical; ADR-039 §Decision 1).
/// VP-106 proof harness covers this predicate.
///
/// # BC-5.38.001
///
/// Non-trivial body — branching on enum variants. Uses `todo!()`.
pub fn should_write_marker(outcome: &DispatchOutcome, policy: FailurePolicy) -> bool {
    // True iff outcome=Indeterminate AND policy=FailClosed (AC-015; BC-1.18.004 PC1).
    // All other combinations return false:
    //   - Indeterminate + FailOpen  → advisory event only (BC-1.18.004 PC2)
    //   - Pass/Fail + FailClosed    → no INDETERMINATE event (write guard)
    //   - Pass/Fail + FailOpen      → no event at all
    // VP-106 proof harness covers this predicate.
    matches!(outcome, DispatchOutcome::Indeterminate { .. }) && policy == FailurePolicy::FailClosed
}

// ---------------------------------------------------------------------------
// ADR-048 §Decision 1 — native crash-path gate check
// ---------------------------------------------------------------------------

/// Check whether a non-expired `.factory/unvalidated-mutation.marker` exists and return
/// `true` (block) iff one does, `false` (allow) otherwise.
///
/// This is the NATIVE (non-WASM) crash-path guard for `on_error = "block_if_marker"`
/// (ADR-048 §Decision 1, BC-1.18.002 v1.5). It is called from the dispatcher's
/// `execute_tiers` loop when a plugin with `on_error = "block_if_marker"` crashes or
/// times out — before the plugin's WASM sandbox gets a chance to run.
///
/// # Decision logic
///
/// - Marker absent (NotFound) → `false` (Allow).
/// - Marker present, non-expired → `true` (Block).
/// - Marker present, expired (`expires_at <= now`) → `false` (Allow).
/// - Marker present, missing/unparseable `expires_at` (legacy pre-ADR-048) → `true` (Block,
///   conservative).
/// - Marker unreadable due to I/O error other than NotFound → `false` (Allow, fail-open
///   on infra fault per CWE-636 balance).
///
/// # Parameters
///
/// - `factory_root`: project root; the marker path is `<factory_root>/.factory/unvalidated-mutation.marker`.
/// - `now`: injectable clock (use `chrono::Utc::now()` in production; fixed value in tests).
pub fn block_if_marker_check(factory_root: &Path, now: DateTime<Utc>) -> bool {
    let marker_path = factory_root
        .join(".factory")
        .join("unvalidated-mutation.marker");
    let content = match std::fs::read_to_string(&marker_path) {
        Ok(c) => c,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return false,
        Err(e) => {
            tracing::warn!(
                error = %e,
                path = %marker_path.display(),
                "block_if_marker: marker read I/O error — allowing (fail-open on infra fault)"
            );
            return false;
        }
    };
    // TTL check (ADR-048 §Decision 2): expired marker → allow (treat as absent).
    match parse_expires_at(&content) {
        Some(exp) if exp <= now => {
            tracing::debug!(
                expires_at = %exp,
                now = %now,
                "block_if_marker: marker TTL elapsed — allowing (expired deadman)"
            );
            false
        }
        // Non-expired expiry, missing expiry (legacy marker), or unparseable expiry → block.
        _ => {
            tracing::info!(
                path = %marker_path.display(),
                "block_if_marker: non-expired marker present — blocking (ADR-048 §Decision 1)"
            );
            true
        }
    }
}

/// Parse the RFC 3339 `expires_at` field from TOML marker content.
///
/// Returns `Some(DateTime<Utc>)` on successful parse; `None` if the field is absent,
/// non-string, or fails RFC 3339 parsing. Callers treat `None` conservatively (block).
fn parse_expires_at(content: &str) -> Option<DateTime<Utc>> {
    let table: toml::Table = toml::from_str(content).ok()?;
    let s = table.get("expires_at")?.as_str()?;
    chrono::DateTime::parse_from_rfc3339(s)
        .ok()
        .map(|dt| dt.with_timezone(&Utc))
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
#[allow(clippy::expect_used, clippy::unwrap_used)]
mod tests {
    use super::*;

    /// EC-008 (BC-1.18.003 INV2 / VP-106): same plugin, DIFFERENT non-empty artifact →
    /// `delete_marker_if_pass` MUST NOT clear the marker (quarantine persists).
    ///
    /// Negative assertion: marker with artifact_path="/abs/A.md" is NOT cleared when
    /// current_artifact_path="/abs/B.md". Positive control: the same marker IS cleared
    /// when current_artifact_path="/abs/A.md" (confirming the predicate is artifact-scoped,
    /// not always-keep).
    #[test]
    fn test_BC_1_18_003_EC_008_different_artifact_preserves_marker() {
        // EC-008 / BC-1.18.003 INV2 / VP-106
        let dir = tempfile::tempdir().expect("tempdir");
        let marker_path = dir.path().join("unvalidated-mutation.marker");

        // Write marker for plugin "p" recording artifact "/abs/A.md".
        let fields = MarkerFields {
            timestamp: "2026-08-31T00:00:00Z".to_string(),
            plugin_name: "p".to_string(),
            artifact_path: "/abs/A.md".to_string(),
            cause: "fuel".to_string(),
            trace_id: "trace-ec-008".to_string(),
            expires_at: "2099-01-01T00:00:00Z".to_string(),
        };
        write_indeterminate_marker(&fields, &marker_path)
            .expect("write_indeterminate_marker must succeed for a writable path");
        assert!(
            marker_path.exists(),
            "pre-condition: marker must exist after write"
        );

        // Phase 1 — PASS on artifact "/abs/B.md" (DIFFERENT): marker MUST NOT be cleared.
        // BC-1.18.003 INV2: marker{artifact=/abs/A.md} MUST persist when current artifact ≠ marker artifact.
        delete_marker_if_pass(&marker_path, "/abs/B.md")
            .expect("delete_marker_if_pass must not return IO error");
        assert!(
            marker_path.exists(),
            "EC-008 / BC-1.18.003 INV2: delete_marker_if_pass(\"/abs/B.md\") MUST NOT clear \
             marker{{artifact_path=\"/abs/A.md\"}} — different artifact, quarantine persists."
        );

        // Phase 2 — positive control: PASS on artifact "/abs/A.md" (SAME): marker MUST be cleared.
        delete_marker_if_pass(&marker_path, "/abs/A.md")
            .expect("delete_marker_if_pass must not return IO error");
        assert!(
            !marker_path.exists(),
            "EC-008 (positive control) / BC-1.18.003 INV2: delete_marker_if_pass(\"/abs/A.md\") \
             MUST clear marker{{artifact_path=\"/abs/A.md\"}} — same artifact, quarantine lifted."
        );
    }

    /// EC-009 (BC-1.18.003 INV2 / VP-106): empty marker `artifact_path` → marker IS cleared
    /// unconditionally for ANY `current_artifact_path` (vacuous/name-only fallback).
    ///
    /// A non-artifact-scoped validator (e.g. a plugin that inspects process state rather
    /// than a specific file) writes an empty `artifact_path`. Its PASS MUST clear the
    /// marker regardless of what artifact the current dispatch is associated with.
    #[test]
    fn test_BC_1_18_003_EC_009_empty_marker_artifact_path_clears_unconditionally() {
        // EC-009 / BC-1.18.003 INV2 / VP-106
        let dir = tempfile::tempdir().expect("tempdir");
        let marker_path = dir.path().join("unvalidated-mutation.marker");

        // Write marker with empty artifact_path (non-artifact-scoped validator).
        let fields = MarkerFields {
            timestamp: "2026-08-31T00:00:00Z".to_string(),
            plugin_name: "p".to_string(),
            artifact_path: "".to_string(),
            cause: "fuel".to_string(),
            trace_id: "trace-ec-009".to_string(),
            expires_at: "2099-01-01T00:00:00Z".to_string(),
        };
        write_indeterminate_marker(&fields, &marker_path)
            .expect("write_indeterminate_marker must succeed for a writable path");
        assert!(
            marker_path.exists(),
            "pre-condition: marker must exist after write"
        );

        // PASS with a non-empty current_artifact_path — the empty marker path is vacuously
        // satisfied; marker MUST be cleared regardless.
        delete_marker_if_pass(&marker_path, "/abs/anything.md")
            .expect("delete_marker_if_pass must not return IO error");
        assert!(
            !marker_path.exists(),
            "EC-009 / BC-1.18.003 INV2: delete_marker_if_pass(\"/abs/anything.md\") MUST clear \
             marker{{artifact_path=\"\"}} — empty artifact_path is the non-artifact-scoped \
             fallback; cleared unconditionally on any PASS."
        );
    }

    /// MEDIUM-4 (RED): The marker TOML serializer MUST correctly round-trip values
    /// containing a double-quote character and a newline. The `toml` crate is the
    /// mandated serialization library (Library table, S-25.01). AC-005.
    ///
    /// Currently FAILS (RED) because `toml_quote` only escapes `\\` and `"` but NOT
    /// `\n` (or other control characters). A newline inside the `artifact_path` value
    /// produces a multi-line TOML basic string that is syntactically INVALID TOML —
    /// `toml::from_str` fails to parse it back, or the round-trip produces a different
    /// string than the original.
    ///
    /// The fix is to use `toml::ser::to_string` (or equivalent) for correct escaping
    /// of all control characters per the TOML specification.
    #[test]
    fn test_BC_1_18_001_marker_toml_round_trips_quote_and_newline() {
        // AC-005 (Library requirement: `toml` crate; hand-rolled toml_quote insufficient)
        let dir = tempfile::tempdir().expect("tempdir");
        let marker_path = dir.path().join("unvalidated-mutation.marker");

        // artifact_path contains both a double-quote and a newline — the two characters
        // that the hand-rolled toml_quote fails to serialize correctly (\n is not escaped).
        let artifact_with_special = "/path/with/\"quote\"\nand newline".to_string();
        let fields = MarkerFields {
            timestamp: "2026-08-31T00:00:00Z".to_string(),
            plugin_name: "validate-factory-path-staging".to_string(),
            artifact_path: artifact_with_special.clone(),
            cause: "fuel".to_string(),
            trace_id: "trace-special-chars-001".to_string(),
            expires_at: "2099-01-01T00:00:00Z".to_string(),
        };

        write_indeterminate_marker(&fields, &marker_path)
            .expect("write_indeterminate_marker must succeed for a writable path");

        let content = std::fs::read_to_string(&marker_path)
            .expect("marker file must be readable after write");

        // Step 1: the content MUST parse as valid TOML.
        // RED: toml_quote does not escape \n, producing invalid multi-line TOML.
        let parsed: toml::Table = toml::from_str(&content).unwrap_or_else(|e| {
            panic!(
                "AC-005: marker content MUST be valid TOML (per Library table, `toml` crate \
                 is mandatory). hand-rolled toml_quote does not escape \\n → invalid TOML. \
                 Parse error: {e}\nMarker content:\n{content}"
            )
        });

        // Step 2: the artifact_path field MUST round-trip to the original value.
        let rt_artifact = parsed
            .get("artifact_path")
            .and_then(|v| v.as_str())
            .expect("artifact_path key must be present and a string after round-trip");

        assert_eq!(
            rt_artifact, artifact_with_special,
            "AC-005: artifact_path containing double-quote and newline MUST round-trip \
             correctly via the `toml` crate serialization (\\n must be escaped as \\\\n)"
        );
    }

    // ── ADR-048 §Decision 1/2: block_if_marker_check + expires_at round-trip ──

    /// BC-1.18.002 PC5/PC6: absent marker → block_if_marker_check returns false (Allow).
    ///
    /// Precondition: no marker file at `factory_root/.factory/unvalidated-mutation.marker`.
    /// Postcondition: returns false (NotFound I/O error → fail-open per ADR-048 §D1 decision logic).
    #[test]
    fn test_BC_1_18_002_block_if_marker_check_absent_allows() {
        let dir = tempfile::tempdir().expect("tempdir");
        let factory_dir = dir.path().join(".factory");
        std::fs::create_dir_all(&factory_dir).expect("create .factory subdir");
        // No marker file written — NotFound path must return false.
        let now = Utc::now();
        assert!(
            !block_if_marker_check(dir.path(), now),
            "BC-1.18.002 PC5: absent marker MUST return false (Allow)"
        );
    }

    /// BC-1.18.002 PC5: marker present with future expires_at → block_if_marker_check returns true.
    ///
    /// Non-expired marker (expires_at >> now) MUST block per ADR-048 §Decision 1.
    #[test]
    fn test_BC_1_18_002_block_if_marker_check_future_expires_at_blocks() {
        let dir = tempfile::tempdir().expect("tempdir");
        let factory_dir = dir.path().join(".factory");
        std::fs::create_dir_all(&factory_dir).expect("create .factory subdir");
        let marker_path = factory_dir.join("unvalidated-mutation.marker");
        let fields = MarkerFields {
            timestamp: "2026-08-31T00:00:00Z".to_string(),
            plugin_name: "p".to_string(),
            artifact_path: String::new(),
            cause: "fuel".to_string(),
            trace_id: "trace-bim-future".to_string(),
            expires_at: "2099-01-01T00:00:00Z".to_string(),
        };
        write_indeterminate_marker(&fields, &marker_path)
            .expect("write_indeterminate_marker must succeed");
        let now = Utc::now();
        assert!(
            block_if_marker_check(dir.path(), now),
            "BC-1.18.002 PC5: marker with future expires_at MUST return true (Block)"
        );
    }

    /// BC-1.18.002 PC6: marker present with past expires_at → block_if_marker_check returns false.
    ///
    /// Expired marker (expires_at <= now) MUST allow per ADR-048 §Decision 2 TTL logic.
    #[test]
    fn test_BC_1_18_002_block_if_marker_check_past_expires_at_allows() {
        let dir = tempfile::tempdir().expect("tempdir");
        let factory_dir = dir.path().join(".factory");
        std::fs::create_dir_all(&factory_dir).expect("create .factory subdir");
        let marker_path = factory_dir.join("unvalidated-mutation.marker");
        let fields = MarkerFields {
            timestamp: "2020-01-01T00:00:00Z".to_string(),
            plugin_name: "p".to_string(),
            artifact_path: String::new(),
            cause: "fuel".to_string(),
            trace_id: "trace-bim-past".to_string(),
            expires_at: "2020-01-02T00:00:00Z".to_string(),
        };
        write_indeterminate_marker(&fields, &marker_path)
            .expect("write_indeterminate_marker must succeed");
        let now = Utc::now();
        assert!(
            !block_if_marker_check(dir.path(), now),
            "BC-1.18.002 PC6: marker with past expires_at MUST return false (Allow, TTL elapsed)"
        );
    }

    /// BC-1.18.002: legacy marker (missing expires_at) → conservative block (true).
    ///
    /// Pre-ADR-048 markers have no expires_at field. The native check treats missing
    /// expires_at as non-expired (conservative) per the ADR-048 §D1 spec.
    #[test]
    fn test_BC_1_18_002_block_if_marker_check_missing_expires_at_blocks_legacy() {
        let dir = tempfile::tempdir().expect("tempdir");
        let factory_dir = dir.path().join(".factory");
        std::fs::create_dir_all(&factory_dir).expect("create .factory subdir");
        let marker_path = factory_dir.join("unvalidated-mutation.marker");
        // 5-field legacy marker — no expires_at field.
        std::fs::write(
            &marker_path,
            "timestamp = \"2026-08-31T00:00:00Z\"\n\
             plugin_name = \"legacy-plugin\"\n\
             artifact_path = \"\"\n\
             cause = \"fuel\"\n\
             trace_id = \"trace-legacy\"\n",
        )
        .expect("write legacy marker");
        let now = Utc::now();
        assert!(
            block_if_marker_check(dir.path(), now),
            "BC-1.18.002: legacy marker (missing expires_at) MUST return true (conservative block)"
        );
    }

    /// BC-1.18.002 I/O error path: directory at marker path triggers a non-NotFound I/O error
    /// on read_to_string → block_if_marker_check returns false (fail-open per CWE-636 balance).
    #[cfg(unix)]
    #[test]
    fn test_BC_1_18_002_block_if_marker_check_io_error_allows() {
        let dir = tempfile::tempdir().expect("tempdir");
        let factory_dir = dir.path().join(".factory");
        std::fs::create_dir_all(&factory_dir).expect("create .factory subdir");
        let marker_path = factory_dir.join("unvalidated-mutation.marker");
        // Place a DIRECTORY at the marker path — read_to_string returns IsADirectory (not NotFound).
        std::fs::create_dir_all(&marker_path).expect("create dir-as-marker-path");
        let now = Utc::now();
        assert!(
            !block_if_marker_check(dir.path(), now),
            "BC-1.18.002: non-NotFound I/O error on marker read MUST return false \
             (fail-open on infra fault per CWE-636 balance)"
        );
    }

    /// BC-1.18.001 v1.1 PC4: write_indeterminate_marker persists all 6 fields correctly;
    /// the round-tripped delta from timestamp to expires_at equals UNVALIDATED_MUTATION_MARKER_TTL_SECONDS.
    ///
    /// Exercises VP-105 (marker write fidelity) and ADR-048 §Decision 2 (24-hour deadman TTL).
    #[test]
    fn test_BC_1_18_001_write_marker_stamps_expires_at_with_86400s_delta() {
        let dir = tempfile::tempdir().expect("tempdir");
        let marker_path = dir.path().join("unvalidated-mutation.marker");
        let ts_str = "2026-08-31T12:00:00Z";
        let ts_dt = chrono::DateTime::parse_from_rfc3339(ts_str)
            .expect("parse test timestamp")
            .with_timezone(&Utc);
        let expires_dt =
            ts_dt + chrono::Duration::seconds(UNVALIDATED_MUTATION_MARKER_TTL_SECONDS as i64);
        let fields = MarkerFields {
            timestamp: ts_str.to_string(),
            plugin_name: "p".to_string(),
            artifact_path: String::new(),
            cause: "fuel".to_string(),
            trace_id: "trace-delta-test".to_string(),
            expires_at: expires_dt.to_rfc3339(),
        };
        write_indeterminate_marker(&fields, &marker_path)
            .expect("write_indeterminate_marker must succeed");
        let content = std::fs::read_to_string(&marker_path).expect("read back marker content");
        let table: toml::Table =
            toml::from_str(&content).expect("round-tripped marker must be valid TOML");
        let rt_ts = table
            .get("timestamp")
            .and_then(|v| v.as_str())
            .expect("timestamp field must be present");
        let rt_exp = table
            .get("expires_at")
            .and_then(|v| v.as_str())
            .expect("expires_at field MUST be present (BC-1.18.001 v1.1 PC4 — 6th required field)");
        let rt_ts_dt = chrono::DateTime::parse_from_rfc3339(rt_ts)
            .expect("round-tripped timestamp must parse as RFC 3339")
            .with_timezone(&Utc);
        let rt_exp_dt = chrono::DateTime::parse_from_rfc3339(rt_exp)
            .expect("round-tripped expires_at must parse as RFC 3339")
            .with_timezone(&Utc);
        let delta_secs = (rt_exp_dt - rt_ts_dt).num_seconds();
        assert_eq!(
            delta_secs, UNVALIDATED_MUTATION_MARKER_TTL_SECONDS as i64,
            "BC-1.18.001 v1.1 PC4: expires_at − timestamp MUST equal \
             UNVALIDATED_MUTATION_MARKER_TTL_SECONDS ({UNVALIDATED_MUTATION_MARKER_TTL_SECONDS}s) \
             — got {delta_secs}s"
        );
    }
}
