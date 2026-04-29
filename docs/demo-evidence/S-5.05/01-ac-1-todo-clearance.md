# AC-1 — TODO marker clearance

**AC statement:** `docs/guide/migrating-from-0.79.md` fully populated
(no `TODO(S-5.5)` markers remaining; S-5.07 gate greps for any `TODO(S-` pattern).

**Evidence type:** grep count

## Verification commands

```
grep -c "TODO(S-5.5)" docs/guide/migrating-from-0.79.md
```

**Result:** `0`

```
grep -c "TODO(S-" docs/guide/migrating-from-0.79.md
```

**Result:** `0`

## Commentary

Both forms return 0. The S-5.07 release gate (`grep -r "TODO(S-"
docs/guide/`) will pass. All 10 skeleton TODO blocks replaced with prose.
