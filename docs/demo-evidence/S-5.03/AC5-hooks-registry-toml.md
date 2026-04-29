# AC5 — hooks-registry.toml entries (two `[[hooks]]` blocks; zero capabilities)

**Story:** S-5.03 — WorktreeCreate / WorktreeRemove hook wiring
**AC:** AC5 — Registry contains exactly one WorktreeCreate entry and one WorktreeRemove entry
**BCs:** BC-4.07.004
**GREEN commit:** `8336cd0`

---

## Actual registry entries

```toml
# ---------- WorktreeCreate ----------

[[hooks]]
name = "worktree-hooks"
event = "WorktreeCreate"
plugin = "hook-plugins/worktree-hooks.wasm"
timeout_ms = 5000

# ---------- WorktreeRemove ----------

[[hooks]]
name = "worktree-hooks"
event = "WorktreeRemove"
plugin = "hook-plugins/worktree-hooks.wasm"
timeout_ms = 5000
```

File: `plugins/vsdd-factory/hooks-registry.toml`

---

## Single-crate / two-entry design (BC-4.07.004)

Both entries route to the **same** `.wasm` binary (`hook-plugins/worktree-hooks.wasm`). This is the BC-4.07.004 "single-crate-two-entries" design:

- One crate (`crates/hook-plugins/worktree-hooks/`) handles both `WorktreeCreate` and `WorktreeRemove`
- Internal dispatch on `payload.event_name` selects the correct emission path
- Two `[[hooks]]` entries are required — one per event — because `hooks-registry.toml` is event-keyed

The implementer used `name = "worktree-hooks"` for both entries. BC-4.07.004 does not require unique names across entries; this is defensible and consistent with the single-crate design rationale.

---

## ZERO capability tables

Neither entry carries any `[hooks.capabilities.*]` tables. This is the Option A zero-capability sandbox profile.

Contrast with `session-start-telemetry` (S-5.01) which declares two capability tables:

```toml
# S-5.01 (session-start-telemetry) — for contrast
[[hooks]]
name = "session-start-telemetry"
event = "SessionStart"
plugin = "hook-plugins/session-start-telemetry.wasm"
timeout_ms = 8000

[hooks.capabilities.read_file]
path_allow = [".claude/settings.local.json"]

[hooks.capabilities.exec_subprocess]
...
```

S-5.03 worktree entries have no equivalent capability tables — the host sandbox grants nothing by default (BC-1.05.001 deny-by-default).

---

## No `once` field on RegistryEntry

No `once` field appears in either entry. `RegistryEntry` has `deny_unknown_fields` — any `once` field would cause a deserialization error at registry-load time. The `once` discipline (re-fire allowed) is enforced at Layer 1 (hooks.json.template) by key absence, not at Layer 2.

---

## BC-4.07.004 checklist

| Invariant | WorktreeCreate | WorktreeRemove |
|-----------|----------------|----------------|
| Exactly one entry per event | confirmed | confirmed |
| `name = "worktree-hooks"` | confirmed | confirmed |
| `plugin = "hook-plugins/worktree-hooks.wasm"` | confirmed | confirmed |
| `hook-plugins/` prefix present (Invariant 3) | confirmed | confirmed |
| `timeout_ms = 5000` | confirmed | confirmed |
| NO capability tables (Postconditions 5–6) | confirmed | confirmed |
| NO `once` field (Invariant 2) | confirmed | confirmed |

---

## Integration test assertion

```text
test worktree_integration::test_bc_4_07_004_hooks_registry_toml_has_worktree_create_and_remove ... ok
```

Key assertions in test (line ~963):
```rust
assert!(
    entry.get("capabilities").is_none(),
    "BC-4.07.004 Postconditions 5-6: {event_name} entry must have NO capability tables ..."
);
assert!(
    entry.get("once").is_none(),
    "BC-4.07.004 Invariant 2: {event_name} RegistryEntry must NOT have a 'once' field ..."
);
```
