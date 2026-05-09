---
document_type: behavioral-contract
level: L3
version: "1.2"
status: draft
producer: codebase-analyzer
timestamp: 2026-04-25T00:00:00
phase: 1.4b
inputs: [bc-id-mapping.md, pass-3-deep-rust-tests.md]
input-hash: "[pending-recompute]"
traces_to: bc-id-mapping.md
origin: brownfield
extracted_from: ".factory/phase-0-ingestion/pass-3-deep-rust-tests.md:596"
subsystem: "SS-01"
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

# Behavioral Contract BC-1.07.006: factory-dispatcher::loads_generated_registry_from_disk — hook count is within sanity bounds (>20 and <100)

## Description

Integration test: After `plugins/vsdd-factory/hooks-registry.toml` is loaded through the production `Registry::load` codepath, the number of parsed hook entries is verified to be within a sanity range. The lower bound (>20) guards against silent elision of the v0.79.x inventory; the upper bound (<100) guards against duplicated-emit bugs in the generator. Both bounds are asserted in `loads_generated_registry_from_disk` (lines 49–59).

## Preconditions

1. `plugins/vsdd-factory/hooks-registry.toml` exists and parses successfully (BC-1.07.005 satisfied).
2. `registry.hooks` is populated.

## Postconditions

1. `registry.hooks.len() > 20` — the full v0.79.x hook inventory is present.
2. `registry.hooks.len() < 100` — no duplicated-emit anomaly has inflated the count.

## Invariants

1. The hook count sanity bounds are CI-enforced; any registry that falls outside the [21, 99] range is a build-time error.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | (No edge cases captured in Phase 0 extraction; to be added in Phase 1.5/test-writer pass) | TBD |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `plugins/vsdd-factory/hooks-registry.toml` (current production file, ~30 hooks) | `registry.hooks.len()` is in range [21, 99] | happy-path |
| TBD | TBD | edge-case |
| TBD | TBD | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6b) | | |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD (anchor in Phase 1.5) |
| L2 Domain Invariants | TBD |
| Architecture Module | SS-01 — `crates/factory-dispatcher/tests/loads_legacy_registry.rs` |
| Stories | TBD (re-anchor in Phase 1.8 from S-N.MM stories) |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `crates/factory-dispatcher/tests/loads_legacy_registry.rs::loads_generated_registry_from_disk` (lines 34–60) — **Note (L-P21-001):** `every_entry_carries_a_script_path` was fabricated at Phase 0 ingestion; function was never committed. The current test verifies parse + count only; dedicated per-entry script_path assertion is pending. |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-deep-rust-tests.md` line `596` |

#### Evidence Types Used

- assertion (integration test)

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

## Amendment 2026-05-08 (v1.1 → v1.2 — F-P23-003: H1 title + body rebrand to cite real production symbols)

**Driver:** F-P23-003 (pass-23 finding). H1 title cited `every_entry_carries_a_script_path` — a fabricated symbol (0 grep matches). Fix-burst-21 sub-burst 2 only patched §Source Evidence; H1 and BC-INDEX row still carried the fabricated name. The actual test fn `loads_generated_registry_from_disk` asserts hook count sanity bounds (lines 49–59), not per-entry `script_path` values.

**H1 before:** `factory-dispatcher::loads_legacy_registry::every_entry_carries_a_script_path — every entry has plugin_config.script_path matching \`hooks/<name>.sh\``

**H1 after:** `factory-dispatcher::loads_generated_registry_from_disk — hook count is within sanity bounds (>20 and <100)`

**Body changes:** Description, Preconditions, Postconditions, and Invariants rewritten to reflect the actual test assertions (count lower bound >20, count upper bound <100). Canonical test vector updated to match real inputs/outputs.

**Verification:** `grep -rn "every_entry_carries_a_script_path" /Users/jmagady/Dev/vsdd-factory/.factory/specs/` returns only Amendment/Changelog historical mentions, not active body or H1.

**Refs:** F-P23-003, L-P21-001, POLICY 1 (BC-INDEX title sync), POLICY 4 (anchors must be grep-verifiable), POLICY 7 (H1 is title source of truth).

## Amendment 2026-05-08 (v1.0 → v1.1 — L-P21-001 retroactive sweep: Source Evidence fabricated function corrected)

**Driver:** L-P21-001 retroactive corpus sweep. §Source Evidence Path cited `loads_legacy_registry.rs::every_entry_carries_a_script_path` — this function does not exist. It was cited from Phase 0 ingestion (pass-3-deep-rust-tests.md line 596) and was never committed to the test file.

**Verification:** `grep -n "fn every_entry_carries_a_script_path" crates/factory-dispatcher/tests/loads_legacy_registry.rs` → no matches. Actual test: `loads_generated_registry_from_disk` (line 34).

**Changes made:**
- §Source Evidence Path: fabricated function replaced with `loads_generated_registry_from_disk` (lines 34-60) with L-P21-001 note.
- Frontmatter `version:` bumped `"1.0"` → `"1.1"`.
