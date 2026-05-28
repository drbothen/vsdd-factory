---
pass: 15
date: 2026-05-27
producer: adversary
artifacts_reviewed:
  - crates/hook-plugins/validate-index-cite-refresh/src/lib.rs
  - crates/hook-plugins/validate-burst-log/src/lib.rs
  - crates/hook-plugins/validate-state-structure/src/lib.rs
  - crates/hook-plugins/validate-dispatch-advance/src/lib.rs
  - crates/hook-plugins/validate-closes-completeness/src/lib.rs
  - crates/hook-plugins/validate-policies-schema/src/lib.rs
  - crates/hook-plugins/session-start-telemetry/src/lib.rs
  - plugins/vsdd-factory/hooks-registry.toml
  - .github/workflows/ci.yml
  - plugins/vsdd-factory/hooks/dim2-gates/
verdict: MEDIUM-HIGH
findings_count:
  CRITICAL: 0
  HIGH: 2
  MEDIUM: 4
  LOW: 2
  NITPICK: 0
fix_burst: "PR #160 (F-PASS15-001/002/004 — 65536→524288 sibling sweep)"
engine_baseline: "develop@ced39c82"
trend: "22→11→16→16→12→2→1→4→5→4→6→7→5→8→8"
prior_pass_closures: "F-PASS14-004 STRUCTURALLY CLOSED (validate-policies-schema); F-PASS14-006 STRUCTURALLY CLOSED (bare integer IDs enforced)"
---

# E-10 Adversarial Review — Pass 15

**Date:** 2026-05-27
**Verdict:** MEDIUM-HIGH (8 findings: 0C+2H+4M+2L)
**Trend:** 22→11→16→16→12→2→1→4→5→4→6→7→5→8→8 (holds at 8)
**Baseline:** develop@ced39c82 (S-15.03 PRIORITY-A COMPLETE; 18 PRs since pass-14 seal)
**Character shift:** Pass-14 findings were governance-process META-class; pass-15 findings are implementation bugs in newly shipped code. S-15.03 automation wave closed the process-gap class.

## Prior-Pass Closure Verification

- F-PASS14-004 (MEDIUM): **STRUCTURALLY CLOSED** — validate-policies-schema hook enforces frontmatter schema
- F-PASS14-006 (MEDIUM): **STRUCTURALLY CLOSED** — bare integer IDs enforced by check_policy_id_format
- F-PASS14-001/002/003/005/007/008: Remain ACCEPTED-AT-ASYMPTOTIC-FLOOR per D-471

## Findings

### F-PASS15-001 — HIGH
**validate-index-cite-refresh 65536 STATE.md cap: silently inert on production STATE.md (95 KiB)**
Location: `crates/hook-plugins/validate-index-cite-refresh/src/lib.rs` lines 312, 377, 412
D-429(b) cross-cell version sweep is functionally dead against production STATE.md.
**Fix:** PR #160 — raised to MAX_BYTES (524_288)

### F-PASS15-002 — HIGH
**validate-burst-log 65536 hardcoded cap: latent silent failure**
Location: `crates/hook-plugins/validate-burst-log/src/lib.rs` line 479
Will silently fail-open once burst-log.md exceeds 64 KiB.
**Fix:** PR #160 — raised to MAX_BYTES (524_288)

### F-PASS15-003 — MEDIUM
**Three hooks hardcode brownfield cycle path `.factory/cycles/v1.0-brownfield-backfill/`**
Location: validate-state-structure, validate-closes-completeness, validate-index-cite-refresh
Will break on cycle rotation. No story/TD attached to the cycle-resolver future work.
**Disposition:** ACCEPTED-AT-FLOOR — latent, not yet causing failures; cycle rotation is future work.

### F-PASS15-004 — MEDIUM
**validate-index-cite-refresh 65536 for index file reads — inconsistent with sibling convention**
Location: `crates/hook-plugins/validate-index-cite-refresh/src/lib.rs` line 312
**Fix:** PR #160 — raised to MAX_BYTES (524_288)

### F-PASS15-005 — MEDIUM
**validate-closes-completeness Phase 2 missing INDEX.md as secondary citation site**
Location: `crates/hook-plugins/validate-closes-completeness/src/lib.rs` lines 1417-1422
INDEX.md Convergence Status `**Closes:**` annotations not checked in cross-site validation.
**Disposition:** ACCEPTED-AT-FLOOR — design gap, not a regression; INDEX.md coverage is partial.

### F-PASS15-006 — MEDIUM
**All 7 new WASM hooks use on_error=continue — crashes are non-blocking**
Location: `plugins/vsdd-factory/hooks-registry.toml`
Production-grade default suggests on_error=block for structural integrity validators.
**Disposition:** ACCEPTED-AT-FLOOR — intentional soft-launch; promote after stabilization.

### F-PASS15-007 — LOW
**CI WASM plugin count assertion hardcodes >=16 but 21 plugins exist**
Location: `.github/workflows/ci.yml` lines 193, 229, 432
**Disposition:** ACCEPTED-AT-FLOOR — floor still catches major regressions.

### F-PASS15-008 — LOW
**find_part_a_start off-by-one in pos tracking (guarded, latent only)**
Location: `crates/hook-plugins/validate-closes-completeness/src/lib.rs` lines 1002-1020
**Disposition:** ACCEPTED-AT-FLOOR — guarded by .min(text.len()); no current bug.

## Observations

- O-PASS15-001: S-15.03 automation wave is well-architected; consistent patterns across 7 new hooks
- O-PASS15-002: validate-closes-completeness (1926 lines) is the largest WASM hook; ADR-022 pointer-file mechanism well-designed
- O-PASS15-003: dim2-gates bash library (S-15.08) directly addresses D-449(a) literal-shell requirement
- O-PASS15-004: wait_for_log_event helper (S-15.05) is a genuine de-flake improvement
- O-PASS15-005: validate-policies-schema handles YAML anchors/aliases and single-document format

## Verdict

8 findings (0C+2H+4M+2L). Trend holds at 8. Character shifted from governance-process to implementation-correctness — the automation wave WORKED. Fix-burst PR #160 closes the 2 HIGHs + 1 MEDIUM (65536→524288 sweep). Remaining 5 findings ACCEPTED-AT-FLOOR.
