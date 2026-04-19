---
name: decompose-stories-shared-context
description: Shared context loaded by all decompose-stories step files. Contains hard gate, templates, prerequisites, plan failures, and self-review checklist.
---

# Decompose Stories — Shared Context

This file is loaded by every step in the decompose-stories skill. It contains cross-cutting constraints that apply to all steps.

## Hard Gate

Do NOT skip to implementation or story delivery. ALL stories MUST be decomposed, dependency-ordered into waves, and approved before any code is written.

## Templates

Read and follow the output format in:
- `${CLAUDE_PLUGIN_ROOT}/templates/story-template.md` — STORY-NNN format
- `${CLAUDE_PLUGIN_ROOT}/templates/epic-template.md` — epic structure
- `${CLAUDE_PLUGIN_ROOT}/templates/wave-schedule-template.md` — wave schedule
- `${CLAUDE_PLUGIN_ROOT}/templates/holdout-scenario-template.md` — hidden acceptance scenarios
- `${CLAUDE_PLUGIN_ROOT}/templates/traceability-matrix-template.md` — requirement traceability

## Prerequisites

- `.factory/specs/prd.md` with behavioral contracts
- `.factory/specs/architecture/` with at least ARCH-INDEX.md
- Phase 1 should be complete (adversarial review passed)

### Reference Repos (conditional)

If `.factory/reference-manifest.yaml` exists, reference implementations inform story decomposition:
- Stories that implement behavior extracted from a reference repo should be tagged `implementation_strategy: gene-transfusion` and include a `Reference Source` field pointing to the relevant `.factory/semport/<project>/` artifacts.
- Stories that diverge from reference behavior should be tagged `implementation_strategy: from-scratch` and note the divergence in Dev Notes.
- When estimating complexity, factor in whether the reference implementation can be adapted vs. needs to be reimagined.

## Scope Check

Before decomposing, verify the PRD describes a single product. If it contains multiple independent products or platforms, stop and split the PRD first — each product gets its own decomposition cycle.

## Plan Failures

These patterns invalidate a story. If you catch any, fix before proceeding:

- "TBD", "TODO", or "implement later" in any section
- "Add appropriate error handling" without specifying which errors
- "Write tests for the above" without actual test descriptions
- "Similar to STORY-NNN" without repeating the relevant details
- Acceptance criteria without testable assertions
- File list that says "and other files as needed"
- Tasks that describe what to do without specifying how

## Self-Review (before adversarial review)

Before routing to adversarial review, check your own work:

1. **Spec coverage:** Does every BC in the PRD trace to at least one story? List any gaps.
2. **Placeholder scan:** Any "TBD", stub stories, or incomplete dependency mappings? Fix them.
3. **Consistency:** Do story IDs match the index? Do wave assignments respect dependencies?
4. **Sizing:** Any story over 13 points? Any story estimated over 60% of agent context window?

Fix issues inline. This is a cheap filter — catch obvious gaps before spending tokens on the adversary.

## Allowed Tools

- Read, Write, Edit, Bash, AskUserQuestion
- Model invocation is disabled (`disable-model-invocation: true`)
