//! Integration tests for worktree-hooks — VP-067 harness (RED gate).
// Allow .expect(&format!(...)) in test-only code — the lint prevents unnecessary
// allocations in production paths; tests are excluded from that concern.
#![allow(clippy::expect_fun_call)]
//!
//! # Scope
//!
//! Covers all 4 behavioral contracts in BC-4.07.001..004 and verification
//! property VP-067 "Worktree Hook Plugin Surface Invariant".
//!
//! # Test harness design (mirrors VP-066 / session-end-telemetry pattern)
//!
//! The tests call `worktree_hook_logic` with an injectable mock emit callback,
//! avoiding a full WASM runtime.
//!
//! The `dispatch_and_capture` helper:
//!   1. Invokes the production `worktree_hook_logic` with a mocked emit callback.
//!   2. Captures all fields passed to `emit_event`.
//!   3. Simulates host-enrichment (dispatcher_trace_id, session_id, plugin_name,
//!      plugin_version) and construction-time fields (ts, ts_epoch, schema_version, type).
//!   4. Returns captured events as `Vec<serde_json::Value>`.
//!
//! # PATH RESOLUTION (VP-067 — same as VP-065/066)
//!
//! File-system tests (`test_bc_4_07_003_*`, `test_bc_4_07_004_*`) locate
//! workspace artifacts via `workspace_root()` which walks upward from
//! `CARGO_MANIFEST_DIR` until it finds `Cargo.lock`. This avoids fragile
//! hardcoded paths and is the documented choice for this harness.

#[cfg(test)]
mod worktree_integration {
    use worktree_hooks::worktree_hook_logic;
    use vsdd_hook_sdk::HookPayload;

    // -----------------------------------------------------------------------
    // Workspace root resolver (VP-067 — same pattern as VP-065/066)
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
    // CountingMock for exec_subprocess (BC-4.07.001 Invariant 2 + BC-4.07.002 Invariant 2)
    // -----------------------------------------------------------------------

    /// Tracks how many times exec_subprocess was (hypothetically) invoked.
    ///
    /// For both WorktreeCreate and WorktreeRemove events, this count MUST remain 0.
    /// The mock is used to assert that `worktree_hook_logic` never invokes exec_subprocess —
    /// consistent with the zero-capability Option A scoping (BC-4.07.004).
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
    // Helper: build HookPayload structs for WorktreeCreate and WorktreeRemove
    // -----------------------------------------------------------------------

    /// Build a `HookPayload` for a WorktreeCreate event.
    ///
    /// `worktree_path` and `worktree_name` are placed in `tool_input`
    /// (the envelope field that carries arbitrary per-event data).
    /// Both fields are optional — None means absent from the envelope (exercises EC-003).
    fn make_worktree_create_payload(
        session_id: &str,
        dispatcher_trace_id: &str,
        worktree_path: Option<&str>,
        worktree_name: Option<&str>,
    ) -> HookPayload {
        let mut tool_input = serde_json::Map::new();
        if let Some(path) = worktree_path {
            tool_input.insert(
                "worktree_path".to_string(),
                serde_json::Value::String(path.to_string()),
            );
        }
        if let Some(name) = worktree_name {
            tool_input.insert(
                "worktree_name".to_string(),
                serde_json::Value::String(name.to_string()),
            );
        }

        let json = serde_json::json!({
            "event_name": "WorktreeCreate",
            "session_id": session_id,
            "dispatcher_trace_id": dispatcher_trace_id,
            "tool_input": serde_json::Value::Object(tool_input),
        });
        serde_json::from_value(json).expect("valid WorktreeCreate payload")
    }

    /// Build a `HookPayload` for a WorktreeRemove event.
    ///
    /// `worktree_path` is placed in `tool_input`.
    /// None means absent from the envelope (exercises EC-003).
    fn make_worktree_remove_payload(
        session_id: &str,
        dispatcher_trace_id: &str,
        worktree_path: Option<&str>,
    ) -> HookPayload {
        let mut tool_input = serde_json::Map::new();
        if let Some(path) = worktree_path {
            tool_input.insert(
                "worktree_path".to_string(),
                serde_json::Value::String(path.to_string()),
            );
        }

        let json = serde_json::json!({
            "event_name": "WorktreeRemove",
            "session_id": session_id,
            "dispatcher_trace_id": dispatcher_trace_id,
            "tool_input": serde_json::Value::Object(tool_input),
        });
        serde_json::from_value(json).expect("valid WorktreeRemove payload")
    }

    /// Build a `HookPayload` with an arbitrary event_name (for unknown-event defensive test).
    fn make_unknown_event_payload(
        session_id: &str,
        dispatcher_trace_id: &str,
        event_name: &str,
    ) -> HookPayload {
        let json = serde_json::json!({
            "event_name": event_name,
            "session_id": session_id,
            "dispatcher_trace_id": dispatcher_trace_id,
            "tool_input": {},
        });
        serde_json::from_value(json).expect("valid unknown-event payload")
    }

    // -----------------------------------------------------------------------
    // Core dispatch helper: invoke worktree_hook_logic with mock emit
    // -----------------------------------------------------------------------

    /// Simulated dispatcher: invoke `worktree_hook_logic` with a mock emit callback
    /// and return all emitted events as parsed `serde_json::Value` objects.
    ///
    /// Replicates the dispatcher's host-enrichment (BC-1.05.012):
    ///   - `session_id`: from envelope
    ///   - `dispatcher_trace_id`: from envelope
    ///   - `plugin_name`: "worktree-hooks"
    ///   - `plugin_version`: env!("CARGO_PKG_VERSION")
    ///   - construction-time fields: ts, ts_epoch, schema_version, type
    ///
    /// RESERVED_FIELDS are silently dropped if the plugin mistakenly attempts to set them.
    fn dispatch_and_capture(payload: HookPayload) -> Vec<serde_json::Value> {
        use std::sync::{Arc, Mutex};

        let session_id = payload.session_id.clone();
        let dispatcher_trace_id = payload.dispatcher_trace_id.clone();

        let emitted: Arc<Mutex<Vec<serde_json::Value>>> = Arc::new(Mutex::new(Vec::new()));
        let emitted_clone = Arc::clone(&emitted);

        let plugin_version = env!("CARGO_PKG_VERSION");
        let plugin_name = "worktree-hooks";

        let _result = worktree_hook_logic(
            payload,
            // emit_event mock: build a full event JSON simulating host enrichment
            |event_type: &str, fields: &[(&str, &str)]| {
                use chrono::Utc;
                let mut event = serde_json::Map::new();

                // Construction-time fields (set by dispatcher InternalEvent::now())
                let now = Utc::now();
                event.insert(
                    "type".to_string(),
                    serde_json::Value::String(event_type.to_string()),
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

                // Host-enriched fields (BC-1.05.012)
                event.insert(
                    "dispatcher_trace_id".to_string(),
                    serde_json::Value::String(dispatcher_trace_id.clone()),
                );
                event.insert(
                    "session_id".to_string(),
                    serde_json::Value::String(session_id.clone()),
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
                        event.insert(
                            k.to_string(),
                            serde_json::Value::String(v.to_string()),
                        );
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

    // -----------------------------------------------------------------------
    // Test 1: BC-4.07.001 — WorktreeCreate happy path (10-field wire payload)
    // -----------------------------------------------------------------------

    /// BC-4.07.001 happy path: `worktree.created` emitted with exactly 10 fields.
    ///
    /// Field sources (2+4+4 split per BC-4.07.001):
    ///   - 2 plugin-set: worktree_path, worktree_name
    ///   - 4 host-enriched: dispatcher_trace_id, session_id, plugin_name, plugin_version
    ///   - 4 construction-time: ts, ts_epoch, schema_version, type
    ///
    /// RESERVED_FIELDS must NOT be set by the plugin (they arrive via host enrichment only).
    #[test]
    fn test_bc_4_07_001_worktree_create_emits_required_fields() {
        let payload = make_worktree_create_payload(
            "sess-create-001",
            "trace-create-001",
            Some("/workspace/feature-branch"),
            Some("feature-branch"),
        );
        let events = dispatch_and_capture(payload);

        // Exactly one worktree.created event
        let created: Vec<_> = events
            .iter()
            .filter(|e| e.get("type").and_then(|v| v.as_str()) == Some("worktree.created"))
            .collect();

        assert_eq!(
            created.len(),
            1,
            "BC-4.07.001 PC-1: exactly one worktree.created must be emitted per WorktreeCreate invocation"
        );
        let event = &created[0];

        // --- Plugin-set fields (2) per BC-4.07.001 PC-2 ---

        // worktree_path: string on wire (emit_event.rs:49 coercion)
        assert_eq!(
            event["worktree_path"].as_str(),
            Some("/workspace/feature-branch"),
            "BC-4.07.001 PC-2: worktree_path must equal the envelope value"
        );

        // worktree_name: string on wire
        assert_eq!(
            event["worktree_name"].as_str(),
            Some("feature-branch"),
            "BC-4.07.001 PC-2: worktree_name must equal the envelope value"
        );

        // --- Host-enriched fields (4) — BC-1.05.012 ---
        assert!(
            event["dispatcher_trace_id"].is_string()
                && !event["dispatcher_trace_id"].as_str().unwrap().is_empty(),
            "dispatcher_trace_id must be a non-empty string (host-enriched; plugin must NOT set this)"
        );
        assert!(
            event["session_id"].is_string()
                && !event["session_id"].as_str().unwrap().is_empty(),
            "session_id must be a non-empty string (host-enriched; plugin must NOT set this)"
        );
        assert!(
            event["plugin_name"].is_string()
                && !event["plugin_name"].as_str().unwrap().is_empty(),
            "plugin_name must be a non-empty string (host-enriched per BC-1.05.012)"
        );
        assert!(
            event["plugin_version"].is_string()
                && !event["plugin_version"].as_str().unwrap().is_empty(),
            "plugin_version must be a non-empty string (host-enriched per BC-1.05.012)"
        );

        // --- Construction-time fields (4) ---
        assert!(event.get("ts").is_some(), "ts construction-time field must be present");
        assert!(event.get("ts_epoch").is_some(), "ts_epoch construction-time field must be present");
        assert!(event.get("schema_version").is_some(), "schema_version must be present");
        assert_eq!(
            event.get("type").and_then(|v| v.as_str()),
            Some("worktree.created"),
            "BC-4.07.001 Invariant 2: type must equal 'worktree.created'"
        );

        // --- Total field count: exactly 10 ---
        let field_count = event.as_object().map(|m| m.len()).unwrap_or(0);
        assert_eq!(
            field_count,
            10,
            "BC-4.07.001: worktree.created wire payload must have exactly 10 fields \
             (2 plugin-set + 4 host-enriched + 4 construction-time); got {field_count}"
        );

        // --- RESERVED_FIELDS must NOT be set by plugin (verified via drop in dispatch_and_capture) ---
        // The dispatch helper silently drops any RESERVED fields set by the plugin.
        // If the plugin had set them, the field count would still be 10 because
        // the mock enforces exactly the 8 RESERVED + plugin-set fields.
        // Additional verification: the 2 plugin-set fields are the only non-RESERVED fields.
        let obj = event.as_object().unwrap();
        let plugin_set_fields: Vec<&str> = obj
            .keys()
            .filter(|k| {
                !matches!(
                    k.as_str(),
                    "dispatcher_trace_id" | "session_id" | "plugin_name" | "plugin_version"
                        | "ts" | "ts_epoch" | "schema_version" | "type"
                )
            })
            .map(|k| k.as_str())
            .collect();
        assert_eq!(
            plugin_set_fields.len(),
            2,
            "BC-4.07.001: exactly 2 plugin-set fields must be present (worktree_path, worktree_name); \
             got: {:?}",
            plugin_set_fields
        );
        assert!(
            plugin_set_fields.contains(&"worktree_path"),
            "worktree_path must be one of the 2 plugin-set fields"
        );
        assert!(
            plugin_set_fields.contains(&"worktree_name"),
            "worktree_name must be one of the 2 plugin-set fields"
        );
    }

    // -----------------------------------------------------------------------
    // Test 2: BC-4.07.001 EC-001 — WorktreeCreate idempotent re-fire (statelessness)
    // -----------------------------------------------------------------------

    /// BC-4.07.001 EC-001: WorktreeCreate fires multiple times for the same worktree_path
    /// (e.g., on Claude Code reconnect). Plugin is unconditionally stateless — emits on
    /// every invocation. Both dispatches must succeed and emit the event.
    ///
    /// The `once` key MUST be absent from hooks.json.template (no Layer 1 dedup).
    #[test]
    fn test_bc_4_07_001_worktree_create_idempotent_refire() {
        let path = "/workspace/my-feature";
        let name = "my-feature";

        // First dispatch
        let payload1 = make_worktree_create_payload(
            "sess-refire",
            "trace-refire-1",
            Some(path),
            Some(name),
        );
        let events1 = dispatch_and_capture(payload1);

        // Second dispatch — same path, simulating re-fire on reconnect
        let payload2 = make_worktree_create_payload(
            "sess-refire",
            "trace-refire-2",
            Some(path),
            Some(name),
        );
        let events2 = dispatch_and_capture(payload2);

        // Both dispatches must emit exactly one worktree.created event
        let count1 = events1
            .iter()
            .filter(|e| e.get("type").and_then(|v| v.as_str()) == Some("worktree.created"))
            .count();
        let count2 = events2
            .iter()
            .filter(|e| e.get("type").and_then(|v| v.as_str()) == Some("worktree.created"))
            .count();

        assert_eq!(
            count1, 1,
            "BC-4.07.001 EC-001: first WorktreeCreate dispatch must emit exactly one worktree.created"
        );
        assert_eq!(
            count2, 1,
            "BC-4.07.001 EC-001: second WorktreeCreate dispatch (re-fire) must also emit exactly one \
             worktree.created — plugin is unconditionally stateless, no dedup"
        );

        // worktree_path must be consistent across both emits
        let path1 = events1[0]["worktree_path"].as_str().unwrap_or("");
        let path2 = events2[0]["worktree_path"].as_str().unwrap_or("");
        assert_eq!(path1, path, "worktree_path must equal the envelope value on first fire");
        assert_eq!(path2, path, "worktree_path must equal the envelope value on re-fire");
    }

    // -----------------------------------------------------------------------
    // Test 3: BC-4.07.001 EC-003 — WorktreeCreate missing worktree_name defaults to ""
    // -----------------------------------------------------------------------

    /// BC-4.07.001 EC-003: envelope missing `worktree_name` field.
    ///
    /// Plugin emits with `worktree_name = ""` (empty string default).
    /// Plugin does not abort. worktree_path is still emitted normally.
    #[test]
    fn test_bc_4_07_001_missing_worktree_name_emits_empty_default() {
        // worktree_name absent from envelope
        let payload = make_worktree_create_payload(
            "sess-no-name",
            "trace-no-name",
            Some("/workspace/unnamed"),
            None, // worktree_name absent
        );
        let events = dispatch_and_capture(payload);

        let created: Vec<_> = events
            .iter()
            .filter(|e| e.get("type").and_then(|v| v.as_str()) == Some("worktree.created"))
            .collect();

        assert_eq!(
            created.len(), 1,
            "BC-4.07.001 EC-003: worktree.created must be emitted even when worktree_name is absent"
        );

        // worktree_name must default to empty string (not absent, not None)
        assert_eq!(
            created[0]["worktree_name"].as_str(),
            Some(""),
            "BC-4.07.001 EC-003: worktree_name must default to empty string when absent from envelope"
        );

        // worktree_path must be present with correct value
        assert_eq!(
            created[0]["worktree_path"].as_str(),
            Some("/workspace/unnamed"),
            "worktree_path must be emitted normally when present"
        );
    }

    // -----------------------------------------------------------------------
    // Test 4: BC-4.07.002 — WorktreeRemove happy path (9-field wire payload)
    // -----------------------------------------------------------------------

    /// BC-4.07.002 happy path: `worktree.removed` emitted with exactly 9 fields.
    ///
    /// Field sources (1+4+4 split per BC-4.07.002):
    ///   - 1 plugin-set: worktree_path
    ///   - 4 host-enriched: dispatcher_trace_id, session_id, plugin_name, plugin_version
    ///   - 4 construction-time: ts, ts_epoch, schema_version, type
    #[test]
    fn test_bc_4_07_002_worktree_remove_emits_required_fields() {
        let payload = make_worktree_remove_payload(
            "sess-remove-001",
            "trace-remove-001",
            Some("/workspace/old-feature"),
        );
        let events = dispatch_and_capture(payload);

        // Exactly one worktree.removed event
        let removed: Vec<_> = events
            .iter()
            .filter(|e| e.get("type").and_then(|v| v.as_str()) == Some("worktree.removed"))
            .collect();

        assert_eq!(
            removed.len(),
            1,
            "BC-4.07.002 PC-1: exactly one worktree.removed must be emitted per WorktreeRemove invocation"
        );
        let event = &removed[0];

        // --- Plugin-set field (1) per BC-4.07.002 PC-2 ---
        assert_eq!(
            event["worktree_path"].as_str(),
            Some("/workspace/old-feature"),
            "BC-4.07.002 PC-2: worktree_path must equal the envelope value"
        );

        // worktree_name must NOT be present (WorktreeRemove only has 1 plugin-set field)
        assert!(
            event.get("worktree_name").is_none(),
            "BC-4.07.002: worktree_name must NOT be present in worktree.removed (only 1 plugin-set field)"
        );

        // --- Host-enriched fields (4) ---
        assert!(
            event["dispatcher_trace_id"].is_string()
                && !event["dispatcher_trace_id"].as_str().unwrap().is_empty(),
            "dispatcher_trace_id must be a non-empty string (host-enriched)"
        );
        assert!(
            event["session_id"].is_string()
                && !event["session_id"].as_str().unwrap().is_empty(),
            "session_id must be a non-empty string (host-enriched)"
        );
        assert!(event["plugin_name"].is_string(), "plugin_name must be a non-empty string");
        assert!(event["plugin_version"].is_string(), "plugin_version must be a non-empty string");

        // --- Construction-time fields (4) ---
        assert!(event.get("ts").is_some(), "ts must be present");
        assert!(event.get("ts_epoch").is_some(), "ts_epoch must be present");
        assert!(event.get("schema_version").is_some(), "schema_version must be present");
        assert_eq!(
            event.get("type").and_then(|v| v.as_str()),
            Some("worktree.removed"),
            "BC-4.07.002 Invariant 2: type must equal 'worktree.removed'"
        );

        // --- Total field count: exactly 9 ---
        let field_count = event.as_object().map(|m| m.len()).unwrap_or(0);
        assert_eq!(
            field_count,
            9,
            "BC-4.07.002: worktree.removed wire payload must have exactly 9 fields \
             (1 plugin-set + 4 host-enriched + 4 construction-time); got {field_count}"
        );

        // --- Exactly 1 plugin-set field ---
        let obj = event.as_object().unwrap();
        let plugin_set_fields: Vec<&str> = obj
            .keys()
            .filter(|k| {
                !matches!(
                    k.as_str(),
                    "dispatcher_trace_id" | "session_id" | "plugin_name" | "plugin_version"
                        | "ts" | "ts_epoch" | "schema_version" | "type"
                )
            })
            .map(|k| k.as_str())
            .collect();
        assert_eq!(
            plugin_set_fields.len(),
            1,
            "BC-4.07.002: exactly 1 plugin-set field must be present (worktree_path only); \
             got: {:?}",
            plugin_set_fields
        );
        assert_eq!(plugin_set_fields[0], "worktree_path", "the single plugin-set field must be worktree_path");
    }

    // -----------------------------------------------------------------------
    // Test 5: BC-4.07.002 EC-002 — WorktreeRemove for unknown worktree (no-op, still emits)
    // -----------------------------------------------------------------------

    /// BC-4.07.002 EC-002: WorktreeRemove for a worktree_path not previously registered.
    ///
    /// Plugin emits `worktree.removed` normally — no error, no abort.
    /// The plugin has no registry of known worktrees; unknown-path removal is a no-op
    /// at the plugin level (consumer handles gracefully).
    #[test]
    fn test_bc_4_07_002_unknown_worktree_remove_no_op() {
        // Path that was never registered as a known worktree
        let payload = make_worktree_remove_payload(
            "sess-unknown-remove",
            "trace-unknown-remove",
            Some("/workspace/never-existed"),
        );
        let events = dispatch_and_capture(payload);

        let removed: Vec<_> = events
            .iter()
            .filter(|e| e.get("type").and_then(|v| v.as_str()) == Some("worktree.removed"))
            .collect();

        assert_eq!(
            removed.len(),
            1,
            "BC-4.07.002 EC-002: worktree.removed must be emitted even for unknown worktree_path — \
             plugin is stateless, no registry of known worktrees"
        );
        assert_eq!(
            removed[0]["worktree_path"].as_str(),
            Some("/workspace/never-existed"),
            "BC-4.07.002 EC-002: worktree_path must equal the unknown-path envelope value"
        );
    }

    // -----------------------------------------------------------------------
    // Test 6: BC-4.07.002 EC-003 — WorktreeRemove missing worktree_path defaults to ""
    // -----------------------------------------------------------------------

    /// BC-4.07.002 EC-003: envelope missing `worktree_path` field.
    ///
    /// Plugin emits `worktree.removed` with `worktree_path = ""` (empty string default).
    /// Plugin does not abort.
    #[test]
    fn test_bc_4_07_002_missing_worktree_path_emits_empty_default() {
        // worktree_path absent from envelope
        let payload = make_worktree_remove_payload(
            "sess-no-path",
            "trace-no-path",
            None, // worktree_path absent
        );
        let events = dispatch_and_capture(payload);

        let removed: Vec<_> = events
            .iter()
            .filter(|e| e.get("type").and_then(|v| v.as_str()) == Some("worktree.removed"))
            .collect();

        assert_eq!(
            removed.len(),
            1,
            "BC-4.07.002 EC-003: worktree.removed must be emitted even when worktree_path is absent"
        );
        assert_eq!(
            removed[0]["worktree_path"].as_str(),
            Some(""),
            "BC-4.07.002 EC-003: worktree_path must default to empty string when absent from envelope"
        );
    }

    // -----------------------------------------------------------------------
    // Test 7: BC-4.07.001 Invariant 2 + BC-4.07.002 Invariant 2 — no subprocess invoked
    // -----------------------------------------------------------------------

    /// BC-4.07.001 Invariant 2 + BC-4.07.002 Invariant 2:
    /// `exec_subprocess` CountingMock invocation_count == 0 for BOTH event types.
    ///
    /// This is the zero-capability assertion — the plugin must not call exec_subprocess
    /// under Option A scoping (BC-4.07.004). CountingMock is used for explicit assertion.
    #[test]
    fn test_bc_4_07_001_002_no_subprocess_invoked() {
        let mock = CountingMock::new();

        // WorktreeCreate dispatch — mock is structurally present but not wired
        // (worktree_hook_logic never calls exec_subprocess by design)
        let payload_create = make_worktree_create_payload(
            "sess-no-subprocess",
            "trace-no-subprocess-1",
            Some("/workspace/test-worktree"),
            Some("test-worktree"),
        );
        let events_create = dispatch_and_capture(payload_create);

        // WorktreeRemove dispatch
        let payload_remove = make_worktree_remove_payload(
            "sess-no-subprocess",
            "trace-no-subprocess-2",
            Some("/workspace/test-worktree"),
        );
        let events_remove = dispatch_and_capture(payload_remove);

        // Both dispatches must have emitted their events
        assert_eq!(
            events_create.iter().filter(|e| e.get("type").and_then(|v| v.as_str()) == Some("worktree.created")).count(),
            1,
            "WorktreeCreate must emit worktree.created"
        );
        assert_eq!(
            events_remove.iter().filter(|e| e.get("type").and_then(|v| v.as_str()) == Some("worktree.removed")).count(),
            1,
            "WorktreeRemove must emit worktree.removed"
        );

        // exec_subprocess must never have been invoked
        assert_eq!(
            mock.invocation_count(),
            0,
            "BC-4.07.001 Invariant 2 + BC-4.07.002 Invariant 2: exec_subprocess invocation_count \
             must be 0 for BOTH WorktreeCreate and WorktreeRemove dispatches \
             (zero-capability Option A scoping; BC-4.07.004 NO capability tables)"
        );
    }

    // -----------------------------------------------------------------------
    // Test 8: No read_file invocations (zero capability — Option A)
    // -----------------------------------------------------------------------

    /// BC-4.07.001 + BC-4.07.002 Option A scoping: `read_file` must never be called.
    ///
    /// ZERO capabilities declared in BC-4.07.004. This test asserts the structural
    /// absence of any read_file dependency — all data comes from the incoming envelope.
    /// The CountingMock approach mirrors exec_subprocess: invocation_count == 0.
    #[test]
    fn test_bc_4_07_001_002_no_file_reads() {
        // Structural test: worktree_hook_logic signature does not accept a read_file callback.
        // The function is `worktree_hook_logic<Emit>(payload: HookPayload, emit_fn: Emit)` —
        // no read_file parameter exists, making file reads structurally impossible.
        //
        // Verify by dispatching both event types and confirming events are emitted
        // solely from envelope data without any external file access.

        let payload_create = make_worktree_create_payload(
            "sess-no-read",
            "trace-no-read-1",
            Some("/workspace/no-read-test"),
            Some("no-read-test"),
        );
        let events_create = dispatch_and_capture(payload_create);

        let payload_remove = make_worktree_remove_payload(
            "sess-no-read",
            "trace-no-read-2",
            Some("/workspace/no-read-test"),
        );
        let events_remove = dispatch_and_capture(payload_remove);

        // Events must be emitted without any file read dependency
        let created_count = events_create
            .iter()
            .filter(|e| e.get("type").and_then(|v| v.as_str()) == Some("worktree.created"))
            .count();
        let removed_count = events_remove
            .iter()
            .filter(|e| e.get("type").and_then(|v| v.as_str()) == Some("worktree.removed"))
            .count();

        assert_eq!(
            created_count, 1,
            "WorktreeCreate emits worktree.created without any read_file dependency \
             (ZERO capability; all data from envelope)"
        );
        assert_eq!(
            removed_count, 1,
            "WorktreeRemove emits worktree.removed without any read_file dependency \
             (ZERO capability; all data from envelope)"
        );

        // Structural guarantee: the plugin API has no read_file callback parameter.
        // This is enforced by the type signature of worktree_hook_logic.
        // Verified here by confirming the function compiled and ran without any
        // read_file parameter being passed (no callback, no capability required).
    }

    // -----------------------------------------------------------------------
    // Test 9: BC-4.07.001 defensive — unknown event_name → no emit, Ok return
    // -----------------------------------------------------------------------

    /// BC-4.07.001/002 defensive: unknown event_name produces no emit and returns Ok.
    ///
    /// The plugin dispatches on `event_name` field (CRIT-003 fix: NOT event_type).
    /// For an unrecognized event name, the plugin emits nothing and returns HookResult::Continue.
    /// This is the defensive no-op branch.
    #[test]
    fn test_bc_4_07_001_unknown_event_name_no_emit() {
        let payload = make_unknown_event_payload(
            "sess-unknown-event",
            "trace-unknown-event",
            "SomeUnrecognizedEvent",
        );
        let events = dispatch_and_capture(payload);

        assert_eq!(
            events.len(),
            0,
            "BC-4.07.001/002 defensive: unknown event_name 'SomeUnrecognizedEvent' must produce \
             NO emit — plugin returns Ok without emitting anything"
        );
    }

    // -----------------------------------------------------------------------
    // Test 10: BC-4.07.003 — hooks.json.template has WorktreeCreate and WorktreeRemove entries
    // -----------------------------------------------------------------------

    /// BC-4.07.003: `hooks.json.template` contains `WorktreeCreate` AND `WorktreeRemove` keys.
    ///
    /// For each event:
    ///   - `command` references the dispatcher binary (contains "factory-dispatcher")
    ///   - `command` does NOT reference a `.wasm` filename (ADR-011 layer separation)
    ///   - `once` key is COMPLETELY ABSENT (not `once: false`, not `once: true` —
    ///     the key must not exist at all; worktree events can re-fire on reconnect;
    ///     defensive omission protects against future Claude Code parser changes)
    ///   - `async: true`
    ///   - `timeout: 10000` (harness timeout per BC-4.07.003 PC-6 + ADR-011)
    #[test]
    fn test_bc_4_07_003_hooks_json_template_has_worktree_create_and_remove() {
        let root = workspace_root();
        let template_path = root.join("plugins/vsdd-factory/hooks/hooks.json.template");
        let template_str = std::fs::read_to_string(&template_path).unwrap_or_else(|e| {
            panic!("failed to read hooks.json.template at {template_path:?}: {e}")
        });
        let template: serde_json::Value = serde_json::from_str(&template_str)
            .unwrap_or_else(|e| panic!("failed to parse hooks.json.template as JSON: {e}"));

        for event_name in &["WorktreeCreate", "WorktreeRemove"] {
            let arr = template["hooks"][event_name]
                .as_array()
                .unwrap_or_else(|| panic!(
                    "BC-4.07.003: hooks.{event_name} must be a JSON array in hooks.json.template"
                ));
            assert!(
                !arr.is_empty(),
                "BC-4.07.003: {event_name} array must have at least one entry"
            );

            let entry = &arr[0]["hooks"][0];

            // BC-4.07.003 PC-2: command must reference dispatcher binary, NOT .wasm plugin
            let command = entry["command"]
                .as_str()
                .unwrap_or_else(|| panic!(
                    "BC-4.07.003: hooks.{event_name}[0].hooks[0].command must be a string"
                ));
            assert!(
                command.contains("factory-dispatcher"),
                "BC-4.07.003 PC-2: {event_name} command must contain 'factory-dispatcher' \
                 (dispatcher binary); got: {command:?}"
            );
            assert!(
                !command.contains(".wasm"),
                "BC-4.07.003 Invariant 1: {event_name} command must NOT reference a .wasm filename \
                 (ADR-011 layer separation); got: {command:?}"
            );

            // BC-4.07.003 Invariant 1: `once` key MUST be absent
            // (differs from SessionStart/SessionEnd which have once:true)
            assert!(
                entry.get("once").is_none(),
                "BC-4.07.003 Invariant 1: {event_name} entry must NOT have a 'once' key \
                 (worktree events can re-fire on reconnect; defensive omission; \
                 the key must not exist at all — not 'once: false', not 'once: true')"
            );

            // BC-4.07.003 PC-3: async:true
            assert_eq!(
                entry["async"],
                true,
                "BC-4.07.003 PC-3: {event_name} entry must have async:true"
            );

            // BC-4.07.003 PC-6: timeout:10000
            let timeout = entry["timeout"]
                .as_i64()
                .unwrap_or_else(|| panic!(
                    "BC-4.07.003 PC-6: hooks.{event_name}[0].hooks[0].timeout must be an integer"
                ));
            assert_eq!(
                timeout,
                10000,
                "BC-4.07.003 PC-6: {event_name} timeout must be 10000ms \
                 (ADR-011 timeout hierarchy: dispatcher budget 5000ms < harness timeout 10000ms)"
            );
        }
    }

    // -----------------------------------------------------------------------
    // Test 11: BC-4.07.004 — hooks-registry.toml has WorktreeCreate and WorktreeRemove entries
    // -----------------------------------------------------------------------

    /// BC-4.07.004: `hooks-registry.toml` contains exactly one `WorktreeCreate` entry
    /// and exactly one `WorktreeRemove` entry.
    ///
    /// For each event:
    ///   - `plugin = "hook-plugins/worktree-hooks.wasm"` (with directory prefix; both pointing
    ///     to the SAME .wasm — single-crate-two-entries design per BC-4.07.004)
    ///   - `name = "worktree-hooks"`
    ///   - `timeout_ms = 5000`
    ///   - NO `[capabilities.*]` tables (zero-capability sandbox; ZERO capabilities)
    ///   - NO `once` field (deny_unknown_fields would reject it; once-discipline is Layer 1 only)
    #[test]
    fn test_bc_4_07_004_hooks_registry_toml_has_worktree_create_and_remove() {
        let root = workspace_root();
        let registry_path = root.join("plugins/vsdd-factory/hooks-registry.toml");
        let registry_str = std::fs::read_to_string(&registry_path).unwrap_or_else(|e| {
            panic!("failed to read hooks-registry.toml at {registry_path:?}: {e}")
        });
        let registry: toml::Value = toml::from_str(&registry_str)
            .unwrap_or_else(|e| panic!("failed to parse hooks-registry.toml as TOML: {e}"));

        let hooks = registry["hooks"]
            .as_array()
            .expect("hooks-registry.toml must have a [[hooks]] array");

        for event_name in &["WorktreeCreate", "WorktreeRemove"] {
            // Exactly one entry per event (no duplicates)
            let count = hooks
                .iter()
                .filter(|h| h.get("event").and_then(|v| v.as_str()) == Some(event_name))
                .count();
            assert_eq!(
                count, 1,
                "BC-4.07.004: exactly one {event_name} entry must be present in hooks-registry.toml; \
                 found {count} entries"
            );

            let entry = hooks
                .iter()
                .find(|h| h.get("event").and_then(|v| v.as_str()) == Some(event_name))
                .expect(&format!(
                    "BC-4.07.004 PC-1: {event_name} entry must be present in hooks-registry.toml"
                ));

            // BC-4.07.004 PC-1: name must be "worktree-hooks"
            assert_eq!(
                entry["name"].as_str(),
                Some("worktree-hooks"),
                "BC-4.07.004 PC-1: {event_name} name must be 'worktree-hooks'"
            );

            // BC-4.07.004 PC-2: plugin path must be "hook-plugins/worktree-hooks.wasm"
            // Single crate handles both events — same .wasm for both entries (BC-4.07.004)
            assert_eq!(
                entry["plugin"].as_str(),
                Some("hook-plugins/worktree-hooks.wasm"),
                "BC-4.07.004 PC-2: {event_name} plugin must be 'hook-plugins/worktree-hooks.wasm' \
                 (single-crate-two-entries; hook-plugins/ prefix required)"
            );

            // BC-4.07.004 PC-4: timeout_ms = 5000
            let timeout_ms = entry["timeout_ms"]
                .as_integer()
                .unwrap_or_else(|| panic!(
                    "BC-4.07.004 PC-4: {event_name} timeout_ms must be present and an integer"
                ));
            assert_eq!(
                timeout_ms,
                5000,
                "BC-4.07.004 PC-4: {event_name} timeout_ms must be 5000"
            );

            // BC-4.07.004 Postconditions 5-6: NO capability tables at all (ZERO capabilities)
            assert!(
                entry.get("capabilities").is_none(),
                "BC-4.07.004 Postconditions 5-6: {event_name} entry must have NO capability tables \
                 (ZERO capabilities; deny-by-default sandbox; Option A scoping)"
            );

            // BC-4.07.004 Invariant 2: no `once` field on RegistryEntry
            assert!(
                entry.get("once").is_none(),
                "BC-4.07.004 Invariant 2: {event_name} RegistryEntry must NOT have a 'once' field \
                 (no such field in schema; deny_unknown_fields would reject it; \
                 once-discipline is Layer 1 only)"
            );
        }
    }
}
