# Excalidraw Integration — Design Spec

## Summary

Add excalidraw diagram support to the visual companion with interactive editing, plus a standalone create-excalidraw skill for generating .excalidraw JSON files. Fix tiered visual tooling tables across all skills and docs.

## Architecture

The visual companion server gains file-type detection: `.html` files serve in the existing HTML frame, `.excalidraw` files serve a React app with the excalidraw component. A `screen.json` manifest enables composed multi-pane views. All files remain accessible via a collapsible history sidebar.

### Key Decisions

- **File-type detection (B):** Server detects `.html` vs `.excalidraw` by extension, routes to appropriate renderer
- **Always editable (A):** Excalidraw canvas is editable by default. User changes sync back via WebSocket. `viewModeEnabled` prop available for future C-mode (agent-controlled per screen)
- **Explicit setup (B):** `/vsdd-factory:visual-companion-setup` installs npm deps and runs Vite build. No auto-install.
- **History sidebar + split view:** Collapsible panel shows all past screens. Click to view, drag to split. Manifest (`screen.json`) for agent-controlled composed layouts.
- **Graceful fallback:** If `dist/` doesn't exist, `.excalidraw` files show a "run setup" message. HTML files always work.

## Files

### New
- `skills/visual-companion/setup.sh` — install npm deps + Vite build
- `skills/visual-companion/package.json` — react, react-dom, @excalidraw/excalidraw, vite
- `skills/visual-companion/vite.config.js` — minimal Vite config
- `skills/visual-companion/src/index.html` — Vite entry point
- `skills/visual-companion/src/main.jsx` — React mount
- `skills/visual-companion/src/App.jsx` — mode router (HTML vs excalidraw) + split view
- `skills/visual-companion/src/ExcalidrawView.jsx` — excalidraw component with WebSocket sync
- `skills/visual-companion/src/HistorySidebar.jsx` — collapsible file history
- `skills/create-excalidraw/SKILL.md` — standalone excalidraw JSON generation skill

### Modified
- `skills/visual-companion/SKILL.md` — add excalidraw docs, setup command
- `skills/visual-companion/visual-guide.md` — add excalidraw JSON generation guide
- `skills/visual-companion/scripts/server.cjs` — file-type detection, React dist serving, /api/files endpoint, drawing save-back
- `skills/visual-companion/scripts/helper.js` — sidebar toggle, split view, history navigation
- `skills/visual-companion/scripts/frame-template.html` — sidebar container + split layout CSS
- `skills/visual-companion/scripts/start-server.sh` — detect dist/ for excalidraw support

### Tiered table fixes (remove excalidraw-export, add correct tiers)
- `skills/brainstorming/SKILL.md`
- `skills/guided-brief-creation/SKILL.md`
- `skills/create-architecture/SKILL.md`
- `docs/guide/cross-cutting-skills.md`
- `docs/guide/phase-1-spec-crystallization.md`

## Tiered Visual Tooling (corrected)

| Tier | Tool | Best for |
|------|------|----------|
| 1 | visual-companion (HTML mode) | Interactive mockups, A/B choices, clickable layouts |
| 1 | visual-companion (excalidraw mode) | Architecture diagrams, entity relationships, interactive editing |
| 2 | create-excalidraw (standalone) | Generate .excalidraw files for offline viewing |
| 3 | Mermaid code blocks | Sequence diagrams, state machines, simple flows |
| 4 | ASCII/text | Wireframe sketches, comparisons |

## Non-Goals

- Replacing the HTML mockup capability — it stays fully intact
- Auto-installing npm dependencies — explicit setup required
- Shipping pre-built dist/ in the plugin — generated locally
- Real-time collaboration between multiple users — single-user agent+human workflow
