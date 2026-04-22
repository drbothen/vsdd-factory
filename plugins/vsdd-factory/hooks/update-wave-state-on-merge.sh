#!/bin/bash
# update-wave-state-on-merge.sh — SubagentStop hook on pr-manager
#
# When pr-manager successfully completes with a merge, auto-updates
# wave-state.yaml: appends merged story to stories_merged, flips
# gate_status to pending when all stories in the wave have merged.
#
# Trigger: SubagentStop on pr-manager subagents.
# Exit 0 always (advisory — never blocks pr-manager completion).
#
# Requires: jq, python3 (for YAML read/write).
# Deterministic, <500ms, no LLM.

set -euo pipefail

if ! command -v jq &>/dev/null || ! command -v python3 &>/dev/null; then
  exit 0
fi

INPUT=$(cat)
AGENT=$(echo "$INPUT" | jq -r '.agent_type // .subagent_name // "unknown"')
RESULT=$(echo "$INPUT" | jq -r '.last_assistant_message // .result // empty')

# Scope: only pr-manager
case "$AGENT" in
  *pr-manager*|*pr_manager*) ;;
  *) exit 0 ;;
esac

# Only act on successful merges — look for merge confirmation
if ! echo "$RESULT" | grep -qiE "STEP_COMPLETE: step=8.*status=ok|merged|squash.*merge"; then
  exit 0
fi

# Extract story ID from the result
STORY_ID=$(echo "$RESULT" | grep -oE 'S-[0-9]+\.[0-9]+' | head -1 || true)
if [[ -z "$STORY_ID" ]]; then
  # Try STORY-NNN format
  STORY_ID=$(echo "$RESULT" | grep -oE 'STORY-[0-9]+' | head -1 || true)
fi
if [[ -z "$STORY_ID" ]]; then
  exit 0  # can't identify which story merged
fi

# Find wave-state.yaml
WAVE_STATE=""
if [[ -f ".factory/wave-state.yaml" ]]; then
  WAVE_STATE=".factory/wave-state.yaml"
fi
if [[ -z "$WAVE_STATE" ]] || [[ ! -f "$WAVE_STATE" ]]; then
  exit 0  # no wave-state file
fi

# Update wave-state.yaml via python3
python3 -c "
import yaml, sys, datetime

wave_state_path = '$WAVE_STATE'
story_id = '$STORY_ID'

with open(wave_state_path) as f:
    state = yaml.safe_load(f)

if not state or 'waves' not in state:
    sys.exit(0)

changed = False
for wave_name, wave_data in state['waves'].items():
    stories = wave_data.get('stories', []) or []
    merged = wave_data.get('stories_merged', []) or []

    if story_id in stories and story_id not in merged:
        merged.append(story_id)
        wave_data['stories_merged'] = merged
        changed = True

        # Check if all stories in wave are now merged
        if set(stories) == set(merged):
            if wave_data.get('gate_status') in ('not_started', None):
                wave_data['gate_status'] = 'pending'
                state['next_gate_required'] = wave_name
                sys.stderr.write(f'update-wave-state-on-merge: all stories in {wave_name} merged. gate_status → pending.\n')
                sys.stderr.write(f'  Run the wave integration gate before starting the next wave.\n')
        break

if changed:
    with open(wave_state_path, 'w') as f:
        yaml.dump(state, f, default_flow_style=False, sort_keys=False, allow_unicode=True)
" 2>&2 || true

exit 0
