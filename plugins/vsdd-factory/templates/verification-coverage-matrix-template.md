---
document_type: architecture-section
level: L3
section: "verification-coverage-matrix"
version: "1.0"
status: draft
producer: architect
timestamp: YYYY-MM-DDTHH:MM:SS
phase: 1b
inputs: [verification-architecture.md, module-decomposition.md, module-criticality.md]
traces_to: ARCH-INDEX.md
---

# Verification Coverage Matrix

> Maps every VP to its target module and tracks per-method verification coverage.
> VP-INDEX.md is the authoritative VP catalog — this matrix must match it exactly
> (Policy 9: `vp_index_is_vp_catalog_source_of_truth`).

## Coverage by Module

| Module | Criticality | Kani Proofs | Proptest | Fuzz Targets | Coverage Target | VPs |
|--------|------------|-------------|----------|-------------|----------------|-----|
| [module-name] | CRITICAL/HIGH/MEDIUM/LOW | [count] | [count] | [count] | [percentage] | VP-NNN, VP-NNN (method annotation) |

**Column definitions:**
- **Module:** Module name from `module-decomposition.md`
- **Criticality:** From `module-criticality.md` (CRITICAL ≥95%, HIGH ≥90%, MEDIUM ≥80%, LOW ≥70%)
- **Kani Proofs:** Count of VPs verified via Kani model checking in this module
- **Proptest:** Count of VPs verified via property-based testing in this module
- **Fuzz Targets:** Count of VPs verified via fuzz testing in this module
- **Coverage Target:** Kill-rate target from criticality classification
- **VPs:** VP-NNN IDs assigned to this module, with method annotation where multiple methods used (e.g., "VP-024 (proptest); VP-038 (fuzz)")

**Rules:**
- Every module from `module-decomposition.md` must have a row, even if all counts are 0
- Per-method column sums must equal VP-INDEX Summary per-method totals
- Every VP listed must exist in VP-INDEX with matching module assignment
- Modules with 0 formal VPs should note their coverage strategy (e.g., "(integration tests only)")

## Summary by Method

| Method | Planned Count | P0 | P1 |
|--------|--------------|----|----|
| Kani proofs | [count] | [P0 count] | [P1 count] |
| Proptest properties | [count] | [P0 count] | [P1 count] |
| Fuzz targets | [count] | [P0 count] | [P1 count] |
| Integration test VPs | [count] | [P0 count] | [P1 count] |
| **Total VPs** | **[total]** | **[P0 total]** | **[P1 total]** |

**Arithmetic invariant:** Total must equal sum of method rows. P0 + P1 must equal Planned Count per row. All counts must match VP-INDEX Summary.

## Coverage Gaps

| Gap | Reason | Mitigation |
|-----|--------|-----------|
| [module]: no formal verification | [why — e.g., effectful I/O] | [testing strategy — e.g., integration tests with mock server] |

## Domain Invariant Verification Map

| Invariant | Verified By | Priority |
|-----------|------------|----------|
| DI-NNN ([description]) | VP-NNN | P0/P1 |
