---
name: phase-f5-scoped-adversarial-shared-context
description: Shared context for Phase F5 scoped adversarial review steps. Contains Iron Law, Red Flags, convergence protocol, information asymmetry walls, fix delivery flow, and cross-cutting protocols.
---

# Phase F5: Scoped Adversarial Review — Shared Context

This file is loaded by every step in the phase-f5-scoped-adversarial skill.

## The Iron Law

> **NO APPROVAL WITHOUT FRESH-CONTEXT REVIEW FIRST**

Violating the letter of the rule is violating the spirit of the rule. Fresh context means the adversary has not seen prior review passes, the author's explanations, or the orchestrator's summary. Loading any of those contaminates the asymmetry the pattern depends on.

## Red Flags

| Thought | Reality |
|---|---|
| "I already reviewed this, I can skip the adversary pass" | Self-review is not adversarial review. Dispatch. |
| "The delta is small, one pass is enough" | Minimum is 3 clean passes. Small deltas can hide deep issues. |
| "Let me include the full codebase for context" | Scoped review means delta files only. Full codebase is Phase 5. |
| "Let me summarize the prior pass for the adversary to save tokens" | That destroys fresh context. Dispatch with only the review package. |
| "The adversary found nothing, let's call it done" | Zero findings on first pass is a prompt bug. Re-dispatch with sharper scope. |
| "This finding isn't really critical, I'll downgrade it" | Severity is the adversary's call. Record as-is. |
| "Let me include the implementation notes so the adversary understands the intent" | The adversary judges the code, not the intent. Intent leaks break asymmetry. |
| "Unchanged files don't need to be excluded, they're just context" | Only include direct dependents of changed code. Unchanged files dilute focus. |

## Convergence Protocol

Same protocol as all adversarial touchpoints. See `phase-1d-adversarial-spec-review/steps/_shared-context.md` for the full quantitative criteria. Key thresholds:

- **Novelty score** < 0.15 for 2+ consecutive passes
- **Minimum 3 consecutive clean passes**
- **Trajectory monotonicity** — finding counts decrease monotonically
- **Zero-findings halt** — suspicious on first pass, re-dispatch with justification requirement
- **Maximum 10 passes** before escalating to human

## Information Asymmetry Walls (DF-025)

The adversary receives ONLY the scoped review package. Explicitly excluded:

- Unchanged source files (unless direct dependents of changed code)
- Previous adversarial review reports (fresh perspective required)
- Implementation notes or rationale (adversary judges code, not intent)
- Phase F4 TDD logs and implementer session notes
- `.factory/semport/**` (gene transfusion history, DF-028)
- `.factory/cycles/**/implementation/red-gate-log*`
- `.factory/cycles/**/implementation/implementer-notes*`
- `.factory/cycles/**/adversarial-reviews/**` (no prior adversarial history)

## Fix PR Delivery (DF-025)

When F5 finds issues on merged develop, fixes go through the per-story delivery flow via `code-delivery.lobster`:

```
FIX-F5-NNN → worktree → fix → demo (if behavior-changing) → PR → AI review
  → security review (if applicable) → merge → re-verify only failing checks
```

## Security Review Touchpoint (DF-025)

If the adversary identifies security findings, `security-reviewer` performs a dedicated CWE/OWASP analysis (Security Review Touchpoint #3). The security-reviewer cannot see the adversary's implementation reasoning (information asymmetry wall).

## Multi-Repo: Contract Compliance Validation (DF-013)

For cross-repo deltas that modify API contracts:
- Run **contract testing** (Pact/Specmatic) to validate all consumer repos comply with the updated contract
- The adversary reviews both the contract change AND the consumer-side adaptations
- Any breaking contract change without corresponding consumer updates is a CRITICAL finding

## Holdout Regression (Conditional)

If F5 fixes are behavior-changing, re-evaluate affected holdout scenarios to verify no regressions were introduced by the fixes.

## Templates

- `${CLAUDE_PLUGIN_ROOT}/templates/adversarial-review-template.md` — review document structure
- `${CLAUDE_PLUGIN_ROOT}/templates/adversarial-finding-template.md` — individual finding format
- `${CLAUDE_PLUGIN_ROOT}/templates/adversary-prompt-templates/phase-5-code-review.md` — adversary prompt

## Prerequisites

- Phase F4 Delta Implementation complete (all tests green, regression clean)
- `.factory/phase-f4-implementation/summary.md` exists
- `.factory/phase-f1-delta-analysis/affected-files.txt` exists

## Output Location

All Phase F5 artifacts are written to `.factory/phase-f5-adversarial/`.
