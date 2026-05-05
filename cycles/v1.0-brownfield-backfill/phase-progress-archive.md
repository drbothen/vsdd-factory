# Phase Progress Archive — Pre-Phase-D Rows

> Extracted from STATE.md during D-237 state-hygiene burst 2026-05-05.
> Active Phase Progress table in STATE.md retains Phase D-4 + Release v1.0.0-rc.11 + Phase C burn-in rows.

| Phase | Status | Artifact |
|-------|--------|----------|
| Phase 0 — Brownfield Ingest | COMPLETE | initial BC migration in pass-3-* + pass-8-final-synthesis.md |
| Phase 1.1 — Architecture Index + ADRs | COMPLETE | ARCH-INDEX (10 SS-NN) + 13 of 13 ADRs (ADR-001..013) |
| Phase 1.2 — Sharded Architecture | COMPLETE | 10 SS-NN-\<name\>.md files |
| Phase 1.3 — L2 Domain Spec | COMPLETE | 8 sharded files (28 CAPs, 17 DIs, 22 DEs, 18 DECs, 35 entities) |
| Phase 1.4 — BC Migration | COMPLETE | 1,917 BC-S.SS.NNN files in 10 ss-NN/ shards + BC-INDEX.md |
| Phase 1.5 — Formal PRD | COMPLETE | 46 FRs (FR-041..FR-045 added prior waves; FR-046 added Wave 13 S-5.01), 76 NFRs, 100% BC traceability |
| Phase 1.6a — DTU Assessment | COMPLETE | DTU_REQUIRED: false |
| Phase 1.6b — Verification Properties | COMPLETE | 66 VPs (all draft, VP-001..VP-066; +2 E-7; +2 S-7.03; +1 VP-065 S-5.01; +1 VP-066 S-5.02) |
| Phase 1.7 — Extraction Validation R2 | in-progress | Migration fidelity check |
| Phase 1.8 — Story Migration | COMPLETE | 47 stories S-N.MM, 8 epics E-0..E-7 (41 migrated + 1 E-6 + 3 E-7 + 2 Wave 11 SS-03) |
| Phase 1d — Adversarial Spec Review | **COMPLETE — 17 passes; CONVERGENCE_REACHED 2026-04-27** | trajectory 25→12→5→2→1→0→0→1→2→4→3→1→1→2→0→0→0; ADR-013 satisfied |
| Release v1.0.0-beta.5 | COMPLETE | PR #5 merged 2001b97; tag 0a95c8c; bot bundle f1ec5bf; 5 plugins · 110 skills |
| Phase 2 — Story Decomposition | not-started | Unblocked; 45 stories ready for dependency graph + wave schedule |
| S-6.01 + E-7 sub-cycles | COMPLETE | S-6.01 create-adr GREEN (25/25 bats); E-7 codify-lessons GREEN (16/16 bats); spec convergence: S-6.01 8-pass 19→0, E-7 7-pass 12→0. Released in beta.6 |
| Release v1.0.0-beta.6 | COMPLETE | Tag at ae426cd; PR #8/#10/#11/#12 merged; GH Release published |
| Hotfix: novelty-test fixture path | COMPLETE | PR #10/#11 merged; release workflow re-fire succeeded after fix |
| S-7.03 spec foundation | COMPLETE | 13 BCs + 2 VPs + FR-043 + story (status=ready) + E-7 epic v1.1 |
| S-7.03 adversarial convergence (17 passes) | **CONVERGENCE_REACHED** | trajectory 25→12→5→2→1→0→0→1→2→4→3→1→1→2→0→0→0; ADR-013 satisfied 2026-04-27 |
| S-7.03 GREEN implementation | **COMPLETE** | feat/tdd-discipline-hardening commit 121d24c → squash-merged via PR #13 to 4db2340; 18/18 bats GREEN |
| Release v1.0.0-beta.7 | COMPLETE | Tag at b08e085; PR #14 merged; hotfix PR #15 merged; back-merge PR #16 merged |
| Hotfix: stub-architect.md policy compliance | COMPLETE | PR #15 merged |
| Hiccup: ci.yml/release.yml validation gap | DEFERRED | Tracked as task #98 |
| Wave 1 SS-01 dispatcher-core re-anchor | COMPLETE | 6-pass convergence; 7 stories anchored to 93 unique SS-01 BCs; trajectory 10→4→3→1→0→0 |
| Wave 2 SS-03 sinks re-anchor | COMPLETE | 13-pass convergence; 9 stories anchored; trajectory 11→1→3→0→1→0→1→2→0→1→0→0→0 |
| Wave 3 SS-04 plugin-ecosystem re-anchor | **CONVERGED** pass-6 (9cc5fe7): 0 findings, 3_of_3 NITPICK passes. 8 stories spec-ready. |
| Wave 4 SS-02 hook-sdk re-anchor | **CONVERGED** pass-5 (896cb72): 0 findings, 3_of_3 NITPICK passes. 2 stories spec-ready. |
| Wave 5 SS-06 skill-catalog re-anchor | **CONVERGED** pass-6 (f8e25d3): 1 LOW process-gap carryover (task #112), 3_of_3 NITPICK passes. 2 stories spec-ready. |
| Wave 6 SS-09 configuration & activation re-anchor | **CONVERGED** pass-7 (5f0719c): 0 findings, 3_of_3 NITPICK passes. 6 stories spec-ready. |
| Wave 7 SS-10 CLI tools re-anchor | **CONVERGED** pass-6 (d8054c8): 0 findings, 3_of_3 NITPICK_ONLY. 3 stories spec-ready. |
| Wave 8 SS-08 templates & rules re-anchor | **CONVERGED** pass-4 (f9392c5): 1 LOW pending intent; 3_of_3 NITPICK_ONLY. 3 stories spec-ready. |
| Wave 9 SS-01 straggler re-anchor (S-2.07) | **CONVERGED** pass-4 (61b38a5): 0 findings, 3_of_3 NITPICK_ONLY. 1 story spec-ready. **Cumulative re-anchored: 41/41 stories (100%).** |
| Wave 11 SS-03 spec convergence (S-4.09 + S-4.10) | **CONVERGED at pass-14 (3_of_3 NITPICK_ONLY)** per ADR-013. Trajectory: 14→4→1→2→1(fp)→0→3→4→2→8→3→0→0→0. 2 new stories spec-ready. |
| Phase A — Pre-W16 hardening sprint | **COMPLETE** (PR #65 merged develop @ 844b0e9). TD-013/016/017/018 closed. D-213 sealed. |
| Phase D-1 — W-16 audit (architect) | **COMPLETE** 2026-05-03. audit-w16.md (510L). D-217 sealed. |
| Phase D-2 — ADR-014 + SS-02/SS-04 (architect) | **COMPLETE** 2026-05-03. ADR-014 (343L); SS-02 +139L; SS-04 +58L. D-218 sealed. |
| Phase D-3 — BC-2.02.013 (product-owner) | **COMPLETE** 2026-05-03. BC-2.02.013 (224L, 24 MUSTs). D-219 sealed. (NOTE: BC-2.02.013 subsequently WITHDRAWN in D-224 scope reversal; BC-1.05.035+036 substituted.) |
| Phase B — v1.0.0-rc.4 cut + force-retag | **COMPLETE** (PRs #66+#67; tag @ e93fef7). D-214/215/216 sealed. |
