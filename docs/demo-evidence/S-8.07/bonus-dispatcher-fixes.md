# Bonus: Three Dispatcher Fixes (Workspace-Shared Improvements)

These fixes landed in commit `216f05e` as part of the S-8.07 green phase.
They are workspace-shared improvements in `crates/factory-dispatcher/` that unblock
future hook ports requiring `host::read_file`.

---

## Fix 1: Real read_file Implementation in invoke.rs

**File:** `crates/factory-dispatcher/src/invoke.rs` (lines 459-534)

**Problem:** `host::read_file` was previously a stub that returned
`codes::CAPABILITY_DENIED` on every call. No native WASM hook plugin could read
files from the host filesystem. warn-pending-wave-gate is the first in-tree consumer
of this path.

**Fix:** Implemented the full out-param protocol for `read_file` in the dispatcher's
`invoke.rs` WASM linker setup. The implementation:

1. Reads the path string from WASM linear memory via `read_wasm_string_sd`.
2. Calls `crate::host::read_file::prepare(&ctx, &path, max_bytes)` — capability
   check, path resolution, and bounded read (all host-side, no WASM memory access).
3. If the file is empty, writes `ptr=0, len=0` to the out-params and returns `OK`.
4. If the file has content, grows WASM linear memory by the required number of 64KiB
   pages (`body.len().div_ceil(65536)` pages), writes the file bytes at the newly
   allocated address, and writes that address and length back to `out_ptr_out` and
   `out_len_out`.
5. Returns `codes::OK`.

The guest-side SDK wrapper (`read_owned_bytes`) reads the bytes from the out-param
pointer and returns them as `Vec<u8>`. The SDK guards `ptr == 0` → returns `Vec::new()`
for the empty-file case.

**Impact:** All future native WASM hook plugins that call `host::read_file` now work
correctly. warn-pending-wave-gate is the first consumer. Other E-8 tier-2 and tier-3
hooks that need filesystem access (e.g. those reading `.claude/settings.local.json`
or `.factory/` files) will use this same path.

**Key code excerpt:**

```rust
// Find the current end of WASM linear memory, then grow by
// enough pages to hold `body`.
let memory = match get_memory_sd(&mut caller) {
    Ok(m) => m,
    Err(_) => return codes::INTERNAL_ERROR,
};
let current_bytes = memory.data_size(&caller);
let pages_needed = body.len().div_ceil(65536) as u64;
if memory.grow(&mut caller, pages_needed).is_err() {
    return codes::INTERNAL_ERROR;
}
let write_offset = current_bytes as u32;
if write_wasm_bytes_sd(&mut caller, write_offset, body.len() as u32, &body).is_err() {
    return codes::INTERNAL_ERROR;
}
if write_wasm_u32_sd(&mut caller, out_ptr_out, write_offset).is_err() {
    return codes::INVALID_ARGUMENT;
}
if write_wasm_u32_sd(&mut caller, out_len_out, body.len() as u32).is_err() {
    return codes::INVALID_ARGUMENT;
}
codes::OK
```

---

## Fix 2: cwd Path Resolution in host/read_file.rs

**File:** `crates/factory-dispatcher/src/host/read_file.rs` (lines 79-113)

**Problem:** The `prepare()` function in `read_file.rs` was resolving relative paths
under `ctx.plugin_root` (the plugin directory, `$CLAUDE_PLUGIN_ROOT`). This caused
`.factory/wave-state.yaml` to be looked up in the plugin directory rather than the
project directory. The capability allow-list check also used the wrong base for
relative entries like `.factory/wave-state.yaml`.

**Fix:** Changed path resolution to use `ctx.cwd` (`$CLAUDE_PROJECT_DIR`, the project
working directory) for all relative paths:

```rust
// Relative paths are resolved under `ctx.cwd` (the project root,
// i.e. `$CLAUDE_PROJECT_DIR`) so that project-relative files like
// `.factory/wave-state.yaml` and `.claude/settings.local.json` are
// found in the project directory, not the plugin directory.
let resolved = resolve_for_read(Path::new(path), &ctx.cwd);

if !path_allowed(&resolved, &caps.path_allow, &ctx.cwd) {
    // ...
}
```

The helper functions were renamed from `resolve_for_write` / `path_allowed` (which
referenced `plugin_root`) to `resolve_for_read` / `path_allowed` (which use `base`
consistently). The `path_allowed` function expands allow-list entries that are
relative (e.g. `.factory/wave-state.yaml`) under `base` (the project cwd).

A unit test was added to verify the fix:

```rust
#[test]
fn relative_path_resolves_under_cwd() {
    // Relative paths (e.g. ".factory/wave-state.yaml") are resolved
    // under ctx.cwd ($CLAUDE_PROJECT_DIR), not plugin_root.
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("rel.txt"), b"yes").unwrap();
    let mut ctx = context_with_caps(allow_read(&["."]));
    ctx.cwd = dir.path().to_path_buf();
    let (bytes, _) = prepare(&ctx, "rel.txt", 1024).unwrap();
    assert_eq!(bytes, b"yes");
}
```

**Impact:** Any hook plugin that reads a project-relative file (e.g.
`.factory/wave-state.yaml`, `.claude/settings.local.json`) via `host::read_file`
now correctly resolves the path under `$CLAUDE_PROJECT_DIR`. Without this fix, all
such reads would fail silently (file-not-found → capability denied → `HookResult::Continue`).

---

## Fix 3: Plugin stderr Relay in main.rs

**File:** `crates/factory-dispatcher/src/main.rs` (lines 181-192)

**Problem:** The WASI sandbox captures all plugin stderr output into a
`MemoryOutputPipe` inside the WASM runtime. Without explicit relay code, this
captured output never reaches the terminal. The WAVE GATE REMINDER from
warn-pending-wave-gate would be silently swallowed, invisible to the user.

**Fix:** Added a stderr relay loop in `main.rs` that runs after `execute_tiers`
completes:

```rust
// Relay any non-empty plugin stderr to the dispatcher's process stderr so
// user-visible hook messages (e.g. WAVE GATE REMINDER from
// warn-pending-wave-gate) reach the terminal. The WASI sandbox captures
// plugin stderr into MemoryOutputPipe; without this relay the output
// would only appear in the internal log, invisible to the user.
for outcome in &summary.per_plugin_results {
    if let PluginResult::Ok { stderr, .. } = &outcome.result
        && !stderr.is_empty()
    {
        eprint!("{stderr}");
    }
}
```

The `PluginResult::Ok { stderr, .. }` field was already being captured (added in
v1.0.0-beta.4 for diagnostic logging) but was not being relayed to the process
stderr stream. This fix closes that gap.

**Impact:** Any native WASM hook plugin that writes to stderr (advisory messages,
user-visible warnings) now has that output reach the terminal. This is the mechanism
that makes warn-pending-wave-gate's WAVE GATE REMINDER visible to the operator
ending a Claude Code session.

---

## Summary

| Fix | File | Problem | Resolution |
|-----|------|---------|------------|
| 1. Real read_file impl | `invoke.rs` | Stub returned CAPABILITY_DENIED; no plugin could read files | Memory-grow + out-param write protocol implemented |
| 2. cwd path resolution | `host/read_file.rs` | Relative paths resolved under plugin_root, not project dir | Resolution changed to ctx.cwd ($CLAUDE_PROJECT_DIR) |
| 3. stderr relay | `main.rs` | Plugin stderr captured into MemoryOutputPipe but never emitted | Relay loop added after execute_tiers |

All three fixes are workspace-wide: they apply to every WASM hook plugin in the
dispatcher, not just warn-pending-wave-gate. These are the foundational runtime
improvements that make native WASM hook plugins viable for the E-8 migration.
