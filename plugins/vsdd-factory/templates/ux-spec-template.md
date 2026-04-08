<!-- DEPRECATED: This monolithic template has been replaced by the sharded
     ux-spec-index-template.md + ux-spec-screen-template.md + ux-spec-flow-template.md.
     Use the sharded templates for new projects. Retained for reference. -->
---
document_type: ux_spec
version: "1.0"
prd_version: "1.0"
design_system_version: "1.0"
status: draft
---

# UX Specification: [Product Name]

## 1. Design System References

> **DF-037 D1:** All components and styles MUST reference design system tokens
> and component contracts by name. Do NOT specify arbitrary colors, sizes, or
> fonts. Use token names (e.g., "color-primary-600", "spacing.4", "Button/primary").

| Component | Contract | Variants Used |
|-----------|----------|--------------|

### Design Tokens Used
| Category | Token | Usage |
|----------|-------|-------|

## 2. Screen Definitions

### Screen: [Screen Name]

**ID:** SCR-NNN
**Purpose:** [What this screen does]
**User Context:** [Who sees this and when]
**Complexity:** simple | complex (>5 interactive elements triggers multi-variant, D5)

#### Components

| Component ID | Contract | Variants | Required States | Async? |
|-------------|----------|----------|----------------|--------|

> **DF-037 D4:** Components that fetch data MUST list all 4 async states:
> loading (skeleton), success, empty (with CTA), error (with retry).

#### Elements

| Element ID | Type | Label | Validation | States |
|-----------|------|-------|-----------|--------|

#### Interactions

| ID | Trigger | Success Path | Error Paths | E2E Test Required |
|----|---------|-------------|-------------|-------------------|

#### Accessibility
- Tab order: [list]
- ARIA labels: [list]
- Keyboard shortcuts: [list]
- Focus management: [modals, page transitions]
- Heading hierarchy: [h1 > h2 > h3, no skipped levels]

## 3. Interaction Flows

### Flow: [Flow Name]

| Step | Screen | User Action | System Response |
|------|--------|------------|-----------------|

### Error Scenarios

| Error | Trigger | User Experience | Recovery |
|-------|---------|----------------|----------|

## 4. Responsive Breakpoints (DF-037 D7)

> Breakpoints from design system tokens (sizing.json). All screens MUST be
> validated at all 4 breakpoints with screenshots stored in .factory/ui-evidence/.

| Breakpoint | Width | Layout Changes | Navigation | Special Rules |
|-----------|-------|---------------|------------|--------------|
| Mobile | 375px | | | Touch targets >= 48px, single column forms |
| Tablet | 768px | | | Touch targets >= 48px |
| Desktop | 1024px | | | Full layout, hover states, keyboard nav |
| Wide | 1440px | | | Content max-width, line length < 80ch |

## 5. Contextual Variants (DF-037 D13)

| Context | Adaptation | Implementation |
|---------|-----------|----------------|
| Dark mode | Token swap (light -> dark tokens) | CSS custom properties swap |
| Reduced motion | Disable/simplify animations | prefers-reduced-motion media query |
| High contrast | Increased contrast, thicker borders | prefers-contrast: more |
| Touch device | Larger touch targets (48px min) | Viewport/pointer media queries |

## 6. Accessibility Checklist (DF-037 D9)

> **Accessibility-first generation:** These constraints are embedded in component
> contracts and enforced during generation, not just audited after.

- [ ] WCAG 2.1 AA color contrast (4.5:1 minimum, AAA where feasible)
- [ ] All images have alt text
- [ ] All form inputs have explicit labels (NOT placeholder-as-label)
- [ ] Focus indicators visible on all interactive elements (focus ring token)
- [ ] Screen reader tested (ARIA landmarks, live regions)
- [ ] Keyboard-only navigation tested (Tab, Enter, Space, Escape, Arrow keys)
- [ ] Focus management for modals (trap + return to trigger)
- [ ] Color is never the sole indicator (icon + text paired with color)
- [ ] Motion safety (prefers-reduced-motion respected globally)
- [ ] Touch targets >= 48px on mobile/tablet
- [ ] Heading hierarchy (no skipped levels, one h1 per page)
- [ ] Semantic HTML used (button, nav, main, form, not div-for-everything)

## 7. Performance Targets (DF-037 D8)

| Metric | Target |
|--------|--------|
| LCP | < 2.5s |
| FID | < 100ms |
| CLS | < 0.1 |
| TTI | < 3.8s |
| Bundle size per route | < 200KB JS |

### Perceived Performance
- [ ] Skeleton screens for every data-fetching view
- [ ] Loading indicators for every async action
- [ ] Optimistic updates where safe
- [ ] Progressive loading (critical content first)
- [ ] Images: WebP, responsive srcset, lazy loading, blur placeholder
