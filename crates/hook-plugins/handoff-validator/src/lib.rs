//! handoff-validator — SubagentStop WASM hook plugin.
//!
//! Ports `plugins/vsdd-factory/hooks/handoff-validator.sh` to native WASM
//! (wasm32-wasip1) as a first-class consumer of the S-8.30 typed projection
//! on `HookPayload`.
//!
//! # Behavior (BC-7.03.042 / BC-7.03.043 / BC-7.03.044)
//!
//! Fires on every SubagentStop. Reads the payload's assistant message via the
//! BC-2.02.012 canonical 2-stage fallback chain, strips whitespace, and:
//!
//! - **Empty result (len == 0):** emits `hook.block` event with
//!   `reason=subagent_empty_result severity=warn`, writes a warning to stderr,
//!   exits 0. (BC-7.03.043)
//! - **Truncated result (1 <= len < 40):** emits `hook.block` event with
//!   `reason=subagent_truncated_result severity=warn result_len=N`, writes a
//!   warning to stderr, exits 0. (BC-7.03.044)
//! - **Sufficient result (len >= 40):** no output, exits 0. (BC-7.03.042
//!   postcondition 2)
//! - **Malformed / missing JSON:** graceful exit 0 (advisory; no jq
//!   dependency since serde_json is self-contained). (BC-7.03.042 invariant 2)
//!
//! # S-8.30 typed-projection usage
//!
//! All four BC-2.02.012 fields are accessed via the typed `Option<String>`
//! projection on `HookPayload` (added in S-8.30):
//!
//! ```text
//! payload.agent_type            — Option<String>
//! payload.subagent_name         — Option<String>
//! payload.last_assistant_message — Option<String>
//! payload.result                — Option<String>
//! ```
//!
//! **Agent identity (BC-2.02.012 Postcondition 5 canonical fallback chain):**
//! ```rust
//! let agent: &str = payload.agent_type.as_deref()
//!     .or(payload.subagent_name.as_deref())
//!     .unwrap_or("unknown");
//! ```
//!
//! **Assistant message (BC-2.02.012 Postcondition 6 canonical 2-stage chain):**
//! ```rust
//! // Handoff-validator divergence path per BC-2.02.012 EC-004:
//! // Bash 3-stage chain `.last_assistant_message // .result // .output // empty`
//! // becomes the canonical 2-stage chain in the WASM port (BC-2.02.012 only
//! // models 4 fields; `output` is intentionally not added to HookPayload to
//! // keep the SDK lean). Behavior parity is preserved because all three bash
//! // arms produce empty-string when all are absent.
//! let result: &str = payload.last_assistant_message.as_deref()
//!     .or(payload.result.as_deref())
//!     .unwrap_or("");
//! ```
//!
//! # Architecture compliance notes
//!
//! - `host::emit_event` replaces all `bin/emit-event` subprocess calls.
//!   `bin/emit-event` is NOT removed (E-8 D-10; deferred to S-8.29).
//! - No dependency on `legacy-bash-adapter` — forbidden per E-8 D-10.
//! - HOST_ABI_VERSION = 1 unchanged (additive projection per D-6 Option A).
//! - `on_error = "block"` in the registry means the dispatcher blocks the
//!   SubagentStop event if this plugin itself panics — the hook's own exit is
//!   always 0 (advisory warnings only, never hard-block).

use vsdd_hook_sdk::{HookPayload, HookResult};

// ---------------------------------------------------------------------------
// Pure decision logic (testable without wasmtime)
// ---------------------------------------------------------------------------

/// Classification of the assistant-message result length.
///
/// Used to drive `run_hook_logic`'s injectable emit/warn callbacks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResultClassification {
    /// `len == 0` — empty or whitespace-only result. (BC-7.03.043)
    Empty,
    /// `1 <= len < 40` — suspiciously short non-whitespace characters. (BC-7.03.044)
    /// Carries the computed non-whitespace character count.
    Short(usize),
    /// `len >= 40` — sufficient result. No warning. (BC-7.03.042)
    Sufficient,
}

/// Classify a result string by its non-whitespace character count.
///
/// Mirrors the bash `TRIMMED=$(echo -n "$RESULT" | tr -d '[:space:]')` /
/// `LEN=${#TRIMMED}` / `(( LEN == 0 ))` / `(( LEN < 40 ))` logic.
///
/// AC-003 / BC-7.03.043: `len == 0` → `Empty`.
/// AC-004 / BC-7.03.044: `1 <= len <= 39` → `Short(len)`.
/// AC-005 EC: `len == 40` → `Sufficient` (threshold is `< 40`, not `<= 40`).
pub fn classify_result(result: &str) -> ResultClassification {
    let trimmed_len: usize = result.chars().filter(|c| !c.is_whitespace()).count();
    match trimmed_len {
        0 => ResultClassification::Empty,
        n if n < 40 => ResultClassification::Short(n),
        _ => ResultClassification::Sufficient,
    }
}

/// Core hook logic with injectable side-effect callbacks.
///
/// Accepts a `HookPayload` and two callbacks so tests can drive every
/// branch without a WASM runtime.
///
/// `emit`: called with `(event_type, fields)` when a warning event is emitted.
/// `warn_stderr`: called with the stderr message string when a warning is emitted.
///
/// Returns `HookResult::Continue` (exit 0) in all cases — this hook is
/// advisory-only (BC-7.03.042 postcondition 2, BC-7.03.043 postcondition 2,
/// BC-7.03.044 postcondition 2).
pub fn handoff_validator_logic<E, W>(payload: HookPayload, emit: E, warn_stderr: W) -> HookResult
where
    E: FnOnce(&str, &[(&str, &str)]),
    W: FnOnce(&str),
{
    // BC-2.02.012 Postcondition 5: canonical agent identity fallback chain.
    let agent: &str = payload
        .agent_type
        .as_deref()
        .or(payload.subagent_name.as_deref())
        .unwrap_or("unknown");

    // BC-2.02.012 Postcondition 6: canonical 2-stage assistant-message chain.
    // Handoff-validator divergence per BC-2.02.012 EC-004: the bash 3-stage
    // chain `.last_assistant_message // .result // .output // empty` becomes
    // this 2-stage chain. The `.output` 3rd-arm is intentionally dropped
    // (BC-2.02.012 does not model `output` in HookPayload; behavior parity
    // is preserved for all observed envelope shapes).
    let result: &str = payload
        .last_assistant_message
        .as_deref()
        .or(payload.result.as_deref())
        .unwrap_or("");

    match classify_result(result) {
        ResultClassification::Empty => {
            // AC-003 / BC-7.03.043 postcondition 1.
            emit(
                "hook.block",
                &[
                    ("hook", "handoff-validator"),
                    ("matcher", "SubagentStop"),
                    ("reason", "subagent_empty_result"),
                    ("severity", "warn"),
                    ("subagent", agent),
                ],
            );
            let msg = format!(
                "handoff-validator: subagent '{}' returned an empty result.\n  This is a silent-failure risk — verify before continuing.\n",
                agent
            );
            warn_stderr(&msg);
        }
        ResultClassification::Short(len) => {
            // AC-004 / BC-7.03.044 postcondition 1.
            let len_str = len.to_string(); // bind temporary before slice
            emit(
                "hook.block",
                &[
                    ("hook", "handoff-validator"),
                    ("matcher", "SubagentStop"),
                    ("reason", "subagent_truncated_result"),
                    ("severity", "warn"),
                    ("subagent", agent),
                    ("result_len", len_str.as_str()),
                ],
            );
            let msg = format!(
                "handoff-validator: subagent '{}' returned only {} non-whitespace characters.\n  Suspiciously short — verify the subagent completed its task.\n",
                agent, len
            );
            warn_stderr(&msg);
        }
        ResultClassification::Sufficient => {
            // AC-005 case (c): no output, no event. BC-7.03.042 postcondition 2.
        }
    }

    HookResult::Continue
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use vsdd_hook_sdk::HookPayload;

    fn make_payload(json: &str) -> HookPayload {
        serde_json::from_str(json).expect("fixture should parse")
    }

    fn base_subagentstop(extra: &str) -> String {
        format!(
            r#"{{"event_name":"SubagentStop","session_id":"s","dispatcher_trace_id":"t"{}}}"#,
            if extra.is_empty() {
                String::new()
            } else {
                format!(",{}", extra)
            }
        )
    }

    // ── classify_result unit tests ──────────────────────────────────────────

    /// BC-7.03.043: empty string → Empty.
    #[test]
    fn test_BC_7_03_043_classify_empty_string() {
        assert_eq!(classify_result(""), ResultClassification::Empty);
    }

    /// BC-7.03.043: whitespace-only → Empty.
    #[test]
    fn test_BC_7_03_043_classify_whitespace_only() {
        assert_eq!(classify_result("   \t\n"), ResultClassification::Empty);
    }

    /// BC-7.03.044: 5 non-whitespace chars → Short(5).
    #[test]
    fn test_BC_7_03_044_classify_short_5_chars() {
        assert_eq!(classify_result("hello"), ResultClassification::Short(5));
    }

    /// AC-005 case (d): exactly 39 non-whitespace chars → Short(39).
    #[test]
    fn test_BC_7_03_044_classify_exactly_39_chars_is_short() {
        let s: String = "a".repeat(39);
        assert_eq!(classify_result(&s), ResultClassification::Short(39));
    }

    /// AC-005 case (e): exactly 40 non-whitespace chars → Sufficient.
    /// Threshold is `< 40`, NOT `<= 40` — off-by-one parity with bash `(( LEN < 40 ))`.
    #[test]
    fn test_BC_7_03_044_classify_exactly_40_chars_is_sufficient() {
        let s: String = "a".repeat(40);
        assert_eq!(classify_result(&s), ResultClassification::Sufficient);
    }

    /// BC-7.03.042: 50 non-whitespace chars → Sufficient.
    #[test]
    fn test_BC_7_03_042_classify_50_chars_is_sufficient() {
        let s: String = "a".repeat(50);
        assert_eq!(classify_result(&s), ResultClassification::Sufficient);
    }

    /// EC-004: whitespace mixed with content — only non-whitespace chars counted.
    #[test]
    fn test_BC_7_03_044_classify_counts_nonwhitespace_only() {
        // "a b c" has 3 non-whitespace chars → Short(3)
        assert_eq!(classify_result("a b c"), ResultClassification::Short(3));
    }

    // ── handoff_validator_logic: empty result path (BC-7.03.043) ──────────

    /// BC-7.03.043 postcondition 1: empty last_assistant_message → emit
    /// subagent_empty_result event and stderr warning.
    #[test]
    fn test_BC_7_03_043_empty_result_emits_event_and_stderr() {
        let payload = make_payload(&base_subagentstop(
            r#""agent_type":"test-agent","last_assistant_message":"""#,
        ));

        let mut emitted_type: Option<String> = None;
        let mut emitted_fields: Vec<(String, String)> = Vec::new();
        let mut stderr_msg: Option<String> = None;

        handoff_validator_logic(
            payload,
            |event_type, fields| {
                emitted_type = Some(event_type.to_string());
                emitted_fields = fields
                    .iter()
                    .map(|(k, v)| (k.to_string(), v.to_string()))
                    .collect();
            },
            |msg| {
                stderr_msg = Some(msg.to_string());
            },
        );

        assert_eq!(emitted_type.as_deref(), Some("hook.block"), "must emit hook.block");
        let reason = emitted_fields
            .iter()
            .find(|(k, _)| k == "reason")
            .map(|(_, v)| v.as_str());
        assert_eq!(reason, Some("subagent_empty_result"), "reason must be subagent_empty_result");
        let severity = emitted_fields
            .iter()
            .find(|(k, _)| k == "severity")
            .map(|(_, v)| v.as_str());
        assert_eq!(severity, Some("warn"), "severity must be warn");
        let subagent = emitted_fields
            .iter()
            .find(|(k, _)| k == "subagent")
            .map(|(_, v)| v.as_str());
        assert_eq!(subagent, Some("test-agent"), "subagent field must carry agent identity");
        let msg = stderr_msg.unwrap();
        assert!(msg.contains("empty result"), "stderr must contain 'empty result'");
        assert!(msg.contains("test-agent"), "stderr must contain agent name");
    }

    /// BC-7.03.043: whitespace-only last_assistant_message → treated as empty.
    #[test]
    fn test_BC_7_03_043_whitespace_only_result_is_empty() {
        let payload = make_payload(&base_subagentstop(
            r#""agent_type":"agent-x","last_assistant_message":"   \t\n   ""#,
        ));

        let mut got_empty = false;
        handoff_validator_logic(
            payload,
            |_, fields| {
                if fields.iter().any(|(k, v)| *k == "reason" && *v == "subagent_empty_result") {
                    got_empty = true;
                }
            },
            |_| {},
        );
        assert!(got_empty, "whitespace-only result must trigger subagent_empty_result");
    }

    // ── handoff_validator_logic: short result path (BC-7.03.044) ──────────

    /// BC-7.03.044 postcondition 1: 5-char result → emit subagent_truncated_result.
    #[test]
    fn test_BC_7_03_044_short_result_emits_truncated_event() {
        let payload = make_payload(&base_subagentstop(
            r#""agent_type":"short-agent","last_assistant_message":"hello""#,
        ));

        let mut emitted_type: Option<String> = None;
        let mut emitted_fields: Vec<(String, String)> = Vec::new();
        let mut stderr_msg: Option<String> = None;

        handoff_validator_logic(
            payload,
            |event_type, fields| {
                emitted_type = Some(event_type.to_string());
                emitted_fields = fields
                    .iter()
                    .map(|(k, v)| (k.to_string(), v.to_string()))
                    .collect();
            },
            |msg| {
                stderr_msg = Some(msg.to_string());
            },
        );

        assert_eq!(emitted_type.as_deref(), Some("hook.block"));
        let reason = emitted_fields
            .iter()
            .find(|(k, _)| k == "reason")
            .map(|(_, v)| v.as_str());
        assert_eq!(reason, Some("subagent_truncated_result"));
        let result_len = emitted_fields
            .iter()
            .find(|(k, _)| k == "result_len")
            .map(|(_, v)| v.as_str());
        assert_eq!(result_len, Some("5"), "result_len must be 5");
        let msg = stderr_msg.unwrap();
        assert!(msg.contains("non-whitespace characters"), "stderr must mention non-whitespace characters");
        assert!(msg.contains("5"), "stderr must contain the char count");
    }

    /// AC-005 case (d): exactly 39 chars → warning emitted (below threshold).
    #[test]
    fn test_BC_7_03_044_exactly_39_chars_emits_warning() {
        let s = "a".repeat(39);
        let json = base_subagentstop(&format!(r#""last_assistant_message":"{}""#, s));
        let payload = make_payload(&json);

        let mut emitted = false;
        handoff_validator_logic(
            payload,
            |_, fields| {
                if fields.iter().any(|(k, v)| *k == "reason" && *v == "subagent_truncated_result") {
                    emitted = true;
                }
            },
            |_| {},
        );
        assert!(emitted, "39-char result must trigger truncation warning");
    }

    /// AC-005 case (e): exactly 40 chars → NO warning.
    #[test]
    fn test_BC_7_03_044_exactly_40_chars_no_warning() {
        let s = "a".repeat(40);
        let json = base_subagentstop(&format!(r#""last_assistant_message":"{}""#, s));
        let payload = make_payload(&json);

        let mut emitted = false;
        handoff_validator_logic(
            payload,
            |_, _| { emitted = true; },
            |_| {},
        );
        assert!(!emitted, "40-char result must NOT trigger any warning");
    }

    // ── handoff_validator_logic: sufficient result (BC-7.03.042) ──────────

    /// BC-7.03.042: 50-char result → no event, no stderr.
    #[test]
    fn test_BC_7_03_042_sufficient_result_no_output() {
        let s = "a".repeat(50);
        let json = base_subagentstop(&format!(r#""last_assistant_message":"{}""#, s));
        let payload = make_payload(&json);

        let mut emitted = false;
        let mut warned = false;
        handoff_validator_logic(
            payload,
            |_, _| { emitted = true; },
            |_| { warned = true; },
        );
        assert!(!emitted, "sufficient result must not emit any event");
        assert!(!warned, "sufficient result must not write any stderr");
    }

    // ── BC-2.02.012 typed projection: fallback chain behavior ─────────────

    /// BC-2.02.012 Postcondition 5: agent_type absent, subagent_name used.
    #[test]
    fn test_BC_7_03_043_agent_identity_fallback_to_subagent_name() {
        let payload = make_payload(&base_subagentstop(
            r#""subagent_name":"fallback-agent","last_assistant_message":"""#,
        ));

        let mut subagent_in_event: Option<String> = None;
        handoff_validator_logic(
            payload,
            |_, fields| {
                subagent_in_event = fields
                    .iter()
                    .find(|(k, _)| *k == "subagent")
                    .map(|(_, v)| v.to_string());
            },
            |_| {},
        );
        assert_eq!(
            subagent_in_event.as_deref(),
            Some("fallback-agent"),
            "must fall back to subagent_name when agent_type is absent"
        );
    }

    /// BC-2.02.012 Postcondition 5: both absent → "unknown".
    #[test]
    fn test_BC_7_03_043_agent_identity_defaults_to_unknown() {
        let payload = make_payload(&base_subagentstop(r#""last_assistant_message":"""#));

        let mut subagent_in_event: Option<String> = None;
        handoff_validator_logic(
            payload,
            |_, fields| {
                subagent_in_event = fields
                    .iter()
                    .find(|(k, _)| *k == "subagent")
                    .map(|(_, v)| v.to_string());
            },
            |_| {},
        );
        assert_eq!(
            subagent_in_event.as_deref(),
            Some("unknown"),
            "both identity fields absent → subagent must be 'unknown'"
        );
    }

    /// BC-2.02.012 Postcondition 6 / EC-004: last_assistant_message absent,
    /// result used as fallback.
    #[test]
    fn test_BC_7_03_043_message_fallback_to_result_field() {
        // result = "" (empty) — should still warn via fallback chain
        let payload = make_payload(&base_subagentstop(r#""result":"""#));

        let mut got_empty = false;
        handoff_validator_logic(
            payload,
            |_, fields| {
                if fields.iter().any(|(k, v)| *k == "reason" && *v == "subagent_empty_result") {
                    got_empty = true;
                }
            },
            |_| {},
        );
        assert!(
            got_empty,
            "missing last_assistant_message with empty result must still warn"
        );
    }

    // ── AC-006 / BC-7.03.042 invariant 2: malformed JSON graceful exit ─────

    /// AC-006 / BC-7.03.042 invariant 2: malformed JSON → HookResult::Continue
    /// (graceful exit 0, advisory). Verified here by direct serde_json check;
    /// the main.rs entry point wraps deserialization errors and exits 0.
    #[test]
    fn test_BC_7_03_042_malformed_json_deserialize_fails_gracefully() {
        let result: Result<HookPayload, _> = serde_json::from_str("not json {{{");
        // Deserialization fails — the entry point handles this gracefully (exit 0).
        assert!(result.is_err(), "malformed JSON must produce a serde error (handled in main.rs)");
    }

    // ── HookResult always Continue (advisory-only) ─────────────────────────

    /// BC-7.03.042 postconditions 2+3: hook always returns HookResult::Continue.
    #[test]
    fn test_BC_7_03_042_hook_always_returns_continue() {
        for json in [
            base_subagentstop(r#""last_assistant_message":"""#), // empty
            base_subagentstop(&format!(r#""last_assistant_message":"{}""#, "a".repeat(5))), // short
            base_subagentstop(&format!(r#""last_assistant_message":"{}""#, "a".repeat(50))), // sufficient
        ] {
            let payload = make_payload(&json);
            let result = handoff_validator_logic(payload, |_, _| {}, |_| {});
            assert_eq!(
                result,
                HookResult::Continue,
                "hook must always return Continue (advisory-only, exit 0)"
            );
        }
    }
}
