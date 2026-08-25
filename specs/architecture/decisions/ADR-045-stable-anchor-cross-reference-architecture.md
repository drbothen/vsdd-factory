---
document_type: adr
adr_id: ADR-045
version: "1.0"
title: "ADR-045: Stable-anchor cross-reference architecture — eliminate load-bearing inline version pins, adopt the Doorstop fingerprint/suspect-link model, and build an AST-based residual validator to close Wave-7 version-pin convergence churn by construction"
status: proposed
date: 2026-08-24
producer: architect
deciders:
  - architect
  - human (ratification required — see Status)
subsystems_affected: [SS-01, SS-04, SS-05, SS-07]
supersedes: null
superseded_by: null
traces_to: .factory/specs/architecture/ARCH-INDEX.md
extends: null
phase: brownfield-backfill
cycle: v1.0-brownfield-backfill
inputs:
  - .factory/research/wave7-xref-consistency-research.md
  - .factory/policies.yaml
  - .factory/specs/architecture/ARCH-INDEX.md
modified:
  - "2026-08-24 (v1.0) — created by architect: proposes three-tier stable-anchor cross-reference architecture grounded in external research report `.factory/research/wave7-xref-consistency-research.md`; addresses Wave-7 pre-TDD convergence structural floor (passes 4–9) where version/ADR-pin propagation churn proved self-regenerating across all remediation sweeps despite substance-clean implementation across every security/structural/coverage/ownership/wiring axis. Status: proposed; requires human ratification via POLICY 22 channel + a follow-up implementation epic before any corpus change. ADR-045 v1.0."
input-hash: "2c7e942"
---

# ADR-045: Stable-anchor cross-reference architecture — eliminate load-bearing inline version pins, adopt the Doorstop fingerprint/suspect-link model, and build an AST-based residual validator to close Wave-7 version-pin convergence churn by construction

## Context

### The Wave-7 convergence floor

Wave-7 pre-TDD adversarial convergence ran passes 1 through 9 (D-1069 through D-1080 inclusive).
Passes 1–3 (D-1069–D-1074) cleared genuine substance findings across security, structural,
coverage, ownership, and wiring axes. By pass 4 (D-1075), all six wave-7 stories were
implementation-clean: S-21.22 achieved 3/3 CLEAN at pass 5 (D-1076); S-21.20 converged at pass 7
(D-1079) before being reset to 0/3 at pass 8 (D-1080); S-21.22 reset the same pass. At the time
of this ADR's authoring, five of six stories remain at 0/3 despite the implementation axis being
fully correct.

Every straggler finding from pass 4 onward is a version-pin propagation failure. The pattern is
self-regenerating by construction: a remediation burst updates `BC-1.03.017` from vN to vN+1, which
requires all citing documents — every story body, every BC-table Version cell, every traceability
row — to be re-anchored in the same commit. Sweeps miss stragglers in forms the detectors cannot
see, the next adversary pass finds them, a new remediation burst updates the BC to vN+2 (or updates
an ADR, triggering another cohort-wide sweep), and the cycle repeats. Four distinct straggler
failure modes were observed:

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

### Why this is a design smell, not a detector-quality problem

The external research report (`.factory/research/wave7-xref-consistency-research.md`, §§0–5)
establishes that the root cause is not detector weakness but a reference scheme that the mature
requirements-engineering industry deliberately designed away:

> "Industry consensus (ISO/IEC/IEEE 29148, ReqIF, IBM DOORS/DOORS Next, DITA keyref, Doorstop,
> Structurizr, log4brains, adr-tools) is that a cross-reference stores a STABLE IDENTITY, never
> a target VERSION. The applicable version is resolved from a separately controlled baseline /
> manifest / configuration, or is derived and rendered mechanically." [research §0]

The single counterexample — OpenFastTrace (`req~name~1`) — embeds revision in the reference
specifically so that a semantic revision voids all coverage links and forces manual re-inspection.
That is a feature for safety-certification re-review; it is also a precise description of the
churn this project is experiencing. The research concludes: if the project does not need
certification-grade forced re-review, it should not be using the certification-grade
forced-re-review reference scheme. [research §4, S4-OFT]

### The root cause in VSDD policy

The current governance policies require inline version citations in load-bearing positions:

- **POLICY 7** (`bc_h1_is_title_source_of_truth`) — prescribes the BC-table Version column
  convention, including inline `vN.NN` cells in BC body traceability tables.
- **POLICY 8** (`bc_array_changes_propagate_to_body_and_acs`) — requires the BC-table
  Version cell to match the target BC's current version, enforced by the D-1080
  TABLE-CELL-AWARE PARITY GATE.
- **POLICY 14** (`kk_n_tripartite_parity_gate`) — the 5-leg parity gate whose version-leg
  semantics require an inline version token to match a live target.
- **POLICY 17** (`nn_n_frontmatter_parity_full_file_type_scope`) — frontmatter parity gate
  that includes version fields.
- **POLICY 19** (`adr_version_cite_volatile_pin_prohibition`) / **TD-VSDD-091** — already
  forbids load-bearing ADR version pins in BC traceability rows, but the prohibition is
  scoped only to that specific cite location. The straggler class it closes (ADR version
  in BC traceability rows) does not cover story bodies, BC table cells, or other load-bearing
  positions, which each require their own equivalent prohibition.

Policies 7, 8, 14, and 17 collectively mandate what the research identifies as the anti-pattern.
Policy 19 / TD-VSDD-091 demonstrates that narrowly-scoped elimination works — but each new
location requires a new policy extension, and the straggler class outruns each extension.

---

## Decision

This ADR proposes a three-tier hybrid architecture that eliminates the root cause at the schema
level, adopts a mechanical suspect-link model for genuine content dependencies, and builds a
residual AST-based validator to guard the migration and any legitimately retained version pins.
All three tiers must be ratified as a unit; partial adoption of Tier 1 without Tier 3 leaves
the corpus unguarded during migration.

### Tier 1 — Stable anchors only; version resolved mechanically (eliminates the disease)

All cross-references in load-bearing positions cite the stable anchor only. The version is
never hand-authored in a citing document. Concretely:

- **Permitted forms:** `BC-1.03.017`, `ADR-039 §Decision 3`, `S-21.NN`, `VP-NNN`.
- **Prohibited forms:** `BC-1.03.017 v1.27`, `ADR-039 §Decision 3 v1.10`,
  `S-21.20 v1.9`, `VP-079 v1.21` in any live, load-bearing section.
- **Permitted exceptions** (non-load-bearing, explicitly marked):
  - Provenance parentheticals: `(as of v1.27 at D-1080)` in a historical note, not a
    traceability cell or AC.
  - Frontmatter `modified:` / `last_amended:` / `[Prior:]` changelog rows — these are
    explicitly historical and must be excluded from live-consistency scanning (see Tier 3).
  - Decision-log D-NNN prose that discusses a SPECIFIC prior revision by version as part of
    the historical record — same exclusion.

Version resolution operates mechanically at lint time against the project's existing INDEX
registries: BC-INDEX for BCs, ARCH-INDEX for ADRs, STORY-INDEX for stories, VP-INDEX for VPs.
The Tier-3 validator enforces this. Where rendered documentation requires visible provenance
(e.g., a generated audit trail), the version annotation is generated from the INDEX at render
time, not hand-maintained in every citing doc.

**Consequence:** a version bump forces zero downstream re-anchors. The straggler class — failure
modes 1, 2, 3, and 4 — vanishes by construction because there is no inline version token to go
stale. This directly ends perturbation of the 3-CLEAN convergence loop: a target version bump
no longer forces edits to any citing document.

### Tier 2 — Doorstop fingerprint/suspect-link model for genuine content dependencies

Where a citing document genuinely depends on a target's CONTENT at a point in time — not merely
its identity — that dependency is recorded as a machine-readable sidecar entry:
`target-UID: <fingerprint-at-last-review>`. The applicable fingerprint is the target's current
content hash (or its existing `version:` field, used as a stable proxy).

On validation, if `current-fingerprint(target) != stored-fingerprint`, the citing document is
automatically flagged **suspect** until a reviewer explicitly clears it (the Doorstop
`doorstop clear` equivalent). This is:

- Mechanically enforceable (a WASM validation hook computes hashes; no human self-attestation
  required).
- Directly compatible with the D-449(a) "literal-shell-execution-evidence" principle: the gate
  computes and reports, rather than asking an agent to narrate.
- Bounded: a target change flags only the documents that record a dependency on it, not the
  entire corpus.
- Closing: failure mode 4 (false self-attestation) is structurally impossible when the
  attestation is computed by the gate, not narrated by the agent.

Tier 2 is optional for any given citation and mandatory only where a BC/story author explicitly
records a content dependency (e.g., "this AC requires BC-1.03.017 to expose the
`plugin_fail_closed_on_exhaustion` function signature defined at its current heading"). The
sidecar format is TBD in the implementation epic; the principle is ratified here.

### Tier 3 — AST-based residual validator as WASM lint hook

For any version pins that remain live after the corpus migration — whether intentionally retained
in exempt historical sections or residually present during the migration window — the project
replaces regex/manual sweeps with a WASM lint hook built on the Rust Markdown AST. The validator:

1. **Parses with a real Markdown AST.** Target crate: **comrak 0.54.0** (verified: crates.io
   max_stable 0.54.0, published 2026-07-12 [research §3.2, V-comrak]) for AST richness
   (arena AST, `root.descendants()`, explicit `NodeValue::SoftBreak` and `LineBreak` inline
   nodes, `sourcepos` on every node, opaque `FrontMatter(String)` node, `Table`/`TableCell`
   nodes). Alternative: **pulldown-cmark 0.13.4** (verified: crates.io max_stable 0.13.4,
   published 2026-05-20 [research §3.2, V-pulldown]) if the WASM fuel budget favors a
   streaming pull-parser with smaller arena allocation — directly relevant given the project's
   documented fuel-exhaustion constraints on large artifacts (ADR-042). Do NOT use
   tree-sitter-markdown; the grammar's own README explicitly warns against correctness-critical
   use. [research §3.2]
2. **Scopes to live sections only.** Builds heading-based section ranges (heading H → next
   heading of depth ≤ H). Applies an allowlist of load-bearing section paths (e.g.
   `Decision`, `Consequences`, `Rationale`, `Implementation`) and a structural denylist
   (headings named `Changelog`, `History`, `Revision History`, `Superseded`, `Examples`,
   `Modified`, `Prior`). Excludes the `FrontMatter` node by type (not by scanning for `---`),
   changelog / history table ancestors by `Table` parent context, and all code blocks.
   [research §3.1, S3-sections]
3. **Normalizes soft breaks.** Within each logical text run (paragraph, list item, table cell),
   collects `Text` nodes and converts `SoftBreak` / `LineBreak` inline nodes to a single space.
   This defeats failure mode 1 (line-wrapped cites) by construction — the ID and version token
   appear in one logical text run regardless of physical wrapping. [research §3.1, S3-algo]
4. **Tokenizes ID and version independently.** Uses separate patterns: `\bBC-\d+\.\d+\.\d+\b`,
   `\bADR-\d+\b`, `\bS-\d+\.\d+\b`, `\bVP-\d+\b` for identities; `\bv(\d+(?:\.\d+)*)\b` for
   versions. Associates the nearest-following version token within the same logical block,
   stopping at another identity token or a sentence boundary / N-token window. This defeats
   failure mode 2 (anchor-interposed cites — `§Decision 3` between the ID and the version is
   ignored) by construction. [research §3.1, S3-algo]
5. **Resolves against the INDEX registries.** For each `(canonical-ID, parsed-version)` pair
   found in a live section, queries the appropriate INDEX (BC-INDEX, ARCH-INDEX, STORY-INDEX,
   VP-INDEX) for the canonical current version. If `parsed-version != current-version`,
   emits a diagnostic with source position (file, line, column) and BLOCKS the commit.
6. **Tier 1 enforcement mode.** When fully deployed post-migration, the validator's primary
   mode is to DETECT any load-bearing inline version pin at all (not merely a stale one) in
   a live section and BLOCK the commit, per Tier 1's prohibition. During the migration window,
   it operates in "stale-only" mode — pins present but current do not block; stale pins block.
7. **WASM fuel posture.** Given the project's known fuel-exhaustion issues on large artifacts
   (ADR-042; lessons.md >3000 lines regularly exhausts 10M–20M fuel), the validator MUST be
   designed for bounded fuel consumption: streaming parse mode if comrak's arena allocation
   proves too large, per-file dispatch (never full-corpus in one invocation), and an
   early-return path once a blocking violation is found. An explicit `fuel_cap` per-plugin
   override (ADR-039 Phase 1 mechanism) SHOULD be set once that mechanism ships.

**Pre-implementation research obligation.** Before building from scratch, spike
**contextlint** (contextlint.dev; rules REF-002 + GRP-001 are the closest semantic prior art)
and **mdbook-lint** (github.com/joshrotenberg/mdbook-lint, v0.14.3, 2026-03-04; ADR-specific
rules) to confirm whether either can be configured/extended rather than replaced. [research §3.3]
If neither covers the five required semantics (prose ID + version detected independently;
associated despite wrapping/intervening text; compared to a canonical version registry; only
live/load-bearing heading ranges counted; frontmatter and changelog/history tables excluded
structurally), build the Rust/WASM rule as a new hook plugin under `crates/hook-plugins/`.

---

## Rationale

**Why Tier 1 is primary, not Tier 3.** A better detector still leaves the churn intact: every
version bump still forces a cohort-wide edit, and the 3-CLEAN convergence loop is still
perturbed on every bump. The detector treats the symptom; the architectural change removes the
disease. The OpenFastTrace evidence from the research report is conclusive: the only mature
tool that embeds revision-in-reference does so specifically to force re-anchoring on every
semantic revision — that design is reserved for safety certification, not for spec-corpus
coherence. [research §4, S4-OFT] Every other surveyed tool (adr-tools, log4brains,
Structurizr, Doorstop, Sphinx-Needs, StrictDoc, IBM DOORS) separates identity from version.
[research §4]

**Why the hybrid, not pure architectural.** The corpus already contains thousands of existing
version pins, accumulated across every phase of the brownfield-backfill cycle. Some provenance
citations are legitimately version-bearing (decision-log D-NNN prose discussing a specific prior
revision). A detector is needed during migration and to guard the residual set. Tier 3 is
necessary but not sufficient as a standalone strategy; hence hybrid, not pure architectural
elimination. [research §0]

**Why the Doorstop fingerprint model for Tier 2.** Doorstop (3.0.2, 2025-06-16) is the only
surveyed tool that implements the exact mechanism needed for failure mode 4 (false
self-attestation): a machine-readable `UID: fingerprint` link that is automatically flagged
suspect when the target changes, cleared only by a reviewed edit. IBM DOORS / DOORS Next
implements the same "suspect on target change, clear from either endpoint" semantic but is a
proprietary enterprise tool. Doorstop's model is portable to this project's existing INDEX
registries and WASM-hook gate architecture with minimal structural change. [research §4,
S4-Doorstop, S4-DOORS] Sphinx-Needs and StrictDoc have stable-ID + generated traceability
matrices but do not implement per-link suspect-on-content-change semantics. [research §4,
S4-sphinxneeds, S4-strictdoc]

**Why comrak over pulldown-cmark as the primary substrate.** The four straggler failure modes
are all node-structure problems: soft-break normalization (explicit `NodeValue::SoftBreak`
nodes needed), frontmatter exclusion (opaque `FrontMatter` node type), table-cell scoping
(explicit `TableCell` nodes), and section-range computation (heading-to-heading ancestor
tracking). comrak exposes all four as first-class AST node types with source positions. The
tradeoff is arena allocation vs WASM fuel; pulldown-cmark's streaming event model is the
fuel-conservative fallback if empirical measurement shows comrak's arena exceeds the available
fuel budget for the largest corpus files. [research §3.2]

**Alignment with existing project principles.** Tier 1 generalizes the existing POLICY 19 /
TD-VSDD-091 principle from "ADR cites in BC traceability rows" to ALL cross-reference positions.
It does not introduce a new category of rule; it extends a proven, already-codified prohibition
to close the gap that each pass-N finding exploits. Tier 3 aligns with D-449(a)
"literal-shell-execution-evidence" — the validator is a mechanical gate executed by the hook
chain, not a narrative attestation by the agent. Both properties are already requirements;
this ADR shows how to satisfy them together for the version-pin problem class.

---

## Consequences

### Positive

- **Cohort re-anchoring eliminated by construction.** A version bump on any BC, ADR, story, or
  VP forces zero downstream document edits. The entire straggler class — failure modes 1–4 —
  cannot exist in the post-migration corpus.
- **3-CLEAN convergence loop stabilized.** Adversarial passes no longer find version-pin
  residue in fully-remediated stories. The floor that held Wave-7 convergence at 0/3 across
  passes 4–9 is structurally removed.
- **Attestation becomes mechanical.** Tier 2 fingerprint gates and Tier 3 validator gates
  replace narrative "full propagation" attestations, directly satisfying D-449(a).
- **POLICY 19 / TD-VSDD-091 generalized uniformly.** A single rule class covers all
  cross-reference positions rather than requiring per-location policy extensions for each
  newly-observed straggler form.
- **Alignment with ISO/IEC/IEEE 29148, ReqIF, DITA, Doorstop, adr-tools, log4brains.**
  The project's reference scheme becomes consistent with the design of every major
  docs-as-code and requirements-traceability tool surveyed. [research §§2.1, 4]

### Negative / Trade-offs

- **One-time corpus migration cost.** Every load-bearing inline version pin in the
  `.factory/` corpus must be stripped to its stable anchor form. The research establishes the
  target design but not the migration cost; that is an internal estimation task. The migration
  must be a dedicated atomic commit (or small set of atomic commits) per document class to
  remain within TD-VSDD-053's single-commit-per-burst discipline.
- **Build cost: Tier-3 validator.** A new Rust/WASM hook plugin must be authored,
  unit-tested, and integrated into `hooks-registry.toml`. Estimated scope: medium (one crate,
  ~200–400 lines of Rust, leveraging comrak or pulldown-cmark as a dependency). The
  contextlint/mdbook-lint spike may shorten this if either can be extended.
- **Loss of inline point-in-time provenance in load-bearing positions.** Readers of a BC
  traceability table or story AC cannot determine from that cell alone which version of the
  target was current when the AC was written. Mitigation: the changelog / `modified:` row
  in the same document retains the version-at-authoring record (exempt from Tier 1
  prohibition); the Tier-2 sidecar records the exact fingerprint for content-dependent
  citations; the INDEX registries provide the canonical current version on demand.
- **POLICY amendments required.** POLICY 7, POLICY 8, POLICY 14, POLICY 17, and POLICY 19
  all encode conventions whose "version-leg" semantics must change to reflect Tier 1.
  These amendments cannot be applied until this ADR is ratified. During the window between
  ratification and migration, existing version-pin conventions remain in force for the
  pre-migration corpus.
- **Tier-2 sidecar format is TBD.** The Doorstop fingerprint/suspect-link model is adopted
  as a principle; the exact sidecar schema (YAML frontmatter extension vs separate `.links`
  file vs augmented INDEX row) is deferred to the implementation epic. This is a design
  detail, not a structural uncertainty.

### Status as of v1.0

**PROPOSED. Not ratified. Not implemented.** This ADR amends multiple governance policies and
requires human ratification via the POLICY 22 channel before any corpus change. No existing
document has been modified as a consequence of authoring this ADR. The implementation requires
a dedicated epic (suggested anchor: S-15.03 PRIORITY-A or a new wave-9 epic); specific stories
are: (1) build Tier-3 validator crate + contextlint/mdbook-lint spike; (2) implement Tier-1
INDEX-resolution in the validator; (3) mechanical corpus migration (strip load-bearing version
pins → stable anchor form); (4) implement Tier-2 fingerprint sidecar schema and suspect-flag
gate; (5) amend POLICY 7/8/14/17/19 in policies.yaml to reflect the new conventions. Wave-7's
residual version-pin tail closes as part of step 3.

---

## Alternatives Considered

- **Detector-only (Tier 3 without Tier 1 or Tier 2).** Build a better regex or AST-based
  detector without changing the reference scheme. Rejected as primary strategy: even a
  perfect detector leaves every version bump forcing a cohort-wide edit, and the 3-CLEAN
  convergence loop remains perturbed on every bump. The OpenFastTrace research evidence is
  decisive — the design is the recognized anti-pattern for systems that do not need
  safety-certification forced re-review. [research §5] Tier 3 is adopted as the residual
  guard, not the primary fix.
- **Per-line regex expansion (wider multiline sweep).** Extend POLICY 5's `tr '\n' ' ' | grep`
  discipline to cover more forms. Rejected: the D-1079 POLICY 5 extension already codified
  the multiline detector for the line-wrap class. The anchor-interposed class (failure mode 2)
  still requires independent tokenization; the live/historical class (failure mode 3) still
  requires section-range logic; the self-attestation class (failure mode 4) is inherently
  a process failure, not a detector failure. No regex expansion closes all four simultaneously.
- **Pure architectural elimination without a residual detector.** Adopt Tier 1 only, skip
  Tier 3. Rejected: the corpus already contains thousands of historical version pins, some
  legitimately version-bearing in exempt sections. During migration, and for the residual
  exempt set, no mechanical gate would enforce the Tier 1 prohibition. The risk of partial
  migration (some documents migrated, some not) with no gate is higher than the cost of
  building Tier 3.
- **Accept the current design as intentional (OpenFastTrace model).** Treat every version bump
  as a deliberate trigger for cohort-wide re-inspection, analogous to OpenFastTrace's
  `req~name~revision` scheme. Rejected: this project does not require safety-certification
  re-inspection on every version bump. The cost — passes 4–9 of Wave-7 stalled on a proven
  substance-clean implementation — is the exact cost the OpenFastTrace design deliberately
  imposes, and the project does not receive the certification benefit in return. [research §4,
  S4-OFT]
- **Adopt Sphinx custom domains + reST corpus migration.** Sphinx provides the strongest
  native version-consistency model surveyed (custom domain + `resolve_xref` + missing-reference
  event + `-n -W` as a CI gate). Rejected: requires migrating the entire `.factory/` Markdown
  corpus to reStructuredText and adopting a Sphinx build pipeline. The migration overhead is
  disproportionate relative to the custom Rust/WASM validator, which operates natively on the
  project's existing Markdown corpus and WASM hook infrastructure. [research §1, S1-sphinx]

---

## Policy Amendments Required (proposed, not yet applied)

The following policies must be amended when this ADR is ratified. State-manager applies them
to policies.yaml as part of the ratification burst; no agent may apply them before ratification.

| Policy | Current semantic | Proposed amendment |
|--------|-----------------|-------------------|
| **POLICY 7** (`bc_h1_is_title_source_of_truth`) | BC-table Version column cell contains inline `vN.NN` matching the target BC's current version; TABLE-CELL-AWARE PARITY GATE enforces match | Version cell contains the stable BC identifier only (`BC-S.SS.NNN`); the parity gate resolves the current version from BC-INDEX at lint time and verifies it mechanically; inline `vN.NN` tokens in load-bearing cells are PROHIBITED and cause the Tier-3 validator to BLOCK |
| **POLICY 8** (`bc_array_changes_propagate_to_body_and_acs`) | D-1080 TABLE-CELL-AWARE PARITY GATE requires BC-table Version cell to match target's current `vN.NN`; "full propagation" attestation is narrative | Parity gate is recast as an INDEX-resolution check: the cell's stable anchor must be present in BC-INDEX and the INDEX's current version is the ground truth; attestation is the Tier-3 validator's PASS/BLOCK output, not a narrative claim |
| **POLICY 14** (`kk_n_tripartite_parity_gate`) | 5-leg parity gate includes a version-leg requiring inline `vN.NN` in specific positions | Version-leg semantics change to stable-anchor presence + INDEX-derived version match; the version token in the parity cell is eliminated; the 5-leg count and gate structure are preserved with the version leg checking stable-anchor + INDEX resolution instead of inline-token match |
| **POLICY 17** (`nn_n_frontmatter_parity_full_file_type_scope`) | Frontmatter parity gate includes version fields requiring inline version tokens to match live targets | Version fields in frontmatter that cross-reference live targets change to stable-anchor form; the parity gate validates anchor presence in the INDEX rather than inline version equality |
| **POLICY 19** (`adr_version_cite_volatile_pin_prohibition`) / **TD-VSDD-091** | Forbids load-bearing ADR version pins specifically in BC traceability rows | Scope extended to ALL load-bearing cross-reference positions across all document types (BC bodies, story ACs, story Tasks, traceability cells, architecture section files); the prohibition now covers every cite form in every live section, not only the BC-traceability-row location |

---

## Migration Plan (phased, post-ratification)

1. **Ratify ADR-045 + policy amendments** (human ratification, POLICY 22 channel).
2. **Spike contextlint and mdbook-lint** for extensibility before building from scratch.
3. **Build Tier-3 validator** (new crate under `crates/hook-plugins/`; comrak or pulldown-cmark;
   initial mode: stale-pin-only detection; section-scoped; fuel-bounded).
4. **Implement Tier-1 INDEX-resolution** (validator queries BC-INDEX / ARCH-INDEX /
   STORY-INDEX / VP-INDEX for canonical current version; no inline version needed in citing doc).
5. **Mechanical corpus migration** (one atomic commit per document class; strip all
   load-bearing inline version pins to stable-anchor form; validator gate provides
   post-migration verification).
6. **Activate Tier-3 enforcement mode** (switch validator from stale-only to
   inline-version-prohibited; BLOCKS any new load-bearing inline version pin).
7. **Implement Tier-2 sidecar schema** (fingerprint/suspect-link gate for explicit
   content-dependent citations; schema design in implementation epic).
8. **Wave-7 residual tail closes** as part of step 5 — all stories re-anchored to stable
   forms in one corpus migration burst; adversary passes clear cleanly.

Suggested anchor: S-15.03 PRIORITY-A (already scoped for validation-layer automation) or a
new wave-9 epic. A dedicated epic is recommended because the migration spans every document
class in `.factory/` and requires coordinated dispatches across state-manager, consistency-
validator, and story-writer roles.

---

## Source / Origin

- **Evidence base:** `.factory/research/wave7-xref-consistency-research.md` (research-agent,
  2026-08-24; four Perplexity deep-research calls, two crates.io registry verifications; all
  sources cited as S1-/S2-/S3-/S4-/V- footnotes).
- **Wave-7 convergence evidence:** D-1069 through D-1080 (STATE.md Decisions Log; full detail
  in `.factory/cycles/v1.0-brownfield-backfill/decision-log.md`).
- **Existing policy context:** `.factory/policies.yaml` POLICY 19 / TD-VSDD-091 (the
  ADR-version-cite prohibition this ADR generalizes).
- **Applicable governance:** POLICY 22 (human ratification required for policy amendments);
  TD-VSDD-053 (single-commit-per-burst; corpus migration bursts must respect this constraint).
- **Standards cited in research:** ISO/IEC/IEEE 29148:2018, OMG ReqIF 1.2, DITA 1.2 keyref,
  IBM DOORS / DOORS Next, Doorstop 3.0.2, OpenFastTrace 4.2.0. [research §§2.1, 4]
- **Rust crate versions (registry-verified):** comrak 0.54.0 (crates.io 2026-07-12);
  pulldown-cmark 0.13.4 (crates.io 2026-05-20). [research V-comrak, V-pulldown]
