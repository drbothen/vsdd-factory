---
name: step-e-convergence-loop
description: Re-spawn adversary with fresh context after fixes, iterate until convergence reached. Minimum 3 clean passes.
---

# Step E: Convergence Loop

> **Shared context:** Read `./_shared-context.md` before executing this step.

After fixes (Step D), spawn the adversary AGAIN with fresh context. Repeat the full cycle (Steps B → C → D → E) until convergence is reached.

## Procedure

### 1. Fresh Context Re-Review

Spawn adversary with:
- FRESH context — no prior conversation history, no knowledge of prior passes
- The SAME scope as the original review (full spec corpus)
- Updated spec artifacts (with fixes applied)
- The accumulated invariant list (so the adversary can check confirmed invariants efficiently)

### 2. Convergence Assessment

After each pass, assess convergence against ALL criteria from the shared context:

- **Novelty score** < 0.15 for 2+ consecutive passes?
- **Severity distribution** median < 2.0, strictly decreasing for 3+ passes?
- **Finding similarity** > 0.75 to prior corpus?
- **Trajectory monotonicity** — finding count decreased from prior pass?
- **Minimum 3 consecutive clean passes** met?

### 3. Decision Logic

```
IF findings contain CRITICAL or HIGH:
  → Route through Step D (triage and fix)
  → Return to Step B (spawn adversary again)

IF findings are all MEDIUM or below AND novelty < 0.15:
  → Check minimum 3 clean passes met
  → IF yes: report CONVERGENCE_REACHED
  → IF no: continue iterating

IF pass count reaches 10 without convergence:
  → ESCALATE to human with current state
```

### 4. No Early Termination

Do NOT shortcut to "it's clean" after 2 consecutive clean passes. Fresh-context review has compounding value — the adversary makes genuinely new findings through pass 9+ in complex projects, including findings every prior pass missed.

### 5. Trajectory Regression

If any pass shows MORE findings than the previous pass:
1. STOP — do not continue convergence passes
2. Investigate root cause:
   - New scope added without pre-validation?
   - A fix introduced a new defect?
   - Adversary's perimeter expanded unexpectedly?
3. Resolve the regression before resuming

### 6. Report Convergence

When convergence is reached, the adversary reports:

```
CONVERGENCE_REACHED — findings are cosmetic only.
Novelty: LOW — findings are refinements, not gaps. Spec has converged.
```

The orchestrator persists this final assessment and updates STATE.md.

## Artifacts

- Per-pass review files (accumulated in `.factory/specs/adversarial-reviews/`)
- Final convergence assessment
- STATE.md updated with convergence status

## Success Criteria

- Minimum 3 consecutive clean passes achieved
- Novelty score < 0.15 for 2+ consecutive passes
- All CRITICAL and HIGH findings from all passes resolved
- Trajectory monotonicity maintained (no regressions)
- Adversary reports `CONVERGENCE_REACHED`
