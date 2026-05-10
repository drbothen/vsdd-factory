# Adversarial Review — S-12.04 Pass 5

**Story:** S-12.04 — WASM Resolver Host ABI & Engine Dispatch
**Pass:** 5
**Date:** 2026-05-10
**Branch SHA at review:** b9592c0d
**Classification:** MEDIUM
**Findings:** 3 (1 MEDIUM + 2 NITPICK)

---

## Summary

Pass 5 reviewed the post-pass-4-burst state of S-12.04 at SHA b9592c0d. The pass-4 burst
addressed all 5 NITPICK findings. One new MEDIUM finding was surfaced: BC-4.12.004 PC2 still
lists `hook_event_name` in the `resolver.error` telemetry event field list, but the
canonical wire format (HOST_ABI.md lines 1107-1112, executor.rs:530-543) uses `event_type`
for the Claude Code envelope event type. Additionally, the top-level envelope discriminator
field in PC2 is listed as `event_type: "resolver.error"` but the InternalEvent envelope
uses `type` not `event_type` for envelope identity. Two NITPICK findings on missing
`session_id` and `plugin_name` fields that F-P4-001 wired but BC-4.12.004 PC2 did not track.

---

## Findings

### F-P5-001 [MEDIUM] BC-4.12.004 PC2 `hook_event_name` field is stale; envelope discriminator uses wrong key

**Location:** `.factory/specs/behavioral-contracts/ss-04/BC-4.12.004.md` lines 67-72 (PC2
telemetry event field list)

**Observation:** PC2 lists the following fields for the `resolver.error` telemetry event:

```
- event_type: "resolver.error"
- resolver_name
- error_kind
- error_detail
- hook_event_name
```

Two problems:

1. **`hook_event_name` to `event_type`:** The pass-2 burst renamed this field in HOST_ABI.md
   and executor.rs but the BC body was not updated. The wire field for the Claude Code
   envelope event type (e.g., "PreToolUse") is `event_type` in the InternalEvent struct and
   in executor.rs:530-543. `hook_event_name` does not appear in the current implementation.

2. **Envelope discriminator `event_type` to `type`:** InternalEvent uses `type` as the
   envelope identity discriminator (the field that carries `"resolver.error"`), not
   `event_type`. PC2 labels this field `event_type: "resolver.error"` which conflates the
   envelope discriminator with the hook-dispatch-context field. The wire field is
   `type: "resolver.error"`.

**Impact:** A developer implementing or testing the `resolver.error` event against BC-4.12.004
PC2 will produce the wrong wire shape. Integration tests asserting `hook_event_name` will
pass against a stale mock and fail against the real executor.

**Fix:** Rename PC2 field list to:

```
- type: "resolver.error"   (envelope discriminator -- InternalEvent)
- resolver_name
- error_kind
- error_detail
- event_type               (Claude Code envelope event type, e.g. "PreToolUse")
- trace_id
- plugin_name
- session_id               (added by F-P4-001)
```

---

### F-P5-002 [NITPICK] BC-4.12.004 PC2 missing `session_id` field (F-P4-001 gap)

**Location:** BC-4.12.004 PC2 field list

**Observation:** The pass-4 burst (F-P4-001) wired `session_id` into the `resolver.error`
event in executor.rs. BC-4.12.004 PC2 was not updated to reflect this addition. The field
is already covered under F-P5-001 fix but noted separately for traceability.

**Fix:** Include `session_id` in PC2 field list (covered by F-P5-001 fix).

---

### F-P5-003 [NITPICK] BC-4.12.004 PC2 missing `plugin_name` and `trace_id` fields

**Location:** BC-4.12.004 PC2 field list

**Observation:** HOST_ABI.md lines 1107-1112 list `plugin_name` and `trace_id` as fields
on `resolver.error`. These were present in the HOST_ABI but not reflected in BC-4.12.004
PC2. Low impact since HOST_ABI is the wire canonical source, but the BC should be a
faithful summary.

**Fix:** Include `plugin_name` and `trace_id` in PC2 field list (covered by F-P5-001 fix).

---

## Recommendation

**PROCEED_TO_FIX** -- F-P5-001 is a MEDIUM wire-field propagation gap. Amend BC-4.12.004
PC2. No implementation changes required; spec-only fix. Pass 6 can follow immediately after
the BC amendment is committed to factory-artifacts.
