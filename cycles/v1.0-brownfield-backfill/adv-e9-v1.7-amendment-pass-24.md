---
document_type: adversarial-review
pass: 24
verdict: SUBSTANTIVE
epic: E-9
version_reviewed: "1.23"
angle: convention-meta-audit
date: 2026-05-05
findings_high: 1
findings_med: 6
findings_low: 3
adr_013_clock: 0_of_3
clock_action: RESET
---

# Pass-24 Adversarial Review — E-9 v1.23 Amendment Surface

**Angle:** Convention-meta audit (NEW per TD-VSDD-057) — audits lessons-corpus artifacts (lessons.md + open-backlog-post-rc8.md) themselves for coherence defects introduced by the 20 TD-VSDD codification bursts. Treats canonical reference artifacts as primary audit targets; verifies they are internally coherent and bidirectionally consistent.

**Verdict:** SUBSTANTIVE — 1 HIGH / 6 MED / 3 LOW. ADR-013 clock RESET to 0_of_3.

---

## HIGH Findings

### H-P24-001: BC-1.05.036 EC-006 annotation style inconsistency (TD-VSDD-076 self-violation)

**Location:** BC-1.05.036.md
- Line 88 (§Edge Cases EC-006 payload field type check): uses `[ ... ]` bracket annotation for `truncated: bool` inline comment
- Line 49 (§Postconditions item 2): uses `/* ... */` C-style comment annotation for the identical `truncated: bool` inline comment

**Defect:** Both sites annotate the SAME field (`truncated: bool`) with the SAME explanatory text about reserved-future-ABI-break semantics. The annotations use different syntactic forms: `[...]` in EC-006 vs `/* ... */` in §Postconditions item 2. This is an intra-document semantic-sibling inconsistency — precisely the class TD-VSDD-076 mandates sweeping. TD-VSDD-076 self-violation: the v1.22/v1.23 sibling-sweep bursts corrected sibling sections but missed annotation-style drift between §Postconditions and §Edge Cases for this specific field.

**Severity:** HIGH — intra-document convention inconsistency directly violating the most recently codified lesson (TD-VSDD-076); BC-1.05.036 is in the amendment surface.

**Fix:** Align EC-006 line 88 to use `/* ... */` form matching §Postconditions item 2.

---

## MED Findings (all in lessons-corpus artifacts — out of amendment scope but canonical references)

### M-P24-001: TD-VSDD-069 stub entry in open-backlog-post-rc8.md

**Location:** open-backlog-post-rc8.md (the TD-VSDD-069 bullet)

**Defect:** `- **TD-VSDD-069 Line-accuracy extension to recursive-scrub.**` — body content is missing. The full lesson body (source citation, pattern description, codification) was merged into the TD-VSDD-070 entry (which concatenates both TD-VSDD-069 and TD-VSDD-070 content). TD-VSDD-069 is effectively unenforceable as a standalone entry; a reader locating it by number finds no content.

**Severity:** MED — canonical reference artifact incoherent; stub makes TD-VSDD-069 appear uncodified.

### M-P24-002: TD-VSDD-071 stub entry in open-backlog-post-rc8.md

**Location:** open-backlog-post-rc8.md (the TD-VSDD-071 bullet)

**Defect:** `- **TD-VSDD-071 OQ-table propagation hook.**` — body content missing. Full content merged into TD-VSDD-072 entry.

**Severity:** MED — same class as M-P24-001.

### M-P24-003: TD-VSDD-075 stub entry in open-backlog-post-rc8.md

**Location:** open-backlog-post-rc8.md (the TD-VSDD-075 bullet)

**Defect:** `- **TD-VSDD-075** (last_amended dependent-citation propagation requirement + source-code-verification discipline).` — body content missing. Full content merged into TD-VSDD-076 entry.

**Severity:** MED — same class as M-P24-001.

### M-P24-004: TD-VSDD-073/075/076 entries placed after wrong H2 boundary in open-backlog-post-rc8.md

**Location:** open-backlog-post-rc8.md

**Defect:** TD-VSDD-073, TD-VSDD-075, and TD-VSDD-076 entries appear AFTER the `## Lessons codified during the cycle (needing follow-up in lessons.md)` H2 boundary, mixed with the lessons table. These are new TD-VSDD entries from Phase D-4 and should be under the `## New from Phase D-4 (2026-05-05)` H2 section alongside TD-VSDD-056 through TD-VSDD-074. The section-boundary violation makes the D-4 TD-VSDD register visually fragmented.

**Severity:** MED — structural incoherence in canonical backlog artifact.

### M-P24-005: Orphaned `[codified]` markers in lessons.md (5+ sites)

**Location:** lessons.md

**Defect:** Multiple `[codified] by D-NNN lessons.md append.` markers appear displaced from their associated lesson bodies:
- Line 414: `**[codified]** by D-252 lessons.md append.` follows TD-VSDD-067 lesson (TD-VSDD-067 was codified at D-254, not D-252; marker is misattributed or orphaned from TD-VSDD-066 lesson)
- Line 484: `**[codified]** by D-257 lessons.md append.` follows TD-VSDD-071 lesson but should follow TD-VSDD-070 (D-257 codified TD-VSDD-070; TD-VSDD-071 was codified at D-258)
- Line 539: `**[codified]** by D-261 lessons.md append.` follows TD-VSDD-074 lesson but should follow TD-VSDD-073

**Severity:** MED — orphaned markers make audit trail ambiguous; downstream bursts citing lessons as "codified at D-NNN" cannot verify against markers.

### M-P24-006: TD-VSDD-074 Source field divergence between lessons.md and open-backlog-post-rc8.md

**Location:** lessons.md line ~535 vs open-backlog-post-rc8.md TD-VSDD-074 entry

**Defect:** open-backlog-post-rc8.md TD-VSDD-074 entry Source field includes `+ PG-P20-001` in the source citation (`Source: D-263 pass-20 finding M-P20-002 + PG-P20-001`). lessons.md LESSON: TD-VSDD-073 must extend to BCs Source field reads `Source: D-263 pass-20 finding M-P20-002` — missing `+ PG-P20-001`. Bidirectional drift on Source field. `PG-P20-001` is a real process-gap finding from pass-20; it should appear in both canonical artifacts.

**Severity:** MED — bidirectional inconsistency in canonical reference artifacts; future audits tracing TD-VSDD-074 origin will see different sources depending on which artifact is consulted.

---

## LOW Findings

### L-P24-001: Non-monotonic TD-VSDD-NNN ordering in open-backlog-post-rc8.md

**Location:** open-backlog-post-rc8.md

**Defect:** Within the `## New from Phase D-4` section, the TD-VSDD-NNN entries are not in strictly ascending order. Specifically: TD-VSDD-073 and TD-VSDD-075 and TD-VSDD-076 appear after the `## Lessons codified` H2 break, and TD-VSDD-074 appears before TD-VSDD-073 in the main D-4 block. Expected monotonic order: ...069, 070, 071, 072, 073, 074, 075, 076.

**Severity:** LOW — cosmetic ordering; does not affect enforceability but reduces scan efficiency.

### L-P24-002: TD-VSDD-072 body in open-backlog-post-rc8.md conflates TD-VSDD-071 content

**Location:** open-backlog-post-rc8.md TD-VSDD-072 entry

**Defect:** The TD-VSDD-072 bullet contains text that properly belongs to TD-VSDD-071 (OQ-table propagation hook content) concatenated with the TD-VSDD-072 body (retired-figure body-grep extension). The concatenation makes TD-VSDD-072's entry twice as long as intended and mixes two distinct lessons.

**Severity:** LOW — same class as M-P24-001/002/003 (stub/merge artifact); classified LOW because TD-VSDD-072 body content itself is present even if prefixed by TD-VSDD-071 content.

### L-P24-003: TD-VSDD-076 lesson title in lessons.md self-references TD-VSDD-075 without anchor precision

**Location:** lessons.md line 564

**Defect:** Lesson title reads `### LESSON: Intra-document semantic-sibling sweep (TD-VSDD-076 extension to TD-VSDD-075)`. The relationship "extension to TD-VSDD-075" is vague — TD-VSDD-075 covers two orthogonal sub-rules (source-code-verification + dependent-citation-propagation). TD-VSDD-076 specifically extends the dependent-citation-propagation discipline from inter-document to intra-document scope. The title should clarify the scope of extension.

**Severity:** LOW — imprecise cross-reference in lesson title; does not affect enforceability.

---

## Pre-flight Checks (amendment surface only)

- Frontmatter `version: "1.23"` matches latest non-reserved changelog summary table row: PASS
- All amendment-surface arch docs carry `last_amended:` field: PASS (all 4 arch docs have `last_amended: 2026-05-05`)
- BC-1.05.036 Postconditions and sibling sections for TIMEOUT/OUTPUT_TOO_LARGE error-path events: consistent (H-P22-001 fix applied correctly) PASS
- BC-1.05.035 INTERIM qualifier in §Postcondition 4: present PASS
- Fix-burst-internal nomenclature scan (`.factory/specs/**` per TD-VSDD-066): PASS (no H-N/M-PN/F-PN/L-PN patterns in non-changelog body sections of amendment surface)
- Summary table rows 1.1..1.23: present and monotonically ordered PASS
- Numeric cross-anchor (AC-3 `643686 bytes`): matches perf-baseline-w16.md PASS

---

## ADR-013 Clock Status

**Clock:** RESET to 0_of_3 by this SUBSTANTIVE verdict.

**Remaining for CONVERGENCE_REACHED:** 3 consecutive NITPICK_ONLY passes (25, 26, 27) needed.

**Finding trajectory:** pass-22 SUBSTANTIVE (2H/3M/2L) → pass-23 NITPICK (0H/0M/2L) → pass-24 SUBSTANTIVE (1H/6M/3L) — RESET.

---

## Disposition

H-P24-001 is in amendment surface (BC-1.05.036). All 6 MEDs and 3 LOWs are in lessons-corpus artifacts (lessons.md + open-backlog-post-rc8.md) — out of E-9 amendment scope but are canonical references future bursts MUST cite. Combined D-267 seal-and-fix burst applies all corrections atomically per TD-VSDD-053 single-commit protocol.
