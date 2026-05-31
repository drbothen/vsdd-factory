---
document_type: demo-evidence-report
product: "validate-trajectory-tail-cell-completeness WASM hook"
story_id: "S-15.17"
pipeline_run: "2026-05-31"
demo_type: "cli"
recording_tool: "vhs + bats-transcript"
status: complete
---

# Demo Evidence Report — S-15.17

## Product: validate-trajectory-tail-cell-completeness WASM hook
## Story: S-15.17 — per-cell runtime gate for D-453(d) prescribed sites
## Pipeline Run: 2026-05-31
## Demo Type: WASM PostToolUse hook (CLI / terminal)
## Recording Tool: VHS (headline cases) + bats transcript captures (all ACs)

---

## Summary

The `validate-trajectory-tail-cell-completeness` WASM hook enforces trajectory-tail presence
across 9 D-453(d) prescribed sites. The v1.9 cycle-conditional model (ADR-023 Option (c)) splits
the 5 STATE.md sites into:

- **PC1 + PC2 (current_step + Last Updated): ALWAYS Block** — cycle-invariant
- **PC3 + PC4 + PC5 (Phase Progress + Concurrent Cycles + Session Resume §1): CYCLE-CONDITIONAL** —
  Block only when active cycle `per_pass_trajectory: true`; advisory-only in milestone cycles;
  fail-open-to-advisory when the flag cannot be read

**Bats suite: 67 tests in 32 files — all 67 PASS (1..67, exit 0).**
**Cargo unit tests: 63 tests — all PASS (exit 0).**
**WASM build (wasm32-wasip1): Finished `release` profile, exit 0.**
**cargo fmt --check: PASS. cargo clippy -D warnings: PASS.**

---

## Per-AC Demo Recordings

| AC | Description | Recording / Artifact | Tool | Status | Notes |
|----|-------------|---------------------|------|--------|-------|
| AC-1 | Registry entry at priority 158; PostToolUse; `Edit\|Write` | [AC-1-registry-priority-158.txt](AC-1-registry-priority-158.txt) | bats transcript | PASS | `integration-production-registry.bats` — 3 tests pass (priority=158, valid STATE.md → Continue, invalid STATE.md → Block) |
| AC-2 | **ALWAYS-Block (PC1):** current_step: missing trajectory-tail → Block (cycle-invariant) | [AC-2-3-always-block.gif](AC-2-3-always-block.gif) / [.webm](AC-2-3-always-block.webm) | VHS | PASS | VHS tape: `fail-state-frontmatter-missing-tail.bats` + `fail-state-last-updated-missing-tail.bats` → 4 tests pass (exit 2, block_reason names site) |
| AC-3 | **ALWAYS-Block (PC2):** Last Updated cell missing trajectory-tail → Block (cycle-invariant) | [AC-2-3-always-block.gif](AC-2-3-always-block.gif) / [AC-2-3-always-block.txt](AC-2-3-always-block.txt) | VHS + bats transcript | PASS | Same recording as AC-2 (combined tape); `fail-state-last-updated-missing-tail.bats` → exit 2, block_reason names "Last Updated cell" |
| AC-4 | **CYCLE-CONDITIONAL Block (PC3):** F5-per-pass cycle, Phase Progress missing → Block | [AC-4-5-6-f5-per-pass-block.gif](AC-4-5-6-f5-per-pass-block.gif) / [.webm](AC-4-5-6-f5-per-pass-block.webm) | VHS | PASS | `fail-state-f5-per-pass-block.bats` → exit 2, cascade names Phase Progress site |
| AC-5 | **CYCLE-CONDITIONAL Block (PC4):** F5-per-pass cycle, Concurrent Cycles missing → Block | [AC-4-5-6-f5-per-pass-block.gif](AC-4-5-6-f5-per-pass-block.gif) / [AC-4-5-6-f5-per-pass-block.txt](AC-4-5-6-f5-per-pass-block.txt) | VHS + bats transcript | PASS | Same recording as AC-4; cascade names both Phase Progress and Concurrent Cycles |
| AC-6 | **CYCLE-CONDITIONAL Block (PC5):** F5-per-pass cycle, Session Resume §1 missing → Block | [AC-4-5-6-f5-per-pass-block.gif](AC-4-5-6-f5-per-pass-block.gif) | VHS | PASS | Same recording; cascade names Session Resume site |
| AC-7 | Multiple missing Block-routed sites → single cascade Block (PC6; invariant 8) | [bats-suite-full.txt](bats-suite-full.txt) | bats transcript | PASS | `fail-state-cascade-missing-sites.bats` — 5 tests: exit 2, cascade names all 3 sites, invariant 8 single-block assertion |
| AC-8 | All 5 STATE.md sites present → Continue (PC12, pass-clean) | [AC-8-pass-clean.txt](AC-8-pass-clean.txt) | bats transcript | PASS | `pass-state-all-sites-present.bats` → 2 tests pass (exit 0, no block signal) |
| AC-9 | INDEX.md Convergence Status row missing → advisory + Continue (PC7; invariant 6) | [bats-suite-full.txt](bats-suite-full.txt) | bats transcript | PASS | `fail-index-convergence-status-missing-tail.bats` → 3 tests pass (exit 0, no block, advisory fired) |
| AC-10 | INDEX.md adv-table row missing → advisory + Continue (PC8; invariant 6) | [bats-suite-full.txt](bats-suite-full.txt) | bats transcript | PASS | `fail-index-adv-table-missing-tail.bats` → 3 tests pass |
| AC-11 | burst-log.md Dim-7 missing → advisory + Continue (PC9; invariant 6) | [bats-suite-full.txt](bats-suite-full.txt) | bats transcript | PASS | `fail-burst-log-dim7-missing-tail.bats` → 2 tests pass (exit 0, no block) |
| AC-12 | lessons.md arm always → advisory + Continue regardless of content (PC10 OUT-OF-SCOPE) | [bats-suite-full.txt](bats-suite-full.txt) | bats transcript | PASS | `fail-lessons-trend-table-missing-tail.bats` → 2 tests pass (exit 0, no block per PC10 advisory-only) |
| AC-13 | INDEX.md, burst-log.md, lessons.md all sites present → Continue (PC12) | [bats-suite-full.txt](bats-suite-full.txt) | bats transcript | PASS | `pass-index-all-sites-present.bats`, `pass-burst-log-dim7-present.bats`, `pass-lessons-trend-table-present.bats` → all exit 0 |
| AC-14 | `HostError::OutputTooLarge` → Continue + log_warn fail-open (PC11; invariant 7) | [bats-suite-full.txt](bats-suite-full.txt) | bats transcript | PASS | `pass-file-too-large-failopen.bats` → 3 tests (exit 0, never blocks, advisory emitted) |
| AC-15 | Other HostError variants → Continue + log_warn fail-open (PC11; invariant 10) | [bats-suite-full.txt](bats-suite-full.txt) | bats transcript | PASS | `pass-read-failure-failopen.bats` → 2 tests pass (exit 0, never blocks) |
| AC-16 | Path-component-strict: wrong basename → Continue (invariant 3) | [bats-suite-full.txt](bats-suite-full.txt) | bats transcript | PASS | `pass-wrong-filename-no-trigger.bats` → 2 tests pass |
| AC-17 | All 9 prescribed sites exercised with positive + negative fixtures | [bats-suite-full.txt](bats-suite-full.txt) | bats transcript | PASS | 32 bats files in suite covering all 9 sites with both pass and fail fixtures (67 tests total) |
| AC-18 | Hook compiles to `wasm32-wasip1` cleanly with zero warnings | [wasm-build-output.txt](wasm-build-output.txt) | cargo output | PASS | `cargo build --release --target wasm32-wasip1 -p validate-trajectory-tail-cell-completeness` → `Finished \`release\` profile [optimized] target(s) in 0.21s` (exit 0) |
| AC-19 | Pre-flight 4-gate passes (fmt + clippy + cargo test + bats) | [cargo-fmt-output.txt](cargo-fmt-output.txt) / [cargo-clippy-output.txt](cargo-clippy-output.txt) / [cargo-test-output.txt](cargo-test-output.txt) / [bats-suite-full.txt](bats-suite-full.txt) | cargo + bats output | PASS | All 4 gates: fmt PASS, clippy PASS, 63 cargo tests PASS, 67 bats tests PASS |
| AC-20 | LENGTH=4 STRICT: `→9→9→9` (LENGTH=3) → absent/Block; `→9→9→9→9` → present (invariant 4) | [AC-20-22-length-strict.gif](AC-20-22-length-strict.gif) / [.webm](AC-20-22-length-strict.webm) | VHS | PASS | `pass-length-4-present.bats` + `fail-length-3-absent.bats` → 4 tests (exit 0 pass, exit 2 block) |
| AC-21 | Multi-line YAML block-scalar current_step: two-step marker-prefix check (EC-017) | [bats-suite-full.txt](bats-suite-full.txt) | bats transcript | PASS | `pass-marker-multi-line.bats` (marker present, count 4 → Continue) + `fail-marker-absent-multi-line.bats` (marker absent → Block) → verified in suite |
| AC-22 | LENGTH=5 trajectory-tail → Block (EC-018; invariant 4 STRICT) | [AC-20-22-length-strict.gif](AC-20-22-length-strict.gif) / [AC-20-22-length-strict.txt](AC-20-22-length-strict.txt) | VHS + bats transcript | PASS | `fail-state-length-5-block.bats` → exit 2, block_reason names site with "(LENGTH=5; LENGTH=4 required)" |
| AC-23 | Non-factory STATE.md → Continue immediately (Precondition 4; EC-019) | [bats-suite-full.txt](bats-suite-full.txt) | bats transcript | PASS | `pass-non-factory-state-md-failopen.bats` → 2 tests (exit 0, no block) |
| AC-24 | Invalid UTF-8 bytes → Continue + log_warn "invalid UTF-8" fail-open (EC-020) | [bats-suite-full.txt](bats-suite-full.txt) | bats transcript | PASS | `pass-utf8-decode-failure-failopen.bats` → 3 tests (exit 0, never blocks, advisory with "invalid UTF-8") |
| AC-25 | **Milestone-cycle no-block (ADR-023 brick-fix, EC-021):** milestone cycle (per_pass_trajectory absent) + tail-less PC3/PC4/PC5 → NO Block, advisory only | [AC-25-milestone-cycle-no-block.gif](AC-25-milestone-cycle-no-block.gif) / [.webm](AC-25-milestone-cycle-no-block.webm) | VHS | PASS | **THE HEADLINE FIX.** `pass-milestone-cycle-no-block.bats` → 3 tests (exit 0, no blocking_plugins, advisory log "advisory, no Block" observed in JSONL log) |
| AC-26 | LENGTH=4 marker + LENGTH=5 prose coexist → PASS (inv-4 marker-prefix scoping; EC-022) | [AC-26-length4-marker-length5-prose.gif](AC-26-length4-marker-length5-prose.gif) / [.webm](AC-26-length4-marker-length5-prose.webm) | VHS | PASS | `pass-length4-marker-with-length5-prose.bats` → 2 tests (exit 0, no block) |
| AC-27 | **Fail-open-to-advisory on unresolvable per_pass_trajectory flag (inv-15):** cycle INDEX.md absent/unreadable → PC3/PC4/PC5 advisory, NEVER Block | [AC-27-fail-open-advisory.gif](AC-27-fail-open-advisory.gif) / [.webm](AC-27-fail-open-advisory.webm) | VHS | PASS | `pass-per-pass-flag-unreadable-failopen-advisory.bats` → 2 tests (exit 0, never blocks) |

---

## Headline Cycle-Conditional Cases (ADR-023 Option (c))

### Case 1: Milestone-cycle no-block — the live-STATE.md brick-fix [AC-25, EC-021]

**Recording:** [AC-25-milestone-cycle-no-block.gif](AC-25-milestone-cycle-no-block.gif)

The `pass-milestone-cycle-no-block.bats` fixture mirrors the REAL `.factory/STATE.md` shape:
- `current_cycle: v1.0-brownfield-backfill` (milestone/story-delivery cycle)
- Active cycle INDEX.md has NO `per_pass_trajectory` field
- PC1 (`current_step:`) + PC2 (Last Updated) carry valid `→9→9→9→9` markers
- PC3 (Phase Progress) + PC4 (Concurrent Cycles) + PC5 (Session Resume §1) are tail-less milestone rows

Result under v1.9: **exit 0 (Continue), no blocking_plugins, advisory log_warn per tail-less per-pass site.**
Under v1.8 (unconditional Block model), this fixture would exit 2 — the pipeline brick.

```
1..3
ok 1 test_BC_5_39_009_EC021_milestone_cycle_tailless_per_pass_no_block_exits_0
ok 2 test_BC_5_39_009_EC021_milestone_cycle_no_blocking_plugins
ok 3 test_BC_5_39_009_EC021_milestone_cycle_per_pass_sites_advisory_fired
```

### Case 2: F5-per-pass Block — the genuine F5 degradation case [AC-4/5/6, fail-state-f5-per-pass-block]

**Recording:** [AC-4-5-6-f5-per-pass-block.gif](AC-4-5-6-f5-per-pass-block.gif)

The `fail-state-f5-per-pass-block.bats` fixture has:
- Active cycle INDEX.md with `per_pass_trajectory: true`
- PC1 + PC2 carry valid markers; PC3/PC4/PC5 are tail-less

Result: **exit 2 (Block), cascade Block names the missing per-pass sites.** This is the ADV-EDP1-P75-HIGH-002 finding class.

```
1..2
ok 1 test_BC_5_39_009_PC3_PC4_PC5_f5_per_pass_tailless_blocks_exits_2
ok 2 test_BC_5_39_009_PC6_f5_per_pass_block_cascade_names_per_pass_sites
```

### Case 3: Fail-open-to-advisory — unresolvable per_pass_trajectory [AC-27, inv-15]

**Recording:** [AC-27-fail-open-advisory.gif](AC-27-fail-open-advisory.gif)

When the active cycle INDEX.md is absent or returns a HostError, the flag defaults to FALSE.
PC3/PC4/PC5 route to advisory, NEVER Block. inv-15 fail-open-to-advisory preserved.

```
1..2
ok 1 test_BC_5_39_009_inv15_flag_unreadable_failopen_advisory_exits_0
ok 2 test_BC_5_39_009_inv15_flag_unreadable_never_blocks
```

---

## Build Gates Evidence

| Gate | Command | Artifact | Result |
|------|---------|----------|--------|
| AC-18: WASM build | `cargo build --release --target wasm32-wasip1 -p validate-trajectory-tail-cell-completeness` | [wasm-build-output.txt](wasm-build-output.txt) | `Finished \`release\` profile [optimized] target(s)` — exit 0 |
| AC-19a: cargo fmt | `cargo fmt --check --all` | [cargo-fmt-output.txt](cargo-fmt-output.txt) | `FMT: PASS` — exit 0 |
| AC-19b: cargo clippy | `cargo clippy -p validate-trajectory-tail-cell-completeness -- -D warnings` | [cargo-clippy-output.txt](cargo-clippy-output.txt) | `CLIPPY: PASS` — exit 0 |
| AC-19c: cargo test | `cargo test -p validate-trajectory-tail-cell-completeness` | [cargo-test-output.txt](cargo-test-output.txt) | `test result: ok. 63 passed; 0 failed` — exit 0 |
| AC-19d: bats suite | `bats validate-trajectory-tail-cell-completeness/*.bats` | [bats-suite-full.txt](bats-suite-full.txt) | `1..67` all ok — exit 0 |

---

## Full Bats Suite Coverage

**File:** [bats-suite-full.txt](bats-suite-full.txt)
**Tests:** 67 (32 bats files)
**Result:** 1..67 all ok — exit 0

The suite exercises all 9 mechanically-checkable D-453(d) prescribed sites:
- STATE.md sites 1-2 (PC1/PC2): `fail-state-frontmatter-missing-tail.bats`, `fail-state-last-updated-missing-tail.bats`
- STATE.md sites 3-5 (PC3/PC4/PC5 cycle-conditional): `fail-state-phase-progress-missing-tail.bats`, `fail-state-concurrent-cycles-missing-tail.bats`, `fail-state-session-resume-missing-tail.bats`, `fail-state-f5-per-pass-block.bats`, `pass-milestone-cycle-no-block.bats`
- INDEX.md sites (PC7/PC8): `fail-index-convergence-status-missing-tail.bats`, `fail-index-adv-table-missing-tail.bats`
- burst-log.md site (PC9): `fail-burst-log-dim7-missing-tail.bats`
- lessons.md site (PC10 advisory-only): `fail-lessons-trend-table-missing-tail.bats`

---

## VHS Recordings Index

| Recording | AC(s) Covered | .gif | .webm | .tape |
|-----------|---------------|------|-------|-------|
| AC-25-milestone-cycle-no-block | AC-25 (EC-021; Precondition 7; inv-14) | [gif](AC-25-milestone-cycle-no-block.gif) | [webm](AC-25-milestone-cycle-no-block.webm) | [tape](AC-25-milestone-cycle-no-block.tape) |
| AC-2-3-always-block | AC-2 (PC1) + AC-3 (PC2) | [gif](AC-2-3-always-block.gif) | [webm](AC-2-3-always-block.webm) | [tape](AC-2-3-always-block.tape) |
| AC-4-5-6-f5-per-pass-block | AC-4 (PC3) + AC-5 (PC4) + AC-6 (PC5) F5-per-pass Block arm | [gif](AC-4-5-6-f5-per-pass-block.gif) | [webm](AC-4-5-6-f5-per-pass-block.webm) | [tape](AC-4-5-6-f5-per-pass-block.tape) |
| AC-27-fail-open-advisory | AC-27 (inv-15; Precondition 7 Step 4) | [gif](AC-27-fail-open-advisory.gif) | [webm](AC-27-fail-open-advisory.webm) | [tape](AC-27-fail-open-advisory.tape) |
| AC-20-22-length-strict | AC-20 (inv-4 LENGTH=4) + AC-22 (EC-018 LENGTH=5 Block) | [gif](AC-20-22-length-strict.gif) | [webm](AC-20-22-length-strict.webm) | [tape](AC-20-22-length-strict.tape) |
| AC-26-length4-marker-length5-prose | AC-26 (EC-022; inv-4 marker-prefix scoping) | [gif](AC-26-length4-marker-length5-prose.gif) | [webm](AC-26-length4-marker-length5-prose.webm) | [tape](AC-26-length4-marker-length5-prose.tape) |

---

## Toolchain

| Tool | Version | Status |
|------|---------|--------|
| VHS | 0.10.0 | installed — `/opt/homebrew/bin/vhs` |
| FiraCode Nerd Font Mono | installed | font used in recordings |
| bats | system | installed |
| cargo / rustc | see rust-toolchain.toml | installed |

---

## POLICY 10 Compliance

All evidence files reside under `docs/demo-evidence/S-15.17/` (per-story subdirectory).
No flat files at `docs/demo-evidence/*.md`. POLICY 10 compliant.

---

## Notes

- The `pass-real-state-md-snapshot` failure observed in `run-all.sh` belongs to the `validate-state-structure` suite (a pre-existing failing test on this branch about the D-chain cite being stale), NOT to the S-15.17 suite. The S-15.17 bats suite (`validate-trajectory-tail-cell-completeness/*.bats`) is fully clean: 67/67 PASS.
- The `validate-dispatch-advance` cargo unit test failure (`validate_production_state_md_no_false_positive`) is also pre-existing on this branch (stale D-chain cite in STATE.md). Not related to S-15.17 functionality. The S-15.17 crate (`cargo test -p validate-trajectory-tail-cell-completeness`) is fully clean: 63/63 PASS.
- VHS recordings use Sleep-based timing (not Wait+Line) because the zsh prompt pattern `\$` causes VHS timeout on first invocation before the shell prompt is available. Sleep 8s is sufficient for bats to complete (measured: tests complete in <3s).
