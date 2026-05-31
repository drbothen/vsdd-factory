---
document_type: architecture-decision-record
level: L3
adr_id: ADR-023
version: "1.0"
status: proposed
producer: architect
timestamp: 2026-05-30T00:00:00Z
title: "ADR-023: validate-trajectory-tail-cell-completeness — cycle-conditional STATE.md Block-arm site model (PC3/PC4/PC5 gated on F5-style per-pass cycles)"
traces_to: .factory/specs/architecture/ARCH-INDEX.md
anchors:
  - BC-5.39.009
  - S-15.17
  - D-453
  - D-453(d)
  - D-454
  - D-454(a)
  - D-522
  - ADR-017
  - ADR-018
subsystem: "SS-05"
supersedes: null
superseded_by: null
decision_status: proposed-pending-human-authorization
human_gate_required: true
human_gate_reason: "Recommended option re-scopes a SEALED BC (BC-5.39.009 v1.8, sealed D-522). Un-SEALing requires explicit human authorization per VSDD spec-amendment rule."
---

# ADR-023: validate-trajectory-tail-cell-completeness — cycle-conditional STATE.md Block-arm site model

## Status

**PROPOSED — pending human authorization.** This ADR is an architect adjudication of a
CRITICAL spec-vs-reality conflict surfaced by the S-15.17 LOCAL adversary cascade (pass-5).
The recommended option requires un-SEALing BC-5.39.009 (sealed at D-522), which is a
human-authorization gate. No SEALED-BC edit, implementation edit, or STATE.md edit is
performed by this ADR. PO / implementer / state-manager execute downstream under human
authorization.

## Context

### The conflict

S-15.17 implements `validate-trajectory-tail-cell-completeness` — a PostToolUse WASM hook
(priority 158) that, per BC-5.39.009 v1.8 (SEALED) and D-453(d), **Blocks** any
`.factory/STATE.md` write where the canonical trajectory-tail marker
(`trajectory-tail →N→N→N→N`, inv-4 two-step marker-prefix check, LENGTH=4 strict) is missing
from **any** of 5 prescribed STATE.md sites:

| Site | Location | inv-4 check |
|------|----------|-------------|
| 1 | frontmatter `current_step:` | marker + count==4 |
| 2 | Last Updated cell | marker + count==4 |
| 3 | Phase Progress latest row | marker + count==4 |
| 4 | Concurrent Cycles latest active row | marker + count==4 |
| 5 | Session Resume §1 body | marker + count==4 |

The implementation is high quality and **faithful to the BC** (5 adversary passes; 52/52 unit
+ 58/58 bats; clippy/fmt/wasm clean; registry priority-158 parity verified). The defect is in
the **BC's site model**, not the code.

### Ground truth from the live artifact (verified)

A `grep -n "trajectory-tail" .factory/STATE.md` against the live
`/Users/jmagady/Dev/vsdd-factory/.factory/STATE.md` (mode `brownfield`,
`current_cycle: v1.0-brownfield-backfill`) returns markers at:

- **line 15** — frontmatter `current_step:` → **site 1 PRESENT** (`trajectory-tail →9→9→9→11`)
- **line 68** — Last Updated cell → **site 2 PRESENT** (`trajectory-tail →9→9→9→11`)
- lines 39 + 44-49 — banner-tracker HTML-comment block (NOT a prescribed site)
- line 240 — a Decisions Log D-513 row (NOT site 3/4/5)
- line 376 — a Session Resume bullet that *describes* the PC2 requirement ("Last Updated cell
  MUST include `trajectory-tail →N→N→N→N` marker") — this is documentation of the requirement,
  **not** a §1-body trajectory tail
- line 436 — a verify-step bullet (NOT site 5 body)

**Sites 3, 4, and 5 carry NO `trajectory-tail →N→N→N→N` marker** on the live artifact:

- **Site 3 (Phase Progress latest row):** the bottommost row is the D-524 milestone row
  ("PR #163 ... + S-15.17 both captured ...") — a STATUS/MILESTONE row with no per-row
  trajectory tail.
- **Site 4 (Concurrent Cycles latest active row):** the bottommost active row is the
  brownfield-backfill bolt-on / F5 PAUSED row — milestone-form, no per-row tail.
- **Site 5 (Session Resume §1 body):** the §1 body carries no `trajectory-tail →N→N→N→N`
  sequence.

Therefore, once S-15.17 merges + releases + activates, every state-manager STATE.md write in
the current cycle would emit a cascade **Block (exit 2)** → **pipeline-brick risk** under the
production-grade-default pipeline.

### Two intertwined root causes (separated)

**Root cause 1 — genuine degradation the hook is designed to catch.** F5 pass-75 finding
ADV-EDP1-P75-HIGH-002 (the anchor S-15.17 closes) was "only 2 of 9 D-453(d) sites carried the
tail after a 14-day pause." Under this reading the hook is correct and the cure is: bring the
live STATE.md into compliance (write the marker into sites 3/4/5) + add a real-STATE.md
regression fixture, then ship.

**Root cause 2 — cycle-convention mismatch.** D-453(d) + BC-5.39.009 model **PER-PASS rows**:
the F5 engine-discipline cycle appends, every adversary pass, a Phase Progress row and a
Concurrent Cycles row that carries that pass's trajectory tail (the `→N→N→N→N` axis-count
arrow sequence is a *per-pass convergence metric*). The CURRENT active cycle,
`v1.0-brownfield-backfill`, is a **story-delivery cycle** whose Phase Progress and Concurrent
Cycles rows are **MILESTONE/STATUS rows** ("S-15.13 SHIPPED", "rc.19 SHIPPED", "PR #163
captured"). A story-delivery cycle has **no per-pass trajectory** — there is no axis-count
metric to put in those rows. Under this reading, PC3/PC4/PC5 are mis-specified for non-F5
cycles, and forcing per-row tails onto milestone rows would write **meaningless data to
satisfy a check** — a paper-compliance smell (TD-VSDD-059 adjacent) and a SOUL.md
correctness violation.

## Decision drivers

1. **Production-grade default** (CLAUDE.md Canonical Principle): no paper-compliance; no
   meaningless data written to satisfy a gate. Fix the real defect, not the symptom.
2. **Code-vs-spec rule** (CLAUDE.md SoT #12): when code and spec conflict, the SPEC wins —
   code is brought into alignment. Here the code is faithful to the spec; the **spec is the
   defect**. The architect (ADR owner) adjudicates the spec direction; PO executes the BC edit.
3. **inv-4 LENGTH=4-strict** must be preserved (BC-5.39.006 inv-6(b) + EC-007 alignment;
   D-433(e)+D-439(c)). The trajectory-tail *semantics* are sound; only the *site applicability*
   is wrong.
4. **Pipeline-brick avoidance is a hard gate.** No option may ship a hook that Blocks every
   state-manager write in the active cycle.
5. **Self-referential constraint:** the hook ships in the engine's own dispatcher; it takes
   effect at the operator level only after release. The live STATE.md is both the test fixture
   and the production artifact. A regression fixture MUST be the real (or real-shaped)
   STATE.md, not a synthetic happy-path one (META-LEVEL-24 false-green class — the very class
   D-453(d)/inv-4 exist to prevent).

## Options considered

### Option (a) — Hook is correct → make STATE.md compliant

State-manager writes `trajectory-tail →N→N→N→N` into sites 3/4/5 (Phase Progress latest row,
Concurrent Cycles latest active row, Session Resume §1); add a real-STATE.md regression
fixture; commit the project to per-row tails on those sites going forward, in **every** cycle
including non-F5 story-delivery cycles.

- **Pro:** No BC reopen. Hook ships as-is. Directly satisfies root-cause-1 reading.
- **Con (decisive):** In a story-delivery cycle there is **no per-pass trajectory** — the
  axis-count `→N→N→N→N` is an F5 convergence metric. Putting it on a milestone row
  ("S-15.13 SHIPPED; trajectory-tail →9→9→9→11") writes a value that is **semantically false
  for that row** (it is a stale carry-across of the last F5 pass's tail, not a property of the
  milestone). This is exactly the meaningless-data-to-satisfy-a-check anti-pattern the
  production-grade default forbids. It also creates a permanent maintenance tax: every future
  milestone row in every future non-F5 cycle must carry a fabricated tail forever.
- **Verdict:** REJECTED. Trades a real correctness defect for paper-compliance.

### Option (b) — BC mis-specified → narrow the Block arm to cycle-invariant sites

Reopen SEALED BC-5.39.009; PO narrows the **Block** arm to sites 1+2 only (current_step +
Last Updated — the two sites that are cycle-invariant: both carry a single canonical
trajectory-tail by state-manager Commit-E discipline regardless of cycle type). Sites 3/4/5
become advisory (log_warn + Continue) or are dropped.

- **Pro:** Eliminates brick risk cleanly. Sites 1+2 are genuinely cycle-invariant and ARE
  present on the live artifact. Simple to implement and reason about.
- **Con:** Loses the per-row degradation detection in F5 cycles, which was a real part of the
  ADV-EDP1-P75-HIGH-002 finding (the F5 pause degraded per-pass rows). Demotes a genuine
  Block-worthy F5 invariant to advisory. Under-fits root-cause-1.
- **Verdict:** Viable fallback, but strictly weaker than (c) for F5 cycles.

### Option (c) — Cycle-conditional site model (RECOMMENDED)

Reopen SEALED BC-5.39.009; PO re-specs the STATE.md Block arm so that:

- **Sites 1 + 2 (current_step, Last Updated): ALWAYS Block** — cycle-invariant. These two
  cells carry the single canonical trajectory-tail in every cycle by state-manager Commit-E
  discipline, and are present on the live artifact today.
- **Sites 3 + 4 + 5 (Phase Progress latest row, Concurrent Cycles latest active row, Session
  Resume §1): conditionally Block — ONLY when the active cycle is an F5-style per-pass cycle.**
  In milestone/story-delivery cycles these three sites are **advisory (log_warn + Continue)
  or pass-through**, because such cycles structurally do not carry per-pass trajectory tails on
  those rows.

Cycle-type detection reuses the machinery the hook **already has**: the INDEX.md arm already
reads `current_cycle:` from STATE.md frontmatter via `extract_current_cycle()` (per F-SP3-001).
The same resolved cycle name classifies the cycle:

- An **F5-style per-pass cycle** is identified by a per-cycle marker. The production-grade
  discriminator is an explicit per-cycle config/frontmatter flag — e.g., a
  `per_pass_trajectory: true` field in the cycle's `INDEX.md` frontmatter (authored by
  state-manager at cycle creation) — NOT a hardcoded cycle-name match (which would repeat the
  F-SP3-001 hardcoding defect). Fallback when the flag is absent or unreadable:
  **fail-open to advisory** for sites 3/4/5 (NEVER fail-open to Block — a Block fail-open is
  the brick). Sites 1/2 are unaffected by the flag and always Block.

- **Pro:** Production-grade. Preserves the full F5 per-row degradation Block (root-cause-1
  satisfied where it actually applies). Eliminates brick risk in milestone cycles
  (root-cause-2 satisfied). No fabricated data. Reuses existing `extract_current_cycle()`
  plumbing — "wiring not redesign." Generalizes correctly to all future cycles.
- **Con:** Most spec + code surface of the three options (new per-cycle flag + conditional
  Block routing + new edge cases). Requires un-SEALing the BC. Requires the cycle-classifier
  flag to be authored on F5 cycle INDEX.md (a one-line state-manager obligation).
- **Verdict:** RECOMMENDED. It is the only option that is correct under **both** readings
  simultaneously without writing meaningless data and without demoting a genuine F5 invariant.

## Decision

**Root cause 2 (cycle-convention mismatch) DOMINATES, with root cause 1 valid only within
F5-style cycles.** Evidence:

- The live artifact is in a **story-delivery cycle** (`current_cycle: v1.0-brownfield-backfill`)
  whose latest Phase Progress row (D-524) and latest active Concurrent Cycles row are
  **milestone-form**, structurally carrying no per-pass tail. This is the **normal, correct
  state** for a story-delivery cycle — not degradation. The ADV-EDP1-P75-HIGH-002 degradation
  was a real F5-cycle event, but the hook as specced cannot tell a *legitimately tail-less
  milestone row* apart from a *degraded F5 pass row* — that conflation is the BC defect.
- D-454(a) itself motivates **cell-level granularity** because a whole-file count cannot
  distinguish per-cell presence. The same reasoning extends one level: a per-cell check must
  also be **cycle-aware**, because whether a cell is *required* to carry a tail depends on the
  cycle's row-convention. The D-453(d) registry was authored inside the F5 cycle and silently
  assumed F5 per-pass row conventions universally.

**Recommended option: (c) Cycle-conditional site model.**

Sites 1+2 always Block; sites 3+4+5 Block only in F5-style per-pass cycles (detected via an
explicit per-cycle flag, fail-open-to-advisory otherwise); inv-4 LENGTH=4-strict semantics
preserved unchanged on all sites that are checked.

## LENGTH=4-vs-LENGTH=5 nuance

The adversary noted the live "Full-cycle trajectory" prose uses `→9→9→9→9→11` (5 segments)
while inv-4 is LENGTH=4 strict. Resolution:

- inv-4's two-step marker-prefix check scopes the count to the segment **from the
  `trajectory-tail ` marker to the first `;`** — it does NOT count arrows in free-prose
  trajectory narratives elsewhere in the cell. The live **marker** values are
  `trajectory-tail →9→9→9→11` (line 15) and `trajectory-tail →9→9→9→11` (line 68) — both
  **LENGTH=4, PASS**. The 5-segment `→9→9→9→9→11` is a *prose* "Full-cycle trajectory"
  history string, NOT a `trajectory-tail ` marker, so inv-4 correctly ignores it.
- **This nuance reinforces Option (c) and the marker-prefix discipline.** It must be preserved
  exactly: the fix MUST NOT relax inv-4 to LENGTH≥4 to "accommodate" the 5-segment prose. The
  marker-prefix scoping is precisely what keeps the prose history string from causing a
  false-Block. The regression fixture (below) MUST include a cell that carries BOTH a
  5-segment prose trajectory AND a 4-segment `trajectory-tail ` marker, asserting PASS — this
  locks in that the prose does not leak into the count.
- **Action item for PO:** add an explicit edge case to BC-5.39.009 (e.g., EC "prose
  multi-segment trajectory coexisting with LENGTH=4 marker → PASS") if not already covered by
  the existing first-semicolon-segment scoping; the existing PC1 prose already asserts this for
  current_step but it should be a named EC with a fixture.

## Downstream work and routing

Execution is gated on human authorization to un-SEAL BC-5.39.009 (see Human Gate below).
Once authorized, route as follows (orchestrator dispatches; architect does not execute):

1. **product-owner** (owns BC-5.39.009): un-SEAL → re-spec the STATE.md Block arm per
   Option (c):
   - PC1/PC2 (sites 1/2): unchanged — always Block.
   - PC3/PC4/PC5 (sites 3/4/5): add cycle-conditional gating — Block only when the active
     cycle is F5-style per-pass; otherwise advisory (log_warn + Continue). Fail-open-to-
     advisory (never Block) when the cycle-type flag is absent/unreadable.
   - Add the cycle-type precondition (read per-cycle `per_pass_trajectory` flag via the
     existing `current_cycle:` resolution path; reuse `extract_current_cycle()`).
   - Add EC: milestone-cycle STATE.md write with tail-less Phase Progress / Concurrent Cycles
     / Session Resume §1 rows → NO Block (advisory only).
   - Add EC: LENGTH=4 marker coexisting with multi-segment prose trajectory → PASS (locks the
     marker-prefix scoping).
   - Preserve inv-4 LENGTH=4-strict verbatim. POLICY 14 5-leg parity. Update §D-453(d)
     mapping-table scope column to reflect the cycle-conditional Block.
   - Bump BC-5.39.009 v1.8 → v1.9; re-SEAL (or leave draft pending re-cascade per S-7.01).

2. **architect** (this ADR): on PO acceptance, ARCH-INDEX gets an ADR-023 row (state-manager
   inserts the row — see note). If the cycle-type flag becomes a new cross-cutting convention,
   architect documents it in the relevant architecture section (SS-05 / state-manager
   convention doc).

3. **state-manager** (owns STATE.md, cycle INDEX.md, indexes):
   - Author the `per_pass_trajectory:` flag on F5-cycle INDEX.md frontmatter
     (`v1.0-feature-engine-discipline-pass-1/INDEX.md` → `per_pass_trajectory: true`); ensure
     story-delivery / milestone cycles either omit it or set `false`.
   - Insert the ADR-023 row into ARCH-INDEX (architect drafts ADR file; ARCH-INDEX row +
     version bump is state-manager's atomic 4-index obligation).
   - Do NOT retro-fabricate tails into milestone Phase Progress / Concurrent Cycles / Session
     Resume rows on the live STATE.md (that would be Option (a) paper-compliance — explicitly
     rejected).
   - Verify the live STATE.md sites 1/2 carry LENGTH=4 markers (already true) before the hook
     activates.

4. **test-writer / implementer** (owns S-15.17 code, under TDD):
   - Implementer adjusts `lib.rs` per re-specced BC: split the STATE.md arm into
     always-Block (sites 1/2) and cycle-conditional (sites 3/4/5) paths; read the
     `per_pass_trajectory` flag via the existing cycle-resolution path; fail-open-to-advisory.
   - test-writer authors the **real-STATE.md regression fixture** (MANDATORY): a fixture
     derived from the actual live STATE.md (milestone-cycle shape: sites 1/2 with LENGTH=4
     markers, sites 3/4/5 tail-less milestone rows) asserting **NO Block**; plus an F5-shaped
     fixture (per-pass rows with tails on sites 3/4/5) asserting Block-on-missing. Include the
     LENGTH=4-marker-with-multi-segment-prose PASS case.

5. **adversary (LOCAL cascade): RESTARTS from 0/3.** The BC re-spec + code change is a
   material behavioral change to a SEALED contract; the prior pass-5 CLEAN/3-of-3 is voided
   for the new behavior. A fresh 3-CLEAN LOCAL cascade on the re-specced BC + adjusted
   implementation is required before merge (BC-5.39.001 3-CLEAN), OR explicit asymptotic-
   acceptance per D-386 Option C if the cascade re-floors.

## Human Gate — un-SEALing BC-5.39.009

**YES — the recommended option (and Option b) requires un-SEALing BC-5.39.009 v1.8, which was
SEALED at D-522.** Per the VSDD Standing Rule, only the human can authorize amending a SEALED
spec. This ADR does NOT perform that edit. **Required human authorization before any
downstream work begins:** "Un-SEAL BC-5.39.009 to apply ADR-023 Option (c) cycle-conditional
site model." Until that authorization is given, S-15.17 per-story-delivery MUST NOT proceed to
merge/release, because shipping the hook as-currently-specced is a confirmed pipeline-brick.

If the human declines the BC reopen, the only non-bricking fallback is to **hold S-15.17 merge
indefinitely** (do not ship a brick) — Option (a)'s paper-compliance path is rejected on
production-grade grounds and should not be offered as the cheap default.

## Consequences

- **Positive:** Hook ships correct under both cycle types; no brick; no fabricated data; full
  F5 per-row degradation detection preserved; generalizes to future cycles; reuses existing
  `extract_current_cycle()` plumbing.
- **Negative / cost:** Un-SEAL + re-cascade cost; new per-cycle `per_pass_trajectory` flag
  convention to author and maintain; larger code/spec surface than Option (b).
- **Risk if NOT done:** Shipping S-15.17 as-specced bricks the active-cycle pipeline on every
  state-manager STATE.md write (exit 2 cascade Block).

## Anchor justifications

- **ADR-017 / ADR-018** (anchored by S-15.17 frontmatter): ADR-017 (per-story adversary
  phasing) governs the LOCAL cascade restart-from-0/3 obligation stated above; ADR-018 (WASM
  plugin context resolvers) governs the `extract_current_cycle()` / `host::read_file`
  cycle-resolution machinery this ADR reuses for cycle-type detection. Both are directly
  load-bearing here.
- **Module ownership:** `crates/hook-plugins/validate-trajectory-tail-cell-completeness`
  exists in the S-15.17 worktree (priority 158, per ARCH-INDEX hooks-registry allocation
  line); referenced as the implementing module. No planned-but-absent modules are referenced.
- **Subsystem:** SS-05 per BC-5.39.009 `subsystem: SS-05` and the BC-5.39.005..009 family
  membership.
- **Next ADR ID:** ADR-023 — next monotonic after ADR-022 (latest registered in ARCH-INDEX
  v2.06 changelog; ARCH-INDEX is at v2.15 with no ADR added since).
