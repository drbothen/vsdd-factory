---
document_type: adversarial-review
review_type: spec-cascade
artifact_under_review: "BC-5.39.009 v1.5 + S-15.17 v1.6"
cycle: brownfield-backfill
pass: 6
producer: adversary
timestamp: 2026-05-28
verdict: HIGH
finding_count: 11
finding_count_by_severity:
  critical: 0
  high: 5
  medium: 4
  low: 1
  nitpick: 1
  process_gap: 1
streak_status: "STREAK 0/3 → 0/3 after pass-6 (HIGH; reset)"
---

# Adversarial Review — S-15.17 Spec Cascade Pass-6

## Part A — Finding Set

### F-S15.17-SP6-001 — HIGH — Missing grep evidence blocks referenced from PC bodies and changelog (Grep-A, NEW-A, NEW-B do not exist)

**Severity:** HIGH
**Confidence:** HIGH
**File evidence:**
- BC body line 235: `segment (count=4) from cascade trajectory prose. See §SDK Grounding Evidence Grep-A.`
- BC body line 448: `Multi-digit values (\`→10→12→11→13\`) are valid (count=4). See §SDK Grounding Evidence Grep-A for literal-shell confirmation...`
- BC last_amended (line 46) claims: `§SDK Grounding Evidence Grep NEW-A added with production STATE.md trajectory-tail marker occurrences` and `§SDK Grounding Evidence Grep NEW-B added showing production current_cycle: whitespace boundary`
- BC §SDK Grounding Evidence (lines 684-775) contains ONLY: Grep 1, Grep 2, Grep 3, Grep 4, Grep 5, Grep 6, Grep 7, Grep 8, Grep 9. **No Grep-A. No Grep NEW-A. No Grep NEW-B.**

**Analysis:** The pass-5 fix-burst cure for F-SP5-001 CRITICAL (inv-4 marker-prefix redesign) explicitly promised in its changelog text to add three new grep evidence blocks (Grep-A, Grep NEW-A, Grep NEW-B) that would ground the marker-prefix discipline with literal-shell production STATE.md evidence. These blocks were **never added to the artifact**. Two PC bodies and the changelog reference grep evidence that does not exist.

**Why HIGH:** A spec section citing nonexistent evidence is functionally a hallucinated anchor. An implementer following PC1/PC4 will look for Grep-A in §SDK Grounding Evidence, fail to find it. Worse, POLICY 5 v1.3 SDK-grounding mandate explicitly requires marker-prefix claims be backed by literal-shell evidence — this BC v1.5 *violates POLICY 5* inside the very same cure that was supposed to apply marker-prefix discipline rigorously.

**Cure recommendation:** Add Grep-A (or replace inline-citations with Grep 4; or add Grep 10 with `grep "trajectory-tail" .factory/STATE.md | head -5`). Either fix body citations to point at existing Grep N numbering OR materialize the promised Grep-A/NEW-A/NEW-B blocks.

---

### F-S15.17-SP6-002 — HIGH — [regression of META-LEVEL-33] [pattern] — Extractor function names diverge between BC body and BC §Architecture Anchors AND between story T-5 and story Architecture Mapping table

**Severity:** HIGH (pattern flag)
**Confidence:** HIGH
**File evidence:**
- BC body line 290 (PC4): uses `extract_concurrent_cycles_latest_row()` — the new F-SP5-002 name
- BC body line 614 (story T-5): uses `extract_last_updated_cell` — the new name
- BC §Architecture Anchors line 618: `(extract_frontmatter_current_step, extract_last_updated_section, extract_phase_progress_latest_row, extract_concurrent_cycles_section, extract_session_resume_section_1)` — **OLD names** (`_section` suffix retained)
- Story Architecture Mapping table lines 862-864: `extract_last_updated_section`, `extract_phase_progress_section`, `extract_concurrent_cycles_section` — **OLD names**

**This is META-LEVEL-33 recurrence inside the META-33 cure-burst** — exactly the failure class POLICY 5 v1.3.3 was supposed to prevent.

**Cure recommendation:** Update BC §Architecture Anchors line 618 and story Architecture Mapping table to use new names: `extract_last_updated_cell`, `extract_concurrent_cycles_latest_row`, add `extract_burst_log_latest_dim7`.

---

### F-S15.17-SP6-003 — HIGH — Stale "BC v1.4" / "BC v1.1" / "BC v1.2" references in story v1.6 non-historical body (META-33 sibling-sweep claim contradicted)

**Severity:** HIGH (pattern flag: 18+ sites affected)
**Confidence:** HIGH
**File evidence (story v1.6 non-historical occurrences):**
- Line 109, 146, 152, 213, 258, 265, 280: "BC v1.4" in parity audit and §Why This Exists narrative
- Lines 434, 453, 473, 474, 478, 502, 515: "BC v1.4" inside T-5 NOTE/code pseudocode
- Line 907: "BC v1.2 inv-9" (Architecture Compliance Rules)
- Line 918: "BC v1.1" (Architecture Compliance Rules)
- Line 1038: "BC v1.4 Precondition 4 F-SP3-008" (EC-019 row)
- Lines 1066, 1067, 1069, 1070, 1071, 1072, 1073: "BC v1.1" / "BC v1.4" in BC gate adjudications
- Lines 1102, 1103, 1104, 1105, 1106: "BC v1.1" / "BC v1.4" in adversary plan

**Story v1.6 changelog claim:** "all non-historical BC v1.4→v1.5 version references swept" — factually false (18+ stale sites remain).

**Cure recommendation:** Literal-shell sweep: `grep -n "BC v1\.[1234]" story.md` then update all matches to v1.5 (preserving Changelog historical rows). Add pre-commit gate verifying zero non-historical stale references.

---

### F-S15.17-SP6-004 — HIGH — BC v1.5 PC2 NOTE describes stale STATE.md production state ("as of D-517"); D-518 production STATE.md already has the marker

**Severity:** HIGH
**Confidence:** HIGH
**File evidence:**
- BC line 250-253: "the marker IS present in `current_step:` but NOT in the Last Updated cell as of D-517"
- Production STATE.md line 62 (post-D-518): `| **Last Updated** | 2026-05-28 — D-518 ... trajectory-tail →9→9→9→11. |` — **marker IS NOW present**

**Cure recommendation:** Update PC2 NOTE to reflect D-518 production state: "Production STATE.md Last Updated cell contains `trajectory-tail →9→9→9→11` after the prose trajectory segment as of D-518."

---

### F-S15.17-SP6-005 — HIGH — BC v1.5 §Adversary Pass Coverage section missing Pass-5 entry (truncated at Pass-4)

**Severity:** HIGH
**Confidence:** HIGH
**File evidence:**
- BC lines 99-107: §Adversary Pass Coverage contains entries for Pass-1, Pass-2, Pass-3, Pass-4. No Pass-5.

**Cure recommendation:** Add line for Pass-5 with: 12 findings 1C+4H+5M+1L+1N IMPROVING; HUMAN-DIRECTED PARTIAL REVERSAL §Cure-Extension Parsimony Note point 2; META-LEVEL-33 CANDIDATE codified.

---

### F-S15.17-SP6-006 — MEDIUM — BC v1.5 §SDK Grounding Evidence Grep 1 violates POLICY 5 v1.3.1 stable-anchor sub-clause (sed -n line numbers + narrative line-number anchor)

**Severity:** MEDIUM
**Confidence:** HIGH
**File evidence:**
- BC line 688: `$ sed -n '82,94p' crates/hook-sdk/src/host.rs`
- BC line 703: `Confirmed: enum closes at line 94. ... Closing brace on line 94`

**Cure recommendation:** Replace narrative line-number anchors with stable-anchor narrative: "Confirmed: enum body shown; closing brace immediately follows the `Other(i32)` variant; no `TooBig` variant exists."

---

### F-S15.17-SP6-007 — MEDIUM — BC v1.5 §SDK Grounding Evidence header label drift ("this BC v1.4" inside v1.5 artifact)

**Severity:** MEDIUM
**Confidence:** HIGH
**File evidence:**
- BC line 682: `cited in this BC v1.4 is backed by literal-shell grep`
- BC frontmatter version: "1.5"

**Cure recommendation:** Change line 682 "this BC v1.4" → "this BC" or "this BC v1.5".

---

### F-S15.17-SP6-008 — MEDIUM — Story v1.6 Behavioral Contracts Table cell vs Parity Verdict invariant-listing contradiction

**Severity:** MEDIUM
**Confidence:** HIGH
**File evidence:**
- Story line 248 (BC Table cell): `invariants 1, 3, 4, 5, 6, 7, 8, 9, 10 (invariants 2/11/12 enforced via Architecture Compliance Rules code review)` — omits inv-13
- Story line 272 (Parity Verdict): `Invariants 1,3,4,5,6,7,8,9,10,12,13 cited in body; 2,11 covered by code-review gate` — includes 12 in body, conflicts with line 248

**Cure recommendation:** Reconcile to: BC Table cell → "invariants 1,3,4,5,6,7,8,9,10,13 (invariants 2/11/12 enforced via Architecture Compliance Rules code review)"; Parity Verdict → "Invariants 1,3,4,5,6,7,8,9,10,13 cited in body; 2,11,12 covered by code-review gate".

---

### F-S15.17-SP6-009 — MEDIUM — Story v1.6 EC-020 description retains stale "EC-020 added in BC v1.4" attribution + lib.rs:1143 line-number anchor

**Severity:** MEDIUM
**Confidence:** HIGH
**File evidence:**
- Story line 1039: `[Mirrored into BC v1.4 by PO at F-SP4-009 closure.]`
- Story line 1046: `validate-policies-schema sibling uses String::from_utf8 decode with fail-open on decode failure at lib.rs:1143`

**Cure recommendation:** Line 1039 → "[Carried forward in BC v1.5; originally mirrored at BC v1.4 F-SP4-009.]"; line 1046 → replace "lib.rs:1143" with stable anchor like `grep "Ok(bytes)" validate-policies-schema/src/lib.rs`.

---

### F-S15.17-SP6-010 — LOW — BC v1.5 PC1 prose claim ("two `trajectory-tail` marker mentions") is imprecise; actual production has 16 occurrences across STATE.md

**Severity:** LOW
**Confidence:** MEDIUM
**Cure recommendation:** Replace "two `trajectory-tail` marker mentions" with verifiable phrasing: "multiple `trajectory-tail` marker mentions; the first-occurrence semicolon-segment scoping yields count=4 → PASS per inv-4."

---

### F-S15.17-SP6-011 — NITPICK — Token Budget total arithmetic inconsistency

**Severity:** NITPICK
**File evidence:** Sum = ~96,500; stated Total ~95,000. Off by ~1,500.

**Cure recommendation:** Update Total to ~96,500 OR re-verify individual rows.

---

### F-S15.17-SP6-PG-001 — process_gap MEDIUM — POLICY 5 v1.3.3 sibling-sweep mandate lacks literal-shell verification gate; sweep claims accepted at face value

**Severity:** MEDIUM (process_gap)
**Confidence:** HIGH
**File evidence:**
- policies.yaml line 111: POLICY 5 v1.3.3 sibling-sweep mandate prescribes enumeration but contains NO requirement to verify the sweep with literal-shell `grep` after-the-fact
- F-SP6-002 + F-SP6-003 + F-SP6-007 demonstrate sweep-claims accepted without execution

**This is META-LEVEL-34 CANDIDATE — sweep-claim-without-execution.** Three consecutive passes have produced sweep claims that did not execute. The POLICY itself becomes the false-green (because reviewers accept the claim).

**Cure recommendation:** Extend POLICY 5 v1.3.3 with verification step requiring literal-shell `grep -n "BC v[0-9.]+" <file>` (or equivalent per category) with captured stdout in fix-burst evidence, before the changelog claim of sibling-sweep completion. Alternatively codify as POLICY 19 (literal-shell-sibling-sweep-verification-gate).

---

## Part B — Convergence Assessment

### Verdict + STREAK + Trajectory

**Verdict:** HIGH
**STREAK:** 0/3 → 0/3 after pass-6 (HIGH; reset)
**Trajectory:** 14 → 11 → 14 → 16 → 12 → 11

### Cure Verification Table

| Cure (from pass-5 v1.5/v1.6) | Held? | Evidence |
|------------------------------|-------|----------|
| F-SP5-001 marker-prefix two-step (inv-4 redesign) | PARTIAL | Logic correct in PC bodies; but Grep-A evidence MISSING (F-SP6-001) |
| F-SP5-002 PC4 extract_concurrent_cycles_latest_row | PARTIAL | PC4 body uses new name; §Architecture Anchors retains old name (F-SP6-002) |
| F-SP5-003 PC10 OUT-OF-SCOPE | HELD | Confirmed |
| F-SP5-005 extract_current_cycle multi-line | HELD | §Architecture Anchors spec specifies all YAML forms |
| F-SP5-006 inv-13 encoding gate | PARTIAL | inv-13 added; but BC Table cell line 248 omits inv-13 (F-SP6-008) |
| F-SP5-007 PC9 extract_burst_log_latest_dim7 | PARTIAL | PC9 body specifies; §Architecture Anchors does NOT list (F-SP6-002) |
| F-SP5-008 PC2 marker requirement | NOT-HELD | PC2 NOTE describes stale D-517 state (F-SP6-004) |
| F-SP5-004 T-5 NOTES grep -n strip | HELD | T-5 narrative comments stripped |
| F-SP5-009 BC Table v1.3→v1.5 | PARTIAL | Cell updated; but 18+ other BC v1.4 sites remain (F-SP6-003) |
| F-SP5-010 Token Budget BC row | HELD | Updated to ~24,000 |
| F-SP5-011 BC Table cell PC11/PC12 cite | HELD | Appended to cell |
| F-SP5-012 NITPICK covered by F-SP5-004 | PARTIAL | sibling-sweep self-application broadly failed |
| META-LEVEL-33 codification (POLICY 5 v1.3.3) | NOT-HELD-AT-PROCESS-LEVEL | Mandate exists but no verification gate (F-PG-001) |

**Score: 3 HELD, 6 PARTIAL, 2 NOT-HELD, 1 PROCESS-NOT-HELD. Net cure success rate: 25% full + 50% partial.**

### Regression Sweep

3 regressions tagged (all META-LEVEL-33 recurrence inside META-33 cure):
- F-SP6-001 [regression of META-LEVEL-33] — missing grep evidence inside cure
- F-SP6-002 [regression of META-LEVEL-33] [pattern] — extractor name divergence
- F-SP6-003 [regression of META-LEVEL-33] [pattern] — stale version references

### META-LEVEL Signals

| META-LEVEL | Recurrence Check | Status |
|------------|------------------|--------|
| META-LEVEL-33 (sibling-sweep-inside-policy-cure) | F-SP6-002/003/007/008/009 | RECURRENCE CONFIRMED (4 passes in a row exhibit this class) |
| META-LEVEL-32 (stable-anchor-recurrence) | F-SP6-006 (Grep 1 narrative line-number anchor inside POLICY 5 v1.3.1 cure) | RECURRENCE CONFIRMED |
| META-LEVEL-30 route (b) (codified-without-runtime-gate) | F-SP6-001 (changelog promises Grep blocks; not materialized) | NEW FORM — codified-without-content-materialization at spec-narrative layer |
| META-LEVEL-24 (false-green) | F-SP6-005 (§Adversary Pass Coverage truncated) | Self-coverage gap |
| **META-LEVEL-34 CANDIDATE** (sweep-claim-without-execution) | F-PG-001 codifies the class | **NEW CANDIDATE** — POLICY 5 v1.3.3 sweep claims not literal-shell-verified |

### Convergence Plausibility (EXPLICIT)

**Question 1: Did pass-6 trajectory drop materially below 12?** NO. Pass-6 = 11 findings, a 1-finding drop. Not material.

**Question 2: Did it stay at floor [8-12]?** YES, at the high end of the floor band.

**Question 3: Did it regress above 12?** NO. 11 < 12, but marginally.

**Conclusion: ASYMPTOTIC-FLOOR CONFIRMED at [11-16] band.** Critically:
- 0 CRITICAL is sustained (good — CRITICAL F-SP5-001 marker-prefix redesign held structurally)
- 5 HIGH is the new floor for HIGH severity
- META-LEVEL-33 cure has DEMONSTRATED INADEQUACY — 3 of 5 HIGH findings are META-33 recurrence INSIDE the META-33 cure-burst's own outputs

**SEAL ADJUDICATION RECOMMENDED.** Without a process-level fix to POLICY 5 v1.3.3 adding literal-shell sweep verification, every subsequent fix-burst will produce another META-33 recurrence and pass-7 will likely produce 8-12 HIGH findings of the same class.

**Alternative recommendation:** Before pass-7, orchestrator should: (a) require human-directed mechanical sweep of `grep -n "BC v1\.[1234]" .factory/stories/S-15.17*.md` and update all matches to v1.5 (preserving Changelog historical rows); (b) materialize the missing Grep-A/NEW-A/NEW-B blocks OR remove their citations; (c) update §Architecture Anchors function names; (d) re-author §Adversary Pass Coverage with pass-5 line; (e) update PC2 NOTE to reflect D-518 production state. THEN dispatch pass-7 fresh-context. If pass-7 produces ≥ 5 NEW findings, SEAL.

### Top 3 Findings (one-line)

1. F-S15.17-SP6-001 HIGH — Grep-A / NEW-A / NEW-B referenced from PC bodies but missing from §SDK Grounding Evidence; cure for F-SP5-001 CRITICAL has zero literal-shell grounding.
2. F-S15.17-SP6-002 HIGH [regression] [pattern] — extractor function names diverge between BC body / BC §Architecture Anchors / story Architecture Mapping; META-33 sibling-sweep self-claim factually false.
3. F-S15.17-SP6-003 HIGH [regression] [pattern] — 18+ stale "BC v1.4" / "BC v1.1" references in story v1.6 non-historical body despite v1.6 changelog claim of full v1.4→v1.5 sweep.

### Iron Law Compliance Attestation

I attest that during this review I did NOT read any of:
- `.factory/code-delivery/S-15.17/adv-spec-pass-1.md`
- `.factory/code-delivery/S-15.17/adv-spec-pass-2.md`
- `.factory/code-delivery/S-15.17/adv-spec-pass-3.md`
- `.factory/code-delivery/S-15.17/adv-spec-pass-4.md`
- `.factory/code-delivery/S-15.17/adv-spec-pass-5.md`

Iron Law (BC-5.39.001) FRESH-CONTEXT compliance: **VERIFIED.** Finding IDs referenced (F-SP1..F-SP5) are reconstructed from artifact-under-review's own changelog narrative, not from prior-pass reports.
