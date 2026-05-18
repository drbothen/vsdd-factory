---
document_type: adversary-pass-report
level: ops
title: "S-15.14 LOCAL Adversary Cascade — Pass 9"
producer: adversary
timestamp: 2026-05-18T00:00:00Z
phase: per-story-step-4-local-adversary-cascade
story: S-15.14
pass: 9
verdict: MEDIUM
finding_count: { critical: 0, high: 0, medium: 2, low: 2, nitpick: 0, process_gap: 0 }
streak_3_clean: "0/3"
---

# S-15.14 LOCAL Adversary Cascade — Pass 9

## Part A — Findings

### F-P9-001 — MEDIUM — Compaction summary rows misattribute E-10 pass-9..14 preservation location
- **Location:** STATE.md:91, STATE.md:131
- **Evidence:** Rows claim "preserved in cycles/v1.0-brownfield-backfill/burst-log.md" but actual preservation is in per-pass files E-10-pass-9.md..E-10-pass-14.md. burst-log.md has zero E-10 h2 entries.
- **Recommendation:** state-manager — update cite to per-pass files.

### F-P9-002 — MEDIUM — Active Branches factory-artifacts SHA stale (pass-7 not pass-8)
- **Location:** STATE.md:177
- **Evidence:** Row cites SHA 66296e29 (pass-7); actual pass-8 SHA is af6ddabd / f6219e6b.
- **Recommendation:** state-manager — D-445(c)+D-446(d)+D-447(c)+D-449(e) SHA-advance on compaction burst missed.

### F-P9-003 — LOW — Concurrent Cycles Status header stale at pass-3
- **Location:** STATE.md:186
- **Evidence:** Bolded summary at pass-3 even though body trail captures pass-4..pass-8. Header was not advanced across 5 consecutive bursts.
- **Recommendation:** state-manager — update bolded header to most-recent pass.

### F-P9-004 — LOW — Compaction trend label mislabeled (passes 9-14 vs full 14-pass cascade)
- **Location:** STATE.md:91
- **Evidence:** Label says "passes 9-14 trend 22→11→16→16→12→2→1→4→5→4→6→7→5→8" (14 values = full cascade). Either label wrong or data wrong.
- **Recommendation:** state-manager — correct label to "passes 1-14 cascade trend" OR trim data to passes 9-14.

## Part B — Summary

**Verdict:** MEDIUM
**Counts:** 0C + 0H + 2M + 2L + 0N + 0PG = 4 findings
**Streak:** 1/3 → **0/3 RESET** (MEDIUM resets per BC-5.39.001)
**Trajectory:** 16→9→8→2→0→1→1→0→4

**Pass-8 verification:** F-P7-001 (D-chain currency) CLOSED. 5-PC E2E all PASS against production STATE.md. TD-VSDD-097 EXTENSION present.

**Burst-log pass-8 entry:** all 8 D-444(c) blocks; literal stdout for 5 PCs + wc-l pre/post.

**Compaction quality:** F5 pass-60..74 preservation VERIFIED in cycles/v1.0-feature-engine-discipline-pass-1/burst-log.md. E-10 pass-9..14 preservation CITE WRONG but content actually exists in per-pass files. No data loss.

**Fix-routing:** All 4 findings → state-manager (mechanical sibling-sweep cleanup).

**Novelty:** MEDIUM — new sibling-sweep class: compaction bursts MUST verify cited preservation paths exist + Active Branches advance + Concurrent Cycles header advance. Codify as forward-looking process-gap (TD-VSDD-098 candidate).

**Honesty:** Genuine real-artifact defects verified by direct grep + file existence checks. Not manufactured.
