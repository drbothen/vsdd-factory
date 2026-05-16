---
document_type: architect-decision
level: ops
status: final
producer: architect
timestamp: 2026-05-16T00:00:00Z
phase: section-12-step-3-m2-inter-story-adjudication
cycle: brownfield-backfill
inputs:
  - .factory/cycles/v1.0-brownfield-backfill/architect-m2-2026-05-16.md
  - .factory/cycles/v1.0-brownfield-backfill/s-15.08-local-adversary-pass-1.md
  - crates/ (routing.rs tool_matches inspection)
  - plugins/vsdd-factory/hooks-registry.toml (tool attribute grep)
input-hash: "df9db17"
traces_to: architect-m2-2026-05-16.md
related_stories: [S-15.07, S-15.11, S-15.09, S-15.14]
closes_finding: F-S15.07-LOCAL-P1-005
---

# Architect Decision — M2 Q5: `tool` Attribute Order Convention for hooks-registry.toml (2026-05-16)

## Context

The LOCAL adversary pass-1 for S-15.07 (validate-index-cite-refresh WASM hook) surfaced finding
F-S15.07-LOCAL-P1-005 (LOW — pending intent verification): the new registry entry used
`tool = "Write|Edit"`, matching the adjacent `validate-artifact-path` neighbor rather than the
dominant `tool = "Edit|Write"` form used by the remaining 29 entries. The finding requested
architect adjudication before downstream M2 stories (S-15.11, S-15.09, S-15.14) propagate the
drift further.

---

## Investigation

### Dispatcher order-sensitivity

`crates/factory-dispatcher/src/routing.rs` `tool_matches()` (lines 47–58):

```rust
fn tool_matches(entry: &RegistryEntry, tool_name: &str) -> bool {
    match &entry.tool {
        None => true,
        Some(pattern) => match regex::Regex::new(pattern) {
            Ok(re) => re.is_match(tool_name),
            Err(_) => false,
        },
    }
}
```

The `tool` field is compiled as a regex and matched via `re.is_match(tool_name)`. The `|`
operator in regex alternation is commutative for this use case: `"Edit|Write"` and `"Write|Edit"`
both match the single token `"Edit"` and the single token `"Write"` identically. There is no
first-match short-circuit that would produce different behavior — `is_match` returns `true` as
soon as any branch matches, and both orderings reach the same boolean result for both inputs.

**Verdict: order does not matter functionally.** The drift is purely cosmetic.

### Occurrence counts (hooks-registry.toml, verified by grep)

| Form | Count | Entries |
|------|-------|---------|
| `tool = "Edit\|Write"` | 29 | Lines 100–729 (dominant; 29 entries spread across all hook classes) |
| `tool = "Write\|Edit"` | 3 | Line 844 (validate-artifact-path), line 861 (validate-index-cite-refresh), line 995 (validate-stable-anchors) |

All three `"Write|Edit"` entries are `validate-*` hooks added in the S-13/S-15 WASM migration
wave. They appear to have followed each other rather than the broader registry majority.

### Prior ADR / decision check

Searched `.factory/specs/architecture/decisions/` (22 ADRs) and `.factory/cycles/v1.0-brownfield-backfill/architect-m2-2026-05-16.md`. No prior architect decision or ADR addresses `tool` attribute ordering. This is a first adjudication. The M2 decision document addressed Questions 1–4 (shared schema crate, crate naming, stories size validation scope, crate naming template for `validate-NOUN`); tool attribute ordering was not recognized as a Q5-class question at M2 dispatch authoring time.

---

## Decision

**Option A — Align all three `"Write|Edit"` entries to the canonical `"Edit|Write"` form.**

**Rationale:** The drift is cosmetic (dispatcher is order-insensitive) but convention consistency
has compounding value: each future M2 story that writes a new registry entry copies a nearby
example. With three `"Write|Edit"` entries now clustered in the `validate-*` section, S-15.09,
S-15.11, and S-15.14 are likely to copy the wrong form. A three-line sweep closes the drift now
rather than accumulating four more minority entries. The sweep is low-risk (functionally
equivalent) and directly in-scope under CLAUDE.md Canonical Principle Rule 4.

Option C (accept heterogeneous state) was considered given the cosmetic nature of the drift.
It was rejected because documenting the rule without fixing the three existing violations leaves
an active wrong-example for downstream stories — the canonical principle requires fixing in scope
when the fix is bounded and unambiguous.

---

## Implementation Plan

**Owner: implementer (routed by orchestrator after this document commits).**

Three single-line edits in `plugins/vsdd-factory/hooks-registry.toml`:

| Line | Current | Change to |
|------|---------|-----------|
| 844 | `tool = "Write\|Edit"` | `tool = "Edit\|Write"` |
| 861 | `tool = "Write\|Edit"` | `tool = "Edit\|Write"` |
| 995 | `tool = "Write\|Edit"` | `tool = "Edit\|Write"` |

No functional change. No test additions required (existing tests cover `Edit|Write` regex matching;
functionally equivalent). A smoke-check grep after the edit must confirm zero remaining
`"Write|Edit"` occurrences in hooks-registry.toml.

**Verification gate (implementer must execute and record stdout):**

```bash
grep -n 'tool = "Write|Edit"' plugins/vsdd-factory/hooks-registry.toml
# Expected output: (empty — zero lines)
```

If the grep returns any lines, the sweep is incomplete and the commit must not land.

**Commit:** Single commit to `develop` branch (not factory-artifacts — this is a source code edit,
not a spec-only artifact). Commit message: `fix(registry): align tool attribute to Edit|Write
canonical form -- sweep validate-artifact-path validate-index-cite-refresh validate-stable-anchors`

---

## Future-Story Convention Lock

All registry entries added by S-15.09, S-15.11, S-15.14, and any subsequent hook story MUST use
`tool = "Edit|Write"` (not `"Write|Edit"`) when the hook fires on file-write operations; this is
the canonical form established by 29 of 32 entries in `plugins/vsdd-factory/hooks-registry.toml`
and documented in this decision as the authoritative convention.
