---
name: phase-1d-adversarial-spec-review
description: >
  VSDD Phase 1d: Present complete spec package to the Adversary (different
  model family, fresh context) for adversarial review.
---

> **Delegation Reference:** This skill describes work the orchestrator delegates
> to specialist agents via sessions_spawn. Each step names the target agent.
> The orchestrator does NOT execute these steps directly.

# Phase 1d: Adversarial Spec Review

## Prerequisites

- Phase 1 spec artifacts complete: PRD, architecture directory, UX spec (if applicable)
- All spec artifacts committed to factory-artifacts branch
- Adversary model configured in `openclaw.json` (different model family from builder)
- `.factory/specs/adversarial-reviews/` directory exists or will be created

## Workflow

### Step 1: Prepare Spec Package
Collect all Phase 1 artifacts:
- `.factory/specs/prd.md`
- `.factory/specs/prd-supplements/` (interface-definitions, error-taxonomy, test-vectors, nfr-catalog)
- `.factory/specs/architecture/ARCH-INDEX.md` + section files
- `.factory/specs/behavioral-contracts/**`
- `.factory/specs/verification-properties/**`
- `.factory/specs/ux-spec/UX-INDEX.md` + screen/flow files (if applicable)

### Step 2: Spawn Adversary
Spawn `adversary` agent (adversary model) with:
- FRESH CONTEXT -- no prior conversation history
- READ-ONLY access to spec artifacts
- Instructions to review for:
  - Ambiguous language
  - Missing edge cases
  - Implicit unstated assumptions
  - Contradictions between sections
  - Properties marked "testable only" that should be provable
  - Purity boundary violations
  - Verification tool mismatches

### Step 3: Collect Findings
Adversary writes sharded findings to `.factory/specs/adversarial-reviews/`
as per-finding files with ADV-P[N]-INDEX.md + FINDINGS.md (DF-021).

### Step 4: Triage and Fix
For each finding:
- CRITICAL -> Fix immediately, re-review
- HIGH -> Fix before Phase 2
- MEDIUM -> Document for later, don't block
- LOW -> Log and continue

### Step 4b: Cross-Document Sync Check

After remediation and before re-review:
1. Spawn consistency-validator with task: "sync check between
   [documents that were remediated]"
2. If sync issues found: fix them (sequential, upstream first)
3. Only proceed to re-review after sync check passes

### Step 5: Re-Review (Fresh Context)
After fixes, spawn adversary AGAIN with fresh context.
Repeat until adversary reports: "CONVERGENCE REACHED -- findings are cosmetic only."

## Output Artifacts

| Artifact | Path | Producer |
|----------|------|----------|
| Per-finding files | `.factory/specs/adversarial-reviews/ADV-P[N]-NNN.md` | adversary |
| Pass index | `.factory/specs/adversarial-reviews/ADV-P[N]-INDEX.md` | adversary |
| Cross-pass tracker | `.factory/specs/adversarial-reviews/FINDINGS.md` | adversary |

## Quality Gate
- [ ] Adversary is a different model family from the builder
- [ ] Fresh context on every review pass
- [ ] All critical/high findings resolved
- [ ] Adversary reports convergence
