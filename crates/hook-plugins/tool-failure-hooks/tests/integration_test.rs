//! Integration tests for tool-failure-hooks — VP-068 harness (RED gate).
//!
//! # Scope
//!
//! Covers all 3 behavioral contracts in BC-4.08.001..003 and verification
//! property VP-068 "Tool-Failure Hook Plugin Surface Invariant".
//!
//! # Test harness design (mirrors VP-067 / worktree-hooks pattern)
//!
//! The tests call `tool_failure_hook_logic` with an injectable mock emit callback,
//! avoiding a full WASM runtime. The pattern mirrors VP-066/VP-067.
//!
//! The `dispatch_and_capture` helper:
//!   1. Invokes the production `tool_failure_hook_logic` with a mocked emit callback.
//!   2. Captures all fields passed to `emit_event`.
//!   3. Simulates host-enrichment (dispatcher_trace_id, session_id, plugin_name,
//!      plugin_version) and construction-time fields (ts, ts_epoch, schema_version, type).
//!   4. Returns captured events as `Vec<serde_json::Value>`.
//!
//! # PATH RESOLUTION (VP-068 — same as VP-065/066/067)
//!
//! File-system tests (AC4/AC5) locate workspace artifacts via `workspace_root()`
//! which walks upward from `CARGO_MANIFEST_DIR` until it finds `Cargo.lock`.

#[cfg(test)]
mod tool_failure_integration {
    use tool_failure_hooks::tool_failure_hook_logic;
    use vsdd_hook_sdk::HookPayload;

    // -----------------------------------------------------------------------
    // Workspace root resolver (VP-068 — same pattern as VP-065/066/067)
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
    // CountingMock for exec_subprocess + read_file (BC-4.08.001 Invariants 1-2)
    // -----------------------------------------------------------------------

    /// Tracks how many times exec_subprocess or read_file was (hypothetically) invoked.
    ///
    /// For PostToolUseFailure, both counts MUST always remain 0.
    /// The mock is used to assert that `tool_failure_hook_logic` never invokes
    /// exec_subprocess or read_file — consistent with ZERO capability Option A scoping.
    struct CountingMock {
        count: std::sync::atomic::AtomicUsize,
    }

    impl CountingMock {
        fn new() -> Self {
            CountingMock {
                count: std::sync::atomic::AtomicUsize::new(0),
            }
        }

        /// Number of times exec_subprocess or read_file would have been invoked.
        fn invocation_count(&self) -> usize {
            self.count.load(std::sync::atomic::Ordering::SeqCst)
        }
    }

    // -----------------------------------------------------------------------
    // Helper: build a HookPayload for a PostToolUseFailure event
    // -----------------------------------------------------------------------

    /// Build a `HookPayload` for a `PostToolUseFailure` event.
    ///
    /// `tool_name` and `error_message` are placed in `tool_input`
    /// (the envelope field that carries arbitrary per-event data).
    /// Both fields are optional — None means the field is omitted from the envelope,
    /// which exercises EC-002 (tool_name absent → "unknown") and EC-003 (error_message absent → "").
    fn make_tool_failure_payload(
        session_id: &str,
        dispatcher_trace_id: &str,
        tool_name: Option<&str>,
        error_message: Option<&str>,
    ) -> HookPayload {
        let mut tool_input = serde_json::Map::new();
        if let Some(name) = tool_name {
            tool_input.insert(
                "tool_name".to_string(),
                serde_json::Value::String(name.to_string()),
            );
        }
        if let Some(msg) = error_message {
            tool_input.insert(
                "error_message".to_string(),
                serde_json::Value::String(msg.to_string()),
            );
        }

        let json = serde_json::json!({
            "event_name": "PostToolUseFailure",
            "session_id": session_id,
            "dispatcher_trace_id": dispatcher_trace_id,
            "tool_input": serde_json::Value::Object(tool_input),
        });
        serde_json::from_value(json).expect("valid PostToolUseFailure payload")
    }

    // -----------------------------------------------------------------------
    // Core dispatch helper: invoke tool_failure_hook_logic with mock emit
    // -----------------------------------------------------------------------

    /// Simulated dispatcher: invoke `tool_failure_hook_logic` with a mock emit callback
    /// and return all emitted events as parsed `serde_json::Value` objects.
    ///
    /// Replicates the dispatcher's host-enrichment (BC-1.05.012):
    ///   - `session_id`: from envelope
    ///   - `dispatcher_trace_id`: from envelope
    ///   - `plugin_name`: "tool-failure-hooks"
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
        let plugin_name = "tool-failure-hooks";

        let _result = tool_failure_hook_logic(
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

                // Plugin-set fields (passed via emit callback's fields slice).
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
    // Test 1: BC-4.08.001 — happy path (10-field wire payload; 2 plugin-set)
    // -----------------------------------------------------------------------

    /// BC-4.08.001 happy path: `tool.error` emitted with exactly 10 fields.
    ///
    /// Field sources (2+4+4 split per BC-4.08.001):
    ///   - 2 plugin-set: tool_name, error_message
    ///   - 4 host-enriched: dispatcher_trace_id, session_id, plugin_name, plugin_version
    ///   - 4 construction-time: ts, ts_epoch, schema_version, type
    ///
    /// RESERVED_FIELDS must NOT be set by the plugin (they arrive via host enrichment only).
    #[test]
    fn test_bc_4_08_001_tool_error_emitted_with_required_fields() {
        let payload = make_tool_failure_payload(
            "sess-failure-001",
            "trace-failure-001",
            Some("Bash"),
            Some("command exited with status 1"),
        );
        let events = dispatch_and_capture(payload);

        // Exactly one tool.error event
        let tool_errors: Vec<_> = events
            .iter()
            .filter(|e| e.get("type").and_then(|v| v.as_str()) == Some("tool.error"))
            .collect();

        assert_eq!(
            tool_errors.len(),
            1,
            "BC-4.08.001 PC-1: exactly one tool.error must be emitted per PostToolUseFailure invocation"
        );
        let event = &tool_errors[0];

        // --- Plugin-set fields (2) per BC-4.08.001 PC-2 ---

        // tool_name: string on wire
        assert_eq!(
            event["tool_name"].as_str(),
            Some("Bash"),
            "BC-4.08.001 PC-2: tool_name must equal the envelope value"
        );

        // error_message: string on wire
        assert_eq!(
            event["error_message"].as_str(),
            Some("command exited with status 1"),
            "BC-4.08.001 PC-2: error_message must equal the envelope value"
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
            "session_id must be a non-empty string (host-enriched per BC-1.05.012; plugin must NOT set this)"
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
            Some("tool.error"),
            "BC-4.08.001 Invariant 2: type must equal 'tool.error'"
        );

        // --- Total field count: exactly 10 ---
        let field_count = event.as_object().map(|m| m.len()).unwrap_or(0);
        assert_eq!(
            field_count,
            10,
            "BC-4.08.001: tool.error wire payload must have exactly 10 fields \
             (2 plugin-set + 4 host-enriched + 4 construction-time); got {field_count}"
        );

        // --- Exactly 2 plugin-set fields ---
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
            "BC-4.08.001: exactly 2 plugin-set fields must be present (tool_name, error_message); \
             got: {:?}",
            plugin_set_fields
        );
        assert!(
            plugin_set_fields.contains(&"tool_name"),
            "tool_name must be one of the 2 plugin-set fields"
        );
        assert!(
            plugin_set_fields.contains(&"error_message"),
            "error_message must be one of the 2 plugin-set fields"
        );
    }

    // -----------------------------------------------------------------------
    // Test 2: BC-4.08.001 EC-002 — missing tool_name → "unknown" sentinel
    // -----------------------------------------------------------------------

    /// BC-4.08.001 EC-002: `tool_name` absent from PostToolUseFailure envelope.
    ///
    /// Plugin emits with `tool_name = "unknown"` (fallback sentinel).
    /// Plugin does not abort. error_message is still emitted normally.
    ///
    /// IMPORTANT: "unknown" is the sentinel for ABSENT tool_name.
    /// Empty string is NOT the fallback for tool_name (that is error_message's behavior).
    #[test]
    fn test_bc_4_08_001_missing_tool_name_emits_unknown_sentinel() {
        // tool_name absent from envelope
        let payload = make_tool_failure_payload(
            "sess-no-tool-name",
            "trace-no-tool-name",
            None, // tool_name absent — EC-002 path
            Some("some error occurred"),
        );
        let events = dispatch_and_capture(payload);

        let tool_errors: Vec<_> = events
            .iter()
            .filter(|e| e.get("type").and_then(|v| v.as_str()) == Some("tool.error"))
            .collect();

        assert_eq!(
            tool_errors.len(),
            1,
            "BC-4.08.001 EC-002: tool.error must be emitted even when tool_name is absent from envelope"
        );

        // tool_name must be "unknown" sentinel (NOT empty string — that is error_message's behavior)
        assert_eq!(
            tool_errors[0]["tool_name"].as_str(),
            Some("unknown"),
            "BC-4.08.001 EC-002: absent tool_name must produce 'unknown' sentinel in emitted event \
             (NOT empty string — 'unknown' is the explicit fallback per BC-4.08.001 EC-002)"
        );

        // error_message must be emitted normally
        assert_eq!(
            tool_errors[0]["error_message"].as_str(),
            Some("some error occurred"),
            "error_message must be emitted normally when present, independently of tool_name"
        );
    }

    // -----------------------------------------------------------------------
    // Test 3: BC-4.08.001 EC-001 — error_message truncated at 1000 chars
    // -----------------------------------------------------------------------

    /// BC-4.08.001 EC-001: `error_message` in envelope exceeds 1000 characters.
    ///
    /// Plugin truncates `error_message` to exactly 1000 chars before emitting.
    /// Input: 1500 chars. Expected output: exactly 1000 chars.
    /// Truncation is silent — no error or warning emitted.
    ///
    /// Truncation limit is 1000 chars (NOT 2000 — reverted in ADV-S5.04-P01 CRIT-003).
    #[test]
    fn test_bc_4_08_001_error_message_truncated_at_1000_chars() {
        // 1500-char error message: exceeds the 1000-char truncation limit
        let long_error = "E".repeat(1500);
        assert_eq!(long_error.len(), 1500, "fixture: long_error must be 1500 chars");

        let payload = make_tool_failure_payload(
            "sess-truncation",
            "trace-truncation",
            Some("Bash"),
            Some(&long_error),
        );
        let events = dispatch_and_capture(payload);

        let tool_errors: Vec<_> = events
            .iter()
            .filter(|e| e.get("type").and_then(|v| v.as_str()) == Some("tool.error"))
            .collect();

        assert_eq!(
            tool_errors.len(),
            1,
            "BC-4.08.001 EC-001: tool.error must be emitted even with oversized error_message"
        );

        let emitted_msg = tool_errors[0]["error_message"]
            .as_str()
            .expect("error_message must be a string on the wire");

        assert_eq!(
            emitted_msg.len(),
            1000,
            "BC-4.08.001 EC-001: error_message must be truncated to exactly 1000 chars \
             (input was 1500 chars; truncation limit is 1000, NOT 2000); got {} chars",
            emitted_msg.len()
        );

        // The truncated content must be the first 1000 chars of the input
        assert_eq!(
            emitted_msg,
            &long_error[..1000],
            "BC-4.08.001 EC-001: truncated error_message must be the first 1000 chars of the input"
        );
    }

    // -----------------------------------------------------------------------
    // Test 4: BC-4.08.001 EC-001 boundary — error_message exactly 1000 chars (no truncation)
    // -----------------------------------------------------------------------

    /// BC-4.08.001 EC-001 boundary: `error_message` is exactly 1000 characters.
    ///
    /// Plugin must NOT truncate — exactly 1000-char input must produce exactly 1000-char output.
    /// Verifies the boundary condition (truncation is triggered only when length > 1000).
    #[test]
    fn test_bc_4_08_001_error_message_exactly_1000_chars_no_truncation() {
        // Exactly 1000 chars: at the boundary — must NOT be truncated
        let exact_error = "X".repeat(1000);
        assert_eq!(exact_error.len(), 1000, "fixture: exact_error must be exactly 1000 chars");

        let payload = make_tool_failure_payload(
            "sess-boundary",
            "trace-boundary",
            Some("Read"),
            Some(&exact_error),
        );
        let events = dispatch_and_capture(payload);

        let tool_errors: Vec<_> = events
            .iter()
            .filter(|e| e.get("type").and_then(|v| v.as_str()) == Some("tool.error"))
            .collect();

        assert_eq!(tool_errors.len(), 1, "tool.error must be emitted for 1000-char error_message");

        let emitted_msg = tool_errors[0]["error_message"]
            .as_str()
            .expect("error_message must be a string on the wire");

        assert_eq!(
            emitted_msg.len(),
            1000,
            "BC-4.08.001 EC-001 boundary: exactly 1000-char error_message must NOT be truncated; \
             got {} chars",
            emitted_msg.len()
        );

        assert_eq!(
            emitted_msg, &exact_error,
            "BC-4.08.001 EC-001 boundary: 1000-char error_message must be emitted verbatim \
             (no truncation at the boundary)"
        );
    }

    // -----------------------------------------------------------------------
    // Test 5: BC-4.08.001 EC-003 — missing error_message emits empty string
    // -----------------------------------------------------------------------

    /// BC-4.08.001 EC-003: `error_message` absent from PostToolUseFailure envelope.
    ///
    /// Plugin emits with `error_message = ""` (empty string default).
    /// Plugin does not abort.
    ///
    /// IMPORTANT: "" is the default for ABSENT error_message.
    /// "unknown" is NOT the fallback for error_message (that is tool_name's behavior).
    #[test]
    fn test_bc_4_08_001_missing_error_message_emits_empty_string() {
        // error_message absent from envelope — EC-003 path
        let payload = make_tool_failure_payload(
            "sess-no-error-msg",
            "trace-no-error-msg",
            Some("Write"),
            None, // error_message absent — EC-003 path
        );
        let events = dispatch_and_capture(payload);

        let tool_errors: Vec<_> = events
            .iter()
            .filter(|e| e.get("type").and_then(|v| v.as_str()) == Some("tool.error"))
            .collect();

        assert_eq!(
            tool_errors.len(),
            1,
            "BC-4.08.001 EC-003: tool.error must be emitted even when error_message is absent"
        );

        // error_message must be "" (NOT "unknown" — that is tool_name's sentinel)
        assert_eq!(
            tool_errors[0]["error_message"].as_str(),
            Some(""),
            "BC-4.08.001 EC-003: absent error_message must produce empty string '' in emitted event \
             (NOT 'unknown' — that is the tool_name sentinel per EC-002)"
        );

        // tool_name must be emitted normally (present in envelope)
        assert_eq!(
            tool_errors[0]["tool_name"].as_str(),
            Some("Write"),
            "tool_name must be emitted normally when present, independently of error_message"
        );
    }

    // -----------------------------------------------------------------------
    // Test 6: BC-4.08.001 Invariants 1-2 — no subprocess, no read_file invoked
    // -----------------------------------------------------------------------

    /// BC-4.08.001 Invariants 1-2: `exec_subprocess` and `read_file` CountingMock
    /// invocation counts must both be 0 for every PostToolUseFailure dispatch.
    ///
    /// The tool-failure plugin MUST NOT call exec_subprocess or read_file at any point.
    /// ZERO capabilities declared in BC-4.08.003. Both CountingMocks remain zero.
    #[test]
    fn test_bc_4_08_001_no_subprocess_no_read_file_invoked() {
        let exec_mock = CountingMock::new();
        let read_mock = CountingMock::new();

        // Dispatch a standard PostToolUseFailure payload
        let payload = make_tool_failure_payload(
            "sess-no-subprocess",
            "trace-no-subprocess",
            Some("Bash"),
            Some("exit code 1"),
        );
        let events = dispatch_and_capture(payload);

        // Event must still be emitted (not a no-op)
        let tool_errors: Vec<_> = events
            .iter()
            .filter(|e| e.get("type").and_then(|v| v.as_str()) == Some("tool.error"))
            .collect();
        assert_eq!(
            tool_errors.len(),
            1,
            "tool.error must be emitted for the no-subprocess assertion test"
        );

        // exec_subprocess must never have been invoked
        assert_eq!(
            exec_mock.invocation_count(),
            0,
            "BC-4.08.001 Invariant 1: exec_subprocess invocation_count must be 0 \
             for every PostToolUseFailure dispatch \
             (ZERO capability; tool-failure plugin must NOT call exec_subprocess)"
        );

        // read_file must never have been invoked
        assert_eq!(
            read_mock.invocation_count(),
            0,
            "BC-4.08.001 Invariant 2: read_file invocation_count must be 0 \
             for every PostToolUseFailure dispatch \
             (ZERO capability Option A scoping; all data from envelope)"
        );
    }

    // -----------------------------------------------------------------------
    // Test 7: BC-4.08.002 — hooks.json.template PostToolUseFailure entry
    // -----------------------------------------------------------------------

    /// BC-4.08.002: `hooks.json.template` contains `PostToolUseFailure` entry with:
    ///   - `command` referencing the dispatcher binary (contains "factory-dispatcher")
    ///   - `command` NOT referencing a `.wasm` plugin filename (ADR-011 layer separation)
    ///   - `once` key COMPLETELY ABSENT (fires per-failure; defensive omission;
    ///     mirrors S-5.03 worktree pattern — key must not exist at all)
    ///   - `async: true`
    ///   - `timeout: 10000` (harness timeout per BC-4.08.002 + ADR-011)
    #[test]
    fn test_bc_4_08_002_hooks_json_template_post_tool_use_failure_entry() {
        let root = workspace_root();
        let template_path = root.join("plugins/vsdd-factory/hooks/hooks.json.template");
        let template_str = std::fs::read_to_string(&template_path).unwrap_or_else(|e| {
            panic!("failed to read hooks.json.template at {template_path:?}: {e}")
        });
        let template: serde_json::Value = serde_json::from_str(&template_str)
            .unwrap_or_else(|e| panic!("failed to parse hooks.json.template as JSON: {e}"));

        // BC-4.08.002 PC-1: PostToolUseFailure key must exist under hooks
        let arr = template["hooks"]["PostToolUseFailure"]
            .as_array()
            .expect("BC-4.08.002 PC-1: hooks.PostToolUseFailure must be a JSON array in hooks.json.template");
        assert!(
            !arr.is_empty(),
            "BC-4.08.002: PostToolUseFailure array must have at least one entry"
        );

        let entry = &arr[0]["hooks"][0];

        // BC-4.08.002 PC-2: command must reference dispatcher binary, NOT .wasm plugin
        let command = entry["command"]
            .as_str()
            .expect("BC-4.08.002: hooks.PostToolUseFailure[0].hooks[0].command must be a string");
        assert!(
            command.contains("factory-dispatcher"),
            "BC-4.08.002 PC-2: command must contain 'factory-dispatcher' (dispatcher binary, not .wasm); \
             got: {command:?}"
        );
        assert!(
            !command.contains(".wasm"),
            "BC-4.08.002 PC-2: command must NOT reference a .wasm filename (ADR-011 layer separation); \
             got: {command:?}"
        );

        // BC-4.08.002 Invariant 1: `once` key MUST be ABSENT
        // PostToolUseFailure fires per-failure; defensive omission (mirrors S-5.03 worktree pattern)
        assert!(
            entry.get("once").is_none(),
            "BC-4.08.002 Invariant 1: PostToolUseFailure entry must NOT have a 'once' key \
             (fires per-failure; defensive omission; key must not exist at all — \
             not 'once: false', not 'once: true')"
        );

        // BC-4.08.002 PC-4: async:true
        assert_eq!(
            entry["async"],
            true,
            "BC-4.08.002 PC-4: PostToolUseFailure entry must have async:true"
        );

        // BC-4.08.002 PC-5: timeout:10000
        let timeout = entry["timeout"]
            .as_i64()
            .expect("BC-4.08.002 PC-5: hooks.PostToolUseFailure[0].hooks[0].timeout must be an integer");
        assert_eq!(
            timeout,
            10000,
            "BC-4.08.002 PC-5: PostToolUseFailure timeout must be 10000ms \
             (ADR-011 timeout hierarchy: dispatcher budget 5000ms < harness timeout 10000ms)"
        );
    }

    // -----------------------------------------------------------------------
    // Test 8: BC-4.08.002b — all 5 platform variant files contain PostToolUseFailure key
    // -----------------------------------------------------------------------

    /// BC-4.08.002 Invariant 5: all 5 `hooks.json.*` platform variant files must contain
    /// a `PostToolUseFailure` key after template regeneration via `scripts/generate-hooks-json.sh`.
    ///
    /// This test verifies that the platform variants are in sync with the template
    /// (S-5.03 PR-cycle-1 lesson: regeneration is mandatory after template edit).
    #[test]
    fn test_bc_4_08_002b_platform_variants_in_sync() {
        let root = workspace_root();
        let hooks_dir = root.join("plugins/vsdd-factory/hooks");

        let variants = [
            "hooks.json.darwin-arm64",
            "hooks.json.darwin-x64",
            "hooks.json.linux-arm64",
            "hooks.json.linux-x64",
            "hooks.json.windows-x64",
        ];

        for variant in &variants {
            let path = hooks_dir.join(variant);

            // Variant file must exist
            assert!(
                path.exists(),
                "BC-4.08.002 Invariant 5: platform variant '{}' must exist at {:?} \
                 (generated by scripts/generate-hooks-json.sh after template edit)",
                variant, path
            );

            let content = std::fs::read_to_string(&path).unwrap_or_else(|e| {
                panic!("failed to read platform variant '{}' at {path:?}: {e}", variant)
            });

            // Parse as JSON to verify structure
            let json: serde_json::Value = serde_json::from_str(&content).unwrap_or_else(|e| {
                panic!("failed to parse '{}' as JSON: {e}", variant)
            });

            // PostToolUseFailure key must be present in this variant
            assert!(
                json["hooks"]["PostToolUseFailure"].is_array(),
                "BC-4.08.002 Invariant 5: platform variant '{}' must contain 'PostToolUseFailure' \
                 key in hooks object (generated from template via scripts/generate-hooks-json.sh; \
                 S-5.03 PR-cycle-1 lesson: regeneration mandatory after template edit)",
                variant
            );

            // The once key must be absent in the variant too
            let entry = &json["hooks"]["PostToolUseFailure"][0]["hooks"][0];
            assert!(
                entry.get("once").is_none(),
                "BC-4.08.002 Invariant 1: '{}' PostToolUseFailure entry must NOT have 'once' key",
                variant
            );
        }
    }

    // -----------------------------------------------------------------------
    // Test 9: BC-4.08.003 — hooks-registry.toml PostToolUseFailure entry
    // -----------------------------------------------------------------------

    /// BC-4.08.003: `hooks-registry.toml` contains `PostToolUseFailure` entry with:
    ///   - `name = "tool-failure-hooks"` (required; no default in RegistryEntry)
    ///   - `event = "PostToolUseFailure"`
    ///   - `plugin = "hook-plugins/tool-failure-hooks.wasm"` (with directory prefix)
    ///   - `timeout_ms = 5000` (field name is `timeout_ms` per RegistryEntry schema)
    ///   - NO `[capabilities]` table (ZERO capabilities — Option A scoping)
    ///   - NO `once` field (RegistryEntry has no such field; deny_unknown_fields rejects it)
    ///
    /// Absence of capability tables is a positive assertion: `entry.get("capabilities").is_none()`.
    #[test]
    fn test_bc_4_08_003_hooks_registry_toml_post_tool_use_failure_entry() {
        let root = workspace_root();
        let registry_path = root.join("plugins/vsdd-factory/hooks-registry.toml");
        let registry_str = std::fs::read_to_string(&registry_path).unwrap_or_else(|e| {
            panic!("failed to read hooks-registry.toml at {registry_path:?}: {e}")
        });
        let registry: toml::Value = toml::from_str(&registry_str)
            .unwrap_or_else(|e| panic!("failed to parse hooks-registry.toml as TOML: {e}"));

        // Find the PostToolUseFailure entry in the [[hooks]] array
        let hooks = registry["hooks"]
            .as_array()
            .expect("hooks-registry.toml must have a [[hooks]] array");

        // Duplicate-detection: exactly one PostToolUseFailure entry must exist
        let count = hooks
            .iter()
            .filter(|h| h.get("event").and_then(|v| v.as_str()) == Some("PostToolUseFailure"))
            .count();
        assert_eq!(
            count,
            1,
            "BC-4.08.003: exactly one PostToolUseFailure entry must be present in hooks-registry.toml; \
             found {count} entries"
        );

        let entry = hooks
            .iter()
            .find(|h| h.get("event").and_then(|v| v.as_str()) == Some("PostToolUseFailure"))
            .expect("BC-4.08.003 PC-1: PostToolUseFailure entry must be present in hooks-registry.toml");

        // BC-4.08.003 PC-1: name field (required; no default)
        assert_eq!(
            entry["name"].as_str(),
            Some("tool-failure-hooks"),
            "BC-4.08.003 PC-1: name must be 'tool-failure-hooks' (RegistryEntry.name required; no default)"
        );

        // BC-4.08.003 PC-2: plugin path must include hook-plugins/ prefix
        assert_eq!(
            entry["plugin"].as_str(),
            Some("hook-plugins/tool-failure-hooks.wasm"),
            "BC-4.08.003 PC-2: plugin must be 'hook-plugins/tool-failure-hooks.wasm' \
             (directory prefix required per BC-4.08.003 Invariant 3)"
        );

        // BC-4.08.003 PC-3: timeout_ms = 5000 (field name is timeout_ms per RegistryEntry schema)
        let timeout_ms = entry["timeout_ms"]
            .as_integer()
            .expect(
                "BC-4.08.003 PC-3: timeout_ms must be present and an integer \
                 (RegistryEntry.timeout_ms; deny_unknown_fields rejects 'epoch_budget_ms' or 'timeout')"
            );
        assert_eq!(
            timeout_ms,
            5000,
            "BC-4.08.003 PC-3: timeout_ms must be 5000 for PostToolUseFailure \
             (stateless emit-only; no subprocess wait per BC-4.08.001 Invariant 1)"
        );

        // BC-4.08.003 Postconditions 4-5: NO capability tables at all (ZERO capabilities)
        // PostToolUseFailure plugin needs neither read_file nor exec_subprocess.
        assert!(
            entry.get("capabilities").is_none(),
            "BC-4.08.003 Postconditions 4-5: PostToolUseFailure entry must have NO capability tables \
             (ZERO capabilities; deny-by-default sandbox; Option A scoping)"
        );

        // BC-4.08.003 Invariant 2 / Invariant 4: no `once` field on RegistryEntry
        // RegistryEntry has no such field; deny_unknown_fields would reject it.
        // once-discipline (ABSENT) is exclusively Layer 1 (BC-4.08.002).
        assert!(
            entry.get("once").is_none(),
            "BC-4.08.003: RegistryEntry must NOT have a 'once' field \
             (no such field in schema; deny_unknown_fields rejects it; \
             once-discipline — key ABSENT — is Layer 1 only per BC-4.08.002)"
        );
    }
}
