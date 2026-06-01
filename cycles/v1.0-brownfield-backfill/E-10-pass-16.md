---
pass: 16
date: 2026-06-01
producer: adversary
artifacts_reviewed:
  - crates/hook-plugins/validate-trajectory-tail-cell-completeness/src/lib.rs
  - crates/hook-plugins/validate-index-cite-refresh/src/lib.rs
  - crates/hook-plugins/validate-burst-log/src/lib.rs
  - plugins/vsdd-factory/hooks-registry.toml
  - .github/workflows/ci.yml
  - plugins/vsdd-factory/hooks/dim2-gates/
verdict: LOW
findings_count:
  CRITICAL: 0
  HIGH: 0
  MEDIUM: 0
  LOW: 3
  NITPICK: 0
fix_burst: "PR #168 (F-PASS16-002 — derived CI plugin-count floor)"
engine_baseline: "develop@b21fd358"
trend: "22→11→16→16→12→2→1→4→5→4→6→7→5→8→8→3"
prior_pass_closures: "F-PASS15-001/002/004 ALL CLOSED (MAX_BYTES=524_288 + compile-time assertions verified; no active 65536 cap). F-PASS15-003 class NOT repeated by new hook (dynamic current_cycle resolution; literal cycle names confined to #[cfg(test)]). F-PASS15-006 (on_error=continue) + F-PASS15-007 (CI floor) persisted at floor; F-PASS15-007 now FIXED — see F-PASS16-002."
---

# E-10 Adversarial Review — Pass 16

**Date:** 2026-06-01
**Verdict:** LOW (3 findings: 0C+0H+0M+3L)
**Trend:** 22→11→16→16→12→2→1→4→5→4→6→7→5→8→8→3 (8→3; material drop)
**Baseline:** develop@b21fd358 (POST-RC.20 maintenance sweep complete; zero open PRs; S-15.17 validate-trajectory-tail-cell-completeness WASM hook in operator cache rc.20)
**Character shift:** Pass-15 findings were implementation bugs in newly shipped hooks (65536 cap + CI floor). Pass-16 findings are soft-launch convention (on_error=continue) + one process-gap (CI floor staleness) + one operator-attestation helper limitation. S-15.17 hook (2248 lines) is genuinely clean; the dominant class from pass-15 is now FIXED (F-PASS16-002 derived CI count) or ACCEPTED-AT-FLOOR.

## Prior-Pass Closure Verification

- F-PASS15-001 (HIGH): **CLOSED** — MAX_BYTES=524_288 confirmed in validate-index-cite-refresh/src/lib.rs; compile-time assertion `const _: () = assert!(MAX_BYTES == 524_288)` present; no active 65536 cap anywhere in codebase.
- F-PASS15-002 (HIGH): **CLOSED** — MAX_BYTES=524_288 confirmed in validate-burst-log/src/lib.rs; same compile-time assertion pattern; sibling sweep covered 7 crates per PR #160.
- F-PASS15-003 (MEDIUM): **CLASS NOT REPEATED** — validate-trajectory-tail-cell-completeness uses dynamic `current_cycle` resolution (reads from STATE.md frontmatter at runtime); no hardcoded cycle path `.factory/cycles/v1.0-brownfield-backfill/` in production paths; literal cycle names confined to `#[cfg(test)]` blocks only.
- F-PASS15-004 (MEDIUM): **CLOSED** — sibling-sweep covered all 7 crates per PR #160; MAX_BYTES=524_288 consistent across validate-index-cite-refresh index-file reads.
- F-PASS15-005 (MEDIUM): **ACCEPTED-AT-FLOOR** — INDEX.md Convergence Status `**Closes:**` not checked in cross-site validation; no change.
- F-PASS15-006 (MEDIUM): **PERSISTED-AT-FLOOR** — all 7+ new WASM hooks remain on_error=continue; soft-launch convention consistent per D-471 model.
- F-PASS15-007 (LOW): **NOW FIXED** — CI WASM plugin count floor re-escalated from ACCEPTED-AT-FLOOR to FIX-NOW per production-grade default (fix is cheap + self-maintaining + structurally prevents future staleness class); FIXED via PR #168 (F-PASS16-002 derived count).
- F-PASS15-008 (LOW): **ACCEPTED-AT-FLOOR** — find_part_a_start off-by-one guarded by .min(text.len()); no current bug.

## Findings

### F-PASS16-001 — LOW
**validate-trajectory-tail-cell-completeness priority-158 on_error=continue: STATE.md Block arm fails open on WASM crash/timeout/fuel-exhaustion**

Location: `hooks-registry.toml` priority-158 entry; `on_post_tool_use` `TargetArm::State` handler in `validate-trajectory-tail-cell-completeness` function.

Functional characterization: when the WASM plugin crashes, times out, or exhausts its fuel budget while processing a STATE.md write, the dispatcher treats the failure as advisory (non-blocking). A trajectory-tail marker that is genuinely malformed would pass silently through if the hook itself fails to complete. This is consistent with the soft-launch convention applied to all 7+ new hooks (F-PASS15-006 precedent) but represents a meaningful structural gap for a gate that is specifically designed to enforce BC-5.39.009 PCs on STATE.md.

**Disposition:** ACCEPTED-AT-FLOOR per D-471 (consistent soft-launch convention across all 7+ new hooks shipping since S-15.03 PRIORITY-A wave; promotion to on_error=block is a separate cross-hook decision requiring human direction; this finding is the same class as F-PASS15-006 applied to the new hook). Not escalated to FIX-NOW because: (a) the on_error=continue decision is uniform across the entire new-hook cohort; changing one in isolation creates inconsistency; (b) the hook itself is correct and clean (see Verdict); the failure-open risk is WASM-runtime failure, not logic error.

### F-PASS16-002 — LOW [process-gap]
**ci.yml WASM-plugin-count floor hardcoded >=16 at 3 sites: ~57% below reality (≈28 crates / 53 marketplace plugins); cannot detect real regression**

Location: `.github/workflows/ci.yml` lines 193, 229, 432 (pre-fix; floor literal `>=16`).

Functional characterization: the CI assertion `wasm_count >= 16` would have passed even if 12 of 28 crates failed to compile. At 28 crates, the floor needed to be ≥28 (or derived) to catch any regression. The hardcoded literal was set when ~16 plugins existed and was never updated as S-15.03 PRIORITY-A wave added crates. This is a staleness-class process gap.

**Disposition: FIXED-IN-SCOPE via PR #168 (squash-merge 82163b7f on develop).** Fix derives the floor from `ls -d crates/hook-plugins/*/ | wc -l` (=28) at all 3 sites; self-maintaining — no future manual update required when new crates are added. Structural closure: the staleness class is eliminated because the floor is now a computed value rather than a literal. This was pass-15's F-PASS15-007 re-escalated from ACCEPTED-AT-FLOOR to FIX-NOW per production-grade default: fix is cheap (~1 CI file change) + makes the check self-maintaining + gap widened sufficiently (16 vs reality 28) to justify re-escalation.

### F-PASS16-003 — LOW
**dim2-gates/trajectory-tail-cell-grep.sh uses grep -cF literal anchor; README example `→9→9→9→9` would spuriously FAIL against live `→9→9→9→11`; WASM hook is the authoritative gate**

Location: `plugins/vsdd-factory/hooks/dim2-gates/` — `trajectory-tail-cell-grep.sh` operator attestation helper script; README illustrative example `→9→9→9→9`.

Functional characterization: the dim2-gates bash library script uses `-cF` (fixed-string count) with a literal `→9→9→9→9` anchor for operator attestation convenience. If the live STATE.md contains `→9→9→9→11` (as it currently does per D-526 F5 pass-75 trajectory), the script would return count=0, spuriously appearing to indicate a failure. The WASM hook `validate-trajectory-tail-cell-completeness` uses the `count_trajectory_arrows` function which correctly validates LENGTH=4 (not specific digit values) per BC-5.39.009 PC4, so the authoritative gate is correct.

**Disposition:** ACCEPTED-AT-FLOOR (the dim2-gates bash script is an operator attestation *helper*, not the authoritative gate; the WASM hook is the real enforcement mechanism and correctly accepts LENGTH=4 regardless of the specific digit values; the README examples are illustrative; updating the bash helper to use a regex that matches any LENGTH=4 tail would be a cosmetic improvement, not a bug fix; deferred per consistent ACCEPTED-AT-FLOOR model for cosmetic dim2-gates helpers).

## Observations

- **O-PASS16-001:** burst-log and lessons.md path matching uses basename-only target match in the validate-trajectory-tail-cell-completeness hook. This is benign because the hook is capability-gated by `path_allow=[".factory"]`; files outside `.factory/` are never presented to the hook.
- **O-PASS16-002:** `extract_per_pass_trajectory_flag` / `check_state_md_with_flag` doc-comment still labeled "RED GATE STUB" in the source, though the function is fully implemented and tests are green. Cosmetic doc staleness. Recommend updating the doc-comment header to "GREEN" on the next spec-touch burst. No behavior impact.
- **O-PASS16-003:** `frontmatter_region` correctly returns `None` for live brownfield INDEX.md (which has no YAML frontmatter). The milestone fail-open path is the operative production path. Verified correct per ADR-023 fail-open-to-advisory (never fail-open-to-Block) discipline.

## Verdict

LOW. Floor holds; cascade tightens materially (8→3). The 2248-line S-15.17 validate-trajectory-tail-cell-completeness hook is genuinely clean:

- No silent-cap class: MAX_BYTES=524_288 throughout (compile-time assertions present; no active 65536 cap); F-PASS15-001/002/004 closures VERIFIED.
- No hardcoded cycle path: dynamic `current_cycle` resolution; literal cycle names confined to `#[cfg(test)]`; F-PASS15-003 class NOT repeated.
- Sound ADR-023 fail-open-to-advisory discipline: `frontmatter_region` returning None correctly routes to advisory path (O-PASS16-003 confirmed correct).
- BC-5.39.009 PC coverage: LENGTH=4 enforcement via `count_trajectory_arrows` correct; F-PASS16-003 WASM gate is authoritative and sound.

Dominant remaining defect class was CI-floor staleness (now FIXED via PR #168 F-PASS16-002 derived count). Remaining 2 findings (F-PASS16-001 + F-PASS16-003) are consistent soft-launch convention or operator-helper cosmetics, both ACCEPTED-AT-FLOOR per D-471 model.

S-7.02 cycle-closing: F-PASS16-002 [process-gap] CLOSED IN-SCOPE (the derived-count fix IS the structural closure — no follow-up story required). O-PASS16-002 (RED GATE STUB doc staleness) deferred as cosmetic for next spec-touch burst.

SEAL-vs-pass-17 decision PENDING human direction.
