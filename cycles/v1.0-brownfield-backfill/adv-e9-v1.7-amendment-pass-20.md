# Adversarial Review — E-9 v1.20 Convention-Closure Burst (D-261) — Pass 20

**Date:** 2026-05-05
**Commit reviewed:** edb340a (v1.20; unchanged from pass-19)
**Cumulative surface:** v1.7..v1.20 (4 files + open-questions.md + BC-1.05.035/036 in scope per D-224 cycle inclusion)
**Verdict:** SUBSTANTIVE
**ADR-013 clock at end of pass:** 1_of_3 → 0_of_3 reset
**Pass methodology angle:** Pre-implementation readiness audit — simulate the S-9.07 implementer (most-complex batch with subprocess) consuming the v1.20 amendment surface as their concrete implementation specification. Enumerate implementation cues required (interface signatures, error types, retry policies, timeout values, capability schema, registry-TOML field shapes, telemetry event names) and identify cues missing or under-specified where implementer would be forced to make decisions outside spec. NEW per TD-VSDD-057.

## Summary

Convention machinery at v1.20 structurally sound. However, fresh-context readiness simulation surfaces 2 MED gaps that would force S-9.07 implementer outside spec: (1) `timeout_ms` and `max_output_bytes` values not pinned (only advisory ranges); (2) BC-1.05.036 emits non-ADR-015-conforming event name despite amendment surface absorbing ADR-015 awareness. 19 prior passes (focused on convention/anchor/citation correctness) read spec as normative document; pass-20 is first to read it as implementation-cue surface for downstream story implementer.

## Findings

### HIGH
None.

### MED

**M-P20-001 [MED]: S-9.07 subprocess capability values under-specified — `timeout_ms` and `max_output_bytes` not pinned.**
- File: E-9 line 385 OQ-3 resolution.
- OQ-3 pins `binary_allow = ["bash"]`, `shell_bypass_acknowledged = "acknowledged"`, `env_allow = ["PATH"]`, `cwd_allow = []`. But `vsdd::exec_subprocess` ABI per gap-analysis §1 lines 45-46 requires `timeout_ms: u32` and `max_output_bytes: u32` as call-site arguments. Gap-analysis §4 line 166 ("30s cap is fine") + §5 line 205 ("5-30s cap") and §4 line 165 ("easily under 64KB") are advisory ranges, not contract values.
- S-9.07 implementer must invent values with no spec authority.
- Fix: extend OQ-3 to pin `timeout_ms = 30000` + `max_output_bytes = 65536`.

**M-P20-002 [MED]: BC-1.05.036 emits `host.exec_subprocess.completed` violating ADR-015 D-15.2 reverse-DNS naming.**
- File: BC-1.05.036 §Description + §Postconditions.
- Event name no `vsdd.` prefix, no `.v1` suffix. ADR-015 D-15.2 mandates reverse-DNS. OQ-W16-001 explicitly tracks the binding to `vsdd.host.exec_subprocess.completed.v1` (option a) or `vsdd.dispatcher.subprocess_completed.v1` (option b).
- BC was never updated for ADR-015 awareness despite v1.7 amendment.
- Fix: add ADR-015 awareness clause binding event name to OQ-W16-001 resolution.

### LOW

**L-P20-001 [LOW] (pending intent verification): BC-1.05.036 EC-006 declares canonicalized full path but BC-1.05.035 only canonicalizes for allow-check.**

**L-P20-002 [LOW]: BC-1.05.036 §Postconditions item 5 "existing distinct events continue to fire on error paths" — only `internal.capability_denied` fires; TIMEOUT/OUTPUT_TOO_LARGE emit nothing per gap-analysis §1.**

## Out-of-scope-but-noted

- BC-1.05.035 §Postconditions item 4 returns INVALID_ARGUMENT (-4) for symlink-escape but emits `internal.capability_denied` event — error-code/event-name misalignment. Out of v1.20 amendment scope.

## Process-gaps

- [process-gap PG-P20-001]: TD-VSDD-073 (last_amended convention) covers arch-doc-class only. BCs cited in amendment landings (BC-1.05.035 + BC-1.05.036) are not covered. M-P20-002 is a direct symptom: gap-analysis amendment changed the emit-contract for the event BC-1.05.036 specifies, but BC-1.05.036 was not amended. File as TD-VSDD-074 (TD-VSDD-073 scope extension to BCs).

## Convention checks

- Frontmatter `version:` matches latest non-reserved row (1.20): PASS
- Arch-doc-class files have `last_amended: 2026-05-05`: PASS
- v1.7-v1.20 summary rows intact (POLICY 1): PASS
- v1.21 preemptive reserved row: PASS (line 484)
- v1.20 H3 section present: PASS (line 1146)
- v1.7-v1.19 block prose preserved as authored (POLICY 1 append-only): PASS
- No "Lines: X → Y" footer at v1.7-v1.20: PASS
- H3 version count matches summary table: PASS
- audit-w16.md line 38 sibling-wording-template consistency: PASS
- No fix-burst-internal IDs leak into permanent specs body: PASS
- Outbound decision-ID anchors semantically compatible: PASS
- Citation line numbers accurate: PASS
- OQ-W16-001 propagated to E-9 Open Questions table: PASS
- No retired-figure residue in non-changelog body: PASS

## Angle-specific outputs (Pre-implementation readiness)

17 implementation cues enumerated. 15 PRESENT, 2 MISSING/CONFLICTING (M-P20-001 timeout/output values; M-P20-002 event-name conflict between BC-1.05.036 and ADR-015). Net: S-9.07 implementer must STOP at `vsdd::exec_subprocess(...)` call site (no timeout/output values) AND at `emit_event!(...)` call site (3-way choice between BC name vs OQ-W16-001 (a) vs (b)).
