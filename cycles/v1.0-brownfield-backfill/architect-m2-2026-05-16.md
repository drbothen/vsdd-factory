---
document_type: architect-decision
level: ops
status: final
producer: architect
timestamp: 2026-05-16T00:00:00Z
phase: section-12-step-3-m2-inter-story-adjudication
cycle: brownfield-backfill
inputs:
  - .factory/cycles/v1.0-brownfield-backfill/s-15.03-wave-m2-dispatch.md
  - .factory/cycles/v1.0-brownfield-backfill/s-15.03-wave-plan-2026-05-15.md
  - .factory/STATE.md
  - .factory/cycles/v1.0-brownfield-backfill/architect-2026-05-15-td-66-67-split.md
  - crates/ (workspace structure inspection)
input-hash: "5af355e"
traces_to: .factory/cycles/v1.0-brownfield-backfill/s-15.03-wave-plan-2026-05-15.md
related_stories: [S-15.07, S-15.09, S-15.11, S-15.14]
develop_head_at_authoring: "c62f952c7307febcc65b6ab722ff02688dfe8c90"
factory_artifacts_head_at_authoring: "f1e88045"
---

# Architect Decision — M2 Inter-Story Order Adjudication (2026-05-16)

## Context

Milestone 1 of the S-15.03 PRIORITY-A wave (S-15.06 + S-15.16 Part A + S-15.08) shipped to develop
on 2026-05-16 at `c62f952c` (PR #144 squash-merge). M2 contains four WASM Rust hook stories:
S-15.07, S-15.09, S-15.11, and S-15.14. The dispatch package (`s-15.03-wave-m2-dispatch.md`,
input-hash ad1c745) gates all M2 story-writer dispatches on this architect adjudication. The
orchestrator may dispatch story-writer for the wave-1 M2 story immediately after this document is
committed to factory-artifacts.

---

## Questions Adjudicated

### Question 1 — Shared schema crate for burst-log-consuming stories

**Question (terse):** Should S-15.09 (parse STATE.md), S-15.11 (parse burst-log.md), and S-15.14
(parse STATE.md frontmatter + INDEX.md) share a common schema crate (e.g.,
`crates/lint-hook-schema/`) providing shared markdown parsing utilities? If YES, propose a new
sub-story ID and estimate effort. If NO, state rationale explicitly.

**Decision:** NO shared schema crate. Each story implements its own parsing surface inline.

**Rationale:** Evidence from `crates/hook-plugins/` and `Cargo.toml` workspace inspection
confirms that all 20 existing hook-plugin crates are standalone — no shared parsing library exists
today and none was introduced for the 15 prior hook crates. The parsing surfaces for the three
stories are distinct in ways that make a shared crate a coupling liability, not an asset:

1. **Non-overlapping structure:** S-15.09 parses the STATE.md *body* (banner line-count, dual-margin
   block, trajectory-tail line within body markdown). S-15.14 parses STATE.md *frontmatter*
   (`current_step:` YAML field) and INDEX.md table rows. S-15.11 parses burst-log.md h2 headings
   and block-type markers — a completely different markdown schema. The regex anchors for each hook
   do not overlap; there is no code that would actually be shared at the struct or function level.

2. **WASM compilation boundary:** Hook plugins compile to `wasm32-wasip1`. A shared library crate
   would need to be a Rust `[lib]` dependency consumed by each hook's `Cargo.toml`. Because the
   total parsing code per story is in the range of 50-150 lines of regex-and-line-count logic, the
   setup overhead of a new workspace member (`Cargo.toml`, workspace entry, separate crate version
   lifecycle) exceeds the code reuse benefit. The `vsdd-hook-sdk` crate (already a workspace
   member) is the only warranted shared dependency.

3. **YAGNI applies:** The wave plan notes "S-15.14 shares STATE.md domain with S-15.09; independent
   enough to parallel-develop." This confirms the architect's own prior read that the parsing surfaces
   diverge enough to not warrant shared schema. No future story in M3 (S-15.10, S-15.12, S-15.13)
   would consume a shared schema crate either — S-15.10 extends S-15.09's *crate* (same plugin,
   phased), not a separate schema library; S-15.12/S-15.13 parse decision-log.md and adv-cycle files.

4. **S-15.07 isolation:** S-15.07 (validate-index-cite-refresh) parses ARCH-INDEX.md version cite
   strings and reads peer index file frontmatter. This is orthogonal to the STATE.md/burst-log.md
   parsing cluster. S-15.07 benefits from S-15.09's crate structural lessons (wiring pattern,
   `host::read_file` usage, `block_with_fix` emission) but shares no parseable schema with S-15.09,
   S-15.11, or S-15.14.

**Implications for dispatch:** No new sub-story required. No shared crate workspace member to create.
Story-writer dispatches for all four M2 stories proceed directly per Question 2 ordering. Each
story's implementer authors its parsing logic inline in `src/lib.rs`.

---

### Question 2 — M2 wave-1 story selection

**Question (terse):** Is S-15.07 wave-1 (or does a shared-schema sub-story ship first), and which
story is wave-2?

**Decision:** S-15.07 is wave-1 (no shared-schema sub-story; Question 1 resolves NO). Wave-2 is
S-15.11. Wave-3 is S-15.09. Wave-4 is S-15.14.

**Rationale:**

- **S-15.07 first (wave-1):** The wave plan explicitly states "S-15.07 (index-cite-refresh) is the
  original story motivation and the highest-visibility deliverable. Recommend shipping first within
  Milestone 2 to demonstrate wave progress." The hook logic is deterministic (version string
  comparison between two files), the TDD seam is clear (stale vs current version cite → block vs
  pass), and there are zero blocking dependencies. S-15.07 also establishes the structural template
  (Cargo.toml pattern, `host::read_file` dual-file read, `block_with_fix` emission) that S-15.09 and
  S-15.14 will reference for their STATE.md parsing. Shipping S-15.07 first is architecturally
  correct for knowledge propagation, not just for visibility.

- **S-15.11 second (wave-2):** S-15.11 (validate-burst-log) is the simplest new crate in M2 — the
  wave plan explicitly labels it "simplest new crate in the milestone." It has zero dependencies on
  S-15.09 or S-15.14 (burst-log.md is independent of STATE.md). Shipping S-15.11 second provides a
  second complete WASM crate delivery before the more complex STATE.md validators begin, and it does
  not add any hooks-registry.toml section that conflicts with S-15.09 or S-15.14's file-pattern
  registrations. It also gives the implementer a second warm-up opportunity on the WASM crate
  scaffolding before tackling the trickier frontmatter-vs-body parsing distinction in S-15.09/S-15.14.

- **S-15.09 third (wave-3):** S-15.09 (validate-state-structure Phase 1) is the highest-impact STATE.md
  validator. Its crate scaffolding is referenced by S-15.14 ("S-15.09 crate patterns recommended as
  reference" per wave plan S-15.14 Dependencies). It also gates M3's S-15.10 ("same crate, phased
  extension"). S-15.09 must ship before S-15.14 so that S-15.14's story-writer and implementer have
  the concrete crate structure to reference rather than a hypothetical. It must also ship before
  S-15.10 is dispatched. Placing S-15.09 at wave-3 (after S-15.07 and S-15.11 have established the
  crate template and bats fixture patterns) reduces S-15.09's implementation risk.

- **S-15.14 fourth (wave-4):** S-15.14 (validate-dispatch-advance) shares the STATE.md trigger
  (PostToolUse Edit/Write on STATE.md) with S-15.09. Per OQ-5, no simultaneous in-flight PRs
  touching the same hooks-registry.toml section. Placing S-15.14 last within M2 ensures S-15.09 is
  fully merged before S-15.14's PR opens, eliminating hooks-registry.toml merge conflicts on the
  STATE.md PostToolUse section and allowing S-15.14's implementer to reference the actual (not
  hypothetical) S-15.09 crate structure.

**Implications for dispatch:** Orchestrator dispatches story-writer for S-15.07 immediately. After
S-15.07 is merged + CI green, dispatch S-15.11. After S-15.11 is merged + CI green, dispatch S-15.09.
After S-15.09 is merged + CI green, dispatch S-15.14. Full serial delivery per Question 3 below.

---

### Question 3 — Parallel vs serial sequencing within M2

**Question (terse):** Does OQ-5 require full serial delivery (one story at a time, no parallel
in-flight PRs), or are parallel feature branches safe as long as merge ordering is managed?

**Decision:** Full serial delivery — one story in-flight at a time. No parallel feature branches
during M2.

**Rationale:**

The OQ-5 resolution in the wave plan states: "orchestrator coordinates timing so their hooks-registry
registrations land in separate PRs without overlapping development windows. No simultaneous in-flight
PRs touching the same hooks-registry.toml section."

"Overlapping development windows" + "no simultaneous in-flight PRs" translates operationally to fully
serial. Three specific constraints enforce this:

1. **hooks-registry.toml conflict risk:** S-15.09 and S-15.14 both register PostToolUse handlers
   for `STATE.md`. If two feature branches are in-flight simultaneously, each will add a
   `[[hooks]]` entry to `plugins/vsdd-factory/hooks-registry.toml`. When both branches target
   develop and one merges first, the second branch's hooks-registry.toml diff will conflict at the
   insertion point. Squash-merge makes this conflict resolution non-trivial. Fully serial eliminates
   this class of conflict entirely.

2. **S-15.14 reference dependency on S-15.09:** The wave plan explicitly states S-15.14's
   "S-15.09 crate patterns recommended as reference (same validate-state-structure domain)." If S-15.09
   and S-15.14 are parallel-developed on separate branches, S-15.14's implementer works from a
   hypothetical S-15.09 crate that does not yet exist in develop. When S-15.09 merges and the actual
   crate lands, S-15.14 may require a rebase or significant rework. Full serial eliminates this.

3. **Adversary cascade blast radius:** The LOCAL adversary cascade per story (BC-5.39.001 3-CLEAN)
   may identify implementation defects that require changes to the dispatch template (hooks-registry.toml
   patterns, `host::read_file` call surface, WASM binary naming). If two stories are in adversary
   cascades simultaneously, a cascade-driven change to the hooks-registry.toml structure in one story
   requires manual propagation to the other story's in-flight branch. Full serial prevents this
   compounding.

**Operational strategy for orchestrator:**

```
dispatch: story-writer → (S-15.07 full pipeline completes + PR merges + CI green)
then:     story-writer → (S-15.11 full pipeline completes + PR merges + CI green)
then:     story-writer → (S-15.09 full pipeline completes + PR merges + CI green)
then:     story-writer → (S-15.14 full pipeline completes + PR merges + CI green)
```

"Full pipeline" means: story-writer authorship → test-writer → implementer → LOCAL adversary 3-CLEAN
→ pr-manager 9-step → squash-merge → state-manager post-merge burst. The next story's story-writer
dispatch does not begin until the current story's state-manager post-merge burst commits to
factory-artifacts.

**Implications for dispatch:** Orchestrator must enforce the gate strictly. The 4-8 day M2 calendar
estimate (wave plan Section 6) assumed serial delivery of "one story per orchestrator session" and
remains valid under this constraint.

---

### Question 4 — Crate naming conventions

**Question (terse):** What is the canonical Rust crate naming pattern for M2 hook crates, consistent
with existing workspace conventions?

**Decision:** `crates/hook-plugins/<kebab-name>/` with Cargo.toml `name = "<kebab-name>"` and
compiled WASM output at `plugins/vsdd-factory/hook-plugins/<kebab-name>.wasm`.

**Rationale:** Evidence from `crates/hook-plugins/` directory listing and three Cargo.toml files
inspected (`validate-artifact-path`, `validate-stable-anchors`, `lint-registry-async-invariant`):

- All 20 existing hook plugin crates live under `crates/hook-plugins/<name>/`.
- All use `name = "<name>"` in Cargo.toml where `<name>` is the kebab-case directory name verbatim.
- Compiled WASM outputs follow `<name>.wasm` (confirmed by `plugins/vsdd-factory/hook-plugins/`
  listing: `validate-artifact-path.wasm`, `validate-stable-anchors.wasm`,
  `lint-registry-async-invariant.wasm`, etc.).
- The existing naming taxonomy uses descriptive verb-noun patterns for validators
  (`validate-artifact-path`, `validate-stable-anchors`, `validate-per-story-adversary-convergence`)
  and `lint-` prefix for lint-style invariants (`lint-registry-async-invariant`). The M2 stories
  follow the validator taxonomy.

**Canonical pattern for M2:**

| Story | Crate path | Cargo.toml name | WASM output |
|-------|-----------|----------------|-------------|
| S-15.07 | `crates/hook-plugins/validate-index-cite-refresh/` | `validate-index-cite-refresh` | `hook-plugins/validate-index-cite-refresh.wasm` |
| S-15.11 | `crates/hook-plugins/validate-burst-log/` | `validate-burst-log` | `hook-plugins/validate-burst-log.wasm` |
| S-15.09 | `crates/hook-plugins/validate-state-structure/` | `validate-state-structure` | `hook-plugins/validate-state-structure.wasm` |
| S-15.14 | `crates/hook-plugins/validate-dispatch-advance/` | `validate-dispatch-advance` | `hook-plugins/validate-dispatch-advance.wasm` |

**Worked example (S-15.07):**

```toml
# crates/hook-plugins/validate-index-cite-refresh/Cargo.toml
[package]
name = "validate-index-cite-refresh"
version = "0.0.1"
edition.workspace = true
license.workspace = true
repository.workspace = true
authors.workspace = true
rust-version.workspace = true
publish = false

[lib]
path = "src/lib.rs"

[[bin]]
name = "validate-index-cite-refresh"
path = "src/main.rs"

[dependencies]
vsdd-hook-sdk = { path = "../../hook-sdk" }
serde = { workspace = true }
serde_json = { workspace = true }

[lints]
workspace = true
```

Workspace `Cargo.toml` gains the entry:
```
"crates/hook-plugins/validate-index-cite-refresh",
```

hooks-registry.toml gains:
```toml
[[hooks]]
name = "validate-index-cite-refresh"
plugin = "hook-plugins/validate-index-cite-refresh.wasm"
```

All four M2 crates follow this exact template; only `name`, `plugin`, `event`, `file_pattern`, and
`priority` differ per story.

**Implications for dispatch:** Story-writer must use these exact paths in each story's Tasks section.
Implementer must create the crate at the exact path. Workspace Cargo.toml must be updated to include
the new member before `cargo build --target wasm32-wasip1` succeeds.

---

## Final M2 Dispatch Order

| Wave | Story | Branch | Blocking gate | Effort class |
|------|-------|--------|---------------|--------------|
| M2-Wave-1 | S-15.07 | `feature/S-15.07-index-cite-refresh-hook` | Architect adjudication (this document) | Multi-day |
| M2-Wave-2 | S-15.11 | `feature/S-15.11-validate-burst-log` | S-15.07 merged + CI green + state-manager post-merge burst complete | Multi-day |
| M2-Wave-3 | S-15.09 | `feature/S-15.09-validate-state-structure-p1` | S-15.11 merged + CI green + state-manager post-merge burst complete | Multi-day |
| M2-Wave-4 | S-15.14 | `feature/S-15.14-validate-dispatch-advance` | S-15.09 merged + CI green + state-manager post-merge burst complete | Multi-day |

Branch names are as specified in the dispatch package scope table. No deviation.

---

## New Sub-Stories Proposed

None. Question 1 resolves NO to a shared schema crate. No new sub-story identifiers required. The
STORY-INDEX remains at v3.31 (102 total stories) until story-writer authors S-15.07.

---

## Crate Naming Convention (Decision)

**Pattern:** `crates/hook-plugins/<validate-NOUN-or-VERB-NOUN>/`

All M2 hook crates live under `crates/hook-plugins/`. Directory name is kebab-case, matches
Cargo.toml `name` field verbatim, and matches the compiled WASM binary name. The `validate-` prefix
applies to all four M2 stories because each plugin enforces a structural correctness invariant on a
specific file class.

This convention is not new — it is already established by `validate-artifact-path`,
`validate-stable-anchors`, and `validate-per-story-adversary-convergence`. M2 extends the same row.

---

## Open Questions / Deferrals

None. All four questions are fully answered in scope. No items deferred to tech-debt-register.

---

## Propagation Plan

The orchestrator and state-manager must perform these propagation actions after this document is
committed to factory-artifacts:

1. **STATE.md Session Resume Checkpoint:** Update Section 4 "Active Tier-A" to reflect M2 dispatch
   order: S-15.07 (wave-1, dispatch-ready) → S-15.11 (wave-2) → S-15.09 (wave-3) → S-15.14 (wave-4).
   Include note: "architect adjudication complete 2026-05-16 (architect-m2-2026-05-16.md)."

2. **STATE.md Section 12:** Update the M2 row to note architect adjudication complete and confirmed
   serial delivery order S-15.07 → S-15.11 → S-15.09 → S-15.14.

3. **STORY-INDEX:** No version bump required from this document alone. Story-writer will bump
   STORY-INDEX when authoring S-15.07 (adds new row).

4. **No ARCH-INDEX update required:** No new ADR is implied by this decision. All four M2 stories
   follow established WASM hook architecture patterns (ADR-017/ADR-018). The crate naming decision
   is a convention alignment, not a new ADR-class architectural decision.

5. **First story-writer dispatch:** Orchestrator dispatches story-writer for S-15.07 immediately
   using the per-story dispatch template in `s-15.03-wave-m2-dispatch.md` §Per-Story Dispatch
   Template. Story-writer must reference:
   - Crate path: `crates/hook-plugins/validate-index-cite-refresh/`
   - Branch: `feature/S-15.07-index-cite-refresh-hook`
   - D-NNN sub-clauses closed: D-405(c) + D-429(b)
   - Hook trigger: PostToolUse Edit/Write on `ARCH-INDEX.md`
   - New BC anchor: E-12
