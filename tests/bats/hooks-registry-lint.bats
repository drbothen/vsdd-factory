#!/usr/bin/env bats
#
# hooks-registry-lint.bats
#
# VP-078 Harness 1 (negative CI lint) + Harness 2 (dispatcher integration).
#
# AC-006: on_error=block implies async=false — enforced at three layers.
# AC-008: CI-PR lint scan of the live hooks-registry.toml.
#
# RED (Harness 1): The live registry does not yet have schema_version=2 or
#   async classifications; the Python lint scan may pass vacuously until T-3h.
#   The Harness 2 dispatcher integration test panics (todo!() in validate()).
#
# RED (Harness 2): factory-dispatcher binary has todo!() in validate_async_block_invariant().
#   Any invocation exits non-zero with a panic message, not a clean E-REG-002 diagnostic.
#
# DI-019: ASYNC_DRAIN_WINDOW_MS — referenced by name; do NOT hardcode 100.
#
# BC traces:
#   BC-7.06.001 v1.7 — schema_version 2 + on_error=block implies async=false
#   VP-078 v1.8 Harnesses 1+2
#   AC-006, AC-008 (S-15.01 v1.14)

REGISTRY="plugins/vsdd-factory/hooks-registry.toml"

# ---------------------------------------------------------------------------
# VP-078 Harness 1: negative CI lint (file-level scan)
# ---------------------------------------------------------------------------

# ---------------------------------------------------------------------------
# VP-078 H1a supporting helpers (positive + negative controls)
# These run BEFORE the live-registry integration probe to confirm the Python
# regex actually fires (POLICY 11 — no_test_tautologies, F-P1-011 fix).
# ---------------------------------------------------------------------------

# Shared Python snippet for the VP-078 H1a lint logic.
# Used in positive-control, negative-control, and live-registry tests.
_lint_registry_content() {
    python3 -c "
import re, sys

content = sys.stdin.read()

entries = re.split(r'\[\[hooks\]\]', content)[1:]  # skip preamble

bad = []
for i, entry in enumerate(entries, 1):
    has_block = bool(re.search(r'on_error\s*=\s*\"block\"', entry))
    has_async = bool(re.search(r'async\s*=\s*true', entry))
    if has_block and has_async:
        m = re.search(r'name\s*=\s*\"([^\"]+)\"', entry)
        name = m.group(1) if m else f'entry #{i}'
        bad.append(name)

if bad:
    print('VIOLATION: on_error=block AND async=true in: ' + ', '.join(bad))
    sys.exit(1)
"
}

@test "VP-078 H1a-positive-control: regex DETECTS on_error=block + async=true co-occurrence" {
    # POLICY 11 (no_test_tautologies) fix — F-P1-011.
    # This positive-control test proves the Python regex in H1a FIRES when a violation
    # is present. Without this, H1a could have a broken regex and still pass (vacuously).
    #
    # Method: construct a temp file with a known-violating [[hooks]] entry,
    # run the lint, and assert a violation IS detected.

    local violating_toml
    violating_toml=$(mktemp)
    cat > "$violating_toml" <<'TOML'
schema_version = 2

[[hooks]]
name = "positive-control-violator"
plugin = "hook-plugins/test.wasm"
on_error = "block"
async = true
event = "PostToolUse"
priority = 100
TOML

    # Run lint directly via python3 (not the helper function) to capture both
    # stdout and exit status cleanly in bats' `run` environment.
    run python3 -c "
import re, sys

with open('${violating_toml}') as f:
    content = f.read()

entries = re.split(r'\[\[hooks\]\]', content)[1:]

bad = []
for i, entry in enumerate(entries, 1):
    has_block = bool(re.search(r'on_error\s*=\s*\"block\"', entry))
    has_async = bool(re.search(r'async\s*=\s*true', entry))
    if has_block and has_async:
        m = re.search(r'name\s*=\s*\"([^\"]+)\"', entry)
        name = m.group(1) if m else 'entry #' + str(i)
        bad.append(name)

if bad:
    print('VIOLATION: on_error=block AND async=true in: ' + ', '.join(bad))
    sys.exit(1)
"
    rm -f "$violating_toml"

    [ "$status" -ne 0 ] || {
        echo "FAIL: positive-control — lint regex failed to detect on_error=block + async=true"
        echo "      The H1a regex is broken or not firing. VP-078 H1a cannot be trusted."
        return 1
    }

    [[ "$output" == *"VIOLATION"* ]] || {
        echo "FAIL: positive-control — lint exited non-zero but output lacks 'VIOLATION'."
        echo "      Got: $output"
        return 1
    }
}

@test "VP-078 H1a-negative-control: regex does NOT fire on valid on_error=block + async=false" {
    # POLICY 11 (no_test_tautologies) fix — F-P1-011.
    # This negative-control test proves the Python regex does NOT produce false positives
    # when on_error=block and async=false (valid combination).
    #
    # Method: construct a temp file with a valid [[hooks]] entry having on_error=block
    # but async=false, run the lint, and assert NO violation is detected.

    local valid_toml
    valid_toml=$(mktemp)
    cat > "$valid_toml" <<'TOML'
schema_version = 2

[[hooks]]
name = "negative-control-validator"
plugin = "hook-plugins/test.wasm"
on_error = "block"
async = false
event = "PostToolUse"
priority = 100
TOML

    run python3 -c "
import re, sys

with open('${valid_toml}') as f:
    content = f.read()

entries = re.split(r'\[\[hooks\]\]', content)[1:]

bad = []
for i, entry in enumerate(entries, 1):
    has_block = bool(re.search(r'on_error\s*=\s*\"block\"', entry))
    has_async = bool(re.search(r'async\s*=\s*true', entry))
    if has_block and has_async:
        m = re.search(r'name\s*=\s*\"([^\"]+)\"', entry)
        name = m.group(1) if m else 'entry #' + str(i)
        bad.append(name)

if bad:
    print('VIOLATION: on_error=block AND async=true in: ' + ', '.join(bad))
    sys.exit(1)
"
    rm -f "$valid_toml"

    [ "$status" -eq 0 ] || {
        echo "FAIL: negative-control — lint produced false-positive on on_error=block + async=false."
        echo "      Got: $output"
        echo "      The H1a regex has a false-positive. VP-078 H1a cannot be trusted."
        return 1
    }

    [ -z "$output" ] || {
        echo "FAIL: negative-control — lint produced unexpected output on valid entry."
        echo "      Got: $output"
        return 1
    }
}

@test "VP-078 H1a: no hooks-registry entry has both on_error=block and async=true" {
    # Integration probe against the live hooks-registry.toml.
    #
    # NOTE: This test passes vacuously when the live registry contains no on_error=block
    # entries combined with async=true — which is the expected clean state. A future
    # engineer adding a violating entry must be caught here.
    #
    # The regex's ability to FIRE is validated separately by the positive-control test
    # above (H1a-positive-control). The two tests together make H1a non-tautological.
    #
    # Implementation: split the file into per-entry blocks on [[hooks]] boundaries,
    # then for each block assert the pair does not co-occur.

    local violations
    violations=$(python3 - <<'EOF'
import re, sys

registry = "plugins/vsdd-factory/hooks-registry.toml"
try:
    with open(registry) as f:
        content = f.read()
except FileNotFoundError:
    print(f"SKIP: {registry} not found")
    sys.exit(0)

# Split on [[hooks]] boundaries
entries = re.split(r'\[\[hooks\]\]', content)[1:]  # skip preamble

bad = []
for i, entry in enumerate(entries, 1):
    has_block = bool(re.search(r'on_error\s*=\s*"block"', entry))
    has_async = bool(re.search(r'async\s*=\s*true', entry))
    if has_block and has_async:
        m = re.search(r'name\s*=\s*"([^"]+)"', entry)
        name = m.group(1) if m else f"entry #{i}"
        bad.append(name)

if bad:
    print("VIOLATION: on_error=block AND async=true in: " + ", ".join(bad))
    sys.exit(1)
EOF
    )

    [ -z "$violations" ] || {
        echo "$violations"
        return 1
    }
}

@test "VP-078 H1b: live registry has schema_version=2 after T-3h" {
    # RED until T-3h: live registry still has schema_version=1.
    # After T-3h ships: this test passes.

    if [ ! -f "$REGISTRY" ]; then
        skip "hooks-registry.toml not found — skipping schema_version check"
    fi

    local schema_version
    schema_version=$(python3 -c "
import sys
try:
    import tomllib
except ImportError:
    import tomli as tomllib

with open('$REGISTRY', 'rb') as f:
    data = tomllib.load(f)
print(data.get('schema_version', 'missing'))
" 2>/dev/null || python3 -c "
import re
with open('$REGISTRY') as f:
    content = f.read()
m = re.search(r'schema_version\s*=\s*(\d+)', content)
print(m.group(1) if m else 'missing')
")

    [ "$schema_version" = "2" ] || {
        echo "FAIL: hooks-registry.toml schema_version is '$schema_version', expected '2' (T-3h not yet implemented)"
        return 1
    }
}

# ---------------------------------------------------------------------------
# VP-078 Harness 2: dispatcher integration — block+async registry rejected
# ---------------------------------------------------------------------------

@test "VP-078 H2: dispatcher rejects registry with on_error=block AND async=true entry" {
    # RED: factory-dispatcher validate_async_block_invariant() is todo!() —
    # the dispatcher panics rather than emitting a clean E-REG-002 diagnostic.
    # After T-3f: this test passes with non-zero exit and recognizable error output.

    # Check that factory-dispatcher binary exists.
    if ! command -v factory-dispatcher &>/dev/null; then
        skip "factory-dispatcher binary not found — run cargo build first"
    fi

    local plugin_root
    plugin_root=$(mktemp -d)

    # Write a violating registry (on_error=block AND async=true).
    cat > "$plugin_root/hooks-registry.toml" <<'TOML'
schema_version = 2

[[hooks]]
name = "violating-plugin"
plugin = "hook-plugins/legacy-bash-adapter.wasm"
on_error = "block"
async = true
event = "PostToolUse"
priority = 400

[hooks.config]
script_path = "test-fixtures/exit0.sh"
TOML

    # Minimal valid stdin envelope.
    local envelope='{"hook_event_name":"PostToolUse","tool_name":"Write","session_id":"test-session-001","tool_input":{}}'

    # Dispatcher must exit non-zero when it encounters the block+async invariant violation.
    run sh -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT=\"$plugin_root\" RUST_LOG=error factory-dispatcher 2>&1"
    [ "$status" -ne 0 ] || {
        echo "FAIL: dispatcher must exit non-zero on block+async violation (E-REG-002)"
        rm -rf "$plugin_root"
        return 1
    }
    # Removed `not yet implemented` panic-fallback disjunct (was Red-Gate; production code is now fully implemented; F-P2-005).
    # F-P3-005: Tightened from 4 disjuncts to 2 — only canonical signals (E-REG-002 error code OR registry_invalid event name).
    # Broad matchers (`async`, `on_error`) removed to prevent false-PASS on unrelated panics.
    [[ "$output" == *"registry_invalid"* ]] || \
        [[ "$output" == *"E-REG-002"* ]] || {
        echo "FAIL: output must name the violation or error code. Got: $output"
        rm -rf "$plugin_root"
        return 1
    }

    rm -rf "$plugin_root"
}

@test "VP-078 H2: dispatcher rejects schema_version=1 registry with E-REG-001 exit" {
    # RED: factory-dispatcher validate_async_block_invariant() is todo!() —
    # the schema_version check already fails before reaching todo!().
    # This test verifies the schema-version enforcement exits non-zero.

    if ! command -v factory-dispatcher &>/dev/null; then
        skip "factory-dispatcher binary not found — run cargo build first"
    fi

    local plugin_root
    plugin_root=$(mktemp -d)

    cat > "$plugin_root/hooks-registry.toml" <<'TOML'
schema_version = 1

[[hooks]]
name = "legacy-plugin"
plugin = "hook-plugins/legacy-bash-adapter.wasm"
on_error = "continue"
event = "PreToolUse"
priority = 100

[hooks.config]
script_path = "test-fixtures/exit0.sh"
TOML

    local envelope='{"hook_event_name":"PreToolUse","tool_name":"Write","session_id":"test-session-002","tool_input":{}}'

    run sh -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT=\"$plugin_root\" RUST_LOG=error factory-dispatcher 2>&1"
    [ "$status" -ne 0 ] || {
        echo "FAIL: dispatcher must exit non-zero on schema_version=1 registry (E-REG-001)"
        rm -rf "$plugin_root"
        return 1
    }

    rm -rf "$plugin_root"
}
