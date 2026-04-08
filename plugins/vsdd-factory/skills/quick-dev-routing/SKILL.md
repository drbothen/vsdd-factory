---
name: quick-dev-routing
description: >
  Routes trivially-scoped changes through a compressed Feature Mode pipeline.
  Skips phases that add no value for zero-blast-radius changes. Preserves
  regression suite and adversarial review even for trivial changes.
---

# Quick Dev Routing

## When This Skill Runs

The Orchestrator invokes Quick Dev Routing when Phase F1 (Delta Analysis)
determines that a change is trivially scoped. This is an optimization that
avoids the full F1-F7 pipeline for changes that cannot break anything.

## Qualification Criteria

A change qualifies for Quick Dev Routing when ALL of the following are true:

| Criterion | Threshold |
|-----------|-----------|
| Files changed | <= 3 source files |
| New files | <= 2 |
| Modified files | <= 1 (internal logic only, no interface changes) |
| Dependent files | 0 (no other files import/depend on the changed files) |
| Architecture impact | NONE (no new components, no changed interfaces) |
| Security impact | NONE (no auth, no crypto, no trust boundary changes) |
| Blast radius | ZERO (no other module can observe the change) |

If ANY criterion is not met, the change goes through the full F1-F7 pipeline.

## Multi-Goal Detection

Before routing, check if the feature request contains multiple independent
goals. Signals:

- "Add X AND also Y" where X and Y affect different modules
- "Fix the bug in A and add the feature to B"
- Multiple success criteria targeting different components

**If multi-goal detected:**
1. Present detected goals to the human:
   "This request contains [N] independent goals:
   - Goal 1: [description]
   - Goal 2: [description]
   Would you like to:
   [S] Split into [N] separate runs (recommended — cleaner scope)
   [K] Keep as one run (scope will be larger, may not qualify for Quick Dev)"
2. If split: create separate feature requests, route each independently
3. If keep: proceed with full F1-F7 (multi-goal rarely qualifies for Quick Dev)

## Compressed Pipeline

For qualifying changes, run this compressed pipeline:

```
F1 (Quick): Scope verification (automated, no human gate)
  → F4 (Full): TDD implementation with full regression suite
  → F5 (Lite): Single adversary pass (no re-review unless CRITICAL finding)
  → F7 (Quick): Automated convergence check + human merge authorization
```

**Skipped phases:**
- F2 (Spec Evolution) — no spec changes for trivial modifications
- F3 (Story Creation) — tracked as a patch, not a story
- F6 (Hardening) — no formal verification for zero-blast-radius changes

**Always preserved (never skipped, even for trivial changes):**
- Full regression test suite (runs in F4)
- At least one adversarial review pass (F5)
- Human merge authorization (F7)

## One-Shot Path

For the absolute simplest changes (single file, internal logic, tests exist):

1. Implementer writes the fix/change
2. Run full regression suite
3. Single adversary review pass
4. If adversary finds nothing CRITICAL → auto-advance to human merge auth
5. Human approves merge

Total: 4 steps instead of the full 7-phase pipeline.

## Output

Write routing decision to `.factory/phase-f1-delta-analysis/routing-decision.md`:
- Qualification assessment (which criteria met/failed)
- Multi-goal detection result
- Routing: QUICK_DEV / ONE_SHOT / FULL_PIPELINE
- Compressed pipeline plan (if Quick Dev or One-Shot)

## Failure Modes

- If delta analysis classifies the change as non-trivial (any qualification criterion fails): route back to standard Feature Mode (F1-F7) and document which criterion failed
- If regression tests fail after Quick Dev implementation: escalate to full F4 with regression-log.md and do not proceed through compressed pipeline
- If multi-goal detection is ambiguous: present the detected goals to human for split/keep decision before routing
