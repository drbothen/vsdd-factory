# F5 Adversary Pass-13 — S-15.01 fix-burst chain (against e5108a2)

## Sanity probes
All confirmed.

## Pass-12 finding resolutions verified
F-P12-001/002 + O-P12-001/002 ALL RESOLVED structurally.

## Verdict: HIGH
Trajectory: 17→15→6→5→0→2→5→1→4→2→2→4→4

## Counts
H: 2  M: 2  L: 3  NIT: 0

## Findings (NEW)

### F-P13-001 [HIGH] VP-079 §Proof Harness Skeleton retains broken sed (pre-F-P12-001)
- Source: VP-079.md:519 uses pattern-only sed; bats:129 uses line-range sed
- Defect: Spec is canonical reference for any future test-writer; broken there means future rebuilds will hit same vacuous-mutation bug. Spec also iterates only 4 SITES while Property 6 v1.11 enumerates 5.
- Fix: VP-079 v1.11 → v1.12 — rewrite §Proof Harness Skeleton with line-range SITES array covering all 5 sites; mirror bats helper logic.

### F-P13-002 [HIGH] BC-7.06.001 line 106 cites stale main.rs lines (148-151, 143-145)
- Source: BC-7.06.001.md:106 §Fail-Closed Symmetry implementation note
- Actual at HEAD e5108a2: catch-all _ => 0 at line 173; AsyncBlockConflict arm at 139-152
- Defect: Post-EC-012 line shift not propagated to BC. Same class as F-P10-002.
- Fix: BC-7.06.001 v1.7 → v1.8. Use stable anchors per TD-VSDD-091 (e.g., RegistryError::DuplicateEntry arm in factory_dispatcher::main::run).

### F-P13-003 [MEDIUM] S-15.01 References cites VP-079 v1.10; actual is v1.11
- Source: S-15.01.md:819
- Defect: Fix-burst-9 amended VP-079 to v1.11 but story References not propagated.
- Fix: Story v1.14 → v1.15.

### F-P13-004 [MEDIUM] vp079-scenario6 PATH dependency may make mutations no-op
- Source: bats:84 uses bats run with conformance file invoking factory-dispatcher from PATH
- Defect: Freshly-built target/debug/factory-dispatcher may not be in PATH; mutations have zero effect on the binary scenarios actually run against.
- Fix: Inject PATH="$SRC_ROOT/target/debug:$PATH" before bats invocation.

### F-P13-005 [LOW] BSD sed inline brace-form portability concern (pending intent)

### F-P13-006 [LOW] run_scenarios_1_to_5 helper name misleading (runs S1-S5 + S7 + S8)

### O-P13-001 [LOW] Cosmetic marker divergence MUTANT: vs MUTANT-SUPPRESSED: (subsumed by F-P13-001 fix)

## ADR-013 clock
- Pass-5: NITPICK_ONLY (1_of_3)
- Pass-6/7/8/9/10/11/12: each MEDIUM/HIGH/LOW (RESET 0)
- Pass-13: HIGH
- Counter: 0_of_3

## Trajectory observation
13 passes; cycle catches real defects each pass via new angles. Sibling propagation is ongoing concern. After fix-burst-12 + pass-14, strategic decision may be warranted if substantive defects continue surfacing.

## Recommendation
Fix-burst-12: VP-079 v1.12 spec rewrite (HIGH) + BC-7.06.001 v1.8 line cite refresh (HIGH) + story propagation (MED) + bats PATH fix (MED) + LOW cleanups.
