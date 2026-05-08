# F5 Adversary Pass-2 — S-15.01 fix-burst (against merged 6050d24)

## Sanity probes
- git rev-parse HEAD → 6050d24 ✓
- partition.rs 0 todo!() ✓
- aggregator.rs 0 todo!() ✓
- 8 Kani harnesses (6+2) ✓
- ADR-020 + S-15.02 + 5 bats files ✓
- Latency canary budget = 1500ms ✓

## Verdict: HIGH

## Counts
H: 3  M: 6  L: 4  NIT: 2

## Findings (NEW only — no relitigation)

### F-P2-001 [HIGH] AC-016 budget revision sweep missed ac017_demo_evidence.rs (still cites 500ms)
- Axis: A, F
- Source: ADR-020 + S-15.01 v1.8 (1500ms); latency_canary.rs:51,199 (1500ms)
- Impl defect: `crates/factory-dispatcher/tests/ac017_demo_evidence.rs:110,120` panic strings still say "p95 ≤ 500ms per AC-016". Sibling test file overlooked during budget sweep.
- Fix: Update both error messages to "≤ 1500ms per ADR-020 Class A".
- Tag: [content-defect]

### F-P2-002 [HIGH] VP-079 v1.7 Scenario 6 SITES patterns use outdated short fn names
- Axis: A, POLICY 4
- Source: VP-079.md:499-506 SITES `emit_schema_mismatch | emit_registry_invalid | emit_async_block_discarded | emit_plugin_timeout` (short names)
- Impl: actual fn names `emit_dispatcher_schema_mismatch`, `emit_dispatcher_registry_invalid`, `emit_plugin_async_block_discarded`, `emit_plugin_timeout_async`. Bats file at vp079-scenario6-mutation-counter-proof.bats:171/194/217/240 uses correct names. Spec body line citations (129/138/362/379) also stale post-refactor (actual: 133/142/394/405).
- Fix: Update VP-079 SITES patterns and line citations.
- Tag: [content-defect]

### F-P2-003 [HIGH] Demo evidence latency-canary.md PASS NOT recorded by canonical test command
- Axis: E (POLICY 11), F
- Source: docs/demo-evidence/S-15.01/latency-canary.md:55-60 admits "ad-hoc shell harness" measurement due to temporary aggregator.rs compile-state during measurement
- Impl: aggregator.rs:3 now correctly placed (compile blocker resolved at HEAD). But demo file was NOT re-recorded using `cargo test --release ... --ignored`. AC-017 guard only checks literal "p95" — doesn't validate canonical-harness execution.
- Fix: Re-run canonical command; record output; update demo evidence with canonical-harness data.
- Tag: [content-defect]

### F-P2-004 [MEDIUM] BC-1.14.001 PC4 vs PC6 internal contradiction
- Axis: A, G
- Source: BC-1.14.001.md:64 PC4 ("results MAY be received concurrently while sync_group is still running"); :74-76 PC6 ("Async group plugins are spawned only after sync_group execution completes")
- Impl: main.rs:292 sync first; lines 306-348 async spawned only after. PC4's "during sync_group" is structurally unreachable.
- Fix: Amend PC4 to remove "while sync_group is still running OR" clause.
- Tag: [content-defect]

### F-P2-005 [MEDIUM] hooks-registry-lint.bats H2 still allows "not yet implemented" panic as PASS (Red Gate fallback)
- Axis: E (POLICY 11)
- Source: tests/bats/hooks-registry-lint.bats:309 OR-disjunct includes `*"not yet implemented"*`
- Impl: validate_async_block_invariant() fully implemented post-merge; the legacy panic-message disjunct is now a regression-mask (todo!() reintroduction would silently pass H2).
- Fix: Remove `"not yet implemented"` from OR-chain; tighten matchers to "E-REG-002" / "registry_invalid".
- Tag: [content-defect]

### F-P2-006 [MEDIUM] VP-079 Scenario 6 conflates "build failure" with "mutation caught" — false positive
- Axis: E (POLICY 11), G
- Source: tests/bats/vp079-scenario6-mutation-counter-proof.bats:128-132 — when mutation breaks build, harness sets caught=0 (claims caught)
- Impl: build-failure-counts-as-caught is verdict laundering — provides no evidence Scenarios 1-5 would have detected emit removal.
- Fix: Build failures must be infrastructure-error (caught=2 or skip); only caught=0 if scenarios 1-5 actually FAIL on a built mutated binary.
- Tag: [content-defect]

### F-P2-007 [MEDIUM] DI-019 says env-var override "deferred" but VSDD_ASYNC_DRAIN_WINDOW_MS is implemented
- Axis: A, B (decision 4)
- Source: invariants.md:146 — "Future may permit env-var override; this is a deferred decision."
- Impl: main.rs:75-76 defines ENV_ASYNC_DRAIN_WINDOW_MS (debug-only); main.rs:308-314 reads + overrides; bats fixtures depend on it.
- Fix: Amend DI-019 to add §Debug-build env-var override clause documenting the shipped feature.
- Tag: [content-defect]

### F-P2-008 [MEDIUM] S-15.01 v1.8 + S-15.02 References cite BC-1.14.001 v1.7 (stale; should be v1.8)
- Axis: F (POLICY 8 propagation)
- Source: BC-1.14.001 v1.8 (current); S-15.01.md:114 + S-15.02.md:384 cite v1.7
- Fix: Story body BC table version labels updated.
- Tag: [content-defect]

### F-P2-009 [MEDIUM] Kani harness name drift: spec proof_vp077_exit_code_independent_of_async vs impl _of_async_group
- Axis: A
- Source: VP-077.md:218,230 + S-15.01.md:343 (no _group); aggregator.rs:220 has _group
- Fix: Either rename impl or amend spec; current state has dead spec citation.
- Tag: [content-defect]

### F-P2-010 [LOW] H5 doc-comment overclaims structural enforcement; harness body proves only determinism
- Axis: A, G
- Fix: Reword H5 doc-comment to clarify type-system enforces independence; Kani proves determinism.
- Tag: [content-defect]

### F-P2-011 [LOW] F-P1-012 protect-secrets duplicate persists; BC-7.06.001 Invariant 7 still violated
- Axis: A
- Source: hooks-registry.toml:686,706 — two protect-secrets entries (PreToolUse, Bash vs Read)
- Fix: User decision: amend Invariant 7 to (name, event, tool) tuple OR rename one entry.
- Tag: [content-defect]
- Pass-1 relation: DEFERRED-FROM-P1

### F-P2-012 [LOW] VP-079 v1.7 Property 6 cites stale main.rs line numbers (129/138/362/379 vs actual 133/142/394/405)
- Axis: A
- Fix: Update line citations in VP-079 spec body.
- Tag: [content-defect]

### F-P2-013 [LOW] Async plugin stderr not relayed to dispatcher process stderr; undocumented
- Axis: G
- Source: main.rs:417-423 stderr-relay scoped to summary.per_plugin_results (sync only); partial_outcomes never iterated for stderr
- Fix: Document the asymmetric behavior near the relay loop OR add async stderr relay if user-visible output is expected.
- Tag: [content-defect]

### F-P2-014 [LOW] [process-gap] AC-017 demo guard checks literal "p95" only — doesn't validate value or budget compliance
- Axis: E, F
- Source: ac017_demo_evidence.rs:117
- Fix: Strengthen guard to extract p95 numerically + assert ≤ 1500ms + verify methodology section.
- Tag: [process-gap]

### F-P2-015 [NIT] BC-3.08.001 v1.5 last_amended frontmatter format includes parenthetical
- Axis: F
- Source: BC-3.08.001.md:8 — "last_amended: 2026-05-08 (v1.5 — F5 pass-1 fix-burst F-P1-007)"
- Fix: Move parenthetical to changelog; restore clean YYYY-MM-DD format.
- Tag: [content-defect]

(NIT-2: ADR-020 line 260 typo "retects" → "detects")

## ADR-013 clock
- Pass-1 verdict: HIGH (counter 0_of_3)
- Pass-2 verdict: HIGH
- Counter: 0_of_3 (HIGH findings present; clock does not advance)

## Notes

**Drain refactor verdict: SOUND with one spec contradiction (F-P2-004).**
- tokio::spawn per-plugin + mpsc::unbounded_channel + tokio::pin! + tokio::select! biased — correct
- drop(tx) after spawn → rx.recv() returns None when all senders drop — correct
- EC-012 partial completions implemented
- Drain timer started AFTER spawns (line 353) — correct
- JoinHandle cleanup via process exit — acceptable per EC-011

**Kani harness body vs property match:**
- H1 totality: MATCH
- H2 disjointness: MATCH (uses contains() both directions)
- H3 async field: MATCH (exhaustive bool case split)
- H4 union completeness: MATCH
- H5 independence: WEAK MATCH (proves determinism, not independence — F-P2-010)
- H6 aggregation: MATCH (both directions of IFF)
- determinism + empty_input: legacy witnesses retained

**VP-079 Scenario 6 effectiveness: NEEDS-WORK** — F-P2-002 (name drift) + F-P2-006 (build-failure conflation)

**trace_id end-to-end: COMPLETE on the wire.** internal_log.rs:116 #[serde(rename = "trace_id")] confirmed; emit_event.rs:347-368 wire-output assertion confirms zero dispatcher_trace_id in serialized JSON. RESERVED_FIELDS contains both (defense-in-depth).

**ADR-020 sufficiency: SUFFICIENT for Class A; minor typo at line 260 ("retects" → "detects").**

**Process-gap codification candidates (NEW):**
1. AC-017-style demo guards must extract numerical values, not literal strings (F-P2-014)
2. Sibling-test-file sweep when ACs change budget constants (F-P2-001)
3. POLICY 4 line-number citations go stale on refactor — use symbol names or CI check (F-P2-002, F-P2-012)
4. Mutation-test soundness — build failure must NEVER count as caught (F-P2-006)

**Recommendation:** Fix-burst-2 should address F-P2-001/002/003 (HIGHs), 6 MEDIUMs, and the trivial NITs/LOWs. F-P2-011 (protect-secrets) needs user adjudication.
