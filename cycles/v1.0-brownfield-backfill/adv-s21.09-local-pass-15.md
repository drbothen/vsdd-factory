---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: vsdd-factory:adversary
timestamp: 2026-08-12T23:45:00Z
phase: 15
inputs: [.factory/stories/S-21.09-wasm-artifact-restore-and-registry-parity.md]
input-hash: "13be9a1"
traces_to: S-21.09-wasm-artifact-restore-and-registry-parity.md
pass: 15
previous_review: adv-s21.09-local-pass-14.md
---

# Adversarial Review — S-21.09 LOCAL cascade (fresh-context pass)

Reviewed: story v1.26 against impl b761477f (bundle_orphan_check.rs 51 tests T-006..T-056 + registry.rs on_error unit test), BC-4.16.001 v1.8, full POLICY 1–22 rubric.

## BLOCKER Findings
None.

## HIGH Findings
None.

## MEDIUM Findings

### F-1 [MEDIUM][spec↔impl doc drift; sibling-sweep gap] — Two stale comments describe the HIGH-1 pre-fix (basename-collapse) behavior as the current T-032 contract, contradicting the code and three authoritative sibling sites
Confidence: HIGH (exact line-level contradiction against executable assertions).
Policy: POLICY 4 (semantic_anchoring_integrity — anchor/claim must be semantically correct); TD-VSDD-060 / Partial-Fix Regression Discipline (a return-contract change not swept across all comment sites).
Location (function/anchor, not line-pinned per TD-VSDD-091): crates/factory-dispatcher/tests/bundle_orphan_check.rs
  - Site A — the test_S_21_09_ac006_T023_boundary_polarity_bare_and_traversal_cancels_excluded preamble comment, **Included region:** bullet list: "Nested subdir: hook-plugins/sub/nested.wasm → nested.wasm (T-032)".
  - Site B — the test_S_21_09_ac006_T034_git_ls_tree_r_finds_nested_committed_wasm preamble comment: "T-032 proves the DECLARED side: hook-plugins/sub/nested.wasm in a registry yields nested.wasm as the declared artifact name."
Why it is wrong (input → wrong claim): For declaration plugin = "hook-plugins/sub/nested.wasm", extract_hook_plugin_name returns joined_parts[expected_depth..].join("/") = "hook-plugins/sub/nested.wasm" (full registry-relative path), NOT the bare "nested.wasm". Both comments assert the declared identifier is nested.wasm. That bare-basename form is precisely the HIGH-1 (pass-9) correctness bug — where a nested declaration collapsed to the same basename as a flat-committed hook-plugins/nested.wasm, producing a false negative — which was explicitly closed. The two comments re-describe the closed bug as the live contract.
Contradicts three authoritative sites in the same file (all state the full-path contract): module docstring Test-Plan row for T-032; extract_hook_plugin_name doc/body + resolution table row; the T-032 test body assertions (assert!(refs.contains("hook-plugins/sub/nested.wasm")) AND assert!(!refs.contains("nested.wasm"))).
Impact: no test-logic impact (comments only), so no false GREEN. But a concrete, mutually-reinforcing false statement of the gate's return contract at two sites, in the boundary-polarity record prose whose purpose is to document exactly what each declaration form resolves to. A maintainer consulting either record would be misled into believing nested declarations collapse to basename — the exact false-negative HIGH-1 fixed — risking a regression during future gate edits. Stale content contradicting current behavior, not a formatting nit.
Root cause: the HIGH-1/pass-9 return-contract change (basename → full path) swept the function body, function docs, T-032 row, and T-032 assertions, but not these two explanatory preamble comment blocks (T-023, T-034). Blast radius = 2 sites.
Minimal fix route: update both comments to the full-path form. Routing: test-writer (test-file comment correction, no logic change).

## LOW Findings
None.

## NIT Findings
None. (The grammatically-imperfect "has only 1 entries" floor message is verbatim-pinned by T-019's #[should_panic(expected=…)] and is deliberately not flagged.)

## Verdict
NOT CLEAN — 1 MEDIUM finding (F-1). Under BC-5.39.001, this resets the LOCAL 3-CLEAN streak to 0/3. The finding is a genuine documentation-vs-code contradiction (two sites) describing a closed correctness bug as current behavior; not manufactured.

## Coverage / What came up clean (verified independently)
- AC-001 git-tracked artifact: validate-factory-path-staging.wasm exists in the b761477f worktree checkout; hook-plugins/ is .gitignore-listed — a gitignored file present in a fresh worktree checkout is present because committed. T-012 is the executable gate.
- T-012 is a real end-to-end gate, not a tautology: run_t012_gate(&workspace_root()) reads real registries, validates the real hooks-registry.toml through production Registry::parse_str, diffs real git-tracked/committed sets. POLICY 11: fixture controls call shared check_declared_subset_tracked / extract_hook_plugin_name / run_t012_gate, not logic replicas.
- Non-vacuity floors present and pinned: hooks floor ≥30 (T-016 pass@30 / T-027 fire@29; T-019 @1), resolvers floor ≥1 (T-022), EC-005a (T-049), EC-005b (T-020).
- Mutation isolation claims verified sound: min-length +2 (T-033), prefix loop (T-026b), gate-3 eq_ignore_ascii_case (T-035/T-031), verbatim/no-lowercasing (T-031/T-042), full-path return (T-032), ls-tree -r (T-034), tracked-vs-committed identity (T-037), in_repo length conjunct (T-050) and prefix conjunct (T-051) each independently isolated and orthogonal, hooks parse_str block (T-052), resolvers schema_version==1 assert (T-053), .unwrap_or(-1) fail-closed sentinel (T-054), detect fail-open Err(_) arm (T-055), lex_norm CurDir arm (T-056), out-of-gate RegistryEntry::on_error defaults-fallback (registry.rs unit test).
- SURV-01 accepted-residual honestly un-isolatable: lex_norm RootDir|Prefix => parts.clear() — a spurious-push mutant prepends the same element to joined_parts, parent_parts, root_parts symmetrically, leaving all relative comparisons invariant; genuinely un-isolatable, correctly characterized as accepted-residual (not a deferral).
- Count-parity clean: T-006..T-056 = 51 tests, no gaps/dupes; S-21.09 owns T-012..T-056 = 45; +1 registry unit test. Consistent across module docstring, ownership header, Architecture Mapping, Purity Classification, story §Tests bullet.
- BC anchoring clean: story behavioral_contracts: [BC-4.16.001]; body BC table title matches BC-4.16.001 H1 verbatim (v1.8); PC1/PC2/PC3/PC4/Precondition 3/Invariant 1 map; AC-003/004/005 bats anchors resolve; subsystem SS-04 consistent.

## Novelty Assessment
Novelty MODERATE. F-1 is a substantive two-site doc/code contradiction reintroducing the description of a closed correctness bug, reflecting a real sibling-sweep gap from the HIGH-1/pass-9 return-contract change. The gate logic and mutation coverage themselves are converged — no logic-level gaps remain after independent re-derivation.
