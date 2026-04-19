---
name: phase-1-spec-crystallization
description: Phase 1 entry point — transform product brief into domain spec, PRD, behavioral contracts, architecture, and verification properties. Delegates to phase sub-workflow.
---

# Phase 1: Spec Crystallization

Phase entry point for spec creation. Produces the complete specification package from product brief through architecture.

## Sub-Workflow

Execute the steps defined in:
```
${CLAUDE_PLUGIN_ROOT}/workflows/phases/phase-1-spec-crystallization.lobster
```

## Steps

| Step | Skill | What It Does |
|------|-------|-------------|
| A | `create-brief/SKILL.md` | Product brief creation |
| B | `create-domain-spec/SKILL.md` | L2 domain specification |
| C | `create-prd/SKILL.md` | PRD + behavioral contracts |
| D | `create-architecture/SKILL.md` | Architecture + verification properties |
| E | `phase-1-prd-revision/SKILL.md` | PRD revision after architect feedback (conditional) |
| F | `phase-1d-adversarial-spec-review/SKILL.md` | Adversarial convergence loop |

## Work Skills

- `/vsdd-factory:create-brief`
- `/vsdd-factory:create-domain-spec`
- `/vsdd-factory:create-prd`
- `/vsdd-factory:create-architecture`

## Prerequisites

- Product concept or existing `.factory/planning/product-brief.md`
- `.factory/` worktree mounted on `factory-artifacts` branch

## Gate Criteria

- All requirements have unique IDs and numerical targets
- Provable Properties Catalog covers all security boundaries
- Purity Boundary Map complete
- Verification tooling selected and documented
- Module criticality classification written
- Adversarial spec review converged
- Input-hash drift check clean
- Human approval
