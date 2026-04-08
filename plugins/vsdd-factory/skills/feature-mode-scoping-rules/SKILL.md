---
name: feature-mode-scoping-rules
description: >
  Reference document defining how scope is determined in Feature Mode.
  Used by all F1-F7 phase skills.
---

# Feature Mode Scoping Rules

## What Is "The Delta"?

The delta is the set of files and artifacts that MUST change to implement a feature,
plus their direct dependents. It is defined in Phase F1 and remains fixed through
all subsequent phases (unless the human explicitly revises scope).

### Delta Definition

```
Delta = {
  NEW files:      Files that will be created for this feature
  MODIFIED files: Existing files that must change
  DEPENDENT files: Unchanged files that import/depend on MODIFIED files
}
```

DEPENDENT files are included in scope for adversarial review and verification
but are NOT modified during implementation.

## Scope by Phase

| Phase | Primary Scope | Regression Scope |
|-------|--------------|-----------------|
| F1: Delta Analysis | Full codebase (to identify the delta) | N/A |
| F2: Spec Evolution | Delta requirements and architecture | N/A |
| F3: Story Creation | Delta stories only | Existing story graph (cycle check) |
| F4: Implementation | Delta files (NEW + MODIFIED) | ALL existing tests (full suite) |
| F5: Adversarial | Delta files (NEW + MODIFIED + DEPENDENT) | Convention check against full codebase |
| F6: Hardening | Delta files for proofs/fuzz/mutation | ALL tests + full dependency audit |
| F7: Convergence | Four dimensions on delta only | ALL tests + full traceability chain |

## Key Scoping Rules

### Rule 1: Regression Is NEVER Scoped

The regression test suite always runs against the FULL existing test suite.
Never skip existing tests because they are "unrelated." Silent breakage in
unrelated modules is the most dangerous kind of regression.

### Rule 2: Adversarial Review Is Delta + Dependents

The adversary reviews:
- All NEW files
- All MODIFIED files (full file, not just diff)
- All DEPENDENT files (unchanged files that import modified files)

The adversary does NOT review:
- Files with no connection to the delta
- Previous adversarial reports (fresh perspective required)

### Rule 3: Formal Verification Is Delta + Modified Dependencies

Kani proofs and fuzz targets cover:
- New modules (prove new properties)
- Modified modules (re-prove existing properties with new code)
- Modules that depend on modified modules IF the interface changed

Formal verification does NOT cover:
- Unchanged modules with unchanged interfaces
- Existing proofs that are unaffected by the delta

### Rule 4: Security Scanning Is Full Tree for Dependencies

While code-level security scanning (Semgrep) can be scoped to the delta,
dependency auditing (cargo audit, npm audit) MUST scan the full dependency tree.
A new dependency added by the feature could introduce vulnerabilities that affect
the entire application.

### Rule 5: Convergence Metrics Are Delta-Scoped, Regression Is Binary

The five-dimensional convergence check (spec fidelity, mutation kill rate,
adversary verification rate, proof status) is calculated on the delta only.
This prevents a small feature from being held up by pre-existing convergence
gaps in unrelated code.

Regression is a separate, binary check: all existing tests pass, or they do not.

### Rule 6: Scope Is Immutable After F1 (Unless Human Revises)

Once the human approves the scope in Phase F1, it does not expand silently.
If during implementation an agent discovers that additional files need modification:
1. Log the discovery
2. Present to the human with rationale
3. Human approves the scope expansion
4. Update `.factory/phase-f1-delta-analysis/affected-files.txt`

This prevents scope creep and ensures regression protection is not undermined.

## Determining File Scope

### Identifying NEW Files

Files that do not exist yet and must be created:
- New source modules (e.g., `src/services/notification_service.rs`)
- New test files (e.g., `tests/notification_tests.rs`)
- New configuration (if feature requires new config entries)

### Identifying MODIFIED Files

Files that exist and must change:
- Modules that gain new public interfaces
- Modules that change internal behavior
- Configuration files that gain new entries
- Test files that need new test cases for modified behavior

### Identifying DEPENDENT Files

Files that import or call MODIFIED files but do not themselves change:
- Modules that call functions in MODIFIED modules
- Modules that use types defined in MODIFIED modules
- Integration test files that exercise MODIFIED modules

Use the project's import graph to identify dependents:
```bash
# Rust: find files that import the modified module
grep -rl "use crate::modified_module" src/

# TypeScript: find files that import the modified module
grep -rl "from './modified_module'" src/

# Python: find files that import the modified module
grep -rl "from modified_module import" src/
```

## Applicability

Reference document -- no quality gate. Consumed by F1-F7 phase skills.
