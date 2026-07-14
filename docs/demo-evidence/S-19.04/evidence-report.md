# S-19.04 Demo Evidence Report

**Story:** S-19.04 — Registry/bundle hygiene: orphan WASM removal + tool-filter regex anchoring convention + lint check
**Branch:** `feature/S-19.04`
**Implementation commit:** `0a7af81d`
**Evidence produced by:** Demo Recorder
**Date:** 2026-07-13

---

## Summary

S-19.04 closes two rc.22 bundle/registry hygiene gaps discovered during the
post-rc.22 operator install inspection:

**Finding (a) — Orphan WASMs removed:** Three unreferenced WASMs
(`hello-hook.wasm`, `vsdd_context_resolvers.wasm`, `wasm_resolver_export.wasm`)
that shipped in the rc.22 bundle are removed. `hello-hook.wasm` is excluded via
BUILD-OMISSION (both the `--example hello-hook` build step and `hello-hook.wasm`
copy step are removed from release.yml). The two underscore variants are excluded
via the `*_*.wasm` outer-glob arm at both artifact-staging sites in release.yml.
The live WaveContextResolver `vsdd-context-resolvers.wasm` (hyphen) is untouched.

**Finding (b) — Tool-filter anchoring convention + lint:** All `tool =` entries in
`hooks-registry.toml` now use fully-anchored patterns (both leading `^` AND trailing
`$`). A preamble comment documents the regex SEARCH semantics and anchoring
convention. A bats lint suite (`registry-tool-filter-anchoring.bats`, 7 tests)
verifies the convention; a Rust integration test suite (`bundle_orphan_check.rs`,
5 tests) verifies dual-registry orphan detection and the staging simulation.

All 7 acceptance criteria met. 5 cargo integration tests + 7 bats lint tests green.
200 workspace tests pass (fmt + clippy clean).

---

## AC Coverage Map

| AC | Title | Status | Evidence File |
|----|-------|--------|--------------|
| AC-001 | hello-hook.wasm build-omission from release.yml | PASS | [AC-001.md](AC-001.md) |
| AC-002 | Orphan WASMs absent, hyphen survivor present | PASS | [AC-002.md](AC-002.md) |
| AC-003 | hooks-registry.toml preamble comment | PASS | [AC-003.md](AC-003.md) |
| AC-004 | All tool= entries fully anchored (both-ends); negative fixtures flagged | PASS | [AC-004.md](AC-004.md) |
| AC-005 | Bats lint suite 7/7 (incl. T-011/T-012 reject-fixtures) | PASS | [AC-005.md](AC-005.md) |
| AC-006 | Rust dual-registry orphan detection 5/5 (T-006..T-009) | PASS | [AC-006.md](AC-006.md) |
| AC-007 | stage_release_bundle staging simulation T-010 | PASS | [AC-007.md](AC-007.md) |

---

## Test Results

### Bats lint suite — registry-tool-filter-anchoring.bats

```
$ bats plugins/vsdd-factory/tests/registry-tool-filter-anchoring.bats
1..7
ok 1 T-001 AC-004/AC-005: unanchored fixture entry detected by lint
ok 2 T-002 AC-004/AC-005: anchored fixture entry passes lint
ok 3 T-003 AC-005: intent-comment exemption passes lint (EC-001)
ok 4 T-004 AC-004/AC-005: actual hooks-registry.toml has no unanchored tool entries
ok 5 T-005 AC-004: verify-factory-lock tool pattern is anchored and includes MultiEdit
ok 6 T-011 AC-004/AC-005: prefix-only anchor (^Bash, no trailing $) detected as violation
ok 7 T-012 AC-004/AC-005: comment-injection fixture (^Edit with $ in comment) flagged as violation
```

7/7 pass. T-011 closes F6-1 (prefix-only-anchor reject-fixture). T-012 closes F7-1
(comment-injection greedy-dot bypass).

### Cargo integration tests — bundle_orphan_check.rs

```
$ cargo test --test bundle_orphan_check
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.47s
     Running tests/bundle_orphan_check.rs (target/debug/deps/bundle_orphan_check-511188a9d6df476c)

running 5 tests
test test_S_19_04_ac006_T008_negative_control_resolvers_only_is_orphan_with_hooks_only_detection ... ok
test test_S_19_04_ac006_T007_neither_registry_wasm_is_orphan_with_orphan_line ... ok
test test_S_19_04_ac006_T006_resolvers_registry_only_wasm_is_non_orphan ... ok
test test_S_19_04_ac007_T010_release_staging_underscore_glob_excludes_orphans ... ok
test test_S_19_04_ac006_T009_hermetic_tracked_bundle_zero_orphans ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```

5/5 pass. T-009 is the hermetic standing-GREEN gate (uses `git ls-files`, not
`fs::read_dir`, to avoid contamination from untracked cargo build artifacts).
T-008 is the negative-control that confirms the dual-registry check is load-bearing.

### Cargo workspace — supporting evidence

```
$ cargo test --workspace --all-targets
[...200 tests across all workspace crates...]
WORKSPACE TOTAL: 200 tests passed, 0 failed

$ cargo fmt --check --all
fmt: CLEAN

$ cargo clippy --workspace --all-targets -- -D warnings
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.38s
(no warnings)
```

200 workspace tests pass. fmt and clippy clean.

---

## Gate Summary

| Gate | Command | Result |
|------|---------|--------|
| AC-001 (i): no hello-hook build step | `! grep -q 'example hello-hook' .github/workflows/release.yml` | PASS (exit 0) |
| AC-001 (ii): no hello-hook copy step | `! grep -q 'hello-hook.wasm' .github/workflows/release.yml` | PASS (exit 0) |
| AC-001 keep (i): resolver wasm present | `test -f plugins/vsdd-factory/hook-plugins/vsdd-context-resolvers.wasm` | PASS (exit 0) |
| AC-001 keep (ii): registry ref intact | `grep -q "hook-plugins/vsdd-context-resolvers.wasm" plugins/vsdd-factory/resolvers-registry.toml` | PASS (exit 0) |
| AC-002: orphans absent (git ls-files) | `git ls-files plugins/vsdd-factory/hook-plugins/` — no underscore orphans | PASS |
| AC-003: preamble markers | `grep -q "regex SEARCH\|fullmatch\|anchoring" plugins/vsdd-factory/hooks-registry.toml` | PASS (exit 0) |
| AC-004: both-ends gate (live registry) | zero-output subshell on live hooks-registry.toml | PASS (empty) |
| AC-004: prefix-only-anchor negative fixture | `tool = "^Bash"` flagged as violation | PASS (flagged) |
| AC-004: comment-inject negative fixture | `tool = "^Edit" # note "$"` flagged | PASS (flagged) |
| AC-005: bats suite | `bats plugins/vsdd-factory/tests/registry-tool-filter-anchoring.bats` | PASS (7/7) |
| AC-006: cargo bundle orphan | `cargo test --test bundle_orphan_check` | PASS (5/5) |
| AC-007: staging simulation T-010 | T-010 in bundle_orphan_check | PASS (exit 0) |

---

## Architecture Compliance Verification

| Rule | Status |
|------|--------|
| Orphan detection checks BOTH hooks-registry + resolvers-registry | PASS — T-008 negative-control confirms dual-registry check is load-bearing |
| `vsdd-context-resolvers.wasm` (hyphen) remains in bundle | PASS — AC-001 keep-assertions + AC-002 + T-009 hermetic gate |
| Both-ends anchoring: `tool =` entries have leading `^` AND trailing `$` | PASS — AC-004 zero-violation gate + AC-005 T-004 |
| D-f singleton patterns applied: `^Bash$`, `^Read$`, `^Agent$` | PASS — AC-004 gate + AC-005 T-004 live registry check |
| No new `.sh` scripts | PASS — orphan detection implemented as Rust cargo test; no new bash scripts |
| POLICY 20 (release_bundle_no_dev_samples) registered | PASS — present in `plugins/vsdd-factory/config/policies.yaml` |
| No AI attribution in commits | PASS — no Co-Authored-By: Claude |

---

## Closures

- **rc.22 smoke finding (a):** CLOSED — 3 orphan WASMs removed from tracked set
  (`hello-hook.wasm` via BUILD-OMISSION in release.yml; `vsdd_context_resolvers.wasm`
  + `wasm_resolver_export.wasm` via deletion from tracked tree + `*_*.wasm` underscore-glob
  exclusion in release.yml). T-009 hermetic gate prevents re-regression.
- **rc.22 smoke finding (b):** CLOSED — all `tool =` entries now use both-ends
  anchored patterns per D-a table. Bats lint suite (T-001..T-005, T-011, T-012)
  enforces the convention; CI will block unanchored additions (EC-002).

---

## Convergence Status

S-19.04 meets all 7 acceptance criteria. LOCAL cascade converged 3/3 per BC-5.39.001
3-CLEAN protocol (pass 16 was the final clean pass). 5 cargo integration tests + 7 bats
lint tests + 200 workspace tests green. rc.22 smoke findings (a) and (b) closed.
