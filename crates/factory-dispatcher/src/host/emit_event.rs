//! `emit_event` host function.
//!
//! Decodes the length-prefixed key/value fields buffer the SDK writes,
//! enriches the event with dispatcher-owned identity fields
//! (`trace_id`, `session_id`, `plugin_name`, `plugin_version`,
//! `ts`, `ts_epoch`, `schema_version`) and pushes onto the
//! [`HostContext::events`] stub queue. S-1.8's file sink becomes the
//! consumer.
//!
//! Note: the wire-format name for the trace correlation field is `trace_id`
//! (BC-3.08.001 v1.5 Invariant 5). The internal Rust field in `InternalEvent`
//! is named `dispatcher_trace_id` but serializes as `"trace_id"` via serde rename.

use serde_json::Value;
use wasmtime::Linker;

use super::memory::{read_wasm_bytes, read_wasm_string};
use super::{HostCallError, HostCaller, HostContext};
use crate::internal_log::InternalEvent;

pub fn register(linker: &mut Linker<HostContext>) -> Result<(), HostCallError> {
    linker
        .func_wrap(
            "vsdd",
            "emit_event",
            |mut caller: HostCaller<'_>,
             type_ptr: u32,
             type_len: u32,
             fields_ptr: u32,
             fields_len: u32| {
                let event_type = match read_wasm_string(&mut caller, type_ptr, type_len) {
                    Ok(s) => s,
                    Err(_) => return, // best-effort; drop malformed
                };
                let fields_buf = match read_wasm_bytes(&mut caller, fields_ptr, fields_len) {
                    Ok(b) => b,
                    Err(_) => return,
                };
                let pairs = decode_fields(&fields_buf).unwrap_or_default();

                let ctx = caller.data();
                let mut ev = InternalEvent::now(&event_type)
                    .with_trace_id(&ctx.dispatcher_trace_id)
                    .with_session_id(&ctx.session_id)
                    .with_plugin_name(&ctx.plugin_name)
                    .with_plugin_version(&ctx.plugin_version);
                for (k, v) in pairs {
                    // Plugins cannot override dispatcher-owned identity
                    // fields. See HOST_ABI.md "Event enrichment".
                    if is_reserved_field(&k) {
                        continue;
                    }
                    ev = ev.with_field(&k, Value::String(v));
                }
                ctx.emit_internal(ev);
            },
        )
        .map_err(|e| HostCallError::Linker(e.to_string()))?;
    Ok(())
}

const RESERVED_FIELDS: &[&str] = &[
    // Canonical wire-format name per BC-3.08.001 v1.5 Invariant 5 + DI-017.
    // Plugins must not spoof the dispatcher's trace correlation value.
    "trace_id",
    // Legacy field name — retained for defense-in-depth per BC-3.08.001 v1.5
    // Implementation Notes. The dispatcher no longer emits this on the wire
    // (InternalEvent serializes as "trace_id" via serde rename), but plugins
    // must still be prevented from injecting it.
    "dispatcher_trace_id",
    "session_id",
    "plugin_name",
    "plugin_version",
    "ts",
    "ts_epoch",
    "schema_version",
    "type",
];

fn is_reserved_field(name: &str) -> bool {
    RESERVED_FIELDS.contains(&name)
}

/// Decode the length-prefixed key/value buffer emitted by
/// `vsdd_hook_sdk::host::encode_fields`.
///
/// ```text
/// [ key_len u32 LE | key bytes | value_len u32 LE | value bytes ]+
/// ```
///
/// Returns `Err` only on truncation — the host does not reject
/// non-UTF-8 key/value bytes (they're lossily converted with
/// `String::from_utf8_lossy` so malformed input doesn't drop the
/// whole event).
pub fn decode_fields(bytes: &[u8]) -> Result<Vec<(String, String)>, &'static str> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        if i + 4 > bytes.len() {
            return Err("truncated key length");
        }
        let klen = u32::from_le_bytes(bytes[i..i + 4].try_into().unwrap()) as usize;
        i += 4;
        if i + klen > bytes.len() {
            return Err("truncated key bytes");
        }
        let key = String::from_utf8_lossy(&bytes[i..i + klen]).into_owned();
        i += klen;
        if i + 4 > bytes.len() {
            return Err("truncated value length");
        }
        let vlen = u32::from_le_bytes(bytes[i..i + 4].try_into().unwrap()) as usize;
        i += 4;
        if i + vlen > bytes.len() {
            return Err("truncated value bytes");
        }
        let val = String::from_utf8_lossy(&bytes[i..i + vlen]).into_owned();
        i += vlen;
        out.push((key, val));
    }
    Ok(out)
}

// ---------------------------------------------------------------------------
// S-15.01 T-3e — 4 new event type emission stubs (BC-3.08.001 v1.6)
//
// These functions emit the four new event types introduced by ADR-019.
// All bodies are `todo!()` per BC-5.38.001 Red Gate; the implementer
// fills in the field map and calls `ctx.emit_internal(ev)` in T-3e.
//
// ASYNC_DRAIN_WINDOW_MS is defined in DI-019 — cite by reference only;
// do NOT hardcode the value (Decision 4).
//
// Event catalog authority: BC-3.08.001 (SS-03). Emission sites: SS-01
// (engine.rs, registry.rs). Wire format defined here per BC-3.08.001 v1.6.
// ---------------------------------------------------------------------------

/// Emit `plugin.async_block_discarded` event (BC-3.08.001 v1.6).
///
/// Fired when an async-group plugin returns exit code 2 (block verdict).
/// The block is discarded because async-group verdicts never reach Claude Code
/// (BC-1.14.001 Invariant 4; EC-005). The event provides diagnostic visibility
/// into discarded block signals for operators reviewing events-*.jsonl.
///
/// Required fields (BC-3.08.001):
/// - `plugin_name`: name of the offending async plugin
/// - `reason`: `"async_plugin_block_verdict_discarded"` (literal string, fixed)
/// - `exit_code`: the exit code returned by the plugin (expected: "2")
///
/// # BC traces
/// - BC-3.08.001 v1.6 — event catalog
/// - BC-1.14.001 EC-005 — async plugin exit code 2 behavior
/// - BC-1.14.001 Error Paths — async plugin returns exit code 2
pub fn emit_plugin_async_block_discarded(ctx: &HostContext, plugin_name: &str, exit_code: i32) {
    let ev = InternalEvent::now("plugin.async_block_discarded");
    // BC-3.08.001 wire format: mandatory `trace_id` and `timestamp` fields (DI-017).
    // `with_trace_id` now serializes as `"trace_id"` on the wire (BC-3.08.001 v1.5 Invariant 5).
    // `with_field("timestamp", ...)` adds the BC-required `timestamp` alias for `ts`.
    let ts = ev.ts.clone();
    let ev = ev
        .with_trace_id(&ctx.dispatcher_trace_id)
        .with_session_id(&ctx.session_id)
        .with_field("timestamp", ts.as_str())
        .with_plugin_name(plugin_name)
        .with_field("reason", "async_plugin_block_verdict_discarded")
        .with_field("exit_code", exit_code as i64);
    ctx.emit_internal(ev);
}

/// Emit `dispatcher.schema_mismatch` event (BC-3.08.001 v1.6).
///
/// Fired when the registry `schema_version` does not match
/// `REGISTRY_SCHEMA_VERSION` (currently 2). This is E-REG-001.
/// The dispatcher exits 2 (fail-closed) after emitting this event
/// (BC-1.14.001 Error Paths; exception to BC-1.08.001 fail-open).
///
/// Required fields (BC-3.08.001):
/// - `got`: the `schema_version` found in the registry file (as string)
/// - `expected`: `REGISTRY_SCHEMA_VERSION` value (as string)
/// - `error_code`: `"E-REG-001"` (literal)
///
/// # BC traces
/// - BC-3.08.001 v1.6 — event catalog
/// - BC-1.14.001 Error Paths — schema_version mismatch
/// - BC-1.08.001 amendment — schema-mismatch is the explicit fail-closed exception
pub fn emit_dispatcher_schema_mismatch(ctx: &HostContext, got: u32, expected: u32) {
    let ev = InternalEvent::now("dispatcher.schema_mismatch");
    // BC-3.08.001 wire format: mandatory `trace_id` and `timestamp` fields (DI-017).
    // `with_trace_id` now serializes as `"trace_id"` on the wire (BC-3.08.001 v1.5 Invariant 5).
    let ts = ev.ts.clone();
    let ev = ev
        .with_trace_id(&ctx.dispatcher_trace_id)
        .with_session_id(&ctx.session_id)
        .with_field("timestamp", ts.as_str())
        .with_field("found_version", got as i64)
        .with_field("expected_version", expected as i64)
        .with_field("error_code", "E-REG-001");
    ctx.emit_internal(ev);
}

/// Emit `dispatcher.registry_invalid` event (BC-3.08.001 v1.6).
///
/// Fired when a registry entry violates a load-time invariant.  The caller
/// supplies the error code and violation string so this function can serve
/// multiple invariant violations:
///
/// - E-REG-002 / `"async_block_conflict"`: `on_error=block` + `async=true`
///   coexistence invariant (BC-7.06.001 Invariant 1).
/// - E-REG-003 / `"duplicate_hook_registration"`: duplicate (name, event, tool)
///   tuple (BC-7.06.001 Invariant 7, F-P8-001).
///
/// Required fields (BC-3.08.001):
/// - `plugin_name`: name of the offending registry entry
/// - `error_code`: caller-supplied error code string (e.g. `"E-REG-002"`)
/// - `violation`: caller-supplied violation identifier string
///
/// # BC traces
/// - BC-3.08.001 v1.6 — event catalog
/// - BC-1.14.001 Error Paths — registry invariant violations
/// - BC-7.06.001 Invariants 1 + 7 — load-time invariant enforcement
pub fn emit_dispatcher_registry_invalid(
    ctx: &HostContext,
    plugin_name: &str,
    error_code: &str,
    violation: &str,
) {
    let ev = InternalEvent::now("dispatcher.registry_invalid");
    // BC-3.08.001 wire format: mandatory `trace_id` and `timestamp` fields (DI-017).
    // `with_trace_id` now serializes as `"trace_id"` on the wire (BC-3.08.001 v1.5 Invariant 5).
    let ts = ev.ts.clone();
    let ev = ev
        .with_trace_id(&ctx.dispatcher_trace_id)
        .with_session_id(&ctx.session_id)
        .with_field("timestamp", ts.as_str())
        .with_field("offending_plugin", plugin_name)
        .with_field("violation", violation)
        .with_field("error_code", error_code);
    ctx.emit_internal(ev);
}

/// Emit `plugin.timeout` event for async-path timeouts (BC-3.08.001 v1.6).
///
/// NOTE: A `plugin.timeout` event is also emitted for sync-path timeouts
/// (BC-1.14.001 Error Paths). This stub specifically covers the async-path
/// variant where the event reaches events-*.jsonl but does NOT influence
/// the dispatcher exit code.
///
/// Required fields (BC-3.08.001):
/// - `plugin_name`: name of the timed-out plugin
/// - `timeout_ms`: the configured `timeout_ms` for the entry (as string)
/// - `execution_group`: async or sync indicator (`"async"` for this variant)
///
/// ASYNC_DRAIN_WINDOW_MS for the drain window is defined in DI-019 — cite
/// by reference only. The drain window and the per-plugin timeout_ms are
/// independent values; do NOT conflate them.
///
/// # BC traces
/// - BC-3.08.001 v1.6 — event catalog
/// - BC-1.14.001 Error Paths — async plugin times out
/// - BC-1.14.001 postcondition 4 — async group best-effort lifetime
/// - DI-019 — ASYNC_DRAIN_WINDOW_MS (drain window, not per-plugin timeout)
pub fn emit_plugin_timeout_async(ctx: &HostContext, plugin_name: &str, timeout_ms: u32) {
    let ev = InternalEvent::now("plugin.timeout");
    // BC-3.08.001 wire format: mandatory `trace_id` and `timestamp` fields (DI-017).
    // `with_trace_id` now serializes as `"trace_id"` on the wire (BC-3.08.001 v1.5 Invariant 5).
    let ts = ev.ts.clone();
    let ev = ev
        .with_trace_id(&ctx.dispatcher_trace_id)
        .with_session_id(&ctx.session_id)
        .with_field("timestamp", ts.as_str())
        .with_plugin_name(plugin_name)
        .with_field("execution_group", "async")
        .with_field("timeout_ms", timeout_ms as i64);
    ctx.emit_internal(ev);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn encode(pairs: &[(&str, &str)]) -> Vec<u8> {
        let mut buf = Vec::new();
        for (k, v) in pairs {
            buf.extend_from_slice(&(k.len() as u32).to_le_bytes());
            buf.extend_from_slice(k.as_bytes());
            buf.extend_from_slice(&(v.len() as u32).to_le_bytes());
            buf.extend_from_slice(v.as_bytes());
        }
        buf
    }

    #[test]
    fn decode_single_pair() {
        let buf = encode(&[("k", "v")]);
        let out = decode_fields(&buf).unwrap();
        assert_eq!(out, vec![("k".to_string(), "v".to_string())]);
    }

    #[test]
    fn decode_multiple_pairs() {
        let buf = encode(&[("a", "1"), ("bb", "22"), ("ccc", "333")]);
        let out = decode_fields(&buf).unwrap();
        assert_eq!(out.len(), 3);
        assert_eq!(out[2].0, "ccc");
        assert_eq!(out[2].1, "333");
    }

    #[test]
    fn decode_empty_buffer_yields_empty_vec() {
        assert_eq!(decode_fields(&[]).unwrap(), Vec::new());
    }

    #[test]
    fn decode_rejects_truncated_key_length() {
        let bad = [0u8, 0, 0]; // 3 bytes, need 4 for length prefix
        assert!(decode_fields(&bad).is_err());
    }

    #[test]
    fn decode_rejects_truncated_key_bytes() {
        let mut bad = Vec::new();
        bad.extend_from_slice(&5u32.to_le_bytes());
        bad.extend_from_slice(b"ab"); // only 2 of 5 bytes
        assert!(decode_fields(&bad).is_err());
    }

    #[test]
    fn reserved_fields_rejected() {
        for name in RESERVED_FIELDS {
            assert!(is_reserved_field(name), "{name} should be a reserved field");
        }
    }

    #[test]
    fn non_reserved_field_accepted() {
        assert!(!is_reserved_field("commit_sha"));
        assert!(!is_reserved_field("file_path"));
    }

    /// BC-3.08.001 v1.5 Invariant 5: `trace_id` is the exclusive wire-format name.
    /// Both `trace_id` and `dispatcher_trace_id` must be in RESERVED_FIELDS.
    #[test]
    fn bc3_08_001_invariant5_trace_id_reserved_and_dispatcher_trace_id_reserved() {
        assert!(
            is_reserved_field("trace_id"),
            "trace_id must be in RESERVED_FIELDS (canonical wire field name per BC-3.08.001 v1.5 Invariant 5)"
        );
        assert!(
            is_reserved_field("dispatcher_trace_id"),
            "dispatcher_trace_id must remain in RESERVED_FIELDS for defense-in-depth per BC-3.08.001 v1.5"
        );
    }

    /// BC-3.08.001 v1.5 Invariant 5: InternalEvent must serialize as "trace_id" on wire,
    /// never as "dispatcher_trace_id". Verifies zero occurrences of the legacy field name.
    #[test]
    fn bc3_08_001_invariant5_wire_output_uses_trace_id_not_dispatcher_trace_id() {
        use crate::internal_log::InternalEvent;
        let ev = InternalEvent::now("test.event")
            .with_trace_id("test-uuid-1234")
            .with_field("extra", "value");
        let json = serde_json::to_string(&ev).expect("serialization must not fail");
        assert!(
            json.contains("\"trace_id\""),
            "serialized event must contain \"trace_id\" key; got: {json}"
        );
        assert!(
            !json.contains("\"dispatcher_trace_id\""),
            "serialized event must NOT contain \"dispatcher_trace_id\" in wire output; \
             BC-3.08.001 v1.5 Invariant 5 violation; got: {json}"
        );
        // Verify exactly one occurrence of trace_id
        let occurrences = json.matches("\"trace_id\"").count();
        assert_eq!(
            occurrences, 1,
            "\"trace_id\" must appear exactly once in wire output; found {occurrences}; got: {json}"
        );
    }
}
