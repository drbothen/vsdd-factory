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

---

# Fresh-Eyes PR Review — Cycle 4

**PR:** #803 `fix/count-propagation-cpu-runaway` → `develop`
**covered_sha:** `c1bd0d54c9be261890b117ffc056a8ebb419a585`
**Reviewed:** 2026-08-31
**Diff scope:** 2 files, +246 / -39 (`plugins/vsdd-factory/hooks/validate-count-propagation.sh`, `plugins/vsdd-factory/tests/hooks.bats`)
**Verdict: REQUEST_CHANGES**

## Prior-cycle closure verification

### BLOCKING-A (error propagation / fail-closed) — CLOSED

Verified empirically, not by reading the claim.

- No live process substitution remains. `grep -n '< <('` returns exactly one hit, line 167, inside a comment. No live `shopt -s extglob` and no live `+([...])` pattern; the only hit (line 160) is the explanatory contrast comment.
- Both callsites now gate on exit status: `if ! _extract_counts "$FILE_PATH" > "$_src_tmp"` (line 246) and `if ! _extract_counts "$sibling" > "$_sib_tmp"` (line 284), each followed by `exit 2`.
- Fault injection, broken `sed` shim at head of PATH → **exit 2** with both messages on stderr:
  `validate-count-propagation: preprocessing pipeline failed for .factory/STATE.md`
  `validate-count-propagation: count extraction failed for .factory/STATE.md`
- Fault injection, broken `awk` shim → **exit 2**, same messages. `pipefail` correctly propagates the first-stage failure.
- Control run without shims → exit 2 via the drift path with the drift message, confirming the two exit-2 paths are distinguishable in output.

### SUGGESTION-C (temp-file cleanup) — CLOSED, and the RETURN trap is safe

I specifically stress-tested the concern that `trap 'rm -f "$_preproc_tmp"' RETURN` (line 179) could leak past `_extract_counts` and fire on the nested `_is_historical_heading` return — which, with `_preproc_tmp` out of scope under `set -u`, would abort the shell. It does not: without `functrace`, the RETURN trap fires exactly once, for the function that installed it. Confirmed on both `/bin/bash` 3.2.57 and bash 5.3.9. No finding.

### BLOCKING-B (comment accuracy) — 2 of 3 clauses CLOSED, 1 still false

| Clause | Claim | Verified? |
|---|---|---|
| `## drift items` guard | "defence-in-depth … does not change the verdict for any file in the current corpus — all live authoritative counts appear outside drift-items sections today" | **ACCURATE.** `STATE.md:224` does carry `## Drift Items / Tech Debt`, but every winning count in STATE.md (`BCs:1993`, `VPs:107`) is extracted before line 224, so the added guard cannot change any current verdict. |
| Rank precedence | "forward-looking hardening; no live corpus file exercises this today … total_vps: / total_bcs: … appear before any prose count mentions in file order … STATE.md has no total_vps: or total_bcs: frontmatter keys at all" | **ACCURATE.** Ordered extraction confirms `BCs:1993:0` is entry #1 in BC-INDEX.md and `VPs:107:0` is entry #1 in VP-INDEX.md, so first-wins would select the same values; all 5 STATE.md entries are rank 1. |
| Blockquote skip | "live banner-cite counts are redundant with the same values on non-blockquote lines in the same file, so **no detection is lost** by excluding blockquote lines" | **FALSE.** See BLOCKING-C. |

### CPU-runaway fix — CONFIRMED

Live corpus, this branch's hook:

| File | Exit | Elapsed |
|---|---|---|
| VP-INDEX.md | 0 | 379 ms |
| BC-INDEX.md | 0 | 585 ms |
| STORY-INDEX.md | 0 | 463 ms |
| STATE.md | 0 | 67 ms (but see SUGGESTION-4 — vacuous) |
| ARCH-INDEX.md | 2 | 685 ms — true positive (`106 VPs` vs `107 VPs`; `1973 BCs` vs `1993 BCs`) |

`bats-full-suite (linux)` completed **success**, so all 12 count-propagation tests — including the 3 new ones — genuinely execute and pass on bash 5. (All 12 SKIP on macOS `/bin/bash` 3.2 via `require_bash4_hook_interp`, so the green `bats-darwin-leg` job is not evidence for them.)

---

## Cycle 4 findings

### BLOCKING-C — blockquote-skip comment asserts "no detection is lost"; falsified for two keywords

| Field | Value |
|---|---|
| Severity | **blocking** |
| Category | coherence / description accuracy |
| Location | `plugins/vsdd-factory/hooks/validate-count-propagation.sh` lines 193–202 (comment) and line 202 (`[[ "$line" =~ ^[[:space:]]*'>' ]] && continue`) |

The comment justifying the unconditional blockquote skip states that live banner-cite counts "are redundant with the same values on non-blockquote lines in the same file, so no detection is lost by excluding blockquote lines." That universal conclusion is false, and it is load-bearing — it is the entire stated reason for skipping *all* `>`-prefixed lines rather than classifying blockquote intent.

Falsifying evidence (differential extraction with the blockquote skip toggled on/off):

- **`subsystems` in BC-INDEX.md.** With blockquotes kept, BC-INDEX.md yields `subsystems:10`. With the skip applied, it yields **no `subsystems` entry at all**. The sole carrier is line 524, a blockquote:
  `> Master index of all 1,967 behavioral contracts across 10 subsystems.`
  A non-blockquote `subsystems` count-match count in that file is **0**. There is no redundant non-blockquote line, so detection for this keyword is not "not lost" — it is entirely removed.
- **`BCs` in STORY-INDEX.md.** With blockquotes kept: `BCs:22`, `BCs:15`, `BCs:03`. With the skip: none. Again blockquote-only.

`subsystems` is one of the five keywords the hook exists to track, and it appears **only** in Pattern A's alternation (Pattern B's alternation is `(BCs|VPs|stories|capabilities)` — see SUGGESTION-5), so blockquote prose is its natural and, in BC-INDEX.md, only carrier. The net effect of this PR is that `subsystems` drift detection is silently dead.

This is the same defect class as BLOCKING-B, which cycle 3 was chartered to close, and it is what CLAUDE.md's Self-Audit Checklist targets: "Doc comment claiming 'this requires capability X' with no capability check → either implement the gate or remove the docs."

Required fix — pick one, both are in-scope:

1. **Preferred (preserves coverage):** narrow the skip so it only drops blockquote lines that are genuinely historical, or exempt the keywords that have no non-blockquote carrier. Then the comment becomes true as written.
2. **Acceptable (accepts the loss, documents it honestly):** keep the unconditional skip and replace the false clause with the measured truth, e.g.:

```
#   Coverage delta (measured, not assumed): for STATE.md the live banner counts
#   ARE redundant — BCs:1993 and VPs:107 both still extract from non-blockquote
#   lines.  They are NOT redundant for two keywords:
#     - BC-INDEX.md `subsystems` — sole carrier is the blockquote master-index
#       banner (`> ... across 10 subsystems.`); after this skip BC-INDEX.md
#       contributes zero `subsystems` entries, so `subsystems` drift is no
#       longer detected anywhere in the corpus.
#     - STORY-INDEX.md `BCs` — blockquote-only per-story column values; losing
#       these is desirable (they are not totals and were a false-positive source).
#   The `subsystems` loss is an accepted, tracked narrowing of the hook's
#   contract, not a no-op.
```

If option 2 is taken, the `subsystems` coverage loss must also be surfaced in the PR body's "Semantic delta" paragraph, which currently claims only an improvement.

### BLOCKING-D — `cargo-host` red on both platforms from an S-25.01 corpus inconsistency

| Field | Value |
|---|---|
| Severity | **blocking** |
| Category | dependency / CI gate |
| Jobs | `cargo-host (ubuntu-latest)` **fail**, `cargo-host (macos-latest)` **fail** |

Failing step: `cargo test (workspace, all targets)`, exit 101. Single failure out of 231:

```
test tests::test_BC_5_39_010_corpus_arm_b2_live_story_index_no_violations ... FAILED
panicked at crates/hook-plugins/validate-cross-site-correspondence/src/lib.rs:1543:9:
  validate-cross-site-correspondence [Class B] POLICY 18: STORY-INDEX.md internal
  parity violation for story S-25.01 — catalog=8bf7fa8 blockquote=1f9fcd2 —
  run `compute-input-hash --update`
```

This is **not** an unrelated flake and not pre-existing: `cargo-host` on both platforms is **green** at develop HEAD `9ab5a6f6`, and this branch's merge-base *is* `9ab5a6f6` (0 commits behind). The violating story is **S-25.01 — this PR's own story**. Its STORY-INDEX.md catalog hash and blockquote hash diverge, i.e. the `.factory/` side of this workstream landed a half-updated row.

The remedy is named in the panic itself and is mechanical, so per Canonical Principle Rule 4 it should be fixed in-scope rather than deferred: reconcile catalog↔blockquote for S-25.01 in STORY-INDEX.md and run `compute-input-hash --update`, in a single state-manager burst (route to `vsdd-factory:state-manager`; the fix lands on `factory-artifacts`, not in this PR's diff, but this PR's required checks gate on it).

### BLOCKING-E — `deny-advisories` red on two new wasmtime RUSTSEC advisories (external; merge gate only)

| Field | Value |
|---|---|
| Severity | **blocking (merge gate)** — not attributable to this diff |
| Category | dependency |

```
error[vulnerability]: Guest controlled-size host heap allocation through WASIp3 streams
  ID: RUSTSEC-2026-0268
error[vulnerability]: Filesystem sandbox escape when paths or symlinks contain trailing slashes
  ID: RUSTSEC-2026-0269
```

Both are wasmtime advisories. This PR touches only a bash hook and a bats file and cannot have caused them; develop's last green `deny-advisories` run was 2026-08-30 and these advisories landed after it, so develop is latently red too. I am recording this as blocking because it is a red required check on #803, but the correct routing is a **separate wasmtime bump PR** (same shape as #781, which cleared five advisories via 44→46.0.2). Do not fold a dependency bump into this hotfix. Once that lands and this branch is rebased, the check clears without any change to #803's diff.

Also note the same job log contains `error: override toolchain '1.95.0-x86_64-unknown-linux-musl' is not installed` — a container/toolchain provisioning warning in the `deny-advisories` job worth a glance while fixing the advisories, since it means the job may not be running under the pinned toolchain.

### BLOCKING-F — PR body no longer matches the code, and still advertises the mechanism cycle 3 removed

| Field | Value |
|---|---|
| Severity | **blocking** |
| Category | description accuracy (checklist item 2) |
| Location | PR #803 body |

Five concrete mismatches against `c1bd0d54`:

| PR body says | Actual code / diff |
|---|---|
| "Replace per-line extglob with two O(n) whole-file pre-processing passes **via process substitution**" (Decision) | Process substitution was **removed in cycle 3** to close BLOCKING-A. The code uses a caller-managed `mktemp` temp file. |
| "The `awk` and `sed` invocations receive the file path as a positional argument **via process substitution** (`< <(awk 'length <= 8192' \"$path\" \| sed -E '...' )`)" (Security Review) | Same. This quotes verbatim the construct the fix deleted. |
| "New regression tests: **2 added**" | **3** tests added (208KB no-hang; sed ID-strip correctness; preprocessing-failure fail-closed). |
| "Timeout-bounded test (208KB line) **<0.2s**"; "**`timeout 5`** regression test" (Traceability) | Fixture is ~200KB, budget is **3s**, and the test deliberately avoids `timeout` (absent on BSD) in favour of a portable fork + 0.1s-poll loop. The test's own name says "200KB … <3s". |
| "VP-INDEX.md now correctly extracts **106 VPs**" (Consequences) | VP-INDEX.md extracts **107** — `total_vps: 107` is rank 0 *and* first in file order, so it wins. `106` is the rank-1 prose value that loses. (The "BC-INDEX.md extracts 13 stories" half of that sentence is correct.) |

Ordinarily most of these would be suggestions. They are blocking here for two reasons. First, the PR body is the reviewer-facing and merge-record artifact, and it currently documents a mechanism that was deliberately removed for a fail-closed correctness reason — a future reader reconciling body against code would reasonably conclude BLOCKING-A was never fixed. Second, cycle 3's explicit remit included description/comment accuracy; shipping with the body still describing the pre-cycle-3 design means that remit is not met.

Also update the two unchecked Pre-Merge Checklist boxes once D/E clear, and re-check "New regression tests" in the coverage table.

---

## Suggestions (non-blocking)

### SUGGESTION-1 — `mktemp` failure fails *open*, partially undoing BLOCKING-A

`mktemp` failure is the one remaining silent path. Injected a failing `mktemp` shim: the hook exits **1** with no message. Under Claude Code hook semantics only exit 2 blocks; other non-zero codes are non-blocking errors. So on a TMPDIR-exhausted or read-only-tmp host the hook degrades to "no verdict, not blocking" — the exact false-pass shape BLOCKING-A was raised against, just via a different trigger. Gate all three `mktemp` calls (lines 178, 243, 244):

```bash
_preproc_tmp="$(mktemp)" || {
  echo "validate-count-propagation: mktemp failed (cannot preprocess $path)" >&2
  return 1
}
```

and for the two top-level ones, `|| { echo ... >&2; exit 2; }`.

### SUGGESTION-2 — the fail-closed test cannot distinguish the path it exists to test

`validate-count-propagation: preprocessing failure exits 2, not 0 (fail-closed)` asserts only `[ "$status" -eq 2 ]`. But the fixture (`42 BCs` vs `total_bcs: 38`) *also* exits 2 via drift detection. The test happens to discriminate today (a regression to process substitution would yield exit 0), but it asserts the wrong thing. Add the discriminating assertion — I confirmed the string is emitted:

```bash
[[ "$output" == *"preprocessing pipeline failed"* ]]
[[ "$output" == *"count extraction failed"* ]]
```

Consider a second case with a broken `awk` shim too; I verified it also exits 2 with the same messages, so the coverage is free.

### SUGGESTION-3 — `_extract_counts` re-runs once per (keyword × sibling)

The extraction is inside the `for keyword` loop (line 284), so each sibling is re-preprocessed and re-parsed once per keyword — up to 5 × 5 = 25 full awk + sed + bash-read passes over the corpus. ARCH-INDEX.md already takes 685 ms at 3 keywords × 1 sibling. Hoist extraction above the keyword loop: extract each sibling once into its own temp file, parse into an associative array keyed `"$sibling|$keyword"`, then have the keyword loop read the map. Cheap, and it removes a super-linear-in-sibling-count factor from a hook whose whole purpose is to stop being slow.

### SUGGESTION-4 — the PR's own "STATE.md: exit=0" evidence line is vacuous

Worth knowing before the evidence is cited as proof. For `FILE_PATH=.../.factory/STATE.md`, the corpus-root walk (lines 56–62) stops at `.../.factory` itself (`basename == ".factory"`). Every sibling candidate is then either `.factory/.factory/...` (nonexistent) or `.factory/STATE.md` (excluded as the source file). I instrumented it: **sibling count for STATE.md = 0**, so the hook returns at line 87 before extracting anything — which is exactly why STATE.md clocks 67 ms against 379–685 ms for the others.

So STATE.md — the file that carries the authoritative live totals and is edited on every Commit E — receives **no drift checking at all**, and "STATE.md: exit=0" in the live-corpus evidence is a vacuous pass rather than a verification. The corpus-root walk is unchanged by this diff, so this is pre-existing and out of scope to fix here; but the evidence block should stop presenting it as a pass, and it should be routed as a follow-up (`vsdd-factory:state-manager` to file it against a real story ID per Canonical Principle Rule 3 — not "later").

### SUGGESTION-5 — `subsystems` missing from Pattern B's alternation

Pattern A (line 209) matches `(BCs|VPs|stories|capabilities|subsystems)`; Pattern B (line 215) matches `(BCs|VPs|stories|capabilities)` — no `subsystems`. So `| subsystems | 10 |` table cells and `subsystems: 10` YAML never match. Pre-existing, but it compounds BLOCKING-C: with Pattern A's only BC-INDEX carrier now skipped as a blockquote and Pattern B unable to match at all, `subsystems` has zero live carriers corpus-wide. Add `subsystems` to Pattern B in the same edit that resolves BLOCKING-C.

---

## Nits

- **NIT-1** — `rm -f "$_preproc_tmp"` appears three times for one temp file: the `RETURN` trap (179), the error branch (182), and post-loop (231). The trap alone is sufficient and correct (verified single-fire, no leak to nested calls, on bash 3.2 and 5.3). Harmless, but the redundancy invites a future reader to think the trap is unreliable.
- **NIT-2** — lines 22–24 say extglob is "Kept disabled to prevent accidental re-introduction," but nothing enforces that; a comment cannot prevent re-introduction. A one-line CI guard would make the claim load-bearing: `! grep -qE '\+\(\[' plugins/vsdd-factory/hooks/*.sh`.

---

## Checklist result

| # | Item | Result |
|---|---|---|
| 1 | Diff coherence — all changes relate to the fix | **PASS.** 2 files, both on-topic; no drive-by edits. |
| 2 | Description accuracy | **FAIL** — BLOCKING-F (5 mismatches, incl. body describing the removed process-substitution mechanism). |
| 3 | Test coverage on changed lines | **PASS.** 3 new tests cover the runaway path, ID-strip correctness, and the fail-closed path; `bats-full-suite (linux)` green, 12/12 count-propagation tests executed on bash 5. Strengthening noted in SUGGESTION-2. |
| 4 | Demo evidence | **N/A** (accepted) — bash hook hotfix, no UI/AC surface; timing + exit-code evidence substitutes and I reproduced it independently. |
| 5 | Commit quality | **PASS.** 4 conventional commits, `fix(hooks):` scope, cycle-numbered, no AI attribution. |
| 6 | Diff size | **PASS.** +246/-39; ~120 of the +246 are explanatory comments. |
| 7 | Missing changes | **FAIL** — BLOCKING-C (comment does not match measured behaviour; `subsystems` coverage silently dropped). |
| 8 | Dependency / CI status | **FAIL** — BLOCKING-D (`cargo-host` ×2, S-25.01 corpus parity) and BLOCKING-E (`deny-advisories`, external). |

## Verdict

**REQUEST_CHANGES.**

The headline fix is real and I verified it independently: the CPU runaway is gone (4/4 live corpus files exit 0 in 67–585 ms, ARCH-INDEX correctly exits 2 as a true positive), BLOCKING-A is genuinely closed under fault injection on both `awk` and `sed`, SUGGESTION-C's `RETURN` trap is safe on bash 3.2 and 5.3, and two of BLOCKING-B's three comment clauses are now accurate as verified against the live corpus.

What remains:

- **BLOCKING-C** — in-diff, must fix. The blockquote-skip justification is measurably false and hides a real loss of `subsystems` drift coverage. Fold SUGGESTION-5 in.
- **BLOCKING-D** — must fix, route to `vsdd-factory:state-manager`. S-25.01's own STORY-INDEX catalog↔blockquote hashes are unreconciled; `cargo-host` is red on both platforms because of it, and it was green on this branch's merge-base.
- **BLOCKING-F** — must fix, PR body edit only.
- **BLOCKING-E** — external; needs a separate wasmtime bump PR, then a rebase. Not a defect in #803.

SUGGESTION-1 is the one non-blocking item I'd urge folding into the same push, since it closes the last remaining fail-open path in the mechanism BLOCKING-A was raised about.

covered_sha: `c1bd0d54c9be261890b117ffc056a8ebb419a585`

