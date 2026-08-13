---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: vsdd-factory:adversary
timestamp: 2026-08-13T09:30:00Z
phase: 22
inputs: [.factory/stories/S-21.09-wasm-artifact-restore-and-registry-parity.md]
input-hash: "d8e69cf"
traces_to: S-21.09-wasm-artifact-restore-and-registry-parity.md
pass: 22
previous_review: adv-s21.09-local-pass-21.md
---

# Adversarial Review — S-21.09 (LOCAL cascade pass)

Story: S-21.09-wasm-artifact-restore-and-registry-parity.md (v1.31). Worktree/HEAD: .worktrees/S-21.09 @ 1c93f499. Governing BC: BC-4.16.001.md (v1.8). Gate under test: bundle_orphan_check.rs + registry.rs.

## Verdict: CLEAN
No finding rises to BLOCKER / HIGH / MEDIUM / LOW / NIT. The 3-CLEAN streak may advance on this pass.

## Verification performed (evidence trail)
Assertion-quality (priority lens) — every determinant traced to source, no vacuity/tautology found:
- run_t012_gate / check_declared_subset_tracked / check_registry_inventory / extract_hook_plugin_name / detect_ungated_declarations / lex_norm / parse_plugin_refs read at their real definitions (bundle_orphan_check.rs lines 217-1003). Narrative in the story Red-Gate/EC rows matches the actual code in every case traced.
- Fixture-driven controls (T-013/T-014/T-015/T-016) call the REAL gate functions (parse_plugin_refs, check_declared_subset_tracked), not replicas; sole load-bearing assertions verified non-vacuous. T-013/T-014 additionally filter comment lines (fixture_body_without_comments) so the body-syntax assertion cannot be satisfied by a header comment — a genuine control-integrity guard.
- Mutation-hardening controls verified load-bearing against the exact production arms they target: T-054 #[should_panic(expected = "schema_version=-1 but production requires 1")] precisely pins the .unwrap_or(-1) sentinel (run_t012_gate line 916); T-055 pairs the fail-open Err(_)=>Vec::new() isolation with a valid-TOML positive control that kills the whole-function-Vec::new() coarse mutant; T-056 pins lex_norm's CurDir arm via direct call.
- Determinant isolation is complete: both conjuncts of the in_repo predicate (T-050 length / T-051 prefix), all three extract_hook_plugin_name gates (T-033 / T-026b / T-035), and both schema determinants (T-052 hooks-parse_str / T-053 resolvers-assert_eq!) each have a dedicated isolating control. #[should_panic] expected-strings (T-019/T-027/T-041/T-052/T-053/T-054) all match production message literals.

Format-lock completeness claim independently confirmed (POLICY 15 / format-fidelity): grep of .contains("(MISSING|STAGED-NOT-COMMITTED|UNGATED-DECLARATION|OUTSIDE-REPO-DECLARATION|UNEXPECTED): returns only 4 hits — all !err.contains("…: ") NEGATIVE assertions (lines 4285, 4627, 4959, 5111). Positive needles prefixed with two spaces = 21. The spec's "zero remaining unindented positive needles" claim holds exactly. No unindented-needle-against-indented-emitter vulnerability remains.

Count-parity confirmed exact: 51 #[test] functions total (50 matching test_S_*_acNNN_TNNN + T-011 test_S_19_06_policy20_T011) = T-006..T-056; 45 S-21.09-owned = T-012..T-056; +1 registry::tests::on_error_falls_back_to_registry_defaults_when_entry_omits_it (registry.rs:642). Matches the story's 51/45+1 claim across all sites.

Other checks: workspace_root() walks up from CARGO_MANIFEST_DIR to the worktree plugins/vsdd-factory — T-012 runs git ls-files/ls-tree against the correct tree (not the main checkout). AC-001 WASM present on disk at the worktree path. BC-4.16.001 H1 ↔ story body BC-table title match verbatim, v1.8 cite current (POLICY 7). Single-BC frontmatter↔body↔AC-trace coherence holds (POLICY 8). SHA 1c93f499 cited throughout = reviewed HEAD.

## Owed-items / accepted-residuals evaluated — confirmed NOT defects
- verification_properties: [] — BC-4.16.001 §VP all "(TBD — assigned by state-manager)"; story invents no VP IDs and records a routing proposal. Documented owed-item, not drift.
- input-hash: "cf3a0c6" POLICY-18 three-way equality — frontmatter comment explicitly marks it owed at first state-manager burst; the three-way STORY-INDEX check is a state-manager closure obligation, out of scope for a per-story implementation review.
- EC-001 corrupt/zero-byte WASM — name-only parity gate intentionally does not catch content corruption; routed to S-21.14 (real story anchor), consistent with all 35 sibling WASMs. Legitimate feature-ordering boundary.
- SURV-01 (lex_norm RootDir | Prefix(_) => parts.clear()) — accepted-residual with a proof of un-isolatability (root component always fires first, parts provably empty). Doc-comment record is the correct disposition, not a paper-fix.
- enabled = false false-positive class (LOW-3) — documented in check_declared_subset_tracked doc; latent (zero instances today), fail-loud. Correctly recorded per POLICY 13.

## Novelty Assessment
Novelty LOW — fresh-context re-derivation surfaced no gap. This deliverable has been hardened across ~14 prior passes. Every determinant is isolated, the format-lock sweep is provably complete, count-parity is exact, and all residuals are documented-and-routed. CLEAN is the correct verdict. No process-gap tags warranted.
