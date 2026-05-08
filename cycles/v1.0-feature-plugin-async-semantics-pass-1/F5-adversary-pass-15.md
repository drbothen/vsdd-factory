# F5 Adversary Pass-15 — S-15.01 fix-burst chain (against 1d19d73)

## Sanity probes — All confirmed
## Pass-14 finding resolutions verified — F-P14-001/002/003/004/005 RESOLVED structurally; F-P14-001 propagation INCOMPLETE (F-P15-001+003+006 below)

## Verdict: HIGH
Trajectory: 17→15→6→5→0→2→5→1→4→2→2→4→4→5→7 (regressed; F-P14-001 Path B propagation gap surfaced)

## Counts
H: 3  M: 3  L: 3  NIT: 0  + 1 process-gap

## Findings (NEW)

### F-P15-001 [HIGH] VP-079 Property Statement / Mandatory-Fields table omits E-REG-003 enrichment fields
- VP-079.md:59 lists only `offending_plugin, violation, ...` for dispatcher.registry_invalid
- BC-3.08.001 v1.8:123 mandates additional `offending_event, offending_tool, session_id` for E-REG-003
- F-P14-001 Path B propagation truncated before VP-079
- Fix: VP-079 v1.13→v1.14 — split E-REG-002/E-REG-003 mandatory fields rows

### F-P15-002 [HIGH] BC mandates offending_tool: null for wildcard E-REG-003; impl OMITS field
- BC-3.08.001 v1.8:123: `offending_tool (string or null, required — null when no tool filter)`
- emit_event.rs:252-257 conditionally emits only when Some(tool); None case omits entirely
- Test gap: neither bats S8 nor Rust S8 exercises tool=None wildcard case
- Fix: Path A — emit_event.rs unconditional emit_field("offending_tool", null) for E-REG-003 + test wildcard case

### F-P15-003 [HIGH] VP-079 frontmatter bcs: cites only BC-3.08.001; should also cite BC-7.06.001
- bats S8 verifies BC-7.06.001 v1.9 Invariant 7 (DuplicateEntry fail-closed)
- VP-079.md:31 bcs: [BC-3.08.001] — frontmatter-body coherence gap
- Fix: VP-079 v1.14 frontmatter bcs: [BC-3.08.001, BC-7.06.001]

### F-P15-004 [MEDIUM] VP-079 doesn't enumerate Scenarios 7 + 8
- bats has 7 scenarios + 1 mutation (S1-S5, S7, S8 + S6 mutation)
- VP-079 §Property Statement describes Properties 1-6, references "Scenarios 1-5" + Scenario 6
- Fix: Add §Scenario 7 + §Scenario 8 prose blocks to VP-079 v1.14

### F-P15-005 [MEDIUM] S-15.01 AC-013 silent on E-REG-003; bats S8 + Rust S8 have NO AC anchor
- S-15.01 v1.13 amendment text self-acknowledges gap: "E-REG-003 path not yet assigned to an AC"
- POLICY 1 allows AC additions
- Fix: Add AC-018 (E-REG-003 DuplicateEntry → dispatcher.registry_invalid) to S-15.01 v1.16→v1.17

### F-P15-006 [MEDIUM] VP-079 v1.13 changelog doesn't cite F-P14-001 Path B
- Same fix-burst applied to BC-3.08.001/BC-7.06.001/impl/tests but VP-079 only got cargo-mutants change (F-P14-003)
- Root cause for F-P15-001+003+004
- Fix: VP-079 v1.13→v1.14 changelog cites F-P14-001 Path B propagation

### F-P15-007 [process-gap] Recurrent post-EC-012 line-drift now affects 4+ documents
- Pattern: F-P10-002 (VP-079 SITE_3/4) + F-P13-002 (BC-7.06.001) + F-P14-002 (DI-019) + ...
- TD-VSDD-091 stable-anchor convention exists but no rule enforces it
- Fix: Codify rule under rules/ enforcing TD-VSDD-091 in spec body text outside changelogs

### O-P15-001 [LOW] session_id missing from BC-3.08.001 wire-format examples for Events 1, 2, 4, E-REG-002 path
### O-P15-002 [LOW] coherent — no defect
### O-P15-003 [LOW] RESERVED_FIELDS list not enumerated in BC-3.08.001 §Implementation Notes

## ADR-013 clock
- Pass-14: HIGH (RESET 0)
- Pass-15: HIGH
- Counter: 0_of_3

## Recommendation
Fix-burst-14: VP-079 v1.14 (F-P15-001/003/004/006) + impl Path A (F-P15-002) + bats wildcard test + S-15.01 AC-018 + BC-3.08.001 v1.9 (O-P15-001/003) + rules/ codification (F-P15-007 → TD-031)
