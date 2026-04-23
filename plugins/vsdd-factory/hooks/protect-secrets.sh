#!/bin/bash
# protect-secrets.sh — PreToolUse hook for Bash and Read tools
#
# Blocks operations that tend to leak secrets from .env files and environment
# variables into the conversation transcript.
#
# Covered surfaces:
#   Read tool       — direct file reads of .env / .env.* / .envrc
#   Bash cat/less/etc — shell reads of .env files
#   Bash cp/mv      — copying/moving .env files (exfil risk)
#   Bash echo/printf — printing $*_TOKEN / $*_SECRET / $*_PASSWORD etc.
#   Bash env|grep   — grepping environment for secret-shaped names
#
# Allowed:
#   ls .env* / test -f .env / [ -f .env ]   — existence checks
#   source .env / . .env                    — loading env in a shell
#   .env.example / .env.sample              — templates (no secrets expected)
#
# Exit 0 = allow, Exit 2 = block with diagnostic on stderr.
# Deterministic, <50ms, no LLM.

set -euo pipefail

if ! command -v jq &>/dev/null; then
  exit 0
fi

INPUT=$(cat)
TOOL=$(echo "$INPUT" | jq -r '.tool_name // empty')

block() {
  local reason="$1"
  local suggestion="${2:-}"
  echo "BLOCKED by protect-secrets:" >&2
  echo "  $reason" >&2
  if [[ -n "$suggestion" ]]; then
    echo "  Suggestion: $suggestion" >&2
  fi
  exit 2
}

# Pattern for a .env-like filename. Matches: .env, .env.local, .env.production,
# .envrc. Does NOT match: .env.example, .env.sample, .env.template (safe templates).
is_secret_env_filename() {
  local name="$1"
  name=$(basename "$name")
  case "$name" in
    .env.example|.env.sample|.env.template) return 1 ;;
    .env|.envrc) return 0 ;;
    .env.*) return 0 ;;
    *) return 1 ;;
  esac
}

# ---------------------------------------------------------------------------
# Read tool: block direct reads of .env files
# ---------------------------------------------------------------------------
if [[ "$TOOL" == "Read" ]]; then
  FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')
  if [[ -n "$FILE_PATH" ]] && is_secret_env_filename "$FILE_PATH"; then
    block \
      "Reading '$FILE_PATH' is blocked — .env files typically contain secrets that would land in the transcript." \
      "If a specific non-secret value is needed, ask the user to paste it directly. For templates, use .env.example."
  fi
  exit 0
fi

# ---------------------------------------------------------------------------
# Bash tool: inspect the command
# ---------------------------------------------------------------------------
if [[ "$TOOL" != "Bash" ]]; then
  exit 0
fi

COMMAND=$(echo "$INPUT" | jq -r '.tool_input.command // empty')
if [[ -z "$COMMAND" ]]; then
  exit 0
fi

# Match .env / .env.X tokens in the command, excluding .env.example|sample|template.
# Used to extract candidate filenames before deciding whether to block.
#
# A filename occurrence is a word boundary + dotted name + (end or slash/space/quote).
ENV_TOKEN_RE='(^|[^A-Za-z0-9_./-])\.env(\.[A-Za-z0-9_-]+)?([^A-Za-z0-9_.-]|$)'

command_references_secret_env() {
  # Returns 0 if the command mentions a .env* filename that is NOT a known template.
  # Extracts all .env* tokens and checks each.
  local tokens
  tokens=$(echo "$COMMAND" | grep -oE '\.env(\.[A-Za-z0-9_-]+)?' || true)
  [[ -z "$tokens" ]] && return 1
  while IFS= read -r tok; do
    case "$tok" in
      .env.example|.env.sample|.env.template) continue ;;
      *) return 0 ;;
    esac
  done <<< "$tokens"
  return 1
}

# ---------------------------------------------------------------------------
# Block content-reading tools targeting .env files
# ---------------------------------------------------------------------------
# cat / less / more / head / tail / bat / xxd / od / strings / grep / awk / sed
if echo "$COMMAND" | grep -qE '\b(cat|less|more|head|tail|bat|xxd|od|strings|grep|awk|sed)\b'; then
  if echo "$COMMAND" | grep -qE "$ENV_TOKEN_RE"; then
    if command_references_secret_env; then
      block \
        "Reading .env file contents via shell is blocked — contents would leak into the transcript." \
        "Use 'ls .env*' to check existence. Ask the user to paste specific values if needed."
    fi
  fi
fi

# ---------------------------------------------------------------------------
# Block copy/move of .env files when the SOURCE is a real .env (exfil risk).
# Allow template → real env (e.g. `cp .env.example .env`) since only the
# destination is sensitive and the source is known-safe.
# ---------------------------------------------------------------------------
if echo "$COMMAND" | grep -qE '\b(cp|mv|rsync|scp)\b'; then
  # Extract the first non-flag positional arg after the command name.
  source_arg=$(echo "$COMMAND" | awk '
    {
      cmd_seen = 0
      for (i = 1; i <= NF; i++) {
        if ($i ~ /^(cp|mv|rsync|scp)$/) { cmd_seen = 1; continue }
        if (cmd_seen && $i !~ /^-/) { print $i; exit }
      }
    }')
  if [[ -n "$source_arg" ]]; then
    base=$(basename "$source_arg")
    case "$base" in
      .env.example|.env.sample|.env.template) : ;;
      .env|.envrc)
        block \
          "Copying/moving '$source_arg' is blocked — real .env files contain secrets." \
          "If you need a template, create .env.example. To restore a .env, have the user do it."
        ;;
      .env.*)
        block \
          "Copying/moving '$source_arg' is blocked — .env.* files typically contain secrets." \
          "If you need a template, use .env.example. To restore a .env, have the user do it."
        ;;
    esac
  fi
fi

# Block tar/zip that include a real .env as an input.
if echo "$COMMAND" | grep -qE '\b(tar|zip)\b'; then
  if command_references_secret_env; then
    block \
      "Archiving .env files is blocked — potential secret exfiltration." \
      "Exclude .env from archives, or handle it outside this session."
  fi
fi

# ---------------------------------------------------------------------------
# Block echo/printf of secret-shaped env variables
# Matches $VAR and ${VAR} forms where VAR contains TOKEN/SECRET/PASSWORD/etc.
# ---------------------------------------------------------------------------
SECRET_NAME_RE='(TOKEN|SECRET|PASSWORD|PASSWD|API[_-]?KEY|PRIVATE[_-]?KEY|ACCESS[_-]?KEY|CREDENTIAL|AUTH)'
if echo "$COMMAND" | grep -qiE "\b(echo|printf)\b[^|&;]*\\\$\{?[A-Za-z0-9_]*${SECRET_NAME_RE}[A-Za-z0-9_]*\}?"; then
  block \
    "Echoing a secret-shaped environment variable is blocked — it would land in the transcript." \
    "Use the variable directly in the command that needs it, without printing it."
fi

# ---------------------------------------------------------------------------
# Block env | grep for secret-shaped names
# ---------------------------------------------------------------------------
if echo "$COMMAND" | grep -qiE '\b(env|printenv|set)\b[^|]*\|[^|]*\bgrep\b[^|]*(token|secret|password|passwd|api[_-]?key|private[_-]?key|access[_-]?key|credential|auth)'; then
  block \
    "Grepping the environment for secret-shaped names is blocked." \
    "If a specific value is needed, ask the user or use the variable directly in the downstream command."
fi

exit 0
