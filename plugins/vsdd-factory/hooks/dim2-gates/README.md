# hooks/dim2-gates/ — Dim-2 Gate Template Registry

**Purpose:** Canonical bash template storage for Dim-2 mechanical gate scripts, as prescribed by D-453(e). Each file in this directory is a reusable shell template invoked (or adapted) during fix-burst Dim-2 attestation.

**Codification authority:** D-453(e) (pass-73 fix burst, Commit B). Instantiation of this directory closes ADV-EDP1-P74-HIGH-002 (codification-references-storage-that-doesn't-exist pattern; remediated at pass-74 Commit A).

**Source vs deployment:** This directory is the SOURCE location for dim2-gate templates within the vsdd-factory plugin source tree. The deployment target per D-453(e) is `.factory/hooks/dim2-gates/` within each factory project worktree. The deployment target path is registered in `config/artifact-path-registry.yaml` (entry: `hooks-dim2-gate-template`); registration was added at pass-74 Commit A. Templates here ship to the deployment target via plugin release.

## Planned Template Files

| File | Gate | Closes / Implements |
|------|------|---------------------|
| `trajectory-tail-cell-grep.sh` | Per-cell trajectory_tail presence check (D-454(a)) | ADV-EDP1-P74-CRIT-001 |
| `freshness-literal-stdout.sh` | Freshness re-execution with captured stdout (D-454(b)) | ADV-EDP1-P74-HIGH-001 + MED-003 |
| `block-label-canonical-form.sh` | Tri-way block-label canonical-form alignment check (D-454(d)) | ADV-EDP1-P74-HIGH-003 |
| `banner-wc-l-gate.sh` | STATE.md banner wc-l freshness gate with full-edit-window scope (D-454(e)) | ADV-EDP1-P74-HIGH-004 |
| `propagation-count-gate.sh` | Propagation count gate scoped to post-edit corpus state | ADV-EDP1-P74-PG-003 |

## Template Creation Cadence

Templates are instantiated in the fix burst that codifies their corresponding D-NNN decision. The D-453(e) pattern prescribes: **codify AND instantiate in the same burst** — no deferred creation. D-454(c) codifies this as a hard rule: `storage-path-without-artifacts` is forbidden; every D-NNN that names a storage path MUST create at least a stub artifact at the same commit.

## Usage

Each template is a standalone bash script conforming to:
- `set -euo pipefail`
- Single positional argument: the target `.factory/` root path
- Exit 0 on PASS, exit 1 on FAIL with human-readable reason on stdout
- Designed to be sourced or invoked from burst-log Dim-2 attestation blocks per D-449(a)

## Registry Status

| Status | Meaning |
|--------|---------|
| `PLANNED` | D-NNN specifies this gate; template not yet written |
| `STUB` | Directory and README created; script body pending |
| `ACTIVE` | Fully implemented and in use at Dim-2 attestation |

All entries in the Planned Template Files table above are currently `PLANNED` pending D-454 codification at Commit B.
