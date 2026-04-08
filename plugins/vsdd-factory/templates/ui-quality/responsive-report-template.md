# Responsive Validation Report

**Product:** ${product.name}
**Date:** ${date}
**Tested by:** e2e-tester

## Summary

- **Screens tested:** ${count}
- **Breakpoints:** 4 (375, 768, 1024, 1440)
- **Pass rate:** ${pass_rate}%
- **Critical failures:** ${critical_count}

## Per-Screen Results

| Screen | 375 (Mobile) | 768 (Tablet) | 1024 (Desktop) | 1440 (Wide) | Issues |
|--------|-------------|-------------|----------------|-------------|--------|
| | P/F | P/F | P/F | P/F | |

## Breakpoint-Specific Checks

### Mobile (375px)
| Check | Global Result | Failing Screens |
|-------|--------------|----------------|
| No horizontal scroll | P/F | |
| Touch targets >= 48px | P/F | |
| Text >= 16px | P/F | |
| Navigation usable | P/F | |
| Images not cropped | P/F | |
| Forms single column | P/F | |

### Tablet (768px)
| Check | Global Result | Failing Screens |
|-------|--------------|----------------|
| Layout adapts | P/F | |
| Sidebar behavior | P/F | |
| Tables scrollable/stacked | P/F | |
| Touch targets >= 48px | P/F | |

### Desktop (1024px)
| Check | Global Result | Failing Screens |
|-------|--------------|----------------|
| Full layout rendered | P/F | |
| Keyboard nav complete | P/F | |
| Hover states functional | P/F | |
| Multi-column correct | P/F | |

### Wide (1440px)
| Check | Global Result | Failing Screens |
|-------|--------------|----------------|
| Content max-width | P/F | |
| Whitespace appropriate | P/F | |
| Line length < 80ch | P/F | |

## Critical Failures

| # | Screen | Breakpoint | Issue |
|---|--------|-----------|-------|
| 1 | | | |

## Screenshots

All stored in `.factory/ui-evidence/`

**Gate Result:** PASS / FAIL
