---
document_type: adversary-review
cycle: v1.0-feature-plugin-async-semantics-pass-1
pass: 21
verdict: HIGH
adr_013_clock_after_pass: 0_of_3
producer: adversary
timestamp: 2026-05-08T23:30:00Z
---

# F5 Pass-21 Adversary Review

## Verdict

**HIGH** — three substantive findings; the same recurrence-of-codified-lesson pattern that has now blocked the ADR-013 chain four passes in a row (P18-001, P19-001, P20-001/002, P21-001). ADR-013 clock RESETS to 0_of_3.

The fix-burst-19 closure of F-P20-001 (extended prose-form sweep) demonstrably did not fully apply L-P19-001 — a sibling instance exists unfixed in BC-5.34.004, while the lesson L-P20-001 itself was being codified.

F-P20-003 closure (L-P19-002 disposition amendment) cites a symbol that does not exist in production code (POLICY 4 violation).

## Trajectory

17→15→6→5→0→2→5→1→4→2→2→4→4→5→7→5→4→HIGH(P18)→HIGH(P19)→HIGH(P20)→**HIGH(P21)**

Four-pass HIGH streak. Strategic assessment: **halt-and-implement S-15.03 (mechanical enforcement) before continuing F5 chain.**

## Findings

### F-P21-001 [HIGH] BC-5.34.004 active body cite to multi-repo.lobster `at lines 575-731` is unfixed sibling of F-P20-001 BC-5.34.003 fix

**Confidence:** HIGH

**Evidence:**
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-05/BC-5.34.003.md:43` — FIXED in fix-burst-19 sub-burst 1 (HTML carve-out comment added).
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-05/BC-5.34.004.md:31` — UNFIXED; identical pattern (`Three additional sub-mode trees defined (feature_mode, bugfix_mode, maintenance_mode) at lines 575-731`).
- BC-5.34.003 and BC-5.34.004 are immediate filename siblings in SS-05, referencing the EXACT same lobster file and line range.
- Fix-burst-19 burst-log table omits BC-5.34.004.

**Impact:** Sibling-pattern recurrence; same root cause as F-P19-001 / F-P20-001. Per S-7.01: blast radius >=2 = HIGH.

**Recommended fix:** Apply the same HTML carve-out comment to BC-5.34.004:31. Or migrate both BCs to a stable lobster section anchor.

---

### F-P21-002 [HIGH] lessons.md L-P19-002 disposition cites fabricated symbol `lib.rs::passes_clean_to_close`

**Confidence:** HIGH

**Evidence:**
- lessons.md:164 reads: `VP-071 (... kani::assume(passes < 3) matches production threshold at lib.rs::passes_clean_to_close ...)`.
- `grep -rn "passes_clean_to_close"` returns NO matches anywhere in the codebase.
- Actual production threshold check is `hook_result_for` at `crates/hook-plugins/validate-per-story-adversary-convergence/src/lib.rs:130-131`: `if s.passes_clean < 3 { ... }`.
- Kani harness `proof_insufficient_passes_always_blocks` at lib.rs:630 calls `hook_result_for(state)`.

**Impact:** POLICY 4 violation. Audit claim "no staleness detected" cannot be verified because the named production symbol does not exist. Future auditor would `grep`, find nothing, and either abandon the audit or propagate the fabrication.

**Recommended fix:** Replace `lib.rs::passes_clean_to_close` with `lib.rs::hook_result_for` and re-state the audit narrative.

---

### F-P21-003 [HIGH] S-15.03 epic anchor and subsystems anchor are semantically incorrect

**Confidence:** HIGH

**Evidence:**
- S-15.03-index-cite-refresh-hook.md:5 declares `epic: E-15` and line 6 declares `subsystems: [SS-04]`.
- E-15-plugin-async-semantics.md:6 epic title is `Plugin Async Semantics — Registry-Layer Partition`. line 8 declares `subsystems_affected: [SS-01, SS-07, SS-09]`.
- S-15.03 scope (per body): WASM hook plugins for ARCH-INDEX cite-refresh + lessons retroactive-sweep enforcement. ZERO relationship to plugin async semantics, registry partition, drain windows.
- `subsystems: [SS-04]` is not in parent epic's `subsystems_affected: [SS-01, SS-07, SS-09]`.

**Impact:** POLICY 4 (semantic_anchoring_integrity), POLICY 5 (creators_justify_anchors). Future implementer reading STORY-INDEX E-15 column and following to E-15 epic would be confused.

**Recommended fix:** Move S-15.03 to a new epic (e.g., E-16: Index Cite-Refresh / Lessons Enforcement) OR re-anchor under engine-discipline cycle's E-12. Update `subsystems:` to match.

---

## Process-gap findings

### O-P21-001 [process-gap] Same-burst lesson-and-application pattern is failing at sibling-class layer for the 4th consecutive pass

- fix-burst-17 codified L-P18-002, applied literally → pass-19 found sibling F-P19-001.
- fix-burst-18 codified L-P19-001, applied retroactively → pass-20 found sibling F-P20-001 (singular caught; plural missed).
- fix-burst-19 codified L-P20-001 + L-P20-002, applied within burst → pass-21 found sibling F-P21-001 (BC-5.34.004 missed in lobster carve-out).

The recurrence pattern motivates halting the F5 chain and prioritizing S-15.03 implementation. Per scope item 7: this is the strategic decision point.

### O-P21-002 [process-gap] fix-burst-19 burst-log sub-burst 2 table omits ARCH-INDEX v1.23 → v1.24 row

Documentation completeness gap. ARCH-INDEX bumped twice in fix-burst-19 but burst-log only records once.

## Notable observations

1. F-P20-001 partial closure: 4 BCs migrated correctly; gap is BCs NOT in the table (F-P21-001).
2. F-P20-002 closure verified clean: ARCH-INDEX cite is now `BC-INDEX v1.44` matching frontmatter.
3. F-P20-005 closure clean: BC-7.06.001 reword unambiguous.
4. L-P20-001 codification text well-stated.
5. L-P20-002 codification text well-stated.
6. VP-077 / VP-071 Kani assumptions correct as audited — bug is in the audit's NAMING, not assumptions.
7. S-15.03 stub body otherwise viable; only defects are epic/subsystem anchors (F-P21-003).
8. STATE.md well-maintained (189 lines).
9. Index versions all match frontmatter: BC-INDEX v1.44, VP-INDEX v1.29, STORY-INDEX v2.50, ARCH-INDEX v1.24.
10. **Four-pass HIGH streak**: trajectory NOT converging via prose-only codification.

## Convergence assessment

**Novelty: HIGH.** All three findings are genuinely new — F-P21-001 sibling not findable from prior pass (fix not committed at P20 review time); F-P21-002 requires grep against actual production code; F-P21-003 emerges only after S-15.03 was filed in fix-burst-19.

**ADR-013 clock:** **0_of_3** (RESET).

**Recommendation:** Halt F5 chain. Implement S-15.03 (after re-anchoring per F-P21-003) before continuing prose-only fix-bursts. Strategic decision criterion in scope item 7 has been met.

If user continues prose-only:
- Fix-burst-20 must close F-P21-001/002/003.
- Must include retroactive sweep over ALL ss-05 BCs that cite multi-repo.lobster line ranges.
- After fix-burst-20, dispatch pass-22 expecting NITPICK_ONLY but with high probability of recurrence.
