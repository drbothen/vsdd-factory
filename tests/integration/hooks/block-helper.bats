#!/usr/bin/env bats
# tests/integration/hooks/block-helper.bats
#
# Unit tests for plugins/vsdd-factory/hooks/lib/block.sh.
#
# Tests:
#   1. block_pre exits 2
#   2. block_pre stderr is exactly one line in canonical format
#   3. block_pre does not duplicate trailing periods
#   4. block_pre_json exits 0 and emits valid JSON with permissionDecision: "deny"
#      and permissionDecisionReason matching the canonical line
#
# Run: bats tests/integration/hooks/block-helper.bats

setup() {
    REPO_ROOT="$(git -C "$(dirname "$BATS_TEST_FILENAME")" rev-parse --show-toplevel)"
    BLOCK_SH="${REPO_ROOT}/plugins/vsdd-factory/hooks/lib/block.sh"
}

# ---------------------------------------------------------------------------
# Test 1: block_pre exits 2
# ---------------------------------------------------------------------------

@test "block_pre exits 2" {
    run bash -c "source '${BLOCK_SH}'; block_pre my-hook 'Something went wrong' 'Try this fix' 'err_code' 2>/dev/null"
    [ "$status" -eq 2 ]
}

# ---------------------------------------------------------------------------
# Test 2: block_pre stderr is exactly one line in canonical BLOCKED format
# ---------------------------------------------------------------------------

@test "block_pre emits exactly one stderr line in BLOCKED by X: Y. Fix: Z. Code: W. format" {
    run bash -c "source '${BLOCK_SH}'; block_pre my-hook 'Something went wrong' 'Try this fix' 'err_code'" 2>&1
    # Count non-empty lines in the output
    line_count=$(printf '%s' "$output" | grep -c '.' || true)
    [ "$line_count" -eq 1 ]
    # Must match the canonical format
    echo "$output" | grep -q '^BLOCKED by my-hook: Something went wrong\. Fix: Try this fix\. Code: err_code\.$'
}

# ---------------------------------------------------------------------------
# Test 3: block_pre does not duplicate trailing periods on reason/recommendation
# ---------------------------------------------------------------------------

@test "block_pre strips trailing periods to avoid double-period sequences" {
    run bash -c "source '${BLOCK_SH}'; block_pre my-hook 'Reason already ends in period.' 'Recommendation already ends.' 'code'" 2>&1
    # Must not contain any double-period
    echo "$output" | grep -qv '\.\.'
    # Must still contain single-period-terminated reason and recommendation
    echo "$output" | grep -q 'Reason already ends in period\.'
    echo "$output" | grep -q 'Recommendation already ends\.'
}

# ---------------------------------------------------------------------------
# Test 4: block_pre_json exits 0 and emits valid JSON with permissionDecision deny
# ---------------------------------------------------------------------------

@test "block_pre_json exits 0 and emits JSON with permissionDecision: deny" {
    if ! command -v jq &>/dev/null; then
        skip "jq not available"
    fi
    run bash -c "source '${BLOCK_SH}'; block_pre_json verify-git-push 'Force push is destructive' 'Use --force-with-lease instead' 'git_force_push'"
    [ "$status" -eq 0 ]
    # Output must be valid JSON
    echo "$output" | jq empty
    # Must contain permissionDecision: "deny"
    decision=$(echo "$output" | jq -r '.hookSpecificOutput.permissionDecision')
    [ "$decision" = "deny" ]
}

@test "block_pre_json permissionDecisionReason matches canonical line format" {
    if ! command -v jq &>/dev/null; then
        skip "jq not available"
    fi
    run bash -c "source '${BLOCK_SH}'; block_pre_json verify-git-push 'Force push is destructive' 'Use --force-with-lease instead' 'git_force_push'"
    [ "$status" -eq 0 ]
    reason=$(echo "$output" | jq -r '.hookSpecificOutput.permissionDecisionReason')
    [ "$reason" = "BLOCKED by verify-git-push: Force push is destructive. Fix: Use --force-with-lease instead. Code: git_force_push." ]
}
