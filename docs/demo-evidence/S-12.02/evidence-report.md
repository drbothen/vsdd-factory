---
story_id: S-12.02
title: Per-Story Adversary Convergence Hook — Demo Evidence
date: 2026-05-07
step: 5 (demo recording)
hook: validate-per-story-adversary-convergence
bcs_covered:
  - BC-4.10.001
  - BC-4.10.002
---

# Evidence Report — S-12.02

Hook: `validate-per-story-adversary-convergence`  
Event: `SubagentStop`  
Priority: 960  
Build: 30/30 cargo tests green, 12/12 bats (11 ok + 1 skip)  
WASM artifact: `plugins/vsdd-factory/hook-plugins/validate-per-story-adversary-convergence.wasm` (148 KB)

## AC Coverage Table

| Demo ID | File | AC(s) Covered | BC | Status |
|---|---|---|---|---|
| AC-001-cleared-state-continues | [AC-001-cleared-state-continues.gif](AC-001-cleared-state-continues.gif) | AC-001 | BC-4.10.001 PC5 | RECORDED |
| AC-002-block-missing-state | [AC-002-block-missing-state.gif](AC-002-block-missing-state.gif) | AC-002 (branch 1) | BC-4.10.001 PC2 | RECORDED |
| AC-002-block-insufficient-passes | [AC-002-block-insufficient-passes.gif](AC-002-block-insufficient-passes.gif) | AC-002 (branch 2) | BC-4.10.001 PC3 | RECORDED |
| AC-002-block-non-nitpick | [AC-002-block-non-nitpick.gif](AC-002-block-non-nitpick.gif) | AC-002 (branch 3) | BC-4.10.001 PC4 | RECORDED |
| AC-003-graceful-degrade-no-wave-gate | [AC-003-graceful-degrade-no-wave-gate.gif](AC-003-graceful-degrade-no-wave-gate.gif) | AC-003 | BC-4.10.002 EC-001 | RECORDED |
| AC-004-graceful-degrade-no-cycle-dir | [AC-004-graceful-degrade-no-cycle-dir.gif](AC-004-graceful-degrade-no-cycle-dir.gif) | AC-004 | BC-4.10.002 inv-3 | RECORDED |
| AC-005-deferred-findings-continue | [AC-005-deferred-findings-continue.gif](AC-005-deferred-findings-continue.gif) | AC-005 | BC-4.10.001 EC-004 | RECORDED |
| AC-006-first-failure-only | [AC-006-first-failure-only.gif](AC-006-first-failure-only.gif) | AC-006 | BC-4.10.001 EC-005 | RECORDED |
| AC-007-malformed-json | [AC-007-malformed-json.gif](AC-007-malformed-json.gif) | AC-007 | BC-4.10.001 EC-002 | RECORDED |
| block-with-fix-canonical-form | [block-with-fix-canonical-form.gif](block-with-fix-canonical-form.gif) | Cross-cutting BC-4.10.001 | VP-071 v1.1 canonical form | RECORDED |
| WASM-artifact-built | [WASM-artifact-built.gif](WASM-artifact-built.gif) | Cross-cutting (artifact) | BC-4.10.001 PC1 | RECORDED |
| hooks-registry-entry | [hooks-registry-entry.gif](hooks-registry-entry.gif) | AC-013, cross-cutting | BC-4.10.001 PC1 | RECORDED |
| bats-green | [bats-green.gif](bats-green.gif) | All ACs (structural suite) | BC-4.10.001+002 | RECORDED |
| cargo-test-green | [cargo-test-green.gif](cargo-test-green.gif) | All ACs (full unit suite) | BC-4.10.001+002 | RECORDED |

## Skipped ACs

| AC | Reason |
|---|---|
| AC-008 (test design / assert hook does not call write_file) | Structural invariant verified by `test_BC_4_10_001_no_write_file_calls` cargo test, visible in `cargo-test-green.gif`. No additional demo surface adds value beyond the test name appearing in the run. |
| AC-009 (no-write invariant) | Same as AC-008 — covered by cargo test name in `cargo-test-green.gif`. |
| AC-010 (structural assertions / bats count) | Bats suite count (11 ok + 1 skip) is directly visible in `bats-green.gif`. |
| AC-011 (host ABI version constant) | Covered by `test_BC_4_10_001_host_abi_version_is_one` in `cargo-test-green.gif` output. |
| AC-012 (hook logic runs without WASM runtime) | Covered by `test_BC_4_10_001_hook_logic_runs_without_wasm_runtime` in `cargo-test-green.gif` output. |

All skipped ACs are structural / test-design acceptance criteria. Their evidence is the test name appearing in the `cargo-test-green.gif` and `bats-green.gif` recordings, which show those tests passing in the full suite.

## Recording Details

All recordings use VHS 0.10.0 with:
- Font: FiraCode Nerd Font Mono
- Theme: Dracula
- Dimensions: 1200x600 (1200x400 for structural/artifact demos)
- Font size: 14, Padding: 20

Each cargo unit-test tape isolates a single named test with `-- --nocapture` and pipes through `tail -12` to show the test result line cleanly. The two structural tapes (WASM artifact, hooks-registry) run shell commands directly. The bats tape runs the full suite showing all 12 test lines.

## Summary

- 14 demos recorded (14 GIFs + 14 tape scripts)
- 5 ACs marked SKIPPED with reference to cargo/bats evidence visible in full-suite recordings
- 0 recordings failed
- Total artifact size: ~1.5 MB
- All demos self-contained and run from worktree root
