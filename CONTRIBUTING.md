# Contributing to vsdd-factory

## Development workflow

### Prerequisites

- Rust toolchain (see `rust-toolchain.toml` — currently 1.95.0)
- `bats` (Bash test runner) for hook tests
- `jq` and `yq` for JSON/YAML validation steps
- `semgrep` for local security scanning (`pip install semgrep` or `brew install semgrep`)

### Running tests locally

```bash
# Rust workspace
cargo test --workspace --all-targets

# Hook and bin tests
bats plugins/vsdd-factory/tests/hooks.bats
bats plugins/vsdd-factory/tests/bin.bats
```

### Branch model

All feature work targets `develop`. PRs from `feature/STORY-NNN` branches are
opened against `develop` and squash-merged after CI passes. Releases merge
`develop` to `main` and tag from `main`.

---

## Security scanning

Semgrep is the project's SAST (Static Application Security Testing) tool.
It covers all four languages in the project footprint: Rust, Python,
TypeScript, and Go (per TD-006 scoping).

### When it runs

- On every PR to `develop` or `main` (diff-scoped scan via `semgrep ci`)
- On every push to `develop` or `main` (full scan)
- Weekly on Monday at noon UTC (scheduled full-repo scan)

Workflow definition: `.github/workflows/semgrep.yml`

### Severity policy

Findings at severity **ERROR** or **WARNING** block merge. Findings at
INFO are advisory and do not gate.

### Running locally

```bash
semgrep --config=auto .
```

Or to match the exact CI rule packs:

```bash
semgrep --config=p/security-audit \
        --config=p/secrets \
        --config=p/owasp-top-ten \
        --config=p/cwe-top-25 \
        .
```

### Triaging a finding

Three options, in preference order:

1. **Remediate** — fix the underlying issue (preferred).
2. **Suppress inline** — add a `# nosemgrep: <rule-id>` comment on the
   offending line with a justification comment above it explaining why the
   finding is a false positive.
3. **Override in `.semgrep.yml`** — add a `paths.exclude` or rule-level
   `filter` for systematic false-positive patterns. Document the rationale
   in a comment.

### Custom rules

Custom organizational rules live in `.semgrep.yml` (currently a stub).
TD-006 follow-up will register two policy rules:

- **POLICY 11** — tautology detection (Check 8 of the TD-006 adversarial
  review checklist): flags spec assertions that are vacuously true.
- **POLICY 12** — BC-TV consistency enforcement (Check 9): flags
  test-vector mismatches against behavioral contracts.

To add a rule, append to the `rules:` list in `.semgrep.yml` following
the Semgrep rule schema at https://semgrep.dev/docs/writing-rules/rule-syntax/

### Baseline

Initial adoption scan (Semgrep 1.156.0, 2026-04-27): **0 findings** across
308 files, 320 rules, covering Python + TypeScript + multilang rule packs.

---

## Spec File Naming

All canonical spec filenames use lowercase. The PRD lives at
`.factory/specs/prd.md`. Any historical `PRD.md` (uppercase) references have
been canonicalized to `prd.md`. If you encounter an uppercase reference in an
adversarial review or consistency report, update it to lowercase in the live
spec file it appears in; archived adversarial-review and log files are
historical records and do not require retroactive updates.
