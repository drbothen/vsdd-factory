# S-15.17 v1.9 LOCAL Adversary Cascade — Pass 1

**Date:** 2026-05-31
**Reviewer:** adversary (fresh-context, read-only)
**Target:** implementation diff 766ab7bc..e2ed562d (BC-5.39.009 v1.9 cycle-conditional re-spec per ADR-023 Option c; un-seal of D-522)
**Scope (BC-5.39.002):** validate-trajectory-tail-cell-completeness crate + hooks-registry.toml priority-158 + bats + fixtures. BC-5.39.009 v1.9; story v1.11.
**Note:** v1.9 is a material re-spec of the formerly-SEALED BC; prior v1.8 3-CLEAN (passes P1 HIGH→P2 LOW→P3/P4 CLEAN→P5 CRITICAL) is VOIDED per ADR-023 §5. This is the v1.9 cascade pass 1; streak entering = 0/3.

## Part A — Overall Verdict

**Verdict: CLEAN** (0 CRITICAL / 0 HIGH / 0 MEDIUM / 0 LOW / 2 NITPICK). **Streak after pass 1: 1/3.**

### Decisive safety check — Live-STATE.md dry-run (inv-15)
Traced the v1.9 STATE arm against the real `.factory/STATE.md` (current_cycle: v1.0-brownfield-backfill) + real `.factory/cycles/v1.0-brownfield-backfill/INDEX.md` (which has NO YAML frontmatter — opens with `# Cycle:` H1):
- extract_current_cycle → "v1.0-brownfield-backfill"; resolve_per_pass_trajectory reads the brownfield INDEX.md → frontmatter_region None → extract_per_pass_trajectory_flag → FALSE.
- per_pass_trajectory=false → check_state_md_with_flag(content, false) → only PC1/PC2 Block-eligible; both carry trajectory-tail →9→9→9→11 → Block set EMPTY → **Continue + advisory for tail-less PC3/4/5**.
- **The v1.9 hook does NOT brick the live milestone STATE.md.** The pass-5 CRITICAL is resolved. Every resolve_per_pass_trajectory exit (current_cycle None, decode Err, flag-extract) returns false → advisory; zero fail-open-to-Block path. inv-15 structurally sound.

## Part B — Findings

CRITICAL/HIGH/MEDIUM/LOW: none. Verified clean: inv-15 fail-open-to-advisory (all 3 resolve exits return false); extract_per_pass_trajectory_flag strict-true-only + frontmatter-scoped (body mention ignored; F5 INDEX.md per_pass_trajectory:true line 11 inside frontmatter reads true; milestone INDEX.md no-frontmatter reads false); PC1/PC2 always-Block independent of flag, flag==true joins single cascade (inv-8); inv-4 count==4 strict preserved; EC-022 genuine marker-prefix scoping; no v1.8 regression (check_state_md = wrapper(c,true)); POLICY 11 12 new tests call production fns; path-component-walk guard; MAX_BYTES u32 524288 on new cycle-INDEX read; is_char_boundary; lessons PC10 always Continue.

NITPICK observations:
- O-1 [process-gap]: resolve_per_pass_trajectory composition (current_cycle None→false, HostError→false) is exercised by bats integration fixtures (pass-milestone-cycle-no-block, pass-per-pass-flag-unreadable-failopen-advisory) rather than a direct end-to-end Rust unit test (resolve is private, takes real host::read_file). Orchestrator confirmed: milestone bats asserts exit 0 AND advisory_present for each PC3/4/5 site (refute_block) — load-bearing, not silent-skip. RESOLVED.
- O-2: resolve_per_pass_trajectory re-reads cycle INDEX.md while INDEX arm re-reads STATE.md — inherent to the BC's dual-read design; both MAX_BYTES-bounded + fail-open. No action.

**Streak after pass 1: 1/3.** Genuine convergence trajectory; proceed to pass 2.
