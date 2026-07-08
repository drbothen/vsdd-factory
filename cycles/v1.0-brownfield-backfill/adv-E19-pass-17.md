# Adversarial Review — E-19 Pass 17 (NOT-CLEAN)

**Perimeter:** E-19 epic + S-19.01..S-19.07 + STORY-INDEX E-19 section
**Reviewer:** fresh-context adversary (zero prior context per Iron Law; rubric = policies.yaml v1.4.1; 20 policies)
**Date:** 2026-07-08
**Verdict:** NOT-CLEAN — BLOCKER 0 / HIGH 0 / MEDIUM 2 / LOW 0 (2 findings + 2 observations)
**Streak:** 0/3 (unchanged; pass-16 remedies verified, but 2 pre-existing defects newly surfaced under fresh re-derivation)
**Model family:** Claude Opus 4.7

## Part A — Fix Verification (pass 16 → pass 17)

Pass-16 verdict NOT-CLEAN B0/H1/M1/L0 (F-P16-001 STORY-INDEX stale BC cites; F-P16-002 S-19.06 inputs missing ffi.rs). Fresh-context re-inspection of pass-17 artifacts:

- **F-P16-001 CLOSED.** STORY-INDEX v4.149→v4.150. Line 685 (delivery summary): "(6) host::read_prefix bounded partial read new FFI entry point (S-19.06; BC-1.17.001 v1.2 LANDED; VP-101; depends_on S-19.03; W2)." — v1.2 (current). Line 701 (BC coverage block): "BC-4.13.001 v1.8 (S-19.02 Phase-A amendment + S-19.07 Phase-B migration); ... BC-1.17.001 v1.2 LANDED (S-19.06; read_prefix FFI)" — both v1.8 and v1.2 (current). Zero stale BC cites in live prose at STORY-INDEX head.
- **F-P16-002 CLOSED.** S-19.06 v1.13 frontmatter inputs: now enumerates 5 entries including crates/hook-sdk/src/ffi.rs; input-hash: "617adeb" matches STORY-INDEX line 700 (S-19.06=617adeb).

2/2 pass-16 findings closed at pass-17 perimeter entry.

## Part B — New Findings

### F-P17-001 — MEDIUM — S-19.07 Test Plan T-005 description contradicts EC-005 Expected Behavior — story-internal semantic contradiction (log_warn vs no-bespoke-warn)

Locus: .factory/stories/S-19.07-verify-factory-lock-read-prefix-migration.md — Test Plan T-005 row (line 200): "Missing capabilities.read_prefix in registry → graceful degrade to Continue + log_warn emitted"; EC-005 Expected Behavior (line 118): "...operator visibility preserved via standard internal.capability_denied event logged on every denied call (no bespoke one-shot warn needed; visibility parity with Phase-A's soft-warn is provided by the denial event class)"; Task 5 (line 150) neutrally says "graceful degrade to Continue" — consistent with EC-005.

Defect: T-005 asserts a log_warn emission that EC-005 explicitly excludes (v1.6 O-P14-05 amendment). Architecture Mapping line 92 removes the only plugin warn source (state_md_approaching_cap soft-warn per BC-4.13.001 v1.8 Invariant 10 removal), so no plugin-emitted warn path remains under Phase-B — T-005's log_warn assertion is structurally unsatisfiable.

Concrete failure scenario: test-writer authors T-005 → asserts log_warn in captured plugin output → implementer removes soft-warn per Architecture Mapping AND EC-005 → T-005 fails against production-grade Phase-B code. Story self-blocking against its own EC-005 semantics.

Policy citations: POLICY 8 (test-plan row must mirror Edge Case Expected Behavior); POLICY 4 (semantic anchoring).

Fix: story-writer S-19.07 v1.6→v1.7 — T-005 → "graceful degrade to Continue + internal.capability_denied event class present in dispatcher log (no bespoke log_warn — visibility parity via denial event per EC-005)".

Class analysis: partial-fix propagation gap from v1.6 (O-P14-05); EC-005 cell updated, T-005 same-file sibling missed. TD-VSDD-060 sibling-site sweep miss. Confidence: HIGH.

### F-P17-002 — MEDIUM — S-19.07 Previous Story Intel S-19.06 row has stale path_allow = [".factory"] — F-P10-001 v1.5 sibling-sweep miss (5th site left unswept)

Locus: S-19.07 line 161 (Previous Story Intel table, S-19.06 row, Patterns Established column): `read_prefix(path, max_bytes=8192, timeout_ms)` call signature; `capabilities.read_prefix` capability block with `path_allow = [".factory"]`.

Defect: v1.5 amendment note states F-P10-001 swept path_allow ['.factory']→['.factory/STATE.md'] across "4 sites: AC-002 gate, Architecture Mapping, Task 8, File Structure". This Previous Story Intel row is an unswept 5th site. AC-002 (line 83), Architecture Mapping (line 98), Task 8 (line 153), File Structure (line 207) all show [".factory/STATE.md"]; this row shows [".factory"]. Additional accuracy issue: the cell mis-attributes to S-19.06 the establishment of a specific path_allow value — S-19.06 documents the capability schema in a preamble comment block but declares NO live path_allow (per S-19.06 Architecture Mapping); the value is set by S-19.07 itself. Even corrected, the sentence is semantically off (POLICY 4).

Policy citations: POLICY 5 v1.3.4 (sibling-sweep); POLICY 4 (semantic anchoring); TD-VSDD-060.

Fix: story-writer S-19.07 v1.6→v1.7 — reword cell: schema documented in preamble; consuming plugins set their own path_allow; this story's verify-factory-lock entry uses [".factory/STATE.md"] per AC-002. Update/acknowledge the v1.5 "4 sites" undercount.

Concrete failure scenario: implementer inherits path_allow = [".factory"] from the Intel row → hooks-registry gets broader allowlist than BC-4.13.001 v1.8 Precondition 3 narrowing intends → security posture regressed.

Class analysis: narrative-table row escape from sibling-sweep at v1.5 fix burst; same defect pattern as F-P16-001. Confidence: HIGH.

## Observations

- **O-P17-01 — [observation; drift-item]** STORY-INDEX line 729 E-19 authorship chronology footnote accretes per-pass fix-burst brackets (pass-1, pass-2, pass-12, pass-13); accretion halted at pass-13; passes 14/15/16 (STORY-INDEX v4.146→v4.150) absent. Within the POLICY 5 v1.3.5 pass-history exempt class for staleness — not a live-cite defect — but the accretion convention quietly lapsed. Recommend explicit closure ("chronology retired at v4.146; further versions tracked via version history only") or resumption.

- **O-P17-02 — [observation; process-gap]** Second consecutive pass surfacing a "narrative-table row escape from a claimed N-site sibling-sweep" defect (pass-16 F-P16-001 escaped per-file BC-cite preflight; pass-17 F-P17-002 escaped a v1.5 explicit "4 sites" enumeration). Recommend codification: when a fix burst declares "swept N sites" for a structural value change, mechanically enumerate ALL occurrences of the outgoing value (whole-file grep count) and require N to equal grep-count minus the intentional keep-set.

## Verifications That PASSED (independently re-derived)

1. Spec version parity PASS (15 artifacts): S-19.01 v1.11 / S-19.02 v1.9 / S-19.03 v1.12 / S-19.04 v1.11 / S-19.05 v1.13 / S-19.06 v1.13 / S-19.07 v1.6 / epic v1.14 / STORY-INDEX v4.150. All live BC cites at story frontmatter + body BC tables current.
2. DAG bidirectional consistency PASS (acyclic; W1→W2→W3 topological order).
3. Story count / point sum PASS (7 stories; 45 pts).
4. Subsystem union PASS.
5. Input-hashes distinct PASS: 7 distinct; S-19.06=617adeb matches STORY-INDEX line 700.
6. F-P16-001 + F-P16-002 CLOSED (ground-truth verified at HEAD).
7. Gate-execution-evidence (D-766 §4) compliance: S-19.06 AC-007 Gate 2 clause (iii) re-derived at HEAD — ffi.rs contains only read_file; both awk-range greps for fn read_prefix exit 1. ✓
8. S-19.05 AC-004 static-leg awk gates re-derived: cfg(debug_assertions) immediately precedes ENV_SINK_FILE const (70/71), flush_sink_file (821/822), use std::sync::Mutex (36/37) — all exit 1 at current state. ✓
9. S-19.01 AC-004 anchored YAML-key grep: no match in ci.yml → exit 1 (pre-implementation; correct). ✓
10. Frontmatter-vs-body BC parity PASS across all 7 stories.
11. Epic v1.14 EAC-008 column-split verified distinct.

## Summary

| Severity | Count |
|----------|-------|
| BLOCKER | 0 |
| HIGH | 0 |
| MEDIUM | 2 |
| LOW | 0 |
| Observations | 2 |

Actionable findings: 2. Trajectory 16→14→20→9→8→5→12→11→4→7→6→6→3→6→7→2→2. Pass-16 fix sweep closed 2/2 without introducing new defects. Both pass-17 findings are longstanding sibling-sweep escapes (F-P17-001 from v1.6; F-P17-002 from v1.5) — same class as pass-16 F-P16-001.

**Overall Assessment:** block
**Convergence:** findings remain — iterate (strict 3-CLEAN per D-761; no cap)
**Class analysis:** cross-pass persistence of "narrative-table row escape from claimed sibling-sweep" class. Recommend O-P17-02 codification.

## Novelty Assessment

| Field | Value |
|-------|-------|
| Pass | 17 |
| New findings | 2 (F-P17-001; F-P17-002) |
| Duplicate/variant findings | 0 |
| Novelty score | 1.0 (2 / 2) |
| Median severity | MEDIUM (2M; no H) |
| Trajectory | 16 → 14 → 20 → 9 → 8 → 5 → 12 → 11 → 4 → 7 → 6 → 6 → 3 → 6 → 7 → 2 → 2 |
| Verdict | FINDINGS_REMAIN — pass-17 fix sweep required under strict-3-CLEAN (no cap per D-761) |

## Coverage Attestation

Artifacts read in full: E-19 epic v1.14; S-19.01 v1.11; S-19.02 v1.9; S-19.03 v1.12; S-19.04 v1.11; S-19.05 v1.13; S-19.06 v1.13; S-19.07 v1.6; STORY-INDEX v4.150 (E-19 section incl. delivery summary, BC coverage, footnote chronology line 729); adv-E19-pass-16.md Part A only.
Ground-truth source reads: crates/hook-sdk/src/ffi.rs (read_prefix absent both blocks); crates/factory-dispatcher/src/main.rs (three cfg-gated sites confirmed); .github/workflows/ci.yml (bats-darwin-leg-macos absent); .factory/specs/behavioral-contracts/ss-04/BC-4.13.001.md (v1.8; path_allow reconciled to .factory/STATE.md).
Gates executed in-mind at HEAD: S-19.06 AC-007 Gate 2 clause (iii) both blocks → exit 1 ✓; S-19.05 AC-004 three awk legs → exit 1 each ✓; S-19.01 AC-004 anchored grep → exit 1 ✓.
Not read (Iron Law): Part B of adv-E19-pass-16.md; adv-E19-pass-1..15; decision-log.md; burst-log.md; lessons.md; STATE.md; session checkpoints; fix-burst records.
