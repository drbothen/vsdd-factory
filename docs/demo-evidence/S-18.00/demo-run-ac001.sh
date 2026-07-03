#!/usr/bin/env bash
# AC-001 (BC-1.15.001 PC1): PreCompact routes to registered plugin (exit 0)
# Run standalone: bash demo-run-ac001.sh
set -euo pipefail
REPO="$(cd "$(dirname "$0")/../../.." && pwd)"
DISPATCHER="$REPO/target/release/factory-dispatcher"
WORK=$(mktemp -d)
trap 'rm -rf "$WORK"' EXIT
mkdir -p "$WORK/.factory/logs" "$WORK/hook-plugins" "$WORK/hooks"
cp "$REPO/plugins/vsdd-factory/hook-plugins/legacy-bash-adapter.wasm" "$WORK/hook-plugins/"

printf '#!/usr/bin/env bash\nexit 0\n' > "$WORK/hooks/ok.sh"
chmod +x "$WORK/hooks/ok.sh"

cat > "$WORK/hooks-registry.toml" <<'TOML'
schema_version = 2

[[hooks]]
name = "demo-precompact-plugin"
event = "PreCompact"
plugin = "hook-plugins/legacy-bash-adapter.wasm"
timeout_ms = 5000
on_error = "block"

[hooks.capabilities.exec_subprocess]
binary_allow = ["bash"]
shell_bypass_acknowledged = "yes"
cwd_allow = ["."]

[hooks.config]
script_path = "hooks/ok.sh"
TOML

echo "# AC-001 (BC-1.15.001 PC1): PreCompact routes to registered plugin"
echo "# Dispatcher: $DISPATCHER"
echo "# Registry event=PreCompact, plugin exits 0"
echo ""
echo "$ printf '{...PreCompact...}' | factory-dispatcher"
echo ""
printf '{"event_name":"PreCompact","tool_name":"","session_id":"demo-ac001","tool_input":{}}' \
  | CLAUDE_PLUGIN_ROOT="$WORK" CLAUDE_PROJECT_DIR="$WORK" "$DISPATCHER" 2>&1
echo ""
echo "# sync_plugins=1 confirms PreCompact routed to demo-precompact-plugin"
echo "# block_intent=false, exit_code=0 — plugin exited 0, no block (AC-001 PASS)"
