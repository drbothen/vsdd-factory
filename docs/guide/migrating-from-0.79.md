# Migrating from v0.79.x to v1.0

> **Status:** skeleton — filled in by S-5.5 (operator-facing migration
> guide) and validated by S-5.7 (gate 16) against at least one real
> 0.79.x factory that isn't vsdd-factory itself. Greppable `TODO(S-X.Y)`
> markers gate the 1.0.0 release per S-5.7 acceptance criteria.

Operators upgrading an existing v0.79.x factory to v1.0 read this guide.
It walks through what changed, what the upgrade requires, what works
out-of-the-box, what doesn't yet on Windows, and how to roll back if
something goes wrong.

## What changed

<!-- TODO(S-5.5): the headline list — Rust dispatcher binary replaces
     hook-by-hook bash dispatch; WASM plugins replace bash scripts (with
     legacy-bash-adapter as a compatibility layer on Linux/macOS); new
     activation step required after install; observability config moves
     to observability-config.toml; per-platform binaries committed to
     the plugin package. -->

## Why v1.0

<!-- TODO(S-5.5): the matcher-bug motivation, cross-platform support,
     observability flexibility (multi-instance multi-backend sinks),
     capability-sandboxed plugins. Keep this paragraph honest — the
     immediate driver was unfixable upstream behavior, not a roadmap
     item. -->

## Prerequisites

<!-- TODO(S-5.5): Claude Code minimum version, git, jq. NOT required:
     Python (never was), Node (never was). Windows users need git-bash
     for the hooks still routed through legacy-bash-adapter. -->

## Upgrade procedure

<!-- TODO(S-5.5): the sequence — `/plugin update vsdd-factory@vsdd-factory`,
     `/vsdd-factory:activate` (NEW required step), full Claude Code
     session restart, `/vsdd-factory:factory-health` to verify. -->

## Verification checklist

<!-- TODO(S-5.5): how an operator confirms the upgrade landed — make a
     test commit and see `commit.made` event flow through, confirm
     existing dashboards still render (file-sink default preserves
     0.79.x event JSONL behavior), check
     `.factory/logs/dispatcher-internal.jsonl` exists and has entries. -->

## Observability migration

<!-- TODO(S-5.5): default config matches 0.79.x file-sink behavior so
     existing Grafana dashboards keep working with no changes. To add
     Datadog / Honeycomb / OTel-grpc, write `observability-config.toml`.
     Zero-disk mode is opt-in (disable file sink). -->

## Windows-specific notes

<!-- TODO(S-5.5): native WASM hooks that work on Windows in 1.0
     (capture-commit-activity, capture-pr-activity, block-ai-attribution,
     plus emit_event as a host function). Other 26 hooks remain on the
     legacy-bash-adapter and require git-bash on Windows. Frame
     expectation: more native ports will follow post-1.0. -->

## Known regressions

<!-- TODO(S-5.5): empty if S-2.7's regression sweep finds none. If any
     emerge from beta/rc shakedown, document them here with workarounds
     and tracked-issue links. -->

## Rollback

<!-- TODO(S-5.5): explicit rollback steps if 1.0 misbehaves on an
     operator's factory — pin to 0.79.4, deactivate the dispatcher,
     restart session. Cover the case where v1.0 already wrote new
     observability-config.toml. -->

## Troubleshooting

<!-- TODO(S-5.5): common symptoms and fixes — dispatcher not firing
     (activation step missed?), Datadog 401 (sink config / API key?),
     legacy-bash-adapter "command not found" (git-bash missing on
     Windows?), event field schema drift (HOST_ABI_VERSION skew?). -->

## Regenerating `hooks-registry.toml`

The v1.0 dispatcher reads `plugins/vsdd-factory/hooks-registry.toml`
to decide which hooks fire on which events. During the v0.79.x → v1.0
migration the file is produced by a generator that reads the historical
bash-hook inventory at `git show 7b4b774^:plugins/vsdd-factory/hooks/hooks.json`
and emits one `[[hooks]]` entry per bash hook, all routed through
`legacy-bash-adapter.wasm`.

Run the generator from the repo root:

```bash
scripts/generate-registry-from-hooks-json.sh
```

The script is idempotent — re-running it on an unchanged input
produces byte-identical output. CI re-runs it on every push and fails
the build if `git diff plugins/vsdd-factory/hooks-registry.toml` is
non-empty.

**When to re-run:**

- During this migration, almost never. The bash hook inventory is
  frozen at the historical commit; ongoing maintenance edits the
  generated TOML directly. The generator exists so the *initial*
  conversion is auditable, not so it runs continuously.
- If a bash hook is added or removed (rare during migration), update
  `git show 7b4b774^:plugins/vsdd-factory/hooks/hooks.json` to match
  reality (or pass an explicit hooks.json path argument), then
  re-run the generator and review the diff.
- After 1.0.0 ships, the generator is retired entirely;
  `hooks-registry.toml` becomes the human-edited source of truth for
  every native-WASM port (S-2.5+).

**What it can't fix:** entries whose underlying bash script no longer
exists. The generator hard-fails on script-without-entry or
entry-without-script — those are operator-resolved drift, not
generator-resolved.

## Known regressions (v1.0.0-beta.1)

None blocking the beta. S-2.7 ran the validation pass and surfaced three
correctness bugs in the adapter pipeline; all three were fixed in commit
`c121d07` and pinned by regression tests in
`plugins/vsdd-factory/tests/regression-v1.0.bats`. Brief history below
for operators upgrading older snapshots.

### Resolved during S-2.7

- **`exec_subprocess` result envelope written to wasm offset 0.** Host
  wrote the envelope to linear-memory offset 0 and reported `result_ptr
  = 0`; the SDK's `read_owned_bytes` short-circuits `ptr == 0` to an
  empty `Vec`, so every bash hook returned `HookResult::Error`. Fixed
  by switching to a guest-pre-allocated result buffer (same pattern as
  `session_id` / read-string host fns). FFI signature changed; SDK +
  host + StoreData-typed handler all updated together. See HOST_ABI.md
  for the new shape.
- **`HostContext.plugin_root` was empty.** main.rs read
  `${CLAUDE_PLUGIN_ROOT}` to find the registry but never wrote it into
  the host context, so `host::plugin_root()` returned the empty string
  and relative `script_path` values resolved against bash's cwd
  instead of the plugin root. Fixed by populating `plugin_root` at
  startup.
- **`HostContext.env_view` was empty.** The host's `exec_subprocess`
  reads names from `ctx.env_view` (a `HashMap`) so the per-plugin
  `env_allow` capability gate can filter without a syscall per call;
  main.rs never populated the map. Fixed by projecting the
  dispatcher's process env into `env_view` once at startup.

### Open follow-ups (non-blocking)

- **dispatcher-internal log does not capture wasm plugin stderr.**
  `crates/factory-dispatcher/src/invoke.rs` allocates a
  `MemoryOutputPipe` for plugin stderr but never reads or forwards it.
  When a plugin panics or returns an error message via stderr, the
  failure mode is invisible to the operator beyond `exit_code:1` on
  `plugin.completed`. Trivial fix; tracked for post-beta.

## Platform validation status

| Platform                         | Bats run        | Dispatcher smoke                       | Notes |
|----------------------------------|-----------------|----------------------------------------|-------|
| macOS arm64 (darwin-aarch64)     | 1245/1245 PASS  | Adapter round-trip verified end-to-end | Validation host for S-2.7 |
| Linux x64                        | CI (`build-dispatcher` matrix) | Not exercised in S-2.7   | Bash hooks are POSIX; parity with macOS expected |
| Windows x64                      | Not run         | Not run                                | Validated via CI build-dispatcher matrix only; full bats run not yet executed on Windows because the bash hooks themselves are POSIX. v1.0.0-beta.1 leaves Windows partial as documented scope. |
