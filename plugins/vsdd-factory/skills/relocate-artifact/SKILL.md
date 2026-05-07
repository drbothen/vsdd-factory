---
name: relocate-artifact
description: Scan .factory/ for artifact files whose paths do not match the canonical patterns in artifact-path-registry.yaml. Default mode is dry-run (table of violations, no filesystem changes). Pass --apply to execute git mv and update cross-references.
argument-hint: "[--apply] [--force-dirty-tree] [--force-references-broken]"

allowed-tools: Read, Bash
---

## Purpose

Scan `.factory/` for artifact files whose current paths do not match the canonical
patterns registered in `plugins/vsdd-factory/config/artifact-path-registry.yaml`.
For each misplaced artifact, propose a canonical destination by reading the file's
frontmatter (`document_type`, `bc_id`, `subsystem`, etc.). Default mode is dry-run
(no filesystem changes). Pass `--apply` to execute `git mv` and update cross-references.

This skill MUST complete successfully with zero remaining violations before the
`validate-artifact-path` WASM hook is registered in `hooks-registry.toml`
(BC-6.22.001 invariant 7; BC-4.11.001 precondition 5).

## Path Resolution (Mandatory)

Before writing any artifact, resolve the canonical path via
`plugins/vsdd-factory/config/artifact-path-registry.yaml`. Do not invent paths.

This skill never embeds a path list. It reads `plugins/vsdd-factory/config/artifact-path-registry.yaml`
via the `Read` tool on every invocation. This is the single-source-of-truth invariant
(BC-6.22.001 invariant 1; BC-4.11.001 invariant 1).

## Usage

```
# Dry-run (default — no filesystem changes):
/vsdd-factory:relocate-artifact

# Apply mode (executes git mv + cross-reference repair + decision-log append):
/vsdd-factory:relocate-artifact --apply

# Apply mode with dirty working tree (use with caution):
/vsdd-factory:relocate-artifact --apply --force-dirty-tree

# Apply mode bypassing cross-reference safety gate (use with caution — log justification):
/vsdd-factory:relocate-artifact --apply --force-references-broken
```

### Example dry-run output

```
Scanning .factory/ for registry violations...

| Current Path | Proposed Canonical Path | Artifact Type | Frontmatter Fields Used |
|---|---|---|---|
| .factory/WRONG-LOCATION/BC-4.11.001.md | .factory/specs/behavioral-contracts/ss-04/BC-4.11.001.md | behavioral-contract | bc_id=BC-4.11.001, subsystem=SS-04 |

1 violations found. Re-run with --apply to execute.
```

### Example clean run output

```
Scanning .factory/ for registry violations...

0 violations found. Registry is clean.
```

### Example apply output

```
Scanning .factory/ for registry violations...

Detected 1 violation(s). Beginning relocation...

git mv .factory/WRONG-LOCATION/BC-4.11.001.md .factory/specs/behavioral-contracts/ss-04/BC-4.11.001.md

Updating cross-references...
  Updated 2 reference(s) in .factory/specs/behavioral-contracts/BC-INDEX.md

Appending to decision-log...

1 file(s) moved. 2 cross-reference(s) updated.

0 violations remaining. Registry is clean. validate-artifact-path hook may now be registered.
```

## Detection Phase

### Step 1: Read the registry

```
Read: plugins/vsdd-factory/config/artifact-path-registry.yaml
```

Parse the YAML to extract all `artifact_type`, `canonical_path_pattern`, and `enforcement_level`
entries. Never embed a path list — always read from the registry file.

### Step 2: Walk .factory/

Use `Bash` to list all `.md` files recursively under `.factory/`:

```bash
find .factory -name "*.md" | sort
```

Also include non-`.md` runtime state files registered in the registry (`.json`, `.yaml`)
if they have registered entries:

```bash
find .factory -name "*.json" -o -name "*.yaml" | sort
```

### Step 3: For each file, read frontmatter

For each file found, extract the YAML frontmatter (between `---` delimiters). Key fields:
- `document_type`: maps to `artifact_type` in the registry
- `bc_id`: used to resolve `{bc-id}` placeholder
- `subsystem`: used to resolve `{subsystem}` placeholder
- `adr_id` or filename prefix: used to resolve `{adr-id}` placeholder
- `vp_id` or filename prefix: used to resolve `{vp-id}` placeholder
- `story_id` or filename: used to resolve `{story-id}` and `{slug}` placeholders

**EC-002:** If `document_type` is absent, emit a warning and skip the file:
```
WARNING: Cannot classify .factory/path/to/file.md — document_type field absent. Skipping.
```

**EC-007:** If the registry pattern requires a placeholder that cannot be resolved
from frontmatter, emit a warning and skip:
```
WARNING: Cannot resolve canonical path for .factory/path/to/file.md — frontmatter field
'<field>' required by pattern '{placeholder}' is absent. Skipping.
```

### Step 4: Compare current path to canonical pattern

For each classified file:
1. Look up the `canonical_path_pattern` for `document_type` in the registry.
2. Resolve all `{placeholder}` values from frontmatter fields and filename.
3. Check if the current path matches the resolved canonical path.
4. If not: record a violation with `(current_path, proposed_canonical_path, artifact_type, fields_used)`.

### Step 5: Emit violation table

Print a Markdown table to stdout:

```markdown
| Current Path | Proposed Canonical Path | Artifact Type | Frontmatter Fields Used |
|---|---|---|---|
| ... | ... | ... | ... |
```

Followed by summary line:
- `"X violations found. Re-run with --apply to execute."` (X > 0)
- `"0 violations found. Registry is clean."` (no violations)

In dry-run mode: **stop here**. Make NO filesystem changes.

## Relocation Phase (--apply only)

**Atomicity requirement (BC-6.22.001 invariant 3):** Detect ALL violations BEFORE
executing ANY `git mv`. If detection fails for any artifact (EC-007, unresolvable
placeholder), abort the entire apply operation with a non-zero exit. No moves are
performed for ANY artifact, even those that were successfully detected.

### Step 6: Pre-flight checks

**Dirty working tree check (EC-006):**
```bash
git status --porcelain
```

If output is non-empty and `--force-dirty-tree` is NOT passed:
```
ERROR: Working tree has uncommitted changes. Run git status to verify before proceeding.
Use --force-dirty-tree to bypass this check.
```
Exit non-zero.

**Canonical path collision check (EC-003):**
For each proposed canonical path, verify it does not already exist:
```bash
[ -e "proposed/canonical/path.md" ]
```

If any proposed path already exists:
```
ERROR: Proposed canonical path 'proposed/canonical/path.md' already exists.
Naming collision — cannot move '.factory/current/path.md'. Resolve manually.
```
Exit non-zero.

### Step 7: Scan cross-references

For each violation's current path, scan all `.md` files under `.factory/` for references
to the old path (by path string or by ID slug). Build a map of `(referencing_file, line_number, old_ref)`
for each found reference.

**Unresolvable reference check:**
If a reference is found in a file that cannot be updated (e.g., a binary file, a locked file,
or a transitive reference graph cycle), and `--force-references-broken` is NOT passed:
```
ERROR: cross-reference to '<old_path>' in '<referencing_file>' cannot be resolved automatically.
Use --force-references-broken to bypass this safety gate. Log justification in decision-log.
```
Exit non-zero.

### Step 8: Execute git mv (one per violation)

For each violation, create target directory if needed and execute:
```bash
mkdir -p "$(dirname 'proposed/canonical/path.md')"
git mv "current/path.md" "proposed/canonical/path.md"
```

`git mv` is the ONLY permitted move mechanism. Direct file copy + delete is PROHIBITED
because it breaks `git log --follow` for the moved artifact (BC-6.22.001 invariant 4).

**EC-005:** If `git mv` fails (e.g., target directory creation failed):
```
ERROR: git mv failed for '.factory/current/path.md' → 'proposed/canonical/path.md'.
Aborting apply operation.
```
Exit non-zero. Do NOT attempt further moves.

### Step 9: Update cross-references

After all moves complete, for each referencing file identified in Step 7:
- Replace occurrences of the old path string with the new canonical path string.
- Use `Bash` with `sed` or equivalent in-place replacement, OR use `Read` + `Edit`.
- Report the count of files and references updated.

### Step 10: Append to decision-log

Find the active cycle's `decision-log.md`:
```bash
# Read .factory/STATE.md to identify the active cycle ID, then:
ls .factory/cycles/<cycle-id>/decision-log.md
```

Append one entry per moved file:
```
D-NNN (auto-relocation): git mv <old_path> → <new_path> (artifact type: <type>; trigger: relocate-artifact --apply)
```

Where `NNN` is the next sequential decision number in that log.

### Step 11: Post-apply verification

Re-run the detection scan (Steps 2–4) to verify zero violations remain.

Emit:
```
0 violations remaining. Registry is clean. validate-artifact-path hook may now be registered.
```

If violations remain (e.g., due to a mid-apply interruption):
```
ERROR: <N> violation(s) remain after apply. See list above.
```
Exit non-zero.

Report total counts:
```
<N> file(s) moved. <M> cross-reference(s) updated.
```

## Safety Constraints

1. **Never embed path list:** This skill reads `plugins/vsdd-factory/config/artifact-path-registry.yaml`
   via `Read` on every invocation. It NEVER embeds a path list. (BC-6.22.001 invariant 1)

2. **Dry-run is idempotent:** Running the skill without `--apply` any number of times
   produces only stdout output. No filesystem changes occur. (BC-6.22.001 invariant 2)

3. **Detect-before-apply atomicity:** All violations are detected BEFORE any `git mv`
   executes. A detection failure aborts the entire apply. (BC-6.22.001 invariant 3)

4. **`git mv` only:** Direct file copy + delete is prohibited. (BC-6.22.001 invariant 4)

5. **`--force-references-broken` bypass:** Justification must be logged in the decision-log
   when this flag is used. (BC-6.22.001 invariant 5)

6. **Registry is read-only:** This skill does NOT add entries to `artifact-path-registry.yaml`.
   Use `/vsdd-factory:register-artifact` to add new artifact types. (BC-6.22.001 invariant 6)

7. **Zero-violation exit is the prerequisite:** The `validate-artifact-path` WASM hook
   MUST NOT be registered in `hooks-registry.toml` until this skill reports
   `"0 violations found. Registry is clean."` or
   `"0 violations remaining. Registry is clean. validate-artifact-path hook may now be registered."`
   (BC-6.22.001 invariant 7)

## Edge Cases

| EC | Description | Behavior |
|----|-------------|----------|
| EC-001 | Zero violations | Dry-run emits `"0 violations found. Registry is clean."` Apply mode skips all moves. Exits 0. |
| EC-002 | No `document_type` in frontmatter | Emit warning `"Cannot classify <path> — document_type field absent. Skipping."` File is not moved. |
| EC-003 | Proposed canonical path already exists | Refuse to move. Report collision. Exit non-zero. |
| EC-004 | Cross-reference in a file that is itself being moved | Resolve transitive reference graph before executing moves. Order moves to resolve dependencies. |
| EC-005 | `git mv` fails | Log error. Abort apply. Exit non-zero. |
| EC-006 | `--apply` with uncommitted changes | Warn and require `--force-dirty-tree` to proceed. |
| EC-007 | Registry pattern placeholder unresolvable from frontmatter | Emit warning and skip file. |
