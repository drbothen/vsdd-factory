---
name: adversarial-review
description: Launch a fresh-context adversarial review of specs or implementation. Uses the adversary agent with information asymmetry to find gaps, contradictions, and missing edge cases. Minimum 2 passes to convergence.
argument-hint: "[specs|implementation]"
disable-model-invocation: true
context: fork
agent: adversary
---

# Adversarial Review

Launch the adversary agent to review specs or implementation with fresh context.

## The Iron Law

> **NO APPROVAL WITHOUT FRESH-CONTEXT REVIEW FIRST**

Violating the letter of the rule is violating the spirit of the rule. Fresh context means the adversary has not seen prior review passes, the author's explanations, or the orchestrator's summary. Loading any of those contaminates the asymmetry the pattern depends on.

## Announce at Start

Before any other action, say verbatim:

> I'm using the adversarial-review skill to launch a fresh-context adversary pass on <target>.

Then create TodoWrite entries: one per planned pass (minimum 2).

## Red Flags

| Thought | Reality |
|---|---|
| "I already reviewed this, I can skip the adversary pass" | Self-review is not adversarial review. Dispatch. |
| "The spec is obviously correct, one pass is enough" | Minimum is 2. The rule exists because round 1 systematically misses things. |
| "Let me summarize the prior pass for the adversary to save tokens" | That destroys fresh context. Dispatch with only the target artifact. |
| "The adversary found nothing, let's call it done" | Zero findings after a short prompt is a prompt bug, not convergence. Re-dispatch with sharper scope. |
| "This finding isn't really critical, I'll downgrade it" | Severity is the adversary's call, not the orchestrator's. Record as-is. |
| "The same finding keeps appearing, the adversary is stuck" | It keeps appearing because it isn't fixed. Fix it, then re-run. |
| "Novelty is LOW after one pass, we've converged" | Minimum 2 passes. No exceptions. |
| "Let me tell the adversary what the prior reviewer found" | Information asymmetry is the mechanism. Do not leak prior findings. |


## Templates

Read and follow the output format in:
- `${CLAUDE_PLUGIN_ROOT}/templates/adversarial-review-template.md` — review document structure
- `${CLAUDE_PLUGIN_ROOT}/templates/adversarial-review-index-template.md` — review index
- `${CLAUDE_PLUGIN_ROOT}/templates/adversarial-finding-template.md` — individual finding format

## Target

Parse `$ARGUMENTS` to determine review target:
- `specs` — review all documents in `.factory/specs/`
- `implementation` — review source code against specs
- If no argument, default to `specs`

## For Spec Review

Read all spec documents:
1. `.factory/specs/product-brief.md`
2. `.factory/specs/domain-spec/L2-INDEX.md` → read index, then all sections (if exists)
3. `.factory/specs/prd.md`
4. `.factory/specs/prd-supplements/*`
5. `.factory/specs/behavioral-contracts/*`
6. `.factory/specs/verification-properties/*`
7. `.factory/specs/architecture/*`

Attack with the adversary protocol. Write findings to `.factory/cycles/<current>/adversarial-reviews/`.

## For Implementation Review

Read specs first, then review source code against them. Focus on spec drift and silent failures.

## Post-Adversary Persistence (MANDATORY)

The adversary agent has only Read/Grep/Glob tools — it cannot write files. After the adversary returns findings as chat text, the orchestrator MUST persist them:

1. **Capture the adversary's full output** verbatim (do not summarize or filter)
2. **Determine the target path:** `.factory/cycles/<current-cycle>/adversarial-reviews/pass-<N>.md`
   - Read `.factory/current-cycle` for the cycle name (e.g., `v1.0.0-greenfield`)
   - If no current-cycle file exists, use the active cycle from STATE.md
   - `<N>` is the pass number (1-based, sequential within the cycle)
3. **Dispatch state-manager** to write the findings file at the target path
4. **Dispatch state-manager** to update the adversarial review index (`ADV-P<N>-INDEX.md`) in the same cycle directory

**Decision rationale:** Option (a) chosen over granting adversary scoped Write access because: (1) preserves fresh-context information asymmetry — adversary never sees its own prior files; (2) state-manager already owns all `.factory/` commits; (3) no harness support for path-scoped tool allowlists.

If the orchestrator skips this step, findings are lost when the conversation context resets. This was observed in practice when direct adversary spawns returned findings as chat text that disappeared on session boundary.

## Pass Management

- Each review is a numbered pass (ADV-P1, ADV-P2, etc.)
- After each pass, assess novelty decay
- When novelty is LOW (findings are refinements, not gaps), report convergence
- Minimum 2 passes, maximum 5 before escalating

## Lessons Learned (apply to ALL projects)

### Fix Root Causes, Not Symptoms

When a finding shows BC-to-story drift (wrong error codes, missing struct fields, wrong formulas), the fix MUST be: read the authoritative BC, then rewrite the contradicting story section from scratch. Never apply targeted text replacements without first reading both the BC and the story section. In practice, incremental line-level patches caused the same findings to recur across 3-5 passes (S-3.01 security limits survived 3 fix attempts; S-3.04 alias system required 6 passes before a full rewrite resolved it in one pass).

### Accumulate Invariants Across Passes

After each adversarial fix cycle, update the adversary prompt with ALL confirmed invariants (struct fields, error codes, version pins, dependency rules, persistence models). The invariant list grows monotonically. Each subsequent pass checks confirmed invariants efficiently and focuses on finding NEW issues.

## No Early Termination

Do NOT shortcut to "it's clean" after 2 consecutive clean passes. Fresh-context review has compounding value — the adversary makes genuinely new findings through pass 9+ in complex projects, including findings every prior pass missed (e.g., phantom crate references that only surface when the adversary reads dependency-graph.md with truly fresh eyes).

Minimum convergence requirement: 3 consecutive clean passes (not 2). Even near-convergence, keep running passes until the minimum is met.

## Trajectory Monotonicity

Finding counts must decrease monotonically across passes. If any pass shows MORE findings than the previous pass, this is a regression — stop and investigate root cause before proceeding. Possible causes:
- New scope was added without pre-validation against the invariant list
- A fix introduced a new defect
- The adversary's perimeter expanded unexpectedly

Do NOT continue convergence passes until the regression is explained and resolved.

### Pre-validate New Scope Additions

When new stories are added during adversarial convergence, they must be written by an agent with access to the full invariant list from prior passes. New stories should be pre-validated against known invariants before being committed. In practice, each new story introduced 3-5 findings because they lacked the rigor of adversarially-converged originals.
