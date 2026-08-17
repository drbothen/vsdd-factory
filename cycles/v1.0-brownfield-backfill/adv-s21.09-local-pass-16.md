---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: vsdd-factory:adversary
timestamp: 2026-08-13T00:30:00Z
phase: 16
inputs: [.factory/stories/S-21.09-wasm-artifact-restore-and-registry-parity.md]
input-hash: "48547f6"
traces_to: S-21.09-wasm-artifact-restore-and-registry-parity.md
pass: 16
previous_review: adv-s21.09-local-pass-15.md
---

# Adversarial Review — S-21.09 LOCAL cascade (fresh-context pass)

Reviewed: story v1.27 against impl 05480619 (bundle_orphan_check.rs 51 tests T-006..T-056 + registry.rs on_error unit test), BC-4.16.001 v1.8, full POLICY 1–22 rubric.

## What I verified as sound (no findings)
- AC-006 gate logic (run_t012_gate, extract_hook_plugin_name, lex_norm, detect_ungated_declarations, check_declared_subset_tracked, check_registry_inventory) matches the spec narrative exactly: three-gate extraction, two-conjunct containment predicate, per-registry floors (hooks >= 30, resolvers >= 1), production-validation gate (Registry::parse_str for hooks, schema_version==1 assert for resolvers with .unwrap_or(-1) fail-closed sentinel), four-way outcome identifiers.
- Conjunct isolation is genuine, not tautological. T-050 (plugin="../.." → exact root_parts self-match) isolates the length conjunct; T-051 (plugin="../../../sib/ghost.wasm") isolates the prefix conjunct with an orthogonal candidate. Both carry the load-bearing negative-identifier assertion (!err.contains("UNGATED-DECLARATION: ")). The claimed over-determination of T-047/T-048 is accurate.
- Gate-determinant controls exist and match: T-033/T-026(b)/T-035 (gates 1/2/3), T-052/T-053 (production-validation deletion), T-054 (.unwrap_or(-1) sentinel), T-055 (fail-open Err(_) arm, direct call), T-056 (lex_norm CurDir arm, direct call). T-054/T-055/T-056 bodies assert precisely what the Red-Gate table and Mutation-Completeness Audit claim. SURV-01 accepted-residual (lex_norm RootDir|Prefix clear) is correctly argued provably-unreachable-with-nonempty-parts.
- Count parity holds everywhere: 51 tests T-006..T-056 (contiguous, no gaps), 45 S-21.09-owned T-012..T-056, +1 registry unit test (on_error_falls_back_to_registry_defaults_when_entry_omits_it, present in registry.rs). Token Budget, Architecture Mapping, Purity Classification, and body all agree.
- No stale current-state SHA cites: every current-state cite is 05480619 (= HEAD under review); the 12f280d1 byte-provenance cite is explicitly historical-by-construction.
- Concrete artifact claims confirmed: validate-factory-path-staging.wasm present on disk under hook-plugins/; top-level plugins/vsdd-factory/ contains exactly {hooks-registry.toml, resolvers-registry.toml} (inventory passes); the "expect 4" grep-count claim is exactly right (2 comments + name + plugin); hooks schema_version=2, resolvers schema_version=1 — real tree satisfies the production-validation gate.
- BC frontmatter [BC-4.16.001] matches the body Behavioral Contracts table (v1.8); verification_properties: [] honestly justified by TBD VP rows.
- Could not independently confirm git-tracked status of the WASM (read-only, no Bash) — on-disk presence plus T-012/T-009 standing gates give no counter-evidence.

## Findings

### LOW
F-L-01 — Module docstring "Stories:" provenance line mis-attributes T-011 to S-19.04
POLICY 4 (semantic anchoring, description-bearing provenance).
Location: crates/factory-dispatcher/tests/bundle_orphan_check.rs, module docstring closing line //! Stories: S-19.04 (T-006..T-011), S-21.09 (T-012..T-056).
Problem: T-011 is an S-19.06 test — the function is test_S_19_06_policy20_T011_read_prefix_fixture_passes_staging_and_is_orphan, the same file's Test-Plan row for T-011 labels it (S-19.06), and the story's own Previous Story Intelligence table attributes T-006..T-010 to S-19.04 and T-011 to S-19.06. The "Stories:" line range S-19.04 (T-006..T-011) over-claims T-011 for S-19.04 and contradicts three in-scope sources. This line was edited during S-21.09 (endpoint is the newly-added T-056), so the boundary should have been corrected in the same sibling-sweep.
Impact: documentation-only; no effect on gate correctness, test execution, AC coverage, or count parity. It mislabels which story owns T-011.
Fix: change to //! Stories: S-19.04 (T-006..T-010), S-19.06 (T-011), S-21.09 (T-012..T-056). Confidence: HIGH.

### NIT
F-N-01 — T-056 comment calls a leading ./ CurDir "interior"
Location: same file, test_...T056_lex_norm_curdir_arm_direct_contract_pin doc-comment ("...with a path that has a literal interior . component").
Problem: the fixture Path::new("./a/b") has a leading CurDir (which Path::components() preserves and which is exactly why the arm is reachable); an interior . (a/./b) is normalized away by std — as the preceding comment correctly states. The word "interior" contradicts the mechanism the same comment relies on. The test itself is correct; only the wording is imprecise.
Fix: replace "interior" with "leading". Confidence: HIGH.

## Verdict
NOT CLEAN — one LOW finding (F-L-01) plus one NIT (F-N-01). Per BC-5.39.001, the LOW finding keeps the LOCAL 3-CLEAN streak at 0/3. F-L-01 is an in-scope internal inconsistency in a line this story edited; not manufactured; nothing at MEDIUM or above exists in these artifacts.

## Coverage / What came up clean
- AC-001 git-tracked artifact present; T-012 executable gate. T-012 real end-to-end gate, not a tautology; POLICY 11 fixture controls call shared functions not replicas. Non-vacuity floors pinned (T-016/T-027/T-019, T-022, T-049, T-020). Mutation isolation claims verified sound (T-033, T-026b, T-035/T-031, T-032, T-034, T-037, T-050, T-051, T-052, T-053, T-054, T-055, T-056, registry on_error). SURV-01 honestly un-isolatable. Count-parity clean (51/T-006..T-056, 45 owned + 1 registry). BC anchoring clean (BC-4.16.001 v1.8 H1 verbatim; PC/Precondition/Invariant map; AC-003/004/005 bats anchors resolve; SS-04 consistent).

## Novelty Assessment
Both findings are documentation-accuracy issues, not gate/coverage/logic defects. The substantive deliverable — declared→tracked parity gate, determinant isolation, mutation-completeness closure, count parity, SHA currency — is sound and well-hardened. No BLOCKER/HIGH/MEDIUM.
