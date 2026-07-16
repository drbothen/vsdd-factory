---
story_id: S-19.07
title: "verify-factory-lock read_prefix migration (D18(e))"
version: "1.0"
recorded: 2026-07-16
branch: feature/S-19.07
head: b0b18dce
product_type: Rust WASM hook plugin (no UI)
evidence_mode: captured-stdout test transcripts
---

# Demo Evidence — S-19.07

**Story:** S-19.07 — verify-factory-lock read\_prefix migration (D18(e))
**Epic:** E-19 — Post-rc.22 Operator Hardening
**BC gate:** BC-4.13.001 v1.17 Phase-B (AC-001 Phase-B Precondition 3 call site + symbol removal; AC-002 registry capability migration; AC-003 correct verdict from 262144-byte prefix; EC-005 graceful degrade)
**VP:** VP-095 v1.5 (Phase-B dual-anchor; proof\_method: unit + static; harness: AC-003 unit family in lib.rs + bats T-001b Gate B static gate)
**LOCAL cascade:** CONVERGED 3/3

This story delivers the Phase-B `read_prefix` migration of the `verify-factory-lock` WASM hook plugin — a Rust WASM hook plugin with no UI or interactive CLI entry point. Evidence is provided as captured-stdout transcripts from `cargo test` and `bats` runs, per the library/test-harness demo-recorder mode.

---

## Coverage Matrix

| AC | Criterion (summary) | Test(s) | Transcript | Result |
|----|---------------------|---------|------------|--------|
| AC-001 | `host::read_prefix` called (Gate A) + Phase-A symbols absent from non-comment code (Gate B) | T-001a, T-001b (bats) + prod grep | transcript-AC001-migration-gates.txt | PASS |
| AC-002 | Both registry entries migrated from `capabilities.read_file` to `capabilities.read_prefix` with `path_allow = [".factory/STATE.md"]` | T-002-vfl, T-002-vfl-bash (bats) + grep | transcript-AC002-registry-capability.txt | PASS |
| AC-003 | 262144 bound + lock semantics: real-shape 35 KB frontmatter fixture (FP1002) + regression fixture + EC-001 boundary; 31/31 unit tests GREEN | T-003/T-004 (FP1002), T-006/T-007 (regression), EC-001 (boundary); full suite 31 (cargo) | transcript-AC003-262144-unit-family.txt | PASS |
| EC-005 | `capabilities.read_prefix` absent (only `read_file` present) + foreign lock → read\_prefix CAPABILITY\_DENIED → Continue (fail-open); real dispatcher | T-005-ec005 (bats) | transcript-EC005-capability-denied-degrade.txt | PASS |

---

## AC-001: Migration Gates — `host::read_prefix` Present + Phase-A Symbols Absent

**Transcript:** `transcript-AC001-migration-gates.txt`
**Tests:** T-001a (Gate A, bats), T-001b (Gate B, bats)

**Gate A** (`grep -q "read_prefix" lib.rs` via awk prod-region scoping, exits 0):
The production code region of `crates/hook-plugins/verify-factory-lock/src/lib.rs` (stopped at `#[cfg(test)]` by `awk`) contains `host::read_prefix`. The production call site is:

```
line 330:    let state_bytes = match (callbacks.read_prefix)(".factory/STATE.md", 262144, READ_TIMEOUT_MS) {
```

`max_bytes=262144` is the ADR-025 §Decision 15 v1.18 adjudicated envelope bound. `extract_frontmatter` is retained unchanged.

**Gate B** (sed block-comment strip + line-comment filter + grep, exits non-zero):
Phase-A symbols (`host::read_file`, `STATE_MD_MAX_BYTES`, `TooLarge`) are absent from non-comment production code. The gate uses:
- awk scoping to exclude the `#[cfg(test)]` module (test module error messages contain these symbol names as string literals — without scoping the gate would never go GREEN)
- sed block-comment strip (`-e ':a' -e 's:/\*...*\*+([^/*]...*/::' -e 'ta'`) to strip `/* ... */` blocks (F-P9-003 block-comment defense)
- `grep -vE '^\s*(//|//!|///)` to strip line comments

**Gate B mutation-liveness check** (T-009g convention): a discriminating fixture `fn read_data() -> i32 { /* host::read_file(path, STATE_MD_MAX_BYTES) TooLarge check */ 0 }` is run through the gate. The sed chain produces empty output (symbols inside block comment are excluded), proving the chain is load-bearing. An old line-comment-only gate would find these symbols and emit a false positive.

```
1..5
ok 1 T-001a S-19.07 AC-001 Gate A: host::read_prefix called in verify-factory-lock/src/lib.rs
ok 2 T-001b S-19.07 AC-001 Gate B: Phase-A symbols absent from non-comment code in verify-factory-lock/src/lib.rs
```

**BC Trace:** BC-4.13.001 v1.17 Phase-B Precondition 3 (read\_prefix call site present; Phase-A symbols removed from non-comment code).

---

## AC-002: Registry Capability Migration

**Transcript:** `transcript-AC002-registry-capability.txt`
**Tests:** T-002-vfl, T-002-vfl-bash (bats)

Both `verify-factory-lock` and `verify-factory-lock-bash` registry entries in `plugins/vsdd-factory/hooks-registry.toml` have been migrated from `[hooks.capabilities.read_file]` to `[hooks.capabilities.read_prefix]`, keeping `path_allow = [".factory/STATE.md"]` unchanged.

Per-entry awk scoping (`BEGIN{p=0} /^\[\[hooks\]\]/{p=0} /^name = "..."/p`) isolates each entry independently — this is the BC-4.13.001 AC-002 gate form, robust when entries are appended after the verify-factory-lock-bash entry.

```
-- verify-factory-lock entry:
[hooks.capabilities.read_prefix]
path_allow = [".factory/STATE.md"]

-- verify-factory-lock-bash entry:
[hooks.capabilities.read_prefix]
path_allow = [".factory/STATE.md"]

capabilities.read_file count for verify-factory-lock:      0
capabilities.read_file count for verify-factory-lock-bash: 0
```

```
ok 3 T-002-vfl S-19.07 AC-002: verify-factory-lock registry entry has capabilities.read_prefix not capabilities.read_file
ok 4 T-002-vfl-bash S-19.07 AC-002: verify-factory-lock-bash registry entry has capabilities.read_prefix not capabilities.read_file
```

**BC Trace:** BC-4.13.001 v1.17 Phase-B Precondition 3 (registry migration); BC-1.17.001 Invariant 3 (capability independence: `read_file` does NOT grant `read_prefix` access — explicit migration required).

---

## AC-003: 262144 Bound + Lock Semantics Unit Family (31/31)

**Transcript:** `transcript-AC003-262144-unit-family.txt`
**Tests:** T-003 (`test_S1907_FP1002_real_shape_35kb_frontmatter_foreign_lock_blocks`), T-004 (`test_S1907_FP1002_real_shape_35kb_frontmatter_no_lock_continues_without_warns`), T-006 (`test_S1907_T003_read_prefix_262144_large_fixture_foreign_lock_blocks`), T-007 (`test_S1907_T004_read_prefix_262144_large_fixture_no_lock_continues_without_warns`), EC-001 (`test_S1907_EC001_read_prefix_262144_delimiter_at_boundary_blocks_on_foreign_lock`)

**31/31 unit tests GREEN:**

```
$ cargo test -p verify-factory-lock --lib

running 31 tests
...
test tests::test_S1907_FP1002_real_shape_35kb_frontmatter_foreign_lock_blocks ... ok
test tests::test_S1907_FP1002_real_shape_35kb_frontmatter_no_lock_continues_without_warns ... ok
test tests::test_S1907_T003_read_prefix_262144_large_fixture_foreign_lock_blocks ... ok
test tests::test_S1907_T004_read_prefix_262144_large_fixture_no_lock_continues_without_warns ... ok
test tests::test_S1907_EC001_read_prefix_262144_delimiter_at_boundary_blocks_on_foreign_lock ... ok

test result: ok. 31 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```

**FP1002 real-shape tests (T-003/T-004):** These are the mutation-check tests that closed F-P1-001 BLOCKER. The fixture has frontmatter ~35 KB (factory\_lock block at ~35 KB from start, immediately before the closing `---`; ~27 KB beyond the retracted 8192-byte window). Body is padded to >= 262144 bytes total. This fixture proves that `read_prefix(max_bytes=262144)` returns enough bytes to include the `factory_lock` block, whereas the old 8192-byte derivation would have been silently inert (the guard receiving an 8192-byte prefix with no closing `---` would fall through to `extract_frontmatter`'s full-input fallback → `MalformedLockBlock` → fail-open `Continue`, never enforcing the lock).

**T-006/T-007 regression fixtures:** factory\_lock in first ~512B (small-frontmatter STATE.md), body > 8192 bytes. Confirms Phase-B does not regress on small-frontmatter STATE.md.

**EC-001 boundary test:** STATE.md closing `---` delimiter exactly at byte 262143 (last byte of prefix). `read_prefix` returns full 262144 bytes; `extract_frontmatter` finds the closing delimiter at the end; verdict correct. Proves Phase-B works at the VP-095 v1.5 byte-envelope boundary.

**BC Trace:** BC-4.13.001 v1.17 Phase-B (frontmatter parsed from 262144-byte prefix; `extract_frontmatter` retained unchanged; real-shape ~35 KB frontmatter validated; 8192 premise-false derivation retracted); VP-095 v1.5 (Phase-B form: unit + static proof\_method; AC-003 unit family named as harness).

---

## EC-005: Capability-Denied Graceful Degrade (Real Dispatcher)

**Transcript:** `transcript-EC005-capability-denied-degrade.txt`
**Test:** T-005-ec005 (bats)

This test exercises the misconfiguration scenario where a deployer migrates the plugin source to Phase-B but forgets to update the registry — the registry still has `capabilities.read_file` (not `read_prefix`) for the `verify-factory-lock` entry, and STATE.md has a foreign unexpired lock.

**Phase-B behavior (GREEN):** The Phase-B guard calls `host::read_prefix` → dispatcher looks up `read_prefix` capability for this plugin → not present → returns `CAPABILITY_DENIED (-1)` → plugin maps as `StateReadError` → `HookResult::Continue` (fail-open per PC6) → dispatcher exit 0.

**Phase-A behavior (was RED gate):** The Phase-A guard calls `host::read_file` → capability present → reads STATE.md → finds foreign lock → BLOCKS (exit 2) → test assertion `exit 0` failed → RED.

This is the load-bearing behavioral discriminator: the bats test was RED against Phase-A and goes GREEN only after Phase-B migration.

```
ok 5 T-005-ec005 S-19.07 EC-005: capabilities.read_prefix absent → graceful degrade to Continue
```

The real `factory-dispatcher` binary (`target/release/factory-dispatcher`) is used — not a mock. The `CLAUDE_PLUGIN_ROOT` and `CLAUDE_PROJECT_DIR` environment variables are set to a tmp workspace with the EC-005 registry fixture.

Operator visibility is preserved via the standard `internal.capability_denied` event class logged by the dispatcher on every denied call. No bespoke plugin-level `log_warn` is needed; this provides parity with Phase-A's `state_md_approaching_cap` diagnostic event per EC-005 Expected Behavior note.

**BC Trace:** BC-4.13.001 v1.17 Phase-B (EC-005: capability denied → fail-open Continue per PC6); BC-1.17.001 Invariant 3 (read\_file-only registry does NOT grant read\_prefix access).

---

## Full Test Summary

**cargo test -p verify-factory-lock --lib:** 31 passed / 0 failed
**bats verify-factory-lock-read-prefix.bats:** 5 passed / 0 failed

0 failures across all tests.

---

## Notes on Evidence Mode

S-19.07 migrates the `verify-factory-lock` WASM hook plugin (`crates/hook-plugins/verify-factory-lock/src/lib.rs`) from `host::read_file` to `host::read_prefix`, and updates both registry entries in `plugins/vsdd-factory/hooks-registry.toml`. There is no UI or interactive CLI entry point. Evidence is captured-stdout transcripts per the VSDD library/test-harness demo-recorder mode. All transcripts are reproducible by running `cargo test -p verify-factory-lock --lib` and `cd plugins/vsdd-factory/tests && bats verify-factory-lock-read-prefix.bats` on branch `feature/S-19.07` (HEAD `b0b18dce`).

---

## Behavioral Discrepancies Found

None. All 31 unit tests and 5 bats tests pass against the implementation on `feature/S-19.07`. No behavioral discrepancy between the implementation and the ACs was observed. LOCAL cascade CONVERGED 3/3 prior to demo recording.

---

## Files

| File | Content |
|------|---------|
| `transcript-AC001-migration-gates.txt` | T-001a Gate A + T-001b Gate B (bats) + prod grep for read\_prefix call site at 262144 (AC-001) |
| `transcript-AC002-registry-capability.txt` | T-002-vfl + T-002-vfl-bash (bats) + grep showing both `[hooks.capabilities.read_prefix]` blocks with `path_allow` + negative assertion 0 read\_file (AC-002) |
| `transcript-AC003-262144-unit-family.txt` | Full 31-test cargo output + targeted 5-test AC-003 run: T-003/T-004 FP1002 real-shape, T-006/T-007 regression, EC-001 boundary (AC-003) |
| `transcript-EC005-capability-denied-degrade.txt` | T-005-ec005 (bats) — real dispatcher, read\_prefix-only registry, foreign lock, exit 0 Continue (EC-005) |
| `evidence-report.md` | This file — coverage matrix + per-AC narrative |
