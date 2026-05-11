---
document_type: cycle-lessons
cycle: v1.0-feature-engine-discipline-pass-1
producer: state-manager
version: "1.0"
created: 2026-05-11
last_updated: 2026-05-11
---

# Lessons Learned — engine-discipline cycle (v1.0-feature-engine-discipline-pass-1)

> F-P9-004 backfill: this file was absent for the first 9 adversary passes.
> Lessons are reconstructed from adv-cycle-pass-1.md through adv-cycle-pass-9.md
> and from SESSION-CHECKPOINT.md. Per state-manager.md line 136 and D-382, all
> future lessons must be appended here when identified.

---

## L-EDP1-001 — Same-class-defect recurrence under fix-burst pressure (CI false-green chain)

**Source:** F-P3-001, F-P4-002, F-P5-001, F-P6-001 (CRITICAL, recurring)
**Date codified:** 2026-05-10

**Pattern:** Across F-P2-001 → F-P3-001 → F-P4-002 → F-P5-001 → F-P6-001, five consecutive
fix bursts introduced new CI false-green defects of the same class while attempting to
close the prior pass's CI false-green finding. Each fix burst declared closure based on
local-only validation; CI was not re-run on a PR branch before closure was recorded.

**Root cause:** No enforcement gate required CI evidence before recording a CI-class finding
as CLOSED. Fix bursts relied on implementer declaration alone.

**Codification:** D-379 (CI-green-signal rule for CRITICAL CI-class closures; authored
2026-05-10 in pass-5 advisory, codified with enforcement teeth in pass-6 fix burst).
Initial application: D-380 (F-P6-001 closure with CI run URL 25651192161 showing both
macos-latest and ubuntu-latest PASS). Broke the 5-pass chain.

**Status:** Codified as prose rule (D-379). Automated enforcement pending S-15.03.

---

## L-EDP1-002 — Sibling-file discipline gap recurrence under fix-burst pressure (cite-refresh + STATE.md + burst-log + INDEX.md)

**Source:** F-P8-001 (MEDIUM), F-P8-003 (MEDIUM), F-P9-001 (HIGH)
**Date codified:** 2026-05-11

**Pattern:** L-P20-002 (codified in plugin-async-semantics cycle) requires ARCH-INDEX
cite-refresh on child-index version bumps. The pass-7 fix burst (closing F-P5-002 /
F-P6-005) bumped BC-INDEX v1.63→v1.64 but missed ARCH-INDEX — the first L-P20-002
violation in 16 consecutive clean cite-refreshes (F-P8-001).

The pass-8 fix burst then codified D-381 (STATE.md mandatory in every fix burst) and
correctly updated STATE.md, but missed burst-log.md AND INDEX.md in that same burst
(F-P9-001 HIGH). D-381's scope was limited to STATE.md, which allowed the burst to
simultaneously comply with D-381 and violate the broader sibling-file discipline.

**Root cause:** Discipline rules codified in prose-only documents (decision-log entries)
without automated enforcement. The "initial application" clause creates a false sense of
completeness when the codified rule's scope is narrower than the actual obligation.

**Codification:** D-381 (STATE.md discipline, pass-8) + D-382 (full cycle-level sibling-file
set: STATE.md + burst-log.md + INDEX.md + lessons.md + decision-log.md, pass-9). Both
pending automation via S-15.03.

**Status:** Codified as prose rules (D-381 + D-382). Automated enforcement pending S-15.03.
Recurrence likely until S-15.03 ships.

---

## L-EDP1-003 — Recursive discipline violation: fix burst violates the rule it codifies

**Source:** F-P6-007 (MEDIUM), F-P9-001 (HIGH)
**Date codified:** 2026-05-11

**Pattern:** Two instances of the "fix burst violates the rule it is simultaneously
codifying" anti-pattern:

1. F-P5-008 (pass-5): advisory recommended CI-green-signal rule. The pass-5 fix burst
   authored the advisory but did not run CI before declaring F-P5-001 CLOSED. F-P6-007
   (pass-6) surfaced the meta-failure. D-379 codified the rule with enforcement teeth.

2. D-381 (pass-8 fix burst): the burst codified "every fix burst MUST update STATE.md"
   and correctly updated STATE.md, but did not update burst-log.md or INDEX.md —
   simultaneously violating the broader sibling-file obligation that D-381 was intended
   to address. F-P9-001 (pass-9) surfaced this. D-382 extended the scope.

**Root cause:** When authoring a new rule during a fix burst, the burst is evaluated
against the rule it is codifying (which is explicit) but not against the broader class
of obligations the rule is intended to enforce (which requires inferential reasoning
about scope). Without a checklist enumerating ALL obligations, the new rule creates a
"partial compliance" state.

**Codification:** D-379 + D-381 + D-382 (all codified as prose). S-15.03 tracks
tooling automation.

**Status:** Pattern documented. Each new fix burst is reminded of D-379/D-381/D-382
via decision-log, but no automated gate prevents omission.

---

## L-EDP1-004 — Forensic-marker proliferation under adversary review pressure

**Source:** F-P3-004 (process-gap observation)
**Date codified:** 2026-05-11

**Pattern:** 321+ `F-P[N]-NNN` forensic markers in production source (observed by
F-P3-004 during pass-3). These markers accumulate across fix bursts as evidence of
applied fixes, but they are in production source files and can create namespace
collisions between cycle-level (F-P[N]-NNN) and per-story identifiers. Cleanup was
deferred but the count continues to grow with each additional fix burst.

**Root cause:** No clean-up protocol established for forensic markers after convergence.
Markers are useful during adversary review but have no value after the cycle closes.

**Codification:** S-14.09 (forensic marker cleanup story; DRAFT for
v1.0-feature-engine-discipline-pass-2 cycle). S-14.09 was registered in STORY-INDEX
v2.65 as part of the F-P6-002/F-P6-004 fix burst.

**Status:** Acknowledged; story S-14.09 filed (draft). Deferred to follow-up cycle.

---

## Process Gaps Documented This Cycle

### PG-EDP1-001 — No lint/hook prevents CI-class CRITICAL closures without CI-green evidence

**Pattern:** D-379 codified the rule; S-15.03 tracks the automation. Until S-15.03 ships,
CI-class CRITICAL finding closures rely entirely on agent/implementer discipline to include
the CI-green URL in the closure record.

**Story:** S-15.03 (DRAFT) — index-cite-refresh + closure-verification hook.

---

### PG-EDP1-002 — No lint/hook enforces sibling-file discipline (cite-refresh, STATE.md, burst-log, INDEX.md, lessons.md)

**Pattern:** L-P20-002 + D-381 + D-382 codify the obligations in prose. The obligations
have been violated 3+ times across this cycle (F-P8-001, F-P9-001) despite being
explicitly codified. Automated enforcement is the only reliable remedy.

**Story:** S-15.03 (DRAFT) — scope expansion to include: (a) BC-INDEX version bump
without ARCH-INDEX changelog entry detection; (b) STATE.md touched without
burst-log.md / INDEX.md also touched detection; (c) fix burst commit without lessons.md
touch when process-gap finding is closed.

---

### PG-EDP1-003 — Cycle-level fix burst discipline reminders are prose-only in decision-log

**Pattern:** D-379, D-381, D-382 are in the decision-log. Agents dispatched for fix bursts
must read the decision-log to know these rules exist. There is no hook, no pre-commit
check, and no structured checklist that surfaces these rules at the moment a fix burst
executes. The STATE-MANAGER-CHECKLIST.md template exists in the engine but was not
instantiated for this cycle.

**Story:** S-15.03 (DRAFT) — instantiate STATE-MANAGER-CHECKLIST.md at cycle init;
add fix-burst discipline reminders to the checklist.

---

### PG-EDP1-004 — Forensic marker namespace not standardized (cycle vs per-story)

**Pattern:** Cycle-level adversary findings use F-P[N]-NNN (e.g., F-P3-001). Per-story
adversary findings use the same format. There is no visual distinction between a
cycle-level finding marker (referring to cycle-wide convergence) and a per-story finding
marker (referring to a specific story's convergence). As the cycle accumulates more
stories and passes, this creates ambiguity in source files about which pass a marker
refers to.

**Story:** S-14.09 (DRAFT; forensic marker cleanup) partially addresses this. A namespace
proposal (e.g., cycle findings as CF-P[N]-NNN; per-story as SF-P[N]-NNN) has been
informally discussed but not codified.

---

All four process gaps converge on S-15.03 scope: a tooling story authored as draft,
awaiting prioritization by the human gate at F7 delta convergence.

---

## L-EDP1-005 — D-383 sub-rule partial application in initial codification burst (layer recursion at D-383)

**Source:** F-P11-001 (MEDIUM), F-P11-002 (MEDIUM)
**Date codified:** 2026-05-11

**Pattern:** The pass-10 fix burst codified D-383 (intra-file content audit + sibling-pattern
sweep) with three explicit sub-rules: (2a) arithmetic consistency, (2b) stale-phrase scan,
(2c) cross-reference verification. In its own initial application, the burst applied:

- 2(a) arithmetic: checked INDEX.md per-row counts (10 rows verified) but did NOT check
  trajectory shorthand cardinality (value count == pass count). Result: stale duplicate "9"
  introduced when inserting "15" at position 2 went undetected (F-P11-001).
- 2(b) stale-phrase: scanned "passes 3-N" but with self-referential window — updated N to
  prior pass (9) instead of current pass (10). Result: "passes 3-9" after the pass-10 burst
  (F-P11-002).
- 2(c) cross-reference: not directly violated this pass.

**Root cause:** L-EDP1-003 pattern (recursive discipline violation — the rule is codified and
partially violated in the same burst) recurred at the D-383 layer. Three consecutive layers:
D-381 (pass-8 burst), D-382 (pass-9 burst), D-383 (pass-10 burst) all exhibited this pattern.
The common thread: when authoring a new rule, the rule's sub-clauses are explicitly enumerated
but the specific sub-clause that applies to the rule's own initial application data is not
cross-checked against the authored rule. Sub-clause 2(a) reads "stated totals match breakdown
sums" — the trajectory shorthand is a "stated total chain" that is arithmetically distinct from
row breakdown sums, requiring a separate check that was not performed.

**Codification:** D-384 extends D-383 with the three missing enforcement sub-rules:
(1) self-referential N clause, (2) trajectory cardinality cross-check, (3) attestation specificity.

**Status:** Codified as prose rule (D-384). S-15.03 automation scope expanded.
Recurrence risk: LOW (three consecutive layers now codified; D-384 is explicit about
cardinality and self-referential N).

---

## L-EDP1-006 — 4-layer recursion: D-384 initial application violated D-384's own sub-rules

**Source:** F-P12-001 (MEDIUM), F-P12-002 (MEDIUM)
**Date codified:** 2026-05-11

**Pattern:** The pass-11 fix burst codified D-384 (extending D-383 with cardinality cross-check,
self-referential N, attestation specificity). In its own initial application, the burst applied
D-384's sub-rules to the canonical 11-value trajectory but did not apply D-385 (being codified
now) because D-385 didn't yet exist. The specific violations:

1. Sub-trajectory sibling enumeration (D-385 sub-rule 1, PG-12-001): corrected the canonical
   "29→15→11→9→8→7→5→6→6→6→4" trajectory in STATE.md Concurrent Cycles + Session Resume
   Checkpoint, but missed the two `9→9→8→7→5` sub-trajectories at STATE.md:63,78 (Phase
   Progress row + Current Phase Steps row). D-384's cardinality check applies to trajectory
   shorthands but did not explicitly require enumerating ALL sub-trajectories in the same file.

2. Immutable-row scope (D-385 sub-rule 2, PG-12-002): added two retroactive NOTE annotations
   to the pass-10 burst-log entry to document corrections. D-383 rule 2(c) forbids "retroactive
   annotations added to immutable rows" but the D-383 text did not enumerate which document types
   have immutable rows, leaving an ambiguity that the pass-11 burst resolved incorrectly.

**Root cause:** L-EDP1-003 pattern (recursive discipline violation) recurred for the 4th consecutive
layer (D-381→D-382→D-383→D-384). Each layer's fix burst has applied the newly codified rule to
the most-prominent defect but missed one sub-class of obligation that the next layer captures.
The common thread: prose rules describe the class of obligation but cannot enumerate every possible
instantiation; each pass finds the next un-enumerated instantiation.

**Codification:** D-385 extends D-383+D-384 with three sub-rules closing the ambiguities:
(1) sub-trajectory sibling enumeration, (2) immutable-row scope explicit enumeration,
(3) per-position attestation completeness (P1-Pn required, not just Pk-Pn).

**Status:** Codified as prose rule (D-385). S-15.03 automation scope expanded.
Recurrence risk: MEDIUM (4 consecutive prose-only layers; prose codification is demonstrably
insufficient; only S-15.03 automation breaks the cycle). Until S-15.03 ships, the risk of a
5th layer remains.

**Corrigendum (pass-14 update):** L-EDP1-007 extends this pattern to a 5th consecutive layer (D-385). See L-EDP1-007 table for the updated chronology. The "4 consecutive layers" count above refers to layers 1-4 (passes 8-11); L-EDP1-007 documents layer 5 (pass-12). The two lessons are complementary, not contradictory.

---

## L-EDP1-007 — Prose-only codification is structurally insufficient for L-EDP1-003 pattern; S-15.03 automation is the only remedy

**Date:** 2026-05-11 (pass-13 cycle-adversary structural diagnosis)
**Source:** F-P13-001 + F-P13-002 + PG-13-003

**Pattern observed across 5 consecutive cycle-level passes:**

| Layer | Burst | Rule Codified | Same-burst Violation |
|-------|-------|---------------|---------------------|
| 1 (pass-8) | D-381 | "fix burst MUST update STATE.md" | missed burst-log + INDEX |
| 2 (pass-9) | D-382 | "fix burst MUST update all 5 sibling files" | introduced intra-file content defects |
| 3 (pass-10) | D-383 | "intra-file content audit + sibling-pattern sweep" | trajectory cardinality + self-ref N missed |
| 4 (pass-11) | D-384 | "3 clarifications to D-383" | sub-trajectories stale; retroactive annotations |
| 5 (pass-12) | D-385 | "3 clarifications to D-383+D-384" | frontmatter schema drift; counting-basis change |

Each new rule closes the prior pass's defect class but the fix burst that codifies the rule introduces NEW defects in dimensions the rule does not govern.

**Diagnosis:** The L-EDP1-003 pattern is structural, not rule-specific. Prose-only codification cannot enumerate ALL possible defect dimensions in advance. Each new rule narrows the failure mode but does not eliminate it; the next failure mode emerges at a new dimension.

**Marginal value of further prose codification:** Approaching zero. After 5 layers + 9 codified rules (D-379, D-381..D-385, plus various sub-rules), each additional pass likely surfaces new defect dimensions.

**Structural remedy:** S-15.03 (automated enforcement lint hook). Concrete scope:
1. Frontmatter schema invariance check across pass-N adversary reviews in a cycle (closes F-P13-001 class)
2. Trajectory cardinality cross-check between INDEX.md, STATE.md, burst-log, lessons.md (closes F-P11-001 class)
3. Sub-trajectory sibling enumeration sweep across all mutable files (closes F-P12-001 class)
4. Immutable-row retroactive-annotation detection (closes F-P12-002 class)
5. Per-position attestation completeness (closes F-P12-003 class)
6. CI-green-link presence on CI-class CRITICAL closures (closes F-P5-001 class)
7. Counting convention enforcement (closes F-P13-002 class)

**Recommendation:** Until S-15.03 ships, accept that prose-only cycle-level convergence is asymptotic. Cycle-level NITPICK_ONLY streak (3/3) may not be achievable without automation. Either:
- (a) Prioritize S-15.03 implementation and re-attempt cycle convergence after it ships, OR
- (b) Define a "human-acceptance" convergence criterion that is laxer than 3-NITPICK_ONLY (e.g., 3 consecutive passes with LOW/NITPICK verdict and no CRITICAL/HIGH/MEDIUM findings on content)

**Status:** Open for orchestrator + human decision.

**Corrigendum (pass-15 fix burst — D-387):** Status RESOLVED. D-386 selected Option C (continue F5; accept asymptotic L-EDP1-003 limit; S-15.03 deferred to next cycle). See L-EDP1-008.

---

## L-EDP1-008 — D-386 closes L-EDP1-007 with Option C selection

**Burst:** F5 pass-14 fix burst (codified) + F5 pass-15 fix burst (this lesson)
**Date codified:** 2026-05-11
**Source:** F-P15-009 (L-EDP1-007 Status stale after D-386)

**Pattern:** L-EDP1-007 Status field was "Open for orchestrator + human decision" pending Options A/B/C (S-15.03 prioritization vs human-acceptance convergence criterion vs continue F5 asymptotically). D-386 (codified in pass-14 fix burst) authoritatively selected Option C: accept asymptotic L-EDP1-003 limit; defer S-15.03 elevation to next cycle. This lesson closes L-EDP1-007's open Status without mutating its body (per D-385/D-387).

**Resolution:** L-EDP1-007 Status is RESOLVED via L-EDP1-008. See decision-log D-386 for the authoritative selection. L-EDP1-007's body remains historically immutable. The corrigendum appended to L-EDP1-007 (per D-387 permitted format) makes the resolution visible at the entry's location.

**Codified rule:** When a prior lesson's Status field becomes stale due to a subsequent decision, author a new L-EDP1-NNN lesson explicitly closing the prior one by reference to the decision. Do NOT edit the prior lesson's Status field directly. Append a corrigendum line (D-387 format: "**Corrigendum**:" prefix, new line before `---` separator) to the prior lesson to point readers to the new closing lesson.

**Status:** Closed (this lesson IS the closure record).

---

## L-EDP1-009 — D-387 self-application partial; 7th-layer L-EDP1-003 recurrence at BC frontmatter sibling-chain dimension

**Burst:** F5 pass-16 fix burst (codifies the lesson; the recurrence itself was in pass-15)
**Date codified:** 2026-05-11
**Source:** F-P16-001, F-P16-002, F-P16-003, F-P16-PG1

**Pattern:** D-387 was codified by the pass-15 fix burst with the explicit clause "(b) propagated
to ALL sibling sites in the same burst (sibling-pattern sweep mandatory per D-383)". The pass-15
burst executed the sweep on two defect classes (MEDIUM-HIGH verdict labels in adversary-review
frontmatter; status:draft on merged stories) but the sweep did NOT extend to additional
sibling-chain dimensions surfaced by pass-16:

- Story `merged_at` ↔ STATE.md Phase Progress merge-date (F-P16-001)
- BC `last_amended` ↔ CHANGELOG most-recent row (F-P16-002)
- BC `input-hash` placeholder propagation (F-P16-004 — resolved by D-389 convention rather than fix)

This is the 7th consecutive layer of the L-EDP1-003 pattern:

| Layer | Burst | Rule Codified | Same-burst Violation |
|-------|-------|---------------|---------------------|
| 1 (pass-8) | D-381 | "fix burst MUST update STATE.md" | missed burst-log + INDEX |
| 2 (pass-9) | D-382 | "fix burst MUST update all 5 sibling files" | introduced intra-file content defects |
| 3 (pass-10) | D-383 | "intra-file content audit + sibling-pattern sweep" | trajectory cardinality + self-ref N missed |
| 4 (pass-11) | D-384 | "3 clarifications to D-383" | sub-trajectories stale; retroactive annotations |
| 5 (pass-12) | D-385 | "3 clarifications to D-383+D-384" | frontmatter schema drift; counting-basis change |
| 6 (pass-15) | D-387 | "structural-correction exception + sibling sweep" | sweep dimensions not enumerated; adjacent sibling-chain dimensions not covered |
| 7 (this) | D-389+D-390 | "input-hash convention + CHANGELOG→last_amended rule" | enumerated below |

**Resolution:** Per D-386 Option C (asymptotic convergence accepted), no further structural
escalation this cycle. Document the recurrence. Future cycles SHOULD prioritize S-15.03 to
provide automated sweep enforcement.

**Codified rule:** Sibling-pattern sweeps under D-383/D-387 SHOULD enumerate the dimensions
explicitly in the fix-burst attestation. A sweep claim with no enumerated dimensions is asserted
but not auditable. The enumeration format: "Sweep dimensions checked: (1) X — result; (2) Y — result."

**Status:** Codified. D-389 + D-390 close the two adjacent defect classes. L-EDP1-003 pattern
continues at asymptotic boundary per D-386 Option C.

**Corrigendum (pass-17 fix burst — D-387):** Layer-7 enumeration (sweep dimensions checked in pass-16):
- (1) STATE.md merge-date sibling chain — enumeration source: explicit per-file (STATE.md rows 60-61 + story frontmatter + INDEX.md rows 39-40). Extent: 3 files, 4 sites. Audited: 4/4. Corrected: 1 (STATE.md rows 60-61 → 2026-05-10).
- (2) BC last_amended ↔ CHANGELOG — enumeration source: project policy rubric (7-BC subset: BC-4.12.001/002/003/004/005 + BC-1.13.001 + BC-5.39.001). Extent: 7. Audited: 7/7. Corrected: 5. NOTE: enumeration source was narrower than full project policy rubric (9 in-cycle BCs); F-P17-001 surfaced 3 missing-field gaps in BC-5.39.002 / BC-7.03.091 / BC-7.03.092 — closed in pass-17 burst.
- (3) adv-cycle-pass-12.md current_step quoting — enumeration source: explicit per-file (1 file). Extent: 1. Audited: 1. Corrected: 1.
- (4) STATE.md factory-artifacts SHA — enumeration source: explicit per-field (STATE.md:123). Extent: 1 line. Audited: 1. Corrected: 1.
- (5) F-P16-008/009 timestamp Z — DEFERRED per adversary recommendation; F-P17-004 expands the dimension to 12 sites (9 adv-cycle-pass files + 3 index files). Closed in pass-17 burst.

Layer-7 was thus partial; F-P17-001/002/003/004 in pass-17 are the residual. D-391 (codified in pass-17) closes F-P17-PG1 by making enumeration source citation mandatory going forward.
