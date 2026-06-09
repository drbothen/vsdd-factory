#!/usr/bin/env bats
# pr-lifecycle-hooks.bats — tests for PR lifecycle enforcement hooks:
#   validate-pr-description-completeness.sh (PostToolUse on Write)
#   validate-pr-merge-prerequisites.sh (PreToolUse on Agent)
# NOTE: validate-pr-review-posted.sh was ported to native WASM (W-15);
# its bats tests were removed.
# NOTE: block-ai-attribution.sh was ported to native WASM (W-15);
# its bats tests were removed.

setup() {
  PLUGIN_ROOT="$(cd "$BATS_TEST_DIRNAME/.." && pwd)"
  HOOKS="$PLUGIN_ROOT/hooks"
  WORK=$(mktemp -d)
  mkdir -p "$WORK/.factory/code-delivery/STORY-001"
}

teardown() {
  rm -rf "$WORK"
}

_run_posttool_write() {
  local hook="$1"
  local file="$2"
  INPUT=$(jq -nc --arg fp "$file" '{tool_input: {file_path: $fp}}')
  run bash -c "echo '$INPUT' | '$HOOKS/$hook' 2>&1"
}

_run_pretool_agent() {
  local subagent="$1"
  local prompt="$2"
  INPUT=$(jq -nc --arg s "$subagent" --arg p "$prompt" '{tool_name: "Agent", tool_input: {subagent_type: $s, prompt: $p}}')
  run bash -c "cd '$WORK' && echo '$INPUT' | '$HOOKS/validate-pr-merge-prerequisites.sh' 2>&1"
}

# ========================================================================
# Syntax and wiring
# ========================================================================

@test "pr-description-completeness: passes syntax check" {
  run bash -n "$HOOKS/validate-pr-description-completeness.sh"
  [ "$status" -eq 0 ]
}

@test "pr-merge-prerequisites: passes syntax check" {
  run bash -n "$HOOKS/validate-pr-merge-prerequisites.sh"
  [ "$status" -eq 0 ]
}

@test "registry wires pr-description-completeness under PostToolUse" {
  load "${BATS_TEST_DIRNAME}/helpers/registry.bash"
  registry_has_hook "validate-pr-description-completeness" "PostToolUse"
}

@test "registry wires pr-merge-prerequisites under PreToolUse Agent" {
  load "${BATS_TEST_DIRNAME}/helpers/registry.bash"
  registry_has_hook "validate-pr-merge-prerequisites" "PreToolUse" "Agent"
}

# ========================================================================
# validate-pr-description-completeness
# ========================================================================

@test "pr-description: passes complete description" {
  cat > "$WORK/.factory/code-delivery/STORY-001/pr-description.md" << 'EOF'
# [S-0.01] Test Story

## Architecture Changes
Component diagram here.

## Story Dependencies
No deps.

## Spec Traceability
BC-2.01.001 -> AC-1 -> test_auth

## Test Evidence
5/5 passing, 90% coverage

## Demo Evidence
![demo](docs/demo-evidence/S-0.01/AC-001.gif)

## Pre-Merge Checklist
- [x] Tests pass
- [x] Demo recorded
EOF
  _run_posttool_write validate-pr-description-completeness.sh "$WORK/.factory/code-delivery/STORY-001/pr-description.md"
  [ "$status" -eq 0 ]
}

@test "pr-description: blocks missing Architecture Changes section" {
  cat > "$WORK/.factory/code-delivery/STORY-001/pr-description.md" << 'EOF'
# [S-0.01] Test Story

## Story Dependencies
No deps.

## Spec Traceability
chain here

## Test Evidence
passing

## Demo Evidence
recorded

## Pre-Merge Checklist
done
EOF
  _run_posttool_write validate-pr-description-completeness.sh "$WORK/.factory/code-delivery/STORY-001/pr-description.md"
  [ "$status" -eq 2 ]
  [[ "$output" == *"Architecture Changes"* ]]
}

@test "pr-description: blocks missing multiple sections" {
  cat > "$WORK/.factory/code-delivery/STORY-001/pr-description.md" << 'EOF'
# [S-0.01] Test Story

## Architecture Changes
done

## Pre-Merge Checklist
done
EOF
  _run_posttool_write validate-pr-description-completeness.sh "$WORK/.factory/code-delivery/STORY-001/pr-description.md"
  [ "$status" -eq 2 ]
  [[ "$output" == *"Story Dependencies"* ]]
  [[ "$output" == *"Test Evidence"* ]]
}

@test "pr-description: blocks unresolved template placeholders" {
  cat > "$WORK/.factory/code-delivery/STORY-001/pr-description.md" << 'EOF'
# [{story_id}] {story_title}

## Architecture Changes
{component_A} calls {component_B}

## Story Dependencies
none

## Spec Traceability
chain

## Test Evidence
{pass_count}/{total_count}

## Demo Evidence
recorded

## Pre-Merge Checklist
done
EOF
  _run_posttool_write validate-pr-description-completeness.sh "$WORK/.factory/code-delivery/STORY-001/pr-description.md"
  [ "$status" -eq 2 ]
  [[ "$output" == *"placeholder"* ]]
  [[ "$output" == *"{story_id}"* ]]
}

@test "pr-description: ignores non-pr-description files" {
  mkdir -p "$WORK/.factory/specs"
  echo "# Not a PR description" > "$WORK/.factory/specs/test.md"
  _run_posttool_write validate-pr-description-completeness.sh "$WORK/.factory/specs/test.md"
  [ "$status" -eq 0 ]
}

@test "pr-description: ignores pr-review.md" {
  echo "# Review findings" > "$WORK/.factory/code-delivery/STORY-001/pr-review.md"
  _run_posttool_write validate-pr-description-completeness.sh "$WORK/.factory/code-delivery/STORY-001/pr-review.md"
  [ "$status" -eq 0 ]
}

# ========================================================================
# validate-pr-merge-prerequisites
# ========================================================================

@test "pr-merge-prerequisites: passes when all evidence files exist" {
  echo "# Description" > "$WORK/.factory/code-delivery/STORY-001/pr-description.md"
  echo "# Review" > "$WORK/.factory/code-delivery/STORY-001/pr-review.md"
  echo "# Security" > "$WORK/.factory/code-delivery/STORY-001/security-review.md"
  _run_pretool_agent "vsdd-factory:github-ops" "cd $WORK && gh pr merge 42 --squash --delete-branch for STORY-001"
  [ "$status" -eq 0 ]
}

@test "pr-merge-prerequisites: blocks when pr-description.md missing" {
  echo "# Review" > "$WORK/.factory/code-delivery/STORY-001/pr-review.md"
  echo "# Security" > "$WORK/.factory/code-delivery/STORY-001/security-review.md"
  _run_pretool_agent "vsdd-factory:github-ops" "cd $WORK && gh pr merge 42 --squash for STORY-001"
  [ "$status" -eq 2 ]
  [[ "$output" == *"pr-description.md"* ]]
}

@test "pr-merge-prerequisites: blocks when pr-review.md missing" {
  echo "# Description" > "$WORK/.factory/code-delivery/STORY-001/pr-description.md"
  echo "# Security" > "$WORK/.factory/code-delivery/STORY-001/security-review.md"
  _run_pretool_agent "vsdd-factory:github-ops" "cd $WORK && gh pr merge 42 --squash for STORY-001"
  [ "$status" -eq 2 ]
  [[ "$output" == *"pr-review.md"* ]]
}

@test "pr-merge-prerequisites: blocks when security-review.md missing" {
  echo "# Description" > "$WORK/.factory/code-delivery/STORY-001/pr-description.md"
  echo "# Review" > "$WORK/.factory/code-delivery/STORY-001/pr-review.md"
  _run_pretool_agent "vsdd-factory:github-ops" "cd $WORK && gh pr merge 42 --squash for STORY-001"
  [ "$status" -eq 2 ]
  [[ "$output" == *"security-review.md"* ]]
}

@test "pr-merge-prerequisites: passes security check when description says no findings" {
  cat > "$WORK/.factory/code-delivery/STORY-001/pr-description.md" << 'EOF'
# Description
## Security Review
Security review: no findings. All clear.
EOF
  echo "# Review" > "$WORK/.factory/code-delivery/STORY-001/pr-review.md"
  _run_pretool_agent "vsdd-factory:github-ops" "cd $WORK && gh pr merge 42 --squash for STORY-001"
  [ "$status" -eq 0 ]
}

@test "pr-merge-prerequisites: ignores non-merge dispatches" {
  _run_pretool_agent "vsdd-factory:github-ops" "cd $WORK && gh pr create --title test for STORY-001"
  [ "$status" -eq 0 ]
}

@test "pr-merge-prerequisites: ignores non-github-ops agents" {
  _run_pretool_agent "vsdd-factory:implementer" "cd $WORK && Implement STORY-001"
  [ "$status" -eq 0 ]
}

@test "pr-merge-prerequisites: reports all missing files in one message" {
  # No evidence files at all
  _run_pretool_agent "vsdd-factory:github-ops" "cd $WORK && gh pr merge 42 --squash for STORY-001"
  [ "$status" -eq 2 ]
  [[ "$output" == *"pr-description.md"* ]]
  [[ "$output" == *"pr-review.md"* ]]
  [[ "$output" == *"security-review.md"* ]]
}

@test "pr-merge-prerequisites: warns when delivery dir not found" {
  _run_pretool_agent "vsdd-factory:github-ops" "cd $WORK && gh pr merge 42 --squash for STORY-999"
  [ "$status" -eq 0 ]
  [[ "$output" == *"WARNING"* ]]
}

# ========================================================================
# pr-manager Step 8 remote-branch-deletion verification (issue #128)
# ========================================================================
# These are prompt-contract tests: they assert that the pr-manager.md
# playbook prose contains the required post-merge ls-remote verification
# block and that STEP_COMPLETE: step=8 is gated on empty ls-remote output.
# They MUST fail against the unmodified pr-manager.md (RED gate).

@test "pr-manager step-8: contains git ls-remote branch-deletion verification" {
  # pr-manager Step 8 must dispatch a git ls-remote check after the merge
  # to verify the remote branch is actually gone (not just requested).
  # Root cause: gh pr merge --delete-branch exit 0 is async (cli/cli#9073).
  local file="$PLUGIN_ROOT/agents/pr-manager.md"
  run grep -F 'git ls-remote' "$file"
  [ "$status" -eq 0 ]
}

@test "pr-manager step-8: ls-remote check targets refs/heads/ namespace" {
  # The verification must use the full refs/heads/<branch> path so it
  # targets the remote tracking ref, not tags or other refs.
  local file="$PLUGIN_ROOT/agents/pr-manager.md"
  run grep -F 'refs/heads/' "$file"
  [ "$status" -eq 0 ]
}

@test "pr-manager step-8: STEP_COMPLETE step=8 gated on ls-remote empty" {
  # STEP_COMPLETE: step=8 must NOT be emitted until ls-remote returns empty.
  # Verify that the file documents that the gate condition is ls-remote
  # returning empty (branch deleted) before the step=8 completion signal.
  local file="$PLUGIN_ROOT/agents/pr-manager.md"
  # The playbook must mention that STEP_COMPLETE is gated on the ls-remote
  # verification confirming the branch is gone.
  run grep -E 'ls-remote.*empty|empty.*ls-remote|ls-remote.*deleted|deleted.*ls-remote|STEP_COMPLETE.*step=8.*ls-remote|ls-remote.*STEP_COMPLETE' "$file"
  [ "$status" -eq 0 ]
}

@test "pr-manager step-8: includes force-delete fallback via github-ops" {
  # If ls-remote is non-empty after the bounded retry, pr-manager must
  # dispatch github-ops to force-delete: git push origin --delete <branch>.
  local file="$PLUGIN_ROOT/agents/pr-manager.md"
  run grep -F 'push origin --delete' "$file"
  [ "$status" -eq 0 ]
}

@test "pr-manager step-8: bounded retry before force-delete" {
  # The verification must include a bounded retry (not an infinite loop)
  # to absorb GitHub's own queued/async deletion before force-deleting.
  # The retry must be in the context of the branch-deletion verification,
  # so we check for the combination of ls-remote and retry/bounded language.
  local file="$PLUGIN_ROOT/agents/pr-manager.md"
  # ls-remote must appear (from earlier test), and there must be a bounded
  # retry described in the step-8 block (cap/up to 3/bounded/retry N times).
  run grep -E 'up to.*[0-9].*time|[0-9].*time.*retry|bounded retry|retry.*up to|cap.*retr' "$file"
  [ "$status" -eq 0 ]
}

@test "pr-manager step-9: post-merge wording reflects verified deletion" {
  # Step 9 wording must say the remote branch is *verified* deleted,
  # not merely assumed deleted by --delete-branch.
  local file="$PLUGIN_ROOT/agents/pr-manager.md"
  run grep -E 'verified|confirmed|ls-remote' "$file"
  [ "$status" -eq 0 ]
}

@test "code-delivery skill: remote branch deletion uses ls-remote verification" {
  # skills/code-delivery/SKILL.md Step 10 must not claim --delete-branch
  # alone handles deletion; it must document the ls-remote verify step.
  local file="$PLUGIN_ROOT/skills/code-delivery/SKILL.md"
  run grep -F 'git ls-remote' "$file"
  [ "$status" -eq 0 ]
}

@test "fix-pr-delivery skill: merge step includes ls-remote verification" {
  # skills/fix-pr-delivery/SKILL.md merge step must include the post-merge
  # ls-remote verification note (sibling-sweep TD-VSDD-060).
  local file="$PLUGIN_ROOT/skills/fix-pr-delivery/SKILL.md"
  run grep -F 'git ls-remote' "$file"
  [ "$status" -eq 0 ]
}

@test "code-delivery workflow: merge step includes ls-remote verification" {
  # workflows/code-delivery.lobster merge-pr task must include the
  # ls-remote verify-and-reconcile clause (sibling-sweep TD-VSDD-060).
  local file="$PLUGIN_ROOT/workflows/code-delivery.lobster"
  run grep -F 'ls-remote' "$file"
  [ "$status" -eq 0 ]
}

@test "greenfield workflow: merge step includes ls-remote verification" {
  # workflows/greenfield.lobster merge-pr task must include the
  # ls-remote verify-and-reconcile clause (sibling-sweep TD-VSDD-060).
  local file="$PLUGIN_ROOT/workflows/greenfield.lobster"
  run grep -F 'ls-remote' "$file"
  [ "$status" -eq 0 ]
}

# ========================================================================
# pr-manager Step 8 adversary hardening: 6 behaviors (issue #128 round 2)
# ========================================================================
# RED GATE: these tests MUST fail against the unmodified pr-manager.md
# before the Step (Green) fixes are applied.

@test "pr-manager step-8: instructs wait/sleep between re-checks" {
  # Each re-check in the bounded retry loop must include an explicit
  # wait/sleep instruction (e.g. "wait 5 seconds", "sleep 5–10 seconds")
  # so the agent absorbs GitHub's async deletion latency instead of
  # firing three re-checks in milliseconds (adversary finding 1).
  # The instruction must appear in the branch-deletion context: near a
  # mention of "second(s)" paired with a delay intent — "wait N s" or
  # "sleep N s" or "5 seconds between" etc.
  local file="$PLUGIN_ROOT/agents/pr-manager.md"
  run grep -iE '(wait|sleep).*([0-9]+.?second|second.*[0-9]+)|([0-9]+.?second|second.*[0-9]+).*(wait|sleep|between|re.?check)' "$file"
  [ "$status" -eq 0 ]
}

@test "pr-manager step-8: detects cross-repo fork PRs before verifying deletion" {
  # For fork PRs the head branch lives on the contributor's fork remote,
  # not on origin. Checking git ls-remote origin gives a false "deleted".
  # pr-manager must instruct: determine whether the PR is cross-repo
  # (e.g. gh pr view --json isCrossRepository) and SKIP origin
  # branch-deletion verification for fork PRs (adversary finding 2).
  local file="$PLUGIN_ROOT/agents/pr-manager.md"
  run grep -iE 'fork|isCrossRepository|cross.?repo|cross-repository' "$file"
  [ "$status" -eq 0 ]
}

@test "pr-manager step-8: confirms PR merged state before force-deleting branch" {
  # If a PR is queued in a GitHub Merge Queue, gh pr merge returns success
  # while the merge is still in-flight. Force-deleting the branch at that
  # point breaks the queue run. pr-manager must confirm the PR state is
  # MERGED (e.g. gh pr view --json state → MERGED) before force-deleting
  # and skip force-delete if the PR is queued (adversary finding 3).
  local file="$PLUGIN_ROOT/agents/pr-manager.md"
  run grep -iE 'merge.?queue|MERGED|--json state|gh pr view.*state|confirm.*merge|merge.*confirm' "$file"
  [ "$status" -eq 0 ]
  # Specifically check that force-delete is guarded by a merged-state check
  run grep -iE '(MERGED|merged state|merge queue).*(force.?delete|push.*delete)|force.?delete.*(MERGED|merged state|merge queue)' "$file"
  [ "$status" -eq 0 ]
}

@test "pr-manager step-8: treats already-deleted branch as success (idempotent)" {
  # git push origin --delete returns non-zero if the branch was already
  # deleted (race between ls-remote check and the push). The agent must
  # be instructed to treat "remote ref does not exist" / already-gone as
  # SUCCESS, not a workflow failure (adversary finding 4).
  local file="$PLUGIN_ROOT/agents/pr-manager.md"
  run grep -iE 'already.?gone|already.?deleted|remote ref does not exist|idempotent|non.?zero.*already|already.*success' "$file"
  [ "$status" -eq 0 ]
}

@test "pr-manager step-8: uses exact-ref match not prefix match for ls-remote" {
  # git ls-remote origin refs/heads/<branch> performs prefix matching:
  # refs/heads/feat matches refs/heads/feat-2. The playbook must instruct
  # an exact-ref check — either git ls-remote --exit-code origin
  # refs/heads/<branch> (exit code 2 = not found) or an exact-line match
  # against the returned ref string (adversary finding 5).
  local file="$PLUGIN_ROOT/agents/pr-manager.md"
  run grep -iE '\-\-exit-code|exact.?ref|exact.?match|exact.?line|exact ref' "$file"
  [ "$status" -eq 0 ]
}

@test "pr-manager step-8: logs warning and proceeds on branch-protection rejection" {
  # If branch-protection or permissions prevent deletion, the current prose
  # emits BLOCKED and stops the whole delivery. Instead the agent must log
  # a WARNING and PROCEED — the branch-leak is surfaced but not fatal
  # (adversary finding 6).
  local file="$PLUGIN_ROOT/agents/pr-manager.md"
  # Must NOT dead-end with BLOCKED; must emit a warning and continue.
  run grep -iE 'warn.*proceed|proceed.*warn|log.*warn|warning.*proceed|protection.*warn|warn.*protection' "$file"
  [ "$status" -eq 0 ]
}

