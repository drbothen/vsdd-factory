#!/usr/bin/env bash
# demo-setup.sh — shared fixture setup for precompact-flush demos.
# Sourced by each demo script. Sets WORK, PROJECT_DIR, FACTORY_ARTS,
# BARE_REMOTE, FAKE_HOME, DISPATCHER, WORK_WASM.
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
PRECOMPACT_WASM="$REPO_ROOT/plugins/vsdd-factory/hook-plugins/precompact-flush.wasm"

WORK="$(mktemp -d)"
PROJECT_DIR="$WORK/project"
BARE_REMOTE="$WORK/bare-remote.git"
FACTORY_ARTS="$PROJECT_DIR/.factory"
FAKE_HOME="$WORK/home"
WORK_WASM="$WORK/hook-plugins/precompact-flush.wasm"

mkdir -p "$WORK/.factory/logs" "$WORK/hook-plugins" "$PROJECT_DIR" "$FAKE_HOME"
cp "$PRECOMPACT_WASM" "$WORK_WASM"

cat > "$FAKE_HOME/.gitconfig" <<'GITCFG'
[user]
  name = Demo Agent
  email = demo@factory.local
[init]
  defaultBranch = develop
GITCFG

# Bare remote
HOME="$FAKE_HOME" git init --bare "$BARE_REMOTE" >/dev/null 2>&1

# Main repo at PROJECT_DIR (== CLAUDE_PROJECT_DIR; must be a git repo)
HOME="$FAKE_HOME" git init "$PROJECT_DIR" >/dev/null 2>&1
git -c user.name="Demo Agent" -c user.email="demo@factory.local" \
    -C "$PROJECT_DIR" commit --allow-empty -m "init develop" >/dev/null 2>&1
git -C "$PROJECT_DIR" remote add origin "$BARE_REMOTE" >/dev/null 2>&1

# factory-artifacts branch
git -C "$PROJECT_DIR" checkout -b factory-artifacts >/dev/null 2>&1
mkdir -p "$PROJECT_DIR/hooks"
cat > "$PROJECT_DIR/STATE.md" <<'STATEMD'
---
document_type: state
version: "0.0.1-demo"
current_cycle: v1.0-brownfield-backfill
current_step: demo-phase/S-18.04a
---
# STATE (demo fixture)
STATEMD
touch "$PROJECT_DIR/hooks/.gitkeep"
git -c user.name="Demo Agent" -c user.email="demo@factory.local" \
    -C "$PROJECT_DIR" add "STATE.md" "hooks" >/dev/null 2>&1
git -c user.name="Demo Agent" -c user.email="demo@factory.local" \
    -C "$PROJECT_DIR" commit -m "init factory-artifacts" >/dev/null 2>&1
git -C "$PROJECT_DIR" push origin factory-artifacts >/dev/null 2>&1

# Back to develop, mount worktree
git -C "$PROJECT_DIR" checkout develop >/dev/null 2>&1
git -C "$PROJECT_DIR" worktree add "$FACTORY_ARTS" factory-artifacts >/dev/null 2>&1

# Write registry
cat > "$WORK/hooks-registry.toml" <<EOF
schema_version = 2

[[hooks]]
name = "precompact-flush"
event = "PreCompact"
plugin = "hook-plugins/precompact-flush.wasm"
priority = 100
timeout_ms = 30000
on_error = "continue"
async = false

[hooks.capabilities.read_file]
path_allow = ["${PROJECT_DIR}/.factory/"]

[hooks.capabilities.write_file]
path_allow = ["${PROJECT_DIR}/.factory/"]

[hooks.capabilities.exec_subprocess]
binary_allow = ["git"]
env_allow = ["HOME", "GIT_CONFIG_GLOBAL", "XDG_CONFIG_HOME", "PATH", "SSH_AUTH_SOCK"]
EOF

export DISPATCHER WORK PROJECT_DIR FACTORY_ARTS BARE_REMOTE FAKE_HOME WORK_WASM

_run_dispatcher() {
    local envelope="$1"
    printf '%s' "$envelope" | \
        CLAUDE_PLUGIN_ROOT="$WORK" \
        CLAUDE_PROJECT_DIR="$PROJECT_DIR" \
        HOME="$FAKE_HOME" \
        "$DISPATCHER" 2>&1
}
export -f _run_dispatcher
