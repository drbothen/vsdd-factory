---
name: step-d-triage-and-fix
description: Classify findings by severity, route to responsible agents, and run cross-document sync check before re-review.
---

# Step D: Triage and Fix

> **Shared context:** Read `./_shared-context.md` before executing this step.

Classify each finding by severity and route to the responsible agent for remediation.

## Procedure

### 1. Severity Routing

For each finding, route based on severity:

| Severity | Action | Blocks Phase 2? |
|----------|--------|-----------------|
| **CRITICAL** | Fix immediately, re-review | Yes |
| **HIGH** | Fix before Phase 2 | Yes |
| **MEDIUM** | Document for later, don't block | No |
| **LOW** | Log and continue | No |

### 2. Fix Root Causes, Not Symptoms

When a finding shows drift (wrong error codes, missing struct fields, wrong formulas), the fix MUST be:
1. Read the authoritative source (BC, PRD, architecture)
2. Rewrite the contradicting section from scratch
3. **Never** apply targeted text replacements without first reading both the source and target

In practice, incremental line-level patches cause the same findings to recur across 3-5 passes.

### 3. Accumulate Invariants

After each fix cycle, update the adversary prompt with ALL confirmed invariants from prior passes (struct fields, error codes, version pins, dependency rules, persistence models). The invariant list grows monotonically — never shrinks.

### 4. Cross-Document Sync Check (MANDATORY before re-review)

After remediation and before re-review:

1. Spawn `consistency-validator` with task: "sync check between [documents that were remediated]"
2. If sync issues found: fix them sequentially, upstream first
3. Only proceed to re-review (Step E) after sync check passes

This prevents fixes from introducing new inconsistencies that the adversary will immediately find in the next pass.

### 5. Pre-validate New Scope Additions

If new stories or spec sections are added during adversarial convergence, they must be:
- Written by an agent with access to the full invariant list from prior passes
- Pre-validated against known invariants before being committed
- Each new story typically introduces 3-5 findings if not pre-validated

## Artifacts

- Fixed spec artifacts (updated in `.factory/specs/`)
- Consistency validation report (from consistency-validator)
- Updated invariant list for next adversary pass

## Success Criteria

- All CRITICAL findings fixed
- All HIGH findings fixed
- Cross-document sync check passes
- Invariant list updated for next pass
- No incremental patches — all fixes are full rewrites from authoritative sources
