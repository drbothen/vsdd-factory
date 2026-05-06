---
document_type: behavioral-contract
level: L3
version: "1.2"
status: draft
producer: codebase-analyzer
timestamp: 2026-04-25T00:00:00
phase: 1.4b
inputs: [bc-id-mapping.md, pass-3-behavioral-contracts.md]
input-hash: "[pending-recompute]"
traces_to: bc-id-mapping.md
origin: brownfield
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts.md:374"
subsystem: "SS-04"
capability: "CAP-009"
lifecycle_status: active
introduced: v1.0.0-beta.4
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# Behavioral Contract BC-4.01.003: legacy-bash-adapter maps bash exit codes to HookResult

## Description

The legacy-bash-adapter maps bash subprocess exit codes deterministically into `HookResult` variants. Exit 0 → Continue; exit 2 → Block (reason taken from the full stderr output, trimmed, up to a 4 KiB UTF-8-character-boundary-safe cap; or synthetic if stderr is empty); any other non-zero exit code → Error with stderr in the message.

**rc.12 note:** Prior to v1.0.0-rc.12, the exit-2 path captured only the first non-empty stderr line. rc.12 changed `crates/hook-plugins/legacy-bash-adapter/src/lib.rs` to capture the full stderr (trimmed) up to a 4 KiB `floor_char_boundary`-safe cap. Stderr exceeding 4096 bytes is truncated with a `…[truncated]` suffix appended. This is the same root-cause change as documented in BC-4.02.002 (D-1 / D-2 co-located fix per audit E-10-rc12-format-audit.md).

## Preconditions

1. A bash hook subprocess has completed with some exit code N.
2. The adapter has captured stdout/stderr.

## Postconditions

1. If `N == 0` → `HookResult::Continue`.
2. If `N == 2` → `HookResult::Block { reason }` where reason is the full stderr output, trimmed, up to a 4 KiB UTF-8-safe cap. If stderr exceeds 4096 bytes, the reason string is truncated at `floor_char_boundary(4096)` and `…[truncated]` is appended. If stderr is empty, reason is empty (or a synthetic value per adapter implementation). (Pre-rc.12 behavior was to use only the first non-empty stderr line; changed in v1.0.0-rc.12.)
3. If `N` is any other non-zero value → `HookResult::Error { message }` populated with stderr content.

## Invariants

1. Exit-code → HookResult mapping is total over `i32` and pure (only depends on N + stderr).

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | exit 2 with empty stderr | Block with empty reason (synthetic or empty handled by adapter) |
| EC-002 | exit 2 with multi-line stderr | Full stderr returned as reason (all lines preserved, trimmed, up to 4 KiB cap). Pre-rc.12 behavior was first-line only; changed in v1.0.0-rc.12. |
| EC-003 | exit 2 with stderr > 4096 bytes | Reason truncated at `floor_char_boundary(4096)` with `…[truncated]` appended. |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Bash exits 0 | Continue | happy-path |
| Bash exits 2 with stderr "blocked: foo" | Block { reason: "blocked: foo" } | edge-case |
| Bash exits 1 with stderr "boom" | Error { message includes "boom" } | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6b) | | |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-009 — Author and publish WASM hook plugins using the Rust SDK |
| Capability Anchor Justification | BC-4.01.003 governs the legacy-bash-adapter's brownfield extraction scope: the bash-subprocess exit-code → HookResult mapping that preserves pre-WASM hook behavior during the bash→WASM migration. Per CAP-009 (capabilities.md §CAP-009), the SDK is the surface through which plugin authors ship `.wasm` hooks without touching the dispatcher; the legacy-bash-adapter is the adapter layer that makes existing bash hooks first-class participants in that surface. This BC is the contract for how the adapter maps bash exit codes (0/2/other) to the HookResult variants (`Continue`/`Block`/`Error`) that the dispatcher consumes via the same CAP-009 plugin SDK ABI. Anchoring to CAP-009 ensures the bash-adapter's behavior contract is traceable to the same capability as its native-WASM sibling (BC-4.09.001 is also anchored to CAP-009 for the same reason). |
| L2 Domain Invariants | TBD |
| Architecture Module | SS-04 — `crates/hook-plugins/legacy-bash-adapter/src/lib.rs` (`adapter_logic` exit-code switch) |
| Stories | TBD (re-anchor in Phase 1.8 from S-N.MM stories) |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `crates/hook-plugins/legacy-bash-adapter/src/lib.rs` (adapter_logic bash exit-code switch) |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-behavioral-contracts.md` line `374` |

#### Evidence Types Used

- assertion (exit-code match arms in adapter_logic)
- documentation (design doc reference)

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | TBD (Phase 1.6b will refine) |
| **Global state access** | TBD |
| **Deterministic** | TBD |
| **Thread safety** | TBD |
| **Overall classification** | TBD |

#### Refactoring Notes

(TBD — to be assessed in Phase 1.6b verification properties pass)

## Changelog

| Version | Date | Description |
|---------|------|-------------|
| 1.0 | 2026-04-25 | Initial brownfield extraction (codebase-analyzer; pass-3-behavioral-contracts.md). Postcondition 2 captured pre-rc.12 "first non-empty stderr line" behavior. |
| 1.1 | 2026-05-06 | D-326 (D-2) — rc.12 alignment: same root cause as BC-4.02.002 D-1. Exit-2 path now captures full stderr trimmed to 4 KiB UTF-8-safe cap, not first line only. Description, Postcondition 2 updated. EC-002 and EC-003 added for multi-line and truncation edge cases. Pre-rc.12 "first non-empty stderr line" preserved in Postcondition 2 historical parenthetical and EC-002 historical note. Change at v1.0.0-rc.12 (4cf59bc). |
| 1.2 | 2026-05-06 | D-328 — E-10 pass-5 F-12 fix: resolved CAP-TBD → CAP-009. Frontmatter `capability` field updated. Traceability `L2 Capability` row updated from `TBD (anchor in Phase 1.5)` to `CAP-009 — Author and publish WASM hook plugins using the Rust SDK`. Capability Anchor Justification added: legacy-bash-adapter (brownfield extraction scope) bridges pre-existing bash hooks into the CAP-009 WASM plugin SDK surface; this BC is the contract for the exit-code→HookResult mapping that makes bash hooks first-class CAP-009 participants. Anchors to the same capability as sibling BC-4.09.001. |
