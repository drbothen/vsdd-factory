---
name: spec-versioning
description: >
  Manages spec evolution using semantic versioning. Determines version bumps,
  maintains changelog, detects drift between code and spec versions.
---

# Spec Versioning

## Version Scheme

Specs follow semantic versioning: `MAJOR.MINOR.PATCH`

### When to Bump MAJOR

A MAJOR bump indicates breaking changes -- existing implementations built against
the previous spec version are no longer compatible.

Examples:
- Architectural rework (new module structure, changed interfaces)
- Removed features or requirements
- Changed behavior of existing requirements (not just additions)
- Renamed or re-ID'd requirements (FR-001 becomes something else)

Impact: all stories built against the previous MAJOR version must be reviewed
for compatibility. Implementation may need migration.

### When to Bump MINOR

A MINOR bump indicates backward-compatible additions. Existing implementations
are still valid; new code is needed for new features only.

Examples:
- New functional requirements (FR-025, FR-026 added)
- New non-functional requirements
- New verification properties
- New components added to architecture (existing components unchanged)
- New edge cases added to catalog

This is the **most common bump in Feature Mode** -- adding features is additive.

### When to Bump PATCH

A PATCH bump indicates clarifications and corrections that do not change behavior.

Examples:
- Wording improvements in requirement descriptions
- Edge case additions that clarify existing behavior (not new behavior)
- Typo fixes in spec documents
- Adding examples to existing requirements
- Correcting a constraint value (was wrong, implementation already correct)

Impact: no code changes needed. The spec caught up to reality.

## Changelog Format

Each spec version bump must be recorded in `.factory/spec-changelog.md`
using `templates/spec-changelog-template.md`.

## Tracking Code-to-Spec Version

Every story spec records which spec version it was built against:

```yaml
# In story frontmatter:
spec_version: "1.2.0"
```

Every implementation commit message references the spec version:

```
feat(notifications): add delivery engine

Implements S-1.08.
Spec version: 1.3.0
```

## Drift Detection

Drift occurs when code was built against spec v1.2.0 but the spec is now v1.4.0.

### Detection Workflow

1. Read all story specs and extract their `spec_version` fields
2. Read the current spec version from the PRD frontmatter
3. For each story where `story.spec_version < current_spec_version`:
   - PATCH drift: informational only (no action needed)
   - MINOR drift: check if new requirements affect this story's scope
   - MAJOR drift: flag for mandatory review -- story may be incompatible

### Drift Report

Write drift findings to `.factory/spec-drift-report.md`:

```markdown
## Spec Drift Report

Current spec version: 1.4.0

| Story | Built Against | Drift | Severity | Action Needed |
|-------|--------------|-------|----------|---------------|
| S-1.01 | 1.0.0 | 4 minor | LOW | Review new reqs for overlap |
| S-1.03 | 1.1.0 | 3 minor | LOW | Review new reqs for overlap |
| S-1.10 | 1.2.0 | 1 major, 1 minor | HIGH | Mandatory review |
```

### When to Run Drift Detection

- At the start of Phase F1 (before delta analysis)
- After Phase F2 (after spec evolution, to verify no unintended drift)
- On demand when the human requests a drift audit

## L4 Immutability (Append-Only After Proof)

Once an L4 verification property (VP-NNN) has a passing proof, its definition is
**immutable** -- it cannot be modified or deleted, only appended to.

### Immutability Rules

1. **Proven VPs are LOCKED:** After `formal-verifier` confirms a proof passes for VP-NNN,
   the VP file is marked with `status: locked` and `proof_date: YYYY-MM-DD`. No agent
   may modify the VP's preconditions, postconditions, or invariants after locking.

2. **Append-only amendments:** If a locked VP needs refinement, create a NEW VP-NNN+1
   that references the original. The original remains unchanged as historical record.
   ```yaml
   # VP-015 (locked, proven 2026-03-20)
   # VP-015a (amendment, extends VP-015 with additional constraint)
   amends: VP-015
   ```

3. **Withdrawal requires justification:** A locked VP can be WITHDRAWN only with
   documented justification, impact analysis, and human approval. Withdrawn VPs
   remain in the registry with `status: withdrawn` and a `withdrawal_reason` field.

4. **Hierarchy versioning:** When any level in the L1→L4 hierarchy changes, version
   bumps propagate upward:
   - L4 VP change → L3 BC patch bump (if VP was linked to BC)
   - L3 BC change → L2 CAP minor bump (if BC was linked to CAP)
   - L2 CAP change → L1 spec version bump

### Enforcement

The `consistency-validator` checks for immutability violations:
- Detect modifications to locked VP files (compare against git history)
- Flag any VP with `status: locked` that has been modified since its proof date
- Report violations as CRITICAL findings that block the pipeline

## Quality Gate

- [ ] Version bump type (MAJOR/MINOR/PATCH) determined with rationale
- [ ] Changelog entry drafted using `templates/spec-changelog-template.md`
- [ ] Drift report produced if drift detection was triggered
- [ ] L4 immutability rules enforced (no locked VPs modified)

## Failure Modes

- If version history is inconsistent (gaps, duplicates, out-of-order): flag the inconsistency and request human resolution before proceeding
- If a locked VP appears to have been modified: report as CRITICAL finding, block pipeline, do not attempt auto-fix
- If spec version in story frontmatter cannot be parsed: flag the story for manual review
