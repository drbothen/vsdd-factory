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
  - .factory/cycles/v1.0-feature-engine-discipline-pass-1/adv-cycle-pass-16.md
  - .factory/specs/behavioral-contracts/BC-INDEX.md
  - .factory/specs/verification-properties/VP-INDEX.md
  - .factory/specs/architecture/ARCH-INDEX.md
  - .factory/stories/STORY-INDEX.md
input-hash: "[pending-recompute]"
traces_to: prd.md
project: vsdd-factory
mode: feature
current_step: F5-adversarial-pass-17
current_cycle: v1.0-feature-engine-discipline-pass-1
pass: 17
previous_review: adv-cycle-pass-16.md
prior-pass-classification: MEDIUM
prior-findings-count: 9
verdict: MEDIUM
findings_count: { critical: 0, high: 0, medium: 5, low: 3, nitpick: 1 }
observations: 0
deferred: 0
process_gap_count: 1
convergence_reached: false
---

# Adversarial Review — Pass 17

## Finding ID Convention

Cycle-level finding IDs follow the `F-P[N]-NNN` convention (e.g., `F-P17-001`)
established at cycle open. This convention is distinct from the template `ADV-<CYCLE>-P[N]-SEV-NNN`
format and is preserved as-is per D-387 (structural-correction exception): cycle-level
reviews authored before the ADV format convention may retain their native ID scheme
because retroactive ID renaming would invalidate all cross-references across 16 passes
of audit history. New findings this pass continue the `F-P17-NNN` sequence.

## Part A — Fix Verification (pass >= 2 only)

All pass-16 fixes verified present and correct. D-389 (input-hash placeholder convention) and
D-390 (CHANGELOG→last_amended propagation rule) codified and verified. L-EDP1-009 (7th-layer
L-EDP1-003 sibling sweep dimension enumeration requirement) authored and confirmed in lessons.md.
Merge-date sibling-chain corrected (STATE.md rows 60-61 → 2026-05-10). 5 BCs corrected for
last_amended field per D-390. adv-cycle-pass-12.md current_step quoting removed. factory-artifacts
SHA updated. F-P16-008/009 deferred per adversary recommendation confirmed as acknowledged.

| Finding | Status | Verification |
|---------|--------|--------------|
| F-P16-001 STATE.md merge-date sibling-chain | CLOSED | STATE.md rows 60-61 corrected to 2026-05-10 ✓ |
| F-P16-002 BC last_amended corrections (5 BCs) | CLOSED | BC-4.12.001/003/005/BC-1.13.001/BC-5.39.001 corrected ✓ |
| F-P16-003 7th-layer L-EDP1-003 | CLOSED/DOCUMENTED | L-EDP1-009 authored; D-386 Option C accepted ✓ |
| F-P16-004 input-hash convention | CLOSED | D-389 codified "[pending-recompute]" canonical ✓ |
| F-P16-005 pass-12 current_step quoting | CLOSED | adv-cycle-pass-12.md corrected ✓ |
| F-P16-006 STATE.md SHA stale | CLOSED | factory-artifacts SHA updated 04930af9→9e45d209 ✓ |
| F-P16-007 (no such finding) | N/A | — |
| F-P16-008 timestamp Z pass-8 | DEFERRED | Acknowledged; not fixed per adversary recommendation |
| F-P16-009 timestamp Z pass-9 | DEFERRED | Acknowledged; not fixed per adversary recommendation |
| F-P16-PG1 sweep dimension enumeration | CLOSED | L-EDP1-009 codifies requirement; D-391 required in next burst |
| F-P16-PG2 CHANGELOG→last_amended | CLOSED | D-390 codified ✓ |

## Part B — New Findings (or all findings for pass 1)

### MEDIUM

#### F-P17-001 [MEDIUM]: BC-5.39.002, BC-7.03.091, BC-7.03.092 frontmatter missing `last_amended:` field

**Location:** BC-5.39.002.md (frontmatter), BC-7.03.091.md (frontmatter), BC-7.03.092.md (frontmatter)

**Description:** D-390 codifies the rule that `last_amended:` in BC frontmatter MUST match the
most-recent CHANGELOG row's date. The pass-16 fix burst applied D-390 to 5 BCs (BC-4.12.001,
BC-4.12.003, BC-4.12.005, BC-1.13.001, BC-5.39.001) using an enumeration source that was scoped
to the pass-16 adversary's focused concern — in-cycle feature BCs modified during F4. However,
L-EDP1-009 requires that sweep dimensions be enumerated, and the full project policy rubric lists
additional BCs that were introduced or significantly modified in this cycle: BC-5.39.002,
BC-7.03.091, and BC-7.03.092 all lack the `last_amended:` field entirely. Per D-390, the absence
of the field when a CHANGELOG section exists is a violation — not merely a stale value.

**Recommendation:** Add `last_amended:` to all three BC files, using the most-recent CHANGELOG
row's date. Apply D-391 sibling-pattern sweep using the project policy rubric as enumeration
source. Document enumeration source per D-391 (which must be codified in this burst).

---

### F-P17-002 [MEDIUM]: BC-7.03.091:10 and BC-7.03.092:10 carry `input-hash: "[live-state]"` (D-389 violation)

**Location:** BC-7.03.091.md line 10, BC-7.03.092.md line 10

**Description:** D-389 codifies that BC files introduced or significantly amended in this cycle
MUST use `input-hash: "[pending-recompute]"` as the canonical placeholder — NOT `"[live-state]"`.
BC-7.03.091 and BC-7.03.092 are both in-scope for this cycle (they received version bumps to v1.4
and v1.5 respectively during F5 fix bursts in passes 5-7), placing them firmly within the
"significantly amended in this cycle" scope. Both files still carry `"[live-state]"` on line 10,
which violates D-389 (POL-4: canonical placeholder must be `"[pending-recompute]"` for
cycle-scope BCs).

**Policy Rubric Citation — POL-4 violation:**
> D-389: For in-cycle BCs (introduced or significantly amended during the current feature cycle),
> the canonical input-hash placeholder is `"[pending-recompute]"`. The `"[live-state]"` placeholder
> is acceptable ONLY for brownfield-origin BCs that pre-date this cycle and have NOT been
> significantly amended in-cycle (i.e., only cosmetic/traceability amendments).

BC-7.03.091 received INV-3 addition (F-P3-001), VP/EC population (F-P5-002), and architecture
anchor correction (F-P6-006) — these are substantive amendments. BC-7.03.092 received equivalent
substantive amendments. Both BCs are squarely "significantly amended in this cycle."

**Recommendation:** Change `input-hash: "[live-state]"` to `input-hash: "[pending-recompute]"`
on both files. Run D-391 sibling-pattern sweep: grep all BC files for `input-hash: "[live-state]"`,
filter to the in-cycle-scope BCs (those that received substantive amendments in this cycle), and
correct any additional instances. Document enumeration source.

---

### F-P17-003 [MEDIUM]: L-EDP1-009 lessons.md:337 "enumerated below" is stub; no actual enumeration follows

**Location:** `.factory/cycles/v1.0-feature-engine-discipline-pass-1/lessons.md` line 337 (approximate)

**Description:** The L-EDP1-009 lesson (pass-16 fix burst) contains the row:
```
| 7 (this) | D-389+D-390 | "input-hash convention + CHANGELOG→last_amended rule" | enumerated below |
```
The phrase "enumerated below" in the Layer table's rightmost column refers to the sweep
dimensions checked in the pass-16 burst for the 7th layer. However, no enumeration follows
within L-EDP1-009. The lesson body's Codified Rule section directs future bursts to enumerate
sweep dimensions but does not itself enumerate the dimensions of the pass-16 sweep that was
its own context. This is a self-referential gap: L-EDP1-009 codifies the enumeration requirement
but leaves its own founding burst's enumeration as a stub placeholder.

**Recommendation:** Per D-387 (non-amending corrigendum format), append a corrigendum to
L-EDP1-009 that enumerates the pass-16 sweep dimensions (5 dimensions were documented in
burst-log but not reproduced in the lesson's layer table column). The corrigendum must also note
that the enumeration source was narrower than the project policy rubric (F-P17-001 surfaced the
gap for BC-5.39.002/BC-7.03.091/BC-7.03.092).

---

### F-P17-004 [MEDIUM]: Z-suffix sibling-chain wider than F-P16-008/009 — pass-3..pass-11 (9 files) + BC-INDEX, ARCH-INDEX, VP-INDEX (3 files) all missing `Z` suffix on timestamp

**Location:** adv-cycle-pass-3.md:7, adv-cycle-pass-4.md:7, adv-cycle-pass-5.md:7,
adv-cycle-pass-6.md:7, adv-cycle-pass-7.md:7, adv-cycle-pass-8.md:7, adv-cycle-pass-9.md:7,
adv-cycle-pass-10.md:7, adv-cycle-pass-11.md:7, BC-INDEX.md:7, ARCH-INDEX.md:7

**Description:** Pass-16 acknowledged F-P16-008/009 (timestamp Z suffix missing on pass-8 and
pass-9) as DEFERRED NITPICKs per adversary recommendation. However, grepping the full sibling
chain reveals the Z-suffix gap is systemic: passes 3 through 11 (9 files) all have timestamps
without the Z suffix. Additionally, BC-INDEX.md:7 and ARCH-INDEX.md:7 both lack the Z suffix.
VP-INDEX.md:7 already has the Z suffix (`2026-05-09T18:00:00Z`).

The pass-16 deferred recommendation was correct that pass-8/9 alone were not worth a burst fix.
However, the full 12-site dimension (9 adv-cycle-pass files + 3 index files with 2 missing and 1
already correct) is a schema-uniformity finding that is addressable as a single sweep, which
changes the cost-benefit analysis from the pass-16 two-site scope.

Per D-387 structural-correction exception: frontmatter timestamp field Z-suffix is a schema
uniformity correction, not a retroactive annotation of historical fact. This correction is
permitted per D-387 (structural correction exception) and D-385 sub-rule 2 (immutable row scope
applies to factual body content, not frontmatter schema uniformity fields).

**Recommendation:** Apply Z-suffix to all 12 sites in a single sweep: 9 adv-cycle-pass files
(passes 3-11) + BC-INDEX.md + ARCH-INDEX.md. VP-INDEX.md already has Z; no action needed.
Document enumeration source as file glob per D-391.

---

### F-P17-005 [MEDIUM]: burst-log pass-13 entry still describes pass-13 verdict as MEDIUM

**Location:** `.factory/cycles/v1.0-feature-engine-discipline-pass-1/burst-log.md` line 131 area (pass-13 burst entry)

**Description:** The burst-log pass-13 entry summary reads: "Addressed 1H+1M+1L content findings
+ 3 process-gaps from pass-13 MEDIUM verdict". However, pass-13's verdict was retroactively
reclassified MEDIUM → HIGH via F-P15-005 in the pass-15 fix burst (adv-cycle-pass-13.md
frontmatter corrected). The burst-log pass-13 entry body is immutable per D-385 sub-rule 2 — the
original entry cannot be amended. However, per D-387, a corrigendum line may be appended at the
END of the pass-13 burst entry (before the `---` separator) to flag the reclassification for
readers who encounter the burst-log entry in isolation.

**Recommendation:** Append a D-387/D-385 corrigendum line at the end of the pass-13 burst-log
entry (before the blank line separator that precedes the pass-14 entry):
> `**Corrigendum (pass-17 fix burst — D-387 / F-P17-005):** Pass-13 verdict was retroactively
> reclassified MEDIUM → HIGH via F-P15-005 in pass-15. See pass-15 burst-log entry and
> adv-cycle-pass-13.md:26.`

---

### F-P17-006 [LOW]: STORY-INDEX:7 and ARCH-INDEX:7 `timestamp:` stale (2026-05-09) relative to `last_amended:` (2026-05-11)

**Location:** STORY-INDEX.md:7, ARCH-INDEX.md:7

**Description:** STORY-INDEX.md has `timestamp: 2026-05-09T00:00:00Z` but `last_amended:` field
in its metadata indicates last amendment was 2026-05-11 (v2.65 — F-P6-002/F-P6-004 changes).
ARCH-INDEX.md has `timestamp: 2026-05-09T00:00:00` (also lacking Z suffix per F-P17-004) but
its changelog shows the most recent entry was 2026-05-11 (v1.45). In both cases the `timestamp:`
frontmatter field lags behind the actual last modification date. While `timestamp:` has a nuanced
definition (it can represent the canonical authoring time rather than the last-touched time),
the pattern established across pass-16 fix bursts (e.g., STATE.md updates always refresh
timestamp) suggests these should be updated to reflect the most recent amendment date.

**Recommendation:** Update STORY-INDEX.md:7 timestamp to `2026-05-11T00:00:00Z` and
ARCH-INDEX.md:7 timestamp to `2026-05-11T00:00:00Z`. Apply D-391 sweep: check VP-INDEX.md:7 and
BC-INDEX.md:7 for the same staleness pattern. VP-INDEX.md uses `2026-05-09T18:00:00Z` (its last
amendment may genuinely have been 2026-05-09). BC-INDEX.md uses `2026-05-11T00:00:00` (missing Z;
covered by F-P17-004). Document enumeration source.

---

### F-P17-007 [LOW]: pass-16 burst attestation "7 in-cycle BCs audited" — arithmetic correct but scope source not cited

**Location:** `.factory/cycles/v1.0-feature-engine-discipline-pass-1/burst-log.md` (pass-16 burst entry, Sub-trajectory sibling sweep dimension 2)

**Description:** The pass-16 burst-log attestation for dimension (2) reads: "BC last_amended ↔
CHANGELOG most-recent row: 7 in-cycle BCs audited; 5 corrected; BC-4.12.002/004 already correct ✓"
This was the first application of L-EDP1-009's enumeration requirement. The count (7) is
arithmetically correct but the enumeration SOURCE is not explicitly named. A reader cannot verify
the sweep was complete without knowing how the 7 was derived: was it the policy rubric? A glob?
An index query? D-391 (to be codified this burst) will retroactively close this gap going
forward, but the pass-16 entry itself does not satisfy the forthcoming D-391 standard.

**Recommendation:** Acknowledge in this burst's burst-log that F-P17-007 is CLOSED BY D-391
RETROACTIVELY — D-391 codifies the enumeration source requirement going forward and the pass-16
sweep (while missing the explicit source citation) is reconstructable as "project policy rubric
subsection: 5 BC-4.12.NNN + BC-1.13.001 + BC-5.39.001 = 7". No content fix needed to burst-log
pass-16 entry (immutable per D-385). Document in this burst's burst-log that D-391 closes this
finding prospectively.

---

### F-P17-008 [LOW]: VP-076 lacks `last_amended:` frontmatter field AND `## Changelog` section (uses Lifecycle instead)

**Location:** `.factory/specs/verification-properties/VP-076.md` (frontmatter, no Changelog section)

**Description:** D-390 codifies that `last_amended:` must be populated and must match the
most-recent CHANGELOG row's date. VP-076 does not have a `## Changelog` section — it uses a
`## Lifecycle` table (Event | Date | Actor rows) instead. The `## Lifecycle` table is the standard
VP format (as distinguished from the BC format which uses `## Changelog`). VP-076 currently has
no `last_amended:` frontmatter field at all. The most-recent Lifecycle event is
`v1.3 amended | 2026-05-10 | implementer`.

Per D-390's literal scope, the rule was authored with BC files in mind (CHANGELOG rows →
last_amended propagation). VP files using Lifecycle tables instead of CHANGELOG sections were not
explicitly addressed. This creates an apparent gap: either D-390 must be amended to cover VP
Lifecycle tables, or VP-076's Lifecycle format is accepted as equivalent and a `last_amended:`
field must still be added.

**Recommendation:** (a) Add `last_amended: 2026-05-10` to VP-076 frontmatter (matching v1.3
Lifecycle event). (b) Author D-392 to recognize VP Lifecycle tables as equivalent to BC CHANGELOG
sections for D-390 purposes — `last_amended:` on a VP MUST match the most-recent Lifecycle
event's date.

---

### F-P17-009 [NITPICK]: (positive verification — no action required)

All 5 MEDIUM findings (F-P17-001 through F-P17-005) and 3 LOW findings (F-P17-006 through
F-P17-008) have clear remediation paths. No cascading architectural gaps were identified.
The pattern of L-EDP1-003 lateral recurrence (8th layer surfaced by this pass) continues
to narrow with each iteration — the scope of gaps is decreasing even as the sibling-chain
dimension analysis grows more comprehensive. D-386 Option C (asymptotic convergence acceptance)
remains the appropriate disposition.

---

### F-P17-PG1 [process-gap]: sibling-pattern sweep "dimension extent" enumeration source not codified

**Description:** L-EDP1-009 codifies the requirement to enumerate sweep dimensions but does not
codify HOW to determine the extent (the cardinality) of each dimension. In pass-16, the
"7 in-cycle BCs audited" count was derived from implicit knowledge of the project policy rubric.
A future adversary cannot verify completeness without knowing what source was used to determine
the extent. The gap between "enumerate dimensions" (L-EDP1-009) and "cite the enumeration source
for each dimension" is the residual process gap.

**Recommendation:** Codify D-391: when performing a sibling-pattern sweep under D-383/D-387/D-390,
the fix-burst's burst-log entry MUST cite the enumeration source used to derive the sweep extent
BEFORE stating the cardinality. Valid enumeration sources: (a) project policy rubric; (b) file
glob result with the glob pattern named; (c) BC-INDEX/VP-INDEX/STORY-INDEX query; (d) explicit
per-file enumeration. Burst-log claims of the form "N files audited" without an enumeration
source are NON-COMPLIANT under L-EDP1-009.

---

## Process Gaps

### F-P17-PG1 — sibling-pattern sweep "dimension extent" enumeration source not codified

**Description:** L-EDP1-009 codifies the requirement to enumerate sweep dimensions but does not
codify HOW to determine the extent (the cardinality) of each dimension. In pass-16, the
"7 in-cycle BCs audited" count was derived from implicit knowledge of the project policy rubric.
A future adversary cannot verify completeness without knowing what source was used to determine
the extent. The gap between "enumerate dimensions" (L-EDP1-009) and "cite the enumeration source
for each dimension" is the residual process gap.

**Recommendation:** Codify D-391: when performing a sibling-pattern sweep under D-383/D-387/D-390,
the fix-burst's burst-log entry MUST cite the enumeration source used to derive the sweep extent
BEFORE stating the cardinality. Valid enumeration sources: (a) project policy rubric; (b) file
glob result with the glob pattern named; (c) BC-INDEX/VP-INDEX/STORY-INDEX query; (d) explicit
per-file enumeration. Burst-log claims of the form "N files audited" without an enumeration
source are NON-COMPLIANT under L-EDP1-009.

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 0 |
| MEDIUM | 5 |
| LOW | 3 |
| NITPICK | 1 |
| Process Gaps | 1 |

**Overall Assessment:** pass-with-findings
**Convergence:** FINDINGS_REMAIN — iterate (F-P17-001/002/003/004/005 require fix routing)
**Readiness:** requires fix burst before pass-18 dispatch

## Policy Rubric Verification

| BC | input-hash value | Status |
|----|------------------|--------|
| BC-4.10.001 | `"[pending-recompute]"` | COMPLIANT |
| BC-4.10.002 | `"[pending-recompute]"` | COMPLIANT |
| BC-4.11.001 | `"[pending-recompute]"` | COMPLIANT |
| BC-4.12.001 | `"[pending-recompute]"` | COMPLIANT |
| BC-4.12.002 | `"[pending-recompute]"` | COMPLIANT |
| BC-4.12.003 | `"[pending-recompute]"` | COMPLIANT |
| BC-4.12.004 | `"[pending-recompute]"` | COMPLIANT |
| BC-4.12.005 | `"[pending-recompute]"` | COMPLIANT |
| BC-5.39.001 | `"[pending-recompute]"` | COMPLIANT |
| BC-5.39.002 | `"40a6fb6"` (content hash — no last_amended field) | COMPLIANT (hash present) |
| BC-1.13.001 | `"[pending-recompute]"` | COMPLIANT |
| BC-6.22.001 | `"[pending-recompute]"` | COMPLIANT |
| BC-7.03.091 | `"[live-state]"` | **VIOLATION — F-P17-002** |
| BC-7.03.092 | `"[live-state]"` | **VIOLATION — F-P17-002** |

## Novelty Assessment

This pass is **MEDIUM (lateral from pass-16)**. Trajectory: ...→9→9.

All 8 content findings are residual from the pass-16 fix burst's under-scoped sweep
dimensions — the adversary surfaced no architecturally novel findings. The pattern is
consistent with L-EDP1-003 layer 8: each fix burst closes the dimensions it addressed
while leaving adjacent sibling-chain dimensions for the next pass to find.

The trajectory plateauing at 9→9 indicates the L-EDP1-003 recurrence is entering its
asymptotic boundary per D-386 Option C. The D-391 process-gap codification (if executed
correctly with self-application attestation in this burst's burst-log) is the structural
mechanism most likely to break the plateau.

## Scope Confirmation

Review scope was bounded to: STATE.md, decision-log.md, lessons.md, INDEX.md, burst-log.md,
adv-cycle-pass-16.md, BC-INDEX.md, VP-INDEX.md, ARCH-INDEX.md, STORY-INDEX.md — plus spot
checks of BC frontmatter files (BC-5.39.002, BC-7.03.091, BC-7.03.092, VP-076) surfaced
during dimension analysis. No out-of-scope artifacts were loaded. All 9 findings (8 content
+ 1 PG) are in-scope per the cycle-level review mandate.
