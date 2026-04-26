---
document_type: adr
adr_id: ADR-013
status: accepted
date: 2026-04-26
subsystems_affected: [SS-05, SS-06]
supersedes: null
superseded_by: null
---

# ADR-013: Cycle-Keyed Adversarial Review Structure

## Context

VSDD includes a mandatory adversarial review phase (Phase 1d) in which a separate
adversary agent challenges the spec package produced by the architect and product
owner. The adversarial review produces findings (CRITICAL, HIGH, MEDIUM, LOW), which
are applied as fixes, and the process repeats until convergence (3 consecutive passes
at NITPICK level, meaning no findings above LOW severity).

Before this ADR was established, adversarial review artifacts had no canonical
storage location or naming convention. During the v1.0-brownfield-backfill cycle —
the first formal VSDD cycle run against vsdd-factory itself — a storage convention
was needed immediately to support the live 6-pass adversarial review that ran on
2026-04-25.

The convention needed to satisfy three requirements: (1) multiple passes per cycle
must be distinguishable by filename; (2) review artifacts must be scoped to the
cycle that produced them, since different cycles review different spec surfaces;
(3) the convergence trajectory (pass-by-pass finding counts and severity) must be
readable as a progression without opening individual files.

## Decision

Adversarial review passes are stored at `.factory/cycles/<cycle-key>/adversarial-reviews/pass-N.md`,
where `<cycle-key>` is the kebab-case cycle identifier (e.g., `v1.0-brownfield-backfill`)
and `N` is the sequential pass number within that cycle. The cycle's `INDEX.md` file
maintains an Adversarial Reviews table summarizing each pass: date, finding count by
severity, and convergence status. A cycle converges when three consecutive passes are
classified as NITPICK (no CRITICAL, HIGH, or substantive MEDIUM findings). The
`CONVERGENCE_REACHED` marker in the INDEX.md table is the authoritative convergence
signal read by the wave-gate skill.

## Rationale

The cycle-keyed structure was established during the v1.0-brownfield-backfill cycle,
which completed 6 adversarial review passes before converging. The convergence
trajectory was:

| Pass | Findings | Classification |
|------|----------|---------------|
| 1 | 17 (1 CRIT + 7 HIGH + 6 MED + 3 LOW) | Substantive |
| 2 | 11 (1 CRIT + 4 HIGH + 4 MED + 2 LOW) | Substantive |
| 3 | 9 (2 HIGH + 5 MED + 2 LOW) | Substantive |
| 4 | 6 (1 MED + 5 LOW) | NITPICK |
| 5 | 4 (4 LOW) | NITPICK |
| 6 | 4 (4 LOW) | NITPICK — CONVERGENCE_REACHED |

The path `cycles/<cycle-key>/adversarial-reviews/pass-N.md` was chosen over
alternatives for three reasons:

First, cycle-keyed scoping prevents pass numbering collision across cycles. A cycle
starting its first adversarial pass always begins at `pass-1.md` regardless of how
many previous cycles occurred. This makes the pass number meaningful as an intra-cycle
index of review quality, not a global sequence.

Second, the `INDEX.md` table gives operators a single file to read to understand
convergence status without opening individual pass files. The `CONVERGENCE_REACHED`
sentinel in the table is machine-readable by the `wave-gate` skill, which checks
Phase 1d convergence before allowing progression to Phase 2.

Third, the `adversarial-reviews/` subdirectory under the cycle directory keeps review
artifacts co-located with the cycle's other outputs (consistency validation, etc.)
while remaining clearly scoped and not mixed with the spec artifacts they review.

The `document_type: adversarial-review-pass` frontmatter field and the `traces_to:
phase-1d-pass-N+1` cross-reference between passes enables traceability: each pass
records what it read, what it found, and what the next pass should verify was fixed.
This is the pattern used in all 6 passes of the v1.0-brownfield-backfill cycle.

## Consequences

### Positive

- Each cycle's adversarial review history is self-contained and browsable at
  `.factory/cycles/<cycle-key>/adversarial-reviews/`.
- Pass numbering is unambiguous within a cycle; `pass-6.md` is always the sixth
  pass of its cycle.
- The `INDEX.md` convergence table provides a one-line summary per pass, readable
  without opening individual pass files.
- The `CONVERGENCE_REACHED` machine-readable sentinel integrates cleanly with the
  wave-gate skill's Phase 1d prerequisite check.

### Negative / Trade-offs

- Review artifacts are stored under `.factory/cycles/`, which is separate from
  `.factory/specs/`. A reader unfamiliar with the convention must know to look in
  `cycles/` for adversarial review history rather than `specs/`.
- There is no global index of all adversarial review passes across all cycles.
  Cross-cycle comparison requires reading each cycle's `INDEX.md` individually.

### Status as of v1.0.0-beta.5

IN-EFFECT. Established during the v1.0-brownfield-backfill cycle
(`.factory/cycles/v1.0-brownfield-backfill/adversarial-reviews/pass-1.md` through
`pass-6.md`). Phase 1d converged after pass 6 (3 consecutive NITPICK). The
cycle's `INDEX.md` records `CONVERGENCE_REACHED` and is read by the wave-gate skill
as the Phase 1d completion signal.

## Alternatives Considered

- **Global sequence under `.factory/specs/adversarial-reviews/`:** All passes for
  all cycles in a single directory with global pass numbers. Rejected: passes from
  different cycles review different spec surfaces; conflating them into a single
  sequence obscures which findings apply to which cycle.
- **Pass files embedded in the phase directory (`.factory/phases/1d/pass-N.md`):**
  Phase-centric rather than cycle-centric storage. Rejected: VSDD cycles can revisit
  phases; a single phase directory would accumulate passes from multiple cycles,
  making the progression ambiguous.
- **No persistent files; findings in git commit messages:** Track findings as
  annotations in commit messages rather than files. Rejected: commit message findings
  are not machine-readable by the wave-gate skill; they cannot be structured with
  frontmatter; they are not co-located with the artifacts they review.

## Source / Origin

- **Code as-built:** `.factory/cycles/v1.0-brownfield-backfill/INDEX.md`
  (Adversarial Reviews table with all 6 passes, finding counts, and CONVERGENCE_REACHED).
- **Code as-built:** `.factory/cycles/v1.0-brownfield-backfill/adversarial-reviews/pass-1.md`
  through `pass-6.md` (live artifacts establishing the convention; pass-1.md
  `document_type: adversarial-review-pass` frontmatter).
- **State tracking:** `.factory/STATE.md` line 29 ("Phase 1d (adversarial spec
  review): CONVERGED (6 passes, 3 consecutive NITPICK)").
- **Subsystems:** SS-05 (Pipeline Orchestration) owns the wave-gate integration
  that reads the `CONVERGENCE_REACHED` sentinel; SS-06 (Skill Catalog) owns the
  adversary and wave-gate skills that produce and consume the review artifacts.
