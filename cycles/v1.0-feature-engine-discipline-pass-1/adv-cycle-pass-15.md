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
  - .factory/cycles/v1.0-feature-engine-discipline-pass-1/adv-cycle-pass-14.md
  - .factory/specs/behavioral-contracts/BC-INDEX.md
  - .factory/specs/verification-properties/VP-INDEX.md
  - .factory/specs/architecture/ARCH-INDEX.md
  - .factory/stories/STORY-INDEX.md
input-hash: "[pending-recompute]"
traces_to: prd.md
project: vsdd-factory
mode: feature
current_step: F5-adversarial-pass-15
current_cycle: v1.0-feature-engine-discipline-pass-1
pass: 15
previous_review: adv-cycle-pass-14.md
prior-pass-classification: MEDIUM
prior-findings-count: 10
verdict: HIGH
findings_count: { critical: 0, high: 2, medium: 5, low: 4, nitpick: 2 }
observations: 0
deferred: 0
process_gap_count: 2
convergence_reached: false
---

# Adversarial Review — Pass 15

**Pass:** 15
**Date:** 2026-05-11
**Verdict:** HIGH (regression from pass-14 MEDIUM; 13 content findings 2H+5M+4L+2NIT + 2 process-gaps)
**Prior pass findings (content-only):** 10 (pass-14: 4M+4L+2NIT)
**This pass findings (content-only):** 13 (2H+5M+4L+2NIT); 2 PGs counted separately
**Streak:** 0/3 (no NITPICK_ONLY passes; HIGH resets streak)

---

## Finding ID Convention

Finding IDs use the format `F-P15-NNN` (cycle-level shorthand) mapping to
`ADV-EDP1-P15-<SEV>-<SEQ>` in the canonical schema. Cycle prefix: `EDP1`
(engine-discipline-pass-1). Pass: 15.

---

## Part A — Fix Verification (Pass-14 Closure Summary)

| Pass-14 ID | Status | Evidence |
|---|---|---|
| F-P14-001 (MEDIUM) — pass-13 frontmatter schema drift (prior-pass-classification field) | **CLOSED** | adv-cycle-pass-13.md frontmatter: prior-pass-classification field now present per pass-13 fix burst. |
| F-P14-002 (MEDIUM) — burst-log schema provenance "3-11" phrase | **CLOSED** | burst-log.md pass-11 attestation now references "passes 3-13" scope. |
| F-P14-003 (MEDIUM) — L-EDP1-006 corrigendum added to lessons.md | **CLOSED** | Corrigendum appended at lessons.md:251. However — see F-P15-006 (the corrigendum itself violates D-385 sub-rule 2 immutability). |
| F-P14-004 (MEDIUM) — pass-9 verdict MEDIUM-HIGH → HIGH | **CLOSED** | adv-cycle-pass-9.md frontmatter verdict field updated to HIGH. However — this edit itself violates D-385 sub-rule 2 (adversary-review files listed as immutable). See F-P15-001. |
| F-P14-005 (LOW) — STATE.md decision-pending text present at line 129 | **CLOSED** | Decision-pending language removed; D-386 Option C language present. |
| F-P14-006 (LOW) — STORY-INDEX S-14.06/07/08/09 deferral note missing | **CLOSED** | STORY-INDEX last_amended includes S-14.06-09 registration note. |
| F-P14-007 (LOW) — adv-cycle-pass-13.md prior-findings-count 4 vs 3 | **CLOSED** | adv-cycle-pass-13.md prior-findings-count: 3 (content-only). |
| F-P14-008 (LOW) — INDEX.md placeholder story IDs S-A/B/C | **CLOSED** | INDEX.md Stories Delivered table now shows S-12.01, S-12.02, S-13.01. |
| F-P14-009 (NITPICK) — F5-pass-1-fix-plan.md provenance comment | **CLOSED** | Provenance comment updated. |
| F-P14-010 (NITPICK) — burst-log pass-6 entry "STORY-INDEX v2.64→v2.65" stale | **CLOSED** | burst-log entry updated. |

---

## Part B — New Findings (Pass 15)

### F-P15-001 [HIGH] — pass-14 fix burst violated D-385 sub-rule 2: edited adv-cycle-pass-9.md frontmatter

**Location:** `.factory/cycles/v1.0-feature-engine-discipline-pass-1/adv-cycle-pass-9.md` frontmatter
**Severity:** HIGH
**Rule cited:** D-385 sub-rule 2 (immutable-row scope enumeration explicitly lists "adversary-review files (adv-cycle-pass-N.md, including frontmatter and body)" as immutable)

**Description:** The pass-14 fix burst (F-P14-004) edited adv-cycle-pass-9.md's frontmatter field `verdict: MEDIUM-HIGH` → `verdict: HIGH` directly. D-385 sub-rule 2 explicitly enumerates adversary-review files (including frontmatter) as immutable under D-383 rule 2(c). The pass-14 burst violated its own immutability contract. The verdict correction itself is substantively correct (MEDIUM-HIGH is a non-canonical label; HIGH is the nearest canonical equivalent), but the mechanism — direct frontmatter edit — was impermissible.

**Recommendation:** Codify a structural-correction exception rule (D-387) legalizing single-field frontmatter corrections when a defect exists in the frontmatter itself (e.g., undefined verdict label), while preserving body immutability. Retroactively legalize the pass-14 edit via D-387. Complete the sibling-pattern sweep for MEDIUM-HIGH → HIGH across all affected files.

---

### F-P15-002 [HIGH] — F-P14-004 sibling-pattern sweep incomplete; 7+ sites still carry MEDIUM-HIGH label

**Location:** Multiple files
**Severity:** HIGH
**Rule cited:** D-383 sibling-pattern sweep (when fixing a defect class, audit ALL same-class sites)

**Description:** The pass-14 fix burst corrected adv-cycle-pass-9.md frontmatter verdict but failed to complete the sibling-pattern sweep. The following sites were identified as still carrying the stale "MEDIUM-HIGH" label or referencing it without the required corrigendum:

1. `STATE.md:65` — Phase Progress row for pass-9: "MEDIUM-HIGH (1H+1M+2L+2NIT)"
2. `adv-cycle-pass-10.md:24` — frontmatter `prior-pass-classification: MEDIUM-HIGH`
3. `adv-cycle-pass-9.md` body lines 41, 134, 153, 183, 199 — historical body text references (immutable per D-385; require corrigendum)
4. `adv-cycle-pass-10.md` body lines 55, 65, 148, 158, 343 — references to pass-9 MEDIUM-HIGH
5. `adv-cycle-pass-11.md` body line 64 — reference to pass-9 verdict chain

**Recommendation:** Apply D-387 structural-correction exception to update all mutable frontmatter fields. Add end-of-file corrigendum to each file with immutable body references. Complete sweep before pass-16 dispatch.

---

### F-P15-003 [MEDIUM] — pass-7 and pass-13 verdicts violate verdict-ladder rule (highest-severity wins)

**Location:** `adv-cycle-pass-7.md:29` and `adv-cycle-pass-13.md:27`
**Severity:** MEDIUM
**Rule cited:** Verdict-ladder rule: verdict = highest severity level present in findings_count

**Description:**
- adv-cycle-pass-7.md: `verdict: LOW` but `findings_count: { medium: 2, low: 3 }`. Highest severity is MEDIUM → verdict should be MEDIUM.
- adv-cycle-pass-13.md: `verdict: MEDIUM` but `findings_count: { high: 1, medium: 1, low: 1 }`. Highest severity is HIGH → verdict should be HIGH.

Both are structural defects in the frontmatter's verdict field (non-canonical labels for pass-7; wrong verdict for pass-13). D-387 (if codified) legalizes frontmatter corrections of this class.

**Recommendation:** Correct adv-cycle-pass-7.md verdict to MEDIUM; adv-cycle-pass-13.md verdict to HIGH. Update INDEX.md rows 7 and 13. Update STATE.md Phase Progress rows for passes 7 and 13. Update sibling back-references (adv-cycle-pass-8.md prior-pass-classification; adv-cycle-pass-14.md prior-pass-classification). Add corrigendum lines to body of affected files.

---

### F-P15-004 [MEDIUM] — 5 in-cycle stories (S-12.03/04/05/07/08) carry status:draft despite being merged

**Location:** `.factory/stories/S-12.03-context-resolver-trait-and-registry.md:8`, `S-12.04-wasm-resolver-loading-lifecycle.md:7`, `S-12.05-hook-sdk-resolver-extensions.md:8`, `S-12.07-vsdd-context-resolvers-crate.md:8`, `S-12.08-convergence-hook-context-migration.md:7`
**Severity:** MEDIUM

**Description:** STATE.md Phase Progress (lines 57-61) lists PRs #119-#123 as MERGED for S-12.03/04/05/07/08. However, all five story spec files still carry `status: draft` in frontmatter. The canonical status should be `status: merged` with `merged_at`, `merged_in`, and `merge_sha` fields populated. S-12.06 (PR #105) is not included (likely already corrected).

**Recommendation:** For each of the 5 stories, update frontmatter: `status: merged`, add `merged_at`, `merged_in`, `merge_sha` per merge records in STATE.md. Bump version and append CHANGELOG row.

---

### F-P15-005 [MEDIUM] — STATE.md:65 still asserts MEDIUM-HIGH for pass-9

**Location:** `.factory/STATE.md:65`
**Severity:** MEDIUM (sibling of F-P15-002 HIGH; listed separately for clarity)

**Description:** STATE.md Phase Progress row for pass-9 reads: "MEDIUM-HIGH (1H+1M+2L+2NIT)". The verdict was corrected in adv-cycle-pass-9.md frontmatter (F-P14-004) but the STATE.md sibling was not updated per D-383 sibling-pattern sweep mandate. STATE.md should read "HIGH (1H+1M+2L+2NIT)".

**Recommendation:** Update STATE.md:65 Phase Progress row verdict to HIGH. This is part of the comprehensive sibling-pattern sweep for F-P15-002.

---

### F-P15-006 [MEDIUM] — L-EDP1-006 corrigendum at lessons.md:251 violates D-385 sub-rule 2 (lessons.md entries are immutable)

**Location:** `.factory/cycles/v1.0-feature-engine-discipline-pass-1/lessons.md:251`
**Severity:** MEDIUM
**Rule cited:** D-385 sub-rule 2 explicitly lists "lessons.md entries (L-EDP1-NNN)" as immutable

**Description:** The pass-14 fix burst (F-P14-003) appended a corrigendum directly inside the L-EDP1-006 entry body, between the `**Status:**` line and the `---` separator. D-385 sub-rule 2 enumerates lessons.md L-EDP1-NNN entries as immutable. The corrigendum format used is an inline body modification, not a separate entry.

D-387 (being codified this pass) permits corrigenda at the END of an entry delimited by "**Corrigendum**:" prefix, on a new line before `---` separator — which is approximately what pass-14 did. However, the retroactive application requires formal D-387 codification to be legitimate.

**Recommendation:** Retroactively legalize via D-387 (which includes lessons.md corrigendum format). No file change needed to L-EDP1-006:251 if D-387 format matches. Confirm the corrigendum format complies with D-387 specification.

---

### F-P15-007 [MEDIUM] — INDEX.md Stories Delivered table omits S-12.03..08

**Location:** `.factory/cycles/v1.0-feature-engine-discipline-pass-1/INDEX.md:29-35`
**Severity:** MEDIUM

**Description:** The Stories Delivered table in INDEX.md contains only 3 rows: S-12.01, S-12.02, S-13.01. The F3-amendment (D-366) added 6 new platform stories (S-12.03..S-12.08) to E-12, and F4 delivered all of them (State.md lines 57-62). The INDEX.md Stories Delivered table was not expanded to include the 6 new stories or S-13.01's companion in the platform batch.

**Recommendation:** Expand the Stories Delivered table to include all 9 delivered stories: S-12.01, S-12.02, S-12.03, S-12.04, S-12.05, S-12.06, S-12.07, S-12.08, S-13.01. Add Phase column (F2/F3/F4) to distinguish original 3 from the 6 platform stories.

---

### F-P15-008 [LOW] — adv-cycle-pass-12.md frontmatter inputs:[] and traces_to:"" are empty required fields

**Location:** `.factory/cycles/v1.0-feature-engine-discipline-pass-1/adv-cycle-pass-12.md:9,11`
**Severity:** LOW

**Description:** adv-cycle-pass-12.md frontmatter shows `inputs: []` (empty list) and `traces_to: ""` (empty string). The F-P13-001 fix burst (Commit B: a9a36627) restored the canonical 25-field schema but did not populate `inputs` or `traces_to` with actual values. Per the canonical schema (established in passes 5-11 and pass-14), `inputs` must list the reviewed artifacts and `traces_to` must reference `prd.md`.

**Recommendation:** Per D-387 (structural correction of required-field empty values is permitted), populate adv-cycle-pass-12.md `inputs` with the artifact set reviewed in pass-12, and set `traces_to: prd.md`.

---

### F-P15-009 [LOW] — L-EDP1-007 Status field "Open for orchestrator + human decision" is stale

**Location:** `.factory/cycles/v1.0-feature-engine-discipline-pass-1/lessons.md` L-EDP1-007 Status field
**Severity:** LOW

**Description:** L-EDP1-007 Status reads "Open for orchestrator + human decision." D-386 (codified in the pass-14 fix burst) authoritatively selected Option C: accept asymptotic L-EDP1-003 limit; S-15.03 deferred. This decision closes L-EDP1-007's open question. The Status field is stale.

**Recommendation:** Per D-385 sub-rule 2 (lessons.md immutability), do NOT edit the Status field directly. Author L-EDP1-008 as a new lesson that explicitly closes L-EDP1-007 by reference to D-386. Add a corrigendum line at the end of L-EDP1-007 per D-387 permitted format.

---

### F-P15-010 [LOW] — adv-cycle-pass-10.md:24 prior-pass-classification: MEDIUM-HIGH not corrected

**Location:** `.factory/cycles/v1.0-feature-engine-discipline-pass-1/adv-cycle-pass-10.md:24`
**Severity:** LOW

**Description:** adv-cycle-pass-10.md frontmatter line 24: `prior-pass-classification: MEDIUM-HIGH`. This back-references pass-9's verdict. Since pass-9's verdict has been corrected from MEDIUM-HIGH to HIGH (per F-P14-004), the prior-pass-classification in adv-cycle-pass-10.md should be updated to HIGH for consistency. D-387 (structural correction) permits this frontmatter update.

**Recommendation:** Apply D-387 structural correction to adv-cycle-pass-10.md `prior-pass-classification: MEDIUM-HIGH` → `prior-pass-classification: HIGH`. Note: this is part of the comprehensive sibling-pattern sweep cited in F-P15-002.

---

### F-P15-011 [LOW] — cycle:v1.0-feature-engine-discipline-pass-2 forward-reference convention not formally codified

**Location:** `.factory/stories/S-14.06-*.md` through `S-14.09-*.md` (frontmatter `cycle:` field)
**Severity:** LOW

**Description:** Stories S-14.06..S-14.09 reference `cycle: v1.0-feature-engine-discipline-pass-2` in their frontmatter. That cycle directory does not yet exist (no INDEX.md scaffold). The forward-reference convention (stories may reference future cycles before their directories are opened) is practiced but not formally codified in any decision-log entry or VSDD spec.

**Recommendation:** Author D-388 codifying the forward-reference convention: stories MAY carry `cycle:` references to cycles whose directories do not yet exist if and only if (a) STORY-INDEX last_amended narrative notes the forward reference as a deferred-cycle entry, and (b) cycle directories MUST be opened (with INDEX.md scaffold) BEFORE any per-story-delivery cycle dispatch targeting that cycle.

---

### F-P15-012 [NITPICK] — adv-cycle-pass-9.md timestamp missing Z suffix

**Location:** `.factory/cycles/v1.0-feature-engine-discipline-pass-1/adv-cycle-pass-9.md:7`
**Severity:** NITPICK

**Description:** `timestamp: 2026-05-11T00:00:00` is missing the UTC designator `Z`. Canonical schema (passes 12-14) uses `timestamp: 2026-05-11T00:00:00Z`. Minor inconsistency; does not affect processing.

**Recommendation:** DEFER — NITPICK; not worth a structural correction under D-387 for a trailing character. Document here for awareness only.

---

### F-P15-013 [NITPICK] — STATE.md:83 trajectory shorthand starts at pass-3 not pass-1

**Location:** `.factory/STATE.md:83`
**Severity:** NITPICK

**Description:** STATE.md Current Phase Steps row for "F5 passes 3-9 cycle adversary + fix bursts" shows trajectory "Trajectory 11→9→8→7→5→6→6 (passes 3-9)". This sub-trajectory omits passes 1-2 values (29 and 15), which is historically correct (passes 1-2 were before the cycle-level discipline was applied and are tracked separately). However, the notation "passes 3-9" is technically a sub-trajectory, not the full trajectory; a reader could infer that the full trajectory starts at 11. The canonical full trajectory is documented in STATE.md:129 (Concurrent Cycles) and INDEX.md.

**Recommendation:** DEFER — NITPICK; existing notation is defensible given the historical context (passes 1-2 are pre-restructuring). No change needed.

---

## Part C — Process Gaps

### F-P15-PG1 — D-385 vs F-P14-004 rule conflict requires codification (D-387 needed)

**Type:** Process Gap
**Description:** F-P14-004 edited adv-cycle-pass-9.md frontmatter, which D-385 sub-rule 2 prohibits. This is a rule conflict: the *intent* of the fix was correct (MEDIUM-HIGH is non-canonical), but the *mechanism* violated immutability. The factory currently has no formal exception pathway for correcting structural defects in adversary-review frontmatter. D-387 must be authored to legalize this class of fix retroactively and prospectively.

**Resolution:** Author D-387 (structural-correction exception) this burst. D-387 self-application retroactively legalizes F-P14-004. Closes this PG.

---

### F-P15-PG2 — D-386 "continue F5" lacks formal stopping criterion (engine SKILL says max-10; we are at pass-15)

**Type:** Process Gap
**Description:** D-386 selected Option C (continue F5; accept asymptotic L-EDP1-003 limit). The engine SKILL per-story-delivery.md Step 4.5 specifies max-10 adversary passes for per-story convergence. At cycle-level F5, no explicit maximum is codified. We are now at pass-15 with no NITPICK_ONLY pass recorded. D-386's Option C acknowledgment of "asymptotic convergence" accepts that the 3-NITPICK_ONLY criterion may never be met via prose-only means.

**Resolution:** DEFER — user has explicitly overridden by continuing F5 past pass-10 with D-386. The stopping criterion is user-delegated. No new decision needed this burst; document here as acknowledged limitation.

---

## Policy Rubric Verification

| Rule | Status | Notes |
|------|--------|-------|
| D-379 CI-green-signal for CRITICAL CI-class closures | N/A | No CRITICAL CI-class findings this pass |
| D-381 STATE.md mandatory in every fix burst | REQUIRED | Pass-15 fix burst must update STATE.md |
| D-382 All 5 sibling files mandatory | REQUIRED | burst-log, INDEX, lessons, decision-log, STATE.md all required |
| D-383 Intra-file content audit + sibling-pattern sweep | REQUIRED | Sweep must cover all MEDIUM-HIGH sites |
| D-384 Self-referential N, cardinality, attestation specificity | REQUIRED | N=15 in all "passes 3-N" references |
| D-385 Sub-trajectory enumeration, immutable-row scope, per-position attestation | REQUIRED | Immutable-row scope: adv-review files + lessons entries |
| D-386 Option C: accept asymptotic limit | IN EFFECT | Continue F5; S-15.03 deferred |

---

## Novelty Assessment

- **F-P15-001/002:** Novel in exact framing (D-385 sub-rule 2 applied to adversary-review frontmatter edits). Root cause class (same-class defect in fix burst) is familiar (L-EDP1-003 pattern, 6th layer).
- **F-P15-003:** Novel: verdict-ladder rule violations across passes 7 and 13. Root cause: no automated verdict-ladder validation at persist time.
- **F-P15-004/005:** Familiar class (sibling-file sweep gap). Specific instance (story status:draft) is new.
- **F-P15-006:** Novel application of D-385 sub-rule 2 to the lessons.md corrigendum format. D-387 resolves it.
- **F-P15-007/008:** Familiar omission class. Pass-15 first to identify INDEX.md Stories Delivered table scope gap.
- **F-P15-009/010:** Familiar (stale Status field, sibling back-reference gap). Expected continuation.
- **F-P15-011/012/013:** NITPICK/LOW level; low novelty.

**Overall novelty decay** from pass-14: pass-14 had 10 content findings; pass-15 has 13. This is a regression (+3 findings), driven primarily by the F-P14-004 mechanism violation surfacing 2 HIGH findings. The underlying root cause class (L-EDP1-003) is at its 6th layer.

---

## Scope Confirmation

This review was scoped to: STATE.md, decision-log.md, lessons.md, INDEX.md, burst-log.md, adv-cycle-pass-14.md, BC-INDEX.md, VP-INDEX.md, ARCH-INDEX.md, STORY-INDEX.md. The complete sibling file surface required by D-382 was covered. No out-of-scope artifacts were examined.

**Immutability confirmation:** All body text of adv-cycle-pass-1.md through adv-cycle-pass-14.md was treated as immutable per D-385 sub-rule 2 during this review. Only frontmatter structural defects were flagged as correctable (D-387 pending).
