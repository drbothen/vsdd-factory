# Porting a bash hook to WASM (v1.0)

> **Status:** skeleton — filled in by stories S-3.1, S-3.2, S-3.3.
> Section bodies are deliberately empty until the story that ports the
> first reference hook lands. Greppable `TODO(S-X.Y)` markers gate the
> 1.0.0 release per S-5.7 acceptance criteria.

The v1.0 dispatcher runs WASM hooks natively but also keeps the existing
bash hooks working through the `legacy-bash-adapter` plugin (Linux/macOS
only). Porting a bash hook to a native WASM module unlocks Windows
parity, faster dispatch, and stricter capability sandboxing. This guide
walks through that port for an existing v0.79.x hook.

## Assessment checklist

<!-- TODO(S-3.1): the questions to answer before starting a port — what
     does the hook read from stdin, what side effects does it have, what
     subprocesses does it shell out to, what file reads does it perform,
     does it block or warn-only. Output: a port-effort estimate (S/M/L). -->

## Common patterns

<!-- TODO(S-3.1): map the typical bash idioms (jq queries on stdin,
     conditional exit codes, emit-event invocation, file path checks)
     to their WASM-SDK equivalents. -->

## Capturing stdin

<!-- TODO(S-3.1): show how Claude Code's hook stdin (a JSON envelope)
     reaches a WASM hook — the SDK macro deserializes into a typed
     event; no manual jq required. -->

## Emitting events

<!-- TODO(S-3.4): show the `emit_event` host function as the new path
     for what bash hooks did via `bin/emit-event`. Include the field-set
     contract (event name, code, severity, fields map). -->

## Testing

<!-- TODO(S-3.1): the WASM-equivalent of the bats `bash -c '... | hook.sh'`
     pattern — a unit test in the same crate that drives the
     `#[hook]` function with a mock event. Plus the integration test that
     loads the built `.wasm` under wasmtime. -->

## Gotchas

<!-- TODO(S-3.1, S-3.2, S-3.3): collected gotchas from the first three
     ports — anything subtle (e.g. Windows-path differences, env var
     scoping, exit-code mapping, async vs sync behavior). -->

## Target platforms (S-2.3)

A WASM hook that compiles for `wasm32-wasip1` runs unchanged on every
platform the v1.0 dispatcher ships for. The dispatcher itself is built
on the 5-platform CI matrix in `.github/workflows/ci.yml`; the canonical
list of platforms is `ci/platforms.yaml`:

| platform     | runner          | rust target                  | tests run? |
| ------------ | --------------- | ---------------------------- | ---------- |
| darwin-arm64 | macos-14        | aarch64-apple-darwin         | yes        |
| darwin-x64   | macos-15-intel  | x86_64-apple-darwin          | yes        |
| linux-x64    | ubuntu-latest   | x86_64-unknown-linux-gnu     | yes        |
| linux-arm64  | ubuntu-latest   | aarch64-unknown-linux-gnu    | no (cross) |
| windows-x64  | windows-latest  | x86_64-pc-windows-msvc       | yes        |

As a hook author you ship one `*.wasm` per plugin and the dispatcher
loads it on every platform. You do NOT need to think about per-platform
target triples for your plugin code — the only target a hook plugin
declares is `wasm32-wasip1`. The native targets above are what the host
dispatcher is compiled for; they constrain which OS runs your plugin
but never the plugin binary itself.

The linux-arm64 entry uses cross-rs/cross because GitHub-hosted Linux
runners are still x86_64; tests are skipped for that entry only because
we cannot natively exec aarch64 binaries on the host. If your plugin
relies on platform-specific dispatcher behavior (rare), test it on the
linux-x64 + darwin-arm64 entries instead.
