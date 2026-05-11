---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-05-11T00:00:00Z
phase: F5
inputs:
  - .factory/specs/architecture/ARCH-INDEX.md
  - .factory/specs/behavioral-contracts/BC-INDEX.md
  - .factory/stories/epics/E-14-engine-discipline-pass-2.md
  - .factory/stories/STORY-INDEX.md
  - .factory/STATE.md
  - .factory/cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md
  - .factory/cycles/v1.0-feature-engine-discipline-pass-1/burst-log.md
input-hash: "[pending-recompute]"
traces_to: prd.md
project: vsdd-factory
mode: feature
current_step: F5-adversarial-pass-8
current_cycle: v1.0-feature-engine-discipline-pass-1
pass: 8
previous_review: adv-cycle-pass-7.md
prior-pass-classification: MEDIUM
prior-findings-count: 5
verdict: MEDIUM
findings_count: { critical: 0, high: 0, medium: 3, low: 2, nitpick: 1 }
observations: 4
deferred: 0
process_gap_count: 3
convergence_reached: false
---

# Adversarial Review — F5 Pass-8 (Cycle-Level, Fresh Context)

Fresh-context audit of the F5 fix burst applied after pass-7 LOW verdict. All 5
F-P7 findings were reported as remediated. This pass verifies those closure claims
and conducts a clean sweep of the sibling-file propagation surface and STATE.md
currency. Verdict: **MEDIUM** — regression from pass-7 LOW.

## Finding ID Convention

Finding IDs in this cycle use the shorthand format `F-P8-NNN` (cycle-scoped pass-8
findings). Formal ADV IDs follow: `ADV-<CYCLE>-P<PASS>-<SEV>-<SEQ>` per the factory
standard. The cycle prefix is omitted per legacy convention established in passes 1–6.

## Part A — Fix Verification (Pass-7 Closure Summary)

| Pass-7 ID | Status | Evidence |
|---|---|---|
| F-P7-001 (MED) — BC-INDEX lines 1782-1783 Capability TBD→CAP-008 | **CLOSED-WITH-CAVEAT** | BC-INDEX v1.63→v1.64 (commit 71e22193): Capability column updated for BC-7.03.091/092. However, L-P20-002 cite-refresh discipline mandates a corresponding ARCH-INDEX bump for every BC-INDEX version change. ARCH-INDEX remains at v1.44 — the ARCH-INDEX body cite and changelog were not updated in the pass-7 fix burst. See F-P8-001. |
| F-P7-002 (MED) — E-14 story_count 5→9; S-14.06/07/08/09 missing from epic | **CLOSED-WITH-CAVEAT** | E-14 v1.1 (71e22193): story_count updated to 9; Stories Planned table updated. However, the forward-ref note added for F-P7-004 incorrectly states "All E-14 stories carry `cycle: v1.0-feature-engine-discipline-pass-2`" — S-14.01 carries pass-1 per D-355-AMEND and D-359. See F-P8-002. |
| F-P7-003 (LOW) — resolver-integration.bats comment + test name cite 3000ms | **CLOSED** | Feature-branch commit 2e6b4372 updated comment and test name to 8000ms. Verified. |
| F-P7-004 (LOW) — cycle forward-ref convention undocumented in E-14 | **CLOSED-WITH-CAVEAT** | Forward-ref note added in E-14 v1.1 (committed as part of F-P7-002 fix). The note was added as part of F-P7-002 fix. However, the note contains a factual error regarding S-14.01's cycle assignment. See F-P8-002. |
| F-P7-005 (LOW) — resolver-integration.bats:594-595 timeout rationale arithmetic | **CLOSED** | Feature-branch commit 2e6b4372 updated comment to accurate arithmetic. Verified. |

**Pass-7 fix burst completeness assessment:** 2 findings CLOSED cleanly; 3 findings CLOSED-WITH-CAVEAT (introduced secondary defects in the corrections themselves). The fix burst was incomplete on sibling-file propagation and introduced a factual error in the E-14 forward-ref note.

## Part B — New Findings

### MEDIUM

#### F-P8-001 [MEDIUM]: ARCH-INDEX cite-refresh discipline (L-P20-002) violated — first missed BC-INDEX bump in 16 consecutive clean apply-burst cycles

- **Severity:** MEDIUM
- **Category:** spec-fidelity / sibling-file propagation gap
- **Location:** `.factory/specs/architecture/ARCH-INDEX.md`
- **Description:** L-P20-002 codifies the BC-INDEX cite-refresh obligation: every BC-INDEX version bump triggers a corresponding ARCH-INDEX version bump and changelog entry in the same burst. The pass-7 fix burst bumped BC-INDEX v1.63→v1.64 (commit 71e22193, Capability TBD→CAP-008 for BC-7.03.091/092) but did not bump ARCH-INDEX and did not add a changelog entry. This is the first L-P20-002 violation in 16 consecutive BC-INDEX bumps (v1.48 through v1.64 were all correctly paired with ARCH-INDEX bumps; v1.64 broke the streak). The ARCH-INDEX body line 174 cite still reads "BC-INDEX v1.64" (last updated correctly in a prior burst), but the changelog does not record the v1.64 bump and the ARCH-INDEX version number did not advance from v1.44 to v1.45.
- **Evidence:** git log shows 71e22193 ("fix(specs): F-P7-001/002/004 — BC-INDEX Capability + E-14 epic update") modified BC-INDEX.md but not ARCH-INDEX.md. ARCH-INDEX.md frontmatter: `version: "1.44"`. ARCH-INDEX.md changelog: latest entry is v1.44 dated 2026-05-09 (cite-refresh from BC-INDEX v1.62→v1.63 in fix-burst-49). No v1.45 entry present. BC-INDEX frontmatter: `version: "1.64"`. The L-P20-002 cite-refresh obligation is documented in every prior ARCH-INDEX changelog entry from v1.25 onward.
- **Proposed Fix:** Bump ARCH-INDEX v1.44→v1.45. Add changelog entry citing L-P20-002 cite-refresh obligation and F-P7-001 as the triggering BC-INDEX bump. Update `last_amended:` to 2026-05-11. This is a factory-artifacts-only fix.

#### F-P8-002 [MEDIUM]: E-14 forward-ref note factual error — claims ALL stories carry pass-2 cycle but S-14.01 carries pass-1 per D-355-AMEND

- **Severity:** MEDIUM
- **Category:** spec-fidelity / documentation accuracy
- **Location:** `.factory/stories/epics/E-14-engine-discipline-pass-2.md:97`
- **Description:** The forward-ref note added in E-14 v1.1 (F-P7-002/F-P7-004 fix burst) states: "All E-14 stories carry `cycle: v1.0-feature-engine-discipline-pass-2` in their frontmatter." This is factually incorrect. S-14.01 (bootstrap convergence-state backfill) carries `cycle: v1.0-feature-engine-discipline-pass-1` because it MUST close before F7 close-out of the current cycle — this is the core sequencing invariant documented in D-355-AMEND and D-359. S-14.01 is not a pass-2 cycle story; it is a pass-1 cycle story that was deferred from the initial 3-story delivery. A fresh agent reading the E-14 epic would believe S-14.01 belongs to pass-2 and omit it from pass-1 F7 close-out, causing CONVERGENCE_STATE_MISSING on the bootstrap stories.
- **Evidence:** E-14 epic line 97 forward-ref note: "All E-14 stories carry `cycle: v1.0-feature-engine-discipline-pass-2`." D-355-AMEND (decision-log): explicitly states S-14.01 must close before F7 close-out of v1.0-feature-engine-discipline-pass-1. D-359 (decision-log): PG-2 inline backfill + S-14.01 as formal story for bootstrap exception. E-14 epic line 46-50 Description section correctly states S-14.01 MUST complete before F7 close-out. The forward-ref note directly contradicts the epic's own Description section.
- **Proposed Fix:** Amend the forward-ref note at E-14 line 97 to correctly state that S-14.01 carries `cycle: v1.0-feature-engine-discipline-pass-1` (the current cycle) while S-14.02 through S-14.09 carry `cycle: v1.0-feature-engine-discipline-pass-2` (forward reference). Bump E-14 v1.1→v1.2, update `last_amended:`. Factory-artifacts fix.

#### F-P8-003 [MEDIUM]: STATE.md stale across 6 passes — phase and current_step still reference F5 pass-3; Session Resume Checkpoint would re-dispatch pass-3

- **Severity:** MEDIUM
- **Category:** state-management / bookkeeping discipline
- **Location:** `.factory/STATE.md` — frontmatter lines 8 and 14; Session Resume Checkpoint section
- **Description:** STATE.md frontmatter `phase:` field reads `engine-discipline-F5-pass-3-adversary-rebaseline`. The `current_step:` reads "Engine-discipline F5 pass-3 — cycle-level adversary re-baseline". The Session Resume Checkpoint section describes next steps starting from pass-3 dispatch. Pass-8 is now dispatching — 6 passes have occurred since STATE.md was last updated on 2026-05-11 (the last update was the F4 COMPLETE / S-12.08 merge record). The Phase Progress table has a row "F5 pass-3 cycle-level adversary re-baseline — **NEXT**" but no rows for passes 3 through 8. A fresh session resuming from STATE.md would re-dispatch F5 pass-3, duplicating all 6 passes of work already completed.
- **Evidence:** STATE.md frontmatter `phase: engine-discipline-F5-pass-3-adversary-rebaseline`. STATE.md `current_step:` describes pass-3 dispatch. Phase Progress table: "F5 pass-3 cycle-level adversary re-baseline — NEXT" (no rows for passes 3-8). Session Resume Checkpoint item 2: "NEXT: Dispatch F5 pass-3 cycle-level adversary fresh-context review." Index versions in Session Resume Checkpoint: "STORY-INDEX v2.64 | ARCH-INDEX v1.44" — STORY-INDEX has been bumped to v2.65 (F-P6 burst registered S-14.06-09) but the checkpoint still shows v2.64.
- **Proposed Fix:** Comprehensive STATE.md update: (1) frontmatter phase → `engine-discipline-F5-pass-8-fix-burst`; (2) current_step → reflects pass-8 fix burst activity; (3) Phase Progress table: update F5 row or add pass 3-8 history rows; (4) Session Resume Checkpoint: replace stale pass-3 checkpoint with accurate pass-8 state including ARCH-INDEX v1.45 and STORY-INDEX v2.65; (5) Index versions: STORY-INDEX v2.64→v2.65. This finding is an instance of the process gap that D-381 will codify.

### LOW

#### F-P8-004 [LOW]: STORY-INDEX `last_amended` field documents v2.64 but frontmatter version is v2.65; the v2.65 bump event is undocumented

- **Severity:** LOW
- **Category:** bookkeeping / audit trail
- **Location:** `.factory/stories/STORY-INDEX.md` — frontmatter `last_amended:` field
- **Description:** STORY-INDEX frontmatter `version: "2.65"` but `last_amended: "2026-05-09 (v2.64)"`. The v2.65 bump event (S-14.06/07/08/09 registration in the F-P6 fix burst) is undocumented in the `last_amended:` field. The field exists to provide a human-readable audit trail for version bumps; when it describes a prior version, the audit trail has a gap. A reader inspecting STORY-INDEX sees v2.65 in the version field but can only find the record of v2.64 in `last_amended:` — no documentation of what changed from v2.64 to v2.65.
- **Evidence:** STORY-INDEX frontmatter: `version: "2.65"` (line 4) + `last_amended: "2026-05-09 (v2.64) — F5 fix-burst-41..."` (line 8). The v2.65 bump event (S-14.06, S-14.07, S-14.08, S-14.09 registration) is not recorded in `last_amended:`.
- **Proposed Fix:** Update `last_amended:` to document the v2.65 bump: "2026-05-11 (v2.65) — F-P6-002 + F-P6-004 (F-P6 fix burst): registered S-14.06 (BC-4.12.001 INV cleanup), S-14.07 (HOST_ABI resolver-input error-response field), S-14.08 (resolver.loaded plugin.log observability), and S-14.09 (forensic marker cleanup). Closes F-P6-002 and F-P6-004." Factory-artifacts fix.

#### F-P8-005 [LOW]: Cycle burst-log silent across passes 3-7 — no entries for 5 consecutive fix bursts

- **Severity:** LOW
- **Category:** bookkeeping / audit trail completeness
- **Location:** `.factory/cycles/v1.0-feature-engine-discipline-pass-1/burst-log.md`
- **Description:** The cycle burst-log contains one entry (rc.14 session checkpoint archive). Passes 3 through 7 each had a fix burst — collectively 12 factory-artifacts commits and 7 feature-branch commits — with no burst-log entries. The burst-log is the cycle's historical narrative. Agents resuming the cycle (fresh session) must reconstruct fix-burst history from git log or review files; there is no human-readable narrative of what each burst addressed, what commits it produced, and what decisions it triggered. This is the same documentation gap that causes downstream ambiguity about whether past passes were correctly executed.
- **Evidence:** burst-log.md has 1 entry ("rc.14 session checkpoint archive"). Pass-3 fix burst: commits c5b110ab, 63be1033, d1251864, 2e00637c (feature) + 2bac730e, d850973d (factory-artifacts). Pass-4 fix burst: commits cec5ae31, 8776d391, fd27f818, b24e3125, 9bc06826, c7e0bf42 (feature) + f5646dc2, fde954f1 (factory-artifacts). Pass-5 fix burst: commits 38ca02f2 (feature) + c6cbec15, e4541f3c (factory-artifacts). Pass-6 fix burst: commits 349c1d8e, ae4778c4 (feature) + 219660d5, 1fa8efcd (factory-artifacts). Pass-7 fix burst: commits 2e6b4372 (feature) + 5f26d1b0, 71e22193 (factory-artifacts). None of these are recorded in burst-log.md.
- **Proposed Fix:** Append burst-log entries for passes 3-7 fix bursts. Each entry should record: pass number, date, summary of fixes applied, and commit SHAs. Factory-artifacts fix.

### NITPICK

#### F-P8-006 [NIT]: BC-INDEX v1.64 changelog line-number citation has drifted — references 1782-1783 but lines shifted to 1784-1785

- **Severity:** NITPICK
- **Category:** documentation accuracy / future-proofing
- **Location:** `.factory/specs/behavioral-contracts/BC-INDEX.md` — v1.64 changelog entry
- **Description:** The BC-INDEX v1.64 changelog entry cites "BC-INDEX:1782-1783" as the location of the updated Capability cells for BC-7.03.091/092. Line numbers in large index files drift whenever rows are inserted above the cited location. The current file may have shifted these rows. Line-number citations in changelog entries become stale references as the file grows. The more robust approach is to cite by BC ID rather than by line number ("BC-7.03.091 + BC-7.03.092 Capability column TBD→CAP-008"), which is stable regardless of file growth.
- **Evidence:** BC-INDEX v1.64 changelog: "BC-INDEX:1782-1783 Capability column TBD→CAP-008 for BC-7.03.091 (warn-pending-wave-gate: identity & registry binding) and BC-7.03.092". Line numbers in a 1947-BC index file are inherently fragile references.
- **Proposed Fix:** Option (b) — update the v1.64 changelog citation to use BC IDs rather than line numbers: "BC-7.03.091 + BC-7.03.092 Capability column TBD→CAP-008." More stable; avoids the same drift class that has triggered false findings in prior passes. Factory-artifacts fix.

## Observations

- **F-P8-007 [REGRESSION ANALYSIS]:** Pass-7 verdict was LOW (first pass below CRITICAL after 6 consecutive CRITICAL passes). Pass-8 verdict is MEDIUM — a regression. The regression is entirely attributable to the pass-7 fix burst's incomplete sibling-file propagation: BC-INDEX was updated but ARCH-INDEX cite-refresh was missed (F-P8-001), and the E-14 forward-ref note introduced a new factual error (F-P8-002). The STATE.md staleness (F-P8-003) predates pass-7 but was not caught in prior passes because previous adversary reviews did not include STATE.md in their input files. Including STATE.md explicitly in this pass's input list is what surfaced F-P8-003.

- **F-P8-008 [process-gap]:** D-379 (CI-green-signal rule) was codified for CRITICAL CI-class findings. No equivalent rule exists for sibling-file propagation discipline in fix bursts. L-P20-002 documents the ARCH-INDEX cite-refresh obligation but is not enforced by any hook or pre-commit check. Two consecutive passes (pass-7 and pass-8) have had sibling-file propagation gaps in fix bursts. Recommend S-15.03 scope expansion to include automated detection of BC-INDEX version bumps that lack a corresponding ARCH-INDEX changelog entry in the same commit.

- **F-P8-009 [process-gap]:** Adversary fix-burst STATE.md update is not codified as a mandatory obligation. D-381 (proposed in this pass) addresses this gap. Until D-381 is adopted, STATE.md will remain stale after every fix burst that lacks an explicit state-manager dispatch. F-P8-003 would not have occurred if STATE.md had been updated after each of passes 3-7. The fix-burst protocol should include a mandatory final step: "STATE.md updated: phase, current_step, Phase Progress row, Session Resume Checkpoint."

- **F-P8-010 [positive]:** Despite the regression verdict, the fix burst correctly closed 2 of 5 pass-7 findings (F-P7-003, F-P7-005) in the feature branch. The feature-branch changes (commit 2e6b4372) show appropriate scope discipline — only cosmetic test-file changes, no production code changes, no new forensic markers. The regression is confined to factory-artifacts bookkeeping failures, not to feature-branch quality.

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 0 |
| MEDIUM | 3 |
| LOW | 2 |
| NITPICK | 1 |
| Process-gap | 3 |

**Overall Assessment:** MEDIUM — regression from pass-7 LOW. Pass-7 fix burst was incomplete on sibling-file propagation and introduced a factual error.
**Convergence:** findings remain — iterate
**Readiness:** requires revision (factory-artifacts fixes only; no feature-branch changes required)

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 8 |
| **New findings** | 6 |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 1.0 (6/6) |
| **Median severity** | MEDIUM |
| **Trajectory** | 29→11→9→9→8→7→5→**6** |
| **Verdict** | FINDINGS_REMAIN |

## Convergence Trajectory

| Pass | Classification | Critical | High | Medium | Low | NIT | Total |
|------|---------------|----------|------|--------|-----|-----|-------|
| P1 | CRITICAL | 2 | 10 | 12 | 5 | 0 | 29 |
| P2 | CRITICAL | 2 | 4 | 5 | 0 | 0 | 11 |
| P3 | CRITICAL | 2 | 4 | 3 | 0 | 0 | 9 |
| P4 | CRITICAL | 2 | 4 | 3 | 0 | 0 | 9 |
| P5 | CRITICAL | 1 | 3 | 3 | 1 | 0 | 8 |
| P6 | CRITICAL | 2 | 3 | 2 | 0 | 0 | 7 |
| P7 | LOW | 0 | 0 | 2 | 3 | 0 | 5 |
| P8 | MEDIUM | 0 | 0 | 3 | 2 | 1 | **6** |

Severity floor regressed from LOW (P7) back to MEDIUM (P8). The regression is attributable to fix-burst incompleteness (sibling-file propagation miss + factual error in correction) and STATE.md staleness accumulated over 6 passes. Streak: 0/3 for cycle convergence.

## Top 5 Most Important Findings (F5 pass-8 fix burst drivers)

1. **MEDIUM F-P8-001** — ARCH-INDEX cite-refresh (L-P20-002) violated; bump v1.44→v1.45 with changelog entry; factory-artifacts fix
2. **MEDIUM F-P8-002** — E-14 forward-ref note factual error: "ALL stories → pass-2" must exclude S-14.01 (pass-1 per D-355-AMEND); factory-artifacts fix
3. **MEDIUM F-P8-003** — STATE.md stale 6 passes; comprehensive update: phase, current_step, Phase Progress, Session Resume Checkpoint; factory-artifacts fix
4. **LOW F-P8-004** — STORY-INDEX last_amended documents v2.64 but version is v2.65; document the v2.65 bump event; factory-artifacts fix
5. **LOW F-P8-005** — Cycle burst-log has no entries for passes 3-7 fix bursts; append narrative entries for each burst; factory-artifacts fix

## Recommendation

Continue F5. Pass-9 target: LOW or NITPICK_ONLY (reset streak). All 6 findings are factory-artifacts-only fixes — no feature-branch changes required. Apply F-P8-001..006 fixes in a single burst. Codify D-381 (adversary fix-burst MUST update STATE.md) in decision-log in the same burst.

`convergence_reached`: false. Verdict MEDIUM. Regression from P7 LOW. Need 3 consecutive NITPICK_ONLY cycle-level passes for cycle convergence. Streak counter reset to 0.

## Process-Gap Findings (3)

F-P8-008 (L-P20-002 cite-refresh discipline lacks automation — S-15.03 scope expansion candidate), F-P8-009 (adversary fix-burst STATE.md update not codified — addressed by D-381 in this same fix burst), F-P8-010 suppressed (positive observation, not a gap).
