# Manual Session Test: AC-005 — Operator Confirmation Pause

**Story:** S-18.03 v1.5 — rehydrate-wave skill  
**BC:** BC-6.24.001 postcondition 5 — operator confirmation required before proceeding  
**VP:** VP-088 §2 (manual session test for LLM presentation step)  
**Phase:** F3 acceptance test (manual; cannot be automated via bats)

---

## Purpose

AC-005 tests the operator confirmation pause behavior of the `rehydrate-wave`
skill in a live Claude Code session. This behavior cannot be automated via
bats because it requires an active LLM session to verify that:

1. The skill presents the injected spec list to the operator.
2. The skill pauses and outputs a confirmation prompt.
3. The session does NOT start any pipeline work before the operator has confirmed.

---

## Prerequisites

1. Story S-18.03 T-2 (implementer) is complete: `plugins/vsdd-factory/skills/rehydrate-wave/SKILL.md` exists.
2. A valid `wave-state.yaml` is committed to the `factory-artifacts` branch on the target repo.
3. All 9 bats Red Gate tests pass (skills implemented, not just stubs).
4. A fresh Claude Code session has been started (no prior pipeline context loaded).

---

## Test Procedure

### Step 1: Prepare the environment

Verify `wave-state.yaml` exists on `factory-artifacts`:

```bash
git show factory-artifacts:wave-state.yaml
```

Expected: the YAML manifest is printed with at least one story and its `spec_files`.

### Step 2: Invoke the skill

In a fresh Claude Code session (after `/clear` or at session start), type:

```
/rehydrate-wave
```

**Bare invocation behavior (F-P1-005):** When invoked with no arguments, the skill resolves
`REPO_DIR=.` (current working directory) and `ARTIFACTS_WT=.factory` by default. This means
the skill will run `git show factory-artifacts:wave-state.yaml` using the current working
directory as the main repo root and will look for the factory-artifacts worktree at `.factory`.
This is the standard production invocation pattern when Claude Code is opened at the repository
root. If the current directory is NOT the repo root, pass explicit arguments:

```
REPO_DIR=/path/to/repo ARTIFACTS_WT=/path/to/repo/.factory /path/to/rehydrate-wave.sh
```

Or use the skill with the worktree/repo flags if the Claude Code session was started from
a different working directory.

### Step 3: Observe skill output

The skill must:

1. Print a human-readable list of injected spec files (BC-6.24.001 Inv4 transparency).
2. Print the `INJECTED_FILE_COUNT=<n>` machine-stable sentinel line.
3. **PAUSE** and output a confirmation prompt asking the operator to proceed.

Example expected output (format may vary; key elements are required):

```
Rehydrating session context from factory-artifacts:wave-state.yaml...

INJECTED_FILE_COUNT=5

Injected files:
  - .factory/STATE.md
  - .factory/stories/S-18.02-foo.md
  - .factory/specs/architecture/ARCH-INDEX.md
  - ... (other listed files)

Confirm rehydration: the above files will be loaded as context for this session.
Please confirm to proceed (type 'yes' or press Enter):
```

### Step 4: Verify the pause

**PASS criteria — confirm all of the following before typing anything:**

- [ ] The injected file list is displayed before the prompt.
- [ ] The confirmation prompt is visible and clearly requests operator input.
- [ ] The session has NOT dispatched any downstream pipeline skill or agent.
- [ ] No story implementation, spec generation, or tool dispatch has occurred.

### Step 5: Confirm and verify continuation

Type `yes` (or the prompt-specified confirmation token) at the prompt.

**PASS criteria after confirmation:**

- [ ] The session proceeds to load the listed spec files into context.
- [ ] The session does NOT load any spec files from prior waves not in the manifest.
- [ ] No `RehydrationError` is emitted.

---

## Failure Criteria

Mark AC-005 as FAILED if any of the following occur:

- The skill dispatches a downstream task before the operator confirms.
- No confirmation prompt is shown (skill completes silently).
- The session loads spec files not listed in `wave-state.yaml`.
- The prompt is shown but the session proceeds without waiting for input.

---

## Test Evidence

Record the following after running this test:

| Item | Value |
|------|-------|
| Date tested | |
| Tester | |
| Session type (fresh/resumed) | |
| wave-state.yaml wave_id | |
| INJECTED_FILE_COUNT value | |
| Confirmation prompt text (verbatim) | |
| Outcome (PASS / FAIL) | |
| Notes | |

---

## Traceability

- BC-6.24.001 postcondition 5: "After presenting the injected spec list to the operator,
  the skill pauses and requests confirmation before the session proceeds with any pipeline
  work. The confirmation step ensures the operator can verify the rehydration scope."
- VP-088 §2: "The step where the skill presents injected spec content to the LLM session
  cannot be automated via bats (it requires an active Claude Code session). This step
  requires a manual session test at F3."
- S-18.03 T-4: "Write manual session test script documenting F3 acceptance test for
  AC-005 (operator confirmation pause in live Claude Code session)."
