## Summary

Comment/prose-only correction of 8 stale `precompact-flush.sh` references across 7 files. The PreCompact flush is delivered as a native WASM plugin (`precompact-flush.wasm`) per S-18.04a / ADR-028, not as a shell script. This PR corrects documentation, comments, and test prose to match the as-delivered implementation. Zero executable or logic changes.

Closes drift item D-703 drift-1 (stale precompact-flush.sh refs); expanded to a tree-wide TD-VSDD-060 sibling-sweep.

## What changed (comment/prose corrections only)

| File | Old reference | Corrected reference |
|------|--------------|---------------------|
| `crates/hook-plugins/validate-burst-log/src/lib.rs` | `precompact-flush.sh` | `` `precompact-flush` PreCompact WASM plugin (`precompact-flush.wasm`, S-18.04a) `` |
| `crates/hook-plugins/validate-dispatch-advance/src/lib.rs` | `precompact-flush.sh` | `` `precompact-flush` PreCompact WASM plugin (`precompact-flush.wasm`, S-18.04a) `` |
| `docs/demo-evidence/S-18.00/README.md` | `precompact-flush.sh` | `` `precompact-flush` PreCompact WASM plugin `` |
| `plugins/vsdd-factory/hooks-registry.toml` | `precompact-flush.sh (S-18.04a)` | `` `precompact-flush` PreCompact WASM plugin (S-18.04a) `` |
| `plugins/vsdd-factory/hooks/check-harness-version.sh` | `precompact-flush.sh (S-18.04a)` | `` `precompact-flush` PreCompact WASM plugin (S-18.04a) `` |
| `plugins/vsdd-factory/hooks/precompact-flush-prune.sh` (×2) | `precompact-flush.sh` | `` `precompact-flush` WASM plugin `` |
| `plugins/vsdd-factory/tests/check-harness-version.bats` | `precompact-flush.sh` | `precompact-flush PreCompact WASM plugin` |

## As-delivered rationale

S-18.04a (ADR-028) delivered the PreCompact flush as a native WASM plugin compiled from `crates/hook-plugins/precompact-flush/src/lib.rs` and registered in `hooks-registry.toml` as `precompact-flush.wasm`. No `precompact-flush.sh` was ever delivered. Comments written before or during early S-18.04a development that referred to the shell-script form were left as stale residue.

## Verification

- `cargo check --workspace` — clean
- `cargo fmt --check --all` — clean
- `cargo clippy --workspace --all-targets -- -D warnings` — clean
- `bash -n plugins/vsdd-factory/hooks/check-harness-version.sh` — OK
- `bash -n plugins/vsdd-factory/hooks/precompact-flush-prune.sh` — OK

**ZERO executable/logic changes in this PR.** All 8 changed lines are in comments, doc strings, or prose strings printed in test failure messages.

## Traceability

- Story: N/A (maintenance)
- BC traceability: N/A (comment-only)
- Drift item: D-703 drift-1
- Sibling-sweep: TD-VSDD-060
- ADR-028 (native WASM delivery): see `.factory/specs/architecture/`

## Architecture Changes

No architectural changes. This PR corrects documentation to accurately reflect the existing WASM-based architecture delivered by S-18.04a / ADR-028.

## Security Review

N/A — comment/prose corrections only; no executable code modified.

## Risk Assessment

- Blast radius: zero (comment-only changes cannot alter runtime behavior)
- Performance impact: none
- Rollback: trivially reversible

## Pre-Merge Checklist

- [x] No executable/logic changes (verified by diff inspection)
- [x] All corrected references verified against as-delivered WASM plugin (`precompact-flush.wasm`)
- [x] `cargo check` clean
- [x] `cargo fmt --check --all` clean
- [x] `cargo clippy` clean
- [x] `bash -n` on edited .sh files OK
- [x] CI checks passing
- [x] PR reviewed (pr-reviewer confirmed no logic changes, factual accuracy verified)
