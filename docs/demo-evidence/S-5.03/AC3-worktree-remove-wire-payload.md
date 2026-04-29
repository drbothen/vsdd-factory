# AC3 — WorktreeRemove wire format (9-field payload + EC-002 unknown-worktree)

**Story:** S-5.03 — WorktreeCreate / WorktreeRemove hook wiring
**AC:** AC3 — Wire format for `worktree.removed`; also covers EC-002 unknown-worktree no-op
**BCs:** BC-4.07.002
**GREEN commit:** `8336cd0`

---

## Wire payload — 9 fields

On a `WorktreeRemove` event, the plugin emits exactly **9 fields** on the wire:

| Field | Source | Value in test |
|-------|--------|---------------|
| `worktree_path` | plugin-set (from envelope) | `"/workspace/old-feature"` |
| `dispatcher_trace_id` | host-enriched (HostContext) | `"trace-remove-001"` |
| `session_id` | host-enriched (HostContext) | `"sess-remove-001"` |
| `plugin_name` | host-enriched (HostContext) | `"worktree-hooks"` |
| `plugin_version` | host-enriched (HostContext) | `"1.0.0-rc.1"` |
| `ts` | construction-time | `"2026-04-28T..."` |
| `ts_epoch` | construction-time | `<unix timestamp>` |
| `schema_version` | construction-time | `1` |
| `type` | construction-time | `"worktree.removed"` |

**Field-count split: 1 plugin-set + 4 host-enriched + 4 construction-time = 9 total**

---

## Differential vs. WorktreeCreate

`WorktreeRemove` has **one fewer field** than `WorktreeCreate` because it omits `worktree_name`:

| Event | Plugin-set fields | Wire total |
|-------|-------------------|-----------|
| WorktreeCreate | `worktree_path`, `worktree_name` (2) | **10** |
| WorktreeRemove | `worktree_path` only (1) | **9** |

`worktree_name` is absent by design: a remove event identifies the worktree by path only. The test explicitly asserts this absence:

```rust
assert!(
    event.get("worktree_name").is_none(),
    "BC-4.07.002: worktree_name must NOT be present in worktree.removed (only 1 plugin-set field)"
);
```

---

## EC-002 — Unknown worktree path emits normally

**Scenario:** `WorktreeRemove` for a `worktree_path` that was never registered.

**Expected behavior:** Plugin emits `worktree.removed` normally — no error, no abort. The plugin has no registry of known worktrees; it is unconditionally stateless. The consumer (observability stack) handles unknown-path removals gracefully.

Test assertion:

```text
test worktree_integration::test_bc_4_07_002_unknown_worktree_remove_no_op ... ok
```

Test drives path `/workspace/never-existed` through `WorktreeRemove` dispatch and asserts:
- Exactly 1 `worktree.removed` event emitted
- `worktree_path == "/workspace/never-existed"` (the unknown path passes through unchanged)

---

## Edge case coverage

| EC | Scenario | Behavior | Test |
|----|----------|----------|------|
| EC-002 | `WorktreeRemove` for unknown `worktree_path` | Emits normally; plugin has no known-worktree registry | `test_bc_4_07_002_unknown_worktree_remove_no_op` |
| EC-003 | `worktree_path` absent from envelope | Emits with `worktree_path = ""` (empty string default); does not abort | `test_bc_4_07_002_missing_worktree_path_emits_empty_default` |

---

## Integration test assertions (field count)

```text
test worktree_integration::test_bc_4_07_002_worktree_remove_emits_required_fields ... ok
test worktree_integration::test_bc_4_07_002_unknown_worktree_remove_no_op ... ok
test worktree_integration::test_bc_4_07_002_missing_worktree_path_emits_empty_default ... ok
```

Source: `crates/hook-plugins/worktree-hooks/tests/integration_test.rs`
Field count assertion at line ~560:
```rust
assert_eq!(field_count, 9, "BC-4.07.002: worktree.removed wire payload must have exactly 9 fields ...");
```
