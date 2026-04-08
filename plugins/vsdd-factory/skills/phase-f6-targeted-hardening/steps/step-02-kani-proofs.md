# Step 2: Formal Verification (Kani Proofs)

Spawn `formal-verifier` agent to write and run Kani proofs for new verification properties.

## Inputs

- Verification delta from Phase F2
- New/modified source modules

## Actions

1. Spawn `formal-verifier` agent
2. Read verification properties from `.factory/phase-f2-spec-evolution/verification-delta.md`
3. Write Kani proof harnesses for each new verification property
4. Run proofs: `cargo kani --harness <name>`
5. For each property:
   - PASS: record in verification log
   - FAIL: identify the violation, fix implementation, re-run
   - UNREACHABLE: verify the property is correctly specified
6. If non-Rust project: substitute appropriate formal verification tool or skip with justification

## Outputs

- `.factory/phase-f6-hardening/kani-results.md`

## Completion Criteria

- All new verification properties have proof harnesses
- All proofs pass (or failures are fixed and re-verified)
- Non-Rust projects have justified skip or alternative tool
