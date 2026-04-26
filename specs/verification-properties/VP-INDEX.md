---
document_type: vp-index
level: L4
version: "1.0"
status: draft
producer: architect
timestamp: 2026-04-25T00:00:00
phase: 1.6b
traces_to: ARCH-INDEX.md
total_vps: 60
---

# VP-INDEX: Verification Properties Master Index

> **Source of truth** for all verification properties.
> VP-INDEX is currently the authoritative source for the Provable Properties Catalog
> and VP-to-Module coverage mapping.
> `architecture/verification-architecture.md` and
> `architecture/verification-coverage-matrix.md` are deferred — when created,
> they will be derived from this index. Until then, use:
> - §Full Index (scope column) as the VP-to-Module coverage table
> - §Kani Upgrade Candidates / §Property-Test Upgrade Candidates as the P0/P1 priority lists

## Summary

| Category | VPs | IDs |
|----------|-----|-----|
| Domain Invariant VPs | 17 | VP-001..VP-017 |
| Dispatcher Core VPs | 10 | VP-018..VP-027 |
| Sink VPs | 10 | VP-028..VP-037 |
| SDK / Plugin ABI VPs | 5 | VP-038..VP-042 |
| Hook Layer VPs | 10 | VP-043..VP-052 |
| Workflow VPs | 5 | VP-053..VP-057 |
| Skill Catalog VPs | 3 | VP-058..VP-060 |
| **Total** | **60** | **VP-001..VP-060** |

## Proof Method Breakdown

| Method | Count | VPs |
|--------|-------|-----|
| unit-test | 40 | VP-003..014, VP-016..024, VP-026..027, VP-029..032, VP-034..042, VP-044..045, VP-050, VP-052 |
| integration | 10 | VP-001, VP-002, VP-025, VP-028, VP-033, VP-043, VP-049, VP-051, VP-058, VP-060 |
| manual | 9 | VP-015, VP-046..048, VP-053..057 |
| kani-proof | 0 | — (upgrade candidates: VP-020, VP-023, VP-042) |
| proptest | 1 | VP-059 (upgrade candidates: VP-019, VP-029, VP-032) |

## Full Index

| VP ID | Title | Type | Proof Method | Scope | Domain Invariant | Status |
|-------|-------|------|-------------|-------|-----------------|--------|
| [VP-001](VP-001.md) | Tier Execution Is Sequential; Intra-Tier Is Parallel | invariant | integration | SS-01 | DI-001 | draft |
| [VP-002](VP-002.md) | Plugin Crash or Timeout Does Not Block Sibling Plugins | invariant | integration | SS-01 | DI-002 | draft |
| [VP-003](VP-003.md) | block_intent Is Aggregate; Tier Runs to Completion | invariant | unit-test | SS-01 | DI-003 | draft |
| [VP-004](VP-004.md) | Capability Denial Produces Return Code AND Audit Event | safety | unit-test | SS-01 | DI-004 | draft |
| [VP-005](VP-005.md) | Shell Interpreters Require Explicit shell_bypass_acknowledged | safety | unit-test | SS-01 | DI-005 | draft |
| [VP-006](VP-006.md) | Setuid/Setgid Binaries Refused Unconditionally | safety | unit-test | SS-01 | DI-006 | draft |
| [VP-007](VP-007.md) | Dispatcher Self-Telemetry Is Always-On and Never Panics | safety | unit-test | SS-01, SS-03 | DI-007 | draft |
| [VP-008](VP-008.md) | Internal Log Filename Derived from Event Timestamp, Not Wall Clock | invariant | unit-test | SS-01, SS-03 | DI-008 | draft |
| [VP-009](VP-009.md) | prune_old Removes Only Dispatcher-Internal Files Older Than Threshold | invariant | unit-test | SS-01, SS-03 | DI-009 | draft |
| [VP-010](VP-010.md) | Plugin Stderr Capped at 4 KiB with Truncation Marker | invariant | unit-test | SS-01 | DI-010 | draft |
| [VP-011](VP-011.md) | Sink submit Must Not Block the Dispatcher | safety | unit-test | SS-03 | DI-011 | draft |
| [VP-012](VP-012.md) | Sink Failure Affects Only That Sink | invariant | unit-test | SS-03 | DI-012 | draft |
| [VP-013](VP-013.md) | Unknown Sink Driver Types Are Non-Fatal | invariant | unit-test | SS-03 | DI-013 | draft |
| [VP-014](VP-014.md) | Schema Version Mismatch Is a Hard Load Error | invariant | unit-test | SS-01 | DI-014 | draft |
| [VP-015](VP-015.md) | Per-Project Activation Required Before Dispatcher Can Run | precondition | manual | SS-09 | DI-015 | draft |
| [VP-016](VP-016.md) | Each Registry Entry Sees Only Its Own plugin_config | invariant | unit-test | SS-01 | DI-016 | draft |
| [VP-017](VP-017.md) | dispatcher_trace_id Present on Every Emitted Event | invariant | unit-test | SS-01 | DI-017 | draft |
| [VP-018](VP-018.md) | Registry Rejects Malformed Configurations at Load Time | precondition | unit-test | SS-01 | DI-014 | draft |
| [VP-019](VP-019.md) | Routing Is Deterministic — Same Input Yields Same Plugin Selection | postcondition | unit-test | SS-01 | DI-001 | draft |
| [VP-020](VP-020.md) | Epoch Timeout Rounds Up and Terminates Infinite Loops | postcondition | unit-test | SS-01 | DI-001, DI-002 | draft |
| [VP-021](VP-021.md) | Capability Deny-by-Default — Each Capability Requires Explicit Allow | safety | unit-test | SS-01 | DI-004, DI-005 | draft |
| [VP-022](VP-022.md) | Dispatcher Exit Code Semantics — 0 for Non-Block, 2 for Block | postcondition | unit-test | SS-01 | DI-014 | draft |
| [VP-023](VP-023.md) | Wire Format Decoders Reject Truncated Input Without Panic | safety | unit-test | SS-01, SS-02 | DI-004 | draft |
| [VP-024](VP-024.md) | Plugin Cache Is Keyed by Path and Invalidated by mtime | invariant | unit-test | SS-01 | — | draft |
| [VP-025](VP-025.md) | Host Function ABI Surface Is Complete and Stable | invariant | integration | SS-01, SS-02 | DI-004 | draft |
| [VP-026](VP-026.md) | InternalEvent Serializes Flat with No Null Optional Fields | invariant | unit-test | SS-01, SS-03 | DI-017 | draft |
| [VP-027](VP-027.md) | HookPayload Parsing Is Robust for All Claude Code Envelope Types | precondition | unit-test | SS-01 | DI-017 | draft |
| [VP-028](VP-028.md) | Sink Fan-Out — Every Event Reaches Every Configured Accepting Sink | postcondition | integration | SS-03 | DI-011, DI-012 | draft |
| [VP-029](VP-029.md) | File Sink Path Template Substitutes {date}, {name}, {project} Correctly | postcondition | unit-test | SS-03 | DI-008 | draft |
| [VP-030](VP-030.md) | Sink Shutdown Drains Queued Events Before Closing | postcondition | unit-test | SS-03 | DI-011 | draft |
| [VP-031](VP-031.md) | Tag Enrichment Does Not Overwrite Producer Fields | invariant | unit-test | SS-03 | DI-012 | draft |
| [VP-032](VP-032.md) | RoutingFilter Default Accepts All Events; Allow-List Is Whitelist; Deny Applied After Allow | invariant | unit-test | SS-03 | DI-011 | draft |
| [VP-033](VP-033.md) | OTLP LogRecord Mapping Is Correct — type to body, ts_epoch to time_unix_nano | postcondition | integration | SS-03 | DI-017 | draft |
| [VP-034](VP-034.md) | OTLP Sink Batch Trigger Thresholds Are Independent | invariant | unit-test | SS-03 | DI-011 | draft |
| [VP-035](VP-035.md) | File Sink Auto-Creates Missing Parent Directories | postcondition | unit-test | SS-03 | DI-007 | draft |
| [VP-036](VP-036.md) | Disabled Sink Drops Every Event Without Writing | invariant | unit-test | SS-03 | DI-013 | draft |
| [VP-037](VP-037.md) | OTLP Resource Attributes — Operator Overrides Win Over Auto-Detected Defaults | invariant | unit-test | SS-03 | DI-012 | draft |
| [VP-038](VP-038.md) | SDK HookResult Exit Codes Are Stable — Continue=0, Error=1, Block=2 | invariant | unit-test | SS-02 | DI-004 | draft |
| [VP-039](VP-039.md) | SDK Wire Format Encoding Is Symmetric with Dispatcher Decoding | invariant | unit-test | SS-02 | DI-004 | draft |
| [VP-040](VP-040.md) | SDK HookPayload Round-Trips via Serde and Carries plugin_config | invariant | unit-test | SS-02 | DI-016 | draft |
| [VP-041](VP-041.md) | SDK Panic Handler Extracts Message for All Payload Types | safety | unit-test | SS-02 | DI-002 | draft |
| [VP-042](VP-042.md) | SDK HostError Code Mapping Is Stable | invariant | unit-test | SS-02 | DI-004 | draft |
| [VP-043](VP-043.md) | Every hooks-registry.toml Entry Routes Through legacy-bash-adapter.wasm | invariant | integration | SS-07, SS-01 | DI-016 | draft |
| [VP-044](VP-044.md) | Legacy Bash Adapter Exit Code Mapping Is Correct | postcondition | unit-test | SS-04, SS-07 | DI-003 | draft |
| [VP-045](VP-045.md) | Legacy Bash Adapter Strips plugin_config Before Piping to Bash | invariant | unit-test | SS-04 | DI-016 | draft |
| [VP-046](VP-046.md) | All hooks-registry.toml Entries Correspond to Registered Hook Scripts | invariant | manual | SS-07 | DI-014 | draft |
| [VP-047](VP-047.md) | Validator Hooks Exit 0 (Pass) or 2 (Block) — No Other Codes | safety | manual | SS-07 | DI-003 | draft |
| [VP-048](VP-048.md) | protect-secrets.sh Fails Closed When jq Is Missing | safety | manual | SS-07 | — | draft |
| [VP-049](VP-049.md) | Generated hooks-registry.toml Round-Trips Through Registry::load | invariant | integration | SS-07, SS-09 | DI-014 | draft |
| [VP-050](VP-050.md) | exec_subprocess Timeout Is Enforced — Hung Commands Are Killed | postcondition | unit-test | SS-01 | DI-002 | draft |
| [VP-051](VP-051.md) | Dispatcher Startup Flow Writes Parseable JSONL with Correct Envelopes | postcondition | integration | SS-01, SS-03 | DI-007, DI-017 | draft |
| [VP-052](VP-052.md) | Epoch Ticker Shuts Down Cooperatively and Idempotently | liveness | unit-test | SS-01 | DI-001 | draft |
| [VP-053](VP-053.md) | Lobster Workflow DAG Is Acyclic — No Circular Dependencies | invariant | manual | SS-05 | — | draft |
| [VP-054](VP-054.md) | Workflow Loop Blocks Are Bounded — max_iterations and exit_condition Required | safety | manual | SS-05 | — | draft |
| [VP-055](VP-055.md) | state-manager Runs Last in Every Burst | invariant | manual | SS-05 | — | draft |
| [VP-056](VP-056.md) | on_failure Semantics — retry → escalate → abort Are Correctly Ordered | invariant | manual | SS-05 | — | draft |
| [VP-057](VP-057.md) | Adversarial Review Convergence — Mis-Anchoring Always Blocks, 3-Clean-Pass Minimum | safety | manual | SS-05 | — | draft |
| [VP-058](VP-058.md) | create-adr Atomicity — No Partial Repository State After Failure | invariant | integration | SS-06 | — | draft |
| [VP-059](VP-059.md) | ID Monotonicity — Allocated ADR-NNN is Strictly Greater Than All Existing IDs | invariant | proptest | SS-06 | — | draft |
| [VP-060](VP-060.md) | Bidirectional Supersession — supersedes ↔ superseded_by is Symmetric After Skill Completion | invariant | integration | SS-06 | — | draft |

## Kani Upgrade Candidates (P0 Priority)

| VP | Property | Justification |
|----|----------|--------------|
| VP-020 | Epoch timeout rounds up (div_ceil) | Pure integer arithmetic, bounded input |
| VP-023 | Wire format decoders reject truncated buffers | Security boundary, pure function |
| VP-042 | HostError code mapping for all negative i32 | ABI contract, exhaustive verification |

## Property-Test Upgrade Candidates

| VP | Property | Strategy |
|----|----------|---------|
| VP-019 | Routing determinism | proptest over arbitrary HookPayload |
| VP-029 | Template substitution | proptest over arbitrary template strings |
| VP-032 | RoutingFilter semantics | proptest over (event_type, allow, deny) triples |
| VP-059 | ADR ID monotonicity | proptest over arbitrary filesystem ID sets (200 trials) |

## Traceability

- All 17 domain invariants (DI-001..DI-017) covered by VP-001..VP-017
- BCs cross-referenced: 95 BC IDs across 60 VPs
- Test evidence cited: 47 VPs have specific Rust test references
- 13 VPs have TBD test evidence (manual or pending CI automation)
