---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-05-10T00:00:00Z
phase: F5
inputs:
  - .factory/specs/behavioral-contracts/ss-04/BC-4.10.001.md
  - .factory/specs/verification-properties/VP-076.md
  - .factory/tech-debt-register.md
  - crates/hook-plugins/warn-pending-wave-gate/src/lib.rs
  - plugins/vsdd-factory/tests/integration/E-8-hook-plugins/warn-pending-wave-gate.bats
  - plugins/vsdd-factory/tests/resolver-integration.bats
  - plugins/vsdd-factory/tests/resolver-capability-confinement.bats
  - .github/workflows/ci.yml
  - .github/workflows/release.yml
input-hash: "[pending-recompute]"
traces_to: prd.md
pass: 4
previous_review: adv-cycle-pass-3.md
cycle: v1.0-feature-engine-discipline-pass-1
prior-pass-classification: CRITICAL
prior-findings-count: 11
verdict: CRITICAL
findings_count: { critical: 2, high: 4, medium: 3, low: 0, nitpick: 0 }
observations: 3
deferred: 0
process_gap_count: 6
convergence_reached: false
---

# Adversarial Review — F5 Pass-4 (Cycle-Level, Fresh Context)

Fresh-context audit of the F5 fix burst applied after pass-3 CRITICAL verdict. F-P3-001 (schema fix), F-P3-005 (BC v1.4), F-P3-006 (TD register), and F-P3-007 (narrow path_allow) were reported as remediated. This pass verifies those closure claims and conducts a clean sweep across all affected files.

## Finding ID Convention

Finding IDs in this cycle use the shorthand format `F-P4-NNN` (cycle-scoped pass-4 findings). Formal ADV IDs follow: `ADV-<CYCLE>-P<PASS>-<SEV>-<SEQ>` per the factory standard. The cycle prefix is omitted per legacy convention established in passes 1–3.

## Part A — Fix Verification (Pass-3 Closure Summary)

| Pass-3 ID | Status | Evidence |
|---|---|---|
| F-P3-001 (CRIT) — warn-pending-wave-gate reads MAPPING form | **PARTIAL** | lib.rs:49 migrated to sequence form; production code fixed. Sibling bats test at `tests/integration/E-8-hook-plugins/warn-pending-wave-gate.bats:100-163` still exercises MAPPING fixture — same defect class orphaned in test layer. See F-P4-001. |
| F-P3-002 (CRIT) — VP-076 capability confinement unproven | **PARTIAL** | `resolver-capability-confinement.bats` file created and registered. VP-076-C stub claim "verified structurally" is FALSE — references nonexistent TODO comment in resolver_loader.rs HostContext. See F-P4-003 (anchor) and F-P4-004 (false stub claim). |
| F-P3-003 (HIGH) — bats coverage 1/5 block codes | **PARTIAL** | `resolver-integration.bats` extended; 3 of 4 missing block codes now covered + 1 skip. `resolver-capability-confinement.bats` exists. Neither file is invoked by ci.yml on PRs. See F-P4-002. |
| F-P3-004 (HIGH) — 321 forensic markers | OPEN | Deferred — no follow-up story filed yet per pass-3 recommendation. |
| F-P3-005 (HIGH) — BC PC11 hook.block mandate | **CLOSED** | BC-4.10.001.md bumped to v1.4; PC11 "emit hook.block before returning HookResult::Block" added. Verified. |
| F-P3-006 (HIGH) — TD-072/TD-073 not in register | **CLOSED** | tech-debt-register.md now contains TD-072 (serde_yaml deprecation) and TD-073 (warn-pending-wave-gate schema disagreement). Verified. |
| F-P3-007 (HIGH) — path_allow over-grants | **CLOSED** | resolvers-registry.toml:17 narrowed to `[".factory/wave-state.yaml", ".factory/STATE.md"]`. Verified. |
| F-P3-008 (HIGH) — no timeout integration test | **PARTIAL** | Timeout bats test added with lower-bound assertion of 1000ms. Bound too loose to catch regressions within the 1500ms window. See F-P4-005. |
| F-P3-009 (MED) — no forward-compat test | OPEN | Not addressed in F-P3 fix burst. |
| F-P3-010 (MED) — non-UTF8 AbiViolation untested | OPEN | Not addressed in F-P3 fix burst. |
| F-P3-011 (MED) — vsdd::log observability unverified | OPEN | Not addressed in F-P3 fix burst. |
| F-P3-012 (obs) — no cross-pass re-verify checkpoint | OPEN | Process gap persists. |
| F-P3-013 (obs) — no cross-consumer schema-convergence test | OPEN | See F-P4-006. |
| F-P3-014 (obs) — S-12.08 input-hash pending | OPEN | S-12.08 input-hash still `[pending-recompute]`. See F-P4-009. |

## Part B — New Findings

### CRITICAL

#### F-P4-001 [CRITICAL]: Stale MAPPING-form bats fixture orphaned by F-P3-001 fix — same sibling-coverage defect class as F-P2-001/F-P3-001

- **Severity:** CRITICAL
- **Category:** verification-gaps
- **Location:** `plugins/vsdd-factory/tests/integration/E-8-hook-plugins/warn-pending-wave-gate.bats:100-163`
- **Description:** The F-P3-001 fix migrated production `lib.rs:49` from `as_mapping()` to `as_sequence()`, correctly aligning code with the producer's SEQUENCE form. However, the sibling bats test file at `warn-pending-wave-gate.bats:100-163` still seeds a MAPPING-form fixture (`waves: {wave1: ...}` YAML dictionary) and asserts "WAVE GATE REMINDER" emission against it. The fixture no longer matches the schema the production code reads. The test now exercises a path that cannot emit the reminder — producing a false-green result if the production code ever regresses back to MAPPING form, and masking the regression if a future change breaks SEQUENCE parsing. This is the same defect class (production fix without sibling bats migration) as F-P2-001 (fix inert hook without updating test harness) and F-P3-001 (schema fix without updating resolver bats).
- **Evidence:** `lib.rs:49` reads `v.as_sequence()`; bats fixture at lines 100-163 seeds mapping-form `waves:` dict. Fixture inputs and production code are now semantically mismatched.
- **Proposed Fix:** Rewrite `warn-pending-wave-gate.bats:100-163` fixture to use SEQUENCE form (`waves: [{name: wave1, status: pending, ...}]`). Verify bats assertion still passes end-to-end. Add a negative fixture (MAPPING form) asserting no reminder emitted to lock against regression.

#### F-P4-002 [CRITICAL]: F-P3 fix-burst bats tests not invoked by ci.yml on PRs — all regression guards from F-P3 fix burst are functionally inert until merge

- **Severity:** CRITICAL
- **Category:** process-gap
- **Location:** `.github/workflows/ci.yml` (validate job); `.github/workflows/release.yml` (run-all.sh)
- **Description:** The `resolver-integration.bats` and `resolver-capability-confinement.bats` files created during the F-P3 fix burst are not referenced in any step of `ci.yml`. The only place `run-all.sh` is invoked is `release.yml`, which runs post-merge at release cut time. As a result, every PR from the F-P3 fix burst received CI green status without executing a single line of the new bats regression guards. A future PR that breaks resolver integration or capability confinement will receive a false-green CI result on PR open. This is a POL-11 false-green class finding identical in mechanism to F-P3-002 (security property unproven in CI): the artifacts exist, the CI does not exercise them.
- **Evidence:** `ci.yml` validate job contains no `bats` invocation or reference to `resolver-integration.bats` or `resolver-capability-confinement.bats`. `release.yml` step `run-all.sh` would cover them, but only runs on release branches, not on every PR to develop.
- **Proposed Fix:** Add a `bats` step to the `ci.yml` validate job (or a dedicated `integration-tests` job) that runs `plugins/vsdd-factory/tests/resolver-integration.bats` and `plugins/vsdd-factory/tests/resolver-capability-confinement.bats` (and the `E-8-hook-plugins/warn-pending-wave-gate.bats` suite) on every PR targeting develop or main. Confirm step is gated so CI fails if any bats test fails.

### HIGH

#### F-P4-003 [HIGH]: VP-076.md:202 anchor references nonexistent bats file path

- **Severity:** HIGH
- **Category:** spec-fidelity
- **Location:** `VP-076.md:202`
- **Description:** VP-076-C cites the evidence file as `wave_context_resolver_capability_confinement.bats`. The actual file created during the F-P3 fix burst is `resolver-capability-confinement.bats` (no `wave_context_` prefix). The anchor is broken — following it leads to a file not found, and any automated traceability tooling that validates VP → evidence anchors will report a false-missing-evidence gap.
- **Evidence:** `plugins/vsdd-factory/tests/resolver-capability-confinement.bats` exists. VP-076.md:202 links to `wave_context_resolver_capability_confinement.bats`.
- **Proposed Fix:** Update VP-076.md:202 anchor to `resolver-capability-confinement.bats`.

#### F-P4-004 [HIGH]: VP-076-C "verified structurally" stub claim is FALSE — references nonexistent TODO in resolver_loader.rs HostContext

- **Severity:** HIGH
- **Category:** verification-gaps
- **Location:** `VP-076.md` (VP-076-C verification status field); `crates/vsdd-context-resolvers/src/resolver_loader.rs` (HostContext impl)
- **Description:** VP-076-C records status as "verified structurally" and cites a TODO comment in `resolver_loader.rs HostContext` as evidence that internal_log is wired. That TODO comment does not exist in the current codebase. The actual HostContext impl does not contain a TODO referencing VP-076-C or internal_log wiring. The stub claim was authored during the F-P3 fix burst without verifying the evidence in the file. VP-076-C remains unverified in reality.
- **Evidence:** `grep -n "internal_log\|VP-076" crates/vsdd-context-resolvers/src/resolver_loader.rs` returns no match for the cited TODO. VP-076-C status field asserts "verified structurally".
- **Proposed Fix:** Either (a) wire `internal_log` in the HostContext impl and re-verify VP-076-C as truly verified, or (b) revert VP-076-C status to "unverified" and remove the false stub claim. Option (a) is the correct long-term fix; (b) is acceptable for this pass if implementation is deferred to a follow-up story.

#### F-P4-005 [HIGH]: Timeout test lower-bound 1000ms too loose — will not catch regressions within the 1500ms enforcement window

- **Severity:** HIGH
- **Category:** coverage-gap
- **Location:** `plugins/vsdd-factory/tests/resolver-integration.bats` (timeout bats test, lower-bound assertion)
- **Description:** The timeout bats test added for F-P3-008 asserts that a long-running resolver scenario completes in more than 1000ms. The production timeout constant is `RESOLVER_TIMEOUT_MS = 1500`. A regression that fires the timeout deadline at 500ms (half the intended value) would pass the 1000ms lower-bound check — the test would still see "completed" at 500ms and report pass, because the resolver was killed before 1000ms elapsed. Only regressions that eliminate the timeout mechanism entirely are caught. This is a correctness-of-test finding: the lower bound must be set high enough to falsify the case where the deadline fires too early.
- **Evidence:** Lower-bound assertion is 1000ms; `RESOLVER_TIMEOUT_MS = 1500`; a deadline firing at 600ms passes the assertion.
- **Proposed Fix:** Raise lower bound to at least 1400ms (within 100ms of the 1500ms deadline). Optionally add an upper bound of 3000ms to catch cases where the timeout is not firing at all (resolver runs to completion on a slow resolver). Update bats assertion accordingly.

#### F-P4-006 [HIGH]: Three sibling WaveEntry structs persist across workspace — TD-073 root cause unaddressed

- **Severity:** HIGH
- **Category:** code-quality
- **Location:** Three separate crate locations in workspace (warn-pending-wave-gate/src/lib.rs; vsdd-context-resolvers/src/lib.rs or resolver.rs; at least one additional crate)
- **Description:** TD-073 was registered in the tech-debt-register.md during the F-P3 fix burst (CLOSED: F-P3-006), but the structural root cause — multiple sibling definitions of `WaveEntry` struct across the workspace — was not addressed. Each crate defines its own `WaveEntry` with potentially divergent field sets. The next schema change (e.g., adding a new field to wave-state.yaml) requires updating every sibling independently, with no compile-time guarantee that all consumers stay in sync. This is precisely the structural condition that allowed TD-073 to manifest as a CRITICAL production defect (F-P3-001). Registering the debt closes the process gap but does not reduce the structural risk.
- **Evidence:** `grep -rn "struct WaveEntry" .` returns matches in at least three separate crate source files. TD-073 in tech-debt-register.md is status OPEN with no follow-up story filed.
- **Proposed Fix:** Hoist `WaveEntry` (and related wave-state types) into a dedicated shared crate (e.g., `crates/wave-state-types`) or into an existing common crate. All consumers import from the shared definition. Alternatively, if a full hoist is out of scope for this cycle, file a concrete follow-up story with acceptance criteria and add the story number to TD-073's register entry so it is not forgotten.

### MEDIUM

#### F-P4-007 [MED]: warn-pending-wave-gate bats docstrings reference removed as_mapping() semantics

- **Severity:** MEDIUM
- **Category:** documentation-drift
- **Location:** `plugins/vsdd-factory/tests/integration/E-8-hook-plugins/warn-pending-wave-gate.bats` (test docstrings / comments throughout)
- **Description:** Following the F-P3-001 schema migration from MAPPING to SEQUENCE form, test docstrings and inline comments in the bats file still describe the MAPPING fixture structure (`waves: {wave1: {...}}` dict) as the expected schema. This creates a documentation-drift hazard: the next developer reading the test sees docstrings that describe the old contract and may cargo-cult the MAPPING fixture into a new test.
- **Evidence:** F-P3-001 fix changed lib.rs to `as_sequence()`. The bats docstrings were not updated as part of the fix burst.
- **Proposed Fix:** Update bats docstrings and fixture comments to reflect SEQUENCE form. Verify all `@description` / `# Given` / `# When` / `# Then` annotations match the actual fixture schema.

#### F-P4-008 [MED]: BC-7.03.091/092 do not specify wave-state.yaml schema canonically — BC-level governance gap that allowed TD-073 to recur

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** `.factory/specs/behavioral-contracts/ss-07/BC-7.03.091.md`; `BC-7.03.092.md`
- **Description:** The behavioral contracts that govern wave-state.yaml (BC-7.03.091 producer contract and BC-7.03.092 consumer contract) do not canonically define the SEQUENCE form for the `waves` field. The contracts reference wave-state.yaml semantics at an abstract level but do not include a schema fixture (example YAML) or formal field-type assertion (`waves: SEQUENCE of WaveEntry objects`). This BC-level ambiguity is the structural condition that allowed the MAPPING vs SEQUENCE schema disagreement (TD-073 / F-P3-001 / F-P4-001) to originate and survive multiple passes without being flagged as a contract violation. A developer reading only BC-7.03.091/092 cannot determine the correct schema form from the contracts alone.
- **Evidence:** BC-7.03.091/092 exist. Neither contains an example YAML fixture or explicit `type: sequence` assertion for the `waves` field.
- **Proposed Fix:** Add a canonical schema fixture block to BC-7.03.091 and BC-7.03.092 showing the SEQUENCE form. Add a PC asserting "waves field MUST be a YAML sequence (list) of WaveEntry objects; MAPPING (dict) form is invalid". Bump both contracts.

#### F-P4-009 [MED]: S-12.08 input-hash still [pending-recompute] — F-P3-014 unfixed

- **Severity:** MEDIUM
- **Category:** verification-gaps
- **Location:** `.factory/stories/S-12.08` (story spec frontmatter)
- **Description:** F-P3-014 identified that S-12.08 carries `input-hash: [pending-recompute]` in its frontmatter. This was not addressed in the F-P3 fix burst. The stale hash means the story's artifact currency check will always report as uncertain, and any automated drift detection (e.g., `/vsdd-factory:check-input-drift`) will flag this story as potentially stale on every run.
- **Evidence:** S-12.08 story spec frontmatter `input-hash: [pending-recompute]` — unchanged from pass-3 observation.
- **Proposed Fix:** Run `/vsdd-factory:check-input-drift` on S-12.08 to recompute and accept the current hash. Update frontmatter accordingly.

## Observations [process-gap]

- **F-P4-010:** The F-P3 fix burst created bats test files and updated production code in the same commit (or burst) without a CI gate that exercises those tests. This is the third consecutive pass where a fix burst introduced new bats tests that were not verified by CI before the burst was declared complete (F-P2-001 class, F-P3-001 class, F-P4-002 class). A process-level guardrail is required: no fix burst that creates or modifies bats files should be declared CLOSED until a CI run has executed and passed those specific bats files on a PR.
- **F-P4-011:** Pass-2 findings F-P2-003/004/007/009/010/011 were explicitly left open in pass-3 with a note to "re-verify in a dedicated reconciliation pass". No such reconciliation pass has occurred. These six findings are now three passes old without explicit triage. Either close them with evidence or file follow-up stories so they are not silently dropped.
- **F-P4-012:** Convergence trajectory for this cycle is P1 CRITICAL → P2 CRITICAL → P3 CRITICAL → P4 CRITICAL. The severity floor is stable. The class of defects has changed each pass (inert hook → schema mismatch → sibling bats orphan → CI gap), but the severity level has not decreased. This pattern suggests the fix bursts are closing individual defects without addressing the structural conditions (shared schema definition, CI coverage of bats, sibling coverage discipline) that allow defects of the same class to recur. Consider a structural-remediation pass targeting all three root conditions before the next adversarial pass.

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 2 |
| HIGH | 4 |
| MEDIUM | 3 |
| LOW | 0 |

**Overall Assessment:** block
**Convergence:** findings remain — iterate
**Readiness:** requires revision

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 4 |
| **New findings** | 9 |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 1.0 (9/9) |
| **Median severity** | 2.0 |
| **Trajectory** | 29→(pass-2)→11→9 |
| **Verdict** | FINDINGS_REMAIN |

## Top 5 Most Important Findings (F5 pass-4+ fix burst drivers)

1. **CRITICAL F-P4-001** — Rewrite stale MAPPING bats fixtures in warn-pending-wave-gate.bats (TEST)
2. **CRITICAL F-P4-002** — Add bats invocation to ci.yml validate job (CI)
3. **HIGH F-P4-003** — Correct VP-076 anchor to resolver-capability-confinement.bats (SPEC)
4. **HIGH F-P4-004** — Wire internal_log OR amend VP-076-C and remove false stub claim (SPEC/CODE)
5. **HIGH F-P4-005** — Raise timeout test lower-bound to ≥1400ms (TEST)

## Recommendation

**Cycle CANNOT proceed to F6 hardening.** Continue F5 fix bursts.

Required before next adversary pass:
- REQUIRED: Close F-P4-001 (rewrite stale bats fixtures; wire into CI if F-P4-002 closed first)
- REQUIRED: Close F-P4-002 (add bats execution step to ci.yml validate job)
- REQUIRED: Close F-P4-003 (correct VP-076.md:202 anchor)
- REQUIRED: Close F-P4-004 (wire internal_log OR amend VP-076-C + remove false stub claim)

Strongly recommended before F6:
- STRONGLY RECOMMENDED: Close F-P4-005 (tighten timeout test lower bound)
- STRONGLY RECOMMENDED: Close F-P4-006 (hoist shared WaveState to common crate OR file follow-up story with story number in TD-073)

`convergence_reached`: false. Verdict CRITICAL. Need 3 consecutive NITPICK_ONLY cycle-level passes for cycle convergence.

## Process-Gap Findings (6)

F-P4-002, F-P4-007 (documentation drift), F-P4-008 (BC governance gap), F-P4-010 (fix-burst CI guardrail missing), F-P4-011 (pass-2 reconciliation overdue), F-P4-012 (structural root-cause not addressed).
