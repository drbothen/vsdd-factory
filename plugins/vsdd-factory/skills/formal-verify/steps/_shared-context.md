---
name: formal-verify-shared-context
description: Shared context for formal verification steps. Contains templates, prerequisites, and output location.
---

# Formal Verification — Shared Context

This file is loaded by every step in the formal-verify skill.

## Templates

- `${CLAUDE_PLUGIN_ROOT}/templates/formal-verification-template.md` — formal verification report
- `${CLAUDE_PLUGIN_ROOT}/templates/fuzz-report-template.md` — fuzz testing report
- `${CLAUDE_PLUGIN_ROOT}/templates/security-review-template.md` — security scan findings
- `${CLAUDE_PLUGIN_ROOT}/templates/security-scan-report-template.md` — security scan report
- `${CLAUDE_PLUGIN_ROOT}/templates/verification-gap-analysis-template.md` — verification coverage gaps

## Output Location

All results are written to `.factory/cycles/<current>/formal-verification-report.md`.

Each step appends its section to the report. The first step creates the file with the Summary table; subsequent steps fill in their sections.

## Prerequisites

Verification tools must be installed. If a tool is not available, report which tools are missing and skip that section. Never fail silently (SOUL.md #4).

```bash
cargo install cargo-kani
cargo install cargo-fuzz
cargo install cargo-mutants
pip install semgrep  # or brew install semgrep
```

## Allowed Tools

- Read, Write, Bash, Glob, Grep
- Model invocation is disabled (`disable-model-invocation: true`)

## Module Criticality

Read `.factory/module-criticality.md` to determine which modules need which level of verification:
- **CRITICAL** and **HIGH**: All 4 techniques (Kani, fuzz, mutation, security)
- **MEDIUM**: Fuzz, mutation, security
- **LOW**: Security scan only
