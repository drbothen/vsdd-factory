---
document_type: adversary-pass-report
level: ops
title: "S-15.14 LOCAL Adversary Cascade — Pass 7"
producer: adversary
timestamp: 2026-05-18T00:00:00Z
phase: per-story-step-4-local-adversary-cascade
story: S-15.14
pass: 7
verdict: HIGH
finding_count: { critical: 0, high: 1, medium: 0, low: 0, nitpick: 0, process_gap: 0 }
streak_3_clean: "0/3"
---

# S-15.14 LOCAL Adversary Cascade — Pass 7

## Part A — Findings

### F-P7-001 — HIGH — Pass-6 fix-burst introduced new regression: current_step D-chain max=D-449 < body max=D-476 → at-deploy stale-D-chain BlockWithFix

- **Severity:** HIGH
- **Category:** spec-vs-artifact-reality drift / replacement-regression / META-LEVEL self-violation (3rd-of-class after F-P3-006 and F-P6-001)
- **Location:** `.factory/STATE.md:15`
- **Evidence:** pre-fix current_step contained anchor list `D-419/420/421/441/442/443/444/449` → max_cited=449. STATE.md body S-15.09 merge row has D-476. Per BC v1.2 invariant 7, max_cited < max_in_file is stale-D-chain BlockWithFix.
- **Issue:** Pass-6 fix closed PC6 (marker) but dropped pre-existing `D-476 latest brownfield` cite, replacing PC6 violation with PC5 violation. Same META-LEVEL self-violation class.
- **Root cause:** Orchestrator dispatch template for pass-6 omitted D-chain currency clause; state-manager followed template verbatim. TD-VSDD-097 scoped too narrowly (PC6 marker only).
- **Recommendation:** state-manager fix; orchestrator extends TD-VSDD-097 to cover ALL 5 BC v1.2 PCs (PC2 + PC3 + PC4 + PC5 + PC6).

## Part B — Summary

**Verdict:** HIGH
**Counts:** 0C + 1H + 0M + 0L + 0N + 0PG = 1 finding
**Streak:** 0/3 → **0/3** (HIGH; no advance)
**Trajectory:** 16 → 9 → 8 → 2 → 0 → 1 → **1**

**Pass-6 verification:**
- F-P6-001 (PC6 marker): CLOSED — `trajectory-tail →9→9→9→9` present
- TD-VSDD-097: CODIFIED but scoped too narrowly
- Burst-log entry: 8 D-444(c) blocks present with literal pre+post grep
- NEW REGRESSION-INTRODUCED: F-P7-001 (PC5 stale D-chain)

**E2E PC matrix:** PC2 PASS, PC3 PASS, PC4 PASS, PC5 **FAIL (F-P7-001)**, PC6 PASS

**Routing:**
- F-P7-001 → state-manager fix
- [process-gap subsumed] → orchestrator: extend TD-VSDD-097 to cover ALL 5 BC v1.2 PCs

**Novelty:** HIGH — 3rd META-LEVEL self-violation class instance. Codification-without-application recurrence demonstrates structural countermeasure needed (e.g., bin/preflight-current-step script running validate_state_md against staged content before state-manager commits).

**Honesty:** Genuine real-artifact defect verified by direct grep + BC/lib.rs trace.
