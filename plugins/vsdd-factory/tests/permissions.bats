#!/usr/bin/env bats
# permissions.bats — regression tests for agent permission model
#
# The permission model is a security boundary: spec producers must NOT have
# shell access, code producers MUST have shell access, and coordinators must
# NOT write files. Changes to these boundaries require deliberate review.
#
# See FACTORY.md "Agent Permission Model" for the full rationale.

setup() {
  AGENTS="${BATS_TEST_DIRNAME}/../agents"
}

# ---------- Spec producers: coding profile, no exec ----------

@test "product-owner has coding profile" {
  grep -q 'Profile: `coding`' "$AGENTS/product-owner.md"
}

@test "product-owner denied exec" {
  grep -q 'Denied:.*exec' "$AGENTS/product-owner.md"
}

@test "story-writer has coding profile" {
  grep -q 'Profile: `coding`' "$AGENTS/story-writer.md"
}

@test "story-writer denied exec" {
  grep -q 'Denied:.*exec' "$AGENTS/story-writer.md"
}

@test "architect has coding profile" {
  grep -q 'Profile: `coding`' "$AGENTS/architect.md"
}

@test "architect denied exec" {
  grep -q 'Denied:.*exec' "$AGENTS/architect.md"
}

# ---------- Code producers: full profile ----------

@test "implementer has full profile" {
  grep -q 'Profile: `full`' "$AGENTS/implementer.md"
}

@test "implementer not denied exec" {
  ! grep -q 'Denied:.*exec' "$AGENTS/implementer.md"
}

@test "test-writer has full profile" {
  grep -q 'Profile: `full`' "$AGENTS/test-writer.md"
}

@test "test-writer not denied exec" {
  ! grep -q 'Denied:.*exec' "$AGENTS/test-writer.md"
}

# ---------- Infrastructure: full profile ----------

@test "devops-engineer has full profile" {
  grep -q 'Profile: `full`' "$AGENTS/devops-engineer.md"
}

@test "state-manager has full profile" {
  grep -q 'Profile: `full`' "$AGENTS/state-manager.md"
}

# ---------- Coordinators: no write, no exec ----------

@test "orchestrator denied write" {
  grep -q 'Denied:.*write' "$AGENTS/orchestrator/orchestrator.md"
}

@test "orchestrator denied exec" {
  grep -q 'Denied:.*exec' "$AGENTS/orchestrator/orchestrator.md"
}

@test "pr-manager has coding profile" {
  grep -q 'Profile: `coding`' "$AGENTS/pr-manager.md"
}

@test "pr-manager denied exec" {
  grep -q 'Denied:.*exec' "$AGENTS/pr-manager.md"
}

# ---------- Reviewers: coding or read-only, no exec ----------

@test "adversary has read-only tools in frontmatter" {
  grep -q 'tools:.*Read' "$AGENTS/adversary.md"
  ! grep -q 'tools:.*Write\|tools:.*Edit\|tools:.*Bash\|tools:.*exec' "$AGENTS/adversary.md"
}

@test "consistency-validator has coding profile" {
  grep -q 'Profile: `coding`' "$AGENTS/consistency-validator.md"
}

@test "consistency-validator denied exec" {
  grep -q 'Denied:.*exec' "$AGENTS/consistency-validator.md"
}

@test "code-reviewer has coding profile" {
  grep -q 'Profile: `coding`' "$AGENTS/code-reviewer.md"
}

@test "code-reviewer denied exec" {
  grep -q 'Denied:.*exec' "$AGENTS/code-reviewer.md"
}

@test "pr-reviewer has coding profile" {
  grep -q 'Profile: `coding`' "$AGENTS/pr-reviewer.md"
}

@test "pr-reviewer denied exec" {
  grep -q 'Denied:.*exec' "$AGENTS/pr-reviewer.md"
}

@test "spec-reviewer has coding profile" {
  grep -q 'Profile: `coding`' "$AGENTS/spec-reviewer.md"
}

@test "spec-reviewer denied exec" {
  grep -q 'Denied:.*exec' "$AGENTS/spec-reviewer.md"
}

@test "security-reviewer has coding profile" {
  grep -q 'Profile: `coding`' "$AGENTS/security-reviewer.md"
}

@test "security-reviewer denied exec" {
  grep -q 'Denied:.*exec' "$AGENTS/security-reviewer.md"
}

# ---------- State-manager commit boundary ----------

@test "state-manager commits .factory/ directly" {
  grep -q 'git operations' "$AGENTS/state-manager.md" || \
  grep -q 'commit.*factory' "$AGENTS/state-manager.md" || \
  grep -q 'git add' "$AGENTS/state-manager.md"
}

@test "spec producers delegate commits to state-manager" {
  grep -q 'state-manager commits' "$AGENTS/product-owner.md" || \
  grep -q 'state-manager.*commit' "$AGENTS/product-owner.md"

  grep -q 'state-manager commits' "$AGENTS/story-writer.md" || \
  grep -q 'state-manager.*commit' "$AGENTS/story-writer.md"

  grep -q 'state-manager commits' "$AGENTS/architect.md" || \
  grep -q 'state-manager.*commit' "$AGENTS/architect.md"
}

# ---------- Tool-profile coherence: no shell instructions for coding-profile agents ----------
# Agents with coding profile (no exec) must not be instructed to run shell commands.
# Checks outside of code blocks, "Do NOT" lines, and Tool Access sections.

_check_no_shell_in_codeblocks() {
  local agent_file="$1"
  local agent_name
  agent_name=$(basename "$agent_file" .md)

  # Skip agents with full profile — they can run anything
  if grep -q 'Profile: `full`' "$agent_file"; then
    return 0
  fi

  # Skip agents without a Profile line (adversary uses frontmatter tools)
  if ! grep -q 'Profile:' "$agent_file"; then
    return 0
  fi

  # Extract content inside ```bash or ``` code blocks, then check for shell
  # commands the agent would need exec to run. This avoids false positives
  # from prose mentions of tool names.
  local violations
  violations=$(awk '/^```(bash|sh|shell)?$/,/^```$/' "$agent_file" \
    | grep -nE '^\s*(npx |cargo |npm |git (add|commit|push|checkout|worktree) |curl |bash )' \
    | grep -viE '(NEVER|NOT|Cannot|Denied|Do not|#)' || true)

  if [ -n "$violations" ]; then
    echo "FAIL: $agent_name has coding profile but code blocks contain shell commands:"
    echo "$violations"
    return 1
  fi
  return 0
}

@test "coding-profile agents have no shell commands in code blocks" {
  fail=0
  for f in "$AGENTS"/*.md; do
    if ! _check_no_shell_in_codeblocks "$f"; then
      fail=$((fail+1))
    fi
  done
  [ "$fail" -eq 0 ]
}

_check_no_inline_shell() {
  local agent_file="$1"
  local agent_name
  agent_name=$(basename "$agent_file" .md)

  # Skip agents with full profile or no Profile line
  if grep -q 'Profile: `full`' "$agent_file"; then return 0; fi
  if ! grep -q 'Profile:' "$agent_file"; then return 0; fi

  # Match inline backtick commands: `npx ...`, `cargo ...`, `git add ...`, etc.
  # These are structured instructions, not prose tool mentions.
  # Exclude lines inside fenced code blocks (already checked above).
  # Exclude negation lines (NEVER, NOT, Cannot, Denied, Do not).
  local violations
  violations=$(awk '
    /^```/ { in_block = !in_block; next }
    !in_block { print NR": "$0 }
  ' "$agent_file" \
    | grep -E '`(npx |cargo |npm run |git (add|commit|push|checkout|worktree) |curl |bash -c)' \
    | grep -viE '(NEVER|NOT|Cannot|Denied|Do not)' || true)

  if [ -n "$violations" ]; then
    echo "FAIL: $agent_name has coding profile but inline backtick shell commands:"
    echo "$violations"
    return 1
  fi
  return 0
}

@test "coding-profile agents have no inline backtick shell commands" {
  fail=0
  for f in "$AGENTS"/*.md; do
    if ! _check_no_inline_shell "$f"; then
      fail=$((fail+1))
    fi
  done
  [ "$fail" -eq 0 ]
}

# ---------- Tool-profile coherence: full-profile agents that run shell tools say so ----------

@test "accessibility-auditor has full profile (needs shell for axe/lighthouse)" {
  grep -q 'Profile: `full`' "$AGENTS/accessibility-auditor.md"
}

# ---------- Governance policy presence tests ----------
# Verify each of the 8 governance policies is present in the correct agents.

# Policy 1: append_only_numbering
@test "policy: append_only_numbering in product-owner" {
  grep -q 'append_only_numbering' "$AGENTS/product-owner.md"
}

@test "policy: append_only_numbering in spec-steward" {
  grep -q 'append_only_numbering' "$AGENTS/spec-steward.md"
}

@test "policy: append_only_numbering in FACTORY.md" {
  grep -q 'append_only_numbering' "${BATS_TEST_DIRNAME}/../docs/FACTORY.md"
}

# Policy 2: lift_invariants_to_bcs
@test "policy: lift_invariants_to_bcs in product-owner" {
  grep -q 'lift_invariants_to_bcs' "$AGENTS/product-owner.md"
}

@test "policy: lift_invariants_to_bcs in adversary (review axis)" {
  grep -q 'Invariant-to-BC Orphan Detection' "$AGENTS/adversary.md"
}

@test "policy: lift_invariants_to_bcs criterion 74 in consistency-validator" {
  grep -q 'Every domain invariant (DI-NNN) cited by at least one BC' "$AGENTS/consistency-validator.md"
}

# Policy 3: state_manager_runs_last
@test "policy: state_manager_runs_last in orchestrator" {
  grep -q 'state-manager LAST' "$AGENTS/orchestrator/orchestrator.md"
}

# Policy 4: semantic_anchoring_integrity
@test "policy: semantic_anchoring_integrity in adversary" {
  grep -q 'Semantic Anchoring Audit' "$AGENTS/adversary.md"
}

@test "policy: semantic_anchoring_integrity criteria 70-73 in consistency-validator" {
  grep -q 'Semantic Anchoring Integrity (Criteria 70-73)' "$AGENTS/consistency-validator.md"
}

# Policy 5: creators_justify_anchors
@test "policy: creators_justify_anchors in product-owner" {
  grep -q 'Anchor Justification Requirement' "$AGENTS/product-owner.md"
}

@test "policy: creators_justify_anchors in architect" {
  grep -q 'Anchor Justification Requirement' "$AGENTS/architect.md"
}

@test "policy: creators_justify_anchors in story-writer" {
  grep -q 'Anchor Justification Requirement' "$AGENTS/story-writer.md"
}

@test "policy: creators_justify_anchors in business-analyst" {
  grep -q 'Anchor Justification Requirement' "$AGENTS/business-analyst.md"
}

# Policy 6: architecture_is_subsystem_name_source_of_truth
@test "policy: subsystem SOT in product-owner" {
  grep -q 'architecture_is_subsystem_name_source_of_truth' "$AGENTS/product-owner.md"
}

@test "policy: subsystem SOT criterion 76 in consistency-validator" {
  grep -q 'BC subsystem labels match ARCH-INDEX' "$AGENTS/consistency-validator.md"
}

# Policy 7: bc_h1_is_title_source_of_truth
@test "policy: bc_h1 SOT in product-owner" {
  grep -q 'bc_h1_is_title_source_of_truth' "$AGENTS/product-owner.md"
}

@test "policy: bc_h1 SOT criterion 75 in consistency-validator" {
  grep -q 'BC file H1 heading matches BC-INDEX title' "$AGENTS/consistency-validator.md"
}

@test "policy: bc_h1 SOT review axis in adversary" {
  grep -q 'BC Title and Subsystem Label Sync' "$AGENTS/adversary.md"
}

# Policy 8: bc_array_changes_propagate_to_body_and_acs
@test "policy: bc_array_propagation in story-writer" {
  grep -q 'bc_array_changes_propagate_to_body_and_acs' "$AGENTS/story-writer.md"
}

@test "policy: bc_array_propagation in product-owner" {
  grep -q 'bc_array_changes_propagate_to_body_and_acs' "$AGENTS/product-owner.md"
}

@test "policy: bc_array_propagation criteria 67-69 in consistency-validator" {
  grep -q 'Story Frontmatter-Body BC Coherence (Criteria 67-69)' "$AGENTS/consistency-validator.md"
}
