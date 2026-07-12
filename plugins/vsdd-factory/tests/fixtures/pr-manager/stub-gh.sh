#!/usr/bin/env bash
# stub-gh.sh — mock gh CLI for S-19.01 bats fixture tests.
#
# Configurable via environment variables:
#   STUB_GH_HEAD_REF_OID          — headRefOid value (default: aaaa000...000)
#   STUB_GH_HEAD_REF_NAME         — headRefName value (default: feature/stub)
#   STUB_GH_STATE                 — PR state field (default: open; set to "closed"/"merged" for EC-003)
#   STUB_GH_FAIL_VIEW             — if "1", exits non-zero on `gh pr view` calls (simulates gh failure)
#   STUB_GH_MALFORMED_JSON        — if "1", returns non-JSON garbage from `gh pr view` (simulates arm 4)
#   STUB_GH_MERGE_EXIT            — exit code for `gh pr merge` calls (default: 0)
#   STUB_GH_NULL_OID              — if "1", returns null for headRefOid value (simulates F-P5-001)
#   STUB_GH_NULL_HEAD_REF_NAME    — if "1", returns null for headRefName value (simulates F-P5-001 sibling)
#   STUB_GH_ARGV_LOG              — if set to a file path, gh pr merge appends all argv (one per
#                                   line) to that file so tests can assert which flags were forwarded
#                                   through enforce-merge-strategy.sh (F-P7-001 arg-forwarding gate).
#
# Notes:
#   - When headRefOid is requested, response always includes both headRefOid AND state so
#     that tests covering ADR-030 §Decision 2 arm 3 (EC-003) work correctly.
#   - STUB_GH_MALFORMED_JSON takes precedence over STUB_GH_FAIL_VIEW (returns garbage, not failure).

set -euo pipefail

STUB_GH_FAIL_VIEW="${STUB_GH_FAIL_VIEW:-0}"
STUB_GH_MALFORMED_JSON="${STUB_GH_MALFORMED_JSON:-0}"
STUB_GH_MERGE_EXIT="${STUB_GH_MERGE_EXIT:-0}"
STUB_GH_STATE="${STUB_GH_STATE:-open}"
STUB_GH_NULL_OID="${STUB_GH_NULL_OID:-0}"
STUB_GH_NULL_HEAD_REF_NAME="${STUB_GH_NULL_HEAD_REF_NAME:-0}"
STUB_GH_ARGV_LOG="${STUB_GH_ARGV_LOG:-}"

case "${1:-}" in
    pr)
        case "${2:-}" in
            view)
                if [[ "${STUB_GH_MALFORMED_JSON}" == "1" ]]; then
                    printf 'not-json-garbage\n'
                    exit 0
                fi
                if [[ "${STUB_GH_FAIL_VIEW}" == "1" ]]; then
                    printf 'stub-gh: gh pr view simulated failure\n' >&2
                    exit 1
                fi
                # Include state in every headRefOid response (ADR-030 §Decision 2 arm 3 support).
                if [[ "${*}" == *"headRefOid"* ]]; then
                    if [[ "${STUB_GH_NULL_OID}" == "1" ]]; then
                        printf '{"headRefOid":null,"state":"%s"}\n' "${STUB_GH_STATE}"
                        exit 0
                    fi
                    printf '{"headRefOid":"%s","state":"%s"}\n' \
                        "${STUB_GH_HEAD_REF_OID:-aaaa000000000000000000000000000000000000}" \
                        "${STUB_GH_STATE}"
                    exit 0
                fi
                if [[ "${*}" == *"headRefName"* ]]; then
                    if [[ "${STUB_GH_NULL_HEAD_REF_NAME}" == "1" ]]; then
                        printf '{"headRefName":null}\n'
                        exit 0
                    fi
                    printf '{"headRefName":"%s"}\n' "${STUB_GH_HEAD_REF_NAME:-feature/stub}"
                    exit 0
                fi
                printf '{"headRefOid":"%s","state":"%s","headRefName":"%s"}\n' \
                    "${STUB_GH_HEAD_REF_OID:-aaaa000000000000000000000000000000000000}" \
                    "${STUB_GH_STATE}" \
                    "${STUB_GH_HEAD_REF_NAME:-feature/stub}"
                ;;
            merge)
                # Record forwarded argv for arg-forwarding gate tests (F-P7-001).
                if [[ -n "${STUB_GH_ARGV_LOG}" ]]; then
                    printf '%s\n' "$@" >> "${STUB_GH_ARGV_LOG}"
                fi
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
