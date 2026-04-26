---
document_type: traceability-matrices
level: ops
version: "1.0"
status: draft
producer: story-writer
timestamp: YYYY-MM-DDTHH:MM:SS
phase: 2
inputs: [behavioral-contracts/, verification-properties/, prd-supplements/nfr-catalog.md]
traces_to: STORY-INDEX.md
---

# Traceability Matrices

> Produced by story-writer as part of `dependency-graph.md`. Enables
> AC completeness verification by the consistency-validator.

## BC to Stories Matrix

| BC-S.SS.NNN | Stories | Full Coverage? |
|-------------|---------|---------------|
| BC-2.01.001 | S-1.03, S-1.05 | Yes |

## VP to Stories Matrix

| VP-NNN | Stories Exercising It | BC Source |
|--------|----------------------|-----------|
| VP-001 | S-1.03 | BC-2.01.001 |

## NFR to Stories Matrix

| NFR-NNN | Stories Implementing It | Validation Method |
|---------|------------------------|-------------------|
| NFR-001 | S-2.07 | Benchmark: 1000 files in ≤2s |

## BC Clause Coverage Matrix

| BC-S.SS.NNN | Clause | Type | Covering AC | Story |
|-------------|--------|------|-------------|-------|
| BC-2.01.001 | 1 | precondition | AC-003 | S-1.05 |
| BC-2.01.001 | 2 | postcondition | AC-001 | S-1.03 |
| BC-2.01.001 | 3 | postcondition | -- | GAP-001 (justified) |

## Edge Case Coverage Matrix

| Source | EC/Error ID | Description | Story | AC/EC Reference |
|--------|-------------|-------------|-------|----------------|
| BC-2.01.001 | EC-001 | Malformed input | S-1.05 | EC-003 |
| error-taxonomy | E-val-001 | Validation failure | S-1.03 | AC-007 |

## Gap Register

| Gap ID | Level | Source | Clause/Item | Justification | Resolution Target |
|--------|-------|--------|-------------|---------------|-------------------|
| GAP-001 | L1 | BC-2.01.001 postcondition 3 | [min 10 chars justification] | v2.0.0 |

**Level:** L1 (BC clause) / L2 (edge case or error) / L3 (NFR, holdout, UI state)

**Rules:**
- Every BC clause must be covered by at least one AC or have a Gap Register entry with justification (min 10 chars)
- Every BC edge case (EC-NNN) must appear in at least one story's ACs or Edge Cases table
- Every E-xxx-NNN from error-taxonomy.md must be covered by at least one story AC or edge case
