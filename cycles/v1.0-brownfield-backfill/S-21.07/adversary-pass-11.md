---
pass: 11
verdict: NOT-CLEAN
reviewed_head: 96b4be19158ae27131ae330f684af414821e7c5f
factory_artifacts_head: 33771c8176adac11ca0f59274051c42daa26a7c9
novelty: null
previous_review: "cycles/v1.0-brownfield-backfill/S-21.07/adversary-pass-10.md"
---

## Summary

Pass-11 fresh-context adversarial review. **1 finding: BLOCKER 0 / HIGH 0 / MEDIUM 1 / LOW 0 / NIT 0.** Streak: **0/3** (BC-5.39.001 — remains open; a NOT-CLEAN verdict does not advance the streak). Trajectory `47→18→25→25→24→20→16→8→10→1` (tail `→16→8→10→1`).

**Reviewed SHAs (stated explicitly):**

| Tree | SHA |
|---|---|
| `feature/S-21.07-validate-cross-site-correspondence` | **`96b4be19158ae27131ae330f684af414821e7c5f`** (unchanged since pass-10 / D-992) |
| `factory-artifacts` | **`33771c8176adac11ca0f59274051c42daa26a7c9`** (D-993 SHA-patch HEAD; reviews ADR-040 v1.15, ARCH-INDEX v3.57 as landed) |

**Tooling disclosure (POLICY 22):** Novelty score not disclosed in the relayed summary for this pass; recorded faithfully as absent rather than fabricated (POLICY 22 relay-fidelity). Precise this pass's scope was narrowly targeted at re-verifying the D-992/D-993 fix-burst content against the pass-10 finding set.

---

## Part A — Findings

### Pass-10 defect classes independently re-verified as RESOLVED (evidence-backed; not findings)

F-004 fuel-cap tautology **RESOLVED** (`DEFAULT_FUEL_CAP=20_000_000` at `invoke.rs`, both defaults source it, pinned by `fuel_cap_defaults_stay_in_sync`; BC-5.39.010 v1.18 states source-HEAD-vs-operator-effective honestly and the `(12M,20M)` failing region is non-tautological). F-001/F-002/F-003 POLICY 15 gate **REDESIGNED** as `crates/policy15-attestation-gate/` Rust crate; CI job not yet wired to `ci.yml` but disclosed as anchored deferral Drift Item `[D-969]` + ADR-040 §Implementation one-remaining-OUTSTANDING — a disclosed hole, not a finding. F-007 fuel-vs-epoch **RESOLVED** (`main.rs` distinct `FUEL_EXHAUSTED` sentinel, commit `62fbcf1a`). F-009 erratum leg-3 **RESOLVED** (POLICY 14 v1.4.24 ERRATUM-ROW MODIFIED[] PARITY CONVENTION). F-005 ADR ratification **MOSTLY RESOLVED** (ADR-041 v1.2 / ADR-042 v1.4 frontmatter active+ratified 2026-08-13; §Status bodies cleanly reconciled with HISTORICAL—RESOLVED labels) — **EXCEPT ADR-040** (the pass-11 finding below).

### MEDIUM

#### F-S2107-P11-001 — MEDIUM — POLICY 4 (semantic-anchoring-integrity) + S-7.01 partial-fix regression discipline

**Location:** ADR-040 `## Proposed policies.yaml Replacement Text` preamble + `### Status as of v1.3` + `### Status as of v1.4`.

**Defect:** ADR-040's v1.13→v1.15 body-vs-frontmatter reconciliation superseded only the trailing §Status paragraph, leaving three sibling sites carrying live un-superseded "Do NOT apply until re-ratification / MUST NOT be edited to v1.4.23" directives that contradict the ratified/applied state (ADR-040 frontmatter `status: active` / `ratified: 2026-08-10`, D-970; `policies.yaml` already at v1.4.24 with the ATTESTATION-LOCATION GATE bullet applied at v1.4.23). Sibling ADR-041/042 received the complete labeling pattern; ADR-040 got 1 of 4 sites.

**Blast radius:** 1 file / multiple sites → MEDIUM.

**Routing:** architect (DONE — see D-994 fix burst; ADR-040 v1.16 bundled).

---

## Observations (non-blocking)

**O-P11-01 (pending intent verification):** ARCH-INDEX ADR-042 §Decision 1 cell `~415KB prefix` reads as measurement-provenance (fixture size at which the 9,920,913-fuel baseline was measured), not a decaying live pointer; adversary did NOT raise it as a POLICY 5/TD-VSDD-091 finding but flags intent ambiguity for adjudication — if the registry-row convention forbids any `~NNNKB` snapshot regardless of provenance framing it is an open F-008 leg; if measurement-provenance figures are permitted it is compliant. The two ADR-body legs of F-008 (ADR-040 "line 294", ADR-042 "line 1464") confirmed remediated.

**O-P11-02 [process-gap]:** the `## Proposed policies.yaml Replacement Text` pattern (ratified proposal whose preamble retains a "Do NOT apply until re-ratification" directive) has no reconciliation convention; a one-line checklist item "on ADR ratification, sweep EVERY 'Do NOT apply / re-ratification required' string in the body, not only the §Status paragraph" would close this recurrence class.
