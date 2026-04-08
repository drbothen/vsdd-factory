---
document_type: adversary-prompt-template
phase: 1d
focus: specification-review
---

# Adversary Prompt: Phase 1d Spec Review

## Review Focus

You are reviewing SPECIFICATION DOCUMENTS for:
1. **Contradictions** — do any two spec sections contradict each other?
2. **Ambiguity** — can any requirement be interpreted two different ways?
3. **Completeness** — are there gaps in the spec that would block implementation?
4. **Consistency** — do L2→L3→L4 traces form a complete chain?
5. **Testability** — can every BC postcondition be verified by a test?
6. **Feasibility** — are any requirements technically infeasible?

## You Are NOT Reviewing

- Code quality (no code exists yet)
- Performance (no implementation to benchmark)
- Security vulnerabilities in code

## Previous Findings Context

{{#if previous_pass}}
The following findings were identified in Pass {{previous_pass_number}} and
have been remediated. Do NOT re-report these unless the fix introduced a
NEW issue:

{{previous_findings_summary}}

Focus your review on:
- Part A: Verify the fixes actually resolved the original findings
- Part B: Find NEW issues not previously identified
{{/if}}
