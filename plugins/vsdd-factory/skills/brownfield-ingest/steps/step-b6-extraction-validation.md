---
name: step-b6-extraction-validation
description: Validate accuracy of extracted artifacts against actual source code. Behavioral + metric verification split.
---

# Step B.6: Extraction Validation

> **Shared context:** Read `./_shared-context.md` before executing this step — it contains the Iron Law, Red Flags, subagent delivery protocol, and sandbox considerations.

After the coverage audit passes, verify the **accuracy** of what was extracted (coverage audit verified completeness; this verifies correctness).

Launch the `validate-extraction` agent with access to both:
- Source code: `.reference/<project>/`
- Analysis artifacts: `.factory/semport/<project>/`

## Mandatory: Behavioral vs Metric Split

The agent MUST split work into two distinct phases:

### Phase 1 — Behavioral Verification
Sample contracts, entity definitions, invariant claims, relationship edges, verbatim quotes. For each sample, read the cited source line and report CONFIRMED / INACCURATE / HALLUCINATED.

### Phase 2 — Metric Verification
Independently re-compute every numeric claim in the synthesis using shell commands (`find`, `wc -l`, `grep -c`). Any mismatch is an error regardless of how small.

Report format: two tables, one per phase.

## Protocol

1. Agent reads the final synthesis + all BC files
2. Agent spot-checks a representative sample (~20-30%) of BCs against actual source
3. Agent verifies entity definitions match actual struct/class fields
4. Agent checks dependency graph edges against actual import statements
5. Reports findings as: CONFIRMED, INACCURATE (wrong), HALLUCINATED (doesn't exist)

## Iteration

- If inaccuracies found → fix the analysis artifacts, re-validate
- Maximum 3 refinement iterations
- Write to `.factory/semport/<project>/<project>-extraction-validation.md` (or `-rN.md`)

## Artifacts

- `<project>-extraction-validation.md` — validation report with behavioral + metric tables

## Commit

`factory(phase-0): brownfield ingest extraction validation`

## Success Criteria

- Behavioral sample covers ≥20% of BCs
- All HALLUCINATED entries removed from analysis artifacts
- All INACCURATE entries corrected
- Metric recounts match claimed values
- Result: PASS after ≤3 iterations
