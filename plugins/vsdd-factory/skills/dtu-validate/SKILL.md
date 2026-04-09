---
name: dtu-validate
description: Digital Twin Universe validation — create and maintain behavioral clones of critical subsystems for regression detection. DTU clones run in parallel with implementation to catch behavioral divergence early.
disable-model-invocation: true
allowed-tools: Read, Write, Edit, Bash, Glob, Grep
---

# DTU Validation (Digital Twin Universe)

Maintain behavioral clones of critical subsystems.

## Templates

Read and follow the output format in:
- `${CLAUDE_PLUGIN_ROOT}/templates/dtu-assessment-template.md` — DTU assessment structure
- `${CLAUDE_PLUGIN_ROOT}/templates/dtu-clone-spec-template.md` — per-clone specification
- `${CLAUDE_PLUGIN_ROOT}/templates/dtu-fidelity-report-template.md` — fidelity comparison report DTU clones are simplified reimplementations that capture the essential behavior — when the clone and the real implementation diverge, something has drifted.

## Concept

A DTU clone is NOT a copy of the code. It's an independent, minimal implementation of the same behavioral contracts, written to be obviously correct rather than performant. When both implementations agree, confidence is high. When they disagree, one of them has a bug.

## Process

### 1. Identify DTU Candidates

Read `.factory/specs/prd-supplements/module-criticality.md`:
- **CRITICAL** modules get DTU clones (mandatory)
- **HIGH** modules get DTU clones (recommended)
- MEDIUM/LOW modules do not (cost > benefit)

### 2. Create DTU Clone

For each candidate module:

1. Read the behavioral contracts (BCs) for that module
2. Write a minimal implementation that satisfies the BCs:
   - Prioritize clarity over performance
   - No optimization — brute force is fine
   - No error handling beyond what BCs require
   - Simple data structures (Vec instead of HashMap if small N)

3. Write to `.factory/dtu-clones/<module>/`:
   ```
   dtu-clones/
   └── <module>/
       ├── clone.rs          # The simplified implementation
       ├── harness.rs        # Test harness that runs both implementations
       └── README.md         # What this clone covers
   ```

### 3. Create Comparison Harness

The harness:
- Takes the same inputs
- Runs both the real implementation and the DTU clone
- Compares outputs
- Reports divergences

```rust
fn compare(input: &Input) -> ComparisonResult {
    let real_result = real_implementation(input);
    let clone_result = dtu_clone(input);
    
    if real_result == clone_result {
        ComparisonResult::Match
    } else {
        ComparisonResult::Divergence {
            input: input.clone(),
            real: real_result,
            clone: clone_result,
        }
    }
}
```

### 4. Run DTU Validation

```bash
cargo test --test dtu_*
```

Or run the comparison harness with property-based inputs:
```rust
#[test]
fn dtu_fuzz_comparison() {
    proptest!(|(input in any_valid_input())| {
        let result = compare(&input);
        assert!(matches!(result, ComparisonResult::Match),
            "DTU divergence on input: {:?}", input);
    });
}
```

### 5. Maintain DTU Assessment

Write/update `.factory/specs/gene-transfusion-assessment.md`:

```markdown
# DTU Assessment

## Active Clones

| Module | BCs Covered | Last Validated | Status |
|--------|------------|----------------|--------|
| <module> | BC-1.01.001, BC-1.01.002 | 2026-04-01 | ✅ in-sync |

## Divergences Found

| Date | Module | Input | Real Output | Clone Output | Resolution |
|------|--------|-------|-------------|-------------|------------|
```

## When to Run

- After each wave merge (before holdout evaluation)
- After any refactoring of CRITICAL/HIGH modules
- As part of convergence check (Phase 6)
- When mutation testing finds survivors in DTU-covered code

## Output

Report divergences immediately. A DTU divergence in a CRITICAL module is a **blocking** finding — implementation must be fixed before proceeding.
