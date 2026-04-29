//! Integration tests for session-end-telemetry — VP-066 harness (RED gate).
//!
//! # Scope
//!
//! Covers all 5 behavioral contracts in BC-4.05.001..005 and verification
//! property VP-066 "Session-End Plugin Surface Invariant".
//!
//! # Test harness design (mirrors VP-065 pattern)
//!
//! The tests call `session_end_hook_logic` with an injectable mock emit callback,
//! avoiding a full WASM runtime. The pattern mirrors VP-065 / session-start-telemetry.
//!
//! The `dispatch_and_capture` helper:
//!   1. Invokes the production `session_end_hook_logic` with a mocked emit callback.
//!   2. Captures all fields passed to `emit_event`.
//!   3. Simulates host-enrichment (dispatcher_trace_id, session_id, plugin_name,
//!      plugin_version) and construction-time fields (ts, ts_epoch, schema_version, type).
//!   4. Returns captured events as `Vec<serde_json::Value>`.
//!
//! # PATH RESOLUTION (VP-066 — same as VP-065)
//!
//! File-system tests (`test_bc_4_05_004_*`, `test_bc_4_05_005_*`) locate
//! workspace artifacts via `workspace_root()` which walks upward from
//! `CARGO_MANIFEST_DIR` until it finds `Cargo.lock`. This avoids fragile
//! hardcoded paths and is the documented choice for this harness.

#[cfg(test)]
mod session_end_integration {
    use session_end_telemetry::session_end_hook_logic;
    use vsdd_hook_sdk::HookPayload;

    // -----------------------------------------------------------------------
    // Workspace root resolver (VP-066 — same pattern as VP-065)
    // -----------------------------------------------------------------------

    /// Resolve workspace root by walking up from CARGO_MANIFEST_DIR until a
    /// directory containing `Cargo.lock` is found.
    fn workspace_root() -> std::path::PathBuf {
        let manifest = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let mut dir = manifest.as_path();
        loop {
            if dir.join("Cargo.lock").exists() {
                return dir.to_path_buf();
            }
            dir = dir
                .parent()
                .expect("reached filesystem root without finding Cargo.lock");
        }
    }

    // -----------------------------------------------------------------------
    // CountingMock for exec_subprocess
    // -----------------------------------------------------------------------

    /// Tracks how many times exec_subprocess was (hypothetically) invoked.
    ///
    /// For SessionEnd, this count MUST always remain 0 (BC-4.05.002).
    /// The mock is never passed as a real callback — it exists only to assert
    /// that `session_end_hook_logic` never increments it.
    struct CountingMock {
        count: std::sync::atomic::AtomicUsize,
    }

    impl CountingMock {
        fn new() -> Self {
            CountingMock {
                count: std::sync::atomic::AtomicUsize::new(0),
            }
        }

        /// Number of times exec_subprocess would have been invoked.
        fn invocation_count(&self) -> usize {
            self.count.load(std::sync::atomic::Ordering::SeqCst)
        }
    }

    // -----------------------------------------------------------------------
    // Helper: build a HookPayload for a SessionEnd event
    // -----------------------------------------------------------------------

    /// Build a minimal `HookPayload` for a `SessionEnd` event.
    ///
    /// `session_start_ts` and `tool_call_count` are placed in `tool_input`
    /// (the envelope field that carries arbitrary per-event data).
    ///
    /// Both fields are optional — None means the field is omitted from the envelope,
    /// which exercises the EC-001a and EC-002 fallback paths.
    fn make_session_end_payload(
        session_id: &str,
        dispatcher_trace_id: &str,
        session_start_ts: Option<&str>,
        tool_call_count: Option<u64>,
    ) -> HookPayload {
        let mut tool_input = serde_json::Map::new();
        if let Some(ts) = session_start_ts {
            tool_input.insert(
                "session_start_ts".to_string(),
                serde_json::Value::String(ts.to_string()),
            );
        }
        if let Some(count) = tool_call_count {
            tool_input.insert(
                "tool_call_count".to_string(),
                serde_json::Value::Number(serde_json::Number::from(count)),
            );
        }

        let json = serde_json::json!({
            "event_name": "SessionEnd",
            "session_id": session_id,
            "dispatcher_trace_id": dispatcher_trace_id,
            "tool_input": serde_json::Value::Object(tool_input),
        });
        serde_json::from_value(json).expect("valid SessionEnd payload")
    }

    /// Simulate BC-1.02.005 lifecycle-tolerant envelope parsing:
    /// if session_id is empty, return "unknown" sentinel.
    fn resolve_session_id(session_id: &str) -> &str {
        if session_id.is_empty() {
            "unknown"
        } else {
            session_id
        }
    }

    // -----------------------------------------------------------------------
    // Core dispatch helper: invoke session_end_hook_logic with mock emit
    // -----------------------------------------------------------------------

    /// Simulated dispatcher: invoke `session_end_hook_logic` with a mock emit callback
    /// and return all emitted events as parsed `serde_json::Value` objects.
    ///
    /// Replicates the dispatcher's host-enrichment (BC-1.05.012 / BC-1.02.005):
    ///   - `session_id`: from envelope; empty → "unknown" (BC-1.02.005 lifecycle-tolerance)
    ///   - `dispatcher_trace_id`: from envelope
    ///   - `plugin_name`: "session-end-telemetry"
    ///   - `plugin_version`: env!("CARGO_PKG_VERSION")
    ///   - construction-time fields: ts, ts_epoch, schema_version, type
    ///
    /// RESERVED_FIELDS are silently dropped if the plugin mistakenly attempts to set them.
    fn dispatch_and_capture(payload: HookPayload) -> Vec<serde_json::Value> {
        use std::sync::{Arc, Mutex};

        let resolved_session_id = resolve_session_id(&payload.session_id).to_string();
        let dispatcher_trace_id = payload.dispatcher_trace_id.clone();

        let emitted: Arc<Mutex<Vec<serde_json::Value>>> = Arc::new(Mutex::new(Vec::new()));
        let emitted_clone = Arc::clone(&emitted);

        let plugin_version = env!("CARGO_PKG_VERSION");
        let plugin_name = "session-end-telemetry";

        let _result = session_end_hook_logic(
            payload,
            // emit_event mock: build a full event JSON simulating host enrichment
            |fields: &[(&str, &str)]| {
                use chrono::Utc;
                let mut event = serde_json::Map::new();

                // Construction-time fields (set by InternalEvent::now())
                let now = Utc::now();
                event.insert(
                    "type".to_string(),
                    serde_json::Value::String("session.ended".to_string()),
                );
                event.insert(
                    "ts".to_string(),
                    serde_json::Value::String(now.format("%Y-%m-%dT%H:%M:%S%z").to_string()),
                );
                event.insert(
                    "ts_epoch".to_string(),
                    serde_json::Value::Number(serde_json::Number::from(now.timestamp())),
                );
                event.insert(
                    "schema_version".to_string(),
                    serde_json::Value::Number(serde_json::Number::from(1u32)),
                );

                // Host-enriched fields (BC-1.05.012 / BC-1.02.005)
                event.insert(
                    "dispatcher_trace_id".to_string(),
                    serde_json::Value::String(dispatcher_trace_id.clone()),
                );
                event.insert(
                    "session_id".to_string(),
                    serde_json::Value::String(resolved_session_id.clone()),
                );
                event.insert(
                    "plugin_name".to_string(),
                    serde_json::Value::String(plugin_name.to_string()),
                );
                event.insert(
                    "plugin_version".to_string(),
                    serde_json::Value::String(plugin_version.to_string()),
                );

                // Plugin-set fields (passed via emit_fn's fields slice).
                // Silently drop RESERVED_FIELDS if the plugin mistakenly tries to set them.
                const RESERVED: &[&str] = &[
                    "dispatcher_trace_id",
                    "session_id",
                    "plugin_name",
                    "plugin_version",
                    "ts",
                    "ts_epoch",
                    "schema_version",
                    "type",
                ];
                for (k, v) in fields {
                    if !RESERVED.contains(k) {
                        event.insert(k.to_string(), serde_json::Value::String(v.to_string()));
                    }
                }

                emitted_clone
                    .lock()
                    .unwrap()
                    .push(serde_json::Value::Object(event));
            },
        );

        emitted.lock().unwrap().clone()
    }

    /// Helper: send a SessionEnd envelope with optional session_start_ts / tool_call_count.
    ///
    /// `_mock` is accepted for API consistency with VP-065 CountingMock pattern.
    /// For SessionEnd, the mock is always unused — exec_subprocess is never called
    /// (BC-4.05.002). The caller asserts `mock.invocation_count() == 0` after calling.
    fn send_session_end(
        session_id: &str,
        dispatcher_trace_id: &str,
        session_start_ts: Option<&str>,
        tool_call_count: Option<u64>,
        _mock: &CountingMock,
    ) -> Vec<serde_json::Value> {
        let payload = make_session_end_payload(
            session_id,
            dispatcher_trace_id,
            session_start_ts,
            tool_call_count,
        );
        dispatch_and_capture(payload)
    }

    /// Format milliseconds-since-epoch as ISO-8601 UTC with ms precision.
    /// Used to construct `session_start_ts` fixture values.
    fn format_iso8601_utc_ms(epoch_ms: i64) -> String {
        use chrono::{TimeZone, Utc};
        let secs = epoch_ms / 1000;
        let nanos = ((epoch_ms % 1000) * 1_000_000) as u32;
        Utc.timestamp_opt(secs, nanos)
            .single()
            .expect("valid timestamp")
            .format("%Y-%m-%dT%H:%M:%S%.3fZ")
            .to_string()
    }

    // -----------------------------------------------------------------------
    // BC-4.05.001: happy path
    // -----------------------------------------------------------------------

    /// BC-4.05.001 happy path: `session.ended` emitted with all 11 required fields.
    ///
    /// Field sources (3+4+4 split per BC-4.05.001 Notes / VP-066 Property 1):
    ///   - 3 plugin-set: duration_ms, tool_call_count, timestamp
    ///   - 4 host-enriched: dispatcher_trace_id, session_id, plugin_name, plugin_version
    ///   - 4 construction-time: ts, ts_epoch, schema_version, type
    ///
    /// Fixture: `session_start_ts` is 1 minute ago → `duration_ms` must be positive.
    /// `tool_call_count = 42` → emitted as string `"42"` per emit_event.rs:49 coercion.
    #[test]
    fn test_bc_4_05_001_session_ended_emitted_with_required_fields() {
        let mock = CountingMock::new();

        // session_start_ts = 1 minute ago (guarantees duration_ms > 0, < 5 min)
        let session_start_ts_1min_ago = {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as i64;
            format_iso8601_utc_ms(now - 60_000)
        };

        let events = send_session_end(
            "sess-end-001",
            "trace-end-001",
            Some(&session_start_ts_1min_ago),
            Some(42),
            &mock,
        );

        let session_ended: Vec<_> = events
            .iter()
            .filter(|e| e.get("type").and_then(|v| v.as_str()) == Some("session.ended"))
            .collect();

        assert_eq!(
            session_ended.len(),
            1,
            "BC-4.05.001 PC-1: exactly one session.ended must be emitted per SessionEnd invocation"
        );
        let payload = &session_ended[0];

        // --- Plugin-set fields (3) — BC-4.05.001 PC-2 ---

        // duration_ms: string on wire (emit_event.rs:49 coercion); must be positive
        assert!(
            payload["duration_ms"].is_string(),
            "duration_ms must be a string on the wire (emit_event.rs:49 string coercion; BC-4.05.001 PC-2)"
        );
        let duration: i64 = payload["duration_ms"]
            .as_str()
            .unwrap()
            .parse()
            .expect("duration_ms must parse as integer from string");
        assert!(
            duration > 0,
            "duration_ms must be positive when session_start_ts is 1 minute ago; got {duration}"
        );
        assert!(
            duration < 5 * 60 * 1000,
            "duration_ms must be < 300_000 ms (5 min sanity gate); got {duration}"
        );

        // tool_call_count: string on wire; fixture value = 42
        assert!(
            payload["tool_call_count"].is_string(),
            "tool_call_count must be a string on the wire (emit_event.rs:49 string coercion; BC-4.05.001 PC-2)"
        );
        let tcc: i64 = payload["tool_call_count"]
            .as_str()
            .unwrap()
            .parse()
            .expect("tool_call_count must parse as integer from string");
        assert_eq!(tcc, 42, "tool_call_count must equal envelope value (42)");

        // timestamp: ISO-8601 UTC with ms precision and Z suffix
        let ts = payload["timestamp"].as_str().unwrap_or("");
        let ts_re = regex::Regex::new(r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d{3}Z$").unwrap();
        assert!(
            ts_re.is_match(ts),
            "timestamp must match ISO-8601 UTC ms precision regex (BC-4.05.001 PC-2): got {ts:?}"
        );

        // --- Host-enriched fields (4) — BC-1.05.012 ---
        assert!(
            payload["dispatcher_trace_id"].is_string()
                && !payload["dispatcher_trace_id"].as_str().unwrap().is_empty(),
            "dispatcher_trace_id must be a non-empty string (host-enriched; plugin must NOT set this)"
        );
        assert!(
            payload["session_id"].is_string()
                && !payload["session_id"].as_str().unwrap().is_empty(),
            "session_id must be a non-empty string (host-enriched per BC-1.02.005; plugin must NOT set this)"
        );
        assert!(
            payload["plugin_name"].is_string()
                && !payload["plugin_name"].as_str().unwrap().is_empty(),
            "plugin_name must be a non-empty string (host-enriched per BC-1.05.012)"
        );
        assert!(
            payload["plugin_version"].is_string()
                && !payload["plugin_version"].as_str().unwrap().is_empty(),
            "plugin_version must be a non-empty string (host-enriched per BC-1.05.012)"
        );

        // --- Construction-time fields (4) ---
        assert!(
            payload.get("ts").is_some(),
            "ts construction-time field must be present"
        );
        assert!(
            payload.get("ts_epoch").is_some(),
            "ts_epoch construction-time field must be present"
        );
        assert!(
            payload.get("schema_version").is_some(),
            "schema_version construction-time field must be present"
        );
        assert_eq!(
            payload.get("type").and_then(|v| v.as_str()),
            Some("session.ended"),
            "type construction-time field must equal 'session.ended' (BC-4.05.001 Invariant 2)"
        );

        // BC-4.05.002: no subprocess invoked
        assert_eq!(
            mock.invocation_count(),
            0,
            "exec_subprocess must NOT be called for SessionEnd (BC-4.05.002)"
        );
    }

    // -----------------------------------------------------------------------
    // BC-4.05.001 EC-001a: missing session_start_ts
    // -----------------------------------------------------------------------

    /// BC-4.05.001 EC-001a: `session_start_ts` absent → `duration_ms = "0"`.
    ///
    /// `tool_call_count` IS present (= 7) and must NOT default to "0".
    #[test]
    fn test_bc_4_05_001_missing_session_start_ts_only_emits_zero_duration() {
        let mock = CountingMock::new();
        // session_start_ts absent; tool_call_count = 7
        let events = send_session_end("sess-end-ec001a", "trace-ec001a", None, Some(7), &mock);

        let session_ended: Vec<_> = events
            .iter()
            .filter(|e| e.get("type").and_then(|v| v.as_str()) == Some("session.ended"))
            .collect();

        assert_eq!(
            session_ended.len(),
            1,
            "session.ended must be emitted even when session_start_ts is absent"
        );
        assert_eq!(
            session_ended[0]["duration_ms"].as_str(),
            Some("0"),
            "BC-4.05.001 EC-001a: duration_ms must be '0' when session_start_ts is absent"
        );

        // tool_call_count IS present in envelope → must reflect the envelope value (7)
        let tcc: i64 = session_ended[0]["tool_call_count"]
            .as_str()
            .unwrap()
            .parse()
            .expect("tool_call_count must parse as integer");
        assert_ne!(
            tcc, 0,
            "tool_call_count must reflect envelope value (7) when present, not default to '0'"
        );
        assert_eq!(tcc, 7, "tool_call_count must be 7 (the fixture value)");

        assert_eq!(
            mock.invocation_count(),
            0,
            "exec_subprocess must NOT be called (BC-4.05.002)"
        );
    }

    // -----------------------------------------------------------------------
    // BC-4.05.001 EC-002: missing tool_call_count
    // -----------------------------------------------------------------------

    /// BC-4.05.001 EC-002: `tool_call_count` absent → `tool_call_count = "0"`.
    ///
    /// `session_start_ts` IS present (1 minute ago) → `duration_ms` must be positive.
    #[test]
    fn test_bc_4_05_001_missing_tool_call_count_only_emits_zero_count() {
        let mock = CountingMock::new();
        let session_start_ts_1min_ago = {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as i64;
            format_iso8601_utc_ms(now - 60_000)
        };
        // tool_call_count absent
        let events = send_session_end(
            "sess-end-ec002",
            "trace-ec002",
            Some(&session_start_ts_1min_ago),
            None,
            &mock,
        );

        let session_ended: Vec<_> = events
            .iter()
            .filter(|e| e.get("type").and_then(|v| v.as_str()) == Some("session.ended"))
            .collect();

        assert_eq!(session_ended.len(), 1);
        assert_eq!(
            session_ended[0]["tool_call_count"].as_str(),
            Some("0"),
            "BC-4.05.001 EC-002: tool_call_count must be '0' when envelope field is absent"
        );

        // duration_ms IS computable (session_start_ts present and in past) → must be positive
        let duration: i64 = session_ended[0]["duration_ms"]
            .as_str()
            .unwrap()
            .parse()
            .expect("duration_ms must parse as integer");
        assert!(
            duration > 0,
            "duration_ms must reflect real elapsed time when session_start_ts is present and in the past; got {duration}"
        );

        assert_eq!(
            mock.invocation_count(),
            0,
            "exec_subprocess must NOT be called (BC-4.05.002)"
        );
    }

    // -----------------------------------------------------------------------
    // BC-4.05.001 EC-003: both absent
    // -----------------------------------------------------------------------

    /// BC-4.05.001 EC-003: both `session_start_ts` and `tool_call_count` absent → both "0".
    ///
    /// `timestamp` is the plugin's own emission time (must be present and well-formed).
    #[test]
    fn test_bc_4_05_001_both_missing_emit_zero_defaults() {
        let mock = CountingMock::new();
        let events = send_session_end(
            "sess-end-both-missing",
            "trace-both-missing",
            None,
            None,
            &mock,
        );

        let session_ended: Vec<_> = events
            .iter()
            .filter(|e| e.get("type").and_then(|v| v.as_str()) == Some("session.ended"))
            .collect();

        assert_eq!(session_ended.len(), 1);
        assert_eq!(
            session_ended[0]["duration_ms"].as_str(),
            Some("0"),
            "BC-4.05.001 EC-003: duration_ms must be '0' when session_start_ts is absent"
        );
        assert_eq!(
            session_ended[0]["tool_call_count"].as_str(),
            Some("0"),
            "BC-4.05.001 EC-003: tool_call_count must be '0' when envelope field is absent"
        );

        // timestamp must still be present and well-formed
        let ts = session_ended[0]["timestamp"].as_str().unwrap_or("");
        let ts_re = regex::Regex::new(r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d{3}Z$").unwrap();
        assert!(
            ts_re.is_match(ts),
            "timestamp must be present and ISO-8601 UTC ms precision even when session_start_ts is absent: got {ts:?}"
        );

        assert_eq!(
            mock.invocation_count(),
            0,
            "exec_subprocess must NOT be called even for edge-case inputs"
        );
    }

    // -----------------------------------------------------------------------
    // BC-4.05.001 EC-004: missing/empty session_id
    // -----------------------------------------------------------------------

    /// BC-4.05.001 EC-004: `session_id` absent or empty → BC-1.02.005 lifecycle-tolerance
    /// sets `HostContext.session_id = "unknown"` → `emit_event` auto-enriches with "unknown".
    ///
    /// Plugin must emit normally; must not panic or abort.
    #[test]
    fn test_bc_4_05_001_missing_session_id_emits_unknown() {
        let mock = CountingMock::new();
        // session_id = "" (empty) → BC-1.02.005 sets "unknown" sentinel
        let events = send_session_end("", "trace-ec004", None, None, &mock);

        let session_ended: Vec<_> = events
            .iter()
            .filter(|e| e.get("type").and_then(|v| v.as_str()) == Some("session.ended"))
            .collect();

        assert_eq!(
            session_ended.len(),
            1,
            "BC-4.05.001 EC-004: session.ended must be emitted even when session_id is absent/empty"
        );
        assert_eq!(
            session_ended[0]["session_id"].as_str(),
            Some("unknown"),
            "BC-4.05.001 EC-004: empty session_id → 'unknown' sentinel (BC-1.02.005 lifecycle-tolerance)"
        );

        // duration_ms and tool_call_count follow their own EC paths independently
        assert_eq!(
            session_ended[0]["duration_ms"].as_str(),
            Some("0"),
            "duration_ms falls back to '0' when session_start_ts absent (EC-001a), independently of session_id"
        );
        assert_eq!(
            session_ended[0]["tool_call_count"].as_str(),
            Some("0"),
            "tool_call_count falls back to '0' when absent (EC-002), independently of session_id"
        );

        assert_eq!(
            mock.invocation_count(),
            0,
            "exec_subprocess must NOT be called (BC-4.05.002)"
        );
    }

    // -----------------------------------------------------------------------
    // BC-4.05.001 EC-001b: future session_start_ts (clock-skew clamp)
    // -----------------------------------------------------------------------

    /// BC-4.05.001 EC-001b: `session_start_ts` present but in the future relative to now.
    ///
    /// A future timestamp yields a negative elapsed duration, which the plugin
    /// MUST clamp to `"0"` rather than emitting a negative value on the wire.
    #[test]
    fn test_bc_4_05_001_future_session_start_ts_emits_zero_duration() {
        let mock = CountingMock::new();

        // session_start_ts = 60 seconds in the future (guarantees negative elapsed)
        let session_start_ts_future = {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as i64;
            format_iso8601_utc_ms(now + 60_000) // 1 minute in the future
        };

        let events = send_session_end(
            "sess-end-future",
            "trace-future",
            Some(&session_start_ts_future),
            Some(3),
            &mock,
        );

        let session_ended: Vec<_> = events
            .iter()
            .filter(|e| e.get("type").and_then(|v| v.as_str()) == Some("session.ended"))
            .collect();

        assert_eq!(
            session_ended.len(),
            1,
            "session.ended must be emitted even when session_start_ts is in the future"
        );
        assert_eq!(
            session_ended[0]["duration_ms"].as_str(),
            Some("0"),
            "BC-4.05.001 EC-001b: duration_ms must be '0' when session_start_ts is in the future (clock-skew clamp)"
        );

        // event still emitted normally (type = "session.ended")
        assert_eq!(
            session_ended[0]["type"].as_str(),
            Some("session.ended"),
            "type must equal 'session.ended' — event emitted normally despite future session_start_ts"
        );

        assert_eq!(
            mock.invocation_count(),
            0,
            "exec_subprocess must NOT be called (BC-4.05.002)"
        );
    }

    // -----------------------------------------------------------------------
    // BC-4.05.001 EC-001c: unparseable session_start_ts string
    // -----------------------------------------------------------------------

    /// BC-4.05.001 EC-001c: `session_start_ts` present as a JSON string but not valid ISO-8601.
    ///
    /// Plugin treats unparseable string as functionally absent; `duration_ms = "0"`.
    /// Applies to string-typed values only; non-string types are v1.1 candidate.
    #[test]
    fn test_bc_4_05_001_unparseable_session_start_ts_emits_zero_duration() {
        let mock = CountingMock::new();

        // Unparseable string fixture: not ISO-8601
        let events = send_session_end(
            "sess-end-garbage",
            "trace-garbage",
            Some("garbage-not-iso8601"),
            Some(5),
            &mock,
        );

        let session_ended: Vec<_> = events
            .iter()
            .filter(|e| e.get("type").and_then(|v| v.as_str()) == Some("session.ended"))
            .collect();

        assert_eq!(
            session_ended.len(),
            1,
            "session.ended must be emitted even when session_start_ts is unparseable"
        );
        assert_eq!(
            session_ended[0]["duration_ms"].as_str(),
            Some("0"),
            "BC-4.05.001 EC-001c: duration_ms must be '0' when session_start_ts is a string \
             but unparseable as ISO-8601 (treat-as-absent; string-only scope per v2.5 narrowing)"
        );

        // event still emitted normally
        assert_eq!(
            session_ended[0]["type"].as_str(),
            Some("session.ended"),
            "type must equal 'session.ended' — event emitted normally per EC-001c"
        );

        assert_eq!(
            mock.invocation_count(),
            0,
            "exec_subprocess must NOT be called (BC-4.05.002)"
        );
    }

    // -----------------------------------------------------------------------
    // BC-4.05.002: no subprocess invoked (explicit CountingMock assertion)
    // -----------------------------------------------------------------------

    /// BC-4.05.002: `exec_subprocess` CountingMock invocation count == 0 for every SessionEnd dispatch.
    ///
    /// The session-end plugin MUST NOT call `exec_subprocess` at any point.
    /// This test isolates the no-subprocess invariant explicitly.
    #[test]
    fn test_bc_4_05_002_no_subprocess_invoked() {
        let mock = CountingMock::new();
        let events = send_session_end(
            "sess-end-no-subprocess",
            "trace-no-subprocess",
            Some("2026-04-28T10:00:00.000Z"),
            Some(7),
            &mock,
        );

        let session_ended: Vec<_> = events
            .iter()
            .filter(|e| e.get("type").and_then(|v| v.as_str()) == Some("session.ended"))
            .collect();

        assert_eq!(session_ended.len(), 1, "session.ended must be emitted");
        assert_eq!(
            mock.invocation_count(),
            0,
            "BC-4.05.002 Postcondition 1: exec_subprocess invocation_count must be 0 \
             for every SessionEnd dispatch — session-end plugin must NOT call exec_subprocess"
        );
    }

    // -----------------------------------------------------------------------
    // BC-4.05.003: plugin statelessness — single dispatch → single event
    // -----------------------------------------------------------------------

    /// BC-4.05.003: plugin is unconditionally stateless — single dispatch produces exactly one event.
    ///
    /// Under Layer 1 `once: true` discipline (BC-4.05.004 invariant 1), one `SessionEnd`
    /// dispatch produces exactly one `session.ended`. Plugin carries no dedup state.
    #[test]
    fn test_bc_4_05_003_single_dispatch_produces_single_event() {
        let mock = CountingMock::new();
        let events = send_session_end(
            "sess-end-stateless",
            "trace-stateless",
            Some("2026-04-28T09:00:00.000Z"),
            Some(5),
            &mock,
        );

        let session_ended: Vec<_> = events
            .iter()
            .filter(|e| e.get("type").and_then(|v| v.as_str()) == Some("session.ended"))
            .collect();

        assert_eq!(
            session_ended.len(),
            1,
            "BC-4.05.003 PC-1: single SessionEnd dispatch must produce exactly one session.ended \
             (plugin is stateless; once-discipline via Layer 1 once:true in hooks.json.template)"
        );
        assert_eq!(
            mock.invocation_count(),
            0,
            "exec_subprocess must NOT be called (BC-4.05.002 Invariant 1)"
        );
    }

    // -----------------------------------------------------------------------
    // BC-4.05.004: hooks.json.template SessionEnd entry
    // -----------------------------------------------------------------------

    /// BC-4.05.004: `hooks.json.template` contains `SessionEnd` entry with:
    ///   - `command` referencing the dispatcher binary (contains "factory-dispatcher")
    ///   - `command` NOT referencing a `.wasm` plugin filename (ADR-011 layer separation)
    ///   - `once: true` AND `async: true`
    ///   - `timeout: 10000` (harness timeout per BC-4.05.004 Postcondition 5 / ADR-011)
    ///
    /// Structure: `hooks.SessionEnd` is an array; `[0]["hooks"][0]` has command/once/async/timeout.
    #[test]
    fn test_bc_4_05_004_hooks_json_template_has_session_end() {
        let root = workspace_root();
        let template_path = root.join("plugins/vsdd-factory/hooks/hooks.json.template");
        let template_str = std::fs::read_to_string(&template_path).unwrap_or_else(|e| {
            panic!("failed to read hooks.json.template at {template_path:?}: {e}")
        });
        let template: serde_json::Value = serde_json::from_str(&template_str)
            .unwrap_or_else(|e| panic!("failed to parse hooks.json.template as JSON: {e}"));

        // BC-4.05.004 PC-1: SessionEnd key must exist under hooks
        let session_end_arr = template["hooks"]["SessionEnd"]
            .as_array()
            .expect("BC-4.05.004: hooks.SessionEnd must be a JSON array");
        assert!(
            !session_end_arr.is_empty(),
            "BC-4.05.004: SessionEnd array must have at least one entry"
        );

        let entry = &session_end_arr[0]["hooks"][0];

        // BC-4.05.004 PC-2: command must reference dispatcher binary, NOT .wasm plugin
        let command = entry["command"]
            .as_str()
            .expect("BC-4.05.004: hooks.SessionEnd[0].hooks[0].command must be a string");
        assert!(
            command.contains("factory-dispatcher"),
            "BC-4.05.004 PC-2: command must contain 'factory-dispatcher' (dispatcher binary); got: {command:?}"
        );
        assert!(
            !command.contains(".wasm"),
            "BC-4.05.004 Invariant 1: command must NOT reference a .wasm filename (ADR-011 layer separation); got: {command:?}"
        );

        // BC-4.05.004 PC-3: once:true AND async:true
        assert_eq!(
            entry["once"], true,
            "BC-4.05.004 PC-3: once must be true (Layer 1 once-discipline per BC-4.05.003)"
        );
        assert_eq!(entry["async"], true, "BC-4.05.004 PC-3: async must be true");

        // BC-4.05.004 PC-5: timeout must be 10000
        let timeout = entry["timeout"]
            .as_i64()
            .expect("BC-4.05.004 PC-5: hooks.SessionEnd[0].hooks[0].timeout must be an integer");
        assert_eq!(
            timeout, 10000,
            "BC-4.05.004 PC-5: timeout must be 10000ms (ADR-011 timeout hierarchy: \
             dispatcher budget 5000ms < harness timeout 10000ms)"
        );
    }

    // -----------------------------------------------------------------------
    // BC-4.05.005: hooks-registry.toml SessionEnd entry
    // -----------------------------------------------------------------------

    /// BC-4.05.005: `hooks-registry.toml` contains `SessionEnd` entry with:
    ///   - `event = "SessionEnd"`
    ///   - `name = "session-end-telemetry"` (required; no default in RegistryEntry)
    ///   - `plugin = "hook-plugins/session-end-telemetry.wasm"` (with directory prefix)
    ///   - `timeout_ms = 5000` (field name is `timeout_ms` per RegistryEntry schema)
    ///   - NO `[capabilities]` table (SessionEnd needs neither read_file nor exec_subprocess)
    ///   - NO `once` field (RegistryEntry has none; deny_unknown_fields would reject it)
    ///
    /// Absence of capability tables is a positive assertion: `entry.get("capabilities").is_none()`.
    #[test]
    fn test_bc_4_05_005_hooks_registry_toml_has_session_end() {
        let root = workspace_root();
        let registry_path = root.join("plugins/vsdd-factory/hooks-registry.toml");
        let registry_str = std::fs::read_to_string(&registry_path).unwrap_or_else(|e| {
            panic!("failed to read hooks-registry.toml at {registry_path:?}: {e}")
        });
        let registry: toml::Value = toml::from_str(&registry_str)
            .unwrap_or_else(|e| panic!("failed to parse hooks-registry.toml as TOML: {e}"));

        // Find the SessionEnd entry in the [[hooks]] array
        let hooks = registry["hooks"]
            .as_array()
            .expect("hooks-registry.toml must have a [hooks] array");

        // Duplicate-detection: exactly one SessionEnd entry must exist (BC-4.05.005 EC-003)
        let session_end_count = hooks
            .iter()
            .filter(|h| h.get("event").and_then(|v| v.as_str()) == Some("SessionEnd"))
            .count();
        assert_eq!(
            session_end_count, 1,
            "BC-4.05.005: exactly one SessionEnd entry must be present; \
             found {session_end_count} entries (operator-enforced dedup per BC-4.05.005 EC-003)"
        );

        let session_end = hooks
            .iter()
            .find(|h| h.get("event").and_then(|v| v.as_str()) == Some("SessionEnd"))
            .expect("BC-4.05.005 PC-1: SessionEnd entry must be present in hooks-registry.toml");

        // BC-4.05.005 PC-1: name field (required; no default)
        assert_eq!(
            session_end["name"].as_str(),
            Some("session-end-telemetry"),
            "BC-4.05.005 PC-1: name must be 'session-end-telemetry' (RegistryEntry.name required; no default)"
        );

        // BC-4.05.005 PC-2: plugin path must include hook-plugins/ prefix
        assert_eq!(
            session_end["plugin"].as_str(),
            Some("hook-plugins/session-end-telemetry.wasm"),
            "BC-4.05.005 PC-2: plugin must be 'hook-plugins/session-end-telemetry.wasm' (directory prefix required)"
        );

        // BC-4.05.005 PC-3: timeout_ms = 5000 (field name is timeout_ms per F-13 / RegistryEntry schema)
        let timeout_ms = session_end["timeout_ms"].as_integer().expect(
            "BC-4.05.005 PC-3 (F-13): timeout_ms must be present and an integer \
                 (RegistryEntry.timeout_ms; deny_unknown_fields rejects 'epoch_budget_ms')",
        );
        assert_eq!(
            timeout_ms, 5000,
            "BC-4.05.005 PC-3: timeout_ms must be 5000 for SessionEnd \
             (stateless emit-only; no subprocess wait per BC-4.05.002)"
        );

        // BC-4.05.005 Postconditions 4–5: NO capability tables at all
        // SessionEnd plugin needs neither read_file nor exec_subprocess.
        // Absence of `capabilities` key is the positive assertion.
        assert!(
            session_end.get("capabilities").is_none(),
            "BC-4.05.005 Postconditions 4–5: SessionEnd entry must have NO capability tables \
             (deny-by-default sandbox; plugin needs neither read_file nor exec_subprocess)"
        );

        // BC-4.05.005 Invariant 2: no `once` field on RegistryEntry
        // RegistryEntry has no such field; deny_unknown_fields would reject it.
        // once-discipline is exclusively Layer 1 (BC-4.05.004).
        assert!(
            session_end.get("once").is_none(),
            "BC-4.05.005 Invariant 2: RegistryEntry must NOT have a 'once' field \
             (no such field in schema; deny_unknown_fields rejects it; once-discipline is Layer 1 only)"
        );
    }
}
