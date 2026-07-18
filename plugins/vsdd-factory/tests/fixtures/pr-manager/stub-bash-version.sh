#!/bin/sh
# stub-bash-version.sh — mock /bin/bash for S-19.01 darwin-leg preflight tests.
#
# Simulates a /bin/bash that reports a configurable version string.
# Used by T-008 to test the darwin-leg preflight behavior with a wrong
# interpreter (e.g., Bash 5.x instead of 3.2.x).
#
# Usage:
#   STUB_BASH_VERSION="5.1.16(1)-release (x86_64-apple-darwin21)" \
#     bash stub-bash-version.sh --version
#
# S-19.01: Red Gate stub — placeholder fixture.

STUB_BASH_VERSION="${STUB_BASH_VERSION:-5.1.16(1)-release (x86_64-apple-darwin21)}"

case "${1:-}" in
    --version)
        printf 'GNU bash, version %s\n' "${STUB_BASH_VERSION}"
        printf 'Copyright (C) 2020 Free Software Foundation, Inc.\n'
        exit 0
        ;;
    *)
        printf 'stub-bash-version: unhandled flag: %s\n' "${1:-}" >&2
        exit 1
        ;;
esac
