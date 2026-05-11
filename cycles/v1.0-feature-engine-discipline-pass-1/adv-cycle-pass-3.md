---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-05-11T00:00:00Z
phase: F5
inputs:
  - .factory/specs/behavioral-contracts/ss-04/BC-4.10.001.md
  - .factory/specs/verification-properties/VP-076.md
  - .factory/tech-debt-register.md
  - crates/hook-plugins/warn-pending-wave-gate/src/lib.rs
  - crates/vsdd-context-resolvers/tests/capability_confinement_test.rs
  - plugins/vsdd-factory/tests/resolver-integration.bats
  - plugins/vsdd-factory/resolvers-registry.toml
input-hash: "[pending-recompute]"
traces_to: prd.md
pass: 3
previous_review: adv-cycle-pass-2.md
cycle: v1.0-feature-engine-discipline-pass-1
prior-pass-classification: CRITICAL
prior-findings-count: 15
verdict: CRITICAL
findings_count: { critical: 2, high: 6, medium: 3, low: 0, nitpick: 0 }
observations: 3
deferred: 0
process_gap_count: 5
convergence_reached: false
---

# Adversarial Review — F5 Pass-3 (Cycle-Level, Fresh Context)

Re-baseline of cycle v1.0-feature-engine-discipline-pass-1 after F4 platform delivery COMPLETE (all 6 stories merged: S-12.03 PR#120, S-12.04 PR#121, S-12.05 PR#119, S-12.06 PR#105, S-12.07 PR#122, S-12.08 PR#123).

## Finding ID Convention

Finding IDs in this cycle use the shorthand format `F-P3-NNN` (cycle-scoped pass-3 findings). Formal ADV IDs follow: `ADV-<CYCLE>-P<PASS>-<SEV>-<SEQ>` per the factory standard. The cycle prefix for this cycle is omitted per legacy convention established in passes 1 and 2.

## Part A — Fix Verification (Pass-2 Closure Summary)

| Pass-2 ID | Status | Evidence |
|---|---|---|
| F-P2-001 (CRIT) — Convergence hook inert | **CLOSED** | S-12.08 PR#123 99d24315; bats resolver-integration.bats AC-008 end-to-end |
| F-P2-002 (CRIT) — BC VP-row drift | **CLOSED** | BC-4.10.001.md:137 + BC-5.39.001.md:122 now "Block Invariant" wording |
| F-P2-003 (HIGH) — F1-delta L391 | OPEN | Not re-verified in this pass |
| F-P2-004 (HIGH) — S-13.01 type names | OPEN | Not re-verified in this pass |
| F-P2-005 (architect — merge signature) | **CLOSED** | resolver.rs:458-461 returns (Map, Vec\<CollisionInfo\>); no callback |
| F-P2-002 (architect — Resolver trait) | **CLOSED** | grep returns no matches in hook-sdk |
| F-P2-006 (HIGH) — BC PC for hook.block emit | OPEN | Carried forward as F-P3-005 |
| F-P2-007 (HIGH) — log_* contradiction | OPEN | Not re-verified in this pass |
| F-P2-008 (MED) — production integration test | **CLOSED** | resolver-integration.bats end-to-end |
| F-P2-009/010/011 (MED) | OPEN | Not re-verified in this pass |
| F-P2-012 (MED) — capability path_allow | **CLOSED** | resolvers-registry.toml:17 (but see F-P3-007 over-permissive) |
| F-P2-013/014/015 (process-gap) | CARRIED FORWARD | See F-P3-012/013/014 |

## Part B — New Findings (or all findings for pass 1)

### CRITICAL

#### F-P3-001 [CRITICAL]: warn-pending-wave-gate reads MAPPING form but producer writes SEQUENCE — hook operationally INERT (same defect class as F-P2-001)

- **Severity:** CRITICAL
- **Category:** verification-gaps
- **Location:** `crates/hook-plugins/warn-pending-wave-gate/src/lib.rs:49`
- **Description:** `state.get("waves").and_then(|v| v.as_mapping())` — producer and new resolver use sequence form. TD #73 attested in-code at lib.rs:1-6 but NOT in tech-debt-register.md.
- **Evidence:** Wave-gate reminder mechanism dead in production. Same SOUL.md #4 silent-failure class as F-P2-001.
- **Proposed Fix:** Migrate lib.rs:49-63 to read sequence form: `state.get("waves").and_then(|v| v.as_sequence())`. Reuse WaveEntry struct or define local sibling. Add bats integration test asserting "WAVE GATE REMINDER" emission.

#### F-P3-002 [CRITICAL]: VP-076 capability confinement verified ONLY by 3 `#[ignore]` + `unimplemented!()` stubs — security property UNPROVEN in CI

- **Severity:** CRITICAL
- **Category:** security-surface
- **Location:** `crates/vsdd-context-resolvers/tests/capability_confinement_test.rs` lines 30-41, 50-60, 74-85
- **Description:** All 3 stubs are `#[ignore]` + unimplemented!(). S-12.07/S-12.08 deferred AC-005/AC-006 to bats; bats NEVER got the harness. Only `resolver_load_test.rs:849-882` has any capability test, but it is the trivial "no capability at all" case — NOT VP-076's "resolver WITH path_allow IS DENIED reading /etc/passwd".
- **Evidence:** Flagship security property is false-green. A future resolver bump could read /etc/passwd and we have no regression test. POL-11 axis (positive-coverage assertion for security CI).
- **Proposed Fix:** Author real bats test (or sibling) that builds resolver fixture calling host::read_file("/etc/passwd") with path_allow=[".factory/"]; assert resolver.capability_denied event in sink + no /etc/passwd content leaked. Un-skip the 3 stubs or replace with `#[ignore = "moved to bats: ..."]` pointing to the new test.

### HIGH

#### F-P3-003 [HIGH]: Bats integration coverage 1/5 — 4 block codes have no end-to-end test

- **Severity:** HIGH
- **Category:** coverage-gap
- **Location:** `plugins/vsdd-factory/tests/resolver-integration.bats:173` (only CONVERGENCE_PASSES_INSUFFICIENT)
- **Description:** 4 block codes lack bats coverage: CONVERGENCE_STATE_MISSING, CONVERGENCE_CLASSIFICATION_INSUFFICIENT, WAVE_CONTEXT_MISSING, WAVE_CONTEXT_SCHEMA_ERROR.
- **Evidence:** Only 1 of 5 block codes verified end-to-end.
- **Proposed Fix:** Add 4 bats cases; each seeds a specific scenario and asserts the corresponding block code in sink.

#### F-P3-004 [HIGH] [process-gap]: ~321 forensic markers across 26 files persist in production source

- **Severity:** HIGH
- **Category:** code-quality
- **Location:** resolver_loader.rs (17), resolver.rs (21), resolver_classify_trap.rs (21), validate-per-story-adversary-convergence/lib.rs (32), 22 others
- **Description:** Grep `F-MED-\|F-P2-\|F-P3-\|P02-\|P03-\|HIGH-00\|MED-00\|LOW-00` returns 321 matches across 26 files. Forensic review markers persist in production source.
- **Evidence:** 321 matches confirmed by grep.
- **Proposed Fix:** Author follow-up story (E-14 or new tech-debt epic) for cycle-close code-comment hygiene. Convert each forensic marker to BC/VP reference OR delete if redundant.

#### F-P3-005 [HIGH]: BC-4.10.001 v1.3 added PC9/PC10 but missed PC11 mandate for hook.block event emission (F-P2-006 still open)

- **Severity:** HIGH
- **Category:** spec-fidelity
- **Location:** `.factory/specs/behavioral-contracts/ss-04/BC-4.10.001.md` v1.3
- **Description:** PC1-PC10 declared; code emits hook.block at 4+ sites; NO PC mandates emit_event before block return.
- **Evidence:** F-P2-006 carried forward unresolved.
- **Proposed Fix:** Add PC11: "Before returning HookResult::Block for any code (PC2-PC4, PC9-PC10), hook MUST emit hook.block event via host::emit_event with fields hook/code/story/reason". Bump to v1.4.

#### F-P3-006 [HIGH] [process-gap]: TD #72 + TD #73 declared in code but NOT in tech-debt-register.md

- **Severity:** HIGH
- **Category:** verification-gaps
- **Location:** `.factory/tech-debt-register.md` (highest entry TD-027; no TD-072/TD-073); `crates/hook-plugins/warn-pending-wave-gate/src/lib.rs:1-6` (in-code attestation)
- **Description:** Two tech-debt items declared in code comments have no corresponding entries in the register.
- **Evidence:** tech-debt-register.md highest entry is TD-027; warn-pending-wave-gate lib.rs:1-6 attests TD-073 in-code.
- **Proposed Fix:** Add TD-072 (serde_yaml ecosystem deprecation; 26 files; MEDIUM severity) + TD-073 (warn-pending-wave-gate schema disagreement; CRITICAL severity — actually a BUG, not debt, must close in cycle).

#### F-P3-007 [HIGH]: path_allow = [".factory/"] over-grants — VP-076 principle of least privilege violated

- **Severity:** HIGH
- **Category:** security-surface
- **Location:** `plugins/vsdd-factory/resolvers-registry.toml:17`; `crates/vsdd-context-resolvers/src/lib.rs:76,82`
- **Description:** Resolver reads ONLY wave-state.yaml + STATE.md but capability grants entire .factory/ directory tree.
- **Evidence:** resolvers-registry.toml:17 grants path_allow=[".factory/"]; lib.rs:76,82 reads only two specific files.
- **Proposed Fix:** Narrow to `path_allow = [".factory/wave-state.yaml", ".factory/STATE.md"]`. Add integration test asserting narrower grant still works.

#### F-P3-008 [HIGH]: No production integration test for concurrent resolvers or 1500ms timeout

- **Severity:** HIGH
- **Category:** coverage-gap
- **Location:** `plugins/vsdd-factory/tests/resolver-integration.bats` (no timeout case); resolver_loader.rs:565 (RESOLVER_TIMEOUT_MS = 1500)
- **Description:** The 1500ms timeout enforcement path has no bats-level regression test; concurrent resolver dispatch also untested.
- **Evidence:** No bats case seeds a slow resolver; RESOLVER_TIMEOUT_MS constant is verified only by unit test.
- **Proposed Fix:** Add bats case registering long_running_resolver (1s sleep) alongside wave_context; assert dispatch completes <3000ms + resolver.timeout event in sink + wave_context still received.

### MEDIUM

#### F-P3-009 [MED]: No test for forward-compat tolerance of extra wave_context fields

- **Severity:** MEDIUM
- **Category:** missing-edge-cases
- **Location:** `crates/vsdd-context-resolvers/src/`
- **Description:** No test verifies that unknown fields in wave_context output are tolerated by consumers.
- **Proposed Fix:** Add `test_extract_stories_tolerates_unknown_wave_context_fields`.

#### F-P3-010 [MED]: No integration test for resolver returning non-UTF8 — AbiViolation path untested

- **Severity:** MEDIUM
- **Category:** coverage-gap
- **Location:** `crates/vsdd-context-resolvers/`
- **Description:** WAT fixture path that writes random bytes + packed-i64 ptr is not tested end-to-end.
- **Proposed Fix:** Author WAT fixture writing random bytes + packed-i64 ptr and assert AbiViolation event in sink.

#### F-P3-011 [MED]: vsdd::log from resolver to plugin.log in sink — observability path unverified end-to-end

- **Severity:** MEDIUM
- **Category:** verification-gaps
- **Location:** `plugins/vsdd-factory/tests/resolver-integration.bats`
- **Description:** The vsdd::log → plugin.log observability chain has no bats-level assertion.
- **Proposed Fix:** Extend bats empty-wave case to assert plugin.log appears in sink.

## Observations [process-gap]

- **F-P3-012:** Cycle has no checkpoint that re-verifies pass-N findings before F4-complete. Process-gap: 6 pass-2 findings (F-P2-003/004/007/009/010/011) survived complete F4 sub-batch without explicit triage.
- **F-P3-013:** No cross-consumer wave-state.yaml schema-convergence test. Process-gap: each new consumer can silently re-introduce TD #73 class of bug.
- **F-P3-014:** S-12.08 v1.2 input-hash `[pending-recompute]`. Run /vsdd-factory:check-input-drift to accept.

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 2 |
| HIGH | 6 |
| MEDIUM | 3 |
| LOW | 0 |

**Overall Assessment:** block
**Convergence:** findings remain — iterate
**Readiness:** requires revision

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 3 |
| **New findings** | 11 |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 1.0 (11/11) |
| **Median severity** | 2.5 |
| **Trajectory** | 29→(pass-2 count)→11 |
| **Verdict** | FINDINGS_REMAIN |

## Top 5 Most Important Findings (F5 pass-3+ fix burst drivers)

1. **CRITICAL F-P3-001** — warn-pending-wave-gate schema fix (CODE)
2. **CRITICAL F-P3-002** — Real VP-076 bats capability harness (TEST)
3. **HIGH F-P3-003** — 4 new bats block-code tests
4. **HIGH F-P3-005** — BC-4.10.001 v1.4 PC11 hook.block mandate
5. **HIGH F-P3-006** — TD-register entries for TD-072 + TD-073

## Recommendation

**Cycle CANNOT proceed to F6 hardening.** F5 needs additional fix bursts to converge:
- REQUIRED: Close F-P3-001 (CRITICAL) and F-P3-002 (CRITICAL)
- REQUIRED: Close F-P3-003, F-P3-005, F-P3-006 (HIGH — must close)
- STRONGLY RECOMMENDED before F6: F-P3-007, F-P3-008, F-P3-004 (follow-up story)
- RE-VERIFY: pass-2 still-open findings F-P2-003/004/007/009/010/011 in dedicated reconciliation pass

`convergence_reached`: false. Verdict CRITICAL. Need 3 consecutive NITPICK_ONLY cycle-level passes for cycle convergence.

## Process-Gap Findings (5)

F-P3-002, F-P3-004, F-P3-006, F-P3-012, F-P3-013.
