---
pass: 61
verdict: CLEAN
severity_summary: "B0/H0/M0/L1"
streak_before: "2/3"
streak_after: "3/3 CONVERGED"
rubric: "policies.yaml v1.4.6"
previous_review: adv-E19-pass-60.md
---

# E-19 Adversarial Review — Pass 61
Rubric: policies.yaml v1.4.6. Perimeter: full E-19 carry-forward (no delta; passes 59–60 CLEAN governance-only). Streak entering: 2/3.

## Finding ID Convention

Findings: F-P61-NNN (blocking). Observations: O-P61-NNN (non-blocking).

## Part A — Findings

No blocking findings (B0/H0/M0).

**O-P61-001 | LOW | POLICY 4 (semantic_anchoring_integrity, v1.4.4 DESCRIPTION-BEARING ANCHOR-PROSE PARITY axis) | `.factory/specs/verification-properties/VP-097.md` §Source Contract (first `**BC:**` bullet)**

Verbatim evidence (ground-truth reads at HEAD):
- VP-097 §Source Contract bullet: `- **BC:** BC-2.07.001 §Invariant 1 — allowlist-bounded file access; absent-but-allowlisted files return NOT_FOUND (-5) not CAPABILITY_DENIED (-1); traversal (`..`) rejected.`
- BC-2.07.001 §Invariant 1 SoT (ground truth): `1. **Traversal defense preserved.** The rejoin algorithm cannot escape the allowlist: it canonicalizes the deepest EXISTING ancestor ..., then appends only the remaining non-existent tail. ... the `starts_with` check catches any escape attempt.` — §Invariant 1 is EXCLUSIVELY traversal defense; it contains no NOT_FOUND/CAPABILITY_DENIED return-code semantics.
- BC-2.07.001 §Postcondition 2 SoT: `**NOT_FOUND returned for allowlisted-but-absent paths.** A `read_file` call on an allowlisted-but-absent path returns `codes::NOT_FOUND (-5)` ...` — the "absent-but-allowlisted files return NOT_FOUND (-5) not CAPABILITY_DENIED (-1)" clause is BC-2.07.001 PC2/PC3 content, not §Invariant 1.
- VP-097 §Traceability bullet for the SAME anchor (clean): `- **BC:** BC-2.07.001 §Invariant 1 — allowlist-bounded file access; traversal rejection` — correctly omits the NOT_FOUND clause.
- VP-097 §Source Contract already carries a separate `- **ADR:** ADR-025 Decision 13 — `codes::NOT_FOUND = -5` distinguishes absent-file from policy-denied.` bullet — so the NOT_FOUND clause inside the §Invariant 1 bullet is both misattributed and redundant.

Violation rationale: The §Source Contract §Invariant 1 bullet's middle clause ("absent-but-allowlisted files return NOT_FOUND (-5) not CAPABILITY_DENIED (-1)") does not derive from the target §Invariant 1 SoT (zero-match against §Invariant 1; it is BC-2.07.001 PC2/PC3 content, which is VP-098's scope, not VP-097's). This is a description-bearing anchor-prose imprecision under POLICY 4 v1.4.4. Severity is LOW rather than MEDIUM because: (a) the anchor ID (§Invariant 1) is correct; (b) the VP's authoritative §Property Statement states the traversal-defense property correctly and unambiguously; (c) the Kani harness (`verify_traversal_defense`) tests traversal defense correctly; (d) the §Traceability bullet for the same anchor is clean. No implementer is misled into building the wrong thing — the mis-anchoring rubric class "label/description stale, actual anchor target correct" = LOW applies (directly parallel to how pass-60 O-P60-001 was classified). The two derived clauses ("allowlist-bounded file access", "traversal rejected") do correctly paraphrase §Invariant 1.

Proposed routing: `vsdd-factory:architect` (VP content owner) — optional in-scope cleanup at next VP-097 touch: drop the "absent-but-allowlisted files return NOT_FOUND (-5) not CAPABILITY_DENIED (-1)" clause from the §Source Contract §Invariant 1 bullet (it is already covered by the sibling ADR-025 Decision 13 bullet), bringing §Source Contract into parity with the clean §Traceability bullet. Non-blocking; does not reset the 3-CLEAN streak.

## Part B — Coverage Attestation

**Perimeter enumeration (versions confirmed by direct read):**

| Artifact | Expected | Actual | Status |
|----------|----------|--------|--------|
| VP-095.md | v1.1 | v1.1 (L5) | PASS — read IN FULL |
| VP-096.md | v1.1 | v1.1 (L5) | PASS — read IN FULL |
| VP-097.md | v1.2 | v1.2 (L5) | PASS — read IN FULL |
| VP-098.md | v1.2 | v1.2 (L5) | PASS — read IN FULL |
| VP-100.md | v1.2 | v1.2 (L5) | PASS — read IN FULL |
| VP-101.md | v1.3 | v1.3 (L5) | PASS — read IN FULL |
| BC-2.07.001 | v1.5 | v1.5 (L4) | PASS — read IN FULL (VP-097/098 SoT) |
| BC-1.17.001 | v1.6 | v1.6 (L4) | PASS — read IN FULL (VP-101 SoT) |
| BC-3.08.001 | v1.21 | v1.21 (L4) | PASS — read L1–494 body + Event 5/Invariant 6 SoT + §VP tables (VP-100 SoT) |
| BC-INDEX | v3.95 | title cells L641/675/736 verbatim-parity confirmed | PASS |
| VP-INDEX | v2.64 / total 101 | total_vps 101; breakdown sums to 101 | PASS |

Versions attested via pass-60 CLEAN carry-forward + no-delta (not re-opened this pass): BC-4.13.001 v1.14, BC-2.02.011 v1.7, BC-5.42.001 v1.6, VP-094 v1.5, ADR-025 v1.15, ADR-030 v1.3, epic E-19 v1.27, stories S-19.01–S-19.07, STORY-INDEX v4.176, ARCH-INDEX v3.00 (VP-094/ADR-030/BC-4.13.001/BC-2.02.011/ADR-025 were pass-60's full-read targets).

**Regions read IN FULL this pass (per dispatch directive, ≥3 under-visited):** VP-095 (STATE.md size-cap integration), VP-096 (extract_frontmatter purity proptest), VP-097 (path_util traversal-defense Kani), VP-098 (allowlisted-absent NOT_FOUND integration), VP-100 (drain-timer plugin.abandoned integration), VP-101 (read_prefix byte-exact-prefix integration) — 6 VP bodies; plus BC-2.07.001 body, BC-1.17.001 body, and BC-3.08.001 body (L1–494 + Event 5/Invariant 6/§VP tables) as the SoT for PC-cite verification.

**Axis sweep:**

| Axis | Result |
|------|--------|
| BC title / subsystem-label sync (POLICY 6/7, D-794) | PASS — BC-INDEX title cells L641 (BC-1.17.001) / L675 (BC-2.07.001) / L736 (BC-3.08.001) match BC H1 verbatim; version cells latest-parity (v1.6/v1.5/v1.21) |
| BC H1 ↔ postcondition consistency | PASS — BC-1.17.001 H1 (NEVER OUTPUT_TOO_LARGE + rejoin + NOT_FOUND) match PC1–PC6; BC-2.07.001 H1 (NOT_FOUND additive + zero false-positive) match PC1–PC5; BC-3.08.001 H1 six-event list matches Events 1–6 |
| VP-INDEX ↔ architecture coherence (POLICY 9) | PASS — total_vps 101; proof-method breakdown 46 unit + 34 integration + 10 manual + 1 static-check + 5 kani-proof + 5 proptest = 101; E-19 (VP-094..101) = 6 integration + 1 proptest (VP-096) + 1 kani-proof (VP-097) = 8; VP-101 catalogued integration-only per F-P28-002 (proptest breakdown correctly omits VP-101) |
| VP PC-cite ↔ BC SoT (POLICY 4/D-812) | PASS — VP-098 PS A/B→BC-2.07.001 PC2, PS C→PC3 (verified); VP-100 PS C mandatory fields (type/trace_id/session_id/plugin_name/entry_index/drain_window_ms/timestamp) match BC-3.08.001 Event 5 verbatim; VP-101 PS A→PC1, PS B→PC3, PS C→PC5 (verified); VP-095/096→BC-4.13.001 §Precondition 3/§Invariant 9 |
| D-817 §Source Contract/§Traceability `**ADR:**` parity | PASS — VP-095/096 both fields name ADR-025 Decision 14; VP-097/098 both name Decision 13; VP-101 both name Decision 13+15; VP-100 cites no ADR (uses DI-019) consistently in both fields; zero "not an ADR-documented decision" denials |
| Semantic anchoring integrity (POLICY 4 v1.4.6) | ONE LOW description-parity imprecision surfaced (O-P61-001, VP-097 §Source Contract §Invariant 1 bullet); all anchor IDs + property statements + harnesses correct |
| Invariant-to-BC orphan (POLICY 2) | PASS — VP-100 domain_invariants:[DI-019] cited in BC-3.08.001 §Traceability L2 Domain Invariants; VP-095/096/097/098/101 domain_invariants:[] consistent with host-ABI-operational convention |
| Volatile-pin / POLICY 19 / D-795 | PASS — all VP source_bc + §Source Contract + §Traceability use stable `§Invariant N`/`§Postcondition N`/`§EC-NNN`/`§Decision N` anchors; zero load-bearing `BC-N.NN.NNN v[0-9]` tokens in active prose (only in historical modified[]/last_amended/Changelog) |
| Story frontmatter-body coherence | PASS (sampled at BC/VP layer) — BC-2.07.001 §VP Anchors ↔ §Verification Properties table ↔ VP-097/098 bcs frontmatter reconcile; BC-1.17.001 ↔ VP-101; BC-3.08.001 ↔ VP-100 |
| Partial-fix regression discipline (S-7.01) | Applied — verified pass-42/43/53 PC-cite corrections propagated: VP-098 (F-P43-004a PS B PC3→PC2, PS C PC4→PC3) and VP-101 (F-P43-004b PS B PC2→PC3, PS C PC3→PC5) confirmed correct against current BC SoT; VP-097/100/101 path-drift (F-P53-001/002) confirmed (host/path_util.rs, host/read_prefix.rs) |

**Standing Gate Roster 1–12:**

| Gate | Result |
|------|--------|
| 1. D-794 BC-INDEX title parity | PASS (BC-1.17.001/BC-2.07.001/BC-3.08.001 title cells verbatim vs H1) |
| 2. D-795 ADR no version-token BC cites | PASS (VP §Source Contract/§Traceability ADR bullets use stable §Decision N form) |
| 3. D-797 VP source_bc volatile-pin sweep | PASS (all E-19 VP source_bc use §Invariant/§Postcondition/§EC stable anchors) |
| 4. D-798 pre-pass class-sweep completeness | PASS (VP-body class + BC-SoT class + BC-INDEX title-cell class swept) |
| 5. D-800 index cells derive from own changelog | PASS (VP-INDEX totals derive from breakdown; BC-INDEX version cells match BC frontmatter) |
| 6. D-801 remediation predicate enumeration | N/A (no fix burst this pass; carry-forward review) |
| 7. D-802 modified[] version-monotonicity | PASS (BC-1.17.001 re-sorted v1.1..v1.6 monotonic per F-P46-001; VP-097 v1.1→v1.2; VP-098 v1.1→v1.2; VP-100 v1.1→v1.2; VP-101 v1.1→v1.3 monotonic) |
| 8. D-803/D-808 epic/index row parity | PASS (BC-3.08.001 §Verification Properties VP-100 row = cardinality+mutual-exclusivity form per VP-INDEX SoT, F-P43-003) |
| 9. D-811 namespace/path sweep | PASS (VP-097 module + harness `host/path_util.rs`; VP-101 `host/read_prefix.rs`; VP-098 `host/read_file.rs` consistent with BC §Architecture Anchors) |
| 10. D-812 PS-* + sentinel/exit match SoT | PASS (VP-098 NOT_FOUND -5/CAPABILITY_DENIED -1 + `internal.file_not_found`; VP-100 mandatory-field set; VP-101 -5/-3 codes; VP-095 OUTPUT_TOO_LARGE -3 all match BC SoT integers/strings) |
| 11. D-815 invocation-signature form matches ADR §Decision | PASS/N-A (VPs 095-101 harnesses invoke factory-dispatcher via envelope pipe / positional probe helpers — no named-flag scripts; ADR-030 script-invocation form applies to VP-094, pass-60 PASS) |
| 12. D-817 §Source Contract/§Traceability `**ADR:**` anchor-field parity | PASS (all E-19 VPs' both fields name governing ADR consistent with §Property Statement; VP-097's LOW imprecision is in the `**BC:**` §Invariant 1 bullet, NOT the `**ADR:**` field — the ADR fields are clean) |

**Do-not-re-report honored:** O-P41-001 (ADR-025 changelog intermediate rows), O-P41-002 (epic §Previous Story Intel ADR-025 provenance), O-P44-001 (BC-3.08.001 VP-100 row case), O-P49-001 (VP-099 S-19.04 scope), O-P60-001 (ADR-025 §Decision intro "ten decisions" stale count) — none re-raised. O-P61-001 is a NEW, distinct observation (VP-097 §Source Contract §Invariant 1 bullet description imprecision), not overlapping any accepted-with-record item.

**Novelty Assessment:** LOW. Zero blocking findings. The E-19 VP-package PC/ADR anchor cites, VP-INDEX arithmetic, and BC-INDEX title parity are clean and faithful to BC/ADR SoT. One genuinely novel LOW observation surfaced by fresh-context full-read of the six under-visited VP bodies (VP-095–101) — VP-097's §Source Contract §Invariant 1 description carries an over-broad NOT_FOUND clause that belongs to PC2/PC3 (VP-098's scope). This is a refinement, not a gap; the anchor, property, and harness are all correct. Spec is at asymptotic convergence.

## Verdict
CLEAN — B0/H0/M0/L1. (CLEAN satisfied: B0/H0/M0; the single L1 observation O-P61-001 is non-blocking and does not reset the 3-CLEAN streak. Streak 2/3 → 3/3.)
