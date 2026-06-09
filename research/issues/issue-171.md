# Issue #171 — revalidate deferred items with research-agent before pulling them into active work

**Date:** 2026-06-09
**Repo:** vsdd-factory (self-referential) @ `develop` `82163b7f`
**Issue:** [#171](https://github.com/) — *feat(workflows): revalidate deferred items with research-agent before pulling them into active work* (label: enhancement; state: OPEN)
**Research agent:** Claude (vsdd-factory:research-agent)
**Consumer:** architect / story-writer / orchestrator-sequence fix-burst

> **Meta-note:** this is the exact feature the operator is invoking *right now* — spawning research-agent to revalidate a backlog of deferred GitHub issues before pulling them into active work. The issue asks to codify that very gate into the pipeline.

---

## Restated Question

When a **deferred item** (a Phase 5/6 finding, a tech-debt entry, a GAP, a blocked-then-unblocked requirement, a long-sitting backlog feature) is later pulled into active work, the pipeline starts implementing it **without first revalidating that it's still worth doing**. Deferred context is treated as evergreen. The ask: a **research-agent revalidation gate** that fires at the moment a deferred item transitions to active work, returns a **STILL-VALID / OBSOLETE-SUPERSEDED** verdict (with cited evidence) before any spec/code work begins, and on OBSOLETE routes to a human decision (re-defer / drop / replace) instead of auto-proceeding — cheaply (age-thresholded, cached, with a `last_revalidated` stamp).

---

## Codebase Grounding (decisive)

Every claim in the issue's "where research validation happens vs doesn't" table was verified. The gate is genuinely absent.

### research-agent IS invoked at initial-planning surfaces — CONFIRMED

- **Discovery idea scoring** (`agents/orchestrator/discovery-sequence.md:31-41`): four research-agent calls (market research, customer-feedback ingestion, competitive monitoring, usage analytics) feed fresh ideas *before* scoring.
- **Feature-mode Market Intelligence** (`agents/orchestrator/feature-sequence.md:32-36`): "Spawn business-analyst + research-agent: 'Market intelligence for proposed feature' … **(skip for bug fixes)**."
- Greenfield Phase 1 domain/market research (per the issue; not re-verified line-by-line here).

### research-agent is NOT invoked on deferred-item-pickup paths — CONFIRMED

- **Maintenance sequence** (`agents/orchestrator/maintenance-sequence.md`): a grep for `research-agent | revalidat | deferred | backlog` returned **zero matches**. Maintenance sweep findings — the main tech-debt source — are routed straight to fix-PR delivery with no "is this debt still worth fixing?" check.
- **Feature sequence** (`feature-sequence.md:32-36`): the single research-agent call (Market Intelligence) is **generic, not conditioned on whether the item was deferred or how old it is, and explicitly skipped for bug fixes**. A TD flagged months ago, or a feature backlogged for cycles, enters Phase F1 → implementation with no staleness check.
- **Discovery cooldown** (`discovery-sequence.md:76-77, 27`): the sequence checks "cooldown periods for previously deferred ideas" and avoids *re-proposing* recently-rejected ideas — but on **cooldown lift it does NOT attach a fresh "has the landscape changed since deferral?" research sub-task**. The cooldown is a re-proposal throttle, not a revalidation gate. Manually-backlogged items never get even that.

### No revalidation primitive anywhere

No skill, workflow (`workflows/*.lobster`), or hook implements a deferred-item revalidation gate, a `last_revalidated` field, or a STILL-VALID/OBSOLETE verdict. (The earlier broad grep for `revalidat` matched only the `validate` word-stem in unrelated validators.)

### Deferral surfaces exist and are real

The issue's deferral-surface table is accurate: tech-debt register (`templates/tech-debt-register-template.md`), STATE.md Blocking Issues + Skip Log (`templates/state-template.md`), GAP register, discovery backlog (`discovery-sequence.md`), Product Backlog (`steady-state.md`). All are write-once-then-resurface with no revalidation step.

### Prior closure check

No CHANGELOG entry references #171 or a deferred-item revalidation gate. Nothing has landed.

---

## External Research — primary sources

`perplexity_research` (reasoning_effort=medium) surveyed staleness-revalidation patterns. The issue's design instinct — **age/TTL-triggered revalidation with a stamp so you don't re-research within a window** — maps directly onto two well-established families:

- **Cache-invalidation / stale-while-revalidate (the direct analog).** RFC 5861 defines `stale-while-revalidate`: serve the cached value but trigger an async revalidation when the entry is past a freshness window — https://datatracker.ietf.org/doc/html/rfc5861. Fastly's stale-content guide and Varnish caching tutorials operationalize "serve stale, revalidate in background past TTL" — https://www.fastly.com/documentation/guides/concepts/cache/stale and https://www.varnish-software.com/developers/tutorials/http-caching-basics/. HTTP conditional revalidation (`If-None-Match` / ETag) is the "has this changed since I snapshotted it?" primitive — https://docs.rs/http/latest/http/header/constant.IF_NONE_MATCH.html and https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Cache-Control. **Maps to:** a deferred item is a cached snapshot with a freshness TTL; on pickup past TTL, revalidate before "serving" (implementing) it; a `last_revalidated` stamp is the freshness marker.
- **Time-based invalidation.** Explicit TTL-then-revalidate guidance — https://oneuptime.com/blog/post/2026-01-30-time-based-invalidation/view — backs the issue's "trigger only past an age threshold (e.g. >30 days or >1 cycle)" heuristic. Cache-stampede prevention — https://oneuptime.com/blog/post/2026-01-30-cache-stampede-prevention/view — backs "one scoped research call per item, reuse the cache" (avoid revalidating the same item repeatedly within a window).
- **Agile backlog grooming / re-triage (the domain analog).** Backlog refinement explicitly re-evaluates aged backlog items for continued relevance before they enter a sprint — https://www.atlassian.com/agile/project-management/backlog-grooming. Work-queue analysis for stale queue items — https://bpdocs.blueprism.com/bp-7-2/en-us/work-queue-analysis.htm. **Maps to:** the deferred→active transition is exactly the "item leaves the backlog and enters active work" moment that grooming targets.

**Synthesis:** the issue is asking to apply a textbook **stale-while-revalidate + TTL invalidation** discipline (with a `last_revalidated` ETag-equivalent) to factory backlog items, and to graft the agile-grooming "re-evaluate before sprint" gate onto the deferred→active transition. Both are well-precedented; the design (age threshold + per-item scoped revalidation + stamp + cache reuse) is the standard shape and avoids the cache-stampede anti-pattern.

All URLs accessed 2026-06-09 via Perplexity `sonar-deep-research`.

---

## Verdict

> **VALID-NEW** — **Confidence: HIGH**
>
> No deferred-item revalidation gate exists anywhere in the pipeline. research-agent runs at *initial planning* (greenfield domain research, discovery idea scoring, feature Market Intelligence) but is **never conditioned on deferral** and is **skipped entirely for bug fixes** — so resurrected tech-debt, GAPs, blocked-then-unblocked requirements, and aged backlog items go straight to spec/implementation on stale snapshot context. The maintenance sequence (the main tech-debt source) has zero research-agent calls. The feature is genuinely new and well-motivated; the production-grade default makes it a real correctness gap (full VSDD rigor spent on potentially-obsolete work).
>
> **NEEDS-HUMAN sub-decisions (the issue's own open questions; surface at design time, not deferrable as work):**
> 1. **Age/cycle threshold** — always vs >30 days vs >N cycles. (Recommend a configurable threshold defaulting to >1 cycle OR >30 days, per RFC-5861 TTL precedent.)
> 2. **Automatic-at-pickup vs always-require-human-proceed** on the verdict. (Recommend: auto STILL-VALID proceeds; OBSOLETE always routes to human — matches the issue's AC.)
> 3. **Per-surface scope** — include Blocking Issues + Skip Log, or limit to tech-debt / GAP / backlog. (Recommend start with tech-debt + GAP + Product Backlog; add Blocking/Skip in a follow-up if signal warrants.)
>
> These are genuine human/architect policy calls (thresholds, automation level), not deferrable implementation work — the gate itself must be built in full.

---

## Recommended Approach (for zero re-research later)

| Item | Detail |
|---|---|
| **Owning agents** | `vsdd-factory:architect` (gate design + insertion-point + threshold policy) → `vsdd-factory:story-writer` / orchestrator-sequence edits → `vsdd-factory:research-agent` is the *invoked* specialist at runtime. STATE.md / template field additions route to `vsdd-factory:state-manager` and the template owner. |
| **Insertion points (issue's, validated)** | (1) **Feature-sequence — primary:** add a "Deferred-Item Revalidation" pre-step *before* Market Intelligence in `agents/orchestrator/feature-sequence.md`, gated on `item.deferred == true` (or age threshold), and crucially **also running for bug fixes** (since Market Intelligence skips those). (2) **Maintenance → refactor cycle:** revalidate P0 tech-debt at promotion in `agents/orchestrator/maintenance-sequence.md` (or `skills/track-debt`), not only at eventual human selection. (3) **Discovery cooldown lift:** attach a "has the landscape changed since deferral?" research sub-task when a deferred idea exits cooldown in `agents/orchestrator/discovery-sequence.md:76-77`. |
| **Key files** | `agents/orchestrator/feature-sequence.md` (new pre-step) · `agents/orchestrator/maintenance-sequence.md` (P0-promotion revalidation) · `agents/orchestrator/discovery-sequence.md` (cooldown-lift research) · `templates/tech-debt-register-template.md` + relevant story/GAP templates (add a `last_revalidated` field, the ETag-equivalent) · optionally `templates/state-template.md` for Product Backlog stamps. A `skills/revalidate-deferred-item/` skill could encapsulate the research-agent prompt + verdict handling for reuse across all three insertion points. |
| **Runtime prompt shape (from issue, refined)** | "This item was deferred on `<date>` (source: `<TD-NNN / GAP-NNN / backlog / blocking-issue>`). Revalidate whether it is still worth working on: (1) market/competitive-landscape shift? (2) dependency/platform/library change — is there now a built-in/better solution? (3) underlying problem already resolved or made moot? (4) user/product needs evolved? Return **STILL-VALID** (proceed) or **OBSOLETE/SUPERSEDED** (recommend re-defer/drop) with cited evidence." |
| **Cheapness / anti-stampede** | Trigger only when `item.deferred == true` AND past age threshold; one scoped research call per item; **reuse the research-cache** (`.factory/research/`); record `last_revalidated` + verdict on the item so repeated pickups within a window skip re-research (RFC-5861 stale-while-revalidate + stampede-prevention precedent). Fresh items skip the gate entirely — no new friction on the normal greenfield/feature path. |
| **Risks** | (a) **Over-triggering** wedges every pickup — the age threshold + stamp are mandatory, not optional. (b) **Bug-fix bypass**: the gate must explicitly run for deferred *bug fixes* despite Market Intelligence skipping them — easy to miss if grafted naively onto the existing Market Intelligence step. (c) **Verdict-routing**: OBSOLETE must hard-route to human (not auto-drop) per the issue AC. (d) **Cache coherence**: `last_revalidated` window must be short enough to catch fast-moving landscapes. |
| **Dependencies** | None blocking. Reuses the existing research-agent + research-cache. Coordinates with the deferral-surface templates (which #129 also touches — see below). |
| **Cross-issue coupling with #129** | #129 (production-grade default canonicalization) tightens *when* items may be deferred and adds `human_directed` + `target_story` fields to the tech-debt register. #171 governs *what happens when a deferred item is picked back up*. They are complementary: #129's stricter deferral metadata (date, source, target-story) gives #171's revalidation gate exactly the snapshot context it needs. Recommend sequencing #129 first (it shapes the deferral record), then #171 (it consumes it), or designing the `last_revalidated` field alongside #129's register-schema change to avoid two template churns. |

---

## Research Methods

| Tool | Queries | Purpose |
|------|---------|---------|
| **Perplexity perplexity_research (PRIMARY)** | 1 (shared with #175) | Staleness-revalidation patterns: stale-while-revalidate (RFC 5861), TTL/time-based invalidation, cache-stampede prevention, agile backlog grooming — primary-source survey |
| Read | 2 | feature-sequence, reference research file |
| Grep | 3 | confirm research-agent absent from maintenance-sequence; cooldown is re-proposal-throttle not revalidation; no revalidation primitive / prior closure |
| Glob | 1 | enumerate workflows/skills surfaces |
| Training data | 0 areas | All staleness patterns sourced externally |

**Total MCP tool calls:** 1 (research call shared across #175 + #171 version/staleness theme)
**Training data reliance:** LOW — pipeline-gap claims cited with file:line; external patterns verified against RFC 5861, Fastly/Varnish, Atlassian backlog-grooming primary docs.
