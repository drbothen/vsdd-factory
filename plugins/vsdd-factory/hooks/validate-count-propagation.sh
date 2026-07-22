#!/bin/bash
# validate-count-propagation.sh — PostToolUse lint hook for cross-document count drift
#
# Triggered: PostToolUse Write/Edit on ARCH-INDEX.md, BC-INDEX.md, VP-INDEX.md,
#            STATE.md, STORY-INDEX.md, PRD.md, or SS-NN-*.md architecture files.
#
# Behavior:
#   1. Extract count-bearing patterns from the modified file using anchored regexes.
#   2. Grep corpus index files for the same count-keyword pairs.
#   3. If the same keyword appears in a sibling document at a DIFFERENT numeric value,
#      emit a structured warning to stderr and exit 2.
#   4. Exit 0 on no drift OR if count is absent from a sibling file (absence != drift).
#
# Scope limit: reports drift, does not modify files, does not interpret semantics.
# Performance: deterministic, <200ms on typical corpus.
#
# S-7.02 / BC-7.05.001, BC-7.05.002

set -euo pipefail
shopt -s extglob

# Source canonical block-message helper if available (provides block_pre).
_SELF_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
_BLOCK_SH="${CLAUDE_PLUGIN_ROOT:+${CLAUDE_PLUGIN_ROOT}/hooks/lib/block.sh}"
_BLOCK_SH="${_BLOCK_SH:-${_SELF_DIR}/lib/block.sh}"
# shellcheck source=lib/block.sh disable=SC1091
if [ -f "$_BLOCK_SH" ]; then source "$_BLOCK_SH"; fi

if ! command -v jq &>/dev/null; then
  exit 0
fi

INPUT=$(cat)
FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')

# Skip if no file path provided or file does not exist
if [[ -z "$FILE_PATH" ]] || [[ ! -f "$FILE_PATH" ]]; then
  exit 0
fi

# Only trigger for index/state/architecture files
BASENAME=$(basename "$FILE_PATH")
case "$BASENAME" in
  ARCH-INDEX.md|BC-INDEX.md|VP-INDEX.md|STATE.md|PRD.md|prd.md|STORY-INDEX.md) ;;
  SS-[0-9][0-9]-*.md) ;;
  *) exit 0 ;;
esac

# Resolve corpus root: walk up from file path to find known anchor
CORPUS_ROOT=""
DIR=$(dirname "$FILE_PATH")
while [[ "$DIR" != "/" ]]; do
  if [[ -d "$DIR/.factory" ]] || [[ -f "$DIR/STATE.md" ]] || [[ "$(basename "$DIR")" == ".factory" ]]; then
    CORPUS_ROOT="$DIR"
    break
  fi
  DIR=$(dirname "$DIR")
done
if [[ -z "$CORPUS_ROOT" ]]; then
  CORPUS_ROOT=$(dirname "$FILE_PATH")
fi

# Build list of sibling index files to check (only those that exist, excluding source file)
SIBLING_FILES=()
for candidate in \
  "$CORPUS_ROOT/.factory/STATE.md" \
  "$CORPUS_ROOT/.factory/specs/architecture/ARCH-INDEX.md" \
  "$CORPUS_ROOT/.factory/specs/behavioral-contracts/BC-INDEX.md" \
  "$CORPUS_ROOT/.factory/specs/verification-properties/VP-INDEX.md" \
  "$CORPUS_ROOT/.factory/stories/STORY-INDEX.md" \
  "$CORPUS_ROOT/STATE.md" \
  "$CORPUS_ROOT/ARCH-INDEX.md" \
  "$CORPUS_ROOT/BC-INDEX.md" \
  "$CORPUS_ROOT/VP-INDEX.md" \
  "$CORPUS_ROOT/STORY-INDEX.md"; do
  if [[ -f "$candidate" ]] && [[ "$candidate" != "$FILE_PATH" ]]; then
    SIBLING_FILES+=("$candidate")
  fi
done

# If no siblings found, nothing to compare against — exit clean
if [[ ${#SIBLING_FILES[@]} -eq 0 ]]; then
  exit 0
fi

# H2 sections whose count mentions are historical/frozen by project convention:
# changelog rows, phase-history rows, and decision-log rows. A count inside one
# of these ("Lists all 36 BCs" in a changelog entry, "PRD (38 BCs)" in a Phase
# Progress row) is an immutable record and MUST be allowed to disagree with the
# current count — comparing it as drift is the #567 false positive. Boundaries
# use the same "^## opens, next ^## closes" idiom as
# validate-changelog-monotonicity.sh.
_is_historical_heading() {
  case "${1,,}" in
    "## changelog"* | "## change log"* | "## historical content"* | \
    "## phase progress"* | "## decisions log"*)
      return 0 ;;
  esac
  return 1
}

# Extract count-bearing pairs from a file.
# Outputs lines of format: KEYWORD:COUNT
# Supported patterns:
#   "NNN BCs" / "NNN,NNN BCs" — count before keyword
#   "BCs | NNN" / "BCs: NNN" — keyword before count (table or YAML)
#   "total_bcs: NNN" / "total_vps: NNN" — YAML frontmatter keys
# Count mentions inside historical H2 sections (see _is_historical_heading) are
# skipped so frozen records are never compared against the current count (#567).
_extract_counts() {
  local path="$1"
  local in_historical=0
  while IFS= read -r line; do
    local count keyword
    # Track historical-section boundaries; skip counts while inside one.
    # Runs BEFORE the ID-token drop below so headings are inspected verbatim
    # and historical lines skip the per-line mutation entirely.
    if [[ "$line" =~ ^##[[:space:]] ]]; then
      if _is_historical_heading "$line"; then in_historical=1; else in_historical=0; fi
    fi
    [[ "$in_historical" -eq 1 ]] && continue
    # Drop identifier tokens (E-11, S-3, BC-2.1.001, TD-001, SS-01) before
    # count extraction: the digits inside an ID are not a quantity. Without
    # this, "5 E-11 stories" mis-parses "11 stories" as a phantom count and
    # fires a false count-propagation drift (#690). Matches <letters>-<digits-
    # and-dots> so multi-part BC/VP ids are dropped whole. Deliberately flat:
    # a nested extglob (`?(*(.+([0-9])))`) matches the same ID tokens but
    # backtracks super-linearly on long dotted-id lines (33s on a 600-char
    # line of BC-N.NN.NN tokens vs 0.5s flat), and this runs per line on
    # repo-controlled content. The flat class additionally eats dots glued to
    # an ID (e.g. a sentence period in "delivered E-11."), which is harmless
    # here — a whitespace-separated count token can never contain them.
    line="${line//+([A-Za-z])-+([0-9.])/}"
    # Pattern A: count before keyword
    if [[ "$line" =~ ([0-9][0-9,]+)[[:space:]]+(BCs|VPs|stories|capabilities|subsystems) ]]; then
      keyword="${BASH_REMATCH[2]}"
      count="${BASH_REMATCH[1]//,/}"
      echo "${keyword}:${count}"
    fi
    # Pattern B: keyword before count (table cell or colon-value)
    if [[ "$line" =~ (BCs|VPs|stories|capabilities)[[:space:]]*[|:][[:space:]]*([0-9][0-9,]+) ]]; then
      keyword="${BASH_REMATCH[1]}"
      count="${BASH_REMATCH[2]//,/}"
      echo "${keyword}:${count}"
    fi
    # Pattern C: YAML "total_bcs: NNN"
    if [[ "$line" =~ total_bcs:[[:space:]]*([0-9][0-9,]+) ]]; then
      count="${BASH_REMATCH[1]//,/}"
      echo "BCs:${count}"
    fi
    # Pattern D: YAML "total_vps: NNN"
    if [[ "$line" =~ total_vps:[[:space:]]*([0-9][0-9,]+) ]]; then
      count="${BASH_REMATCH[1]//,/}"
      echo "VPs:${count}"
    fi
  done < "$path"
}

# Extract counts from modified file (first occurrence per keyword wins)
declare -A SOURCE_COUNTS
while IFS=: read -r kw cnt; do
  [[ -z "$kw" || -z "$cnt" ]] && continue
  if [[ -z "${SOURCE_COUNTS[$kw]:-}" ]]; then
    SOURCE_COUNTS["$kw"]="$cnt"
  fi
done < <(_extract_counts "$FILE_PATH")

# Nothing to compare if no count patterns found.
# Guard via ${arr[*]:-} rather than ${#arr[@]}: under `set -u`, expanding the
# element count of an *empty associative array* is itself an unbound-variable
# error in bash 4+. Two paths above can leave this array empty: the ID-token
# drop (an all-phantom line like "5 E-11 stories" with no genuine count) and
# the historical-section skip (a file whose only counts were changelog/
# phase-history rows). Either way this
# no-count path is now reachable and must exit 0 cleanly, not crash.
if [[ -z "${SOURCE_COUNTS[*]:-}" ]]; then
  exit 0
fi

DRIFT_DETECTED=0
DRIFT_MESSAGES=()

for keyword in "${!SOURCE_COUNTS[@]}"; do
  source_count="${SOURCE_COUNTS[$keyword]}"

  for sibling in "${SIBLING_FILES[@]}"; do
    # Extract first matching count for this keyword from sibling
    sib_count=""
    while IFS=: read -r kw cnt; do
      if [[ "$kw" == "$keyword" ]]; then
        sib_count="$cnt"
        break
      fi
    done < <(_extract_counts "$sibling")

    # Absence of keyword in sibling is NOT drift — only report mismatch
    if [[ -n "$sib_count" ]] && [[ "$sib_count" != "$source_count" ]]; then
      DRIFT_DETECTED=1
      DRIFT_MESSAGES+=("COUNT DRIFT DETECTED: '${source_count} ${keyword}' in $(basename "$FILE_PATH") but '${sib_count} ${keyword}' in $(basename "$sibling").")
      DRIFT_MESSAGES+=("  Run: grep -r \"${keyword}\" .factory/specs/ .factory/STATE.md to reconcile.")
    fi
  done
done

if [[ "$DRIFT_DETECTED" -eq 1 ]]; then
  # S2 fix: join ALL drift messages so every violation is visible at once, not just the first.
  _ALL_DRIFT=$(printf '%s; ' "${DRIFT_MESSAGES[@]}" | sed 's/; $//')
  _REASON="Count propagation drift detected in $(basename "$FILE_PATH"): $_ALL_DRIFT"
  _REC="Update the lagging index to match the source-of-truth count: BC-INDEX total_bcs, ARCH-INDEX subsystem counts, or STORY-INDEX bcs count — see the specific drift cited above"

  if declare -f block_pre >/dev/null 2>&1; then
    block_pre "validate-count-propagation" \
      "$_REASON" \
      "$_REC" \
      "count_propagation_drift"
    # block_pre exits 2; unreachable
  else
    echo "BLOCKED by validate-count-propagation: ${_REASON}. Fix: ${_REC}. Code: count_propagation_drift." >&2
    exit 2
  fi
fi

exit 0
