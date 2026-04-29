# AC-4 — Prerequisites, Upgrade procedure, Verification checklist, Rollback

**AC statement:** Sections "Prerequisites", "Upgrade procedure",
"Verification checklist", "Rollback" populated. Upgrade procedure includes
guidance for users with custom hooks.json entries (EC-001).

**Evidence type:** file snippets (4 sections)

## Prerequisites (lines 54-70)

```markdown
## Prerequisites

Before running the upgrade procedure, confirm:

- **Claude Code** with plugin support enabled (any version shipping after
  the v1.0 plugin ABI landed; if unsure, run `claude --version` and
  compare against the release notes).
- **git** in `$PATH` — the dispatcher and several hooks shell out to git.
- **jq** in `$PATH` — used by the legacy-bash-adapter and several bash hooks.

Not required: Python (never was a dependency), Node.js (never was a
dependency).
```

## Upgrade procedure — custom hooks.json coverage (EC-001) (lines 84-94)

```markdown
2. **Activate the dispatcher** — this step is new in v1.0 and required:
   ```
   /vsdd-factory:activate
   ```
   Activation registers the Rust dispatcher binary with Claude Code's hook
   system and writes the initial `hooks-registry.toml` if one does not
   exist. If you have **custom entries** in your `hooks.json` from v0.79.x,
   the activation step reads them and generates corresponding registry
   entries routed through `legacy-bash-adapter.wasm`. Review the generated
   `plugins/vsdd-factory/hooks-registry.toml` diff to confirm your custom
   hooks were captured.
```

## Verification checklist (lines 107-126)

```markdown
## Verification checklist

After the upgrade procedure completes, confirm the following:

- [ ] **Test commit fires hooks.** Make a test commit (e.g.,
  `git commit --allow-empty -m "test: verify v1.0 hook dispatch"`) and
  confirm the `commit.made` event appears in
  `.factory/logs/dispatcher-internal.jsonl`.
- [ ] **Existing dashboards render.** If you have Grafana or another tool
  reading the JSONL event log, confirm it still renders. The v1.0 file
  sink writes the same event schema as v0.79.x by default; no dashboard
  changes are required.
- [ ] **Dispatcher log has entries.** Open
  `.factory/logs/dispatcher-internal.jsonl` and confirm it exists and
  has recent entries (timestamps within the current session).
- [ ] **Factory health is green.** `/vsdd-factory:factory-health` reports
  all checks passing.
```

## Rollback (lines 182-206)

```markdown
## Rollback

If v1.0 misbehaves on your factory and you need to revert:

1. **Deactivate the dispatcher** to stop it from receiving hook events:
   ```
   /vsdd-factory:deactivate
   ```
2. **Pin to v0.79.4** (the last stable 0.79.x release):
   ```
   /plugin install vsdd-factory@vsdd-factory@0.79.4
   ```
3. **Restart your Claude Code session** to clear the v1.0 dispatcher
   process and load the v0.79.x hooks.
4. **Handle `observability-config.toml` if present.** If v1.0 wrote an
   `observability-config.toml` to your factory root before you rolled back,
   that file is ignored by v0.79.x (which has no dispatcher to read it).
```

## Commentary

All four sections present with prose. EC-001 (custom hooks.json migration)
is explicitly covered in the Upgrade procedure step 2, instructing operators
to review the generated `hooks-registry.toml` diff to confirm custom entries
were captured.
