# Fresh-Eyes PR Review — Cycle 2

**PR:** #803 `fix/count-propagation-cpu-runaway` → `develop`
**covered_sha:** `4f52863c991157838d269427557acb9f7ca419e5`
**Diff:** 2 files, +130 / -17
**Reviewer:** `vsdd-factory:pr-reviewer` (cycle 2, re-review after cycle-1 fixes)
**Verdict:** REQUEST_CHANGES

> **Posting note:** GitHub rejects `gh pr review --request-changes` on a self-owned PR
> (`GraphQL: Review Can not request changes on your own pull request`). This review is
> therefore recorded as a formal `gh pr review --comment` review record with the
> REQUEST_CHANGES verdict stated inline. `gh pr comment` was NOT used. The blocking
> findings below must be treated as merge-blocking despite the GitHub review state
> reading COMMENTED.

---

## Summary

The CPU-runaway fix is real and I verified it empirically: BC-INDEX.md now completes in **0.83s** where `origin/develop` hangs past 60s. That part of the PR is sound.

However **BLOCKING-1 from cycle 1 is not resolved.** I ran the PR-head hook against the live `.factory/` corpus under bash 5.3.9 and 3 of the 5 index files still exit 2. The PR body itself states the gate — *"All five live index files must exit 0 post-fix"* — and the PR does not meet it. Two new factual inaccuracies were also introduced into the very comment block that cycle 1's BLOCKING-2 was about, and the new process-substitution plumbing converts a loud failure into a silent always-pass.

---

## Findings

| # | Severity | Category | Finding |
|---|----------|----------|---------|
| BLOCKING-1 | blocking | missing | 3 of 5 live index files still exit 2; cycle-1 BLOCKING-1 unresolved |
| BLOCKING-2 | blocking | description | Code comment asserts a pre-fix failure mode that never existed; "correct totals" claim is false; PR body count is stale |
| BLOCKING-3 | blocking | coherence | Process substitution silently disables the lint on `awk`/`sed` failure |
| SUGGESTION-1 | suggestion | coverage | `pkill -P` does not reap `awk`/`sed` grandchildren |
| SUGGESTION-2 | suggestion | coverage | `fail` is undefined in this bats setup — diagnostics never render |
| SUGGESTION-3 | suggestion | coverage | Exit-status capture is unreachable on the non-zero path |
| SUGGESTION-4 | suggestion | coverage | Two new tests use different interpreters; guard checks only one |
| SUGGESTION-5 | suggestion | description | PR body Test Evidence is stale vs the shipped tests |
| SUGGESTION-6 | suggestion | demo | No captured before/after timing evidence |
| NIT-1 | nit | coherence | `extglob` removal verified safe (no action) |
| NIT-2 | nit | coherence | Frontmatter skip misses YAML sequence-item form |

---

### BLOCKING-1 — 3 of 5 live index files still exit 2 (cycle-1 BLOCKING-1 unresolved)

Measured with PR head `4f52863c`, `/opt/homebrew/bin/bash` 5.3.9, against the live corpus.

Method note: the hook reads its JSON payload from stdin and ignores `argv`, so invoking it with a bare path as `$1` does not exercise it (it blocks on stdin or no-ops). Real JSON was piped instead:

```bash
echo '{"tool_input":{"file_path":"<path>"}}' | bash validate-count-propagation.sh
```

| File | Post-fix | Pre-fix (`origin/develop`) | Detail |
|------|----------|----------------------------|--------|
| BC-INDEX.md | **exit 0** (0.83s) | HANG >60s | — |
| VP-INDEX.md | **exit 2** (0.50s) | HANG >15s | `'107 VPs' in VP-INDEX.md but '19 VPs' in STATE.md` |
| STORY-INDEX.md | **exit 2** (0.88s) | HANG >15s | `'15 BCs' in STORY-INDEX.md but '1993 BCs' in STATE.md` |
| ARCH-INDEX.md | **exit 2** (1.49s) | HANG >15s | `'106 VPs' vs '19 VPs'` **and** `'1973 BCs' vs '1993 BCs'` |
| STATE.md | **exit 0** (0.42s) | exit 0 | — |

Pre-fix, VP-INDEX / STORY-INDEX / ARCH-INDEX all hang because every source file's sibling list includes the 195KB BC-INDEX.md, so all three hit the runaway before emitting a verdict.

So this PR converts a silent hang into an agent-visible `block_intent=true exit_code=2` on **3 of the 4 canonical indexes** — precisely the files the Commit-D "4-index version bump" step of every fix burst must Edit. As written, merging this halts the fix-burst pipeline.

The new guard `[[ "$line" =~ ^[[:space:]]*(last_amended|change): ]]` does not reach any of the three residual causes. I traced each by replicating `_extract_counts` line-for-line:

1. **`.factory/STATE.md` L230** — `| **[D-945] VP-102..VP-120 pending allocation** | DEFERRED … | 19 VPs per BC-5.39.010 §VP Anchors. |`
   A legitimate *sub-range* count (VP-102..VP-120 = 19), in a body table. First-wins takes it over the real `106 VPs` at L267. Not frontmatter, not an H2 historical section.

2. **`.factory/stories/STORY-INDEX.md` L76 / L83** — `> Updated 2026-05-07: … 91 stories` and `> Updated 2026-05-06: … 15 BCs`
   Markdown **blockquote** changelog notes. The new guard matches only `last_amended:` / `change:` keys; `_is_historical_heading` matches only H2 sections. These stale May-2026 records are still extracted.

3. **`.factory/specs/architecture/ARCH-INDEX.md` L405 / L451** — `grand total 106 VPs` and `**Total BCs: 1,973 (per BC-INDEX v3.42; …)**` against live `total_vps: 107` / `total_bcs: 1993`.
   These are **true positives** — genuinely stale ARCH-INDEX prose.

That splits the remainder into two workstreams, and both need to land before or alongside this PR:

**(a) Hook logic.** Make the authoritative frontmatter keys win rather than relying on line order. Patterns C/D (`total_bcs:` / `total_vps:`) are the source of truth; Patterns A/B scrape narrative prose. Today they compete purely on which appears first in the file. Concretely:

```bash
# Emit authoritative frontmatter keys with a precedence marker, and let the
# first-wins loop prefer them over prose scrapes:
#   Pattern C/D -> "BCs:1993:0"   (rank 0, authoritative)
#   Pattern A/B -> "stories:91:1" (rank 1, prose)
# Then in the SOURCE_COUNTS loop, only overwrite when the incoming rank is lower.
```

And extend the historical guard to cover date-stamped blockquote records:

```bash
[[ "$line" =~ ^[[:space:]]*\>[[:space:]]*Updated[[:space:]][0-9]{4}-[0-9]{2}-[0-9]{2} ]] && continue
```

**(b) Corpus.** ARCH-INDEX L405/L451 are genuinely stale and need a state-manager fix (`106 VPs` → 107, `1,973 BCs` → 1,993). Per the routing table that is `vsdd-factory:state-manager`, not this PR — but the hook cannot be allowed to block every index edit in the interim.

---

### BLOCKING-2 — comment asserts a pre-fix failure mode that never existed

Cycle 1's "no semantic change" claim is correctly gone, and the "intentional improvement, not a no-op" framing is accurate. But three new inaccuracies landed in the same block:

**(i) `validate-count-propagation.sh` L147-148:**

```
#       b) Even for short lines, 2595 per-line subshell invocations of
#          `printf '%s' … | sed …` added ~20s of fork-overhead on macOS.
```

The pre-fix code contains **no such construct**. I grepped `origin/develop`'s version: the only `printf … | sed` in the entire file is at L207, in the drift-message join. The code this PR replaces is a pure-bash parameter expansion — `line="${line//+([A-Za-z])-+([0-9.])/}"` — which spawns zero subshells. The comment invents a second failure mode and attributes a 20s cost to it. This is the same defect class cycle 1 raised: a comment describing something the code never did. Delete clause (b), or replace it with the real secondary cost if one was measured.

**(ii) L129-133:** *"post-fix, it completes and uses the correct totals."* Measured, it does not: STORY-INDEX yields `stories:91` / `BCs:15` from a May-2026 blockquote, and STATE.md yields `VPs:19` from a sub-range. Soften to describe what the length filter actually guarantees.

**(iii) PR body:** *"VP-INDEX.md now correctly extracts 106 VPs (was 85 from frontmatter blob)."* Measured first-wins is **107**, from `total_vps: 107`. (The companion claim "BC-INDEX.md extracts 13 stories" I verified as correct.)

---

### BLOCKING-3 — process substitution silently disables the lint

`validate-count-propagation.sh` L189:

```bash
done < <(awk 'length <= 8192' "$path" | sed -E 's/[A-Za-z]+-[0-9.]+//g')
```

Neither `set -e` nor `pipefail` observes the exit status of a process substitution. If `awk` or `sed` fails for any reason — most plausibly an `awk` implementation with a record-length limit meeting the 198KB line, which is *exactly* the input class this PR targets — the substitution yields zero lines, `SOURCE_COUNTS` ends up empty, and the hook takes the `exit 0` path at L209. The lint silently reports "no drift" forever.

Pre-fix, `done < "$path"` failed loudly under `set -e`. This is the "silent return where partial-failure data should propagate" anti-pattern named in CLAUDE.md. Suggested fix:

```bash
local _tmp; _tmp="$(mktemp)"
if ! awk 'length <= 8192' "$path" | sed -E 's/[A-Za-z]+-[0-9.]+//g' > "$_tmp"; then
  echo "validate-count-propagation: preprocessing failed for $path" >&2
  rm -f "$_tmp"; return 1
fi
while IFS= read -r line; do … done < "$_tmp"
rm -f "$_tmp"
```

---

### SUGGESTION-1 — `pkill -P` misses grandchildren

`hooks.bats`: `pkill -P "$pid"` reaps only direct children of the background subshell. The hook now spawns `awk` and `sed` per `_extract_counts` call, which are *grandchildren* — on a timeout those survive, which is the leak cycle 1 asked to close. Under unfixed code the spin is in bash itself so the current form does reach it, but the guarantee is incomplete. Use a process group: `set -m` before the `&`, then `kill -- -"$pid"`.

### SUGGESTION-2 — `fail` is undefined; diagnostics never render

`hooks.bats` loads no bats-support (no `load` / `bats_load_library` anywhere in the file) and bats-core 1.13.0 ships no `fail`. Verified directly:

```
# /tmp/failtest/t.bats: line 3: fail: command not found
```

All three `fail "…"` call sites exit 127, so the tests still fail correctly — no false negative — but the messages cycle 1's fixes were written to emit never appear. Replace with `{ echo "msg" >&2; return 1; }`, or load bats-support.

### SUGGESTION-3 — exit-status capture unreachable on the non-zero path

```bash
{ echo '…' | "$HOOKS/validate-count-propagation.sh"
  echo $? > "$BATS_TEST_TMPDIR/exit_status.txt"; } &
```

The background subshell inherits bats' errexit. If the hook exits non-zero the subshell aborts at the pipeline and `exit_status.txt` is never written — so the assertion falls through to `fail "hook exited non-zero: "` with an empty value, losing the actual status. Add `|| echo $? > "$…"`, or `set +e` inside the group.

### SUGGESTION-4 — inconsistent interpreter between the two new tests

Test 1 invokes the hook directly (relying on its `#!/bin/bash` shebang); Test 2 invokes it via `bash -c` (PATH bash). `require_bash4_hook_interp` gates on `/bin/bash`, so Test 2's skip guard does not describe the interpreter Test 2 actually uses. Pick one form.

### SUGGESTION-5 — PR body Test Evidence is stale

The body says "via `timeout 5`", "asserts the hook exits within 5s", "208KB", "<0.2s". The shipped test uses a fork+poll loop with a **3s** budget, awk-based generation, ~200KB. Also worth stating plainly: both new tests **skip** on macOS, so the green `bats-darwin-leg` run counts them as 2 skips, and `bats-full-suite (linux)` was still *pending* at review time — the "59/59 PASS" badge has not yet been demonstrated on any leg where these two tests actually execute.

### SUGGESTION-6 — no captured timing evidence

`docs/demo-evidence/` has nothing for this change. This is a PostToolUse bash hook with no UI surface, so per-AC recordings are not meaningful — but the PR's entire value proposition is a timing delta, and VHS is available. A short capture of the before/after `time` on BC-INDEX.md would make the central claim reviewable rather than asserted.

### NIT-1 — extglob removal verified safe (no action needed)

I grepped for surviving `+(` / `?(` / `*(` / `@(` / `!(` patterns: none in the hook (L49's `SS-[0-9][0-9]-*.md` is a plain glob, valid without extglob), and `lib/block.sh` uses none. Dropping `shopt -s extglob` is clean.

### NIT-2 — frontmatter skip misses YAML sequence-item form

`^[[:space:]]*(last_amended|change):` matches the live `    change: "…"` form in ARCH-INDEX, but not the sequence-item variant `  - change: "…"`. Allow an optional `-[[:space:]]*` if that form can occur.

---

## Checklist coverage (8-item)

| # | Item | Result |
|---|------|--------|
| 1 | Diff coherence | PASS — both files relate to the single stated fix; no unrelated changes |
| 2 | Description accuracy | **FAIL** — BLOCKING-2(iii), SUGGESTION-5 |
| 3 | Test coverage | PARTIAL — 2 new tests cover the changed lines, but both skip on macOS and the Linux leg was pending; SUGGESTION-2/3/4 |
| 4 | Demo evidence | N/A for a stdin/stderr bash hook; SUGGESTION-6 recommends a timing capture |
| 5 | Commit quality | PASS — conventional format, detailed bodies, no `Co-Authored-By` / AI attribution |
| 6 | Diff size | PASS — 130/17 across 2 files, well under the 500-line flag |
| 7 | Missing changes | **FAIL** — BLOCKING-1: the PR body's own five-file exit-0 gate is unmet |
| 8 | Dependency status | PASS — standalone hotfix, no upstream story deps |

---

## What I verified (beyond the findings)

Recording this so the findings are not a rubber stamp in either direction:

- **CPU runaway genuinely eliminated** — measured, not taken from the PR body: BC-INDEX.md 0.83s on PR head vs >60s hang on `origin/develop`.
- **`sed -E 's/[A-Za-z]+-[0-9.]+//g'` semantics traced by hand** — does not eat `v1.0.0-rc.24` (char before `-` is a digit), `2026-08-31`, `BC-INDEX.md` (no digits after `-`), or `total_bcs: 42`. Confirmed `42 BCs` survives the strip, which is what Test 2 exists to protect.
- **Heading detection unaffected by moving the strip ahead of `_is_historical_heading`** — all five patterns (`## changelog`, `## change log`, `## historical content`, `## phase progress`, `## decisions log`) are ID-free, so the comment's claim at L151-153 holds.
- **Test isolation is safe** — `setup()` does `mktemp -d` + `cd`, so the tests' writes to `.factory/BC-INDEX.md` and `.factory/STATE.md` cannot touch the real corpus. Worth calling out explicitly since those paths look alarming in the diff.
- **`deny-advisories` CI failure is pre-existing on `develop`** (develop head `9ab5a6f6` also failed) — **not** attributable to this PR, and not held against it.
- **Pre-existing, not introduced:** the hook's `#!/bin/bash` shebang combined with `declare -A` (L193) means it cannot run at all under macOS `/bin/bash` 3.2. Unchanged by this PR and correctly acknowledged by `require_bash4_hook_interp`, but worth a follow-up given the hook ships to macOS operators.

---

## Path to approval

1. Resolve BLOCKING-1 so all five live index files exit 0 — hook-side precedence for authoritative frontmatter keys plus a blockquote-record guard, coordinated with a state-manager fix for the genuinely stale ARCH-INDEX totals. Re-run the five-file matrix and paste the output.
2. Correct the three inaccuracies in BLOCKING-2 (drop the invented `printf | sed` clause, soften "correct totals", fix 106 → 107).
3. Close BLOCKING-3 so a preprocessing failure cannot silently pass the lint.
4. Suggestions 1-6 at your discretion, though 2 and 3 are cheap and restore the diagnostics cycle 1 asked for.
