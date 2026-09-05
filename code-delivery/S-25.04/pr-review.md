---
document_type: pr-review
story: S-25.04
pr: 814
pr_head: 4283e2e0
reviewer: vsdd-factory:pr-reviewer
review_cycle: 3
verdict: REQUEST_CHANGES
date: 2026-09-04
---

# Fresh-Eyes PR Review — PR #814 (S-25.04), Cycle 3 at `4283e2e0`

**Verdict: REQUEST_CHANGES** — 1 blocking, 3 suggestions.

The cycle-1 (M1) and cycle-2 (mutation-resistance) findings are **both fully and verifiably
closed**. One previously-unnoticed blocking issue remains, in the committed WASM artifact.

---

## Prior-cycle findings — both CONFIRMED CLOSED

### M1 (cycle 1) — untested git-diff-failure fail-open arms — CLOSED

Both fail-open arms of the staged-path-listing `match` in `hook_logic` now have dedicated
coverage:

- `test_bc4_16_002_t10_fail_open_on_staged_path_listing_non_zero_exit` (the `exit_code != 0` arm)
- `test_bc4_16_002_pc6_fail_open_on_staged_path_listing_exec_subprocess_err` (the `Err(e)` arm)

The previously-stale comment above that `match` now cites real, existing spec anchors
(BC-4.16.002 v1.1 PC6 / Invariant 9 / EC-009 / T-10) and names both covering tests. Verified
that `HookResult::Block` as referenced in the new comment is a real SDK variant
(`crates/hook-sdk/src/result.rs:24`), so the comment is technically accurate, not aspirational.

### Cycle 2 — T-10 fixture not load-bearing — CLOSED (empirically verified)

The mutation-resistance claim was not taken on faith. The experiment was reproduced against a
clean worktree at `4283e2e0` by deleting the fail-open guard outright:

```rust
// mutant: exit-code fail-open guard removed
Ok((_exit_code, stdout, _stderr)) => find_staged_factory_path(&stdout),
```

| Fixture | Result against the mutant |
|---|---|
| **New** (`exec_ok(128, ".factory/STATE.md\n", ...)`) | `test_..._t10_...` **FAILED** — mutant killed |
| **Old** (`exec_ok(128, "", ...)`) | **42 passed, 0 failed** — mutant survives undetected |

Both halves of the claim confirmed: the cycle-2 finding was real (the old fixture provided zero
mutation coverage for that guard), and the one-line fix is genuinely load-bearing —
`test_bc4_16_002_t10_fail_open_on_staged_path_listing_non_zero_exit` is now the *sole* test in
the suite that detects deletion of the guard.

---

## Findings

| # | Severity | Category | Finding | Suggestion |
|---|----------|----------|---------|------------|
| B1 | blocking | missing / coherence | Committed `validate-factory-path-staged.wasm` is stale — reproducibly built from `d8e91606`, embedding the HIGH `NEW-1` 512-byte cap defect that `fa025c5c` fixes in this same PR, plus pre-`4bb6fb87` unsanitized telemetry | Rebuild the release WASM from HEAD source and commit it |
| S1 | suggestion | description | Test counts stale (45/45) in five PR-body locations and evidence-report AC-006; actual is 47/47 (42 unit + 5 proptest) after `c67ed890` | Update the badge, the Test Results table, "New tests \| 45 added", the "…full 45/45" row, the coverage checkbox, and the AC-006 evidence row |
| S2 | suggestion | description | PR body's "code frozen at `ff54428a`; subsequent commits … no semantic/code change" is now false — `c67ed890` and `4283e2e0` modify `src/tests.rs` | Amend to say the freeze covers *production* code, and enumerate the two review-driven test commits |
| S3 | nit | description | Blast Radius says "No existing crate or registry entry is modified"; `crates/factory-dispatcher/src/registry.rs` is modified (test-only) and the comment block above `validate-factory-path-staging`'s stanza changed | Reword: AC-003's substance (stanza key/value fields byte-unchanged) holds, but "byte-unchanged" / "no existing crate modified" are imprecise |

---

## B1 (BLOCKING) — Committed WASM artifact is stale, embeds the HIGH `NEW-1` defect

**File:** `plugins/vsdd-factory/hook-plugins/validate-factory-path-staged.wasm`

The committed binary was built at `d8e91606` and never rebuilt, despite **three subsequent
production-code commits** to `src/lib.rs` on this same branch. Confirmed by reproducible-build
bisection (release profile, `wasm32-wasip1`):

| Source revision | Built SHA-256 (first 16) | Size |
|---|---|---|
| `d8e91606` | `f862584b36327ff8` | 180,016 |
| `4bb6fb87` (telemetry sanitization) | `d267f913d362f565` | 180,006 |
| `fa025c5c` (NEW-1 cap fix) | `e58c526b89d53f26` | 180,086 |
| `ff54428a` (NEW-3 extraction) | `e58c526b89d53f26` | 180,086 |
| `c67ed890` / `4283e2e0` (HEAD) | `e58c526b89d53f26` | 180,086 |
| **committed `.wasm`** | **`f862584b36327ff8`** | **180,016** |

The committed artifact is a byte-exact match for `d8e91606` and differs from HEAD. The shipped
binary therefore embeds:

1. **The undersized `512`-byte `max_output_bytes` cap** on `git diff --cached --name-only`
   (confirmed at `d8e91606:src/lib.rs:404`) — the exact HIGH `NEW-1` self-wedge defect that
   `fa025c5c` raised to `131_072`. In that build, a routine `git add -A` staging a handful of
   `.factory/` paths can trip `OutputTooLarge` → PC3 INDETERMINATE → a spurious next-advance
   gate block. That `fa025c5c` and `ff54428a` hash identically also independently confirms
   `ff54428a` was a pure refactor and `fa025c5c` carried the behavior change.
2. **Unsanitized `branch` / `staged_path` values** reaching `emit_event` and the operator-facing
   block message, pre-dating `4bb6fb87`'s `is_ascii_graphic` control-char filtering.

**Blast radius (stated honestly).** Both automated pipelines regenerate this file from source
(`ci.yml` "Stage WASM plugins to hook-plugins directory" stages the debug build; `release.yml`
"Stage wasm plugins" stages from the freshly-built `linux-x64` leg), and the
`chore: bundle dispatcher binaries` release-bot commit re-bundles it — the sibling
`validate-factory-path-staging.wasm` history (`2e8087af` feature commit → `89f6f87c` bot
re-bundle) shows exactly that lifecycle. The *released* tarball will be correct. Exposure is
local/dev runs that load `plugins/vsdd-factory/hook-plugins/` without the CI staging step, plus
a repo committing a binary that contradicts its own PR description and CHANGELOG entry.

**Why blocking rather than advisory.** The repo convention is that the feature PR commits a
*current* artifact; the fix is one build command plus a copy; and under the Canonical Principle
(production-grade default, Rule 4 — AI-built defects are the AI's responsibility to fix),
merging a binary that embeds a HIGH defect the same PR fixes is the precise anti-pattern that
default exists to prevent. Release-time regeneration is a legitimate human risk-acceptance
override, but it should be explicit, not silent.

**Fix:**
```bash
cargo build --release -p validate-factory-path-staged \
  --bin validate-factory-path-staged --target wasm32-wasip1
cp target/wasm32-wasip1/release/validate-factory-path-staged.wasm \
  plugins/vsdd-factory/hook-plugins/
```
Expected result: `e58c526b89d53f26…`, 180,086 bytes.

---

## Checklist coverage

| # | Item | Result |
|---|------|--------|
| 1 | Diff coherence | PASS — all changes trace to S-25.04; delta since cycle 2 is exactly the described fixture line plus comment, nothing smuggled |
| 2 | Description accuracy | **FAIL** — S1, S2, S3 (stale counts, false freeze claim, imprecise blast radius) |
| 3 | Test coverage | PASS — both fail-open arms covered; T-10 empirically mutation-resistant |
| 4 | Demo evidence | PASS — `evidence-report.md` present; `.gif` + `.webm` + `.tape` for all six ACs plus a CORE recording covering both block and pass paths |
| 5 | Commit quality | PASS — conventional format, story ID on every commit, clear messages |
| 6 | Diff size | 2,679 additions — above the 500-line flag, but that is a new crate plus binary demo evidence; not a finding |
| 7 | Missing changes | **FAIL** — B1: the compiled artifact does not reflect the source changes the story claims to deliver |
| 8 | Dependency status | PASS — targets `develop`; upstream #813 already merged into the branch |

## Independent verification performed

- Mutation experiment reproduced in both directions (mutant + new fixture → killed; mutant + old
  fixture → survives).
- `cargo fmt --check --all` — clean.
- `cargo clippy -p validate-factory-path-staged -p factory-dispatcher --all-targets` — zero
  warnings/errors.
- `cargo test --workspace --all-targets` — 221 test binaries, all `test result: ok.`, zero
  failures.
- Reproducible-build bisection of the committed WASM across all five candidate revisions.
- `hook_logic` control flow read end to end against the BC-4.16.002 PC1–PC6 claims; fail-open
  arms, `factory-artifacts` bypass, product-branch block, and sanitization filter all match
  their documented contracts.
- Registry entry (`priority = 161`, `PostToolUse ^Bash$`, `fail-closed`, `on_error = "continue"`,
  `binary_allow = ["git"]`) cross-checked against crate docs, the dispatcher allowlist test, and
  the CHANGELOG.

B1 is the only item standing between this PR and an approve.

## Posting note

PR #814 is authored by the same GitHub account performing this review. The formal verdict was
submitted with `gh pr review 814 --request-changes --body-file <this file>` via `github-ops`;
GitHub's API rejected it with:

```
failed to create review: GraphQL: Review Can not request changes on your own pull request
(addPullRequestReview)
```

GitHub structurally forbids a PR's own author from submitting a `CHANGES_REQUESTED` review, so
no retry can succeed. The findings were therefore published with
`gh pr review 814 --comment --body-file` — still a formal review record on the PR, in COMMENTED
state, which is the only state GitHub permits here. The REQUEST_CHANGES verdict recorded in this
file is the operative one for triage.
