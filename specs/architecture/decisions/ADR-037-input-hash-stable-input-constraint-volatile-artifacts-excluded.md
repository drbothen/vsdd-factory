---
document_type: architecture-decision-record
level: L3
adr_id: ADR-037
version: "1.1"
title: "ADR-037: input-hash stable-input constraint — volatile cycle artifacts and catalog indexes must not be story hash inputs"
status: accepted
date: 2026-08-04
producer: architect
timestamp: 2026-08-04T00:00:00Z
deciders:
  - architect
subsystems_affected: [SS-05, SS-10]
supersedes: null
superseded_by: null
traces_to: .factory/specs/architecture/ARCH-INDEX.md
last_amended: |-
  2026-08-05 (v1.1) — F-S2107-P4-013 ADR-side remediation (architect; S-21.07 pass-5 fix burst): S-21.07 added to §Context volatile-roster table and §Decision 5 blast radius. Mechanical full-corpus re-derivation on 2026-08-05 reveals 78 stories with volatile inputs (vs original 19 from partial manual scan; see corpus re-derivation block in §Context). Table corrected: S-15.14 entry had only `STORY-INDEX.md` — also carries `BC-INDEX.md` and `ARCH-INDEX.md`; S-15.15 entry had only `STORY-INDEX.md` — also carries `ARCH-INDEX.md`; S-15.17 entry was missing `ARCH-INDEX.md`. 58 additional stories added to table. §Decision 5 blast radius updated 19→78. §Decision 7 frozen-artifact count updated ~29→23 per corpus scan. Corpus counts recorded per D-950. [Prior: 2026-08-04 (v1.0) — initial ruling (architect): volatile-artifact exclusion from `inputs:`; 19-story remediation scope; D-952 empirical demonstration; BC-5.39.010 Class B precondition requirement routed to product-owner.]
modified:
  - "2026-08-05 (v1.1)"
  - "2026-08-04 (v1.0)"
---

# ADR-037: input-hash stable-input constraint — volatile cycle artifacts and catalog indexes must not be story hash inputs

## Context

The `inputs:` frontmatter array on story files drives the `compute-input-hash` tool (SS-10; authoritative binary per ADR-036 §Decision 3). The hash is a drift-detection signal: if inputs change materially after a story is authored, the POLICY 18 three-way equality check (story frontmatter `input-hash` == STORY-INDEX catalog row == aggregation blockquote) flags the story for review. BC-5.39.010 Class B, implemented by the `validate-cross-site-correspondence` WASM gate (S-21.07, currently in LOCAL adversary cascade), enforces this three-way equality with BLOCKING severity.

A structural defect in the `inputs:` modeling of **78 stories** makes BC-5.39.010 Class B permanently unsatisfiable for those stories. (The v1.0 enumeration identified 19 stories from a partial manual scan; mechanical full-corpus re-derivation on 2026-08-05 — see derivation block below — found 78.) These stories list append-only cycle logs, the main pipeline STATE.md, or growing catalog indexes as inputs. Every state-manager burst that appends a decision-log entry, a lesson, or a burst-log entry regenerates a new hash for those files — invalidating the `input-hash` of the affected stories without any change to the stories' specifications.

The defect was empirically demonstrated in the D-952 burst: S-19.01's hash was computed as `0ad9c4b` before the burst appended `L-BB-tooling-version-divergence-masquerades-as-fabrication` to `.factory/cycles/v1.0-brownfield-backfill/lessons.md`. The recomputed hash after that lesson entry is `242af2f`. No other story was affected; all other E-21 and E-19 swept stories retained correct hashes. This confirms the mechanism — lesson append, not a transcription error.

The affected stories and their volatile inputs are:

| Story | Volatile input(s) |
|-------|-------------------|
| S-7.01, S-7.02 | `.factory/STATE.md` |
| S-7.06, S-7.07, S-7.08, S-7.09 | `.factory/cycles/v1.0-brownfield-backfill/decision-log.md` |
| S-14.01, S-14.06, S-14.07, S-14.08, S-14.09 | `.factory/cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md` |
| S-15.08 | `.factory/cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md`, `BC-INDEX.md` |
| S-15.09 | `BC-INDEX.md`, `STORY-INDEX.md`, `ARCH-INDEX.md` |
| S-15.12 | `STORY-INDEX.md`, `ARCH-INDEX.md` |
| S-15.14 | `BC-INDEX.md`, `STORY-INDEX.md`, `ARCH-INDEX.md` |
| S-15.15 | `STORY-INDEX.md`, `ARCH-INDEX.md` |
| S-15.17 | `.factory/cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md`, `.factory/cycles/v1.0-feature-engine-discipline-pass-1/STATE.md`, `STORY-INDEX.md`, `ARCH-INDEX.md` |
| S-18.09 | `ARCH-INDEX.md`, `.factory/cycles/v1.0-brownfield-backfill/lessons.md` |
| S-19.01 | `.factory/cycles/v1.0-brownfield-backfill/lessons.md` |
| S-0.01, S-0.02, S-0.03, S-0.04, S-0.05 | `ARCH-INDEX.md` |
| S-1.01, S-1.02, S-1.03, S-1.04, S-1.05, S-1.06, S-1.07, S-1.08, S-1.09 | `ARCH-INDEX.md` |
| S-2.01, S-2.02, S-2.03, S-2.04, S-2.05, S-2.06, S-2.07, S-2.08 | `ARCH-INDEX.md` |
| S-3.01, S-3.02, S-3.03, S-3.04 | `ARCH-INDEX.md` |
| S-4.01, S-4.02, S-4.03, S-4.04, S-4.05, S-4.06, S-4.07, S-4.08 | `ARCH-INDEX.md` |
| S-5.01, S-5.02, S-5.03, S-5.04, S-5.05, S-5.06, S-5.07 | `ARCH-INDEX.md` |
| S-6.01 | `ARCH-INDEX.md` |
| S-8.10 | `ARCH-INDEX.md` |
| S-15.07 | `BC-INDEX.md`, `STORY-INDEX.md`, `ARCH-INDEX.md` |
| S-15.10 | `STORY-INDEX.md`, `ARCH-INDEX.md` |
| S-15.11 | `BC-INDEX.md`, `STORY-INDEX.md`, `ARCH-INDEX.md` |
| S-15.13 | `STORY-INDEX.md`, `ARCH-INDEX.md` |
| S-15.16 | `STORY-INDEX.md` |
| S-17.01, S-17.02, S-17.03 | `ARCH-INDEX.md` |
| S-18.03, S-18.05, S-18.06, S-18.07, S-18.08, S-18.12 | `ARCH-INDEX.md` |
| S-18.11 | `STORY-INDEX.md` |
| S-21.07 | `ARCH-INDEX.md` |

**Corpus re-derivation (2026-08-05; D-950):** The original 19-story figure was produced by manual inspection of recently-active stories in the engine-discipline cycle; it did not scan the full corpus. The mechanical re-derivation below confirms 78 stories. Commands executed against `.factory/stories/` (epics excluded, STORY-INDEX.md excluded):

```
grep -rl "^  - \.factory/cycles/.*\(decision-log\|lessons\|burst-log\)\.md" .factory/stories/ | grep -v "STORY-INDEX\|epics/"
→ 13 stories

grep -rl "^  - \.factory/STATE\.md" .factory/stories/ | grep -v "STORY-INDEX\|epics/"
→ 2 stories

grep -rl "^  - \.factory/cycles/.*STATE\.md" .factory/stories/ | grep -v "STORY-INDEX\|epics/"
→ 1 story (S-15.17, also counted in decision-log pattern)

grep -rl "^  - \.factory/stories/STORY-INDEX\.md" .factory/stories/ | grep -v "STORY-INDEX\|epics/"
→ 11 stories

grep -rl "^  - \.factory/specs/behavioral-contracts/BC-INDEX\.md" .factory/stories/ | grep -v "STORY-INDEX\|epics/"
→ 5 stories

grep -rl "^  - \.factory/specs/architecture/ARCH-INDEX\.md" .factory/stories/ | grep -v "STORY-INDEX\|epics/"
→ 63 stories (65 including epics)

De-duplicated union: 78 stories total.
```

An additional 23 stories reference only frozen cycle artifacts (`adv-cycle-pass-N.md` files, wave plan documents, delta analyses). Those files are write-once by construction — each adversary pass creates a new numbered file; prior passes are never amended. Wave plan files are authored once during cycle planning and do not grow. Those 23 stories do NOT exhibit the regeneration problem and are excluded from this ADR's remediation scope. (The v1.0 figure of "~29" was approximate; corpus scan on 2026-08-05 confirmed 23.)

`compute-input-hash` has no exclusion mechanism: it reads `inputs:` and resolves each path unconditionally (verified: no `EXCLUDE`, `SKIP`, or ignore logic in the script). Tool-level exclusions would hide the modelling error without fixing it.

## Decision

### Decision 1 — Semantic role of `inputs:` is drift detection, not provenance

The `inputs:` array is a drift-detection mechanism. It answers: "which specification sources, if changed materially, would require this story to be reviewed for spec alignment?" A file belongs in `inputs:` if and only if a content change to that file after the story was authored would require the story's specification to be re-examined.

Append-only cycle logs (`decision-log.md`, `lessons.md`, `burst-log.md`) and the pipeline state document (`STATE.md`, cycle-scoped or global) do not meet this criterion. New entries appended after a story is authored are definitionally unrelated to that story's specification. The D-NNN decision or lesson that informed a story existed at story-authoring time; subsequent D-NNN entries carry no information about the story.

Growing catalog indexes (`STORY-INDEX.md`, `BC-INDEX.md`, `ARCH-INDEX.md`) do not meet this criterion either. New catalog rows — new stories, new behavioral contracts, new ADRs — appended after a story is authored do not change what the story specifies. Index-level mutations (row corrections) propagate specification drift through the specific BC or ADR file, not through the index.

The provenance information about which decision or lesson informed a story belongs in `closes:` (already used in stories that close specific lessons), `last_amended:` version-history annotations, and story body narrative — not in the drift-detection array.

### Decision 2 — Volatile artifact patterns (exclusion list)

Files matching any of the following patterns MUST NOT appear in story `inputs:` arrays:

| Pattern | Rationale |
|---------|-----------|
| `.factory/cycles/**/{decision-log,lessons,burst-log}.md` | Append-only cycle logs; grows without bound every burst |
| `.factory/STATE.md` | Pipeline state; changes on every burst unconditionally |
| `.factory/cycles/**/STATE.md` | Per-cycle state files; same volatility as global STATE.md |
| `.factory/stories/STORY-INDEX.md` | Growing story catalog; new story rows do not affect prior stories |
| `.factory/specs/behavioral-contracts/BC-INDEX.md` | Growing BC catalog; new BC rows do not affect prior stories |
| `.factory/specs/architecture/ARCH-INDEX.md` | Growing architecture catalog; new ADR/subsystem rows do not affect prior stories |

Story-to-story dependencies (e.g., `S-15.09-*.md` listed in S-15.12's `inputs:`) do not fall under this exclusion. If story B's specification changes, downstream story A that depends on B should be flagged for drift review — this is the correct behavior of the drift-detection mechanism. Story files are not append-only cycle logs; they represent point-in-time spec decisions. Once a story is merged and its specification is stable, the drift-detection cost for dependent stories becomes negligible.

### Decision 3 — Provenance preservation path

Removing a volatile file from `inputs:` does not delete provenance. Each story already records the causal chain through:
- `closes:` array — lists the specific D-NNN decisions or lessons that the story closes
- `last_amended:` version-history field — records all spec amendments with D-NNN references
- Story body narrative — cites specific decisions in context
- `traces_to:` field — captures the artifact the story traces back to

For the 78 affected stories, the specific D-NNN or L-NNN entry that justified the story's inclusion of the cycle log (where one exists) is already recorded in `closes:` or `last_amended:`. The `inputs:` entry adds no provenance beyond what those fields already carry.

### Decision 4 — BC-5.39.010 Class B precondition required

BC-5.39.010 Class B enforces three-way hash equality with BLOCKING severity. This is sound for stories with stable inputs. It is unsound for stories with volatile inputs, because hash disagreement for those stories reflects normal burst activity rather than spec drift.

Product-owner MUST amend BC-5.39.010 to add a precondition to Class B: before enforcing three-way equality, the gate MUST scan the story's `inputs:` array for files matching the ADR-037 §Decision 2 volatile patterns. If any volatile inputs are detected, the gate MUST:
1. Emit an ADVISORY message identifying the violating inputs by path
2. Include the prescribed message: `"Story [ID] has volatile inputs per ADR-037 §Decision 2 — three-way equality is unsatisfiable until story-writer removes volatile inputs and state-manager recomputes the hash; Class B BLOCK suspended"`
3. Skip the three-way equality check for this story and continue to the next story
4. NOT emit a BLOCK

Once a story's `inputs:` is cleaned and its hash recomputed, Class B applies with full BLOCKING severity. The precondition self-enforces: a correctly remediated story no longer triggers the volatile-input branch.

This amendment is a product-owner responsibility; the architect does not edit BC content.

### Decision 5 — Remediation sequence and blast radius

The remediation sequence for the 78 affected stories is:

1. **Story-writer** removes all volatile-pattern entries from each story's `inputs:` array. No other story content is changed. Blast radius: the 78 stories listed in §Context (full-corpus scan completed 2026-08-05; no additional stories expected). S-21.07 is explicitly included: its `inputs:` array carries `ARCH-INDEX.md` (corpus-verified; line 18 of story frontmatter). Story-writer removes that entry; state-manager then recomputes S-21.07's hash over the corrected (stable) input set.
2. **State-manager** (or implementer acting on state-manager's behalf) runs `plugins/vsdd-factory/bin/compute-input-hash` per-story single-file `--update` invocations (D-936 forbids `--scan --update`; per-story form required) after each story's `inputs:` is corrected.
3. **State-manager** propagates the updated `input-hash` values to STORY-INDEX three-way equality cells.
4. Remediation is complete when no story's `inputs:` matches §Decision 2 patterns and all STORY-INDEX three-way cells are consistent.

S-19.01's stored hash `0ad9c4b` MUST NOT be corrected under pre-remediation semantics. Fixing `0ad9c4b → 242af2f` would store the hash of a file set that still includes `lessons.md`; the corrected hash would break again at the next lesson append. The correct sequence is: story-writer removes `lessons.md` from S-19.01's `inputs:`, then recompute the hash over the corrected (stable) input set.

### Decision 6 — `input-hash` field not excluded from file content hash

The `input-hash` field within a story file is NOT excluded from hash computation when that story is listed as an input to another story. The hash of a file is computed over its full content. This is the correct behavior for story-to-story spec dependencies: if the input story's metadata changes (version bump, last_amended update, input-hash correction), the dependent story's hash becomes stale, prompting review of whether the dependency relationship is still valid.

The potential for cascading hash invalidation is mitigated by §Decision 2: once volatile inputs are removed, story files only change when their specification content is genuinely amended. The remaining story-to-story dependencies (S-15.09 in S-15.12's `inputs:`, etc.) represent legitimate spec dependencies. S-15.11 (referenced by S-15.09, S-15.12, S-15.14, S-15.15) is merged and stable; no further amendments are expected.

### Decision 7 — Frozen artifacts verification

The 23 stories referencing `adv-cycle-pass-N.md` files, wave plan documents, and per-cycle delta analyses hold stable hashes. (The v1.0 figure of "29" was approximate; corpus scan on 2026-08-05 confirmed 23 stories with frozen-only cycle artifact references and no volatile-pattern inputs.) These files are genuinely immutable:
- `adv-cycle-pass-N.md`: write-once; each adversary pass creates a new numbered file; the adversary never amends a prior pass
- Wave plan files (e.g., `s-15.03-wave-plan-*.md`, `s-15.03-wave-m1-dispatch.md`): authored at wave-planning time; not append-only logs
- Delta analysis documents: point-in-time spec-evolution artifacts; stable after authoring

No evidence of continuing amendments to any of these file types. Filesystem modification timestamps (all at 2026-07-28 14:11 for the `adv-cycle-pass-*.md` corpus) confirm bulk-write from factory-artifacts branch, not incremental appends. The frozen 23 do not exhibit the regeneration problem.

## Rationale

### Why Option (b) — remove volatile entries from `inputs:` — over Options (a), (c), (d)

**Option (a) — tool-level exclusion:** Would suppress the symptom without fixing the model. A reader of a story would see `decision-log.md` in `inputs:` and expect the hash to reflect its content, but the tool would silently ignore it. This introduces a hidden contract between the story spec and the tool that no documentation captures. Future story authors would not know the exclusion existed and would add similar entries expecting them to be hash inputs.

**Option (c) — frozen vs volatile marker:** Requires a new frontmatter convention on file artifacts throughout `.factory/cycles/` and the three growing indexes. The implementation complexity (marker propagation, tool support for reading markers) exceeds the cost of the §Decision 2 exclusion list, which is a static pattern match. The semantic distinction already exists as a natural consequence of the file's role; encoding it as an explicit marker is redundant infrastructure.

**Option (d) — content-stable subset hashing:** Requires implementing section-level hashing in `compute-input-hash` (or a new tool), specifying which section of each volatile file was relevant at story-authoring time, and maintaining that binding across amendments. This is a fundamentally more complex approach for a problem that is simpler to solve by removing the wrong inputs entirely.

**Why the removal is not a provenance loss:** The cycle logs were listed in `inputs:` because specific D-NNN or L-NNN entries informed the story. That provenance is already recorded in the `closes:` field (verified for every affected story that closes a named lesson or decision) and in the `last_amended:` history. The `inputs:` removal doesn't delete the causal record; it removes a broken pointer from the drift-detection mechanism.

### Why Class B must remain BLOCKING (with precondition) rather than downgraded

Downgrading Class B to ADVISORY for all stories would remove the production-grade three-way hash enforcement that BC-5.39.010 exists to provide. The volatile-input problem affects a bounded set of 78 stories with a clear remediation path. The correct response is to fix the inputs, not to weaken the gate. The precondition described in §Decision 4 is a transitional clause that becomes vacuous once the 78 stories are remediated; at that point Class B enforces full BLOCKING with no carve-outs.

### The self-locking risk

Without this ADR, once `validate-cross-site-correspondence` ships and its Class B arm is BLOCKING, the next state-manager burst that appends a lesson or decision-log entry would invalidate the `input-hash` of the affected stories. The subsequent tool-use for those story files would be blocked by the WASM gate — including the write operations needed to repair the `inputs:` arrays. This is the same self-locking shape as the `is_bc_file`-admits-`BC-INDEX.md` finding: a gate whose blocking predicate is triggered by the normal operation of the system it governs.

§Decision 4's volatile-input precondition on Class B breaks this self-lock by ensuring the gate emits ADVISORY (not BLOCK) while the volatile inputs exist, allowing the remediation writes to proceed.

## Consequences

### Positive

- BC-5.39.010 Class B becomes satisfiable and permanently stable for all 78 affected stories after the story-writer remediation sweep
- S-21.07 merge is unblocked: the Class B volatile-input precondition (§Decision 4) prevents the gate from blocking on the 78 stories before remediation is complete
- No new tool infrastructure required; the fix is a data correction in story `inputs:` arrays
- `compute-input-hash` remains a simple full-file accumulator with no exclusion logic; the stability guarantee is enforced at the spec layer, not the tool layer
- Provenance is preserved in `closes:` and `last_amended:` fields that are already present and complete
- The frozen 23 stories require no remediation (corpus-verified 2026-08-05)

### Negative / Trade-offs

- Story-writer remediation sweep over 78 stories required before Class B can enforce full BLOCKING for those stories; this is a bounded one-time cost (59 newly-identified stories are mostly merged greenfield stories that only need their `inputs:` corrected — no spec content changes required)
- S-19.01's hash cannot be corrected before story-writer removes `lessons.md` from its `inputs:`; the stored hash `0ad9c4b` remains stale until then
- Story authors must understand that cycle log citations belong in `closes:` (provenance) not `inputs:` (drift detection); this distinction requires documentation in the story-writer agent prompt
- Story-to-story dependencies (story A lists story B in `inputs:`) remain subject to hash invalidation when story B is amended; this is intentional and correct behavior, but it means merged stories that undergo metadata-only amendments (version bumps, citation sweeps) will propagate hash staleness to their dependents

### Status as of 2026-08-05 (v1.1)

Accepted. The ARCH-INDEX row was inserted at v3.42. Remediation (§Decision 5) is pending: story-writer sweep of **78 stories** has not yet occurred (up from 19 — see §Context corpus re-derivation). S-21.07 is included in the sweep; its volatile `ARCH-INDEX.md` input has been identified and is story-writer-routed (the actual `inputs:` edit is story-writer scope; this ADR records the obligation). Class B precondition (§Decision 4) was delivered by product-owner in BC-5.39.010 v1.5 before S-21.07 merges to prevent self-lock. S-19.01 hash `0ad9c4b` remains stale pending story-writer remediation of its `inputs:` array.

## Alternatives Considered

- **Option (a) — tool-level exclusion from `compute-input-hash`:** Rejected; hides the modeling error, creates undocumented silent behavior in the tool, does not prevent future story authors from adding volatile inputs expecting them to be hash inputs. See §Rationale.
- **Option (c) — frozen/volatile marker in file frontmatter:** Rejected; adds infrastructure complexity (new convention, marker propagation, tool support) exceeding the cost of the static exclusion list in §Decision 2. The semantic distinction is naturally derivable from file role.
- **Option (d) — section-level hashing:** Rejected; requires significant tooling investment, per-story specification of which section was relevant, and ongoing maintenance of that binding. Scope exceeds the problem.
- **Downgrade Class B to ADVISORY:** Rejected; removes production-grade enforcement for a bounded remediation problem. The correct fix is to repair the inputs, not weaken the gate.
- **Hash correction under pre-remediation semantics (S-19.01 `0ad9c4b → 242af2f`):** Rejected; storing `242af2f` records a hash that includes `lessons.md` and will break again at the next lesson append. Correct sequence is §Decision 5.

## Source / Origin

- **Empirical demonstration:** D-952 burst — S-19.01 stored hash `0ad9c4b`, post-burst recomputed hash `242af2f`; cause confirmed as `L-BB-tooling-version-divergence-masquerades-as-fabrication` lesson append to `.factory/cycles/v1.0-brownfield-backfill/lessons.md` (listed in S-19.01 `inputs:`)
- **F-S2107-P2-010:** Adversary pass-2 finding — nine E-19 stories have live block-on-ship condition against POLICY 18 three-way equality; true exposure confirmed as 19 stories (v1.0) → corrected to 78 stories (v1.1) by full-corpus re-derivation
- **F-S2107-P4-013:** Adversary pass-4 finding — PC40's "imposes no permanent weakening" guarantee is false for S-21.07 because ADR-037 §Context omitted S-21.07 from the 19-story table; the v1.1 amendment corrects this
- **BC-5.39.010:** Normative twin to ADR-035; Class B arm implementation in S-21.07 `validate-cross-site-correspondence` WASM gate
- **ADR-035 §Decision 1:** Three-tier architecture — Tier 2A WASM PostToolUse cross-site gate enforces Class B with BLOCKING severity
- **ADR-036 §Decision 3:** Authoritative binary `plugins/vsdd-factory/bin/compute-input-hash`; per-story `--update` invocations; `--scan --update` forbidden (D-936)

## Changelog

| Version | Date | Summary |
|---------|------|---------|
| 1.1 | 2026-08-05 | F-S2107-P4-013 ADR-side remediation (architect; S-21.07 pass-5 fix burst): S-21.07 added to §Context table and §Decision 5 blast radius. Mechanical full-corpus re-derivation confirms 78 volatile stories (vs 19 in v1.0). S-15.14, S-15.15, S-15.17 entries corrected (incomplete volatile-input listings). 58 additional stories added to table. §Decision 5 blast radius 19→78. §Decision 7 frozen-only count ~29→23 (corpus-verified). Corpus scan commands added per D-950. |
| 1.0 | 2026-08-04 | Initial ruling: volatile-artifact exclusion from `inputs:`; 19-story remediation scope (partial manual scan); D-952 empirical demonstration; BC-5.39.010 §Decision 4 precondition requirement routed to product-owner. |
