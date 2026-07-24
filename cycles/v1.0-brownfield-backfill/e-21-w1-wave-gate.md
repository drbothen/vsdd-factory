---
document_type: wave-gate-report
wave: E-21-W1
producer: state-manager
decision: D-891
date: 2026-07-24
scope: W1 (S-21.01 + S-21.02 + S-21.03) — full wave closure gate
stories:
  - {id: S-21.01, pr: 759, sha: "7bb0e797", wave: W1, merged: "2026-07-23"}
  - {id: S-21.02, pr: 760, sha: "a4a79f09", wave: W1, merged: "2026-07-24"}
  - {id: S-21.03, pr: 761, sha: "ebf9fb6d", wave: W1, merged: "2026-07-24"}
fix_pr: {pr: 763, sha: "948f0fb1", merged: "2026-07-24"}
develop_base: ebf9fb6d
develop_head: 948f0fb1
gate_verdict: PASSED
next_wave: W2 (S-21.04, S-21.05) DISPATCHABLE
---

# E-21 W1 Wave-Gate Report

**Decision:** D-891
**Date:** 2026-07-24
**Wave:** E-21 W1
**Stories:** 3 W1 stories — S-21.01 (PR #759 7bb0e797 2026-07-23), S-21.02 (PR #760 a4a79f09 2026-07-24), S-21.03 (PR #761 ebf9fb6d 2026-07-24)
**Consolidated Fix PR:** #763 squash-merged 948f0fb1 2026-07-24 (human-executed)
**Develop base → HEAD:** ebf9fb6d → 948f0fb1

## Scope Note

W1 contains 3 stories: S-21.01 (validate-factory-path-staging WASM guard + orchestrator merge pre-check; PR #759 7bb0e797 merged 2026-07-23), S-21.02 (post-rebase diff-integrity gate; PR #760 a4a79f09 merged 2026-07-24), S-21.03 (pr-manager trunk assertions + post-merge ancestry check; PR #761 ebf9fb6d merged 2026-07-24). Gate covers all 3 W1 stories. W2 (S-21.04, S-21.05) remains draft and will be dispatched separately per D-862 SEQUENTIAL.

## Gate Results Summary

| Gate | Name | Status |
|------|------|--------|
| 1 | Full Test Suite | PASS-AFTER-REMEDIATION |
| 2 | DTU Validation | SKIP |
| 3 | Wave Adversarial Review | CLEAN (N2/OBS2) |
| 4 | Code Review | APPROVE-WITH-NOTES |
| 5 | Security Review | PASS (W-SEC-001 MEDIUM deferred→S-21.06; 3 LOW) |
| 6 | Consistency | CONDITIONAL PASS → FULL PASS |
| 7 | Holdout Evaluation | N/A-WITH-JUSTIFICATION |

**GATE VERDICT: PASSED — W2 (S-21.04, S-21.05) DISPATCHABLE**

## Gate 1: Full Test Suite

**Status:** PASS-AFTER-REMEDIATION

**Initial run result (pre-D-890):** RED — 5 failures:
- `sprint-state-format.bats` T-1, T-7, T-12 FAIL: S-21.01/S-21.02/S-21.03 had `status: completed` in sprint-state.yaml `stories:` list; BC-5.41.004 INV-1 canonical 8-value enum does not include `completed`; EC-007 test vector `completed-is-unknown-hard-abort` rejects it.
- `cargo test -p validate-state-structure` ×2 FAIL: STATE.md banner `wc -l` value stale (433 vs actual 351) since D-862 compaction.

**Root cause:** deliver-story skill Step 9 says "story status → completed"; three W1 close bursts (D-884, D-887, D-889) followed this wording faithfully. Status `completed` is not in the canonical 8-value enum.

**Remediation:** D-890 (factory-artifacts `3fa124a9` 2026-07-24): (1) S-21.01/S-21.02/S-21.03 `status: completed` → `merged` in sprint-state.yaml `stories:` Partition A; (2) S-7.12 added to sprint-state.yaml Partition B (T-12 completeness gap); (3) STORY-INDEX S-21.01/S-21.02/S-21.03 status cells `completed`→`merged`; (4) STATE.md banner updated to 351 lines.

**Post-fix verification:**
- `bats plugins/vsdd-factory/tests/sprint-state-format.bats` → 14/14 PASS (T-1/T-7/T-12 now PASS)
- `cargo test -p validate-state-structure` → 65/65 PASS (0 failures)
- S-21.01 test suite: 36/36 PASS
- S-21.02 test suite: 6/6 PASS (range-diff gate + bats)
- S-21.03 test suite: 7/7 PASS
- `cargo fmt --check --all` → CLEAN
- `cargo clippy --workspace --all-targets -- -D warnings` → CLEAN
- CI 13/13 green at develop `948f0fb1` (full matrix: cargo-host ×2, build-dispatcher ×5, bats-full-suite, bats-darwin-leg, bats-wave-handoff, platforms-drift, validate, SAST)
- Residual `mktemp` failures in local env: pre-existing issue unrelated to W1 stories; does not appear in CI.

**Lesson codified:** L-BB-deliver-story-step9-status-wording-conflicts-with-BC-5.41.004 (D-890).

## Gate 2: DTU Validation

**Status:** SKIP

DTU assessment 2026-04-25: `dtu_required: false`. No module-criticality registry. No DTU clones. Precedented across all E-18/E-19/E-21 waves. No action required.

## Gate 3: Wave Adversarial Review

**Status:** CLEAN (N2/OBS2)

**Adversary:** `wg-adversary` (fresh-context; reads only current deliver state on develop `948f0fb1`)
**Findings:** 0 BLOCKER / 0 HIGH / 0 MEDIUM / 0 LOW new architectural findings
**Observations (N2):**
- OBS-1: hooks-registry.toml env_allow comment could be more precise about which env vars are observed vs passed through
- OBS-2: env_allow rationale would benefit from an explicit ADR-031 cross-reference

Both observations fixed in PR #763 as F-WG1-001/002 (hooks-registry.toml comment corrections).

**Defense-chain composition verified:** validate-factory-path-staging (PreToolUse; gate: repo state before file write) → post-rebase-diff-integrity-gate (devops skill; gate: rebase integrity before PR creation) → pr-manager trunk assertion + post-merge ancestry check (pr-manager; gate: develop ancestry after merge). No inter-gate gaps found. All three gates address distinct INV-E21 invariants with no overlap or dead zone.

**Streak:** N/A — wave-gate adversary pass (not per-story LOCAL cascade). BC-5.39.001 3-CLEAN protocol applies to per-story cascades only.

## Gate 4: Code Review

**Status:** APPROVE-WITH-NOTES

**Reviewer:** `wg-codereview`
**Round 1:** REQUEST_CHANGES (4 MINORs + 8 nitpicks identified)
**Round 2:** APPROVE (all MINORs and selected nitpicks addressed in PR #763; remaining nitpicks accepted-with-record)

**Evidence:** `.factory/code-delivery/fix-e21-w1-gate/pr-review.md` (round-1 REQUEST_CHANGES), `.factory/code-delivery/fix-e21-w1-gate/pr-review-round-2.md` (round-2 submission), `.factory/code-delivery/fix-e21-w1-gate/pr-review-round-2-final.md` (APPROVE verdict)

**Note on GitHub self-review restriction:** Agents push under the human's GitHub account; GitHub prohibits PR authors from approving their own PRs. Formal `gh pr review --approve` verdict was therefore GitHub-rejected on the agent-authored PR. Review gate satisfied-by-file-record per L-BB-github-self-review-restriction-on-agent-authored-prs [process-gap] (codified D-891); constraint disclosed at human merge gate; human merged with full awareness of two-party-review limitation. S-7.13 scope note applies.

### Findings Table

| ID | Severity | Description | Disposition |
|----|----------|-------------|-------------|
| MINOR-1 | MINOR | `skip_global_options` dedup refactor — extract helper to avoid duplicate ~45-line block | Fixed in PR #763 (behavior-identical; 138/138 tests incl. proptest) |
| MINOR-2 | MINOR | `_run_dispatcher` temp-file envelope — kills quote-escaping fragility in bats test helper | Fixed in PR #763 |
| MINOR-3 | MINOR | T-005b EC-005 stat-only fallback isolation test — missing per-mutant verification per POLICY 15 v1.4.10 | Fixed in PR #763 (per-mutant evidence added) |
| MINOR-4 | MINOR | git-checkout pre-check remote-ref guidance missing from per-story-delivery.md | Fixed in PR #763 |
| NITPICK-1 | NITPICK | log-level consts style improvement | Fixed in PR #763 |
| NITPICK-2 | NITPICK | accepted-with-record | accepted-with-record |
| NITPICK-3 | NITPICK | accepted-with-record | accepted-with-record |
| NITPICK-4 | NITPICK | `_RUN_DISPATCHER_STDERR` contract rename | Fixed in PR #763 |
| NITPICK-5 | NITPICK | accepted-with-record | accepted-with-record |
| NITPICK-6 | NITPICK | accepted-with-record | accepted-with-record |
| NITPICK-7 | NITPICK | doc rationale improvement | Fixed in PR #763 |
| NITPICK-8 | NITPICK | cross-ref addition | Fixed in PR #763 |

## Gate 5: Security Review

**Status:** PASS (W-SEC-001 MEDIUM deferred by human ruling → S-21.06 registered; 3 LOW)

**Reviewer:** `wg-security`

### Security Findings

| ID | Severity | CWE | Description | Disposition |
|----|----------|-----|-------------|-------------|
| W-SEC-001 | MEDIUM | CWE-693 (Protection Mechanism Failure) | Layer-2 main-checkout sync-protocol enforced via prose-only skill doc (orchestrator/per-story-delivery.md §Main-Checkout Sync Protocol); no PreToolUse WASM guard provides technical enforcement | **HUMAN RULING (AskUserQuestion 2026-07-24):** ADR-031 §Decision 2 accepted trade-off — prose-only Layer-2 is the current explicit ADR position. S-21.06 registered (Layer-2 sync-protocol WASM guard; W-SEC-001; `faece5a8`). Deferred to S-21.06 per human directive. |
| W-SEC-002 | LOW | — | Story-worktree path-traversal defense — defense-in-depth observation; gap exists but is bounded by existing guards | Accepted-with-record. No gate blocker. Anchor: S-21.04 implementation scope. |
| W-SEC-003 | LOW | CWE-116 (Improper Encoding) | pr-manager.md Step 5c: `gh pr comment --body '...'` — shell word-splitting vulnerability if comment body contains special chars | Fixed in PR #763: `--body-file "$TMPFILE"` (mktemp pattern) |
| W-SEC-004 | LOW | — | Timing observation in staging check: TOCTOU race window between path validation and use in validate-factory-path-staging | Accepted-with-record. Pre-existing local-env TOCTOU class per [D-826 W1-tracked]. Security reviewer classified LOW; not a gate blocker. |

**Human ruling provenance:** Direct AskUserQuestion response 2026-07-24 — human stated: "register a story for the Layer-2 WASM guard".

## Gate 6: Consistency

**Status:** CONDITIONAL PASS → FULL PASS (post-fix)

**Reviewer:** `wg-consistency`

**Initial finding F-WG21-001:** BC-INDEX.md BC-6.10.002 row lifecycle cell showed stale value not reflecting POL-14 promotion that occurred at S-21.03 merge (ebf9fb6d). Cell value inconsistent with actual `lifecycle_status: active` in the BC file.

**Fix:** `87ff340f` — BC-INDEX v4.24→v4.25; BC-6.10.002 lifecycle cell corrected. Applied prior to PR #763 merge.

**Post-fix consistency gate result:** FULL PASS. All cross-document checks:
- BC-INDEX total_bcs count matches corpus (no new BCs this W1 gate close; BC-5.41.004 wording sweep in PR #763 is behavioral-contracts-neutral)
- ID cross-references consistent (BC-6.10.002, BC-5.44.001, BC-4.16.001, BC-5.43.001 lifecycle_status all `active` post POL-14)
- Story status cells in STORY-INDEX agree with sprint-state.yaml (post D-890 fix)
- Version cite chains: BC-6.10.002 v1.5 in S-21.03 table row matches BC file

## Gate 7: Holdout Evaluation

**Status:** N/A-WITH-JUSTIFICATION

No E-21 holdout scenarios have been authored. `.factory/holdout-evaluations/` directory is empty. This is not a process gap:

1. **Self-referential project:** vsdd-factory IS the product being onboarded. Holdout scenarios for self-referential engine behaviors would require adversarial scenarios that test the factory against itself — a category not yet established.
2. **Adversarial precedent:** Wave-gate adversarial review (Gate 3 above) provides the adversarial coverage analog to holdout evaluation for this project. The adversary examined actual defense-chain composition and implementation correctness under fresh context.
3. **Consistent precedent:** All E-18/E-19/E-21 waves have N/A holdout with the same justification. D-854 wave-gate human disposition established this as accepted pattern.

`wg-adversary` confirmed Gate 3 is the adversarial coverage substitute for Gate 7 in this project context.

## D-890 Bookkeeping Fixes

Bookkeeping fixes applied before gate re-run (factory-artifacts `3fa124a9`; D-890 2026-07-24):

1. `sprint-state.yaml`: S-21.01/S-21.02/S-21.03 `status: completed` → `merged` in Partition A; S-7.12 added to Partition B
2. `STORY-INDEX.md`: S-21.01/S-21.02/S-21.03 status cells `completed` → `merged`; STORY-INDEX v4.245→v4.246
3. `STATE.md`: banner `wc -l` updated 433→351; STATE.md v6.26→v6.27

Process-gap lesson codified (D-890): L-BB-deliver-story-step9-status-wording-conflicts-with-BC-5.41.004 — deliver-story skill Step 9 wording `completed`→`merged` skill-doc fix in PR #763.

## Human Rulings (AskUserQuestion 2026-07-24)

| Ruling | Decision |
|--------|----------|
| Consolidate all fix-PR findings into one PR | APPROVED — PR #763 created and squash-merged 948f0fb1 |
| W-SEC-001 MEDIUM CWE-693 Layer-2 hook | Register follow-up story → S-21.06 (Layer-2 sync-protocol WASM guard; W-SEC-001; faece5a8; P1; 8pts; wave 3; draft) |
| Post-gate action | Wrap after gate close → D-892 session-wrap next action |

## Consolidated Fix PR #763

**Branch:** fix/e21-w1-gate-findings
**PR:** #763
**Merge SHA:** 948f0fb1
**Merged:** 2026-07-24 (human-executed squash merge)
**CI:** 13/13 green at 948f0fb1

**Commits (12 in PR, squash-merged):**
1. MINOR-1: `skip_global_options` dedup refactor (behavior-identical; 138/138 tests)
2. MINOR-2: `_run_dispatcher` temp-file envelope (kills quote-escaping fragility)
3. MINOR-3: T-005b EC-005 stat-only fallback isolation test (POLICY 15 v1.4.10 per-mutant evidence)
4. MINOR-4: git-checkout pre-check remote-ref guidance (per-story-delivery.md)
5. W-SEC-003: CWE-116 fix — `gh pr comment --body '...'`→`--body-file "$TMPFILE"` (mktemp) in pr-manager.md Step 5c
6. BC-5.41.004 wording sweep: `completed`→`merged` in 6 instructional files (deliver-story/wave-gate/SKILL.md/phase-f3/step-01-load-story-graph.md + related)
7. NITPICK-1: log-level consts
8. NITPICK-4: `_RUN_DISPATCHER_STDERR` contract rename
9. NITPICK-7: doc rationale
10. NITPICK-8: cross-ref addition
11. F-WG1-001: hooks-registry.toml env_allow comment precision
12. F-WG1-002: hooks-registry.toml ADR-031 cross-reference

**2-round pr-reviewer convergence:** round-1 REQUEST_CHANGES (pr-review.md) → round-2 APPROVE (pr-review-round-2-final.md)

**Evidence files:** `.factory/code-delivery/fix-e21-w1-gate/`

## Gate Verdict

**GATE VERDICT: PASSED**

All required gate legs satisfied:
- Full suite: PASS-AFTER-REMEDIATION (all failures traced to D-890 bookkeeping class; fixed pre-merge; CI 13/13 green)
- Wave adversary: CLEAN (N2/OBS2 fixed in PR #763)
- Code review: APPROVE-WITH-NOTES (all MINORs fixed; nitpicks fixed or accepted-with-record)
- Security: PASS (W-SEC-001 MEDIUM deferred to S-21.06 per human ruling; W-SEC-003 fixed; W-SEC-002/004 accepted-with-record)
- Consistency: FULL PASS (F-WG21-001 fixed at 87ff340f prior to PR #763 merge)
- Holdout: N/A-WITH-JUSTIFICATION (consistent project-wide precedent; wave adversary is coverage substitute)

**W2 (S-21.04, S-21.05) DISPATCHABLE** per D-862 SEQUENTIAL human approval.

NEXT: D-892 session-wrap (human /wrap directive 2026-07-24); then next session: E-21 W2 dispatch (S-21.04→S-21.05) + rc.24 decision.
