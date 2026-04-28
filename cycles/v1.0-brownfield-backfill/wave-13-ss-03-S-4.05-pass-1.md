# S-4.05 Adversarial Review — Pass 1

## Summary
- Findings count: 11
- Severity breakdown: HIGH=4, MEDIUM=5, LOW=2, NITPICK=0
- Verdict: SUBSTANTIVE

## Findings

### F-001 [HIGH]: Internal contradiction between DLQ filename pattern (AC-001) and daily rotation (AC-004)
**File:** /Users/jmagady/Dev/vsdd-factory/.factory/stories/S-4.05-dead-letter-queue.md lines 68 and 74
**Issue:** AC-001 specifies the DLQ path as `.factory/logs/dlq/<sink-name>.jsonl` — no date in the filename. AC-004 states "DLQ file has daily rotation matching file sink rotation". File-sink rotation (BC-3.02.001) is achieved by interpolating `{date}` into the path template. Furthermore, CAP-024 explicitly specifies the filename as `dead-letter-<sink>-<date>.jsonl`. Three sources disagree.
**Suggestion:** Pick one. Either match CAP-024 verbatim — `dead-letter-<sink-name>-{date}.jsonl` — OR keep the dlq directory but include `{date}` in the filename: `dlq/<sink-name>-{date}.jsonl`.

### F-002 [HIGH]: Architectural layering violation — sink-core cannot depend on sink-file
**File:** /Users/jmagady/Dev/vsdd-factory/.factory/stories/S-4.05-dead-letter-queue.md line 75 and crates/sink-core/Cargo.toml
**Issue:** AC-004 traceability claims DLQ inherits rotation by reusing file-sink path-template code. But sink-core has no dependency on sink-file, and sink-file already depends on sink-core. Adding sink-core → sink-file would create a circular dependency. Story does not include the required refactor task.
**Suggestion:** Add task: "Extract resolve_path_template + FileSinkError::UnknownPlaceholder from sink-file into a new sink-core::path_template module; update sink-file to import from sink-core; verify zero behavioral change."

### F-003 [HIGH]: Mis-anchored AC traces — BC-3.02.013 cited for behavior it does not contract
**File:** /Users/jmagady/Dev/vsdd-factory/.factory/stories/S-4.05-dead-letter-queue.md lines 69, 79, 81
**Issue:** AC-001/AC-006/AC-007 cite BC-3.02.013 for behaviors it doesn't contract. BC-3.02.013 is about `take_failures()` returning a SinkFailure, NOT writing to a file path or HTTP failure scenarios. Per POLICY 4 (semantic_anchoring_integrity), every anchor claim must be semantically correct.
**Suggestion:** Either re-anchor each AC to the correct BC (BC-3.02.005/006 for mkdir-p; v1.1 BC candidates explicitly for DLQ behaviors), OR keep BC-3.02.013 as related-only and route all ACs through v1.1 candidates.

### F-004 [HIGH]: VP-INDEX anchor-back missing — POLICY 9 violation pending
**File:** /Users/jmagady/Dev/vsdd-factory/.factory/specs/verification-properties/VP-INDEX.md lines 155-156
**Issue:** Story claims VP-011 + VP-012, but VP-INDEX Story Anchors does not list S-4.05.
**Suggestion:** Append VP-011 → S-4.05 and VP-012 → S-4.05 in same delivery wave.

### F-005 [MEDIUM]: VP-011 anchoring is semantically weak
**File:** S-4.05 frontmatter line 24, body line 122
**Issue:** VP-011 (Sink submit Must Not Block) is contracted by try_send on the dispatcher hot path. DLQ writes occur on the worker thread AFTER submit() returned. Mis-anchoring.
**Suggestion:** Drop VP-011. Keep VP-012. Register a new VP for "DLQ write must not block submit" if desired.

### F-006 [MEDIUM]: Missing edge cases — sink_name path traversal, file collision, concurrent writes, recursive DLQ
**File:** /Users/jmagady/Dev/vsdd-factory/.factory/stories/S-4.05-dead-letter-queue.md lines 132-137
**Issue:** Edge Cases table only covers EC-001 (dir missing) and EC-002 (disk full). Missing: path traversal in sink_name, sink-name collision, concurrent DLQ writes, recursive DLQ feedback (internal.sink_dlq_write fanned through failing sink), mid-write crash.
**Suggestion:** Add EC-003..EC-007 covering each, plus sink-name sanitization v1.1 BC candidate.

### F-007 [MEDIUM]: AC-006 integration test misplaced relative to target_module
**File:** S-4.05 lines 32, 78, 198
**Issue:** target_module is sink-core/dead_letter.rs but AC-006 specifies sink with mock 5xx server (HTTP). HTTP-based sinks live in sink-http. Integration test must be in sink-http (or workspace-level). Tasks omit the wiring task; FileStructure mentions sink-http modify but Tasks doesn't enumerate.
**Suggestion:** Add task: "Wire DLQ emission into sink-http retry-exhaustion error path; add mock-5xx integration test in sink-http/tests/."

### F-008 [MEDIUM]: AC-005 commits to event field schema with no contract backing
**File:** S-4.05 lines 76-77, 150
**Issue:** AC-005 names internal.sink_dlq_write fields (sink_name, event_type, reason, ts). Neither anchored BC contracts these fields. POLICY 12 (bc_tv_emitter_consistency) requires emitter behavior to align with BC canonical TVs. PRD §3.1's INTERNAL_EVENT_SCHEMA_VERSION = 1 likely requires additional fields (dispatcher_trace_id, schema_version).
**Suggestion:** Promote BC-3.NN.NNN-dlq-internal-event-emission to a real BC ID this same delivery wave with a Canonical TV table pinning the full field set.

### F-009 [MEDIUM]: Configuration story is missing — DLQ enable/disable, retention, size cap
**File:** Whole story
**Issue:** Story does not specify enable/disable mechanism, default state, location in observability-config.toml, retention policy (does prune_old apply?), size cap.
**Suggestion:** Either add explicit AC for on-by-default + per-sink dlq_enabled toggle + size cap + retention, OR explicitly defer with v1.1 BC candidate link.

### F-010 [LOW]: Tasks list incomplete vs. FileStructure table
**File:** S-4.05 lines 166-172, 196-198
**Issue:** Tasks 1-5 cover dead_letter.rs but FileStructure declares sink-http/lib.rs modification. Tasks omit explicit sink-http wiring.
**Suggestion:** Expand Tasks to enumerate every file in FileStructure.

### F-011 [LOW]: EC-002 silently violates "no silent failures" engine principle
**File:** S-4.05 line 137
**Issue:** EC-002 says DLQ disk full → log to stderr only. Stderr is lossy under hooks-driven invocation. Per AGENT-SOUL.md #4 (no silent failures), DLQ-write failure means double event loss; stderr-only is silent in practice.
**Suggestion:** Strengthen EC-002 to also emit internal.sink_dlq_failure (or piggyback on internal.sink_error) so loss is observable in fan-out.
