---
document_type: dtu-fidelity-report
service_name: "[Service Name]"
fidelity_level: L1|L2|L3|L4
fidelity_score: 0.NN
status: pass|fail
producer: dtu-validator
timestamp: YYYY-MM-DDTHH:MM:SS
phase: 3
---

# DTU Fidelity Report: [Service Name] Clone

## Summary

| Metric | Value | Threshold | Status |
|--------|-------|-----------|--------|
| Fidelity score | NN% | >=NN% | PASS/FAIL |
| Endpoints tested | N/N | 100% | |
| State transitions | N/N correct | 100% (L2+) | |
| Error responses | N/N match | >=NN% (L3+) | |
| Failure injection | working/broken | working (L4) | |

## Endpoint Comparison

| Endpoint | Method | Real Status | Clone Status | Real Body Match? | Delta |
|----------|--------|------------|-------------|-----------------|-------|
| /api/v1/users | GET | 200 | 200 | YES | -- |
| /api/v1/users | POST (invalid) | 400 | 400 | PARTIAL | error.detail format |

## State Transition Tests (L2+)

| Sequence | Expected | Actual | Pass? |
|----------|----------|--------|-------|
| POST -> GET | Created resource returned | Created resource returned | YES |
| DELETE -> GET | 404 returned | 404 returned | YES |

## Error Response Tests (L3+)

| Condition | Real Response | Clone Response | Match? |
|-----------|--------------|----------------|--------|
| Invalid auth | 401 {...} | 401 {...} | YES |
| Rate limit | 429 + retry-after | 429 + retry-after | YES |

## Failure Injection Tests (L4)

| Mode | Configured | SUT Behavior | Pass? |
|------|-----------|-------------|-------|
| 5% error rate | YES | Retries + succeeds | YES |
| 2s latency | YES | Timeout + fallback | YES |

## Deltas Requiring Attention

| # | Endpoint | Issue | Severity | Fix Required? |
|---|----------|-------|----------|--------------|
| 1 | POST /users (invalid) | error.detail format differs | LOW | Optional for L2, required for L3+ |
