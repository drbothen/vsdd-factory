---
document_type: traceability-matrix
level: ops
version: "1.0"
status: draft
producer: "[agent-id]"
timestamp: YYYY-MM-DDTHH:MM:SS
phase: 7
inputs: []
input-hash: "[md5]"
traces_to: ""
---

# Traceability Matrix: [Product Name]

## Forward Traceability (L1 → Proof)

| L1 Section | L2 CAP-NNN | L3 BC-S.SS.NNN | NFR-NNN | Story | AC-NNN | Test | Source File | VP-NNN | Proof Status |
|------------|-----------|----------------|---------|-------|--------|------|------------|--------|-------------|
| [section ref] | CAP-001 | BC-1.01.001 | NFR-001 | S-1.01 | AC-001 | test_BC_1_01_001_xxx | src/module.rs | VP-001 | [proven/pending/withdrawn] |

## Reverse Traceability (Proof → L1)

| VP-NNN | Proof Status | BC-S.SS.NNN | L2 CAP-NNN | L1 Section |
|--------|-------------|-------------|-----------|-------------|
| VP-001 | [proven/pending/withdrawn] | BC-1.01.001 | CAP-001 | [section ref] |

## Coverage Summary

### L1 → L2

| L1 Section | CAP Count | Gaps |
|------------|-----------|------|
| [section] | [count] | [gap description or "none"] |

### L2 → L3

| CAP-NNN | BC Count | Gaps |
|---------|----------|------|
| CAP-001 | [count] | [gap description or "none"] |

### L3 → Stories

| Subsystem | Total BCs | BCs with Stories | Gaps |
|-----------|----------|-----------------|------|
| [subsystem] | [n] | [n] | [gap description or "none"] |

### L3 → L4

| Subsystem | Total BCs | BCs with VPs | Justification for No VP |
|-----------|----------|-------------|------------------------|
| [subsystem] | [n] | [n] | [justification or "all covered"] |

### VP Status

| Status | Count | IDs |
|--------|-------|-----|
| Verified | [n] | [VP-001, VP-002, ...] |
| In Development | [n] | [VP-003, ...] |
| Withdrawn | [n] | [VP-NNN, ...] |

## VP Status Summary

| Status | Count | Percentage |
|--------|-------|------------|
| Proven | [n] | [pct]% |
| Pending | [n] | [pct]% |
| Failed | [n] | [pct]% |
| Withdrawn | [n] | [pct]% |
| **Total** | **[n]** | **100%** |

## Gap Register

| Gap ID | Level | Missing Link | Impact | Priority | Resolution Plan |
|--------|-------|-------------|--------|----------|----------------|
| GAP-001 | [L1-L2/L2-L3/L3-L4] | [what is missing] | [impact description] | [P0/P1/P2] | [plan to close] |

## Withdrawn VP Impact

| VP-NNN | Original BC | Replacement VP | Impact on Coverage | Notes |
|--------|-----------|---------------|-------------------|-------|
| VP-NNN | BC-S.SS.NNN | [VP-NNN or none] | [coverage delta] | [explanation] |
