---
document_type: prd-supplement-error-taxonomy
level: L3
version: "1.0"
status: draft
producer: product-owner
timestamp: 2026-09-05T00:00:00Z
phase: F2
inputs:
  - .factory/specs/prd.md
  - .factory/cycles/v1.0-brownfield-backfill/S-25.02-f2-prd-delta.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.18.006.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.18.007.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.18.008.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.18.009.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.18.011.md
input-hash: "b2f670f"
traces_to: .factory/specs/prd.md
---

# Error Taxonomy: vsdd-factory

> PRD supplement — extracted from PRD §5 (Error Taxonomy). Referenced by: implementer, test-writer.
> Materializes the `.factory/specs/prd-supplements/error-taxonomy.md` reference that `prd.md`
> §5.1/§5b have carried since before this document existed (a pre-existing dangling reference,
> not introduced by S-25.02 — flagged by the S-25.02 F2 PRD delta §7 and closed here per
> F-S2502-F2-006). This file's error categories mirror `prd.md` §5.1's summary table exactly, plus
> the `E-SHD-NNN` category `prd.md` §5.1 does not yet enumerate.

## Error Categories

| Category Code | Category | Description |
|--------------|----------|-------------|
| REG | Registry errors | Hook registry parsing/validation failures (schema version, tool regex, unknown fields) |
| PAY | Payload errors | Malformed or incomplete tool-call payloads reaching the dispatcher |
| CAP | Capability denial | Host-function calls denied by the plugin's declared capability set |
| PLG | Plugin execution | Timeout, crash, or resource-exhaustion outcomes during WASM plugin execution |
| SNK | Sink errors | Event-sink (JSONL/OTLP) write or routing failures |
| ACT | Activation errors | Platform/binary/hooks.json activation failures |
| HK | Hook gate blocks | PreToolUse gate blocks (secrets, destructive commands, branch protection) |
| SHD | Shard management errors | Layer-2 artifact-sharding failures (rotation, backfill, migration) — native dispatcher `HookResult::Error` outcomes, not process exit codes |

## Error Catalog

| Error Code | Category | Severity | Exit Code / HookResult | Message Format |
|-----------|----------|----------|-----------------------|---------------|
| E-REG-001 | Registry errors | broken | exit 0 | `schema_version mismatch: expected <N>, found <M>` |
| E-REG-002 | Registry errors | broken | exit 0 | `invalid tool regex in entry <name>: <regex-error>` |
| E-REG-003 | Registry errors | broken | exit 0 | `unknown field <field> in hooks-registry.toml entry <name>` |
| E-PAY-001 | Payload errors | broken | exit 0 | `missing event_name in payload` |
| E-PAY-002 | Payload errors | broken | exit 0 | `invalid JSON in tool-call payload: <parse-error>` |
| E-CAP-001 | Capability denial | blocked | exit 2 | `exec_subprocess denied: capability not declared for plugin <name>` |
| E-CAP-002 | Capability denial | blocked | exit 2 | `shell-bypass not acknowledged for command <cmd>` |
| E-CAP-003 | Capability denial | blocked | exit 2 | `setuid refused for plugin <name>` |
| E-PLG-001 | Plugin execution | degraded | exit 0 | `plugin <name> timeout (epoch budget exceeded)` |
| E-PLG-002 | Plugin execution | degraded | exit 0 | `plugin <name> timeout (fuel budget exceeded: cap=<N>, consumed=<M>)` |
| E-PLG-003 | Plugin execution | degraded | exit 0 | `plugin <name> crashed (trap): <trap-reason>` |
| E-SNK-001 | Sink errors | degraded | non-blocking | `sink <name> queue full, event dropped` |
| E-SNK-002 | Sink errors | degraded | non-blocking | `sink <name> write failure: <io-error>` |
| E-SNK-003 | Sink errors | degraded | non-blocking | `unknown sink driver type <type>` |
| E-ACT-001 | Activation errors | broken | non-zero | `unsupported platform <platform>` |
| E-ACT-002 | Activation errors | broken | non-zero | `dispatcher binary missing for platform <platform>` |
| E-ACT-003 | Activation errors | broken | non-zero | `hooks.json write failure: <io-error>` |
| E-HK-001 | Hook gate blocks | blocked | exit 2 | `secret detected in <path>: <pattern-name>` |
| E-HK-002 | Hook gate blocks | blocked | exit 2 | `destructive command blocked: <command>` |
| E-HK-003 | Hook gate blocks | blocked | exit 2 | `branch protection violation: <branch>` |
| E-SHD-001 | Shard management errors | broken | `HookResult::Error` | `seal-rename failure for <artifact>: <io-error>` (BC-1.18.006 EC-003; disk-full/permission error mid-roll) |
| E-SHD-002 | Shard management errors | broken | `HookResult::Error` | `shard-index missing or corrupt for <artifact>: <parse-error>` (BC-1.18.007 EC-005) |
| E-SHD-003 | Shard management errors | broken | `HookResult::Error` | `backfill-split content-preservation verification failed for <artifact>: <mismatch-detail>` (BC-1.18.008 EC-004, Postcondition 6) |
| E-SHD-004 | Shard management errors | broken | `HookResult::Error` | `rotate_changelog invocation failed for <artifact>: <io-error>` (BC-1.18.009 EC-003) |
| E-SHD-005 | Shard management errors | broken | `HookResult::Error` / migration abort | `B2 BC-INDEX migration verification failed (content-preservation or independent-census mismatch): <mismatch-detail>; original body left untouched` (BC-1.18.011 Postconditions 2/4, EC-001/EC-004) |

All dispatcher-level errors except E-CAP and E-PLG exit 0 (non-blocking per NFR-REL-001).
`E-SHD-NNN` errors are native-gate `HookResult::Error` outcomes (a PreToolUse hook result, not a
process exit code) — dispatcher exit-code semantics do not apply to this category, per BC-1.18.006
through BC-1.18.011's own Postcondition/Edge-Case tables.

## Severity Definitions

| Severity | Meaning | Exit Code / HookResult Impact |
|----------|---------|-------------------------------|
| broken | Cannot continue | Non-zero exit, OR (for E-SHD) `HookResult::Error` — the triggering write is blocked and no partial state is left authoritative |
| blocked | Explicitly denied by policy | Exit 2 |
| degraded | Partial result possible | Zero exit with warnings, non-blocking |
| cosmetic | Formatting/display issue | Zero exit |

## Changelog

| Version | Date | Author | Change |
|---------|------|--------|--------|
| 1.0 | 2026-09-05 | product-owner | Initial materialization of this supplement file (F-S2502-F2-006, MEDIUM). Mirrors `prd.md` §5.1's existing 7 categories (REG/PAY/CAP/PLG/SNK/ACT/HK) verbatim and adds the 8th category, `E-SHD-NNN` (Shard management errors), previously defined only in the S-25.02 F2 PRD-delta doc §4. Includes `E-SHD-001` (seal-rename failure), `E-SHD-002` (missing/corrupt shard-index), `E-SHD-003` (backfill-split content-preservation failure), `E-SHD-004` (`rotate_changelog` invocation failure), and NEW `E-SHD-005` (B2 BC-INDEX migration verification failure — content-preservation OR independent-census mismatch, BC-1.18.011). Resolves the pre-existing dangling `prd-supplements/error-taxonomy.md` reference `prd.md` §5.1/§5b have carried since before this file existed (not introduced by S-25.02; flagged by the F2 PRD-delta doc §7, closed here). Companion registry entry `prd-supplement` added to `plugins/vsdd-factory/config/artifact-path-registry.yaml` in the same burst (mechanical prerequisite — the write was blocked `ARTIFACT_PATH_UNREGISTERED` with no prior entry for this path pattern). |
