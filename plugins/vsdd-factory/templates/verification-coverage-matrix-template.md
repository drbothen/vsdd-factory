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

| Module | Criticality | [Method 1] | [Method 2] | [Method N] | Coverage Target | VPs |
|--------|------------|------------|------------|------------|----------------|-----|
| [module-name] | CRITICAL/HIGH/MEDIUM/LOW | [count] | [count] | [count] | [percentage] | VP-NNN, VP-NNN (method annotation) |

**Method columns are project-specific.** Use the verification methods from your tooling selection. Examples by language:

| Language | Typical columns |
|----------|----------------|
| Rust | Kani Proofs, Proptest, Fuzz Targets |
| TypeScript | fast-check, Stryker |
| Python | Hypothesis, mutmut |
| Multi-language | Model Checking, Property Tests, Fuzz Tests |

The `validate-vp-consistency.sh` hook auto-detects method columns from the header — no configuration needed. It identifies columns between "Module"/"Criticality" and "Coverage Target"/"VPs" as method columns and validates their sums against VP-INDEX Summary.

**Column definitions:**
- **Module:** Module name from `module-decomposition.md`
- **Criticality:** From `module-criticality.md` (CRITICAL ≥95%, HIGH ≥90%, MEDIUM ≥80%, LOW ≥70%)
- **[Method N]:** Count of VPs verified via this method in this module. One column per verification method used in the project.
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
| [Method 1 name] | [count] | [P0 count] | [P1 count] |
| [Method 2 name] | [count] | [P0 count] | [P1 count] |
| [Method N name] | [count] | [P0 count] | [P1 count] |
| **Total VPs** | **[total]** | **[P0 total]** | **[P1 total]** |

**Method names must match VP-INDEX Summary labels** (normalized to snake_case by the hook). For example, a "Coverage by Module" column named "Kani Proofs" matches a Summary row named "Kani proofs" (both normalize to `kani_proofs`).

**Arithmetic invariant:** Total must equal sum of method rows. P0 + P1 must equal Planned Count per row. All counts must match VP-INDEX Summary.

## Coverage Gaps

| Gap | Reason | Mitigation |
|-----|--------|-----------|
| [module]: no formal verification | [why — e.g., effectful I/O] | [testing strategy — e.g., integration tests with mock server] |

## Domain Invariant Verification Map

| Invariant | Verified By | Priority |
|-----------|------------|----------|
| DI-NNN ([description]) | VP-NNN | P0/P1 |
