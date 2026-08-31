# Fresh-eyes PR review — #803 `fix/count-propagation-cpu-runaway`

> **Note on review state:** this review is submitted in GitHub's `COMMENT` state only because GitHub refuses `--request-changes` on a self-authored PR (`Review Can not request changes on your own pull request`). **The verdict below is REQUEST_CHANGES, not an approval.** Do not treat the absence of a red "changes requested" badge as a green light.

**Verdict: REQUEST_CHANGES** (2 blocking, 3 suggestions, 4 nits)
**covered_sha:** `0f9d86629e1690c1d6d11085b6611a888c991dfa`
**Reviewed:** full diff, both changed files, 8-item checklist, plus independent empirical verification against the live corpus and against pre-fix `develop`.

The core fix is **correct, portable, and load-bearing** — I verified it rather than taking the PR body's word for it. The blocking findings are not about the awk/sed mechanism; they are about a real, reproducible change in drift-detection *outcome* on this repository's own corpus that the PR body explicitly claims does not exist.

---

## What I verified independently (not a rubber-stamp)

| Claim under test | Method | Result |
|---|---|---|
| Regression test is load-bearing (not a paper-fix, TD-VSDD-059) | Ran the test-1 scenario against both `origin/develop` and PR head with bash 5.3.9 as interpreter | **Confirmed.** Pre-fix: timed out >3s. Post-fix: ~0.3s. Test genuinely fails without the fix. |
| Dropping `shopt -s extglob` is safe | Grepped the whole script for extglob syntax `[+*?@!](` | **Confirmed safe.** Only remaining match is the comment on line 133; patterns A–D are `[[ =~ ]]` ERE and unaffected. |
| BSD awk survives a 208KB record | `/usr/bin/awk 'length <= 8192'` and `'{print length}'` on a 208,001-byte single-line file | **Confirmed.** `length` = 208000, filter emits 0 bytes, exit 0. No record-size failure. |
| `sed -E` portability + linearity | BSD `sed -E` on an 8,161-char ID-dense line, and on the raw 208KB line | **Confirmed.** 0.047s on 8KB; even the unguarded 208KB line completes with exit 0. awk-before-sed ordering is belt-and-braces, not load-bearing. |
| Process substitution safe on the darwin leg | `< <(...)` is bash 2+ | **Confirmed.** Hook already requires bash 4 for `declare -A`, so no new floor. |
| `pipefail` + early `break` in the sibling loop does not trip `set -e` | Ran test-2 scenario (exercises the `break` path, SIGPIPE to awk/sed) | **Confirmed.** Exits 2 correctly; process-substitution status is not propagated. |
| `awk` needs declaring in `binary_allow` | Checked registry (`binary_allow = ["bash", "jq"]`) against 13 other `.sh` hooks that use awk | **Non-issue.** No hook declares awk; `binary_allow` gates adapter-level execs only. No registry change needed. |
| Test 2 passes | Ran against both versions | Passes on both — correctly labelled a *correctness* test, not a regression test. Only test 1 is bug-load-bearing. |

Registry context that corroborates the root-cause narrative: `timeout_ms = 5000`, `on_error = "continue"`. The adapter times out at 5s but does not reap the bash descendant — which is exactly the reported orphan-to-PPID-1 pathology.

---

## BLOCKING findings

### [BLOCKING-1] The fix un-masks pre-existing false-positive drift, turning three live corpus files from "silent hang" into "exit-2 block"

| Field | Value |
|---|---|
| Severity | **blocking** |
| Category | correctness / operator impact |
| File | `plugins/vsdd-factory/hooks/validate-count-propagation.sh` |

Pre-fix, the hook **hangs indefinitely** on every corpus file with a >8192-char line, so it never emits a verdict. I measured this directly against `origin/develop` with a 15s budget:

```
VP-INDEX.md:    STILL RUNNING after 15s  (never blocks)
STORY-INDEX.md: STILL RUNNING after 15s  (never blocks)
ARCH-INDEX.md:  STILL RUNNING after 15s  (never blocks)
```

Post-fix, the hook completes — and **returns exit 2 on three of the five live index files**:

```
STATE.md         exit=0
BC-INDEX.md      exit=0
VP-INDEX.md      exit=2  '106 VPs' in VP-INDEX.md    but '19 VPs'   in STATE.md
STORY-INDEX.md   exit=2  '15 BCs'  in STORY-INDEX.md but '1993 BCs' in STATE.md
ARCH-INDEX.md    exit=2  '92 VPs'  in ARCH-INDEX.md  but '19 VPs'   in STATE.md
```

`legacy-bash-adapter` maps script `exit_code == 2` to a block with the stderr text as `permissionDecisionReason` (`crates/hook-plugins/legacy-bash-adapter/src/lib.rs`, the `match outcome.exit_code` arm). `on_error = "continue"` governs *plugin* errors, not a deliberate exit-2 block signal — so this surfaces to the session on **every** `Edit`/`Write`/`MultiEdit` to those three files after merge.

**And these are false positives**, driven by two pre-existing weaknesses that the length guard now exposes:

1. `_is_historical_heading` skips only H2 *sections*. It can never reach YAML **frontmatter**, so the `last_amended:` changelog scalar was always in scope for count extraction.
2. First-occurrence-wins picks *scoped sub-counts* over corpus totals. The `19 VPs` above comes from `STATE.md` L231 — `| ... | 19 VPs per §VP Anchors. |`, a per-story anchor count. The `15 BCs` is a per-story table cell. The genuine totals live at `STATE.md` L329: `BC-INDEX v5.33 (1,993 BCs). VP-INDEX v2.90 (106 VPs). STORY-INDEX v4.417 (175 stories; 25 epics).` — which agree with the siblings.

**Suggestion (in-scope per Canonical Principle Rule 4):** the 8192 guard is acting as an accidental proxy for "skip frozen changelog records." Make that intent explicit and correct instead:

```bash
# Skip YAML frontmatter changelog scalars — frozen historical records,
# same rationale as _is_historical_heading (#567), which only reaches H2 sections.
[[ "$line" =~ ^[[:space:]]*(last_amended|change): ]] && continue
```

and/or anchor patterns A/B so a scoped sub-count cannot become the source of truth. At minimum, the live corpus must return exit 0 on all five index files before this merges — otherwise the state-manager's next index write starts emitting block reasons.

---

### [BLOCKING-2] Shipped code comment and PR risk assessment both assert a semantics-preserving no-op that is empirically false

| Field | Value |
|---|---|
| Severity | **blocking** |
| Category | description accuracy / misleading rationale in shipped artifact |
| File | `plugins/vsdd-factory/hooks/validate-count-propagation.sh` (Pass 1 comment block) + PR body |

Three claims, all contradicted by measurement:

- Code comment: *"A legitimate count keyword ... is a short token that cannot live in a 195KB line."*
- PR body, Blast Radius: *"no change to count extraction logic or drift detection semantics."*
- PR body, Root cause: *"Behaviour on normal lines is identical to the old form — no false drift alerts."*

I re-implemented both the old (per-line extglob, no length filter) and new (awk + whole-file sed) extraction paths and diffed the first-wins map on the real corpus:

| File | OLD first-wins | NEW first-wins | Delta |
|---|---|---|---|
| `STATE.md` | BCs=1993 VPs=19 | BCs=1993 VPs=19 | none |
| `BC-INDEX.md` | BCs=1993 **stories=29** subsystems=10 | BCs=1993 **stories=13** subsystems=10 | stories 29 → 13 |
| `VP-INDEX.md` | **VPs=85** | **VPs=106** | VPs 85 → 106 |
| `STORY-INDEX.md` | BCs=15 stories=91 **subsystems=4** | BCs=15 stories=91 | `subsystems` lost |
| `ARCH-INDEX.md` | BCs=1954 **capabilities=18** stories=23 VPs=92 | BCs=1954 stories=23 VPs=92 | `capabilities` lost |

Genuine pattern matches **do** occur inside >8192-char lines. All four deltas trace to line 8, the frontmatter `last_amended:` blob — e.g. ARCH-INDEX L8 yields `capabilities: 18`, STORY-INDEX L8 yields a `subsystems` count, VP-INDEX L8 yields the stale `85 VPs` that was shadowing the correct `total_vps: 106` on L11.

The net effect is arguably an **improvement** (stale changelog counts stop shadowing real totals — VP-INDEX now reports the correct 106). That is a good outcome, but it must be documented as an *intentional semantic change*, not asserted as a no-op. As written, the next maintainer debugging a missing `capabilities` drift alert will read "identical behaviour" and look in the wrong place. Please correct the comment and the Risk Assessment section, and state the intended scope of the guard.

---

## Suggestions

### [SUGGESTION-1] Regression test leaks orphaned CPU-spinning processes on failure

`kill "$pid"` targets only the backgrounded subshell. I reproduced the failure path against pre-fix code and after the kill:

```
leftover hook processes after killing only $pid:
5952 /opt/homebrew/bin/bash .../validate-count-propagation.sh
5965 /opt/homebrew/bin/bash .../validate-count-propagation.sh
```

Since the failure mode under test *is* a CPU runaway, a regressed CI run leaves a core pegged for the remainder of the job — reproducing the exact pathology the PR fixes. Reap descendants:

```bash
if kill -0 "$pid" 2>/dev/null; then
  pkill -P "$pid" 2>/dev/null || true   # reap the bash/awk/sed descendants
  kill "$pid" 2>/dev/null || true
  wait "$pid" 2>/dev/null || true
  timed_out=1
fi
```

### [SUGGESTION-2] Test 1 never asserts exit status — passes vacuously on instant failure

The test asserts only `timed_out -eq 0`. If the hook died immediately (missing `awk`, a syntax error, or a future early-return guard added above `_extract_counts`), the test would still pass while verifying nothing. Given this repo gates on `attestation-gate-non-vacuity-controls`, add the positive assertion:

```bash
{ echo '...' | bash "$HOOKS/validate-count-propagation.sh"; echo "$?" > exit.txt; } &
...
[ "$(cat exit.txt)" -eq 0 ] || fail "hook exited non-zero: $(cat exit.txt)"
```

### [SUGGESTION-3] The `skip "perl may be unavailable"` guard is unreachable

bats runs test bodies under errexit, so if `perl` is absent, `perl -e ... > .factory/BC-INDEX.md` returns 127 and aborts the test *before* the `wc -c` size check ever runs. The guard only protects against "perl succeeded but produced <150KB", which cannot happen. Drop the dependency and the dead guard:

```bash
awk 'BEGIN { s=""; for (i=0;i<8000;i++) s = s "BC-1.18.003 VP-105 DI-007 "; print s }' > .factory/BC-INDEX.md
```

---

## Nits

- **[NIT-1]** Test 1 runs the hook via PATH `bash`, but `require_bash4_hook_interp` validates `/bin/bash`; test 2 uses the shebang. Inconsistent — test 1 would actually execute correctly on macOS via Homebrew bash, yet is skipped by a guard checking a different interpreter.
- **[NIT-2]** Test name says "<3s" and the comment says "Budget: 3s", but 30 iterations of `sleep 0.1` plus per-iteration fork overhead makes the real wall budget ~3.5–4s.
- **[NIT-3]** Undocumented behaviour change: awk re-emits records with `ORS`, so a final line lacking a trailing newline is **now** processed. Pre-fix, `while IFS= read -r line < "$path"` silently dropped it. This is an improvement; worth a line in the comment block.
- **[NIT-4]** Error-path change: `done < "$path"` previously hard-failed under `set -e` on an unreadable file. Now `awk` writes to stderr, the loop yields zero counts, and the hook exits 0 silently. Low risk (both call sites are `-f`-guarded), but it is a silent-failure path where there previously was none.

---

## CI / evidence caveat (not blocking, but must be resolved before merge)

- `deny-advisories` is **FAILURE**. This diff touches zero Rust/Cargo files (`gh pr diff --name-only` = the 2 files under review) and the same job fails on `develop` (`9ab5a6f6`), so it is pre-existing and unrelated — but the PR's own Pre-Merge Checklist requires all checks green. Needs an explicit waiver.
- `bats-full-suite (linux)` had **not reported** at review time. That is the *only* job that actually executes these tests: on macOS all 12 count-propagation tests **skip** via `require_bash4_hook_interp` (`/bin/bash` is 3.2). So the "59/59 PASS" evidence is 12 skips reported as `ok`, not 12 passes. The commit message does disclose the skip behaviour — credit for that — but the PR body's coverage table does not. I independently ran both new tests under bash 5.3.9 and confirmed they pass, so the substance holds; the CI gate still needs to go green.

---

## 8-item checklist

| # | Item | Result |
|---|---|---|
| 1 | Diff coherence | **PASS** — 2 files, both on-topic, no drive-by changes. |
| 2 | Description accuracy | **FAIL** — see BLOCKING-2. Risk Assessment asserts "no change to drift detection semantics"; outcome changes on 4 of 5 live index files. |
| 3 | Test coverage | **PASS with suggestions** — regression test is load-bearing (verified). Weakened by vacuity (S-2) and orphan leak (S-1). No test covers the changed first-wins behaviour from BLOCKING-2. |
| 4 | Demo evidence | **N/A, justified** — bash hook with no UI surface; timing measurements substitute appropriately. |
| 5 | Commit quality | **PASS** — conventional `fix(hooks):`, detailed root-cause body, no AI attribution. |
| 6 | Diff size | **PASS** — +102/-17, well under 500. |
| 7 | Missing changes | **FAIL** — the frontmatter changelog skip that BLOCKING-1 requires is absent; the length guard is standing in for it implicitly. |
| 8 | Dependency status | **PASS** — standalone hotfix, no upstream PR dependency. |

---

## Path to APPROVE

1. Make the live corpus clean (exit 0 on all five index files) — explicit frontmatter changelog skip, and/or anchored count patterns so scoped sub-counts are not treated as totals.
2. Correct the code comment and PR Risk Assessment to describe the actual semantic delta (with the first-wins table above), rather than asserting a no-op.
3. Optionally fold in SUGGESTION-1/2/3 — all are small, and S-1 matters because the failure mode is itself a CPU runaway.

The awk + sed mechanism is the right call and I found no defect in it. Re-review should be quick once the drift outcome is addressed.
