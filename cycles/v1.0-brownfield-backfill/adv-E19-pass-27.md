# Adversarial Review — E-19 Pass 27 (post-D-780 delta; perimeter = S-19.02 v1.11 + S-19.07 v1.11)

**Perimeter:** E-19 epic + S-19.01..S-19.07 + STORY-INDEX E-19 section + ADR-030 v1.3 + BC-5.42.001 v1.3 + BC-2.02.011 v1.5 + BC-4.13.001 v1.9 + BC-1.17.001 v1.3
**Reviewer:** fresh-context adversary (Iron Law; rubric = policies.yaml v1.4.1)
**Date:** 2026-07-08
**Verdict:** NOT-CLEAN — BLOCKER 0 / HIGH 1 / MEDIUM 2 / LOW 1 (4 items: 1 HIGH finding + 2 MEDIUM findings + 1 LOW observation)
**Streak:** 0/3 RESET (streak was 0/3 entering pass-27 per D-780; severity regression: 2→4 items across passes 26-27)
**Model family:** Claude Opus 4.7
**Delta artifact versions verified:** S-19.02 v1.11 (was v1.10); S-19.07 v1.11 (was v1.10).

## Part A — D-780 Delta Verification + New Findings

### Amendment 1 — S-19.02 v1.10 → v1.11 (Phase-A-complete assertion → merge-conditional)

Phase-A-complete assertion in §Narrative converted to merge-conditional form ("Phase-A of BC-4.13.001 will be complete when this story merges") ✓. Whole-file tense predicate run confirmed no additional present-tense delivered-state assertions ✓. Input-hash ccd11cf unchanged (tense-only update; no behavioral content changed) ✓. POLICY 14 5-leg parity confirmed ✓.

**F-P27-002 MEDIUM — S-19.02 test-set citation does not match BC-4.13.001 v1.9 exclusive boundary form.**

S-19.02 v1.11 §Acceptance Criteria contains a boundary citation in the unit test assertions (AC-005 test-set, Unit test A / O-P12-03 affirmative statements) that describes the extracted slice boundary with phrasing that elides the `0..` prefix. BC-4.13.001 v1.9 Invariant 9 specifies the extraction boundary in terms of `delimiter_start_offset`, but does not explicitly state the exclusive form `0..delimiter_start_offset`. The story's AC-005 affirmative assertion "slice starts at byte 0 (bytes 0..delimiter_start_offset)" is present, but the corresponding AC-005 framing of the slice boundary in the test-set header / task description uses imprecise phrasing that conflates inclusive vs exclusive interpretations. When BC-4.13.001 is updated to v1.10 (F-P27-003) to make the exclusive boundary explicit, the story's cite must track the updated version.

This is a sibling-site propagation finding: the BC changes and the story's test-set citation must be co-consistent.

**Locus:** S-19.02 v1.11 §Acceptance Criteria AC-005 test-set boundary framing and BC-version cite.
**Routing:** story-writer.
**Fix:** S-19.02 v1.11→v1.12: test-set cite corrected to match BC-4.13.001 v1.10 exclusive boundary form (`0..delimiter_start_offset`); BC-4.13.001 v1.9→v1.10 cite sweep across all AC/Task/Token Budget cites. Whole-file predicate run. Input-hash ccd11cf→59d0856 (content changed). POLICY 14 5-leg parity applied.

### Amendment 2 — S-19.07 v1.10 → v1.11 ("already works" / "correct and operational" → design-tense)

Three "already works" / "correct and operational" rhetorical delivered-frame phrases converted to design-tense / merge-conditional per BC-4.13.001 v1.9 tense class ✓. Whole-file tense predicate run confirmed ✓. Input-hash 01bed1d unchanged (tense-only update; no behavioral content changed) ✓. POLICY 14 5-leg parity confirmed ✓.

**F-P27-001 HIGH — S-19.07 DISTINCT-block parity regression (pass-12 partial-fix class; TD-VSDD-060 sibling-sweep miss).**

S-19.07 v1.11 §Previous Story Intel and §Architecture Mapping reference the capability-schemas preamble block introduced by O-P12-04 in S-19.06 v1.8. S-19.07 describes this block as "capabilities.read_prefix schema documentation" in narrative form, without preserving the DISTINCT-block framing that O-P12-04 established: the S-19.06 §Architecture Mapping and Task 12 explicitly require a **DISTINCT** "Capability Schemas" preamble block, separate from S-19.04's tool-filter-anchoring block. S-19.07 v1.11's references to this block do not reflect the DISTINCT-block structure — they describe it as if it extends or continues S-19.04's block, rather than being a separate DISTINCT block.

This is a pass-12 partial-fix regression: O-P12-04 established the DISTINCT-block form in S-19.06 and the story body was updated, but the sibling sweep at that time did not propagate the DISTINCT-block framing to S-19.07's Previous Story Intel or the E-19 epic's Wave-2 sequencing note. The regression class is TD-VSDD-060 (sibling-site sweep miss at value-change point — O-P12-04 changed the description of the S-19.04 preamble from single-block to two-DISTINCT-blocks, and S-19.07+epic were not updated accordingly).

**Locus:** S-19.07 v1.11 §Previous Story Intel (S-19.06 Patterns Established row — preamble block description); §Architecture Mapping (capabilities section note). E-19 epic v1.16 Wave-2 sequencing note for S-19.04 parenthetical ("adds capabilities.read_prefix schema documentation to that same section").
**Routing:** story-writer (S-19.07; E-19 epic).
**Fix:** S-19.07 v1.11→v1.12: DISTINCT-block parity applied — §Previous Story Intel S-19.06 row and §Architecture Mapping updated to reflect DISTINCT "Capability Schemas" preamble block (separate from S-19.04's tool-filter-anchoring block). BC-4.13.001 v1.9→v1.10 cite sweep also applied. Input-hash 01bed1d→82287d6 (content changed). POLICY 14 5-leg parity applied. **E-19 epic v1.16→v1.17:** Wave-2 S-19.04 parenthetical corrected to DISTINCT-block form ("S-19.06 adds a DISTINCT 'Capability Schemas' preamble block, separate from S-19.04's tool-filter-anchoring block; ordering-only dependency so two preamble blocks land without merge conflict"). Input-hash 0ff893e→bf647fc (SM correction; inputs changed). **CLOSED F-P27-001. TD-VSDD-060 sibling-sweep re-lesson recorded.**

### Full E-19 Epic and Story Suite Review

**F-P27-003 MEDIUM — BC-4.13.001 v1.9 Invariant 9 does not state the exclusive boundary byte-exactly.**

BC-4.13.001 v1.9 Invariant 9 governs the `extract_frontmatter()` behavior: the extracted slice is used by verify-factory-lock to read only the frontmatter header without loading the full STATE.md body. Invariant 9 describes the slice boundary in terms of `delimiter_start_offset` but does not make explicit that the form is `0..delimiter_start_offset` (exclusive upper bound), meaning the byte at `delimiter_start_offset` (the start of the closing `---` line) is NOT included in the slice, while the byte at offset 0 (the first byte of the opening `---\n` line) IS included.

This ambiguity is material: an implementer reading v1.9 Invariant 9 could implement either inclusive or exclusive boundary semantics. O-P12-03 adjudication (D-779 decision log, story-writer v1.9 AC-005 unit test A) established that the slice starts at byte 0 and is byte-exact. However, the BC body text in v1.9 does not reflect this explicitly — it relies on the AC-005 story-level unit test wording rather than stating it normatively in the BC invariant.

**Locus:** BC-4.13.001 v1.9 Invariant 9 boundary description.
**Routing:** product-owner.
**Fix:** BC-4.13.001 v1.9→v1.10: Invariant 9 amended to state `0..delimiter_start_offset` (exclusive byte-exact boundary; opening `---\n` marker included at byte 0; closing `---` marker at `delimiter_start_offset` NOT included in slice). O-P12-03 adjudication upheld verbatim. H1 title UNCHANGED (POLICY 7). **CLOSED F-P27-003.**

No additional findings on S-19.01 v1.14, S-19.03 v1.14, S-19.04 v1.11, S-19.05 v1.13, S-19.06 v1.14, ADR-030 v1.3, BC-5.42.001 v1.3, BC-1.17.001 v1.3, or BC-5.42.001 v1.3.

## Observations

**O-P27-001 [LOW; housekeeping] — BC-INDEX catalog row for BC-2.02.011 shows status "draft" instead of "active".**

BC-2.02.011 file frontmatter (`ss-02/BC-2.02.011.md`) declares `status: ready` and `lifecycle_status: active`. The BC-INDEX catalog row for BC-2.02.011 (ss-02 section) shows the status cell as "draft". Per BC-INDEX column convention, the status cell should reflect the BC's authoritative `lifecycle_status` field (as exemplified by active BCs such as BC-4.13.001 and BC-4.14.001 which show "active" in the status column).

This is a catalog-drift observation: the BC file was transitioned to `lifecycle_status: active` (presumably when S-19.03 was authorized and BC-2.02.011 entered an active implementation story), but the catalog row was not updated.

**Routing:** state-manager (BC-INDEX catalog maintenance).
**Fix:** BC-INDEX catalog row BC-2.02.011 status cell "draft"→"active". H1 title UNCHANGED. Record in D-781 decision log. **CLOSED O-P27-001.**

## Part B — Dimensions

| Dimension | Status |
|-----------|--------|
| Dim-1: BC/VP coverage | PASS — BC-4.13.001 v1.9 exclusive boundary consistently applicable to F-P27-002/F-P27-003; F-P27-001 DISTINCT-block is tense-class sibling-sweep miss (no BC amendment required) |
| Dim-2: AC gate execution | Not re-run (frozen stories for pass-27 delta; no gate changes in D-780 delta) |
| Dim-3: POLICY 14 5-leg parity | PASS on both D-780 delta amendments (S-19.02 v1.11/S-19.07 v1.11) |
| Dim-4: POLICY 8 BC propagation | PASS — no BC amendments in D-780 delta |
| Dim-5: Input-hash consistency | PASS — S-19.02 ccd11cf, S-19.07 01bed1d match expected values at time of review |
| Dim-6: STORY-INDEX BC-coverage | PASS — STORY-INDEX v4.158 row versions match frontmatter at time of review |
| Dim-7: ADR/BC interface parity | PASS (ADR-030 v1.3 canonical TOML shape correct per D-777; BC-4.13.001 v1.9 exclusive boundary gap is BC-internal, not ADR interface) |

## Summary

| Severity | Count |
|----------|-------|
| BLOCKER | 0 |
| HIGH | 1 |
| MEDIUM | 2 |
| LOW | 0 |
| Observations | 1 |

**Severity regression: 2 items (pass-26) → 4 items (pass-27).** Regression class: DISTINCT-block sibling-sweep miss (F-P27-001 HIGH; TD-VSDD-060 class — O-P12-04 established DISTINCT-block form in S-19.06 body but sweep did not propagate to S-19.07 + epic; this class has appeared previously as F-P3-008/F-P13-002/F-P15-002 in sibling-sweep misses on other preamble elements). F-P27-002/F-P27-003 are consequence of BC-4.13.001 v1.9 exclusive-boundary ambiguity — once BC is clarified (F-P27-003 PO leg), the story cites must track (F-P27-002 SW legs). O-P27-001 is housekeeping catalog drift. D-781 fix burst applied (PO BC-4.13.001 v1.9→v1.10 + SW E-19 epic v1.16→v1.17 + SW S-19.02 v1.11→v1.12 + SW S-19.06 v1.14→v1.15 + SW S-19.07 v1.11→v1.12 + SM index bumps + SM STATE.md advance).

**Overall Assessment:** NOT-CLEAN — B0/H1/M2/L1. Streak 0/3. NEXT: pass-28.

## Novelty Assessment

| Field | Value |
|-------|-------|
| Pass | 27 |
| New findings | 3 |
| New observations | 1 |
| Duplicate/variant findings | 0 (F-P27-001 is TD-VSDD-060 sibling-sweep class with new locus; counted as new per locus-distinct detection) |
| Novelty score | MEDIUM — F-P27-001 HIGH is TD-VSDD-060 class regression (new locus: DISTINCT-block O-P12-04 partial-fix); F-P27-002/003 are linked BC-exclusive-boundary propagation findings (new BC clause + story cite) |
| Median severity | MEDIUM |
| Verdict | NOT-CLEAN — streak 0/3; pass-28 NEXT |

## Coverage Attestation

Artifacts read in full: BC-4.13.001 v1.9 (1-end); S-19.02 v1.11 (1-end); S-19.07 v1.11 (1-end); STORY-INDEX E-19 section (680-710).
Spot-checked (no changes): ADR-030 v1.3; BC-5.42.001 v1.3; BC-2.02.011 v1.5; BC-1.17.001 v1.3; S-19.01 v1.14; S-19.03 v1.14; S-19.04 v1.11; S-19.05 v1.13; S-19.06 v1.14; E-19 epic v1.16.
Not read (Iron Law): decision-log; burst-log; lessons.md; STATE.md; checkpoints; adv passes 1-26; fix-burst records.

## Fix Burst Closure (D-781)

**Leg 1 — Product-owner (BC-4.13.001):** BC-4.13.001 v1.9→v1.10. Invariant 9 amended to state `0..delimiter_start_offset` (exclusive byte-exact boundary; opening `---\n` marker included at byte 0; O-P12-03 adjudication upheld). H1 title UNCHANGED (POLICY 7). Input-hash 2cca156. POLICY 14 5-leg parity applied. **CLOSED F-P27-003.**

**Leg 2 — Story-writer (E-19 epic):** E-19 epic v1.16→v1.17. Wave-2 S-19.04 parenthetical corrected to DISTINCT-block form per O-P12-04. Input-hash SM-corrected 0ff893e→bf647fc (story-writer missed recompute; inputs changed: BC-4.13.001 v1.10 + S-19.02 v1.12 + S-19.06 v1.15 + S-19.07 v1.12). **CLOSED F-P27-001 epic leg.**

**Leg 3 — Story-writer (S-19.02):** S-19.02 v1.11→v1.12. Test-set cite corrected to match BC-4.13.001 v1.10 exclusive boundary; BC-4.13.001 v1.9→v1.10 cite sweep (all AC/Task/Token Budget sites). Whole-file predicate run confirmed. Input-hash ccd11cf→59d0856 (content changed). POLICY 14 5-leg parity applied. **CLOSED F-P27-002 (S-19.02 leg).**

**Leg 4 — Story-writer (S-19.06):** S-19.06 v1.14→v1.15. Boundary table column header corrected per BC-4.13.001 v1.10 exclusive boundary; BC-4.13.001 v1.9→v1.10 cite sweep applied. Whole-file predicate run confirmed. Input-hash 5af0d9f unchanged (boundary header is a non-hash-affecting metadata field). POLICY 14 5-leg parity applied. **CLOSED F-P27-002 (S-19.06 leg).**

**Leg 5 — Story-writer (S-19.07):** S-19.07 v1.11→v1.12. DISTINCT-block parity fix applied — §Previous Story Intel S-19.06 Patterns Established row and §Architecture Mapping updated to DISTINCT "Capability Schemas" framing (separate from S-19.04 block per O-P12-04). BC-4.13.001 v1.9→v1.10 cite sweep applied. Whole-file tense predicate run confirmed. Input-hash 01bed1d→82287d6 (content changed). POLICY 14 5-leg parity applied. **CLOSED F-P27-001 (S-19.07 leg).**

**Leg 6 — State-manager (index bumps + O-P27-001):** BC-INDEX v3.80→v3.81 (BC-4.13.001 catalog row version cell v1.9→v1.9|v1.10; BC-2.02.011 catalog row status "draft"→"active"). STORY-INDEX v4.158→v4.159 (S-19.02 row v1.11→v1.12 hash ccd11cf→59d0856; S-19.06 row v1.14→v1.15 hash 5af0d9f unchanged; S-19.07 row v1.11→v1.12 hash 01bed1d→82287d6; epic header v1.16→v1.17; input-hash table S-19.02+S-19.07 updated; BC coverage footer BC-4.13.001 v1.9→v1.10). E-19 epic input-hash corrected 0ff893e→bf647fc. VP-INDEX v2.53 UNCHANGED. ARCH-INDEX v2.94 UNCHANGED. D-494 gate PASS (4 index versions verified literal-shell). **CLOSED O-P27-001.**

**Leg 7 — State-manager (STATE.md advance + adv persist + decision-log):** STATE.md v5.31→v5.32 (frontmatter phase D-781-E19-ADV-PASS-27-CLOSED, trajectory tail →4→2→2→4, session resume checkpoint refresh). adv-E19-pass-27.md persisted (this file). INDEX.md pass-27 row appended + Convergence Status updated. Decision-log D-781 codified. **CLOSED all governance legs.**

**O-P27-001:** CLOSED — BC-INDEX catalog row BC-2.02.011 status "draft"→"active" per Leg 6 above.
