# AC-4: Section — Breaking change policy (major version bump, migration guide required)

**AC statement:** Section: Breaking change policy (major version bump, migration guide
required).

**Evidence type:** section heading grep + verbatim rules snippet

## Verification command

```
grep -n "Breaking change policy\|major version bump\|migration guide" \
  /private/tmp/vsdd-S-5.06/docs/guide/semver-commitment.md
```

**Result:**
```
100:## Breaking change policy
109:### Rules
111:1. **Major version bump required.** No breaking change to any stable surface may
114:2. **Migration guide required.** Every major version release MUST include a
```

## Verbatim snippet — breaking change policy section (L100-L141)

```markdown
## Breaking change policy

A **breaking change** is any change to a stable surface listed above that:

- Removes or renames a stable API, type, constant, or file format field
- Changes the semantics of an existing stable API in a backward-incompatible way
- Changes `HOST_ABI_VERSION` in either `factory-dispatcher` or `vsdd_hook_sdk`
- Changes `schema_version` on `hooks-registry.toml` or `observability-config.toml`

### Rules

1. **Major version bump required.** No breaking change to any stable surface may
   ship without incrementing the major version (e.g., `1.x.y` → `2.0.0`).

2. **Migration guide required.** Every major version release MUST include a
   migration guide documenting what changed, why it changed, the step-by-step
   upgrade procedure, and any rollback steps. See
   [Migrating from 0.79.x](migrating-from-0.79.md) as the model for format and
   voice.

3. **Coordinated bump required for ABI changes.** A change to `HOST_ABI_VERSION`
   in the dispatcher must be matched by a simultaneous bump in `vsdd_hook_sdk`
   (BC-2.01.003, EC-001). A mismatch between the two constants is a major-version
   event and would block the release gate.

4. **Deprecation before removal.** Stable surfaces are deprecated in a minor
   release (with a `#[deprecated]` annotation and/or a doc notice) before removal
   in the next major version. Exception: security fixes may require immediate
   removal.

5. **No breaking changes in patch releases.** Patch releases (`x.y.Z`) are
   reserved for bug fixes and security patches. Non-breaking additions may ship
   in minor releases (`x.Y.0`).
```

## Both required elements confirmed

| Required element | Present? | File line |
|-----------------|---------|-----------|
| Major version bump required | Yes | L111 — "Major version bump required. No breaking change... may ship without incrementing the major version" |
| Migration guide required | Yes | L114 — "Migration guide required. Every major version release MUST include a migration guide" |

## Commentary

The section opens by defining what constitutes a breaking change (4 bullet points), then
enumerates 5 rules for handling breaking changes. Rule 3 is particularly important: it
requires a coordinated bump of `HOST_ABI_VERSION` in both crates simultaneously, citing
BC-2.01.003 as the enforcement contract. The section also explicitly defines what does
NOT constitute a breaking change, providing clear guidance for maintainers.
