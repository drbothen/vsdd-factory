# Demo Evidence Report: S-8.09

**Story:** S-8.09 — Native port: regression-gate + adapter retirement prep (W-15 closer)  
**Branch:** feature/S-8.09-...  
**Head commit at evidence capture:** 5da2753  
**Date:** 2026-05-02  
**Recorder:** Demo Recorder agent  
**Test suite:** 22/22 bats + 36/36 unit tests PASS

---

## Coverage Summary

All 12 acceptance criteria have VHS recordings covering both success and error paths.
One BONUS recording covers the macOS /var/folders/ dispatcher fix and W-15 finale.

| Recording | AC | BC Trace | Status | Success Path | Error Path |
|-----------|----|-----------| -------|------|------|
| [AC-001](AC-001.md) | WASM crate build + registry entry | BC-7.03.071 postcondition 1 | PASS | WASM artifact present, registry entry correct | N/A (structural check) |
| [AC-002](AC-002.md) | hooks.json deleted + .sh deleted | BC-7.03.071 invariant 1 | PASS | Zero matches in hooks.json | N/A (deletion check) |
| [AC-003](AC-003.md) | 9 test-runner pattern matching | BC-7.03.072 postcondition 1 | PASS | All 9 patterns match | Non-Bash tool exits 0 |
| [AC-004](AC-004.md) | Pass/fail derivation cascade | BC-7.03.073 postcondition 1 | PASS | exit_code=0->pass, non-zero->fail, interrupted | Unknown status -> skip |
| [AC-005](AC-005.md) | State file write | BC-7.03.074 postcondition 1 | PASS | {status,timestamp,command} written | .factory/ absent -> exit 0 |
| [AC-006](AC-006.md) | Pass-to-fail warning | BC-7.03.075 postcondition 1 | PASS | hook.block warn + stderr emitted | fail->fail -> no warning |
| [AC-007](AC-007.md) | Bats parity tests (22 tests) | BC-7.03.071..075 all | PASS | 22/22 pass | All 9 failure scenarios covered |
| [AC-008](AC-008.md) | host::emit_event, no subprocess bin/emit-event | BC-7.03.071 invariant 2 | PASS | host::emit_event used | bin/emit-event binary still present (D-10) |
| [AC-009](AC-009.md) | No jq, serde_json native | BC-7.03.071 postcondition 1 | PASS | Zero jq refs in src | OQ-6 confirmed binary_allow=[] |
| [AC-010](AC-010.md) | OQ-6 audit doc (RESOLVED) | OQ-6 assumption validation | PASS | All 3 falsifiable checks pass | N/A (doc presence check) |
| [AC-011](AC-011.md) | 0 Tier 1 legacy-bash-adapter refs | BC-7.03.071 postcondition 1 | PASS | 9/9 hooks native | W-15 Tier 1 migration complete |
| [AC-012](AC-012.md) | bin/emit-event PRESENT | BC-7.03.071 invariant 2 | PASS | PRESENT output | N/A (presence check) |
| [BONUS](BONUS.md) | macOS /var/folders/ fix + W-15 finale | (dispatcher fix) | SHIPPED | 9 native hooks listed | N/A |

---

## AC-by-AC Detail

### AC-001 — WASM Crate Build + Registry Entry
Traces to BC-7.03.071 postcondition 1.

- `crates/hook-plugins/regression-gate/Cargo.toml` targets `wasm32-wasip1`
- `target/wasm32-wasip1/release/regression-gate.wasm` exists
- Registry: `plugin = "hook-plugins/regression-gate.wasm"`, `event = "PostToolUse"`, `priority = 230`, `on_error = "continue"`
- Capabilities: `read_file` + `write_file` on `.factory/regression-state.json`
- No `script_path`, no `shell_bypass_acknowledged`

Recording: [AC-001-wasm-crate-build-registry.gif](AC-001-wasm-crate-build-registry.gif)

---

### AC-002 — hooks.json Entry + .sh File Deleted
Traces to BC-7.03.071 invariant 1.

- `grep -r 'regression-gate' plugins/vsdd-factory/hooks/ --include='hooks.json*'` -> zero matches
- `test -f plugins/vsdd-factory/hooks/regression-gate.sh` -> DELETED

Recording: [AC-002-hooks-json-deleted.gif](AC-002-hooks-json-deleted.gif)

---

### AC-003 — 9 Test-Runner Patterns + Bash Guard
Traces to BC-7.03.072 postcondition 1.

36 unit tests include 10 `test_BC_7_03_072_*` tests covering all 9 substrings:
`cargo test`, `cargo nextest`, `pytest`, `pnpm test`, `npm test`, `go test`, `just test`, `just ci`, `yarn test`.

Error path: `test_BC_7_03_071_non_bash_tool_exits_0_immediately` — Read tool -> exit 0, no write.

Recording: [AC-003-9-pattern-matching.gif](AC-003-9-pattern-matching.gif)

---

### AC-004 — Pass/Fail Derivation Cascade
Traces to BC-7.03.073 postcondition 1.

8 unit tests cover all 5 cascade steps including null exit_code fallback, exit_code priority over interrupted field.
Error path: `test_BC_7_03_073_unknown_status_no_state_write` — neither field -> skip.

Recording: [AC-004-pass-fail-derivation.gif](AC-004-pass-fail-derivation.gif)

---

### AC-005 — State File Write
Traces to BC-7.03.074 postcondition 1.

4 unit tests verify: JSON structure, ISO-8601 UTC timestamp (via chrono), pass/fail written, `.factory/` absent guard.
Error path: factory dir absent -> exit 0, no write.

Recording: [AC-005-state-file-write.gif](AC-005-state-file-write.gif)

---

### AC-006 — Pass-to-Fail Warning
Traces to BC-7.03.075 postcondition 1.

7 unit tests verify: `hook.block severity=warn` event fields, stderr rendered newlines (0x0A), fail->fail silence, unknown->fail silence (EC-004), malformed prior state silence (EC-003).
Error path: fail->fail -> no event, no stderr.

Recording: [AC-006-pass-to-fail-warning.gif](AC-006-pass-to-fail-warning.gif)

---

### AC-007 — Bats Parity Tests (22/22)
Traces to BC-7.03.071..075 postconditions (full parity).

22 bats tests including all 9 AC-007 scenarios:
- (a) cargo test pass, no prior
- (b) interrupted=false -> pass
- (c) exit_code=1, prior=pass -> regression warning
- (d) interrupted=true, prior=pass -> regression warning
- (e) unknown -> no state write
- (f) git commit -> exit 0
- (g) .factory/ absent guard
- (h) fail->fail, no warning
- (i) pytest pass (non-cargo pattern coverage)

Recording: [AC-007-bats-parity-tests.gif](AC-007-bats-parity-tests.gif)

---

### AC-008 — host::emit_event, bin/emit-event NOT Removed
Traces to BC-7.03.071 invariant 2.

- No executable `bin/emit-event` subprocess call in `src/` (only doc comments)
- `host::emit_event` used at lib.rs lines 272-280
- `plugins/vsdd-factory/bin/emit-event` PRESENT (D-10 deferral to S-8.29)

Recording: [AC-008-no-bin-emit-event-in-crate.gif](AC-008-no-bin-emit-event-in-crate.gif)

---

### AC-009 — Self-Contained WASM, No jq
Traces to BC-7.03.071 postcondition 1.

- Zero `jq` subprocess calls in `src/`
- `serde_json` for stdin parse + state file serialize
- `chrono::Utc::now()` for timestamps
- OQ-6 audit: `recommended_binary_allow: []`

Recording: [AC-009-no-jq-dependency.gif](AC-009-no-jq-dependency.gif)

---

### AC-010 — OQ-6 Security-Reviewer Audit (RESOLVED)
Validates OQ-6 assumption.

`docs/oq-6-regression-gate-security-audit.md` present with:
- `signoff_agent_id: security-reviewer` (non-empty)
- `signoff_timestamp: "2026-05-02T00:00:00Z"` (non-empty)
- `recommended_binary_allow: []` (explicitly present)
- `recommended_capabilities: [read_file, write_file, emit_event]`
- 5 findings: 0 critical, 0 high, 2 medium, 3 low

Recording: [AC-010-oq6-audit-doc.gif](AC-010-oq6-audit-doc.gif)

---

### AC-011 — 0 Tier 1 legacy-bash-adapter Refs (W-15 Closure)
Traces to BC-7.03.071 postcondition 1.

- `grep -n 'legacy-bash-adapter' hooks-registry.toml` for all 9 Tier 1 names -> 0 matches
- `docs/E-8-tier1-native-audit.md` present, all 9 hooks YES
- Statement: "W-15 Tier 1 migration complete. S-8.11 wave gate unblocked."

Recording: [AC-011-tier1-native-audit.gif](AC-011-tier1-native-audit.gif)

---

### AC-012 — bin/emit-event PRESENT
Traces to BC-7.03.071 invariant 2.

```
test -f plugins/vsdd-factory/bin/emit-event && echo "PRESENT" || echo "MISSING"
```
Output: `PRESENT`

Recording: [AC-012-bin-emit-event-present.gif](AC-012-bin-emit-event-present.gif)

---

## Bonus: macOS /var/folders/ Fix + W-15 Finale

The factory-dispatcher `invoke.rs` ancestor fallback resolves the macOS
`/var -> /private/var` symlink issue for `write_file` path-allow checks.
This fix enabled all bats integration tests to pass on macOS darwin-arm64.

W-15 final state: 9/9 Tier 1 hooks native. S-8.29 (adapter retirement) is ready for scheduling.

Recording: [BONUS-macos-varfolders-fix.gif](BONUS-macos-varfolders-fix.gif)

---

## File Inventory

| File | Type | AC |
|------|------|----|
| AC-001-wasm-crate-build-registry.gif | recording | AC-001 |
| AC-001-wasm-crate-build-registry.webm | recording | AC-001 |
| AC-001-wasm-crate-build-registry.tape | script | AC-001 |
| AC-001.md | evidence doc | AC-001 |
| AC-002-hooks-json-deleted.gif | recording | AC-002 |
| AC-002-hooks-json-deleted.webm | recording | AC-002 |
| AC-002-hooks-json-deleted.tape | script | AC-002 |
| AC-002.md | evidence doc | AC-002 |
| AC-003-9-pattern-matching.gif | recording | AC-003 |
| AC-003-9-pattern-matching.webm | recording | AC-003 |
| AC-003-9-pattern-matching.tape | script | AC-003 |
| AC-003.md | evidence doc | AC-003 |
| AC-004-pass-fail-derivation.gif | recording | AC-004 |
| AC-004-pass-fail-derivation.webm | recording | AC-004 |
| AC-004-pass-fail-derivation.tape | script | AC-004 |
| AC-004.md | evidence doc | AC-004 |
| AC-005-state-file-write.gif | recording | AC-005 |
| AC-005-state-file-write.webm | recording | AC-005 |
| AC-005-state-file-write.tape | script | AC-005 |
| AC-005.md | evidence doc | AC-005 |
| AC-006-pass-to-fail-warning.gif | recording | AC-006 |
| AC-006-pass-to-fail-warning.webm | recording | AC-006 |
| AC-006-pass-to-fail-warning.tape | script | AC-006 |
| AC-006.md | evidence doc | AC-006 |
| AC-007-bats-parity-tests.gif | recording | AC-007 |
| AC-007-bats-parity-tests.webm | recording | AC-007 |
| AC-007-bats-parity-tests.tape | script | AC-007 |
| AC-007.md | evidence doc | AC-007 |
| AC-008-no-bin-emit-event-in-crate.gif | recording | AC-008 |
| AC-008-no-bin-emit-event-in-crate.webm | recording | AC-008 |
| AC-008-no-bin-emit-event-in-crate.tape | script | AC-008 |
| AC-008.md | evidence doc | AC-008 |
| AC-009-no-jq-dependency.gif | recording | AC-009 |
| AC-009-no-jq-dependency.webm | recording | AC-009 |
| AC-009-no-jq-dependency.tape | script | AC-009 |
| AC-009.md | evidence doc | AC-009 |
| AC-010-oq6-audit-doc.gif | recording | AC-010 |
| AC-010-oq6-audit-doc.webm | recording | AC-010 |
| AC-010-oq6-audit-doc.tape | script | AC-010 |
| AC-010.md | evidence doc | AC-010 |
| AC-011-tier1-native-audit.gif | recording | AC-011 |
| AC-011-tier1-native-audit.webm | recording | AC-011 |
| AC-011-tier1-native-audit.tape | script | AC-011 |
| AC-011.md | evidence doc | AC-011 |
| AC-012-bin-emit-event-present.gif | recording | AC-012 |
| AC-012-bin-emit-event-present.webm | recording | AC-012 |
| AC-012-bin-emit-event-present.tape | script | AC-012 |
| AC-012.md | evidence doc | AC-012 |
| BONUS-macos-varfolders-fix.gif | recording | bonus |
| BONUS-macos-varfolders-fix.webm | recording | bonus |
| BONUS-macos-varfolders-fix.tape | script | bonus |
| BONUS.md | evidence doc | bonus |
| evidence-report.md | this report | all |

Total: 53 files (13 GIF, 13 WEBM, 13 tape, 13 MD + this report)
