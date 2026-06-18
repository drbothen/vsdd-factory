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
/wave-handoff
```

No arguments required. All inputs are read from the real substrate:
- `.factory/stories/sprint-state.yaml` — story states, wave derivation
- `git rev-parse origin/develop` — `last_verified_develop_sha`
- `.factory/STATE.md` — `factory_lock_holder`, fallback `wave_id` derivation
- `.factory/specs/behavioral-contracts/` — `active_bcs` path resolution
- `.factory/stories/STORY-INDEX.md` — `next_wave_stories` story ID validation
- `.factory/hooks/precompact-flush-log` — `precompact_flush_sha` (three-state rule)

## Required Arguments

None. All field values are derived from external ground truth at invocation time.
No field may be hardcoded or supplied from in-context memory (BC-5.41.001 INV1).

## Optional Arguments

None.

## Behavior Overview

1. Parse sprint-state.yaml — derive `wave_id`, classify stories by terminal/pending/draft/broken state
2. Detect EPIC-COMPLETE (all stories terminal) or BrokenSprintState (non-terminal, non-pending/draft)
3. Gather all 9 base HANDOFF.md fields with anti-fabrication cross-checks
4. Write HANDOFF.md (with `epic_status: complete` on EPIC-COMPLETE wave only)
5. On non-EPIC-COMPLETE wave: write wave-state.yaml with 6 required fields
6. Commit HANDOFF.md + wave-state.yaml atomically to factory-artifacts
   - Commit message: `HANDOFF wave-<N> <ISO-timestamp>` (exact format per BC-5.41.001 INV1)
7. On EPIC-COMPLETE: emit stdout announcement; do NOT write wave-state.yaml

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
  pre-flight validation in `main()` and by `write_wave_state` in `lib/write-wave-state.sh`)
- `StoryIndexMissing` — STORY-INDEX.md absent at wave-close when next_wave_stories is non-empty;
  emitted by pre-flight in `main()` and by `write_wave_state` (BC-5.41.002 PC2 precondition 2)
- `PrecompactShaMismatch` — precompact_flush_sha null/wrong when precompact-flush-log is present+valid;
  emitted by `lib/write-handoff.sh`
- `NoWaveIdSubstrate` — neither sprint-state ordinal nor STATE.md current_step yields a wave_id;
  emitted by `lib/parse-sprint-state.sh` `derive_wave_id()`
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
