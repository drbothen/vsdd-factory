---
document_type: feature-delta-analysis
cycle_id: v1.0-feature-engine-discipline-pass-1
phase: F1
status: draft
created: 2026-05-06
author: architect
---

<!-- L-P25-002 carve-out: This is a Phase F1 (architect-proposal) artifact preserved as historical audit record. Pseudocode symbols within are PRE-MERGE PLANNING VOCABULARY. Downstream specs MUST replace with merged-code symbols per L-P21-001 + L-P24-002. This file itself is exempt from fabricated-symbol sweeps under the L-P25-002 F1-architect-proposal carve-out (extends L-P24-001 brownfield Phase 0 carve-out). Codified at lessons.md L-P25-002. -->

# F1 Delta Analysis — Engine Discipline Pass 1

## 1. Scope Summary

This cycle codifies two governance gaps that exist today:

**Cluster A — Per-Story Adversary Workflow**
Items 1–6 from original scope: formal adversary review after every story convergence,
with the adversary agent, the AGENT.md reconciliation requirement, and a WASM hook
(`validate-per-story-adversary-convergence`) that enforces the workflow gate.

**Cluster B — Artifact Path Governance**
Items 7–11 from expanded scope: a YAML path registry as the single source of truth
for canonical `.factory/` artifact locations, a WASM hook (`validate-artifact-path`)
that enforces writes against that registry, skill-layer updates that consult the
registry before writing, writing-agent prompt preambles that culturally reinforce
the constraint, and a relocation skill that detects and repairs misplaced artifacts.

The cycle name — `v1.0-feature-engine-discipline-pass-1` — anticipates that further
governance gaps may surface and warrant future passes.

---

## 2. Impact Boundary

### Changed artifact classes

| Class | Cluster A | Cluster B |
|---|---|---|
| Rust crate: `crates/hook-plugins/validate-per-story-adversary-convergence/` | NEW | — |
| Rust crate: `crates/hook-plugins/validate-artifact-path/` | — | NEW |
| YAML config: `plugins/vsdd-factory/config/artifact-path-registry.yaml` | — | NEW |
| Writing-agent .md prompts (architect, product-owner, business-analyst, story-writer, technical-writer) | — | MODIFIED |
| Skill: `plugins/vsdd-factory/skills/register-artifact/SKILL.md` | — | MODIFIED |
| Skill: `plugins/vsdd-factory/skills/create-adr/SKILL.md` | — | MODIFIED |
| Skill: `plugins/vsdd-factory/skills/create-prd/SKILL.md` | — | MODIFIED |
| Skill: `plugins/vsdd-factory/skills/create-architecture/SKILL.md` | — | MODIFIED |
| Skill: `plugins/vsdd-factory/skills/create-domain-spec/SKILL.md` | — | MODIFIED |
| Skill: `plugins/vsdd-factory/skills/create-story/SKILL.md` | — | MODIFIED |
| Skill: `plugins/vsdd-factory/skills/create-brief/SKILL.md` | — | MODIFIED |
| Skill: `plugins/vsdd-factory/skills/relocate-artifact/SKILL.md` | — | NEW |
| `hooks-registry.toml` (two new entries) | MODIFIED | MODIFIED |
| `plugins/vsdd-factory/hook-plugins/validate-per-story-adversary-convergence.wasm` | BUILD OUTPUT | — |
| `plugins/vsdd-factory/hook-plugins/validate-artifact-path.wasm` | — | BUILD OUTPUT |
| Per-story-delivery orchestrator workflow (Lobster YAML) | MODIFIED | — |
| AGENT.md files for adversary, story-writer, and convergence-checker roles | MODIFIED | — |

### Out-of-scope for this cycle

- No changes to `crates/factory-dispatcher/`
- No changes to `crates/hook-sdk/` (ABI remains at v1)
- No retirement of any existing bash hooks (Tier E migration continues independently)
- No changes to `.factory/specs/` artifacts from `v1.0-brownfield-backfill`

### Overlap with `validate-factory-path-root.sh`

`validate-factory-path-root.sh` catches worktree-relative `.factory/` paths (i.e.,
`.worktrees/STORY-NNN/.factory/...`). It does NOT validate that the canonical path
within `.factory/` matches any registry pattern. The new `validate-artifact-path` WASM
hook is complementary, not duplicative: it fires after the root guard and checks
whether the path inside `.factory/` matches a registered artifact pattern.

---

## 3. WASM Hook Architecture Notes

Both new hooks follow the established pattern from
`crates/hook-plugins/handoff-validator/` and `crates/hook-plugins/regression-gate/`:

### Pattern: injectable-callback core logic

All host I/O (read_file for registry, emit_event, log) is injected as closures
into a pure `fn hook_logic(..., callbacks)` function. Unit tests exercise every
branch without a WASM runtime. The WASM `main.rs` entry wires real host functions.

### ABI conformance

- `HOST_ABI_VERSION = 1` (additive — no new host functions required)
- Both hooks use `HookPayload` fields already present in the SDK
- `validate-artifact-path` fires on `PreToolUse` for `Write|Edit` tool events —
  it reads `tool_input.file_path` from the payload
- `validate-per-story-adversary-convergence` fires on `SubagentStop` events —
  it reads `agent_type` / `subagent_name` and `last_assistant_message` / `result`
  via the canonical BC-2.02.012 two-stage fallback chains (same as handoff-validator)

### Block-message convention

Both hooks use `HookResult::block_with_fix(hook, reason, recommendation, code)` per
the canonical Why/Fix/Code pattern established in HOST_ABI.md. No bare
`HookResult::block()` calls.

### Registry consultation pattern (`validate-artifact-path`)

The hook reads `plugins/vsdd-factory/config/artifact-path-registry.yaml` at runtime
via `host::read_file`. This is feasible because WASI preopened directory access covers
the project root (which includes `plugins/`). The registry is parsed with `serde_yaml`
(or manually, given WASM binary size constraints — to be decided in F2). Path matching
uses the `canonical_path_pattern` field with `{placeholder}` expansion against the
actual write target.

### `validate-artifact-path` enforcement_level rollout

The path registry schema includes `enforcement_level: block | warn | advisory`.
The hook MUST read this field per entry and respect it:
- `block` → emit `HookResult::block_with_fix(...)` to hard-reject the write
- `warn` → emit `hook.warn` event + write to stderr + return `HookResult::Continue`
- `advisory` → log only, no stderr, return `HookResult::Continue`

**Rollout strategy:** all entries in the initial registry ship with
`enforcement_level: warn`. After the `relocate-artifact` skill has cleaned existing
misplaced artifacts (verified via a one-cycle audit), entries promote to `block`. This
prevents false-positive blocks on artifacts that predate the registry and were
legitimately placed.

---

## 4. Path Registry Design (`artifact-path-registry.yaml`)

### Schema (minimum required fields)

```yaml
artifacts:
  - artifact_type: behavioral-contract
    canonical_path_pattern: ".factory/specs/behavioral-contracts/ss-{ss-id}/BC-{bc-id}.md"
    description: "Behavioral contract files — sharded by subsystem"
    enforcement_level: warn    # promoted to block after relocation sweep

  - artifact_type: adr
    canonical_path_pattern: ".factory/specs/architecture/decisions/ADR-{adr-id}-{slug}.md"
    description: "Architectural decision records"
    enforcement_level: warn

  - artifact_type: verification-property
    canonical_path_pattern: ".factory/specs/verification-properties/VP-{vp-id}.md"
    description: "Verification property files"
    enforcement_level: warn

  - artifact_type: cycle-document
    canonical_path_pattern: ".factory/cycles/{cycle-id}/{document-name}.md"
    description: "Feature cycle documents (F1–F7, adversary passes, gate reports)"
    enforcement_level: advisory

  - artifact_type: story-spec
    canonical_path_pattern: ".factory/specs/stories/{story-id}.md"
    description: "Story specification files"
    enforcement_level: warn

  - artifact_type: phase-delta-analysis
    canonical_path_pattern: ".factory/cycles/{cycle-id}/F{phase}-delta-analysis.md"
    description: "Phase-level delta analysis documents"
    enforcement_level: advisory
```

### Single-source-of-truth invariant

`artifact-path-registry.yaml` is the ONE canonical source. Neither the WASM hook
(item 7), the skill updates (item 9), nor the relocation skill (item 11) may embed a
duplicate list. All three must read the registry at runtime.

---

## 5. Skill Update Requirements (Item 9)

Each of the seven creation skills plus `register-artifact` requires:

1. **Path resolution step:** Before any Write, resolve the target path against
   `artifact-path-registry.yaml`. If no canonical pattern matches the artifact type,
   refuse to write.
2. **Error message:** On mismatch, emit: "Artifact type `<type>` has no canonical path
   in `plugins/vsdd-factory/config/artifact-path-registry.yaml`. Consult the registry
   or use `/vsdd-factory:relocate-artifact` to determine the correct location. Do not
   invent directory names."
3. **Pattern:** Skills read the YAML file (via `Read` tool) at the start of their
   procedure, not embedded as literals. This ensures they pick up registry updates
   without skill modification.

**Additional survey needed in F4:** `ls plugins/vsdd-factory/skills/` reveals 120+
skills. Any skill whose name begins with `create-` or whose description contains
"write" or "scaffold" should be surveyed for artifact-creation paths. The seven named
skills are confirmed; others require a targeted survey during F4 implementation.

---

## 6. Relocation Skill (`relocate-artifact`) Design (Item 11)

### Procedure summary

1. **Detect:** Walk `.factory/` (and governed roots defined in registry). For each
   `.md` file, attempt to classify it by frontmatter `document_type`. Check its
   current path against the registry pattern for that type.
2. **Diagnose:** For each misplaced artifact, propose canonical location using
   frontmatter fields (e.g., `bc_id`, `subsystem` → `ss-{ss-id}/BC-{bc-id}.md`).
3. **Dry-run output** (default): emit a table of proposed moves. No filesystem changes.
4. **Apply** (`--apply` flag): execute `git mv`, update cross-references (story IDs,
   BC IDs, ADR IDs, anchor links) in other files, append move summary to active
   cycle's `decision-log.md`.
5. **Safety:** refuse to relocate if cross-references cannot be resolved, unless
   `--force-references-broken` is passed.

### Registry dependency

The relocation skill reads `artifact-path-registry.yaml` for canonical patterns. It
does not embed its own path list. This is the third consumer of the registry (after the
WASM hook and the creation skills), preserving the single-source-of-truth invariant.

---

## 7. Writing-Agent Prompt Preamble (Item 10)

The following standard paragraph is added to every writing-agent system prompt
(architect, product-owner, business-analyst, story-writer, technical-writer, and any
other agent that produces `.factory/` artifacts):

> Before any Write under `.factory/`, verify the target path matches a pattern in
> `plugins/vsdd-factory/config/artifact-path-registry.yaml`. If unsure, use the
> `register-artifact` skill or list existing structure first. Do not invent directory
> names.

This is the "cultural reinforcement" layer in the defense-in-depth stack:
1. WASM hook — enforcement floor (hard block or warn at write time)
2. Skill layer — structured path resolution before writing
3. Prompt language — cognitive priming before the agent even constructs a path

---

## 8. Proposed Story Decomposition

### Recommendation: 3 stories

**Reasoning:** The user explicitly invited a split. The two scope clusters have
distinct ownership, different reviewers (hook authors vs. skill authors vs. agent
authors), and independent rollout concerns (registry must ship before hook or skills
can reference it). Shippability is improved: Story A can ship before Story B's WASM
hook is production-ready, and Story C can ship independently of both.

#### Story A — Per-Story Adversary Workflow + AGENT.md Reconciliation

**Scope:** Items 1–5 (original scope)
- Formal adversary review gate in per-story-delivery Lobster workflow
- Adversary agent behavioral contract (what constitutes a clean pass)
- AGENT.md reconciliation: adversary agent, story-writer, convergence-checker
- Convergence criterion: minimum 3 clean passes without new findings
- No new WASM hook in Story A (gate enforced by workflow precondition)

**Rationale for deferring WASM hook to Story B:** The workflow and agent contract can
be specified and tested independently. The WASM hook is implementation detail that
adds enforcement without changing the behavioral contract.

#### Story B — WASM Hook: `validate-per-story-adversary-convergence`

**Scope:** Item 6 (original scope, now confirmed WASM)
- Rust crate at `crates/hook-plugins/validate-per-story-adversary-convergence/`
- Fires on `SubagentStop` for convergence-checker agent type
- Reads convergence state file from `.factory/`
- Blocks if story is marked convergence-required but adversary gate has not been cleared
- Full unit test suite (injectable callbacks, no WASM runtime required)
- bats integration tests

**Depends on:** Story A (defines the convergence state file schema and behavioral contract)

#### Story C — Path Governance Bundle

**Scope:** Items 7–11
- `artifact-path-registry.yaml` (initial entries, all `enforcement_level: warn`)
- WASM hook `validate-artifact-path` (Rust crate)
- Skill updates for 7+ creation skills + `register-artifact`
- Writing-agent prompt preambles
- `relocate-artifact` skill (dry-run + --apply)
- bats tests for relocation skill
- Unit tests for `validate-artifact-path` WASM hook

**Independence:** Story C has no dependency on Stories A or B. It could ship first if
the path governance work is prioritized.

**Suggested delivery order:** C → A → B, because the registry and prompt preambles
reduce the risk of misplaced artifacts during Stories A and B's own implementation.

---

## 9. Proposed BC IDs for F2

The next available subsystem for new engine-discipline BCs is SS-10 (CLI Tools and
Bin, last BC: BC-10.12.004). However, these items are best placed in:

- **SS-06 (Skill Catalog)** for the relocation skill and skill-update contracts
- **SS-04 (Plugin Ecosystem)** for the two WASM hooks and the path registry
- **SS-07 (Hook Bash Layer)** is NOT appropriate — these are WASM hooks, not bash
- **SS-05 (Pipeline Orchestration)** for the per-story adversary workflow gate

Proposed BC numbering (to be confirmed in F2 by reading BC-INDEX for exact next ID
within each subsystem):

| BC ID (provisional) | Scope | Subsystem |
|---|---|---|
| BC-4.NN.001 | validate-artifact-path WASM hook behavioral contract | SS-04 |
| BC-4.NN.002 | artifact-path-registry.yaml schema + enforcement_level semantics | SS-04 |
| BC-5.NN.001 | per-story adversary workflow gate (Lobster precondition) | SS-05 |
| BC-5.NN.002 | validate-per-story-adversary-convergence WASM hook | SS-05 |
| BC-6.NN.001 | relocate-artifact skill behavioral contract | SS-06 |
| BC-6.NN.002 | creation skill path-resolution requirement | SS-06 |

Exact NN values require reading current BC-INDEX counts per subsystem in F2. Using
provisional placeholders.

---

## 10. Proposed ADR IDs for F2

Next sequential ADR is ADR-016 (current max: ADR-015).

| ADR ID | Title | Subsystems |
|---|---|---|
| ADR-016 | Artifact Path Registry as Single Source of Truth for `.factory/` Canonical Paths | SS-04, SS-06 |
| ADR-017 | Per-Story Adversary Convergence Gate: Workflow Precondition vs. WASM Hook | SS-04, SS-05 |

ADR-016 governs the design decision that the YAML registry (not embedded literals in
skills or hooks) is the one source of truth. This is the key architectural invariant
that prevents future divergence.

ADR-017 records the split between the workflow gate (Story A) and the WASM enforcement
hook (Story B), including the rationale for phased delivery.

---

## 11. Proposed VP IDs for F2

Next available VP is VP-069 (current max: VP-068).

| VP ID | Description | Module | Tool | Phase |
|---|---|---|---|---|
| VP-069 | `validate-artifact-path`: registry load never panics on malformed YAML | validate-artifact-path | proptest | P1 |
| VP-070 | `validate-artifact-path`: path matching is pure (deterministic given same registry + path) | validate-artifact-path | kani | P1 |
| VP-071 | `validate-per-story-adversary-convergence`: Block Invariant — kani harness verifies `HookResult::Block` on non-converged input (canonical `block_with_fix` form; OQ-9 resolved D-349, VP-071 v1.1) | validate-per-story-adversary-convergence | kani | P1 |
| VP-072 | `artifact-path-registry.yaml` as single source: no skill or hook embeds a duplicate path list | cross-cutting | integration | P1 |

VP-072 is notable as a cross-cutting integration-test VP — it verifies the
single-source-of-truth invariant programmatically (grep for hardcoded path patterns
in skills and hook source, assert zero hits).

---

## 12. Proposed Crate Paths (Full Absolute Paths)

**Hook 1 — Per-story adversary convergence:**
`/Users/jmagady/Dev/vsdd-factory/crates/hook-plugins/validate-per-story-adversary-convergence/`

Structure follows `handoff-validator/` template:
- `src/lib.rs` — pure logic with injectable callbacks; unit tests inline
- `src/main.rs` — WASM entry point wiring real host fns
- `Cargo.toml` — `[lib] crate-type = ["cdylib"]`; deps: `vsdd-hook-sdk`, `serde_json`
- `tests/` — integration tests (bats, if needed; or Rust integration tests)

Build output: `plugins/vsdd-factory/hook-plugins/validate-per-story-adversary-convergence.wasm`

**Hook 2 — Artifact path validation:**
`/Users/jmagady/Dev/vsdd-factory/crates/hook-plugins/validate-artifact-path/`

Structure follows `regression-gate/` template (needs file reads + state):
- `src/lib.rs` — pure logic with injectable callbacks; reads registry via callback;
  unit tests inline (fixtures supply parsed registry, no file I/O in tests)
- `src/main.rs` — WASM entry point wiring `host::read_file`, `host::emit_event`,
  `host::log_*`
- `Cargo.toml` — deps: `vsdd-hook-sdk`, `serde_json`, and a YAML parser
  (`serde_yaml` or `basic-toml`-pattern manual parse — binary size TBD)
- `tests/` — unit tests; bats integration tests

Build output: `plugins/vsdd-factory/hook-plugins/validate-artifact-path.wasm`

**YAML dependency note:** `serde_yaml` adds non-trivial WASM binary size. F2 should
evaluate whether to parse the registry with `serde_yaml` or with a minimal custom
parser. If binary size exceeds the project's informal 500 KB target per plugin, switch
to a hand-written YAML-subset parser (the registry schema is simple enough).

---

## 13. Affected Tests

### Rust unit tests (per crate, inline in lib.rs)

`validate-per-story-adversary-convergence`:
- Clear gate path → HookResult::Continue, no stdout block signal
- Uncleaned gate path → advisory block stdout + hook.block event + stderr
- Malformed state file → graceful Continue (EC-001)
- Missing state file (first run) → Continue, no block
- Agent identity fallback chain (BC-2.02.012 postcondition 5) — mirrors handoff-validator tests

`validate-artifact-path`:
- Path matches registry entry → Continue
- Path matches entry but `enforcement_level: warn` → warn event + stderr + Continue
- Path matches entry but `enforcement_level: advisory` → log only + Continue
- Path matches entry with `enforcement_level: block` → block_with_fix
- Path does not match any entry → block_with_fix (treated as `block` by default)
- Registry YAML malformed → graceful Continue (non-fatal; log error)
- Registry file absent → graceful Continue (non-fatal; log error)
- Non-`.factory/` path → early-exit Continue (out of scope)

### bats integration tests

Both WASM hooks: real `.wasm` load via factory-dispatcher in test harness. Pattern
follows `tests/agent-guards-emission.bats` and `tests/hook-robustness.bats` (already
in the test surface).

`relocate-artifact` skill: bats tests for dry-run output format, `--apply` git mv
execution, cross-reference update correctness, and `--force-references-broken` safety
gate.

### Lobster workflow tests

`validate-workflow` skill and existing Lobster validation suite: add a test case for
the per-story adversary gate precondition. Verify that a story without adversary
clearance triggers the gate and that a story with clearance passes through.

---

## 14. Regression Risk Assessment

### `validate-artifact-path` — HIGH blast radius

This hook fires on every Write/Edit tool call to `.factory/`. If registry patterns are
too narrow, it will block legitimate writes from all agents. Mitigation:

1. **Registry ships with `enforcement_level: warn` only.** No blocking in the first
   cycle. Agents see stderr warnings but writes are not rejected.
2. **One-cycle audit:** After the registry ships, run `relocate-artifact --dry-run` to
   catalogue misplaced artifacts. Fix them. Verify false-positive rate is zero.
3. **Promote to `block`** in the subsequent pass (`v1.0-feature-engine-discipline-pass-2`
   or equivalent) after audit evidence confirms the registry patterns are correct.
4. **Pattern quality gate in F6:** Before shipping, run the hook against a fixture set
   of known-good write paths and assert zero false positives.

### `validate-per-story-adversary-convergence` — MEDIUM blast radius

Fires only on `SubagentStop` for convergence-checker agent type. Narrower surface than
the path hook. Primary regression risk: if the convergence state file format changes,
the hook reads a stale schema. Mitigation: define the state file schema formally in
BC-5.NN.002 and version it; hook validates schema version field.

### Skill updates — LOW blast radius

Creation skills adding a registry-read step at the start of their procedure. If the
registry file is absent (edge case: fresh install before registry ships), the skill
should default to permissive (warn, proceed) rather than blocking. This must be
explicit in the skill update contract produced in F4.

### Writing-agent prompt preambles — LOW blast radius

Pure text addition. No behavioral change; no hook interaction. Zero regression risk.

---

## 15. Epic Placement Recommendation

### Cluster A (Stories A and B) — Engine Governance epic

The per-story adversary workflow and its enforcement hook are squarely in the
"engine self-governance" domain. They belong in an existing or new epic titled
**Engine Governance** (or equivalent) alongside the existing adversarial-review
infrastructure. If the current epic structure has an `E-Governance` or `E-Engine`
epic, place them there. If not, create a new epic with ID to be allocated in F2.

### Cluster B (Story C) — Artifact Integrity epic

The path registry, WASM path hook, skill updates, and relocation skill form a cohesive
**Artifact Integrity** theme — preventing and repairing misplaced artifacts. This is
distinct from the adversary workflow concern. Recommend a separate epic:
**Artifact Integrity** (new, to be allocated in F2). Placing it in the same epic as
Cluster A would obscure the different review and rollout cadences.

### Two-epic split rationale

Different stakeholders review each cluster: hook authors and Rust engineers own
Cluster B's WASM crate; pipeline/orchestration owners review Cluster A's Lobster
changes; skill authors own the skill updates in Story C. Keeping them in separate
epics preserves reviewer clarity and allows independent scheduling.

---

## 16. Compressed F2–F7 Plan

### F2 — Spec Evolution

Produce:
- 2 new epics (Engine Governance ID, Artifact Integrity ID)
- 6 BCs across SS-04, SS-05, SS-06 (see Section 9)
- 2 ADRs: ADR-016 (path registry single-source), ADR-017 (adversary gate phasing)
- 4 VPs: VP-069–VP-072 (see Section 11)
- PRD delta: add per-story adversary gate and path governance to feature requirements
- Initial `artifact-path-registry.yaml` schema spec (not the file itself — that's F4)

### F3 — Incremental Stories

3 story specs (Stories A, B, C per Section 8). Story C written first to minimize
misplaced-artifact risk during subsequent story delivery.

### F4 — Delta Implementation

**Delivery order:**
1. `artifact-path-registry.yaml` (foundation — Stories B and C depend on it)
2. Writing-agent prompt preambles (zero-risk; deploy immediately)
3. Skill updates for creation skills + `register-artifact` (registry-read step)
4. `validate-artifact-path` WASM crate (in `warn` mode via registry)
5. `relocate-artifact` skill (dry-run first; `--apply` after audit)
6. Per-story adversary workflow + AGENT.md reconciliation (Story A)
7. `validate-per-story-adversary-convergence` WASM crate (Story B; depends on Story A state schema)

### F5 — Scoped Adversarial

Adversary passes focused on:
- Path registry completeness (are all artifact types covered?)
- False-positive scenarios for `validate-artifact-path` in `warn` mode
- Convergence gate edge cases (first story in new cycle, missing state file)
- Relocation skill cross-reference update correctness

### F6 — Targeted Hardening

- `cargo test` + `cargo clippy` on both WASM crates
- Mutation testing (`cargo-mutants`) on pure logic functions in both crates
- bats tests on `relocate-artifact` skill
- `shellcheck` on any modified bash files (hooks/lib/block.sh if touched)
- Full Lobster workflow regression on adversary gate
- False-positive gate: run `validate-artifact-path` against fixture of known-good
  write paths; assert 0 blocks
- Confirm `enforce_level: warn` produces no CI failures

### F7 — Delta Convergence

- BC sign-off on all 6 BCs
- VP harness skeletons instantiated for VP-069–VP-072
- Registry audit: run `relocate-artifact --dry-run`; target 0 misplaced artifacts
- Prepare `v1.0-feature-engine-discipline-pass-2` scope notes if new gaps identified
- Gate: all new tests green; no regressions in existing bats suite

---

## 17. Open Questions

### OQ-1: YAML parser for `validate-artifact-path`

`serde_yaml` is the natural choice but adds to WASM binary size. Alternative: inline
a minimal YAML-subset parser sufficient for the registry format (flat list, 5 fields
per entry). Decision should be made in F2 when the registry schema is finalized.
**Recommendation:** spike `serde_yaml` in F4; if binary > 500 KB, switch to manual.

### OQ-2: Registry path for `artifact-path-registry.yaml`

The registry must be readable by the WASM hook via `host::read_file`. The path
`plugins/vsdd-factory/config/artifact-path-registry.yaml` is within the WASI preopened
project root. Confirm in F2 whether the `config/` subdirectory exists under
`plugins/vsdd-factory/` — the `ls` of that path returned empty/error in the
reference reads. If absent, the directory must be created as part of Story C delivery.

### OQ-3: `validate-per-story-adversary-convergence` state file schema

What is the convergence state file path and schema? Two options:
(a) Reuse `.factory/STATE.md` with a new convergence status section
(b) New file `.factory/adversary-convergence-state.json` per story

Option (b) is cleaner for the hook (no prose parsing required) and mirrors
`regression-state.json`. **Recommendation:** option (b). Confirm in F2 when writing
BC-5.NN.002.

### OQ-4: Additional creation skills to survey

The 7 named creation skills are confirmed. `ls plugins/vsdd-factory/skills/` shows
120+ skills. The F4 implementer must survey for any `create-*`, `scaffold-*`, or
`register-*` skills that write to `.factory/` and are not yet covered. A mechanical
grep for `Write` tool usage in SKILL.md files provides a starting list.

### OQ-5: `enforcement_level` promotion timeline

The rollout plan says "promote to `block` in a subsequent pass." Should this be a
hard gate in `v1.0-feature-engine-discipline-pass-2`, or is it deferred to the user's
discretion? **Recommendation:** add a tracked TODO in the cycle's `decision-log.md`
with a criterion: "promote when `relocate-artifact --dry-run` reports 0 misplaced
artifacts AND one full delivery cycle completes without false-positive blocks."

### OQ-6: Adversary agent `AGENT.md` locations

Which AGENT.md files require modification for the per-story adversary workflow? The
F1 analysis assumes adversary, story-writer, and convergence-checker roles. The exact
file paths under `plugins/vsdd-factory/` need to be confirmed in F2 by reading the
agent file directory structure.

---

## 18. Cycle Metadata

| Field | Value |
|---|---|
| Cycle name | `v1.0-feature-engine-discipline-pass-1` |
| Cycle directory | `.factory/cycles/v1.0-feature-engine-discipline-pass-1/` |
| Phase | F1 (delta analysis) |
| Clusters | A (per-story adversary), B (artifact path governance) |
| Stories proposed | 3 (A, B, C — see Section 8) |
| New Rust crates | 2 (validate-per-story-adversary-convergence, validate-artifact-path) |
| New config files | 1 (artifact-path-registry.yaml) |
| New skills | 1 (relocate-artifact) |
| Modified skills | 8+ (7 creation skills + register-artifact) |
| Modified agent prompts | 5+ (architect, product-owner, business-analyst, story-writer, technical-writer) |
| BCs proposed for F2 | 6 (provisional IDs, 2×SS-04, 2×SS-05, 2×SS-06) |
| ADRs proposed for F2 | 2 (ADR-016, ADR-017) |
| VPs proposed for F2 | 4 (VP-069 through VP-072) |
| Epics proposed | 2 (Engine Governance, Artifact Integrity) |
| WASM hook ABI | HOST_ABI_VERSION = 1 (unchanged) |
| Hook enforcement mode | warn (initial); block (post-audit) |
