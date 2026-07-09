# Adversarial Review — E-19 Pass 28 (post-D-781 delta; perimeter = S-19.02 v1.12 + S-19.06 v1.15 + S-19.07 v1.12)

**Perimeter:** E-19 epic v1.17 + S-19.01..S-19.07 + STORY-INDEX E-19 section + VP-INDEX VP-094..VP-101 + BC-5.42.001 v1.3 + BC-4.13.001 v1.10 + BC-2.07.001 v1.2 + BC-2.02.011 v1.5 + BC-3.08.001 v1.19 + BC-1.17.001 v1.3
**Reviewer:** fresh-context adversary (Iron Law; rubric = policies.yaml v1.4.1)
**Date:** 2026-07-08
**Verdict:** NOT-CLEAN — BLOCKER 0 / HIGH 0 / MEDIUM 3 / LOW 3 (6 total; first zero-HIGH zero-BLOCKER of re-cascade)
**Streak:** 0/3 (pass-28 NOT-CLEAN; severity decrease from B0/H1/M2/L1 → B0/H0/M3/L3)
**Model family:** Claude Opus 4.7
**Delta artifact versions verified:** S-19.02 v1.12 (was v1.11); S-19.06 v1.15 (was v1.14); S-19.07 v1.12 (was v1.11).

## Part A — D-781 Delta Verification + New Findings

### Amendment 1 — S-19.02 v1.11 → v1.12 (test-set cite corrected per BC-4.13.001 v1.10 exclusive boundary)

Test-set cite in AC-005 boundary framing corrected to match BC-4.13.001 v1.10 exclusive form (`0..delimiter_start_offset`) ✓. BC-4.13.001 v1.9→v1.10 cite sweep confirmed across all AC/Task/Token Budget sites ✓. Whole-file predicate run confirmed no additional stale v1.9 cites ✓. Input-hash ccd11cf→59d0856 (content changed) ✓. POLICY 14 5-leg parity confirmed ✓.

**F-P28-003 MEDIUM — S-19.02 v1.12 has no proptest Task or test for VP-096.**

S-19.02 v1.12 wires VP-095 and VP-096 in its `behavioral_contracts` frontmatter and `verification_properties` references. VP-095 is covered by AC-006 (integration test for the 262144-byte size boundary). VP-096 is documented as the `extract_frontmatter` purity proptest property (BC-4.13.001 Invariant 9 — byte-exact prefix, exclusive boundary, deterministic for all symbolic inputs). However, S-19.02 v1.12 contains no Task or acceptance criterion for a proptest harness targeting VP-096. The story's task list (Tasks 1–10 post-v1.12) defines AC-005 as a unit test suite (input-hash boundary assertions) but does not include a proptest module for VP-096's byte-equality and determinism property.

VP-096 explicitly calls out `proptest` as its proof method in VP-INDEX (Proof Method column: proptest). Per POLICY 9 same-burst propagation and BC-5.39.001 convergence discipline, a VP wired to a story with `proof_method: proptest` must have a corresponding proptest task in the story.

**Locus:** S-19.02 v1.12 Task list — VP-096 proptest harness absent.
**Routing:** story-writer.
**Fix:** S-19.02 v1.12→v1.13: new Task 11 added (proptest harness for VP-096: `extract_frontmatter` byte-exact prefix property and determinism; BC-4.13.001 v1.10 Invariant 9 exclusive boundary authoritative; VP-INDEX "Through Second --- Delimiter" framing is inclusive — BC exclusive form is authoritative); old Tasks 11→12, 12→13, 13→14 renumbered; T-010 added (AC-005 proptest module, VP-096); Library & Framework proptest row added (workspace-pinned v1.6); File Structure lib.rs row extended (T-010 proptest module). Input-hash 59d0856 unchanged (no hash-affecting behavioral content changed in proptest addition — proptest wiring references same AC-005 module path). POLICY 14 5-leg parity applied. **CLOSED F-P28-003.**

### Amendment 2 — S-19.06 v1.14 → v1.15 (boundary table column header corrected)

Boundary table column header corrected per BC-4.13.001 v1.10 exclusive boundary form ✓. BC-4.13.001 v1.9→v1.10 cite sweep applied ✓. Whole-file predicate run confirmed ✓. Input-hash 5af0d9f unchanged (boundary header correction is non-hash-affecting) ✓. POLICY 14 5-leg parity confirmed ✓.

### Amendment 3 — S-19.07 v1.11 → v1.12 (DISTINCT-block parity fix + BC-4.13.001 v1.10 cite sweep)

DISTINCT-block parity fix applied — §Previous Story Intel S-19.06 row and §Architecture Mapping updated to reflect DISTINCT "Capability Schemas" preamble block (separate from S-19.04's tool-filter-anchoring block per O-P12-04) ✓. BC-4.13.001 v1.9→v1.10 cite sweep confirmed ✓. Whole-file tense predicate run confirmed ✓. Input-hash 01bed1d→82287d6 (content changed) ✓. POLICY 14 5-leg parity confirmed ✓.

### Full E-19 Epic and Story Suite Review

**F-P28-001 MEDIUM — VP-INDEX VP-096 title and description use inclusive "Through Second --- Delimiter" framing inconsistent with BC-4.13.001 v1.10 Invariant 9 exclusive `0..delimiter_start_offset` form.**

VP-INDEX v2.53 Full Index row for VP-096 reads:

> `extract_frontmatter Purity — Output Byte-Equals File Prefix Through Second --- Delimiter; Deterministic for Any Input`

BC-4.13.001 v1.10 Invariant 9 (the normative source for VP-096) states the extraction boundary as `0..delimiter_start_offset` — an exclusive upper bound, meaning the byte at `delimiter_start_offset` (the start of the closing `---` line) is NOT included in the output. The phrase "Through Second --- Delimiter" is inclusive framing — it implies the output includes the second `---` delimiter, which contradicts the v1.10 exclusive boundary.

The VP-INDEX title is the first artifact a reader encounters when looking up VP-096. The inclusive framing is a persistent documentation defect: even though BC-4.13.001 v1.10 Invariant 9 was fixed by F-P27-003, VP-096's title and description in VP-INDEX still carry the pre-v1.10 inclusive language. This is a POLICY 9 same-burst propagation miss from the D-781 fix burst — BC-4.13.001 was updated but VP-096 title in VP-INDEX was not co-updated.

**Locus:** VP-INDEX v2.53 Full Index VP-096 row — title and description column.
**Routing:** state-manager (VP-INDEX catalog row maintenance; VP-096 title sync).
**Fix:** VP-INDEX v2.53→v2.54: VP-096 Full Index row title updated from "Through Second --- Delimiter" to "Up To (Excluding) the Second --- Delimiter Line (bytes 0..delimiter_start_offset; opening ---\n included)"; description updated to include explicit exclusive-boundary note; v1.1 changelog note appended. Story Anchors VP-096 row also updated. POLICY 9: verification-architecture.md + verification-coverage-matrix.md propagated same-burst by architect (VP-096.md v1.1 already updated by architect in same burst). **CLOSED F-P28-001.**

**F-P28-002 MEDIUM — BC-1.17.001 v1.3 Traceability section cites VP-101 with proof-method language that does not align to VP-INDEX integration-only classification per POLICY 9.**

BC-1.17.001 v1.3 §Traceability references VP-101 as the verification property for `host::read_prefix`. The BC body prose describing VP-101's proof method uses language that implies the property can be verified by both unit test and integration test paths. VP-INDEX v2.53 Full Index VP-101 row (Proof Method column) records VP-101 as `integration` exclusively — not `unit-test` and not `proptest`. The BC-1.17.001 v1.3 Traceability prose is inconsistent with the VP-INDEX authoritative Proof Method classification for VP-101.

Per POLICY 9, proof-method classifications for VPs are authoritative in VP-INDEX; BC Traceability sections that cite a VP's proof method must align to VP-INDEX's canonical Proof Method column value.

**Locus:** BC-1.17.001 v1.3 §Traceability — VP-101 proof-method characterization.
**Routing:** product-owner (BC-1.17.001 Traceability body content).
**Fix:** BC-1.17.001 v1.3→v1.4: §Traceability VP-101 proof-method cite aligned to VP-INDEX `integration` classification. Narrative, AC-007 ×4, BC table version, Token Budget, Architecture Compliance Rules ×3 = 10 sites swept. H1 title UNCHANGED (POLICY 7). POLICY 14 5-leg parity applied. Input-hash refreshed (adbb5f3). **CLOSED F-P28-002.**

No additional findings on S-19.01 v1.14, S-19.03 v1.14, S-19.04 v1.11, ADR-030 v1.3, BC-5.42.001 v1.3, BC-4.13.001 v1.10, BC-2.07.001 v1.2, BC-2.02.011 v1.5, BC-3.08.001 v1.19.

## Observations

**O-P28-01 [LOW; informational] — S-19.07 v1.12 Coverage Attestation footnote cites BC-4.13.001 v1.9 in a parenthetical cross-reference note (the artifacts-read list at file end). The normative AC content correctly cites BC-4.13.001 v1.10 throughout. The attestation note is documentation metadata, not a normative clause.**

This is a minor documentation gap: the coverage footnote was last updated when the file was at v1.9 era and was not refreshed during the D-781 tense-predicate sweep (tense-sweep correctly targets normative clauses, not documentation footnotes). No normative content is affected. **Recorded only — no fix required.**

**O-P28-02 [LOW; housekeeping] — S-19.05 v1.13 awk gates for AC-004 (ENV_SINK_FILE, flush_sink_file) and T-006 (Mutex) lack grep -q presence-prechecks.**

The three awk gates fire against live source files to detect the presence of adjacent `#[cfg(...)]` attribute annotation. The awk range patterns match correctly when the symbol is present. However, if the symbol is renamed or removed, the awk gate exits 0 (no match found → no cfg-gate detected → vacuously PASS). This is the same class of vacuous-pass risk that was addressed for the grep-B1 idiom in F-P15-004/005/006. A grep -q presence-precheck (exits non-zero if the symbol is absent) would close the vacuous-pass vector.

**Routing:** story-writer.
**Fix:** S-19.05 v1.13→v1.14: AC-004 ENV_SINK_FILE awk gate and flush_sink_file awk gate each hardened with `grep -q` presence-precheck (exits non-zero if symbol absent OR cfg-gated); T-006 Mutex awk gate hardened with `grep -qE '^use std::sync::.*Mutex'` presence-precheck; absent-symbol fixture evidence: all three precheck commands exit 1 on `/tmp/absent_*.rs` fixtures. Input-hash 9e54d68 unchanged (presence-precheck is a gate-hardening addition to existing tests; no hash-affecting behavioral content changed). POLICY 14 5-leg parity applied. **CLOSED O-P28-02.**

**O-P28-03 [LOW; housekeeping] — STORY-INDEX retirement note for E-19 understates current story count.**

The STORY-INDEX §E-19 retirement note reads "5 stories" reflecting the initial authored set (S-19.01..S-19.05 at epic v1.0). E-19 currently has 7 stories (S-19.01..S-19.07) at 45 pts. The retirement note should be clarified to avoid misleading future readers who consult the note.

**Routing:** state-manager (STORY-INDEX catalog maintenance).
**Fix:** STORY-INDEX retirement note: append `[Historical v1.0 tally; current: 7 stories/45 pts — see row summary]` clarifier. **CLOSED O-P28-03.**

## Part B — Dimensions

| Dimension | Status |
|-----------|--------|
| Dim-1: BC/VP coverage | PASS — VP-INDEX VP-096 exclusive-boundary sync (F-P28-001) closed; VP-101 proof-method alignment (F-P28-002) closed; S-19.02 proptest Task (F-P28-003) closed |
| Dim-2: AC gate execution | Not re-run (frozen delta stories; no gate changes in D-781 delta artifacts) |
| Dim-3: POLICY 14 5-leg parity | PASS on all three D-781 delta amendments (S-19.02 v1.12 / S-19.06 v1.15 / S-19.07 v1.12) |
| Dim-4: POLICY 8 BC propagation | PASS — no BC structural amendments in D-781 delta (BC-4.13.001 v1.10 was the PO leg; parity confirmed) |
| Dim-5: Input-hash consistency | PASS — S-19.02 59d0856, S-19.06 5af0d9f, S-19.07 82287d6 match expected values at time of review |
| Dim-6: STORY-INDEX BC-coverage | PASS — STORY-INDEX v4.159 row versions match frontmatter at time of review |
| Dim-7: ADR/BC interface parity | PASS — ADR-030 v1.3 canonical TOML shape correct; BC-4.13.001 v1.10 exclusive boundary now explicit; VP-INDEX VP-096 sync is index-level correction not ADR interface |

## Summary

| Severity | Count |
|----------|-------|
| BLOCKER | 0 |
| HIGH | 0 |
| MEDIUM | 3 |
| LOW | 3 (2 actionable observations + 1 recorded-only) |

**Severity improvement: B0/H1/M2/L1 (pass-27) → B0/H0/M3/L3 (pass-28). First zero-HIGH zero-BLOCKER pass of the E-19 re-cascade.** Regression class analysis: F-P28-001 is a POLICY 9 same-burst propagation miss from D-781 (BC-4.13.001 updated but VP-096 title not co-updated); F-P28-002 is a proof-method alignment gap (BC-1.17.001 Traceability prose vs VP-INDEX canonical Proof Method column); F-P28-003 is a missing VP-wired proptest Task in S-19.02 (VP-096 proof_method=proptest but no corresponding Task authored). O-P28-01/02/03 are housekeeping observations. D-782 fix burst applied (architect VP-096 v1.1 + verification-architecture.md v1.8 + verification-coverage-matrix.md v1.5 + PO BC-1.17.001 v1.4 + epic v1.18 + SW×3 S-19.02 v1.13 + S-19.05 v1.14 + S-19.06 v1.16 + SM×2 4-index + STATE.md).

**Overall Assessment:** NOT-CLEAN — B0/H0/M3/L3. Streak 0/3. NEXT: pass-29.

## Novelty Assessment

| Field | Value |
|-------|-------|
| Pass | 28 |
| New findings | 3 |
| New observations | 3 |
| Duplicate/variant findings | 0 (F-P28-001 is POLICY 9 propagation miss with new locus VP-INDEX VP-096; F-P28-002 is proof-method alignment class at new locus BC-1.17.001; F-P28-003 is missing-VP-task class at new locus VP-096 proptest; all locus-distinct) |
| Novelty score | MEDIUM — all three MEDIUM findings are known class regressions (POLICY 9 propagation miss; BC-VP proof-method alignment; missing-VP-task) with new loci; severity improvement is structural (HIGH eliminated) |
| Median severity | MEDIUM |
| Verdict | NOT-CLEAN — streak 0/3; pass-29 NEXT |

## Coverage Attestation

Artifacts read in full: BC-4.13.001 v1.10 (1-end); BC-1.17.001 v1.3 (1-end); S-19.02 v1.12 (1-end); S-19.06 v1.15 (1-end); S-19.07 v1.12 (1-end); VP-INDEX v2.53 §Full Index VP-094..VP-101.
Spot-checked (no changes): ADR-030 v1.3; BC-5.42.001 v1.3; BC-2.02.011 v1.5; BC-2.07.001 v1.2; BC-3.08.001 v1.19; S-19.01 v1.14; S-19.03 v1.14; S-19.04 v1.11; S-19.05 v1.13; E-19 epic v1.17; STORY-INDEX E-19 section.
Not read (Iron Law): decision-log; burst-log; lessons.md; STATE.md; checkpoints; adv passes 1-27; fix-burst records.

## Fix Burst Closure (D-782)

**Leg 1 — Architect (VP-096.md + verification-architecture.md + verification-coverage-matrix.md):** VP-096.md v1.0→v1.1 — title and description updated from inclusive "Through Second --- Delimiter" framing to exclusive "Up To (Excluding) the Second --- Delimiter Line (bytes 0..delimiter_start_offset; opening ---\n included)" per BC-4.13.001 v1.10 Invariant 9; v1.1 changelog note appended. Input-hash 6af1247. verification-architecture.md v1.7→v1.8 (VP-096 catalog row updated — same exclusive-boundary language). verification-coverage-matrix.md v1.4→v1.5 (VP-096 SS-04 row updated — exclusive form). POLICY 9 same-burst propagation applied. **CLOSED F-P28-001 architect leg.**

**Leg 2 — Product-owner (BC-1.17.001 + E-19 epic):** BC-1.17.001 v1.3→v1.4 — §Traceability VP-101 proof-method aligned to VP-INDEX `integration` classification; 10-site sweep (Narrative ×1, AC-007 ×4, BC table version ×1, Token Budget ×1, Architecture Compliance Rules ×3). H1 title UNCHANGED (POLICY 7). Input-hash adbb5f3. POLICY 14 5-leg parity applied. E-19 epic v1.17→v1.18 — BC-1.17.001 v1.3→v1.4 cite propagation (all body cites swept); input-hash bf647fc→6e3bb2c. **CLOSED F-P28-002.**

**Leg 3 — Story-writer (S-19.02):** S-19.02 v1.12→v1.13 — new Task 11 added (proptest harness for VP-096 byte-exact prefix property + determinism; BC-4.13.001 v1.10 Invariant 9 exclusive boundary authoritative); old Tasks 11→12, 12→13, 13→14 renumbered; T-010 added (AC-005 proptest module, VP-096); Library & Framework proptest row added (workspace-pinned v1.6); File Structure lib.rs row extended (T-010 proptest module). Input-hash 59d0856 unchanged. POLICY 14 5-leg parity applied. **CLOSED F-P28-003.**

**Leg 4 — Story-writer (S-19.05):** S-19.05 v1.13→v1.14 — AC-004 ENV_SINK_FILE awk gate and flush_sink_file awk gate each hardened with `grep -q` presence-precheck; T-006 Mutex awk gate hardened with `grep -qE '^use std::sync::.*Mutex'` presence-precheck; absent-symbol fixture evidence added. Input-hash 9e54d68 unchanged. POLICY 14 5-leg parity applied. **CLOSED O-P28-02.**

**Leg 5 — Story-writer (S-19.06):** S-19.06 v1.15→v1.16 — BC-1.17.001 v1.3→v1.4 body-scope cite sweep (proof-method-only — Narrative ×1, AC-007 ×4, BC table version ×1, Token Budget ×1, Architecture Compliance Rules ×3 = 10 sites); input-hash refreshed 5af0d9f→a8bb758 (content changed). POLICY 14 5-leg parity applied. **BC-1.17.001 v1.4 cite propagation to S-19.06.**

**Leg 6 — State-manager (4-index bumps + O-P28-03):** BC-INDEX v3.81→v3.82 (BC-1.17.001 catalog row version cell v1.3→v1.3|v1.4). VP-INDEX v2.53→v2.54 (VP-096 Full Index row title + description + v1.1 note; Story Anchors row updated; POLICY 9 propagation confirmed). STORY-INDEX v4.159→v4.160 (S-19.02 row v1.12→v1.13; S-19.05 row v1.13→v1.14; S-19.06 row v1.15→v1.16 hash 5af0d9f→a8bb758; epic header v1.17→v1.18; delivery note updated; BC coverage footer BC-1.17.001 v1.3→v1.4; retirement note clarifier appended per O-P28-03). ARCH-INDEX v2.94→v2.95 (verification-architecture.md v1.4→v1.8 note; verification-coverage-matrix.md v1.2→v1.5 note). D-494 gate PASS (4 index versions verified literal-shell). **CLOSED O-P28-03.**

**Leg 7 — State-manager (STATE.md advance + adv persist + decision-log):** STATE.md v5.32→v5.33 (frontmatter phase D-782-E19-ADV-PASS-28-NOT-CLEAN-CLOSED, trajectory tail →2→2→4→6, session resume checkpoint refresh). adv-E19-pass-28.md persisted (this file). INDEX.md pass-28 row appended + Convergence Status updated. Decision-log D-782 codified. **CLOSED all governance legs.**

**O-P28-01:** Recorded only — no fix required (S-19.07 v1.12 coverage attestation footnote stale BC cite is documentation metadata; normative ACs correctly cite v1.10).
