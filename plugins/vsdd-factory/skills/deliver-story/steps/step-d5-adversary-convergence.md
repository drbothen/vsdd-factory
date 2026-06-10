---
name: step-d5-adversary-convergence
description: Run the per-story adversary convergence loop to 3 clean NITPICK_ONLY passes before proceeding to demos. Writes convergence-state.json per BC-5.39.001.
---

# Step D.5: Per-Story Adversary Convergence

> **Shared context:** Read `./_shared-context.md` before executing this step — it contains the Iron Law, dispatch discipline, context discipline, model selection, and verification rules.

## Purpose

Ensures the story implementation is adversarially reviewed to convergence before demo recording begins. Blocks wave-gate dispatch (via the `validate-per-story-adversary-convergence` WASM hook) unless convergence state exists and satisfies `passes_clean >= 3 AND last_classification == "NITPICK_ONLY"`.

## Behavioral Contract Anchors

- BC-5.39.001 — convergence criterion, state file schema, loop procedure
- BC-5.39.002 — deferred-finding classification rules (cross-story, integration, system-level, architectural)
- ADR-017 — per-story adversary phasing rationale

## Convergence Criterion

`passes_clean >= 3 AND last_classification == "NITPICK_ONLY"` in
`.factory/cycles/<cycle-id>/<story-id>/adversary-convergence-state.json`.

The `passes_clean` counter increments by 1 per pass where `last_classification == "NITPICK_ONLY"`.
It RESETS to 0 if any pass produces a finding above NITPICK_ONLY. Minimum 3 clean passes — no exceptions.

## Dispatch Loop

**Step 1 — Adversary dispatch:**

Before building the adversary context package, the orchestrator MUST capture and assert the worktree-identity tuple by calling the tested helper:

```bash
# The orchestrator records the implementer's final micro-commit SHA immediately
# after the TDD-green step. This is a PRE-PUSH local cascade — no remote
# tracking branch exists yet. Replace <IMPLEMENTER-FINAL-COMMIT-SHA> with the
# value captured right after the implementer's last commit.
STORY_ID="<STORY-ID>"
EXPECTED_HEAD_SHA="<IMPLEMENTER-FINAL-COMMIT-SHA>"

# Invoke the tested helper to resolve + assert the 4-field WORKTREE-IDENTITY TUPLE.
# The helper:
#   1. Resolves CANONICAL_REPO_ROOT nesting-safe via git-common-dir
#   2. Guards that canonical .factory is mounted (factory-artifacts)
#   3. Parses git worktree list --porcelain SPACE-SAFE (${line#worktree }, not awk $2)
#      with ANCHORED, case-insensitive story-ID matching (S-12.08 never matches S-12.088)
#   4. Asserts git -C <worktree> rev-parse HEAD == EXPECTED_HEAD_SHA
#   5. On mismatch, emits dispatch-error and exits non-zero
#   6. On success, prints the 4-field tuple to stdout
#
# Any non-zero exit from the helper is a preflight assertion failure:
# STOP — do NOT dispatch the adversary. Fix the checkout and re-run.
IDENTITY_TUPLE="$(STORY_ID="$STORY_ID" EXPECTED_HEAD_SHA="$EXPECTED_HEAD_SHA" \
  "${CLAUDE_PLUGIN_ROOT}/bin/resolve-worktree-identity.sh")" || {
  echo "dispatch-error: worktree-identity preflight failed — $IDENTITY_TUPLE"
  exit 1
}

# Parse the 4 tuple fields from the helper output:
#   worktree-abs-path:   <path>
#   feature-HEAD-SHA:    <sha>
#   story-id:            <id>
#   canonical-repo-root: <root>
WORKTREE_ABS_PATH="$(echo "$IDENTITY_TUPLE"  | grep '^worktree-abs-path:'   | sed 's/^worktree-abs-path:[[:space:]]*//')"
FEATURE_HEAD_SHA="$(echo "$IDENTITY_TUPLE"   | grep '^feature-HEAD-SHA:'    | sed 's/^feature-HEAD-SHA:[[:space:]]*//')"
CANONICAL_REPO_ROOT="$(echo "$IDENTITY_TUPLE" | grep '^canonical-repo-root:' | sed 's/^canonical-repo-root:[[:space:]]*//')"
# The feature HEAD SHA returned by the helper equals EXPECTED_HEAD_SHA (helper asserted this).
# EXPECTED_HEAD_SHA and FEATURE_HEAD_SHA are identical — use either for the embedded tuple.
```

**Expected-SHA model:** This step runs LOCAL (pre-push) in the per-story flow (stubs → tests → TDD green → LOCAL adversary 3-CLEAN → demo → push → PR). There is NO remote tracking branch at this point. The `EXPECTED_HEAD_SHA` is therefore NOT resolved from `@{upstream}` or any remote ref — it is the SHA the orchestrator recorded from the implementer's last commit. The orchestrator MUST capture this value (`git -C "$WORKTREE_ABS_PATH" rev-parse HEAD`) immediately after the TDD-green step, before dispatching the adversary. At the PR-level adversarial perimeter (after push), the expected value IS the pushed remote-branch tip and `@{upstream}` is appropriate there; do not conflate the two contexts.

**Structural guarantee:** The adversary reads ONLY from the embedded `worktree-abs-path` for feature code, so cwd-resolution to the wrong checkout (the #176 failure) cannot occur. The SHA equality assertion is the additional temporal guard: same worktree, same commit, before and after the dispatch.

The dispatch MUST embed the expected feature HEAD SHA (`EXPECTED_HEAD_SHA`), the absolute worktree path (`WORKTREE_ABS_PATH`), the story-id, and the canonical repo root (`CANONICAL_REPO_ROOT`) as a WORKTREE-IDENTITY TUPLE (4 fields) in the adversary task prompt (see adversarial-review SKILL.md "Worktree-Identity Preflight (MANDATORY)" for the exact format). The `canonical-repo-root` is the main repo root where `factory-artifacts` is mounted at `.factory/`; it is the authoritative source for spec, BC, and ADR files — the adversary reads from `<canonical-repo-root>/.factory/...`, NOT from the stale worktree `.factory/specs` snapshot. The embedded `feature HEAD SHA` is the EXPECTED commit — the orchestrator-recorded implementer tip. A mismatch between the worktree's actual HEAD and the expected feature HEAD SHA is a STOP/dispatch-error condition, not a content finding: fix the worktree checkout and re-run, do NOT proceed to the adversary with a mismatched tree.

This identity tuple is the orchestrator's assertion, made before the adversary reads any files, that the worktree is on the correct commit. The preflight assertion MUST pass — i.e., the adversary MUST find the tuple present and internally consistent — before findings are accepted. Any adversary response that omits tuple verification or emits a `dispatch-error` about a missing tuple MUST be treated as a dispatch misconfiguration, not a content finding; fix the dispatch and re-run.

Dispatch `adversary` agent (model tier: Capable) with context:
- WORKTREE-IDENTITY TUPLE (4 fields) (embedded verbatim as described above)
- Story worktree diff (`.worktrees/<STORY-ID>/`) — use `worktree-abs-path` from the identity tuple as the read root
- Story spec (`<canonical-repo-root>/.factory/stories/<STORY-ID>-*.md`) — canonical repo-root path only (NOT worktree snapshot)
- Anchored BCs listed in the story's `behavioral_contracts:` frontmatter field — canonical repo-root paths only
- Current convergence state file (if it exists)

Task: "Review the story diff against the story spec and anchored BCs. Classify each finding as CRITICAL, HIGH, MEDIUM, LOW, or NITPICK_ONLY. Tag out-of-scope findings (cross-story, integration, system-level, architectural) as deferred per BC-5.39.002. Write updated convergence state JSON to `.factory/cycles/<cycle-id>/<story-id>/adversary-convergence-state.json`."

**Step 2 — State-manager backup:**
Dispatch `state-manager` to commit the updated state file to `factory-artifacts`.

**Step 3 — Check convergence:**
Read `.factory/cycles/<cycle-id>/<story-id>/adversary-convergence-state.json`.
- If `passes_clean < 3` OR `last_classification != "NITPICK_ONLY"`:
  dispatch `implementer` to fix within-story findings, then repeat from Step 1.
- If `passes_clean >= 3 AND last_classification == "NITPICK_ONLY"`: proceed to Step E.

## Deferred Findings (BC-5.39.002 PC4)

Out-of-scope findings are written to `deferred_findings[]` and do NOT block convergence.
They are surfaced at the wave-gate or Phase 5 adversary pass as appropriate.

Deferred categories:
- `cross-story` — requires context from another story → target: `wave-gate`
- `integration` — requires multi-story or subsystem context → target: `wave-gate`
- `system-level` — concerns system-wide behavior → target: `phase-5`
- `architectural` — concerns design decisions spanning the architectural boundary → target: `phase-5`

## State File Schema (BC-5.39.001 PC2)

Path: `.factory/cycles/<cycle-id>/<story-id>/adversary-convergence-state.json`

```json
{
  "passes_clean": 0,
  "last_finding_count": 3,
  "last_classification": "HIGH",
  "last_timestamp": "2026-05-07T00:00:00Z",
  "deferred_findings": []
}
```

## Exit Condition

`passes_clean >= 3 AND last_classification == "NITPICK_ONLY"` in the state file.

**Verify independently** — read the state file after the final adversary pass and confirm both fields before proceeding to Step E.

## Artifacts

- `.factory/cycles/<cycle-id>/<story-id>/adversary-convergence-state.json` — convergence state (committed to factory-artifacts by state-manager)
- Any within-story fix commits on the feature branch

## Note on Bootstrap Exemption (D-354)

The bootstrap cohort stories (S-12.01, S-12.02, S-13.01 in cycle v1.0-feature-engine-discipline-pass-1) were delivered before this gate existed and are exempt from the gate's blocking behavior for that cycle per D-354. For all stories in subsequent cycles, this step is mandatory and blocking.
