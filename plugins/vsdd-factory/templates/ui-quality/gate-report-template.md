# UI Quality Gate Report

**Product:** ${product.name}
**Gate Level:** ${gate_level} (per-story | wave | build | convergence)
**Date:** ${date}
**Result:** PASS / FAIL

## Checklist

### Design System Compliance
| Check | Status | Details |
|-------|--------|---------|
| Token compliance | | |
| Component compliance | | |
| Pattern compliance | | |

### Completeness
| Check | Status | Details |
|-------|--------|---------|
| Screen coverage | | /N |
| State coverage | | /N |
| Interaction coverage | | /N |
| Responsive coverage | | /4 breakpoints |

### Quality
| Check | Status | Details |
|-------|--------|---------|
| Heuristic score >= 0.7 | | /1.0 |
| Task completion >= 0.8 | | /1.0 |
| A11y compliance (zero violations) | | N violations |
| Keyboard navigable | | |
| Performance targets met | | |

### Visual
| Check | Status | Details |
|-------|--------|---------|
| Visual regression | | |
| Responsive validation | | |
| Screenshot evidence | | |

### States
| Check | Status | Details |
|-------|--------|---------|
| Loading states | | /N components |
| Error states | | /N components |
| Empty states | | /N components |

## Performance Metrics

| Metric | Value | Target | Status |
|--------|-------|--------|--------|
| LCP | | < 2.5s | |
| FID | | < 100ms | |
| CLS | | < 0.1 | |
| TTI | | < 3.8s | |
| Bundle size | | < 200KB/route | |

## Failures

| # | Check | Issue | Fix Story |
|---|-------|-------|-----------|
| 1 | | | FIX-UI-NNN |

## Gate Decision

**Result:** PASS / FAIL
**Blocking Issues:** N
**Fix Stories Created:** N
