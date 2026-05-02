# AC-001 Evidence — WASM Crate Exists, Registry Migrated

**AC:** AC-001 (traces to BC-7.03.081 postcondition 1)
**Statement:** WASM crate `crates/hook-plugins/track-agent-stop/` exists with `Cargo.toml`
targeting `wasm32-wasip1`; registry entry updated to native `.wasm` path; `script_path`,
`shell_bypass_acknowledged`, and `[hooks.capabilities]` section fully removed; binding
preserved: `event = "SubagentStop"`, `priority = 930`, `on_error = "continue"`, `timeout_ms = 5000`.

## Recording

- `AC-001-wasm-crate-registry-migration.gif` / `.webm` / `.tape`

## Verification Steps Shown

1. `cat crates/hook-plugins/track-agent-stop/Cargo.toml` — shows `name = "track-agent-stop"`,
   `vsdd-hook-sdk` dependency, `serde_json`, `regex`. No `wasm32-wasi` (deprecated alias).
2. `grep -A 8 'track-agent-stop' plugins/vsdd-factory/hooks-registry.toml` — shows
   `plugin = "hook-plugins/track-agent-stop.wasm"`, no `script_path`, no `shell_bypass_acknowledged`,
   no `[hooks.capabilities]` section.
3. `ls -lh target/wasm32-wasip1/release/track-agent-stop.wasm` — artifact exists.

## Registry Stanza (confirmed)

```toml
[[hooks]]
name = "track-agent-stop"
event = "SubagentStop"
plugin = "hook-plugins/track-agent-stop.wasm"
priority = 930
timeout_ms = 5000
on_error = "continue"
```

No `script_path`. No `shell_bypass_acknowledged`. No `[hooks.capabilities]` section. Aligned
with S-8.01 canonical pattern per AC-001 spec.

## Result

PASS — WASM crate built, registry migrated, all legacy fields removed.
