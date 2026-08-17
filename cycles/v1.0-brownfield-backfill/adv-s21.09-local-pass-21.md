---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: vsdd-factory:adversary
timestamp: 2026-08-13T08:00:00Z
phase: 21
inputs: [.factory/stories/S-21.09-wasm-artifact-restore-and-registry-parity.md]
input-hash: "48547f6"
traces_to: S-21.09-wasm-artifact-restore-and-registry-parity.md
pass: 21
previous_review: adv-s21.09-local-pass-20.md
---

# Adversarial Review — S-21.09 (LOCAL cascade pass, strengthened rubric), worktree .worktrees/S-21.09 @ fc0e613b

Verified facts: WASM validate-factory-path-staging.wasm exists on disk in the worktree; registry entry present (hooks-registry.toml 944-947); T-012 (run_t012_gate(&workspace_root())) is the class-wise proof it is git-tracked. Count parity: 75 [[hooks]] entries and 75 plugin = "hook-plugins/ lines (spec's "75 [[hooks]]" pin accurate); 51 tests (T-006..T-056), 45 S-21.09-owned (T-012..T-056), +1 registry unit test. Every #[should_panic(expected=...)] substring matches production byte-for-byte and is tight (T-019/020/022/027/041/049/052/053/054). T-048 tautology genuinely removed (single exact-vec-equality). T-050/T-051 conjunct-isolation controls real and orthogonal. T-055 (fail-open) and T-056 (lex_norm CurDir) both pair the isolating assertion with a positive control; Path::new("./a/b").components() yields a leading CurDir so T-056 is non-vacuous. Every indented production format string is locked by ≥1 indented assertion: "  UNEXPECTED:"←T-017/T-024/T-030A; "  MISSING:" (inventory)←T-018; "  MISSING:" (subset)←T-015; "  STAGED-NOT-COMMITTED:"←T-021; "  {ungated}"←T-038/T-050/T-051.

## Critical Findings (BLOCKER)
None.
## High Findings
None.
## Medium Findings
None. (The strengthened FORMAT-LOCK FIDELITY axis is satisfied for the audited emitters: every two-space-indented production emitter is locked by at least one assertion carrying the exact two-space needle. No indent-dropping mutant on the named emitters survives via the locked tests.)

## Low Findings
### F-S2109-P21-001 [LOW] — POLICY 15 / Partial-Fix Regression Discipline: pass-20 format-lock sibling-sweep is incomplete
Location: test_S_21_09_ac006_T030_wiring_control_both_check_calls_are_active (phase B) asserts contains("MISSING: hook-plugins/ctx.wasm") (unindented); test_S_21_09_ac006_T037_staged_not_committed_fires_staged_not_committed asserts contains("STAGED-NOT-COMMITTED: hook-plugins/staged-probe.wasm") (unindented); test_S_21_09_ac006_T039_subdir_declared_vs_flat_committed_fires_missing asserts contains("MISSING: hook-plugins/sub/h00.wasm") (unindented); test_S_21_09_ac006_T040_ungated_declaration_in_resolvers_fires asserts contains("UNGATED-DECLARATION: other-dir/evil-resolver.wasm") (unindented).
Evidence/rationale: The pass-20 format-lock burst (fc0e613b) explicitly tightened positive-identifier .contains() needles to the two-space-indented production form across T-015/T-017/T-018/T-024/T-030(phase A)/T-038/T-043-T-047/T-050/T-051, on the stated principle that "an unindented needle would also match inside the indented production line via .contains() and could never catch an indent-dropping mutation." That same principle applies verbatim to the four assertions above — unindented needles against the identical indented production format strings — yet the sweep did not touch them. T-038's own comment names this a "sibling gap."
Impact (honest): NIL mutation-detection impact. Each affected format is already redundantly locked by a swept sibling — "  MISSING:" by T-015, "  STAGED-NOT-COMMITTED:" by T-021, "  UNGATED-DECLARATION:"/"  {ungated}" by T-038. An indent-drop mutation on any of these production format! strings is caught by the sibling, so no mutant escapes. These four needles remain non-vacuous for their own primary purpose (distinguishing the correct outcome/path from Ok or a wrong path); they simply do not additionally contribute to format-locking. Incomplete hardening sweep / consistency gap, not a coverage hole.
Intent adjudication: cannot adjudicate whether these four were deliberately left as redundant or missed. Surfacing per "do NOT silently skip differences that might be intentional." Minimal fix route: either tighten the four needles to the two-space form for uniformity, OR add a one-line note on each that the format is locked by its named sibling (T-015/T-021/T-038).

## Observations (non-blocking)
- VP gap (documented, human-acknowledged): verification_properties: [] in story frontmatter, all four BC-4.16.001 §VP rows "(TBD — to be assigned by state-manager after VP authoring pass)". No VP IDs invented, no POLICY 4/9 mis-anchor. Owed-VP allocation routed in story §BC Status.
- Volatile count evidence (TD-VSDD-091 borderline): §AC-006 narrative pins "75 [[hooks]] + 1 [[resolvers]] = 76 entries, 36 unique names" as verification evidence for the hooks floor of 30. Confirmed 75 [[hooks]] currently — accurate today but a decaying count; framed as point-in-time verification evidence, not a load-bearing anchor, so at most informational.
- BC frontmatter status: draft vs lifecycle_status: active (POL-14 auto-promotion at S-21.01 merge 7bb0e797); story §BC Status correctly flags as stale metadata not governing enforcement; pre-existing BC-file condition outside S-21.09's diff.

## Novelty Assessment
Implementation exceptionally mature (pass-20+). Assertion quality high: no vacuous or tautological assertions found, should_panic substrings tight, every indented production emitter format-locked by a dedicated control. The single LOW finding is a consistency/sibling-sweep-completeness item with nil coverage impact.

## Verdict
NOT-CLEAN by the letter of BC-5.39.001 (one LOW finding keeps the streak from advancing). The finding is a documented-hardening-sweep consistency gap, not a correctness/coverage defect. Reporting the difference rather than suppressing it per the intent-adjudication rule.
