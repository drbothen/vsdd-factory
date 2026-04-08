---
document_type: maintenance-sweep-report
date: YYYY-MM-DD
trigger: scheduled | manual | post-deploy
---

# Maintenance Sweep Report: YYYY-MM-DD

## Summary

| Sweep | Status | Findings | PRs Opened | Issues Created |
|-------|--------|----------|-----------|----------------|
| Dependency Audit | CLEAN / FINDINGS | [N] | [N] | [N] |
| Documentation Drift | CLEAN / FINDINGS | [N] | [N] | [N] |
| Pattern Consistency | CLEAN / FINDINGS | [N] | [N] | [N] |
| Holdout Freshness | CLEAN / FINDINGS | [N] | [N] | [N] |
| Performance Baseline | CLEAN / FINDINGS | [N] | [N] | [N] |

## Overall Health: [HEALTHY / NEEDS_ATTENTION / DEGRADED]

---

## Dependency Audit

### New Vulnerabilities

| Dependency | Version | CVE/Advisory | Severity | Fix Available | Action |
|-----------|---------|-------------|----------|--------------|--------|
| [dep] | [ver] | [CVE-XXXX] | [sev] | [yes/no] | [PR #N / logged] |

---

## Documentation Drift

### Stale Documentation

| Document | Section | Drift Type | Severity | Action |
|----------|---------|-----------|----------|--------|
| [doc] | [section] | [outdated / broken ref / missing] | [high/med/low] | [PR #N / issue] |

---

## Pattern Consistency

### Inconsistencies Detected

| Pattern | Expected | Found In | Severity | Action |
|---------|----------|----------|----------|--------|
| [pattern] | [convention] | [file:line] | [high/med/low] | [PR #N / logged] |

---

## Holdout Scenario Freshness

| Metric | Value |
|--------|-------|
| Total scenarios | [N] |
| Still valid | [N] |
| Stale (intentional change) | [N] |
| Missing coverage (features with 0 scenarios) | [N] |

---

## Performance Baseline

| Benchmark | Previous | Current | Delta | Status |
|-----------|----------|---------|-------|--------|
| [bench] | [value] | [value] | [+/-X%] | [OK / WARNING / CRITICAL] |

---

## Trend (Last 5 Sweeps)

| Date | Dependencies | Docs | Patterns | Holdouts | Performance |
|------|-------------|------|----------|----------|-------------|
| [date] | [findings] | [findings] | [findings] | [pass rate] | [delta] |
