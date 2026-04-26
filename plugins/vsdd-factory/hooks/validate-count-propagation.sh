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

# Extract count-bearing pairs from a file.
# Outputs lines of format: KEYWORD:COUNT
# Supported patterns:
#   "NNN BCs" / "NNN,NNN BCs" — count before keyword
#   "BCs | NNN" / "BCs: NNN" — keyword before count (table or YAML)
#   "total_bcs: NNN" / "total_vps: NNN" — YAML frontmatter keys
_extract_counts() {
  local path="$1"
  while IFS= read -r line; do
    local count keyword
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

# Nothing to compare if no count patterns found
if [[ ${#SOURCE_COUNTS[@]} -eq 0 ]]; then
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
  for msg in "${DRIFT_MESSAGES[@]}"; do
    echo "$msg" >&2
  done
  exit 2
fi

exit 0
