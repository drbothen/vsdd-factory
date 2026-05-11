---
document_type: cycle-lessons
cycle: v1.0-feature-engine-discipline-pass-1
producer: state-manager
version: "1.0"
created: 2026-05-11
last_updated: 2026-05-11
---

# Lessons Learned — engine-discipline cycle (v1.0-feature-engine-discipline-pass-1)

> F-P9-004 backfill: this file was absent for the first 9 adversary passes.
> Lessons are reconstructed from adv-cycle-pass-1.md through adv-cycle-pass-9.md
> and from SESSION-CHECKPOINT.md. Per state-manager.md line 136 and D-382, all
> future lessons must be appended here when identified.

---

## L-EDP1-001 — Same-class-defect recurrence under fix-burst pressure (CI false-green chain)

**Source:** F-P3-001, F-P4-002, F-P5-001, F-P6-001 (CRITICAL, recurring)
**Date codified:** 2026-05-10

**Pattern:** Across F-P2-001 → F-P3-001 → F-P4-002 → F-P5-001 → F-P6-001, five consecutive
fix bursts introduced new CI false-green defects of the same class while attempting to
close the prior pass's CI false-green finding. Each fix burst declared closure based on
local-only validation; CI was not re-run on a PR branch before closure was recorded.

**Root cause:** No enforcement gate required CI evidence before recording a CI-class finding
as CLOSED. Fix bursts relied on implementer declaration alone.

**Codification:** D-379 (CI-green-signal rule for CRITICAL CI-class closures; authored
2026-05-10 in pass-5 advisory, codified with enforcement teeth in pass-6 fix burst).
Initial application: D-380 (F-P6-001 closure with CI run URL 25651192161 showing both
macos-latest and ubuntu-latest PASS). Broke the 5-pass chain.

**Status:** Codified as prose rule (D-379). Automated enforcement pending S-15.03.

---

## L-EDP1-002 — Sibling-file discipline gap recurrence under fix-burst pressure (cite-refresh + STATE.md + burst-log + INDEX.md)

**Source:** F-P8-001 (MEDIUM), F-P8-003 (MEDIUM), F-P9-001 (HIGH)
**Date codified:** 2026-05-11

**Pattern:** L-P20-002 (codified in plugin-async-semantics cycle) requires ARCH-INDEX
cite-refresh on child-index version bumps. The pass-7 fix burst (closing F-P5-002 /
F-P6-005) bumped BC-INDEX v1.63→v1.64 but missed ARCH-INDEX — the first L-P20-002
violation in 16 consecutive clean cite-refreshes (F-P8-001).

The pass-8 fix burst then codified D-381 (STATE.md mandatory in every fix burst) and
correctly updated STATE.md, but missed burst-log.md AND INDEX.md in that same burst
(F-P9-001 HIGH). D-381's scope was limited to STATE.md, which allowed the burst to
simultaneously comply with D-381 and violate the broader sibling-file discipline.

**Root cause:** Discipline rules codified in prose-only documents (decision-log entries)
without automated enforcement. The "initial application" clause creates a false sense of
completeness when the codified rule's scope is narrower than the actual obligation.

**Codification:** D-381 (STATE.md discipline, pass-8) + D-382 (full cycle-level sibling-file
set: STATE.md + burst-log.md + INDEX.md + lessons.md + decision-log.md, pass-9). Both
pending automation via S-15.03.

**Status:** Codified as prose rules (D-381 + D-382). Automated enforcement pending S-15.03.
Recurrence likely until S-15.03 ships.

---

## L-EDP1-003 — Recursive discipline violation: fix burst violates the rule it codifies

**Source:** F-P6-007 (MEDIUM), F-P9-001 (HIGH)
**Date codified:** 2026-05-11

**Pattern:** Two instances of the "fix burst violates the rule it is simultaneously
codifying" anti-pattern:

1. F-P5-008 (pass-5): advisory recommended CI-green-signal rule. The pass-5 fix burst
   authored the advisory but did not run CI before declaring F-P5-001 CLOSED. F-P6-007
   (pass-6) surfaced the meta-failure. D-379 codified the rule with enforcement teeth.

2. D-381 (pass-8 fix burst): the burst codified "every fix burst MUST update STATE.md"
   and correctly updated STATE.md, but did not update burst-log.md or INDEX.md —
   simultaneously violating the broader sibling-file obligation that D-381 was intended
   to address. F-P9-001 (pass-9) surfaced this. D-382 extended the scope.

**Root cause:** When authoring a new rule during a fix burst, the burst is evaluated
against the rule it is codifying (which is explicit) but not against the broader class
of obligations the rule is intended to enforce (which requires inferential reasoning
about scope). Without a checklist enumerating ALL obligations, the new rule creates a
"partial compliance" state.

**Codification:** D-379 + D-381 + D-382 (all codified as prose). S-15.03 tracks
tooling automation.

**Status:** Pattern documented. Each new fix burst is reminded of D-379/D-381/D-382
via decision-log, but no automated gate prevents omission.

---

## L-EDP1-004 — Forensic-marker proliferation under adversary review pressure

**Source:** F-P3-004 (process-gap observation)
**Date codified:** 2026-05-11

**Pattern:** 321+ `F-P[N]-NNN` forensic markers in production source (observed by
F-P3-004 during pass-3). These markers accumulate across fix bursts as evidence of
applied fixes, but they are in production source files and can create namespace
collisions between cycle-level (F-P[N]-NNN) and per-story identifiers. Cleanup was
deferred but the count continues to grow with each additional fix burst.

**Root cause:** No clean-up protocol established for forensic markers after convergence.
Markers are useful during adversary review but have no value after the cycle closes.

**Codification:** S-14.09 (forensic marker cleanup story; DRAFT for
v1.0-feature-engine-discipline-pass-2 cycle). S-14.09 was registered in STORY-INDEX
v2.65 as part of the F-P6-002/F-P6-004 fix burst.

**Status:** Acknowledged; story S-14.09 filed (draft). Deferred to follow-up cycle.

---

## Process Gaps Documented This Cycle

### PG-EDP1-001 — No lint/hook prevents CI-class CRITICAL closures without CI-green evidence

**Pattern:** D-379 codified the rule; S-15.03 tracks the automation. Until S-15.03 ships,
CI-class CRITICAL finding closures rely entirely on agent/implementer discipline to include
the CI-green URL in the closure record.

**Story:** S-15.03 (DRAFT) — index-cite-refresh + closure-verification hook.

---

### PG-EDP1-002 — No lint/hook enforces sibling-file discipline (cite-refresh, STATE.md, burst-log, INDEX.md, lessons.md)

**Pattern:** L-P20-002 + D-381 + D-382 codify the obligations in prose. The obligations
have been violated 3+ times across this cycle (F-P8-001, F-P9-001) despite being
explicitly codified. Automated enforcement is the only reliable remedy.

**Story:** S-15.03 (DRAFT) — scope expansion to include: (a) BC-INDEX version bump
without ARCH-INDEX changelog entry detection; (b) STATE.md touched without
burst-log.md / INDEX.md also touched detection; (c) fix burst commit without lessons.md
touch when process-gap finding is closed.

---

### PG-EDP1-003 — Cycle-level fix burst discipline reminders are prose-only in decision-log

**Pattern:** D-379, D-381, D-382 are in the decision-log. Agents dispatched for fix bursts
must read the decision-log to know these rules exist. There is no hook, no pre-commit
check, and no structured checklist that surfaces these rules at the moment a fix burst
executes. The STATE-MANAGER-CHECKLIST.md template exists in the engine but was not
instantiated for this cycle.

**Story:** S-15.03 (DRAFT) — instantiate STATE-MANAGER-CHECKLIST.md at cycle init;
add fix-burst discipline reminders to the checklist.

---

### PG-EDP1-004 — Forensic marker namespace not standardized (cycle vs per-story)

**Pattern:** Cycle-level adversary findings use F-P[N]-NNN (e.g., F-P3-001). Per-story
adversary findings use the same format. There is no visual distinction between a
cycle-level finding marker (referring to cycle-wide convergence) and a per-story finding
marker (referring to a specific story's convergence). As the cycle accumulates more
stories and passes, this creates ambiguity in source files about which pass a marker
refers to.

**Story:** S-14.09 (DRAFT; forensic marker cleanup) partially addresses this. A namespace
proposal (e.g., cycle findings as CF-P[N]-NNN; per-story as SF-P[N]-NNN) has been
informally discussed but not codified.

---

All four process gaps converge on S-15.03 scope: a tooling story authored as draft,
awaiting prioritization by the human gate at F7 delta convergence.
