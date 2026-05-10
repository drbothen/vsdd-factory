#!/bin/bash
# validate-wave-gate-prerequisite.sh — PreToolUse hook on Agent dispatches
#
# Fires when orchestrator spawns a worker subagent. Blocks Wave N+1
# dispatch if Wave N gate is pending. Prevents the FM4 pattern where
# documentation says "run the gate" but nothing mechanically enforces it.
#
# Trigger: PreToolUse on Agent tool invocations.
# Exit 0 on pass (gate passed, no wave-state, can't determine wave).
# Exit 2 blocks the dispatch with instructions to run the pending gate.
#
# Requires: jq, python3 (for YAML parsing).
# Deterministic, <500ms, no LLM.

set -euo pipefail

# Source canonical block-message helper (provides block_pre).
_SELF_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
_BLOCK_SH="${CLAUDE_PLUGIN_ROOT:+${CLAUDE_PLUGIN_ROOT}/hooks/lib/block.sh}"
_BLOCK_SH="${_BLOCK_SH:-${_SELF_DIR}/lib/block.sh}"
# shellcheck source=lib/block.sh disable=SC1091
if [ -f "$_BLOCK_SH" ]; then source "$_BLOCK_SH"; fi

if ! command -v jq &>/dev/null; then
  exit 0
fi

if ! command -v python3 &>/dev/null; then
  exit 0  # graceful degradation — can't parse YAML without python3
fi

INPUT=$(cat)
TOOL=$(echo "$INPUT" | jq -r '.tool_name // ""')

# Scope: only Agent tool dispatches
if [[ "$TOOL" != "Agent" ]]; then
  exit 0
fi

SUBAGENT=$(echo "$INPUT" | jq -r '.tool_input.subagent_type // ""')
PROMPT=$(echo "$INPUT" | jq -r '.tool_input.prompt // ""')

# Scope split: adversary dispatches go through SHA-currency hygiene
# (state-manager burst integrity check). Worker-agent dispatches go
# through the gate-prerequisite check (prior-wave gate must be passed).
# Other subagents pass through.
case "$SUBAGENT" in
  *adversary*)
    # Locate verify-sha-currency.sh in the same way we find
    # wave-state.yaml: prefer cwd, then project path extracted from
    # `cd <path> &&` in the prompt.
    SHA_HOOK=""
    if [[ -x ".factory/hooks/verify-sha-currency.sh" ]]; then
      SHA_HOOK=".factory/hooks/verify-sha-currency.sh"
      SHA_PROJECT_ROOT="$PWD"
    else
      PROJECT_PATH=$(echo "$PROMPT" | grep -oE 'cd [^ ]+' | head -1 | sed 's/^cd //' || true)
      if [[ -n "$PROJECT_PATH" ]] && [[ -x "$PROJECT_PATH/.factory/hooks/verify-sha-currency.sh" ]]; then
        SHA_HOOK="$PROJECT_PATH/.factory/hooks/verify-sha-currency.sh"
        SHA_PROJECT_ROOT="$PROJECT_PATH"
      fi
    fi

    if [[ -z "$SHA_HOOK" ]]; then
      # Project hasn't installed verify-sha-currency.sh yet; skip.
      # Operators opt in by copying templates/verify-sha-currency.sh
      # into .factory/hooks/.
      exit 0
    fi

    if bash "$SHA_HOOK" --project-root "$SHA_PROJECT_ROOT" 2>&1; then
      # Hook returned 0 (PASS or PASS-with-WARN). Allow the dispatch.
      exit 0
    fi

    # Hook returned non-zero — block the dispatch with diagnostic.
    block_pre "validate-wave-gate-prerequisite" \
      "SHA currency check failed — factory-artifacts hygiene is dirty (stale SHA cites, multi-commit chains, in-progress narrative, or cross-record drift)" \
      "Resolve with: bash $SHA_HOOK; if multi-commit chain use: git -C .factory reset --soft HEAD~N then redo Stage 1+2 of burst protocol" \
      "wave_gate_blocking"
    ;;
  *test-writer*|*implementer*|*demo-recorder*|*pr-manager*|*devops-engineer*) ;;
  *) exit 0 ;;
esac

# Extract story ID from prompt (e.g., "S-6.07", "S-0.02")
STORY_ID=$(echo "$PROMPT" | grep -oE 'S-[0-9]+\.[0-9]+' | head -1 || true)
if [[ -z "$STORY_ID" ]]; then
  exit 0  # can't determine wave; skip check
fi

# Find wave-state.yaml — check cwd first, then extract path from prompt
WAVE_STATE=""
if [[ -f ".factory/wave-state.yaml" ]]; then
  WAVE_STATE=".factory/wave-state.yaml"
else
  # Try to extract project path from "cd <path> &&" in prompt
  PROJECT_PATH=$(echo "$PROMPT" | grep -oE 'cd [^ ]+' | head -1 | sed 's/^cd //' || true)
  if [[ -n "$PROJECT_PATH" ]] && [[ -f "$PROJECT_PATH/.factory/wave-state.yaml" ]]; then
    WAVE_STATE="$PROJECT_PATH/.factory/wave-state.yaml"
  fi
fi

if [[ -z "$WAVE_STATE" ]] || [[ ! -f "$WAVE_STATE" ]]; then
  exit 0  # no wave-state file = project hasn't opted in
fi

# Use python3 for YAML parsing: find story's wave and check predecessors
BLOCKING=$(python3 -c "
import yaml, sys

with open('$WAVE_STATE') as f:
    state = yaml.safe_load(f)

if not state or 'waves' not in state:
    sys.exit(0)

waves = state['waves']
wave_names = list(waves.keys())
story_id = '$STORY_ID'

# Find which wave this story belongs to
target_wave = None
for name, data in waves.items():
    stories = data.get('stories', []) or []
    if story_id in stories:
        target_wave = name
        break

if not target_wave:
    sys.exit(0)  # story not in any wave; skip

# Check all waves BEFORE target for unpassed gates
target_idx = wave_names.index(target_wave)
for i in range(target_idx):
    name = wave_names[i]
    status = waves[name].get('gate_status', 'unknown')
    if status not in ('passed', 'deferred'):
        print(f'{name}:{status}')
        sys.exit(0)
" 2>/dev/null || true)

if [[ -n "$BLOCKING" ]]; then
  BLOCKING_WAVE="${BLOCKING%%:*}"
  BLOCKING_STATUS="${BLOCKING#*:}"

  # Determine the story's target wave for the message
  TARGET_WAVE=$(python3 -c "
import yaml, sys
with open('$WAVE_STATE') as f:
    state = yaml.safe_load(f)
for name, data in state['waves'].items():
    if '$STORY_ID' in (data.get('stories', []) or []):
        print(name); break
" 2>/dev/null || true)

  block_pre "validate-wave-gate-prerequisite" \
    "Earlier wave's gate has not passed: $BLOCKING_WAVE (gate_status=$BLOCKING_STATUS) must pass before dispatching $STORY_ID ($TARGET_WAVE)" \
    "Resolve $BLOCKING_WAVE integration gate first: invoke /vsdd-factory:wave-gate, fix blocking findings, then update wave-state.yaml gate_status: passed" \
    "wave_gate_blocking"
fi

exit 0
