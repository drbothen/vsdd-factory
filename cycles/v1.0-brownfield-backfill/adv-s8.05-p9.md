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
  - .factory/cycles/v1.0-brownfield-backfill/adv-s8.05-p8.md
  - .factory/specs/behavioral-contracts/ss-02/BC-2.02.012.md
  - .factory/specs/behavioral-contracts/ss-07/BC-7.04.040.md
  - .factory/specs/behavioral-contracts/ss-07/BC-7.04.041.md
  - .factory/specs/behavioral-contracts/ss-07/BC-7.04.042.md
  - .factory/specs/behavioral-contracts/ss-07/BC-7.04.043.md
  - .factory/specs/behavioral-contracts/ss-07/BC-7.04.044.md
  - crates/hook-sdk/src/payload.rs
input-hash: "eee327d"
traces_to: prd.md
pass: p9
previous_review: adv-s8.05-p8.md
story_id: "S-8.05"
story_version: "1.8"
story_input_hash: "eee327d"
pass_number: 9
target: story
target_file: .factory/stories/S-8.05-native-port-validate-pr-review-posted.md
verdict: NITPICK_ONLY
clock: 1_of_3
findings_critical: 0
findings_high: 0
findings_medium: 0
findings_low: 0
findings_nit: 1
---

# Adversarial Review Pass-9 — S-8.05 v1.8

## Finding ID Convention

`F-S805-P9-NNN`

## Part A — Pass-8 Fix Verification

Pass-8 SUBSTANTIVE (clock 0/3 held). v1.7 → v1.8 fix burst applied 5 closures:

| Finding | Severity | Fix Verification |
|---------|----------|-----------------|
| F-S805-P8-001 MED (`agent.as_str()` on `&str` — compile error in T-5 snippet) | MED | **CLOSED** — v1.8 T-5: `("subagent", agent.as_str())` → `("subagent", agent)`; `agent: &str` binding used directly; confirmed at T-5 emit_event tuple construction site. No residual `.as_str()` call on `&str` type. |
| F-S805-P8-002 MED (AC-007 first JSON example mislabeled as Case (e); content is Case (a)) | MED | **CLOSED** — v1.8 AC-007: first JSON example relabeled "Case (a) all-pass concrete input (for reference)"; second JSON example remains "Case (e) concrete input (no-verdict)". Label-to-content correspondence now correct. Bats test author will receive correct behavioral expectation. |
| F-S805-P8-003 LOW (T-0 STOP CHECK grep weaker than prose — field-name only, not Option<String>) | LOW | **CLOSED** — v1.8 T-0 STOP CHECK strengthened to `grep -E 'pub (agent_type\|subagent_name\|last_assistant_message\|result): Option<String>'`; bounds against exact type annotation. process-gap-D-185-A pattern applied. |
| F-S805-P8-004 LOW (assumption_validations: [] empty despite explicit T-0 STOP CHECK) | LOW | **CLOSED** — v1.8 frontmatter `assumption_validations` populated: S-8.30 SDK extension merged prerequisite + BC-2.02.012 typed projection compile guarantee. |
| F-S805-P8-005 NIT (T-7 cites "7 cases" but g.1/g.2 sub-cases imply 8) | NIT | **CLOSED** — v1.8 T-7 updated to "(7 cases per AC-007 + 2 agent-fallback sub-cases g.1/g.2 per T-5)"; implementer now has correct total for bats suite. |

**All 5 v1.8 fixes verified CLOSED with file:line evidence.**

**Universal-patch anchors: ALL PASS.** wasm32-wasip1 target confirmed; vsdd-hook-sdk path `../../hook-sdk` confirmed; hooks.json positive-verification confirmed; SS-01/02/04/07 subsystems confirmed; CAP-022 cross-CAP stretch disclosure confirmed; wave:15 [process-gap] confirmed; HOST_ABI_VERSION=1 confirmed; S-8.29 and S-8.30 references confirmed.

**Anti-fabrication HARD GATE: PASS.** BC-2.02.012 PC-5+PC-6 verbatim typed-projection chains present in T-3 and AC-003; BC-7.04.040/041/042/043/044 quotations verbatim; no BC text fabricated.

**process-gap-D-183-A audit: PASS.** Typed-projection fully specified; no `envelope.get(...)` references.

**process-gap-D-184-A audit: PASS.** T-0 STOP CHECK anchors against payload.rs field names with Option<String> type constraint; S-8.30 in depends_on.

**process-gap-D-185-A audit: PASS.** Method-resolution against declared binding types verified: `agent: &str` binding (from `unwrap_or("unknown")`) used directly in emit_event tuple — no `.as_str()` on `&str`. All projection chain sites type-correct.

## Part B — New Findings (Pass-9)

### CRITICAL / HIGH / MEDIUM / LOW

None.

### NIT

#### F-S805-P9-001 — T-5 uses unqualified `host::emit_event` while sibling capture-commit-activity uses fully-qualified `vsdd_hook_sdk::host::emit_event`

- **Severity:** NIT
- **Location:** S-8.05 T-5 — emit_event call snippet
- **Description:** S-8.05 T-5 references `host::emit_event(...)` without the crate-level qualifier. The sibling story `S-8.02` (capture-commit-activity) uses the fully-qualified form `vsdd_hook_sdk::host::emit_event(...)`. Both forms are valid Rust (assuming `use vsdd_hook_sdk::host;` is in scope), but the sibling-parity drift may cause the test author or implementer to question whether a `use` statement is required or whether the unqualified path is resolvable by default in the hook plugin crate context.
- **Proposed Fix (pending intent):** Align to fully-qualified `vsdd_hook_sdk::host::emit_event(...)` form across all sibling stories, or add an explicit `use vsdd_hook_sdk::host;` declaration note in T-5 to clarify the import prerequisite.
- **Disposition:** SKIP-FIX-eligible per S-7.03 (sibling-parity drift; cosmetic; does not affect spec correctness).

## Open Questions

None.

## Pass-10 Priors

1. Re-verify F-S805-P9-001 carryover: `host::emit_event` vs fully-qualified form — confirm SKIP-FIX maintained.
2. Anti-fabrication HARD GATE on BC-7.04.040/041/042/043/044 verbatim.
3. process-gap-D-185-A method-resolution check: confirm `agent` binding used as `&str` directly (no regression from F-P8-001 fix).
4. AC-007 Case (a)/(e) label correctness (verify F-P8-002 fix stable).

## Verdict

**NITPICK_ONLY** — clock 0/3 → **1/3** (first NIT-only pass for S-8.05 post-Phase F reset).

Single NIT: F-S805-P9-001 (T-5 uses unqualified `host::emit_event` while sibling capture-commit-activity uses fully-qualified form — sibling-parity drift; SKIP-FIX-eligible per S-7.03). All 5 v1.8 fixes verified CLOSED with file:line evidence. No content defects.

## Trajectory

| Pass | H | M | L | NIT | Total |
|------|---|---|---|-----|-------|
| p1 | 4 | 5 | 2 | 1 | 12 |
| p2 | 0 | 3 | 1 | 0 | 4 |
| p3 | 0 | 2 | 2 | 1 | 5 |
| p4 | 2 | 1 | 1 | 0 | 4 |
| p5 | 2 | 1 | 1 | 0 | 4 |
| p6 | 2 | 2 | 1 | 0 | 5 |
| p7 | 1 | 0 | 1 | 1 | 3 (CRITICAL closed via Phase F) |
| p8 | 0 | 2 | 2 | 1 | 5 (SUBSTANTIVE → v1.8 fix burst) |
| p9 | 0 | 0 | 0 | 1 | 1 (NITPICK_ONLY — first NIT-only pass) |

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 9 |
| **New findings** | 1 (F-S805-P9-001 host::emit_event sibling-parity drift) |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 1/1 = 1.0 |
| **Median severity** | NIT |
| **Trajectory** | 12→4→5→4→4→5→3→5→1 |
| **Verdict** | FINDINGS_REMAIN (clock 1/3; 2 more NITPICK_ONLY passes needed for ADR-013 convergence) |

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 0 |
| MEDIUM | 0 |
| LOW | 0 |
| NIT | 1 |

**Overall Assessment:** ADVANCE — first NIT-only pass for S-8.05 in the post-Phase F re-convergence cycle. All 5 v1.8 fix burst closures verified (compile-error fix, AC-007 label, T-0 type-strengthening, assumption_validations, T-7 case count). Remaining finding is cosmetic sibling-parity drift (SKIP-FIX-eligible per S-7.03).

**Convergence:** Clock 0/3 → **1/3**. Two more NITPICK_ONLY passes required for ADR-013 convergence.

**Readiness:** Pass-10 dispatch.
