#!/usr/bin/env bats
#
# envelope-sync-invariant.bats
#
# AC-010: hooks.json.template and all 5 platform variants must have NO
# "async": true flags in any event entry after the envelope flip (T-3g).
#
# BC-9.01.006 postconditions 1-3: all platform hooks.json.* files must be
# uniformly synchronous — async key absent from every event entry.
#
# RED until T-3g: The current hooks.json.* files may still contain "async": true.
# After T-3g: this test passes (grep returns zero hits).
#
# BC traces:
#   BC-9.01.006 v1.2 — hooks.json.template envelope sync invariant
#   AC-010 (S-15.01 v1.6)
#   ADR-019 Decision 1: every Claude Code hook event must be sync at the envelope

HOOKS_DIR="plugins/vsdd-factory/hooks"

@test "AC-010: no hooks.json.* file contains \"async\": true (envelope flip)" {
    # RED until T-3g: hooks.json.* files may still have "async": true entries.
    # After T-3g: grep returns zero hits and this test passes.

    if [ ! -d "$HOOKS_DIR" ]; then
        skip "hooks directory not found at $HOOKS_DIR"
    fi

    local violations
    violations=$(grep -rl '"async": true' "$HOOKS_DIR" 2>/dev/null || true)

    [ -z "$violations" ] || {
        echo "FAIL: AC-010 envelope flip not yet applied — async:true found in:"
        echo "$violations"
        echo ""
        echo "Note: RED until T-3g implements the envelope flip across all 5 platform variants."
        echo "ADR-019 Decision 1: every Claude Code hook event must be sync at the envelope."
        return 1
    }
}

@test "AC-010: hooks.json.template exists and lacks async:true" {
    # Verify the template file specifically (all 5 platform variants derive from it).

    local template="$HOOKS_DIR/hooks.json.template"
    if [ ! -f "$template" ]; then
        # Try alternate locations.
        template=$(find . -name "hooks.json.template" -not -path "*/target/*" 2>/dev/null | head -1)
        [ -n "$template" ] || skip "hooks.json.template not found"
    fi

    ! grep -q '"async": true' "$template" || {
        echo "FAIL: hooks.json.template contains \"async\": true — T-3g must remove it."
        echo "ADR-019 Decision 1: every Claude Code hook event must be sync at the envelope."
        return 1
    }
}

@test "AC-010: all 5 platform variant files exist after T-3g" {
    # Verify all required platform variants are present.
    # RED until T-3g: these may not exist yet.

    local platforms=("darwin-arm64" "darwin-x64" "linux-x64" "linux-arm64" "windows-x64")
    local missing=()

    for platform in "${platforms[@]}"; do
        local path="$HOOKS_DIR/hooks.json.$platform"
        if [ ! -f "$path" ]; then
            missing+=("$path")
        fi
    done

    [ ${#missing[@]} -eq 0 ] || {
        echo "FAIL: Missing platform hooks.json variants (T-3g scope):"
        printf '  %s\n' "${missing[@]}"
        return 1
    }
}
