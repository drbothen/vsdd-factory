---
story: S-21.01
title: "validate-factory-path-staging WASM guard + orchestrator merge pre-check"
version: "1.8"
evidence_produced: "2026-07-23"
produced_by: demo-recorder
method: scripted-terminal-capture
---

# S-21.01 Per-AC Demo Evidence Report

**Story:** S-21.01 — validate-factory-path-staging WASM guard + Layer-2 protocol
**Epic:** E-21 — Factory State Data-Loss Hardening
**Story version:** v1.8 (9 ACs)
**ACs covered:** AC-001 through AC-009 (all 9)
**Method note:** This is a CLI/hook artifact. VHS is not installed; evidence uses scripted
terminal captures (text output logs from bats + cargo + grep). This note is included per
the demo-recorder instruction ("note which").

---

## Evidence Artifact Index

| Artifact file | Contents | ACs covered |
|---------------|----------|-------------|
| `ac-001-registry-entry.txt` | grep/cat of hooks-registry.toml validate-factory-path-staging entry; field-by-field pass verification | AC-001 |
| `ac-001-ac005-bats-dispatcher.txt` | Full bats run (36/36 ok), real dispatcher invocations; T-001..T-030 | AC-001..AC-005, AC-007..AC-009 |
| `ac-006-proptest-cargo.txt` | cargo test -p validate-factory-path-staging; 133 unit + 5 proptest; coverage detail | AC-006 |
| `ac-007-ac009-layer2-grep.txt` | Section-scoped grep captures from per-story-delivery.md for all Layer-2 requirements | AC-007..AC-009 |

---

## AC Coverage Table

| AC | Requirement | Artifact(s) | Success path | Error/fail path | Notes |
|----|-------------|-------------|--------------|-----------------|-------|
| AC-001 | guard blocks `git add .factory/<path>` on product branch; exit 2 + `FactoryPathOnProductBranch` | `ac-001-registry-entry.txt`, `ac-001-ac005-bats-dispatcher.txt` | T-006..T-006d (registry shape: ok 9-12); T-001 (block on develop: ok 1) | T-001b (block on main: ok 2); T-001c (git add -A conservative block: ok 3) | Real dispatcher path used |
| AC-002 | guard passes unconditionally on `factory-artifacts` branch | `ac-001-ac005-bats-dispatcher.txt` | T-002 (pass git add .factory/STATE.md on factory-artifacts: ok 4); T-019 (bare .factory: ok 25) | — | Real dispatcher path used |
| AC-003 | guard passes non-`git add`/`git stage` commands (PC4) | `ac-001-ac005-bats-dispatcher.txt` | T-003 (git commit: ok 5); T-003b (git merge: ok 6) | — | Confirms Layer-1 scope narrow (ADR-031 §Decision 2) |
| AC-004 | guard passes `git add` with no `.factory/` paths | `ac-001-ac005-bats-dispatcher.txt` | T-004 (git add src/main.rs on develop: ok 7) | — | Real dispatcher path used |
| AC-005 | guard fails-open on branch detection failure | `ac-001-ac005-bats-dispatcher.txt` | T-005 (no git repo → exit 0: ok 8) | — | Test runs outside any git repo; branch detection fails; guard returns exit 0 per BC-4.16.001 Invariant 3 |
| AC-006 | proptest covers ≥20 diverse `.factory/` path variants; all trigger block on develop | `ac-006-proptest-cargo.txt` | `test_ac006_canonical_factory_path_vectors_block_on_develop` (20+ explicit variants); `prop_BC_4_16_001_ac006_factory_path_variants_block_on_develop` (proptest 256 cases) | `prop_BC_4_16_001_ac006_non_factory_path_passes_on_develop` (non-factory passes) | Cargo test path; proptest harness in crates/hook-plugins/validate-factory-path-staging/tests/ |
| AC-007 | `per-story-delivery.md` has §Main-Checkout Sync Protocol with (a) git diff gate; (b) FactoryPathDeletionInMergeDiff halt; (c) transparent pass; (d) covers git pull/merge | `ac-007-ac009-layer2-grep.txt`, `ac-001-ac005-bats-dispatcher.txt` | T-007..T-010 (ok 13-16); grep captures confirm all 4 sub-requirements | FactoryPathDeletionInMergeDiff halt shown at line 184/191 of per-story-delivery.md | Doc-content assertion; no WASM required |
| AC-008 | merge pre-check passes (merge proceeds) when diff returns no `.factory/` paths | `ac-007-ac009-layer2-grep.txt`, `ac-001-ac005-bats-dispatcher.txt` | T-011 (section-scoped grep for "MUST proceed normally": ok 17) | — | Section-scoped via `_extract_main_checkout_sync_protocol_section` awk helper; avoids tautological whole-file grep |
| AC-009 | merge pre-check fails-open (proceeds with warning) when `git diff` returns non-zero | `ac-007-ac009-layer2-grep.txt`, `ac-001-ac005-bats-dispatcher.txt` | T-012 (non-zero in section-scoped grep: ok 18); T-013 (warning + proceed + non-zero in §Fail-Open subsection: ok 19) | — | §Fail-Open When git diff Fails subsection explicitly documents non-zero exit trigger |

---

## Test Execution Summary

### Bats suite (real dispatcher path)

```
cd plugins/vsdd-factory/tests && bats validate-factory-path-staging.bats
1..36
ok 1..36   (all 36 passed)
EXIT: 0
```

Build commands used:
```
cargo build --target wasm32-wasip1 -p validate-factory-path-staging
cp target/wasm32-wasip1/debug/validate-factory-path-staging.wasm \
   plugins/vsdd-factory/hook-plugins/validate-factory-path-staging.wasm
```

The dispatcher binary was already built at `target/release/factory-dispatcher`.
The WASM was built in debug profile (semantically equivalent for correctness testing;
CI builds release profile with `--release` flag).

### Cargo tests (unit + proptest)

```
cargo test -p validate-factory-path-staging
133 unit tests: ok
5 proptest tests: ok
Total: 138 passed, 0 failed
EXIT: 0
```

---

## Dispatcher vs Fallback Decision

For AC-001..AC-006, the **real dispatcher path** was used as instructed:
- `target/release/factory-dispatcher` was pre-built (found at expected path).
- `validate-factory-path-staging.wasm` was built via `cargo build --target wasm32-wasip1 -p validate-factory-path-staging` and staged to `hook-plugins/`.
- No fallback to cargo-test-only was required.

For AC-007..AC-009 (Layer-2 content checks), no WASM/dispatcher is required by design —
bats T-007..T-013 assert protocol documentation is present by section-scoped grep, which
matches the AC spec ("bats: mock git diff --name-only returning...; assert...").
These tests run unconditionally regardless of build state.

---

## POLICY 10 Compliance

All artifacts committed to the feature branch (`feature/S-21.01`) under the story-scoped
subfolder `docs/demo-evidence/S-21.01/` per POLICY 10.
