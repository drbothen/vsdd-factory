# Demo Evidence — S-18.11: sprint-state.yaml per-story producer format + def-b ordering

**Story:** S-18.11 (v1.10)
**Branch:** feature/S-18.11
**Deliverables:** `plugins/vsdd-factory/skills/wave-scheduling/SKILL.md` Step 5 (producer algorithm), `plugins/vsdd-factory/skills/wave-handoff/lib/parse-sprint-state.sh` (consumer allowlist + EC-010 supersession-tolerance guard), `plugins/vsdd-factory/tests/sprint-state-format.bats` (14 tests)
**Product type:** CLI bash (bats test suite + bash skill scripts)
**Recording method:** VHS terminal recordings (6 tapes; `.gif` + `.webm` per tape; font: Menlo)
**Suite result:** 14/14 green (5 CI-portable fixture-based + 9 production-file-guarded)
**BC gates:** BC-5.41.004 v1.4 (PC1-6, INV-1/2/3, EC-007/010); BC-5.41.001 v1.28 (PC2/PC3); BC-5.41.002 v1.20 (PC3)
**ADR anchors:** ADR-026 §Decision 3a (two-partition ordering); ADR-026 §Wave-Identity Derivation

---

## Coverage Mapping

| Tape | GIF | WEBM | Tape Script | AC(s) / RG(s) | BC Clause(s) | What Is Demonstrated |
|------|-----|------|-------------|----------------|-------------|----------------------|
| AC-FULL-suite-14-green | [gif](AC-FULL-suite-14-green.gif) | [webm](AC-FULL-suite-14-green.webm) | [tape](AC-FULL-suite-14-green.tape) | ALL (AC-001..AC-006, RG-1..RG-9) | BC-5.41.004 PC1-6, INV-1/2/3, EC-007/010; BC-5.41.001 PC2/PC3; BC-5.41.002 PC3 | Full 14-test bats suite passes; `echo exit:$?` confirms exit 0 |
| AC-001-AC-003-RG7-stories-list-and-status-fidelity | [gif](AC-001-AC-003-RG7-stories-list-and-status-fidelity.gif) | [webm](AC-001-AC-003-RG7-stories-list-and-status-fidelity.webm) | [tape](AC-001-AC-003-RG7-stories-list-and-status-fidelity.tape) | AC-001, AC-003, RG-7 | BC-5.41.004 PC1, PC2, PC4, INV-1, INV-2 | `test_sprint_state_stories_list_present` (stories: list is YAML sequence, per-story entries exist, 8-value enum valid) + `test_real_production_file_completeness_and_status_fidelity` (149 stories = STORY-INDEX; statuses match) |
| AC-002-RG9-def-b-two-partition-depth-order | [gif](AC-002-RG9-def-b-two-partition-depth-order.gif) | [webm](AC-002-RG9-def-b-two-partition-depth-order.webm) | [tape](AC-002-RG9-def-b-two-partition-depth-order.tape) | AC-002, RG-9 | BC-5.41.004 PC3 v1.4 (def-b), INV-3; BC-5.41.001 PC2 P-SPRINT-STATE-WAVE-ORDER; ADR-026 §Decision 3a | `test_sprint_state_stories_wave_order` (fixture: S-1.01 wave-1 first, no phantom wave: field) + `test_partitions_sorted_by_full_graph_depth_def_b` (full-graph depth order; S-8.09 depth=4 before S-9.00 depth=5; S-4.07 depth=11 before S-4.08 depth=12; done-first contiguity) |
| AC-004-AC-006-consumer-roundtrip-wave-id | [gif](AC-004-AC-006-consumer-roundtrip-wave-id.gif) | [webm](AC-004-AC-006-consumer-roundtrip-wave-id.webm) | [tape](AC-004-AC-006-consumer-roundtrip-wave-id.tape) | AC-004, AC-006 | BC-5.41.002 PC3; BC-5.41.001 PC2/PC3; BC-5.41.004 PC3, INV-3; ADR-026 §Wave-Identity Derivation | `test_wave_handoff_parses_migrated_sprint_state` (S-1.02 draft in next_wave_stories; S-1.01 merged excluded; no BrokenSprintState; exit 0) + `test_wave_id_wave_group_ordinal` (10-merged-one-block + 2-draft → completed_waves=1 → wave_id=2; NOT 11) |
| EC-010-RG8-supersession-tolerance | [gif](EC-010-RG8-supersession-tolerance.gif) | [webm](EC-010-RG8-supersession-tolerance.webm) | [tape](EC-010-RG8-supersession-tolerance.tape) | EC-010, RG-8 | BC-5.41.004 v1.4 EC-010 (tolerate supersession-edge); BC-5.41.004 PC3 | `test_supersession_edge_tolerated_partition_placement` (migration was EMITTED; S-3.04 partial+superseded_by:ADR-015 in non-terminal partition; S-3.01/S-3.02/S-3.03/S-4.07/S-4.08 merged in terminal prefix; no TopoViolation abort) |
| RG-3-RG6-allowlist-partial-accepted-complete-rejected | [gif](RG-3-RG6-allowlist-partial-accepted-complete-rejected.gif) | [webm](RG-3-RG6-allowlist-partial-accepted-complete-rejected.webm) | [tape](RG-3-RG6-allowlist-partial-accepted-complete-rejected.tape) | RG-3, RG-6 | BC-5.41.004 INV-1, EC-007; BC-5.41.002 PC3 | SUCCESS PATH: `test_consumer_accepts_partial_status` (partial IS in 8-value enum; CLASSIFY_RESULT=has-next-wave; S-1.02 in BROKEN_STORY_IDS) / ERROR PATH: `test_consumer_rejects_complete_status` (complete NOT in enum; exit non-zero; "unknown story status 'complete'") |

---

## Acceptance Criteria Full Coverage

| AC | Title | Status | Tape(s) | BC Clause |
|----|-------|--------|---------|-----------|
| AC-001 | stories: list present with per-story {id, status} entries; YAML sequence | COVERED | AC-001-AC-003-RG7, AC-FULL | BC-5.41.004 PC1, PC2, INV-1 |
| AC-002 | two-partition def-b ordering: terminal prefix, non-terminal suffix; full-graph wave-depth; no phantom wave: field | COVERED | AC-002-RG9, AC-FULL | BC-5.41.004 PC3 v1.4, INV-3; ADR-026 §Decision 3a |
| AC-003 | per-story statuses match STORY-INDEX; no fabricated values; EC-007 UnknownStatusToken | COVERED | AC-001-AC-003-RG7, AC-FULL | BC-5.41.004 PC2, INV-2, EC-007 |
| AC-004 | consumer round-trip: draft appears in next_wave_stories; merged excluded; no BrokenSprintState | COVERED | AC-004-AC-006, AC-FULL | BC-5.41.002 PC3; BC-5.41.001 PC2/PC3 |
| AC-005 | WASM gate processes HANDOFF.md derived from migrated sprint-state.yaml (P-SPRINT-STATE-WAVE-ORDER met) | COVERED (via AC-004 exit 0) | AC-004-AC-006, AC-FULL | BC-5.41.001 PC2; BC-5.41.004 PC3 |
| AC-006 | wave_id = wave-group ordinal; 10-merged-1-block + 2-draft → wave_id=2 | COVERED | AC-004-AC-006, AC-FULL | BC-5.41.001 PC2; ADR-026 §Wave-Identity Derivation |
| AC-007 | wave-scheduling SKILL.md Step 5 cites BC-5.41.004 as producer authority | COVERED (via bats suite passing; Step 5 is load-bearing for AC-002 ordering) | AC-FULL | BC-5.41.004 PC1-PC3 |

---

## Regression Guard Coverage

| Label | @test function | Tape | What Is Demonstrated |
|-------|---------------|------|----------------------|
| RG-1 | test_epics_coexistence_nested_stories_ignored | AC-FULL | awk /^stories:/ column-0 anchor; nested epics[*].stories: sub-keys not scanned; count=3, no enum-invalid values |
| RG-2 | test_real_production_file_round_trip | AC-FULL | Real sprint-state.yaml → derive_wave_id exits 0 + positive integer wave_id |
| RG-3 | test_consumer_accepts_partial_status | RG-3-RG6, AC-FULL | partial in 8-value enum; CLASSIFY_RESULT=has-next-wave; S-1.02 in BROKEN_STORY_IDS |
| RG-4 | test_consumer_rejects_interleaved_ordering | AC-FULL | terminal after non-terminal → WaveOrderUnverifiable abort (P-SPRINT-STATE-WAVE-ORDER) |
| RG-5 | test_consumer_partial_only_raises_broken_sprint_state | AC-FULL | partial-only → CLASSIFY_RESULT=broken-sprint-state (has_broken=1, has_next_wave=0) |
| RG-6 | test_consumer_rejects_complete_status | RG-3-RG6, AC-FULL | complete NOT in enum → exit non-zero + "unknown story status 'complete'" |
| RG-7 | test_real_production_file_completeness_and_status_fidelity | AC-001-AC-003-RG7, AC-FULL | PC4 completeness (149 stories; no phantom/missing); PC2/INV-2 status-fidelity |
| RG-8 | test_supersession_edge_tolerated_partition_placement | EC-010-RG8, AC-FULL | S-3.04 superseded in non-terminal partition; S-3.01/2/3+S-4.07/8 merged in terminal prefix; EC-010 TOLERATE locked |
| RG-9 | test_partitions_sorted_by_full_graph_depth_def_b | AC-002-RG9, AC-FULL | Full-graph depth order; monotone depth + lex tie-break; done-first contiguity; spot anchors S-8.09/S-9.00/S-4.07/S-4.08 |

---

## Exit Code Evidence

Every tape includes `echo exit:$?` after the final bats invocation, confirming exit 0. The `AC-FULL` tape shows exit 0 after the complete 14-test suite. This satisfies the non-blocking exit semantics required by the story scope.

---

## Sandbox Discipline

All recordings ran in the worktree root `/Users/zious/Documents/GITHUB/vsdd-factory/.worktrees/feature-S-18.11`. The bats tests that operate on the production `.factory/stories/sprint-state.yaml` (RG-2, RG-7, RG-8, RG-9) read the file via the factory-artifacts worktree mounted at `.factory/` — read-only access; no writes to `.factory/` during recording. Fixture-based tests (AC-002, AC-003, AC-004, AC-006, RG-1, RG-3..RG-6) use files in `plugins/vsdd-factory/tests/fixtures/sprint-state-format/` — no production file dependency.

---

## Spec References

| Reference | Clause | Coverage |
|-----------|--------|----------|
| BC-5.41.004 | PC1 | AC-001 (stories: key present) |
| BC-5.41.004 | PC2 | AC-001, AC-003 (per-story id+status schema; mechanically from STORY-INDEX) |
| BC-5.41.004 | PC3 v1.4 (def-b) | AC-002 (two-partition; full-graph wave-depth; cross-partition supersession edges included) |
| BC-5.41.004 | PC4 | AC-003, RG-7 (completeness: 149 stories; no missing/phantom IDs) |
| BC-5.41.004 | PC5 | RG-1 (legacy epics: section coexists; nested stories: sub-keys ignored) |
| BC-5.41.004 | PC6 | AC-001, AC-FULL (YAML well-formed) |
| BC-5.41.004 | INV-1 | AC-001, RG-3, RG-6 (8-value canonical enum: draft/ready/in-progress/partial/blocked/merged/withdrawn/cancelled) |
| BC-5.41.004 | INV-2 | AC-003, RG-7 (no fabricated statuses) |
| BC-5.41.004 | INV-3 | AC-002, AC-006, RG-9 (no phantom wave: field; ordering from depends_on topo-sort) |
| BC-5.41.004 | EC-007 | RG-6 (UnknownStatusToken hard-abort on complete) |
| BC-5.41.004 | EC-010 | EC-010-RG8 (supersession-edge TOLERATE path; S-3.04 partial+superseded_by:ADR-015) |
| BC-5.41.001 | PC2 | AC-006 (wave-group ordinal: completed terminal wave groups + 1; P-SPRINT-STATE-WAVE-ORDER) |
| BC-5.41.001 | PC3 | AC-004 (anti-fabrication cross-checks pass) |
| BC-5.41.002 | PC3 | AC-004 (stories from status:draft; reserved-pending no-op; BrokenSprintState handling) |
| ADR-026 §Decision 3a | two-partition ordering | AC-002, RG-9 |
| ADR-026 §Wave-Identity Derivation | wave-group ordinal | AC-006 |
