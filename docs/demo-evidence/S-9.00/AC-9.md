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

# Verify via git history (factory-artifacts branch)
git -C .factory log --oneline -5
```

## Output

```
-rwxr-xr-x  1 ...  .factory/measurements/measure-bundle-sizes.sh
-rw-r--r--  1 ...  .factory/architecture/perf-baseline-w16.md
-rw-r--r--  1 ...  .factory/measurements/fixtures/handoff-validator-input.json
```

## Artifact Inventory

| Artifact | Canonical Path | Committed | Executable |
|----------|----------------|-----------|-----------|
| Measurement script | `.factory/measurements/measure-bundle-sizes.sh` | Yes (factory-artifacts) | Yes (`-x`) |
| Baseline + ceiling doc | `.factory/architecture/perf-baseline-w16.md` | Yes (factory-artifacts) | N/A |
| Cold-start fixture | `.factory/measurements/fixtures/handoff-validator-input.json` | Yes (factory-artifacts) | N/A |

Note: Initial implementation committed in a single burst per POLICY 3 (state-manager-runs-last). Adversary pass-1 and pass-2 fix-bursts applied subsequent corrections to the script and baseline doc on factory-artifacts; all artifacts remain at their canonical paths.

### Windows Portability Note

`measure-bundle-sizes.sh` requires Git Bash on Windows (per AC-9 spec). PowerShell alternative: `(Get-Item <file>).Length`. If Git Bash unavailable on windows-x64 CI runner, document that platform as "CI-only with re-run instructions" (per EC-002).

## Bats Gate

```
$ bats plugins/vsdd-factory/tests/perf-baseline.bats
ok 9 S-9.00 AC-9: all three required artifacts exist at canonical paths
```

## Verdict

PASS — All three required artifacts exist at canonical paths. Script is executable. Committed to factory-artifacts branch. Bats test AC-9 passes.
