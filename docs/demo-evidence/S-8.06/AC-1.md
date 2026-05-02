# AC-001 Evidence — WASM crate exists, builds wasm32-wasip1, registry updated

**Story:** S-8.06 — Native port: session-learning (Stop)
**BC trace:** BC-7.03.076 postcondition 1 (identity & registry binding)

---

## Claim

WASM crate `crates/hook-plugins/session-learning/` exists with `Cargo.toml` targeting
`wasm32-wasip1`, implements the `vsdd_hook_sdk` hook interface, and builds successfully.
Registry entry in `hooks-registry.toml` is updated to `plugin = "hook-plugins/session-learning.wasm"`.
`exec_subprocess` block and top-level `[hooks.capabilities]` block both removed.
Event binding preserved: `event = "Stop"`, `priority = 910`, `on_error = "continue"`.

---

## Cargo.toml (crates/hook-plugins/session-learning/Cargo.toml)

```toml
[package]
name = "session-learning"
version = "0.0.1"
edition.workspace = true
license.workspace = true
repository.workspace = true
authors.workspace = true
rust-version.workspace = true
description = "WASM hook plugin: appends timestamped session-end markers to .factory/sidecar-learning.md on Stop events"
publish = false

[lib]
path = "src/lib.rs"

[[bin]]
name = "session-learning"
path = "src/main.rs"

[dependencies]
vsdd-hook-sdk = { path = "../../hook-sdk" }
chrono = { workspace = true }

[dev-dependencies]
tempfile.workspace = true

[lints]
workspace = true
```

Key observations:
- No `serde_json` dependency (not needed — session-learning does not parse stdin).
- No `legacy-bash-adapter` dependency (forbidden per AC-001 compliance rules).
- `vsdd-hook-sdk` as path dep `../../hook-sdk` (empirically verified form per D-172 finding #8).
- `chrono` workspace pin used for UTC timestamp formatting.

---

## hooks-registry.toml entry (lines 845-851)

```toml
[[hooks]]
name = "session-learning"
event = "Stop"
plugin = "hook-plugins/session-learning.wasm"
priority = 910
timeout_ms = 5000
on_error = "continue"
```

Key observations:
- `plugin` points to native WASM artifact (no `legacy-bash-adapter.wasm`).
- `script_path` field absent (removed per T-6).
- `shell_bypass_acknowledged` absent (removed per T-6).
- `[hooks.config]` sub-block absent (removed per T-6).
- `[hooks.capabilities.exec_subprocess]` sub-block absent (removed per T-6).
- `[hooks.capabilities]` top-level block absent (removed per T-6 — no env vars needed).
- `event = "Stop"`, `priority = 910`, `on_error = "continue"` preserved.

---

## WASM artifact existence

The `.wasm` artifact is present at:

```
target/wasm32-wasip1/debug/session-learning.wasm
```

Build command: `cargo build --target wasm32-wasip1 -p session-learning`

The crate is registered in the workspace `Cargo.toml` members array:

```
"crates/hook-plugins/session-learning"
```

---

## session-learning.sh deleted

`plugins/vsdd-factory/hooks/session-learning.sh` is absent from the worktree — deleted
in the implementation commit (T-7 complete). The git diff for this story shows only
the `.sh` file deleted and `hooks-registry.toml` modified; no `hooks.json` files appear
(DRIFT-004 is in the AFTER state).

---

**Verdict: AC-001 PASS**
