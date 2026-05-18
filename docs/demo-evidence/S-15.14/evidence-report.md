# S-15.14 Demo Evidence Report
# validate-dispatch-advance WASM hook — per-AC demo evidence

**Story:** S-15.14 v1.2  
**BC anchor:** BC-5.39.006 v1.3  
**Hook:** `crates/hook-plugins/validate-dispatch-advance/` → `plugins/vsdd-factory/hook-plugins/validate-dispatch-advance.wasm`  
**Evidence recorded:** 2026-05-18  
**Evidence form:** Literal bats test invocation + captured TAP output (per D-449(a) literal-shell-execution-evidence discipline). Cargo commands for ACs requiring compilation and unit-test execution.  
**Total ACs:** 22  
**ACs with evidence:** 22  
**ACs surfaced as untestable:** 0  

---

## Coverage Map

| AC | Evidence File | What It Demonstrates | Status |
|----|--------------|----------------------|--------|
| AC-1 | AC-01-meta-commentary-watch.txt | `META-LEVEL-N WATCH` pattern in `current_step:` → BlockWithFix citing D-440(a)+D-441(a)+D-442(a) | PASS |
| AC-2 | AC-02-meta-commentary-self-app.txt | `self-app TEST` substring in `current_step:` → BlockWithFix | PASS |
| AC-3 | AC-03-meta-commentary-expected-verdict.txt | `expected verdict` substring in `current_step:` → BlockWithFix | PASS |
| AC-4 | AC-04-missing-arch-index-cite.txt | ARCH-INDEX v absent from `current_step:` → BlockWithFix naming missing cite, cites D-439(b) | PASS |
| AC-5 | AC-05-tail-3-components.txt | trajectory-tail LENGTH=3 (marker present) → BlockWithFix naming actual=3, required=4, D-451(c) | PASS |
| AC-6 | AC-06-tail-5-components.txt | trajectory-tail LENGTH=5 (marker present) → BlockWithFix naming actual=5, required=4 | PASS |
| AC-7 | AC-07-stale-d-chain-absent.txt | No `D-\d+` reference in `current_step:` → BlockWithFix citing D-443(a) | PASS |
| AC-8 | AC-08-stale-d-chain-terminal.txt | `D-476` in `current_step:` (max_cited=476) while STATE.md body has D-477 (max_in_file=477) → BlockWithFix citing stale D-443(a) | PASS |
| AC-9 | AC-09-all-valid-state-pass.txt | All conditions satisfied → Continue (no block) | PASS |
| AC-10 | AC-10-all-four-violations.txt | All 4 STATE.md violations simultaneously → single BlockWithFix enumerating all 4 classes | PASS |
| AC-11 | AC-11-index-5-col-row.txt | INDEX.md adversary-pass row with 4 columns (5 pipes) in 5-col-header section → BlockWithFix naming actual=4, required=5, D-441(b) | PASS |
| AC-12 | AC-12-index-7-col-row.txt | INDEX.md adversary-pass row with 6 columns (7 pipes) in 5-col-header section → BlockWithFix naming actual=6, required=5 | PASS |
| AC-13 | AC-13-index-6-col-pass.txt | All INDEX.md adversary-pass rows exactly 5 columns in 5-col-header section → Continue | PASS |
| AC-14 | AC-14-fail-open-state-unreadable.txt | STATE.md unreadable via host::read_file → Continue (fail-open); no block | PASS |
| AC-15 | AC-15-fail-open-index-unreadable.txt | INDEX.md unreadable via host::read_file → Continue (fail-open); no block | PASS |
| AC-16 | AC-16-path-guard-xstate.txt | Path xSTATE.md → is_state_md_target() returns false → Continue (path-component-strict) | PASS |
| AC-17 | AC-17-path-guard-xindex.txt | Path xINDEX.md → is_index_md_target() returns false → Continue (path-component-strict) | PASS |
| AC-18 | AC-18-wasm-compilation.txt | `cargo build --release --target wasm32-wasip1 -p validate-dispatch-advance` exits 0; binary at expected path (177763 bytes) | PASS |
| AC-19 | AC-19-registry-entry.txt | hooks-registry.toml has `event = "PostToolUse"`, `tool = "Edit\|Write"`; no `file_pattern` field; path-component-strict guards present in production lib.rs | PASS |
| AC-20 | AC-20-em-dash-utf8-boundary.txt | `validate_current_step_with_em_dash` unit test passes; em-dash (U+2014) adjacent to digit does not panic; is_char_boundary() guards effective | PASS |
| AC-21 | AC-21-preflight-4-gates.txt | All 4 pre-flight gates pass: `cargo fmt --check`, `cargo clippy -D warnings`, `cargo test --workspace`, `bats run-all.sh` (31/31 tests pass) | PASS |
| AC-22 | AC-22-missing-trajectory-tail-marker.txt | `trajectory-tail ` marker absent from `current_step:` → BlockWithFix citing D-451(c)/F-P3-006/EC-023; LENGTH count does NOT run | PASS |

---

## Evidence Form Notes

- **ACs 1–17 and AC-22:** Bats integration tests running the real compiled WASM binary
  (`validate-dispatch-advance.wasm`) via the real dispatcher (`factory-dispatcher`) with
  fixture STATE.md/INDEX.md files placed at the path the dispatcher reads. Each test
  asserts dispatcher exit code + output patterns. Evidence captured via literal
  `bats --tap <file>` invocation per D-449(a).

- **AC-18:** `cargo build --release --target wasm32-wasip1` confirms zero-warning WASM
  compilation. Binary existence confirmed via `ls -la`.

- **AC-19:** `grep` against production `hooks-registry.toml` confirms canonical
  `tool = "Edit|Write"` (not `"Write|Edit"`), `event = "PostToolUse"`, no `file_pattern`.
  Separate `grep` against `src/lib.rs` confirms `is_state_md_target`/`is_index_md_target`
  guards are in production code (not just test code).

- **AC-20:** `cargo test` running the `validate_current_step_with_em_dash` unit test,
  which directly exercises the byte-boundary guard on multi-byte UTF-8 input.

- **AC-21:** Combined output of all four pre-flight gate commands, each returning exit 0.
  Full bats suite output included (31/31 tests passing across validate-dispatch-advance
  test directory plus integration-production-registry).

---

## Bats Suite Full Run Summary

All 31 test cases in `plugins/vsdd-factory/tests/validate-dispatch-advance/` pass:

```
1..31
ok 1  AC-1  FAIL: hook blocks when current_step: contains META-LEVEL-5 WATCH pattern
ok 2  AC-1  FAIL: block message names META-LEVEL WATCH pattern and cites D-440
ok 3  AC-2  FAIL: hook blocks when current_step: contains 'self-app TEST' substring
ok 4  AC-3  FAIL: hook blocks when current_step: contains 'expected verdict' substring
ok 5  AC-4  FAIL: hook blocks when current_step: is missing ARCH-INDEX v cite
ok 6  AC-4  FAIL: block message names ARCH-INDEX as missing cite and cites D-439
ok 7  AC-5  FAIL: hook blocks when trajectory-tail has 3 arrow-N groups (not 4)
ok 8  AC-5  FAIL: block message names 3 components and required LENGTH=4 citing D-451
ok 9  AC-6  FAIL: hook blocks when trajectory-tail has 5 arrow-N groups (not 4)
ok 10 AC-6  FAIL: block message names actual count 5 and required 4
ok 11 AC-7  FAIL: hook blocks when current_step: has no D-382..D-NNN pattern
ok 12 AC-7  FAIL: block message for absent D-chain cites D-443
ok 13 AC-8  FAIL: hook blocks when current_step: D-chain terminal D-476 is below latest D-477
ok 14 AC-8  FAIL: block message names stale D-476 and latest D-477
ok 15 AC-9  PASS: hook emits Continue when current_step: has no violations (all conditions satisfied)
ok 16 AC-10 FAIL: hook blocks with single consolidated message when all 4 STATE.md violations present
ok 17 AC-10 FAIL: single block message covers meta-commentary, missing index cite, tail, and D-chain violations
ok 18 AC-11 FAIL: hook blocks when INDEX.md adversary-pass row has 4 columns (5 pipe chars)
ok 19 AC-11 FAIL: block message names actual=4 required=5 and cites D-441
ok 20 AC-12 FAIL: hook blocks when INDEX.md adversary-pass row has 6 columns (7 pipe chars)
ok 21 AC-12 FAIL: block message names actual=6 and required=5
ok 22 AC-13 PASS: hook emits Continue when all INDEX.md adversary-pass rows are exactly 6 columns
ok 23 AC-14 PASS (fail-open): hook emits Continue when STATE.md is not readable via host::read_file
ok 24 AC-15 PASS (fail-open): hook emits Continue when INDEX.md is not readable via host::read_file
ok 25 AC-16 PASS: path-component-strict guard passes when file is xSTATE.md (not STATE.md)
ok 26 AC-17 PASS: path-component-strict guard passes when file is xINDEX.md (not INDEX.md)
ok 27 AC-22 FAIL: hook blocks when trajectory-tail prefix absent from current_step
ok 28 AC-22 FAIL: block message names missing trajectory-tail canonical marker and cites F-P3-006/EC-023
ok 29 AC-22 PASS: hook does not block when trajectory-tail prefix present with LENGTH=4
ok 30 PROD-REGISTRY: hook emits Continue for valid STATE.md using production path_allow entry
ok 31 PROD-REGISTRY: hook blocks for invalid STATE.md using production path_allow entry (not fail-open)
```

31 tests, 0 skipped, 0 failed.
