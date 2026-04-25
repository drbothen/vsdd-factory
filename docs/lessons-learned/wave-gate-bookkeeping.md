# Wave-Gate Bookkeeping — SHA Drift & Narrative-Staleness Defect Class

> **TL;DR:** A real wave-gate convergence cycle hit six consecutive
> recurrences of two defect classes — SHA drift and narrative-staleness
> — before the orchestrator finally executed remediation under a strict
> protocol that broke the loop. This doc captures the trajectory, the
> root cause at each step, and the structural fix shipped in
> vsdd-factory's `state-burst` skill + `verify-sha-currency.sh` hook +
> `state-manager-checklist-template.md`.

## The defect classes

**SHA drift.** A state-manager burst commits to factory-artifacts and
the documents reference SHA values that don't match the actual git
HEAD. Detected by:
- `STATE.md` frontmatter cites `factory-artifacts HEAD: <SHA-A>` while
  `git rev-parse HEAD` is `<SHA-B>`.
- `wave-state.yaml`'s `gate_pass_N.remediation_sha` cites `<SHA-X>`
  while `STATE.md`'s frontmatter pass-N entry cites `<SHA-Y>`.

**Narrative-tense staleness.** A state-manager burst writes
"REMEDIATION IN PROGRESS" voice during the active burst. The voice is
correct at write-time but wrong post-burst. There's no transition step
that rewrites it to "REMEDIATED — Awaiting Pass N+1" — so the next
pass surfaces it as a finding.

Both classes are "narrow-scope burst" failures: the burst fixes the
adversarial findings but doesn't sweep all the bookkeeping fields in
sibling documents.

## The convergence trajectory

In one wave-gate convergence cycle, the adversarial-review pass count
went:

```
Pass 1: 11 findings  (M-001 narrative-staleness)
Pass 2: 12 findings  (H-002 narrative-staleness recurrence)
Pass 3: 10 findings  (H-002 narrative-staleness recurrence + OBS-002 add hook)
Pass 4: 10 findings  (H-002 narrative-staleness recurrence + 4-commit chain)
Pass 5: 11 findings  (H-001 SHA fragmentation, hook tightened)
Pass 6:  7 findings  (H-001 cross-record SHA drift, schema clarified)
Pass 7:  3 findings  (CLEAN — manual orchestrator execution proved the
                       strict protocol works)
```

**Six recurrences before convergence.** Each recurrence taught the
team something new about why the previous fix was insufficient. The
final form is what ships in vsdd-factory now.

## Root-cause chain

The recurrence wasn't "the team kept making the same mistake" — it was
"each fix exposed a deeper assumption that needed structural
correction." The chain:

### 1. Hook scope too narrow (Passes 1-2)

Initial fix: a `verify-sha-currency.sh` hook that ran pre-push and
checked that `STATE.md` cited the current `factory-artifacts HEAD`.

Why it was insufficient: the hook only checked HEAD-currency in two
documents. It didn't catch:
- Mismatch between `STATE.md` and `wave-state.yaml`
- Multi-commit chains where each commit was cited somewhere
- Tense-flip in narrative

### 2. Two-commit protocol added but unbounded (Passes 3-4)

Insight: state-manager bursts can't know their commit's SHA before
committing. So write a placeholder in commit 1, backfill in commit 2.

Why it was insufficient: the "exception" in the hook for the cited
SHA being one commit behind HEAD was unbounded. The team naturally
extended the chain when fixes-of-fixes accumulated:

```
HEAD:    chore: backfill SHAs into pass-3 records       (cites HEAD~3)
HEAD~1:  fix: hook regex for cross-record check         (no backfill)
HEAD~2:  chore: backfill SHAs into pass-3 records       (cites HEAD~3)
HEAD~3:  fix: pass-3 remediation deltas
```

The exception fired four commits deep. The hook said PASS. But every
intermediate commit had its SHA cited somewhere, fragmenting the
canonical SHA across multiple values.

### 3. Per-document SHA writes fragmented citations (Pass 5)

Even within a single 2-commit burst, writing each document's SHA cite
in turn — `STATE.md` first, then `SESSION-HANDOFF.md`, then
`wave-state.yaml` — meant the operator was committing partial
updates. Stage 2 then had to know which intermediate SHA to backfill
where. Mistakes accumulated.

**Structural fix (Single Canonical SHA Rule):** Stage 1 writes the
literal placeholder `15fa97e6` everywhere a SHA is needed. Stage 2 is
a single global `sed s/15fa97e6/<actual>/g` across all three
documents. There is exactly ONE SHA to backfill, in exactly ONE
operation. The hook's two-commit-protocol exception was tightened to
require HEAD has `backfill` AND HEAD^ does NOT — eliminating the
unbounded chain.

### 4. Cross-record drift unmasked (Pass 6)

Even with single-canonical-SHA discipline, an older defect surfaced:
`STATE.md`'s frontmatter `adversary_<wave>_pass_3_*.remediation_sha`
cited the Pass 4 SHA (because Pass 4's burst had partially closed
Pass 3 leftovers). `wave-state.yaml`'s `gate_pass_3.remediation_sha`
cited the original Pass 3 SHA. The two records disagreed.

**Structural fix (Cross-record SHA verification):** the hook now
iterates every wave's `gate_pass_N` records and asserts the
`remediation_sha` agrees with `STATE.md`'s frontmatter pass-N entry.
Mismatches FAIL the gate.

**Schema semantics clarification:** when a single burst closes
findings from multiple prior passes, each pass's `remediation_sha`
points to where IT was first remediated (even partially). Subsequent
re-closures don't advance the SHA backward. Documented in the
checklist's "Schema Semantics" section.

### 5. Manual execution proved the protocol works (Pass 7)

The orchestrator manually executed Pass 7's remediation under the
strict Single Canonical SHA + Two-Commit Protocol. The result: 3
findings, all observation-level — the first clean pass after six
recurrences.

The protocol is now codified in:
- `plugins/vsdd-factory/skills/state-burst/SKILL.md` — the procedural
  guide.
- `plugins/vsdd-factory/templates/state-manager-checklist-template.md`
  — the bookkeeping checklist (instantiate into `.factory/STATE-MANAGER-CHECKLIST.md`).
- `plugins/vsdd-factory/templates/verify-sha-currency.sh` — the hook
  (instantiate into `.factory/hooks/verify-sha-currency.sh`).
- `plugins/vsdd-factory/agents/state-manager.md` — agent prompt
  references the skill and lists the anti-patterns.

## Anti-pattern catalog

These are the specific shapes the recurrences took. The hook detects
each one.

### Tense flip in Stage 1 narrative

```diff
- ## Pass 4 — REMEDIATION IN PROGRESS
- Closing findings from Pass 3...
+ ## Pass 4 — REMEDIATED — Awaiting Pass 5
+ Findings closed at SHA 15fa97e6 (Stage 1 placeholder).
```

Detected by hook's `TENSE_FLIP_PATTERNS` regex: `IN_PROGRESS`,
`in progress`, `this burst remediates`, `burst remediates`,
`REMEDIATION IN PROGRESS`. WARN-level (not FAIL).

### 3rd-commit chain extension

```
HEAD:    chore: backfill stage-1 SHA       (backfill=YES)
HEAD~1:  chore: backfill stage-1 SHA       (backfill=YES)  ← FAILURE
HEAD~2:  fix: original Stage 1
```

Detected by hook's multi-commit-chain check: HEAD and HEAD^ both
contain `backfill`. Reports `MULTI_COMMIT_CHAIN_NOT_ALLOWED` and
FAILs.

### Fragmented SHA citations within a single burst

```diff
# STATE.md frontmatter
- adversary_wave_1_pass_4_*: { remediation_sha: a1b2c3d4 }
+ adversary_wave_1_pass_4_*: { remediation_sha: 15fa97e6 }

# wave-state.yaml
- gate_pass_4: { remediation_sha: e5f67890 }
+ gate_pass_4: { remediation_sha: 15fa97e6 }

# SESSION-HANDOFF.md
- factory-artifacts HEAD: 12345678
+ factory-artifacts HEAD: 15fa97e6
```

The Stage 2 backfill is a single `sed s/15fa97e6/<actual>/g` across
all three files. Three different SHAs in Stage 1 means Stage 2 has to
sort out which-SHA-goes-where, and the mistakes compound.

### Cross-record SHA drift (post-burst)

```yaml
# wave-state.yaml — Pass 3 was first remediated at b1b145b3
gate_pass_3: { remediation_sha: b1b145b3 }

# STATE.md — frontmatter erroneously cites Pass 4's burst SHA
adversary_wave_1_pass_3_*: { remediation_sha: 99563fd1 }
                                                # ↑ DRIFT
```

Detected by hook's cross-record check: the python+yaml loader
iterates every wave's `gate_pass_N`, looks up the matching STATE.md
frontmatter entry, and reports `DRIFT` if they disagree.

## What ships in vsdd-factory

| Artifact | Where it lives | What it does |
|----------|----------------|--------------|
| `state-burst` skill | `plugins/vsdd-factory/skills/state-burst/SKILL.md` | Procedural guide for executing the protocol |
| Checklist template | `plugins/vsdd-factory/templates/state-manager-checklist-template.md` | Instantiate into `.factory/STATE-MANAGER-CHECKLIST.md` |
| Hook template | `plugins/vsdd-factory/templates/verify-sha-currency.sh` | Instantiate into `.factory/hooks/verify-sha-currency.sh` |
| Agent prompt | `plugins/vsdd-factory/agents/state-manager.md` | References the skill, lists anti-patterns |
| Case study | `docs/lessons-learned/wave-gate-bookkeeping.md` (this doc) | Records the trajectory + root-cause chain |

## When does this protocol apply?

Use the `state-burst` skill (and the protocol it enforces) when:

- You're committing a remediation burst to `factory-artifacts`.
- The burst updates `STATE.md` + `SESSION-HANDOFF.md` + `wave-state.yaml`
  together (this is the typical wave-gate convergence shape).
- The burst depends on referencing the burst's own commit SHA in
  multiple documents.

Skip it for:

- The first state-manager burst on a fresh project (no
  `wave-state.yaml` yet).
- One-off STATE.md updates that don't need a SHA cite (session
  checkpoint refresh, tech-debt entry append).

## Future work

A `wave-gate-prerequisite` integration hook (per the v0.52-era
backlog) should wire `verify-sha-currency.sh` into the
adversary-dispatch workflow, blocking the next adversarial pass from
running until the previous burst's hygiene is clean. That hook is
queued; the manual `bash .factory/hooks/verify-sha-currency.sh`
invocation is the current practice.
