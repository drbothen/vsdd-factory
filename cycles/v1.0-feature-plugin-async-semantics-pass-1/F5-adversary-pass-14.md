# F5 Adversary Pass-14 — S-15.01 fix-burst chain (against 028a596)

## Sanity probes — All confirmed

## Pass-13 finding resolutions verified — F-P13-001/002/003/004/005/006 + O-P13-001 ALL RESOLVED

## Verdict: HIGH (substantive new defects via fresh angles)
Trajectory: 17→15→6→5→0→2→5→1→4→2→2→4→4→5

## Counts
H: 2  M: 2  L: 1  NIT: 0

## Findings (NEW)

### F-P14-001 [HIGH] Cross-BC wire-format contradiction: BC-7.06.001 v1.8 declares E-REG-003 payload includes offending_event + offending_tool; BC-3.08.001 v1.7 + implementation only emit offending_plugin
- BC-7.06.001:190-202: E-REG-003 wire payload lists offending_plugin/event/tool
- BC-3.08.001:109-120 (canonical SS-03 catalog): only offending_plugin
- Impl emit_event.rs:221-239: only emits offending_plugin
- Sibling bats S8 (line 651-665): asserts only offending_plugin
- Defect since fix-burst-7 (BC-7.06.001 v1.6); survived 7+ passes
- Fix: PO chooses canonical surface — either trim BC-7.06.001 or extend BC-3.08.001+impl

### F-P14-002 [HIGH] DI-019 cites stale main.rs:308-312 three times — actual is :329-334 (EC-012 line drift)
- invariants.md:148, 150, 156 all cite 308-312
- Actual: 329-334 post-EC-012 partial-drain refactor
- 3rd occurrence of post-EC-012 line drift (F-P10-002, F-P13-002 prior)
- Pattern recurrence justifies process-gap codification
- Fix: Migrate to TD-VSDD-091 stable anchors OR refresh line numbers

### F-P14-003 [MEDIUM] VP-079 spec preferred cargo-mutants targets WRONG test file (event_emission_fault_injection.rs — the file F-P1-002 flagged as INSUFFICIENT for production-path coverage)
- VP-079.md:471-473 spec invocation
- Spec-internal contradiction: Property 6 says "Scenarios 1-5 must fail" (those live in bats); preferred mutants target Rust unit tests
- Fix: Spec line 473 should target `bats tests/bats/async-event-schema-conformance.bats`

### F-P14-004 [MEDIUM] Test-file version citation propagation gap across 5 files (bats + Rust integration tests)
- 5 files cite stale versions: vp079-scenario6:5/21/34 (VP-079 v1.11→v1.12); async-event-schema-conformance:5/21/29 + 571/581/588/630/634; hooks-registry-lint:20; lint-registry-async-invariant:17; event_emission_fault_injection.rs:35-36 (six-version drift VP-079 v1.6, nine-version drift S-15.01 v1.6)
- Fix: Either sweep all test-file cites OR strip version pins (use unversioned references)

### F-P14-005 [LOW] BC-7.06.001 line 106 wording ambiguous (reads as describing current defect rather than F-P8-001 fix already in place)
- Cosmetic rewording

## ADR-013 clock
- Pass-14: HIGH (RESET 0)
- Counter: 0_of_3

## Process-gap candidates (NEW)
1. Post-refactor line-cite sweep — F-P10-002 + F-P13-002 + F-P14-002 (3 instances, confirmed pattern)
2. POLICY 8 scope ambiguity — does propagation extend to test-file doc comments?
3. Cross-BC wire-format contradiction detection — review axis for adversary

## Strategic recommendation (adversary)
Pass-14 explicitly recommends: A (continue), B (surface to user), or C (time-box). Trajectory 14 passes deep, oscillating, finding genuine defects via fresh angles faster than fix-burst velocity can close. Adversary recommendation: surface strategic decision to user.
