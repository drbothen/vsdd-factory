---
document_type: vp-snapshot
level: ops
version: "1.0"
status: sealed
producer: architect
timestamp: 2026-04-29T17:30:00Z
cycle: v1.0-brownfield-backfill
tag_candidate: v1.0.0-rc.1
develop_sha_at_snapshot: d134648
gate_reference: AC-Q2 of S-4.08 — VP T0 snapshot for rc.1 release-gate
---

# rc.1 VP T0 Snapshot

> **Sealed at:** 2026-04-29 (T0 for v1.0.0-rc.1 cut)
> **Develop SHA:** d134648
> **Cycle:** v1.0-brownfield-backfill
> **Out-of-scope clause:** VPs added to VP-INDEX after this T0 timestamp (develop SHA d134648, VP total = 68) roll forward to v1.0.0-rc.2 or v1.0.0 GA — they are NOT retrofitted into this cut.

## Verification methodology

VPs with `proof_method: unit-test` are GREEN if the test runner is currently green on develop @ d134648. Test-suite snapshot recorded at T0:

| Runner | Command | Result |
|--------|---------|--------|
| `cargo test --workspace` | rustc 1.95.0 — 25 test binaries, 254 tests | PASS (0 failed) |
| `bats plugins/vsdd-factory/tests/` | 1316 tests | PASS (0 failed) |
| `scripts/check-platforms-drift.py` | 5 platforms | PASS (in sync) |

All unit-test VPs are GREEN by transitivity: the test functions named in their `test_evidence` fields belong to the crates verified above.

Kani Upgrade Candidates (VP-020, VP-023, VP-042) carry `proof_method: unit-test` in VP-INDEX — their current verification is via unit tests, not Kani. Kani harnesses are not yet committed; current status is GREEN (unit-test) / NOT-VERIFIED (kani). The Kani upgrade is scheduled for a future formal-verify wave.

## Snapshot Set A — Kani Upgrade Candidates (3 VPs)

| VP | Title | Current proof_method | Status at T0 | Anchor story | Test evidence |
|----|-------|---------------------|-------------|-------------|---------------|
| VP-020 | Epoch Timeout Rounds Up and Terminates Infinite Loops | unit-test (kani candidate) | GREEN (unit-test) | TBD | `engine.rs::tests::timeout_ms_to_epochs_rounds_up` |
| VP-023 | Wire Format Decoders Reject Truncated Input Without Panic | unit-test (kani candidate) | GREEN (unit-test) | S-1.03 | `emit_event.rs::tests::decode_rejects_truncated_key_length` |
| VP-042 | SDK HostError Code Mapping Is Stable | unit-test (kani candidate) | GREEN (unit-test) | S-1.03 | `host.rs::tests` (6 unit tests) |

Note: Kani harnesses are listed in VP files as `proof_harness_skeleton` only — no `#[kani::proof]` harnesses are committed yet. The formal-verifier wave that upgrades these to `proof_method: kani` has not run. This snapshot captures the unit-test baseline that must stay GREEN at the rc.1 tag.

## Snapshot Set B — proof_method: unit-test VPs (37 VPs)

Per VP-INDEX proof method breakdown: `unit-test` count = 40. Set A overlaps with Set B (VP-020, VP-023, VP-042 all have `proof_method: unit-test`). The 37 non-Kani-candidate unit-test VPs are listed below.

| VP | Title | Status at T0 | Anchor story | Module |
|----|-------|-------------|-------------|--------|
| VP-003 | block_intent Is Aggregate; Tier Runs to Completion | GREEN | — | SS-01 dispatcher engine |
| VP-004 | Capability Denial Produces Return Code AND Audit Event | GREEN | — | SS-01 dispatcher |
| VP-005 | Shell Interpreters Require Explicit shell_bypass_acknowledged | GREEN | — | SS-01 dispatcher |
| VP-006 | Setuid/Setgid Binaries Refused Unconditionally | GREEN | — | SS-01 dispatcher |
| VP-007 | Dispatcher Self-Telemetry Is Always-On and Never Panics | GREEN | S-4.10 | SS-01/SS-03 |
| VP-008 | Internal Log Filename Derived from Event Timestamp, Not Wall Clock | GREEN | — | SS-01/SS-03 |
| VP-009 | prune_old Removes Only Dispatcher-Internal Files Older Than Threshold | GREEN | — | SS-01/SS-03 |
| VP-010 | Plugin Stderr Capped at 4 KiB with Truncation Marker | GREEN | — | SS-01 dispatcher |
| VP-011 | Sink submit Must Not Block the Dispatcher | GREEN | S-4.09 | SS-03 sinks |
| VP-012 | Sink Failure Affects Only That Sink | GREEN | S-4.09, S-4.10 | SS-03 sinks |
| VP-013 | Unknown Sink Driver Types Are Non-Fatal | GREEN | — | SS-03 sinks |
| VP-014 | Schema Version Mismatch Is a Hard Load Error | GREEN | — | SS-01 dispatcher |
| VP-016 | Each Registry Entry Sees Only Its Own plugin_config | GREEN | — | SS-01 dispatcher |
| VP-017 | dispatcher_trace_id Present on Every Emitted Event | GREEN | — | SS-01 dispatcher |
| VP-018 | Registry Rejects Malformed Configurations at Load Time | GREEN | — | SS-01 dispatcher |
| VP-019 | Routing Is Deterministic — Same Input Yields Same Plugin Selection | GREEN | — | SS-01 dispatcher |
| VP-021 | Capability Deny-by-Default — Each Capability Requires Explicit Allow | GREEN | — | SS-01 dispatcher |
| VP-022 | Dispatcher Exit Code Semantics — 0 for Non-Block, 2 for Block | GREEN | — | SS-01 dispatcher |
| VP-024 | Plugin Cache Is Keyed by Path and Invalidated by mtime | GREEN | — | SS-01 dispatcher |
| VP-026 | InternalEvent Serializes Flat with No Null Optional Fields | GREEN | — | SS-01/SS-03 |
| VP-027 | HookPayload Parsing Is Robust for All Claude Code Envelope Types | GREEN | — | SS-01 dispatcher |
| VP-029 | File Sink Path Template Substitutes {date}, {name}, {project} Correctly | GREEN | — | SS-03 sink-file |
| VP-030 | Sink Shutdown Drains Queued Events Before Closing | GREEN | — | SS-03 sinks |
| VP-031 | Tag Enrichment Does Not Overwrite Producer Fields | GREEN | — | SS-03 sinks |
| VP-032 | RoutingFilter Default Accepts All Events; Allow-List Is Whitelist; Deny Applied After Allow | GREEN | — | SS-03 sinks |
| VP-034 | OTLP Sink Batch Trigger Thresholds Are Independent | GREEN | — | SS-03 sink-otel-grpc |
| VP-035 | File Sink Auto-Creates Missing Parent Directories | GREEN | — | SS-03 sink-file |
| VP-036 | Disabled Sink Drops Every Event Without Writing | GREEN | — | SS-03 sinks |
| VP-037 | OTLP Resource Attributes — Operator Overrides Win Over Auto-Detected Defaults | GREEN | — | SS-03 sink-otel-grpc |
| VP-038 | SDK HookResult Exit Codes Are Stable — Continue=0, Error=1, Block=2 | GREEN | S-3.03, S-1.03 | SS-02 hook-sdk |
| VP-039 | SDK Wire Format Encoding Is Symmetric with Dispatcher Decoding | GREEN | S-1.03 | SS-02 hook-sdk |
| VP-040 | SDK HookPayload Round-Trips via Serde and Carries plugin_config | GREEN | S-1.03 | SS-02 hook-sdk |
| VP-041 | SDK Panic Handler Extracts Message for All Payload Types | GREEN | S-1.03 | SS-02 hook-sdk |
| VP-044 | Legacy Bash Adapter Exit Code Mapping Is Correct | GREEN | — | SS-04/SS-07 |
| VP-045 | Legacy Bash Adapter Strips plugin_config Before Piping to Bash | GREEN | — | SS-04 |
| VP-050 | exec_subprocess Timeout Is Enforced — Hung Commands Are Killed | GREEN | — | SS-01 dispatcher |
| VP-052 | Epoch Ticker Shuts Down Cooperatively and Idempotently | GREEN | — | SS-01 dispatcher |

## Total snapshot

- **Set A (Kani Upgrade Candidates):** 3 (VP-020, VP-023, VP-042)
- **Set B (unit-test, non-Kani-candidate):** 37 (VP-003..VP-014, VP-016..VP-019, VP-021..VP-022, VP-024, VP-026..VP-027, VP-029..VP-032, VP-034..VP-042 excluding Set A, VP-044..VP-045, VP-050, VP-052)
- **Total (Set A union Set B):** 40 VPs
- **Status:** all 40 GREEN (unit-test baseline)
- **Kani harness status:** NOT-VERIFIED for VP-020/VP-023/VP-042 — Kani harnesses are sketched in VP files but not committed; formal-verifier wave pending

## Roll-forward notes

VP-INDEX total at T0 = 68 VPs. VPs with `proof_method: integration` (16), `manual` (10), `static-check` (1), and `proptest` (1) are NOT in this snapshot — AC-Q2 scopes only Kani Upgrade Candidates and `proof_method: unit-test` VPs.

Any VPs added to VP-INDEX after develop SHA d134648 are NOT incorporated into this rc.1 cut. The next snapshot (rc.2 or v1.0.0 GA) will include them.

## Sealed

Sealed by architect agent at 2026-04-29T17:30:00Z for AC-Q2 of S-4.08 rc.1 release-gate.
