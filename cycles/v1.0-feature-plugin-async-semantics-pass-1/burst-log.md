---
document_type: burst-log
cycle: v1.0-feature-plugin-async-semantics-pass-1
producer: state-manager
version: "1.0"
last_updated: 2026-05-07
---

# Burst Log — v1.0-feature-plugin-async-semantics-pass-1

Plugin async semantics: partition belongs at the registry layer; defeats silent-block
bleed observed in the prism audit (2026-05-07, 55 silent blocks from
validate-template-compliance).

---

## Burst 1 — F1 delta analysis authored + cycle registered + tech-debt expansion

**Date:** 2026-05-07
**Dispatchers:** orchestrator → architect → state-manager
**Phase:** F1 COMPLETE → human-review-gate

### Outputs

| File | Author | Notes |
|------|--------|-------|
| `.factory/cycles/v1.0-feature-plugin-async-semantics-pass-1/F1-delta-analysis.md` | architect | F1 delta analysis: 4 stories sketched (dispatcher partition, plugin classification, envelope flip, CI lint invariant). 1 ADR + 2 new BCs + 2 new VPs proposed for F2. |
| `.factory/STATE.md` | state-manager | Cycle registration: current_cycle flipped, Active Cycles table updated (plugin-async-semantics added; engine-discipline-pass-1 → PAUSED). Phase Progress + Session Checkpoint updated. |
| `.factory/tech-debt-register.md` | state-manager | TD-027 authored (Stop-hook async-block surfacing; medium severity, S-M effort, v1.1 target). Scope statement broadened from defect-only to general deferred-work inbox. |
| `.factory/cycles/v1.0-feature-plugin-async-semantics-pass-1/burst-log.md` | state-manager | This file (cycle burst log initialized). |

### Decisions sealed

None. F1 is exploratory; F2 will produce seal-able specs once human approval gate clears.

### Open questions carried forward

- OQ-1: Stop-hook (or SubagentStop) turn-end summary for async-block decisions — now tracked as TD-027.
- OQ-2: Plugin classification taxonomy (deterministic validators vs advisory/telemetry) — architect's F1 analysis outlines; product-owner + architect to finalize in F2.
- OQ-3: Envelope flip scope — which plugins get `on_error: "block"` → sync reclassification vs which legitimately stay async.

### Status

F1 COMPLETE. Awaiting human review gate before F2 spec evolution.

---
