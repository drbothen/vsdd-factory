---
document_type: adversarial-review
review_type: spec-cascade
artifact_under_review: "BC-5.39.009 v1.6 + S-15.17 v1.7"
cycle: brownfield-backfill
pass: 7
producer: adversary
timestamp: 2026-05-29
verdict: HIGH
finding_count: 9
finding_count_by_severity:
  critical: 0
  high: 3
  medium: 4
  low: 1
  nitpick: 1
  process_gap: 1
streak_status: "STREAK 0/3 → 0/3 after pass-7 (HIGH; reset). META-LEVEL-34 cure FAILED self-application at fresh-context replay → META-LEVEL-35 CANDIDATE emerges (verification-gate-self-application-asserts-pass-but-gate-predicate-yields-non-empty-stdout-when-replayed)."
---

# Adversarial Review — Pass 7 — S-15.17 Spec Cascade

## Verdict & Top 3 Findings

**Verdict: HIGH** — 9 findings; 3 HIGH (one is META-LEVEL-34/35 regression), 4 MEDIUM, 1 LOW, 1 NITPICK, 1 PROCESS-GAP.

**Top 3:**
1. **F-S15.17-SP7-001 HIGH [META-LEVEL-34 RECURRENCE → META-35 CANDIDATE]** — POLICY 5 v1.3.4 literal-shell verification gate self-application FAILED at fresh-context replay. Story body contains 6+ non-historical `BC v1.5` references that should have been swept to `v1.6`. PO+story-writer claimed "all 4 gates empty = PASS" — re-execution yields non-empty stdout. EXACT META-LEVEL-34 pattern the v1.3.4 cure was meant to prevent.
2. **F-S15.17-SP7-002 HIGH** — BC §SDK Grounding Evidence Grep 1 narrative arithmetic error: claims "all **4 variants**" but listed variants show **5** (CapabilityDenied, Timeout, OutputTooLarge, InvalidArgument, Other(i32)). Self-contradiction inside SDK-grounding evidence.
3. **F-S15.17-SP7-003 HIGH** — Story Risk row line 1087: "BC-5.39.009 v1.5 authored 2026-05-28" — BC is now v1.6 authored 2026-05-29. META-33 sibling-sweep blind-spot (Risk-Mitigation table not in POLICY 5 v1.3.3 categories).

## Part A — Finding Set

### F-S15.17-SP7-001 HIGH [META-LEVEL-34 RECURRENCE → META-LEVEL-35 CANDIDATE]
**Location:** Story narrative + policies.yaml POLICY 5 v1.3.4 gate fails re-execution.

**Evidence:** 6+ non-historical narrative claims with stale BC version refs:
- Line 296 (AC-12): "PC10 is OUT-OF-SCOPE per BC-5.39.009 v1.5 F-SP5-003"
- Line 555 (T-5 code comment): "EQUALITY semantics per BC-5.39.009 v1.5 inv-4"
- Line 603 (T-5 header): "AUTHORITATIVE EXTRACTOR SPECIFICATIONS (from BC-5.39.009 v1.5 PC1-5..."
- Line 691 (T-5 NOTE): "v1.5 introduces marker-prefix discipline"
- Line 1019 (Edge Cases section): "Story EC IDs aligned 1:1 with BC-5.39.009 v1.1 EC IDs"
- Line 1087 (Risk row): "BC-5.39.009 v1.5 authored 2026-05-28"

**META-LEVEL-35 CANDIDATE:** verification-gate-self-application-asserts-pass-but-gate-predicate-yields-non-empty-stdout-when-replayed-by-fresh-context-adversary.

**Routing:** story-writer + product-owner + policy extension.

### F-S15.17-SP7-002 HIGH
**Location:** BC line 705.
**Evidence:** Line 705: "all **4 variants** (CapabilityDenied, Timeout, OutputTooLarge, InvalidArgument, Other(i32))" — but lists **5** variants.
**Routing:** product-owner — fix arithmetic (4 → 5).

### F-S15.17-SP7-003 HIGH
**Location:** Story line 1087 (Risk table).
**Evidence:** Risk title "now that BC-5.39.009 v1.5 is authored" + mitigation "BC-5.39.009 v1.5 authored 2026-05-28" — BC is v1.6 (2026-05-29).
**Routing:** story-writer — update to v1.6. POLICY 5 v1.3.3 categories should be extended with (f) Risk-Mitigation table, (g) Parity Audit Note, (h) LOCAL Adversary Cascade Plan prerequisite refs.

### F-S15.17-SP7-004 MEDIUM
**Location:** BC Grep 10 (lines 778-791).
**Evidence:** Grep 10 stdout is D-518 snapshot (12 in tail); production now D-519 (12→11 in tail). Non-reproducible at fresh-context replay.
**Routing:** product-owner — annotate Grep 10 with timestamp/D-NNN; POLICY 15 amendment.

### F-S15.17-SP7-005 MEDIUM
**Location:** BC §Architecture Anchors lines 615/632/649/665.
**Evidence:** Extractor return types inconsistent — `extract_last_updated_cell` + `extract_session_resume_section_1` return `Option<&str>`; `extract_phase_progress_latest_row` + `extract_concurrent_cycles_latest_row` return `Option<String>`.
**Routing:** product-owner — normalize all to `Option<String>` (owned form).

### F-S15.17-SP7-006 MEDIUM
**Location:** BC PC2 (lines 242-261) and PC5.
**Evidence:** PC2 + PC5 bodies do NOT name their extractor functions; PC3/PC4/PC9 DO name theirs.
**Routing:** product-owner — add explicit function name refs to PC2 + PC5.

### F-S15.17-SP7-007 MEDIUM
**Location:** Story Token Budget line 1006.
**Evidence:** STATE.md estimate ~8,000 stale (current is ~10,000+ tokens after monotonic growth).
**Routing:** story-writer — annotate "estimate reflects D-513 snapshot; recompute at T-5 start".

### F-S15.17-SP7-008 LOW
**Location:** BC line 112 (§Adversary Pass Coverage Pass-6 entry).
**Evidence:** "11 findings 0C+5H+4M+1L+1N+1PG" — breakdown sums to 12; STATE.md treats PG as separate.
**Routing:** product-owner — reformat: "11 findings 0C+5H+4M+1L+1N + 1 process-gap".

### F-S15.17-SP7-009 NITPICK
**Location:** BC last_amended field.
**Evidence:** Single-line YAML scalar ~10,000 chars with 6 nested `[Prior: ...]` blocks.
**Routing:** Deferred per S-15.03 PRIORITY-A scope.

### F-S15.17-SP7-PG-001 PROCESS-GAP HIGH
**Location:** policies.yaml POLICY 5 v1.3.4.
**Evidence:** Cure DID specify verification gate. Pass-6 burst DID claim self-application. Fresh-context replay yields non-empty stdout. Cure-of-cure-of-cure failed at self-application.
**Routing:** product-owner — codify META-LEVEL-35 via POLICY 5 v1.3.4 → v1.3.5 extension: (1) explicit enumeration of "historical-by-construction" sites (only YAML modified[] + ## Changelog rows + [Prior:] nested clauses); (2) adversary-replay-reproducibility mandate (capture parent-commit-SHA + stdout; adversary replay against same SHA must yield identical stdout).

## Part B — Convergence Assessment

### Cure Verification Table

| Cure | Pass-7 Replay | Status |
|------|----------------|--------|
| META-LEVEL-34 (POLICY 5 v1.3.4) literal-shell verification gate | Non-empty stdout (6+ non-historical hits in story) | **FAILED self-application** |
| §Architecture Anchors function names sweep | Verified consistent BC + story | **PASSED** |
| Grep 10 production STATE.md evidence | D-519 production state different (non-reproducible) | **PARTIAL stale snapshot** |
| Token Budget total ~95,000 → ~96,500 | Arithmetic verified | **PASSED** |
| §Adversary Pass Coverage Pass-5+6 entries | Pass-6 count narrative ambiguous (11 + 1PG?) | **PARTIAL narrative ambiguous** |

### Regression Sweep
- **META-LEVEL-34 RECURRENCE** (F-SP7-001): cure failed self-application
- **META-LEVEL-33 RECURRENCE** (F-SP7-003): Risk-Mitigation table not in POLICY 5 v1.3.3 categories — sibling-sweep blind-spot
- No CRITICAL regressions (marker-prefix cure HOLDS — 0 CRITICAL sustained 3 passes)

### META-LEVEL Signals
- **NEW META-LEVEL-35 CANDIDATE:** cure-claim-of-self-application-asserts-empty-stdout-but-replay-by-fresh-context-adversary-yields-non-empty-stdout
- **META-LEVEL-34 CONFIRMED RECURRENT** at pass-7 (codified pass-6; immediately recurred)
- **META-LEVEL-33 CONFIRMED RECURRENT**: POLICY 5 v1.3.3 categories (a)-(e) non-exhaustive

### Convergence Plausibility

**Pass-7 trajectory: 9 findings.** Compared to pass-6 (11) and asymptotic-floor [11-16], this is a **MATERIAL DECREASE to ~9** — first sub-11 since pass-1.

**META-34 cure PARTIALLY worked:** finding count dropped 11→9. But cure itself failed self-application (META-35 emerged).

**SEAL NOT urgent because:** pass-7 trajectory dropped to 9 (below floor [11-16]). However, cure-of-cure-of-cure recursion (META-34 → META-35) is structural evidence.

**Diagnostic next 2 passes:**
- If pass-8 floor < 9 with NO new META class → META-34/35 cure working; convergence plausible
- If pass-8 floor ≥ 9 OR new META class → recursion structural; SEAL becomes production-grade

### Iron Law Attestation

Did NOT read adv-spec-pass-{1,2,3,4,5,6}.md. Findings derive from independent re-reading of BC v1.6, story v1.7, policies.yaml v1.3.4, host.rs, STATE.md, BC-INDEX v2.60, hooks-registry.toml. POLICY 5 v1.3.4 verification gate predicates re-executed with verbatim stdout capture. Findings cite stable anchors per POLICY 5 v1.3.1.
