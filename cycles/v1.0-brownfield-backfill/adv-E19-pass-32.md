# Adversarial Review — E-19 Pass 32 (post-D-785 delta; perimeter = epic v1.20 + full E-19 suite at D-785 versions)

**Perimeter:** E-19 epic v1.20 + S-19.01..S-19.07 (at D-785 versions) + STORY-INDEX E-19 section + VP-INDEX VP-094..VP-101 + BC-5.42.001 v1.4 + BC-4.13.001 v1.12 + BC-2.07.001 v1.3 + BC-2.02.011 v1.5 + BC-3.08.001 v1.19 + BC-1.17.001 v1.5 + VP-095.md v1.1 + VP-096.md v1.1 + ADR-025 v1.10 (§Decision 15 in scope as read_prefix governance)
**Reviewer:** fresh-context adversary (Iron Law; rubric = policies.yaml v1.4.1)
**Date:** 2026-07-09
**Verdict:** NOT-CLEAN — BLOCKER 0 / HIGH 0 / MEDIUM 1 / LOW 2 (3 total)
**Streak:** 0/3 (pass-32 NOT-CLEAN; single-digit total; all items CLOSED in D-786 fix burst)
**Model family:** Claude Sonnet 4.6
**Delta artifact versions verified:** epic v1.20 (was v1.19; F-P31-001 §Out of Scope BC-1.17.001 bullet corrected).

## Part A — D-785 Delta Verification + New Findings

### Amendment 1 — E-19 epic v1.19 → v1.20 (F-P31-001: §Out of Scope BC-1.17.001 bullet "LANDED as v1.3" corrected to "LANDED as v1.5")

F-P31-001 fix applied — §Out of Scope BC-1.17.001 bullet introductory phrase corrected: "LANDED as v1.3" → "LANDED as v1.5 (subsequently amended through v1.5 — see BC changelog)" ✓. Input-hash 68a89c0 (unchanged) ✓. POLICY 14 5-leg parity confirmed ✓. STORY-INDEX E-19 section header updated to v1.20 ✓.

One new finding identified in the v1.20 correction text:

**O-P32-02 LOW — E-19 epic v1.20 §Out of Scope BC-1.17.001 bullet — tautological "subsequently amended through v1.5" clause when bullet already states "LANDED as v1.5".**

The corrected v1.20 text reads: "LANDED as v1.5 (subsequently amended through v1.5 — see BC changelog)". The parenthetical "subsequently amended through v1.5" is tautological: the bullet already declares "LANDED as v1.5", so stating it was "subsequently amended through v1.5" adds no information — the BC arrived at v1.5 and was amended through v1.5 are the same version. The "subsequently amended" phrasing carries meaning only when the amendment endpoint differs from the landing version (e.g., "LANDED as v1.2, subsequently amended through v1.5"). Retaining the clause for "LANDED as v1.5" introduces a self-referential construction that will confuse the implementer about which version is canonical.

**Locus:** E-19 epic v1.20 §Out of Scope BC-1.17.001 bullet — parenthetical "(subsequently amended through v1.5 — see BC changelog)".
**Routing:** story-writer (epic body content — §Out of Scope bullet text).
**Fix:** Story-writer epic v1.20→v1.21 — drop the tautological parenthetical; retain only "LANDED as v1.5". Input-hash 77985d8 (computed post-fix). **CLOSED O-P32-02.**

### Full E-19 Suite Review — ADR-025 §Decision 15 + BC-2.07.001 + Story Suite

**F-P32-001 MEDIUM — ADR-025 v1.10 §Decision 15 body is stale relative to the Phase-B migration defined by BC-4.13.001 and implemented by S-19.07.**

ADR-025 §Decision 15 establishes `host::read_prefix` as an additive host function. The §Decision 15 body contains two stale references relative to the completed Phase-B migration:

(a) **Primary consumers paragraph:** The paragraph currently describes `STATE_MD_MAX_BYTES` as the byte budget used by `verify-factory-lock` when calling `read_prefix` and implies this constant persists post-migration. However, BC-4.13.001 Phase-B (the normative contract for S-19.07) specifies that `STATE_MD_MAX_BYTES` is removed entirely at S-19.07 — the Phase-B migration removes the constant and replaces it with the inline `max_bytes=8192` call-site argument per §Precondition 3. After S-19.07 lands, `STATE_MD_MAX_BYTES` will not exist. The §Decision 15 primary-consumers description must reflect Phase-B: `read_prefix` is called with `max_bytes=8192` (per BC-4.13.001 §Precondition 3 Phase-B) as the sole post-migration read bound; `STATE_MD_MAX_BYTES` is removed.

(b) **Truncation-example sentence:** The §Decision 15 truncation example cites "262144 bytes" as a `read_prefix` bound: "any content beyond 262144 bytes is silently truncated". This is incorrect — 262144 bytes is the Decision 14 `host::read_file` OUTPUT_TOO_LARGE cap (a hard error threshold), not a `read_prefix` argument. `read_prefix` takes an explicit `max_bytes` parameter; the Phase-B `max_bytes` is 8192 (per BC-4.13.001 §Precondition 3), not 262144. The 262144 figure is Phase-A-historical and should be explicitly labeled as such to prevent implementers from confusing the Phase-A cap with the Phase-B `read_prefix` argument.

**Locus:** ADR-025 v1.10 §Decision 15 body — (a) Primary consumers paragraph; (b) truncation-example sentence.
**Routing:** architect (ADR-025 owner; §Decision 15 amendment).
**Fix:** Architect ADR-025 v1.10→v1.11: (a) Primary consumers paragraph reworded — STATE_MD_MAX_BYTES removed entirely at S-19.07; read_prefix max_bytes=8192 per BC-4.13.001 §Precondition 3 Phase-B is the sole post-migration read bound. (b) Truncation-example reframed from Phase-A 262144 cap to Phase-B 8192 bound; 262144 marked Phase-A-historical. **CLOSED F-P32-001.**

**O-P32-01 LOW — BC-2.07.001 v1.3 §Traceability §L2 Domain Invariants cell retains "DI-TBD" placeholder — pass-30 sibling-sweep miss.**

BC-2.07.001 v1.3 §Traceability section contains `L2 Domain Invariants: DI-TBD`. This placeholder was established in the original v1.0 authoring and was not retired when the pass-30 fix burst (D-784) retired the same placeholder in BC-1.17.001 v1.5 (`domain_invariants: []` none). The convention established by BC-1.17.001 v1.5 and BC-4.13.001 is that the `domain_invariants:` field must be an affirmative statement: either a list of invariant IDs or `[] (none)` when no domain invariants apply. For BC-2.07.001 (`host::read_file absent-file semantics: codes::NOT_FOUND (-5)`), there are no applicable L2 Domain Invariants — the NOT_FOUND error code is an additive host-ABI convention, not a domain invariant in the L2 business-logic sense. The D-784 pass-30 sibling-sweep retired `DI-TBD` in BC-1.17.001 but did not extend to BC-2.07.001, which had been authored at the same session with the same placeholder pattern.

**Locus:** BC-2.07.001 v1.3 §Traceability §L2 Domain Invariants — value "DI-TBD".
**Routing:** product-owner (BC body content); story-writer (S-19.03 BC-2.07.001 cite propagation).
**Fix:** Product-owner BC-2.07.001 v1.3→v1.4: §Traceability L2 Domain Invariants DI-TBD → none (host-ABI operational; no L2 domain invariants applicable); aligned to BC-1.17.001/BC-4.13.001 convention; input-hash 9d60fc5. Story-writer S-19.03 v1.15→v1.16: BC-2.07.001 v1.3→v1.4 cite sweep ×3 sites (BC table Version cell, AC-001 gate, Token Budget); input-hash 8d1225d unchanged. **CLOSED O-P32-01.**

### Full Story Suite Verification

All D-785 amendments verified closed as documented above. No further findings in the full E-19 story suite (S-19.01 v1.15 / S-19.02 v1.15 / S-19.03 v1.15 / S-19.04 v1.11 / S-19.05 v1.14 / S-19.06 v1.17 / S-19.07 v1.14). STORY-INDEX E-19 section verified consistent with story versions (post-D-785). BC-INDEX v3.84 / VP-INDEX v2.55 / ARCH-INDEX v2.95 verified consistent (pre-D-786 fix burst). No POLICY violations detected beyond the three items above.

## Part B — Severity + Novelty

**Severity (B0/H0/M1/L2):** One MEDIUM finding (F-P32-001) and two LOW observations (O-P32-01, O-P32-02). Total: 3 items. Severity increase from pass-31 (1 total) — regression from 1 item to 3 items, though all severity levels are LOW/MEDIUM with no HIGH or BLOCKER. The F-P32-001 MEDIUM is a substantive spec accuracy issue (ADR §Decision 15 stale relative to BC-4.13.001 Phase-B); the two LOWs are documentation hygiene issues.

**Novelty:** LOW-MEDIUM. O-P32-02 (tautological clause) is a new sub-class of the partial-sweep-escape pattern: the D-785 fix burst introduced a self-referential construction while closing F-P31-001 — a "fix introduces a different gap" pattern distinct from the prior "sweep misses a site" pattern. F-P32-001 (ADR §Decision 15 Phase-B stale) is a genuine new finding: the Phase-B migration removes `STATE_MD_MAX_BYTES` and changes the normative bound from 262144 to 8192, and the ADR body did not track this when BC-4.13.001 Phase-B was authored. O-P32-01 (DI-TBD sibling-sweep miss) is a recurrence of the sibling-sweep-miss class first observed at pass-30 (D-784 swept BC-1.17.001 but not BC-2.07.001). No new structural defect class beyond the tautological-fix pattern (O-P32-02).

**Cascade trajectory (pass-22 onward, count):** 4→3→4→2→2→4→6→5→4→1→3. Trajectory tail (passes 29-32): →5→4→1→3. Upward tick at pass-32 (1→3) from a one-finding low. Asymptotic floor remains low-single-digit; pass-32 regression is driven by three distinct independently-routable items, all closed.

## Fix Burst Closure (D-786)

**Fix burst D-786 applied.** Architect ADR-025 v1.10→v1.11 (F-P32-001: §Decision 15 body corrected — STATE_MD_MAX_BYTES removal + 262144 Phase-A-historical framing). Product-owner BC-2.07.001 v1.3→v1.4 (O-P32-01: DI-TBD → none; input-hash 9d60fc5). Story-writer S-19.03 v1.15→v1.16 (BC-2.07.001 v1.3→v1.4 cite sweep ×3 sites; input-hash 8d1225d unchanged). Story-writer epic v1.20→v1.21 (O-P32-02: tautological clause dropped; input-hash 77985d8). ARCH-INDEX v2.95→v2.96 (SM: ADR-025 v1.11 row note). BC-INDEX v3.84→v3.85 (SM: BC-2.07.001 v1.4 row note). STORY-INDEX v4.163→v4.164 (SM: epic header v1.21; S-19.03 row v1.16; DAG footnote pass-32 note; BC coverage v1.4). VP-INDEX v2.55 UNCHANGED. STATE.md v5.36→v5.37 (SM: D-786 advance; trajectory →5→4→1→3; checkpoint refresh). Streak 0/3. NEXT: E-19 adv pass-33 (fresh context).
