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
