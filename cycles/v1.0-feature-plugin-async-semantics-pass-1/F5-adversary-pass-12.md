# F5 Adversary Pass-12 — S-15.01 fix-burst chain (against 70652a6)

## Sanity probes
All confirmed.

## Pass-11 finding resolutions
F-P11-001 + F-P11-002 RESOLVED structurally; F-P11-001 has FUNCTIONAL DEFECT (see F-P12-001).

## Verdict: HIGH
Trajectory: 17 → 15 → 6 → 5 → 0 → 2 → 5 → 1 → 4 → 2 → 2 → 4 (regressed; pass-11 introduced vacuous scaffolding)

## Counts
H: 1  M: 0  L: 1  NIT: 2 (observations pending intent verification)

## Findings (NEW)

### F-P12-001 [HIGH] SITE_2/SITE_5 mutation strategy is structurally vacuous due to multi-line emit call
- Axis: Y (final integrity audit)
- Source: tests/bats/vp079-scenario6-mutation-counter-proof.bats:128 line-range sed; main.rs:142-147 + 162-167 (6-line emit calls)
- Defect: With line_range="142,142" or "162,162", sed comments only the first line of a 6-line call. Result is syntax error (`&err_ctx,` becomes stray expression). `cargo build` fails → helper returns caught=2 (INFRA-ERROR) → caller treats as success. The F-P11-001 fix added scaffolding (line_range param, SITE_5 arm) but the mutation doesn't actually exercise the production path for these two sites.
- Net coverage delta from pass-11 fix on SITE_2/SITE_5 = ZERO.
- Fix: Extend line_range semantics to span full multi-line call (e.g., "142,147", "162,167") AND drop the `/${fn_pattern}(/` filter when range is provided OR comment every line in range. ALTERNATIVELY: reject caught==2 as test failure when line_range is provided (precise mutation must produce buildable binary).
- Tag: structural-defect, vacuous-test, S-15.01-introduced

### F-P12-002 [LOW] bats SITE_3/SITE_4 in-test annotations cite stale main.rs line numbers
- Source: bats:234 cites ~main.rs:394 (actual: 416); bats:257 cites ~main.rs:405 (actual: 427)
- Defect: SITES header at lines 22-26 was updated correctly per F-P10-002; per-test annotations not propagated. Doesn't affect mutation behavior (no line_range used for SITE_3/4) but misleads readers.
- Fix: Update annotations 394→416, 405→427.
- Tag: annotation-drift

### O-P12-001 [LOW pending intent] Sibling bats test header version labels stale across 4 files
- Source: async-event-schema-conformance.bats:24,29,30 (cite v1.4/v1.6/v1.6 — canonical v1.7/v1.11/v1.14); lint-registry-async-invariant.bats:17,19; hooks-registry-lint.bats:20,22; envelope-sync-invariant.bats:16
- Pass-11 F-P11-001 refreshed cites in vp079-scenario6 only; sibling bats files in same `tests/bats/` directory not updated.
- Pending intent: stale labels may pin to "version where behavior was introduced" (acceptable convention) OR "current canonical" (drift)

### O-P12-002 [LOW pending intent] Source files cite BC-7.06.001 v1.6 Invariant 7 (12+ sites) while BC is canonically v1.7
- Source: main.rs:159, partition.rs:150/181/258, registry.rs:1076/1104, vp078_harness3:33/34/58/66, async-event-schema-conformance.bats:571/581/588/630/634
- Pass-11 F-P11-002 refreshed lint plugin lib.rs:14 cite only; sibling source/test files not updated.
- Pending intent: same as O-P12-001 — version pinning convention adjudication

## ADR-013 clock
- Pass-5: NITPICK_ONLY (1_of_3)
- Pass-6/7/8/9/10/11: each MEDIUM/HIGH/LOW (RESET 0)
- Pass-12: HIGH
- Counter: 0_of_3 — does NOT advance

## Recommendation
Fix-burst-11 — substantial:
- F-P12-001 (HIGH): bats helper extends line_range semantics to span multi-line calls; OR fail-on-INFRA-ERROR when line_range provided
- F-P12-002 (LOW): bats SITE_3/SITE_4 annotation refresh
- O-P12-001/002 (USER-DECIDED per "most correct, not fastest"): sweep ALL sibling cites to current canonical versions. ~16+ sites across bats + source files.
