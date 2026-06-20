---
name: wave-handoff
description: Write a verified HANDOFF.md (9 base required fields) and an atomically co-committed wave-state.yaml (6 required fields) to the factory-artifacts branch at wave close. Provides the lossless wave-boundary checkpoint required by BC-5.41.001 and BC-5.41.002.
argument-hint: ""

allowed-tools: Read, Write, Edit, Bash, Glob, Grep
---

# Wave Handoff

Produce HANDOFF.md and wave-state.yaml atomically on the factory-artifacts branch at
wave close, satisfying BC-5.41.001 (HANDOFF.md with 9 base required fields +
anti-fabrication cross-checks) and BC-5.41.002 (wave-state.yaml with 6 required
fields derived from sprint-state.yaml — no RAG).

## Invocation Contract

```
/wave-handoff \
  --artifacts-worktree <path> \
  --sprint-state <path> \
  --state-md <path> \
  --bc-dir <path> \
  [--precompact-flush-log <path>]
```

All four required arguments must be supplied explicitly (or via env-var fallbacks;
see below). No field may be hardcoded or supplied from in-context memory
(BC-5.41.001 INV1). All inputs are derived from external ground truth at invocation
time.

Production invocation example (all paths relative to repo root):

```
/wave-handoff \
  --artifacts-worktree .factory \
  --sprint-state .factory/stories/sprint-state.yaml \
  --state-md .factory/STATE.md \
  --bc-dir .factory/specs/behavioral-contracts
```

## Required Arguments

All four of the following arguments are mandatory. The skill hard-errors
(`exit 1`) if any is absent and the corresponding env-var fallback is also unset.

| Argument | Env-var fallback | Production path | Purpose |
|----------|-----------------|-----------------|---------|
| `--artifacts-worktree <path>` | `ARTIFACTS_WT` | `.factory` | Root of the factory-artifacts worktree. `HANDOFF.md` and `wave-state.yaml` are written here; git commit targets `git -C "$ARTIFACTS_WT"`. |
| `--sprint-state <path>` | `SPRINT_STATE_YAML` | `.factory/stories/sprint-state.yaml` | Path to `sprint-state.yaml`; used for story classification, wave derivation, and next-wave story enumeration. |
| `--state-md <path>` | `STATE_MD_PATH` | `.factory/STATE.md` | Path to `STATE.md`; used for `factory_lock_holder` and fallback `wave_id` derivation from `current_step: "pass-N"`. |
| `--bc-dir <path>` | `BC_DIR` | `<ARTIFACTS_WT>/specs/behavioral-contracts` | Directory scanned (recursively) for active BC `.md` files; populates `active_bcs` in `HANDOFF.md`. See production invocation example above for the concrete path. |

## Optional Arguments

| Argument | Env-var fallback | Default | Purpose |
|----------|-----------------|---------|---------|
| `--precompact-flush-log <path>` | `PRECOMPACT_FLUSH_LOG` | `${ARTIFACTS_WT}/hooks/precompact-flush-log` | Path to the precompact flush log; used for the three-state `precompact_flush_sha` rule (BC-5.41.001 PC5). |

## Agent Steps (ADR-026 §Decision 8 — agent-orchestrated subcommand model)

These are GENUINE execution steps for the agent. The agent invokes each Bash subcommand in
sequence. The Write tool step (S2) is what causes the PostToolUse
`validate-wave-handoff-completeness` gate to fire.

**S1 (Bash — emit-handoff):**
```
bash wave-handoff.sh \
  --artifacts-worktree <ARTIFACTS_WT> \
  --sprint-state <SPRINT_STATE_YAML> \
  --state-md <STATE_MD_PATH> \
  --bc-dir <BC_DIR> \
  --emit-handoff
```
Assembles the complete HANDOFF.md payload with full anti-fabrication cross-checks and emits
it to stdout. No file is written to disk. Capture the stdout output for S2.

**S2 (Agent Write tool — HANDOFF.md):**
Write the captured payload to `${ARTIFACTS_WT}/HANDOFF.md` using the Claude Code Write tool.
This is the step that causes the PostToolUse `validate-wave-handoff-completeness` gate to fire.
Do NOT use bash redirection to write HANDOFF.md — the Write tool call is mandatory.

**S3 (Agent verifies):**
Confirm the written file content is byte-identical to the `--emit-handoff` payload before
proceeding. If the content differs, surface a hard error identifying the discrepancy and do
not proceed to S4/S5.

**S4 (Bash — emit-wave-state, skip on EPIC-COMPLETE):**
```
bash wave-handoff.sh \
  --artifacts-worktree <ARTIFACTS_WT> \
  --sprint-state <SPRINT_STATE_YAML> \
  --state-md <STATE_MD_PATH> \
  --bc-dir <BC_DIR> \
  --emit-wave-state
```
Writes wave-state.yaml to `${ARTIFACTS_WT}/wave-state.yaml` via bash. On EPIC-COMPLETE
(all stories terminal), this step exits 0 without writing wave-state.yaml — skip to S5.

**S5 (Bash — commit):**
```
bash wave-handoff.sh \
  --artifacts-worktree <ARTIFACTS_WT> \
  --sprint-state <SPRINT_STATE_YAML> \
  --state-md <STATE_MD_PATH> \
  --bc-dir <BC_DIR> \
  --commit
```
Creates ONE atomic git commit on factory-artifacts. On HAS-NEXT-WAVE, stages both
HANDOFF.md (written by S2 Write tool) and wave-state.yaml (written by S4) in a single
commit. On EPIC-COMPLETE, stages HANDOFF.md alone and removes any stale wave-state.yaml.
Aborts with `HandoffFileAbsent` if HANDOFF.md is absent (EC-017).

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success — HANDOFF.md (and wave-state.yaml for non-EPIC-COMPLETE) written and committed |
| 1 | BrokenSprintState — non-terminal stories present but no pending/draft entries |
| 1 | Hard field error — required field absent or anti-fabrication check failed |
| 1 | PrecompactShaMismatch — precompact-flush-log present+valid but SHA mismatch or malformed |
| 1 | NoWaveIdSubstrate — neither sprint-state ordinal nor STATE.md pass-N yields a wave_id |
| 1 | StoryIndexMissing — STORY-INDEX.md absent at wave-close with non-empty next_wave_stories |
| 1 | active_bcs empty at wave close |
| 1 | Atomic commit failure — git push to factory-artifacts failed |

## Error Codes (canonical)

- `BrokenSprintState` — sprint-state.yaml has non-terminal stories but no pending/draft entries
- `AntiFabricationFailed` — a field cross-check against external ground truth failed (emitted by
  pre-flight validation in `cmd_emit_handoff()` and by `write_wave_state` in `lib/write-wave-state.sh`)
- `StoryIndexMissing` — STORY-INDEX.md absent at wave-close when next_wave_stories is non-empty;
  emitted by pre-flight in `cmd_emit_handoff()` and by `write_wave_state` (BC-5.41.002 PC2 precondition 2)
- `PrecompactShaMismatch` — precompact_flush_sha null/wrong when precompact-flush-log is present+valid;
  emitted by `lib/write-handoff.sh`
- `NoWaveIdSubstrate` — neither sprint-state ordinal nor STATE.md current_step yields a wave_id;
  emitted by `lib/parse-sprint-state.sh` `derive_wave_id()`
- `HandoffWriteToolUnavailable` — `HANDOFF_WRITE_TOOL_UNAVAILABLE=1` env set or Write tool is
  unavailable; `--emit-handoff` hard-errors and MUST NOT fall back to bash redirection (BC-5.41.001 EC-016)
- `HandoffFileAbsent` — `--commit` invoked but HANDOFF.md absent at `${ARTIFACTS_WT}/HANDOFF.md`;
  aborting atomic commit (BC-5.41.001 EC-017); fires on both HAS-NEXT-WAVE and EPIC-COMPLETE paths
- `UnexpectedEpicStatus` — epic_status present on a non-final wave; emitted by the downstream WASM
  gate (S-18.02), NOT by this skill
- `MissingEpicStatus` — epic_status absent on an EPIC-COMPLETE wave; emitted by the downstream WASM
  gate (S-18.02), NOT by this skill

## Forbidden Dependencies

This skill MUST NOT shell out to Python, jq, or any language runtime beyond bash.
All logic uses POSIX-compatible bash with standard UNIX tools (grep, awk, sort, git).

## Architecture Anchors

- BC-5.41.001 (HANDOFF.md schema + anti-fabrication cross-checks)
- BC-5.41.002 (wave-state.yaml curated manifest — no RAG)
- ADR-026 §Decision 2 (HANDOFF.md 9-field schema)
- ADR-026 §Decision 4 (wave-state.yaml minimum required fields)
- S-18.01 (implementation story)
