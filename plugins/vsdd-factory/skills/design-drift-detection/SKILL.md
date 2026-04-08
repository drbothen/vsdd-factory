---
name: design-drift-detection
description: >
  Detects design system drift during maintenance sweeps. Scans for token
  overrides, component misuse, pattern violations, and emergent patterns.
  Runs as Sweep 10 in maintenance.lobster (UI products only).
agents:
  primary: ux-designer
  supporting: [consistency-validator]
inputs:
  - .factory/design-system/ (tokens, contracts, patterns)
  - Product source code (CSS, components)
outputs:
  - .factory/maintenance/design-drift.md
condition: "state.has_ui == true"
---

# Design Drift Detection

## Sweep 10: Design Drift (UI Products Only)

Runs weekly in the maintenance sweep alongside the existing 9 sweeps.

## Detection Checks

### 1. Token Override Scan
Find CSS that overrides design token values with hardcoded values:
- Custom color values not in the palette
- Custom spacing values not on the scale
- Custom font sizes not in the type scale
- Custom shadow values not in the elevation tokens
- Custom border-radius values not in the sizing tokens

### 2. Component Misuse Scan
Find components used with invalid prop combinations:
- Props not defined in the component contract
- Variants not in the contract's variant list
- Missing required props
- States implemented but not in the contract

### 3. Pattern Violation Scan
Find layouts that don't match approved patterns:
- Form layouts not following form-patterns.yaml
- Navigation patterns not following navigation-patterns.yaml
- Layout structures not in layout-patterns.yaml

### 4. New Pattern Detection (D15: Emergent Pattern Detection)
Find repeated structures not in the component registry:
- Cluster similar DOM tree shapes across codebase
- If cluster size >= 3 instances: propose as design system component
- Generate: proposed name, props interface, variants, states
- Present to human for approval
- If approved: add to design system, create refactoring stories

### 5. Style Consistency Scan
Find inconsistent usage across pages:
- Same semantic purpose using different tokens
- Inconsistent spacing between similar elements
- Typography styles varying for same content type

## Output

Write to `.factory/maintenance/design-drift.md`:

```markdown
## Design Drift Report

### Token Overrides: N found
| File | Line | Token | Override Value | Recommended Token |
|------|------|-------|---------------|-------------------|
| ... | ... | ... | ... | ... |

### Component Misuse: N found
| File | Component | Issue |
|------|-----------|-------|
| ... | ... | ... |

### Pattern Violations: N found
| File | Pattern | Violation |
|------|---------|-----------|
| ... | ... | ... |

### Emergent Patterns: N detected
| Pattern | Instances | Proposed Name | Action |
|---------|-----------|--------------|--------|
| ... | N | ... | Propose / Ignore |

### Style Inconsistencies: N found
| Issue | Files | Recommendation |
|-------|-------|---------------|
| ... | ... | ... |

### Fix Actions
- Token overrides: fix PRs to use tokens
- Component misuse: fix PRs with correct props
- Emergent patterns (if approved): add to design system + refactor
```

## Quality Gate

- [ ] All token overrides flagged with file, line, and recommended token
- [ ] All component misuses documented with file and issue description
- [ ] Emergent pattern scan completed (cluster threshold >= 3 instances)
- [ ] Design drift report written to `.factory/maintenance/design-drift.md`

## Failure Modes

- If design system directory (`.factory/design-system/`) not found: report "no design system detected", skip all checks, exit cleanly
- If source code contains no CSS or UI components: report "no UI artifacts found", skip scan
- If pattern definition files (e.g., `form-patterns.yaml`) are missing: skip that pattern check, note which patterns could not be validated
