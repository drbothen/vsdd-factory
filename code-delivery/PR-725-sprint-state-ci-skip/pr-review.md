# Fresh-Eyes PR Review — PR #725

**Title:** fix(tests): skip live sprint-state.yaml assertions under CI — restore documented local-only design
**Author:** arcavenai (external, fork PR from ArcavenAE/vsdd-factory)
**Base:** develop  •  **Head:** fix/sprint-state-live-tests-ci-skip
**Diff:** 1 file, +30 / -0 — `plugins/vsdd-factory/tests/sprint-state-format.bats`
**CI:** 13/13 green (incl. bats-full-suite linux)

## Verdict: APPROVE — no blocking findings

The change is correct, minimal, and well-documented. One MINOR (stale PR
description) and two ADVISORY items below. None block merge.

---

## Checklist results

### 1. Guard-function correctness — PASS
`_skip_live_artifact_in_ci` skips only when `GITHUB_EVENT_NAME == "pull_request"`.
This is the correct, narrow signal: in a `pull_request` Actions run, GitHub sets
`GITHUB_EVENT_NAME=pull_request`, and CI mounts the BASE repo's factory-artifacts —
data no PR can modify. Push runs (`GITHUB_EVENT_NAME=push`) and local runs (var
unset) fall through and keep full enforcement. The guard is placed as the first
line of each test, before the existing `[ ! -f "${_PRODUCTION_SPRINT_STATE}" ]`
check, so the early skip is reached before any live-data access. Ordering is
correct. `skip` is the proper bats primitive. CI green confirms the branch fires.

### 2. Callsite coverage — PASS (with ADVISORY on completeness)
All 5 live-production-file tests named in the description are guarded, and each
guarded test is exactly the set that also carries the `_PRODUCTION_SPRINT_STATE`
file-existence guard — a reliable marker of a live-data test:
- test_sprint_state_stories_list_present
- test_real_production_file_round_trip
- test_real_production_file_completeness_and_status_fidelity
- test_supersession_edge_tolerated_partition_placement
- test_partitions_sorted_by_full_graph_depth_def_b

See ADVISORY-1 re: verifying no un-guarded live test remains (info-wall limit).

### 3. Skip-message clarity — PASS
Message cites issue #724, explains the base-mount mismatch, and states push runs
keep the canary. Informative for a debugging reader. The header comment block is
thorough and correctly explains the rationale.

### 4. Scope cleanliness — PASS
Single test file, additions only, no deletions, no unrelated changes. No
factory-artifact touches. Class-0 change confined to develop-tracked files.

### 5. Local-only design consistency — PASS (per diff)
Consistent with the pre-existing `[ ! -f ... ]` skip guards already in this file
and with the PORTABILITY notes cited. Cross-suite consistency with OTHER test
files could not be verified from the diff (info wall) but the in-file pattern is
coherent.

---

## Findings

### MINOR — Description accuracy: PR body describes the superseded `GITHUB_ACTIONS` approach
`sprint-state-format.bats` (guard implementation) — PR body "What changed" + "Test evidence"

The PR body's "What changed" section states *"Guard condition is `GITHUB_ACTIONS`
(any Actions run)"* and the "Test evidence" block simulates `GITHUB_ACTIONS=true`.
The actual merged code (commit 7d1d396, "narrow live-artifact skip to pull_request
runs") keys on `GITHUB_EVENT_NAME == "pull_request"`, not `GITHUB_ACTIONS`. The
body's own follow-up note and the second commit message document the narrowing,
but the primary "What changed" / "Test evidence" prose was not updated and now
contradicts the code. The code is correct; only the description is stale.
Suggestion: update the PR body so a human reviewer relying on it sees the
`pull_request` guard and matching evidence.

### ADVISORY — develop push CI stays red until #724 Fix A (data re-sort) lands
By design, push runs keep the five live tests active as a canary, so the
`bats-full-suite` job on develop-push will report red for
`test_partitions_sorted_by_full_graph_depth_def_b` until the sprint-state.yaml
PC3 def-b re-sort (the separate, maintainer-owned Fix A) lands. This is
disclosed in the PR body and is the intended, correct behavior — the narrow
`pull_request` guard is more production-grade than a blanket `GITHUB_ACTIONS`
skip precisely because it preserves this canary. Flagged only so merging
reviewers are aware develop-push will be red (not a regression, not a blocker).

### ADVISORY — Cannot confirm no other live-data test remains un-guarded (info wall)
Review sees only the diff, not the full 14-test file. The guarded set matches
every test carrying the `_PRODUCTION_SPRINT_STATE` existence guard shown in the
hunks, and the description states fixture tests 2–6/8–11 are untouched, so the
mapping is internally consistent. Recommend a maintainer with full-file view
confirm there is no additional test that reads live `.factory/` artifacts
without the new guard.

---

## Reviewer notes / what was verified
- Diff is additions-only; no existing assertion weakened for local/push paths.
- Guard reached before file-existence check and before any live-data read.
- `skip` semantics correct; CI 13/13 green corroborates the skip path.
- No AI attribution, no `--no-verify`, no forbidden patterns in the diff.
