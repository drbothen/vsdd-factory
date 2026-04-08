# Step 5: Security Scanning

Run security scans on the full dependency tree and changed files.

## Inputs

- Changed file list
- Full project dependency tree

## Actions

1. Run Semgrep on changed files: `semgrep --config=auto <changed-files>`
2. Run dependency audit on full tree: `cargo audit` or `npm audit` or `pip-audit`
3. Any CRITICAL or HIGH severity findings must be fixed before proceeding

## Outputs

- `.factory/phase-f6-hardening/security-scan-results.md`

## Completion Criteria

- Semgrep scan completed on all changed files
- Dependency audit completed on full tree
- No CRITICAL or HIGH findings unresolved
