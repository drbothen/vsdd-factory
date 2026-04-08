---
name: design-system-bootstrap
description: >
  Bootstraps a design system for UI products. In greenfield: creates from
  product brief + brand guidelines. In brownfield: extracts from existing
  codebase. Produces design tokens, component registry, and constraints.
  Copies templates from templates/design-system/ into .factory/design-system/.
agents:
  primary: ux-designer
  supporting: [architect, codebase-analyzer, accessibility-auditor]
inputs:
  - Product brief or existing codebase
  - Brand guidelines (if provided by human)
outputs:
  - .factory/design-system/ (complete directory)
gate: Human reviews design system before implementation
condition: "feature_type in ['ui', 'full-stack']"
---

# Design System Bootstrap

## When This Skill Runs

- **Greenfield (Phase 1):** After spec crystallization, before story decomposition
- **Brownfield (Phase 0):** During codebase ingestion, before delta analysis
- **Feature Mode (F2):** Only if no design system exists yet

## Greenfield Flow

1. **Copy template structure:**
   Copy `templates/design-system/` into `.factory/design-system/`.
   This provides the token schema, component contracts, patterns, and constraints.

2. **ux-designer customizes tokens:**
   - Read product brief for brand guidelines (colors, fonts, voice)
   - If brand guidelines provided by human: use them to populate tokens
   - If no brand guidelines: bootstrap minimal design system
     (neutral palette, system fonts, 8px grid)
   - Customize colors.json with product-specific palette
   - Customize typography.json with product-specific fonts
   - Set product-specific spacing scale if non-standard

3. **accessibility-auditor validates tokens:**
   - Verify all color combinations meet WCAG AA contrast ratios
   - Verify touch target minimums specified correctly
   - Verify focus styles defined for all interactive states
   - Verify reduced-motion overrides present

4. **ux-designer creates component registry:**
   - Read UX spec screens/flows
   - Identify which components from the template registry are needed
   - Add product-specific components not in the template
   - Create component contracts for any new components

5. **architect reviews contracts:**
   - Verify component contracts are technically feasible
   - Verify performance budgets are realistic
   - Verify component boundaries support the architecture

6. **Human approves design system:**
   - Present complete `.factory/design-system/` for human review
   - Human may override colors, fonts, components, constraints
   - After approval, design system becomes the constraint layer

## Brownfield Flow

1. **codebase-analyzer extracts existing styles:**
   - Scan for CSS variables, theme files, design tokens
   - Scan for existing component library (React components, Vue components)
   - Identify existing patterns (layouts, forms, navigation)
   - Catalog color values, font sizes, spacing values in use

2. **ux-designer normalizes into token format:**
   - Map extracted values to design token schema
   - Fill gaps (missing tokens) from template defaults
   - Resolve inconsistencies (3 different blues -> 1 primary scale)

3. **accessibility-auditor flags non-compliant tokens:**
   - Color contrast failures
   - Missing focus styles
   - Touch target violations

4. **ux-designer creates component registry:**
   - Map existing components to contract format
   - Identify components without contracts (add them)
   - Flag inconsistent component patterns

5. **Human approves design system**

## Minimal Bootstrap (No Brand Guidelines)

When no brand guidelines are provided, create a minimal design system:
- **Colors:** Neutral gray palette + single blue primary
- **Typography:** System font stack (-apple-system, ...)
- **Spacing:** Standard 4px/8px grid
- **Components:** Full template registry
- **Patterns:** Full template patterns

Human can override any token after bootstrap.

## Output Structure

```
.factory/design-system/
  tokens/
    colors.json
    typography.json
    spacing.json
    sizing.json
    motion.json
    elevation.json
    accessibility.json
  components/
    component-registry.yaml
    contracts/
      button.yaml
      form-field.yaml
      modal.yaml
      navigation.yaml
      data-table.yaml
      card.yaml
      list.yaml
      toast.yaml
      alert.yaml
      dropdown.yaml
      tabs.yaml
      ... (product-specific)
  patterns/
    layout-patterns.yaml
    form-patterns.yaml
    navigation-patterns.yaml
  constraints.yaml
```

## Quality Gate

- [ ] Design tokens produced in `.factory/design-system/tokens/` (colors, typography, spacing at minimum)
- [ ] Component registry created in `.factory/design-system/components/component-registry.yaml`
- [ ] Accessibility contrast ratios validated (WCAG AA for all color combinations)
- [ ] Human review gate completed before design system is used downstream

## Agent Enforcement After Bootstrap

Once the design system exists in `.factory/design-system/`:
- **ux-designer:** UX specs reference tokens/components by name
- **implementer:** Uses token CSS variables, component contracts. Custom CSS logged.
- **test-writer:** Tests validate token usage and contract compliance
- **consistency-validator:** Design system compliance at every gate
