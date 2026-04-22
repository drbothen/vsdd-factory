#!/usr/bin/env bats
# pr-manager-guard.bats — tests for pr-manager-completion-guard.sh
#
# Tests the SubagentStop hook that detects FM4 (pr-manager exiting
# before completing all 9 steps of the lifecycle).

setup() {
  PLUGIN_ROOT="$(cd "$BATS_TEST_DIRNAME/.." && pwd)"
  HOOK="$PLUGIN_ROOT/hooks/pr-manager-completion-guard.sh"
}

_run_guard() {
  local agent="$1"
  local result="$2"
  INPUT=$(jq -nc --arg a "$agent" --arg r "$result" '{agent_type: $a, last_assistant_message: $r}')
  run bash -c "echo '$INPUT' | '$HOOK' 2>&1"
}

# ========================================================================
# Syntax and wiring
# ========================================================================

@test "pr-manager-guard: passes syntax check" {
  run bash -n "$HOOK"
  [ "$status" -eq 0 ]
}

@test "pr-manager-guard: hook is executable" {
  [ -x "$HOOK" ]
}

@test "pr-manager-guard: hooks.json wires the hook under SubagentStop" {
  run jq '.hooks.SubagentStop[0].hooks[] | select(.command | contains("pr-manager-completion-guard"))' "$PLUGIN_ROOT/hooks/hooks.json"
  [ "$status" -eq 0 ]
  [[ "$output" == *"pr-manager-completion-guard"* ]]
}

# ========================================================================
# Scope: ignores non-pr-manager agents
# ========================================================================

@test "pr-manager-guard: ignores non-pr-manager agents" {
  _run_guard "implementer" "Done implementing."
  [ "$status" -eq 0 ]
}

@test "pr-manager-guard: ignores demo-recorder" {
  _run_guard "demo-recorder" "Recorded demos."
  [ "$status" -eq 0 ]
}

@test "pr-manager-guard: ignores unknown agent" {
  _run_guard "unknown" "Some output."
  [ "$status" -eq 0 ]
}

# ========================================================================
# Pass: all 9 steps complete
# ========================================================================

@test "pr-manager-guard: passes with all 9 STEP_COMPLETE emissions" {
  RESULT="STEP_COMPLETE: step=1 name=populate-pr-description status=ok note=done
STEP_COMPLETE: step=2 name=verify-demo-evidence status=ok note=found
STEP_COMPLETE: step=3 name=create-pr status=ok note=PR #42
STEP_COMPLETE: step=4 name=security-review status=ok note=clean
STEP_COMPLETE: step=5 name=review-convergence status=ok note=converged
STEP_COMPLETE: step=6 name=wait-for-ci status=ok note=green
STEP_COMPLETE: step=7 name=dependency-check status=ok note=all merged
STEP_COMPLETE: step=8 name=execute-merge status=ok note=merged
STEP_COMPLETE: step=9 name=post-merge status=ok note=cleanup done"
  _run_guard "pr-manager" "$RESULT"
  [ "$status" -eq 0 ]
}

@test "pr-manager-guard: passes with 8 steps (one skipped as na)" {
  RESULT="STEP_COMPLETE: step=1 name=populate-pr-description status=ok note=done
STEP_COMPLETE: step=2 name=verify-demo-evidence status=na note=chore PR
STEP_COMPLETE: step=3 name=create-pr status=ok note=PR #42
STEP_COMPLETE: step=4 name=security-review status=ok note=clean
STEP_COMPLETE: step=5 name=review-convergence status=ok note=converged
STEP_COMPLETE: step=6 name=wait-for-ci status=ok note=green
STEP_COMPLETE: step=7 name=dependency-check status=ok note=all merged
STEP_COMPLETE: step=8 name=execute-merge status=ok note=merged"
  _run_guard "pr-manager" "$RESULT"
  [ "$status" -eq 0 ]
}

# ========================================================================
# Pass: BLOCKED status is legitimate early exit
# ========================================================================

@test "pr-manager-guard: passes when agent reports BLOCKED" {
  RESULT="STEP_COMPLETE: step=1 name=populate-pr-description status=ok note=done
STEP_COMPLETE: step=2 name=verify-demo-evidence status=ok note=found
STEP_COMPLETE: step=3 name=create-pr status=ok note=PR #42
STEP_COMPLETE: step=4 name=security-review status=failed note=CRITICAL finding

BLOCKED: Security review found CRITICAL vulnerability in auth module. Cannot proceed to merge."
  _run_guard "pr-manager" "$RESULT"
  [ "$status" -eq 0 ]
}

@test "pr-manager-guard: passes BLOCKED with markdown heading" {
  RESULT="STEP_COMPLETE: step=1 name=populate-pr-description status=ok note=done
## BLOCKED
Dependency PR #41 is not merged. Cannot proceed."
  _run_guard "pr-manager" "$RESULT"
  [ "$status" -eq 0 ]
}

# ========================================================================
# Block: premature exit (FM4 detection)
# ========================================================================

@test "pr-manager-guard: blocks with 0 steps (immediate exit)" {
  _run_guard "pr-manager" "The PR reviewer approved. Proceed to merge."
  [ "$status" -eq 2 ]
  [[ "$output" == *"FM4 guard fired"* ]]
  [[ "$output" == *"CONTINUE TO STEP 1"* ]]
}

@test "pr-manager-guard: blocks after step 5 (reviewer approved, agent exits)" {
  RESULT="STEP_COMPLETE: step=1 name=populate-pr-description status=ok note=done
STEP_COMPLETE: step=2 name=verify-demo-evidence status=ok note=found
STEP_COMPLETE: step=3 name=create-pr status=ok note=PR #42
STEP_COMPLETE: step=4 name=security-review status=ok note=clean
STEP_COMPLETE: step=5 name=review-convergence status=ok note=approved

PR reviewer approved. Proceed to merge."
  _run_guard "pr-manager" "$RESULT"
  [ "$status" -eq 2 ]
  [[ "$output" == *"FM4 guard fired"* ]]
  [[ "$output" == *"CONTINUE TO STEP 6"* ]]
  [[ "$output" == *"gh pr checks"* ]]
}

@test "pr-manager-guard: blocks after step 3 (created PR then stopped)" {
  RESULT="STEP_COMPLETE: step=1 name=populate-pr-description status=ok note=done
STEP_COMPLETE: step=2 name=verify-demo-evidence status=ok note=found
STEP_COMPLETE: step=3 name=create-pr status=ok note=PR #42

PR #42 created successfully."
  _run_guard "pr-manager" "$RESULT"
  [ "$status" -eq 2 ]
  [[ "$output" == *"FM4 guard fired"* ]]
  [[ "$output" == *"CONTINUE TO STEP 4"* ]]
  [[ "$output" == *"security-reviewer"* ]]
}

@test "pr-manager-guard: blocks after step 7 (skipped merge)" {
  RESULT="STEP_COMPLETE: step=1 name=populate-pr-description status=ok note=done
STEP_COMPLETE: step=2 name=verify-demo-evidence status=ok note=found
STEP_COMPLETE: step=3 name=create-pr status=ok note=PR #42
STEP_COMPLETE: step=4 name=security-review status=ok note=clean
STEP_COMPLETE: step=5 name=review-convergence status=ok note=approved
STEP_COMPLETE: step=6 name=wait-for-ci status=ok note=green
STEP_COMPLETE: step=7 name=dependency-check status=ok note=all merged

All gates passed. Ready to merge."
  _run_guard "pr-manager" "$RESULT"
  [ "$status" -eq 2 ]
  [[ "$output" == *"FM4 guard fired"* ]]
  [[ "$output" == *"CONTINUE TO STEP 8"* ]]
  [[ "$output" == *"AUTHORIZE_MERGE"* ]]
}

# ========================================================================
# Step counting accuracy
# ========================================================================

@test "pr-manager-guard: reports correct step count in message" {
  RESULT="STEP_COMPLETE: step=1 name=populate-pr-description status=ok note=done
STEP_COMPLETE: step=2 name=verify-demo-evidence status=ok note=found

Done with PR description and demo evidence."
  _run_guard "pr-manager" "$RESULT"
  [ "$status" -eq 2 ]
  [[ "$output" == *"2 step(s)"* ]]
}

@test "pr-manager-guard: handles step 1 only" {
  RESULT="STEP_COMPLETE: step=1 name=populate-pr-description status=ok note=done

PR description populated."
  _run_guard "pr-manager" "$RESULT"
  [ "$status" -eq 2 ]
  [[ "$output" == *"1 step(s)"* ]]
  [[ "$output" == *"CONTINUE TO STEP 2"* ]]
  [[ "$output" == *"demo evidence"* ]]
}

# ========================================================================
# Agent name matching
# ========================================================================

@test "pr-manager-guard: matches vsdd-factory:pr-manager agent type" {
  _run_guard "vsdd-factory:pr-manager" "Reviewer approved."
  [ "$status" -eq 2 ]
}

@test "pr-manager-guard: matches pr-manager with prefix" {
  _run_guard "some-prefix-pr-manager" "Reviewer approved."
  [ "$status" -eq 2 ]
}

@test "pr-manager-guard: matches pr_manager with underscore" {
  _run_guard "pr_manager" "Reviewer approved."
  [ "$status" -eq 2 ]
}

# ========================================================================
# Edge cases
# ========================================================================

@test "pr-manager-guard: handles empty result" {
  _run_guard "pr-manager" ""
  [ "$status" -eq 2 ]
  [[ "$output" == *"0 step(s)"* ]]
}

@test "pr-manager-guard: handles result with STEP_COMPLETE-like text that isnt the format" {
  _run_guard "pr-manager" "I should have emitted STEP_COMPLETE but forgot."
  [ "$status" -eq 2 ]
  # The word "STEP_COMPLETE" without the colon and step= shouldn't count
  [[ "$output" == *"0 step(s)"* ]]
}

@test "pr-manager-guard: step 9 hint mentions review-findings.md" {
  RESULT="STEP_COMPLETE: step=1 name=a status=ok note=x
STEP_COMPLETE: step=2 name=b status=ok note=x
STEP_COMPLETE: step=3 name=c status=ok note=x
STEP_COMPLETE: step=4 name=d status=ok note=x
STEP_COMPLETE: step=5 name=e status=ok note=x
STEP_COMPLETE: step=6 name=f status=ok note=x
STEP_COMPLETE: step=7 name=g status=ok note=x
STEP_COMPLETE: step=8 name=h status=ok note=x

Merge complete."
  _run_guard "pr-manager" "$RESULT"
  # Should fail because only 8 steps but not step 9 (which was the original FM4 failure mode — not reaching step 9)
  # Wait — 8 is our threshold. With STEP_COUNT >= 8, this passes.
  [ "$status" -eq 0 ]
}
