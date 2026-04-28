//! session-end-telemetry — SessionEnd WASM hook plugin.
//!
//! Emits `session.ended` with 3 plugin-set fields per BC-4.05.001:
//!   - `duration_ms` — elapsed milliseconds since `session_start_ts` in the envelope,
//!     or `"0"` when absent, future, or unparseable (BC-4.05.001 PC-2)
//!   - `tool_call_count` — tool call count from envelope, or `"0"` if absent (BC-4.05.001 PC-2)
//!   - `timestamp` — ISO-8601 UTC with millisecond precision and `Z` suffix (plugin emit time)
//!
//! 4 host-enriched fields are auto-injected by the `emit_event` host fn from `HostContext`
//! (BC-1.05.012): `dispatcher_trace_id`, `session_id`, `plugin_name`, `plugin_version`.
//!
//! 4 construction-time fields are set by `InternalEvent::now()`:
//! `ts`, `ts_epoch`, `schema_version`, `type`.
//!
//! Plugin is unconditionally stateless (idempotency enforced at Layer 1 by Claude Code's
//! `once: true` directive in hooks.json.template per BC-4.05.003 + BC-4.05.004).
//!
//! No `exec_subprocess` call is made (BC-4.05.002). No `read_file` call is made.
//! All data comes from the incoming envelope's `tool_input` fields.

use vsdd_hook_sdk::{HookPayload, HookResult};

// ---------------------------------------------------------------------------
// Timestamp helper
// ---------------------------------------------------------------------------

/// Generate an ISO-8601 UTC timestamp with millisecond precision and `Z` suffix.
///
/// Format: `^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d{3}Z$`
pub fn now_timestamp() -> String {
    use chrono::Utc;
    Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string()
}

/// Return the current time as milliseconds since UNIX epoch.
pub fn now_ms() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64
}

// ---------------------------------------------------------------------------
// duration_ms computation (BC-4.05.001 PC-2)
// ---------------------------------------------------------------------------

/// Compute `duration_ms` from an ISO-8601 `session_start_ts` string.
///
/// Returns a non-negative elapsed duration in milliseconds as a decimal string.
/// Returns `"0"` per BC-4.05.001 PC-2 when:
///   (a) `session_start_ts` is `None` (absent from envelope)
///   (b) `session_start_ts` is in the future relative to `now_ms` (clock-skew clamp)
///   (c) `session_start_ts` is present as a string but not parseable as ISO-8601
///
/// Non-string envelope types are out of scope (v1.1 candidate per BC-4.05.001 EC-001c).
pub fn compute_duration_ms(session_start_ts: Option<&str>, current_now_ms: i64) -> String {
    let ts_str = match session_start_ts {
        None => return "0".to_string(),
        Some(s) => s,
    };

    // Parse as ISO-8601 — treat any parse failure as absent (EC-001c).
    let start_ms = match chrono::DateTime::parse_from_rfc3339(ts_str) {
        Ok(dt) => dt.timestamp_millis(),
        Err(_) => return "0".to_string(),
    };

    // Clock-skew clamp: negative elapsed duration → "0" (EC-001b).
    let elapsed = current_now_ms - start_ms;
    if elapsed < 0 {
        "0".to_string()
    } else {
        elapsed.to_string()
    }
}

// ---------------------------------------------------------------------------
// Public hook logic surface (testable without WASM runtime)
// ---------------------------------------------------------------------------

/// Top-level session-end hook logic with injectable emit callback.
///
/// All host function dependencies are injected so unit/integration tests can
/// drive every branch without a WASM runtime — the same pattern as
/// `session-start-telemetry::session_start_hook_logic`.
///
/// - `emit_fn`: called exactly once to emit the `session.ended` event
///
/// The plugin is unconditionally stateless (BC-4.05.003): it emits on every
/// invocation without dedup state. Once-discipline is Layer 1 (BC-4.05.004).
///
/// Data sources:
/// - `duration_ms`: computed from `payload.tool_input["session_start_ts"]` (string field) +
///   current clock; falls back to `"0"` per BC-4.05.001 PC-2 branches (a), (b), (c)
/// - `tool_call_count`: from `payload.tool_input["tool_call_count"]`; falls back to `"0"`
/// - `timestamp`: ISO-8601 UTC ms precision Z suffix; plugin's own emission instant
///
/// RESERVED_FIELDS the plugin MUST NOT set (8 total):
/// Host-enriched: `dispatcher_trace_id`, `session_id`, `plugin_name`, `plugin_version`
/// Construction-time: `ts`, `ts_epoch`, `schema_version`, `type`
pub fn session_end_hook_logic<Emit>(payload: HookPayload, emit_fn: Emit) -> HookResult
where
    Emit: FnOnce(&[(&str, &str)]),
{
    // 1. Compute duration_ms per BC-4.05.001 PC-2.
    let current_now_ms = now_ms();
    let duration_ms = {
        let ts_opt = payload
            .tool_input
            .get("session_start_ts")
            .and_then(|v| v.as_str());
        compute_duration_ms(ts_opt, current_now_ms)
    };

    // 2. Compute tool_call_count per BC-4.05.001 PC-2 / EC-002.
    //    Envelope may carry it as int OR string; coerce to non-negative decimal string.
    //    Falls back to "0" if absent.
    let tool_call_count = match payload.tool_input.get("tool_call_count") {
        None => "0".to_string(),
        Some(v) => {
            if let Some(n) = v.as_u64() {
                n.to_string()
            } else if let Some(n) = v.as_i64() {
                if n < 0 { "0".to_string() } else { n.to_string() }
            } else if let Some(s) = v.as_str() {
                match s.parse::<u64>() {
                    Ok(n) => n.to_string(),
                    Err(_) => "0".to_string(),
                }
            } else {
                "0".to_string()
            }
        }
    };

    // 3. Compute timestamp: ISO-8601 UTC with ms precision and Z suffix.
    let timestamp = now_timestamp();

    // 4. Emit session.ended with the 3 plugin-set fields.
    //    RESERVED_FIELDS are NOT set here (host-enriched / construction-time).
    emit_fn(&[
        ("duration_ms", duration_ms.as_str()),
        ("tool_call_count", tool_call_count.as_str()),
        ("timestamp", timestamp.as_str()),
    ]);

    HookResult::Continue
}

// ---------------------------------------------------------------------------
// Top-level entry point called from main.rs (no callbacks — uses host fns).
// ---------------------------------------------------------------------------

/// Called from the WASI entry point in `main.rs`.
///
/// Wires the real vsdd_hook_sdk host functions to the injectable-callback
/// surface of `session_end_hook_logic`.
pub fn on_session_end(payload: HookPayload) -> HookResult {
    session_end_hook_logic(payload, |fields| {
        vsdd_hook_sdk::host::emit_event("session.ended", fields);
    })
}
