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
///
/// # Errors
///
/// Returns `io::Error` if the temp-file write or rename fails (EC-008).
/// The caller emits the `plugin.indeterminate` event regardless of whether this write succeeds.
///
/// # BC-5.38.001
///
/// Non-trivial body — filesystem I/O, TOML serialization, atomic rename. Uses `todo!()`.
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

    // Serialize 5 mandatory TOML fields (BC-1.18.001 postcondition 4).
    // Escape TOML string values by representing them as quoted strings.
    // Each field is individually quoted; all five are mandatory even when empty.
    let content = format!(
        "timestamp = {t}\nplugin_name = {p}\nartifact_path = {a}\ncause = {c}\ntrace_id = {r}\n",
        t = toml_quote(&fields.timestamp),
        p = toml_quote(&fields.plugin_name),
        a = toml_quote(&fields.artifact_path),
        c = toml_quote(&fields.cause),
        r = toml_quote(&fields.trace_id),
    );

    // Write to temp file (O_CREAT | O_WRONLY | O_TRUNC semantics via fs::write).
    std::fs::write(&tmp_path, content.as_bytes())?;

    // Atomic rename temp → final path (single-marker policy: overwrites existing).
    std::fs::rename(&tmp_path, marker_path)?;

    Ok(())
}

/// Escape a string for use as a TOML basic string value.
/// Wraps in double-quotes and escapes backslashes and double-quotes.
/// This is sufficient for the 5 marker fields (timestamps, names, paths, cause, trace IDs).
fn toml_quote(s: &str) -> String {
    let escaped = s.replace('\\', "\\\\").replace('"', "\\\"");
    format!("\"{escaped}\"")
}

/// Idempotently delete the unvalidated-mutation marker file.
///
/// If `marker_path` does not exist (`io::ErrorKind::NotFound`), returns `Ok(())` (no-op).
/// All other `io::Error` variants are propagated as `Err`.
///
/// Scoping (BC-1.18.003 postcondition 1 + invariant 2): the caller is responsible for
/// verifying that the `plugin_name` field in the marker matches the re-validating plugin
/// before calling this function. `delete_marker_if_pass` itself does not read the marker.
///
/// # BC-5.38.001
///
/// Non-trivial body — filesystem I/O, NotFound handling. Uses `todo!()`.
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
