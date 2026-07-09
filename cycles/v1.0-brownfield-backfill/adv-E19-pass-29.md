# Adversarial Review — E-19 Pass 29 (post-D-782 delta; perimeter = S-19.02 v1.13 + S-19.05 v1.14 + S-19.06 v1.16 + epic v1.18 + VP-096 v1.1 + BC-1.17.001 v1.4)

**Perimeter:** E-19 epic v1.18 + S-19.01..S-19.07 + STORY-INDEX E-19 section + VP-INDEX VP-094..VP-101 + BC-5.42.001 v1.3 + BC-4.13.001 v1.10 + BC-2.07.001 v1.2 + BC-2.02.011 v1.5 + BC-3.08.001 v1.19 + BC-1.17.001 v1.4
**Reviewer:** fresh-context adversary (Iron Law; rubric = policies.yaml v1.4.1)
**Date:** 2026-07-09
**Verdict:** NOT-CLEAN — BLOCKER 0 / HIGH 2 / MEDIUM 2 / LOW 1 (5 total)
**Streak:** 0/3 (pass-29 NOT-CLEAN; severity increase from B0/H0/M3/L3 → B0/H2/M2/L1)
**Model family:** Claude Opus 4.7
**Delta artifact versions verified:** S-19.02 v1.13 (was v1.12); S-19.05 v1.14 (was v1.13); S-19.06 v1.16 (was v1.15); E-19 epic v1.18 (was v1.17); VP-096.md v1.1 (was v1.0); BC-1.17.001 v1.4 (was v1.3).

## Part A — D-782 Delta Verification + New Findings

### Amendment 1 — S-19.02 v1.12 → v1.13 (VP-096 proptest Task 11 added)

F-P28-003 fix applied — new Task 11 (proptest harness for VP-096 byte-exact prefix property + determinism) ✓. Old Tasks 11→12, 12→13, 13→14 renumbered ✓. T-010 added (AC-005 proptest module, VP-096) ✓. Library & Framework proptest row added (workspace-pinned v1.6) ✓. File Structure lib.rs row extended (T-010 proptest module) ✓. Input-hash 59d0856 unchanged (non-hash-affecting proptest task addition) ✓. POLICY 14 5-leg parity confirmed ✓.

However, two new defects are present in the v1.13 content:

**F-P29-001 HIGH — S-19.02 v1.13 cites `extract_frontmatter` in `crates/hook-sdk/src/lib.rs` — wrong crate location.**

S-19.02 v1.13 Task 1 (implementation anchor), §Previous Story Intel rows referencing `extract_frontmatter`, and the §File Structure entry for the function all cite `crates/hook-sdk/src/lib.rs` as the home of `extract_frontmatter`. This is wrong.

`extract_frontmatter` is a pure function: it takes a `&[u8]` slice and returns a `Result<&[u8], FrontmatterError>` with no I/O, no capability side-effects, and no host FFI. Placing it in `hook-sdk` (an I/O-containing, capability-aware crate) violates the purity separation principle of ADR-025 and creates an inverted dependency: `hook-sdk` has I/O-boundary functions that would co-reside with a pure parsing function. The correct location per the purity separation principle is `crates/factory-lock-parse/src/lib.rs` — a pure crate with no I/O dependencies. Additionally, `factory-lock-parse` is already the canonical home for `parse_factory_lock` and all STATE.md parsing logic; `extract_frontmatter` is logically the same crate's function. All lib-crate functions in the dispatcher expose a test-build path under WASM compilation constraints, and `factory-lock-parse` is the crate that satisfies those WASM test-build constraints for pure parsing functions.

A count of the citation sites in S-19.02 v1.13 yields 8 loci: Task 1 implementation anchor path, §File Structure `lib.rs` row, §Previous Story Intel S-19.02 Patterns Established row, §Architecture Mapping extract_frontmatter row, §Verification Properties VP-095 locus entry, §Verification Properties VP-096 locus entry, Token Budget file-list cite, AC-005 path reference in Gate 1.

**Locus:** S-19.02 v1.13 — 8 citation sites for `extract_frontmatter` crate path (hook-sdk → factory-lock-parse).
**Routing:** architect (crate-location ruling) → story-writer (S-19.02 relocation sweep).
**Fix:** Architect text-ruling: `extract_frontmatter` belongs in `crates/factory-lock-parse/src/lib.rs` (purity separation per ADR-025; lib-crate function parity; WASM test-build constraints). S-19.02 v1.13→v1.14: 8-site relocation sweep (all `crates/hook-sdk/src/lib.rs` citations for `extract_frontmatter` → `crates/factory-lock-parse/src/lib.rs`). Input-hash da5acd7 (content changed). POLICY 14 5-leg parity applied. **CLOSED F-P29-001.**

**F-P29-002 HIGH — S-19.02 v1.13 Task 11 pins proptest via volatile `Cargo.toml:102` line number reference.**

Task 11 in S-19.02 v1.13 (the newly added proptest task) describes the proptest workspace pin as "added at `Cargo.toml:102`." This is a TD-VSDD-091 violation: volatile line-number pins in spec content decay on every subsequent diff to `Cargo.toml`. The proptest pin will not remain at line 102 once other workspace dependencies are added, removed, or reordered — and the spec's AC/Task wording will silently become wrong. The correct form for spec content is a behavioral anchor (e.g., "proptest workspace pin present in `Cargo.toml` `[workspace.dependencies]` section") or a TOML key anchor (e.g., `proptest = { version = "1.6", ...}` without a line-number citation).

**Locus:** S-19.02 v1.13 Task 11 — `Cargo.toml:102` volatile line-pin.
**Routing:** story-writer.
**Fix:** S-19.02 v1.13→v1.14: Task 11 `Cargo.toml:102` cite replaced with stable behavioral anchor (proptest workspace pin in `[workspace.dependencies]` without line-number). Input-hash da5acd7 (absorbed into same v1.14 sweep as F-P29-001). **CLOSED F-P29-002.**

**F-P29-003 MEDIUM — S-19.02 v1.13 Task 11 references VP-096 with its inclusive-framing title from before VP-INDEX v2.54 update.**

Task 11 of S-19.02 v1.13 introduces the VP-096 proptest task and refers to VP-096 as "Output Byte-Equals File Prefix Through Second --- Delimiter." This is the pre-v2.54 inclusive framing, which was corrected by F-P28-001 (VP-INDEX v2.53→v2.54) to "Up To (Excluding) the Second --- Delimiter Line (bytes 0..delimiter_start_offset)." The F-P28-003 fix that authored Task 11 was produced in the same burst as the F-P28-001 VP-096 title correction, but the Task 11 prose was authored with the stale inclusive title rather than the newly canonical exclusive form.

**Locus:** S-19.02 v1.13 Task 11 — VP-096 title cite ("Through Second --- Delimiter" inclusive form).
**Routing:** story-writer.
**Fix:** S-19.02 v1.13→v1.14: Task 11 VP-096 title cite updated to exclusive form per VP-INDEX v2.54 canonical title. Input-hash da5acd7 (absorbed into v1.14 sweep). **CLOSED F-P29-003.**

### Amendment 2 — S-19.05 v1.13 → v1.14 (awk gate presence-prechecks)

O-P28-02 fix applied — AC-004 ENV_SINK_FILE awk gate hardened with `grep -q` presence-precheck ✓. flush_sink_file awk gate hardened ✓. T-006 Mutex awk gate hardened with `grep -qE '^use std::sync::.*Mutex'` presence-precheck ✓. Absent-symbol fixture evidence added ✓. Input-hash 9e54d68 unchanged ✓. POLICY 14 5-leg parity confirmed ✓. No new findings.

### Amendment 3 — S-19.06 v1.15 → v1.16 (BC-1.17.001 v1.4 cite propagation)

F-P28-002 propagation to S-19.06 applied — BC-1.17.001 v1.3→v1.4 proof-method-only cite sweep (Narrative ×1, AC-007 ×4, BC table version ×1, Token Budget ×1, Architecture Compliance Rules ×3 = 10 sites) ✓. Input-hash 5af0d9f→a8bb758 (content changed) ✓. POLICY 14 5-leg parity confirmed ✓. No new findings.

### Amendment 4 — E-19 epic v1.17 → v1.18 (BC-1.17.001 v1.4 cite propagation)

F-P28-002 epic propagation applied — BC-1.17.001 v1.3→v1.4 all body cites swept ✓. Input-hash bf647fc→6e3bb2c ✓. POLICY 14 5-leg parity confirmed ✓. No new findings.

### Amendment 5 — VP-096.md v1.0 → v1.1 (exclusive boundary title correction)

F-P28-001 fix applied — VP-096.md title and description updated to exclusive form ("Up To (Excluding) the Second --- Delimiter Line (bytes 0..delimiter_start_offset; opening ---\n included)") ✓. v1.1 changelog note appended ✓. Input-hash 6af1247 ✓. POLICY 9 same-burst propagation confirmed (verification-architecture.md v1.8, verification-coverage-matrix.md v1.5) ✓. No new findings.

### Amendment 6 — BC-1.17.001 v1.3 → v1.4 (VP-101 proof-method alignment)

F-P28-002 fix applied — §Traceability VP-101 proof-method aligned to VP-INDEX `integration` classification ✓. 10-site sweep confirmed ✓. H1 title UNCHANGED (POLICY 7) ✓. Input-hash adbb5f3 ✓. POLICY 14 5-leg parity confirmed ✓. No new findings.

### Full E-19 Epic and Story Suite Review

**F-P29-004 MEDIUM — BC-4.13.001 v1.10 §Verification Properties section carries "VP Anchors TBD" despite VP-095 and VP-096 being authoritatively assigned in VP-INDEX v2.54.**

BC-4.13.001 v1.10 §Verification Properties (or equivalent VP-anchoring section) contains a placeholder indicating VP assignment is pending ("VP Anchors TBD" or substantively equivalent language that does not cite VP-095 and VP-096). VP-INDEX v2.54 Full Index explicitly records:

- VP-095 (Story Anchors: BC-4.13.001 v1.10 AC-006 locus, BC-5.42.001 v1.3 AC-007 locus)
- VP-096 (Story Anchors: BC-4.13.001 v1.10 Invariant 9 locus, S-19.02 v1.13 AC-005 locus)

Both VPs are authoritatively assigned and their BC-4.13.001 loci are recorded in VP-INDEX. Yet BC-4.13.001 v1.10's own body does not back-cite VP-095/VP-096 in its §Verification Properties rows — the VP references have not been propagated from VP-INDEX into the BC body per POLICY 9. This is a POLICY 9 same-burst propagation miss: when VP-095 and VP-096 were assigned (in earlier passes), the corresponding BC-4.13.001 §Verification Properties rows were not updated.

**Locus:** BC-4.13.001 v1.10 §Verification Properties — VP-095 and VP-096 back-citations absent.
**Routing:** product-owner (BC-4.13.001 body content).
**Fix:** BC-4.13.001 v1.10→v1.11: §Verification Properties VP Anchors TBD retired; VP-095 (AC-006 byte-boundary boundary property locus) and VP-096 (Invariant 9 extract_frontmatter purity locus) added as explicit VP rows with proof-method citations per VP-INDEX v2.54 canonical form. H1 title UNCHANGED (POLICY 7). POLICY 14 5-leg parity applied. Input-hash 26d21bf. **CLOSED F-P29-004.**

No additional findings on S-19.01 v1.14, S-19.03 v1.14, S-19.04 v1.11, ADR-030 v1.3, BC-5.42.001 v1.3, BC-2.07.001 v1.2, BC-2.02.011 v1.5, BC-3.08.001 v1.19.

## Observations

**O-P29-01 [LOW; housekeeping] — STORY-INDEX S-19.06 v1.15 row description misattributes the v1.15 change to F-P27-002 boundary header correction when v1.15 actually fixed F-P27-001 DISTINCT-block form.**

The STORY-INDEX v4.160 row for S-19.06 records the v1.15 history entry as "F-P27-002 boundary table column header corrected." However, S-19.06 v1.15's own frontmatter changelog records the v1.15 change as the F-P27-001 fix: `§Depends on S-19.04` prose converted from `'extends that section'` to DISTINCT-block form (a separate "Capability Schemas" preamble block distinct from S-19.04's tool-filter-anchoring block; ordering-only dependency). The F-P27-002 boundary table column header correction was applied to S-19.02 v1.12, not S-19.06 v1.15. The STORY-INDEX row description is misattributed.

**Routing:** state-manager (STORY-INDEX catalog row maintenance).
**Fix:** STORY-INDEX v4.160→v4.161: S-19.06 v1.15 history entry corrected from "F-P27-002 boundary table column header corrected" to "F-P27-001 §Depends on S-19.04 prose fixed to DISTINCT-block form." **CLOSED O-P29-01.**

## Part B — Dimensions

| Dimension | Status |
|-----------|--------|
| Dim-1: BC/VP coverage | FAIL — BC-4.13.001 v1.10 VP Anchors TBD (F-P29-004); S-19.02 v1.13 VP-096 stale title cite (F-P29-003). CLOSED post-burst |
| Dim-2: AC gate execution | PASS — no gate-bearing AC changes in D-782 delta artifacts (proptest Task 11 gate content is new but non-conflicting); awk prechecks in S-19.05 v1.14 verified structurally |
| Dim-3: POLICY 14 5-leg parity | PASS on all D-782 delta amendments (S-19.02 v1.13 / S-19.05 v1.14 / S-19.06 v1.16 / epic v1.18 / VP-096 v1.1 / BC-1.17.001 v1.4) |
| Dim-4: POLICY 8 BC propagation | FAIL — BC-4.13.001 v1.10 VP back-citations absent (F-P29-004 same-burst propagation miss). CLOSED post-burst |
| Dim-5: Input-hash consistency | PASS — S-19.02 59d0856, S-19.05 9e54d68, S-19.06 a8bb758, epic 6e3bb2c match expected values at time of review |
| Dim-6: STORY-INDEX BC-coverage | PASS — STORY-INDEX v4.160 row versions match frontmatter versions at time of review |
| Dim-7: ADR/BC interface parity | PASS — ADR-025 purity separation correctly motivates F-P29-001 routing; architect ruling aligns to ADR-025 canonical form |

## Summary

| Severity | Count |
|----------|-------|
| BLOCKER | 0 |
| HIGH | 2 |
| MEDIUM | 2 |
| LOW | 1 (1 actionable observation) |

**Severity regression: B0/H0/M3/L3 (pass-28) → B0/H2/M2/L1 (pass-29). Two HIGH findings introduced.** Regression class analysis: F-P29-001 is a crate-location defect in the newly added proptest Task 11 content (extract_frontmatter cited in wrong crate — hook-sdk instead of factory-lock-parse; 8 loci); F-P29-002 is a TD-VSDD-091 volatile-pin defect in the same Task 11 (Cargo.toml:102 line reference); F-P29-003 is a stale VP-096 inclusive-title cite in the same Task 11 (authored during same burst as the title correction but used pre-correction language); F-P29-004 is a POLICY 9 same-burst propagation miss from earlier passes (BC-4.13.001 VP Anchors not back-cited despite VP-INDEX authoritative assignment). O-P29-01 is a STORY-INDEX row misattribution (F-P27-002 vs F-P27-001). D-783 fix burst applied (architect text-ruling + PO BC-4.13.001 v1.11 + SW S-19.02 v1.14 + SW S-19.07 v1.13 + SM 4-index + STATE.md).

**Overall Assessment:** NOT-CLEAN — B0/H2/M2/L1. Streak 0/3. NEXT: pass-30.

## Novelty Assessment

| Field | Value |
|-------|-------|
| Pass | 29 |
| New findings | 4 |
| New observations | 1 |
| Duplicate/variant findings | 0 (F-P29-001 is a new class: crate-location defect via wrong-crate citation in Task-authored content; F-P29-002 is a new locus for TD-VSDD-091 volatile-pin class (first occurrence in proptest Task content); F-P29-003 is stale-VP-title class with new locus (Task 11 VP-096 cite); F-P29-004 is POLICY 9 same-burst propagation miss with new locus (BC-4.13.001 VP Anchors) — all locus-distinct) |
| Novelty score | MEDIUM-HIGH — F-P29-001 crate-location defect class is novel for this cascade (prior findings were citation-drift and boundary-form classes); F-P29-002/003 are known class regressions with new loci; F-P29-004 is a known POLICY 9 propagation miss at a new locus |
| Median severity | HIGH |
| Verdict | NOT-CLEAN — streak 0/3; pass-30 NEXT |

## Coverage Attestation

Artifacts read in full: BC-4.13.001 v1.10 (1-end); BC-1.17.001 v1.4 (1-end); S-19.02 v1.13 (1-end); S-19.05 v1.14 (1-end); S-19.06 v1.16 (1-end); VP-INDEX v2.54 §Full Index VP-094..VP-101; VP-096.md v1.1 (1-end).
Spot-checked (no changes): ADR-030 v1.3; BC-5.42.001 v1.3; BC-2.02.011 v1.5; BC-2.07.001 v1.2; BC-3.08.001 v1.19; S-19.01 v1.14; S-19.03 v1.14; S-19.04 v1.11; E-19 epic v1.18; STORY-INDEX E-19 section v4.160.
Not read (Iron Law): decision-log; burst-log; lessons.md; STATE.md; checkpoints; adv passes 1-28; fix-burst records.

## Fix Burst Closure (D-783)

**Architect text-ruling (F-P29-001 crate-location):** `extract_frontmatter` belongs in `crates/factory-lock-parse/src/lib.rs`. Rationale: purity separation per ADR-025 — pure parsing functions must not co-reside with I/O-boundary functions in `hook-sdk`; `factory-lock-parse` is the canonical pure-crate home for all STATE.md parsing logic (already houses `parse_factory_lock`); WASM test-build constraints are satisfied by the pure-crate boundary; lib-crate function parity requires `extract_frontmatter` to be callable from both the dispatcher and test harnesses without pulling in `hook-sdk` I/O dependencies. Text-ruling applies to S-19.02 (8-site relocation sweep) and S-19.07 (§Previous Story Intel S-19.02 Patterns Established row correction). No file edit by architect required — ruling is codified in this adversary report and in D-783. **Architect ruling recorded.**

**Leg 1 — Product-owner (BC-4.13.001):** BC-4.13.001 v1.10→v1.11 — §Verification Properties VP Anchors TBD retired; VP-095 (AC-006 byte-boundary property locus; proof_method integration) and VP-096 (Invariant 9 extract_frontmatter purity locus; proof_method proptest) added as explicit VP rows with VP-INDEX v2.54 back-citations. H1 title UNCHANGED (POLICY 7). Input-hash 26d21bf. POLICY 14 5-leg parity applied. **CLOSED F-P29-004.**

**Leg 2 — Story-writer (S-19.02):** S-19.02 v1.13→v1.14 — (i) 8-site crate-path relocation sweep: all `crates/hook-sdk/src/lib.rs` citations for `extract_frontmatter` replaced with `crates/factory-lock-parse/src/lib.rs` (Task 1, §File Structure lib.rs row, §Previous Story Intel S-19.02 row, §Architecture Mapping row, §Verification Properties VP-095 locus, §Verification Properties VP-096 locus, Token Budget file-list cite, AC-005 Gate 1 path reference); (ii) Task 11 `Cargo.toml:102` volatile pin replaced with stable behavioral anchor (proptest workspace pin in `[workspace.dependencies]`); (iii) Task 11 VP-096 title updated from stale inclusive form to VP-INDEX v2.54 canonical exclusive form ("Up To (Excluding) the Second --- Delimiter Line (bytes 0..delimiter_start_offset)"); (iv) BC-4.13.001 v1.10→v1.11 cite sweep (all body cites). Input-hash da5acd7. POLICY 14 5-leg parity applied. **CLOSED F-P29-001, F-P29-002, F-P29-003.**

**Leg 3 — Story-writer (S-19.07):** S-19.07 v1.12→v1.13 — (i) BC-4.13.001 v1.10→v1.11 cite sweep (all body cites per BC-4.13.001 v1.11 cascade); (ii) §Previous Story Intel S-19.02 Patterns Established row `extract_frontmatter` crate path corrected to `crates/factory-lock-parse/src/lib.rs` per architect text-ruling. Input-hash 6bb4361. POLICY 14 5-leg parity applied. **Propagation of F-P29-001 fix to S-19.07.**

**Leg 4 — State-manager (4-index bumps + O-P29-01):** BC-INDEX v3.82→v3.83 (BC-4.13.001 catalog row version cell v1.10→v1.10|v1.11; F-P29-004 cite). STORY-INDEX v4.160→v4.161 (S-19.02 row v1.13→v1.14 hash da5acd7; S-19.07 row v1.12→v1.13 hash 6bb4361; S-19.06 v1.15 description corrected per O-P29-01; BC coverage BC-4.13.001 v1.10→v1.11; DAG footnote hashes updated). VP-INDEX v2.54 UNCHANGED. ARCH-INDEX v2.95 UNCHANGED. D-494 gate PASS (literal-shell stdout: STORY-INDEX v4.161 / BC-INDEX v3.83 / ARCH-INDEX v2.95 / VP-INDEX v2.54). **CLOSED O-P29-01.**

**Leg 5 — State-manager (STATE.md advance + adv persist + decision-log):** STATE.md v5.33→v5.34 (frontmatter phase D-783-E19-ADV-PASS-29-NOT-CLEAN-CLOSED, trajectory tail →2→4→6→5, session resume checkpoint refresh). adv-E19-pass-29.md persisted (this file). INDEX.md pass-29 row appended + Convergence Status updated. Decision-log D-783 codified. **CLOSED all governance legs.**
