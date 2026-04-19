---
document_type: fix
level: ops
version: "1.0"
status: open
producer: pr-manager
timestamp: YYYY-MM-DDTHH:MM:SS
phase: "4|5|6"
fix_id: "FIX-P[N]-NNN"
source_finding: "ADV-<CYCLE>-P[N]-[SEV]-NNN"
source_phase: "4|5|6"
severity: "CRIT|HIGH|MED|LOW"
traces_to: ""
---

# FIX-P[N]-NNN: [Fix Title]

## Source Finding

| Field | Value |
|-------|-------|
| Finding ID | [ADV/CR/SEC finding ID] |
| Severity | [CRIT/HIGH/MED/LOW] |
| Phase | [4: adversarial / 5: hardening / 6: convergence] |
| Description | [what the finding identified] |

## Fix Description

[What was changed and why. Reference specific BCs, stories, or VPs affected.]

## Files Changed

| File | Change Type | Description |
|------|-------------|-------------|
| [path] | modified / created / deleted | [what changed] |

## Verification

- [ ] Root cause addressed (not just symptom)
- [ ] All tests pass after fix
- [ ] No regressions introduced
- [ ] Fix verified in next adversarial/review pass

## ID Format

`FIX-P[N]-NNN` where:
- `P[N]` = source phase (P4 = adversarial, P5 = hardening, P6 = convergence)
- `NNN` = sequential within the cycle (lifecycle-scoped, append-only)
