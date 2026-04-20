---
name: phase-1d-adversarial-spec-review-shared-context
description: Shared context for Phase 1d adversarial spec review steps. Contains Iron Law, Red Flags, convergence protocol, information asymmetry rules, templates, and persistence protocol.
---

# Phase 1d: Adversarial Spec Review — Shared Context

This file is loaded by every step in the phase-1d-adversarial-spec-review skill.

## The Iron Law

> **NO APPROVAL WITHOUT FRESH-CONTEXT REVIEW FIRST**

Violating the letter of the rule is violating the spirit of the rule. Fresh context means the adversary has not seen prior review passes, the author's explanations, or the orchestrator's summary. Loading any of those contaminates the asymmetry the pattern depends on.

## Red Flags

| Thought | Reality |
|---|---|
| "I already reviewed this, I can skip the adversary pass" | Self-review is not adversarial review. Dispatch. |
| "The spec is obviously correct, one pass is enough" | Minimum is 3 clean passes. The rule exists because round 1 systematically misses things. |
| "Let me summarize the prior pass for the adversary to save tokens" | That destroys fresh context. Dispatch with only the target artifacts. |
| "The adversary found nothing, let's call it done" | Zero findings after a short prompt is a prompt bug, not convergence. Re-dispatch with sharper scope. |
| "This finding isn't really critical, I'll downgrade it" | Severity is the adversary's call, not the orchestrator's. Record as-is. |
| "The same finding keeps appearing, the adversary is stuck" | It keeps appearing because it isn't fixed. Fix it, then re-run. |
| "Novelty is LOW after one pass, we've converged" | Minimum 3 clean passes. No exceptions. |
| "Let me tell the adversary what the prior reviewer found" | Information asymmetry is the mechanism. Do not leak prior findings. |

## Convergence Protocol

All adversarial review touchpoints use the same convergence protocol. The scope changes; the protocol does not.

### Quantitative Criteria

- **Novelty score** < 0.15 for 2+ consecutive passes (85%+ of findings are duplicates/variants)
- **Severity distribution**: median < 2.0 on 1-5 scale, strictly decreasing for 3+ passes
- **Finding similarity** > 0.75 (semantic embedding) to prior corpus
- **Reviewer confidence** < 0.55 for 2+ passes signals hallucination regime
- **Verification rate** < 60% independently confirmed signals convergence
- **Convergence Index** CI < 0.3 and declining for 3+ iterations

### Behavioral Rules

- **Minimum 3 consecutive clean passes** — no early termination, even if novelty is LOW after 2
- **Trajectory monotonicity** — finding counts must decrease monotonically across passes. If any pass shows MORE findings than the previous, stop and investigate regression.
- **Zero-findings halt** — zero findings on a first pass is suspicious, not convergence. Re-dispatch with explicit justification requirement.
- **Maximum 10 passes** before escalating to human

### Convergence Verdict

The adversary reports one of:
- `CONVERGENCE_REACHED` — findings are cosmetic only, novelty exhausted
- `FINDINGS_REMAIN` — substantive findings remain, iterate

## Information Asymmetry Rules

The adversary agent has **read-only tools** (`Read`, `Grep`, `Glob`). It cannot write, edit, or execute commands.

**The adversary CANNOT access:**
- Prior adversarial review passes (fresh context per pass)
- Implementation commit history or PR discussions
- Other agents' working notes
- Holdout scenarios (train/test separation)

**The adversary CAN access:**
- All spec documents in `.factory/specs/`
- Architecture documents
- Behavioral contracts and verification properties

**Why read-only:** Information asymmetry is the mechanism that makes adversarial review effective. If the adversary could write files, it could see its own prior reviews (breaking fresh-context) or modify specs (crossing the builder/reviewer boundary).

## Post-Adversary Persistence Protocol

The adversary agent cannot write files. After the adversary returns findings as chat text, the orchestrator MUST persist them:

1. **Capture** the adversary's full output verbatim (do not summarize or filter)
2. **Determine target path:** `.factory/specs/adversarial-reviews/pass-<N>.md`
3. **Dispatch state-manager** to write the findings file
4. **Dispatch state-manager** to update the adversarial review index (`ADV-P<N>-INDEX.md`)

If the orchestrator skips this step, findings are lost when the conversation context resets.

## Policy Rubric Auto-Loading

Before dispatching the adversary, the orchestrator MUST:

1. Read `.factory/policies.yaml` (skip if absent)
2. Format each policy as a rubric block: `POLICY <id> (<name>): <description>` with severity, scope, and verification steps
3. Append all rubric blocks under `## Project Policy Rubric` in the adversary's task prompt
4. The adversary executes verification steps for each policy and reports compliance

## Templates

- `${CLAUDE_PLUGIN_ROOT}/templates/adversarial-review-template.md` — review document structure
- `${CLAUDE_PLUGIN_ROOT}/templates/adversarial-finding-template.md` — individual finding format
- `${CLAUDE_PLUGIN_ROOT}/templates/adversarial-review-index-template.md` — review index
- `${CLAUDE_PLUGIN_ROOT}/templates/adversary-prompt-templates/phase-1d-spec-review.md` — adversary prompt

## Prerequisites

- Phase 1 spec artifacts complete: PRD, architecture directory, UX spec (if applicable)
- All spec artifacts committed to factory-artifacts branch
- Adversary model configured (different model family from builder)
- `.factory/specs/adversarial-reviews/` directory exists or will be created

## Output Location

All adversarial review artifacts are written to `.factory/specs/adversarial-reviews/`.

## Finding ID Format

`ADV-<CYCLE>-P<PASS>-<SEV>-<SEQ>`

- `ADV`: fixed prefix
- `<CYCLE>`: cycle prefix from `.factory/current-cycle` (e.g., `P1CONV`)
- `<PASS>`: two-digit pass number (e.g., `P01`)
- `<SEV>`: severity abbreviation (`CRIT`, `HIGH`, `MED`, `LOW`)
- `<SEQ>`: three-digit sequence (e.g., `001`)

## Filename Collision Guard

Before writing any review file, check for filename collisions:

1. Compute target path: `.factory/specs/adversarial-reviews/pass-<N>.md`
2. If the target exists with different content: **REFUSE the write**. Emit error and point to `/vsdd-factory:factory-cycles-bootstrap`.
3. If legacy flat files exist in `.factory/specs/adversarial-review-pass-*.md`: warn about migration.
