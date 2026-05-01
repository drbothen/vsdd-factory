---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-05-01T00:00:00Z
phase: 2
inputs:
  - .factory/stories/S-8.03-native-port-track-agent-stop.md
  - .factory/cycles/v1.0-brownfield-backfill/adv-s8.03-p5.md
  - .factory/specs/behavioral-contracts/ss-02/BC-2.02.012.md
  - .factory/specs/behavioral-contracts/ss-07/BC-7.03.081.md
  - .factory/specs/behavioral-contracts/ss-07/BC-7.03.082.md
  - crates/hook-sdk/src/payload.rs
input-hash: "95d6e01"
traces_to: prd.md
pass: p6
previous_review: adv-s8.03-p5.md
story_id: "S-8.03"
story_version: "1.5"
story_input_hash: "95d6e01"
pass_number: 6
target: story
target_file: .factory/stories/S-8.03-native-port-track-agent-stop.md
verdict: NITPICK_ONLY
clock: 2_of_3
findings_critical: 0
findings_high: 0
findings_medium: 0
findings_low: 2
findings_nit: 1
---

# Adversarial Review Pass-6 — S-8.03 v1.5 (post-D-183 Phase F reset)

## Finding ID Convention

`F-S803-P6-NNN`

## Part A — Pass-5 Carryover Verification

Pass-5 findings (all SKIP-FIX, no fix burst applied between p5 and p6):

| Finding | Severity | Status |
|---------|----------|--------|
| F-S803-P5-001 LOW (T-0 STOP CHECK does not bound gate against Option<String> typing) | LOW | CARRYOVER — SKIP-FIX held per S-7.03 (pending intent) |
| F-S803-P5-002 LOW (depends_on lists S-8.30 but reverse blocks arrow on S-8.30 unverified) | LOW | CARRYOVER — pending intent verification |
| F-S803-P5-003 NIT (Changelog v1.5 entry doesn't explicit-tag process-gap-D-184-A) | NIT | CARRYOVER — SKIP-FIX held per S-7.03 |
| F-S803-P5-004 NIT (T-0 STOP CHECK grep portability across BSD/GNU, carryover F-P4D183-001) | NIT | CARRYOVER — SKIP-FIX held per S-7.03 |

**Universal-patch anchors: ALL PASS.** wasm32-wasip1 target confirmed; vsdd-hook-sdk path `../../hook-sdk` confirmed; hooks.json positive-verification confirmed; SS-04 in subsystems confirmed; CAP-022 cross-CAP stretch disclosure confirmed; wave:15 [process-gap] confirmed; HOST_ABI_VERSION=1 confirmed; S-8.29 and S-8.30 references confirmed.

**Anti-fabrication HARD GATE: PASS.** BC-2.02.012 PC-5+PC-6 verbatim typed-projection chains present in T-3; BC-7.03.081 and BC-7.03.082 quotations verbatim; no BC text fabricated.

**process-gap-D-183-A audit: PASS.** Typed-projection fully specified (`payload.agent_type.as_deref()...`, `payload.last_assistant_message.as_deref()...`). No `envelope.get(...)` references.

**process-gap-D-184-A audit: PASS.** T-0 STOP CHECK anchors against payload.rs field names; S-8.30 in depends_on.

**process-gap-D-185-A audit: PASS.** Method-resolution against declared binding types checked; no `.as_str()` on `&str` binding; all typed-projection chains type-correct per BC-2.02.012 PC-5+PC-6.

## Part B — New Findings (Pass-6)

### CRITICAL / HIGH / MEDIUM

None.

### LOW

#### F-S803-P6-001 — T-3 `set +e` PSI invariant unenforced (pending intent)

- **Severity:** LOW (pending intent verification)
- **Location:** S-8.03 T-3 — bash emit_event call block
- **Description:** S-8.03 T-3 issues the `emit_event` call in a context where `set +e` may not be explicitly established for the subprocess invocation. Sibling S-8.02 T-3 establishes an explicit `set +e` guard before the emit_event call to prevent premature hook exit on non-zero SDK return. If S-8.03's T-3 relies on the outer script PSI invariant without local scoping, a future refactor that wraps the call in a subshell could break the invariant silently.
- **Proposed Fix (pending intent):** Verify whether the PSI invariant is inherited by the T-3 call site or whether an explicit `set +e` guard is needed; align with sibling S-8.02 pattern.
- **Disposition:** SKIP-FIX-eligible per S-7.03 (pending intent on whether S-8.03 T-3 is in scope for the `set +e` explicit guard).

#### F-S803-P6-002 — AC-007 regex unwrap path (transitively covered)

- **Severity:** LOW
- **Location:** S-8.03 AC-007 — regex matching acceptance criterion
- **Description:** AC-007 specifies a regex pattern for matching the agent-stop event payload. The pattern uses a multi-field match that does not explicitly handle the case where one of the Optional fields is absent (i.e., `None` serialized as JSON null or absent key). The story's typed-projection chain (BC-2.02.012 PC-5) correctly handles the `Option<String>` unwrap, but AC-007's regex-based test criterion does not account for the `null` field value path. This is transitively covered by the typed-projection fallback chain producing `"unknown"`, but the AC does not make this coverage explicit for the test author.
- **Proposed Fix (pending intent):** Add a note to AC-007 clarifying that the regex must handle the `null` / absent field case via the typed-projection chain result ("unknown" sentinel), not raw JSON null pattern.
- **Disposition:** SKIP-FIX-eligible per S-7.03 (transitively covered; cosmetic coverage disclosure gap).

### NIT

#### F-S803-P6-003 — T-5 case count "8 cases" should be "9"

- **Severity:** NIT
- **Location:** S-8.03 T-5 bats parity tests task
- **Description:** T-5 cites "(8 cases per AC-007)" but AC-007 enumerates agent-stop scenarios where the agent-type and last-message fields produce distinct test vectors. The typed-projection chain introduces 2 sub-cases for the agent-type fallback path (agent_type present vs subagent_name fallback), making the effective case count 9 rather than 8. The "8 cases" annotation may cause the test author to write only 8 bats tests and miss the fallback sub-case.
- **Proposed Fix:** Change "(8 cases per AC-007)" to "(9 cases including agent_type/subagent_name fallback sub-cases per BC-2.02.012 PC-5)".
- **Disposition:** SKIP-FIX-eligible per S-7.03 (cosmetic precision; the fallback sub-cases are specified in the typed-projection section).

## Open Questions

None beyond pending-intent items noted above.

## Pass-7 Priors

1. Re-verify F-S803-P6-001 carryover: `set +e` PSI invariant status.
2. Re-verify F-S803-P6-002 carryover: AC-007 regex null-field coverage disclosure.
3. Anti-fabrication HARD GATE on BC-7.03.081/082 verbatim.
4. process-gap-D-185-A method-resolution check on T-3 binding sites.

## Verdict

**NITPICK_ONLY** — clock 1/3 → **2/3**.

Two LOW (T-3 `set +e` PSI invariant unenforced pending intent + AC-007 regex unwrap null path transitively covered, both SKIP-FIX-eligible) + 1 NIT (T-5 case count "8 cases" vs 9, SKIP-FIX-eligible). All 4 pass-5 carryovers maintained unchanged and verified. No content defects.

## Trajectory

| Pass | Findings | Notes |
|------|----------|-------|
| p1 | 13 | Baseline |
| p2 | 9 | Decay |
| p3 | 3 | NITPICK_ONLY (clock 0/3→1/3) |
| p4 | 1 | NITPICK_ONLY (clock 1/3→2/3) |
| p5 | 0 | CONVERGENCE_REACHED (v1.2, original cycle) |
| D-183 Phase A reset | — | Status reset to draft for typed-projection re-convergence |
| p4 (D-183) | 4 | NITPICK_ONLY (clock 0/3; adv-s8.03-p4-d183.md) |
| Phase F reset | — | v1.4→v1.5 dep wiring; clock resets to 0/3 |
| p5 (Phase G) | 4 | NITPICK_ONLY (clock 0/3→1/3) |
| p6 (Phase H) | 3 | NITPICK_ONLY (clock 1/3→2/3) |

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 6 (Phase H post-Phase F reset) |
| **New findings** | 3 (F-S803-P6-001 set+e PSI; F-S803-P6-002 AC-007 regex null; F-S803-P6-003 T-5 case count) |
| **Duplicate/variant findings** | 4 (F-S803-P5-001/002/003/004 all maintained as carryovers) |
| **Novelty score** | 3/7 = 0.43 |
| **Median severity** | LOW |
| **Trajectory** | 13→9→3→[v1.2 CONVERGED]→[D-183 reset]→4→[Phase F reset]→4→3 |
| **Verdict** | FINDINGS_REMAIN (clock 2/3; 1 more NITPICK_ONLY pass needed for ADR-013 convergence) |

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 0 |
| MEDIUM | 0 |
| LOW | 2 |
| NIT | 1 |

**Overall Assessment:** ADVANCE — Phase F dependency wiring structurally correct. Typed-projection chains confirmed across all audit layers. All pass-5 carryovers verified unchanged. Remaining findings are cosmetic or pending-intent verification; none affect spec correctness.

**Convergence:** Clock 1/3 → **2/3** (NITPICK_ONLY pass-6). One more NITPICK_ONLY pass required for ADR-013 convergence.

**Readiness:** Pass-7 dispatch (final convergence pass expected).
