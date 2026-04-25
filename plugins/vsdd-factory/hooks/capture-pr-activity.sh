#!/bin/bash
# capture-pr-activity.sh — PostToolUse hook on Bash.
#
# Watches for `gh pr create` and `gh pr merge` invocations and emits
# structured pr.opened / pr.merged events when they succeed. Direct
# capture from the Bash command — not inferred from a subagent's last
# message — so it catches PRs opened or merged outside the factory's
# pr-manager workflow too.
#
# Emitted event types:
#   type=pr.opened  hook=capture-pr-activity matcher=Bash
#                   pr_url, pr_number, pr_repo, [title]
#   type=pr.merged  hook=capture-pr-activity matcher=Bash
#                   pr_url, pr_number, pr_repo, [merge_strategy]
#
# Both are hook-action-style state changes; dashboards filter them via
# the event_type label (promoted to Loki label by the collector hint
# in v0.70.2).
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
# Claude Code's `tool_response` for Bash does NOT include `exit_code`;
# it sends `interrupted`, `stdout`, `stderr`, `isImage`, `noOutputExpected`.
# Treat `interrupted: false` (or missing) as success. Fall back to
# `exit_code == 0` for back-compat with hosts that do send exit_code.
EXIT_CODE=$(echo "$INPUT" | jq -r '.tool_response.exit_code // empty')
INTERRUPTED=$(echo "$INPUT" | jq -r '.tool_response.interrupted // empty')
STDOUT=$(echo "$INPUT" | jq -r '.tool_response.stdout // ""')

# Only consider successful commands — a failed `gh pr create` didn't
# actually open a PR, and downstream dashboards shouldn't count it.
if [[ "$INTERRUPTED" == "true" ]] || [[ -n "$EXIT_CODE" && "$EXIT_CODE" != "0" ]]; then
  exit 0
fi

_emit() {
  if [ -n "${CLAUDE_PLUGIN_ROOT:-}" ] && [ -x "${CLAUDE_PLUGIN_ROOT}/bin/emit-event" ]; then
    "${CLAUDE_PLUGIN_ROOT}/bin/emit-event" "$@" 2>/dev/null || true
  fi
  return 0
}

# Resolve the log directory the same way emit-event does (v0.70+):
# explicit VSDD_LOG_DIR wins; else main worktree's .factory/logs; else cwd.
# Used only for the PR open→merge duration lookup. Failure silently returns
# the empty string — the merge event just won't get an open_to_merge_seconds
# field.
_resolve_log_dir() {
  if [ -n "${VSDD_LOG_DIR:-}" ]; then
    printf '%s' "$VSDD_LOG_DIR"
    return 0
  fi
  local main_wt
  main_wt=$(git worktree list --porcelain 2>/dev/null | awk '/^worktree /{print $2; exit}' 2>/dev/null)
  if [ -n "$main_wt" ] && [ -d "$main_wt" ]; then
    printf '%s' "${main_wt}/.factory/logs"
    return 0
  fi
  printf '%s' ".factory/logs"
}

# Given a PR number, scan the last few event files for the most recent
# pr.opened event with a matching pr_number and emit its ts_epoch on stdout.
# Prints nothing if no match. Bounded to ~7 files (week+) to cap runtime.
_find_opened_ts_epoch() {
  local pr_number="$1"
  local log_dir
  log_dir=$(_resolve_log_dir)
  if [ -z "$log_dir" ] || [ ! -d "$log_dir" ]; then
    return 0
  fi
  # Search newest files first, stop at first match.
  local match
  # shellcheck disable=SC2012
  for f in $(ls -t "$log_dir"/events-*.jsonl 2>/dev/null | head -7); do
    match=$(grep -F '"type":"pr.opened"' "$f" 2>/dev/null \
      | grep -F "\"pr_number\":\"$pr_number\"" \
      | tail -1)
    if [ -n "$match" ]; then
      echo "$match" | jq -r '.ts_epoch // empty' 2>/dev/null
      return 0
    fi
  done
}

# Extract a GitHub PR URL (format: https://github.com/<owner>/<repo>/pull/<N>)
# from either stdout or the command itself.
_extract_pr_url() {
  local text="$1"
  echo "$text" | grep -oE 'https://github\.com/[^/[:space:]]+/[^/[:space:]]+/pull/[0-9]+' | head -1
}

_pr_number_from_url() {
  echo "$1" | grep -oE '[0-9]+$'
}

_pr_repo_from_url() {
  echo "$1" | sed -E 's|https://github\.com/([^/]+/[^/]+)/pull/.+|\1|'
}

# Match `gh pr create` as a real subcommand (not inside an echo / comment).
# The regex looks for the command start-ish: beginning of line, a semicolon,
# pipe, ampersand, or whitespace, followed by `gh` `pr` `create`.
if [[ "$COMMAND" =~ (^|[;&|[:space:]])gh[[:space:]]+pr[[:space:]]+create([[:space:]]|$) ]]; then
  PR_URL=$(_extract_pr_url "$STDOUT")
  # `gh pr create` prints the URL on success. If there's no URL in stdout,
  # something unusual happened — don't emit.
  if [ -z "$PR_URL" ]; then
    exit 0
  fi
  PR_NUMBER=$(_pr_number_from_url "$PR_URL")
  PR_REPO=$(_pr_repo_from_url "$PR_URL")
  # Best-effort title extraction from `--title "..."`.
  TITLE=$(echo "$COMMAND" | grep -oE -- '--title[[:space:]]+"[^"]*"' | head -1 | sed -E 's/--title[[:space:]]+"([^"]*)"/\1/')
  _emit type=pr.opened hook=capture-pr-activity matcher=Bash \
        pr_url="$PR_URL" \
        pr_number="$PR_NUMBER" \
        pr_repo="$PR_REPO" \
        ${TITLE:+title="$TITLE"}
  exit 0
fi

# Match `gh pr merge` the same way.
if [[ "$COMMAND" =~ (^|[;&|[:space:]])gh[[:space:]]+pr[[:space:]]+merge([[:space:]]|$) ]]; then
  # Stdout first (modern gh prints URL), fall back to command args (URL form
  # or `gh pr merge <number>`).
  PR_URL=$(_extract_pr_url "$STDOUT")
  if [ -z "$PR_URL" ]; then
    PR_URL=$(_extract_pr_url "$COMMAND")
  fi

  PR_NUMBER=""
  PR_REPO=""
  if [ -n "$PR_URL" ]; then
    PR_NUMBER=$(_pr_number_from_url "$PR_URL")
    PR_REPO=$(_pr_repo_from_url "$PR_URL")
  else
    # `gh pr merge 42` form — positional PR number.
    PR_NUMBER=$(echo "$COMMAND" | grep -oE 'gh[[:space:]]+pr[[:space:]]+merge[[:space:]]+[0-9]+' | grep -oE '[0-9]+$')
  fi

  # Detect merge strategy from command flags.
  MERGE_STRATEGY=""
  case "$COMMAND" in
    *--squash*) MERGE_STRATEGY="squash" ;;
    *--rebase*) MERGE_STRATEGY="rebase" ;;
    *--merge*)  MERGE_STRATEGY="merge" ;;
  esac

  # If we can't even determine a PR number, the command probably didn't
  # merge a specific PR (e.g., `gh pr merge --help`). Don't emit.
  if [ -z "$PR_NUMBER" ]; then
    exit 0
  fi

  # Open→merge duration (v0.75+). Look back in the events file(s) for a
  # matching pr.opened event with the same pr_number. If found, include
  # the elapsed time as open_to_merge_seconds. Silent no-op on failure —
  # the pr.merged event still emits without the duration field.
  OPEN_EPOCH=$(_find_opened_ts_epoch "$PR_NUMBER")
  NOW_EPOCH=$(date +%s 2>/dev/null || true)
  DURATION_ARG=()
  if [ -n "$OPEN_EPOCH" ] && [ -n "$NOW_EPOCH" ]; then
    DURATION=$((NOW_EPOCH - OPEN_EPOCH))
    # Reject absurd values (negative or wildly large) — likely a PR
    # number collision across weeks, not a real duration.
    if [ "$DURATION" -ge 0 ] && [ "$DURATION" -lt 2592000 ]; then  # 30 days
      DURATION_ARG=(open_to_merge_seconds="$DURATION")
    fi
  fi

  _emit type=pr.merged hook=capture-pr-activity matcher=Bash \
        ${PR_URL:+pr_url="$PR_URL"} \
        pr_number="$PR_NUMBER" \
        ${PR_REPO:+pr_repo="$PR_REPO"} \
        ${MERGE_STRATEGY:+merge_strategy="$MERGE_STRATEGY"} \
        "${DURATION_ARG[@]}"
  exit 0
fi

exit 0
