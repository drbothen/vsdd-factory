---
name: responsive-validation
description: >
  Automated responsive testing at 4+ breakpoints for every screen.
  Captures screenshots, validates breakpoint-specific rules, and stores
  evidence in .factory/ui-evidence/.
agents:
  primary: e2e-tester
  supporting: [visual-reviewer]
inputs:
  - .factory/ui-traceability.yaml (screen list)
  - .factory/design-system/tokens/sizing.json (breakpoints)
outputs:
  - .factory/ui-evidence/SCR-NNN/ (screenshots per breakpoint)
  - .factory/ui-quality/responsive-report.md
condition: "feature_type in ['ui', 'full-stack']"
---

# Responsive Design Validation

## When It Runs

- **Per-story** (in code-delivery.lobster, after implementation)
- **Per-wave** (at wave gate)
- **Before convergence** (full responsive suite)

## Breakpoint Test Suite

### Mobile (375px)
- No horizontal scroll
- Touch targets >= 48px
- Text readable (min 16px)
- Navigation usable (hamburger or bottom nav)
- Images not cropped or overflowing
- Forms single column

### Tablet (768px)
- Layout adapts (not just scaled mobile)
- Sidebar behavior correct (collapsed or visible)
- Data tables scrollable or stacked
- Touch targets >= 48px

### Desktop (1024px)
- Full layout rendered
- Keyboard navigation complete
- Hover states functional
- Multi-column layouts correct

### Wide (1440px)
- Content max-width enforced (no extreme stretching)
- Whitespace appropriate
- Readability maintained (line length < 80ch for prose)

## Implementation

e2e-tester runs responsive validation using Playwright:

```
For each screen in ui-traceability.yaml:
  For each breakpoint [375, 768, 1024, 1440]:
    1. Resize viewport to breakpoint width
    2. Run breakpoint-specific test assertions
    3. Capture full-page screenshot
    4. Store screenshot in .factory/ui-evidence/SCR-NNN/
    5. Check against design spec layout
    6. Report pass/fail per test
```

## Screenshot Storage

```
.factory/ui-evidence/
  SCR-001/
    mobile-375.png
    tablet-768.png
    desktop-1024.png
    wide-1440.png
    CMP-001-default.png
    CMP-001-loading.png
    CMP-001-error.png
    ...
```

## Critical Failures (Blocking)

- Horizontal scroll at any breakpoint
- Touch targets < 48px on mobile/tablet
- Text unreadable at any breakpoint (< 14px)
- Navigation unusable at any breakpoint
- Content overflow or cropping

## Report Format

Output to `.factory/ui-quality/responsive-report.md`:

```markdown
## Responsive Validation Report

### Summary
- Screens tested: N
- Breakpoints: 4 (375, 768, 1024, 1440)
- Pass rate: N%

### Per-Screen Results
| Screen | 375 | 768 | 1024 | 1440 | Issues |
|--------|-----|-----|------|------|--------|
| ...    | P/F | P/F | P/F  | P/F  | ...    |

### Critical Failures
1. [Screen]: [Issue at breakpoint]

### Screenshots
All stored in .factory/ui-evidence/
```

## Quality Gate

- [ ] All 4 breakpoints (375, 768, 1024, 1440) tested for every screen
- [ ] Screenshots captured and stored in `.factory/ui-evidence/SCR-NNN/` per screen
- [ ] Breakpoint-specific rules validated (touch targets, scroll, text size, layout)
- [ ] Responsive report produced with per-screen pass/fail matrix

## Failure Modes

- If Playwright cannot resize to a breakpoint: log the failure, skip that breakpoint, and flag in report
- If screenshot capture fails: retry once, then record as missing evidence in report
- If a screen is not reachable (auth required, broken route): flag as untestable and report to orchestrator
