---
document_type: feature-delta-analysis
cycle_id: v1.0-feature-plugin-async-semantics-pass-1
phase: F1
status: draft
created: 2026-05-07
author: architect
version: "1.0"
inputs:
  - plugins/vsdd-factory/hooks-registry.toml
  - plugins/vsdd-factory/hooks/hooks.json.darwin-arm64
  - crates/factory-dispatcher/src/registry.rs
  - crates/factory-dispatcher/src/routing.rs
  - crates/factory-dispatcher/src/invoke.rs
  - crates/factory-dispatcher/src/engine.rs
  - .factory/specs/architecture/ARCH-INDEX.md
  - .factory/specs/behavioral-contracts/BC-INDEX.md
  - .factory/specs/verification-properties/VP-INDEX.md
  - /Users/jmagady/Dev/prism/.factory/logs/dispatcher-internal-2026-05-07.jsonl
input-hash: "[to-be-computed-by-state-manager]"
---

<!-- F-P25-007 / L-P25-002 carve-out: This is a Phase F1 architect proposal preserved as historical audit record. The pseudocode symbols (run_tiers, spawn_detached, run_event, drain_async_tasks, etc.) are PRE-MERGE PLANNING VOCABULARY. Downstream specs MUST replace these with merged-code symbols per L-P24-002 + L-P21-001. This file itself is exempt from L-P21-001 / L-P23-001 fabricated-symbol sweeps under the Phase F1 architect-proposal carve-out (extending L-P24-001 brownfield Phase 0 carve-out to greenfield F1 architect proposals). See L-P25-002 codification at lessons.md. -->

# F1 Delta Analysis — Plugin Async Semantics (Pass 1)

## 1. Trigger / Motivation

**Seed evidence from prism project audit (2026-05-07).**

Log source: `/Users/jmagady/Dev/prism/.factory/logs/dispatcher-internal-2026-05-07.jsonl`
(212,398 lines, 21,721 unique dispatcher trace IDs, 92,385 total plugin invocations)

### Observed failures

| Plugin | Invocations | exit_code=2 (block) | Block rate |
|--------|-------------|---------------------|------------|
| validate-template-compliance | 1,011 | **55** | 5.4% |
| validate-table-cell-count | ~1,011 | 200 | 19.8% |
| validate-changelog-monotonicity | ~1,011 | 169 | 16.7% |
| validate-state-size | ~1,011 | 111 | 11.0% |
| validate-input-hash | ~1,011 | 14 | 1.4% |
| validate-factory-path-root | ~1,011 | (unmeasured) | — |

All of these plugins carry `on_error = "block"` or `on_error = "continue"` in
`hooks-registry.toml`. All are wired to the PostToolUse event. Every single one of
the 55 validate-template-compliance blocks was **silently discarded** — the user
received no notification, the dispatcher's exit code was never surfaced to Claude Code,
and no remediation was possible because Claude Code had already proceeded.

### Root cause

`hooks.json.darwin-arm64` declares PostToolUse with `"async": true`:

```json
"PostToolUse": [{"hooks": [{"type": "command",
  "command": "...factory-dispatcher", "timeout": 10000, "async": true}]}]
```

When Claude Code executes a hook envelope with `async: true`, it **fires and forgets**:
the dispatcher process runs detached, its exit code is never read, and
`HookResult::block_with_fix` output on stdout is discarded before Claude Code can
act on it. The contract `on_error = "block"` in the registry is therefore
**structurally inert** for every plugin bound to PostToolUse, Stop, SubagentStop,
SessionStart, SessionEnd, WorktreeCreate, WorktreeRemove, or PostToolUseFailure —
all of which are declared async in the current envelope (source of truth:
`plugins/vsdd-factory/hooks/hooks.json.darwin-arm64`).

Only PreToolUse and PermissionRequest are declared synchronous today.

---

## 2. Current State

```
Claude Code
    │ PostToolUse event fires (async: true in hooks.json)
    ▼
factory-dispatcher  ← exit code DISCARDED by Claude Code
    │
    ├── routing.rs:match_plugins()  — matches ~15-30 plugins per Edit/Write
    ├── routing.rs:group_by_priority()  — groups by priority tier
    │
    └── per tier: plugins run in parallel (ADR-008)
            validate-template-compliance (on_error=block, priority=400)
                exit code 2 → stdout: HookResult::block_with_fix(...)
                ← STDOUT NEVER READ  ← EXIT CODE NEVER SEEN
            validate-input-hash (on_error=block, priority=310)
                exit code 2 → (same fate)
            validate-factory-path-root (on_error=block, priority=280)
                exit code 2 → (same fate)
            capture-commit-activity (on_error=continue, priority=110)
                exit code 0 → (fine — continue is correct here)
```

The dispatcher itself is correct: it routes, invokes, and collects exit codes. The
bug is one layer above — the Claude Code envelope setting makes the entire dispatcher
call advisory regardless of what plugins return.

---

## 3. Proposed State

Four interlocking changes restore the invariant that `on_error = "block"` actually
blocks:

### Change 1 — hooks.json removes `async: true` from PostToolUse and siblings

The Claude Code envelope for PostToolUse, Stop, SubagentStop, PostToolUseFailure,
WorktreeCreate, WorktreeRemove becomes synchronous (no `async` key). SessionStart and
SessionEnd remain async+once (they have no blocking plugins and fire before any user
action can be blocked).

### Change 2 — hooks-registry.toml schema bump: v1 → v2, adds per-plugin `async: bool`

New field in `RegistryEntry`:
```toml
async = false   # default; omit = sync
```

Registry defaults remain `async = false`. Telemetry plugins (capture-commit-activity,
capture-pr-activity, session-*-telemetry) declare `async = true`.

`registry.rs` bumps `REGISTRY_SCHEMA_VERSION` to 2. `validate()` accepts the new
field. Old v1 registries are rejected (hard error, existing pattern per line 29 in
`registry.rs`).

### Change 3 — Dispatcher partitions matched plugins into sync and async groups

In `engine.rs` / execution path (the `run_event` dispatch loop):

```
matched_plugins = match_plugins(registry, payload)

sync_group  = [p for p in matched_plugins if !p.async]
async_group = [p for p in matched_plugins if  p.async]

// Sync group: run in priority-tier order; any block → exit 2
sync_results = run_tiers(sync_group)
exit_code = if any_block(sync_results) { 2 } else { 0 }

// Async group: spawn tokio tasks (or OS threads), fire-and-forget
// log to events; never gate the user
for p in async_group { spawn_detached(p) }

return exit_code  ← Claude Code reads this
```

ADR-008 (parallel-within-tier) continues to govern sync group execution. Async group
parallelism is bounded by a configurable worker pool (see OQ-2).

### Change 4 — CI lint invariant: `on_error = "block"` ⇒ `async = false`

A new bash validation hook (or bats CI test) scans `hooks-registry.toml` and asserts
no entry has both `on_error = "block"` and `async = true`. This prevents future
regressions where a new plugin is classified async but tagged block.

### Partitioned execution sequence diagram

```
Claude Code (synchronous call)
    │
    ▼
factory-dispatcher
    ├── SYNC GROUP (run tiers, await all)
    │       validate-template-compliance  (on_error=block, async=false)
    │           → exit 2: block_with_fix surfaced to user
    │       validate-input-hash          (on_error=block, async=false)
    │       validate-factory-path-root   (on_error=block, async=false)
    │       convergence-tracker          (on_error=continue, async=false)
    │       [any block] → exit 2
    │
    ├── ASYNC GROUP (spawn detached, do not await)
    │       capture-commit-activity  (async=true)
    │       capture-pr-activity      (async=true)
    │       [always returns exit 0 to Claude Code regardless]
    │
    └── exit code → Claude Code reads → block or proceed
```

---

## 4. Subsystem Impact

Source: `ARCH-INDEX.md` Subsystem Registry (current_cycle v1.0-feature-engine-discipline-pass-1).

| Subsystem | Impact | Affected Artifacts |
|-----------|--------|-------------------|
| SS-01 Hook Dispatcher Core | **CRITICAL** | `registry.rs` (new `async` field), `routing.rs` (partition logic), execution path in `main.rs` or equivalent dispatch loop |
| SS-07 Hook Bash Layer | **HIGH** | `hooks-registry.toml` (schema v1→v2, `async` field per plugin), existing hook classification decisions |
| SS-09 Configuration and Activation | **HIGH** | `hooks.json.template` + all 5 platform variants (remove `async: true` from PostToolUse/Stop/SubagentStop/PostToolUseFailure/Worktree* entries); `hooks.json.darwin-arm64` confirmed affected |
| SS-02 Hook SDK and Plugin ABI | **LOW** | `HookPayload` struct: no change. `RegistryEntry` exposed via SDK: `async` field may need SDK-side visibility if plugins inspect their own classification (unlikely — assess in F2) |
| SS-04 Plugin Ecosystem | **LOW** | No plugin WASM code changes. Registry entries for telemetry plugins flip `async = true` |
| SS-10 CLI Tools and Bin | **NONE** | Not affected |
| SS-05/SS-06/SS-08 | **NONE** | Not affected |

SS-09 note: `hooks.json.template` is the single-source for CI generation (per SS-09 Modules table, line 45 in SS-09-config-activation.md). The template change propagates to all 5 platform variants during the next CI run. The activation skill does not need modification.

---

## 5. BC Touchpoints

Source: `BC-INDEX.md` (total_bcs: 1,943; version 1.18).

### Existing BCs requiring amendment

| BC ID | Current Scope | Amendment Needed |
|-------|--------------|-----------------|
| BC-1.05.* (dispatcher routing family) | Match + execution contract | Add `async` field to matched-plugin representation; document partition logic |
| BC-9.01.002 (or equivalent hooks.json schema BC) | hooks.json event declarations | Record that PostToolUse, Stop, SubagentStop, PostToolUseFailure, Worktree* change from async to sync |
| BC-7.* (hooks-registry.toml schema BCs) | schema_version = 1 | Bump to schema_version = 2; document `async` field semantics |

The exact BC IDs within BC-7 for registry schema BCs must be located in BC-INDEX during F2 (the BC-INDEX was read to line 60; the BC-7 block starts further down). Target: any BC that references `schema_version = 1` in SS-07 scope.

**Amendment vs. new-BC threshold:** Existing BCs are amended (not replaced) when the
change is a behavioral constraint on an existing contract boundary (same module, same
invariant class, new precondition). New BCs are created when a new behavioral
obligation surfaces with no prior anchor.

### New BCs needed (2)

| Proposed ID | Scope | Justification |
|------------|-------|---------------|
| BC-1.NN.001 (SS-01) | Dispatcher partition contract: sync group runs await-all; async group fire-and-forget; exit code is sync group aggregate | New behavioral obligation — the partition model has no prior BC anchor |
| BC-7.NN.001 (SS-07) | Per-plugin `async: bool` declaration semantics in hooks-registry.toml v2 schema; CI invariant `on_error=block` ⇒ `async=false` | New schema obligation requiring formal contract |

The two new BCs replace the current informal assumption that all dispatched plugins
are implicitly synchronous (an assumption that proved incorrect in production).

---

## 6. ADR Touchpoints

Source: `ARCH-INDEX.md` Architecture Decisions table (next sequential: ADR-019).

### New ADR (1)

**ADR-019 — Async Semantics at Registry Layer, Not Envelope Layer**

Decision: the Claude Code envelope becomes uniformly synchronous for blocking event
types; per-plugin async classification lives in `hooks-registry.toml`, not in
`hooks.json`. The dispatcher partitions matched plugins into sync and async groups
at runtime.

Justification: the envelope layer is a blunt instrument — one `async: true` flag
silences every plugin bound to that event, regardless of individual `on_error` intent.
Moving async declaration to the registry gives per-plugin precision while keeping
the Claude Code contract surface minimal.

Subsystems: SS-01, SS-07, SS-09.

### Prior ADRs to review (none superseded, one to annotate)

ADR-008 (Parallel-within-tier, sequential-between-tier execution) continues to
govern sync group execution unchanged. An annotation should be added noting that
async group plugins are excluded from the tier ordering model entirely — they are
unordered, fire-and-forget. This is an annotation, not a supersession.

ADR-011 (Dual hooks.json + hooks-registry.toml during migration) is relevant context.
No change needed — the dual routing table arrangement is orthogonal to this proposal.

No prior ADR explicitly set hooks.json async semantics; the current `async: true`
was an undocumented authoring convention. ADR-019 formalizes the decision retroactively
and specifies the correct future state.

---

## 7. VP Touchpoints

Source: `VP-INDEX.md` (total_vps: 76; next available: VP-077).

### New VPs proposed (2)

| Proposed ID | Description | Module | Method | Phase |
|-------------|-------------|--------|--------|-------|
| VP-077 (SS-01) | Dispatcher partition purity: `partition_plugins(matched, registry)` is a pure fn — given same input, always produces same (sync_group, async_group) split | factory-dispatcher/routing | kani-proof | P1 |
| VP-078 (SS-07) | CI registry lint: no entry has both `on_error = "block"` and `async = true` | hooks-registry.toml / lint hook | integration | P1 |

VP-077 is a natural fit for Kani because `partition_plugins` will be a pure,
deterministic function with no I/O — exactly the category Kani verifies well.

VP-078 is an integration-test VP (mechanical scan of the registry file), analogous
to VP-072 (cross-cutting single-source invariant).

### Existing VPs requiring scope amendment

VP-001 (Tier Execution Is Sequential; Intra-Tier Is Parallel) currently assumes all
matched plugins enter the tier model. Amendment needed: scope the VP to the sync
group only; async group plugins are explicitly excluded from tier ordering.

VP-002 (Plugin Crash or Timeout Does Not Block Sibling Plugins) — same scope
amendment: this invariant holds within the sync group. Async group crashes are fire-
and-forget; the amendment must clarify that async group crashes emit an event but
do not affect the sync group's verdict.

---

## 8. PRD Touchpoints

This change is internal contract evolution: the external user-visible behavior is
that blocking hooks now *actually block* (i.e., the PRD's existing FR for
hook-enforced governance is now correctly implemented). No new functional requirement
exists. PRD amendment is NOT needed.

However, the PRD's NFR for latency (if any exists for PostToolUse hook overhead)
should be reviewed in F2 to ensure the sync group's expected latency is within
acceptable bounds. The current PostToolUse median latency in prism is ~29ms per
plugin (observed from prism dispatcher-internal log: `elapsed_ms: 29` for
validate-template-compliance exit_code=0 invocations). With 15-20 sync plugins per
Edit/Write tier-parallel execution, peak sync overhead is approximately 30-100ms
(limited by slowest plugin in each tier, not sum). This is within typical hook
latency budgets but requires measurement under the new model (see R-1 below).

---

## 9. Schema Migration

### hooks-registry.toml: schema_version 1 → 2

Migration is additive: the new `async` field has a safe default of `false`. Existing
entries without the field continue to behave as sync (no behavioral change for
PreToolUse and other already-sync events). The dispatcher's `validate()` function in
`registry.rs` currently hard-errors on schema_version != 1 (line 26-30). This must
change to hard-error on schema_version != 2 after the bump.

**Rollout sequence:**

1. **Phase A (additive):** Bump `REGISTRY_SCHEMA_VERSION` to 2. Add `async: bool`
   field to `RegistryEntry` struct (serde default `false`). Dispatcher loads v2
   registries; v1 registries produce a clear migration error with instructions.
   Implement partition logic in dispatcher (sync/async split). All plugins classified
   async=false by default → behavior unchanged from v1 except the code path is correct.

2. **Phase B (reclassify):** Update `hooks-registry.toml`: bump `schema_version = 2`.
   Set `async = true` for: `capture-commit-activity`, `capture-pr-activity`,
   `session-start-telemetry`, `session-end-telemetry`. All validator plugins with
   `on_error = "block"` remain `async = false` (default, can be explicit for clarity).

3. **Phase C (envelope flip):** Remove `"async": true` from PostToolUse, Stop,
   SubagentStop, PostToolUseFailure, WorktreeCreate, WorktreeRemove in
   `hooks.json.template`. Regenerate all 5 platform variants. This is the high-impact
   step — from this moment, the dispatcher's exit code gates Claude Code on every
   PostToolUse. Only deploy after Phase B is confirmed correct.

4. **Phase D (CI invariant):** Add bats test or lint hook enforcing
   `on_error = "block"` ⇒ `async = false`. Gate CI on this invariant.

**Backward compatibility:** A product installing v2 dispatcher with a v1 registry
produces a hard schema-version error. Products must migrate registry to v2 before
upgrading the dispatcher. The migration is mechanical (no content changes needed —
the `async` field defaults to false if absent, but adding `schema_version = 2` is
required to pass validation). The activation skill should detect v1 registry and
emit an actionable migration hint. See OQ-4.

---

## 10. Risk Assessment

### R-1 (HIGH) — Latency regression on PostToolUse sync conversion

**Scenario:** Edit/Write currently matches 30+ plugins in prism (post-audit count:
22+ PostToolUse entries in hooks-registry.toml, plus all-tool entries). After the
envelope flip, Claude Code blocks on the dispatcher for the duration of the slowest
tier's slowest plugin. Current median: 29ms; observed max: 409ms
(validate-template-compliance on a blocking check). In worst case with 4-5 tiers,
total sync latency could reach 500-800ms per Edit/Write.

**Mitigation:** Aggressive async classification of non-blocking plugins (see R-2);
measure P95 latency during Phase C canary before full rollout; consider a global
sync-group timeout cap as a safety valve.

### R-2 (MEDIUM) — Misclassified plugin tagged sync causes noticeable lag

**Scenario:** A telemetry plugin or slow-running advisory plugin is left with default
`async = false`, adding its full execution time to the user-visible sync path.

**Mitigation:** F2 produces a complete classification table for all 30+ PostToolUse
plugins. F5 adversarial review challenges every `async = false` classification with
latency justification. The CI lint (Phase D) prevents future regressions in the
opposite direction (async block plugins); a companion lint warning for sync plugins
exceeding a timeout threshold can be added.

### R-3 (MEDIUM) — Backward compatibility breaks on v2 dispatcher with v1 registry

**Scenario:** An existing install (prism, secops-factory, or another downstream)
upgrades the dispatcher binary but does not update hooks-registry.toml to v2. The
dispatcher hard-errors at startup, blocking all hooks.

**Mitigation:** Phase A and Phase B must ship together in the same release. The
migration guide must be explicit. The activation skill should detect the v1/v2
mismatch and surface a one-command fix. Downstream products need advance notice
(release notes + migration guide) before the version lands.

### R-4 (LOW) — Scope conflict with engine-discipline-pass-1 cycle

**Scenario:** The engine-discipline-pass-1 cycle (currently in F3-amendment phase,
D-362) produced `validate-artifact-path` and `validate-per-story-adversary-convergence`
hooks. If these hooks are classified async by default (schema v1 assumption), they
would break silently under the new model until classified.

**Mitigation:** The engine-discipline-pass-1 hooks are both `on_error = "continue"`,
not block. Their classification as `async = false` (sync default) is correct — they
should run synchronously and surface verdicts. No conflict; the schema migration
handles them correctly. Flag in F2 to confirm classification when those hooks
are registered.

### R-5 (LOW) — Downstream products with async-dependent assumptions

**Scenario:** prism or other downstream products have workflows that assume
PostToolUse is non-blocking (e.g., a heavy PostToolUse plugin that was intentionally
written with the async assumption). Converting to sync makes it user-blocking.

**Mitigation:** Phase B reclassification survey must cover all downstream product
registries before Phase C envelope flip. Flag in F2 for a multi-repo impact analysis.
Products with legitimately heavy PostToolUse plugins can explicitly set `async = true`
in their registry — the schema supports this.

---

## 11. Migration Path

| Phase | Scope | Risk | Gate |
|-------|-------|------|------|
| A — Schema bump + partition logic | `registry.rs`: schema_version 2, `async` field. Dispatch loop: partition code (all plugins still sync by default). No behavior change. | LOW | Existing bats tests green; registry parses correctly |
| B — Plugin reclassification | `hooks-registry.toml`: schema_version = 2; telemetry plugins → async=true | LOW | CI lint passes; all sync plugins have documented latency justification |
| C — Envelope flip | `hooks.json.template` + 5 platform variants: remove async:true from blocking events | **HIGH** | Phase A+B shipped; P95 latency measured in canary; no false-positive blocks observed |
| D — CI invariant | Lint: on_error=block ⇒ async=false | LOW | CI green; VP-078 harness instantiated |

Phase C is the point of no return — once the envelope is synchronous, every PostToolUse
that takes >timeout_ms will block the user. Do not ship Phase C without latency
measurement evidence from a staging run or prism canary.

---

## 12. Story Sketch (3-4 stories for F3 formalization)

**Story A — Dispatcher schema v2 + partition runtime (SS-01 + SS-07)**
Scope: `registry.rs` schema bump, `async` field parsing, partition logic in dispatch
loop, unit tests for `partition_plugins()`, bats integration tests for sync/async
split behavior. Does NOT flip the registry or envelope. Delivers Phase A.

**Story B — Plugin classification audit + registry v2 (SS-07)**
Scope: Set `schema_version = 2` in `hooks-registry.toml`. Classify all plugins
(assign `async = true` to telemetry set; leave validators at default `async = false`).
Delivers Phase B. Depends on Story A.

**Story C — Envelope flip + activation migration (SS-09)**
Scope: Modify `hooks.json.template`; regenerate 5 platform variants; update activation
skill with v1/v2 migration hint; update SS-09 BCs. Delivers Phase C. Depends on
Story B. HIGH risk — requires latency canary evidence before merge.

**Story D — CI lint invariant (SS-07, SS-10)**
Scope: Implement bats test or pre-commit hook asserting `on_error=block ⇒ async=false`;
add VP-078 harness. Delivers Phase D. Can be parallelized with Stories B/C.

---

## 13. Open Questions

**OQ-1:** Should async plugins still surface a notification on block decisions, even
if not user-blocking? (e.g., "FYI: hook X returned block verdict during async run;
here's what was wrong") This preserves diagnostic value without gating the user.
Async plugins currently never produce `on_error = "block"` (enforced by CI lint),
so block verdicts from async plugins would be defensive — likely a classifier mistake.
Recommendation: emit a `hook.async_block_discarded` warning event; do not surface
to user.

**OQ-2:** Should the dispatcher's parallelism for sync plugins be bounded (configurable
worker pool) or unbounded? Current behavior: tier-parallel, no pool cap. With 20+
sync plugins per PostToolUse on Edit/Write, unbounded could spawn 20+ concurrent
wasmtime instances simultaneously, each consuming significant memory. Recommend a
pool cap of 8 concurrent WASM instances as a default, configurable in
`hooks-registry.toml` `[defaults]`.

**OQ-3:** Is there a mechanism to mark a single plugin async at the hook envelope
(Claude Code side) without breaking the partition model? Short answer: no — Claude
Code's `async` flag is per-envelope, not per-plugin. The proposed model (envelope
synchronous; per-plugin async in registry) is the only way to achieve per-plugin
granularity given Claude Code's current hook API. This is the architectural thesis
of ADR-019. Document as a known constraint.

**OQ-4:** Does the schema bump force a coordinated release of dispatcher + registry?
Yes. A v2 dispatcher requires a v2 registry. The migration guide must state: "upgrade
`hooks-registry.toml` to `schema_version = 2` before deploying the new dispatcher
binary." The activation skill should detect the mismatch and halt with a migration
hint rather than crashing silently. This should be a BC on the activation skill
(SS-09 amendment).

**OQ-5:** What is the canonical test for "envelope is synchronous"? Claude Code's
hook execution is not directly testable in bats. The proxy: verify that the
dispatcher's exit code gates the CI test harness (which invokes dispatcher directly
without the async wrapper). For production: the absence of silent block events in
the dispatcher-internal log is the observable signal — measure via
`grep exit_code=2 dispatcher-internal-*.jsonl | wc -l` against user-observed behavior
after Phase C.

---

## 14. Recommendation

**Full Feature Mode F1–F7.**

Justification: This change modifies the dispatcher's core execution contract
(`registry.rs` schema, `routing.rs` partition logic, `main.rs` / `engine.rs` execution
path), touches 5 platform-specific deployment artifacts (`hooks.json.*`), requires
new BCs formalizing the partition model, requires ADR-019 to record the architectural
decision, and carries a latency regression risk that needs adversarial review (F5)
before Phase C ships. The blast radius on downstream products (prism, secops-factory)
makes a coordinated migration process essential.

Quick-dev-routing is not appropriate: the dispatcher contract change crosses
subsystem boundaries (SS-01, SS-07, SS-09), the schema bump introduces a hard
backward-incompatibility, and the Phase C envelope flip is a production-behavior
change that needs F4 measurement + F5 adversarial + F6 hardening gate before merge.

Summary for F2–F7:

| Phase | Work |
|-------|------|
| F2 | 1 ADR (ADR-019), 2 new BCs (SS-01 + SS-07), 2 new VPs (VP-077/078), amend VP-001/VP-002 scope, BC amendments for SS-07 schema + SS-09 activation |
| F3 | 4 stories (A–D) scoped above |
| F4 | Story A (dispatcher), then B (registry), then C (envelope) in sequence; D in parallel |
| F5 | Adversarial focus on latency regression, misclassification risk, backward compat |
| F6 | Kani harness for VP-077; bats for VP-078; latency benchmark Phase C canary |
| F7 | BC sign-off; VP harness instantiation; migration guide in release notes |

---

## 15. References

| Artifact | Path |
|----------|------|
| Registry schema | `/Users/jmagady/Dev/vsdd-factory/plugins/vsdd-factory/hooks-registry.toml` |
| Envelope file (darwin-arm64) | `/Users/jmagady/Dev/vsdd-factory/plugins/vsdd-factory/hooks/hooks.json.darwin-arm64` |
| Dispatcher registry parser | `/Users/jmagady/Dev/vsdd-factory/crates/factory-dispatcher/src/registry.rs` |
| Dispatcher routing | `/Users/jmagady/Dev/vsdd-factory/crates/factory-dispatcher/src/routing.rs` |
| Dispatcher invocation | `/Users/jmagady/Dev/vsdd-factory/crates/factory-dispatcher/src/invoke.rs` |
| Dispatcher engine (epoch/fuel) | `/Users/jmagady/Dev/vsdd-factory/crates/factory-dispatcher/src/engine.rs` |
| Architecture index | `/Users/jmagady/Dev/vsdd-factory/.factory/specs/architecture/ARCH-INDEX.md` |
| BC index | `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/BC-INDEX.md` |
| VP index | `/Users/jmagady/Dev/vsdd-factory/.factory/specs/verification-properties/VP-INDEX.md` |
| SS-09 config-activation | `/Users/jmagady/Dev/vsdd-factory/.factory/specs/architecture/SS-09-config-activation.md` |
| Prior F1 template | `/Users/jmagady/Dev/vsdd-factory/.factory/cycles/v1.0-feature-engine-discipline-pass-1/F1-delta-analysis.md` |
| Prism dispatcher log (seed evidence) | `/Users/jmagady/Dev/prism/.factory/logs/dispatcher-internal-2026-05-07.jsonl` |
| Pipeline state | `/Users/jmagady/Dev/vsdd-factory/.factory/STATE.md` |
