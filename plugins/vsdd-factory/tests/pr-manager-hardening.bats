#!/usr/bin/env bats
# pr-manager-hardening.bats — S-19.01 integration tests.
#
# Covers AC-001..AC-004 per BC-5.42.001 / VP-094 / ADR-030 §Decision 2 / §Decision 3.
#
# Test plan (T-001..T-019):
#   T-001 AC-001 — READY verdict without covered_sha triggers READY_SHA_MISSING advisory
#   T-002 AC-001 — gh failure on covered_sha fetch → READY_SHA_FETCH_FAILED on stderr
#   T-003 AC-002 — check-stale-verdict.sh: stale SHA → exit 1 + STALE_READY_VERDICT
#   T-004 AC-002 — check-stale-verdict.sh: matching SHA → exit 0 (fresh verdict)
#   T-005 AC-003 — enforce-merge-strategy.sh: release/v* + --squash → exit 1 + RELEASE_PR_SQUASH_FORBIDDEN
#   T-006 AC-003 — enforce-merge-strategy.sh: release/v* + --merge → exit 0 (allowed)
#   T-007 AC-003 — enforce-merge-strategy.sh: non-release + --squash → exit 0 (delegated)
#   T-008 AC-004 — darwin-leg preflight: wrong interpreter exits 1 + DARWIN_LEG_WRONG_INTERPRETER
#   T-009 AC-004 — darwin-leg preflight: Linux runners skip gracefully (exit 0)
#   T-010 AC-002 — check-stale-verdict.sh: malformed covered_sha → READY_SHA_MISSING (TV4)
#   T-011 AC-003 — enforce-merge-strategy.sh: release/v* + --rebase → RELEASE_PR_SQUASH_FORBIDDEN (TV3)
#   T-012 AC-003 — enforce-merge-strategy.sh: release/v* + no flag → defaults --merge (EC-005/TV5)
#   T-013 AC-002 — check-stale-verdict.sh: STALE_READY_VERDICT exact canonical format (BC-5.42.001 §b)
#   T-014 AC-001 — check-stale-verdict.sh: READY_SHA_FETCH_FAILED exact canonical format (EC-001)
#   T-015 AC-003 — enforce-merge-strategy.sh: RELEASE_PR_SQUASH_FORBIDDEN exact canonical format (§c)
#   T-016 AC-002 — check-stale-verdict.sh: 40-char uppercase-hex → READY_SHA_MISSING (EC-002)
#   T-017 AC-004 — darwin-leg fragment: while IFS= read -r bash-3.2 compat (regression pin rc.22)
#   T-018 AC-002 — check-stale-verdict.sh: closed PR (matching SHA) → CHECK_STALE_VERDICT_ERROR (EC-003)
#   T-019 AC-002 — check-stale-verdict.sh: malformed gh JSON → CHECK_STALE_VERDICT_ERROR (arm 4)
#
# Green status: T-001..T-016 pass after implementation; T-017 green (positive + neg-control);
#   T-018/T-019 GREEN — positive verification of ADR-030 §Decision 2 arms 3+4 (post-implementation).
#
# BC trace: BC-5.42.001 PC-1 (T-001/T-002/T-014), PC-2 (T-003/T-004/T-010/T-013/T-016/T-018/T-019),
#           PC-3 (T-005/T-006/T-007/T-011/T-012/T-015), AC-004 (T-008/T-009/T-017)

PLUGIN_ROOT=""
BIN_DIR=""
FIXTURES_DIR=""
MOCK_BIN=""

# Darwin-leg suite preflight (AC-004 mechanism gate, F-S1901-P1-003).
# On macOS: verifies /bin/bash --version contains "version 3.2" before any test runs.
# On Linux: no-op (darwin-leg tests individually skip via 'skip' bats directive).
# Exit 1 here aborts the whole file — intentional for the bats-darwin-leg-macos CI job.
#
# Also builds pr-manager-completion-guard.wasm on first use if absent (mirrors the
# S-19.03 on-demand build pattern from warn-pending-wave-gate.bats commit 5c3dbae9).
# In CI the WASM is not tracked in the repo; in local dev it may be absent after
# git rm --cached. The build step requires the wasm32-wasip1 target to be installed.
setup_file() {
    local repo_root
    repo_root="$(cd "${BATS_TEST_DIRNAME}/../../.." && pwd)"
    local wasm_dir="${repo_root}/plugins/vsdd-factory/hook-plugins"
    local wasm="${wasm_dir}/pr-manager-completion-guard.wasm"
    if [ ! -f "$wasm" ]; then
        echo "# setup_file: pr-manager-completion-guard.wasm absent; building..." >&3
        cargo build --release --target wasm32-wasip1 -p pr-manager-completion-guard 2>&1 | tail -5 >&2
        mkdir -p "$wasm_dir"
        cp "${repo_root}/target/wasm32-wasip1/release/pr-manager-completion-guard.wasm" "$wasm"
    fi
    # Darwin-leg /bin/bash 3.2 preflight — macOS only.
    if [[ "$(uname)" != "Darwin" ]]; then
        return 0
    fi
    local bash_version
    bash_version="$(/bin/bash --version 2>/dev/null | head -1)"
    if ! printf '%s' "${bash_version}" | grep -q 'version 3\.2'; then
        printf 'DARWIN_LEG_WRONG_INTERPRETER: expected /bin/bash 3.2.x, got %s\n' \
            "${bash_version}" >&2
        return 1
    fi
}

setup() {
    PLUGIN_ROOT="$(cd "${BATS_TEST_DIRNAME}/.." && pwd)"
    BIN_DIR="${PLUGIN_ROOT}/bin"
    FIXTURES_DIR="${BATS_TEST_DIRNAME}/fixtures/pr-manager"
    MOCK_BIN="$(mktemp -d "${BATS_TMPDIR}/mock-bin-XXXXXX")"

    # Locate factory-dispatcher binary (needed for T-001 WASM hook test)
    REPO_ROOT="$(cd "${PLUGIN_ROOT}/../.." && pwd)"
    DISPATCHER="${REPO_ROOT}/target/debug/factory-dispatcher"
    if [[ ! -x "${DISPATCHER}" ]]; then
        DISPATCHER="${REPO_ROOT}/target/release/factory-dispatcher"
    fi
    export PLUGIN_ROOT BIN_DIR FIXTURES_DIR MOCK_BIN REPO_ROOT DISPATCHER
}

teardown() {
    rm -rf "${MOCK_BIN}"
}

# ─────────────────────────────────────────────────────────────────────────────
# AC-001: covered_sha enforcement (BC-5.42.001 PC-1, Invariant 1, Invariant 5)
# ─────────────────────────────────────────────────────────────────────────────

# T-001: READY verdict without covered_sha must trigger READY_SHA_MISSING advisory
# from the pr-manager-completion-guard SubagentStop WASM hook.
#
# Red Gate: WASM hook does not yet inspect READY verdict for covered_sha field.
# After implementation: dispatcher emits READY_SHA_MISSING in VSDD_SINK_FILE.
@test "T-001: READY verdict without covered_sha triggers READY_SHA_MISSING advisory" {
    if [[ ! -x "${DISPATCHER}" ]]; then
        skip "factory-dispatcher binary not found at ${DISPATCHER}; run cargo build first"
    fi

    local sink_file
    sink_file="$(mktemp "${BATS_TMPDIR}/T001-sink-XXXXXX.jsonl")"

    # SubagentStop payload: pr-manager agent emits READY verdict WITHOUT covered_sha field
    local payload
    payload=$(printf '%s' '{"event_name":"SubagentStop","session_id":"T001","dispatcher_trace_id":"T001-trace","agent_type":"vsdd-factory:pr-manager","last_assistant_message":"READY: PR #42 has been reviewed. All checks pass. Steps 1-9 complete. STEP_COMPLETE: step=1 name=populate status=ok note= STEP_COMPLETE: step=2 name=demo status=ok note= STEP_COMPLETE: step=3 name=create-pr status=ok note= STEP_COMPLETE: step=4 name=security status=ok note= STEP_COMPLETE: step=5 name=review status=ok note= STEP_COMPLETE: step=6 name=checks status=ok note= STEP_COMPLETE: step=7 name=deps status=ok note= STEP_COMPLETE: step=8 name=merge status=ok note= STEP_COMPLETE: step=9 name=cleanup status=ok note="}')

    run bash -c "cd '${PLUGIN_ROOT}' && printf '%s' '${payload}' | VSDD_SINK_FILE='${sink_file}' CLAUDE_PLUGIN_ROOT='${PLUGIN_ROOT}' '${DISPATCHER}'"

    # Assertion: READY_SHA_MISSING must appear in sink events
    # (BC-5.42.001 PC-1; ADR-030 §Decision 1 advisory-block-mode)
    grep -q "READY_SHA_MISSING" "${sink_file}" 2>/dev/null || {
        echo "FAIL: READY_SHA_MISSING not found in sink events"
        echo "Sink contents:"
        cat "${sink_file}" 2>/dev/null || echo "(empty or missing)"
        echo "Dispatcher output: ${output}"
        return 1
    }

    rm -f "${sink_file}"
}

# T-002: gh failure during covered_sha fetch → check-stale-verdict.sh exits non-zero
# with READY_SHA_FETCH_FAILED on stderr (BC-5.42.001 EC-001 verbatim wording).
#
# Red Gate: check-stale-verdict.sh stub exits 99 with UNIMPLEMENTED.
# After implementation: exits 1 with READY_SHA_FETCH_FAILED on stderr.
@test "T-002: gh failure → READY_SHA_FETCH_FAILED on stderr (BC-5.42.001 EC-001)" {
    local pr_number="42"
    local covered_sha="aaaa000000000000000000000000000000000000"

    # Mock gh: exits non-zero to simulate network/auth failure
    cat > "${MOCK_BIN}/gh" <<'GHEOF'
#!/usr/bin/env bash
printf 'Error: HTTP 503: server temporarily unavailable\n' >&2
exit 1
GHEOF
    chmod +x "${MOCK_BIN}/gh"

    run env PATH="${MOCK_BIN}:${PATH}" bash "${BIN_DIR}/check-stale-verdict.sh" \
        "${pr_number}" "${covered_sha}" 2>&1

    [ "${status}" -ne 0 ] || {
        echo "FAIL: expected non-zero exit on gh failure; got exit ${status}"
        echo "Output: ${output}"
        return 1
    }
    echo "${output}" | grep -q "READY_SHA_FETCH_FAILED" || {
        echo "FAIL: READY_SHA_FETCH_FAILED not found in output"
        echo "Output: ${output}"
        return 1
    }
    echo "${output}" | grep -q "PR #${pr_number}" || {
        echo "FAIL: PR number not included in READY_SHA_FETCH_FAILED message"
        echo "Output: ${output}"
        return 1
    }
}

# ─────────────────────────────────────────────────────────────────────────────
# AC-002: stale-verdict detection (BC-5.42.001 PC-2, Invariant 2)
# ─────────────────────────────────────────────────────────────────────────────

# T-003: check-stale-verdict.sh exits 1 with STALE_READY_VERDICT when
# live headRefOid differs from covered_sha.
#
# Red Gate: check-stale-verdict.sh stub exits 99 with UNIMPLEMENTED.
# After implementation: exits 1 with STALE_READY_VERDICT diagnostic on stderr.
@test "T-003: check-stale-verdict.sh: stale SHA → exit 1 + STALE_READY_VERDICT" {
    local pr_number="42"
    local covered_sha="aaaa000000000000000000000000000000000000"
    local live_sha="bbbb111111111111111111111111111111111111"

    # Mock gh: returns a DIFFERENT live SHA (covered_sha != headRefOid)
    cat > "${MOCK_BIN}/gh" <<GHEOF
#!/usr/bin/env bash
if [[ "\$1" == "pr" && "\$2" == "view" && "\$*" == *"headRefOid"* ]]; then
    printf '{"headRefOid":"${live_sha}"}\n'
    exit 0
fi
printf 'MOCK_GH_UNHANDLED: %s\n' "\$*" >&2; exit 1
GHEOF
    chmod +x "${MOCK_BIN}/gh"

    run env PATH="${MOCK_BIN}:${PATH}" bash "${BIN_DIR}/check-stale-verdict.sh" \
        "${pr_number}" "${covered_sha}" 2>&1

    [ "${status}" -eq 1 ] || {
        echo "FAIL: expected exit 1 (fail-closed) on stale verdict; got ${status}"
        echo "Output: ${output}"
        return 1
    }
    echo "${output}" | grep -q "STALE_READY_VERDICT" || {
        echo "FAIL: STALE_READY_VERDICT sentinel not present in output"
        echo "Output: ${output}"
        return 1
    }
    echo "${output}" | grep -q "PR #${pr_number}" || {
        echo "FAIL: PR number not in STALE_READY_VERDICT message"
        echo "Output: ${output}"
        return 1
    }
    echo "${output}" | grep -q "${live_sha}" || {
        echo "FAIL: live SHA not in STALE_READY_VERDICT message"
        echo "Output: ${output}"
        return 1
    }
    echo "${output}" | grep -q "${covered_sha}" || {
        echo "FAIL: covered_sha not in STALE_READY_VERDICT message"
        echo "Output: ${output}"
        return 1
    }
}

# T-004: check-stale-verdict.sh exits 0 when covered_sha matches live headRefOid.
# Positive control: fresh verdict; safe to proceed with merge.
#
# Red Gate: check-stale-verdict.sh stub exits 99 with UNIMPLEMENTED.
# After implementation: exits 0 (silent success).
@test "T-004: check-stale-verdict.sh: matching SHA → exit 0 (fresh verdict)" {
    local pr_number="42"
    local sha="cccc222222222222222222222222222222222222"

    # Mock gh: returns THE SAME SHA as covered_sha (fresh verdict)
    cat > "${MOCK_BIN}/gh" <<GHEOF
#!/usr/bin/env bash
if [[ "\$1" == "pr" && "\$2" == "view" && "\$*" == *"headRefOid"* ]]; then
    printf '{"headRefOid":"${sha}"}\n'
    exit 0
fi
printf 'MOCK_GH_UNHANDLED: %s\n' "\$*" >&2; exit 1
GHEOF
    chmod +x "${MOCK_BIN}/gh"

    run env PATH="${MOCK_BIN}:${PATH}" bash "${BIN_DIR}/check-stale-verdict.sh" \
        "${pr_number}" "${sha}"

    [ "${status}" -eq 0 ] || {
        echo "FAIL: expected exit 0 on matching SHA; got ${status}"
        echo "Output: ${output}"
        return 1
    }
}

# ─────────────────────────────────────────────────────────────────────────────
# AC-003: merge-strategy enforcement (BC-5.42.001 PC-3, Invariant 3, Invariant 4)
# ─────────────────────────────────────────────────────────────────────────────

# T-005: enforce-merge-strategy.sh exits 1 with RELEASE_PR_SQUASH_FORBIDDEN
# when --squash is requested on a release/v* branch (no gh API call made).
#
# Red Gate: enforce-merge-strategy.sh stub exits 99 with UNIMPLEMENTED.
# After implementation: exits 1 before any gh pr merge invocation.
@test "T-005: enforce-merge-strategy.sh: release/v* + --squash → exit 1 + RELEASE_PR_SQUASH_FORBIDDEN" {
    local pr_number="10"
    local release_branch="release/v1.0.0-rc.23"

    # Mock gh: returns release branch name; gh pr merge must NOT be called
    local gh_merge_called_file
    gh_merge_called_file="$(mktemp "${BATS_TMPDIR}/gh-merge-called-XXXXXX")"
    rm -f "${gh_merge_called_file}"

    cat > "${MOCK_BIN}/gh" <<GHEOF
#!/usr/bin/env bash
if [[ "\$1" == "pr" && "\$2" == "view" && "\$*" == *"headRefName"* ]]; then
    printf '{"headRefName":"${release_branch}"}\n'
    exit 0
fi
if [[ "\$1" == "pr" && "\$2" == "merge" ]]; then
    touch "${gh_merge_called_file}"
    exit 0
fi
printf 'MOCK_GH_UNHANDLED: %s\n' "\$*" >&2; exit 1
GHEOF
    chmod +x "${MOCK_BIN}/gh"

    run env PATH="${MOCK_BIN}:${PATH}" bash "${BIN_DIR}/enforce-merge-strategy.sh" \
        "${pr_number}" --squash 2>&1

    [ "${status}" -eq 1 ] || {
        echo "FAIL: expected exit 1 for --squash on release branch; got ${status}"
        echo "Output: ${output}"
        return 1
    }
    echo "${output}" | grep -q "RELEASE_PR_SQUASH_FORBIDDEN" || {
        echo "FAIL: RELEASE_PR_SQUASH_FORBIDDEN message absent"
        echo "Output: ${output}"
        return 1
    }
    echo "${output}" | grep -q "${release_branch}" || {
        echo "FAIL: branch name not in RELEASE_PR_SQUASH_FORBIDDEN message"
        echo "Output: ${output}"
        return 1
    }
    # Verify gh pr merge was NOT called (BC-5.42.001 Invariant 3: no gh call before exit)
    [[ ! -f "${gh_merge_called_file}" ]] || {
        echo "FAIL: gh pr merge was called despite RELEASE_PR_SQUASH_FORBIDDEN (no API call permitted)"
        return 1
    }

    rm -f "${gh_merge_called_file}"
}

# T-006: enforce-merge-strategy.sh exits 0 when --merge is used on a release/v* branch.
# Positive control: --merge is the required strategy; no blocking.
#
# Red Gate: enforce-merge-strategy.sh stub exits 99 with UNIMPLEMENTED.
# After implementation: exits 0 (merge proceeds).
@test "T-006: enforce-merge-strategy.sh: release/v* + --merge → exit 0 (allowed)" {
    local pr_number="10"
    local release_branch="release/v1.0.0-rc.23"

    # Mock gh: returns release branch; gh pr merge exits 0
    cat > "${MOCK_BIN}/gh" <<GHEOF
#!/usr/bin/env bash
if [[ "\$1" == "pr" && "\$2" == "view" && "\$*" == *"headRefName"* ]]; then
    printf '{"headRefName":"${release_branch}"}\n'
    exit 0
fi
if [[ "\$1" == "pr" && "\$2" == "merge" ]]; then exit 0; fi
printf 'MOCK_GH_UNHANDLED: %s\n' "\$*" >&2; exit 1
GHEOF
    chmod +x "${MOCK_BIN}/gh"

    run env PATH="${MOCK_BIN}:${PATH}" bash "${BIN_DIR}/enforce-merge-strategy.sh" \
        "${pr_number}" --merge

    [ "${status}" -eq 0 ] || {
        echo "FAIL: expected exit 0 for --merge on release branch; got ${status}"
        echo "Output: ${output}"
        return 1
    }
}

# T-007: enforce-merge-strategy.sh delegates --squash unchanged on non-release branches.
# Non-release branches pass the caller-supplied flag through (BC-5.42.001 Invariant 4).
#
# Red Gate: enforce-merge-strategy.sh stub exits 99 with UNIMPLEMENTED.
# After implementation: exits 0 (delegates to gh pr merge --squash).
@test "T-007: enforce-merge-strategy.sh: non-release + --squash → exit 0 (delegated)" {
    local pr_number="10"
    local feature_branch="feature/S-19.01"

    # Mock gh: returns non-release branch; --squash passes through unchanged
    cat > "${MOCK_BIN}/gh" <<GHEOF
#!/usr/bin/env bash
if [[ "\$1" == "pr" && "\$2" == "view" && "\$*" == *"headRefName"* ]]; then
    printf '{"headRefName":"${feature_branch}"}\n'
    exit 0
fi
if [[ "\$1" == "pr" && "\$2" == "merge" ]]; then exit 0; fi
printf 'MOCK_GH_UNHANDLED: %s\n' "\$*" >&2; exit 1
GHEOF
    chmod +x "${MOCK_BIN}/gh"

    run env PATH="${MOCK_BIN}:${PATH}" bash "${BIN_DIR}/enforce-merge-strategy.sh" \
        "${pr_number}" --squash

    [ "${status}" -eq 0 ] || {
        echo "FAIL: expected exit 0 for non-release branch with --squash; got ${status}"
        echo "Output: ${output}"
        return 1
    }
}

# ─────────────────────────────────────────────────────────────────────────────
# AC-004: darwin-leg shell-dialect simulation discipline (S-19.01 D-g note)
# ─────────────────────────────────────────────────────────────────────────────

# T-008: darwin-leg preflight exits 1 + DARWIN_LEG_WRONG_INTERPRETER when
# /bin/bash does not report "version 3.2" (e.g., Homebrew bash 5.x).
#
# Gate (AC-004): preflight assertion in setup_file exits 1 on wrong interpreter.
# Literal CI-config gate: grep -qE '^  bats-darwin-leg-macos:$' .github/workflows/ci.yml
#   exits 0 after the job is added (O-P15-04; anchored exact-YAML-key form).
#
# Red Gate: darwin-leg-preflight.sh stub exits 99 with UNIMPLEMENTED;
#   CI-config gate will be added in implementation step.
# After implementation: exits 1 + DARWIN_LEG_WRONG_INTERPRETER when wrong interpreter.
@test "T-008: darwin-leg preflight: wrong interpreter exits 1 + DARWIN_LEG_WRONG_INTERPRETER" {
    # PREFLIGHT_BASH_BIN seam (commit 9e0a5453): production script uses
    #   PREFLIGHT_BASH_BIN="${PREFLIGHT_BASH_BIN:-/bin/bash}"
    # Point it at stub-bash-version.sh which reports bash 5.1.x by default (wrong).
    # No PATH manipulation needed; no recursion risk — absolute-path seam injection.
    #
    # POLICY 11: drives darwin-leg-preflight.sh (production script) via the seam;
    # seam was explicitly added for testability (not a test-only workaround).
    run env PREFLIGHT_BASH_BIN="${FIXTURES_DIR}/stub-bash-version.sh" \
        bash "${FIXTURES_DIR}/darwin-leg-preflight.sh" 2>&1

    [ "${status}" -ne 0 ] || {
        echo "FAIL: darwin-leg preflight should exit non-zero when stub reports bash 5.1"
        echo "Output: ${output}"
        return 1
    }
    echo "${output}" | grep -q "DARWIN_LEG_WRONG_INTERPRETER" || {
        echo "FAIL: DARWIN_LEG_WRONG_INTERPRETER not found in output"
        echo "Output: ${output}"
        return 1
    }

    # Literal CI-config gate (O-P15-04, AC-004 drift-proof anchor):
    # bats-darwin-leg-macos job must be present in ci.yml.
    grep -qE '^  bats-darwin-leg-macos:$' "${REPO_ROOT}/.github/workflows/ci.yml" || {
        echo "FAIL (AC-004 CI-config gate): bats-darwin-leg-macos job not found in ci.yml"
        echo "Expected: job key '  bats-darwin-leg-macos:' (two-space indented, exact)"
        return 1
    }
}

# T-009: darwin-leg preflight: on Linux runners, darwin-leg tests skip gracefully.
# The bats-darwin-leg-macos CI job does not run on Linux; the preflight exits 0
# on non-Darwin platforms (EC-003).
#
# Red Gate: darwin-leg-preflight.sh stub exits 99 with UNIMPLEMENTED;
#   on macOS this test is skipped (darwin-leg is a macOS-only concern).
# After implementation: exits 0 (graceful skip) on non-Darwin platforms.
@test "T-009: darwin-leg preflight: Linux runners skip gracefully (exit 0)" {
    if [[ "$(uname)" == "Darwin" ]]; then
        skip "T-009 is a Linux-runner test; darwin-leg runs on macOS, not Linux (EC-003)"
    fi

    # On Linux: darwin-leg preflight should exit 0 (graceful platform skip)
    run bash "${FIXTURES_DIR}/darwin-leg-preflight.sh" 2>&1

    [ "${status}" -eq 0 ] || {
        echo "FAIL: darwin-leg preflight should exit 0 on Linux (graceful skip); got ${status}"
        echo "Output: ${output}"
        return 1
    }
}

# ─────────────────────────────────────────────────────────────────────────────
# Missing BC Canonical Test Vectors + exact-text sentinel assertions (gap fill)
# Sources: BC-5.42.001 §Canonical Test Vectors TV3/TV4/TV5; §Description b/c;
#          EC-002/EC-005; VP-094 §Postcondition B/C; ADR-030 §Decision 2/3.
# All tests below FAIL at Red Gate (stubs exit 99 UNIMPLEMENTED / todo!() panics).
# ─────────────────────────────────────────────────────────────────────────────

# T-010: check-stale-verdict.sh: malformed covered_sha (< 40 chars, non-hex)
# → exit 1 + READY_SHA_MISSING: covered_sha is malformed
# (BC-5.42.001 §Canonical Test Vectors TV4: covered_sha=ZZZZZZ invalid hex, <40 chars)
#
# Red Gate: stub exits 99 UNIMPLEMENTED.
# After implementation: exits 1 with READY_SHA_MISSING: covered_sha is malformed on stderr.
@test "T-010: check-stale-verdict.sh: malformed covered_sha → exit 1 + READY_SHA_MISSING" {
    local pr_number="42"
    # BC-5.42.001 §Canonical Test Vectors TV4: non-hex, too short — same as ZZZZZZ in the spec
    local malformed_sha="ZZZZZZ"

    # gh must NOT be called — malformed SHA is rejected before any network I/O.
    cat > "${MOCK_BIN}/gh" <<'GHEOF'
#!/usr/bin/env bash
printf 'MOCK_GH_SHOULD_NOT_BE_CALLED_FOR_MALFORMED_SHA\n' >&2
exit 1
GHEOF
    chmod +x "${MOCK_BIN}/gh"

    run env PATH="${MOCK_BIN}:${PATH}" bash "${BIN_DIR}/check-stale-verdict.sh" \
        "${pr_number}" "${malformed_sha}" 2>&1

    [ "${status}" -eq 1 ] || {
        echo "FAIL: expected exit 1 on malformed covered_sha (TV4); got ${status}"
        echo "Output: ${output}"
        return 1
    }
    echo "${output}" | grep -q "READY_SHA_MISSING" || {
        echo "FAIL: READY_SHA_MISSING sentinel not found for malformed covered_sha (BC-5.42.001 TV4)"
        echo "Expected: READY_SHA_MISSING: covered_sha is malformed"
        echo "Output: ${output}"
        return 1
    }
}

# T-011: enforce-merge-strategy.sh: release/v* + --rebase → exit 1 + RELEASE_PR_SQUASH_FORBIDDEN
# (BC-5.42.001 §Canonical Test Vectors TV3: --rebase is forbidden on release/v* same as --squash)
# (BC-5.42.001 Invariant 3: release-branch squash/rebase is mechanically impossible via wrapper)
#
# Red Gate: stub exits 99 UNIMPLEMENTED.
# After implementation: exits 1 with RELEASE_PR_SQUASH_FORBIDDEN; no gh pr merge called.
@test "T-011: enforce-merge-strategy.sh: release/v* + --rebase → exit 1 + RELEASE_PR_SQUASH_FORBIDDEN" {
    local pr_number="10"
    local release_branch="release/v1.0.0-rc.23"
    local gh_merge_called_file
    gh_merge_called_file="$(mktemp "${BATS_TMPDIR}/gh-merge-T011-XXXXXX")"
    rm -f "${gh_merge_called_file}"

    cat > "${MOCK_BIN}/gh" <<GHEOF
#!/usr/bin/env bash
if [[ "\$1" == "pr" && "\$2" == "view" && "\$*" == *"headRefName"* ]]; then
    printf '{"headRefName":"${release_branch}"}\n'
    exit 0
fi
if [[ "\$1" == "pr" && "\$2" == "merge" ]]; then
    touch "${gh_merge_called_file}"
    exit 0
fi
printf 'MOCK_GH_UNHANDLED: %s\n' "\$*" >&2; exit 1
GHEOF
    chmod +x "${MOCK_BIN}/gh"

    run env PATH="${MOCK_BIN}:${PATH}" bash "${BIN_DIR}/enforce-merge-strategy.sh" \
        "${pr_number}" --rebase 2>&1

    [ "${status}" -eq 1 ] || {
        echo "FAIL: expected exit 1 for --rebase on release branch (TV3); got ${status}"
        echo "Output: ${output}"
        return 1
    }
    echo "${output}" | grep -q "RELEASE_PR_SQUASH_FORBIDDEN" || {
        echo "FAIL: RELEASE_PR_SQUASH_FORBIDDEN message absent for --rebase on release branch"
        echo "Output: ${output}"
        return 1
    }
    [[ ! -f "${gh_merge_called_file}" ]] || {
        echo "FAIL: gh pr merge was called despite RELEASE_PR_SQUASH_FORBIDDEN (BC-5.42.001 Invariant 3: no API call permitted)"
        return 1
    }
    rm -f "${gh_merge_called_file}"
}

# T-012: enforce-merge-strategy.sh: release/v* + no merge flag → defaults to --merge, exit 0
# (BC-5.42.001 EC-005: missing flag on release branch → script injects --merge)
# (BC-5.42.001 §Canonical Test Vectors TV5: no flag supplied → gh pr merge <n> --merge)
#
# Red Gate: stub exits 99 UNIMPLEMENTED.
# After implementation: exits 0; gh pr merge receives --merge flag even though caller supplied none.
@test "T-012: enforce-merge-strategy.sh: release/v* + no flag → defaults to --merge (EC-005)" {
    local pr_number="10"
    local release_branch="release/v1.0.0-rc.23"
    local merge_flag_file
    merge_flag_file="$(mktemp "${BATS_TMPDIR}/merge-flag-T012-XXXXXX.txt")"
    rm -f "${merge_flag_file}"

    cat > "${MOCK_BIN}/gh" <<GHEOF
#!/usr/bin/env bash
if [[ "\$1" == "pr" && "\$2" == "view" && "\$*" == *"headRefName"* ]]; then
    printf '{"headRefName":"${release_branch}"}\n'
    exit 0
fi
if [[ "\$1" == "pr" && "\$2" == "merge" ]]; then
    # Record all positional args passed to gh pr merge to verify --merge was injected
    printf '%s\n' "\$@" > "${merge_flag_file}"
    exit 0
fi
printf 'MOCK_GH_UNHANDLED: %s\n' "\$*" >&2; exit 1
GHEOF
    chmod +x "${MOCK_BIN}/gh"

    run env PATH="${MOCK_BIN}:${PATH}" bash "${BIN_DIR}/enforce-merge-strategy.sh" \
        "${pr_number}"

    [ "${status}" -eq 0 ] || {
        echo "FAIL: expected exit 0 for release branch with no explicit flag (EC-005); got ${status}"
        echo "Output: ${output}"
        return 1
    }
    # EC-005: --merge must be injected even when caller supplied no merge-strategy flag
    { [[ -f "${merge_flag_file}" ]] && grep -q -- "--merge" "${merge_flag_file}"; } || {
        echo "FAIL: --merge not injected for release branch with no flag (BC-5.42.001 EC-005 / TV5)"
        echo "gh pr merge args recorded: $(cat "${merge_flag_file}" 2>/dev/null || echo '(gh not called)')"
        return 1
    }
    rm -f "${merge_flag_file}"
}

# T-013: check-stale-verdict.sh: STALE_READY_VERDICT exact canonical message format
# BC-5.42.001 §Description b canonical form:
# "STALE_READY_VERDICT: PR #<n> HEAD <current_sha> != covered_sha <covered_sha>"
# (VP-094 §Postcondition B; ADR-030 §Decision 2 stderr routing)
#
# Red Gate: stub exits 99 UNIMPLEMENTED.
# After implementation: exact canonical diagnostic appears on stderr.
@test "T-013: check-stale-verdict.sh: STALE_READY_VERDICT exact canonical message format" {
    local pr_number="42"
    local covered_sha="aaaa000000000000000000000000000000000000"
    local live_sha="bbbb111111111111111111111111111111111111"

    cat > "${MOCK_BIN}/gh" <<GHEOF
#!/usr/bin/env bash
if [[ "\$1" == "pr" && "\$2" == "view" && "\$*" == *"headRefOid"* ]]; then
    printf '{"headRefOid":"${live_sha}"}\n'
    exit 0
fi
printf 'MOCK_GH_UNHANDLED: %s\n' "\$*" >&2; exit 1
GHEOF
    chmod +x "${MOCK_BIN}/gh"

    run env PATH="${MOCK_BIN}:${PATH}" bash "${BIN_DIR}/check-stale-verdict.sh" \
        "${pr_number}" "${covered_sha}" 2>&1

    [ "${status}" -ne 0 ] || {
        echo "FAIL: expected non-zero exit for stale verdict; got ${status}"
        return 1
    }
    # Exact canonical format (BC-5.42.001 §Description b + TV2 verbatim connector text):
    echo "${output}" | grep -q "STALE_READY_VERDICT: PR #${pr_number} HEAD ${live_sha} != covered_sha ${covered_sha}" || {
        echo "FAIL: exact canonical STALE_READY_VERDICT format not found in stderr"
        printf 'Expected substring: STALE_READY_VERDICT: PR #%s HEAD %s != covered_sha %s\n' \
            "${pr_number}" "${live_sha}" "${covered_sha}"
        echo "Output: ${output}"
        return 1
    }
}

# T-014: check-stale-verdict.sh: READY_SHA_FETCH_FAILED exact canonical message format
# BC-5.42.001 EC-001 verbatim wording:
# "READY_SHA_FETCH_FAILED: gh pr view failed for PR #<pr_number>"
# (S-19.01 AC-001 Gate; v1.9 F-P11-001 corrected literal wording)
#
# Red Gate: stub exits 99 UNIMPLEMENTED.
# After implementation: exact BC-5.42.001 EC-001 wording on stderr.
@test "T-014: check-stale-verdict.sh: READY_SHA_FETCH_FAILED exact canonical message format (EC-001)" {
    local pr_number="42"
    local covered_sha="aaaa000000000000000000000000000000000000"

    # Mock gh: exits non-zero to simulate network/auth failure (EC-001 trigger)
    cat > "${MOCK_BIN}/gh" <<'GHEOF'
#!/usr/bin/env bash
printf 'Error: HTTP 503: server temporarily unavailable\n' >&2
exit 1
GHEOF
    chmod +x "${MOCK_BIN}/gh"

    run env PATH="${MOCK_BIN}:${PATH}" bash "${BIN_DIR}/check-stale-verdict.sh" \
        "${pr_number}" "${covered_sha}" 2>&1

    [ "${status}" -ne 0 ] || {
        echo "FAIL: expected non-zero exit on gh failure (EC-001); got ${status}"
        return 1
    }
    # Exact BC-5.42.001 EC-001 wording (§Canonical Test Vectors TV3 verbatim):
    echo "${output}" | grep -q "READY_SHA_FETCH_FAILED: gh pr view failed for PR #${pr_number}" || {
        echo "FAIL: exact canonical READY_SHA_FETCH_FAILED format not found in stderr"
        printf 'Expected substring: READY_SHA_FETCH_FAILED: gh pr view failed for PR #%s\n' \
            "${pr_number}"
        echo "Output: ${output}"
        return 1
    }
}

# T-015: enforce-merge-strategy.sh: RELEASE_PR_SQUASH_FORBIDDEN exact canonical message format
# BC-5.42.001 §Description c canonical form:
# "RELEASE_PR_SQUASH_FORBIDDEN: branch <name> requires --merge per RELEASING.md"
# (VP-094 §Postcondition C; ADR-030 §Decision 3 verbatim; "per RELEASING.md" anchor required)
#
# Red Gate: stub exits 99 UNIMPLEMENTED.
# After implementation: exact canonical diagnostic on stderr including "per RELEASING.md" anchor.
@test "T-015: enforce-merge-strategy.sh: RELEASE_PR_SQUASH_FORBIDDEN exact canonical message format" {
    local pr_number="10"
    local release_branch="release/v1.0.0-rc.23"

    cat > "${MOCK_BIN}/gh" <<GHEOF
#!/usr/bin/env bash
if [[ "\$1" == "pr" && "\$2" == "view" && "\$*" == *"headRefName"* ]]; then
    printf '{"headRefName":"${release_branch}"}\n'
    exit 0
fi
printf 'MOCK_GH_UNHANDLED: %s\n' "\$*" >&2; exit 1
GHEOF
    chmod +x "${MOCK_BIN}/gh"

    run env PATH="${MOCK_BIN}:${PATH}" bash "${BIN_DIR}/enforce-merge-strategy.sh" \
        "${pr_number}" --squash 2>&1

    [ "${status}" -eq 1 ] || {
        echo "FAIL: expected exit 1 for --squash on release branch; got ${status}"
        echo "Output: ${output}"
        return 1
    }
    # Exact canonical format (BC-5.42.001 §Description c + VP-094 §Postcondition C verbatim);
    # "per RELEASING.md" is a mandatory anchor per ADR-030 §Decision 3.
    echo "${output}" | grep -q "RELEASE_PR_SQUASH_FORBIDDEN: branch ${release_branch} requires --merge per RELEASING.md" || {
        echo "FAIL: exact canonical RELEASE_PR_SQUASH_FORBIDDEN format not found in stderr"
        printf 'Expected substring: RELEASE_PR_SQUASH_FORBIDDEN: branch %s requires --merge per RELEASING.md\n' \
            "${release_branch}"
        echo "Output: ${output}"
        return 1
    }
}

# T-016: check-stale-verdict.sh: 40-char uppercase-hex covered_sha → READY_SHA_MISSING (EC-002)
# BC-5.42.001 Invariant 5: exactly 40 LOWERCASE hex characters required.
# EC-002: 40-char value with non-lowercase hex chars is treated the same as absent.
# (pr-manager-completion-guard READY_SHA_MISSING same code; check-stale-verdict.sh rejects too)
#
# Red Gate: stub exits 99 UNIMPLEMENTED.
# After implementation: exits 1 with READY_SHA_MISSING on stderr (Invariant 5 / EC-002).
@test "T-016: check-stale-verdict.sh: 40-char uppercase-hex covered_sha → READY_SHA_MISSING (EC-002)" {
    local pr_number="42"
    # BC-5.42.001 Invariant 5 + EC-002: 40-char string but uppercase hex = malformed
    local uppercase_sha="AAAA000000000000000000000000000000000000"

    # gh must NOT be called — malformed SHA rejected before network I/O.
    cat > "${MOCK_BIN}/gh" <<'GHEOF'
#!/usr/bin/env bash
printf 'MOCK_GH_SHOULD_NOT_BE_CALLED_FOR_UPPERCASE_SHA\n' >&2
exit 1
GHEOF
    chmod +x "${MOCK_BIN}/gh"

    run env PATH="${MOCK_BIN}:${PATH}" bash "${BIN_DIR}/check-stale-verdict.sh" \
        "${pr_number}" "${uppercase_sha}" 2>&1

    [ "${status}" -eq 1 ] || {
        echo "FAIL: expected exit 1 for uppercase (malformed) covered_sha (EC-002 / Invariant 5); got ${status}"
        echo "Output: ${output}"
        return 1
    }
    echo "${output}" | grep -q "READY_SHA_MISSING" || {
        echo "FAIL: READY_SHA_MISSING not found for uppercase covered_sha (BC-5.42.001 EC-002 / Invariant 5)"
        echo "Output: ${output}"
        return 1
    }
}

# ─────────────────────────────────────────────────────────────────────────────
# F-S1901-P1-003 (HIGH) — AC-004 mandatory mechanism test (pass-1 adversary gap)
# T-017: darwin-leg fragment execution — while IFS= read -r bash-3.2 compat regression pin
# ─────────────────────────────────────────────────────────────────────────────

# T-017: Regression pin for release.yml rc.22 failure site (commit c10dc6ca).
# The "Verify registry-declared WASM plugins are staged" step previously used
# mapfile (bash 4.0+ only), failing on macOS /bin/bash 3.2. This test extracts
# the while IFS= read -r loop fragment from the fixed release.yml, runs it
# under /bin/bash with a fixture registry, and asserts exit 0 + staging verdict.
#
# GREEN on Darwin (fix is already in release.yml at c10dc6ca) — acceptable per
# F-S1901-P1-003: this is a regression-pinning mechanism test.
# Negative control: mapfile variant FAILS under /bin/bash 3.2 → confirms the
# test is load-bearing (reverts to mapfile = suite turns red again).
#
# Skipped on Linux: /bin/bash there is bash 5.x so mapfile would pass, making
# the negative control meaningless; the darwin-leg CI job runs on macOS only.
@test "T-017: darwin-leg fragment: while IFS= read -r bash-3.2 compat (regression pin rc.22)" {
    if [[ "$(uname)" != "Darwin" ]]; then
        skip "T-017 is macOS-only: negative control (mapfile) requires /bin/bash 3.2.x"
    fi

    # Build fixture work directory with:
    #   plugins/vsdd-factory/hooks-registry.toml  (copied from repo — 73 entries, ≥ 30 guard)
    #   artifact/<name>.wasm                       (stub empty files for each declared plugin)
    local work_dir
    work_dir="$(mktemp -d "${BATS_TMPDIR}/T017-work-XXXXXX")"
    mkdir -p "${work_dir}/plugins/vsdd-factory" "${work_dir}/artifact"
    cp "${REPO_ROOT}/plugins/vsdd-factory/hooks-registry.toml" \
        "${work_dir}/plugins/vsdd-factory/hooks-registry.toml"

    # Create stub WASMs for every declared plugin (mirrors the real artifact/ from cargo build).
    while IFS= read -r name; do
        [ -n "$name" ] && touch "${work_dir}/artifact/${name}"
    done < <(
        grep -E '^\s*plugin\s*=\s*"hook-plugins/' \
            "${work_dir}/plugins/vsdd-factory/hooks-registry.toml" \
            | grep -oE 'hook-plugins/[^"]+' \
            | sed 's|hook-plugins/||' \
            | sort -u
    )

    # ── POSITIVE TEST ────────────────────────────────────────────────────────
    # Extract the "Verify registry-declared WASM plugins are staged" step shell
    # body from .github/workflows/release.yml at test time (awk strips the
    # 10-space YAML indent). If mapfile is ever reintroduced into release.yml
    # this extraction picks it up → fragment fails under /bin/bash 3.2 → suite RED.
    local pos_script="${work_dir}/fragment-positive.sh"
    {
        printf '#!/bin/bash\n'
        printf 'set -euo pipefail\n'
        printf 'cd %q\n' "${work_dir}"
        awk '
            /- name: Verify registry-declared WASM plugins are staged/ && !found { found=1 }
            found && /^        run: [|]/ { in_run=1; next }
            in_run && /^      - / { exit }
            in_run { sub(/^          /, ""); print }
        ' "${REPO_ROOT}/.github/workflows/release.yml"
    } > "${pos_script}"
    chmod +x "${pos_script}"

    # Pre-flight: confirm extraction captured the while-IFS-read-r fix, not mapfile.
    grep -q "while IFS= read -r" "${pos_script}" || {
        echo "FAIL: extracted fragment missing 'while IFS= read -r' — release.yml may have lost the rc.22 fix"
        cat "${pos_script}"
        return 1
    }
    if grep -qE '^\s*mapfile\b' "${pos_script}"; then
        echo "FAIL: extracted fragment invokes mapfile — rc.22 regression reintroduced in release.yml"
        cat "${pos_script}"
        return 1
    fi

    run /bin/bash "${pos_script}" 2>&1
    [ "${status}" -eq 0 ] || {
        echo "FAIL: while-IFS-read-r fragment should exit 0 under /bin/bash 3.2 (c10dc6ca fix)"
        echo "Output: ${output}"
        return 1
    }
    echo "${output}" | grep -q "Registry-vs-staged: all" || {
        echo "FAIL: expected 'Registry-vs-staged: all' staging verdict in output"
        echo "Output: ${output}"
        return 1
    }

    # ── NEGATIVE CONTROL ─────────────────────────────────────────────────────
    # mapfile is bash 4.0+; must fail under /bin/bash 3.2. If this passes,
    # the test is not load-bearing (we are not running under bash 3.2).
    local neg_script="${work_dir}/fragment-mapfile.sh"
    {
        printf '#!/bin/bash\n'
        printf 'set -euo pipefail\n'
        printf 'cd %q\n' "${work_dir}"
        cat << 'MAPFILE_FRAGMENT'
# mapfile requires bash 4.0+; this MUST fail under /bin/bash 3.2 on macOS.
mapfile -t declared < <(
  grep -E '^\s*plugin\s*=\s*"hook-plugins/' \
    plugins/vsdd-factory/hooks-registry.toml \
    | grep -oE 'hook-plugins/[^"]+' \
    | sed 's|hook-plugins/||' \
    | sort -u
)
echo "mapfile-form: ${#declared[@]} entries (should not reach here under bash 3.2)"
MAPFILE_FRAGMENT
    } > "${neg_script}"
    chmod +x "${neg_script}"

    run /bin/bash "${neg_script}" 2>&1
    [ "${status}" -ne 0 ] || {
        echo "FAIL (negative control): mapfile form should fail under /bin/bash 3.2"
        echo "If this passes, /bin/bash is not 3.2 — check interpreter version:"
        /bin/bash --version 2>&1 | head -1
        echo "Output: ${output}"
        return 1
    }
    # Documented: negative control confirms mapfile not portable to bash 3.2;
    # reverting c10dc6ca's while-IFS-read-r fix would cause this test to fail.
}

# ─────────────────────────────────────────────────────────────────────────────
# F-S1901-P1-004 (MEDIUM, closed) — ADR-030 §Decision 2 arms 3+4 now covered
# T-018: EC-003 (closed/merged PR) — GREEN: arm 3 verified post-implementation
# T-019: arm 4 (malformed/unparseable gh JSON) — GREEN: arm 4 verified post-implementation
# ─────────────────────────────────────────────────────────────────────────────

# T-018: check-stale-verdict.sh: closed PR with matching headRefOid → exit 1 + CHECK_STALE_VERDICT_ERROR
# ADR-030 §Decision 2 arm 3: PR closed/merged between READY verdict and check invocation.
# BC-5.42.001 EC-003: gh returns non-open state; script exits non-zero with diagnostic.
# Sentinel (ADR-030 §Decision 2 arm 3):
#   CHECK_STALE_VERDICT_ERROR: PR #<n> is <state> (expected: open)
#
# Verification: implementation fetches state alongside headRefOid; even with matching
# SHAs, non-open state triggers arm 3 — exits non-zero with:
# CHECK_STALE_VERDICT_ERROR: PR #<n> is <state> (expected: open).
@test "T-018: check-stale-verdict.sh: closed PR (matching SHA) → exit 1 + CHECK_STALE_VERDICT_ERROR (EC-003)" {
    local pr_number="42"
    local sha="dddd333333333333333333333333333333333333"

    # Mock gh: returns state=closed + headRefOid MATCHING covered_sha.
    # The SHAs match; the implementation correctly checks state regardless and emits
    # CHECK_STALE_VERDICT_ERROR when state != "open" (ADR-030 §Decision 2 arm 3).
    cat > "${MOCK_BIN}/gh" <<GHEOF
#!/usr/bin/env bash
if [[ "\$1" == "pr" && "\$2" == "view" ]]; then
    printf '{"headRefOid":"${sha}","state":"closed"}\n'
    exit 0
fi
printf 'MOCK_GH_UNHANDLED: %s\n' "\$*" >&2; exit 1
GHEOF
    chmod +x "${MOCK_BIN}/gh"

    run env PATH="${MOCK_BIN}:${PATH}" bash "${BIN_DIR}/check-stale-verdict.sh" \
        "${pr_number}" "${sha}" 2>&1

    [ "${status}" -ne 0 ] || {
        echo "FAIL: expected non-zero exit when PR is closed (EC-003); got exit 0"
        echo "arm 3 is implemented: expected non-zero exit; script must fetch state and emit"
        echo "ADR-030 §Decision 2 arm 3: CHECK_STALE_VERDICT_ERROR: PR #<n> is closed (expected: open)"
        echo "Output: ${output}"
        return 1
    }
    echo "${output}" | grep -q "CHECK_STALE_VERDICT_ERROR" || {
        echo "FAIL: CHECK_STALE_VERDICT_ERROR not found in output (ADR-030 §Decision 2 arm 3)"
        echo "Output: ${output}"
        return 1
    }
    echo "${output}" | grep -q "PR #${pr_number}" || {
        echo "FAIL: PR number not in CHECK_STALE_VERDICT_ERROR message"
        echo "Output: ${output}"
        return 1
    }
    echo "${output}" | grep -q "closed" || {
        echo "FAIL: PR state 'closed' not in CHECK_STALE_VERDICT_ERROR message"
        echo "Expected substring: CHECK_STALE_VERDICT_ERROR: PR #${pr_number} is closed (expected: open)"
        echo "Output: ${output}"
        return 1
    }
    echo "${output}" | grep -q "expected: open" || {
        echo "FAIL: '(expected: open)' not in CHECK_STALE_VERDICT_ERROR message (ADR-030 §Decision 2 arm 3 form)"
        echo "Expected: CHECK_STALE_VERDICT_ERROR: PR #${pr_number} is closed (expected: open)"
        echo "Output: ${output}"
        return 1
    }
}

# T-019: check-stale-verdict.sh: malformed/unparseable gh JSON → exit 1 + CHECK_STALE_VERDICT_ERROR
# ADR-030 §Decision 2 arm 4: gh exits 0 but returns non-JSON that cannot be parsed.
# BC-5.42.001 EC-004 catch-all: CHECK_STALE_VERDICT_ERROR: <description> on stderr.
#
# Verification: implementation distinguishes gh-failure (arm 1: READY_SHA_FETCH_FAILED)
# from parse-failure (arm 4: CHECK_STALE_VERDICT_ERROR). gh exits 0 but with
# non-JSON output triggers the arm 4 catch-all: CHECK_STALE_VERDICT_ERROR: <description>.
@test "T-019: check-stale-verdict.sh: malformed gh JSON → exit 1 + CHECK_STALE_VERDICT_ERROR (arm 4)" {
    local pr_number="99"
    local covered_sha="eeee444444444444444444444444444444444444"

    # Mock gh: exits 0 (no network failure) but returns non-JSON garbage.
    # Implementation: gh succeeds but JSON parse fails → emits CHECK_STALE_VERDICT_ERROR: <description>
    # (arm 4 catch-all), distinguishing parse failure from gh failure (arm 1: READY_SHA_FETCH_FAILED).
    cat > "${MOCK_BIN}/gh" <<'GHEOF'
#!/usr/bin/env bash
if [[ "$1" == "pr" && "$2" == "view" ]]; then
    printf 'not-json-garbage\n'
    exit 0
fi
printf 'MOCK_GH_UNHANDLED: %s\n' "$*" >&2; exit 1
GHEOF
    chmod +x "${MOCK_BIN}/gh"

    run env PATH="${MOCK_BIN}:${PATH}" bash "${BIN_DIR}/check-stale-verdict.sh" \
        "${pr_number}" "${covered_sha}" 2>&1

    [ "${status}" -ne 0 ] || {
        echo "FAIL: expected non-zero exit on malformed gh JSON; got exit 0"
        echo "Output: ${output}"
        return 1
    }
    echo "${output}" | grep -q "CHECK_STALE_VERDICT_ERROR" || {
        echo "FAIL: CHECK_STALE_VERDICT_ERROR not found in output (ADR-030 §Decision 2 arm 4)"
        echo "arm 4 is implemented: expected CHECK_STALE_VERDICT_ERROR (parse failure)"
        echo "  distinct from arm 1 (READY_SHA_FETCH_FAILED = gh failure)"
        echo "Output: ${output}"
        return 1
    }
}
