# PR Review — #723 fix(config): register L1 product-brief path in artifact-path-registry

**Reviewer:** pr-reviewer (fresh-eyes, different-model cognitive diversity)
**Branch:** fix/register-product-brief → develop
**Issue closed:** #300
**CI:** GREEN (bats-full-suite linux, bats-darwin-leg macOS, bats-wave-handoff macOS, cargo-host ubuntu+macos, all build-dispatcher targets, SAST Semgrep)

## Verdict: APPROVE

Minimal, correct fix for a real governance gap. Tests lock the fix against the **real shipped registry** (not a fixture), which is the right call. No BLOCKER or MAJOR findings. Three MINOR follow-ups noted below; none block merge.

## Files reviewed (all changed files in diff)

- `plugins/vsdd-factory/config/artifact-path-registry.yaml` — new `product-brief` entry
- `crates/hook-plugins/validate-artifact-path/src/tests.rs` — two issue-#300 regression tests + helpers

## What I verified

- **Schema conformance:** New entry has all four fields (`artifact_type`, `canonical_path_pattern`, `description`, `enforcement_level`) in the same order and quoting style as the adjacent `prd` entry (path quoted, description bare). YAML parses cleanly — no colon-space in the description, so the em-dash and `skills/create-brief` slash do not create a mapping/scalar ambiguity.
- **enforcement_level: block is correct:** A *matching* block entry produces `HookResult::Continue` (write governed-and-allowed); an *unregistered* path is what triggers `ARTIFACT_PATH_UNREGISTERED`. Consistent with `prd` and other spec entries. The two tests confirm both halves: `MatchResult::Block` from `matches_canonical`, and `HookResult::Continue` from `run_logic`.
- **Placement:** `product-brief` inserted at the head of the "Product Requirements / State" section, ahead of `prd` — mirrors pipeline order (brief precedes PRD). Sensible.
- **Loop safety:** The `dir.pop()` upward walk is safe — `pop()` returns `false` at filesystem root, hitting the `panic!` branch. No infinite-loop risk. Resolution is deterministic in the monorepo (walks up from `crates/hook-plugins/validate-artifact-path` to repo root containing `plugins/vsdd-factory/config/...`).
- **Function reuse:** `load_registry` / `matches_canonical` are the same pure helpers used by neighboring `BC-4.11.001` tests; feeding them the production YAML string is appropriate and side-effect-free.

## Findings

| Severity | Category | Finding | Suggestion |
|----------|----------|---------|------------|
| MINOR | coverage | Hook-level test exercises only `make_payload("Write", …)`. `validate-artifact-path` likely also fires on `Edit`/`apply_patch`. Path-matching is tool-agnostic once the path is extracted, so `matches_canonical` covers the actual bug — this is defense-in-depth. | Add an `Edit` variant of `test_issue_300_product_brief_hook_allows_write` to guard against a future re-break if `create-brief` ever writes via `Edit`/`apply_patch`. |
| MINOR | coverage | Tests hard-panic if the shipped registry is not found in an ancestor dir. Desirable for CI (fail loud on drift) but couples the crate's unit tests to repo layout — the crate cannot be tested in a vendored/published-standalone context. | Acceptable trade-off given this is an internal workspace crate and the PR's explicit goal is to test the real registry. Flagged as a conscious decision only. |
| MINOR (informational) | dependency | Sibling registry PR (#473). No logic-level ordering dependency visible in this diff — distinct exact-match `canonical_path_pattern`s, no overlapping globs in the hunk, so first-match precedence is not a concern here. Only residual risk is a textual git conflict if #473 also inserts near the "Product Requirements / State" header. | Cannot confirm without #473's diff; would be a trivial rebase, not a semantic problem. "Applies cleanly in either order" is plausible. |

## Out-of-scope / could-not-verify (information wall)

- Exact write mechanism used by `skills/create-brief` (Write vs Edit/apply_patch).
- Whether `product-brief` `artifact_type` collides with an existing entry elsewhere in the full registry file.
- Whether any earlier broad-glob entry would shadow this exact path under first-match precedence.

All three are low-risk given the green `matches_canonical` assertion against the real registry. A follow-up could confirm.
