---
document_type: proposal
proposal_id: SK-MCP-001
title: "spec-kit MCP server + dispatcher integration"
status: draft
author: orchestrator (via architect dispatch)
date: 2026-05-17
amended:
  - date: 2026-05-17
    summary: "(1) added `prerequisite: UNI-PLUG-001` to frontmatter; (2) binary path updated to plugin-namespaced layout per UNI-PLUG-001 §3; (3) dispatcher hook priority set to 500 (UNI-PLUG-001 plugin band); (4) hooks-registry.toml manual-edit instructions replaced with `plugin.toml` declaration pattern per UNI-PLUG-001 §4."
  - date: 2026-05-18
    summary: "(S-15.14 amendment) Added second empirical basis: vsdd-factory S-15.14 LOCAL adversary cascade pass-1..11 (TD-VSDD-095..100, 6 META classes codified, best streak 1/3 under prose-only governance). Extended ArtifactKind with 5 OPS-layer variants. Added INV-011..014 typed ops-layer invariants. Added §1.5 second empirical basis. Added §3.4 ops-layer tool surface. Updated §4 dispatcher integration with validate-dispatch-advance precedent. Updated §10 references. Added Appendix D mapping TD-VSDD-095..100 → INV-NNN."
empirical_basis:
  - "prism cascade session FB53-FB75 (POL-29 v1.14→v1.28, 11 amendments, 23 bursts)"
  - "vsdd-factory S-15.14 LOCAL adversary cascade pass-1..11 (TD-VSDD-095..100, 6 META classes codified, best streak 1/3 under prose-only governance; validate-dispatch-advance hook ships as per-class proof of dispatcher-layer enforcement pattern)"
target: vsdd-factory upstream
implementation_tiers: [Tier 1 read-only MVP, Tier 2 atomic mutations + dispatcher, Tier 3 full schema enforcement]
estimated_effort: ~1 month total all tiers
priority: HIGH (replaces accumulating amendment chains that have NOT reached fixed-point — 11 POL-29 amendments on spec layer, 6 TD-VSDD-09X lessons on ops layer)
prerequisite: UNI-PLUG-001
---

> **Amendments applied 2026-05-17:** (1) added `prerequisite: UNI-PLUG-001` to frontmatter; (2) binary path updated to plugin-namespaced layout per UNI-PLUG-001 §3; (3) dispatcher hook priority set to 500 (UNI-PLUG-001 plugin band); (4) hooks-registry.toml manual-edit instructions replaced with `plugin.toml` declaration pattern per UNI-PLUG-001 §4.

> **Amendments applied 2026-05-18 (S-15.14 amendment):** (A) frontmatter `empirical_basis` updated to cite both prism FB53-FB75 AND vsdd-factory S-15.14 cascade; amended date trail converted to array form. (B) §1.5 added: second empirical basis summary. (C) §5.1 ArtifactKind enum extended with 5 OPS-layer variants; companion struct definitions added. (D) §5.2 invariants extended with INV-011..014 (ops-layer). (E) §3.4 added: ops-layer MCP tool surface (3 new tools). (F) §4.4 added: validate-dispatch-advance as precursor hook. (G) §10 references updated with S-15.14 empirical evidence sources. (H) Appendix D added: TD-VSDD-095..100 → INV-NNN mapping.

# SK-MCP-001: spec-kit MCP Server + Dispatcher Integration

## Executive Summary

The VSDD spec cascade for the prism project ran 23 fix-bursts (FB53-FB75) across passes 65-87,
during which POL-29 (within_fb_sibling_sweep_discipline) was amended **11 times** in a single
session (v1.14 → v1.28). Every amendment closed one class of integrity failure and exposed a
sibling class. The amendment chain has not reached a fixed point. This document proposes
replacing the accumulating prose-rule amendment chain with a structural solution: a **spec-kit
MCP server** backed by a shared Rust library that enforces spec-graph integrity at mutation time,
with a factory-dispatcher hook plugin providing a defensive pre-commit gate.

---

## 1. Problem Statement

### 1.1 Empirical Evidence: The POL-29 Amendment Chain

POL-29 (within_fb_sibling_sweep_discipline, adopted FB50 at v1.12) reached v1.28 by the end of
a single session. The amendment sequence maps directly to discovered classes of knowledge-graph
integrity failure:

| Version | Amendment | Root Failure Class |
|---------|-----------|-------------------|
| v1.15 (FB53) | step 3a(a): error-taxonomy variant-form enumeration | Citation form aliasing |
| v1.16 (FB54) | step 3a(b): ADR-026 D7 pin variants registered | Citation form aliasing |
| v1.17 (FB56) | step 8a: diff-derived value-class enumeration | Side-effect escaping FB declaration |
| v1.18 (FB62) | step 8b: transitive closure | Pin cascade incomplete |
| v1.19 (FB63) | step 8c: per-variant grep enumeration (combined-regex forbidden) | Regex aliasing masking hits |
| v1.20 (FB66) | step 3d: structural-table-completeness sibling-sweep | Missing FSR/token-budget rows |
| v1.21 (FB67) | step 3e: AC-to-Task implementation-instruction coverage | Missing task instructions |
| v1.22 (FB68) | step 3e refinement: construction vs definition site discrimination | Site classification incomplete |
| v1.23 (FB69) | step 8d: META-META self-induced bump detection | Cascade of cascade escape |
| v1.24 (FB71) | step 8e: fixed-point iteration mandate | Self-induced cascade not iterated |
| v1.25 (FB72) | step 8f: INDEX-row summary-cell sync | INDEX row vs changelog asymmetry |
| v1.26 (FB73) | step 8g: cross-value-class side-effect bump detection | Parallel value classes not swept |
| v1.27 (FB74) | step 8f extended: enumerate ALL INDEX files touched in burst | First-application missed sibling INDEXes |
| v1.28 (FB75) | step 8h: same-burst dependent-artifact self-bump + step 8i: within-file self-cite | Two additional self-reference dimensions |

The adversary correctly noted at the close of FB75 (OBS-LP87): "POL-29 is at a
growth-complexity asymptote — 5 step-3a classes + 9 step-8 substeps — and the pattern of
first-application surfacing a sibling class has not broken. Session-reviewer cycle-close
assessment recommended per AgenticAKM 3-iteration diminishing-returns threshold."

### 1.2 Nature of the Problem: Knowledge-Graph Integrity

VSDD spec artifacts form a dense, typed knowledge graph:

```
DI-NNN (domain invariants)
  ↓ cited by
BC-M.NN.NNN (behavioral contracts) — versioned, have frontmatter
  ↓ traced by
VP-NNN (verification properties) — versioned, have frontmatter
  ↓ exercised by
HS-NNN (holdout scenarios)
  ↕
Story specs — versioned, reference BCs + VPs + error codes + ADR pins
  ↕
BC-INDEX, VP-INDEX, STORY-INDEX — summary tables that cache artifact state
  ↕
ADR-NNN (architectural decision records) — versioned, cited by stories + BCs
  ↕
Error taxonomy (E-SPEC-NNN, E-SENSOR-NNN, etc.) — multi-form citations
```

Every node has:
- A canonical version (frontmatter field)
- Multiple citation forms (bare ID, backtick-quoted, parenthesized, with-.md extension)
- Inbound and outbound edges (citations in both directions)
- Index row summary cells that must stay synchronized with the node's current state

Every mutation to a node must propagate to:
1. All inbound citations (callers that reference this node's version pin)
2. All outbound citations (nodes this node references — check for staleness)
3. All INDEX rows that cache this node's version or status
4. All INDEX summary cells (not just the row, but inline summary text)
5. All parallel value-class anchors (if a D7 pin bumps, sweep D3/D5 too)
6. All within-file self-citations (a story's own changelog citing itself)

Every one of the 11 POL-29 amendments is a graph invariant manually encoded as prose. The
fundamental problem is that prose rules cannot be mechanically executed; they depend on agent
attention to correctly identify scope, correctly run grep variants, and correctly iterate
until no new bumps surface. Each new amendment adds another dimension that can be missed.

### 1.3 Why Prose Rules Cannot Converge

Prose rules have three structural weaknesses against knowledge-graph integrity:

**Attention scope decay.** Each POL-29 verification step requires the agent to enumerate ALL
artifact touches, construct per-variant grep commands, run them, compare pre/post counts, and
iterate until the fixed point is reached. With 9 step-8 substeps, the total verification
procedure requires 40-60 mechanical operations that must all be executed in correct sequence.
Agent attention degrades across this sequence.

**The "first application" gap.** Each amendment is written after a defect is found. On its first
application in the NEXT burst, the amendment is correctly applied to the trigger case but misses
the sibling cases that share the same root cause. POL-29 v1.19 (per-variant grep; FB63) caught
error-taxonomy variant aliasing but missed the ADR-026 D7 pin variant aliasing that became
v1.16. POL-29 v1.27 (all INDEX files; FB74) caught the first INDEX file but missed the sibling
INDEX in the same burst. The prose amendment cannot know what it doesn't know.

**No enforcement surface.** A prose rule is read by the agent performing the burst. The same
agent wrote the burst content. Both reader and writer share the same context and the same
cognitive blind spots. The adversary catches failures AFTER the burst is committed. At that
point, finding closure requires a fix-burst (FB+1), which triggers another adversary pass, which
has a chance to find new sibling classes.

### 1.4 Engineering Cost

The 23 fix-burst session (FB53-FB75) had a measurable engineering overhead footprint:

- **Per-burst cost:** Each fix-burst requires an adversary dispatch, a state-manager dispatch,
  and a consistency-validator dispatch. At prism's typical 30-45 min per burst (including
  human review), 23 bursts = ~12-17 hours of session time.
- **Defect escape cost:** F-LP78/79/80/81 all reached Phase 3 implementation territory before
  being caught. F-LP79 (AC-to-Task coverage gap) had direct engineering impact: validators
  E-SPEC-012/013/014 required by BC + Red Gate tests were absent from story Tasks, meaning
  the codebase grep confirmed validators absent at that point.
- **Fixed-point failure cost:** POL-29 has not converged. Each amendment has a non-zero
  probability of surfacing a new sibling class on first application. The expected number of
  remaining amendments is unbounded under the current architecture.

### 1.5 Second Empirical Basis: vsdd-factory S-15.14 Cascade

The prism cascade (§1.1–§1.4) is a SPEC-layer data point. vsdd-factory's own S-15.14
LOCAL adversary cascade provides an independent OPS-layer data point demonstrating the
same structural failure in a different artifact class.

**S-15.14 cascade summary:**

S-15.14 attempted to achieve 3-CLEAN convergence per BC-5.39.001 (requiring three
consecutive adversary passes with zero findings). The cascade ran 11 passes (trajectory:
16→9→8→2→0→1→1→0→4→1→2). Best streak achieved: **1/3, twice**. The cascade never
reached 3/3. Six META-LEVEL classes were codified as TD-VSDD lessons:

| TD lesson | Meta class | Artifact affected | INV closes it |
|-----------|-----------|-------------------|---------------|
| TD-VSDD-095 | TDD micro-commit discipline | Implementer commits | OUT OF SCOPE (implementer-side) |
| TD-VSDD-096 | Literal-evidence stdout in burst-log Dim-2 | burst-log.md | INV-013 |
| TD-VSDD-097-EXTENDED | orchestrator dispatch templates must satisfy ALL 5 BC-5.39.006 v1.2 PCs simultaneously | STATE.md current_step | INV-011 |
| TD-VSDD-098 | compaction-burst sibling-sweep completeness | STATE.md compaction ops | INV-014 |
| TD-VSDD-099 | burst-log own-entry structural integrity (4 Dim blocks + 8 D-444(c) canonical blocks) | burst-log.md | INV-012 |
| TD-VSDD-100 | Dim-2 PC attestations MUST read production artifact (not synthetic `echo` input) | burst-log.md | INV-013 |

**The structural parallel to prism:**

Both cascades exhibit the same failure pattern at different layers:

- **prism (spec layer):** Each POL-29 amendment closed one class of graph-integrity failure
  and exposed a sibling class. 11 amendments; no fixed point.
- **vsdd-factory S-15.14 (ops layer):** Each fix-burst closed one META-LEVEL class of
  ops-artifact integrity and exposed an adjacent one. 11 passes; best streak 1/3, never 3/3.

In both cases, the root cause is identical: **prose rules cannot enforce transitive closure
across interconnected artifacts**. The prism knowledge graph connects BC → VP → Story → INDEX.
The vsdd-factory ops graph connects STATE.md → burst-log → decision-log → lessons → 4-indexes.
The topology differs; the structural failure mode is the same.

**Proof of dispatcher-layer enforcement pattern:**

vsdd-factory S-15.14 ships `validate-dispatch-advance` (priority 154 in hooks-registry.toml).
This hook validates STATE.md `current_step` against all 5 BC-5.39.006 v1.2 postconditions
(PC2 forbidden-meta / PC3 4-index cites / PC4 trajectory-tail length / PC5 D-chain currency /
PC6 canonical marker present) at PreToolUse time. After deploy, all 5 PCs pass against the
production STATE.md — the first real-artifact evidence that dispatcher-layer enforcement of
typed invariants works at the per-class level.

spec-kit generalizes this pattern: instead of one hand-written hook per invariant class,
`vsdd-spec-kit-core` defines invariants structurally and the dispatcher hook calls
`lint_burst()` across ALL artifact classes in one pass. The S-15.14 evidence validates
the architectural pattern; spec-kit extends it to full coverage.

---

## 2. Proposed Solution: spec-kit MCP Server

### 2.1 Domain

spec-kit is a **knowledge-graph integrity service** for VSDD spec artifacts. It provides:

1. A queryable graph model of all spec artifacts and their citations
2. Atomic mutation operations that enforce graph invariants at write time
3. Validation operations that check invariant satisfaction across a set of staged files
4. An MCP interface for proactive agent use during spec authoring
5. A shared Rust library interface for defensive dispatcher hook use at pre-commit time

### 2.2 Runtime

**MCP server** (Rust binary, JSON-RPC 2.0 over stdio per MCP protocol specification).

Rationale for MCP protocol selection:
- Consistent with vsdd-factory's existing MCP integration pattern (Perplexity, Context7, Tavily
  are all MCP servers consumed by factory agents)
- JSON-RPC over stdio is the lowest-friction distribution model: binary ships with the plugin
  package, registered in `.claude/settings.local.json` during onboarding
- Stateful server can maintain an in-memory graph index after initial artifact scan, making
  repeated queries fast without re-parsing markdown on every call
- Rust binary is consistent with vsdd-factory's existing toolchain (Cargo workspace, edition
  2024, stable channel per `rust-toolchain.toml`)

### 2.3 Distribution

spec-kit ships as part of the vsdd-factory plugin package, under the plugin-namespaced
layout established by UNI-PLUG-001 §3:

```
~/.claude/plugins/cache/claude-mp/vsdd-factory/<version>/
  plugins/
    spec-kit/
      bin/
        spec-kit-mcp      # MCP server binary (darwin-arm64, darwin-x86_64, linux-x86_64, linux-musl)
      plugin.toml         # declares [hooks] and [mcp] sections (see §4.3)
  bin/
    factory-dispatcher    # existing
    factory-health        # existing
    ...
  hook-plugins/
    vsdd-spec-kit-validator.wasm   # NEW: dispatcher pre-commit plugin (aggregated by build.rs)
    ...
```

### 2.4 Auto-Discovery

During the `/vsdd-factory:onboard-observability` flow (or a new `/vsdd-factory:setup-spec-kit`
onboarding step), the devops-engineer agent adds to `.claude/settings.local.json`:

```json
{
  "mcpServers": {
    "spec-kit": {
      "command": "~/.claude/plugins/cache/claude-mp/vsdd-factory/<version>/plugins/spec-kit/bin/spec-kit-mcp",
      "args": ["--factory-root", ".factory"],
      "env": {}
    }
  }
}
```

The MCP registration entry is auto-generated by the dispatcher's session-start logic from the
`[mcp]` section of `plugins/spec-kit/plugin.toml` per UNI-PLUG-001 §4 — no manual editing of
`.claude/settings.local.json` is required.

The server scans `.factory/` on startup and maintains an in-memory graph index. File-watch
events (inotify/FSEvents) keep the index current without full rescans.

---

## 3. Tool Surface

### 3.1 Read Operations

#### `spec_kit_list_artifacts`

```typescript
spec_kit_list_artifacts(params: {
  kind?: "bc" | "vp" | "adr" | "hs" | "story" | "di" | "pol" | "td" | "all"
  status?: "active" | "draft" | "retired" | "all"
  subsystem?: string   // filter by subsystem (e.g., "SS-02")
}) => Array<ArtifactRef>

type ArtifactRef = {
  id: string          // e.g., "BC-2.01.016"
  kind: ArtifactKind
  version: string     // e.g., "1.4"
  status: string      // e.g., "active"
  file_path: string   // absolute path
  title: string       // H1 heading
}
```

#### `spec_kit_get_artifact`

```typescript
spec_kit_get_artifact(params: {
  id: string          // e.g., "BC-2.01.016" or "VP-153"
}) => {
  id: string
  kind: ArtifactKind
  version: string
  frontmatter: Record<string, unknown>
  body_sections: Array<{ heading: string, content: string }>
  citations_inbound: Array<CitationRef>   // who cites this artifact
  citations_outbound: Array<CitationRef>  // who this artifact cites
  index_rows: Array<IndexRowRef>          // which INDEX files track this artifact
}

type CitationRef = {
  source_artifact_id: string
  target_artifact_id: string
  citation_form: CitationForm   // see schema section
  file_path: string
  line: number
  context: string   // surrounding text for disambiguation
}

type IndexRowRef = {
  index_file: string
  row_key: string     // the artifact's ID as it appears in the row
  current_version_in_row: string
  current_status_in_row: string
}
```

#### `spec_kit_find_citations`

```typescript
spec_kit_find_citations(params: {
  target_id: string           // e.g., "ADR-026" or "E-SPEC-012"
  form?: CitationForm | "all" // filter to specific citation form
}) => Array<{
  file: string
  line: number
  context: string
  citation_form: CitationForm
  source_artifact_id: string | null  // null if citation is in a non-artifact file
}>

type CitationForm =
  | "bare"             // ADR-026
  | "backtick"         // `ADR-026`
  | "parenthesized"    // (ADR-026)
  | "with_md"          // ADR-026.md
  | "with_hash"        // #ADR-026
  | "pin_value"        // v1.21 (the version string itself, not the ID)
  | "prose"            // "per ADR-026 D7" (prose citation detected by pattern)
```

#### `spec_kit_trace`

```typescript
spec_kit_trace(params: {
  from_id: string
  to_id: string
  max_depth?: number  // default 6
}) => Array<TracePath>

type TracePath = {
  nodes: Array<{ id: string, kind: ArtifactKind, edge_label: string }>
  length: number
}
// Example: DI-012 → BC-2.01.016 (traces_to) → VP-153 (verifies) → HS-001 (exercises)
```

#### `spec_kit_check_coverage`

```typescript
spec_kit_check_coverage(params: {
  story_id: string   // e.g., "S-PLUGIN-PREREQ-D"
}) => {
  story_id: string
  story_version: string
  ac_count: number
  task_coverage: Array<{
    ac_id: string
    ac_text: string
    tasks_covering: Array<string>    // task IDs
    has_implementation_instruction: boolean
    construction_sites: Array<string>   // e.g., ["error.rs", "spec_parser.rs"]
    definition_sites: Array<string>
  }>
  vp_coverage: Array<{ vp_id: string, ac_ids: Array<string> }>
  hs_coverage: Array<{ hs_id: string, ac_ids: Array<string> }>
  gaps: Array<Gap>
}

type Gap = {
  kind: "missing_task" | "missing_construction_site" | "missing_definition_site"
        | "missing_vp" | "missing_hs" | "missing_red_gate_test"
  ac_id: string
  description: string
  severity: "HIGH" | "MEDIUM" | "LOW"
}
```

### 3.2 Atomic Mutation Operations

These are the core value of spec-kit. Every mutation:
- Computes ALL required side-effects before writing any file
- Returns a preview (`dry_run: true`) or applies atomically (`dry_run: false`)
- Produces a structured result showing every file touched and every change made
- Enforces graph invariants — if a mutation would violate an invariant, it FAILS with a
  structured error, not a silent partial write

#### `spec_kit_bump_artifact`

The most frequently needed operation. Bumps an artifact version and propagates ALL
downstream effects.

```typescript
spec_kit_bump_artifact(params: {
  id: string              // artifact ID to bump, e.g., "ADR-026"
  change_summary: string  // human-readable description for changelog
  burst_label: string     // e.g., "FB75"
  dry_run?: boolean       // default true — preview before applying
  max_transitive_depth?: number  // default 5; iteration cap for fixed-point
}) => BumpResult

type BumpResult = {
  dry_run: boolean
  id: string
  pre_version: string
  post_version: string
  // All citation pin updates required across the workspace
  cite_pin_updates: Array<{
    file: string
    line: number
    before: string   // the stale citation text
    after: string    // the updated citation text
    citation_form: CitationForm
    artifact_id: string  // which artifact this citation refers to
  }>
  // INDEX row updates required
  index_row_updates: Array<{
    index_file: string
    row_artifact_id: string
    field: "version" | "status" | "summary_cell"
    before: string
    after: string
  }>
  // Changelog entry to append
  changelog_entry: {
    version: string
    date: string
    burst: string
    change: string
  }
  // Recursive: artifacts whose pins to THIS artifact also need bumping
  transitive_bumps: Array<TransitiveBumpResult>
  // Self-citation check (within-file self-cite, POL-29 step 8i)
  within_file_self_cites: Array<{
    file: string
    line: number
    cite_text: string
    action: "update" | "leave"   // update if stale, leave if already current
  }>
  // Fixed-point iteration log
  iteration_log: Array<{
    iteration: number
    new_bumps_found: number
    artifacts_bumped: Array<string>
  }>
}
```

**Invariant enforcement in `bump_artifact`:**
- After computing `post_version`, grep all files for all citation forms of `id` (bare,
  backtick, parenthesized, with_md, prose patterns that include a version pin)
- For each found citation that includes a version pin at `pre_version`, classify as stale
- Iterate: if any stale citation is itself inside an artifact that must then be bumped
  (e.g., a story citing ADR-026 at v1.21 — the story's version itself must bump because it
  changed), add that artifact to the transitive bump set and recurse
- Continue until `new_bumps_found == 0` (fixed point) or `max_transitive_depth` exceeded
- Check all parallel value classes: if `id` is "ADR-026 D7", also sweep "ADR-026 D3" and
  "ADR-026 D5" citation forms for independent staleness
- Check within-file self-citations (a changelog entry citing the file's own version)

#### `spec_kit_add_task`

```typescript
spec_kit_add_task(params: {
  story_id: string
  after_task: string   // insert after this task ID, e.g., "T-07b"
  content: string      // full task markdown text
  burst_label: string
  dry_run?: boolean
}) => AddTaskResult

type AddTaskResult = {
  dry_run: boolean
  task_id: string        // assigned task ID (next sequential)
  story_version_bump: { pre: string, post: string }
  // FSR table rows that must be added (one per new crate touched)
  fsr_rows_needed: Array<{
    crate: string
    suggested_row: string   // markdown table row template
    already_present: boolean
  }>
  // Token budget rows that must be added (parallel to FSR)
  token_budget_rows_needed: Array<{
    crate: string
    already_present: boolean
  }>
  // AC coverage check — does this task provide coverage for at least one AC?
  ac_coverage_check: {
    status: "pass" | "fail" | "warning"
    covered_acs: Array<string>
    uncovered_acs: Array<string>
    // If a new AC should be added to cover this task, suggest it
    suggested_ac?: string
  }
}
```

#### `spec_kit_add_invariant`

```typescript
spec_kit_add_invariant(params: {
  bc_id: string      // behavioral contract to add invariant to
  di_id: string      // domain invariant this traces to
  statement: string  // invariant statement text
  burst_label: string
  dry_run?: boolean
}) => AddInvariantResult

type AddInvariantResult = {
  dry_run: boolean
  bc_version_bump: { pre: string, post: string }
  di_update: {
    di_id: string
    di_version_bump: { pre: string, post: string }
    // DI must now list bc_id as enforcer
    enforcer_bcs_update: { before: Array<string>, after: Array<string> }
  }
  // Any subsystem summary tables that aggregate invariant counts
  summary_propagation: Array<{
    file: string
    section: string
    before: string
    after: string
  }>
  // VP coverage matrix updates (new invariant may need a new VP)
  coverage_matrix_update: {
    needs_vp: boolean
    suggested_vp_id?: string    // next available VP-NNN
    coverage_gap_description?: string
  }
}
```

### 3.3 Validation Operations

#### `spec_kit_verify_invariants`

```typescript
spec_kit_verify_invariants(params: {
  scope?: "all" | string  // story_id, burst_label, or "all"
  policies?: Array<string>  // e.g., ["POL-29", "POL-2", "POL-23"]
  staged_only?: boolean   // only check staged (git-staged) files
}) => InvariantReport

type InvariantReport = {
  pass: boolean
  violation_count: number
  violations: Array<Violation>
  policy_coverage: Array<{ policy_id: string, checks_run: number, violations: number }>
}

type Violation = {
  policy_id: string
  step: string       // e.g., "POL-29.step-8b"
  severity: "HIGH" | "MEDIUM" | "LOW"
  file: string
  line?: number
  description: string
  suggested_fix?: string
  // For citation staleness violations:
  stale_citation?: {
    artifact_id: string
    found_version: string
    canonical_version: string
    citation_form: CitationForm
  }
}
```

This single tool replaces the entirety of the POL-29 step-8 verification procedure.
The graph model knows every artifact's canonical version and every citation's form.
Staleness detection is O(edges) not O(grep-commands-manually-run-per-step).

#### `spec_kit_lint_burst`

```typescript
spec_kit_lint_burst(params: {
  staged_files: Array<string>  // absolute paths of staged files
  burst_label: string
  policies?: Array<string>     // subset of policies to run; default all
}) => LintReport

type LintReport = {
  pass: boolean
  burst_label: string
  staged_file_count: number
  artifact_touches: Array<{
    artifact_id: string
    kind: ArtifactKind
    version_before: string | null   // null if artifact is new
    version_after: string | null    // null if artifact is deleted
  }>
  passes: Array<{ check: string, description: string }>
  failures: Array<Violation>
  suggestions: Array<{ description: string, auto_fixable: boolean }>
  // POL-29 specific outputs
  citation_staleness: {
    checked_count: number
    stale_count: number
    stale_citations: Array<{
      source_file: string
      line: number
      artifact_id: string
      stale_version: string
      canonical_version: string
    }>
  }
  index_sync_check: {
    checked_index_count: number
    out_of_sync_count: number
    details: Array<{ index_file: string, artifact_id: string, issue: string }>
  }
}
```

### 3.4 Ops Layer Operations (S-15.14 amendment)

These tools extend spec-kit to OPS-layer artifacts: STATE.md, burst-log entries, and
compaction operations. They correspond to INV-011..014 and replace the TD-VSDD-096..100
prose governance rules at the tool layer.

#### `spec_kit_validate_state_md`

```typescript
spec_kit_validate_state_md(params: {
  file_path?: string        // defaults to .factory/STATE.md
  staged_content?: string   // for pre-commit validation (pass raw content from staged file)
}) => StateMdValidationResult

type StateMdValidationResult = {
  // INV-011 breakdown — all 5 must pass for overall_continue: true
  pc2_forbidden_meta: {
    passed: boolean
    matches: Array<string>   // offending phrases found, if any
  }
  pc3_index_cites: {
    passed: boolean
    present: Array<string>   // e.g., ["(BC-INDEX v3.14)", "(VP-INDEX v2.07)"]
    missing: Array<string>   // index names absent from current_step
  }
  pc4_trajectory_tail: {
    passed: boolean
    count: number            // number of arrow-separated values found
    expected: 4
    raw_tail: string         // the trajectory tail substring detected
  }
  pc5_d_chain_currency: {
    passed: boolean
    max_cited_in_current_step: number | null
    max_in_body: number | null
    delta: number | null     // max_cited - max_in_body; negative means stale
  }
  pc6_canonical_marker: {
    passed: boolean
    found: boolean
    expected_phrase: string  // the exact marker phrase from BC-5.39.006 v1.2
  }
  overall_continue: boolean  // true only if all 5 PCs pass
}
```

**Relationship to `validate-dispatch-advance` hook:** This MCP tool runs the same 5-PC
check as the S-15.14 dispatcher hook, but via the proactive MCP surface (agent-callable
before committing STATE.md). The dispatcher hook provides the unconditional PreToolUse gate.

#### `spec_kit_validate_burst_entry`

```typescript
spec_kit_validate_burst_entry(params: {
  burst_log_file: string          // path to burst-log.md, e.g., ".factory/cycles/v1.0-brownfield-backfill/burst-log.md"
  h2_heading_pattern: string      // e.g., "S-15.14-pass-10-fix-burst" or a regex
}) => BurstEntryValidationResult

type BurstEntryValidationResult = {
  h2_heading_matched: string
  // INV-012: canonical block completeness
  canonical_block_count: number
  canonical_blocks_present: Array<CanonicalBlockKind>
  canonical_blocks_missing: Array<CanonicalBlockKind>
  inv_012_passed: boolean
  // INV-013: Dim attestation evidence-form validity
  dim_attestations: {
    dim2: DimEvidenceCheck
    dim5: DimEvidenceCheck
    dim6: DimEvidenceCheck
    dim7: DimEvidenceCheck
  }
  inv_013_passed: boolean  // true only if all present Dim blocks are LiteralShellWithStdout
  overall_passed: boolean
}

type DimEvidenceCheck = {
  present: boolean
  evidence_form: EvidenceForm | null   // null if block absent
  violations: Array<string>            // human-readable description of each form violation
}
```

#### `spec_kit_compact_state_md`

```typescript
spec_kit_compact_state_md(params: {
  archive_row_predicates: Array<string>  // glob-like patterns matching Phase Progress rows to archive
  preservation_paths: Array<string>      // paths that will preserve the archived content
  dry_run?: boolean                      // default true
}) => CompactionResult

type CompactionResult = {
  dry_run: boolean
  rows_to_archive: number
  // INV-014(a): preservation path verification
  preservation_paths_verified: Array<{
    path: string
    exists: boolean
    content_match: boolean   // true if archived rows appear verbatim in the file
  }>
  // INV-014(b): Active Branches SHA advance
  active_branches_sha_advance: {
    factory_artifacts_row_before: string
    factory_artifacts_row_after: string | null   // null if dry_run
  }
  // INV-014(c): Concurrent Cycles header advance
  concurrent_cycles_header_advance: {
    before: string
    after: string | null   // null if dry_run
  }
  // INV-014(d): summary label accuracy
  summary_labels_validated: Array<{
    field: string     // e.g., "trend_cardinality", "row_range"
    claimed: string   // what the compaction prose claimed
    actual: string    // what the source data actually shows
    match: boolean
  }>
  inv_014_passed: boolean
  overall_passed: boolean
}
```

---

## 4. Integration with factory-dispatcher

### 4.1 Architecture Options Analysis

**Option A: MCP replaces dispatcher hooks**
MCP server hosts all validation; dispatcher hooks call out to it.

Rejected. The dispatcher hook runtime is WASM (synchronous, sandboxed, no async I/O). An MCP
server call over JSON-RPC requires a socket/stdio channel — not available in the WASM sandbox.
Even if a host function were added to bridge the call, the round-trip latency would add 50-200ms
per hook invocation, degrading the pre-commit experience. More critically: MCP server availability
is conditional (server must be running), while dispatcher hooks must be unconditionally reliable.

**Option C: MCP-only (no dispatcher integration)**
Agents call MCP for mutations; no dispatcher enforcement.

Rejected. Agents can and do bypass MCP by using the Edit/Write tools directly. The prism
session that generated this proposal used Edit/Write throughout because MCP wasn't available.
Without a defensive gate, agents operating without MCP access (or forgetting to use it) will
accumulate exactly the same graph integrity failures that caused FB53-FB75.

**Option B: Shared validator library (RECOMMENDED)**

Extract all validation logic into a shared Rust library (`vsdd-spec-kit-core`). Both the MCP
server binary and the dispatcher's WASM hook plugin link against this library. The invariants
are defined once in the library; both surfaces enforce them.

Division of labor:
- **MCP (proactive):** agents call during spec authoring; mutations are atomic and invariant-safe
- **Dispatcher (defensive):** pre-commit hook runs `lint_burst()` via library FFI; catches any
  direct Edit/Write that bypassed MCP

This is the only option where:
1. The invariant logic lives in exactly ONE place (library)
2. Enforcement is unconditional (dispatcher gate cannot be bypassed by Edit tool)
3. The MCP provides the ergonomic, high-value authoring experience
4. No circular dependency: library has no dependency on MCP or dispatcher

### 4.2 Architectural Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                   vsdd-spec-kit-core                         │
│  (Rust library — graph schema, validators, mutators)         │
│                                                              │
│  SpecGraph::load(factory_root: &Path) -> Result<SpecGraph>  │
│  SpecGraph::bump_artifact(id, summary) -> BumpResult         │
│  SpecGraph::lint_burst(files) -> LintReport                  │
│  SpecGraph::verify_invariants(scope) -> InvariantReport      │
│  ... (full tool surface as library methods)                  │
└──────────────────┬──────────────────────┬───────────────────┘
                   │                      │
        link (native binary)    link (wasm32-wasip1 target)
                   │                      │
   ┌───────────────▼──────┐  ┌────────────▼──────────────────┐
   │    spec-kit-mcp      │  │  vsdd-spec-kit-validator.wasm  │
   │  (proactive surface) │  │  (defensive surface)           │
   │                      │  │                                │
   │  JSON-RPC 2.0/stdio  │  │  PreToolUse hook               │
   │  MCP protocol        │  │  Registered in hooks-registry  │
   │  Agent-callable      │  │  Fires before every Write/Edit │
   │  Stateful graph idx  │  │  to .factory/ files            │
   │  File-watch updates  │  │  Calls lint_burst(staged)      │
   └──────────────────────┘  └────────────────────────────────┘

Agent authoring flow:          Direct-edit fallback flow:
  spec_kit_bump_artifact()  →    agent uses Edit tool directly
  returns BumpResult         →    dispatcher fires PreToolUse
  agent applies changes      →    hook calls lint_burst()
  graph integrity maintained      invariant violations BLOCK commit
```

### 4.3 Dispatcher Hook Registration

Declare in `plugins/spec-kit/plugin.toml` `[hooks]` and `[mcp]` sections; vsdd-factory's
`build.rs` aggregates into `plugins-registry.toml`; dispatcher session-start auto-registers
MCP in project's `.claude/settings.local.json` per UNI-PLUG-001 §4.

The `plugin.toml` `[hooks]` entry for the spec-kit validator:

```toml
[[hooks]]
name = "vsdd-spec-kit-validator"
event = "PreToolUse"
plugin = "hook-plugins/vsdd-spec-kit-validator.wasm"
priority = 500  # UNI-PLUG-001 plugin band (500+); core hooks use 20-460
timeout_ms = 3000
on_error = "continue"   # degraded mode: if spec-kit unavailable, continue with warning

[hooks.filter]
tool_names = ["Write", "Edit"]
path_pattern = "**/.factory/**/*.md"

[hooks.capabilities.read_file]
path_allow = [".factory/**/*.md", ".factory/policies.yaml"]

[hooks.config]
# Paths to scan for invariant checking (relative to factory_root)
artifact_paths = [
  "specs/behavioral-contracts/",
  "specs/verification-properties/",
  "specs/domain-spec/",
  "specs/architecture/",
  "stories/",
  "policies.yaml"
]
# Which policies to enforce at pre-commit time (HIGH severity only for blocking)
enforced_policies = ["POL-1", "POL-2", "POL-7", "POL-9", "POL-22", "POL-23", "POL-25", "POL-29"]
# Block on HIGH violations; warn on MEDIUM
block_severity = "HIGH"
```

The hook plugin reads the file being written, parses it as a spec artifact (if it is one),
compares against the graph state loaded from `.factory/`, and calls `lint_burst()` to check
invariants. If `lint_burst()` returns any HIGH violations, the hook blocks the write with a
structured error message identifying the specific invariant failure.

This is architecturally identical to the existing `validate-stable-anchors.wasm` hook
(which blocks writes containing volatile line-number citations). The new hook is a sibling
plugin with broader scope: citation staleness, index sync, transitive closure completeness.

### 4.4 Precursor Hook: validate-dispatch-advance (S-15.14)

vsdd-factory S-15.14 ships `validate-dispatch-advance` as a per-class precursor to the
broader spec-kit dispatcher enforcement. The hook lives at:

```
crates/hook-plugins/validate-dispatch-advance/src/lib.rs
```

and fires as PreToolUse for writes to STATE.md, validating the `current_step:` frontmatter
value against all 5 BC-5.39.006 v1.2 PCs (INV-011 at the per-class level). After S-15.14
merges to develop, this hook is registered at priority 154 in hooks-registry.toml and
validated against the production STATE.md at deploy time — all 5 PCs pass.

**Relationship to spec-kit Tier 2 dispatcher hook:**

When `vsdd-spec-kit-validator.wasm` lands (Tier 2), the architect adjudicates whether:

- **Option A — Migrate:** `validate-dispatch-advance` is migrated under spec-kit (its INV-011
  logic subsumed by `vsdd-spec-kit-core`; priority moves to the 500+ plugin band). The
  per-class hook is retired; spec-kit becomes the single enforcement surface.
- **Option B — Defense-in-depth sibling:** `validate-dispatch-advance` remains at priority 154
  as a specialized early-gate for STATE.md writes; spec-kit at priority 500+ runs the broader
  spec-layer invariants. Both fire on `.factory/**/*.md` writes; each enforces its own
  invariant class.

This is architectural: Option A reduces surface area; Option B provides defense-in-depth.
The decision is deferred to implementation time per the architect's adjudication authority.
Option B is marginally safer during the transition (if spec-kit Tier 2 has an early-stage
false-positive rate, the `validate-dispatch-advance` hook keeps INV-011 enforced). Option A
is cleaner long-term. Recommendation: ship Option B at rc.19; migrate to Option A at rc.20
after false-positive rate is empirically measured.

---

## 5. Schema Definition

The graph schema defines the entities, their fields, invariants, and citation forms.
Schema versions replace POL amendments: when a new integrity class is discovered, the schema
version increments and the validator is updated — no prose amendment needed.

### 5.1 Core Entity Types

```rust
// vsdd-spec-kit-core/src/schema.rs

/// Artifact kinds in the VSDD spec graph.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ArtifactKind {
    BehavioralContract,   // BC-M.NN.NNN
    VerificationProperty, // VP-NNN
    DomainInvariant,      // DI-NNN
    Capability,           // CAP-NNN
    HoldoutScenario,      // HS-NNN
    Story,                // S-NNN or S-SLUG
    ArchDecisionRecord,   // ADR-NNN
    Policy,               // POL-NN
    TechDebt,             // TD-VSDD-NNN
    ErrorCode,            // E-SPEC-NNN, E-SENSOR-NNN, E-QUERY-NNN
    Index,                // BC-INDEX, VP-INDEX, STORY-INDEX, ARCH-INDEX

    // OPS-LAYER kinds (S-15.14 amendment — vsdd-factory empirical basis):
    /// STATE.md — pipeline state with current_step BC PC compliance (INV-011).
    OpsState,
    /// h2 sections in cycles/<cycle>/burst-log.md — each section is a structured entry
    /// with mandatory canonical blocks and Dim attestations (INV-012, INV-013).
    BurstLogEntry,
    /// D-NNN entries in cycles/<cycle>/decision-log.md — structured decision records
    /// with mandatory schema parallel to BC frontmatter (gap: no typed invariant today).
    DecisionLogEntry,
    /// PG-* / L-EDP1-NNN lessons in cycles/<cycle>/lessons.md — lesson records.
    LessonEntry,
    /// cycles/<cycle>/INDEX.md — adversary-pass rows + Convergence Status row.
    /// Distinct from spec INDEXes: has adversary-pass rows and h2-section semantics.
    CycleIndex,
}

/// A parsed spec artifact with all structural information extracted.
#[derive(Debug, Clone)]
pub struct Artifact {
    pub id: String,
    pub kind: ArtifactKind,
    pub version: semver::Version,  // parsed from frontmatter `version:` field
    pub status: ArtifactStatus,    // active / draft / retired
    pub file_path: PathBuf,
    pub frontmatter: serde_yaml::Value,
    pub body_sections: Vec<BodySection>,
    /// All citations found in this artifact's body (outbound edges)
    pub outbound_citations: Vec<Citation>,
}

#[derive(Debug, Clone)]
pub struct BodySection {
    pub heading: String,
    pub level: u8,      // 1=H1, 2=H2, etc.
    pub content: String,
}

/// A directed citation edge in the spec graph.
#[derive(Debug, Clone)]
pub struct Citation {
    pub source_artifact_id: String,
    pub target_artifact_id: String,
    pub target_version_pin: Option<String>,  // the pinned version if present
    pub form: CitationForm,
    pub file_path: PathBuf,
    pub line: usize,
    pub context: String,
}

// ── OPS-LAYER entity types (S-15.14 amendment) ──────────────────────────────

/// Parsed STATE.md current_step value with BC-5.39.006 v1.2 PC compliance fields.
/// Testable: `cargo test -- inv_011` runs all 5 PC checks against a real STATE.md snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateMdCurrentStep {
    /// The raw string value of the `current_step:` frontmatter field.
    pub raw_value: String,
    /// PC2: no forbidden meta-commentary phrases present (e.g., "Note:", "Observation:").
    pub pc2_forbidden_meta_present: bool,
    /// PC3: all 4 index cites present — (BC-INDEX vX.Y), (VP-INDEX vX.Y), (STORY-INDEX vX.Y), (ARCH-INDEX vX.Y).
    pub pc3_index_cites: Vec<(String, String)>,   // (index_name, version_string)
    /// PC4: trajectory tail has exactly 4 arrow-separated values (`→N→N→N→N`).
    pub pc4_trajectory_tail_length: usize,
    /// PC5: maximum D-NNN referenced in current_step does not exceed max D-NNN in decision-log body.
    pub pc5_max_cited_d_nnn: Option<u32>,
    /// PC6: canonical marker phrase present (exact string per BC-5.39.006 v1.2).
    pub pc6_marker_present: bool,
    /// For PC5 cross-check: maximum D-NNN found anywhere in STATE.md body.
    pub d_chain_max_in_body: Option<u32>,
}

/// Parsed h2 section from cycles/<cycle>/burst-log.md.
/// Testable: `cargo test -- inv_012` parses a burst-log snapshot and verifies all blocks present.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BurstLogEntryData {
    pub h2_heading: String,         // e.g., "S-15.14-pass-10-fix-burst"
    pub commit_sha: String,
    /// D-444(c) canonical blocks present in this entry.
    pub canonical_blocks: BTreeMap<CanonicalBlockKind, BodySection>,
    /// Dim attestation blocks (Dim-2, Dim-5, Dim-6, Dim-7).
    pub dim_attestations: BTreeMap<u8, DimAttestation>,
    /// Finding IDs listed in the Closes block.
    pub closes_findings: Vec<String>,
    /// SHAs listed in the Factory-artifacts commits block.
    pub factory_artifacts_commits: Vec<String>,
}

/// Enumeration of the 8 D-444(c) canonical blocks required in every burst-log h2 section.
/// Testable: `cargo test -- canonical_block_kind_coverage` verifies all 8 variants are checked.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum CanonicalBlockKind {
    ParentCommit,
    AdversaryVerdict,
    FilesTouched,
    Codifications,
    Dim2Attestation,
    Dim5Attestation,
    Dim6Attestation,
    Dim7Attestation,
    Closes,
    FactoryArtifactsCommits,
}

/// Evidence form classification for a Dim attestation block.
/// INV-013 requires LiteralShellWithStdout; other forms are violations.
/// Testable: `cargo test -- inv_013_evidence_form` classifies test fixtures for each form.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvidenceForm {
    /// PASS: contains a literal shell command invocation AND captured stdout lines from
    /// a production artifact. Example: `grep -oE 'D-[0-9]+' .factory/STATE.md | tail -1`
    /// followed by actual stdout output.
    LiteralShellWithStdout,
    /// FAIL: paraphrases what the shell output would show without running the command.
    /// Example: "D-chain max in STATE.md is D-449 (verified by inspection)."
    NarrativeParaphrase,
    /// FAIL: runs `echo` or `printf` on a hardcoded synthetic string rather than reading
    /// the production artifact. Example: `echo "D-449" | grep -oE 'D-[0-9]+'`.
    SyntheticEcho,
    /// FAIL: contains `[...]` placeholder brackets with no actual command or output.
    PlaceholderBrackets,
}

/// A Dim attestation block with its evidence-form classification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DimAttestation {
    pub block_index: u8,           // 2, 5, 6, or 7
    pub evidence_form: EvidenceForm,
    pub content: String,           // raw block text for reporting
}

/// ── END OPS-LAYER entity types ───────────────────────────────────────────────

/// Enumeration of all known citation surface forms.
/// This replaces the per-value-class variant-form registries in POL-29 step 3a.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CitationForm {
    /// BC-2.01.016 (bare, no decoration)
    Bare,
    /// `BC-2.01.016` (backtick-quoted in markdown)
    Backtick,
    /// (BC-2.01.016) (parenthesized)
    Parenthesized,
    /// BC-2.01.016.md (with .md extension)
    WithMdExtension,
    /// #BC-2.01.016 (hash-prefixed, used in link anchors)
    WithHash,
    /// v1.21 (version pin only, associated with a parent artifact by context)
    PinValue { associated_id: String },
    /// "per ADR-026 D7 pin" (prose, detected by known patterns)
    Prose { pattern: String },
}
```

### 5.2 Graph Invariants

These invariants replace the 11 POL-29 amendments. Each is a typed predicate, not a prose rule:

```rust
// vsdd-spec-kit-core/src/invariants.rs

/// INV-001: No stale version pins.
/// For every Citation with a version pin, the pinned version must equal
/// the target artifact's current canonical version.
pub struct NoCitationStaleness;

/// INV-002: No orphan INDEX rows.
/// For every row in an INDEX file, an artifact with the cited ID must exist.
/// For every artifact with status=active, an INDEX row must exist.
pub struct IndexRowCompleteness;

/// INV-003: INDEX row summary cell consistency.
/// The version field in each INDEX row must match the artifact's current version.
/// The status field in each INDEX row must match the artifact's current status.
/// (Distinguishes row-existence from row-accuracy — POL-29 step 8f gap.)
pub struct IndexRowAccuracy;

/// INV-004: Parallel value class coverage.
/// If an artifact has multiple named value classes (e.g., ADR-026 D3/D5/D7),
/// a citation sweep for one class name MUST include all sibling class names.
/// (POL-29 step 8g gap.)
pub struct ParallelValueClassCoverage;

/// INV-005: Transitive closure completeness.
/// If artifact A cites artifact B at version V, and B has since reached
/// version V+N, then A must be bumped to reflect its dependency on V+N.
/// The fixed-point condition: bump(A) may require bump(C) if C cites A's version.
/// (POL-29 step 8b + 8d + 8e gap.)
pub struct TransitiveClosureCompleteness;

/// INV-006: Within-file self-citation consistency.
/// If an artifact's changelog cites its own version (e.g., "v1.4→v1.5 in FB75"),
/// the self-citation must refer to the artifact's current canonical version.
/// (POL-29 step 8i gap.)
pub struct WithinFileSelfCiteConsistency;

/// INV-007: Story structural completeness.
/// For every story:
///   (a) Every AC must have at least one task covering it
///   (b) Every crate in crates_touched must have an FSR row
///   (c) Every crate in crates_touched must have a token budget row
///   (d) Every AC with a Red Gate test must have an implementation instruction in tasks
///   (e) Every AC with a new type/enum must have a definition-site task instruction
/// (POL-29 steps 3d + 3e + 3e-refinement gaps.)
pub struct StoryStructuralCompleteness;

/// INV-008: DI-to-BC coverage.
/// Every active DI-NNN must be cited by at least one active BC's Traceability section.
/// (POL-2 as a graph invariant.)
pub struct DomainInvariantCoverage;

/// INV-009: Error code canonical form.
/// Error code citations (E-SPEC-NNN, E-SENSOR-NNN, E-QUERY-NNN) must use the
/// exact canonical form defined in the error-taxonomy.md, not aliases or variants.
/// (POL-29 step 3a(a) gap.)
pub struct ErrorCodeCanonicalForm;

/// INV-010: Version monotonicity.
/// Artifact versions must be monotonically increasing within each artifact's history.
/// A changelog entry for version V must follow all entries for versions < V.
pub struct VersionMonotonicity;

// ── OPS-LAYER invariants (S-15.14 amendment) ────────────────────────────────

/// INV-011: STATE.md current_step BC PC compliance.
/// STATE.md `current_step:` frontmatter value must simultaneously satisfy all 5
/// BC-5.39.006 v1.2 postconditions:
///   PC2: no forbidden meta-commentary phrases present
///   PC3: all 4 index cites present ((BC-INDEX vX.Y), (VP-INDEX vX.Y), (STORY-INDEX vX.Y), (ARCH-INDEX vX.Y))
///   PC4: trajectory tail has exactly 4 arrow-separated values
///   PC5: max cited D-NNN in current_step ≤ max D-NNN in decision-log body
///   PC6: canonical marker phrase present
/// Replaces TD-VSDD-097-EXTENDED (dispatch-template completeness prose rule).
/// The `validate-dispatch-advance` hook (S-15.14, priority 154) is the per-class precursor
/// that validated this invariant at deploy time against the production STATE.md.
/// Testable: `cargo test -- inv_011` reads a real STATE.md fixture and asserts all 5 PCs pass.
pub struct StateMdCurrentStepCompliance;

/// INV-012: Burst-log entry structural completeness.
/// Every h2 section in cycles/<cycle>/burst-log.md must contain all 10 D-444(c)
/// canonical block kinds (ParentCommit / AdversaryVerdict / FilesTouched / Codifications /
/// Dim2Attestation / Dim5Attestation / Dim6Attestation / Dim7Attestation / Closes /
/// FactoryArtifactsCommits) as distinct, non-empty sub-sections.
/// Replaces TD-VSDD-099 (own-burst-log structural-integrity prose rule).
/// Testable: `cargo test -- inv_012` parses a burst-log snapshot and asserts
/// `canonical_blocks.len() == 10` for every h2 section.
pub struct BurstLogEntryStructuralCompleteness;

/// INV-013: Dim-2 attestation evidence-form validity (combined TD-VSDD-096 + TD-VSDD-100).
/// Every Dim-2 (and by extension Dim-5, Dim-6, Dim-7) attestation block in burst-log entries
/// must classify as `EvidenceForm::LiteralShellWithStdout`. Specifically:
///   (a) The block must contain at least one literal shell command invocation (detected by
///       presence of `$`, `|`, ` grep`, ` diff`, ` printf`, or equivalent shell syntax)
///   (b) The block must contain captured stdout lines that originate from a PRODUCTION artifact
///       file path (not from `echo`-synthesized input or `[...]` placeholder brackets)
/// Replaces TD-VSDD-096 (literal-evidence stdout rule) and TD-VSDD-100 (production-artifact
/// read requirement) as a single combined invariant.
/// Testable: `cargo test -- inv_013_shell_command` confirms command detection regex;
///           `cargo test -- inv_013_synthetic_echo` confirms synthetic-echo rejection;
///           `cargo test -- inv_013_placeholder` confirms bracket-placeholder rejection.
pub struct DimAttestationEvidenceFormValidity;

/// INV-014: Compaction operation sibling-sweep completeness.
/// When a STATE.md compaction burst archives Phase Progress rows, the burst MUST:
///   (a) Verify every cited preservation path exists via literal shell (INV-013 subsumes form)
///   (b) Advance the Active Branches factory-artifacts row SHA to the current burst HEAD
///   (c) Advance the Concurrent Cycles Status bolded-summary header to reflect the current pass
///   (d) Verify compaction summary labels (trend cardinality, row range) match source data
///       exactly — no approximation, no rounding
/// Replaces TD-VSDD-098 (compaction-burst sibling-sweep prose rule).
/// Testable: `cargo test -- inv_014_preservation_paths` reads a compaction fixture and verifies
/// path existence; `cargo test -- inv_014_sha_advance` verifies SHA field was updated;
/// `cargo test -- inv_014_label_accuracy` compares claimed vs actual row count.
pub struct CompactionOperationSiblingSweep;
```

### 5.3 Index Row Schema

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexRow {
    /// Which index file this row lives in
    pub index_file: PathBuf,
    /// The artifact ID this row represents
    pub artifact_id: String,
    /// The version as recorded in the index row
    pub indexed_version: String,
    /// The status as recorded in the index row
    pub indexed_status: String,
    /// The full markdown table row text (for editing)
    pub raw_row: String,
    /// The line number in the index file
    pub line: usize,
    /// Summary cell text (the human-readable description column)
    pub summary_cell: String,
}
```

### 5.4 Task Schema

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub story_id: String,
    pub task_id: String,      // e.g., "T-07b"
    pub content: String,
    pub crates_touched: Vec<String>,
    pub has_fsr_row: bool,
    pub has_token_budget_row: bool,
    pub covered_acs: Vec<String>,
    pub construction_sites: Vec<String>,   // files where types are defined
    pub definition_sites: Vec<String>,     // files where types are implemented
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcceptanceCriterion {
    pub story_id: String,
    pub ac_id: String,          // e.g., "AC-3"
    pub text: String,
    pub traces_to: Vec<AcTrace>,
    pub red_gate_test_id: Option<String>,
    pub has_implementation_instruction: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcTrace {
    pub bc_id: Option<String>,
    pub error_code: Option<String>,    // E-SPEC-NNN, etc.
    pub rule_id: Option<String>,       // POL-NN, DI-NNN, etc.
    pub invariant_id: Option<String>,  // INV-AUTH-OPEN-NNN, etc.
}
```

### 5.5 Schema Migration

When a new integrity class is discovered (equivalent to a POL-29 amendment today), the
schema version increments:

```toml
# vsdd-spec-kit-core/src/schema_version.toml
schema_version = "1.0.0"
# Invariants enforced at this schema version:
# INV-001 through INV-010 as above
```

Migration scripts (in `vsdd-spec-kit-core/src/migrations/`) transform existing artifact
files to be compliant with the new schema version. This replaces the current pattern where:
1. A defect is found
2. POL is amended
3. A fix-burst is dispatched to bring existing artifacts into compliance
4. The fix-burst may itself introduce new violations

Under the schema migration approach:
1. A new invariant class is discovered
2. Schema version increments
3. Migration script runs against `.factory/` to bring all artifacts into compliance
4. Graph index reloads; `verify_invariants()` now enforces the new invariant

---

## 6. Implementation Roadmap

### Tier 1: MVP Read-Only (~1-2 days)

Goal: Prove the graph model works against an existing project's `.factory/`. No mutations.
No dispatcher integration. Read-only MCP server that demonstrates value immediately.

**Crate structure:**
```
vsdd-factory/crates/
  vsdd-spec-kit-core/        # NEW: shared library
    src/
      lib.rs
      schema.rs              # Artifact, Citation, IndexRow, Task types
      parser/
        mod.rs
        frontmatter.rs       # YAML frontmatter extraction
        body.rs              # Section parsing, citation extraction
        citation_forms.rs    # CitationForm detection regexes
      graph.rs               # SpecGraph: in-memory graph, query methods
      invariants/
        mod.rs
        inv_001_no_staleness.rs
        inv_002_index_completeness.rs
        ...through inv_010
    Cargo.toml
  spec-kit-mcp/              # NEW: MCP server binary
    src/
      main.rs                # stdio JSON-RPC loop
      tools/
        list_artifacts.rs
        get_artifact.rs
        find_citations.rs
        trace.rs
        check_coverage.rs
    Cargo.toml
```

**Proof point:** Run `spec_kit_list_artifacts()` against prism's `.factory/` — should return
all ~150 story specs, ~160 VP files, ~290 BC files. Run `spec_kit_verify_invariants()` —
should surface the same stale citations that POL-29 step-8 sweeps currently catch.

**Acceptance criteria for Tier 1:**
- `spec_kit_list_artifacts()` returns complete artifact inventory matching BC-INDEX + VP-INDEX
  + STORY-INDEX counts
- `spec_kit_find_citations()` for "ADR-026" returns all forms (bare, backtick, prose "D7 pin")
  without requiring per-variant grep commands
- `spec_kit_verify_invariants()` surfaces all INV-001 violations present in the prism
  `.factory/` at the time of the run, with no false positives
- MCP server starts in < 500ms against prism's `.factory/` on an M-series Mac
- MCP server re-indexes on file change within 200ms

**Workspace integration:** Add to `[workspace]` in `Cargo.toml`:
```toml
members = [
    ...existing members...,
    "crates/vsdd-spec-kit-core",
    "crates/spec-kit-mcp",
]
```

### Tier 2: Atomic Mutations + Dispatcher Pre-Commit (~1 week)

Goal: Replace the manual POL-29 step-8 procedure with MCP mutations. Add dispatcher hook
plugin for defensive enforcement.

**New crate:**
```
vsdd-factory/crates/
  hook-plugins/vsdd-spec-kit-validator/    # NEW: dispatcher hook plugin
    src/
      lib.rs                               # WASM hook entry point
      validator.rs                         # calls vsdd_spec_kit_core::lint_burst()
    Cargo.toml                             # target: wasm32-wasip1
```

**MCP additions (in spec-kit-mcp/src/tools/):**
- `bump_artifact.rs` — implements `spec_kit_bump_artifact()` with full BumpResult
- `add_task.rs` — implements `spec_kit_add_task()` with FSR + token-budget checks
- `add_invariant.rs` — implements `spec_kit_add_invariant()` with DI + coverage matrix
- `lint_burst.rs` — implements `spec_kit_lint_burst()` (also available to dispatcher via lib)

**Key implementation detail for `bump_artifact()`:**
The transitive closure fixed-point iteration (INV-005 enforcement) is the most complex
algorithm. Implementation sketch:

```rust
// vsdd-spec-kit-core/src/mutators/bump.rs
pub fn bump_artifact(graph: &mut SpecGraph, id: &str, summary: &str) -> BumpResult {
    let mut iteration = 0;
    let mut all_bumps = vec![BumpedArtifact::primary(id)];
    let mut worklist = vec![id.to_string()];

    while !worklist.is_empty() && iteration < MAX_ITERATIONS {
        iteration += 1;
        let this_round = std::mem::take(&mut worklist);
        let mut new_bumps = 0;

        for bumped_id in &this_round {
            // Find all artifacts that have a version pin citation to bumped_id
            let inbound = graph.citations_to(bumped_id);
            for citation in inbound {
                if citation.target_version_pin.as_deref() == Some(&graph.pre_version(bumped_id)) {
                    let source_id = &citation.source_artifact_id;
                    if !all_bumps.iter().any(|b| &b.id == source_id) {
                        // This artifact must also be bumped (it pins a stale version)
                        all_bumps.push(BumpedArtifact::transitive(source_id, bumped_id));
                        worklist.push(source_id.clone());
                        new_bumps += 1;
                    }
                }
            }
        }
        if new_bumps == 0 { break; }
    }
    // Compute all file mutations from all_bumps; return as BumpResult (dry_run)
    // Apply if !dry_run
    BumpResult { all_bumps, iteration_log: ..., ... }
}
```

**Acceptance criteria for Tier 2:**
- `spec_kit_bump_artifact("ADR-026", "v1.28→v1.29 step 8j amendment", "FB76")` in dry_run
  mode returns all 21+ stale citation sites that the prism session's FB62 step-8b sweep
  found, without any manual grep
- `spec_kit_bump_artifact()` in apply mode produces a git diff that passes
  `spec_kit_verify_invariants()` with no INV-001, INV-003, INV-004, or INV-005 violations
- `vsdd-spec-kit-validator.wasm` blocks a direct Edit to a BC file that would leave a stale
  ADR version pin, with error message identifying the exact stale citation
- `vsdd-spec-kit-validator.wasm` does NOT block correct edits (no false positives in
  the prism `.factory/` test corpus)
- POL-29 step-8 procedure is no longer required in state-manager agent prompts for citation
  pin updates (superseded by `spec_kit_bump_artifact()` or `lint_burst()` block)

### Tier 3: Full Schema Enforcement + Auto-Derivation (~2-3 weeks)

Goal: Zero remaining prose-rule enforcement for graph integrity. All invariants mechanical.
INDEX rows auto-derived. Changelog auto-generated. POL-29 simplified to a single rule.

**Features:**

**INDEX auto-derivation:** Instead of manually updating BC-INDEX, VP-INDEX, STORY-INDEX, the
spec-kit graph is the source of truth for index state. Index files become generated artifacts:

```typescript
spec_kit_sync_indexes(params: {
  dry_run?: boolean
}) => IndexSyncResult {
  indexes_updated: Array<{
    index_file: string
    rows_added: number
    rows_updated: number
    rows_removed: number   // artifacts retired or deleted
  }>
}
```

Index sync runs as a post-mutation step in `bump_artifact()`, `add_task()`, and
`add_invariant()`. Human-authored text in INDEXes (section prose, preamble) is preserved;
only table rows are generated.

**Changelog auto-generation:** The `changelog_entry` field in `BumpResult` becomes
auto-appended to the artifact's `## Changelog` section with no manual step:

```
| v1.29 | 2026-05-18 | FB76 | D-698 | Bumped ADR-026 D7 pin to v1.29 per spec-kit bump cascade |
```

**Burst-label auto-assignment:** The current session's burst label is tracked by the MCP
server's session context (set at startup or via a `spec_kit_begin_burst()` call). All
mutations in the session inherit the burst label automatically, eliminating the drift risk
identified in FB74 (burst label derivation from context, which was itself a source of
inconsistency).

**Policy simplification:** After Tier 3, POL-29 is replaced by:

```yaml
- id: 29
  name: within_fb_sibling_sweep_discipline
  description: "All spec graph mutations must go through spec-kit MCP tools or be validated
    by the vsdd-spec-kit-validator dispatcher hook. Direct file edits that violate INV-001
    through INV-010 are blocked at pre-commit time."
  adopted: FB76-spec-kit-migration
  severity: HIGH
  enforced_by: [spec-kit-mcp, vsdd-spec-kit-validator-hook]
  scope: [bc, vp, adr, hs, story, di, pol, td]
  lint_hook: "hook-plugins/vsdd-spec-kit-validator.wasm"
  verification_steps:
    - "Run spec_kit_verify_invariants() — zero violations is the pass condition"
```

The 11-amendment prose rule chain (5 step-3a classes + 9 step-8 substeps) is archived in
the policy changelog as historical record and superseded by the schema-enforced invariants.

**Migration for existing projects:**

```
Phase 1: spec-kit MCP added to vsdd-factory plugin package (ships in rc.19+)
Phase 2: Existing project agents continue using grep/Edit; dispatcher pre-commit
         lint_burst() now runs as new hook plugin
Phase 3: Agent prompts updated: "Prefer spec_kit_bump_artifact() over manual grep+Edit
         for version pin updates"
Phase 4: POL-29 v1.28 → simplified single policy. Old amendment chain archived.
Phase 5: spec_kit_sync_indexes() becomes the INDEX update mechanism; manual index
         editing deprecated
```

For prism specifically:
- Run `spec_kit_verify_invariants(scope="all")` as baseline — documents current INV state
- Run migration scripts for any INV-007 (story structural completeness) violations
- spec-kit becomes the authoring tool for all subsequent fix-bursts
- First fix-burst using spec-kit (FB76) should complete in 1-3 bursts vs the 23-burst
  FB53-FB75 session

---

## 7. Migration Path for Existing Projects

### 7.1 Zero-Disruption Migration

The dispatcher hook (`vsdd-spec-kit-validator.wasm`) is the ONLY change that affects existing
agent workflows. It fires on PreToolUse for writes to `.factory/**/*.md`. On HIGH violations,
it blocks the write with an actionable error. On MEDIUM violations, it produces warnings.

The critical design decision: the hook uses `on_error = "continue"` (same as existing hooks
like `capture-commit-activity`). If the spec-kit WASM plugin panics or fails to load, the
write continues. This ensures spec-kit adoption does not create a reliability dependency —
spec-kit supplements, never gates.

### 7.2 Rollout Sequence (vsdd-factory rc.19)

1. **rc.19 ships:** `plugins/spec-kit/` directory with `spec-kit-mcp` binary +
   `vsdd-spec-kit-validator.wasm` in plugin package; declare in
   `plugins/spec-kit/plugin.toml` `[hooks]` and `[mcp]` sections; vsdd-factory's `build.rs`
   aggregates into `plugins-registry.toml`; dispatcher session-start auto-registers MCP in
   project's `.claude/settings.local.json` per UNI-PLUG-001 §4.
2. **onboarding skill updated:** `/vsdd-factory:setup` triggers the UNI-PLUG-001 dispatcher
   session-start auto-registration flow; no manual `.claude/settings.local.json` editing
   required.
3. **`block_severity = "MEDIUM"` for the first release** (warn-only; accumulate false-positive
   data before hardening to HIGH-block); configured via `plugin.toml` `[hooks.config]` section.
4. **rc.20 target:** Based on real-project telemetry (false positive rate, block frequency),
   harden `block_severity = "HIGH"` for INV-001, INV-003, INV-005 (the three highest-ROI
   invariants per FB53-FB75 evidence)

### 7.3 Agent Prompt Updates

The state-manager agent prompt currently contains extensive POL-29 step-8 verification
procedure text. After Tier 2 ships, this section is replaced with:

```
## Citation Pin Updates (POL-29)

Use spec-kit MCP for all version pin updates:
  spec_kit_bump_artifact(id, summary, burst_label, dry_run=true)
→ Review BumpResult: all stale citations listed with exact file+line
  spec_kit_bump_artifact(id, summary, burst_label, dry_run=false)
→ Apply: all citations updated atomically

Manual grep for citation staleness is DEPRECATED. The spec-kit graph
model knows all citation forms and invariants mechanically.
```

---

## 8. Cost-Benefit Analysis

### 8.1 Engineering Cost

| Tier | Scope | Estimated Effort |
|------|-------|-----------------|
| Tier 1: MVP read-only | `vsdd-spec-kit-core` crate + `spec-kit-mcp` binary (read tools only) | 1-2 days |
| Tier 2: Mutations + dispatcher | Mutation tools + `vsdd-spec-kit-validator.wasm` | 1 week |
| Tier 3: Full enforcement | INDEX auto-derivation, changelog auto-gen, policy migration | 2-3 weeks |
| **Total** | All tiers | **~1 month** |

These estimates assume familiarity with the vsdd-factory codebase (hook-sdk patterns,
hooks-registry.toml conventions, WASM target build). The existing `validate-stable-anchors`
hook plugin is the closest architectural template and is ~400 lines of Rust.

### 8.2 Cost Avoided

**Per-session cascade overhead (prism baseline):**
- FB53-FB75: 23 fix-bursts × ~30-45 min/burst = 11-17 hours wall-clock
- Adversary passes: 23 passes × ~20 min/pass = 8 hours
- Human review: 23 review cycles × ~10 min each = 4 hours
- **Total: ~23-29 hours for this session's cascade overhead**

**With spec-kit (estimated):**
- Same 23 POL-29 related findings would not surface as findings at all (blocked at authoring
  time by MCP invariants or pre-commit by dispatcher hook)
- Residual cascade work: substantive spec defects unrelated to graph integrity (~3-5 bursts)
- **Total: ~3-7 hours** (estimated 75-85% reduction in cascade overhead for graph-integrity class)

**ROI:** ~20-25 hours saved per project per major cascade session. At vsdd-factory's current
scale (prism is one project; vsdd-factory itself has comparable cascade patterns in its own
brownfield sessions), Tier 1+2 pay for themselves within the first project cascade session
after shipping.

### 8.3 Quality Risk Reduction

F-LP78/79/80/81 all reached Phase 3 implementation before being caught. F-LP79 specifically
(AC-to-Task implementation-instruction coverage gap) caused validators E-SPEC-012/013/014 to
be absent from the story Tasks section — confirmed by codebase grep. Under spec-kit:

- F-LP78 class (structural table completeness) → blocked by INV-007(b/c)
- F-LP79 class (AC-to-Task coverage) → blocked by INV-007(a/d)
- F-LP80 class (construction vs definition site) → blocked by INV-007(e)
- F-LP81 class (version pin staleness via transitive closure) → blocked by INV-005

These findings would not reach Phase 3 implementation because they are enforced at spec
authoring time (MCP) and pre-commit time (dispatcher hook), not discovered at adversary
review time.

---

## 9. Risks and Mitigations

### Risk 1: Schema Format Lock-In

**Description:** Committing to a typed schema makes spec format evolution harder. A future
spec format change (e.g., new frontmatter fields, new citation forms) requires a schema
migration.

**Mitigation:**
- Schema versioning with explicit migration scripts (see Section 5.5)
- `serde(deny_unknown_fields = false)` on frontmatter parsing — unknown fields are preserved,
  not rejected. Schema evolution is additive-first.
- Schema version is stored in `vsdd-spec-kit-core/schema_version.toml`; mismatches produce
  a clear error with migration instructions, not silent data loss.
- Assessment: this risk is LOWER than the alternative (prose rules that cannot be mechanically
  applied); schema evolution is a known engineering problem with known solutions.

### Risk 2: MCP Server Availability

**Description:** If `spec-kit-mcp` is not running or crashes, agents fall back to direct
Edit/Write, bypassing invariant enforcement at the MCP layer.

**Mitigation:**
- The dispatcher hook provides a defensive gate that does NOT depend on MCP availability.
  It is compiled WASM, always loaded by the dispatcher, and runs on every `.factory/*.md`
  write regardless of MCP server state.
- MCP supplements the authoring experience; dispatcher protects integrity unconditionally.
- `on_error = "continue"` on the dispatcher hook means a WASM plugin crash does not block
  all spec writes. The risk of a crash → integrity failure is accepted as a tail-risk;
  telemetry (session-end-telemetry hook) catches WASM plugin crash rates.

### Risk 3: Agents Bypass MCP via Edit Tool

**Description:** An agent authoring spec content with Edit/Write directly (not via MCP)
could produce graph integrity violations that only surface at the dispatcher pre-commit gate,
not at mutation time.

**Mitigation:**
- This is the "degraded path" that the dispatcher hook defends against. The experience is:
  agent writes → dispatcher fires → invariant violation reported → agent must fix before
  write succeeds.
- This is strictly better than the current state (no enforcement; violation discovered by
  adversary N passes later).
- As agent prompts are updated to use MCP tools, the bypass rate decreases. The dispatcher
  hook ensures bypass is never silent.

### Risk 4: False Positives Block Legitimate Writes

**Description:** Overly strict invariant enforcement blocks correct spec mutations, degrading
agent productivity.

**Mitigation:**
- Tier 2 ships with `block_severity = "MEDIUM"` (warn-only for all) during the ramp period.
  Telemetry accumulates false-positive evidence.
- `block_severity = "HIGH"` only hardens INV-001, INV-003, INV-005 in rc.20 after false
  positive rates are empirically measured.
- `spec_kit_lint_burst()` result includes `suggestions` (non-blocking) and `failures`
  (blocking) as separate lists.
- Exemption zones: same pattern as `validate-stable-anchors` (§Changelog sections, code
  fences, historical sections exempt from certain checks).

### Risk 5: WASM Target Build Complexity

**Description:** `vsdd-spec-kit-core` must compile to both native (for the MCP binary) and
`wasm32-wasip1` (for the dispatcher hook plugin). Some Rust crates do not compile cleanly
to WASM.

**Mitigation:**
- `vsdd-spec-kit-core` will use the same dependency discipline as existing hook plugins:
  no tokio (sync file I/O only), no native-only system calls, no complex proc-macro crates.
- The existing hook plugins (e.g., `validate-stable-anchors`) demonstrate the pattern:
  pure Rust with `serde_json`, `regex`, `serde_yaml` — all WASM-compatible.
- `serde_yaml` has known WASM issues in some versions; use `serde_yml` (WASM-clean fork)
  or pre-parse YAML in the native binary layer and pass structured data to the shared library.
- File I/O in the WASM hook plugin uses host functions (as existing hooks do) rather than
  direct `std::fs`. The `vsdd-spec-kit-core` graph loading function takes a
  `impl Fn(&Path) -> Result<String>` reader closure, making it testable and WASM-adaptable.

---

## 10. References

### Empirical Evidence Sources

**Prism spec-layer cascade (first empirical basis):**

- **POL-29 v1.14→v1.28 amendment chain:** `/Users/jmagady/Dev/prism/.factory/policies.yaml`
  `metadata.changelog` section, versions 1.14 through 1.28.
- **Session cascade log:** prism `SESSION-HANDOFF.md` §Resume Snapshot 2026-05-17
  (FB53-FB75 burst sequence; finding IDs F-LP65-001 through F-LP87-002).
- **Adversary OBS-LP87 growth-complexity asymptote flag:** policies.yaml v1.28 changelog
  note: "11th POL-29 amendment this session. NOTE: POL-29 now at 5 classes + 9 step-8
  substeps — adversary OBS-LP87 flagged growth-complexity asymptote concern."
- **F-LP78/79/80/81 engineering impact:** prism `SESSION-HANDOFF.md` §RESUME SNAPSHOT,
  finding closure details for passes 78-81.

**vsdd-factory S-15.14 ops-layer cascade (second empirical basis):**

- **Lesson records (TD-VSDD-095..100):** `.factory/cycles/v1.0-brownfield-backfill/lessons.md`
- **Pass reports:** `.factory/code-delivery/S-15.14/adv-local-pass-{1..11}.md`
  (trajectory: 16→9→8→2→0→1→1→0→4→1→2; best streak 1/3 twice; never 3/3)
- **BC under validation:** `.factory/specs/behavioral-contracts/ss-05/BC-5.39.006.md` (v1.3)
- **Proof of dispatcher-hook architecture pattern:**
  `crates/hook-plugins/validate-dispatch-advance/src/lib.rs`
  (working per-class enforcement at priority 154; all 5 BC-5.39.006 v1.2 PCs PASS at deploy
  against the production STATE.md; validates that typed-invariant dispatcher enforcement
  works at the per-artifact-class level — spec-kit generalizes this to all artifact classes)

### vsdd-factory Architecture References

- **factory-dispatcher architecture:** `hooks-registry.toml` (v1.0.0-rc.18, 28 plugins,
  21 native-WASM + 7 legacy-bash-adapter)
- **Hook plugin template:** `crates/hook-plugins/validate-stable-anchors/src/lib.rs`
  (nearest architectural analog: PreToolUse, path-filtered, structured violation reporting)
- **Hook SDK:** `crates/hook-sdk/src/lib.rs` (HookPayload, HookResult, host:: FFI)
- **WASM target conventions:** `rust-toolchain.toml` (wasm32-wasip1 cross-compile target)

### Protocol and Standards

- **MCP protocol specification:** https://modelcontextprotocol.io
  (JSON-RPC 2.0 over stdio; tool registration format; error codes)
- **Existing MCP integrations in vsdd-factory:** Perplexity, Context7, Tavily (registered
  in `.claude/settings.local.json` of factory-consuming projects)

### Design Analogues

- **Language server protocol (LSP):** The read-query tool surface (`spec_kit_get_artifact`,
  `spec_kit_find_citations`, `spec_kit_trace`) is structurally analogous to LSP's
  `textDocument/references`, `textDocument/definition`, and `workspace/symbol` operations.
  The spec artifact graph is analogous to a codebase symbol graph.
- **Database constraint enforcement vs application-layer validation:** The dispatcher hook
  is analogous to a database constraint (enforced at write time, unconditional); the MCP
  mutation tools are analogous to an ORM (provide ergonomic, invariant-safe interface over
  the raw write surface). Both are necessary; neither alone is sufficient.

---

## Appendix A: State Tracking Recommendation

**Does vsdd-factory's `.factory/STATE.md` need an entry for this proposal?**

Yes. The proposal is a significant architectural addition to vsdd-factory's own roadmap.
Recommended `current_step` update:

```
SK-MCP-001 proposal authored 2026-05-17 by architect dispatch. Tier 1 implementation
gated on human review + approval. Proposal at .factory/proposals/spec-kit-mcp.md.
Next: human reviews SK-MCP-001 → approves → story-writer authors S-SK-001 (Tier 1
vsdd-spec-kit-core crate + spec-kit-mcp binary, read-only tools).
```

The proposal does NOT need its own STATE.md decision row yet — that comes when the human
approves the proposal and the first implementation story is commissioned. At that point,
a D-NNN decision row recording the approval + Tier 1 story authoring mandate should be
added by the state-manager.

**STATE.md update is NOT required before this proposal is useful.** The proposal is
self-contained and can be acted on by a future engineering session without any additional
STATE.md context. The state-manager should update STATE.md as part of the approval
decision burst, not as part of proposal authoring.

---

## Appendix B: Quick-Start for Future Engineering Session

If you are picking this up to implement, here is the minimal context needed:

1. **The problem in one sentence:** VSDD spec artifacts form a knowledge graph, and every
   graph mutation (version bump, citation update, INDEX row sync) has transitive effects that
   are currently enforced by accumulating prose rules in POL-29 — rules that have not
   converged after 11 amendments in one session.

2. **The solution in one sentence:** Replace prose rules with a typed Rust graph model
   (`vsdd-spec-kit-core`) that enforces invariants at mutation time (via MCP tools) and
   at pre-commit time (via dispatcher WASM hook).

3. **Start here for Tier 1:** Create `crates/vsdd-spec-kit-core/` and
   `crates/spec-kit-mcp/`, implement `SpecGraph::load()` + `spec_kit_list_artifacts()`.
   Test against prism's `.factory/` (copy or use as a file path). The graph model is the
   hard part; MCP wire-up is straightforward once the model works.

4. **The existing template to follow:** `crates/hook-plugins/validate-stable-anchors/`
   is the closest architectural analog. Read it first. The hook plugin for Tier 2 is
   essentially this hook with a much richer invariant checker replacing the single
   volatile-line-number regex.

5. **The single most important invariant to get right:** INV-005 (transitive closure
   completeness with fixed-point iteration). This invariant is what POL-29 steps 8b, 8d,
   and 8e are all trying to approximate in prose. Get the fixed-point loop right and the
   other invariants are straightforward.

6. **Do not start with Tier 3.** INDEX auto-derivation changes the authoring model
   significantly. Validate Tier 1 and Tier 2 against real project cascades before
   committing to auto-derivation.

7. **For OPS-layer invariants (INV-011..014):** The `validate-dispatch-advance` hook
   (`crates/hook-plugins/validate-dispatch-advance/src/lib.rs`) is the direct implementation
   reference for INV-011. Read it before writing the spec-kit OPS-layer validators — the
   5-PC check logic is already proven; spec-kit's job is to call it through the shared
   library interface rather than as a standalone hook.

---

## Appendix C: Schema Migration Reference

_(Content unchanged from original §5.5 — moved here for navigation consistency.)_

See §5.5 Schema Migration above.

---

## Appendix D: TD-VSDD-095..100 → INV-NNN Mapping (S-15.14 amendment)

This appendix records the explicit traceability from vsdd-factory's S-15.14 TD-VSDD
lessons to the spec-kit invariants that close them. It serves as the authoritative
mapping for future implementers deciding which invariants to prioritize.

| TD-VSDD lesson | Meta class (S-15.14) | Artifact layer | INV closes it | Notes |
|----------------|---------------------|----------------|---------------|-------|
| TD-VSDD-095 | TDD micro-commit discipline (implementer must not combine unrelated changes in one commit) | Implementer commits | **OUT OF SCOPE** | Implementer-side discipline; spec-kit governs `.factory/` artifact writes, not git commit granularity. No INV assigned. |
| TD-VSDD-096 | Literal-evidence stdout in burst-log Dim-2 attestations (replaces D-449(a) prose with structural gate) | burst-log.md | **INV-013** (Dim attestation evidence-form validity, part a: literal shell command detection) | Combined with TD-VSDD-100 into single invariant. |
| TD-VSDD-097-EXTENDED | orchestrator dispatch templates for STATE.md current_step writes MUST satisfy ALL 5 BC-5.39.006 v1.2 PCs simultaneously | STATE.md current_step | **INV-011** (StateMdCurrentStepCompliance) | The `validate-dispatch-advance` hook (S-15.14, priority 154) is the per-class precursor implementation. |
| TD-VSDD-098 | orchestrator compaction-burst sibling-sweep (cited preservation paths exist + Active Branches SHA advance + Concurrent Cycles header advance + label accuracy) | STATE.md compaction ops | **INV-014** (CompactionOperationSiblingSweep) | Four sub-clauses map to INV-014(a)/(b)/(c)/(d) respectively. |
| TD-VSDD-099 | burst-log own-entry structural integrity (4 Dim attestation blocks + 8 D-444(c) canonical blocks) | burst-log.md | **INV-012** (BurstLogEntryStructuralCompleteness) | Note: canonical block count is 10 in the spec-kit model (8 D-444(c) blocks + 2 additional Dim blocks not in the original count); see CanonicalBlockKind enum. |
| TD-VSDD-100 | Dim-2 PC attestations MUST read production artifact (not synthetic `echo` input) | burst-log.md | **INV-013** (Dim attestation evidence-form validity, part b: production-artifact requirement) | Combined with TD-VSDD-096. The `EvidenceForm::SyntheticEcho` variant specifically models this failure mode. |

**Coverage summary:**
- 5 of 6 TD-VSDD lessons → 4 INV-NNN invariants (some lessons merged, TD-VSDD-096+100 → INV-013)
- 1 of 6 lessons (TD-VSDD-095) is explicitly out of scope for spec-kit
- INV-011..014 are additive to INV-001..010; they extend the graph model to OPS artifacts
  without changing spec-layer invariant semantics

**Open question for implementation:** The `validate-dispatch-advance` hook enforces INV-011
at priority 154 (core hook band, `migration_source` exempt). When spec-kit Tier 2 ships,
the architect adjudicates Option A (migrate under spec-kit) vs Option B (defense-in-depth
sibling) per §4.4. This choice determines whether INV-011 has one or two enforcement paths.
