---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: product-owner
timestamp: 2026-04-25T00:00:00
phase: 1a
inputs: [.factory/stories/S-7.02-defensive-sweep-hook-meta-rule.md]
input-hash: ""
traces_to: .factory/stories/S-7.02-defensive-sweep-hook-meta-rule.md
origin: greenfield
extracted_from: ".factory/stories/S-7.02-defensive-sweep-hook-meta-rule.md#AC-002"
subsystem: "SS-07"
capability: "CAP-001"
lifecycle_status: active
introduced: v1.0-brownfield-backfill
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
bc_id: BC-7.05.001
section: "7.05"
---

# BC-7.05.001: validate-count-propagation.sh detects count drift across index files and exits non-zero

## Description

The new Bash hook `validate-count-propagation.sh` must detect when a count value (BC count, VP count, story count) appears at different numeric values in sibling index documents. On detecting any such drift, the hook emits a structured warning to stderr and exits non-zero. The hook is triggered as a PostToolUse hook on Write/Edit to the key index files.

## Preconditions

1. The hook is invoked as a PostToolUse event after a Write or Edit operation on: `STATE.md`, `ARCH-INDEX.md`, `BC-INDEX.md`, `VP-INDEX.md`, `STORY-INDEX.md`, any `SS-NN-*.md` sharded architecture file, or `prd.md`.
2. The hook script exists at `plugins/vsdd-factory/hooks/validate-count-propagation.sh` and is executable.
3. The hook uses `set -euo pipefail`.

## Postconditions

1. If the same count keyword pair (e.g., "BCs") appears at different numeric values in two or more sibling files, the hook exits non-zero (exit code 2 per PostToolUse convention) and emits a structured warning to stderr:
   ```
   COUNT DRIFT DETECTED: 'NNN BCs' appears as NNN in FILE-A but MMM in FILE-B.
   Run: grep -r "BCs" .factory/specs/ .factory/STATE.md to reconcile.
   ```
2. If all count keyword pairs are consistent across checked files (or a count is absent from siblings), the hook exits 0.
3. If the modified file contains no count-bearing patterns, the hook exits 0 immediately.

## Invariants

1. Count extraction uses anchored context patterns: `\bNNN BCs\b`, `\bNNN VPs\b`, `\bNNN stories\b`, `Total.*NNN` — not bare number grep.
2. Absence of a count in a sibling file is NOT drift (exit 0). Only conflicting values in two or more files that DO contain the count trigger the warning.
3. The hook is read-only: it never writes to any file. It emits to stderr and sets exit code only.
4. Hook runtime must be under 200ms on typical corpus sizes (enforced by NFR).

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | STATE.md says "1,875 BCs", ARCH-INDEX.md says "1,863 BCs" | Hook detects drift; exits non-zero with structured warning naming both files and values |
| EC-002 | STATE.md says "1,875 BCs"; ARCH-INDEX.md does not mention BCs | Absence is not drift; hook exits 0 |
| EC-003 | Hook fires during a mid-burst write where counts are intentionally transitional | Hook may report false-positive drift. The hook's value is post-burst catch. If too noisy, `SKIP_COUNT_PROPAGATION=1` env var suppresses the check without removing it. |
| EC-004 | Count string appears in a code comment or changelog without semantic meaning | Anchored pattern should avoid false matches. If a false positive is reported, the regex should be tightened to exclude the specific context. |
| EC-005 | A count uses comma-formatted numbers ("1,875") vs unformatted ("1875") in different files | Normalization step: strip commas before comparing numeric values. Both "1,875 BCs" and "1875 BCs" resolve to 1875. |

## Canonical Test Vectors

| Input State | Expected Exit Code | Expected Stderr | Category |
|-------------|-------------------|-----------------|----------|
| STATE.md: "1,875 BCs"; ARCH-INDEX.md: "1,863 BCs" | 2 | `COUNT DRIFT DETECTED: 'BCs' appears as 1875 in STATE.md but 1863 in ARCH-INDEX.md` | negative (drift detected) |
| STATE.md: "1,875 BCs"; ARCH-INDEX.md: "1,875 BCs"; BC-INDEX.md: "1,875 BCs" | 0 | (empty) | happy-path |
| STATE.md: "1,875 BCs"; ARCH-INDEX.md: no BC count mention | 0 | (empty) | edge-case (absence not drift) |
| File modified has no count-bearing lines | 0 | (empty) | edge-case |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-062 | validate-count-propagation.sh correctly flags drift and exits non-zero | integration (test harness with fixture files) |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-001 |
| Capability Anchor Justification | Anchored to CAP-001 ("Run a self-orchestrating LLM-driven SDLC pipeline") per capabilities.md §CAP-001 — this hook is a pipeline enforcement tool that prevents count inconsistency in the SDLC pipeline's artifact indexes, which is a structural property of the self-orchestrating pipeline. |
| L2 Domain Invariants | none |
| Architecture Module | plugins/vsdd-factory/hooks/validate-count-propagation.sh |
| Stories | S-7.02 |
| Source AC | S-7.02 §AC-002 |
| FR | FR-042 |

## Related BCs

- BC-5.37.001 — composes with (state-manager manual sweep; this hook automates the same check)
- BC-7.05.002 — sibling (exit code and diagnostic format specification)
- BC-9.02.001 — sibling (hook registration in hooks-registry.toml)

## Architecture Anchors

- `plugins/vsdd-factory/hooks/validate-count-propagation.sh` — hook script to be created
- `plugins/vsdd-factory/hooks-registry.toml` — hook registration

## Story Anchor

S-7.02

## VP Anchors

- VP-062 — count propagation consistency invariant
