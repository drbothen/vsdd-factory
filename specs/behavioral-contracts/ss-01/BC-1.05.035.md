---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: product-owner
timestamp: 2026-05-03T00:00:00Z
last_amended: 2026-05-05
phase: 1.4b
inputs: [gap-analysis-w16-subprocess.md]
input-hash: "[pending-recompute]"
traces_to: gap-analysis-w16-subprocess.md
origin: brownfield
extracted_from: ".factory/architecture/gap-analysis-w16-subprocess.md:Section 5"
subsystem: "SS-01"
capability: "CAP-TBD"
lifecycle_status: active
introduced: v1.0.0
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# Behavioral Contract BC-1.05.035: factory-dispatcher::host::exec_subprocess::canonicalizes_binary_path_before_allow_check — Path::canonicalize() applied before binary_allow match; symlink-based traversal rejected

## Description

**ADR-015 Awareness (added per E-9 v1.7 Post-Audit Amendment, propagated to BC at v1.22 per TD-VSDD-074):**
This BC's denial-path postcondition references the existing `internal.capability_denied` event name. Per ADR-015 D-15.2 reverse-DNS naming requirement and gap-analysis-w16-subprocess.md §"How ADR-015 affects the telemetry gap" lines 339-349, this event MUST be renamed to `vsdd.capability.denied.exec_subprocess.v1` to map to the `audit` category per ADR-015 D-15.2 registry line 329 (`vsdd.capability.denied.* | audit`). The current name is INTERIM. The S-9.07 implementer (or the SS-01 implementer of any host-emit-fix story) MUST rename `internal.capability_denied` → `vsdd.capability.denied.exec_subprocess.v1` before merge. No new OQ filed; rename target is unambiguous per gap-analysis amendment.

The dispatcher MUST call `Path::new(cmd).canonicalize()` on the binary path BEFORE the binary_allow capability check. Canonicalization resolves symlinks and eliminates `..` segments. NUL-byte rejection is performed earlier by the existing `read_wasm_string` error path (see §Postcondition 2 and the Precedence Ladder). This closes a defense-in-depth gap identified in gap-analysis-w16-subprocess.md Section 5: `exec_subprocess.rs:230` currently passes `cmd` directly to `Command::new` with no traversal check.

## Preconditions

1. Plugin calls `vsdd::exec_subprocess`.
2. Plugin has a valid `Capabilities.exec_subprocess` block.
3. A `cmd` argument is provided.

## Postconditions

1. If `cmd` passes string-level validation (`../` absent, no NUL bytes) AND `Path::new(cmd).canonicalize()` succeeds AND the canonicalized path does not contain `..` segments, the allow-list check proceeds normally.
2. If `cmd` contains a NUL byte or fails basic string validation, returns `codes::INVALID_ARGUMENT` (-4) — existing `read_wasm_string` error path.
3. If `Path::new(cmd).canonicalize()` fails (binary not on disk, path invalid, IO error), returns `codes::CAPABILITY_DENIED` (-1) — existing exec_subprocess error semantics preserved.
4. If the canonicalized path contains `..` components after resolution (symlink-based escape attempt), returns `codes::INVALID_ARGUMENT` (-4) and the existing `internal.capability_denied` event is emitted. (event name `internal.capability_denied` is INTERIM — see §Description ADR-015 awareness clause; rename to `vsdd.capability.denied.exec_subprocess.v1` per ADR-015 D-15.2 registry line 329)

**Precedence ladder (per pass-22 M-P22-003):** When multiple validation conditions could fire, the host applies them in this order: (1) NUL byte in `cmd` → `Err(INVALID_ARGUMENT -4)`; (2) `Path::new(cmd).canonicalize()` returns Err → `Err(CAPABILITY_DENIED -1)` (path doesn't exist or symlink loop); (3) canonicalized path contains `..` segments (rare; only with non-existent intermediates) → `Err(INVALID_ARGUMENT -4)`; (4) canonicalized path not in `binary_allow` list → `Err(CAPABILITY_DENIED -1)`. Per `crates/factory-dispatcher/src/host/exec_subprocess.rs:230` (entry point) and BC-1.05.005/BC-1.05.032 sibling contracts.

## Invariants

1. Path canonicalization MUST precede the binary_allow list match on every exec_subprocess invocation.

## Related BCs

- BC-1.05.001 — exec_subprocess capability check (depends on: this BC extends the pre-check chain)
- BC-1.05.002 — binary allow-list enforcement (composes with: this BC adds canonicalization before the allow-list match)
- BC-1.05.003 — shell bypass gate (sibling pre-check in the same gate chain)
- BC-1.05.036 — success-path telemetry (sibling extension from same gap analysis)

## Architecture Anchors

- `crates/factory-dispatcher/src/host/exec_subprocess.rs:230` — current `Command::new(cmd)` call site; canonicalize-before-check step added here
- `.factory/architecture/gap-analysis-w16-subprocess.md` Section 5 — authority for this extension

## Story Anchor

S-9.07 (validate-wave-gate-prerequisite WASM port) — implementation task

## VP Anchors

(TBD — to be assigned in Phase 1.6b)

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | `cmd` contains `../etc/passwd` literal | Returns `CAPABILITY_DENIED` (-1); allow-list match never reached |
| EC-002 | `cmd` is a symlink pointing outside CLAUDE_PROJECT_DIR | `canonicalize()` resolves it; `..` components detected; returns `INVALID_ARGUMENT` (-4) |
| EC-003 | `cmd` is a valid absolute path with no traversal | `canonicalize()` succeeds; allow-list check proceeds normally |
| EC-004 | `cmd` binary does not exist on disk | `canonicalize()` fails with IO error; returns `CAPABILITY_DENIED` (-1) |
| EC-005 | `cmd` contains NUL byte | Returns `INVALID_ARGUMENT` (-4) via existing `read_wasm_string` error path before canonicalize |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `cmd = "../etc/passwd"` | `CAPABILITY_DENIED` (-1) | error |
| `cmd = "/usr/bin/bash"` (exists, no traversal) | Proceeds to allow-list check | happy-path |
| Symlink `cmd` resolving outside project dir (resolves to `../escape`) | `INVALID_ARGUMENT` (-4) | edge-case |
| Non-existent binary path | `CAPABILITY_DENIED` (-1) | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6b) | | |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD (anchor in Phase 1.5) |
| L2 Domain Invariants | TBD |
| Architecture Module | SS-01 — `crates/factory-dispatcher/src/host/exec_subprocess.rs` |
| Stories | S-9.07 |
| Capability Anchor Justification | CAP-TBD — capability anchor to be confirmed in Phase 1.5; this BC governs exec_subprocess security hardening in factory-dispatcher |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `.factory/architecture/gap-analysis-w16-subprocess.md` Section 5 item 1; `crates/factory-dispatcher/src/host/exec_subprocess.rs:230` |
| **Confidence** | HIGH (gap explicitly identified by architect gap analysis) |
| **Extraction Date** | 2026-05-03 |
| **Extracted from** | `.factory/architecture/gap-analysis-w16-subprocess.md` Section 5 |

#### Evidence Types Used

- gap-analysis (architect-identified missing guard at exec_subprocess.rs:230)
- inferred (analogous to BC-2.02.013 I-2.6 which specified the same traversal check for the withdrawn run_subprocess path)

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | YES — `Path::canonicalize()` performs filesystem stat; failure is handled deterministically |
| **Global state access** | No |
| **Deterministic** | YES given a fixed filesystem state |
| **Thread safety** | YES — read-only filesystem stat |
| **Overall classification** | Deterministic with external I/O dependency |

#### Refactoring Notes

(TBD — to be assessed in Phase 1.6b verification properties pass)
