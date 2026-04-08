---
document_type: ux-spec-screen
screen_id: "SCR-NNN"
screen_name: "[Screen Name]"
version: "1.0"
status: draft
producer: ux-designer
timestamp: YYYY-MM-DDTHH:MM:SS
phase: 1c
complexity: simple|complex
traces_to: UX-INDEX.md
prd_requirements: []
wireframe: "screens/wireframes/SCR-NNN-[name].png"
wireframe_source: "screens/wireframes/SCR-NNN-[name].excalidraw"
---

# Screen: [Screen Name] (SCR-NNN)

> **Sharded UX screen (DF-021).** One file per screen. Navigate via
> `UX-INDEX.md`. Each screen targets 800-1,200 tokens. Complex screens
> (>5 interactive elements) may reach 1,500 tokens.

## Wireframe

![SCR-NNN wireframe](wireframes/SCR-NNN-[name].png)

> Created via Excalidraw MCP. Source: `wireframes/SCR-NNN-[name].excalidraw`
> Edit at excalidraw.com or VS Code Excalidraw extension.

## Purpose and User Context

[What this screen does and when the user sees it]

## Components

| ID | Component Contract | Variants | Required States | Async? |
|----|-------------------|----------|----------------|--------|
| CMP-001 | button.yaml | primary, secondary | default, hover, active, focus, disabled, loading | no |

## Elements

| ID | Type | Label | Validation | States |
|----|------|-------|-----------|--------|
| ELM-001 | text-input | [label] | [rules] | empty, filled, focused, error |

## Interactions

| ID | Trigger | Success Path | Error Paths | E2E Test Required |
|----|---------|-------------|-------------|-------------------|
| INT-001 | [user action] | [what happens] | [error scenarios] | yes/no |

## Accessibility

- **Tab order:** [numbered sequence of focusable elements]
- **ARIA labels:** [elements requiring explicit ARIA]
- **Keyboard shortcuts:** [if any]
- **Focus management:** [on modal/dialog/transition]
- **Heading hierarchy:** [H1 → H2 → H3 structure]

## Responsive Adaptations

| Breakpoint | Layout Change |
|-----------|--------------|
| 375px (mobile) | [how this screen adapts] |
| 768px (tablet) | [how this screen adapts] |
| 1024px (desktop) | [default layout] |
| 1440px (wide) | [max-width behavior] |
