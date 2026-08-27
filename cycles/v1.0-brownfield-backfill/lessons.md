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
at the same v1.25 registration burst (D-1059) and also un-caught through 5 subsequent bursts,
D-1060..D-1063 (sample) — not an exhaustive burst-by-burst enumeration.

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

---

**[codified] A "POLICY 8 full propagation" claim for a BC version pin re-anchor is only valid if it includes a table-cell-aware grep that isolates the pipe-delimited BC-table Version column cell — free-text body sweep is insufficient.**

At D-1080 pass-8/R7, findings F-S2120-R7-001/F-S2121-P8-001/F-S2122-P8-001 (MED×3) revealed that three stories (S-21.20/21/22) had their BC-table Version column cells stuck at v1.25 even though D-1079 claimed "POLICY 8 full propagation" for BC-1.03.017 v1.25→v1.26. The D-1079 re-anchor sweep checked frontmatter `behavioral_contracts:`, H1 cite, and body narrative cites using free-text grep, but did not run a table-cell-aware grep (`grep -nE '\| *BC-1\.03\.017 *\| *v1\.[0-9]+'`) to isolate the Version column cell in the pipe-delimited BC table. The pipe-delimited table format (`| BC-1.03.017 | v1.25 |`) is structurally distinct from narrative prose and is not reliably captured by a free-text search for `BC-1.03.017 v1.25`. A sweep can find ALL narrative occurrences and declare "full propagation" while the table cell silently stays at the old version.

The consequence was significant: S-21.20 had reached 3/3 CONVERGED PROVISIONAL at D-1079 (streak 2/3→3/3 ADVANCE). Pass-8 re-confirmation found the BC table cell had NOT been propagated, which constitutes a genuine POLICY 8 violation. Streak reset 3/3→0/3.

**Disposition:** POLICY 8 TABLE-CELL-AWARE PARITY GATE codified at D-1080 (policies.yaml v1.4.26): any burst claiming "POLICY 8 full propagation" for a BC version pin re-anchor MUST run and capture a table-cell-aware grep (`grep -nE '\| *<BC-ID> *\| *v[0-9]'`) to isolate the BC-table Version column cell BEFORE attesting propagation. An unbacked "full propagation" claim is itself a POLICY 8 finding. The captured stdout MUST appear in burst-log Dim-2/Dim-6 evidence per D-449(a) literal-shell discipline. `[codified; POLICY-8; table-cell-aware-parity-gate; BC-version-pin; re-anchor-propagation; F-S2120-R7-001; F-S2121-P8-001; F-S2122-P8-001; straggler-class; D-1080; wave-7]`

---

**A field-identical sibling-struct type name is still a spec-vs-code defect the adversary must catch — structural similarity is not a substitute for verifying the actual call graph.**

At D-1082 (ADR-046 pass-25), finding F-P25-001 (MED) revealed that ADR-046 §Decision 1 and BC-7.07.001 Invariant 3b had annotated the value `renew_lock_if_holder` resolves at its holder-present step as `lock_state: FactoryLock`. Ground truth: `renew_lock_if_holder` performs its own independent `flp::parse_factory_lock(content)` parse at that step, which returns `LockState` (crate `factory-lock-parse`) — a distinct type from `FactoryLock` (crate `factory-lock`, produced only by the different function `factory_lock::parse_lock`, never called on this path). The two structs are field-identical (same field names, same field types), which is precisely why the misattribution survived 24 prior adversarial passes without producing any observable behavioral test failure or internal contradiction — nothing in the spec's own text was inconsistent with itself; the defect was only visible by tracing the actual function call and its actual return type against `crates/factory-lock/src/lib.rs` + `crates/factory-lock-parse/src/lib.rs`, not by reading the spec in isolation. This is the same underlying pattern as TD-VSDD-059 (paper-fix detection: a rename or doc-comment can look like a fix without being one) applied in reverse — here, a type name can look correct (same shape, same usage pattern) without being the actual type the code produces. The finding was originally caught only as a LOW "type-provenance imprecision" nit (O-P24-001) at pass-24 and required a second, more skeptical look to escalate it to a genuine MED spec-vs-code mismatch at pass-25.

**Disposition:** When a spec annotates a value's type by name (not just by shape/usage), the adversary MUST independently trace the actual function that PRODUCES that value in the referenced source file and confirm the type name matches — field-identical shape between two distinct types in sibling crates is not sufficient grounds to treat a type-name citation as low-severity or cosmetic. This generalizes beyond this specific `FactoryLock`/`LockState` pair: any spec citing a concrete Rust type name for a value sourced from a shared/reused function (not a fresh local computation) should have that citation checked against the function's actual return type, not just against whether the cited type "would work" given identical fields. Not a [process-gap] — this is a content defect in the spec artifact itself, correctly caught by the adversarial review process operating as designed (the process caught it; the spec authors introduced it). `[content-defect; type-provenance; spec-vs-code; sibling-struct; field-identical-types; F-P25-001; O-P24-001-escalation; D-1082; adr-046-gate; not-process-gap]`

---

**ADR-vs-BC implementing-story anchor drift: when a spec cites a story ID in narrative prose before that story is registered in every companion artifact's Traceability section, the citation itself is not a substitute for actually closing the cross-reference.**

At D-1082 (ADR-046 pass-25), finding F-P25-002 (MED) revealed that ADR-046 named S-17.05 as the implementing story in its own narrative text, while all three companion BCs (BC-4.17.001, BC-7.07.001, BC-5.40.001) still carried `[pending]` placeholders in their Traceability §Stories rows and §Story Anchor fields — and ADR-046's own "referenced in the File-Change Plan" cross-reference to S-17.05 did not actually resolve, because S-17.05 was not yet listed in the File-Change Plan itself. The root cause is a variant of TD-VSDD-060 (sibling-site sweep): when a story anchor is decided (here, during S-17.05's drafting), the decision needs to propagate to EVERY sibling artifact that references "the implementing story" — the ADR's own File-Change Plan, and each companion BC's Traceability section — in the same burst, not left as narrative-only in one location while formal cross-reference fields elsewhere still say `[pending]`. A reader trusting only the ADR's prose would believe the anchor was already closed; a reader (or validator) checking the BCs' formal Traceability fields would find it open.

**Disposition:** Whenever a spec artifact's narrative prose asserts a fact (e.g., "the implementing story is S-N.NN") that should also be reflected in a structured/formal field elsewhere (a Traceability table row, a File-Change Plan entry, a §Story Anchor field), the two MUST be updated in the same burst — narrative-prose-only assertions are a distinct drift class from the formal field they describe, and a sibling-site sweep across all artifacts sharing that cross-reference is required before declaring the anchor resolved. Not a [process-gap] — per adversary disposition, this is a content/traceability defect in the artifacts themselves, not a gap in the adversarial-review process (which correctly caught it). `[content-defect; traceability; story-anchor; sibling-site-sweep; TD-VSDD-060-variant; F-P25-002; D-1082; adr-046-gate; not-process-gap]`

---

**A self-referential version-bump DIRECTIVE inside an ADR's own File-Change Plan is itself a parity leg — TD-VSDD-060 sibling-sweep discipline extends to instruction rows, not just content rows.**

At D-1083 (ADR-046 pass-26), finding F-P26-001 (MED) revealed that ADR-046's File-Change Plan carries a row that does not describe the ADR's own content — it INSTRUCTS a downstream artifact (ARCH-INDEX) what version to cite for this ADR. That row had drifted stale: it still directed "bump to v1.10," leading with the pass-21/F-P21-001 disposition, even after the pass-25 architect edit (same session, immediately prior burst) had already advanced the ADR to v1.11 and added this same table's own new S-17.05 row (F-P25-002). The pass-25 edit swept every locus stating the ADR's substantive content (§Decision 1(b), the type-provenance loci, the Changelog/frontmatter) but did not sweep this SIBLING instruction row, because the row is not itself an assertion ABOUT the ADR — it is a downstream-facing directive whose own correctness depends on staying pinned to whatever version the CURRENT revision produces, a target that moves every single time the ADR's version bumps, including the very revision that just fixed something else entirely.

**Disposition:** TD-VSDD-060 (sibling-site sweep on value changes) already required sweeping every site that CITES a changed value. This generalizes it one layer further: a version-bump obligates a sweep not just of every site citing the ADR's OLD version, but of every site whose own text INSTRUCTS what the ADR's NEW version should be cited as elsewhere — a self-referential directive is itself a parity leg, and it goes stale on every single-version bump the ADR undergoes, not just on bumps that change its substantive content. Practically: any architect edit that changes an ADR's `version:` frontmatter field must include a check of the File-Change Plan (or any other section) for rows of the form "artifact X's row should cite version vN" and update N to match the version this very edit produces — a `grep -n "version bump to\|bump to v[0-9]"` sweep against the ADR's own body, run as part of every version-bump edit, is sufficient to catch this class before adversarial review does. Not a [process-gap] — per adversary disposition, this is a content defect in the ADR artifact itself (the row's own text was wrong), correctly caught by the adversarial review process operating as designed. `[content-defect; self-referential-directive; sibling-instruction-row; TD-VSDD-060-variant; version-bump-obligation; F-P26-001; D-1083; adr-046-gate; not-process-gap]`

---

**Resolving a `[pending]` implementing-story anchor to a real story ID must sweep ALL sibling loci in the SAME burst — §Story Anchor (incl. any cardinality quantifier), §Traceability §Stories, status/lifecycle_status parity, `inputs:` completeness, and every prose mention, not just the Traceability rows the initial fix touched.**

At D-1084 (ADR-046 pass-27), findings F-P27-001 (HIGH), F-P27-002 (MED), and F-P27-003 (MED) all trace back to the SAME root event: the pass-25 (D-1082) fix that resolved BC-5.40.001's and BC-7.07.001's Traceability §Stories rows from `[pending]` to the confirmed implementing story S-17.05. That fix correctly updated the Traceability §Stories rows in both files, but left three sibling loci un-swept for two full passes (25 and 26): (1) each BC's own §Story Anchor section — a DIFFERENT heading than §Traceability §Stories, which had never been touched and in BC-5.40.001's case carried a now-incorrect cardinality quantifier ("Dual-story anchor" when the story count had grown to three); (2) BC-7.07.001's `status: draft`/`lifecycle_status: active` frontmatter contradiction, pre-existing and unrelated in root cause to the S-17.05 anchor but caught by the same pass; (3) BC-7.07.001's `inputs:` frontmatter completeness relative to its own body's code/BC citations. A partial anchor-resolution that touches only the primary Traceability row leaves HIGH-severity contradictions in sibling sections — §Story Anchor is a formally-distinct field from §Traceability §Stories even though both describe "the implementing story," and a reader (or validator) trusting one while the other still says something different (or omits the story entirely) has no way to know which is authoritative.

**Disposition:** This generalizes TD-VSDD-060 (sibling-site sweep on value changes) to a specific, recurring high-value checklist for the "resolve a `[pending]` story-anchor placeholder" class of fix, distinct from — but structurally identical in shape to — the D-1082/F-P25-002 lesson and the D-1083/F-P26-001 self-referential-directive lesson already codified above: whenever a burst resolves an implementing-story `[pending]` placeholder to a real story ID anywhere in a spec artifact, the SAME burst must sweep, at minimum: (a) §Story Anchor (including any cardinality-quantifier word like "Dual"/"Tri" that encodes the story count), (b) §Traceability §Stories (the row this class of fix most often touches first), (c) status/lifecycle_status parity across the file and its BC-INDEX row (a pending amendment does not itself imply draft status, but any existing status contradiction surfaced during the same review should be resolved in the same burst rather than left for a later pass), (d) `inputs:` frontmatter completeness for any newly-cited code/BC dependency the resolved story's implementation touches, and (e) every other prose mention of "the implementing story" or "pending story" in the file. Not a [process-gap] — per adversary disposition, these are content defects in the BC artifacts themselves (the sibling sections' own text was stale or incomplete), correctly caught by the adversarial review process operating as designed; the process is not missing a capability, the BC-authoring/fix bursts have twice now under-swept the sibling-loci set for this specific placeholder-resolution class. `[content-defect; story-anchor; sibling-sweep; TD-VSDD-060-variant; cardinality-quantifier; status-lifecycle-parity; inputs-completeness; F-P27-001; F-P27-002; F-P27-003; O-P27-001; D-1084; adr-046-gate; not-process-gap]`

---

**A fix landing on 1 of N siblings carrying an identical claim doesn't just leave the other N-1 siblings' gap open — it can inject a FALSE cross-reference/premise into the fix's OWN disposition text, because the fixer asserted something about a sibling it never actually checked.**

At D-1085 (ADR-046 pass-28), findings F-P28-001 (HIGH) and F-P28-002 (MED) both trace to the SAME root event: the pass-27 (D-1084) fix that resolved BC-7.07.001's `inputs:` completeness and `status:` contradiction. That fix's own disposition prose made two comparative claims about sibling BC-4.17.001 — "mirroring sibling BC-4.17.001's input set" (F-P27-003) and "sibling BC-4.17.001/BC-5.40.001 both carry `status: active`" (F-P27-002) — and BOTH were false: BC-4.17.001 did not yet contain the cited crate in its own `inputs:` at that time, and BC-4.17.001 is correctly `status: draft` (its own base deliverable, story S-17.05, has not shipped), not `active`. The pass-27 fixer never actually opened BC-4.17.001 to verify either claim; it asserted a sibling's state as supporting rationale for its own disposition without checking it. This is a distinct failure mode from ordinary sibling-sweep omission (D-1084's lesson, immediately above): there, sibling loci were simply left untouched; here, the fix's own justifying prose made an affirmative, checkable, and false claim about a sibling it did not verify.

**Disposition:** When fixing a claim shared across N artifacts, the correct discipline is not just "enumerate and fix all N" (per TD-VSDD-060 and the sibling-sweep lesson above) but also: NEVER write a disposition asserting a sibling artifact's current state ("mirrors X", "same as sibling Y", "parity with Z") without opening and verifying that sibling's actual current content at fix time. A comparative claim used as supporting rationale is itself a checkable fact, and an adversary (or a careful self-review) can and should verify it independently of whether the primary fix itself was correct — the primary fix values (here, BC-7.07.001's `inputs:`/`status:` VALUES) can be entirely correct while the PROSE justifying them is false. Tag as content-defect discipline; relates to but is distinct from TD-VSDD-060 (sibling-site sweep is about propagating a VALUE change to all sites; this is about not fabricating an unverified claim about ANOTHER site's value while fixing THIS site). Not a `[process-gap]` — the adversarial-review process caught it correctly; the pass-27 fix burst asserted unverified facts. `[content-defect; false-premise; unverified-sibling-claim; comparative-disposition-prose; TD-VSDD-060-adjacent; F-P28-001; F-P28-002; D-1085; adr-046-gate; not-process-gap]`

---

**[codified] A self-referential version-bump DIRECTIVE that hard-codes its target as a literal number will recur indefinitely — the only structural fix is to make the directive read the artifact's own live `version:` field, not to keep patching the literal each time it's caught stale.**

At D-1085 (ADR-046 pass-28), observation O-P28-002 (`[process-gap]`, LOW) caught ADR-046's own File-Change Plan ARCH-INDEX sync-instruction row stale for the THIRD time: F-P25-002 (D-1082) added a new File-Change Plan row without sweeping the directive forward; F-P26-001 (D-1083) caught and rewrote a stale "v1.10" straggler to "v1.12"; pass-28 caught it again — and, had it been patched the same way a third time (bump the literal to "v1.13"), it would have gone stale a FOURTH time the moment this very burst's fix landed, since the directive's own correctness depends on staying pinned to whatever version the CURRENT revision produces, a target that moves on every single version bump the ADR undergoes. Patching the literal number is a treadmill, not a fix — the D-1083 lesson (already codified) correctly identified that a self-referential directive is itself a parity leg requiring sweep, but did not resolve the recurrence, because sweeping a literal-valued directive still leaves a literal-valued directive that will go stale again on the very next bump.

**Disposition:** ROOT-CAUSE FIXED this burst, not symptomatically patched a third time. The directive is restructured to a VERSION-STABLE instruction: instead of "bump ARCH-INDEX's ADR-046 row to vX.Y" (a literal that must be re-edited every single version bump), it now reads "bump ARCH-INDEX's ADR-046 row to match this ADR's current frontmatter `version:` field" — a construction that is correct by definition regardless of what the ADR's version number is at any future bump, because it delegates to the live field rather than embedding a snapshot of it. This generalizes beyond ADR-046: any spec artifact containing a directive of the form "the downstream artifact should cite version vN" is structurally guaranteed to drift on every subsequent version bump unless N is expressed as a reference to the artifact's own live version field rather than a literal number. When authoring or fixing such a directive, prefer the version-stable form from the start; when found in literal form during review, restructure it (don't just re-literal-patch it) — the recurrence count (3+ occurrences here) is itself the signal that literal-patching is the wrong fix class. `[codified; process-gap; self-referential-directive; version-stable-directive; TD-VSDD-060-generalization; recurrence-class; O-P25-002; F-P26-001; O-P28-002; D-1083; D-1085; adr-046-gate; root-cause-fix]`

---

**A cross-language home-crate mis-attribution can survive many passes of "attribution audit" scrutiny undetected, because every prior audit only re-checked SYMBOLS that had ALREADY been flagged once — the audit's own scope was implicitly bounded by its own history, not by an exhaustive symbol inventory.**

At D-1086 (ADR-046 pass-29), finding F-P29-001 (HIGH) revealed that ADR-046 self-contradicted on the private Rust fn `rewrite_expires_at`'s home crate: one locus (F-P10-001's own v1.8 citation) correctly placed it in `crates/factory-lock/src/lib.rs`, while two OTHER loci (the Companion Amendment 2 PC4-reconciliation bullet and the v1.8 Changelog entry's own closing sentence) wrongly described it as living inside the bash script `factory-lock-write.sh`. This ADR had already been the subject of a dedicated, successful, ADR-wide "home-crate/hedge audit" at pass-7 (F-P7-001), which decisively pinned `trim_git_email`'s shared home to `crates/factory-lock` across every hedge locus that mentioned it — but that audit, by construction, only swept loci that CITED `trim_git_email` by name; it never established a general inventory of every cross-language symbol this ADR makes claims about and checked each one's file-of-record. `rewrite_expires_at` was never on that list, so the pass-7 audit's success created a false sense that "the home-crate attribution class" was closed, when in fact only the ONE symbol that pass-7 investigated had been verified — `TTL_SECONDS`, `LockState`, `STATE_MD_MAX_BYTES`, and `rewrite_expires_at` each required (or, in `rewrite_expires_at`'s case, still require as of this pass) their OWN independent verification pass.

**Disposition:** When an ADR or BC makes a claim of the form "symbol X lives in crate/file Y" for a cross-language or cross-crate implementation detail (a function name, a constant, a type), do not treat a prior successful "home-crate audit" for a DIFFERENT symbol as evidence the ADR's home-crate attributions are generally sound — each symbol is an independent claim requiring independent verification against the actual source file. The correct discipline is a FULL-DOCUMENT symbol inventory (every named function, constant, and type the artifact makes a concrete "lives at path P" claim about), each one checked by inspection against the cited file, not a spot-check triggered only by a symbol some prior pass happened to flag. This generalizes the D-1082/F-P25-001 lesson (spec-vs-code type-provenance) from "verify the type a function RETURNS" to "verify the crate/file a symbol IS DEFINED IN" — both require tracing the actual source, not trusting a prior pass's audit scope as if it were exhaustive. Not a `[process-gap]` — per adversary disposition, this is a content defect in the ADR/BC artifacts themselves (a genuinely new defect class, not a regression of a prior fix), correctly caught by the adversarial review process operating as designed. `[content-defect; home-crate-mis-attribution; cross-language-symbol; spec-vs-code; audit-scope-not-exhaustive; F-P29-001; D-1086; adr-046-gate; not-process-gap]`

---

**[process-observation] Three consecutive passes (27, 28, 29) each shed a partial-fix regression of the immediately-prior pass's own fix — the behavioral core has converged, but the metadata/hygiene layer (`inputs:` completeness, array-ordering, cross-reference accuracy) is not reaching literal 3-CLEAN under manual sweep discipline alone; this is an ASYMPTOTIC-FLOOR pattern, not a random-noise pattern.**

At D-1086 (ADR-046 pass-29), the adversary's own Part B assessment explicitly separated two classes: the BEHAVIORAL core (write-composition table, five-outcome table, identity-gating logic, event-sourcing struct-variant text) has been VERIFIED CLEAN across three consecutive passes (27, 28, 29) with zero regressions — the substantive design is sound and stable. But the spec has NOT reached literal 3-CLEAN, because each of the last three passes has found a METADATA-layer defect that traces directly to the immediately-prior pass's OWN fix: pass-28's F-P28-002 (de-scoped BC-5.40.001 from a sibling POLICY 18 sweep) produced pass-29's F-P29-002; pass-27's O-P27-001 `modified:`-array reorder fix was re-regressed by pass-28's own v1.30 edit, producing pass-29's F-P29-003. This is structurally different from ordinary "adversary keeps finding new bugs" — it is a fix-introduces-its-own-next-finding cascade confined entirely to the bookkeeping/metadata layer (inputs arrays, ordering arrays, cross-reference prose), never the behavioral content. Three of the last three passes fit this pattern exactly.

**Disposition:** This is recorded as a `[process-observation]`, not resolved this burst per explicit task instruction (a convergence-strategy question anchored for human decision, not a spec content defect this state-manager burst can fix). The pattern suggests that fully-manual metadata-consistency sweeps (recompute every sibling's `inputs:`/`modified:`/cross-reference-prose state by hand, every burst) have an inherent per-burst error rate for LARGE multi-artifact bursts (3-4 files touched simultaneously) that is not converging to zero through additional manual passes — the same class of observation S-15.03 PRIORITY-A automation is meant to address for OTHER artifact classes (BC-INDEX/ARCH-INDEX `last_amended` growth, cycle-log trim cadence). Whether this warrants (a) continuing literal 3-CLEAN under manual discipline (accepting the current per-pass floor as noise that will eventually clear), (b) accepting D-386 Option C asymptotic-acceptance for THIS gate specifically (as already applied to the E-10 sub-cycle and the F5 cycle), or (c) building a structural cross-artifact consistency checker (mirroring S-15.03's planned automation) before continuing the manual cascade, is a human decision — not resolved here. `[process-observation; asymptotic-floor; partial-fix-regression-cascade; convergence-strategy; metadata-layer-vs-behavioral-core; F-P28-002; F-P27-003-vs-F-P29-003; D-1086; adr-046-gate; anchored-human-decision; not-resolved-this-burst]`

---

**[convergence-strategy][codified] Spot-fixing only the single locus the adversary explicitly flags perpetuates the exact partial-fix-regression cascade the D-1086 process-observation identified — switching to a COMPREHENSIVE per-dimension sweep (fix the flagged locus AND every sibling that shares the same defect class, in the same burst) is what closed the metadata layer.**

At D-1087 (ADR-046 pass-30), findings F-P30-001 (HIGH) and F-P30-002 (MED) were BOTH instances of defect classes the immediately-prior three passes (27, 28, 29) had each already fixed on ONE artifact but never swept to every sibling that shared the identical defect: F-P30-001 is the F-P29-003 `modified:`/Changelog array-ordering-parity class, previously fixed only on BC-7.07.001, left un-swept on BC-4.17.001 and BC-5.40.001 (which carried the identical ascending-vs-descending mismatch the whole time); F-P30-002 is the F-P28-001/F-P29-002 `inputs:`-completeness class, previously swept across the three companion BCs but never applied to ADR-046 itself via a MANDATORY complete-document audit (only a spot-check, which pass-28 had already tried and missed 2 of the 6 omissions pass-30 found). This confirms the D-1086 process-observation's asymptotic-floor pattern was NOT random noise — it was the direct, predictable consequence of remediation bursts fixing only the specific locus an adversary pass named, rather than exhaustively fixing every artifact that shares the same defect class.

**Disposition:** This burst's remediation deliberately switched technique: (1) for F-P30-001, product-owner did not just reorder the two BCs the adversary explicitly named — it ran a full 3-BC cluster parity audit (version / Changelog-head / modified-head / last_amended-prefix parity, `inputs:` completeness, §Story-Anchor cardinality) across ALL THREE companion BCs, confirming BC-7.07.001 already clean rather than assuming it based on the pass-29 fix; (2) for F-P30-002, architect ran a MANDATORY complete inputs-completeness audit of ADR-046's ENTIRE document body (every load-bearing current-state file citation, not just the two files the adversary flagged), finding and fixing 6 omissions instead of the 2 initially named, while explicitly documenting the ones REJECTED as non-load-bearing padding (avoiding the opposite failure mode of over-adding). Pass-30 returned ZERO spec-vs-code contradictions — both findings were pure metadata parity — which is consistent with the behavioral core having been genuinely stable since pass-27 (per the D-1086 observation) and suggests the metadata layer itself may now be closing under this comprehensive-sweep technique where single-locus spot-fixes had not. **This is a hypothesis pending confirmation, not a closed result**: if pass-31 returns CLEAN, this comprehensive-per-dimension-sweep technique is the mechanism that closed the metadata layer and should be the DEFAULT remediation technique for all future spec-convergence findings on multi-artifact clusters, not merely a one-off response to this specific finding pair. If pass-31 finds further metadata stragglers despite the comprehensive sweep, the D-1086 process-observation's harder question (structural cross-artifact consistency automation, per S-15.03 PRIORITY-A) remains open and the technique alone is insufficient. `[convergence-strategy; codified; comprehensive-sweep; per-dimension-audit; anti-spot-fix; F-P30-001; F-P30-002; D-1087; adr-046-gate; asymptotic-floor-followup; pending-pass-31-confirmation]`

---

**[convergence-strategy][codified] Comprehensive audits yield stragglers a spot-fix of the flagged findings alone would leave for future passes — extending the D-1087 sweep technique from "every sibling BC sharing the flagged defect class" to "every cross-anchor citation and every spec-inputs claim inside the SAME BC a flagged finding already touched" is a further convergence accelerant, confirmed (not refuted) by this pass.**

At D-1088 (ADR-046 pass-31), the two flagged findings — F-P31-001 (MED, BC-5.40.001 `inputs:` completeness) and F-P31-002 (MED, BC-7.07.001 cross-reference retarget) — were fixed by product-owner exactly as named. But per the D-1087 convergence-strategy lesson's own discipline (fix the flagged locus AND sweep every related surface, not just the named one), product-owner additionally ran (a) a full cross-anchor semantic audit — open every `BC-X.YY.ZZZ §Section`/`PCn`/`Invariant-N` citation in BC-5.40.001's and BC-7.07.001's bodies and verify it against the cited BC's actual section content, not just the citation F-P31-002 already flagged — and (b) a full spec-inputs completeness audit — check every load-bearing claim BC-7.07.001's body makes against files absent from its own `inputs:`, not just the fact that F-P31-001 had already found a gap on the sibling BC-5.40.001. These two audits caught 3 GENUINE additional defects the 2 flagged findings alone would not have surfaced: BC-5.40.001's own PC1/PC2 mis-cite of "BC-6.23.001 PC3/PC4" (should be PC4-only — PC3 is an unrelated acquire-path refusal), and 5 missing load-bearing spec inputs on BC-7.07.001 (BC-5.40.001, BC-5.41.003, BC-1.15.001, BC-2.02.011, domain-spec/invariants.md). Left unaudited, these would have surfaced as separate findings on pass-32 or pass-33, each resetting nothing (streak already 0/3) but each costing a full pass-cycle before discovery.

**Disposition:** This CONFIRMS the D-1087 convergence-strategy hypothesis was on the right track, and generalizes it one dimension further: the useful unit of "comprehensiveness" for spec-convergence remediation is not just "every sibling artifact sharing the flagged defect's CLASS" (D-1087's scope: modified:-array ordering swept across all 3 BCs; inputs:-completeness swept across the whole ADR document) but ALSO "every claim of the SAME KIND as the flagged claim, inside the SAME artifact the fix already opened" — if a finding flags one wrong cross-anchor citation, audit every cross-anchor citation in that file while it's already open; if a finding flags one `inputs:` gap, audit the whole file's body for other load-bearing citations absent from `inputs:` while it's already open. This pass did NOT reach a literal-CLEAN verdict (2 findings were flagged, meaning genuine defects existed BEFORE the audit ran) — so the technique does not, by itself, guarantee a CLEAN pass on first application to a newly-audited dimension. But its YIELD (3 extra genuine defects caught same-burst, at zero marginal pass-cycle cost, because the file was already open for editing) is exactly the kind of accelerant the D-1086 asymptotic-floor process-observation called for. Practically: whenever a remediation burst opens a BC file to fix one flagged cross-reference or one flagged `inputs:` gap, the fixer should default to running the FULL cross-anchor-citation audit or FULL spec-inputs-completeness audit on that file in the same pass, not just the one flagged locus — the marginal cost of auditing the rest of an already-open file is far lower than the cost of a dedicated future pass discovering the same defect piecemeal. Not a `[process-gap]` — per adversary disposition, these are content defects in the BC artifacts themselves, correctly caught by comprehensive-audit discipline operating as designed. `[convergence-strategy; codified; comprehensive-audit-yield; cross-anchor-audit; spec-inputs-audit; same-file-broader-scope; F-P31-001; F-P31-002; D-1088; adr-046-gate; convergence-accelerant; confirms-D-1087-hypothesis]`

---

**[codified][process-gap] The `modified:`-array-head-omission-on-version-bump defect has now recurred THREE times (F-P29-003, F-P30-001, F-P32-001) — a version bump that correctly updates `version:` + the `## Changelog` table + `last_amended:` but forgets to prepend the corresponding `modified:`-array head entry is a recurring SHAPE, not three independent one-off slips, and manual per-burst discipline alone is not preventing it.**

At D-1089 (ADR-046 pass-32), finding F-P32-001 (HIGH) found BC-7.07.001's `modified:` array missing its own v1.32 entry: the Pass-31 remediation burst (D-1088) had correctly bumped `version:` to 1.32, added the `## Changelog` v1.32 row, and updated the `last_amended:` prefix to `(v1.32)` — three of the four in-file parity legs agreed — but never prepended the corresponding `modified:`-array entry, leaving the array's head at v1.31. This is the THIRD occurrence of this exact shape on this gate: F-P29-003 (pass-29, D-1086) found the SAME omission on BC-7.07.001 itself (a re-regression of an even earlier ordering fix, O-P27-001); F-P30-001 (pass-30, D-1087) found the same class on BOTH BC-4.17.001 and BC-5.40.001. Each prior occurrence was fixed as a spot-fix on the specific artifact the adversary flagged — the D-1087 comprehensive-sweep technique (codified two lessons above) was applied to `inputs:`-completeness and cross-anchor-citation classes, but was never retroactively applied to this specific `modified:`-array-head-parity class as a standing PRE-BURST discipline, only as a POST-HOC fix each time the adversary caught it fresh. A pattern is a codification signal at 2 occurrences and a MANDATORY-fix signal at 3+; this is the 3rd.

**Disposition:** CODIFIED this burst as a MANDATORY pre-declare-done check, not merely a further spot-fix: every BC/artifact version bump MUST run a 4-leg head==version self-check — `version:` == `modified:`-array-head == `## Changelog`-table-head == `last_amended:`-prefix, with NO gap in the `modified:` array (i.e., the array must contain a contiguous, strictly-descending run from the new head down to v1.1, not merely have SOME entry matching the new version anywhere in the array) — BEFORE the burst that performs the bump is declared done. This is a `[process-gap]`, distinct from the `[content-defect]`-tagged sibling-sweep lessons above (D-1084, D-1085, D-1086): those were defects IN the spec artifacts' substantive claims, discoverable only by domain review; this is a MECHANICAL, purely-structural self-consistency property of a single frontmatter block, checkable by grep/regex with no domain knowledge required, and its 3+ recurrence under manual discipline is itself evidence that manual discipline is not the right layer to enforce it at. Per the S-7.02 cycle-closing checklist, a follow-up anchor is recorded (not fixed this burst — this is `.factory/` artifact-authoring/process discipline, not factory-artifacts scope) for a MECHANICAL `validate-modified-head-parity` validator hook: a PreToolUse or PostToolUse WASM guard on Edit/Write to any BC/ADR file that extracts `version:`, the `modified:`-array's first element, the `## Changelog` table's first row, and the `last_amended:` prefix, and fail-closed-blocks (or fail-open-advises, pending human policy call on enforcement strength) when they disagree. The existing `validate-changelog-monotonicity` hook is the natural sibling to extend or the precedent to model the new hook on — it already checks Changelog-table date/version monotonicity but does NOT check `modified:`-array-head==`version:` parity, which is the specific gap this 3rd recurrence exposes. Anchored to the same S-15.03 PRIORITY-A automation tranche already carrying this gate's other mechanical-consistency-checker follow-ups (e.g., the `[D-1082]` cyclic-hash structural fix, the BC-INDEX/ARCH-INDEX `last_amended` unbounded-growth compaction). `[codified; process-gap; modified-head-parity; version-bump-self-check; 4-leg-parity; mechanical-validator-anchor; validate-modified-head-parity; F-P29-003; F-P30-001; F-P32-001; D-1089; adr-046-gate; recurrence-class; S-15.03-priority-a-anchor]`

---

**[codified][process-gap] An "inputs-completeness audit" performed as a human read-through is NOT the same discipline as a GREP-COMPLETE mechanical audit — three consecutive prose-narrative audits (passes 28, 30, 31) each believed themselves complete and each still shed exactly one straggler on the NEXT pass; only switching to mechanical file-path-token enumeration finally drained the class.**

At D-1090 (ADR-046 pass-33), finding F-P33-001 (MED) found ADR-046's own `inputs:` array still missing `crates/hook-sdk/src/result.rs` — despite THREE prior passes (28, 30, 31) each having run what its own authoring agent described as a "MANDATORY complete inputs-completeness audit" or "MANDATORY COMPLETE inputs-completeness audit (not a spot sweep)" on this exact ADR and its companion BCs. Each of those three audits found and fixed real omissions (pass-28: 1 file; pass-30: 6 files; pass-31: 2 files on one BC plus 5 on another), and each believed its own sweep to be exhaustive — yet each was followed by a subsequent pass finding at least one MORE straggler of the identical parity-gap character. This burst's remediation changed METHOD, not just effort: instead of a human reading the document body section-by-section and noting every file mention encountered (a read-through — thorough, but bounded by what the reader's attention actually catches on a single pass), the architect ran a MECHANICAL `grep -noE` sweep across every file-path-shaped token class in the document (`crates/[...]\.rs`, `plugins/[...]\.(sh|toml)`, `.factory/[...]\.(md|yaml)`, bare `[...]\.(toml|md|yaml|bats)` basenames, backtick-quoted path literals, `(BC|ADR|VP|DI)-[...]` identifiers), producing an exhaustive candidate list independent of reading-attention, then checked EACH hit against `inputs:` and against whether it is a genuine load-bearing current-state claim (vs. a forward-only sync instruction or pure locator, explicitly rejected with a stated reason). This single grep-complete pass caught the flagged item (`result.rs`) PLUS one further genuine omission (`BC-4.17.001.md`, cited ~20 times) PLUS a latent bracket-balance defect in the same field's `last_amended` history — three distinct catches in one sweep, where the prior three read-through audits combined caught only their own respective single-digit counts and still left stragglers behind each time.

**Disposition:** CODIFIED this burst as a MANDATORY discipline, not merely a one-off technique choice: a claimed "inputs-completeness audit" is only valid if it is GREP-COMPLETE — i.e., performed via mechanical file-path-token enumeration across the pattern classes above, with the resulting per-path disposition (found in `inputs:` already / added / explicitly rejected with a stated reason) recorded in an auditable table, as done in ADR-046's own v1.16 Changelog entry — NOT a human read-through, however careful or however many times repeated. A read-through's completeness is bounded by what the reader's attention happens to catch during one linear pass through prose; a grep sweep's completeness is bounded only by whether the pattern-class set itself is exhaustive (itself checkable, and expandable if a future pass finds a token shape the existing classes miss). This generalizes beyond ADR-046: any spec artifact's `inputs:`-completeness claim, for ANY BC/ADR/VP, should default to the grep-complete method from the first authoring pass, not escalate to it only after 3+ read-through misses. This is the THIRD distinct convergence-technique discipline this gate's history has produced, alongside the version-stable-directive fix (O-P28-002, D-1085 — restructure a self-referential literal into a live-field reference) and the 4-leg head==version parity check (D-1089 — a mechanical frontmatter self-consistency gate). Like the D-1089 lesson, this is a `[process-gap]`, not a `[content-defect]`: the underlying omissions were genuine content gaps, but the RECURRENCE across three nominally-complete audits is a defect in the AUDIT METHOD itself, not in the spec content each audit was checking. No new mechanical validator is anchored by this lesson (unlike D-1089's `validate-modified-head-parity` follow-up) — the fix here is a PROCESS discipline (grep-complete audit method), applicable by any agent performing this class of audit, not a candidate for automated enforcement via a WASM guard, since "is this file a load-bearing current-state citation vs. a forward-only sync instruction" requires judgment a mechanical hook cannot exercise; the grep step (candidate enumeration) is mechanical, but the disposition step (load-bearing or not) remains a human/agent judgment call, faithfully recorded in the audit table. `[codified; process-gap; inputs-completeness; grep-complete-audit; read-through-insufficient; mechanical-enumeration; F-P33-001; D-1090; adr-046-gate; recurrence-class; third-convergence-technique; audit-method-not-content]`

---

**[convergence-confirmation][codified] The three convergence-technique disciplines codified across this gate's history (version-stable directive O-P28-002/D-1085, 4-leg `modified:`-array head==version parity D-1089, GREP-COMPLETE mechanical inputs-completeness audit D-1090), applied together and proactively from the start of a pass, are now empirically confirmed — not merely hypothesized — to be sufficient to drain the asymptotic metadata floor: pass-34 is the FIRST literal zero-finding CLEAN result this gate has produced across its 34-pass history.**

At D-1091 (ADR-046 pass-34), fresh-context adversary review against the unchanged frozen set (ADR-046 v1.16 + BC-4.17.001 v1.15 + BC-5.40.001 v1.14 + BC-7.07.001 v1.33) returned **VERDICT: CLEAN — zero findings at any severity**, the first such result in this gate's history. This is significant because the metadata/hygiene layer — as distinct from the behavioral core, which had already been stable since pass-27 — had produced a genuine finding on EVERY SINGLE prior pass from 27 through 33 (7 consecutive passes: F-P27-001/002/003, F-P28-001/002, F-P29-001/002/003, F-P30-001/002, F-P31-001/002, F-P32-001, F-P33-001), each traced to one of three recurring shapes: (1) a sibling-sweep gap (a fix applied to one artifact never propagated to siblings sharing the identical claim), (2) a `modified:`-array-head omission on version bump, or (3) an `inputs:`-completeness gap surviving a nominally "complete" human read-through. Each of these three shapes was, in turn, closed by a specific codified discipline: sibling-sweep gaps by the D-1087/D-1088 comprehensive-per-dimension-sweep technique; `modified:`-array-head omissions by the D-1089 mandatory 4-leg self-check; `inputs:`-completeness gaps by the D-1090 GREP-COMPLETE mechanical audit method. Pass-34 is the first pass to apply all three disciplines PROACTIVELY — as the DEFAULT review method from the start, not as a reactive technique-change triggered by a fresh finding mid-pass — and it is the first pass to return CLEAN.

**Disposition:** This is recorded as `[convergence-confirmation]`, distinct from the `[codified]`-only entries above: those entries codified a NEW discipline in response to a fresh finding; this entry confirms an ALREADY-codified set of disciplines, taken together, actually closes the class they were each designed to close, rather than merely reducing its recurrence rate. Per BC-5.39.001, one CLEAN pass is not itself convergence — the streak requires 3 CONSECUTIVE clean passes (34, 35, 36), and any finding on pass 35 or 36 would reset the streak to 0/3 and reopen the question of whether a fourth, still-undiscovered defect class exists in this metadata layer. The correct disposition for passes 35 and 36 is therefore to CONTINUE applying all three disciplines proactively (not to relax review rigor on the theory that the layer is "probably clean now") — the value of this confirmation is that it validates the TECHNIQUE, not that it renders further verification unnecessary. If passes 35 and 36 also return CLEAN, this generalizes into a standing recommendation: any future ADR/BC spec-convergence gate on this or other subsystems should apply comprehensive-sibling-sweep, 4-leg-parity, and grep-complete-inputs-audit as its DEFAULT review posture from pass 1, rather than discovering the need for each incrementally through a multi-pass recurrence-then-codify cycle as this gate did. `[convergence-confirmation; codified; empirical-confirmation; three-disciplines; comprehensive-sweep; 4-leg-parity; grep-complete-audit; first-clean-pass; D-1091; adr-046-gate; pending-pass-35-36-confirmation; not-yet-converged]`

---

**[codified][process-gap] Comprehensive cross-anchor audits on this gate had only ever validated `BC→BC §Section` citations, never `ADR §Decision`/`§N.M` citations against the cited ADR's own section content — these are two structurally distinct citation classes, and a clean result on one does NOT imply the other is clean.**

At D-1092 (ADR-046 pass-35), fresh-context adversary review against the pass-34 CLEAN frozen set found F-P35-001 (HIGH): 3 loci across 2 companion BCs (BC-4.17.001 §Precondition 4, BC-5.40.001 §Precondition 6, BC-5.40.001 §Architecture Anchors) cited `ADR-025 §Decision 12 §12.5` for the 256 KiB `STATE_MD_MAX_BYTES` cap — verified against ADR-025's own text: §Decision 12 §12.5 is "Shared parse logic — no duplication," stating no byte-cap value at all; the decision that actually raised the cap is §Decision 14. Every prior comprehensive cross-anchor audit this gate has run — the D-1088 cross-anchor semantic audit (BC-5.40.001 PC1/PC2's "BC-6.23.001 PC3/PC4" mis-cite, corrected to PC4-only) and the D-1090/D-1091 grep-complete `inputs:`-completeness audits — checked BC-to-BC `§Section`/`PCn`/`Invariant-N` references and `inputs:`-array completeness exhaustively, and correctly found those clean. None of them independently re-derived, from the cited ADR's own section content, whether an `ADR-NNN §Decision N`/`§N.M` citation names the CORRECT decision number — they verified the BC's paraphrase of the cited decision's CONTENT was accurate (and it was, for the 262144 figure itself: the VALUE was right at all 3 loci throughout, only the decision NUMBER attributing it was wrong), which is a different check than opening ADR-025 and confirming `§Decision 14`, not `§Decision 12 §12.5`, is the decision that says so.

**Disposition:** CODIFIED this burst as a MANDATORY discipline: comprehensive cross-anchor audits MUST validate BOTH citation classes — (1) `BC→BC §Section`/`PCn`/`Invariant-N` cross-references (the class every prior audit on this gate already covers) AND (2) `ADR §Decision N`/`§N.M` citations against the cited ADR's own section content (the class this pass reveals as previously unaudited) — as two DISTINCT checks, not one check assumed to cover the other. A `BC→BC` citation audit confirms the CITING BC's characterization of another BC's section is accurate; an `ADR §Decision` anchor audit confirms the citing artifact named the correct decision NUMBER within the cited ADR — these can diverge independently, as this pass demonstrates (the 262144 figure's characterization was accurate at all 3 loci; only the source decision-number attribution was wrong). Architect additionally ran this new dimension against ADR-046 itself (its sole cross-ADR anchor, ADR-025 §Decision 12 §12.2, confirmed correct) and product-owner ran it against BC-7.07.001 (clean) — both confirmed no further mis-anchors exist across the frozen set on this dimension. This is the FOURTH distinct convergence-technique discipline this gate's history has produced, alongside the version-stable-directive fix (O-P28-002, D-1085), the 4-leg head==version parity check (D-1089), and the GREP-COMPLETE mechanical inputs-completeness audit (D-1090) — like D-1090, this is a `[process-gap]` in the AUDIT METHOD, not a `[content-defect]` in what any single audit checked; the underlying citation gap was a genuine content defect, but the fact that 8+ prior comprehensive audits never covered this citation CLASS at all is the process gap. No mechanical validator hook is anchored by this lesson (unlike D-1089's `validate-modified-head-parity` follow-up) — confirming "does `§Decision N` in the citing artifact match the actual decision-number that states this fact in the cited ADR" requires opening and semantically reading the cited ADR's section, which is a judgment call no WASM guard can exercise without full-document natural-language comprehension. `[codified; process-gap; adr-decision-anchor-audit; semantic-anchoring-integrity; two-distinct-citation-classes; F-P35-001; D-1092; adr-046-gate; fourth-convergence-technique; audit-dimension-not-content]`

---

**[process-observation] The gate reached its first literal-CLEAN result at pass-34 (streak 1/3) and RESET at pass-35 on a newly-revealed audit dimension — this is empirical confirmation, not merely a hypothesis, that the asymptotic-floor reality described at D-1091 holds even immediately following a zero-finding CLEAN pass, and is decision-relevant for the human's continue-vs-accept-provisional choice.**

D-1091 recorded pass-34 as the first literal zero-finding CLEAN result this gate has produced across 34 passes, and explicitly flagged the confirmation as "provisional pending passes 35/36 also returning CLEAN" — not a closure. Pass-35 (D-1092) is exactly the scenario that provisional framing anticipated: a finding on the very next pass, on a dimension no prior pass (including pass-34's own thorough re-verification of "every dimension this gate's history has ever found a defect in") had ever audited. The substance stayed clean throughout — both F-P35-001 and F-P35-002 are the same cap-migration-lineage citation-cluster defect, not a behavioral or write-composition regression, and pass-34's zero-finding result on every dimension it DID check remains valid and unregressed. What changed is the SCOPE of what gets checked, not the quality of what was previously verified.

**Disposition:** This is recorded as `[process-observation]`, not a fix and not itself a codification (the codification is the preceding `[codified][process-gap]` entry) — it is a meta-level observation about the SHAPE of convergence on this gate, offered as decision-relevant context for the human's standing choice between continuing to loop toward literal 3-CLEAN (BC-5.39.001's strict discipline) versus accepting D-386 Option C asymptotic acceptance for this gate. The empirical pattern now on record: 34 passes to reach the first zero-finding result, immediately followed by a reset on a dimension that a full 34-pass history — including three previously-codified comprehensive-audit disciplines — had never covered. This does not prove convergence is unreachable; it demonstrates that each codified discipline closes the SPECIFIC class it targets, and that "zero findings against every dimension a discipline currently checks" is not equivalent to "zero findings against every dimension that could exist" — new dimensions can and do surface even after codified disciplines are applied proactively and a clean result is achieved. The correct disposition remains to CONTINUE looping under manual discipline (per the standing human decision recorded at D-1087 and reaffirmed through D-1091), now applying FOUR convergence-technique disciplines proactively (comprehensive-sibling-sweep, 4-leg-parity, grep-complete-inputs-audit, ADR-§Decision-anchor-audit) rather than three — but the human should weigh this pattern explicitly rather than assume the next clean-streak attempt is categorically more likely to reach literal 3-CLEAN than the last one was. `[process-observation; asymptotic-floor; empirical-confirmation; streak-reset; clean-then-reset; D-1091; D-1092; adr-046-gate; human-decision-relevant; not-a-fix; not-itself-a-codification]`

---

**[convergence-progress][codified] Pass-36 re-applied all FOUR now-codified convergence-technique disciplines proactively — including the ADR-anchor audit that reset the streak just one pass prior — and found the fourth dimension fully drained across the whole frozen set, not merely the 3 loci pass-35 explicitly fixed; streak ADVANCES 0/3 → 1/3, the SECOND clean pass this gate has produced.**

At D-1093 (ADR-046 pass-36), fresh-context adversary review against the newly-frozen set (ADR-046 v1.16 + BC-4.17.001 v1.16 + BC-5.40.001 v1.15 + BC-7.07.001 v1.33) — the set produced by the pass-35 fix burst — returned **VERDICT: CLEAN — zero findings at any severity**. This directly follows the pass-35 reset that revealed the ADR §Decision/§N.M anchor dimension as a previously-unaudited citation class (D-1092). Pass-36 independently re-derived, from each cited ADR's own section content, every `ADR-NNN §Decision N`/`§N.M` citation across the frozen set — not just the 3 loci pass-35 corrected — including ADR-046's own sole cross-ADR anchor and BC-5.40.001's separate `§Decision 7` citation, and found zero mis-anchors anywhere. All three previously-confirmed disciplines (version-stable directive, 4-leg parity, grep-complete inputs audit) were also re-verified holding, with no regression.

**Disposition:** This is recorded as `[convergence-progress]`, distinct from D-1091's `[convergence-confirmation]` (which confirmed three disciplines together sufficient against the dimensions then known) and from D-1092's `[codified][process-gap]` (which introduced the fourth discipline in response to a fresh finding): this entry is the first direct EVIDENCE that the fourth discipline, applied proactively, closes the class it targets the way the first three closed theirs — evidence, not yet proof, since a single clean pass is not itself a confirmation the way three consecutive passes would be. Per BC-5.39.001, this is 1 of 3 required clean passes counting from the pass-35 reset — the streak requires 2 further CONSECUTIVE clean passes (37, 38), and any finding on either would reset the streak to 0/3 again and reopen the question of whether a fifth, still-undiscovered defect class exists. The correct disposition for passes 37 and 38 is to CONTINUE applying all four disciplines proactively without relaxing rigor on the theory that "the anchor dimension is now proven safe" — one clean pass on a freshly-codified discipline is progress, not closure. `[convergence-progress; codified; four-disciplines; adr-decision-anchor-audit; second-clean-pass; D-1093; adr-046-gate; pending-pass-37-38-confirmation; not-yet-converged]`

---

**[codified][process-gap] Fix-burst disposition prose that makes a sweeping self-attested completeness claim ("read in full, all correct") is itself falsifiable attack surface for a fresh-context adversary — pass-37 found the pass-35 remediation's OWN audit-narrative miscounted ADR-046's Decision list (asserted 1–5, actual 1–6); MITIGATION now in force: disposition prose must be MINIMAL and factual, and self-attested audits need mechanical (greppable) backing, not a bare completeness assertion.**

At D-1094 (ADR-046 pass-37), fresh-context adversary review against the SAME unchanged frozen set (ADR-046 v1.16 + BC-4.17.001 v1.16 + BC-5.40.001 v1.15 + BC-7.07.001 v1.33) found F-P37-001 (MED): both BC-4.17.001's and BC-5.40.001's own `modified:`/`last_amended`/Changelog prose from the pass-35 remediation (D-1092) asserted ADR-046's `## Decision` section is "a flat list, 1–5, ... read in full, all correct" — but ADR-046 actually has 6 numbered decisions; item 6 (same-release ship + CI-gating registry-invariant XOR check) was silently dropped from the count. Every ACTUAL `ADR-046 Decision N` citation in both BCs' live body text remained correctly numbered and unaffected — this defect lives entirely inside the remediation's own summary/bookkeeping narrative, not in any operative spec content. O-P37-001 ([process-gap], LOW) additionally observes that nothing mechanically verifies a "read in full" claim against the cited artifact's actual section-list cardinality before the claim is written down — the gap that let this miscount stand for two full passes (36's CLEAN re-verification checked citation NUMBERS against ADR-046's text but did not independently re-derive the cited artifact's own item COUNT from its narrative summary).

**Disposition:** CODIFIED as a mandatory discipline for all future ADR-046 gate remediation bursts (and, by extension, any fix-burst disposition prose on this project that asserts audit completeness): (1) disposition prose describing a "read in full" or "comprehensive audit" claim MUST be MINIMAL and factual — state what was checked and what was found, without a sweeping certification of exhaustiveness beyond what was actually mechanically verified; (2) any prose that states a cardinality (a count of items, decisions, sections, citations) as part of an audit claim MUST be independently greppable/countable at write-time (e.g., `grep -cE '^### Decision [0-9]+' ADR-046.md` before asserting "1–5" or "1–6") rather than trusted from memory or from a prior pass's own narrative. This is distinct from D-1090's `[codified][process-gap]` (GREP-COMPLETE inputs-completeness audits) and D-1092's (ADR §Decision anchor citation-NUMBER correctness) — this lesson targets the CARDINALITY/COUNT claim inside disposition prose itself, a third structurally-distinct failure mode alongside wrong-citation-target and incomplete-inputs-array. `[codified; process-gap; self-attested-completeness-claim; falsifiable-narrative; mechanical-backing-required; F-P37-001; O-P37-001; D-1094; adr-046-gate; disposition-prose-discipline]`

---

**[process-observation] ASYMPTOTIC-FLOOR meta-observation STRENGTHENED: the gate has now reached 1/3 twice (pass-34, pass-36) and RESET twice (pass-35 on the ADR-anchor dimension, pass-37 on that same dimension's OWN remediation-prose bookkeeping) — the second reset came from the remediation's own bookkeeping rather than a fresh spec-vs-code defect, empirically confirming that prose-only codification leaves literal 3-CLEAN structurally fragile even when the underlying discipline is sound.**

D-1091 (pass-34) and D-1093 (pass-36) each reached the gate's zero-finding CLEAN result, both explicitly flagged provisional. D-1092 (pass-35) reset the streak on a genuinely NEW audit dimension (ADR §Decision/§N.M anchor correctness) no prior pass had covered. D-1094 (pass-37) reset the streak AGAIN — but this time not on a new dimension: the finding is a miscount INSIDE the pass-35 remediation's own narrative describing that same fourth dimension. This is a qualitatively different failure mode from pass-35's: pass-35 demonstrated "a codified discipline can miss a class it doesn't yet check"; pass-37 demonstrates "even after a discipline is codified and correctly applied to the underlying substance, the REMEDIATION'S OWN bookkeeping narrative describing that application is itself a fresh source of falsifiable claims a later adversary can catch." Cf. the F5-cycle's own META-LEVEL taxonomy (L-EDP1-007/051/061): "prose-only codification → literal 3-CLEAN structurally fragile" is not a hypothesis specific to the F5 cycle's dimension-count — it recurs here, on an entirely different gate (ADR-046 vs. F5's engine-discipline cycle), via an entirely different concrete mechanism (decision-count miscitation vs. F5's meta-level-ply taxonomy gaps). This is now empirically observed on TWO separate gates in this project, strengthening the claim from gate-specific anecdote toward a general property of the BC-5.39.001 3-CLEAN discipline under prose-only (non-mechanically-enforced) codification.

**Disposition:** Recorded as `[process-observation]`, not a fix — decision-relevant context for the human's standing continue-vs-accept-provisional choice, re-surfaced at this decision point. **The human RE-AFFIRMED "CONTINUE looping toward literal 3-CLEAN" at this decision point** — accept-provisional under D-386 Option C was offered and declined again, the second such explicit reaffirmation this session (the first at the pass-35 reset). The correct disposition going forward is unchanged in kind but now reinforced by the D-1094/lessons `[codified][process-gap]` mitigation above (minimal, factual, mechanically-backed disposition prose) — applying it should reduce (though not provably eliminate) the rate at which a remediation's OWN narrative becomes the next pass's finding. `[process-observation; asymptotic-floor; strengthened; second-gate-confirmation; streak-reset; clean-then-reset-twice; D-1091; D-1092; D-1093; D-1094; adr-046-gate; f5-cycle-parallel; human-reaffirmed-continue; human-decision-relevant; not-a-fix]`

---

**[convergence-progress][codified] Pass-38 re-applied every now-codified convergence-technique discipline proactively — including BOTH dimensions whose discovery caused this session's two resets (ADR-anchor correctness at pass-35, self-attested cardinality claims at pass-37) — and found the entire frozen set clean on every dimension; streak ADVANCES 0/3 → 1/3, the THIRD clean pass this gate has produced this session.**

At D-1095 (ADR-046 pass-38), fresh-context adversary review against the SAME unchanged frozen set (ADR-046 v1.16 + BC-4.17.001 v1.17 + BC-5.40.001 v1.16 + BC-7.07.001 v1.33) — the set produced by the pass-37 fix burst — returned **VERDICT: CLEAN — zero findings at any severity**. This is the first pass to directly re-verify BOTH previously-reset dimensions in the same review: the ADR §Decision/§N.M anchor-correctness class (D-1092/pass-35's dimension) and the self-attested cardinality/completeness-claim class (D-1094/pass-37's dimension, including an independent recount of ADR-046's own `## Decision` section confirming 6 items against both BCs' now-corrected "1–6" prose). Every other previously-codified dimension (version-stable directive, 4-leg parity, grep-complete inputs audit) was also re-verified holding, with no regression.

**Disposition:** Recorded as `[convergence-progress]`, the same tag used for D-1093's pass-36 confirmation, but distinguished from it by scope: D-1093 confirmed the ADR-anchor dimension alone (immediately after its own reset); this entry confirms BOTH reset dimensions simultaneously, on the same pass, against the same frozen set that triggered the second reset. This is the first direct EVIDENCE that the D-1094 mitigation (minimal, factual, mechanically-backed disposition prose) holds under independent fresh-context re-derivation — evidence, not yet proof, since a single clean pass is not itself a confirmation the way three consecutive passes would be. Per BC-5.39.001, this is 1 of 3 required CONSECUTIVE clean passes — the streak requires 2 further consecutive clean passes (39, 40), and any finding on either would reset the streak to 0/3 again. The correct disposition for passes 39 and 40 is to CONTINUE applying all convergence-technique disciplines proactively without relaxing rigor on the theory that "both reset dimensions are now proven safe" — one clean pass covering two previously-reset dimensions is progress, not closure. `[convergence-progress; codified; both-reset-dimensions-reconfirmed; third-clean-pass; D-1095; adr-046-gate; pending-pass-39-40-confirmation; not-yet-converged]`

---

**[codified][process-gap][convergence-observation] Arm-parity sibling-sweep + substantive-vs-metadata reset distinction — pass-39's finding was a genuine data-destructive internal contradiction in BC-4.17.001's own operative Precondition/Invariant text, the FIRST substantive (not metadata/prose) reset this gate has produced, surfacing a sixth distinct convergence-technique discipline: what-vs-how reconciliations must sweep to every analogous sibling arm/case in the same burst.**

At D-1096 (ADR-046 pass-39), fresh-context adversary review against the frozen set produced by
pass-38 (ADR-046 v1.16 + BC-4.17.001 v1.17 + BC-5.40.001 v1.16 + BC-7.07.001 v1.33) found F-P39-001
(MED): BC-4.17.001's Precondition 4 and Invariant 7 mandated `extract_frontmatter`-slice confinement
for BOTH the `timestamp:` re-stamp arm AND the `expires_at` renewal arm, directly contradicting
Precondition 2's/Invariant 9's own requirement that `renew_lock_if_holder` be fed the FULL
`content_after_pc1` for the single composed `host::write_file`. A literal reading of the `expires_at`
arm's slice-exclusivity directive would have truncated `RenewOutcome::Renewed(new_content)` to the
frontmatter region on write — since that truncated value becomes the entire composed write, this
would have DESTROYED STATE.md's body content on a live write. This defect survived 37 prior passes
(including the Pass-16/O-P16-001 fix that reconciled the analogous what-vs-how tension for PC4's
OTHER case — the `timestamp:` arm) because no prior pass's audit checklist included "when a
what-vs-how reconciliation is applied to one arm/case, check whether every sibling arm/case with
analogous language received the same reconciliation" as a discrete check.

**Part (a) — `[codified][process-gap]`:** when a what-vs-how (semantic-scope vs. mechanism)
reconciliation is applied to ONE arm/case of a contract, ALL sibling arms/cases carrying analogous
language MUST receive the same reconciliation in the SAME burst. This is the arm-parity variant of
the sibling-sweep discipline already codified for callsites (TD-VSDD-060) and for BC clusters
(D-1087/D-1088's comprehensive-sweep convergence-strategy lessons) — but operating at a FINER
granularity: not "every BC that shares this defect class," but "every CLAUSE-ARM within the SAME BC
that uses analogous language to the clause just reconciled." The Pass-16 fix reconciled PC4's
`timestamp:`-arm what-vs-how tension (frontmatter-slice-directive vs. actual write-mechanism scope)
but never checked whether Invariant 7's PARALLEL `expires_at`-arm language carried the identical
tension — it did, and it went unreconciled for 23 further passes (16 through 38) until a
fresh-context adversary traced the full write-composition data flow from first principles rather
than re-running any previously-codified checklist item.

**Part (b) — `[process-observation][convergence-observation]`:** this is the gate's THIRD reset this
session (after pass-35 and pass-37), but the FIRST that is SUBSTANTIVE rather than metadata/prose.
Pass-35's reset was a citation-accuracy gap (wrong ADR §Decision NUMBER, correct underlying value).
Pass-37's reset was a bookkeeping miscount INSIDE a prior remediation's own narrative prose (no
operative-content risk at all — the actual `ADR-046 Decision N` citations in live body text were
never wrong). **Pass-39's finding is neither of those shapes** — it is a genuine unreconciled
contradiction in the BC's own live Precondition/Invariant text that, if shipped into S-17.05's TDD
implementation, would have produced a real data-truncation bug on the very first `expires_at`
renewal write. This is decision-relevant evidence for the human's standing continue-vs-accept-
provisional choice (§5 Pending Human Decision): the two prior resets could be read as the gate
"finding diminishing returns" on prose-level polish; this reset demonstrates the OPPOSITE — the gate
is still finding genuine, consequential defects a fresh adversary's first-principles trace can
surface that no accumulated checklist of previously-codified disciplines would have caught, because
each codified discipline targets the SPECIFIC failure shape its own discovery pass revealed, not the
general class "any two Preconditions/Invariants governing the same composed write may silently
diverge in scope."

**Disposition:** Recorded as `[codified][process-gap]` for part (a) — the arm-parity sibling-sweep
discipline is now in force for all future ADR-046 gate remediation bursts and any other BC exhibiting
a what-vs-how reconciliation pattern across multiple arms/cases — and `[process-observation]
[convergence-observation]` for part (b) — recorded for the human's decision-relevance, not itself a
fix. The human's standing choice (CONTINUE looping toward literal 3-CLEAN, re-affirmed twice already
this session) is not altered by this entry; this entry supplies additional evidence FOR that choice's
continued soundness, since a substantive defect just surfaced at pass-39, the same discipline that
found and fixed 30 prior genuine findings across the gate's history. `[codified; process-gap;
convergence-observation; arm-parity; sibling-sweep; what-vs-how; substantive-reset;
metadata-vs-substantive; D-1096; adr-046-gate; f-p39-001; third-reset; sixth-discipline]`

---

**[codified][process-gap] Pass-40's F-P40-001 empirically VALIDATED the D-1096 arm-parity
sibling-sweep codification — and simultaneously revealed the codification's own scope was
under-specified: the pass-39 fix swept Preconditions/Invariants but missed the sibling
VERIFICATION PROPERTY carrying the identical guarantee**

At D-1097 (ADR-046 pass-40), fresh-context adversary review against the frozen set produced by the
pass-39 fix (ADR-046 v1.16 + BC-4.17.001 v1.18 + BC-5.40.001 v1.16 + BC-7.07.001 v1.33) found
F-P40-001 (MED): BC-4.17.001's VP-TBD-8 — the §Verification Properties table row covering the
identical `extract_frontmatter`-use guarantee Precondition 4/Invariant 7 govern — still lumped the
`timestamp:` and `expires_at` arms under one joint frontmatter-slice confinement, re-encoding the
exact pre-F-P39-001 framing the Precondition/Invariant pair was corrected away from one pass
earlier. This is not a new class of defect independent of pass-39 — it is the SAME defect,
un-swept at a sibling locus, and it is the first direct empirical test of whether the D-1096
arm-parity sibling-sweep codification (recorded one pass ago) actually holds under fresh-context
re-derivation. The answer is: PARTIALLY. The codification's own text ("every analogous sibling
arm/case") was interpreted narrowly at v1.18 to mean the Precondition/Invariant pair alone — the
locus class the pass-39 finding itself was discovered in — and did not extend the sweep to other
locus CLASSES (§Verification Properties rows, Architecture Anchors, SDK-grounding blocks) that
carry the same guarantee in different prose form.

**Extension to the D-1096 codification (this entry's substantive content):** arm-scope/what-vs-how
reconciliations MUST sweep to ALL loci carrying the guarantee, not just the locus class where the
reconciliation was first discovered. Concretely, for this BC-cluster's recurring pattern, the sweep
checklist now explicitly enumerates: Preconditions, Postconditions, Invariants, §Verification
Properties rows, Architecture Anchors, and SDK-grounding blocks. The pass-40 comprehensive 8-locus
sweep (Precondition 4, Invariant 7, PC1's rewrite-mechanism paragraph, PC3a, PC4, Invariant 5, Edge
Cases, Canonical Test Vectors, Architecture Anchors, Description, and VP-TBD-8 itself) is the model
going forward: when in doubt about how far a sibling-sweep must reach, grep the artifact for every
mention of the load-bearing term/mechanism name (here, `extract_frontmatter`) and classify each hit
as ALREADY-CORRECTED, GENUINELY-UNAFFECTED, or STRAGGLER — do not stop at the locus class where the
original finding was discovered.

**Disposition:** Recorded as `[codified][process-gap]`. This is the SEVENTH distinct
convergence-technique discipline this gate has produced (after: (1) comprehensive-sibling-sweep
D-1087/D-1088; (2) 4-leg `modified:`-array head==version parity D-1089; (3) GREP-COMPLETE
`inputs:`-completeness audit D-1090; (4) ADR `§Decision`/`§N.M` anchor audit D-1092; (5)
minimal-prose + mechanical-audit-backing discipline D-1094; (6) arm-parity what-vs-how
reconciliation sweep D-1096; (7) this entry's locus-class extension of (6)). It is also
decision-relevant evidence of a DIFFERENT shape than D-1096's part (b): rather than showing the
gate finds NEW classes of defect, pass-40 shows the gate catches INCOMPLETE APPLICATION of its own
most-recently-codified discipline — meta-level evidence that codified disciplines need their own
follow-up verification pass, not just a one-time application, before being trusted as fully closed.
`[codified; process-gap; arm-parity; sibling-sweep; verification-properties; locus-class-extension;
what-vs-how; D-1097; adr-046-gate; f-p40-001; seventh-discipline; meta-verification]`

---

**[convergence-progress][codified] Pass-41 re-applied every now-codified convergence-technique
discipline proactively — including BOTH the sixth (arm-parity sweep, D-1096) and seventh
(locus-class extension, D-1097) disciplines together, against the exact frozen set those two fixes
themselves produced — and found the entire frozen set clean on every dimension; streak ADVANCES
0/3 → 1/3, the FOURTH clean pass this gate has produced this session**

At D-1098 (ADR-046 pass-41), fresh-context adversary review against the newly-frozen set (ADR-046
v1.16 + BC-4.17.001 v1.19 + BC-5.40.001 v1.16 + BC-7.07.001 v1.33) — the set produced by the pass-40
fix burst — returned **VERDICT: CLEAN — zero findings at any severity**. This is the first pass to
directly re-verify BOTH the arm-parity what-vs-how reconciliation dimension (D-1096/pass-39's
dimension) and its locus-class extension (D-1097/pass-40's dimension) in the same review, including
an independent re-derivation of all eleven `extract_frontmatter`-guarantee loci across BC-4.17.001
(Precondition 4, Invariant 7, VP-TBD-8, PC1's rewrite-mechanism paragraph, PC3a, PC4, Invariant 5,
Edge Cases, Canonical Test Vectors, Architecture Anchors, Description) — all confirmed consistently
arm-split, with VP-TBD-8 specifically re-opened and confirmed correct. Every other previously-codified
dimension (ADR §Decision anchor correctness, self-attested cardinality claims, version-stable
directive, 4-leg parity, grep-complete inputs audit) was also re-verified holding, with no
regression.

**Disposition:** Recorded as `[convergence-progress]`, the same tag used for D-1095's pass-38
confirmation, but distinguished from it by scope: D-1095 confirmed the pass-35/pass-37 dimensions
(citation-accuracy and bookkeeping-cardinality) together; this entry confirms the pass-39/pass-40
dimensions (arm-parity and its locus-class extension) together, the two MOST RECENTLY codified
disciplines, on the same pass, against the frozen set those two fixes themselves produced. This is
the first direct EVIDENCE that the D-1096/D-1097 pair — arguably the most consequential disciplines
this gate has produced, since D-1096 closed a genuine data-destructive hazard — holds under
independent fresh-context re-derivation. Evidence, not yet proof, since a single clean pass is not
itself a confirmation the way three consecutive passes would be. Per BC-5.39.001, this is 1 of 3
required CONSECUTIVE clean passes — the streak requires 2 further consecutive clean passes (42, 43),
and any finding on either would reset (or, if already zero, hold) the streak. The correct
disposition for passes 42 and 43 is to CONTINUE applying all seven convergence-technique disciplines
proactively without relaxing rigor on the theory that "the two most recent dimensions are now proven
safe" — one clean pass covering two previously-found dimensions is progress, not closure.
`[convergence-progress; codified; sixth-and-seventh-dimensions-reconfirmed; fourth-clean-pass;
D-1098; adr-046-gate; pending-pass-42-43-confirmation; not-yet-converged]`

---

**[convergence-governance] At 2/3, a pre-existing dated-historical cosmetic observation is accepted
as documentary-historical-deferred rather than fixed, because touching the frozen set to fix a
non-defect out of the feature perimeter would reset a live convergence streak — the correct
governance call is accept-and-track, not fix-and-reset**

At D-1099 (ADR-046 pass-42), fresh-context adversary review against the SAME unchanged frozen set
(ADR-046 v1.16 + BC-4.17.001 v1.19 + BC-5.40.001 v1.16 + BC-7.07.001 v1.33) returned **VERDICT:
CLEAN — zero BLOCKING findings**, the SECOND CONSECUTIVE clean pass (following pass-41), advancing
the BC-5.39.001 streak 1/3 → 2/3. The pass also surfaced ONE non-blocking observation, O-P42-001: a
pre-existing cosmetic asymmetry in BC-5.40.001's oldest `modified:` frontmatter array entries
(v1.1–v1.4 bare version/date strings vs. v1.5–v1.16 full disposition prose) — a defect that predates
ADR-046 entirely, breaks no head-parity check, and carries no operative risk.

The disposition question this pass raised: should O-P42-001 be fixed in-scope (per the project's
general production-grade default), or accepted as a tracked non-blocking item? The answer is
accept-and-track, for a reason specific to this gate's mechanics: fixing O-P42-001 requires editing
BC-5.40.001 — one of the four artifacts in the FROZEN set this streak's byte-unchanged invariant
depends on. Editing ANY of the four frozen artifacts, for ANY reason, resets the convergence context
this streak is measuring (the next pass would then be reviewing a DIFFERENT frozen set, and the
2/3 progress toward literal 3-CLEAN would no longer be "3 consecutive clean passes against the same
set" — it would be a new streak against a new set). The observation itself is out of the feature
perimeter (a pre-existing dated-historical row, not something ADR-046 introduced or is responsible
for) and non-blocking (no head-parity violation, no propagation gap, no operative risk). Under these
specific conditions — pre-existing, out-of-perimeter, non-blocking, and fixing it would cost a live
convergence streak for no correctness gain — the correct call is to accept it as a tracked item
(recorded in STATE.md Drift Items, anchored to a concrete future touch-point: the next maintenance
sweep or the S-15.03 PRIORITY-A historical-row backfill automation) rather than fix it now.

**Disposition:** Recorded as `[convergence-governance]`, a NEW tag distinguishing this class of
decision (a disposition call about WHETHER to fix something found mid-streak) from
`[convergence-progress]` (evidence a discipline holds) and `[codified][process-gap]` (a new
convergence-technique discipline). This is not a general license to defer fixable defects found
during this gate's passes — the default remains fix-in-scope. The narrow condition that justifies
deferral here is the CONJUNCTION of all four factors above (pre-existing, out-of-perimeter,
non-blocking, and touching the frozen set would cost the streak) — any BLOCKING finding, or any
observation that IS caused by or contemporaneous with ADR-046's own content, must still be fixed
same-burst per this gate's established practice (F-P37-001 through F-P40-001 were all fixed
same-burst, none deferred). The correct disposition for pass-43 is to continue applying all seven
convergence-technique disciplines proactively, treating O-P42-001 as an already-accepted item (not
re-litigating it as a fresh finding), and to fix in-scope any NEW blocking finding pass-43 might
surface (which would reset the streak, as any genuine finding always has).
`[convergence-governance; accept-and-track; documentary-historical-deferred; O-P42-001; streak-2of3;
D-1099; adr-046-gate; frozen-set-preservation; out-of-perimeter]`

---

**[codified][process-gap] Grep-complete mechanical inputs audits (D-1090) must be applied to
EVERY artifact in a cluster, not just the anchor/ADR document — the BCs' own piecemeal inputs
fixes left capabilities.md (+3 more on BC-5.40.001) missing until pass-43, the FIRST pass to
scope the audit to the BCs themselves**

D-1090 (pass-33) codified that inputs-completeness audits must be GREP-COMPLETE mechanical
file-path-token enumeration, not human read-throughs — and every subsequent inputs-completeness
finding through pass-40 (F-P33-001, F-P30-002, F-P29-002, F-P31-001, F-P35-002, F-P40-001, etc.)
was found by running that discipline against ADR-046's own `inputs:` array, or against a single
BC in response to a specific triggering claim. What no prior pass had done was run the
grep-complete audit against EACH of the three companion BCs' own `inputs:` arrays as a routine,
comprehensive sweep — the audits were always triggered reactively (an adversary spotting one
specific missing citation) rather than run proactively and exhaustively per-artifact the way
D-1090 itself was.

At pass-43, product-owner ran the FIRST such comprehensive per-BC audit (prompted by F-P43-001's
discovery that all three BCs were missing `capabilities.md` from `inputs:` despite quoting their
capability's description verbatim) and immediately found 3 MORE genuinely-missing files on
BC-5.40.001 alone — files that had been load-bearing citations since as early as pass-29
(`inputs:` was last touched on BC-5.40.001 at v1.14/pass-31, nine passes before this gap was
caught). This is a structural gap in how the D-1090 discipline had been APPLIED, not a weakness in
the discipline's own definition: D-1090's text never restricted itself to "the ADR only," but every
prior application of it happened to be ADR-scoped or single-BC-triggered, and no burst had ever
explicitly run it as "audit ALL FOUR frozen-set artifacts' own `inputs:` arrays, mechanically,
every time."

**Disposition:** Extend the D-1090 codification's PRACTICE (not its definition — the mechanical
grep-complete method itself is unchanged) to explicitly require: when this gate's grep-complete
inputs audit runs, it MUST run against every artifact in the frozen/reviewed set — the ADR AND
every companion BC — not just the artifact a specific triggering finding happens to implicate.
This closes a class of gap where a defect in one artifact (missing `capabilities.md`) is fixed
locally without checking whether sibling artifacts in the same cluster carry the identical gap.
`[codified][process-gap][inputs-audit-scope-extension][D-1100; adr-046-gate; F-P43-001;
grep-complete; all-cluster-artifacts]`

---

**[content-defect-discipline] A cross-reference to an acceptance criterion must name the OWNING
artifact (story vs. BC) and anchor to the BC's actual normative locus, not assume the AC belongs
to whichever artifact happens to be citing it**

At pass-43, F-P43-002 found that ADR-046's Companion Amendment 3 (and BC-7.07.001's own v1.19
Changelog row, independently) both referred to "AC-018" as though it were a normative
acceptance-criterion section of BC-7.07.001 itself — "The BC's existing AC-018 ('If `Renewed`:
call `host::write_file`') is otherwise unchanged." A full section-heading sweep of BC-7.07.001
confirms it has no Acceptance Criteria section and no AC-NNN numbering scheme at all: this BC's
sections are Description / Preconditions / Postconditions / Invariants / Edge Cases / Canonical
Test Vectors / Related BCs / Architecture Anchors / Story Anchor / VP Anchors / Verification
Properties / Traceability / Changelog. AC-018 is in fact a STORY-level acceptance criterion,
belonging to `.factory/stories/S-18.04a-precompact-flush-sh-core.md`'s own `### AC-018` section —
which itself traces down INTO BC-7.07.001 at a real, citable, section-anchored locus: Postcondition
3 case 5 / Invariant 3 step 4. The label "AC-018" was not fabricated (it genuinely exists, at the
story level, and genuinely traces to the cited BC locus) — it was mis-scoped, attributed to the
wrong owning artifact, by BOTH the ADR and the BC that echo it.

**Disposition:** When a cross-reference names an acceptance criterion (AC-NNN) or any other
artifact-scoped identifier, the citing text must (a) verify which artifact actually OWNS that
identifier's numbering scheme (a story's AC-NNN list is distinct from a BC's own PC/Invariant/EC
numbering — a BC citing "AC-NNN" as if it were its own section is a category error unless that BC
demonstrably has its own AC-NNN scheme), and (b) anchor the citation at the OWNING artifact's
actual locus while ALSO naming the concrete downstream section (PC/Invariant/EC) in the cited BC
that the story-level AC traces to — both halves are required for the cross-reference to resolve to
something real on both ends. This is a distinct class from the AC-018 mis-scoping bug's own
symptom (a plausible-sounding identifier a reader would not think to double-check) and from
ADR-025 §Decision anchor mis-cites (D-1092) — this class is specifically about IDENTIFIER-OWNERSHIP
attribution across the story/BC boundary, not section-number accuracy within one document.
`[content-defect-discipline; ac-owning-artifact; cross-reference-integrity; D-1100; adr-046-gate;
F-P43-002; story-vs-bc-boundary]`

---

**[process-observation][convergence-observation] META: the 4th reset (at the convergence pass
itself) was SUBSTANTIVE/provenance, not gaming or noise — the BC inputs perimeter had never been
grep-audited before this pass, so finding it now is convergence PROGRESS, not a regression**

Pass-43 was dispatched specifically to CLOSE the gate at literal 3-CLEAN (streak was 2/3 entering
this pass). Instead it found 2 MED findings and reset the streak to 0/3 — the 4th reset this
session. It would be tempting to read a reset at exactly the convergence pass as evidence the gate
is not actually converging, or that findings are being manufactured to avoid closure. Neither
reading holds up: F-P43-001 is the FIRST-EVER application of the mandatory grep-complete inputs
audit to the three companion BCs' own `inputs:` arrays (as opposed to ADR-046's) — a genuinely new
DIMENSION this gate's 42 prior passes never checked, not a re-discovery of an already-checked
dimension regressing. F-P43-002 is a genuine cross-reference defect (AC-018 mis-scoping) that
existed since BC-7.07.001's v1.19 (pass-2-era) and was never independently caught because both the
ADR and the BC echoing each other created a false consensus — exactly the kind of two-way
mutual-reinforcement defect fresh-context adversarial review with information asymmetry exists to
catch. Both findings sit at the provenance/cross-reference perimeter, not the behavioral core,
which remains independently re-verified clean for 17 consecutive passes (since pass-27). **The
correct reading: this reset represents the gate finding a previously-unaudited dimension and
draining it — the BC inputs perimeter is now drained (all three BCs grep-complete-audited for the
first time), which is convergence PROGRESS, structurally identical to the D-1096/D-1097 pattern
(a new dimension found, fixed, then re-confirmed clean on subsequent passes) rather than evidence
against the gate's validity.** Fresh pass-44 starts a new streak with one more dimension now
provably clean than pass-43 started with.
`[process-observation; convergence-observation; substantive-reset; not-gaming; D-1100; adr-046-gate;
fourth-reset; bc-inputs-perimeter-drained; convergence-progress]`

---

**[content-defect-discipline] An illustrative "verbatim quote" attached to a fix's own
disposition prose must cite the ACTUAL text of the cited source, not the fix's own paraphrase or
justification sentence — check sibling-parity across every disposition narrative touched in the
same burst**

At pass-44, O-P44-001 found that BC-5.40.001's v1.17 `last_amended` disposition (the entry
recording pass-43's F-P43-001 `capabilities.md` `inputs:` fix) illustrated the newly-added
citation with a parenthetical claiming to quote CAP-031's verbatim description — but the quoted
text, `'this BC defines the authoritative lock state data structure...'`, was in fact this BC's
own Capability Anchor Justification sentence (this BC's argument for why CAP-031 is its correct
anchor), not any text appearing in `capabilities.md`'s CAP-031 entry itself. The underlying
`inputs:` fix was correct throughout — `capabilities.md` genuinely is load-bearing and CAP-031
genuinely is this BC's correct capability anchor — the defect is narrowly that the illustrative
quote-snippet attached to the disposition prose cited the wrong source.

What makes this catchable in-burst rather than only in hindsight: BC-4.17.001 v1.20 and
BC-7.07.001 v1.34 received the IDENTICAL class of fix in the SAME pass-43 burst (each adding
`capabilities.md` to `inputs:` with an illustrative quote of their own respective CAP's
description/title), and BOTH siblings' equivalent illustrative quotes were verbatim-accurate
against their cited capability's actual text. A sibling-parity check — comparing all three BCs'
analogous disposition paragraphs for the same fix class, in the same burst — would have caught
BC-5.40.001's misattribution immediately, since the correct pattern was directly available two
files over.

**Disposition:** When any fix's own disposition prose illustrates a claim with a "verbatim
quote" of a cited source (a capability description, an ADR §Decision text, another BC's PC/
Invariant text, etc.), the quote MUST be checked against the actual cited source's text, not
assumed correct because the surrounding reasoning (which source to cite, why it applies) is
sound. When the same fix class is applied to multiple sibling artifacts in the same burst, their
analogous illustrative quotes must be cross-checked against EACH OTHER as well as against the
source — a correct sibling pattern sitting two files away, unconsulted, is itself evidence the
check was skipped. This is the ninth distinct convergence-technique discipline this gate has
produced, and a specific sub-class of citation-accuracy discipline distinct from ADR §Decision
anchor correctness (D-1092, which concerns *which section* is cited) — this concerns whether the
*quoted text itself* is genuinely present in the cited section.
`[content-defect-discipline; illustrative-quote-verbatim-accuracy; sibling-parity-check; D-1101;
adr-046-gate; O-P44-001; ninth-discipline]`

---

**[convergence-governance] Fix-vs-accept disposition for a LOW non-blocking observation: FIX
when the streak is at 0/3 (zero cost) AND the item is a fresh, in-session, sibling-confirmed-
correctable defect; reserve accept-and-track for genuinely pre-existing, out-of-perimeter,
dated-historical items**

At D-1099 (pass-42), O-P42-001 (a pre-existing, pre-ADR-046 `modified:`-array documentary
asymmetry in BC-5.40.001's oldest historical rows) was correctly ACCEPTED-and-tracked rather
than fixed: the streak was at 2/3, fixing would have required editing one of the four frozen-set
artifacts, breaking the byte-unchanged invariant the 2/3 streak depended on and forfeiting 2
already-banked clean passes for a cosmetic item that predated this gate's entire history and
carried zero operative risk. That was the correct call under BC-5.39.001's discipline.

At D-1101 (pass-44), O-P44-001 presented superficially the same shape — a LOW, non-blocking
observation on BC-5.40.001 found by a CLEAN-otherwise adversary pass — but the governance facts
were materially different: (a) the streak was ALREADY at 0/3 entering this pass (pass-43's own
FINDINGS verdict had already reset it), so editing the frozen set here costs ZERO additional
streak — there was no banked-clean-passes value to protect; (b) O-P44-001 is a FRESH defect
introduced in THIS SESSION'S OWN pass-43 remediation prose, not inherited pre-existing history
predating the gate; and (c) the two sibling BCs' equivalent disposition prose, edited in the
SAME pass-43 burst, got the analogous quote right — proving the correct pattern was known and
simply unapplied to this one BC, not a genuinely ambiguous or hard-to-verify claim. Under
CLAUDE.md's Canonical Principle Rule 4 (AI-built defects are the AI's responsibility to fix; the
default action is to fix in scope, not defer), and given the fix cost nothing in streak terms,
the correct call here was FIX, not accept-and-track.

**Disposition:** When this gate (or any BC-5.39.001-governed convergence gate) finds a LOW
non-blocking observation, the governance call is FIX (not accept-and-track) when ALL of: (i) the
streak is already at 0/3 (so the fix costs no banked clean passes), AND (ii) the item is a
fresh, in-session-introduced, correctable inaccuracy rather than pre-existing dated-historical
content, AND (iii) — where applicable — a correct sibling pattern in the same burst demonstrates
the fix is mechanical and low-risk, not a judgment call requiring a new design decision. Reserve
accept-and-track (the O-P42-001/D-1099 pattern) for observations that are genuinely
PRE-EXISTING, OUT-OF-PERIMETER (predating the current feature's own history), and whose fix
would cost banked streak progress for no operative gain. This is a governance-DECISION lesson,
not a mechanical validator — the state-manager applies this disposition rule at each future LOW
observation, consulting the streak position and the item's provenance (fresh vs. inherited)
before defaulting to either accept or fix.
`[convergence-governance; fix-vs-accept; zero-streak-cost; fresh-vs-inherited; D-1101;
adr-046-gate; O-P44-001; O-P42-001-contrast]`

---

**[convergence-progress] Pass-45 re-applied every now-codified convergence-technique
discipline proactively — including the ninth (illustrative-quote verbatim-source-accuracy +
sibling-parity-check, D-1101) whose discovery caused the immediately preceding burst's
governance-fix disposition — against the pass-44-corrected set, and found zero findings, zero
observations, of any severity; streak ADVANCES 0/3 → 1/3, the cleanest pass this gate has
produced across all 45 passes.**

At D-1101 (pass-44), the ninth discipline was freshly codified in response to O-P44-001 — a
misattributed illustrative quote confined to BC-5.40.001's own disposition prose, fixed
same-burst. Pass-45 is the first fresh-context re-derivation to run against the exact
post-fix bytes, re-checking not only BC-5.40.001's newly-corrected CAP-031 quote but both
sibling BCs' equivalent CAP-031/CAP-032 quotes as well — all three confirmed verbatim-correct
against `capabilities.md`. Every other dimension this gate has ever codified (arm-parity
sweep, locus-class extension, ADR-anchor correctness, minimal-prose-plus-mechanical-backing,
grep-complete cluster-wide `inputs:` completeness, AC-owning-artifact cross-reference
discipline) was also re-verified clean, with the arm-parity and locus-class-extension pair now
at its FIFTH consecutive confirmation.

**Disposition:** Recorded as `[convergence-progress]`, the same tag used at D-1093/D-1095/D-1098
for prior single-pass confirmations following a fresh codification or a reset. This is the
first direct EVIDENCE that the ninth discipline, applied proactively from the start of a
fresh-context pass, closes the class it targets — evidence, not yet proof, since a single clean
pass is not itself a confirmation the way three consecutive passes would be. Per BC-5.39.001,
this is 1 of 3 required CONSECUTIVE clean passes counting from the pass-44 governance-fix — the
streak requires 2 further consecutive clean passes (46, 47), and any finding on either would
reset the streak to 0/3 again. The correct disposition for passes 46 and 47 is to CONTINUE
applying all nine disciplines proactively without relaxing rigor on the theory that "the
illustrative-quote dimension is now proven safe" — one clean pass covering a freshly-codified
discipline is progress, not closure.
`[convergence-progress; ninth-discipline-reconfirmed; fifth-clean-pass; cleanest-set-yet;
D-1102; adr-046-gate; pending-pass-46-47-confirmation; not-yet-converged]`

---

**[codified][process-gap] A sweep that judges a locus "clean" must RECORD WHY (arm-split-
correct / semantic-region-true-both-arms / needs-caveat), not just assert "checked" —
self-attested-but-unexplained checks can hide a locus still carrying stale framing**

At D-1097 (pass-40), the sibling-sweep-includes-VPs discipline swept every
`extract_frontmatter`/frontmatter-slice/byte-range locus in BC-4.17.001 and listed Invariant 5
as "confirmed correct, arm-split since v1.18, not re-broken." That verdict was WRONG: Invariant
5's headline ("body never read or parsed") was still carrying the pre-F-P39-001 un-caveated
byte-range framing, and its "Mirrors BC-4.13.001 Invariant 9" citation imported BC-4.13.001's
read-only `read_prefix` semantics without the reader-vs-writer caveat this hook's whole-file
`host::read_file`/`host::write_file` call site requires. The "checked" label recorded that the
locus was inspected, but not the REASONING that produced the "correct" verdict — so a
one-word self-attestation survived six further passes (41 through 45) without being
independently re-derived from first principles, because each subsequent pass trusted the
prior pass's "checked" label rather than re-deriving the arm-split from the locus's own text.
This is a distinct failure mode from O-P37-001's "self-attested read-in-full has no mechanical
backing" — that lesson concerns claiming a document was READ; this one concerns claiming a
VERDICT was correct without recording the reasoning that makes it checkable by a future pass.

**Disposition:** When any convergence-technique sweep (arm-parity, locus-class-extension,
byte-range/body-confinement, or any future class) judges a locus "clean" / "correct" /
"unaffected," the record MUST capture WHY — the specific classification the locus falls into
(e.g., "arm-split-correct: byte-range-confined on PC1, semantic-region-guarantee on PC2" or
"semantic-region-true-under-both-arms" or "unrelated to this claim class entirely") — not a
bare "checked" or "confirmed correct" verdict with no reasoning attached. This makes a future
fresh-context pass able to verify the PRIOR pass's reasoning against the locus's actual current
text, rather than trusting an opaque pass/fail label. Applied this burst: the mandatory
exhaustive byte-range/body-confinement locus audit (F-P46-001's remediation) records, for every
locus enumerated, which of the three classifications it falls into and why — the model for all
future locus-class audits under this discipline.
`[codified; process-gap; record-why-not-just-checked; byte-range-class-drained; D-1103;
adr-046-gate; F-P46-001; pass-40-sweep-gap]`

---

**[codified][process-gap] The grep-complete-all-cluster-artifacts audit discipline (D-1100,
eighth discipline) extends to AC-NNN cross-references and their attached verbatim quotes, not
only to `inputs:`-array completeness — a fix scoped to ONE cited AC-NNN locus does not by
itself confirm every other AC-NNN reference across the cluster is correctly scoped**

At D-1100 (pass-43), F-P43-002 fixed ADR-046's/BC-7.07.001's mis-scoped "AC-018" cross-
reference (implying it was BC-7.07.001's own acceptance criterion, when AC-018 is in fact
S-18.04a's story-level AC). That fix was scoped to the ONE locus the pass-43 finding named. It
did not trigger a grep-complete sweep of every OTHER `AC-[0-9]+` token across the frozen set —
so ADR-046's own separate "BC-5.40.001 Invariant 2/AC-007" cross-reference, carrying the
identical mis-scoping pattern (AC-007 is S-17.01's story-level AC, not BC-5.40.001's own),
survived three further passes (44, 45, unfound-at-43) unaudited, plus a fabricated "verbatim
quote" attached to it that was never checked against either candidate source text. The
underlying discipline (D-1100) was correctly grep-complete for `inputs:`-array
citations — it had not yet been extended to treat AC-NNN cross-references (and any illustrative
quote attached to them) as their own grep-complete class.

**Disposition:** Any burst that fixes a mis-scoped AC-NNN cross-reference MUST, in the same
pass, run a grep-complete sweep of every `AC-[0-9]+` token across the full frozen set's live
body (not just the one finding's own locus) and verify: (a) the citation names the actual
OWNING artifact (story vs. BC — extending D-1100's existing owning-artifact discipline), and
(b) any quote presented as verbatim is checked against the actual cited source text via a
repo-wide grep, not assumed accurate because it "sounds right." Applied this burst: F-P46-002's
mandatory exhaustive AC-reference audit swept every `AC-[0-9]+` token in ADR-046's body (found
AC-007 ×2, both now fixed; AC-018 ×2, both already correct from pass-43) — the model for all
future AC-reference-scoped fixes at this gate. This audit class is now confirmed DRAINED for
the two-instance cluster identified across passes 43 and 46 (AC-018, AC-007); a future AC-NNN
citation added to any of the four frozen-set artifacts must re-trigger this same grep-complete
sweep before the burst closes.
`[codified; process-gap; ac-reference-grep-complete; verbatim-quote-source-check; D-1103;
adr-046-gate; F-P46-002; D-1100-extension]`

---

**[codified][process-gap] Class-draining audits must sweep ALL cluster artifacts in the SAME
burst — unifying meta-lesson spanning passes 43, 46, and 47's AC-attribution stragglers**

The AC-attribution defect class (a live-body citation mis-scoping a story-level AC-NNN as if it
belonged to the BC it cites) recurred THREE times across three passes before it was genuinely
drained: pass-43 (F-P43-002) fixed BC-7.07.001's own "AC-018" mis-scoping, but the fix's audit
was scoped only to BC-7.07.001 — it did not sweep ADR-046 or the other companion BCs for the
same pattern. Pass-46 (F-P46-002) then found ADR-046's own separate "AC-007" mis-scoping — a
DIFFERENT artifact, the SAME pattern — and fixed it, but again the audit was scoped only to
ADR-046, not to the two companion BCs that also cite BC-5.40.001/AC-007. Pass-47 (F-P47-001)
then found BC-4.17.001's own Invariant 3 carrying the identical AC-007 mis-scoping — the THIRD
instance, the direct cluster-sibling of pass-46's fix, surviving because pass-46's audit never
swept BC-4.17.001 in the same burst. Each individual audit (pass-43, pass-46) was itself
correctly grep-complete WITHIN its own artifact's scope — the gap was never in the grep
mechanics, but in the SCOPE BOUNDARY: each audit treated "the artifact the finding named" as the
sweep's perimeter, when the actual perimeter should always have been "every artifact in the
frozen cluster" (ADR-046 + BC-4.17.001 + BC-5.40.001 + BC-7.07.001). Pass-47's own audit broke
this pattern deliberately — it swept all three companion BCs in one pass BEFORE any further
finding could surface the gap piecemeal — and confirmed the class is now genuinely DRAINED
cluster-wide: zero remaining live-body AC-NNN mis-anchors across any of the four frozen-set
artifacts.

This is not limited to the AC-attribution class. The SAME single-artifact-scoping failure mode
could recur for any other class-draining audit this gate runs: `inputs:`-completeness
(D-1090/D-1100), byte-range/body-confinement arm-scope (D-1096/D-1097), BC-to-BC/ADR
cross-anchor citations (D-1092/F-P35-001 class), or illustrative verbatim-quote accuracy
(D-1101). Each of those disciplines was ALREADY extended to be grep-complete WITHIN a single
artifact by its originating pass — but none of them had an explicit, standing requirement that
the sweep itself default to cluster-wide scope regardless of which one artifact triggered it.

**Disposition:** ANY class-draining grep audit run at this gate — whether triggered by an
`inputs:`-completeness finding, an AC-NNN cross-reference finding, a byte-range/arm-scope
finding, a BC↔BC/ADR cross-anchor finding, or a verbatim-quote-accuracy finding — MUST, as a
standing default, sweep ALL FOUR cluster artifacts (ADR-046, BC-4.17.001, BC-5.40.001,
BC-7.07.001) in the SAME burst that produces the fix, not just the one artifact where the
finding happened to surface. This eleventh discipline supersedes the "sweep the artifact this
finding names" default that pass-43 and pass-46 each implicitly applied. Applied this burst:
F-P47-001's own audit swept BC-4.17.001, BC-5.40.001, and BC-7.07.001 in a single pass —
confirming the AC-attribution class fully drained, with zero remaining live-body mis-anchors in
any of the three BCs and (from pass-46) ADR-046. Future class-draining audits at this gate
should apply this same cluster-wide-by-default scope from the outset, rather than waiting for a
cluster-sibling straggler to surface it pass-by-pass.
`[codified; process-gap; cluster-wide-audit-scope; ac-reference-grep-complete; D-1104;
adr-046-gate; F-P47-001; D-1100-extension; unifying-meta-lesson; pass-43-46-47-root-cause]`

---

**[codified][process-gap] VERBATIM-ABSENCE claims require a TRUE repository-wide grep, with
scope stated explicitly — twelfth distinct convergence-technique discipline**

At pass-46 (F-P46-002), the architect's fix corrected ADR-046's fabricated "verbatim quote"
claim by asserting the quoted phrase ("MUST NOT be overridden via environment or arguments") was
"not present anywhere in this repository," backed by a `grep -rn` sweep. That sweep was, in
fact, rooted at `.factory/` — it never searched `plugins/` or `crates/`. At pass-48 (F-P48-001),
a TRUE repository-wide grep (executed this pass, spanning the whole repository, not
`.factory/`-scoped) found the phrase verbatim-present in
`plugins/vsdd-factory/bin/factory-lock-write.sh`'s `TTL_SECONDS` header comment — an
`inputs:`-listed file this ADR already cites for the same TTL constant. The phrase was INHERITED
from that file, not fabricated; the pass-46 "fabricated" claim was itself false, caused by an
under-scoped grep presented with unqualified "anywhere in this repository" language.

**Disposition:** ANY assertion in disposition/changelog prose that a phrase, quote, or pattern is
"fabricated," "not present anywhere," or "verbatim-absent" MUST be backed by a grep that
genuinely covers the full repository — every `inputs:`-listed file class, including `plugins/`
and `crates/`, not just `.factory/` — and the disposition prose recording the claim MUST STATE
the scope of the grep performed (e.g., "repo-wide, not `.factory/`-scoped") so a later pass can
verify the claim's provenance without re-deriving it from scratch. An absence claim that does not
state its own scope is not verifiable and should be treated as suspect. Applied this burst:
F-P48-001's own fix explicitly states "a TRUE repo-wide `grep -rn` for the phrase, across the
whole repository, not `.factory/`-scoped" as the basis for the corrected claim.
`[codified; process-gap; verbatim-absence-claim; repo-wide-grep-scope; D-1105; adr-046-gate;
F-P48-001; twelfth-discipline]`

---

**[content-defect-discipline] SUMMARY-ENUMERATION claims must match the normative body exactly —
thirteenth distinct convergence-technique discipline**

At pass-48 (O-P48-001), BC-7.07.001's Description used "only" to enumerate the hook's exit-0
conditions as success/no-op/STATE.md-unreadable — omitting two normatively-documented exit-0
paths (Precondition 4's worktree-discovery-failure/split-tree-mismatch, and Postcondition 9's
hook-crash-under-`on_error=continue`). The mandatory within-artifact sweep this finding triggered
(grepping the BC's Description plus all other summary/overview prose for further "only"/
"exclusively"/"exhaustively" claims, cross-checked against the normative Preconditions/
Postconditions/Invariants) found a SIBLING straggler of the identical defect class inside the
NORMATIVE body itself: Postcondition 8's own closing sentence restated the same under-inclusive
exit-0 list ("The ONLY exit codes this hook ever produces are 0 ... and 2 ...") — a more serious
instance than the Description summary, since it purports to be part of the authoritative
enumeration rather than an informal overview.

**Disposition:** any Description/overview sentence using "only," "exclusively," "exhaustively,"
or equivalent totalizing language to enumerate a set of outcomes, conditions, or exit codes MUST
be cross-checked against the FULL set of Preconditions/Postconditions/Invariants that can produce
that outcome — not just the ones the summary sentence's author had in mind at drafting time — and
any such claim found in one location REQUIRES a within-artifact sweep for sibling instances of the
same pattern before the burst closes, since (as this pass demonstrated) an under-inclusive claim
in an informal summary section often has a normative-section sibling carrying the identical
defect. Applied this burst: the Description fix and the Postcondition 8 sibling fix were both
applied same-burst, from the single triggering finding's mandated sweep.
`[content-defect-discipline; summary-enumeration-accuracy; sibling-sweep; D-1105; adr-046-gate;
O-P48-001; thirteenth-discipline]`

---

**[process-observation][convergence-observation] META: a remediation's OWN disposition/changelog
prose is itself attack surface — now confirmed across FOUR instances (pass-37, pass-44, and TWO
at pass-48)**

Four separate findings at this gate now share the same recurring root pattern: the prose a fix
writes to DESCRIBE its own correction contains a factual error independent of the substantive
fix itself. Pass-37 (F-P37-001): ADR-046's own decision-count mis-statement in disposition prose.
Pass-44 (O-P44-001): BC-5.40.001's illustrative "verbatim quote" misattributed to the wrong
source. Pass-48 (F-P48-001): ADR-046's own "fabricated" provenance claim, itself false, caused by
an under-scoped grep. Pass-48 (O-P48-001): BC-7.07.001's own Description/Postcondition-8
under-inclusive exhaustive-enumeration claim. In every instance, the LIVE-BODY substantive
correction being described was itself accurate — only the NARRATIVE describing/justifying it
carried the defect. This confirms disposition/changelog/`last_amended` prose is not inert
documentation of a fix; it is itself a surface an adversarial pass will find defects in, at a
rate comparable to the live body.

**Disposition (reaffirmed, no change from pass-44's original framing):** keep disposition prose
MINIMAL, and verify every factual claim it makes — every quote, every count, every "fabricated"/
"absent"/"only" assertion — via direct evidence (grep, count, cross-reference) BEFORE writing it,
rather than composing persuasive narrative first and treating its claims as self-evidently true
because they read plausibly. Applied this burst: F-P48-001's own fix narrative states the exact
grep command scope used to verify the corrected claim; O-P48-001's own fix narrative states the
exact sweep performed and its one additional finding (Postcondition 8) — both narratives are
falsifiable, not merely assertive.
`[process-observation; convergence-observation; meta-lesson; disposition-prose-attack-surface;
D-1105; adr-046-gate; F-P37-001; O-P44-001; F-P48-001; O-P48-001; four-instance-pattern]`

---

**[codified][process-gap] CITATION→INPUT PARITY — a body edit that adds a verbatim citation must
add its source to `inputs:` in the SAME burst — fourteenth distinct convergence-technique
discipline**

At pass-49 (F-P49-001), ADR-046's own v1.19 disposition prose — itself the pass-48 fix that
re-attributed AC-007 to S-17.01 and quoted BC-5.40.001 Invariant 2 verbatim — quoted S-17.01's
AC-007 verbatim without adding S-17.01 to ADR-046's own `inputs:` array. This is distinct from
every prior `inputs:`-completeness finding at this gate (F-P28-002, F-P29-002, F-P30-002,
F-P31-001, F-P33-001, F-P35-002, F-P43-001): those were all PRE-EXISTING omissions a
not-yet-performed audit would have caught. F-P49-001 is the FIRST instance where a PREVIOUSLY-RUN
grep-complete audit (pass-43's, extended cluster-wide at pass-44) was subsequently invalidated by
a LATER body edit (pass-46/48's AC-007 re-attribution) that added a brand-new citation without a
same-burst `inputs:` update — the audit was correct when it ran; the ground shifted under it
afterward. The mandatory re-audit this finding triggered found 6 more instances of the identical
mechanism, cluster-wide: ADR-046's own §Companion Amendment 3 (S-18.04a, cited since pass-43);
BC-4.17.001's Invariant 3 (S-17.01, cited since pass-47) and Invariant 5 (BC-1.17.001); BC-5.40.001's
§VP Anchors (S-19.08); BC-7.07.001's PC4/Architecture Anchors (`factory-lock-write.sh`) and
Related BCs (BC-7.07.002).

**Disposition:** (i) any body edit that ADDS a verbatim citation or quote of a source file or
story — not merely a passing mention, but content the artifact's own claims depend on — MUST add
that source to `inputs:` in the SAME burst as the edit that introduces the citation; this is a
point-of-edit obligation, not deferred to the next scheduled audit. (ii) Because the
grep-complete inputs audit (D-1090/D-1100, eighth discipline) is inherently POINT-IN-TIME, it
does not stay valid across subsequent body-evolving edits — after any RUN of body-evolving
bursts (such as this gate's own passes 43 through 48, each adding or correcting a live citation),
a fresh CLUSTER-WIDE grep-complete inputs re-audit must be performed, not merely trusted because
an earlier pass's audit reported clean. Applied this burst: state-manager ran the compute-input-
hash `--check`/`--update` recompute across all four cluster artifacts, and product-owner/architect
each ran the triggered re-audit as a cluster-wide sweep rather than scoping to the single artifact
the initiating finding named — directly applying the D-1104 eleventh-discipline standing default
to this fourteenth discipline's own re-audit.
`[codified; process-gap; citation-input-parity; cluster-wide-reaudit; D-1106; adr-046-gate;
F-P49-001; fourteenth-discipline]`

---

**[codified][process-gap] "Verified present in STORY-INDEX" claims require a mechanical
membership check, not a BC-prose-only Traceability fix**

At pass-50 (F-P50-001), BC-4.17.001's own §Story Anchor and ADR-046's own §File-Change Plan both
asserted that S-17.05 (the ADR-046 implementing story) was "verified present in STORY-INDEX.md" /
"is its catalog entry." This claim was FALSE: STORY-INDEX's E-17 roster ended at S-17.04, marked
"COMPLETE"; S-17.05 existed as a drafted story FILE (authored by story-writer earlier this
session) but was never REGISTERED as a STORY-INDEX catalog row. The false "verified present"
language traces to the pass-25 F-P25-002 remediation (2026-08-26), which resolved the
Traceability `[pending]`→S-17.05 anchor in the BC's own prose but never performed — nor claimed to
perform — the actual STORY-INDEX membership check the "verified present" wording asserts. The gap
survived ~24 further passes (pass-26 through pass-49) because every intervening audit type
(grep-complete `inputs:`-completeness, AC-attribution cross-reference) checks CITATION
correctness, not INDEX-ROW EXISTENCE — a claim can cite a story by exact path, quote its content
correctly, and still be lying about whether that story is catalogued anywhere.

**Disposition:** any spec-prose assertion of the form "X is verified present in <INDEX>" or "X is
<INDEX>'s catalog entry" is NOT self-certifying — it MUST be backed by an actual mechanical
index-membership check (e.g. `grep -c "^| X |" <INDEX>.md`) performed at or before the time the
assertion is written, not inferred from a sibling Traceability-row edit in a DIFFERENT artifact.
Standing mandate: (i) any newly-drafted implementing story MUST be registered in STORY-INDEX at
draft time, not deferred until its own PR; (ii) any BC/ADR/spec §Story Anchor or similar
"present-in-index" assertion about a story, BC, VP, or ADR requires the citing agent (or the
state-manager burst that reconciles it) to run the membership check and record that it was run,
not merely assert the conclusion. This is distinct from CITATION→INPUT PARITY (fourteenth
discipline, D-1106): that discipline governs whether a citation's SOURCE is listed in `inputs:`;
this discipline governs whether an "present-in-<INDEX>" CLAIM about that source is actually true.
`[codified][process-gap][catalog-membership-verification; D-1107; adr-046-gate; F-P50-001;
fifteenth-discipline]`

---

**[codified][process-gap] CITATION→INPUT PARITY (fourteenth discipline) extends to exact-path
STORY citations, not merely file/BC/ADR citations**

At pass-50 (F-P50-002), all three ADR-046 companion BCs (BC-4.17.001, BC-5.40.001, BC-7.07.001)
cited S-17.05 by exact file path and content claims (§Story Anchor: E-17 Wave 5, 8pts, `tdd_mode:
strict`) in their live bodies, but S-17.05 was absent from all three BCs' `inputs:` arrays. This is
a direct instance of the CITATION→INPUT PARITY discipline (fourteenth, D-1106) — a body edit that
adds a verbatim or exact-path citation of a source MUST add that source to `inputs:` in the SAME
burst — applied to a citation TYPE the discipline's initial codification did not explicitly
enumerate: exact-path STORY citations, as distinct from the file/BC/ADR citations D-1106's own
finding set covered. BC-5.40.001's own cross-check (product-owner, in-scope this pass) additionally
found and fixed a sibling gap of the identical class: S-17.01, cited with content claims since
PR #181/D-544 (2026-06-11), was ALSO missing from BC-5.40.001's `inputs:`.

**Disposition:** the fourteenth discipline's perimeter is CONFIRMED to cover every citation TYPE
that makes a load-bearing claim about a source's content — file paths, BC/ADR references, AND
story IDs cited by exact path — not merely the file/BC/ADR subset D-1106's own finding happened to
instantiate. Future grep-complete `inputs:` audits at this gate must explicitly include a
story-ID-shaped token sweep (`S-[0-9]+\.[0-9]+`) alongside the existing file-path and BC/ADR-ID
sweeps, not treat story citations as a separate, lower-priority class.
`[codified][process-gap][citation-input-parity-extension; D-1107; adr-046-gate; F-P50-002;
fourteenth-discipline-extension]`

---

**[content-defect-discipline] Illustrative "analogous to T-NNN" enumerations must match the
authoritative implementing-BC's own basis — an extension of the ninth discipline (D-1101) from
verbatim quotes to illustrative example-lists**

At pass-51 (O-P51-001), ADR-046's own §Decision 5 per-element reconciliation table illustratively
enumerated BC-4.17.001's migrated VP-row analogs as "analogous to T-001/T-002/T-003/T-004/T-007."
This was IMPRECISE: T-002/T-003 are BC-5.40.001's staleness-BLOCK tests with no stamper analog
(the always-allow-and-correct stamper never blocks, so they were never migrated), and T-005
(`extract_frontmatter`/no-delimiter fail-open) was omitted despite being migrated. BC-4.17.001's
own §Verification Properties note cites the exact set "T-001/T-004/T-005/T-007" as the
authoritative migrated-analog basis (VP-TBD-7/8/9) — the sibling BC had it right; only ADR-046's
own illustrative parenthetical carried the imprecise enumeration.

**Disposition:** this is NOT a new standalone discipline — it is a content-defect instance of the
existing NINTH discipline (D-1101), which required illustrative "verbatim quote" content attached
to a fix's own disposition prose to cite the ACTUAL text of the cited source, cross-checked via
sibling-parity. O-P51-001 demonstrates the same underlying obligation applies to a DIFFERENT
illustrative-content shape: an "analogous to <ID-list>" example enumeration, not only a quoted
string. Standing mandate (extended): any illustrative content in an ADR/BC that references another
artifact's own basis for a claim — whether a verbatim quote OR an enumerated ID-list — MUST be
cross-checked against that other artifact's own authoritative statement of the same claim before
being asserted independently, and any within-artifact sibling instances of the same illustrative
reference MUST be swept for the identical defect (this pass's T-NNN sweep found none, confirming
the defect was confined to this single locus).
`[codified][process-gap][content-defect-discipline][ninth-discipline-extension; D-1108;
adr-046-gate; O-P51-001]`

---

**[process-observation][convergence-observation][meta] Pass-51 zero-BLOCKER/HIGH/MED result
confirms the substance and all fourteen prior metadata-layer disciplines continue holding**

Pass-51 is the cleanest pass this gate has produced since the last clean streak (pass-45): zero
BLOCKER/HIGH/MED findings, only a single LOW illustrative-enumeration observation (O-P51-001),
itself an instance of an already-codified discipline (the ninth, D-1101) rather than a new failure
mode. The behavioral core (write-composition table, five-outcome table, identity-gating logic,
event-sourcing struct-variant text) remains independently re-verified CLEAN for the 25th
consecutive pass (since pass-27). None of the fourteen previously-codified convergence-technique
disciplines (comprehensive-sibling-sweep, 4-leg parity, grep-complete inputs, ADR anchor audit,
minimal-prose discipline, arm-parity sweep, sibling-sweep-includes-VPs, AC-attribution
cross-reference, illustrative-quote accuracy, record-why-not-just-checked, cluster-wide-audit
scope, verbatim-absence grep, summary-enumeration accuracy, CITATION→INPUT PARITY) regressed this
pass. **Disposition:** because ADR-046 itself was edited this burst (O-P51-001's fix), the
BC-5.39.001 3-CLEAN streak does NOT advance on the strength of this clean-of-blockers result — the
literal-3-CLEAN count restarts fresh at pass-52 against the newly-edited set, per BC-5.39.001's
own discipline (any spec edit, even a fix, requires re-review of the edited artifact before it can
count toward a clean streak).
`[process-observation][convergence-observation][meta; D-1108; adr-046-gate; pass-51]`

---

**[convergence-progress] Pass-52 re-applied all fifteen now-codified convergence-technique
disciplines proactively — including the ninth discipline's D-1108 extension to illustrative
example-enumerations, the exact dimension pass-51's own finding (O-P51-001) targeted — and found
the entire O-P51-001-corrected frozen set clean on every dimension; streak ADVANCES 0/3 → 1/3, the
first clean pass against the corrected set.**

Pass-52 dispatched fresh-context, information-asymmetric per the Iron Law, against the frozen set
ADR-046 v1.21 / BC-4.17.001 v1.24 / BC-5.40.001 v1.20 / BC-7.07.001 v1.37 — the set produced by the
pass-51 fix burst. Zero findings at any severity. Most directly, ADR-046 §Decision 5's illustrative
"analogous to T-NNN" enumeration (fixed by architect at D-1108 to read "T-001/T-004/T-005/T-007")
was independently re-derived and confirmed matching BC-4.17.001's own authoritative basis, with a
fresh within-artifact sweep finding no sibling recurrence anywhere else. All fourteen prior
disciplines (comprehensive-sibling-sweep, 4-leg parity, grep-complete inputs, ADR anchor audit,
minimal-prose discipline, arm-parity sweep, sibling-sweep-includes-VPs, AC-attribution
cross-reference, illustrative-quote verbatim accuracy, record-why-not-just-checked,
cluster-wide-audit scope, verbatim-absence grep, summary-enumeration accuracy, CITATION→INPUT
PARITY, catalog-membership-verification) also re-verified holding with zero regression. The
behavioral core (write-composition, five-outcome table, identity-gating, event-sourcing) remains
independently re-verified CLEAN for the 26th consecutive pass (since pass-27).

**Disposition:** Recorded as `[convergence-progress]`, the same tag used for D-1093's pass-36 and
D-1095's pass-38 confirmations, distinguished by scope: this entry is the first direct EVIDENCE
that the ninth discipline's D-1108 extension (illustrative example-enumerations, not only verbatim
quotes) holds under a fresh adversary's independent re-derivation — evidence, not yet proof, since
a single clean pass is not itself a confirmation the way three consecutive passes would be. Per
BC-5.39.001, this is 1 of 3 required CONSECUTIVE clean passes — the streak requires 2 further
consecutive clean passes (53, 54), and any finding OR spec edit on either would reset the streak to
0/3 again. The correct disposition for passes 53 and 54 is to CONTINUE applying all fifteen
disciplines proactively without relaxing rigor on the theory that "the illustrative-enumeration
extension is now proven safe" — one clean pass on a freshly-extended discipline is progress, not
closure. `[convergence-progress; fifteen-disciplines; ninth-discipline-extension-reconfirmed;
first-clean-pass-post-O-P51-001; D-1109; adr-046-gate; pending-pass-53-54-confirmation;
not-yet-converged]`

---

**[convergence-progress] Pass-53 is the second CONSECUTIVE clean pass against the unchanged
O-P51-001-corrected set — streak ADVANCES 1/3 → 2/3, one clean pass short of literal 3-CLEAN.**

Pass-53 dispatched fresh-context, information-asymmetric per the Iron Law, against the SAME frozen
set pass-52 reviewed (ADR-046 v1.21 / BC-4.17.001 v1.24 / BC-5.40.001 v1.20 / BC-7.07.001 v1.37).
Zero findings at any severity. All fifteen previously-codified convergence-technique disciplines
re-verified holding with zero regression, including a THIRD consecutive independent re-derivation
of the ninth discipline's D-1108 illustrative-enumeration extension (pass-51 fix, pass-52
re-derivation, pass-53 re-derivation). The behavioral core remains independently re-verified CLEAN
for the 27th consecutive pass (since pass-27).

**Disposition:** Recorded as `[convergence-progress]`, distinguished from D-1109's entry by being
the SECOND consecutive confirmation rather than the first — this is materially stronger evidence
that the ninth-discipline extension and all fourteen other disciplines are stable under repeated
independent scrutiny, not a one-off. Per BC-5.39.001, this is 2 of 3 required CONSECUTIVE clean
passes — exactly 1 further consecutive clean pass (54) is required for literal 3-CLEAN convergence.
Any finding OR spec edit on pass 54 resets the streak to 0/3 again. The correct disposition for
pass 54 remains to CONTINUE applying all fifteen disciplines proactively without relaxing rigor —
being one pass from convergence is not itself a reason to review less carefully.
`[convergence-progress; fifteen-disciplines; ninth-discipline-extension-triple-confirmed;
second-consecutive-clean-pass; D-1110; adr-046-gate; pending-pass-54-confirmation;
not-yet-converged]`

---

**[convergence-governance] At streak 2/3, an adversary-adjudicated-defensible LOW descriptive item
is accepted-and-tracked rather than fixed — fixing a defensible non-defect would reset a live
convergence streak for no substantive gain.**

Pass-53's adversary considered BC-7.07.001 §Description's "Renewal is a no-op when: … or
`expires_at` is malformed (never repaired)" phrasing against the normative body's actual
`Err(LockError::Malformed(msg))` return (Postcondition 3 case 1 / Invariant 3b table row 1 /
EC-004), and correctly distinguished a plain-English observable-effect summary ("no-op" describing
the fact that nothing gets written) from an assertion of the `RenewOutcome::NoOp` Rust enum variant
(which this case explicitly is NOT). The Description's substantive claim is true; only its
colloquial label differs from the normative body's precise type vocabulary. **Disposition:**
tracked as O-P53-DESC-NOOP, ACCEPTED non-blocking, NOT fixed this pass — applying the same
`[convergence-governance]` fix-vs-accept discipline the O-P42-001 disposition (D-1099) established:
a defensible non-defect found while the streak is live (2/3, one pass from convergence) is accepted
and tracked rather than fixed in-place, because the fix costs a live streak for a purely
editorial/precision gain, not a correctness gain. This differs from the D-1101 fix-vs-accept
precedent (which fixed O-P44-001/O-P48-001/O-P51-001 at streak-floor 0/3, where a fix costs
nothing) — the disposition RULE is the same (weigh fix-cost against streak-state), but the
STREAK-STATE input differs, correctly producing the opposite action (accept, not fix) at non-floor
streak. Anchor: optional Description-precision tightening at a future non-gating touch.
`[convergence-governance][accept-vs-fix][streak-state-dependent; O-P53-DESC-NOOP; D-1110;
adr-046-gate; O-P42-001-precedent-consistent; O-P51-001-precedent-distinguished]`

---

**[codified][process-gap] STEP-NUMBER CITATION discipline — any "Step N"/"Steps N-M" citation of a
module's own internal enumeration MUST be cross-checked against that module's actual
`//!`/doc-comment step numbering, not merely checked for functional/arm correctness.**

Pass-54's adversary found F-P54-001: ADR-046 mis-cited `verify-state-timestamp-refresh`'s own
module-doc step numbers at four loci — the lock-expiry (`factory_lock.expires_at`) staleness arm
was labeled "Step 7" (correct: "Step 8"), and the timestamp staleness arm was labeled "Steps 4-6"
(correct: "Steps 4-7"), confirmed against the module's own doc-comment enumeration ("skip Steps
4-7; proceed to Step 8"). This defect survived 53 prior passes because every prior pass verified
the FUNCTIONAL correctness of the two-arms description (which was always accurate — both arms
correctly attributed to the right behavior) without independently re-deriving the LITERAL step
numbers from the cited module's own source. Functional correctness and numeric-label correctness
are separable failure modes: a citation can correctly describe WHAT a step does while citing the
WRONG NUMBER for it, and no prior discipline's grep/audit pattern (inputs-completeness,
AC-attribution, illustrative-content-accuracy, etc.) was scoped to catch numeric step-citation
drift specifically. **Disposition:** CODIFIED as the SIXTEENTH convergence-technique discipline —
any "Step N"/"Steps N-M" citation of a module's own enumeration must be cross-checked, via direct
inspection of that module's actual `//!`/doc-comment numbering, before being asserted in a spec
document; a within-artifact grep-complete sweep of every `Step[s] [0-9]` token plus a cross-BC check
for the same class must accompany any such fix, mirroring the now-established pattern (fourteenth,
fifteenth disciplines) of pairing a point fix with a mechanical corpus-wide re-audit.
`[codified][process-gap][step-number-citation; sixteenth-discipline; F-P54-001; D-1111;
adr-046-gate; functional-vs-numeric-correctness-distinct]`

---

**[META] Sixth streak reset this session, SECOND at the convergence pass itself (2/3→0/3 at both
pass-43 and pass-54) — both convergence-pass resets came from a fresh-context adversary finding
ONE genuine-but-narrow defect via a lens no prior pass had used; empirical asymptotic-floor
confirmation.**

The BC-5.39.001 3-CLEAN streak has now reset six times this session (passes 35, 37, 39, 43, 46,
54), and twice specifically AT the convergence pass itself — i.e., with the streak already at 2/3
and only one more consecutive clean pass required for literal 3-CLEAN. Pass-43 (D-1101 range)
reset 2/3→0/3 via a `capabilities.md` inputs-completeness lens no prior pass had applied at that
depth; pass-54 (D-1111) reset 2/3→0/3 via an exact module-doc step-number citation lens no prior
pass had applied. In both cases the finding was a real, narrow, single-locus-class defect (not a
regression of a previously-fixed item, not a false positive), and in both cases the behavioral core
itself remained untouched and independently re-verified CLEAN. This is a second empirical data
point supporting the same conclusion the pass-43 reset first suggested: a 50+-pass-deep gate can
still be genuinely one fresh cross-check dimension away from a real (if narrow) defect, which is
the strongest argument yet for the D-386 Option C asymptotic-acceptance alternative to chasing
literal 3-CLEAN indefinitely — the marginal defects being found are increasingly confined to the
citation/provenance/numbering perimeter, never the behavioral core (CLEAN for 28 consecutive
passes). **Disposition:** recorded as a META observation for the human's ongoing convergence-
strategy decision (STATE.md §5 Pending Human Decision). The human was RE-OFFERED accept-provisional
under D-386 Option C this burst and has again DECLINED, choosing to CONTINUE looping toward literal
3-CLEAN. `[META][convergence-strategy][sixth-reset; second-convergence-pass-reset; pass-43-parallel;
D-1111; adr-046-gate; asymptotic-floor-empirical-confirmation; human-reaffirmed-continue]`

---

**[convergence-progress] Pass-55 is the first clean pass against the pass-54-corrected set, and the
FIRST independent re-derivation of the newly-codified sixteenth discipline (STEP-NUMBER CITATION)
since its D-1111 codification — streak ADVANCES 0/3 → 1/3.**

Pass-55 dispatched fresh-context, information-asymmetric per the Iron Law, against the frozen set
produced by the pass-54 fix burst (ADR-046 v1.22 / BC-4.17.001 v1.24 / BC-5.40.001 v1.20 /
BC-7.07.001 v1.37). Zero findings at any severity. Most directly, the adversary independently
re-derived every "Step N"/"Steps N-M" citation of `verify-state-timestamp-refresh`'s own module-doc
enumeration across ADR-046's body (§Context item 2, §Rationale, §Decision 3, §Decision 5) and
confirmed all four F-P54-001 loci now correctly read "Step 8" (lock-expiry arm) and "Steps 4–7"
(timestamp arm), cross-checked directly against the module's own source doc-comment — the fix landed
completely and correctly, with no partial fix and no sibling recurrence found anywhere in the frozen
set (within-artifact sweep plus cross-BC check, both clean). All fifteen other previously-codified
disciplines also re-verified holding with zero regression, including a fourth consecutive
independent re-derivation of the ninth discipline's D-1108 illustrative-enumeration extension. The
behavioral core remains independently re-verified CLEAN for the 29th consecutive pass (since
pass-27).

**Disposition:** Recorded as `[convergence-progress]`, the same tag used for D-1093/D-1095/D-1098/
D-1109/D-1110's confirmations, distinguished by being the first evidence that a freshly-codified
discipline (the sixteenth, STEP-NUMBER CITATION) holds under independent fresh-context
re-derivation immediately following its own codifying fix — evidence, not yet proof, since a single
clean pass is not itself a confirmation the way three consecutive passes would be. Per BC-5.39.001,
this is 1 of 3 required CONSECUTIVE clean passes — the streak requires 2 further consecutive clean
passes (56, 57), and any finding OR spec edit on either would reset the streak to 0/3 again (already
at floor). The correct disposition for passes 56 and 57 is to CONTINUE applying all sixteen
disciplines proactively without relaxing rigor on the theory that "the step-number dimension is now
proven safe" — one clean pass on a freshly-codified discipline is progress, not closure.
`[convergence-progress; sixteen-disciplines; sixteenth-discipline-first-confirmation;
first-clean-pass-post-F-P54-001; D-1112; adr-046-gate; pending-pass-56-57-confirmation;
not-yet-converged]`
