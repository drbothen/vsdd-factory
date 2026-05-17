---
document_type: adversary-pass-report
level: ops
title: "S-15.09 LOCAL Adversary Cascade — Pass 3"
producer: adversary
timestamp: 2026-05-17T00:00:00Z
phase: per-story-step-4-local-adversary-cascade
story: S-15.09
pass: 3
verdict: MEDIUM
finding_count: { critical: 0, high: 0, medium: 2, low: 1, nitpick: 1, process_gap: 0 }
streak_3_clean: "0/3"
---

# S-15.09 LOCAL Adversary Cascade — Pass 3

## Part A — Findings

### F-P3-001 — MEDIUM — `extract_trajectory_tail_line` full-document fallback picks first qualifying narrative line, not "the trajectory tail"

- **Location:** `crates/hook-plugins/validate-state-structure/src/lib.rs:359-379` (`extract_trajectory_tail_line` body-document fallback)
- **Evidence:** Real STATE.md `.factory/STATE.md:69` contains `| F5 passes 3-7 cycle-level adversary | **COMPLETE** | Trajectory 11→9→8→7→5; verdict MEDIUM at pass-7 ...` — adjacent run `→9→8→7→5` (4 components). The pass-2 narrative-arrow discriminator only applied to the BANNER block scan, not the full-document fallback. Lines 90, 96, 99 of real STATE.md have 9, 12, 14 adjacent arrow-digit components respectively.
- **Issue:** Pass-2 F-P2-001 fix (`has_adjacent_arrow_digit_run` predicate requiring ≥3 components) closed the banner-block class but the full-document fallback applies the SAME weak predicate. Real STATE.md passes ONLY by coincidence (line 69 happens to have 4 components, matching canonical). Any state-manager edit that moves a higher-count narrative line ahead of line 69 would convert the hook into a false-positive blocker. This is a sibling-site miss (TD-VSDD-060).
- **Recommendation:** Anchor the trajectory tail on a structural marker. Preferred: discriminate on first-`→`-precursor — canonical `Trajectory →9→9→9→9` has whitespace before first `→`, narrative `Trajectory 11→9→8→7→5` has digit before first `→`. Add regression unit + bats tests covering body-narrative class.

### F-P3-002 — LOW — Snapshot fixture `pass-real-state-md-snapshot/factory/STATE.md` is frozen; does not auto-track real STATE.md

- **Location:** `plugins/vsdd-factory/tests/validate-state-structure/pass-real-state-md-snapshot.bats:24` (reads frozen fixture) vs `lib.rs:1236-1275` (`test_BC_5_39_005_full_validation_against_real_state_md` reads live)
- **Issue:** The bats fixture is a static snapshot; the Rust unit test reads live STATE.md. Drift class: state-manager fix-burst amends STATE.md, snapshot becomes stale, bats keeps passing, Rust test detects drift. Bats fixture freezes line ordering — combined with F-P3-001's line-ordering dependence, this masks regression detection at the bats layer.
- **Recommendation:** Replace fixture with auto-copy in setup (`cp .factory/STATE.md $WORK/.factory/`) — fixture-free, always tracks live.

### F-P3-003 — MEDIUM — Story spec function-name drift: `check_*` / `count_actual_lines` in spec vs `validate_*` / `count_newlines` in implementation

- **Location:** `.factory/stories/S-15.09-validate-state-structure-phase-1.md:332-345` (T-6), :462-475 (Architecture Mapping), :247 (AC-13 cites `check_banner_line_count` etc.)
- **Issue:** Story spec authored at 26490be7 prescribed function names that the implementer chose differently (more idiomatic). Spec-implementation drift. Per CLAUDE.md Standing Rule 3 "code-vs-spec — SPEC wins" but the implementer's names are CLEARER (`validate_*` matches `Violation` return type). The implementation should ship; the SPEC should be updated to match (production-grade default Rule 6: mechanical question answered in-scope).
- **Recommendation:** Update T-6 + Architecture Mapping + AC-13 narrative function names to match implementation: `validate_banner_wc`, `validate_dual_margin`, `validate_trajectory_tail`, `count_newlines`.

### F-P3-004 — NITPICK — Spec narrative on payload `content` vs `host::read_file`

- **Location:** Story spec T-7 Performance Considerations AND BC-5.39.005 Precondition 3 (drift in both)
- **Issue:** Spec says "implementer should check whether the payload includes content before calling `host::read_file`." Implementation always calls `host::read_file` for filesystem authority. Spec narrative inaccurate.
- **Recommendation:** Update both narratives to reflect implementation: filesystem authority always.

## Part B — Production-Grade Default Audit

- **F-P3-001** is a sibling-site sweep gap (TD-VSDD-060): pass-2 fixed the symptom site (banner) but not the structural class (any narrative-arrow location). Fresh-context adversary correctly identified the missed sibling.
- **F-P3-002** is the snapshot defer-pattern: relying on manual refresh of a frozen fixture is the kind of "fix later" rationalization the production-grade default forbids. Auto-copy eliminates the drift class.
- **F-P3-003** is acknowledged spec-implementation drift. The implementer's choice is BETTER than the spec's, but the spec was not updated. Per Rule 4 (AI-built defects are AI's responsibility): fix in scope.
- **No "MVP," "TODO for architect," paper-fix observed.**
- **Sibling-site sweep applied** to BC Invariant 9 extension (banner→body), VP rows, test vectors.

## Part C — Self-Application Audit (META-LEVEL)

- Iron Law respected: no prior pass reports read.
- Fresh-context compounding-value pattern validated: pass-2 saw "banner narrative arrows," pass-3 sees "narrative arrows anywhere." Same defect class, sibling site.
- Production-target recursion: direct grep against `.factory/STATE.md` confirmed line 69 has 4 components, lines 90/96/99 have 9/12/14. Implementation passes by coincidence.
- POLICY 13 + POLICY 15 compliance: regex-alternation grep + verbatim file:line evidence used throughout.
- NOVELTY: HIGH. F-P3-001 is genuinely new — fresh context exposed the axis-specific nature of the pass-2 fix.

## Verdict & Streak

- Pass-3 verdict: **MEDIUM** (2M + 1L + 1N).
- Streak: **0/3** (≥LOW findings reset).
- Mandatory fix-burst before pass-4.

## Fix-burst routing (orchestrator routed; complete)

- implementer @ `304c1bdd` — F-P3-001 closed (first-arrow-precursor + two-pass body fallback; 4 unit + 1 bats regression tests; 24/24 S-15.09 bats; 4-gate PASS)
- test-writer @ `ec21a5d7` — F-P3-002 closed (auto-copy mechanism; snapshot drift class structurally eliminated)
- product-owner @ `7e6fc312` factory-artifacts — F-P3-001 BC half + F-P3-004 BC half closed (BC v1.1→v1.2; Invariant 9 generalized to all-locations; EC-016/017 + VP rows + test vectors added)
- story-writer @ `2b85f2c7` factory-artifacts — F-P3-003 + F-P3-004 story half closed (story v1.2→v1.3; 17 function-name occurrences updated; T-7 narrative aligned)
- state-manager — this persistence commit

## Closure verification

Implementer's regression tests `test_BC_5_39_005_f_p3_001_narrative_arrow_burst_not_trajectory` + `test_BC_5_39_005_f_p3_001_high_count_narrative_not_trajectory` + `test_BC_5_39_005_f_p3_001_body_narrative_before_canonical_finds_canonical` + `test_BC_5_39_005_f_p3_001_injected_narrative_does_not_displace_canonical_tail` lock the body-narrative discriminator structurally. F-P3-001 closed STRUCTURALLY, not paper-fixed.
