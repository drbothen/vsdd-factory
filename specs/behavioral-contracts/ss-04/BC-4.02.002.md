---
document_type: behavioral-contract
level: L3
version: "1.1"
status: draft
producer: codebase-analyzer
timestamp: 2026-04-25T00:00:00
phase: 1.4b
inputs: [bc-id-mapping.md, pass-3-behavioral-contracts-deep-r1.md]
input-hash: "[pending-recompute]"
traces_to: bc-id-mapping.md
origin: brownfield
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts-deep-r1.md:449"
subsystem: "SS-04"
capability: "CAP-TBD"
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

# Behavioral Contract BC-4.02.002: Adapter exit-code mapping: 0 → Continue, 2 → Block (reason=full stderr trimmed to 4 KiB OR synthetic), other → Error (message includes script path + code + stderr)

## Description

For every bash exit code N, the legacy-bash-adapter produces a deterministic `HookResult`. N==0 → Continue; N==2 → Block (reason = full stderr output, trimmed, up to a 4 KiB UTF-8-character-boundary-safe cap; or `"legacy bash hook {script_path} blocked"` synthetic if stderr is empty); any other N → Error with message `"legacy bash hook {script_path} exited with code {N}: {stderr}"`.

**rc.12 note:** Prior to v1.0.0-rc.12, the exit-2 path captured only the first non-empty stderr line. rc.12 changed `crates/hook-plugins/legacy-bash-adapter/src/lib.rs` to capture the full stderr (trimmed) up to a 4 KiB `floor_char_boundary`-safe cap. Stderr exceeding 4096 bytes is truncated with a `…[truncated]` suffix appended. See Source Evidence for updated path annotation.

## Preconditions

1. The bash subprocess has exited with code N.
2. The adapter has captured stderr.

## Postconditions

1. `N == 0` → `HookResult::Continue`.
2. `N == 2` and stderr non-empty → `Block { reason = full stderr output, trimmed, up to a 4 KiB UTF-8-safe cap }`. If stderr exceeds 4096 bytes, the reason string is truncated at the `floor_char_boundary(4096)` byte boundary and `…[truncated]` is appended.
3. `N == 2` and stderr empty → `Block { reason = "legacy bash hook {script_path} blocked" }` synthetic.
4. Other N → `HookResult::Error { message = "legacy bash hook {script_path} exited with code {N}: {stderr}" }`.

## Invariants

1. Mapping is total over `i32`.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | exit 2 with multi-line stderr | Full stderr is used as the reason (trimmed, up to 4 KiB cap). All lines are preserved, not just the first. (Pre-rc.12 behavior was first-line only; changed in v1.0.0-rc.12.) |
| EC-002 | exit 2 with empty stderr | Synthetic reason `"legacy bash hook {script_path} blocked"` |
| EC-003 | exit 2 with stderr > 4096 bytes | Reason is first 4096 bytes of trimmed stderr at `floor_char_boundary(4096)` (UTF-8-safe boundary), with `…[truncated]` appended. |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `N=0` | Continue | happy-path |
| `N=2`, stderr "denied: foo" | Block { reason: "denied: foo" } | edge-case |
| `N=2`, stderr "line1\nline2\nline3" | Block { reason: "line1\nline2\nline3" } (full multi-line stderr returned, not just first line) | edge-case-multiline |
| `N=2`, stderr is 5000 bytes of "x" | Block { reason: "xxx...xxx…[truncated]" } where reason is 4096 bytes truncated at UTF-8 boundary with `…[truncated]` suffix | edge-case-truncation |
| `N=42`, stderr "boom" | Error { message: "legacy bash hook {path} exited with code 42: boom" } | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6b) | | |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD (anchor in Phase 1.5) |
| L2 Domain Invariants | TBD |
| Architecture Module | SS-04 — `crates/hook-plugins/legacy-bash-adapter/src/lib.rs` (exit-code match arms) |
| Stories | TBD (re-anchor in Phase 1.8 from S-N.MM stories) |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `crates/hook-plugins/legacy-bash-adapter/src/lib.rs` (exit-2 path; line numbers are not authoritative — use the exit-code match arm as the canonical reference) |
| **Confidence** | HIGH (5 distinct test cases pin every branch) |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-behavioral-contracts-deep-r1.md` line `449` |
| **rc.12 change** | v1.0.0-rc.12 (4cf59bc on develop) changed the exit-2 path from first-stderr-line capture to full-stderr capture (trimmed, `floor_char_boundary`-safe 4 KiB cap). Pre-rc.12 line numbers `103-119` are no longer authoritative; refer to the exit-2 match arm by function structure. |

#### Evidence Types Used

- assertion (5 unit tests pin every arm)

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
| 1.0 | 2026-04-25 | Initial brownfield extraction (codebase-analyzer; pass-3-behavioral-contracts-deep-r1.md). Postcondition 2 captured pre-rc.12 "first non-empty stderr line" behavior. |
| 1.1 | 2026-05-06 | D-326 (D-1) — rc.12 alignment: exit-2 path now captures full stderr trimmed to 4 KiB UTF-8-safe cap (`floor_char_boundary`), not first line only. Title, Description, Postcondition 2, EC-001, and test vectors updated. EC-003 added for truncation edge case (stderr > 4096 bytes). Source Evidence path annotation updated to note rc.12 change at v1.0.0-rc.12 (4cf59bc). Pre-rc.12 "first stderr line" language preserved in EC-001 historical note and Source Evidence rc.12 change row. |
