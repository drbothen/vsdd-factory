---
document_type: adversary-pass-report
level: ops
title: "S-15.09 LOCAL Adversary Cascade — Pass 7"
producer: adversary
timestamp: 2026-05-17T00:00:00Z
phase: per-story-step-4-local-adversary-cascade
story: S-15.09
pass: 7
verdict: MEDIUM
finding_count: { critical: 0, high: 0, medium: 1, low: 1, nitpick: 0, process_gap: 0 }
streak_3_clean: "0/3"
---

# S-15.09 LOCAL Adversary Cascade — Pass 7

## Part A — Findings

### F-P7-001 MEDIUM — Token Budget stale version self-cite (META-LEVEL recurrence of F-P6-003 class)

- **Location:** `.factory/stories/S-15.09-validate-state-structure-phase-1.md:768`
- **Evidence:** `| This story spec (v1.5; 13 ACs) | ~11,215 |` — frontmatter is v1.6, Token Budget cites v1.5.
- **Issue:** Pass-6 fix-burst updated Token Budget v1.3→v1.5 in the same burst that promoted the file to v1.6, leaving the cite stale at v1.5. Same-burst-self-cite-sweep gap. Per Partial-Fix Regression Discipline (S-7.01), value-change fixes must grep for all references to the old value.
- **Recommendation:** Update Token Budget to match new version. Codify same-burst-self-cite-sweep discipline: when a version-bump burst touches Token Budget cites, the cite must reference the NEW version (target of the bump), not the immediate predecessor.

### F-P7-002 LOW — AC-13 EC namespace mixed-scope citation

- **Location:** story `:247` (AC-13 last column)
- **Evidence:** `BC-5.39.005 postconditions 1-5; EC-007; EC-015 (banner-narrative-arrow case)`
- **Issue:** Story's local EC table enumerates EC-001..EC-012; AC-13 cites EC-007 (story-local: "All three violations simultaneously") AND EC-015 (BC-only: "banner-narrative-arrow"). Mixed namespace.
- **Recommendation:** Prefix every BC-scope EC citation: `BC-5.39.005 EC-007; BC-5.39.005 EC-015`.

## Part B — Production-Grade Default Audit

- Workspace member registration confirmed at root Cargo.toml:28.
- Bats `tool = "Edit|Write"` canonical across all 13 bats files.
- Bats `event = "PostToolUse"` parity.
- Production registry `path_allow = [".factory"]` bare.
- `is_state_md_target` path-component-strict.
- `is_char_boundary()` guard present.
- No `unwrap`/`expect`/`panic!` in production paths.
- BC PC4 max_bytes 524288 consistent with story PC4 (524_288) and lib.rs MAX_BYTES_STATE_MD (524_288). Sibling-sweep clean.
- `scan_for_last_wc_l` private-intent documented (F-P6-006 closure intact).
- BC subsystem SS-05 "Pipeline Orchestration" matches ARCH-INDEX:247.

## Part C — Self-Application Audit (META-LEVEL)

- F-P7-001 is the recurrence pattern: pass-6 fix moved the staleness window forward by one version rather than eliminating it. The cascade is hitting asymptotic floor on the same-burst-self-cite-sweep axis. Future bursts must apply the discipline same-burst.
- POLICY 13 regex-alternation + POLICY 15 verbatim-stdout compliance throughout.

## Verdict & Streak

- Pass-7 verdict: **MEDIUM** (1M + 1L).
- Streak: 0/3 → **0/3** (≥LOW resets).
- Fix-burst applied in-scope (story-writer @ 310c773b: story v1.6→v1.7 with Token Budget self-cite sweep + AC-13 EC namespace prefix).

## Fix-burst routing (orchestrator-routed; complete)

- story-writer @ `310c773b` — F-P7-001 + F-P7-002 closed (story v1.6→v1.7; Token Budget v1.7 cite; AC-13 EC-namespace prefixed; same-burst-self-cite-sweep discipline applied).
- state-manager — this persistence commit.

## Closure verification

- Story v1.7: Token Budget cites v1.7 (matches frontmatter); AC-13 EC-citations namespace-prefixed.
- Same-burst-self-cite-sweep verified by story-writer: zero remaining live body cites of v1.6 or earlier.
