---
document_type: delta-analysis-report
feature_name: "[Feature Name]"
created: YYYY-MM-DD
spec_version_at_analysis: "X.Y.Z"
status: draft
intent: "feature | enhancement | bug-fix"
feature_type: "ui | backend | full-stack | infrastructure"
scope: "trivial | standard"
severity: "N/A | CRITICAL | HIGH | MEDIUM | LOW"
---

# Delta Analysis Report: [Feature Name]

## Feature Request

- **Brief:** [Link to feature request document or inline summary]
- **Requested by:** [Human / ticket ID]
- **Date:** YYYY-MM-DD

## Classifications

### Intent Classification

| Intent | Detection Signals | Route |
|--------|------------------|-------|
| `feature` | Human says "add", "build", "new" | Full F1-F7 |
| `enhancement` | Human says "improve", "update", "change" | Full F1-F7 (may be quick dev if trivial) |
| `bug-fix` | Human says "fix", "bug", "broken", "regression" | Bug fix route (skip F2, F3) |

**Classified intent:** `[feature | enhancement | bug-fix]`
**Rationale:** [Why this classification]

### Feature Type Classification

| Type | Description |
|------|-------------|
| `ui` | Primarily front-end changes (screens, components, styles) |
| `backend` | API, database, service logic only |
| `full-stack` | Both UI and backend |
| `infrastructure` | CI/CD, tooling, config only |

**Classified type:** `[ui | backend | full-stack | infrastructure]`
**Rationale:** [Why this classification]

### Trivial Scope Classification

A change is trivial when ALL of the following are true:

- [ ] Impact boundary: single module, single file, or documentation only
- [ ] No new BCs needed
- [ ] No architecture change
- [ ] No new external dependencies
- [ ] Regression risk: LOW

**Classified scope:** `[trivial | standard]`
**Rationale:** [Why this classification]

> If trivial: quick dev routing applies (F1 -> F4 single story -> regression -> F7 lite -> PATCH release)

### Severity Classification (bug-fix intent only)

| Severity | Criteria |
|----------|----------|
| CRITICAL | Production down, data loss, security breach |
| HIGH | Major functionality broken, no workaround |
| MEDIUM | Functionality impaired, workaround exists |
| LOW | Minor issue, cosmetic, edge case |

**Classified severity:** `[N/A | CRITICAL | HIGH | MEDIUM | LOW]`

> If CRITICAL: expedited flow applies (minimal gates, async human approval)

## Impact Assessment

| Dimension | Affected | Details |
|-----------|----------|---------|
| PRD Requirements | N new, M modified | [List IDs: BC-S.SS.NNN new; BC-S.SS.NNN modified] |
| Architecture | N components added, M modified | [List: NewComponent (new), ExistingComponent (modified)] |
| UX Screens | N new, M modified | [List screen names] |
| Stories | N new stories estimated | [Brief description of each] |
| Existing Tests | N tests in regression risk zone | [List test files/modules touching affected code] |
| Verification Properties | N new proofs needed | [List: VP-NNN description] |

## Files Changed

### New Files

| File Path | Purpose |
|-----------|---------|
| `src/path/to/new_file.rs` | [Brief purpose] |

### Modified Files

| File Path | Change Type | Risk |
|-----------|------------|------|
| `src/path/to/existing.rs` | Interface change | HIGH |
| `src/path/to/other.rs` | Internal logic | MEDIUM |

### Dependent Files (unchanged but depend on modified files)

| File Path | Depends On | Regression Risk |
|-----------|-----------|----------------|
| `src/path/to/dependent.rs` | `existing.rs` | MEDIUM |

## Files NOT Changed (Regression Baseline)

These files must not be modified during implementation. All their tests
must continue to pass after implementation.

- `src/core/*` -- [rationale: pure core, unrelated to feature]
- `src/services/unrelated_service.rs` -- [rationale: no dependency on changed code]
- [List all unchanged areas]

## Risk Assessment

| Risk Type | Level | Rationale |
|-----------|-------|-----------|
| Regression | HIGH/MEDIUM/LOW | [Why] |
| Architecture | HIGH/MEDIUM/LOW | [Why] |
| Security | HIGH/MEDIUM/LOW | [Why] |
| Performance | HIGH/MEDIUM/LOW | [Why] |

## Regression Baseline

- **Total existing tests:** N
- **Tests in risk zone:** M (tests touching affected modules)
- **Risk zone test files:** [list]

## Scope Recommendation

- **Mode:** Feature Mode / Full Pipeline (if delta too large)
- **Estimated new stories:** N
- **Estimated effort:** [points or days]
- **Can parallelize:** [which stories are independent]

## Open Questions

- [Questions for human to resolve before proceeding to F2]
