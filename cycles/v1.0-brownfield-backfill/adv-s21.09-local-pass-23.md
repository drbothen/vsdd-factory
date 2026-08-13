---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: vsdd-factory:adversary
timestamp: 2026-08-13T11:00:00Z
phase: 23
inputs: [.factory/stories/S-21.09-wasm-artifact-restore-and-registry-parity.md]
input-hash: "d8e69cf"
traces_to: S-21.09-wasm-artifact-restore-and-registry-parity.md
pass: 23
previous_review: adv-s21.09-local-pass-22.md
---

# Adversarial Review — S-21.09 (LOCAL cascade pass), fresh-context

Artifacts: story S-21.09-...md v1.31; worktree 1c93f499; bundle_orphan_check.rs (51 tests) + registry.rs unit test; governing BC-4.16.001 v1.8; policies.yaml v1.4.23.

## Verdict: CLEAN
No finding rises to BLOCKER, HIGH, MEDIUM, LOW, or NIT. Actively hunted for defects across every axis in the rubric and found none that survive scrutiny.

### What I independently verified (not inherited from the spec's own claims)
Count / SHA / version parity (all consistent): 51 #[test] functions T-006..T-056; 45 S-21.09-owned = T-012..T-056; +1 registry unit test (on_error_falls_back_to_registry_defaults_when_entry_omits_it). Consistent across story frontmatter, module docstring, File-Structure row, and all three recent changelog rows (v1.29/1.30/1.31). Current-state SHA 1c93f499 matches the dispatched HEAD and is cited consistently.

BC anchoring (POLICY 4/6/7/8 — all sound): BC-4.16.001 file version 1.8 matches every story cite. BC H1 reproduced verbatim in the story body BC table. behavioral_contracts: [BC-4.16.001] ↔ body BC table ↔ AC traces (Precondition 3, PC1-4, Invariant 1) bidirectionally coherent. ACs trace to real BC clauses; the parity-gate → Precondition 3 anchor is semantically correct, not a mis-anchor. subsystems: [SS-04] matches BC subsystem SS-04.

AC-001 corroboration (avoided a false "absent WASM" finding): Glob returned "No files found" for hook-plugins/*.wasm — that directory is .gitignore'd and Glob respects gitignore, a known false-negative vector, not evidence of absence. Direct Read of validate-factory-path-staging.wasm returned a binary-file error (file exists), corroborating AC-001. The git-tracked proof is T-012 running against workspace_root().

Gate-function correctness (read the real definitions): lex_norm, extract_hook_plugin_name (3 reachable gates + 4 audited defensive-unreachable exits), detect_ungated_declarations (single-copy delegation to extract_hook_plugin_name; both conjuncts of the in_repo predicate independently isolated by T-050/T-051), parse_plugin_refs (toml-crate), check_registry_inventory (fail-closed *.toml, not *-registry.toml), check_declared_subset_tracked (floors + subset + staged-not-committed), run_t012_gate, and the git helpers. No spec↔impl drift: the story's 4-step extract_hook_plugin_name narrative, the EC table (EC-004 via inventory ?, EC-005a/b guards), and the production-validation gate all match code.

Assertion quality (STRENGTHENED rubric — no vacuity/tautology found): POLICY 11 — every fixture-driven test calls the REAL gate functions, not logic copies; the v1.29 PR-review burst removed the three tautological re-derivations (T-007/T-048/T-026). #[should_panic] precision: T-054 pins the exact -1 sentinel ("schema_version=-1 but production requires 1"), T-019/T-022/T-027 pin floor-identifier strings, T-041/T-049 pin distinct git-failure vs EC-005a identifiers — none near-miss-loose. Positive controls present where a dead/inert mutant could otherwise pass: T-055 pairs the malformed-TOML (is_empty) arm with a valid-TOML non-empty control (closes whole-function-Vec::new()); T-056 pins lex_norm CurDir contract directly. Format-lock fidelity independently confirmed: grep for positive .contains("<IDENT>: needles WITHOUT a leading two-space indent returns only the 4 legitimate !err.contains(...) negative-identifier assertions (T-043/T-047/T-050/T-051); grep for the two-space-indented positive form returns 21 hits. Zero unindented positive needles survive against any indented production emitter — the pass-21 completeness claim holds under my own execution.

Mutation-audit disposition: SURV-01 (lex_norm RootDir|Prefix parts.clear()) is a genuine provably-un-isolatable dead arm (a root/prefix component is always first from Path::components(), so parts is empty when it fires) — correctly recorded as accepted-residual via doc comment, not a vacuous test. SURV-02/03/04/05 each have a dedicated firing control.

### Near-misses I evaluated and ruled out (not findings)
1. >= 30 floor constant duplicated at 3 sites; resolver-arm vacuous-pass and T-017 first-match extractor in release.yml/pr-manager-hardening.bats. Documented in §Routing Proposals, human-directed to keep S-21.09 narrow, anchored to a concrete future story (S-21.14). Meets all three CLAUDE.md Rule-3 deferral conditions → owed-item, not a defect.
2. EC-001 content-validation gap (name-only gate admits a zero-byte/corrupt WASM). Explicitly documented, out of the human-scoped story boundary, routed to S-21.14. Not a defect.
3. verification_properties: []. BC-4.16.001's four VP rows all read "(TBD — assigned after VP authoring pass)"; the story invents no VP IDs and routes allocation to state-manager. Documented + routed owed-item, not drift.
4. BC-4.16.001 frontmatter status: draft vs lifecycle_status: active. Pre-existing BC-metadata state owned by product-owner (outside the story's own artifacts), explicitly reconciled in §BC Status per POL-14. Not an S-21.09 defect; at most a product-owner-domain observation, non-blocking here.

### Novelty Assessment
Novelty LOW — no gaps found. This deliverable has been through 21 LOCAL passes + a PR-review vacuity pass + an exhaustive mutation audit; hardening is genuine (confirmed format-lock and floor-boundary controls by independent grep and code-trace, not by trusting the narrative). The v1.29 reopening to 0/3 was driven by a vacuity lens that has since been fully applied; re-applied fresh and it is closed. Legitimately converged, production-grade artifact. CLEAN — this pass advances the 3-CLEAN streak.
