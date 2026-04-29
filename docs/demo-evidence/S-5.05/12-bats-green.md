# Bats — 17/17 PASS

**Evidence type:** full bats invocation output

## Invocation

```
cd /private/tmp/vsdd-S-5.05
bats plugins/vsdd-factory/tests/docs-completeness/s-5.05-migration-guide.bats
```

## Output

```
1..17
ok 1 AC-1: migrating-from-0.79.md has zero TODO(S-5.5) markers
ok 2 AC-2: 'What changed' section has no TODO markers and has prose
ok 3 AC-3: 'Why v1.0' section has no TODO markers and has prose
ok 4 AC-4a: 'Prerequisites' section has no TODO markers and has prose
ok 5 AC-4b: 'Upgrade procedure' section has no TODO markers and has prose
ok 6 AC-4c: 'Verification checklist' section has no TODO markers and has prose
ok 7 AC-4d: 'Rollback' section has no TODO markers and has prose
ok 8 AC-4e: guide references custom hooks.json migration (EC-001 coverage)
ok 9 AC-5a: 'Observability migration' section has no TODO markers and has prose
ok 10 AC-5b: 'Regenerating hooks-registry.toml' pre-filled section is preserved
ok 11 AC-6: 'Windows-specific notes' section has no TODO markers and has prose
ok 12 AC-7: 'Troubleshooting' section has at least 5 distinct issues
ok 13 AC-8a: 'Known regressions' section has no TODO markers and has prose
ok 14 AC-8b: 'Known regressions (v1.0.0-beta.1)' pre-filled section is preserved
ok 15 AC-10: Status banner no longer contains 'skeleton' text
ok 16 Task-16: README no longer says 'skeleton, finalized in S-5.5'
ok 17 Task-17: v1.0-index.md contains no bare 'S-5.7' references
```

**Result: 17/17 PASS — no failures.**
