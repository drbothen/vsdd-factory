# AC-001: WASM crate built; registry migration complete

**BC trace:** BC-7.04.040 postcondition 1 (identity & registry binding)
**Status:** PASS

## What was verified

Crate `crates/hook-plugins/validate-pr-review-posted/` exists with:
- `[lib]` section (`path = "src/lib.rs"`) and `[[bin]]` section (`name = "validate-pr-review-posted"`, `path = "src/main.rs"`)
- Target: `wasm32-wasip1` (not deprecated `wasm32-wasi`)
- Dependencies: `vsdd-hook-sdk = { path = "../../hook-sdk" }`, `serde_json`, `regex`
- WASM artifact: `target/wasm32-wasip1/debug/validate-pr-review-posted.wasm` exists
- WASM artifact copied to: `plugins/vsdd-factory/hook-plugins/validate-pr-review-posted.wasm`

Registry entry in `plugins/vsdd-factory/hooks-registry.toml`:
```toml
[[hooks]]
name = "validate-pr-review-posted"
event = "SubagentStop"
plugin = "hook-plugins/validate-pr-review-posted.wasm"
priority = 950
timeout_ms = 5000
on_error = "continue"
```

Key fields preserved: `name`, `event = "SubagentStop"`, `priority = 950`, `on_error = "continue"`, `timeout_ms = 5000`.

Registry migration: `script_path`, `shell_bypass_acknowledged`, `binary_allow`, `[hooks.capabilities.exec_subprocess]`, and all `env_allow` children are absent — replaced by native WASM with no subprocess needs.

## Recording

[AC-001-registry-migration.gif](AC-001-registry-migration.gif)
