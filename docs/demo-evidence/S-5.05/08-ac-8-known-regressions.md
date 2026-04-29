# AC-8 — Known regressions

**AC statement:** Section "Known regressions" populated (general section).
"Known regressions (v1.0.0-beta.1)" is PRE-FILLED by S-0.05 — populate
the general section only.

**Evidence type:** file snippet (2 sections)

## General "Known regressions" section (lines 168-179)

```markdown
## Known regressions

No blocking regressions were identified in the S-2.07 regression sweep.
The three correctness bugs found during the beta period (exec_subprocess
result envelope offset, empty plugin_root, empty env_view) were all
resolved before the v1.0.0 release; see "Known regressions (v1.0.0-beta.1)"
below for the full history.

If a regression surfaces in your factory after upgrading, open an issue
with your OS/arch, the hook name, and the relevant lines from
`dispatcher-internal.jsonl`. A workaround of pinning to `0.79.4` and
rolling back is available; see the Rollback section.
```

## PRE-FILLED "Known regressions (v1.0.0-beta.1)" section (lines 285-292)

```markdown
## Known regressions (v1.0.0-beta.1)

None blocking the beta. S-2.7 ran the validation pass and surfaced three
correctness bugs in the adapter pipeline; all three were fixed in commit
`c121d07` and pinned by regression tests in
`plugins/vsdd-factory/tests/regression-v1.0.bats`. Brief history below
for operators upgrading older snapshots.
```

## Commentary

The general "Known regressions" section was a TODO block — now filled
with an accurate no-blocking-regressions statement cross-referencing the
PRE-FILLED beta.1 history section. The PRE-FILLED section (S-0.05 origin)
is confirmed present and unmodified.
