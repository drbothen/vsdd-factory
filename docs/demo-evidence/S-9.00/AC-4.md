---
story_id: S-9.00
ac: AC-4
title: Measurement script committed
---

# AC-4: Measurement script committed at canonical path

**Statement:** Shell script that reproduces the AC-1 measurement committed to `.factory/measurements/measure-bundle-sizes.sh`. Must: (a) accept bundle directory as `$1`, (b) emit JSON, (c) use `wc -c < <file>` (portable), (d) `#!/usr/bin/env bash` with `set -euo pipefail`.

## Command

```bash
# Verify script exists, is executable, and shebang is correct
ls -la .factory/measurements/measure-bundle-sizes.sh
head -3 .factory/measurements/measure-bundle-sizes.sh

# Verify set -euo pipefail
grep "set -euo pipefail" .factory/measurements/measure-bundle-sizes.sh

# Verify wc -c usage (not du -sb)
grep "wc -c" .factory/measurements/measure-bundle-sizes.sh | head -5
```

## Output

```
-rwxr-xr-x  1 ...  measure-bundle-sizes.sh

#!/usr/bin/env bash
# measure-bundle-sizes.sh — S-9.00 reproducible WASM bundle size measurement

set -euo pipefail

DISPATCHER_BYTES=$(LC_ALL=C wc -c < "$DISPATCHER_BINARY" | tr -d ' \t\n')
ALL_WASM_BYTES=$((ALL_WASM_BYTES + sz))   # sz=$(LC_ALL=C wc -c < "$f" | tr -d ' \t\n')
sz=$(LC_ALL=C wc -c < "$wasm_file" | tr -d ' \t\n')
```

## Script Properties Verified

| Property | Expected | Actual |
|----------|----------|--------|
| Shebang | `#!/usr/bin/env bash` | Yes |
| `set -euo pipefail` | Present | Yes |
| Accepts `$1` as bundle dir | Yes | Yes |
| Emits JSON | Yes | Yes (via `printf`) |
| Byte-count method | `wc -c < <file>` | Yes |
| `du -sb` absent | True | Confirmed absent |
| `stat -c` absent | True | Confirmed absent (non-portable) |
| trap INT/TERM | Required (adversary pass 2) | Yes (`trap "..." EXIT INT TERM`) |
| portable mktemp | Required (adversary pass 2) | Yes (`mktemp "${TMPDIR:-/tmp}/hyperfine.XXXXXX"`) |
| Windows note | Documented | Git Bash + PowerShell fallback in comments |

## Bats Gate

```
$ bats plugins/vsdd-factory/tests/perf-baseline.bats
ok 4 S-9.00 AC-4: script is idempotent — two runs produce identical byte counts
```

Note: AC-4's bats test covers idempotency (two runs produce identical counts), which is a superset of the script-exists check. The script-exists assertion is part of every bats test in the suite (each begins with `[ -f "$SCRIPT" ]` + `[ -x "$SCRIPT" ]`).

## Verdict

PASS — Script committed to `.factory/measurements/measure-bundle-sizes.sh`. Shebang, `set -euo pipefail`, `wc -c` portability, JSON output, and `$1` argument handling all verified. Script is executable (`-x`). Trap updated to catch INT/TERM (adversary pass-2 fix MEDIUM-2). mktemp updated to portable form (adversary pass-2 fix MEDIUM-3). Bats test AC-4 passes.
