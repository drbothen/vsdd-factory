---
document_type: adversarial-review
level: F5
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-05-11T00:00:00Z
phase: F5
inputs: []
input-hash: "[live-state]"
traces_to: prd.md
cycle: v1.0-feature-engine-discipline-pass-1
pass: 28
Z-suffix: true
date: 2026-05-11
previous_review: adv-cycle-pass-27.md
prior-pass-classification: HIGH
prior-findings-count: 12
verdict: HIGH
findings_count:
  critical: 0
  high: 3
  medium: 2
  low: 4
  nitpick: 1
process_gap_count: 1
convergence_reached: false
---

# Adversarial Review — Cycle v1.0-feature-engine-discipline-pass-1 — Pass 28

**Date:** 2026-05-11
**Verdict:** HIGH (3H + 2M + 4L + 1NIT + 1PG)
**Prior pass:** Pass 27 — HIGH (2H + 5M + 3L + 2NIT + 1PG)
**Convergence streak:** 0/3

---

## Finding ID Convention

Findings in this cycle use `F-P<PASS>-<SEQ>` format (cycle-specific convention established
at pass-1; not ADV-format). Process gaps use `F-P<PASS>-PG<N>` suffix.

---

## Part A — Pass-27 Fix Burst Verification

Verification of pass-27 fix burst claims against current artifact state per D-402+D-407(b)
mandatory independent re-execution.

### D-407 codification

- `grep -c 'D-407' decision-log.md` → 1 (D-407 row with 4 sub-clauses present) ✓

### 4-index acknowledgment D-389..D-407

- BC-INDEX `grep -cE 'version: "1\.69"'` → 1 ✓
- VP-INDEX `grep -cE 'version: "1\.45"'` → 1 ✓
- STORY-INDEX `grep -cE 'version: "2\.70"'` → 1 ✓
- ARCH-INDEX `grep -cE 'version: "1\.50"'` → 1 ✓
- All 4 indexes: `grep -c 'D-389..D-407'` → 1 each ✓

### L-EDP1-019 appended; L-EDP1-018 Layer-17 inline-replaced

- `grep -c 'L-EDP1-019' lessons.md` → 1 ✓
- `grep -c 'F-P27-001 D-406 not in 4 indexes' lessons.md` → 1 ✓

### F-P27-002 corrigendum self-validation discrepancy

- `grep -cE 'Corrigendum \(pass-27 fix burst — D-387 / F-P27-002' burst-log.md` → 1 ✓ (corrigendum present)
- Self-validation at Dim-5 (burst-log line 982): claims `→ 6 ✓`
- Self-validation in F-P27-002 corrigendum body (burst-log line 849): claims `→ 4 ✓`
- **DISCREPANCY**: Dim-5 says 6 (correct), corrigendum body says 4 (wrong). → **F-P28-001**

### STATE.md pass-count check

- `grep -c '27 F5 cycle-level reviews' STATE.md` → actual **2** (line 143 Concurrent Cycles + line 192 Session Resume Checkpoint)
- Pass-27 Dim-7 claimed `→ 1 ✓` → **F-P28-002**

### INDEX.md pass-27 row

- `grep -c '| 27 |' INDEX.md` → 1 ✓

---

## Part B — New Findings

### HIGH

**F-P28-001 [HIGH] — F-P27-002 corrigendum self-validation `→ 4 ✓` actual=6**

**Location:** burst-log.md, F-P27-002 corrigendum body (line ~849) vs Dim-5 (line ~982)

**Description:** Pass-27 Dim-5 correctly records the self-validation count as `→ 6` (4 original pass-25 corrigenda + F-P26-002 corrigendum + F-P27-002 corrigendum itself). However, the F-P27-002 corrigendum body at burst-log line 849 states `self-validation per D-407(b): ... burst-log.md → 4 ✓ (verified by independent execution)`. The corrigendum body was authored before the full count was realized; the Dim-5 block later recognized the 2 self-references but did not retroactively correct the corrigendum body. The corrigendum body false-green claim (`→ 4`) persists alongside the correct Dim-5 claim (`→ 6`). This is precisely the class of error D-407(b) was introduced to prevent.

**Root cause:** Corrigendum body authored before full verification; back-correction omitted from the corrigendum body after Dim-5 produced the correct count.

**Required fix:** Append D-387 corrigendum to the F-P27-002 corrigendum body correcting count from 4 to 6, citing D-408(c) per D-408 codification.

---

**F-P28-002 [HIGH] — Pass-27 Dim-7 `grep -c '27 F5 cycle-level reviews' STATE.md → 1 ✓` actual=2**

**Location:** burst-log.md, pass-27 Dim-7 Verification (line ~997)

**Description:** Pass-27 Dim-7 Verification: `grep -c '27 F5 cycle-level reviews' STATE.md → 1 ✓`. Independent re-execution yields 2: STATE.md line 143 (Concurrent Cycles row Notes cell) and STATE.md line 192 (Session Resume Checkpoint). Per D-402 exact-count: claimed `→ 1`, actual `→ 2`. False-green.

**Required fix:** Append D-387 corrigendum to pass-27 burst-log Dim-7 correcting count to 2 with site identification.

---

**F-P28-003 [HIGH] — Pass-27 Dim-2 and Dim-3 false-greens (claimed 1, actual 2 each)**

**Location:** burst-log.md, pass-27 Dim-2 (~line 956) and Dim-3 (~line 963)

**Description:**

Dim-2: `grep -c 'F-P27-001 D-406 not in 4 indexes' lessons.md → 1 ✓`. Actual: 2 — the string appears in the L-EDP1-018 Layer-17 inline-replace row content AND in the L-EDP1-019 layer-history table row 17 "Same-burst Violation" column.

Dim-3: `grep -c 'L-EDP1-019' lessons.md → 1 ✓`. Actual: 2 — the section header line (`### L-EDP1-019 — 18th-layer...`) and the L-EDP1-018 layer-history table Layer-18 row, which contains `L-EDP1-019` as a forward reference.

Both: D-408(b) (to be codified) requires that when a Verification grep target appears in both source content AND layer-history table cells, the count must bound the search to the original site OR cite the multi-match count explicitly (e.g., `→ 2 (1 source + 1 table cell) ✓`).

**Required fix:** Append D-387 corrigendum to pass-27 Dims 2 and 3 correcting counts to 2 with site identification per D-408(b).

---

### MEDIUM

**F-P28-004 [MEDIUM] — Pass-27 Dim-7 Extent "4 edits" lists 5 fields, actual 6+ sites**

**Location:** burst-log.md, pass-27 Dim-7 (~line 994)

**Description:** Dim-7 states Extent "4 edits" but the inlined list enumerates 5 fields: `phase:, current_step:, Last Updated, Current Phase, Concurrent Cycles`. Actual STATE.md edit sites in pass-27 fix burst: at minimum 6 (frontmatter line 8 `phase:`, frontmatter line 14 `current_step:`, Last Updated row ~41, Current Phase row ~42, Phase Progress table rows 102-103, Concurrent Cycles row 143, Session Resume Checkpoint ~192). The "4 edits" extent claim is understated on both the inlined-list count (5 not 4) and the actual site count (6+ not 4-5). Per D-391 enumeration-source integrity.

**Required fix:** Append D-387 corrigendum correcting Extent to match actual site count.

---

**F-P28-005 [MEDIUM] — L-EDP1-019 narrative omits in-burst corrigendum-body false-green sub-pattern**

**Location:** lessons.md, L-EDP1-019 body

**Description:** L-EDP1-019 documents the 18th-layer recurrence as two sub-recurrences: (1) D-404 unconditional conflation and (2) corrigendum-prescribed regex invalid. F-P28-001 reveals a third sub-pattern not captured: a corrigendum body can introduce a false-green count even when the Dim-N Verification block is correct (Dim-5 says 6, corrigendum body says 4). This "in-burst false-green at corrigendum body level, masked by a correct Dim-N Verification block" is a distinct L-EDP1-003 sub-variant not documented in L-EDP1-019. Layer-18 inline-replace should capture this.

**Required fix:** Incorporate in L-EDP1-019 Layer-18 inline-replace (Commit B of this burst).

---

### LOW

**F-P28-006 [LOW] — Range-form vs explicit literal acknowledgment ambiguity**

**Location:** burst-log.md, pass-27 Dim-4 (~line 974)

**Description:** Dim-4 uses range-form `D-389..D-407` to attest D-404 literal-acknowledgment compliance. Range-form `D-389..D-407` implies D-407 is acknowledged, but does not explicitly state "D-407 by literal ID." D-404 requires literal-by-ID; range-form satisfies D-404 only implicitly (D-407 is within the range). Future bursts should note this distinction to prevent claims that range-form does not satisfy D-404.

---

**F-P28-007 [LOW] — SHA placeholder persists in Active Branches**

**Location:** STATE.md line 133

**Description:** Active Branches row for `factory-artifacts` reads "F5 pass-27 fix burst Commit E — state-manager final" — stale narrative from pass-27. This burst's Commit E should roll this forward.

---

**F-P28-008 [LOW] — STORY-INDEX `last_amended` as inline accumulation vs changelog list**

**Location:** stories/STORY-INDEX.md frontmatter

**Description:** STORY-INDEX uses `last_amended` as a deeply-nested inline string accumulating all amendment history (1,000+ characters, 5+ nested bracket sequences), while BC-INDEX, VP-INDEX, and ARCH-INDEX use a structured `changelog:` YAML list. Schema inconsistency across the 4 indexes makes programmatic parsing unreliable.

---

**F-P28-009 [LOW] — Dim-1 verifies pass-26 marker, not pass-27 marker**

**Location:** burst-log.md, pass-27 Dim-1 (~line 871)

**Description:** Pass-27 Dim-1 verifies `grep -c 'pass-26 fix burst COMPLETE' STATE.md → 4 ✓` — this is the pass-26 verification carried forward, not pass-27. The pass-27 marker is verified in Dim-7 (`pass-27 fix burst COMPLETE → 3`). Dim-1 should verify pass-27 canonical markers. Low severity because Dim-7 correctly validates pass-27.

---

### NITPICK

**F-P28-010 [NITPICK] — Dim-5 self-validation count 6 comment could note self-referential inevitability**

**Location:** burst-log.md, pass-27 Dim-5 (~line 982)

**Description:** The corrected regex `Corrigendum \(pass-25 fix burst — D-387 / F-P25-(005|006|010|011)` (without trailing `\)`) will match any future corrigendum body that cites this pattern. The Dim-5 comment notes `→ 6 (4 original + F-P26-002 + this F-P27-002 corrigendum)` which is correct, but does not note that count will grow with each future corrigendum that cites the regex. Per D-408(c): this is acknowledged multi-match artifact. A note to this effect would prevent future re-finding.

---

## Part C — Process Gap

**F-P28-PG1 — D-407(b) scope too narrow: covers corrigendum-prescribed regexes but not all Dim Verification blocks**

**Description:** D-407(b) requires corrigenda that prescribe Verification regexes to self-validate. F-P28-001 demonstrates that false-greens can also arise in primary Dim Verification blocks that verify claims in corrigendum bodies (Dim-5 says 6, corrigendum body says 4 — both are in the burst-log, both are attestation claims, but D-407(b) only mandates self-validation for corrigendum-prescribed regexes, not for Dim Verification greps generally).

**Required codification (D-408):** Extend the inline-validation obligation to ALL `Verification: grep -c <pattern> <file> → N ✓` lines in burst-log attestations. Every such line MUST be independently re-executed before commit; the reported integer MUST match actual `grep -c` output. Additionally: when a Verification grep target string appears in both source content and layer-history table cells, the Verification MUST bound the search to the original site OR cite the multi-match count explicitly (e.g., `→ 2 (1 source instance + 1 layer-history table cell) ✓`). And: D-407(b) corrigendum self-validation must count corrigenda-about-corrigenda when the regex matches their bodies.

---

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 3 |
| MEDIUM | 2 |
| LOW | 4 |
| NITPICK | 1 |
| Process Gap | 1 |

**Overall Assessment:** HIGH — 3 HIGH findings (D-402 false-greens in the same burst that introduced D-407(b) to prevent them)
**Convergence:** FINDINGS_REMAIN — streak reset; requires 3 consecutive NITPICK_ONLY
**Readiness:** Requires fix burst before pass-29 dispatch

---

## Policy Rubric Assessment

| Policy | Status |
|--------|--------|
| D-382 sibling-file sweep | APPLIED (pass-27) |
| D-383 intra-file content audit | APPLIED (pass-27) |
| D-384 trajectory cardinality | APPLIED (pass-27) |
| D-385 immutable-row scope | APPLIED (pass-27) |
| D-387 corrigendum protocol | APPLIED (pass-27) |
| D-391 enumeration-source | GAP (Dim-7 extent miscount — F-P28-004) |
| D-393 second-source query | APPLIED (pass-27) |
| D-395 file-state grep-back | GAP (Dims 2, 3, 7 false-greens) |
| D-397 intent-match | APPLIED (pass-27) |
| D-399 canonical marker | APPLIED (pass-27) |
| D-400 Layer-N inline-replace | APPLIED (pass-27) |
| D-401 cross-index sync | APPLIED (pass-27) |
| D-402 exact-count | VIOLATED (F-P28-001/002/003 — 3 false-greens) |
| D-403 regex precision | APPLIED (pass-27) |
| D-404 literal acknowledgment | APPLIED (pass-27) |
| D-405 pattern-class recognition | APPLIED (pass-27) |
| D-406 attestation-accuracy | APPLIED (pass-27) |
| D-407 unconditional + self-validation | VIOLATED (F-P27-002 corrigendum body claims 4 vs actual 6) |

---

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 28 |
| **New findings** | 11 content + 1 PG |
| **Duplicate/variant findings** | 0 (all novel instances even if same D-402 class) |
| **Novelty score** | 12/(12+0) = 1.0 (all new) |
| **Median severity** | HIGH (dominated by F-P28-001/002/003 HIGH trio) |
| **Trajectory** | 29→15→11→9→8→7→5→6→6→6→4→3→3→10→13→9→9→10→11→10→10→11→11→10→12→10→12→11 |
| **Verdict** | FINDINGS_REMAIN |

---

## Scope Statement

This pass reviewed: burst-log.md (pass-27 entry, Dims 1-8, corrigendum blocks), STATE.md (pass-27 fix burst Commit E updates — all 4 cells), INDEX.md (pass-27 row), decision-log.md (D-407), lessons.md (L-EDP1-018 Layer-17 inline-replace result + L-EDP1-019 new entry), and the 4 index files (BC-INDEX v1.69, VP-INDEX v1.45, STORY-INDEX v2.70, ARCH-INDEX v1.50). Full F5 cycle artifact set within scope. No source code or external systems reviewed.
