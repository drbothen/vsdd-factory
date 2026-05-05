# Adversarial Review — E-9 v1.12 Fix Burst (D-250) — Pass 8

**Date:** 2026-05-05
**Commit reviewed:** 353c172 (v1.11 → v1.12; merged D-249 seal + D-250 fix burst per TD-VSDD-064 codified case)
**Cumulative surface:** v1.7..v1.12 (4 files + open-questions.md)
**Verdict:** SUBSTANTIVE
**ADR-013 clock at end of pass:** 0_of_3 reset
**Pass methodology angle:** Story-writer simulation + reverse-derivation outbound-decision-ID semantic-anchor check (NEW per TD-VSDD-057). Mentally played the story-writer authoring S-9.01..S-9.07 + the architect authoring perf-baseline-w16.md, then verified each cross-document decision-ID anchor against its target document by following references outward.

## Summary

Pass-8 found ONE MED cross-document misanchor that survived passes 1-7. perf-baseline-w16.md "Gate Model" section (line 156) cites `E-9 D-9.4 "Option C"` as the source of the latency-primary + advisory-ceiling model. But D-9.4 in the E-9 epic body is "BC Anchor Strategy — reuse existing BC-7.xx family per hook" — its "Option C" reference is back to E-8 D-2 (BC reuse), not bundle ceiling. Authoritative gate-model source is ADR-014 R-8.09 Amendment + E-9 AC-3 (line 368) which explicitly enumerates the latency-primary model. The "E-9 D-9.4 'Option C' +" prefix is a fabricated cross-reference.

Severity gradient near floor: 0 HIGH, 1 MED, 0 LOW. All other surface inspected clean: v1.12 line 38 rewrite (no internal inconsistencies), v1.7 H3 promised changes (all four propagated), changelog summary table append-only invariant (v1.7-v1.12 preserved + v1.13 reserved), frontmatter version coherence (1.12 matches latest non-reserved row), ADR-015 D-15.x clause citations all match source, SS-04/SS-07 frontmatter labels match ARCH-INDEX exactly, OQ-W16-001 acceptance criterion well-formed.

## Findings

### HIGH

(none)

### MED

**M-P8-001 [MED]: Cross-document decision-ID misanchor — perf-baseline cites non-existent "E-9 D-9.4 Option C" for gate model**

- File: `/Users/jmagady/Dev/vsdd-factory/.factory/architecture/perf-baseline-w16.md` line 154 + 156.
- Evidence: line 154 `## W-16 Gate Model (ADR-014 R-8.09 Revised — Option C)`; line 156 `Reference: E-9 D-9.4 "Option C" + ADR-014 R-8.09 (Amendment 2026-05-03).`
- Verification: E-9 line 314 `### D-9.4: BC Anchor Strategy — reuse existing BC-7.xx family per hook`; line 316 `Mirrors E-8 D-2 Option C: reuse existing BCs; no new BC family.` E-9 D-9.4 contains no gate-model "Option C" decision; the "Option C" cited is a back-reference to E-8 D-2 (BC reuse).
- Impact: story-writer authoring S-9.01..S-9.07 following the perf-baseline anchor would look in E-9 D-9.4 expecting a gate-model decision, find a BC anchor strategy decision instead, and either (a) get confused, or (b) mis-cite D-9.4 as gate-model authority in story bodies, propagating the error.
- Why prior passes missed it: decision-ID looked plausible ("Option C" recurs through the surface) but D-9.4's semantic content was never cross-checked against this perf-baseline reference. Passes 1-7 focused on E-9 internal consistency, ADR-015 citations, POLICY 1 — not on outbound references from arch docs to epics.
- Fix: replace `E-9 D-9.4 "Option C" +` with `E-9 AC-3 +` (AC-3 at E-9 line 368 explicitly enumerates the latency-primary model) OR drop the E-9 reference entirely.

### LOW

(none)

## Out-of-scope-but-noted

- L-P7-002 (still deferred): audit-w16 line 38 `**Standard.**` label asymmetric for row containing 2 block-mode hooks. v1.12 H3 explicitly defers.
- M-P6-001 (still deferred): frontmatter convention drift across 3 arch docs. Deferred per D-244/D-247.

## Process-gaps

- **[process-gap] PG-P8-001:** Outbound cross-document decision-ID anchors not validated by any pre-commit hook or convergence check. TD-VSDD-058 catches ADR-015 D-15.x citations. TD-VSDD-063 catches fix-burst nomenclature leakage. No equivalent check that "decision IDs cited from arch docs to epic docs (D-9.x, AC-N) actually correspond to cited semantic content in target." M-P8-001 is the class this gap creates. Codify as TD-VSDD-065 (Decision-ID outbound semantic-anchor check).

## Convention checks

- Frontmatter `version:` matches latest non-reserved row (1.12): PASS
- v1.7-v1.12 summary rows intact (POLICY 1): PASS
- v1.13 preemptive reserved row: PASS
- v1.12 H3 section present: PASS
- v1.7-v1.11 block prose preserved as authored (POLICY 1 append-only): PASS — including v1.10 explicit preservation of v1.8 historical "Wave 3 AC-3" wording per D-245
- No "Lines: X → Y" footer at v1.7-v1.12: PASS
- H3 version count matches summary table: PASS (6 H3s for v1.7-v1.12)
- audit-w16 line 38 sibling-wording-template consistency: PASS
- No fix-burst-internal IDs leak into permanent specs body: PASS

## Angle-specific outputs

Story-writer simulation walk-through (the angle that found M-P8-001):

Imagined being story-writer authoring S-9.04 (validate-state-size). Story body needs a "Bundle Size Telemetry" AC. Consult parent epic E-9 AC-3 (line 368) — tells me to publish `bundle_size_delta_bytes`. AC-3 references "ADR-014 R-8.09 revised model" + "S-9.00 baseline." Open perf-baseline-w16.md to get canonical baseline values. Section "W-16 Gate Model" line 154 cites `Reference: E-9 D-9.4 "Option C" + ADR-014 R-8.09 (Amendment 2026-05-03).` Open E-9 looking for D-9.4 "Option C" gate model — find D-9.4 is BC anchor strategy. Either waste 5-10 min resolving, or mis-cite D-9.4 in story body. Both bad. Error invisible until fresh reader does same lookup.

Reverse-derivation tooling check:

Future "is-this-amendment-ready-for-production?" tool walking outbound decision-ID references and asserting target-heading semantic compatibility: perf-baseline-w16.md line 156 → E-9 D-9.4 would FAIL (target heading "BC Anchor Strategy" not semantically compatible with reference context "W-16 Gate Model"). All other outbound refs sampled (audit-w16 → ADR-015; gap-analysis → ADR-015; epic D-9.x → ADR-014; perf-baseline → ADR-014 R-8.09) PASS.
