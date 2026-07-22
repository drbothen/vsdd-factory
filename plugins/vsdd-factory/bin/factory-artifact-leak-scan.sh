#!/usr/bin/env bash
# factory-artifact-leak-scan.sh — content-based factory-artifact leak detector (#515)
#
# Companion to the path-prefix leak check proposed in #341. That check greps
# `git ls-files '.factory/*'`; it cannot see a factory artifact committed to a
# product path OUTSIDE `.factory/` (e.g. the root-level red-gate-log-S-12.08.md
# that #515 removed via PR #524). This scanner closes that blind spot by keying
# on CONTENT — the frontmatter `document_type:` — instead of location.
#
# Detection rule (derived from existing enumerations, no invented list):
#   A tracked file is a leaked factory artifact iff ALL hold:
#     1. its path is NOT under a `.factory/` (or `.factory-project/`) directory;
#        those are the artifact worktree and are .gitignored anyway.
#     2. its path is NOT plugin machinery — templates/, tests/, skills/, rules/
#        under plugins/vsdd-factory/ carry `document_type:` frontmatter as
#        examples/fixtures, not as live artifacts.
#     3. its frontmatter `document_type` is a factory-produced type — i.e. a
#        `document_type` declared by some template in
#        `${PLUGIN_ROOT}/templates/*.md`. This is the same enumeration
#        validate-template-compliance.sh trusts to resolve a template.
#     4. its (document_type, path) pair is NOT in PRODUCT_TRACKED_HOMES — the
#        small, data-derived set of factory doctypes the project intentionally
#        ships on the product branch (demo evidence), each exempt ONLY under
#        its canonical home directory. The same doctype anywhere else is a
#        leak. See that constant below.
#
# The artifact-path-registry (config/artifact-path-registry.yaml, ADR-016) is
# the single source of truth that EVERY registered artifact type is homed under
# `.factory/`; the doctypes in PRODUCT_TRACKED_HOMES are exactly the
# template-backed types that appear NOWHERE in that registry, so they have no
# `.factory/` home and legitimately live in the product tree — but only under
# their canonical directory; the exemption is path-scoped, not global.
#
# Usage:
#   factory-artifact-leak-scan.sh                 # table of leaks to stdout
#   factory-artifact-leak-scan.sh --list          # one leaked path per line
#   factory-artifact-leak-scan.sh --count         # number of leaks
#
# Exit codes:
#   0 — no leaks found (registry-clean)
#   1 — one or more leaked factory artifacts detected
#   2 — usage / environment error (not a git repo, missing templates dir)
#
# Deterministic, no network, no LLM. Advisory by design (see #515 discussion):
# it REPORTS leaks for a human/orchestrator to relocate; it does not mutate.

set -euo pipefail

_die() { echo "factory-artifact-leak-scan: $*" >&2; exit 2; }

# --- Resolve plugin root + repo root ---
_SELF_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PLUGIN_ROOT="${CLAUDE_PLUGIN_ROOT:-$(cd "$_SELF_DIR/.." && pwd)}"
TEMPLATES="$PLUGIN_ROOT/templates"

[[ -d "$TEMPLATES" ]] || _die "templates dir not found at $TEMPLATES"

# Repo root — scan tracked files here. Honour an explicit override for testing.
REPO_ROOT="${VSDD_REPO_ROOT:-}"
if [[ -z "$REPO_ROOT" ]]; then
  REPO_ROOT="$(git rev-parse --show-toplevel 2>/dev/null)" || _die "not inside a git repository"
fi

MODE="${1:-table}"
case "$MODE" in
  --list | --count | table) ;;
  *) _die "unknown mode '$MODE' (expected --list | --count | or no argument)" ;;
esac

# Factory doctypes the project intentionally tracks on the product branch. These
# are exactly the template-backed document_type values that have NO entry in the
# artifact-path-registry (no `.factory/` canonical home) and are demonstrably
# tracked under docs/ on develop: the demo-evidence deliverables. Kept explicit
# and small so the exclusion is auditable; extend only with the same evidence
# (a template-backed doctype with no `.factory/` registry home that the project
# ships in the product tree).
declare -A PRODUCT_TRACKED_HOMES=(
  [demo-evidence-report]="docs/demo-evidence/"
  [demo-evidence-index]="docs/demo-evidence/"
)

# --- Read the frontmatter document_type of a file (first block only) ---
_frontmatter_doctype() {
  awk '
    /^---[[:space:]]*$/ { fm++; if (fm == 2) exit; next }
    fm == 1 && /^document_type:/ {
      sub(/^document_type:[[:space:]]*/, "")
      gsub(/["'"'"']/, "")
      gsub(/^[[:space:]]+|[[:space:]]+$/, "")
      print
      exit
    }
  ' "$1"
}

# --- Build the set of factory-produced document_type values from templates ---
declare -A FACTORY_DOCTYPES
while IFS= read -r t; do
  dt="$(_frontmatter_doctype "$t")"
  [[ -n "$dt" ]] && FACTORY_DOCTYPES["$dt"]=1
done < <(find "$TEMPLATES" -name '*.md' -type f)

[[ -n "${FACTORY_DOCTYPES[*]:-}" ]] || _die "no factory document_types found in $TEMPLATES"

# --- Scan tracked .md files for leaks ---
LEAKS=()
while IFS= read -r rel; do
  [[ "$rel" == *.md ]] || continue
  # (1) skip the artifact worktree directories
  case "/$rel" in
    */.factory/* | */.factory-project/*) continue ;;
  esac
  case "$rel" in
    .factory/* | .factory-project/*) continue ;;
    # (2) skip plugin machinery: templates/tests/skills/rules carry example frontmatter
    plugins/vsdd-factory/templates/* | \
    plugins/vsdd-factory/tests/* | \
    plugins/vsdd-factory/skills/* | \
    plugins/vsdd-factory/rules/*) continue ;;
  esac
  abs="$REPO_ROOT/$rel"
  [[ -f "$abs" ]] || continue
  dt="$(_frontmatter_doctype "$abs")"
  [[ -n "$dt" ]] || continue
  # (3)+(4) leaked iff doctype is factory-produced AND the file is outside
  # the doctype's canonical product home (exemption is path-scoped: a
  # demo-evidence-report at repo root is a leak, under docs/demo-evidence/
  # it is a shipped deliverable).
  if [[ -n "${FACTORY_DOCTYPES[$dt]:-}" ]]; then
    home="${PRODUCT_TRACKED_HOMES[$dt]:-}"
    if [[ -n "$home" && "$rel" == "$home"* ]]; then
      continue
    fi
    LEAKS+=("$dt|$rel")
  fi
done < <(cd "$REPO_ROOT" && git ls-files '*.md')

# --- Emit ---
if [[ "$MODE" == "--count" ]]; then
  echo "${#LEAKS[@]}"
  [[ ${#LEAKS[@]} -eq 0 ]] && exit 0 || exit 1
fi

if [[ ${#LEAKS[@]} -eq 0 ]]; then
  [[ "$MODE" == "table" ]] && echo "0 leaked factory artifacts found. Product tree is clean."
  exit 0
fi

if [[ "$MODE" == "--list" ]]; then
  for entry in "${LEAKS[@]}"; do echo "${entry#*|}"; done
  exit 1
fi

# table mode
echo "Leaked factory artifacts detected on the product branch (#515):" >&2
echo "" >&2
printf '%-24s  %s\n' "document_type" "tracked path" >&2
printf '%-24s  %s\n' "------------------------" "------------" >&2
for entry in "${LEAKS[@]}"; do
  printf '%-24s  %s\n' "${entry%%|*}" "${entry#*|}" >&2
done
echo "" >&2
echo "These carry factory-artifact frontmatter but live outside .factory/." >&2
echo "Relocate to their canonical .factory/ home (see config/artifact-path-registry.yaml)" >&2
echo "or, if a genuine product deliverable, add its (document_type -> home" >&2
echo "directory) pair to PRODUCT_TRACKED_HOMES in this script with justification." >&2
exit 1
