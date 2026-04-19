---
document_type: formal-verification-report
level: ops
version: "1.0"
status: draft
producer: formal-verifier
timestamp: YYYY-MM-DDTHH:MM:SS
phase: 5
inputs: [verification-properties/, architecture/verification-architecture.md]
traces_to: VP-INDEX.md
---

# Formal Verification Report

## Summary

| Technique | Status | Results |
|-----------|--------|---------|
| Kani proofs | PASS/FAIL | [N] harnesses, all pass / [N] failures |
| Fuzz testing | PASS/FAIL | [N] targets, [duration], [crashes] |
| Mutation testing | PASS/FAIL | Kill rate: [N]%, [N] survivors |
| Security scan | PASS/FAIL | [N] findings ([N] critical) |

## Kani Results

[Per-harness results]

## Fuzz Results

[Per-target results, any crashes]

## Mutation Survivors

[List of surviving mutants with analysis]

## Security Findings

[Semgrep findings by severity]

## Gate: PASS | FAIL

Criteria: all Kani pass, no fuzz crashes, mutation kill ≥90%, no critical security findings.
