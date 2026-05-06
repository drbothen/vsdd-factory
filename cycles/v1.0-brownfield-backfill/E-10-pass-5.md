---
pass: 5
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
  - .factory/specs/domain-spec/capabilities.md
  - .factory/specs/domain-spec/invariants.md
  - .factory/specs/open-questions.md
  - .factory/stories/STORY-INDEX.md
verdict: HIGH
post_seal_sha: 4c1b003
engine_baseline: v1.0.0-rc.12 @ 4cf59bc
---

# Adversarial Review — Pass 5 (E-10 single-stream OTel)

> **NOTE — PARTIAL RECONSTRUCTION:** The verbatim adversary output was not available
> in the state-manager context window at archive time. This file contains the full
> finding inventory (titles, severities, all metadata) reconstructed from the
> orchestrator's dispatch record. The orchestrator should re-dispatch the adversary
> to produce the canonical full-text report if the finding bodies are needed for
> remediation. The finding titles and severities below are authoritative per the
> orchestrator's pass-5 summary.

## Summary

- **Verdict:** HIGH
- **Finding count:** 12 (3 HIGH, 9 MEDIUM/LOW)
- **Observations:** 5 (O-1 through O-5; O-3 and O-4 are [process-gap] candidates)
- **Convergence counter:** 0 (not advancing — HIGH verdict)
- **Trend:** pass-1 CRIT (22) → pass-2 CRIT (11) → pass-3 HIGH (16) → pass-4 HIGH (16) → pass-5 HIGH (12)

---

## Critical Findings

### F-1 [HIGH] BC-1.11.002 ↔ S-10.02 ↔ BC-INDEX three-way drift (POLICY 8 + Partial-Fix Regression)

**Confidence:** HIGH
**Files:**
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-01/BC-1.11.002.md`
- `/Users/jmagady/Dev/vsdd-factory/.factory/stories/S-10.02-adr015-wave1-filesink-single-stream.md`
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/BC-INDEX.md`

**Summary:** BC-1.11.002, S-10.02, and BC-INDEX have a three-way POLICY 8 drift. The
BC's Stories cell, the story's `behavioral_contracts` frontmatter array, and the BC-INDEX
Stories column are mutually inconsistent. This is a Partial-Fix Regression — a prior fix
burst updated one surface without propagating to the other two.

---

### F-2 [HIGH] BC-3.05.004 internally inconsistent: D-15.4 vs D-15.1/OQ-1

**Confidence:** HIGH
**Files:**
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-03/BC-3.05.004.md`
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/architecture/decisions/ADR-015-single-stream-otel-schema.md`

**Summary:** BC-3.05.004 cites D-15.4 as its normative architecture anchor for a
semantics clause that is actually specified in D-15.1 and OQ-1. The internal
cross-reference is wrong; the correct source-of-truth section is not D-15.4.

---

### F-3 [HIGH] S-10.04 AC-002 trace text not propagated after BC-1.11.001 PC restructure (Partial-Fix Regression)

**Confidence:** HIGH
**Files:**
- `/Users/jmagady/Dev/vsdd-factory/.factory/stories/S-10.04-adr015-wave1-trace-propagation-lifecycle-events.md`
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-01/BC-1.11.001.md`

**Summary:** A prior fix burst restructured BC-1.11.001's Postconditions. S-10.04 AC-002
contains a trace-text reference (e.g., `BC-1.11.001 PC2`) that was not updated to reflect
the restructure. The trace now points at the wrong postcondition number, causing an
implementer reading S-10.04 to pull the wrong postcondition into context.

---

## Important Findings

### F-4 [MEDIUM] Canonical block-message placeholder name inconsistent across BC-1.12.006 and BC-2.06.001

**Confidence:** HIGH
**Files:**
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-01/BC-1.12.006.md`
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-02/BC-2.06.001.md`

**Summary:** The placeholder name used for the canonical block-message field differs
between BC-1.12.006 and BC-2.06.001. Both BCs describe the same underlying concept
but use different identifier strings, creating implementer confusion about the canonical
name.

---

### F-5 [MEDIUM] BC-1.12.006 Postcondition 2 has no `reason` field, but post-rc.12 note describes the audit-event's "free-text description"

**Confidence:** HIGH
**Files:**
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-01/BC-1.12.006.md`

**Summary:** BC-1.12.006 Postcondition 2 specifies the audit-event schema but omits
a `reason` field. A post-rc.12 note elsewhere in the same BC or a sibling document
describes the audit-event as including a "free-text description" field. The postcondition
body and the note are inconsistent — either the note is wrong or Postcondition 2 is
incomplete.

---

### F-6 [MEDIUM] BC-INDEX status column for BC-3.05.001/002/003 says "retired" but BC-INDEX uses non-standard status alongside otherwise "draft" rows

**Confidence:** HIGH
**Files:**
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/BC-INDEX.md`

**Summary:** BC-INDEX rows for BC-3.05.001, BC-3.05.002, and BC-3.05.003 carry the
status "retired" — a non-standard value not used elsewhere in the index, where all
active BCs use "draft". The status vocabulary should be consistent. If these BCs are
retired, the status value must be defined in the BC-INDEX legend.

---

### F-7 [MEDIUM] `dispatcher_trace_id` references in architecture documents persist after DI-017 rename

**Confidence:** HIGH
**Files:** 7+ architecture document files (ADR-015 and related ARCH-INDEX shards)

**Summary:** DI-017 was renamed (the old `dispatcher_trace_id` symbol was replaced
by a new canonical name). Multiple architecture documents still contain the old
`dispatcher_trace_id` symbol string. This is the same rename-propagation pattern
flagged in prior passes for other symbols. Recommendation: a separate cleanup story
rather than an inline fix, per the adversary's own recommendation.

---

### F-8 [MEDIUM] `main.rs:143` and `sinks/mod.rs lines 11-15` line-number anchors persist in stories and ADR-015 (TD-VSDD-091 violation, recurrence pattern)

**Confidence:** HIGH
**Files:** 5+ story and ADR files

**Summary:** TD-VSDD-091 established that line-number anchors are not authoritative
and should be replaced with symbol/function-name anchors. At least 5 files still
contain `main.rs:143` and `sinks/mod.rs lines 11-15` style references that violate
TD-VSDD-091. This is a recurrence of the same class of defect addressed in prior
passes. Recommendation: a separate cleanup story per the adversary's own
recommendation.

---

### F-9 [MEDIUM] BC-2.06.001 EC-006 disagrees with Postcondition 2 / Invariant 2 about which CHANGELOG sections are conditionally vs. unconditionally required

**Confidence:** HIGH
**Files:**
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-02/BC-2.06.001.md`

**Summary:** BC-2.06.001 EC-006 and Postcondition 2 / Invariant 2 give contradictory
answers about whether certain CHANGELOG sections are conditionally required (only
when changes exist) or unconditionally required (always present). An implementer
reading EC-006 will build different behavior than one reading PC2/INV-2.

---

### F-10 [MEDIUM] Plugin enumeration mismatch between S-10.05 body and BC-4.09.001 Scope

**Confidence:** HIGH
**Files:**
- `/Users/jmagady/Dev/vsdd-factory/.factory/stories/S-10.05-adr015-wave2-plugin-schema-migration.md`
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-04/BC-4.09.001.md`

**Summary:** S-10.05 body text and BC-4.09.001 Scope section enumerate different
plugin sets. This is a continuation/recurrence of the plugin-set drift identified
in pass-4 F-1. Pass 4's fix burst addressed one surface but did not achieve full
four-way sync. The mismatch count is reduced from pass-4 but not zero.

---

### F-11 [MEDIUM] BC-1.12.005 Postcondition 4 cited as anchor for S-10.03 AC-007, but the cited text doesn't exist

**Confidence:** HIGH
**Files:**
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-01/BC-1.12.005.md`
- `/Users/jmagady/Dev/vsdd-factory/.factory/stories/S-10.03-adr015-wave1-resource-attribute-enrichment.md`

**Summary:** S-10.03 AC-007 carries a trace citation `(traces: BC-1.12.005 Postcondition 4)`.
BC-1.12.005 does not have a Postcondition 4 matching the cited semantics — the postcondition
numbering or content has diverged. The trace reference is a dangling anchor.

---

### F-12 [MEDIUM] BC-4.02.002 / BC-4.01.003 capability still "CAP-TBD" after rc.12 alignment cycle

**Confidence:** HIGH
**Files:**
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/BC-INDEX.md`

**Summary:** BC-4.02.002 and BC-4.01.003 both carry "CAP-TBD" in their Capability
column in BC-INDEX. The rc.12 alignment cycle resolved CAP-TBD entries for other
BCs but missed these two. Both BCs are in the E-10 reading list scope; their
capability anchors should be resolved against capabilities.md.

---

## Observations

### O-1 [LOW] [process-gap] Post-fix burst naming convention inconsistency between D-number suffixes and canonical D-NNN format

**Summary:** Some fix burst decision entries in decision-log.md use a D-NNN.N sub-decimal
format (e.g., D-327.1) while the canonical convention is integer D-NNN. Low-impact
editorial inconsistency; recommend standardizing to integer D-NNN for new entries.

---

### O-2 [LOW] BC-1.12.001 Changelog v1.3+ entries use inconsistent date formats (YYYY-MM-DD vs. YYYY/MM/DD)

**Summary:** Changelog date formats within a single BC alternate between ISO 8601
hyphen format and slash format. Minor editorial inconsistency; no semantic impact.

---

### O-3 [LOW] [process-gap] `behavioral_contracts` frontmatter arrays in stories are not sorted by BC identifier

**Summary:** Story frontmatter `behavioral_contracts:` arrays list BC identifiers
in authoring order rather than sorted order. This is a process gap — no sort
discipline has been established. Unsorted arrays make it harder to detect missing
entries by visual inspection. Recommend: define a sort convention (numeric by
BC subsystem then by number) and apply it on next fix burst.

---

### O-4 [LOW] [process-gap] BC-INDEX "Stories" column multi-value entries use inconsistent separators (`, ` vs ` / ` vs `;`)

**Summary:** The BC-INDEX Stories column uses commas, slashes, and semicolons
interchangeably as multi-value separators across rows. This is a process gap —
no canonical separator is defined. Recommend: standardize on `, ` (comma-space)
as the canonical separator for all multi-value cells.

---

### O-5 [LOW] BC-1.12.006 and BC-1.12.007 Purity Classification cells describe the same sub-point in different syntactic forms

**Summary:** BC-1.12.006 and BC-1.12.007 both classify as EFFECTFUL-OBSERVABLE
but describe the effect in different prose forms within the same cell schema.
Not a correctness defect; cosmetic divergence between sibling BCs that share
the same architectural surface. Could be collapsed in a future editorial sweep.

---

## Novelty Assessment

Pass 5 found **3 HIGH** findings (F-1 three-way drift, F-2 D-15.4 misattribution, F-3
S-10.04 AC-002 PC restructure regression) and **9 MEDIUM/LOW** findings.

The HIGH findings are distinct from pass-4 HIGH findings (pass-4 F-1 was the plugin-set
drift; pass-5 F-1 is the three-way BC-1.11.002 drift). F-3 is a Partial-Fix Regression
introduced by the D-327 fix burst that restructured BC-1.11.001 — the fix addressed the
BC body but did not propagate to S-10.04 AC-002 trace text.

F-7 (dispatcher_trace_id rename, 7+ files) and F-8 (line-N citations, 5+ files) are
explicitly recommended by the adversary for separate cleanup stories rather than inline
fixes, due to blast radius.

Trend is improving (16 → 12) but the counter does NOT advance because at least 2 HIGH
findings remain (F-1, F-2, F-3 are all HIGH).

**Review complete.** Verdict: **HIGH**. Counter does NOT advance to 1 (not NITPICK_ONLY).

---

## File References

- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-01/BC-1.11.001.md`
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-01/BC-1.11.002.md`
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-01/BC-1.11.003.md`
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-01/BC-1.12.001.md`
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-01/BC-1.12.002.md`
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-01/BC-1.12.003.md`
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-01/BC-1.12.004.md`
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-01/BC-1.12.005.md`
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-01/BC-1.12.006.md`
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-01/BC-1.12.007.md`
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-01/BC-1.12.009.md`
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-02/BC-2.06.001.md`
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-03/BC-3.05.004.md`
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-04/BC-4.09.001.md`
- `/Users/jmagady/Dev/vsdd-factory/.factory/stories/S-10.02-adr015-wave1-filesink-single-stream.md`
- `/Users/jmagady/Dev/vsdd-factory/.factory/stories/S-10.03-adr015-wave1-resource-attribute-enrichment.md`
- `/Users/jmagady/Dev/vsdd-factory/.factory/stories/S-10.04-adr015-wave1-trace-propagation-lifecycle-events.md`
- `/Users/jmagady/Dev/vsdd-factory/.factory/stories/S-10.05-adr015-wave2-plugin-schema-migration.md`
- `/Users/jmagady/Dev/vsdd-factory/.factory/stories/S-10.09-adr015-wave5-crate-retirement-ss03-rewrite.md`
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/BC-INDEX.md`
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/architecture/ARCH-INDEX.md`
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/domain-spec/capabilities.md`
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/domain-spec/invariants.md`
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/open-questions.md`
- `/Users/jmagady/Dev/vsdd-factory/.factory/stories/STORY-INDEX.md`
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/architecture/decisions/ADR-015-single-stream-otel-schema.md`
