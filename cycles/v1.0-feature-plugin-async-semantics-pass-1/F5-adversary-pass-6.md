# F5 Adversary Pass-6 — S-15.01 fix-burst chain (against 3a5eb6e)

## Sanity probes
HEAD verification: state of main.rs matches F5-T-A drain refactor. Pass-5 file confirmed.

## Review angles chosen
- Angle B (test coverage per BC postcondition/invariant) — primary
- Angle C (concurrency/race scrutiny) — cleared no findings
- Angle E (operator-facing docs) — by-design clear
- Angle F (CI workflow) — by-design clear (latency canary intentionally #[ignore])

## Verdict: MEDIUM
Trajectory: 17 → 15 → 6 → 5 → 0 → 2 (Angle B surfaced new gaps pass-5 didn't target)

## Counts
H: 0  M: 2  L: 0  NIT: 0

## Findings (NEW)

### F-P6-001 [MEDIUM] BC-1.14.001 v1.9 PC4 EC-012 (partial drain completion across multiple async plugins) has no falsifiable test
- Axis: B
- Source: BC-1.14.001.md:142-144 (EC-012); :68 (PC4); S-15.01.md:215-226 (AC-005)
- Impl: main.rs:359-375 implements the partial_outcomes loop with biased tokio::select!
- Defect: EC-012 specifies "2 of 3 plugins finished in 50ms; 3rd takes 200ms; drain fires" → "Completed plugins' terminal events MUST emit before dispatcher exit". AC-005 cites Scenarios 1/4/5 each with EXACTLY ONE async plugin (S1: test-async-blocker; S4: slow-async-plugin; S5: slow-async-plugin-over-drain). No scenario constructs the "2 of 3 completed + 1 in-flight" fault pattern that EC-012 distinguishes from the all-or-nothing anti-pattern. Rust integration tests exercise emit_* in isolation, not the full pipeline.
- Fix: Add bats Scenario 7 with two async plugins — one fast (exits 2 within ~10ms producing plugin.async_block_discarded) and one slow (sleeps beyond drain — must NOT produce plugin.timeout). Assert presence of fast event AND absence of slow event in same dispatcher run.
- Tag: test-coverage-gap

### F-P6-002 [MEDIUM] BC-7.06.001 v1.5 Invariant 7 string-equality-not-regex-equivalence has no falsifiable test
- Axis: B
- Source: BC-7.06.001.md:77 (Invariant 7) + :94-96 (Implementation Notes)
- Impl: registry.rs:410-423 (validate_name_event_tool_uniqueness with HashSet<(String, String, Option<String>)>)
- Defect: v1.5 amendment specifies "Two entries with tool='^Bash$' and tool='Bash' are DISTINCT entries despite matching the same tool surface". Existing tests cover (a) identical-tool duplicate (rejected); (b) different-tool accepted; (c) None==None rejected. They DO NOT cover the v1.5-specific case: regex-variants `'^Bash$'` vs `'Bash'`. A future maintainer "improving" validate to do regex-equivalence comparison would NOT break any existing test, despite violating Invariant 7 v1.5. Regression-risk gap.
- Fix: Add Rust unit test `test_validate_treats_regex_variants_as_distinct_per_v1_5_amendment` in `f_p2_011_name_event_tool_uniqueness` module. Two [[hooks]] entries differing ONLY in tool='^Bash$' vs 'Bash'. Assert Registry::parse_str(toml).is_ok().
- Tag: test-coverage-gap, regression-risk

## ADR-013 clock
- Pass-1: HIGH (0_of_3)
- Pass-2: HIGH (0_of_3)
- Pass-3: MEDIUM (0_of_3)
- Pass-4: MEDIUM (0_of_3)
- Pass-5: NITPICK_ONLY (1_of_3)
- Pass-6: MEDIUM
- Counter: 0_of_3 (RESET; clock returns to start of chain)

## Notes

**Why pass-5 missed these:** Pass-5 focused on F-P4 finding resolutions + bidirectional coherence checks. Pass-5 did NOT cross-reference each PC/Invariant clause back to its falsifiable test artifact. Fresh-context Angle B is the value-add of pass-6.

**Other angles cleared:**
- Angle C: biased tokio::select! + drop(tx) before loop is correct; mpsc::unbounded_channel send-after-rx-drop is correctly ignored via `let _ = tx.send(...)`; _async_handles drop-detach is intended (EC-011 dispatcher process-exit semantics).
- Angle D: malformed UTF-8 → RegistryError::Io → fail-open per BC-1.08.001 (only SchemaVersion + AsyncBlockConflict are fail-closed exceptions). Consistent.
- Angle E: VSDD_ASYNC_DRAIN_WINDOW_MS absent from operator-facing docs — by design (debug-only per DI-019 v1.4; release builds compile out).
- Angle F: latency canary #[ignore]'d — by design (manual demo-recorder execution is documented evidence path per AC-016).

**Process-gap candidates (NEW):** None. The fix-burst pattern is functioning. The findings are content-level test coverage gaps.

**Recommendation:** Fix-burst-5 = test-writer authors 2 test additions (one bats Scenario 7 for EC-012 + one Rust unit test for regex-variants). Both localized; low-risk. Pass-7 then re-establishes convergence chain (clock 0_of_3 → 1_of_3 if NITPICK_ONLY).
