---
document_type: adversarial-review
review_type: spec-cascade
artifact_under_review: "BC-5.39.009 v1.8 + S-15.17 v1.9"
cycle: brownfield-backfill
pass: 9
producer: adversary
timestamp: 2026-05-29
verdict: HIGH
finding_count: 9
finding_count_by_severity:
  critical: 0
  high: 4
  medium: 3
  low: 1
  nitpick: 1
  process_gap: 0
streak_status: "STREAK 0/3 → 0/3 after pass-9 (HIGH; reset). META-LEVEL-37 CANDIDATE emerges (scalar-snapshot-of-cardinality-presented-as-structural-form-invariant-but-disowned-in-prose). DIAGNOSTIC RESULT: META-36 cure POLICY 5 v1.3.6 DID NOT bottom out recursion; SEAL ADJUDICATION RECOMMENDED."
---

# Adversarial Review — Pass 9 — DIAGNOSTIC

**Iron Law Attestation (BC-5.39.001):** Did NOT read `.factory/code-delivery/S-15.17/adv-spec-pass-{1..8}.md`. Fresh-context.

## Part A — Findings

### F-S15.17-SP9-001 HIGH [META-LEVEL-37 CANDIDATE] — Grep 10 scalar `16` non-reproducible at HEAD (now `17`); POLICY 5 v1.3.6 Part B self-violation
**File:** BC §SDK Grounding Evidence Grep 10 (L787-795)
**Evidence:** BC v1.8 Grep 10 captures `16` trajectory-tail occurrences. Live re-execution at HEAD: `17`. Scalar drifted between PO commit and adversary replay. POLICY 5 v1.3.6 Part B forbids snapshot-annotation-only; mandates HEAD-reproducible OR structural-form-only. Captured number `16` is neither (a) HEAD-reproducible (now 17) nor (b) structural-form (a scalar count is a snapshot of cardinality, NOT a structural invariant). The structural form is "count > 0", not "count == 16".
**META-37 candidate:** scalar-snapshot-of-cardinality-presented-as-structural-form-invariant-but-disowned-in-prose. Distinct from META-36 (SHA-pinning) — captures a NUMERICAL VALUE and calls the value structural.
**Routing:** product-owner — POLICY 5 v1.3.7: forbid captured-stdout numerical scalars; mandate predicate-only narrative ("grep ... → non-zero asserted"). Cure-of-cure-of-cure recursion at level 7.

### F-S15.17-SP9-002 HIGH [paper-fix residual; TD-VSDD-059] — PC10 OUT-OF-SCOPE rationale is structurally false; `^## LESSON-` is valid anchor parallel to `^### Dim-7`
**File:** BC L399-401
**Evidence:** PC10 OUT-OF-SCOPE annotation claims lessons.md has "NO structural anchor." Live verification: `grep -c "trajectory-tail" lessons.md` = 12; `^## LESSON-` anchor count = 50. The structural anchor IS `^## LESSON-` (parallel to PC9's `^### Dim-7`). PC9 cure approach trivially applies: bottommost lesson + marker-prefix check + absent-marker = log_warn advisory. The OUT-OF-SCOPE is a paper-fix renaming ("hard to spec" → "no structural anchor") admitting the wrong rationale.
**Routing:** product-owner — actually ground PC10 to `^## LESSON-` bottommost block + marker-prefix two-step check; lessons.md arm is symmetric to burst-log arm.

### F-S15.17-SP9-003 HIGH [POLICY 5 v1.3.4 sibling-sweep gate violation; META-33 recurrence] — §Architecture Anchors claims "All 7 extractors return Option<String>" but embedded T-5 NOTE signatures still `Option<&str>`
**File:** BC L627 + embedded T-5 NOTE comments at BC L632 + BC L682
**Evidence:** L627 rationale says all 7 extractors return `Option<String>`. L632 + L682 embedded pseudocode comments for `extract_last_updated_cell` + `extract_session_resume_section_1` still show `Option<&str>`. POLICY 5 v1.3.4 sibling-sweep gate would detect this with grep. Pass-8 sweep claim incomplete.
**Routing:** product-owner — sibling-sweep embedded T-5 NOTE signatures; literal-shell verification gate at HEAD before commit.

### F-S15.17-SP9-004 HIGH — Story Risk L1108 falsely conflates "v1.8 amendment date 2026-05-29" with "BC authored" semantic (BC v1.0 authored 2026-05-28)
**File:** Story Risk row L1108
**Evidence:** "BC-5.39.009 v1.8 authored 2026-05-29" — but BC was authored 2026-05-28 (v1.0 changelog row). Pass-7 sweep distorted "authored" semantic during version-bump propagation.
**Routing:** story-writer — Risk row: "BC-5.39.009 v1.8 (latest amendment 2026-05-29; original authoring 2026-05-28)" OR "BC-5.39.009 v1.8 (2026-05-29 amendment)".

### F-S15.17-SP9-005 MEDIUM — PC2 NOTE claims "D-518 dispatch templates updated" without literal-shell SDK-grounding evidence; POLICY 5 v1.3 violation
**File:** BC L632 PC2 NOTE
**Evidence:** Load-bearing claim about state-manager dispatch template content; no Grep in §SDK Grounding Evidence verifies template body contains marker. Effect (marker in STATE.md) is grounded (Grep 10); cause (template update) is not.
**Routing:** product-owner — add Grep grounding the dispatch template OR rephrase to effect-only ("marker present in production STATE.md per Grep 10").

### F-S15.17-SP9-006 MEDIUM — D-453(d) Site 8 enumerates 3 Dim-7 heading variants; HEAD has 6 variants (parentheticals missed)
**File:** BC L97 (D-453(d) Site 8)
**Evidence:** Live grep shows 6 actual variants; BC enumerates 3. Forward-compatible (PC9 uses `^### Dim-7` prefix-match) but enumeration narrative stale per POLICY 5 v1.3.6 Part B.
**Routing:** product-owner — remove specific enumeration; replace with "see Grep 4 at HEAD; all variants share `^### Dim-7` prefix per PC9 spec."

### F-S15.17-SP9-007 MEDIUM — Story §Bidirectional Parity Audit verdict reasoning sloppy (inv-2/inv-11 cited in body Invariant Coverage table; verdict says "code-review-gate only")
**File:** Story L245 verdict
**Evidence:** Verdict misleading; body actually IS bidirectionally consistent but reasoning sloppy.
**Routing:** story-writer — rephrase verdict.

### F-S15.17-SP9-008 HIGH [paper-fix; arithmetic mismatch from F-SP8-001 rewrite] — §Cure-Extension Parsimony Note point 2 enumerates "ALL 5 STATE.md sites (PC1/PC2/PC4/PC5)" — 4 PCs listed for "5 sites"
**File:** BC L667
**Evidence:** Pass-8 F-SP8-001 rewrite of point 2 introduced arithmetic mismatch. 5 STATE.md sites are PC1/PC2/PC3/PC4/PC5; enumeration lists 4. PC3 missing (PC3 uses two-step check per inv-4 line 451 with single-row extractor exception).
**Routing:** product-owner — fix to `(PC1/PC2/PC3/PC4/PC5)` matching "ALL 5" count.

### F-S15.17-SP9-009 LOW — EC table EC-004 description contains provenance noise about `OutputTooLarge` vs `TooBig`; belongs in §Cure-Extension Parsimony Note
**File:** BC L519
**Routing:** product-owner — strip provenance to §Cure-Extension Parsimony Note; keep EC-004 behavioral.

### F-S15.17-SP9-010 NITPICK — §Adversary Pass Coverage Pass-8 entry naming convention drift ("pass-8 CRITICAL" should be "pass-8 HIGH; 1 CRITICAL" per pass-3 pattern)
**File:** BC L118
**Routing:** product-owner — rephrase per pass-3 sibling convention.

## Part B — Convergence Assessment

### Verdict + STREAK + Trajectory

**Verdict:** HIGH 9 (0C+4H+3M+1L+1N)
**STREAK:** 0/3 → 0/3 (reset)
**Trajectory:** 14→11→14→16→12→11→9→11→**9** (asymptotic at [9, 11])

### DIAGNOSTIC ANSWER — Did META-36 cure structurally bottom out recursion?

**NO.** META-LEVEL-37 CANDIDATE emerged in F-SP9-001 — scalar-snapshot of cardinality presented as structural-form invariant. POLICY 5 v1.3.6 Part B self-violation. Cure-of-cure-OF-cure recursion advanced to LEVEL 7.

### TD-VSDD-059 paper-fix residuals?

**YES, partial.** Pass-8 closed 3 paper-fixes visibly but pass-9 detects 2 residuals:
- F-SP9-002: PC10 OUT-OF-SCOPE is paper-fix renaming when `^## LESSON-` is valid anchor
- F-SP9-008: Cure-Extension point 2 rewrite introduced arithmetic mismatch (5 sites ≠ 4 PCs)

### Path to 3-CLEAN: plausible OR SEAL?

**SEAL RECOMMENDATION: YES.** Evidence:
1. META-37 candidate emergence at pass-9 confirms recursion continues monotonically. POLICY 5 v1.3.6 Part B self-violates in its own self-application example.
2. Persistent residual paper-fixes — TD-VSDD-059 detection axis discovers new paper-fixes faster than closures land.
3. 9-pass asymptotic floor [9, 11] HIGH confirmed.
4. META-LEVEL ply ascending monotonically: 30→31→32→33→34→35→36→37 (8 META-LEVELs in 9 passes).
5. POLICY 5 cure evolution: v1.3→v1.3.1→v1.3.3→v1.3.4→v1.3.5→v1.3.6 (6 cure layers in 5 passes).
6. Each cure addresses prior cure's self-application gap; recursion has NOT structurally bounded.

L-EDP1-007/051/061 precedent (3-CLEAN structurally impossible under prose-only codification) applies. F5 D-386 Option C + S-15.14 D-477 SEAL precedent for asymptotic-acceptance form of convergence.

**Convergence is not plausible within 2-3 passes.** Continuing would produce META-38/39/40 at 1-pass intervals.

### Top 3 Findings

1. F-SP9-001 HIGH META-37 candidate — Grep 10 scalar `16` non-reproducible at HEAD (`17`); POLICY 5 v1.3.6 Part B self-violation
2. F-SP9-002 HIGH paper-fix residual — PC10 OUT-OF-SCOPE rationale structurally false; `^## LESSON-` valid anchor
3. F-SP9-003 HIGH sibling-sweep incomplete — embedded T-5 NOTE signatures still `Option<&str>` despite §Architecture Anchors "All 7 Option<String>" claim

### New META-LEVEL classes

**META-LEVEL-37 CANDIDATE:** scalar-snapshot-of-cardinality-presented-as-structural-form-invariant-but-disowned-in-prose. Distinct from META-36 (SHA-pinning) — captures NUMERICAL VALUE and calls value structural when only the PREDICATE is structural.

### Iron Law Attestation

Did NOT read prior-pass reports. Fresh-context. POLICY 5 v1.3.6 gates re-executed at HEAD with literal grep.
