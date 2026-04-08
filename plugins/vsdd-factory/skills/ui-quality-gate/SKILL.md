---
name: ui-quality-gate
description: >
  Comprehensive UI quality gate that validates all dimensions: design system
  compliance, completeness, heuristics, accessibility, responsive, performance,
  visual regression, and state coverage. Strictness scales by pipeline point.
agents:
  primary: consistency-validator
  supporting: [accessibility-auditor, e2e-tester, performance-engineer, ux-designer, visual-reviewer]
inputs:
  - .factory/design-system/ (tokens, contracts, constraints)
  - .factory/ui-traceability.yaml
  - .factory/ui-quality/ (all quality reports)
  - Implemented source code
outputs:
  - .factory/ui-quality/gate-report.md
condition: "feature_type in ['ui', 'full-stack']"
---

# UI Quality Gate

## Quality Gate

### Design System Compliance
- [ ] Token compliance: all styles use design tokens (no hardcoded values)
- [ ] Component compliance: all components match contracts
- [ ] Pattern compliance: layouts use approved patterns

### Completeness
- [ ] Screen coverage: all specified screens implemented
- [ ] State coverage: all required states implemented per component contract
- [ ] Interaction coverage: all specified interactions functional
- [ ] Responsive coverage: all breakpoints tested with screenshots

### Quality
- [ ] Heuristic score: >= 0.7 on all 10 Nielsen heuristics
- [ ] Task completion: >= 0.8 on all key user tasks
- [ ] A11y compliance: zero axe-core violations
- [ ] Keyboard navigable: all pages fully keyboard navigable
- [ ] Performance targets: LCP < 2.5s, FID < 100ms, CLS < 0.1

### Visual
- [ ] Visual regression: no unintended visual changes
- [ ] Responsive validation: no horizontal scroll, touch targets valid
- [ ] Screenshot evidence: screenshots at all breakpoints for all screens

### States
- [ ] Loading states: every async view has skeleton/spinner
- [ ] Error states: every async view has error + retry
- [ ] Empty states: every list/table has empty state

## Gate Strictness by Pipeline Point

| Pipeline Point | Checks Applied | Failure Mode |
|---------------|---------------|-------------|
| Per-story (code-delivery) | Token + a11y + component test | Block merge |
| Wave gate | Above + responsive + performance + states | Block next wave |
| Build verification | All checks | Block release |
| Before convergence | 100% on every dimension | Block convergence |

## Per-Story Gate (Lightweight)

Run during code-delivery.lobster after implementation:
1. Design system token compliance (no hardcoded CSS)
2. Accessibility: axe-core zero violations
3. Component test: run-story-tests via Storybook MCP (if available)
4. State coverage: required states from component contract implemented

## Wave Gate (Standard)

Run at each wave gate:
1. All per-story checks
2. Responsive validation at 4 breakpoints
3. Performance: Lighthouse CI (LCP, FID, CLS, TTI)
4. State coverage: all async states (loading/success/empty/error)
5. UI completeness check (partial -- gap report)

## Build Verification Gate (Full)

Run before release:
1. All wave gate checks
2. Heuristic evaluation score >= 0.7
3. Task completion analysis score >= 0.8
4. Visual regression (comparison against baseline)
5. Full keyboard navigation test
6. Screen reader scenario testing

## Convergence Gate (Complete)

Run before convergence sign-off:
1. All build verification checks
2. UI completeness: ZERO gaps in traceability matrix
3. Fidelity: 100% (all specified elements implemented)
4. All screenshots captured at all breakpoints
5. Performance trend analysis (not getting slower)
6. Design drift check (no token overrides)

## Failure Modes

When a check fails:
1. Log failure in `.factory/ui-quality/gate-report.md`
2. Create fix story: FIX-UI-NNN
3. Route through code-delivery.lobster
4. Re-run gate after fix

## Performance Targets (D8)

| Metric | Target | Measurement |
|--------|--------|------------|
| LCP (Largest Contentful Paint) | < 2.5s | Lighthouse CI |
| FID (First Input Delay) | < 100ms | Lighthouse CI |
| CLS (Cumulative Layout Shift) | < 0.1 | Lighthouse CI |
| TTI (Time to Interactive) | < 3.8s | Lighthouse CI |
| Bundle size per route | < 200KB JS | Build analysis |
| Image optimization | WebP + responsive + lazy | Asset audit |

### Perceived Performance Requirements
- Skeleton screens for every data-fetching view
- Loading indicators for every async action
- Optimistic updates where safe
- Progressive loading (critical content first)
- Image optimization: WebP, responsive srcset, lazy loading, blur placeholder

## Report Format

Output to `.factory/ui-quality/gate-report.md`:

```markdown
## UI Quality Gate Report

### Gate Level: [per-story | wave | build | convergence]
### Result: [PASS | FAIL]

### Checklist
| Check | Status | Details |
|-------|--------|---------|
| Token compliance | PASS/FAIL | ... |
| ... | ... | ... |

### Failures
1. [Check]: [Issue] -> FIX-UI-NNN

### Performance Metrics
| Metric | Value | Target | Status |
|--------|-------|--------|--------|
| LCP | Ns | < 2.5s | PASS/FAIL |
| ... | ... | ... | ... |
```
