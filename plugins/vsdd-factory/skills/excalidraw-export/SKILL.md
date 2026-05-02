---
name: excalidraw-export
description: Batch-render .excalidraw wireframe diagrams to pixel-perfect PNG using headless Firefox via Playwright. Reference-only skill, not directly invokable.
---

# Excalidraw PNG Export

Batch-render `.excalidraw` wireframe diagrams to pixel-perfect PNG using headless Firefox via Playwright.

## Location

```
_bmad-output/planning-artifacts/wireframes/
├── export-png.sh                # Batch export script
├── EXPORT-TOOLS-RESEARCH.md     # Tool evaluation & setup research
├── architecture/*.excalidraw    # Architecture diagrams (104 files)
├── ui/*.excalidraw              # Web UI wireframes (52 files)
├── tui/*.excalidraw             # Terminal UI wireframes (12 files)
└── mobile/*.excalidraw          # Mobile wireframes (6 files)
```

## Usage

```bash
cd _bmad-output/planning-artifacts/wireframes

./export-png.sh              # Convert all .excalidraw files to PNG
./export-png.sh --changed    # Only convert files modified since last run
./export-png.sh --clean      # Remove all generated .png files
```

## Prerequisites

```bash
npm install -g excalidraw-brute-export-cli
npx playwright install firefox
```

## How It Works

- Uses `excalidraw-brute-export-cli` v0.4.0+ (headless Firefox renders identically to excalidraw.com)
- 4 parallel workers, 2x scale, white background
- PNGs placed alongside source files (e.g., `01-dashboard-main.excalidraw` -> `01-dashboard-main.png`)
- `--changed` mode uses a state file timestamp to skip unchanged files
- ~5-8 sec/diagram, ~4-6 min for all 175 files

## Known Issues

- Arrow elements **must** have a `points` property (e.g., `[[0,0],[width,height]]`) or excalidraw.com shows an error modal that blocks export
- Must use v0.4.0+ of excalidraw-brute-export-cli (v0.2.0 times out on current excalidraw.com UI)
- See `EXPORT-TOOLS-RESEARCH.md` for full tool evaluation and troubleshooting notes
