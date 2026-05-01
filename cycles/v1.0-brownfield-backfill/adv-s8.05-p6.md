---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-05-01T00:00:00Z
phase: 2
inputs:
  - .factory/stories/S-8.05-native-port-validate-pr-review-posted.md
  - .factory/cycles/v1.0-brownfield-backfill/adv-s8.05-p5.md
  - plugins/vsdd-factory/hooks/validate-pr-review-posted.sh
  - plugins/vsdd-factory/hooks-registry.toml
  - crates/hook-plugins/capture-commit-activity/src/main.rs
  - crates/hook-plugins/capture-pr-activity/src/lib.rs
  - crates/hook-sdk/src/host.rs
  - crates/hook-sdk/src/payload.rs
input-hash: "e0882ac"
traces_to: prd.md
story_id: "S-8.05"
story_version: "1.5"
story_input_hash: "e0882ac"
pass_number: 6
pass: p6
previous_review: adv-s8.05-p5.md
target: story
target_file: .factory/stories/S-8.05-native-port-validate-pr-review-posted.md
verdict: SUBSTANTIVE
clock: 0_of_3
findings_critical: 0
findings_high: 2
findings_medium: 2
findings_low: 1
findings_nit: 0
---

# Adversarial Review Pass-6 — S-8.05 v1.5

## Finding ID Convention

`F-S805-P6-NNN`

## Part A — Pass-5 Fix Verification

| Fix ID | P5 Severity | Status | Evidence |
|---|---|---|---|
| F-S805-P5-001 T-5 emit_event bare statement form | HIGH | **CLOSED** | T-5 lines 395-407: bare `host::emit_event(...);` followed by `;` — no `let _`, no "discard result" prose. Sibling parity verified. |
| F-S805-P5-002 SS-04 canonical "Plugin Ecosystem" | HIGH | **CLOSED** | Lines 83 + 91 verified. No remaining "Hook Plugin Layer" anti-patterns in story body or anchors. (Changelog v1.5 entry 493 contains historical reference describing the fix — acceptable meta-reference.) |
| F-S805-P5-003 AC-005 → AC-003 dual-fallback re-anchor | MED | **CLOSED** | g.1/g.2 now anchored in AC-003 lines 201-205. AC-005 lines 212-214 clean. T-5 line 411 reads `(per AC-003)`. |
| F-S805-P5-004 T-3 RESULT lowercase | LOW | **CLOSED** | Line 383 uses lowercase `result`. No remaining bash-allcaps. |

**S-7.01 partial-fix regression sweep:** All 4 fixes propagated; no sites missed.

## Part B — New Findings (Pass-6 Fresh-Context Analysis)

### F-S805-P6-001 — HIGH — semantic-fidelity gap in dual-fallback null handling

- **Site:** T-3 lines 379 and 384.
- **Evidence:**
  - T-3 line 379: `envelope.get("agent_type").or(envelope.get("subagent_name")).and_then(|v| v.as_str()).unwrap_or("unknown")`
  - T-3 line 384: `envelope.get("last_assistant_message").or(envelope.get("result")).unwrap_or("")`
  - Bash source `validate-pr-review-posted.sh:21-22`: `jq -r '.agent_type // .subagent_name // "unknown"'` and `jq -r '.last_assistant_message // .result // empty'`.
- **Defect:** jq's `//` operator treats `null` AND `false` as alternative-triggering. `Option<&Value>::or(...)` only fires when the FIRST field is **absent** (None), NOT when present-but-`Value::Null`. If envelope contains `"agent_type": null, "subagent_name": "pr-reviewer"`:
  - jq behavior: agent_type null → use subagent_name → `"pr-reviewer"`
  - Rust snippet behavior: `Some(&Value::Null)` → `.or(...)` short-circuits → `.and_then(|v| v.as_str())` → None → `.unwrap_or("unknown")` → `"unknown"` → exit 0 immediately, **silently skipping validation**.
- **Required Fix:** T-3 must use jq-`//`-equivalent fallback that treats Null as "advance":
  ```rust
  fn pick<'a>(envelope: &'a serde_json::Value, keys: &[&str]) -> Option<&'a str> {
      keys.iter().find_map(|k| envelope.get(*k).and_then(|v| v.as_str()))
  }
  ```
- **Severity rationale:** Diverges from bash parity (BC-7.04.041); silent failure mode (SOUL.md #4); creates regression on real envelopes with explicit-null fields.

### F-S805-P6-002 — HIGH — `envelope` is undefined (HookPayload typed-projection mismatch)

- **Site:** T-3 lines 378-388.
- **Evidence:**
  - Story T-3 uses bare identifier `envelope.get(...)`.
  - SDK delivers `HookPayload` (typed struct) — `crates/hook-sdk/src/payload.rs:15-53` defines: `event_name`, `tool_name`, `session_id`, `dispatcher_trace_id`, `tool_input` (Value), `tool_response` (Option<Value>), `plugin_config` (Value).
  - HookPayload has NO top-level `agent_type`, `subagent_name`, `result`, `last_assistant_message`.
  - Sibling `capture-pr-activity/src/lib.rs:15-21` uses `payload.tool_input.get("command")` typed projection.
- **Defect:** `envelope` is unbound. T-3 doesn't specify whether `envelope` is `payload.tool_response.as_ref().unwrap_or(&Value::Null)`, `payload.tool_input`, or fresh deserialization of stdin. Implementer cannot mechanically write working code.
- **Required Fix:** T-3 must bind `envelope` explicitly:
  ```rust
  let envelope: &serde_json::Value = payload.tool_response.as_ref().unwrap_or(&serde_json::Value::Null);
  let agent = envelope.get("agent_type")...
  ```
  (Or whatever projection the dispatcher source-of-truth mandates for SubagentStop envelopes.)
- **Severity rationale:** Mechanical-correctness gap; type-system ambiguity; blocks working code generation. Same severity class as F-S805-P3-001 (SDK path).

### F-S805-P6-003 — MED — AC-007 case (e) envelope `event_type` field misrepresents lifecycle

- **Site:** AC-007 line 248-251.
- **Evidence:**
  - AC-007 case (e) JSON: `{"event_type": "PostToolUse", "tool_name": "Bash", "result": "...", "subagent_name": "pr-reviewer", "pr_number": "42"}`.
  - SDK schema field: `event_name` not `event_type` (per `payload.rs:19`).
  - Registry: `event = "SubagentStop"` for this hook.
- **Defect:** Field name `event_type` is wrong (SDK uses `event_name`); value `"PostToolUse"` is wrong (this hook fires on SubagentStop). bats test will fail to deserialize.
- **Required Fix:** Use `event_name: "SubagentStop"` (correct field name + correct lifecycle).

### F-S805-P6-004 — MED — AC-008 retains "let _ =" form as "acceptable" — partial regression of pass-5 fix #1

- **Site:** AC-008 lines 269-274.
- **Defect:** AC-008 still allows both bare-statement AND `let _ = ...` forms ("Both ... are acceptable forms"). Pass-5 fix landed at T-5 only; AC-008 was not updated. Contract (AC) and task disagree.
- **Required Fix:** AC-008 should also forbid `let _ = ...` (or demote to "tolerated") to match pass-5 directive and T-5 implementation.

### F-S805-P6-005 — LOW — AC-001 binding text omits `name` field

- **Site:** AC-001 line 182.
- **Defect:** AC-001 enumerates `event/priority/on_error/timeout_ms` but omits `name` (part of registry identity binding per BC-7.04.040).
- **Severity rationale:** Omission only; implementer would naturally retain. SKIP-FIX-eligible.

## Observations / Process Notes

- **[process-gap]** AC-008/T-5 emit_event call form has oscillated across v1.1 (forbade bare), v1.4 (allowed both), v1.5 (T-5 bare-only, AC-008 still both). Three consecutive cycles touching same construct without converging — fix-burst protocol does not enforce that fix directives propagate to ACs as well as tasks.

## Open Questions

- **OQ-1:** For SubagentStop envelopes, does the dispatcher pack `agent_type/subagent_name/last_assistant_message/result` into `HookPayload.tool_response`, or are there additional top-level fields? T-3 cannot be implemented without this answer.
- **OQ-2:** Is jq-`//` null-as-alternative parity a per-hook concern or a shared SDK helper concern (e.g., `vsdd-hook-sdk::helpers::pick`)?

## Pass-7 Priors

1. Verify F-S805-P6-001 fix: T-3 snippets handle `Value::Null` as "advance to fallback".
2. Verify F-S805-P6-002 fix: T-3 binds `envelope` explicitly to a HookPayload field.
3. Verify F-S805-P6-003 fix: AC-007 case (e) uses correct field names + lifecycle.
4. Verify F-S805-P6-004 fix: AC-008 prose reduces to bare-statement-only.
5. AC-001 `name` field optional addition.

## Verdict

**SUBSTANTIVE — clock RESETS to 0/3 HELD.**

2 HIGH (F-P6-001 dual-fallback null semantics, F-P6-002 envelope undefined typed-projection) + 2 MED + 1 LOW. Pass-5 fixes all CLOSED with no partial-fix regressions, but fresh-context surfaced novel issues at deeper layers (typed-projection mapping and jq-vs-Option semantics) that prior passes did not surface.

## Trajectory

| Pass | Findings | HIGH | MED | LOW | NIT |
|------|----------|------|-----|-----|-----|
| P1 | 12 | 4 | 5 | 2 | 1 |
| P2 | 7 | 0 | 4 | 2 | 1 |
| P3 | 5 | 2 | 2 | 1 | 0 |
| P4 | 4 | 2 | 0 | 2 | 0 |
| P5 | 4 | 2 | 1 | 1 | 0 |
| P6 | 5 | 2 | 2 | 1 | 0 |

**Trajectory: not converging.** Each pass uncovers novel HIGH-severity findings at progressively deeper layers (regex semantics → path semantics → null/Option semantics → typed-projection semantics). Story keeps refining at prose level but deep mechanical-correctness layers (HookPayload projection, jq-`//` parity) remain unspecified.

## Novelty Assessment

**Novelty: HIGH — findings are genuinely new.** F-S805-P6-001 (jq-`//` null semantics) and F-S805-P6-002 (`envelope` undefined against typed HookPayload) emerged from fresh re-derivation of bash source line 21-22 against Rust snippets, and from fresh inspection of `payload.rs` typed-struct shape against the story's bare `envelope.get(...)` references. Prior passes anchored on regex correctness, paths, registry diffs, and emit_event call-form — none audited typed-projection or jq-equivalence layers.

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 2 |
| MEDIUM | 2 |
| LOW | 1 |
| NIT | 0 |

**Overall Assessment:** ITERATE — Pass-5 fixes cleanly closed but fresh-context surfaces 2 HIGH novel findings at deeper mechanical-correctness layers; 2 MED test-fidelity / contract-task ambiguity; 1 LOW omission. Spec needs HookPayload typed-projection binding + jq-null-parity helper specification.

**Convergence:** RESET to 0/3. v1.6 fix burst required to specify typed-projection and null-handling semantics.

**Readiness:** NOT READY — F-P6-001 + F-P6-002 are mechanical blockers for implementation.
