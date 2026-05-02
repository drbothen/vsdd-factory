---
name: generate-pdf
description: Generate a professional 1898 & Co. branded PDF from a markdown research document. Use when the user wants to create a PDF, export a report, or convert markdown to PDF. Validates frontmatter, uses branded template, and reports results.
allowed-tools: Read, Bash
disable-model-invocation: true
---

# PDF Report Generator

Generate professional 1898 & Co. branded PDFs from markdown research documents.

## Prerequisites

- pandoc (install: `brew install pandoc`)
- weasyprint (install: `brew install weasyprint`)

## Workflow

### Step 1: Identify Input File

If the user provides a file path, use it. Otherwise, ask which markdown file to convert.

Valid input files:
- Must be `.md` files
- Should be in `_bmad-output/planning-artifacts/research/` directory
- Should have YAML frontmatter for title page generation

### Step 2: Validate Frontmatter

Read the markdown file and check for YAML frontmatter:

```yaml
---
title: "Report Title"
subtitle: "Optional Subtitle"  # optional
author: "Author Name"
date: "Month Year"
classification: "Confidential - Internal Use Only"  # optional
---
```

**Required fields**: title, author, date

If frontmatter is missing or incomplete, warn the user and ask if they want to:
1. Proceed without a proper title page
2. Add frontmatter first (provide template)

### Step 3: Generate PDF

Run the generation script:

```bash
cd _bmad-output/planning-artifacts/research
./generate-pdf.sh -i <input.md> -o <output.pdf>
```

If no output name specified, default is `<input>.pdf` (same name as input).

### Step 4: Report Results

On success:
- Report the output PDF path
- Note the page count if possible
- Remind user where to find the file

On failure:
- Report the specific error
- Suggest fixes (missing prerequisites, invalid markdown, etc.)

## Template Features

The 1898 & Co. branded template includes:
- Title page generated from frontmatter
- Page footer: "1898 & Co. - Confidential" with page numbers
- Each h1 section starts on a new page
- Each h2 section starts on a new page
- Branded table styling (Prussian Blue headers)
- Orange-accented blockquote callouts

## Brand Colors Reference

| Color | Hex | Usage |
|-------|-----|-------|
| Primary Orange | `#ff6a39` | Accents, callouts |
| Dark Charcoal | `#393a3c` | Body text |
| Prussian Blue | `#002d59` | Headings, table headers |
| B&M Cobalt | `#003fb3` | Links, h3 headings |

## Example Usage

User: `/generate-pdf domain-qbr-customer-communication-analysis-2026-02-03.md`

Output:
```
Generating PDF from: domain-qbr-customer-communication-analysis-2026-02-03.md

Validating frontmatter...
  title: Monroe Energy Q1 2026 QBR Analysis
  author: Joshua Magady
  date: February 2026

Generating PDF...

PDF generated successfully:
  _bmad-output/planning-artifacts/research/domain-qbr-customer-communication-analysis-2026-02-03.pdf
```

## Error Handling

| Error | Solution |
|-------|----------|
| pandoc not found | `brew install pandoc` |
| weasyprint not found | `brew install weasyprint` |
| Input file not found | Check path, ensure file exists |
| Missing frontmatter | Add YAML frontmatter block |
| Template not found | Ensure templates/1898-report.css exists |
