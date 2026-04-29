# AC2 — WorktreeCreate wire format (10-field payload + zero capability)

**Story:** S-5.03 — WorktreeCreate / WorktreeRemove hook wiring
**AC:** AC2 — Wire format for `worktree.created`; also covers zero-capability (Option A)
**BCs:** BC-4.07.001 (primary); BC-4.07.001 Invariant 2 + BC-4.07.004 (zero capability)
**GREEN commit:** `8336cd0`

---

## Wire payload — 10 fields

On a `WorktreeCreate` event, the plugin emits exactly **10 fields** on the wire:

| Field | Source | Value in test |
|-------|--------|---------------|
| `worktree_path` | plugin-set (from envelope) | `"/workspace/feature-branch"` |
| `worktree_name` | plugin-set (from envelope) | `"feature-branch"` |
| `dispatcher_trace_id` | host-enriched (HostContext) | `"trace-create-001"` |
| `session_id` | host-enriched (HostContext) | `"sess-create-001"` |
| `plugin_name` | host-enriched (HostContext) | `"worktree-hooks"` |
| `plugin_version` | host-enriched (HostContext) | `"1.0.0-rc.1"` |
| `ts` | construction-time | `"2026-04-28T..."` |
| `ts_epoch` | construction-time | `<unix timestamp>` |
| `schema_version` | construction-time | `1` |
| `type` | construction-time | `"worktree.created"` |

**Field-count split: 2 plugin-set + 4 host-enriched + 4 construction-time = 10 total**

The plugin sets only `worktree_path` and `worktree_name` — all other 8 fields are RESERVED_FIELDS injected by the host runtime (HostContext for the 4 host-enriched; dispatcher InternalEvent machinery for the 4 construction-time). The plugin must NOT attempt to set any RESERVED_FIELD.

Wire coercion: per `emit_event.rs:49`, all plugin-set values arrive as strings on the wire, regardless of their source type.

---

## RESERVED_FIELDS (8 fields — plugin must NOT set)

```
Host-enriched (4):   dispatcher_trace_id, session_id, plugin_name, plugin_version
Construction-time (4): ts, ts_epoch, schema_version, type
```

The VP-067 test harness (`dispatch_and_capture`) silently drops any RESERVED field the plugin mistakenly attempts to set, then asserts `field_count == 10`. If the plugin were setting RESERVED fields, the drop would produce a mis-count and the assertion would fail.

---

## Zero capability profile (Option A scoping)

The plugin has **zero declared capabilities**: no `read_file`, no `exec_subprocess`.

All data for both plugin-set fields comes directly from the incoming envelope (`payload.tool_input`):

```
worktree_path  → payload.tool_input["worktree_path"].as_str().unwrap_or("")
worktree_name  → payload.tool_input["worktree_name"].as_str().unwrap_or("")  (WorktreeCreate only)
```

**Structural proof:** `worktree_hook_logic` has signature:

```rust
pub fn worktree_hook_logic<Emit>(payload: HookPayload, emit_fn: Emit) -> HookResult
where
    Emit: Fn(&str, &[(&str, &str)]),
```

No `read_file` parameter exists. File reads are structurally impossible from within `worktree_hook_logic`.

Contrast with S-5.01 (SessionStart):
- S-5.01 declares `[hooks.capabilities.read_file]` (reads `.claude/settings.local.json`) and `[hooks.capabilities.exec_subprocess]`
- S-5.03 declares neither — denying both by the host sandbox's deny-by-default policy (BC-1.05.001)

---

## Edge case coverage

| EC | Scenario | Behavior | Test |
|----|----------|----------|------|
| EC-001 | WorktreeCreate fires again for same path (reconnect) | Plugin is stateless — emits on every invocation | `test_bc_4_07_001_worktree_create_idempotent_refire` |
| EC-003 | `worktree_name` absent from envelope | Emits with `worktree_name = ""` (empty string default); does not abort | `test_bc_4_07_001_missing_worktree_name_emits_empty_default` |

---

## Integration test assertion (field count)

```text
test worktree_integration::test_bc_4_07_001_worktree_create_emits_required_fields ... ok
test worktree_integration::test_bc_4_07_001_worktree_create_idempotent_refire ... ok
test worktree_integration::test_bc_4_07_001_missing_worktree_name_emits_empty_default ... ok
test worktree_integration::test_bc_4_07_001_002_no_subprocess_invoked ... ok
test worktree_integration::test_bc_4_07_001_002_no_file_reads ... ok
```

Source: `crates/hook-plugins/worktree-hooks/tests/integration_test.rs`
Field count assertion at line ~345:
```rust
assert_eq!(field_count, 10, "BC-4.07.001: worktree.created wire payload must have exactly 10 fields ...");
```
