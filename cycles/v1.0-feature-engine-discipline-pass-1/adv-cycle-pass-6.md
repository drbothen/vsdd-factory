---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-05-10T00:00:00Z
phase: F5
inputs:
  - .factory/specs/behavioral-contracts/ss-07/BC-7.03.091.md
  - .factory/specs/behavioral-contracts/ss-07/BC-7.03.092.md
  - .factory/specs/verification-properties/VP-076.md
  - .factory/cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md
  - .factory/stories/S-12.08
  - .factory/stories/S-14.01
  - .factory/stories/S-14.02
  - .factory/stories/S-14.03
  - .factory/stories/S-14.04
  - .factory/stories/S-14.05
  - .github/workflows/ci.yml
  - crates/vsdd-context-resolvers/src/resolver_loader.rs
  - plugins/vsdd-factory/tests/integration/E-8-hook-plugins/warn-pending-wave-gate.bats
  - plugins/vsdd-factory/tests/resolver-integration.bats
  - plugins/vsdd-factory/tests/perf-baseline.bats
input-hash: "[pending-recompute]"
traces_to: prd.md
project: vsdd-factory
mode: feature
current_step: F5-adversarial-pass-6
current_cycle: v1.0-feature-engine-discipline-pass-1
pass: 6
previous_review: adv-cycle-pass-5.md
prior-pass-classification: CRITICAL
prior-findings-count: 8
verdict: CRITICAL
findings_count: { critical: 2, high: 3, medium: 2, low: 0, nitpick: 0 }
observations: 6
deferred: 0
process_gap_count: 4
convergence_reached: false
---

# Adversarial Review — F5 Pass-6 (Cycle-Level, Fresh Context)

Fresh-context audit of the F5 fix burst applied after pass-5 CRITICAL verdict. F-P5-001 (CI WASM staging gap), F-P5-002 (BC-7.03.091/092 TBD fields), F-P5-003 (9 accumulated MEDIUMs triage), F-P5-004 (S-12.08 input-hash), F-P5-005 (forensic markers), F-P5-006 (VP-076 contradiction), F-P5-007 (timeout calibration), and F-P5-008 (CI-green-signal codification) were reported as remediated or closed. This pass verifies those closure claims and conducts a clean sweep across all affected files.

## Finding ID Convention

Finding IDs in this cycle use the shorthand format `F-P6-NNN` (cycle-scoped pass-6 findings). Formal ADV IDs follow: `ADV-<CYCLE>-P<PASS>-<SEV>-<SEQ>` per the factory standard. The cycle prefix is omitted per legacy convention established in passes 1–5.

**Note — namespace collision (see F-P6-013 observation):** per-story fix bursts during S-12.08 implementation introduced forensic markers also using `F-P5-NNN` notation, creating a collision between cycle-level pass IDs (this document) and per-story pass IDs. This ambiguity is tracked as a process-gap observation.

## Part A — Fix Verification (Pass-5 Closure Summary)

| Pass-5 ID | Status | Evidence |
|---|---|---|
| F-P5-001 (CRIT) — CI WASM staging gap | **CLOSED-WITH-ORDERING-CONCERN** | WASM staging step added to ci.yml at lines 202–222. However, the step is positioned AFTER `perf-baseline.bats` executes. `perf-baseline.bats` reads `BUNDLE_DIR` which resolves to the hook-plugins/ directory. At perf-baseline.bats:200 the staging step has not yet run — `BUNDLE_DIR` is empty. AC-1 assertion fails. CI fails. Same defect class as F-P5-001 itself. See F-P6-001. |
| F-P5-002 (HIGH) — BC-7.03.091/092 TBD fields | **CLOSED-WITH-CAVEAT** | BC-7.03.091 and BC-7.03.092 amended: SEQUENCE schema fixture added; hook path updated from `.sh` to `.wasm`. However, the VP-TBD entries at BC-7.03.091:77 and BC-7.03.092:79-80 were not included in the amendment scope. VP-TBD entries persist in the Verification Properties column. See F-P6-005. |
| F-P5-003 (HIGH) — 9 accumulated MEDIUMs untriaged | **PARTIALLY-CLOSED** | D-378 in the decision-log records explicit dispositions for all 9 accumulated MEDIUMs. 6 of 9 are CLOSED or WONT-FIX with documented justification. 3 of 9 are recorded as "FOLLOW-UP STORY — file as S-14.xx under E-14." Those 3 follow-up stories do not exist in `.factory/stories/` — only S-14.01 through S-14.05 are present. The D-378 closure overclaims. See F-P6-002. |
| F-P5-004 (HIGH) — S-12.08 input-hash pending | **CLOSED** | S-12.08 frontmatter `input-hash` now reads `4a88bec`. Verified. No other S-12.xx stories carry `[pending-recompute]`. Finding closed. |
| F-P5-005 (MED) — forensic markers proliferating | **NOT CLOSED** | No follow-up story filed for forensic-marker cleanup. Additionally, the S-12.08 per-story implementation passes added 24+ new `F-P5-NNN` markers across 5 production files. Pass-5 advisory ("STOP adding F-P5-NNN markers in production code") was not observed. Marker count is growing. See F-P6-004. |
| F-P5-006 (MED) — VP-076:163 Expected Outcome contradiction | **CLOSED** | VP-076.md:163 bullet now reads "DEFERRED — see §Architectural Limitation." Cross-reference to deferred story added. Internal contradiction resolved. Finding closed. |
| F-P5-007 (MED) — timeout calibration gap 1100ms | **CLOSED-WITH-CAVEAT** | Lower bound raised to 1300ms (not 1400ms). Reduces regression corridor from 400ms to 200ms but does not fully close it. Per-pass record amended with rationale. Wording note: comment at timeout assertion reads "catches ≥15% reductions" but 1300/1500 = 13.3%. Arithmetic inconsistency. See F-P6-008 (observation). |
| F-P5-008 (LOW — process-gap) — no CI-green-signal codification | **NOT CODIFIED** | No decision-log entry (D-NNN) was created codifying the CI-green-signal rule for CRITICAL CI-class finding closures. The pass-5 review document states the rule textually but no D-NNN entry was authored in the cycle decision-log. The rule is operationally inert — it exists in a past adversary report that is not referenced at fix-burst time. See F-P6-003. |

## Part B — New Findings

### CRITICAL

#### F-P6-001 [CRITICAL]: ci.yml WASM staging step mis-ordered — runs after perf-baseline.bats; BUNDLE_DIR empty at assertion time; CI fails

- **Severity:** CRITICAL
- **Category:** process-gap / verification-gaps
- **Location:** `.github/workflows/ci.yml:202–222`; `plugins/vsdd-factory/tests/perf-baseline.bats:200`
- **Description:** The F-P5-001 fix added a WASM staging step to ci.yml. However, the step is positioned after the `perf-baseline.bats` job step. `perf-baseline.bats` reads `BUNDLE_DIR` (the hook-plugins/ directory) at line 200 to perform its AC-1 assertion. At that point in the job graph, the staging step has not yet executed — `BUNDLE_DIR` is empty. The AC-1 assertion therefore fails on CI with a missing-artifact error, which is the same defect class F-P5-001 was intended to fix. The fix added a staging step but in the wrong position relative to the bats consumer. A single CI dry-run on a PR branch would have revealed this ordering error in under 2 minutes.
- **Evidence:** `ci.yml:202–222` — WASM staging step present but positioned after perf-baseline.bats step. `perf-baseline.bats:200` — AC-1 reads `BUNDLE_DIR`. `BUNDLE_DIR` points to `hook-plugins/` which is gitignored and not present before the staging step runs.
- **Proposed Fix:** Move the WASM staging step (ci.yml:202–222) to execute BEFORE the perf-baseline.bats step and all other bats steps that consume `BUNDLE_DIR`. **MANDATORY: Run CI green on the PR branch before declaring this finding closed (per F-P6-003 enforcement). No self-certification.** This is the third consecutive generation of the same defect class (F-P4-002 → F-P5-001 → F-P6-001).

#### F-P6-002 [CRITICAL]: D-378 closure overclaimed — 3 of 9 MEDIUMs declared "FOLLOW-UP STORY — file as S-14.xx" but S-14.06/07/08 do not exist

- **Severity:** CRITICAL
- **Category:** process-gap / audit-integrity
- **Location:** `.factory/cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md` (D-378); `.factory/stories/` (S-14.xx)
- **Description:** D-378 records dispositions for all 9 accumulated MEDIUM findings addressed by F-P5-003. Three of those dispositions read "FOLLOW-UP STORY — file as S-14.xx under E-14." The decision-log entry implies these stories exist or will be created atomically with the disposition record. `.factory/stories/` contains only S-14.01 through S-14.05 — no S-14.06, S-14.07, or S-14.08. The D-378 closure was committed without creating the referenced stories. This means three accumulated MEDIUM findings have a disposition of "deferred to story" with no corresponding story artifact — the deferred work is untracked and invisible to the sprint-state and story index. F-P5-003 PARTIALLY-CLOSED status above reflects this gap; the pass-5 closure claim was inaccurate.
- **Evidence:** D-378 in decision-log references S-14.06, S-14.07, S-14.08. `ls .factory/stories/` shows S-14.01 through S-14.05 only. STORY-INDEX.md does not contain S-14.06/07/08 entries.
- **Proposed Fix:** Either (a) author S-14.06, S-14.07, and S-14.08 story files with acceptance criteria derived from the three deferred MEDIUMs and register them in STORY-INDEX.md, OR (b) amend D-378 to record concrete S-14.xx IDs for stories that already exist and cover this scope if applicable. Option (a) is the correct resolution. Stories must exist before closure is claimed.

### HIGH

#### F-P6-003 [HIGH]: F-P5-008 CI-green-signal rule not codified in decision-log — operationally inert; no D-NNN entry authored

- **Severity:** HIGH
- **Category:** process-gap
- **Location:** `.factory/cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md`; fix-burst protocol
- **Description:** Pass-5 (F-P5-008) identified the structural process gap: no rule requires CI green signal before CRITICAL CI-class finding closure. The proposed fix was to "codify the following rule in the fix-burst protocol and cycle decision-log." The pass-5 fix burst did not author a decision-log entry (D-NNN) recording this rule. The rule appears in the pass-5 adversary report (a historical artifact read rarely) but not in the decision-log (a living document referenced at fix-burst time). Because the rule was never codified where it is consulted, it was immediately violated: the F-P5-001 fix burst closed F-P5-001 without running CI, introducing F-P6-001 (same defect class). The recursive failure is direct evidence that operational inertness of the un-codified rule was the cause.
- **Evidence:** Decision-log has no D-NNN entry for CI-green-signal rule. F-P5-008 proposed fix states "Add to STATE.md Decisions Log" — this was not done. F-P6-001 demonstrates the rule was violated immediately after pass-5.
- **Proposed Fix:** Author D-379 in the cycle decision-log codifying: "CRITICAL findings in the CI-class (coverage, staging, CI step invocation, bats runner configuration) MUST demonstrate CI green on a PR branch before being recorded as CLOSED. Self-certification is not accepted for this class. Adversary review documents alone are insufficient evidence of closure." This entry must exist before the next fix burst addressing F-P6-001.

#### F-P6-004 [HIGH]: Forensic markers proliferating — 24+ F-P5-NNN markers added to production files during S-12.08 per-story passes; no cleanup story filed

- **Severity:** HIGH
- **Category:** code-quality
- **Location:** `crates/vsdd-context-resolvers/src/resolver_loader.rs`; `plugins/vsdd-factory/tests/resolver-integration.bats`; `plugins/vsdd-factory/tests/perf-baseline.bats`; `plugins/vsdd-factory/tests/resolver-capability-confinement.bats`; `plugins/vsdd-factory/tests/integration/E-8-hook-plugins/warn-pending-wave-gate.bats`
- **Description:** F-P5-005 explicitly warned: "STOP adding F-P5-NNN markers directly to production code." The S-12.08 per-story implementation passes ignored this advisory and added 24+ new `F-P5-NNN` forensic markers across 5 production and test files. The original 321+ baseline count from F-P3-004 is now materially higher. No follow-up story for forensic-marker cleanup was filed (the pass-5 recommendation to file one was not executed). The markers are growing without any bound or cleanup mechanism.
- **Evidence:** 24+ `F-P5-` prefixed markers added in S-12.08 per-story passes across the 5 listed files. No story file exists for forensic-marker cleanup in `.factory/stories/`. F-P3-004 baseline: 321+ markers. Current count is higher.
- **Proposed Fix:** (1) File a follow-up story (e.g., S-14.09 "Forensic marker cleanup") with ACs: remove all pass-specific `F-P[N]-NNN` markers from production and test files; convert legitimate TODOs to `TD-NNN` references; add CI lint rule blocking future pass-specific marker introduction. (2) Codify in fix-burst protocol: forensic markers belong in adversary reports and decision-log entries — NOT in production or test source files.

#### F-P6-005 [HIGH]: BC-7.03.091:77 and BC-7.03.092:79-80 still contain VP-TBD entries — F-P5-002 amendment scope did not include VP table

- **Severity:** HIGH
- **Category:** spec-fidelity
- **Location:** `.factory/specs/behavioral-contracts/ss-07/BC-7.03.091.md:77`; `.factory/specs/behavioral-contracts/ss-07/BC-7.03.092.md:79-80`
- **Description:** F-P5-002 fix burst amended BC-7.03.091 and BC-7.03.092 to add the SEQUENCE schema fixture and update the hook artifact path from `.sh` to `.wasm`. The amendment scope explicitly excluded the Verification Properties column. BC-7.03.091:77 and BC-7.03.092:79-80 still carry `VP-TBD` in that column. F-P5-002 was recorded as CLOSED-WITH-CAVEAT above (the caveat being this gap). `VP-TBD` entries in production BCs mean there is no formal link from the behavioral contracts to the verification properties layer — drift detection, traceability matrices, and phase-gate checks cannot validate these contracts against verification coverage. The VP-TBD entries represent unresolved spec-layer debt that predates this cycle.
- **Evidence:** BC-7.03.091:77 — `VP-TBD`. BC-7.03.092:79-80 — `VP-TBD`. F-P5-002 amendment commit did not touch these lines. VP table not in amendment diff.
- **Proposed Fix:** Assign concrete VP IDs to BC-7.03.091:77 and BC-7.03.092:79-80. If the relevant VP does not yet exist, create it (or extend VP-076 with a sub-property covering this BC scope) and register it in VP-INDEX.md. If the VP exists under a different ID, cross-reference it. Do not leave `VP-TBD` in shipped BCs.

### MEDIUM

#### F-P6-006 [MEDIUM]: BC-7.03.091:94 and BC-7.03.092:97 reference `architecture/ss-07-hook-bash.md` (lowercase) — actual file is `SS-07-hook-bash.md`; broken on case-sensitive filesystems

- **Severity:** MEDIUM
- **Category:** spec-fidelity / broken-reference
- **Location:** `.factory/specs/behavioral-contracts/ss-07/BC-7.03.091.md:94`; `.factory/specs/behavioral-contracts/ss-07/BC-7.03.092.md:97`
- **Description:** Both amended BCs reference the architecture subsystem document as `architecture/ss-07-hook-bash.md` (lowercase `ss-07`). The actual filename is `SS-07-hook-bash.md` (uppercase `SS-07`). On macOS (case-insensitive HFS+) this resolves silently. On Linux (case-sensitive ext4, used by CI) this is a broken anchor — tools that resolve architecture cross-references will fail to locate the file. The F-P5-002 amendment introduced this reference during the SEQUENCE schema fixture addition without validating the path casing.
- **Evidence:** BC-7.03.091:94 and BC-7.03.092:97 contain `architecture/ss-07-hook-bash.md`. `.factory/specs/architecture/` contains `SS-07-hook-bash.md` (uppercase). Linux CI would report file-not-found for this reference.
- **Proposed Fix:** Update both BC references to use the correct case: `architecture/SS-07-hook-bash.md`. Verify the corrected path exists in `.factory/specs/architecture/` before committing.

#### F-P6-007 [MEDIUM]: F-P5-008 CI-green precondition violated by the pass-5 fix burst itself — recursive process-discipline failure

- **Severity:** MEDIUM
- **Category:** process-gap
- **Location:** Pass-5 fix burst commit; F-P6-001 (evidence of violation)
- **Description:** Pass-5 authored F-P5-008, which states that CRITICAL CI-class findings MUST demonstrate a CI green signal before closure. The pass-5 fix burst then closed F-P5-001 without running CI on a PR branch — the very precondition F-P5-008 defined. F-P6-001 is the direct consequence: the staging step was added but mis-ordered, and CI was never run to catch it. The recursion is exact: the pass wrote the rule, then violated the rule in the same burst, producing the same defect class in the next generation. This is not an isolated oversight — it is structural evidence that process rules authored in adversary reports and not codified in decision-log entries (F-P6-003) are not consulted at fix-burst time.
- **Evidence:** F-P5-008 proposed fix: "Add to STATE.md Decisions Log as an explicit governance decision." Not done. F-P6-001: staging step mis-ordered. No CI run referenced in F-P5-001 closure record.
- **Proposed Fix:** This finding is addressed by F-P6-003 (codify D-379) and F-P6-001 (fix staging ordering + run CI green). No additional fix beyond those two is required. Record this finding as a pattern-level note: process rules that are not codified in the decision-log will not survive the next fix burst.

## Observations

- **F-P6-008 [observation — wording]:** Timeout assertion comment reads "catches ≥15% reductions" but the bound is 1300ms against 1500ms deadline (1300/1500 = 13.3%). The arithmetic is inconsistent with the comment. No behavioral regression — the bound is still an improvement over 1100ms. Fix the comment to read "catches ≥13% reductions" or raise the bound to 1350ms and update to "catches ≥10% reductions" for round-number accuracy. Minor — cosmetic.

- **F-P6-009 [observation — positive]:** The WASM staging step added for F-P5-001 uses the correct POL-11 form (build debug, copy to hook-plugins/, gate downstream steps on success). The step content is structurally correct; only its position in the job graph is wrong (F-P6-001). Once re-ordered, the step is ready to use without further modification.

- **F-P6-010 [observation — positive]:** VP-076 Expected Outcome fix (F-P5-006 closure) is clean. The DEFERRED marker and cross-reference to §Architectural Limitation are clear and unambiguous. No further action required on this finding.

- **F-P6-011 [observation — positive]:** S-12.08 input-hash recompute (F-P5-004 closure) is clean. Hash value `4a88bec` is present and consistent across the story frontmatter. No other S-12.xx stories carry stale hashes. Drift detection will no longer emit false positives for this story.

- **F-P6-012 [observation — positive]:** CHANGELOG monotonicity is preserved across the pass-5 and pass-6 fix burst windows. No out-of-order entries detected. The changelog discipline introduced in prior passes is holding.

- **F-P6-013 [observation — process-gap]:** Namespace collision between cycle-level finding IDs and per-story finding IDs. Cycle-level pass-5 findings use `F-P5-NNN` (this document's convention). Per-story fix bursts during S-12.08 implementation also used `F-P5-NNN` for their own pass-scoped markers. A reader of `resolver_loader.rs:509` seeing `TODO(F-P5-002)` cannot determine whether this references cycle-level F-P5-002 (BC TBD fields) or a per-story pass-5 finding. Recommend establishing a namespace separation: cycle-level findings use `CYC-P[N]-NNN`, per-story findings use `STR-[STORY_ID]-P[N]-NNN`, or adopt a different separator that is unambiguous. Codify in fix-burst protocol.

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 2 |
| HIGH | 3 |
| MEDIUM | 2 |
| LOW | 0 |
| Process-gap | 4 |

**Overall Assessment:** block
**Convergence:** findings remain — iterate
**Readiness:** requires revision

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 6 |
| **New findings** | 7 |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 1.0 (7/7) |
| **Median severity** | 2.3 (between HIGH and MEDIUM) |
| **Trajectory** | 29→11→9→9→8→**7** |
| **Verdict** | FINDINGS_REMAIN |

## Convergence Trajectory

| Pass | Classification | Critical | High | Medium | Low | Total |
|------|---------------|----------|------|--------|-----|-------|
| P1 | CRITICAL | 2 | 10 | 12 | 5 | 29 |
| P2 | CRITICAL | 2 | 4 | 5 | 0 | 11 |
| P3 | CRITICAL | 2 | 4 | 3 | 0 | 9 |
| P4 | CRITICAL | 2 | 4 | 3 | 0 | 9 |
| P5 | CRITICAL | 1 | 3 | 3 | 1 | 8 |
| P6 | CRITICAL | 2 | 3 | 2 | 0 | **7** |

Severity floor stable at CRITICAL across all 6 passes. Pattern: fix bursts close individual findings while introducing same-class defects in adjacent artifacts. Critical count increased P5→P6 (1→2) due to CI staging mis-ordering and D-378 closure overclaim. Severity floor is not declining.

## Top 5 Most Important Findings (F5 pass-6+ fix burst drivers)

1. **CRITICAL F-P6-001** — Move WASM staging step before perf-baseline.bats; MUST run CI green on PR branch before closure; no self-certification (CI)
2. **CRITICAL F-P6-002** — Author S-14.06/07/08 stories OR amend D-378 with concrete IDs; closure cannot be claimed without story artifacts (PROCESS)
3. **HIGH F-P6-003** — Author D-379 codifying CI-green-signal rule in decision-log before next fix burst (PROCESS)
4. **HIGH F-P6-004** — File forensic-marker cleanup story; stop adding pass-specific markers to production files (CODE)
5. **HIGH F-P6-005** — Assign concrete VP IDs to BC-7.03.091:77 and BC-7.03.092:79-80; VP-TBD not acceptable in shipped BCs (SPEC)

## Recommendation

**Cycle CANNOT proceed to F6 hardening.** Continue F5 fix bursts. This is the 6th consecutive cycle-level CRITICAL verdict.

Pattern confirmed: fix bursts keep introducing same-class defects. The root cause is structural — process rules are authored in adversary reports but not codified in the decision-log (F-P6-003), so they are not consulted at fix-burst time. F-P6-007 is the direct evidence: the rule was written and violated in the same burst.

Required before next adversary pass:
1. **REQUIRED:** Move staging step BEFORE perf-baseline.bats (F-P6-001)
2. **REQUIRED:** Run CI green on PR branch BEFORE declaring F-P6-001 closed — mandatory; F-P6-007 enforcement. This is the third generation of this defect class. No exceptions.
3. **REQUIRED:** Author S-14.06/07/08 stories OR amend D-378 with concrete existing story IDs (F-P6-002)
4. **REQUIRED:** Author D-379 codifying CI-green-signal rule in decision-log (F-P6-003)
5. **REQUIRED:** Fix BC-7.03.091:94 and BC-7.03.092:97 anchor case — `ss-07` → `SS-07` (F-P6-006)

Strongly recommended before F6:
6. **STRONGLY RECOMMENDED:** Assign actual VP IDs to BC-7.03.091:77 and BC-7.03.092:79-80; retire VP-TBD entries (F-P6-005)
7. **STRONGLY RECOMMENDED:** File forensic-marker cleanup story; codify namespace separation rule (F-P6-004, F-P6-013)

`convergence_reached`: false. Verdict CRITICAL. Need 3 consecutive NITPICK_ONLY cycle-level passes for cycle convergence.

## Process-Gap Findings (4)

F-P6-001 (CI staging mis-ordered — same class as F-P5-001/F-P4-002; third consecutive generation), F-P6-002 (D-378 closure overclaimed — follow-up stories not filed), F-P6-003 (F-P5-008 CI-green-signal rule not codified in decision-log; operationally inert), F-P6-007 (recursive process-discipline failure — rule authored and violated in same burst).
