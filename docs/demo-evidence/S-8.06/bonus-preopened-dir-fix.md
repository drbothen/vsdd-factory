# Bonus: WASI preopened_dir dispatcher fix

**File:** `crates/factory-dispatcher/src/invoke.rs`
**Commit:** `5d5733a feat(s-8.06): green — dispatcher WASI preopen + bats dispatcher invocation`

---

## Context

Prior to this story's green commit, the dispatcher did not preopen any host directory
into the WASM guest's WASI namespace. This meant that any plugin calling `std::fs`
would receive `EBADF` (bad file descriptor) because WASI requires explicit directory
preopens — the guest filesystem namespace is empty by default.

session-learning is the first native WASM plugin that uses `std::fs` directly (all
prior Tier 1 plugins either used `host::emit_event` or went through the
`legacy-bash-adapter`'s subprocess path). Without the preopen fix, every `std::fs`
call in session-learning would fail silently with `EBADF`.

---

## Fix (crates/factory-dispatcher/src/invoke.rs, lines 130-157)

```rust
// Preopen the project directory (host_ctx.cwd) as "." in the WASI guest.
// This enables std::fs operations from WASM plugins that perform filesystem
// I/O relative to the project root (e.g. session-learning appending to
// .factory/sidecar-learning.md). Plugins without filesystem needs are
// unaffected — they simply ignore the preopened handle.
// If the cwd path cannot be opened (e.g. missing dir in tests), the WASI
// context is built without a preopen and std::fs calls will return EBADF.
let mut wasi_builder = WasiCtxBuilder::new();
wasi_builder
    .stdin(MemoryInputPipe::new(payload_json.to_vec()))
    .stdout(stdout.clone())
    .stderr(stderr.clone());
if host_ctx.cwd.as_os_str().is_empty() {
    // No project dir — build without filesystem preopen.
} else if let Err(e) = wasi_builder.preopened_dir(
    &host_ctx.cwd,
    ".",
    DirPerms::all(),
    FilePerms::all(),
) {
    // Non-fatal: log and continue without filesystem access.
    // Plugin may still function if it doesn't need std::fs.
    tracing::debug!(
        cwd = %host_ctx.cwd.display(),
        err = %e,
        "wasi preopen failed; plugin std::fs calls will fail"
    );
}
let wasi_ctx = wasi_builder.build_p1();
```

---

## Why this matters

This is a workspace-shared improvement. The fix lives in the dispatcher (`crates/factory-dispatcher`),
not in session-learning itself. Every future hook plugin that uses `std::fs` will benefit
automatically — no per-plugin change needed.

Affected scenarios unblocked by this fix:

| Future story | Filesystem need |
|---|---|
| Any plugin reading config files at project root | `std::fs::read_to_string(...)` |
| Any plugin creating output artifacts | `std::fs::write(...)` |
| session-learning (S-8.06, this story) | `std::fs::write` + `OpenOptions::append` to `.factory/sidecar-learning.md` |

---

## Failure mode without this fix

Without the preopen:
- `std::fs::write(".factory/sidecar-learning.md", ...)` returns `Err(Os { code: 9, kind: Other, message: "Bad file descriptor" })`
- Plugin returns `HookResult::Error(...)` with that message
- Dispatcher logs the hook failure (but `on_error = "continue"` prevents session block)
- `.factory/sidecar-learning.md` is never written; BC-7.03.077 postcondition silently violated

---

## Defense-in-depth design

The fix uses a non-fatal fallback: if `preopened_dir` fails (e.g., cwd does not exist
in a test environment), the WASI context is built without a preopen. A `tracing::debug`
log records the failure. This ensures the dispatcher does not crash when invoked with
an empty or invalid cwd — plugins that don't use `std::fs` continue to function normally.

---

## Imports added to invoke.rs

```rust
use wasmtime_wasi::{DirPerms, FilePerms, I32Exit, WasiCtxBuilder};
```

`DirPerms::all()` and `FilePerms::all()` grant the guest full read+write+execute
permissions on the preopened directory. This is appropriate for trusted hook plugins
running in the project's own cwd; plugins are already trusted code compiled from the
workspace.
