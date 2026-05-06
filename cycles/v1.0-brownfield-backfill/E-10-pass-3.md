---
pass: 3
date: 2026-05-06
producer: adversary
artifacts_reviewed:
  - .factory/specs/architecture/decisions/ADR-015-single-stream-otel-schema.md
  - .factory/stories/epics/E-10-single-stream-otel-event-emission.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.11.001.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.11.003.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.12.001.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.12.002.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.12.003.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.12.004.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.12.005.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.12.006.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.12.007.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.12.009.md
  - .factory/specs/behavioral-contracts/ss-02/BC-2.06.001.md
  - .factory/specs/behavioral-contracts/ss-03/BC-3.05.004.md
  - .factory/specs/behavioral-contracts/ss-04/BC-4.09.001.md
  - .factory/stories/S-10.02-adr015-wave1-filesink-single-stream.md
  - .factory/stories/S-10.03-adr015-wave1-resource-attribute-enrichment.md
  - .factory/stories/S-10.04-adr015-wave1-trace-propagation-lifecycle-events.md
  - .factory/stories/S-10.05-adr015-wave2-plugin-schema-migration.md
  - .factory/stories/S-10.09-adr015-wave5-crate-retirement-ss03-rewrite.md
  - .factory/specs/behavioral-contracts/BC-INDEX.md
  - .factory/specs/architecture/ARCH-INDEX.md
  - .factory/specs/domain-spec/capabilities.md
  - .factory/specs/domain-spec/invariants.md
  - .factory/specs/open-questions.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.11.002.md
  - .factory/specs/behavioral-contracts/ss-03/BC-3.05.001.md
  - .factory/specs/behavioral-contracts/ss-03/BC-3.05.002.md
  - .factory/specs/behavioral-contracts/ss-03/BC-3.05.003.md
verdict: HIGH
post_seal_sha: 6af4f64
---

# Adversarial Review — Pass 3 (E-10 single-stream OTel)

Pass 3 finds genuinely new defects, mostly along POLICY 8 sibling-propagation seams that prior fix bursts left half-completed (D-318/D-319/D-320/D-321). Verdict is **HIGH** — there is no CRITICAL implementer-misleading mis-anchor, but there are multiple HIGH-severity content drifts (frontmatter↔body, primary↔sibling, BC body↔BC-INDEX) and one HIGH semantic mis-citation (`default-members` applied to types that were never crates). Convergence counter does NOT advance.

## Critical Findings

(none — the spec package can be implemented without an implementer being misled into building the wrong thing)

## High Findings

### F-1 [HIGH]: BC-1.12.001 attributes `Cargo.toml default-members` exclusion to types `Router`/`SinkRegistry`/`DlqWriter` that were never crates
**Artifact:** /Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-01/BC-1.12.001.md
**Policy / Axis:** POLICY 4 (semantic_anchoring_integrity); Axis C (ADR-015 internal consistency)
**Issue:** BC-1.12.001 Postcondition 2 states: "`Router::submit` is NOT called. `SinkRegistry` dispatch is NOT called. `DlqWriter` is NOT called. These code paths are excluded from `Cargo.toml` `default-members` (deprecated Wave 1) and physically removed (retired Wave 5)." Invariant 3 repeats: "Their absence from `default-members` (deprecated)... enforces this invariant." This is semantically wrong. Per ADR-015 D-15.1 and BC-1.12.007 Postcondition 2/3, only the **`sink-otel-grpc` crate** is excluded from `default-members`. `Router`, `SinkRegistry`, and `DlqWriter` are **types within the kept `sink-core` crate** — they cannot be "excluded from default-members" because they are not crates. ADR-015 v1.4 (F-2) explicitly reconciled this distinction. BC-1.12.001 still uses the pre-v1.4 conflated language.
**Evidence:** BC-1.12.001 lines 64–66 (Postcondition 2); lines 106–108 (Invariant 3). Compare to ADR-015 D-15.1 ("Crates are excluded from `default-members`"), BC-1.12.007 Postcondition 2 ("The deprecated items... are excluded from `Cargo.toml` `default-members`. The workspace's default build (`cargo build`) does NOT compile these crates/types unless explicitly opted in"), and S-10.02 line 248 ("'Deprecated' = excluded from `default-members` + `publish = false`" — applies to crates only).
**Routing:** PO (BC-1.12.001 owner)
**Suggested fix:** Rewrite Postcondition 2 to attribute `default-members` exclusion only to the `sink-otel-grpc` crate; describe Router/SinkRegistry/DlqWriter as "deprecated TYPES within the kept `sink-core` crate (marked `#[deprecated]` or behind feature gates) — Wave 5 physically removes the type definitions." Same fix to Invariant 3.

### F-2 [HIGH]: BC-INDEX line 166 stamps BC-1.11.001 with Story = `S-10.03`, but BC body Story Anchor and Stories cell both say `S-10.04`; S-10.04 frontmatter contains BC-1.11.001
**Artifact:** /Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/BC-INDEX.md
**Policy / Axis:** POLICY 8 (bc_array_changes_propagate); Axis I (cross-BC traceability bidirectional); Axis O (POLICY 8 reverse-direction sweep)
**Issue:** BC-INDEX line 166 records BC-1.11.001 → Stories = `S-10.03`. BC body Story Anchor (BC-1.11.001 line 95–97) and Traceability Stories cell (line 135) both record `S-10.04`. S-10.04 frontmatter (line 24) contains `BC-1.11.001`. S-10.03 frontmatter (lines 25–28) does NOT contain BC-1.11.001. The D-320 F-4 fix sealed BC-1.11.001 into S-10.04's `behavioral_contracts:` array and updated the BC's Story Anchor + Stories cell — but BC-INDEX was not updated in the same burst. This is a burst-completion gap.
**Evidence:** BC-INDEX.md line 166: `| [BC-1.11.001](ss-01/BC-1.11.001.md) | factory-dispatcher::host::exec_subprocess::injects... | draft | CAP-029 | S-10.03 |`; BC-1.11.001.md line 97 (Story Anchor: `S-10.04`); BC-1.11.001.md line 135 (Stories: `S-10.04`); S-10.04.md frontmatter line 24 (`- BC-1.11.001`); S-10.03.md frontmatter lines 25–28 (no BC-1.11.001 entry).
**Routing:** state-manager
**Suggested fix:** In BC-INDEX line 166, change Stories cell from `S-10.03` to `S-10.04`. Add a BC-INDEX changelog entry citing this as a D-320 follow-up (the F-4 fix burst missed BC-INDEX propagation).

### F-3 [HIGH]: Story S-10.05 body still says "four-state taxonomy" in AC-002 and AC-010 after BC-1.12.009 was canonicalized to five-state
**Artifact:** /Users/jmagady/Dev/vsdd-factory/.factory/stories/S-10.05-adr015-wave2-plugin-schema-migration.md
**Policy / Axis:** POLICY 8 (BC-array changes propagate to body and ACs); Axis F (dual-emit five-state classification); partial-fix regression discipline (S-7.01)
**Issue:** BC-1.12.009 H1 enumerates "five-state event classification (paired-current / paired-deprecated / orphaned-deprecated-half / orphaned-current-half / non-paired)" (D-315 F-12 + D-319 F-2 sweep). The S-10.05 BC table row (line 154) correctly carries the five-state H1. But the story body still says: AC-002 line 81 "This implements the D-15.2.e **four-state** identity contract"; AC-010 title line 179 "consumers classify dual-emit pairs using **four-state taxonomy**"; AC-010 body line 182 "the BC-1.12.009 **four-state taxonomy**"; AC-010 enumeration lines 184–187 lists only States 1–4 (omits the fifth `non-paired` state). The D-320 fix swept the BC table row (line 154) but missed the AC text — primary-vs-body partial propagation.
**Evidence:** S-10.05.md lines 81, 179, 182, 184–187 (still "four-state"); BC-1.12.009.md line 28 (H1: "five-state"); BC-INDEX.md line 176 (cell: "five-state"). The S-10.05 v1.3 changelog claims "BC-1.12.009 body BC table row state-order corrected to match H1 verbatim" but did not amend the AC body.
**Routing:** story-writer (S-10.05)
**Suggested fix:** In S-10.05 AC-002, AC-010 title, AC-010 body intro, replace "four-state" with "five-state". Append a fifth row "Non-paired: all three correlation fields absent" to the AC-010 enumeration. Add v1.4 changelog entry.

### F-4 [HIGH]: BC-1.11.002 still has `capability: "CAP-TBD"` while sibling BC-1.11.001 was re-anchored to CAP-029 in D-321 — sibling-fix regression
**Artifact:** /Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-01/BC-1.11.002.md
**Policy / Axis:** Partial-fix regression discipline (S-7.01) — sibling files in same architectural layer; POLICY 4 (semantic_anchoring_integrity)
**Issue:** D-317 swept CAP-TBD → CAP-029 for BC-1.11.001; D-321 swept additional capability re-anchors. BC-1.11.002 (FileSink partial-write recovery, sibling in the same BC-1.11.* cluster covering ADR-015 single-stream contracts) STILL has `capability: "CAP-TBD"` (frontmatter line 16) and `L2 Capability | CAP-TBD` (line 162). This is a same-layer sibling skipped by the cap-anchor sweep. BC-1.11.002 governs the FileSink write-failure cascade — the same single-stream surface that CAP-029 covers (FileSink IS the single-stream writer per CAP-029). BC-INDEX line 167 also reflects CAP-TBD. Per partial-fix regression discipline, blast radius = 2 files (BC-1.11.002 + BC-INDEX) → HIGH.
**Evidence:** BC-1.11.002.md line 16 (`capability: "CAP-TBD"`), line 162 (`L2 Capability | CAP-TBD`); BC-INDEX.md line 167 (`CAP-TBD`); compare to BC-1.11.001 (CAP-029 since D-321), BC-1.11.003 (CAP-009 since D-317). BC-1.12.001 throughout references BC-1.11.002 as the failure-path sibling.
**Routing:** PO (BC-1.11.002 owner) + state-manager (BC-INDEX)
**Suggested fix:** Re-anchor BC-1.11.002 to CAP-029 with a Capability Anchor Justification paragraph explaining how the partial-write recovery contract preserves the CAP-029 single-stream guarantee against FileSink write failures. Update BC-INDEX line 167. Mark a CAP-anchor-sweep follow-up for BC-1.11.* cluster.

### F-5 [HIGH]: BC-1.12.007 carries an internally contradictory "TD-015-a (closed in D-318)" claim while body simultaneously describes the TD as deferred-with-no-owner
**Artifact:** /Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-01/BC-1.12.007.md
**Policy / Axis:** POLICY 4 (semantic_anchoring_integrity); Axis G (Wave-1 call-graph invariant testable, no contradictions)
**Issue:** Section "Open Questions / Future Work" (line 126) states: "TD-015-a (closed in D-318)... Resolution: cargo-call-stack chosen for consistency with Postcondition 1's named verification tool. Owner: TBD — assignment flagged for sprint planning. Decision-by date: Wave 5 closure (unchanged). The check is NOT a postcondition of Wave 1 but MUST be implemented before Wave 5 crate deletion." This is contradictory: a "closed" TD does not have "Owner: TBD" or "Decision-by date: Wave 5 closure." Either the tool selection was decided (the only thing D-318 actually closed) but the implementation/assignment remain deferred, OR the closure marker is wrong. Worse, the BC H1 still says `TD-015-a CI check deferred to Wave 5`; Invariant 3 says `the workspace MUST be manually audited (or TD-015-a implemented) to ensure no re-coupling occurs`; the Traceability "Technical Debt" row says `TD-015-a — CI check... deferred; must be resolved before Wave 5 deletion`. Five places describe TD-015-a as deferred while one says "closed." An implementer reading this BC cannot determine whether they must implement the cargo-call-stack CI check now or whether it is still deferred.
**Evidence:** BC-1.12.007.md line 28 (H1: "deferred"), line 99–105 (Invariant 1: "verified by callgraph static analysis using `cargo-call-stack` at every PR merge to develop" — implies live), line 113–118 (Invariant 3: "manual audit... or TD-015-a implemented" — implies deferred), line 126 (TD section: "closed in D-318" + "Owner: TBD" — internally contradictory), line 210 (Traceability: "deferred").
**Routing:** PO (BC-1.12.007 owner)
**Suggested fix:** Reword line 126 to clarify what was closed: "TD-015-a tool selection (closed in D-318): `cargo-call-stack` chosen as the verification tool. Implementation/owner assignment of the CI check itself remains deferred to pre-Wave-5; this BC's invariants cite cargo-call-stack as the named tool, but the CI gate is not yet wired." Either remove "closed in D-318" altogether or restrict the closure scope explicitly.

### F-6 [HIGH]: BC-1.11.003 Stories cell stale ("Wave 2 story (TBD — dual-emit shim wave)") despite S-10.05 being its actual anchor; no `## Story Anchor` heading
**Artifact:** /Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-01/BC-1.11.003.md
**Policy / Axis:** POLICY 8 reverse-direction (BC ↔ story `bcs:` array bidirectional); Axis I (Cross-BC traceability)
**Issue:** S-10.05 frontmatter (line 25) lists `BC-1.11.003`. BC-INDEX line 168 names Stories = `Wave 2 TBD`. BC-1.11.003 body has no `## Story Anchor` heading at all (compare to BC-1.11.001 line 95, BC-1.12.001 line 137, BC-1.12.003 line 170, etc.). The Traceability row (line 206) says `Stories | Wave 2 story (TBD — dual-emit shim wave)`. POLICY 8 reverse-direction (D-319 F-3) is supposed to ensure BC Story Anchor ↔ story `bcs:` array bidirectional sync. This BC was missed: S-10.05 was authored after BC-1.11.003 and references it but BC-1.11.003 still says "TBD". Same pattern as F-2 but for a different BC — this is the second instance of the same defect class.
**Evidence:** BC-1.11.003.md line 206 (Stories cell still TBD); no `## Story Anchor` section anywhere in file (Grep `^## Story Anchor` returns no matches in BC-1.11.003.md); S-10.05.md line 25 (`- BC-1.11.003`); BC-INDEX.md line 168 (Stories cell `Wave 2 TBD`).
**Routing:** PO (BC-1.11.003 owner) + state-manager (BC-INDEX)
**Suggested fix:** Add a `## Story Anchor` section before `## VP Anchors` reading "S-10.05 (Wave 2: Plugin schema migration — emit_pair host helper for atomic dual-emit, optional migration path per AC-009)". Update Traceability Stories cell to S-10.05. Update BC-INDEX line 168 from "Wave 2 TBD" to "S-10.05".

### F-7 [HIGH]: Multiple BCs incremented version frontmatter (v1.1+, v1.2+) without adding a `## Changelog` section to record what changed — pattern across 4 E-10 BCs
**Artifact:** BC-1.11.001 (v1.1), BC-1.11.003 (v1.1), BC-1.12.001 (v1.1), BC-1.12.006 (v1.1), BC-1.12.007 (v1.2)
**Policy / Axis:** POLICY 5 (creators_justify_anchors — extends to history-of-changes); axis behavior of partial-fix regression
**Issue:** Five E-10 BCs have frontmatter `version: > 1.0` but lack a `## Changelog` section in the body. Other E-10 BCs that received the same fix-burst sweeps (BC-1.12.002, BC-1.12.003, BC-1.12.004, BC-1.12.005, BC-1.12.009) DO have `## Changelog` sections. This is a same-burst sibling propagation gap with blast radius ≥ 5. An adversarial reviewer (or implementer) cannot determine WHAT changed at each version bump — defeating the purpose of the version field.
**Evidence:** Grep `## Changelog` over E-10 BC files returns 5 matches (BC-1.12.002/003/004/005/009 + BC-1.05.012). BC-1.11.001 frontmatter version "1.1", no changelog. BC-1.11.003 version "1.1", no changelog. BC-1.12.001 version "1.1", no changelog (and the v1.0→v1.1 move is implied by D-318 sweep but undocumented in the body). BC-1.12.006 version "1.1", no changelog. BC-1.12.007 version "1.2", no changelog (despite F-9 P1 + Inv1 sharpening per S-10.09 v1.2 changelog mentioning BC-1.12.007 "F-9 P1 + Inv1 sharpening"). Per partial-fix regression rule, blast radius ≥ 5 → HIGH severity with pattern flag.
**Routing:** PO (each BC owner)
**Suggested fix:** Add `## Changelog` table to each affected BC documenting the v1.0 → v1.1+ moves with date and one-line description per row. Cross-reference D-numbers (D-315/D-317/D-318/D-319/D-320/D-321) where applicable.

## Medium Findings

### F-8 [MEDIUM]: BC-1.11.003 frontmatter `subsystem: "SS-01"` but Architecture Module spans SS-01 + SS-02; capability anchor is CAP-009 (SS-02-owned)
**Artifact:** /Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-01/BC-1.11.003.md
**Policy / Axis:** POLICY 6 (architecture is subsystem-name source of truth); POLICY 4 (semantic anchoring integrity)
**Issue:** BC-1.11.003 has `subsystem: "SS-01"` (frontmatter line 14, file-system location ss-01/), `L2 Capability | CAP-009` (line 203 — per ARCH-INDEX, CAP-009 is anchored to SS-02), and `Architecture Module | SS-01 — crates/factory-dispatcher/src/host/emit_event.rs; SS-02 — crates/hook-sdk/src/host.rs` (line 205). The capability justification at line 210 explicitly grounds CAP-009 in the SDK side: "the `vsdd-hook-sdk` crate provides the... `vsdd::*` host function bindings... emit_pair function is precisely one of those `vsdd::*` host function bindings: it lives in `vsdd_hook_sdk::host`." So the BC's primary semantic surface is SS-02 (SDK), but it lives in ss-01/ subdirectory with `subsystem: "SS-01"`. This conflicts with POLICY 6 (ARCH-INDEX is canonical) — the BC ID is in the SS-01 cluster but the BC's own justification places it in SS-02. Compare BC-2.06.001 (SS-02, also CAP-009) and BC-4.09.001 (SS-04, CAP-009) — both NEW BCs were correctly placed in their authoritative subsystem clusters. BC-1.11.003 was authored before this discipline crystallized and should be relocated or its split clarified.
**Evidence:** BC-1.11.003.md lines 14, 203, 205, 210; ARCH-INDEX.md line 75 (SS-02 owns hook-sdk); CAP-009 in capabilities.md line 58–61 (Subsystems: SS-02). Compare to BC-2.06.001 and BC-4.09.001 (correctly placed).
**Routing:** Architect (subsystem boundary adjudication) — orchestrator may file LOW pending intent verification
**Suggested fix:** Architect to adjudicate: (a) leave BC-1.11.003 in SS-01 because the dispatcher-side `emit_pair` host function lives in factory-dispatcher; OR (b) split into two BCs (one SS-01 host-side, one SS-02 SDK-side); OR (c) relocate to SS-02 since the SDK ABI is the dominant surface. If (a), update Capability Anchor Justification to lead with SS-01 host-side coordination role rather than SS-02 SDK exposure.

### F-9 [MEDIUM]: Epic E-10 frontmatter `subsystems_affected: [SS-01, SS-03]` but the spec package now adds BCs in SS-02 (BC-2.06.001) and SS-04 (BC-4.09.001)
**Artifact:** /Users/jmagady/Dev/vsdd-factory/.factory/stories/epics/E-10-single-stream-otel-event-emission.md
**Policy / Axis:** POLICY 6 (architecture-is-subsystem source of truth); Axis I (Cross-BC traceability)
**Issue:** Epic E-10 frontmatter (line 14) declares `subsystems_affected: [SS-01, SS-03]`. The Subsystem Anchors section (lines 86–90) only describes SS-01 and SS-03. But D-321 added BC-2.06.001 (SS-02) and BC-4.09.001 (SS-04) as part of the E-10 Wave 2 plugin schema migration. S-10.05 frontmatter (line 33) declares `subsystems: ["SS-01", "SS-02", "SS-04"]`. The epic frontmatter and Subsystem Anchors section are now out of sync with their constituent stories and BCs. An adversarial reviewer scanning for E-10 footprint via the epic would miss SS-02 and SS-04 entirely.
**Evidence:** E-10 epic line 14 (`subsystems_affected: [SS-01, SS-03]`); lines 86–90 (only SS-01 and SS-03 anchors); S-10.05 line 33 (`subsystems: ["SS-01", "SS-02", "SS-04"]`); BC-2.06.001 line 15 (`subsystem: "SS-02"`); BC-4.09.001 line 15 (`subsystem: "SS-04"`); E-10 changelog (line 196) does not mention SS-02/SS-04 expansion.
**Routing:** story-writer (epic owner)
**Suggested fix:** Update E-10 frontmatter `subsystems_affected: [SS-01, SS-02, SS-03, SS-04]`. Add SS-02 and SS-04 anchor entries to Subsystem Anchors section noting their Wave 2 contribution (SS-02 SDK semver bump per BC-2.06.001; SS-04 plugin event-name migration per BC-4.09.001). Add a v1.6 changelog entry.

### F-10 [MEDIUM]: BC-3.05.004 cites BC-1.12.007 as a "sibling" for "schema also removes multi-sink stanzas" but BC-1.12.007 covers call-graph invariant, not config-schema removal — false sibling claim
**Artifact:** /Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-03/BC-3.05.004.md
**Policy / Axis:** POLICY 4 (semantic_anchoring_integrity); Axis I (cross-BC traceability)
**Issue:** BC-3.05.004 Related BCs (line 188) says: "BC-1.12.007 — Wave 1 call-graph invariant (sibling: the config schema also removes multi-sink stanzas; this BC is the schema-side enforcement)". This conflates two unrelated concerns. BC-1.12.007 is about the dispatcher call-graph invariant (Router/SinkRegistry/DlqWriter not called); it has nothing to do with config-schema enforcement. The "schema also removes multi-sink stanzas" claim is about ADR-015 D-15.1, not about BC-1.12.007 specifically. A reader chasing this related-BC cross-reference will find no schema content in BC-1.12.007. The Related BCs section is supposed to identify true behavioral siblings.
**Evidence:** BC-3.05.004.md line 188 (current text); BC-1.12.007.md scope (call-graph invariant, types not called from production hot path; no mention of `[[sinks]]` config stanzas or schema validation).
**Routing:** PO (BC-3.05.004 owner)
**Suggested fix:** Reword line 188 to: "BC-1.12.007 — Wave 1 call-graph invariant (cross-cutting: BC-1.12.007 is the runtime call-graph enforcement; this BC is the static config-schema enforcement; together they ensure ADR-015 D-15.1 multi-sink retirement holds at both compile-time/config-time and runtime)." Or move BC-1.12.007 reference out of Related BCs and into Architecture Anchors as a cross-cutting note.

### F-11 [MEDIUM]: Capability Anchor Justification of BC-1.12.007 quotes a string that is NOT in CAP-029's actual text
**Artifact:** /Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-01/BC-1.12.007.md
**Policy / Axis:** Axis B (TD-VSDD-093 closure-narrative source-of-truth validation); Axis J (CAP-029 / CAP-030 quality)
**Issue:** BC-1.12.007 line 204 (Capability Anchor Justification) quotes: "CAP-029 states 'Router, SinkRegistry, and DlqWriter are retired; all downstream multi-sink fan-out is delegated to an external OTel Collector.'" Verifying against capabilities.md §CAP-029 (line 95–97), the actual CAP-029 wording is: "The dispatcher writes every user-facing domain event as a JSONL record to a single `events-YYYY-MM-DD.jsonl` file via FileSink. Router, SinkRegistry, and DlqWriter are retired; all downstream multi-sink fan-out is delegated to an external OTel Collector that reads the file." The quote is substantially correct but truncates "that reads the file" — a near-miss. More important: a similar quote pattern in BC-1.12.001 line 183 says CAP-029 reads "The dispatcher writes every user-facing domain event as a JSONL record to a single `events-YYYY-MM-DD.jsonl` file via FileSink. Router, SinkRegistry, and DlqWriter are retired." — which omits the "all downstream..." sentence and the closing "that reads the file." Both are paraphrases attributed as direct quotes (no ellipsis). This is TD-VSDD-093 source-of-truth violation.
**Evidence:** capabilities.md line 95–97 (actual CAP-029); BC-1.12.007.md line 204; BC-1.12.001.md line 183. Compare to BC-1.12.005.md line 181 which uses CAP-029 as a thematic reference rather than a direct quote — correct pattern.
**Routing:** PO (each BC owner)
**Suggested fix:** Either replace the in-text "CAP-029 states '...'" pattern with "Per CAP-029 (capabilities.md §CAP-029): the single-stream design retires Router/SinkRegistry/DlqWriter..." (no quote marks), OR include the full CAP-029 text with proper ellipsis. Apply consistently across all BCs in the cluster.

### F-12 [MEDIUM]: Story S-10.04 AC-005 routes BC-1.12.003 EC-014 testing to S-10.04 but EC-014 was authored as a "host_id_fallback" emission test — verification ownership unclear
**Artifact:** /Users/jmagady/Dev/vsdd-factory/.factory/stories/S-10.04-adr015-wave1-trace-propagation-lifecycle-events.md
**Policy / Axis:** Axis I (cross-BC traceability bidirectional); POLICY 8
**Issue:** S-10.04 AC-005 (line 117) traces to BC-1.12.003 EC-014. AC-005 says "This AC validates the lifecycle event contract established in S-10.03 AC-003 by confirming the event type exists in the `event.category` registry as `lifecycle`." But BC-1.12.003 EC-014 (line 197) tests something different: it tests that `gethostname()` returns empty → terminal `unknown-host` fallback → `vsdd.internal.host_id_fallback.v1` emission. The AC-005 falsifiable test (line 124) says: "Unit test: `categorize('vsdd.internal.host_id_fallback.v1')` = `lifecycle`." This is a registry-categorization test, not an EC-014 test. The trace target (BC-1.12.003 EC-014) tests emission; the AC tests categorization. They don't actually trace to each other behaviorally. This conflates two distinct test surfaces under one AC.
**Evidence:** S-10.04.md AC-005 lines 117–124; BC-1.12.003.md EC-014 line 197 (cascade fallback emission test); BC-1.12.003.md EC-007 line 189 (also emission test). The AC-005 target should arguably be BC-1.12.004 Postcondition 2 (the registry table for `vsdd.internal.*` → `lifecycle`).
**Routing:** story-writer (S-10.04)
**Suggested fix:** Either re-target AC-005 trace from "BC-1.12.003 EC-014" to "BC-1.12.004 Postcondition 2 (vsdd.internal.* prefix → lifecycle in registry)", OR expand AC-005 to two tests: one categorization test (per current text) AND one emission test (per BC-1.12.003 EC-014). Distinguish via separate ACs if needed.

## Low Findings

### F-13 [LOW] (pending intent verification): BC-1.12.006 Description still says "exit code 2" but ADR-015 D-15.3 §Block path audit trail does not mandate exit code 2 explicitly
**Artifact:** /Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-01/BC-1.12.006.md
**Policy / Axis:** Axis C (ADR-015 internal consistency); Axis H (CAP-029/CAP-008 dual-anchor)
**Issue:** BC-1.12.006 Description (line 35) says "the dispatcher exited with code 2 (the block exit code) but emitted NO observable event" and Postconditions describe "exit code 2 is deferred until after the audit event is written" (line 53), "Exit code 2 is not returned to Claude Code until..." (line 76). The "exit code 2" comes from BC-1.08.001 (referenced in Related BCs line 113). ADR-015 D-15.3 §Block path audit trail (lines 374–378 of the ADR) only says "before exiting" — it doesn't specify exit code 2. This is a downstream inheritance from BC-1.08.001 but the BC body doesn't make that inheritance explicit. An adversarial reviewer may flag the "exit code 2" claim as unsourced; the actual source is BC-1.08.001, not ADR-015.
**Evidence:** BC-1.12.006.md lines 35, 53, 76; ADR-015 D-15.3 lines 374–378; BC-1.08.001 (referenced as Related BC at line 113).
**Routing:** PO (BC-1.12.006 owner) — orchestrator adjudicates intent
**Suggested fix:** Add a parenthetical citation to BC-1.08.001 wherever "exit code 2" appears: e.g., "exit code 2 (per BC-1.08.001 — `HookResult::Block` exit-code semantics)". Optional but improves source-of-truth traceability per TD-VSDD-093.

### F-14 [LOW]: BC-2.06.001 Postcondition 2 enumerates host-owned fields but the list does not exactly match BC-1.12.003 Postcondition 1's 15-field Resource attributes table
**Artifact:** /Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-02/BC-2.06.001.md
**Policy / Axis:** Axis P (BC-2.06.001 quality); POLICY 4
**Issue:** BC-2.06.001 Postcondition 2 (line 73–80) lists Resource fields plugin authors must remove: "`service.name`, `service.namespace`, `service.instance.id`, `service.version`, `deployment.environment.name`, `host.name`, `host.id`, `os.type`, `process.pid`, `vcs.*`, `worktree.id`, `schema_url`". The list says `vcs.*` which is non-canonical — BC-1.12.003 Postcondition 1 enumerates four distinct VCS fields: `vcs.repository.url.full`, `vcs.repository.name`, `vcs.provider.name`, `vcs.owner.name`. The `vcs.*` shorthand is undefined. A plugin author reading BC-2.06.001 cannot know exactly which VCS fields are host-owned vs plugin-asserted (e.g., `vcs.ref.head.name`, `vcs.ref.head.revision`, `vcs.ref.head.type` are per-event fields per BC-1.12.004, NOT Resource attributes). The wildcard collapses two field categories.
**Evidence:** BC-2.06.001.md line 75 (`vcs.*`); BC-1.12.003.md lines 73–78 (four explicit `vcs.repository.*`, `vcs.provider.name`, `vcs.owner.name`); BC-1.12.004.md lines 96–99 (per-event `vcs.ref.head.name|revision|type`).
**Routing:** PO (BC-2.06.001 owner)
**Suggested fix:** Replace `vcs.*` in BC-2.06.001 Postcondition 2 with the explicit four VCS Resource fields from BC-1.12.003: `vcs.repository.url.full, vcs.repository.name, vcs.provider.name, vcs.owner.name`. Note that `vcs.ref.head.*` are per-event identity fields (per BC-1.12.004) — also host-stamped but in a different category.

### F-15 [LOW]: BC-INDEX changelog entry for D-321 mentions "BC-1.12.006 CAP-008→CAP-029 primary + CAP-008 secondary" but BC-1.12.006 frontmatter `capability:` is CAP-029 only — secondary cap not stored in frontmatter
**Artifact:** /Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/BC-INDEX.md
**Policy / Axis:** POLICY 4; POLICY 7 (BC H1 is title source of truth)
**Issue:** BC-INDEX changelog (line 15) records "BC-1.12.006 CAP-008→CAP-029 primary + CAP-008 secondary." BC-1.12.006 frontmatter `capability:` is "CAP-029" alone (line 16). The body has both `L2 Capability | CAP-029` and a separate `Secondary Capability Reference | CAP-008` row in Traceability (line 174). BC-INDEX line 174 stamps the row's Capability column as `CAP-029` (not `CAP-029, CAP-008` or similar). The secondary anchor is not visible from BC-INDEX or frontmatter — it exists only in the body Traceability table. This is consistent with the "primary anchor in frontmatter" pattern but means the secondary anchor is not machine-discoverable. For BC-3.05.004 the same pattern applies (CAP-029 primary + CAP-010 secondary, secondary not in frontmatter). For BC-4.09.001 the same (CAP-009 primary + CAP-029 secondary). This is a project-wide convention, not a bug — but it merits a process note: the orchestrator's POLICY 4 audit cannot detect a stale secondary cap from index data alone. **[process-gap]**
**Evidence:** BC-1.12.006.md line 16 (frontmatter capability), line 174 (secondary in body); BC-INDEX.md line 174 (cell shows `CAP-029` only); same pattern in BC-3.05.004 (frontmatter line 17 vs body line 273) and BC-4.09.001 (frontmatter line 16 vs body line 219).
**Routing:** state-manager / orchestrator (process gap)
**Suggested fix:** Either (a) extend BC frontmatter schema to support `secondary_capabilities: [CAP-XXX]` array; OR (b) extend BC-INDEX Capability column to render `CAP-PRIMARY (+CAP-SECONDARY)`; OR (c) document the convention explicitly in BC template/policy that secondary cap is body-only and the index reflects primary only. Choice is governance-level. Tag `[process-gap]`.

### F-16 [LOW]: BC-1.12.007 architecture anchor cites `factory-dispatcher::sinks::Sink trait dispatch surface` but the surface that ADR-015 actually retires is the `Router` wiring, not the `Sink` trait itself
**Artifact:** /Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-01/BC-1.12.007.md
**Policy / Axis:** Axis B (closure-narrative source-of-truth validation)
**Issue:** BC-1.12.007 line 137 (Architecture Anchors) reads: "`factory-dispatcher::sinks::Sink` trait dispatch surface (in `crates/factory-dispatcher/src/sinks/mod.rs`) — the open integration point that ADR-015 closes; `Router::submit` is NOT wired here after Wave 1; this TODO is resolved by removing the comment and the dead code." The `Sink` trait is in `crates/sink-core/`, not `crates/factory-dispatcher/src/sinks/`. ADR-015 D-15.1 retains `sink-core` (FileSink + Sink trait) and retires only Router/SinkRegistry/DlqWriter types within sink-core. The architectural anchor as written conflates the sinks/mod.rs integration TODO (legitimate anchor) with the Sink trait dispatch surface (kept). Minor mis-citation; reader confusion possible.
**Evidence:** BC-1.12.007.md line 137; ADR-015 D-15.1 lines 127–135 ("`sink-core` trait and `sink-file` driver are KEPT — `FileSink` becomes the direct writer for `events-*.jsonl`. The `sink-otel-grpc` crate AND the `Router`, `SinkRegistry` types within `sink-core` are retired").
**Routing:** PO (BC-1.12.007 owner)
**Suggested fix:** Reword to "the open integration TODO at `factory-dispatcher::sinks::mod` (in `crates/factory-dispatcher/src/sinks/mod.rs`) — the wiring point for `Router::submit` that ADR-015 closes by leaving it unwired. The `Sink` trait itself in `sink-core` is KEPT per D-15.1; only the Router wiring is retired here." Stable-anchor note already references function/trait name rather than line numbers.

## Per-Policy Compliance

| POLICY | Compliance | Findings |
|--------|------------|----------|
| POLICY 1 (append-only numbering) | OK | No ID reuse; BC-3.05.004 was correctly assigned next free slot per D-312 |
| POLICY 2 (lift_invariants_to_bcs) | OK | DI-007/008/011/012/013/014/017 amendments propagated to BC-1.12.002, BC-1.12.003, BC-3.05.004 |
| POLICY 3 (state_manager_runs_last) | N/A | Not visible from artifacts at HEAD |
| POLICY 4 (semantic_anchoring_integrity) | DRIFT | F-1, F-4, F-10, F-11 |
| POLICY 5 (creators_justify_anchors) | DRIFT | F-7 (no-changelog pattern); generally substantive justifications elsewhere |
| POLICY 6 (architecture_is_subsystem_name_source_of_truth) | DRIFT | F-8 (BC-1.11.003 SS-01 vs SS-02), F-9 (epic subsystems list stale) |
| POLICY 7 (bc_h1_is_title_source_of_truth) | OK at H1 level | F-3 fixes body but H1↔index sync is solid |
| POLICY 8 (bc_array_changes_propagate_to_body_and_acs) | DRIFT | F-2, F-3, F-6 (multiple sibling-propagation gaps) |
| POLICY 9 (vp_index_is_vp_catalog_source_of_truth) | N/A | Pre-VP-assignment |
| POLICY 10 (demo_evidence_story_scoped) | N/A | Pre-implementation |
| POLICY 11 (no_test_tautologies) | N/A | Pre-implementation |
| POLICY 12 (bc_tv_emitter_consistency) | OK | TVs designed as future-implementation witnesses, sane patterns |

## Per-Axis Compliance

| Axis | Compliance | Findings |
|------|------------|----------|
| A. TD-VSDD-091 stable-anchor citations | OK | All BCs use function/method-name anchors with explicit "stable anchor per TD-VSDD-091; line numbers not authoritative" notes |
| B. TD-VSDD-093 closure-narrative source validation | DRIFT | F-11, F-16 (paraphrase-as-quote and mis-attributed module path) |
| C. ADR-015 internal consistency | DRIFT | F-1 (default-members type vs crate confusion in BC-1.12.001) |
| D. OQ-W16-011 RESOLVED enforcement | OK | BC-1.12.002 v1.1, BC-3.05.004 PC5 both encode 12-factor override correctly |
| E. OQ-W16-012 RESOLVED enforcement | OK | BC-3.05.001/002/003 marked retired with superseded_by ADR-015 |
| F. Dual-emit five-state classification | DRIFT | F-3 (S-10.05 still says four-state in body) |
| G. Wave-1 call-graph invariant | DRIFT | F-5 (BC-1.12.007 TD-015-a "closed in D-318" contradicts deferred body) |
| H. CAP-029/CAP-008 dual-anchor | OK | BC-1.12.006 secondary CAP-008 reference is substantive and well-justified |
| I. Cross-BC traceability bidirectional | DRIFT | F-2 (BC-INDEX line 166 stale), F-6 (BC-1.11.003 stories cell stale) |
| J. CAP-029/CAP-030 quality | OK with errata | F-14 (vcs.* shorthand non-canonical in BC-2.06.001) |
| K. CAP-003/CAP-023/CAP-024 supersession | OK | All three have explicit "Status: SUPERSEDED/REWRITTEN per ADR-015" markers |
| L. DI amendment correctness | OK | DI-007/011/012/013/017 amendments traced into BCs |
| M. Re-anchor justification quality | OK | BC-1.12.001/003/004/005/006/007/009 all carry substantive paragraphs |
| N. POLICY 8 four-way audit on S-10.04 + S-10.05 | DRIFT | F-3 (S-10.05 body four-state vs five-state) |
| O. POLICY 8 reverse-direction sweep | DRIFT | F-2, F-6 (two stale Stories cells in BCs that should reference S-10.04 and S-10.05) |
| P. BC-2.06.001 quality | OK with caveats | F-14 (vcs.* shorthand) |
| Q. BC-4.09.001 quality | OK | Dual CAP-009 primary + CAP-029 secondary anchor is substantive; emit_pair contract aligns with BC-1.11.003 |
| R. CAP-030 errata vs BC-1.12.003 alignment | OK | BC-1.12.003 v1.2 errata correctly references ADR-015 D-15.2 as authoritative; capability anchor justification mirrors BC's own Postcondition 1 |
| S. BC-1.11.001 capability resolution | OK | CAP-029 anchor with substantive POLICY 5 justification citing trace_id stamping per DI-017 amendment; Story Anchor S-10.04 added |
| T. BC-1.12.006 dual-anchor coherence | OK | Secondary CAP-008 paragraph cites Postconditions explicitly and explains gating→audit-trail closure |

## Convergence Status

- Pass 3 verdict: **HIGH** (substantive findings remain — primarily POLICY 8 burst-completion gaps and one HIGH semantic mis-citation about `default-members`).
- Convergence counter: stays at **0** (NOT advanced).
- Outstanding fix-burst topics: F-1 (BC-1.12.001 default-members), F-2/F-6 (BC-INDEX + BC-1.11.003 Stories cell), F-3 (S-10.05 four-state→five-state), F-4 (BC-1.11.002 CAP re-anchor), F-5 (BC-1.12.007 TD-015-a contradiction), F-7 (changelog gaps across 5 BCs), F-9 (epic subsystem list stale).
- Pass 4 should re-verify these and look for residual sibling-propagation seams (the same defect class recurs).

## Routing Summary

| Routing target | Findings |
|----------------|----------|
| PO (product-owner) | F-1, F-4, F-5, F-7, F-10, F-11, F-13, F-14, F-16 |
| story-writer | F-3, F-9, F-12 |
| state-manager | F-2 (BC-INDEX line 166), F-4 (BC-INDEX line 167), F-6 (BC-INDEX line 168), F-15 (process gap) |
| Architect | F-8 (BC-1.11.003 subsystem boundary adjudication) |

## Process-Gap Candidates

- **[process-gap] F-15:** BC frontmatter has only `capability:` (single primary). Secondary capability anchors live only in the body Traceability table, invisible to BC-INDEX and machine-discovery. Affects BC-1.12.006 (CAP-029+CAP-008), BC-3.05.004 (CAP-029+CAP-010), BC-4.09.001 (CAP-009+CAP-029) — three E-10 BCs and likely others outside E-10. Recommend either schema extension (`secondary_capabilities:` array) or BC-INDEX rendering policy update or template-level documentation that "primary in index, secondary in body only is the convention."
- **[process-gap] F-7 pattern:** Multiple BCs incremented version frontmatter without adding `## Changelog` section. Suggests BC template enforcement is missing — neither pre-merge hooks nor adversarial passes prior to this one caught the systematic gap. Recommend adding a hook/check that every `version:` increment must produce a changelog row.
- **[process-gap] D-318 follow-up gap:** "Closed in D-318" markers (e.g., BC-1.12.007 line 126) need a clearer convention for "tool selected but implementation deferred" vs full closure. The current binary "closed/deferred" verbs are insufficient.

## Novelty Assessment

**Novelty: MODERATE-HIGH** — findings are substantive and not nitpicks. F-1 (default-members type/crate confusion in BC-1.12.001) is a HIGH semantic defect that prior passes missed because they focused on BC-1.12.007 (which has the correct distinction). F-2 and F-6 are POLICY 8 reverse-direction gaps that the D-319 sweep partially closed but not for BC-INDEX (F-2) or BC-1.11.003 (F-6). F-3 (S-10.05 four-state in body vs BC-1.12.009 five-state in H1) is a cross-document partial-fix regression that the D-320 BC-table-row sweep missed. F-5 (BC-1.12.007 TD-015-a contradiction) is internal-consistency drift introduced by D-318's closure attempt. F-7 (no-changelog pattern across 5 BCs) is a systematic gap.

These are NOT refinements or rewordings. The spec package has NOT converged — partial-fix regression discipline (S-7.01) is the dominant defect class and a sustained HIGH-severity flag should remain until the propagation sweeps are completed in a single coherent burst rather than piecemeal across D-318/D-319/D-320/D-321. **Convergence counter remains 0.**

Recommend: a single fix burst that (a) re-anchors BC-1.11.002 (F-4), (b) updates BC-INDEX lines 166–168 (F-2, F-4, F-6), (c) sweeps S-10.05 body for "four-state"→"five-state" (F-3), (d) corrects BC-1.12.001 Postcondition 2 + Invariant 3 default-members language (F-1), (e) reconciles BC-1.12.007 TD-015-a closure scope (F-5), (f) adds Changelog sections to 5 affected BCs (F-7), (g) updates E-10 epic subsystems_affected (F-9). Then pass 4.
