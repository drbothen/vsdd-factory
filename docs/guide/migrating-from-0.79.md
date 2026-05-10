# Migrating from v0.79.x to v1.0

> **Audience:** factory operators upgrading from v0.79.x. Last updated 2026-05-04.

Operators upgrading an existing v0.79.x factory to v1.0 read this guide.
It walks through what changed, what the upgrade requires, what works
out-of-the-box, what doesn't yet on Windows, and how to roll back if
something goes wrong.

> **Important: marketplace change in v1.0.0-rc.7.** Through rc.6 the plugin
> was distributed via the `drbothen/vsdd-factory` marketplace. As of rc.7
> the marketplace was split into a separate repo: **`drbothen/claude-mp`**.
> If you're upgrading FROM v0.79.x and skipping straight past rc.6, you
> should add the new marketplace and install from it (commands below
> updated accordingly). If you're already on rc.6 or earlier with the old
> marketplace registered, run `/plugin marketplace remove vsdd-factory`
> first, then add `drbothen/claude-mp` per the commands below.

## What changed

v1.0 replaces the hook-by-hook bash dispatch model with a single Rust
dispatcher binary (`factory-dispatcher`) that reads a registry file and
routes events to WASM plugins. Key changes:

- **Rust dispatcher binary** replaces the per-hook bash dispatch chain.
  One process starts at session open, receives all Claude Code hook events,
  and routes them to the correct WASM plugin entry points.
- **WASM plugins** replace bash scripts as the hook execution unit.
  Plugins run in a capability-sandboxed WASM runtime (Wasmtime). Bash hooks
  from v0.79.x are not deleted — they are wrapped by `legacy-bash-adapter.wasm`,
  which invokes them via `exec_subprocess`, preserving existing behavior.
- **Activation step required.** After installing or updating the plugin, you
  must run `/vsdd-factory:activate` once to register the dispatcher with
  Claude Code's hook system. Without activation, hooks do not fire.
- **Observability config moves** from hardcoded file output to
  `observability-config.toml`. The default behavior (writing JSONL to
  `.factory/logs/`) matches v0.79.x with no config changes required.
- **Per-platform binaries** are committed to the plugin package.
  The dispatcher is a compiled Rust binary; the correct binary for your
  OS/arch is selected at activation time.

## Why v1.0

The immediate driver was an unfixable bug in the upstream bash dispatch
pipeline: the v0.79.x hook matcher silently dropped events under certain
terminal width conditions, causing hooks to not fire without any error
message. The bug lived in behavior baked into the Claude Code shell
integration layer and could not be patched from the plugin side.

Rather than ship a workaround, the decision was to move to a stable ABI.
v1.0 provides:

- **Reliable event delivery** via the Rust dispatcher's typed event
  deserialization — no more silent drops from bash string-matching bugs.
- **Cross-platform support** — the dispatcher compiles for darwin-aarch64,
  linux-x64, and windows-x64; bash-only hooks were not portable to Windows.
- **Observability flexibility** — multiple simultaneous sinks (file, Datadog,
  Honeycomb, OTel-grpc) via `observability-config.toml`, rather than the
  single hardcoded file path in v0.79.x.
- **Capability-sandboxed plugins** — WASM plugins declare which host
  capabilities they need; the runtime enforces the allow-list.

## Prerequisites

Before running the upgrade procedure, confirm:

- **Claude Code** with plugin support enabled (any version shipping after
  the v1.0 plugin ABI landed; if unsure, run `claude --version` and
  compare against the release notes).
- **git** in `$PATH` — the dispatcher and several hooks shell out to git.
- **jq** in `$PATH` — used by the legacy-bash-adapter and several bash hooks.

Not required: Python (never was a dependency), Node.js (never was a
dependency).

**Windows operators** additionally need **git-bash** installed and available
in the system `PATH`. The legacy-bash-adapter routes bash hook invocations
through git-bash on Windows; without it, those hooks will fail to execute.
See "Windows-specific notes" below for which hooks are affected.

## Upgrade procedure

Follow these steps in order. Do not skip the activation step — it is the
most common cause of "hooks not firing" reports.

1. **Add the new marketplace and install** from within a Claude Code session:
   ```
   /plugin marketplace remove vsdd-factory   # if you had the old marketplace
   /plugin marketplace add drbothen/claude-mp
   /plugin install vsdd-factory@claude-mp
   ```
   Wait for the install to complete and confirm the version number matches
   the v1.0 release you intend to run.

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

3. **Restart your Claude Code session** completely (close and reopen the
   terminal or IDE integration). The dispatcher process only starts fresh
   at session open; a restart is required for activation to take effect.

4. **Verify** using the health check:
   ```
   /vsdd-factory:factory-health
   ```
   All checks should report green. If any check is red, see the
   Troubleshooting section below.

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
- [ ] **Plugin version is correct.** The health check output includes the
  dispatcher version; confirm it matches the installed release.

## Observability migration

The default observability configuration in v1.0 writes the same JSONL
event log as v0.79.x — to `.factory/logs/dispatcher-internal.jsonl`.
If you have existing dashboards (Grafana, custom scripts, log-tailing
workflows) reading that file, they continue working with no changes.

To add additional sinks or modify behavior, create
`observability-config.toml` in your factory root. Example additions:

- **Datadog:** add a `[sink.datadog]` stanza with your API key and site.
- **Honeycomb:** add a `[sink.honeycomb]` stanza with your API key and
  dataset name.
- **OTel-grpc:** add a `[sink.otel_grpc]` stanza with your collector
  endpoint.

Multiple sinks can run simultaneously — the dispatcher fans events out
to all configured sinks. The file sink can be disabled entirely for
zero-disk mode by removing or commenting out the `[sink.file]` stanza,
though note that disabling it also disables the default dashboard feed.

## Windows-specific notes

v1.0 ships four hooks as native WASM that work on Windows without git-bash:

- `capture-commit-activity`
- `capture-pr-activity`
- `block-ai-attribution`
- `emit_event` (host function, not a hook per se, but used by the above)

The remaining 26 hooks in the legacy inventory are bash scripts routed
through `legacy-bash-adapter.wasm`. On Windows, the adapter invokes them
via git-bash. If git-bash is not installed or not in `PATH`, those hooks
will fail silently (the dispatcher logs `exit_code: 1` on the
`plugin.completed` event, visible in `dispatcher-internal.jsonl`).

More native WASM ports are planned for post-1.0 stories (S-2.5 and
onwards), which will reduce the git-bash dependency surface on Windows.
For now, Windows operators should install git-bash from
https://gitforwindows.org/ before running the upgrade procedure.

## Known regressions

No blocking regressions were identified in the S-2.07 regression sweep.
The three correctness bugs found during the beta period (exec_subprocess
result envelope offset, empty plugin_root, empty env_view) were all
resolved before the v1.0.0 release; see "Known regressions (v1.0.0-beta.1)"
below for the full history.

If a regression surfaces in your factory after upgrading, open an issue
with your OS/arch, the hook name, and the relevant lines from
`dispatcher-internal.jsonl`. A workaround of pinning to `0.79.4` and
rolling back is available; see the Rollback section.

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
   You can leave it in place safely; it will be picked up again if you
   re-upgrade to v1.0 later.

5. **Report the issue** so it can be fixed before your next upgrade attempt.
   Include your OS/arch, plugin version, and the contents of
   `dispatcher-internal.jsonl` from the failing session.

## Troubleshooting

- **Dispatcher not firing / hooks not running.** The most common cause is
  a missed activation step. Run `/vsdd-factory:activate` and then restart
  your Claude Code session completely. Confirm activation wrote entries to
  `.factory/logs/dispatcher-internal.jsonl` on the next hook-triggering
  action (e.g., make a commit).

- **Datadog 401 Unauthorized.** The Datadog sink is returning an auth
  error. Check `observability-config.toml` — confirm the `api_key` value
  is set and matches a valid Datadog API key for your account. Also
  confirm the `site` value matches your Datadog region (e.g.,
  `datadoghq.com` vs `datadoghq.eu`). The dispatcher logs the HTTP
  response code in `dispatcher-internal.jsonl` for each failed delivery.

- **`legacy-bash-adapter`: command not found (Windows).** The adapter is
  trying to invoke a bash hook but cannot find the bash interpreter. On
  Windows, the adapter uses git-bash. Install git-bash from
  https://gitforwindows.org/ and ensure it is in your system `PATH`.
  After installing, restart your Claude Code session.

- **Event field schema drift / unexpected fields missing.** If a hook
  or downstream consumer expects fields that are no longer present (or
  new fields that weren't there before), this indicates a `HOST_ABI_VERSION`
  skew — the installed dispatcher binary and the plugin's compiled WASM
  hooks are out of sync. Run `/plugin update vsdd-factory@claude-mp`
  to ensure both the dispatcher binary and the WASM artifacts are from
  the same release, then run `/vsdd-factory:activate` and restart.

- **Platform binary mismatch / dispatcher fails to start.** The dispatcher
  is a compiled Rust binary; one binary is included per supported platform
  in the plugin package. If the binary for your OS/arch is missing from the
  package (a packaging or release artifact issue, not an activation problem),
  the dispatcher will fail to start entirely rather than silently dropping
  events. Check `.factory/logs/dispatcher-internal.jsonl` for startup errors.
  If the binary is absent, open an issue with your OS/arch so the release
  can be patched.

## Regenerating `hooks-registry.toml` (historical, retired at 1.0.0 GA)

> **Status (1.0.0 GA):** The generator described below was **retired
> at 1.0.0 GA**. `plugins/vsdd-factory/hooks-registry.toml` is now the
> human-edited source of truth and the canonical place to add, remove,
> or rewire hooks. This section is preserved as historical migration
> guidance for operators reconstructing the v0.79.x → v1.0 path; it
> does not describe an active workflow.

The v1.0 dispatcher reads `plugins/vsdd-factory/hooks-registry.toml`
to decide which hooks fire on which events. During the v0.79.x → v1.0
migration the file was produced by a generator that read the historical
bash-hook inventory at `git show 7b4b774^:plugins/vsdd-factory/hooks/hooks.json`
and emitted one `[[hooks]]` entry per bash hook, all routed through
`legacy-bash-adapter.wasm`.

The generator was invoked from the repo root:

```bash
scripts/generate-registry-from-hooks-json.sh
```

The script was idempotent — re-running it on an unchanged input
produced byte-identical output. CI re-ran it on every push and failed
the build if `git diff plugins/vsdd-factory/hooks-registry.toml` was
non-empty.

**When it was used:**

- During the migration window, rarely. The bash hook inventory was
  frozen at the historical commit; ongoing maintenance edited the
  generated TOML directly. The generator existed so the *initial*
  conversion was auditable, not so it would run continuously.
- If a bash hook was added or removed (rare during migration),
  `git show 7b4b774^:plugins/vsdd-factory/hooks/hooks.json` was
  updated to match reality (or an explicit hooks.json path was passed
  as argument), then the generator was re-run and the diff reviewed.
- After 1.0.0 shipped, the generator was retired entirely;
  `hooks-registry.toml` became the human-edited source of truth for
  every native-WASM port (S-2.5+).

**What it couldn't fix:** entries whose underlying bash script no
longer existed. The generator hard-failed on script-without-entry or
entry-without-script — those were operator-resolved drift, not
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
