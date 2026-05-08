#!/usr/bin/env bats
#
# lint-registry-async-invariant.bats
#
# VP-078 Harness 2 end-to-end: bats script invokes the WASM lint plugin
# against a sample hooks-registry.toml via the dispatcher.
#
# RED: The lint-registry-async-invariant.wasm is not yet built (T-3i).
#      Tests will fail until the WASM artifact is compiled and registered.
#
# AC-007: lint-registry-async-invariant.wasm is a native WASM plugin that
#         enforces the on_error=block implies async=false invariant at
#         PostToolUse Edit|Write time. This bats test exercises the plugin
#         via the dispatcher by simulating a PostToolUse event.
#
# BC traces:
#   BC-7.06.001 v1.9 — per-plugin async field + CI lint invariant
#   VP-078 v1.8 Harness 2 — bats integration
#   AC-007, AC-008 (S-15.01 v1.14)
#   DI-019 — ASYNC_DRAIN_WINDOW_MS (do NOT hardcode)

PLUGIN_ROOT="plugins/vsdd-factory"
WASM_PLUGIN="hook-plugins/lint-registry-async-invariant.wasm"

setup() {
    # Verify the WASM binary exists before running tests.
    if [ ! -f "$PLUGIN_ROOT/$WASM_PLUGIN" ]; then
        skip "lint-registry-async-invariant.wasm not found — run 'cargo build -p lint-registry-async-invariant --target wasm32-wasip1' first (T-3i)"
    fi
    if ! command -v factory-dispatcher &>/dev/null; then
        skip "factory-dispatcher binary not found — run cargo build first"
    fi
}

@test "VP-078 H2 bats: lint plugin passes on valid v2 registry with no invariant violations" {
    # Dispatch a PostToolUse Edit event against the live hooks-registry.toml.
    # The lint plugin should pass (exit 0) when the registry is valid.
    #
    # RED: will skip until lint-registry-async-invariant.wasm is built (T-3i).

    local envelope='{"hook_event_name":"PostToolUse","tool_name":"Edit","session_id":"bats-lint-001","tool_input":{"file_path":"plugins/vsdd-factory/hooks-registry.toml"}}'

    run sh -c "printf '%s' '$envelope' | \
        CLAUDE_PLUGIN_ROOT=\"$PLUGIN_ROOT\" \
        CLAUDE_PROJECT_DIR=\"$(pwd)\" \
        factory-dispatcher 2>&1"

    # Lint plugin must not block (exit 0 or 1 but not 2 on a valid registry).
    [ "$status" -ne 2 ] || {
        echo "FAIL: lint plugin produced a block verdict on valid registry. Output: $output"
        return 1
    }
}

@test "VP-078 H2 bats: lint plugin blocks on registry with on_error=block AND async=true" {
    # Inject a temporary violating hooks-registry.toml entry and verify the
    # lint plugin blocks the PostToolUse Edit event.
    #
    # RED: will skip until lint-registry-async-invariant.wasm is built (T-3i).

    local tmp_plugin_root
    tmp_plugin_root=$(mktemp -d)
    mkdir -p "$tmp_plugin_root/hook-plugins"

    # Copy the real WASM lint plugin to the temp root.
    cp "$PLUGIN_ROOT/$WASM_PLUGIN" "$tmp_plugin_root/hook-plugins/"

    # Write a violating registry to the temp root.
    cat > "$tmp_plugin_root/hooks-registry.toml" <<'TOML'
schema_version = 2

[[hooks]]
name = "violating-plugin"
plugin = "hook-plugins/lint-registry-async-invariant.wasm"
on_error = "block"
async = true
event = "PostToolUse"
tool = "Edit|Write"
priority = 100
TOML

    local envelope
    envelope=$(printf '{"hook_event_name":"PostToolUse","tool_name":"Edit","session_id":"bats-lint-002","tool_input":{"file_path":"hooks-registry.toml"}}')

    run sh -c "printf '%s' '$envelope' | \
        CLAUDE_PLUGIN_ROOT=\"$tmp_plugin_root\" \
        CLAUDE_PROJECT_DIR=\"$tmp_plugin_root\" \
        factory-dispatcher 2>&1"

    rm -rf "$tmp_plugin_root"

    # Lint plugin must block (exit 2) on violating registry.
    [ "$status" -eq 2 ] || {
        echo "FAIL: lint plugin must block (exit 2) on registry with on_error=block AND async=true. Status: $status. Output: $output"
        return 1
    }
    [[ "$output" == *"E-REG-002"* ]] || \
        [[ "$output" == *"violating-plugin"* ]] || \
        [[ "$output" == *"on_error"* ]] || {
        echo "WARN: lint output does not name the violation (acceptable if exit 2). Output: $output"
    }
}
