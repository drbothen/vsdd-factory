---
document_type: adr
adr_id: ADR-011
status: accepted
date: 2026-04-26
subsystems_affected: [SS-07, SS-09]
supersedes: null
superseded_by: null
---

# ADR-011: Dual hooks.json + hooks-registry.toml During Migration

## Context

v1.0 introduced a clear separation between two routing concerns that were previously
conflated in a single `hooks.json` file:

1. **Claude Code harness wiring** — the JSON file Claude Code reads to know which
   executable to invoke on which hook event. This must be in the format Claude Code
   expects: a JSON object with `hooks` keys mapping event types to arrays of hook
   entries, each with a `command` field pointing to an executable.

2. **Dispatcher internal routing** — the TOML file the compiled dispatcher reads at
   runtime to know which WASM plugins to load, which events/tools they match, their
   priority tiers, timeout overrides, capability grants, and error policies.

In v0.79.x, `hooks.json` served both purposes simultaneously: it was both the
harness wiring document and the implicit routing table. Each entry in `hooks.json`
invoked a distinct bash script, and the script path was the entire routing
specification. With v1.0's single dispatcher binary as the sole `hooks.json`
entry point, a new routing layer was needed. The question was how to manage this
during the transition period when both the old behavior and the new must coexist.

DRIFT-004 in the project STATE tracks this dual-table situation as a known,
intentional architectural state. The DRIFT note ("hooks.json + hooks-registry.toml
dual routing tables | MEDIUM-HIGH | cutover before rc.1") records the planned
resolution point rather than an unintended divergence.

## Decision

During the v1.0 migration period (beta releases through rc.1), two routing tables
coexist with distinct, non-overlapping purposes:

- `plugins/vsdd-factory/hooks/hooks.json` (and its `.platform` variants) — Claude
  Code harness wiring only. Contains a single entry pointing to the dispatcher
  binary. Never used by the dispatcher itself. Written by the activation skill
  (ADR-009); gitignored; source of truth is `hooks.json.template`.

- `plugins/vsdd-factory/hooks-registry.toml` — dispatcher routing only. Declares
  every WASM plugin, its event/tool match patterns, priority, timeout, capabilities,
  and error policy. Never read by the Claude Code harness. Committed and operator-editable.

The cutover from dual-table to single-table is planned for rc.1 (DRIFT-004 disposition
"L-P0-002 cutover before rc.1") when the dispatcher assumes full ownership of routing
and `hooks.json` becomes a pure harness wiring artifact with no routing semantics.

## Rationale

The two files have fundamentally different audiences and update cadences:

`hooks.json` is written once at activation time by a skill, not by humans. Its format
is constrained by what Claude Code accepts. Operators never edit it directly; the skill
regenerates it on each activation. It is platform-specific (per ADR-009) and gitignored.

`hooks-registry.toml` is the operator's configuration surface. Operators edit it to
add plugins, adjust priorities, grant capabilities, or change error policies. It is
platform-agnostic (the same TOML works on all platforms because it references
`.wasm` files, not platform binaries). It is committed to the repository.

Merging these into a single file would require either: (a) Claude Code learning to
parse TOML (not feasible), or (b) the dispatcher parsing JSON with TOML semantics
embedded (wrong format for operator authoring — see ADR-004), or (c) maintaining a
dual-format file that serves neither audience well.

The dual-table approach during migration is the correct minimal footprint for the
beta period. It makes the boundaries explicit and allows each file to evolve
independently. The DRIFT-004 tracking ensures the architectural debt is visible and
has a planned resolution, not silently accumulated.

The distinction is confirmed by the design: `hooks.json` is a harness wiring file
(analogous to a systemd unit file pointing to a binary), while `hooks-registry.toml`
is a routing configuration file (analogous to the application's internal config).
These are different layers with different change surfaces.

## Consequences

### Positive

- `hooks.json` format remains exactly what Claude Code expects; no harness
  compatibility risks are introduced by routing changes in `hooks-registry.toml`.
- Operators can add, remove, or reconfigure WASM plugins by editing
  `hooks-registry.toml` alone, with no interaction with the harness-wiring layer.
- The separation makes the migration path to rc.1 safe: the cutover affects only
  how `hooks.json` is generated, not the dispatcher's routing logic.

### Negative / Trade-offs

- Two files with overlapping conceptual territory create operator confusion during
  the beta period. Documentation must clearly explain which file controls what.
- DRIFT-004 carries MEDIUM-HIGH severity because the dual-table state is temporary
  debt that must be resolved before rc.1. If not resolved, future operators may
  attempt to add routing logic to `hooks.json`, breaking the architectural boundary.
- The gitignored status of `hooks.json` means CI cannot lint it directly; the
  `.platform` variants are the only committed artifacts representing the harness
  wiring layer.

### Status as of v1.0.0-beta.5

IN-EFFECT (migration period). Both files are present and serving their distinct
purposes. DRIFT-004 is open with planned resolution at rc.1 (L-P0-002). The
`hooks.json.template` and `.platform` variants are committed; `hooks.json` is
gitignored. `hooks-registry.toml` is committed and routed through by the dispatcher.

## Alternatives Considered

- **Single TOML routing file that also serves as hooks.json:** Requires Claude Code
  to parse TOML. Claude Code does not support TOML for `hooks.json`. Rejected.
- **Single JSON file with dispatcher routing embedded:** Embeds TOML-style registry
  declarations in JSON. Poor operator authoring experience; JSON lacks comment support
  (see ADR-004). Rejected.
- **Migrate to single file immediately at beta.1:** Merge routing into `hooks.json`
  at the start of v1.0. Rejected: requires stabilizing the routing format before
  the dispatcher has been dogfooded; beta period is explicitly for discovering
  format issues.

## Source / Origin

- **Master design doc:** `.factory/legacy-design-docs/2026-04-24-v1.0-factory-plugin-kit-design.md`
  lines 88–114 (file tree showing `hooks.json.template`, `.platform` variants, and
  `hooks-registry.toml` as separate files), lines 44–53 (dual-file decision in the
  Decisions section).
- **State tracking:** `.factory/STATE.md` line 119 (DRIFT-004 entry — "hooks.json +
  hooks-registry.toml dual routing tables | MEDIUM-HIGH | L-P0-002 cutover before rc.1").
- **Code as-built:** `plugins/vsdd-factory/hooks/hooks.json.template` (harness wiring
  source of truth), `plugins/vsdd-factory/hooks-registry.toml` (dispatcher routing).
- **Skill documentation:** `plugins/vsdd-factory/skills/activate/SKILL.md` step 6
  (activation writes `hooks.json` from `.platform` variant, never touches `hooks-registry.toml`).
