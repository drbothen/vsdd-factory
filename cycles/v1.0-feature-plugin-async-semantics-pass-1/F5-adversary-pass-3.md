# F5 Adversary Pass-3 — S-15.01 fix-burst-2 (against 2cfe3c1)

## Sanity probes
All passed.

## Pass-2 finding resolutions verified
All 14 pass-2 findings RESOLVED at HEAD 2cfe3c1.

## Verdict: MEDIUM
Trajectory: 5H/6M/4L/2NIT → 3H/6M/4L/2NIT → 0H/2M/2L/2NIT (IMPROVING)

## Counts
H: 0  M: 2  L: 2  NIT: 2

## Findings

### F-P3-001 [MEDIUM] VP-079 v1.8 amendment INCOMPLETE — sibling-doc gap in same Proof Harness Skeleton
- Axis: A, POLICY 4, S-7.01 partial-fix
- Source: VP-079.md:458-462 (mutation-site comment) + :469-471 (cargo-mutants --filter)
- Defect: F-P2-002 amendment fixed Property 6 (lines 83-85) and Scenario 6 SITES array (lines 501-506) but missed two sibling references in the SAME Proof Harness Skeleton bash code-fence:
  - Lines 459-462: SITE_1..SITE_4 still cite short names (emit_schema_mismatch, emit_registry_invalid, emit_async_block_discarded, emit_plugin_timeout) + old line numbers (129/138/362/379)
  - Lines 469-471: cargo-mutants --filter='emit_schema_mismatch|emit_registry_invalid|emit_async_block_discarded|emit_plugin_timeout' uses same short names
- Fix: Update VP-079 v1.8 → v1.9 — sweep all 4 sibling sites to full fn names + new line numbers (133/142/394/405)
- Tag: [content-defect]

### F-P3-002 [MEDIUM] partition.rs Kani harness doc-comments cite stale (name, event) tuple
- Axis: A, S-7.01 sibling-file propagation
- Source: VP-077 v1.9 + BC-7.06.001 v1.4 (both updated to (name, event, tool))
- Impl defect: partition.rs:149-150 H1 doc-comment still says "(name, event) tuple uniqueness ... per BC-7.06.001 Invariant 7". H2 (line 175) and H4 doc-comments likely have same drift.
- Note: Actual kani::assume blocks don't exist (entries unique by construction via name "plugin-{i}"); doc-comment is the only place the constraint surfaces.
- Fix: Update partition.rs H1/H2/H4 doc-comments to "(name, event, tool)" per BC-7.06.001 v1.4 Invariant 7
- Tag: [content-defect]

### F-P3-003 [LOW] BC-7.06.001 v1.4 Invariant 7 doesn't explicitly state "string equality, not regex equivalence" for tool field
- Axis: A
- Source: BC-7.06.001.md:77 — says "raw regex string value" but doesn't clarify two regex-equivalent but string-distinct values are NOT duplicates
- Defect: Future operator might assume the dispatcher does regex-equivalence detection. Test coverage: test_validate_treats_two_none_tools_as_duplicate covers None==None; no test covers Some("Bash") vs Some("^Bash$").
- Fix: Add sentence to Invariant 7: "string equality, not regex equivalence — tool='^Bash$' and tool='Bash' are distinct entries even though they match the same tool". Optionally add positive-control test.
- Tag: [content-defect]

### F-P3-004 [LOW] DI-019 v1.2 §Debug-build env-var clause doesn't specify malformed-value handling
- Axis: G
- Source: invariants.md:148; main.rs:308-312 silently falls back on parse failure
- Defect: VSDD_ASYNC_DRAIN_WINDOW_MS=abc (parse fail), =0 (parses but absurd), =99999...... (overflows) — all silently fall back to canonical 100ms with no warning. SOUL.md #4 says no silent failures.
- Fix: Either (a) DI-019 explicitly says "silent fallback to canonical value on any parse error" (documenting the contract), or (b) main.rs emits internal.dispatcher_warning event on malformed override.
- Tag: [content-defect]

### F-P3-005 [NIT] hooks-registry-lint.bats H2 OR-disjunct includes overly-broad matchers
- Axis: E (POLICY 11)
- Source: hooks-registry-lint.bats:308 — has *"async"* and *"on_error"* matchers that could match unrelated panics
- Fix: Remove *"async"* and *"on_error"*; require either *"E-REG-002"* OR *"registry_invalid"* (canonical signals)
- Tag: [content-defect]

### F-P3-006 [NIT] ADR-020 rationale stale claim about p99 budget margin
- Axis: F
- Source: ADR-020 line 117 cites "Clears p99 by 35%"; actual v1.9 p99=1570ms vs 1500ms budget = 4.6% OVER
- Defect: Stale rationale (probably based on v1.7 debug data). Not a verdict failure (AC-016 is p95-only) but rationale is misleading.
- Fix: Update ADR-020 rationale paragraph to acknowledge actual p99=1570ms (release v1.9) is over Class A budget by 4.6%; OR add a non-blocking Class A p99 ≤ 2000ms soft-guard.
- Tag: [content-defect]

## ADR-013 clock
- Pass-1: HIGH (0_of_3)
- Pass-2: HIGH (0_of_3)
- Pass-3: MEDIUM (0_of_3)
- Counter resets on M; advances only on NITPICK_ONLY.

## Notes

**New-surface verdicts:**
1. registry.rs DuplicateEntry: SOUND (Hash/Eq correct; tests genuine; POLICY 11 satisfied)
2. VP-077 H1/H2/H4 spec text correct; partition.rs impl drift on doc-comments → F-P3-002
3. AC-017 guard regex SOUND
4. latency-canary.md re-record SOUND (p95=1161ms 10% variance from prior; p99 over-budget per F-P3-006)
5. bats H2 matchers WEAK → F-P3-005
6. bats vp079-scenario6 mutation soundness SOUND (post F-P2-006 fix)

**Process-gap codification candidates (NEW):**
1. Spec-internal consistency sweeps — when amendment renames symbols/lines, scan ENTIRE doc for sibling occurrences (F-P3-001 demonstrates the recurrence)
2. Sibling-file Rust doc-comment propagation — BC/VP version bumps must propagate to Rust doc-comments quoting the BC (F-P3-002)

**Convergence trajectory: IMPROVING SHARPLY.** All HIGHs resolved. The 2 MEDIUMs are spec/doc consistency (S-7.01 axis), not behavioral defects. One more fix-burst should clear all M/L/NIT; pass-4 then ideally NITPICK_ONLY (1_of_3).
