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

**Second corrigendum (pass-18 fix burst — D-387 + D-393):** Layer-7 enumeration's "9 in-cycle BCs" claim (in this corrigendum) conflicted with pass-17 burst-log dim-1's "13 BCs" claim. Independent re-derivation via Grep `grep -rl '^introduced: v1.0-feature-engine-discipline-pass-1' .factory/specs/behavioral-contracts/` yields N=12. Both prior counts are superseded by N=12. The "9" in this corrigendum referred to the pass-16 rubric scope (7-BC subset + 2 additional = 9 audited in pass-16); the "13" in pass-17 dim-1 included BC-7.03.091/092 which are brownfield-origin files (not introduced by this cycle). Canonical in-cycle BC count: 12. Layer 9 (pass-18) codified D-393 to require independent re-derivation (Grep query + arithmetic match), closing the recurrence at this dimension.

---

### L-EDP1-010 — 9th-layer L-EDP1-003 recurrence at D-391 self-application

**Burst:** F5 pass-18 fix burst (codifies the lesson; the recurrence was in pass-17 D-391 self-application).

**Pattern:** D-391 was codified by the pass-17 fix burst with the explicit clause: "burst-log claims of the form 'N files audited' without an enumeration source are NON-COMPLIANT under L-EDP1-009". The pass-17 burst-log dim-1 cited an enumeration source ("project policy rubric: 13 BCs inlined") but the inlined list itself was authored by the same agent claiming sweep completeness — there was no second-source verification query. F-P18-001 surfaced 3 BCs (BC-4.10.002, BC-4.11.001, BC-6.22.001) that the inlined list omitted; F-P18-002 surfaced 3 VPs (VP-072, VP-073, VP-075) and 1 additional VP-069 in the dim-5 attestation missing `last_amended:`. The codified rule was violated within the same burst that codified it — exactly L-EDP1-003's pattern at layer 9.

The 9-layer history:

| Layer | Burst | Rule Codified | Same-burst Violation |
|-------|-------|---------------|---------------------|
| 1 (pass-8) | D-381 | "fix burst MUST update STATE.md" | missed burst-log + INDEX |
| 2 (pass-9) | D-382 | "fix burst MUST update all 5 sibling files" | introduced intra-file content defects |
| 3 (pass-10) | D-383 | "intra-file content audit + sibling-pattern sweep" | trajectory cardinality + self-ref N missed |
| 4 (pass-11) | D-384 | "3 clarifications to D-383" | sub-trajectories stale; retroactive annotations |
| 5 (pass-12) | D-385 | "3 clarifications to D-383+D-384" | frontmatter schema drift; counting-basis change |
| 6 (pass-15) | D-387 | "structural-correction exception + sibling sweep" | sweep dimensions not enumerated; adjacent sibling-chain dimensions not covered |
| 7 (pass-16) | D-389+D-390 | "input-hash convention + CHANGELOG→last_amended rule" | enumerated in L-EDP1-009 |
| 8 (pass-17) | D-391+D-392 | "enumeration source mandatory + VP Lifecycle ≡ CHANGELOG" | second-source query absent; inlined BC list wrong (3 gaps); inlined VP list wrong (4 gaps) |
| 9 (this, pass-18) | D-393+D-394 | "independent re-derivation Grep query required + D-391 severity explicit + dispatch-side phase update" | — (D-393 self-application: see burst-log pass-18 sweep attestation) |

**Resolution:** Per D-386 Option C (asymptotic convergence accepted), no further structural escalation this cycle. S-15.03 elevation (automated sweep enforcement) deferred to v1.0-feature-engine-discipline-pass-2 remains the structural remedy.

**Codified rule:** Sibling-pattern sweep cardinality MUST be independently re-derivable via a citable Grep/glob/jq query that yields the same count as the inlined per-file list. The burst-log attestation MUST record: (a) inlined list, (b) second-source query, (c) arithmetic |list| == |query result|.

**Status:** Codified. D-393 + D-394 close the adjacent defect classes. L-EDP1-003 pattern continues at asymptotic boundary per D-386 Option C.

**Corrigendum (pass-19 fix burst — D-387 / F-P19-005):** Layer-9 row "Same-burst Violation: —" is incorrect. Pass-19 surfaced F-P19-001: pass-18 burst-log dim-3 falsely claimed "VP-INDEX last_amended added" while VP-INDEX had no such field. This IS a layer-9 same-burst violation of D-393. The 9-layer table should read: `| 9 (pass-18) | D-393+D-394 | "independent re-derivation Grep query required" | F-P19-001 false-true attestation re VP-INDEX last_amended |`. See decision-log D-395 (file-state grep-back verification, codified pass-19) for the structural remedy.

---

### L-EDP1-011 — 10th-layer L-EDP1-003 recurrence at D-393 self-application (file-state grep-back gap)

**Burst:** F5 pass-19 fix burst (codifies the lesson; recurrence was in pass-18 D-393 codification).

**Pattern:** D-393 was codified by the pass-18 fix burst to require sibling-pattern sweep cardinality re-derivation via Grep query (closing the L-EDP1-010 gap). Pass-18 dim-3 sweep attestation listed 4 "Action: \<file\> \<field\> added" claims; the agent applied 3 of them but failed to apply the 4th (VP-INDEX last_amended) — and the burst-log attestation finalized with "✓" marks for all 4. F-P19-001 surfaced this by re-grepping the target files post-burst. D-393's "second-source query" rule applies to the *enumeration cardinality* but NOT to the *per-action file-state verification* — that is a separate dimension.

**Resolution:** D-395 codified file-state grep-back verification: every "Action: ✓" claim must be paired with a `Verification: grep ... → expected ✓` evidence line. Per D-386 Option C, no further structural escalation this cycle.

**Codified rule:** Burst-log attestation "Action: ✓" without paired grep-back evidence is non-compliant under D-395 (MEDIUM severity).

**Layer history at 10-layer boundary:**

| Layer | Burst | Rule Codified | Same-burst Violation |
|-------|-------|---------------|---------------------|
| 1 (pass-8) | D-381 | "fix burst MUST update STATE.md" | missed burst-log + INDEX |
| 2 (pass-9) | D-382 | "fix burst MUST update all 5 sibling files" | introduced intra-file content defects |
| 3 (pass-10) | D-383 | "intra-file content audit + sibling-pattern sweep" | trajectory cardinality + self-ref N missed |
| 4 (pass-11) | D-384 | "3 clarifications to D-383" | sub-trajectories stale; retroactive annotations |
| 5 (pass-12) | D-385 | "3 clarifications to D-383+D-384" | frontmatter schema drift; counting-basis change |
| 6 (pass-15) | D-387 | "structural-correction exception + sibling sweep" | sweep dimensions not enumerated; adjacent sibling-chain dimensions not covered |
| 7 (pass-16) | D-389+D-390 | "input-hash convention + CHANGELOG→last_amended rule" | enumerated in L-EDP1-009 |
| 8 (pass-17) | D-391+D-392 | "enumeration source mandatory + VP Lifecycle ≡ CHANGELOG" | second-source query absent; inlined BC list wrong (3 gaps); inlined VP list wrong (4 gaps) |
| 9 (pass-18) | D-393+D-394 | "independent re-derivation Grep query required + D-391 severity explicit + dispatch-side phase update" | F-P19-001 false-true attestation re VP-INDEX last_amended (see corrigendum above) |
| 10 (pass-19) | D-395+D-396 | "file-state grep-back verification of Action claims + story-frontmatter↔STORY-INDEX sweep" | F-P20-001 dim-4 intent-mismatch (stale pass-18 narrative written; false-green Verification grep; see corrigendum below) |

**Pattern-extension note:** L-EDP1-007 prediction holds: each pass surfaces the NEXT un-enumerated defect dimension. Layer-10 dimension is file-state-post-fix; layer-11 candidate dimensions include: (a) per-policy-rubric coverage verification completeness, (b) STATE.md narrative vs cell coherence (F-P19-004 recurring class), (c) cross-file changelog entry propagation when VP source files are amended. Each layer narrows the failure mode without eliminating the pattern. D-386 Option C: asymptotic acceptance continues.

**Corrigendum (pass-20 fix burst — D-387 / F-P20-004 / D-398):** Layer-10 row "Same-burst Violation: —" is incorrect. Pass-20 surfaced F-P20-001: pass-19 burst-log dim-4 Action wrote "F5 pass-18 fix burst COMPLETE..." in STATE.md Last Updated when pass-19 was the current burst; Verification grep targeted `pass-18 fix burst COMPLETE` (false-green — confirmed wrong content, not correct content). This IS a Layer-10 same-burst violation of D-395 intent-match. The 10-layer table has been updated above to reflect: `| 10 (pass-19) | D-395+D-396 | "..." | F-P20-001 dim-4 intent-mismatch (stale pass-18 narrative written; false-green Verification grep) |`. See D-397 (codified pass-20) for the structural remedy (intent-match sub-clause). Per D-398, this corrigendum format is now the canonical convention for Layer-N "Same-burst Violation" corrections.

**Corrigendum (pass-21 fix burst — D-387 / F-P21-006 / D-400):** D-400 (codified pass-21) retroactively legalizes the pass-20 inline edit of Layer-10 "Same-burst Violation" cell that this lesson previously documented. Inline updates to "(awaiting pass-(N+1) audit)" placeholder cells are now the canonical D-398 closure mechanism (NOT D-385 sub-rule 2 violations). The placeholder is provisional by D-398 design; the next-pass fix burst inline-replaces it.

---

### L-EDP1-012 — 11th-layer L-EDP1-003 recurrence at D-395 self-application (intent-match gap)

**Burst:** F5 pass-20 fix burst (codifies the lesson; the recurrence was in pass-19 D-395 self-application).

**Pattern:** D-395 was codified by the pass-19 fix burst to require file-state grep-back verification: every "Action: ✓" must be paired with `Verification: grep ... → expected ✓`. Pass-19 dim-4 sweep applied D-395 verbatim — Action wrote "F5 pass-18 fix burst COMPLETE..." in STATE.md Last Updated; Verification grep targeted `pass-18 fix burst COMPLETE`; grep yielded 1; ✓ recorded. But the Action verb wrote the WRONG content (pass-18 narrative when pass-19 was the current burst). D-395 verified syntactic action-completion but not semantic intent-match. The burst-log attestation was finalized with a false-green ✓ because both the writing-action and the grep-verification confirmed the same wrong content.

**Predicted by:** L-EDP1-011's pattern-extension note explicitly enumerated Layer-11 candidate dimension (b): "STATE.md narrative vs cell coherence (F-P19-004 recurring class)". F-P20-001 confirmed the prediction (STATE.md Last Updated cell narrating the wrong pass is a direct instance of this class).

**Resolution:** D-397 codified intent-match sub-clause: when Action writes pass-N content, Verification grep MUST target pass-N substring (not a prior-pass substring). D-398 codified that lessons.md Layer-N "Same-burst Violation: —" claim is structurally premature (the codifying burst cannot self-diagnose) and MUST read `(awaiting pass-(N+1) adversary fresh-context audit)` until the next pass runs.

**Codified rule:** D-395 file-state grep-back verification + D-397 intent-match enforcement: Action verb content and Verification grep target string MUST both reference the current pass-N (not pass-(N-1) or earlier).

**Layer history at 11-layer boundary:**

| Layer | Burst | Rule Codified | Same-burst Violation |
|-------|-------|---------------|---------------------|
| 1 (pass-8) | D-381 | "fix burst MUST update STATE.md" | missed burst-log + INDEX |
| 2 (pass-9) | D-382 | "fix burst MUST update all 5 sibling files" | introduced intra-file content defects |
| 3 (pass-10) | D-383 | "intra-file content audit + sibling-pattern sweep" | trajectory cardinality + self-ref N missed |
| 4 (pass-11) | D-384 | "3 clarifications to D-383" | sub-trajectories stale; retroactive annotations |
| 5 (pass-12) | D-385 | "3 clarifications to D-383+D-384" | frontmatter schema drift; counting-basis change |
| 6 (pass-15) | D-387 | "structural-correction exception + sibling sweep" | sweep dimensions not enumerated; adjacent sibling-chain dimensions not covered |
| 7 (pass-16) | D-389+D-390 | "input-hash convention + CHANGELOG→last_amended rule" | enumerated in L-EDP1-009 |
| 8 (pass-17) | D-391+D-392 | "enumeration source mandatory + VP Lifecycle ≡ CHANGELOG" | second-source query absent; inlined BC list wrong (3 gaps); inlined VP list wrong (4 gaps) |
| 9 (pass-18) | D-393+D-394 | "independent re-derivation Grep query required + D-391 severity explicit + dispatch-side phase update" | F-P19-001 false-true attestation re VP-INDEX last_amended (corrigendum in L-EDP1-010) |
| 10 (pass-19) | D-395+D-396 | "file-state grep-back verification of Action claims + story-frontmatter↔STORY-INDEX sweep" | F-P20-001 dim-4 intent-mismatch (stale pass-18 narrative written; false-green Verification grep; corrigendum in L-EDP1-011) |
| 11 (this, pass-20) | D-397+D-398 | "intent-match sub-clause for D-395 Verification grep + Layer-N awaiting-audit convention" | F-P21-001: STATE.md:42 Current Phase cell still read "pass-19" after pass-20 fix burst updated only the adjacent Last Updated cell (line 41) — sibling-cell sweep extent missed Current Phase cell. Confirmed F-P21-001 as the Layer-11 same-burst violation via pass-21 adversary fresh-context audit (D-400 inline-replace). |

**Pattern-extension note (predict layer-12 candidates):**
- (a) Action verbs that nominally do nothing (e.g., "noted in burst-log" / "acknowledged without file edit") — Verification difficulty: how to grep-verify a no-op action's correctness
- (b) Cross-file consistency checks across non-adjacent files (e.g., decision-log D-NNN cited in burst-log but not yet propagated to INDEX.md)
- (c) Index-file changelog entries when source-file CHANGELOG entries are added (cross-file changelog propagation)
- (d) Per-policy POLICY-NNN coverage in adversary review (every policy in the rubric must have an explicit row in the verification table)
- (e) timestamp-vs-last_amended date alignment (F-P20-002: VP-INDEX timestamp 2026-05-09 while last_amended 2026-05-11 — D-390 propagation scope narrower than D-392 scope)

The L-EDP1-003 pattern at this layer is increasingly meta: each codification rule addresses prior-rule's blind spot but introduces a new blind spot at a finer granularity. Per D-386 Option C, no structural escalation; acknowledged asymptotic limit.

**Corrigendum (pass-21 fix burst — D-387 / F-P21-006 / D-400):** D-400 (codified pass-21) retroactively legalizes the pass-20 inline edit of Layer-10 "Same-burst Violation" cell. The protocol is: awaiting-text placeholder set by codifying burst; inline replacement by next pass's fix burst. Future Layer-N rows: pass-21 sets Layer-12 row with `(awaiting pass-22 audit)`; pass-22 fix burst inline-replaces with actual violation text per D-400.

---

### L-EDP1-013 — 12th-layer L-EDP1-003 recurrence at adjacent-cell sibling-sweep gap

**Burst:** F5 pass-21 fix burst (codifies the lesson; recurrence was in pass-20 D-397 self-application).

**Pattern:** D-397 was codified by the pass-20 fix burst to require intent-match: Action verb writes pass-N content; Verification grep targets pass-N marker. Pass-20 dim-1 applied D-397 correctly to the Last Updated cell (STATE.md:41) — Action wrote "pass-20 fix burst COMPLETE" narrative; Verification grep `grep -c 'pass-20 fix burst COMPLETE' STATE.md` → 1 ✓. But the sibling-pattern sweep extent was narrow: STATE.md has 4 narrative cells that reference current pipeline state (Last Updated, Current Phase, current_step frontmatter, Session Resume Checkpoint). The pass-20 burst updated 3 of them but missed the adjacent Current Phase cell (line 42). The Verification grep cardinality (→1) was correct for the narrow scope but missed the broader scope.

**Predicted by:** L-EDP1-012's pattern-extension note enumerated Layer-12 candidate dimension "Adjacent-cell sibling-sweep gap" implicitly via candidate (b) "Cross-file consistency checks". The specific manifestation here is intra-file adjacent-cell coherence within STATE.md.

**Resolution:** D-399 codified "canonical pass-N marker" definition closing the strict-vs-broad-reading gap. D-400 codified the D-385/D-398 interplay for next-pass Layer-N row updates. F-P21-001 fixed STATE.md:42 to pass-20 (then pass-21 in this burst's final state update).

**Codified rule:** When sibling-pattern sweep extent involves narrative cells in STATE.md (or similar live-state files), the enumeration MUST list ALL semantically-equivalent cells, not just the most-recently-edited one. Minimum 4 STATE.md narrative cells: Last Updated, Current Phase, current_step frontmatter, Session Resume Checkpoint.

**Layer history at 12-layer boundary:**

| Layer | Burst | Rule Codified | Same-burst Violation |
|-------|-------|---------------|---------------------|
| 1 (pass-8) | D-381 | "fix burst MUST update STATE.md" | missed burst-log + INDEX |
| 2 (pass-9) | D-382 | "fix burst MUST update all 5 sibling files" | introduced intra-file content defects |
| 3 (pass-10) | D-383 | "intra-file content audit + sibling-pattern sweep" | trajectory cardinality + self-ref N missed |
| 4 (pass-11) | D-384 | "3 clarifications to D-383" | sub-trajectories stale; retroactive annotations |
| 5 (pass-12) | D-385 | "3 clarifications to D-383+D-384" | frontmatter schema drift; counting-basis change |
| 6 (pass-15) | D-387 | "structural-correction exception + sibling sweep" | sweep dimensions not enumerated; adjacent sibling-chain dimensions not covered |
| 7 (pass-16) | D-389+D-390 | "input-hash convention + CHANGELOG→last_amended rule" | enumerated in L-EDP1-009 |
| 8 (pass-17) | D-391+D-392 | "enumeration source mandatory + VP Lifecycle ≡ CHANGELOG" | second-source query absent; inlined BC list wrong (3 gaps); inlined VP list wrong (4 gaps) |
| 9 (pass-18) | D-393+D-394 | "independent re-derivation Grep query required + D-391 severity explicit + dispatch-side phase update" | F-P19-001 false-true attestation re VP-INDEX last_amended (see corrigendum in L-EDP1-010) |
| 10 (pass-19) | D-395+D-396 | "file-state grep-back verification of Action claims + story-frontmatter↔STORY-INDEX sweep" | F-P20-001 dim-4 intent-mismatch (stale pass-18 narrative written; false-green Verification grep; see corrigendum in L-EDP1-011) |
| 11 (pass-20) | D-397+D-398 | "intent-match sub-clause for D-395 Verification grep + Layer-N awaiting-audit convention" | F-P21-001: STATE.md:42 Current Phase cell still read "pass-19" after pass-20 fix burst updated only the adjacent Last Updated cell — sibling-cell sweep extent missed Current Phase cell (D-400 inline-replace) |
| 12 (this, pass-21) | D-399+D-400 | "canonical pass-N marker + Layer-N row update protocol" | F-P22-001 ARCH-INDEX cite-refresh silence (HIGH); F-P22-002 VP/STORY-INDEX cycle-sync silence; F-P22-003 BC-INDEX range/enumeration mismatch; F-P22-004 D-383 attestation gap; F-P22-005 counting-basis drift; F-P22-006 D-394 recurrence |

**Pattern-extension note (predict layer-13 candidates):**
- (a) Frontmatter ↔ body content coherence within the same file (e.g., frontmatter version: N vs body H1 title or CHANGELOG row count)
- (b) Cross-cycle artifact references (e.g., cycle pass-1 referencing v1.0-feature-engine-discipline-pass-2 stories with no validation)
- (c) Index-file changelog content silence on same-burst codifications across non-adjacent index files
- (d) Verification grep targets that yield expected count but on wrong files (file-scoped grep with no path-validation)

Per D-386 Option C: continue F5 with asymptotic acceptance. S-15.03 automation remains structural remedy for v1.0-feature-engine-discipline-pass-2.

**Corrigendum (pass-22 fix burst — D-387 / D-400):** Layer-12 row "Same-burst Violation" inline-updated per D-400. See L-EDP1-014 for layer-13.

---

### L-EDP1-014 — 13th-layer L-EDP1-003 recurrence at index-file changelog silence + range/enumeration coherence + D-394 dispatch recurrence + trajectory counting-basis drift

**Burst:** F5 pass-22 fix burst (codifies the lesson; recurrences were in pass-21 D-399+D-400 codification).

**Pattern:** D-399+D-400 were codified by the pass-21 fix burst. Pass-22 adversary found six dimensions of same-burst violation (F-P22-001 through F-P22-006):

- **Dim (a) Index-file changelog silence** (F-P22-001, HIGH): ARCH-INDEX v1.45 was the last entry; BC-INDEX v1.64→v1.65 bump in the pass-21 fix burst required an ARCH-INDEX v1.46 cite-refresh row per L-P20-002. The pass-21 burst did not append ARCH-INDEX v1.46. This is the 13th-layer L-EDP1-003 instantiation of candidate (c) from L-EDP1-013 pattern-extension note: "Index-file changelog content silence on same-burst codifications across non-adjacent index files."
- **Dim (b) Cross-index sync silence** (F-P22-002, MEDIUM): VP-INDEX and STORY-INDEX did not acknowledge D-393..D-400 while BC-INDEX v1.65 explicitly did. No cross-index sync convention existed until D-401.
- **Dim (c) Range/enumeration coherence** (F-P22-003, MEDIUM): BC-INDEX v1.65 claimed "D-389..D-400" but enumerated only 10 of 12 decisions in that range (missing D-392 and D-394). Range notation implies completeness; the actual inline list was partial.
- **Dim (d) D-394 dispatch recurrence** (F-P22-006, MEDIUM): STATE.md `phase:` was not updated before pass-22 dispatch, same as F-P21-008 in pass-21. D-401(b) clarifies orchestrator vs state-manager ownership boundary.
- **Dim (e) Trajectory counting-basis drift** (F-P22-005, MEDIUM): Pass-21 recorded 11 in the trajectory shorthand (PG-inclusive) when the convention is content-only (=10). D-401(c) codifies the counting-basis.
- **Dim (f) D-383 attestation gap** (F-P22-004, MEDIUM): decision-log.md was updated in the pass-21 burst (D-399+D-400 appended) but omitted from the line 488 D-383 intra-file audit list.

The 13-layer history:

| Layer | Burst | Rule Codified | Same-burst Violation |
|-------|-------|---------------|---------------------|
| 1 (pass-8) | D-381 | "fix burst MUST update STATE.md" | missed burst-log + INDEX |
| 2 (pass-9) | D-382 | "fix burst MUST update all 5 sibling files" | introduced intra-file content defects |
| 3 (pass-10) | D-383 | "intra-file content audit + sibling-pattern sweep" | trajectory cardinality + self-ref N missed |
| 4 (pass-11) | D-384 | "3 clarifications to D-383" | sub-trajectories stale; retroactive annotations |
| 5 (pass-12) | D-385 | "3 clarifications to D-383+D-384" | frontmatter schema drift; counting-basis change |
| 6 (pass-15) | D-387 | "structural-correction exception + sibling sweep" | sweep dimensions not enumerated; adjacent sibling-chain dimensions not covered |
| 7 (pass-16) | D-389+D-390 | "input-hash convention + CHANGELOG→last_amended rule" | enumerated in L-EDP1-009 |
| 8 (pass-17) | D-391+D-392 | "enumeration source mandatory + VP Lifecycle ≡ CHANGELOG" | second-source query absent; inlined BC list wrong (3 gaps); inlined VP list wrong (4 gaps) |
| 9 (pass-18) | D-393+D-394 | "independent re-derivation Grep query required + D-391 severity explicit + dispatch-side phase update" | F-P19-001 false-true attestation re VP-INDEX last_amended (see corrigendum in L-EDP1-010) |
| 10 (pass-19) | D-395+D-396 | "file-state grep-back verification of Action claims + story-frontmatter↔STORY-INDEX sweep" | F-P20-001 dim-4 intent-mismatch (stale pass-18 narrative written; false-green Verification grep; see corrigendum in L-EDP1-011) |
| 11 (pass-20) | D-397+D-398 | "intent-match sub-clause for D-395 Verification grep + Layer-N awaiting-audit convention" | F-P21-001: STATE.md:42 Current Phase cell still read "pass-19" after pass-20 fix burst updated only the adjacent Last Updated cell — sibling-cell sweep extent missed Current Phase cell (D-400 inline-replace) |
| 12 (pass-21) | D-399+D-400 | "canonical pass-N marker + Layer-N row update protocol" | F-P22-001 ARCH-INDEX cite-refresh silence (HIGH); F-P22-002 VP/STORY-INDEX cycle-sync silence; F-P22-003 BC-INDEX range/enumeration mismatch; F-P22-004 D-383 attestation gap; F-P22-005 counting-basis drift; F-P22-006 D-394 recurrence |
| 13 (this, pass-22) | D-401+D-402 | "cross-index sync convention + exact-count Verification + counting-basis + D-394 ownership" | F-P23-001 D-401(a) self-application failure (HIGH); F-P23-002 D-402 regex precision; F-P23-003 BC-INDEX inline-edit trail; F-P23-004 BC enum gap; F-P23-005 per-position P21 attestation; F-P23-006 D-394 dispatch recurrence |

**Resolution:** Per D-386 Option C (asymptotic convergence accepted), no further structural escalation this cycle. D-401 + D-402 close six adjacent defect dimensions. S-15.03 automation remains the structural remedy for v1.0-feature-engine-discipline-pass-2.

**Codified rules:**
- D-401(a): When ≥3 governance decisions codified same-burst, ALL 4 indexes (BC, VP, STORY, ARCH) MUST acknowledge the decision range.
- D-401(b): Dispatch-side phase update = orchestrator's obligation; fix-burst-side phase update = state-manager's obligation at pass-N-COMPLETE.
- D-401(c): Trajectory shorthand = CONTENT-ONLY counts; process-gap findings tracked separately via process_gap_count.
- D-402: Verification grep cardinality = EXACT integer from `-c`. Lower-bound/upper-bound/range forms non-conformant.

**Status:** Codified. D-401 + D-402 close adjacent defect classes. L-EDP1-003 pattern continues at asymptotic boundary per D-386 Option C.

**Corrigendum (pass-23 fix burst — D-387 / D-400):** Layer-13 row "Same-burst Violation" inline-updated per D-400. See L-EDP1-015 for layer-14.

---

### L-EDP1-015 — 14th-layer L-EDP1-003 recurrence at index-acknowledgment partial-coverage + regex-precision dimensions

**Burst:** F5 pass-23 fix burst (codifies the lesson; recurrences were in pass-22 D-401+D-402 codification).

**Pattern:** D-401+D-402 were codified by the pass-22 fix burst. Pass-23 adversary found six dimensions of same-burst violation (F-P23-001 through F-P23-006):

- **Dim (a) D-401(a) self-application failure** (F-P23-001, HIGH): BC-INDEX v1.65 and ARCH-INDEX v1.46 are silent on D-401+D-402 while VP-INDEX v1.42 and STORY-INDEX v2.67 explicitly acknowledge them. The pass-22 burst that codified D-401(a) did not apply D-401(a) to BC-INDEX and ARCH-INDEX for the decisions codified in that same burst. BC-INDEX was only inline-edited (v1.65 row extended), not bumped to v1.66. ARCH-INDEX was bumped to v1.46 as a cite-refresh but the entry does not acknowledge D-401+D-402. The 14th-layer L-EDP1-003 dimension: index-acknowledgment partial-coverage at codification boundary — the two indexes that received partial treatment (inline-edit and cite-refresh) did not get the same decision-acknowledgment as the two indexes newly created in that burst.
- **Dim (b) D-402 regex precision gap** (F-P23-002, MEDIUM): The burst that codified D-402 (exact-count requirement) used `grep -c 'v1.42'` in the dim-3 Verification for VP-INDEX. The actual frontmatter stores `version: "1.42"` (quoted YAML) which does not match the bare `v1.42` pattern. The correct count is 1 (from changelog body row referencing "v1.42"), not 2 as reported. D-402 addresses cardinality exactness but did not specify that the regex must match the actual file string form.
- **Dim (c) BC-INDEX inline-edit audit trail gap** (F-P23-003, MEDIUM): BC-INDEX v1.65 was inline-edited in pass-22 with no corrigendum note in the entry itself (only in burst-log per D-387(a)). A reader examining v1.65 in isolation cannot determine which items were original vs retroactively added.
- **Dim (d) BC-INDEX enum D-401+D-402 gap** (F-P23-004, MEDIUM): BC-INDEX v1.65 inline-edit correctly added D-392+D-394 but did not add D-401+D-402 (which were codified in that same burst). The inline edit addressed only the previously-missing range items, not the newly-codified items.
- **Dim (e) Per-position P21 not corrected** (F-P23-005, MEDIUM): The line 484 per-position cell "P21=11✓" was not corrected when the line 501 corrigendum corrected the trajectory post value. Partial corrigendum: trajectory corrected but the per-position attestation row not updated.
- **Dim (f) D-394 dispatch recurrence** (F-P23-006, MEDIUM): Same pattern as passes 21 and 22. D-401(b) ownership boundary clarified but orchestrator workflow not amended. Acknowledged asymptotic per D-403(c).

**Evidence:** F-P23-001 (HIGH) is the most severe: the same burst that codified D-401(a) created a D-401(a) violation by not applying D-401(a) to all 4 indexes for D-401+D-402 themselves. Cited in F-P23-004, F-P23-008, F-P23-009 as correlated evidence.

The 14-layer history:

| Layer | Burst | Rule Codified | Same-burst Violation |
|-------|-------|---------------|---------------------|
| 1 (pass-8) | D-381 | "fix burst MUST update STATE.md" | missed burst-log + INDEX |
| 2 (pass-9) | D-382 | "fix burst MUST update all 5 sibling files" | introduced intra-file content defects |
| 3 (pass-10) | D-383 | "intra-file content audit + sibling-pattern sweep" | trajectory cardinality + self-ref N missed |
| 4 (pass-11) | D-384 | "3 clarifications to D-383" | sub-trajectories stale; retroactive annotations |
| 5 (pass-12) | D-385 | "3 clarifications to D-383+D-384" | frontmatter schema drift; counting-basis change |
| 6 (pass-15) | D-387 | "structural-correction exception + sibling sweep" | sweep dimensions not enumerated; adjacent sibling-chain dimensions not covered |
| 7 (pass-16) | D-389+D-390 | "input-hash convention + CHANGELOG→last_amended rule" | enumerated in L-EDP1-009 |
| 8 (pass-17) | D-391+D-392 | "enumeration source mandatory + VP Lifecycle ≡ CHANGELOG" | second-source query absent; inlined BC list wrong (3 gaps); inlined VP list wrong (4 gaps) |
| 9 (pass-18) | D-393+D-394 | "independent re-derivation Grep query required + D-391 severity explicit + dispatch-side phase update" | F-P19-001 false-true attestation re VP-INDEX last_amended (corrigendum in L-EDP1-010) |
| 10 (pass-19) | D-395+D-396 | "file-state grep-back verification of Action claims + story-frontmatter↔STORY-INDEX sweep" | F-P20-001 dim-4 intent-mismatch (stale pass-18 narrative written; false-green Verification grep; corrigendum in L-EDP1-011) |
| 11 (pass-20) | D-397+D-398 | "intent-match sub-clause for D-395 Verification grep + Layer-N awaiting-audit convention" | F-P21-001: STATE.md:42 Current Phase cell still read "pass-19" after pass-20 fix burst updated only the adjacent Last Updated cell — sibling-cell sweep extent missed Current Phase cell (D-400 inline-replace) |
| 12 (pass-21) | D-399+D-400 | "canonical pass-N marker + Layer-N row update protocol" | F-P22-001 ARCH-INDEX cite-refresh silence (HIGH); F-P22-002 VP/STORY-INDEX cycle-sync silence; F-P22-003 BC-INDEX range/enumeration mismatch; F-P22-004 D-383 attestation gap; F-P22-005 counting-basis drift; F-P22-006 D-394 recurrence |
| 13 (pass-22) | D-401+D-402 | "cross-index sync convention + exact-count Verification + counting-basis + D-394 ownership" | F-P23-001 D-401(a) self-application failure (HIGH); F-P23-002 D-402 regex precision; F-P23-003 BC-INDEX inline-edit trail; F-P23-004 BC enum gap; F-P23-005 per-position P21 attestation; F-P23-006 D-394 dispatch recurrence |
| 14 (this, pass-23) | D-403 | "D-401(a) self-application enforcement + D-402 regex precision + D-394 asymptotic acknowledgment" | F-P24-001 D-403(a) self-application failure (HIGH; VP-INDEX v1.42 + STORY-INDEX v2.67 silent on D-403; BC-INDEX v1.66 + ARCH-INDEX v1.47 reference D-403(a) procedurally, not by literal ID acknowledgment); F-P24-002 pass-21 burst-log line 483 cardinality cell P21=11 stale (should be 10, content-only per D-401(c)); F-P24-003 BC-INDEX v1.66 enum gap (D-403 absent from enumeration); F-P24-004 ARCH-INDEX v1.47 range "D-389..D-402" excludes D-403; F-P24-006 D-394 dispatch recurrence asymptotic (pass-24 dispatch DID update phase; D-403(c) continues to apply) |

**Resolution:** Per D-386 Option C (asymptotic convergence accepted), no further structural escalation this cycle. D-403 closes six adjacent defect dimensions. S-15.03 automation remains the structural remedy for v1.0-feature-engine-discipline-pass-2.

**Codified rules:**
- D-403(a): When a fix burst codifies cycle-governance decisions, ALL 4 indexes MUST acknowledge those decisions IN THE SAME BURST — including decisions codified that very burst. Inline-editing an existing index row does NOT satisfy the acknowledgment obligation for new decisions; a NEW changelog row must be appended.
- D-403(b): Verification grep patterns SHOULD match the actual file string form. Quoted YAML frontmatter (`version: "1.42"`) is NOT matched by bare pattern `v1.42`; correct pattern is `"1\.42"` or `version:.*1\.42`. The exact-integer obligation per D-402 stands.
- D-403(c): D-394+D-401(b) dispatch-side phase update continues to recur at each pass boundary per L-EDP1-007 Option C asymptotic. Tactical fix-burst remedy is acceptable; structural remedy requires orchestrator workflow amendment.

**Status:** Codified. D-403 closes the adjacent defect classes. L-EDP1-003 pattern continues at asymptotic boundary per D-386 Option C.

**Corrigendum (pass-24 fix burst — D-387 / D-400):** Layer-14 row "Same-burst Violation" inline-updated per D-400. See L-EDP1-016 for layer-15.

---

### L-EDP1-016 — 15th-layer L-EDP1-003 recurrence at D-403(a) self-application boundary (literal vs procedural acknowledgment distinction)

**Burst:** F5 pass-24 fix burst (codifies the lesson; recurrence was in pass-23 D-403 codification).

**Pattern:** D-403 was codified by the pass-23 fix burst. Pass-24 adversary found the 15th-layer L-EDP1-003 recurrence (F-P24-001) across the same index-acknowledgment class but at a finer granularity:

- **Dim (a) D-403(a) self-application — literal vs procedural distinction** (F-P24-001, HIGH): VP-INDEX v1.42 and STORY-INDEX v2.67 were not bumped in the pass-23 fix burst; they are entirely silent on D-403. BC-INDEX v1.66 and ARCH-INDEX v1.47 were appended in that burst but reference D-403(a) only in the form "per D-403(a)" (procedural rationale) — they do not acknowledge D-403 by literal ID in their enumeration. D-403(a) requires acknowledgment "AT THE TIME the burst lands" but does not distinguish procedural vs literal forms. D-404 (codified this burst) closes this gap by mandating LITERAL acknowledgment (D-NNN by ID in enumeration, not "per D-NNN(x)" reference).
- **Dim (b) Three-cell attestation block sibling miss** (F-P24-002, MEDIUM): Pass-21 burst-log has three sibling attestation cells in a block (lines 482/483/484). Pass-22 corrected line 482; pass-23 corrected line 484; line 483 (cardinality P21=11) was missed — the intermediate cell. Each pass corrected one boundary cell but left the interior cell uncorrected.
- **Dim (c) B-INDEX enum D-403 gap** (F-P24-003, MEDIUM): BC-INDEX v1.66 was authored per D-403(a) but only enumerates D-401+D-402 without adding D-403 to the literal enumeration. D-404 self-application: the v1.67 entry must explicitly enumerate D-403.
- **Dim (d) ARCH-INDEX range excludes D-403** (F-P24-004, MEDIUM): ARCH-INDEX v1.47 range "D-389..D-402" was authored per D-403(a) but excludes D-403 itself. The fix burst that codified D-403 appended a range entry that stops at D-402 — one shy of the burst's own new decision.

The 15-layer history:

| Layer | Burst | Rule Codified | Same-burst Violation |
|-------|-------|---------------|---------------------|
| 1 (pass-8) | D-381 | "fix burst MUST update STATE.md" | missed burst-log + INDEX |
| 2 (pass-9) | D-382 | "fix burst MUST update all 5 sibling files" | introduced intra-file content defects |
| 3 (pass-10) | D-383 | "intra-file content audit + sibling-pattern sweep" | trajectory cardinality + self-ref N missed |
| 4 (pass-11) | D-384 | "3 clarifications to D-383" | sub-trajectories stale; retroactive annotations |
| 5 (pass-12) | D-385 | "3 clarifications to D-383+D-384" | frontmatter schema drift; counting-basis change |
| 6 (pass-15) | D-387 | "structural-correction exception + sibling sweep" | sweep dimensions not enumerated; adjacent sibling-chain dimensions not covered |
| 7 (pass-16) | D-389+D-390 | "input-hash convention + CHANGELOG→last_amended rule" | enumerated in L-EDP1-009 |
| 8 (pass-17) | D-391+D-392 | "enumeration source mandatory + VP Lifecycle ≡ CHANGELOG" | second-source query absent; inlined BC list wrong (3 gaps); inlined VP list wrong (4 gaps) |
| 9 (pass-18) | D-393+D-394 | "independent re-derivation Grep query required + D-391 severity explicit + dispatch-side phase update" | F-P19-001 false-true attestation re VP-INDEX last_amended (corrigendum in L-EDP1-010) |
| 10 (pass-19) | D-395+D-396 | "file-state grep-back verification of Action claims + story-frontmatter↔STORY-INDEX sweep" | F-P20-001 dim-4 intent-mismatch (stale pass-18 narrative written; false-green Verification grep; corrigendum in L-EDP1-011) |
| 11 (pass-20) | D-397+D-398 | "intent-match sub-clause for D-395 Verification grep + Layer-N awaiting-audit convention" | F-P21-001: STATE.md:42 Current Phase cell still read "pass-19" after pass-20 fix burst updated only the adjacent Last Updated cell — sibling-cell sweep extent missed Current Phase cell (D-400 inline-replace) |
| 12 (pass-21) | D-399+D-400 | "canonical pass-N marker + Layer-N row update protocol" | F-P22-001 ARCH-INDEX cite-refresh silence (HIGH); F-P22-002 VP/STORY-INDEX cycle-sync silence; F-P22-003 BC-INDEX range/enumeration mismatch; F-P22-004 D-383 attestation gap; F-P22-005 counting-basis drift; F-P22-006 D-394 recurrence |
| 13 (pass-22) | D-401+D-402 | "cross-index sync convention + exact-count Verification + counting-basis + D-394 ownership" | F-P23-001 D-401(a) self-application failure (HIGH); F-P23-002 D-402 regex precision; F-P23-003 BC-INDEX inline-edit trail; F-P23-004 BC enum gap; F-P23-005 per-position P21 attestation; F-P23-006 D-394 dispatch recurrence |
| 14 (pass-23) | D-403 | "D-401(a) self-application enforcement + D-402 regex precision + D-394 asymptotic acknowledgment" | F-P24-001 D-403(a) self-application failure (HIGH); F-P24-002 pass-21 line 483 cardinality cell; F-P24-003 BC enum D-403 gap; F-P24-004 ARCH range excludes D-403 |
| 15 (pass-24) | D-404 | "literal acknowledgment enforcement — D-NNN by ID in all 4 index enumerations" | F-P25-001 D-404 itself not literally acknowledged in 4 indexes (HIGH); F-P25-002 6-site stale "VP-INDEX blocked" narrative post-TD-031 fix (HIGH); F-P25-003 4-cell STATE narrative dispatch mismatch; F-P25-004 decision-log range stale; F-P25-005 D-402 lower-bound recurrence; F-P25-006 self-referential greps; F-P25-PG1 dominant L-EDP1-003 sub-pattern across layers 13-16 |

**Resolution:** Per D-386 Option C (asymptotic convergence accepted), no further structural escalation this cycle. D-404 closes the literal-vs-procedural acknowledgment gap. S-15.03 automation (automated cross-index sync check at commit time) remains the structural remedy for v1.0-feature-engine-discipline-pass-2.

**Codified rules:**
- D-404: When a fix burst codifies D-NNN, ALL 4 indexes MUST acknowledge D-NNN BY LITERAL ID in their changelog enumeration within the same burst. "per D-NNN(x)" is procedural rationale, not literal acknowledgment. "Acknowledges D-NNN" or a range "D-AAA..D-NNN" containing D-NNN is literal acknowledgment. Version-bump may be no-content-change if the sole purpose is adding the literal acknowledgment.

**Status:** Codified. D-404 closes the literal-acknowledgment gap. L-EDP1-003 pattern continues at asymptotic boundary per D-386 Option C.

**Corrigendum (pass-25 fix burst — D-387 / D-400):** Layer-15 row "Same-burst Violation" inline-updated per D-400. See L-EDP1-017 for layer-16.

---

### L-EDP1-017 — 16th-layer L-EDP1-003 recurrence at D-404 self-application boundary

**Burst:** F5 pass-25 fix burst (codifies the lesson; recurrence was in pass-24 D-404 codification).

**Pattern:** D-404 was codified by the pass-24 fix burst. Pass-25 adversary found the 16th-layer L-EDP1-003 recurrence (F-P25-001). D-404 requires literal acknowledgment of D-NNN by ID in all 4 indexes within the same burst. The pass-24 fix burst that codified D-404 cited D-404 only as "per D-404" (procedural rationale) in the index changelog entries — the form that D-404 explicitly excludes from constituting literal acknowledgment. This is the same failure mode D-404 was authored to prevent.

**Additional finding (F-P25-002, HIGH):** The pass-24 fix burst successfully normalized VP-INDEX from v1.42 to v1.43 (TD-031 historical violations resolved via Write tool; hook passed cleanly at dd91044a). However, 6 sites in STATE.md and INDEX.md still carry the stale "VP-INDEX blocked at v1.42 / TD-031 OPEN" narrative, actively misrepresenting the artifact state at session start.

The 16-layer history:

| Layer | Burst | Rule Codified | Same-burst Violation |
|-------|-------|---------------|---------------------|
| 1 (pass-8) | D-381 | "fix burst MUST update STATE.md" | missed burst-log + INDEX |
| 2 (pass-9) | D-382 | "fix burst MUST update all 5 sibling files" | introduced intra-file content defects |
| 3 (pass-10) | D-383 | "intra-file content audit + sibling-pattern sweep" | trajectory cardinality + self-ref N missed |
| 4 (pass-11) | D-384 | "3 clarifications to D-383" | sub-trajectories stale; retroactive annotations |
| 5 (pass-12) | D-385 | "3 clarifications to D-383+D-384" | frontmatter schema drift; counting-basis change |
| 6 (pass-15) | D-387 | "structural-correction exception + sibling sweep" | sweep dimensions not enumerated; adjacent sibling-chain dimensions not covered |
| 7 (pass-16) | D-389+D-390 | "input-hash convention + CHANGELOG→last_amended rule" | enumerated in L-EDP1-009 |
| 8 (pass-17) | D-391+D-392 | "enumeration source mandatory + VP Lifecycle ≡ CHANGELOG" | second-source query absent; inlined BC list wrong (3 gaps); inlined VP list wrong (4 gaps) |
| 9 (pass-18) | D-393+D-394 | "independent re-derivation Grep query required + D-391 severity explicit + dispatch-side phase update" | F-P19-001 false-true attestation re VP-INDEX last_amended (corrigendum in L-EDP1-010) |
| 10 (pass-19) | D-395+D-396 | "file-state grep-back verification of Action claims + story-frontmatter↔STORY-INDEX sweep" | F-P20-001 dim-4 intent-mismatch (stale pass-18 narrative written; false-green Verification grep; corrigendum in L-EDP1-011) |
| 11 (pass-20) | D-397+D-398 | "intent-match sub-clause for D-395 Verification grep + Layer-N awaiting-audit convention" | F-P21-001: STATE.md:42 Current Phase cell still read "pass-19" after pass-20 fix burst updated only the adjacent Last Updated cell — sibling-cell sweep extent missed Current Phase cell (D-400 inline-replace) |
| 12 (pass-21) | D-399+D-400 | "canonical pass-N marker + Layer-N row update protocol" | F-P22-001 ARCH-INDEX cite-refresh silence (HIGH); F-P22-002 VP/STORY-INDEX cycle-sync silence; F-P22-003 BC-INDEX range/enumeration mismatch; F-P22-004 D-383 attestation gap; F-P22-005 counting-basis drift; F-P22-006 D-394 recurrence |
| 13 (pass-22) | D-401+D-402 | "cross-index sync convention + exact-count Verification + counting-basis + D-394 ownership" | F-P23-001 D-401(a) self-application failure (HIGH); F-P23-002 D-402 regex precision; F-P23-003 BC-INDEX inline-edit trail; F-P23-004 BC enum gap; F-P23-005 per-position P21 attestation; F-P23-006 D-394 dispatch recurrence |
| 14 (pass-23) | D-403 | "D-401(a) self-application enforcement + D-402 regex precision + D-394 asymptotic acknowledgment" | F-P24-001 D-403(a) self-application failure (HIGH); F-P24-002 pass-21 line 483 cardinality cell; F-P24-003 BC enum D-403 gap; F-P24-004 ARCH range excludes D-403 |
| 15 (pass-24) | D-404 | "literal acknowledgment enforcement — D-NNN by ID in all 4 index enumerations" | F-P25-001 D-404 itself not literally acknowledged in 4 indexes (HIGH); F-P25-002 6-site stale "VP-INDEX blocked" narrative post-TD-031 fix (HIGH); F-P25-003 4-cell STATE narrative dispatch mismatch; F-P25-004 decision-log range stale; F-P25-005 D-402 lower-bound recurrence; F-P25-006 self-referential greps; F-P25-PG1 dominant L-EDP1-003 sub-pattern across layers 13-16 |
| 16 (this, pass-25) | D-405 | "D-404 self-application correction + pattern-class recognition + S-15.03 PRIORITY-A elevation" | (awaiting pass-26 adversary fresh-context audit) |

**Resolution:** Per D-386 Option C (asymptotic convergence accepted), no further structural escalation this cycle. D-405 closes the D-404 self-application gap and codifies the PRIORITY-A elevation of S-15.03 for the next cycle. S-15.03 automation (automated cross-index sync check at commit time) is the structural remedy for v1.0-feature-engine-discipline-pass-2.

**Codified rules:**
- D-405(a): The pass-25 fix burst MUST literally acknowledge D-404 AND D-405 by literal ID in all 4 index changelog enumerations. The "per D-404" form is procedural rationale (explicitly excluded per D-404 + D-405(a)). Compliant form: "decision range D-AAA..D-405" or "Acknowledges D-404, D-405".
- D-405(b): The index-acknowledgment self-application defect class is the dominant L-EDP1-003 sub-pattern across layers 13-16. Prose codification has marginal value approaching zero for this class.
- D-405(c): S-15.03 cross-index-sync-at-commit-time check is PRIORITY-A in v1.0-feature-engine-discipline-pass-2 cycle planning.

**Status:** Codified. D-405 closes the D-404 self-application gap. L-EDP1-003 pattern continues at asymptotic boundary per D-386 Option C.
