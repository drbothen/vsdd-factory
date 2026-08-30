---
document_type: epic
level: L3
traces_to: .factory/stories/STORY-INDEX.md
epic_id: "E-23"
version: "v1.1"
status: draft
title: "ADR-045 Phase A — Stable-Anchor Cross-Reference Migration: thin new-pin guard, Tier-1 INDEX-resolution, pre-commit normalization codemod, mechanical corpus migration (156 files / 6 document classes), and the Phase-B residual-leakage measurement gate"
prd_capabilities: []
subsystems_affected: [SS-01, SS-04, SS-05, SS-07]
target_release: "v1.0.0-rc.26+ (wave-9; exact rc TBD at release planning)"
story_count: 14
producer: story-writer
timestamp: "2026-08-25T00:00:00Z"
phase: brownfield-backfill
cycle: v1.0-brownfield-backfill
depends_on: []
inputs:
  - .factory/specs/architecture/decisions/ADR-045-stable-anchor-cross-reference-architecture.md
  - .factory/research/adr-045-independent-validation.md
  - .factory/research/wave7-xref-consistency-research.md
  - .factory/stories/S-21.13-read-file-range-targeted-bc-index-row-lookup.md
  - .factory/stories/S-15.03-index-cite-refresh-hook.md
  - crates/hook-plugins/validate-stable-anchors/src/lib.rs
input-hash: "e9ecaa5"
last_amended: "2026-08-30 (v1.1) — frontmatter normalization: add level: L3 (story-writer spec-hygiene sweep)"
modified:
  - "2026-08-25 (v1.0) — initial authoring (story-writer): 14 stories across 5 build stories (S-23.01..S-23.05), 8 mechanical-migration-execution stories (S-23.06..S-23.13), and 1 Phase-B measurement gate story (S-23.14). No BCs exist yet for this scope — all stories carry `behavioral_contracts: []` with the pending-PO-authorship marker per Spec-First Gate (BC-8.30.001); ACs trace to ADR-045 Decision/Migration-Plan clauses instead. Handoff to product-owner for BC authorship is itemized in the epic's Behavioral Contract Gap section below."
---

# Epic E-23: ADR-045 Phase A — Stable-Anchor Cross-Reference Migration

## Description

ADR-045 v1.2 was ratified by the human on 2026-08-25 (see ADR-045 frontmatter `modified:`
top entry). Ratification authorizes exactly the **Tier 1 unit** — stable anchors, INDEX-
mechanical version resolution, and a **thin new-pin guard** — for immediate build-and-migrate
(ADR-045 Migration Plan, Phase A). Tiers 2 and 3 (Doorstop fingerprint/suspect-link model;
full AST-based residual validator) are ratified as *design direction only* and are explicitly
**not** authorized for build by this epic — their build is gated on the Phase-B measurement
this epic's final story produces (ADR-045 §Decision preamble; §Migration Plan Phase B/C).

E-23 exists to close the root cause of the Wave-7 pre-TDD convergence floor (STATE.md D-1069
through D-1080; five of six wave-7 stories held at 0/3 CLEAN by a self-regenerating
version-pin-propagation failure class, per ADR-045 §Context). It delivers:

1. A shared **live-section classifier + newline-normalized, anchor-interposed-tolerant
   pin-pattern detector** (S-23.01) — the common detection substrate for both the codemod and
   the guard, defeating Wave-7 failure modes 1 (line-wrapped cites) and 2 (anchor-interposed
   cites) by construction.
2. A **pre-commit normalization codemod** (S-23.02) that mechanically strips load-bearing
   inline version tokens from cross-references, converting them to stable-anchor form.
3. A **thin new-pin guard** WASM PreToolUse hook (S-23.03) that blocks any newly-introduced
   or newly-stale load-bearing inline version pin in a live section, preventing regression
   during and after migration.
4. A **Tier-1 INDEX-resolution mechanism** (S-23.04) that mechanically resolves a stable ID's
   current version from BC-INDEX / ARCH-INDEX / STORY-INDEX / VP-INDEX, extending the
   `read_file_range` targeted-row-lookup precedent S-21.13 established for BC-INDEX to the
   other three registries.
5. **POLICY 7/8/14/17/19 enforcement wiring** (S-23.05) — the `validate-cross-site-correspondence`
   (BC-5.39.010) plugin and sibling code-enforced parity paths are extended to check
   anchor-presence-in-INDEX instead of inline-version-token equality, so the amended policy
   semantics (applied to `policies.yaml` separately by state-manager at the ratification burst,
   per ADR-045 §Policy Amendments Required) have working machinery underneath them.
6. **Mechanical corpus migration** (S-23.06..S-23.13) — one atomic commit per document class
   per TD-VSDD-053, prioritizing the 4 INDEX files (51% of the ~3,074 live pins) as the
   highest-value/highest-risk first commits, closing with the story-files document class
   commit that **closes the Wave-7 residual version-pin tail** (S-21.19/20/21/23 stragglers).
7. **Phase-B residual-leakage measurement** (S-23.14) — the gate that decides whether ADR-045
   Phase C (Tiers 2/3) proceeds. Not a build story; an instrumentation-and-measurement story.

## Trigger / Motivation

ADR-045 v1.2, §Status: "**PROPOSED. Not ratified. Not implemented.**" was superseded by the
human ratification recorded in ADR-045's own `modified:` changelog top entry
(2026-08-25, v1.2): "Phase A ... RATIFIED by human at the standard human-approval gate for
governance-policy/spec amendment ... Status proposed→accepted." Per ADR-045 §Migration Plan,
"A dedicated epic is recommended because the migration spans every document class in
`.factory/` and requires coordinated dispatches across state-manager, consistency-validator,
and story-writer roles ... Suggested anchor: S-15.03 PRIORITY-A ... or a new wave-9 epic."
E-23 is that new wave-9 epic (S-15.03 remains its own narrower-scoped story — ARCH-INDEX
cite-refresh + lessons retroactive-sweep — and is referenced as precedent infrastructure,
not subsumed).

## Epic Placement Justification

E-23 is the next free epic ID under POLICY 1 (append-only numbering): E-20 is a pre-existing
registry gap, E-21 and E-22 are the two most recently allocated IDs (E-22 dissolved but its
file retained), so E-23 is the next unallocated slot at authoring time (2026-08-25).

**Why a dedicated epic, not folded into E-21 or S-15.03:** E-21 (Factory State Data-Loss
Hardening) is a fixed-scope, already-largely-merged collection of factory artifact write-path
defect fixes; ADR-045 is a distinct architectural elimination of an entire cross-reference
class, spanning 156 files across 6 document classes with its own build-then-migrate-then-measure
phase structure. S-15.03 (ARCH-INDEX cite-refresh hook) is a narrower, already-scoped-and-draft
story that enforces cite *freshness* against a single index (ARCH-INDEX); it is real precedent
for "mechanical enforcement beats prose codification" but does not cover the other three INDEX
registries, the codemod, the guard, or the corpus migration. E-23 is chartered specifically for
the ADR-045 Phase A build-and-migrate-and-measure unit.

## PRD Capabilities Covered

E-23 introduces no new PRD capabilities; it is process/governance-tooling infrastructure that
eliminates a structural defect class in the factory's own spec-authoring discipline. No CAP-NNN
row is added.

| Capability ID | Note |
|--------------|------|
| (none) | E-23 is governance/tooling infrastructure; no new PRD capabilities |

## Behavioral Contract Gap (handoff to product-owner)

No behavioral contracts exist yet for ADR-045 Tier-1 capabilities. Per the Spec-First Gate
(BC-8.30.001) and this epic's story-writer mandate, every story below carries
`behavioral_contracts: []` with a `# BC status: pending PO authorship` frontmatter comment and
`status: draft` (never `ready`) until product-owner authors and anchors real
`BC-S.SS.NNN`-form contracts. Acceptance criteria trace instead to specific ADR-045 clauses
(`ADR-045 §Decision <tier>`, `ADR-045 §Migration Plan Phase A step N`) as an interim,
inspectable trace target. **Product-owner handoff — new BCs needed:**

| Proposed BC anchor point | Subsystem | Scope |
|---------------------------|-----------|-------|
| Live-section classifier + pin detector semantics (5 failure-mode-defeating behaviors) | SS-01 | S-23.01 |
| Pre-commit normalization codemod idempotence + exemption-zone correctness | SS-01 | S-23.02 |
| Thin new-pin guard block/allow decision function | SS-04 | S-23.03 |
| Tier-1 INDEX-resolution mechanism (4-registry targeted lookup) | SS-01 | S-23.04 |
| `validate-cross-site-correspondence` INDEX-resolution-based parity (supersedes/extends BC-5.39.010's own version-leg checks) | SS-01 | S-23.05 |
| Migration invariant: post-migration corpus has zero live load-bearing inline version pins outside permitted-exception forms | SS-05 | S-23.06..S-23.13 |
| Phase-B measurement window semantics + Phase-C gate decision rule | SS-05 | S-23.14 |

Until these land, every story's `behavioral_contracts:` array stays `[]` and status stays
`draft`; a story cannot be promoted to `status: ready` under BC-8.30.001 postcondition 3 while
that array is empty.

## Stories

| Story ID | Title | Wave | Points | Depends On | Traces To |
|----------|-------|------|--------|------------|-----------|
| S-23.01 | Live-section classifier + newline-normalized pin-pattern detector (shared foundation) | 9 | 8 | — | ADR-045 §Decision (Tier 1); §Context failure modes 1–3 |
| S-23.02 | Pre-commit normalization codemod | 9 | 8 | S-23.01 | ADR-045 §Migration Plan Phase A step 2; §Alternatives Considered (codemod-class guard) |
| S-23.03 | Thin new-pin guard (WASM PreToolUse hook) | 9 | 8 | S-23.01, S-23.04 | ADR-045 §Decision preamble item 1; §Migration Plan Phase A step 2 |
| S-23.04 | Tier-1 INDEX-resolution mechanism (4-registry targeted lookup) | 9 | 13 | — | ADR-045 §Decision, Tier 1 ("Version resolution operates mechanically..."); §Migration Plan Phase A step 3 |
| S-23.05 | POLICY 7/8/14/17/19 enforcement wiring (`validate-cross-site-correspondence` INDEX-resolution extension) | 9 | 8 | S-23.04, S-23.02 | ADR-045 §Policy Amendments Required table; §Migration Plan Phase A step 3 |
| S-23.06 | Corpus migration: BC-INDEX.md (421 pins) | 10 | 5 | S-23.02, S-23.03 | ADR-045 §Migration Plan Phase A step 5; §Consequences Negative/Trade-offs (INDEX concentration) |
| S-23.07 | Corpus migration: ARCH-INDEX.md (287 pins) + newline-normalized ADR anchor-interposed pass (~30–40 pins) | 10 | 8 | S-23.02, S-23.03 | ADR-045 §Migration Plan Phase A step 5 (newline-normalized pass clause) |
| S-23.08 | Corpus migration: STORY-INDEX.md (597 pins) | 10 | 8 | S-23.02, S-23.03 | ADR-045 §Migration Plan Phase A step 5 |
| S-23.09 | Corpus migration: VP-INDEX.md (268 pins) | 10 | 3 | S-23.02, S-23.03 | ADR-045 §Migration Plan Phase A step 5 |
| S-23.10 | Corpus migration: story files document class (73 files) — **closes Wave-7 residual tail** | 10 | 13 | S-23.02, S-23.03, S-23.06, S-23.08 | ADR-045 §Migration Plan Phase A step 5 + step 6 (Wave-7 tail closure) |
| S-23.11 | Corpus migration: behavioral-contracts files document class (46 files) | 10 | 8 | S-23.02, S-23.03, S-23.06 | ADR-045 §Migration Plan Phase A step 5 |
| S-23.12 | Corpus migration: architecture files document class (26 files, excludes ARCH-INDEX.md) | 10 | 5 | S-23.02, S-23.03, S-23.07 | ADR-045 §Migration Plan Phase A step 5 |
| S-23.13 | Corpus migration: VP files (8) + domain-spec files (2) + PRD (1) document classes | 10 | 3 | S-23.02, S-23.03, S-23.09 | ADR-045 §Migration Plan Phase A step 5 |
| S-23.14 | Phase-B residual-leakage measurement (gates Phase C) | 11 | 5 | S-23.03, S-23.05, S-23.06..S-23.13 | ADR-045 §Migration Plan Phase B step 7; §Decision preamble item 2 |

**Total:** 14 stories, 103 story points, 3 waves (9 build, 10 migration-execution, 11 measurement).

## Dependency Graph (topological, acyclic)

```
Wave 9  (build):
  S-23.01 ──┬──> S-23.02 ──┐
            └──> S-23.03 <─┤ (S-23.03 also depends on S-23.04)
  S-23.04 ──┬──> S-23.03   │
            └──> S-23.05 <─┘ (S-23.05 also depends on S-23.02)

Wave 10 (migration execution; each depends on S-23.02 codemod tool + S-23.03 guard being live):
  S-23.06 (BC-INDEX)     ─┐
  S-23.07 (ARCH-INDEX)    ├─> S-23.10 (story files; also depends on S-23.08)
  S-23.08 (STORY-INDEX)  ─┘
  S-23.09 (VP-INDEX)     ──> S-23.13 (VP+domain-spec+PRD files)
  S-23.06 ──> S-23.11 (BC files)
  S-23.07 ──> S-23.12 (architecture files)

Wave 11 (measurement):
  {S-23.03, S-23.05, S-23.06..S-23.13} ──> S-23.14 (Phase-B measurement)
```

No cycle exists: every edge points from a lower-wave or same-wave-earlier-in-topological-order
story to a later one; S-23.14 is a sink with no outgoing edges within this epic.

## Forbidden Dependencies (epic-wide)

- No story in this epic may depend on Tier 2 (fingerprint/suspect-link sidecar) or Tier 3
  (full AST-based residual validator) infrastructure — neither is authorized for build under
  this ratification. If an implementer finds a genuine need for either during S-23.01..S-23.14,
  that is a signal to stop and route to the orchestrator/architect, not to build ahead of the
  Phase-B gate.
- The migration-execution stories (S-23.06..S-23.13) MUST NOT introduce a new inline version
  pin anywhere they touch, even transiently within a single commit — the thin new-pin guard
  (S-23.03) enforces this mechanically once live, but the codemod (S-23.02) itself must also
  be idempotent and never regress a file it has already migrated.
- No migration-execution story may modify `.factory/policies.yaml`, `.factory/STATE.md`, or
  ADR-045 itself — those are state-manager's and architect's exclusive write paths per CLAUDE.md
  Pipeline Authority and Forbidden Patterns.

## Acceptance Criteria (Epic Acceptance Criteria — EAC)

| ID | Criterion | Validation Method | Status |
|----|-----------|--------------------| -------|
| EAC-001 | All 14 stories merged to `develop` | STORY-INDEX shows all 14 stories `status: complete` | Not started |
| EAC-002 | Zero live load-bearing inline version pins remain in the 156-file / 6-document-class corpus scanned by ADR-045's corpus scan, outside the Tier-1 permitted-exception forms | Full-corpus grep for `\b(ADR-\d+\|BC-\d+(?:\.\d+)+\|S-\d+\.\d+\|VP-\d+)\b[^\n]{0,80}\bv\d+(?:\.\d+)*\b` restricted to live sections (exempt-zone-excluded) returns zero matches | Not started |
| EAC-003 | Thin new-pin guard is live in `hooks-registry.toml` and blocks a synthetic newly-introduced stale pin in a live section | Integration test: attempt to Write a `.factory/` file containing a stale inline version pin; hook blocks with `exit_code=2` | Not started |
| EAC-004 | `validate-cross-site-correspondence` resolves versions via INDEX lookup rather than requiring an inline version token in the citing document | Integration test: BC-table cell citing a stable BC ID with no inline version passes the parity gate when the ID is present in BC-INDEX | Not started |
| EAC-005 | Wave-7 residual version-pin tail (S-21.19/20/21/23 stragglers) is closed | Adversary pass on S-21.19/20/21/23 post-S-23.10 finds zero version-pin-class findings | Not started |
| EAC-006 | Phase-B measurement window is instrumented and reports a residual-leakage count | S-23.14 measurement report exists with a numeric residual-leakage count and a Phase-C go/no-go recommendation | Not started |

## Known Future Scope (out of this epic)

- ADR-045 Tiers 2 (Doorstop fingerprint/suspect-link) and 3 (full AST-based residual validator)
  — gated on S-23.14's Phase-B measurement; a separate follow-up epic if triggered.
- The `policies.yaml` POLICY 7/8/14/17/19 amendments themselves (state-manager's exclusive
  write path; applied at the ratification burst per ADR-045 §Policy Amendments Required, not
  by any story in this epic).
