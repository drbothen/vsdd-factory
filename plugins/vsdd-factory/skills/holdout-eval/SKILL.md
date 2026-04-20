---
name: holdout-eval
description: Run holdout evaluation against merged wave code. Spawns the holdout-evaluator agent with strict information asymmetry — cannot see specs, source internals, or prior reviews. Returns satisfaction scores per hidden scenario.
argument-hint: "[wave-N]"
disable-model-invocation: true
context: fork
agent: holdout-evaluator
---

# Holdout Evaluation

Launch the holdout-evaluator agent to run hidden acceptance scenarios against merged code.

## The Iron Law

> **NO HOLDOUT EVALUATION WITHOUT INFORMATION ASYMMETRY FIRST**

The holdout evaluator must NEVER see specs, source code, behavioral contracts, architecture, implementation rationale, or prior adversarial reviews. The evaluator sees only the product brief, the hidden scenarios, and the running application. Breaking this wall turns holdout evaluation into a self-confirming test — the evaluator will pass things that "look right" based on spec knowledge instead of testing whether the system actually works.

## Red Flags

| Thought | Reality |
|---|---|
| "The evaluator needs the PRD to understand what to test" | The evaluator has the product brief and hidden scenarios. PRD access breaks the wall. |
| "Let me show the evaluator the architecture so it can test more effectively" | Architecture knowledge biases evaluation toward implementation details, not behavior. |
| "The evaluator should see the source code to understand error paths" | The evaluator tests observable behavior. Internal error paths are the adversary's job. |
| "Satisfaction of 0.84 is basically 0.85, let's round up" | 0.85 is the threshold. 0.84 fails. No rounding. |
| "This holdout scenario is too hard, let me modify it" | Holdout scenarios are written before implementation. They test what SHOULD work, not what DOES work. |
| "The evaluator scored low because it misunderstood the scenario" | Low scores are findings. Investigate whether the implementation or the scenario is wrong. |
| "We can skip holdout for this wave — it's just infrastructure" | Infrastructure affects behavior. Holdout covers the user-visible impact. |
| "Let me tell the evaluator which tests already pass" | Test results from the test-writer leak implementation knowledge. The evaluator runs its own tests. |

## Templates

Read and follow the output format in:
- `${CLAUDE_PLUGIN_ROOT}/templates/holdout-evaluation-report-template.md` — evaluation report
- `${CLAUDE_PLUGIN_ROOT}/templates/evaluation-summary-template.md` — evaluation summary
- `${CLAUDE_PLUGIN_ROOT}/templates/evaluation-per-scenario-template.md` — per-scenario scoring
- `${CLAUDE_PLUGIN_ROOT}/templates/evaluation-index-template.md` — evaluation index

## Input

`$ARGUMENTS` — wave identifier (e.g., `wave-1`)

## When to Run

- After all stories in a wave are merged to `develop`
- Before starting the next wave
- This is Phase 4 (Holdout Evaluation) — between implementation (Phase 3) and adversarial refinement (Phase 5)

## What the Evaluator Receives

The holdout-evaluator agent sees ONLY:
1. `.factory/holdout-scenarios/wave-scenarios/<wave>/` — the hidden scenarios
2. `.factory/specs/product-brief.md` — high-level product description
3. The running application (via CLI or API)
4. Test pass/fail results (not test source code)

## What the Evaluator Cannot See

- `.factory/specs/prd.md` and behavioral contracts
- `.factory/specs/architecture/`
- Source code internals (`src/`)
- `.factory/semport/` artifacts
- `.factory/cycles/*/adversarial-reviews/`
- PR discussions and implementation rationale

## Gate Criteria

- **PASS**: Mean satisfaction ≥ 0.85, every critical scenario ≥ 0.60
- **FAIL**: Below thresholds — stories need remediation before next wave

## After Evaluation

Results written to `.factory/holdout-scenarios/evaluations/wave-<N>/`.

If PASS: proceed to next wave or Phase 5 (adversarial refinement).
If FAIL: report gaps, create remediation stories.
