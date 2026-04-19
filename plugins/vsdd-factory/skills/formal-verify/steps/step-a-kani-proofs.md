---
name: step-a-kani-proofs
description: Write and run Kani proof harnesses for pure core functions in CRITICAL and HIGH criticality modules.
---

# Step A: Kani Proofs

> **Shared context:** Read `./_shared-context.md` before executing this step — it contains templates, prerequisites, and module criticality rules.

## Procedure

For functions in the pure core (no side effects) of CRITICAL and HIGH criticality modules:

1. Write proof harnesses:

```rust
#[cfg(kani)]
mod verification {
    use super::*;

    #[kani::proof]
    fn verify_no_panic() {
        let input: u32 = kani::any();
        kani::assume(input < MAX_VALUE);
        let result = function_under_test(input);
        // assert properties
    }
}
```

2. Run each harness:
```bash
cargo kani --harness <harness_name>
```

**What to prove:**
- Absence of panics for all valid inputs
- Arithmetic overflow safety
- Array bounds safety
- Invariant preservation across state transitions

## Artifacts

- Proof harnesses in `src/` (inline `#[cfg(kani)]` modules)
- Kani Results section in `.factory/cycles/<current>/formal-verification-report.md`

## Success Criteria

- All CRITICAL and HIGH modules have proof harnesses
- All harnesses pass (`cargo kani` returns 0)
- Per-harness results documented in report
