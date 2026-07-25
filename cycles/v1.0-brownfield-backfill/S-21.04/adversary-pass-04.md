---
pass: 4
verdict: NOT-CLEAN
reviewed_head: b44442b2
novelty: 0.71
previous_review: "cycles/v1.0-brownfield-backfill/S-21.04/adversary-pass-03.md"
---

## Summary

Pass-4 adversarial review of S-21.04 implementation (step-g-cleanup.md, BC-6.26.001 v1.5, story v1.6, red-gate-log v1.2, test suite). 12 findings (B0 / H6 / M5 / L1). 6 HIGH findings represent genuine functional gaps: missing guard for non-path-absent find exits in the test suite, caller-side propagation not present in BC Invariant 6, awk field splitting unsafe for paths with spaces, missing test coverage for AC-007 caller-side propagation path, red-gate-log PC2c semantics cite a fabricated "PREFLIGHT BLOCKED" message not in §G.1, and conflicting precedence claims over Step 8 teardown gate. Streak: 0/3.

---

## Part A — Findings

### Finding Table

| ID | Severity | Category | Location | Summary |
|----|----------|----------|----------|---------|
| F-S2104-P4-001 | HIGH | Missing guard | test suite AC-006 | No test exercises non-path-absent find exit (e.g., permission denial) — PC2c path is untested |
| F-S2104-P4-002 | HIGH | Spec gap | BC-6.26.001 Invariant 6 | Invariant 6 states "MUST fail-closed" but does not specify caller-side propagation; AC-007 references propagation but Invariant 6 is the normative source — gap between AC and Invariant |
| F-S2104-P4-003 | HIGH | Space-unsafe | bin/ scripts referencing worktree paths | `awk '{print $2}'` field splitting is unsafe for paths containing spaces; worktree paths from `git worktree list` can contain spaces |
| F-S2104-P4-004 | HIGH | Missing test | AC-007 | No test validates caller-side propagation behavior; AC-007 is accepted but untested |
| F-S2104-P4-005 | HIGH | Semantic error | red-gate-log.md line 149 | PC2c entry cites "PREFLIGHT BLOCKED" as the required message; §G.1 PC2c in step-g-cleanup.md does not use that phrase — fabricated message in log |
| F-S2104-P4-006 | HIGH | Precedence conflict | step-g-cleanup.md vs BC-6.26.001 | §G.1 Step 8 teardown gate: step-g-cleanup.md says "MUST HALT if find exits non-zero for non-path-absent reason" but BC-6.26.001 PC2c clause and §G.1 have conflicting precedence claims — two mutually exclusive normative sources for the same behavior |
| F-S2104-P4-007 | MEDIUM | Version cite drift | BC-INDEX.md S-21.04 row | BC-6.26.001 version cited as v1.5; story v1.6 advanced BC to v1.5 but BC was already at v1.5 from pass-2 work — version cite is accurate but the BC body has not yet incorporated all pass-3 fixes |
| F-S2104-P4-008 | MEDIUM | Missing coverage | test suite | `git worktree list --porcelain` parsing not tested for the case where the worktree line contains a bare path with embedded spaces |
| F-S2104-P4-009 | MEDIUM | Incomplete AC | AC-006 test | AC-006 test checks path-absent case only; does not check that find exits 0 for a valid worktree (baseline positive case missing) |
| F-S2104-P4-010 | MEDIUM | Stale trace | red-gate-log.md frontmatter | `traces_to: v1.5` — should trace to BC v1.5 per most recent BC version at time of log creation; log was created before BC advanced to v1.5 from pass-2 |
| F-S2104-P4-011 | MEDIUM | Observation promoted | implementation/step-g-cleanup.md | §H.1 operator surface clause uses "SHOULD" rather than "MUST" for surfacing find exit code and stderr to operator — weakened requirement inconsistent with fail-closed mandate |
| F-S2104-P4-012 | LOW | Label imprecision | STORY-INDEX.md S-21.04 row | "AC-007 fail-closed" label in story row does not distinguish AC-007's caller-side propagation from AC-006's find-exit halt — semantically ambiguous for downstream readers |

---

## Part B — New Findings

Findings F-S2104-P4-001 through F-S2104-P4-012 are all new relative to pass-3. No pass-3 finding is re-raised. Novelty score 0.71 reflects that 6 of 12 findings are structural gaps not surfaced in prior passes.

Key novel findings:
- **F-001**: PC2c non-path-absent find exit path is completely untested. Prior passes addressed the spec language; the test gap remained.
- **F-002**: BC Invariant 6 / AC-007 normative gap — Invariant 6 was strengthened for fail-closed semantics in pass-2 but caller-side propagation was only codified in AC-007, not elevated to the Invariant.
- **F-003**: Space-unsafe awk in bin/ — not story-specific but surfaced during path handling review; affects any worktree with spaces in path.
- **F-005/F-006**: red-gate-log fabricated message + precedence conflict — both emerged from cross-reading step-g-cleanup.md §G.1 against the log entry verbatim.

---

## Novelty Assessment

Novelty: 0.71. 6 HIGH findings (F-001 through F-006) represent genuine new structural gaps. 5 MEDIUM findings (F-007 through F-011) are version-cite drift, coverage gaps, and weak-modal issues. 1 LOW finding (F-012) is a label precision issue addressed immediately as a fix-burst candidate. No finding repeats a prior-pass finding verbatim. The F-005/F-006 PC2c precedence cluster is the highest-novelty discovery: two authoritative documents assert conflicting normative ownership of the same behavioral gate.

---

## Fix Mapping

| Finding ID | Fix leg | Commit SHA | Disposition |
|------------|---------|------------|-------------|
| F-S2104-P4-005 | red-gate-log PC2c semantics fix | fcfce450 | FIXED — line 149 now cites verbatim §G.1 PC2c semantics; "PREFLIGHT BLOCKED" removed |
| F-S2104-P4-012 | STORY-INDEX label fix | a5192209 (D-898 burst; STORY-INDEX v4.251→this pass) | FIXED — "AC-007 fail-closed" → "AC-007 caller-side propagation" |
| F-S2104-P4-007 | BC-INDEX version cite | fcfce450 | FIXED — BC-6.26.001 row updated v1.5→v1.6 in BC-INDEX.md v4.28 |
| F-S2104-P4-001 | Deferred — test suite gap | — | OPEN — no test for non-path-absent find exit; deferred to pass-5 or follow-up story anchor |
| F-S2104-P4-002 | Deferred — BC Invariant 6 | — | OPEN — Invariant 6 caller-side propagation gap; deferred to product-owner for BC amendment |
| F-S2104-P4-003 | Deferred — space-unsafe awk | — | OPEN PENDING — human story-anchor decision required before bin/ sweep |
| F-S2104-P4-004 | Deferred — AC-007 test | — | OPEN — no test for caller-side propagation; follows from F-002 BC gap |
| F-S2104-P4-006 | OPEN — precedence conflict | — | OPEN — two mutually exclusive normative sources; requires human adjudication |
| F-S2104-P4-008 | Deferred | — | OPEN — spaces-in-path test gap |
| F-S2104-P4-009 | Deferred | — | OPEN — AC-006 baseline positive case missing |
| F-S2104-P4-010 | FIXED in D-899 burst | fcfce450 | FIXED — red-gate-log frontmatter traces_to updated v1.5→v1.6 |
| F-S2104-P4-011 | Deferred | — | OPEN — §H.1 "SHOULD" vs "MUST" for operator surface |
