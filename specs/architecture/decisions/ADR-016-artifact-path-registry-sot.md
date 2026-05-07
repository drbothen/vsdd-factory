---
document_type: adr
adr_id: ADR-016
status: accepted
accepted_date: 2026-05-06
date: 2026-05-06
cycle: v1.0-feature-engine-discipline-pass-1
subsystems_affected: [SS-04, SS-06]
supersedes: null
superseded_by: null
---

# ADR-016: Artifact Path Registry as Single Source of Truth for `.factory/` Canonical Paths

## Context

### Path-invention errors during agent dispatch

During the `v1.0-feature-engine-discipline-pass-1` F1 dispatch, the architect
agent invented a `feature-deltas/` directory under `.factory/` that does not
exist in the canonical artifact layout. This was a live demonstration of a
broader pattern: writing agents infer artifact paths from context rather than
consulting a canonical source, causing artifact drift that is silent at write
time and only discovered during later retrieval or gate validation.

### The `.factory/` artifact discipline gap

The `.factory/` directory is the authoritative artifact store for all VSDD
pipeline outputs. Its subdirectory layout encodes semantic structure (cycles,
specs, verification-properties, behavioral-contracts, stories). Without a
machine-readable registry of canonical patterns, agents must infer structure
from examples — and inference errors compound across cycles.

### Defense-in-depth motivation

A single enforcement mechanism — whether cultural (prompt instructions), skill-
level (path resolution before writing), or mechanical (WASM hook at write time)
— provides a single point of failure. Each layer fails for a different reason:
a prompt can be ignored under context pressure; a skill can be bypassed when a
raw Write call is made; a hook can be disabled in unusual configurations. The
defense-in-depth model requires all three layers to be independently implemented
and traceable to the same registry source.

### OQ-5 resolution: immediate `block` enforcement

The F2 open question on enforcement_level promotion was resolved before ADR
finalization. Per OQ-5, the initial registry ships all entries at
`enforcement_level: block`. Story C delivery MUST run the `relocate-artifact`
skill in dry-run mode and apply all corrections before registering the
`validate-artifact-path` hook — this preflight ensures no legitimate artifact
is blocked by the newly active hook. Phased warn→block promotion is rejected
(see Alternatives Considered).

---

## Decision

A YAML registry at `plugins/vsdd-factory/config/artifact-path-registry.yaml`
is the **single source of truth** for canonical `.factory/` artifact locations.

Three enforcement layers are defined. All three MUST consult the registry at
runtime — none may embed a duplicate path list.

### Layer 1 — WASM hook `validate-artifact-path` (mechanical block)

The hook fires on `PreToolUse` for `Write` and `Edit` tool events. It reads
`tool_input.file_path` from the `HookPayload`. For any path that begins with
`.factory/`, the hook validates the path against `canonical_path_pattern`
entries in the registry. Paths that match no registered pattern, or match a
pattern with `enforcement_level: block`, are rejected via
`HookResult::block_with_fix` using the canonical Why/Fix/Code format
(HOST_ABI.md Block-message convention). Non-`.factory/` paths pass through
without inspection (early-exit Continue).

Capability requirements: `read_file` (for registry load), `emit_event` (for
`hook.block` telemetry). YAML parsing: `serde_yaml` is acceptable per OQ-1
resolution.

Implementation path:
`/Users/jmagady/Dev/vsdd-factory/crates/hook-plugins/validate-artifact-path/`

Build output:
`plugins/vsdd-factory/hook-plugins/validate-artifact-path.wasm`

### Layer 2 — Creation skills (structured resolution)

Each writing skill whose procedure produces a `.factory/` artifact MUST, before
any Write operation, read `plugins/vsdd-factory/config/artifact-path-registry.yaml`
via the `Read` tool and resolve the target path against the matching
`canonical_path_pattern`. If no pattern matches the artifact type, the skill
MUST refuse the write and emit the standard error:

> Artifact type `<type>` has no canonical path in
> `plugins/vsdd-factory/config/artifact-path-registry.yaml`. Consult the
> registry or use `/vsdd-factory:relocate-artifact` to determine the correct
> location. Do not invent directory names.

Skills affected (confirmed): `create-adr`, `create-prd`, `create-architecture`,
`create-domain-spec`, `create-story`, `create-brief`, `register-artifact`.
Additional `create-*`, `scaffold-*`, and `register-*` skills require a targeted
survey in F4 (120+ skills in `plugins/vsdd-factory/skills/`).

### Layer 3 — Writing-agent prompt preambles (cultural reinforcement)

The following paragraph is added to every writing-agent system prompt
(architect, product-owner, business-analyst, story-writer, technical-writer,
and any agent that produces `.factory/` artifacts):

> Before any Write under `.factory/`, verify the target path matches a pattern
> in `plugins/vsdd-factory/config/artifact-path-registry.yaml`. If unsure, use
> the `register-artifact` skill or list existing structure first. Do not invent
> directory names.

### Registry schema

```yaml
artifacts:
  - artifact_type: <string>
    canonical_path_pattern: <string>   # placeholders: {cycle-id}, {bc-id},
                                       # {phase}, {story-id}, {ss-id}, etc.
    description: <string>
    enforcement_level: block | warn | advisory
```

`enforcement_level` semantics:
- `block` — Layer 1 hook emits `HookResult::block_with_fix`. Write is rejected.
- `warn` — Layer 1 hook emits `hook.warn` event + stderr message, then
  `HookResult::Continue`. Write proceeds.
- `advisory` — Layer 1 hook logs only. No stderr, no event. Write proceeds.

All entries in the initial registry ship with `enforcement_level: block`
(OQ-5 resolution). The field is retained in the schema for future flexibility
(e.g., advisory classification for ephemeral scratch paths).

### Relocation preflight (Story C delivery requirement)

Before `validate-artifact-path` is registered in `hooks-registry.toml`, Story C
MUST run the `relocate-artifact` skill with `--apply` to ensure all existing
`.factory/` artifacts conform to registry patterns. The preflight is complete
when `relocate-artifact --dry-run` reports zero misplaced artifacts. Only then
is the hook entry added to `hooks-registry.toml`.

---

## Rationale

A YAML registry provides:

1. **Mechanical enforceability.** A WASM hook can parse a YAML file at runtime;
   it cannot parse a prose document. The registry schema is deliberately simple
   enough to parse with `serde_yaml` or a hand-written subset parser.
2. **Single mutation point.** Adding a new artifact type requires one registry
   edit. The hook, the skills, and the prompt preambles all pick up the change
   automatically on their next invocation.
3. **Auditable drift detection.** The `relocate-artifact` skill can compare
   existing `.factory/` content against registry patterns and produce a
   deterministic report of misplaced artifacts. This is not possible when path
   knowledge is embedded in multiple skill files.

---

## Subsystem Assignments

**SS-04 (Plugin Ecosystem):** Referencing SS-04 because the `validate-artifact-path`
WASM hook crate at `crates/hook-plugins/validate-artifact-path/` is a new plugin
in the SS-04 plugin ecosystem. The registry YAML at
`plugins/vsdd-factory/config/artifact-path-registry.yaml` is infrastructure for
that plugin.

**SS-06 (Skill Catalog):** Referencing SS-06 because all seven confirmed creation
skills and `register-artifact` are SS-06-scoped skills. Their modification to
consult the registry is an SS-06 behavior change.

---

## Alternatives Considered

### (a) Prompt-only cultural enforcement

Rejected. Prompt instructions can be ignored under context pressure (long context
windows, multi-step tasks, agent substitution). Cultural enforcement is necessary
but not sufficient. Provides no mechanical teeth.

### (b) Phased warn→block rollout

Rejected per OQ-5. The user explicitly requested immediate block enforcement.
The relocation preflight (Story C delivery requirement above) serves the same
false-positive mitigation purpose as a warn phase, but runs once at delivery
time rather than permanently reducing enforcement strength. The warn phase adds
ongoing operational complexity (tracking promotion criteria across cycles)
without improving the final state.

### (c) Per-skill hardcoded paths

Rejected. Violates the single-source-of-truth principle. When artifact layouts
change, all skill files require manual updates — exactly the drift problem this
ADR is designed to prevent. Additionally, a WASM hook cannot reference skill
files at runtime; it requires a machine-readable format.

---

## Consequences

- **Any agent attempting a `.factory/` write outside registered patterns is
  blocked.** This is the desired steady-state behavior.
- **New artifact types require registry updates** as a normal part of feature
  delivery. This is not a hot-path concern — artifact types are stable within
  a feature cycle and are added deliberately.
- **Story C must include a relocation preflight** that resolves all pre-existing
  misplaced artifacts before hook registration. This is a one-time cost.
- **The `relocate-artifact` skill is the third consumer of the registry.** It
  reads `artifact-path-registry.yaml` for canonical patterns and never embeds
  its own path list. This preserves the single-source-of-truth invariant.
- **`validate-artifact-path` fires on every Write/Edit to `.factory/`.** The
  performance impact is bounded by the registry size (typically < 20 entries)
  and a simple substring/glob match. No measurable latency concern at current
  scale.

---

## Verification Properties

- VP-069: `validate-artifact-path` registry-load purity (proptest, P1)
- VP-070: `validate-artifact-path` path-pattern matching is pure and deterministic
  (kani, P1)
- VP-072: Single-source-of-truth invariant — no skill or hook embeds a
  duplicate path list (integration/bats, P1)

See `/Users/jmagady/Dev/vsdd-factory/.factory/specs/verification-properties/`
for full VP definitions.

---

## Decision Log Reference

| Decision | ID | Rationale |
|----------|----|-----------|
| Cycle opened with path-governance cluster | D-336 | Triggering event: `feature-deltas/` invention |
| WASM-only hooks for this cycle | D-337 | No new Bash hook debt |
| Immediate block enforcement (OQ-5) | F2 OQ-5 | Relocation preflight replaces warn phase |
