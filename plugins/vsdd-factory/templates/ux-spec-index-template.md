---
document_type: ux-spec-index
version: "1.0"
status: draft
producer: ux-designer
timestamp: YYYY-MM-DDTHH:MM:SS
phase: 1c
inputs: [prd.md, domain-spec/L2-INDEX.md]
input-hash: "[md5]"
traces_to: prd.md
prd_version: "[version]"
design_system_version: "1.0"
screens: []
flows: []
---

# UX Specification: [Product Name]

> **Sharded artifact (DF-021).** This index contains global UX settings
> (design system refs, breakpoints, a11y, performance). Per-screen and
> per-flow details live in separate files under `screens/` and `flows/`.

## Screen Inventory

| SCR ID | Name | Purpose | Components | Complexity | Wireframe | File |
|--------|------|---------|-----------|-----------|-----------|------|
| SCR-001 | [name] | [purpose] | [n] | simple/complex | screens/wireframes/SCR-001-[name].png | screens/SCR-001-[name].md |

## Flow Inventory

| FLOW ID | Name | Screens Involved | Steps | File |
|---------|------|-----------------|-------|------|
| FLOW-001 | [name] | SCR-001, SCR-002 | [n] | flows/FLOW-001-[name].md |

## Cross-References

| If you need... | Read these together |
|----------------|-------------------|
| Implement a specific screen | UX-INDEX.md (globals) + screens/SCR-NNN-[name].md |
| Write E2E tests for a flow | flows/FLOW-NNN-[name].md + referenced screen files |
| Accessibility audit a screen | UX-INDEX.md (a11y checklist) + screens/SCR-NNN-[name].md |
| Full UX review | UX-INDEX.md + all screen and flow files |

---

## Design System References

### Component Contracts

| Component | Contract File | Usage |
|-----------|--------------|-------|
| [component] | design-system/components/contracts/[name].yaml | [where used] |

### Design Tokens

| Category | Token File |
|----------|-----------|
| Colors | design-system/tokens/colors.json |
| Typography | design-system/tokens/typography.json |
| Spacing | design-system/tokens/spacing.json |
| Sizing | design-system/tokens/sizing.json |

---

## Responsive Breakpoints

| Breakpoint | Width | Layout Strategy |
|-----------|-------|----------------|
| Mobile | 375px | Single column, stacked |
| Tablet | 768px | Two column, collapsible nav |
| Desktop | 1024px | Full layout, sidebar |
| Wide | 1440px | Max-width container, centered |

---

## Contextual Variants

| Variant | Trigger | Behavior |
|---------|---------|----------|
| Dark mode | `prefers-color-scheme: dark` | Swap to dark token set |
| Reduced motion | `prefers-reduced-motion: reduce` | Disable all animations |
| High contrast | `prefers-contrast: more` | Increase contrast ratios |
| Touch device | Touch detection | 48px min touch targets |

---

## Accessibility Checklist (Global)

- [ ] All interactive elements keyboard-accessible
- [ ] Focus indicators visible on all focusable elements
- [ ] No color-only indicators (shape/icon/text supplement)
- [ ] Heading hierarchy: no skipped levels
- [ ] All forms have explicit labels (not placeholder-as-label)
- [ ] All images have alt text (decorative: alt="")
- [ ] Focus management on modal open/close and page transitions
- [ ] prefers-reduced-motion respected globally
- [ ] ARIA landmarks on all major page sections
- [ ] Minimum 4.5:1 contrast ratio for normal text

---

## Performance Targets

| Metric | Target | Measurement |
|--------|--------|------------|
| LCP | < 2.5s | Largest Contentful Paint |
| FID | < 100ms | First Input Delay |
| CLS | < 0.1 | Cumulative Layout Shift |
| TTI | < 3.5s | Time to Interactive |
| Bundle size per route | < 200KB gzipped | Per-route JS bundle |
