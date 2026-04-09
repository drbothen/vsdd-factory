---
name: wave-status
description: Report current wave readiness from `.factory/stories/sprint-state.yaml`. Use to answer "what wave are we on, and is it ready to ship?"
---

# Wave Status

Wraps `${CLAUDE_PLUGIN_ROOT}/bin/wave-state` to report pipeline wave state at a glance.

## Procedure

1. Run `bin/wave-state summary` — prints `wave=N/M state=<path>`.
2. Run `bin/wave-state current` — the active wave number.
3. Run `bin/wave-state stories` — list stories in the current wave.
4. Run `bin/wave-state ready` — readiness report: `wave=N ready=X total=Y`. Exit 0 if all stories are `status: ready`.
5. Report:
   - Current wave and total wave count
   - Story list for the current wave
   - Readiness breakdown (ready / in-progress / blocked / not-ready)
   - Recommendation: if all ready → run `/vsdd-factory:wave-gate`; if not → list blockers

Do not mutate `sprint-state.yaml`. This skill is read-only. Wave advancement is the orchestrator's job via `/vsdd-factory:run-phase` or `/vsdd-factory:wave-gate`.
