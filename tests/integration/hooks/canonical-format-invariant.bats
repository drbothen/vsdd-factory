#!/usr/bin/env bats
# tests/integration/hooks/canonical-format-invariant.bats
#
# Integration test: every blocking bash hook emits a single-line stderr
# message in the canonical Why/Fix/Code format:
#
#   BLOCKED by <hook-name>: <reason>. Fix: <recommendation>. Code: <code>.
#
# For each hook with a synthesizable block path, this test:
#   1. Constructs a minimal JSON input that will trigger the block.
#   2. Runs the hook with CLAUDE_PLUGIN_ROOT set so it sources lib/block.sh.
#   3. Asserts exit code 2 and that the first stderr line matches the regex.
#
# Hooks using block_pre_json (protect-bc, protect-vp) emit JSON to stdout
# with permissionDecisionReason; those are tested via jq parsing instead.
#
# Run: bats tests/integration/hooks/canonical-format-invariant.bats

# Canonical format: BLOCKED by <hook-name>: <reason>. Fix: <recommendation>. Code: <code>.
# hook-name: lowercase letters and hyphens
# reason/recommendation: any content (may contain periods internally)
# code: lowercase letters, digits, and underscores
CANONICAL_RE='^BLOCKED by [a-z][a-z0-9-]*: .+\. Fix: .+\. Code: [a-z][a-z0-9_]*\.$'

setup() {
    REPO_ROOT="$(git -C "$(dirname "$BATS_TEST_FILENAME")" rev-parse --show-toplevel)"
    HOOKS_DIR="${REPO_ROOT}/plugins/vsdd-factory/hooks"
    export CLAUDE_PLUGIN_ROOT="${REPO_ROOT}/plugins/vsdd-factory"
    WORK="$(mktemp -d)"
}

teardown() {
    rm -rf "${WORK:-}" 2>/dev/null || true
}

# ---------------------------------------------------------------------------
# Helper: run hook, check exit 2 and canonical stderr
# ---------------------------------------------------------------------------
check_block() {
    local hook="$1"
    local json="$2"

    # Use bats run to capture stderr+stdout and exit status
    # run merges stdout+stderr into $output when not using run --separate-stderr
    # We redirect hook stdout to /dev/null and capture stderr as the output
    run bash -c "echo '$(echo "$json" | sed "s/'/'\\\\''/g")' | bash '$hook' 2>&1 >/dev/null"
    local exitcode="$status"
    local stderr_content="$output"

    if [[ "$exitcode" -ne 2 ]]; then
        echo "Expected exit 2 but got $exitcode" >&3
        echo "Output: $stderr_content" >&3
        return 1
    fi

    local first_line
    first_line=$(printf '%s' "$stderr_content" | head -1)
    if ! echo "$first_line" | grep -qE "$CANONICAL_RE"; then
        echo "Stderr first line does not match canonical format:" >&3
        echo "  Got:   $first_line" >&3
        echo "  Regex: $CANONICAL_RE" >&3
        return 1
    fi
    return 0
}

# check_json_deny: for hooks that emit permissionDecision: deny (exit 0)
check_json_deny() {
    local hook="$1"
    local json="$2"

    if ! command -v jq &>/dev/null; then
        skip "jq not available"
    fi

    local stdout exitcode
    stdout=$(echo "$json" | bash "$hook" 2>/dev/null)
    exitcode=$?

    # block_pre_json exits 0
    if [[ "$exitcode" -ne 0 ]]; then
        echo "Expected exit 0 (JSON deny) but got $exitcode" >&3
        return 1
    fi

    local decision
    decision=$(echo "$stdout" | jq -r '.hookSpecificOutput.permissionDecision // empty' 2>/dev/null || true)
    if [[ "$decision" != "deny" ]]; then
        echo "Expected permissionDecision: deny but got: $decision" >&3
        return 1
    fi

    local reason
    reason=$(echo "$stdout" | jq -r '.hookSpecificOutput.permissionDecisionReason // empty' 2>/dev/null || true)
    if ! echo "$reason" | grep -qE "$CANONICAL_RE"; then
        echo "permissionDecisionReason does not match canonical format:" >&3
        echo "  Got:   $reason" >&3
        echo "  Regex: $CANONICAL_RE" >&3
        return 1
    fi
    return 0
}

# ---------------------------------------------------------------------------
# verify-git-push — force push
# ---------------------------------------------------------------------------
@test "verify-git-push: force push canonical block" {
    if ! command -v jq &>/dev/null; then skip "jq not available"; fi
    local json='{"tool_name":"Bash","tool_input":{"command":"git push origin main --force"}}'
    check_block "${HOOKS_DIR}/verify-git-push.sh" "$json"
}

@test "verify-git-push: protected branch canonical block" {
    if ! command -v jq &>/dev/null; then skip "jq not available"; fi
    local json='{"tool_name":"Bash","tool_input":{"command":"git push origin main"}}'
    check_block "${HOOKS_DIR}/verify-git-push.sh" "$json"
}

# ---------------------------------------------------------------------------
# destructive-command-guard
# ---------------------------------------------------------------------------
@test "destructive-command-guard: git reset --hard canonical block" {
    if ! command -v jq &>/dev/null; then skip "jq not available"; fi
    local json='{"tool_name":"Bash","tool_input":{"command":"git reset --hard HEAD~1"}}'
    check_block "${HOOKS_DIR}/destructive-command-guard.sh" "$json"
}

# ---------------------------------------------------------------------------
# protect-secrets
# ---------------------------------------------------------------------------
@test "protect-secrets: echo secret variable canonical block" {
    if ! command -v jq &>/dev/null; then skip "jq not available"; fi
    local json
    # Use a JSON string that will trigger the secret pattern
    json='{"tool_name":"Bash","tool_input":{"command":"echo $MY_API_KEY"}}'
    check_block "${HOOKS_DIR}/protect-secrets.sh" "$json"
}

# ---------------------------------------------------------------------------
# brownfield-discipline
# ---------------------------------------------------------------------------
@test "brownfield-discipline: write to .reference/ canonical block" {
    if ! command -v jq &>/dev/null; then skip "jq not available"; fi
    local json='{"tool_name":"Edit","tool_input":{"file_path":"/project/.reference/some-repo/file.py"}}'
    check_block "${HOOKS_DIR}/brownfield-discipline.sh" "$json"
}

# ---------------------------------------------------------------------------
# protect-bc (JSON deny path)
# ---------------------------------------------------------------------------
@test "protect-bc: green BC canonical deny JSON" {
    if ! command -v jq &>/dev/null; then skip "jq not available"; fi
    local fake_path="${WORK}/.factory/specs/behavioral-contracts/BC-1.01.001.md"
    mkdir -p "$(dirname "$fake_path")"
    echo "Status: green" > "$fake_path"
    local json="{\"tool_name\":\"Edit\",\"tool_input\":{\"file_path\":\"${fake_path}\"}}"
    check_json_deny "${HOOKS_DIR}/protect-bc.sh" "$json"
}

# ---------------------------------------------------------------------------
# protect-vp (JSON deny path)
# ---------------------------------------------------------------------------
@test "protect-vp: green VP canonical deny JSON" {
    if ! command -v jq &>/dev/null; then skip "jq not available"; fi
    local fake_path="${WORK}/.factory/specs/verification-properties/VP-1.01.001.md"
    mkdir -p "$(dirname "$fake_path")"
    echo "Status: green" > "$fake_path"
    local json="{\"tool_name\":\"Edit\",\"tool_input\":{\"file_path\":\"${fake_path}\"}}"
    check_json_deny "${HOOKS_DIR}/protect-vp.sh" "$json"
}

# ---------------------------------------------------------------------------
# validate-bc-title
# ---------------------------------------------------------------------------
@test "validate-bc-title: H1/INDEX mismatch canonical block" {
    if ! command -v jq &>/dev/null; then skip "jq not available"; fi
    local bc_dir="${WORK}/behavioral-contracts"
    mkdir -p "$bc_dir"
    local bc_file="${bc_dir}/BC-1.01.001.md"
    echo "# BC-1.01.001: Correct Title" > "$bc_file"
    local bc_index="${bc_dir}/BC-INDEX.md"
    echo "| BC-1.01.001 | Wrong Title | SS-01 |" > "$bc_index"
    local json="{\"tool_name\":\"Edit\",\"tool_input\":{\"file_path\":\"${bc_file}\"}}"
    check_block "${HOOKS_DIR}/validate-bc-title.sh" "$json"
}

# ---------------------------------------------------------------------------
# validate-state-size
# ---------------------------------------------------------------------------
@test "validate-state-size: STATE.md over 500 lines canonical block" {
    if ! command -v jq &>/dev/null; then skip "jq not available"; fi
    local factory_dir="${WORK}/.factory"
    mkdir -p "$factory_dir"
    local state_file="${factory_dir}/STATE.md"
    python3 -c "print('\n'.join(['line ' + str(i) for i in range(510)]))" > "$state_file"
    local json="{\"tool_name\":\"Write\",\"tool_input\":{\"file_path\":\"${state_file}\"}}"
    check_block "${HOOKS_DIR}/validate-state-size.sh" "$json"
}

# ---------------------------------------------------------------------------
# validate-demo-evidence-story-scoped
# ---------------------------------------------------------------------------
@test "validate-demo-evidence-story-scoped: flat demo-evidence file canonical block" {
    if ! command -v jq &>/dev/null; then skip "jq not available"; fi
    local evidence_dir="${WORK}/docs/demo-evidence"
    mkdir -p "$evidence_dir"
    local flat_file="${evidence_dir}/evidence-report.md"
    echo "# Evidence" > "$flat_file"
    local json="{\"tool_name\":\"Write\",\"tool_input\":{\"file_path\":\"${flat_file}\"}}"
    check_block "${HOOKS_DIR}/validate-demo-evidence-story-scoped.sh" "$json"
}

# ---------------------------------------------------------------------------
# validate-factory-path-root
# ---------------------------------------------------------------------------
@test "validate-factory-path-root: worktree-relative path canonical block" {
    if ! command -v jq &>/dev/null; then skip "jq not available"; fi
    local json='{"tool_name":"Write","tool_input":{"file_path":"/project/.worktrees/STORY-001/.factory/STATE.md"}}'
    check_block "${HOOKS_DIR}/validate-factory-path-root.sh" "$json"
}

# ---------------------------------------------------------------------------
# validate-subsystem-names
# ---------------------------------------------------------------------------
@test "validate-subsystem-names: unknown SS-ID canonical block" {
    if ! command -v jq &>/dev/null; then skip "jq not available"; fi
    local factory_dir="${WORK}/.factory"
    local bc_dir="${factory_dir}/specs/behavioral-contracts"
    local arch_dir="${factory_dir}/specs/architecture"
    mkdir -p "$bc_dir" "$arch_dir"

    local bc_file="${bc_dir}/BC-1.01.001.md"
    cat > "$bc_file" <<'EOF'
---
subsystem: SS-99
---
# BC-1.01.001: Test
EOF

    local arch_index="${arch_dir}/ARCH-INDEX.md"
    cat > "$arch_index" <<'EOF'
# ARCH-INDEX

## Subsystem Registry

| SS ID | Name | Architecture Doc | Implementing Modules | Phase |
|---|---|---|---|---|
| SS-01 | Core | arch/ss-01.md | crates/core | 1 |
| SS-02 | API | arch/ss-02.md | crates/api | 1 |
EOF

    local json="{\"tool_name\":\"Edit\",\"tool_input\":{\"file_path\":\"${bc_file}\"}}"
    check_block "${HOOKS_DIR}/validate-subsystem-names.sh" "$json"
}

# ---------------------------------------------------------------------------
# validate-table-cell-count
# ---------------------------------------------------------------------------
@test "validate-table-cell-count: wrong cell count canonical block" {
    if ! command -v jq &>/dev/null; then skip "jq not available"; fi
    local factory_dir="${WORK}/.factory"
    mkdir -p "$factory_dir"
    local md_file="${factory_dir}/some-doc.md"
    cat > "$md_file" <<'EOF'
| Col1 | Col2 | Col3 |
|---|---|---|
| a | b | c | extra |
EOF
    local json="{\"tool_name\":\"Write\",\"tool_input\":{\"file_path\":\"${md_file}\"}}"
    check_block "${HOOKS_DIR}/validate-table-cell-count.sh" "$json"
}

# ---------------------------------------------------------------------------
# validate-pr-description-completeness
# ---------------------------------------------------------------------------
@test "validate-pr-description-completeness: missing sections canonical block" {
    if ! command -v jq &>/dev/null; then skip "jq not available"; fi
    local delivery_dir="${WORK}/.factory/code-delivery/STORY-001"
    mkdir -p "$delivery_dir"
    local pr_file="${delivery_dir}/pr-description.md"
    echo "# PR Description" > "$pr_file"
    echo "" >> "$pr_file"
    echo "Some content without required sections." >> "$pr_file"
    local json="{\"tool_name\":\"Write\",\"tool_input\":{\"file_path\":\"${pr_file}\"}}"
    check_block "${HOOKS_DIR}/validate-pr-description-completeness.sh" "$json"
}

# ---------------------------------------------------------------------------
# validate-novelty-assessment
# ---------------------------------------------------------------------------
@test "validate-novelty-assessment: missing section canonical block" {
    if ! command -v jq &>/dev/null; then skip "jq not available"; fi
    local adv_dir="${WORK}/.factory/cycles/E-1/adversarial-reviews"
    mkdir -p "$adv_dir"
    local review_file="${adv_dir}/pass-1.md"
    cat > "$review_file" <<'EOF'
# Adversary Review Pass 1

## Summary

Some content without novelty assessment section.
EOF
    local json="{\"tool_name\":\"Write\",\"tool_input\":{\"file_path\":\"${review_file}\"}}"
    check_block "${HOOKS_DIR}/validate-novelty-assessment.sh" "$json"
}

# ---------------------------------------------------------------------------
# validate-input-hash — wrong hash format
# ---------------------------------------------------------------------------
@test "validate-input-hash: wrong hash length canonical block" {
    if ! command -v jq &>/dev/null; then skip "jq not available"; fi
    local factory_dir="${WORK}/.factory"
    mkdir -p "$factory_dir"
    local md_file="${factory_dir}/story.md"
    cat > "$md_file" <<'EOF'
---
inputs:
  - some-file.md
input-hash: abcdef01234
---
# Story
EOF
    local json="{\"tool_name\":\"Write\",\"tool_input\":{\"file_path\":\"${md_file}\"}}"
    check_block "${HOOKS_DIR}/validate-input-hash.sh" "$json"
}

# ---------------------------------------------------------------------------
# validate-state-pin-freshness
# ---------------------------------------------------------------------------
@test "validate-state-pin-freshness: version pin drift canonical block" {
    if ! command -v jq &>/dev/null; then skip "jq not available"; fi
    local factory_dir="${WORK}/.factory"
    local bc_specs_dir="${factory_dir}/specs/behavioral-contracts"
    mkdir -p "$bc_specs_dir"

    local bc_index="${bc_specs_dir}/BC-INDEX.md"
    cat > "$bc_index" <<'EOF'
---
version: v1.5
---
# BC-INDEX
EOF

    local state_file="${factory_dir}/STATE.md"
    cat > "$state_file" <<'EOF'
---
bc_index_version: v1.4
---
# STATE
EOF

    local json="{\"tool_name\":\"Write\",\"tool_input\":{\"file_path\":\"${state_file}\"}}"
    check_block "${HOOKS_DIR}/validate-state-pin-freshness.sh" "$json"
}

# ---------------------------------------------------------------------------
# red-gate — strict mode
# ---------------------------------------------------------------------------
@test "red-gate: strict mode canonical block" {
    if ! command -v jq &>/dev/null; then skip "jq not available"; fi
    local state_file="${WORK}/.factory/red-gate-state.json"
    mkdir -p "$(dirname "$state_file")"
    echo '{"mode":"strict","red":[]}' > "$state_file"

    local src_file="${WORK}/src/lib.rs"
    mkdir -p "$(dirname "$src_file")"
    echo "// source" > "$src_file"

    local json="{\"tool_name\":\"Edit\",\"tool_input\":{\"file_path\":\"${src_file}\"}}"

    # red-gate reads .factory/red-gate-state.json relative to CWD, so we must
    # cd to WORK before running. Use run bash -c to avoid set -e tripping on exit 2.
    run bash -c "cd '${WORK}' && printf '%s' '$(printf '%s' "$json" | sed "s/'/'\\\\''/g")' | bash '${HOOKS_DIR}/red-gate.sh' 2>&1 >/dev/null"
    local exitcode="$status"
    local stderr_content="$output"

    if [[ "$exitcode" -ne 2 ]]; then
        echo "Expected exit 2 but got $exitcode" >&3
        echo "Output: $stderr_content" >&3
        return 1
    fi

    local first_line
    first_line=$(printf '%s' "$stderr_content" | head -1)
    if ! echo "$first_line" | grep -qE "$CANONICAL_RE"; then
        echo "Stderr first line does not match canonical format:" >&3
        echo "  Got:   $first_line" >&3
        echo "  Regex: $CANONICAL_RE" >&3
        return 1
    fi
}

# ---------------------------------------------------------------------------
# factory-branch-guard — .factory/ not a worktree
# ---------------------------------------------------------------------------
@test "factory-branch-guard: non-worktree .factory/ canonical block" {
    if ! command -v jq &>/dev/null; then skip "jq not available"; fi
    local factory_dir="${WORK}/.factory"
    mkdir -p "$factory_dir"
    # No .git file → not a worktree
    local json="{\"tool_name\":\"Write\",\"tool_input\":{\"file_path\":\"${factory_dir}/STATE.md\"}}"
    check_block "${HOOKS_DIR}/factory-branch-guard.sh" "$json"
}
