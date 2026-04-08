---
name: validate-consistency
description: Cross-file consistency validation — verify spec IDs, anchor links, counts, naming, and traceability across all planning artifacts. Catches stale references, broken links, and mismatched counts.
disable-model-invocation: true
allowed-tools: Read, Bash, Glob, Grep
---

# Validate Consistency

Check cross-file consistency across all factory artifacts.

## Templates

Read and follow the output format in:
- `.claude/templates/consistency-report-template.md` — consistency validation report

## Checks

### 1. BC ID Integrity

- Every BC file in `behavioral-contracts/` is listed in BC-INDEX.md
- Every BC referenced in prd.md exists as a file
- No duplicate BC IDs
- BC numbering follows S.SS.NNN convention (no gaps expected, but no duplicates)

### 2. VP ID Integrity

- Every VP file in `verification-properties/` is listed in VP-INDEX.md
- Every VP referenced in architecture docs exists as a file
- VP status matches across index and file

### 3. Story Traceability

- Every story references at least one BC
- Every BC is referenced by at least one story (no orphan BCs)
- Story dependencies in dependency-graph.md match story file contents
- Wave assignments are consistent between story files and wave-schedule.md

### 4. Architecture Cross-References

- ARCH-INDEX.md lists all ARCH-NN files that exist
- No ARCH-NN files exist that aren't in the index
- Architecture section references in BCs point to valid sections

### 5. Count Consistency

- Story count in STORY-INDEX.md matches actual story files
- BC count in BC-INDEX.md matches actual BC files
- Epic story counts in epics.md match actual stories per epic

### 6. Status Consistency

- sprint-state.yaml statuses match STORY-INDEX.md statuses
- BC-INDEX.md statuses match individual BC file statuses

### 7. Naming Consistency

- Entity names in stories match domain-spec/ubiquitous-language.md
- Module names in architecture match story file lists

## Output

```markdown
# Consistency Validation Report

## Summary
- Checks run: <N>
- Passed: <N>
- Failed: <N>
- Warnings: <N>

## Failures
| Check | Issue | Files Involved |
|-------|-------|---------------|
| BC ID | BC-1.01.003 in prd.md but no file exists | prd.md |

## Warnings
| Check | Issue | Files Involved |
|-------|-------|---------------|
| Orphan BC | BC-2.01.001 not referenced by any story | BC-2.01.001.md |

## All Passed
<List of checks that passed cleanly>
```
