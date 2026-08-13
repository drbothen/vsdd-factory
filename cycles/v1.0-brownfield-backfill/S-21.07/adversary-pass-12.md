---
pass: 12
verdict: NOT-CLEAN
reviewed_head: 96b4be19158ae27131ae330f684af414821e7c5f
factory_artifacts_head: 1147b554523b34df7a7e5321b776db0ae0ebedbb
novelty: null
previous_review: "cycles/v1.0-brownfield-backfill/S-21.07/adversary-pass-11.md"
---

## Summary

Pass-12 fresh-context adversarial review. **1 finding: BLOCKER 0 / HIGH 0 / MEDIUM 1 / LOW 0 / NIT 0.** Streak: **0/3** (BC-5.39.001 — remains open; a NOT-CLEAN verdict does not advance the streak). Trajectory `47→18→25→25→24→20→16→8→10→1→1` (tail `→10→1→1`, locus moved ADR-layer→story-layer; novelty MEDIUM).

**Reviewed SHAs (stated explicitly):**

| Tree | SHA |
|---|---|
| `feature/S-21.07-validate-cross-site-correspondence` | **`96b4be19158ae27131ae330f684af414821e7c5f`** (unchanged since pass-10 / D-992) |
| `factory-artifacts` | **`1147b554523b34df7a7e5321b776db0ae0ebedbb`** (D-994 SHA-patch HEAD; reviews ADR-040 v1.16, ARCH-INDEX v3.58 as landed) |

**Tooling disclosure (POLICY 22):** Novelty score not disclosed in the relayed summary for this pass; recorded faithfully as absent rather than fabricated (POLICY 22 relay-fidelity).

---

## Part A — Findings

### Verified RESOLVED (evidence-backed; not findings)

Pass-11 F-S2107-P11-001 (ADR-040 three sibling sites now SUPERSEDED w/ blockquote+strikethrough, grep-confirmed no live directives outside labeled-historical blocks); BC-5.39.010 v1.18 fuel-cap claim accurate (`invoke.rs:279` `DEFAULT_FUEL_CAP=20_000_000`, both defaults source it, `fuel_cap_defaults_stay_in_sync` test exists, source-HEAD-vs-operator honestly stated); POLICY 15 gate [D-969] honestly-disclosed deferral (no policy-15/attestation job in `.github/workflows/`; crate exists); O-P11-01 ARCH-INDEX `~415KB` now provenance-framed; ADR-041/042 frontmatter↔body consistent; STATE.md streak 0/3 internally consistent.

### MEDIUM

#### F-S2107-P12-001 — MEDIUM — POLICY 8 (bc-array/version propagation) + POLICY 4 (semantic-anchoring)

**Location:** `S-21.07-validate-cross-site-correspondence.md` — AC-019 §Build constraints, AC-020 Notes, + title/frontmatter/body BC-version cites pinned at v1.14.

**Defect:** story (v1.9, edited 2026-08-07) still cites governing BC-5.39.010 at v1.14 and carries fuel-cap claims the BC (v1.15..v1.18) has superseded/retracted. Three drifts: (1) story asserts `max_bytes` caps "calibrated to keep reads within the 10M fuel budget" — BC v1.15 (F-S2107-P9-002) RETRACTED this verbatim as FALSE (1 MiB PC4 cap ~76% above ~594 KB exhaustion point; measured fuel 9,920,913/10,000,000 = 99.21%); story repeats the exact FALSE claim. (2) story instructs "No `fuel_cap` field: MUST NOT include a `fuel_cap` registry field" citing BC v1.14 §Gate Spec + ADR-035 §Decision 5 — BC v1.16 LIFTED the prohibition (ADR-042 §Decision 2; SHOULD set per-plugin cap once ADR-039 Phase 1 ships); cited anchors no longer support the instruction. (3) story asserts flat 10M budget per ADR-035 §Decision 5 — ADR-035 v1.1 §Decision 5 documents 10M→20M (`invoke.rs`=20_000_000); story lacks the source-HEAD-vs-operator-effective nuance BC v1.18 was corrected (F-S2107-P10-004) to state. Mechanical: BC frontmatter version 1.18; BC `modified[]` v1.15-v1.18 post-date story last edit; BC-INDEX v4.58 propagated leg-5 but story is sole un-propagated downstream site. Severity MEDIUM (operator-level 10M still in force through rc.23, per-plugin `fuel_cap` genuinely unavailable, so items 2-3 partially operator-accurate; item 1 unconditionally wrong, borders HIGH). Routing: story-writer POLICY 8 propagation (DONE — see fix).

## Observations

**O-P12-01 (verification):** pass-11 F-P11-001 confirmed RESOLVED, O-P11-02 codified as `L-BB-adr-reconciliation-sweep-scope-on-ratification`.

**O-P12-02 (verification):** POLICY 15 gate honestly-anchored deferral `[D-969]`.

**O-P12-03 [process-gap]:** no convention gating "narrative-only BC amendment → downstream story-propagation obligation"; the datum-copy rule lets a story silently lag a governing BC across multiple versions with no drift signal. A rule that a governing-BC bump touching §Gate-Spec normative prose enqueues a downstream story-propagation task would close this recurrence class (companion to POLICY 8).

## Coverage (Level-2 partial)

Fully reviewed: ADR-040 directives, ADR-041/042 frontmatter↔body, BC-5.39.010 + code, BC-INDEX leg-5, ARCH-INDEX rows + 415KB, `.github/workflows` gate wiring, STATE.md streak/rows, story fuel/BC-cite sites. Not exhaustively audited (budget): full BC PC1-PC40 body, D-994 burst-log Dim-2 line-by-line (pass-11 fix was documentary, low D-449 exposure), VP-INDEX/STORY-INDEX arithmetic. No findings suppressed.
