---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-05-11T00:00:00Z
phase: F5
inputs:
  - .factory/STATE.md
  - .factory/cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md
  - .factory/cycles/v1.0-feature-engine-discipline-pass-1/lessons.md
  - .factory/cycles/v1.0-feature-engine-discipline-pass-1/INDEX.md
  - .factory/cycles/v1.0-feature-engine-discipline-pass-1/burst-log.md
  - .factory/cycles/v1.0-feature-engine-discipline-pass-1/adv-cycle-pass-18.md
  - .factory/specs/behavioral-contracts/BC-INDEX.md
  - .factory/specs/verification-properties/VP-INDEX.md
  - .factory/specs/architecture/ARCH-INDEX.md
  - .factory/stories/STORY-INDEX.md
input-hash: "[pending-recompute]"
traces_to: prd.md
project: vsdd-factory
mode: feature
current_step: F5-adversarial-pass-19
current_cycle: v1.0-feature-engine-discipline-pass-1
pass: 19
previous_review: adv-cycle-pass-18.md
prior-pass-classification: HIGH
prior-findings-count: 10
verdict: HIGH
findings_count: { critical: 0, high: 2, medium: 5, low: 3, nitpick: 1 }
observations: 0
deferred: 0
process_gap_count: 2
convergence_reached: false
---

# F5 Adversarial Review — Pass 19

**Cycle:** v1.0-feature-engine-discipline-pass-1
**Pass:** 19
**Prior Pass Verdict:** HIGH (pass-18; 10 findings: 1H+5M+3L+1NIT+1PG)
**This Pass Verdict:** HIGH (sustained; 11 findings: 2H+5M+3L+1NIT + 2 process-gaps)
**Trajectory:** ...→10→**11**

---

## Finding ID Convention

Finding IDs use the format: `F-P<PASS>-<SEQ>` (cycle-internal shorthand consistent with prior passes in this cycle).

---

## Part A — Fix Verification (pass >= 2 only)

| ID | Previous Severity | Status | Notes |
|----|-------------------|--------|-------|
| F-P18-001 (BC last_amended missing: BC-4.10.002, BC-4.11.001, BC-6.22.001) | HIGH | RESOLVED | All 3 BCs carry `last_amended: 2026-05-09` |
| F-P18-002 (VP last_amended missing: VP-069, VP-072, VP-073, VP-075) | MEDIUM | RESOLVED | All 4 VPs carry `last_amended:` fields |
| F-P18-003 (closed by D-393) | MEDIUM | CLOSED | D-393 codified in decision-log |
| F-P18-004 (dispatch-side STATE.md update) | MEDIUM | RESOLVED | STATE.md phase updated pre-dispatch |
| F-P18-005 (arithmetic reconciliation N=12) | MEDIUM | RESOLVED | Canonical Grep yields 12; burst-log dim-1 documents |
| F-P18-006 (closed by D-394) | MEDIUM | CLOSED | D-394 codified in decision-log |
| F-P18-007 (VP-INDEX timestamp T18→T00) | LOW | RESOLVED | VP-INDEX timestamp: 2026-05-09T00:00:00Z |
| F-P18-008 (INDEX.md parentheticals) | LOW | RESOLVED | Convergence Status simplified |
| F-P18-009 (last_amended on BC-INDEX/ARCH-INDEX/VP-INDEX) | LOW | PARTIALLY RESOLVED | BC-INDEX ✓, ARCH-INDEX ✓; VP-INDEX last_amended NOT PRESENT — see F-P19-001 |
| F-P18-010 (NITPICK) | NITPICK | ACKNOWLEDGED | No action per policy |
| F-P18-PG1 (closed by D-393) | process-gap | CLOSED | D-393 codified |

**10th-layer L-EDP1-003 recurrence detected:** Pass-18 dim-3 burst-log attestation claimed "VP-INDEX last_amended added" (Action ✓) but VP-INDEX frontmatter has no `last_amended:` field as of this review. The attestation was a false-true claim. D-393's "second-source Grep query" requirement was applied to the *enumeration cardinality* but NOT to the *per-action file-state verification after application*. This gap is a new defect dimension — file-state grep-back verification — codified here as process gap F-P19-PG1 and resolved via D-395.

---

## Part B — New Findings (or all findings for pass 1)

### HIGH

#### F-P19-001: VP-INDEX.md frontmatter lacks `last_amended:`

- **Severity:** HIGH
- **Category:** spec-fidelity
- **Location:** `.factory/specs/verification-properties/VP-INDEX.md` frontmatter
- **Description:** The pass-18 fix burst dim-3 attestation states "VP-INDEX last_amended added" with a ✓ mark. However, VP-INDEX.md frontmatter contains no `last_amended:` field. `grep -c '^last_amended:' VP-INDEX.md` yields 0. The attestation was a false-true claim — the action was stated as complete but not executed.
- **Evidence:** VP-INDEX.md frontmatter (lines 1-84) has no `last_amended:` key. Pass-18 burst-log dim-3: "Action: VP-INDEX last_amended added ✓" — contradicted by file state.
- **Rule violated:** D-393 (per-action file-state verification required) + D-390 (last_amended propagation mandatory when CHANGELOG amended).
- **Proposed Fix:** Add `last_amended: 2026-05-11` to VP-INDEX.md frontmatter near other dated fields. Run grep-back verification per D-395 (codified this burst): `grep -c '^last_amended:' VP-INDEX.md` → expect 1.

---

#### F-P19-002: STORY-INDEX body table — 5 stories show `draft` despite `status: merged`

- **Severity:** HIGH
- **Category:** spec-fidelity
- **Location:** `.factory/stories/STORY-INDEX.md` lines 520-525 (E-12 body table)
- **Description:** Stories S-12.03, S-12.04, S-12.05, S-12.07, S-12.08 all have `status: merged` in their story frontmatter files. The STORY-INDEX.md body table cells for these 5 stories still show `| draft |` in the Status column. Merge metadata from story frontmatter: S-12.03 PR #120 2026-05-10; S-12.04 PR #121 2026-05-10; S-12.05 PR #119 2026-05-10; S-12.07 PR #122 2026-05-10; S-12.08 PR #123 2026-05-10.
- **Evidence:** `grep -E '\| S-12\.0(3|4|5|7|8)\s*\|.*\| draft \|' STORY-INDEX.md` → 5 matches (expected 0 given merged status).
- **Rule violated:** D-396 (codified this burst, retroactive): story-frontmatter ↔ STORY-INDEX body-table sibling sweep required same-burst when status changes.
- **Proposed Fix:** Update 5 body-table Status cells `draft` → `merged`; add PR # and merge date to Notes column. Verify with `grep -E '\| S-12\.0(3|4|5|7|8)\s*\|.*\| merged \|' STORY-INDEX.md` → expect 5.

---

### MEDIUM

#### F-P19-003: 8 in-cycle VP files lack Z suffix on `timestamp:`

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** VP-069.md, VP-070.md, VP-071.md, VP-072.md, VP-073.md, VP-074.md, VP-075.md, VP-076.md — frontmatter `timestamp:` field
- **Description:** All 8 in-cycle VP source files carry timestamps without Z suffix (e.g., `timestamp: 2026-05-06T00:00:00` instead of `timestamp: 2026-05-06T00:00:00Z`). The pass-17 fix burst dim-3 corrected adv-cycle-pass files and index files but did not extend scope to VP source files.
- **Evidence:** VP-069: `2026-05-06T00:00:00`; VP-070: `2026-05-06T00:00:00`; VP-071: `2026-05-06T00:00:00`; VP-072: `2026-05-06T00:00:00`; VP-073: `2026-05-07T00:00:00`; VP-074: `2026-05-07T00:00:00`; VP-075: `2026-05-07T00:00:00`; VP-076: `2026-05-07T00:00:00`.
- **Proposed Fix:** Append `Z` to all 8 timestamps. Verify: `grep -L 'T[0-9]\{2\}:[0-9]\{2\}:[0-9]\{2\}Z' .factory/specs/verification-properties/VP-{069..076}.md` → expect 0 files returned.

---

#### F-P19-004: STATE.md "Last Updated" narrative stale at pass-17

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** `.factory/STATE.md` line 41 (Last Updated table cell)
- **Description:** The "Last Updated" narrative reads "2026-05-11 — F5 pass-17 fix burst COMPLETE..." while the "Current Phase" cell at line 42 reflects pass-18 state. The Last Updated narrative was not updated during the pass-18 fix burst. STATE.md has an internal inconsistency: narrative vs. Current Phase cell contradict each other.
- **Proposed Fix:** Update Last Updated narrative to reflect pass-18 fix burst completion and trajectory through pass-18.

---

#### F-P19-005: L-EDP1-010 lessons.md Layer-9 row "Same-burst Violation: —" contradicted by F-P19-001

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** `.factory/cycles/v1.0-feature-engine-discipline-pass-1/lessons.md` line 381 (Layer-9 row in L-EDP1-010 layer-history table)
- **Description:** The L-EDP1-010 layer-history table row for Layer-9 (pass-18) reads `| 9 (pass-18) | D-393+D-394 | "independent re-derivation Grep query required" | — |`. The "Same-burst Violation: —" claim is incorrect. F-P19-001 demonstrates pass-18 DID have a same-burst violation: dim-3 falsely attested "VP-INDEX last_amended added ✓" while VP-INDEX had no such field.
- **Proposed Fix:** Append a D-387-format corrigendum at the end of L-EDP1-010 entry correcting the Layer-9 row annotation. The corrigendum should note: "Layer-9 row 'Same-burst Violation: —' is incorrect; F-P19-001 surfaced a false-true attestation at dim-3. See D-395."

---

#### F-P19-006: STATE.md:133 trajectory cardinality ambiguity — "passes 3-18" vs full-cycle trajectory

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** `.factory/STATE.md` line 133 (Concurrent Cycles table Notes cell)
- **Description:** The Notes cell reads "F5 passes 3-18 complete (trajectory 29→15→...→10)". "Passes 3-18" implies 16 F5-only passes, but the trajectory has 18 values (full cycle passes 1-18). A reader counting F5-only passes gets 16; counting trajectory values gets 18. The sub-range "3-18" is the F5 sub-sequence but the trajectory covers the full cycle including passes 1-2 which were not F5 cycle-level adversary passes.
- **Proposed Fix:** Clarify to distinguish F5 sub-sequence from full-cycle: "F5 passes 3-18 complete (16 F5 passes); full-cycle trajectory (pass-1..18): 29→15→11→9→8→7→5→6→6→6→4→3→3→10→13→9→9→10."

---

### LOW

#### F-P19-007: Pass-17 burst-log dim-1 not corrigended re D-393 N=12 correction

- **Severity:** LOW
- **Category:** spec-fidelity
- **Location:** `.factory/cycles/v1.0-feature-engine-discipline-pass-1/burst-log.md` pass-17 entry, Sweep dim-1 attestation
- **Description:** The pass-17 dim-1 attestation states 13 BCs. The pass-18 fix burst (dim-1 attestation) reconciled N=12 and documented this correction in the pass-18 burst-log entry. However, the pass-17 dim-1 attestation was not corrigended with a reference to the N=12 correction. Per D-387, a corrigendum line appended to the burst-log entry is the permitted form for this correction.
- **Proposed Fix:** Append a D-387-format corrigendum to the pass-17 burst-log dim-1 attestation noting N=12 per D-393. The correction already exists in pass-18 burst-log; this corrigendum adds a forward-reference from the erroneous entry.

---

#### F-P19-008: STORY-INDEX last_amended doesn't mention F-P15-004 status:merged event

- **Severity:** LOW
- **Category:** spec-fidelity
- **Location:** `.factory/stories/STORY-INDEX.md` `last_amended:` narrative
- **Description:** The STORY-INDEX `last_amended:` narrative does not explicitly mention the F-P15-004 status:merged propagation event (5 stories retrofitted in pass-15 fix burst). This is addressed by the F-P19-002 fix — STORY-INDEX version bump to v2.66 with new changelog row should cite F-P15-004 propagation completion via D-396.
- **Proposed Fix:** Covered by F-P19-002 STORY-INDEX version bump. New changelog row and `last_amended:` narrative must reference F-P15-004 propagation completion and D-396.

---

#### F-P19-009: VP-INDEX changelog has no 2026-05-10/11 entry despite in-cycle VP edits

- **Severity:** LOW
- **Category:** spec-fidelity
- **Location:** `.factory/specs/verification-properties/VP-INDEX.md` changelog section
- **Description:** The VP-INDEX changelog's most recent entries are dated 2026-05-09. Multiple in-cycle VP housekeeping edits have occurred since then (VP-069/072/073/075 gained `last_amended:` via pass-18 fix; VP-076 gained `last_amended:` via pass-17 fix). No VP-INDEX changelog row records this sweep activity. Per D-390+D-392, VP-INDEX CHANGELOG should reflect VP Lifecycle-equivalent sweep events.
- **Proposed Fix:** Append v1.41 changelog entry covering: `last_amended:` added to VP-INDEX (F-P19-001); Z-suffix added to VP-069..VP-076 (F-P19-003); VP housekeeping sweep (D-390+D-392 application: last_amended fields added to VP-069/072/073/075 per F-P18-002).

---

#### F-P19-010: STATE.md mode:brownfield vs cycle mode:feature asymmetry undocumented

- **Severity:** LOW
- **Category:** spec-fidelity
- **Location:** `.factory/STATE.md` frontmatter `mode:` field
- **Description:** STATE.md frontmatter has `mode: brownfield` while the current cycle (`v1.0-feature-engine-discipline-pass-1`) is a feature cycle. This asymmetry is not documented. A reader seeing `mode: brownfield` alongside feature-mode cycle references may misunderstand the scope of the field. Note: this is a documentation gap, not a factual error — `mode: brownfield` correctly reflects the project-level pipeline mode.
- **Proposed Fix:** No file edit required. Document in burst-log pass-19 entry that STATE.md `mode: brownfield` is intentional (project-level mode), while cycle-level `mode: feature` reflects this cycle's character. The two fields apply at different scopes.

---

### NITPICK

#### F-P19-011: INDEX.md "D-387..D-394 codified" shorthand obscures D-388 separateness

- **Severity:** NITPICK
- **Category:** spec-fidelity
- **Location:** `.factory/cycles/v1.0-feature-engine-discipline-pass-1/INDEX.md` line 79 (Convergence Status prose)
- **Description:** The shorthand "D-387..D-394 codified" uses range notation implying a contiguous homogeneous set. D-388 (forward-reference cycle: convention) is separable from D-387 (structural-correction exception) and D-389..D-394 (sweep discipline rules). Cosmetic only.
- **Proposed Fix:** No action required. The shorthand is acceptable for practitioners familiar with the decision log. Acknowledged here for completeness.

---

## Process Gaps

### F-P19-PG1: D-393 lacks file-state grep-back verification of "Action" claims

- **Finding:** D-393 codified "independent re-derivation Grep query required" for *enumeration cardinality* but does not address *per-action file-state verification after application*. The pass-18 dim-3 attestation listed "Action: VP-INDEX last_amended added ✓" without running a grep to verify the field was actually written post-action. This gap produced F-P19-001.
- **Required action:** Codify D-395: for every "Action: <file> <field> <verb>" item in a burst-log sweep attestation, the agent MUST, AFTER applying the action, re-grep the target file to verify the field appears. Evidence MUST be inlined as `Verification: <grep command> → <expected result> ✓` sibling of the Action claim. Retroactively closes F-P19-PG1.

### F-P19-PG2: Story-frontmatter ↔ STORY-INDEX body-table sweep dimension not enumerated

- **Finding:** No existing decision codifies that when story `status:` frontmatter changes, the STORY-INDEX body-table cell for that story must be updated same-burst. D-382 sibling-file enumeration covers cycle-level files (STATE.md, burst-log, INDEX.md, lessons.md, decision-log), but not STORY-INDEX body-table status sync. This gap produced F-P19-002.
- **Required action:** Codify D-396: when a fix burst changes story frontmatter (`status:`, `merged_at:`, `merged_in:`, `merge_sha:`, `wave:`, `points:`), the burst MUST also sweep STORY-INDEX.md body-table cells for matching story IDs same-burst. Executable query: `grep -E '\| S-[0-9.]+\s*\|.*\| (draft|in-flight|partial) \|' STORY-INDEX.md` cross-referenced against `grep -l 'status: merged' .factory/stories/S-*.md`. Set-difference yields propagation gap. Retroactively closes F-P19-PG2 and F-P19-002.

---

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 2 |
| MEDIUM | 5 |
| LOW | 3 |
| NITPICK | 1 |
| Process Gaps | 2 |

**Overall Assessment:** block — fix burst required
**Convergence:** findings remain — iterate (10th-layer L-EDP1-003 recurrence; F-P19-PG1 surfaces new defect dimension)
**Readiness:** requires revision — pass-20 after fix burst

---

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 19 |
| **New findings** | 6 (F-P19-001: file-state-post-fix dimension; F-P19-002: story-frontmatter↔STORY-INDEX sync; F-P19-003: VP source file Z-suffix scope; F-P19-005: L-EDP1-010 corrigendum meta-layer; F-P19-006: trajectory cardinality ambiguity; F-P19-009: VP-INDEX changelog gap) |
| **Duplicate/variant findings** | 5 (F-P19-004: stale-narrative recurring; F-P19-007: cross-entry corrigendum gap; F-P19-008: last_amended narrative completeness; F-P19-010: documentation gap; F-P19-011: cosmetic) |
| **Novelty score** | 6 / (6 + 5) = 0.545 |
| **Median severity** | 2.5 (MEDIUM) |
| **Trajectory** | 29→15→11→9→8→7→5→6→6→6→4→3→3→10→13→9→9→10→11 |
| **Verdict** | FINDINGS_REMAIN — streak reset to 0/3 |

---

## Policy Rubric Verification

| Policy | Requirement | Compliant in pass-18? |
|--------|-------------|----------------------|
| D-381 | STATE.md updated at fix burst completion | YES |
| D-382 | All 5 sibling files updated | YES |
| D-383 | Intra-file content audit + sibling-pattern sweep | PARTIAL — F-P19-001 (action executed, attestation wrong) |
| D-384 | Self-referential N clause; cardinality cross-check | YES |
| D-385 | Sub-trajectory sibling enumeration; immutable-row scope | YES |
| D-387 | Structural-correction exception protocol | YES |
| D-388 | Forward-reference cycle: convention | YES |
| D-389 | input-hash placeholder convention | YES |
| D-390 | CHANGELOG→last_amended propagation | PARTIAL — F-P19-001 (VP-INDEX last_amended not applied) |
| D-391 | Enumeration-source citation mandatory | YES |
| D-392 | VP Lifecycle ≡ BC CHANGELOG for D-390 | PARTIAL — F-P19-003 (VP source files not swept) |
| D-393 | Independent re-derivation Grep query required | PARTIAL — per-action file-state gap (F-P19-PG1) |
| D-394 | D-391 severity explicit + dispatch-side phase update | YES |

---

## Scope Confirmation

All reviewed artifacts are within cycle scope (v1.0-feature-engine-discipline-pass-1). VP-069..VP-076 confirmed in-cycle via `introduced: v1.0-feature-engine-discipline-pass-1` frontmatter field. S-12.03/04/05/07/08 confirmed in-cycle via story files and `cycle: v1.0-feature-engine-discipline-pass-1` frontmatter. No out-of-cycle artifacts flagged.

**Verdict: HIGH — fix burst required before pass-20 dispatch.**
