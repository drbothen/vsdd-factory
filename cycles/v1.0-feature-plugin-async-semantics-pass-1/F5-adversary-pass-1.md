# F5 Adversary Pass-1 — S-15.01 (against merged 453eee19)

## Sanity probes (all confirmed)

- `partition.rs` is implemented (no `todo!()`); pure `partition_plugins` function exists at `crates/factory-dispatcher/src/partition.rs:90` — PASS
- 4 `#[kani::proof]` harnesses present in `crates/factory-dispatcher/src/partition.rs:148,173,196,216` — PASS
- `pub const ASYNC_DRAIN_WINDOW_MS` defined at `crates/factory-dispatcher/src/lib.rs:63` — PASS
- Schema version bumped to 2: `crates/factory-dispatcher/src/registry.rs:19` — PASS
- `validate_async_block_invariant()` implemented at `crates/factory-dispatcher/src/registry.rs:372-382` — PASS
- All 4 emit fns wired in `main.rs` (lines 129, 138, 362, 379) — PASS
- 4 bats files present — PASS
- Envelope flip complete: zero `"async": true` matches across `plugins/vsdd-factory/hooks/` — PASS
- 5 demo evidence files present at `docs/demo-evidence/S-15.01/` — PASS
- `hooks-registry.toml:11` has `schema_version = 2`, lint plugin registered at lines 957-967 — PASS

## Verdict: HIGH

## Counts
H: 5  M: 6  L: 4  NIT: 2

## Findings (sorted by severity)

### F-P1-001 [HIGH] VP-077: Kani harnesses cover only 2-3 of 6 declared properties (exit-code independence + aggregation NOT proven)
- Axis: A, D — Spec ↔ implementation alignment + VP-079 sufficiency
- Source-of-truth: VP-077.md:35-66 declares 6 properties; S-15.01.md:336-351 names 4 specific harnesses
- Implementation: partition.rs:148-224 has 4 harnesses but body of "totality_and_disjointness" only checks totality, not disjointness; ExitCodeIndep + AggregationCorrectness have no harness; aggregate_exit_code fn does not exist (zero grep hits)
- Required fix: Either (a) amend VP-077 to bounded scope of 2 properties, OR (b) author missing 4 harnesses + aggregate_exit_code fn
- Tag: [content-defect]

### F-P1-002 [HIGH] VP-079 structurally insufficient — production-path emission never proven (USER-DIRECTED Q1/Q2/Q3)
- Axis: D — VP-079 sufficiency
- Source-of-truth: VP-079.md:36-77
- Implementation: event_emission_fault_injection.rs:71-350 — all 8 Rust unit tests directly call emit_*(&ctx, ...) against isolated make_test_ctx() (line 52-59); no production dispatcher path exercised
- Q1: Property Statement says dispatcher MUST emit JSONL to events-*.jsonl (production path implied), but no property worded as "production code path actually invokes emit fn"
- Q2: NO for Rust unit tests; bats tests skip if factory-dispatcher binary unavailable; no fault-injection counter-proof
- Q3: YES — amend VP-079 v1.6 → v1.7 to add Property 6: production-path emission counter-proof. Removing any production caller (registry-load schema mismatch branch, registry-load async-block-conflict branch, async dispatch loop block detection, async dispatch loop timeout match arm) MUST cause at least one VP-079 scenario to fail
- Tag: [process-gap]

### F-P1-003 [HIGH] AC-016 latency canary measures a NO-OP (POLICY 11 violation)
- Axis: E — POLICY 11 (no_test_tautologies)
- Source-of-truth: S-15.01.md:354-362 — AC-016 requires sync_group p95 measurement on representative ~30+ plugin workload
- Implementation: latency_canary.rs:114-128 — measurement loop body is `let _ = std::hint::black_box(&registry); latencies.push(start.elapsed());`. Comment line 121 explicitly notes "RED: this placeholder makes the test compile but measures zero latency"
- Defect: 42ns p95 measures Instant::now() overhead, not dispatch latency. AC-016 observationally satisfied but actually unverified
- Required fix: Replace black_box placeholder with actual sync_group dispatch invocation; re-record latency-canary.md
- Tag: [content-defect]

### F-P1-004 [HIGH] BC-1.14.001 + VP-077 cite routing.rs for partition_plugins; implementation lives in partition.rs
- Axis: A, F — semantic anchoring + sibling consistency
- Source-of-truth: BC-1.14.001.md:14 (frontmatter inputs), :116 (Architecture Anchors), :171 (Traceability); VP-077.md:14 (frontmatter module), :91, :219, :236
- Implementation: partition.rs:90; zero references to partition_plugins in routing.rs
- Defect: 5 spec reference sites assert routing.rs is the home; impl chose partition.rs. POLICY 4 mis-anchor; POLICY 6 architecture-source-of-truth violated
- Required fix: Amend BC-1.14.001 v1.6 → v1.7 and VP-077 v1.6 → v1.7. Update all 5 reference sites to partition.rs
- Tag: [content-defect]

### F-P1-005 [HIGH] STORY-INDEX shows S-15.01 as `ready` after merge; story frontmatter status not updated post-merge
- Axis: F — sibling/index consistency
- Source-of-truth: STORY-INDEX.md:554 (status=ready); S-15.01.md:8 (frontmatter status: ready)
- Implementation: 453eee19 is squash-merge of PR #106 to develop; story is shipped
- Defect: Both STORY-INDEX and frontmatter still show `ready` after merge. STORY-INDEX changelog says "awaiting F4 TDD dispatch" (lines 49-51, 554) — contradicts merge state
- Required fix: Bump status to merged/shipped (verify engine convention). Update STORY-INDEX row + lifecycle commentary. State-manager closing pass.
- Tag: [content-defect]

### F-P1-006 [MEDIUM] T-3c deviation: drain implementation awaits within tokio::time::timeout instead of tokio::spawn fire-and-forget — semantic mismatch with BC-1.14.001 PC4
- Axis: A, C — spec ↔ impl + deviation ratification
- Source-of-truth: BC-1.14.001.md:62-68 PC4: "fire-and-forget task (tokio task or equivalent). Dispatcher does NOT await beyond drain window"
- Implementation: main.rs:329-334 — tokio::time::timeout(effective_drain_window, execute_tiers(async_inputs, async_tiers)).await.ok(). NOT fire-and-forget; bounded await consumes FULL drain window even when async plugins finish in 1ms
- Defect: PC4 + Invariant 3 ("Async group plugins are excluded from tier ordering model") violated — implementation runs `group_by_priority` then `execute_tiers` for async, preserving tier ordering inside async_group
- Required fix: (a) refactor to tokio::spawn-based fire-and-forget + tokio::select! drain timer, OR (b) amend BC-1.14.001 Invariant 3 to permit async tier ordering. Current state is undocumented contradiction.
- Tag: [content-defect]

### F-P1-007 [MEDIUM] BC-3.08.001 wire format duplicates trace fields — trace_id AND dispatcher_trace_id both present in serialized event
- Axis: A, G
- Source-of-truth: BC-3.08.001.md:51-60,70-79,90-98,108-116 — wire format examples show "trace_id" only
- Implementation: emit_event.rs:150-157 — ev.with_trace_id(...) sets dispatcher_trace_id AND with_field("trace_id", ...) adds top-level. Both serialized
- Defect: Sink consumers may parse one or the other. RESERVED_FIELDS includes dispatcher_trace_id but NOT trace_id; plugins could conflict. Per-event ABI inconsistency
- Required fix: Decide canonical name. (a) BC-3.08.001 v1.4 → v1.5 amend wire format to include both as canonical+alias, OR (b) impl removes dispatcher_trace_id and only emits trace_id. Add trace_id to RESERVED_FIELDS regardless
- Tag: [content-defect]

### F-P1-008 [MEDIUM] vp078_harness3_telemetry_classification.rs uses find() which returns first match — fails to verify ALL entries with required name
- Axis: A, G — test correctness
- Source-of-truth: BC-7.06.001.md:75-77 Invariant 6 lists 9 plugins INCLUDING worktree-hooks; Invariant 7 says (name, event) tuple unique
- Implementation: hooks-registry.toml:39-53 — TWO worktree-hooks entries (WorktreeCreate + WorktreeRemove); test at vp078_harness3_telemetry_classification.rs:86-87 uses .find() which returns first match
- Defect: If second worktree-hooks entry is misclassified async=false while first is correct async=true, test passes but invariant violated
- Required fix: Modify Harness 3 to assert async_flag==true for ALL entries with required names. Update REQUIRED_ASYNC_PLUGINS to (name, event) tuples
- Tag: [content-defect]

### F-P1-009 [MEDIUM] Demo evidence file latency-canary.md reports false PASS based on no-op canary
- Axis: F, E
- Source-of-truth: S-15.01.md:354-362 AC-016
- Implementation: docs/demo-evidence/S-15.01/latency-canary.md:31-56 — p95=42ns reported as PASS
- Defect: Reviewer would conclude AC-016 satisfied; it's not (see F-P1-003). status: PASS at line 7 is false claim
- Required fix: After F-P1-003 fix, re-record real measurements; OR mark status: PROVISIONAL with banner
- Tag: [content-defect]

### F-P1-010 [MEDIUM] PC4 + EC-010 violation — when drain truncates, ALL completed-within-window events are also discarded
- Axis: G
- Source-of-truth: BC-1.14.001.md:64-68 PC4 + EC-010: completed-within-drain events DO emit
- Implementation: main.rs:329-345 — tokio::time::timeout returns Err on truncation; `if let Some(async_summary)` skips ALL diagnostic emission
- Defect: 3 async plugins, 2 finish in 50ms, 1 takes 200ms, drain (100ms) fires while 3rd running, ALL 3 discarded. PC4 + EC-010 promise completed-within-drain events emit. Spec violation
- Required fix: Restructure async dispatch with tokio::spawn + channel for partial results; emit events for any results received before drain timer fires. Don't use tokio::time::timeout(execute_tiers) because execute_tiers is all-or-nothing
- Tag: [content-defect]

### F-P1-011 [MEDIUM] hooks-registry-lint.bats Harness 1a is a tautology — passes vacuously when no violations exist
- Axis: E — POLICY 11
- Source-of-truth: tests/bats/hooks-registry-lint.bats:30-73 — explicit comment "may pass vacuously"
- Implementation: Python regex scan; live registry has no violating combinations; assertion never fires
- Defect: Functionally `assert true`. Cannot detect regex regression
- Required fix: Add second test with temp file containing on_error="block" + async=true; assert violation detected. Add positive-control + negative-control
- Tag: [content-defect]

### F-P1-012 [LOW] hooks-registry.toml has TWO entries with name=protect-secrets (lines 686, 706) — both PreToolUse, different tool regexes; (name, event) tuple violates Invariant 7
- Axis: A
- Source-of-truth: BC-7.06.001.md:77 Invariant 7 — (name, event) unique
- Implementation: hooks-registry.toml:686 (event=PreToolUse, tool=Bash), :706 (event=PreToolUse, tool=Read)
- Defect: Either Invariant 7 needs (name, event, tool), OR rename plugins. validate() does NOT enforce uniqueness check
- Required fix: Amend BC-7.06.001 Invariant 7 to (name, event, tool); update VP-077 Harness 1 kani::assume; add load-time uniqueness check in validate()
- Tag: [content-defect]

### F-P1-013 [LOW] DI-019 cited in 2 source files only; bats fixtures cite it correctly — no fix needed; demoted from observation
- Tag: [content-defect]

### F-P1-014 [LOW] BC-3.08.001 schema-mismatch wire example uses int 2; bats compares string "2" — works but doesn't distinguish int/string regression
- Required fix: Add Python type assertion in bats
- Tag: [content-defect]

### F-P1-015 [LOW] HOST_ABI_VERSION=1 not emitted in BC-3.08.001 events — minor; defer to future ABI bump
- Tag: [content-defect]

### F-P1-016 [NIT] VP-079 frontmatter vp_id redundant with filename; pre-existing pattern
- Tag: [content-defect]

### F-P1-017 [NIT] BC-3.08.001 plugin.timeout overlaps BC-1.14.001 path; clarify authoritative ownership
- Tag: [content-defect]

## ADR-013 clock
- Pass-1 verdict: HIGH
- Counter: 0_of_3 (HIGH findings present; clock does not advance)

## Notes

**VP-079 amendment recommendation: YES** — VP-079 v1.6 → v1.7 must add Property 6: production-path emission counter-proof. Concrete: removing any production caller (main.rs:129/138/362/379) MUST cause at least one VP-079 scenario to fail. Verified by mutation testing or commit-revert-and-rerun.

**Process-gap codification candidates:**
1. Negative-mutation requirement for emit-style VPs — codify in rules/vp-design.md
2. POLICY 11 enforcement script — validate-no-test-tautologies hook scanning for std::hint::black_box, "may pass vacuously" comments, tests not calling production fns
3. Spec-impl module-path drift detector — for each BC frontmatter inputs: and Architecture Anchor file path, assert path exists and contains expected symbol
4. Post-merge story status flip — auto-flip ready → merged when squash-merge SHA observed on develop

## Files of interest cited

- crates/factory-dispatcher/src/partition.rs:148-224 (Kani harnesses — F-P1-001)
- crates/factory-dispatcher/tests/event_emission_fault_injection.rs:71-350 (isolated emit tests — F-P1-002)
- crates/factory-dispatcher/tests/latency_canary.rs:114-128 (no-op canary — F-P1-003)
- .factory/specs/behavioral-contracts/ss-01/BC-1.14.001.md:14,116,171 (routing.rs anchor — F-P1-004)
- .factory/specs/verification-properties/VP-077.md:14,91,219,236 (routing.rs anchor — F-P1-004)
- .factory/stories/S-15.01-plugin-async-semantics.md:8 + STORY-INDEX.md:554 (status — F-P1-005)
- crates/factory-dispatcher/src/main.rs:299-385 (drain implementation — F-P1-006, F-P1-010)
- crates/factory-dispatcher/src/host/emit_event.rs:143-256 (dual trace fields — F-P1-007)
- crates/factory-dispatcher/tests/vp078_harness3_telemetry_classification.rs:86-87 (find-first bug — F-P1-008)
- docs/demo-evidence/S-15.01/latency-canary.md:31-56 (false PASS — F-P1-009)
- plugins/vsdd-factory/hooks-registry.toml:686,706 (protect-secrets duplicate — F-P1-012)
