# Demo Evidence Report — S-18.03: rehydrate-wave skill

**Story:** S-18.03 v1.7 — rehydrate-wave skill — git-sourced scoped rehydration + wave-reset SKILL.md  
**BC gate:** BC-6.24.001 v1.10  
**VP:** VP-088 v1.1  
**Capture tool:** VHS v0.11.0 (terminal recording)  
**Product type:** CLI/bash skill  
**Recorded:** 2026-06-25  

---

## Tool Selection

**VHS** was used for all recordings (`which vhs` → `/opt/homebrew/bin/vhs`). This is a CLI/bash skill deliverable — VHS terminal recordings are the correct tool per the demo-recorder protocol. `asciinema` is not installed; `Wait+Line` is unsupported in VHS v0.11.0 (times out), so `Sleep` is used for terminal synchronization.

Pre-baked fixture shell scripts in `/tmp/` encapsulate all `git` fixture setup (hermetic repos matching the bats test fixture pattern). VHS tapes call these scripts so the viewer sees the skill runtime output, not setup noise.

---

## Coverage Map

| Demo Artifact | ACs Covered | Path | Result |
|---------------|-------------|------|--------|
| `AC-001-002-004-010-happy-path.gif/.webm/.tape` | AC-001, AC-002, AC-004, AC-010 | Success | PASS |
| `AC-006-missing-spec-warn-continue.gif/.webm/.tape` | AC-006 (EC-003) | Success + Error path | PASS |
| `AC-007-008-missing-manifest-hard-block.gif/.webm/.tape` | AC-007, AC-008 | Error path | PASS |
| `AC-009-epic-complete-handoff.gif/.webm/.tape` | AC-009 (EC-EPIC) | Success | PASS |
| `EC-004-006-empty-stories-arch-warnings.gif/.webm/.tape` | EC-004, EC-006 | Warning paths | PASS |
| `bats-suite-16-16.gif/.webm/.tape` | All 16 bats tests (supporting) | All green | 16/16 |
| AC-005 | MANUAL — see manual test doc | N/A | See below |

---

## Per-AC Coverage Detail

### AC-001 — git-sourced read (no working-tree fallback)
**Demo:** `AC-001-002-004-010-happy-path.gif`  
**Fixture:** `factory-artifacts:wave-state.yaml` has real stories; working-tree `.factory/wave-state.yaml` has `S-18.STALE` (stale content). Skill output shows real spec files — `S-18.STALE` absent from injected list.  
**Result:** PASS — git-sourced read verified; working-tree ignored.

### AC-002 — exactly listed specs injected + INJECTED_FILE_COUNT sentinel
**Demo:** `AC-001-002-004-010-happy-path.gif`  
**Fixture:** 2 stories + 1 arch file + state_pointer. `INJECTED_FILE_COUNT=4` visible in output (BC-6.24.001.md deduplicated — listed in 2 stories, counted once).  
**Result:** PASS — sentinel line `INJECTED_FILE_COUNT=4` present; all listed files in output.

### AC-003 — no stale prior-wave specs
**Covered by:** `AC-001-002-004-010-happy-path.gif` (stale `S-18.STALE` file absent) and `bats-suite-16-16.gif` (test 3: `test_rehydrate_wave_does_not_inject_prior_wave_specs` passes).  
**Result:** PASS — prior-wave files not in manifest are not injected.

### AC-004 — STATE.md pointer always injected
**Demo:** `AC-001-002-004-010-happy-path.gif`  
**Fixture:** `state_pointer: .factory/STATE.md` in wave-state.yaml; `.factory/STATE.md` is NOT listed in `spec_files`. Output shows `.factory/STATE.md` in injected set and counted in `INJECTED_FILE_COUNT`.  
**Result:** PASS — state_pointer always injected regardless of spec_files listing.

### AC-005 — operator confirmation required (MANUAL)
**Status: MANUAL — not automated.**  
AC-005 requires a live Claude Code session to verify the confirmation pause behavior. Automated bats and VHS tests cannot simulate the LLM-session confirmation loop.  
**Manual test documentation:** `plugins/vsdd-factory/tests/manual/rehydrate-wave-AC-005-manual.md`  
The confirmation prompt text `Confirm rehydration: <n> file(s) listed above will be injected into the session context.` is visible in all demo recordings (happy-path, AC-006, EC-004/006) confirming the prompt exists in the skill output. The manual test covers the session-pause behavior specifically.

### AC-006 — missing spec files: warn not block
**Demo:** `AC-006-missing-spec-warn-continue.gif`  
**Fixture:** `wave-state.yaml` lists `[A.md, missing-spec.md, C.md]`; only `A.md` and `C.md` exist. `missing-spec.md` absent from both filesystem and `factory-artifacts`.  
**Observed:** `WARNING: listed spec file not found on filesystem: missing-spec.md` on stderr; `A.md` and `C.md` injected; `Confirm rehydration:` shown; exit 0.  
**Result:** PASS — warn-not-block path demonstrated.

### AC-007 — wave-state.yaml not found: hard block with RehydrationError
**Demo:** `AC-007-008-missing-manifest-hard-block.gif`  
**Fixture:** `factory-artifacts` branch has no `wave-state.yaml` and no `HANDOFF.md`.  
**Observed:** `RehydrationError: wave-state.yaml not found on factory-artifacts; cannot rehydrate. Run /wave-handoff on wave N to produce the manifest.` on stderr; `Exit_code=1` shown.  
**Result:** PASS — hard block with canonical error message and non-zero exit.

### AC-008 — no RAG fallback on missing manifest
**Demo:** `AC-007-008-missing-manifest-hard-block.gif` (same scenario as AC-007)  
**Observed:** No `INJECTED_FILE_COUNT` sentinel in output (no files injected); no spec file list present before the error. Only the `RehydrationError` message.  
**Result:** PASS — no file injection occurred; no RAG fallback path.

### AC-009 — EPIC-COMPLETE path reads HANDOFF.md only
**Demo:** `AC-009-epic-complete-handoff.gif`  
**Fixture:** No `wave-state.yaml`; `HANDOFF.md` with `epic_status: complete`, `next_wave_stories: []`, `arch_files: [.factory/specs/architecture/ARCH-INDEX.md]`.  
**Observed:** `Epic complete — no next-wave stories`; `.factory/STATE.md` and `.factory/specs/architecture/ARCH-INDEX.md` injected; `INJECTED_FILE_COUNT=2`; no `RehydrationError`.  
**Result:** PASS — EPIC-COMPLETE path verified.

### AC-010 — INJECTED_FILE_COUNT set semantics (deduplication)
**Demo:** `AC-001-002-004-010-happy-path.gif`  
**Fixture:** Two stories both list `BC-6.24.001.md`; deduplicated set has 4 files (not 5). `INJECTED_FILE_COUNT=4` sentinel confirms set semantics.  
**Result:** PASS — deduplication verified via machine-stable sentinel.

### EC-004 — empty stories list: operator WARNING + continues
**Demo:** `EC-004-006-empty-stories-arch-warnings.gif` (first half)  
**Observed:** `WARNING: wave-state.yaml lists no stories (stories: [] or no spec_files); injecting arch_files + state_pointer only.`; `INJECTED_FILE_COUNT=2`; exit 0.  
**Result:** PASS — warning path demonstrated; arch_files + state_pointer injected.

### EC-006 — empty arch_files: operator WARNING + continues
**Demo:** `EC-004-006-empty-stories-arch-warnings.gif` (second half)  
**Observed:** `WARNING: wave-state.yaml lists no arch_files; no architectural context will be injected.`; story spec_files + state_pointer injected; `INJECTED_FILE_COUNT=2`; exit 0.  
**Result:** PASS — warning path demonstrated; story spec_files + state_pointer injected.

---

## Bats Suite — Supporting Evidence

**Demo:** `bats-suite-16-16.gif`  
16/16 tests pass, covering:
- AC-001..AC-010 (minus AC-005 manual)
- EC-004 (`test_rehydrate_wave_warns_on_empty_stories_list`)
- EC-006 (`test_rehydrate_wave_warns_on_empty_arch_files`)
- F-P1-003 (real producer manifest shape with `status:` between `id:` and `spec_files:`)
- F-P1-004 (inline `spec_files: []` story form)
- F-P2-001 (bare invocation defaults `REPO_DIR=.`, `ARTIFACTS_WT=.factory`)
- F-P2-003 (EPIC-COMPLETE branch `_check_missing_file` corroboration)
- F-P2-004 (EPIC-COMPLETE contradictory manifest: `epic_status=complete` + non-empty `next_wave_stories`)

---

## AC-005 Manual Coverage Note

AC-005 is explicitly not automated per the story spec:

> "Test (manual): Manual session test at F3 — invoke /rehydrate-wave in a live Claude Code session; verify that a confirmation prompt is presented and session pauses before any downstream pipeline step is dispatched."

The manual test procedure is documented at:  
`plugins/vsdd-factory/tests/manual/rehydrate-wave-AC-005-manual.md`

The `Confirm rehydration:` prompt text is visible in all VHS recordings where the skill exits 0 (AC-001/002/004/010, AC-006, EC-004, EC-006, AC-009), confirming the static output is present. The interactive pause behavior in a live Claude Code session is covered by the manual test document.

---

## Artifact Index

| File | Type | Size |
|------|------|------|
| `AC-001-002-004-010-happy-path.gif` | VHS GIF | 125 KB |
| `AC-001-002-004-010-happy-path.webm` | VHS WebM | 137 KB |
| `AC-001-002-004-010-happy-path.tape` | VHS script | 977 B |
| `AC-006-missing-spec-warn-continue.gif` | VHS GIF | 124 KB |
| `AC-006-missing-spec-warn-continue.webm` | VHS WebM | 130 KB |
| `AC-006-missing-spec-warn-continue.tape` | VHS script | 975 B |
| `AC-007-008-missing-manifest-hard-block.gif` | VHS GIF | 92 KB |
| `AC-007-008-missing-manifest-hard-block.webm` | VHS WebM | 98 KB |
| `AC-007-008-missing-manifest-hard-block.tape` | VHS script | 929 B |
| `AC-009-epic-complete-handoff.gif` | VHS GIF | 126 KB |
| `AC-009-epic-complete-handoff.webm` | VHS WebM | 132 KB |
| `AC-009-epic-complete-handoff.tape` | VHS script | 898 B |
| `EC-004-006-empty-stories-arch-warnings.gif` | VHS GIF | 173 KB |
| `EC-004-006-empty-stories-arch-warnings.webm` | VHS WebM | 259 KB |
| `EC-004-006-empty-stories-arch-warnings.tape` | VHS script | 853 B |
| `bats-suite-16-16.gif` | VHS GIF | 200 KB |
| `bats-suite-16-16.webm` | VHS WebM | 606 KB |
| `bats-suite-16-16.tape` | VHS script | 787 B |
