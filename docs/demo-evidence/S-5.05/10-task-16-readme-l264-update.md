# Task 16 — README.md L264 description update

**Task statement:** Update README.md line ~264 description: replace
'skeleton, finalized in S-5.5' with post-shipping wording.

**Evidence type:** before/after diff

## Before (HEAD~1 — commit de1aa09)

```
| [Migrating from 0.79.x](docs/guide/migrating-from-0.79.md) | Operator-facing upgrade guide — skeleton, finalized in S-5.5. |
```

## After (HEAD — commit ae860b0)

```
| [Migrating from 0.79.x](docs/guide/migrating-from-0.79.md) | Operator-facing upgrade guide for v0.79.x → v1.0 dispatcher migration. |
```

## Verification command

```
grep "Migrating from 0.79" README.md
```

**Result:**
```
| [Migrating from 0.79.x](docs/guide/migrating-from-0.79.md) | Operator-facing upgrade guide for v0.79.x → v1.0 dispatcher migration. |
```

## Commentary

'skeleton, finalized in S-5.5' removed. Description now reflects the
completed state. This is Task 16's canonical fix (S-5.06 Task 9 ownership
removed per F-S5.05-P02-003; S-5.05 is the sole owner).
