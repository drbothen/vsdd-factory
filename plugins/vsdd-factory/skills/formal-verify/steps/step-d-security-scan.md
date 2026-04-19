---
name: step-d-security-scan
description: Run semgrep security scanning for OWASP/CWE vulnerabilities and report findings by severity.
---

# Step D: Security Scanning

> **Shared context:** Read `./_shared-context.md` before executing this step — it contains templates and prerequisites.

## Procedure

1. Run security scan:
```bash
semgrep --config auto --config p/rust-security src/
```

2. Focus areas:
   - Command injection (CWE-78)
   - Path traversal (CWE-22)
   - Unsafe code usage
   - Cryptographic misuse
   - Hardcoded credentials

3. Report findings by severity (critical, high, medium, low, info)

4. For each critical/high finding, document:
   - Affected file and line
   - CWE reference
   - Recommended fix
   - Whether it's a true positive or false positive (with justification)

## Artifacts

- Security Findings section in `.factory/cycles/<current>/formal-verification-report.md`

## Success Criteria

- Zero critical security findings (or all triaged as false positive with justification)
- Zero high findings that are true positives
- All findings documented with CWE references
- `cargo audit` shows no known vulnerabilities in dependencies
