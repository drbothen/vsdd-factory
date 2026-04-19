---
name: conform-to-template
description: Fix structural gaps in a VSDD artifact file by adding missing frontmatter fields, section headings, and table structures from its template. Preserves all existing content — never deletes, only adds structure.
argument-hint: "<file-path> [--template=<template-name>]"
---

# Conform to Template

Fix structural gaps in a VSDD artifact file by comparing it against its template and adding missing structure.

## When to Use

- After `validate-template-compliance` reports WARN or FAIL on a file
- When migrating a file to a new template version
- When an agent produced output that's missing required sections
- When manually created artifacts need to match the template structure

## Safety Guarantees

1. **Never deletes content.** Only adds missing structure (fields, headings, placeholders).
2. **Never modifies existing content.** Existing frontmatter values, section content, and table data are preserved as-is.
3. **Always shows changes before applying.** Presents a diff and waits for user approval.
4. **Creates backup before modifying.** Copies the original to `<filename>.backup-YYYY-MM-DD-HHMMSS`.
5. **Single file only.** Does not batch-modify — each file requires explicit invocation and approval.

## Procedure

### Step 1: Resolve the template

Parse `$ARGUMENTS` for the file path and optional `--template=` override.

If `--template` is provided, use that template directly from `${CLAUDE_PLUGIN_ROOT}/templates/`.

If not provided, resolve the template using the same lookup logic as `validate-template-compliance`:
1. Read `document_type` from file frontmatter → find matching template
2. Fall back to path pattern matching

If no template can be resolved, report error and stop.

### Step 2: Run compliance check

Run the same three-level check as `validate-template-compliance` (frontmatter, sections, tables). Identify all gaps.

If no gaps found, report: "File already conforms to template. No changes needed." and stop.

### Step 3: Plan changes

For each gap, determine the fix:

#### Missing frontmatter fields

For each field in the template but not in the file:
- Copy the field with its template placeholder value (e.g., `supplements: []`, `status: draft`)
- Insert at the end of the frontmatter block, before the closing `---`

#### Missing sections

For each `## ` heading in the template but not in the file:
- Determine the correct insertion point (after the preceding template section that IS present in the file)
- Insert the heading followed by a placeholder:
  ```markdown
  ## [Section Name]

  [TODO: populate this section per template]
  ```

#### Missing table columns

For tables with missing columns:
- Report the mismatch but do NOT auto-fix. Table content is semantic — adding columns with empty data could mislead downstream agents.
- Instead, report: "Table in section '[section]' is missing columns: [list]. Manual fix required."

#### Section reordering

If sections are present but in the wrong order relative to the template:
- Report the misordering but do NOT auto-reorder. Moving sections could break cross-references.
- Instead, report: "Sections are out of template order. Current: [A, C, B]. Expected: [A, B, C]. Manual reorder recommended."

### Step 4: Present changes

Show the planned changes to the user:

```
## Planned Changes for <filename>

Template: <template-name>

### Frontmatter additions (3 fields):
+ supplements: []
+ input-hash: "[md5]"
+ pass: 1

### Section additions (1 section):
+ ## 5b. Test Vectors (after ## 5. Error Taxonomy)
  [TODO: populate this section per template]

### Manual fixes needed:
- Table in "Architecture Mapping": missing column "Pure/Effectful"
- Sections out of order: "Edge Cases" should come before "Dependencies"

Apply these changes? [yes / no / edit]
```

### Step 5: Apply changes

On user approval:

1. **Create backup:** Copy file to `<filename>.backup-<timestamp>`
2. **Add frontmatter fields:** Insert missing fields before closing `---`
3. **Add sections:** Insert missing section headings with `[TODO]` placeholders at correct positions
4. **Report:** "Conformed `<filename>` to `<template>`. Added N frontmatter fields, M sections. N manual fixes still needed."

If user says "no", stop without changes.
If user says "edit", allow user to modify the plan before applying.

### Step 6: Post-conformance validation

After applying changes, run `validate-template-compliance` on the modified file to confirm structural compliance improved.

Report the before/after:
```
Before: Frontmatter 10/12, Sections 6/8, Tables PASS → Overall WARN
After:  Frontmatter 12/12, Sections 8/8, Tables PASS → Overall PASS
```

## Limitations

- Does NOT fix table column mismatches — reports them for manual fix
- Does NOT reorder sections — reports misordering for manual fix
- Does NOT populate placeholder content — adds `[TODO]` markers only
- Does NOT handle files without frontmatter — requires at least `---` markers
- Does NOT modify template files — only target artifact files

## Out of Scope

- Content quality assessment (is the section well-written?)
- Semantic validation (do the values make sense?)
- Template creation (use `validate-template-compliance` to identify inline formats that need templates)
