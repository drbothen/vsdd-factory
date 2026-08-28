# S-17.05 LOCAL Adversarial Review Pass 11

**Reviewed artifact:** S-17.05 implementation at `a8d85160` (stamp-state-timestamp PostToolUse hook + factory-lock crates; 27 commits ahead develop)
**Review date:** 2026-08-28
**Verdict:** FINDINGS (1 MEDIUM)
**LOCAL BC-5.39.001 streak:** 0/3 (RESETS — was 1/3 from pass 10 CLEAN)
**D-chain:** none (per per-story local-cascade convention — no D-NNN allocated)

## Part A — Finding Set

**MEDIUM (1):**

- **F-P11-001 (MEDIUM, POLICY 4 / version-cite):** The S-17.05 story body operative `**BC gate:**` header cited stale version tokens: `BC-4.17.001 v1.0` (actual current: v1.28) and `BC-5.40.001 v1.4` (actual current: v1.21). The header also asserted a now-false present-tense `[pending]` traceability claim ("S-17.05 implements BC-5.40.001 PC4 — traceability [pending]"). Ground truth: BC-5.40.001 v1.21 Traceability section lists S-17.05 as a confirmed implementing/anchor story, with this traceability established at v1.10 (F-P25-002). The `[pending]` claim inverts the actual state and would mislead an implementer into re-doing already-done traceability work. **Fixed in-scope this burst** by story-writer: `**BC gate:**` header updated to `BC-4.17.001 v1.28 / BC-5.40.001 v1.21`; false `[pending]` claim removed (replaced with confirmed present-tense traceability statement). Story version bumped v1.6→v1.7. No AC/PC/semantic change — doc-only header correction.

**LOW (2) / ADVISORY (1) — non-blocking, fixed in-scope:**

- **O-P11-1 (LOW, cosmetic):** Story `## Token Budget` section self-label read `S-17.05 v1.1` (stale from the v1.1 draft; should track the current story version). **Fixed in-scope:** story-writer updated self-label to `v1.7`.

- **O-P11-2 (LOW, doc-comment freshness per TD-VSDD-091):** `crates/stamp-state-timestamp/src/main.rs` doc-comments cited specific BC version tokens as volatile pin references (e.g., `BC-4.17.001 v1.28`, `BC-5.40.001 v1.21`, `S-17.05 v1.2`) that will drift on future version bumps. TD-VSDD-091 (anti-volatile-pin) requires citing function names and behavioral anchors, not version numbers. **Fixed in-scope** by implementer: de-pinned all 5 volatile version-token sites; cite stable function names and behavioral roles instead. 32 tests pass; fmt/clippy clean.

- **O-P11-3 (ADVISORY, same class):** Companion doc-comment in `crates/stamp-state-timestamp/src/main.rs` cited `31 unit tests` (should be 32 per the test suite as of `a8d85160`, confirmed by `cargo test`). **Fixed in-scope** by implementer (same commit as O-P11-2 de-pin).

## Part B — Disposition

**VERDICT: FINDINGS (1 MEDIUM).** BC-5.39.001 LOCAL streak **RESETS 1/3 → 0/3**.

**F-P11-001 (MEDIUM):** Fixed in-scope this burst by story-writer. `**BC gate:**` header updated to cite BC-4.17.001 v1.28 / BC-5.40.001 v1.21 (current versions); false `[pending]` claim removed. Story v1.6→v1.7 (doc-only; no AC/PC/semantic change). Input-hash UNCHANGED at `6067e5f` (inputs-array files did not change; compute-input-hash confirmed "already current").

**O-P11-1 (LOW):** Fixed in-scope. Token Budget self-label updated v1.1→v1.7.

**O-P11-2 + O-P11-3 (LOW + ADVISORY):** Fixed in-scope by implementer. Volatile BC + story version tokens de-pinned from 5 doc-comment sites per TD-VSDD-091; test-count comment corrected 31→32. `feature/S-17.05` HEAD advanced `a8d85160` → `a73086a5605c1953a797f8b3520de94730b2c4a4` (doc-comment de-pin commits pushed to origin). 32 tests pass, fmt/clippy clean.

**Novelty:** MEDIUM. F-P11-001 is a semantically significant stale-version-cite in the operative BC gate header (not just a body table cell). The `[pending]` claim inversion is a qualitative step above a mere version lag — it actively misrepresents confirmed traceability.

**Process self-observation (POLICY 14 leg-2 / pass-10 seal gap):** The pass-10 seal (S1705-P10-CLEAN-BURST) bumped story v1.5→v1.6 and updated the STORY-INDEX catalog row, but did NOT add a `v1.6` row to the story Changelog table (the `## Changelog` section at the bottom of the story file). The story-writer discovered this missing row when preparing the v1.7 bump (the `validate-changelog-monotonicity` hook was blocking because the hook expected a v1.6 row before v1.7 could be added). Story-writer backfilled BOTH the missing v1.6 row AND the new v1.7 row in this burst. Going-forward discipline: the seal burst MUST add the Changelog row in the same commit as any story version bump.

Next pass: re-run local adversary **pass 12** (fresh context, against `feature/S-17.05` @ `a73086a5`). Streak at 0/3; need 3 consecutive CLEAN passes (12/13/14) for local 3-CLEAN.
