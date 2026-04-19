---
name: step-f-final-synthesis
description: Produce the definitive synthesis incorporating all broad passes, deepening rounds, coverage audit, and extraction validation results.
---

# Step C: Final Synthesis

> **Shared context:** Read `./_shared-context.md` before executing this step — it contains the Iron Law, Red Flags, file naming convention, Phase D (Vision Disposition), and How Brownfield Differs from Semport.

After ALL passes converge, coverage audit passes, AND extraction validation passes, produce the definitive synthesis.

## Procedure

Read all pass files (broad + all deepening rounds) and produce:

- Complete feature set
- Bounded context map
- Complexity ranking
- Critical design decisions
- Anti-patterns
- Spec crystallization recommendations
- Convergence report: rounds per pass, novelty trajectory, total coverage metrics

Output: `.factory/semport/<project>/<project>-pass-8-deep-synthesis.md`

## Mandatory: Priority-Ordered Lessons Section

The synthesis MUST include a `## Lessons for <target-project>` section organized in priority order. Without this section, downstream skills have to re-derive actionable conclusions.

Each lesson names four things:
- **(a) What the target does today** — cite target file:line or "nothing in target"
- **(b) What the reference does** — cite reference file:line
- **(c) The gap** — one sentence, concrete
- **(d) Specific action items** — file paths that need editing, plus nature of edit

Priority buckets:
- **P0 — Correctness gaps** that must fix before next release
- **P1 — High-ROI improvements** to adopt
- **P2 — Worth considering** (list trade-offs)
- **P3 — Known divergences** to document

## Templates

- `${CLAUDE_PLUGIN_ROOT}/templates/recovered-architecture-template.md`

## Artifacts

- `<project>-pass-8-deep-synthesis.md` — definitive synthesis

## Commit

`factory(phase-0): brownfield ingest final synthesis — all passes converged`

## Success Criteria

- Synthesis cross-references all passes and deepening rounds
- Lessons section present with P0/P1/P2/P3 buckets
- Convergence report shows rounds-per-pass and total metrics
- All cited artifacts are CONFIRMED (post-extraction-validation)
