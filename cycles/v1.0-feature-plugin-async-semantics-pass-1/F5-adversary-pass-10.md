# F5 Adversary Pass-10 — S-15.01 fix-burst chain (against 6e9efcb)

## Sanity probes
All confirmed.

## Pass-9 finding resolutions verified
F-P9-001/002/003/004 ALL RESOLVED.

## Verdict: HIGH
Trajectory: 17 → 15 → 6 → 5 → 0 → 2 → 5 → 1 → 4 → 2 (substantive findings remain)

## Counts
H: 1  M: 1  L: 3  NIT: 0

## Findings (NEW)

### F-P10-001 [HIGH] lint-registry-async-invariant WASM plugin emits LEGACY violation string `on_error_block_with_async_true`
- Axis: Y + Z2 (final sweep + self-consistency of pass-9 fix)
- Source: crates/hook-plugins/lint-registry-async-invariant/src/lib.rs:176 emits `("violation", "on_error_block_with_async_true")` for E-REG-002 path
- Canonical: BC-3.08.001 v1.7 line 89 + BC-7.06.001 v1.7 line 188 specify `async_block_conflict`. Dispatcher main.rs:146 uses canonical. Tests use canonical.
- Defect: LIVE WASM plugin (registered in hooks-registry.toml:957-967, PostToolUse + Edit|Write + on_error=block + priority=160) emits WRONG wire-format violation string when triggered by a developer's edit to hooks-registry.toml. Sink consumers per BC-3.08.001 v1.7 schema constraint will reject as schema violation.
- F-P9-003 sweep targeted VERSION LABELS not ENUM STRING VALUES — process gap.
- Fix: lib.rs:176 replace string literal; rebuild WASM artifact at plugins/vsdd-factory/hook-plugins/lint-registry-async-invariant.wasm; add unit test for emit_event payload field value.
- Tag: propagation-gap, canonical-string-drift, wire-format-divergence, S-15.01-introduced

### F-P10-002 [MEDIUM] VP-079 v1.10 SITE_3 / SITE_4 line citations stale by ~22 lines (EC-012 refactor shift)
- Axis: Y + S-7.01 partial-fix regression
- Source: VP-079.md:83-86 (Property 6) + :460-466 (Scenario 6 SITES comment) cite main.rs:394 + main.rs:405 for emit_plugin_async_block_discarded + emit_plugin_timeout_async
- Actual sites at HEAD 6e9efcb: main.rs:416 + main.rs:427 (EC-012 partial-drain refactor added ~22 lines)
- Defect: F-P9-004 fix-burst added SITE_5 at line 162 and verified that SPECIFIC line, but did NOT re-verify SITE_3 and SITE_4 in same Property 6 enumeration. Cargo-mutants `--filter` matches by fn name (still works mechanically) but spec text is misleading — readers chasing line 394 land on `break;`.
- Fix: VP-079 v1.10 → v1.11; update Property 6 line cites; update Scenario 6 SITES comment block.
- Tag: spec-code-drift, line-citation-staleness

### O-P10-001 [LOW] BC-3.08.001 frontmatter `phase: F8` anomalous
- Source: BC-3.08.001.md:9 says `phase: F8`. Sibling BCs (BC-7.06.001, BC-1.14.001) declare F2.
- Defect: Either stale value or undocumented convention.
- Fix: PO adjudicate; recommend phase: F2 to match siblings.
- Tag: frontmatter-anomaly

### O-P10-002 [LOW] VP-079 timestamp missing UTC Z suffix
- Source: VP-079.md:7 has `timestamp: 2026-05-08T00:00:00` (no Z)
- Sibling files have Z suffix
- Fix: Add Z to VP-079 timestamp on next amendment.
- Tag: iso-8601-format

### O-P10-003 [process-gap] Canonical-string sweeps need separate discipline from version-label sweeps
- Pattern: F-P9-003 swept 38 version-label sites; F-P10-001 surfaces a canonical enum-string drift the version-label sweep cannot detect.
- Codification: future canonical-value renames in BC wire format should include a grep over `**/*.rs` + `**/*.bats` for OLD string value as part of fix-burst sweep checklist.
- Recommend: TD-030 entry.

## ADR-013 clock
- Pass-5: NITPICK_ONLY (1_of_3)
- Pass-6/7/8/9: MEDIUM (RESET 0)
- Pass-10: HIGH
- Counter: 0_of_3 — does NOT advance

## Recommendation
Fix-burst-9: F-P10-001 (HIGH; lint plugin fix + WASM rebuild) + F-P10-002 (MEDIUM; VP-079 line cite refresh) + O-P10-001 (BC-3.08.001 phase F2) + O-P10-002 (VP-079 timestamp Z). Codify TD-030 for canonical-string sweep discipline.
