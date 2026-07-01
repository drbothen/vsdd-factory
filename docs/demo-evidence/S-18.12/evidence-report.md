# Demo Evidence — S-18.12: portability-lint guard extension

**Story:** S-18.12 (v1.12)
**Branch:** feature/S-18.12
**HEAD at capture:** `9cbd9439` (clean worktree)
**Suite:** `plugins/vsdd-factory/tests/wave-handoff.bats` (portability-lint subset)
**Product type:** Gate-enforcement bats suite (CLI/shell — no UI)
**Recording method:** Literal bats execution output (VHS not applicable; the deliverable is a
static-lint bats guard, not an interactive CLI tool — same rationale as S-18.09's evidence report)
**Captured:** 2026-07-01T16:09:43Z

---

## Gate Run: All 6 Portability Tests Green

Command executed from worktree root (`/Users/zious/Documents/GITHUB/vsdd-factory/.worktrees/S-18.12`):

```
bats -f "test_portability_|test_BC_5_41_001_F_P11_001_bsd_portability" plugins/vsdd-factory/tests/wave-handoff.bats
```

**Captured output (verbatim):**

```
1..6
ok 1 test_BC_5_41_001_F_P11_001_bsd_portability_no_pcre_classes_in_grep_sed
ok 2 test_portability_no_unguarded_local_A_associative_array
ok 3 test_portability_no_unguarded_bash4_case_modifiers
ok 4 test_portability_no_global_ifs_mutation
ok 5 test_portability_no_python_shellout
ok 6 test_portability_no_jq_shellout
```

Exit code: 0. No failures, no skipped tests. Bats 1.13.0.

---

## Per-AC Summary (AC-001 through AC-005, plus AC-007 regression)

Each of the five new tests embeds its own positive-control (synthetic BAD fixture that MUST be
flagged) and negative-control (synthetic GOOD/exempt fixture that MUST NOT be flagged) assertions
against the detector regex, in addition to scanning the real `wave-handoff` skill scripts. All
fixtures are synthetic files under `BATS_TEST_TMPDIR` — no real wave-handoff scripts are executed
or mutated by the controls. A single `ok` line therefore proves both discrimination-correctness
(the regex fires on bad input and stays silent on good input) and the real-scan-set result (no
hazard present in the shipped skill).

| Test # | Test Name | AC | Positive control (MUST flag) | Negative control (MUST NOT flag) | Result |
|--------|-----------|----|------------------------------|-----------------------------------|--------|
| 1 | `test_BC_5_41_001_F_P11_001_bsd_portability_no_pcre_classes_in_grep_sed` | AC-007 (regression) | Pre-existing PCRE-shorthand-class guard (`\d`, `\w`, `\s` in `grep`/`sed`) | — | PASS |
| 2 | `test_portability_no_unguarded_local_A_associative_array` | AC-001 | `declare -A`, `declare -Ax`, `declare -gA`, `local -A` (incl. EOL forms), and an unguarded entrypoint sourcing a `local -A` lib | `declare -a` (indexed array, bash-3-safe); a guarded entrypoint (`BASH_VERSINFO` check precedes `source`); a commented-out guard (F-P14-001) does NOT satisfy the oracle | PASS |
| 3 | `test_portability_no_unguarded_bash4_case_modifiers` | AC-002 | `${var^^}`, `${var,,}`, `${var^}`, `${var,}`, positional/special-param forms (`${1^^}`, `${@^^}`, `${*^^}`, `${#^^}`), and `${var@U}`/`${var@L}`/`${var@u}` `@`-operator transforms | Named vars without case-modifier operators; guarded usage | PASS |
| 4 | `test_portability_no_global_ifs_mutation` | AC-003 | Bare `IFS='|'`; `export`/`readonly`/`declare -g IFS=`; `cmd; IFS=`; `cmd && / || / & IFS=`; `then/do/else/elif IFS=`; brace-group `{ IFS=`; case-pattern `) IFS=`; `IFS='|'; read x` (semicolon-separated, F-P3-001) | `local IFS=':'`; `IFS=',' read -ra arr` (command-prefix form); `foo() { ... }` (function-def brace, not IFS); `) ` with no following `IFS=` | PASS |
| 5 | `test_portability_no_python_shellout` | AC-004 | `python3`, `python2`, `pip3`, `pipx`, version-suffixed `python3.11`, and stdlib-only `python3 -c 'import json'` (F-P11-001 Option A: stdlib is now also forbidden) across line-start / pipe / `&&` / backtick / `$(...)` / `if`/`then`/`do`/`else`/`elif` / `time`/`env`/`command`/`sudo` / `xargs` (incl. `-n1`) / brace-group / case-pattern / subshell positions, incl. a `command -v python3` preflight guard (F-P13-002: still forbidden, no preflight-acceptance path) | `python_bin=python3.11` (var name, not invocation); `# python3 is not used...` (comment); `echo "install python3 first"` (argument, not invocation); `foo() { echo; }` (function-brace); `echo "${python3_x}"` (param expansion) | PASS |
| 6 | `test_portability_no_jq_shellout` | AC-005 | `jq` at line-start / `$(...)` / backtick / `&&` / `else`/`elif`/`if`/`then`/`do` / `time`/`env`/`command`/`sudo` / `xargs` (incl. `-n1`) / brace-group / case-pattern / subshell positions, incl. a `command -v jq` preflight guard (forbidden-removal model: no preflight-acceptance path, mirrors AC-004) | `# no jq dependency; using awk instead` (comment); `result=$(other_cmd . f)` (non-jq cmdsubst); `foo() { echo "hello"; }` (function-brace); `echo "${jq_var}"` (param expansion) | PASS |

---

## What This Suite Enforces

The five new `test_portability_*` tests extend the existing `bsd_portability_no_pcre_classes`
bats guard (AC-007, preserved unmodified and still passing as test 1) to close the four runtime
portability gaps that a static PCRE-class-only lint missed and that were only caught by the macOS
CI leg in S-18.01 (lesson `L-S18-macos-ci-leg-caught-runtime-portability`):

- **AC-001 — bash 4+ associative arrays.** `local -A` / `declare -A` (and combined/prefixed flag
  forms) require bash ≥ 4.0; macOS ships bash 3.2 as `/bin/bash`. The guard requires either no
  associative-array usage in the scan set, or an executable `BASH_VERSINFO` guard that precedes
  every entrypoint's first `source` of a lib using the syntax (EC-006 entrypoint-guard soundness;
  a commented-out guard does not count, per F-P14-001).
- **AC-002 — bash 4+ case-modifier parameter expansion.** `${var^^}`/`${var,,}`/`${var^}`/`${var,}`
  and the bash-4.4 `@`-operator transforms (`${var@U}` etc.) are syntax errors on bash 3.2.
- **AC-003 — global IFS mutation.** A bare/`export`/`readonly`/`declare -g` `IFS=` assignment at
  current-shell scope leaks into the parent shell and corrupts word-splitting for the rest of the
  script. `local IFS=` and the `IFS=... read` command-prefix idiom are the only exemptions;
  subshell-scoped `(IFS=...)` is exempt by shell semantics (does not leak).
- **AC-004 — forbidden python/pip shell-outs.** SKILL.md's "Forbidden Dependencies" section
  forbids any language runtime beyond bash. Per F-P11-001 Option A, ALL python/pip invocations
  (including stdlib-only, and even behind a `command -v` preflight guard) are violations; the fix
  is removal, not acceptance.
- **AC-005 — forbidden jq shell-outs.** Same forbidden-removal model as AC-004 (v1.11 reframe
  closed the earlier python/jq asymmetry): jq is forbidden in any execution position, with no
  preflight-acceptance path.

AC-006 (guard is scoped to wave-handoff scripts and is not a false-positive source) and AC-007
(existing PCRE guard preserved) require no dedicated new test per the story's Test Plan — AC-006
is demonstrated structurally by every test scanning only `wave-handoff_skill_dir/*.sh`, and AC-007
is the regression check that test 1 above continues to pass unmodified.

---

## Coverage Mapping

| AC | Test | Result |
|----|------|--------|
| AC-001 | `test_portability_no_unguarded_local_A_associative_array` | PASS |
| AC-002 | `test_portability_no_unguarded_bash4_case_modifiers` | PASS |
| AC-003 | `test_portability_no_global_ifs_mutation` | PASS |
| AC-004 | `test_portability_no_python_shellout` | PASS |
| AC-005 | `test_portability_no_jq_shellout` | PASS |
| AC-006 | (no dedicated test — structural: scan scope is `wave-handoff_skill_dir` only, per story Test Plan) | N/A (by design) |
| AC-007 | `test_BC_5_41_001_F_P11_001_bsd_portability_no_pcre_classes_in_grep_sed` (regression) | PASS |

All 5 Red Gate tests (AC-001 through AC-005) plus the AC-007 regression check are green. Each new
test's own positive/negative-control block (see table above) demonstrates that the detector
regexes discriminate correctly, independent of whether the real `wave-handoff` skill happens to
contain the hazard today — the guard is a true regression fence, not a vacuous pass.

## Reproduction

```bash
# Re-run just the portability subset (this evidence):
bats -f "test_portability_|test_BC_5_41_001_F_P11_001_bsd_portability" plugins/vsdd-factory/tests/wave-handoff.bats

# Re-run the full wave-handoff suite:
bats plugins/vsdd-factory/tests/wave-handoff.bats
```
