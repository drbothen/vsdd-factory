---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: vsdd-factory:adversary
timestamp: 2026-08-13T02:00:00Z
phase: 17
inputs: [.factory/stories/S-21.09-wasm-artifact-restore-and-registry-parity.md]
input-hash: "48547f6"
traces_to: S-21.09-wasm-artifact-restore-and-registry-parity.md
pass: 17
previous_review: adv-s21.09-local-pass-16.md
---

# Adversarial Review — S-21.09 LOCAL cascade

Reviewed: story spec v1.28, BC-4.16.001 v1.8, implementation at .worktrees/S-21.09 @ 12d0fe98. Derived independently from the artifacts.

## Verdict: CLEAN
No finding at any severity (BLOCKER / HIGH / MEDIUM / LOW / NIT) rises to a genuine defect. The 3-CLEAN streak may advance.

## What I independently verified (not trusting the narrative)
Gate logic — every determinant is isolated. The functions-under-test live in crates/factory-dispatcher/tests/bundle_orphan_check.rs (lex_norm, extract_hook_plugin_name, detect_ungated_declarations, parse_plugin_refs, check_registry_inventory, check_declared_subset_tracked, run_t012_gate, git_tracked_wasm_names, git_committed_wasm_names); only Registry::parse_str is production (src/registry.rs). Re-derived the three-gate structure of extract_hook_plugin_name (min-length expected_depth+2, registry-parent prefix loop, eq_ignore_ascii_case("hook-plugins")) and confirmed each gate has a dedicated isolation control with the other two held intact (gate-1↔T-033, gate-2↔T-026(b), gate-3↔T-035). The two-conjunct in_repo containment predicate in detect_ungated_declarations has both conjuncts independently isolated (length↔T-050 ../.. exact self-match; prefix↔T-051 ../../../sib/ghost.wasm), and the orthogonality claim holds (M2 vs .all→.any/.all→true are non-overlapping kills). The three-class totality invariant (gated / UNGATED / OUTSIDE-REPO) is exhaustive and mutually exclusive — a Some from extract always implies in_repo true, so no double-classification is possible.

Fail-open arms are pinned, not silently admitted. detect_ungated_declarations's Err(_) => Vec::new() malformed-TOML arm and lex_norm's CurDir/RootDir|Prefix arms are unreachable through run_t012_gate (upstream Registry::parse_str + resolvers toml::Value parse panic first). They are pinned by direct-call unit tests (T-055, T-056) or recorded as SURV-01 — a provably un-isolatable accepted-residual (a Normal component can never precede a root component per std::path invariant, so the arm's body is unobservable under every possible input, not merely every input tried). The resolvers_schema_version.unwrap_or(-1) fail-closed sentinel is isolated by T-054 (absent-key fixture, distinct from T-053's wrong-integer fixture) — a genuine correctness control, correctly distinguished from the assert_eq! block T-053 isolates.

Count parity holds across all sites. 51 #[test] functions T-006..T-056 (the raw #[test] grep of 53 includes two doc-comment mentions of the literal #[test]); 45 S-21.09-owned (T-012..T-056); plus one out-of-gate registry unit test (on_error_falls_back_to_registry_defaults_when_entry_omits_it) isolating the SURV-05 Default::default() body mutant. Matches story lines 689–691 and the AC-006/AC-007 traceability exactly.

AC-001 artifact delivery confirmed. plugins/vsdd-factory/hook-plugins/ contains 36 WASMs (35 hooks-unique + vsdd-context-resolvers.wasm), including validate-factory-path-staging.wasm, and hooks-registry.toml declares it (plugin = "hook-plugins/validate-factory-path-staging.wasm"). This matches declared=tracked=36 → T-012 GREEN. (Initial Glob "No files found" was a rooting artifact — .worktrees/ is gitignored in the main repo; per worktree-identity discipline it was not reported as an absent-file finding.) plugins/vsdd-factory/ contains exactly the two expected top-level .toml files, so check_registry_inventory (T-012 step 1) passes at HEAD.

Traceability/anchoring integrity. BC-4.16.001 v1.8 H1 matches the story body Behavioral Contracts table title verbatim; ACs 001–007 trace to PC1/PC2/PC3/PC4/Precondition 3/Invariant 1 consistently with the BC clauses. subsystems: [SS-04] matches BC subsystem: SS-04. behavioral_contracts: [BC-4.16.001] propagates to body table + AC traces (POLICY 7/8 satisfied). SHA 12d0fe98 cited in the story matches the review HEAD.

Accepted residuals are honestly characterized, not concealed. LOW-3 (enabled = false entries still contribute to declared — latent today, fail-loud), SURV-01 (un-isolatable dead arm), the verification_properties: [] VP gap (all 4 BC-4.16.001 VP rows are authored as TBD upstream — inherited, disclosed, and routed, not introduced by S-21.09), and BC-4.16.001's stale document-level status: draft vs active lifecycle_status (a BC-owned field, correctly flagged by the story) are all disclosed with routing.

## Item considered and dismissed (transparency, not a finding)
The empirical mutation-kill records for T-052/T-053 read "deleting the block sends T-052 RED while all other 47 tests stay GREEN" (test-file doc comments and story Red Gate rows). At HEAD there are 50 peer tests, because T-054/T-055/T-056 were added in the same commit 12d0fe98. Evaluated whether this is a stale-count / POLICY-5 drift. It is not a genuine defect: the §Mutation-Completeness Audit explicitly frames its baseline as "the pre-existing T-006..T-053 suite" (48 tests), making "47 others" the internally-consistent pass-14 empirical figure; TD-VSDD-091 excepts pass-report/empirical-mutation changelogs from volatile-count strictness; and the load-bearing property (each test isolates its own determinant — only it goes RED) is verifiably correct. Re-running the mutant at HEAD leaves 50 GREEN, which does not contradict the isolation claim. This is a defensible historical-empirical annotation, not a contradiction of implemented behavior.

## Novelty Assessment
Novelty: LOW. This deliverable has converged. It has been through 14+ adversarial passes plus a dedicated mutation-completeness burst; fresh-context re-derivation of the gate structure, totality/partition invariants, isolation controls, count parity, artifact delivery, and BC traceability surfaced no new gap. CLEAN is the warranted verdict for a well-hardened deliverable.
