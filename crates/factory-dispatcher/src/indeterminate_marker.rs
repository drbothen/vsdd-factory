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
use crate::internal_log::{InternalEvent, InternalLog, PLUGIN_MARKER_CLEARED};
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

/// Read all six TOML fields from the unvalidated-mutation marker file.
///
/// Returns:
/// - `Ok(Some(fields))` if the file exists and all six fields are present and parseable.
/// - `Ok(None)` if the file does not exist (NotFound → no marker).
/// - `Ok(None)` if the file exists but cannot be parsed or any field is missing
///   (corrupt/legacy marker → treat as absent; conservative for the caller).
///
/// Used by the dispatcher's PASS-clear path to obtain the marker's `trace_id`,
/// `plugin_name`, and `artifact_path` before deleting the marker, so that
/// `marker.cleared` (BC-3.08.001 Event 9 / ADR-048 v1.1) can be emitted with
/// the correct provenance fields linking back to the originating `plugin.indeterminate`.
pub fn read_all_marker_fields(marker_path: &Path) -> io::Result<Option<MarkerFields>> {
    match std::fs::read_to_string(marker_path) {
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(e),
        Ok(content) => {
            let table: toml::Table = match toml::from_str(&content) {
                Ok(t) => t,
                Err(_) => return Ok(None),
            };
            let get_str =
                |key: &str| -> Option<String> { table.get(key)?.as_str().map(str::to_string) };
            // All five required fields must be present; missing/non-string fields → None.
            // `expires_at` may be absent on legacy pre-ADR-048 markers → default empty string.
            let (
                Some(timestamp),
                Some(plugin_name),
                Some(artifact_path),
                Some(cause),
                Some(trace_id),
            ) = (
                get_str("timestamp"),
                get_str("plugin_name"),
                get_str("artifact_path"),
                get_str("cause"),
                get_str("trace_id"),
            )
            else {
                return Ok(None);
            };
            Ok(Some(MarkerFields {
                timestamp,
                plugin_name,
                artifact_path,
                cause,
                trace_id,
                expires_at: get_str("expires_at").unwrap_or_default(),
            }))
        }
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
/// Returns:
/// - `Ok(true)`  — the marker file was actually removed by this call.
/// - `Ok(false)` — no-op: marker absent, artifact-path mismatch, or corrupt TOML.
/// - `Err(_)`    — I/O error other than `NotFound` during read or delete.
///
/// Callers use the `bool` return to decide whether to emit `marker.cleared`
/// (BC-3.08.001 Event 9 / ADR-048 v1.1): only emit when `Ok(true)`.
pub fn delete_marker_if_pass(marker_path: &Path, current_artifact_path: &str) -> io::Result<bool> {
    // Read the marker to obtain its artifact_path for the scoped-clear predicate.
    let marker_artifact: String = match std::fs::read_to_string(marker_path) {
        // NotFound: no marker exists — idempotent no-op (AC-013).
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(false),
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
                Err(_) => return Ok(false),
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
        return Ok(false);
    }

    // Delete the marker. NotFound means a concurrent process already cleared it — treat as
    // no-op (Ok(false)), since THIS call did not perform the removal.
    match std::fs::remove_file(marker_path) {
        Ok(()) => Ok(true),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(false),
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
// ADR-048 §Decision 4 v1.2 — dispatcher-native marker.cleared emission
// (S-25.01 LOCAL adversary pass 2 F-P2-002 HIGH + F-P2-003 MED resolution)
// ---------------------------------------------------------------------------

/// Emit `marker.cleared` (BC-3.08.001 Event 9 / ADR-048 §Decision 4).
///
/// Single dispatcher-native emission point for ALL THREE `clear_mode` values
/// (`REVALIDATED`, `TTL_EXPIRED`, `OPERATOR_OVERRIDE`) per ADR-048 §D4 v1.2 —
/// relocated here (from a private `executor.rs` function of the same name)
/// so `check_and_clear_expired_marker` and `reconcile_raw_delete` share the
/// same emission logic as the REVALIDATED path's callsites in `executor.rs`.
///
/// The `trace_id` on this event is set to the MARKER'S stored `trace_id` (not
/// the current dispatch's trace), so the event links back to the originating
/// `plugin.indeterminate` (Event 8) that wrote the marker. This function
/// constructs the `InternalEvent` directly and writes it to `InternalLog` —
/// it NEVER crosses the WASM `emit_event` host ABI boundary, so the
/// RESERVED_FIELDS enrichment that would otherwise silently overwrite
/// `trace_id`/`plugin_name` with the CALLING plugin's own dispatch identity
/// never applies (ADR-048 §D4 v1.2 Emission-Point Correction).
///
/// Best-effort: errors in `log.write` are swallowed (see `InternalLog::write`).
pub(crate) fn emit_marker_cleared(
    log: &InternalLog,
    session_id: &str,
    marker_fields: &MarkerFields,
    clear_mode: &str,
    actor_type: &str,
    reason: Option<&str>,
) {
    let ev = InternalEvent::now(PLUGIN_MARKER_CLEARED)
        // Use marker's trace_id (not current dispatch's) to link back to plugin.indeterminate.
        .with_trace_id(&marker_fields.trace_id)
        .with_session_id(session_id)
        .with_plugin_name(&marker_fields.plugin_name)
        .with_field(
            "artifact_path",
            serde_json::Value::String(marker_fields.artifact_path.clone()),
        )
        .with_field(
            "clear_mode",
            serde_json::Value::String(clear_mode.to_string()),
        )
        .with_field(
            "actor_type",
            serde_json::Value::String(actor_type.to_string()),
        )
        .with_field(
            "reason",
            reason.map_or(serde_json::Value::Null, |r| {
                serde_json::Value::String(r.to_string())
            }),
        );
    log.write(&ev);
}

/// Dispatcher-native pre-check: detect and clear an expired
/// `.factory/unvalidated-mutation.marker`, emitting the audited
/// `marker.cleared(TTL_EXPIRED)` event (BC-3.08.001 Event 9 / ADR-048 §D4 v1.2).
///
/// Called from `executor.rs`'s tier-execution loop BEFORE every Arm 1/Arm 2
/// (`on_error = "block_if_marker"`) plugin invocation on the NORMAL (non-crash)
/// path. This is a DISTINCT code path from the crash-path native check
/// (`block_if_marker_check`), which never auto-deletes or emits — a crash
/// means this pre-check either did not run or was interrupted before
/// completing (BC-1.18.003 §EC-014; VP-108 Postcondition 4).
///
/// # Why dispatcher-native (not the WASM gate plugin)
///
/// The WASM `emit_event` host ABI's RESERVED_FIELDS enrichment unconditionally
/// overwrites plugin-supplied `trace_id`/`plugin_name` with the CALLING
/// plugin's own dispatch identity — a WASM plugin can never emit an event
/// carrying the marker's OWN (foreign) `trace_id`/`plugin_name`, which the
/// Event 9 wire contract requires (ADR-048 §D4 v1.2 Emission-Point
/// Correction; S-25.01 LOCAL adversary pass 2 F-P2-002 HIGH). By the time
/// `evaluate_gate` (hook-plugins crate) subsequently runs, the marker is
/// guaranteed already absent-or-non-expired, so it performs no `expires_at`
/// parsing of its own — it is a pure presence check.
///
/// # Decision logic
///
/// - Marker absent, unreadable, or corrupt (unparseable TOML) — `Ok(None)`,
///   no clear (fail-safe).
/// - Marker present, `expires_at` absent or unparseable (legacy pre-ADR-048
///   marker) — treated as non-expired, conservative — `Ok(None)`.
/// - Marker present, `expires_at > now` — `Ok(None)`, no clear.
/// - Marker present, `expires_at <= now` — delete the marker file
///   (idempotent; a `NotFound` race with a concurrent clear is treated as
///   "this call did not clear it" — `Ok(None)`), emit
///   `marker.cleared(TTL_EXPIRED, deadman, reason=null)` with the MARKER's
///   own `trace_id`/`plugin_name`, and return `Ok(Some(fields))`.
///
/// # Errors
///
/// Propagates non-`NotFound` I/O errors from the marker read or delete.
/// Callers MUST treat an `Err` as "no clear occurred" (fail-safe) and
/// continue the dispatch rather than failing it — a durable pre-check must
/// never itself become a new self-lock source.
pub fn check_and_clear_expired_marker(
    factory_root: &Path,
    now: DateTime<Utc>,
    log: &InternalLog,
    session_id: &str,
) -> io::Result<Option<MarkerFields>> {
    let marker_path = factory_root
        .join(".factory")
        .join("unvalidated-mutation.marker");

    let fields = match read_all_marker_fields(&marker_path)? {
        Some(f) => f,
        None => return Ok(None),
    };

    let expired = chrono::DateTime::parse_from_rfc3339(&fields.expires_at)
        .map(|dt| dt.with_timezone(&Utc) <= now)
        // Absent/unparseable expires_at (legacy marker) -> conservative non-expired.
        .unwrap_or(false);

    if !expired {
        return Ok(None);
    }

    match std::fs::remove_file(&marker_path) {
        Ok(()) => {}
        // Concurrent clear (e.g. a racing REVALIDATED delete) already removed
        // it -- this call did not perform the clear; no emission from here.
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(e) => return Err(e),
    }

    emit_marker_cleared(log, session_id, &fields, "TTL_EXPIRED", "deadman", None);

    Ok(Some(fields))
}

/// Bound on the number of trailing bytes read from today's dispatcher-internal
/// log when [`reconcile_raw_delete`] scans for an unmatched `plugin.indeterminate`
/// event. ADR-048 §D4 v1.2: this scan runs before every Arm 1/Arm 2 dispatch on
/// the marker-absent path, so it MUST be bounded — a fixed-size tail read, not
/// a full-file scan (explicit production-grade constraint, not an optional
/// optimization; an unbounded scan on every Agent/git-commit dispatch would
/// reintroduce the large-artifact resource cost class S-25.01 exists to
/// eliminate).
const RECONCILE_SCAN_BYTE_CAP: u64 = 256 * 1024;

/// Dispatcher-native, best-effort reconciliation for the T3 (human out-of-band
/// `rm`) marker clear path (ADR-048 §Decision 4 v1.2 RAW_DELETE_DETECTED).
///
/// The out-of-band `rm` is never mediated by the dispatcher, so no real-time
/// `marker.cleared(OPERATOR_OVERRIDE)` can be emitted at the moment of
/// deletion. This function is called from the SAME native pre-check that
/// runs before every Arm 1/Arm 2 dispatch, in the branch where the marker is
/// confirmed absent. It performs a bounded scan of TODAY's
/// `dispatcher-internal-<date>.jsonl` for a `plugin.indeterminate` (fail-closed)
/// record with no subsequent `marker.cleared` for the same
/// `(plugin_name, artifact_path)` pair, and retroactively emits
/// `marker.cleared(OPERATOR_OVERRIDE)` — with a non-null `reason` per
/// BC-1.18.003 §PC3 / ADR-048 §D4's event field contract — for each such
/// unmatched pair found.
///
/// # Bounded and best-effort (ADR-048 §D4 v1.2 production-grade requirement)
///
/// - Scans ONLY today's log file (never prior days).
/// - Reads at most the last [`RECONCILE_SCAN_BYTE_CAP`] bytes (tail read).
/// - Any failure to locate/read/parse the log is swallowed — this
///   reconciliation step never gates the dispatch decision (BC-3.08.001
///   Invariant 3) and never returns an error to the caller for a
///   missing/unavailable log.
///
/// # Idempotency
///
/// Once a pair is reconciled (its `marker.cleared` written), the next call
/// sees that emitted event within the scan window and does not re-reconcile
/// it — steady state converges to zero unmatched pairs.
///
/// # Defensive re-check
///
/// Re-verifies the marker is still absent before scanning; a marker found
/// present (a race with the caller's own absent-branch decision) short-
/// circuits to `Ok(())` with no scan performed.
pub fn reconcile_raw_delete(
    factory_root: &Path,
    log: &InternalLog,
    session_id: &str,
) -> io::Result<()> {
    let marker_path = factory_root
        .join(".factory")
        .join("unvalidated-mutation.marker");
    if marker_path.exists() {
        return Ok(());
    }

    let date = chrono::Local::now().format("%Y-%m-%d").to_string();
    let log_path = log
        .log_dir()
        .join(format!("dispatcher-internal-{date}.jsonl"));

    let content = match read_tail(&log_path, RECONCILE_SCAN_BYTE_CAP) {
        Ok(c) => c,
        // No log yet today, or unavailable for any reason: best-effort no-op.
        Err(_) => return Ok(()),
    };

    // Track the most recent unmatched fail-closed plugin.indeterminate per
    // (plugin_name, artifact_path). A subsequent marker.cleared for the same
    // pair (any clear_mode) marks it reconciled. Last-write-wins per pair:
    // an older INDETERMINATE superseded by a newer one for the same pair was
    // itself superseded (single-marker policy), not raw-deleted.
    let mut unmatched: std::collections::HashMap<(String, String), MarkerFields> =
        std::collections::HashMap::new();

    for line in content.lines() {
        let Ok(v) = serde_json::from_str::<serde_json::Value>(line) else {
            // A leading partial line from the tail read, or a corrupt line --
            // skip; best-effort.
            continue;
        };
        let Some(type_) = v.get("type").and_then(|t| t.as_str()) else {
            continue;
        };
        let plugin_name = v
            .get("plugin_name")
            .and_then(|p| p.as_str())
            .unwrap_or("")
            .to_string();
        let artifact_path = v
            .get("artifact_path")
            .and_then(|a| a.as_str())
            .unwrap_or("")
            .to_string();
        let key = (plugin_name.clone(), artifact_path.clone());

        match type_ {
            "plugin.indeterminate" => {
                // Only fail-closed INDETERMINATE events ever write a marker
                // (VP-106 Postcondition B); fail-open events never did, so
                // they can never be the subject of a RAW_DELETE_DETECTED clear.
                if v.get("failure_policy").and_then(|f| f.as_str()) != Some("fail-closed") {
                    continue;
                }
                let trace_id = v
                    .get("trace_id")
                    .and_then(|t| t.as_str())
                    .unwrap_or("")
                    .to_string();
                let cause = v
                    .get("cause")
                    .and_then(|c| c.as_str())
                    .unwrap_or("")
                    .to_string();
                let timestamp = v
                    .get("ts")
                    .and_then(|t| t.as_str())
                    .unwrap_or("")
                    .to_string();
                unmatched.insert(
                    key,
                    MarkerFields {
                        timestamp,
                        plugin_name,
                        artifact_path,
                        cause,
                        trace_id,
                        expires_at: String::new(),
                    },
                );
            }
            "marker.cleared" => {
                unmatched.remove(&key);
            }
            _ => {}
        }
    }

    for fields in unmatched.into_values() {
        emit_marker_cleared(
            log,
            session_id,
            &fields,
            "OPERATOR_OVERRIDE",
            "operator",
            Some(
                "RAW_DELETE_DETECTED: marker absent without prior marker.cleared event; \
                 inferred operator out-of-band clear",
            ),
        );
    }

    Ok(())
}

/// Read at most the trailing `cap` bytes of the file at `path`. Drops a
/// possibly-truncated leading partial line when the read did not start at
/// byte 0 (a tail read may begin mid-write of the byte immediately before
/// the first fully-retained record).
fn read_tail(path: &Path, cap: u64) -> io::Result<String> {
    use std::io::{Read, Seek, SeekFrom};
    let mut f = std::fs::File::open(path)?;
    let len = f.metadata()?.len();
    let start = len.saturating_sub(cap);
    if start > 0 {
        f.seek(SeekFrom::Start(start))?;
    }
    let mut buf = Vec::new();
    f.read_to_end(&mut buf)?;
    let text = String::from_utf8_lossy(&buf).into_owned();
    if start > 0 {
        return Ok(match text.find('\n') {
            Some(idx) => text[idx + 1..].to_string(),
            None => String::new(),
        });
    }
    Ok(text)
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

    // ── ADR-048 §D4 v1.2: check_and_clear_expired_marker + reconcile_raw_delete ──
    // (S-25.01 LOCAL adversary pass 2 F-P2-002 HIGH + F-P2-003 MED resolution)

    /// Read back the single JSONL line written under `log_dir` and parse it as JSON.
    /// Panics if zero or more than one file/line is present — every test using this
    /// helper writes exactly one InternalLog-backed event before calling it.
    fn read_only_log_event(log_dir: &Path) -> serde_json::Value {
        let files: Vec<_> = std::fs::read_dir(log_dir)
            .expect("log dir must exist after a write")
            .map(|e| e.expect("dir entry").path())
            .collect();
        assert_eq!(
            files.len(),
            1,
            "expected exactly one log file, got {files:?}"
        );
        let content = std::fs::read_to_string(&files[0]).expect("read log file");
        let lines: Vec<&str> = content.lines().collect();
        assert_eq!(
            lines.len(),
            1,
            "expected exactly one log line, got {lines:?}"
        );
        serde_json::from_str(lines[0]).expect("log line must be valid JSON")
    }

    /// VP-108 PC2 / VP-106 PC-F: expired marker → `check_and_clear_expired_marker`
    /// deletes it and emits exactly one `marker.cleared(TTL_EXPIRED)` event carrying
    /// the MARKER's own trace_id/plugin_name (not the caller's session_id-scoped
    /// identity) and `reason = null`.
    #[test]
    fn test_ADR_048_D4_check_and_clear_expired_marker_ttl_expired_deletes_and_emits() {
        let dir = tempfile::tempdir().expect("tempdir");
        let factory_dir = dir.path().join(".factory");
        std::fs::create_dir_all(&factory_dir).expect("create .factory subdir");
        let marker_path = factory_dir.join("unvalidated-mutation.marker");
        let fields = MarkerFields {
            timestamp: "2020-01-01T00:00:00Z".to_string(),
            plugin_name: "regression-gate".to_string(),
            artifact_path: "/tmp/.factory/STATE.md".to_string(),
            cause: "fuel".to_string(),
            trace_id: "deadbeef-0106-0001-0001-000000000001".to_string(),
            expires_at: "2020-01-02T00:00:00Z".to_string(),
        };
        write_indeterminate_marker(&fields, &marker_path).expect("write marker");

        // Log dir deliberately NOT under `.factory` so the #206 mount gate does
        // not suppress the write (matches the executor.rs test pattern).
        let log = InternalLog::new(dir.path().join("logs"));
        let now = chrono::DateTime::parse_from_rfc3339("2020-01-03T00:00:00Z")
            .expect("parse now")
            .with_timezone(&Utc);

        let cleared = check_and_clear_expired_marker(dir.path(), now, &log, "sess-ttl")
            .expect("check_and_clear_expired_marker must not error");

        assert!(
            matches!(&cleared, Some(f) if f.trace_id == fields.trace_id),
            "VP-106 PC-F: expired marker MUST yield Some(fields) with the marker's own trace_id"
        );
        assert!(
            !marker_path.exists(),
            "VP-106 PC-F: check_and_clear_expired_marker MUST auto-delete the expired marker"
        );

        let event = read_only_log_event(&dir.path().join("logs"));
        assert_eq!(
            event["type"], "marker.cleared",
            "VP-108 PC2: type must be marker.cleared"
        );
        assert_eq!(
            event["clear_mode"], "TTL_EXPIRED",
            "VP-108 PC2: clear_mode must be TTL_EXPIRED"
        );
        assert_eq!(
            event["actor_type"], "deadman",
            "VP-108 PC2: actor_type must be deadman"
        );
        assert_eq!(
            event["trace_id"], fields.trace_id,
            "VP-108 PC2: trace_id MUST be the MARKER's own trace_id, not the gate plugin's dispatch trace_id"
        );
        assert_eq!(event["plugin_name"], fields.plugin_name);
        assert_eq!(event["artifact_path"], fields.artifact_path);
        assert!(
            event["reason"].is_null(),
            "VP-108 PC2: reason must be null/absent for TTL_EXPIRED"
        );
    }

    /// VP-106 INV5: non-expired marker → `check_and_clear_expired_marker` returns
    /// `None`, does NOT delete the marker, and emits NO event.
    #[test]
    fn test_ADR_048_D4_check_and_clear_expired_marker_non_expired_returns_none_no_emit() {
        let dir = tempfile::tempdir().expect("tempdir");
        let factory_dir = dir.path().join(".factory");
        std::fs::create_dir_all(&factory_dir).expect("create .factory subdir");
        let marker_path = factory_dir.join("unvalidated-mutation.marker");
        let fields = MarkerFields {
            timestamp: "2026-08-31T00:00:00Z".to_string(),
            plugin_name: "p".to_string(),
            artifact_path: String::new(),
            cause: "fuel".to_string(),
            trace_id: "trace-non-expired".to_string(),
            expires_at: "2099-01-01T00:00:00Z".to_string(),
        };
        write_indeterminate_marker(&fields, &marker_path).expect("write marker");

        let log_dir = dir.path().join("logs");
        let log = InternalLog::new(log_dir.clone());
        let now = Utc::now();

        let cleared = check_and_clear_expired_marker(dir.path(), now, &log, "sess-active")
            .expect("check_and_clear_expired_marker must not error");

        assert!(
            cleared.is_none(),
            "VP-106 INV5: non-expired marker MUST yield None"
        );
        assert!(
            marker_path.exists(),
            "VP-106 INV5: non-expired marker MUST NOT be deleted"
        );
        assert!(
            !log_dir.exists(),
            "VP-108: no marker.cleared event (and no log dir at all) when nothing was cleared"
        );
    }

    /// VP-106: absent marker → `check_and_clear_expired_marker` returns `None`, no emit.
    #[test]
    fn test_ADR_048_D4_check_and_clear_expired_marker_absent_returns_none() {
        let dir = tempfile::tempdir().expect("tempdir");
        std::fs::create_dir_all(dir.path().join(".factory")).expect("create .factory subdir");
        let log_dir = dir.path().join("logs");
        let log = InternalLog::new(log_dir.clone());

        let cleared = check_and_clear_expired_marker(dir.path(), Utc::now(), &log, "sess-absent")
            .expect("check_and_clear_expired_marker must not error on absent marker");

        assert!(cleared.is_none(), "absent marker MUST yield None");
        assert!(
            !log_dir.exists(),
            "no event MUST be emitted when there is nothing to clear"
        );
    }

    /// VP-106 PC-G: legacy marker (no expires_at field) → treated as non-expired
    /// (conservative); `check_and_clear_expired_marker` returns `None`, no delete, no emit.
    #[test]
    fn test_ADR_048_D4_check_and_clear_expired_marker_legacy_missing_expires_at_returns_none() {
        let dir = tempfile::tempdir().expect("tempdir");
        let factory_dir = dir.path().join(".factory");
        std::fs::create_dir_all(&factory_dir).expect("create .factory subdir");
        let marker_path = factory_dir.join("unvalidated-mutation.marker");
        // Legacy 5-field marker — no expires_at.
        std::fs::write(
            &marker_path,
            "timestamp = \"2026-08-31T00:00:00Z\"\n\
             plugin_name = \"legacy-plugin\"\n\
             artifact_path = \"\"\n\
             cause = \"fuel\"\n\
             trace_id = \"trace-legacy-check\"\n",
        )
        .expect("write legacy marker");

        let log_dir = dir.path().join("logs");
        let log = InternalLog::new(log_dir.clone());

        let cleared = check_and_clear_expired_marker(dir.path(), Utc::now(), &log, "sess-legacy")
            .expect("check_and_clear_expired_marker must not error");

        assert!(
            cleared.is_none(),
            "VP-106 PC-G: legacy marker (absent expires_at) MUST yield None — conservative"
        );
        assert!(
            marker_path.exists(),
            "VP-106 PC-G: legacy marker MUST NOT be auto-deleted"
        );
        assert!(
            !log_dir.exists(),
            "no marker.cleared event MUST be emitted for a legacy marker"
        );
    }

    /// VP-108 PC3: an unmatched fail-closed `plugin.indeterminate` with the marker
    /// now absent → `reconcile_raw_delete` retroactively emits exactly one
    /// `marker.cleared(OPERATOR_OVERRIDE)` with the indeterminate event's own
    /// trace_id/plugin_name/artifact_path and a NON-null `reason`
    /// (BC-1.18.003 §PC3 / ADR-048 §D4 event field contract — `reason` is
    /// mandatory, not null, for OPERATOR_OVERRIDE).
    #[test]
    fn test_ADR_048_D4_reconcile_raw_delete_unmatched_event_emits_operator_override() {
        let dir = tempfile::tempdir().expect("tempdir");
        std::fs::create_dir_all(dir.path().join(".factory")).expect("create .factory subdir");
        // No marker file — this is the marker-absent branch reconcile_raw_delete runs in.

        let log_dir = dir.path().join("logs");
        std::fs::create_dir_all(&log_dir).expect("create log dir");
        let date = chrono::Local::now().format("%Y-%m-%d").to_string();
        let log_file = log_dir.join(format!("dispatcher-internal-{date}.jsonl"));
        let indeterminate_line = serde_json::json!({
            "type": "plugin.indeterminate",
            "trace_id": "trace-raw-delete-1",
            "session_id": "sess-orig",
            "plugin_name": "regression-gate",
            "artifact_path": "/tmp/.factory/STATE.md",
            "cause": "fuel",
            "failure_policy": "fail-closed",
            "ts": "2026-08-31T00:00:00+0000",
            "ts_epoch": 1_798_675_200_i64,
            "schema_version": 1,
        });
        std::fs::write(&log_file, format!("{indeterminate_line}\n")).expect("seed log");

        let log = InternalLog::new(log_dir.clone());
        reconcile_raw_delete(dir.path(), &log, "sess-reconcile")
            .expect("reconcile_raw_delete must not error");

        let content = std::fs::read_to_string(&log_file).expect("read log");
        let lines: Vec<&str> = content.lines().collect();
        assert_eq!(
            lines.len(),
            2,
            "VP-108 PC3: exactly one marker.cleared line MUST be appended — got {lines:?}"
        );
        let event: serde_json::Value =
            serde_json::from_str(lines[1]).expect("second line must be valid JSON");
        assert_eq!(event["type"], "marker.cleared");
        assert_eq!(event["clear_mode"], "OPERATOR_OVERRIDE");
        assert_eq!(event["actor_type"], "operator");
        assert_eq!(
            event["trace_id"], "trace-raw-delete-1",
            "VP-108 PC3: trace_id MUST come from the unmatched plugin.indeterminate event"
        );
        assert_eq!(event["plugin_name"], "regression-gate");
        assert_eq!(event["artifact_path"], "/tmp/.factory/STATE.md");
        assert!(
            event["reason"]
                .as_str()
                .map(|r| !r.is_empty())
                .unwrap_or(false),
            "BC-1.18.003 §PC3 / ADR-048 §D4: reason MUST be non-null and non-empty for \
             OPERATOR_OVERRIDE — got {:?}",
            event["reason"]
        );
    }

    /// VP-108 PC3 (negative): a `plugin.indeterminate` already followed by its own
    /// `marker.cleared` for the same (plugin_name, artifact_path) is already
    /// reconciled — `reconcile_raw_delete` MUST NOT emit a duplicate.
    #[test]
    fn test_ADR_048_D4_reconcile_raw_delete_already_matched_emits_nothing() {
        let dir = tempfile::tempdir().expect("tempdir");
        std::fs::create_dir_all(dir.path().join(".factory")).expect("create .factory subdir");

        let log_dir = dir.path().join("logs");
        std::fs::create_dir_all(&log_dir).expect("create log dir");
        let date = chrono::Local::now().format("%Y-%m-%d").to_string();
        let log_file = log_dir.join(format!("dispatcher-internal-{date}.jsonl"));
        let indeterminate_line = serde_json::json!({
            "type": "plugin.indeterminate",
            "trace_id": "trace-matched-1",
            "plugin_name": "regression-gate",
            "artifact_path": "/tmp/.factory/STATE.md",
            "cause": "fuel",
            "failure_policy": "fail-closed",
            "ts": "2026-08-31T00:00:00+0000",
        });
        let cleared_line = serde_json::json!({
            "type": "marker.cleared",
            "trace_id": "trace-matched-1",
            "plugin_name": "regression-gate",
            "artifact_path": "/tmp/.factory/STATE.md",
            "clear_mode": "REVALIDATED",
            "actor_type": "validator",
            "ts": "2026-08-31T00:05:00+0000",
        });
        std::fs::write(&log_file, format!("{indeterminate_line}\n{cleared_line}\n"))
            .expect("seed log");

        let log = InternalLog::new(log_dir.clone());
        reconcile_raw_delete(dir.path(), &log, "sess-reconcile")
            .expect("reconcile_raw_delete must not error");

        let content = std::fs::read_to_string(&log_file).expect("read log");
        assert_eq!(
            content.lines().count(),
            2,
            "already-reconciled pair MUST NOT produce a new marker.cleared emission"
        );
    }

    /// `reconcile_raw_delete` defensive re-check: marker still present → no-op,
    /// regardless of log content (never emits while the marker itself is active).
    #[test]
    fn test_ADR_048_D4_reconcile_raw_delete_marker_present_short_circuits() {
        let dir = tempfile::tempdir().expect("tempdir");
        let factory_dir = dir.path().join(".factory");
        std::fs::create_dir_all(&factory_dir).expect("create .factory subdir");
        let marker_path = factory_dir.join("unvalidated-mutation.marker");
        let fields = MarkerFields {
            timestamp: "2026-08-31T00:00:00Z".to_string(),
            plugin_name: "p".to_string(),
            artifact_path: String::new(),
            cause: "fuel".to_string(),
            trace_id: "trace-present".to_string(),
            expires_at: "2099-01-01T00:00:00Z".to_string(),
        };
        write_indeterminate_marker(&fields, &marker_path).expect("write marker");

        let log_dir = dir.path().join("logs");
        let log = InternalLog::new(log_dir.clone());

        reconcile_raw_delete(dir.path(), &log, "sess-present")
            .expect("reconcile_raw_delete must not error");

        assert!(
            !log_dir.exists(),
            "reconcile_raw_delete MUST NOT scan or emit while the marker is present"
        );
    }

    /// ADR-048 §D4 v1.2 bounded-scan requirement: an unmatched `plugin.indeterminate`
    /// that falls OUTSIDE the trailing `RECONCILE_SCAN_BYTE_CAP` window (because it
    /// is followed by more than the cap's worth of filler bytes) MUST NOT be
    /// reconciled — the scan is a bounded tail read, not a full-file scan.
    #[test]
    fn test_ADR_048_D4_reconcile_raw_delete_respects_byte_cap() {
        let dir = tempfile::tempdir().expect("tempdir");
        std::fs::create_dir_all(dir.path().join(".factory")).expect("create .factory subdir");

        let log_dir = dir.path().join("logs");
        std::fs::create_dir_all(&log_dir).expect("create log dir");
        let date = chrono::Local::now().format("%Y-%m-%d").to_string();
        let log_file = log_dir.join(format!("dispatcher-internal-{date}.jsonl"));

        let indeterminate_line = serde_json::json!({
            "type": "plugin.indeterminate",
            "trace_id": "trace-out-of-window",
            "plugin_name": "regression-gate",
            "artifact_path": "/tmp/.factory/STATE.md",
            "cause": "fuel",
            "failure_policy": "fail-closed",
            "ts": "2026-08-31T00:00:00+0000",
        })
        .to_string();

        // Filler well past the byte cap, written AFTER the target line so the
        // trailing tail-read window no longer includes it.
        let filler_line = serde_json::json!({
            "type": "plugin.completed",
            "plugin_name": "irrelevant-plugin",
            "ts": "2026-08-31T00:00:01+0000",
        })
        .to_string();
        let mut content = format!("{indeterminate_line}\n");
        while (content.len() as u64) < RECONCILE_SCAN_BYTE_CAP + 4096 {
            content.push_str(&filler_line);
            content.push('\n');
        }
        std::fs::write(&log_file, &content).expect("seed large log");

        let log = InternalLog::new(log_dir.clone());
        reconcile_raw_delete(dir.path(), &log, "sess-bounded")
            .expect("reconcile_raw_delete must not error");

        let after = std::fs::read_to_string(&log_file).expect("read log");
        assert!(
            !after.contains("OPERATOR_OVERRIDE"),
            "bounded scan MUST NOT reconcile an unmatched event that falls outside the \
             trailing RECONCILE_SCAN_BYTE_CAP window"
        );
    }

    /// `reconcile_raw_delete` no-op when today's log file does not exist at all
    /// (fresh session, nothing indeterminate has happened yet today).
    #[test]
    fn test_ADR_048_D4_reconcile_raw_delete_no_log_file_is_noop() {
        let dir = tempfile::tempdir().expect("tempdir");
        std::fs::create_dir_all(dir.path().join(".factory")).expect("create .factory subdir");
        let log_dir = dir.path().join("logs");
        // log_dir itself does not exist yet.
        let log = InternalLog::new(log_dir.clone());

        let result = reconcile_raw_delete(dir.path(), &log, "sess-no-log");
        assert!(
            result.is_ok(),
            "reconcile_raw_delete MUST be a best-effort no-op when no log exists yet, got {result:?}"
        );
    }
}
