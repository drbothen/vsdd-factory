---
document_type: adversarial-review
review_type: spec-cascade
artifact_under_review: "BC-5.39.009 v1.3 + S-15.17 v1.4"
cycle: brownfield-backfill
pass: 4
producer: adversary
timestamp: 2026-05-28
verdict: HIGH
finding_count: 16
finding_count_by_severity:
  critical: 1
  high: 6
  medium: 5
  low: 2
  nitpick: 1
  process_gap: 1
streak_status: "STREAK 0/3 after pass-4 (HIGH verdict; reset)"
---

# Adversarial Review — BC-5.39.009 v1.3 + S-15.17 v1.4 Spec Cascade Pass 4

## Part A — Finding Set

### F-S15.17-SP4-001 — CRITICAL — `count_trajectory_arrows == 4` equality semantics produce false-Block on multi-tail extracted regions

BC inv-4 specifies `count == 4 equality semantics` operating on extracted region. PC3 says "the bottommost non-archived non-compacted row". Story T-5 `count_trajectory_arrows` accepts `text: &str` with no scoping and counts ALL arrows. Phase Progress section contains ~59 arrow segments (literal-shell verified). If implementer passes whole-section text, `count == 4` false-Blocks. The mitigation "use windowed scan if multiple tails possible" is non-prescriptive — punts to implementer. Equality semantics REQUIRE per-row extraction at extractor level.

**Routing:** product-owner — add ONE-tail-per-extracted-region precondition; tighten PC3 to "the bottommost row text (single row, between `|` delimiters)". Story T-5 `extract_phase_progress_latest_row` must return single row.

### F-S15.17-SP4-002 — HIGH — STATE.md `## Phase Progress` line cite STALE (61 claimed, actual 64) — TD-VSDD-091 anti-volatile-pin INSIDE the POLICY 5 v1.3 cure

BC PC3 (line 257) + Story T-5 (552-554) cite `61:## Phase Progress`. Actual:
```
$ grep -n "^## Phase Progress" .factory/STATE.md
64:## Phase Progress
```
Line drifted 3 lines between authoring and pass-4 review. POLICY 5 v1.3 SDK-grounding mandate introduced precisely to prevent stale claims about external artifacts — yet the mandate's own captures are anti-volatile-pinned by definition.

**Routing:** product-owner — strip ALL line numbers from BC body grep evidence; keep command + content excerpts only. §SDK Grounding Evidence section should use re-execute-at-burst discipline OR stable anchors (heading prefix-match, no line cite).

### F-S15.17-SP4-003 — HIGH [regression of F-SP3-001] — Story Architecture Mapping table 758-760 STILL hardcodes paused F5 cycle path

Story lines 758-760:
```
| Target cycle INDEX.md | `.factory/cycles/v1.0-feature-engine-discipline-pass-1/INDEX.md` | ...
| Target burst-log.md | `.factory/cycles/v1.0-feature-engine-discipline-pass-1/burst-log.md` | ...
| Target lessons.md | `.factory/cycles/v1.0-feature-engine-discipline-pass-1/lessons.md` | ...
```
Hardcoded F5 paths in section the implementer would read to understand WHICH files the hook targets. Direct regression of F-SP3-001 CRITICAL closure in unreached section.

**Routing:** story-writer — use `<active-cycle>` placeholder OR sentinel `.factory/cycles/${ACTIVE_CYCLE}/...` per BC §Architecture Anchors lines 564-566.

### F-S15.17-SP4-004 — HIGH — BC PC9 specifies `burst-log.md latest-pass Dim-7 (Attestation)` extraction but `Dim-7` heading DOES NOT EXIST in active-cycle burst-log.md

```
$ grep -E "^## Dim-7|Dim-7 \(Attestation\)" .factory/cycles/v1.0-brownfield-backfill/burst-log.md
(zero output)
```
PC9 extractor target absent. Story T-5 line 598-600 scans for `## Dim-7` and finds nothing → site treated as "present" (PC3 last-resort fail-open) → site permanently silent. META-LEVEL-30 route (b) recurrence INSIDE the very hook that supposedly closes route (b).

**Routing:** product-owner + state-manager — specify actual Dim-7 heading text via §SDK Grounding Evidence grep against both v1.0-brownfield-backfill/burst-log.md AND v1.0-feature-engine-discipline-pass-1/burst-log.md; either align headings OR scope PC9 to one cycle.

### F-S15.17-SP4-005 — HIGH — `extract_current_cycle()` function called by T-5 but never specified

Story T-5 line 458: `let active_cycle = extract_current_cycle(&state_md_str).unwrap_or_default();`. No spec anywhere in BC or story. Production STATE.md uses bare-form `current_cycle: v1.0-brownfield-backfill` — but may be single-quoted, double-quoted, have trailing comment, or be multi-line block-scalar. Without extractor spec, F-SP3-001 cycle-resolution cure is incomplete.

**Routing:** product-owner — add `extract_current_cycle(state_md_content: &str) -> Option<String>` extractor spec to BC §Architecture Anchors with explicit handling parallel to EC-017 current_step.

### F-S15.17-SP4-006 — HIGH [regression of F-SP3-001 / F-SP3-008] — INDEX.md cycle-path guard uses substring `String::contains` → false-positive on overlapping cycle names

Story T-5 line 459:
```rust
if active_cycle.is_empty() || !file_path.contains(active_cycle.as_str()) {
```
If `active_cycle == "v1.0-brownfield-backfill"`, substring `contains` returns true for `v1.0-brownfield-backfill-bolt-on/INDEX.md` (false-positive — STATE.md actually has Concurrent Cycles row labeled this way) AND `v1.0-brownfield-backfill-archive/INDEX.md`. Path-component-walk discipline applied to STATE.md parent guard (F-SP3-008 cure) NOT applied to cycle-path guard. Cure is half-applied.

**Routing:** product-owner + story-writer — use `Path::new(file_path).components().any(|c| c.as_os_str() == active_cycle.as_str())`.

### F-S15.17-SP4-007 — HIGH — BC §Cure-Extension Parsimony Note point 3 narrative INVERTS PC11/PC12 collapse direction

Lines 601-609 say: "this BC collapses PC11 into PC12 (all HostError variants → fail-open)". INVERSE of actual: OLD PC11 (TooBig-specific) collapsed INTO new PC11 (uniform HostError); OLD PC13 (all-clean pass) RENUMBERED to new PC12. Story v1.4 last_amended gets it right; BC's own narrative is inverted. META-30 route (a) seed (contradictory narrative).

**Routing:** product-owner — rewrite to: "collapses the old PC11 (HostError::TooBig-specific) into the new PC11 (uniform HostError fail-open). The old PC13 (all-sites-present pass case) becomes the new PC12. Net: PC count 13 → 12."

### F-S15.17-SP4-008 — MEDIUM — Story Risk row 966 falsely claims "ACs 1-24 all carry BC-5.39.009 traces" — AC-18/19 cite toolchain/CI hygiene not BC

AC-18 trace = "Architect Q4 WASM compilation convention" (not BC anchor). AC-19 = "CLAUDE.md production-grade default" (not BC). Risk row narrative incorrect.

**Routing:** story-writer — either reword to "ACs 1-17 and 20-24 carry BC traces; AC-18/19 toolchain hygiene" OR extend AC-18/19 BC column to "BC-5.39.009 §Architecture Compliance Rules code review gate".

### F-S15.17-SP4-009 — MEDIUM — EC-020 in story not yet mirrored in BC; CLAUDE.md Canonical Principle Rule 3 deferral violation

```
$ grep -cE "^\| EC-[0-9]+" .factory/specs/behavioral-contracts/ss-05/BC-5.39.009.md
19
$ grep -cE "^\| EC-[0-9]+" .factory/stories/S-15.17-validate-trajectory-tail-cell-completeness.md
20
```
Story EC-020 (UTF-8 fail-open) referenced by AC-24 and T-5 line 627 — but BC missing the EC. "Next PO burst" deferral target is not a specific story ID per Rule 3.

**Routing:** product-owner — mirror EC-020 into BC at this burst before pass-5 close.

### F-S15.17-SP4-010 — MEDIUM — BC §SDK Grounding Evidence Grep 1 enumeration is narrative paraphrase not literal stdout (POLICY 15 violation INSIDE POLICY 5 cure)

Grep 1 shows one stdout line then "Enum body (lines 82-95)" narrative. POLICY 15 requires verbatim command + captured file:line: stdout. Same root cause (mental-model-vs-literal-grep) POLICY 5 v1.3 was supposed to prevent.

**Routing:** product-owner — re-execute with `sed -n '82,94p' crates/hook-sdk/src/host.rs` and paste literal stdout. Note: BC says "lines 82-95" but actual closing brace is line 94.

### F-S15.17-SP4-011 — MEDIUM — Story §Bidirectional Parity Audit Note POLICY 15 verbatim discipline incomplete for invariant-coverage claim

Audit shows verbatim stdout for PC-citation grep + invariant-citation grep — good. But "Invariant 2,5 covered via Architecture Compliance Rules" is narrative without grep backing. Independent verification confirms invariants 2 and 5 NOT cited in body — substance correct, form non-compliant.

**Routing:** story-writer — add third literal-shell stdout for invariant-coverage claim against audit-block-stripped story body + Architecture Compliance Rules.

### F-S15.17-SP4-012 — MEDIUM — Hardcoded F5 cycle name remains in `target_arm` rationale comment (story line 400-402)

Comment: "NOT hardcoded to v1.0-feature-engine-discipline-pass-1 (F-SP3-001 fix)". Pedagogically fine but anti-volatile-pin risk: if F5 rotates out as historical-paused-cycle reference, comment becomes stale. Same pattern in BC line 165-172.

**Routing:** product-owner + story-writer — replace cycle-name examples with structural form: "NOT hardcoded to any specific cycle name; resolved from STATE.md current_cycle: at runtime".

### F-S15.17-SP4-013 — LOW — STATE.md Last Updated cell PC2 extractor spec ambiguous about whitespace handling

PC2 spec doesn't clarify: leading/trailing whitespace; right delimiter; multi-line continuation. Production has no inner `|` so currently hypothetical.

**Routing:** product-owner — add to PC2: "Cell value extraction: capture text between second and third unescaped `|` characters; join continuation lines on whitespace."

### F-S15.17-SP4-014 — LOW — BC Test Vectors table PC11/PC12 sweep VERIFIED CLEAN (negative finding for audit trail)

All Test Vectors rows correctly cite new PC11/PC12 numbering. No defect. Including as LOW to capture sweep was done.

**Routing:** none — informational.

### F-S15.17-SP4-015 — NITPICK [regression of F-SP1-003] — Story EC-007 row 910 cites `(PC13)` (old numbering) — evaded POLICY 8 v1.2 audit grep predicate

```
$ grep -n "PC13" .factory/stories/S-15.17-validate-trajectory-tail-cell-completeness.md
910:| EC-007 | STATE.md with all 5 sites carrying `→9→9→9→9` | `HookResult::Continue`; no Block or advisory (PC13) | EC-007 |
```
Audit grep predicate `BC-5.39.009 PC[0-9]+` requires BC ID prefix. Bare `(PC13)` evades. META-31 sub-sub-route: audit-grep-predicate-too-narrow.

**Routing:** story-writer — fix EC-007 to PC12; update audit grep predicate to `(BC-5\.39\.009 )?PC[0-9]+`.

### F-S15.17-SP4-016 — PROCESS-GAP MEDIUM — `[needs-po]` deferral pattern violates CLAUDE.md Canonical Principle Rule 3

STATE.md line 305 + story line 229 surface EC-020 gap but route to "next PO burst" — not specific future story per Rule 3. Orchestrator routed F-SP3-006 (story UTF-8) but did NOT same-burst route F-SP3-006-mirror (BC EC-020).

**Routing:** orchestrator routing-rule codification — "story-local EC additions naming BC anchor REQUIRE same-burst PO mirror". Target: codify in S-15.03 or current cycle.

## Part B — Convergence Assessment

### Verdict: HIGH (1C+6H+5M+2L+1N+1PG = 16 findings)
### STREAK: 0/3 reset
### Trajectory: pass-1 14 → pass-2 11 → pass-3 14 → pass-4 **16** (REGRESSING)

### Cure Verification

**POLICY 5 v1.3 SDK-grounding:** PARTIAL success. §SDK Grounding Evidence section exists with 9 captures (verified). BUT stale line-number pins (F-SP4-002), narrative paraphrase inside captures (F-SP4-010), undocumented external claims (F-SP4-004 Dim-7, F-SP4-005 extract_current_cycle). Cure direction correct; cure scope incomplete.

**POLICY 8 v1.2 audit-block-exclusion:** PARTIAL success. Audit form correct + 12 distinct PCs verified by independent run + matches BC v1.3 exactly. BUT bare-form `(PC13)` at story EC-007 evades audit grep predicate (F-SP4-015 META-31 sub-sub-route). BUT invariant-coverage narrative inside audit block (F-SP4-011).

### Regression Sweep

3 regression-class findings (tagged [regression]):
- F-SP4-003 (F-SP3-001 Architecture Mapping hardcoded)
- F-SP4-006 (F-SP3-001/F-SP3-008 substring not path-component-walk)
- F-SP4-015 (F-SP1-003 bare PC cite evaded)

### META-LEVEL Signals

- **META-LEVEL-32 CANDIDATE (SDK-grounding-mandate-with-stale-pins):** POLICY 5 v1.3 captures stale by next burst (F-SP4-002 line 64≠61). Forward-watch: pass-5+ may surface F-SP5-XXX SDK-grounding-stale-pin class findings.
- **META-LEVEL-31 sub-sub-route (audit-grep-predicate-too-narrow):** POLICY 8 v1.2 audit-block-exclusion prevents self-counting but predicate too narrow for bare-form citations (F-SP4-015). Cure-of-cure-cure recursion.
- **META-LEVEL-24 recurring inside POLICY 5 cure:** rule-codification-without-self-application (F-SP4-010 — POLICY 15 verbatim discipline not self-applied inside POLICY 5).
- **META-LEVEL-30 route (b) recurring INSIDE the cure BC:** PC9 Dim-7 extractor will silently no-op on actual burst-log files (F-SP4-004) — same class of defect the BC was supposed to cure.

### Convergence Plausibility

Trajectory 14→11→14→**16** NEGATIVE. Cascade NOT converging. Root causes:
1. PO and story-writer fix-bursts introducing new defects faster than closing known ones.
2. Cure-of-cure pattern itself recursing — each cure layer (POLICY 5, POLICY 8) introduces a new sub-route.
3. §SDK Grounding Evidence section captures one-time grep at PO authoring time, but BC body full of narrative claims about external state PRE-DATING and POST-DATING the section.

**Estimated to 3-CLEAN:** 7-10 more passes IF cures are systematic. If pass-5 introduces another cure-of-cure layer, cascade may need D-386 Option C asymptotic-acceptance seal at floor [11-16] HIGH precedent.

### Top 3 findings
1. F-SP4-001 CRITICAL: count==4 equality false-Block on multi-row Phase Progress.
2. F-SP4-004 HIGH: PC9 Dim-7 silent no-op on actual burst-log (META-30 route (b) inside cure).
3. F-SP4-003 HIGH: Architecture Mapping hardcoded F5 paths (F-SP3-001 regression).

### Iron Law compliance
Confirmed: did not read adv-spec-pass-1/2/3.md. Fresh-context only.

---

Pass-4 review COMPLETE. STREAK 0/3 reset. Cascade NOT converging at current trajectory. Orchestrator decision: continue with more systematic cures, OR D-386 Option C asymptotic-acceptance seal, OR reframe scope (split BC into narrower BCs).
