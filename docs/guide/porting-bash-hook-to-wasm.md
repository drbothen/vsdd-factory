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
