---
document_type: adversary-pass
cycle: v1.0-feature-plugin-async-semantics-pass-1
phase: F2
pass: 6
producer: adversary
producer_model: claude-opus-4-7[1m]
fresh_context: true
verdict: SUBSTANTIVE
finding_count: { high: 2, medium: 2, low: 1, nit: 0 }
adr-013_clock_action: reset
clock: 0_of_3
timestamp: 2026-05-07T00:00:00Z
---

# Adversary Pass-6 Findings — F2 spec package

## Verdict
**SUBSTANTIVE.** Trajectory 19→19→7→6→3→5. Findings concentrated as harness defects (partial-fix regressions of F-P2-008 and F-P4-005). Clock RESETS to 0/3.

## Counts
HIGH: 2, MEDIUM: 2, LOW: 1, NIT: 0

## Findings

### F-P6-001 [HIGH] — VP-078 + VP-079 fixtures use plural `events = [...]` but registry schema is singular `event = "..."`

**Evidence:**
- Production hooks-registry.toml lines 17, 31, 39, 47, 55, 63: `event = "SessionStart"` (singular scalar string)
- registry.rs line 149: `pub event: String` (singular, not Vec<String>)
- VP-078 fixtures use plural at lines 157, 195, 216, 234, 326, 345, 363, 387
- VP-079 fixtures use plural at lines 197, 208, 245, 291, 349, 361, 410, 422

**Impact:** Every fixture fails registry deserialization. Test-writer cannot instantiate proof harnesses. Same defect class F-P2-008 fix burst claimed to close.

**Partial-fix regression:** F-P2-008 fixed `script` → `plugin` + `[hooks.config] script_path` but did NOT catch `events` field name. Per S-7.01 partial-fix discipline. Blast-radius = 2 files.

**Fix:** Mass-replace `events = ["X"]` → `event = "X"` in VP-078.md and VP-079.md fixture blocks (16 sites total). Pure schema alignment; no test-content changes.

### F-P6-002 [HIGH] — VP-078 Rust unit tests still use `script = "..."` form (broken since F-P2-008)

**Evidence:** VP-078.md lines 192, 214, 231, 320, 345, 363, 387: `script = "X.sh"` in Rust unit tests for Harness 4.

The bats Harness 2 (lines 145-159) was migrated to correct form by F-P2-008. The Rust unit tests at lines 180-397 were missed in that burst and remain broken.

**Impact:** All Harness 4 Rust unit tests fail at TOML parse time with "missing field `plugin`". Blocks parse-time-enforcement layer of three-layer defense in depth.

**Fix:** Replace each `script = "X.sh"` with `plugin = "hook-plugins/legacy-bash-adapter.wasm"` + `[hooks.config]\nscript_path = "X.sh"` block, in all 6+ Rust unit tests.

### F-P6-003 [MEDIUM] — BC-3.08.001 still inlines `ASYNC_DRAIN_WINDOW_MS = 100 ms` literal in live Traceability

**Evidence:** BC-3.08.001.md line 196 (Traceability table, L2 Domain Invariants cell):

```
DI-019 — `ASYNC_DRAIN_WINDOW_MS = 100 ms` (the `plugin.timeout` async path …)
```

BC-1.14.001 v1.5 (F-P4-005) explicitly removed this pattern. Same fix not propagated to BC-3.08.001 even though BC-3.08.001 v1.2 was the DI-019 cross-reference addition burst.

**Impact:** Violates §Constant Reference rule. Sibling-fix gap. Per S-7.01.

**Fix:** Replace `ASYNC_DRAIN_WINDOW_MS = 100 ms` with `ASYNC_DRAIN_WINDOW_MS (per DI-019)`.

### F-P6-004 [MEDIUM] — VP-078 bats Harness 2 fixture has TOML table-ordering hazard

**Evidence:** VP-078.md lines 145-159:

```toml
[[hooks]]
name = "violating-plugin"
plugin = "hook-plugins/legacy-bash-adapter.wasm"

[hooks.config]
script_path = "test-fixtures/exit0.sh"

on_error = "block"
async = true
events = ["PostToolUse"]
priority = 400
```

After `[hooks.config]` sub-table header, subsequent flat assignments belong to `hooks.config`. So `on_error`, `async`, `events`, `priority` parse as `hooks.config.*`, NOT as `[[hooks]].*`. Violation-detection invisible to parser.

**Impact:** Even if F-P6-001 is fixed, the harness reports `[[hooks]]` has no `on_error`/`async` and passes trivially (false negative on the violation it exists to catch).

**Fix:** Reorder fixture so top-level fields precede `[hooks.config]` sub-table:

```toml
[[hooks]]
name = "violating-plugin"
plugin = "hook-plugins/legacy-bash-adapter.wasm"
on_error = "block"
async = true
event = "PostToolUse"
priority = 400

[hooks.config]
script_path = "test-fixtures/exit0.sh"
```

VP-079 fixtures verified correct ordering (top-level before sub-table). Single-file scope.

### F-P6-005 [LOW] — ADR-019 §Consequences keeps inline `100ms` parenthetical (intent unclear)

**Evidence:** ADR-019.md line 200:
```
…(governed by DI-019; default 100ms) for async tasks to emit terminal events…
```

BC-1.14.001 v1.5 (F-P4-005) removed parenthetical literals. ADR-019 §Consequences subsection retains the pattern.

**Pending intent verification:** ADR may legitimately preserve as reading-aid; or sibling-fix gap. Adjudication needed.

**Fix (if intent is consistency):** `(governed by DI-019; default 100ms)` → `(governed by DI-019)`. If literal is intentional, document explicitly.

## Policy compliance

| Policy | Status |
|---|---|
| 1 | PASS |
| 2 | PASS |
| 3 | N/A |
| 4 | PASS |
| 5 | PASS |
| 6 | PASS |
| 7 | PASS — sampled 9 BCs byte-for-byte |
| 8 | N/A |
| 9 | PASS — VP rows match frontmatter |
| 10 | N/A |
| 11 | PASS |
| 12 | **AT RISK** via F-P6-001/002/004 — fixtures structurally broken |

## Top 3

1. **F-P6-001 (HIGH)** — 16 fixture sites use plural `events`, schema is singular `event`
2. **F-P6-002 (HIGH)** — VP-078 Rust unit tests use `script` form, pre-F-P2-008
3. **F-P6-004 (MEDIUM)** — VP-078 bats Harness 2 TOML table-ordering hazard
