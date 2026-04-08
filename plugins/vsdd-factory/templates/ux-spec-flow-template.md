---
document_type: ux-spec-flow
flow_id: "FLOW-NNN"
flow_name: "[Flow Name]"
version: "1.0"
status: draft
producer: ux-designer
timestamp: YYYY-MM-DDTHH:MM:SS
phase: 1c
traces_to: UX-INDEX.md
screens: []
prd_requirements: []
flow_diagram: "flows/diagrams/FLOW-NNN-[name].svg"
flow_diagram_source: "flows/diagrams/FLOW-NNN-[name].excalidraw"
---

# Flow: [Flow Name] (FLOW-NNN)

> **Sharded UX flow (DF-021).** One file per interaction flow. Navigate via
> `UX-INDEX.md`. References screen files in `screens/` directory.

## Flow Diagram

![FLOW-NNN diagram](diagrams/FLOW-NNN-[name].svg)

> Created via Excalidraw MCP. Source: `diagrams/FLOW-NNN-[name].excalidraw`
> Edit at excalidraw.com or VS Code Excalidraw extension.

## Flow Steps

| Step | Screen | User Action | System Response |
|------|--------|-------------|----------------|
| 1 | SCR-NNN | [what user does] | [what system does] |
| 2 | SCR-NNN | [what user does] | [what system does] |

## Error Scenarios

| Error | Trigger | User Experience | Recovery |
|-------|---------|----------------|----------|
| [error] | [what causes it] | [what user sees] | [how to recover] |

## Screen References

| Screen | File | Role in Flow |
|--------|------|-------------|
| SCR-NNN | screens/SCR-NNN-[name].md | [entry/intermediate/exit] |
