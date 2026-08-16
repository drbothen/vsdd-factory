# PR #777 — Final Fresh-Eyes Review (pr-reviewer) — ROUND 3

- **PR:** #777 — fix(policy15-gate): implement POLICY 15 attestation-location gate as Rust crate
- **Branch:** feature/policy15-gate-rust → develop
- **Reviewed HEAD:** 6854b25fd00ea3ed8809cc5998db39c0962f2452 (LOCAL)
- **New since round 2 (HEAD 010e6140):** 6c1ad212 (fix), 6854b25f (tests)
- **Reviewer:** vsdd-factory:pr-reviewer (fresh-eyes, round 3)
- **Date:** 2026-08-15
- **Supersedes:** round-2 REQUEST_CHANGES (same file, prior content)
- **Posted to GitHub:** NO — deliberately scoped LOCAL review. The launching agent
  instructed no PR post (gh write actions are blocked by this session's permission
  classifier); verdict returned to the launching agent directly. This file is the
  local artifact of that review, not a GitHub post.

## VERDICT: APPROVE — ready to merge to develop

All 2 HIGH + 3 MEDIUM round-2 findings and CR-1..CR-5 are genuinely resolved,
verified by **executing the compiled binary against real scratch git repos**
(ADR-040 §Consequences) plus load-bearing mutation spot-checks. No regressions
found in the GateResult refactor.

- `cargo test -p policy15-attestation-gate`: 28/28 green (22 lib + 5 binary-integration + 1 doctest)
- `cargo clippy -p policy15-attestation-gate --all-targets -- -D warnings`: clean
- `cargo fmt --check`: clean

## Round-2 findings — disposition

| Finding | Status | Verification (executed, not read) |
|---|---|---|
| H-1 merge false-FAIL | RESOLVED | PR#777-shaped `--no-ff` merge of already-attested crate content → binary `PASS-1-activations` exit 0; inert merge combined diff empty → `skipped_merge_inert`. |
| H-1 bypass stays closed | RESOLVED | Merge whose combined diff adds a crate `.rs` in neither parent, no attestation → `FAIL` exit 2, keyed on first parent. |
| H-2 quotePath false-PASS | RESOLVED | Non-ASCII `café.rs`: no attestation → FAIL exit 2 (proves activation); with attestation → PASS-1 exit 0. Demonstrated default quoting yields `"…caf\303\251.rs"` (fails prefix+suffix). Applied to both two-dot and combined-diff calls. |
| M-1 `_ => {}` wildcard | RESOLVED | Explicit `PassWithActivations(_)` / `PassZeroActivations` arms; only remaining `_ =>` is in a comment. |
| M-2 stale EXPECTED-RED docs | RESOLVED | Grep for expected_red / pending_implementer / EXPECTED RED / `_expected_red_pending_implementer` → none. (v1.17 cite at lib.rs:1298 is a legitimate historical reference, not a stale bump.) |
| M-3 ambiguous-attestation mislabel | RESOLVED | New `FailReason::AttestationAmbiguous { count }` for count>=2, distinct from `AttestationMissing` (count==0); `count != 1` boundary unchanged; `test_f2` drives count==2. |
| CR-1..CR-5 | ALL LANDED | v1.18 cites; structural GateResult wrapper + WARNING moved to main.rs from `skipped_parentless`; `run_git_line`/`run_git_lines` dedup; `commit_has_parent` returns resolved parent SHA (reused); `short_sha` extraction. `git_rev_parse` fully removed, no dangling refs. |

## Refactor regression hunt — all clear

- Guard ordering: stale-pin before range check in both entry points; verified live.
- Exit-code fidelity: 0 pass / 2 fail+unreachable / 1 hard-error; confirmed.
- Combined-diff parsing: `--no-commit-id` suppresses the SHA line; inert merge → empty list.
- GateResult field population: all 5 construction sites populate both skipped_* fields.
- Merge + `--allow-empty`: empty combined diff → `skipped_merge_inert` (routine), never `UnmeasurableDiff` (reserved for parent-count==1). Correct-by-design.

## Mutation load-bearing spot-check
- Disable merge combined-diff path → `test_h1_merge_pass_through...` FAILS.
- Drop `core.quotePath=false` → `test_h2_non_ascii_path_activates_gate` FAILS.
Both mutants killed; tree restored clean.

## Non-blocking observation (NIT — no action required this PR)
Diff helpers use `--name-only` line-splitting, not `-z` NUL-delimited parsing.
`core.quotePath=false` fixes non-ASCII paths (the H-2 class) but git still C-quotes
control-char (embedded newline/tab) filenames. Pathological for real `.rs`/`.bats`
source; not a refactor regression. Production-grade hardening would use `-z`. Mentioned
for completeness only.

## Bottom line
Production-grade. Ready to merge to develop.
