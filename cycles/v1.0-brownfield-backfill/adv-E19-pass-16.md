# Adversarial Review — E-19 Pass 16 (NOT-CLEAN)

**Perimeter:** E-19 epic + S-19.01..S-19.07 + STORY-INDEX E-19 section
**Reviewer:** fresh-context adversary (zero prior context per Iron Law; rubric = policies.yaml v1.4.1; 20 policies)
**Date:** 2026-07-08
**Verdict:** NOT-CLEAN — BLOCKER 0 / HIGH 1 / MEDIUM 1 / LOW 0 (2 findings + 3 observations)
**Streak:** 0/3
**Model family:** Claude Opus 4.7

## Part A — Fix Verification (pass 15 → pass 16)

Pass-15 verdict NOT-CLEAN B0/H6/M1/L0 (7 findings: F-P15-001..F-P15-007). Fresh-context re-inspection of pass-16 artifacts:

- **F-P15-001 CLOSED.** S-19.06 v1.12 AC-007 Gate 2 clause (iii) replaced with awk block-containment form. Verified against `/Users/zious/Documents/GITHUB/vsdd-factory/crates/hook-sdk/src/ffi.rs`: the `awk '/^#\[cfg\(target_arch = "wasm32"\)\]/,/^}/'` range correctly captures the extern block; the `awk '/^#\[cfg\(not\(target_arch = "wasm32"\)\)\]/,/^}/'` range captures `pub mod host_stubs`. Both ranges currently `grep -q 'fn read_prefix'` → exit 1 (read_prefix absent), per D-766 §4 evidence. Fixed-fixture would exit 0.
- **F-P15-002 CLOSED.** S-19.06 v1.12 File Structure adds `crates/hook-sdk/src/ffi.rs` row; Task 11 corrected to split (host.rs safe wrapper + ffi.rs raw extern + host_stubs stub).
- **F-P15-003 CLOSED.** S-19.06 v1.12 Architecture Mapping hook-sdk section split correctly: host.rs receives `pub fn read_prefix(...) -> Result<Vec<u8>, HostError>` safe wrapper mirroring existing `pub fn read_file`; ffi.rs receives `pub safe fn read_prefix` 6-param extern mirroring `pub safe fn read_file`.
- **F-P15-004/005/006 CLOSED.** S-19.05 v1.13 AC-004 static legs and T-006 gate replaced with awk preceding-line form (`awk '/^#\[cfg\(/{c=1; next} /const ENV_SINK_FILE:/{exit c} {c=0}'`). Independently verified against current `crates/factory-dispatcher/src/main.rs`: `#[cfg(debug_assertions)]` immediately precedes `const ENV_SINK_FILE:`, `fn flush_sink_file(...)`, and `use std::sync::Mutex;` → each awk exits 1 (defect detected); fixed-fixture (cfg gate removed) exits 0.
- **F-P15-007 CLOSED.** S-19.03 v1.12 AC-006 gate replaced with `{ [ ! -s "$SINK_FILE" ] || jq -e 'true' < "$SINK_FILE" >/dev/null 2>&1; } && [ "$(jq -r 'select(...)' | wc -l)" -eq 0 ]` form. Traced across 4 scenarios (empty stream / matching event / malformed JSON / excluded reason) — behaves correctly. `pipefail`+`grep-c` no-match false-fail eliminated.

7/7 pass-15 findings closed at pass-16 perimeter entry.

## Part B — New Findings

**F-P16-001 — HIGH — STORY-INDEX v4.149 contains three stale BC version citations in live prose citations outside POLICY 5 v1.3.5 exempt site classes.**

Locus: `.factory/stories/STORY-INDEX.md` lines 685 and 701.

Defect statement: The E-19 delivery-summary paragraph (line 685) and BC coverage block (line 701) contain live-tense BC version citations that do not match current BC frontmatter versions. Specifically:

1. Line 685: `"(6) host::read_prefix bounded partial read new FFI entry point (S-19.06; BC-1.17.001 v1.1 LANDED; VP-101; depends_on S-19.03; W2)."` — current BC-1.17.001 is v1.2. Cite is stale.
2. Line 701: `"BC-4.13.001 v1.7 (S-19.02 Phase-A amendment + S-19.07 Phase-B migration)"` — current BC-4.13.001 is v1.8. Cite is stale.
3. Line 701: `"BC-1.17.001 v1.1 LANDED (S-19.06; read_prefix FFI)"` — current BC-1.17.001 is v1.2. Cite is stale.

Ground-truth verification: `grep -n "BC-4\.13\.001 v1\.[0-9]+\|BC-1\.17\.001 v1\.[0-9]+"` on STORY-INDEX.md returns lines 685 and 701 with the three stale cites. Cross-referenced against BC frontmatter: BC-4.13.001 v1.8; BC-1.17.001 v1.2.

Neither line is in a POLICY 5 v1.3.5 exempt site class (not frontmatter modified[], not Changelog row, not [Prior:] clause, not Adversary Pass Coverage, not lessons cross-ref).

Policy citations: POLICY 5 v1.3.5; POLICY 14 (upstream-index leg); POLICY 17.

Concrete failure scenario: An operator or new implementer reading STORY-INDEX line 685/701 for E-19 planning would conclude BC-1.17.001 is v1.1 and BC-4.13.001 is v1.7, missing the v1.2 layering parenthetical §(a) and the v1.8 Phase-A/Phase-B split with path_allow narrowed to `[".factory/STATE.md"]`. Real misdirection risk.

Class analysis: longstanding under-propagation of the pass-4 (BC-1.17.001 v1.0→v1.1), pass-10 (BC-4.13.001 v1.7→v1.8), and pass-12 (BC-1.17.001 v1.1→v1.2) BC bumps into STORY-INDEX prose; every intervening preflight scanned per-file only. Not fix-burst-introduced.

Fix: state-manager STORY-INDEX v4.149→v4.150 — `BC-1.17.001 v1.1` → `v1.2` (lines 685 + 701) and `BC-4.13.001 v1.7` → `v1.8` (line 701). Extend BC-cite preflight to STORY-INDEX prose blocks.

**F-P16-002 — MEDIUM — S-19.06 v1.12 `inputs:` frontmatter list omits `crates/hook-sdk/src/ffi.rs`, which the story now mandates modifications to per F-P15-002/003 sweep — partial-fix propagation gap.**

Locus: S-19.06 frontmatter `inputs:` field (4 entries; no ffi.rs).

Defect statement: pass-15 F-P15-002/003 added ffi.rs to File Structure + Task 11, but frontmatter `inputs:` was not updated. Sibling convention (S-19.02, S-19.03): modification targets + context files go in `inputs:`. Consequence: `input-hash: "03c6f12"` cannot detect ffi.rs drift between story authorship and implementation — precisely the gap input-hash exists to catch.

Policy citations: POLICY 18; POLICY 5 v1.3.6.

Fix: story-writer S-19.06 v1.12→v1.13 — add ffi.rs to inputs; refresh input-hash via compute-input-hash; STORY-INDEX Input-hashes line updated (POLICY 14 upstream-index leg).

## Observations

**O-P16-01 — [observation; accepted-with-record]** E-19 epic v1.13 frontmatter lacks `modified:` array and `last_amended:` field. POLICY 17 mandates 5-leg parity for epics; systematic across 12/20 epic files. Longstanding convention gap. Policy-level adjudication required: (a) exempt epics explicitly, or (b) require the two legs on future epics. Not a story-writer fix.

**O-P16-02 — [observation; drift-item]** E-19 epic v1.13 EAC-008 Validation Method and Test Scenarios columns contain identical text ("S-19.05 AC-002 test suite"). Degenerate but not incorrect; cosmetic.

**O-P16-03 — [observation; process-gap]** Pass-15 preflight §6 declared "BC-cite preflight PASS ... zero stale live citations" while STORY-INDEX lines 685/701 already carried three stale cites. The preflight scans per-file (per-story) but does not cover STORY-INDEX prose blocks (delivery-summary items, BC coverage blocks). Since STORY-INDEX is the POLICY 14 upstream-index leg, a stale prose citation there is a parity-leg gap. Recommend codification of "BC-cite preflight — STORY-INDEX prose leg" as mandatory.

## Verifications That PASSED

1. Spec version parity PASS (15 artifacts): S-19.01 v1.11 / S-19.02 v1.9 / S-19.03 v1.12 / S-19.04 v1.11 / S-19.05 v1.13 / S-19.06 v1.12 / S-19.07 v1.6 / epic v1.13 / STORY-INDEX v4.149. Story-level BC cites all current; zero stale live citations at story-level.
2. DAG bidirectional consistency PASS (epic mermaid ↔ per-story depends_on; acyclic; zero orphans).
3. Story count / point sum PASS (7 stories; 45 pts; matches epic + STORY-INDEX).
4. Subsystem union PASS (epic subsystems_affected = exact union of per-story subsystems).
5. Input-hashes distinct PASS (7 distinct hashes matching story frontmatter + STORY-INDEX).
6. F-P15-001..007 all CLOSED (ground-truth verified).
7. Gate-execution-evidence rule (D-766 §4) COMPLIANCE for all pass-15-swept gates (S-19.01 AC-004; S-19.03 AC-006 4-scenario matrix; S-19.05 AC-004 + T-006; S-19.06 AC-007 Gate 2 clause iii).
8. Gate independent verification: awk patterns re-derived against actual main.rs/ffi.rs source lines — each produces the claimed exit code.

## Summary

| Severity | Count |
|----------|-------|
| BLOCKER | 0 |
| HIGH | 1 |
| MEDIUM | 1 |
| LOW | 0 |
| Observations | 3 |

Actionable findings: 2. Trajectory 16→14→20→9→8→5→12→11→4→7→6→6→3→6→7→2. Pass-15 fix sweep closed 7/7 without introducing new gate defects. F-P16-001 longstanding (since pass-4/pass-10); F-P16-002 pass-15 same-burst sibling-miss.

**Overall Assessment:** block
**Convergence:** findings remain — iterate (strict 3-CLEAN per human directive D-761; no cap)
**Class analysis:** shift from gate-logic defects (passes 12-15) to propagation-leg gaps (pass 16); root cause: preflight/parity gates operate at per-file/per-cell granularity and miss cross-artifact prose-leg propagation.

## Novelty Assessment

| Field | Value |
|-------|-------|
| Pass | 16 |
| New findings | 2 (F-P16-001, F-P16-002) |
| Duplicate/variant findings | 0 |
| Novelty score | 1.0 (2 / 2) |
| Median severity | HIGH (1H + 1M) |
| Trajectory | 16 → 14 → 20 → 9 → 8 → 5 → 12 → 11 → 4 → 7 → 6 → 6 → 3 → 6 → 7 → 2 |
| Verdict | FINDINGS_REMAIN — pass-16 fix sweep required under strict-3-CLEAN (no cap per D-761) |

## Coverage Attestation

Artifacts read in full: E-19 epic v1.13; S-19.01 v1.11; S-19.02 v1.9; S-19.03 v1.12; S-19.04 v1.11; S-19.05 v1.13; S-19.06 v1.12; S-19.07 v1.6; STORY-INDEX v4.149 (E-19 section + header/BC-coverage/delivery-summary); adv-E19-pass-15.md Part A only.
Ground-truth source reads: crates/hook-sdk/src/ffi.rs (full); crates/hook-sdk/src/host.rs (grep: pub fn read_file); crates/factory-dispatcher/src/main.rs (grep: ENV_SINK_FILE, flush_sink_file, Mutex, Arc); .github/workflows/ci.yml (grep: bats jobs; bats-darwin-leg-macos absent).
Gates executed in-mind against current source: S-19.01 AC-004 anchored leg → exit 1; S-19.05 AC-004 ENV_SINK_FILE/flush_sink_file/T-006 Mutex awk legs → each exit 1; S-19.06 AC-007 Gate 2 clause (iii) wasm32 + host_stubs block scopes → each exit 1. All match D-766 §4 (a) claims.
Not read (Iron Law): Part B of adv-E19-pass-15.md; adv-E19-pass-1..14; decision-log; burst-log; lessons; STATE.md; session checkpoints; fix-burst records.
