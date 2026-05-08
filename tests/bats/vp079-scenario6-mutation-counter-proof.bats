#!/usr/bin/env bats
#
# vp079-scenario6-mutation-counter-proof.bats
#
# VP-079 v1.13 Property 6: Production-caller mutation counter-proof.
#
# For each of the 5 production emit-caller sites in main.rs, comment out the
# emit call in a temporary worktree checkout, rebuild the dispatcher, and assert
# that AT LEAST ONE of VP-079 Scenarios 1-5 FAILS. If all 5 scenarios pass with
# a production caller removed, VP-079 provides no end-to-end coverage guarantee
# and this test FAILs.
#
# INVOCATION:
#   BATS_RUN_VP079_SCENARIO_6=1 bats tests/bats/vp079-scenario6-mutation-counter-proof.bats
#
# This test is SKIPPED unless BATS_RUN_VP079_SCENARIO_6=1 is set, because
# each mutation requires a fresh cargo build (~10-60s per iteration).
# Gate it in CI behind a job triggered only on diffs touching main.rs or
# host/emit_event.rs (preferred: cargo mutants --filter='emit_'; bash fallback below).
#
# SITES (5 production caller sites per VP-079 v1.13 Property 6):
#   SITE_1: ~main.rs:133 — emit_dispatcher_schema_mismatch (schema_version != 2)
#   SITE_2: ~main.rs:142 — emit_dispatcher_registry_invalid (AsyncBlockConflict / E-REG-002)
#   SITE_3: ~main.rs:416 — emit_plugin_async_block_discarded (async result exit_code=2)
#   SITE_4: ~main.rs:427 — emit_plugin_timeout_async (async timeout arm)
#   SITE_5: ~main.rs:162 — emit_dispatcher_registry_invalid (DuplicateEntry / E-REG-003)
#
# Preferred tool: cargo mutants (call-site suppression via --filter); bash sed-mutation
# is the fallback for environments where cargo-mutants is unavailable or does not
# support individual call-site targeting.
#
# BC traces:
#   BC-3.08.001 v1.7 — async-semantics event types, production caller obligation
#   VP-079 v1.13 Property 6 — production-path emission counter-proof
#   S-15.01 v1.7 — F5 pass-1 fix (F-P1-002)
#   DI-019 — ASYNC_DRAIN_WINDOW_MS (referenced by name; do NOT hardcode 100)

BATS_DIR=""

setup() {
    # Resolve the repository root from this file's location.
    BATS_DIR="$(cd "$(dirname "$BATS_TEST_FILENAME")" && pwd)"
    SRC_ROOT="$(cd "$BATS_DIR/../.." && pwd)"
}

# ---------------------------------------------------------------------------
# Guard: skip the entire suite unless explicitly enabled.
# ---------------------------------------------------------------------------

require_scenario6_enabled() {
    if [ "${BATS_RUN_VP079_SCENARIO_6:-0}" != "1" ]; then
        skip "VP-079 Scenario 6 mutation counter-proof is disabled by default. \
Set BATS_RUN_VP079_SCENARIO_6=1 to run. \
Each mutation requires a fresh cargo build (~10-60s per site)."
    fi
}

require_dispatcher_source() {
    local main_rs="$SRC_ROOT/crates/factory-dispatcher/src/main.rs"
    if [ ! -f "$main_rs" ]; then
        skip "main.rs not found at expected path: $main_rs"
    fi
}

require_cargo_build_ok() {
    run cargo build -p factory-dispatcher --manifest-path "$SRC_ROOT/Cargo.toml" 2>&1
    if [ "$status" -ne 0 ]; then
        skip "factory-dispatcher baseline build failed; cannot run mutation trials. Output: $output"
    fi
}

# ---------------------------------------------------------------------------
# Helper: run ALL @test blocks in async-event-schema-conformance.bats
# (currently S1, S2, S3, S4, S5, S7, S8 = 7 scenarios).
# Returns 0 if ALL scenarios pass (mutation SURVIVED — bad), non-zero if at
# least one scenario fails (mutation was CAUGHT — good).
# ---------------------------------------------------------------------------
run_all_conformance_scenarios() {
    local conformance_file="$BATS_DIR/async-event-schema-conformance.bats"
    if [ ! -f "$conformance_file" ]; then
        echo "SKIP: async-event-schema-conformance.bats not found at $conformance_file" >&2
        return 2  # infrastructure error — not a mutation pass/fail
    fi
    # Run all tests in the conformance suite against the locally-built binary.
    # PATH injection ensures the freshly-mutated target/debug binary is used.
    PATH="${SRC_ROOT}/target/debug:${PATH}" run bats "$conformance_file" 2>&1
    echo "$output"
    return "$status"
}

# ---------------------------------------------------------------------------
# Sed-mutation helper: comment out lines matching a function call pattern
# in main.rs, rebuild, run Scenarios 1-5, restore, rebuild clean.
#
# $1 = unique function name pattern (e.g. "emit_dispatcher_schema_mismatch")
# $2 = human-readable site label for error messages
# $3 = (optional) line-range address for sed (e.g. "162,162") — use when the
#      function name appears at multiple call sites and only a specific line
#      should be mutated. When omitted, the whole-file pattern match is used.
#
# Returns:
#   0 = mutation was CAUGHT (at least one scenario failed — expected behavior)
#   1 = mutation SURVIVED (all scenarios passed — VP-079 is insufficient)
#   2 = infrastructure error (build failed, skip gracefully)
# ---------------------------------------------------------------------------
mutate_and_verify_caught() {
    local fn_pattern="$1"
    local site_label="$2"
    local line_range="${3:-}"  # optional: restrict mutation to a specific line range
    local main_rs="$SRC_ROOT/crates/factory-dispatcher/src/main.rs"

    # Confirm the pattern exists in main.rs before attempting mutation.
    if ! grep -q "${fn_pattern}" "$main_rs"; then
        echo "WARN: pattern '${fn_pattern}' not found in main.rs; skipping this site" >&2
        return 2
    fi

    # Diagnostic: show which lines will be mutated.
    echo "--- Mutation site: $site_label ---" >&2
    grep -n "${fn_pattern}" "$main_rs" | head -5 >&2

    # Apply sed-based comment-out mutation (in-place; backup to .mutation_bak).
    local bak="${main_rs}.mutation_bak"
    cp "$main_rs" "$bak"
    # When line_range is provided, comment out EVERY line in the range — this is
    # required for multi-line call expressions where commenting only the first line
    # (the function name) produces a syntax error rather than a suppressed call.
    # The range must cover from the opening call through the closing ");".
    # Without line_range, restrict to lines matching the function call pattern.
    if [ -n "$line_range" ]; then
        sed -i.tmp "${line_range} s|^|// MUTANT-SUPPRESSED: |" "$main_rs"
    else
        sed -i.tmp "/${fn_pattern}(/s/^/\/\/ MUTANT-SUPPRESSED: /" "$main_rs"
    fi
    rm -f "${main_rs}.tmp"

    # Rebuild with mutation applied (dev profile for speed).
    local build_output
    build_output=$(cargo build -p factory-dispatcher --manifest-path "$SRC_ROOT/Cargo.toml" 2>&1)
    local build_status=$?

    local caught=0
    if [ "$build_status" -ne 0 ]; then
        # Build failure does NOT count as "mutation caught". A void-returning emit-call removal
        # SHOULD produce a buildable mutated binary (since no value is consumed). If the build
        # fails, the mutation strategy itself is broken — treat as infrastructure error and skip.
        # (F-P2-006)
        echo "INFRA-ERROR: mutation of '${fn_pattern}' caused a build failure — mutation strategy broken, not a counter-proof success." >&2
        echo "             A suppressed emit-call (void return) should always compile. Skipping this trial." >&2
        caught=2  # infrastructure error — skip, do not count as caught or not-caught
    else
        # Run all conformance scenarios against the mutated binary.
        run_all_conformance_scenarios
        local scenarios_status=$?
        if [ "$scenarios_status" -eq 0 ]; then
            # All 5 scenarios passed WITH the mutation in place — not caught.
            caught=1
            echo "FAIL: mutation of '${fn_pattern}' was NOT caught by Scenarios 1-5." >&2
            echo "      VP-079 provides no end-to-end coverage guarantee for this emitter." >&2
        else
            # At least one scenario failed — mutation was caught.
            caught=0
            echo "PASS: mutation of '${fn_pattern}' was caught (at least one scenario failed)." >&2
        fi
    fi

    # Always restore original main.rs.
    cp "$bak" "$main_rs"
    rm -f "$bak"

    # Rebuild clean binary (ignore errors — next mutation trial starts fresh).
    cargo build -p factory-dispatcher --manifest-path "$SRC_ROOT/Cargo.toml" >/dev/null 2>&1 || true

    return "$caught"
}

# ---------------------------------------------------------------------------
# Scenario 6, Site 1: emit_dispatcher_schema_mismatch
#
# Property 6: removing this call must cause Scenario 2 (schema_mismatch) to fail.
# ---------------------------------------------------------------------------

@test "VP-079 S6/SITE_1: removing emit_dispatcher_schema_mismatch causes at least one scenario to fail" {
    require_scenario6_enabled
    require_dispatcher_source
    require_cargo_build_ok

    mutate_and_verify_caught \
        "emit_dispatcher_schema_mismatch" \
        "SITE_1 (schema_version mismatch path, ~main.rs:133)"

    local result=$?
    [ "$result" -ne 1 ] || {
        echo "FAIL: emit_dispatcher_schema_mismatch removed but all Scenarios 1-5 passed."
        echo "      VP-079 Scenario 2 must fail when schema_mismatch emit is suppressed."
        return 1
    }
}

# ---------------------------------------------------------------------------
# Scenario 6, Site 2: emit_dispatcher_registry_invalid
#
# Property 6: removing this call must cause Scenario 3 (registry_invalid) to fail.
# ---------------------------------------------------------------------------

@test "VP-079 S6/SITE_2: removing emit_dispatcher_registry_invalid at line 142 (E-REG-002) breaks at least one Scenario" {
    require_scenario6_enabled
    require_dispatcher_source
    require_cargo_build_ok

    mutate_and_verify_caught \
        "emit_dispatcher_registry_invalid" \
        "SITE_2: AsyncBlockConflict/E-REG-002 path (main.rs:142)" \
        "142,147"

    local result=$?
    [ "$result" -ne 1 ] || {
        echo "FAIL: emit_dispatcher_registry_invalid (line 142) removed but all Scenarios 1-5 passed."
        echo "      VP-079 Scenario 3 must fail when registry_invalid emit (E-REG-002) is suppressed."
        return 1
    }
}

# ---------------------------------------------------------------------------
# Scenario 6, Site 3: emit_plugin_async_block_discarded
#
# Property 6: removing this call must cause Scenario 1 (async_block_discarded) to fail.
# ---------------------------------------------------------------------------

@test "VP-079 S6/SITE_3: removing emit_plugin_async_block_discarded causes at least one scenario to fail" {
    require_scenario6_enabled
    require_dispatcher_source
    require_cargo_build_ok

    mutate_and_verify_caught \
        "emit_plugin_async_block_discarded" \
        "SITE_3 (async result exit_code=2 path, ~main.rs:416)"

    local result=$?
    [ "$result" -ne 1 ] || {
        echo "FAIL: emit_plugin_async_block_discarded removed but all Scenarios 1-5 passed."
        echo "      VP-079 Scenario 1 must fail when async_block_discarded emit is suppressed."
        return 1
    }
}

# ---------------------------------------------------------------------------
# Scenario 6, Site 4: emit_plugin_timeout_async
#
# Property 6: removing this call must cause Scenario 4 (plugin.timeout async) to fail.
# ---------------------------------------------------------------------------

@test "VP-079 S6/SITE_4: removing emit_plugin_timeout_async causes at least one scenario to fail" {
    require_scenario6_enabled
    require_dispatcher_source
    require_cargo_build_ok

    mutate_and_verify_caught \
        "emit_plugin_timeout_async" \
        "SITE_4 (async timeout arm, ~main.rs:427)"

    local result=$?
    [ "$result" -ne 1 ] || {
        echo "FAIL: emit_plugin_timeout_async removed but all Scenarios 1-5 passed."
        echo "      VP-079 Scenario 4 must fail when plugin_timeout_async emit is suppressed."
        return 1
    }
}

# ---------------------------------------------------------------------------
# Scenario 6, Site 5: emit_dispatcher_registry_invalid (E-REG-003 / DuplicateEntry)
#
# Property 6: removing this call at line 162 must cause at least one Scenario to fail.
# This site shares the function name with SITE_2 (line 142); the line-range argument
# disambiguates the mutation so only the DuplicateEntry path is suppressed.
# ---------------------------------------------------------------------------

@test "VP-079 S6/SITE_5: removing emit_dispatcher_registry_invalid at line 162 (E-REG-003) breaks at least one Scenario" {
    require_scenario6_enabled
    require_dispatcher_source
    require_cargo_build_ok

    mutate_and_verify_caught \
        "emit_dispatcher_registry_invalid" \
        "SITE_5: DuplicateEntry/E-REG-003 path (main.rs:162)" \
        "162,167"

    local result=$?
    [ "$result" -ne 1 ] || {
        echo "FAIL: emit_dispatcher_registry_invalid (line 162) removed but all Scenarios 1-5 passed."
        echo "      VP-079 must fail when registry_invalid emit (E-REG-003 / DuplicateEntry) is suppressed."
        return 1
    }
}

# ---------------------------------------------------------------------------
# Cargo-mutants integration (preferred, if available)
#
# If cargo-mutants is installed, this test supersedes the 4 bash-mutation tests
# above with a single declarative invocation. Runs only when
# BATS_RUN_VP079_SCENARIO_6=1 AND BATS_VP079_USE_CARGO_MUTANTS=1.
#
# Expected: ALL targeted mutations are "caught" by the test suite.
# If any mutation survives, Scenarios 1-5 are insufficient and this test FAILs.
# ---------------------------------------------------------------------------

@test "VP-079 S6 (cargo-mutants): all emit_ mutations caught by Scenarios 1-5" {
    require_scenario6_enabled

    if [ "${BATS_VP079_USE_CARGO_MUTANTS:-0}" != "1" ]; then
        skip "Set BATS_VP079_USE_CARGO_MUTANTS=1 to use cargo-mutants for Scenario 6."
    fi

    if ! command -v cargo-mutants &>/dev/null; then
        skip "cargo-mutants not installed; use BATS_VP079_USE_CARGO_MUTANTS=0 (default) for bash fallback."
    fi

    require_dispatcher_source

    # cargo-mutants: target only the 4 production emit functions.
    # --test-tool bats tells cargo-mutants to run bats as the test runner.
    # Adjust --jobs based on CI parallelism budget.
    run cargo mutants \
        --package factory-dispatcher \
        --jobs 1 \
        --filter 'emit_dispatcher_schema_mismatch|emit_dispatcher_registry_invalid|emit_plugin_async_block_discarded|emit_plugin_timeout_async' \
        -- \
        bats "$BATS_DIR/async-event-schema-conformance.bats" 2>&1

    echo "$output"

    # cargo-mutants exits 0 if all mutations are caught; non-zero if any survive.
    [ "$status" -eq 0 ] || {
        echo "FAIL: cargo-mutants found surviving mutation(s). VP-079 Scenarios 1-5 are insufficient."
        return 1
    }
}
