---
document_type: session-review
date: 2026-07-13
run_id: D-832..D-834 (W1-merge-completion; v1.0-brownfield-backfill)
path: 2
path_name: brownfield
product: vsdd-factory
duration: ~2.5h (CI runner queue dominant; merge window 14:33Z–15:54Z)
total_cost: ~$0.50 (estimated; 3 SM post-merge bursts + 1 governance burst; no adversary runs)
stories_delivered: 3
cycle: v1.0-brownfield-backfill
stories_merged: [S-19.01, S-19.02, S-19.03]
decisions_codified: [D-832, D-833, D-834, D-835]
---

# Session Review: vsdd-factory — brownfield — 2026-07-13

> **NOTE:** File placed at flat cycle level; `session-reviews/` subdirectory is not registered in artifact-path-registry.yaml. Registration needed before subdirectory can be used (fold into S-19.04 PR which already touches artifact-path-registry.yaml).

## Executive Summary

Wave 1 of E-19 completed cleanly: all three stories (S-19.01, S-19.02, S-19.03) merged to develop with CI 12/12 GREEN, POL-14 promotions executed correctly, and merged_count advancing 98→101. The dominant friction was CI runner queue latency (~2.5h wall-clock for three PRs) and large-file Edit tool pressure (STORY-INDEX.md, lessons.md, decision-log.md all producing WASM fuel-timeout advisories). Top recommendation: compact decision-log.md and lessons.md at the next cycle boundary before these files further degrade Edit tool usability.

## Run Overview

| Metric | Value | Benchmark | Status |
|--------|-------|-----------|--------|
| Total cost | ~$0.50 (est) | — | — |
| Duration | ~2.5h | — | CI-bound |
| Stories delivered | 3 | — | PASS |
| Adversarial rounds | 0 (W1 TDD session only) | — | n/a |
| PR review rounds | 1 per story (pre-merge CI gate) | — | PASS |
| Gate failures | 1 (stale-verdict; resolved by human) | — | RESOLVED |
| Human interventions | 3 (stale-verdict; ADR-030 §D3 redirect; W2 gate) | — | EXPECTED |
| Holdout satisfaction | n/a (W1 merge session; no holdout eval) | — | — |
| Mutation kill rate | n/a | — | — |

---

## 1. Cost Analysis

Low agent cost — three sequential SM post-merge bursts (D-832, D-833, D-834) and one governance burst (D-835). No adversary dispatches, no implementer or test-writer dispatches.

Dominant infrastructure cost: CI runner queue latency (~2.5h wall-clock for the three-PR merge window). GitHub Actions queue contention on the bats-full-suite leg was the primary bottleneck. This matches TD #70 (runner cache optimization) as a recurring session cost. TD #70 elevated to scheduled (maintenance sweep / W2 window) per IP-005.

macOS/Windows binary builds (~1h+) are not triggered by these stories (no dispatcher source changes), but remain a background cost on sessions that do touch dispatcher source.

**Recommendation:** Implement TD #70 Option C in the W2/maintenance window to reduce per-session CI latency.

## 2. Timing Analysis

| Event | Time (Z) | Elapsed |
|-------|----------|---------|
| S-19.02 PR #610 MERGED (f5ea12e9) | 2026-07-13T14:33:57Z | t+0 |
| S-19.01 PR #613 MERGED (8d1721f7) | 2026-07-13T14:49:50Z | +16 min |
| S-19.03 PR #611 MERGED (091ce499) | 2026-07-13T15:54:21Z | +1h 20 min |

S-19.03 bottleneck: CI re-triggered after develop advanced (PR #613 merge). bats-full-suite leg was last to complete. The ~1h gap between S-19.01 and S-19.03 is 80% GH runner queue, 20% SM burst time.

**Recommendation:** TD #70 — action at W2 window.

## 3. Convergence Analysis

Not applicable this session. E-19 converged at pass-61 CLEAN (D-823, 2026-07-11; streak 3/3; BC-5.39.001 satisfied). W1 TDD was dispatched in parallel (D-825) post-convergence.

**Historical note:** The 61-pass trajectory is higher than typical (E-18 required ~17 passes). The elevated count reflects VP authoring concurrent with TDD implementation (passes 47–58): VP-094..VP-101 were authored and refined in the same adversarial cycle, with each VP citation sweep triggering new spec-drift finding classes. The trade-off (parallel authoring → more passes but earlier integration detection) was accepted per D-386 Option C.

**Recommendation:** For W2 (S-19.04, S-19.05, S-19.06), VP-099..VP-101 are already authored; expect fewer convergence passes than E-19 overall.

## 4. Agent Behavior Analysis

**State-manager single-writer discipline:** Held throughout. All three post-merge bursts were strictly sequential in merge order. Single-commit-per-burst (TD-VSDD-053) verified for D-832, D-833, D-834. No concurrent SM writes.

**D-832 POL-3 bash-bypass (self-reported):** A `bash cat>> append` was used for decision-log.md during the D-832 burst in a prior session. This is a POL-3 / TD-FACTORY-HOOK-BYPASS-001 P0 violation. Root cause: decision-log.md at ~10K lines creates Edit friction (WASM timeouts are advisory, not blocking, but add noise). Lesson codified: L-BB-decision-log-file-growth-triggers-pol3-bypass-risk. Fix path: always use Edit; compact when over budget (IP-003 this burst).

**Orchestrator stale-verdict escalation:** Correct. check-stale-verdict.sh surfaced a stale READY verdict for S-19.01 after develop advanced. Orchestrator surfaced to human; human resolved via delta-integrity check. No spurious re-review triggered. Protocol: L-BB-post-ready-delta-integrity-check-for-mechanical-sync-commits.

## 5. Gate Outcome Analysis

| Gate | Outcome | Detail |
|------|---------|--------|
| POL-14 BC-5.42.001 | PASS | draft→active v1.7→v1.8; 5-leg parity; input-hash 95907cf→8d50681 |
| POL-14 BC-4.13.001 | PASS (already active) | D-545; no promotion needed |
| POL-14 BC-2.07.001 | PASS | draft→active v1.5→v1.6; 5-leg parity; input-hash a694373→c9f4101 |
| POL-14 BC-2.02.011 | PASS (already active) | v1.7; no promotion needed |
| POLICY 16 D-NNN gate | PASS (D-832/D-833/D-834) | confirmed max in decision-log.md before each allocation |
| D-803 epic heading-parity | PASS 11/11 each burst | E-9..E-19 headings UNCHANGED |
| Stale-verdict (S-19.01) | RESOLVED | human delta-integrity check; single sync merge; no story files touched |
| feature/S-19.02 branch | RESOLVED (D-835) | GET → 404; D-832 anomaly closed |
| CI green pre-merge | PASS (12/12 each PR) | verified at merge gate |

All mandatory POL-14 and POLICY 16 gates passed. One stale-verdict gate required human resolution (correct escalation). feature/S-19.02 anomaly resolved within the D-835 governance burst.

## 6. Wall Integrity Analysis

Not applicable this session. All W1 story PRs were pure implementation stories; no spec-wall modifications were made. The spec-wall (BCs, VPs, architecture) was not touched by W1 PRs. POL-14 promotions are lifecycle-status changes, not spec-content changes.

## 7. Quality Signal Analysis

| Story | PR | CI | POL-14 | Known Defects | Notes |
|-------|-----|-----|--------|---------------|-------|
| S-19.01 pr-manager-hardening | #613 | 12/12 GREEN | BC-5.42.001 promoted | S-19.03 F-005 TOCTOU pre-existing LOW (out of scope) | feature/S-19.01 DELETED |
| S-19.02 verify-factory-lock-output-too-large | #610 | 12/12 GREEN | BC-4.13.001 already active | None | feature/S-19.02 anomaly resolved D-835 |
| S-19.03 warn-pending-wave-gate-file-not-found | #611 | 12/12 GREEN | BC-2.07.001 promoted | S-19.03 F-005 TOCTOU pre-existing LOW | feature/S-19.03 DELETED |

No post-merge defects detected. All stories LOCAL-converged 3/3 before dispatch. W1 quality signal: CLEAN.

## 8. Pattern Detection

**Pattern 1 — Large-file Edit pressure (systemic, recurring):**
Three file classes are now consistently triggering WASM fuel-timeout PostToolUse advisories:
- STORY-INDEX.md (~88K tokens): every Edit produces timeout. Advisory only; writes succeed.
- lessons.md (6510 lines > 4000 hard cap per D-442(e)): compaction needed. Appends will timeout.
- decision-log.md (10233 lines > to-be-established 3500 hard cap per IP-003): entering friction zone.

This pattern will worsen each session without compaction. Mitigation: compact at cycle boundaries.

**Pattern 2 — CI transient failures (recurring):**
At least one CI infra-class failure occurred pre-merge. The triage distinction (infra-class: retry; code-class: diagnose) was not consistently applied, adding ~30 min of diagnostic overhead. Pattern will recur. Protocol codified: L-BB-transient-ci-infra-failure-recovery-runbook.

**Pattern 3 — Sequential SM burst queuing discipline (confirmed-practice):**
All three post-merge bursts executed in strict merge order (D-832→D-833→D-834). This pattern is correct and is now codified as L-BB-sequential-sm-burst-queuing-on-multi-merge-sessions.

**Pattern 4 — Feature branch non-deletion (recurrence risk):**
feature/S-19.02 was not auto-deleted post-merge (D-832 anomaly). The previous standard response ("human can delete at will") is insufficient — it left an open hygiene item with no concrete follow-up gate. The new standard: record explicit cleanup gate in STATE.md open items. Codified: L-BB-feature-branch-non-deletion-requires-explicit-cleanup-action.

## 9. Governance Policy Audit

**Existing policy enforcement check:**

- `append_only_numbering` — PASS. D-832, D-833, D-834, D-835 assigned in sequence. No ID reuse or renumbering.
- `lift_invariants_to_bcs` — PASS. No new domain invariants surfaced this session.
- `state_manager_runs_last` — PASS. SM committed last in all three post-merge bursts.
- `semantic_anchoring_integrity` — PASS. No anchor changes this session.
- `creators_justify_anchors` — PASS. No anchor creation this session.
- `architecture_is_subsystem_name_source_of_truth` — PASS. No subsystem references modified.
- `bc_h1_is_title_source_of_truth` — PASS. BC-2.07.001 and BC-5.42.001 titles unchanged.
- `bc_array_changes_propagate_to_body_and_acs` — PASS. POL-14 promotions are lifecycle-status only; no `bcs:` array changes.
- `vp_index_is_vp_catalog_source_of_truth` — PASS. VP-INDEX unchanged this session.

**New policy candidates:**

None at 3+ burst threshold this session (W1 merge was 3 bursts, each a clean execution). The L-BB lessons capture process-gaps but no class recurred 3+ times within this session. Candidates for next session review:
- Large-file compaction trigger: if WASM fuel exhaustion advisories recur on 3+ separate files in a single session, it qualifies as a policy candidate.
- CI infra-class triage: if the triage failure recurs in W2 session, elevate to governance policy.

---

## Improvement Proposals

### Proposal 1: CI-Triage Runbook (IP-001)
- **Category:** workflow
- **Priority:** MEDIUM
- **Evidence:** ~30 min diagnostic overhead from infra-class CI failure triage this session. Same pattern occurred in prior sessions.
- **Recommendation:** Add infra-class vs code-class CI failure classification to orchestrator per-story-delivery workflow doc. Protocol: infra-class (pre-code, 503/runner errors) → wait completion → `gh run rerun --failed`; GitHub 403-blocks reruns mid-run; escalate only after a rerun fails on a code step.
- **Affected files:** orchestrator per-story-delivery workflow doc; possibly CLAUDE.md §Operational tips.
- **Risk:** Low. Documentation change only.

### Proposal 2: BC-5.42.001 Delta-Integrity Invariant (IP-002)
- **Category:** gate
- **Priority:** MEDIUM
- **Evidence:** Stale-verdict case for S-19.01 required human resolution; no existing BC invariant covers the post-ready develop-sync scenario.
- **Recommendation:** BC-5.42.001 amendment — add delta-integrity invariant as explicit AC or invariant: when PR HEAD advances past READY by exactly one develop-sync merge commit, the READY verdict is preserved if (a) single additional merge commit, (b) diff touches only already-reviewed develop files, (c) no story-delivery files touched, (d) human approves proceed.
- **Affected files:** BC-5.42.001.md; BC-INDEX.md annotation.
- **Risk:** Medium. BC amendment requires PO dispatch and 5-leg parity.

### Proposal 3: Decision-Log Size Budget (IP-003)
- **Category:** workflow
- **Priority:** HIGH
- **Evidence:** decision-log.md at 10233 lines; D-832 POL-3 bypass occurred due to file friction. No budget codified.
- **Recommendation:** Codify size budget in cycles/v1.0-brownfield-backfill/INDEX.md: soft ≤2500 lines / hard ≤3500 lines; compaction destination decisions-log-archive.md (POLICY 1 preserved via archive).
- **Affected files:** INDEX.md (this burst). decision-log.md compaction (next cycle boundary).
- **Risk:** Low. Documentation + future compaction action.

### Proposal 4: Authorize verify-state-timestamp-refresh Fix Story (IP-004)
- **Category:** quality
- **Priority:** HIGH (P0)
- **Evidence:** verify-state-timestamp-refresh 64KiB live bug confirmed 3× in prior sessions. Human authorized new W2 story 2026-07-13.
- **Recommendation:** Story-writer dispatch for S-19.08 (E-19 wave 2/3; TD-VSDD-061-class; fix the 64KiB limit that causes verify-state-timestamp-refresh to pass spuriously on large STATE.md files).
- **Affected files:** New S-19.08 story file; STORY-INDEX.md; sprint-state.yaml.
- **Risk:** Low to medium. Well-scoped fix; same subsystem as S-19.02 (verify-factory-lock byte budget fix).

### Proposal 5: TD #70 Runner Cache Scheduling (IP-005)
- **Category:** timing
- **Priority:** HIGH
- **Evidence:** ~2.5h CI wall-clock this session; bats-full-suite consistently last leg; TD #70 has been tracked for multiple sessions.
- **Recommendation:** Schedule TD #70 Option C (release pipeline build-time optimization / runner cache) for maintenance sweep or W2 window. Elevate from tracked to scheduled.
- **Affected files:** CI workflow files (.github/workflows/*.yml); tech-debt-register.md.
- **Risk:** Medium. CI workflow changes require careful testing.

---

## Metrics for Next Run

1. **CI latency per PR** — measure bats-full-suite leg duration after TD #70 if implemented.
2. **decision-log.md line count** — verify compaction brought below 2500 soft cap before W2 session close.
3. **lessons.md line count** — verify compaction planned (currently 6510 lines, above 4000 hard cap D-442(e)).
4. **W2 POL-14 gate** — S-19.04 (behavioral_contracts: [] per S-18.09 precedent), S-19.05 (BC-3.08.001 — verify already active), S-19.06 (BC-1.17.001 — verify status before merge).
5. **S-19.08 story quality** — verify delta-integrity invariant is correctly scoped to not break existing stale-verdict detection.
