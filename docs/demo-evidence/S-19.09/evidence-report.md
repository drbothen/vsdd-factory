---
story_id: S-19.09
title: "post-E-19 host ABI fixes: read_prefix production path registration, timeout_ms framing, telemetry hygiene"
version: "1.0"
recorded: 2026-07-15
branch: feature/S-19.09
head: 923ebff4
product_type: Rust library (no UI)
evidence_mode: captured-stdout test transcripts
---

# Demo Evidence — S-19.09

**Story:** S-19.09 — post-E-19 host ABI fixes: `read_prefix` production path registration, `timeout_ms` framing, telemetry hygiene
**Epic:** E-19 — Post-rc.22 Operator Hardening
**BC gates:** BC-1.17.001 v1.7 (AC-001/AC-002/AC-003); BC-3.08.001 v1.24 §Postconditions Event 6 (AC-009)
**LOCAL cascade:** CONVERGED 3/3

This story delivers four deliverables (D19–D22) in `crates/factory-dispatcher/src/`. There is no UI
or interactive CLI entry point. Evidence is provided as captured-stdout transcripts from `cargo test`
and `bats` runs, constituting the TDD red-gate-to-green proof per the VSDD library demo-recorder mode.

---

## Coverage Matrix

| AC | Criterion (summary) | Test(s) | Transcript | Result |
|----|---------------------|---------|------------|--------|
| AC-001 | `read_prefix` import instantiates without link error via `setup_host_on_store_data` (production path) | T-001 (cargo) | transcript-AC001-003-T015-production-linker.txt | PASS |
| AC-002 | Round-trip read via production path: bytes correct + `out_ptr > 0` (memory-grow protocol) | T-002, T-002b (cargo) | transcript-AC001-003-T015-production-linker.txt | PASS |
| AC-003 | `CAPABILITY_DENIED (-1)` returned via production path when `read_prefix` cap absent | T-003 (cargo) | transcript-AC001-003-T015-production-linker.txt | PASS |
| AC-004 | Stale `epoch interruption` comment absent from both files; corrected `structurally unenforced` text present | T-004..T-007 (bats) + direct grep | transcript-AC004-005-bats-hygiene.txt | PASS |
| AC-005 | Two-linker `out_ptr=0` duality comment present in `read_file.rs` (`Linker<StoreData>/setup_host_on_store_data`) | T-008 (bats) + direct grep | transcript-AC004-005-bats-hygiene.txt | PASS |
| AC-006 | `pub const INTERNAL_FILE_NOT_FOUND: &str = "internal.file_not_found";` exported from `internal_log.rs` | T-009 (cargo) + grep | transcript-AC006-007-const-value-pin.txt | PASS |
| AC-007 | `pub const PLUGIN_ABANDONED: &str = "plugin.abandoned";` exported from `internal_log.rs` | T-010 (cargo) + grep | transcript-AC006-007-const-value-pin.txt | PASS |
| AC-008 | Zero bare literals `"internal.file_not_found"`, `"plugin.abandoned"`, `"plugin.completed"`, `"plugin.timeout"` in production code of all three target files | T-011/T-012 (bats) + awk-scoped grep | transcript-AC008-bare-literal-sweep.txt | PASS |
| AC-009 | `emit_plugin_completed_async` emits event with non-empty `timestamp` field | T-013 (cargo) | transcript-AC009-timestamp-field.txt | PASS |
| AC-010 | `cargo test --workspace --all-targets`: 2055 passed, 0 failed | workspace summary | transcript-AC010-workspace-regression.txt | PASS |

---

## AC-001: Production Path Instantiation — No Link Error (D19)

**Transcript:** `transcript-AC001-003-T015-production-linker.txt`
**Test:** `t001_s19_09_read_prefix_instantiates_without_link_error_via_production_linker` (invoke.rs `#[cfg(test)]`)

The test constructs a minimal WAT module with a `vsdd::read_prefix` import declaration and
instantiates it via `setup_host_on_store_data` (the production dispatch path). Before D19, this
test failed with a wasmtime link error because `read_prefix` was only registered on the test-path
`Linker<HostContext>` in `host/mod.rs::setup_linker`, not on the production-path `Linker<StoreData>`
in `invoke.rs::setup_host_on_store_data`.

```
test invoke::tests::t001_s19_09_read_prefix_instantiates_without_link_error_via_production_linker ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 203 filtered out; finished in 0.06s
```

**BC Trace:** BC-1.17.001 v1.7 (production-path instantiation, no link error); ADR-025 §Decision 16.

---

## AC-002: Round-Trip Read — Correct Bytes + `out_ptr > 0` (D19)

**Transcript:** `transcript-AC001-003-T015-production-linker.txt`
**Tests:**
- `t002_s19_09_read_prefix_round_trip_bytes_correct_and_out_ptr_nonzero_via_production_path` —
  writes a tmp file with known content; calls `read_prefix` via `setup_host_on_store_data`; asserts
  returned bytes equal expected content; asserts `out_ptr_out > 0` (production memory-grow protocol
  at `current_bytes`, confirming the WASM linear memory was grown and written).
- `t002b_s19_09_read_prefix_head_c_bound_clamps_out_len_to_max_bytes` — file larger than `max_bytes`;
  asserts `out_len_out == max_bytes` (head-c semantics at the production path boundary; T-002b).

```
test invoke::tests::t002_s19_09_read_prefix_round_trip_bytes_correct_and_out_ptr_nonzero_via_production_path ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 203 filtered out; finished in 0.01s

test invoke::tests::t002b_s19_09_read_prefix_head_c_bound_clamps_out_len_to_max_bytes ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 203 filtered out; finished in 0.01s
```

**BC Trace:** BC-1.17.001 v1.7 PC-1 + PC-2; ADR-025 §Decision 16 (memory-grow protocol at `current_bytes`).

---

## AC-003: `CAPABILITY_DENIED` via Production Path (D19)

**Transcript:** `transcript-AC001-003-T015-production-linker.txt`
**Tests:**
- `t003_s19_09_read_prefix_capability_absent_returns_capability_denied_via_production_path` —
  constructs a `StoreData` with no `read_prefix` capability block; calls `setup_host_on_store_data`;
  asserts return code is `codes::CAPABILITY_DENIED (-1)`.
- `t015_s19_09_read_prefix_empty_file_returns_ok_with_zero_ptr_len_no_grow` (T-015, EC-002 lock) —
  empty file via production path returns `codes::OK (0)` with `ptr=0, len=0` and no WASM memory
  growth; closes F-P6-001.

```
test invoke::tests::t003_s19_09_read_prefix_capability_absent_returns_capability_denied_via_production_path ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 203 filtered out; finished in 0.00s

test invoke::tests::t015_s19_09_read_prefix_empty_file_returns_ok_with_zero_ptr_len_no_grow ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 203 filtered out; finished in 0.01s
```

**BC Trace:** BC-1.17.001 v1.7 PC-4 (CAPABILITY\_DENIED via production path); EC-002 (empty file → `OK + ptr=0, len=0`).

---

## AC-004: `timeout_ms` Doc Correction — Epoch Interruption Text Removed (D20)

**Transcript:** `transcript-AC004-005-bats-hygiene.txt`
**Tests:** bats T-004/T-005 (Gate A: stale text absent) + T-006/T-007 (Gate B: corrected text present)

Gate A (negative — stale `epoch interruption` comment absent):
```
ok 1 T-004 AC-004 Gate A: stale 'epoch interruption' text absent from read_file.rs
ok 2 T-005 AC-004 Gate A: stale 'epoch interruption' text absent from read_prefix.rs
```

Gate B (positive — corrected `structurally unenforced` text present):
```
ok 3 T-006 AC-004 Gate B: corrected 'structurally unenforced' text present in read_file.rs
ok 4 T-007 AC-004 Gate B: corrected 'structurally unenforced' text present in read_prefix.rs
```

Direct grep confirmation:
```
$ ! grep -q "epoch interruption" crates/factory-dispatcher/src/host/read_file.rs
Gate A: stale text absent in read_file.rs — exit 0
$ ! grep -q "epoch interruption" crates/factory-dispatcher/src/host/read_prefix.rs
Gate A: stale text absent in read_prefix.rs — exit 0
$ grep -q "structurally unenforced" crates/factory-dispatcher/src/host/read_file.rs
Gate B: corrected text present in read_file.rs — exit 0
$ grep -q "structurally unenforced" crates/factory-dispatcher/src/host/read_prefix.rs
Gate B: corrected text present in read_prefix.rs — exit 0
```

The corrected form reads: `// accepted for ABI forward-compatibility; per-host-function timeout is structurally unenforced in the current synchronous func_wrap dispatch path; the store-level epoch deadline governs coarse plugin-level time.`

**BC Trace:** ADR-025 §Decision 18 (accurate framing: synchronous `func_wrap` dispatch path cannot be interrupted by epoch ticks; store-level epoch deadline governs coarse plugin-level time only).

---

## AC-005: Two-Linker `out_ptr=0` Duality Comment in `read_file.rs` (D20)

**Transcript:** `transcript-AC004-005-bats-hygiene.txt`
**Test:** bats T-008

```
ok 5 T-008 AC-005: two-linker duality comment (Linker<StoreData>/setup_host_on_store_data) present in read_file.rs
```

Direct grep confirmation:
```
$ grep -qE "Linker.*StoreData|setup_host_on_store_data" crates/factory-dispatcher/src/host/read_file.rs
Gate: two-linker duality comment present in read_file.rs — exit 0
```

The comment documents the intentional duality: the test path (`Linker<HostContext>` / `setup_linker`
in `host/mod.rs`) always returns `out_ptr=0` (handled by the hook-sdk `read_owned_bytes` ptr==0
guard); the production path (`Linker<StoreData>` / `setup_host_on_store_data` in `invoke.rs`) grows
WASM memory and writes at `current_bytes > 0`. This anchor (`Linker<StoreData>` /
`setup_host_on_store_data`) was not present in the file before D20.

**BC Trace:** ADR-025 §Decision 17 (two-linker protocol boundary documented in `read_file.rs`).

---

## AC-006: `INTERNAL_FILE_NOT_FOUND` Constant Exported from `internal_log.rs` (D21)

**Transcript:** `transcript-AC006-007-const-value-pin.txt`
**Test:** T-009 (`t_009_internal_file_not_found_value_pin` in `internal_log.rs` `#[cfg(test)]`)

```
test internal_log::tests::t_009_internal_file_not_found_value_pin ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 203 filtered out; finished in 0.00s
```

Direct grep gate:
```
$ grep -qF 'pub const INTERNAL_FILE_NOT_FOUND: &str = "internal.file_not_found";' crates/factory-dispatcher/src/internal_log.rs
PASS: INTERNAL_FILE_NOT_FOUND exported — exit 0
```

T-009 asserts `assert_eq!(INTERNAL_FILE_NOT_FOUND, "internal.file_not_found")` — value-pin locking
the constant to its intended literal, closing any future rename mismatch at compile-time.

**BC Trace:** F-WG-002 (bare string literal `"internal.file_not_found"` eliminated from production code).

---

## AC-007: `PLUGIN_ABANDONED` Constant Exported from `internal_log.rs` (D21)

**Transcript:** `transcript-AC006-007-const-value-pin.txt`
**Test:** T-010 (`t_010_plugin_abandoned_value_pin` in `internal_log.rs` `#[cfg(test)]`)

```
test internal_log::tests::t_010_plugin_abandoned_value_pin ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 203 filtered out; finished in 0.00s
```

Direct grep gate:
```
$ grep -qF 'pub const PLUGIN_ABANDONED: &str = "plugin.abandoned";' crates/factory-dispatcher/src/internal_log.rs
PASS: PLUGIN_ABANDONED exported — exit 0
```

**BC Trace:** F-WG-002 (bare string literal `"plugin.abandoned"` eliminated from production code).

---

## AC-008: Zero Bare Literals in Production Code of Target Files (D21 + F-P1-002)

**Transcript:** `transcript-AC008-bare-literal-sweep.txt`
**Tests:** bats T-011 (three-file four-literal awk-scoped gate) + T-012 (cargo regression)

```
ok 8 T-011 AC-008: zero bare literals in production code of read_file.rs, read_prefix.rs, emit_event.rs, executor.rs, vsdd_sink.rs
ok 9 T-012 AC-008 regression: cargo test -p factory-dispatcher passes after D21 sweep
```

Direct awk-scoped grep (production code only, before `#[cfg(test)]` boundary):
```
read_file.rs: '' (empty = PASS)     — zero bare "internal.file_not_found" or "plugin.abandoned"
read_prefix.rs: '' (empty = PASS)   — zero bare "internal.file_not_found" or "plugin.abandoned"
emit_event.rs: '' (empty = PASS)    — zero bare "internal.file_not_found", "plugin.abandoned",
                                       "plugin.completed", or "plugin.timeout"
                                       (F-P1-002 four-literal extension)
```

All production call sites for all four bare literals now reference named constants
(`INTERNAL_FILE_NOT_FOUND`, `PLUGIN_ABANDONED`, `PLUGIN_COMPLETED`, `PLUGIN_TIMEOUT`). Existing
test assertions (`assert_eq!(e.type_, "internal.file_not_found")`) pass unmodified — constant
values are byte-identical to the literals they replace.

**BC Trace:** F-WG-002 + F-P1-002 (four-literal extension: `"plugin.completed"` + `"plugin.timeout"` in `emit_event.rs` also swept; TD-VSDD-060 sibling-sweep closure).

---

## AC-009: `timestamp` Field in `plugin.completed` Async Event (D22)

**Transcript:** `transcript-AC009-timestamp-field.txt`
**Test:** T-013 (`test_s19_09_t013_emit_plugin_completed_async_has_timestamp_field` in `emit_event.rs` `#[cfg(test)]`)

```
running 1 test
test host::emit_event::tests::test_s19_09_t013_emit_plugin_completed_async_has_timestamp_field ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 203 filtered out; finished in 0.00s
```

T-013 follows the T-003 pattern (same file): constructs a `HostContext` with a capturing sink,
calls `emit_plugin_completed_async`, retrieves the captured `InternalEvent`, and asserts
`event.fields["timestamp"]` is present as a non-empty string. Before D22, `emit_plugin_completed_async`
omitted `let ts = ev.ts.clone()` and the `.with_field("timestamp", ts.as_str())` chain — it was the
only emitter in the file without a `timestamp` field.

**BC Trace:** BC-3.08.001 §Postconditions Event 6 Mandatory-fields (timestamp; per-event alias
convention, Events 1-6 parity); F-WG-003.

---

## AC-010: Full Workspace Regression Gate

**Transcript:** `transcript-AC010-workspace-regression.txt`

```
$ cargo test --workspace --all-targets 2>&1 | awk '/^test result:/{passed+=$4; failed+=$6} END{print passed " passed, " failed " failed"}'

2055 passed, 0 failed
```

All 2055 workspace tests pass. The factory-dispatcher crate's 204 tests include all S-19.09
T-001/T-002/T-002b/T-003/T-013/T-015 and the T-009/T-010 constant value-pins in `internal_log`.

**bats host-abi-hygiene.bats:** 9/9 pass (T-004..T-012).

Two pre-existing environmental failures appear in `run-all.sh` and are unrelated to S-19.09:
- `resolver-integration`: stale temp file race condition in the test host's `/var/folders/`
  temp directory; passes when run independently after cleanup; not caused by S-19.09 code changes.
- `pass-real-state-md-snapshot`: requires `$REPO_ROOT/.factory/STATE.md`, which is not present in
  the `feature/S-19.09` worktree (the `factory-artifacts` branch is a separate worktree); this is a
  structural worktree environment constraint, not a code regression from S-19.09.
Neither failing suite references any of the five files modified by S-19.09.

**BC Trace:** AC-010 regression gate (S-19.09 story criterion).

---

## Files

| File | Content |
|------|---------|
| `transcript-AC001-003-T015-production-linker.txt` | T-001/T-002/T-002b/T-003/T-015 — D19 production-path cargo tests (AC-001/AC-002/AC-003 + EC-002 lock) |
| `transcript-AC004-005-bats-hygiene.txt` | T-004..T-008 bats + direct grep gates — D20 doc-correction static gates (AC-004/AC-005) |
| `transcript-AC006-007-const-value-pin.txt` | T-009/T-010 cargo + grep gates — D21 constant exports (AC-006/AC-007) |
| `transcript-AC008-bare-literal-sweep.txt` | T-011/T-012 bats + awk-scoped grep — D21 bare-literal sweep (AC-008) |
| `transcript-AC009-timestamp-field.txt` | T-013 cargo — D22 `plugin.completed` timestamp field (AC-009) |
| `transcript-AC010-workspace-regression.txt` | `cargo test --workspace --all-targets` summary — regression gate (AC-010) |
| `evidence-report.md` | This file — coverage matrix + per-AC narrative |

---

## Behavioral Discrepancies Found

None. All 10 ACs satisfied against HEAD `923ebff4` on `feature/S-19.09`. LOCAL cascade
CONVERGED 3/3 prior to demo recording.
