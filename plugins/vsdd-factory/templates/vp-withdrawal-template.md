---
document_type: vp-withdrawal
level: L4
version: "1.0"
status: pending|approved|rejected
producer: formal-verifier
timestamp: YYYY-MM-DDTHH:MM:SS
phase: 5|6
inputs: [VP-NNN.md]
input-hash: "[md5]"
traces_to: VP-NNN.md
---

# VP Withdrawal: VP-NNN

## Withdrawal Details

| Field | Value |
|-------|-------|
| VP ID | VP-NNN |
| Property | [original property statement] |
| Status Before | VERIFIED |
| Withdrawal Date | YYYY-MM-DD |
| Initiated By | formal-verifier |
| Proof Completed Date | [original proof date] |

## Reason for Withdrawal

| Category | Description |
|----------|-------------|
| Category | proof-incorrect / requirement-changed / implementation-changed / spec-contradiction |
| Detail | [specific explanation of what's wrong] |
| Discovery Method | [how was the issue found — adversary review, regression, manual audit?] |

## Impact Assessment

### Downstream Artifacts Affected

| Artifact | Type | Impact | Action Required |
|----------|------|--------|----------------|
| [test file/function] | Test | Test based on invalid VP | Remove or rewrite test |
| [proof harness] | Proof | Proof proves wrong property | Delete harness |
| [story AC-NNN] | Story | AC references withdrawn VP | Update AC reference |
| [BC-S.SS.NNN] | Contract | BC postcondition relied on VP | Review BC validity |

### Cascade Check

- [ ] All tests referencing VP-NNN identified and updated
- [ ] Proof harness removed from verification suite
- [ ] Story acceptance criteria updated
- [ ] Consistency validator notified of withdrawal
- [ ] Convergence report updated (L4 dimension)

## Replacement

| Field | Value |
|-------|-------|
| Replacement VP | VP-NNN-NEW (or "none — property no longer applies") |
| Replacement Rationale | [why the new VP is correct where the old one wasn't] |
| Replacement Status | draft / in-development / verified |

## Approval

| Role | Decision | Date |
|------|----------|------|
| Formal Verifier | [initiator — documents reason] | |
| Architect | [approve/reject withdrawal] | |
| Human (if escalated) | [approve/reject] | |
