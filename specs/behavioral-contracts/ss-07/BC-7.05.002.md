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
bc_id: BC-7.05.002
section: "7.05"
---

# BC-7.05.002: validate-count-propagation.sh runs in under 200ms and is deterministic

## Description

The `validate-count-propagation.sh` hook must complete in under 200ms on a typical corpus (up to ~2,000 files in `.factory/`) to avoid adding perceptible latency to Write/Edit operations. The hook must also be deterministic: identical file contents always produce identical exit codes and stderr output, with no dependence on external network calls, clocks, or random sources.

## Preconditions

1. The hook is invoked in a repository with up to ~2,000 files in `.factory/` (typical production corpus size).
2. The host system is a macOS or Linux machine with POSIX grep available.
3. The corpus files are on local disk (not a network filesystem).

## Postconditions

1. The hook completes in under 200ms wall time in the common case.
2. Given the same file contents as input, the hook produces the same exit code and same stderr output every time it runs.
3. The hook does not call any network endpoint, external binary (other than `grep`, `awk`, `sed`, `cat`, standard POSIX), or time-dependent function.

## Invariants

1. No network calls. No external API calls.
2. No clock-dependent logic (no `date`, `sleep`, or timeout-sensitive branches in the happy path).
3. File reads are bounded by the defined corpus scope — the hook does not recursively walk the entire git repository.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Repository has >10,000 files in `.factory/` (much larger than typical) | Hook may exceed 200ms. The scope list is explicit (7 known index files + SS-NN shards), not a recursive walk. Explicit list keeps runtime bounded even on large repos. |
| EC-002 | One of the target files is missing (e.g., VP-INDEX.md does not exist yet) | Hook skips missing files gracefully. A missing file is not treated as drift (not an error condition). |
| EC-003 | Two concurrent hook invocations for the same Write event | Each invocation reads its own snapshot of the files; both produce consistent outputs. No shared mutable state. |

## Canonical Test Vectors

| Input State | Expected Behavior | Category |
|-------------|-------------------|----------|
| Full fixture with 7 index files; count consistent across all | Exit 0 in <200ms | happy-path |
| Full fixture with 7 index files; 2 have conflicting counts | Exit 2 in <200ms, structured stderr | negative |
| Hook run twice on identical input | Identical exit code and stderr output | determinism check |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-062 | Hook exits correctly and runs in under 200ms | integration (test harness with timing assertion) |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-001 |
| Capability Anchor Justification | Anchored to CAP-001 ("Run a self-orchestrating LLM-driven SDLC pipeline") per capabilities.md §CAP-001 — a slow or non-deterministic hook would degrade the pipeline's tool-call overhead, undermining the CAP-001 promise of autonomous operation without manual agent handoffs. |
| L2 Domain Invariants | none |
| Architecture Module | plugins/vsdd-factory/hooks/validate-count-propagation.sh |
| Stories | S-7.02 |
| Source AC | S-7.02 §AC-002 |
| FR | FR-042 |

## Related BCs

- BC-7.05.001 — depends on (this BC refines performance and determinism properties of the hook defined there)

## Architecture Anchors

- `plugins/vsdd-factory/hooks/validate-count-propagation.sh` — hook script

## Story Anchor

S-7.02

## VP Anchors

- VP-062 — count propagation hook correctness (includes timing)
