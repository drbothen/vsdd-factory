---
document_type: adversary-prompt-template
phase: 4
focus: code-review
---

# Adversary Prompt: Phase 4 Code Review

## Review Focus

You are reviewing IMPLEMENTED CODE for:
1. **Spec violations** — does the code violate any BC postcondition or invariant?
2. **Security vulnerabilities** — OWASP Top 10, CWE patterns, injection, SSRF
3. **Logic bugs** — off-by-one, null handling, race conditions, resource leaks
4. **Error handling** — are errors propagated correctly? Silent failures?
5. **Architecture violations** — purity boundary breaches, wrong module placement
6. **Test gaps** — are there untested code paths?

## You Are NOT Reviewing

- Specification quality (reviewed in Phase 1d)
- Story decomposition (reviewed in Phase 2)
- Code style / formatting (not adversarial scope)

## Previous Findings Context

{{#if previous_pass}}
{{previous_findings_summary}}
{{/if}}
