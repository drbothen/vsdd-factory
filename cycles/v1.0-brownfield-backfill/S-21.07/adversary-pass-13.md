---
pass: 13
verdict: NOT-CLEAN
reviewed_head: 96b4be19158ae27131ae330f684af414821e7c5f
factory_artifacts_head: 40e09b5fe6ed64068d08e0706608f09e1fa375cc
novelty: null
previous_review: "cycles/v1.0-brownfield-backfill/S-21.07/adversary-pass-12.md"
---

## Summary

Pass-13 fresh-context adversarial review. **2 findings: BLOCKER 0 / HIGH 0 / MEDIUM 2 / LOW 0 / NIT 0.** Streak: **0/3** (BC-5.39.001 — remains open; a NOT-CLEAN verdict does not advance the streak). Trajectory pass-10=10, pass-11=1, pass-12=1, pass-13=2 (both un-swept siblings of the F-P12-001 class; novelty MEDIUM — new sites, recurring class per S-7.01 partial-fix regression).

**Reviewed SHAs (stated explicitly):**

| Tree | SHA |
|---|---|
| `feature/S-21.07-validate-cross-site-correspondence` | **`96b4be19158ae27131ae330f684af414821e7c5f`** (unchanged since pass-10 / D-992) |
| `factory-artifacts` | **`40e09b5fe6ed64068d08e0706608f09e1fa375cc`** (D-995 SHA-patch HEAD; reviews story v1.10, STORY-INDEX v4.319 as landed) |

**Tooling disclosure (POLICY 22):** Novelty score not disclosed in the relayed summary for this pass; recorded faithfully as absent rather than fabricated (POLICY 22 relay-fidelity).

---

## Part A — Findings

### Verified RESOLVED (evidence-backed; not findings)

Verified: primary F-P12-001 fix site (AC-020 Notes) correctly propagated (false calibration retracted, fuel_cap prohibition lifted, source-vs-operator framing); three-way input-hash parity holds (7bc1850 across frontmatter/catalog L732/blockquote L741; line-8 S-21.07= tokens are frozen historical transitions, correctly not live). BUT the D-995 fix self-scoped to "AC-020 Notes" (story L58 last_amended) and did not sibling-sweep — two un-swept sibling sites survived.

### MEDIUM

#### F-S2107-P13-001 — MEDIUM — POLICY 8 + POLICY 5 (sibling-sweep) + POLICY 4 (semantic-anchoring); S-7.01 partial-fix regression

**Location:** S-21.07 story — (a) §Out-of-Scope row L761, (b) AC-019 §Build constraints L476.

**Defect:** (a) Out-of-Scope row justifies deferring per-plugin fuel_cap with "this hook uses on_error=continue and needs no fuel_cap override" — the RETRACTED premise (F-P12-001 item 2); contradicts AC-020 L554-556 ("SHOULD be set once ADR-039 Phase 1 ships"); on_error=continue means fuel exhaustion → silent non-finding = broken gate (why the premise was retracted); also anchors to stale ADR-035 §Decision 5 vs the re-anchored ADR-042 §Decision 2 (POLICY 4/19). (b) AC-019 L476 cites flat "10M-instruction fuel cap" with no source-vs-operator qualifier — violates the rule AC-020 L568-571 establishes. Mechanical: `grep '10M|fuel_cap|calibrat'` → L761, L476; story `last_amended` L58 self-scopes fix to "in AC-020 Notes" confirming siblings never swept. Blast radius 1 file → MEDIUM. Routing: story-writer.

#### F-S2107-P13-002 — MEDIUM — POLICY 8 + POLICY 4 + POLICY 14/17 parity + POLICY 5 (sibling-sweep, same-file aggregation cell)

**Location:** STORY-INDEX.md §Epic E-21 "BC coverage" blockquote L742.

**Defect:** coverage blockquote cites BC-5.39.010 v1.14 (S-21.07) while the S-21.07 catalog row (L732) reads v1.18 and BC frontmatter is v1.18 — same-file version disagreement; L743 classifies the coverage blockquote as a LIVE traceability surface (not frozen-historical). D-995 bumped catalog row + delivery-blockquote hash but did not sweep the coverage-blockquote BC pin. Mechanical: `grep 'BC-5.39.010 v1' STORY-INDEX` → L732 v1.18 vs L742 v1.14. Fix: sync L742 → v1.18 or drop the version token (coverage-blockquote BC entries are predominantly versionless). Routing: state-manager (STORY-INDEX owner).

## Observations

**O-P13-01 [process-gap]:** D-995 `last_amended` self-scoped the fix to "AC-020 Notes" but the retracted-claim class is story-wide (recurs at ≥3 sites); a "retracted-claim grep gate" (grep whole story for the retracted phrase-class after any fuel-claim amendment) would close it — companion to POLICY 8.

**O-P13-02 (verification):** three-way input-hash parity 7bc1850 confirmed.

**O-P13-03 (verification):** STORY-INDEX "34 ECs" (story EC count) vs Token Budget "36 ECs" (BC EC count) NOT contradictory (different artifacts); AC=24, VP=19 consistent.

## Coverage (Level-2 partial)

Fully reviewed F-P12-001 fix propagation (all story fuel/BC sites via grep), story AC-001..024 + Out-of-Scope + Token Budget + Tasks, STORY-INDEX row + blockquotes + coverage-blockquote + input-hash, BC self-parity + Preconditions Part A/B. Not exhaustive (budget): BC PC28-PC40 + Gate Spec pseudocode, ADR frontmatter↔body (pass-11/12 RESOLVED), D-995 burst-log Dim-2, feature-branch code (story draft, unbuilt). No findings suppressed.
