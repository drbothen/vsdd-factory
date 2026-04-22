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

# Scope: only worker-agent dispatches that do story work
case "$SUBAGENT" in
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

  echo "" >&2
  echo "wave-gate-prerequisite: BLOCKED." >&2
  echo "" >&2
  echo "You are dispatching $SUBAGENT for $STORY_ID (wave=$TARGET_WAVE)," >&2
  echo "but an earlier wave has not passed its integration gate:" >&2
  echo "" >&2
  echo "  $BLOCKING_WAVE: gate_status=$BLOCKING_STATUS" >&2
  echo "" >&2
  echo "Before dispatching $TARGET_WAVE work, do ONE of:" >&2
  echo "" >&2
  echo "1. Run the wave integration gate for $BLOCKING_WAVE:" >&2
  echo "   Invoke /vsdd-factory:wave-gate OR spawn the reviewer agents" >&2
  echo "   against the wave's merged commits. Fix any blocking findings." >&2
  echo "   Then update .factory/wave-state.yaml gate_status: passed." >&2
  echo "" >&2
  echo "2. If the wave has no new behavioral surface (e.g., pure docs)," >&2
  echo "   set gate_status: deferred with a rationale field." >&2
  echo "" >&2
  echo "Do NOT silently skip. Do NOT manually edit wave-state.yaml" >&2
  echo "without running the checks." >&2

  exit 2
fi

exit 0
