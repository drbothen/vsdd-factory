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
  - .factory/cycles/v1.0-brownfield-backfill/adv-s8.05-p6.md
  - plugins/vsdd-factory/hooks/validate-pr-review-posted.sh
  - crates/hook-sdk/src/payload.rs
  - crates/hook-sdk/src/host.rs
  - crates/hook-plugins/capture-pr-activity/src/lib.rs
input-hash: "df5d60e"
traces_to: prd.md
story_id: "S-8.05"
story_version: "1.6"
story_input_hash: "df5d60e"
pass_number: 7
pass: p7
previous_review: adv-s8.05-p6.md
target: story
target_file: .factory/stories/S-8.05-native-port-validate-pr-review-posted.md
verdict: SUBSTANTIVE
clock: 0_of_3
findings_critical: 1
findings_high: 0
findings_medium: 1
findings_low: 1
findings_nit: 0
---

# Adversarial Review Pass-7 — S-8.05 v1.6

## Finding ID Convention

`F-S805-P7-NNN`

## Part A — Pass-6 Fix Verification

| Fix ID | P6 Severity | Status | Evidence |
|---|---|---|---|
| F-P6-001 jq null-semantics → typed projection | HIGH | **CLOSED-CONDITIONAL** | T-3:423-449 uses `payload.<field>.as_deref()...` form. Conditional on typed fields existing — see F-P7-001. |
| F-P6-002 `envelope` undefined → typed projection | HIGH | **CLOSED-with-new-defect** | Verified zero `envelope.get(...)` in T-N task implementation. NEW defect: cited fields don't exist on HookPayload. |
| F-P6-003 AC-007 case (e) field names + lifecycle | MED | **CLOSED** | `event_name: "SubagentStop"` (was `event_type: "PostToolUse"`); `tool_name` removed. |
| F-P6-004 AC-008 `let _ =` form prohibition | MED | **CLOSED** | AC-008:313-319 forbids `let _ =`. T-5:462 uses bare statement. |
| F-P6-005 AC-001 `name` field | LOW | **CLOSED** | AC-001:190 includes `name`. |

## Part B — New Findings (Pass-7)

### F-S805-P7-001 — CRITICAL — Typed-projection fields cited in T-3/AC-003/AC-007 do NOT exist on HookPayload

- **Site:** T-3:423-449; AC-003:209-219; AC-007:294-301; Library row:538.
- **Evidence:**
  - Story claims: `payload.agent_type.as_deref()...`, `payload.subagent_name.as_deref()...`, `payload.last_assistant_message.as_deref()...`, `payload.result.as_deref()...`
  - Actual `crates/hook-sdk/src/payload.rs:15-53`: HookPayload has 7 fields: `event_name`, `tool_name`, `session_id`, `dispatcher_trace_id`, `tool_input`, `tool_response`, `plugin_config`. **No `agent_type`/`subagent_name`/`last_assistant_message`/`result` fields exist.**
  - Sibling `capture-pr-activity/src/lib.rs:15-21` uses `payload.tool_input.get("command")` indirect projection — no precedent for typed top-level SubagentStop fields.
- **Defect:** Cited code is a compile-time error against the SDK as it currently exists. T-3 cannot be implemented as written.
- **Required Fix:** Add `S-8.30` to `depends_on` (S-8.30 is the SDK extension story that adds these fields to HookPayload). Add T-0 STOP CHECK verifying HookPayload has the 4 fields before T-3 begins (mirrors S-8.04's T-0 STOP CHECK pattern for S-8.10).
- **Severity rationale (CRITICAL):** Mis-anchor; implementer would write non-compiling code. process-gap-D-183-A audit was supposed to verify EXACTLY this — verified syntactic absence of `envelope.get(...)` but not semantic resolvability of `payload.<field>` against SDK source. Audit gate is incomplete.

### F-S805-P7-002 — MED — Library Requirements row claims SDK provides typed BC-2.02.012 fields without pinning version

- **Site:** Library Requirements:537-538.
- **Defect:** Asserts workspace-current vsdd-hook-sdk provides BC-2.02.012 typed fields without pinning version. No `depends_on` link to S-8.30 in frontmatter:21.
- **Fix:** Add S-8.30 to depends_on OR pin SDK version.

### F-S805-P7-003 — LOW — `assumption_validations: []` empty despite D-183 Phase D dependency

- **Site:** Frontmatter:44.
- **Fix:** Add D-183 / BC-2.02.012 entry with verification step.

## Process-Gap Notes

- **[process-gap]** process-gap-D-183-A audit gate is structurally insufficient: tests for syntactic presence of typed projection but not field-existence on cited SDK struct. Future audit gate must include "verify cited typed fields exist on the named SDK struct" check.
- **[process-gap]** Pass-6 trajectory note "not converging" confirmed at pass-7: each pass uncovers next-deeper-layer issue (regex → paths → null/Option semantics → typed-projection → typed-field-existence). Spec workflow needs sibling-crate pattern verification gate.

## Verdict

**SUBSTANTIVE — clock RESETS to 0/3 HELD.**

1 CRITICAL (F-P7-001 typed fields don't exist) + 1 MED (SDK version not pinned) + 1 LOW (assumption_validations empty).

## Trajectory

| Pass | C | H | M | L | NIT | Total |
|------|---|---|---|---|-----|-------|
| P1 | 0 | 4 | 5 | 2 | 1 | 12 |
| P2 | 0 | 0 | 4 | 2 | 1 | 7 |
| P3 | 0 | 2 | 2 | 1 | 0 | 5 |
| P4 | 0 | 2 | 0 | 2 | 0 | 4 |
| P5 | 0 | 2 | 1 | 1 | 0 | 4 |
| P6 | 0 | 2 | 2 | 1 | 0 | 5 |
| P7 | 1 | 0 | 1 | 1 | 0 | 3 |

Each pass continues to surface novel-layer issues. Convergence requires SDK extension landing OR projection reframe.

## Novelty Assessment

Novelty: HIGH — F-S805-P7-001 is genuinely new and severity-elevated (CRITICAL). Pass-6 audit verified absence of `envelope.get(...)` syntactically; pass-7 fresh-context derivation read actual SDK `payload.rs` source and confirmed replacement symbols (`payload.agent_type` etc.) also do not resolve. Validates AgenticAKM "fresh-context compounding value" lesson — pass 7's fresh re-derivation from SDK source caught what 6 prior passes did not.

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 1 |
| HIGH | 0 |
| MEDIUM | 1 |
| LOW | 1 |
| NIT | 0 |

**Overall Assessment:** ITERATE — Phase F dependency wiring required (S-8.30 in depends_on + T-0 STOP CHECK).

**Convergence:** RESET to 0/3.

**Readiness:** NOT READY — F-P7-001 mechanical implementation blocker.
