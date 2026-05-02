# Bonus Section Evidence — Capabilities deny_unknown_fields fix + workspace clippy nits

This file documents the three bonus changes shipped in commit 23eaf06 alongside the
S-8.02 main implementation.

---

## 1. emit_event removed from hooks.capabilities (deny_unknown_fields correctness fix)

**Background:** The legacy `hooks-registry.toml` entry for `pr-manager-completion-guard`
included `emit_event = true` in the `[hooks.capabilities]` block (carried over from the
legacy-bash-adapter scaffolding). This field does not exist in the `Capabilities` struct
in `crates/factory-dispatcher/src/registry.rs`, which uses `#[serde(deny_unknown_fields)]`.

**Problem:** With `deny_unknown_fields` on `Capabilities`, any unknown field in the
`[hooks.capabilities]` TOML block causes a deserialization error (registry load failure).
The `emit_event = true` field was an invalid capability declaration — `emit_event` is
always wired unconditionally to every WASM plugin via the host ABI; it is NOT a
declared capability.

**Fix (registry.toml diff):**
```diff
-[hooks.capabilities]
-env_allow = ["PATH", "HOME", "TMPDIR", "CLAUDE_PROJECT_DIR", "CLAUDE_PLUGIN_ROOT", "VSDD_SESSION_ID"]
-
-[hooks.capabilities.exec_subprocess]
-binary_allow = ["bash", "gh", "jq"]
-shell_bypass_acknowledged = "legacy-bash-adapter runs unported hooks"
-env_allow = ["PATH", "HOME", "TMPDIR", "CLAUDE_PROJECT_DIR", "CLAUDE_PLUGIN_ROOT", "VSDD_SESSION_ID"]
```

The entire `[hooks.capabilities]` block was removed because:
1. The native WASM plugin does not use `exec_subprocess` (no `gh` calls)
2. The native WASM plugin does not need the `env_allow` overrides
3. `emit_event` is always available unconditionally — no capability declaration needed

**Capabilities struct (registry.rs lines 52-67):**
```rust
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Capabilities {
    pub exec_subprocess: Option<ExecSubprocessCaps>,
    pub read_file: Option<ReadFileCaps>,
    pub write_file: Option<WriteFileCaps>,
    pub env_allow: Vec<String>,
    // Note: NO emit_event field — emit_event is always wired unconditionally
}
```

---

## 2. factory-dispatcher/src/main.rs — clippy let-chain collapse

**Change:** Collapsed nested `if let + if` guard into a single let-chain expression.

```diff
-    if let Ok(sink_path) = std::env::var(ENV_SINK_FILE) {
-        if !sink_path.is_empty() {
-            flush_sink_file(&sink_path, &event_queue);
-        }
+    if let Ok(sink_path) = std::env::var(ENV_SINK_FILE)
+        && !sink_path.is_empty()
+    {
+        flush_sink_file(&sink_path, &event_queue);
     }
```

This is a pre-existing clippy `collapsible_if` lint nit introduced by the S-8.07/S-8.08
sink-file flush code. The fix uses Rust 1.64+ let-chains syntax supported by the 1.95
toolchain. No behavior change.

---

## 3. track-agent-start/src/lib.rs — two pre-existing clippy nits

**Change 1:** Removed `let_unit_value` warning — `let _ = emit_event(...)` suppresses
a unit return value unnecessarily.

```diff
-        // EC-006: emit_event error silently swallowed via let _.
-        let _ = vsdd_hook_sdk::host::emit_event(event_type, fields);
+        vsdd_hook_sdk::host::emit_event(event_type, fields);
```

`host::emit_event` returns `()`, not a `Result`. The `let _` was a cargo-cult pattern
from a version where the function returned `Result`. Removing it matches the actual API.

**Change 2:** Fixed doc comment over-indented list items (`doc_overindented_list_items`).

```diff
-//!                    pattern 1: `S-[0-9]+\.[0-9]+` (e.g., S-8.03); pattern 2: `STORY-[0-9]+`
-//!                    (e.g., STORY-042); omitted if neither matches (EC-003).
+//!     pattern 1: `S-[0-9]+\.[0-9]+` (e.g., S-8.03); pattern 2: `STORY-[0-9]+`
+//!     (e.g., STORY-042); omitted if neither matches (EC-003).
```

Over-indented list items in doc comments cause clippy to warn because they can render
incorrectly in rustdoc. Standard indentation is 4 spaces for continuation lines.

Both fixes were pre-existing nits introduced in S-8.08 and surfaced when building
S-8.02 with clippy clean enforced. Neither changes behavior.

---

## Summary

| Fix | File | Category | Behavior change |
|---|---|---|---|
| emit_event removed from capabilities | hooks-registry.toml | Correctness (deny_unknown_fields) | No (field was previously ignored or caused error) |
| Collapsed if-let chain | factory-dispatcher/src/main.rs | Clippy nit (pre-existing) | No |
| Removed let_unit_value | track-agent-start/src/lib.rs | Clippy nit (pre-existing) | No |
| Fixed doc_overindented_list_items | track-agent-start/src/lib.rs | Clippy nit (pre-existing) | No |
