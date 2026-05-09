---
document_type: behavioral-contract
level: L3
version: "1.1"
status: draft
producer: product-owner
timestamp: 2026-05-06T00:00:00Z
phase: 1a
inputs:
  - .factory/cycles/v1.0-feature-engine-discipline-pass-1/F1-delta-analysis.md
  - .factory/cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md
input-hash: "40a6fb6"
traces_to: .factory/cycles/v1.0-feature-engine-discipline-pass-1/F1-delta-analysis.md
origin: greenfield
subsystem: "SS-06"
capability: "CAP-018"
lifecycle_status: active
introduced: v1.0-feature-engine-discipline-pass-1
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
bc_id: BC-6.22.001
section: "6.22"
---

# BC-6.22.001: relocate-artifact skill MUST scan .factory/ for registry violations, propose canonical destinations, perform git mv, and update cross-references; MUST run to zero violations before validate-artifact-path hook is registered

## Description

The `/vsdd-factory:relocate-artifact` skill scans `.factory/` for artifact files whose
current paths do not match the canonical pattern registered in
`plugins/vsdd-factory/config/artifact-path-registry.yaml`. For each misplaced artifact,
the skill proposes a canonical destination by reading the artifact's frontmatter
(`document_type`, `bc_id`, `subsystem`, etc.). Default mode is dry-run (no filesystem
changes). The `--apply` flag executes `git mv` and updates cross-references in other files.
The skill MUST complete successfully with zero remaining violations before the
`validate-artifact-path` WASM hook is registered in `hooks-registry.toml`. This
sequencing is a hard delivery prerequisite per OQ5.

## Preconditions

1. `plugins/vsdd-factory/config/artifact-path-registry.yaml` exists and is readable.
   The skill reads the registry via the `Read` tool — never embeds a duplicate path list.
2. `.factory/` exists and is walkable (the skill uses the `Read` tool on directory listings).
3. In `--apply` mode: the story worktree has `git` available and the working tree is clean
   (no uncommitted changes that would interfere with `git mv`).
4. In `--apply` mode: cross-reference update requires that all referencing files are identified
   before any move is executed (detect first, then apply).
5. The skill MUST be invoked before `validate-artifact-path` is added to `hooks-registry.toml`.

## Postconditions

### Dry-run mode (default — no `--apply` flag)

1. The skill walks `.factory/` and, for each `.md` file:
   a. Reads frontmatter to extract `document_type` and type-specific ID fields (`bc_id`, `subsystem`, etc.).
   b. Looks up the canonical path pattern for `document_type` in the registry.
   c. If the current path does not match the canonical pattern: records a violation.
2. The skill emits a Markdown table to stdout listing all violations:
   ```
   | Current Path | Proposed Canonical Path | Artifact Type | Frontmatter Fields Used |
   ```
3. The skill emits a summary line: `"X violations found. Re-run with --apply to execute."` or
   `"0 violations found. Registry is clean."` when no violations exist.
4. The skill makes NO filesystem changes in dry-run mode.
5. No `git mv` is executed. No cross-reference files are modified.

### Apply mode (`--apply` flag)

6. For each violation identified in the detect phase:
   a. The skill executes `git mv <current-path> <canonical-path>`.
   b. After all moves complete, the skill scans all `.md` files under `.factory/` for
      references to the old path (by path string or by ID slug) and updates them in-place.
   c. The skill appends a move summary to the active cycle's `decision-log.md`:
      `"D-NNN (auto-relocation): git mv <old> → <new> (artifact type: <type>; trigger: relocate-artifact --apply)"`
7. If a cross-reference cannot be resolved (the referencing file cannot be determined):
   the skill REFUSES to move the artifact unless `--force-references-broken` is also passed.
8. After all moves, the skill re-runs the violation scan and emits:
   `"0 violations remaining. Registry is clean. validate-artifact-path hook may now be registered."`
   If violations remain, the skill exits with a non-zero status and lists the remaining violations.
9. The skill reports the number of files moved and cross-references updated.

### Sequencing gate

10. The skill's successful zero-violation run (in `--apply` mode or verified by a subsequent
    dry-run showing 0 violations) is the MANDATORY prerequisite before the
    `validate-artifact-path` hook is added to `hooks-registry.toml`. Story C's acceptance
    criteria MUST verify this sequencing before the hook registration commit is made.

## Invariants

1. The skill NEVER embeds a path list. It reads `plugins/vsdd-factory/config/artifact-path-registry.yaml`
   via `Read` tool on every invocation. This is the single-source-of-truth invariant (shared
   with BC-4.11.001 and the 9 creation skills).
2. In dry-run mode, the skill is fully idempotent and safe to run any number of times.
3. In `--apply` mode, the skill MUST detect all violations BEFORE executing any `git mv`.
   Partial moves (move some artifacts, fail on others) leave the repository in an
   inconsistent state and are prohibited. If detection fails for any artifact, the entire
   apply operation aborts.
4. `git mv` is the ONLY mechanism for moving files. Direct file copy + delete is prohibited
   because it breaks `git log --follow` for the moved artifact.
5. The `--force-references-broken` flag bypasses the cross-reference resolution safety gate.
   Its use must be justified and logged in the decision log.
6. The skill does NOT add entries to `artifact-path-registry.yaml`. It only reads the registry.
   Adding new artifact types requires `register-artifact` or manual registry edit.
7. The skill's zero-violation exit condition is the ONLY acceptable prerequisite for hook
   registration. A non-zero violation count from the skill is a hard block on Story C delivery.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Zero violations found (repository already clean) | Dry-run emits `"0 violations found. Registry is clean."` Apply mode skips all moves. Exits 0. |
| EC-002 | Artifact has no `document_type` frontmatter field | Skill emits a warning: `"Cannot classify <path> — document_type field absent. Skipping."` File is not moved. |
| EC-003 | Proposed canonical path already exists (naming collision) | Skill REFUSES to move and reports the collision. User must manually resolve before re-running. |
| EC-004 | Cross-reference found in a file that is itself being moved | The skill resolves the transitive reference graph before executing any move. Moves are ordered to resolve dependencies. |
| EC-005 | `git mv` fails (e.g., target directory does not exist) | Skill creates the directory with `mkdir -p` equivalent (Read + Write pattern), then retries `git mv`. If still fails, logs the error and aborts the apply operation. |
| EC-006 | `--apply` invoked with uncommitted changes in working tree | Skill warns: `"Working tree has uncommitted changes. Run git status to verify before proceeding."` Proceeds only if `--force-dirty-tree` is also passed. |
| EC-007 | Registry pattern uses a placeholder (`{bc-id}`) that cannot be resolved from frontmatter | Skill emits: `"Cannot resolve canonical path for <path> — frontmatter field '<field>' required by pattern '{placeholder}' is absent. Skipping."` |

## Canonical Test Vectors

| Input State | Mode | Expected Output |
|-------------|------|----------------|
| 0 misplaced artifacts | dry-run | `"0 violations found. Registry is clean."` |
| 3 misplaced BCs | dry-run | Table of 3 proposed moves; no filesystem changes |
| 3 misplaced BCs | --apply | 3 `git mv` calls; cross-refs updated; decision-log appended; 0 violations remaining |
| Artifact missing document_type | dry-run | Warning emitted; file skipped |
| Proposed canonical path collision | --apply | Error emitted; move aborted; exit 1 |
| Unresolvable cross-reference | --apply (without --force) | Error: "cross-reference unresolvable; use --force-references-broken"; exit 1 |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-072 | Skill reads registry at runtime (no embedded path list) | static analysis: grep skill source for hardcoded `.factory/` path patterns; assert zero hits |
| (bats-test) | dry-run produces correct violation table | bats integration test: fixture with known misplaced artifacts |
| (bats-test) | --apply executes git mv and updates cross-references correctly | bats integration test: pre-move fixture; post-apply verification |
| (bats-test) | --force-references-broken bypasses safety gate | bats integration test: unresolvable reference fixture |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-018 |
| Capability Anchor Justification | CAP-018 ("Validate spec consistency across all artifact layers") per capabilities.md §CAP-018 — this BC governs the `relocate-artifact` skill, which detects and repairs artifact path inconsistencies between the actual `.factory/` directory structure and the canonical patterns registered in `artifact-path-registry.yaml`. CAP-018 defines the capability as cross-checking that entity definitions, BC identifiers, and artifact paths are consistent across all layers. The `relocate-artifact` skill is the enforcement tool for the path-consistency layer of that cross-check: it finds artifacts that are in the wrong location (inconsistent with the registry) and moves them to the canonical location. |
| Secondary Capability Reference | CAP-001 ("Run a self-orchestrating LLM-driven SDLC pipeline") per capabilities.md §CAP-001 — the skill is invoked by the orchestrator as part of the Story C delivery sequence, making it part of the pipeline orchestration surface. CAP-018 is the primary anchor because path consistency is the core capability being served. |
| L2 Domain Invariants | none |
| Architecture Module | plugins/vsdd-factory/skills/relocate-artifact/SKILL.md (new skill); plugins/vsdd-factory/config/artifact-path-registry.yaml (registry read source) |
| Stories | S-13.01 |
| FR | FR-047 (per-story adversarial convergence + artifact path discipline — to be added in PRD delta) |

## Related BCs

- BC-4.11.001 — composes with (validate-artifact-path hook; this skill MUST run to zero violations before that hook is registered; hard sequencing dependency)
- BC-6.20.001 — sibling (create-adr skill; one of the 9 creation skills that must add registry-read step per BC-4.11.001 Postcondition 8)

## Architecture Anchors

- `plugins/vsdd-factory/skills/relocate-artifact/SKILL.md` — new skill file (authored in Story C delivery)
- `plugins/vsdd-factory/config/artifact-path-registry.yaml` — registry read source (single source of truth)
- `hooks-registry.toml` — the validate-artifact-path hook registration MUST NOT occur until this skill reports 0 violations

## Story Anchor

Story C — v1.0-feature-engine-discipline-pass-1 (F3 story decomposition)

## VP Anchors

- VP-072 — single-source-of-truth invariant (cross-cutting; shared with BC-4.11.001)

## Changelog

| Version | Date | Description |
|---------|------|-------------|
| 1.0 | 2026-05-06 | Initial authoring (product-owner; F2 phase of v1.0-feature-engine-discipline-pass-1). OQ5 resolution applied: immediate block mode confirmed; relocate-artifact must run before hook registration (hard sequencing prerequisite). detect-then-apply mode specified. |
| 1.1 | 2026-05-09 | F-P45-001 — Traceability Stories row propagated: "Story C" placeholder → S-13.01. S-13.01 merged PR #97 cites BC-6.22.001 in behavioral_contracts frontmatter; BC-INDEX carried TBD (fixed bidirectionally in this burst). Refs: F-P45-001, fix-burst-42. |
