---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: vsdd-factory:adversary
timestamp: 2026-08-20T00:00:00Z
phase: 3
inputs:
  - .factory/stories/S-21.11-validator-exhaustion-fail-closed-calibration-and-enforcement.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.03.017.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.03.018.md
  - .factory/specs/architecture/decisions/ADR-039-validator-failure-policy-resource-exhaustion-fail-closed.md
input-hash: "8a3f0c9"
traces_to: S-21.11-validator-exhaustion-fail-closed-calibration-and-enforcement.md
pass: 11
cascade: S-21.11-v2
previous_review: adv-s21.11-v2-local-pass-10.md
---

# Adversarial Review — S-21.11 v2 (LOCAL cascade, pass 11)

> **Persistence note (state-manager, this burst):** content below is transcribed verbatim from the
> orchestrator-relayed adversary dispatch per POLICY 22 (orchestrator-transcribed; the live
> adversary session ran in a separate context this burst resumes from), following the
> `adv-s21.11-v2-local-pass-N.md` convention established at pass-3.

Artifacts reviewed: story `S-21.11-validator-exhaustion-fail-closed-calibration-and-enforcement.md`
v2.9 (input-hash `97029a5`); `BC-1.03.017.md` v1.18; `BC-1.03.018.md` v1.1; `ADR-039` v1.13
(§Erratum E-005, `status: ratified`); factory-artifacts bundle HEAD `2ff524ab` (D-1050 commit) —
the SAME bundle reviewed at pass-10, unchanged per the BC-5.39.001 3-CLEAN protocol. Rubric: full
`.factory/policies.yaml` (POLICY 1-22).

## Verdict: NOT-CLEAN

3 MEDIUM streak-resetting findings (F-S2111V2-P11-001/002/003), plus independent re-confirmation
of pass-10's 3 non-resetting LOW/ADVISORY cosmetic observations (F-S2111V2-P10-001/002/003, still
present and unremediated — pass-10 deliberately deferred them). BC-5.39.001 streak **RESETS 1/3 →
0/3**.

## Finding ID Convention

Finding IDs for the S-21.11 v2 cascade use the format `F-S2111V2-P<PASS>-<SEQ>`, established at
pass-1 (F-S2111V2-P1-001) and continued through pass-10 (F-S2111V2-P10-001..003). This pass's 3
MEDIUM streak-resetting findings use `F-S2111V2-P11-001..003`.

## Part A — Fix Verification (pass >= 2 only)

No BLOCKER/HIGH/MEDIUM findings were open from pass-10 to verify — pass-10 returned CLEAN. The 3
LOW/ADVISORY cosmetic observations pass-10 recorded (F-S2111V2-P10-001/002/003) were explicitly
DOCUMENTED-not-remediated at pass-10 (deferred to convergence-close per that pass's own
disposition); this pass independently re-confirms all 3 are still present, unchanged, and correctly
classified as LOW/ADVISORY non-resetting cosmetic nits (no escalation in severity found).

## Part B — New Findings

### F-S2111V2-P11-001 (MEDIUM, streak-resetting)

**Location:** Story §"BC authoring routing — RESOLVED" (Routing Proposals section), opening
sentence.

**Finding:** The sentence reads `BC-1.03.017 (\`factory-dispatcher::executor::failure_policy
enforcement\`, v1.17)` — a stale version cite. This directly self-contradicts the SAME paragraph's
own later, correct reference to "BC-1.03.017 v1.18's new Invariant 11" two lines later — an
intra-paragraph version contradiction. The staleness survived every prior sweep in this cascade
(including the v2.9/D-1049 60-cite propagation sweep) because a backtick-quoted title
(`` `factory-dispatcher::executor::failure_policy enforcement` ``) intervenes between the BC ID and
its version token — a form the story's standard contiguous-normalized detector
(`BC-1\.03\.017 +v1\.[0-9]+`, whitespace-tolerant but not backtick-tolerant) structurally cannot
match, exactly analogous in kind (though not in mechanism) to the pass-7 line-wrap blind spot
D-1047 codified.

**Impact:** A reader following the Routing Proposals section's own opening sentence is told
BC-1.03.017 is at v1.17, contradicted two lines later by the same section citing v1.18 — an
internal document self-contradiction, not merely a stale-vs-current mismatch against an external
source.

### F-S2111V2-P11-002 (MEDIUM, streak-resetting)

**Location:** Story Task #16 (pointer note) and Task #29 (test authoring), governing AC-009.

**Finding:** AC-009's genuine red-first TDD gate test
(`test_all_six_validator_class_plugins_are_fail_closed`) is (a) mis-pointed by Task #16's note as
"(Task #21)" — Task #21 is the unrelated Node-E BC-1.03.018-reading task, not where the test is
authored — and (b) actually authored at Task #29, which is sequenced in Node (E)/Phase 4c, AFTER
the Phase-4c fail-closed-flip Tasks #26–#28 that create the very registry annotations the test
asserts. Authoring the test after the annotations land makes a genuine first-run RED observation
structurally impossible: the test would already be GREEN the first time it runs, because the
registry state it asserts against has already been flipped. This violates AC-009's own postcondition
that the RED observation occur BEFORE Phase 4 annotations land, and violates `tdd_mode: strict`'s
Red Gate density requirement (a Red Gate test authored where it cannot observe RED is not a Red
Gate test in the methodology's sense, regardless of its assertion content being correct).

**Impact:** A structural TDD-discipline violation for one of the story's two central red-first
gates (AC-009), not a wording or cite defect — an implementer following the story's task order as
written would never observe a genuine RED run for this test.

### F-S2111V2-P11-003 (MEDIUM, streak-resetting)

**Location:** Story Task #18 (authoring) and Task #20 (verification enumeration), governing
AC-010.

**Finding:** AC-010 (epoch exhaustion + `FailurePolicy::FailClosed` → block;
`test_epoch_exhaustion_fail_closed_blocks`; traces to BC PC6 / ADR-039 §Decision 1 epoch-parity) has
no explicit authoring task anywhere in the story, and is absent from Task #20's RED-gate
verification enumeration — unlike its fuel-axis sibling ACs, which ARE explicitly covered
(AC-002/AC-003 at Task #17; AC-004/AC-005 at Task #18). This is a coverage gap: the epoch-axis
counterpart to the fuel-axis exhaustion tests has no home in the Tasks DAG at all.

**Impact:** Absent an explicit authoring task, AC-010's coverage depends entirely on an
implementer independently noticing the gap; Task #20's enumeration ("Verify RED Gates for AC-002
through AC-005 pass") does not even name AC-010 as something to check, so a mechanical
task-completion audit would not catch the omission either.

## Re-confirmed non-resetting observations (carried forward from pass-10, unremediated)

- **F-S2111V2-P10-001 (LOW):** story DAG node-(D) box caption enumerates only "AC-002..AC-006,
  AC-010, AC-011," omitting Node (D)'s authoritative ownership of Task #19b (AC-013b) and Task #19c
  (AC-024..AC-041). Caption-only staleness; Tasks section itself remains complete/correct.
- **F-S2111V2-P10-002 (LOW):** DAG/AC-011 reference label "Executor Extension" is non-verbatim
  against the Tasks section's actual heading "Node (D) — Executor decision-function extension."
  Resolves unambiguously; readability nit only.
- **F-S2111V2-P10-003 (ADVISORY):** break-glass ACs (AC-015/AC-016/AC-019) cite EC-004/EC-005/
  EC-006 using BC-1.03.018's own internal EC numbering, conflating with the story's own
  same-numbered EC rows. The existing "BC-1.03.018 PCn"-style prefix at each site disambiguates
  unambiguously.

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 0 |
| MEDIUM | 3 |
| LOW | 2 |
| ADVISORY | 1 |

**Overall Assessment:** not-clean, streak-resetting
**Convergence:** streak RESETS **1/3 → 0/3**. A fresh CLEAN pass-12 restarts the count from 0/3 per
BC-5.39.001.
**Readiness:** route F-S2111V2-P11-001/002/003 to story-writer for same-burst remediation
(story-only fix — no BC/ADR change required for any of the three); fold in the 3 pass-10 cosmetic
observations in the same burst to close the deferred convergence-close sweep early, since the
bundle is being touched (re-versioned) regardless this burst.

## Novelty Assessment

Novelty **MEDIUM** — two of the three MEDIUM findings are genuinely new defect classes for this
cascade: F-S2111V2-P11-002 (a structural red-first-gate ordering defect — the test author placed a
RED-gate test AFTER the code changes it is meant to catch, a TDD-discipline class not previously
seen in this cascade's 10 prior passes) and F-S2111V2-P11-003 (a coverage-enumeration gap — an AC
entirely missing its authoring task, distinct from every prior finding in this cascade, which have
all been version-cite or numeric-magnitude defects in EXISTING task text, not missing tasks).
F-S2111V2-P11-001 is a recurrence of the established D-1006 version-cite-propagates family, with a
new sub-mechanism (backtick-title-intervening, as opposed to pass-7's line-wrap-intervening) that
the existing whitespace-normalized detector still cannot see — this pass recommends the detector
itself be upgraded to a backtick-tolerant form
(`BC-N\.NN\.NNN[^v]{0,120}v1\.[0-9]+`) as a durable fix, per the same "fix the tooling, not just
the instance" discipline D-1047(h) established for the line-wrap class.
