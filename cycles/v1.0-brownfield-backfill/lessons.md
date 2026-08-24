---
document_type: lessons-codification
level: ops
cycle: v1.0-brownfield-backfill
producer: session-reviewer
timestamp: 2026-05-04T00:00:00Z
---

# Lessons Codified — v1.0-brownfield-backfill

> Lessons from this cycle that have been promoted to follow-up artifacts (stories or
> STATE.md Drift Items). Each entry records the gap, the evidence, and the disposition.

---

## RECURRENCE NOTE (D-1063) — D-1044(g)/D-995 class recurs one layer further out: BC-INDEX Stories-column + VP-INDEX Story-Anchors-row propagation legs

**Category:** process-gap (recurrence, not a new class)

**Session evidence** (2026-08-21, S-21.25 pre-TDD pass-5, F-S2125-P5-001/002): the D-1059 burst
that registered BC-3.08.001's Event 7 (`plugin.fuel_headroom_warning`) and bumped VP-079 to v1.20
correctly propagated the version-cite legs and the §Full Index row's six→seven event-type
enumeration, but self-deferred two further propagation legs that surfaced two clean-looking bursts
later (D-1060 through D-1062, three intervening passes, before the wider cross-index audit at pass
5 caught it): (1) BC-INDEX's own BC-3.08.001 Stories column never gained `S-21.25`, even though the
BC file's own §Traceability Stories row and §Story Anchor section were updated at the time Event 7
was registered to name S-21.25 as the anchor; (2) VP-INDEX's §Story Anchors VP-079 row (a
DIFFERENT row than the already-correctly-swept §Full Index row) retained the pre-registration
"six event types" enumeration and a now-doubly-stale `per BC-3.08.001 v1.19` version pin. Both are
the SAME underlying failure shape already codified at D-1044(g) (BC-version-bump-mid-cascade lacks
same-burst story-propagation-dispatch discipline) and D-995
([[L-BB-story-propagation-obligation-on-governing-bc-normative-prose-amendment]]) — a governing
artifact's version bump obligates a sweep of every downstream/sibling site citing it, and no
existing convention enumerates ALL such sites in one pass. This recurrence demonstrates the class
extends one layer further than either prior instance covered: D-995 was BC→story propagation;
D-1044(g) was BC-bump→story-frontmatter-and-body propagation; this recurrence is
BC-event-registration→INDEX-file-Stories-column-and-Story-Anchors-row propagation — a THIRD
distinct sweep-target surface within the same underlying obligation.

**Disposition (per explicit dispatch instruction):** NOT a new codification, NOT a new follow-up
story. Recorded as a recurrence anchored to the existing S-15.03 PRIORITY-A candidate — a
POLICY-14-leg-5 same-burst index-Stories-column sweep gate: whenever a BC-registration burst adds
a new anchor story to a BC's §Traceability/§Story Anchor sections, the SAME burst must sweep (a)
that BC's own BC-INDEX Stories column, and (b) every VP-INDEX row (both §Full Index AND
§Story Anchors, which are DISTINCT rows that can independently drift) whose event/property
enumeration references the same BC clause. Both findings (F-S2125-P5-001, F-S2125-P5-002) fixed
in scope at D-1063; no story-body defect — the S-21.25 story itself independently re-derived CLEAN
across all 7 previously-named risk areas at the same pass.

**Cites:** recurrence of [[L-BB-story-propagation-obligation-on-governing-bc-normative-prose-amendment]]
(D-995) and D-1044(g); companion to the D-996/D-998/D-1000/D-1004/D-1006 "fix/attest scoped to the
named site, not the class" family — this is the fourth generation of that family's underlying
insight applied to a NEW surface (cross-index Stories-column/Story-Anchors-row propagation, as
distinct from BC-body prose, story-body prose, index-aggregation-cell arithmetic, or
attestation-predicate adequacy). `[process-gap; recurrence; POLICY-8; POLICY-9; POLICY-14; BC-INDEX; VP-INDEX; Stories-column; Story-Anchors-row; story-propagation; state-manager; F-S2125-P5-001; F-S2125-P5-002; D-1044; D-995; D-1063; S-21.25; S-15.03-PRIORITY-A-anchor]`

## RECURRENCE NOTE (D-1064) — POLICY 19 (adr_version_cite_volatile_pin_prohibition) never applied to BC-1.03.019's own Traceability row across v1.0-v1.2 authoring + 5 prior adversary passes

**Category:** process-gap (recurrence-class candidate, not a new codification)

**Session evidence** (2026-08-21, S-21.25 pre-TDD pass-6, F-S2125-P6-001): a corpus-wide grep sweep
for POLICY 19 compliance (every BC Traceability row citing an ADR) found BC-1.03.019's own
Traceability ADR row was the SOLE outlier in the entire BC corpus, carrying a load-bearing
`ADR-039 v1.15 §Decision 5 Mitigation 1` version pin since its v1.1 authoring (D-1059) — a pattern
POLICY 19 explicitly prohibits precisely because ADR-039 continues amending (it already had, twice,
through §AMD-003 and multiple Erratum entries, by the time this pin went stale). The pin survived
five prior adversary passes (pass-1 through pass-5) of the S-21.25 LOCAL cascade, none of which
caught it, because each pass's rubric review focused on the story's own 7 named risk areas plus
whatever finding classes the immediately-prior pass had surfaced — no pass before pass-6 ran a
corpus-wide POLICY-19-specific grep against BC-1.03.019's own file. Sibling BC-3.08.001 carried the
identical pattern in its own Traceability ADR row for the same Event-7 provenance clause, introduced
at the same v1.25 registration burst (D-1059) and also un-caught through 5 subsequent bursts
(D-1060..D-1063).

**Disposition (per explicit dispatch instruction):** NOT a new codification, NOT a new follow-up
story. Recorded as a recurrence-class candidate anchored to S-15.03 PRIORITY-A — a tree-wide
POLICY-19 Traceability-row sweep gate applied at BC-authoring time (i.e., a lint/validator that
greps every new or amended BC's Traceability ADR row for a bare `ADR-NNN vX.Y` token and fails
closed if found, rather than relying on adversarial cascades to eventually catch it via corpus-grep).
This is a DIFFERENT failure shape than the D-1044(g)/D-995/D-1063 family (which concerns
propagation of a version bump OUTWARD to sibling sites) — this is a gap in APPLYING an existing
POLICY to a site AT AUTHORING TIME, only caught by a much-later corpus-wide audit. Both findings
(F-S2125-P6-001 HIGH, F-S2125-P6-002 LOW) fixed in scope at D-1064 by product-owner (BC-1.03.019
v1.2→v1.3, BC-3.08.001 v1.26→v1.27) with story-writer downstream cite-propagation (S-21.25
v1.4→v1.5); no story-body defect — the S-21.25 story itself independently re-derived CLEAN across
all 7 previously-named risk areas at the same pass.

**Cites:** distinct governance-discipline gap from the D-995/D-1044(g)/D-1063 propagation-sweep
family — this is a POLICY-application-at-authoring-time gap, not a propagation-sweep gap. Companion
observation: F-S2125-P6-003 (VP-079 internal six/seven event-type inconsistency, deferred to
architect) is itself a further instance of the SAME general shape (a correction applied at one site
— the Property Statement — not swept to a sibling site — the header comments — within the same
file), reinforcing that within-file sibling-sweep discipline (TD-VSDD-060) remains incompletely
internalized even at single-document scope. `[process-gap; recurrence-class-candidate; POLICY-19; adr_version_cite_volatile_pin_prohibition; BC-1.03.019; BC-3.08.001; Traceability-row; corpus-grep; state-manager; product-owner; story-writer; F-S2125-P6-001; F-S2125-P6-002; F-S2125-P6-003; D-1064; S-21.25; S-15.03-PRIORITY-A-anchor]`

## LESSON (D-1065) — S-21.19 is the first of the 7 split stories to converge its pre-TDD cascade; the fresh-context loop's value in surfacing a governance defect AFTER substantive convergence

**Category:** positive-signal / methodology note (not a process-gap)

**Session evidence** (2026-08-21, S-21.19 pre-TDD pass-7): S-21.19 reached BC-5.39.001 3-CLEAN
convergence (passes 5, 6, 7 all CLEAN) after 4 substantive remediation passes (1-4). Of the seven
split stories spawned from the D-1057 sizing-override decomposition of the converged S-21.11 v2.11
(S-21.19, S-21.20, S-21.21, S-21.22, S-21.23, S-21.24, S-21.25), S-21.19 is the FIRST to close its
own independent pre-TDD LOCAL adversarial cascade — 7 passes total, an asymptotic hygiene tail
(substantive findings at passes 1-4, zero substantive findings at passes 5-7). This establishes an
empirical baseline for the remaining six seams: S-21.25 is mid-cascade (7 passes, streak 1/3,
distinct 1 HIGH POLICY-19 governance finding at pass 6 after its own body had already been CLEAN
since pass 5); S-21.20/S-21.21/S-21.22/S-21.23 (Wave 7) and S-21.24 (Wave 8) have not yet started.

**Methodology observation:** S-21.19's own pass-6 finding pattern (zero findings, second consecutive
CLEAN pass) illustrates that a story's own body can be substantively converged well before its
governing BCs' cross-cutting governance properties (like POLICY 19 Traceability-row hygiene) are
independently audited — S-21.25's pass-6 HIGH was discovered by a corpus-wide grep sweep against
the GOVERNING BC (BC-1.03.019), not by re-reviewing the story body, which had already been CLEAN
since pass 5. This confirms the fresh-context adversarial loop's value extends beyond finding
story-body defects: periodic corpus-wide governance sweeps (POLICY-N compliance across ALL BCs, not
just the ones a given story cites) surface defects that per-story rubric review alone would miss,
because no single story's cascade has a natural trigger to audit BCs it does not itself amend.

**Disposition:** Not a process-gap requiring codification — this is a positive validation that the
BC-5.39.001 3-CLEAN protocol, applied independently per split seam, correctly distinguishes
"this seam's own perimeter is stable" from "the seam is ready for TDD in isolation from its
siblings" (the latter remains an orchestrator/human wave-sequencing decision, not an adversary
verdict). Recorded as a methodology note for the remaining six seams' own cascades, and as
context for the S-15.03 PRIORITY-A tree-wide POLICY-19 authoring-time sweep gate candidate
(D-1064) — such a gate would have caught F-S2125-P6-001 at BC-authoring time rather than five
adversary passes later. `[positive-signal; methodology; BC-5.39.001; 3-CLEAN-convergence; S-21.19; first-of-seven; split-seam; D-1057; D-1065; asymptotic-hygiene-tail; corpus-wide-governance-sweep; S-15.03-PRIORITY-A-context]`

## LESSON (D-1066) — WAVE 6 COMPLETE: both split seams with zero blocking DAG dependencies independently reached BC-5.39.001 3-CLEAN convergence; asymmetric pass-counts (7 vs 9) validate per-seam independence

**Category:** positive-signal / methodology note (not a process-gap)

**Session evidence** (2026-08-21, S-21.25 pre-TDD passes 8-9): S-21.25 reached BC-5.39.001 3-CLEAN
convergence (passes 7, 8, 9 all CLEAN) after a 9-pass cascade — 4 substantive remediation passes
(1-4), 2 index-propagation-residue passes (5-6, including a HIGH POLICY-19 governance finding at
pass 6 discovered AFTER the story body itself had already re-derived CLEAN at pass 5), then 3
consecutive CLEAN passes (7-9). Combined with S-21.19's own 7-pass convergence at D-1065, **both
Wave-6 seams (S-21.19 and S-21.25 — the two of the seven D-1057 split stories with zero DAG
dependencies blocking their own convergence work, `depends_on: [S-21.10]` and `depends_on: []`
respectively) have now independently reached full BC-5.39.001 3-CLEAN convergence, completing
Wave 6.** Fourth dispatch attempt for this closing burst — three prior state-manager delegates died
to API connection loss before any commit landed; this is the first successful commit for D-1066,
not a recovery from a partial or backfill state (TD-VSDD-053 single-commit discipline held: no
partial artifact was ever pushed by the three failed attempts).

**Methodology observation:** the two seams needed different pass-counts to converge (S-21.19: 7
passes; S-21.25: 9 passes) despite starting from the same D-1057 split-decomposition event and
running under the identical `.factory/policies.yaml` rubric. This asymmetry is expected and
correct, not a process anomaly: S-21.25's own governing BC (BC-1.03.019) had a POLICY-19
Traceability-row defect dating back to its v1.0 authoring (5 passes before it was caught by a
corpus-wide grep at pass 6), while S-21.19's governing BCs did not carry an equivalent latent
defect. This confirms the BC-5.39.001 3-CLEAN protocol correctly measures each seam's OWN
convergence state rather than assuming split seams inherit a shared clock from their common
parent — the D-1057 sizing-override decision explicitly anticipated this (per D-1057(k): "splitting
a converged spec does not inherit convergence for the split parts").

**Wave-sequencing implication:** with Wave 6 fully closed, the pipeline's next action is Wave 7
(S-21.20, S-21.21, S-21.22, S-21.23 — each starts its own pre-TDD cascade from pass-1, none has run
yet) followed by Wave 8 (S-21.24 capstone, STRICTLY LAST, gated on all five prior seams). Wave 7's
first three stories (S-21.20/21/22) each carry a known pre-existing drift item — a stale
`BC-1.03.017 v1.18` cite that must be re-anchored to `v1.19` before or during their own pass-1,
per the D-1060 deferral (extended at D-1064/F-S2119-P6-001 to also cover the decomposition-plan.md
§1 per-story detail and the STORY-INDEX sibling rows). S-21.23 cites only `BC-1.03.018` (confirmed
D-1062) and needs no re-anchor before its own pass-1. This front-loads the re-anchor work into the
FIRST burst of each affected story's cascade rather than deferring it further, avoiding a repeat of
the same class of index-propagation residue that cost S-21.25 two non-substantive passes (5-6) in
its own cascade.

**Disposition:** Not a process-gap requiring codification — this is a positive milestone marking
the completion of Wave 6 and validating that the per-seam independent-convergence discipline
(D-1057(k)) produces correct, if asymmetric, outcomes. Recorded as context for Wave 7's dispatch
planning — front-load the known BC-1.03.017 re-anchor rather than letting it surface as a
mid-cascade finding. `[positive-signal; methodology; milestone; BC-5.39.001; 3-CLEAN-convergence; wave-6-complete; S-21.19; S-21.25; split-seam; D-1057; D-1065; D-1066; asymmetric-pass-counts; wave-7-planning; wave-sequencing]`

## LESSON (D-1067) — Cycle-wide logs have no automated trim cadence, and the only related tool (`/compact-state`) feeds them rather than trimming them, so they grew unbounded until they broke state-manager burst reliability

**Category:** process-gap

**Session evidence** (2026-08-21, cycle-log-trim burst, root-caused from the six consecutive
D-1066 dispatch deaths): `decision-log.md`, `burst-log.md`, and `lessons.md` for
`v1.0-brownfield-backfill` grew to 21,539 / 29,806 / 11,330 lines respectively across the
continuous brownfield-onboarding cascade with no trim ever applied. The only existing tool with
"compact" in its name — the `/compact-state` skill (`plugins/vsdd-factory/skills/compact-state/`)
— does the OPPOSITE of what these files needed: it extracts historical content OUT OF STATE.md
and FEEDS it INTO these same cycle files (burst logs, adversary passes, session checkpoints,
lessons) to keep STATE.md itself under its own 200-line/415-line budget. There has never been a
tool or scheduled discipline that trims the cycle-file destination once content lands there. As a
direct consequence, the three files eventually exceeded the WASM-sandboxed PostToolUse validators'
`DEFAULT_FUEL_CAP` on nearly every Edit/Write/MultiEdit against them, each state-manager burst
touching them ballooned to roughly 40 minutes of wall-clock time, and six consecutive D-1066
seal-burst dispatch attempts died to "API connection lost mid-response" before any commit could
land — D-1066 was only rescued via a fourth-attempt direct commit of already-completed work.

**Root cause:** growth is asymmetric and directional. `/compact-state` (STATE.md → cycle files)
has an explicit trigger (STATE.md approaching its own size budget) and a well-defined target
(<200 lines). The cycle files themselves (decision-log.md, burst-log.md, lessons.md) have no
matching trigger or target — they are treated as unbounded historical logs by every existing
skill and hook, and nothing in the pipeline ever asks "is THIS file now too big to safely edit?"
until a validator starts failing closed on it (as `[D-954]` and `[D-442(e)]` both independently
recorded, months apart, without either resulting in a fix at the time).

**Disposition:** Section-aware archival at a named D-NNN cutoff boundary (active file retains the
current cascade forward from the boundary; everything before moves verbatim to a
`<log>-archive-through-D<NNN>.md` sibling file, with heading-count conservation independently
re-verified) is now the established remediation pattern for this class of drift item — see
`[D-954]`/`[D-442(e)]` RESOLVED this cycle via `decision-log-archive-through-D1056.md` /
`burst-log-archive-through-D1056.md` / `lessons-archive-pre-D1057.md`. This is anchored
**S-15.03 PRIORITY-A**: cycle logs must be trimmed at wave/epic boundaries, or proactively when a
file approaches the WASM validators' effective line-count budget (empirically, low thousands of
lines per file), not reactively after bursts start dying. A future S-15.03 PRIORITY-A story should
automate the trim trigger (e.g., a maintenance-sweep check that flags any cycle file above a
threshold) rather than relying on an AI agent noticing the file is large during an unrelated burst.
`[process-gap; cycle-log-bloat; wasm-fuel-exhaustion; burst-fragility; state-manager;
D-954; D-442(e); D-1057; D-1066; D-1067; S-15.03-PRIORITY-A; compact-state-asymmetry;
section-aware-archival]`

---

**Wave-7 floor-break (D-1077): consolidated full-perimeter consistency audit broke the asymptotic
floor in a single pass — process technique, not a defect.**

At D-1076 HEAD, a consolidated full-perimeter consistency-validator audit (all 10 audit classes,
all 6 stories simultaneously) surfaced all five remaining residual findings (C-W7-001..C-W7-005)
in one pass rather than the one-or-two-per-pass asymptotic pattern observed in prior Wave-7
rounds. This is a process-technique effect: the full-perimeter audit eliminates the partial-scan
selection bias that would cause some cross-story sibling-sweep misses to go undetected until a
later pass. The findings themselves were consistent with already-codified TD-VSDD-059/060
patterns (sibling-sweep misses, missing durable-gate task, missing FSR entry) — no new failure
mode or new codification was required. References: TD-VSDD-059 (paper-fix detection), TD-VSDD-060
(sibling-site sweep). `[process-technique; full-perimeter-audit; floor-breaking; consistency-validator;
TD-VSDD-059; TD-VSDD-060; D-1077; asymptotic-convergence; wave-7]`

---

**[codified] POLICY 19 must cover story-bodies in addition to behavioral-contracts-traceability-rows — omitting story-bodies leaves a sweep gap that re-accumulates forbidden ADR version pins in story bodies across each re-anchor pass.**

At D-1079 pass-7/R6, finding F-S2123-P7-P19-001 MED surfaced 6 ADR-039 version-pin cites in S-21.23's story body (`ADR-039 §Decision 3 v1.10` and `ADR-039 §Decision 3 v1.9` at 6 sites). POLICY 19, which prohibits load-bearing ADR version pins, was written to cover `behavioral-contracts-traceability-rows` — the class where the pin first appeared. Story body text was not explicitly in scope, so story-writer sweeps at earlier passes did not catch the story-body pins. The root cause is the scope enumeration in POLICY 19's `applies_to` field: it named only the initially-affected artifact type, not the broader logical class. The fix: extend POLICY 19 scope to `story-bodies` (policies.yaml v1.4.25) so that all future story-writer re-anchor sweeps are instructed to check story bodies as well as traceability rows. Changelog entries remain exempt (they record historical provenance; stripping version info from changelogs destroys audit trail).

**Disposition:** POLICY scope enumerations must explicitly cover every artifact class that contains the prohibited pattern — not just the class where the pattern was first detected. When a new finding class emerges in a previously unguarded artifact type (story bodies, ADR body narrative, architecture docs), the remediation should simultaneously: (1) fix the current instance, and (2) extend the POLICY scope to prevent silent re-accumulation in the same artifact type on the next re-anchor pass. `[codified; POLICY-19; story-bodies; ADR-version-pin; scope-enumeration; F-S2123-P7-P19-001; D-1079; wave-7]`

---

**[codified] Per-physical-line grep misses wrapped version-pin cites — multiline-normalized `tr '\n' ' ' | grep` detector is mandatory for all re-anchor sweeps.**

At D-1079 pass-7/R6, finding F-S2119-R6-001 MED revealed that S-21.19's Task 2 contained `ADR-044 v1.3` split across two physical lines ("`ADR-044`" on one line, "`v1.3`" on the next). The D-1078 pass-6/R5 sweep claimed 14 corrected sites but missed the 15th because it used single-line `grep 'ADR-044 v1.3'`, which cannot match tokens that cross a physical line boundary. This is a recurrence of F-S2119-P3-001, which first identified the line-wrap mechanism. The root cause: D-1078 documented the correct `tr '\n' ' ' | grep -oE 'ADR-[0-9]+ v[0-9.]+'` fix in its sweep log but the fix was NOT carried forward as a process discipline in POLICY 5 — meaning the next re-anchor sweep was free to revert to per-line grep.

**Disposition:** POLICY 5 (re-anchor sweep discipline) codified the mandatory `tr '\n' ' ' | grep` normalized detector at D-1079 (policies.yaml v1.4.25). Any version-pin sweep that uses `grep 'ADR-NNN v1.X'` directly on a file without the `tr` normalization stage is non-compliant with POLICY 5 and will silently miss wrapped cites. The `tr '\n' ' ' | grep -oE 'ADR-[0-9]+[[:space:]]+v[0-9.]+'` form is the canonical detector; sed-or-awk alternatives are acceptable provided they also normalize across physical lines before matching. `[codified; POLICY-5; multiline-sweep; tr-normalized; re-anchor-discipline; F-S2119-R6-001; F-S2119-P3-001-regression; D-1079; wave-7]`
