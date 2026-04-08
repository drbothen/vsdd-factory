# Dark Factory Orchestrator Heartbeat

Run this checklist every heartbeat. Only alert if something needs attention.
If everything is healthy, reply HEARTBEAT_OK.

## 1. Pipeline Health

- Read `.factory/STATE.md` in the target project (use the resolved project path from your session)
- If a phase has been `in_progress` for more than 60 minutes with no subagent activity, alert: `STALE PIPELINE: [phase] stuck for [duration]`
- If STATE.md shows `blocked` or `failed`, alert with the reason

## 2. Subagent Status

- Run `/subagents list` to check active subagent runs
- If any subagent has been running longer than `runTimeoutSeconds` (default 7200s), alert: `LONG-RUNNING AGENT: [agentId] running for [duration]`
- If a subagent completed but its result wasn't processed (check session context for unhandled announce events), re-read the result and act on it

## 3. Worktree Health

- If a target project is connected, verify `.factory/` worktree exists and is on the correct branch
- If `.factory/.git` is missing or corrupt, alert: `WORKTREE UNHEALTHY: [details]`
- Do NOT run git commands yourself — just check file existence

## 4. Cost Tracking

- If `<project>/.factory/cost-summary.md` exists, read the total spend
- If spend exceeds 80% of `budgetMaxUsd` from merge-config.yaml, alert: `BUDGET WARNING: [spent]/[budget] ([pct]%)`
- If spend exceeds 95%, alert: `BUDGET CRITICAL: approaching limit`

## 5. Human Approval Check

- If STATE.md shows `waiting_human_approval` and the timestamp is older than 4 hours, alert: `AWAITING APPROVAL: [phase] has been waiting for [duration]. Nudge the human.`

## 6. Quality Gate Drift

- If the pipeline is in Phase 3+ and `.factory/specs/` exists, spot-check that STORY-INDEX.md and ARCH-INDEX.md are present and non-empty
- If any required index file is missing, alert: `SPEC DRIFT: [file] missing from .factory/specs/`

## Rules

- Do NOT spawn subagents during heartbeat — keep it lightweight (reads only)
- Do NOT write files during heartbeat — only read and report
- If multiple alerts, combine them into a single message
- Prefix all alerts with `[HEARTBEAT ALERT]` so the human can filter
