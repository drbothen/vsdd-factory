---
name: visual-companion
description: >
  Browser-based visual companion for showing mockups, diagrams, and interactive
  options during brainstorming, brief creation, and architecture design. Runs a
  local Node.js server that watches for HTML files and serves them with live reload.
  Optional — requires Node.js.
---

# Visual Companion

Browser-based tool for showing visual content during early pipeline stages. The server watches a directory for HTML files, serves the newest one, and relays user interactions (clicks, selections) back as JSON events.

## Prerequisites

- Node.js must be installed (any version with ES2020 support)
- User must consent to opening a local URL

## Excalidraw Setup (optional)

To enable interactive excalidraw diagrams in the browser:

```bash
${CLAUDE_PLUGIN_ROOT}/skills/visual-companion/setup.sh
```

This installs React + @excalidraw/excalidraw and builds the viewer. Run once. Without this, `.excalidraw` files show a "run setup" message while HTML mockups continue to work.

## Starting a Session

Run the start script from the skill's scripts directory:

```bash
${CLAUDE_PLUGIN_ROOT}/skills/visual-companion/scripts/start-server.sh --project-dir <project-path>
```

Returns JSON:
```json
{"type":"server-started","port":52341,"url":"http://localhost:52341",
 "screen_dir":"<project>/.factory/visual-companion/<session>/content",
 "state_dir":"<project>/.factory/visual-companion/<session>/state"}
```

Save `screen_dir` and `state_dir`. Tell the user to open the URL.

## The Loop

1. **Check server is alive** — verify `$STATE_DIR/server-info` exists
2. **Write HTML** to a new file in `screen_dir` using the Write tool (never cat/heredoc)
   - Use semantic filenames: `layout.html`, `architecture.html`
   - Never reuse filenames — each screen gets a fresh file
   - Write content fragments (no `<!DOCTYPE>` needed) — server wraps in frame template
3. **Tell user** what's on screen, remind them of the URL, end your turn
4. **On next turn** — read `$STATE_DIR/events` for browser interactions, merge with terminal text
5. **Iterate or advance** — new file for revisions, next question when validated
6. **Unload** when returning to terminal — push a waiting screen to clear stale content

## Stopping

```bash
${CLAUDE_PLUGIN_ROOT}/skills/visual-companion/scripts/stop-server.sh <session_dir>
```

The server also auto-exits after 30 minutes of inactivity.

## When to Use vs Not

**Use the browser** for content that IS visual: mockups, wireframes, layout comparisons, architecture diagrams, side-by-side designs.

**Use the terminal** for content that is text: requirements, conceptual choices, tradeoff lists, scope decisions.

A question about a UI topic is not automatically a visual question. "What does this feature mean?" is conceptual — use the terminal. "Which of these layouts works better?" is visual — use the browser.

## Excalidraw Mode

When the agent writes an `.excalidraw` file to the content directory, the server automatically switches to excalidraw mode:

- The file renders as an interactive excalidraw canvas in the browser
- The user can drag, resize, add, and delete elements
- Changes sync back to the server via WebSocket and are saved to the file
- The agent reads the updated file on its next turn

Use excalidraw mode for architecture diagrams, entity relationships, flow charts, and any diagram the user might want to modify spatially.

Use HTML mode for mockups, A/B choices, and interactive options with click tracking.

## History Sidebar

All files (HTML and excalidraw) are accessible via a collapsible sidebar on the left. Click any past screen to view it. The newest file is active by default.

## Composed Views

Write a `screen.json` manifest to show multiple files side-by-side:

```json
{
  "layout": "split",
  "panes": [
    { "file": "arch-overview.excalidraw", "label": "Architecture" },
    { "file": "component-mockup.html", "label": "Mockup" }
  ]
}
```

## See Also

- `visual-guide.md` — CSS classes, event format, design tips
- `/vsdd-factory:excalidraw-export` — for static architecture diagrams
- `/vsdd-factory:create-excalidraw` — generate .excalidraw JSON files programmatically
