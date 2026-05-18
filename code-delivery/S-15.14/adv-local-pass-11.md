---
document_type: adversary-pass-report
level: ops
title: "S-15.14 LOCAL Adversary Cascade — Pass 11"
producer: adversary
timestamp: 2026-05-18T00:00:00Z
phase: per-story-step-4-local-adversary-cascade
story: S-15.14
pass: 11
verdict: HIGH
finding_count: { critical: 0, high: 2, medium: 0, low: 0, nitpick: 0, process_gap: 0 }
streak_3_clean: "0/3"
---

# S-15.14 LOCAL Adversary Cascade — Pass 11

## Part A — Findings

### F-P11-001 — HIGH — BC v1.2 invariant 6(b) under-specifies LENGTH-count scope end-boundary
- **Location:** BC-5.39.006.md:165-170 vs lib.rs:396-401
- **Evidence:** BC text says "substring AFTER marker"; code narrows to "marker → first `;`". Production current_step has 14 `→\d+` matches in full substring-after-marker but only 4 within first-semicolon segment.
- **Issue:** SPEC-wins per CLAUDE.md Architectural Authority rule 12. Same paper-fix class as original F-P1-008.
- **Routing:** product-owner — amend BC v1.3 (DONE this fix-burst).

### F-P11-002 — HIGH — 6th META-LEVEL class: pass-10 Gate 3 uses synthetic `echo` not production STATE.md read
- **Location:** burst-log.md:935-939 (pass-10) + L815-819 (pass-9 pattern)
- **Evidence:** Gate 3 ran `echo 'trajectory-tail ->9->9->9->9' | grep -oE "->[0-9]+" | wc -l` — ASCII `->` not Unicode `→`; hand-crafted string not production read. Gates structurally present but content-inert.
- **Issue:** TD-VSDD-099 closed STRUCTURAL presence of Dim blocks; F-P11-002 reveals CONTENT inertness of attestations. 6th META-LEVEL class.
- **Routing:** state-manager — retroactively fix pass-10 Gate 3 + codify TD-VSDD-100 + apply production-read pattern in pass-11 entry.

## Part B — Summary

**Verdict:** HIGH
**Counts:** 0C + 2H + 0M + 0L + 0N + 0PG = 2 findings
**Streak:** 0/3 → **0/3** (HIGH per BC-5.39.001)
**Trajectory:** 16→9→8→2→0→1→1→0→4→1→**2**

**Pass-10 verification:** F-P10-001 CLOSED (pass-9 Dim-7 inserted retroactively). TD-VSDD-099 CODIFIED. Pass-10 own entry has 4 Dim blocks + 10 canonical blocks. But Dim-2 PC attestations are content-inert (F-P11-002).

**5-PC E2E:** PC2/3/5/6 PASS; PC4 PASS per code semantics (BC v1.3 codifies); spec-vs-code drift closed at v1.3.

**Novelty:** HIGH. 6th META-LEVEL class found. Codifying-burst closes structural presence but admits content-inert attestations. Pattern: each codification opens an adjacent class.

**Routing:** F-P11-001 → product-owner (BC v1.3 — DONE); F-P11-002 → state-manager (Gate 3 retrofit + TD-VSDD-100).

**Honesty:** Both findings real, file-grep-verified. Not manufactured.
