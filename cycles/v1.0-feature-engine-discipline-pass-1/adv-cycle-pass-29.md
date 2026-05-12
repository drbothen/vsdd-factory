---
document_type: adversarial-review
producer: adversary
cycle: v1.0-feature-engine-discipline-pass-1
pass: 29
previous_review: adv-cycle-pass-28.md
prior-pass-classification: HIGH
prior-findings-count: 11
verdict: HIGH
findings_count:
  critical: 0
  high: 2
  medium: 4
  low: 3
  nitpick: 1
process_gap_count: 1
convergence_reached: false
timestamp: 2026-05-11T00:00:00Z
---

# Adversarial Review — F5 Pass 29

**Cycle:** v1.0-feature-engine-discipline-pass-1
**Pass:** 29
**Prior verdict:** HIGH (pass-28: 3H+2M+4L+1NIT+1PG)
**This verdict:** HIGH (2H+4M+3L+1NIT+1PG)
**Convergence reached:** false

## Findings

### F-P29-001 [HIGH] — Pass-28 Dim-7 false-green: `grep -c '28 F5 cycle-level reviews' STATE.md` actual=1 (claimed 2)

**File:** STATE.md (Concurrent Cycles row)
**Finding:** Pass-28 Dim-7 Verification `grep -c '28 F5 cycle-level reviews' STATE.md → 2 ✓` claimed count=2 (Concurrent Cycles row + Session Resume Checkpoint). Re-execution shows actual count=1. The Concurrent Cycles Notes cell reads "29 F5 cycle-level reviews" (updated by the pass-29 adversary dispatch STATE.md update). The Session Resume Checkpoint (lines 194-196) reads "F5 pass-28 fix burst COMPLETE" without the exact phrase "28 F5 cycle-level reviews". The Dim-7 Verification claimed two matches for a string present only once at the time of pass-28 fix burst Commit E.
**Severity:** HIGH (D-408(a) independent re-execution: Verification count cited does not match actual grep-c output)

### F-P29-002 [HIGH] — Pass-28 Dim-5 four false-greens: corrigendum prefix pattern returns 2 each (1 corrigendum body + 1 Verification line self-reference)

**File:** burst-log.md (pass-28 Dim-5 Verification block)
**Finding:** Pass-28 Dim-5 recorded four Verification lines for F-P28-001/002/003/004 corrigenda, each claiming count=1. Pattern form: `Corrigendum (pass-28 fix burst — D-387 / F-P28-NNN`. Actual count for each: 2. The corrigendum body contains the prefix string, AND the Dim-5 Verification line itself contains the identical prefix quoted in backticks inside the grep invocation. Per D-408(b) / D-409(a) (to be codified): when a Verification grep target string appears in the Verification line itself (because the Verification statement quotes the pattern it is testing in backtick form), the self-reference adds 1 to the count. Four separate Verification lines, each returning 2 (1 corrigendum body + 1 Verification line self-reference), were each claimed as count=1. All four are false-greens of the same structural class.
**Severity:** HIGH (4 × false-green Verification; D-408(a)+D-408(b) violation; class: Verification-line self-reference — distinct from corrigendum-body self-reference D-408(c) and layer-history table multi-match D-408(b))

### F-P29-003 [MEDIUM] — Pass-28 Dim-1 line-vs-occurrence semantics undocumented in Verification form

**File:** burst-log.md (pass-28 Dim-7 Verification block)
**Finding:** Dim-7 Verification `grep -c 'pass-28 fix burst COMPLETE' STATE.md → 3 ✓` with annotation "3 = current_step frontmatter + Last Updated + Session Resume Checkpoint". The count of 3 is plausible (3 lines). However, `grep -c` counts LINES not occurrences. For patterns appearing once per line this distinction is moot, but the Verification form does not document this. Future passes sweeping files where a pattern appears multiple times on one line will encounter count ambiguity. The form should note "grep -c (line count)" vs "grep -o | wc -l (occurrence count)" when the distinction matters. Severity MEDIUM because the specific count-3 annotation is site-specific and correct in this instance.
**Severity:** MEDIUM (D-402 documentation gap — line-vs-occurrence distinction not noted in Verification form)

### F-P29-004 [MEDIUM] — D-385 sub-trajectory sibling-sweep attestation cites only Concurrent Cycles; misses Phase Progress and INDEX.md

**File:** burst-log.md (pass-28 attestation, D-385 sub-rule 1 line)
**Finding:** Pass-28 attestation: `Sub-trajectory sibling sweep (D-385 sub-rule 1): STATE.md Concurrent Cycles row updated to "(pass-1..28): 29→...→12→11" ✓`. D-385 sub-rule 1 requires sweeping ALL locations where trajectory sub-strings appear. Missing from attestation scope: (a) Phase Progress rows (STATE.md lines ~63-82 contain per-pass HIGH/MEDIUM/COMPLETE rows with partial trajectory references); (b) INDEX.md Convergence Status trajectory; (c) burst-log cardinality attestation line. Prior passes 26-28 exhibit the same scoping. The D-385 sub-rule 1 attestation has consistently cited only the Concurrent Cycles site.
**Severity:** MEDIUM (D-385 sweep scope consistently narrower than the rule requires)

### F-P29-005 [MEDIUM] — Pass-28 Trigger Codifications closure-set omits F-P28-004 and F-P28-005

**File:** burst-log.md (pass-28 Trigger/Codifications block)
**Finding:** Codifications closure-set reads "Closes F-P28-001, F-P28-002, F-P28-003, F-P28-PG1." The burst also closed F-P28-004 (Extent miscount — via corrigendum appended in pass-27 corrigendum section, Dim-7) and F-P28-005 (L-EDP1-019 Layer-18 inline-replace — via Dim-2). Neither appears in the closure-set. The Trigger "Three HIGH findings" accurately counts only HIGH-severity findings, so the Trigger count is not wrong; the Codifications closure-set should enumerate ALL findings closed regardless of severity. Complete correct closure: F-P28-001/002/003/004/005/PG1. Per D-409(c): decision-log D-NNN closure-set MUST enumerate all findings closed by the burst, not just the primary/HIGH ones.
**Severity:** MEDIUM (closure-set incomplete; same gap pattern as F-P29-007 in D-408 decision-log entry)

### F-P29-006 [MEDIUM] — INDEX.md frontmatter missing timestamp, last_amended, status, phase fields

**File:** INDEX.md (frontmatter, lines 1-6)
**Finding:** INDEX.md frontmatter contains only: document_type, producer, cycle, version. Missing: `timestamp` (Z-suffix ISO-8601), `last_amended`, `status`, `phase`. All other cycle-level index artifacts (BC-INDEX, VP-INDEX, STORY-INDEX, ARCH-INDEX) carry these fields. Per D-409(b): cycle INDEX.md MUST carry frontmatter fields parallel to BC/VP/STORY/ARCH-INDEX. The INDEX.md is the authoritative convergence-status record updated at every pass; absence of these fields makes it non-queryable by tooling that filters by phase or status.
**Severity:** MEDIUM (structural frontmatter schema gap)

### F-P29-007 [LOW] — D-408 decision-log closure-set omits F-P28-004 and F-P28-005

**File:** decision-log.md (D-408 entry, line 88)
**Finding:** D-408 closing annotation: "Closes F-P28-001, F-P28-002, F-P28-003, F-P28-PG1." The pass-28 fix burst also closed F-P28-004 (Extent miscount) and F-P28-005 (L-EDP1-019 Layer-18 inline-replace). Per D-409(c): D-NNN closure-set MUST enumerate all findings closed by the burst. Severity LOW because the D-408 body text is correct and the omission is only in the closure annotation at end of the table row.
**Severity:** LOW (closure-set annotation incomplete)

### F-P29-008 [LOW] — STATE.md dtu_assessment date stale (2026-04-25)

**File:** STATE.md (frontmatter, line 17)
**Finding:** `dtu_assessment: 2026-04-25`. DTU status unchanged (dtu_required: false). LOW because no functional impact — DTU status is genuinely not reassessed mid-session.
**Severity:** LOW (stale date field; no functional impact)

### F-P29-009 [LOW] — Active Branches SHA placeholder not replaced as deferred in pass-28 Deferrals

**File:** STATE.md (Active Branches table) + burst-log.md (pass-28 Deferrals)
**Finding:** Pass-28 Deferrals: "F-P28-007 (SHA placeholder — LOW; STATE.md Active Branches roll-forward to pass-28 in Commit E ✓)". The ✓ claims resolution. STATE.md Active Branches row for factory-artifacts still reads `(see git log)` in SHA column. The deferral was claimed resolved but the SHA was not updated. This is a false-green deferral resolution.
**Severity:** LOW (false-green deferral resolution; parallel defect class to Dim Verification false-greens)

### F-P29-010 [NITPICK] — INDEX.md Adversarial Reviews table: passes 3-7 lack +NPG suffix form

**File:** INDEX.md (Adversarial Reviews table)
**Finding:** Rows for passes 3-7 use plain integer form (e.g., "11 (2C+6H+3M)"). Rows for passes 8+ consistently use "+NPG" when process gaps present (e.g., "3 (2M+1L) +3PG"). Cosmetic inconsistency from D-387 retroactive verdict corrections.
**Severity:** NITPICK

### F-P29-011 [LOW] — F-P28-007 false-green deferral: SHA placeholder marked resolved but not updated

*(Covered above in F-P29-009 — combined finding.)*

---

## Process Gap

### F-P29-PG1 — D-408(a) self-application boundary: Verification-line self-referential grep pattern

**Classification:** process-gap
**Finding:** D-408(a) mandates independent re-execution of all Dim Verification greps. When a Verification line quotes its own target pattern in backticks (e.g., `` `grep -c 'Corrigendum (pass-28 fix burst — D-387 / F-P28-001' burst-log.md` ``), the grep command being documented matches the Verification line itself. This creates a structurally unavoidable +1: any unbounded grep-c over the file will count the Verification line as a match alongside the corrigendum body it is verifying. D-408(a) requires re-execution but does not specify whether the Verification line's own self-referential match should be excluded from the reported count. F-P29-002 is the manifestation (4 × count=1 claimed, actual=2 each). D-409(a) codifies two valid resolution forms: (i) count = N+1 with explicit annotation identifying the +1 as the Verification line self-reference; OR (ii) use a more specific bounded pattern that excludes the Verification line (e.g., anchor with leading whitespace or exclude lines containing 'Verification:'). Default convention going forward: form (i) explicit annotation. Closes F-P29-PG1.

---

## Part A — Policy Rubric

| Policy | Status | Note |
|--------|--------|------|
| POLICY 1 (append-only IDs) | PASS | decision-log D-NNN sequence intact |
| POLICY 2 (D-402 exact integer counts) | FAIL | F-P29-001: claimed 2, actual 1; F-P29-002: claimed 1 each ×4, actual 2 each |
| POLICY 3 (state-manager final commit) | PASS | Commit E present |
| POLICY 4 (D-408(a) independent re-execution) | FAIL | F-P29-001 and F-P29-002 are D-408(a) failures |
| POLICY 5 (D-404 unconditional acknowledgment) | PENDING | D-409 to be codified; 4 indexes must acknowledge |

---

## Novelty Assessment

F-P29-002 introduces a third distinct Verification self-reference sub-variant (Verification-line self-reference via backtick quoting), distinct from D-408(b) layer-history table multi-match and D-408(c) corrigendum-body self-referential count. D-409(a) closes this variant. F-P29-001 is 20th-layer L-EDP1-003 recurrence. F-P29-003 through F-P29-011 are established defect classes.

---

## Scope

factory-artifacts branch; cycle v1.0-feature-engine-discipline-pass-1; pass-28 fix burst outputs. Files reviewed: burst-log.md (pass-28 section), STATE.md, decision-log.md (D-408), lessons.md (L-EDP1-020), INDEX.md.
