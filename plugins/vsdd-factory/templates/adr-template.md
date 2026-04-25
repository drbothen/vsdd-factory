---
document_type: adr
adr_id: ADR-NNN
status: proposed
date: YYYY-MM-DD
subsystems_affected: []
supersedes: null
superseded_by: null
---

# ADR-NNN: [Decision Title]

> **One-per-file:** Each architectural decision lives in its own file.
> Filename convention: `ADR-NNN-<short-name>.md` (e.g., `ADR-001-rust-dispatcher.md`)
> ADR IDs are sequential 3-digit (`ADR-001`, `ADR-002`, ...). Once issued, never renumber.
> Lifecycle: `proposed` -> `accepted` -> (optional) `superseded` or `deprecated`.
> Frontmatter `subsystems_affected` is an array of `SS-NN` identifiers from ARCH-INDEX
> Subsystem Registry. `supersedes` / `superseded_by` link to other ADR IDs (e.g., `ADR-007`).

## Context

[2-5 paragraphs] Background, forces driving the decision, prior art, and constraints.
What problem are we solving? What's the state of the world that makes this decision necessary now?
Cite source documents (master design, code lines, BCs, prior ADRs) inline.

## Decision

[1-3 paragraphs] The architectural choice itself. Single, declarative, unambiguous.
"We will [do X] using [approach Y] because [primary rationale]."
Avoid hedging ("We might..."), avoid scope-bleed ("We will also...").
If multiple choices were combined, list them as a numbered set.

## Rationale

[2-5 paragraphs] Why this decision over the alternatives considered.
What evidence, requirement, or constraint forces this choice?
Cite specific BCs, NFRs, or invariants that this decision satisfies.

## Consequences

What this decision causes downstream. Use sub-headings:

### Positive

- [Bullet] Benefit 1, with measurable or observable outcome
- [Bullet] Benefit 2

### Negative / Trade-offs

- [Bullet] Cost 1 — what we give up by choosing this path
- [Bullet] Cost 2 — known risks, performance implications, complexity additions

### Status as of [version-or-date]

[1-2 sentences] Is this decision in-effect, partially-implemented, deferred, or rejected?
What evidence (commit, test, BC traceability) shows the status?

## Alternatives Considered

For each alternative not chosen, state:

- **Option [name]:** [1-line description] — Rejected because [reason].

Include at least the top 2-3 alternatives. "We didn't consider alternatives" is rarely true; surface what was rejected so future readers understand the path-not-taken.

## Source / Origin

Provenance for the decision. Cite at least one of:

- **Master design doc** with file:line (e.g., `legacy-design-docs/X-design.md §3.2`)
- **PRD section** if formalized later (e.g., `specs/PRD.md §Decision 4.1`)
- **Behavioral contracts** that imply or enforce this decision (`BC-S.SS.NNN`)
- **Code as-built** if extracted from existing implementation (`crates/<crate>/src/<file>.rs:<line>`)
- **Discussion** (issue, PR, design review meeting date) — link or reference

Brownfield ADRs (extracted from existing code) MUST cite the implementation evidence so reviewers can verify the decision matches what shipped.
