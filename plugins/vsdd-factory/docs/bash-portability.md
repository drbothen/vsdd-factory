# Bash Portability — Wave-Handoff Static Lint Guard

This document describes the four bash portability anti-patterns that the S-18.12
portability-lint bats guard detects. The guard performs static analysis only — it
greps script sources and never executes them. It scans every `.sh` file under
`plugins/vsdd-factory/skills/wave-handoff/` (main entry point and `lib/` helpers).
Test fixtures, hook scripts, and scripts outside that directory are out of scope.

These guards are anchored to lesson `L-S18-macos-ci-leg-caught-runtime-portability`,
which records the four macOS CI failures in S-18.01 that the earlier PCRE-only lint
missed. Each anti-pattern below has a matching bats test in
`plugins/vsdd-factory/tests/wave-handoff.bats`.

---

## 1. Unguarded bash 4+ associative arrays (AC-001)

**Pattern detected:** `local -A varname`, `declare -A varname`, or any flag string
containing `A` at any position, such as `local -Ax` or `declare -gA`, in any
wave-handoff script. Lowercase `-a` (indexed arrays, bash-3-safe) is NOT flagged.

**Why it breaks:** macOS ships bash 3.2 at `/bin/bash`. Homebrew bash is not on PATH
by default. `local -A` and `declare -A` are bash 4+ features. On bash 3.2 they do not
produce a parse-time error — the function body parses without complaint. Instead they
produce a **runtime** builtin error (`declare: -A: invalid option`) at the moment the
builtin executes, which only happens if the array-using function is actually invoked.
GitHub Actions macOS runners also use the system bash unless the workflow explicitly
installs a newer version. This was the root cause of the S-18.01 CI failure recorded at
commit ea7328ac.

The entrypoint guard works because `wave-handoff.sh` runs the `${BASH_VERSINFO[0]} -lt 4`
check at the top of the script, before it `source`s any `lib/*.sh` script, and exits. The lib functions
that use `local -A` or `declare -A` are therefore never reached on bash 3.2, and the
runtime builtin error never fires. Per-function guards inside individual lib functions
are not impossible — they are simply unnecessary given the entrypoint early-exit.

**Portable fix:** Add a version guard at the top of `wave-handoff.sh` (the main entry
point, before any lib script is sourced) so the check fires before any bash 4+ syntax
is evaluated:

```bash
if [ "${BASH_VERSINFO[0]:-0}" -lt 4 ]; then
  echo "ERROR: wave-handoff requires bash >= 4.0 (associative arrays)" >&2
  echo "  On macOS: brew install bash" >&2
  exit 1
fi
```

**Soundness boundary:** The entrypoint-guard model is sound only because `wave-handoff.sh`
is the sole guarded entrypoint and the `lib/*.sh` scripts are sourced exclusively by it
(after the guard). Adding a new lib script that is sourced only by the guarded entrypoint
is safe — the guard fires before any lib is sourced, so the new lib is never reached on
bash 3.2. The hazard is a new entrypoint that sources these libs without its own bash-4
guard: in that case the scan-set-wide `BASH_VERSINFO` check could pass while that new
code path crashes on bash 3.2 at runtime. Any new entrypoint that sources these libs
must carry its own bash-4 guard.

**What the guard checks:** If any file in the scan set matches
`(local|declare)[[:space:]]+-[a-zA-Z]*A[a-zA-Z]*([[:space:]]|$)` — covering bare `-A`, compound flags
with `A` at any position such as `-Ax` or `-gA`, and `-A` at end-of-line, but NOT
lowercase `-a` (indexed arrays, which are bash-3-safe) — the test then verifies that an
**executable** (non-comment) bash-version conditional exists in the scan set. Specifically,
the detector strips comment lines (`^[[:space:]]*#`) from each file before applying the
guard pattern `([[].*BASH_VERSINFO|[(][(].*BASH_VERSINFO)`, which requires either `[` or `((`
to appear on the same non-comment line as `BASH_VERSINFO`. This covers both
`[ "${BASH_VERSINFO[0]:-0}" -lt 4 ]` and `(( BASH_VERSINFO[0] < 4 ))`. A scan set with
associative arrays but no executable guard fails the test. A commented-out guard line
such as `# if [[ ${BASH_VERSINFO[0]} -lt 4 ]]; then` does NOT satisfy the check —
the comment is stripped before the pattern is applied. A scan set with neither
associative arrays nor a guard passes (the feature is simply absent, so no guard is
required).

**Enforcing test:** `test_portability_no_unguarded_local_A_associative_array`

---

## 2. Unguarded bash 4+ case modifiers (AC-002)

**Pattern detected:** `${varname^^}` (to-uppercase), `${varname,,}` (to-lowercase),
`${varname^}` (first-character uppercase), `${varname,}` (first-character lowercase),
`${varname@U}` (bash-4.4 to-uppercase transform), `${varname@L}` (bash-4.4
to-lowercase transform), or `${varname@u}` (bash-4.4 first-character uppercase
transform) in any wave-handoff script, including array-element forms such as
`${arr[0]^^}`, `${map[k],,}`, and `${arr[i]^}`, and positional and special parameters
such as `${1^^}`, `${@^^}`, `${*^^}`, and `${#^^}`.

**Why it breaks:** These parameter expansion operators were introduced in bash 4.0. On
bash 3.2 (macOS system bash) they produce a **runtime** `bad substitution` error at the
moment that specific parameter expansion executes — not a parse-time error. The script
body and function declarations parse without complaint on bash 3.2; the error fires only
when execution reaches the offending expansion. The same entrypoint guard that covers
AC-001 prevents this: `wave-handoff.sh` exits before sourcing the lib scripts, so
expansions using these operators are never evaluated on bash 3.2.

**Portable fix (option A):** Use POSIX-portable alternatives instead of the bash 4+
operators:

```bash
# to-uppercase
upper="$(printf '%s' "$var" | tr 'a-z' 'A-Z')"
# or with awk
upper="$(printf '%s' "$var" | awk '{print toupper($0)}')"
```

**Portable fix (option B):** Add the same `BASH_VERSINFO` guard as AC-001 to
`wave-handoff.sh` before any lib script is sourced.

**What the guard checks:** If any file matches the pattern
`\$\{([a-zA-Z_][a-zA-Z0-9_]*|[0-9]+|[@*#])(\[[^]]*\])?(\^\^?|,,?|@[ULu])` — covering the
two-char forms `^^` and `,,`, the single-char forms `^` and `,`, the bash-4.4
`@`-operator transforms `@U` / `@L` / `@u`, named variable forms
(`${varname^^}`), array-element forms such as `${arr[0]^^}`, `${map[k],,}`, and
`${arr[i]^}`, and positional and special-parameter forms such as `${1^^}`, `${@^^}`,
`${*^^}`, and `${#^^}` — the test verifies that an **executable** (non-comment)
bash-version conditional exists in the scan set using the same comment-stripping
mechanism as AC-001: comment lines (`^[[:space:]]*#`) are stripped before the guard
pattern `([[].*BASH_VERSINFO|[(][(].*BASH_VERSINFO)` is applied. A commented-out guard
line does NOT satisfy the check. Case modifiers without an executable version guard fail
the test. The current scan set contains no case modifiers, so this test passes on a
clean codebase (no guard is required when the feature is absent).

**Enforcing test:** `test_portability_no_unguarded_bash4_case_modifiers`

---

## 3. Global IFS mutation (AC-003)

**Pattern detected:** A bare `IFS=...` assignment that is not scoped to a function
local variable, a command prefix for `read`, or a subshell — detected at line-start,
after `;`, after `&&` or `||`, after a single `&` (background command), after `{ `
(brace group — runs in current shell), after `) ` (case pattern-action body — runs in
current shell), and after the keywords `then`, `do`, `else`, and `elif`.

**Why it breaks:** Assigning to `IFS` at script scope or as a standalone statement
inside a function mutates the shell's global field separator for the remainder of that
shell process. All subsequent `read` calls, word splits, and `for` expansions in the
same process inherit the mutated IFS, causing silent misclassification of delimited
fields. This is a SOUL.md §4 silent-failure category defect: the script continues
running, produces wrong output, and emits no error. This was the root cause of the
S-18.01 failure recorded at commit 2b40dfd5 (`IFS='|'` in `parse-sprint-state.sh`
caused every subsequent `read` to split on `|` instead of newline).

**Allowed forms (not flagged):**

```bash
local IFS=':'           # scoped to the enclosing function body
while IFS= read -r line # command prefix, scoped to that single read
IFS=',' read -ra arr    # command prefix, scoped to that single read
(IFS='|'; command)      # subshell-scoped (subshell opens on the same line)
```

**Flagged forms (violations):**

```bash
IFS='|'                 # standalone assignment — global mutation
IFS=$'\n'               # standalone assignment — global mutation
cmd && IFS=':'          # &&-operator prefix — global mutation
cmd & IFS='|'           # single-& background then global mutation
cmd || IFS=$'\n'        # ||-operator prefix — global mutation
if ...; then IFS=':'    # then-keyword prefix — global mutation in if-body
for ...; do IFS='|'     # do-keyword prefix — global mutation in loop-body
else IFS=':'            # else-keyword prefix — global mutation in else-branch
elif ...; then IFS=':'  # elif-keyword prefix — global mutation in elif-body
{ IFS=$'\n'; read x; }  # brace-group — runs in current shell, global mutation
case $x in p) IFS='|' ;; esac  # case pattern-action body — runs in current shell, global mutation
```

**Portable fix:** Replace the global assignment with one of the scoped forms above.
The S-18.01 fix replaced `IFS='|'` field splitting in `parse-sprint-state.sh` with
`awk -F'|'`, which keeps field-separator semantics entirely inside awk without touching
the shell's IFS.

**What the guard checks:** The test applies the step-1 anchor
`(^|;|&&|&|[|][|]|(then|do|else|elif)[[:space:]]|\{[[:space:]]+|[)][[:space:]]+)[[:space:]]*(export[[:space:]]+|readonly[[:space:]]+|declare[[:space:]]+-g[[:space:]]+)?IFS=`,
which anchors the positions where `IFS=` mutates the current (parent) shell: bare
line-start `IFS=`; second-statement uses such as `cmd; IFS=`; operator-prefixed
mutations such as `cmd && IFS=`, `cmd & IFS=` (background-then-global), and
`cmd || IFS=`; brace-group mutations such as `{ IFS=...` (brace group — runs in the
current shell, not a subshell); case pattern-action body mutations such as `) IFS=...`
(case pattern-action body — runs in the current shell, not a subshell); and
keyword-prefixed mutations where `IFS=` follows `then`, `do`, `else`, or `elif` with
whitespace (e.g., `then IFS=...` in an `if` body or `do IFS=...` in a loop body). The
subshell form `( IFS=...; ... )` is intentionally excluded from the anchor: `IFS` set
inside a subshell is scoped to that subshell and does not leak back to the parent
shell. The anchor covers the qualified forms `export IFS=`, `readonly IFS=`, and
`declare -g IFS=` in any of those positions. The following forms are exempted and not
flagged: `local IFS=` (function-scoped), `while IFS= read` and `IFS=... read`
(command-prefix scoped to the single `read` call), and `(IFS=...; ...)` (subshell-scoped,
as above). The command-prefix exemption applies only to the no-separator form `IFS=val read`
— the exemption does not cross `;`, `&`, or `|` statement separators, so `IFS=val; read`
(global assignment followed by a separate `read` statement) IS flagged. Any match not
covered by an exemption is reported as a violation.

**Enforcing test:** `test_portability_no_global_ifs_mutation`

---

## 4. Undeclared runtime dependencies (AC-004 and AC-005)

Wave-handoff scripts must not invoke external tools. Two tools are guarded: python/pip
(AC-004, any invocation is a violation) and `jq` (AC-005, any invocation is a violation).

### AC-004 — python / pip shell-out prohibition (F-P11-001 Option A)

**Pattern detected:** Any invocation of `python`, `python2`, `python3`, a
version-suffixed variant such as `python3.11` or `python3.12`, `pip`, `pip2`, `pip3`,
or `pipx` in a command position in any wave-handoff script.

**No exceptions:** There is no stdlib exemption and no preflight-acceptance path.
`python3 -c 'import json'` is a violation. `python3 -c 'import yaml'` is a violation.
Any python or pip invocation in a command position is a violation, period. The fix is
removal of the invocation, not the addition of a guard.

**Flagged forms (violations):**

```bash
python3 parse.py                             # command-position python3 invocation
python3 -c 'import json; print(x)'          # stdlib python3 — still flagged
python3 -c 'import yaml; ...'               # third-party python3 — still flagged
python2 parse.py                             # python2 variant
pip3 install pyyaml                          # pip3 invocation
pipx run black .                             # pipx invocation
$(python3 -c 'import json; ...')             # command substitution — flagged
| python3 -c '...'                           # pipe position — flagged
```

**Why it is forbidden:** SKILL.md 'Forbidden Dependencies' section states: "This skill MUST NOT shell out to
Python, jq, or any language runtime beyond bash." Python is treated identically to jq —
the constraint is a hard prohibition, not a "declare a dependency" requirement. macOS
does not guarantee `python3` on PATH — it is absent on a clean macOS install and on CI
images that do not provision Python separately. PEP 668 (externally-managed Python
environments, adopted in Python 3.11+ and backported by major Linux distributions)
prevents `pip` from installing third-party packages without `--break-system-packages`,
making any preflight guard that attempts to install dependencies non-functional on modern
Linux and macOS systems. The S-18.01 history (commit aaa8da8a: pip failure; commit
3fe11ea1: `--break-system-packages` workaround) is the concrete manifestation of this
fragility. (Background: `.factory/planning/research/s-18.12-python-dep-policy.md`.)

**Required fix:** Remove the python/pip invocation entirely. Replace with a POSIX
portable alternative using `awk`, `grep`, or `sed`.

**What the guard checks:** Detects `python[0-9.]*` and `pip[0-9x]*` as command tokens
in all execution positions using command-position anchoring analogous to the jq detector
(AC-005). The detector regex is:

```
(^[[:space:]]*(python[0-9.]*|pip[0-9x]*)([[:space:]]|$)|[|;&][[:space:]]*(python[0-9.]*|pip[0-9x]*)([[:space:]]|$)|\$[(](python[0-9.]*|pip[0-9x]*)([[:space:]]|$)|`(python[0-9.]*|pip[0-9x]*)([[:space:]]|$)|(xargs|if|then|do|else|elif|time|env|command|sudo)[[:space:]]+(python[0-9.]*|pip[0-9x]*)([[:space:]]|$)|xargs([[:space:]]+-[^[:space:]]+)+[[:space:]]+(python[0-9.]*|pip[0-9x]*)([[:space:]]|$)|\{[[:space:]]*(python[0-9.]*|pip[0-9x]*)([[:space:]]|$)|\)[[:space:]]*(python[0-9.]*|pip[0-9x]*)([[:space:]]|$)|\([[:space:]]*(python[0-9.]*|pip[0-9x]*)([[:space:]]|$))
```

Any match is a violation. The test is single-phase (no preflight-acceptance phase).
Variable names that start with `python` (e.g., `python_bin=python3.11`) do not match
because `_bin` immediately follows `python`, not a space or end-of-line.

**Enforcing test:** `test_portability_no_python_shellout`

### AC-005 — jq

**Pattern detected:** `jq` appearing as a standalone command token in ANY execution
position: at the start of a line, after a pipe (`|`), after a semicolon (`;`), after an
ampersand (`&`), inside a command substitution (`$(jq ...)` or backtick form), as an
argument to `xargs` (including `xargs` with intervening options such as `xargs -n1 jq`),
following wrapper keywords `if`, `then`, `do`, `else`, `elif`, `time`, `env`, `command`,
or `sudo`, inside a brace group (`{ jq . f; }`), as a case-pattern-action (`case $x in
p) jq … ;;`), or inside a subshell (`( jq … )`).

**No exceptions:** There is no preflight-acceptance path. A `command -v jq` preflight
guard does not make a subsequent `jq` invocation acceptable — `jq` is forbidden in any
execution position regardless of whether a guard precedes it. The fix is removal of the
invocation, not the addition of a guard.

**Flagged forms (violations):**

```bash
jq '.key' data.json                          # command-position jq invocation
| jq .                                       # pipe position — flagged
$(jq '.key' data.json)                       # command substitution — flagged
command -v jq && jq '.key' data.json         # preflight guard present — jq still flagged
if command -v jq; then jq '.key' f; fi       # preflight guard present — jq still flagged
xargs -n1 jq '.key'                          # xargs position — flagged
```

**Why it is forbidden:** SKILL.md 'Forbidden Dependencies' section states: "This skill MUST NOT shell out to
Python, jq, or any language runtime beyond bash." `jq` is a non-guaranteed third-party
binary — it is absent from minimal macOS installs and from CI images that do not
provision it separately. The constraint is a hard prohibition, not a "declare a
dependency" requirement. `jq` is treated identically to Python — the correct fix for any
detection is removal, not the addition of a preflight guard.

**Required fix:** Remove the `jq` invocation entirely. Replace with a POSIX-portable
alternative using `awk`, `grep`, or `sed`.

**What the guard checks:** Detects files where `jq` appears as a command word in any
execution position using POSIX ERE (no `\b` shorthand). The detector regex is:

```
(^[[:space:]]*jq([[:space:]]|$)|[|;&][[:space:]]*jq([[:space:]]|$)|\$[(]jq([[:space:]]|$)|`jq([[:space:]]|$)|(xargs|if|then|do|else|elif|time|env|command|sudo)[[:space:]]+jq([[:space:]]|$)|xargs([[:space:]]+-[^[:space:]]+)+[[:space:]]+jq([[:space:]]|$)|\{[[:space:]]*jq([[:space:]]|$)|\)[[:space:]]*jq([[:space:]]|$)|\([[:space:]]*jq([[:space:]]|$))
```

This covers `jq` at line-start, after pipe/semicolon/ampersand, inside `$(...)` or
backtick command substitution, after `xargs` (with or without intervening options), after
the wrapper keywords `if`, `then`, `do`, `else`, `elif`, `time`, `env`, `command`, and
`sudo`, inside a brace group (`{ jq`), as a case-pattern-action body (`) jq`), and
inside a subshell (`( jq`). Any match is a violation. The test is single-phase (no
preflight-acceptance phase). No `jq` invocations currently exist in the wave-handoff
scripts; this guard was added prospectively to prevent the dependency from being
introduced silently.

**Enforcing test:** `test_portability_no_jq_shellout`

---

## Guard scope and non-vacuity

Every test in this suite enforces a non-vacuity invariant (EC-005): each test asserts
that at least one `.sh` file was found under `plugins/vsdd-factory/skills/wave-handoff/`
before drawing any conclusions. If the scan set is empty — because the scripts were
renamed or moved — the test fails with a scope-drift message rather than silently
passing. This prevents the guard from becoming a no-op after a refactor.

The guard does not scan test fixture files, hook scripts under
`plugins/vsdd-factory/hooks/`, or any script outside the wave-handoff skill directory.
