#!/usr/bin/env bats
# visual-companion.bats — TAP tests for the visual companion server

setup() {
  PLUGIN_ROOT="$(cd "$BATS_TEST_DIRNAME/.." && pwd)"
  SCRIPTS="$PLUGIN_ROOT/skills/visual-companion/scripts"
  WORK="$(mktemp -d)"
  SESSION_DIR="$WORK/session"
  CONTENT_DIR="$SESSION_DIR/content"
  STATE_DIR="$SESSION_DIR/state"
  mkdir -p "$CONTENT_DIR" "$STATE_DIR"

  # Pick a random high port
  PORT=$((49152 + RANDOM % 16383))

  # Start server in background
  env VISUAL_COMPANION_DIR="$SESSION_DIR" \
      VISUAL_COMPANION_HOST="127.0.0.1" \
      VISUAL_COMPANION_URL_HOST="localhost" \
      VISUAL_COMPANION_PORT="$PORT" \
      node "$SCRIPTS/server.cjs" &
  SERVER_PID=$!
  echo "$SERVER_PID" > "$STATE_DIR/server.pid"

  # Wait for server to be ready
  for i in $(seq 1 50); do
    if curl -s "http://localhost:$PORT/" >/dev/null 2>&1; then
      break
    fi
    sleep 0.1
  done
}

teardown() {
  if [ -f "$STATE_DIR/server.pid" ]; then
    kill "$(cat "$STATE_DIR/server.pid")" 2>/dev/null || true
    sleep 0.2
  fi
  rm -rf "$WORK"
}

# ---------- Basic server ----------

@test "visual-companion: serves waiting page when no files exist" {
  run curl -s "http://localhost:$PORT/"
  [ "$status" -eq 0 ]
  [[ "$output" == *"VSDD Visual Companion"* ]]
  [[ "$output" == *"Waiting"* ]]
}

# ---------- HTML mode ----------

@test "visual-companion: serves HTML file in frame" {
  printf '<h2>Test Content</h2>' > "$CONTENT_DIR/test.html"
  sleep 0.3
  run curl -s "http://localhost:$PORT/"
  [ "$status" -eq 0 ]
  [[ "$output" == *"Test Content"* ]]
  [[ "$output" == *"VSDD Visual Companion"* ]]
}

@test "visual-companion: /html/ route serves specific HTML file" {
  printf '<h2>Specific File</h2>' > "$CONTENT_DIR/layout.html"
  sleep 0.3
  run curl -s "http://localhost:$PORT/html/layout.html"
  [ "$status" -eq 0 ]
  [[ "$output" == *"Specific File"* ]]
}

# ---------- Excalidraw mode ----------

@test "visual-companion: detects .excalidraw as newest file" {
  # Push HTML first, then excalidraw
  printf '<h2>Old HTML</h2>' > "$CONTENT_DIR/old.html"
  sleep 0.2
  printf '{"type":"excalidraw","version":2,"source":"test","elements":[],"appState":{},"files":{}}' > "$CONTENT_DIR/diagram.excalidraw"
  sleep 0.3
  run curl -s "http://localhost:$PORT/"
  [ "$status" -eq 0 ]
  # Should serve React app (if dist exists) or setup message
  # Either way, should NOT serve the old HTML content
  [[ "$output" != *"Old HTML"* ]]
}

@test "visual-companion: /api/drawing serves excalidraw JSON" {
  printf '{"type":"excalidraw","version":2,"source":"test","elements":[{"type":"rectangle","id":"r1"}],"appState":{},"files":{}}' > "$CONTENT_DIR/arch.excalidraw"
  sleep 0.3
  run curl -s "http://localhost:$PORT/api/drawing/arch.excalidraw"
  [ "$status" -eq 0 ]
  [[ "$output" == *'"type":"excalidraw"'* ]]
  [[ "$output" == *'"id":"r1"'* ]]
}

@test "visual-companion: /api/drawing returns 404 for missing file" {
  run curl -s -o /dev/null -w "%{http_code}" "http://localhost:$PORT/api/drawing/nonexistent.excalidraw"
  [ "$output" = "404" ]
}

# ---------- /api/files ----------

@test "visual-companion: /api/files returns empty array when no files" {
  run curl -s "http://localhost:$PORT/api/files"
  [ "$status" -eq 0 ]
  [ "$output" = "[]" ]
}

@test "visual-companion: /api/files lists both HTML and excalidraw" {
  printf '<h2>HTML</h2>' > "$CONTENT_DIR/page.html"
  printf '{"type":"excalidraw","version":2,"elements":[],"appState":{},"files":{}}' > "$CONTENT_DIR/diagram.excalidraw"
  sleep 0.3
  run curl -s "http://localhost:$PORT/api/files"
  [ "$status" -eq 0 ]
  [[ "$output" == *"page.html"* ]]
  [[ "$output" == *"diagram.excalidraw"* ]]
}

@test "visual-companion: /api/files does not list non-screen files" {
  printf 'some notes' > "$CONTENT_DIR/notes.txt"
  printf '<h2>HTML</h2>' > "$CONTENT_DIR/page.html"
  sleep 0.3
  run curl -s "http://localhost:$PORT/api/files"
  [ "$status" -eq 0 ]
  [[ "$output" == *"page.html"* ]]
  [[ "$output" != *"notes.txt"* ]]
}

# ---------- File serving ----------

@test "visual-companion: /files/ serves content files" {
  printf '{"data":"test"}' > "$CONTENT_DIR/data.json"
  sleep 0.2
  run curl -s "http://localhost:$PORT/files/data.json"
  [ "$status" -eq 0 ]
  [[ "$output" == *'"data":"test"'* ]]
}

@test "visual-companion: /files/ returns 404 for missing file" {
  run curl -s -o /dev/null -w "%{http_code}" "http://localhost:$PORT/files/nope.json"
  [ "$output" = "404" ]
}

# ---------- Newest file detection ----------

@test "visual-companion: serves newest file when multiple exist" {
  printf '<h2>First</h2>' > "$CONTENT_DIR/first.html"
  sleep 0.3
  printf '<h2>Second</h2>' > "$CONTENT_DIR/second.html"
  sleep 0.3
  run curl -s "http://localhost:$PORT/"
  [ "$status" -eq 0 ]
  [[ "$output" == *"Second"* ]]
}

# ---------- setup.sh ----------

@test "visual-companion: setup.sh exists and is executable" {
  [ -x "$PLUGIN_ROOT/skills/visual-companion/setup.sh" ]
}

# ---------- Script structure ----------

@test "visual-companion: start-server.sh exists and is executable" {
  [ -x "$SCRIPTS/start-server.sh" ]
}

@test "visual-companion: stop-server.sh exists and is executable" {
  [ -x "$SCRIPTS/stop-server.sh" ]
}

@test "visual-companion: server.cjs exists" {
  [ -f "$SCRIPTS/server.cjs" ]
}

@test "visual-companion: frame-template.html exists" {
  [ -f "$SCRIPTS/frame-template.html" ]
}

@test "visual-companion: helper.js exists" {
  [ -f "$SCRIPTS/helper.js" ]
}
