# F5 Adversary Pass-11 — S-15.01 fix-burst chain (against f7faad3)

## Sanity probes
All confirmed.

## Pass-10 finding resolutions verified
F-P10-001/002 + O-P10-001/002/003 ALL RESOLVED.

## Verdict: LOW
Trajectory: 17 → 15 → 6 → 5 → 0 → 2 → 5 → 1 → 4 → 2 → 2 (continuing oscillation; partial-fix gaps recur)

## Counts
H: 0  M: 0  L: 2  NIT: 0

## Findings (NEW)

### F-P11-001 [LOW] VP-079 Scenario 6 mutation harness has 4 sites; Property 6 (v1.11) declares 5. SITE_5 not mechanized; bash sed conflates SITE_2+SITE_5
- Axis: S-7.01 partial-fix + Angle DD (mutation harness coverage)
- Source: VP-079.md:83-86 enumerates 5 sites; tests/bats/vp079-scenario6-mutation-counter-proof.bats:21-25 + 192-207 mechanizes 4. SITE_5 (main.rs:162 DuplicateEntry) not separately tested.
- Bash sed `/${fn_pattern}(/s/...` with pattern `emit_dispatcher_registry_invalid` matches BOTH line 142 (SITE_2) AND line 162 (SITE_5) — conflated mutation trial.
- Header cites also stale: bats:5 says VP-079 v1.7 (current v1.11); bats:32 says BC-3.08.001 v1.5 (current v1.7).
- Defect: F-P9-004 spec addition of SITE_5 was not propagated to bash counter-proof harness. SITE_5 functionally covered by Scenario 8 production-path test, but mutation counter-proof incomplete.
- Fix: Add 5th @test arm using line-range sed targeting line 162; bump header cites to current spec versions; refactor mutate_and_verify_caught helper to take line-range param.
- Tag: spec-harness-drift, mutation-counter-proof-gap, S-7.01-propagation

### F-P11-002 [LOW pending intent] lint plugin lib.rs:14 cites BC-7.06.001 v1.6 but BC is v1.7; v1.7 introduced async_block_conflict canonical string the impl now emits
- Axis: POLICY 4 + S-7.01
- Source: crates/hook-plugins/lint-registry-async-invariant/src/lib.rs:14 doc-comment cites v1.6; companion test integration_test.rs:329,386 correctly cites v1.7
- Defect: Impl emits v1.7 canonical value; doc-comment cites v1.6 (pre-canonicalization). Sibling-propagation gap from F-P10-001 fix-burst.
- Fix: lib.rs:14 cite v1.6 → v1.7. (Pending intent: confirm convention requires bumping doc-comment cites on every BC amendment)
- Tag: policy-4-stale-cite

## ADR-013 clock
- Pass-5: NITPICK_ONLY (1_of_3)
- Pass-6/7/8/9/10: each MEDIUM/HIGH (RESET 0)
- Pass-11: LOW
- Counter: 0_of_3 — does NOT advance

## Recommendation
Fix-burst-10: F-P11-001 (bats SITE_5 arm + header cite refresh) + F-P11-002 (lib.rs:14 cite bump v1.6→v1.7).
