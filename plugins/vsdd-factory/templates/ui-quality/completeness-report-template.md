# UI Completeness Report

**Product:** ${product.name}
**Phase:** ${phase}
**Date:** ${date}
**Checked by:** consistency-validator

## Summary

- **Screens:** ${specified} specified, ${implemented} implemented, ${verified} fully verified
- **Fidelity Score:** ${fidelity}%
- **Total Gaps:** ${gap_count}

## Screen-by-Screen Status

| Screen | Status | Components | States | Interactions | Responsive | A11y | Perf |
|--------|--------|-----------|--------|-------------|-----------|------|------|
| | | /N | /N | /N | /4 | P/F | P/F |

## Gaps Found

| # | Screen | Element | Type | Description | Fix Story |
|---|--------|---------|------|-------------|-----------|
| 1 | | | | | FIX-UI-NNN |

## State Coverage

| Component | Required States | Implemented | Tested | Gaps |
|-----------|----------------|-------------|--------|------|
| | N | N | N | N |

## Async State Coverage

| Component | Loading | Success | Empty | Error |
|-----------|---------|---------|-------|-------|
| | P/F | P/F | P/F | P/F |

## Responsive Coverage

| Screen | 375 | 768 | 1024 | 1440 | Screenshots |
|--------|-----|-----|------|------|-------------|
| | P/F | P/F | P/F | P/F | Y/N |

## Resolution Required Before Convergence

All gaps MUST be resolved. Each gap becomes a fix story (FIX-UI-NNN)
routed through code-delivery.lobster.

**Gate Result:** PASS / FAIL (zero gaps required for PASS)
