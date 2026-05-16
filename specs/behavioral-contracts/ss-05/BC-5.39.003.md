---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: story-writer
timestamp: 2026-05-16T00:00:00Z
phase: section-12-step-3
cycle: brownfield-backfill
inputs:
  - .factory/cycles/v1.0-brownfield-backfill/s-15.03-wave-plan-2026-05-15.md
  - .factory/cycles/v1.0-brownfield-backfill/architect-m2-2026-05-16.md
  - .factory/cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md
input-hash: "ad1c745"
traces_to: .factory/cycles/v1.0-brownfield-backfill/s-15.03-wave-plan-2026-05-15.md
extracted_from: .factory/stories/S-15.07-validate-index-cite-refresh.md
origin: brownfield
subsystem: "SS-05"
capability: "E-12"
lifecycle_status: draft
introduced: v1.0-brownfield-backfill
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
bc_id: BC-5.39.003
section: "5.39"
last_amended: 2026-05-16
---

# BC-5.39.003: validate-index-cite-refresh hook MUST block on stale 4-index version citations in ARCH-INDEX, STATE.md, and INDEX.md

## Description

The `validate-index-cite-refresh` WASM hook enforces that any Edit/Write to `ARCH-INDEX.md`
does not leave stale version citations for any of the four canonical factory indexes
(BC-INDEX, VP-INDEX, STORY-INDEX, ARCH-INDEX). The hook fires PostToolUse and compares every
version string cited in the ARCH-INDEX body against the live `version:` frontmatter field of
the corresponding index file. If any cited version is numerically older than the live version,
the hook emits a `block_with_fix` signal naming the stale cite and the current version.

Additionally, the hook enforces D-429(b) cross-cell sibling sweep: STATE.md trajectory-bearing
cells and the brownfield-backfill INDEX.md Convergence Status cells must not cite 4-index
versions that disagree with the live index frontmatter values.

This BC closes the recurring class of adversary findings in the F5 engine-discipline cycle
(passes 25–74) where ARCH-INDEX body cites would drift stale after a fix-burst bumped one
of the four indexes. D-405(c) codified the primary staleness gate; D-429(b) extended it to
cross-cell agreement.

## Preconditions

1. A PostToolUse Edit/Write event has fired on a file matching the `ARCH-INDEX.md` file pattern.
2. The dispatcher has invoked the `validate-index-cite-refresh` WASM plugin with the write payload.
3. The four canonical index files are readable via `host::read_file`:
   - `.factory/specs/behavioral-contracts/BC-INDEX.md`
   - `.factory/specs/verification-properties/VP-INDEX.md`
   - `.factory/stories/STORY-INDEX.md`
   - `.factory/specs/architecture/ARCH-INDEX.md`
4. `host::read_file` is available with `max_bytes = 65536` and `timeout_ms = 2000` per call.
5. STATE.md (`.factory/STATE.md`) and the brownfield cycle INDEX.md
   (`.factory/cycles/v1.0-brownfield-backfill/INDEX.md`) are readable for D-429(b) cross-cell check.

## Postconditions

1. If ALL of the following hold, the hook emits `HookResult::Continue` (pass):
   - Every version string in ARCH-INDEX body matching the pattern
     `(BC-INDEX|VP-INDEX|STORY-INDEX|ARCH-INDEX) v(\d+)\.(\d+)` equals the `version:` frontmatter
     field of the corresponding index file (numeric comparison: both major and minor parts).
   - Every version string in STATE.md matching the same pattern agrees with the corresponding
     live index frontmatter value.
   - Every version string in the brownfield-backfill INDEX.md matching the same pattern agrees
     with the corresponding live index frontmatter value.
2. If ANY cited version string in ARCH-INDEX body is numerically older than the live index
   version (cited.major < live.major, or cited.major == live.major AND cited.minor < live.minor),
   the hook emits `HookResult::BlockWithFix` with:
   - A message naming the specific stale cite: which index file, cited version, live version.
   - A remediation instruction: update the ARCH-INDEX cite to the live version.
3. If ANY cross-cell version string in STATE.md or INDEX.md is numerically older than the live
   index version, the hook emits `HookResult::BlockWithFix` citing the specific source document,
   the mismatched index name, the cited version, and the live version.
4. If `host::read_file` returns an error for any index file (HostError::CapabilityDenied,
   HostError::Timeout, HostError::OutputTooLarge, or any other error), the hook emits
   `HookResult::Continue` and logs a warning via `host::log_warn` — a read failure is NOT
   treated as a structural violation.
5. Version comparison is strictly numeric: `v2.9` is older than `v2.10`. Both major and minor
   parts are parsed as u32 integers before comparison.

## Invariants

1. The hook NEVER writes to any file. It has no `write_file` capability in its registry entry.
   It is a read-only post-write validator.
2. The hook fires PostToolUse only — it never prevents a write; it signals AFTER the write
   has completed. The dispatcher records the block signal; the author must correct and re-write.
3. Version extraction is limited to exactly four index name tokens:
   `BC-INDEX`, `VP-INDEX`, `STORY-INDEX`, `ARCH-INDEX`. No other index name patterns match.
4. The hook does not validate ARCH-INDEX changelog entries, frontmatter changelog arrays, or
   ADR table content — only body prose version-cite strings and the cross-cell documents.
5. All `host::read_file` calls are fail-open: read errors produce Continue + log_warn, not Block.
   The total timeout budget across all five reads is bounded by the registry `timeout_ms = 5000`
   limit enforced by the dispatcher.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | ARCH-INDEX body has no 4-index version-cite strings | Hook emits Continue — vacuously passing |
| EC-002 | Cited version equals live version exactly | Hook emits Continue — equality is not staleness |
| EC-003 | Cited version is numerically newer than live version | Hook emits Continue — only older cites are stale |
| EC-004 | Any index file is unreadable (HostError of any kind) | Hook emits Continue + log_warn; does not block |
| EC-005 | ARCH-INDEX cites `BC-INDEX v1.05` when live BC-INDEX is v2.24 | BlockWithFix: "ARCH-INDEX cites BC-INDEX v1.05 but live version is v2.24 — update cite to v2.24" |
| EC-006 | STATE.md trajectory cell cites stale STORY-INDEX version | BlockWithFix citing source="STATE.md" and the version discrepancy |
| EC-007 | Malformed version string in ARCH-INDEX body (e.g. `BC-INDEX vX.Y`) | Hook skips the malformed cite; logs warning; does not block |
| EC-008 | Multiple stale cites in one Edit (BC-INDEX and STORY-INDEX both stale) | Single BlockWithFix message enumerating all violations |
| EC-009 | INDEX.md at brownfield path does not exist | Continue + log_warn for INDEX.md read failure; fail-open per invariant 5 |
| EC-010 | ARCH-INDEX frontmatter `version:` field is absent or malformed | ARCH-INDEX self-cite check skipped; log_warn; peer-index cite checks unaffected |

## Canonical Test Vectors

| Scenario | Input Condition | Expected Hook Output | Decision |
|----------|----------------|---------------------|----------|
| All cites current | ARCH-INDEX body cites `BC-INDEX v2.24`, `VP-INDEX v1.97`, `STORY-INDEX v3.31`; each index frontmatter matches exactly | `HookResult::Continue` | PASS |
| Stale BC-INDEX cite | ARCH-INDEX body cites `BC-INDEX v1.05`; live BC-INDEX frontmatter `version: "2.24"` | `HookResult::BlockWithFix` naming BC-INDEX, v1.05, v2.24 | BLOCK |
| Stale VP-INDEX cite | ARCH-INDEX body cites `VP-INDEX v1.80`; live VP-INDEX frontmatter `version: "1.97"` | `HookResult::BlockWithFix` naming VP-INDEX, v1.80, v1.97 | BLOCK |
| Stale STORY-INDEX cite | ARCH-INDEX body cites `STORY-INDEX v3.28`; live STORY-INDEX frontmatter `version: "3.31"` | `HookResult::BlockWithFix` naming STORY-INDEX, v3.28, v3.31 | BLOCK |
| STATE.md cross-cell stale | ARCH-INDEX cites are current; STATE.md trajectory cell contains `STORY-INDEX v3.28` | `HookResult::BlockWithFix` citing source=STATE.md | BLOCK |
| INDEX.md cross-cell stale | ARCH-INDEX cites are current; INDEX.md Convergence Status cell contains `BC-INDEX v2.20` | `HookResult::BlockWithFix` citing source=INDEX.md | BLOCK |
| BC-INDEX unreadable | `host::read_file` returns `HostError::CapabilityDenied` for BC-INDEX.md | `HookResult::Continue` + `host::log_warn` | PASS (fail-open) |
| Multiple stale cites | ARCH-INDEX cites both BC-INDEX and STORY-INDEX stale | Single `HookResult::BlockWithFix` enumerating both violations | BLOCK |
| Version equal | ARCH-INDEX cites `STORY-INDEX v3.31`; live STORY-INDEX `version: "3.31"` | `HookResult::Continue` | PASS |
| Minor version newer (cited > live, impossible but tested) | ARCH-INDEX cites `BC-INDEX v2.25`; live BC-INDEX `version: "2.24"` | `HookResult::Continue` — not stale | PASS |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (pending) | Block Invariant — hook emits BlockWithFix when cited version < live version | bats integration test (fail-stale-bc-index fixture) |
| (pending) | Pass Invariant — hook emits Continue when all cited versions match live versions | bats integration test (pass-all-current fixture) |
| (pending) | Fail-open Invariant — hook emits Continue when index file is unreadable | bats integration test (fail-open-missing-index fixture) |

VP IDs are pending VP-INDEX allocation by state-manager at post-merge burst.

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | E-12 (Engine Governance — cite-refresh automation sub-capability) |
| Capability Anchor Justification | E-12 governs factory engine discipline automation. This BC formalizes the PostToolUse gate that mechanically prevents the stale-cite drift class codified in D-405(c) and D-429(b). The hook targets ARCH-INDEX.md writes — a governance artifact, not a runtime subsystem artifact. |
| Architecture Module | `crates/hook-plugins/validate-index-cite-refresh/` (Rust WASM plugin); `plugins/vsdd-factory/hooks-registry.toml` (registry entry); `plugins/vsdd-factory/hook-plugins/validate-index-cite-refresh.wasm` (compiled binary) |
| D-NNN Sub-Clauses Closed | D-405(c) (ARCH-INDEX version-cite staleness gate); D-429(b) (cross-cell sibling sweep for STATE.md and INDEX.md) |
| Stories | S-15.07 |

## Related BCs

- BC-5.39.001 — governs the per-story adversarial convergence loop (3-CLEAN gate); S-15.07 must
  achieve 3-CLEAN per BC-5.39.001 before PR dispatch
- BC-5.39.002 — governs adversary scope limits (out-of-scope findings deferred)
- BC-4.11.001 — validates write targets against artifact-path-registry (sister PostToolUse hook;
  structural analog for path validation vs version-cite validation)

## Architecture Anchors

- `crates/hook-plugins/validate-index-cite-refresh/src/lib.rs` — hook implementation (pure logic functions + effectful orchestration)
- `crates/hook-sdk/src/host.rs` — `host::read_file(path, max_bytes, timeout_ms)` API consumed by this hook
- `plugins/vsdd-factory/hooks-registry.toml` — PostToolUse registration with `file_pattern = "ARCH-INDEX.md"`

## Story Anchor

S-15.07 — v1.0-brownfield-backfill (S-15.03 PRIORITY-A M2 Wave-1)

## Changelog

| Version | Date | Description |
|---------|------|-------------|
| 1.0 | 2026-05-16 | Initial authoring (story-writer; brownfield-backfill S-15.03 M2 wave-1 story authoring). Anchors D-405(c) + D-429(b). BC-5.39.003 allocated as next monotonic ID after BC-5.39.002 in ss-05/. |
