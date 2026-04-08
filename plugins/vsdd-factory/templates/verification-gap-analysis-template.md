# Verification Gap Analysis: [PROJECT_NAME]

> Analyzed by Dark Factory Phase 0e
> Source codebase: `[CODEBASE_PATH]`
> Date: [GENERATION_DATE]

---

## Test Coverage Baseline

### Overall Metrics

| Metric | Value | Tool Used |
|--------|-------|-----------|
| **Line coverage** | [X% or "not measurable"] | [tool or "no coverage tool available"] |
| **Branch coverage** | [X% or "not measurable"] | [tool or "not supported by tool"] |
| **Function coverage** | [X% or "not measurable"] | [tool or "not supported by tool"] |
| **Total test count** | [N] | [test runner] |
| **Test execution time** | [N seconds] | [test runner] |

### Per-Module Coverage

| Module | Path | Line Coverage | Tests | Assessment |
|--------|------|-------------|-------|------------|
| [module-1] | `[path]` | [X%] | [N tests] | [adequate / insufficient / none] |
| [module-2] | `[path]` | [X%] | [N tests] | [adequate / insufficient / none] |

### Coverage Tool Setup

If coverage tool was not available:
- **Recommended tool:** [cargo-llvm-cov / tarpaulin / coverage.py / c8]
- **Install command:** `[installation command]`
- **Run command:** `[coverage run command]`

---

## Purity Assessment

### Module Classification

| Module | Path | Classification | I/O Types | Refactoring Needed |
|--------|------|---------------|-----------|-------------------|
| [module-1] | `[path]` | [pure] | [none] | [none] |
| [module-2] | `[path]` | [effectful shell] | [file read, HTTP] | [none] |
| [module-3] | `[path]` | [mixed] | [database, logging] | [extract I/O to shell] |
| [module-4] | `[path]` | [opaque] | [FFI, dynamic] | [cannot classify] |

### Purity Boundary Map

```text
+-- Pure Core (safe for formal verification) --+
|                                               |
|   [module-1]    [module-5]    [module-8]      |
|                                               |
+-----------------------------------------------+
                      |
                      v (called by)
+-- Effectful Shell (I/O at boundary) ---------+
|                                               |
|   [module-2]    [module-6]                    |
|                                               |
+-----------------------------------------------+
                      |
                      v (called by)
+-- Mixed (needs refactoring) -----------------+
|                                               |
|   [module-3]    [module-7]                    |
|                                               |
+-----------------------------------------------+
```

### Summary

| Classification | Count | Percentage |
|---------------|-------|------------|
| Pure | [N] | [X%] |
| Effectful shell | [N] | [X%] |
| Mixed | [N] | [X%] |
| Opaque | [N] | [X%] |

---

## Formal Verification Readiness

### Tool Compatibility

| Tool | Applicable? | Setup Required | Notes |
|------|------------|---------------|-------|
| **Kani** (Rust) | [yes / no / N/A] | [install command or "already installed"] | [notes] |
| **CBMC** (C/C++) | [yes / no / N/A] | [install command] | [notes] |
| **proptest** (Rust) | [yes / no / N/A] | [add to Cargo.toml] | [notes] |
| **hypothesis** (Python) | [yes / no / N/A] | [pip install] | [notes] |
| **fast-check** (JS/TS) | [yes / no / N/A] | [npm install] | [notes] |

### Per-Module Readiness

| Module | Path | Verifiable Now? | Blocking Issue | Effort to Ready |
|--------|------|----------------|---------------|-----------------|
| [module-1] | `[path]` | [yes] | [none] | [0 hours] |
| [module-2] | `[path]` | [no] | [I/O interleaved with logic] | [4 hours -- extract pure core] |
| [module-3] | `[path]` | [no] | [unbounded inputs] | [2 hours -- add input constraints] |

### Recommended Verification Targets

Priority-ordered list of modules where formal verification would provide the most value:

1. **[module-1]** -- [reason: handles security-critical logic, already pure, bounded inputs]
2. **[module-2]** -- [reason: core business logic, needs minor refactoring]
3. **[module-3]** -- [reason: data validation, good candidate for proptest]

---

## Security Posture

### Scanner Results

| Scanner | Run Successfully? | Command | Notes |
|---------|------------------|---------|-------|
| [cargo audit] | [yes / no / N/A] | `[command]` | [notes] |
| [cargo deny] | [yes / no / N/A] | `[command]` | [notes] |
| [npm audit] | [yes / no / N/A] | `[command]` | [notes] |
| [pip-audit] | [yes / no / N/A] | `[command]` | [notes] |
| [Semgrep] | [yes / no / N/A] | `[command]` | [notes] |
| [bandit] | [yes / no / N/A] | `[command]` | [notes] |

### Vulnerability Findings

| # | Severity | Source | Finding | Affected Component | CWE | Recommendation |
|---|----------|--------|---------|-------------------|-----|----------------|
| V-1 | [critical] | [scanner] | [description] | `[path]` | [CWE-XXX] | [fix] |
| V-2 | [high] | [scanner] | [description] | `[path]` | [CWE-XXX] | [fix] |

### Dependency Advisories

| Dependency | Version | Advisory | Severity | Fix Available? |
|-----------|---------|----------|----------|---------------|
| [dep-1] | [version] | [advisory ID] | [severity] | [yes -- upgrade to X / no] |

### Security Summary

| Metric | Value |
|--------|-------|
| **Critical vulnerabilities** | [N] |
| **High vulnerabilities** | [N] |
| **Dependencies with known advisories** | [N] |
| **Unsafe code blocks** | [N] (Rust-specific) |
| **eval() / exec() usage** | [N] |

---

## Mutation Testing Baseline

### Overall Results

| Metric | Value | Tool Used |
|--------|-------|-----------|
| **Total mutants generated** | [N or "not run"] | [cargo-mutants / mutmut / stryker] |
| **Mutants killed** | [N] | |
| **Mutants survived** | [N] | |
| **Kill rate** | [X%] | |
| **Timeout** | [N seconds per mutant] | |

### Per-Module Results

| Module | Path | Mutants | Killed | Survived | Kill Rate |
|--------|------|---------|--------|----------|-----------|
| [module-1] | `[path]` | [N] | [N] | [N] | [X%] |

### Surviving Mutants (Top Concerns)

| # | Module | Mutation | Why It Survived | Risk |
|---|--------|---------|----------------|------|
| SM-1 | `[path:line]` | [mutation description] | [missing assertion / untested branch] | [high / medium / low] |

### Setup Notes

If mutation testing was not run:
- **Recommended tool:** [cargo-mutants / mutmut / stryker]
- **Install command:** `[installation command]`
- **Run command:** `[mutation testing command]`
- **Estimated runtime:** [based on test suite size and complexity]

---

## Identified Gaps

### Critical Gaps (Must Fix Before Extending)

| # | Gap | Affected Area | Risk | Effort |
|---|-----|--------------|------|--------|
| GAP-1 | [description] | [modules] | [high] | [hours/days] |

### Important Gaps (Should Fix Soon)

| # | Gap | Affected Area | Risk | Effort |
|---|-----|--------------|------|--------|
| GAP-2 | [description] | [modules] | [medium] | [hours/days] |

### Minor Gaps (Fix Opportunistically)

| # | Gap | Affected Area | Risk | Effort |
|---|-----|--------------|------|--------|
| GAP-3 | [description] | [modules] | [low] | [hours/days] |

---

## Remediation Plan

### Phase 0 Recommendations

These actions should be completed before proceeding to Phase 1:

1. **[Highest priority action]** -- [description, estimated effort]
2. **[Second priority action]** -- [description, estimated effort]
3. **[Third priority action]** -- [description, estimated effort]

### Ongoing Verification Improvements

These should be incorporated into the VSDD pipeline as new features are added:

1. **[Improvement 1]** -- [integrate into which VSDD phase]
2. **[Improvement 2]** -- [integrate into which VSDD phase]

### Total Estimated Remediation Effort

| Priority | Gap Count | Total Effort |
|----------|----------|-------------|
| Critical | [N] | [hours/days] |
| Important | [N] | [hours/days] |
| Minor | [N] | [hours/days] |
| **Total** | **[N]** | **[hours/days]** |
