---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-05-10T00:00:00
phase: F5
inputs:
  - .factory/specs/behavioral-contracts/ss-04/BC-4.10.001.md
  - .factory/specs/behavioral-contracts/ss-07/BC-7.03.091.md
  - .factory/specs/behavioral-contracts/ss-07/BC-7.03.092.md
  - .factory/specs/verification-properties/VP-076.md
  - .factory/tech-debt-register.md
  - crates/hook-plugins/warn-pending-wave-gate/src/lib.rs
  - crates/vsdd-context-resolvers/src/resolver_loader.rs
  - plugins/vsdd-factory/tests/integration/E-8-hook-plugins/warn-pending-wave-gate.bats
  - plugins/vsdd-factory/tests/resolver-integration.bats
  - plugins/vsdd-factory/tests/resolver-capability-confinement.bats
  - .github/workflows/ci.yml
  - .github/workflows/release.yml
  - .gitignore
input-hash: "[pending-recompute]"
traces_to: prd.md
project: vsdd-factory
mode: feature
current_step: F5-adversarial-pass-5
current_cycle: v1.0-feature-engine-discipline-pass-1
pass: 5
previous_review: adv-cycle-pass-4.md
prior-pass-classification: CRITICAL
prior-findings-count: 9
verdict: CRITICAL
findings_count: { critical: 1, high: 3, medium: 3, low: 1, nitpick: 0 }
observations: 1
deferred: 0
process_gap_count: 4
convergence_reached: false
---

# Adversarial Review — F5 Pass-5 (Cycle-Level, Fresh Context)

Fresh-context audit of the F5 fix burst applied after pass-4 CRITICAL verdict. F-P4-001 (stale MAPPING bats fixture), F-P4-002 (bats not in ci.yml), F-P4-003 (VP-076 anchor), F-P4-004 (VP-076-C false stub claim), F-P4-005 (timeout bound), F-P4-006 (sibling WaveEntry structs), and F-P4-007 (bats docstrings) were reported as remediated or closed-with-caveat. This pass verifies those closure claims and conducts a clean sweep across all affected files.

## Finding ID Convention

Finding IDs in this cycle use the shorthand format `F-P5-NNN` (cycle-scoped pass-5 findings). Formal ADV IDs follow: `ADV-<CYCLE>-P<PASS>-<SEV>-<SEQ>` per the factory standard. The cycle prefix is omitted per legacy convention established in passes 1–4.

## Part A — Fix Verification (Pass-4 Closure Summary)

| Pass-4 ID | Status | Evidence |
|---|---|---|
| F-P4-001 (CRIT) — stale MAPPING bats fixture | **CLOSED** | `warn-pending-wave-gate.bats:100-163` rewritten to SEQUENCE form; negative MAPPING fixture added. cwd fix for bats SEQUENCE test confirmed in commit. |
| F-P4-002 (CRIT) — bats tests not in ci.yml | **REGRESSION-INTRODUCED** | ci.yml step added referencing bats suite. However, WASM artifacts are not staged before the bats step — hook-plugins/ is gitignored (.gitignore:58) and cargo builds to target/wasm32-wasip1/debug/. No copy step in ci.yml (only release.yml:249 has staging logic). All 3 new bats files will fail CI at runtime. See F-P5-001. |
| F-P4-003 (HIGH) — VP-076 anchor broken | **CLOSED** | VP-076.md:202 anchor corrected to `resolver-capability-confinement.bats`. Verified. |
| F-P4-004 (HIGH) — VP-076-C false stub claim | **PARTIAL** | VP-076-C status field updated. However, VP-076.md:163 Expected Outcome bullet still asserts VP-076-C is "verified" while §Architectural Limitation (lines 166-176) explicitly defers it. Internal document contradiction persists. See F-P5-006. |
| F-P4-005 (HIGH) — timeout lower-bound 1000ms | **CLOSED-WITH-CAVEAT** | Lower bound raised to 1100ms per fix burst commit. Catches 25%+ deadline reductions (1500ms → <1100ms) but misses 13% reductions (1500ms → ~1300ms). See F-P5-007 for calibration gap. |
| F-P4-006 (HIGH) — sibling WaveEntry structs | **CLOSED** | TD-074 registered; 3 sibling struct TODOs added referencing TD-074 for follow-up. Correct process closure. |
| F-P4-007 (MED) — bats docstrings stale | **CLOSED** | Docstrings updated to SEQUENCE form semantics. Verified. |
| F-P4-008 (MED) — BC-7.03.091/092 TBD fields | **STILL OPEN** | F-P4 skip justified as "pre-existing changelog monotonicity issue" — but this conflated BC *content* gaps with changelog format. BC-7.03.091/092 still contain TBD stub fields and no canonical SEQUENCE schema fixture. Hook artifact path in BC still references `.sh` (Bash); production is WASM. No canonical SEQUENCE schema fixture in BC layer. See F-P5-002. |
| F-P4-009 (MED) — S-12.08 input-hash pending | **STILL OPEN** | S-12.08 input-hash still `[pending-recompute]`. Not addressed in F-P4 fix burst. See F-P5-004. |

## Part B — New Findings

### CRITICAL

#### F-P5-001 [CRITICAL]: CI WASM-staging gap — bats step added but hook-plugins/ artifacts not staged; all 3 new bats files will fail on CI

- **Severity:** CRITICAL
- **Category:** process-gap / verification-gaps
- **Location:** `.github/workflows/ci.yml:202-213`; `.gitignore:58`; `crates/hook-plugins/warn-pending-wave-gate/`
- **Description:** The F-P4-002 fix added a bats invocation step to ci.yml. However, no step in ci.yml stages the WASM plugin artifacts before bats runs. `.gitignore:58` confirms `hook-plugins/` is gitignored — these artifacts are not present in the checked-out working tree. Cargo builds to `target/wasm32-wasip1/debug/` but no copy step exists in ci.yml. Only `release.yml:249` contains the WASM staging pattern (`cp target/wasm32-wasip1/release/...`). As a result, when the ci.yml bats step executes, the hook plugin binaries are absent and all 3 new bats files (`resolver-integration.bats`, `resolver-capability-confinement.bats`, `warn-pending-wave-gate.bats`) will fail at runtime with a missing-plugin error — the opposite of a green CI signal. This is the same false-green defect class F-P4-002 was intended to fix: CI step now exists but bats still cannot succeed.
- **Evidence:** `.gitignore:58` — `hook-plugins/` gitignored. `ci.yml:202-213` — bats step present, no prior `cargo build --target wasm32-wasip1` + copy step. `release.yml:240-260` — staging pattern exists there only.
- **Proposed Fix:** Add a WASM staging step to ci.yml before the bats step. Copy the pattern from `release.yml:240-260`: build with `cargo build --target wasm32-wasip1` (debug profile for CI) and copy artifacts to `hook-plugins/`. Gate the bats step to depend on this staging step. Verify CI green on a PR branch before declaring F-P5-001 closed. **MUST show CI green signal before closure — no exceptions per F-P5-008 process codification.**

### HIGH

#### F-P5-002 [HIGH]: BC-7.03.091/092 still stub-form with TBD fields — F-P4-008 skip conflated BC content with changelog format; skip unjustified

- **Severity:** HIGH
- **Category:** spec-fidelity
- **Location:** `.factory/specs/behavioral-contracts/ss-07/BC-7.03.091.md`; `BC-7.03.092.md`
- **Description:** F-P4-008 was skipped with the justification "pre-existing changelog monotonicity issue." This justification conflated the BC *content* quality problem with a changelog ordering concern — two separate issues. The BC content gap stands regardless of changelog state: BC-7.03.091 (producer contract) and BC-7.03.092 (consumer contract) both contain TBD stub fields that leave the SEQUENCE-vs-MAPPING schema ambiguity unspecified at the contract layer. Additionally, the hook artifact path within these BCs still references the `.sh` (Bash) extension — production is WASM (`.wasm`). No canonical SEQUENCE schema fixture exists in either BC. This is the structural condition that allowed the MAPPING/SEQUENCE schema disagreement (TD-073 / F-P3-001 / F-P4-001) to originate and survive 4 passes without being flagged as a contract violation. A developer reading BC-7.03.091/092 alone cannot determine the correct schema form.
- **Evidence:** BC-7.03.091/092 TBD fields confirmed present. Hook path references `.sh`. No `waves: SEQUENCE of WaveEntry` assertion in either BC. F-P4-008 skip justification references "changelog monotonicity" — a separate concern.
- **Proposed Fix:** Address the BC content gap independently of any changelog ordering fix. Add canonical SEQUENCE schema fixture to both BCs. Add PC asserting `waves MUST be YAML sequence (list) of WaveEntry; MAPPING form invalid`. Update hook artifact path from `.sh` to `.wasm`. Bump both BCs. Do not conflate with changelog format fixes.

#### F-P5-003 [HIGH]: 9 accumulated MEDIUM findings (F-P2-003/004/007/009/010/011 + F-P3-009/010/011) untouched for 3 passes — triage required

- **Severity:** HIGH
- **Category:** process-gap
- **Location:** Pass-2 and Pass-3 finding records; STATE.md convergence tracking
- **Description:** Six findings from pass-2 (F-P2-003, F-P2-004, F-P2-007, F-P2-009, F-P2-010, F-P2-011) and three from pass-3 (F-P3-009, F-P3-010, F-P3-011) have been carried as OPEN for 3 consecutive passes without explicit triage. Pass-3 noted these would be addressed in a "dedicated reconciliation pass" that never occurred. Pass-4 acknowledged them (F-P4-011) but did not triage them either. Nine MEDIUMs accumulating silently across 3 passes constitutes a governance gap: they may represent legitimate deferred work, accepted risk, or won't-fix decisions — but without explicit triage, none of these dispositions is recorded and the findings cannot be considered closed.
- **Evidence:** F-P4-011 observation confirmed the accumulation. adv-cycle-pass-3.md and adv-cycle-pass-4.md both carry these findings as OPEN without disposition.
- **Proposed Fix:** Run a triage pass on all 9 accumulated MEDIUMs. For each: (a) CLOSE with evidence if addressed, (b) FILE a follow-up story in .factory/stories/ if deferred, or (c) record WONT-FIX with explicit justification in the cycle decision-log. All 9 must have an explicit disposition before F6 hardening begins.

#### F-P5-004 [HIGH]: S-12.08 input-hash [pending-recompute] — thrice unaddressed across passes 3/4/5

- **Severity:** HIGH
- **Category:** verification-gaps
- **Location:** `.factory/stories/S-12.08` (story spec frontmatter `input-hash` field)
- **Description:** F-P3-014 (pass-3), F-P4-009 (pass-4), and now pass-5 all observe that S-12.08 carries `input-hash: [pending-recompute]`. This finding has been acknowledged three consecutive times without remediation. A stale input-hash means any automated drift detection (`/vsdd-factory:check-input-drift`) will flag S-12.08 as potentially stale on every run, polluting drift reports and desensitizing operators to real drift signals. The fix is trivial — recompute and accept the current hash.
- **Evidence:** S-12.08 frontmatter `input-hash: [pending-recompute]` — unchanged across passes 3, 4, and 5.
- **Proposed Fix:** Run `/vsdd-factory:check-input-drift` on S-12.08 to recompute the hash. Update the frontmatter field. Verify no other story specs in the S-12 group carry stale hashes. Single-commit fix.

### MEDIUM

#### F-P5-005 [MEDIUM]: Forensic markers (321+) growing — pass-4 fix burst added more TODO(F-P4-NNN) markers; no follow-up story filed

- **Severity:** MEDIUM
- **Category:** code-quality
- **Location:** `crates/vsdd-context-resolvers/src/resolver_loader.rs:509-531`; codebase-wide forensic marker count
- **Description:** F-P3-004 identified 321+ forensic markers in the codebase and recommended filing a follow-up story. No follow-up story was filed. The pass-4 fix burst added MORE forensic markers: `resolver_loader.rs:509-531` now contains `TODO(F-P4-002)` and `TODO(VP-076-C / F-P4-004)`. The forensic marker count is growing, not declining. Continued accumulation without a cleanup story means the markers will persist indefinitely, conflating real TODO debt with pass-specific fix-burst annotations that should have been transient.
- **Evidence:** F-P3-004 identified 321+ markers. resolver_loader.rs:509-531 contains TODO(F-P4-002) and TODO(VP-076-C / F-P4-004) added in pass-4 burst. No follow-up story in .factory/stories/.
- **Proposed Fix:** File a follow-up story (e.g., STORY-NNN "Forensic marker cleanup — resolve 321+ TODO items") with acceptance criteria covering: (a) remove all pass-specific TODO(F-P[N]-NNN) markers, (b) convert legitimate TODOs into filed stories or inline comments with TD-NNN references, (c) CI lint rule to block future pass-specific marker introduction. STOP adding F-P5-NNN markers directly to production code.

#### F-P5-006 [MEDIUM]: VP-076.md:163 Expected Outcome bullet still asserts VP-076-C is verified; §Architectural Limitation (166-176) explicitly defers it — internal contradiction

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** `VP-076.md:163`; `VP-076.md:166-176`
- **Description:** F-P4-004 (PARTIAL) updated the VP-076-C status field but did not reconcile the Expected Outcome section. VP-076.md:163 contains a bullet asserting that VP-076-C property (internal_log capability confinement) is "verified." The §Architectural Limitation section at lines 166-176 explicitly states that this property is deferred and cannot be fully verified with the current architecture. These two statements contradict each other within the same document. A reader of VP-076.md:163 receives a false assurance that a deferred property has been proven.
- **Evidence:** VP-076.md:163 — bullet asserts "verified." VP-076.md:166-176 — §Architectural Limitation defers VP-076-C explicitly. No reconciliation performed in pass-4 fix burst.
- **Proposed Fix:** Strike through or tag the VP-076.md:163 bullet as DEFERRED. Add a cross-reference: "(see §Architectural Limitation — deferred to follow-up story)." Ensures a reader of either section receives consistent information about the verification status of VP-076-C.

#### F-P5-007 [MEDIUM]: Timeout bound calibration gap — 1100ms catches 25%+ reductions but misses 13% reductions; tighten or add structural telemetry assertion

- **Severity:** MEDIUM
- **Category:** coverage-gap
- **Location:** `plugins/vsdd-factory/tests/resolver-integration.bats` (timeout lower-bound assertion at 1100ms)
- **Description:** F-P4-005 was closed-with-caveat: the lower bound was raised from 1000ms to 1100ms against a 1500ms deadline. The pass-4 fix burst acknowledged in comments that this leaves a 13% regression corridor uncaught (1500ms → ~1300ms fires correctly but passes the 1100ms lower-bound check). This gap was noted at closure time but not addressed. A regression that reduces the timeout deadline by 13% would still produce a passing CI result, giving a false-green signal for a meaningful behavioral regression.
- **Evidence:** Timeout assertion at 1100ms; RESOLVER_TIMEOUT_MS = 1500ms; gap: regressions in 1100-1499ms range are not caught. Pass-4 commit comments acknowledge the calibration gap.
- **Proposed Fix:** Option A (preferred): Raise lower bound to 1400ms (within 100ms of deadline) and add upper bound of 3000ms to catch hung resolvers. Option B: Add a structural sink-telemetry assertion that captures the actual deadline-firing timestamp from vsdd::log output and asserts it is within ±100ms of RESOLVER_TIMEOUT_MS. Either option eliminates the 13% regression corridor.

### LOW

#### F-P5-008 [LOW — process-gap]: 4-pass recurrence of fix-burst-introduced same-class defects — no rule requiring CI green signal before CRITICAL CI-class finding closure

- **Severity:** LOW
- **Category:** process-gap
- **Location:** Fix-burst protocol documentation; cycle decision-log
- **Description:** Passes 2, 3, 4, and 5 have each introduced or failed to close a CI-class false-green finding (F-P2-001 inert hook, F-P3-001 bats not in CI, F-P4-002 bats added without staging, F-P5-001 staging missing). The pattern indicates that no rule in the current fix-burst protocol requires demonstrating an actual CI green signal before declaring a CRITICAL CI-class finding closed. Fix bursts self-certify closure without running the CI pipeline against the fix. This process gap is structural: it allows the same defect class to recur indefinitely because the verification mechanism (CI) is never invoked as part of the closure gate.
- **Evidence:** 4 consecutive CI-class CRITICAL findings across passes 2-5. No CI green signal referenced in any closure justification for these findings.
- **Proposed Fix:** Codify the following rule in the fix-burst protocol and cycle decision-log: "CRITICAL findings in the CI-class (coverage, staging, CI step invocation) MUST demonstrate a CI green signal on a PR branch before the finding is recorded as CLOSED. Self-certification is not accepted for this class." Add to STATE.md Decisions Log as an explicit governance decision.

## Observations

- **F-P5-009 [observation — positive]:** TD-074 registration and the 3 sibling struct TODO additions (F-P4-006 closure) were executed correctly. The fix correctly scoped the closure to process-level (register debt + annotate) without overclaiming structural remediation. This is high-quality process closure and provides a model for future tech-debt registration closures in this cycle.

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 1 |
| HIGH | 3 |
| MEDIUM | 3 |
| LOW | 1 |
| Process-gap | 4 |

**Overall Assessment:** block
**Convergence:** findings remain — iterate
**Readiness:** requires revision

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 5 |
| **New findings** | 8 |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 1.0 (8/8) |
| **Median severity** | 2.5 (between HIGH and MEDIUM) |
| **Trajectory** | 29→11→9→**8** |
| **Verdict** | FINDINGS_REMAIN |

## Convergence Trajectory

| Pass | Classification | Critical | High | Medium | Low | Total |
|------|---------------|----------|------|--------|-----|-------|
| P1 | CRITICAL | 2 | 10 | 12 | 5 | 29 |
| P2 | CRITICAL | 2 | 4 | 5 | 0 | 11 |
| P3 | CRITICAL | 2 | 4 | 3 | 0 | 9 |
| P4 | CRITICAL | 2 | 4 | 3 | 0 | 9 |
| P5 | CRITICAL | 1 | 3 | 3 | 1 | **8** |

Severity floor stable at CRITICAL across all 5 passes. Defect class shifted: P1-P3 structural gaps → P4 CI-coverage gap → P5 fix-burst-introduced regression (CI WASM staging). Fix bursts closing individual findings while introducing same-class defects in adjacent artifacts.

## Top 5 Most Important Findings (F5 pass-5+ fix burst drivers)

1. **CRITICAL F-P5-001** — Add WASM staging step to ci.yml before bats step; MUST show CI green before closure (CI)
2. **HIGH F-P5-002** — Address BC-7.03.091/092 TBD content gaps; fix hook path from .sh to .wasm; add canonical SEQUENCE schema fixture (SPEC)
3. **HIGH F-P5-003** — Triage all 9 accumulated MEDIUMs: close with evidence, file story, or record WONT-FIX (PROCESS)
4. **HIGH F-P5-004** — Recompute S-12.08 input-hash; single-commit fix (STATE)
5. **MEDIUM F-P5-005** — File forensic-marker cleanup story; STOP adding F-P5-NNN markers in production code (CODE)

## Recommendation

**Cycle CANNOT proceed to F6 hardening.** Continue F5 fix bursts. This is the 5th consecutive cycle-level CRITICAL verdict.

Required before next adversary pass:
- REQUIRED: Close F-P5-001 (CI WASM staging — MUST verify actual CI green signal on PR branch before closure; no self-certification)
- REQUIRED: Close F-P5-002 (BC governance — address F-P4-008 skip; fix BC content, not changelog)
- REQUIRED: Close F-P5-003 (triage 9 accumulated MEDIUMs — explicit disposition for each)
- REQUIRED: Close F-P5-004 (input-hash recompute — trivial, 3 passes overdue)

Strongly recommended before F6:
- STRONGLY RECOMMENDED: Close F-P5-005 (file forensic-marker follow-up story; no more code-embedded pass markers)
- STRONGLY RECOMMENDED: Close F-P5-006 (VP-076.md:163 DEFERRED tag)
- STRONGLY RECOMMENDED: Close F-P5-007 (tighten timeout bound to 1400ms or add telemetry assertion)

Process-gap codification (F-P5-008): Add CI-green-signal requirement to fix-burst protocol for CRITICAL CI-class findings.

`convergence_reached`: false. Verdict CRITICAL. Need 3 consecutive NITPICK_ONLY cycle-level passes for cycle convergence.

## Process-Gap Findings (4)

F-P5-001 (CI staging gap — same class as F-P4-002 fix-burst regression), F-P5-003 (9 accumulated MEDIUMs without triage), F-P5-008 (no CI-green-signal rule for CRITICAL CI-class closures), F-P5-005 (forensic markers growing with no cleanup story).
