#!/bin/bash
# bump-version.sh — prepare release narrative for the next version.
#
# Prepends a CHANGELOG.md section heading with today's date so the
# release notes have a starting point. Does NOT commit or tag — that's
# intentional; the caller reviews the diff and stages the changes.
#
# v1.0.0-beta.4 cache-staleness fix: this script no longer touches
# plugins/vsdd-factory/.claude-plugin/plugin.json or .claude-plugin/
# marketplace.json. Those fields are now written by the Release workflow
# (.github/workflows/release.yml), in the same bot commit that bundles
# the dispatcher binaries. This eliminates the race window where the
# chore commit advertised version X but did not yet have version-X
# binaries, which caused Claude Code's plugin cache to lock to a stale
# binary set under version X. With this change, plugin.json:version
# stays at X-1 during the workflow run; the bot's binary-bundle commit
# writes X (from the git tag) atomically with the binaries.
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

for f in "$CHANGELOG"; do
  if [ ! -f "$f" ]; then
    echo "error: required file missing: $f" >&2
    exit 1
  fi
done

# --- Read current versions for context display only -------------------------
# plugin.json + marketplace.json are no longer touched by this script (the
# release workflow's bot commit writes them from the git tag). We still
# read + display the current values so the operator knows what version
# they're moving away from.
OLD_PLUGIN=$(jq -r '.version' "$PLUGIN_JSON" 2>/dev/null || echo "(unreadable)")
OLD_MKT=$(jq -r '.plugins[0].version // empty' "$MARKETPLACE_JSON" 2>/dev/null || echo "(unreadable)")

echo "Current versions (display-only — workflow bot commit writes these):"
echo "  plugin.json      = $OLD_PLUGIN"
echo "  marketplace.json = $OLD_MKT"
echo "  new              = $NEW_VERSION"

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
echo "Prepared $NEW_VERSION:"
echo "  CHANGELOG.md     $CHANGELOG_UPDATED"
echo "  plugin.json      unchanged (stays at $OLD_PLUGIN; bot writes from tag)"
echo "  marketplace.json unchanged (stays at $OLD_MKT; bot writes from tag)"
echo
echo "Next steps:"
echo "  1. Edit CHANGELOG.md to fill in the TODOs (if a stub was prepended)."
echo "  2. git add CHANGELOG.md"
echo "  3. git commit -m \"chore: release v$NEW_VERSION — ${TITLE:-<title>}\""
echo "  4. git tag -a v$NEW_VERSION -m \"v$NEW_VERSION — ${TITLE:-<title>}\""
echo "  5. git push origin main && git push origin v$NEW_VERSION"
echo
echo "The Release workflow will:"
echo "  - Build dispatcher binaries for 5 platforms"
echo "  - Build legacy-bash-adapter.wasm + plugin wasms"
echo "  - Write plugin.json:version = $NEW_VERSION (from tag)"
echo "  - Write marketplace.json:plugins[0].version = $NEW_VERSION"
echo "  - Commit binaries + JSON updates atomically as github-actions[bot]"
echo "  - Force-update the tag to point at the bot commit"
echo "  - Create the GH (pre-)release"
echo
echo "Pro tip: write the real CHANGELOG entry under ## $NEW_VERSION BEFORE running"
echo "this script. This script detects an existing heading and skips the stub,"
echo "which eliminates the race that bit v0.76.1 / v0.78.1 / v0.79.0."
