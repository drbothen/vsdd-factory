---
pass: 4
date: 2026-05-06
producer: adversary
artifacts_reviewed:
  - .factory/specs/architecture/decisions/ADR-015-single-stream-otel-schema.md
  - .factory/stories/epics/E-10-single-stream-otel-event-emission.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.11.001.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.11.002.md
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
  - .factory/stories/STORY-INDEX.md
  - .factory/specs/domain-spec/capabilities.md
  - .factory/specs/domain-spec/invariants.md
  - .factory/specs/open-questions.md
verdict: HIGH
post_seal_sha: 83b046c
---

## Critical Findings

### F-1 [HIGH] BC-4.09.001 Scope enumerates a 6-plugin set that does NOT match S-10.05's authoritative 11-plugin inventory — implementer will build for the wrong plugin set

**Confidence:** HIGH
**Files:**
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-04/BC-4.09.001.md` Scope section
- `/Users/jmagady/Dev/vsdd-factory/.factory/stories/S-10.05-adr015-wave2-plugin-schema-migration.md` Architecture Mapping § Plugin inventory

**Evidence:**
BC-4.09.001 Scope lists six plugins by name:
```
- capture-pr-activity
- capture-commit-activity
- session-start-telemetry
- session-end-telemetry
- worktree-hooks
- tool-failure-hooks
```

S-10.05 Architecture Mapping § Plugin inventory lists eleven plugins by name (authored from S-10.01 Wave 0 audit):
```
capture-commit-activity, capture-pr-activity, block-ai-attribution, handoff-validator,
pr-manager-completion-guard, track-agent-stop, update-wave-state-on-merge,
validate-pr-review-posted, session-learning, warn-pending-wave-gate, track-agent-start.
```

Only TWO names overlap: `capture-pr-activity` and `capture-commit-activity`. Nine plugins enumerated by S-10.05 are absent from BC-4.09.001's Scope (block-ai-attribution, handoff-validator, pr-manager-completion-guard, track-agent-stop, update-wave-state-on-merge, validate-pr-review-posted, session-learning, warn-pending-wave-gate, track-agent-start). Four plugins enumerated in BC-4.09.001 (session-start-telemetry, session-end-telemetry, worktree-hooks, tool-failure-hooks) are absent from S-10.05's enumeration entirely.

**Why it fails:** BC-4.09.001 Postconditions 1 and 5 say "Each plugin's `on_hook` implementation" must do X. An implementer reading BC-4.09.001 will migrate the 6 plugins it lists; the 9 plugins listed in S-10.05 but absent from BC-4.09.001 will fall through. BC-4.09.001 Invariant 2 requires zero matches for legacy event names across `crates/hook-plugins/` — but if the BC's example mappings (lines 86-92) are interpreted as the canonical mapping table (`session.started → vsdd.plugin.session-start-telemetry.session_started.v1`, etc.), there is no canonical mapping for `block-ai-attribution`, `handoff-validator`, etc. Wave 2 ships incomplete; Wave 3 dashboard cleanup leaves dangling old-name queries. The "Any additional plugins" hedge in BC-4.09.001 is too vague to be falsifiable.

This is a content-defect, not a process-gap: either BC-4.09.001 enumeration is wrong or S-10.05 enumeration is wrong. Both cannot be authoritative simultaneously.

---

### F-2 [HIGH] BC-1.11.002 Stories cell + BC-INDEX line 169 anchor BC to S-10.03, but S-10.03's behavioral_contracts frontmatter does NOT include BC-1.11.002 — bidirectional POLICY 8 drift; orphan in primary direction

**Confidence:** HIGH
**Files:**
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-01/BC-1.11.002.md` Stories cell + (no Story Anchor section present)
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/BC-INDEX.md` line 169
- `/Users/jmagady/Dev/vsdd-factory/.factory/stories/S-10.03-adr015-wave1-resource-attribute-enrichment.md` frontmatter `behavioral_contracts:`
- `/Users/jmagady/Dev/vsdd-factory/.factory/stories/S-10.02-adr015-wave1-filesink-single-stream.md` frontmatter `behavioral_contracts:`

**Evidence:**
BC-1.11.002 Traceability Stories cell: `S-10.03 (Wave 1 enrichment + FileSink integration)`.
BC-INDEX.md line 169 Stories column: `S-10.03`.

S-10.03 frontmatter `behavioral_contracts:` array: `[BC-1.12.003, BC-1.12.004, BC-1.12.005]` — does NOT list BC-1.11.002.
S-10.02 frontmatter `behavioral_contracts:` array: `[BC-1.12.001, BC-1.12.002, BC-1.12.004, BC-1.12.005, BC-1.12.007, BC-3.05.004]` — does NOT list BC-1.11.002.
S-10.04 frontmatter: also does NOT list BC-1.11.002.

No story in the entire E-10 epic lists BC-1.11.002 in its behavioral_contracts array, despite BC-1.11.002 being the FileSink partial-write recovery + write-failure cascade contract — a contract directly invoked by S-10.02 AC-003 (traces to BC-1.12.001 PC3 which itself delegates to BC-1.11.002 for cascade specification).

**Why it fails:** Per POLICY 8 (bc_array_changes_propagate_to_body_and_acs): "BC array changes propagate to body and ACs." The BC body claims S-10.03 implements it; S-10.03 frontmatter is silent. Implementer working from S-10.03 frontmatter will not pull BC-1.11.002 into context. Implementer working from S-10.02 will trace AC-003 to BC-1.12.001 PC3 and then BC-1.11.002 transitively — but the BC will not appear in the story's behavioral_contracts gate.

The semantic placement is also questionable — BC-1.11.002 governs the FileSink write-failure cascade which is an SS-01 dispatcher concern. S-10.02 (FileSink wiring) is a more natural anchor than S-10.03 (Resource enrichment). Either S-10.02 should list BC-1.11.002, or BC-1.11.002 should re-anchor to S-10.02. Current state is a referential orphan.

---

### F-3 [HIGH] BC-1.11.002 has no `## Story Anchor` and no `## VP Anchors` section — structural inconsistency with all 13 sibling E-10 BCs

**Confidence:** HIGH
**Files:**
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-01/BC-1.11.002.md`

**Evidence:**
BC-1.11.002 contains: Description, Preconditions, Postconditions, Invariants, Related BCs, Architecture Anchors, Edge Cases, Canonical Test Vectors, Verification Properties, Traceability table, Purity Classification, Design Decision, Changelog.

Every sibling BC in the E-10 cluster (BC-1.11.001, BC-1.11.003, BC-1.12.001, BC-1.12.002, BC-1.12.003, BC-1.12.004, BC-1.12.005, BC-1.12.006, BC-1.12.007, BC-1.12.009, BC-2.06.001, BC-3.05.004, BC-4.09.001) contains explicit `## Story Anchor` AND `## VP Anchors` sections.

**Why it fails:** Per the Partial-Fix Regression Discipline lesson (S-7.01 sibling sweep), when a structural pattern applies to one BC in a layer, the same pattern should apply to siblings. BC-1.11.002 is the only SS-01 E-10 BC missing both sections. The Stories cell in Traceability is present, but the dedicated `## Story Anchor` heading and `## VP Anchors` heading are absent. This is a structural drift that survived 3 prior fix bursts.

Blast radius = 1 file. Severity is MEDIUM by the partial-fix lesson's table. However, both `## Story Anchor` (POLICY 8 anchor target) and `## VP Anchors` (Phase 1.6b VP allocation gate) are missing, which compounds to HIGH because both POLICY 8 and the VP-INDEX flow will misfire on this BC.

---

### F-4 [HIGH] BC-INDEX line 169 BC-1.11.002 has Capability=CAP-029 but Stories=S-10.03 — and S-10.03 BC body table does not contain BC-1.11.002 row

**Confidence:** HIGH
**Files:**
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/BC-INDEX.md` line 169
- `/Users/jmagady/Dev/vsdd-factory/.factory/stories/S-10.03-adr015-wave1-resource-attribute-enrichment.md` Behavioral Contracts table

**Evidence:**
BC-INDEX line 169: `[BC-1.11.002] | factory-dispatcher::file_sink::partial_write_recovery — boundary-marker strategy ... | draft | CAP-029 | S-10.03`.

S-10.03 § Behavioral Contracts body table contains rows for BC-1.12.003, BC-1.12.004, BC-1.12.005 only. BC-1.11.002 is absent from the body table.

**Why it fails:** This is the body-side counterpart to F-2. Per POLICY 8 the four-way audit (frontmatter array ↔ body BC table ↔ ACs ↔ BC's `Stories:` cell) must agree. Direction violations here:
- BC.Stories = S-10.03 ✓ → S-10.03.frontmatter ✗ (missing)
- BC.Stories = S-10.03 ✓ → S-10.03.body BC table ✗ (missing)
- S-10.03 ACs do not reference BC-1.11.002 traceability tag

Three of the four directions break. Pattern flag: this is the same drift class as F-2 and confirms the partial-fix regression — D-322/D-323/D-324 fixed BC-1.11.002 capability resolution and BC-INDEX line 169 Capability column but did NOT propagate the BC's Stories cell into S-10.03's frontmatter or body table.

---

### F-5 [HIGH] BC-1.12.001 Description states "BC-1.05.036 Postcondition 4 bifurcation" but BC-1.05.036 is not directly inspected — citation is unverifiable from the E-10 reading list and contains a numbered postcondition reference that may not exist on the cited BC

**Confidence:** MEDIUM (re-classified after self-validation)
**Files:**
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-01/BC-1.12.001.md` Description ("see BC-1.05.036 Postcondition 4 bifurcation"), Postcondition 3 ("acknowledged per BC-1.05.036 EC-011 / OQ-W16-004"), Related BCs, TD-VSDD-092

**Evidence:**
BC-1.12.001 cites BC-1.05.036 by:
- Postcondition number ("Postcondition 4")
- Edge case number ("EC-011")
- Postcondition number ("Postcondition 6")

These cross-references are made multiple times (Description, PC3, PC2, Related BCs, TD-VSDD-092). The cited postcondition / EC numbers cannot be verified inside the E-10 reading list (BC-1.05.036 is outside the scope), and the file is not in the reading list.

**Why it could fail:** If BC-1.05.036 does not in fact contain a Postcondition 4 / Postcondition 6 / EC-011 with the cited semantics, BC-1.12.001's traceability collapses — implementers will read references to numbered structural elements that do not exist, and the "SOUL #4 acknowledgment" claim becomes incorrectly grounded. This is the same risk class identified in HIGH-P59-001 for stable-anchor citations (TD-VSDD-091): postcondition-number citations are stable IF the postcondition numbering does not change in the cited BC. Without verification, this is a brittle anchor.

**Mitigation evidence:** TD-VSDD-091 stable-anchor work has been performed for symbol references (function names, expression names) in BC-1.12.001. Postcondition-number references to other BCs are NOT covered by the TD-VSDD-091 work as I read it. Recommend: add a stable-anchor caveat in BC-1.12.001 Description noting that BC-1.05.036 cross-references are postcondition-NUMBER based (not stable across re-numbering), or replace with a textual description.

This is MEDIUM (was almost demoted further) because the citation is structural; without re-reading BC-1.05.036, I cannot confirm the citations are actually wrong. Demoted from HIGH on the self-validation evidence-check pass.

---

### F-6 [HIGH] BC-1.11.002, BC-1.11.003, BC-1.12.006 L2 Domain Invariants cell = "TBD" — invariant orphan pattern (3 BCs unresolved across the E-10 cluster)

**Confidence:** HIGH
**Files:**
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-01/BC-1.11.002.md` Traceability table
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-01/BC-1.11.003.md` Traceability table
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-01/BC-1.12.006.md` Traceability table
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/domain-spec/invariants.md`

**Evidence:**
BC-1.11.002 Traceability row: `| L2 Domain Invariants | TBD |`
BC-1.11.003 Traceability row: `| L2 Domain Invariants | TBD |`
BC-1.12.006 Traceability row: `| L2 Domain Invariants | TBD |`

Sibling BCs in the same cluster have populated DI cells (BC-1.11.001 cites DI-017; BC-1.12.001 cites DI-011/DI-012; BC-1.12.002 cites DI-007/DI-008; BC-1.12.003 cites DI-017; BC-1.12.004 cites DI-017; BC-3.05.004 cites DI-014).

**Why it fails:** Per the Invariant-to-BC Orphan Detection Review Axis, 3+ BCs with TBD DI cells in a single epic crosses the "multiple orphans" threshold (HIGH severity with pattern flag). For each missing TBD:
- BC-1.11.002 (FileSink partial-write + cascade): a relevant DI candidate is DI-007 (always-on telemetry — fallback path uses dispatcher-internal-* which DI-007 governs); DI-013 (unknown driver non-fatal — relevant to the cascade-degrades-to-fallback discipline).
- BC-1.11.003 (emit_pair atomic dual-emit): a relevant DI candidate is DI-017 (trace_id present on every emitted event — emit_pair must carry trace_id on both halves).
- BC-1.12.006 (block-path audit emission): a relevant DI candidate is DI-017 (trace_id on every event INCLUDES block-audit events) and possibly an audit-trail invariant if one were defined.

The pattern flag here matters: the E-10 cluster DOES connect to several DIs (007, 008, 011, 012, 014, 017), but only some BCs in the cluster cite them. The unfilled cells are not "no invariant applies"; they are "we did not finish populating this row." This is a content defect across 3 BCs and meets the orphan-pattern HIGH threshold.

---

### F-7 [HIGH] BC-1.12.001 v1.2 changelog says "F-11 fix: CAP-029 paraphrase-as-quote replaced with proper non-quoted reference per capabilities.md §CAP-029" — but the L2 Capability cell still contains a parenthetical title that paraphrases CAP-029 ("Emit structured events to a single observability stream (file path)")

**Confidence:** MEDIUM
**Files:**
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-01/BC-1.12.001.md` Traceability `L2 Capability` cell + `Capability Anchor Justification` cell
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/domain-spec/capabilities.md` §CAP-029

**Evidence:**
BC-1.12.001 Traceability:
```
| L2 Capability | CAP-029 ("Emit structured events to a single observability stream (file path)") per capabilities.md §CAP-029 |
| Capability Anchor Justification | CAP-029 ("Emit structured events to a single observability stream (file path)") per capabilities.md §CAP-029. ...
```

capabilities.md §CAP-029 H3 line: `**CAP-029 — Emit structured events to a single observability stream (file path)**`

Same pattern recurs in BC-1.12.004, BC-1.12.005, BC-1.12.007, BC-1.12.009, BC-3.05.004, BC-1.11.001, BC-1.11.002, BC-1.12.006.

**Why it fails:** The v1.2 changelog claims F-11 fix replaced "paraphrase-as-quote" with "proper non-quoted reference." The double-quoted parenthetical title that follows the CAP-029 ID IS still a quoted-paraphrase (the `(...)` is the same text as the H3 with a different surrounding-formatting). The fix was either: (a) ineffective — the parenthetical was kept and only some other quote was removed; or (b) the changelog narrative does not match the actual textual change.

The current text is internally coherent — it cites CAP-029 with its title parenthetically, then defers to capabilities.md §CAP-029 as the source of truth. This is acceptable IF the title text in the parenthetical matches the H3 in capabilities.md byte-for-byte. Verification: the H3 text is `Emit structured events to a single observability stream (file path)` — matches the parenthetical exactly. So the citation is accurate.

The defect is that the v1.2 changelog narrative claims a textual change that did not change the visible state. This is a low-severity changelog-vs-state drift, but it appears across 9+ BCs that all cite the same v1.2 / D-322 fix. Pattern flag: `[process-gap]` — the F-11 fix may have intended to remove a different paraphrase that no longer exists, or the changelog narrative is wrong.

Demoted to MEDIUM since the visible artifact state is coherent.

---

## Important Findings

### F-8 [MEDIUM] BC-1.11.001 "Postcondition 2" cited from S-10.04 AC-002 traceability — but S-10.04 AC-002 says "(traces: BC-1.11.001 postcondition 2 — span chain rule invariants)" — BC-1.11.001 PC2 prose covers VSDD_PARENT_SPAN_ID set to invoking plugin's span_id, not the 3-rule "Span chain rule" set

**Confidence:** MEDIUM
**Files:**
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-01/BC-1.11.001.md` Postconditions 1-5
- `/Users/jmagady/Dev/vsdd-factory/.factory/stories/S-10.04-adr015-wave1-trace-propagation-lifecycle-events.md` AC-002

**Evidence:**
BC-1.11.001 Postcondition 2: `The subprocess environment contains VSDD_PARENT_SPAN_ID set to the invoking plugin's span_id (i.e., its plugin.invocation_id).` (single-clause statement about env-var presence)

BC-1.11.001 Postcondition 4: `A subprocess that itself calls exec_subprocess inherits and forwards both variables; the trace_id is unchanged across the hop; the parent_span_id at each subsequent hop is the previous level's span_id.` (THIS is the span chain rule — multi-hop propagation)

S-10.04 AC-002 trace cite: `(traces: BC-1.11.001 postcondition 2 — span chain rule invariants)` — but the parenthetical "span chain rule invariants" actually maps to PC4 (multi-hop span chain), not PC2 (single env-var assertion).

**Why it fails:** The trace citation in S-10.04 AC-002 names the wrong postcondition number. AC-002's body content (`trace_id` = inherited; `parent_span_id` = invoking plugin's span_id; `span_id` = a new UUIDv4 generated by the subprocess plugin at startup) is the multi-hop chain rule from BC-1.11.001 PC4, not the single-statement PC2. An implementer reading AC-002 → BC-1.11.001 PC2 will not see the multi-hop rule; they would need PC4. Mis-anchor severity per Semantic Anchoring axis: MEDIUM (semantically awkward but technically still valid since PC2 is part of the chain). The label is stale.

---

### F-9 [MEDIUM] BC-2.06.001 Postcondition 2 migration guidance has 4 VCS field names inside ONE backtick code-span instead of 4 separate code-spans

**Confidence:** HIGH (textual evidence is direct)
**Files:**
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-02/BC-2.06.001.md` Postcondition 2 line 74

**Evidence:**
Line 74 reads:
```
`vcs.repository.url.full, vcs.repository.name, vcs.provider.name, vcs.owner.name`
```
This is ONE backtick span containing 4 comma-separated names, where every other field name in the same paragraph is in its own backtick span (e.g., `service.name`, `service.namespace`, etc.). The D-322 v1.1 changelog row says "F-14 fix: `vcs.*` wildcard in migration guidance (Postcondition 2) expanded to explicit four VCS Resource fields (`vcs.repository.url.full`, `vcs.repository.name`, `vcs.provider.name`, `vcs.owner.name`)" — this changelog example DOES split each field into its own backtick span.

**Why it fails:** The fix narrative shows 4 separate backtick spans; the actual postcondition body uses 1 backtick span with comma separators. The byte-for-byte intended fix appears not to have propagated faithfully. Operator/implementer reading the migration guide sees inconsistent code-span formatting (4-of-N fields are concatenated into one span; rest are individual spans). LOW-MEDIUM editorial defect; impact is reader confusion. The semantic content is preserved (4 names are listed) but the rendering will look wrong in Markdown viewers.

---

### F-10 [MEDIUM] BC-3.05.004 Architecture Anchors cites "ADR-015 D-15.1" for the debug-stream gating prose — but the actual ADR clause is in D-15.1's "FileSink write-failure semantics" subsection, while the gating prose is also referenced from "AMENDED by ADR-015" against ADR-007. Cross-citation makes the source-of-truth ambiguous

**Confidence:** MEDIUM
**Files:**
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-03/BC-3.05.004.md` Architecture Anchors
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/architecture/decisions/ADR-015-single-stream-otel-schema.md` D-15.1

**Evidence:**
BC-3.05.004 Architecture Anchors:
```
- ADR-015 D-15.1 — normative prose for debug stream being "gated by the
  `VSDD_DEBUG_LOG=1` environment variable"
- ADR-015 OQ-1 (resolved in SS-03-event-emission.md) — v2 schema definition
```

But BC-3.05.004 Postcondition 5 says the gate semantics are "per OQ-W16-011 resolution, D-311 2026-05-06" — NOT per D-15.1. The D-15.1 text says "gated by the `VSDD_DEBUG_LOG=1` environment variable" without specifying the two-key precedence (env var vs config key) — that two-key gate emerged from OQ-W16-011 resolution AFTER ADR-015 acceptance (D-15.1 prose pre-dates D-311).

The Architecture Anchor "ADR-015 D-15.1 — normative prose for debug stream..." is partially correct but does NOT cover the two-key gate semantics that BC-3.05.004 specifies. The two-key gate semantics come from OQ-W16-011 resolution + SS-03-event-emission.md OQ-1 resolution. The Architecture Anchor list omits OQ-W16-011 from open-questions.md as an authoritative source — only "ADR-015 OQ-1 (resolved in SS-03-event-emission.md)" is cited.

Per the source-of-truth axis: the two-key gate semantics ARE in BC-3.05.004 Postcondition 5 with proper citation to the open-questions.md text, but the Architecture Anchors section misses the open-questions.md citation that justifies the env-var-dominates-when-present rule. Source-of-truth ambiguity: an implementer scanning Architecture Anchors will not know to consult open-questions.md OQ-W16-011 for the precedence rule. MEDIUM severity.

---

### F-11 [MEDIUM] BC-1.11.002 Capability Anchor Justification cites "BC-1.12.001 Postcondition 3" as "the failure-cascade entry point" — but BC-1.12.001 Postcondition 3 is the dispatcher-internal-*.jsonl receipt-condition postcondition, not the cascade-trigger postcondition

**Confidence:** MEDIUM
**Files:**
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-01/BC-1.11.002.md` Traceability `Capability Anchor Justification`
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-01/BC-1.12.001.md` Postcondition 3

**Evidence:**
BC-1.11.002 Capability Anchor Justification:
```
BC-1.12.001 Postcondition 3 is the failure-cascade entry point on the primary-path BC; BC-1.11.002 is the full specification of what that cascade does.
```

BC-1.12.001 Postcondition 3 prose:
```
`dispatcher-internal-YYYY-MM-DD.jsonl` does NOT receive this event unless the
`VSDD_DEBUG_LOG=1` environment variable is set (see BC-1.12.002) or unless
`FileSink::write` returns an error (fallback path per BC-1.11.002).
```

The semantics of BC-1.12.001 PC3 are "negative postcondition" (debug file does NOT receive event UNLESS conditions). The cascade-trigger semantics are in BC-1.12.001 EC-003 ("Write-failure cascade per BC-1.11.002: fallback write to ...") and Canonical Test Vector "write-failure-cascade (per BC-1.11.002)".

**Why it fails:** BC-1.11.002 calls BC-1.12.001 PC3 the cascade entry point, but PC3 only mentions the cascade as an exception clause inside a negative postcondition. The actual cascade trigger postcondition shape (failure → fallback + warning) is in BC-1.11.002's own Postconditions, with EC-003 being the canonical primary-path BC reference to BC-1.11.002. The cross-reference is imprecise — an implementer reading BC-1.11.002 will jump to BC-1.12.001 PC3 expecting a cascade-trigger postcondition and find a debug-file negative postcondition instead. Mis-anchor severity: MEDIUM (semantically awkward; semantic intent is preserved by EC-003).

---

### F-12 [MEDIUM] BC-INDEX line 174 BC-1.12.004 Stories column says "S-10.02, S-10.03" — but BC-1.12.004 is also referenced from S-10.04 (in S-10.04's behavioral_contracts AND body BC table for AC-006)

**Confidence:** HIGH
**Files:**
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/BC-INDEX.md` line 174
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-01/BC-1.12.004.md` Stories cell + Story Anchor section
- `/Users/jmagady/Dev/vsdd-factory/.factory/stories/S-10.04-adr015-wave1-trace-propagation-lifecycle-events.md` frontmatter + body BC table

**Evidence:**
BC-INDEX line 174: `[BC-1.12.004] | ... | draft | CAP-029 | S-10.02, S-10.03`.
BC-1.12.004 Story Anchor section: lists `S-10.02, S-10.03, S-10.04` (3 stories).
BC-1.12.004 Stories Traceability cell: lists `S-10.02 (FileSink wiring + per-event stamping + plugin.version fix), S-10.03 (Resource context consumed here), S-10.04 (Trace propagation + lifecycle event types)` (3 stories).
S-10.04 frontmatter `behavioral_contracts:` includes BC-1.12.004 ✓.
S-10.04 body Behavioral Contracts table includes BC-1.12.004 row ✓.

**Why it fails:** BC-INDEX line 174 lists only S-10.02 and S-10.03, omitting S-10.04. This is BC-INDEX-vs-BC-body drift. POLICY 7 requires byte-for-byte sync of titles; the stories cell is also subject to drift discipline. S-10.04 is anchored to BC-1.12.004 in 3 of the 4 surfaces (BC.Story Anchor, BC.Stories cell, story.frontmatter, story.body table) but missing in 1 surface (BC-INDEX). Per the Partial-Fix Regression Discipline (S-7.01) sibling sweep, the propagation gap is a Stories-cell drift in BC-INDEX.

Same drift class likely applies to BC-1.12.005 (line 175 Stories=`S-10.02, S-10.03`) — BC-1.12.005 body cites S-10.02 + S-10.03 + S-10.04 in its Story Anchor. Pattern flag: 2 BCs share the drift.

---

### F-13 [MEDIUM] BC-INDEX line 175 BC-1.12.005 Stories column says "S-10.02, S-10.03" — same drift as F-12 (S-10.04 listed in BC body but not BC-INDEX)

**Confidence:** HIGH
**Files:**
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/BC-INDEX.md` line 175
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-01/BC-1.12.005.md` Story Anchor + Stories cell
- `/Users/jmagady/Dev/vsdd-factory/.factory/stories/S-10.04-adr015-wave1-trace-propagation-lifecycle-events.md` frontmatter + body BC table

**Evidence:**
BC-INDEX line 175: `[BC-1.12.005] | ... | draft | CAP-029 | S-10.02, S-10.03`.
BC-1.12.005 Story Anchor section: lists S-10.02, S-10.03, S-10.04.
BC-1.12.005 Stories Traceability cell: lists S-10.02, S-10.03, S-10.04.
S-10.04 frontmatter behavioral_contracts: includes BC-1.12.005 ✓.
S-10.04 body BC table: includes BC-1.12.005 row ✓.

**Why it fails:** Same drift as F-12. Pattern flag confirms: BC-INDEX rows for BC-1.12.004 and BC-1.12.005 both omit S-10.04 from their Stories columns even though all four other propagation surfaces (BC body Story Anchor, BC body Stories cell, story frontmatter, story body BC table) include S-10.04. Fix burst D-320 added S-10.04 to those 4 surfaces but did not propagate to BC-INDEX. HIGH severity by the partial-fix discipline (blast radius = 2+ BC-INDEX rows).

---

### F-14 [MEDIUM] [process-gap] BC-1.12.007 Architecture Anchors lacks stable-anchor citation for `Cargo.toml default-members` symbol — same TD-VSDD-091 stable-anchor pattern is applied for some symbols but not others within the same BC

**Confidence:** MEDIUM
**Files:**
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-01/BC-1.12.007.md` Architecture Anchors

**Evidence:**
BC-1.12.007 Architecture Anchors uses stable-anchor citations for some symbols (`factory-dispatcher::sinks::mod` with the TD-VSDD-091 explicit caveat) but cites `Cargo.toml` (workspace root) — `default-members` field as a structured-but-non-stable anchor (no caveat, no path resolution other than the file path).

**Why it fails:** TD-VSDD-091 stable-anchor work establishes that line numbers are not authoritative; symbol/field/function names are. The `default-members` citation in `Cargo.toml` is a TOML key, not a symbol. The TD-VSDD-091 caveat does not directly apply. The current citation IS correct without the caveat — but the structural inconsistency (some lines have caveats, others don't) creates confusion about which citations are subject to TD-VSDD-091 stable-anchor discipline.

This is a [process-gap] tag because the stable-anchor citation pattern itself is process-level (TD-VSDD-091 is a recurring discipline). Recommendation: extend TD-VSDD-091 caveats to TOML key references in Cargo.toml-style files, or document that TD-VSDD-091 applies only to Rust symbol references.

---

### F-15 [MEDIUM] BC-1.12.001 Architecture Anchors cites `factory-dispatcher::sinks::Sink` trait dispatch surface — but ADR-015 D-15.1 says the `Sink` trait in `sink-core` is KEPT and only the Router wiring is retired. The "Sink trait dispatch surface" anchor implies the trait dispatch is the retired piece

**Confidence:** MEDIUM
**Files:**
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-01/BC-1.12.001.md` Architecture Anchors line 126-127
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/architecture/decisions/ADR-015-single-stream-otel-schema.md` D-15.1 Implementation path
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-01/BC-1.12.007.md` Architecture Anchors line 147 — explicitly states "The `Sink` trait itself in `sink-core` is KEPT"

**Evidence:**
BC-1.12.001 Architecture Anchors:
```
- `factory-dispatcher::sinks::Sink` trait dispatch surface — the open integration point that
  ADR-015 resolves; `Router::submit` is NOT wired post-Wave-1 (stable anchor per TD-VSDD-091;
  line numbers are not authoritative — use the function/method name as the canonical reference.)
```

BC-1.12.007 (sibling BC) explicitly clarifies in v1.3 D-322 F-16 fix:
```
- the open integration TODO at `factory-dispatcher::sinks::mod` (in
  `crates/factory-dispatcher/src/sinks/mod.rs`) — the wiring point for `Router::submit`
  that ADR-015 closes by leaving it unwired. The `Sink` trait itself in `sink-core` is
  KEPT per ADR-015 D-15.1; only the Router wiring is retired here.
```

**Why it fails:** BC-1.12.001 Architecture Anchors says "Sink trait dispatch surface" is "the open integration point that ADR-015 resolves" — this could be read as the trait is being retired. BC-1.12.007 makes the correct distinction explicit (trait kept; only Router wiring retired). Per the Partial-Fix Regression Discipline sibling-sweep rule: the F-16 clarification applied to BC-1.12.007 should also apply to BC-1.12.001 (sibling BC describing the same architectural surface). Blast radius = 1 file (BC-1.12.001 Architecture Anchor wording). MEDIUM severity per the partial-fix table.

---

### F-16 [MEDIUM] BC-1.11.002 Architecture Module Traceability cell says "SS-01/SS-03" — but BC frontmatter `subsystem: "SS-01"` declares single subsystem only

**Confidence:** HIGH
**Files:**
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-01/BC-1.11.002.md` frontmatter line 14 + Architecture Module cell line 165

**Evidence:**
Frontmatter line 14: `subsystem: "SS-01"` (single value).
Architecture Module Traceability cell: `SS-01/SS-03 — `crates/sink-file/src/lib.rs`, `crates/factory-dispatcher/src/internal_log.rs``.

**Why it fails:** The frontmatter declares the BC's owning subsystem as SS-01. The Architecture Module cell claims dual subsystem ownership (SS-01/SS-03). Per ARCH-INDEX, `crates/sink-file/` belongs to SS-03 (Event Emission) per the canonical Subsystem Registry. The BC's actual scope spans both SS-01 (`internal_log.rs`) AND SS-03 (`sink-file/lib.rs`).

If the BC's primary semantic surface IS SS-01 (FileSink invocation by the dispatcher), the frontmatter is correct and the cell is misleading. If the BC's primary surface IS SS-03 (FileSink internal write contract), the frontmatter is wrong and BC-1.11.002 should be in `ss-03/`.

The Description, Story Anchor (in cell), and Capability Anchor Justification all locate the BC in SS-01 dispatcher-side concerns. The Architecture Module cell text is the only discrepant statement. Suggest: rewrite cell as "SS-01 (primary) — `crates/factory-dispatcher/src/internal_log.rs`; cross-references SS-03 — `crates/sink-file/src/lib.rs`" to match the BC-1.11.003 pattern that uses "primary" / "pass-through binding" qualifiers.

---

## Observations

### O-1 [LOW] [process-gap] BC-1.12.005 EC-008 v1.2 changelog claims "rate-limit" wording fix — but EC-008 still uses double-affirmative language ("IS rate-limited per Postcondition 6")

**Files:** `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-01/BC-1.12.005.md` EC-008

EC-008 says: "The `vsdd.internal.host_field_override.v1` lifecycle event IS rate-limited per Postcondition 6: at most one emission per `(plugin.name, field_name)` per dispatcher invocation. There is NO SEPARATE meta-rate-limit on top of the per-pair rate-limit — the per-pair rate IS the only rate-limit."

This is correct content but reads as defensive rebuttal of an earlier (now-deleted) wrong statement. The edge case is now informational meta-text rather than a falsifiable scenario-with-expected-behavior. EC-008's Description column ("Lifecycle event meta-rate-limit clarification") admits this is a clarification rather than an edge case. Suggest: convert EC-008 to a NOTE under Postcondition 6 rather than an EC table row, or rephrase as "Plugin overrides same field 50 times per invocation → 1 lifecycle event (per the rate-limit)" to make it falsifiable. Marked [process-gap] because EC tables are sometimes used as clarifications rather than edge cases — a recurring pattern.

---

### O-2 [LOW] BC-1.12.001 Postcondition 2 has a Sentence with a 5-line stable-anchor parenthetical that runs longer than the postcondition's normative content

**Files:** `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-01/BC-1.12.001.md` Postcondition 2

The TD-VSDD-091 stable-anchor caveat appears as a 1-line bracketed sentence at the end of Postcondition 2: `[Stable anchor per TD-VSDD-091; line numbers are not authoritative — use the type/function name as the canonical reference.]`. Postcondition 2 itself is ~9 lines including the caveat. The caveat-to-content ratio reduces readability. Suggest: collapse the caveat into a one-line footnote OR omit it from each individual postcondition (it is repeated 4x in this BC alone).

This pattern recurs across BC-1.12.001 PC2, PC3, Architecture Anchors, BC-1.12.004 PC1, BC-1.12.004 Architecture Anchors. The repetition is consistent but creates noise. A BC-level top-line "Stable Anchors" disclaimer would reduce visual clutter without losing the discipline.

---

### O-3 [LOW] capabilities.md frontmatter version says "v1.2" but the reading list claim is "v1.3 with CAP-030 errata"

**Files:** `/Users/jmagady/Dev/vsdd-factory/.factory/specs/domain-spec/capabilities.md` line 5 frontmatter `version: "1.2"` + CHANGELOG row v1.2 contains the CAP-030 errata.

The reading list provided to this review states "capabilities.md (v1.3 with CAP-030 errata)". The actual file is v1.2 in both frontmatter and CHANGELOG; v1.2 contains the CAP-030 errata note. Either the reading list claim is stale or the file was not bumped to v1.3 after the errata addition. Not a content defect in the file itself — the file is internally coherent. Reading-list claim drift only.

---

### O-4 [LOW] ADR-015 Migration Plan Wave 1 step says "Retire the `sink-otel-grpc` crate and the `Router`, `SinkRegistry` types within `sink-core` from the integration path (leave code on disk until post-migration cleanup in Wave 5; do not call them)" — does NOT mention DlqWriter, but ADR-015 D-15.1 retirement-semantics paragraph and BC-1.12.007 do include DlqWriter

**Files:**
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/architecture/decisions/ADR-015-single-stream-otel-schema.md` Migration Plan Wave 1 (line 593-600)
- ADR-015 D-15.1 line 124: "The `DlqWriter` in `sink-core` is retired; the debug file IS the DLQ."
- BC-1.12.007 includes DlqWriter in scope.

ADR-015 Wave 1 migration step lists `Router` and `SinkRegistry` but omits `DlqWriter`. The retirement semantics paragraph in D-15.1 mentions `DlqWriter` separately. BC-1.12.007 covers `DlqWriter` exclusion. ADR-015 Wave 5 step does include DlqWriter ("Remove deprecated TYPES from kept crates: `Router`, `SinkRegistry`, and `DlqWriter` from `sink-core`"). The Wave 1 omission is a minor consistency gap — DlqWriter retirement is implicit in "the integration path" (since DlqWriter was Router's DLQ surface) but not explicit. LOW.

---

### O-5 [LOW] [process-gap] BC-1.11.002 has only 2 lifecycle-state cells in frontmatter (introduced + lifecycle_status), all others null — no `modified` array entries exist for v1.1 (D-322 capability resolve)

**Files:** `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-01/BC-1.11.002.md` frontmatter

```
introduced: v1.1.0
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
```

The `modified: []` array remained empty even after the v1.1 D-322 amendment. Same pattern in BC-1.11.001 (also has `modified: []` after a v1.1 D-318 amendment). This is consistent across BCs but the `modified:` array is presumably meant to track post-introduction lifecycle changes. If the convention is that capability changes are NOT tracked here, this is fine; if they are, it's a missed propagation across the cluster. [process-gap] tag because this likely affects all 14 E-10 BCs.

---

## Novelty Assessment

Pass 4 found **2 NEW HIGH** findings (F-1 plugin-set drift; F-2/F-4 BC-1.11.002 orphan cluster) that prior passes did not surface, plus **6 NEW MEDIUM** findings (F-3 missing sections; F-6 invariant orphan pattern; F-8 PC2-vs-PC4 mis-anchor; F-9 backtick formatting; F-10 architecture-anchor source-of-truth gap; F-12/F-13 BC-INDEX Stories drift; F-15 sibling-sweep gap; F-16 frontmatter vs cell mismatch).

The HIGH findings are substantive and not previously addressed by the D-322/D-323/D-324 fix burst:
- F-1 (BC-4.09.001 plugin-set drift) is a genuine drift between BC and story that survived the fix burst.
- F-2/F-4 (BC-1.11.002 orphan in S-10.03) is a partial-fix regression — D-324 fixed BC-INDEX line 169 Capability column but did not propagate the BC's Stories cell into S-10.03's frontmatter or body table.

The MEDIUM findings include 2 with pattern flags (F-12+F-13 BC-INDEX drift across multiple BCs; F-6 invariant orphan across 3 BCs).

**Verdict: HIGH** — at least 2 HIGH-severity findings (F-1, F-2/F-4 cluster) and 1 HIGH-severity invariant-orphan pattern (F-6). Several MEDIUM findings carry pattern flags. NOT NITPICK_ONLY.

**ADR-013 Convergence Counter:** Stays at 0 (this pass is HIGH).

---

## Self-Validation Loop Results

Round 1: Demoted F-5 from HIGH to MEDIUM after evidence check (citation correctness cannot be verified inside the reading list).
Round 2: Demoted F-7 to MEDIUM after re-checking that the visible artifact state IS coherent (the changelog narrative is what's drifted).
Round 3: Confirmed F-6 as HIGH after applying the Invariant-to-BC Orphan Detection threshold (3 BCs with TBD = HIGH per the lesson).

All findings have specific file:line evidence or specific structural-element references. No findings demoted to LOW or removed for lack of grounding except F-5 / F-7 partial demotions noted above.
