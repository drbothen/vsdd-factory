---
name: step-e-triage-fix
description: Route adversary findings to responsible agents by severity. Fix blocking issues before re-review.
---

# Step E: Triage and Fix

> **Shared context:** Read `./_shared-context.md` before executing this step.

Route findings to responsible agents and fix blocking issues.

## Procedure

### 1. Severity Routing

| Severity | Action | Blocks F6? |
|----------|--------|-----------|
| **CRITICAL** | Must be fixed before proceeding | Yes |
| **HIGH** | Must be fixed before proceeding | Yes |
| **MEDIUM** | Should be fixed, orchestrator decides | Orchestrator call |
| **LOW** | Documented but not blocking | No |
| **COSMETIC** | Logged, not blocking | No |

### 2. For Each Fix

1. Route to the responsible agent (implementer, test-writer, etc.)
2. Agent makes the fix
3. Re-run relevant tests (new + regression)
4. Verify the fix addresses the specific finding

### 3. Fix Delivery (DF-025)

Fixes go through the per-story delivery flow via `code-delivery.lobster`:

```
FIX-F5-NNN → worktree → fix → demo (if behavior-changing) → PR → AI review
  → security review (if applicable) → merge → re-verify only failing checks
```

### 4. Security Routing

If the adversary identifies security findings, route to `security-reviewer` for dedicated CWE/OWASP analysis. The security-reviewer cannot see the adversary's reasoning (information asymmetry wall).

## Artifacts

- Fixed code (for CRITICAL/HIGH findings)
- Updated test results
- Finding resolution status per finding

## Success Criteria

- All CRITICAL and HIGH findings resolved
- Tests still pass after fixes
- Resolution status documented for each finding
- Security findings routed to security-reviewer
