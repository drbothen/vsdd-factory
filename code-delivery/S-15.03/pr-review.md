# PR #805 — Fresh-Eyes Review, Cycle 2

**Story:** S-15.03 — `last_amended` Write-Path Durable Fix (`last-amended-migrate` CLI + write-path discipline)
**Branch:** `feature/S-15.03` → `develop`
**Merge-base:** `8b4b60e6`
**Reviewed range:** `git diff origin/develop...HEAD` (46 files, 5823 insertions, 0 deletions)

## VERDICT: REQUEST_CHANGES — 1 blocking, 4 suggestions, 3 nits

Gates verified independently in the worktree, not taken on trust:

| Gate | Result |
|------|--------|
| `cargo fmt --check --all` | clean |
| `cargo clippy --workspace --all-targets -- -D warnings` | clean, 0 warnings |
| `cargo test -p last-amended-migrate` | 81/81 passing across 15 test files |
| Diff additivity | confirmed additive-only (0 deletions) |

---

## Cycle-1 blocking findings — re-verification

| Finding | Verified status |
|---|---|
| **B1** — PC7 split emits invalid YAML from colon-prefixed non-date chain-entry prefixes | **RESOLVED.** `render_item_block` (`changelog.rs`) now emits `  - date: "{}"` quoted, and `migrate.rs` routes `date` through `needs_escaping`/`escape_value` alongside `version`/`summary`. Coverage is real and load-bearing: `bc_10_13_001_pc7_date_quoting_test.rs` carries both an end-to-end `migrate_file` test over a genuine `D-1149:`-prefixed unversioned chain entry and a unit test on the rendering primitive, each asserting `strict_yaml_parse` succeeds. Independently confirmed the pre-fix shape (`date: D-1149:`) is a real YAML scanner error. |
| **B2** — PC7 split on STATE.md silently drops recovered entries | **PARTIALLY resolved — the false-positive determination is too broad.** The coverage gap is genuinely closed (`bc_10_13_001_ec006_pc7_state_md_split_test.rs` does exercise `migrate_file`'s PC7 path against a STATE.md fixture end-to-end), but it pins the behavior without addressing the defects surrounding it. See **B2-R** below. |
| **B3** — BC-4.18.001 fuel-relief proof was vacuous | **RESOLVED, and well done.** `realistic_multi_entry_prior_chain(100, 3_000)` builds a ~300K chain of 100 modest entries; `test_BC_4_18_001_B3_realistic_multi_entry_chain_relief_is_per_line_bounded` asserts both halves of the claim — `last_amended` shrinks AND `longest_line < 4_000` AND `after.matches("  - date:").count() == 100` AND strict-YAML sees exactly 100 items. That combination genuinely catches a "moved the problem, didn't fix it" regression. The scope note separating the monolithic 350K fixture (VP-110 bounded scan) from fuel relief (VP-115) is accurate. |
| **B4** — PR description overclaimed `eligibility.rs` path enforcement | **RESOLVED.** The Security Review section correctly attributes enforcement to `path_guard.rs`/`cli.rs` and carries an explicit correction of the prior overclaim. `path_guard.rs` does what it claims. |

---

## BLOCKING

### B2-R — Shipped operator docs and CLI output both state that `migrate` relocates STATE.md's chained entries; the code deletes them

| Field | Value |
|---|---|
| Severity | **blocking** |
| Category | coherence / description accuracy / silent data loss |
| Files | `plugins/vsdd-factory/skills/state-burst/SKILL.md` (Recovery block); `plugins/vsdd-factory/agents/state-manager.md` (Recovery mode block); `crates/last-amended-migrate/src/migrate.rs` (`migrate_file`, the `if !is_state {}` guard); `crates/last-amended-migrate/src/cli.rs` (`run`, the `entries_recovered={}` println); PR description (`migrate` subcommand table row) |

This finding does **not** re-litigate "STATE.md never gains a `changelog:` field". That is accepted as spec-mandated, and is independently asserted by `state-burst/SKILL.md` §3 within this diff. The blocking issue is that three artifacts *inside this diff* tell the reader the opposite of what the code does, on an irreversible operation.

**What the code does for STATE.md** (`migrate.rs::migrate_file`): `set_last_amended` truncates `last_amended` to the current entry, destroying the chain text; `split_tail_entries` computes N entries; `ensure_changelog_field` returns `SkippedStateFile`; the `if !is_state` guard skips the prepend loop entirely. The N entries are dropped on the floor and the truncated file is written. This is the only lossy path of the five — the other four are lossless relocations.

**What this PR ships as documentation**, with no STATE.md carve-out in either Recovery block:

- `state-burst/SKILL.md`: *"The tool splits the chain in place — the current entry stays in `last_amended`; every chained historical entry is relocated into `changelog:` as a new item, newest-first, verbatim (D-1144-escaped)"* — introduced by *"if a mega-line/inline `[Prior: ...]` chain is ever detected on one of the 5 files"*.
- `agents/state-manager.md`: *"This splits the chain in place (current entry stays in `last_amended`; every chained entry relocates into `changelog:`, newest-first, verbatim, D-1144-escaped)"* — same 5-file framing.
- PR description, `migrate` row: *"relocates every chained historical entry into `changelog:` newest-first, verbatim — recovering 100% of a legacy mega-line with zero data loss."*

Both SKILL.md and state-manager.md carve STATE.md out in their *write-path* steps (step 3 / the third bullet) but **not** in their *Recovery* steps. A state-manager following the shipped prompt runs `migrate --path .factory/STATE.md` — or, following the SKILL's *"re-run the same subcommand without `--check` to apply"*, plain `migrate`, which reaches all five files via `migrate_all` — believing history is being relocated, when it is being deleted.

The CLI reinforces the false belief: `cli.rs` prints one identically-worded line per file, so a STATE.md run reads `entries_recovered=347 mutated=true`. The new EC-006 test's own doc comment concedes the semantics is *"materially different"* for STATE.md while calling it *"still accurate and non-misleading"* — a shared field name in a shared output line with no distinguishing marker is not non-misleading.

Two further points on the false-positive reasoning itself:

1. **The premise is systematically weakest exactly where PC7 fires.** The justification is that the body Decisions Log already holds the content because the write-path discipline requires writing it in the same burst. But a chain on STATE.md's `last_amended` is *proof the discipline was not followed* — the test's own comment calls it "a discipline violation `migrate --path` exists to recover from". Worse, that discipline is **introduced by this PR**; any pre-existing chain accumulated under the old read-wrap-rewrite path, which carried no such body-write guarantee. The tool never verifies the premise before acting on it.
2. **The repo's own precedent is preservation, not deletion.** This PR registers `.factory/STATE-amendment-history.md` in `artifact-path-registry.yaml` as the *"Frozen pre-migration D-1149 amendment-history sidecar for STATE.md"*. The human D-1149 surgery relocated displaced STATE.md history into a sidecar; the automated successor deletes it. "There is nowhere to put it" is not true of this repo.

**Suggested fix — minimum bar for unblocking (cheap, in-scope):**

1. Add the STATE.md carve-out to **both** Recovery blocks (`state-burst/SKILL.md`, `agents/state-manager.md`) and to the PR description's `migrate` row: state plainly that on STATE.md the chained entries are **stripped, not relocated**, and that the operator must confirm the body-level `## Decisions Log` already carries them before applying.
2. Make the report distinguish the two dispositions — e.g. add a `RecoveryDisposition { Relocated, Discarded }` to `FileMigrationReport` (or an `entries_discarded` field), and have `cli.rs` print a distinct, loud line for the discarded case rather than reusing `entries_recovered=N`.

**Preferred (fully production-grade) route:** have `migrate_file` **refuse** the STATE.md split with a specific `MigrateError` variant naming the disposition, directing the operator to relocate the entries into the body log or a sidecar first, with an explicit opt-in flag (e.g. `--discard-state-chain`) to proceed. That preserves EC-006's "no `changelog:` on STATE.md" mandate without making irreversible deletion the default of a command documented as lossless. Either route closes the finding; route 1 is the minimum.

---

## SUGGESTIONS

### S1 — `escape_value` does not neutralize literal backslashes; SEC-001's escaping is still incomplete

| Field | Value |
|---|---|
| Severity | suggestion |
| Category | coverage / correctness (CWE-116 class) |
| File | `crates/last-amended-migrate/src/escape.rs` |

`escape.rs` passes `\` through verbatim, to preserve idempotency for already-escaped `\"`. Consequence: any source value containing `\` followed by a non-escape character, or a trailing `\`, renders as an invalid YAML double-quoted scalar. Verified independently: `a: "C:\path\to"` and `a: "ends with backslash\"` are both scanner errors; `a: "ok \n esc"` parses. This is **fail-closed** — `yaml_guard::validate_frontmatter_yaml` catches it and returns `InvalidYamlProduced` before any write, so there is no corruption — but the tool then cannot remediate that file at all, which is the failure mode BC-10.13.001 exists to prevent. Not blocking (fail-closed, and backslashes are unlikely in this corpus), but it is the same CWE-116 class SEC-001 set out to close.

**Fix:** on the PC7/legacy-remediation path, treat a `\` not followed by a valid YAML escape character as literal data and emit `\\`; add fixtures with `C:\path` and a trailing-backslash summary.

### S2 — PR description numbers are stale against the current diff

| Field | Value |
|---|---|
| Severity | suggestion |
| Category | description accuracy |

Several figures predate the seven fix commits: the badge reads `tests-2710/2710` while the Coverage Summary reads `2743/2743` (and the Pre-Merge Checklist repeats `2710/2710`); the `last--amended--migrate-48/48` badge vs `~81 tests` in the table (81 is correct — counted); *"28 files changed, 3686 insertions"* vs the actual 46 files / 5823 insertions; the Area table's `11` test files vs the actual 15. The "New Tests (by BC/PC)" table also omits all six files added after cycle 1 (`sec001`, `sec002`, `sec003`, `ec006_pc7`, `pc7_date_quoting`). Refresh them.

### S3 — Pre-Merge Checklist contradicts the Security Review section

| Field | Value |
|---|---|
| Severity | suggestion |
| Category | description accuracy |

`- [ ] No critical/high security findings unresolved — pending dedicated security-reviewer pass` is unchecked, while the Security Review section states the pass ran and all three findings (SEC-001/002/003) are fixed in-PR. Check the box or state what is still pending.

### S4 — `validate_registry_path` allowlists on basename only

| Field | Value |
|---|---|
| Severity | suggestion |
| Category | security depth |
| File | `crates/last-amended-migrate/src/path_guard.rs` |

Accepts any path whose basename is `artifact-path-registry.yaml`, anywhere on disk; `register_artifact_paths` then appends to it. Low risk under the CLI-boundary threat model and an improvement over nothing, but calling it an "allowlist" overstates it. Consider anchoring it to a `plugins/vsdd-factory/config/` ancestor the way `validate_rotate_path` anchors to `.factory/`.

---

## NITS

| ID | Finding |
|----|---------|
| **N1** | `frontmatter.rs`'s module doc still claims *"A YAML library (`serde_norway`) IS used, but only in this crate's `dev-dependencies` … never in this production hot path."* SEC-001 promoted it to a real dependency and `yaml_guard.rs` uses it in production. The scoping claim (not in *this* hot path) remains true; the dev-dependency claim is now stale. |
| **N2** | `atomic_write::write_atomic` does not preserve the target's file mode (the renamed temp file gets fresh default perms) and does not `fsync` before rename, so a crash can leave a durable rename over non-durable content. Both minor for `.factory/` markdown; worth a doc line if intentional. |
| **N3** | Demo evidence covers AC-006/008/009/010 with real `.gif` + `.webm` per AC and both success and dry-run/error paths — genuinely good, and the disclosure about recording against uncommitted SEC hardening is honest. AC-004/AC-005 (skill + agent-prompt codification) have no recording; plausibly non-demoable, but `evidence-report.md` should say so explicitly rather than omit them. Separately, no recording exercises the STATE.md path — the "zero data loss" claim is demonstrated only on the lossless file class. |

---

## 8-Item Checklist Outcome

| # | Item | Result |
|---|------|--------|
| 1 | Diff coherence | PASS — all changes trace to S-15.03 scope extension; no unrelated changes |
| 2 | Description accuracy | **FAIL** — B2-R (`migrate` row overclaims zero data loss), S2 (stale counts), S3 (checklist contradiction) |
| 3 | Test coverage | PASS — 81 tests, 15 files; B1/B2/B3 each carry load-bearing regression guards. Gap noted in S1. |
| 4 | Demo evidence | PASS — `docs/demo-evidence/S-15.03/` has `evidence-report.md` plus `.gif` + `.webm` + `.tape` per AC, success and dry-run/error paths. Nit N3. |
| 5 | Commit quality | PASS — conventional format, story ID in every subject, clear bodies, session trailer, no AI attribution |
| 6 | Diff size | Large (5823 insertions) but justified: net-new crate, additive-only, no existing behavior altered |
| 7 | Missing changes | **FAIL** — B2-R: Recovery-path documentation does not match shipped behavior for one of the five governed files |
| 8 | Dependency status | PASS — merge-base `8b4b60e6` is an ancestor of `origin/develop`; no upstream PR dependency |

---

## Scope note

Per the information-asymmetry wall, nothing under `.factory/` was read during this review. Consequently BC-10.13.001 v1.1 EC-006's text could not be verified as pre-dating cycle 1, nor as reading the way the new test's doc comment quotes it. **B2-R is deliberately framed so it does not depend on that** — every artifact it cites lives inside this PR's diff. If EC-006 provenance needs confirming (v1.1 vs the v1.2 the traceability table cites in several rows — the two versions are used inconsistently across the description, tests, and commit messages), that check requires an agent on the other side of the wall.
