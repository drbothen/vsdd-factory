# Fresh-Eyes PR Review — Cycle 3

**PR:** #803 `fix/count-propagation-cpu-runaway` → `develop`
**covered_sha:** `3d988d829bb94c7c4193d3f09655737078162bb8`
**Diff:** 2 files, +190 / -37
**Reviewer:** `vsdd-factory:pr-reviewer` (cycle 3, re-review after cycle-2 fixes)
**Verdict:** REQUEST_CHANGES

> **Posting note:** GitHub rejects `gh pr review --request-changes` on a self-owned PR
> (`GraphQL: Review Can not request changes on your own pull request` — exit 1, captured
> verbatim from `github-ops`). This review is therefore recorded as a formal
> `gh pr review --comment` review record with the REQUEST_CHANGES verdict stated inline
> (posted 2026-08-31T20:29:24Z, 16108 bytes). `gh pr comment` was NOT used. The blocking
> findings below must be treated as merge-blocking despite the GitHub review state
> reading COMMENTED.

---

## Summary

Every claim below was verified by execution against the live corpus, not by reading. Method and captured output are inline.

The CPU-runaway fix itself remains sound, and cycle 2 genuinely closed BLOCKING-2, SUGGESTION-2, and SUGGESTION-3. The STORY-INDEX false positive that cycle 2 targeted is also genuinely fixed.

However **BLOCKING-3 is not fixed** — the temp file was placed inside `_extract_counts`, but the function is still invoked through process substitution, so its `return 1` is discarded and the hook still exits 0 ("no drift") when preprocessing fails. And of the three guards added for BLOCKING-1, only one changes any verdict on the live corpus; the other two ship comments asserting fixes they demonstrably do not deliver, which is the third consecutive cycle of the same comment-accuracy defect class that BLOCKING-2 was about.

The cycle-2 gate — *"all five live index files must exit 0"* — is still not met, but for a legitimate reason this time: ARCH-INDEX exit 2 is a verified TRUE POSITIVE and correctly does not block.

---

## Findings

| # | Severity | Category | Finding |
|---|----------|----------|---------|
| BLOCKING-A | blocking | coherence / silent failure | BLOCKING-3 not fixed: preprocessing failure still yields exit 0. `return 1` inside process substitution is discarded. |
| BLOCKING-B | blocking | description | 2 of 3 cycle-2 guards change no verdict; 3 comment/commit claims assert behavior that provably does not occur. |
| SUGGESTION-C | suggestion | coherence | `mktemp` has no `trap` cleanup — leaks on kill, this PR's own failure mode. |
| SUGGESTION-D | suggestion | size / efficiency | Each sibling re-parsed once **per keyword** (3 full awk+sed+mktemp passes over STATE.md per ARCH-INDEX run). |
| SUGGESTION-E | suggestion | coverage | All three cycle-2 guards and the new error path have zero test coverage. |
| OBSERVATION-F | nit | missing (pre-existing) | Real `.factory/STATE.md` resolves to zero siblings → hook is a vacuous no-op there; bats fixtures use a non-representative flat layout. |

---

## BLOCKING-A — preprocessing failure still silently disables the lint (fail-open)

| Field | Value |
|-------|-------|
| Severity | blocking |
| Category | coherence / silent failure |
| Location | `validate-count-propagation.sh` — `_extract_counts` gate; callsites at the `SOURCE_COUNTS` loop and the sibling loop |

BLOCKING-3 was: *"process substitution silently disables the lint → false-pass exit 0."* Cycle 2 added a temp file **inside** `_extract_counts`:

```bash
awk 'length <= 8192' "$path" | sed -E 's/[A-Za-z]+-[0-9.]+//g' > "$_preproc_tmp" || {
  echo "validate-count-propagation: preprocessing pipeline failed for $path" >&2
  rm -f "$_preproc_tmp"
  return 1
}
```

The `||` gate does fire correctly (pipefail is in effect, so an `awk` failure is caught). But `_extract_counts` is **still invoked through process substitution** at both callsites:

```bash
done < <(_extract_counts "$FILE_PATH")   # SOURCE_COUNTS loop
done < <(_extract_counts "$sibling")     # sibling loop
```

A `return 1` inside `<( … )` executes in a subshell. The parent `while … done < <(…)` never reads that status — it receives zero lines, `SOURCE_COUNTS` stays empty, and the hook takes the `exit 0` path at the `${SOURCE_COUNTS[*]:-}` guard. **Control flow is unchanged from cycle 2.** The only delta is one extra stderr line; `awk`'s own stderr was already visible pre-fix, so even the "audible" improvement is marginal.

**Proof** — a fixture with genuine drift, run twice on identical input, only `sed` swapped for a failing stub:

```
# fixture: .factory/STATE.md contains "42 BCs"; .factory/BC-INDEX.md has total_bcs: 38
$ echo '{"tool_input":{"file_path":".factory/STATE.md"}}' | ./validate-count-propagation.sh
status=2
BLOCKED by validate-count-propagation: COUNT DRIFT DETECTED: '42 BCs' in STATE.md but '38 BCs' in BC-INDEX.md.
  -> correct

# same fixture, failing sed on PATH
$ PATH=/tmp/shim2:$PATH ./validate-count-propagation.sh < same-input
validate-count-propagation: preprocessing pipeline failed for .factory/STATE.md
EXIT = 0        <-- real, detectable drift reported as clean
```

Same result with a failing `awk` against the real `VP-INDEX.md`: stderr message printed, `EXIT=0`.

The cycle-2 commit message states: *"With the temp file approach, a non-zero pipeline exit is detected and the function returns 1."* True of the function; false of the hook, because nothing observes the return value. **Paper-fix under TD-VSDD-059** — the claimed closure has no load-bearing effect on the verdict.

**Suggestion:** hoist the temp file to the **caller** so the function's exit status becomes observable, and decide the failure posture explicitly.

```bash
_src_tmp="$(mktemp)"
trap 'rm -f "$_src_tmp"' EXIT
if ! _extract_counts "$FILE_PATH" > "$_src_tmp"; then
  echo "validate-count-propagation: count extraction failed for $FILE_PATH" >&2
  exit 2   # fail-closed: an unparseable corpus is not "no drift"
fi
while IFS=: read -r kw cnt rnk; do
  ...
done < "$_src_tmp"
```

Apply the same shape to the sibling loop. Either fail-closed (`exit 2`) or loud-advisory is defensible for a PostToolUse lint — but `exit 0` meaning "no drift" when extraction never ran is not. Pair with the regression test in SUGGESTION-E.

---

## BLOCKING-B — non-load-bearing guards with factually false justifications

| Field | Value |
|-------|-------|
| Severity | blocking |
| Category | description (comment / commit-message accuracy) |
| Location | `_extract_counts` rank-precedence header comment; `_is_historical_heading` `## drift items` arm; blockquote-guard comment; cycle-2 commit message |

Verdict of every variant against the five live index files in the **real repo layout** (`exit=2` means the hook blocks):

| variant | STATE | ARCH | BC | VP | STORY |
|---|---|---|---|---|---|
| cycle 1 (`4f52863c`) | 0 | 2 | 0 | 0 | **2** |
| **cycle 2 (`3d988d82`, this head)** | 0 | 2 | 0 | 0 | **0** |
| cycle 2 − blockquote guard | 0 | 2 | 0 | 0 | **2** |
| cycle 2 − `## drift items` guard | 0 | 2 | 0 | 0 | 0 |
| cycle 2 − rank precedence | 0 | 2 | 0 | 0 | 0 |

Reading across: **the blockquote guard is the entire fix.** It alone flips STORY-INDEX 2 → 0. That guard is load-bearing and correct.

The other two change nothing, and the comments justifying them describe events that do not occur.

**B1 — `## drift items` guard.** Commit message: *"Fixes VP-INDEX.md false positive (19 ≠ 107)."* VP-INDEX already exited 0 at cycle 1. Removing the guard from this head still yields VP-INDEX exit 0, because in file order STATE.md emits `VPs:107` first and the `19 VPs` entry is third — first-wins had already discarded it:

```
$ _extract_counts .factory/STATE.md | grep '^VPs'   # with the guard removed
VPs:107:1
VPs:107:1
VPs:19:1
VPs:107:1
```

**B2 — rank precedence.** The header comment and the commit message both say: *"STATE.md has `total_vps: 107` and also `19 VPs per §VP Anchors` in prose — rank 0 wins"* / *"Ensures `total_vps: 107` beats `19 VPs per §VP Anchors` when STATE.md is the source file."* Both premises are false:

```
$ grep -nE '^(total_vps|total_bcs):' .factory/STATE.md
(no output — STATE.md has no total_vps:/total_bcs: keys at all)
```

and STATE.md resolves to zero siblings (OBSERVATION-F), so STATE.md is never the source of a comparison in the real layout. Where rank-0 *does* fire — VP-INDEX and BC-INDEX — the rank-0 entry is already first in file order, so first-wins gives the identical answer:

```
$ _extract_counts .factory/specs/verification-properties/VP-INDEX.md | grep '^VPs'
VPs:107:0     <-- already first
VPs:17:1
VPs:10:1
...
$ _extract_counts .factory/specs/behavioral-contracts/BC-INDEX.md | grep '^BCs'
BCs:1993:0    <-- already first
BCs:125:1
...
```

**B3 — blockquote comment over-generalizes.** It asserts blockquotes are *"exclusively historical or documentary records … None carry an authoritative live count."* `.factory/STATE.md:270` is the live Session Resume Checkpoint banner, refreshed every Commit E per D-446(c), carrying current values:

```
> **SELF-SUFFICIENT RESUME CONTEXT.** … BC-INDEX v5.34 (1,993 BCs). VP-INDEX v2.91 (107 VPs). STORY-INDEX v4.418 (175 stories; 25 epics). … merged_count 115.
```

Those are today's real totals, not a frozen record. Blanket-skipping `>` lines remains the right call (the same counts appear on non-blockquote lines 122/159/333, so no detection is lost today), but the stated reason is wrong and the guard is quietly broader than the comment admits.

**Suggestion:** keep both mechanisms — they are defensible robustness hardening — but correct the claims. Specifically:

- Rank precedence: describe as forward-looking hardening ("prefers frontmatter over prose should a rank-1 entry ever precede the frontmatter key; no current corpus file exercises this"), and drop the STATE.md `total_vps` example — that key does not exist.
- `## drift items`: describe as defence-in-depth consistent with the other historical headings; drop the "fixes VP-INDEX false positive" claim.
- Blockquote guard: state that blockquotes are skipped wholesale because the project uses them for historical *and* banner-cite content, and banner cites are redundant with the non-blockquote lines — rather than asserting no blockquote carries a live count.

Comments-and-body edit, not a redesign. Held at blocking severity because cycle-2's BLOCKING-2 was exactly this defect class, and shipping two fresh instances of it inside the fix for it is the pattern the Canonical Principle production-grade default exists to stop.

---

## SUGGESTION-C — no `trap`; temp file leaks on kill

| Field | Value |
|-------|-------|
| Severity | suggestion |
| Category | coherence |

`_preproc_tmp="$(mktemp)"` is removed only on the happy path and the error path inside the function. There is no `trap`. If the hook is killed mid-run — the exact scenario this PR exists to address, an orphaned PostToolUse process being reaped — or if `set -e` fires anywhere inside the `while` loop, the temp file survives in `/tmp` indefinitely.

**Suggestion:** `trap 'rm -f "$_preproc_tmp"' RETURN` inside the function, or fold into the caller-side `EXIT` trap from BLOCKING-A's patch.

## SUGGESTION-D — sibling files re-parsed once per keyword

| Field | Value |
|-------|-------|
| Severity | suggestion |
| Category | size / efficiency |

Sibling extraction sits inside the `for keyword` loop, so each sibling is fully re-preprocessed for every keyword. Instrumented invocation counts:

```
source=ARCH-INDEX.md   _extract_counts invocations=4   (STATE.md parsed 3x)
source=BC-INDEX.md     _extract_counts invocations=3   (STATE.md parsed 2x)
source=VP-INDEX.md     _extract_counts invocations=2   (STATE.md parsed 1x)
```

Three separate `mktemp` + `awk` + `sed` passes over STATE.md for one ARCH-INDEX write. The structure predates this PR, but this PR adds a `mktemp` per pass, so the cost grew. Measured runtimes are still fine (0.26–0.48s), so not urgent.

**Suggestion:** hoist sibling extraction above the keyword loop into a single `declare -A SIB_COUNTS` keyed by `sibling:keyword` — one pass per sibling. Nearly free alongside BLOCKING-A's refactor, which touches the same loops.

## SUGGESTION-E — cycle-2 guards and error path are untested

| Field | Value |
|-------|-------|
| Severity | suggestion |
| Category | coverage |

No test in `hooks.bats` exercises rank precedence, the blockquote guard, the `last_amended:`/`change:` frontmatter skip, the `## drift items` heading, or preprocessing failure:

```
$ git show 3d988d82:plugins/vsdd-factory/tests/hooks.bats | grep -iE 'rank|blockquote|total_vps|preprocessing|last_amended'
507:# Root cause: BC-INDEX.md contains a single ~195KB "last_amended:" blob line;   (comment only)
```

Every cycle-2 behavior change is protected only by the live corpus — which is exactly how the STORY-INDEX false positive survived cycle 1.

**Suggestion:** add at minimum —
1. Blockquote guard: a source file whose only BC count is inside a `> …` line → assert exit 0; the same count outside the blockquote → assert exit 2. Locks in the one guard that works.
2. Preprocessing failure (pairs with BLOCKING-A): stub `awk` or `sed` to exit 1 on `PATH`, use a fixture with genuine drift, assert the hook does **not** exit 0.

## OBSERVATION-F — real STATE.md has zero siblings (pre-existing)

| Field | Value |
|-------|-------|
| Severity | nit (pre-existing; route as follow-up) |
| Category | missing |

`CORPUS_ROOT` walks up from the file and stops at the first directory containing `STATE.md`. For `<repo>/.factory/STATE.md` that resolves to `<repo>/.factory`, so candidates become `<repo>/.factory/.factory/specs/…` (nonexistent) and `<repo>/.factory/ARCH-INDEX.md` (nonexistent — the real indexes live under `.factory/specs/…`):

```
CORPUS_ROOT=/Users/zious/Documents/GITHUB/vsdd-factory/.factory
(sibling list is empty)
```

`STATE.md` therefore exits 0 at the `${#SIBLING_FILES[@]} -eq 0` early return, having compared nothing. The other four indexes get exactly one sibling — `.factory/STATE.md` — and never see each other.

Not introduced by this PR (the walk-up is untouched) and not blocking. Two consequences worth recording:

- The PR body cites "STATE.md exit=0" as post-fix verification. That exit 0 is vacuous and must not be counted as evidence.
- The bats fixtures use a flat `.factory/*.md` layout under which siblings *do* resolve. That is why the suite passes while the real layout silently compares nothing — the tests are non-representative of production topology.

**Routing:** the `CORPUS_ROOT` resolution defect should go to a follow-up (orchestrator's call on specialist), since fixing it will surface a fresh batch of real cross-index drift and should not be bundled into a CPU-runaway hotfix.

---

## What I verified as good

- **BLOCKING-2 comment fix — correct.** The invented `printf | sed` / "2595 per-line subshell invocations" clause is gone. The remaining description of the pre-fix extglob form (`O(n·k)` bash global-replace, per-line, ~8000 matches on a 195KB line) is accurate, and the "correct totals" over-claim is replaced with the properly hedged "removes oversized lines from extraction scope so stale counts no longer shadow authoritative values."
- **Blockquote guard — load-bearing and correct.** Sole fix for the STORY-INDEX false positive (matrix in BLOCKING-B). `[[ "$line" =~ ^[[:space:]]*'>' ]]` correctly treats the quoted `>` as a literal.
- **The CPU-runaway fix itself — sound.** `awk 'length <= 8192' | sed -E` is genuinely linear. Live timings: BC-INDEX 0.48s, VP-INDEX 0.26s, STORY-INDEX 0.40s, ARCH-INDEX 0.45s. The 208KB synthetic fixture completes in ~0.3s.
- **Both new bats tests pass under bash 4.** They skip on the macOS host (`/bin/bash` 3.2 — all 12 count-propagation tests skip; suite reports 59/59 ok with 12 skips), so both bodies were re-run manually against a bash-4 interpreter: `TEST 1: PASS` (208001B fixture, ~0.3s, exit 0), `TEST 2: PASS` (status 2, output contains `42 BCs`).
- **SUGGESTION-2 — fixed.** Bare `fail` replaced with `{ echo … >&2; return 1; }`; `bats-support` is indeed not loaded in this file.
- **SUGGESTION-3 — fixed.** Exit status captured in the background subshell via `&& echo 0 > f || echo $? > f` and asserted after the poll loop. `perl` correctly replaced with `awk`.
- **ARCH-INDEX exit 2 is a TRUE POSITIVE and does not block this PR.** Reproduced: `'106 VPs' in ARCH-INDEX.md but '107 VPs' in STATE.md` and `'1973 BCs' in ARCH-INDEX.md but '1993 BCs' in STATE.md`. Authoritative sources confirm 107 (`total_vps: 107` in VP-INDEX) and 1993 (`total_bcs: 1993` in BC-INDEX). The hook is doing its job; the stale prose at ARCH-INDEX L405/L451 is the defect. A lint correctly reporting genuine drift is not grounds to withhold approval, and the state-manager routing note in the PR body is the right disposition — required follow-up, not a blocker.

---

## CI

| Check | Status | Assessment |
|-------|--------|------------|
| `deny-advisories` | **fail** | RUSTSEC-2026-0268 / RUSTSEC-2026-0269 on wasmtime 46.0.2. Pre-existing on `develop`, not introduced here. Not held against this PR; needs its own hotfix. |
| `bats-full-suite (linux)` | **pending** at review time | The only leg that actually executes the count-propagation tests — `bats-darwin-leg` passes but skips all 12. The "tests 59/59" badge is accurate only in that skips count as ok. Manual bash-4 run of both new tests passed, so this leg is expected green; confirm before merge. |
| `SAST (Semgrep)`, `validate`, `platforms-drift`, `policy-15-attestation-location`, `attestation-gate-non-vacuity-controls`, `bats-darwin-leg`, `bats-wave-handoff`, `build-dispatcher (linux-arm64)` | pass | — |

---

## Disposition

**REQUEST_CHANGES.**

BLOCKING-A and BLOCKING-B must be addressed. A is a substantive correctness fix (roughly the patch sketched above, plus one test); B is a comment / PR-body accuracy edit. SUGGESTION-C through E are non-blocking, but C and D are cheap to fold into A's refactor. OBSERVATION-F should be routed as a follow-up, not fixed here.

Re-request review once A and B are pushed and the Linux bats leg is green.
