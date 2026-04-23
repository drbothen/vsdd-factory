#!/bin/bash
# bump-version.sh — atomically bump the vsdd-factory release version.
#
# Updates the two version fields that MUST move in lockstep or the
# Release workflow fails on tag push (see v0.68.1, v0.69.0 fallout):
#   - plugins/vsdd-factory/.claude-plugin/plugin.json
#   - .claude-plugin/marketplace.json   (.plugins[0].version)
#
# Also prepends a CHANGELOG.md section heading with today's date so the
# release notes have a starting point. Does NOT commit or tag — that's
# intentional; the caller reviews the diff and stages the changes.
#
# Usage:
#   scripts/bump-version.sh 0.71.0
#   scripts/bump-version.sh 0.71.0 "Claude dashboard + CI parity check"
#
# Args:
#   1: semver version (required; must match N.N.N)
#   2: short release title (optional; used in CHANGELOG heading)
#
# Exits nonzero if:
#   - version arg is missing or malformed
#   - jq is unavailable
#   - either version file is missing
#   - git working tree has uncommitted changes in the files we touch
#     (prevents clobbering in-progress edits)
#
# Safe to re-run: if the version is already set in both files, this is
# a no-op for the JSON files. The CHANGELOG heading is prepended each
# invocation, so don't run twice for the same version without reverting.

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
PLUGIN_JSON="$REPO_ROOT/plugins/vsdd-factory/.claude-plugin/plugin.json"
MARKETPLACE_JSON="$REPO_ROOT/.claude-plugin/marketplace.json"
CHANGELOG="$REPO_ROOT/CHANGELOG.md"

# --- Validate args ----------------------------------------------------------
if [ $# -lt 1 ]; then
  echo "usage: $0 <semver-version> [short-title]" >&2
  echo "  e.g.: $0 0.71.0 \"Claude dashboard + CI parity\"" >&2
  exit 1
fi

NEW_VERSION="$1"
TITLE="${2:-}"

if ! [[ "$NEW_VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
  echo "error: version must match N.N.N (got: $NEW_VERSION)" >&2
  exit 1
fi

# --- Sanity checks ----------------------------------------------------------
if ! command -v jq >/dev/null 2>&1; then
  echo "error: jq is required but not installed" >&2
  exit 1
fi

for f in "$PLUGIN_JSON" "$MARKETPLACE_JSON" "$CHANGELOG"; do
  if [ ! -f "$f" ]; then
    echo "error: required file missing: $f" >&2
    exit 1
  fi
done

# Refuse to run if the target files have uncommitted changes. We don't
# want to quietly clobber work-in-progress.
for f in "$PLUGIN_JSON" "$MARKETPLACE_JSON"; do
  if ! git -C "$REPO_ROOT" diff --quiet -- "$f"; then
    echo "error: $f has uncommitted changes. Commit or stash first." >&2
    exit 1
  fi
done

# --- Read current versions --------------------------------------------------
OLD_PLUGIN=$(jq -r '.version' "$PLUGIN_JSON")
OLD_MKT=$(jq -r '.plugins[0].version // empty' "$MARKETPLACE_JSON")

echo "Current versions:"
echo "  plugin.json      = $OLD_PLUGIN"
echo "  marketplace.json = $OLD_MKT"
echo "  new              = $NEW_VERSION"

if [ "$OLD_PLUGIN" = "$NEW_VERSION" ] && [ "$OLD_MKT" = "$NEW_VERSION" ]; then
  echo "Both files already at $NEW_VERSION; nothing to bump."
  echo "(Not touching CHANGELOG — re-run with version intentionally bumped.)"
  exit 0
fi

# --- Atomic bump via temp-file + mv -----------------------------------------
# jq can't edit in place portably, so we write to a sibling temp file and
# mv on success. Temp file must be on the same FS as the target for mv
# to be atomic.
_bump_json() {
  local file="$1"
  local filter="$2"
  local tmp="${file}.tmp.$$"
  jq "$filter" "$file" > "$tmp"
  mv "$tmp" "$file"
}

_bump_json "$PLUGIN_JSON"      ".version = \"$NEW_VERSION\""
_bump_json "$MARKETPLACE_JSON" ".plugins[0].version = \"$NEW_VERSION\""

# --- Prepend CHANGELOG heading ----------------------------------------------
DATE=$(date +%Y-%m-%d)
HEADING_LINE="## $NEW_VERSION — ${TITLE:-TODO: fill in release title} ($DATE)"
STUB="$HEADING_LINE\n\nTODO: describe the release.\n\n### Fixed\n\n- \n\n### Added\n\n- \n\n### Migration\n\nNo breaking changes.\n\n"

CHANGELOG_TMP="${CHANGELOG}.tmp.$$"
{
  head -n 1 "$CHANGELOG"
  echo
  printf "$STUB"
  tail -n +3 "$CHANGELOG"
} > "$CHANGELOG_TMP"
mv "$CHANGELOG_TMP" "$CHANGELOG"

# --- Report -----------------------------------------------------------------
echo
echo "Bumped to $NEW_VERSION:"
echo "  plugin.json      $OLD_PLUGIN -> $NEW_VERSION"
echo "  marketplace.json $OLD_MKT -> $NEW_VERSION"
echo "  CHANGELOG.md     prepended \"$HEADING_LINE\""
echo
echo "Next steps:"
echo "  1. Edit CHANGELOG.md to fill in the TODOs."
echo "  2. git add plugins/vsdd-factory/.claude-plugin/plugin.json .claude-plugin/marketplace.json CHANGELOG.md"
echo "  3. git commit -m \"chore: release v$NEW_VERSION — ${TITLE:-<title>}\""
echo "  4. git tag -a v$NEW_VERSION -m \"v$NEW_VERSION — ${TITLE:-<title>}\""
echo "  5. git push origin main && git push origin v$NEW_VERSION"
