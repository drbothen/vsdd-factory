---
document_type: prd-supplement-test-vectors
level: L3
version: "1.0"
status: draft
producer: product-owner
timestamp: YYYY-MM-DDTHH:MM:SS
phase: 1a
inputs: [prd.md, behavioral-contracts/]
input-hash: "[md5]"
traces_to: prd.md
---

# Canonical Test Vectors: [Product Name]

> PRD supplement — extracted from PRD Section 7.
> Referenced by: test-writer, implementer, holdout-evaluator.

## Per-Subsystem Test Vectors

### Subsystem: [Name] (CAP-NNN)

#### BC-S.SS.NNN: [Contract Title]

| Input | Expected Output | Category | Notes |
|-------|----------------|----------|-------|
| [example input] | [expected result] | happy-path | |
| [edge case input] | [expected result] | edge-case | |
| [invalid input] | [expected error] | error | |

## Cross-Subsystem Integration Vectors

| Scenario | Input | Step 1 Output | Step 2 Input | Final Output |
|----------|-------|--------------|-------------|-------------|
| [integration flow] | | | | |

## Golden File References

| Vector Set | File | Format | BC Coverage |
|-----------|------|--------|------------|
| [name] | test-data/[file] | JSON/TOML/raw | BC-S.SS.NNN |
