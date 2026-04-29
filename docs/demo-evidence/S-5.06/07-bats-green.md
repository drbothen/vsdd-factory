# Bats test suite — 18/18 PASS

**Evidence type:** full bats invocation output

## Invocation

```bash
cd /private/tmp/vsdd-S-5.06
bats plugins/vsdd-factory/tests/docs-completeness/s-5.06-semver-commitment.bats
```

## Full output

```
1..18
ok 1 AC-1: docs/guide/semver-commitment.md exists
ok 2 AC-1: semver-commitment.md has at least 100 non-blank lines
ok 3 AC-2: stable surface section heading exists
ok 4 AC-2: stable surface lists hook-sdk ABI
ok 5 AC-2: stable surface lists registry schema
ok 6 AC-2: stable surface lists hooks.json format
ok 7 AC-2: stable surface lists event type namespaces
ok 8 AC-3: unstable surface section heading exists
ok 9 AC-3: unstable surface lists internal JSONL format
ok 10 AC-3: unstable surface lists dispatcher invocation args
ok 11 AC-4: breaking change policy section heading exists
ok 12 AC-4: breaking change policy mentions major version bump
ok 13 AC-4: breaking change policy mentions migration guide
ok 14 AC-5: plugin backward compat section heading exists
ok 15 AC-5: plugin compat mentions HOST_ABI_VERSION
ok 16 AC-6: v1.0-index.md For operators table contains semver-commitment.md row
ok 17 AC-6: README.md v1.0 Factory Plugin Kit section contains semver-commitment.md row
ok 18 AC-6: README.md L261 reads 'links the five below'
```

**Result: 18/18 PASS — 0 failures**

## AC coverage by test

| Test # | AC | Assertion |
|--------|----|-----------|
| 1 | AC-1 | File exists at `docs/guide/semver-commitment.md` |
| 2 | AC-1 | File has ≥100 non-blank lines (actual: 204 total lines) |
| 3 | AC-2 | Stable surface section heading present |
| 4 | AC-2 | hook-sdk ABI listed in stable surface |
| 5 | AC-2 | Registry schema listed in stable surface |
| 6 | AC-2 | hooks.json format listed in stable surface |
| 7 | AC-2 | Event type namespaces listed in stable surface |
| 8 | AC-3 | Unstable surface section heading present |
| 9 | AC-3 | Internal JSONL format listed in unstable surface |
| 10 | AC-3 | Dispatcher invocation args listed in unstable surface |
| 11 | AC-4 | Breaking change policy section heading present |
| 12 | AC-4 | Major version bump requirement stated |
| 13 | AC-4 | Migration guide requirement stated |
| 14 | AC-5 | Plugin backward compat section heading present |
| 15 | AC-5 | HOST_ABI_VERSION mentioned in plugin compat section |
| 16 | AC-6 | v1.0-index.md "For operators" table has semver-commitment.md row |
| 17 | AC-6 | README.md "v1.0 Factory Plugin Kit" section has semver-commitment.md row |
| 18 | AC-6 | README.md L261 text is "links the five below" |
