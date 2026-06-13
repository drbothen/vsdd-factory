#!/usr/bin/env bash
# setup-cas-fixture.sh — build the real git bare+clone fixture for AC-005 CAS demos
# Called by VHS tapes; outputs path to demo workdir
#
# Sets up:
#   /tmp/s1701-cas/bare.git   — shared bare remote
#   /tmp/s1701-cas/clone-b    — session-B's .factory worktree (wants to push)
#   /tmp/s1701-cas/racer      — racer clone (will push between fetch and push)
#   /tmp/s1701-cas/workdir    — demo workdir (has .factory -> clone-b symlink)
#
# Usage:
#   source setup-cas-fixture.sh
#   # Then: CAS_WORKDIR, CAS_BARE, CAS_CLONE_B, CAS_RACER are set

set -euo pipefail

CAS_DIR=/tmp/s1701-cas
rm -rf "$CAS_DIR"
mkdir -p "$CAS_DIR"

CAS_BARE="$CAS_DIR/bare.git"
CAS_CLONE_B="$CAS_DIR/clone-b"
CAS_RACER="$CAS_DIR/racer"
CAS_WORKDIR="$CAS_DIR/workdir"

# Bootstrap bare repo
git init "$CAS_BARE" --bare -b factory-artifacts >/dev/null 2>&1
BOOTSTRAP="$CAS_DIR/bootstrap"
git clone "$CAS_BARE" "$BOOTSTRAP" >/dev/null 2>&1
git -C "$BOOTSTRAP" config user.email "demo@vsdd.local"
git -C "$BOOTSTRAP" config user.name "Demo"
echo "v0: initial STATE.md" > "$BOOTSTRAP/STATE.md"
git -C "$BOOTSTRAP" add .
git -C "$BOOTSTRAP" commit -m "v0: initial" >/dev/null 2>&1
git -C "$BOOTSTRAP" push origin HEAD:factory-artifacts >/dev/null 2>&1
rm -rf "$BOOTSTRAP"

# Clone B — session-B (will try to push, will be rejected)
git clone "$CAS_BARE" "$CAS_CLONE_B" >/dev/null 2>&1
git -C "$CAS_CLONE_B" config user.email "session-b@vsdd.local"
git -C "$CAS_CLONE_B" config user.name "Session B"
# Session B writes a burst commit
echo "v1: session-B factory_lock burst work" > "$CAS_CLONE_B/STATE.md"
git -C "$CAS_CLONE_B" add .
git -C "$CAS_CLONE_B" commit -m "v1: session-B burst — factory_lock added" >/dev/null 2>&1

# Racer clone (simulates another concurrent session)
git clone "$CAS_BARE" "$CAS_RACER" >/dev/null 2>&1
git -C "$CAS_RACER" config user.email "racer@vsdd.local"
git -C "$CAS_RACER" config user.name "Racer Session"
echo "v1: racer concurrent write" > "$CAS_RACER/STATE.md"
git -C "$CAS_RACER" add .
git -C "$CAS_RACER" commit -m "v1: racer concurrent write" >/dev/null 2>&1

# Create workdir with .factory -> clone-b
mkdir -p "$CAS_WORKDIR"
ln -sf "$CAS_CLONE_B" "$CAS_WORKDIR/.factory"

export CAS_WORKDIR CAS_BARE CAS_CLONE_B CAS_RACER CAS_DIR

printf 'CAS fixture ready:\n'
printf '  bare:    %s\n' "$CAS_BARE"
printf '  clone-b: %s (has local burst commit)\n' "$CAS_CLONE_B"
printf '  racer:   %s (will push between fetch+push)\n' "$CAS_RACER"
printf '  workdir: %s (.factory -> clone-b)\n' "$CAS_WORKDIR"
