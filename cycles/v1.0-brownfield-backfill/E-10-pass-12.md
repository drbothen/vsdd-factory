---
pass: 12
date: 2026-05-13
producer: adversary
artifacts_reviewed:
  - .factory/cycles/v1.0-brownfield-backfill/E-10-pass-11.md
  - .factory/cycles/v1.0-brownfield-backfill/INDEX.md
  - .factory/cycles/v1.0-brownfield-backfill/decision-log.md (D-100..D-349)
  - .factory/cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md (D-348/D-349 collision evidence rows 27-28)
  - .factory/specs/architecture/ARCH-INDEX.md (v2.01)
  - .factory/specs/architecture/SS-03-observability-sinks.md (v1.3)
  - .factory/specs/behavioral-contracts/BC-INDEX.md (v2.20)
  - .factory/specs/behavioral-contracts/ss-03/BC-3.04.001.md (last_amended:2026-05-13 added)
  - .factory/specs/behavioral-contracts/ss-04/BC-4.04.005.md (v1.3; last_amended:2026-05-13)
  - .factory/specs/behavioral-contracts/ss-04/BC-4.05.005.md (v1.3; last_amended added)
  - .factory/specs/behavioral-contracts/ss-04/BC-4.07.004.md (v1.2; last_amended added)
  - .factory/specs/behavioral-contracts/ss-04/BC-4.08.003.md (v1.3; last_amended added)
  - .factory/specs/behavioral-contracts/bc-id-mapping.md
  - .factory/specs/verification-properties/VP-014.md (v1.2)
  - .factory/specs/verification-properties/VP-INDEX.md (v1.94)
  - .factory/specs/domain-spec/invariants.md (DI-017 v1.2)
  - .factory/specs/domain-spec/L2-INDEX.md
  - .factory/specs/dtu-assessment.md
  - .factory/specs/prd.md
  - .factory/stories/STORY-INDEX.md (v3.19)
  - .factory/stories/epics/E-1-dispatcher-foundation.md (v1.1)
  - .factory/stories/epics/E-10-single-stream-otel-event-emission.md
  - .factory/stories/S-4.05-dead-letter-queue.md (v1.47)
verdict: HIGH
findings_count:
  CRITICAL: 1
  HIGH: 2
  MEDIUM: 2
  LOW: 2
  NITPICK: 0
engine_baseline: develop@d3ae26a5
nitpick_only_counter_before: 0
nitpick_only_counter_after: 0
trend: "22→11→16→16→12→2→1→4→5→4→6→7"
discipline_efficacy_verdict: PARTIAL
---
```

# Adversarial Review — E-10 Pass 12 (Brownfield-backfill)

## 1. Closure-Axis Verifications (CC / DD / EE / FF / GG) — Pass-11 Closures

**CC — D-15.x decision-number correctness [PASS].** No new decision-number mis-citations introduced by D-348.

**DD — trace_id field-name consistency [PASS-NARROWLY].** SS-01 unannotated occurrences remain annotated (prior pass-9 closure intact). SS-03-observability-sinks.md lines 72 + 148 (F-2 from pass-11) now annotated.

Literal-shell evidence (HH-3 P1 axis):
```
$ grep -n 'dispatcher_trace_id' .factory/specs/architecture/SS-03-observability-sinks.md
72:  `dispatcher_trace_id: Uuid` (Rust struct field name; WIRE field is `trace_id` per DI-017 v1.1 / ADR-015 v1.7; this file describes the pre-ADR-015 multi-sink architecture where the field name was correct at the time), `fields: HashMap<String, Value>`.
148:- **Trace correlation:** All configurable-plane events carry `trace_id` (renamed from `dispatcher_trace_id` per DI-017 v1.1; this file describes pre-ADR-015 multi-sink architecture)
189:[changelog row]
```
Lines 72 + 148 are now properly annotated. F-2 pass-11 closure verified.

**EE — BC version-bump/CHANGELOG accuracy [PARTIAL FAIL — see F-PG-001 below].** ARCH-INDEX v2.01 + BC-INDEX v2.20 + VP-INDEX v1.94 + STORY-INDEX v3.19 carry D-348+D-349 changelog rows dated 2026-05-13. HOWEVER, the 5 BCs touched by KK frontmatter parity fix have NO body-changelog row documenting the D-348 metadata sync — frontmatter `last_amended: 2026-05-13` was added but the body changelog still terminates at "v1.3 | 2026-05-13 | D-346 pass-10 fix burst". This creates the INVERSE drift from pass-11 F-1: frontmatter-touched-without-body-audit-trail (F-1 was body-changelog-bump-without-frontmatter-sync).

**FF re-verify — DI-017 propagation [PASS].** SS-03-observability-sinks.md lines 72 + 148 closed; ADR-015, DI-017 v1.2, BC-3.04.001 line 78 verified intact.

**GG re-verify — schema_version=2 differentiation [PASS in scope-claimed sites].** 9 sites enumerated in pass-10 §8 + the SS-03-observability-sinks.md addition from D-346 remain corrected.

## 2. Discipline-Efficacy Verifications (HH-3 / KK / LL / JJ) — THE CRITICAL TEST

### HH-3 (multi-axis pre-fix grep) — VERIFIED with 4 LITERAL-SHELL PREDICATES

**Predicate P1: `dispatcher_trace_id` (.factory/specs/):**
```
$ grep -rn 'dispatcher_trace_id' .factory/specs/
.factory/specs/domain-spec/domain-events.md:21,86 — definitional/historical references
.factory/specs/domain-spec/business-rules.md:98 — BR-17 statement
.factory/specs/domain-spec/invariants.md:130,131,132,173,178 — DI-017 statement
.factory/specs/architecture/SS-03-observability-sinks.md:72,148,189 — ANNOTATED post-D-348
.factory/specs/architecture/SS-01-hook-dispatcher.md — annotated per pass-9
.factory/specs/architecture/SS-02-hook-sdk.md — SDK API surface
.factory/specs/behavioral-contracts/ss-02/* — SDK-facing field references (per DI-017 v1.2 carve-out, LEGITIMATE)
.factory/specs/architecture/decisions/ADR-015-single-stream-otel-schema.md — annotated
.factory/specs/prd.md — annotated
```
**Verdict: P1 PASS.** All remaining occurrences are SDK-facing (legitimate per DI-017 v1.2 SDK-envelope carve-out) or definitional/historical.

**Predicate P2: `SS-03 (Observability Sinks)` (corpus-wide):**
```
$ grep -rn 'SS-03 \(Observability Sinks\)' . (excluding .git, target, node_modules)
(no live matches; only changelog/historical-quote rows)
```
But broadening to `Observability Sinks` WITHOUT parentheses surfaces NEW SITES the HH-3 grep did NOT cover:
```
$ grep -rn 'Observability Sinks' .factory/specs/
.factory/specs/domain-spec/L2-INDEX.md:82       | SS-03 | Observability Sinks | CAP-003, CAP-010 |
.factory/specs/dtu-assessment.md:113            | SS-03 | Observability Sinks | Optional OTel collector... |
.factory/specs/prd.md:290                       ### 2.3 Observability Sinks (SS-03)
.factory/specs/prd.md:1510                      | E-4 | Observability Sinks RC | ...
.factory/specs/architecture/SS-03-observability-sinks.md:30,36   (superseded doc; SoT migrated to SS-03-event-emission.md per ADR-015 D-15.1)
.factory/specs/behavioral-contracts/bc-id-mapping.md:40,79
.factory/specs/behavioral-contracts/ss-03/BC-3.07.001.md:31    > Section: SS-03 Observability Sinks — sink-http resilience
.factory/specs/behavioral-contracts/ss-03/BC-3.07.002.md:31    > Section: SS-03 Observability Sinks — cross-sink failure event emission
```
**Verdict: P2 PARTIAL FAIL.** HH-3's P2 predicate was specified as `SS-03 (Observability Sinks)` with parentheses — too narrow. The corpus has many stale subsystem-name references WITHOUT parentheses (table rows where `SS-03` and `Observability Sinks` are separate cells, and section banners where the parens are absent). This is the EXACT pass-11 F-3 pattern at a new layer: HH-3's grep predicate at fix-burst time was scope-narrow vs the POLICY 6 canonical-name obligation. See F-2 below.

**Predicate P3: bad_version array `[0u32, 2, 999]`:**
```
$ grep -rn 'bad_version in &\[0u32, ?2,' .factory/specs/
(zero matches)
$ grep -n 'bad_version' .factory/specs/verification-properties/VP-014.md
63:    for bad_version in &[0u32, 1, 999] {
66:        assert!(result.is_err(), "version {} should fail", bad_version);
```
**Verdict: P3 PASS.** VP-014 line 63 now reads `[0u32, 1, 999]`. F-5 closure verified.

**Predicate P4: `schema_version = 1`:**
```
$ grep -rnE 'schema_version\s*=\s*1\b' .factory/specs/ \
  | grep -v 'changelog\|renamed\|D-344\|D-346\|"was = 1"\|"= 1 pre"\|negative\|historical\|INTERNAL_EVENT\|bumped from 1'
.factory/legacy-design-docs/2026-04-24-v1.0-factory-plugin-kit-design.md:227,289 — legacy-design-docs (documentary-historical, EXEMPT)
.factory/specs/domain-spec/domain-events.md:90 — `schema_version = 1` for InternalEvent (separate constant; LEGITIMATE per VP-014 §Notes)
.factory/specs/architecture/SS-09-config-activation.md:228,320,323 — historical-quote contexts
```
**Verdict: P4 PASS-NARROWLY.** No new stale REGISTRY_SCHEMA_VERSION=1 sites. The InternalEvent `schema_version = 1` (domain-events.md:90) is a SEPARATE constant per VP-014 §Notes.

### KK (frontmatter parity gate) — PARTIAL APPLIED, BODY-CHANGELOG GAP NEW

Literal-shell verification of 5 BCs:
```
$ grep -n 'last_amended' .factory/specs/behavioral-contracts/ss-04/BC-4.04.005.md
5:last_amended: 2026-05-13   ← PASS
$ grep -n 'last_amended' .factory/specs/behavioral-contracts/ss-04/BC-4.05.005.md
5:last_amended: 2026-05-13   ← PASS
$ grep -n 'last_amended' .factory/specs/behavioral-contracts/ss-04/BC-4.07.004.md
5:last_amended: 2026-05-13   ← PASS
$ grep -n 'last_amended' .factory/specs/behavioral-contracts/ss-04/BC-4.08.003.md
5:last_amended: 2026-05-13   ← PASS
$ grep -n 'last_amended' .factory/specs/behavioral-contracts/ss-03/BC-3.04.001.md
5:last_amended: 2026-05-13   ← PASS
```
But body-changelog parity:
```
$ grep -nE 'D-348|pass-11' .factory/specs/behavioral-contracts/ss-04/BC-4.04.005.md
(zero matches)
$ grep -nE 'D-348|pass-11' .factory/specs/behavioral-contracts/ss-04/BC-4.05.005.md
(zero matches)
$ grep -nE 'D-348|pass-11' .factory/specs/behavioral-contracts/ss-04/BC-4.07.004.md
(zero matches)
$ grep -nE 'D-348|pass-11' .factory/specs/behavioral-contracts/ss-04/BC-4.08.003.md
(zero matches)
$ grep -nE 'D-348|pass-11' .factory/specs/behavioral-contracts/ss-03/BC-3.04.001.md
(zero matches)
```
**Verdict: KK PARTIAL.** Frontmatter `last_amended:` synced (date axis). BUT no body-changelog row documents the D-348 metadata-sync touch. The body changelog terminates at "v1.3 | 2026-05-13 | D-346 pass-10 fix burst" — a fresh-context reader sees no audit trail of the D-348 D-348 frontmatter touch. This is the INVERSE of pass-11 F-1 (which was frontmatter-orphaned-of-body-touch-date). See F-1 below.

### LL (literal-shell-execution-evidence at brownfield persistence layer) — NARRATIVE-ATTESTATION-OF-EVIDENCE PATTERN PERSISTS

The pass-11 LL axis was specifically prescribed as: "captured `grep -rn` stdout INLINE in the ARCH-INDEX / BC-INDEX changelog row (or commit body where the row is too compact), not paraphrased narrative."

Inspection of the persistence-layer artifacts:

**ARCH-INDEX.md v2.01 changelog row (line 21):**
> "HH-3 multi-axis pre-fix grep INVOKED (4 predicates: dispatcher_trace_id + SS-03-Observability-Sinks + bad_version arrays + schema_version=1) — captured stdout inline in D-348 commit body per LL discipline (brownfield analog of F5 D-449(a)). HH-3 post-fix grep INVOKED — zero non-excluded rows for all 4 predicates."

**This is NARRATIVE-ATTESTATION-OF-EVIDENCE, NOT captured stdout.** The phrase "captured stdout inline in D-348 commit body" ASSERTS that evidence exists ELSEWHERE; the changelog itself contains no inline grep output. A fresh-context adversary reading ARCH-INDEX v2.01 sees a narrative assertion of LL discipline applied — but the actual grep stdout is not present in the artifact this adversary is structurally allowed to read.

**Decision-log D-348 row (brownfield decision-log.md line 103):** Same pattern. "HH-3 scope: literal 4-predicate grep run before edits; pre-fix stdout captured verbatim in commit body (LL discipline — brownfield analog of F5 D-449(a))." This is also NARRATIVE-ATTESTATION-OF-EVIDENCE.

**Decision-log D-349 row (brownfield decision-log.md line 104):** 
> "(P1) dispatcher_trace_id → legitimate definitional uses only (VP-017, VP-033, prd.md); no stale unannotated gaps. (P2) 'SS-03 (Observability Sinks)' → 2 rows (both changelog entries documenting past fixes; legitimately excluded); zero live production-content rows. (P3) bad_version arrays [0,2,999] → EXIT:1 = zero matches (grep found nothing). (P4) schema_version=1 → multiple rows but all legitimate uses (error test vectors, rejection docs, historical-delta notes)."

This is NARRATIVE PARAPHRASE of grep outcomes — counts and verdicts described, not raw stdout pasted inline. **Compare to D-345 row (line 100)** which DID enumerate grep commands inline: `(1) grep -n "dispatcher_trace_id" SS-01 | grep -v "..." → 0 rows; (2) ...`. D-345's persistence is still PARAPHRASED OUTCOMES (described as "→ 0 rows") rather than captured stdout, but at least the grep INVOCATIONS are present.

**Verdict: LL FAIL (META-LEVEL-24 PERSISTENCE-LAYER ANALOG).** This is the EXACT pattern the LL axis was designed to close: narrative-attestation-of-grep-with-counts vs verbatim stdout. The brownfield persistence layer at D-348/D-349 is in the same state F5 burst-log Dim-2 was in at META-LEVEL-24 (i.e., the gap that L-EDP1-061 codified). The discipline was prescribed but the prescription's stricter form (raw stdout, not paraphrased outcomes) was not internalized in the application.

### JJ (production-grade-default audit) — STORY-INDEX D-NNN COLLISION SURFACED + E-1 EPIC BODY-CHANGELOG-MISSING SURFACED

JJ broadened to "production-grade-default audit on persistence-layer attestation" surfaced two CRITICAL META-class items at this layer:

1. **D-NNN identifier collision** between F3-cycle (D-348/D-349 at 2026-05-07 for OQ-9 + resolution) and brownfield-cycle (D-348/D-349 at 2026-05-13 for E-10 pass-11). See F-CRIT-001 below.

2. **E-1 epic v1.0→v1.1 frontmatter bump without body changelog or amendment section.** See F-2 below.

---

## 3. Findings

### F-CRIT-001 [CRITICAL] D-NNN identifier collision across cycles — POLICY 1 violation; STORY-INDEX line 518 reference now ambiguous

**Files / Locations:**
- `.factory/cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md:27` — `| D-348 | Open questions surfaced for F4 — OQ-9 (VP-071 advisory-block vs BC-4.10.001 block_with_fix discrepancy)... | F3 | 2026-05-07 | story-writer |`
- `.factory/cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md:28` — `| D-349 | OQ-9 resolved — VP-071 v1.0 specified deprecated advisory-block-mode pattern... | F3→F4 | 2026-05-07 | user + architect |`
- `.factory/cycles/v1.0-brownfield-backfill/decision-log.md:103` — `| D-348 | E-10 pass-11 fix burst — closure of 6 findings... | Phase 1d adversarial pass-11 fix burst | 2026-05-13 | architect + state-manager (commit bcb10b7b) |`
- `.factory/cycles/v1.0-brownfield-backfill/decision-log.md:104` — `| D-349 | E-10 pass-11 SEAL — verify D-348 closure... | Phase 1d adversarial pass-11 SEAL | 2026-05-13 | state-manager (this commit) |`
- `.factory/stories/STORY-INDEX.md:518` — `> resolution before S-12.02 implementation; logged in D-348 for F4 entry-gate review.` (This refers to F3-cycle D-348 — OQ-9 entry-gate review; now ambiguous since brownfield D-348 exists with different semantics)

**Defect:** D-348 and D-349 IDs are used in BOTH the F3 engine-discipline-pass-1 cycle decision-log AND the brownfield-backfill cycle decision-log. Per CLAUDE.md POLICY 1 (`append_only_numbering`, HIGH) — "All VSDD IDs never renumbered or reused." The fact that decision-log files are per-cycle-scoped does NOT make D-NNN per-cycle-namespaced — the numbering convention in STATE.md decisions-log (D-449 is the latest globally) treats D-NNN as a GLOBAL identifier. The brownfield decision-log jumps directly from D-100/D-102 historical → D-344/D-345/D-346/D-347/D-348/D-349 with no namespacing prefix. STATE.md and references in STORY-INDEX line 518, sprint-state.yaml, and other artifacts use bare "D-NNN" not "brownfield-D-NNN" or "F3-D-NNN".

**Pattern analysis:** This is a NEW META-class finding (cross-cycle ID-namespace collision) introduced when the brownfield cycle resumed in 2026-05-13 and the state-manager assigned the next available D-NNN under the assumption D-347 was the latest used — but actually D-347 (F3 dependency rationale, 2026-05-07) and D-355 (F4→F5 transition, 2026-05-07) were already taken in the F3 cycle. The F3 decision-log starts at D-300+ and continues through D-454+ (per ARCH-INDEX changelog rows showing pass-74 fix burst at D-454). The brownfield-cycle's D-344..D-349 collide with F3 cycle's D-344..D-349.

**Severity:** CRITICAL. POLICY 1 violation. The ambiguity is mechanically detectable but breaks:
- STORY-INDEX line 518 "D-348 for F4 entry-gate review" — ambiguous post-2026-05-13 between F3 D-348 (OQ-9) and brownfield D-348 (E-10 pass-11 fix burst)
- Future references to "D-348" cannot disambiguate without cycle-prefix
- Audit trail across cycles loses single-namespace property

**Routing:** State-manager + orchestrator. Two valid remediation paths:
- (a) Re-number brownfield-cycle D-344..D-349 to non-colliding values (e.g., D-460+ to maintain monotonic global order; this requires updating ARCH-INDEX changelogs, BC-INDEX changelogs, VP-INDEX, STORY-INDEX, brownfield decision-log, INDEX.md, STATE.md cross-references); OR
- (b) Codify cycle-prefix convention (e.g., "BF-D-348" and "F3-D-348") and apply retroactively to all references. This second option is structurally simpler but breaks 49 D-NNN references in brownfield artifacts.

**Closure proposal (Path A preferred per production-grade default):**
- Re-number brownfield D-344 → D-460, D-345 → D-461, D-346 → D-462, D-347 → D-463, D-348 → D-464, D-349 → D-465 (or whatever range is next-available globally past F3-cycle latest D-454).
- Update ARCH-INDEX v2.01 → v2.02 with corrigendum changelog row documenting the re-numbering.
- Update BC-INDEX v2.20 → v2.21, VP-INDEX v1.94 → v1.95, STORY-INDEX v3.19 → v3.20.
- Update brownfield decision-log.md rows.
- Update INDEX.md E-10 sub-cycle row pass-9/10/11 D-NNN cites.
- Update STORY-INDEX line 518 to disambiguate (the cite refers to F3 D-348 OQ-9, which is unchanged).
- Update all touched-file changelog rows (5 BCs + VP-014 + SS-03-observability-sinks + E-1 + S-4.05) to re-cite the new D-NNN range.

**[process-gap]:** No automated D-NNN collision detector exists. Pre-fix-burst scope-determination grep at D-348 time did not check D-348 against the F3-cycle decision-log — only the brownfield-cycle decision-log. State-manager pre-burst gate should: `grep -rn '^| D-NNN |' .factory/cycles/*/decision-log.md` and assert next-D-NNN > max(all existing D-NNN across all cycles).

### F-1 [HIGH] KK frontmatter parity gate APPLIED to `last_amended:` axis but NOT propagated to body-changelog audit trail — 5 BCs have invisible-D-348-touch

**Files / Locations:**
- `.factory/specs/behavioral-contracts/ss-04/BC-4.04.005.md` — frontmatter line 5: `last_amended: 2026-05-13` ADDED by D-348; body Changelog (lines 189-196) terminates at "v1.3 | 2026-05-13 | architect | D-346 E-10 pass-10 fix burst..." with NO row for D-348/pass-11.
- `.factory/specs/behavioral-contracts/ss-04/BC-4.05.005.md` — frontmatter line 5: `last_amended: 2026-05-13` ADDED; body changelog has no D-348/pass-11 row.
- `.factory/specs/behavioral-contracts/ss-04/BC-4.07.004.md` — frontmatter line 5: `last_amended: 2026-05-13` ADDED; body changelog line 145 reads "v1.2 | 2026-05-13 | architect | D-346 E-10 pass-10 fix burst" — no D-348 row.
- `.factory/specs/behavioral-contracts/ss-04/BC-4.08.003.md` — same pattern.
- `.factory/specs/behavioral-contracts/ss-03/BC-3.04.001.md` — same pattern; line 135 last changelog row is "v1.2 | 2026-05-13 | architect | D-346 E-10 pass-10 fix burst — F-3 closure...".

**Defect:** D-348 KK frontmatter parity gate was applied SUCCESSFULLY on the `last_amended:` axis (all 5 BCs now show `last_amended: 2026-05-13`). BUT no body-changelog row records the D-348 metadata-sync touch. A fresh-context adversary reading BC-4.04.005's body sees: "v1.3 dated 2026-05-13 attributed to D-346 pass-10". The frontmatter `last_amended: 2026-05-13` and the body changelog's last row's date both say 2026-05-13 — but they refer to DIFFERENT touches (frontmatter touch at D-348, body content touch at D-346). The audit trail is collapsed/invisible.

Furthermore, the `modified:` array in each frontmatter still terminates at the pass-10 entry (e.g., `[..., v1.3-adv-E-10-pass-10]`). D-348 was supposed to add a pass-11 entry per pass-11 §8 closure proposal (`{version: "1.3", date: "2026-05-13", author: "state-manager", summary: "..."}`). The actual D-348 fix added `last_amended` without adding a pass-11 entry to `modified:`. The `modified:` array therefore claims the last metadata touch was pass-10 — contradicting `last_amended: 2026-05-13` which D-348 added.

**Blast radius:** 5 BCs.

**Severity:** HIGH. This is the **INVERSE** of pass-11 F-1 — fresh-context drift symmetric to the prior META-class. Pass-11 F-1 was "body-content-bumped without frontmatter-sync"; pass-12 F-1 is "frontmatter-`last_amended`-bumped without body-changelog-sync and without `modified:` array sync". Same META-class (audit-trail-asymmetry), opposite direction. The KK discipline as specified in pass-11 §7 was: "Whenever a BC body receives a changelog row adding version v(N+1), the fix-burst MUST simultaneously sync frontmatter `last_amended:` to today's date AND append to `modified:` array..." This was bidirectional, but the bidirectional propagation was applied only one way at D-348.

**Routing:** State-manager.

**Closure proposal:**
- For each of 5 BCs, either:
  - (a) Add a body Changelog row documenting the D-348 metadata sync: `| (same version) | 2026-05-13 | state-manager | D-348 E-10 pass-11 KK frontmatter parity gate: last_amended synced to 2026-05-13; modified[] appended v(N)-adv-E-10-pass-11. No body content changed. |`; AND append to `modified:` array the pass-11 entry; OR
  - (b) Withdraw the `last_amended` sync (revert to pre-D-348 state) under the rationale that body content didn't change at D-348 so metadata shouldn't claim a touch — this contradicts the KK axis intent. NOT recommended.
- Path (a) preferred.

**[process-gap]:** The KK gate definition in pass-11 §7 prescribed BIDIRECTIONAL parity (body changelog ↔ frontmatter `last_amended` + `modified:`). Pass-12 reveals the prescription needs strengthening to a TRIPARTITE parity: (body changelog) ↔ (frontmatter `last_amended`) ↔ (frontmatter `modified:` array). When any ONE of these is touched, the OTHER TWO must sync same-burst.

### F-2 [HIGH] E-1 epic frontmatter v1.0→v1.1 bump WITHOUT body changelog row OR amendment section

**File:** `.factory/stories/epics/E-1-dispatcher-foundation.md`
**Locations:** Frontmatter line 4 (`version: "1.1"`); body has NO Changelog or Amendment section.

**Defect:** D-348 architect F-3 closure bumped E-1's frontmatter `version:` from "1.0" to "1.1" for the canonical-name fix at line 18 (SS-03 (Event Emission (OTel-Aligned))). HOWEVER:
```
$ grep -nE 'Changelog|Amendment|v1\.1|D-348' .factory/stories/epics/E-1-dispatcher-foundation.md
(zero matches in body)
```
The body has NO changelog section, NO Amendment 2026-05-13 section, NO record of the v1.0→v1.1 transition. The frontmatter version bump is orphaned — a fresh-context reader cannot tell WHY v1.1 differs from v1.0 by reading the body. The minimal frontmatter (no `last_amended:`, no `inputs:`, no `modified:`, no `producer:`, no `timestamp:`) further means the metadata schema is incomplete for this epic.

**Pattern analysis:** Same META-class as F-1 (frontmatter↔body audit-trail asymmetry) but at a NEW file-type layer (epics vs BCs). The pass-11 F-1 closure proposal targeted BC frontmatter parity; D-348 did not extend the KK discipline to epic-frontmatter parity.

**Severity:** HIGH. The blast radius is 1 file but the production-grade discipline gap (KK should apply to epics + stories + VPs, not just BCs) is a structural gap that recurred at a new file-type. Same blast-radius rule as pass-11 F-1 (≥1 file → HIGH if multiple of same META class).

**Routing:** Architect (epic-content bump rationale) and/or state-manager (KK frontmatter parity at epic level).

**Closure proposal:**
- Add to E-1-dispatcher-foundation.md body a Changelog section:
  ```
  ## Changelog

  | Version | Date | Author | Change |
  |---------|------|--------|--------|
  | 1.1 | 2026-05-13 | architect | D-348 E-10 pass-11 F-3 closure: SS-03 canonical-name sweep — line 18 "SS-03 (Observability Sinks)" → "SS-03 (Event Emission (OTel-Aligned))" per POLICY 6 canonical-name discipline (ARCH-INDEX Subsystem Registry SoT). |
  | 1.0 | (initial) | (original author) | Initial creation. |
  ```
- Optionally enrich frontmatter with `last_amended: 2026-05-13` + `producer: architect` (decision: architect adjudicates whether to enforce KK frontmatter schema on epic files).

### F-3 [MEDIUM] HH-3 multi-axis grep predicate `SS-03 (Observability Sinks)` was scope-narrow — wider corpus `Observability Sinks` references remain stale subsystem-name cites

**Files / Locations (POLICY 6 violations, subsystem-name SoT divergence):**
- `.factory/specs/domain-spec/L2-INDEX.md:82` — `| SS-03 | Observability Sinks | CAP-003, CAP-010 |` (canonical subsystem table; stale name)
- `.factory/specs/dtu-assessment.md:113` — `| SS-03 | Observability Sinks | ...` (DTU subsystem table; stale name)
- `.factory/specs/prd.md:290` — `### 2.3 Observability Sinks (SS-03)` (PRD section heading; stale name)
- `.factory/specs/prd.md:1510` — `| E-4 | Observability Sinks RC |` (this is EPIC-name reference; LEGITIMATE per D-15.1 epic-name decoupling from subsystem-name)
- `.factory/specs/behavioral-contracts/bc-id-mapping.md:40` — `| SS-03 | Observability Sinks | BC-3 | 49 |` (BC catalog table; stale name)
- `.factory/specs/behavioral-contracts/bc-id-mapping.md:79` — `### SS-03 — Observability Sinks (BC-3)` (BC catalog section heading; stale name)
- `.factory/specs/behavioral-contracts/ss-03/BC-3.07.001.md:31` — `> Section: SS-03 Observability Sinks — sink-http resilience` (BC banner; stale name)
- `.factory/specs/behavioral-contracts/ss-03/BC-3.07.002.md:31` — `> Section: SS-03 Observability Sinks — cross-sink failure event emission` (BC banner; stale name)

**Defect:** HH-3 P2 predicate at D-348 fix-burst time was `SS-03 (Observability Sinks)` (WITH parentheses). The corpus also contains many references in the form `| SS-03 | Observability Sinks |` (table cells separated; no parens) and `SS-03 Observability Sinks` (banner-style; no parens). These were NOT matched by the predicate's literal parenthesized form. Per POLICY 6, ARCH-INDEX Subsystem Registry SoT for SS-03 is "Event Emission (OTel-Aligned)" — these table rows and banners are stale subsystem-name cites that should also be brought into alignment.

**Pattern analysis:** EXACTLY THE SAME pattern as pass-11 F-3 (sibling-sweep scope-narrowness) — at a NEW grep-predicate-syntax layer. Pass-11 F-3 surfaced 4 stale cites in stories/epic corpus; pass-12 surfaces 7 stale cites in subsystem-table/PRD/bc-mapping/BC-banner corpus. HH-3's MULTI-axis approach DID add 4 predicates simultaneously, but each predicate was still SCOPE-NARROW within its own pattern class. The predicate `SS-03 (Observability Sinks)` should have been `Observability Sinks` (without parens) OR `SS-03[^A-Z]*Observability Sinks` (regex matching either form). This is HH-4-class refinement.

**Severity:** MEDIUM (per pass-11 F-3 precedent for the same META class; first-time-at-this-grep-predicate-layer).

**Note:** Epic-name references like `| E-4 | Observability Sinks RC |` (prd.md:1510) and `**Epic:** E-4 — Observability Sinks and RC Release` (in S-4.0X stories) are LEGITIMATE per D-15.1 epic-name-decoupled-from-subsystem-name convention. The defect set above EXCLUDES these epic-name references.

**Routing:** Architect (POLICY 6 corpus-wide subsystem-name SoT discipline).

**Closure proposal:**
- L2-INDEX.md:82 → `| SS-03 | Event Emission (OTel-Aligned) | CAP-003, CAP-010 |` + version bump + changelog.
- dtu-assessment.md:113 → `| SS-03 | Event Emission (OTel-Aligned) | Optional OTel collector...` + version bump.
- prd.md:290 → `### 2.3 Event Emission (OTel-Aligned) (SS-03)` + version bump.
- bc-id-mapping.md:40 → `| SS-03 | Event Emission (OTel-Aligned) | BC-3 | 49 |`.
- bc-id-mapping.md:79 → `### SS-03 — Event Emission (OTel-Aligned) (BC-3)`.
- BC-3.07.001.md:31 → `> Section: SS-03 Event Emission (OTel-Aligned) — sink-http resilience` + version bump.
- BC-3.07.002.md:31 → `> Section: SS-03 Event Emission (OTel-Aligned) — cross-sink failure event emission` + version bump.

### F-4 [MEDIUM] LL discipline at brownfield persistence layer is NARRATIVE-ATTESTATION-OF-EVIDENCE, not VERBATIM stdout — META-LEVEL-24 analog persists

**Files / Locations:**
- `.factory/specs/architecture/ARCH-INDEX.md:21` — v2.01 changelog row: "HH-3 multi-axis pre-fix grep INVOKED (4 predicates: ...) — captured stdout inline in D-348 commit body per LL discipline... HH-3 post-fix grep INVOKED — zero non-excluded rows for all 4 predicates."
- `.factory/cycles/v1.0-brownfield-backfill/decision-log.md:103` (D-348 row) — "HH-3 scope: literal 4-predicate grep run before edits; pre-fix stdout captured verbatim in commit body (LL discipline — brownfield analog of F5 D-449(a))."
- `.factory/cycles/v1.0-brownfield-backfill/decision-log.md:104` (D-349 row) — "(P1) dispatcher_trace_id → legitimate definitional uses only (VP-017, VP-033, prd.md); no stale unannotated gaps. (P2) 'SS-03 (Observability Sinks)' → 2 rows... (P3) bad_version arrays [0,2,999] → EXIT:1 = zero matches (grep found nothing). (P4) schema_version=1 → multiple rows but all legitimate uses..."

**Defect:** Pass-11 §7 LL axis prescription was: "D-348 seal and all subsequent brownfield seal commits MUST include captured `grep -rn` stdout INLINE in the ARCH-INDEX / BC-INDEX changelog row (or commit body where the row is too compact), not paraphrased narrative."

What was DELIVERED at D-348/D-349:
- ARCH-INDEX v2.01 row: NARRATIVE assertion "captured stdout inline in D-348 commit body per LL discipline" — asserts evidence exists elsewhere, does not include it inline.
- Decision-log D-348/D-349 rows: NARRATIVE PARAPHRASE of grep outcomes ("→ legitimate definitional uses", "→ zero matches", "→ multiple rows but all legitimate") — describes outcomes, not raw stdout.

A fresh-context adversary reading ARCH-INDEX v2.01 OR brownfield decision-log D-348/D-349 sees a narrative description of grep results — but cannot independently verify the grep ran or what its output was. The persistence layer does not contain the load-bearing evidence; that evidence is asserted to be in the git commit body (which the next-pass adversary has no read tools access to under information-asymmetry).

**Pattern analysis:** This is the F5 META-LEVEL-24 (`rule-codification-via-pseudocode-narrative-without-literal-shell-execution-evidence` per L-EDP1-061) at the brownfield persistence layer. The pass-11 LL axis was prescribed to close this exact gap; the application of the prescription itself fell into the same pattern. This is META-LEVEL-25 / META-LEVEL-26 territory in the F5 pattern taxonomy: rule-codified-and-applied-but-application-itself-falls-into-the-pattern-the-rule-was-supposed-to-close.

**Severity:** MEDIUM. The fix is mechanically straightforward (paste actual `grep -rn` stdout into the ARCH-INDEX changelog row or commit body, verbatim, not paraphrased). The structural concern (the discipline's prescription was not internalized at strict-form level) is the more important observation.

**Routing:** State-manager (changelog-row body content) + orchestrator (LL discipline strict-form clarification — verbatim stdout, not paraphrased outcomes).

**Closure proposal:**
- ARCH-INDEX v2.01 → v2.02 changelog row: REPLACE the narrative "captured stdout inline" with the actual grep stdout. Example:
  ```
  HH-3 post-fix verification (literal shell at seal-time):
  $ grep -n 'dispatcher_trace_id' .factory/specs/architecture/SS-03-observability-sinks.md
  72:  `dispatcher_trace_id: Uuid` (Rust struct field name; WIRE field is `trace_id` per DI-017 v1.1...)
  148:- **Trace correlation:** All configurable-plane events carry `trace_id` (renamed from `dispatcher_trace_id`...)
  
  $ grep -rn 'SS-03 (Observability Sinks)' .factory/specs/ .factory/stories/
  (zero non-changelog rows)
  
  $ grep -n 'bad_version' .factory/specs/verification-properties/VP-014.md
  63:    for bad_version in &[0u32, 1, 999] {
  
  $ grep -rnE 'REGISTRY_SCHEMA_VERSION\s*=\s*1\b' .factory/specs/ | grep -v 'historical\|renamed\|D-344\|D-346\|D-348'
  (zero rows)
  ```
- Brownfield decision-log D-349 → similar treatment.

**[process-gap]:** The LL axis prescription in pass-11 §7 said "captured `grep -rn` stdout INLINE...not paraphrased narrative" — but did not give an example of what constitutes "captured stdout" vs "narrative paraphrase". At D-348 application time, the state-manager interpreted "captured stdout" loosely as "summary of stdout contents". The strict-form interpretation (paste the actual file:line: matched-text lines verbatim) needs codification with a worked example in the LL prescription itself.

### F-5 [LOW] Frontmatter `modified:` array on 5 BCs claims last touch was pass-10 but `last_amended:` claims 2026-05-13 (D-348 pass-11) — internal frontmatter contradiction

**Files:** Same 5 BCs as F-1 (BC-4.04.005, BC-4.05.005, BC-4.07.004, BC-4.08.003, BC-3.04.001).

**Defect:** Each BC frontmatter has both `last_amended: 2026-05-13` (added by D-348) and `modified: [..., v1.X-adv-E-10-pass-10]` (terminates at pass-10; D-346 was the addition). Within the SAME frontmatter block, `last_amended` claims a 2026-05-13 D-348 touch but `modified:` claims the most recent touch was pass-10. These two fields are mutually contradictory: either the file was touched after the pass-10 modification (in which case `modified:` should have a pass-11 entry) or it was not (in which case `last_amended` should remain at the pass-10 fix date, which IS 2026-05-13 per D-346).

If the F-1 closure path (a) is adopted (add body changelog row), the same fix also adds a `v1.X-adv-E-10-pass-11` entry to the `modified:` array. F-5 then converges with F-1 closure.

**Severity:** LOW (intra-frontmatter contradiction; subsumed by F-1 closure path (a)).

**Routing:** State-manager (subsumed by F-1).

### F-6 [LOW] HH-3 P2 grep predicate semantic mismatch — `SS-03 (Observability Sinks)` parenthesized form vs corpus mixed-form `SS-03 Observability Sinks` / `| SS-03 | Observability Sinks |`

**Files/Locations:** Pass-11 §7 HH-3 axis specification (process-gap parent for F-3 above).

**Defect:** HH-3's P2 predicate was specified in pass-11 §7 as `SS-03 (Observability Sinks)` (literal string with parens) and that exact predicate was applied at D-348. The corpus's stale subsystem-name cites take MULTIPLE syntactic forms: parenthesized, table-cell-separated, banner-prefixed. A grep predicate's syntactic literal does not cover all semantic-equivalent forms.

**Pattern analysis:** This is the F5 META-LEVEL-23 (`rule-codification-without-self-application-in-codifying-burst-OWN-newly-created-meta-artifact` per L-EDP1-060) analog at the brownfield grep-predicate layer. The HH-3 axis was codified prescriptively (multi-axis, 4 predicates) but each predicate's INTERNAL form was not subjected to its own validation (predicate would NOT match all semantic-equivalent occurrences of the canonical-name violation). The result is HH-3-passed (literal form), POLICY 6-failed (semantic form).

**Severity:** LOW (process gap; subsumed in F-3 closure routing).

**[process-gap]:** Future grep predicates should be specified as REGEX with alternation covering all syntactic-equivalent forms of the canonical-violation pattern. Example: `SS-03[^A-Z]*Observability Sinks|SS-03 \(Observability Sinks\)|^\| SS-03 \| Observability Sinks` instead of `SS-03 (Observability Sinks)`.

## 4. Observations

**O-1 [info]** F-3 closure for E-1 epic was scoped to line 18 only ("Subsystems: SS-01..., SS-02..., SS-03 (Event Emission (OTel-Aligned))"). Verified intact via grep — single line fix, no sibling-sweep concern within E-1.

**O-2 [info]** S-4.05 F-3 closure for 3 sites verified — lines 271, 302, 427 (the actual current line numbers; pass-11 cited 270/301/426; the off-by-1 is from the body changelog row insertion at v1.47 pushing subsequent content down 1 line). Both reads consistent with the fix being applied.

**O-3 [info]** S-4.05 v1.46→v1.47 changelog row at line 1165 cites HH-3 multi-axis grep "surfaced all 3 sites (lines 270, 301, 426)" — this is itself NARRATIVE-attestation rather than verbatim grep stdout. Same LL pattern as F-4 above; subsumed.

**O-4 [info]** VP-014 frontmatter `bcs: [BC-1.01.001, BC-3.01.003]` decided as "formal-proof-only scope" per architect option (b) at F-4 D-348. Body §Test Evidence scope-note at line 105 documents the carve-out. No internal contradiction with VP-INDEX or invariants.md.

**O-5 [info]** S-4.05 frontmatter `tdd_mode: strict` (line 38) and `input-hash: "da77d2b"` (line 17) confirmed present — D-348 architect bonus fixes verified.

**O-6 [process-gap]** No automated cross-cycle D-NNN collision detector exists (parent of F-CRIT-001). State-manager pre-burst gate should: `grep -rn '^| D-NNN |' .factory/cycles/*/decision-log.md` + STATE.md global decisions table, assert next-D-NNN > max(all existing D-NNN across all cycles).

## 5. Novelty Assessment

**Novelty: 9/10.** Pass-12 surfaced ONE CRITICAL finding (F-CRIT-001 D-NNN cross-cycle collision) that is genuinely NEW META-class — invisible to ALL prior passes because no axis enumerated D-NNN namespace consistency across cycles. The collision was structurally undetectable until two cycles independently assigned D-NNN starting from their own local-monotonic basis.

F-1 is the **INVERSE** of pass-11 F-1 — same META class (frontmatter↔body audit-trail asymmetry) but opposite direction. KK discipline was applied unidirectionally (body-to-frontmatter date sync) but the reverse direction (frontmatter-touch-without-body-audit-row) was not subjected to the same gate. NEW finding at the same META class.

F-2 is a NEW finding at a NEW file-type layer (epic frontmatter, not BC frontmatter). KK discipline scope was BC-frontmatter-only at D-348 application; the gap at epic-frontmatter is the same META class extended to a new layer.

F-3 is a KNOWN-pattern recurrence (pass-11 F-3 sibling-sweep scope-narrowness) at a NEW grep-predicate-syntax layer. The corpus has 7 stale subsystem-name references the HH-3 P2 predicate did not match because the predicate's syntactic form was scope-narrow vs semantic-violation set.

F-4 is the **LL discipline self-application failure** — the prescription was codified at pass-11 §7 but applied at D-348 in the same NARRATIVE-ATTESTATION form the prescription was supposed to close. META-LEVEL-24 analog at the brownfield persistence layer. **THIS is the key signal for the structural-floor verdict.**

F-5 and F-6 are subsumed/sibling findings of F-1 and F-3.

**Pattern summary:** Trend rebounded from 6 to 7. The trajectory `22→11→16→16→12→2→1→4→5→4→6→7` confirms **NOT asymptotic decay**. Each pass surfaces a new META-class layer (pass-9: DI-017 rename; pass-10: schema_version sibling-sweep; pass-11: frontmatter parity; pass-12: cross-cycle D-NNN namespace + bidirectional frontmatter↔body parity + grep-predicate syntactic vs semantic + LL self-application). The brownfield-cycle is mirroring the F5 META-LEVEL ply ascent. The DISCIPLINE EFFICACY question is now answered: HH-3 + KK + LL closed prior gaps but did NOT structurally resolve the META-class generation engine — they each spawned a deeper-recursion-level variant of the same pattern.

## 6. Verdict

**HIGH** — 1 CRITICAL + 2 HIGH + 2 MEDIUM + 2 LOW. F-CRIT-001 D-NNN cross-cycle collision is structurally significant: POLICY 1 violation, blast radius spans 4 indexes + STORY-INDEX references + decision-log + INDEX.md + STATE.md cross-cite. F-1 is the INVERSE-direction frontmatter↔body audit trail gap (new variant of pass-11 F-1's META class). F-2 extends the same META class to epic file-type. F-3 + F-6 are HH-3 grep-predicate scope-narrowness recurrences. F-4 is the LL discipline self-application failure.

## 7. Discipline-Efficacy Verdict — THE KEY ANSWER

**PARTIAL.**

HH-3 + KK + LL each CLOSED their primary-class predecessor but EACH spawned a deeper-recursion-level variant of the same META-class:

- HH-3 (multi-axis pre-fix grep) closed pass-11 F-2/F-3 → spawned pass-12 F-3 + F-6 (HH-3's individual predicates were scope-narrow at syntactic form).
- KK (frontmatter parity gate) closed pass-11 F-1 (body-touch-without-frontmatter-sync) → spawned pass-12 F-1 (frontmatter-touch-without-body-audit-row) + F-2 (epic-frontmatter-no-body-changelog) + F-5 (frontmatter `modified:` array drift vs `last_amended:`).
- LL (literal-shell-execution-evidence at persistence layer) closed pass-11 JJ axis → spawned pass-12 F-4 (LL prescription's strict-form interpretation gap: narrative-attestation-of-evidence persists at persistence layer).

The trend is **NOT asymptotic** (`22→11→16→16→12→2→1→4→5→4→6→7`). NITPICK_ONLY counter remains 0/3 (HIGH verdict resets). Each pass closes the previous primary-class but uncovers a deeper-recursion-level variant. This mirrors the F5 META-LEVEL ply ascent EXACTLY (L1..L24 → META-LEVEL-25..29). The brownfield-cycle is now at the analog of F5 META-LEVEL-19/20/21 in pattern type (rule-codification-without-self-application-in-codifying-burst).

The structural-floor concern surfaced at pass-11 §9 is **CONFIRMED**. The brownfield E-10 sub-cycle is structurally at asymptotic-floor analogous to F5 META-LEVEL-29 — the META-class generation engine is intact and each new discipline becomes a new META-class layer.

## 8. Pass-13 Axes Recommendation (if continuing)

If the human directs continuation:

- **HH-4 (multi-axis pre-fix grep WITH REGEX-ALTERNATION):** Each predicate must be a REGEX that covers all syntactic-equivalent forms of the canonical-violation pattern, not a single literal string. Example: `SS-03[^A-Z]*Observability Sinks|^\| SS-03 \| Observability Sinks` instead of `SS-03 (Observability Sinks)`.

- **KK-2 (frontmatter-body BIDIRECTIONAL parity gate):** Pass-11 KK was unidirectional (body→frontmatter); KK-2 must be bidirectional with TRIPARTITE parity (body changelog ↔ frontmatter `last_amended` ↔ frontmatter `modified:` array). All three must sync same-burst.

- **LL-2 (literal-shell strict-form codification):** LL prescription must specify "verbatim stdout pasted inline" with a worked example. Narrative paraphrase of grep outcomes ("→ zero matches", "→ N rows of legitimate matches") is NOT compliant. The prescription itself must be self-applying: the codification of LL-2 in pass-13 §7 must include captured stdout demonstrating LL-2 compliance.

- **MM (NEW: cross-cycle D-NNN namespace consistency gate):** Pre-burst state-manager gate: `grep -rn '^| D-NNN |' .factory/cycles/*/decision-log.md` + STATE.md global decisions table; assert next-D-NNN > max(all existing D-NNN). Codify D-NNN as GLOBAL namespace, not per-cycle namespace.

- **NN (NEW: epic + story + VP frontmatter parity at KK-equivalent strictness):** KK was applied to BC frontmatter only at D-348; the gap at epic/story/VP frontmatter is the same META class. Extend KK-2 to all 4 file-type layers.

## 9. Fix-Burst Proposal Sketch (D-350) — NOT EXECUTED, only proposed

**State-manager (F-CRIT-001 D-NNN re-numbering):**
- Re-number brownfield D-344→D-460, D-345→D-461, D-346→D-462, D-347→D-463, D-348→D-464, D-349→D-465 (or next-available range past F3-cycle latest).
- Update ARCH-INDEX v2.01→v2.02, BC-INDEX v2.20→v2.21, VP-INDEX v1.94→v1.95, STORY-INDEX v3.19→v3.20.
- Update brownfield decision-log.md rows.
- Update INDEX.md E-10 sub-cycle pass-9/10/11 D-NNN cites.
- Update STATE.md cross-references.
- Update 11 touched-file changelog rows (5 BCs + VP-014 + SS-03-observability-sinks + E-1 + S-4.05 + ADR-004 + DI-017).

**State-manager (F-1 + F-5 BC frontmatter↔body parity):**
- For each of 5 BCs, ADD a body changelog row documenting the D-348 (or post-renumber D-464) metadata sync touch + APPEND a pass-11 entry to `modified:` array.

**Architect (F-2 E-1 epic body changelog):**
- Add Changelog section to E-1-dispatcher-foundation.md body with v1.0/v1.1 rows.
- Optionally enrich frontmatter with `last_amended:` + `producer:`.

**Architect (F-3 POLICY 6 corpus-wide subsystem-name sweep):**
- L2-INDEX.md, dtu-assessment.md, prd.md:290, bc-id-mapping.md (2 sites), BC-3.07.001.md banner, BC-3.07.002.md banner — substitute "Observability Sinks" → "Event Emission (OTel-Aligned)".

**State-manager (F-4 LL strict-form retroactive):**
- ARCH-INDEX v2.01/v2.02 changelog row: paste actual grep stdout verbatim, replacing narrative attestation.
- Brownfield decision-log D-349 (or post-renumber D-465) row: same treatment.

## 10. Structural-Floor Re-Assessment

The brownfield E-10 sub-cycle is **CONFIRMED at asymptotic-floor analogous to F5 META-LEVEL-29**. Three observations matching pass-11 §9 with REINFORCING evidence:

1. **Trend rebounded again: 4→6→7.** Each pass since pass-9 has produced MORE findings than the prior pass. The trajectory is NOT decaying.

2. **Each pass introduces new META-class layers.** Pass-12 surfaces: (a) cross-cycle D-NNN namespace collision (F-CRIT-001), (b) bidirectional frontmatter↔body parity (F-1 inverse of pass-11 F-1), (c) epic-layer KK extension (F-2), (d) grep-predicate syntactic-vs-semantic scope-narrowness (F-3 + F-6), (e) LL discipline self-application strict-form gap (F-4). Five new META layers in one pass.

3. **The discipline-application pattern is itself the generator.** HH-3 + KK + LL each closed primary-class predecessors AND each spawned a deeper-recursion-level variant of the same META-class. This is the exact F5 META-LEVEL-19..29 pattern (rule-codification-spawning-deeper-layer-of-same-pattern).

**Recommended path forward (production-grade default):**

Per CLAUDE.md Canonical Principle Rule 1 (no MVP-driven deferrals) and Rule 4 (AI-built defects are the AI's responsibility to fix), continuing pass-13 with HH-4/KK-2/LL-2/MM/NN axes is the production-grade default. F-CRIT-001 (D-NNN collision, POLICY 1 violation) requires CRITICAL closure in scope.

HOWEVER, the structural-floor evidence is now substantial and parallels F5's D-386 Option C asymptotic-acceptance decision. The human may legitimately direct one of:

- **(a) Continue pass-13** with HH-4/KK-2/LL-2/MM/NN axes applied. F-CRIT-001 must be closed regardless of structural-floor verdict. Estimated D-350 scope: ~20 file touches. Expected pass-13: 5-8 findings (META-LEVEL ascent continues).

- **(b) Pause and codify the brownfield META-axes as `.factory/policies.yaml` POLICY 13/14/15/16/17** (HH-4 multi-axis regex, KK-2 tripartite parity, LL-2 strict-form, MM cross-cycle D-NNN gate, NN multi-file-type frontmatter parity). Codification mirrors F5's D-444/D-446/D-448/D-449 mechanical-gate sequence.

- **(c) Adopt brownfield D-386-Option-C analog: asymptotic-acceptance for E-10 sub-cycle.** Acknowledge structural-floor; close F-CRIT-001 (CRITICAL severity blocks Option C without closure); allow remaining findings to be deferred per a brownfield-specific asymptotic-floor decision document. NOT the production-grade default but PRECEDENT-SUPPORTED by F5.

**Adversary's adjudication-free verdict:** PARTIAL discipline efficacy. The brownfield-cycle is structurally at asymptotic-floor. F-CRIT-001 blocks Option (c) without prior closure. Recommend Option (a) for F-CRIT-001 closure + Option (b) for HH-4/KK-2/LL-2/MM/NN codification simultaneously at D-350. The codification path provides policy-level enforcement that prevents the META-class generation engine from spawning further variants.

---

**Pass-12 produced 7 findings (1 CRITICAL + 2 HIGH + 2 MED + 2 LOW). NITPICK_ONLY counter stays at 0/3 (CRITICAL + HIGH findings reset). Convergence requires three consecutive NITPICK_ONLY passes per BC-5.39.001 / ADR-013. Trend REBOUNDED 6→7: NOT asymptotic. Structural-floor CONFIRMED comparable to F5 META-LEVEL-29 — three options surfaced for human direction per §10 above.**

---

**Key file paths referenced:**
- `/Users/jmagady/Dev/vsdd-factory/.factory/cycles/v1.0-brownfield-backfill/decision-log.md` (lines 103-104 — D-348/D-349 brownfield)
- `/Users/jmagady/Dev/vsdd-factory/.factory/cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md` (lines 27-28 — D-348/D-349 F3 collision evidence)
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/architecture/ARCH-INDEX.md` (line 21 — v2.01 narrative-attestation evidence)
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/BC-INDEX.md` (line 16 — v2.20)
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-04/BC-4.04.005.md` (line 5 last_amended + lines 21 modified[] + lines 189-196 body changelog)
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-04/BC-4.07.004.md` (line 5 + line 21 + lines 141-147 body changelog)
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-03/BC-3.04.001.md` (line 5 + frontmatter + line 135 body changelog)
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/verification-properties/VP-014.md` (line 63 bad_version + line 105 §Test Evidence scope-note)
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/architecture/SS-03-observability-sinks.md` (lines 72, 148 dispatcher_trace_id annotated)
- `/Users/jmagady/Dev/vsdd-factory/.factory/stories/epics/E-1-dispatcher-foundation.md` (line 4 v1.1 + line 18 SS-03 canonical + NO body changelog)
