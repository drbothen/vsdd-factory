#!/usr/bin/env bats
# registry-tool-filter-anchoring.bats — AC-005 lint tests for hooks-registry.toml
# tool-filter regex anchoring convention (S-19.04).
#
# Tests every tool= line in hooks-registry.toml for AC-004 compliance:
# either (a) anchored pattern starting with ^ or (b) carries an # intent:
# inline comment explaining intentional unanchored use (EC-001).
#
# Test Plan (story S-19.04):
#   T-001  AC-004/AC-005  Fixture unanchored entry → lint detects violation
#   T-002  AC-004/AC-005  Fixture anchored entry → lint clean
#   T-003  AC-005         Intent-comment entry → lint clean (EC-001 exemption)
#   T-004  AC-004/AC-005  Actual hooks-registry.toml post-fix → lint clean (FAILS at Red Gate)
#   T-005  AC-004         verify-factory-lock pattern is fully anchored (^ AND $) + includes MultiEdit (FAILS at Red Gate)
#   T-011  AC-004/AC-005  Prefix-only fixture (^Bash, no trailing $) → lint detects violation (negative control)
#   T-012  AC-004/AC-005  Comment-injection fixture (^Edit" # note "$") → lint detects violation (greedy-dot bypass class)
#
# Red Gate status:
#   T-001..T-003  PASS (test the detection algorithm against controlled fixtures)
#   T-004         FAIL (actual registry still has unanchored entries pre-implementation)
#   T-005         FAIL (verify-factory-lock tool value not yet anchored pre-implementation)
#
# VP Trace: VP-099
# Story: S-19.04

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../.." && pwd)"
  REGISTRY="$REPO_ROOT/plugins/vsdd-factory/hooks-registry.toml"
  FIXTURES_DIR="$BATS_TEST_DIRNAME/fixtures/registry-tool-filter"
}

# ---------------------------------------------------------------------------
# AC-004 lint logic (inline).
# Extracts tool= values that are NOT fully anchored (missing leading ^ OR
# trailing $ before the closing quote) and carry no # intent: comment.
# Empty output = all entries compliant.
#
# FULLY-ANCHORED means: value begins with ^ AND ends with $ immediately
# before the closing quote (either " or ').  A leading-^-only value such
# as "^Bash" is a violation — it anchors the start but not the end, so
# regex-SEARCH still matches "BashAsync" as a substring.
#
# Implements the gate from AC-004:
#   grep -E '^tool = ' <file> \
#     | grep -vE 'tool = ["'"'"']\^[^"'"'"']*\$["'"'"']' \
#     | grep -vi '# *intent:'
#
# The trailing "|| true" prevents the pipeline from propagating a non-zero
# exit code when grep -v finds no lines (all entries are fully anchored/exempted).
# grep -v returns exit code 1 for "no lines matched", which is the success
# case here; we must not let that propagate as a test failure.
#
# Usage: result=$(_lint_tool_filter <registry-file>)
#        [ -z "$result" ]   # passes when all entries are fully anchored or intent-commented
# ---------------------------------------------------------------------------
_lint_tool_filter() {
  local toml="$1"
  grep -E '^tool = ' "$toml" \
    | grep -vE 'tool = ["'"'"']\^[^"'"'"']*\$["'"'"']' \
    | grep -vi '# *intent:' \
    || true
}

# ---------------------------------------------------------------------------
# T-001  AC-004/AC-005
# Fixture with unanchored tool= entry must be detected by lint.
# RG-001: exit with non-empty output (lint fails for unanchored entry).
# ---------------------------------------------------------------------------
@test "T-001 AC-004/AC-005: unanchored fixture entry detected by lint" {
  local fixture="$FIXTURES_DIR/unanchored.toml"
  [ -f "$fixture" ]
  result=$(_lint_tool_filter "$fixture")
  # Lint must produce non-empty output — unanchored entry is a violation.
  [ -n "$result" ]
}

# ---------------------------------------------------------------------------
# T-002  AC-004/AC-005
# Fixture with anchored tool= entry must pass lint (empty output).
# RG-002: exit 0 (anchored entry passes).
# ---------------------------------------------------------------------------
@test "T-002 AC-004/AC-005: anchored fixture entry passes lint" {
  local fixture="$FIXTURES_DIR/anchored.toml"
  [ -f "$fixture" ]
  result=$(_lint_tool_filter "$fixture")
  # Lint must produce empty output — anchored entry is compliant.
  [ -z "$result" ]
}

# ---------------------------------------------------------------------------
# T-003  AC-005
# Fixture with unanchored entry that carries an # intent: inline comment
# must pass lint (EC-001 exemption).
# RG-003: exit 0 (intent-comment entry passes).
# ---------------------------------------------------------------------------
@test "T-003 AC-005: intent-comment exemption passes lint (EC-001)" {
  local fixture="$FIXTURES_DIR/intent-comment.toml"
  [ -f "$fixture" ]
  result=$(_lint_tool_filter "$fixture")
  # Lint must produce empty output — # intent: comment grants exemption.
  [ -z "$result" ]
}

# ---------------------------------------------------------------------------
# T-004  AC-004/AC-005  [FAILS AT RED GATE]
# Actual hooks-registry.toml must pass the lint (zero unanchored entries)
# after the implementer anchors all tool= values per the D-a table.
#
# Red Gate failure: current registry contains unanchored tool= entries
# (e.g., tool = "Edit|Write"), so _lint_tool_filter returns non-empty output
# and [ -z "$result" ] fails.
# ---------------------------------------------------------------------------
@test "T-004 AC-004/AC-005: actual hooks-registry.toml has no unanchored tool entries" {
  result=$(_lint_tool_filter "$REGISTRY")
  if [ -n "$result" ]; then
    echo "FAIL: unanchored tool= entries found in hooks-registry.toml:"
    echo "$result"
    false
  fi
}

# ---------------------------------------------------------------------------
# T-005  AC-004  [FAILS AT RED GATE]
# Positive-scope verification: after anchoring, verify-factory-lock's tool
# pattern must (a) start with ^ (confirming it is anchored) and (b) contain
# MultiEdit (confirming the anchoring did not narrow the intended tool set).
#
# Red Gate failure: current registry has tool = "Edit|Write|MultiEdit|Agent"
# (no leading ^), so the anchoring check (result starts with ^) fails.
# ---------------------------------------------------------------------------
@test "T-005 AC-004: verify-factory-lock tool pattern is anchored and includes MultiEdit" {
  # Extract the tool= value for the verify-factory-lock entry.
  result=$(awk '
    /^\[\[hooks\]\]/ { in_block=1; name=""; tool=""; next }
    in_block && /^name = / { gsub(/"/, "", $3); name=$3; next }
    in_block && /^tool = / { gsub(/"/, "", $3); tool=$3; next }
    in_block && /^$|^\[\[/ {
      if (name == "verify-factory-lock") { print tool; exit }
      in_block=0
    }
    END { if (name == "verify-factory-lock") print tool }
  ' "$REGISTRY")

  if [ -z "$result" ]; then
    echo "FAIL: verify-factory-lock entry not found in $REGISTRY"
    false
  fi

  # (a) Pattern must start with ^ (leading anchor).
  if [[ "$result" != ^* ]]; then
    echo "FAIL: verify-factory-lock tool pattern is not anchored (no leading ^): $result"
    false
  fi

  # (b) Pattern must end with $ before the closing quote (trailing anchor).
  # Leading-^-only values (e.g. "^Bash") still match BashAsync via regex-SEARCH;
  # only ^...$ fully-anchored patterns prevent substring matches.
  if [[ "$result" != *'$' ]]; then
    echo "FAIL: verify-factory-lock tool pattern is not fully anchored (no trailing \$): $result"
    false
  fi

  # (c) Pattern must include MultiEdit (positive-scope confirmation).
  if [[ "$result" != *MultiEdit* ]]; then
    echo "FAIL: verify-factory-lock tool pattern does not include MultiEdit: $result"
    false
  fi
}

# ---------------------------------------------------------------------------
# T-011  AC-004/AC-005 — negative control: prefix-only anchor is a violation
#
# A value like "^Bash" (leading ^ present, trailing $ absent) must be
# detected as a violation by _lint_tool_filter.  This is the load-bearing
# negative control that proves the tightened lint checks BOTH anchors, not
# just the leading one.  Without this test, a leading-^-only check silently
# passes "^Bash" even though regex-SEARCH still matches "BashAsync".
#
# Fixture: fixtures/registry-tool-filter/prefix-only-anchor.toml
#   tool = "^Bash"   (leading ^ present, trailing $ absent → violation)
# ---------------------------------------------------------------------------
@test "T-011 AC-004/AC-005: prefix-only anchor (^Bash, no trailing \$) detected as violation" {
  local fixture="$FIXTURES_DIR/prefix-only-anchor.toml"
  [ -f "$fixture" ]
  result=$(_lint_tool_filter "$fixture")
  # Lint must produce non-empty output — prefix-only anchor is a violation.
  if [ -z "$result" ]; then
    echo "FAIL: _lint_tool_filter passed 'tool = \"^Bash\"' (no trailing \$); " \
         "lint must FLAG prefix-only anchors as violations"
    false
  fi
}

# ---------------------------------------------------------------------------
# T-012  AC-004/AC-005 — greedy-dot bypass class: comment-injected $ must be flagged
#
# A value like:
#   tool = "^Edit" # note "$"
# has a TOML string value of "^Edit" (no trailing $) but a trailing comment
# containing "$".  A greedy-dot pattern (\^.*\$) would incorrectly match
# across the closing quote into the comment, falsely classifying this as
# a compliant fully-anchored entry.
#
# The [^"']* quantifier stops at the first quote character, preventing the
# match from spanning outside the string literal.  This test proves that
# the bypass class is closed.
#
# Fixture: fixtures/registry-tool-filter/comment-inject.toml
#   tool = "^Edit" # note "$"   (value lacks trailing $; comment has $ → bypass class)
# ---------------------------------------------------------------------------
@test "T-012 AC-004/AC-005: comment-injection fixture (^Edit with \$ in comment) flagged as violation" {
  local fixture="$FIXTURES_DIR/comment-inject.toml"
  [ -f "$fixture" ]
  result=$(_lint_tool_filter "$fixture")
  # Lint must produce non-empty output — trailing $ lives in the COMMENT, not
  # the string value; the entry has no trailing anchor and must be flagged.
  if [ -z "$result" ]; then
    echo "FAIL: _lint_tool_filter passed 'tool = \"^Edit\" # note \"\$\"'; " \
         "the \$ is in the comment, not the string value — greedy-dot bypass is open"
    false
  fi
}
