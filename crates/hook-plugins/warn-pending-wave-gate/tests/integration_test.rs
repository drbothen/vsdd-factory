//! Integration tests for warn-pending-wave-gate — Red Gate for S-8.07.
//!
//! # Scope
//!
//! Covers BC-7.03.091 (identity & registry binding) and BC-7.03.092
//! (stderr warning when any wave has gate_status: pending).
//!
//! Also covers AC-001..AC-007 acceptance criteria, all 9 edge cases from
//! the story (EC-001..EC-009), and the 5 bats parity scenarios.
//!
//! # Test harness design
//!
//! Tests call `warn_pending_wave_gate_logic` with injectable callbacks:
//!   - `read_wave_state`: returns `Option<String>` (None = file absent / CapabilityDenied)
//!   - `emit`: captures `(event_type, fields)` — vec of `EmittedEvent`
//!   - `write_stderr`: captures the stderr message as `String`
//!
//! This avoids a WASM runtime; the production `main.rs` wires the real
//! `host::read_file` and `host::emit_event` callbacks.
//!
//! # Naming convention
//!
//! Tests follow `test_BC_7_03_NNN_<assertion>()` per VSDD traceability convention.

// Allow uppercase BC-N.NN.NNN identifiers in test names (workspace lint).
#![allow(non_snake_case)]

#[cfg(test)]
mod warn_pending_wave_gate_integration {
    use vsdd_hook_sdk::HookPayload;
    use warn_pending_wave_gate::warn_pending_wave_gate_logic;

    // -----------------------------------------------------------------------
    // Captured output types
    // -----------------------------------------------------------------------

    #[derive(Debug, Clone)]
    struct EmittedEvent {
        event_type: String,
        fields: Vec<(String, String)>,
    }

    // -----------------------------------------------------------------------
    // Helpers
    // -----------------------------------------------------------------------

    /// Build a minimal Stop HookPayload for testing.
    fn make_stop_payload() -> HookPayload {
        let json = serde_json::json!({
            "event_name": "Stop",
            "session_id": "test-session-001",
            "dispatcher_trace_id": "test-trace-001",
        });
        serde_json::from_value(json).expect("valid Stop payload")
    }

    /// Resolve workspace root by walking up from CARGO_MANIFEST_DIR until
    /// a directory containing `Cargo.lock` is found.
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

    /// Invoke warn_pending_wave_gate_logic with Arc<Mutex<...>> captures.
    /// Returns (emitted_events, stderr_output, hook_result).
    fn dispatch(
        yaml_content: Option<&str>,
    ) -> (Vec<EmittedEvent>, String, vsdd_hook_sdk::HookResult) {
        use std::sync::{Arc, Mutex};

        let yaml_owned = yaml_content.map(|s| s.to_string());
        let events: Arc<Mutex<Vec<EmittedEvent>>> = Arc::new(Mutex::new(Vec::new()));
        let stderr: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));

        let events_clone = Arc::clone(&events);
        let stderr_clone = Arc::clone(&stderr);

        let result = warn_pending_wave_gate_logic(
            make_stop_payload(),
            move || yaml_owned.clone(),
            move |event_type: &str, fields: &[(&str, &str)]| {
                events_clone.lock().unwrap().push(EmittedEvent {
                    event_type: event_type.to_string(),
                    fields: fields
                        .iter()
                        .map(|(k, v)| (k.to_string(), v.to_string()))
                        .collect(),
                });
            },
            move |msg: &str| {
                stderr_clone.lock().unwrap().push_str(msg);
            },
        );

        let captured_events = events.lock().unwrap().clone();
        let captured_stderr = stderr.lock().unwrap().clone();
        (captured_events, captured_stderr, result)
    }

    // -----------------------------------------------------------------------
    // YAML fixtures
    // -----------------------------------------------------------------------

    const YAML_ONE_PENDING: &str = r#"
waves:
  W-15:
    gate_status: pending
    started: 2026-04-01
"#;

    const YAML_TWO_PENDING: &str = r#"
waves:
  W-15:
    gate_status: pending
  W-16:
    gate_status: pending
"#;

    const YAML_ALL_PASSED: &str = r#"
waves:
  W-14:
    gate_status: passed
  W-15:
    gate_status: passed
"#;

    const YAML_MIXED: &str = r#"
waves:
  W-14:
    gate_status: passed
  W-15:
    gate_status: pending
  W-16:
    gate_status: deferred
"#;

    const YAML_EMPTY: &str = "";

    const YAML_NO_WAVES_KEY: &str = r#"
config:
  version: 1
"#;

    const YAML_WAVES_NULL: &str = r#"
waves: ~
"#;

    const YAML_WAVES_EMPTY_MAP: &str = r#"
waves: {}
"#;

    const YAML_NON_STRING_GATE_STATUS: &str = r#"
waves:
  W-15:
    gate_status: 42
  W-16:
    gate_status: true
  W-17:
    gate_status: [pending]
"#;

    const YAML_WAVES_KEY_IS_LIST: &str = r#"
waves:
  - W-15
  - W-16
"#;

    const YAML_MALFORMED: &str = r#"
waves:
  W-15:
    gate_status: pending
  bad_indent:
 - broken yaml
    [unclosed bracket
"#;

    // -----------------------------------------------------------------------
    // BC-7.03.091: identity & registry binding
    // -----------------------------------------------------------------------

    // -----------------------------------------------------------------------
    // Test 1: BC-7.03.091 postcondition 2(a) — wave-state.yaml absent → silent exit 0
    // -----------------------------------------------------------------------

    /// BC-7.03.091 PC-2(a): wave-state.yaml absent → HookResult::Continue, no emit,
    /// no stderr.
    ///
    /// Canonical test vector: read_wave_state returns None (file missing).
    /// Mirrors bash branch: `[[ ! -f .factory/wave-state.yaml ]] && exit 0`
    #[test]
    fn test_BC_7_03_091_silent_exit_when_wave_state_absent() {
        let (events, stderr, result) = dispatch(None);

        assert!(
            matches!(result, vsdd_hook_sdk::HookResult::Continue),
            "BC-7.03.091 PC-2(a): absent wave-state.yaml must return HookResult::Continue (exit 0)"
        );
        assert!(
            events.is_empty(),
            "BC-7.03.091 PC-2(a): no emit_event call when wave-state.yaml absent; got {} events",
            events.len()
        );
        assert!(
            stderr.is_empty(),
            "BC-7.03.091 PC-2(a): no stderr output when wave-state.yaml absent; got: {:?}",
            stderr
        );
    }

    // -----------------------------------------------------------------------
    // Test 2: BC-7.03.091 postcondition 2(b) — YAML parse fails → silent exit 0
    // -----------------------------------------------------------------------

    /// BC-7.03.091 PC-2(b): malformed YAML → HookResult::Continue, no emit, no stderr.
    ///
    /// Mirrors bash branch: python3 parse error → sys.exit(0) via `|| true`
    #[test]
    fn test_BC_7_03_091_silent_exit_on_malformed_yaml() {
        let (events, stderr, result) = dispatch(Some(YAML_MALFORMED));

        assert!(
            matches!(result, vsdd_hook_sdk::HookResult::Continue),
            "BC-7.03.091 PC-2(b): malformed YAML must return HookResult::Continue"
        );
        assert!(
            events.is_empty(),
            "BC-7.03.091 PC-2(b): no emit_event on malformed YAML; got {} events",
            events.len()
        );
        assert!(
            stderr.is_empty(),
            "BC-7.03.091 PC-2(b): no stderr on malformed YAML; got: {:?}",
            stderr
        );
    }

    // -----------------------------------------------------------------------
    // Test 3: BC-7.03.091 postcondition 2(b) — waves key absent → silent exit 0
    // -----------------------------------------------------------------------

    /// BC-7.03.091 PC-2(b): `waves` key absent from YAML → HookResult::Continue,
    /// no emit, no stderr.
    ///
    /// Mirrors bash branch: `if not state or 'waves' not in state: sys.exit(0)`
    /// EC-001 from story.
    #[test]
    fn test_BC_7_03_091_silent_exit_when_waves_key_absent() {
        let (events, stderr, result) = dispatch(Some(YAML_NO_WAVES_KEY));

        assert!(
            matches!(result, vsdd_hook_sdk::HookResult::Continue),
            "BC-7.03.091 PC-2(b): absent 'waves' key must return HookResult::Continue"
        );
        assert!(
            events.is_empty(),
            "BC-7.03.091 PC-2(b): no emit_event when waves key absent; got {} events",
            events.len()
        );
        assert!(
            stderr.is_empty(),
            "BC-7.03.091 PC-2(b): no stderr when waves key absent; got: {:?}",
            stderr
        );
    }

    // -----------------------------------------------------------------------
    // Test 4: BC-7.03.091 postcondition 2(c) — no pending waves → silent exit 0
    // -----------------------------------------------------------------------

    /// BC-7.03.091 PC-2(c): no wave has gate_status == "pending" → HookResult::Continue,
    /// no emit, no stderr.
    ///
    /// Mirrors bash branch: jq `select(.gate_status == "pending")` returning empty.
    #[test]
    fn test_BC_7_03_091_silent_exit_when_no_pending_waves() {
        let (events, stderr, result) = dispatch(Some(YAML_ALL_PASSED));

        assert!(
            matches!(result, vsdd_hook_sdk::HookResult::Continue),
            "BC-7.03.091 PC-2(c): no pending waves must return HookResult::Continue"
        );
        assert!(
            events.is_empty(),
            "BC-7.03.091 PC-2(c): no emit_event when no pending waves; got {} events",
            events.len()
        );
        assert!(
            stderr.is_empty(),
            "BC-7.03.091 PC-2(c): no stderr when no pending waves; got: {:?}",
            stderr
        );
    }

    // -----------------------------------------------------------------------
    // Test 5: BC-7.03.091 invariant 1 — always exits 0 (HookResult::Continue)
    //          even when pending waves found
    // -----------------------------------------------------------------------

    /// BC-7.03.091 Invariant 1 / postcondition 2: hook always returns
    /// HookResult::Continue — it is advisory only, never blocks session end.
    ///
    /// Verifies that even the "warn" path returns Continue (not Block or Error).
    /// Maps to AC-003: exits 0 at end of pending-wave path.
    #[test]
    fn test_BC_7_03_091_always_returns_continue_even_with_pending_waves() {
        let (_, _, result) = dispatch(Some(YAML_ONE_PENDING));

        assert!(
            matches!(result, vsdd_hook_sdk::HookResult::Continue),
            "BC-7.03.091 Invariant 1: pending waves found must still return \
             HookResult::Continue — hook is advisory only, never blocks session end"
        );
    }

    // -----------------------------------------------------------------------
    // BC-7.03.092: stderr warning when any wave has gate_status: pending
    // -----------------------------------------------------------------------

    // -----------------------------------------------------------------------
    // Test 6: BC-7.03.092 postcondition 1 — emit hook.block with correct fields
    // -----------------------------------------------------------------------

    /// BC-7.03.092 PC-1: when a pending wave is found, emit hook.block with the
    /// canonical 5-field signature.
    ///
    /// Canonical test vector (BC-7.03.092): "Stop with waves.W3.gate_status: pending
    /// produces stderr 'W3 gate is pending'."
    ///
    /// Maps to AC-003.
    #[test]
    fn test_BC_7_03_092_emits_hook_block_with_canonical_fields() {
        let (events, _, _) = dispatch(Some(YAML_ONE_PENDING));

        assert_eq!(
            events.len(),
            1,
            "BC-7.03.092 PC-1: exactly one emit_event call when one pending wave found; \
             got {} events",
            events.len()
        );

        let ev = &events[0];
        assert_eq!(
            ev.event_type, "hook.block",
            "BC-7.03.092 PC-1: event_type must be 'hook.block' (positional first arg); \
             got {:?}",
            ev.event_type
        );

        // Verify all 5 canonical fields from AC-003
        let field = |name: &str| -> Option<&str> {
            ev.fields
                .iter()
                .find(|(k, _)| k == name)
                .map(|(_, v)| v.as_str())
        };

        assert_eq!(
            field("hook"),
            Some("warn-pending-wave-gate"),
            "BC-7.03.092 PC-1: hook field must be 'warn-pending-wave-gate'"
        );
        assert_eq!(
            field("matcher"),
            Some("Stop"),
            "BC-7.03.092 PC-1: matcher field must be 'Stop'"
        );
        assert_eq!(
            field("reason"),
            Some("pending_wave_gate_at_session_end"),
            "BC-7.03.092 PC-1: reason field must be 'pending_wave_gate_at_session_end'"
        );
        assert_eq!(
            field("severity"),
            Some("warn"),
            "BC-7.03.092 PC-1: severity field must be 'warn'"
        );

        // pending_waves must be the wave name (comma-joined; EC-004 covers multi)
        let pending_waves = field("pending_waves").expect("pending_waves field must be present");
        assert!(
            pending_waves.contains("W-15"),
            "BC-7.03.092 PC-1: pending_waves must contain 'W-15'; got {:?}",
            pending_waves
        );
    }

    // -----------------------------------------------------------------------
    // Test 7: BC-7.03.092 postcondition 1 — stderr WAVE GATE REMINDER format
    // -----------------------------------------------------------------------

    /// BC-7.03.092 PC-1: stderr WAVE GATE REMINDER must match the exact bash
    /// source format (blank line before/after, indented wave lines, hint lines).
    ///
    /// From bash source:
    ///   `echo ""` → blank line (newline)
    ///   `echo "WAVE GATE REMINDER:"` → header
    ///   `echo "  - $w gate is pending. Run the gate before starting the next wave."`
    ///   `echo ""` → blank line
    ///   `echo "  Invoke /vsdd-factory:wave-gate or update .factory/wave-state.yaml"`
    ///   `echo "  with gate_status: passed (after running checks) or deferred (with rationale)."`
    ///
    /// Maps to AC-003 (exact format parity with bash source).
    #[test]
    fn test_BC_7_03_092_stderr_wave_gate_reminder_exact_format() {
        let (_, stderr, _) = dispatch(Some(YAML_ONE_PENDING));

        // Must not be empty
        assert!(
            !stderr.is_empty(),
            "BC-7.03.092 PC-1: stderr must contain WAVE GATE REMINDER when pending wave found"
        );

        // Leading newline (blank line before "WAVE GATE REMINDER:")
        assert!(
            stderr.starts_with('\n'),
            "BC-7.03.092 PC-1: stderr must begin with a newline (blank line before header); \
             got: {:?}",
            &stderr[..std::cmp::min(stderr.len(), 20)]
        );

        // WAVE GATE REMINDER: header
        assert!(
            stderr.contains("WAVE GATE REMINDER:\n"),
            "BC-7.03.092 PC-1: stderr must contain 'WAVE GATE REMINDER:\\n'; \
             got: {:?}",
            stderr
        );

        // Wave line with exact format
        assert!(
            stderr.contains(
                "  - W-15 gate is pending. Run the gate before starting the next wave.\n"
            ),
            "BC-7.03.092 PC-1: stderr must contain the wave reminder line with exact format; \
             got: {:?}",
            stderr
        );

        // Blank line after wave list
        // The blank line comes before the hint lines
        assert!(
            stderr.contains("\n\n"),
            "BC-7.03.092 PC-1: stderr must contain a blank line (\\n\\n) separating \
             wave list from hint; got: {:?}",
            stderr
        );

        // Invoke hint line
        assert!(
            stderr
                .contains("  Invoke /vsdd-factory:wave-gate or update .factory/wave-state.yaml\n"),
            "BC-7.03.092 PC-1: stderr must contain invocation hint line; got: {:?}",
            stderr
        );

        // Rationale line
        assert!(
            stderr.contains(
                "  with gate_status: passed (after running checks) or deferred (with rationale).\n"
            ),
            "BC-7.03.092 PC-1: stderr must contain rationale line; got: {:?}",
            stderr
        );
    }

    // -----------------------------------------------------------------------
    // Test 8: BC-7.03.092 — emit called BEFORE stderr write (ordering)
    // -----------------------------------------------------------------------

    /// BC-7.03.092 PC-1: emit_event is called before the stderr write.
    ///
    /// This test verifies ordering by checking that when only the emit callback
    /// is wired (stderr ignored), the event is still captured. The ordering
    /// assertion is structural — a separate ordering_log approach would require
    /// unsafe shared state; we verify each independently instead.
    ///
    /// The AC-003 spec says: "calls host::emit_event ... then writes to stderr".
    #[test]
    fn test_BC_7_03_092_emit_called_when_pending_wave_found() {
        use std::sync::{Arc, Mutex};

        let yaml = YAML_ONE_PENDING;
        let call_order: Arc<Mutex<Vec<&'static str>>> = Arc::new(Mutex::new(Vec::new()));

        let order_emit = Arc::clone(&call_order);
        let order_stderr = Arc::clone(&call_order);

        let _ = warn_pending_wave_gate_logic(
            make_stop_payload(),
            move || Some(yaml.to_string()),
            move |_event_type: &str, _fields: &[(&str, &str)]| {
                order_emit.lock().unwrap().push("emit");
            },
            move |_msg: &str| {
                order_stderr.lock().unwrap().push("stderr");
            },
        );

        let order = call_order.lock().unwrap().clone();
        assert_eq!(
            order,
            vec!["emit", "stderr"],
            "BC-7.03.092 PC-1: emit_event must be called BEFORE stderr write; \
             got order: {:?}",
            order
        );
    }

    // -----------------------------------------------------------------------
    // EC-004 / AC-003: two pending waves → comma-joined pending_waves field
    // -----------------------------------------------------------------------

    /// EC-004: multiple waves pending → pending_waves field is comma-joined.
    ///
    /// e.g. pending_waves = "W-15,W-16" in emit_event call.
    /// Stderr REMINDER lists each wave on its own line.
    ///
    /// Maps to AC-003 + AC-005 bats test case (b).
    #[test]
    fn test_BC_7_03_092_ec004_two_pending_waves_comma_joined_field() {
        let (events, stderr, _) = dispatch(Some(YAML_TWO_PENDING));

        assert_eq!(
            events.len(),
            1,
            "EC-004: exactly one emit_event call for two pending waves"
        );

        let pending_waves = events[0]
            .fields
            .iter()
            .find(|(k, _)| k == "pending_waves")
            .map(|(_, v)| v.as_str())
            .expect("EC-004: pending_waves field must be present");

        // Must contain both wave names comma-joined (order may vary by map iteration)
        assert!(
            pending_waves.contains("W-15") && pending_waves.contains("W-16"),
            "EC-004: pending_waves must contain both 'W-15' and 'W-16'; got: {:?}",
            pending_waves
        );
        assert!(
            pending_waves.contains(','),
            "EC-004: pending_waves must be comma-joined; got: {:?}",
            pending_waves
        );

        // Stderr must list each wave on its own line
        assert!(
            stderr.contains(
                "  - W-15 gate is pending. Run the gate before starting the next wave.\n"
            ),
            "EC-004: stderr must list W-15 wave line; got: {:?}",
            stderr
        );
        assert!(
            stderr.contains(
                "  - W-16 gate is pending. Run the gate before starting the next wave.\n"
            ),
            "EC-004: stderr must list W-16 wave line; got: {:?}",
            stderr
        );
    }

    // -----------------------------------------------------------------------
    // EC-001: waves key absent → silent exit 0
    // -----------------------------------------------------------------------

    /// EC-001: wave-state.yaml exists but `waves` key absent.
    ///
    /// Maps to AC-004(b) — YAML parse succeeds, waves key missing → exit 0.
    #[test]
    fn test_BC_7_03_091_ec001_waves_key_absent_silent_exit() {
        let (events, stderr, result) = dispatch(Some(YAML_NO_WAVES_KEY));

        assert!(
            matches!(result, vsdd_hook_sdk::HookResult::Continue),
            "EC-001: waves key absent → HookResult::Continue"
        );
        assert!(events.is_empty(), "EC-001: no emit when waves key absent");
        assert!(stderr.is_empty(), "EC-001: no stderr when waves key absent");
    }

    // -----------------------------------------------------------------------
    // EC-002: wave-state.yaml empty (zero bytes) → silent exit 0
    // -----------------------------------------------------------------------

    /// EC-002: wave-state.yaml empty → YAML parse returns None/empty → exit 0.
    ///
    /// Maps to AC-004(b).
    #[test]
    fn test_BC_7_03_091_ec002_empty_yaml_silent_exit() {
        let (events, stderr, result) = dispatch(Some(YAML_EMPTY));

        assert!(
            matches!(result, vsdd_hook_sdk::HookResult::Continue),
            "EC-002: empty YAML → HookResult::Continue"
        );
        assert!(events.is_empty(), "EC-002: no emit for empty YAML");
        assert!(stderr.is_empty(), "EC-002: no stderr for empty YAML");
    }

    // -----------------------------------------------------------------------
    // EC-003: wave entries without gate_status key → wave skipped
    // -----------------------------------------------------------------------

    /// EC-003: wave entry has no `gate_status` key → wave is skipped (not pending).
    ///
    /// Value::as_str on missing key returns None → not "pending" → skip.
    #[test]
    fn test_BC_7_03_091_ec003_wave_without_gate_status_skipped() {
        let yaml = r#"
waves:
  W-15:
    started: 2026-04-01
  W-16:
    gate_status: passed
"#;
        let (events, stderr, result) = dispatch(Some(yaml));

        assert!(
            matches!(result, vsdd_hook_sdk::HookResult::Continue),
            "EC-003: wave without gate_status key → HookResult::Continue (no pending)"
        );
        assert!(
            events.is_empty(),
            "EC-003: no emit when no wave has gate_status: pending"
        );
        assert!(
            stderr.is_empty(),
            "EC-003: no stderr when no wave has gate_status: pending"
        );
    }

    // -----------------------------------------------------------------------
    // EC-005: waves is a list (wrong structure) → graceful degradation
    // -----------------------------------------------------------------------

    /// EC-005: valid YAML but waves is a list (not a map) → as_mapping() returns
    /// None → HookResult::Continue, no output.
    ///
    /// Maps to AC-004(b) graceful degradation path.
    #[test]
    fn test_BC_7_03_091_ec005_waves_list_structure_graceful_degradation() {
        let (events, stderr, result) = dispatch(Some(YAML_WAVES_KEY_IS_LIST));

        assert!(
            matches!(result, vsdd_hook_sdk::HookResult::Continue),
            "EC-005: waves-as-list → HookResult::Continue (graceful degradation)"
        );
        assert!(events.is_empty(), "EC-005: no emit when waves is a list");
        assert!(stderr.is_empty(), "EC-005: no stderr when waves is a list");
    }

    // -----------------------------------------------------------------------
    // EC-007: waves key present but null or empty map → no pending waves
    // -----------------------------------------------------------------------

    /// EC-007(null): waves key present but value is null → as_mapping() returns
    /// None → HookResult::Continue, no output.
    #[test]
    fn test_BC_7_03_091_ec007_waves_null_silent_exit() {
        let (events, stderr, result) = dispatch(Some(YAML_WAVES_NULL));

        assert!(
            matches!(result, vsdd_hook_sdk::HookResult::Continue),
            "EC-007(null): waves: null → HookResult::Continue"
        );
        assert!(events.is_empty(), "EC-007(null): no emit for waves: null");
        assert!(stderr.is_empty(), "EC-007(null): no stderr for waves: null");
    }

    /// EC-007(empty): waves key present but value is empty map → no pending waves
    /// found → HookResult::Continue, no output.
    #[test]
    fn test_BC_7_03_091_ec007_waves_empty_map_silent_exit() {
        let (events, stderr, result) = dispatch(Some(YAML_WAVES_EMPTY_MAP));

        assert!(
            matches!(result, vsdd_hook_sdk::HookResult::Continue),
            "EC-007(empty): waves: {{}} → HookResult::Continue"
        );
        assert!(
            events.is_empty(),
            "EC-007(empty): no emit for empty waves map"
        );
        assert!(
            stderr.is_empty(),
            "EC-007(empty): no stderr for empty waves map"
        );
    }

    // -----------------------------------------------------------------------
    // EC-008: gate_status is non-string (integer, bool, list) → wave skipped
    // -----------------------------------------------------------------------

    /// EC-008: wave entry has non-string gate_status (integer 42, bool true, list)
    /// → Value::as_str returns None → not "pending" → wave skipped.
    ///
    /// The WASM port uses `data.get("gate_status").and_then(Value::as_str)` for
    /// exactly this graceful non-string handling.
    #[test]
    fn test_BC_7_03_091_ec008_non_string_gate_status_skipped() {
        let (events, stderr, result) = dispatch(Some(YAML_NON_STRING_GATE_STATUS));

        assert!(
            matches!(result, vsdd_hook_sdk::HookResult::Continue),
            "EC-008: non-string gate_status values must yield HookResult::Continue \
             (not panic, not emit)"
        );
        assert!(
            events.is_empty(),
            "EC-008: no emit when gate_status is non-string; got {} events",
            events.len()
        );
        assert!(
            stderr.is_empty(),
            "EC-008: no stderr when gate_status is non-string; got: {:?}",
            stderr
        );
    }

    // -----------------------------------------------------------------------
    // EC-009: CapabilityDenied (read_file returns None) → silent exit 0
    // -----------------------------------------------------------------------

    /// EC-009: host::read_file returns CapabilityDenied → read_wave_state()
    /// returns None → HookResult::Continue, no output (graceful, same as
    /// absent-file semantics).
    ///
    /// In the test harness, returning None from read_wave_state models both
    /// "file absent" and "CapabilityDenied" — both map to the None early-exit.
    /// AC-001 verifies capability declaration to prevent this path in production.
    #[test]
    fn test_BC_7_03_091_ec009_capability_denied_silent_exit() {
        // None from read_wave_state models CapabilityDenied
        let (events, stderr, result) = dispatch(None);

        assert!(
            matches!(result, vsdd_hook_sdk::HookResult::Continue),
            "EC-009: CapabilityDenied (None) → HookResult::Continue"
        );
        assert!(
            events.is_empty(),
            "EC-009: no emit on CapabilityDenied; got {} events",
            events.len()
        );
        assert!(
            stderr.is_empty(),
            "EC-009: no stderr on CapabilityDenied; got: {:?}",
            stderr
        );
    }

    // -----------------------------------------------------------------------
    // AC-003 parity: mixed waves (passed + pending + deferred) — only pending warned
    // -----------------------------------------------------------------------

    /// AC-003 parity: wave-state.yaml with mixed gate_status values.
    /// Only the pending wave triggers the warning; passed and deferred are silent.
    ///
    /// Canonical test vector from BC-7.03.092: "Stop with waves.W3.gate_status: pending
    /// produces stderr 'W3 gate is pending'."
    #[test]
    fn test_BC_7_03_092_mixed_waves_only_pending_emitted() {
        let (events, stderr, _) = dispatch(Some(YAML_MIXED));

        assert_eq!(
            events.len(),
            1,
            "BC-7.03.092 mixed: exactly one emit_event for the single pending wave"
        );

        let pending_waves = events[0]
            .fields
            .iter()
            .find(|(k, _)| k == "pending_waves")
            .map(|(_, v)| v.as_str())
            .expect("pending_waves field must be present");

        // Only W-15 is pending; W-14 (passed) and W-16 (deferred) must NOT appear
        assert!(
            pending_waves.contains("W-15"),
            "BC-7.03.092 mixed: pending_waves must contain 'W-15'"
        );
        assert!(
            !pending_waves.contains("W-14"),
            "BC-7.03.092 mixed: 'W-14' (passed) must NOT appear in pending_waves"
        );
        assert!(
            !pending_waves.contains("W-16"),
            "BC-7.03.092 mixed: 'W-16' (deferred) must NOT appear in pending_waves"
        );

        // Stderr must mention W-15 but not W-14 or W-16
        assert!(
            stderr.contains("W-15 gate is pending"),
            "BC-7.03.092 mixed: stderr must mention W-15; got: {:?}",
            stderr
        );
        assert!(
            !stderr.contains("W-14"),
            "BC-7.03.092 mixed: W-14 (passed) must NOT appear in stderr"
        );
        assert!(
            !stderr.contains("W-16"),
            "BC-7.03.092 mixed: W-16 (deferred) must NOT appear in stderr"
        );
    }

    // -----------------------------------------------------------------------
    // AC-001: hooks-registry.toml entry for warn-pending-wave-gate
    // -----------------------------------------------------------------------

    /// AC-001(a): hooks-registry.toml has a warn-pending-wave-gate entry with
    /// `plugin = "hook-plugins/warn-pending-wave-gate.wasm"`, `event = "Stop"`,
    /// `priority = 920`, `on_error = "continue"`.
    ///
    /// Traces to BC-7.03.091 postcondition 1 (identity & registry binding).
    /// RED GATE: Fails until T-6 updates hooks-registry.toml.
    #[test]
    fn test_BC_7_03_091_ac001_registry_has_wasm_plugin_entry() {
        let root = workspace_root();
        let registry_path = root.join("plugins/vsdd-factory/hooks-registry.toml");
        let registry_str = std::fs::read_to_string(&registry_path).unwrap_or_else(|e| {
            panic!(
                "AC-001: failed to read hooks-registry.toml at {:?}: {}",
                registry_path, e
            )
        });
        let registry: toml::Value = toml::from_str(&registry_str)
            .unwrap_or_else(|e| panic!("AC-001: failed to parse hooks-registry.toml: {}", e));

        let hooks = registry["hooks"]
            .as_array()
            .expect("AC-001: hooks-registry.toml must have a [[hooks]] array");

        // Find the warn-pending-wave-gate Stop entry
        let entry = hooks
            .iter()
            .find(|h| {
                h.get("name").and_then(|v| v.as_str()) == Some("warn-pending-wave-gate")
                    && h.get("event").and_then(|v| v.as_str()) == Some("Stop")
            })
            .expect(
                "AC-001: hooks-registry.toml must have a warn-pending-wave-gate Stop entry \
                 (fails until T-6 updates registry to WASM plugin path)",
            );

        // AC-001: plugin path must be native WASM (not legacy-bash-adapter)
        assert_eq!(
            entry["plugin"].as_str(),
            Some("hook-plugins/warn-pending-wave-gate.wasm"),
            "AC-001: plugin must be 'hook-plugins/warn-pending-wave-gate.wasm' \
             (native WASM, not legacy-bash-adapter)"
        );

        // AC-001: priority = 920
        assert_eq!(
            entry["priority"].as_integer(),
            Some(920),
            "AC-001: priority must be 920 (BC-7.03.091 registry binding)"
        );

        // AC-001: on_error = "continue"
        assert_eq!(
            entry["on_error"].as_str(),
            Some("continue"),
            "AC-001: on_error must be 'continue' (advisory hook, never blocks)"
        );

        // AC-001: script_path must be absent (WASM plugin, not bash script)
        assert!(
            entry.get("script_path").is_none(),
            "AC-001: script_path must be absent from WASM plugin registry entry"
        );

        // AC-001: shell_bypass_acknowledged must be absent
        assert!(
            entry.get("shell_bypass_acknowledged").is_none(),
            "AC-001: shell_bypass_acknowledged must be absent from WASM plugin registry entry"
        );
    }

    /// AC-001(b): hooks-registry.toml warn-pending-wave-gate entry has
    /// `[hooks.capabilities.read_file]` with `path_allow = [".factory/wave-state.yaml"]`.
    ///
    /// Missing capability declaration → CapabilityDenied at runtime (EC-009).
    /// RED GATE: Fails until T-6 adds the read_file capability block.
    #[test]
    fn test_BC_7_03_091_ac001_registry_has_read_file_capability() {
        let root = workspace_root();
        let registry_path = root.join("plugins/vsdd-factory/hooks-registry.toml");
        let registry_str = std::fs::read_to_string(&registry_path)
            .unwrap_or_else(|e| panic!("failed to read hooks-registry.toml: {}", e));
        let registry: toml::Value = toml::from_str(&registry_str)
            .unwrap_or_else(|e| panic!("failed to parse hooks-registry.toml: {}", e));

        let hooks = registry["hooks"]
            .as_array()
            .expect("hooks-registry.toml must have a [[hooks]] array");

        let entry = hooks
            .iter()
            .find(|h| {
                h.get("name").and_then(|v| v.as_str()) == Some("warn-pending-wave-gate")
                    && h.get("event").and_then(|v| v.as_str()) == Some("Stop")
                    && h.get("plugin").and_then(|v| v.as_str())
                        == Some("hook-plugins/warn-pending-wave-gate.wasm")
            })
            .expect(
                "AC-001(b): WASM warn-pending-wave-gate registry entry must exist \
                 (fails if T-6 not yet applied)",
            );

        // AC-001: read_file capability must be declared
        let capabilities = entry.get("capabilities").expect(
            "AC-001(b): capabilities block must be present in warn-pending-wave-gate entry",
        );

        let read_file = capabilities
            .get("read_file")
            .expect("AC-001(b): capabilities.read_file must be declared");

        let path_allow = read_file["path_allow"]
            .as_array()
            .expect("AC-001(b): capabilities.read_file.path_allow must be an array");

        let paths: Vec<&str> = path_allow.iter().filter_map(|v| v.as_str()).collect();
        assert!(
            paths.contains(&".factory/wave-state.yaml"),
            "AC-001(b): path_allow must include '.factory/wave-state.yaml'; got: {:?}",
            paths
        );

        // AC-006: exec_subprocess block must be absent
        assert!(
            capabilities.get("exec_subprocess").is_none(),
            "AC-006: exec_subprocess capability block must be absent from WASM entry \
             (python3 subprocess replaced by serde_yaml)"
        );
    }

    // -----------------------------------------------------------------------
    // AC-002: warn-pending-wave-gate.sh deleted from repository
    // -----------------------------------------------------------------------

    /// AC-002: `plugins/vsdd-factory/hooks/warn-pending-wave-gate.sh` must be
    /// absent (deleted per T-7).
    ///
    /// Traces to BC-7.03.091 invariant 1 (hook path lifecycle).
    /// RED GATE: Fails until T-7 deletes the bash script.
    #[test]
    fn test_BC_7_03_091_ac002_bash_script_deleted() {
        let root = workspace_root();
        let sh_path = root.join("plugins/vsdd-factory/hooks/warn-pending-wave-gate.sh");

        assert!(
            !sh_path.exists(),
            "AC-002 / BC-7.03.091 Invariant 1: warn-pending-wave-gate.sh must be \
             deleted from the repository (T-7); found at {:?}",
            sh_path
        );
    }

    // -----------------------------------------------------------------------
    // AC-006: no python3 or exec_subprocess reference in WASM crate
    // -----------------------------------------------------------------------

    /// AC-006: the warn-pending-wave-gate WASM crate must not invoke python3 or
    /// exec_subprocess.
    ///
    /// Traces to BC-7.03.091 invariant 2 (exit-code semantics).
    ///
    /// The test checks for subprocess invocation patterns (e.g. `Command::new("python3")`,
    /// `exec_subprocess`, `process::Command`, `std::process`), NOT documentation comments
    /// that mention python3 as a porting note. The porting note comment is expected and
    /// describes what was replaced.
    ///
    /// Also verifies that exec_subprocess is not referenced (no subprocess capability needed).
    ///
    /// NOTE: For integration tests in tests/, CARGO_MANIFEST_DIR is the crate
    /// root (the directory containing Cargo.toml), not the tests/ subdirectory.
    #[test]
    fn test_BC_7_03_091_ac006_no_python3_subprocess_invocation_in_wasm_crate() {
        // CARGO_MANIFEST_DIR = crates/hook-plugins/warn-pending-wave-gate/ (the crate root)
        let crate_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));

        // Check Cargo.toml: no exec_subprocess dependency
        let cargo_toml = std::fs::read_to_string(crate_dir.join("Cargo.toml"))
            .expect("AC-006: Cargo.toml must be readable");
        assert!(
            !cargo_toml.contains("exec_subprocess"),
            "AC-006: Cargo.toml must not reference 'exec_subprocess'"
        );

        // Check src/main.rs: no subprocess invocation patterns
        let main_rs = std::fs::read_to_string(crate_dir.join("src/main.rs"))
            .expect("AC-006: src/main.rs must be readable");
        // Check for actual subprocess invocation patterns (not documentation comments)
        for pattern in &[
            "std::process::Command",
            "exec_subprocess",
            r#"Command::new("python"#,
        ] {
            assert!(
                !main_rs.contains(pattern),
                "AC-006: src/main.rs must not contain subprocess invocation pattern '{}'",
                pattern
            );
        }

        // Check src/lib.rs: no subprocess invocation patterns
        let lib_rs = std::fs::read_to_string(crate_dir.join("src/lib.rs"))
            .expect("AC-006: src/lib.rs must be readable");
        // Check for actual subprocess invocation patterns (not documentation comments)
        for pattern in &[
            "std::process::Command",
            "exec_subprocess",
            r#"Command::new("python"#,
        ] {
            assert!(
                !lib_rs.contains(pattern),
                "AC-006: src/lib.rs must not contain subprocess invocation pattern '{}'",
                pattern
            );
        }

        // Verify serde_yaml is the YAML parser (not a subprocess call)
        // src/lib.rs must import serde_yaml for YAML parsing
        assert!(
            lib_rs.contains("serde_yaml"),
            "AC-006: src/lib.rs must use serde_yaml for YAML parsing \
             (python3 subprocess replaced per AC-006)"
        );
    }

    // -----------------------------------------------------------------------
    // AC-007: no bin/emit-event reference in WASM crate
    // -----------------------------------------------------------------------

    /// AC-007: warn-pending-wave-gate WASM crate must not reference bin/emit-event.
    /// host::emit_event replaces the bash _emit() function.
    ///
    /// Traces to BC-7.03.092 postcondition 1 (emit_event host fn).
    ///
    /// NOTE: For integration tests in tests/, CARGO_MANIFEST_DIR is the crate
    /// root (the directory containing Cargo.toml), not the tests/ subdirectory.
    #[test]
    fn test_BC_7_03_092_ac007_no_bin_emit_event_reference_in_wasm_crate() {
        // CARGO_MANIFEST_DIR = crates/hook-plugins/warn-pending-wave-gate/ (the crate root)
        let crate_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));

        for filename in &["src/main.rs", "src/lib.rs"] {
            let content = std::fs::read_to_string(crate_dir.join(filename))
                .unwrap_or_else(|e| panic!("AC-007: failed to read {}: {}", filename, e));
            assert!(
                !content.contains("bin/emit-event"),
                "AC-007: {} must not reference 'bin/emit-event'; replace with host::emit_event",
                filename
            );
        }
    }
}
