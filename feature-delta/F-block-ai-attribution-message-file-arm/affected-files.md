# Affected Files — F-block-ai-attribution-message-file-arm

Machine-readable change list for F1 delta analysis.
Format: `CHANGE_TYPE | FILE`

## Source Code

| Change | File |
|--------|------|
| MODIFIED | `crates/hook-plugins/block-ai-attribution/src/lib.rs` |
| MODIFIED | `crates/hook-plugins/block-ai-attribution/src/main.rs` |
| MODIFIED | `crates/hook-plugins/block-ai-attribution/Cargo.toml` |
| MODIFIED | `plugins/vsdd-factory/hooks-registry.toml` |

## Behavioral Contracts

| Change | File |
|--------|------|
| NEW | `.factory/specs/behavioral-contracts/ss-07/BC-7.03.094.md` |
| NEW | `.factory/specs/behavioral-contracts/ss-07/BC-7.03.095.md` |
| MODIFIED (version bump only) | `.factory/specs/behavioral-contracts/ss-07/BC-7.03.001.md` |

## Verification Properties

| Change | File |
|--------|------|
| NEW | `.factory/specs/verification-properties/VP-080.md` |

## Architecture Section Files

| Change | File |
|--------|------|
| MODIFIED | `.factory/specs/architecture/SS-07-hook-bash.md` |
| MODIFIED | `.factory/specs/architecture/SS-04-plugin-ecosystem.md` |

## Indexes (state-manager activity; F2/F3)

| Change | File |
|--------|------|
| MODIFIED | `.factory/specs/behavioral-contracts/BC-INDEX.md` |
| MODIFIED | `.factory/specs/verification-properties/VP-INDEX.md` |
| MODIFIED | `.factory/stories/STORY-INDEX.md` |
| MODIFIED | `.factory/specs/architecture/ARCH-INDEX.md` |

## New Stories and Epic (F3 story-writer activity)

| Change | File |
|--------|------|
| NEW | `.factory/stories/epics/E-16-hook-plugin-capability-extensions.md` |
| NEW | `.factory/stories/S-16.01-block-ai-attribution-posttooluse-head-verification.md` |
| NEW | `.factory/stories/S-16.02-block-ai-attribution-pretooluse-file-read-arm.md` |

## DEPENDENT — Regression Baseline (must not change)

| File | Reason |
|------|--------|
| `crates/hook-sdk/src/result.rs` | VP-038 ABI contract |
| `crates/factory-dispatcher/src/host/exec_subprocess.rs` | Host function; no changes needed |
| `crates/factory-dispatcher/src/host/read_file.rs` | Host function; no changes needed |
| `.factory/specs/behavioral-contracts/ss-07/BC-7.03.002.md` | Attribution command-string gate; content frozen |
| `.factory/specs/behavioral-contracts/ss-07/BC-7.03.003.md` | Co-Authored-By detection contract; content frozen |
| `.factory/specs/behavioral-contracts/ss-07/BC-7.03.004.md` | Generated-with/email detection contract; content frozen |
| `.factory/specs/behavioral-contracts/ss-07/BC-7.03.005.md` | jq-absent graceful no-op (bash predecessor); content frozen |
| `.factory/specs/verification-properties/VP-038.md` | ABI stability VP; must remain frozen |
| `plugins/vsdd-factory/hooks/block-ai-attribution.sh` | Bash predecessor; superseded; do not modify |
