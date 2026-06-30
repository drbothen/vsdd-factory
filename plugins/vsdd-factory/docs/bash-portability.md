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
lowercase `-a` (indexed arrays, which are bash-3-safe) — the test then verifies that
the token `BASH_VERSINFO` also appears somewhere in the scan set. Both
`[ "${BASH_VERSINFO[0]:-0}" -lt 4 ]` and `(( BASH_VERSINFO[0] < 4 ))` satisfy this
check. A scan set with associative arrays but no `BASH_VERSINFO` token fails the test.
A scan set with neither associative arrays nor a guard passes (the feature is simply
absent, so no guard is required).

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
`${*^^}`, and `${#^^}` — the test verifies that `BASH_VERSINFO` appears somewhere in
the scan set. Case modifiers without a version guard fail the test. The current scan
set contains no case modifiers, so this test passes on a clean codebase (no guard is
required when the feature is absent).

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

Wave-handoff scripts must not invoke external tools without a preflight availability
check. Two specific tools are guarded: `python3`/PyYAML (AC-004) and `jq` (AC-005).

### AC-004 — python3 / PyYAML

**Pattern detected:** Any invocation of `python`, `python2`, `python3`, or a
version-suffixed variant such as `python3.11` or `python3.12`, followed on the same
line by a yaml token; any invocation of `pip`, `pip2`, `pip3`, or `pipx` followed by
a pyyaml token (case-insensitive, so both `pyyaml` and `PyYAML` are matched); or a
bare `import yaml` statement in any wave-handoff script.

**Exception (not flagged):** `python3` invocations that use only stdlib modules (`json`,
`os`, `sys`, `re`, etc.) are fine and are not flagged. The guard targets only external
pip packages.

**Why it breaks:** macOS GitHub Actions runners enforce PEP 668 "externally managed
environment" isolation, which blocks `pip install` commands from modifying the system
Python. The S-18.01 history has two entries for this failure: commit aaa8da8a (original
pip failure) and commit 3fe11ea1 (attempted workaround with `--break-system-packages`).
The `--break-system-packages` flag is explicitly not accepted as a long-term resolution
in wave-handoff scripts — it works around the runner policy rather than eliminating the
dependency.

**Required fix:** Either add a preflight check before the first PyYAML invocation:

```bash
python3 -c "import yaml" 2>/dev/null || {
  echo "ERROR: PyYAML is required (pip install pyyaml)" >&2
  exit 1
}
```

or replace yaml parsing with a POSIX-portable alternative (`awk` or `sed` key
extraction). The current wave-handoff implementation uses `awk`-based YAML parsing and
contains no `import yaml` invocations, so this test passes on a clean codebase.

**What the guard checks (phase 1):** Detects files containing any of: a `python`,
`python2`, `python3`, or version-suffixed binary (`python3.11`, `python3.12`, etc.)
invocation followed anywhere on the same line by a case-insensitive yaml token (e.g.,
`yaml`, `Yaml`, `YAML`) — the python arm uses the pattern `python[0-9.]*` to cover all
such variants; a `pip`, `pip2`, `pip3`, or `pipx` invocation followed by a
case-insensitive `pyyaml` token — the pyyaml token match is case-insensitive so the
canonical PyPI name `PyYAML` (as in `pip3 install PyYAML`) is detected alongside
lowercase `pyyaml`; or a bare `import yaml` statement. If none found, the test passes
immediately. **Phase 2 (if
matches found):** The test first flags any occurrence of `--break-system-packages` as
an explicit violation. For remaining matches without that flag, it checks whether the
file contains an acceptable preflight guard matching
`(python[0-9.]*[[:space:]]+-c[[:space:]]+["']import yaml["']|command[[:space:]]+-v[[:space:]]+python[0-9.]*)`.
This covers both bare `python3 -c "import yaml"` and `python3 -c 'import yaml'`
(single or double quotes accepted), versioned-binary forms such as
`python3.11 -c "import yaml"`, and both `command -v python3` and versioned forms such as
`command -v python3.11`; absence of any such guard is a violation.

**Enforcing test:** `test_portability_no_undeclared_pyyaml_dep`

### AC-005 — jq

**Pattern detected:** `jq` appearing as a standalone command token in ANY execution
position: at the start of a line, after a pipe (`|`), after a semicolon (`;`), after an
ampersand (`&`), inside a command substitution (`$(jq ...)` or backtick form), as an
argument to `xargs` (including `xargs` with intervening options such as `xargs -n1 jq`),
following wrapper keywords `if`, `then`, `do`, `else`, `elif`, `time`, `env`, `command`,
or `sudo`, inside a brace group (`{ jq . f; }`), as a case-pattern-action (`case $x in
p) jq … ;;`), or inside a subshell (`( jq … )`).

**Why it is forbidden:** `jq` is prohibited by the wave-handoff SKILL.md contract —
scripts MUST NOT shell out to `jq`. This is not a "check for presence before using"
situation: the contract forbids `jq` in any execution position, including subshells. Any
detection of `jq` as a command word is a violation regardless of position. The correct
fix is removal, not the addition of a preflight guard.

**What the guard checks (phase 1):** Detects files where `jq` appears as a command
word in any execution position using POSIX ERE (no `\b` shorthand). The detector regex is:

```
(^[[:space:]]*jq([[:space:]]|$)|[|;&][[:space:]]*jq([[:space:]]|$)|\$[(]jq([[:space:]]|$)|`jq([[:space:]]|$)|(xargs|if|then|do|else|elif|time|env|command|sudo)[[:space:]]+jq([[:space:]]|$)|xargs([[:space:]]+-[^[:space:]]+)+[[:space:]]+jq([[:space:]]|$)|\{[[:space:]]*jq([[:space:]]|$)|\)[[:space:]]*jq([[:space:]]|$)|\([[:space:]]*jq([[:space:]]|$))
```

This covers `jq` at line-start, after pipe/semicolon/ampersand, inside `$(...)` or
backtick command substitution, after `xargs` (with or without intervening options), after
the wrapper keywords `if`, `then`, `do`, `else`, `elif`, `time`, `env`, `command`, and
`sudo`, inside a brace group (`{ jq`), as a case-pattern-action body (`) jq`), and
inside a subshell (`( jq`). If no such files are found, the test passes. **Phase 2:** For
each file containing a `jq` invocation, the test verifies that `command -v jq` or
`which jq` appears somewhere in the same file; absence is a violation. No `jq`
invocations currently exist in the wave-handoff scripts; this guard was added
prospectively to prevent the dependency from being introduced silently.

**Soundness boundary:** The jq preflight check is whole-file (greps for `command -v jq`
or `which jq` anywhere in the file), NOT positional (unlike AC-001's entrypoint guard,
which requires the `BASH_VERSINFO` check to precede first use). This is accepted as
designed. `jq` is forbidden by the wave-handoff SKILL.md contract — scripts MUST NOT
shell out to `jq` — so this detector is defense-in-depth against prohibited dependency
introduction, not a runtime-ordering guard. For a dependency that must never appear at
all, whole-file presence detection is adequate: if `jq` appears anywhere, it is a
violation regardless of where a guard would be placed. A positional precedence refinement
(requiring the guard to appear before the first `jq` invocation) is intentionally out of
scope because the correct fix for any `jq` detection is removal, not guard placement.

**Enforcing test:** `test_portability_no_undeclared_jq_dep`

---

## Guard scope and non-vacuity

Every test in this suite enforces a non-vacuity invariant (EC-005): each test asserts
that at least one `.sh` file was found under `plugins/vsdd-factory/skills/wave-handoff/`
before drawing any conclusions. If the scan set is empty — because the scripts were
renamed or moved — the test fails with a scope-drift message rather than silently
passing. This prevents the guard from becoming a no-op after a refactor.

The guard does not scan test fixture files, hook scripts under
`plugins/vsdd-factory/hooks/`, or any script outside the wave-handoff skill directory.
