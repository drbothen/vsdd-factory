#!/usr/bin/env bats
# executable-bit.bats — meta-tests for dim2-gate template script hygiene
#
# Closes: F-S15.08-LOCAL-P1-005 — no bats test verifies [ -x "$script" ] for all 11 scripts.
# Also closes B-007 [process-gap] — EC-008 --help discipline was manually verified (T-4) but
# not test-enforced; this meta-test adds the parametric coverage.
#
# Why: cross-platform/clone hazards (Windows clone, core.fileMode=false, plugin tarball
# mode-stripping) silently break scripts with confusing exit-126 diagnostics. Production-grade
# scripts deployed via the plugin marketplace must preserve mode and shebang discipline.
#
# These 4 meta-tests cover:
#   1. Executable bit ([ -x ]) on all 11 scripts
#   2. Shebang line format (#!/usr/bin/env bash)
#   3. set -euo pipefail presence (first line after shebang)
#   4. --help exits 0 and emits a USAGE keyword (EC-008 test-enforced)
#
# References: AC-001..AC-011 (S-15.08), EC-008, CLAUDE.md Bash hook scripts discipline

setup() {
  PLUGIN_ROOT="$(cd "${BATS_TEST_DIRNAME}/../.." && pwd)"
  HOOKS_DIR="${PLUGIN_ROOT}/hooks/dim2-gates"
}

@test "all 11 dim2-gate scripts are executable" {
  local failed=0
  for script in "${HOOKS_DIR}"/*.sh; do
    if [ ! -x "$script" ]; then
      echo "not executable: $script" >&2
      failed=1
    fi
  done
  [ "$failed" -eq 0 ]
}

@test "all 11 dim2-gate scripts have #!/usr/bin/env bash shebang" {
  local failed=0
  for script in "${HOOKS_DIR}"/*.sh; do
    if ! head -n 1 "$script" | grep -qE '^#!/usr/bin/env bash'; then
      echo "missing/wrong shebang: $script (found: $(head -n 1 "$script"))" >&2
      failed=1
    fi
  done
  [ "$failed" -eq 0 ]
}

@test "all 11 dim2-gate scripts have set -euo pipefail" {
  local failed=0
  for script in "${HOOKS_DIR}"/*.sh; do
    if ! grep -qE '^set -euo pipefail' "$script"; then
      echo "missing set -euo pipefail: $script" >&2
      failed=1
    fi
  done
  [ "$failed" -eq 0 ]
}

@test "all 11 dim2-gate scripts respond to --help with exit 0 and USAGE keyword (EC-008)" {
  local failed=0
  for script in "${HOOKS_DIR}"/*.sh; do
    run "$script" --help
    if [ "$status" -ne 0 ]; then
      echo "$script --help exited $status (expected 0)" >&2
      failed=1
    fi
    if ! echo "$output" | grep -qiE 'USAGE|Usage|usage'; then
      echo "$script --help missing USAGE keyword" >&2
      failed=1
    fi
  done
  [ "$failed" -eq 0 ]
}
