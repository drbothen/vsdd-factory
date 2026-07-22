# PR #717 Fresh-Eyes Review — fix(config): register holdout scenario-selection path in artifact-path-registry

**Reviewer:** pr-reviewer (fresh-context, different-model cognitive diversity)
**Branch:** `fix/register-scenario-selection` → `develop`
**Refs:** #473 (pairs with sibling registry PR for #300)

## VERDICT: APPROVE

No blocking findings. Clean, tightly-scoped fix: one registry entry mirroring the neighboring
Holdout Evaluations schema exactly, plus a proper RED→GREEN test pair that loads the real shipped
registry (never vacuous — helper hard-panics if the registry can't be located).

## Checklist

| # | Item | Result |
|---|------|--------|
| 1 | Diff coherence | PASS — both files serve #473; registry entry + tests only. No unrelated changes. |
| 2 | Description accuracy | PASS — PR body matches the diff exactly. |
| 3 | Test coverage | PASS — RED (2 failing) → GREEN (2 passing) shown; tests load the actual production registry. |
| 4 | Demo evidence | N/A — config/hook fix PR (Class 0), not a story with ACs; fix-pr-delivery skips demos. |
| 5 | Commit quality | PASS — conventional `fix(config):`, Refs #473. |
| 6 | Diff size | PASS — ~90 lines added, well under the 500-line flag. |
| 7 | Missing changes | PASS with one advisory (see finding 1 — workflow files not in diff, parity unverifiable). |
| 8 | Dependency status | Notes dependence on #527 (workflow correction) and sibling #300 PR; no hard correctness coupling — the registry entry is harmless even if #527 lags. |

## Findings

### [ADVISORY] Pattern-vs-workflow parity unverifiable from the diff
The value of this PR rests on the literal pattern `.factory/holdout-scenarios/scenario-selection.json`
exactly matching what the phase-4 workflow writes. Neither phase-4 workflow file is in the diff, so I
cannot confirm the workflow writes that exact path with no intermediate glob segment. A per-cycle
subdirectory (e.g. `.factory/holdout-scenarios/<cycle>/scenario-selection.json`) would NOT match this
literal, non-globbed pattern. The two new tests assert against the same hard-coded string, so they
cannot catch a registry-vs-runtime mismatch. **Recommend the maintainer confirm the workflow's write
path is the literal, un-nested string before merge.** This is the only thing that could make the fix a
no-op in production while tests stay green.

### [SUGGESTION] Disclosed EC-007 pre-existing defect is real test-coverage rot
The description notes EC-007 loads the production registry via a hard-coded `../../../../` path that
resolves one level too high and "silently falls back to a fixture" — meaning EC-007 likely validates a
fixture, not the shipped registry (vacuous-pass risk on an existing test). Under the production-grade
default, the cleanest resolution is to repoint EC-007 at the same ancestor-walk helper this PR
introduces rather than deferring to a separate observation. Not a blocker for this PR's scope; the fix
is a few lines and reuses code already added here.

### [ADVISORY] Ancestor-walk locator robust against vacuous passes, not out-of-tree builds
`production_registry_path_i473()` walks up from `CARGO_MANIFEST_DIR` and hard-panics if the registry
isn't found — a genuine improvement over EC-007 that prevents vacuous passes. Residual risk: a
packaged/vendored build where `plugins/vsdd-factory/config/` is not an ancestor would panic rather than
skip. Acceptable (loud failure > silent skip); noted for awareness.

### [NIT] `*_i473` issue-number suffix on helper identifiers
Reads slightly unusual as a permanent API surface, but namespaces the helpers to avoid collision with
the EC-007 harness and is self-documenting. Fine to keep.

## What I verified
- The new registry entry uses a literal (non-glob) `canonical_path_pattern`, so it cannot over-match
  other `.factory/` paths.
- The entry is placed inside the Holdout Evaluations section, before the Measurements section, and
  mirrors the neighboring entry schema (`artifact_type` / `canonical_path_pattern` / `description` /
  `enforcement_level: block`).
- RED evidence shows both tests failing with `NoMatch` / `ARTIFACT_PATH_UNREGISTERED` before the entry;
  GREEN shows both passing after — confirming the tests are load-bearing and tied to the registry entry.
