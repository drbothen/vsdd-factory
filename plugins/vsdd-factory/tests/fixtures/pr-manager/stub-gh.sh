#!/usr/bin/env bash
# stub-gh.sh — mock gh CLI for S-19.01 bats fixture tests.
#
# Configurable via environment variables:
#   STUB_GH_HEAD_REF_OID   — headRefOid value to return on `gh pr view --json headRefOid`
#   STUB_GH_HEAD_REF_NAME  — headRefName value to return on `gh pr view --json headRefName`
#   STUB_GH_FAIL_VIEW      — if set to "1", exits non-zero on `gh pr view` calls
#   STUB_GH_MERGE_EXIT     — exit code for `gh pr merge` calls (default: 0)
#
# S-19.01: Red Gate stub — this fixture is a placeholder used in bats tests.
# The actual behavior is configured inline by each test using inline here-doc
# mock scripts; this file is the fallback stub for fixture-based tests.

set -euo pipefail

STUB_GH_FAIL_VIEW="${STUB_GH_FAIL_VIEW:-0}"
STUB_GH_MERGE_EXIT="${STUB_GH_MERGE_EXIT:-0}"

case "${1:-}" in
    pr)
        case "${2:-}" in
            view)
                if [[ "${STUB_GH_FAIL_VIEW}" == "1" ]]; then
                    printf 'stub-gh: gh pr view simulated failure\n' >&2
                    exit 1
                fi
                # Detect which JSON field is requested
                if [[ "${*}" == *"headRefOid"* ]]; then
                    printf '{"headRefOid":"%s"}\n' "${STUB_GH_HEAD_REF_OID:-aaaa000000000000000000000000000000000000}"
                    exit 0
                fi
                if [[ "${*}" == *"headRefName"* ]]; then
                    printf '{"headRefName":"%s"}\n' "${STUB_GH_HEAD_REF_NAME:-feature/stub}"
                    exit 0
                fi
                printf '{"headRefOid":"%s","headRefName":"%s"}\n' \
                    "${STUB_GH_HEAD_REF_OID:-aaaa000000000000000000000000000000000000}" \
                    "${STUB_GH_HEAD_REF_NAME:-feature/stub}"
                ;;
            merge)
                exit "${STUB_GH_MERGE_EXIT}"
                ;;
            *)
                printf 'stub-gh: unhandled gh pr subcommand: %s\n' "${2:-}" >&2
                exit 1
                ;;
        esac
        ;;
    *)
        printf 'stub-gh: unhandled gh command: %s\n' "${1:-}" >&2
        exit 1
        ;;
esac
