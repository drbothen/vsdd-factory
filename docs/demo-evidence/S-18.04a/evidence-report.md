---
story_id: S-18.04a
title: "precompact-flush Native WASM Plugin Core — Demo Evidence"
recorded_by: vsdd-factory:demo-recorder
recording_tool: VHS 0.11.0
recorded_at: 2026-06-23
---

# Demo Evidence — S-18.04a: precompact-flush Native WASM Plugin Core

## Coverage Summary

| Recording | ACs Covered | Result |
|-----------|-------------|--------|
| AC-005-positive-flush | AC-004, AC-005 | PASS |
| AC-007-log-entry | AC-007 | PASS |
| AC-009-push | AC-001, AC-009 | PASS |
| AC-017-durability-degraded | AC-017 | PASS |
| AC-014-hermetic-git-only | AC-001, AC-014 | PASS |

All five acceptance criteria families have both success and error path coverage.

---

## Segment 1 — Positive Flush (AC-004 / AC-005)

**Tape:** `AC-005-positive-flush.tape`
**GIF:** `AC-005-positive-flush.gif`
**WebM:** `AC-005-positive-flush.webm`

**What it demonstrates:**
- Sets up a git fixture with a factory-artifacts worktree mounted at `<project>/.factory`
- Stages a pending change to STATE.md
- Shows `git log` on the factory-artifacts worktree **before** flush (HEAD at init commit)
- Pipes a `PreCompact` JSON envelope to `factory-dispatcher`
- Shows `git log` **after** flush: a new commit appears — `PreCompact flush <cycle>/<step> <ISO-timestamp>`
- Asserts HEAD advanced (distinct before/after SHAs)

**BC / ADR anchors:** BC-7.07.001 §PC3 (flush-commit-lands), §INV3 (positive-flush-completion), ADR-028 §Decision 3

---

## Segment 2 — Flush Log Entry (AC-007)

**Tape:** `AC-007-log-entry.tape`
**GIF:** `AC-007-log-entry.gif`
**WebM:** `AC-007-log-entry.webm`

**What it demonstrates:**
- Shows `precompact-flush-log` does **not exist** before the first flush
- Runs the PreCompact flush
- Shows the log file after flush: one newline-terminated line in the format  
  `<ISO-timestamp> <SHA> <cycle>/<step> commit`
- Verifies the appended SHA matches the factory-artifacts HEAD

**BC / ADR anchors:** BC-7.07.001 §PC6b (append-log-entry), ADR-028 §Decision 3

---

## Segment 3 — Remote Push (AC-009)

**Tape:** `AC-009-push.tape`
**GIF:** `AC-009-push.gif`
**WebM:** `AC-009-push.webm`

**What it demonstrates:**
- Shows the bare remote `factory-artifacts` ref **before** flush
- Runs the PreCompact flush
- Shows the bare remote ref **after** flush — it now matches local HEAD
- Confirms `PATH` + `SSH_AUTH_SOCK` in `env_allow` is sufficient for git push to a local bare remote

**BC / ADR anchors:** BC-7.07.001 §PC5 (push-to-remote), ADR-028 §Decision 1 (env_allow includes PATH)

---

## Segment 4 — DURABILITY DEGRADED Advisory (AC-017)

**Tape:** `AC-017-durability-degraded.tape`
**GIF:** `AC-017-durability-degraded.gif`
**WebM:** `AC-017-durability-degraded.webm`

**What it demonstrates (error path):**
- Sets up `CLAUDE_PROJECT_DIR` as a **plain directory** (not a git repo — no factory-artifacts worktree)
- Sends the same PreCompact envelope to the dispatcher
- Shows the advisory output: `DURABILITY DEGRADED — git worktree list command failed (exit 128); factory-artifacts worktree cannot be discovered; flush SKIPPED`
- Verifies exit code is **0** (non-blocking advisory, not a hard failure)

**BC / ADR anchors:** BC-7.07.001 §PC8 (degrade-gracefully), ADR-028 §Decision 15 (DURABILITY DEGRADED advisory on worktree-not-found)

---

## Segment 5 — Hermetic / git-only (AC-014 / AC-001)

**Tape:** `AC-014-hermetic-git-only.tape`
**GIF:** `AC-014-hermetic-git-only.gif`
**WebM:** `AC-014-hermetic-git-only.webm`

**What it demonstrates:**
- Shows the registry `binary_allow = ["git"]` — bash is not listed
- Runs the PreCompact flush and captures dispatcher output
- Confirms `sync_plugins=1` (plugin was invoked via WASM, not a bash subprocess)
- Confirms no `"binary":"bash"` in dispatcher output
- Confirms `binary_allow` contains `"git"` and does NOT contain `"bash"`

**BC / ADR anchors:** BC-7.07.001 §PC1 (native-wasm-only), §INV1 (no-bash-subprocess), ADR-028 §Decision 2 (binary_allow=["git"] only)

---

## Toolchain

- **Recording:** VHS 0.11.0 (`/opt/homebrew/bin/vhs`)
- **Terminal font:** Menlo (system default on macOS)
- **Theme:** Dracula
- **Shell:** bash
- **Dispatcher under test:** `target/release/factory-dispatcher` (commit 3127dd1b)
- **WASM under test:** `plugins/vsdd-factory/hook-plugins/precompact-flush.wasm`
- **Fixture pattern:** Git worktree topology from `plugins/vsdd-factory/tests/precompact-flush-native.bats` `_init_git_fixture`

## POLICY 10 Compliance

All files reside under `docs/demo-evidence/S-18.04a/` (story-scoped subfolder).
No flat files at `docs/demo-evidence/*.md`.
