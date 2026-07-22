# PR #754 — Re-Review (head 667c8936)

**Title:** fix(config): register .factory/planning artifact paths in artifact-path-registry
**Repo:** drbothen/vsdd-factory
**Branch:** fix/planning-registry-entries @ `667c89366c79b7a30e806e628b10b355b46785e0`
**Reviewer model:** claude-sonnet-4-6 (independent fresh-context re-review)
**Prior head reviewed:** 9b9a4b02 — REQUEST_CHANGES (HIGH: TD-VSDD-060 sibling sweep incomplete; MEDIUM: description overstated completeness)
**Date:** 2026-07-22

---

## Verdict: APPROVE

Both prior findings are structurally closed. The author performed an exhaustive independent grep across skills/, agents/, workflows/, docs/, templates/ and found 8 additional paths, bringing the total to 16. My own independent sweep confirms the 16-path inventory is now exhaustive. No new blockers found.

---

## Prior-finding closure status

### HIGH (sibling-sweep miss) — CLOSED

**Independent verification method:** Re-ran the full grep sweep myself across all non-.factory source files:

```
grep -r "\.factory/planning/" plugins/vsdd-factory/skills/  → 12 unique paths
grep -r "\.factory/planning/" plugins/vsdd-factory/agents/  → .factory/planning/* wildcard + 3 named paths
grep -r "\.factory/planning/" plugins/vsdd-factory/workflows/ → 0 (workflows/ at root doesn't exist; lobster files are in plugins/)
grep -r "\.factory/planning/" . (excluding .git, target, .factory) → 16 named paths + .factory/planning/* wildcard
```

All 16 paths in the PR inventory appear in the grep output. The only path in my sweep not in the 16-item inventory is `.factory/planning/research/s-18.12-python-dep-policy.md`, which appears in `plugins/vsdd-factory/docs/bash-portability.md:241` as a parenthetical background-context citation ("Background: …"), not as a skill write target. It is correctly excluded from the registry.

The `.factory/planning/*` wildcard in `research-agent.md:176` ("Write only to `.factory/planning/` or `.factory/specs/domain-research.md`") is a broad write-scope declaration for the agent, not a specific unnamed output file. The named concrete path `domain-research.md` is the canonical planning-dispatch output and is registered.

**Conclusion:** 16-path inventory is exhaustive.

### MEDIUM (description overstated completeness) — CLOSED

The PR description now contains a full 16-row per-path table with explicit writer and source-evidence columns. The test plan states 84 total (52 pre-existing + 32 new) — verifiable by count. No overstatement of completeness.

---

## Spot-verification: 8 new paths (paths 9–16)

Each of the 8 sibling-sweep additions was independently verified against the cited source before this verdict:

| Path | Cited source | Verified |
|------|-------------|---------|
| `brief-validation.md` | validate-brief/SKILL.md §Output | Line 129: "Write validation report to `.factory/planning/brief-validation.md`" |
| `readiness-report.md` | implementation-readiness/SKILL.md §Output | Line 125: "Write readiness report to `.factory/planning/readiness-report.md`" |
| `research-report.md` | planning-research/SKILL.md Output Artifacts | Line 66: "`.factory/planning/research-report.md`" |
| `research-sources.md` | planning-research/SKILL.md Output Artifacts | Line 67: "`.factory/planning/research-sources.md` (citations and links)" |
| `domain-research.md` | research-agent.md write scope | Line 176: "Write only to `.factory/planning/` or `.factory/specs/domain-research.md`" — primary evidence confirmed |
| `prd-validation.md` | planning.lobster validate-existing-prd step | ~Line 201: "Write validation report to .factory/planning/prd-validation.md." |
| `architecture-validation.md` | planning.lobster validate-existing-architecture step | ~Line 213: "Write report to .factory/planning/architecture-validation.md." |
| `market-context.md` | docs/FACTORY.md dispatch example | Line 171: business-analyst "Write analysis to … `.factory/planning/market-context.md`" |

All 8 confirmed. One secondary citation note: the PR description lists "orchestrator.md absolute-path example line 151" as a second source for `domain-research.md`. That file does not exist at `plugins/vsdd-factory/agents/orchestrator.md`, and `docs/FACTORY.md:151` does not match the claimed content. This is a minor documentation inaccuracy in the PR description — the path is fully justified by the primary `research-agent.md:176` citation, so it does not affect the registry entry's correctness or completeness. Not blocking.

8/8 verified from source.

---

## Test quality (PASS — non-vacuous, follows established pattern)

- 32 new tests (2 per path: `matches_canonical → Block`, `hook_logic → Continue`).
- Uses `production_registry_path_bc607038()` / `production_registry_yaml_bc607038()` helpers that walk up from `CARGO_MANIFEST_DIR` until `plugins/vsdd-factory/config/artifact-path-registry.yaml` is found, panicking if not found. Cannot silently pass against a fixture.
- This is the established per-group helper pattern: existing file has `_i300` and `_i473` groups with their own identically-structured helpers. `_bc607038` is consistent, not a DRY violation.
- The `matches_canonical → Block` assertion would fail if the registry entry were absent or set to advisory level. The `hook_logic → Continue` assertion would fail if the validator blocked on an unregistered path. Both are load-bearing.
- CI green: `cargo test -p validate-artifact-path` reports 84 passed (52 + 32), 0 failed.

---

## Registry diff quality (PASS)

- 16 entries, all `enforcement_level: block`. Correct: these are structured factory outputs, not advisory-only paths.
- Descriptions are specific — each cites the writing skill/agent and the step/section, distinguishing overlapping paths (e.g., `planning/product-brief.md` vs. `specs/product-brief.md`, `market-context.md` vs. `market-intel.md`) in prose.
- YAML schema conformant: `artifact_type` / `canonical_path_pattern` / `description` / `enforcement_level` pattern matches existing adjacent entries. Folded block scalars (`>`) parse correctly. No `required_frontmatter` (these are free-form planning docs — appropriate).
- Single conventional commit; no AI attribution.

---

## New findings

None.

---

## Checklist

1. HIGH (sibling-sweep miss) — CLOSED
2. MEDIUM (description accuracy) — CLOSED
3. New tests follow production harness — PASS
4. 8 new path writer citations independently verified — PASS
5. One minor secondary citation inaccuracy (orchestrator.md domain-research ref) — OBSERVATION, not blocking
6. Registry diff: schema, enforcement levels, descriptions — PASS
7. CI green — PASS
8. No new issues found in diff hunks
