---
name: step-f-convergence-loop
description: Re-spawn adversary with fresh context after fixes. Iterate until full convergence — novelty < 0.15, min 3 clean passes.
---

# Step F: Convergence Loop

> **Shared context:** Read `./_shared-context.md` before executing this step.

After fixes (Step E), spawn the adversary AGAIN with fresh context. Iterate the full cycle (Steps B → C → D → E → F) until convergence.

## Procedure

### 1. Re-Review Scope

If CRITICAL or HIGH findings were fixed:
- Spawn adversary again with fresh context
- Include only the FIXED files (not the full delta again)
- This narrows the adversary's focus to verify fixes and find fix-introduced issues

### 2. Convergence Assessment

After each pass, assess against full convergence criteria:

- **Novelty score** < 0.15 for 2+ consecutive passes?
- **Severity distribution** — all findings MEDIUM or below?
- **Minimum 3 consecutive clean passes** met?
- **Trajectory monotonicity** — finding count decreased from prior pass?

### 3. Decision Logic

```
IF findings contain CRITICAL or HIGH:
  → Route through Step E (triage and fix)
  → Return to Step B (spawn adversary again)

IF findings are all MEDIUM or below AND novelty < 0.15:
  → Check minimum 3 clean passes met
  → IF yes: proceed to Step G (secondary pass) or Step H (report)
  → IF no: continue iterating

IF pass count reaches 10 without convergence:
  → ESCALATE to human
```

### 4. Write Per-Round Review

Each re-review round produces: `.factory/phase-f5-adversarial/round-N-review.md`

## Artifacts

- `.factory/phase-f5-adversarial/round-N-review.md` (one per re-review round)

## Success Criteria

- Minimum 3 consecutive clean passes achieved
- Novelty score < 0.15 for 2+ consecutive passes
- No CRITICAL or HIGH findings remain
- Fresh context used for each re-review round
- Trajectory monotonicity maintained
