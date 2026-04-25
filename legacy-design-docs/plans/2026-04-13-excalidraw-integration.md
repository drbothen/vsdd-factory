# Excalidraw Integration Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add interactive excalidraw diagram support to the visual companion, create a standalone excalidraw JSON generation skill, and fix tiered visual tooling tables across all skills and docs.

**Architecture:** The visual companion server gains file-type detection — `.html` files serve in the existing HTML frame, `.excalidraw` files serve a Vite-built React app with the @excalidraw/excalidraw component. A history sidebar shows all past screens. User edits sync back via WebSocket. A separate `create-excalidraw` skill generates .excalidraw JSON files independently of the browser companion.

**Tech Stack:** React 18, @excalidraw/excalidraw v0.18+, Vite (bundler), Node.js (existing server)

**Spec:** `.factory/specs/2026-04-13-excalidraw-integration-design.md`

---

### Task 1: Create the React app scaffold (package.json, vite.config.js, src/)

**Files:**
- Create: `plugins/vsdd-factory/skills/visual-companion/package.json`
- Create: `plugins/vsdd-factory/skills/visual-companion/vite.config.js`
- Create: `plugins/vsdd-factory/skills/visual-companion/src/index.html`
- Create: `plugins/vsdd-factory/skills/visual-companion/src/main.jsx`
- Create: `plugins/vsdd-factory/skills/visual-companion/.gitignore`

- [ ] **Step 1: Create package.json**

Create `plugins/vsdd-factory/skills/visual-companion/package.json`:

```json
{
  "name": "vsdd-visual-companion",
  "version": "1.0.0",
  "private": true,
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "vite build",
    "preview": "vite preview"
  },
  "dependencies": {
    "@excalidraw/excalidraw": "^0.18.0",
    "react": "^18.3.0",
    "react-dom": "^18.3.0"
  },
  "devDependencies": {
    "@vitejs/plugin-react": "^4.3.0",
    "vite": "^6.0.0"
  }
}
```

- [ ] **Step 2: Create vite.config.js**

Create `plugins/vsdd-factory/skills/visual-companion/vite.config.js`:

```javascript
import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';

export default defineConfig({
  plugins: [react()],
  root: 'src',
  build: {
    outDir: '../dist',
    emptyOutDir: true,
  },
  define: {
    'process.env.IS_PREACT': JSON.stringify('false'),
  },
});
```

Note: The `process.env.IS_PREACT` define is required by @excalidraw/excalidraw to avoid build errors.

- [ ] **Step 3: Create src/index.html**

Create `plugins/vsdd-factory/skills/visual-companion/src/index.html`:

```html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>VSDD Visual Companion</title>
  <style>
    * { box-sizing: border-box; margin: 0; padding: 0; }
    html, body, #root { height: 100%; width: 100%; overflow: hidden; }
  </style>
</head>
<body>
  <div id="root"></div>
  <script type="module" src="/main.jsx"></script>
</body>
</html>
```

- [ ] **Step 4: Create src/main.jsx**

Create `plugins/vsdd-factory/skills/visual-companion/src/main.jsx`:

```jsx
import React from 'react';
import { createRoot } from 'react-dom/client';
import App from './App.jsx';

createRoot(document.getElementById('root')).render(<App />);
```

- [ ] **Step 5: Create .gitignore**

Create `plugins/vsdd-factory/skills/visual-companion/.gitignore`:

```
node_modules/
dist/
```

- [ ] **Step 6: Commit**

```bash
git add plugins/vsdd-factory/skills/visual-companion/package.json \
       plugins/vsdd-factory/skills/visual-companion/vite.config.js \
       plugins/vsdd-factory/skills/visual-companion/src/index.html \
       plugins/vsdd-factory/skills/visual-companion/src/main.jsx \
       plugins/vsdd-factory/skills/visual-companion/.gitignore
git commit -m "feat(visual-companion): add React app scaffold for excalidraw integration"
```

---

### Task 2: Create the React components (App, ExcalidrawView, HistorySidebar)

**Files:**
- Create: `plugins/vsdd-factory/skills/visual-companion/src/App.jsx`
- Create: `plugins/vsdd-factory/skills/visual-companion/src/ExcalidrawView.jsx`
- Create: `plugins/vsdd-factory/skills/visual-companion/src/HistorySidebar.jsx`

- [ ] **Step 1: Create App.jsx**

Create `plugins/vsdd-factory/skills/visual-companion/src/App.jsx`:

This is the mode router. It:
- Connects to the WebSocket server
- Fetches the file list from `/api/files`
- Determines the active file (newest by mtime, or user-selected from sidebar)
- Renders ExcalidrawView for `.excalidraw` files
- Renders an iframe to `/html/<filename>` for `.html` files (server serves HTML frame at this route)
- Manages split view state (when manifest `screen.json` exists or user drags from sidebar)
- Listens for WebSocket `reload` and `file-list-updated` messages

Key state:
```
activeFile: string | null        // currently displayed file name
files: Array<{name, type, mtime}>  // all files from /api/files
splitFile: string | null         // second file for split view (null = no split)
sidebarOpen: boolean             // sidebar visibility
ws: WebSocket                    // connection to server
```

Layout structure:
```
+--+----------------------------+
|S |  Active view               |
|I |  (ExcalidrawView or iframe)|
|D |                            |
|E |----------------------------|
|B |  Split view (optional)     |
|A |                            |
|R |                            |
+--+----------------------------+
```

Sidebar is 250px wide when open, 0px when closed. Toggle button always visible.

- [ ] **Step 2: Create ExcalidrawView.jsx**

Create `plugins/vsdd-factory/skills/visual-companion/src/ExcalidrawView.jsx`:

This component:
- Dynamically imports `@excalidraw/excalidraw` (lazy load to avoid SSR issues)
- Receives `initialData` prop (parsed .excalidraw JSON)
- Receives `ws` prop (WebSocket connection)
- Receives `fileName` prop (for save-back identification)
- Stores `excalidrawAPI` ref for programmatic updates
- `onChange` callback debounces 500ms, sends `{ type: "drawing-updated", file: fileName, data: { elements, appState, files } }` via WebSocket
- Listens for WebSocket messages with `type: "load-drawing"` to update scene via `excalidrawAPI.updateScene()`
- Reads `viewMode` from initialData appState if present (future C-mode support)
- Full height/width of parent container

```jsx
import React, { useCallback, useRef, useState, useEffect } from 'react';

// Lazy import excalidraw to avoid SSR/build issues
let Excalidraw = null;

export default function ExcalidrawView({ initialData, ws, fileName }) {
  const [loaded, setLoaded] = useState(false);
  const excalidrawAPI = useRef(null);
  const debounceTimer = useRef(null);

  useEffect(() => {
    import('@excalidraw/excalidraw').then((mod) => {
      Excalidraw = mod.Excalidraw;
      setLoaded(true);
    });
  }, []);

  useEffect(() => {
    if (!ws) return;
    const handler = (msg) => {
      const data = JSON.parse(msg.data);
      if (data.type === 'load-drawing' && excalidrawAPI.current) {
        excalidrawAPI.current.updateScene({
          elements: data.elements,
          appState: data.appState,
          storeAction: 'capture',
        });
      }
    };
    ws.addEventListener('message', handler);
    return () => ws.removeEventListener('message', handler);
  }, [ws]);

  const handleChange = useCallback((elements, appState, files) => {
    if (debounceTimer.current) clearTimeout(debounceTimer.current);
    debounceTimer.current = setTimeout(() => {
      if (ws && ws.readyState === WebSocket.OPEN) {
        ws.send(JSON.stringify({
          type: 'drawing-updated',
          file: fileName,
          data: { elements, appState, files },
        }));
      }
    }, 500);
  }, [ws, fileName]);

  if (!loaded) return <div style={{display:'flex',alignItems:'center',justifyContent:'center',height:'100%'}}>Loading Excalidraw...</div>;

  const viewMode = initialData?.appState?.viewMode || false;

  return (
    <div style={{ height: '100%', width: '100%' }}>
      <Excalidraw
        excalidrawAPI={(api) => { excalidrawAPI.current = api; }}
        initialData={initialData}
        onChange={handleChange}
        viewModeEnabled={viewMode}
        theme={window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light'}
      />
    </div>
  );
}
```

- [ ] **Step 3: Create HistorySidebar.jsx**

Create `plugins/vsdd-factory/skills/visual-companion/src/HistorySidebar.jsx`:

This component:
- Receives `files` prop (array of {name, type, mtime})
- Receives `activeFile` prop
- Receives `onSelect` callback (called with filename when user clicks)
- Receives `isOpen` and `onToggle` props
- Shows a list of files sorted by mtime (newest first)
- Each file shows an icon (page icon for .html, diamond icon for .excalidraw, gear icon for screen.json)
- Active file is highlighted
- Collapsible via toggle button

```jsx
import React from 'react';

const ICONS = {
  '.html': '\u{1F4C4}',
  '.excalidraw': '\u{1F4D0}',
  '.json': '\u{2699}',
};

export default function HistorySidebar({ files, activeFile, onSelect, isOpen, onToggle }) {
  return (
    <>
      <button
        onClick={onToggle}
        style={{
          position: 'fixed', left: isOpen ? 250 : 0, top: '50%',
          transform: 'translateY(-50%)', zIndex: 1000,
          background: 'var(--bg-secondary, #fff)', border: '1px solid var(--border, #ccc)',
          borderLeft: 'none', borderRadius: '0 6px 6px 0',
          padding: '8px 4px', cursor: 'pointer', fontSize: '12px',
          transition: 'left 0.2s',
        }}
      >
        {isOpen ? '\u25C0' : '\u25B6'}
      </button>
      {isOpen && (
        <div style={{
          position: 'fixed', left: 0, top: 0, bottom: 0, width: 250,
          background: 'var(--bg-secondary, #fff)', borderRight: '1px solid var(--border, #ccc)',
          overflowY: 'auto', zIndex: 999, padding: '12px 0',
        }}>
          <div style={{ padding: '0 12px 8px', fontSize: '11px', color: 'var(--text-secondary, #888)', textTransform: 'uppercase', letterSpacing: '0.05em' }}>
            History
          </div>
          {files.map((f) => {
            const ext = f.name.slice(f.name.lastIndexOf('.'));
            const icon = ICONS[ext] || '\u{1F4C4}';
            const isActive = f.name === activeFile;
            return (
              <div
                key={f.name}
                onClick={() => onSelect(f.name)}
                style={{
                  padding: '8px 12px', cursor: 'pointer', fontSize: '13px',
                  background: isActive ? 'var(--selected-bg, #e8f4fd)' : 'transparent',
                  borderLeft: isActive ? '3px solid var(--accent, #0071e3)' : '3px solid transparent',
                }}
              >
                {icon} {f.name}
              </div>
            );
          })}
        </div>
      )}
    </>
  );
}
```

- [ ] **Step 4: Commit**

```bash
git add plugins/vsdd-factory/skills/visual-companion/src/App.jsx \
       plugins/vsdd-factory/skills/visual-companion/src/ExcalidrawView.jsx \
       plugins/vsdd-factory/skills/visual-companion/src/HistorySidebar.jsx
git commit -m "feat(visual-companion): add React components for excalidraw and history sidebar"
```

---

### Task 3: Modify server.cjs for file-type detection, /api/files, and drawing save-back

**Files:**
- Modify: `plugins/vsdd-factory/skills/visual-companion/scripts/server.cjs`

- [ ] **Step 1: Read the current server.cjs**

Read `plugins/vsdd-factory/skills/visual-companion/scripts/server.cjs`.

- [ ] **Step 2: Update getNewestScreen to support all file types**

Replace the `getNewestScreen` function (currently only filters `.html`) to support `.html`, `.excalidraw`, and `screen.json`:

```javascript
const SCREEN_EXTENSIONS = ['.html', '.excalidraw'];

function getNewestScreen() {
  const files = fs.readdirSync(CONTENT_DIR)
    .filter(f => SCREEN_EXTENSIONS.some(ext => f.endsWith(ext)) || f === 'screen.json')
    .map(f => {
      const fp = path.join(CONTENT_DIR, f);
      return { name: f, path: fp, mtime: fs.statSync(fp).mtime.getTime() };
    })
    .sort((a, b) => b.mtime - a.mtime);
  return files.length > 0 ? files[0] : null;
}
```

- [ ] **Step 3: Update file watcher to watch all screen types**

In the `startServer` function, update the `knownFiles` set and watcher filter to include `.excalidraw` files:

```javascript
const knownFiles = new Set(
  fs.readdirSync(CONTENT_DIR).filter(f =>
    SCREEN_EXTENSIONS.some(ext => f.endsWith(ext)) || f === 'screen.json'
  )
);

// In the watcher callback, change the filter:
if (!filename || !SCREEN_EXTENSIONS.some(ext => filename.endsWith(ext)) && filename !== 'screen.json') return;
```

- [ ] **Step 4: Add DIST_DIR constant and excalidraw availability check**

After the existing constants section, add:

```javascript
const DIST_DIR = path.resolve(__dirname, '..', 'dist');
const EXCALIDRAW_AVAILABLE = fs.existsSync(path.join(DIST_DIR, 'index.html'));
```

- [ ] **Step 5: Update handleRequest for file-type routing**

Replace the `handleRequest` function to support three modes:

```javascript
function handleRequest(req, res) {
  touchActivity();

  if (req.method === 'GET' && req.url === '/') {
    const screen = getNewestScreen();
    if (!screen) {
      res.writeHead(200, { 'Content-Type': 'text/html; charset=utf-8' });
      res.end(WAITING_PAGE);
      return;
    }

    if (screen.name.endsWith('.excalidraw')) {
      serveExcalidraw(res, screen);
    } else if (screen.name === 'screen.json') {
      serveManifest(res, screen);
    } else {
      serveHtml(res, screen);
    }
  } else if (req.method === 'GET' && req.url.startsWith('/html/')) {
    // Serve a specific HTML file in the frame (for iframe embedding from React app)
    const fileName = decodeURIComponent(req.url.slice(6));
    const filePath = path.join(CONTENT_DIR, path.basename(fileName));
    if (!fs.existsSync(filePath)) { res.writeHead(404); res.end('Not found'); return; }
    const raw = fs.readFileSync(filePath, 'utf-8');
    let html = isFullDocument(raw) ? raw : wrapInFrame(raw);
    if (html.includes('</body>')) {
      html = html.replace('</body>', helperInjection + '\n</body>');
    } else {
      html += helperInjection;
    }
    res.writeHead(200, { 'Content-Type': 'text/html; charset=utf-8' });
    res.end(html);
  } else if (req.method === 'GET' && req.url === '/api/files') {
    const files = fs.readdirSync(CONTENT_DIR)
      .filter(f => SCREEN_EXTENSIONS.some(ext => f.endsWith(ext)) || f === 'screen.json')
      .map(f => {
        const fp = path.join(CONTENT_DIR, f);
        const ext = path.extname(f);
        return { name: f, type: ext === '.excalidraw' ? 'excalidraw' : ext === '.json' ? 'manifest' : 'html', mtime: fs.statSync(fp).mtime.getTime() };
      })
      .sort((a, b) => b.mtime - a.mtime);
    res.writeHead(200, { 'Content-Type': 'application/json' });
    res.end(JSON.stringify(files));
  } else if (req.method === 'GET' && req.url.startsWith('/api/drawing/')) {
    // Serve raw excalidraw JSON for the React app to load
    const fileName = decodeURIComponent(req.url.slice(13));
    const filePath = path.join(CONTENT_DIR, path.basename(fileName));
    if (!fs.existsSync(filePath)) { res.writeHead(404); res.end('Not found'); return; }
    res.writeHead(200, { 'Content-Type': 'application/json' });
    res.end(fs.readFileSync(filePath));
  } else if (req.method === 'GET' && req.url.startsWith('/assets/')) {
    // Serve Vite-built assets
    const filePath = path.join(DIST_DIR, req.url);
    if (!fs.existsSync(filePath)) { res.writeHead(404); res.end('Not found'); return; }
    const ext = path.extname(filePath).toLowerCase();
    res.writeHead(200, { 'Content-Type': MIME_TYPES[ext] || 'application/octet-stream' });
    res.end(fs.readFileSync(filePath));
  } else if (req.method === 'GET' && req.url.startsWith('/files/')) {
    const fileName = req.url.slice(7);
    const filePath = path.join(CONTENT_DIR, path.basename(fileName));
    if (!fs.existsSync(filePath)) { res.writeHead(404); res.end('Not found'); return; }
    const ext = path.extname(filePath).toLowerCase();
    res.writeHead(200, { 'Content-Type': MIME_TYPES[ext] || 'application/octet-stream' });
    res.end(fs.readFileSync(filePath));
  } else {
    res.writeHead(404);
    res.end('Not found');
  }
}

function serveHtml(res, screen) {
  const raw = fs.readFileSync(screen.path, 'utf-8');
  let html = isFullDocument(raw) ? raw : wrapInFrame(raw);
  if (html.includes('</body>')) {
    html = html.replace('</body>', helperInjection + '\n</body>');
  } else {
    html += helperInjection;
  }
  res.writeHead(200, { 'Content-Type': 'text/html; charset=utf-8' });
  res.end(html);
}

function serveExcalidraw(res, screen) {
  if (!EXCALIDRAW_AVAILABLE) {
    const setupPage = `<!DOCTYPE html><html><head><meta charset="utf-8"><title>VSDD Visual Companion</title>
<style>body{font-family:system-ui,sans-serif;padding:2rem;max-width:600px;margin:0 auto;}
h1{color:#333;}p{color:#666;margin:1rem 0;}code{background:#f0f0f0;padding:2px 6px;border-radius:3px;}</style>
</head><body><h1>Excalidraw Support Requires Setup</h1>
<p>An <code>.excalidraw</code> file was detected, but the React app hasn't been built yet.</p>
<p>Run <code>/vsdd-factory:visual-companion-setup</code> to install dependencies and build the excalidraw viewer.</p>
<p>HTML mockups continue to work without setup.</p></body></html>`;
    res.writeHead(200, { 'Content-Type': 'text/html; charset=utf-8' });
    res.end(setupPage);
    return;
  }
  // Serve the Vite-built React app with the drawing filename injected
  let html = fs.readFileSync(path.join(DIST_DIR, 'index.html'), 'utf-8');
  const injection = `<script>window.__ACTIVE_FILE__=${JSON.stringify(screen.name)};window.__WS_URL__="ws://"+window.location.host;</script>`;
  html = html.replace('</head>', injection + '\n</head>');
  res.writeHead(200, { 'Content-Type': 'text/html; charset=utf-8' });
  res.end(html);
}

function serveManifest(res, screen) {
  // For screen.json manifests, serve the React app which handles composed layouts
  if (!EXCALIDRAW_AVAILABLE) {
    serveHtml(res, { path: path.join(CONTENT_DIR, 'waiting.html'), name: 'waiting.html' });
    return;
  }
  let html = fs.readFileSync(path.join(DIST_DIR, 'index.html'), 'utf-8');
  const manifest = JSON.parse(fs.readFileSync(screen.path, 'utf-8'));
  const injection = `<script>window.__MANIFEST__=${JSON.stringify(manifest)};window.__WS_URL__="ws://"+window.location.host;</script>`;
  html = html.replace('</head>', injection + '\n</head>');
  res.writeHead(200, { 'Content-Type': 'text/html; charset=utf-8' });
  res.end(html);
}
```

- [ ] **Step 6: Add drawing save-back to handleMessage**

Update the `handleMessage` function to handle `drawing-updated` events:

```javascript
function handleMessage(text) {
  let event;
  try {
    event = JSON.parse(text);
  } catch (e) {
    console.error('Failed to parse WebSocket message:', e.message);
    return;
  }
  touchActivity();
  console.log(JSON.stringify({ source: 'user-event', ...event }));

  if (event.type === 'drawing-updated' && event.file && event.data) {
    // Save excalidraw changes back to file
    const filePath = path.join(CONTENT_DIR, path.basename(event.file));
    const excalidrawData = {
      type: 'excalidraw',
      version: 2,
      source: 'vsdd-visual-companion',
      elements: event.data.elements || [],
      appState: event.data.appState || {},
      files: event.data.files || {},
    };
    fs.writeFileSync(filePath, JSON.stringify(excalidrawData, null, 2));
    console.log(JSON.stringify({ type: 'drawing-saved', file: filePath }));
    return;
  }

  if (event.choice) {
    const eventsFile = path.join(STATE_DIR, 'events');
    fs.appendFileSync(eventsFile, JSON.stringify(event) + '\n');
  }
}
```

- [ ] **Step 7: Update WAITING_PAGE title**

Change "Brainstorm Companion" to "VSDD Visual Companion" in the WAITING_PAGE constant.

- [ ] **Step 8: Add .excalidraw to MIME_TYPES**

```javascript
'.excalidraw': 'application/json'
```

- [ ] **Step 9: Commit**

```bash
git add plugins/vsdd-factory/skills/visual-companion/scripts/server.cjs
git commit -m "feat(visual-companion): add file-type detection, excalidraw routing, and drawing save-back to server"
```

---

### Task 4: Create setup.sh and update start-server.sh

**Files:**
- Create: `plugins/vsdd-factory/skills/visual-companion/setup.sh`
- Modify: `plugins/vsdd-factory/skills/visual-companion/scripts/start-server.sh`

- [ ] **Step 1: Create setup.sh**

Create `plugins/vsdd-factory/skills/visual-companion/setup.sh`:

```bash
#!/usr/bin/env bash
# Setup excalidraw support for the visual companion.
# Installs npm dependencies and builds the React app.
# Usage: setup.sh
#
# Run once. Re-run after updating package.json.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$SCRIPT_DIR"

echo '{"status": "installing", "step": "npm install"}'

if ! command -v node >/dev/null 2>&1; then
  echo '{"error": "Node.js is not installed. Install Node.js 18+ to enable excalidraw support."}'
  exit 1
fi

if ! command -v npm >/dev/null 2>&1; then
  echo '{"error": "npm is not installed."}'
  exit 1
fi

npm install --no-audit --no-fund 2>&1 | tail -1

echo '{"status": "building", "step": "vite build"}'

npx vite build 2>&1 | tail -3

if [ -f "dist/index.html" ]; then
  echo '{"status": "complete", "dist": "'"$SCRIPT_DIR/dist"'"}'
else
  echo '{"error": "Build failed — dist/index.html not found"}'
  exit 1
fi
```

Make executable: `chmod +x plugins/vsdd-factory/skills/visual-companion/setup.sh`

- [ ] **Step 2: Update start-server.sh to report excalidraw availability**

Read `plugins/vsdd-factory/skills/visual-companion/scripts/start-server.sh`. In the JSON output where `server-started` is reported, the server already includes this info via the `EXCALIDRAW_AVAILABLE` constant. No changes needed to start-server.sh — the server itself detects dist/ at startup.

- [ ] **Step 3: Commit**

```bash
git add plugins/vsdd-factory/skills/visual-companion/setup.sh
git commit -m "feat(visual-companion): add setup.sh for excalidraw dependency installation and build"
```

---

### Task 5: Create the create-excalidraw skill

**Files:**
- Create: `plugins/vsdd-factory/skills/create-excalidraw/SKILL.md`

- [ ] **Step 1: Create directory and SKILL.md**

```bash
mkdir -p plugins/vsdd-factory/skills/create-excalidraw
```

Create `plugins/vsdd-factory/skills/create-excalidraw/SKILL.md`:

```markdown
---
name: create-excalidraw
description: >
  Use when generating architecture diagrams, entity relationships, flow charts,
  or any visual diagram as an .excalidraw JSON file. Files can be viewed in
  excalidraw.com, VS Code, or the visual companion browser.
---

# Create Excalidraw Diagram

Generate `.excalidraw` JSON files programmatically. Each file is a valid excalidraw document that can be opened in excalidraw.com, VS Code (with the excalidraw extension), or rendered interactively in the visual companion browser.

## Output Location

Write files to `.factory/diagrams/<name>.excalidraw`.

## JSON Structure

Every `.excalidraw` file has this structure:

` ``json
{
  "type": "excalidraw",
  "version": 2,
  "source": "vsdd-factory",
  "elements": [...],
  "appState": {
    "viewBackgroundColor": "#ffffff",
    "gridSize": null
  },
  "files": {}
}
` ``

## Element Types

### Rectangle

` ``json
{
  "type": "rectangle",
  "id": "unique-id",
  "x": 100, "y": 100,
  "width": 200, "height": 100,
  "strokeColor": "#1e1e1e",
  "backgroundColor": "#a5d8ff",
  "fillStyle": "solid",
  "strokeWidth": 2,
  "strokeStyle": "solid",
  "roughness": 1,
  "opacity": 100,
  "angle": 0,
  "seed": 1234567890,
  "versionNonce": 987654321,
  "isDeleted": false,
  "groupIds": [],
  "boundElements": [],
  "updated": 1700000000000,
  "link": null,
  "locked": false
}
` ``

### Text

` ``json
{
  "type": "text",
  "id": "text-id",
  "x": 120, "y": 130,
  "width": 160, "height": 40,
  "text": "API Gateway",
  "fontSize": 20,
  "fontFamily": 1,
  "textAlign": "center",
  "verticalAlign": "middle",
  "strokeColor": "#1e1e1e",
  "backgroundColor": "transparent",
  "fillStyle": "solid",
  "strokeWidth": 1,
  "strokeStyle": "solid",
  "roughness": 1,
  "opacity": 100,
  "angle": 0,
  "seed": 1234567891,
  "versionNonce": 987654322,
  "isDeleted": false,
  "groupIds": [],
  "boundElements": null,
  "updated": 1700000000000,
  "containerId": null,
  "originalText": "API Gateway",
  "autoResize": true,
  "lineHeight": 1.25
}
` ``

To put text inside a rectangle, set `containerId` to the rectangle's ID and add the text element to the rectangle's `boundElements` array as `{ "type": "text", "id": "text-id" }`.

### Arrow

` ``json
{
  "type": "arrow",
  "id": "arrow-id",
  "x": 300, "y": 150,
  "width": 100, "height": 0,
  "points": [[0, 0], [100, 0]],
  "strokeColor": "#1e1e1e",
  "backgroundColor": "transparent",
  "fillStyle": "solid",
  "strokeWidth": 2,
  "strokeStyle": "solid",
  "roughness": 1,
  "opacity": 100,
  "angle": 0,
  "seed": 1234567892,
  "versionNonce": 987654323,
  "isDeleted": false,
  "groupIds": [],
  "boundElements": null,
  "updated": 1700000000000,
  "startBinding": { "elementId": "source-rect-id", "focus": 0, "gap": 1 },
  "endBinding": { "elementId": "target-rect-id", "focus": 0, "gap": 1 },
  "startArrowhead": null,
  "endArrowhead": "arrow"
}
` ``

When using `startBinding`/`endBinding`, also add the arrow to each rectangle's `boundElements` array as `{ "type": "arrow", "id": "arrow-id" }`.

### Ellipse and Diamond

Same structure as rectangle but `"type": "ellipse"` or `"type": "diamond"`.

## Layout Helpers

### Grid Positioning

For architecture diagrams, use a grid layout:
- Column width: 250px (200px box + 50px gap)
- Row height: 150px (100px box + 50px gap)
- Starting position: x=50, y=50

### Auto-spacing

For flow charts (left-to-right):
- Box width: 200px, height: 80px
- Horizontal gap: 100px (for arrows)
- Arrow length: 100px

## Styling Guide

| Use case | strokeColor | backgroundColor | fillStyle |
|----------|------------|----------------|-----------|
| Service/component | #1e1e1e | #a5d8ff | solid |
| Database | #1e1e1e | #b2f2bb | solid |
| External system | #1e1e1e | #ffd8a8 | solid |
| User/actor | #1e1e1e | #d0bfff | solid |
| Decision | #1e1e1e | #fff3bf | solid |
| Highlighted/active | #e03131 | #ffc9c9 | solid |

## Pushing to Visual Companion

If the visual companion is running, write the `.excalidraw` file to the session's `screen_dir`. The server detects the file type and serves the interactive excalidraw editor. The user can modify the diagram in the browser, and changes sync back to the file.

## Standalone Usage

Without the visual companion, `.excalidraw` files can be:
- Opened at https://excalidraw.com (File → Open)
- Opened in VS Code with the Excalidraw extension
- Committed to the repo for documentation

## ID Generation

Use deterministic IDs based on element purpose: `"api-gateway-rect"`, `"db-to-api-arrow"`. Avoid random UUIDs — deterministic IDs make diffs readable and updates predictable.
```

- [ ] **Step 2: Commit**

```bash
git add plugins/vsdd-factory/skills/create-excalidraw/SKILL.md
git commit -m "feat(skills): add create-excalidraw skill for generating .excalidraw JSON files"
```

---

### Task 6: Update visual companion SKILL.md and visual-guide.md

**Files:**
- Modify: `plugins/vsdd-factory/skills/visual-companion/SKILL.md`
- Modify: `plugins/vsdd-factory/skills/visual-companion/visual-guide.md`

- [ ] **Step 1: Update SKILL.md**

Read the current file. Add these sections:

After the "Prerequisites" section, add:

```markdown
## Excalidraw Setup (optional)

To enable interactive excalidraw diagrams in the browser:

` ``bash
${CLAUDE_PLUGIN_ROOT}/skills/visual-companion/setup.sh
` ``

This installs React + @excalidraw/excalidraw and builds the viewer. Run once. Without this, `.excalidraw` files show a "run setup" message while HTML mockups continue to work.
```

After the "When to Use vs Not" section, add:

```markdown
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

` ``json
{
  "layout": "split",
  "panes": [
    { "file": "arch-overview.excalidraw", "label": "Architecture" },
    { "file": "component-mockup.html", "label": "Mockup" }
  ]
}
` ``
```

- [ ] **Step 2: Update visual-guide.md**

Read the current file. Add an "Excalidraw Files" section covering:
- How to write .excalidraw files (reference create-excalidraw skill)
- How they render in the companion (interactive canvas)
- Save-back behavior (user edits sync to file)
- Mixing HTML and excalidraw in the same session

- [ ] **Step 3: Commit**

```bash
git add plugins/vsdd-factory/skills/visual-companion/SKILL.md \
       plugins/vsdd-factory/skills/visual-companion/visual-guide.md
git commit -m "feat(visual-companion): document excalidraw mode, setup, history sidebar, and composed views"
```

---

### Task 7: Fix tiered visual tooling tables across all skills and docs

**Files:**
- Modify: `plugins/vsdd-factory/skills/brainstorming/SKILL.md`
- Modify: `plugins/vsdd-factory/skills/guided-brief-creation/SKILL.md`
- Modify: `plugins/vsdd-factory/skills/create-architecture/SKILL.md`
- Modify: `plugins/vsdd-factory/docs/guide/cross-cutting-skills.md`
- Modify: `plugins/vsdd-factory/docs/guide/phase-1-spec-crystallization.md`

- [ ] **Step 1: Read all 5 files and find the tiered tables**

Search for "excalidraw-export" in each file. Replace the entire tiered table in each with:

```markdown
| Tier | Tool | Check | Best for |
|------|------|-------|----------|
| 1 | `/vsdd-factory:visual-companion` | Node.js available, user accepts | Interactive mockups, A/B choices, clickable layouts |
| 1 | `/vsdd-factory:visual-companion` (excalidraw) | Setup completed | Architecture diagrams, entity relationships, interactive editing |
| 2 | `/vsdd-factory:create-excalidraw` | Always available | Generate .excalidraw files for offline viewing in excalidraw.com or VS Code |
| 3 | Mermaid code blocks | Always available | Sequence diagrams, state machines, simple flows |
| 4 | ASCII/text | Always available | Wireframe sketches, table layouts, comparisons |
```

For `create-architecture/SKILL.md`, the table is slightly different (architecture-focused "Best for" column). Adapt accordingly.

- [ ] **Step 2: Commit all 5 files**

```bash
git add plugins/vsdd-factory/skills/brainstorming/SKILL.md \
       plugins/vsdd-factory/skills/guided-brief-creation/SKILL.md \
       plugins/vsdd-factory/skills/create-architecture/SKILL.md \
       plugins/vsdd-factory/docs/guide/cross-cutting-skills.md \
       plugins/vsdd-factory/docs/guide/phase-1-spec-crystallization.md
git commit -m "fix: update tiered visual tooling tables — replace excalidraw-export with correct tiers"
```

---

### Task 8: Update README and CHANGELOG, run tests

**Files:**
- Modify: `README.md`
- Modify: `docs/guide/cross-cutting-skills.md` (add create-excalidraw)

- [ ] **Step 1: Update README skill count**

The skill count needs to increase by 1 (create-excalidraw). Find `95` in the skills row and update to `96`.

- [ ] **Step 2: Add create-excalidraw to cross-cutting skills doc**

In the Visual Tooling section, add after the visual-companion entry:

```markdown
### `/vsdd-factory:create-excalidraw`

Generate `.excalidraw` JSON files for architecture diagrams, entity relationships, and flow charts. Files can be opened in excalidraw.com, VS Code (with the excalidraw extension), or rendered interactively in the visual companion browser.

```
/vsdd-factory:create-excalidraw
```

Includes element type reference (rectangle, ellipse, diamond, arrow, text), styling guide, layout helpers, and arrow binding documentation. Output to `.factory/diagrams/`.
```

- [ ] **Step 3: Run test suite**

```bash
cd plugins/vsdd-factory/tests && ./run-all.sh
```

All 62 tests must pass. The new skill (create-excalidraw) should be auto-discovered. If any structural tests fail (e.g., template reference checks), fix the issue.

- [ ] **Step 4: Run remaining pre-release checks**

```bash
shellcheck plugins/vsdd-factory/hooks/*.sh
test -f plugins/vsdd-factory/.claude-plugin/plugin.json
for f in plugins/vsdd-factory/workflows/*.lobster plugins/vsdd-factory/workflows/phases/*.lobster; do
  plugins/vsdd-factory/bin/lobster-parse "$f" '.workflow.name' >/dev/null
done
```

- [ ] **Step 5: Commit**

```bash
git add README.md docs/guide/cross-cutting-skills.md
git commit -m "docs: add create-excalidraw to cross-cutting skills, update README count"
```
