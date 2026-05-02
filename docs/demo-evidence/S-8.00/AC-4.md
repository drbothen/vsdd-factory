---
story_id: S-8.00
ac: AC-4
title: BC-anchor verification table produced for all 9 Tier 1 hooks
---

# AC-4: BC-anchor verification table produced for all 9 Tier 1 hooks

**Statement:** BC-anchor verification table produced for all 9 Tier 1 hooks (handoff-validator, pr-manager-completion-guard, track-agent-stop, update-wave-state-on-merge, validate-pr-review-posted, session-learning, warn-pending-wave-gate, track-agent-start, regression-gate). Table schema: Hook, BC ID(s), BC Title(s), Spec-Current Y/N, Gap-Found Y/N, Action-Needed.

## Evidence

### Audit table excerpt from `.factory/cycles/v1.0-brownfield-backfill/E-8-bc-anchor-table.md`

```markdown
| Hook | BC ID(s) | BC Title(s) | Spec-Current Y/N | Gap-Found Y/N | Action-Needed |
|------|----------|-------------|-----------------|--------------|---------------|
| handoff-validator.sh | BC-7.03.042, BC-7.03.043, BC-7.03.044 | handoff-validator: identity & registry binding; warns on empty subagent result; warns on suspiciously short result (<40 chars) | Y | N | None — BC-7.03.042/043/044 confirmed. Hook validates non-empty last_assistant_message via jq; BCs match source behavior |
| pr-manager-completion-guard.sh | BC-7.03.045, BC-7.03.046, BC-7.03.047, BC-7.03.048 | pr-manager-completion-guard: identity & registry binding; counts STEP_COMPLETE emissions (≥8 required); BLOCKED status is legitimate early exit; blocks with step-specific continuation hint | Y | N | None — BC-7.03.045/046/047/048 confirmed. Hook enforces 9-step PR lifecycle completion; on_error=block; BCs match source behavior |
| track-agent-stop.sh | BC-7.03.081, BC-7.03.082 | track-agent-stop: identity & registry binding; classifies result as ok/blocked/needs_context/done_with_concerns | Y | N | None — BC-7.03.081/082 confirmed. Hook emits agent.stop telemetry event via emit-event; on_error=continue; BCs match source behavior |
| update-wave-state-on-merge.sh | BC-7.03.083, BC-7.03.084, BC-7.03.085, BC-7.03.086 | update-wave-state-on-merge: identity & registry binding; scopes to pr-manager + successful merge signal; appends story to wave_data.stories_merged via python YAML; flips gate_status to pending when wave fully merged | Y | N | None — BC-7.03.083/084/085/086 confirmed. Hook requires jq + python3; updates wave state YAML on SubagentStop after merge; BCs match source behavior |
| validate-pr-review-posted.sh | BC-7.04.040, BC-7.04.041, BC-7.04.042, BC-7.04.043, BC-7.04.044 | validate-pr-review-posted: identity & registry binding; scopes to pr-reviewer / pr-review-triage agents; blocks when pr-review.md not written; blocks gh pr comment fallback; blocks when no formal review posted | Y | N | None — BC-7.04.040/041/042/043/044 confirmed. Hook enforces formal gh pr review verdict requirement; on_error=block; BCs match source behavior |
| session-learning.sh | BC-7.03.076, BC-7.03.077, BC-7.03.078 | session-learning: identity & registry binding; appends timestamped marker to .factory/sidecar-learning.md; skips when .factory/ absent | Y | N | None — BC-7.03.076/077/078 confirmed. Hook appends session-end learning marker on Stop event; on_error=continue; BCs match source behavior |
| warn-pending-wave-gate.sh | BC-7.03.091, BC-7.03.092 | warn-pending-wave-gate: identity & registry binding; stderr warning when any wave has gate_status: pending | Y | N | None — BC-7.03.091/092 confirmed. Hook warns at session end when pending wave gates detected; on_error=continue; BCs match source behavior |
| track-agent-start.sh | BC-7.03.079, BC-7.03.080 | track-agent-start: identity & registry binding; emits agent.start with subagent + best-effort story_id | Y | N | None — BC-7.03.079/080 confirmed. Hook emits agent.start telemetry on PreToolUse:Agent; on_error=continue; BCs match source behavior |
| regression-gate.sh | BC-7.01.003, BC-7.03.071, BC-7.03.072, BC-7.03.073, BC-7.03.074, BC-7.03.075 | regression-gate: fails when bash command interrupted (BC-7.01.003); identity & registry binding; matches 9 test runners; pass/fail derivation (exit_code priority, interrupted fallback); writes state file with status/timestamp/command; warns on pass→fail transition | Y | N | OQ-6 unresolved; BC-anchor confirmation deferred to S-8.09 security-reviewer audit (E-8 epic OQ-6). Existing BCs confirm current behavior. OQ-6 (subprocess capability profile audit) is a downstream gate for S-8.09 security-reviewer; it does not block S-8.00 close. |
```

### Verification: all 9 hooks present

All 9 Tier 1 hooks named in AC-4 are represented:
1. handoff-validator.sh — BC-7.03.042/043/044
2. pr-manager-completion-guard.sh — BC-7.03.045/046/047/048
3. track-agent-stop.sh — BC-7.03.081/082
4. update-wave-state-on-merge.sh — BC-7.03.083/084/085/086
5. validate-pr-review-posted.sh — BC-7.04.040/041/042/043/044
6. session-learning.sh — BC-7.03.076/077/078
7. warn-pending-wave-gate.sh — BC-7.03.091/092
8. track-agent-start.sh — BC-7.03.079/080
9. regression-gate.sh — BC-7.01.003, BC-7.03.071/072/073/074/075

**Result:** AC-4 SATISFIED. BC-anchor verification table covers all 9 Tier 1 hooks with required schema columns (Hook, BC ID(s), BC Title(s), Spec-Current Y/N, Gap-Found Y/N, Action-Needed).
