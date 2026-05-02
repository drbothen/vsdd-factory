---
name: wave-gate
description: Run the post-wave integration gate — full test suite on develop, adversarial review of wave diff, holdout evaluation, demo evidence validation, and DTU validation for critical modules. Blocks next wave until all checks pass.
argument-hint: "[wave-N]"

allowed-tools: Read, Write, Edit, Bash, Glob, Grep, AskUserQuestion
---

# Wave Gate

Orchestrate the post-wave quality gate. All checks must pass before the next wave can begin.

## The Iron Law

> **NO WAVE ADVANCE WITHOUT ALL SIX GATES PASSING FIRST**

Violating the letter of the rule is violating the spirit of the rule. "Close enough" is not passing. A gate that was skipped, mocked, or partially verified counts as failed.

## Announce at Start

Before any other action, say verbatim:

> I'm using the wave-gate skill to run the post-wave integration gate for wave-N.

Then create a TodoWrite entry per gate (six entries). Mark each in-progress before running and completed only after its pass criteria are independently verified.

## Red Flags

| Thought | Reality |
|---|---|
| "Gate 2 doesn't apply to this wave, skip it entirely" | Skip means report `SKIP` with the reason. Do not delete the gate from the output. |
| "One test failure is flaky, let me re-run just that test" | Flaky tests are findings. Record the flake, fix or quarantine, then re-run Gate 1. |
| "The adversary only found HIGH severity, that's not blocking" | HIGH without remediation is a tech debt entry that must be filed before advancing. |
| "Demo evidence exists for most ACs, we'll backfill the rest" | Missing demo evidence blocks the gate. Dispatch demo-recorder for the missing ACs. |
| "Holdout scored 0.84, that's basically 0.85" | 0.85 is the threshold. 0.84 fails. No rounding. |
| "Let me run the gates in a different order to save time" | Gate order is load-bearing. Test suite first because everything else assumes it passes. |
| "I'll advance the wave now and circle back to fix Gate 5 later" | Advancing with a failed gate means the next wave builds on unverified ground. Do not. |
| "The test suite passed on my branch, I can skip the develop run" | Gate 1 runs on `develop` specifically to catch merge-order surprises. Run it there. |


## Input

`$ARGUMENTS` — wave identifier (e.g., `wave-1`, `wave-2`)

## Prerequisites

- All stories in this wave must be merged to `develop`
- Verify by reading `.factory/stories/sprint-state.yaml` — all wave stories should have status `merged`

If any stories are still `in-review` or `in-progress`, abort and report which stories are blocking.

## Gate Sequence

Run these checks in order. If any check fails, stop and report — do not continue to the next check.

### Gate 1: Full Test Suite

```bash
git checkout develop
cargo test --release 2>&1
cargo clippy --workspace --all-targets --all-features -- -D warnings 2>&1
cargo +nightly fmt --all --check 2>&1
```

**Pass criteria:** All tests pass, clippy clean, format clean.

If failed: report failures. Stories need fixes before continuing.

### Gate 2: DTU Validation (if applicable)

Check `.factory/specs/prd-supplements/module-criticality.md` for CRITICAL/HIGH modules touched by this wave's stories.

If any exist, run DTU comparison:
- Read `.factory/dtu-clones/` for active clones
- Run comparison harnesses against develop
- Any divergence in a CRITICAL module is a **blocking** failure

**Pass criteria:** All DTU clones in sync, or no DTU-covered modules in this wave.

### Gate 3: Adversarial Review of Wave Diff

Get the wave diff:
```bash
git log develop --oneline --since="<wave start date>"
git diff <pre-wave commit>..develop
```

Launch `/adversarial-review implementation` scoped to the wave's changes.

**Pass criteria:** No CRITICAL findings. HIGH findings documented in tech debt register or addressed.

### Gate 4: Demo Evidence

For each story in this wave, verify demo evidence exists:
- Check `.factory/demo-evidence/STORY-NNN/demo-report.md`
- Verify all acceptance criteria have evidence entries

If any story is missing demo evidence, run `/record-demo STORY-NNN` for each missing story.

**Pass criteria:** All stories have demo reports covering all ACs.

### Gate 5: Holdout Evaluation

Launch `/holdout-eval wave-N`.

This spawns the holdout-evaluator agent with information asymmetry — it cannot see specs or source code, only the running application and hidden scenarios.

**Pass criteria:** Mean satisfaction ≥ 0.85, every critical scenario ≥ 0.60.

### Gate 6: State Update

If all gates pass:
- Update sprint-state.yaml: all wave stories → `completed`
- Update STATE.md with wave completion
- Commit to factory-artifacts

## Telemetry

After completing each gate, emit a structured GATE_CHECK line. The
`validate-wave-gate-completeness` hook validates these are present in
the gate report before allowing `gate_status: passed` in wave-state.yaml.

```
GATE_CHECK: gate=1 name=test-suite status=pass note=<N> tests, 0 failures
GATE_CHECK: gate=2 name=dtu-validation status=skip note=no critical modules in wave
GATE_CHECK: gate=3 name=adversarial-review status=pass note=novelty LOW, 0 critical
GATE_CHECK: gate=4 name=demo-evidence status=pass note=<N> stories, all ACs covered
GATE_CHECK: gate=5 name=holdout-eval status=pass note=mean 0.92, min critical 0.78
GATE_CHECK: gate=6 name=state-update status=pass note=sprint-state updated
```

Valid status values: `pass`, `fail`, `skip` (with mandatory reason in note).

## Output

Include both the human-readable summary AND the GATE_CHECK lines in the gate report:

```
Wave Gate: wave-N

  Gate 1 — Test Suite:       ✅ PASS (<N> tests, 0 failures)
  Gate 2 — DTU Validation:   ✅ PASS (2 clones in sync) | ⏭️ SKIP (no critical modules)
  Gate 3 — Adversarial:      ✅ PASS (novelty LOW, 0 critical findings)
  Gate 4 — Demo Evidence:    ✅ PASS (<N> stories, all ACs covered)
  Gate 5 — Holdout Eval:     ✅ PASS (mean: 0.92, min critical: 0.78)
  Gate 6 — State Update:     ✅ PASS (sprint-state.yaml updated)

  GATE_CHECK: gate=1 name=test-suite status=pass note=<N> tests, 0 failures
  GATE_CHECK: gate=2 name=dtu-validation status=pass note=2 clones in sync
  GATE_CHECK: gate=3 name=adversarial-review status=pass note=novelty LOW, 0 critical
  GATE_CHECK: gate=4 name=demo-evidence status=pass note=<N> stories, all ACs covered
  GATE_CHECK: gate=5 name=holdout-eval status=pass note=mean 0.92, min critical 0.78
  GATE_CHECK: gate=6 name=state-update status=pass note=sprint-state updated

  WAVE GATE: ✅ PASSED — ready for wave-<N+1>
```

Or on failure:

```
  GATE_CHECK: gate=1 name=test-suite status=pass note=42 tests
  GATE_CHECK: gate=2 name=dtu-validation status=skip note=no critical modules
  GATE_CHECK: gate=3 name=adversarial-review status=fail note=1 CRITICAL finding

  WAVE GATE: ❌ FAILED at Gate 3

  Blocking findings:
    - ADV-P1-001: <critical finding description>
  
  Fix these before retrying: /wave-gate wave-N
```

## Mutation Testing (facade-mode wave-gate)

This gate runs mutation testing for facade-mode stories and Option-B-elected stories
as a compensating control for the bypassed Red Gate density check. It enforces the
≥80% kill-rate floor required by BC-6.21.001 and BC-6.21.002.

### Scan: Which Stories Require Mutation Testing?

Before running mutation tests, scan all stories merged in this wave:

1. Stories with `tdd_mode: facade` in frontmatter (BC-8.30.002)
2. Stories with `mutation_testing_required: true` in frontmatter (Option B election
   from BC-8.29.003 EC-001 — low RED_RATIO accepted with mutation obligation)

If the wave has **zero** facade stories AND zero `mutation_testing_required: true`
stories, skip mutation testing — but emit an explicit log entry (never silent):

```
no facade stories in wave — mutation testing step skipped
```

### Tool Availability Check

Before invoking cargo-mutants, verify it is installed:

```bash
cargo mutants --version 2>/dev/null || echo "NOT_FOUND"
```

If `cargo-mutants` is not installed, **BLOCK the wave gate** with:

```
cargo-mutants not found — install with cargo install cargo-mutants
```

Do not proceed past this check until the tool is available.

### Execution

For each crate that requires mutation testing (from the scan above):

```bash
cargo mutants -p <crate> --jobs $(command -v nproc >/dev/null && nproc || sysctl -n hw.ncpu || echo 4) --timeout 300
```

**Cross-platform nproc note:** `nproc` is a GNU coreutils command available on Linux.
On macOS use `sysctl -n hw.ncpu`. The portable form above handles both automatically.

**Wave-level timeout:** The sum of all per-crate mutation runs must complete within
60 minutes. If the wave-level budget is exceeded, report partial results and block
the gate with: `mutation testing budget exceeded (60 min) — partial results at <path>`.

### Report

Write the mutation report for each crate to:

```
.factory/logs/mutation-report-<wave-id>-<crate>.md
```

Commit this report as part of the wave PR (BC-6.21.001 invariant 3). The report
must be committed before the wave gate is declared passing.

### Kill-Rate Gate (BC-6.21.002)

Compute the kill rate using integer-precise arithmetic (avoids float rounding):

```
killed * 100 / total >= 80
```

If `killed * 100 / total < 80`, the wave gate **fails**. The threshold is exactly
80 — no rounding, no "close enough."

All surviving mutants must be dispositioned. Use the following table format in the
mutation report:

| Mutant | File | Line | Mutation | Disposition (A/B/C) | Notes |
|--------|------|------|----------|---------------------|-------|

**Disposition A — New test:** Write a new test that kills this mutant and re-run
`cargo mutants` to confirm the mutant is now killed. Commit the new test before
closing the disposition.

**Disposition B — Dead-code-equivalent:** Explain the execution condition that makes
this mutation non-reachable in practice. The explanation must name the specific
condition, not just assert "unreachable."

**Disposition C — Explicit waiver:** Name the mutant, file, line, and mutation type.
Justify why killing this mutant is not feasible or not valuable. NO blanket waivers
are accepted — each surviving mutant requires its own named entry.

Unprocessed surviving mutants (no disposition assigned) block the wave gate merge.

## After Passing

Tell the user:
- Wave N is complete
- Next wave stories are unblocked
- Use `/deliver-story STORY-NNN` to start the next wave
- Or if this was the last wave, proceed to Phase 5: `/adversarial-review implementation` for full codebase review
