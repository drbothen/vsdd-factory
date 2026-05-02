---
story_id: S-8.00
ac: AC-7
title: Bundle-size baseline measured
---

# AC-7: Bundle-size baseline measured

**Statement:** v1.0.0 dispatcher bundle size measured: run `du -sb` on bundled `.wasm` artifacts and dispatcher binary. Result committed to `.factory/measurements/E-8-bash-baseline.json` in the `bundle_size` object with `legacy_bash_adapter_wasm_bytes`, `all_hook_plugins_wasm_bytes`, `dispatcher_binary_bytes` sub-fields.

## Evidence

### Bundle-size section from `.factory/measurements/E-8-bash-baseline.json`

```json
{
  "bundle_size": {
    "measured_at": "2026-05-02T00:00:00Z",
    "legacy_bash_adapter_wasm_bytes": 169370,
    "all_hook_plugins_wasm_bytes": 321843,
    "dispatcher_binary_bytes": 12150752,
    "measurement_note": "bundle directory (plugins/vsdd-factory/hook-plugins/) contains legacy-bash-adapter.wasm (169370 bytes), hello-hook.wasm (152370 bytes), and capture_commit_activity.wasm (103 bytes stub) at baseline. Per-hook WASM ports begin at S-8.01; all_hook_plugins_wasm_bytes reflects full bundle at this baseline measurement."
  }
}
```

### Field interpretation

| Field | Value | Notes |
|-------|-------|-------|
| `legacy_bash_adapter_wasm_bytes` | 169,370 bytes (~165 KB) | The legacy-bash-adapter.wasm artifact that routes to bash hooks; this is the artifact being replaced by S-8.01..S-8.09 |
| `all_hook_plugins_wasm_bytes` | 321,843 bytes (~314 KB) | Total of all .wasm files in `plugins/vsdd-factory/hook-plugins/` at baseline; includes legacy-bash-adapter + hello-hook + capture_commit_activity stub |
| `dispatcher_binary_bytes` | 12,150,752 bytes (~11.6 MB) | The dispatcher binary (contains embedded WASM plugins); baseline for R-8.09 25% growth ceiling |

### Measurement commands used

```bash
# legacy-bash-adapter size
du -sb plugins/vsdd-factory/hook-plugins/legacy-bash-adapter.wasm

# all hook plugin wasm files
du -sb plugins/vsdd-factory/hook-plugins/*.wasm | awk '{sum+=$1} END {print sum}'

# dispatcher binary
du -sb target/release/claude-code-hook-dispatcher
```

### R-8.09 growth ceiling baseline

25% of 12,150,752 bytes = 3,037,688 bytes (~2.9 MB) headroom before R-8.09 triggers size-optimization review.

**Result:** AC-7 SATISFIED. Bundle-size baseline captured for all three required fields. Committed to `.factory/measurements/E-8-bash-baseline.json` `bundle_size` section.
