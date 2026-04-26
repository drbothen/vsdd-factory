---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: product-owner
timestamp: 2026-04-26T00:00:00
phase: 1a
inputs: [.factory/stories/S-7.03-tdd-discipline-hardening.md]
input-hash: "a3187d9"
traces_to: .factory/stories/S-7.03-tdd-discipline-hardening.md
origin: brownfield
extracted_from: ".factory/stories/S-7.03-tdd-discipline-hardening.md#AC-010"
subsystem: "SS-06"
capability: "CAP-016"
lifecycle_status: active
introduced: v1.0-brownfield-backfill
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
bc_id: BC-6.21.001
section: "6.21"
---

# BC-6.21.001: wave-gate skill must run cargo mutants for every story with tdd_mode=facade in the wave

## Description

The wave-gate skill (`/vsdd-factory:wave-gate`) must, as part of its execution, identify all stories in the wave with `tdd_mode: facade` and run `cargo mutants -p <crate> --jobs N --timeout 300` against each such story's crate. This execution is not optional and not skippable without explicit human waiver. The mutation testing compensates for the absence of a strict Red Gate cycle in facade-mode stories, providing mathematical coverage assurance in place of procedural discipline.

## Preconditions

1. The wave-gate skill is executing for a wave that contains at least one story with `tdd_mode: facade`.
2. The crate name for each facade story is available (from story frontmatter or implementation manifest).
3. `cargo-mutants` is installed in the toolchain (`cargo install cargo-mutants`).
4. The wave's implementation commits have been applied (mutation testing runs against implemented code, not stubs).

## Postconditions

1. `cargo mutants -p <crate> --jobs N --timeout 300` has been executed for each facade-mode crate in the wave.
2. The mutation results are written to `.factory/logs/mutation-report-<wave-id>-<crate>.md` with: total mutants, killed mutants, surviving mutants (list with source location), kill rate percentage.
3. The kill rate for each crate is compared against the 80% floor (BC-6.21.002).
4. Wave gate proceeds only if all facade crates meet the kill rate threshold (or have surviving mutants addressed per BC-6.21.002 disposition options).

## Invariants

1. `--timeout 300` is the maximum per-mutant timeout. Higher values indicate a flaky test suite and must be investigated separately.
2. `--jobs N` should be set to the available CPU count (default: number of logical cores). Do not set below 2. On Linux use `$(nproc)`. On macOS use `$(sysctl -n hw.ncpu)`. Implementations should use a portable wrapper: `$(nproc 2>/dev/null || sysctl -n hw.ncpu)`.
3. The mutation report must be committed to `.factory/logs/` as part of the wave gate PR. It is an auditable artifact.
4. If `cargo-mutants` is not installed, wave gate BLOCKS with error "cargo-mutants not found — install with `cargo install cargo-mutants` before running wave gate."
5. Mutation testing runs on the FINAL implementation state, not intermediate commits. Ensure all step commits for facade stories are applied before running.
6. Wave-level mutation budget is 60 minutes wall-clock. If projected runtime (estimated total mutants × 300s ÷ jobs) exceeds budget, scope MUST be reduced via `--file` filters BEFORE invocation, not after timeout. The wave-gate skill is responsible for the projection calculation and scope-narrowing decision.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Wave contains no facade stories | Mutation testing step is skipped. Wave gate proceeds normally. |
| EC-002 | `cargo mutants` times out on a large crate | The timeout-300 flag limits per-mutant time. If the overall run exceeds wave-gate SLA (60 min), reduce crate scope with `--file` or `--function` flags. Log the scoping decision. |
| EC-003 | A story has `mutation_testing_required: true` (Option B from BC-8.29.003) but `tdd_mode: strict` | Mutation testing is still required — Option B creates the same obligation as facade mode. The same execution applies. |
| EC-004 | Same crate targeted by two facade stories in the same wave | Run mutation testing once against the combined crate state after both stories are applied. Do not run twice. |
| EC-005 | Story frontmatter `wave: null` (wave field is null or absent) | The mutation report path uses `<story-id>` instead of `<wave-id>` to avoid the literal string "null" in the filename: `.factory/logs/mutation-report-<story-id>-<crate>.md`. If `wave:` has an integer or string value, that value is used as `<wave-id>`. |

## Canonical Test Vectors

| Wave Contents | Expected Wave Gate Behavior |
|---------------|-----------------------------|
| 3 strict stories, 2 facade stories | `cargo mutants -p <crate1>` and `cargo mutants -p <crate2>` both run; reports produced; kill rates checked |
| 5 strict stories, 0 facade stories | No mutation testing step; wave gate skips to normal checks |
| 1 facade story, cargo-mutants not installed | BLOCKS with install instruction |
| 1 facade story, mutation report shows kill rate = 75% | BLOCKS per BC-6.21.002; surviving mutants must be addressed |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-064 | Wave-gate skill executes cargo mutants for all facade-mode crates and blocks on missing execution | manual (procedural verification) |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-016 |
| Capability Anchor Justification | CAP-016 ("Drive TDD delivery with red/green/refactor gate enforcement") per capabilities.md §CAP-016 — this BC specifies how the wave-gate skill enforces TDD quality assurance for facade-mode stories by running mutation testing as the compensating control that replaces the strict Red Gate cycle CAP-016 otherwise requires. |
| L2 Domain Invariants | none |
| Architecture Module | plugins/vsdd-factory/skills/wave-gate/SKILL.md |
| Stories | S-7.03 |
| Source AC | S-7.03 §AC-010 |
| FR | FR-043 |

## Related BCs

- BC-6.21.002 — sibling (80% kill rate floor and disposition options)
- BC-8.30.002 — depends on (facade mode that triggers this execution)
- BC-8.29.003 — depends on (Option B mutation obligation feeds the same execution)

## Architecture Anchors

- `plugins/vsdd-factory/skills/wave-gate/SKILL.md` — mutation testing step implementation

## Story Anchor

S-7.03

## VP Anchors

- VP-064 — facade-mode mutation gate enforcement
