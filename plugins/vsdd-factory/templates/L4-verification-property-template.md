---
document_type: verification-property
level: L4
version: "1.0"
status: draft|in-development|verified|withdrawn
producer: architect
timestamp: YYYY-MM-DDTHH:MM:SS
phase: 1b
inputs: [prd.md, architecture.md]
input-hash: "[md5]"
traces_to: prd.md
source_bc: BC-S.SS.NNN  # Primary BC. For VPs that span multiple BCs, set this to the dominant BC and use the `bcs:` list field below to enumerate all covered BCs. The body Source Contract section labels primary vs partial coverage explicitly. Schema note: a future revision may rename this to `source_bcs: [...]` for VPs covering N>1 BCs; until then, primary-singleton + `bcs:` list is the canonical pattern (see VP-058 for an example).
module: [implementation module name]
proof_method: kani|proptest|fuzz|manual|tla+
feasibility: feasible|needs-research|infeasible
verification_lock: false
proof_completed_date: null
proof_file_hash: null
# Lifecycle fields (DF-030)
lifecycle_status: active        # active | deprecated | retired | removed | withdrawn
introduced: vX.Y.Z             # cycle that created this VP
modified: []                    # cycles that modified it
deprecated: null                # cycle that deprecated (null if active)
deprecated_by: null             # which cycle deprecated it
replacement: null               # replacement VP ID (e.g., VP-015)
retired: null                   # cycle that retired (null if active)
withdrawn: null                 # cycle that withdrew this VP (null if not withdrawn)
withdrawal_reason: null         # why withdrawn (e.g., "Property no longer applies after parser rewrite")
removed: null                   # cycle that removed (null if not removed)
removal_reason: null             # why removed
---

# VP-NNN: [Property Name]

> **One-per-file:** Each verification property lives in its own file.
> Filename convention: `vp-NNN-[short-description].md`

## Property Statement

[Formal statement of what must be proven. Must be unambiguous and testable.]

## Source Contract

- **BC:** BC-S.SS.NNN — [contract title]
- **Postcondition/Invariant:** [which specific postcondition or invariant this verifies]

## Proof Method

| Method | Tool | Bounded? | Coverage |
|--------|------|----------|----------|
| [kani/proptest/fuzz] | [tool version] | [yes — bound size / no] | [what input space is covered] |

## Proof Harness Skeleton

```rust
#[kani::proof]
fn verify_VP_NNN() {
    // Precondition from BC-S.SS.NNN
    let input = kani::any();
    kani::assume(/* precondition */);

    // Execute
    let result = function_under_test(input);

    // Assert postcondition
    assert!(/* postcondition from BC */);

    // Check invariant
    assert!(/* invariant from BC */);
}
```

## Feasibility Assessment

| Factor | Assessment | Notes |
|--------|-----------|-------|
| Input space size | | |
| Proof complexity | | |
| Tool support | | |
| Estimated proof time | | |

## Lifecycle

| Event | Date | Actor |
|-------|------|-------|
| Created | | architect |
| Proof harness committed | | formal-verifier |
| Proof first passed | | formal-verifier |
| Locked (VERIFIED) | | formal-verifier |
