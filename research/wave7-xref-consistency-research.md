# Wave 7 — Cross-Reference / Version-Pin Consistency: External Research

> **Path note.** This report was requested at `.factory/planning/wave7-xref-consistency-research.md`,
> but that path is not registered in `plugins/vsdd-factory/config/artifact-path-registry.yaml`, so the
> `validate-artifact-path` governance hook blocks writes there (`ARTIFACT_PATH_UNREGISTERED`). The
> registry's canonical, block-allowed home for a research document with a custom filename is
> `research-doc → .factory/research/{filename}.md`, so the report lives here. No hook was bypassed.
>
> **Purpose.** Inform a terminal decision on how to close out the stubborn, self-regenerating
> version-pin propagation churn in the interlinked Markdown spec corpus (BC-x.xx.xxx, ADR-NNN,
> S-NN.MM, VP, index files). This report answers whether the correct fix is to **build a better
> detector**, to **eliminate load-bearing version pins architecturally**, or to adopt a **hybrid** —
> grounded in cited prior art and verified tool maturity.
>
> **Scope constraint honored.** No spec, story, STATE.md, or pipeline artifact was modified.
> This is a research report only. No other agents were dispatched.
>
> **Author:** research-agent · **Date:** 2026-08-24 · **Accessed dates:** all web sources
> accessed 2026-08-24 unless otherwise noted. Perplexity deep-research snapshots internally
> dated themselves 2026-08-25 (model clock); registry facts were independently re-verified
> against crates.io on 2026-08-24.

---

## 0. Executive summary (the terminal recommendation)

**Adopt a HYBRID, weighted decisively toward architectural elimination of load-bearing version
pins.** The observed failure mode — every remediation bumps a version, which forces a cohort-wide
re-anchor, a sweep misses a straggler, the next review finds it — is **not a detector-quality
problem. It is a design smell that mature requirements-engineering and docs-as-code tooling
deliberately designed away decades ago.** The single most important finding of this research:

> **Industry consensus (ISO/IEC/IEEE 29148, ReqIF, IBM DOORS/DOORS Next, DITA keyref, Doorstop,
> Structurizr, log4brains, adr-tools) is that a cross-reference stores a STABLE IDENTITY, never a
> target VERSION. The applicable version is resolved from a separately controlled baseline /
> manifest / configuration, or is derived and rendered mechanically. Putting the version inside
> every citing document is the known anti-pattern that produces exactly the re-anchoring churn we
> are suffering.** [S2-ReqIF][S2-29148][S2-DITA][S4-DOORS][S4-Doorstop]

The one tool that *does* embed revision in the reference — **OpenFastTrace** (`req~name~1`) — does
so on purpose so that a semantic revision **voids all coverage links by design**, forcing manual
re-inspection. That is a *feature* for safety-certification re-review, and it is **a precise
description of our current pain**. OpenFastTrace's existence validates that our present design is a
recognized pattern whose recognized cost is deliberate re-anchoring churn. [S4-OFT] If we do not
want certification-grade forced re-review, we should not be using the certification-grade
forced-re-review reference scheme.

The three legs of the recommended hybrid:

1. **(Primary) Make stale pins impossible by construction.** Author references as stable anchors
   (`ADR-039 §Decision 3`, `BC-1.03.017`), never with an inline `vN.NN`. Resolve/verify the version
   mechanically at lint/validate time against the registries the project already maintains
   (`BC-INDEX.md`, `VP-INDEX.md`, `STORY-INDEX.md`, `ARCH-INDEX.md`). This eliminates cohort-wide
   re-anchoring *entirely* — there is nothing to re-anchor. [S2-29148][S2-DITA][S2-antora]

2. **(Secondary, where provenance is genuinely load-bearing)** Keep a version only where point-in-time
   provenance is part of the meaning (e.g. decision-log D-NNN prose that discusses a *specific* prior
   revision, changelog rows). Mark those sections explicitly historical/exempt so they are excluded
   from live-consistency scanning — matching the Doorstop/DOORS separation of "identity" from
   "baseline snapshot." [S2-DOORS][S4-Doorstop]

3. **(Residual detector)** For any pins that legitimately remain live, replace the regex/manual sweep
   with an **AST-based, section-scoped validator** that adopts **Doorstop's fingerprint/suspect-link
   model**: store the target's current fingerprint/version in a machine-readable sidecar, and have a
   WASM lint hook automatically flag every citing document as "suspect" when the target changes,
   cleared only by an explicit reviewed edit. This is mechanically enforceable and defeats the
   "unreliable human/agent self-attestation" failure directly. Build it on the **comrak** or
   **pulldown-cmark** Rust crate (both WASM-compatible, both used-in-anger, versions verified below).
   [S3-comrak][S3-pulldown][S4-Doorstop]

**Why not "just build a better detector"?** Because a better detector still leaves the churn intact:
you would detect the straggler faster, but every version bump would still force a cohort-wide edit,
and the 3-CLEAN convergence loop would still be perturbed on every bump. A detector treats the
symptom; the architectural change removes the disease. The detector is necessary but insufficient,
which is why it is leg 3, not leg 1.

**Why not "pure architectural, no detector"?** Because the corpus already contains thousands of
existing pins, and some provenance citations are legitimately version-bearing. A detector is needed
during migration and to guard the residual set. Hence hybrid, not pure.

---

## 1. Q1 — Docs-as-code cross-reference / link-integrity tooling

**Verdict on Q1: No off-the-shelf tool natively enforces "citation `vN` must equal target's current
version" for a custom `BC-`/`ADR-` identifier scheme.** Every tool either (a) checks only URLs/paths/
heading anchors, or (b) provides an *extension point* (custom rule / domain / plugin / preprocessor)
on which such a check must be built. The good news: the AST-parsing tools give a robust substrate;
the regex-scoped ones inherit our exact line-wrap fragility.

| Tool | Approach: AST vs regex; robust to line-wrap? | Semantic custom-ID xref? | Can enforce version-consistency? | Maturity / last release | Source |
|------|----------------------------------------------|--------------------------|----------------------------------|--------------------------|--------|
| **Vale** | Hybrid: real markup parsers (Goldmark/Asciidoctor/Docutils) normalize prose into scopes, then **rules are mostly regex over scoped prose**. Line-wrap inside one paragraph is safe; but a citation is not an AST node, so a wrapping rule still needs `\s+`-tolerant regex. `scope:raw` is source-fragile. | No native cross-file resolver/registry. Can flag ID shape/deprecation via `existence`/`substitution`/`consistency`/script rules; semantic resolution needs a generated style. | Only via generated rules from an external registry, or a script rule + CI. Not built-in. | **v3.14.2, 2026-05-15**; ~5.4–5.6k stars, 189 releases; active. | [S1-vale] |
| **markdownlint + markdownlint-cli2 + custom rules** | Custom rules can consume **micromark tokens** (structural, wrap-safe) or `parser:"none"` raw text (regex-fragile). Prefer token-based. | Not built-in; a JS custom rule can index a corpus-wide registry and validate IDs, but rules run per-file so you build/cache the global index yourself. | Yes, via a custom JS rule that loads target metadata and compares ID + version. Not via config alone. | **cli2 v0.23.2, 2026-07-27** (lib markdownlint 0.41.1, 2026-07); ~6.1k / ~0.9k stars; active. | [S1-mdl] |
| **remark + remark-lint + remark-validate-links** | **Full mdast AST** (unist positions). Robust to legal line-wrap and multiline link labels. Best JS AST substrate. | `remark-validate-links` checks local files + same-file/other-file headings only; deliberately not arbitrary `BC`/`ADR` IDs. A custom unified plugin can do true cross-file semantic resolution. | Yes — one of the strongest extension foundations: a transformer parses citation+version nodes, loads a repo map, compares target frontmatter, emits vfile diagnostics. | remark 15.0.1 / remark-cli 12.0.1 (2024-04-30); remark-lint 10.x (releases into 2025); **remark-validate-links 13.1.0, 2025-02-21 — repo reported archived 2026-06-04** (mature but no longer actively developed). | [S1-remark][S3-rvl] |
| **mdBook core + mdbook-linkcheck** | pulldown-cmark event stream (parser-based, wrap-safe), not a persistent AST. linkcheck validates Link/Image events, paths, HTTP, fragments. | No general semantic validator; `ADR-039` participates only as a link/path/fragment. Custom preprocessor/backend needed. | Only via a custom preprocessor/backend. | mdBook very active (~22k stars; 0.5.x, 0.5.3 dated 2026-05-19). **mdbook-linkcheck effectively dormant: v0.7.7, 2022-10-03**, ~159 stars. | [S1-mdbook] |
| **Sphinx (`:ref:`, nitpicky `-n`, `-W`, linkcheck, intersphinx)** | **Full Docutils doctree/AST.** reST wrap is structural. **Best native semantic model in the survey.** | **Yes** — custom domains/roles/object-types make `BC-1.03.017`/`ADR-039` first-class targets; `Domain.resolve_xref` + `missing-reference` event give corpus-wide resolution; intersphinx imports external inventories. | **Yes, via a custom domain**: store current version in each target object, parse cited version in the role, fail resolution on mismatch; `-W -n --keep-going` makes it a CI gate. Not stock `:ref:`. Note: `-n`/nitpicky checks internal refs; `-b linkcheck` is external-link only — use both. | **v9.1.0, 2025-12-31**; ~8k stars; ~91.8M PyPI downloads/mo. Very mature/active. (Would require moving the corpus to reST or a Sphinx build.) | [S1-sphinx] |
| **Antora / Asciidoctor `xref:`** | Full Asciidoctor AST + Antora content catalog. Versioned coordinates `xref:version@component:module:page#anchor[]`. | Yes for Antora resource semantics; arbitrary `ADR-039` needs a custom inline macro / extension. Spring's antora-xref-extension validates anchors. | Partly native (coordinates encode version) + fully enforceable by a content-catalog extension comparing displayed `v1.27` to target attributes. Not a standard core check. | **Antora 3.1.15, 2026-05-26**; active. (AsciiDoc migration required.) | [S1-antora] |
| **MkDocs + Material + mkdocs-htmlproofer-plugin** | Python-Markdown render-tree + tree processor; `validation.*` + `--strict` (since 1.6). htmlproofer parses rendered HTML (source wrap irrelevant, but can't see source-only syntax). | No native arbitrary object graph; IDs work only as file/anchor. Plugin via `on_page_markdown`/`on_files` can build a registry. | Yes via a plugin/hook, not core link settings. | MkDocs v1.6.1 (2024-08-30, active); **Material v9.7.7, 2026-07-17 but explicitly in maintenance mode**, critical fixes only through Nov 2026 (dev shifting to Zensical). | [S1-mkdocs] |
| **lychee** | pulldown-cmark for Markdown / HTML5 parser for HTML (wrap-safe); reST/AsciiDoc fall back to `linkify` (no AST). | No semantic domain refs; checks HTTP/mail/file + `--include-fragments`. Cannot know `BC-1.03.017` exists in a registry; cannot detect a stale ref whose URL still 200s. | No. A wrapper could synthesize URLs, but lychee has no target-metadata comparison. | **v0.25.0 workflow 2026-07-30**; ~4.2k stars; active. Good transport/anchor second layer only. | [S1-lychee] |
| **VS Code Markdown Language Service / Marksman / markdown-oxide (LSP)** | Token/parser-based; robust to normal wrap. Marksman/oxide index projects (wiki links, headings, backlinks). | Native scope is ordinary Markdown / PKM links; no configurable `BC-`/`ADR-` domain and no documented custom-resolver API. | No built-in version check; would need a bespoke LSP/extension. | vscode-markdown-languageservice 0.5.0-alpha.9 (2025-03-24); Marksman 2026-02-08 (~3.2k stars); markdown-oxide ~0.25.12 (mid-2026). | [S1-lsp] |

**Practical conclusion for Q1.** For the *current* Markdown corpus, the cleanest AST substrate is
**remark/unified** (JS) or a **Rust markdown-AST crate** (native to our WASM hooks). **Sphinx custom
domains** are the strongest *native* version-consistency model but require reST/a Sphinx build we do
not have. Use lychee / MkDocs validation / mdbook-linkcheck / Sphinx linkcheck only as a
transport/anchor second layer — none of them is a semantic or version-consistency validator.

---

## 2. Q2 — The architectural alternative: eliminate load-bearing version pins

**Verdict on Q2: Strongly supported and conclusive. The industry rule is "reference identity in the
prose; bind configuration in the build; freeze provenance in the baseline."** [S2] Standards and
mature tools do **not** put a target version inside every citing reference; they use a stable ID and
resolve/snapshot the version separately.

### 2.1 What the standards say

| Standard / practice | Identity & link model | How versions/baselines work | Requires inline version pins? | Source |
|---------------------|-----------------------|-----------------------------|-------------------------------|--------|
| **ISO/IEC/IEEE 29148:2018** | Unique persistent requirement IDs; traceability links are separate from identification. | Items placed under configuration control; functional/allocated/developmental/product **baselines** capture the point-in-time snapshot; changes are formally captured. | **No.** Does not prescribe a cross-ref syntax or require the target version inline. IEEE overview says it does not prescribe format/medium. | [S2-29148] |
| **ReqIF 1.2 (OMG)** | Every `Identifiable` has a globally unique, **lifetime-immutable** `identifier`; `SpecRelation` links source/target by identity; `lastChange` timestamp. | Exchanges snapshots; IDs correlate the same logical element across exchanges. No baseline/revision field in the core model — left to tools/process. | **No.** Endpoints are identities, not `ID+version` pairs. | [S2-ReqIF] |
| **IBM DOORS / DOORS Next** | Object-to-object links in link modules / relationship records using persistent identity. | A **baseline** is a read-only snapshot of artifact versions; baseline *sets* snapshot modules + the links between them; global configuration selects versions. | **No.** Baseline/config supplies the historical version; explicit version-specific links used only where lifecycle semantics require. | [S2-DOORS][S4-DOORS] |
| **DITA `keyref`/`conkeyref`** | Addresses a **stable key** + optional element ID; maps bind keys to resources at processing time ("late-bound"). | Different maps bind the same key to different resources → version chosen by map/build context. | **No.** The clearest standardized example of the requested pattern. | [S2-DITA] |
| **Traceability matrices (NASA/INCOSE)** | Rows keyed by unique requirement ID + source doc; do not duplicate requirement text. | Matrix is configuration-controlled / generated per named baseline; a regulated project *may* add a "source revision"/"applicable baseline" column, but that is a CM decision, not a universal RTM rule. | **No universal inline-pin rule.** | [S2-NASA] |

**Identity ≠ version.** The canonical decomposition the sources converge on:

```
REQ-SYS-0042              stable identity        (goes in the citing prose)
revision 9                one state of it        (never manually repeated in every citation)
Requirements-Baseline-3.2 named set → revision 9 (the provenance snapshot)
Product-Release-2026.08   higher config → baseline
```

### 2.2 Transclusion / single-sourcing mechanisms (secondary — solves *duplication*, not *version provenance*)

Antora partials/xref, Sphinx `include`/`literalinclude`/substitutions (`|release|`), MkDocs Material
PyMdown Snippets (`--8<--`), AsciiDoc `include::` with tagged regions, Hugo `ref`/`relref` +
`RenderShortcodes`, DITA `conref`. **All resolve at build time from the checked-out source; none of
them supply artifact-version provenance by themselves** — the build inputs still need to be baselined
(Git tag/commit, lockfile, submodule rev). Sphinx substitutions are a literal "derive at build time"
mechanism for *displaying* a resolved version, but do not choose which revision of *another* artifact
to link to. [S2] (Security note: PyMdown Snippets had an arbitrary-file-read advisory GHSA-jh85-wwv9-24hv;
restrict include paths. [S2-snippets])

### 2.3 The trade-off (with the recommended resolution)

| Approach | Advantage | Cost | Best use |
|----------|-----------|------|----------|
| **Inline version pin in every xref (our current design)** | Self-contained point-in-time provenance | **High update churn; stale visible pins; citation vs target can silently disagree** — exactly our failure mode | External/contractual editions where the edition is the meaning |
| **Stable ID → moving "latest"** | Minimal effort, never stale text | Not reproducible; old doc may point at a semantically changed target | Live non-regulated docs where "current" is intentionally the contract |
| **Stable ID + locked baseline/manifest** | No cohort re-anchoring; reproducible; provenance once per release not once per link | Readers need the baseline unless resolved versions are rendered; resolver + governance to build | **Recommended default** for versioned suites, requirements repos, regulatory evidence |
| **Stable ID + generated inline version annotation** | Clean source **and** visible provenance; stale hand-pins impossible by construction | Slightly noisier output; resolver must expose revision metadata | Best compromise when auditors/offline readers need the revision beside each citation |

**The resolution the sources recommend and that fits vsdd-factory:** author `artifact-id[#semantic-anchor]`;
never manually append a version; keep a central resolver (`ID → path → commit/revision → status`)
— **which the project already substantially has in its INDEX files** — and, where visible provenance
is wanted, **generate** the version annotation at render/validate time rather than maintaining it by
hand in every citing doc. Reserve hard inline pins for references whose specific edition is itself the
meaning and cannot be established by the enclosing baseline. [S2]

---

## 3. Q3 / Q3b — AST-based structured detection, incl. Rust/WASM

**Verdict on Q3: The failure modes are fully solvable with an AST-scoped, token-based detector — but
no mature off-the-shelf tool does exactly ID+version consistency with live-section scoping. It is a
small custom rule on a solid AST substrate. The hard part is not parsing; it is defining the
ID↔version association policy.** [S3]

### 3.1 How an AST detector kills each observed failure mode

- **Failure 1 (line-wrapped `BC-1.03.017\nv1.26`).** In every real Markdown AST, a soft line break
  inside a paragraph is a `SoftBreak` node (or retained line-ending in text). Walk the paragraph's
  descendants, **normalize every soft/hard break to a single space**, and the ID and version land in
  one logical text run regardless of physical wrapping. Single-line grep cannot do this; the AST can.
  [S3-algo]
- **Failure 2 (interposed anchor `ADR-039 §Decision 3 v1.10`).** Do **not** use one adjacency regex.
  **Tokenize the ID and the version independently** (`\bADR-(\d+)\b` and `\bv(\d+(?:\.\d+)*)\b`), then
  **associate by nearest-following version within the same logical block**, stopping at another ID or
  a sentence boundary / N-token window. `§Decision 3` between them is irrelevant. [S3-algo]
- **Failure 3 (live vs historical).** Build **section ranges** from heading depth (a section runs from
  heading H to the next heading of depth ≤ H). Use an **allowlist of load-bearing section paths**
  (e.g. `Decision`, `Consequences`, `Implementation`) and a denylist of excluded headings
  (`Changelog`, `History`, `Revision History`, `Superseded`, `Examples`). Exclude YAML/TOML
  **frontmatter by node type** (not by scanning for `---`) and exclude history/changelog **tables** by
  ancestor. This structurally separates live cites from exempt provenance. [S3-sections]
- **Failure 4 (unreliable self-attestation).** Make the check a **mechanical CI/WASM gate**, not a
  human sweep. Combined with the fingerprint/suspect model (§4.2), the gate — not the agent —
  attests propagation. This directly matches the CLAUDE.md D-449(a) "literal-shell-execution-evidence"
  principle: mechanically executed gates beat narrative attestation.

**The recommended algorithm (from the deep-research synthesis) in brief:** parse with the actual
dialect (GFM tables, frontmatter); build heading-based section ranges; apply an allowlist/denylist
scope policy; exclude frontmatter/code/HTML/link-destinations/history-tables; form one logical text
run per paragraph/list-item/cell, normalizing breaks to spaces and *not* inserting false spaces at
inline-markup boundaries (`ADR-**039**` → `ADR-039`, not `ADR- 039`); keep a source map for
diagnostics; tokenize ID and version separately; associate nearest-following within the block;
validate each `(canonical ID, parsed version)` against the registry. [S3-algo]

### 3.2 Rust/WASM markdown-AST crate comparison (our hooks are Rust→WASM)

| Crate | Full AST + source spans? | Soft-break handling | Fit for scoped ID+version detection | Maturity / last release (registry-verified) | Source |
|-------|--------------------------|---------------------|--------------------------------------|---------------------------------------------|--------|
| **comrak** | **Yes** — arena AST, `root.descendants()`, one-based `sourcepos` on every node (reliable for blocks except lists/list-items, inlines, most extensions). | **Explicit distinct `NodeValue::SoftBreak` and `LineBreak`** inline nodes — normalize either to a space while keeping source position. | **Strong.** Has `Heading`, `Table/TableRow/TableCell`, opaque `FrontMatter(String)`. Maintain a heading stack, skip frontmatter, skip table ancestors, allowlist sections. | **0.54.0, 2026-07-12** — *independently verified on crates.io 2026-08-24* (114,382 downloads). Mature (since 2017), active. | [S3-comrak][V-comrak] |
| **pulldown-cmark** | **No owned AST** — pull parser of balanced `Start`/`End` events; `into_offset_iter()` gives `(Event, Range<usize>)` so a stack reconstructs the needed hierarchy. | **Explicit `Event::SoftBreak` / `Event::HardBreak`.** | **Strong** if streaming/speed preferred. Push heading/paragraph/table/metadata state on `Start`, pop on `End`; collect `Text`; append space on breaks. Note consecutive `Text` events (use `TextMergeStream` if no source map needed). | **0.13.4, 2026-05-20** — *independently verified on crates.io 2026-08-24*. De-facto Rust standard; tens of thousands of dependents. | [S3-pulldown][V-pulldown] |
| **markdown-rs (`markdown` crate, wooorm)** | **Yes** — `to_mdast()` full Rust mdast mirroring JS mdast; `Position` with 1-based line/col + 0-based offset. | Ordinary soft endings follow mdast (not `Break`); hard break is `Node::Break`; collectors normalize line endings. | **Strong**, and ports the remark algorithm cleanly to Rust. Has `Heading`, `Table`, `Yaml`, `Toml`. | **1.0.0, 2025-04-23** (this stable release withdrew the old RUSTSEC-2022-0044 informational advisory). Stable but younger than comrak/pulldown. | [S3-markdownrs] |
| **tree-sitter-markdown** | Yes, but a **CST split across two parses** (block grammar marks inline ranges, inline grammar reparses); excellent byte ranges. | Line endings live in source ranges; **soft-break node naming not conclusively documented** — reconstruct from inline source ranges instead. | Usable for structural capture (headings/tables/metadata) but multi-token ID↔version join is easier in host code. **README explicitly warns it is not recommended where correctness matters** (highlighting-oriented). | grammar `v0.4.1, 2025-03-21`; bindings vary. **Correctness caveat is significant.** | [S3-treesitter] |

**Recommendation for 3b:** build the validator on **comrak** (full AST + explicit `SoftBreak` +
`sourcepos` + frontmatter/table nodes = the least-friction path for section-scoped, wrap-robust,
provenance-aware detection), or **pulldown-cmark** if the WASM fuel budget favors a streaming parser
with no arena allocation (relevant given the project's documented WASM fuel-exhaustion issues on large
artifacts — a streaming event parser has a smaller, more predictable memory/fuel footprint than an
arena AST over multi-thousand-line files). Avoid tree-sitter-markdown for a correctness-critical gate.

### 3.3 Prior art on reference-consistency linters (verify-before-you-name)

| Project | What it is / relevance | Maturity / last release | Fit for exact ID+version problem | Source |
|---------|------------------------|-------------------------|----------------------------------|--------|
| **remark-lint `no-undefined-references` / `no-unused-definitions`** | Official rules that collect definitions and uses in separate passes — direct architectural precedent for a two-pass registry check. | Mature (10.x; releases into 2025). | "Reference" means Markdown `[label][id]`, not prose `ADR-039 v1.10`. Precedent only. | [S3-remarklint] |
| **remark-validate-links** | Repo-wide indexing + source-positioned diagnostics across a Git repo. | 13.1.0, 2025-02-21; **repo reported archived 2026-06-04**. | Validates links/anchors, not textual ID+version pairs. Design precedent only. | [S3-rvl] |
| **mdast-util-heading-range** | Existing utility for heading-delimited ranges — directly useful for "scan this section but not History." | Unified ecosystem utility; exact current release inconclusive. | Useful building block for section scoping. | [S3-headingrange] |
| **contextlint** (contextlint.dev) | **Closest conceptual prior art found.** Checks broken cross-refs, duplicate IDs, missing sections, table constraints, anchors, **stability consistency, traceability chains, cycles, orphan docs**; `REF-002` validates ID definitions/refs across files; `GRP-001` validates traceability chains. | **Young but active**: docs describe 21 rules, v0.9 with LSP/editor integration, April 2026. | Very relevant semantically. **Inconclusive:** public docs did not confirm its underlying Markdown AST library or whether it associates a prose ID with a *non-adjacent* version token. Worth a spike. | [S3-contextlint] |
| **mdbook-lint** (joshrotenberg) | **Closest Rust ADR-specific prior art.** Rules for required ADR sections, valid statuses, unique/sequential ADR numbers, valid ADR links; `ADR010` requires superseded ADRs to reference replacements; `ADR013` verifies ADR links. | **Active: v0.14.3, 2026-03-04**, 29 releases. | Validates explicit ADR relationships; **no rule for "all live prose refs to ADR-X must carry and agree on version Y."** Parser backend / custom-rule extensibility not conclusively documented. | [S3-mdbooklint] |
| **rumdl** (rvben) | Active Rust Markdown linter/formatter; uses pulldown-cmark; frontmatter-aware. | Active/rapid; GitHub v0.2.4 (2026-05-29) vs packaging 0.2.55 (2026-08) — index inconsistency noted. | No confirmed custom-plugin API or semantic ID/version rule; likely a fork or separate tool. Precedent for a Rust linter architecture. | [S3-rumdl] |
| **textlint + @textlint/markdown-to-ast** | AST-over-mdast prose-linter substrate with `loc`/`range`/`raw`. | Active ecosystem; exact release inconclusive. | Alternative substrate if the rule is mainly prose/token analysis; repo-wide indexing still custom. | [S3-textlint] |
| **traceability-tool (`tracey`)** | Markdown requirement IDs in frontmatter + source annotations → trace links. | Release/maturity **inconclusive** in retrieved results. | Domain-relevant; AST/soft-break/live-scoping behavior unconfirmed. | [S3-tracey] |

**Prior-art gap (conclusive finding):** mature components exist for AST-aware linting, undefined-ref
checks, link/anchor validation, ADR relationship rules, and requirement-ID traceability graphs, but
**no clearly documented, mature, public rule combines all five requested semantics** — (1) prose ID
and version detected independently, (2) associated despite wrapping/intervening text, (3) compared to
a canonical version registry, (4) only live/load-bearing heading ranges counted, (5) frontmatter and
changelog/history tables excluded structurally. **This is a custom rule** — but a small one atop
comrak/pulldown-cmark/remark. contextlint and mdbook-lint are the two most promising bases to spike
before building from scratch. [S3-gap]

---

## 4. Q4 — Requirements-traceability & ADR tooling: how mature tools avoid the churn

**Verdict on Q4: Conclusive. Mature tooling avoids version-bump churn in one of three ways —**
(a) immutable artifacts with new stable IDs + explicit supersession; (b) stable logical IDs whose
links survive content changes while the tool marks them **"suspect"**; (c) a model/graph from which
traceability views are **regenerated**. **The stand-out direct match is Doorstop's fingerprint/
suspect-link model. OpenFastTrace is the deliberate counter-example that mirrors our current pain.**
[S4]

### 4.1 Comparison

| Tool | Exists / maturity / last verified release | Xref: stable ID or ID+version? | Auto stale/suspect detection on target change? | Auto-generated traceability matrix? | Churn assessment | Source |
|------|-------------------------------------------|--------------------------------|------------------------------------------------|-------------------------------------|------------------|--------|
| **adr-tools (npryce)** | Exists; classic, **release-dormant: 3.0.0, 2018-07-25** (still packaged in Homebrew/Ubuntu). | ADR number / filename link, **not a version**. `adr new -s 9` writes the supersession link both ways. | **No** content-change/fingerprint detection; only the explicit `-s` lifecycle op. | Index only (`adr generate toc`), not an RTM. | **Good** — ADRs immutable; a changed decision gets a *new ID* + one supersession link; no "version bump" for all citations to chase. | [S4-adrtools] |
| **log4brains (thomvaill)** | Exists; stable, low recent activity: **v1.1.0, 2024-12-17**. | Stable **date+slug** filename as unique ID; refs are Markdown links resolved to routes; **no version in ref**. | **No** verified suspect-on-change; ADR is immutable except status → change = deprecation/supersession. | Auto-builds a searchable static decision site/nav; not an RTM. | **Low churn** via immutability + stable slugs; supersession links still hand-authored. | [S4-log4brains] |
| **Structurizr / C4** | Exists; mature, active docs: CLI **v2025.03.28**; DSL/tooling active 2026. | Elements/relationships are first-class **model entities** by DSL identifier; ADRs imported via `!adrs`, supersession from content — **not version-qualified**. | **No** verified ADR suspect-link feature (model parsing catches unresolved IDs structurally). | **Yes for C4 diagrams/relationship views** (derived from model); **partial for decisions** (decision nav + supersession graph). | Strong for model relationships; ADR supersession stays an explicit historical edge. | [S4-structurizr] |
| **dotnet-adr (endjin)** | Exists; stable, low recent activity: NuGet `adr` **1.1.8, 2024-03-21**. | Numbered ADR identity; `adr new -i N` creates replacement + updates old status. | **No** fingerprint mechanism. | **No** verified RTM. | Same as adr-tools: new ID + two-sided supersession. | [S4-dotnetadr] |
| **adr-log** | Exists; mature single-purpose, **2.2.0, 2020-10-21**. | Discovers ADRs by prefix/frontmatter; emits `[ADR-0001](0001-…md)` links; **no target version**. | **No.** | **Auto-regenerates an ADR index/TOC** between markers — eliminates *index* churn, not semantic-ref churn. | Removes index churn only. | [S4-adrlog] |
| **Doorstop (doorstop-dev)** | Exists; **mature docs-as-code requirements tool: 3.0.2, 2025-06-16**; active 2026. | Stable **UID**; a child link stores **parent UID + the parent fingerprint last reviewed** (`REQ001: <hash>`) — **not a human version number**. | **YES — automatic suspect detection.** If current parent fingerprint ≠ stored fingerprint, validation marks the link suspect; `doorstop clear` records review without editing every target/version by hand; also reports invalid/unknown UIDs. | **YES** — validates the tree and publishes the hierarchy **including a trace matrix** (text/HTML/LaTeX/Markdown). | **Best direct match.** Stable UID prevents re-anchoring; stored fingerprint gives automatic impact/suspect signaling; review clears suspicion without changing the logical target ID. | [S4-Doorstop] |
| **Sphinx-Needs (useblocks)** | Exists; **mature, highly active: 8.3.1, 2026-08-11**. | Links carry **only the need ID** (`:links: REQ_001`); `:need:` derives title/URL; incoming links derived from outgoing. Versioned `needs.json` snapshots optional. | Broken/unknown ID → dead-link warning **yes**; **semantic-change suspect-review state: no** built-in (same ID retained ⇒ incoming links not auto-suspect). | **YES** — `needtable`/`needflow` generate filtered tables + relationship diagrams from the needs graph. | No bump churn when stable IDs retained; strong generated traceability; no Doorstop-style suspicion. | [S4-sphinxneeds] |
| **StrictDoc** | Exists; rapid 0.x, mature feature set: docs identify **0.28.1, gen 2026-08-18** (exact release timestamp not independently verified). | Stable `UID`; `RELATIONS` Parent/Child/File; inline `[LINK: <UID>]`; document `VERSION` is separate metadata (often from Git) — **not in links**. | Reference integrity (missing links/cycles/dead source refs) **yes**; computes diffs/changelogs between versions; **Doorstop/DOORS-style per-link suspect-on-text-change: not verified**. | **YES** — full Traceability Index + forward/backward + compliance matrices. | Strong stable-ID + generated-matrix design; add a review policy if true suspect-link semantics needed. | [S4-strictdoc] |
| **OpenFastTrace (OFT)** | Exists; mature, active: GitHub **4.2.0, 2025-06-22** (Maven Central 4.9.0 indexed Aug 2026 but release date unverified). | **Explicit ID + revision:** `artifact-type~name~revision` (`req~name~1`); coverage tags reference the full identity (`[impl->req~example~1]`). | **YES, by deliberate revision invalidation** (not fingerprints): incrementing a semantic revision **voids all existing coverage links**, forcing covering-item authors to re-inspect. Trivial edits should *not* bump the revision. Reports under/over/missing coverage. | **YES** — imports requirements + coverage tags, links, computes coverage, generates HTML/tracing reports. | **This intentionally creates controlled re-anchoring churn** — appropriate for certification, and **a precise mirror of our current failure mode.** Use stable revisions for non-semantic edits. | [S4-OFT] |
| **Capella Requirements VP / ReqIF** | Exists; mature add-on, slow cadence: **v0.14.0, 2024-07-17**; Capella 7.1 compat concerns reported. | ReqIF identities preserved; relationships are first-class model objects, not textual ID+version. | **No** verified auto-suspect in the add-on; iterative ReqIF re-import has documented link-breakage cases. | Derived visualization yes; automatic link creation generally no. | Stable identity usually avoids textual re-anchoring; test iterative-import robustness. | [S4-capella] |
| **IBM DOORS / DOORS Next** | Exists; enterprise-mature, active: Classic **9.7.2.10, 2025-07-07**; Next **7.2.0 GA 2025-12-18**. | Object-to-object links using persistent identity; baselines/configs snapshot separately — **no version embedded per citation**. | **YES** — marks linked objects suspect on target change, records changes since suspicion cleared, clear from either endpoint; DOORS Next Links Explorer flags links from changed artifacts. | **YES** — dynamic traceability/impact views (not hand-maintained matrices). | **Canonical enterprise solution:** stable links survive revisions; changed content produces *review work*, not mass reference editing. | [S4-DOORS] |
| **"ITU requirement tracing"** | **Could not verify a distinct tool by this name.** Searches returned unrelated tools + OFT docs. Likely shorthand/typo for in-source/OFT coverage-tag tracing. | Unverified | Unverified | Unverified | Do not attribute to a separate "ITU" tool without a canonical URL. | [S4-itu] |

### 4.2 The Doorstop model, applied to vsdd-factory (the key transferable pattern)

Doorstop's mechanism is the exact answer to failure mode 4 ("unreliable self-attestation") and to the
churn generally:

1. Each target artifact (BC/ADR/VP/story) has a **stable UID** (already true) and a computable
   **fingerprint** (content hash, or the existing version field used as the fingerprint).
2. Each *link* records `parent-UID: <fingerprint-at-last-review>` — machine-readable, in a sidecar,
   **not** re-typed into prose.
3. On validate, if `current-fingerprint(parent) != stored-fingerprint`, the link is **automatically
   flagged suspect**. The agent/human does not attest propagation — the tool computes it.
4. Reviewing the citing doc and running the equivalent of `doorstop clear` updates the stored
   fingerprint. **No cohort-wide prose edit; only a reviewed metadata clear.**
5. The trace matrix is **generated**, not hand-maintained.

This maps directly onto the project's existing INDEX registries and WASM-hook gate architecture, and
onto the D-449(a) "mechanical gate executed, not narratively attested" principle. It converts
"re-anchor every citing document's inline `vN`" (churn) into "clear a reviewed fingerprint on the
links that actually depend on the changed content" (bounded, mechanical).

---

## 5. Strategic recommendation (the terminal decision)

**Decision: HYBRID — architectural elimination first, mechanical detector second.** Concretely, in
priority order:

**Tier 1 — Eliminate load-bearing inline version pins (removes the disease).**
Stop authoring `BC-1.03.017 v1.27` / `ADR-039 §Decision 3 v1.10` as live references. Author the
**stable anchor only** (`BC-1.03.017`, `ADR-039 §Decision 3`). The version is resolved/verified
mechanically against the project's existing INDEX registries at lint time, and *if visible provenance
is wanted*, generated into rendered output — never hand-maintained in the citing doc. This is the
ISO 29148 / ReqIF / DITA / DOORS / Doorstop / Structurizr / log4brains consensus and it makes
cohort-wide re-anchoring **structurally impossible**, which is strictly stronger than making stragglers
*detectable*. [S2][S4] It also directly ends the perturbation of the 3-CLEAN convergence loop, because
a target version bump no longer forces edits to any citing document.

**Tier 2 — Adopt the Doorstop fingerprint/suspect-link model for genuine dependency tracking.**
Where a citing document genuinely *depends on the content* of a target at a point in time (decision-log
D-NNN prose discussing a specific prior revision; provenance chains), record the dependency as a
machine-readable `UID: fingerprint` link in a sidecar and auto-flag suspect on change, cleared by
review. This replaces unreliable human "full propagation" attestation with a computed gate. [S4-Doorstop]

**Tier 3 — Build the residual AST detector on comrak/pulldown-cmark (guards the migration and the
exempt set).** For any pins that remain live during and after migration, replace regex/manual sweeps
with a WASM lint hook that: builds heading-based section ranges; allowlists live sections and excludes
frontmatter + changelog/history tables by node type; normalizes soft breaks to spaces; tokenizes ID
and version independently and associates by nearest-following within a logical block; and validates
against the registry. This defeats failure modes 1–3 by construction. Prefer **comrak** (0.54.0,
verified) for AST richness, or **pulldown-cmark** (0.13.4, verified) for a smaller WASM fuel footprint
given the project's known fuel-exhaustion constraints. Spike **contextlint** and **mdbook-lint** first
to see if either can be configured/extended rather than built from scratch. [S1][S3][S4]

**Rationale for the weighting.** "Build a better detector" alone (Tier 3 only) was rejected as the
primary strategy because it treats the symptom: even a perfect detector leaves every version bump
forcing a cohort-wide edit and perturbing convergence. The OpenFastTrace evidence is decisive — the
*only* mature tool that embeds revision-in-reference does so specifically to force re-anchoring on
every semantic revision, which is exactly our pain; that design is reserved for safety certification,
not for internal spec coherence. Every other mature tool separates identity from version. The
architectural change (Tiers 1–2) is therefore the load-bearing fix; the detector (Tier 3) is the
necessary guard-rail and migration aid, not the strategy.

---

## 6. Inconclusive areas / follow-up needed

1. **contextlint's internals** — it is the closest semantic prior art (REF-002, GRP-001, stability
   consistency, traceability chains) but public docs did not confirm its Markdown AST library or
   whether it associates a prose ID with a *non-adjacent* version token. **Follow-up:** short spike /
   contact maintainers before adopting or reimplementing. [S3-contextlint]
2. **mdbook-lint extensibility** — strong Rust ADR prior art, but custom-rule/parser-backend
   extensibility for an "all-live-refs-agree-on-version" rule was not conclusively documented.
   **Follow-up:** read the crate's plugin/rule API. [S3-mdbooklint]
3. **StrictDoc & Sphinx-Needs suspect-on-content-change** — both have stable-ID + generated matrices,
   but neither was verified to implement Doorstop/DOORS-style *per-link suspect on target text change*.
   If that semantic is wanted without adopting Doorstop, confirm feasibility. [S4-strictdoc][S4-sphinxneeds]
4. **tree-sitter-markdown soft-break node naming** — not conclusively documented; irrelevant if
   comrak/pulldown-cmark are chosen (both have explicit SoftBreak). [S3-treesitter]
5. **OpenFastTrace 4.9.0** — Maven Central indexed 4.9.0 in Aug 2026 but its release date/GitHub tag
   could not be verified; last fully verified release is 4.2.0 (2025-06-22). [S4-OFT]
6. **"ITU requirement tracing"** — no distinct tool of this name could be verified; treat as
   OFT/in-source coverage-tag tracing unless a canonical URL surfaces. [S4-itu]
7. **remark-validate-links archival** — one source reported the repo archived 2026-06-04; if the JS
   path is chosen, confirm maintenance status or budget for a fork. [S3-rvl]
8. **Migration cost of Tier 1** — the sources establish the *target* design but not the *cost* of
   converting an existing large corpus of inline pins to stable-anchor + derived-version. That is an
   internal estimation task, not a research question; flagged so it is not lost.

---

## 7. Sources

All accessed **2026-08-24**. Tool versions in §§1,3,4 are as reported by the cited pages / release
metadata; the two load-bearing Rust crate versions were independently re-verified against crates.io.

**Q1 — docs-as-code xref tooling (Perplexity deep-research synthesis + primary docs cited within):**
- [S1-vale] Vale — github.com/vale-cli/vale ; vale.sh/features/markup ; docs.vale.sh/topics/scopes ; docs.vale.sh/guides/regex
- [S1-mdl] markdownlint — github.com/DavidAnson/markdownlint/blob/main/doc/CustomRules.md ; github.com/DavidAnson/markdownlint-cli2/blob/main/CHANGELOG.md
- [S1-remark] remark/remark-lint — github.com/remarkjs/remark ; github.com/remarkjs/remark-lint
- [S1-mdbook] mdBook + linkcheck — rust-lang.github.io/mdBook/continuous-integration.html ; github.com/Michael-F-Bryan/mdbook-linkcheck/releases
- [S1-sphinx] Sphinx — sphinx-doc.org/en/master/usage/configuration.html ; sphinx-doc.org/en/master/extdev/domainapi.html ; sphinx-doc.org/en/master/usage/extensions/intersphinx.html
- [S1-antora] Antora/Asciidoctor xref — docs.antora.org/antora/latest/page/xref/ ; docs.antora.org/antora/latest/whats-new/ ; github.com/spring-io/antora-xref-extension
- [S1-mkdocs] MkDocs/Material/htmlproofer — mkdocs.org/user-guide/configuration/ ; squidfunk.github.io/mkdocs-material/changelog/ ; squidfunk.github.io/mkdocs-material/blog/2026/02/18/mkdocs-2.0/ ; github.com/manuzhang/mkdocs-htmlproofer-plugin
- [S1-lychee] lychee — lychee.cli.rs/internals/how-it-works/ ; lychee.cli.rs/recipes/anchors/ ; github.com/lycheeverse/lychee
- [S1-lsp] LSPs — github.com/microsoft/vscode-markdown-languageservice ; github.com/artempyanykh/marksman ; oxide.md/

**Q2 — eliminate version pins / single-source-of-truth (Perplexity deep-research synthesis + standards):**
- [S2] Overall synthesis "reference identity in the prose; bind configuration in the build; freeze provenance in the baseline."
- [S2-29148] ISO/IEC/IEEE 29148:2018 — standards.ieee.org/standard/29148-2018.html ; normsplash.com/Samples/BSI/141655572/BS-ISO-IEC-IEEE-29148-2018-en-2.pdf
- [S2-ReqIF] OMG ReqIF 1.2 — omg.org/spec/ReqIF/1.2/PDF/changebar ; omg.org/reqif/
- [S2-DITA] DITA key-based addressing — docs.oasis-open.org/dita/v1.2/os/spec/archSpec/key-based_addressing.html ; dita-lang.org/dita/archspec/base/using-keys-for-addressing
- [S2-DOORS] DOORS baselines/baseline-sets — ibm.com/docs/.../doors/9.7.2?topic=requirements-baseline-set-definitions-baseline-sets ; ibm.com/docs/.../doors-next/7.1.0?topic=requirements-managing-artifacts-by-using-configurations
- [S2-NASA] NASA RVM / SE handbook — nasa.gov/reference/appendix-d-requirements-verification-matrix/ ; swehb.nasa.gov/plugins/viewsource/viewpagesrc.action?pageId=215777306
- [S2-antora] Antora resource-ID coordinates / version sorting — docs.antora.org/antora/latest/page/resource-id-coordinates/ ; docs.antora.org/antora/latest/how-component-versions-are-sorted/
- [S2-snippets] PyMdown Snippets + advisory — facelessuser.github.io/pymdown-extensions/extensions/snippets/ ; github.com/facelessuser/pymdown-extensions/security/advisories/GHSA-jh85-wwv9-24hv

**Q3 / Q3b — AST-based detection + Rust/WASM (Perplexity deep-research synthesis + docs):**
- [S3-algo] recommended extraction/association algorithm (soft-break normalization; independent ID/version tokenization; nearest-following association) — synthesis
- [S3-sections] section-boundary-aware linting; frontmatter/table exclusion by node type — synthesis
- [S3-comrak] comrak — docs.rs/comrak ; docs.rs/comrak/latest/comrak/nodes/enum.NodeValue.html ; github.com/kivikakk/comrak/releases
- [S3-pulldown] pulldown-cmark — pulldown-cmark.github.io/pulldown-cmark/ ; docs.rs/pulldown-cmark/latest/pulldown_cmark/ ; github.com/pulldown-cmark/pulldown-cmark/releases
- [S3-markdownrs] markdown-rs — github.com/wooorm/markdown-rs ; docs.rs/markdown/latest/markdown/mdast/ ; rustsec.org/advisories/RUSTSEC-2022-0044.html
- [S3-treesitter] tree-sitter-markdown — github.com/tree-sitter-grammars/tree-sitter-markdown ; tree-sitter.github.io/tree-sitter/using-parsers/queries/4-api.html
- [S3-remarklint] remark-lint no-undefined-references — github.com/remarkjs/remark-lint ; npmjs.com/package/remark-lint-no-undefined-references
- [S3-rvl] remark-validate-links — github.com/remarkjs/remark-validate-links/releases
- [S3-headingrange] mdast-util-heading-range — github.com/syntax-tree/mdast-util-heading-range
- [S3-contextlint] contextlint — contextlint.dev/ ; contextlint.dev/docs/rules/
- [S3-mdbooklint] mdbook-lint — github.com/joshrotenberg/mdbook-lint ; joshrotenberg.com/mdbook-lint/rules/adr/
- [S3-rumdl] rumdl — github.com/rvben/rumdl ; github.com/rvben/rumdl/blob/main/CHANGELOG.md
- [S3-textlint] textlint markdown-to-ast — npmjs.com/package/@textlint/markdown-to-ast
- [S3-tracey] traceability-tool — github.com/konstantin-hatvan/traceability-tool
- [S3-gap] prior-art gap assessment — synthesis

**Q4 — traceability & ADR tooling (Perplexity deep-research synthesis + primary docs):**
- [S4-adrtools] adr-tools — github.com/npryce/adr-tools ; github.com/npryce/adr-tools/releases ; formulae.brew.sh/formula/adr-tools
- [S4-log4brains] log4brains — github.com/thomvaill/log4brains ; github.com/thomvaill/log4brains/releases ; thomvaill.github.io/log4brains/adr/adr/20201016-use-the-adr-slug-as-its-unique-id/
- [S4-structurizr] Structurizr — docs.structurizr.com/dsl/adrs ; docs.structurizr.com/ui/decisions/ ; github.com/structurizr/cli/releases
- [S4-dotnetadr] dotnet-adr — github.com/endjin/dotnet-adr ; nuget.org/packages/adr
- [S4-adrlog] adr-log — github.com/adr/adr-log ; github.com/adr/adr-log/blob/main/CHANGELOG.md
- [S4-Doorstop] Doorstop — github.com/doorstop-dev/doorstop ; doorstop.readthedocs.io/en/latest/cli/validation.html ; doorstop.readthedocs.io/en/latest/cli/publishing.html ; github.com/doorstop-dev/doorstop/blob/develop/docs/reference/item.md
- [S4-sphinxneeds] Sphinx-Needs — github.com/useblocks/sphinx-needs ; sphinx-needs.readthedocs.io/en/latest/directives/needtable.html ; sphinx-needs.readthedocs.io/en/latest/changelog.html
- [S4-strictdoc] StrictDoc — github.com/strictdoc-project/strictdoc ; strictdoc-project.github.io/features/ ; strictdoc.readthedocs.io/en/stable/sphinx/strictdoc_01_user_guide.html
- [S4-OFT] OpenFastTrace — github.com/itsallcode/openfasttrace ; github.com/itsallcode/openfasttrace/blob/main/doc/user_guide.md ; github.com/itsallcode/openfasttrace/releases ; central.sonatype.com/artifact/org.itsallcode.openfasttrace/openfasttrace/4.2.0
- [S4-capella] Capella Requirements VP — github.com/eclipse-capella/capella-requirements-vp ; github.com/eclipse-capella/capella-requirements-vp/releases
- [S4-DOORS] IBM DOORS suspect links — ibm.com/docs/.../doors/9.7.2?topic=data-suspect-links-changed-objects ; ibm.com/docs/.../doors-next720 ; jazz.net/downloads/requirements-management-doors-next
- [S4-itu] "ITU requirement tracing" — unverified; no canonical source found

**Independent registry verification (crates.io, accessed 2026-08-24):**
- [V-comrak] crates.io/api/v1/crates/comrak → latest & max_stable **0.54.0**, published 2026-07-12
- [V-pulldown] crates.io/api/v1/crates/pulldown-cmark → latest & max_stable **0.13.4**, published 2026-05-20

---

## 8. Research Methods

| Tool | Queries | Purpose |
|------|---------|---------|
| **Perplexity perplexity_research (PRIMARY)** | 4 | Q1 docs-as-code xref/link-integrity tooling; Q2 eliminate-version-pins architecture + standards/transclusion; Q3/Q3b AST-based detection + Rust/WASM crates + prior art; Q4 ADR + requirements-traceability tooling. Each a deep multi-source synthesis with citations. |
| Perplexity perplexity_reason | 0 | — |
| Perplexity perplexity_search | 0 | — |
| Perplexity perplexity_ask | 0 | — |
| Context7 | 1 attempted, 0 succeeded | Attempted to cross-verify comrak/pulldown-cmark docs; **tool unavailable in this environment** (see note below). Substituted with crates.io registry verification. |
| Tavily (any) | 0 | — |
| WebFetch | 2 | Independent registry verification of comrak & pulldown-cmark latest versions/dates against crates.io API. |
| WebSearch | 0 | — |
| Glob | 2 | Checked for prior research + planning artifacts to avoid duplication (none found). |
| Training data | 0 areas | No claim rests on training data alone; all tool/standard/version claims are web- or registry-sourced. Model knowledge used only to structure and interpret the cited findings. |

**Total MCP tool calls:** 4 successful (`perplexity_research` ×4) + 1 failed Context7 attempt.
**Training data reliance:** low — every named tool, standard, and version number is web- or
registry-cited; the two load-bearing Rust crate versions were independently re-verified against
crates.io.

**Context7 availability note (not a full MCP-UNAVAILABLE escalation).** The Context7 tool did not
resolve in this environment — both calls returned verbatim: `Error: No such tool available:
mcp__context7__resolve-library-id`. Perplexity MCP was fully available and provided ≥1 (in fact 4)
`perplexity_research` calls, so the mandatory MCP gate is satisfied; the Context7 gap was mitigated by
direct crates.io registry verification via WebFetch. Flagging so the orchestrator can route Context7
toolchain repair if library-doc lookups are needed in future runs.
