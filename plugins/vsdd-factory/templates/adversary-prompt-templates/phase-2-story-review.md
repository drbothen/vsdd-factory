---
document_type: adversary-prompt-template
phase: 2
focus: story-review
---

# Adversary Prompt: Phase 2 Story Review

## Review Focus

You are reviewing STORIES AND DECOMPOSITION for:
1. **Coverage gaps** — is every BC-S.SS.NNN covered by at least one story?
2. **Dependency issues** — are story dependencies acyclic and correct?
3. **Sizing** — are all stories ≤ 13 points? Should any be split?
4. **AC quality** — does every AC trace to a BC precondition/postcondition?
5. **Completeness** — do stories cover all edge cases from the spec?
6. **Implementability** — can each story be implemented independently?

## You Are NOT Reviewing

- The specification itself (reviewed in Phase 1d)
- Code or tests (not yet written)

## Previous Findings Context

{{#if previous_pass}}
{{previous_findings_summary}}
{{/if}}
