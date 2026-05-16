---
document_type: architect-decision
level: ops
status: final
producer: architect
timestamp: 2026-05-16T00:00:00Z
phase: section-12-step-3-m2-fix-burst-2
cycle: brownfield-backfill
inputs:
  - .factory/cycles/v1.0-brownfield-backfill/architect-m2-q5-tool-attribute-2026-05-16.md
  - .factory/cycles/v1.0-brownfield-backfill/s-15.08-local-adversary-pass-2.md
  - plugins/vsdd-factory/tests/validate-index-cite-refresh/pass-all-current.bats
  - plugins/vsdd-factory/tests/validate-index-cite-refresh/fail-stale-vp-index.bats
  - plugins/vsdd-factory/tests/validate-index-cite-refresh/fail-open-missing-index.bats
  - plugins/vsdd-factory/tests/validate-index-cite-refresh/fail-cross-cell-index-md.bats
  - plugins/vsdd-factory/tests/validate-index-cite-refresh/fail-stale-bc-index.bats
  - plugins/vsdd-factory/tests/validate-index-cite-refresh/fail-multi-stale-cites.bats
  - plugins/vsdd-factory/tests/validate-index-cite-refresh/fail-stale-story-index.bats
  - plugins/vsdd-factory/tests/validate-index-cite-refresh/fail-cross-cell-state-md.bats
input-hash: "a777108"
traces_to: architect-m2-q5-tool-attribute-2026-05-16.md
related_stories: [S-15.07, S-15.11, S-15.09, S-15.14]
closes_finding: F-S15.07-LOCAL-P2-002
---

# Architect Decision — M2 Q6: Bats Inline Registry Scope of Q5 Convention Lock (2026-05-16)

## Context

Q5 (architect-m2-q5-tool-attribute-2026-05-16.md) established `tool = "Edit|Write"` as the
canonical form for hook registry entries and swept the three production-registry violations
(lines 844, 861, 995 of `plugins/vsdd-factory/hooks-registry.toml`). LOCAL adversary pass-2
surfaced F-S15.07-LOCAL-P2-002 (LOW, pending intent verification): 8 bats inline `_write_registry()`
heredocs in `plugins/vsdd-factory/tests/validate-index-cite-refresh/*.bats` still use
`tool = "Write|Edit"`. The question is whether Q5's convention lock extends to these test-side
inline registries. Investigation confirmed that all 8 bats files are on the unmerged
`feature/S-15.08-dim2-gates-bash-templates` PR (merge-base is develop's `224fa184`); no file has
landed on `develop` yet.

---

## Decision

**Option Q6-A: IN-SCOPE. Sweep all 8 bats inline registries to `tool = "Edit|Write"`.**

The `_write_registry()` heredoc writes the TOML fragment that the dispatcher subprocess reads as
its live registry during bats test execution. This is not decorative fixture data — the dispatcher
processes it identically to the production registry when the test invokes the binary. Two
additional factors clinch Q6-A: (1) all 8 files are still in the open PR and the fix is bounded
(8 one-line edits, ~5 minutes), making in-scope remediation the canonical-principle default under
Rule 4; (2) S-15.09, S-15.11, and S-15.14 test-writers will copy these bats templates directly,
so leaving `Write|Edit` in merged test scaffolding propagates the minority form into three
downstream stories — the exact accumulation risk Q5 was designed to prevent.

---

## Q5 Scope Clarification

Q5's Future-Story Convention Lock paragraph stated "registry entries added by S-15.09/S-15.11/S-15.14"
and cited `plugins/vsdd-factory/hooks-registry.toml` by name; it did not explicitly address bats
inline registries because the S-15.07 test files had not yet been reviewed at Q5 authoring time.
Going forward, the Q5 convention lock binds ALL TOML hook registry entries read by the dispatcher
binary, whether in `plugins/vsdd-factory/hooks-registry.toml` (production) or in a bats
`_write_registry()` heredoc (test). It does NOT bind TOML fixture snippets that are parsed only
by test assertion logic (grep/awk/diff) and never fed to the dispatcher subprocess; those are
documentation-equivalent and may use any convenient form.

---

## Implementation Plan

**Owner: implementer (routed by orchestrator after this document commits).**

8 single-line edits in `plugins/vsdd-factory/tests/validate-index-cite-refresh/*.bats`:

| File | Line (approx) | Current | Change to |
|------|----------------|---------|-----------|
| pass-all-current.bats | 37 | `tool = "Write\|Edit"` | `tool = "Edit\|Write"` |
| fail-stale-vp-index.bats | 38 | `tool = "Write\|Edit"` | `tool = "Edit\|Write"` |
| fail-open-missing-index.bats | 43 | `tool = "Write\|Edit"` | `tool = "Edit\|Write"` |
| fail-cross-cell-index-md.bats | 39 | `tool = "Write\|Edit"` | `tool = "Edit\|Write"` |
| fail-stale-bc-index.bats | 38 | `tool = "Write\|Edit"` | `tool = "Edit\|Write"` |
| fail-multi-stale-cites.bats | 42 | `tool = "Write\|Edit"` | `tool = "Edit\|Write"` |
| fail-stale-story-index.bats | 38 | `tool = "Write\|Edit"` | `tool = "Edit\|Write"` |
| fail-cross-cell-state-md.bats | 40 | `tool = "Write\|Edit"` | `tool = "Edit\|Write"` |

No functional change (dispatcher regex matching is order-insensitive per Q5 §Investigation).
No new tests required.

**Verification gate (implementer must execute and record stdout):**

```bash
grep -rn 'tool = "Write|Edit"' plugins/vsdd-factory/tests/validate-index-cite-refresh/
# Expected output: (empty — zero lines)
```

If the grep returns any lines, the sweep is incomplete and the commit must not land.

**Commit target:** `feature/S-15.08-dim2-gates-bash-templates` (current open PR branch).
Commit message: `fix(tests): align bats inline-registry tool attr to Edit|Write canonical form (S-15.07 F-P2-002 per Q6-A)`

---

## Future-Story Guidance

S-15.09, S-15.11, S-15.14, and all subsequent hook stories MUST use `tool = "Edit|Write"` (not
`"Write|Edit"`) in every `_write_registry()` heredoc or inline registry TOML block passed to the
dispatcher subprocess during bats test execution.
