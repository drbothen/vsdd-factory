# F5 Adversary Pass-8 — S-15.01 fix-burst chain (against 4b2eb6a)

## Sanity probes
All confirmed.

## Pass-7 finding resolutions verified
F-P7-001/002/003/004/005 ALL RESOLVED.

## Verdict: MEDIUM
Trajectory: 17 → 15 → 6 → 5 → 0 → 2 → 5 → 1 (improving)

## Counts
H: 0  M: 1  L: 0  NIT: 0

## Finding (NEW)

### F-P8-001 [MEDIUM] Spec-impl drift — RegistryError::DuplicateEntry silently exits 0 despite BC saying "dispatcher refuses to start"
- Axis: O (error msg quality) + M (boundary scrutiny on production logic)
- Source: BC-7.06.001 v1.5 line 87 ("Hard-error with RegistryError::DuplicateEntry {...} on the first violation; dispatcher refuses to start") + line 158 (Canonical Test Vector)
- Impl defect: main.rs:148-151 has catch-all `_ => 0` consuming DuplicateEntry. Compare:
  - SchemaVersion (E-REG-001) → main.rs:134-136: stderr eprintln + exit 2 (fail-closed)
  - AsyncBlockConflict (E-REG-002) → main.rs:143-145: stderr eprintln + exit 2 (fail-closed)
  - DuplicateEntry → main.rs:148-151: NO stderr, exit 0 (fail-open) — silent failure
- BC says all three are "refuses to start" — same spec contract, asymmetric implementation
- Operational risk: operator pastes duplicate [[hooks]] row → dispatcher silent-exits 0 → no hooks loaded → no observable signal. This is exactly the silent-failure class the cycle is trying to PREVENT.
- Test-coverage gap (compounding): no bats scenario covers DuplicateEntry. VP-079 has S2 (schema_mismatch) and S3 (registry_invalid) but NO scenario for tuple-uniqueness violation reaching the dispatch path.
- Required fix (Resolution 1 — tighten impl per "most correct"):
  1. main.rs: add `RegistryError::DuplicateEntry { ... } => { eprintln!("[E-REG-003] ..."); emit_dispatcher_registry_invalid(...); 2 }` arm
  2. registry.rs: add `[E-REG-003]` prefix to DuplicateEntry error message
  3. BC-7.06.001 v1.5 → v1.6: explicit `[fail-closed]` classification on Invariant 7; clarify "refuses to start" semantics
  4. tests/bats/async-event-schema-conformance.bats: add Scenario 8 for DuplicateEntry → exit 2 + dispatcher.registry_invalid event
- Tag: spec-impl-drift, fail-closed-asymmetry, S-15.01-introduced

## ADR-013 clock
- Pass-5: NITPICK_ONLY (1_of_3) | Pass-6: MEDIUM (RESET 0) | Pass-7: MEDIUM (RESET 0) | Pass-8: MEDIUM (RESET 0)
- Counter: 0_of_3

## Process-gap codification candidates (NEW)

1. [process-gap] Spec-impl drift on fail-closed/fail-open classification — every RegistryError variant should map to exactly one of fail-closed-with-stderr or operational-fail-open arms; CI gate to enforce. F-P8-001 demonstrates DuplicateEntry slipped through to fail-open silently when spec said fail-closed.
2. [process-gap] Bats integration tests should cover all RegistryError variants — when a new variant is added, automatic check enforces creation of corresponding bats scenario.

## Recommendation
Fix-burst-7 (substantive single-finding): main.rs arm + E-REG-003 + stderr + event + bats scenario + BC v1.6 amendment + story body propagation.
