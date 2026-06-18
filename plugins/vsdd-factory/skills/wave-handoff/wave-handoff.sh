#!/usr/bin/env bash
# wave-handoff.sh — Main entrypoint for the wave-handoff skill
# Writes HANDOFF.md + wave-state.yaml atomically to factory-artifacts at wave close.
# BC-5.41.001 + BC-5.41.002 | S-18.01 scaffold
#
# TODO S-18.01 — implemented in TDD green step
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# shellcheck source=lib/parse-sprint-state.sh
source "${SCRIPT_DIR}/lib/parse-sprint-state.sh"
# shellcheck source=lib/write-handoff.sh
source "${SCRIPT_DIR}/lib/write-handoff.sh"
# shellcheck source=lib/write-wave-state.sh
source "${SCRIPT_DIR}/lib/write-wave-state.sh"
# shellcheck source=lib/commit-to-artifacts.sh
source "${SCRIPT_DIR}/lib/commit-to-artifacts.sh"

main() {
  # TODO S-18.01 — implemented in TDD green step
  # Stub: exits non-zero so all Red Gate tests fail.
  echo "TODO S-18.01: wave-handoff not yet implemented" >&2
  exit 1
}

main "$@"
