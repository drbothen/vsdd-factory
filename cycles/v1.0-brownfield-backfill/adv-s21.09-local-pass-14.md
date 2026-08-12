---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: vsdd-factory:adversary
timestamp: 2026-08-12T22:00:00Z
phase: 14
inputs: [.factory/stories/S-21.09-wasm-artifact-restore-and-registry-parity.md]
input-hash: "0d4859c"
traces_to: S-21.09-wasm-artifact-restore-and-registry-parity.md
pass: 14
previous_review: adv-s21.09-local-pass-13.md
---

# Adversarial Review: S-21.09 WASM Artifact Restore and Registry Parity (Pass 14)

**Verdict: NOT CLEAN — 1 MEDIUM + 1 LOW. Streak → 0/3.**
**Reviewed commit: `46e334da` (feature/S-21.09)**
**LOCAL streak: 0/3 — fourteen passes, zero CLEAN**
**D-chain: D-976**

Reviewed: story v1.24 against implementation at `.worktrees/S-21.09` @ `46e334da`, governing BC-4.16.001 v1.8, full POLICY 1–22 rubric. Fresh-context per Iron Law — read only `adv-s21.09-local-pass-13.md` Part A. **NOT a CLEAN pass.** Streak remains 0/3.

---

## Finding ID Convention

Finding IDs use the format: `ADV-<CYCLE>-P<PASS>-<SEV>-<SEQ>`

- `ADV`: Fixed prefix
- `<CYCLE>`: `BB` (v1.0-brownfield-backfill per `.factory/current-cycle`)
- `<PASS>`: Two-digit pass number — `P14` for this pass
- `<SEQ>`: Three-digit sequence

This pass's dispatch used the local finding labels headed `MEDIUM` and `LOW` (as sourced from the adversary's dispatch report, with no `F-N` numbering assigned by the adversary itself); the canonical cross-reference to the `ADV-BB-P14-<SEV>-NNN` form is: the `MEDIUM` finding = `ADV-BB-P14-MED-001`; the `LOW` finding = `ADV-BB-P14-LOW-001`. Both forms are retained verbatim below — the dispatched finding headers are NOT renamed, per D-448(a) source-attestation parity with the dispatched review text.

---

## Part A — Fix Verification (pass-13 carry-over items)

| ID | Previous Severity | Status | Notes |
|----|-------------------|--------|-------|
| ADV-BB-P13-HIGH-001 (pass-13 F-1) | HIGH | RESOLVED | Cross-file stale test-range cite (story AC-006 "Tests:" bullet + test-file module docstring, both omitting T-051) closed via test-writer commit `46e334da` (docstring-only) + story-writer story v1.24. This pass's "Axes verified CLEAN" section independently re-confirms: "Count parity holds. 6 tests T-006..T-011 + 40 tests T-012..T-051 = 46. Matches AC-006 'Tests:' bullet, Architecture Mapping, Token Budget, File Structure, module docstring test-plan." — a fresh-context re-derivation of the exact two sites pass-13's F-1 flagged, both now consistent. No residual finding on this axis. |
| ADV-BB-P10-MED-001 | MEDIUM | UNRESOLVED — not addressed this pass | `hook-plugins/sub/` directory-only staging control remains open; not re-verified in this dispatch. Carries to pass-15. |
| ADV-BB-P10-LOW-001 | LOW | UNRESOLVED — not addressed this pass | NUL byte / trailing-space name admission remains open; not re-verified in this dispatch. Carries to pass-15. |
| ADV-BB-P10-LOW-002 | LOW | UNRESOLVED — not addressed this pass | Fail-open arms guarded only by unasserted call ordering remain open; not re-verified in this dispatch. Carries to pass-15. |
| ADV-BB-P10-LOW-003 | LOW | UNRESOLVED — not addressed this pass | `workspace_root()` untested directly remains open; not re-verified in this dispatch. Carries to pass-15. |

---

## Part B — New Findings

> The section below is the adversary's dispatched review text, persisted verbatim per D-448(a) source-attestation parity (standard template scaffolding only; finding body not paraphrased).

# Adversarial Review — S-21.09 LOCAL cascade (fresh context)

Scope reviewed: story spec S-21.09-wasm-artifact-restore-and-registry-parity.md v1.24 (full); crates/factory-dispatcher/tests/bundle_orphan_check.rs @ worktree 46e334da (all production/shared helpers + representative tests); BC-4.16.001 v1.8; .factory/policies.yaml (policies 1–13 in full, 14+ scanned). WASM artifact presence confirmed (binary read succeeded → file present on disk; registry entry present at hooks-registry.toml:944; gitignored-dir Glob emptiness is a tooling artifact, not absence — not reported).

## Axes verified CLEAN (no finding)
- Count parity holds. 6 tests T-006..T-011 + 40 tests T-012..T-051 = 46. Matches AC-006 "Tests:" bullet, Architecture Mapping, Token Budget, File Structure, module docstring test-plan.
- Mutation-isolation arithmetic is sound. Independently re-derived the lex_norm resolution for T-050 (../.. → joined_parts == root_parts, length conjunct sole determinant, M2 flip correct) and T-051 (../../../sib/ghost.wasm → root_parts[0..N-1]+["sib","ghost.wasm"], prefix conjunct sole determinant, .all→.any/.all→true flips correct). Both negative-identifier assertions hold under live code. T-048's 18-case partition classifications all check out (gate-1/2/3 and containment predicate).
- BC traceability correct. Story body BC table H1 matches BC file H1 verbatim (BC-4.16.001.md:44); PC1/PC2/PC3/PC4/Precondition 3/Invariant 1 anchors resolve. subsystem: SS-04 / capability: CAP-034 consistent with BC frontmatter.
- check_declared_subset_tracked determinants (hooks floor 29/30, resolvers is_empty, EC-005b, step-3 MISSING, step-4 STAGED-NOT-COMMITTED) each have a dedicated boundary/isolation control that goes RED on deletion (T-016/T-027/T-019/T-022/T-020/T-015/T-021).

## MEDIUM — Two production-validation determinants in run_t012_gate survive whole-suite deletion (un-isolated gate; mutation-completeness gap)
Confidence: HIGH (evidence) / severity MEDIUM (blast radius = test robustness, not live misclassification)
Location: run_t012_gate — Registry::parse_str(&hooks_content) block (bundle_orphan_check.rs ~882–891) and the resolvers assert_eq!(resolvers_schema_version, 1, …) block (~896–911).
Failure scenario: Delete either block. No test in the 46-test suite goes RED. Every fixture that reaches run_t012_gate (T-030 phase B, T-034/36/37/39/41/49, T-050, T-051, and T-012 on the real registry) writes hooks with schema_version = 2 and resolvers with schema_version = 1, so Registry::parse_str always succeeds and the resolvers assert always passes. No fixture constructs a registry that is TOML-parseable + passes inventory + passes detect_ungated_declarations + clears the hooks floor but fails production schema validation (e.g. schema_version = 3, an unknown field, an invalid tool regex, or a resolvers schema_version = 2). Both gates are therefore sole-determinant mutants that survive — exactly the "claimed gate with un-isolated determinant" class the project closed for the containment predicate via T-050 (length conjunct) and T-051 (prefix conjunct).
Why the self-disclosure does not discharge it: the story states (AC-006 §2d) "No single test ID owns this gate in isolation; every T-012..T-051 test … implicitly exercises it." Per CLAUDE.md Standing Rule 3 §1, implementer self-disclosure is not authoritative: implicit exercise on a valid registry does not kill a deletion mutant. The resolvers schema_version == 1 assert has no acknowledgement at all. By the project's own T-050/T-051 precedent (fixtures are constructed to prove a gate fires even for conditions that cannot occur on the real registry, e.g. ../..), both gates warrant a firing fixture.
Minimal fix route: add two controls — (a) a fixture hooks-registry that passes inventory + floors + detect_ungated but sets schema_version = 3 (or injects an unknown field / bad tool regex), asserting the #[should_panic(expected = "fails production validation")] path; (b) a fixture with resolvers schema_version = 2, asserting the resolvers schema assert fires. (Alternatively, if the authors deem these defensive-only and out of mutation scope, that must be an explicit accepted-residual for both gates, adjudicated by the human — not left as "implicitly exercised.")

## LOW — Story RG-plan / EC-005b rows cite the wrong #[should_panic] literal for T-020 (spec↔impl drift, self-contradictory)
Confidence: HIGH
Location: story spec Red Gate Test Plan row T-020 (line 694) and edge-case EC-005b (line 736) both state T-020 uses #[should_panic(expected = "T-012 EC-005")]. Actual code: #[should_panic(expected = "T-012 EC-005b")] (bundle_orphan_check.rs:2177); the guard panics "T-012 EC-005b:" (:773); the in-file module docstring (:30) correctly says "T-012 EC-005b".
Why it's real, not cosmetic: the story simultaneously asserts T-020 "specifically pins EC-005b (not EC-005a)." The literal it documents ("T-012 EC-005") is a substring of both "T-012 EC-005a" and "T-012 EC-005b" — so if the code were "aligned" to the SoT literal, T-020 would match the EC-005a panic too and lose the a/b distinction the story guarantees. The code is correct and more precise; the SoT literal is stale and internally inconsistent. The story is the outlier (module docstring + code agree on EC-005b).
Minimal fix route: correct both story citations (lines 694, 736) to #[should_panic(expected = "T-012 EC-005b")].

## Novelty Assessment
Both findings are substantive, not rewording. The MEDIUM is a genuine un-isolated-determinant gap of the same lineage the project has been closing (T-048→T-050→T-051), but at two determinants those passes did not reach; it is not a nitpick. The LOW is a concrete, dual-location SoT literal that contradicts the story's own stated pinning guarantee.

## Verdict
NOT CLEAN — streak remains 0/3. One MEDIUM + one LOW finding. No BLOCKER/HIGH; the deliverable is otherwise exceptionally well-hardened (correctness gates, containment-predicate conjuncts, path-normalization edge cases, and count/traceability parity all verified sound). No process-gap tags warranted.

---

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 0 |
| MEDIUM | 1 |
| LOW | 1 |

**Overall Assessment:** pass-with-findings
**Convergence:** 2 findings (1 MEDIUM new + 1 LOW new); pass-13's F-1 verified CLOSED (see Part A); 4 pass-10 carry-over items (MED-001, LOW-001/002/003) remain open, not re-addressed this pass — see Part A.
**Readiness:** requires revision

---

## Novelty Assessment (tabular)

| Field | Value |
|-------|-------|
| **Pass** | 14 |
| **Story version reviewed** | v1.24 |
| **Reviewed commit** | 46e334da |
| **New findings** | 2 (1 MEDIUM + 1 LOW) |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 2 / (2 + 0) = 1.00 |
| **Median severity** | MEDIUM |
| **Severity trajectory (HIGH)** | 3→2→3→2→1→1→3→2→1→3→1→1→1→0 |
| **Total finding trajectory** | 3→3→2→13→11→9→9→8→8→15→2→1→1→2 (pass-1: 3; pass-2: 3; pass-3: 2; pass-4: 13; pass-5: 11; pass-6: 9; pass-7: 9; pass-8: 8; pass-9: 8; pass-10: 15; pass-11: 2; pass-12: 1; pass-13: 1; pass-14: 2) |
| **Verdict** | FINDINGS_REMAIN |

The two findings are the same lineage as the T-048→T-050→T-051 un-isolated-determinant class, reaching two determinants (`Registry::parse_str` hooks production validation; resolvers `schema_version == 1` assert) that prior passes did not isolate, plus one concrete spec↔impl literal drift (T-020/EC-005b). The gate/mutation machinery is otherwise exceptionally well-hardened — no BLOCKER/HIGH, no surviving-mutant on the containment predicate, no fail-open, no path-normalization edge gap.

## Verdict (restated per template)

NOT CLEAN — 1 MEDIUM + 1 LOW. Streak → 0/3.
