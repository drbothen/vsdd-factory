#!/bin/bash
# capture-commit-activity.sh — PostToolUse hook on Bash.
#
# Watches for `git commit` invocations and emits a structured commit.made
# event when they succeed. Motivated by a discovery in v0.77.0: the Claude
# Code OTel metric `claude_code.commit.count` that the Factory ROI
# "Cost per commit" panel depended on was documented upstream but never
# actually emitted in our sessions — the panel read N/A from v0.74.0
# through v0.76.1. Owning the signal ourselves (matching what we did for
# PRs in v0.73.1 with capture-pr-activity) puts us back in control.
#
# Emitted event:
#   type=commit.made  hook=capture-commit-activity  matcher=Bash
#     commit_sha        (from `[<branch> <sha>]` in stdout)
#     branch            (same source)
#     message_subject   (stdout's first line, if parseable)
#     amended           "true" when the command included --amend
#
# `--amend` commits ARE emitted but flagged via `amended="true"` so the
# dashboard can choose to include or exclude them. Amends typically
# mutate the previous commit, so for a pure "new commits shipped" count
# the ROI panel filters them out; other analyses might want them.
#
# Failed commits (non-zero exit) are no-ops — a pre-commit hook that
# rejected the commit should not count against cost-per-commit.
#
# Exit 0 on every path — advisory, never blocks.
#
# Trigger: PostToolUse on Bash.
# Deterministic, <50ms, no LLM.

set +e

if ! command -v jq >/dev/null 2>&1; then
  exit 0
fi

INPUT=$(cat)

TOOL=$(echo "$INPUT" | jq -r '.tool_name // ""')
if [[ "$TOOL" != "Bash" ]]; then
  exit 0
fi

COMMAND=$(echo "$INPUT" | jq -r '.tool_input.command // ""')
EXIT_CODE=$(echo "$INPUT" | jq -r '.tool_response.exit_code // -1')
STDOUT=$(echo "$INPUT" | jq -r '.tool_response.stdout // ""')

if [[ "$EXIT_CODE" != "0" ]]; then
  exit 0
fi

# Match `git commit` as a real subcommand (not inside an echo / comment /
# string literal). Anchored on word boundaries the same way capture-pr-activity
# anchors its gh matchers. Excludes `git commit-tree`, `git committed-foo`, etc.
if [[ ! "$COMMAND" =~ (^|[;&|[:space:]])git[[:space:]]+commit([[:space:]]|$) ]]; then
  exit 0
fi

# Parse the `[<branch> <sha>]` preamble that `git commit` prints on success.
# Format varies slightly:
#   [main abc1234] commit message     ← normal
#   [main (root-commit) abc1234] …    ← first commit in a repo
#   [HEAD detached at abc123 def4567] …  ← detached HEAD
# We only emit when we can extract a sha — otherwise the command probably
# didn't actually make a commit (e.g., `git commit --dry-run`, pre-commit
# hook aborted without exiting non-zero, nothing staged).
#
# The sha is the LAST whitespace-separated token inside the brackets.
FIRST_LINE=$(printf '%s\n' "$STDOUT" | head -n 1)
if [[ ! "$FIRST_LINE" =~ ^\[([^]]+)\][[:space:]]?(.*)$ ]]; then
  exit 0
fi
BRACKET_CONTENTS="${BASH_REMATCH[1]}"
MESSAGE_SUBJECT="${BASH_REMATCH[2]}"

# Tokenize the bracket contents; sha is the last token, branch is the first.
# shellcheck disable=SC2206
BRACKET_TOKENS=($BRACKET_CONTENTS)
BRACKET_LAST_IDX=$((${#BRACKET_TOKENS[@]} - 1))
if [ "$BRACKET_LAST_IDX" -lt 0 ]; then
  exit 0
fi
COMMIT_SHA="${BRACKET_TOKENS[$BRACKET_LAST_IDX]}"
BRANCH="${BRACKET_TOKENS[0]}"

# Sanity: sha must be 7+ hex chars. Protects against parsing garbage when
# someone pipes weird text into stdout that also starts with [.
if [[ ! "$COMMIT_SHA" =~ ^[0-9a-f]{7,40}$ ]]; then
  exit 0
fi

# Detect --amend anywhere in the command. It can appear as --amend, --amend=,
# or in combined short-form contexts (there isn't a short form for --amend,
# but we still match loosely).
AMENDED=""
if [[ "$COMMAND" =~ (^|[[:space:]])--amend([[:space:]=]|$) ]]; then
  AMENDED="true"
fi

_emit() {
  if [ -n "${CLAUDE_PLUGIN_ROOT:-}" ] && [ -x "${CLAUDE_PLUGIN_ROOT}/bin/emit-event" ]; then
    "${CLAUDE_PLUGIN_ROOT}/bin/emit-event" "$@" 2>/dev/null || true
  fi
  return 0
}

_emit type=commit.made hook=capture-commit-activity matcher=Bash \
      commit_sha="$COMMIT_SHA" \
      branch="$BRANCH" \
      ${MESSAGE_SUBJECT:+message_subject="$MESSAGE_SUBJECT"} \
      ${AMENDED:+amended="$AMENDED"}

exit 0
