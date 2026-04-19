#!/bin/bash
# validate-template-compliance.sh — PostToolUse hook for template compliance
#
# After every Write to .factory/**/*.md, checks that the file contains:
# 1. Required frontmatter fields from its corresponding template
# 2. Required H2 section headings from its corresponding template
#
# Template resolution: reads document_type from the file's frontmatter,
# searches ${CLAUDE_PLUGIN_ROOT}/templates/ for a template with matching
# document_type. Falls back to path-pattern matching.
#
# Non-blocking (PostToolUse) — the file is already written. The agent
# sees the warning immediately and can fix missing structure.
#
# Exit 0 on pass (or if no template found).
# Exit 2 on missing required fields/sections with diagnostic on stderr.
#
# Deterministic, <500ms, no LLM.

set -euo pipefail

if ! command -v jq &>/dev/null; then
  exit 0
fi

INPUT=$(cat)
FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')

if [[ -z "$FILE_PATH" ]] || [[ ! -f "$FILE_PATH" ]]; then
  exit 0
fi

# Only trigger for .factory/ markdown files
case "$FILE_PATH" in
  *.factory/*.md) ;;
  *) exit 0 ;;
esac

# Skip INDEX files (auto-generated, no dedicated template)
case "$FILE_PATH" in
  *INDEX.md|*STORY-INDEX*|*BC-INDEX*|*VP-INDEX*|*ARCH-INDEX*|*HS-INDEX*|*L2-INDEX*|*UX-INDEX*|*EVAL-INDEX*|*ADV-*-INDEX*) exit 0 ;;
  *) ;;
esac

# Skip current-cycle pointer file and config files
case "$FILE_PATH" in
  *current-cycle|*.yaml|*.json) exit 0 ;;
  *) ;;
esac

# Resolve CLAUDE_PLUGIN_ROOT — hook runs from the plugin directory
PLUGIN_ROOT="${CLAUDE_PLUGIN_ROOT:-}"
if [[ -z "$PLUGIN_ROOT" ]]; then
  # Fallback: derive from this script's location
  PLUGIN_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
fi
TEMPLATES="$PLUGIN_ROOT/templates"

if [[ ! -d "$TEMPLATES" ]]; then
  exit 0
fi

# --- Step 1: Extract document_type from the written file ---
DOC_TYPE=$(awk '
  /^---$/{ fm++; next }
  fm==1 && /^document_type:/ {
    sub(/^document_type:[ \t]*/, "")
    gsub(/["'"'"']/, "")
    gsub(/^[ \t]+|[ \t]+$/, "")
    print
    exit
  }
' "$FILE_PATH")

# --- Step 2: Find matching template ---
TEMPLATE=""

# Primary: match by document_type
if [[ -n "$DOC_TYPE" ]]; then
  for t in "$TEMPLATES"/*.md; do
    t_type=$(awk '
      /^---$/{ fm++; next }
      fm==1 && /^document_type:/ {
        sub(/^document_type:[ \t]*/, "")
        gsub(/["'"'"']/, "")
        gsub(/^[ \t]+|[ \t]+$/, "")
        print
        exit
      }
    ' "$t")
    if [[ "$t_type" == "$DOC_TYPE" ]]; then
      TEMPLATE="$t"
      break
    fi
  done
fi

# Fallback: match by path pattern
if [[ -z "$TEMPLATE" ]]; then
  case "$FILE_PATH" in
    *behavioral-contracts/BC-*) TEMPLATE="$TEMPLATES/behavioral-contract-template.md" ;;
    *verification-properties/VP-*) TEMPLATE="$TEMPLATES/L4-verification-property-template.md" ;;
    *stories/STORY-*) TEMPLATE="$TEMPLATES/story-template.md" ;;
    *holdout-scenarios/HS-*) TEMPLATE="$TEMPLATES/holdout-scenario-template.md" ;;
    *architecture/verification-coverage-matrix*) TEMPLATE="$TEMPLATES/verification-coverage-matrix-template.md" ;;
    *architecture/verification-architecture*) TEMPLATE="$TEMPLATES/verification-architecture-template.md" ;;
    *architecture/*) TEMPLATE="$TEMPLATES/architecture-section-template.md" ;;
    *domain-spec/*) TEMPLATE="$TEMPLATES/L2-domain-spec-section-template.md" ;;
    *prd.md) TEMPLATE="$TEMPLATES/prd-template.md" ;;
    *product-brief.md) TEMPLATE="$TEMPLATES/product-brief-template.md" ;;
    *STATE.md) TEMPLATE="$TEMPLATES/state-template.md" ;;
    *dtu-assessment.md) TEMPLATE="$TEMPLATES/dtu-assessment-template.md" ;;
    *module-criticality.md) TEMPLATE="$TEMPLATES/module-criticality-template.md" ;;
    *) ;; # No match — skip
  esac
fi

if [[ -z "$TEMPLATE" ]] || [[ ! -f "$TEMPLATE" ]]; then
  exit 0  # No template to validate against
fi

TEMPLATE_NAME=$(basename "$TEMPLATE")

# --- Step 3: Extract required frontmatter keys from template ---
TEMPLATE_KEYS=$(awk '
  /^---$/{ fm++; next }
  fm==1 && /^[a-z]/ {
    sub(/:.*/, "")
    gsub(/^[ \t]+|[ \t]+$/, "")
    print
  }
' "$TEMPLATE" | sort)

# --- Step 4: Extract frontmatter keys from written file ---
FILE_KEYS=$(awk '
  /^---$/{ fm++; next }
  fm==1 && /^[a-z]/ {
    sub(/:.*/, "")
    gsub(/^[ \t]+|[ \t]+$/, "")
    print
  }
' "$FILE_PATH" | sort)

# --- Step 5: Find missing frontmatter keys ---
MISSING_KEYS=""
for key in $TEMPLATE_KEYS; do
  if ! echo "$FILE_KEYS" | grep -qx "$key"; then
    MISSING_KEYS="${MISSING_KEYS:+$MISSING_KEYS, }$key"
  fi
done

# --- Step 6: Extract required H2 sections from template ---
# Only check sections that aren't conditional (skip those with "only" in the heading)
TEMPLATE_SECTIONS=$(grep '^## ' "$TEMPLATE" | sed 's/^## //' | grep -vi 'only\|optional\|recommended\|conditional' || true)

# --- Step 7: Extract H2 sections from written file ---
FILE_SECTIONS=$(grep '^## ' "$FILE_PATH" | sed 's/^## //' || true)

# --- Step 8: Find missing sections ---
MISSING_SECTIONS=""
while IFS= read -r section; do
  [[ -z "$section" ]] && continue
  # Exact match or prefix match (template may have "(MANDATORY)" suffix)
  section_base=$(echo "$section" | sed 's/ (MANDATORY)//;s/ (DF-[0-9]*)//;s/ (.*//')
  if ! echo "$FILE_SECTIONS" | grep -qiF "$section_base"; then
    MISSING_SECTIONS="${MISSING_SECTIONS:+$MISSING_SECTIONS||}$section"
  fi
done <<< "$TEMPLATE_SECTIONS"

# --- Step 9: Report ---
if [[ -n "$MISSING_KEYS" || -n "$MISSING_SECTIONS" ]]; then
  echo "TEMPLATE COMPLIANCE WARNING ($(basename "$FILE_PATH") → $TEMPLATE_NAME):" >&2
  if [[ -n "$MISSING_KEYS" ]]; then
    TOTAL_TMPL=$(echo "$TEMPLATE_KEYS" | wc -w | tr -d ' ')
    TOTAL_FILE=$(echo "$FILE_KEYS" | wc -w | tr -d ' ')
    echo "  Frontmatter: $TOTAL_FILE/$TOTAL_TMPL fields present. Missing: $MISSING_KEYS" >&2
  fi
  if [[ -n "$MISSING_SECTIONS" ]]; then
    echo "  Sections missing:" >&2
    echo "$MISSING_SECTIONS" | tr '||' '\n' | while IFS= read -r s; do
      [[ -n "$s" ]] && echo "    - ## $s" >&2
    done
  fi
  echo "  Fix: run /vsdd-factory:conform-to-template $(basename "$FILE_PATH") to add missing structure." >&2
  exit 2
fi

exit 0
