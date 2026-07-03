# Demo Evidence Report — S-18.01

Story: HANDOFF.md Schema + wave-handoff Skill; wave-state.yaml Atomic Production
BCs: BC-5.41.001 v1.21, BC-5.41.002 v1.15
Recorded: 2026-06-18

All recordings use VHS with hermetic git fixtures matching the bats test setup pattern
(ADR-027 two-arg invocation model; no nested .factory/ inside ARTIFACTS_WT).

---

## Coverage Matrix

| Recording | File | ACs Covered | Scenario |
|-----------|------|-------------|---------|
| Happy-path has-next-wave | AC-001-013-017-happy-path | AC-001, AC-002, AC-003, AC-013, AC-015, AC-017 | Skill runs with pending+draft stories; shows committed HANDOFF.md (9 base fields) + wave-state.yaml (6 fields) in single atomic commit |
| EPIC-COMPLETE | AC-012-epic-complete | AC-012, AC-005 | All sprint-state entries terminal; shows verbatim 3-line stdout announcement, epic_status:complete in HANDOFF.md, wave-state.yaml absent from committed tree |
| Anti-fabrication hard-block | AC-anti-fabrication | AC-015 (preflight guard) | Phantom story S-99.99 not in STORY-INDEX.md; shows exit 1, AntiFabricationFailed error, no partial HANDOFF.md written |
| Multi-table topo-sort | AC-015-topo-sort | AC-015, AC-002, AC-014 | sprint-state in non-dep file order (S-18.04a,S-18.02,S-18.03); shows committed wave-state.yaml stories in topo order (S-18.02→S-18.03→S-18.04a); multi-table STORY-INDEX with E-0 (spaced Depends On) + E-18 (hyphenated Depends-On) |

---

## AC Coverage Status

| AC | Description | Coverage |
|----|-------------|---------|
| AC-001 | HANDOFF.md has all 9 base fields | AC-001-013-017-happy-path (happy-path) |
| AC-002 | wave_id derived from sprint-state/STATE.md, no phantom current_wave: | AC-001-013-017-happy-path, AC-015-topo-sort |
| AC-003 | last_verified_develop_sha is 40-char hex from git rev-parse | AC-001-013-017-happy-path |
| AC-004 | active_bcs non-empty or hard error | AC-001-013-017-happy-path (non-empty path covered; empty-BC hard-error is covered by bats test_active_bcs_empty_dir_causes_hard_error — VHS demo infeasible: fixture requires removing BC dir mid-setup) |
| AC-005 | next_wave_stories: [] on EPIC-COMPLETE | AC-012-epic-complete |
| AC-006 | open_decisions: [] valid | AC-001-013-017-happy-path (field present, empty list) |
| AC-007 | pending_fixes: [] valid | AC-001-013-017-happy-path (field present, empty list) |
| AC-008 | process_gaps: [] valid | AC-001-013-017-happy-path (field present, empty list) |
| AC-009 | precompact_flush_sha three-state rule | AC-001-013-017-happy-path (null case — log absent); three-state rule fully exercised by bats test_precompact_flush_sha_three_state_rule |
| AC-010 | factory_lock_holder null or string | AC-001-013-017-happy-path (null case) |
| AC-011 | Commit message format: HANDOFF wave-N ISO-timestamp | AC-001-013-017-happy-path (commit shown in output) |
| AC-012 | EPIC-COMPLETE: verbatim 3-line announcement + epic_status:complete + no wave-state.yaml | AC-012-epic-complete |
| AC-013 | wave-state.yaml has 6 required fields | AC-001-013-017-happy-path |
| AC-014 | generated_from_handoff_sha is prior HEAD before commit (null for wave 1) | AC-015-topo-sort (null case shown for first wave); prior-SHA population exercised by bats test_BC_5_41_002_F_P4_002 |
| AC-015 | stories derived from sprint-state.yaml only, topo-sorted | AC-015-topo-sort (multi-table + diamond DAG), AC-anti-fabrication (preflight guard) |
| AC-016 | arch_files from story anchored_adrs/subsystem | AC-001-013-017-happy-path (arch_files in committed blob) |
| AC-017 | Atomic single commit for both files | AC-001-013-017-happy-path (Files: HANDOFF.md wave-state.yaml in single commit) |
| AC-018 | BrokenSprintState exit 1 with canonical message | Covered by bats test_broken_sprint_state_canonical_message — VHS not separately recorded (identical pattern to AC-019/AC-020; bats suite provides full coverage) |
| AC-019 | HandoffMissing emitted by shell-side wave-gate | Shell-side responsibility; not exercised directly by wave-handoff.sh (documented in AC-019 narrative) |
| AC-020 | review-pending triggers BrokenSprintState | Covered by bats test_review_pending_triggers_broken_sprint_state |

---

## Notes on Non-Recorded ACs

**AC-004 empty-BC hard-error path:** The VHS fixture would require the BC dir to be empty at invocation time. This is structurally covered by bats `test_active_bcs_empty_dir_causes_hard_error` (pass 5, ok). The happy-path demo confirms active_bcs non-empty path.

**AC-018 / AC-020 (BrokenSprintState):** These are exit-1 error paths with distinct sprint-state fixtures. They are fully exercised by the bats suite (tests 19-20, both ok). Pattern is identical to AC-anti-fabrication error flow. Separate VHS recording would be redundant.

**AC-019 (HandoffMissing):** This is a shell-side wave-gate responsibility (not wave-handoff.sh itself). wave-handoff.sh ensures HANDOFF.md is always written on success, making HandoffMissing structurally impossible after skill completion.

---

## Behavioral Discrepancies Found

None. All 48 bats tests pass against the implementation on feature/S-18.01. No behavioral discrepancy between the implementation and the ACs was observed during demo recording.

---

## Files

| File | Type | Content |
|------|------|---------|
| AC-001-013-017-happy-path.gif | GIF | Happy-path has-next-wave recording |
| AC-001-013-017-happy-path.webm | WebM | Happy-path has-next-wave recording (archival) |
| AC-001-013-017-happy-path.tape | VHS tape | Source script for happy-path recording |
| AC-012-epic-complete.gif | GIF | EPIC-COMPLETE path recording |
| AC-012-epic-complete.webm | WebM | EPIC-COMPLETE path recording (archival) |
| AC-012-epic-complete.tape | VHS tape | Source script for EPIC-COMPLETE recording |
| AC-anti-fabrication.gif | GIF | Anti-fabrication hard-block recording |
| AC-anti-fabrication.webm | WebM | Anti-fabrication hard-block recording (archival) |
| AC-anti-fabrication.tape | VHS tape | Source script for anti-fabrication recording |
| AC-015-topo-sort.gif | GIF | Multi-table topo-sort recording |
| AC-015-topo-sort.webm | WebM | Multi-table topo-sort recording (archival) |
| AC-015-topo-sort.tape | VHS tape | Source script for topo-sort recording |
| demo-AC001-AC013-AC017-happy-path.sh | Shell | Demo invocation script (happy-path) |
| demo-AC012-epic-complete.sh | Shell | Demo invocation script (EPIC-COMPLETE) |
| demo-anti-fabrication.sh | Shell | Demo invocation script (anti-fabrication) |
| demo-AC015-topo-sort.sh | Shell | Demo invocation script (topo-sort) |
