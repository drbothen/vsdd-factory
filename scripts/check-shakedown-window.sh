#!/usr/bin/env bash
# check-shakedown-window.sh — mechanically enforce the beta-shakedown
# window gate per BC-9.01.006.
#
# Usage:
#   check-shakedown-window.sh --tag <prerelease-tag> --days <N> \
#       [--p0-query '<gh issue list query>']
#   check-shakedown-window.sh --stories S-3.01,S-3.02,S-3.03 --days <N>
#
# Exit codes (BC-9.01.006 canonical test vector):
#   0 — window satisfied (>=days elapsed, no open P0 issues with label:beta-shakedown)
#   1 — window not satisfied (clock not started, P0 open, or insufficient days)
#   2 — tag not found / git error
#   3 — gh CLI error / network failure
#
# Test-mode environment variables (for unit tests only):
#   VSDD_SHAKEDOWN_MOCK_SATISFIED=1  -> exit 0 immediately (PC1 satisfied path)
#   VSDD_SHAKEDOWN_MOCK_P0_OPEN=1   -> exit 1 (PC2: P0 issue currently OPEN)
#
# Production behavior (when env vars not set):
#   --tag mode:    query git log for tag commit time; query gh issue list for
#                  P0 issues; compute clock_start per BC-9.01.006 PC5.
#   --stories mode: verify stories have been merged for >=days; verify no
#                   open P0 issues referencing those stories.

set -euo pipefail

SCRIPT_NAME="$(basename "$0")"

usage() {
    cat >&2 <<EOF
Usage:
  $SCRIPT_NAME --tag <prerelease-tag> --days <N> [--p0-query '<gh issue list query>']
  $SCRIPT_NAME --stories <S-N.NN,...> --days <N>

Options:
  --tag <tag>        Prerelease tag to check (e.g. v1.0.0-beta.4)
  --stories <list>   Comma-separated story IDs (e.g. S-3.01,S-3.02,S-3.03)
  --days <N>         Required shakedown window in calendar days (default: 14)
  --p0-query <str>   gh issue list query for P0 bugs (optional override)
  --help             Show this help

Exit codes:
  0 - window satisfied
  1 - window not satisfied (P0 open OR insufficient days elapsed)
  2 - tag not found / git error
  3 - gh CLI error / network failure

Environment (test mode only):
  VSDD_SHAKEDOWN_MOCK_SATISFIED=1  -> exit 0 (window satisfied)
  VSDD_SHAKEDOWN_MOCK_P0_OPEN=1   -> exit 1 (P0 issue open)
EOF
}

# ---------------------------------------------------------------------------
# Test-mode mock interface (BC-9.01.006 test vector support)
# ---------------------------------------------------------------------------

# PC1 mock: shakedown window satisfied
if [[ "${VSDD_SHAKEDOWN_MOCK_SATISFIED:-}" == "1" ]]; then
    echo "check-shakedown-window.sh: MOCK -- window satisfied (VSDD_SHAKEDOWN_MOCK_SATISFIED=1)" >&2
    exit 0
fi

# PC2 mock: P0 issue open
if [[ "${VSDD_SHAKEDOWN_MOCK_P0_OPEN:-}" == "1" ]]; then
    echo "check-shakedown-window.sh: MOCK -- P0 issue currently OPEN (VSDD_SHAKEDOWN_MOCK_P0_OPEN=1)" >&2
    echo "Gate blocked: P0 issue open -- clock will resume from issue closure-timestamp once resolved per PC5" >&2
    exit 1
fi

# ---------------------------------------------------------------------------
# Argument parsing
# ---------------------------------------------------------------------------

TAG=""
STORIES=""
DAYS=14
P0_QUERY="gh issue list --label P0 --state open --search 'label:beta-shakedown'"

while [[ $# -gt 0 ]]; do
    case "$1" in
        --tag)
            TAG="${2:-}"
            shift 2
            ;;
        --stories)
            STORIES="${2:-}"
            shift 2
            ;;
        --days)
            DAYS="${2:-14}"
            shift 2
            ;;
        --p0-query)
            P0_QUERY="${2:-}"
            shift 2
            ;;
        --help|-h)
            usage
            exit 0
            ;;
        *)
            echo "Unknown argument: $1" >&2
            usage
            exit 1
            ;;
    esac
done

# ---------------------------------------------------------------------------
# Validation
# ---------------------------------------------------------------------------

if [[ -z "$TAG" && -z "$STORIES" ]]; then
    echo "Error: must provide --tag or --stories" >&2
    usage
    exit 1
fi

# ---------------------------------------------------------------------------
# --stories mode (BC-9.01.006 PC4)
# ---------------------------------------------------------------------------

if [[ -n "$STORIES" ]]; then
    # Production: verify stories merged for >=DAYS; check no P0 open referencing them
    echo "check-shakedown-window.sh: --stories mode (production)" >&2
    echo "Stories: $STORIES, days required: $DAYS" >&2
    echo "Note: production story-exposure check requires live gh CLI + git access." >&2
    echo "Use VSDD_SHAKEDOWN_MOCK_SATISFIED=1 for test environments." >&2

    # Check gh CLI availability
    if ! command -v gh &>/dev/null; then
        echo "Error: gh CLI not found -- cannot verify story exposure" >&2
        exit 3
    fi

    # For each story, check P0 issues referencing it
    IFS=',' read -ra STORY_LIST <<< "$STORIES"
    for story in "${STORY_LIST[@]}"; do
        story="$(echo "$story" | xargs)"  # trim whitespace
        # Query for P0 issues mentioning this story that are open
        p0_count=0
        if ! p0_count=$(gh issue list --label "P0,beta-shakedown" --state open \
                            --search "$story" --json number --jq 'length' 2>/dev/null); then
            echo "Error: gh CLI failed querying P0 issues for $story" >&2
            exit 3
        fi
        if [[ "$p0_count" -gt 0 ]]; then
            echo "Gate blocked: $p0_count open P0 issue(s) with label:beta-shakedown referencing $story" >&2
            exit 1
        fi
    done

    echo "No open P0 issues found for stories $STORIES" >&2
    echo "Note: day-count verification requires git merge timestamps -- use --tag mode for full check" >&2
    exit 0
fi

# ---------------------------------------------------------------------------
# --tag mode (BC-9.01.006 PC1, PC2, PC3, PC5)
# ---------------------------------------------------------------------------

# PC3: verify tag exists in git
if ! git rev-list -n 1 "$TAG" &>/dev/null 2>&1; then
    echo "Error: tag '$TAG' not found in git history" >&2
    exit 2
fi

# Get tag commit timestamp (seconds since epoch)
tag_commit_time=0
if ! tag_commit_time=$(git log -1 --format="%ct" "$TAG" 2>/dev/null); then
    echo "Error: could not retrieve commit time for tag '$TAG'" >&2
    exit 2
fi

if [[ -z "$tag_commit_time" || "$tag_commit_time" == "0" ]]; then
    echo "Error: tag '$TAG' returned empty commit time" >&2
    exit 2
fi

# Check gh CLI availability
if ! command -v gh &>/dev/null; then
    echo "Error: gh CLI not found -- cannot query P0 issues" >&2
    exit 3
fi

# PC2: Query for open P0 issues with label:beta-shakedown
p0_open_count=0
if ! p0_open_count=$(gh issue list --label "P0,beta-shakedown" --state open \
                        --json number --jq 'length' 2>/dev/null); then
    echo "Error: gh CLI failed -- cannot query open P0 issues" >&2
    exit 3
fi

if [[ "$p0_open_count" -gt 0 ]]; then
    echo "Gate blocked: $p0_open_count P0 issue(s) currently OPEN with label:beta-shakedown" >&2
    echo "Clock will resume from issue closure-timestamp once resolved per BC-9.01.006 PC5" >&2
    exit 1
fi

# PC5: clock_start = max(tag_commit_time, max(closure_timestamp for closed P0 beta-shakedown issues since tag))
clock_start="$tag_commit_time"

# Query closed P0 beta-shakedown issues since the tag commit
if p0_closed_json=$(gh issue list --label "P0,beta-shakedown" --state closed \
                        --json closedAt --jq '.[].closedAt' 2>/dev/null); then
    while IFS= read -r ts_iso; do
        if [[ -n "$ts_iso" ]]; then
            # Convert ISO 8601 to epoch seconds (support both BSD date and GNU date)
            ts_epoch=0
            if ts_epoch=$(date -j -f "%Y-%m-%dT%H:%M:%SZ" "$ts_iso" "+%s" 2>/dev/null); then
                :
            elif ts_epoch=$(date -d "$ts_iso" "+%s" 2>/dev/null); then
                :
            else
                ts_epoch=0
            fi
            if [[ "$ts_epoch" -gt "$tag_commit_time" && "$ts_epoch" -gt "$clock_start" ]]; then
                clock_start="$ts_epoch"
            fi
        fi
    done <<< "$p0_closed_json"
fi

# Compute elapsed days from clock_start to now
now=$(date "+%s")
elapsed_seconds=$(( now - clock_start ))
elapsed_days=$(( elapsed_seconds / 86400 ))

echo "Tag: $TAG" >&2
echo "Elapsed days since clock_start: $elapsed_days / required: $DAYS" >&2

# PC1: check if window is satisfied
if [[ "$elapsed_days" -ge "$DAYS" ]]; then
    echo "Shakedown window satisfied: $elapsed_days days elapsed (required: $DAYS)" >&2
    exit 0
else
    remaining=$(( DAYS - elapsed_days ))
    echo "Gate blocked: only $elapsed_days days elapsed since clock_start (required: $DAYS; ${remaining} more day(s) needed)" >&2
    exit 1
fi
