---
document_type: wave-audit
audit_id: E-8-tier1-native
story_id: S-8.09
wave: W-15
status: complete
timestamp: "2026-05-02T14:00:00Z"
---

# W-15 Tier 1 Native Migration Audit

**Story:** S-8.09 — Native port: regression-gate + adapter retirement prep (W-15 closer)
**Date:** 2026-05-02
**Purpose:** AC-011 — Verify all 9 Tier 1 hooks are now native WASM; confirm zero
`legacy-bash-adapter.wasm` references for Tier 1 entries in hooks-registry.toml.

---

## Tier 1 Hook Status (All 9 Hooks)

| Hook Name | WASM Plugin Path | Prior (legacy-bash-adapter) | Native Now? |
|-----------|-----------------|----------------------------|-------------|
| handoff-validator | `hook-plugins/handoff-validator.wasm` | yes | YES |
| pr-manager-completion-guard | `hook-plugins/pr-manager-completion-guard.wasm` | yes | YES |
| track-agent-stop | `hook-plugins/track-agent-stop.wasm` | yes | YES |
| update-wave-state-on-merge | `hook-plugins/update-wave-state-on-merge.wasm` | yes | YES |
| validate-pr-review-posted | `hook-plugins/validate-pr-review-posted.wasm` | yes | YES |
| session-learning | `hook-plugins/session-learning.wasm` | yes | YES |
| warn-pending-wave-gate | `hook-plugins/warn-pending-wave-gate.wasm` | yes | YES |
| track-agent-start | `hook-plugins/track-agent-start.wasm` | yes | YES |
| regression-gate | `hook-plugins/regression-gate.wasm` | yes (this story) | YES |

All 9 Tier 1 hooks confirmed native. Zero `legacy-bash-adapter.wasm` references
in the Tier 1 section of `plugins/vsdd-factory/hooks-registry.toml`.

---

## Adapter Status

**legacy-bash-adapter crate:** STILL PRESENT in workspace (`crates/hook-plugins/legacy-bash-adapter/`).
Adapter is NOT deleted — deferred to S-8.29 (per E-8 D-10 and S-8.09 AC-011 scope).

The adapter still serves Tier 2/3 hooks that have not yet been ported.

**bin/emit-event:** STILL PRESENT at `plugins/vsdd-factory/bin/emit-event`.
Deletion deferred to S-8.29 (per E-8 D-10). Tier 2/3 bash hooks in W-16/W-17
require it via hooks.json direct path.

---

## Verification Command

```
grep -n "legacy-bash-adapter" plugins/vsdd-factory/hooks-registry.toml | head -5
```

The output no longer contains any of the 9 Tier 1 hook names. Tier 2/3 entries
(validate-anchor-capabilities-union, validate-bc-title, etc.) still reference
legacy-bash-adapter — expected and correct.

---

## Statement

**W-15 Tier 1 migration complete. S-8.11 wave gate unblocked.**

All 9 Tier 1 hooks (PostToolUse + SubagentStop lifecycle) have been successfully
ported from bash via `legacy-bash-adapter.wasm` to native WASM crates. The W-15
wave gate is satisfied. W-16 stories (S-8.11+) may now be dispatched.

**Scope qualifier (W-15 gate fix CRIT-W15-005):** W-15 retired **Tier 1 only**
(12 hooks across 9 crates). Approximately 30+ Tier 2/3 hooks remain on
`legacy-bash-adapter` (e.g., `validate-bc-title`, `validate-anchor-capabilities-union`,
`convergence-tracker`). Full Tier 2/3 retirement is tech debt TD-014, calendar-gated
to v1.0 GA close via story S-8.29. The phrase "0 hooks on legacy-bash-adapter"
always means "0 **Tier 1** hooks on legacy-bash-adapter".

---

## OQ-6 Resolution

OQ-6 (E-8 epic open question: "regression-gate appears to invoke external test
runners") is **RESOLVED** by this story.

Security audit confirmed: regression-gate is a pure PostToolUse OBSERVER hook.
It does NOT invoke test runners — it reads the already-completed Bash tool's
output envelope. Capability profile confirmed:
- `binary_allow = []` (empty — no subprocess needed)
- `read_file` for `.factory/regression-state.json`
- `write_file` for `.factory/regression-state.json`
- `emit_event` (unconditionally wired, no declaration needed)

OQ-6 audit document: `E-8-oq6-capability-profile.md` (this cycle directory).
