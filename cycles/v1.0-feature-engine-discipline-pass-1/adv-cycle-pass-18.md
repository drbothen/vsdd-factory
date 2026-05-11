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
  - .factory/cycles/v1.0-feature-engine-discipline-pass-1/adv-cycle-pass-17.md
  - .factory/specs/behavioral-contracts/BC-INDEX.md
  - .factory/specs/verification-properties/VP-INDEX.md
  - .factory/specs/architecture/ARCH-INDEX.md
  - .factory/stories/STORY-INDEX.md
input-hash: "[pending-recompute]"
traces_to: prd.md
project: vsdd-factory
mode: feature
current_step: F5-adversarial-pass-18
current_cycle: v1.0-feature-engine-discipline-pass-1
pass: 18
previous_review: adv-cycle-pass-17.md
prior-pass-classification: MEDIUM
prior-findings-count: 9
verdict: HIGH
findings_count: { critical: 0, high: 1, medium: 5, low: 3, nitpick: 1 }
observations: 0
deferred: 0
process_gap_count: 1
convergence_reached: false
---

# Adversarial Review: vsdd-factory engine-discipline cycle (Pass 18)

**Verdict:** HIGH (regression from pass-17 MEDIUM)
**Trajectory:** 29→15→11→9→8→7→5→6→6→6→4→3→3→10→13→9→9→**10**

## Finding ID Convention

Finding IDs for this cycle-level review use the format: `F-P18-NNN` (consistent with prior
passes in this cycle). The `ADV-CYCLE-PPASS-SEV-SEQ` format applies to per-story adversary
reviews; cycle-level reviews use the F-PNN-NNN shorthand established at pass-1.

## Part A — Fix Verification (pass >= 2 only)

| ID | Previous Severity | Status | Notes |
|----|-------------------|--------|-------|
| F-P17-001 (3 BCs missing last_amended) | MEDIUM | VERIFIED FIXED | BC-5.39.002 last_amended:2026-05-09; BC-7.03.091 last_amended:2026-05-10; BC-7.03.092 last_amended:2026-05-10 present in burst-log Commit C attestation |
| F-P17-002 (input-hash [live-state] on BC-7.03.091/092) | MEDIUM | VERIFIED FIXED | [live-state]→[pending-recompute] per D-389; Commit C |
| F-P17-003 (L-EDP1-009 corrigendum) | MEDIUM | VERIFIED FIXED | Corrigendum appended to lessons.md (Commit B); layer-7 enumeration documented |
| F-P17-004 (Z-suffix sweep: 12 sites) | MEDIUM | VERIFIED FIXED | 9 adv-pass + BC-INDEX + ARCH-INDEX corrected; Commit D |
| F-P17-005 (burst-log pass-13 corrigendum MEDIUM→HIGH) | MEDIUM | VERIFIED FIXED | Corrigendum appended per D-387 format; Commit D |
| F-P17-006 (STORY-INDEX/ARCH-INDEX timestamp) | LOW | VERIFIED FIXED | Both →2026-05-11T00:00:00Z; Commit D |
| F-P17-007 (D-391 codification) | LOW | VERIFIED FIXED | D-391 codified in decision-log (Commit B); retroactively closes PG1 |
| F-P17-008 (VP-076 last_amended) | LOW | VERIFIED FIXED | VP-076 last_amended:2026-05-10 added (Commit C) |
| F-P17-009 (positive verification) | NITPICK | N/A | Positive finding; no action required |
| F-P17-PG1 (enumeration source requirement) | PROCESS-GAP | VERIFIED CLOSED | D-391 codified |

Pass-17 verification: 9/9 findings resolved. No regressions from pass-16 verified.

## Part B — New Findings (or all findings for pass 1)

### HIGH

#### F-P18-001 [HIGH] — 3 in-cycle BCs missing `last_amended:`

- **Severity:** HIGH
- **Category:** spec-fidelity
- **Location:** BC-4.10.002.md, BC-4.11.001.md, BC-6.22.001.md (frontmatter)
- **Description:** Three in-cycle BCs received CHANGELOG amendments but are missing the
  `last_amended:` frontmatter field required by D-390. Pass-17 burst-log dim-1 attestation
  claimed all 13 in-cycle BCs were audited, but the inlined enumeration list was authored
  by the same agent claiming sweep completeness — no independent second-source query was run.
  Independent re-derivation via `grep -rl '^introduced: v1.0-feature-engine-discipline-pass-1'`
  yields 12 BCs (not 13); the 3 missing-field BCs are in that 12-BC canonical set.
- **Evidence:**
  - BC-4.10.002.md: version 1.2, CHANGELOG row `1.2 | 2026-05-09`; no `last_amended:` in frontmatter
  - BC-4.11.001.md: version 1.3, CHANGELOG row `1.3 | 2026-05-09`; no `last_amended:` in frontmatter
  - BC-6.22.001.md: version 1.1, CHANGELOG row `1.1 | 2026-05-09`; no `last_amended:` in frontmatter
- **Proposed Fix:** Add `last_amended: 2026-05-09` to each of the 3 BC files. Run canonical
  Grep (`grep -rl '^introduced: v1.0-feature-engine-discipline-pass-1' .factory/specs/behavioral-contracts/`)
  and verify the full 12-BC in-cycle list — all must have `last_amended:`.

### MEDIUM

#### F-P18-002 [MEDIUM] — 3 in-cycle VPs missing `last_amended:`

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** VP-072.md, VP-073.md, VP-075.md (frontmatter)
- **Description:** Three in-cycle VPs lack `last_amended:` field required by D-392
  (VP Lifecycle table ≡ BC CHANGELOG for D-390 purposes). Pass-17 dim-5 attestation
  stated "VP-069..VP-075 — all have last_amended or Lifecycle event consistent" but
  VP-072/073/075 do not have `last_amended:` in frontmatter. The attestation lacked
  an independent second-source query (non-compliant under D-391/D-393).
- **Evidence:**
  - VP-072.md: Lifecycle Created 2026-05-06; no `last_amended:` field
  - VP-073.md: Lifecycle Created 2026-05-07; no `last_amended:` field
  - VP-075.md: Lifecycle Created 2026-05-07; no `last_amended:` field
  - VP-069.md: Lifecycle Created 2026-05-06; also missing `last_amended:` (4th gap)
- **Proposed Fix:** Add `last_amended:` to VP-072 (2026-05-06), VP-073 (2026-05-07),
  VP-075 (2026-05-07). Verify VP-069 per D-392 (Created = last_amended = 2026-05-06).
  Run independent Grep: `grep -rl '^introduced: v1.0-feature-engine-discipline-pass-1'
  .factory/specs/verification-properties/`.

#### F-P18-003 [MEDIUM] — D-391 enumeration-source citation lacks executable query

- **Severity:** MEDIUM
- **Category:** missing-edge-cases
- **Location:** burst-log.md pass-17 dim-1 attestation; decision-log.md D-391
- **Description:** D-391 requires that sweep attestations cite an enumeration source.
  The pass-17 dim-1 used "project policy rubric" as the source without providing an
  executable Grep command, file glob, or index query. An independent reader cannot verify
  the enumeration without knowing which rubric section applies. F-P18-001 is the direct
  consequence: the inlined list was wrong (3 BCs missing) because no second-source query
  confirmed cardinality.
- **Proposed Fix:** D-393 should codify that the enumeration source MUST be an executable
  query (Grep/glob/jq) that yields the same cardinality as the inlined list. Burst-log
  attestations must record: (a) inlined per-file list, (b) second-source query, (c)
  arithmetic |list| == |query result|.

#### F-P18-004 [MEDIUM] — STATE.md `phase:` field stale at pass-18 dispatch

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** STATE.md frontmatter line 8
- **Description:** At pass-18 dispatch time, STATE.md `phase:` reads
  `engine-discipline-F5-pass-17` — reflecting the completed pass-17 state, not the
  in-progress pass-18. D-381 requires STATE.md to reflect the current step.
  The dispatch-side update obligation (before adversary returns) was not previously
  explicit in D-381's text.
- **Proposed Fix:** Update STATE.md `phase:` to `engine-discipline-F5-pass-18`. Codify
  in D-394 that dispatch-side STATE.md update is mandatory BEFORE adversary returns review.

#### F-P18-005 [MEDIUM] — Arithmetic disagreement: "9 in-cycle BCs" vs "13 BCs"

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** lessons.md L-EDP1-009 corrigendum (~line 352); burst-log pass-17 dim-1
- **Description:** lessons.md corrigendum cites "9 in-cycle BCs" as the pass-16
  enumeration basis. burst-log pass-17 dim-1 cites "13 BCs." Neither count matches the
  canonical Grep result of 12 BCs via `introduced:` field. All three counts conflict;
  the canonical Grep result supersedes both.
- **Proposed Fix:** Run canonical Grep to establish N=12. Append D-387 corrigenda to
  lessons.md L-EDP1-009 and burst-log pass-17 dim-1 attestation citing the Grep query
  and correcting the count. Per D-393, the correction must include the second-source query.

#### F-P18-006 [MEDIUM] — D-391 lacks explicit severity classification for non-compliance

- **Severity:** MEDIUM
- **Category:** ambiguous-language
- **Location:** decision-log.md D-391 row
- **Description:** D-390 explicitly states "Violations of this rule are MEDIUM severity."
  D-391 — which forms a rule pair with D-390 — does not state a severity for violations.
  An adversary cannot classify a D-391 non-compliance without inferring from D-390 by
  analogy.
- **Proposed Fix:** D-394 should explicitly state D-391 violations are MEDIUM severity,
  parallel to D-390's explicit classification.

### LOW

#### F-P18-007 [LOW] — VP-INDEX.md timestamp hour-precision asymmetry

- **Severity:** LOW
- **Category:** spec-fidelity
- **Location:** VP-INDEX.md frontmatter line 7
- **Description:** VP-INDEX `timestamp: 2026-05-09T18:00:00Z` uses hour precision.
  BC-INDEX, ARCH-INDEX, and STORY-INDEX all use day precision (`T00:00:00Z`). Schema
  asymmetry between sibling index files.
- **Proposed Fix:** Change VP-INDEX.md timestamp to `2026-05-09T00:00:00Z`.

#### F-P18-008 [LOW] — INDEX.md Convergence Status trajectory parentheticals asymmetry

- **Severity:** LOW
- **Category:** ambiguous-language
- **Location:** INDEX.md line 78 (F5 Convergence Status row)
- **Description:** The trajectory string in the Convergence Status row embeds parenthetical
  annotations `(content-only; P7=MEDIUM/P9=HIGH/P13=HIGH per D-387; P12 restated per
  F-P13-002)`. The per-row Verdict cells in the Adversarial Reviews table are the
  source-of-truth for each pass verdict. The parenthetical creates ambiguity about whether
  the trajectory values are correct.
- **Proposed Fix:** Simplify to the pure numeric sequence
  `29→15→11→9→8→7→5→6→6→6→4→3→3→10→13→9→9→10` with no inline parentheticals.
  Verdict classifications are fully documented in the per-row Verdict column.

#### F-P18-009 [LOW] — BC-INDEX lacks frontmatter `last_amended:` field

- **Severity:** LOW
- **Category:** spec-fidelity
- **Location:** BC-INDEX.md, ARCH-INDEX.md, VP-INDEX.md frontmatter
- **Description:** STORY-INDEX has `last_amended:` in its body. BC-INDEX, ARCH-INDEX, and
  VP-INDEX do not have `last_amended:` frontmatter field, creating schema asymmetry across
  sibling index files.
- **Proposed Fix (Option A — preferred):** Add `last_amended: 2026-05-11` to BC-INDEX.md
  and ARCH-INDEX.md frontmatter; add `last_amended: 2026-05-09` to VP-INDEX.md frontmatter
  (matching most-recent changelog entry dates).

### NITPICK

#### F-P18-010 [NITPICK] — `pass:` + `previous_review:` frontmatter redundancy

- **Severity:** NITPICK
- **Category:** ambiguous-language
- **Location:** adv-cycle-pass-17.md and adv-cycle-pass-18.md frontmatter
- **Description:** The `pass:` and `previous_review:` frontmatter fields are redundant:
  `previous_review` can be mechanically derived from `pass - 1`. The redundancy is benign
  and provides human-readable cross-reference convenience.
- **Proposed Fix:** No action. Note for S-14.03 (input-hash automation) that schema
  normalization MAY simplify these fields.

## Process Gaps

### F-P18-PG1 [process-gap] — D-391 self-application audit lacks independent re-derivation

D-391 codified that sweep attestations must cite an enumeration source. The pass-17 burst
applied D-391 self-application (mandatory per D-391 own text) but used "project policy rubric"
as the source without providing an executable query, query result count, or arithmetic
confirmation that |inlined list| == |query result|. F-P18-001 and F-P18-002 are direct
consequences: 3 BCs and at minimum 3 VPs were missing from the sweep despite the attestation
claiming completeness.

**Codification required:** D-393 must require that the enumeration source in any sweep
attestation be an executable query that an independent reader can run to verify cardinality.
The burst-log attestation must record: (1) inlined per-file list, (2) second-source
Grep/glob/jq query, (3) arithmetic |list| == |query result|. Discrepancies block
sweep-completion claim.

**Meta-pattern:** This is the 9th-layer recurrence of L-EDP1-003 (recursive discipline
violation). D-391 was codified by pass-17 and violated within the same burst that codified
it. L-EDP1-010 should document this.

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 1 |
| MEDIUM | 5 |
| LOW | 3 |
| NITPICK | 1 |
| PROCESS-GAP | 1 |

**Overall Assessment:** block — HIGH finding F-P18-001 requires fix before next pass
**Convergence:** FINDINGS_REMAIN — 10 content findings + 1 process gap; HIGH verdict; streak reset to 0/3
**Readiness:** requires fix burst before pass-19 dispatch

## Policy Rubric Verification

Independent re-derivation check (D-391 framework; D-393 self-application):

**BC `last_amended` sweep:**
- Enumeration source: `grep -rl '^introduced: v1.0-feature-engine-discipline-pass-1' .factory/specs/behavioral-contracts/`
- Query result: 12 BCs (BC-1.13.001, BC-4.10.001/002, BC-4.11.001, BC-4.12.001-005, BC-5.39.001/002, BC-6.22.001)
- Pass-17 dim-1 inlined count: 13 (included BC-7.03.091/092 which are brownfield-origin)
- Arithmetic: |query 12| ≠ |inlined 13| — **MISMATCH → F-P18-001/F-P18-005**
- BCs missing `last_amended:` from the 12-BC canonical set: BC-4.10.002, BC-4.11.001, BC-6.22.001 (3 gaps)

**VP `last_amended` sweep:**
- Enumeration source: `grep -rl '^introduced: v1.0-feature-engine-discipline-pass-1' .factory/specs/verification-properties/`
- Query result: 8 VPs (VP-069..VP-076)
- Pass-17 dim-5 inlined count: 8 (VP-069..VP-076) — count matches, but no field-level check per VP was performed
- VPs missing `last_amended:` from the 8-VP canonical set: VP-069, VP-072, VP-073, VP-075 (4 gaps)
- Pass-17 claimed 0 gaps — **MISMATCH → F-P18-002**

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 18 |
| **New findings** | 10 |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 1.0 (10 / (10 + 0)) |
| **Median severity** | MEDIUM (1H+5M+3L+1NIT) |
| **Trajectory** | 29→15→11→9→8→7→5→6→6→6→4→3→3→10→13→9→9→10 |
| **Verdict** | FINDINGS_REMAIN |

All 10 content findings are novel vs passes 1-17. F-P18-001/002 surfaced gaps in the
pass-17 dim-1/dim-5 attestations that were explicitly claimed as complete. F-P18-003
operationalizes D-391 with a second-source query requirement not previously codified.
F-P18-004/006 close ambiguities in D-381/D-391. F-P18-007/008/009 address schema
asymmetries across index files. F-P18-010 is a benign observation.

Trajectory regression 9→10 is the HIGH verdict trigger (F-P18-001 surfaced 3 previously
unclaimed BC gaps). Per D-386 Option C, no structural escalation; continue F5 with fix burst.

## Scope Confirmation

Review scope was bounded to: STATE.md, decision-log.md (D-336..D-392), lessons.md
(including L-EDP1-009 corrigendum), INDEX.md, burst-log.md (pass-17 dim-1 and dim-5
attestations), adv-cycle-pass-17.md, BC-INDEX.md, VP-INDEX.md, ARCH-INDEX.md,
STORY-INDEX.md — plus spot checks of 3 BC frontmatter files (BC-4.10.002, BC-4.11.001,
BC-6.22.001) and 8 in-cycle VP files (VP-069..VP-076). No out-of-scope artifacts loaded.
All 10 content findings + 1 PG are within the cycle-level factory artifact governance scope.
