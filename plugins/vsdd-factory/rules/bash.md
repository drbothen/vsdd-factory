<!-- Implementation rules for SOUL.md principles. SOUL owns "why", this file owns "how". -->

# Bash Error Handling Standards

These rules apply to all shell scripts, especially test assertions in `tests/ci-validation/`.

## Never suppress stderr with 2>/dev/null

Suppressing stderr makes tool crashes indistinguishable from "value not found." Capture stderr and check it:

```bash
# BAD — python3 crash reports "got ''" instead of the real error
actual="$(python3 -c "..." "$file" 2>/dev/null)" || actual=""

# GOOD — use _run_with_stderr_guard from assertions.sh
_run_with_stderr_guard "$desc" actual python3 -c "..." "$file" || return

# GOOD — manual stderr guard when not in assertion context
local stderr_file
stderr_file="$(mktemp)"
actual="$(some_tool arg 2>"$stderr_file")" || {
  if [[ -s "$stderr_file" ]]; then
    echo "ERROR: tool failed: $(cat "$stderr_file")" >&2
    rm -f "$stderr_file"
    exit 1
  fi
}
rm -f "$stderr_file"
```

## Never use eval in shell helpers

`eval "$cmd"` is a command injection vector (CWE-78). Use `"$@"` parameter expansion:

```bash
# BAD
output="$(eval "$cmd" 2>&1)"

# GOOD — pass command and args as separate parameters
my_function() {
  local desc="$1"; shift
  local output
  output="$("$@" 2>&1)" || true
}
```

## Tool guard pattern for justfile recipes

Every recipe that depends on an optional tool must check availability before running:

```just
recipe-name:
    #!/usr/bin/env bash
    set -euo pipefail
    if ! command -v tool-name &>/dev/null; then
        echo "tool-name not installed. Run 'just setup' or 'cargo install tool-name --locked'"
        exit 1
    fi
    tool-name actual-command
```

## Test prerequisite checks

Every test file must verify its tool dependencies at the top, before any assertions:

```bash
if ! command -v required-tool &>/dev/null; then
  echo "ERROR: required-tool is required but not found." >&2
  echo "Install: cargo install required-tool" >&2
  exit 1
fi
```

## Negative assertion semantics

Tests asserting something does NOT exist must first verify the search tool works. A tool crash must not be interpreted as "pattern absent."

## Negative assertions must verify the tool ran

Tests asserting something does NOT exist must first verify the search tool works. A tool crash must not be interpreted as "pattern absent." This is the most common source of false-pass bugs in the test suite.

```bash
# BAD — yq crash → empty → "no nightly found" → false pass
job_text="$(yq eval ".jobs.${job}" "$file" 2>/dev/null)" || job_text=""

# GOOD — detect tool failure before interpreting result
yq_stderr="$(yq eval ".jobs.${job}" "$file" 2>&1 1>/dev/null)" || true
if [[ -n "$yq_stderr" ]]; then
  _fail "$desc" "yq failed: ${yq_stderr}"
  return
fi
```

## Use `grep -F` for literal string matching

`grep -q` without `-F` treats the pattern as regex. The `.` in `Cargo.lock` matches any character. Use `-F` (fixed-string) when matching literals.

```bash
# BAD — "Cargo.lock" matches "CargoXlock"
grep -q "Cargo.lock" "$file"

# GOOD
grep -qF "Cargo.lock" "$file"
```

## Test headers must state accurate counts

Every test file header that claims a count (e.g., "11 tests") must match the actual TAP plan. Stale counts are caught in review every time.

## File path references must be validated

If CLAUDE.md, SOUL.md, or any doc references a file path, a structural test must assert that path exists on disk. Broken references are the #1 class of review finding across all PRs.

## Tag stderr suppressions with STDERR-EXEMPT

Every `2>/dev/null` in assertion code must carry a `# STDERR-EXEMPT: <rationale>` tag. A self-check test enforces this — untagged instances fail the test suite.

## CI parity

`just ci` must run the same commands as `.github/workflows/ci.yml`. `just check` is the fast pre-commit subset (fmt + clippy + deny). Document any intentional divergence in both the justfile comments and the PR description.
