---
name: factory-cycles-bootstrap
description: Migrate from flat adversarial-review layout to cycle-keyed directory structure. Archives historical reviews, creates cycle directories, and sets the current-cycle pointer.
argument-hint: "[cycle-name]"
---

# Factory Cycles Bootstrap

Migrate a project's `.factory/` from the flat layout (all reviews in `specs/`) to the cycle-keyed layout (reviews organized by convergence cycle).

## When to Use

- First time running adversarial review on a project that has historical `specs/adversarial-review-pass-*.md` files
- Starting a new convergence cycle (e.g., moving from Phase 1 convergence to Phase 3 patch cycle)
- When the adversarial-review skill's collision guard warns about legacy flat files

## Procedure

### Step 1: Determine cycle name

If `$ARGUMENTS` is provided, use it as the cycle name. Otherwise prompt:

> "What should this convergence cycle be named? Examples: `v1.0.0-greenfield`, `phase-3-patch`, `v1.1.0-feature-auth`"

Cycle names must be lowercase, hyphenated, no spaces.

### Step 2: Scan for existing reviews

Check for historical adversarial review files:

```
.factory/specs/adversarial-review-pass-*.md
.factory/specs/adversarial-review-*.md
```

If found, report:
> "Found N existing review files in .factory/specs/. These are from a prior convergence cycle. I'll archive them under a cycle directory."

Ask the user to name the historical cycle (default: `phase-1-convergence`).

### Step 3: Create cycle directory structure

For each cycle (historical + new):

```
.factory/cycles/<cycle-name>/
  adversarial-reviews/
    pass-1.md
    pass-2.md
    ...
  INDEX.md
```

### Step 4: Archive historical reviews

If historical files exist:

1. Create the historical cycle directory
2. Move each file using `git mv` (preserves history):
   - `specs/adversarial-review-pass-1.md` → `cycles/<historical-cycle>/adversarial-reviews/pass-1.md`
   - Renumber if naming is inconsistent
3. Create `INDEX.md` in the historical cycle directory listing all archived passes

### Step 5: Set current-cycle pointer

Write `.factory/current-cycle` with the new cycle name (single line, no trailing newline):

```
<cycle-name>
```

This file is read by the adversarial-review skill to determine the output path.

### Step 6: Create the new cycle directory

```
.factory/cycles/<new-cycle-name>/
  adversarial-reviews/
  INDEX.md
```

INDEX.md content:

```markdown
# Cycle: <cycle-name>

## Adversarial Reviews

| Pass | Date | Findings | Status |
|------|------|----------|--------|
```

### Step 7: Report

Emit a summary:

```
Cycle bootstrap complete:
- Historical reviews: N files archived to cycles/<historical-cycle>/
- New cycle: cycles/<new-cycle-name>/ created
- Current cycle pointer: .factory/current-cycle → <new-cycle-name>
- Adversarial-review skill will now write to: .factory/cycles/<new-cycle-name>/adversarial-reviews/pass-<N>.md
```

## Integration

After bootstrapping, the adversarial-review skill reads `.factory/current-cycle` to determine the output directory. The collision guard checks for existing files in the cycle directory before writing.

## Out of Scope

- Does NOT modify or re-number existing finding IDs (that's P1.2 — finding-ID cycle prefix)
- Does NOT run adversarial review — it only sets up the directory structure
- Does NOT modify STATE.md — dispatch state-manager for that
