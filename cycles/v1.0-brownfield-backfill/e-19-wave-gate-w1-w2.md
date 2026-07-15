---
document_type: wave-gate-report
wave: E-19-combined-W1-W2
producer: state-manager
decision: D-845
date: 2026-07-15
scope: combined W1+W2 (retroactive — no formal W1 gate had run; D-835 governance approval was W1's exit record)
stories:
  - {id: S-19.01, pr: 613, sha: "8d1721f7", wave: W1, merged: "2026-07-13"}
  - {id: S-19.02, pr: 610, sha: "f5ea12e9", wave: W1, merged: "2026-07-13"}
  - {id: S-19.03, pr: 611, sha: "091ce499", wave: W1, merged: "2026-07-13"}
  - {id: S-19.04, pr: 639, sha: "d4a23a02", wave: W2, merged: "2026-07-14"}
  - {id: S-19.05, pr: 640, sha: "7b35c8e4", wave: W2, merged: "2026-07-14"}
  - {id: S-19.08, pr: 646, sha: "1304d280", wave: W2, merged: "2026-07-14"}
  - {id: S-19.06, pr: 657, sha: "9787c056", wave: W2, merged: "2026-07-15"}
develop_base: da2f648f
develop_head: 9787c056
---

# E-19 Combined W1+W2 Wave-Gate Report

**Decision:** D-845  
**Date:** 2026-07-15  
**Wave:** E-19 combined W1+W2  
**Stories:** 7 (S-19.01 #613, S-19.02 #610, S-19.03 #611, S-19.04 #639, S-19.05 #640, S-19.08 #646, S-19.06 #657)  
**Develop base → HEAD:** da2f648f → 9787c056  

## Scope Note

This is a combined W1+W2 gate per human direction 2026-07-15 (retroactive scope consolidation). No formal W1 wave gate had run; D-835 governance approval served as W1's exit record. The combined gate covers all 7 E-19 stories that have been merged to develop as of 9787c056.

## Gate Results

GATE_CHECK: gate=1 name=test-suite status=pass note=2045 cargo tests 0 failures + full bats suite green on develop 9787c056 (after D-844 hygiene remediation of 5 live-state defects; 1 perf-baseline timing flake recorded, passed clean re-run)

GATE_CHECK: gate=2 name=dtu-validation status=skip note=dtu_required false; no module-criticality registry; no DTU clones

GATE_CHECK: gate=3 name=adversarial-review status=pass note=0 CRITICAL; 1 HIGH F-WG-001 RESOLVED D-846 grandfather 2026-07-15 (human-directed); 2 LOW routed opportunistic

GATE_CHECK: gate=4 name=demo-evidence status=pass note=7 stories, story-scoped evidence dirs + AC-referenced reports on develop (POLICY 10 re-verified by Gate 3)

GATE_CHECK: gate=5 name=holdout-eval status=skip note=no holdout scenarios authored for this self-referential engine project; holdout-evaluations/ empty; precedented across all E-18/E-19 waves

GATE_CHECK: gate=6 name=state-update status=pass note=sprint-state already terminal (merged status); STATE.md wave-gate record this burst

no facade stories in wave — mutation testing step skipped (all 7 stories tdd_mode: strict; zero mutation_testing_required)

## Gate 1: Test Suite

**Status:** PASS

First run failed with 2 cargo test failures + 4 bats failures. Root causes decomposed:
- **Environment litter:** stale mktemp sinks left by prior test runs caused bats fixture interference; cleaned per D-844 pre-gate hygiene.
- **1 perf-baseline timing flake:** `test_resolver_integration_timing` hit the timing threshold on first run; recorded as timing flake; passed on solo re-run and full clean re-run.
- **3 REAL live-state defects fixed at D-844 (commit f9493f3b):**
  1. `sprint-state.yaml` S-19.06 PC3 done-first ordering violation (terminal prefix ordinal 53 missing → ordinal 129 non-terminal placement)
  2. `derive_wave_id` abort root-caused to same sprint-state ordering defect
  3. `STATE.md` banner `wc -l` value: `~NNN` (D-843 placeholder) → `417` → `420` (accurate post-D-844)

**Full re-run after D-844 remediation:** 2045 cargo tests 0 failures + full bats suite green on develop 9787c056.

## Gate 2: DTU Validation

**Status:** SKIP

`dtu_required: false` in STATE.md frontmatter. No module-criticality registry for this self-referential engine project. No DTU clones authored for any E-19 story. Precedented across all prior E-18 and E-19 waves.

## Gate 3: Adversarial Review

**Status:** PASS (F-WG-001 HIGH RESOLVED D-846 grandfather 2026-07-15)

**Findings summary:** 0 CRITICAL; 1 HIGH (RESOLVED D-846); 2 LOW.

### F-WG-001 HIGH: POLICY 21 timing collision — S-19.01 shell scripts

**Severity:** HIGH  
**Finding:** 5 new `.sh` files shipped in S-19.01 (PR #613 merged 2026-07-13): `bin/check-stale-verdict.sh`, `bin/enforce-merge-strategy.sh`, and 3 test fixture files. POLICY 21 (`no_new_shell_scripts`, blocking severity) was codified at D-836 on 2026-07-13. S-19.01 spec was frozen at v1.18 on 2026-07-11, two days before POLICY 21 was enacted. The PR merged 2026-07-13 post-D-836 enactment.

**Governing rule:** POLICY 21 explicitly grandfathers existing `.sh` scripts "until class-migration E-20" but does NOT contain an explicit grandfather entry for S-19.01's 5 new files. ADR-030 §Decision 2 and §Decision 3 govern the merge-integrity tooling.  

**Architect approval:** ADR-030 §D2/D3 architect-approved (existing architecture decision). The scripts implement merge-strategy enforcement and stale-verdict detection per the ADR.

**CONDITION:** RESOLVED. Human decision 2026-07-15: Option A — grandfather. D-846 POLICY-21-GRANDFATHER-CLAUSE burst (state-manager 2026-07-15): `grandfather_clause` field added to POLICY 21 in `policies.yaml` v1.4.7→v1.4.8, enumerating all 5 S-19.01 files with rationale. Migration anchored to E-20 factory-tools class migration.

**Status:** RESOLVED D-846 (grandfather, human-directed 2026-07-15). Wave gate now unconditionally PASSED. W3 may open.

### F-WG-002 LOW: Telemetry const hygiene

**Severity:** LOW  
**Finding:** Bare string literals `internal.file_not_found` and `plugin.abandoned` used in async telemetry emitters despite named constants existing for synchronous counterparts. `file_not_found` literal duplicated across `read_file` and `read_prefix` modules.  
**Routing:** Candidate opportunistic fix PR (devops-engineer or implementer) or fold into a future architect-story. Does not block wave gate.

### F-WG-003 LOW: plugin.completed missing timestamp field

**Severity:** LOW  
**Finding:** `plugin.completed` telemetry event struct is missing a `timestamp` field that all sibling events (`plugin.invoked`, `plugin.failed`, etc.) carry.  
**Routing:** Candidate opportunistic fix PR or fold into architect story. Does not block wave gate.

### Gate 3 positive attestations

- **Byte-cap consistency:** 262144 (`max_bytes`) consistent across both frontmatter readers; 512KiB structural validators pre-date this wave (Phase-5 question out of scope).
- **Registry anchor count:** 54 anchored filters, zero regressions introduced in this wave.
- **Bundle floor math reconciles:** 34 staged WASMs; orphan-detection gate (S-19.04) operating correctly.
- **Telemetry namespaces:** coherent; no cross-namespace pollution introduced.
- **Squash-merge artifacts:** zero; all 7 story PRs were clean squash-merges.
- **POLICY 10 demo evidence:** AC-referenced reports present in story-scoped evidence dirs on develop (re-verified this gate).
- **POLICY 20:** no new external dependencies introduced without ADR authorization.
- **Release-merge-strategy guard:** `enforce-merge-strategy.sh` / WASM migration (F-WG-001 scope) unexercised until next release cut; noted for E-20.

## Gate 4: Demo Evidence

**Status:** PASS

7 stories with story-scoped evidence directories on develop at HEAD 9787c056. Each merged story has AC-referenced reports confirming behavioral acceptance criteria were exercised. POLICY 10 re-verified by Gate 3 adversarial sweep.

## Gate 5: Holdout Evaluation

**Status:** SKIP

No holdout scenarios have been authored for this self-referential engine project. The `holdout-evaluations/` directory is empty. This skip precedent is established and consistent across all E-18 and E-19 waves.

## Gate 6: State Update

**Status:** PASS

Sprint-state entries for all 7 stories are already in terminal `merged` state. STATE.md wave-gate record written in this burst (D-845). Wave gate telemetry persisted to this report file.

## Pre-Commit Verification

### POLICY 14 4-Index Gate (literal shell)

```
$ grep "^version:" .factory/specs/behavioral-contracts/BC-INDEX.md .factory/specs/verification-properties/VP-INDEX.md .factory/stories/STORY-INDEX.md .factory/specs/architecture/ARCH-INDEX.md
.factory/specs/verification-properties/VP-INDEX.md:version: "2.68"
.factory/specs/architecture/ARCH-INDEX.md:version: "3.01"
.factory/stories/STORY-INDEX.md:version: "4.193"
.factory/specs/behavioral-contracts/BC-INDEX.md:version: "4.05"
```

Result: BC v4.05 / VP v2.68 / STORY v4.193 / ARCH v3.01 — ALL UNCHANGED (wave-gate burst does not bump any index).

### validate-state-structure cargo test (literal shell)

```
$ cargo test -p validate-state-structure --lib
test result: ok. 65 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Result: 65/65 PASS. Banner `wc -l` assertion passes against current STATE.md.

## Summary

E-19 combined W1+W2 wave gate: **PASSED unconditionally**. F-WG-001 HIGH RESOLVED D-846 grandfather (human-directed 2026-07-15).

All 7 stories are merged to develop at HEAD 9787c056. Test suite green. DTU and holdout skipped per standing project policy. Demo evidence present. State updated this burst.

**W3 may open:** F-WG-001 RESOLVED D-846. No blockers remaining. S-19.07 unblocked (depends_on S-19.02+S-19.06 both merged); authorize S-19.07 TDD dispatch.
