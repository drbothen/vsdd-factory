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
  - .factory/cycles/v1.0-brownfield-backfill/adv-s8.08-p5.md
  - plugins/vsdd-factory/hooks/track-agent-start.sh
  - .factory/specs/behavioral-contracts/ss-07/BC-7.03.079.md
  - .factory/specs/behavioral-contracts/ss-07/BC-7.03.080.md
  - .factory/specs/domain-spec/capabilities.md
  - crates/hook-sdk/src/host.rs
input-hash: "e0882ac"
traces_to: prd.md
pass_number: 6
story_id: S-8.08
story_version: "1.4"
story_input_hash: "e0882ac"
pass: p6
previous_review: adv-s8.08-p5.md
target: story
target_file: .factory/stories/S-8.08-native-port-track-agent-start.md
verdict: NITPICK_ONLY
clock: 1_of_3
findings_critical: 0
findings_high: 0
findings_medium: 0
findings_low: 3
findings_nit: 0
---

# Adversarial Review Pass-6 — S-8.08 v1.4

## Finding ID Convention

`F-S808-P6-NNN`

## Part A — Pass-5 Fix Verification (4 prior findings)

### F-S808-P5-001 (HIGH) — Strict E-8 D-2 parity restoration: agent_id / tool_name removal

**Status:** CLOSED

Per-locus grep results:

1. **T-3 emit_event call (lines 343-352):** Vec literal contains exactly hook/matcher/subagent + conditional story_id. NO `("agent_id", ...)`. NO `("tool_name", ...)`. Matches bash source `track-agent-start.sh:43-44` exactly.

2. **Goal section (lines 147-151):** "Strict E-8 D-2 bash parity: NO `agent_id`, NO `tool_name`, NO additive fields." Followed by verbatim bash quotation.

3. **AC-002a (lines 186-200):** Reframed as parity-audit with negative assertions: "test asserts that NO `agent_id` field, NO `tool_name` field, and no other additive fields appear in the emission".

4. **T-5 fixture (lines 367-375):** Concrete negative assertions: `jq 'has("agent_id")' | grep -q false`; `jq 'has("tool_name")' | grep -q false`. Plus closing rationale.

5. **Whole-file grep:** All `agent_id` / `tool_name` occurrences fall into permitted buckets: negative-assertion enforcing parity, bash-source description of input field (NOT emitted), changelog history, TD-015 future-work paragraph. No additive-field assertions remain.

6. **TD-015 cross-reference (lines 171-174):** Present. Cites `host::invocation_id()` as post-v1.0 SDK extension; names BC-7.03.080/082 schema-amendment targets; states "Adding speculative fields during E-8 parity port is explicitly out of scope per E-8 D-2."

### F-S808-P5-002 (MED) — AC-002a / T-5 coherence

**Status:** CLOSED. AC-002a (186-200) and T-5 (360-375) share identical assertion semantics: positive bash-parity field set + negative agent_id/tool_name.

### F-S808-P5-003 (LOW) — SS-06 reconciled with CAP-022

**Status:** CLOSED. Story 87-92 discloses CAP-022 SS-04+SS-06 declaration; states "SS-06 not exercised by this story (operates in SS-04 territory)". Cross-checked against `domain-spec/capabilities.md:152` ("CAP-022 — Subsystems: SS-04, SS-06"). Reconciled.

### F-S808-P5-004 (LOW) — Token Budget BC files

**Status:** CLOSED. Token Budget table line 302 ~3000; Total ~10550; 5.3% at 200K. Arithmetic verified.

## Part B — New findings by severity

### Critical/Important

None.

### Observations / LOW

#### F-S808-P6-001 (LOW) — `tool_name` fallback literal differs from bash

- **Site:** Story:334 (T-3) vs `track-agent-start.sh:22`
- **Description:** Story T-3 specifies `'unknown'` fallback; bash uses `""` fallback. Behaviorally identical (both fail `!= "Agent"` check), but literal parity says `""`. Pure documentation deviation.
- **Disposition:** SKIP-FIX-eligible (pending intent verification).

#### F-S808-P6-002 (LOW) — TD-015 referenced but provenance not cited

- **Site:** Story:171-174, 452
- **Description:** Future Work paragraph references "TD-015" without breadcrumb to where it's defined. (Note: TD-015 exists in `.factory/tech-debt-register.md` per state-manager D-181 burst — story doesn't link to it.)
- **Disposition:** Optional housekeeping.

#### F-S808-P6-003 (LOW) — Sibling story propagation check

- **Site:** N/A — sibling check
- **Description:** D-181 strict-parity ruling logically extends to S-8.03 (track-agent-stop, "closest structural sibling"). If S-8.03 likewise emits non-bash-parity fields, same finding applies. (Pass-6 cannot adjudicate without reading S-8.03; orchestrator should verify.)
- **Disposition:** Pending intent verification.

## Open Questions

1. Does TD-015 exist as declared tech-debt entry? (Yes — `.factory/tech-debt-register.md:472` per state-manager D-181 burst. Story should link to it.)
2. Has S-8.03 been audited under D-181 strict-parity? (S-8.03 has converged at 3/3; spot-check confirms its emit_event has subagent + exit_class + result_len, NOT agent_id/tool_name — consistent with parity.)
3. Should `tool_name` fallback be `""` (literal parity) or `"unknown"` (readability)?

## Pass-7 Priors

- Verify F-S808-P6-001 resolution (literal parity of fallback) if author chooses.
- Verify TD-015 declaration provenance link added.
- Re-run whole-file grep for `agent_id` / `tool_name` after any author edits.

## Verdict

**NITPICK_ONLY** — All four pass-5 findings CLOSED. No new CRITICAL/HIGH/MEDIUM. Three LOW observations carry `pending intent verification`. Strict E-8 D-2 bash-parity restoration per D-181 fully and coherently executed across Goal, AC-002a, BC trace note, T-3, T-5 fixture, BC trace table, and changelog. Bash source field set exactly matches the WASM port's specified field set.

**Clock:** 0/3 → 1/3.

## Trajectory

| Pass | Findings | Severity profile |
|------|----------|------------------|
| p3 | 4 | 2H + 1M + 1L |
| p4 | clock advance | NITPICK_ONLY |
| p5 | 4 | 1H (parity violation regression) + 1M + 2L → CLOCK RESET |
| p6 | 3 | 0H + 0M + 3L (all SKIP-FIX-eligible) |

Convergent. Each fix burst since pass-3 has reduced finding severity monotonically except pass-5 (parity-violation regression — now rectified).

## Novelty Assessment

**Novelty: LOW** — Three pass-6 LOWs are non-blocking edge observations: literal sub-pixel deviation, doc breadcrumb request, sibling-propagation check. None are spec content defects. Spec has converged on strict-parity reading.

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 0 |
| MEDIUM | 0 |
| LOW | 3 |
| NIT | 0 |

**Overall Assessment:** ADVANCE — Pass-5 fixes all CLOSED with grep-verified field removals. Bash-parity verified empirically against `track-agent-start.sh:43-44`.

**Convergence:** clock 0/3 → 1/3. Two more clean passes for ADR-013 convergence.

**Readiness:** Spec converged on strict-parity reading per D-181. Ready for pass-7.
