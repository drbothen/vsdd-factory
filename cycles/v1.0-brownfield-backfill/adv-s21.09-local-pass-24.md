---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: vsdd-factory:adversary
timestamp: 2026-08-13T12:00:00Z
phase: 24
inputs: [.factory/stories/S-21.09-wasm-artifact-restore-and-registry-parity.md]
input-hash: "d8e69cf"
traces_to: S-21.09-wasm-artifact-restore-and-registry-parity.md
pass: 24
previous_review: adv-s21.09-local-pass-23.md
---

# Adversarial Review — S-21.09 (LOCAL cascade pass)

Artifacts: story spec v1.31; worktree .worktrees/S-21.09 @ 1c93f499; bundle_orphan_check.rs (51 tests T-006..T-056); registry.rs; governing BC-4.16.001 v1.8; policies.yaml (POLICY 1-22).

## Verdict: CLEAN
No finding rises to BLOCKER, HIGH, MEDIUM, LOW, or NIT. This deliverable is genuinely clean.

## What I verified (actual adversarial probes)
Gate functions under test (all in the test crate) + production coupling: lex_norm, extract_hook_plugin_name, detect_ungated_declarations, parse_plugin_refs, check_registry_inventory, check_declared_subset_tracked, run_t012_gate, git_tracked_wasm_names, git_committed_wasm_names — read the real definitions. The three-gate resolution logic is single-copy (detect_ungated_declarations delegates to extract_hook_plugin_name), closing the complementarity-drift surface honestly. run_t012_gate genuinely couples to production factory_dispatcher::Registry::parse_str for hooks validation (isolated by T-052), so the gate is not vacuous against a registry production would reject.

Assertion-quality lens (VACUITY / TAUTOLOGY / MUTATION-NARRATIVE / should_panic PRECISION / FORMAT-LOCK): #[should_panic(expected=…)] strings match production panic/Err text verbatim: T-019 ("has only 1 entries"), T-020 ("EC-005b"), T-022 ("resolvers registry declared set is empty"), T-027 ("only 29 entries"), T-041 ("git ls-files exited with status"), T-049 ("EC-005a"), T-052 ("fails production validation"), T-053 ("schema_version=2 but production requires 1"), T-054 ("schema_version=-1 but production requires 1"). Each git-fixture should_panic test commits a full valid fixture so the deletion mutant produces a clean Ok(()) rather than an incidental downstream panic — the controls are true sole-determinant isolations.
Mutation-isolation controls trace correctly to code: T-050 (length conjunct >→>=, ../.. → exactly root_parts), T-051 (prefix .all→.any/true, ../../../sib/ghost.wasm), T-054 (.unwrap_or(-1) sentinel, absent-key fixture), T-055 (fail-open Err(_) arm + positive control closing the whole-function-Vec::new() gap), T-056 (CurDir arm; ./a/b → ["a","b"] matches Path::components() semantics). All narratives match actual code behavior.
T-048 totality verified non-tautological: single exact-vec-equality per case; the previously-tautological paired extract.is_some()/is_none() was removed (PR-review). Classifications (/abs/hook-plugins/foo.wasm→OUTSIDE, HOOK_PLUGINS/foo.wasm→UNGATED via hyphen≠underscore, bare hook-plugins→UNGATED via gate-1) all trace correctly.
Format-lock (D-970 outcome-identifier) fidelity: the story's pass-21 "format-lock sweep COMPLETE" claim is a concrete, checkable assertion — grepped it. Every positive-identifier assertion uses the two-space-indented needle ("  MISSING:", "  UNGATED-DECLARATION:", etc.); negative-identifier assertions correctly test the opposite identifier unindented. No unindented positive needle survives against any indented production emitter. Claim holds.

Spec↔impl / anchoring / count integrity: BC-4.16.001 H1 ↔ story body Behavioral Contracts table title: verbatim (POLICY 7). Subsystem SS-04 / CAP-034 / traces_to consistent across story frontmatter, body, BC (POLICY 4/6). Count parity holds: 51 functions T-006..T-056 (consecutive, none skipped) / 45 S-21.09-owned T-012..T-056 / +1 registry.rs unit test. Token Budget BC count = 1 = len(behavioral_contracts) (POLICY 8). SHA currency: current-state cites = 1c93f499 = HEAD; Changelog v1.31 = 1c93f499 = frontmatter version. No stale-vs-HEAD drift (TD-VSDD-091). Accepted residuals (SURV-01 un-isolatable dead arm; LOW-3 enabled=false; F8 non-recursive staging) honestly characterized as audited residuals, not coverage gaps.

## Honest scope limitations (not findings)
AC-001 git-tracking not independently verifiable by me (read-only, no Bash). hook-plugins/ is gitignored, so Glob (which honors .gitignore) returned "No files found" for all 35 committed WASMs in that directory — an expected gitignore artifact, NOT evidence of absence. AC-001 is mechanically verified by T-012's real-tree declared ⊆ tracked assertion in CI, which I cannot execute. I explicitly did NOT manufacture a "WASM missing" finding from the inconclusive Glob (the exact false-positive class the worktree-identity preflight warns against). AC-002/003/004/005 are covered by pre-existing bats tests (validate-factory-path-staging.bats T-001/T-002/T-004) outside the 51-test Rust scope; the story characterizes their verification honestly.

## Transparency note (explicitly NOT a suppressed finding)
S-21.09 frontmatter carries no last_amended:/modified: fields while having a ## Changelog body section. Checked whether this is a POLICY 17 parity gap: sibling cohort S-21.09/10/11/12/13 all share this exact pattern (body-changelog-only), while the earlier S-21.01-07 cohort uses frontmatter fields — indicating an intentional convention shift, consistent with the CLAUDE.md changelog-migration-deferred-to-S-15.03 note and the story's own frontmatter flag that state-manager seal obligations (POLICY 18 three-way hash) are owed at first state-manager burst. This is a cross-cohort convention matter pending a state-manager seal, not a defect localized to S-21.09. Surfaced for transparency; does not constitute a finding.

## Novelty Assessment
Novelty LOW-toward-zero. Re-derived the gate logic and assertion quality from scratch and found the deliverable at a genuinely converged, production-grade state — mutation-complete across the audited scope with every killable survivor closed and the one un-isolatable residual (SURV-01) correctly recorded as accepted rather than papered over. CLEAN is the correct and expected verdict.
