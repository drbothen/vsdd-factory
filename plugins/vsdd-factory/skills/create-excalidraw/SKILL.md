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

```json
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
```

## Element Types

### Rectangle

```json
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
```

### Text

```json
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
```

To put text inside a rectangle, set `containerId` to the rectangle's ID and add the text element to the rectangle's `boundElements` array as `{ "type": "text", "id": "text-id" }`.

### Arrow

```json
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
```

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
- Opened at https://excalidraw.com (File > Open)
- Opened in VS Code with the Excalidraw extension
- Committed to the repo for documentation

## ID Generation

Use deterministic IDs based on element purpose: `"api-gateway-rect"`, `"db-to-api-arrow"`. Avoid random UUIDs — deterministic IDs make diffs readable and updates predictable.
