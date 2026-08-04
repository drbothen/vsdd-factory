---
document_type: architecture-decision-record
level: L3
adr_id: ADR-036
version: "1.0"
title: "ADR-036: compute-input-hash binary version authority, algorithm migration, and invariant-11 extension"
status: accepted
date: 2026-08-03
producer: architect
timestamp: 2026-08-03T00:00:00Z
deciders:
  - architect
supersedes: null
superseded_by: null
traces_to: .factory/specs/architecture/ARCH-INDEX.md
related_adrs:
  - ADR-035 (cross-site correspondence validation three-tier architecture — §Decision 5 governs validate-input-hash `on_error = "block"` audit and fuel error taxonomy; this ADR fixes the binary-version ambiguity that made POLICY 18 unenforceable as written)
anchors:
  - SS-07
  - SS-10
subsystems_affected:
  - SS-07
  - SS-10
last_amended: "2026-08-03 (v1.0) — initial ruling (architect): binary version authority; bootstrap migration; mandatory invocation path; invariant-11 algorithm-divergent extension; E-19/E-21 remediation sequence. Addresses F-S2107-P2-010 and unblocks S-21.07 merge."
modified:
  - "2026-08-03 (v1.0)"
---

# ADR-036: compute-input-hash binary version authority, algorithm migration, and invariant-11 extension

## Context

`compute-input-hash` is the tooling authority for POLICY 18 input-hash bookkeeping. As of
2026-08-03 two physically distinct copies exist, and they produce different outputs for identical
inputs:

| Copy | Path | Size | Date | Provenance |
|------|------|------|------|------------|
| CACHE | `~/.claude/plugins/cache/claude-mp/vsdd-factory/1.0.0-rc.23/bin/compute-input-hash` | 15,816 B | 2026-07-18 | marketplace tarball, rc.23 |
| DEV | `plugins/vsdd-factory/bin/compute-input-hash` | 18,056 B | 2026-07-22 | develop branch |

Both are Bash scripts. The divergence traces to two commits merged to `develop` on 2026-07-22,
after rc.23 was tagged on or before 2026-07-18:

- `e628b884` — "fix(bin): preserve trailing newlines in compute-input-hash accumulation (#715)"
  Replaced the core accumulation pattern from
  `CONCAT="${CONCAT}$(cat file)"` + `echo -n "$CONCAT" | md5sum/md5`
  to temp-file accumulation `cat file >> $HASH_INPUT` + `md5sum < $HASH_INPUT`.
  Shell command substitution `$(cat file)` unconditionally strips trailing newlines; the new path
  preserves every raw byte. Issue #637 is the defect report; the fix comment is explicit.

- `15a85f43` — "fix(bin): compute-input-hash --update upserts absent input-hash field (#718)"
  Reworked `--update` mode to detect field-presence separately from field-value; previously a
  missing `input-hash:` field caused a silent no-op (exit 0, no write) instead of an insert.
  Fixes #623.

The `validate-input-hash.sh` hook (installed at `$CLAUDE_PLUGIN_ROOT/hooks/validate-input-hash.sh`)
resolves its hash tool at runtime:

```bash
PLUGIN_ROOT="${CLAUDE_PLUGIN_ROOT:-}"
if [[ -z "$PLUGIN_ROOT" ]]; then
  PLUGIN_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
fi
HASH_TOOL="$PLUGIN_ROOT/bin/compute-input-hash"
```

When the harness sets `CLAUDE_PLUGIN_ROOT` (normal operation), this resolves to the CACHE
copy. Agents that follow CLAUDE.md Tooling documentation invoking
`plugins/vsdd-factory/bin/compute-input-hash` (the DEV copy) therefore compute hashes the hook
then rejects — POLICY 18's mechanical verification mandate is impossible to satisfy
unambiguously while the two binaries diverge.

The E-21 hash corpus as measured by the orchestrator illustrates the consequence:

| Story | Stored | DEV | CACHE | Classification |
|-------|--------|-----|-------|----------------|
| S-21.01 | 32aaccc | 4bde987 | 9c2e2c0 | stale-by-content |
| S-21.02 | 8bd32e5 | 9f5c7e9 | 34a5cea | stale-by-content |
| S-21.03 | 59e687e | f902141 | 9ea7a2c | stale-by-content |
| S-21.04 | 47a65c9 | 47a65c9 | 1acf3c6 | algorithm-divergent (DEV-correct) |
| S-21.05 | c9265f0 | 6f20e8e | a16a71b | stale-by-content |
| S-21.06 | b807086 | 537b230 | 716441a | stale-by-content |
| S-21.07 | 52f0bf3 | 88fc803 | 52f0bf3 | algorithm-divergent (CACHE-correct) |

A human-supplied constraint is in force: **no rc.24 release until E-21 completes.** E-21 is
7 stories / 46 points / 4 waves; S-21.07 is the final story and is currently in LOCAL adversary
cascade. A "just release it" solution would resolve the divergence but violates this constraint.

The adversarial review finding `F-S2107-P2-010` (HIGH) identifies nine E-19 stories with
story-frontmatter↔index hash disagreement and three with catalog↔blockquote disagreement.
Once S-21.07 ships (POLICY 18 WASM gate — Class B three-way equality enforcement), every
write to an E-19 or E-21 story file or STORY-INDEX row will exit 2 and block. Reconciliation
must happen before the merge.

## Decision

### Decision 1 — DEV algorithm (commit e628b884) is the canonical algorithm

The CACHE binary's trailing-newline stripping via `$(cat file)` is a correctness defect. Raw-byte
hashing (preserving all bytes including trailing newlines) is the only semantically stable
definition: it matches `md5sum <file`, `md5 <file`, and any byte-level reader such as Python
`hashlib.md5(open(f,'rb').read())`. The temp-file accumulation in commit `e628b884` implements
this correctly. The CACHE binary produced an algorithm-dependent hash that cannot be reproduced
by any external tool from the same file bytes, and that diverges from the same tool after a
bugfix was applied. Referencing ADR-036 §Decision 1.

### Decision 2 — Bootstrap migration: copy DEV binary to the CACHE path

The earliest unblocking path — one that does not require an rc.24 release, does not require
session environment changes, and makes the hook enforce the correct algorithm — is a one-time
local patch: copy `plugins/vsdd-factory/bin/compute-input-hash` (DEV) to
`~/.claude/plugins/cache/claude-mp/vsdd-factory/1.0.0-rc.23/bin/compute-input-hash` (CACHE).

After this patch:
- The hook resolves to the patched CACHE binary → uses DEV algorithm
- Agents invoking `plugins/vsdd-factory/bin/compute-input-hash` → same algorithm
- Both paths are byte-identical; divergence is eliminated without a release

This patch is an operator-local file change, not a release artifact. If rc.23 is re-installed
from the marketplace, the patch is lost; re-apply immediately after any marketplace reinstall.
At rc.24 release, the marketplace tarball ships the DEV binary and the patch is permanent. The
devops-engineer executes this step; it is a prerequisite for all remediation in §Decision 5.

Verification after patching:

```bash
cmp plugins/vsdd-factory/bin/compute-input-hash \
    ~/.claude/plugins/cache/claude-mp/vsdd-factory/1.0.0-rc.23/bin/compute-input-hash
echo $?   # must be 0
```

### Decision 3 — Mandatory invocation path and self-identification

After the bootstrap migration, the single canonical invocation an agent MUST use is:

```
plugins/vsdd-factory/bin/compute-input-hash <artifact-file> [--update|--check|--resolve]
```

This path is what CLAUDE.md Tooling section documents (modulo the pre-existing `bin/` vs
`plugins/vsdd-factory/bin/` path error — see §Consequences). After the migration it is
byte-identical to what the hook enforces.

An agent that suspects it may be running the wrong binary MUST compare the two copies:

```bash
cmp plugins/vsdd-factory/bin/compute-input-hash \
    ~/.claude/plugins/cache/claude-mp/vsdd-factory/1.0.0-rc.23/bin/compute-input-hash
```

If `cmp` reports divergence, the migration has not been applied or the cache was reinstalled;
halt and request operator intervention before computing any new hashes.

The tool does not currently expose a `--version` flag. A follow-up story SHOULD add one so that
agents can self-identify the binary version programmatically rather than via `cmp`. This is
tracked as a follow-up to this ADR; see §Consequences.

### Decision 4 — Invariant-11 extension: algorithm-divergent is a distinct third classification

BC-5.39.010 invariant 11 distinguishes two hash classifications: stale (hash was correct at
computation time, inputs subsequently changed) and fabricated (hash was never computed from
actual inputs at any revision). The binary-divergence incident reveals a third category that
does not fit either:

**Algorithm-divergent**: A hash that was legitimately computed from the actual inputs using a
specific binary version whose algorithm differs from the current operational authority binary.
The hash is correct-for-its-algorithm on those inputs. It does not match what the current
authoritative binary would compute for the same inputs, not because the inputs changed but
because the two binaries implement different hash semantics.

Key distinctions from the existing categories:

| | Stale | Fabricated | Algorithm-divergent |
|---|---|---|---|
| Was it computed from real inputs? | Yes, at time T | No | Yes (current or prior) |
| Did inputs change? | Yes | N/A | Not necessarily |
| Does auth binary match? | No (new inputs) | No (never valid) | No (different algorithm) |
| Provenance break annotation required? | No | Yes | No |
| Remedy | Recompute with auth binary | Annotate + recompute | Recompute with auth binary |

The PROVENANCE-BREAK annotation (per BC-5.39.010) is reserved for fabricated hashes only.
Algorithm-divergent hashes are remediable without any annotation — they simply need
recomputation with the authoritative binary. Referencing ADR-036 §Decision 4.

### Decision 5 — E-19/E-21 remediation sequence

Prerequisite: §Decision 2 bootstrap migration COMPLETE (verified by `cmp` exit 0).

All remediation uses single-file `compute-input-hash <file> --update` invocations only.
The `--scan --update` batch form with 418-file blast radius is forbidden per D-936. Each
invocation is reviewable individually: state-manager runs each update and records the old→new
hash transition in the STORY-INDEX three-way equality burst.

**Step 1 — Recompute all affected story frontmatter hashes (ground truth)**

Run `plugins/vsdd-factory/bin/compute-input-hash <story-file>` (print mode, no write) for each
affected story to determine the correct hash under the now-unified algorithm. Stories to include:

- E-21 stale-by-content: S-21.01, S-21.02, S-21.03, S-21.05, S-21.06 (five stories; stored NEITHER)
- E-21 CACHE-correct: S-21.07 (stored `52f0bf3`; unified algorithm computes `88fc803`)
- E-21 DEV-correct: S-21.04 (stored `47a65c9`; unified algorithm still `47a65c9` → no change)
- E-19: all nine stories with frontmatter↔index disagreement

**Step 2 — Update frontmatter (Arm B1 ground-truth anchor)**

For each story requiring an update: run `compute-input-hash <story-file> --update`. This writes
the correct hash into the story frontmatter and constitutes the Arm B1 ground truth.

**Step 3 — Sync STORY-INDEX three-way equality (Arm B1 + Arm B2)**

For each updated story, state-manager writes the catalog row and blockquote in STORY-INDEX to
match the new frontmatter hash. This simultaneously resolves:
- Arm B1 (frontmatter↔index disagreement)
- Arm B2 (catalog↔blockquote disagreement) for the three affected E-19 stories (S-19.02, S-19.04,
  S-19.07)

POLICY 18 three-way equality (frontmatter = catalog = blockquote) must hold after each story's
update before proceeding to the next.

**Step 4 — Verify POLICY 18 gate**

After all updates: for each corrected story, run `compute-input-hash <story-file> --check`. Exit 0
confirms frontmatter hash is current. STORY-INDEX catalog and blockquote equality is verified by
inspection. All three legs must be identical before S-21.07 LOCAL adversary cascade proceeds to
pass 3.

## Rationale

### Why DEV algorithm, not CACHE algorithm?

Raw-byte hashing is the correct semantic. No reputable content-addressable storage, integrity
checker, or formal hash specification strips trailing newlines before digesting. The CACHE
binary's `echo -n "$CONCAT"` pattern strips them as a side effect of shell command substitution —
this is a well-known Bash footgun (#637 documents it). Any external reproduction attempt (Python,
md5sum piped from cat, Go io.Reader) would produce a different hash from the CACHE binary for the
same input content. This is an operationally unsound definition for a drift-detection tool.

### Why bootstrap patch rather than waiting for rc.24?

The constraint "no rc until E-21 done" is human-directed. E-21 will not be done until S-21.07
merges. S-21.07 cannot merge until E-19/E-21 hashes are reconciled. Reconciliation requires
knowing which binary is authoritative. Waiting for rc.24 is a circular dependency. The bootstrap
patch breaks the cycle with minimal risk: it changes a single local file that affects only this
session's hash enforcement. If the patch is reverted by a marketplace reinstall, the worst
outcome is hash divergence again — no data loss, no irrecoverable state.

### Why not patch the hook to accept both algorithms?

The hook enforcing the sum of CACHE and DEV outputs would double the false-pass surface and
make POLICY 18 weaker, not stronger. The goal is a single authoritative algorithm, not
multi-algorithm tolerance.

### Why is this an architecture decision rather than a state-manager bookkeeping burst?

The binary-version authority question is a formal architectural precedent that must survive
context loss, team rotation, and future rc releases. It defines which copy of a tool is
authoritative and under what conditions. That is the definition of an ADR. State-manager
bookkeeping of the individual hash corrections (§Decision 5) is downstream of this ADR; the
remediation cannot be executed correctly without first knowing §Decision 1.

## Consequences

### Positive

- POLICY 18 becomes unambiguously satisfiable: after the bootstrap migration, both the hook and
  the documented agent invocation path use the same algorithm.
- E-19 Arm B1 and Arm B2 violations are resolved, unblocking S-21.07 LOCAL adversary cascade.
- The invariant-11 algorithm-divergent category gives state-manager and adversary a correct
  classification for hashes that differ due to algorithm change, preventing future
  misdiagnosis of legitimate hashes as fabricated.
- The CACHE binary correction is permanent when rc.24 ships; no further migration needed.

### Negative / Trade-offs

- The bootstrap patch requires operator execution (devops-engineer or human). It is not
  automated. If forgotten after a marketplace reinstall, divergence silently recurs.
- S-21.07 stores `52f0bf3` (CACHE-computed, previously correct) and must be recomputed to
  `88fc803` (DEV algorithm) before the POLICY 18 gate passes. This is one extra update cycle
  relative to a world where CACHE was declared authoritative.
- The `--version` flag is absent from the current binary; the `cmp` check is the only
  verification path available to agents. This is unergonomic and should be resolved in a
  follow-up story. Tracked as a companion issue to this ADR.
- CLAUDE.md documents `bin/compute-input-hash` (a known path error since pass-29 "CLAUDE.md
  path error" orchestrator note). The correct path is `plugins/vsdd-factory/bin/compute-input-hash`.
  This ADR normalizes the mandatory invocation to the correct path; CLAUDE.md correction is
  routed to the human as a documentation update (CLAUDE.md edits require explicit human
  direction per project operating rules).

### Status as of 2026-08-03

Accepted pending execution of §Decision 2 bootstrap migration. No code has been changed.
§Decision 5 remediation proceeds immediately after `cmp` confirms migration. The four
annotations in the permanent record (STORY-INDEX.md:727, VP-INDEX.md:8, STATE.md:134,
STATE.md:226) that assert `1acf3c6` was fabricated are INCORRECT per §Decision 4 and are
routed to state-manager for correction; see §Ruling on four annotations below.

## Alternatives Considered

- **Option A — Declare CACHE authoritative; mandate CACHE binary path for all agents**: Rejected.
  This perpetuates the trailing-newline bug and requires S-21.04's `47a65c9` (correctly computed
  by the state-manager pass-29/C using DEV) to be reverted to `1acf3c6`. It also requires all
  future hashes to use the buggy algorithm until rc.24. The cleanup at rc.24 release would still
  require a full 15-story rehash. Net: same remediation cost, worse algorithm semantics.

- **Option B — Set `CLAUDE_PLUGIN_ROOT` environment variable to point to a custom directory
  containing only the DEV binary**: Rejected. This requires environment configuration changes
  that persist across sessions and cannot be reliably enforced by the hook itself. Fragile.

- **Option C — Add dual-algorithm tolerance to the hook**: Rejected. Makes POLICY 18 weaker
  (two hashes accepted for any file) and does not address the systematic inconsistency in stored
  values. A one-pass migration to a single algorithm is preferable.

- **Option D — Wait for rc.24**: Rejected under the "no rc until E-21 done" constraint.
  This creates a circular dependency: rc.24 unblocks the fix, but rc.24 requires E-21 completion,
  which requires S-21.07 merge, which requires hash reconciliation. The bootstrap patch breaks the
  cycle.

## Ruling on four pass-29/30 annotations

Addresses the four permanent-record annotations that label `1acf3c6` as a fabricated hash:

**Finding:** `1acf3c6` is exactly what the CACHE binary (rc.23) computes for S-21.04's current
inputs. The CACHE binary at that time (or any time the algorithm was equivalent) would produce
`1acf3c6` for S-21.04. This is not fabrication.

**Root cause of the misdiagnosis:** The orchestrator ran `compute-input-hash --check` (invoking
the DEV binary) and observed `DRIFT — 1acf3c6 ≠ computed 47a65c9`. Since only one binary was
known to exist, the orchestrator concluded that a value not matching DEV's output must be
fabricated. The second binary (CACHE) was not known at pass-29 analysis time. The conclusion
was reasonable given available information but is factually incorrect given the fuller picture.

**Correct classification per ADR-036 §Decision 4:** `1acf3c6` is algorithm-divergent — produced
by the CACHE binary (rc.23, which strips trailing newlines) from S-21.04's inputs. It is not a
fabrication. No PROVENANCE-BREAK annotation is appropriate. The current stored value `47a65c9`
(DEV-computed, set by state-manager in commit `12ad0123` pass-29/C) is correct under the
canonical DEV algorithm and requires no change.

**Required corrections to permanent record** (routed to state-manager):

1. S-21.04 story `modified[]` provenance chain — the entry that records `input-hash: 1acf3c6`
   in the `4be9d21→1acf3c6` transition: replace any assertion that `1acf3c6` was "never
   computed at any revision" or "fabricated" with the correct characterization:
   `[ALGORITHM-DIVERGENT per ADR-036 §Decision 4: 1acf3c6 was produced by CACHE binary (rc.23)
   which strips trailing newlines; corrected to 47a65c9 (DEV algorithm) at pass-29/C;
   no fabrication occurred]`

2. STORY-INDEX.md:727 (S-21.04 row) — any annotation asserting fabrication of `1acf3c6`:
   replace with `[ALGORITHM-DIVERGENT per ADR-036 §Decision 4]` inline note.

3. VP-INDEX.md:8 — verify the text; if it asserts `1acf3c6` was fabricated, replace with
   `[ALGORITHM-DIVERGENT per ADR-036 §Decision 4]`.

4. STATE.md:134 and STATE.md:226 — verify the text at both locations; replace any fabrication
   assertion on `1acf3c6` with `[ALGORITHM-DIVERGENT per ADR-036 §Decision 4]`.

The state-manager executes these corrections in a single burst following this ADR's acceptance.
All four corrections are annotation-level edits that do not change stored hash values or BC
content — they replace one incorrect characterization with the correct one.

**Note on BC-5.39.010:** The D-947 pass-30 fix burst added `[PROVENANCE-BREAK: hash never
computed at any revision; first real computation this burst]` to BC-5.39.010 v1.2. If this
annotation was applied to any `1acf3c6` reference in BC-5.39.010, it is also incorrect per
ADR-036 §Decision 4 and should be corrected. BC content corrections are routed to
product-owner (not state-manager).

## Source / Origin

- `plugins/vsdd-factory/bin/compute-input-hash` (DEV binary, 18,056 B, 2026-07-22)
- `~/.claude/plugins/cache/claude-mp/vsdd-factory/1.0.0-rc.23/bin/compute-input-hash` (CACHE binary, 15,816 B, 2026-07-18)
- Git commit `e628b884db69c7c10eca8c5cec7caf291c170c60` — "fix(bin): preserve trailing newlines in compute-input-hash accumulation (#715)" — the algorithm change
- Git commit `15a85f43004e274ef4f951f0c6aa04337cb5e212` — "fix(bin): compute-input-hash --update upserts absent input-hash field (#718)" — the --update upsert fix
- `~/.claude/plugins/cache/claude-mp/vsdd-factory/1.0.0-rc.23/hooks/validate-input-hash.sh` — hook source that resolves `$CLAUDE_PLUGIN_ROOT/bin/compute-input-hash`
- Adversary finding `F-S2107-P2-010` (HIGH) — E-19 nine-story hash population inconsistency
- BC-5.39.010 §Invariant 11 — stale vs fabricated distinction (this ADR extends to three categories)
- CLAUDE.md §Tooling (documented path `bin/compute-input-hash` contains a pre-existing error noted in S-21.04 pass-29 orchestrator note)
- Git commit `12ad0123d816bc0407cba52d005d3a7a338e2894` — state(pass-29/C) S-21.04 sweep where `1acf3c6` was relabeled as drift and corrected to `47a65c9`
