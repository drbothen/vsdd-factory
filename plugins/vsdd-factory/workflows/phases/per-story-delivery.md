---
name: per-story-delivery
description: Per-story TDD delivery workflow reference. Governs Step 1 through Step 9 of the story delivery cycle including stub discipline, Red Gate density check, and facade-mode semantics. Loaded by the orchestrator during Phase 3 implementation.
---

# Per-Story Delivery Workflow

This document is the authoritative reference for per-story TDD delivery within a wave. It governs stub discipline, Red Gate density enforcement, and the facade-mode alternative flow. The `deliver-story` skill (`skills/deliver-story/SKILL.md`) is the entry point; this file is the playbook. If the two disagree, this file wins.

## Step 1 — Create worktree (devops-engineer)

Dispatch `devops-engineer` with task: "Create worktree `.worktrees/S-N.MM/` on branch `feature/S-N.MM-<desc>` from `develop`."

**Exit condition:** `git worktree list` shows the new worktree on the correct branch. Verify before proceeding.

## Step 2 — Generate stubs (test-writer as Stub Architect)

> **ANTI-PRECEDENT GUARD:** Do not use sibling crates with pre-implemented stubs as templates for your stub work. If you observe that a sibling crate (e.g., a DTU clone or prior story's scaffold) contains full business logic rather than todo!() macros, treat it as a historical anti-pattern. Your stub must use todo!() for all non-trivial function bodies. Anti-precedent evidence: Prism commits aa706543, 6d2d005e, 20b4a12a. Model precedent: e86d03f2.

**tdd_mode prelude:** Read the story's `tdd_mode:` frontmatter field before dispatching. If `tdd_mode` is absent or unrecognized, default to `strict` and emit: `[WARN] tdd_mode not set for story <id> — defaulting to strict.` (BC-8.30.001 invariant 2 — safety default; no existing story gets silently promoted to facade mode.)

Dispatch `test-writer` with task: "Create compilable stubs in `.worktrees/S-N.MM/` matching the story's file list. Use `todo!()` or `unimplemented!()` bodies. Commit: `feat(S-N.MM): add module stubs`."

**Exit condition:** `cargo check` passes inside the worktree. If it fails, dispatch a new test-writer to fix stubs — do not proceed until clean.

## Step 3 — Write failing tests (test-writer as Test Writer)

Dispatch `test-writer` with task: "Write failing tests in `.worktrees/S-N.MM/` for each acceptance criterion / BC. Commit: `test(S-N.MM): add failing tests for <BC-ref>`."

**Red Gate (mandatory).** After dispatch returns, independently run `cd .worktrees/S-N.MM && cargo test` and verify:

- Tests compile
- All new tests fail
- Tests fail with **assertion errors**, not build errors
- The failure messages reference the behavior under test (not "not yet implemented")

If Red Gate fails, dispatch a new test-writer to fix the tests. Do not proceed to Step 3.5 (Red Gate Density Check) or Step 4 until Red Gate is green (i.e., tests are correctly red).

Record the Red Gate outcome in `.factory/cycles/<cycle-id>/<story-id>/implementation/red-gate-log.md`.

## Red Gate Density Check (BLOCKING before Step 4)

After Step 3 Red Gate passes and before dispatching the Step 4 implementer, compute the RED_RATIO density check. This gate is BLOCKING for `tdd_mode: strict` stories (BC-8.29.001).

### Formula

```
RED_RATIO = RED_TESTS / (TOTAL_NEW_TESTS - EXEMPT_TESTS)
```

where:
- `RED_TESTS` = count of new tests that fail (red) against the stub
- `TOTAL_NEW_TESTS` = count of all tests introduced in this story's delivery
- `EXEMPT_TESTS` = `GREEN-BY-DESIGN_count` + `WIRING-EXEMPT_count`

**GREEN-BY-DESIGN tests** (BC-5.38.002): Tests that exercise functions whose correct behavior is deterministic from the type system alone (e.g., enum variant labels, pure data accessors). These are excluded from the denominator because they cannot be made red without making the stub non-compilable.

**WIRING-EXEMPT tests** (BC-5.38.003): Tests that verify infrastructure wiring (e.g., that a struct implements a trait, that a constructor returns the correct type). These are excluded from the denominator because they pass as soon as the correct type signature exists in the stub, not because of premature implementation.

Integer-precise form (avoids float rounding): `RED_TESTS * 2 >= (TOTAL_NEW_TESTS - EXEMPT_TESTS)`

### Threshold

**RED_RATIO ≥ 0.5** (BLOCKING). If `(TOTAL_NEW_TESTS - EXEMPT_TESTS) > 0` and `RED_RATIO < 0.5`, Step 4 dispatch is blocked.

### Full-Exception Path (denominator = 0)

When all effective tests (after excluding GREEN-BY-DESIGN + WIRING-EXEMPT) are themselves exempt (denominator = 0), the gate does NOT vacuously pass. The orchestrator must explicitly acknowledge this in the red-gate-log file (`.factory/logs/red-gate-log-<story-id>.md`) with `full_exception_path: true`. This documents intent and prevents silent bypass.

### Red Gate Log Format

Write to `.factory/logs/red-gate-log-<story-id>.md` BEFORE any exception-path invocation. Required fields:

```yaml
red_ratio: <computed value>
red_count: <integer>
total_new_tests: <integer>
exempt_count: <integer>
remediation: <option_a | option_b | "" >
full_exception_path: <true | false>
```

Each unexpectedly-GREEN test requires a table entry with:

| test_name | result | rationale_category | notes |
|-----------|--------|-------------------|-------|
| test_foo  | GREEN  | PURE-DATA \| FRAMEWORK-WIRING \| STRUCTURAL-ASSERTION \| PRE-EXISTING-BEHAVIOR \| OTHER-JUSTIFIED \| UNJUSTIFIED | explanation |

UNJUSTIFIED entries are blocking and cannot be waived without human sign-off.

### Remediation Options

When RED_RATIO < 0.5 with UNJUSTIFIED GREEN tests present, the orchestrator must choose one of exactly two options before proceeding:

**Option A (default for automated orchestrators):** Roll back the stub commit and re-dispatch stub-architect with a stricter prompt. Include the explicit list of UNJUSTIFIED functions from the log and instruct stub-architect to replace them with `todo!()`. Step 3 then runs again with the corrected stub. RED_RATIO is recomputed. (BC-8.29.003 EC-001: automated orchestrator MUST default to Option A unless `mutation_testing_required: true` is pre-authorized in story frontmatter.)

**Option B (accept with mutation obligation):** Accept the low ratio and register `mutation_testing_required: true` in the story frontmatter. The wave gate must run `cargo mutants -p <crate> --jobs $(nproc) --timeout 300` for this story's crate as a compensating control (BC-6.21.001, BC-6.21.002). The PR description must disclose: "RED_RATIO was <value> at Step 3 Red Gate. Mutation testing applied at wave gate as compensating control."

No other path forward is permissible. "Proceed without remediation" is not an option.

## Step 4 — Implement (implementer)

Dispatch `implementer` with task: "Implement in `.worktrees/S-N.MM/` via TDD. For each failing test, write the minimum code to make it pass. Micro-commit per test: `feat(S-N.MM): implement <behavior>`. Do not write code not covered by a test."

**Exit condition:** all tests green, clippy clean, `cargo +nightly fmt --all --check` clean, zero `todo!()` / `unimplemented!()` in production code.

## Step 4.5 — Per-Story Adversary Convergence Loop (adversary + implementer)

**Behavioral contracts:** BC-5.39.001 (loop mechanics), BC-5.39.002 (scope constraints). **Decision record:** ADR-017.

This step MUST execute between Step 4 (Implement) and Step 5 (Record demos). Demos record the final converged state — recording before convergence would require re-recording after adversary-driven fixes.

### Scope (BC-5.39.002 PC1)

The adversary's per-story scope is bounded to exactly three information sources:

1. The story worktree diff against `develop` (computed via `git diff develop...HEAD` in the story worktree)
2. The story spec file
3. The BCs anchored to the story via the story's `bcs:` frontmatter array

The adversary MUST NOT load: full codebase context, other stories' specs, PRD sections not referenced in the story spec, or architecture documents not directly cited by the anchored BCs.

### Convergence criterion (BC-5.39.001 PC5)

Convergence is reached when: `passes_clean >= 3 AND last_classification == "NITPICK_ONLY"`.

The `passes_clean` counter increments by 1 for each pass where `last_classification == "NITPICK_ONLY"`. It RESETS to 0 if any pass produces a finding classified above NITPICK_ONLY. Minimum 3 clean passes required — no exceptions.

### Deferred findings (BC-5.39.002 PC4)

Findings that are cross-story, integration-level, system-level, or architectural in scope MUST be tagged as deferred and written to the `deferred_findings` array in the convergence state file. They do NOT block per-story convergence and do NOT reset `passes_clean`.

The four deferred categories are:
- `cross-story` — requires context from another story's scope → target: `wave-gate`
- `integration` — requires knowledge of how multiple stories or subsystems interact → target: `wave-gate`
- `system-level` — concerns system-wide behavior not representable in a single story diff → target: `phase-5`
- `architectural` — concerns design decisions spanning the architectural boundary → target: `phase-5`

### Convergence state file (BC-5.39.001 PC2)

Path: `.factory/cycles/<cycle-id>/<story-id>/adversary-convergence-state.json`

Schema:
```json
{
  "passes_clean": <int>,
  "last_finding_count": <int>,
  "last_classification": "CRITICAL"|"HIGH"|"MEDIUM"|"LOW"|"NITPICK_ONLY",
  "last_timestamp": "<ISO-8601 UTC>",
  "deferred_findings": [
    {
      "finding_id": "<string>",
      "category": "cross-story"|"integration"|"system-level"|"architectural",
      "target": "wave-gate"|"phase-5",
      "note": "<string>"
    }
  ]
}
```

### Loop procedure

1. Dispatch `adversary` agent against the story worktree diff, story spec, and anchored BCs only.
2. Adversary classifies each finding as CRITICAL, HIGH, MEDIUM, LOW, or NITPICK_ONLY. Out-of-scope findings are tagged as deferred (see above) and written to `deferred_findings[]`. They do NOT block convergence.
3. Adversary writes updated convergence state JSON to the per-story state file.
4. If `passes_clean < 3` or `last_classification != "NITPICK_ONLY"`: dispatch `implementer` to fix within-story findings. Repeat from step 1.
5. If convergence criterion is met (`passes_clean >= 3 AND last_classification == "NITPICK_ONLY"`): proceed to Step 5.

**Exit condition:** `passes_clean >= 3 AND last_classification == "NITPICK_ONLY"` in `.factory/cycles/<cycle-id>/<story-id>/adversary-convergence-state.json`.

## Step 5 — Record demos (demo-recorder)

**MUST NOT execute** while `passes_clean < 3` or `last_classification != "NITPICK_ONLY"` in the per-story convergence state file (BC-5.39.001 PC6). Step 4.5 MUST complete successfully before this step begins.

Dispatch `demo-recorder` with task: "Record per-AC demos in `.worktrees/S-N.MM/docs/demo-evidence/<STORY-ID>/`. Use VHS for CLI or Playwright for web. Capture both success and error paths. Generate `docs/demo-evidence/<STORY-ID>/evidence-report.md`."

**Exit condition:** every acceptance criterion has at least one demo artifact referenced in the evidence report.

## Step 6 — Push feature branch (implementer)

Dispatch `implementer` with task: "Push `feature/S-N.MM-<desc>` to remote origin."

**Exit condition:** `git ls-remote origin feature/S-N.MM-<desc>` returns the expected SHA.

## Step 7 — PR lifecycle (pr-manager)

Dispatch `pr-manager` with the full PR process for S-N.MM. Feature branch: `feature/S-N.MM-<desc>`. Target: `develop`.

**Exit condition:** PR merged or blocker reported.

## Step 8 — Cleanup (devops-engineer)

Dispatch `devops-engineer` with task: "Remove worktree `.worktrees/S-N.MM/` and delete local branch `feature/S-N.MM-<desc>`."

**Exit condition:** `git worktree list` no longer shows the worktree.

## Step 9 — State update

Update `.factory/stories/sprint-state.yaml`: story status → `completed`.
Update `.factory/stories/STORY-INDEX.md`: status column for this story.

---

## tdd_mode: facade — Modified Flow

When a story has `tdd_mode: facade` in its frontmatter (explicit; facade-mode does NOT activate by default), the per-story delivery workflow operates under modified semantics. This section documents the facade-mode delivery flow (BC-8.30.002).

Facade mode is appropriate for: DTU API clones, mock server implementations, structural fakes, config parsing wrappers. It is NOT appropriate for algorithmic business logic or domain rule implementations.

### Step 2 (facade-mode)

Combined scaffold+impl commit is allowed. No `todo!()` obligation — the stub-architect (or implementer-as-stub-architect) may write full implementations in the "stub" commit because the facade IS the implementation. The stub commit message MUST include `[facade-mode: tdd_mode=facade]` tag.

### Step 3 (facade-mode)

Test-writer writes spec-anchored fidelity tests post-hoc that verify the scaffold matches its behavioral contract (shape, error codes, field names). Tests may be GREEN immediately — no red phase is required for facade-mode stories. The Red Gate density check (BC-8.29.001) is BYPASSED for facade-mode stories. Mutation testing replaces it as the quality gate.

Every fidelity test must trace to a BC (BC-8.30.002 invariant 3). Unanchored shape-checking is not facade mode.

### Step 4 (facade-mode)

May be a no-op if Step 2 produced a complete implementation. Implementer reviews fidelity tests and confirms correctness.

### Wave gate (facade-mode)

`cargo mutants -p <crate> --jobs $(nproc) --timeout 300` is REQUIRED for ALL facade crates in the wave (BC-6.21.001). Kill rate must be ≥ 80% (BC-6.21.002). Failure to meet kill rate blocks wave gate merge.

`tdd_mode: facade` does NOT exempt a story from mutation testing — it changes WHEN mutation testing runs (wave gate instead of step gate) and WHAT it tests (the complete implementation). If mutation testing is skipped for any reason, the facade bypass of Red Gate density check retroactively violates BC-8.30.002.
