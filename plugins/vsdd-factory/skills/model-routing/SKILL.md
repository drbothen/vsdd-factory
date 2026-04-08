---
name: model-routing
description: >
  Reference skill for LiteLLM model routing strategy. Documents which
  models serve which agents and how fallback chains work.
---

# Model Routing Strategy

## Tier Definitions

| Tier | Models | Cost/M (in/out) | Use For |
|------|--------|-----------------|---------|
| judgment | Claude Opus 4.6 | $5/$25 | Architecture, specs, security review |
| implementation | Claude Sonnet 4.6 | $3/$15 | Code generation, test writing, refactoring |
| validation | Claude Haiku 4.5 | $1/$5 | Consistency checks, formatting, docs |
| adversary | GPT-5.4 | $2.50/$15 | Primary adversarial review -- NEVER Claude |
| review | Gemini 3.1 Pro | $2/$12 | Secondary adversary, rotating code review, security 2nd opinion |
| fallback/fast | Codestral 22B | $0 (self-hosted) / ~$0.15/M | Fast code gen fallback -- NOT for adversarial use |
| fallback/standard | DeepSeek-V3 0324 | ~$0.60/M (OpenRouter) | General fallback -- tool calling, reasoning, code gen |
| fallback/reasoning | Qwen3-235B-A22B (thinking) | ~$2.80/M (OpenRouter) | Reasoning fallback -- adversary/judgment tasks only |

## Routing Rules

1. The Orchestrator selects the model tier based on agent role
2. LiteLLM proxy at localhost:4000 handles actual routing
3. If primary model fails after 2 retries, three-tier fallback chain activates
4. Budget cap: $500 per pipeline run (configurable)
5. The Adversary MUST use adversary tier for primary pass -- never judgment or implementation
6. The Review tier (Gemini 3.1 Pro) is used for secondary adversarial passes and rotating code review
7. Holdout evaluation MUST use adversary tier (GPT-5.4) -- fallback to DeepSeek-V3 (NOT Codestral)
8. Small models (fallback/fast) must NOT be used for adversarial validation -- inverted safety profiles

## Compounding Correctness Constraint

Below a model quality threshold, iterations compound errors — each adversarial pass or
implementation cycle introduces new bugs while fixing old ones. Above the threshold,
iterations compound correctness — each pass genuinely improves the artifact.

**This is why the factory uses frontier models for critical paths:**
- If budget constraints force a model downgrade during Phase 3-5, it is better to
  **pause the pipeline** and resume when the primary model is available than to continue
  with an underpowered model that compounds errors
- The `cost-tracker` plugin (DF-009) detects underspend conditions — if a pipeline
  completes Phase 4 with total spend below 10% of budget, it warns that adversarial
  review may not have been thorough enough
- Holdout evaluation (Phase 3.5) must ALWAYS use frontier models (GPT-5.4 or Gemini 3.1 Pro)
  because satisfaction scoring requires deep behavioral reasoning that small models cannot provide

## Fallback Behavior (Three-Tier)

When a primary model fails:
1. LiteLLM retries 2x with the same model
2. If still failing, routes to the appropriate fallback tier:
   - **Implementation/Validation agents** -> `fallback/fast` (Codestral) -> `fallback/standard` (DeepSeek-V3)
   - **Adversary/Judgment agents** -> `fallback/standard` (DeepSeek-V3) -> `fallback/reasoning` (Qwen3-235B thinking)
   - **Review agents** -> `fallback/standard` (DeepSeek-V3)
3. `fallback/standard` via OpenRouter falls back to `fallback/standard-local` (self-hosted DeepSeek-V3) for sensitive data
4. If all fallbacks fail, the agent reports the error to the Orchestrator
5. The Orchestrator decides: retry later, use a different tier, or escalate to human

## Cost Tracking

LiteLLM tracks cost per request in its database. The Orchestrator reads
cost data and updates STATE.md cost tracking section after each phase.

## Applicability

Reference document — no quality gate. Consumed by orchestrator and LiteLLM configuration.
