# F5 Adversary Pass-7 — S-15.01 fix-burst chain (against e2cb7a5)

## Sanity probes
HEAD e2cb7a5; Scenario 7 + regex-variant test confirmed; spec versions confirmed; cargo test PASS.

## Pass-6 finding resolutions verified
F-P6-001 (Scenario 7) and F-P6-002 (regex-variant test) BOTH RESOLVED.

## Review angles chosen
- Angle G (AC traceability — sweep all 17 ACs)
- Angle H (cross-story coherence)
- Angle I (build artifact freshness — source/test/plugin doc-comments)
- Angle J (regression watchpoints)
- Angle K (demo evidence completeness)
- Angle L (red-team)

## Verdict: MEDIUM
Trajectory: 17 → 15 → 6 → 5 → 0 → 2 → 5

## Counts
H: 0  M: 2  L: 3  NIT: 0

## Findings (NEW — Angle I + S-7.01 sibling-sweep, plus AC-005 enumeration drift)

### F-P7-001 [MEDIUM] Stale BC/VP/Story version cites in source/test/plugin doc-comments
- Axis: I + S-7.01 sibling-sweep
- Files (blast radius = 6+):
  - main.rs:296, 317 — BC-1.14.001 v1.7 (current v1.9)
  - executor.rs:236 — BC-1.14.001 v1.7 (current v1.9)
  - partition.rs:294 — VP-077 v1.7 H1-H4 (current v1.10 H1-H6)
  - host/emit_event.rs:125-248 (10 sites) — BC-3.08.001 v1.4 (current v1.6)
  - event_emission_fault_injection.rs:31 — BC-3.08.001 v1.4
  - vp078_harness3.rs:34, 58 — BC-7.06.001 v1.3 (name, event) (current v1.5 (name, event, tool))
  - latency_canary.rs:39, 40, 228 — S-15.01 v1.6/v1.8 (current v1.11)
  - lint-registry-async-invariant/src/lib.rs:14 — BC-7.06.001 v1.3
- Defect: Fix-burst sweeps targeted SPEC artifacts but NOT source-tree doc-comments. S-7.01 sibling-sweep discipline requires propagation to non-binding doc-comments.
- Fix: Sweep all 6+ files; replace stale version labels with current spec versions.
- Tag: propagation-gap, sibling-sweep, stale-version-label

### F-P7-002 [MEDIUM] AC-005 falsifiable-test enumeration does not include Scenario 7
- Axis: G + L
- Source: S-15.01.md:223-226 lists Scenarios 1, 4, 5 only
- Impl: bats Scenario 7 added (commit 638bc5d) for EC-012
- Defect: AC-005 enumeration stale relative to test corpus. POLICY 8 propagation.
- Fix: Append Scenario 7 to AC-005 falsifiable tests.
- Tag: spec-drift, falsifiable-test-enumeration

### F-P7-003 [LOW] partition.rs:294 cites VP-077 v1.7 H1-H4 (current v1.10 H1-H6)
- Subset of F-P7-001; flagged separately for stale-enumeration count change

### F-P7-004 [LOW] latency_canary.rs:228 user-facing assertion error message has stale "S-15.01 v1.8"
- Subset of F-P7-001; flagged separately because line 228 is user-facing assertion output

### F-P7-005 [LOW] vp078_harness3.rs:34, 58 cite OLD (name, event) tuple wording vs current (name, event, tool)
- Subset of F-P7-001; flagged separately for semantic shape change (not just version label)

## ADR-013 clock
- Pass-1: HIGH | Pass-2: HIGH | Pass-3: MEDIUM | Pass-4: MEDIUM | Pass-5: NITPICK_ONLY (1_of_3)
- Pass-6: MEDIUM (RESET 0_of_3)
- Pass-7: MEDIUM
- Counter: 0_of_3 — does NOT advance

## Process-gap candidates (NEW)

1. [process-gap] No automated check enforces that source-tree doc-comments stay in sync with spec version bumps. POLICY 8 targets stories only. Recommendation: extend POLICY 8 to add code/test sibling sweep verification, OR add new policy "source_code_bc_cite_sync" with a lint-hook scanning crates/**/*.rs for stale version labels. Pattern recurred in F-P4-001 (manual identification) and F-P7-001 (recurrence).

2. [process-gap] When test-writer adds NEW falsifiable test artifact (e.g., bats Scenario 7), no automated check propagates artifact name into AC body falsifiable-test enumeration. Recommendation: codify a check that detects new bats `@test` decorators or `fn test_` items and verifies AC enumeration freshness in related stories.

## Convergence trajectory
17 → 15 → 6 → 5 → 0 → 2 → 5 (regressed on count but Angle I/G targets a new surface pass-5/6 didn't sweep)

## Recommendation
Fix-burst-6 = (a) sweep 6+ source/test files for version-label refresh; (b) AC-005 enumeration update. Both localized; mechanical. Pass-8 re-establishes convergence chain.
