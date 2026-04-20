---
name: step-g-secondary-pass
description: Optional secondary adversary pass using review-tier model for cognitive diversity. Recommended for security-critical or large deltas.
---

# Step G: Secondary Adversarial Pass

> **Shared context:** Read `./_shared-context.md` before executing this step.

After primary adversary convergence, optionally spawn a secondary review using the review-tier model for cognitive diversity.

## Procedure

### 1. Decision — Run or Skip

**Recommended for:**
- Security-critical deltas (auth, crypto, access control changes)
- Large deltas spanning many files (review-tier model's large context window can review the entire delta + dependent files in one pass)
- When maximum cognitive diversity is valued

**NOT recommended for:**
- Trivial bug fixes
- Cosmetic changes
- Single-file modifications with narrow scope

Document the decision and rationale regardless of outcome.

### 2. Spawn Secondary Adversary

Spawn `review/primary` (review-tier model) with:
- Fresh context (no knowledge of primary adversary's findings)
- Same review package as the primary adversary
- Same information asymmetry walls

### 3. Process Secondary Findings

- Write secondary findings to `.factory/phase-f5-adversarial/gemini-review.md`
- Any new CRITICAL/HIGH findings route through Step E/F fix cycle
- Secondary findings are **additive** — they can only extend, not replace primary findings

### 4. Multi-Repo Contract Compliance (DF-013)

For cross-repo deltas that modify API contracts:
- Run **contract testing** (Pact/Specmatic) to validate all consumer repos comply
- The adversary reviews both the contract change AND consumer-side adaptations
- Breaking contract change without consumer updates = CRITICAL finding

## Artifacts

- `.factory/phase-f5-adversarial/gemini-review.md` (if secondary pass was run)
- `.factory/phase-f5-adversarial/contract-compliance.md` (multi-repo projects only)

## Success Criteria

- Decision to run or skip documented with rationale
- If run: findings processed through same triage/fix cycle
- If run: both primary and secondary adversaries independently report cosmetic-only before convergence
- For multi-repo: contract compliance validated across all consumer repos
