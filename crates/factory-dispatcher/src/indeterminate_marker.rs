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

use crate::executor::DispatchOutcome;
use crate::registry::FailurePolicy;

/// The five required TOML fields for `.factory/unvalidated-mutation.marker`.
///
/// BC-1.18.001 postcondition 4: all five fields MUST be present.
/// `artifact_path` MUST be empty string (not omitted) when no artifact context.
/// `cause` MUST be one of `"fuel"`, `"epoch"`, `"output-too-large"` (BC-3.08.001 Event 8).
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
}

/// Atomically write the unvalidated-mutation marker file via write-to-temp + rename.
///
/// Algorithm (BC-1.18.001 postcondition 4 + invariant 3):
/// 1. Compute temp path: `<marker_path>.tmp` in the same directory as `marker_path`.
/// 2. Serialize the five required TOML fields to the temp file (O_CREAT | O_WRONLY | O_TRUNC).
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
    // Using sibling .tmp file guarantees same-filesystem rename.
    let file_name = marker_path
        .file_name()
        .and_then(|f| f.to_str())
        .unwrap_or("unvalidated-mutation.marker");
    let tmp_name = format!("{file_name}.tmp");
    let tmp_path = marker_path
        .parent()
        .unwrap_or_else(|| std::path::Path::new("."))
        .join(&tmp_name);

    // Serialize 5 mandatory TOML fields via the `toml` crate (Library table mandate).
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

    let content = toml::to_string(&table)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;

    // Write to temp file (O_CREAT | O_WRONLY | O_TRUNC semantics via fs::write).
    std::fs::write(&tmp_path, content.as_bytes())?;

    // Atomic rename temp → final path (single-marker policy: overwrites existing).
    std::fs::rename(&tmp_path, marker_path)?;

    Ok(())
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
#[allow(clippy::expect_used, clippy::unwrap_used)]
mod tests {
    use super::*;

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
            rt_artifact,
            artifact_with_special,
            "AC-005: artifact_path containing double-quote and newline MUST round-trip \
             correctly via the `toml` crate serialization (\\n must be escaped as \\\\n)"
        );
    }
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

/// Idempotently delete the unvalidated-mutation marker file.
///
/// If `marker_path` does not exist (`io::ErrorKind::NotFound`), returns `Ok(())` (no-op).
/// All other `io::Error` variants are propagated as `Err`.
///
/// Scoping (BC-1.18.003 postcondition 1 + invariant 2): the caller is responsible for
/// verifying that the `plugin_name` field in the marker matches the re-validating plugin
/// before calling this function. `delete_marker_if_pass` itself does not read the marker.
pub fn delete_marker_if_pass(marker_path: &Path) -> io::Result<()> {
    // Idempotent delete: NotFound is silently swallowed (AC-013; BC-1.18.003 PC2).
    // All other io::Error variants are propagated as Err.
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
