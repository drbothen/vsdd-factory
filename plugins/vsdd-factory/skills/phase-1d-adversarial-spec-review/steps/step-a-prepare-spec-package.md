---
name: step-a-prepare-spec-package
description: Collect all Phase 1 spec artifacts into a review package for the adversary.
---

# Step A: Prepare Spec Package

> **Shared context:** Read `./_shared-context.md` before executing this step.

Collect all Phase 1 artifacts that the adversary will review. The adversary receives the FULL spec corpus — not a summary, not selected sections.

## Procedure

1. **Collect all spec artifacts:**
   - `.factory/specs/prd.md`
   - `.factory/specs/prd-supplements/` (interface-definitions, error-taxonomy, test-vectors, nfr-catalog)
   - `.factory/specs/architecture/ARCH-INDEX.md` + all section files
   - `.factory/specs/behavioral-contracts/**` (all BC files)
   - `.factory/specs/verification-properties/**` (all VP files)
   - `.factory/specs/ux-spec/UX-INDEX.md` + screen/flow files (if applicable)
   - `.factory/specs/domain-spec/L2-INDEX.md` + all sections (if exists)
   - `.factory/specs/module-criticality.md`

2. **Verify completeness:**
   - All spec artifacts committed to factory-artifacts branch
   - No TODO/TBD placeholders in committed artifacts
   - BC-INDEX, ARCH-INDEX, VP-INDEX exist and are populated

3. **Prepare context include/exclude lists** for adversary spawn (used in Step B).

## Artifacts

- Review package (in-memory) — list of all spec artifact paths for adversary context

## Success Criteria

- All Phase 1 spec artifacts are identified and accessible
- No artifacts are missing or in draft state
- Include/exclude context lists are ready for Step B
