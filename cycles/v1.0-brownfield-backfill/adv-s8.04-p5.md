---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-05-01T00:00:00Z
phase: 2
inputs:
  - .factory/stories/S-8.04-native-port-update-wave-state-on-merge.md
  - .factory/stories/S-8.10-sdk-extension-write-file.md
  - .factory/specs/architecture/ARCH-INDEX.md
  - .factory/specs/behavioral-contracts/ss-07/BC-7.03.083.md
  - plugins/vsdd-factory/hooks-registry.toml
  - crates/hook-sdk/src/host.rs
  - .factory/cycles/v1.0-brownfield-backfill/adv-s8.04-p4.md
input-hash: "e441e99"
traces_to: prd.md
story_id: "S-8.04"
pass_number: 5
story_version: "1.3"
story_input_hash: "e441e99"
pass: p5
previous_review: adv-s8.04-p4.md
target: story
target_file: .factory/stories/S-8.04-native-port-update-wave-state-on-merge.md
verdict: NITPICK_ONLY
clock: 1_of_3
findings_critical: 0
findings_high: 0
findings_medium: 0
findings_low: 0
findings_nit: 1
---

# Adversarial Review Pass-5 — S-8.04 v1.3

## Finding ID Convention

Finding IDs use the format `F-S804-P5-NNN` (P5 = Pass 5; NNN = three-digit sequence).

## Part A — Pass-4 Fix Verification

Pass-4 produced 2 HIGH + 1 MED + 3 LOW (6 findings). Pass-5 verifies each.

| ID | Pass-4 Severity | Pass-5 Status | Evidence |
|----|-----------------|---------------|----------|
| F-S804-P4-001 SS-04 fabricated "Wave State" | HIGH | **CLOSED** | Story lines 52-61 (frontmatter SS comments), 119-125 (Stretch-Anchor Disclosure), and 129 all use "Plugin Ecosystem"; line 121 explicitly states `"Wave State" is NOT a subsystem name`; line 129 records the v1.3 correction. ARCH-INDEX:77 = "Plugin Ecosystem" verified. POLICY 6 satisfied. |
| F-S804-P4-002 max_bytes 4-param signature propagation | HIGH | **CLOSED** | All four `write_file` call sites use 4-param form: line 96 (BLOCKER DISCLOSURE step 2), line 239 (AC-004), line 429 (T-5(h) with `&bytes, 65536, 10000`), line 505 (Library table). Verbatim match to S-8.10 v1.1 AC-1 line 144 = `pub fn write_file(path: &str, contents: &[u8], max_bytes: u32, timeout_ms: u32) -> Result<(), HostError>`. POLICY 8 satisfied. |
| F-S804-P4-003 BC-7.03.083 stale line range | MED | **CLOSED-AS-DEFERRED** | T-9.5 process-gap task (lines 466-472) explicitly tracks BC file update separately; BC trace table line 146 carries [process-gap] note. BC source file `ss-07/BC-7.03.083.md` line 36 still cites `877-894` (live registry confirmed at 942-948 per registry read). Story explicitly delegates BC source-file fix to maintenance ticket — appropriate scope separation. |
| F-S804-P4-004 Library table version assertion premature | LOW | **CLOSED** | Line 505 reworded to "0.2.0 (asserted post-S-8.10 merge — gate at T-0: confirm `version = "0.2.0"` in `crates/hook-sdk/Cargo.toml`; if still 0.1.x, S-8.10 is incomplete — STOP)". STOP gate explicit. |
| F-S804-P4-005 fixture `gate: ~` vs `gate_status: ~` | LOW | **CLOSED** | Line 283 of AC-006 fixture block now reads `gate_status: ~`. Verified verbatim. |
| F-S804-P4-006 T-1.5 "before deprecation" wording | LOW | **CLOSED** | T-1.5 (lines 394-400) now reads: "Pin to `0.9.34` (last release; dtolnay deprecated the crate at this version per 2024 announcement)". No "before deprecation" phrase remains. |

**Carryover note (D-6 external blocker):** S-8.10 v1.1 status remains `draft`; S-8.04 still requires S-8.10 status=done before T-1 begins. T-0 STOP CHECK enforces this. No regression.

**Carryover wiring (D-6 dependency):** depends_on line 24 = `["S-8.00", "S-8.10"]`; BLOCKER DISCLOSURE (lines 83-101) cites S-8.10 v1.1 AC-1 form correctly. POLICY 8 propagation verified.

**Pass-4 fix burst: 6/6 closed (5 fully resolved, 1 deferred-with-tracking).**

## Part B — New Findings (Pass-5)

### CRITICAL

(none)

### HIGH

(none)

### MEDIUM

(none)

### LOW

(none)

### NIT

#### F-S804-P5-001: T-5(h) write `max_bytes` semantic comment is slightly imprecise — SKIP-FIX eligible

- **Severity:** NIT
- **Confidence:** LOW
- **Category:** documentation
- **Location:** S-8.04 line 429-432 (T-5(h) write_file call)
- **Description:** T-5(h) uses `max_bytes=65536` for the write call with the comment "mirroring read_file cap". For a `write_file` capability the `max_bytes` parameter caps the **outbound payload size** (server-side enforcement per S-8.10 AC-2: "if `contents_len > max_bytes`, return `codes::OUTPUT_TOO_LARGE`"). The "mirroring read_file cap" framing suggests symmetry but the semantics differ slightly: read_file `max_bytes` bounds the inbound buffer the dispatcher writes; write_file `max_bytes` bounds the inbound payload from the guest. For a wave-state.yaml this is fine in practice (the file is far smaller than 65 KiB), but the framing could mislead a careful reader. Pure documentation polish.
- **Proposed Fix:** Optional — change comment to "max_bytes=65536 caps wave-state.yaml payload size; ample headroom over typical ~1-4 KiB wave-state files".
- **Disposition:** SKIP-FIX per S-7.03 (NIT, single-site, semantic clarity only; current text is not incorrect).

## Open Questions

- None new in pass-5. OQ-001 (regex precedence) and OQ-002 (serde_yaml deprecation) remain documented in story body lines 343-345 with explicit decisions.

## Pass-6 Priors

If pass-6 occurs (clock would be at 2_of_3 entering it):
- **Anchor invariants confirmed in pass-5** (do not re-verify unless story version changes):
  - SS-04 canonical name = "Plugin Ecosystem" (ARCH-INDEX:77)
  - SS-07 canonical name = "Hook Bash Layer" (ARCH-INDEX:80)
  - SS-01 canonical name = "Hook Dispatcher Core" (ARCH-INDEX:74)
  - host::write_file 4-param signature propagated to all 4 sites in S-8.04
  - BC-7.03.083-086 anchored; BC-7.03.083 source file line 36 stale (BC maintenance ticket, NOT story scope)
  - depends_on `["S-8.00", "S-8.10"]` correct
  - Registry binding live at hooks-registry.toml:942-948
  - HOST_ABI_VERSION = 1 invariant (lib.rs:58); no bump
- **External blocker:** S-8.10 status. If S-8.10 transitions to `ready`/`done` between pass-5 and pass-6, verify Library table version assertion does not become stale (currently "asserted post-S-8.10 merge" with STOP gate).
- **Pass-5 NIT (F-S804-P5-001):** can be re-checked but is SKIP-FIX eligible.

## Verdict

**NITPICK_ONLY.** Clock advances **0_of_3 → 1_of_3**.

All 6 pass-4 findings closed. Single new finding is NIT-severity documentation polish (SKIP-FIX eligible per S-7.03). No HIGH/MED/CRITICAL drift introduced by the v1.3 fix burst. SS-04 canonical-name fix and max_bytes 4-param propagation both verified bidirectionally against ARCH-INDEX and S-8.10 v1.1.

## Trajectory

| Pass | C | H | M | L | NIT | Total |
|------|---|---|---|---|-----|-------|
| p1 | 0 | 7 | 6 | 3 | 1 | 17 |
| p2 | 0 | 3 | 5 | 2 | 1 | 11 |
| p3 | 0 | 0 | 1 | 3 | 0 | 4 |
| p4 | 0 | 2 | 1 | 3 | 0 | 6 |
| p5 | 0 | 0 | 0 | 0 | 1 | 1 |

**Trajectory:** 17 \| 11 \| 4 \| 6 \| 1. Sharp recovery from p4 regression. Severity profile fully collapsed (2H \| 1M \| 3L → 0H \| 0M \| 0L \| 1NIT). v1.3 fix burst was effective.

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 5 |
| **New findings** | 1 (NIT) |
| **Closures** | 6 (all pass-4 findings) |
| **Novelty score** | 0.1 (single nit; semantic polish only) |
| **Median severity** | NIT |
| **Verdict** | CONVERGENCE TRACK |

Novelty: **LOW** — the only new finding is a documentation comment polish; not a substantive gap. Spec has substantively converged on content; remaining iteration is cosmetic.

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 0 |
| MEDIUM | 0 |
| LOW | 0 |
| NIT | 1 |

**Overall Assessment:** PASS. v1.3 fix burst fully resolved both HIGH findings (SS-04 fabricated subsystem name + write_file API drift) and all carryover LOW findings. The single NIT is SKIP-FIX eligible per S-7.03.

**Convergence:** ON TRACK (clock 1_of_3 of 3 required NITPICK_ONLY passes per ADR-013). Two more NITPICK_ONLY passes required for full convergence.

**Readiness:** Story content is implementation-ready pending S-8.10 status=done external blocker. No revision required from S-8.04 author. POLICY 6, POLICY 7, POLICY 8 all satisfied. POLICY 5 satisfied via Stretch-Anchor Disclosure rewrite. Empirical anchors all verified.
