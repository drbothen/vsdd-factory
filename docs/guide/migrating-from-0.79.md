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

S-2.7 ran the validation pass against HEAD `0438b78` (Phase 2 complete:
S-2.1 + S-2.2 landed). Findings, in severity order:

### blocker: legacy-bash-adapter cannot round-trip `exec_subprocess` results

Every bash hook routed through the dispatcher currently fails inside the
adapter with `HostError::Other(-99)` (subprocess result decode failed).
The dispatcher reports `exit_code: 1` (`HookResult::Error`) for every
matched plugin and the bash side effects (e.g. `commit.made`,
`hook.block` events written by `bin/emit-event`) never happen — the bash
script never actually executes.

**Root cause:** the `vsdd::exec_subprocess` host implementation writes
the result envelope to wasm linear memory **offset 0** and sets the
SDK's out-pointer to `0`
([`crates/factory-dispatcher/src/host/exec_subprocess.rs:81-90`][1] and
the StoreData mirror at
[`crates/factory-dispatcher/src/invoke.rs:482-497`][2]). The SDK's
`read_owned_bytes` short-circuits `ptr == 0` to an empty `Vec`
([`crates/hook-sdk/src/host.rs:283-291`][3]), so
`decode_subprocess_result` parses a zero-length buffer and the SDK
returns `HostError::Other(-99)`. The adapter reports this as a hook
error and exits 1.

**Impact for v1.0.0-beta.1:** every gate hook routed through the adapter
silently no-ops at the protect-and-emit layer. Direct-bash invocation of
the same hook still works (proven by the 1245/1245 bats baseline), so
operators who continue running `hooks.json` (the v0.79.x dispatcher
shape) are unaffected; operators who switch to the v1.0 dispatcher are
broken until the bug is fixed.

**Fix path:** the host needs to either (a) call a guest-exported
allocator to allocate a buffer in linear memory and write the envelope
into that buffer (typical wasmtime pattern), or (b) write into a
pre-arranged scratch region that the SDK knows about and reads from. The
current "offset 0" write also silently clobbers whatever data the wasm
module had at that address — it's a memory-safety problem in addition
to a correctness one.

**Tracking:** pinned in `plugins/vsdd-factory/tests/regression-v1.0.bats`
under the `[BUG]` test names. When the fix lands, flip those assertions
and remove the `TODO(S-2.7-fixup)` markers.

### important: dispatcher-internal log does not capture wasm plugin stderr

`crates/factory-dispatcher/src/invoke.rs:104-110` allocates a
`MemoryOutputPipe` for the wasm plugin's stderr but never reads or
forwards it. When a plugin (including the legacy adapter) panics or
returns an error message via stderr, the failure mode is invisible to
the operator beyond the `exit_code: 1` field on `plugin.completed`.
Combined with the blocker above, this made the legacy-adapter bug much
harder to diagnose than necessary.

**Fix path:** when a plugin's exit_code is non-zero or it traps,
include the stderr buffer (truncated) on the `plugin.completed` /
`plugin.crashed` event. Trivial change once exec_subprocess is fixed.

### suggestion: bench script latency numbers are not yet meaningful

`benches/legacy-adapter-latency.sh` runs end-to-end and reports
adapter / direct ratios in the 50%-620% range. **These numbers are
not a fair indicator of the 30% adapter-overhead budget** because the
adapter currently early-exits in `exec_subprocess` rather than
launching bash — it pays the wasm instantiation cost but skips the
bash-subprocess cost, while the "adapter" total includes every other
plugin matched by the same event. Re-run after the blocker is fixed
to get a meaningful budget check.

## Platform validation status

| Platform | Bats run | Dispatcher smoke | Notes |
|----------|----------|------------------|-------|
| macOS arm64 (darwin-aarch64) | 1245/1245 PASS | Routes plugins; legacy-adapter offset-0 bug surfaces | Validation host for S-2.7 |
| Linux x64 | Tested via CI (`build-dispatcher` matrix) | Not exercised in S-2.7 | Bash hooks are POSIX, expected parity with macOS |
| Windows x64 | Not run | Not run | Validated via CI build-dispatcher matrix only; full bats run not yet executed on Windows because the bash hooks themselves are POSIX. v1.0.0-beta.1 leaves Windows partial as a documented note per the spec's loose Windows scope. |

[1]: ../../crates/factory-dispatcher/src/host/exec_subprocess.rs
[2]: ../../crates/factory-dispatcher/src/invoke.rs
[3]: ../../crates/hook-sdk/src/host.rs
