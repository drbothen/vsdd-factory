# F-P2-001 + F-P2-008 Closure Summary

**Story:** S-12.08 — Migrate convergence hook to consume `plugin_config.wave_context.stories`
**Branch:** `feature/S-12.08-convergence-hook-context-migration`
**Convergence:** Pass-6, 3/3 NITPICK_ONLY streak (BC-5.39.001 satisfied)

---

## End-to-End Pipeline

```
wave-state.yaml
      |
      v
WaveContextResolver (S-12.07)
  reads wave-state.yaml
  emits: wave_context = {
    "stories": ["S-FAKE-001", "S-FAKE-002"],
    "wave_id": "w1",
    "cycle_id": "test-cycle-001"
  }
      |
      v
Dispatcher context injection (BC-1.13.001)
  hooks-registry.toml: needs_context = ["wave_context"]
  merges wave_context into plugin_config (additive overlay, BC-4.12.005)
      |
      v
validate-per-story-adversary-convergence (S-12.08)
  reads plugin_config["wave_context"]["stories"]
  for each story_id:
    read .factory/cycles/{cycle_id}/{story_id}/adversary-convergence-state.json
    if passes_clean < 3 → Block(CONVERGENCE_PASSES_INSUFFICIENT)
    if last_classification != "NITPICK_ONLY" → Block(CONVERGENCE_CLASSIFICATION_INSUFFICIENT)
  if all pass → Continue
      |
      v
Dispatcher exit code: 2 (Block) or 0 (Continue)
```

---

## F-P2-001 Root Cause

**Finding:** `validate-per-story-adversary-convergence` was INERT in production.

**Root cause:** The hook read story IDs from `plugin_config["stories"]` — a key that
was NEVER present at dispatch time. The key only appeared in test fixtures with static
config. At real dispatch time, `plugin_config["stories"]` was always absent, causing the
old `extract_stories_from_config` function to silently return `Err` and the hook to
emit `Continue` — every time, unconditionally.

**Effect:** The convergence gate never actually blocked anything. Any wave-gate dispatch
would pass regardless of story convergence state.

**Fix (S-12.08):** Replace the static-config read with `plugin_config["wave_context"]["stories"]`,
which IS populated at dispatch time by `WaveContextResolver` (S-12.07). Remove the old
graceful-degrade fallback (the `Err → Continue` path). Enforce: absent `wave_context` → Block.

---

## F-P2-008 Root Cause

**Same root cause as F-P2-001.** F-P2-008 was filed as a separate tracking item for
the same underlying defect: the convergence hook silently continued when the stories key
was absent. Both findings are closed by S-12.08.

---

## Adversary Convergence Trajectory

| Pass | Severity | Key Issues |
|------|----------|------------|
| Pass-1 | MED | WAVE_CONTEXT_MISSING not in BC-4.10.001; bats test completeness unclear |
| Pass-2 | MED | EC-004 (empty wave) semantic undecided; VP-071 kani toolchain block not documented |
| Pass-3 | LOW | P02-MED-003 fix (empty wave → Continue) verified; minor doc gaps |
| Pass-4 | NITPICK_ONLY | No blocking issues found |
| Pass-5 | NITPICK_ONLY | No blocking issues found |
| Pass-6 | NITPICK_ONLY | No blocking issues found — BC-5.39.001 convergence reached |

**3/3 NITPICK_ONLY streak = BC-5.39.001 convergence criterion satisfied.**

---

## Cross-Story Spec Alignment

| Artifact | Version | Amendment |
|----------|---------|-----------|
| BC-1.13.001 | v1.2 | Added postcondition 4: needs_context triggers resolver injection |
| BC-4.10.001 | v1.3 | Added WAVE_CONTEXT_MISSING + WAVE_CONTEXT_SCHEMA_ERROR block codes |
| ADR-018 | amended | WaveContextResolver registered as resolver plugin |
| S-12.07 | v1.5 | WaveContextResolver emits wave_context with stories array |
| S-12.08 | v1.2 | This story — convergence hook reads wave_context.stories |

---

## Resolver-Linker WASI Preview1 Fix

During Step 3b (WASM rebuild), a resolver-linker incompatibility was discovered:
the WASM artifact was built against `wasm32-wasi` (deprecated) but the dispatcher
expected `wasm32-wasip1`. This was fixed by rebuilding with the correct target triple.
The fix is transparent to story ACs but was necessary for the bats integration test
to exercise the real WASM pipeline.

---

## Closure Evidence

| AC | Test | Status |
|----|------|--------|
| AC-001 | `test_extract_stories_from_wave_context_reads_nested_array` | GREEN |
| AC-002 | 8 unit tests (absent/null wave_context → Block) | GREEN |
| AC-003 | 4 unit tests (wrong-type stories → Block) | GREEN |
| AC-004 | 6 vp071_equiv cargo tests (kani toolchain blocked, mitigated) | GREEN |
| AC-005 | `test_hooks_registry_has_needs_context_for_convergence_hook` + grep line 951 | GREEN |
| AC-006 | `test_static_config_preserved_after_wave_context_injection` | GREEN |
| AC-007 | `test_fake_callbacks_inject_wave_context_payload` + 52-test suite | GREEN |
| AC-008 | **bats test 1: unconverged → exit 2 (Block)** | GREEN — F-P2-001 CLOSED |
| AC-009 | **bats test 2: all converged → exit 0 (Continue)** | GREEN |
| AC-010 | `test_old_extract_stories_from_config_removed` + grep audit | GREEN |
| EC-001 | bats test 3 + `test_BC_4_10_001_ec001_empty_wave_returns_continue` | GREEN |

**Workspace:** 1346 tests PASS, 0 FAIL. Clippy: clean. Bats: 3/3 PASS.

**F-P2-001 CLOSED. F-P2-008 CLOSED. CRITICAL PATH TERMINUS REACHED.**
