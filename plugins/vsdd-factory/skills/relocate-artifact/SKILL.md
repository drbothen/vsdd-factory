---
name: relocate-artifact
description: Scan .factory/ for artifact files whose paths do not match the canonical patterns in artifact-path-registry.yaml. Default mode is dry-run (table of violations, no filesystem changes). Pass --apply to execute git mv and update cross-references.
argument-hint: "[--apply] [--force-dirty-tree] [--force-references-broken]"

allowed-tools: Read, Bash
---

## Purpose

> TODO: S-13.01 Step 4 implementer fills this in per BC-6.22.001 PC[1].
>
> Describe: detect misplaced .factory/ artifacts by comparing each file's
> document_type frontmatter against registry canonical_path_pattern entries.
> Propose canonical destinations. Optionally execute git mv + cross-reference
> repair + decision-log append.

## Usage

> TODO: S-13.01 Step 4 implementer fills this in per BC-6.22.001 PC[2-5].
>
> Document dry-run and --apply invocation forms. Include example output
> (violation table format, summary line formats). Document --force-dirty-tree
> and --force-references-broken flags and their safety implications.

## Detection Phase

> TODO: S-13.01 Step 4 implementer fills this in per BC-6.22.001 PC[1a-1c].
>
> Describe: walk .factory/ using Read tool; for each .md file read frontmatter;
> extract document_type and type-specific ID fields (bc_id, subsystem, etc.);
> look up canonical_path_pattern in registry; classify as violation or clean.
> Handle EC-002 (no document_type) and EC-007 (unresolvable placeholder).

## Diagnosis Phase

> TODO: S-13.01 Step 4 implementer fills this in per BC-6.22.001 PC[2].
>
> Describe: emit Markdown violation table to stdout with columns:
> Current Path | Proposed Canonical Path | Artifact Type | Frontmatter Fields Used.
> Emit summary line: "X violations found. Re-run with --apply to execute."
> or "0 violations found. Registry is clean."

## Relocation Phase

> TODO: S-13.01 Step 4 implementer fills this in per BC-6.22.001 PC[6a-6c].
>
> Describe --apply mode: detect ALL violations before executing any git mv
> (invariant 3 — atomicity). Execute git mv for each violation. Scan all .md
> files for cross-references to old paths and update in-place. Handle EC-003
> (collision), EC-004 (transitive cross-reference graph), EC-005 (git mv failure),
> EC-006 (dirty working tree). Append decision-log entry per PC[6c] format.

## Audit Logging

> TODO: S-13.01 Step 4 implementer fills this in per BC-6.22.001 PC[6c].
>
> Describe: after all moves complete, append to active cycle's decision-log.md:
> "D-NNN (auto-relocation): git mv <old> -> <new> (artifact type: <type>;
> trigger: relocate-artifact --apply)" for each moved file.
> Report count of files moved and cross-references updated.

## Safety Constraints

> TODO: S-13.01 Step 4 implementer fills this in per BC-6.22.001 invariants 1-7.
>
> Enumerate: (1) never embed path list — read registry on every invocation;
> (2) dry-run is idempotent; (3) detect-before-apply atomicity;
> (4) git mv only (no copy+delete); (5) --force-references-broken bypass;
> (6) skill does NOT add entries to registry; (7) zero-violation exit is the
> ONLY acceptable prerequisite for validate-artifact-path hook registration.
