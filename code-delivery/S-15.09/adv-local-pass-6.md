---
document_type: adversary-pass-report
level: ops
title: "S-15.09 LOCAL Adversary Cascade — Pass 6"
producer: adversary
timestamp: 2026-05-17T00:00:00Z
phase: per-story-step-4-local-adversary-cascade
story: S-15.09
pass: 6
verdict: HIGH
finding_count: { critical: 0, high: 2, medium: 1, low: 2, nitpick: 1, process_gap: 0 }
streak_3_clean: "0/3"
---

# S-15.09 LOCAL Adversary Cascade — Pass 6

## Part A — Findings

### F-P6-001 HIGH — BC-5.39.005 PC4 contradicts story v1.5 + implementation (spec triad drift)

- **Location:** BC `:73`; story v1.5 PC4 `:178`; impl `lib.rs:60`.
- **Evidence:** BC PC4 reads `max_bytes = 65536 and timeout_ms = 2000`. Story v1.5 PC4 says `max_bytes=524_288`. Implementation `MAX_BYTES_STATE_MD: u32 = 524_288`.
- **Issue:** Pass-5 fix-burst raised the cap in code + story but did NOT propagate to BC (L3 source of truth). POLICY 8 atomic-propagation violation. A future implementer reading BC PC4 would write `max_bytes = 65536`, regressing F-P5-002.
- **Recommendation:** Sync BC PC4 to 524288 same-burst as story v1.5.

### F-P6-002 HIGH — validate-index-cite-refresh has 3× silent-inert `host::read_file(...65536...)` callsites

- **Location:** `crates/hook-plugins/validate-index-cite-refresh/src/lib.rs:312`, `:377`, `:412`.
- **Evidence:** Real STATE.md = 95,185 bytes (145% of 65536 cap). `host::read_file` returns `Err(HostError::OutputTooLarge)` on oversize → hook fail-opens (Continue + log_warn). The cross-cell STATE.md cite check (`cross_cell_check`) is SILENTLY INERT against the real production target.
- **Issue:** Same META-LEVEL-24 silent-inert-validator class F-P5-002 closed for validate-state-structure is alive in this S-15.07 sibling. TD-VSDD-060 sibling-site sweep was incomplete: F-P5-002 fixed the local site but did not sweep sibling crates. validate-burst-log likely has the same defect (reads burst-log.md = 608,723 bytes; 9.4x cap).
- **Severity:** HIGH cross-story (out-of-S-15.09 scope; affects already-merged S-15.07 / S-15.11 on develop). Route as Drift Item with explicit follow-up story attachment per CLAUDE.md Principle 3.

### F-P6-003 MEDIUM — Story v1.5 Token Budget cites stale "(v1.3; 13 ACs)"

- **Location:** story `:768`.
- **Issue:** Frontmatter v1.5 but Token Budget cites v1.3. POLICY 14/17 frontmatter parity violated.
- **Recommendation:** Update cite to v1.5.

### F-P6-004 LOW — Story Risk + EC-012 call `524_288` a "placeholder" after F-P5-002 implementation

- **Location:** story `:498`, `:810`.
- **Issue:** v1.5 reconciled the value but "placeholder" wording lingers.
- **Recommendation:** Replace with accurate descriptive wording.

### F-P6-005 LOW — Changelog rows v1.4 + v1.5 both attributed to "Pass-5"

- **Location:** story `:857-858`.
- **Issue:** Reader cannot distinguish stage from version label.
- **Recommendation:** Clarify stage in row description.

### F-P6-006 NITPICK — `scan_for_last_wc_l` private; sibling extractors `pub`

- **Location:** `lib.rs:145`.
- **Issue:** F-P5-005 narrative claimed visibility consistency; this internal scanner stayed private.
- **Recommendation:** Document the deliberate-private intent OR promote to pub.

## Part B — Production-Grade Default Audit

- F-P6-001 + F-P6-002 form a coordinated TD-VSDD-060 sibling-site sweep miss. F-P5-002 partially closed the silent-validator class but the sweep was incomplete (BC not synced + 3 sibling-crate callsites left).
- No MVP-deferrals, no paper-fix smells, no pending-architect markers in spec body.
- Production-grade default mandates F-P6-001 in-scope fix (BC PC4 same-burst sync). F-P6-002 is legitimately cross-story (affects already-shipped S-15.07/S-15.11 code) — route as Drift Item per Principle 3.

## Part C — Self-Application Audit (META-LEVEL)

- Iron Law respected: no prior pass reports read.
- Cross-component reasoning surfaced: F-P5-002 fix at one site reveals the same defect class at sibling sites.
- Orchestrator-independently verified F-P5-002 base rate (real STATE.md = 95,185 bytes); the sibling-crate finding by-extension is sound.
- POLICY 13 regex-alternation + POLICY 15 verbatim-stdout compliance throughout.

## Verdict & Streak

- Pass-6 verdict: **HIGH** (2H + 1M + 2L + 1N).
- Streak: **0/3** (was 0/3 from pass-5; HIGH findings maintain reset).
- In-scope fix-burst required + cross-story Drift Item.

## Fix-burst routing (orchestrator-routed; complete)

- product-owner @ `c1928240` factory-artifacts — F-P6-001 closed (BC v1.2→v1.3 PC4 sync).
- story-writer @ `57321f77` factory-artifacts — F-P6-003/004/005 closed (story v1.5→v1.6).
- implementer @ `8b12e772` feature branch — F-P6-006 closed (Option B doc-comment).
- state-manager — this persistence commit + F-P6-002 cross-story Drift Item record.

## CROSS-STORY FINDING — F-P6-002 routing decision

F-P6-002 is OUT OF SCOPE for S-15.09 (it targets validate-index-cite-refresh = S-15.07 crate, validate-burst-log = S-15.11 crate). Both crates already shipped on develop. Recorded as Drift Item in STATE.md per CLAUDE.md Principle 3 with explicit future-story attachment. Severity HIGH operational — silent-inert security/correctness hooks on shipped code is a wave-gate-class regression candidate.

## Closure verification

- BC v1.3: PC4 cites `max_bytes = 524288` matching story v1.6 + impl. Spec triad consistent.
- Story v1.6: Token Budget refreshed to v1.5 (wait — actually story is v1.6 now per story-writer commit; cite should be v1.6); placeholder removed; changelog v1.4/v1.5 stages distinguished.
- Implementer feature branch: F-P6-006 doc-comment intent codified.
