#!/usr/bin/env bash
# helpers/dispatcher-provenance.bash — emit factory-dispatcher binary provenance.
#
# Loaded by setup_file() in suites that invoke factory-dispatcher, so every run
# produces an auditable record: which binary was exercised, its sha256, and mtime.
#
# D-693 / F-S2107-P6-017: a "N/N GREEN" attestation is only meaningful when the
# specific binary exercised is recorded (path + content hash + mtime).
#
# Usage (from setup_file):
#   load "${BATS_TEST_DIRNAME}/helpers/dispatcher-provenance.bash"
#   emit_dispatcher_provenance               # auto-resolve: debug → release
#   emit_dispatcher_provenance "${_my_path}" # explicit path (caller selects)
#
# Output goes to >&3 (TAP comment lines — always visible in bats 1.x output).
# Under CI_REQUIRE_ARTIFACTS=1, returns non-zero when the binary is not found.

emit_dispatcher_provenance() {
    local _disp="${1:-}"

    if [[ -z "${_disp}" ]]; then
        # Auto-resolve from the calling test file's repo root (debug preferred over release).
        local _repo_root
        _repo_root="$(cd "$(dirname "${BATS_TEST_FILENAME}")/../../.." && pwd)"
        _disp="${_repo_root}/target/debug/factory-dispatcher"
        [[ -x "${_disp}" ]] || _disp="${_repo_root}/target/release/factory-dispatcher"
    fi

    if [[ -x "${_disp}" ]]; then
        local _sha _mtime
        _sha="$(shasum -a 256 "${_disp}" | cut -d' ' -f1)"
        _mtime="$(date -r "${_disp}" +%Y-%m-%dT%H:%M:%S 2>/dev/null \
                  || stat --format='%y' "${_disp}" 2>/dev/null | sed 's/ /T/; s/\..*//')"
        echo "# dispatcher-provenance: path=${_disp}" >&3
        echo "# dispatcher-provenance: sha256=${_sha}  mtime=${_mtime}" >&3
    else
        echo "# dispatcher-provenance: UNRESOLVED — binary not found at ${_disp}" >&3
        if [[ "${CI_REQUIRE_ARTIFACTS:-}" == "1" ]]; then
            echo "FATAL: CI_REQUIRE_ARTIFACTS=1 — dispatcher must be pre-built at ${_disp}" >&2
            return 1
        fi
    fi
}
