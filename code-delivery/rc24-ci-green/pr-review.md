# pr-reviewer — fresh-eyes review of PR #782

**PR:** [#782](https://github.com/drbothen/vsdd-factory/pull/782) — `fix(ci): rc.24 CI-green prep — test-harness fixes + h2 0.4.19 RUSTSEC-2026-0258 bump`
**Branch:** `fix/rc24-ci-green` → `develop`
**Head:** `285adc95`
**Verdict: REQUEST_CHANGES** (1 blocking, 2 suggestions, 1 nit)

Reviewed all 4 changed files in the actual diff (`gh pr diff 782`), not the description narrative. 42 insertions / 14 deletions.

---

## Findings

### F-782-001 — [BLOCKING] AC-003 skip path is unreachable under bats errexit

| Field | Value |
|-------|-------|
| Severity | blocking |
| Category | coverage |
| File | `plugins/vsdd-factory/tests/s21-12-version-and-deny-gate.bats` (AC-003) |

**Finding.** The new guard does not do what its own comment claims:

```bash
metadata_json=$(cd "$REPO_ROOT" && cargo metadata --format-version 1 --locked 2>/dev/null)
metadata_exit=$?

if [ "$metadata_exit" -ne 0 ] || [ -z "$metadata_json" ]; then
  skip "cargo metadata --locked failed or produced no output ..."
fi
```

bats-core runs test bodies with `set -e`. A plain assignment from a failing command substitution trips errexit and aborts the test **at the assignment line** — `metadata_exit` is never read and the `skip` never fires. Verified empirically against the pinned Bats 1.13.0 with the identical construct:

```
not ok 1 errexit probe
# (in test file t.bats, line 3)
#   `out=$(bash -c 'echo boom >&2; exit 3' 2>/dev/null)' failed with status 3
```

The commit message's claim — *"skip cleanly with a clear reason when cargo metadata itself fails ... instead of hard-failing on an environment gap"* — does not hold for the `exit != 0` arm. Only the `-z` (exit 0, empty stdout) arm is live. This is a paper-fix under TD-VSDD-059: the comment asserts behavior the code does not implement.

**Suggestion.** The correct pattern already exists **in this same file**, in the adjacent AC-004 test (~L154–158):

```bash
set +e
output=$(cd "$REPO_ROOT" && cargo deny check advisories 2>&1)
exit_code=$?
set -e
```

Wrap the `cargo metadata` capture identically. Two lines; makes the skip branch real and the comment true.

The core stderr fix itself is correct and worth keeping — `2>&1 | jq` genuinely did corrupt the JSON stream, and routing stderr to `/dev/null` fixes it.

---

### F-782-002 — [SUGGESTION] PR body §1 under-describes commit 1

| Field | Value |
|-------|-------|
| Severity | suggestion |
| Category | description |

**Finding.** The PR body describes commit 1 as only the stderr/jq fix. The diff also converts an environment-gap **hard-fail into a skip** — a deliberate loosening of the gate. The commit message discloses this; the PR body does not. Checklist item 2 (description accuracy).

**Suggestion.** Add one sentence to the body so a future reader does not have to diff to find the behavior change.

---

### F-782-003 — [SUGGESTION] `!/\*\*SUPERSEDED/` is unanchored to the Status column

| Field | Value |
|-------|-------|
| Severity | suggestion |
| Category | coverage |
| File | `plugins/vsdd-factory/tests/sprint-state-format.bats` (ASSERT 1 + ASSERT 2) |

**Finding.** The filter matches `**SUPERSEDED` **anywhere in the row**, not just the Status cell (the paired `**retired**` filter at least carries both delimiters). Today exactly **one** STORY-INDEX row matches (S-21.11, L735), so there is no live over-match — but a future Notes cell reading `**SUPERSEDED BY S-21.19**` on an *active* story would silently drop it from PC4 completeness, and a silent completeness hole is the exact failure class this gate exists to catch.

Also inconsistent casing: `**retired**` lowercase vs `**SUPERSEDED` uppercase, with no normalization — a `**superseded**` status cell would not be excluded.

**Suggestion.** Anchor both exclusions to `$status_col` (ASSERT 2 already resolves it dynamically), or at minimum normalize case.

---

### F-782-004 — [NIT] The ASSERT 2 (PC2) exclusion is a no-op

| Field | Value |
|-------|-------|
| Severity | nit |
| Category | coherence |
| File | `plugins/vsdd-factory/tests/sprint-state-format.bats` (ASSERT 2) |

**Finding.** ASSERT 2 iterates `ss_ids` (sprint-state IDs), not `idx_ids`. Any superseded ID present in sprint-state already fails ASSERT 1 as a phantom ID; with this change it additionally fails ASSERT 2 as *"not found in STORY-INDEX (non-retired)"* rather than matching. It delivers neither the claimed *"not looked up for a matching status"* benefit nor any harm — it is pure symmetry with the retired handling.

**Suggestion.** Fine to keep for symmetry; the commit message overstates what it does.

---

## Verified clean (no rubber-stamp — what was actually checked)

| Check | Result |
|---|---|
| **Cargo.lock minimality** | Clean. `h2` version + checksum only; the crate's `dependencies` list is unchanged. 4 incidental `windows-sys 0.52.0 → 0.61.2` unifications, all consistent with tokio's existing `^0.61`. No unrelated churn. |
| **h2 advisory actually cleared** | `deny-advisories` CI job **already PASSED** on this PR head — independent confirmation of RUSTSEC-2026-0258 clearance, not just a claim. 0.4.19 ≥ 0.4.16 fix floor. |
| **CHANGELOG placement** | Correct — sibling bullet under `## [Unreleased]` → `### Security`, alongside the S-21.12 entry. Not misfiled into rc.23. |
| **sprint-state coupling** | Verified `S-21.11` appears in `sprint-state.yaml` **only in a comment** (L11), not as a `stories:` entry — the awk extractor will not pick it up, so the ASSERT 1 phantom-ID direction stays green. Companion `factory-artifacts` commits are genuinely required and genuinely already landed. |
| **Diff coherence** | All 4 files relate to rc.24 CI-green prep. No product source (`crates/`) touched. No `.factory/` files in the diff — correct ownership boundary for a non-state-manager branch. |
| **Diff size** | 42/-14 — far under the 500-line flag. |
| **Commit quality** | Conventional format, accurate scopes, no AI attribution. Commit bodies disclose the incidental `windows-sys` unification rather than hiding it. |
| **Test coverage** | No new tests needed; these *are* test-harness fixes plus a lockfile bump. Correct call. |
| **Demo evidence** | N/A justified — no user-observable behavior change. Agreed. |
| **Story dependencies** | N/A — no `STORY-NNN`, no upstream PR gating. Correct. |

## Merge readiness

`mergeStateStatus: UNSTABLE`. Still **pending**: `bats-full-suite (linux)`, `cargo-host` (ubuntu + macos), and all 5 `build-dispatcher` legs. `bats-full-suite` is precisely the job that exercises both changed `.bats` files — the blocking finding above lives in a branch that job will not reach on a healthy runner, so a green result there will **not** disconfirm it.

Fix the AC-003 errexit guard (F-782-001) and this is an easy approve. The security work is solid and independently verified.

## Posting note

`gh pr review --request-changes` was attempted and **rejected by the GitHub API**: `Review Can not request changes on your own pull request (addPullRequestReview)` — the PR author and the authenticated `gh` identity are the same user. This is a hard GitHub constraint, not a tooling choice. Findings were posted to the PR thread instead: https://github.com/drbothen/vsdd-factory/pull/782#issuecomment-5413884144
