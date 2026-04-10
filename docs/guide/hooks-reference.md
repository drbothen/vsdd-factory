# Hooks Reference

The vsdd-factory plugin ships 10 hook scripts wired through `hooks.json`. Hooks fire automatically on tool use events, subagent completion, and session end. They enforce pipeline discipline without requiring manual intervention.

---

## Hook Summary

| Hook | Event | Matcher | What It Enforces | Blocking |
|------|-------|---------|-----------------|----------|
| `brownfield-discipline.sh` | PreToolUse | Edit\|Write | `.reference/` directories are read-only | Yes |
| `protect-vp.sh` | PreToolUse | Edit\|Write | Green Verification Properties are immutable | Yes |
| `protect-bc.sh` | PreToolUse | Edit\|Write | Green Behavioral Contracts are immutable | Yes |
| `red-gate.sh` | PreToolUse | Edit\|Write | TDD red-before-green discipline | Yes (when strict mode active) |
| `verify-git-push.sh` | PreToolUse | Bash | Warn before `git push`, block `--force` | Conditional |
| `check-factory-commit.sh` | PreToolUse | Bash | Remind about STATE.md after `.factory/` commits | No (advisory) |
| `purity-check.sh` | PostToolUse | Edit\|Write | Pure-core boundary -- no side effects in pure modules | No (warn-only) |
| `regression-gate.sh` | PostToolUse | Bash | Track test pass/fail transitions | No (telemetry) |
| `handoff-validator.sh` | SubagentStop | (all) | Subagent output is non-empty and structurally plausible | No (warn-only) |
| `session-learning.sh` | Stop | (all) | Append learning marker to `.factory/sidecar-learning.md` | No (non-blocking) |

---

## Hook Details

### brownfield-discipline.sh

**Event:** PreToolUse on Edit or Write

Blocks any edit or write to paths containing `/.reference/` or starting with `.reference/`. Reference codebases are cloned into `.reference/` during brownfield ingestion for analysis. Editing them corrupts the extraction and poisons downstream spec work.

**Debugging:** If a legitimate edit is blocked, verify the file path does not traverse a `.reference/` directory. Reference repos are read-only by design.

### protect-vp.sh

**Event:** PreToolUse on Edit or Write

Blocks edits to Verification Property files in `.factory/specs/verification-properties/` that have reached `Status: green`. Per SOUL.md principle 4 and spec-format rules, green VPs are immutable. To change a green VP, create a new VP that supersedes it and retire the old one.

**Response format:** Emits a PreToolUse JSON envelope with `permissionDecision`:
```json
{"hookSpecificOutput":{"hookEventName":"PreToolUse","permissionDecision":"deny"}}
```

**Debugging:** Read the VP file and check its `Status` field. If it shows `green`, the hook will deny edits. Create a new VP-NNN file instead of modifying the existing one.

### protect-bc.sh

**Event:** PreToolUse on Edit or Write

Companion to `protect-vp.sh`. Blocks edits to Behavioral Contract files in `.factory/specs/behavioral-contracts/BC-*.md` that have reached `Status: green`. Non-green contracts are freely editable.

**Response format:** Same `permissionDecision` envelope as `protect-vp.sh`.

**Debugging:** Same as protect-vp -- check the `Status` field. Draft and reviewed BCs allow edits; green BCs do not.

### red-gate.sh

**Event:** PreToolUse on Edit or Write

Enforces TDD red-before-green discipline when strict mode is active. Strict mode is opt-in: create `.factory/red-gate-state.json` with the shape `{"mode": "strict", "red": ["path/to/file.rs", ...]}`. When active, only files listed in the `red` array may be edited. Test files are always allowed.

When mode is `"off"` or the state file does not exist, the hook allows all edits.

**Debugging:** Check `.factory/red-gate-state.json`. If strict mode is active and your edit is blocked, add the file to the `red` array or switch mode to `"off"`.

### verify-git-push.sh

**Event:** PreToolUse on Bash

Intercepts `git push` commands. Blocks force pushes (`--force` or `-f`). For normal pushes, injects a reminder to ensure `cargo test`, `clippy`, and `fmt` are clean before pushing.

**Debugging:** If a push is blocked, check for `--force` in the command. Force pushes require removal of the flag.

### check-factory-commit.sh

**Event:** PreToolUse on Bash

Advisory hook that fires after `git commit` commands involving `.factory/`. If STATE.md was not part of the commit, it reminds the user to update STATE.md if a phase transition occurred. Non-blocking -- the commit proceeds regardless.

### purity-check.sh

**Event:** PostToolUse on Edit or Write

Enforces the pure-core boundary from SOUL.md. Files under `*/pure/**`, `*/core/**`, or ending in `_pure.rs` / `.pure.ts` are scanned for known side-effect patterns (I/O, network, global state mutation). Emits a warning to stderr when violations are found. Non-blocking by design -- architectural drift is surfaced, not enforced. The regression gate and CI catch hard failures.

### regression-gate.sh

**Event:** PostToolUse on Bash

Watches Bash commands that run tests (`cargo test`, `pytest`, `npm test`, `go test`, `just test`, `just ci`). Records pass/fail to `.factory/regression-state.json`. If the test suite transitions from pass to fail, emits a warning so the next edit can be informed. This is a telemetry hook consumed by the red-gate when strict mode is active.

### handoff-validator.sh

**Event:** SubagentStop

Validates that subagent output is non-empty and structurally plausible. Warns to stderr if the result is empty, whitespace-only, or under 40 characters (suspiciously short for any factory agent). An empty subagent result is a common silent failure -- the orchestrator would act on nothing and the pipeline would drift. Non-blocking -- the orchestrator receives the result and decides how to proceed.

### session-learning.sh

**Event:** Stop

At session end, appends a timestamped learning stub to `.factory/sidecar-learning.md`. The actual synthesis is done by the `/vsdd-factory:session-review` skill; this hook ensures a marker exists so nothing is silently lost when sessions end abruptly. Non-blocking and safe to fail.

---

## Hook Wiring

All hooks are configured in `plugins/vsdd-factory/hooks/hooks.json`. The wiring uses four event types:

| Event | When It Fires |
|-------|--------------|
| `PreToolUse` | Before a tool call executes. Can block (exit 2) or inject context (exit 0 with JSON). |
| `PostToolUse` | After a tool call completes. Cannot block -- advisory only. |
| `SubagentStop` | When a subagent finishes and returns its result. |
| `Stop` | When the session ends. |

Each hook has a 5-second timeout (10 seconds for `verify-git-push.sh`). All hooks require `jq` for JSON parsing of the tool input envelope.
