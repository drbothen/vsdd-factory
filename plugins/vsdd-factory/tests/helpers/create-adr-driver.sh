#!/usr/bin/env bash
# create-adr-driver.sh — RED phase stub for S-6.01 create-adr skill tests.
#
# This stub exists solely so the bats tests have something to invoke during the
# RED (failing) phase of TDD.  Every invocation exits 1 to ensure ALL tests fail
# before any implementation is written.
#
# When the implementer completes GREEN phase they replace this script with real
# logic (or wire it to SKILL.md via a claude invocation) while keeping the same
# CLI surface:
#
#   create-adr-driver.sh [options]
#
#   Options:
#     --title      "Decision Title"       (required)
#     --subsystems "SS-06,SS-08"         (comma-separated SS-NN ids)
#     --supersedes "ADR-NNN"             (optional, existing ADR id)
#     --brownfield                        (flag; no argument)
#     --id         "ADR-NNN"             (optional explicit id override)
#
#   Environment variables consumed by tests (and later by real driver):
#     DECISIONS_DIR   — absolute path to the decisions/ directory (fixture or real)
#     ARCH_INDEX      — absolute path to ARCH-INDEX.md
#     ADR_TEMPLATE    — absolute path to adr-template.md
#     VALIDATE_BIN    — absolute path to validate-template-compliance.sh
#                       (or a mock that honours MOCK_VALIDATE_EXIT)
#
# Exit codes:
#   0  — success
#   1  — any error / not-yet-implemented

echo "create-adr: NOT YET IMPLEMENTED" >&2
exit 1
