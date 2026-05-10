# Session Checkpoints — v1.0-feature-plugin-async-semantics-pass-1

Archived session resume checkpoints (superseded by newer entries in STATE.md).

---

## Checkpoint archived 2026-05-08 (superseded by pass-12 checkpoint)

**Last update:** 2026-05-08 — F5 fix-burst-10 COMPLETE. Both pass-11 LOW findings resolved (code/test only; no spec amendments). F-P11-001 [L]: bats Scenario 6 SITE_5 arm added + line-range sed targeting + header cite refresh (test-writer 346a5e6). F-P11-002 [L]: lib.rs:14 doc-comment cite v1.6→v1.7 (implementer 70652a6). ADR-013 clock 0_of_3 — LOW findings do not advance chain. Trajectory: 17→15→6→5→0→2→5→1→4→2→2.

**ACTIVE STEP: F5 pass-12 dispatch — targeting NITPICK_ONLY → 1_of_3. PR held until ADR-013 = 3_of_3.**

**Branches:**
- fix/S-15.01-F5-convergence @ 70652a6 — long-lived; 22 commits ahead of develop; no PR until 3_of_3
- develop @ 6050d24 (F5 fix-burst PR #107 squash-merge 2026-05-08)
- factory-artifacts @ (this burst — see git log)
- main @ fb3e297 (rc.11; behind develop)

**Index versions:** BC-INDEX v1.36 | VP-INDEX v1.22 | STORY-INDEX v2.44 | ARCH-INDEX v1.22
**ADR-013 clock:** **0_of_3** (RESET — pass-6/7/8/9/10 each MEDIUM/HIGH; 3 NITPICK_ONLY required to reach CONVERGED)
**E-9:** v1.53 CONVERGENCE_REACHED (D-308; ADR-013 clock 3_of_3)
**E-10:** paused (D-343); adversary pass-9 queued; resume after plugin-async-semantics F5-F7 complete
**E-10 BC authorship:** COMPLETE (D-313; 13 BCs; total_bcs 1931); finding trend 22→11→16→16→12→2→1→4
**5 user-locked decisions:** (1) envelope sync, (2) no backwards compat, (3) no phased rollout, (4) ASYNC_DRAIN_WINDOW_MS=100ms, (5) WASM-only. Full text in F4-handoff.md §3.
**v1.0.0-rc.13 tag (remote):** PINNED at ba63c9f — INVALID; delete: `git push origin :refs/tags/v1.0.0-rc.13`
**F-7 + F-8:** deferred to cleanup stories #115/#116. Do NOT re-include in adversary scope.

---

## Checkpoint archived 2026-05-09 (superseded by rc.14 ship checkpoint)

**Last update:** 2026-05-09 — CRITICAL FIX LANDED. PR #110 MERGED (squash SHA 80c282f1). Crashed AND Timeout sync gate hooks with on_error=Block now correctly fail-closed (exit 2) per ADR-019 Decision 2. TC-8 (Crashed+Block) + TC-12 (Timeout+Block) integration tests assert correct semantics. Plugin async semantics validation COMPLETE end-to-end.

**ACTIVE STEP:** Plugin async semantics validation COMPLETE. Next: rc.13 release-prep PR.

**Branches:**
- develop @ 80c282f1 (PR #110 squash-merge 2026-05-09 — critical fail-closed fix)
- factory-artifacts @ (see git log)
- main @ fb3e297 (rc.11; behind develop)

**Index versions:** BC-INDEX v1.63 | VP-INDEX v1.40 | STORY-INDEX v2.64 | ARCH-INDEX v1.44
**ADR-013 clock:** 3_of_3 = CONVERGED (2026-05-09 pass-57)
**E-9:** v1.53 CONVERGENCE_REACHED (D-308; ADR-013 clock 3_of_3)
**E-10:** paused (D-343); adversary pass-9 queued; resume after plugin-async-semantics F5-F7 complete
**E-10 BC authorship:** COMPLETE (D-313; 13 BCs; total_bcs 1931 at D-313 (now 1947)); finding trend 22→11→16→16→12→2→1→4
**5 user-locked decisions:** (1) envelope sync, (2) no backwards compat, (3) no phased rollout, (4) ASYNC_DRAIN_WINDOW_MS=100ms, (5) WASM-only. Full text in F4-handoff.md §3.
**v1.0.0-rc.13 tag (remote):** PINNED at ba63c9f — INVALID; delete: `git push origin :refs/tags/v1.0.0-rc.13`
**F-7 + F-8:** deferred to cleanup stories #115/#116. Do NOT re-include in adversary scope.
**Retroactive sweep COMPLETE:** E-6/7/9/10/11 swept in fix-burst-46 (53 BCs: E-6:12, E-7:28, E-10:13; 0 drift). Combined with E-3+E-4+E-5 (fix-burst-43/45), corpus-wide retroactive sweep across all v1.0 epics is now complete.
**Ghost BCs flagged:** BC-3.07.003, BC-3.07.004, BC-1.06.011 — cited in story frontmatter but missing from BC-INDEX and ss-03/. Investigate in future fix-burst.
