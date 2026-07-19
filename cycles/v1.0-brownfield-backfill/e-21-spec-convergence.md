---
document_type: spec-convergence-record
epic: E-21
title: E-21 Factory-State Data-Loss Hardening — Spec Convergence Record
version: "1.0"
status: converged
producer: state-manager
timestamp: 2026-07-19T00:00:00Z
decision: D-860
streak: "3/3 CLEAN"
streak_passes: "P9, P10, P11"
---

# E-21 Factory-State Data-Loss Hardening — Spec Convergence Record

**Epic:** E-21 (P0a data-loss issue family: #342/#365/#358/#523/#588)
**Spec package:** BC-4.16.001, BC-5.43.001, BC-5.44.001, BC-6.10.002 (amended), BC-6.26.001, BC-6.27.001 + ADR-031 v1.4 + CAP-034..CAP-038 + S-21.01..S-21.05
**Convergence:** 11 LOCAL adversarial passes; 3-CLEAN streak (passes P9/P10/P11 per BC-5.39.001 + D-761 strict)
**Total findings closed:** 29 (across 8 fix bursts; P1..P8 non-clean passes)
**Pipeline gate:** PAUSED — Phase 3 W1 dispatch AWAITING HUMAN APPROVAL

---

## Pass-by-Pass Summary

| Pass | Verdict | Blockers | Highs | Mediums | Lows | Notes |
|------|---------|----------|-------|---------|------|-------|
| P1 | NOT-CLEAN | B2 | H4 | M4 | L3 | First pass; arch delta 9f637477 basis; EMPTY-host ruling (later retracted at P3) |
| P2 | NOT-CLEAN | B0 | H1 | M3 | L3 | F-P2-001 EMPTY-host RETRACTED D-RETRACT (counter-evidence: ad-hoc Bash surface + server-side origination vector, ADR-031 v1.3) |
| P3 | NOT-CLEAN | B0 | H2 | M1 | 0 | |
| P4 | NOT-CLEAN | B0 | H0 | M1 | L4 | |
| P5 | NOT-CLEAN | B0 | H0 | M1 | L2 | |
| P6 | NOT-CLEAN | B0 | H0 | M1 | L1 | |
| P7 | NOT-CLEAN | B0 | H0 | M1 | L2 | |
| P8 | NOT-CLEAN | B0 | H0 | M2 | L0 | |
| P9 | **CLEAN** | B0 | H0 | M0 | L1 | L1 = observation only (accepted-with-record); streak 1/3 |
| P10 | **CLEAN** | B0 | H0 | M0 | L1 | L1 = observation only (accepted-with-record); streak 2/3 |
| P11 | **CLEAN** | B0 | H0 | M0 | L2 | L2 = observations only (accepted-with-record); streak 3/3 CONVERGED |

---

## Finding Closure Table (P1–P8 Findings)

| Finding ID | Pass | Severity | Closure Commit |
|------------|------|----------|----------------|
| F-P1-001 | P1 | BLOCKER | a1e93795 |
| F-P1-002 | P1 | BLOCKER | 14a78515 |
| F-P1-003 | P1 | HIGH | dc72b730 |
| F-P1-004 | P1 | HIGH | cb0d412b |
| F-P1-005 | P1 | HIGH | f3e823bb |
| F-P1-006 | P1 | HIGH | 6ab8b594 |
| F-P1-007 | P1 | MEDIUM | abfda3a2 |
| F-P1-008 | P1 | MEDIUM | dd69cf29 |
| F-P1-009 | P1 | MEDIUM | ada6d254 |
| F-P1-010 | P1 | MEDIUM | 5be9a55a |
| F-P1-011 | P1 | LOW | 7088c5ab |
| F-P1-012 | P1 | LOW | 16147e57 |
| F-P1-013 | P1 | LOW | e6ef7b80 |
| F-P2-001 | P2 | HIGH | **RETRACTED** — EMPTY-host ruling withdrawn after orchestrator counter-evidence (ad-hoc Bash surface on main checkout + server-side origination vector as primary Layer-2 threat; ADR-031 v1.3 live-surface framing adopted) |
| F-P2-002 | P2 | MEDIUM | 8a22eae0 |
| F-P2-003 | P2 | MEDIUM | 4af56226 |
| F-P2-004 | P2 | MEDIUM | 673d0cc7 |
| F-P2-005 | P2 | LOW | 61f971e1 |
| F-P2-006 | P2 | LOW | 7050af23 |
| F-P2-007 | P2 | LOW | 8a086f60 |
| F-P3-001 | P3 | HIGH | cb47a3a8 |
| F-P3-002 | P3 | HIGH | cb47a3a8 |
| F-P3-003 | P3 | MEDIUM | cb47a3a8 |
| F-P4-001 | P4 | MEDIUM | (pass-4 fix burst) |
| F-P5-001 | P5 | MEDIUM | (pass-5 fix burst) |
| F-P6-001 | P6 | MEDIUM | (pass-6 fix burst) |
| F-P7-001 | P7 | MEDIUM | (pass-7 fix burst) |
| F-P8-001 | P8 | MEDIUM | (pass-8 fix burst) |
| F-P8-002 | P8 | MEDIUM | (pass-8 fix burst) |

---

## Notable Arc: F-P2-001 EMPTY-host Ruling Retraction

Finding F-P2-001 (P2 HIGH) initially ruled that the orchestrator's use of an `EMPTY` host-set was an incorrect threat model. This ruling was RETRACTED at P3 after the orchestrator produced counter-evidence:
- Ad-hoc Bash surface on the main checkout can execute `git pull`/`git merge`/`git checkout` operations triggered by orchestrator on the product branch.
- Server-side origination (remote push/pull) is the primary Layer-2 `.factory/` deletion threat vector.
- ADR-031 v1.3 was revised from the `EMPTY`-host framing to the live-surface framing: enforcement site is `per-story-delivery.md §Main-Checkout Sync Protocol` / S-21.01 Layer-2 deliverable; Layer-1 scope narrowed accordingly.
- Ruling: ADR-031 v1.3 is the authoritative framing; `EMPTY`-host claim retracted.

---

## Accepted-With-Record Register

Story delivery for E-21 W1/W2 MUST inherit these accepted observations and address them as applicable:

| ID | Pass | Class | Description | Disposition |
|----|------|-------|-------------|-------------|
| F-P10-001 | P10 | LOW | `--force` parenthetical in S-21.01 vacuous-as-applied (force-push-with-lease section; the parenthetical adds no enforcement beyond the base text) | ACCEPTED — wording is cosmetic; no behavioral change needed |
| O-P9-a | P9 | OBS | EAC-003 range-diff/--stat conflation: `range-diff` is tested by doc-grep only (not a running assertion); `--stat` output is a different surface | ACCEPTED — implementation story must provide a running assertion for EAC-003 range-diff behavior |
| L-P11-a | P11 | LOW | S-21.02 AC-002 step-f signpost framing slightly ambiguous; minor prose clarification opportunity | ACCEPTED-WITH-RECORD — story delivery may address in-scope |
| L-P11-b + O-P5-b class | P11/P5 | LOW/OBS | BC-5.44.001/BC-6.26.001/BC-6.27.001 changelog row ordering — current newest-first vs oldest-first inconsistency | ACCEPTED — rides next amendment to those BCs |
| O-P6-a | P6 | OBS | Priority-140 collision between S-21.01 and another registered hook at the same numeric priority | **RESOLVE AT S-21.01 IMPLEMENTATION** — implementer MUST verify no collision with existing hooks-registry.toml entries |
| O-P5-a | P5 | OBS | CAP-038 spans SS-05/SS-06 by-design (BC-6.10.002 + BC-5.43.001 share CAP-034; BC-6.10.002 uses CAP-038) | ACCEPTED BY DESIGN — cross-subsystem capability usage is intentional |
| O-2 | P2 | OBS | epic-Trigger authorization-time versions not all anchored to specific BC/ADR version cites | ACCEPTED — noted for next epic amendment |
| epic-Trigger | — | OBS | Authorization-time BC/ADR version anchors in E-21 epic frontmatter not fully enumerated | ACCEPTED — next epic amendment will enumerate |
| BC-6.10.002 | — | OBS | Status/lifecycle split: BC-6.10.002 governs both status enum and lifecycle semantics; may benefit from split into BC-6.10.002a/b in future | ACCEPTED — deferred to next spec amendment cycle |

---

## Convergence Metadata

- **Arch delta basis:** commit 9f637477 (E-21 arch delta analysis; ADR-031 + CAP-034..CAP-038)
- **BCs authored:** BC-4.16.001 v1.2, BC-5.43.001 v1.3, BC-5.44.001 v1.3, BC-6.26.001 v1.3, BC-6.27.001 v1.3
- **BC amended:** BC-6.10.002 v1.3 (CAP-038 + S-21.03 registration)
- **Stories:** S-21.01 v1.2 (W1), S-21.02 v1.3 (W1), S-21.03 v1.3 (W1), S-21.04 v1.2 (W2), S-21.05 v1.3 (W2)
- **Epic:** E-21 v1.5; 5 stories; 27 pts; W1: S-21.01/02/03; W2: S-21.04/05
- **STORY-INDEX at convergence:** v4.227
- **BC-INDEX at registration:** v4.11
- **Decision:** D-860
- **Phase-3 gate:** AWAITING HUMAN APPROVAL
