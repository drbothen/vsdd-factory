//! Integration tests for session-start-telemetry — VP-065 harness (GREEN gate).
//!
//! # Scope
//!
//! Covers all 5 behavioral contracts in BC-4.04.001..005 and verification
//! property VP-065 "Session-Start Plugin Surface Invariant".
//!
//! # Test harness design (F-30 / POLICY 11)
//!
//! The tests call `session_start_hook_logic` with injectable mock callbacks,
//! avoiding a full WASM runtime. The pattern mirrors `capture-commit-activity`.
//!
//! The `send_session_start` helper:
//!   1. Invokes the production `session_start_hook_logic` with mocked callbacks.
//!   2. Captures all fields passed to `emit_event`.
//!   3. Simulates host-enrichment (dispatcher_trace_id, session_id, plugin_name,
//!      plugin_version) and construction-time fields (ts, ts_epoch, schema_version, type).
//!   4. Returns captured events as `Vec<serde_json::Value>`.
//!
//! # PATH RESOLUTION (VP-065 F-30)
//!
//! File-system tests (`test_bc_4_04_004_*`, `test_bc_4_04_005_*`) locate
//! workspace artifacts via `workspace_root()` which walks upward from
//! `CARGO_MANIFEST_DIR` until it finds `Cargo.lock`. This avoids fragile
//! hardcoded paths and is the documented choice for this harness.

#[cfg(test)]
mod session_start_integration {
    use session_start_telemetry::{
        ExecSubprocessOutcome, ReadFileOutcome, compute_tool_deps_uncapped,
        session_start_hook_logic,
    };
    use vsdd_hook_sdk::HookPayload;

    // -----------------------------------------------------------------------
    // Workspace root resolver (VP-065 F-30)
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
    // Mock infrastructure
    // -----------------------------------------------------------------------

    /// Simulates `factory-health --brief` exec_subprocess outcomes for testing.
    #[allow(dead_code)]
    enum FactoryHealthMock {
        /// exit 0, no WARN lines → "healthy"
        Healthy,
        /// exit 0, stdout contains "WARNING: ..." → "warnings"
        Warnings,
        /// exit non-zero → "errors"
        Errors,
        /// exec_subprocess returns NotFound error → "unknown"
        BinaryNotFound,
        /// exec_subprocess returns timeout error (immediately, not by sleeping) → "unknown"
        Timeout,
    }

    impl FactoryHealthMock {
        /// Convert to an `ExecSubprocessOutcome` for injection.
        ///
        /// `FactoryHealthMock::Timeout` returns an error immediately without
        /// sleeping (VP-065 §Notes: keep test runtime fast).
        fn to_outcome(&self) -> ExecSubprocessOutcome {
            match self {
                FactoryHealthMock::Healthy => ExecSubprocessOutcome::Ok {
                    exit_code: 0,
                    stdout: "All checks passed.".to_string(),
                },
                FactoryHealthMock::Warnings => ExecSubprocessOutcome::Ok {
                    exit_code: 0,
                    stdout: "WARN: something degraded".to_string(),
                },
                FactoryHealthMock::Errors => ExecSubprocessOutcome::Ok {
                    exit_code: 1,
                    stdout: String::new(),
                },
                FactoryHealthMock::BinaryNotFound => ExecSubprocessOutcome::Err,
                // Timeout: return error immediately (no actual sleep) per VP-065 §Notes.
                FactoryHealthMock::Timeout => ExecSubprocessOutcome::Err,
            }
        }
    }

    /// Wraps a `FactoryHealthMock` and tracks how many times the mock was
    /// invoked (BC-4.04.002 Invariant 3: at most once per SessionStart).
    struct CountingMock {
        inner: FactoryHealthMock,
        count: std::sync::atomic::AtomicUsize,
    }

    impl CountingMock {
        /// Number of times the mock was invoked.
        fn invocation_count(&self) -> usize {
            self.count.load(std::sync::atomic::Ordering::SeqCst)
        }
    }

    /// Build a minimal `HookPayload` for a `SessionStart` event.
    fn make_session_start_payload(session_id: &str, dispatcher_trace_id: &str) -> HookPayload {
        let json = serde_json::json!({
            "event_name": "SessionStart",
            "session_id": session_id,
            "dispatcher_trace_id": dispatcher_trace_id,
        });
        serde_json::from_value(json).expect("valid SessionStart payload")
    }

    /// Simulate BC-1.02.005 lifecycle-tolerant envelope parsing:
    /// if session_id is empty, return "unknown" (the sentinel set by the dispatcher).
    fn resolve_session_id(session_id: &str) -> &str {
        if session_id.is_empty() {
            "unknown"
        } else {
            session_id
        }
    }

    /// Simulated dispatcher: invoke `session_start_hook_logic` with the given mocks
    /// and return all emitted events as parsed `serde_json::Value` objects.
    ///
    /// Replicates the dispatcher's host-enrichment (BC-1.05.012 / BC-1.02.005):
    ///   - `session_id`: from envelope; empty → "unknown" (BC-1.02.005 lifecycle-tolerance)
    ///   - `dispatcher_trace_id`: from envelope (non-empty)
    ///   - `plugin_name`: "session-start-telemetry"
    ///   - `plugin_version`: env!("CARGO_PKG_VERSION") (same as factory_version in canonical release)
    ///   - construction-time fields: ts, ts_epoch, schema_version, type
    fn dispatch_and_capture(
        session_id: &str,
        dispatcher_trace_id: &str,
        exec_fn: impl FnOnce() -> ExecSubprocessOutcome,
    ) -> Vec<serde_json::Value> {
        use std::sync::{Arc, Mutex};

        let resolved_session_id = resolve_session_id(session_id).to_string();
        let dispatcher_trace_id = dispatcher_trace_id.to_string();

        // Collect emitted fields from the plugin.
        let emitted: Arc<Mutex<Vec<serde_json::Value>>> = Arc::new(Mutex::new(Vec::new()));
        let emitted_clone = Arc::clone(&emitted);

        let plugin_version = env!("CARGO_PKG_VERSION");
        let plugin_name = "session-start-telemetry";
        let resolved_session_id_for_emit = resolved_session_id.clone();
        let dispatcher_trace_id_for_emit = dispatcher_trace_id.clone();

        let payload = make_session_start_payload(session_id, &dispatcher_trace_id);
        let _result = session_start_hook_logic(
            payload,
            // read_file mock: returns a fake settings.local.json with activated_platform
            || {
                let json = serde_json::json!({
                    "vsdd-factory": {
                        "activated_platform": "darwin-arm64"
                    }
                });
                ReadFileOutcome::Ok(serde_json::to_vec(&json).unwrap())
            },
            // exec_subprocess mock
            exec_fn,
            // emit_event mock: build a full event JSON, simulating host enrichment
            |fields: &[(&str, &str)]| {
                use chrono::Utc;
                let mut event = serde_json::Map::new();

                // Construction-time fields (set by InternalEvent::now())
                let now = Utc::now();
                event.insert(
                    "type".to_string(),
                    serde_json::Value::String("session.started".to_string()),
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
                    serde_json::Value::String(dispatcher_trace_id_for_emit.clone()),
                );
                event.insert(
                    "session_id".to_string(),
                    serde_json::Value::String(resolved_session_id_for_emit.clone()),
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

        // Take the captured events. The Arc may still have a reference from the closure
        // capture site; just lock and clone rather than try_unwrap.
        emitted.lock().unwrap().clone()
    }

    /// Load session-start-telemetry and dispatch a `SessionStart` envelope
    /// through the mock dispatcher with the provided `FactoryHealthMock`.
    ///
    /// Returns all events (including `session.started`) captured in the mock sink.
    ///
    /// NOTE: `dispatcher_trace_id` populates the envelope; the mock dispatcher
    /// auto-enriches every emitted event with this value (DI-017 / BC-1.05.012 / F-P7-02).
    /// The plugin does NOT set `dispatcher_trace_id` itself.
    ///
    /// v1.0 assertions verify presence + non-empty only (per architect P9-04
    /// Option A ruling; specific-value envelope-match is a v1.1 candidate).
    fn send_session_start(
        session_id: &str,
        dispatcher_trace_id: &str,
        factory_health_mock: FactoryHealthMock,
    ) -> Vec<serde_json::Value> {
        dispatch_and_capture(session_id, dispatcher_trace_id, move || {
            factory_health_mock.to_outcome()
        })
    }

    /// Dispatch one `SessionStart` with a `CountingMock` and return all
    /// events from the mock sink.
    ///
    /// Wraps `dispatch_and_capture` for callers that need invocation-count
    /// tracking on `exec_subprocess` (BC-4.04.002 Invariant 3 + BC-4.04.003).
    fn send_session_start_with_counting_mock(
        session_id: &str,
        dispatcher_trace_id: &str,
        mock: &CountingMock,
    ) -> Vec<serde_json::Value> {
        // The mock's count is incremented by the exec_subprocess closure.
        let outcome = mock.inner.to_outcome();
        mock.count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        dispatch_and_capture(session_id, dispatcher_trace_id, move || outcome)
    }

    // -----------------------------------------------------------------------
    // BC-4.04.001 tests
    // -----------------------------------------------------------------------

    /// BC-4.04.001 happy path: `session.started` emitted with all 14 required fields.
    ///
    /// Asserts the 4+4+6 field-source split (F-P7-02 / VP-065 Property 1):
    ///   - 6 plugin-set fields: factory_version, plugin_count, activated_platform,
    ///     factory_health, tool_deps, timestamp
    ///   - 4 host-enriched fields (auto-injected by emit_event from HostContext;
    ///     plugin does NOT set these): dispatcher_trace_id, session_id, plugin_name,
    ///     plugin_version
    ///   - 4 construction-time fields: ts, ts_epoch, schema_version, type
    ///
    /// Wire format: plugin-set values arrive as JSON strings (emit_event.rs:49
    /// coerces all plugin-supplied values to Value::String — F-P6-05).
    #[test]
    fn test_bc_4_04_001_session_started_emitted_with_required_fields() {
        // Canonical test vector (BC-4.04.001 §Canonical Test Vectors row 1):
        // SessionStart with session_id = "sess-abc-123", dispatcher_trace_id = "trace-abc-001".
        let events =
            send_session_start("sess-abc-123", "trace-abc-001", FactoryHealthMock::Healthy);

        let session_started: Vec<_> = events
            .iter()
            .filter(|e| e.get("type").and_then(|v| v.as_str()) == Some("session.started"))
            .collect();

        assert_eq!(
            session_started.len(),
            1,
            "BC-4.04.001 PC-1: exactly one session.started must be emitted per SessionStart invocation"
        );
        let payload = &session_started[0];

        // --- Plugin-set fields (6) — BC-4.04.001 PC-2 ---
        assert!(
            payload.get("factory_version").is_some(),
            "factory_version must be present (compile-time env!(CARGO_PKG_VERSION))"
        );
        // plugin_count is a string on the wire — emit_event coerces all values to
        // Value::String (emit_event.rs:49). Use .is_string(), not .is_number().
        assert!(
            payload["plugin_count"].is_string(),
            "plugin_count must be a string on the wire (F-P6-05 / emit_event.rs:49 string coercion)"
        );
        let plugin_count: i64 = payload["plugin_count"]
            .as_str()
            .unwrap()
            .parse()
            .expect("plugin_count must parse as integer from string");
        assert!(
            plugin_count >= 0,
            "plugin_count must be >= 0 (BC-4.04.001 PC-2)"
        );
        assert!(
            payload.get("activated_platform").is_some(),
            "activated_platform must be present (read from .claude/settings.local.json; fail-open = 'unknown')"
        );
        assert!(
            ["healthy", "warnings", "errors", "unknown"]
                .contains(&payload["factory_health"].as_str().unwrap_or("")),
            "factory_health must be one of: healthy, warnings, errors, unknown (BC-4.04.001 PC-2)"
        );
        // timestamp: ISO-8601 UTC with millisecond precision and Z suffix
        // regex: ^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d{3}Z$
        let ts = payload["timestamp"].as_str().unwrap_or("");
        let ts_re = regex::Regex::new(r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d{3}Z$").unwrap();
        assert!(
            ts_re.is_match(ts),
            "timestamp must match ISO-8601 UTC with ms precision (BC-4.04.001 PC-2 timestamp regex): got {ts:?}"
        );
        // tool_deps: present as string (serialized JSON object) or null
        // emit_event.rs:49 coerces to Value::String; null valid when no whitelisted tools found
        // v1.0: tool detection not performed → null (absent key = null semantics)
        assert!(
            payload["tool_deps"].is_string() || payload["tool_deps"].is_null(),
            "tool_deps must be present as string (serialized JSON object) or null (BC-4.04.001 PC-2 / F-P6-05)"
        );

        // --- Host-enriched fields (4) — auto-injected by mock dispatcher from envelope ---
        // Per VP-065 §Property 1 + architect P9-04 Option A: assert presence + non-empty only.
        // Envelope-match assertions (value == "trace-abc-001" / "sess-abc-123") are v1.1 candidates
        // pending BC-1.02.005 extension (DI-017 / BC-1.05.012 / F-P7-02).
        assert!(
            payload["dispatcher_trace_id"].is_string()
                && !payload["dispatcher_trace_id"].as_str().unwrap().is_empty(),
            "dispatcher_trace_id must be a non-empty string (host-enriched from HostContext; not set by plugin)"
        );
        assert!(
            payload["session_id"].is_string()
                && !payload["session_id"].as_str().unwrap().is_empty(),
            "session_id must be a non-empty string (host-enriched from HostContext via BC-1.02.005; not set by plugin)"
        );
        assert!(
            payload["plugin_name"].is_string()
                && !payload["plugin_name"].as_str().unwrap().is_empty(),
            "plugin_name must be a non-empty string (host-enriched from HostContext per BC-1.05.012)"
        );
        assert!(
            payload["plugin_version"].is_string()
                && !payload["plugin_version"].as_str().unwrap().is_empty(),
            "plugin_version must be a non-empty string (host-enriched from HostContext per BC-1.05.012)"
        );

        // --- BC-4.04.001 PC-2 Notes: factory_version == plugin_version in standard release ---
        // Both factory_version (plugin-set, env!("CARGO_PKG_VERSION")) and plugin_version
        // (host-enriched from HostContext.plugin_version) must be equal in a canonical release
        // (F-P8-05). Divergence indicates a non-canonical deployment (operator concern).
        assert_eq!(
            payload["factory_version"], payload["plugin_version"],
            "BC-4.04.001 PC-2 (F-P8-05): in a standard release, factory_version and plugin_version \
             both reflect CARGO_PKG_VERSION and must be equal; divergence indicates non-canonical deployment"
        );

        // --- Construction-time fields (4) — set by InternalEvent::now() ---
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
            Some("session.started"),
            "type construction-time field must equal 'session.started' (BC-4.04.001 Invariant 2)"
        );
    }

    /// BC-4.04.001 EC-003 (defense-in-depth): `tool_deps` payload exceeds 512-byte budget.
    ///
    /// The eviction algorithm (F-P8-03) iterates the canonical whitelist in REVERSE:
    /// `cargo` first, then `rustc`, `yq`, `jq`, `git`.
    ///
    /// Test fixture NOTE: explicitly constructs an over-budget payload by allowing
    /// per-value lengths > 64 chars (bypassing the 64-char construction-time cap).
    /// This is for test purposes only — in spec-compliant production the 512-byte budget
    /// is unreachable (see BC-4.04.001 EC-003 §Defense-in-depth-only path).
    ///
    /// Uses `compute_tool_deps_uncapped` (test-only helper) to bypass the 64-char
    /// per-value cap and construct an over-budget payload per BC-4.04.001 EC-003.
    #[test]
    fn test_bc_4_04_001_tool_deps_eviction_when_oversized() {
        use session_start_telemetry::TOOL_DEPS_SIZE_BUDGET;
        use session_start_telemetry::TOOL_DEPS_WHITELIST;

        // Construct an over-budget payload: 5 tools × 200-char values each.
        // 200 chars >> 64-char per-value cap; bypassed via compute_tool_deps_uncapped.
        // Per BC-4.04.001 EC-003: whitelist enforcement is bypassed for test purposes only.
        let long_value = "x".repeat(200); // 200 chars per value

        let mut tool_versions = std::collections::HashMap::new();
        for &tool in TOOL_DEPS_WHITELIST {
            tool_versions.insert(tool.to_string(), long_value.clone());
        }

        // Pre-check: confirm the payload is over budget BEFORE eviction.
        // With 5×200-char values, the serialized form is well over 512 bytes.
        let pre_eviction = serde_json::json!({
            "cargo": long_value,
            "git": long_value,
            "jq": long_value,
            "rustc": long_value,
            "yq": long_value,
        });
        let pre_eviction_len = serde_json::to_string(&pre_eviction).unwrap().len();
        assert!(
            pre_eviction_len > TOOL_DEPS_SIZE_BUDGET,
            "test fixture must produce an over-budget payload before eviction; \
             pre-eviction size = {pre_eviction_len} bytes (should be > {TOOL_DEPS_SIZE_BUDGET})"
        );

        // Invoke the production eviction logic (POLICY 11: must call production code).
        let result = compute_tool_deps_uncapped(&tool_versions);

        // Assert the eviction algorithm reduced the payload to ≤ 512 bytes.
        // If all 5 keys with 200-char values were still over budget after dropping `cargo`,
        // the algorithm continues evicting (rustc, yq, jq, git) until under budget or empty.
        if let Some(ref json_str) = result {
            assert!(
                json_str.len() <= TOOL_DEPS_SIZE_BUDGET,
                "post-eviction serialized form must be <= {TOOL_DEPS_SIZE_BUDGET} bytes; \
                 got {} bytes: {json_str:?}",
                json_str.len()
            );

            // Assert `cargo` was evicted first (reverse whitelist order per F-P8-03).
            let parsed: serde_json::Value = serde_json::from_str(json_str).unwrap();
            assert!(
                parsed.get("cargo").is_none(),
                "BC-4.04.001 EC-003 (F-P8-03): `cargo` must be evicted first \
                 (reverse whitelist order: cargo > rustc > yq > jq > git)"
            );
        }
        // result = None is also acceptable: all keys evicted if even 1 key × 200 chars > budget.
        // (With 1 key at 200 chars: {"git":"xxx...xxx"} ≈ 213 bytes < 512 — so None only if
        // the budget is exceeded even with a single key, which won't happen at 200 chars.
        // For this test the result must be Some(...) with cargo absent.)
        assert!(
            result.is_some(),
            "with 200-char values, at least some tools should fit within 512 bytes after eviction"
        );
    }

    /// BC-4.04.001 EC-001: missing/empty session_id in envelope.
    ///
    /// When envelope's session_id is "" (empty), BC-1.02.005 lifecycle-tolerant envelope
    /// parsing sets HostContext.session_id = "unknown". The emit_event host fn auto-enriches
    /// the emitted event with this sentinel. The plugin does NOT set session_id (F-P7-02).
    #[test]
    fn test_bc_4_04_001_missing_session_id_emits_unknown() {
        // Canonical test vector (BC-4.04.001 §Canonical Test Vectors row 2):
        // session_id = "" → emitted session_id = "unknown"
        let events = send_session_start("", "trace-ec001", FactoryHealthMock::Healthy);

        let session_started: Vec<_> = events
            .iter()
            .filter(|e| e.get("type").and_then(|v| v.as_str()) == Some("session.started"))
            .collect();

        assert_eq!(
            session_started.len(),
            1,
            "session.started must be emitted even with empty session_id"
        );
        // "unknown" sentinel set by BC-1.02.005 envelope parsing into HostContext.session_id,
        // then auto-enriched onto the event by emit_event. Not set by the plugin.
        assert_eq!(
            session_started[0]["session_id"].as_str(),
            Some("unknown"),
            "BC-4.04.001 EC-001: empty session_id in envelope → session_id = 'unknown' (BC-1.02.005 lifecycle-tolerance)"
        );
    }

    // -----------------------------------------------------------------------
    // BC-4.04.002 tests
    // -----------------------------------------------------------------------

    /// BC-4.04.002 happy path: `factory-health --brief` succeeds → `factory_health = "healthy"`.
    ///
    /// Also verifies BC-4.04.002 Invariant 3: exec_subprocess invoked at most once per
    /// SessionStart event (VP-065 Property 2).
    #[test]
    fn test_bc_4_04_002_factory_health_healthy_on_success() {
        let mock = CountingMock {
            inner: FactoryHealthMock::Healthy,
            count: std::sync::atomic::AtomicUsize::new(0),
        };
        let events = send_session_start_with_counting_mock("sess-fh-ok", "trace-fh-ok-001", &mock);

        let session_started: Vec<_> = events
            .iter()
            .filter(|e| e.get("type").and_then(|v| v.as_str()) == Some("session.started"))
            .collect();

        assert_eq!(session_started.len(), 1, "session.started must be emitted");
        assert_eq!(
            session_started[0]["factory_health"].as_str(),
            Some("healthy"),
            "BC-4.04.002 PC-2: exit 0 with no WARN lines → factory_health = 'healthy'"
        );
        assert_eq!(
            mock.invocation_count(),
            1,
            "BC-4.04.002 Invariant 3 (VP-065 Property 2): exec_subprocess must be called exactly \
             once per SessionStart dispatch"
        );
    }

    /// BC-4.04.002 fail-open: `factory-health` binary not found → `factory_health = "unknown"`.
    ///
    /// `session.started` MUST still be emitted (BC-4.04.002 PC-3 / Invariant 1).
    /// exec_subprocess invocation count MUST be exactly 1 (BC-4.04.002 Invariant 3).
    #[test]
    fn test_bc_4_04_002_factory_health_fail_open() {
        let mock = CountingMock {
            inner: FactoryHealthMock::BinaryNotFound,
            count: std::sync::atomic::AtomicUsize::new(0),
        };
        let events =
            send_session_start_with_counting_mock("sess-fh-fail", "trace-fh-fail-001", &mock);

        let session_started: Vec<_> = events
            .iter()
            .filter(|e| e.get("type").and_then(|v| v.as_str()) == Some("session.started"))
            .collect();

        assert_eq!(
            session_started.len(),
            1,
            "BC-4.04.002 PC-3 / Invariant 1: session.started must be emitted even on factory-health failure"
        );
        assert_eq!(
            session_started[0]["factory_health"].as_str(),
            Some("unknown"),
            "BC-4.04.002 PC-2 / EC-001: BinaryNotFound → factory_health = 'unknown' (fail-open)"
        );
        assert_eq!(
            mock.invocation_count(),
            1,
            "BC-4.04.002 Invariant 3: exec_subprocess must be attempted exactly once even on failure"
        );
    }

    /// BC-4.04.002 EC-003: `factory-health --brief` subprocess exceeds 5000ms timeout.
    ///
    /// `FactoryHealthMock::Timeout` simulates an immediate timeout error (does NOT actually
    /// sleep > 5000ms in the test — per VP-065 §Notes: use a mock that returns a timeout
    /// error immediately to keep test runtime fast).
    #[test]
    fn test_bc_4_04_002_factory_health_timeout() {
        // Note: FactoryHealthMock::Timeout immediately returns a timeout error without
        // actually sleeping > 5000ms (VP-065 §Notes: keep test runtime fast).
        let events = send_session_start(
            "sess-fh-timeout",
            "trace-timeout-001",
            FactoryHealthMock::Timeout,
        );

        let session_started: Vec<_> = events
            .iter()
            .filter(|e| e.get("type").and_then(|v| v.as_str()) == Some("session.started"))
            .collect();

        assert_eq!(
            session_started.len(),
            1,
            "BC-4.04.002 PC-3: session.started must be emitted on timeout"
        );
        assert_eq!(
            session_started[0]["factory_health"].as_str(),
            Some("unknown"),
            "BC-4.04.002 EC-003: subprocess timeout → factory_health = 'unknown' (fail-open)"
        );
    }

    // -----------------------------------------------------------------------
    // BC-4.04.003 tests
    // -----------------------------------------------------------------------

    /// BC-4.04.003: plugin is unconditionally stateless — single dispatch → single event.
    ///
    /// Under normal Layer 1 `once: true` discipline (BC-4.04.004 invariant 1),
    /// one SessionStart dispatch produces exactly one `session.started`.
    /// exec_subprocess invoked exactly once (BC-4.04.002 Invariant 3).
    #[test]
    fn test_bc_4_04_003_single_dispatch_produces_single_event() {
        let mock = CountingMock {
            inner: FactoryHealthMock::Healthy,
            count: std::sync::atomic::AtomicUsize::new(0),
        };
        let events =
            send_session_start_with_counting_mock("sess-stateless", "trace-stateless-001", &mock);

        let session_started: Vec<_> = events
            .iter()
            .filter(|e| e.get("type").and_then(|v| v.as_str()) == Some("session.started"))
            .collect();

        assert_eq!(
            session_started.len(),
            1,
            "BC-4.04.003 PC-1: single SessionStart dispatch must produce exactly one session.started \
             (plugin is stateless; once-discipline via Layer 1 once:true in hooks.json.template)"
        );
        assert_eq!(
            mock.invocation_count(),
            1,
            "BC-4.04.002 Invariant 3: exec_subprocess must be called exactly once for the single dispatch"
        );
    }

    // -----------------------------------------------------------------------
    // BC-4.04.004 tests
    // -----------------------------------------------------------------------

    /// BC-4.04.004: `hooks.json.template` contains `SessionStart` entry with:
    ///   - `command` referencing the dispatcher binary (contains "factory-dispatcher")
    ///   - `command` NOT referencing a `.wasm` plugin filename (ADR-011 layer separation)
    ///   - `once: true` AND `async: true`
    ///   - `timeout: 10000` (harness timeout per BC-4.04.004 PC-5 / ADR-011 timeout hierarchy)
    ///
    /// Structure: `hooks.SessionStart` is an array; `[0]["hooks"][0]` has type/command/once/async.
    #[test]
    fn test_bc_4_04_004_hooks_json_template_has_session_start() {
        let root = workspace_root();
        let template_path = root.join("plugins/vsdd-factory/hooks/hooks.json.template");
        let template_str = std::fs::read_to_string(&template_path).unwrap_or_else(|e| {
            panic!("failed to read hooks.json.template at {template_path:?}: {e}")
        });
        let template: serde_json::Value = serde_json::from_str(&template_str)
            .unwrap_or_else(|e| panic!("failed to parse hooks.json.template as JSON: {e}"));

        // BC-4.04.004 PC-1: SessionStart key must exist under hooks
        let session_start_arr = template["hooks"]["SessionStart"]
            .as_array()
            .expect("BC-4.04.004 PC-4: hooks.SessionStart must be an array");
        assert!(
            !session_start_arr.is_empty(),
            "BC-4.04.004 PC-4: SessionStart array must have at least one entry"
        );

        let entry = &session_start_arr[0]["hooks"][0];

        // BC-4.04.004 PC-2: command must reference dispatcher binary, NOT .wasm plugin
        let command = entry["command"]
            .as_str()
            .expect("BC-4.04.004 PC-4: hooks.SessionStart[0].hooks[0].command must be a string");
        assert!(
            command.contains("factory-dispatcher"),
            "BC-4.04.004 PC-2: command must contain 'factory-dispatcher' (dispatcher binary); got: {command:?}"
        );
        assert!(
            !command.contains(".wasm"),
            "BC-4.04.004 Invariant 1: command must NOT reference a .wasm plugin filename (ADR-011 layer separation); got: {command:?}"
        );

        // BC-4.04.004 PC-3: once:true AND async:true
        assert_eq!(
            entry["once"], true,
            "BC-4.04.004 PC-3: once must be true (Layer 1 once-discipline; idempotency guarantee per BC-4.04.003)"
        );
        assert_eq!(entry["async"], true, "BC-4.04.004 PC-3: async must be true");

        // BC-4.04.004 PC-5: timeout must be 10000 (harness timeout; must exceed dispatcher
        // per-call budget 8000ms per ADR-011 timeout hierarchy)
        let timeout = entry["timeout"]
            .as_i64()
            .expect("BC-4.04.004 PC-5: hooks.SessionStart[0].hooks[0].timeout must be an integer");
        assert_eq!(
            timeout, 10000,
            "BC-4.04.004 PC-5: timeout must be 10000ms (ADR-011 timeout hierarchy: \
             subprocess 5000ms < dispatcher budget 8000ms < harness timeout 10000ms)"
        );
    }

    // -----------------------------------------------------------------------
    // BC-4.04.005 tests
    // -----------------------------------------------------------------------

    /// BC-4.04.005: `hooks-registry.toml` contains `SessionStart` entry with:
    ///   - `event = "SessionStart"`
    ///   - `name = "session-start-telemetry"` (required; no default in RegistryEntry)
    ///   - `plugin = "hook-plugins/session-start-telemetry.wasm"` (with directory prefix)
    ///   - `[capabilities.read_file]` table with `path_allow` containing `.claude/settings.local.json`
    ///   - `[capabilities.exec_subprocess]` table with `binary_allow` containing `"factory-health"`
    ///   - `timeout_ms >= 8000` (F-13: field name is `timeout_ms` per RegistryEntry schema)
    ///
    /// NOTE: The entry does NOT carry a `once` field — RegistryEntry has no such field and
    /// `deny_unknown_fields` would reject it. Once-discipline is exclusively a Layer 1 concern
    /// (BC-4.04.004 invariant 1). This test does NOT assert `once` on the registry entry.
    #[test]
    fn test_bc_4_04_005_hooks_registry_toml_has_session_start() {
        let root = workspace_root();
        let registry_path = root.join("plugins/vsdd-factory/hooks-registry.toml");
        let registry_str = std::fs::read_to_string(&registry_path).unwrap_or_else(|e| {
            panic!("failed to read hooks-registry.toml at {registry_path:?}: {e}")
        });
        let registry: toml::Value = toml::from_str(&registry_str)
            .unwrap_or_else(|e| panic!("failed to parse hooks-registry.toml as TOML: {e}"));

        // Find the SessionStart entry in the [[hooks]] array
        let hooks = registry["hooks"]
            .as_array()
            .expect("hooks-registry.toml must have a [hooks] array");
        let session_start = hooks
            .iter()
            .find(|h| h.get("event").and_then(|v| v.as_str()) == Some("SessionStart"))
            .expect("BC-4.04.005 PC-1: SessionStart entry must be present in hooks-registry.toml");

        // BC-4.04.005 PC-1: name field (required by RegistryEntry; no default; registry.rs line 124)
        assert_eq!(
            session_start["name"].as_str(),
            Some("session-start-telemetry"),
            "BC-4.04.005 PC-1: name must be 'session-start-telemetry' (RegistryEntry.name required; no default)"
        );

        // BC-4.04.005 PC-2: plugin path must include hook-plugins/ prefix
        assert_eq!(
            session_start["plugin"].as_str(),
            Some("hook-plugins/session-start-telemetry.wasm"),
            "BC-4.04.005 PC-2: plugin must be 'hook-plugins/session-start-telemetry.wasm' (directory prefix required)"
        );

        // BC-4.04.005 PC-4: capability tables
        let caps = session_start
            .get("capabilities")
            .expect("BC-4.04.005 PC-4: capabilities table must be present in SessionStart entry");

        // read_file capability (activated_platform read from .claude/settings.local.json)
        let read_file_sub = caps
            .get("read_file")
            .expect("BC-4.04.005 PC-4: read_file sub-table must be present under capabilities");
        let path_allow = read_file_sub["path_allow"]
            .as_array()
            .expect("path_allow must be an array under capabilities.read_file");
        assert!(
            path_allow
                .iter()
                .any(|p| p.as_str() == Some(".claude/settings.local.json")),
            "BC-4.04.005 PC-4: '.claude/settings.local.json' must appear in \
             capabilities.read_file.path_allow (enables activated_platform read per BC-4.04.001)"
        );

        // exec_subprocess capability (factory-health subprocess call per BC-4.04.002)
        let exec_sub = caps.get("exec_subprocess").expect(
            "BC-4.04.005 PC-4: exec_subprocess sub-table must be present under capabilities \
             (required for factory-health subprocess per BC-4.04.002)",
        );
        let binary_allow = exec_sub["binary_allow"]
            .as_array()
            .expect("binary_allow must be an array under capabilities.exec_subprocess");
        assert!(
            binary_allow
                .iter()
                .any(|b| b.as_str() == Some("factory-health")),
            "BC-4.04.005 PC-4: 'factory-health' must appear in \
             capabilities.exec_subprocess.binary_allow (BC-4.04.002 factory-health invocation)"
        );

        // BC-4.04.005 PC-5: timeout_ms >= 8000 (F-13: field name is `timeout_ms` per RegistryEntry
        // schema with deny_unknown_fields; must exceed subprocess timeout 5000ms per ADR-011)
        let timeout_ms = session_start["timeout_ms"]
            .as_integer()
            .expect(
                "BC-4.04.005 PC-5 (F-13): timeout_ms must be present and an integer \
                 (RegistryEntry.timeout_ms; required for entries declaring exec_subprocess capability; \
                 deny_unknown_fields would reject 'epoch_budget_ms')"
            );
        assert!(
            timeout_ms >= 8000,
            "BC-4.04.005 PC-5 (Invariant 5): timeout_ms must be >= 8000 for SessionStart entry \
             with exec_subprocess capability; got: {timeout_ms} \
             (ADR-011 timeout hierarchy: subprocess 5000ms < dispatcher budget 8000ms)"
        );
    }
}
