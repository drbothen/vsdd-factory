# Dim-2 Gate Tests — bats Suite

**Story:** S-15.08 — Dim-2 gate template library (11 canonical bash scripts for fix-burst attestation)
**Phase:** 2/6 (test-writer — Red Gate)

## Overview

This directory contains the bats test suite for the 11 dim2-gate bash scripts in
`plugins/vsdd-factory/hooks/dim2-gates/`. These scripts implement canonical mechanical
gates for Dim-2 fix-burst attestation per D-449(a), closing D-450(b/c/d/e), D-451(a),
D-452(a/b), D-453(e), and D-454(a/b/c/d).

## Naming Convention

One `.bats` file per script, named to match the script:

| Script | bats file |
|--------|-----------|
| `trajectory-tail-cell-grep.sh` | `trajectory-tail-cell-grep.bats` |
| `freshness-literal-stdout.sh` | `freshness-literal-stdout.bats` |
| `block-label-canonical-form.sh` | `block-label-canonical-form.bats` |
| `banner-wc-l.sh` | `banner-wc-l.bats` |
| `propagation-completeness.sh` | `propagation-completeness.bats` |
| `dim7-dispatched-count-sweep.sh` | `dim7-dispatched-count-sweep.bats` |
| `dim1-file-count-arithmetic.sh` | `dim1-file-count-arithmetic.bats` |
| `active-branches-sha-currency.sh` | `active-branches-sha-currency.bats` |
| `decision-log-monotonic-rows.sh` | `decision-log-monotonic-rows.bats` |
| `layer-ordinal-dual-direction.sh` | `layer-ordinal-dual-direction.bats` |
| `meta-level-ack-grep.sh` | `meta-level-ack-grep.bats` |

## Fixture Pattern

Each script has a pair of fixture directories under `tests/fixtures/dim2-gates/`:

- `<script-name>-pass/` — well-formed factory-artifacts input; script exits 0
- `<script-name>-fail/` — input with injected defect; script exits 1

Each fixture directory contains:
- The minimal factory structure the script reads (STATE.md, burst-log.md, decision-log.md, etc.)
- A `README.md` documenting the scenario, injected defect (if any), expected exit code, and invocation

## Test Structure

Each `.bats` file has exactly 2 `@test` blocks:
- `PASS: <script> exits 0 when <well-formed condition>`
- `FAIL: <script> exits 1 when <defect condition>`

Tests invoke the script as `"$PLUGIN_ROOT/hooks/dim2-gates/<script>.sh"` via the bats `run`
command and assert on `$status` and `$output`.

## Running

Run the full suite (picks up this directory via updated `run-all.sh`):
```
cd plugins/vsdd-factory/tests && ./run-all.sh
```

Run just this dim2-gates suite:
```
bats plugins/vsdd-factory/tests/dim2-gates/
```

Run a single script's tests:
```
bats plugins/vsdd-factory/tests/dim2-gates/trajectory-tail-cell-grep.bats
```

## Red Gate Status

During phase 2/6 (test-writer), all 22 test cases FAIL because the 11 scripts do not
yet exist. Each test will report `command not found` (exit 127) when the script is missing,
causing the status assertions to fail. This is the intended Red Gate state per BC-5.38.001.

The implementer (phase 3/6) writes the script bodies to make each test green.

## active-branches-sha-currency env-var override

The `active-branches-sha-currency.bats` tests set `GIT_TEST_SHA_OVERRIDE_<branch>` env vars
to avoid requiring a real git remote during CI. The script must check for these overrides:

```bash
# In active-branches-sha-currency.sh — env-var override for testability
_get_sha() {
  local branch="$1"
  local env_key="GIT_TEST_SHA_OVERRIDE_$(echo "$branch" | tr '-' '_')"
  if [ -n "${!env_key:-}" ]; then
    echo "${!env_key}"
  else
    git -C "$FACTORY_ROOT" rev-parse "origin/$branch"
  fi
}
```

This pattern keeps the script realistic (real git invocation in production) while
allowing hermetic bats testing without a network connection.
