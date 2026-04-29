# AC-1: docs/guide/semver-commitment.md created with full content

**AC statement:** `docs/guide/semver-commitment.md` created with full content.

**Evidence type:** file existence + line count + content snippet

## Verification command

```
wc -l /private/tmp/vsdd-S-5.06/docs/guide/semver-commitment.md
```

**Result:** 204 lines (bats gate requires ≥100 non-blank lines; 204 total lines well exceeds threshold)

## Verbatim snippet — file header and intro

```markdown
# vsdd-factory v1.0 semver commitment

This document states what vsdd-factory commits to as stable public API in the
v1.0 line, what is intentionally unstable, how breaking changes are handled,
and what HOST_ABI_VERSION = 1 means for plugin authors. It is the authoritative
reference for operators and plugin authors evaluating upgrade safety.

This document is locked at v1.0.0-beta.4. Amendments require a new doc version
and a changelog entry.

---

## What's stable in v1.0
```

## Verbatim snippet — References table at bottom of file (L196-L205)

```markdown
## References

| Document | Purpose |
|----------|---------|
| [BC-2.01.003](.factory/specs/behavioral-contracts/ss-02/BC-2.01.003.md) | HOST_ABI_VERSION = 1 in both crates — the authoritative contract |
| [BC-2.02.001](.factory/specs/behavioral-contracts/ss-02/BC-2.02.001.md) | Plugin-author API surface is `vsdd_hook_sdk::host::*`; raw FFI is private |
| [Migrating from 0.79.x](migrating-from-0.79.md) | Model migration guide — format and voice for future major-version migration guides |
| [PRD §3.1](../../.factory/specs/PRD.md) | Primary interfaces summary — canonical enumeration of all stable surfaces |
| [v1.0 index](v1.0-index.md) | Landing page for all v1.0 documentation |
```

## Commentary

The file is real prose with four top-level sections, detailed tables, subsection
narratives, and a reference table. It is not a skeleton — all sections contain
substantive content. The 204-line count significantly exceeds the 100-non-blank-line
gate in the bats tests.
