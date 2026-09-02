---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: product-owner
timestamp: 2026-09-02T00:00:00Z
phase: F2
cycle: v1.0-feature-engine-discipline-pass-1
inputs:
  - .factory/specs/architecture/decisions/ADR-049-last-amended-write-path-durable-fix-current-entry-plus-changelog-sequence.md
  - .factory/stories/S-15.03-index-cite-refresh-hook.md
  - .factory/specs/domain-spec/capabilities.md
input-hash: "0d49431"
traces_to: .factory/specs/architecture/decisions/ADR-049-last-amended-write-path-durable-fix-current-entry-plus-changelog-sequence.md
origin: greenfield
extracted_from: null
subsystem: "SS-04"
capability: "CAP-042"
lifecycle_status: draft
introduced: v1.0-feature-engine-discipline-pass-1
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
bc_id: BC-4.18.001
section: "4.18"
last_amended: "2026-09-02 (v1.0) — Initial authoring (product-owner; ADR-049 Phase B; S-15.03 last_amended Write-Path Durable Fix)."
---

# BC-4.18.001: Bash-Adapter WASM Fuel-Budget Relief on `last_amended`/`changelog:` Edits — No Fuel Exhaustion After the Write-Path Fix

## Description

After BC-5.45.001's write-path discipline and BC-10.13.001's one-time migration have both
shipped, editing any of the five ADR-049-governed files (`STORY-INDEX.md`, `BC-INDEX.md`,
`ARCH-INDEX.md`, `VP-INDEX.md`, `STATE.md`) — via `Edit` or `Write` — MUST NOT exhaust the
`legacy-bash-adapter`-hosted WASM validators' fuel budget (`DEFAULT_FUEL_CAP`,
`crates/factory-dispatcher/src/invoke.rs`, ADR-042 §Decision 1). This BC is the regression-proof
counterpart to the write-path fix: it does not change the fuel cap itself, it verifies that the
root-cause payload growth the cap was being exhausted by (the unbounded `last_amended` mega-line)
can no longer occur, directly targeting the 743-fuel-timeouts/day symptom observed 2026-09-02 that
motivated S-15.03's Scope Extension.

## Preconditions

1. BC-5.45.001's write-path discipline is in force (every future write to `last_amended`
   overwrites current-entry-only and prepends to `changelog:` rather than bracket-wrapping).

2. BC-10.13.001's one-time migration has run against the fixture/target file (so `STORY-INDEX.md`
   carries `changelog:`, and the D-1144 escape defect on `BC-INDEX.md`/`ARCH-INDEX.md`/`STATE.md`
   is remediated).

3. An `Edit` or `Write` tool call targets one of the five governed files, triggering the
   `legacy-bash-adapter`-hosted PostToolUse/PreToolUse WASM validators registered against that
   file's path pattern (per `plugins/vsdd-factory/hooks-registry.toml`).

## Postconditions

### PC1 — `last_amended` byte length is bounded regardless of cumulative burst count

After any number of bursts (N ≥ 0) have each overwritten `last_amended` and prepended one
`changelog:` item under BC-5.45.001's discipline, the `last_amended` field's own byte length never
exceeds a fixed per-entry ceiling — it is always exactly one dated entry, independent of N. This
is the structural guarantee that a mega-line (the pre-fix 323,499-char case) cannot re-form.

### PC2 — a representative synthetic-burst regression run completes within the fuel budget

A regression/integration test simulates a representative run of N synthetic state-manager bursts
(N chosen to be at least an order of magnitude larger than the burst count that produced the
pre-fix mega-line's practical trigger point) against a fixture file that starts in the
post-migration ADR-049 shape. Each synthetic burst overwrites `last_amended` with a fresh
current entry and prepends one `changelog:` item, per BC-5.45.001. After all N bursts, invoking
the `legacy-bash-adapter`-hosted validator(s) that scan this fixture completes within the
configured fuel budget for that scenario, producing zero `Trap::OutOfFuel` /
`plugin.indeterminate` (cause=fuel) events.

### PC3 — the 743-fuel-timeouts/day symptom does not recur under the new write-path

A differential regression test compares two fixture states: (a) a PRE-FIX fixture reproducing the
pre-migration mega-line shape (calibrated to the D-1149 323,499-char `STORY-INDEX.md` case or
equivalent), which MUST reproduce fuel exhaustion (establishing the test's own negative control —
the bug is real and the harness can detect it); and (b) a POST-FIX fixture at the same cumulative
history depth but in ADR-049 shape (current-entry-only `last_amended` + N-item `changelog:`
sequence), which MUST NOT exhaust the fuel budget. The differential outcome (fail on (a), pass on
(b)) is the regression proof.

## Invariants

1. **Fuel consumption for validating a governed file's frontmatter grows at most linearly with
   cumulative `changelog:` size, never superlinearly, and is independent of burst count for the
   `last_amended` field specifically.** Per ADR-049 §Audit finding 3, no validator in the current
   surface parses `changelog:` semantically — its growth does not by itself increase fuel cost for
   today's validators. Fuel cost for these files' validators is dominated by total file BYTE SIZE,
   which BC-10.13.001's rotation utility bounds for very long-running cycles.

2. **This BC changes no validator code and no fuel-cap constant.** Per ADR-049 §Audit findings
   1-6, zero production validator changes are required for the write-path fix itself; this BC's
   regression test is a verification artifact, not a source of new production code paths in
   `crates/hook-plugins/legacy-bash-adapter/` or `crates/factory-dispatcher/src/invoke.rs`.

3. **The regression test is reproducible against a committed fixture, not live production files.**
   The synthetic-burst simulation (PC2) and the differential comparison (PC3) run against
   version-controlled fixture files sized and shaped to reproduce the documented symptom, so the
   test's outcome does not depend on the live size of `BC-INDEX.md`/`STORY-INDEX.md`/etc. at any
   given moment.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | A very long-running cycle accumulates a `changelog:` sequence large enough to approach the fuel budget on total-file-byte-size grounds alone (not via `last_amended`) | BC-10.13.001's rotation subcommand is the required safety net (S-15.03 AC-006(ii)/AC-010) — this BC's PC2 regression test MUST include a scenario at or near the rotation threshold to confirm the pre-rotation state still passes, establishing the threshold is chosen conservatively. |
| EC-002 | The differential PRE-FIX fixture (EC negative control) fails to reproduce fuel exhaustion in a test environment with a raised fuel cap (e.g. a future `DEFAULT_FUEL_CAP` increase makes the old mega-line no longer exhaust the budget) | The regression test MUST pin or parametrize the fuel-cap value used in the differential comparison rather than relying on the ambient/default cap, so the negative control remains meaningful independent of future cap changes (ADR-042 §Decision 1 raised the default from 10M to 20M; operator-level cache lag per the project's own Hook Diagnostics table means the effective cap at any given time may differ from `develop`'s constant). |
| EC-003 | A synthetic burst in the PC2 simulation includes an entry with an embedded double-quote (D-1144 class) | The synthetic burst still completes within budget — escaping does not materially change payload size or fuel cost; this edge case exists to confirm the fuel-relief property is orthogonal to (not coupled with) the YAML-escape correctness property already covered by BC-5.45.001 Invariant 3 / BC-10.13.001 PC3. |
| EC-004 | Validator plugin registered against a governed file is itself changed in a future story (e.g. `validate-count-propagation.sh` gains new logic) | Out of scope for this BC's regression test to predict — this BC's fixed-point is the fuel behavior AS OF the ADR-049 write-path fix; a future validator change that reintroduces linear-or-worse scanning cost is a new regression to be caught by that future story's own tests, not retroactively by this BC. |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Post-migration `BC-INDEX.md`-shaped fixture; 50 synthetic bursts each overwriting `last_amended` + prepending 1 `changelog:` item | All 50 bursts' validator invocations complete within the configured fuel budget; zero `plugin.indeterminate`(cause=fuel) events | happy-path |
| Pre-migration mega-line fixture (calibrated to reproduce the D-1149 323,499-char case) vs. post-migration fixture at equivalent cumulative history depth | Pre-migration fixture reproduces fuel exhaustion (negative control passes); post-migration fixture does not (positive fix confirmed) | edge-case |
| Post-migration fixture with `changelog:` grown past the BC-10.13.001 rotation threshold, rotation NOT yet invoked | Validator invocation still completes within budget at the threshold boundary (confirms the threshold is chosen conservatively per EC-001), but a further-unbounded-growth scenario beyond the threshold is explicitly out of this BC's guarantee — rotation is required beyond that point | error (unbounded-growth-without-rotation is the failure this BC does NOT claim to prevent indefinitely) |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — route to architect) | N synthetic bursts under BC-5.45.001's write-path never grow `last_amended` byte length beyond a fixed ceiling (PC1) | proptest: assert `last_amended` byte length is O(1) in N across the synthetic-burst simulation |
| (TBD — route to architect) | Post-migration fixture completes N synthetic bursts' validator invocations within the configured fuel budget (PC2) | integration: bats/Rust-workspace test invoking the real `legacy-bash-adapter`-hosted validator(s) against the fixture, asserting `plugin.indeterminate`(cause=fuel) count == 0 |
| (TBD — route to architect) | Differential pre-fix/post-fix fixture comparison reproduces the symptom on pre-fix and not on post-fix (PC3) | integration: differential test with fixed/pinned fuel-cap value per EC-002 |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-042 |
| Capability Anchor Justification | CAP-042 ("`last_amended` Write-Path Durable Fix: current-entry-only scalar, `changelog:` prepend discipline, sanctioned migration/rotation tooling, and bash-adapter fuel-budget relief") per `.factory/specs/domain-spec/capabilities.md` §CAP-042. BC-4.18.001 is the fuel-relief-implementing BC for CAP-042 (BC-5.45.001 implements the write-path-invariant clause; BC-10.13.001 implements the tooling clause). |
| L2 Domain Invariants | none (operational infrastructure) |
| Architecture Module | `crates/hook-plugins/legacy-bash-adapter/` (validator host); `crates/factory-dispatcher/src/invoke.rs` (`DEFAULT_FUEL_CAP`, fuel-consumption accounting) |
| Stories | S-15.03 |
| Source Issues | 743-fuel-timeouts/day symptom (2026-09-02, cited in ADR-049 §Context and S-15.03 AC-009) |
| ADR Reference | ADR-049 §Context; ADR-042 §Decision 1 (`DEFAULT_FUEL_CAP`) |

## Related BCs

- BC-5.45.001 — the write-path invariant whose structural guarantee (PC1 of this BC) this BC
  verifies under regression load (depends on)
- BC-10.13.001 — the migration/rotation tool whose successful run is a precondition for this BC's
  post-fix fixture state, and whose rotation subcommand is the safety net referenced in EC-001
  (depends on)
- BC-1.03.019 — fuel-headroom WARN event (>90% consumption early-warning); a related but distinct
  mechanism — that BC governs an early-warning signal on ANY plugin invocation approaching its
  cap, while this BC governs the specific root-cause elimination for the five ADR-049-governed
  files (related to)
- BC-1.18.001 / BC-1.18.002 / BC-1.18.003 / BC-1.18.004 — the INDETERMINATE-outcome/durable-marker/
  next-advance-gate family that fires WHEN a fail-closed validator exhausts fuel; this BC prevents
  the triggering condition (fuel exhaustion on these five files) from arising in the first place
  for the specific payload-growth cause ADR-049 fixes (related to)

## Architecture Anchors

- `crates/hook-plugins/legacy-bash-adapter/` — hosts the bash-script-wrapping WASM validators
  (`validate-count-propagation.sh`, `validate-changelog-monotonicity.sh`, and others per
  `hooks-registry.toml`) that scan these five files and whose fuel budget this BC protects.
- `crates/factory-dispatcher/src/invoke.rs` — `DEFAULT_FUEL_CAP` constant and fuel-consumption
  accounting (`fuel_consumed_from_store`), per ADR-042 §Decision 1.
- `.factory/specs/architecture/decisions/ADR-049-last-amended-write-path-durable-fix-current-entry-plus-changelog-sequence.md` — §Context (743-fuel-timeouts/day symptom), §Audit finding 4.

## Story Anchor

S-15.03 (E-12 Engine Governance — `last_amended` Write-Path Durable Fix, Scope Extension AC-009)

## VP Anchors

TBD — VP needs flagged above (3 candidate VP rows); route to architect for VP-NNN assignment and registration in VP-INDEX.md per `vp_index_is_vp_catalog_source_of_truth`.

## Changelog

| Version | Date | Description |
|---------|------|-------------|
| 1.0 | 2026-09-02 | Initial authoring (product-owner; ADR-049 Phase B; S-15.03 AC-009). PC1 bounded `last_amended` byte length; PC2 synthetic-burst regression within fuel budget; PC3 differential pre-fix/post-fix symptom-non-recurrence proof. 3 invariants (linear-not-superlinear fuel growth, zero validator/fuel-cap code change, fixture reproducibility). 4 edge cases EC-001..EC-004. 3 test vectors. 3 VP candidates flagged for architect. lifecycle_status: draft. |
