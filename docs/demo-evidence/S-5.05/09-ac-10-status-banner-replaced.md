# AC-10 — Status banner replaced

**AC statement:** Skeleton Status banner (lines 3-7) replaced with
non-skeleton banner (e.g., "**Audience:** factory operators upgrading
from v0.79.x. Last updated YYYY-MM-DD.").

**Evidence type:** file snippet + grep count

## New banner content (lines 1-8)

```markdown
# Migrating from v0.79.x to v1.0

> **Audience:** factory operators upgrading from v0.79.x. Last updated 2026-04-29.

Operators upgrading an existing v0.79.x factory to v1.0 read this guide.
It walks through what changed, what the upgrade requires, what works
out-of-the-box, what doesn't yet on Windows, and how to roll back if
something goes wrong.
```

## Verification command

```
grep -c "^> \*\*Status:\*\* skeleton" docs/guide/migrating-from-0.79.md
```

**Result:** `0`

## Commentary

The old skeleton banner read:
```
> **Status:** skeleton — see TODO(S-5.5) markers for outstanding sections.
```

It has been replaced with the Audience + Last updated banner. The skeleton
text is gone; the S-5.07 gate grep for `TODO(S-` will not match this line.
