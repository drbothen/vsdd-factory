---
pass: 11
date: 2026-05-13
producer: adversary
artifacts_reviewed:
  - .factory/cycles/v1.0-brownfield-backfill/E-10-pass-10.md
  - .factory/specs/architecture/decisions/ADR-004-toml-config.md (v1.3)
  - .factory/specs/architecture/decisions/ADR-011-dual-hook-routing-tables.md
  - .factory/specs/architecture/decisions/ADR-015-single-stream-otel-schema.md (v1.10)
  - .factory/specs/architecture/SS-01-hook-dispatcher.md (v1.3)
  - .factory/specs/architecture/SS-02-hook-sdk.md (v1.2)
  - .factory/specs/architecture/SS-03-observability-sinks.md (v1.2 superseded)
  - .factory/specs/architecture/SS-03-event-emission.md (v1.0; consulted for canonicalness)
  - .factory/specs/architecture/SS-09-config-activation.md
  - .factory/specs/architecture/ARCH-INDEX.md (v2.00)
  - .factory/specs/behavioral-contracts/BC-INDEX.md (v2.19)
  - .factory/specs/behavioral-contracts/ss-03/BC-3.04.001.md (v1.2)
  - .factory/specs/behavioral-contracts/ss-04/BC-4.04.005.md (v1.3)
  - .factory/specs/behavioral-contracts/ss-04/BC-4.05.005.md (v1.3)
  - .factory/specs/behavioral-contracts/ss-04/BC-4.07.004.md (v1.2)
  - .factory/specs/behavioral-contracts/ss-04/BC-4.08.003.md (v1.3)
  - .factory/specs/verification-properties/VP-014.md (1.1)
  - .factory/specs/domain-spec/invariants.md (1.11; DI-017 v1.2)
  - .factory/specs/domain-spec/business-rules.md (BR-14)
  - .factory/specs/domain-spec/domain-events.md (line 90 schema note)
  - .factory/specs/prd.md (lines 1112, 1164, 1384)
  - .factory/stories/epics/E-10-single-stream-otel-event-emission.md
  - crates/factory-dispatcher/src/registry.rs (REGISTRY_SCHEMA_VERSION=2 verification only)
verdict: HIGH
findings_count:
  CRITICAL: 0
  HIGH: 1
  MEDIUM: 2
  LOW: 2
  NITPICK: 0
fix_burst: D-464 (renumbered from D-348 per F-CRIT-001 resolution 2026-05-13)
seal_dispatch: D-465 (renumbered from D-349 per F-CRIT-001 resolution 2026-05-13)
engine_baseline: develop@d3ae26a5
nitpick_only_counter_before: 0
nitpick_only_counter_after: 0
trend: "22→11→16→16→12→2→1→4→5→4→6"
---

# Adversarial Review — Pass 11 (E-10 spec package)

## 1. Closure-Axis Verifications (CC / DD / EE / FF / GG)

**CC — D-15.x decision-number correctness [PASS].** No new mis-citations introduced by D-346.

**DD — trace_id field-name consistency [PASS].** SS-01 unannotated occurrences remain annotated or canonicalized per D-344. No regressions introduced by D-346.

Literal-shell evidence:
```
$ grep -n 'dispatcher_trace_id' .factory/specs/architecture/SS-01-hook-dispatcher.md \
   | grep -v 'renamed from\|D-344\|DI-017 v1.1'
(zero rows)
```

Lines 38, 59, 91, 132 retain their `(renamed from dispatcher_trace_id per DI-017 v1.1 / ADR-015 v1.7)` annotations. F-1 SS-01 closure from pass-9 remains intact.

**EE — BC version-bump/CHANGELOG accuracy [PASS].** ARCH-INDEX v2.00, BC-INDEX v2.19 carry D-346+D-347 changelog entries dated 2026-05-13. BC-4.04.005 v1.2→v1.3, BC-4.05.005 v1.2→v1.3, BC-4.08.003 v1.2→v1.3 version bumps confirmed. BC-4.07.004 v1.1→v1.2 confirmed.

**FF re-verify — DI-017 propagation [PARTIAL].** F-3 BC-3.04.001 line 78 was fixed per D-346 (SS-03 (Observability Sinks) → SS-03 (Event Emission (OTel-Aligned))). ARCH-INDEX v2.00, SS-01 verified as corrected. **HOWEVER:** SS-03-observability-sinks.md lines 72 and 148 were amended by D-346 for `schema_version` only (v1.1→v1.2) but still retain `dispatcher_trace_id` in their prose — see F-2 below. FF is PARTIAL on the dispatcher_trace_id axis for SS-03-observability-sinks.md.

**GG re-verify — schema_version=2 differentiation [PASS in scope-claimed sites].** ADR-004 line 116 now cites `REGISTRY_SCHEMA_VERSION: u32 = 2 (post-ADR-019 F2 2026-05-07)`. VP-014.md lines 45, 107 updated. business-rules.md line 85 updated. prd.md line 1164 updated. BC-4.04.005 line 39, BC-4.05.005 line 38, BC-4.07.004 line 48, BC-4.08.003 line 38 all updated. GG verified PASS for the 9 sites enumerated in pass-10 §8 proposal sketch.

## 2. Refinement-Axis Verifications (HH-2 / II-2 / JJ)

**HH-2 — Pre-fix scope-determination multi-axis grep [PARTIAL].** The D-346 fix burst correctly ran a pre-fix grep for `schema_version` patterns and enumerated all 9 sites (expanding beyond pass-10's listed 8). However, the pre-fix grep was **schema-version-content-only** — it did not include the `dispatcher_trace_id` sibling-pattern that was simultaneously in scope (DI-017 axis). The fix-burst's scope-determination grep treated schema-version and dispatcher_trace_id as independent patterns despite both being live fix axes at D-346 time. Per the production-grade default (CLAUDE.md Canonical Rule 4), the pre-fix scope-determination grep should have been MULTI-AXIS at fix-burst time: `grep -rn 'schema_version.*=.*1\|REGISTRY_SCHEMA_VERSION.*1\|dispatcher_trace_id' .factory/specs/` simultaneously. The scope-narrowness allowed SS-03-observability-sinks.md dispatcher_trace_id occurrences to survive.

**II-2 — Cross-doc sibling-sweep post-fix [PARTIAL].** D-346 successfully swept BC-4.x Preconditions (4 BCs), VP-014, business-rules.md, prd.md, and BC-3.04.001 for the schema_version pattern. However, the story/epic corpus was not included in the sibling-sweep scope. Specifically: `stories/epics/E-10-single-stream-otel-event-emission.md` line 18, `stories/S-4.05-dead-letter-queue.md` lines 270, 301, 426 retain stale `SS-03 (Observability Sinks)` cites. The pass-10 II axis was described as "cross-doc sibling-sweep" but the fix-burst interpreted this as `.factory/specs/` only. POLICY 6 (canonical-name SoT) applies to stories corpus as well.

**JJ — Production-grade-default audit on D-347 seal attestation [PARTIAL FAIL].** ARCH-INDEX v2.00 changelog row for D-346+D-347 contains a narrative summary of the closure verification rather than inline literal-shell stdout per F5 D-449(a) brownfield-layer application. Specifically, the ARCH-INDEX v2.00 changelog entry reads "D-346 fix burst closed F-1/F-2/F-3/F-4 with HH-2 pre-fix grep scope-expansion (3 additional sites beyond pass-10 §8)" — this is a narrative paraphrase, not captured grep output. Per D-449(a), the equivalent at the brownfield persistence layer is that D-348/D-349 seal commits should contain inline literal-shell stdout for each closure assertion. The ARCH-INDEX changelog is the persistence-layer analog of a burst-log Dim-2 attestation block; prose paraphrase does not satisfy the mechanical-gate standard.

## 3. Findings

### F-1 [HIGH] Frontmatter `last_amended` + `modified:` array drift — 5 BCs lack 2026-05-13 metadata sync after D-346 body content changes

**Files / Locations:**
- `.factory/specs/behavioral-contracts/ss-04/BC-4.04.005.md` — frontmatter line 5: `last_amended: 2026-05-08` (stale); `modified:` array missing v1.3 entry
- `.factory/specs/behavioral-contracts/ss-04/BC-4.05.005.md` — frontmatter: NO `last_amended:` field present; `modified:` array missing v1.3 entry
- `.factory/specs/behavioral-contracts/ss-04/BC-4.07.004.md` — frontmatter: NO `last_amended:` field present; `modified:` array missing v1.2 entry
- `.factory/specs/behavioral-contracts/ss-04/BC-4.08.003.md` — frontmatter: NO `last_amended:` field present; `modified:` array missing v1.3 entry
- `.factory/specs/behavioral-contracts/ss-03/BC-3.04.001.md` — frontmatter: NO `last_amended:` field present; `modified:` array missing v1.2 entry (D-346 added line 78 canonical-name fix + changelog row to body, but did not update frontmatter)

**Defect:** D-346 amended body content (Preconditions rows, changelog entries) in all 5 BCs. The version numbers were bumped correctly in the body (H1 title line + changelog table). However, frontmatter `last_amended:` and `modified:` array were not updated to reflect the 2026-05-13 fix. BC-4.04.005 retains a stale `last_amended: 2026-05-08` from an earlier session; BC-4.05.005/4.07.004/4.08.003/BC-3.04.001 have NO `last_amended:` field — it was never added when the schema was introduced.

**Blast radius:** 5 BCs. This is a NEW META-class finding: **primary-content-fix-without-metadata-propagation**. Prior passes identified sibling-sweep scope-narrowness at the body-content layer (schema_version values, subsystem names). This pass identifies scope-narrowness at the **metadata layer** — the fix-burst's scope-determination grep targeted body content patterns only and did not include frontmatter parity checks.

**Severity:** HIGH. Frontmatter drift is detectable by the adversary in a fresh-context scan (no implementation required to verify the gap). It creates audit-trail inconsistency: the changelog in the body says v1.3 amended 2026-05-13, but frontmatter says `last_amended: 2026-05-08` (or absent) — a direct contradiction. For BCs where `last_amended:` is absent entirely, the metadata schema is incomplete for 4 of 5 affected files.

**Routing:** State-manager (frontmatter parity is a bookkeeping discipline; state-manager is responsible for `last_amended:` + `modified:` array sync per STATE.md Canonical Principle).

**Closure proposal:**
- BC-4.04.005 frontmatter: update `last_amended: 2026-05-13`; append to `modified:` array: `{version: "1.3", date: "2026-05-13", author: "state-manager", summary: "Precondition 2 schema_version 1→2 post-ADR-019 F2; D-346 cite-refresh"}`.
- BC-4.05.005 frontmatter: add `last_amended: 2026-05-13`; append to `modified:` array: `{version: "1.3", date: "2026-05-13", author: "state-manager", summary: "Precondition 2 schema_version 1→2 post-ADR-019 F2; D-346 cite-refresh"}`.
- BC-4.07.004 frontmatter: add `last_amended: 2026-05-13`; append to `modified:` array: `{version: "1.2", date: "2026-05-13", author: "state-manager", summary: "Precondition 2 schema_version 1→2 post-ADR-019 F2; D-346 cite-refresh"}`.
- BC-4.08.003 frontmatter: add `last_amended: 2026-05-13`; append to `modified:` array: `{version: "1.3", date: "2026-05-13", author: "state-manager", summary: "Precondition 2 schema_version 1→2 post-ADR-019 F2; D-346 cite-refresh"}`.
- BC-3.04.001 frontmatter: add `last_amended: 2026-05-13`; append to `modified:` array: `{version: "1.2", date: "2026-05-13", author: "state-manager", summary: "Architecture Module canonical name SS-03 (Event Emission (OTel-Aligned)); D-346 cite-refresh"}`.

**[process-gap]:** D-346 pre-fix HH-2 grep was schema-version-content-only and did not include frontmatter parity check. The fix-burst scope-determination grep at D-348 time MUST be MULTI-AXIS: simultaneously check body-content patterns (schema_version, subsystem names) AND frontmatter fields (`last_amended:`, `modified:` array) for all files being touched. KK axis (frontmatter parity gate) is proposed for pass-12 (see §7 below).

### F-2 [MEDIUM] SS-03-observability-sinks.md lines 72 + 148 retain stale `dispatcher_trace_id` references — DI-017 sibling-axis missed by D-346 scope-determination grep

**File:** `.factory/specs/architecture/SS-03-observability-sinks.md`
**Locations:** Line 72 ("`dispatcher_trace_id: Uuid`" in event-schema table); Line 148 ("events carry `dispatcher_trace_id`" in prose description).

**Defect:** D-346 amended SS-03-observability-sinks.md for schema_version (v1.1→v1.2) and for one changelog row. However, the DI-017 trace_id rename axis was NOT applied to this file. Per DI-017 v1.2 wire-format-exclusivity §Scope, the legacy alias `dispatcher_trace_id` MUST NOT appear in serialized output prose for active wire-emit semantics. Lines 72 and 148 present `dispatcher_trace_id` as an active field in the event schema table and in descriptive prose — neither is annotated as "renamed from" or "retained as SDK-facing alias only".

**Context:** SS-03-observability-sinks.md is now superseded by SS-03-event-emission.md (per ADR-015 D-15.1 renaming). However, the superseded document remains in the corpus and is used by ARCH-INDEX as a reference. Per POLICY 6 + DI-017 v1.2, the stale `dispatcher_trace_id` prose at lines 72 and 148 constitutes a direct violation regardless of superseded status.

**Severity:** MEDIUM. The file is labeled superseded, which moderates impact, but the violation is structural (DI-017 wire-format-exclusivity is a domain invariant, not an advisory). The fix is surgical: annotate both lines similar to the SS-01 annotation pattern (e.g., `(renamed from dispatcher_trace_id per DI-017 v1.2 / ADR-015; see SS-03-event-emission.md for current spec)` or replace with the canonical `trace_id` field name).

**Routing:** Architect (SS-03-observability-sinks.md is an architecture artifact; annotation or replacement decision requires architect judgment on how to handle a superseded file's internal consistency obligations).

**Closure proposal:**
- SS-03-observability-sinks.md line 72: annotate `` `dispatcher_trace_id: Uuid` `` → `` `trace_id: Uuid` `` (renamed per DI-017 v1.2; was `dispatcher_trace_id`; see SS-03-event-emission.md for current canonical schema)`.
- SS-03-observability-sinks.md line 148: annotate or replace "events carry `dispatcher_trace_id`" → "events carry `trace_id` (renamed from `dispatcher_trace_id` per DI-017 v1.2; see SS-03-event-emission.md for current spec)".
- Bump version v1.2→v1.3 with changelog row citing E-10 pass-11 DI-017 sibling-axis annotation.

### F-3 [MEDIUM] 4 cross-spec stale `SS-03 (Observability Sinks)` cites in story/epic corpus — POLICY 6 violation; sibling-pattern of pass-10 F-3

**Files / Locations:**
- `.factory/stories/epics/E-10-single-stream-otel-event-emission.md` line 18: `SS-03 (Observability Sinks)` in epic scope line
- `.factory/stories/S-4.05-dead-letter-queue.md` line 270: `SS-03 (Observability Sinks)` in implementation reference
- `.factory/stories/S-4.05-dead-letter-queue.md` line 301: `SS-03 (Observability Sinks)` in architecture module row
- `.factory/stories/S-4.05-dead-letter-queue.md` line 426: `SS-03 (Observability Sinks)` in acceptance criteria reference

**Defect:** Per ADR-015 D-15.1, canonical subsystem name is `SS-03 (Event Emission (OTel-Aligned))`. Pass-10 F-3 fixed BC-3.04.001 line 78 for the sibling-pattern in the BC corpus. D-346's II-2 sweep was scoped to `.factory/specs/` only and did not include `.factory/stories/`. The story corpus carries 4 stale subsystem-name references. POLICY 6 (canonical-name SoT from ARCH-INDEX.md) is corpus-wide — stories are not exempt.

**Pattern analysis:** This is the third occurrence of the subsystem-name sibling-sweep scope-narrowness pattern (pass-8 F-3: SS-01 scope-narrow → pass-10 F-3: BC-3.04.001 → pass-11 F-3: story/epic corpus). Each pass's fix-burst defines sweep scope as the files enumerated in the prior-pass adversary report plus direct siblings; the story corpus is not automatically included in the scope-determination grep even though it shares the same POLICY 6 obligation.

**Severity:** MEDIUM. Story corpus `SS-03 (Observability Sinks)` cites create inconsistency between stories (which developers read during implementation) and the canonical ARCH-INDEX subsystem registry. E-10 epic line 18 is particularly visible — the epic's scope statement references the superseded subsystem name.

**Routing:** Architect (story content correctness) and/or PO (epic scope statement). State-manager (version bumps for story/epic files after architect/PO approval).

**Closure proposal:**
- E-10-single-stream-otel-event-emission.md line 18: `SS-03 (Observability Sinks)` → `SS-03 (Event Emission (OTel-Aligned))` + version bump + changelog row.
- S-4.05-dead-letter-queue.md lines 270, 301, 426: same substitution + version bump + changelog row.

### F-4 [LOW] VP-014 frontmatter `bcs:` array drift — body cites BC-7.06.001 + BC-1.01.007 but frontmatter array is `[BC-1.01.001, BC-3.01.003]` only

**File:** `.factory/specs/verification-properties/VP-014.md`
**Locations:** Frontmatter `bcs:` field vs. body Test Evidence prose (post-D-346 v1.1).

**Defect:** After D-346 amended VP-014 lines 45 and 107 (schema_version references), the document body at Test Evidence section cites BC-7.06.001 and BC-1.01.007 in the prose description of what the VP validates. However, the frontmatter `bcs:` array remains `[BC-1.01.001, BC-3.01.003]` — neither BC-7.06.001 nor BC-1.01.007 appears there. This creates a frontmatter↔body alignment gap analogous to F-1's `last_amended:` drift.

**Intent question:** The VP specification does not define whether `bcs:` in frontmatter is (a) all BCs that the VP validates semantically, (b) only the BCs that the VP's formal-proof scope covers, or (c) only the BCs listed in the formal Test Evidence table. The answer determines whether BC-7.06.001 and BC-1.01.007 must be added to the frontmatter `bcs:` array. If interpretation (a), the array is incomplete. If interpretation (b) or (c), the body prose citation may be correct without frontmatter inclusion.

**Severity:** LOW (pending intent verification per S-7.01). Not an implementation defect — the behavioral gap is ambiguous pending schema intent clarification. However, per CLAUDE.md "Pending architect review is forbidden when the question is answerable in current scope" — the schema intent for `bcs:` in VP frontmatter should be resolved and documented in VP-014 comments or VP-INDEX schema notes.

**Routing:** Architect (intent adjudication for VP frontmatter `bcs:` array semantics; document decision inline in VP-014 or VP-INDEX schema header).

**Closure proposal (if frontmatter-includes-all-cited is the correct interpretation):**
- VP-014 frontmatter `bcs:`: add `BC-7.06.001` and `BC-1.01.007`.
- VP-014 v1.1→v1.2 + changelog row citing E-10 pass-11 frontmatter parity fix.

**Closure proposal (if frontmatter-is-formal-proof-scope-only):**
- Add a comment in VP-014 frontmatter block or VP-INDEX schema header documenting that `bcs:` covers formal-proof scope only; body prose citations are informational. v1.1→v1.2 + changelog row documenting intent.

### F-5 [LOW] VP-014 test-harness pseudocode contradiction at lines 56 + 62 — `bad_version in &[0u32, 2, 999]` but `2` is now the GOOD version post-ADR-019

**File:** `.factory/specs/verification-properties/VP-014.md`
**Locations:** Lines 56, 62 (test harness pseudocode block).

**Defect:** D-346 updated VP-014 lines 45 and 107 to reflect `schema_version = 2 (post-ADR-019)` as the canonical GOOD value. However, the test harness pseudocode at lines 56 and 62 still contains `bad_version in &[0u32, 2, 999]` — meaning `2` appears in the "bad versions" list. Post-ADR-019, `2` is the GOOD version. A test harness following this pseudocode would mark `schema_version = 2` as a bad-version rejection case, which is semantically inverted.

**Context:** D-346 was correctly scoped to the inline value-citation lines (45, 107) but missed the pseudocode block. This is a direct analog of the body-content scope-narrowness pattern: the fix-burst scope-determination grep matched `= 1` literal value references but did not match the pseudocode array literal `&[0u32, 2, 999]` because the pattern didn't include numeric-literal-in-array form.

**Severity:** LOW (spec pseudocode, not production code; pseudocode in VP is prescriptive not executed). However, if a verification engineer implements from this pseudocode, the test will incorrectly reject the valid schema version and accept only `0` and `999` as the "bad" test cases — inverting the D-346 intent.

**Routing:** Architect (VP-014 content correctness; the pseudocode is within architect's domain as a verification-property artifact).

**Closure proposal:**
- VP-014 line 56: `bad_version in &[0u32, 2, 999]` → `bad_version in &[0u32, 1, 999]` (1 is the pre-ADR-019 value; 0 and 999 are invalid; 2 is now GOOD).
- VP-014 line 62: same substitution if line 62 contains the same or analogous array literal.
- VP-014 v1.1→v1.2 + changelog row citing E-10 pass-11 test-harness correction (lines 56/62 bad-version array post-ADR-019 fix).

## 4. Observations

**O-1 [process-gap]** ARCH-INDEX v2.00 changelog for D-346+D-347 confirms the correct version-bump accounting (BC-INDEX v2.19 + ARCH-INDEX v2.00) but the narrative summary does not include per-finding inline captured grep output. Per F5 D-449(a) brownfield-layer application (formalized at D-347 seal), the persistence-layer analog of a Dim-2 Attestation block in burst-log is the ARCH-INDEX / BC-INDEX changelog row body. A narrative paraphrase "HH-2 pre-fix grep surfaced 3 additional sites" does not satisfy the mechanical-gate standard. This is the JJ axis finding formalized as F (pending-fail) at D-348 time — see §7 below for the LL axis proposal.

**O-2 [info]** ARCH-INDEX v2.00 + BC-INDEX v2.19 frontmatter sync confirmed correct (gap is per-BC frontmatter only, not the index files themselves). O-1 in pass-10 confirmed this; pass-11 re-verifies: no regression in index-level frontmatter.

**O-3 [info]** Pass-10 historical-quote convention for SS-09-config-activation.md line 313 confirmed retained correctly (contextually appropriate quote of pre-ADR-019 state). No defect introduced by D-346.

**O-4 [info]** F-4 pass-10 (DI-017 §wire-format-exclusivity scope statement) was closed by D-346 — verified. DI-017 v1.2 now includes the SDK-envelope carve-out sentence explicitly. This closure is correct and complete; no regression.

**O-5 [info]** D-347 seal correctly cited "zero stale REGISTRY_SCHEMA_VERSION=1" for the 9 enumerated sites. The claim is accurate for the schema_version axis only. The broader sibling-pattern assertion was not made, which avoids false-closure; however the narrowness of the assertion (9 sites only) did not trigger a pass-11 re-check of sibling-pattern applicability to SS-03-observability-sinks.md dispatcher_trace_id — that gap is F-2 above.

**O-6 [process-gap]** D-347 seal (state-manager) included narrative paraphrase of D-449(a) self-application ("literal-shell-execution-evidence applied") without capturing the actual grep stdout inline in the commit body. Per D-449(a) strict interpretation, "captured stdout" means the grep output must appear verbatim in the persistence artifact (ARCH-INDEX changelog row or commit body), not be described as having occurred. This is the JJ-axis finding's source: at the persistence-layer, narrative-attestation-of-evidence is equivalent to the F5 burst-log Dim-2 pseudocode problem — the evidence appears to exist but is not mechanically verifiable without re-running the grep.

## 5. Novelty Assessment

**Novelty: 7/10.** F-1 is genuinely NEW META-class — **frontmatter↔body drift on per-BC `last_amended:` and `modified:` fields** — invisible to prior axes because grep predicates targeted body-content patterns (schema_version values, subsystem-name strings) rather than frontmatter metadata fields. No prior pass in the E-10 sub-cycle enumerated frontmatter `last_amended:` + `modified:` array parity as a closure axis at fix-burst time.

F-2 and F-3 are KNOWN-pattern recurrences (sibling-sweep scope-narrowness from passes 9 and 10 surfacing at a new sibling-doc layer and a new corpus layer respectively). The underlying pattern — fix-burst scope-determination grep matches the body-content pattern used by the prior-pass adversary, but not the full POLICY 6 blast radius — has now recurred at: SS-01 (pass-8), BC-3.04.001 (pass-10), SS-03-observability-sinks.md dispatcher_trace_id (pass-11 F-2), and stories/epics corpus (pass-11 F-3).

F-4 and F-5 are content-level VP-014 defects **introduced by D-346** — the fix-burst corrected schema_version value citations at lines 45 and 107 but left the test-harness pseudocode (lines 56/62) and frontmatter `bcs:` array unchecked. This is the "production-grade-fix introduces-new-defects" pattern codified at F5 D-451(e): a fix applied to a specific grep-matched line without auditing the full document for implications of the changed canonical value.

**Pattern summary:** The trend rebounded from 4 to 6 findings. This is NOT asymptotic decay toward NITPICK_ONLY — each pass introduces new axes as the fix-burst scope-determination discipline finds new layers to miss. The structural-floor observation is now applicable to the brownfield E-10 sub-cycle: the trajectory mirrors F5's multi-axis recurrence (L-EDP1-003 pattern) but at the brownfield-scope layer. ADR-013/BC-5.39.001 NITPICK_ONLY counter does NOT advance; remains 0/3.

## 6. Verdict

**HIGH** — 1 HIGH + 2 MED + 2 LOW + 0 NITPICK. F-1 is the structurally significant finding: a NEW META-class (frontmatter↔body drift) introducing a 5-BC blast radius, mechanically detectable but missed by all prior passes because no axis targeted frontmatter fields. F-2/F-3 are KNOWN-pattern sibling-sweep recurrences at new corpus layers (superseded arch doc + story/epic corpus). F-4/F-5 are D-346-introduced content defects in VP-014 (pseudocode array + frontmatter `bcs:` array).

## 7. Pass-12 Axes Recommendation

**HH-3 (refinement of HH-2):** Pre-fix scope-determination grep MUST be MULTI-AXIS at fix-burst time. When a fix-burst is addressing findings from multiple active axes (e.g., schema_version + DI-017 trace_id + subsystem name + frontmatter metadata), the scope-determination grep MUST cover ALL pattern classes simultaneously: `grep -rn 'schema_version.*=.*1\|REGISTRY_SCHEMA_VERSION.*1\|dispatcher_trace_id\|Observability Sinks\|last_amended' .factory/specs/ .factory/stories/`. Running N single-pattern greps sequentially does not satisfy MULTI-AXIS scope-determination — the grep must be issued once with all alternation patterns, and the output reviewed as a unified hit list before any patching begins.

**KK (new):** Frontmatter parity gate at fix-burst time. Whenever a BC body receives a changelog row adding version v(N+1), the fix-burst MUST simultaneously sync frontmatter `last_amended:` to today's date AND append to `modified:` array the corresponding `{version, date, author, summary}` entry. State-manager checklist item: before declaring D-NNN fix-burst complete, grep all touched BC/VP/story files for `version:` in H1 title or body changelog and verify each has a matching frontmatter `last_amended:` with the same date. Failure to sync is HIGH severity (F-1 class).

**LL (new):** Literal-shell-execution-evidence at brownfield-cycle persistence layer. D-348 seal and all subsequent brownfield seal commits MUST include captured `grep -rn` stdout INLINE in the ARCH-INDEX / BC-INDEX changelog row (or commit body where the row is too compact), not paraphrased narrative. Brownfield analog of F5 D-449(a) burst-log Dim-2. Specifically: the ARCH-INDEX v2.01+ changelog row for D-348+D-349 must embed the actual captured output of the zero-row verification greps, not a sentence asserting "all gates returned zero rows".

**JJ (broaden):** Production-grade-default audit on D-347 attestation broadened to cover persistence-layer attestation (not just the fix-burst's scope-determination grep). Every brownfield seal commit is a persistence-layer event; the D-449(a) standard applies to it as much as to the F5 burst-log Dim-2 block.

**CC/DD/EE/FF/GG re-verify post-D-348:** Standard axis battery. Specifically verify FF (dispatcher_trace_id) includes SS-03-observability-sinks.md lines 72 + 148 post-F-2 closure; verify II-3 (subsystem-name sweep) includes stories corpus post-F-3 closure.

## 8. Fix-Burst Proposal Sketch (D-348) — NOT EXECUTED, only proposed

**State-manager (F-1 frontmatter parity — 5 BCs):**
- BC-4.04.005: frontmatter `last_amended: 2026-05-13`; `modified:` array v1.3 entry.
- BC-4.05.005: frontmatter add `last_amended: 2026-05-13`; `modified:` array v1.3 entry.
- BC-4.07.004: frontmatter add `last_amended: 2026-05-13`; `modified:` array v1.2 entry.
- BC-4.08.003: frontmatter add `last_amended: 2026-05-13`; `modified:` array v1.3 entry.
- BC-3.04.001: frontmatter add `last_amended: 2026-05-13`; `modified:` array v1.2 entry.
- Multi-axis pre-fix scope-determination grep covering frontmatter + body patterns MUST be run before any edit and output embedded in commit body (HH-3 + KK applied).

**Architect (F-2 — SS-03-observability-sinks.md dispatcher_trace_id sibling-axis):**
- SS-03-observability-sinks.md lines 72 + 148: annotate with DI-017 v1.2 rename note + v1.2→v1.3 + changelog.

**Architect (F-3 — cross-spec subsystem-name sweep in story/epic corpus):**
- E-10-single-stream-otel-event-emission.md line 18: canonical name substitution + version bump + changelog.
- S-4.05-dead-letter-queue.md lines 270, 301, 426: canonical name substitution + version bump + changelog.

**Architect (F-4 — VP-014 frontmatter `bcs:` intent adjudication):**
- Adjudicate VP frontmatter `bcs:` semantics; document decision; apply fix per adjudication.
- VP-014 v1.1→v1.2 + changelog row.

**Architect (F-5 — VP-014 harness bad-version array):**
- VP-014 lines 56/62: `bad_version in &[0u32, 2, 999]` → `bad_version in &[0u32, 1, 999]` + v1.1→v1.2 + changelog.
- (Note: F-4 and F-5 can be bundled into a single VP-014 v1.1→v1.2 touch if both are addressed in D-348.)

**State-manager (D-349 seal — LL axis applied):**
- ARCH-INDEX v2.01 changelog row citing D-348+D-349 with inline literal-shell stdout (captured `grep -rn` zero-row verification output per LL axis recommendation).
- BC-INDEX cite-refresh v2.20.
- INDEX.md pass-11 row sealed + Convergence Status updated.

## 9. Structural-Floor Assessment

The brownfield E-10 sub-cycle is **structurally healthy** (all specs are internally consistent within their version boundaries) but **procedurally trending toward asymptotic-floor analogous to F5 META-LEVEL-29**. Three observations:

1. **Trend rebounded 4→6**: pass-11 produced MORE findings than pass-10, not fewer. The trajectory `22→11→16→16→12→2→1→4→5→4→6` is NOT asymptotically decaying — passes 3, 4, and now 11 show rebounds. The rebounded values are smaller in magnitude than the early-pass highs, but the direction is wrong.

2. **Each pass introduces new axis classes**: passes 9-11 have each surfaced a genuinely new defect class (DI-017 rename scope, schema_version sibling-sweep scope, frontmatter↔body drift) rather than narrowing toward an exhausted finding space. This indicates the procedural discipline at the fix-burst layer has structural gaps that generate new defect classes faster than existing classes are closed.

3. **META-class evolution**: F-1 (frontmatter drift, 5-BC blast radius) mirrors F5's per-ply codification of new META-LEVEL-N classes. If the brownfield cycle does not codify the frontmatter-parity-gate (KK), multi-axis-pre-fix-grep (HH-3), and persistence-layer-literal-shell (LL) as mechanical gates analogous to F5's D-444/D-446/D-448/D-449, the cycle will continue to generate META-class findings at each subsequent pass.

**Three options for human direction:**

**(a) Continue pass-12** with HH-3 + KK + LL discipline applied. Path of least surprise per production-grade default (CLAUDE.md §Canonical Principle). F-1 HIGH severity requires fixing in scope regardless of structural-floor concern. Estimated D-348 scope: ~10 file touches (5 BC frontmatters + SS-03-observability-sinks.md + E-10 epic + S-4.05 3 sites + VP-014). Estimated pass-12: 5-7 findings if HH-3/KK/LL are applied correctly; potential convergence toward 2-3 findings by pass-13 if the three new axes successfully close the remaining structural gaps.

**(b) Pause to codify META-axes** HH-3 + KK + LL as `.factory/policies.yaml` policies BEFORE D-348 fix burst. Path of structural rigor — mirrors F5's D-444+D-446+D-448+D-449 mechanical-gate codification sequence. Specifically: add POLICY 13 (HH-3 multi-axis pre-fix grep mandatory at every fix-burst), POLICY 14 (KK frontmatter parity gate at every BC/VP touch), POLICY 15 (LL literal-shell-execution-evidence at every brownfield seal commit). After policies are registered, resume pass-12 with the policies enforcing the axes. This adds ~1 session turn but provides policy-level enforcement rather than axis-level recommendation.

**(c) Pivot to wave-gate / phase-5 perimeter** for the brownfield cycle as a whole. Path of strategic re-scoping — analogous to F5 META-LEVEL-29 asymptotic-acceptance pivot (D-386 Option C). If the E-10 sub-cycle structural-floor assessment indicates continued per-pass finding regeneration, a phase-5 system-level adversarial review of the entire brownfield artifact corpus (not just E-10 spec package) might close more blast-radius than incremental pass-by-pass E-10 sub-cycle work. This is NOT the production-grade default (F-1 HIGH severity requires closure in scope per CLAUDE.md Canonical Principle Rule 1).

**Recommendation:** Production-grade default per CLAUDE.md says continue — option (a). F-1 HIGH severity requires fixing in scope; deferring via (b) or (c) would require explicit rationale that the deferral is a legitimate scope-boundary defer (CLAUDE.md Rule 3). Options (b) and (c) are valid orthogonal improvements but should NOT block the F-1..F-5 closure cycle. After D-348+D-349 seal, the human may elect to apply (b) as a pass-12 preparation step or (c) as a cycle-level pivot.

---

**Pass-11 produced 6 findings (1 HIGH + 2 MED + 2 LOW). NITPICK_ONLY counter stays at 0/3 (HIGH finding resets). Convergence requires three consecutive NITPICK_ONLY passes per BC-5.39.001 / ADR-013. Trend REBOUNDED 4→6: NOT asymptotic. Structural-floor comparable to F5 META-LEVEL-29 — three options surfaced for human direction per §9 above.**
