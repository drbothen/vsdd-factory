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
