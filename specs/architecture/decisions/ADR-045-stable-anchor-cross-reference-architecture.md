---
document_type: adr
adr_id: ADR-045
version: "1.3"
title: "ADR-045: Frozen-provenance cross-reference architecture — retain load-bearing version pins as point-in-time provenance, elevate the content-based fingerprint/suspect-link model to the primary churn-solution, and reconcile POLICY 19 with permitted provenance pins"
status: accepted
date: 2026-08-24
producer: architect
deciders:
  - architect
  - human (ratified frozen-provenance + suspect-link pivot, 2026-08-25 — see Status)
subsystems_affected: [SS-01, SS-04, SS-05, SS-07]
supersedes: null
superseded_by: null
traces_to: .factory/specs/architecture/ARCH-INDEX.md
extends: null
phase: brownfield-backfill
cycle: v1.0-brownfield-backfill
inputs:
  - .factory/research/wave7-xref-consistency-research.md
  - .factory/research/adr-045-independent-validation.md
  - .factory/policies.yaml
  - .factory/specs/architecture/ARCH-INDEX.md
modified:
  - "2026-08-25 (v1.3) — MODEL PIVOT recorded by architect per direct human instruction (the human, acting as senior architect, overrode the v1.2 ratified direction the same day it was recorded — before any state-manager recording burst had applied it: policies.yaml, ARCH-INDEX, and STATE.md remained untouched by v1.2, so this revision replaces v1.2 in place; no supersession ceremony is needed because nothing was ever durably applied). Human requirement, verbatim: 'I don't want to change the version numbers pinned inside the ACs. I want ULTIMATE TRACEABILITY.' This inverts the core model. **Tier 1 REDEFINED** — from v1.2's 'strip every load-bearing inline version pin' to 'RETAIN existing pins, reinterpreted as FROZEN POINT-IN-TIME PROVENANCE': `BC-1.03.017 v1.27` now means 'this AC was authored/reviewed against v1.27,' is explicitly NOT required to equal the target's current version, and a target version bump forces ZERO downstream edits because the pin is provenance, not a liveness assertion (the INDEX registries still resolve the current version mechanically on demand, unchanged from v1.1/v1.2). **Tier 2 ELEVATED to PRIMARY, IN PHASE A** — no longer deferred/gated behind a Phase B leakage measurement: the content-based fingerprint/suspect-link mechanism (content-hash, never a version-field proxy, per the v1.1/R1 correction, which is retained unchanged) becomes the churn-solution — every load-bearing pin is, by construction, a fingerprint-tracked edge (the pin's presence IS the content-dependency declaration; no separate opt-in markup is introduced); when a target's CONTENT changes, every citing edge with a baseline is auto-flagged SUSPECT until a reviewer re-reviews and clears it. Baseline remains edge-local; `review`/`clear` remain distinct transitions; validation remains read-only (never auto-clears) — all four v1.1 corrections to the Tier-2 design are retained unchanged by this pivot. **Tier 3 REPURPOSED, not deferred** — the comrak-based AST section-scoping/soft-break-normalization/anchor-tokenization substrate (originally built to detect and block stale or newly-introduced pins) is now needed, in Phase A, as the canonical-content extractor that computes what Tier 2 fingerprints — the same four failure-mode fixes (line-wrap, anchor-interposition, frontmatter/changelog exclusion, live-section scoping) apply, but in service of correct fingerprinting rather than pin-detection; its 'block on any load-bearing pin' enforcement mode is retired because pins are no longer prohibited. **Gate semantics inverted**: POLICY 7/8/14/17's version-leg no longer checks version-string equality against BC-INDEX/ARCH-INDEX/STORY-INDEX/VP-INDEX; it checks fingerprint-suspect status. A frozen/historical pin in a load-bearing position is explicitly NOT a finding — this is what the Wave-7 stragglers (S-21.19/20/21/23, 'AC cites vN but current is vN+1') resolve to: legitimate provenance, reclassified, not edited. **POLICY 19 / TD-VSDD-091 INVERTED** (see Policy Amendments table) — the prohibition on load-bearing ADR version pins is retired and replaced with a positive frozen-provenance-and-fingerprint-tracking mandate; the churn-prevention role POLICY 19 used to serve is now served by the suspect-link, not by forbidding the pin. **The v1.2 corpus-strip migration (~3,074 pins across 156 files) is REMOVED in its entirety — no strip is needed; the pins stay exactly as authored.** Phase A work is now mechanism-build, not corpus-strip: (a) build the content-fingerprint/suspect-link WASM hook (comrak-based section extractor + SHA-256 content hash + edge-local sidecar store); (b) establish baseline fingerprints for the existing ~3,074 live/load-bearing citations across 156 files (the same corpus scan from v1.1/v1.2, now read as SCALE context for baselining rather than as strip scope — 51% concentrated in the 4 INDEX files, ~30–40 anchor-interposed/line-wrapped ADR pins requiring the newline-normalized extraction pass); (c) change the POLICY 7/8/14/17 parity-gate semantics so frozen pins are not findings; (d) the Wave-7 stragglers resolve by reclassification, not editing. **Honesty note on the OpenFastTrace alternative**: v1.0–v1.2 evaluated and explicitly REJECTED 'accept the OpenFastTrace embed-revision-and-void-coverage-on-change model' as certification-grade overkill this project does not need. The human's explicit 'ultimate traceability' requirement reverses that judgment: v1.3 adopts a controlled version of exactly that model, narrower than OFT's own design (OFT voids ALL coverage links project-wide on any semantic revision bump to the covered item; this ADR flags only the specific citing edges that hold a baseline against the changed target, and only on CONTENT change, never on a bare version-number bump) — see Rationale and Alternatives Considered, where that alternative is inverted from rejected to chosen and the prior rejection is preserved as historical record, not deleted. The re-review signal this pivot accepts is framed as a FEATURE (meaningful, content-triggered traceability, not mechanical version-string chasing) for two independent reasons: (1) it fires only when cited CONTENT actually changes, never on a version-number-only bump with no substantive change underneath it — this is the exact distinction Wave-7's churn lacked; (2) because a frozen pin is no longer a finding, the 3-CLEAN convergence loop is no longer perturbed by target version bumps at all — it is perturbed only by CONTENT changes to targets a citing edge has a recorded dependency on, which is precisely the set of events that should force re-review, and clearing a suspect is a bounded, edge-local review action, never a corpus-wide re-anchoring burst. **E-23** (`.factory/stories/epics/E-23-adr045-phase-a-stable-anchor-migration.md` and its child stories S-23.01–S-23.14) was scoped entirely for the v1.2 strip model (anchor-pin-detection-classifier, precommit-normalization-codemod, thin-new-pin-guard-hook, per-document-class migration bursts, Phase-B residual-leakage measurement) and **requires re-scoping** to the frozen-provenance + suspect-link mechanism-build described here; this ADR revision does not itself re-scope E-23 (story content is story-writer/orchestrator-owned per the Agent Routing Table) — it is flagged here for that follow-up. ADR-045 v1.2→v1.3. [Prior: 2026-08-25 (v1.2) — RATIFICATION recorded by architect. Phase A (Tier 1: stable anchors + INDEX-resolution + thin new-pin guard + pre-commit normalization codemod) RATIFIED by human at the standard human-approval gate for governance-policy/spec amendment (orchestrator-enforced, per CLAUDE.md Pipeline Authority). Tiers 2 and 3 ratified as design-direction only, GATED on Phase-B measured residual leakage. Policy amendments (POLICY 7/8/14/17/19) authorized for application by state-manager per the Policy Amendments Required table. Status proposed→accepted. ADR-045 v1.1→v1.2. [Prior: 2026-08-25 (v1.1) — revised by architect in response to `.factory/research/adr-045-independent-validation.md` (independent adversarial validation, research-agent, VERDICT: PARTIALLY-SUPPORTS ratification). Seven corrections applied: (1) Tier 2 fingerprint corrected from `version:`-field proxy to CONTENT-BASED hash (R1 HIGH/BLOCKER) — the version-proxy shortcut re-introduced the exact version-coupling Tier 1 removes; also incorporates baseline-belongs-to-the-citing-edge (not the target), `review` vs `clear` as two distinct Doorstop state transitions, and read-only validation (never auto-clear on mismatch). (2) Restructured to PHASED RATIFICATION — Tier 1 (stable anchors + INDEX resolution + thin new-pin guard) is the immediate ratification-and-migration unit; Tiers 2 and 3 are ratified as design direction only, GATED on measured residual leakage after Tier 1 migration lands, informed by a cheap-alternative bake-off. Supersedes v1.0's \"all three tiers ratify as a unit\" framing (R5). (3) Added explicit Rust/WASM-over-remark-lint rationale (single-toolchain grounds: project is Rust-WASM-native; POLICY 21 no_new_shell_scripts platform-agnostic-tooling intent; remark-lint added to Alternatives Considered with reasoned rejection) (R6). (4) Softened overstated framing — \"industry consensus: never version a cross-ref\" reframed as \"dominant pattern for configuration-managed mutable artifacts, not universal law\" with counterexamples (classic DOORS versioned links, versioned APIs, version-specific DOIs, edition-pinned citations) acknowledged; OpenFastTrace's \"safety-certification\" framing corrected from asserted OFT design intent to reasoned inference, and OFT identified as one of several revision-embedding exemplars, not the single counterexample — conclusion (this project does not need forced re-anchoring) unchanged (R2, R3). (5) Fuel claim reframed — \"pulldown-cmark is more fuel-conservative\" corrected from settled premise to an unbenchmarked hypothesis requiring an actual wasmtime bake-off; comrak-on-WASM confirmed viable via `default-features = false` (the \"syntect blocks WASM\" concern is outdated, resolved at comrak 0.13.1); crate versions re-confirmed against crates.io (comrak 0.54.0, pulldown-cmark 0.13.4) (R4). (6) Added pre-commit normalization codemod + thin new-pin guard to Alternatives Considered as the cheap-alternative bake-off comparator against the full Tier-3 AST build (R5). (7) Corrected \"POLICY 22 channel\" mislabel throughout — POLICY 22 (`subagent_report_fidelity_literal_shell`) governs subagent-report fidelity, not human ratification; there is no numbered policy defining a ratification channel; all occurrences replaced with accurate description of the standard human-approval gate for governance-policy/spec amendment (human-as-senior-architect model), enforced by the orchestrator, per CLAUDE.md Pipeline Authority. Also folded in corpus-scan-verified migration-scope numbers (replacing v1.0's unquantified \"internal estimation task\" language): ~3,074 live/load-bearing version pins across 156 files in 6 document classes (stories 73, behavioral-contracts 46, architecture 26, VPs 8, domain-spec 2, prd 1); ~81% of all pins (~13,460) are exempt historical and correctly not migrated; 51% of live pins (1,573) concentrate in the 4 INDEX files (STORY-INDEX 597, BC-INDEX 421, ARCH-INDEX 287, VP-INDEX 268); ~30–40 live ADR pins are anchor-interposed/line-wrapped and require a newline-normalized migration pass; migration magnitude is LARGE, reinforcing a dedicated epic. Status unchanged: PROPOSED, not ratified, not implemented. ADR-045 v1.0→v1.1. [Prior: 2026-08-24 (v1.0) — created by architect: proposes three-tier stable-anchor cross-reference architecture grounded in external research report `.factory/research/wave7-xref-consistency-research.md`; addresses Wave-7 pre-TDD convergence structural floor (passes 4–9) where version/ADR-pin propagation churn proved self-regenerating across all remediation sweeps despite substance-clean implementation across every security/structural/coverage/ownership/wiring axis. Status: proposed; requires human ratification via POLICY 22 channel + a follow-up implementation epic before any corpus change. ADR-045 v1.0.]]]"
input-hash: "e93f383"
---

# ADR-045: Frozen-provenance cross-reference architecture — retain load-bearing version pins as point-in-time provenance, elevate the content-based fingerprint/suspect-link model to the primary churn-solution, and reconcile POLICY 19 with permitted provenance pins

## Context

### The Wave-7 convergence floor

Wave-7 pre-TDD adversarial convergence ran passes 1 through 9 (D-1069 through D-1080 inclusive).
Passes 1–3 (D-1069–D-1074) cleared genuine substance findings across security, structural,
coverage, ownership, and wiring axes. By pass 4 (D-1075), all six wave-7 stories were
implementation-clean: S-21.22 achieved 3/3 CLEAN at pass 5 (D-1076); S-21.20 converged at pass 7
(D-1079) before being reset to 0/3 at pass 8 (D-1080); S-21.22 reset the same pass. At the time
of the original (v1.0) authoring, five of six stories remained at 0/3 despite the implementation
axis being fully correct.

Every straggler finding from pass 4 onward was a version-pin propagation failure. The pattern was
self-regenerating by construction under the PRE-v1.3 policy regime: a remediation burst updates
`BC-1.03.017` from vN to vN+1, which — under the OLD gate semantics — required all citing
documents to be re-anchored in the same commit or be flagged stale. Sweeps missed stragglers in
forms the detectors could not see, the next adversary pass found them, a new remediation burst
updated the BC to vN+2, and the cycle repeated. Four distinct straggler failure modes were
observed:

1. **Line-wrapped cites.** A BC ID and its version token appear on adjacent physical lines within
   a single paragraph. Per-line `grep` cannot find both tokens together; it silently misses the
   straggler. Recurred at a single locus three times across passes (D-1075, D-1078, D-1079).
2. **Anchor-interposed cites.** A semantic anchor intervenes between the ID and the version
   (`ADR-039 §Decision 3 v1.10`). Single-adjacency regex does not match; the straggler is
   invisible to a detector looking for `ADR-039 v1.10` as a contiguous token pair.
3. **Live vs historical ambiguity.** A document's changelog / `modified:` / `last_amended:`
   section legitimately retains the old version at the point-in-time the change was made.
   An undiscriminating sweep that normalizes the historical entry to the current version
   destroys provenance. A sweep that skips those sections misses genuinely live cites in
   adjacent body text if the section boundary is misidentified.
4. **False self-attestation.** State-manager and story-writer attest "full propagation"
   after a sweep. Because the attestation is narrative (not a mechanical gate), the attestor
   cannot detect its own blind spots. Straggler classes 1–3 survive every such attestation.

**v1.3 reframing of the diagnosis (human-directed pivot, 2026-08-25).** v1.0–v1.2 diagnosed the
*existence* of the inline version pin as the disease and prescribed removing it. Re-examined under
the human's explicit "ultimate traceability" requirement, the more precise diagnosis is narrower:
**the disease was never the pin itself — it was the policy requiring the pin to stay LIVE (equal
to the target's current version) at all times.** A frozen, point-in-time pin that nobody is
obligated to keep current is inert with respect to all four failure modes above: failure modes 1–3
only matter because a detector was hunting for *staleness*; if staleness is no longer a defect,
line-wraps, anchor-interposition, and live/historical ambiguity stop being failure modes and become
merely parsing conveniences (still worth solving well, for the fingerprint substrate below, but no
longer load-bearing for correctness). Failure mode 4 (false self-attestation) is closed the same
way in both v1.2 and v1.3 — by replacing narrative attestation with a mechanical gate — but the
v1.3 gate is a content-fingerprint comparison, not a version-string comparison. This reframing does
not discard the Wave-7 evidence; it changes which mechanism the evidence justifies.

### Why this is a design smell, not a detector-quality problem

The external research report (`.factory/research/wave7-xref-consistency-research.md`, §§0–5)
establishes that the root cause is not detector weakness but a reference scheme that the mature
requirements-engineering industry deliberately designed away:

> "Industry consensus (ISO/IEC/IEEE 29148, ReqIF, IBM DOORS/DOORS Next, DITA keyref, Doorstop,
> Structurizr, log4brains, adr-tools) is that a cross-reference stores a STABLE IDENTITY, never
> a target VERSION. The applicable version is resolved from a separately controlled baseline /
> manifest / configuration, or is derived and rendered mechanically." [research §0]

**v1.1 correction (per `.factory/research/adr-045-independent-validation.md` Claim 1, R2 MEDIUM),
retained unchanged by v1.3.** An independent adversarial validation pass re-verified this claim
against fresh sources and found it overstated as written. Stable-identity-plus-resolved-version is
genuinely the **dominant, recommended pattern for mutable artifacts managed as a coherent
configuration** — ISO 29148 treats requirement IDs as non-reusable and change-durable, ReqIF makes
`Identifiable::identifier` lifetime-immutable, and IBM DOORS Next global configurations are the
strongest real "resolve version from a separate config context" exemplar. **But it is not a
universal law, and OpenFastTrace is not the single counterexample.** Classic IBM DOORS supports
genuinely versioned links (`targetVersion(Link)`) where lifecycle semantics require them; versioned
API references embed version inline deliberately (Kubernetes `apiVersion: batch/v1`, OpenAPI
version-bearing `$ref` URIs); version-specific DOIs (Zenodo, DataCite `HasVersion`) denote one
exact version by design; and edition-pinned normative citations (`ISO/IEC/IEEE 29148:2018`,
point-in-time legal citations) travel with the version inline as a matter of correctness. The
defensible formulation — the one this ADR relies on — is: *for mutable artifacts managed as a
coherent configuration, mature requirements platforms commonly store a stable identity and resolve
the applicable state from a baseline/config/checkout.* **v1.3 adds a further nuance this
formulation already accommodated but v1.0–v1.2 did not exploit**: nothing in that formulation
forbids a citing document from ALSO recording, alongside the stable identity, the exact version it
was authored against, PROVIDED that recorded version is not treated as a liveness assertion. Edition-
pinned normative citations and version-specific DOIs are precisely this pattern — a permanent,
frozen point-in-time record co-existing with a resolvable stable identity. v1.3's Tier 1 adopts that
exact shape for this project's cross-references.

OpenFastTrace (`req~name~1`) embeds revision in the reference so that a semantic revision voids all
coverage links and forces re-inspection of every covering item — this mechanical behavior is
confirmed directly from OFT's own guide and design spec. **v1.1 correction (Claim 2, R3 LOW),
retained unchanged by v1.3:** v1.0 characterized this as a documented "safety-certification"
feature; that characterization was this ADR's own inference, not OFT's stated design intent. OFT's
docs describe tracing as a general "safety net for non-trivial software projects" and explicitly
disclaim that producing audit proof is "a main goal" — though OFT is used in functional-safety
contexts (e.g., Xen). OFT is also not the only tool with a revision-embedding design (see the
DOORS/API/DOI/legal-citation counterexamples above). The corrected, better-grounded argument is:
revision-in-identity is a legitimate design whose known, deliberate cost is re-anchoring churn on
every semantic change. [research §4, S4-OFT; validation Claims 1–2, R2, R3]

**v1.3 correction to the conclusion drawn from this evidence (the pivot itself).** v1.0–v1.2 read
the OFT evidence as "this project incurs OFT's deliberate cost without needing OFT's certification-
grade benefit; therefore reject the OFT-shaped design entirely." **That conclusion is now
explicitly reversed by direct human instruction.** The human's stated requirement — "ultimate
traceability" — is precisely the benefit OFT's design is built to buy (forced re-inspection on
substantive change), and the human has judged that benefit worth having for this project. What
changes in v1.3 is not the evidence about OFT; it is the judgment about whether this project wants
OFT's trade-off. v1.3 says yes, with one deliberate narrowing relative to OFT's own scheme: **OFT
voids ALL coverage links for an item on ANY semantic revision bump to that item, project-wide; this
ADR's Tier 2 flags only the specific citing edges that hold a recorded dependency on the changed
target, and only when the target's CONTENT changes, never on a bare version-number increment with
no substantive change underneath it.** This is a controlled, edge-scoped, content-triggered version
of OFT's mechanism — see Rationale and Alternatives Considered below, where the "accept the
OpenFastTrace model" alternative is inverted from REJECTED (v1.0–v1.2) to CHOSEN (v1.3).

### The root cause in VSDD policy — reframed for v1.3

The governance policies inherited by v1.0–v1.2 required inline version citations in load-bearing
positions to be LIVE (mechanically equal to the target's current version):

- **POLICY 7** (`bc_h1_is_title_source_of_truth`) — prescribed the BC-table Version column
  convention as a live-equality cell.
- **POLICY 8** (`bc_array_changes_propagate_to_body_and_acs`) — required the BC-table
  Version cell to match the target BC's current version, enforced by the D-1080
  TABLE-CELL-AWARE PARITY GATE.
- **POLICY 14** (`kk_n_tripartite_parity_gate`) — the 5-leg parity gate whose version-leg
  semantics required an inline version token to match a live target.
- **POLICY 17** (`nn_n_frontmatter_parity_full_file_type_scope`) — frontmatter parity gate
  that included version fields checked for live equality.
- **POLICY 19** (`adr_version_cite_volatile_pin_prohibition`) / **TD-VSDD-091** — forbade
  load-bearing ADR version pins in BC traceability rows (and, per the D-1079 extension, story
  bodies), on the theory that the pin itself was the hazard.

**v1.3 reframing.** The defect in this policy set was never "a pin is present" — it was "the
gate requires the pin to remain current." v1.3 does not retire these policies' concern with
cross-reference integrity; it redirects each policy's version-leg from a **live-equality check**
to a **fingerprint-suspect check**. This is a narrower, more surgical amendment than v1.2's
approach (which retired the version-leg content entirely and replaced it with pure
INDEX-resolution): v1.3 keeps a per-citation liveness *signal*, but the signal is content-drift
on the specific target, not version-number drift. See Decision and Policy Amendments Required
below.

---

## Decision

This ADR adopts a frozen-provenance-plus-suspect-link architecture: load-bearing cross-references
retain their author-recorded version pin as permanent point-in-time provenance, and a
content-based fingerprint/suspect-link mechanism — not version-string equality — becomes the
mechanical signal for "does this citation need re-review." The AST-based parsing work originally
scoped as a residual pin-detector is retained but repurposed as the substrate that computes what
the fingerprint mechanism hashes.

**v1.3 restructure — Tier 2 is now primary and immediate; Tier 1 is redefined, not eliminated;
Tier 3 is repurposed, not deferred.** This inverts the v1.1 restructure's ordering (which made
Tier 1 the immediate unit and gated Tiers 2/3 behind measured need). The reason for the inversion
is direct human instruction, not a re-evaluation of technical merit: the human wants the pins kept,
which makes Tier 1 in its v1.2 form (strip the pins) inapplicable, and wants "ultimate
traceability," which makes Tier 2 (the mechanism that actually delivers meaningful traceability)
the thing worth building first.

### Tier 1 — Stable anchors WITH retained version pins, reinterpreted as frozen provenance (redefined)

All cross-references in load-bearing positions continue to cite the stable anchor. **Unlike v1.2,
the version pin is not removed and its presence is not prohibited.** Concretely:

- **Permitted forms (unchanged from v1.1/v1.2):** bare stable anchors — `BC-1.03.017`,
  `ADR-039 §Decision 3`, `S-21.NN`, `VP-NNN`.
- **Permitted forms (v1.3 addition — the reversal of v1.2's prohibition):**
  `BC-1.03.017 v1.27`, `ADR-039 §Decision 3 v1.10`, `S-21.20 v1.9`, `VP-079 v1.21` in any
  load-bearing section, INCLUDING story ACs, story Tasks, and BC traceability cells. **The pin is
  no longer required to equal the target's current version.** It is read as: "this citation was
  authored or last reviewed against the target at this version." A target version bump forces
  ZERO downstream edits to any citing pin.
- **What is prohibited (the narrowed, v1.3-specific carve-out):** a pin that is explicitly framed
  as a LIVENESS assertion rather than a point-in-time record — e.g., prose that reads "must always
  match vX.Y" or "(current version)" annotated onto a pin — is misleading under the new semantics
  and is a POLICY 19 finding (see Policy Amendments). This is the one new prohibition v1.3
  introduces; it is narrow (a phrasing/framing check, not a presence check) and exists to prevent a
  reader from mistaking frozen provenance for a liveness guarantee the gate no longer enforces.

Version resolution for CURRENT state continues to operate mechanically at lint time against the
project's existing INDEX registries (BC-INDEX, ARCH-INDEX, STORY-INDEX, VP-INDEX) — this Tier-1
capability from v1.1/v1.2 is unchanged and remains useful: any tool or reader that wants "what is
the CURRENT version of BC-1.03.017" resolves it from BC-INDEX, never from grepping citing prose.
What changes is that the citing prose's OWN pin is no longer required to track that current value.

**Consequence:** a version bump forces zero downstream re-anchors (same outcome as v1.2, achieved
by a different means — decoupling the gate from the pin's currency, rather than deleting the pin).
The straggler class — failure modes 1–4 — is neutralized because none of them is load-bearing
anymore: a line-wrapped, anchor-interposed, or "stale-looking" pin is no longer a defect by
construction, because "does the pin match current" is no longer the question the gate asks.

### Tier 2 — Content-based fingerprint/suspect-link, PRIMARY and IN PHASE A (elevated from deferred/gated)

**v1.3 elevates Tier 2 from "deferred, gated on Phase-B measurement" (v1.1/v1.2) to "the primary
churn-solution, built in Phase A, not gated on anything."** This is the direct consequence of
"ultimate traceability": a frozen pin alone tells a reader what version an AC was written against,
but says nothing about whether that target has since changed underneath it. Tier 2 is what makes
the traceability *meaningful* rather than merely *archival*.

The design retains every correction the v1.1 independent validation required (R1, and the four
related hazards under Claim 3) **unchanged**:

- **The fingerprint is content-based, never version-based.** Computed from the target's normalized
  semantic content (heading text + body prose of the cited section, excluding frontmatter/
  changelog, with explicit newline/Unicode normalization and a persisted `hash_version`). Using
  the target's `version:` field as a proxy is withdrawn and remains withdrawn — it would
  re-introduce exactly the version-coupling Tier 1 exists to decouple from.
- **The baseline belongs to the citing edge, not the target.** Stored as
  `{dependent_id, dependency_id, accepted_fingerprint, fingerprint_policy_version}`, one stamp per
  `{dependent, dependency}` pair — matching Doorstop's per-link storage model.
- **`review` and `clear` are two distinct state transitions.** A target being reviewed (its own
  content is now vouched-for) is not the same event as a dependent acknowledging a specific target
  fingerprint. Both are modeled explicitly, not conflated into a single "clear."
- **Validation is read-only.** The gate computes `current-fingerprint(target)`, compares it to
  `accepted_fingerprint`, and reports; it MUST NEVER auto-clear or re-stamp on mismatch. Only an
  explicit, reviewed edit (the `clear`-equivalent transition) may update the stored fingerprint.
  This satisfies D-449(a) directly — the gate computes and reports, an agent never narrates.
- **`null` (unacknowledged) is a distinct workflow state** — unreviewed, not suspect and not clear.
  The gate distinguishes unreviewed / suspect / clear as three states, never collapsing "no
  baseline yet" into "accepted."

**v1.3 addition — the trigger for Tier 2 tracking is now the pin itself, not a separate opt-in
annotation.** v1.1/v1.2 scoped Tier 2 as "optional for any given citation and mandatory only where
a BC/story author explicitly records a content dependency," deferring the sidecar-trigger
mechanism to the implementation epic. Under the frozen-provenance model, that trigger already
exists in the corpus: **every load-bearing citation that carries a version pin IS, by construction,
a declaration that the citing author depended on the target's content as it stood at that version.**
No new markup is introduced. The existing ~3,074 live/load-bearing pins across 156 files (the same
corpus this ADR has scanned since v1.1) become the initial Tier-2 tracked-edge set directly, at
zero additional authoring cost — this is precisely the leverage "ultimate traceability" asked for:
comprehensive coverage using syntax the corpus already contains.

On validation, if `current-fingerprint(target) != accepted_fingerprint`, the citing edge is flagged
**suspect** until a reviewer explicitly clears it via the dependency-acknowledgement transition.
This is mechanically enforceable (a WASM validation hook computes hashes; no human self-attestation
required), directly compatible with D-449(a), bounded (a target change flags only the citing edges
that record a dependency on it, never the entire corpus), and closes failure mode 4 (false
self-attestation) structurally.

### Tier 3 — AST-based content extractor, REPURPOSED as the Tier-2 substrate (built in Phase A, not deferred)

v1.1/v1.2 scoped this as a residual validator: parse with a real Markdown AST, scope to live
sections, normalize soft breaks, tokenize ID and version independently, and resolve against the
INDEX registries to BLOCK any stale or (in the full v1.2 enforcement mode) any newly-introduced
load-bearing pin. **Under v1.3, "block any load-bearing pin" is retired — pins are permitted — but
every other capability this tier built is exactly what Tier 2 needs to compute a correct,
stable fingerprint:**

1. **Parses with a real Markdown AST.** Target crate: **comrak 0.54.0** (crates.io max_stable,
   published 2026-07-12; independently re-verified against the live crates.io registry
   2026-08-25). Alternative: **pulldown-cmark 0.13.4** (independently re-verified, same date). The
   choice between them remains an implementation-epoch task (an actual wasmtime fuel bake-off, per
   the v1.1 R4 correction — no crate ships a published WASM fuel benchmark for documents of this
   corpus's size), not settled by this ADR; comrak's 1:1 node-type fit to the extraction problem
   (explicit `SoftBreak`, opaque `FrontMatter`, explicit `TableCell`) makes it the defensible
   default regardless of the bake-off's outcome.
2. **Scopes to the cited section's canonical content.** Builds heading-based section ranges and
   extracts exactly the section a citation targets, excluding frontmatter, changelog/history
   tables, and code blocks by AST node type — this is now in service of computing "the content this
   citation depends on," not "is this a live/historical section for pin-liveness purposes."
3. **Normalizes soft breaks and whitespace** within each logical text run before hashing, so that
   line-wrapping (the same failure mode 1 from Wave-7) does not perturb the fingerprint — a
   reflowed paragraph with identical semantic content must hash identically.
4. **Persists a `hash_version`** alongside every stored fingerprint so the canonicalization/hashing
   policy itself can evolve later without silently flipping the entire corpus suspect on a
   normalization-rule change.
5. **Does NOT resolve against the INDEX registries for a live/stale verdict** — that check is
   retired. The INDEX-resolution capability from Tier 1 remains for "what is the current version,"
   a separate, non-gating query.

**Pre-implementation research obligation (retained from v1.1/v1.2).** Before building from scratch,
spike **contextlint** and **mdbook-lint** to confirm whether either can be configured/extended
rather than replaced for the section-extraction/canonicalization problem. Neither was found, at the
v1.1 independent-validation pass, to cover all required semantics out of the box (independent ID
detection with wrap/anchor tolerance; canonical section-content extraction; frontmatter/changelog
exclusion) — this ADR retains the Rust/WASM crate as the build target on the same single-toolchain
grounds argued in v1.1 (this project's hook chain is Rust→WASM-native; POLICY 21's
platform-agnostic-tooling rationale applies with equal force to a JS/remark-lint alternative, which
remains technically capable but is rejected on integration-cost grounds, not incapability).

---

## Rationale

**Why frozen provenance, not stripped pins.** The human's explicit requirement — "I don't want to
change the version numbers pinned inside the ACs. I want ULTIMATE TRACEABILITY" — is dispositive.
It is also, independently, a technically coherent position: a stripped pin can only ever answer
"what is the CURRENT version of the target" (via INDEX resolution); a retained pin additionally
answers "what version was the target when THIS citation was authored," which is a strictly richer
historical record and the literal meaning of "traceability." v1.2's INDEX-only model traded that
historical record away in exchange for eliminating churn; v1.3 shows the churn can be eliminated
WITHOUT that trade, by decoupling the gate from the pin's currency rather than deleting the pin.

**Why Tier 2, not Tier 1 alone, is now primary.** A frozen pin with no fingerprint tracking is
inert — it records history but cannot tell a reader whether that history is still trustworthy (has
the cited content since drifted out from under the citation). "Ultimate traceability" requires
both halves: the point-in-time record (Tier 1) AND a live signal for when that record needs
re-examination (Tier 2). Tier 1 alone is necessary but not sufficient for the human's stated goal;
this is why v1.3 elevates Tier 2 to Phase A rather than leaving it gated.

**Why this is a controlled version of the OpenFastTrace model, and why that is now the right
choice — stated plainly, not softened.** v1.0–v1.2 rejected "accept the OpenFastTrace model" on
the grounds that this project does not need certification-grade forced re-inspection. That
rejection was correct GIVEN the goal in place at the time (eliminate the Wave-7 churn with minimum
build cost, no explicit traceability mandate). The human has since supplied a different, explicit
goal — ultimate traceability — that OFT's design is built to serve, and v1.3 accepts the trade-off
OFT accepts: **a real, recurring cost of re-review whenever cited content changes, in exchange for
a real, recurring guarantee that no citation silently drifts out of sync with what it depends on.**
This ADR does not pretend that cost is free. It is deliberately narrower than OFT's own mechanism
in exactly one respect (edge-scoped and content-triggered rather than global-and-revision-triggered
— see Context, above) — a narrowing that keeps the OFT trade-off's benefit while bounding its blast
radius to the citations that actually declared a dependency, and to changes that actually alter
what was depended on.

**Why this is not a reprise of the Wave-7 churn (the load-bearing distinction).** Wave-7's churn had
three properties this mechanism structurally lacks: (1) it fired on a bare **version-number**
increment, regardless of whether the underlying content changed at all — Tier 2 fires only on
**content** change; (2) it required an **edit** to every citing document (re-anchor the pin) —
Tier 2 requires only a **review** action recorded in a sidecar, never a text edit to the citing
prose itself; (3) it was detected by **narrative self-attestation** that regularly missed
stragglers — Tier 2 is detected by a **mechanical, read-only gate**. Because a frozen pin is no
longer a finding under the amended POLICY 7/8/14/17 (see Policy Amendments), and because clearing a
suspect never requires touching the citing AC's own text, the 3-CLEAN convergence loop is not
perturbed by target version bumps at all post-pivot — it is perturbed only by the substantive
content changes "ultimate traceability" exists to surface, which is the intended behavior, not a
regression to the old failure mode.

**Why comrak over pulldown-cmark as the primary substrate (unchanged from v1.1/v1.2).** The
extraction problem Tier 2 now depends on is structurally the same node-structure problem v1.1/v1.2
identified for Tier 3: soft-break normalization, frontmatter exclusion, table-cell scoping, and
section-range computation. comrak exposes all four as first-class AST node types with source
positions and is confirmed to build for `wasm32-unknown-unknown` with `default-features = false`.
The pulldown-cmark fuel tradeoff remains unsettled by benchmark, per the v1.1 R4 correction, and
still requires an actual wasmtime bake-off before finalizing; comrak remains the defensible default
given its 1:1 node-type fit.

**Why POLICY 19 is inverted, not merely narrowed or retired.** Three options were considered for
POLICY 19 given the pivot: (a) **narrow** its scope (keep the prohibition but shrink which
positions it applies to); (b) **retire** it outright (delete the rule, add nothing in its place);
(c) **invert** it (replace the prohibition with a positive mandate covering the same ground). (a)
was rejected because the human's instruction is not "prohibit pins in fewer places" — it is
"permit pins everywhere they already exist," which a narrowed prohibition does not cleanly express.
(b) was rejected because bare retirement leaves a policy vacuum: nothing would then require that a
retained pin be fingerprint-tracked, and "ultimate traceability" specifically requires that
tracking, not merely the pins' passive survival. (c) — inversion — was chosen because it preserves
POLICY 19's original churn-prevention INTENT (stop a stale-looking cross-reference from silently
misleading a reader) while replacing its MECHANISM (prohibit the pin) with the mechanism that
actually delivers on that intent under the new model (fingerprint-track the pin). See Policy
Amendments Required, below, for the precise inverted text.

**Alignment with existing project principles (unchanged).** Tier 3's AST-parsing capability
continues to align with D-449(a) "literal-shell-execution-evidence" — the fingerprint computation
is a mechanical gate executed by the hook chain, not a narrative attestation by the agent.

---

## Consequences

### Positive

- **Ultimate traceability delivered as stated.** Every load-bearing citation retains its
  point-in-time provenance (Tier 1) AND gains an automatic, content-triggered signal for when that
  provenance may have gone stale (Tier 2) — the two halves the human's requirement names explicitly.
- **Zero corpus-edit migration cost.** The ~3,074 existing live/load-bearing pins across 156 files
  remain exactly as authored. What was a LARGE migration cost under v1.2 (an order of magnitude
  above a quick cleanup, per the v1.1 corpus scan) is, under v1.3, not incurred at all — this is now
  a straightforwardly POSITIVE consequence rather than a mitigated negative.
- **3-CLEAN convergence loop stabilized without information loss.** Unlike v1.2 (which stabilized
  the loop by deleting all inline version information), v1.3 stabilizes it by making version
  bumps non-load-bearing while preserving every existing pin, and by replacing the old mechanical
  "does this string match current" churn with a bounded, meaningful, content-triggered suspect
  signal.
- **Wave-7 stragglers close immediately by reclassification, not by an editing burst.**
  S-21.19/20/21/23's "AC cites vN but current is vN+1" findings stop being defects the moment the
  amended gate semantics land — no corpus edit is required, only a baseline-fingerprint entry
  (Migration Plan, Phase A).
- **POLICY 19 reconciled to a positive mandate that matches the human's intent precisely** —
  pins are permitted and mandatorily fingerprint-tracked, rather than prohibited outright.
- **Existing pin syntax becomes the Tier-2 trigger at zero new authoring cost.** No new sidecar
  markup is required to bootstrap comprehensive Tier-2 coverage; the corpus's own ~3,074 pins are
  the initial tracked-edge set.

### Negative / Trade-offs

- **Phase-A build scope is larger than v1.1/v1.2's Tier-1-only plan, though smaller than v1.2's
  full corpus-strip migration.** The fingerprint/suspect-link WASM hook (comrak-based section
  extraction, SHA-based content hashing, edge-local sidecar store, review/clear transition
  handling, read-only validation gate) is now mandatory Phase-A work, not a gated Phase-C
  escalation. This is a real, non-trivial build cost incurred immediately rather than only if
  measured need justifies it.
- **A recurring re-review cost is accepted, by deliberate choice, in exchange for the traceability
  benefit.** Every content change to a fingerprint-tracked target flags every citing edge that
  holds a baseline against it as suspect, requiring a reviewer to examine and clear it. This is
  framed as a feature (see Rationale — it is content-triggered and edge-scoped, unlike the Wave-7
  churn), but it is not free: a target that changes frequently and is widely cited will generate a
  correspondingly wide, though still bounded and non-editing, review backlog.
- **Backlog risk is real and must be measured, not assumed away.** If suspects accumulate faster
  than reviewers clear them, "ultimate traceability" risks degrading into "ultimate noise" —
  a large body of unaddressed suspect flags that nobody trusts or acts on. Phase B (below) exists
  specifically to measure this risk empirically rather than assert it away.
- **Baseline reconstruction for pre-existing pins is imperfect and must be disclosed as such.**
  For the ~3,074 pins that predate this mechanism, the exact content of their targets AT THE
  ORIGINAL AUTHORING TIME cannot be reconstructed (no historical content snapshot exists at that
  granularity). The Phase-A baselining pass necessarily stamps each existing pin with the target's
  CURRENT content fingerprint as of the baselining pass, annotated explicitly as a baseline-pass
  timestamp, not an authoring-time timestamp. Every NEW pin created after this mechanism ships gets
  an exact authoring-time baseline. This is an honest limitation of retrofitting the mechanism onto
  an existing corpus, not a defect in the mechanism's design.
- **Fingerprint normalization risk is unchanged from v1.1/v1.2 but now more consequential.**
  Because Tier 2 is mandatory/primary rather than optional, a canonicalization bug (e.g.,
  under-normalized whitespace producing false suspects on cosmetic reflow) now affects the entire
  corpus's suspect-signal trustworthiness from day one, rather than only a narrow, explicitly-opted-
  in subset. The `hash_version` persistence mechanism (Tier 3) exists specifically to allow
  correcting such a bug without corrupting historical acceptance records, but the initial
  canonicalization design must be gotten right the first time to avoid an early credibility loss
  for the mechanism.
- **POLICY amendments required.** POLICY 7, POLICY 8, POLICY 14, POLICY 17, and POLICY 19 all
  encode version-leg semantics that must change to reflect the frozen-provenance + suspect-link
  model. These amendments are scoped to Phase A and cannot be applied until this ADR is ratified
  (see Status).
- **E-23 requires re-scoping.** The migration epic and its 14 child stories
  (`.factory/stories/epics/E-23-adr045-phase-a-stable-anchor-migration.md`, S-23.01–S-23.14) were
  built entirely around the v1.2 strip model (anchor-pin-detection-classifier,
  precommit-normalization-codemod, thin-new-pin-guard-hook, per-document-class migration bursts,
  Phase-B residual-leakage measurement). None of that story content is applicable to the
  frozen-provenance + suspect-link mechanism-build described here. This ADR does not itself
  re-scope E-23 — that is story-writer/orchestrator-owned work — but flags it explicitly as a
  required follow-up (see Migration Plan and Status, below).

### Status as of v1.3

**ACCEPTED (frozen-provenance + suspect-link model). Implementation NOT STARTED.** On 2026-08-25,
the human directed this pivot at the standard human-approval gate for governance-policy/spec
amendment — human-as-senior-architect, enforced by the orchestrator per CLAUDE.md Pipeline
Authority. This IS the human-ratified direction as of 2026-08-25, replacing v1.2 in place:

- **v1.2 was never durably recorded or applied.** State-manager had not yet run the v1.2
  recording burst when this pivot was directed — `policies.yaml`, `ARCH-INDEX.md`, and
  `STATE.md` remained untouched by v1.2's ratification. Because nothing was ever applied, this
  revision replaces v1.2 in place, as a same-day architecture correction, not as a supersession of
  an applied policy state. `supersedes`/`superseded_by` remain `null` accordingly — there is no
  prior *applied* ADR-045 state to supersede.
- **Ratified (architecture, effective now):** the v1.3 model — Tier 1 redefined as frozen
  provenance, Tier 2 elevated to primary/Phase A, Tier 3 repurposed as the Tier-2 substrate — is
  ACCEPTED as the target architecture. The Policy Amendments Required table below (POLICY
  7/8/14/17/19) is AUTHORIZED for application to `.factory/policies.yaml` by state-manager, per the
  standard Agent Routing Table (ADR content is architect-owned; policy-registry mechanics are
  state-manager-owned).
- **NOT yet done: implementation.** As of this v1.3 recording, none of the following has started:
  the fingerprint/suspect-link WASM hook, the comrak-based section extractor, the corpus-wide
  baseline-fingerprinting pass, or the policy amendments' actual application to `policies.yaml`.
  Those are follow-up work this ratification authorizes to begin — it has not yet begun.
- **E-23 requires re-scoping before Phase A work can proceed under it** — see Consequences and
  Migration Plan. This ADR revision flags the re-scoping need; it does not perform it.
- **Until the policy amendments and fingerprint mechanism land, the pre-pivot gate semantics
  (version-string equality, per POLICY 7/8/14/17 as amended through D-1080) remain nominally in
  force**, though per this ADR's own reclassification the Wave-7 stragglers should not be
  re-flagged as defects pending the amendment landing — this is a narrow, explicit exception
  recorded here so the interim window does not silently re-open the closed stragglers as findings.

**There is no numbered policy that defines a "ratification channel"; POLICY 22 is
`subagent_report_fidelity_literal_shell` (subagent-to-agent report fidelity) and is unrelated to
ratification** (this correction, made at v1.1, remains in force). The implementation requires a
re-scoped epic (see Migration Plan, below).

---

## Alternatives Considered

- **Strip inline version pins entirely, resolve only via INDEX (the v1.2 model).** This was this
  ADR's own prior ratified direction. **Now REJECTED, superseded by direct human instruction
  2026-08-25.** Technical rationale for the rejection, stated on its own merits (not merely "the
  human said so"): stripping the pin discards the one piece of information a frozen pin uniquely
  carries — what version the target was at authoring time — leaving only "what is the CURRENT
  version" (via INDEX), which is a strictly poorer historical record. For a project whose explicit
  goal is "ultimate traceability," that loss is disqualifying even though it does eliminate churn.
  v1.3 shows the churn can be eliminated without the loss (Rationale, above).
- **Accept the current design as intentional (OpenFastTrace model).** v1.0–v1.2 REJECTED this,
  reasoning that this project does not require the certification-adjacent forced re-inspection
  OFT's design is built to force, and that the cost — Wave-7 passes 4–9 stalling on a proven
  substance-clean implementation — is the exact cost OFT's revision-embedding design deliberately
  imposes without the project needing the benefit that cost is meant to buy. **v1.3 INVERTS this:
  now CHOSEN, in a controlled form, per direct human instruction.** The human's "ultimate
  traceability" requirement is exactly the benefit OFT's design exists to provide; v1.3 accepts
  OFT's underlying trade-off (a real re-review cost in exchange for a real drift-detection
  guarantee) while narrowing OFT's own mechanism to be edge-scoped and content-triggered rather
  than global-and-revision-triggered (see Context and Rationale). This is not a new evaluation of
  OFT's technical merits — the v1.1-corrected factual record about OFT (a legitimate design, not
  unique, whose deliberate cost is re-anchoring churn) is unchanged — it is a reversal of the
  PRIOR JUDGMENT about whether this project wants that cost, driven by an explicit human
  requirement that supersedes the earlier judgment. The prior rejection is preserved here as
  historical record, not deleted, per the Standing Rule that spec amendments require explicit
  justification for any reversal.
- **Version-field-as-fingerprint-proxy for Tier 2 (v1.0's original design).** Remains WITHDRAWN,
  unaffected by the pivot. Using the target's `version:` field as the fingerprint would
  re-introduce exactly the version-coupling this ADR's Tier 1 decouples from — a version bump
  would flip every dependent to suspect regardless of whether the content actually changed, which
  is the Wave-7 churn again under a different name. The fingerprint remains strictly content-based.
- **Detector-only (Tier 3 without Tier 1 or Tier 2), unchanged rejection.** A better detector alone
  does not deliver "ultimate traceability" — it can at most tell a reader whether a version number
  matches, not whether the content it depends on has changed, and does nothing to preserve
  point-in-time provenance. Rejected on the same grounds as v1.1/v1.2, now reinforced by the
  explicit traceability requirement.
- **Per-line regex expansion (wider multiline sweep), unchanged rejection.** Irrelevant under the
  pivot — there is no longer a "stale pin" class to detect via regex sweep — but retained here for
  completeness: even if version-liveness detection were still the goal, no regex expansion closes
  all four Wave-7 failure modes simultaneously (line-wrap, anchor-interposition, live/historical
  ambiguity, self-attestation).
- **A single remark-lint custom rule instead of a bespoke Rust/WASM crate, unchanged rejection.**
  The single-toolchain/integration-cost argument from v1.1 (this project's hook chain is
  Rust→WASM-native; adopting remark-lint would add the first Node/JS runtime dependency) applies
  identically to the repurposed Tier-3 content-extractor — the technical capability gap remains
  closed by comrak/pulldown-cmark either way; the rejection is on integration grounds, not
  incapability, unchanged from v1.1.
- **Adopt Sphinx custom domains + reST corpus migration, unchanged rejection.** Requires migrating
  the entire `.factory/` Markdown corpus to reStructuredText — disproportionate relative to a
  custom Rust/WASM validator operating on the existing corpus and hook infrastructure, unaffected
  by the pivot.

---

## Policy Amendments Required (revised 2026-08-25, v1.3 — authorized for application; not yet applied to policies.yaml)

**These amendments REPLACE the v1.2 Policy Amendments table in full.** All five still belong to a
single ratification-authorized unit; none depends on a Phase-B measurement gate (Tier 2/3 are no
longer gated). Ratification AUTHORIZES state-manager to apply them to `.factory/policies.yaml`; as
of this v1.3 recording they have not yet been applied.

| Policy | Current semantic (pre-v1.3) | v1.3 amendment |
|--------|-----------------|-------------------|
| **POLICY 7** (`bc_h1_is_title_source_of_truth`) | BC-table Version column cell contains inline `vN.NN` matching the target BC's current version; TABLE-CELL-AWARE PARITY GATE enforces live match | Version cell contains the stable BC identifier, OPTIONALLY followed by a frozen version pin (`BC-S.SS.NNN vN.NN`) recording the version the citing content was authored/reviewed against. The pin is NOT required to equal BC-INDEX's current version — a mismatch alone is NOT a finding. The parity gate is replaced by a fingerprint-suspect check: if the cell's pin is present, the citing edge must carry a Tier-2 baseline fingerprint against the target; the gate flags the cell only if `current-fingerprint(target) != accepted_fingerprint` AND the edge has not been reviewer-cleared. A pin phrased as a liveness assertion (e.g., "current version," "must match") IS a finding under the narrow POLICY 19 carve-out below. |
| **POLICY 8** (`bc_array_changes_propagate_to_body_and_acs`) | D-1080 TABLE-CELL-AWARE PARITY GATE requires BC-table Version cell to match target's current `vN.NN`; "full propagation" attestation is narrative | The live-equality requirement is RETIRED. When a BC's frontmatter `behavioral_contracts` version changes, downstream citing cells are NOT required to update their pinned version number. Instead: the BC's content-fingerprint changes, and every citing edge with a Tier-2 baseline against it is mechanically flagged suspect by the fingerprint-suspect gate (Tier 2), replacing narrative "full propagation" attestation entirely. Attestation is the gate's own PASS/SUSPECT/CLEAR output, never a narrative claim. |
| **POLICY 14** (`kk_n_tripartite_parity_gate`) | 5-leg parity gate includes a version-leg requiring inline `vN.NN` in specific positions to equal a live target | The version-leg is replaced by a fingerprint-leg: (a) if a pin is present, it is well-formed frozen provenance (not phrased as a liveness assertion); (b) the citing edge's Tier-2 fingerprint-suspect status is either CLEAR, or explicitly acknowledged-suspect-pending-review (an open suspect flag is not itself a gate failure — an unaddressed suspect flag past a defined review SLA, if one is later adopted in Phase C, would be). The 5-leg count and gate structure are preserved. |
| **POLICY 17** (`nn_n_frontmatter_parity_full_file_type_scope`) | Frontmatter parity gate includes version fields requiring inline version tokens to match live targets | Frontmatter fields that cross-reference another document's version (e.g., a story's `behavioral_contracts: [BC-X vN]` array) are frozen provenance, identical in kind to body-prose pins. The parity gate validates: the referenced stable anchor exists in the appropriate INDEX (unchanged from v1.1/v1.2), and — where the frontmatter array is the Tier-2 trigger point for that document class — a Tier-2 baseline fingerprint exists for the edge. Live version-number equality is no longer checked. |
| **POLICY 19** (`adr_version_cite_volatile_pin_prohibition` → renamed `adr_version_cite_frozen_provenance_mandate`) / **TD-VSDD-091 (ADR-version-pin instance only; the line-number-citation instance of TD-VSDD-091 is unaffected and out of scope here)** | Forbade load-bearing ADR version pins in BC traceability rows and (per D-1079) story bodies (ACs, Tasks, Traceability cells), on the theory that any such pin is a stale-liveness hazard | **INVERTED.** Load-bearing ADR version pins are PERMITTED in all positions across all document types (BC bodies, story ACs, story Tasks, traceability cells, architecture section files) as frozen point-in-time provenance. The rule becomes a POSITIVE mandate: any load-bearing ADR version pin MUST (a) not be phrased as a liveness assertion (no "current version," "must match," or equivalent framing attached to the pin — this narrow phrasing check is the one new prohibition v1.3 introduces, and it is the sole remaining POLICY 19 finding class), and (b) once the Tier-2 mechanism ships, carry a Tier-2 baseline fingerprint recording what ADR content the citation depended on. Detection for (a): grep load-bearing ADR citations for liveness-framing phrases co-located with a version token; flag as POLICY 19 HIGH. Detection for (b), once Tier 2 ships: absent-baseline pins are POLICY 19 MEDIUM (a build/rollout gap, not a citing-author error, during the Phase-A rollout window) escalating to HIGH once baselining is complete project-wide. |

---

## Migration Plan (revised, post-pivot 2026-08-25)

**v1.3 replaces the v1.2 Migration Plan's Phase A entirely.** There is no corpus strip. The
~3,074-live-pin / 156-file / 6-document-class / 51%-in-4-INDEX-files corpus scan from v1.1/v1.2 is
retained, but its role changes from "migration scope to strip" to "baselining scope to
fingerprint." Phase B's role changes from "measure whether the strip was sufficient" to "measure
whether the suspect-link mechanism is operationally healthy." Phase C remains optional hardening,
now scoped to different concerns than v1.2's.

### Phase A — mechanism build + corpus baselining (immediate, upon ratification; supersedes v1.2 Phase A)

1. **Ratify ADR-045 v1.3 — DONE (2026-08-25).** Ratified by direct human instruction at the
   standard human-approval gate for governance-policy/spec amendment. The remaining Phase A steps
   are authorized to begin but have not yet started.
2. **Re-scope E-23.** `.factory/stories/epics/E-23-adr045-phase-a-stable-anchor-migration.md` and
   its 14 child stories (S-23.01–S-23.14) were built for the v1.2 strip model and do not describe
   the work below. Re-scoping E-23 (or retiring it and opening a new epic) is story-writer/
   orchestrator-owned work, flagged here as a required follow-up, not performed by this ADR
   revision.
3. **Build the fingerprint/suspect-link WASM hook.** A new crate under `crates/hook-plugins/`
   implementing: comrak-based (or pulldown-cmark, pending the wasmtime fuel bake-off)
   section-scoped canonical content extraction; SHA-based content fingerprinting with a persisted
   `hash_version`; an edge-local sidecar store keyed by `{dependent_id, dependency_id}`; the
   `review` and `clear` transitions modeled as two distinct operations; and a read-only validation
   gate that reports unreviewed/suspect/clear status without ever auto-clearing.
4. **Amend POLICY 7/8/14/17/19** in policies.yaml per the table above (state-manager, ratification
   burst).
5. **Baseline the existing corpus.** For each of the ~3,074 live/load-bearing pins across 156
   files, compute and store the target's CURRENT content fingerprint as the `accepted_fingerprint`,
   explicitly timestamped as a baseline-pass stamp (not a reconstructed authoring-time stamp — see
   Consequences, Negative/Trade-offs). Prioritize the 4 INDEX files (1,573 pins, 51% of the total)
   first, matching v1.1/v1.2's identified locus of maximum leverage. The ~30–40 anchor-interposed/
   line-wrapped ADR pins require the same newline-normalized extraction pass identified in v1.1/
   v1.2 — now for fingerprint-source-extraction correctness rather than pin-detection correctness.
6. **Wave-7 stragglers close by reclassification.** S-21.19/20/21/23's "AC cites vN but current is
   vN+1" findings are reclassified as legitimate frozen provenance once the amended gate semantics
   (step 4) land and each straggler's citing edge receives its baseline fingerprint (step 5) — no
   text edit to any story AC is required.

### Phase B — operational-health measurement window (no new build; retained structurally from v1.1/v1.2, re-scoped)

7. **Measure suspect-link operational health** over a defined post-Phase-A window: false-suspect
   rate (fingerprint drift attributable to formatting/normalization noise rather than substantive
   content change — a canonicalization-quality signal), suspect-clearing latency and backlog size
   (a reviewer-capacity signal, directly addressing the "ultimate traceability degrading into
   ultimate noise" risk flagged in Consequences), and baseline coverage completeness (% of
   load-bearing pins with a recorded Tier-2 baseline). This measurement is materially different
   from v1.2's Phase B (which measured residual STRIP leakage) — it measures whether the NEW
   mechanism is trustworthy and sustainable in practice, not whether an alternative mechanism is
   needed.

### Phase C — optional hardening (gated on Phase B data; not authorized by ratifying this ADR alone)

8. **If Phase B shows a high false-suspect rate:** revisit the canonicalization/normalization rules
   (whitespace, Unicode, list/table edge cases) and bump `hash_version`, re-baselining affected
   edges without disturbing unaffected ones.
9. **If Phase B shows a growing, unaddressed suspect backlog:** consider review-SLA tooling
   (staleness thresholds, priority tiers by module-criticality, batched review workflows) — this is
   the point at which an unaddressed suspect flag could reasonably become a hard gate failure rather
   than an informational flag, per the POLICY 14 amendment's forward-reference above.
10. **If Phase B shows the wasmtime fuel bake-off (comrak vs. pulldown-cmark) still unresolved
    from Phase A:** finalize it against representative large-artifact fixtures before further
    scaling the extractor's use.
11. **If Phase B shows negligible operational risk:** the Phase-A mechanism is the standing
    steady-state architecture; no further build is required.

Re-scoped E-23 (or its replacement epic) is the suggested anchor for Phase A. A dedicated epic
remains appropriate because the mechanism-build and corpus-baselining pass span every document
class in `.factory/` and require coordinated dispatches across architect (Tier-3 crate design
detail, if any residual ambiguity), implementer (the WASM hook crate), state-manager (policy
amendments + baselining-pass commits), and consistency-validator (post-baseline verification).

---

## Source / Origin

- **Evidence base (v1.0):** `.factory/research/wave7-xref-consistency-research.md` (research-agent,
  2026-08-24; four Perplexity deep-research calls, two crates.io registry verifications; all
  sources cited as S1-/S2-/S3-/S4-/V- footnotes).
- **Evidence base (v1.1):** `.factory/research/adr-045-independent-validation.md` (research-agent,
  independent adversarial validation pass, 2026-08-25; VERDICT: PARTIALLY-SUPPORTS ratification;
  four independent Perplexity deep-research calls plus two independent crates.io WebFetch
  verifications; findings R1–R7 and Claims 1–6 drive every technical correction retained through
  v1.3 — see the `modified:` changelog for the full list).
- **Wave-7 convergence evidence:** D-1069 through D-1080 (STATE.md Decisions Log; full detail
  in `.factory/cycles/v1.0-brownfield-backfill/decision-log.md`).
- **Human direction (v1.3 pivot):** direct instruction from the human (senior architect),
  2026-08-25, verbatim: "I don't want to change the version numbers pinned inside the ACs. I want
  ULTIMATE TRACEABILITY." This is the dispositive input for every v1.2→v1.3 change in this
  revision; it is a business/architecture-authority decision, not a re-derivation from the
  underlying research (the underlying research's factual claims about ISO 29148/ReqIF/DOORS/OFT/
  Doorstop/comrak/pulldown-cmark are unaffected by the pivot and are retained unchanged).
- **Existing policy context:** `.factory/policies.yaml` POLICY 19 / TD-VSDD-091 (the
  ADR-version-cite rule this ADR now inverts); POLICY 21 `no_new_shell_scripts`
  (single-toolchain/platform-agnostic-tooling rationale cited for the Rust/WASM-over-remark-lint
  decision); POLICY 22 `subagent_report_fidelity_literal_shell` (subagent-to-agent report
  fidelity — **not** a ratification-channel policy; v1.0 mislabeled it as such and that mislabel
  remains corrected).
- **Applicable governance:** the standard human-approval gate for governance-policy/spec
  amendment (human-as-senior-architect, enforced by the orchestrator per CLAUDE.md Pipeline
  Authority — no numbered policy defines this gate); TD-VSDD-053 (single-commit-per-burst;
  policy-amendment and baselining bursts must respect this constraint).
- **Standards cited in research (unchanged relevance):** ISO/IEC/IEEE 29148:2018, OMG ReqIF 1.2,
  DITA 1.2 keyref, IBM DOORS / DOORS Next, Doorstop 3.0.2, OpenFastTrace 4.2.0. [research §§2.1, 4]
- **Rust crate versions (registry-verified twice — v1.0 and independently re-verified for
  v1.1; unaffected by the pivot):** comrak 0.54.0 (crates.io 2026-07-12); pulldown-cmark 0.13.4
  (crates.io 2026-05-20). [research V-comrak, V-pulldown; validation Claim 4 — both independently
  confirmed accurate 2026-08-25]
- **Migration-scope corpus scan (v1.1, re-read as baselining scope under v1.3):** verified
  2026-08-25 — ~3,074 live pins / 156 files / 6 document classes; ~13,460 exempt-historical pins
  (~81% of corpus total, unaffected by the pivot — still correctly excluded from Tier-2
  baselining as non-load-bearing); 1,573 live pins (51%) concentrated in the 4 INDEX files; ~30–40
  anchor-interposed/line-wrapped ADR pins. Cited in Consequences and Migration Plan Phase A.
- **Downstream follow-up flagged by this revision (not resolved here):**
  `.factory/specs/architecture/ARCH-INDEX.md` row for ADR-045 (currently line ~547) still shows
  v1.0, status PROPOSED, and the pre-v1.1 "POLICY 22 channel" mislabel — this row requires
  correction to v1.3 / status accepted / corrected ratification-channel description, by
  state-manager, in the same recording burst that applies the Policy Amendments table above.
  `.factory/stories/epics/E-23-adr045-phase-a-stable-anchor-migration.md` and S-23.01–S-23.14
  require re-scoping to the frozen-provenance + suspect-link mechanism-build (story-writer/
  orchestrator-owned).
</content>
