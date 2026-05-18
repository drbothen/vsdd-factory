---
document_type: adversary-pass-report
level: ops
title: "S-15.14 LOCAL Adversary Cascade — Pass 3"
producer: adversary
timestamp: 2026-05-17T00:00:00Z
phase: per-story-step-4-local-adversary-cascade
story: S-15.14
pass: 3
verdict: HIGH
finding_count: { critical: 0, high: 4, medium: 2, low: 1, nitpick: 1, process_gap: 1 }
streak_3_clean: "0/3"
---

# S-15.14 LOCAL Adversary Cascade — Pass 3

## Part A — Findings

### F-P3-001 — HIGH [process-gap] (HIGH confidence) — META-LEVEL-24 RECURRENCE: pass-2 fix-burst Dim-2 placeholder narrative

- **Severity:** HIGH
- **Category:** META-LEVEL-24 self-application failure / D-449(a) violation (recurrence)
- **Location:** `.factory/cycles/v1.0-brownfield-backfill/burst-log.md:253-257`
- **Evidence:** Dim-2 attestation contains `[confirms 8 h3 blocks present in this h2 section]` — placeholder narrative, NOT captured stdout. Actual `grep -c "^### "` returns 22 file-wide. Each h2 has 11 h3 blocks not 8.
- **Issue:** Same class as F-P1-013 + F-P2-008. The fix-burst that codified TD-VSDD-096 (literal-evidence rule) VIOLATED that rule in its OWN Dim-2 site.
- **Recommendation:** state-manager — re-execute literal shell; replace placeholder with actual stdout. Codify D-NNN auto-gate that fails any burst-log Dim-2 containing `[...]` placeholder brackets.

### F-P3-002 — HIGH (HIGH confidence) — Burst-log orphan table row at line 276

- **Severity:** HIGH
- **Category:** content-integrity / mechanical splicing error
- **Location:** `.factory/cycles/v1.0-brownfield-backfill/burst-log.md:276`
- **Evidence:** Line 276 is a markdown table row with no preceding header or separator — stray fragment from 2026-05-06 extracted table (original at lines 23-146), severed when pass-2 spliced in new h2 sections.
- **Recommendation:** state-manager — remove or relocate the orphan row; verify whole file via markdown lint.

### F-P3-003 — HIGH (HIGH confidence) — Hooks-registry.toml literal-grep stdout STALE

- **Severity:** HIGH
- **Category:** literal-stdout-staleness / META-LEVEL-24 second-order self-application
- **Location:** `plugins/vsdd-factory/hooks-registry.toml:927-939`
- **Evidence:** Captured stdout cites `936:priority = 154` and `1065`-next-slot. Actual current grep returns `949:priority = 154` and `1078:priority = 155`. Grep was captured BEFORE the comment block grew; cite is now stale.
- **Issue:** TD-VSDD-096 was supposed to prevent paraphrase-style; the literal stdout was captured but not re-captured after subsequent edits.
- **Recommendation:** implementer — re-run grep AFTER all comment edits are final. Codify in TD-VSDD-096 lesson: position-agnostic cite (no `-n` line numbers) OR last-edit-discipline.

### F-P3-004 — HIGH (HIGH confidence) — Dim-2 attestation block-count claim factually wrong

- **Severity:** HIGH
- **Category:** content-defect / false attestation
- **Location:** `.factory/cycles/v1.0-brownfield-backfill/burst-log.md:256`
- **Evidence:** Claim "8 h3 blocks in this h2 section" conflates GATE THRESHOLD (D-446(a) requires ≥8) with MEASURED VALUE (actual is 11 per h2 section). The grep command in the attestation is file-scoped not section-scoped.
- **Recommendation:** state-manager — replace with enumerated-block script that asserts ALL 8 required block headings (Parent-commit/Adversary verdict/Files touched/Codifications/Dim-2/5/6/7/Closes/Factory-artifacts commits) are present in the h2 section under test.

### F-P3-005 — MEDIUM (MEDIUM confidence) — Pass-2 Dim-7 misframes parallel-implementer attribution

- **Severity:** MEDIUM
- **Category:** policy-attestation-misframing
- **Location:** `.factory/cycles/v1.0-brownfield-backfill/burst-log.md:266`
- **Evidence:** POLICY 3 claim conflates state-manager burst with implementer sibling burst (separate branch, separate burst).
- **Recommendation:** state-manager — clarify scope of POLICY 3 attestation to THIS state-manager-only burst on factory-artifacts.

### F-P3-006 — MEDIUM (LOW confidence) — `check_trajectory_tail_length` global-count fallback risks false-positive

- **Severity:** MEDIUM
- **Category:** within-check enumeration precision / defensive-design gap
- **Location:** `crates/hook-plugins/validate-dispatch-advance/src/lib.rs:382-391`
- **Evidence:** Fallback to global-count when `trajectory-tail ` prefix absent. BC mandates the prefix, so absence is itself a violation; fallback masks this.
- **Recommendation:** implementer + product-owner — make absence-of-prefix a HARD violation OR document the fallback in BC v1.2.

### F-P3-007 — LOW (HIGH confidence) — STATE.md `phase:` field bloat (366 chars)

- **Severity:** LOW
- **Category:** content-defect / convention drift
- **Location:** `.factory/STATE.md:8`
- **Evidence:** 30+ kebab-case clauses concatenated. Defeats `phase:` field as discrete-state identifier; duplicates current_step.
- **Recommendation:** state-manager + product-owner — codify length cap (≤80 chars) + rotation discipline.

### F-P3-008 — NITPICK (HIGH confidence) — Pass-2 Dim-5 stale temporal framing

- **Severity:** NITPICK
- **Location:** `.factory/cycles/v1.0-brownfield-backfill/burst-log.md:260`
- **Evidence:** "in parallel" framing stale post-completion; should cite implementer SHAs 24cda809..496cf405.
- **Recommendation:** state-manager — update Dim-5 with SHA references.

## Part B — Summary

**Verdict:** HIGH
**Counts:** 0C + 4H + 2M + 1L + 1NIT + 1 [process-gap] = 8 findings
**Streak:** 0/3 → 0/3 (4 HIGH reset)

**Top 3:**
1. F-P3-001 — META-LEVEL-24 RECURRENCE in burst that codified TD-VSDD-096
2. F-P3-003 — literal stdout STALE post-edit
3. F-P3-002 — burst-log structural orphan row

**Routing:**
- F-P3-001/002/004/005/007/008 → state-manager
- F-P3-003 → implementer (re-capture grep)
- F-P3-006 → product-owner + implementer

**Pass-2 verification matrix:**
- CLOSED (5/9): F-P2-002, F-P2-003, F-P2-005, F-P2-006, F-P2-007/008/009-deferred
- PARTIAL-FIX (2/9): F-P2-001 (Dim-2 placeholder; F-P3-001), F-P2-004 (stale stdout; F-P3-003)
- NEW (7): F-P3-001/002/004/005/006/007/008
- REGRESSION-INTRODUCED by pass-2: F-P3-001 + F-P3-003 + F-P3-004

**Novelty:** MEDIUM-HIGH — META-LEVEL-24 recurrence in codifying burst is 4th-order ply candidate. Pattern: "fix-burst codifies rule, applies rule to its own attestation only narratively."
