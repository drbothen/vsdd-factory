# F5 Adversary Pass-17 — S-15.01 fix-burst chain (against 2d700d94)

## Sanity probes — All confirmed
## Pass-16 finding resolutions verified — 3 PARTIAL (F-P16-001, 004, 005); 5 fully resolved

## Verdict: HIGH (substantive findings; meta-pattern recurrence)
Trajectory: 17→15→6→5→0→2→5→1→4→2→2→4→4→5→7→5→4

## Counts
H: 3  M: 1  L: 0  + 4 observations (1 process-gap)

## META-PATTERN
Same fix-burst-15 that codified TD-031 stable-anchor migration LEFT SITES array with stale line ranges (drift 1/5/7/7 lines vs live main.rs). Continuation of TD-031 surface-only-codification problem first observed in pass-16.

## Findings (NEW)

### F-P17-001 [HIGH/CRITICAL] VP-079 v1.15 §Proof Harness Skeleton SITES array line ranges drifted 1/5/7/7 lines vs live main.rs — TD-031 violation introduced by fix-burst-15
- VP-079.md:526-537 SITES array cites: 142,147 + 162,167 + 416 + 427 (stale per pass-16); should be 143,150 + 167,174 + 423 + 434
- bats file (test-writer's `2d700d94`) is correct; spec inline harness skeleton was NOT refreshed
- F-P10-002 (v1.11) precedent established that ranges MUST track main.rs
- Operational impact: any reader copying spec instead of bats produces broken mutation harness (silent PASS)
- Fix: VP-079 v1.15→v1.16 — refresh SITES array to live values; verify against grep

### F-P17-002 [HIGH] BC-3.08.001 v1.10 §Common Fields claims plugin_version for all 4 events; impl emits ZERO plugin_version
- §Common Fields paragraph (added v1.10 to fix F-P16-005) introduces NEW contradiction
- emit_event.rs: zero with_plugin_version calls; only with_plugin_name on Events 1+4
- Wire-format examples for Events 1+4 show only plugin_name (not plugin_version)
- F-P16-005 fix was plugin_name-only correct; left plugin_version mis-described
- Fix: Either (a) remove plugin_version from §Common Fields (matches impl) OR (b) extend impl to emit plugin_version (likely a — matches current state + bats assertions)

### F-P17-003 [HIGH] S-15.01 AC-018 line 401 retains stale :581-700 bats range — F-P16-004 sibling propagation gap
- VP-079 v1.15 §Scenario 8 migrated to stable anchor (line 625)
- S-15.01 AC-018 line 401 still cites `tests/bats/async-event-schema-conformance.bats:581-700`
- Actual S8 @test at line 613
- Per S-7.01 partial-fix discipline: 2-file blast radius
- Fix: S-15.01 v1.19→v1.20 update line 401 to match VP-079 stable anchor

### F-P17-004 [MEDIUM] vp079-scenario6.bats header internally inconsistent
- Line 28 cites VP-079 v1.15
- Lines 5, 21, 37 cite VP-079 v1.13 (stale-by-2)
- Lines 36, 38 cite BC-3.08.001 v1.7 + S-15.01 v1.7 (stale-by-3+)
- Fix: refresh 4 header cites OR replace with stable anchors

## Observations

### O-P17-001 [LOW] VP-079 §Property 6: "four event types" / "five caller sites" terminology asymmetry
### O-P17-002 [stale] DI-019 cite confirmed correct
### O-P17-003 [process-gap] TD-031 enforcement still surface-only after fix-burst-15 escalation
### O-P17-004 [pending intent] BC-3.08.001 §Mandatory fields exclude plugin_version (root same as F-P17-002)

## ADR-013 clock
- Pass-16: HIGH (RESET 0)
- Pass-17: HIGH
- Counter: 0_of_3

## Strategic note
17th pass. Recurrence pattern stable. TD-031 codification surface-only without enforcement. Fix-burst-16 should: (1) close pass-17 findings; (2) IMPLEMENT TD-031 enforcement lint-hook so future fix-bursts cannot introduce TD-031 violations.
