# F5 Adversary Pass-9 — S-15.01 fix-burst chain (against 8b0050a)

## Sanity probes
All confirmed.

## Pass-8 finding resolution verified
F-P8-001 RESOLVED.

## Verdict: MEDIUM
Trajectory: 17 → 15 → 6 → 5 → 0 → 2 → 5 → 1 → 4

## Counts
H: 0  M: 1  L: 3  NIT: 0

## Findings (NEW)

### F-P9-001 [MEDIUM] BC-7.06.001 line 204 stale "Sibling BC-3.08.001 note" misstates v1.7's completed state
- Axis: T + S-7.01 propagation
- Source: BC-7.06.001.md:204 says "BC-3.08.001 v1.6 currently enumerates only error_code: E-REG-002 with violation: on_error_block_with_async_true. E-REG-003 ... MUST be added" — but BC-3.08.001 is now v1.7 with both codes enumerated and async_block_conflict canonical.
- Defect: Operative-body factual error in a current spec; misleads readers about cross-reference state.
- Fix: Update line 204 to reflect BC-3.08.001 v1.7 completed state.
- Tag: spec-body-staleness

### F-P9-002 [LOW] Bats Scenario 3 (AC-013 / E-REG-002) does not assert canonical `violation` field value
- Axis: S + U
- Source: tests/bats/async-event-schema-conformance.bats:293-343 — checks field PRESENCE only, not value
- Defect: Asymmetric rigor vs Scenario 8 (which asserts violation=duplicate_hook_registration); regression to old `on_error_block_with_async_true` would NOT be caught at bats level (Rust unit test event_emission_fault_injection.rs IS covered).
- Fix: Add violation field-value assertion to Scenario 3, mirroring Scenario 8 pattern.
- Tag: test-coverage-gap

### F-P9-003 [LOW pending intent] Source-code doc-comment BC version-label staleness across 4 files / 13 sites
- Axis: S-7.01 sibling propagation (pending intent)
- Files: partition.rs (3 sites cite v1.5, current v1.6); registry.rs (2 sites cite v1.5); vp078_harness3.rs (3-4 sites); host/emit_event.rs (~22 sites cite v1.5/v1.6, current v1.7)
- Defect: Fix-burst-7 swept stories/specs but did NOT extend version-label propagation to source-code doc-comments
- Fix: Sweep 13+ source-code doc-comment sites; update BC-7.06.001 v1.5/v1.6 → v1.6 and BC-3.08.001 → v1.7. (User-decision-pending: should source-code doc-comments be in scope for POLICY 8?)
- Tag: propagation-gap

### F-P9-004 [LOW] VP-079 Property 6 SITES enumeration missing main.rs:162 (5th DuplicateEntry caller)
- Axis: S + V
- Source: VP-079.md:80-90 enumerates 4 caller sites (133/142/394/405); F-P8-001 added 5th site at main.rs:162 for DuplicateEntry; spec text doesn't reflect this
- Defect: Documentary, not behavioral. cargo-mutants pattern still catches via fn-name regex.
- Fix: Update VP-079 v1.9 → v1.10 to enumerate 5 sites (add SITE_5: main.rs:162 DuplicateEntry/E-REG-003 path).
- Tag: spec-text-completeness

## ADR-013 clock
- Pass-5: NITPICK_ONLY (1_of_3) | Pass-6/7/8: MEDIUM (RESET 0)
- Pass-9: MEDIUM
- Counter: 0_of_3 — does NOT advance

## Recommendation
Fix-burst-8: address all 4 findings (per "most correct" rule). Targeted edits:
- BC-7.06.001 v1.6 → v1.7: clean line 204 stale sibling note
- VP-079 v1.9 → v1.10: enumerate 5th SITE
- Source-tree doc-comment sweep: 13+ sites
- Bats Scenario 3: add violation field-value assertion
