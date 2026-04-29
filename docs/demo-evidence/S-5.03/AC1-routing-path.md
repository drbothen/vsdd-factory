# AC1 — Full routing path wired

**Story:** S-5.03 — WorktreeCreate / WorktreeRemove hook wiring
**AC:** AC1 — Full Layer 1 → dispatcher → Layer 2 → plugin wiring path
**BCs:** BC-4.07.003 + BC-4.07.004
**GREEN commit:** `8336cd0`

---

## What is verified

Both `WorktreeCreate` and `WorktreeRemove` travel through two routing layers (ADR-011 dual-hook-routing-tables):

| Layer | File | Routes to |
|-------|------|-----------|
| Layer 1 (harness) | `plugins/vsdd-factory/hooks/hooks.json.template` | `factory-dispatcher` binary |
| Layer 2 (dispatcher) | `plugins/vsdd-factory/hooks-registry.toml` | `worktree-hooks.wasm` |

---

## Layer 1 — hooks.json.template (BC-4.07.003)

File: `plugins/vsdd-factory/hooks/hooks.json.template`
Keys: `hooks.WorktreeCreate[0].hooks[0]` and `hooks.WorktreeRemove[0].hooks[0]`

Both entries are structurally identical:

```json
{
  "type": "command",
  "command": "${CLAUDE_PLUGIN_ROOT}/hooks/dispatcher/bin/{{PLATFORM}}/factory-dispatcher{{EXE_SUFFIX}}",
  "timeout": 10000,
  "async": true
}
```

Invariants confirmed:
- `command` contains `factory-dispatcher` — routes to the dispatcher binary, NOT a `.wasm` file (ADR-011 layer separation)
- `once` key is **completely absent** — worktree events can re-fire on Claude Code reconnect (EC-001); defensive omission per BC-4.07.003 Invariant 1; the key must not exist, not `once: false`, not `once: true`
- `async: true` — non-blocking harness invocation
- `timeout: 10000` — harness timeout (outer bound of timeout hierarchy; dispatcher budget is 5000 < 10000)

This contrasts with `SessionStart` and `SessionEnd` entries which carry `once: true` (session events fire exactly once per session).

---

## Layer 2 — hooks-registry.toml (BC-4.07.004)

File: `plugins/vsdd-factory/hooks-registry.toml`

```toml
[[hooks]]
name = "worktree-hooks"
event = "WorktreeCreate"
plugin = "hook-plugins/worktree-hooks.wasm"
timeout_ms = 5000

[[hooks]]
name = "worktree-hooks"
event = "WorktreeRemove"
plugin = "hook-plugins/worktree-hooks.wasm"
timeout_ms = 5000
```

Invariants confirmed:
- Both entries route to the **same** `.wasm` — single-crate-two-entries design (BC-4.07.004)
- `plugin` path includes `hook-plugins/` prefix — required for dispatcher binary resolution (BC-4.07.004 Invariant 3)
- `timeout_ms = 5000` — dispatcher budget; within the `timeout: 10000` harness outer bound
- No `[hooks.capabilities.*]` tables — zero-capability sandbox profile (ZERO declared capabilities; Option A scoping)
- No `once` field — `RegistryEntry` has no such field; `deny_unknown_fields` would reject it; once-discipline is Layer 1 only

---

## Integration test confirmation

Both routing layers verified by tests in the VP-067 harness:

```text
test worktree_integration::test_bc_4_07_003_hooks_json_template_has_worktree_create_and_remove ... ok
test worktree_integration::test_bc_4_07_004_hooks_registry_toml_has_worktree_create_and_remove ... ok
```

Sources:
- `crates/hook-plugins/worktree-hooks/tests/integration_test.rs:817`  (BC-4.07.003 test)
- `crates/hook-plugins/worktree-hooks/tests/integration_test.rs:902`  (BC-4.07.004 test)
