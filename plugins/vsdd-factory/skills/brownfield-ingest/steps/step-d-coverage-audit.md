---
name: step-d-coverage-audit
description: Grep-driven coverage audit verifying all source directories have analysis coverage. Catches topic drift that round-driven deepening misses.
---

# Step B.5: Coverage Audit

> **Shared context:** Read `./_shared-context.md` before executing this step — it contains the Iron Law, Red Flags, subagent delivery protocol, sandbox considerations, and file naming convention.
>
> **B.5 is mandatory** even after exhaustive deepening.

After all passes reach NITPICK, run a deep audit before final synthesis. Round-driven deepening selects targets from prior-round flags, which means topic drift toward repeatedly-covered areas — entire directories can stay unwalked even when overall round count is high. B.5 is the only check that catches this.

## Method

**Must be grep-driven, not agent-judgment-driven.** Don't ask the agent "are there gaps" — make it prove coverage with greps.

Launch a general-purpose agent with this task:

1. **Read the Pass 6 synthesis and all convergence artifacts**
2. **Scan the source directory tree** (`ls -R` on `.reference/<project>/`)
3. **Cross-reference** the directory listing against analysis artifacts — identify directories, modules, or file clusters with zero or surface-only coverage
4. **For each missed subsystem**, read the actual source files and produce:
   - Entity catalog with types
   - Behavioral contracts (BC-AUDIT-NNN format)
   - Integration points with already-documented subsystems
   - Architectural patterns
5. **Write** to `.factory/semport/<project>/<project>-coverage-audit.md`
6. **Novelty assessment**: SUBSTANTIVE if gaps found, PASS if coverage is comprehensive

**The test:** Compare the set of source directories against the set of documented subsystems. Any directory with >100 lines of code and no corresponding entity or BC documentation is a gap.

## Audit Loop

1. Run the audit agent (grep-driven coverage matrix)
2. If gaps found → launch **surgical per-blind-spot mini-rounds** (one targeted file per blind spot). Each mini-round file: `<project>-phase-b5-tr-N.md`. After all mini-rounds land, re-run the audit to verify they closed the gaps.
3. If **PASS** — no remaining directories with substantive uncovered code → audit is complete

**One agent per project — always.**

## Artifacts

- `<project>-coverage-audit.md` (full audit, `-rN.md` for re-runs)
- `<project>-phase-b5-tr-N.md` (targeted blind-spot mini-rounds)

## Commit

`factory(phase-0): brownfield ingest coverage audit + targeted fills`

## Success Criteria

- Coverage matrix produced (package × pass → covered yes/partial/no)
- All directories with >100 LOC have analysis coverage
- Audit result is PASS
