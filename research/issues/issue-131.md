# Issue #131 — URL/Endpoint/Path Coherence Check in consistency-validator

**Date:** 2026-06-09
**Issue:** [#131] `feat(consistency-validator): add URL/endpoint/path coherence check across diagrams + tables + prose`
**Label:** enhancement · **State:** OPEN
**Researcher:** research-agent (vsdd-factory)
**Repo state:** `develop` @ `82163b7f`

---

## Restated Question

The `consistency-validator` agent / `validate-consistency` (a.k.a.
`consistency-validation`) skill does not check that the same logical URL / endpoint /
path is written **identically** across mermaid diagrams, markdown tables, code fences,
and body prose within a spec package. A real case (`monocle`, 2026-05-12) had three
artifacts citing the same hook endpoint with two divergent paths
(`/hooks/user-prompt-submit` vs `/hooks/prompt-submit`); the divergence survived 5
validation passes because the validator checks IDs, anchors, counts, naming, and
frontmatter cross-refs — but **not free-text path-string coherence in body content**.
The ask: add a "URL/endpoint/path coherence" axis (BLOCKING) that extracts path
candidates, clusters semantically-equivalent ones, and flags clusters with multiple
distinct path strings.

---

## Codebase Grounding

### What the validator covers today

The `consistency-validator` agent (`plugins/vsdd-factory/agents/consistency-validator.md`)
runs **80 numbered criteria**; the `consistency-validation` skill
(`plugins/vsdd-factory/skills/consistency-validation/SKILL.md`) runs **36 rules**.
I read both in full. The closest existing checks to #131's ask are:

- **Agent Criterion 14** — "API contracts are consistent across all documents:
  *Endpoint paths, methods, status codes match*" (severity Major). **But** the
  surrounding criteria (13 data models, 9 PRD→story) and the agent's own
  "Context Discipline" (index-first; load `BC-INDEX`, `VP-INDEX`, etc.) make clear
  this is a **structured-field / index-level** check, not a free-text scan of path
  strings embedded in mermaid arrows or prose tables. There is no operational
  procedure in either the agent or the skill describing how Criterion 14 extracts and
  compares literal path strings across diagram/table/prose representations.
- **Skill Rule 8** — "Data Model Consistency… compare entity names and field types"
  (structured, not path strings).
- **Skill Rule 11 / agent "Drift Detection"** — semantic-similarity and naming-drift
  detection, but operating on BC/AC intent and renamed IDs, not URL path literals.

### The confirmed gap

I grepped the `consistency-validation` SKILL for
`endpoint|URL|/hooks/|API contract|path coherence|mermaid|diagram` — **zero matches**.
The issue's enumeration of what the validator does NOT look at is accurate:

- URL paths in mermaid diagrams vs. URL paths in body prose tables — **not checked**.
- Endpoint paths in prose vs. in schemas/code fences — **not checked**.
- File paths embedded in body content (frontmatter paths ARE checked via
  `traces_to`/`inputs`/`supplements`; **body-text paths are not**).

No `url.coherence`/`endpoint.coherence`/`.factory/consistency-validator.config.yaml`
hits anywhere in the repo. The decision-log and CHANGELOG contain no closure of a
path-coherence concern. The agent is read-only (Profile `coding`; denies `exec`),
so any extraction logic must be prose-pattern based, not a runtime probe.

### Routing note

Per CLAUDE.md Companion Principle, the consistency-validator **detects** but does not
own spec content. A path divergence it finds routes to the content owner
(business-analyst / product-owner / architect) for the canonical-path decision — the
issue's `canonical_source` config field encodes exactly this. Adding the *axis* is a
consistency-validator change (its own domain); choosing the canonical path on a real
finding is the content owner's call.

---

## External Research

### The fundamental divide: link-resolution vs. semantic-path-coherence

Perplexity deep-research confirms the issue is targeting a real, under-served niche:

> "the domain of semantic path coherence checking—ensuring consistent representation
> of identical API endpoints across varied documentation formats—remains surprisingly
> underdeveloped."

> "link resolution checking and semantic path consistency verification represents a
> fundamental conceptual divide… Link resolution checking, implemented by tools such
> as markdown-link-check and remark-lint-no-dead-urls, focuses exclusively on whether
> a given URL can be successfully accessed via HTTP… semantic path coherence
> validation… examines whether textual representations of API endpoints maintain
> consistent string patterns across diverse documentation artifacts… at the lexical
> level rather than the network layer."

Implication: off-the-shelf markdown link checkers (**lychee**, **markdown-link-check**,
**remark-lint-no-dead-urls**) do **NOT** solve #131 — they verify a URL *resolves*,
not that two prose mentions of the same logical endpoint *agree on the path string*.
The `/hooks/user-prompt-submit` vs `/hooks/prompt-submit` case would pass every link
checker (neither is even a resolvable HTTP URL — they're wire-contract paths).

### Where coherence partially exists for free: OpenAPI/AsyncAPI

> "OpenAPI and AsyncAPI specifications inherently promote path coherence through their
> structured representation of endpoints, as each path appears exactly once in the
> specification with consistent representation."

> "[Spectral supports] custom rules using YAML configuration files that specify
> patterns to validate against path strings… a rich library of built-in functions."

So one production-grade *architectural* answer is **single-source-of-truth**: define
each endpoint once (in a schema/index the validator already reads) and have diagrams +
prose *reference* it rather than re-spell it. Spectral (OpenAPI/AsyncAPI linter) and
Vale (prose linter) can enforce path patterns but neither natively does cross-artifact
*diagram-vs-prose-vs-table* clustering. Vale is the closest off-the-shelf prose tool:

> "Vale, a customizable syntax-aware prose linter… convert[s] style guides into
> executable validation rules that can target API-specific terminology and path
> patterns within documentation prose… [with] custom rules specifically designed to
> identify inconsistent path representations… [flag] variations [against a] canonical
> path pattern."

But Vale operates on prose, not mermaid arrow targets or table cells specifically, and
must be told the canonical pattern.

### Clustering approach (the issue's "full" first cut) and its false-positive risk

> "clustering algorithms that group near-identical path strings while considering
> their surrounding documentation context… using Levenshtein distance [the minimum
> number of single-character edits]… clusters of similar paths that likely represent
> the same logical endpoint but with inconsistent textual representations, flagging
> them for human review."

Critical caveat (the issue's "Notes / false-positive" concern is corroborated):

> "Clustering for path coherence validation requires sophisticated contextual filtering
> mechanisms to avoid the high false positive rates that plague naive string similarity
> approaches… effective when combined with contextual constraints specific to API path
> structures."

So the issue's **"heuristic-only first cut"** (grep a project-configurable endpoint
prefix like `/hooks/[a-z-]+`, list unique-by-spelling results with file:line, flag
when >1 spelling shares a file set) is the **lower-false-positive, recommended starting
point** — it side-steps the Levenshtein-clustering false-positive problem entirely by
requiring an explicit prefix pattern. The deep-research supports configurable
contextual constraints over naive edit-distance.

Primary sources (per deep-research citations): Spectral/Redocly (OpenAPI linting),
[vale.sh] (prose linting), markdown-link-check / remark-lint-no-dead-urls (link
resolution only), Levenshtein-distance clustering literature.

---

## Verdict

**VALID-NEW** — Confidence: **HIGH**

Rationale:
- The gap is **confirmed**: zero path-coherence prose-scanning in the 36-rule skill;
  agent Criterion 14 is a structured/index-level check, not the diagram/table/prose
  textual coherence the issue describes. The `monocle` reproduction is consistent with
  the actual validator scope.
- The need is **substantively unique** — no off-the-shelf tool covers semantic
  cross-artifact path-string coherence (deep-research: "surprisingly underdeveloped";
  link checkers solve a different problem). This is genuinely new validator surface,
  not a re-skin of an existing rule.
- Wire-contract divergence is load-bearing (BLOCKING severity is justified): an
  implementer wiring the wrong path builds a non-compliant artifact that fails clone/
  DTU validation. This aligns with vsdd-factory's production-grade default — it is not
  a cosmetic axis.

Slight nuance (keeps it VALID-NEW rather than VALID-PARTIAL): Criterion 14 *names* the
intent ("endpoint paths match") but provides **no operational mechanism** for the
diagram/table/prose textual scan — so the issue adds the missing implementation of a
stated-but-unimplemented intent, which is net-new work.

---

## Recommended Approach (zero re-research)

**Owning agent/skill:** `consistency-validator` agent + `consistency-validation`
(and `validate-consistency`) skill. Test fixtures under
`plugins/vsdd-factory/tests/fixtures/consistency-validator/`. On a real finding,
route the canonical-path decision to business-analyst/product-owner/architect.

### Scope decomposition (recommend the heuristic-first cut as v1)

1. **v1 — configurable-prefix heuristic (low false-positive, ships fast, production-grade).**
   Add a new criterion/rule "URL/Path Coherence":
   - Extract path candidates from **all** spec artifacts in the package: mermaid code
     blocks (arrow targets `--> POST /path`), markdown table cells containing
     `/<segment>`, code fences (rust/yaml/toml/json), and prose via
     `(?:GET|POST|PUT|DELETE|PATCH)\s+\S+/\S+` plus a project-configurable endpoint
     prefix (e.g. `/hooks/[a-z-]+`, `/api/v[0-9]+/[a-z-]+`).
   - For each configured prefix, collect unique-by-spelling results with `file:line`.
     If >1 spelling exists AND the same file set references both, emit a BLOCKING
     finding listing every variant + location.
   - This is the issue's "heuristic-only first cut" and the deep-research-endorsed
     low-false-positive path (explicit contextual constraint, no Levenshtein noise).
     It would have caught the `monocle` case in <30s.

2. **v2 (optional, only if v1 proves insufficient) — context-aware clustering.**
   Levenshtein + ±50-char surrounding context to group "same semantic endpoint"
   without a pre-declared prefix. **Gate this behind the deep-research's explicit
   warning**: naive edit-distance clustering has "high false positive rates"; only add
   it with contextual filtering, and keep it ADVISORY (not BLOCKING) until tuned.
   Per the production-grade default, do NOT ship v2 half-tuned — defer the whole v2
   *feature* rather than ship a noisy version of it.

3. **Configuration.** Add `.factory/consistency-validator.config.yaml` (the issue's
   sketch): `url_coherence.enabled`, `patterns` (prefix list), `severity: BLOCKING`,
   `canonical_source` (artifact that wins on conflict). Production-grade default if
   no config present: enabled with a built-in `/hooks/[a-z-]+` + `/api/...` pattern,
   so it's not silently off.

4. **Report integration.** New "§URL/Path Coherence" section in the consistency report
   (template `../../templates/consistency-report-template.md`). Severity BLOCKING.
   Evidence format: variant string + `file:line` per the issue.

5. **Test fixture (issue AC-3).** Reproduce the `monocle` UserPromptSubmit case under
   `plugins/vsdd-factory/tests/fixtures/consistency-validator/` — 3 files, 2 distinct
   paths — and assert the validator flags BLOCKING.

### Key files

- `plugins/vsdd-factory/agents/consistency-validator.md` — add the criterion (its
  scope is criteria 1–80; this becomes criterion 81 or folds into a redefined 14).
- `plugins/vsdd-factory/skills/consistency-validation/SKILL.md` — add the rule (37th)
  + the `validate-consistency/SKILL.md` sibling.
- `plugins/vsdd-factory/tests/fixtures/consistency-validator/` — new fixture dir.
- `../../templates/consistency-report-template.md` — new report section.

### Risks / dependencies

- **False-positive risk** is the dominant design risk — mitigated by recommending the
  prefix-heuristic v1 over Levenshtein clustering (deep-research-supported).
- **No-exec constraint:** consistency-validator denies `exec`; extraction must be
  pure-prose pattern matching (consistent with how it reads indexes today). No runtime
  endpoint probing.
- **Reconcile Criterion 14:** redefine/cross-reference agent Criterion 14 so the new
  axis and the existing "API contracts consistent" criterion don't double-report or
  contradict. (Companion-Principle: this is a consistency-validator-internal change,
  not a content rewrite.)
- **Cross-issue:** independent of #151 and #172; no shared code surface.

---

## Research Methods

| Tool | Queries | Purpose |
|------|---------|---------|
| **Perplexity perplexity_research (PRIMARY)** | 1 (`reasoning_effort=high`) | Cross-artifact URL/path coherence checking: link-resolution vs semantic-coherence divide; lychee/markdown-link-check/remark scope; Spectral/OpenAPI/AsyncAPI/Vale; Levenshtein clustering + false-positive risk |
| Read | 3 | issue body; consistency-validator agent (80 criteria); consistency-validation skill (36 rules) |
| Grep | 4 | endpoint/URL/mermaid scan of skill (zero hits); `url.coherence` repo sweep; decision-log/CHANGELOG closure check; deep-research extraction |
| Training data | 0 areas | All tool/technique claims sourced from live deep-research |

**Total MCP tool calls:** 1 (shared across the 3-issue batch; this issue's findings drawn from the dedicated path-coherence deep-research call)
**Training data reliance:** LOW — link-checker/Spectral/Vale/clustering claims are externally sourced and version-agnostic (capability claims, not pinned versions).
