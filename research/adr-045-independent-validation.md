# ADR-045 — Independent Adversarial Validation

> **Purpose.** Independent, adversarial external-research validation of the approach proposed in
> ADR-045 (stable-anchor cross-reference architecture), to inform a human RATIFICATION decision.
> This report does **not** parrot the in-repo research (`.factory/research/wave7-xref-consistency-research.md`);
> it re-verifies that report's load-bearing claims against fresh sources and looks specifically for
> weaknesses, overstatements, and cheaper options the ADR may have rejected too fast.
>
> **Author:** research-agent (adversarial validation pass) · **Date:** 2026-08-25 ·
> **Sources accessed:** 2026-08-25 (crates.io registry facts re-verified same day).
> **Scope constraint honored:** no ADR, policy, spec, story, or STATE.md artifact was modified.
> This is a validation report only.

---

## VERDICT — PARTIALLY-SUPPORTS ratification

Independent research **supports the core architectural thesis of ADR-045 but challenges several
of its supporting claims and its build-cost framing.** The load-bearing decision — Tier 1: stop
hand-authoring inline version pins, cite stable anchors, resolve the version mechanically from the
INDEX registries — is the **correct direction** and is well-grounded: stable-identity-plus-resolved-version
is the dominant, sound pattern for mutable artifacts managed as a coherent configuration, and the
project already maintains the INDEX registries that make it cheap. Tier 2 (Doorstop
fingerprint/suspect-link) is a real, verified mechanism whose *kernel* is cleanly portable. Tier 3
(AST validator) is technically feasible; the two named crates and their versions check out against
crates.io. **However**, the ADR (inheriting the in-repo research) (a) **overstates the "industry
consensus"** — it is a dominant pattern, not a universal law, and there are more counterexamples than
the ADR admits; (b) **over-frames OpenFastTrace as a "safety-certification" feature** — that is a
reasonable inference, not OFT's documented rationale, and OFT is **not the only** counterexample;
(c) **glosses a genuine Tier-2 design hazard** — using the `version:` field "as a stable proxy" for a
content fingerprint re-introduces the exact version-coupling the ADR is trying to remove, and the
baseline belongs to the *edge* not the *target*; and (d) **under-weights materially cheaper
alternatives** — a pre-commit normalization codemod (Tier-1-first, thin-Tier-3) and a single
remark-lint rule can get most of the benefit for far less build cost than a bespoke Rust/WASM crate.
None of these is fatal. **Recommendation: ratify the Tier-1 direction, but require (1) the ADR to
soften its consensus/OFT claims to what the evidence supports, (2) a Tier-2 fingerprint-scope
specification that does *not* use the version field as the hash, and (3) a cheap-alternative bake-off
(codemod + Tier-1-first) as an explicit gate before committing to the full three-tier Rust/WASM build.**

---

## Per-claim findings

### Claim 1 — "Separate identity from version is the mature-industry consensus" → **PARTIALLY-SUPPORTED (overstated as stated)**

**What the ADR/research asserts:** ISO/IEC/IEEE 29148, ReqIF, IBM DOORS/DOORS Next, DITA keyref,
Doorstop, Structurizr, log4brains, adr-tools all store a *stable identity* in a cross-reference,
never a *target version*, resolving version from a baseline/manifest. OpenFastTrace is "the single
counterexample."

**Independent finding:** The *direction* is correct, but "industry consensus / single counterexample"
is too strong. Verified point by point:

- **Stable identity is genuinely the dominant, recommended pattern** for evolving requirements
  graphs: ISO 29148 says a requirement ID should not change even when the requirement changes and
  should not be reused after deletion; ReqIF makes `Identifiable::identifier` globally unique and
  **lifetime-immutable**; NASA/INCOSE keep matrices keyed by stable ID. DOORS Next global
  configurations are the strongest real "resolve version from a separate config context" example.
  DITA keyref and Sphinx-Needs `needimport :version:` are genuine late-binding / centralized-binding
  analogues. **This validates the ADR's core move.** [S-29148][S-ReqIF][S-DOORSNext][S-DITA][S-sphinxneeds]
- **But it is not universal, and several stated exemplars are weaker than claimed:**
  - **ISO 29148 does not prescribe any cross-reference serialization at all** — it mandates unique
    IDs, revision info, CM, and baselines, but explicitly leaves representation open. Citing it as
    prescribing "stable identity in the reference" is an over-read. [S-29148]
  - **ReqIF standardizes immutable identity but has no standard baseline/version-resolution
    mechanism** — the snapshot is whatever the exporting tool chooses. It supports the pattern; it
    doesn't mandate a resolver. [S-ReqIF]
  - **Several docs-as-code tools (adr-tools, log4brains, StrictDoc, Structurizr) don't have a
    "separate version-resolution layer" at all** — they simply resolve IDs against the *checked-out
    repository*. That is a whole-project snapshot, not the sophisticated ID→version resolver the ADR
    implies is industry practice. Useful, but architecturally different from DOORS Next. [S-adrtools][S-log4brains][S-strictdoc][S-structurizr]
- **Counterexamples beyond OpenFastTrace exist** (the ADR's "single counterexample" claim is wrong):
  - **Classic IBM DOORS supports genuinely versioned links** whose endpoint carries the target
    module version/baseline (DXL `targetVersion(Link)`). A mature enterprise tool deliberately
    embeds version in the link where lifecycle semantics require it. [S-DOORSclassic]
  - **Versioned API references** deliberately embed version inline (Kubernetes `apiVersion: batch/v1`,
    `/apis/batch/v1`; OpenAPI `$ref` permits version-bearing URIs). [S-k8s][S-openapi]
  - **Version-specific DOIs** (Zenodo concept-DOI vs version-DOI; DataCite `HasVersion`/`IsVersionOf`;
    Crossref recommends distinct DOIs for materially-updated versions) — the citation denotes one
    exact version. [S-zenodo][S-datacite][S-crossref]
  - **Exact-edition legal citations** (`17 U.S.C. § 107 (2012)`, point-in-time legislation.gov.uk
    URLs) and **edition-pinned normative-standard references** (`ISO/IEC/IEEE 29148:2018`) travel
    with the version inline. [S-legal]
  - **Oracle Innovation Management and Squash TM** retain version-specific requirement linkages. [S-oracle][S-squash]

**Adversarial read:** The best-defensible formulation is *"for mutable artifacts managed as a
coherent configuration, mature requirements platforms commonly store a stable identity and resolve
the applicable state from a baseline/config/checkout"* — **which still fully justifies ADR-045's
Tier 1.** The ADR should not lean on "industry consensus … never a version," because that broader
claim is contradicted by classic DOORS, versioned APIs, version DOIs, and edition-pinned citations.
The recommendation survives; the rhetoric should be softened.

---

### Claim 2 — OpenFastTrace embeds revision to force re-review; this project doesn't need it → **PARTIALLY-SUPPORTED**

**Confirmed:**
- OFT's specification-item ID is **artifact-type + name + revision**, serialized `type~name~revision`
  (e.g. `req~html5-exporter~1`); all three parts are integral to the ID. [S-OFT-guide][S-OFT-design]
- Incrementing the revision **deliberately obsoletes/voids existing coverage links** ("voids all
  existing links"); old `impl->req~name~1` no longer covers `req~name~2`, which OFT reports as
  Outdated/defective, failing clean coverage. The design intent is exactly to force covering-item
  authors to check and adapt. [S-OFT-guide][S-OFT-design]
- **Trivial/editorial edits are explicitly not supposed to bump the revision** (add a missing period
  → no bump; change the supported-browser list → bump). This confirms the ADR's reading that the
  churn is a deliberate cost of the scheme. [S-OFT-guide]

**Challenged / overstated:**
- **OFT's own docs do NOT frame the revision mechanism as a "safety-certification forced-re-review"
  feature.** OFT calls tracing a general "safety net for non-trivial software projects" and says
  producing audit proof is explicitly "not a main goal." The "safety-certification" characterization
  is a *reasonable inference* (and OFT *is* used that way — Xen functional safety uses
  `req_type~name~revision` and bumps on modification), but presenting it as OFT's design intent is an
  over-attribution. [S-OFT-guide][S-OFT-sysreq][S-xen]
- **OFT does not prove human review occurred** — it forces attention by breaking the trace, not by
  recording a review. Minor point, but the ADR's language ("forces manual re-inspection") slightly
  overclaims.
- **OFT is not unique** (see Claim 1). Oracle IM, Squash TM version-specific links, and classic DOORS
  versioned links are also revision/version-embedding schemes. The ADR's "single counterexample —
  OpenFastTrace" is inaccurate.

**Does this project benefit from the forced-re-review model?** Independent judgment: **No, correctly.**
This is a Markdown spec corpus for a dev-tooling engine, not a DO-178C/ISO 26262/IEC 61508 certified
artifact set. The evidence confirms the forced-re-anchoring cost is real and deliberate in OFT; the
project incurs that cost (Wave-7 passes 4–9) without the certification benefit. The ADR's conclusion
holds even though its OFT framing is imprecise. **The argument would be *stronger*, not weaker, if it
dropped the "safety-certification" gloss** and simply said: "revision-in-identity is a legitimate
design whose known, intended cost is re-anchoring churn; we don't want that cost."

---

### Claim 3 — Doorstop fingerprint/suspect-link is real and portable to a custom INDEX+WASM design → **SUPPORTED (mechanism); PARTIALLY-SUPPORTED (portability, with a real hazard the ADR glosses)**

**Mechanism confirmed against Doorstop docs:**
- A child item's `links` entry stores `parent-UID: <fingerprint-at-last-review>`; a fresh/unacknowledged
  link may be `- REQ001: null` (or omit the hash) — so "always stores both immediately" is too strong.
  [S-doorstop-item]
- On `doorstop` validation, if the parent's current fingerprint ≠ the stored one, the link is reported
  **suspect** (`WARNING: … suspect link: REQ001`). It's a derived validation state, not a persisted
  `suspect:true` flag. [S-doorstop-validation]
- **`doorstop clear`** re-stamps the stored parent-link fingerprints (clears suspect). **`doorstop
  review`** updates an item's *own* `reviewed` fingerprint — a *different* stamp. Migrations run
  `review all` **then** `clear all`. The ADR conflates these into one "clear" concept; a faithful
  implementation must model **two distinct state transitions** (self-review vs dependency-acknowledgement).
  [S-doorstop-validation][S-doorstop-changelog]

**Portability — the kernel is cleanly extractable** (supports the ADR): the portable abstraction is a
*versioned, per-dependency acceptance stamp* — `{dependent_id, dependency_id, accepted_fingerprint,
fingerprint_policy_version}`; `validate` recomputes and compares; `acknowledge` re-stamps. Doorstop's
item YAML, document-tree hierarchy, folder layout, publishing model, and exact CLI are **incidental**
and need not be adopted. This maps onto the project's INDEX registries + WASM hook cleanly. [S-doorstop-item]

**Hazards the ADR under-states (this is the adversarial core of Claim 3):**
1. **The ADR proposes using "the target's current content hash (or its existing `version:` field,
   used as a stable proxy)" as the fingerprint.** Using the **version field as the fingerprint
   re-introduces exactly the version-coupling Tier 1 removes** — a version bump would flip every
   dependent to suspect, which is the churn again by another name. Doorstop deliberately hashes
   *semantic content* (UID + text + ref + link-UIDs), **not** a human version number. Tier 2 should
   hash content, not version, or it defeats Tier 1. **This is a concrete design defect to fix before
   ratification.**
2. **Fingerprint scope is policy, not crypto.** The design must specify which fields are normative,
   canonical ordering, newline/Unicode normalization, and a persisted `hash_version`. Hashing raw
   bytes yields false suspects (formatting) or missed changes. The ADR defers the sidecar schema
   entirely ("TBD in the implementation epic") — acceptable, but the *fingerprint-scope policy* is a
   ratification-level decision, not a mere schema detail.
3. **The baseline belongs to the edge, not the target.** Two dependents may have acknowledged the
   same target at different times; a single global "last-reviewed target hash" cannot represent both.
   The store must be edge-local.
4. **Validation must be read-only (never auto-clear).** If a lint run re-stamps on mismatch, every
   change self-approves and the safeguard is void. This aligns with D-449(a) but must be stated.
5. **`null` has workflow meaning** (unreviewed vs suspect vs error) and must be defined, not silently
   treated as accepted.

**Verdict:** the mechanism is real and its kernel is portable (supports the ADR), but the ADR's
"version field as a stable proxy" shortcut is a genuine smell that would partially re-create the
disease; portability is SUPPORTED only with a corrected, content-based fingerprint spec.

---

### Claim 4 — AST tooling (comrak / pulldown-cmark) versions and capabilities → **SUPPORTED (versions + nodes); PARTIALLY-SUPPORTED (fuel rationale is inferred, one claim is outdated)**

**Registry verification (crates.io, re-checked 2026-08-25 — independent of the ADR's numbers):**
- **comrak — latest & max_stable `0.54.0`, published 2026-07-12.** Matches the ADR exactly. (Recent:
  0.54.0 2026-07-12, 0.53.0 2026-07-02, 0.52.0 2026-04-04; ~7.1M total downloads.) [V-comrak]
- **pulldown-cmark — latest & max_stable `0.13.4`, published 2026-05-20.** Matches the ADR exactly.
  (Recent: 0.13.4 2026-05-20, 0.13.3 2026-03-22, 0.13.2 2026-03-21; ~139.8M total downloads.) [V-pulldown]

  Both version pins in ADR-045 are **CONFIRMED accurate** against the live registry.

**comrak node-type claims — CONFIRMED (with one caveat):**
- Distinct `NodeValue::SoftBreak` (soft) and `NodeValue::LineBreak` (hard) inline nodes: **confirmed.** [S-comrak-nodes]
- Opaque `FrontMatter(String)` and `Table`/`TableRow(bool)`/`TableCell` block nodes: **confirmed**
  (front matter + tables require enabling the extensions). [S-comrak-nodes][S-comrak-ext]
- `sourcepos` on every node: **structurally yes** (every `Ast` carries a `sourcepos`), **but accuracy
  has documented exceptions** — manually built nodes can be `(0,0)`, and reliability caveats exist for
  lists/list-items and some extensions (esp. description lists). The validator must not assume every
  position is exact. [S-comrak-nodes][S-comrak-render]

**pulldown-cmark streaming claim — CONFIRMED but the fuel rationale is OVERSTATED:**
- Pull-parser `Event` iterator, distinct `Event::SoftBreak`/`Event::HardBreak`, balanced `Start`/`End`,
  `into_offset_iter()` → `(Event, Range<usize>)` byte offsets, no public owned arena AST: **all
  confirmed.** [S-pulldown-docs]
- **But "streaming ⇒ clearly more fuel-conservative" is inferred, not proven.** pulldown-cmark still
  runs a first pass and retains an internal `Tree<Item>` plus reference-definition maps; it is
  "streaming" at the *consumer API*, not in the strong O(nesting-depth) sense. WASM fuel meters
  executed operations, not "AST bytes," so comrak's arena is only part of the cost — block
  recognition, delimiter/reference processing, Unicode, and the linter's own logic may dominate. A
  hierarchy-reconstruction stack on top of pulldown-cmark also costs compute. **No crate ships a WASM
  fuel benchmark for 3000-line docs.** The expected pulldown advantage is well-motivated but must be
  **empirically benchmarked in the actual wasmtime runtime**, not assumed. [S-fuel-synthesis]
- **One ADR-adjacent belief is outdated:** "comrak can't compile to WASM because of syntect" is no
  longer true — comrak added WASM support in 0.13.1 with target-specific syntect handling; both crates
  build for `wasm32-unknown-unknown`. The correct guidance is `default-features = false` for a small
  sandbox module (drops CLI/syntect weight). The ADR doesn't repeat the stale claim outright, but any
  implementation should note comrak-on-WASM is viable. [S-comrak-wasm][S-pulldown-wasm]

**tree-sitter-markdown warning — CONFIRMED:** the grammar's README explicitly warns it is not
recommended where correctness matters (highlighting-oriented, two-pass CST, soft-break node naming
under-documented). Avoiding it for a correctness-critical gate is sound. [S-treesitter]

**Verdict:** versions and node capabilities are accurate; the fuel-based preference for pulldown-cmark
should be reframed as "benchmark both, expect pulldown to be lighter for a genuinely single-pass rule"
rather than presented as settled. Given comrak's node types map 1:1 onto the four failure modes,
comrak-with-`default-features=false` is a perfectly defensible primary choice.

---

### Claim 5 — Spike contextlint / mdbook-lint; build custom if neither fits → **PARTIALLY-SUPPORTED (spike is right; a cheaper tool was under-weighted)**

**Tool existence and fit — verified:**
- **contextlint exists** (`@contextlint/cli` / `@contextlint/core`), and is **built on the
  remark/unified stack** (`remark-parse`, `remark-gfm`, `unified`, `unist-util-visit`) — so its
  underlying AST is **mdast**, resolving the in-repo research's "inconclusive AST library" flag.
  **REF-002** does bidirectional ID defined-vs-referenced checking (via `idColumn`/`idPattern`);
  **GRP-001** validates a table-column traceability chain (requirements→design→test). **Neither
  captures a prose ID and a separate version token, associates them across wrapping/anchors, nor does
  an ID→current-version registry lookup.** It is the closest semantic prior art but insufficient as
  configured; no public user-loadable custom-rule API is documented (unverifiable whether one exists).
  [S-contextlint-rules][S-contextlint-core]
- **mdbook-lint exists** with **17 ADR rules (ADR001–ADR017)** for Nygard/MADR *document* structure,
  built over **comrak**. It has **no** "all live refs to ADR-X agree on version Y" rule, and **no
  documented drop-in custom-rule/plugin loader** — extending it means compiling a custom binary or
  contributing upstream (i.e. effectively building a validator). [S-mdbooklint-rules][S-mdbooklint-core]

So the ADR's spike recommendation is sound and will very likely conclude "neither is drop-in
configurable; build custom." **But the spike framing skips the cheaper build target:**

- **markdownlint-cli2 custom rule** (micromark tokens): can do all five semantics, async/network-capable,
  frontmatter supplied separately — but micromark is lower-level than mdast (more bookkeeping). [S-markdownlint]
- **A single remark-lint custom rule over mdast** can do **all five** required semantics (independent
  ID/version detection; paragraph-flattening across wraps and link/anchor nodes; registry lookup;
  heading-section scoping; frontmatter/changelog exclusion) as **one small JS/TS plugin** — no new
  parser, no WASM component, no framework. This is the research's explicit "materially simpler"
  recommendation and it is well-founded on remark's mature AST + utilities. [S-remark-lint]
- **Vale** (sequence/script rules) and **rumdl** are weaker fits (Vale sentence-scoped association is
  limited; rumdl has no runtime plugin API). [S-vale][S-rumdl]

**The important adversarial nuance (and a partial defense of the ADR):** "remark-lint is simpler" is
true *in the abstract*, but this project's hook chain is **entirely Rust→WASM**. Adopting remark
introduces a **new Node/JS toolchain and runtime** into a Rust-WASM-native project — a real,
recurring maintenance and CI cost the JS-centric research did not price in. So the ADR's choice of a
Rust/WASM crate is **defensible on integration grounds**, even though remark is cheaper to write. The
gap in the ADR is that it doesn't *make this argument* — it should explicitly justify Rust/WASM over
remark-lint on "single-toolchain / existing-hook-infrastructure" grounds, rather than jumping to
"build custom" as if remark weren't an option.

---

### Claim 6 — Is a simpler/better option rejected too fast? → **PARTIALLY-SUPPORTED (two cheaper paths were under-weighted)**

The four formally-rejected alternatives (detector-only; wider multiline regex; pure-architectural-no-detector;
Sphinx custom domains) are each rejected for sound reasons — detector-only treats the symptom, regex
can't close all four failure modes, no-detector leaves migration ungated, and Sphinx requires a full
reST corpus migration (disproportionate). Those rejections hold up.

**But two materially cheaper options were not given fair evaluation:**

1. **Pre-commit normalization codemod (Tier-1-first, thin-Tier-3).** A one-shot codemod that strips
   `\b(ADR-\d+|BC-\d+(?:\.\d+)+|S-\d+\.\d+|VP-\d+)\b .{0,N}? \bv\d+(?:\.\d+)*\b` in live sections,
   plus a lightweight pre-commit guard that rejects new load-bearing pins, gets **most of Tier 1's
   benefit** (kills the churn class) at a fraction of the build cost of a full AST validator — and can
   ship *before* the heavy Tier-3 crate. The research explicitly recommends this "eliminate mutable
   versions from prose" path as the preferred solution when historical pinning isn't required. The
   ADR treats Tier 3 as a co-required unit ("all three tiers must be ratified as a unit"); a
   **phased** ratification (Tier 1 + codemod now; Tier 3 AST validator only if residual leakage is
   measured) is a legitimate cheaper trajectory the ADR dismisses by construction. [S-cheaper-synthesis]
2. **A single remark-lint rule instead of a bespoke Rust/WASM crate** (see Claim 5) — cheaper to write,
   though it carries the cross-toolchain cost noted above.

**Adversarial conclusion for Claim 6:** the three-tier hybrid is a *reasonable, defensible* design,
but the ADR's insistence that all three tiers ratify "as a unit" is stronger than the evidence
warrants. The correct-by-construction win lives almost entirely in **Tier 1**; Tiers 2 and 3 are
guards whose full cost should be gated on measured need. A cheaper, phased path (Tier 1 + codemod +
thin new-pin guard first; escalate to AST validator / fingerprint sidecar only if data shows residual
leakage) delivers most of the value sooner and should be evaluated head-to-head before the full build
is authorized.

---

## Risks & cheaper alternatives (summary)

| # | Risk / cheaper option | Severity | Recommendation |
|---|----------------------|----------|----------------|
| R1 | **Tier-2 "version field as fingerprint proxy" re-introduces version coupling** (partially undoing Tier 1). | HIGH | Specify a **content-based** fingerprint (Doorstop hashes semantic content, not version). Fix in the ADR before ratification. |
| R2 | **"Industry consensus / single counterexample" is overstated** (classic DOORS versioned links, versioned APIs, version DOIs, edition-pinned citations contradict it). | MED | Soften ADR rhetoric to "dominant pattern for configuration-managed mutable artifacts." Recommendation still holds. |
| R3 | **OFT "safety-certification feature" framing is an inference, not OFT's documented rationale**; OFT is not unique. | LOW | Reframe as "revision-in-identity's intended cost is re-anchoring churn; we don't want it." Argument gets stronger. |
| R4 | **Fuel preference for pulldown-cmark is inferred, not benchmarked**; comrak-on-WASM is viable with `default-features=false`. | MED | Require an actual wasmtime fuel bake-off before choosing; comrak is a defensible primary given node-type fit. |
| R5 | **Three-tier "ratify as a unit" over-couples the cheap win (Tier 1) to expensive guards.** | MED | Phase ratification: Tier 1 + pre-commit codemod + new-pin guard first; escalate to Tier 3 AST validator / Tier 2 sidecar on measured residual leakage. |
| R6 | **remark-lint single-rule alternative under-weighted** (cheaper to write; but adds a JS toolchain to a Rust-WASM project). | LOW | ADR should explicitly justify Rust/WASM over remark on single-toolchain grounds, not skip it. |
| R7 | **`sourcepos` accuracy exceptions** (lists/list-items, some extensions) and **review-vs-clear are two distinct Doorstop transitions**. | LOW | Note in implementation epic; model both state transitions; don't assume exact positions everywhere. |

**Net:** none of R1–R7 is fatal. R1 is the one that should block ratification *of Tier 2 as written*
until corrected. R2–R6 are refinements that make the ADR more accurate and cheaper to execute.

---

## Sources

All web sources accessed 2026-08-25. Registry facts independently re-verified against crates.io the
same day. Sources grouped by claim.

**Registry verification (crates.io API, accessed 2026-08-25):**
- [V-comrak] crates.io/api/v1/crates/comrak → latest & max_stable **0.54.0**, published **2026-07-12** (~7.11M downloads); 0.53.0 (2026-07-02), 0.52.0 (2026-04-04).
- [V-pulldown] crates.io/api/v1/crates/pulldown-cmark → latest & max_stable **0.13.4**, published **2026-05-20** (~139.8M downloads); 0.13.3 (2026-03-22), 0.13.2 (2026-03-21).

**Claim 1 — identity vs version across standards/tools:**
- [S-29148] ISO/IEC/IEEE 29148:2018 — iso.org/standard/72089.html ; standards.ieee.org/ieee/29148/6937/ (unique persistent IDs; no prescribed cross-ref serialization; baselines/CM).
- [S-ReqIF] OMG ReqIF — omg.org/spec/ReqIF/1.2 ; omg.org/reqif/ (lifetime-immutable `identifier`; `lastChange`; no standard baseline resolver).
- [S-DOORSNext] IBM DOORS Next / ELM global configuration — ibm.com/docs/en/engineering-lifecycle-management-suite/lifecycle-management/7.2.0?topic=configurations-cross-project-links.
- [S-DOORSclassic] IBM DOORS classic versioned links / `targetVersion(Link)` / baseline sets — ibm.com/docs/.../doors/9.7.2?topic=requirements-baselines ; stackoverflow.com/questions/15094959 (DXL versioned link to specific baseline).
- [S-DITA] DITA keys/keyref — docs.oasis-open.org/dita/dita/v1.3/os/part1-base/archSpec/base/keys-core-concepts.html.
- [S-adrtools] adr-tools — github.com/npryce/adr-tools (filename/number link; supersession, no version).
- [S-log4brains] log4brains — thomvaill.github.io/log4brains/adr/ ; github.com/thomvaill/log4brains (date-slug ID; immutable+supersede).
- [S-structurizr] Structurizr DSL identifiers — docs.structurizr.com/dsl/identifiers ; docs.structurizr.com/onpremises/workspace-versioning.
- [S-strictdoc] StrictDoc — strictdoc.readthedocs.io user guide (UID + RELATIONS; document VERSION separate from links).
- [S-sphinxneeds] Sphinx-Needs — sphinx-needs.readthedocs.io/en/latest/directives/needimport.html ; roles.html (need ID in links; version pinned once at import).
- [S-k8s] Kubernetes API versioning — kubernetes.io/docs/reference/using-api/ (group/version in paths + `apiVersion`).
- [S-openapi] OpenAPI referencing — spec.openapis.org/oas/v3.1.2.html ; learn.openapis.org/referencing/overview.html.
- [S-zenodo] Zenodo DOI versioning — zenodo.org/help/versioning ; [S-datacite] support.datacite.org/docs/versioning ; [S-crossref] crossref.org/documentation/principles-practices/best-practices/versioning/.
- [S-legal] U.S./UK legal citation guides — guides.ll.georgetown.edu ; legislation.gov.uk/help (edition-year / point-in-time).
- [S-oracle] Oracle Innovation Management — docs.oracle.com/en/cloud/saas/supply-chain-and-manufacturing/25b/fauim/ ; [S-squash] Squash TM requirement versions — tm-en.doc.squashtest.com/v8/user-guide/manage-requirements/make-requirement-versions.html.

**Claim 2 — OpenFastTrace:**
- [S-OFT-guide] OFT user guide — github.com/itsallcode/openfasttrace/blob/main/doc/user_guide.md (ID = type+name+revision; revision voids coverage; trivial edits don't bump; "safety net", not "certification").
- [S-OFT-design] OFT design spec — github.com/itsallcode/openfasttrace/blob/main/doc/spec/design.md (grammar `type "~" id "~" revision`).
- [S-OFT-sysreq] OFT system requirements — github.com/itsallcode/openfasttrace/blob/main/doc/spec/system_requirements.md ("audit proof … not a main goal").
- [S-xen] Xen functional-safety requirements (OFT in a safety context) — xenbits.xen.org/docs/unstable/fusa/reqs/intro.html.

**Claim 3 — Doorstop:**
- [S-doorstop-item] Doorstop item reference — github.com/doorstop-dev/doorstop/blob/develop/docs/reference/item.md ; doorstop.readthedocs.io/en/latest/reference/item.html (links store parent UID + fingerprint; `- REQ001: null`; SHA-256/URL-safe-base64 of UID+text+ref+link-UIDs).
- [S-doorstop-validation] Doorstop validation CLI — doorstop.readthedocs.io/en/latest/cli/validation.html (suspect-link warning; `doorstop clear`).
- [S-doorstop-changelog] Doorstop changelog — doorstop.readthedocs.io/en/v2.1/about/changelog/ (`Item.clear()` saves linked hashes vs `Item.review()` saves own hash; `review all` then `clear all`).

**Claim 4 — comrak / pulldown-cmark / tree-sitter:**
- [S-comrak-nodes] comrak NodeValue — docs.rs/comrak/latest/comrak/nodes/enum.NodeValue.html (SoftBreak, LineBreak, FrontMatter(String), Table/TableRow(bool)/TableCell; `Ast.sourcepos`).
- [S-comrak-ext] comrak Extension options — docs.rs/comrak/latest/comrak/options/struct.Extension.html.
- [S-comrak-render] comrak Render options / sourcepos caveats — docs.rs/comrak/latest/comrak/options/struct.Render.html ; docs.rs/comrak/latest/comrak/nodes/index.html.
- [S-comrak-wasm] comrak WASM support (added 0.13.1; default-features guidance) — github.com/kivikakk/comrak (Cargo.toml/changelog) ; github.com/benwis/comrak-wasm.
- [S-pulldown-docs] pulldown-cmark — docs.rs/pulldown-cmark/latest/pulldown_cmark/ (Event, SoftBreak/HardBreak, Start/End, `into_offset_iter`) ; struct.Parser.html ; struct.OffsetIter.html.
- [S-pulldown-wasm] pulldown-cmark wasm32 CI — github.com/pulldown-cmark/pulldown-cmark/pull/1006.
- [S-fuel-synthesis] Fuel/memory analysis synthesis — pulldown-cmark guide (pulldown-cmark.github.io/pulldown-cmark/) + internal `Tree<Item>` (docs.rs/pulldown-cmark/latest/src/…/lib.rs.html); no published WASM fuel benchmark for 3000-line docs.
- [S-treesitter] tree-sitter-markdown README correctness warning — github.com/tree-sitter-grammars/tree-sitter-markdown.

**Claim 5 / 6 — existing tools & cheaper alternatives:**
- [S-contextlint-rules] contextlint rules — contextlint.dev/docs/rules/ ; /docs/rules/grp-001/ (REF-002, GRP-001).
- [S-contextlint-core] contextlint on remark/unified — classic.yarnpkg.com/en/package/@context-lint/core ; contextlint.dev/docs/get-started/.
- [S-mdbooklint-rules] mdbook-lint ADR rules — joshrotenberg.com/mdbook-lint/rules/index.html ; /rules/adr/.
- [S-mdbooklint-core] mdbook-lint architecture (comrak; PluginRegistry, no CLI plugin loader) — joshrotenberg.com/mdbook-lint/architecture.html ; docs.rs/mdbook-lint-core.
- [S-markdownlint] markdownlint custom rules (micromark tokens) — github.com/DavidAnson/markdownlint/blob/main/doc/CustomRules.md.
- [S-remark-lint] remark-lint custom rule over mdast — github.com/remarkjs/remark-lint/blob/main/doc/create-a-custom-rule.md ; github.com/remarkjs/remark-frontmatter.
- [S-vale] Vale scopes/sequence/script — vale.sh/docs/scopes ; docs.vale.sh/checks/sequence ; docs.vale.sh/checks/script.
- [S-rumdl] rumdl (no runtime plugin API) — rumdl.dev/comparison/ ; github.com/rvben/rumdl.
- [S-cheaper-synthesis] "eliminate mutable versions from prose / pre-commit codemod / single remark-lint rule" — synthesis over contextlint pre-commit docs (contextlint.dev/docs/integrations/ci-cd/pre-commit/) + remark-lint custom-rule docs.

---

## Research Methods

| Tool | Queries | Purpose |
|------|---------|---------|
| **Perplexity perplexity_research (PRIMARY)** | 4 | (1) Industry identity-vs-version consensus across ISO 29148/ReqIF/DOORS/DITA/Doorstop/etc. + counterexamples; (2) OpenFastTrace revision-in-identity design intent + uniqueness; (3) Doorstop fingerprint/suspect mechanism + portability/hidden-coupling; (4) contextlint/mdbook-lint/markdownlint/remark/vale/rumdl fit + materially-simpler alternatives. Each a deep multi-source synthesis with citations. |
| Perplexity perplexity_reason | 0 | — |
| Perplexity perplexity_search | 0 | — |
| Perplexity perplexity_ask | 0 | — |
| Context7 | 0 | Not used; crates.io registry facts verified via WebFetch (authoritative for version/publish-date) and comrak/pulldown-cmark node APIs cross-checked inside the perplexity_research call citing docs.rs. |
| Tavily (any) | 0 | — |
| WebFetch | 2 | Independent crates.io API verification of comrak (0.54.0, 2026-07-12) and pulldown-cmark (0.13.4, 2026-05-20) — did NOT trust the ADR's numbers; both confirmed. |
| WebSearch | 0 | — |
| Read | 2 | ADR-045 + the in-repo wave7 research report (the artifacts under validation). |
| Training data | 0 areas | No claim rests on training data alone; every standard, tool, mechanism, and version is web- or registry-cited. Model knowledge used only to structure/interpret and to construct adversarial counter-hypotheses. |

**Total MCP tool calls:** 4 (`perplexity_research` ×4). **Plus** 2 WebFetch registry verifications.
**Training data reliance:** low — all load-bearing claims are independently web/registry-sourced; the
crate version pins were re-verified against the live crates.io API rather than copied from the ADR.

**Inconclusive / flagged:**
- Whether contextlint has an unpublished user-loadable custom-rule API is **unverifiable** from public
  docs (documented config exposes built-in rules + options only).
- The pulldown-cmark-vs-comrak **fuel** advantage is **inferred, not benchmarked** — no crate ships a
  WASM fuel benchmark for 3000-line documents; requires an in-runtime bake-off.
- comrak `sourcepos` accuracy exceptions (lists/list-items, some extensions) are documented but not
  quantified for this corpus.
