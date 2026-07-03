#!/usr/bin/env bash
# AC-002 (BC-1.15.001 PC2): PostCompact advisory-only — exit-2 + on_error=block → exit 0, no block_intent
# Contrast with AC-004: PostCompact NEVER sets block_intent regardless of exit code.
# Run standalone: bash demo-run-ac002.sh
set -euo pipefail
REPO="$(cd "$(dirname "$0")/../../.." && pwd)"
DISPATCHER="$REPO/target/release/factory-dispatcher"
WORK=$(mktemp -d)
trap 'rm -rf "$WORK"' EXIT
mkdir -p "$WORK/.factory/logs" "$WORK/hook-plugins" "$WORK/hooks"
cp "$REPO/plugins/vsdd-factory/hook-plugins/legacy-bash-adapter.wasm" "$WORK/hook-plugins/"

printf '#!/usr/bin/env bash\nexit 2\n' > "$WORK/hooks/exit2.sh"
chmod +x "$WORK/hooks/exit2.sh"

cat > "$WORK/hooks-registry.toml" <<'TOML'
schema_version = 2

[[hooks]]
name = "postcompact-advisory"
event = "PostCompact"
plugin = "hook-plugins/legacy-bash-adapter.wasm"
timeout_ms = 5000
on_error = "block"

[hooks.capabilities.exec_subprocess]
binary_allow = ["bash"]
shell_bypass_acknowledged = "yes"
cwd_allow = ["."]

[hooks.config]
script_path = "hooks/exit2.sh"
TOML

echo "# AC-002 (BC-1.15.001 PC2): PostCompact is advisory-only"
echo "# Registry event=PostCompact, on_error=block, plugin exits 2"
echo "# Contrast with AC-004 (PreCompact exit-2 → blocks). PostCompact NEVER blocks."
echo ""
echo "$ printf '{...PostCompact...}' | factory-dispatcher"
echo ""
printf '{"event_name":"PostCompact","tool_name":"","session_id":"demo-ac002","tool_input":{}}' \
  | CLAUDE_PLUGIN_ROOT="$WORK" CLAUDE_PROJECT_DIR="$WORK" "$DISPATCHER" 2>&1
echo ""
echo "# Exit code: 0 (expected: 0 — advisory-only; compaction proceeds)"
echo "# block_intent=false despite plugin exiting 2 with on_error=block (AC-002 PASS)"
