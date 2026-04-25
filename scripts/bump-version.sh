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
#   scripts/bump-version.sh 1.0.0-beta.1 "Factory Plugin Kit beta"
#
# Args:
#   1: semver version (required; matches N.N.N or N.N.N-prerelease per
#      semver 2.0 §9, e.g., 1.0.0, 1.0.0-beta.1, 1.0.0-rc.2, 1.0.0-alpha)
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
  echo "  e.g.: $0 1.0.0-beta.1 \"Factory Plugin Kit beta\"" >&2
  exit 1
fi

NEW_VERSION="$1"
TITLE="${2:-}"

# Accept semver core (N.N.N) with an optional prerelease suffix (semver 2.0 §9).
# Prerelease identifier set is dot-separated alphanumerics-and-hyphens, which
# covers our `beta.N` / `rc.N` / `alpha` / `alpha.1.dev3` use cases. Build
# metadata (`+...`) is intentionally not accepted — we don't use it and
# allowing it would let drift into release tooling.
if ! [[ "$NEW_VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+(-[0-9A-Za-z.-]+)?$ ]]; then
  echo "error: version must match N.N.N or N.N.N-prerelease (got: $NEW_VERSION)" >&2
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

# --- Prepend CHANGELOG heading (idempotent) ---------------------------------
# If the CHANGELOG already has a `## $NEW_VERSION` heading (because the author
# wrote the entry before running bump-version.sh — the preferred flow), skip
# the stub insertion so we don't duplicate headings. This avoids the race
# where a post-bump Edit collides with the injected TODO and ships a stale
# stub in the release notes (happened three times across v0.76.1/v0.78.1/
# v0.79.0 before this guard landed).
DATE=$(date +%Y-%m-%d)
HEADING_LINE="## $NEW_VERSION — ${TITLE:-TODO: fill in release title} ($DATE)"

if grep -qE "^## $NEW_VERSION([[:space:]]|$)" "$CHANGELOG"; then
  echo "CHANGELOG.md: ## $NEW_VERSION heading already present — not prepending a stub."
  CHANGELOG_UPDATED="no (entry already present)"
else
  STUB=$(printf '%s\n\nTODO: describe the release.\n\n### Fixed\n\n- \n\n### Added\n\n- \n\n### Migration\n\nNo breaking changes.\n\n' "$HEADING_LINE")
  CHANGELOG_TMP="${CHANGELOG}.tmp.$$"
  {
    head -n 1 "$CHANGELOG"
    echo
    printf '%s' "$STUB"
    tail -n +3 "$CHANGELOG"
  } > "$CHANGELOG_TMP"
  mv "$CHANGELOG_TMP" "$CHANGELOG"
  CHANGELOG_UPDATED="prepended \"$HEADING_LINE\""
fi

# --- Report -----------------------------------------------------------------
echo
echo "Bumped to $NEW_VERSION:"
echo "  plugin.json      $OLD_PLUGIN -> $NEW_VERSION"
echo "  marketplace.json $OLD_MKT -> $NEW_VERSION"
echo "  CHANGELOG.md     $CHANGELOG_UPDATED"
echo
echo "Next steps:"
echo "  1. Edit CHANGELOG.md to fill in the TODOs (if a stub was prepended)."
echo "  2. git add plugins/vsdd-factory/.claude-plugin/plugin.json .claude-plugin/marketplace.json CHANGELOG.md"
echo "  3. git commit -m \"chore: release v$NEW_VERSION — ${TITLE:-<title>}\""
echo "  4. git tag -a v$NEW_VERSION -m \"v$NEW_VERSION — ${TITLE:-<title>}\""
echo "  5. git push origin main && git push origin v$NEW_VERSION"
echo
echo "Pro tip: write the real CHANGELOG entry under ## $NEW_VERSION BEFORE running"
echo "this script. This script detects an existing heading and skips the stub,"
echo "which eliminates the race that bit v0.76.1 / v0.78.1 / v0.79.0."
