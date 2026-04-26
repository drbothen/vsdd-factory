---
description: Scaffold a new ADR file with frontmatter, insert ARCH-INDEX row, optionally patch a superseded ADR bidirectionally, and validate template compliance. Atomic-or-nothing.
argument-hint: "--title <text> --subsystems <SS-NN[,...]> [--supersedes <ADR-NNN>] [--brownfield] [--id <ADR-NNN>] [--dry-run]"
---

Use the `vsdd-factory:create-adr` skill via the Skill tool.

Arguments: $ARGUMENTS

## Flags

| Flag | Required | Description |
|------|----------|-------------|
| `--title <text>` | YES | Decision title. Used verbatim in ARCH-INDEX row; slug-derived for filename. |
| `--subsystems <SS-NN[,...]>` | YES | Comma-separated subsystem IDs (e.g., `SS-06,SS-08`). Validated against ARCH-INDEX registry. |
| `--supersedes <ADR-NNN>` | no | Existing ADR being superseded. Patches old ADR with `superseded_by`. Also implies `--brownfield`. |
| `--brownfield` | no | Injects Source/Origin warning requiring implementation evidence before ADR can be accepted. |
| `--id <ADR-NNN>` | no | Override auto-allocated ID. Refused if ID already exists. |
| `--dry-run` | no | Print proposed ID + slug only. No files written. |

## Examples

```
/vsdd-factory:create-adr --title "Use Rust for dispatcher" --subsystems "SS-01,SS-09"
/vsdd-factory:create-adr --title "Replace WASM with native plugins" --subsystems "SS-02" --supersedes "ADR-002"
/vsdd-factory:create-adr --title "Adopt OpenTelemetry" --subsystems "SS-03" --brownfield
/vsdd-factory:create-adr --title "Test ID" --subsystems "SS-06" --dry-run
```
