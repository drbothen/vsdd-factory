---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-05-01T00:00:00Z
phase: 2
inputs:
  - .factory/stories/S-8.08-native-port-track-agent-start.md
  - .factory/cycles/v1.0-brownfield-backfill/adv-s8.08-p7.md
  - plugins/vsdd-factory/hooks/track-agent-start.sh
  - .factory/specs/behavioral-contracts/ss-07/BC-7.03.079.md
  - .factory/specs/behavioral-contracts/ss-07/BC-7.03.080.md
input-hash: "e0882ac"
traces_to: prd.md
pass: p8
previous_review: adv-s8.08-p7.md
story_id: "S-8.08"
story_version: "1.4"
story_input_hash: "e0882ac"
pass_number: 8
target: story
target_file: .factory/stories/S-8.08-native-port-track-agent-start.md
verdict: NITPICK_ONLY
clock: 3_of_3
findings_critical: 0
findings_high: 0
findings_medium: 0
findings_low: 1
findings_nit: 0
convergence: CONVERGENCE_REACHED
---

# Adversarial Review Pass-8 — S-8.08 v1.4

## Finding ID Convention

`F-S808-P8-NNN`

## Part A — Pass-7 Carryover Verification

- F-S808-P7-001 LOW (AC-005 scenario count "6" vs 5 bats + 1 perf): **STILL OPEN — SKIP-FIX held.** Cosmetic count ambiguity; does not affect bats test authoring (AC-005 text enumerates scenarios explicitly).
- F-S808-P7-002 LOW (F-P6-001/002 restatement): **SKIP-FIX held.**

**Strict E-8 D-2 bash-parity verification (empirical):**

Bash source `track-agent-start.sh:43-44` field set: `type=agent.start hook=track-agent-start matcher=Agent subagent=<SUBAGENT> [story_id=<ID>]` — exactly 5 fields (4 mandatory + 1 conditional).

T-3 emit_event vec literal: `("hook", "track-agent-start"), ("matcher", "Agent"), ("subagent", subagent.as_str())` + optional `("story_id", s)` — with first positional `"agent.start"` as event_type. Field count: 5 (4 + conditional). **CONFIRMED exact parity.**

`agent_id` audit: zero occurrences in T-3 emit_event call. **CONFIRMED.**
`tool_name` audit: zero occurrences in T-3 emit_event call. **CONFIRMED.**

Both `agent_id` and `tool_name` appear ONLY in: (a) AC-002a negative-assertion enforcement ("does NOT contain `agent_id`"), (b) descriptions of input-JSON fields being read from stdin (NOT emitted), and (c) changelog history. **All permitted contexts. CONFIRMED.**

**BC-2.02.012 typed projection:** Not applicable for S-8.08 — reads `tool_input.subagent_type` via existing HookPayload.tool_input JSON path pattern (PreToolUse envelope, not SubagentStop).

**Anti-fabrication HARD GATE: PASS.** BC-7.03.079 Invariant 1 character-exact match verified at story:194 ("Hook script identity (script path) and registry binding remain stable across the contract lifetime."); BC-7.03.080 Postcondition 1 faithfully paraphrased at AC-004:222-225 (story_id extraction two-pattern cascade). PASS.

**Universal-patch anchors: ALL PASS.** wasm32-wasip1 confirmed; vsdd-hook-sdk path `../../hook-sdk` confirmed; SS-04 "Hook Plugin Ecosystem" primary anchor confirmed (story lines 70-75); SS-01/SS-07 stretch confirmed; CAP-022 cross-CAP stretch disclosure confirmed; wave:15 [process-gap] confirmed; HOST_ABI_VERSION=1 confirmed; S-8.29 reference confirmed; TD-015 cross-reference confirmed.

**Sibling propagation audit:** S-8.03 (track-agent-stop) verified at pass-7 — no agent_id/tool_name in emit_event; consistent with S-8.08 parity. **CLEAN.**

## Part B — New Findings (Pass-8)

### CRITICAL / HIGH / MEDIUM

None.

### LOW

#### F-S808-P8-001 — Architecture Compliance row references "AC-002" but AC was split into AC-002a/AC-002b in v1.2

- **Severity:** LOW
- **Category:** ambiguous-language (cosmetic staleness)
- **Location:** S-8.08 Architecture Compliance Rules table — "Dispatcher routing" rule row, Enforcement column
- **Description:** The Architecture Compliance table enforcement column cites "AC-002" as the verification command for dispatcher routing (hooks.json.* zero-entry verification). In v1.2, AC-002 was split into AC-002a (identity stability / parity-audit) and AC-002b (hook path lifecycle / Architecture Compliance). The correct AC for dispatcher routing verification (grep for zero hooks.json entries) is AC-002b. The stale "AC-002" label is cosmetically incorrect.
- **Evidence:** Story lines ~202-214 define AC-002a (parity audit) and AC-002b (hook path lifecycle). The Architecture Compliance table references AC-002 without the suffix.
- **Proposed Fix:** Update the Enforcement column reference from "AC-002" to "AC-002b" in the Architecture Compliance Rules table.
- **Disposition:** Cosmetic staleness — SKIP-FIX-eligible per S-7.03. Clock advances regardless.

### NIT

None.

## Open Questions

None. Spec is stable.

## Pass-9 Priors

Pass-9 should not occur for S-8.08 — clock reaches CONVERGENCE_REACHED at 3_of_3 with this pass per ADR-013.

If for any reason a pass-9 is dispatched:
1. Re-verify strict E-8 D-2 bash-parity: agent_id and tool_name must not appear in T-3 emit_event call.
2. Re-verify BC-7.03.079 Invariant 1 character-exact match.
3. Verify AC-002b label propagation if F-S808-P8-001 was fixed.

## Verdict

**NITPICK_ONLY** — clock 2/3 → **3/3 — CONVERGENCE_REACHED** per ADR-013.

One LOW finding (AC-002 → AC-002b cosmetic staleness; SKIP-FIX-eligible). Three consecutive NITPICK_ONLY passes: p6 (3 LOW), p7 (2 LOW), p8 (1 LOW). Strict E-8 D-2 bash-parity empirically verified across all three passes. H/M descended monotonically post-p5 RESET. Anti-fabrication HARD GATE PASS.

## Trajectory

| Pass | H | M | L | NIT | Total | Notes |
|------|---|---|---|-----|-------|-------|
| p1 | — | — | — | — | — | Batch pass-1 |
| p2 | 2 | 5 | 2 | 0 | 9 | |
| p3 | 2 | 1 | 1 | 0 | 4 | v1.3 fix burst |
| p4 | 0 | 0 | 3 | 0 | 3 | (clock advance) |
| p5 | 1 | 1 | 2 | 0 | 4 | **CLOCK RESET** — parity violation |
| p6 | 0 | 0 | 3 | 0 | 3 | NITPICK_ONLY (clock 0/3→1/3); parity restored |
| p7 | 0 | 0 | 2 | 0 | 2 | NITPICK_ONLY (clock 1/3→2/3) |
| p8 | 0 | 0 | 1 | 0 | 1 | NITPICK_ONLY (clock 2/3→3/3 = CONVERGENCE_REACHED) |

Monotonically convergent post-p5 RESET. Trajectory (substantive: H+M): 12→9→4→3→4→3→2→1 — H/M descended to zero by p6; LOW-only floor maintained for 3 consecutive passes.

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 8 |
| **New findings** | 1 (F-S808-P8-001 AC-002 stale label) |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 1/1 = 1.0 |
| **Median severity** | LOW |
| **Trajectory** | p3=4 (2H+1M+1L) → p4 advance → p5=4 (1H+1M+2L RESET) → p6=3 (3L) → p7=2 (2L) → p8=1 (1L) |
| **Verdict** | CONVERGENCE_REACHED — clock 3/3 |

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 0 |
| MEDIUM | 0 |
| LOW | 1 |
| NIT | 0 |

**Overall Assessment:** CONVERGENCE_REACHED — strict E-8 D-2 bash-parity verified empirically (5 fields exact: type/hook/matcher/subagent/[story_id]; zero agent_id; zero tool_name). Three consecutive NITPICK_ONLY passes at LOW-only floor. Anti-fabrication HARD GATE PASS (BC-7.03.079 Inv 1 character-exact; BC-7.03.080 PC-1 faithful paraphrase). Universal-patch anchors PASS. Sibling propagation audit clean.

**Convergence:** **CONVERGENCE_REACHED** — clock 2/3 → **3/3** per ADR-013.

**Readiness:** S-8.08 v1.4 status: draft → **ready**. Spec is implementation-ready. Single LOW (AC-002b label) is cosmetic and non-blocking.
