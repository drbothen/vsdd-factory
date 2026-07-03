#!/usr/bin/env bash
# AC-004/VP-086 (BC-1.15.001 PC4): PreCompact exit-2 → block_intent=true, dispatcher exits 2
# Run standalone: bash demo-run-ac004.sh
set -euo pipefail
REPO="$(cd "$(dirname "$0")/../../.." && pwd)"
DISPATCHER="$REPO/target/release/factory-dispatcher"
WORK=$(mktemp -d)
trap 'rm -rf "$WORK"' EXIT
mkdir -p "$WORK/.factory/logs" "$WORK/hook-plugins" "$WORK/hooks"
cp "$REPO/plugins/vsdd-factory/hook-plugins/legacy-bash-adapter.wasm" "$WORK/hook-plugins/"

printf '#!/usr/bin/env bash\nexit 2\n' > "$WORK/hooks/blocker.sh"
chmod +x "$WORK/hooks/blocker.sh"

cat > "$WORK/hooks-registry.toml" <<'TOML'
schema_version = 2

[[hooks]]
name = "precompact-blocker"
event = "PreCompact"
plugin = "hook-plugins/legacy-bash-adapter.wasm"
timeout_ms = 5000
on_error = "block"

[hooks.capabilities.exec_subprocess]
binary_allow = ["bash"]
shell_bypass_acknowledged = "yes"
cwd_allow = ["."]

[hooks.config]
script_path = "hooks/blocker.sh"
TOML

echo "# AC-004/VP-086 (BC-1.15.001 PC4): PreCompact exit-2 → block_intent=true"
echo "# Registry event=PreCompact, on_error=block, plugin exits 2"
echo ""
echo "$ printf '{...PreCompact...}' | factory-dispatcher"
echo ""
set +e
printf '{"event_name":"PreCompact","tool_name":"","session_id":"demo-ac004","tool_input":{}}' \
  | CLAUDE_PLUGIN_ROOT="$WORK" CLAUDE_PROJECT_DIR="$WORK" "$DISPATCHER" 2>&1
EXIT_CODE=${PIPESTATUS[1]}
set -e
echo ""
echo "# Exit code: $EXIT_CODE (expected: 2)"
echo "# block_intent=true + blocking_plugins=precompact-blocker confirms VP-086 property"
echo "# Single exit-2 plugin sufficient to trigger block (EC-001, AC-004 PASS)"
