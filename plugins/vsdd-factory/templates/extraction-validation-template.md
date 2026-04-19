---
document_type: extraction-validation
level: ops
version: "1.0"
status: draft
producer: validate-extraction
timestamp: YYYY-MM-DDTHH:MM:SS
phase: 0
inputs: []
traces_to: ""
---

# Extraction Validation Report: [project]

## Phase 1 — Behavioral Verification

| Pass | Items Checked | Verified | Inaccurate | Hallucinated | Unverifiable |
|------|--------------|----------|------------|-------------|-------------|
| 1: Architecture | [N] | [N] | [N] | [N] | [N] |
| 2: Domain Model | [N] | [N] | [N] | [N] | [N] |
| 3: Behavioral Contracts | [N] | [N] | [N] | [N] | [N] |
| 4: NFRs | [N] | [N] | [N] | [N] | [N] |

## Phase 2 — Metric Verification

| Claim | Claimed | Recounted | Delta | Command |
|-------|---------|-----------|-------|---------|
| [claim description] | [N] | [N] | [±N] | `[shell command used to recount]` |

Every numeric claim in the analysis must appear in this table.
A row with `Delta: 0` is a pass; any non-zero delta is an error regardless of magnitude.

## Refinement Iterations: [N]/3

## Inaccurate Items (Corrected)

| Item | Original Claim | Actual Behavior | Correction Applied |
|------|---------------|-----------------|-------------------|

## Hallucinated Items (Removed)

| Item | Claim | Why Hallucinated |
|------|-------|-----------------|

## Unverifiable Items

| Item | Reason |
|------|--------|

## Confidence Assessment

- Overall extraction accuracy: [N]%
- Recommendation: TRUST | TRUST WITH CAVEATS | RE-ANALYZE
