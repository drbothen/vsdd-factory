# AC-9 — Human review (manual gate, deferred to PR review)

**AC statement:** Reviewed by a human who has actually performed the
migration from v0.79.x to v1.0.

**Evidence type:** gate status

## Status

DEFERRED — manual gate. No automated evidence possible.

## Rationale

AC-9 is a human qualification gate: it requires a reviewer who has
personally performed the v0.79.x → v1.0 migration to confirm the
guide's accuracy. This cannot be demonstrated by automated tooling
or by the implementing agent.

This gate is deferred to the PR review cycle. The PR description
should explicitly call for a reviewer who has performed the migration.

## v1.1 BC candidate

`BC-8.31.008-human-migration-reviewer-qualification` — registered in
story S-5.05 v1.1 BC Candidates table. Not yet contracted; escalate to
a v1.0 BC if the reviewer-qualification gate is required before the
1.0.0 release tag.

## Architecture Compliance Rule

From S-5.05 Architecture Compliance Rules table:
> Migration guide content must be reviewed by a human who has performed
> the migration. Enforcement: manual gate.

## PR action required

PR reviewer must confirm they have run `/plugin update vsdd-factory@vsdd-factory`
and `/vsdd-factory:activate` on a factory that previously ran v0.79.x
before approving this story.
