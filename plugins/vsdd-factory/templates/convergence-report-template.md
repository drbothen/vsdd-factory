---
document_type: convergence-report
level: ops
version: "1.0"
status: draft
producer: "[agent-id]"
timestamp: YYYY-MM-DDTHH:MM:SS
phase: 7
inputs: []
input-hash: "[md5]"
traces_to: ""
---

# Convergence Report

## Pipeline Run: {timestamp}
## Product: {product_name}
## Iterations: {iteration_count}

---

## Seven-Dimension Convergence Scorecard

| Dimension | Status | Evidence | Score |
|-----------|--------|----------|-------|
| Spec → Code Traceability (L1→L4) | {status} | L1→L2→L3→L4 chain completeness | {score} |
| Implementation Completeness | {status} | All stories implemented, tests green | {score} |
| Adversarial Review Convergence | {status} | finding decay: [{N₁}, {N₂}, ..., 0] | {score} |
| Formal Verification (L4) | {status} | VP pass rate, fuzz coverage | {score} |
| Holdout Evaluation | {status} | mean satisfaction, must-pass rate | {score} |
| **L3 BC Convergence** | {status} | % BCs with tests + implementation | {score} |
| **L4 VP Convergence** | {status} | % VPs verified, none withdrawn unresolved | {score} |

## Overall: {CONVERGED / NOT_CONVERGED / OVERRIDDEN}

---

## Dimension 1: Spec Convergence

### Finding Novelty Across Passes

| Pass | New Findings | Duplicates | Novelty Score | Median Severity |
|------|-------------|------------|---------------|-----------------|
| 1 | {n} | {d} | {novelty} | {severity} |
| 2 | {n} | {d} | {novelty} | {severity} |
| ... | | | | |

### Assessment
{Narrative explaining spec convergence status with reference to thresholds from CONVERGENCE.md}

---

## Dimension 2: Test Convergence

### Mutation Kill Rates by Module Criticality

| Module | Criticality | Kill Rate | Target | Status |
|--------|------------|-----------|--------|--------|
| {module} | {tier} | {rate}% | {target}% | {MET/NOT_MET} |
| ... | | | | |

### Surviving Mutant Classification

| Category | Count | Percentage |
|----------|-------|------------|
| Equivalent mutant | {n} | {pct}% |
| Dead code | {n} | {pct}% |
| Insufficient assertions | {n} | {pct}% |
| Complex logic | {n} | {pct}% |

### Property-Based Test Coverage
- Cataloged invariants: {total}
- With property tests: {covered}
- Coverage: {pct}%

### Assessment
{Narrative explaining test convergence status}

---

## Dimension 3: Implementation Convergence

### Finding Decay Curve

| Pass | Total Findings | Verified Real | Verification Rate | Cost |
|------|---------------|---------------|-------------------|------|
| 1 | {n} | {n} | {rate}% | ${cost} |
| 2 | {n} | {n} | {rate}% | ${cost} |
| ... | | | | |

### Convergence Index Trend

| Pass | Novelty | Similarity | Median Severity | Cost | CI |
|------|---------|------------|-----------------|------|----|
| 1 | {val} | {val} | {val} | ${val} | {val} |
| 2 | {val} | {val} | {val} | ${val} | {val} |
| ... | | | | | |

### Power-Law Fit
- Exponent: {c}
- R-squared: {r2}
- Projected next-iteration findings: {projected}

### Assessment
{Narrative explaining implementation convergence status}

---

## Adversarial Convergence Metrics

### Finding Decay

| Pass | Findings | CRIT | HIGH | MED | LOW | Novelty Rate |
|------|----------|------|------|-----|-----|-------------|
| 1 | {n} | {n} | {n} | {n} | {n} | 1.000 |
| 2 | {n} | {n} | {n} | {n} | {n} | {rate} |
| 3 | {n} | {n} | {n} | {n} | {n} | {rate} |

### Convergence Assessment
- Decay pattern: {exponential / linear / plateau}
- Passes to convergence: {N}
- Final pass novelty: {0.000 = converged}
- Convergence index: novelty_rate = new_findings / total_possible
- Power-law fit parameters: exponent={c}, R²={r2} (if enough data points)

---

## Dimension 4: Verification Convergence

### Kani Proofs
- Total harnesses: {total}
- Passing: {passing}
- Proof core coverage: {pct}%

### Fuzz Testing
- Targets: {total}
- Saturated (no crashes in 5 min): {saturated}/{total}
- Last novel crash: {timestamp or "none"}

### Security Scans
- Semgrep critical findings: {n}
- Semgrep high findings: {n}
- Dependency CVEs: {n}

### Purity Audit
- Pure-core modules: {total}
- Verified effect-free: {verified}
- Violations: {n}

### Assessment
{Narrative explaining verification convergence status}

---

## Dimension 5: Holdout Scenario Convergence

### Holdout Satisfaction Scores

| Scenario | Category | Priority | Satisfaction | Confidence |
|----------|----------|----------|-------------|------------|
| {id} | {category} | {priority} | {score} | {confidence} |
| ... | | | | |

### Aggregate Metrics

| Metric | Value |
|--------|-------|
| Total scenarios | {n} |
| Must-pass scenarios | {n} |
| Mean satisfaction score | {value} |
| Satisfaction std dev | {value} |
| Must-pass minimum score | {value} |
| Stability (monotonic non-decreasing 2+ iterations) | {YES / NO} |

### Assessment
{Narrative explaining holdout convergence status}

---

## Dimension 6: L3 BC Convergence

### Behavioral Contract Coverage

| Subsystem | Total BCs | Implemented | Tested | Proven | Coverage |
|-----------|----------|-------------|--------|--------|----------|
| [subsystem] | [n] | [n] | [n] | [n] | [pct]% |

### BC Status Summary

| Status | Count | Percentage |
|--------|-------|------------|
| Fully verified | [n] | [pct]% |
| Implemented + tested | [n] | [pct]% |
| Implemented only | [n] | [pct]% |
| Not started | [n] | [pct]% |

### Assessment
{Narrative explaining L3 BC convergence status}

---

## Dimension 7: L4 VP Convergence

### Verification Property Status

| Status | Count | Percentage |
|--------|-------|------------|
| Proven | [n] | [pct]% |
| Pending | [n] | [pct]% |
| Failed | [n] | [pct]% |
| Withdrawn | [n] | [pct]% |

### VP-to-BC Coverage

| BC ID | VP Count | Proven | Gap |
|-------|----------|--------|-----|
| BC-S.SS.NNN | [n] | [n] | [description or "none"] |

### Withdrawn VP Impact

| VP-NNN | Replacement | Coverage Impact |
|--------|------------|----------------|
| [id] | [replacement VP or "none"] | [impact description] |

### Assessment
{Narrative explaining L4 VP convergence status}

---

## Cost-Benefit Analysis

| Metric | Value |
|--------|-------|
| Total pipeline cost (tokens/USD) | {total} |
| Cost per converged BC | {cost_per_bc} |
| Cost per verified VP | {cost_per_vp} |
| Adversarial passes to convergence | {passes} |
| EV of undetected defects | ${ev} |
| Threshold for release | {threshold} |
| MVR (Minimum Viable Rigor) assessment | {REACHED / NOT_REACHED} |

---

## Traceability Summary

| Artifact | Count |
|----------|-------|
| Spec requirements | {n} |
| Verification properties | {n} |
| Test cases | {n} |
| Implementation files | {n} |
| Adversarial review passes | {n} |
| Formal proofs | {n} |
| Adversarial findings (total) | {n} |
| Adversarial findings (resolved) | {n} |

---

## Human Override

| Field | Value |
|-------|-------|
| Override requested? | {yes / no} |
| Override reason | {text or N/A} |
| Dimensions overridden | {list or N/A} |
| Approver | {human name or N/A} |
| Date | {YYYY-MM-DD or N/A} |
