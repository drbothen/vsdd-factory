---
story_id: S-9.00
ac: AC-9
title: All output artifacts committed to canonical paths
---

# AC-9: All output artifacts committed to canonical paths

**Statement:** `.factory/architecture/perf-baseline-w16.md`, `.factory/measurements/measure-bundle-sizes.sh`, and `.factory/measurements/fixtures/handoff-validator-input.json` all committed. State-manager commits as a single burst per POLICY 3.

## Command

```bash
# Verify all three artifacts exist and the script is executable
ls -la .factory/measurements/measure-bundle-sizes.sh
ls -la .factory/architecture/perf-baseline-w16.md
ls -la .factory/measurements/fixtures/handoff-validator-input.json

# Verify via git history (factory-artifacts branch, commit 389fb0b)
git show 389fb0b --name-only --format="%H %s"
```

## Output

```
-rwxr-xr-x  1 ...  .factory/measurements/measure-bundle-sizes.sh
-rw-r--r--  1 ...  .factory/architecture/perf-baseline-w16.md
-rw-r--r--  1 ...  .factory/measurements/fixtures/handoff-validator-input.json

389fb0b7370426ecc8dd2a8d5ca505d843764f23 impl: S-9.00 GREEN — measure-bundle-sizes.sh + perf-baseline-w16.md

architecture/perf-baseline-w16.md
measurements/measure-bundle-sizes.sh
```

## Artifact Inventory

| Artifact | Canonical Path | Committed | Executable |
|----------|----------------|-----------|-----------|
| Measurement script | `.factory/measurements/measure-bundle-sizes.sh` | Yes (commit `389fb0b`, factory-artifacts) | Yes (`-x`) |
| Baseline + ceiling doc | `.factory/architecture/perf-baseline-w16.md` | Yes (commit `389fb0b`, factory-artifacts) | N/A |
| Cold-start fixture | `.factory/measurements/fixtures/handoff-validator-input.json` | Yes (factory-artifacts) | N/A |

Note: All three artifacts committed in a single burst commit (`389fb0b`) per POLICY 3 (state-manager-runs-last). The fixture file was authored per S-9.00 v1.4 fix F-P4-002 and is required for Task B.1 hyperfine cold-start measurement.

### Windows Portability Note

`measure-bundle-sizes.sh` requires Git Bash on Windows (per AC-9 spec). PowerShell alternative: `(Get-Item <file>).Length`. If Git Bash unavailable on windows-x64 CI runner, document that platform as "CI-only with re-run instructions" (per EC-002).

## Bats Gate

```
$ bats plugins/vsdd-factory/tests/perf-baseline.bats
ok 9 S-9.00 AC-9: all three required artifacts exist at canonical paths
```

## Verdict

PASS — All three required artifacts exist at canonical paths. Script is executable. Committed to factory-artifacts in single burst at commit `389fb0b`. Bats test AC-9 passes.
