#!/usr/bin/env bash
# run-fetch-failure-demo.sh — demonstrate AC-010 fetch failure aborts CAS push
#
# Creates a real git fixture, then removes the bare remote so fetch fails
# with a "repository not found" error. The push MUST NOT be attempted.

set -uo pipefail

WORKTREE=/Users/zious/Documents/GITHUB/vsdd-factory/.worktrees/S-17.01
HELPER="$WORKTREE/plugins/vsdd-factory/bin/factory-cas-push.sh"

echo "=== Building real git bare+clone fixture ==="
FETCH_DIR=/tmp/s1701-fetch-fail
rm -rf "$FETCH_DIR"
mkdir -p "$FETCH_DIR"

BARE="$FETCH_DIR/bare.git"
CLONE="$FETCH_DIR/clone"
WORKDIR="$FETCH_DIR/workdir"

# Bootstrap
git init "$BARE" --bare -b factory-artifacts >/dev/null 2>&1
BOOTSTRAP="$FETCH_DIR/bootstrap"
git clone "$BARE" "$BOOTSTRAP" >/dev/null 2>&1
git -C "$BOOTSTRAP" config user.email "demo@vsdd.local"
git -C "$BOOTSTRAP" config user.name "Demo"
echo "v0: initial" > "$BOOTSTRAP/STATE.md"
git -C "$BOOTSTRAP" add .
git -C "$BOOTSTRAP" commit -m "v0: initial" >/dev/null 2>&1
git -C "$BOOTSTRAP" push origin HEAD:factory-artifacts >/dev/null 2>&1
rm -rf "$BOOTSTRAP"

# Clone (represents .factory worktree)
git clone "$BARE" "$CLONE" >/dev/null 2>&1
git -C "$CLONE" config user.email "demo@vsdd.local"
git -C "$CLONE" config user.name "Demo"

# Write a local burst commit
echo "v1: burst work" > "$CLONE/STATE.md"
git -C "$CLONE" add .
git -C "$CLONE" commit -m "v1: burst work" >/dev/null 2>&1

# Create workdir with .factory -> clone
mkdir -p "$WORKDIR"
ln -sf "$CLONE" "$WORKDIR/.factory"

echo ""
echo "=== Clone local log ==="
git -C "$CLONE" log --oneline | head -3
echo ""
echo "=== Remote path: $BARE ==="
echo ""

echo "=== [SABOTAGE] Removing bare remote directory to cause fetch failure ==="
rm -rf "$BARE"
echo "Remote directory removed: $BARE"
echo ""

echo "=== Running factory-cas-push.sh (fetch will fail — remote is gone) ==="
cd "$WORKDIR" && bash "$HELPER" 2>&1
RESULT=$?
echo ""
echo "Exit code: $RESULT (expected: 1)"
echo ""
echo "=== Verify push was NOT attempted (clone log is unchanged) ==="
git -C "$CLONE" log --oneline | head -3
