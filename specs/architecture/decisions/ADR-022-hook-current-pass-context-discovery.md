---
document_type: adr
adr_id: ADR-022
status: accepted
accepted_date: 2026-05-15
date: 2026-05-15
version: "1.0"
cycle: v1.0-feature-engine-discipline-pass-1
subsystems_affected: [SS-04, SS-05]
supersedes: null
superseded_by: null
related_adrs: [ADR-017, ADR-018]
related_decisions: [D-337]
inputs:
  - .factory/specs/architecture/ARCH-INDEX.md
  - .factory/specs/architecture/decisions/ADR-017-per-story-adversary-phasing.md
  - .factory/specs/architecture/decisions/ADR-018-wasm-plugin-context-resolvers.md
input-hash: "8ca8e03"
---

# ADR-022: Hook Plugin Access to Current Adversary Pass Context

## Context

### The operational requirement

`validate-closes-completeness` (S-15.13 Phase 2) is a WASM hook that enforces that
every adversarial finding cited in a closes-block has a verifiable resolution artifact.
Phase 1 of this hook validates static closure syntax. Phase 2 must also verify that a
finding cited as "closed in pass-N" actually appears in the pass-N adversary review
file — i.e., the hook must know the current pass number to locate the right review
artifact.

The current adversary pass number is not part of the standard dispatcher payload
(`HookPayload`). It lives in `STATE.md` (as the highest pass-N adversary review file
present in `.factory/cycles/<cycle>/`) and is authoritative only when read from the
file system. The hook cannot derive it from the dispatch envelope alone.

### Three options for supplying current-pass context

**Option (a) — Hook reads STATE.md directly via `host::read_file`.**
The WASM hook uses its existing `path_allow` capability to read `STATE.md` and
parses the Phase Progress section or the adversary review file listing to extract
the highest-N pass number.

Risk: `STATE.md` is a large prose document (~500+ lines, growing monotonically).
Its Phase Progress section has no stable machine-readable schema — it is a markdown
table whose row format is maintained by convention, not by a schema validator. Any
structural change to the Phase Progress table (new column, changed heading, reflow
of narrative) silently breaks the hook's regex or line-based parser. The hook
becomes tightly coupled to the prose structure of a living document that is
edited by multiple agents under high-frequency conditions (every fix burst).

**Option (b) — New CycleContextResolver crate extending ADR-018 platform.**
A new resolver (or extension to the existing `crates/vsdd-context-resolvers/`)
exposes "current adversary pass N" as a resolver-injected context value under the
`cycle_context` key. The WASM hook receives this value via `plugin_config` after
the dispatcher invokes the resolver. Follows the ADR-018 separation-of-concerns
model precisely.

Risk: requires authoring a new resolver WASM artifact (or extending the existing
`vsdd-context-resolvers` crate), updating `resolvers-registry.toml`, and adding
the `needs_context = ["cycle_context"]` field to the hook's `hooks-registry.toml`
entry. Meaningful scope overhead for what amounts to reading one integer from the
file system.

**Option (c) — Pointer file written by state-manager at every Commit A.**
State-manager writes `.factory/current-adversary-pass.txt` (content: a single
integer, the current pass number) at every Commit A of the fix-burst sequence.
The WASM hook reads this file via `host::read_file` under a narrowly-scoped
`path_allow`. The pointer file is a stable, machine-readable, single-value
artifact that does not require parsing a prose document.

Risk: introduces a new state-manager obligation (write the pointer file at every
Commit A). If the pointer file is missing (e.g., pre-existing repos without it,
or a Commit A that was produced before this ADR was adopted), the hook must degrade
gracefully. The pointer file is a new artifact that can drift from STATE.md's
narrative if the state-manager fails to write it.

### Why Option (c) is the correct production-grade choice

Option (a)'s coupling to STATE.md's prose structure is a maintainability trap.
STATE.md is the document most frequently edited in the entire factory pipeline —
every fix burst modifies it in multiple commits. A WASM hook that regex-parses
STATE.md is a fragile contract: it will break silently when STATE.md's structure
evolves, and that breakage will manifest as false negatives (the hook passes when
it should block) rather than as loud failures. False negatives in a security or
governance gate are the worst outcome.

Option (b) is architecturally ideal but disproportionately expensive for a
single-integer data dependency. The ADR-018 resolver platform was designed for
complex, frequently-needed context (like the active wave's story list). A resolver
that returns one integer from one file is engineering overhead with marginal benefit
over a simpler mechanism.

Option (c) is the production-grade choice for this specific need. The pointer file
is:
- A single-value artifact with no parsing ambiguity (line 1 = integer pass N).
- Narrowly scoped (one file, one value, one obligation).
- Stable in structure — it will not change format as prose documents evolve.
- Already consistent with the per-story state-file pattern from ADR-017
  (`adversary-convergence-state.json`) — that ADR established that machine-readable
  sidecar files are the correct pattern for WASM-hook-readable state, not prose docs.

The state-manager obligation (write at Commit A) is small, well-defined, and
auditable. Missing-file graceful degradation is a bounded implementation concern.

---

## Decision

**Option (c): pointer file at `.factory/current-adversary-pass.txt`.**

State-manager writes this file at every Commit A of every fix burst. The file
contains a single line: the integer pass number (e.g., `15`). The WASM hook
`validate-closes-completeness` (S-15.13 Phase 2) reads this file via `host::read_file`
to determine which adversary review file to inspect for finding verification.

---

## Rationale

### Stability beats elegance for a governance gate

A hook that enforces closes-completeness is only useful if it reliably reads the
correct pass number under all operational conditions. Option (a) reads a prose document
whose structure can change without warning. Option (c) reads a file whose structure
is a single integer — the simplest possible format, with no parsing surface area to
break.

### Proportionality: resolver platform overhead vs pointer-file simplicity

The ADR-018 resolver platform requires: a new resolver WASM binary, a
`resolvers-registry.toml` entry, capability declarations, resolver ABI conformance,
resolver error handling in the dispatcher, and a `needs_context` field in the hook
entry. This is correct overhead when the context value is complex (a structured
object like `wave_context`) or is needed by many hooks. For a single integer read
by a single hook, the resolver overhead exceeds the benefit. Option (c) achieves
the same result with a bash-level pointer file and one `host::read_file` call.

### Precedent from ADR-017

ADR-017 established that machine-readable sidecar files at predictable per-story
paths are the correct pattern for WASM-hook-readable state. The pointer file
follows this precedent at the cycle level (one file per cycle, not per story).

---

## Consequences

### Positive

- The WASM hook has a stable, unambiguous, always-parseable source of truth for the
  current pass number.
- The hook's `path_allow` grant is maximally narrow: one file, read-only.
- No new resolver crate or resolver infrastructure is required.
- The pointer file is a human-readable artifact (a single integer) that can be
  inspected directly for debugging.

### Negative

- State-manager gains a new Commit A obligation: write
  `.factory/current-adversary-pass.txt` before writing any other Commit A artifacts.
  This obligation must be documented in the fix-burst sequence specification.
- The pointer file can drift from STATE.md's Phase Progress narrative if a Commit A
  is produced without updating it. The hook's missing-file graceful degradation
  mitigates this but does not eliminate the possibility of a stale value.
- Pre-existing cycles (before this ADR) have no pointer file. The hook must handle
  the absent-file case for repos in-progress at the time this ADR is adopted.

### Mitigations

- **Graceful degradation on absent file:** If `.factory/current-adversary-pass.txt`
  does not exist, the hook emits a non-blocking advisory (not a hard block) and
  skips Phase 2 closes-verification. The advisory message directs the developer to
  run `state-manager update-pass-pointer`. This prevents the hook from bricking
  repos that predate the pointer file.
- **Parse error handling:** If the file exists but its content is not a valid
  positive integer, the hook emits a hard block with a clear error message. This
  catches pointer file corruption early.
- **State-manager obligation codified:** The Commit A sequence specification in
  `per-story-delivery.md` and the fix-burst sequence documentation in
  `plugins/vsdd-factory/workflows/phases/` must explicitly enumerate
  "write `.factory/current-adversary-pass.txt`" as Commit A step 0 (before all
  other artifact writes). This obligation is a story-level acceptance criterion
  for S-15.13.

---

## Pointer File Specification

**Path:** `.factory/current-adversary-pass.txt`

**Format:** Single line, no trailing newline, positive integer.

```
15
```

**Contents semantics:** The integer is the ordinal pass number of the most recently
dispatched adversary review in the current cycle. "Dispatched" means a Commit A has
been recorded for that pass — the pass review file exists at
`.factory/cycles/<cycle-id>/adv-cycle-pass-N.md` where N is the integer in the
pointer file.

**Write timing:** State-manager writes this file at Commit A, after the adversary
review file has been persisted but before the INDEX.md row is updated. This ordering
ensures the pointer is written only when the corresponding review file exists.

**Read path for WASM hook:** The hook reads the file via `host::read_file` with
`path_allow = [".factory/current-adversary-pass.txt"]`. It parses the content as
a `u32` (trimming whitespace). On parse failure: hard block. On absent file:
non-blocking advisory.

**Update frequency:** Once per fix burst (at Commit A). The pointer is stable for
the entire duration of a fix burst's Commits B through E.

**Multi-cycle note:** The pointer file at `.factory/current-adversary-pass.txt`
reflects the currently active cycle. If multiple cycles run concurrently (rare in
vsdd-factory's single-active-cycle model), the file reflects the most recently
updated cycle's pass. Concurrent-cycle cases are out of scope for S-15.13.

---

## Implementation Notes

### For S-15.13 story authorship

**Hook location:** `crates/hook-plugins/validate-closes-completeness/` (existing
crate from Phase 1). Phase 2 adds logic to the existing binary — no new crate.

**New capability grant required:** Add
`path_allow = [".factory/current-adversary-pass.txt"]` to the
`validate-closes-completeness` entry in `plugins/vsdd-factory/hooks-registry.toml`.
This is additive to the existing `path_allow` entries for that hook.

**State-manager obligation:** S-15.13 must include an acceptance criterion
requiring that the state-manager Commit A sequence writes
`.factory/current-adversary-pass.txt` before any other Commit A artifacts. The
bats test suite for S-15.13 must verify that the pointer file is present and
correct after a simulated Commit A.

**Hook Phase 2 logic (pseudo-Rust):**

```rust
let pass_file = host::read_file(".factory/current-adversary-pass.txt");
let pass_n: u32 = match pass_file {
    Err(_) => {
        emit_advisory("current-adversary-pass.txt not found; skipping Phase 2");
        return Continue;
    }
    Ok(content) => content.trim().parse().map_err(|_| {
        return BlockWithFix {
            message: "current-adversary-pass.txt contains invalid pass number",
            fix: "Run state-manager update-pass-pointer to regenerate",
            code: "invalid_pass_pointer",
        };
    })?,
};
let review_file = format!(".factory/cycles/{}/adv-cycle-pass-{}.md", cycle_id, pass_n);
// Verify that finding IDs cited in closes-blocks appear in review_file
```

**Bats test scenarios for Phase 2:**
1. Pointer file absent: hook exits 0 with advisory (non-blocking).
2. Pointer file present, invalid content (e.g., `"abc"`): hook exits 2 with block.
3. Pointer file present, finding cited in closes-block not in pass-N review: exits 2.
4. Pointer file present, all findings in closes-block verified in pass-N review: exits 0.

**D-337 note:** `validate-closes-completeness` is an existing WASM hook plugin.
Phase 2 extends it. No new bash hook plugins are introduced. D-337 is not implicated.

---

## Alternatives Considered

### Option (a) — Hook reads STATE.md directly

Rejected. STATE.md is a prose document under high-frequency edit by multiple agents.
Its Phase Progress table has no stable machine-readable schema. A WASM hook parsing
STATE.md to extract the current pass number will break silently when the table
structure evolves. False negatives in a closes-completeness gate are unacceptable —
the hook must be reliable under STATE.md's ongoing evolution.

The coupling cost is also ongoing: every time the STATE.md format is refined
(adding a column, changing a heading, splitting a section), the hook's parser must
be updated. This maintenance burden is not justified when a simpler, stable
alternative exists.

### Option (b) — New CycleContextResolver crate

Rejected as disproportionate for a single-integer dependency. The ADR-018 resolver
platform is the correct mechanism for complex, reusable context values (the wave
story list is consumed by multiple hooks; a resolver amortizes the parsing work
across all consumers). A resolver for one integer consumed by one hook is
overengineering. The resolver platform adds meaningful crate count, ABI complexity,
and operational surface without a proportionate benefit over a pointer file.

If future hooks need additional cycle-level context beyond the pass number, extending
to a resolver (or adding a full `CycleContextResolver` with multiple fields) is a
natural upgrade path. This ADR does not foreclose that option.

---

## Subsystem Assignments

**SS-04 (Plugin Ecosystem):** Referencing SS-04 because the Phase 2 extension to
`validate-closes-completeness` lives in `crates/hook-plugins/` and its
pointer-file read contract is an SS-04 behavioral contract surface.

**SS-05 (Pipeline Orchestration):** Referencing SS-05 because the state-manager
Commit A obligation to write `.factory/current-adversary-pass.txt` is a change to
the fix-burst sequence specification in `plugins/vsdd-factory/workflows/phases/`.
The pointer file write is an SS-05 pipeline obligation.

---

## Decision Log Reference

| Decision | ID | Rationale |
|----------|----|-----------|
| Per-story state file as WASM-readable sidecar (precedent) | D-341 (OQ-3) | ADR-017 established machine-readable JSON sidecars as the correct WASM-hook-readable pattern |
| WASM-only for new hook plugins in this cycle | D-337 | Phase 2 extends an existing WASM plugin; no new bash hook debt |
| WASM resolver platform design | D-361 | ADR-018; Option (c) consciously defers resolver use as disproportionate here |

## Source / Origin

Decision originated from the S-15.03 PRIORITY-A wave-planning document (2026-05-15
session). Open question OQ-3 was left unresolved at wave-plan authorship time with
three options enumerated. The wave plan recommended Option (c) with the note
"simplest and avoids adding a new resolver crate; requires state-manager to write
pointer at every Commit A." This ADR confirms that recommendation under the
production-grade lens and documents the pointer file specification in full.

- **Wave plan:** `.factory/cycles/v1.0-feature-engine-discipline-pass-1/` S-15.03
  planning document — OQ-3 section enumerates all three options and notes the
  Option (c) recommendation.
- **ADR-017 precedent:** `decisions/ADR-017-per-story-adversary-phasing.md` —
  per-story `adversary-convergence-state.json` established the machine-readable
  sidecar pattern that this ADR extends to the cycle level.
- **ADR-018 precedent:** `decisions/ADR-018-wasm-plugin-context-resolvers.md` —
  resolver platform design; considered and ruled disproportionate for this use case.
- **Architect dispatch:** 2026-05-15, S-15.03 PRIORITY-A wave Milestone 3 ADR
  authorship batch.
