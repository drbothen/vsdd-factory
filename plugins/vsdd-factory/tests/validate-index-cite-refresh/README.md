# validate-index-cite-refresh Tests — bats Suite

**Story:** S-15.07 — validate-index-cite-refresh WASM hook (PostToolUse stale 4-index cite gate)
**BC:** BC-5.39.003
**Phase:** 2/6 (test-writer — Red Gate)
**Closes:** D-405(c) + D-429(b)

## Overview

This directory contains the bats test suite for the `validate-index-cite-refresh` WASM hook at
`plugins/vsdd-factory/hook-plugins/validate-index-cite-refresh.wasm`.

The hook fires PostToolUse on Edit/Write to `ARCH-INDEX.md` and:
1. Verifies all 4-index version cites in ARCH-INDEX body are current (D-405(c))
2. Cross-validates STATE.md and INDEX.md trajectory cells agree with live 4-index versions (D-429(b))

## Test Files

| File | AC | Fixture |
|------|----|---------|
| `pass-all-current.bats` | AC-4 | `pass-all-current/` — all cites current |
| `fail-stale-bc-index.bats` | AC-1, AC-10 | `fail-stale-bc-index/` — BC-INDEX v1.05 cited, live is v2.24 |
| `fail-stale-vp-index.bats` | AC-2 | `fail-stale-vp-index/` — VP-INDEX v1.80 cited, live is v1.97 |
| `fail-stale-story-index.bats` | AC-3 | `fail-stale-story-index/` — STORY-INDEX v3.28 cited, live is v3.31 |
| `fail-cross-cell-state-md.bats` | AC-5 | `fail-cross-cell-state-md/` — STATE.md cites stale STORY-INDEX |
| `fail-cross-cell-index-md.bats` | AC-6 | `fail-cross-cell-index-md/` — INDEX.md cites stale BC-INDEX |
| `fail-open-missing-index.bats` | AC-7 | `fail-open-missing-index/` — BC-INDEX.md absent |

AC-8 (WASM compiles cleanly), AC-9 (hooks-registry.toml entry present), and AC-11 (pre-flight
4-gate) are validated by the implementer phase and CI gate, not by these bats tests.

## Fixture Layout

Fixtures live at `plugins/vsdd-factory/tests/fixtures/validate-index-cite-refresh/<scenario>/factory/`.

Each fixture uses `factory/` as a directory name (not `.factory/`) to avoid `factory-branch-guard`
hook interference during test authoring. The bats `setup()` function copies the fixture to a tmpdir
and renames `factory/` to `.factory/` before running the dispatcher+WASM invocation. This ensures
the WASM hook sees the expected `.factory/...` path structure via `host::read_file`.

## Dispatcher Invocation Pattern

Each test:
1. Copies fixture to `WORK=$(mktemp -d)` and renames `factory/` -> `.factory/`
2. Writes a synthetic `hooks-registry.toml` with only the `validate-index-cite-refresh` entry
3. Copies the WASM binary to `$WORK/hook-plugins/`
4. Runs `CLAUDE_PLUGIN_ROOT=$WORK CLAUDE_PROJECT_DIR=$WORK factory-dispatcher <PostToolUse envelope>`
5. Asserts on exit code (0=pass, 2=block) and stderr `blocking_plugins=` / `block_reason=` content

## Red Gate Status

All 7 tests are in SKIP state (`skip "WASM binary not built..."`) until the implementer
compiles `validate-index-cite-refresh.wasm` in Phase 3/6 (T-7 of S-15.07).

This is the intended Red Gate state per BC-5.38.001.

## Running

```bash
# Full suite (picked up via run-all.sh glob):
cd plugins/vsdd-factory/tests && ./run-all.sh

# Just this suite:
bats plugins/vsdd-factory/tests/validate-index-cite-refresh/

# Single test:
bats plugins/vsdd-factory/tests/validate-index-cite-refresh/pass-all-current.bats
```
