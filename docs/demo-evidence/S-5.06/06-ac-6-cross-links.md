# AC-6: Cross-links in v1.0-index.md and README.md

**AC statement:** Cross-linked from `docs/guide/v1.0-index.md` (under the "For operators"
table) and from README.md "v1.0 Factory Plugin Kit" section. Note: when adding the new
entry, also update the v1.0-Index description in README from "links the four below" to
"links the five below".

**Evidence type:** grep results + verbatim table row snippets

## Verification commands

```bash
# v1.0-index.md "For operators" table
grep -n "semver-commitment" /private/tmp/vsdd-S-5.06/docs/guide/v1.0-index.md

# README.md "v1.0 Factory Plugin Kit" section
grep -n "semver\|Semver" /private/tmp/vsdd-S-5.06/README.md

# README.md L261 count update
sed -n '261p' /private/tmp/vsdd-S-5.06/README.md
```

## Result — v1.0-index.md

```
19:| [Semver commitment](semver-commitment.md) | factory operators + plugin authors | When evaluating upgrade safety; ABI / format stability guarantees |
```

## Verbatim snippet — v1.0-index.md "For operators" table (L15-L21)

```markdown
## For operators

| Doc | Audience | When to read |
|-----|----------|--------------|
| [Migrating from 0.79.x](migrating-from-0.79.md) | factory operators upgrading | Before running `/plugin update` to 1.0.x |
| [Semver commitment](semver-commitment.md) | factory operators + plugin authors | When evaluating upgrade safety; ABI / format stability guarantees |
| [Observability sinks](observability-sinks.md) | operators tuning telemetry | When adding Datadog / Honeycomb / OTel exporters |
```

The row is placed under the "For operators" table as required by the AC adjudication:
operators benefit from semver awareness for upgrade safety; plugin authors also reference
this doc but typically arrive via "Authoring hooks" or "Porting" cross-links.

## Result — README.md

```
265:| [Semver Commitment](docs/guide/semver-commitment.md) | v1.0 stability guarantees — what's stable, what's not, breaking change policy. |
```

## Verbatim snippet — README.md "v1.0 Factory Plugin Kit" section (L257-L266)

```markdown
### v1.0 Factory Plugin Kit (in progress)

| Doc | Description |
|-----|-------------|
| [v1.0 Index](docs/guide/v1.0-index.md) | Landing page for the v1.0 docs set; links the five below. |
| [Authoring Hooks](docs/guide/authoring-hooks.md) | Writing WASM hooks against the v1.0 SDK — skeleton, populated as Phase 1–3 stories ship. |
| [Porting Bash Hooks to WASM](docs/guide/porting-bash-hook-to-wasm.md) | Walkthrough for porting an existing v0.79.x hook — skeleton, populated as Phase 3 ports land. |
| [Migrating from 0.79.x](docs/guide/migrating-from-0.79.md) | Operator-facing upgrade guide — skeleton, finalized in S-5.5. |
| [Semver Commitment](docs/guide/semver-commitment.md) | v1.0 stability guarantees — what's stable, what's not, breaking change policy. |
| [Observability Sinks](docs/guide/observability-sinks.md) | Multi-backend telemetry config — skeleton, populated as Phase 1 + 4 sink drivers ship. |
```

## L261 count update confirmed

README.md line 261 reads:
```
| [v1.0 Index](docs/guide/v1.0-index.md) | Landing page for the v1.0 docs set; links the five below. |
```

The text "links the five below" confirms the count was updated from "four" to "five"
(Task 8 in the story). This satisfies bats test 18.

## Commentary

All three cross-link requirements are satisfied:
1. `v1.0-index.md` "For operators" table has the semver-commitment.md row.
2. README.md "v1.0 Factory Plugin Kit" table has the Semver Commitment row.
3. README.md L261 reads "links the five below" (not "four").
