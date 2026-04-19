---
name: validate-template-compliance
description: Audit whether VSDD artifact files conform to their corresponding templates. Checks frontmatter fields, section headings, and table column headers. Read-only — reports findings without modifying files.
argument-hint: "<file-or-directory>"
---

# Validate Template Compliance

Audit whether artifact files in `.factory/` conform to their corresponding templates from `${CLAUDE_PLUGIN_ROOT}/templates/`.

## When to Use

- Before a phase gate — verify all artifacts produced in this phase match their templates
- After a batch creation — verify story-writer or product-owner output is structurally complete
- During maintenance sweeps — find artifacts that drifted from templates over time
- After template updates — identify artifacts that need to conform to the new template version

## Procedure

### Step 1: Determine scope

Parse `$ARGUMENTS`:
- **Single file:** `/vsdd-factory:validate-template-compliance .factory/specs/prd.md`
- **Directory:** `/vsdd-factory:validate-template-compliance .factory/specs/` — recurse all `.md` files
- **All artifacts:** `/vsdd-factory:validate-template-compliance .factory/` — full audit

### Step 2: For each file, resolve the template

**Primary lookup — `document_type` frontmatter:**
1. Read the file's YAML frontmatter
2. Extract the `document_type` field
3. Search `${CLAUDE_PLUGIN_ROOT}/templates/` for a template with matching `document_type`

**Fallback lookup — file path pattern:**

| Path Pattern | Template |
|-------------|----------|
| `behavioral-contracts/BC-*.md` | `behavioral-contract-template.md` |
| `behavioral-contracts/BC-INDEX.md` | (no template — skip) |
| `verification-properties/VP-*.md` | `L4-verification-property-template.md` |
| `verification-properties/VP-INDEX.md` | (no template — skip) |
| `stories/STORY-*.md` | `story-template.md` |
| `stories/STORY-INDEX.md` | `story-index-template.md` |
| `architecture/ARCH-INDEX.md` | `architecture-index-template.md` |
| `architecture/verification-coverage-matrix.md` | `verification-coverage-matrix-template.md` |
| `architecture/verification-architecture.md` | `verification-architecture-template.md` |
| `architecture/*.md` (other sections) | `architecture-section-template.md` |
| `domain-spec/L2-INDEX.md` | `L2-domain-spec-index-template.md` |
| `domain-spec/*.md` (sections) | `L2-domain-spec-section-template.md` |
| `holdout-scenarios/HS-*.md` | `holdout-scenario-template.md` |
| `prd.md` | `prd-template.md` |
| `product-brief.md` | `product-brief-template.md` |
| `STATE.md` | `state-template.md` |
| `dtu-assessment.md` | `dtu-assessment-template.md` |
| `module-criticality.md` | `module-criticality-template.md` |

If no template can be resolved, report: "No template found for `<file>` (document_type: `<type>`, path: `<path>`). Skipping."

### Step 3: Run three-level compliance check

For each file + template pair:

#### Level 1: Frontmatter Compliance

1. Extract all YAML frontmatter keys from the template (everything between `---` markers)
2. Extract all YAML frontmatter keys from the file
3. Report:
   - **Present:** Keys in both template and file
   - **Missing:** Keys in template but not in file (these are required fields)
   - **Extra:** Keys in file but not in template (these may be intentional extensions)
4. Severity: Missing required fields = FAIL. Extra fields = INFO (not a violation).

#### Level 2: Section Compliance

1. Extract all `## ` (H2) headings from the template, in order
2. Extract all `## ` headings from the file, in order
3. Report:
   - **Present:** Template sections found in file
   - **Missing:** Template sections not found in file
   - **Extra:** File sections not in template (may be intentional)
   - **Out of order:** Sections present but in different order than template
4. Severity: Missing required sections = WARN. Out of order = INFO.

#### Level 3: Table Column Compliance

1. For each table in the template (identified by `| Header1 | Header2 |` followed by `|---|`):
   - Extract column headers (split by `|`, trim whitespace)
2. For each corresponding table in the file (match by nearest preceding H2 heading):
   - Extract column headers
3. Report:
   - **Match:** Column headers identical
   - **Missing columns:** Template has columns the file doesn't
   - **Extra columns:** File has columns the template doesn't
   - **Reordered:** Same columns but different order
4. Severity: Missing columns = WARN. Reordered = INFO.

### Step 4: Generate report

**Per-file report:**
```
## <filename>
Template: <template-name>

### Frontmatter
- Present: 10/12 fields
- Missing: supplements, input-hash
- Extra: custom_field (OK — project extension)
- Status: WARN

### Sections
- Present: 7/8 sections
- Missing: ## 5b. Test Vectors
- Extra: ## Custom Analysis (OK)
- Order: CORRECT
- Status: WARN

### Tables
- Section "Acceptance Criteria": columns MATCH
- Section "Architecture Mapping": MISSING column "Pure/Effectful"
- Status: WARN

### Overall: PASS | WARN | FAIL
```

**Summary table (for directory scans):**
```
| File | Template | Frontmatter | Sections | Tables | Overall |
|------|----------|-------------|----------|--------|---------|
| prd.md | prd-template | 10/12 WARN | 7/8 WARN | PASS | WARN |
| BC-2.01.001.md | behavioral-contract | 22/22 PASS | 7/7 PASS | PASS | PASS |
| STORY-001.md | story | 18/20 WARN | 10/12 WARN | WARN | WARN |
```

**Aggregate counts:**
```
Files checked: 42
PASS: 28 (67%)
WARN: 12 (29%)
FAIL: 2 (5%)
```

## Limitations

- Does NOT check content quality — only structural compliance (fields exist, sections exist, columns match)
- Does NOT validate frontmatter VALUES — only that fields are present
- Does NOT check whether table DATA is correct — only that column headers match
- Template resolution depends on `document_type` frontmatter or path patterns — files without either are skipped
- INDEX files (BC-INDEX, VP-INDEX, STORY-INDEX, ARCH-INDEX) generally have no template — they're auto-generated

## Templates

Use `${CLAUDE_PLUGIN_ROOT}/templates/consistency-validation-report-template.md` for the output report format.
