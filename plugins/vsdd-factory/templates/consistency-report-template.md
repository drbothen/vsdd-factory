---
document_type: consistency-report
level: ops
version: "1.0"
status: "pass|fail"
producer: consistency-validator
timestamp: YYYY-MM-DDTHH:MM:SS
phase: 2
inputs: [domain-spec/L2-INDEX.md, prd.md, architecture.md, "stories/"]
input-hash: "[md5]"
traces_to: prd.md
---

# Consistency Validation Report: [Project Name]

## Report Metadata

| Field | Value |
|-------|-------|
| **Product** | [product name] |
| **Generated** | YYYY-MM-DDTHH:MM:SS |
| **Generator** | consistency-validator |
| **Artifacts Scanned** | [n] |

## Summary

| # | Check | Result |
|---|-------|--------|
| 1 | L2 to L3 Requirement Coverage | pass / fail |
| 2 | L3 to L4 Verification Property Coverage | pass / fail |
| 3 | Dependency Acyclicity | pass / fail |
| 4 | Architecture Alignment | pass / fail |
| 5 | Acceptance Criteria Quality | pass / fail |
| 6 | Story Sizing (all <= 13 points) | pass / fail |
| 7 | Priority Consistency | pass / fail |
| 8 | L1 to L2 to L3 to L4 Chain Completeness | pass / fail |
| 9 | AC Completeness Coverage | pass / fail |
| 10 | ASM/R Traceability | pass / fail |

## 1. L2 to L3 Requirement Coverage

### 1.1 Domain Capabilities to Behavioral Contracts

| CAP-NNN | Description | Covered by BC-NNN? | Gap? |
|---------|-------------|-------------------|------|
| [CAP-001] | [capability description] | [BC-S.SS.NNN] | [yes/no] |

## 2. L3 to L4 Verification Property Coverage

### 2.1 Behavioral Contracts to Verification Properties

| BC-S.SS.NNN | Description | VP-NNN? | Justification if no VP |
|-------------|-------------|---------|----------------------|
| [BC-1.01.001] | [contract description] | [VP-001] | [why no VP if missing] |

## 3. Dependency Acyclicity

### 3.1 Topological Order

[List stories in topological order. If a cycle is detected, list the cycle members.]

### 3.2 Critical Path

[Longest dependency chain from first story to last. This determines minimum wall-clock
time for serial execution.]

## 4. Architecture Alignment

### 4.1 Module Coverage

| Architecture Component | Stories Covering It | Coverage |
|-----------------------|--------------------|---------:|
| [component] | [STORY-NNN, ...] | [full/partial/none] |

### 4.2 Component Consistency

[Verify that story architecture mappings match the declared architecture.
Flag stories that reference undeclared components or modules.]

## 5. Acceptance Criteria Quality

### 5.1 Concreteness

| Story | AC Count | Vague ACs | Untestable ACs |
|-------|----------|-----------|----------------|
| [STORY-NNN] | [n] | [list] | [list] |

### 5.2 Testability

[Every AC must be testable by the test-writer. Flag ACs that are subjective,
unmeasurable, or require human judgment.]

## 6. Story Sizing

| Story | Points | Status |
|-------|-------:|--------|
| [STORY-NNN] | [n] | [ok / over-13 / missing] |

## 7. Priority Consistency

[Verify that P0 stories have no unresolved dependencies on P1/P2 stories.
Verify that blocking stories have equal or higher priority than the stories they block.]

## 8. L1 to L2 to L3 to L4 Chain Completeness

> Every L1 brief section must trace to L2 CAP, every CAP to BC, every BC to story.
> Gaps must have explicit justification in Gap Register.

### L1 to L2 to L3 to L4 Chain Overview

| Level | Artifact | Count | Traced Forward | Traced Backward | Coverage |
|-------|----------|-------|---------------|----------------|----------|
| L1 | Product Brief sections | [n] | [n] to L2 | N/A | [pct]% |
| L2 | Domain Capabilities (CAP-NNN) | [n] | [n] to L3 | [n] to L1 | [pct]% |
| L3 | Behavioral Contracts (BC-S.SS.NNN) | [n] | [n] to L4 | [n] to L2 | [pct]% |
| L4 | Verification Properties (VP-NNN) | [n] | N/A | [n] to L3 | [pct]% |

### Broken Chains

| Gap ID | From | To | Missing Link | Impact | Priority |
|--------|------|----|-------------|--------|----------|
| CHAIN-001 | [source artifact] | [expected target] | [what is missing] | [impact] | [P0/P1/P2] |

### Orphaned Artifacts

| Artifact | Level | Issue | Resolution |
|----------|-------|-------|------------|
| [artifact ID] | [level] | [no parent trace / no child trace] | [action needed] |

## 9. AC Completeness Coverage

> Verifies that story ACs completely cover all specified behavioral contracts,
> edge cases, error paths, and cross-cutting requirements. Three levels of checks.
> Gate threshold: >= 90% weighted overall.

### 9.1 BC Clause Coverage (Level 1)

| BC-S.SS.NNN | Total Clauses | Covered | Uncovered | Gap Entries | Coverage % |
|-------------|---------------|---------|-----------|-------------|------------|
| [BC-1.01.001] | [n] | [n] | [n] | [n] | [pct]% |

**L1 Score:** [pct]%

### 9.2 Edge Case & Error Coverage (Level 2)

| Source | Total IDs | Covered | Orphaned | Coverage % |
|--------|-----------|---------|----------|------------|
| BC Edge Cases (EC-NNN) | [n] | [n] | [n] | [pct]% |
| Error Taxonomy (E-xxx-NNN) | [n] | [n] | [n] | [pct]% |

**L2 Score:** [pct]%

### 9.3 Cross-Cutting Coverage (Level 3)

| Category | Total | Covered | Uncovered | Coverage % |
|----------|-------|---------|-----------|------------|
| NFR-NNN (P0/P1) | [n] | [n] | [n] | [pct]% |
| Holdout-BC Alignment | [n] clauses | [n] aligned | [n] misaligned | [pct]% |
| UI Component States | [n] states | [n] covered | [n] missing | [pct]% |

**L3 Score:** [pct]%

### 9.4 AC Completeness Summary

| Level | Weight | Score | Weighted |
|-------|--------|-------|----------|
| L1 -- BC Clause Coverage | 50% | [pct]% | [weighted]% |
| L2 -- Edge Case & Error Coverage | 30% | [pct]% | [weighted]% |
| L3 -- Cross-Cutting Coverage | 20% | [pct]% | [weighted]% |
| **Overall** | **100%** | | **[total]%** |

**Gate Result:** [PASS / FAIL] (threshold: >= 90% weighted overall)

## 10. ASM/R Traceability

> Validates that assumptions and risks from the L2 Domain Spec are properly traced,
> covered by holdout scenarios and stories, and have consistent status across artifacts.

### 10.1 Assumption Coverage

| ASM-NNN | Description | Status | Traced To | Holdout? | Story? | Coverage |
|---------|-------------|--------|-----------|----------|--------|----------|
| [ASM-001] | [assumption] | [unvalidated/validated/invalidated] | [artifact IDs] | [HS-NNN or --] | [STORY-NNN or --] | [full/partial/none] |

### 10.2 Risk Register Coverage

| R-NNN | Description | Status | Category | Impact | Traced To | NFR? | Architecture? | Security? | Coverage |
|-------|-------------|--------|----------|--------|-----------|------|---------------|-----------|----------|
| [R-001] | [risk] | [open/mitigated/accepted/closed] | [category] | [H/M/L] | [artifact IDs] | [NFR-NNN or --] | [section or --] | [SEC-NNN or --] | [full/partial/none] |

### 10.3 ASM/R Gate Summary

| Metric | Value | Threshold | Status |
|--------|-------|-----------|--------|
| HIGH-impact ASMs with holdout scenario | [n/total] | 100% | [pass/fail] |
| Testable ASMs with story + assumption_validations | [n/total] | 100% | [pass/fail] |
| HIGH-impact R-NNNs with architecture mitigation | [n/total] | 100% | [pass/fail] |
| Security R-NNNs in security review scope | [n/total] | 100% | [pass/fail] |
| R-NNN NFR candidates with corresponding NFR | [n/total] | 100% | [pass/fail] |
| HIGH/HIGH R-NNNs with holdout scenario | [n/total] | 100% | [pass/fail] |
| Unvalidated ASMs after Phase 3 | [n] | 0 | [pass/fail] |
| Invalidated ASMs with risk escalation | [n/total] | 100% | [pass/fail] |
| R-NNN Traced To bidirectional consistency | [n/total] | 100% | [pass/fail] |

## Cross-Reference Validation

### ID Consistency

| Check | Status | Issues |
|-------|--------|--------|
| BC IDs unique | [pass/fail] | [duplicate IDs if any] |
| VP IDs unique | [pass/fail] | [duplicate IDs if any] |
| CAP IDs unique | [pass/fail] | [duplicate IDs if any] |
| BC traces to valid CAP | [pass/fail] | [broken references] |
| VP traces to valid BC | [pass/fail] | [broken references] |
| Story ACs trace to valid BC | [pass/fail] | [broken references] |

### Naming Convention Compliance

| Convention | Expected Pattern | Violations |
|-----------|-----------------|------------|
| BC naming | BC-S.SS.NNN | [list violations] |
| VP naming | VP-NNN | [list violations] |
| CAP naming | CAP-NNN | [list violations] |
| Error taxonomy | E-xxx-NNN | [list violations] |

### Canonical Frontmatter Validation

| Artifact | document_type | level | version | producer | traces_to | Status |
|----------|--------------|-------|---------|----------|-----------|--------|
| [file] | [present/missing] | [present/missing] | [present/missing] | [present/missing] | [present/missing] | [pass/fail] |

## Spec vs Implementation Drift

| Artifact | Spec Version | Implementation State | Drift Detected | Notes |
|----------|-------------|---------------------|---------------|-------|
| [artifact] | [version] | [current/outdated/missing] | [yes/no] | [description] |

## Findings

### Critical
[Findings that block pipeline progression. Must be resolved before Phase 3.]

### Major
[Findings that degrade quality but do not block. Should be resolved before Phase 3.]

### Minor
[Findings that are cosmetic or low-impact. May be deferred.]

## Validation Gate Result

**[PASS / FAIL]** -- [list blocking findings if FAIL]

## Overall Metrics

| Metric | Value |
|--------|-------|
| **Total Checks** | [n] |
| **Passed** | [n] |
| **Failed** | [n] |
| **Warnings** | [n] |
| **Overall Status** | [consistent / inconsistencies-found / critical-gaps] |

[Narrative summary of consistency findings and recommended actions.]

## Appendix: Validation Methodology

[Describe the validation approach, tools used, and any assumptions made.
Reference the consistency-validator agent's AGENTS.md for the full list
of validation criteria.]
