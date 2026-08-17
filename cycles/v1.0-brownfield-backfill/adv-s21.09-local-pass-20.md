---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: vsdd-factory:adversary
timestamp: 2026-08-13T07:00:00Z
phase: 20
inputs: [.factory/stories/S-21.09-wasm-artifact-restore-and-registry-parity.md]
input-hash: "48547f6"
traces_to: S-21.09-wasm-artifact-restore-and-registry-parity.md
pass: 20
previous_review: adv-s21.09-local-pass-19.md
---

# Adversarial Review — S-21.09 LOCAL cascade (fresh-context pass, strengthened rubric)

Reviewed: story v1.29, BC-4.16.001 v1.8, impl at .worktrees/S-21.09 @ c9cccea9 (bundle_orphan_check.rs 51 tests T-006..T-056 + registry.rs unit test), policies 1-13+. Verified AC-001 delivery (WASM present on disk + registry entry at hooks-registry.toml:944-947), count parity (51 / T-006..T-056 / 45-owned + 1 registry), version cites (BC v1.8), SURV-01 accepted-residual doc comment present.

The suite is genuinely excellent — mutation narratives are accurate, should_panic expected= strings are tight (T-019/027/052/053/054 pin exact values; T-054's -1 sentinel pin correctly excludes the -2 near-miss), the negative-identifier assertions (T-043/047/050/051) correctly distinguish the colon-form finding lines from the equals-form header prose, and the T-048 tautology / T-026/T-015 vacuity issues flagged by the earlier PR-review were correctly closed in c9cccea9. But one genuine defect remains.

## Critical Findings
None.
## High Findings
None.

## Medium Findings

### F-S2109-LOCAL-FMTLOCK-01 — check_registry_inventory two-space-indent format is not locked by any assertion (sibling of the same-commit PR-review indent fix, not swept) — MEDIUM, HIGH confidence, POLICY 13 (mutation-completeness) + FORMAT-LOCK FIDELITY
Location: Production: check_registry_inventory emits format!("  UNEXPECTED: {}", name) and format!("  MISSING: {}", name) — both two-space indent (block after `if unexpected.is_empty() && missing.is_empty()`). Assertions that read them: T-017 (msg.contains("UNEXPECTED: metrics-registry.toml")), T-018 (contains("MISSING: hooks-registry.toml")/"MISSING: resolvers-registry.toml"), T-024 (contains("UNEXPECTED: metrics_registry.toml")), T-030 phase A (.contains("UNEXPECTED: taplo.toml")).
Why it's a defect: every one of those assertions uses an UNINDENTED needle. .contains() on the unindented needle matches inside the real indented "  UNEXPECTED: …"/"  MISSING: …" line, so all four tests stay GREEN under a mutation dropping the two-space indent from check_registry_inventory's two format strings. No test asserts the indented inventory form (grep-confirmed: the only two contains("  <IDENT>: …") sites are T-015 and T-021, both targeting check_declared_subset_tracked, a different function). The indent-dropping mutation on check_registry_inventory survives the entire 51-test suite.
This is precisely the defect class the PR-review fix in this same commit (c9cccea9) closed for the sibling lines. T-015's own comment states the unindented assertion "could never catch an indent-dropping mutation" and that "asserting the indented form here closes that gap." That fix was applied to check_declared_subset_tracked's MISSING (T-015) and STAGED-NOT-COMMITTED (T-021) lines, but NOT swept to the byte-identical two-space-indent pattern in check_registry_inventory (T-017/T-018/T-024/T-030A). Partial-fix / sibling-sweep miss (S-7.01) + format-lock gap.
Adjacent site (same class): run_t012_gate's ungated-block emitter format!("  {}", p) (the "  {}" line joining UNGATED-DECLARATION/OUTSIDE-REPO-DECLARATION findings) is likewise never indent-locked — every consumer (T-038/043/044/045/046/047/050/051) asserts the unindented "<IDENT>: <path>" needle, so dropping that "  " prefix also survives.
Scope note: the story's Mutation-Completeness Audit (SURV-01..05) explicitly scoped only run_t012_gate, detect_ungated_declarations, lex_norm, and registry.rs — check_registry_inventory was not in audited scope, and the classification-literal swaps the audit enumerated covered the IDENT: tokens, not the leading indent. So this surviving mutant does not contradict the audit's "zero killable surviving mutants" claim (which is scoped), but it is a real, killable, uncaught mutant in a function the T-012 gate depends on.
Minimal fix route: change the inventory-line assertions to the indented needle matching the T-015/T-021 pattern — T-017 → contains("  UNEXPECTED: metrics-registry.toml"); T-018 → "  MISSING: hooks-registry.toml" and "  MISSING: resolvers-registry.toml"; T-024 → "  UNEXPECTED: metrics_registry.toml"; T-030A → "  UNEXPECTED: taplo.toml". Optionally sweep the ungated-line consumers (T-038/043/…/051) to "  UNGATED-DECLARATION: …"/"  OUTSIDE-REPO-DECLARATION: …" to lock run_t012_gate's "  {}" prefix.

## Observations
- The suite is unusually mature; the PR-review fixes in c9cccea9 (T-015/T-021 indent, T-026 bare-basename vacuity removal, T-048 tautology collapse, T-054 sentinel-pin tightening, T-055 positive control) are all correctly landed and verified against the actual code. The single finding above is the one instance of the indent-lock fix that did not propagate to its siblings.
- AC-001 is genuinely delivered: plugins/vsdd-factory/hook-plugins/validate-factory-path-staging.wasm present in the worktree checkout (force-tracked, gitignored path) and registry entry exists (hooks-registry.toml:944-947), so run_t012_gate(&workspace_root()) (T-012) exercises the real declared/tracked parity.

## Novelty Assessment
Novelty LOW–MEDIUM. One substantive, previously-un-swept format-lock gap (not a nitpick — a concrete surviving mutant in a T-012-load-bearing function), plus confirmation that the prior PR-review defect classes are otherwise closed.

## Verdict
NOT CLEAN. One genuine MEDIUM finding (F-S2109-LOCAL-FMTLOCK-01). Per BC-5.39.001 3-CLEAN, this pass does not advance the streak. Routes to test-writer for assertion tightening; a same-commit sibling-sweep completion of the existing PR-review indent fix.
