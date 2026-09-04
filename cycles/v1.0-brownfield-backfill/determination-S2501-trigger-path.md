# Determination: PR #807 (S-25.01) MAJOR Finding — Layer-1 marker→gate trigger path

**Task:** Determine whether the fresh-eyes pr-reviewer's MAJOR finding on PR #807 ("all three
fail-closed Cohort A validators run at PreToolUse, but the marker WRITE path is
PostToolUse-only, so Layer-1's marker→gate machinery has no active production trigger path")
is (A) intended foundational infrastructure whose PR description merely overstates activation,
or (B) a real gap.

**Determination: (B) — REAL GAP, but a spec/narrative-overclaim gap, not a code gap. It is
also DEEPER than the pr-reviewer's finding: the pr-reviewer treated `validate-factory-path-staging`
("Cohort A-immediate / EFFECTIVE-NOW") as the one validator that IS effectively closed. It is
not. Ground-truth analysis shows ALL THREE Cohort A validators — including the "EFFECTIVE-NOW"
one — have zero live production enforcement (neither current-dispatch block nor next-advance
marker gate) today, and `validate-factory-path-staging` has no currently-scoped future story
that will ever give it one either.**

Does NOT block CI-green merge of PR #807 — the CODE is correct and matches its own literal ACs
(AC-005, BC-1.18.001 INV4) and the story's own EC-009 row (which is already accurate). The
defect lives in already-ratified spec/architecture narrative (ADR-047, BC-1.18.004, and
S-25.01's own headline framing), not in this PR's diff.

---

## 1. Registry ground truth — no PostToolUse `failure_policy`-bearing validator exists

Confirmed via `awk` scan of `.worktrees/S-25.01/plugins/vsdd-factory/hooks-registry.toml`
(name/event/tool/failure_policy extraction across every `[[hooks]]` block):

| Validator | event | tool | failure_policy | on_error |
|---|---|---|---|---|
| `validate-pr-merge-prerequisites` | PreToolUse | `^Agent$` | fail-closed | block |
| `validate-wave-gate-prerequisite` | PreToolUse | `^Agent$` | fail-closed | block |
| `validate-factory-path-staging` | PreToolUse | `^Bash$` | fail-closed | continue |
| `validate-unvalidated-mutation-marker` | PreToolUse | `^Agent$` | fail-open | block_if_marker |
| `validate-unvalidated-mutation-marker-git` | PreToolUse | `^Bash$` | fail-open | block_if_marker |

Zero `failure_policy`-bearing entries anywhere in the registry are `PostToolUse`. There is
literally no configured plugin that can ever produce a PostToolUse fail-closed INDETERMINATE.

## 2. Code ground truth — marker write requires `event == "PostToolUse"`, unconditionally

`crates/factory-dispatcher/src/executor.rs` (S-25.01 worktree), both call sites:

- `execute_tier` (~line 592): `if should_write_marker(&outcome, failure_policy) && entry_clone.event == "PostToolUse"`
- `spawn_async_plugin` (~line 902): identical guard

`classify_outcome` (line 132) maps `PluginResult::Timeout{cause: Fuel|Epoch}` → `Indeterminate`
regardless of hook event; the PostToolUse gate is applied only at the marker-write call sites,
exactly as the pr-reviewer described. `should_write_marker`/`write_indeterminate_marker`/
`emit_write_tied_audit_events` are therefore reachable ONLY from a PostToolUse dispatch — dead
code for every currently-registered validator. This is spec-correct per BC-1.18.001 invariant 4
("marker write is PostToolUse only") — the code is not the defect.

Separately, current-dispatch blocking (independent of the marker) is governed exclusively by
`plugin_fail_closed` (Crashed/Timeout + `on_error == Block`) and `plugin_block_if_marker`
(Crashed/Timeout + `on_error == BlockIfMarker` + marker present) — both keyed on `on_error`, not
on `failure_policy`/INDETERMINATE classification directly (`executor.rs` lines ~1210–1245,
`execute_tiers` ~283–338). `DispatchOutcome::Indeterminate` by itself never sets `block_intent`.

## 3. Does S-25.01's own narrative/ACs/BCs claim active, live production enforcement?

**Yes — and this is the real defect.** The story's headline narrative (S-25.01 lines 193–198),
AC-016 (lines 943–968), BC-1.18.004 Postcondition 4, and ADR-047 (lines 374–399, 660–685) all
assert:

> "the silent CWE-754 hole … is permanently closed for `validate-factory-path-staging`
> (Cohort A-immediate; EFFECTIVE-NOW at S-25.01 merge) … Layer-1 effective fail-closed count at
> S-25.01 merge: 1."

BC-1.18.004 PC4 additionally makes a **factually false** claim about this validator's hook type:

> "`validate-factory-path-staging` — EFFECTIVE-NOW (Cohort A-IMMEDIATE): … **Governs `.factory/`
> write-path PostToolUse hooks on Edit/Write/MultiEdit/Bash artifacts** targeting `.factory/`
> paths."

The registry (§1) shows exactly one entry for this plugin: `event = "PreToolUse"`,
`tool = "^Bash$"` — no Edit/Write/MultiEdit matcher, not PostToolUse. ADR-047's own changelog
(v1.3, line 18) records a **partial** correction: "Factual fix: validate-factory-path-staging
tool pattern corrected from `^(Edit|Write|MultiEdit)$` to `^Bash$` to match
hooks-registry.toml" — i.e. the architect already caught and fixed the tool-pattern half of this
error in ADR-047 itself, but the companion `PostToolUse`→`PreToolUse` correction was never made,
and the fix was never propagated to BC-1.18.004 (still §PC4, unchanged since v1.1, still reads
"PostToolUse … Edit/Write/MultiEdit/Bash"). This is a stale, unpropagated correction — the same
class of defect POLICY 9/TD-VSDD-060 exist to catch, just in a BC↔ADR pair rather than a VP pair.

**The story's own Edge Cases table already states the true, correct behavior** (EC-009, line
1063):

> "`validate-factory-path-staging` (PreToolUse `^Bash$`, Cohort A-immediate, fail-closed)
> fuel-exhausts | PreToolUse INDETERMINATE path: no marker written (PostToolUse-only per
> BC-1.18.001 invariant 4); advisory `plugin.indeterminate` event only. **ADR-039 Phase 4
> enforcement (current-dispatch blocking) activates at S-21.24**; `on_error = "continue"`
> prevents self-lock."

So the story text is internally self-contradictory: the headline narrative/AC-016/BC-1.18.004
say "EFFECTIVE-NOW / permanently closed / count = 1"; the story's own EC-009 row says the
opposite — no marker, advisory-only, and (see §4) even the "S-21.24 activation" escape hatch
does not apply to this specific validator.

## 4. ADR-047's own reasoning proves "EFFECTIVE-NOW" delivers zero enforcement, ever, for this validator

ADR-047 §"Why the partition?" (lines 390–396) states, in the architect's own words:

> "`validate-factory-path-staging` has `on_error = "continue"` — **even when S-21.24 wires the
> ADR-039 Phase 4 executor enforcement, this validator cannot self-lock the session (a
> fuel-exhausted `on_error=continue` validator never blocks the current dispatch).**"

This is the architect explicitly documenting that current-dispatch blocking will NEVER apply to
`validate-factory-path-staging`, present or future (by deliberate design — `on_error=continue`
is the self-lock-avoidance choice for this plugin). Combined with §2/§3 (marker write is
PostToolUse-only, and this validator is PreToolUse, so it will never write a marker either),
**there is no mechanism, current or currently-planned, by which `validate-factory-path-staging`'s
own INDETERMINATE outcomes ever produce any observable effect beyond the advisory
`plugin.indeterminate` log event** — which is exactly BC-1.18.004's own definition of the
*fail-open* path (PC1: "Advisory `plugin.indeterminate` event emitted only … purely
observational"). Setting `failure_policy = "fail-closed"` on this validator today changes zero
runtime behavior versus leaving it fail-open; it is a config bit primed for a trigger surface
that does not exist and is not scoped to be created.

Cross-checked against ADR-039 §Decision 2's own list of the "six §Decision 2 named plugins"
targeted by the S-21.19→S-21.24 exhaustion-leg (`plugin_fail_closed_on_exhaustion`)
current-dispatch-blocking flip: `validate-cross-site-correspondence`,
`validate-factory-path-root`, `validate-input-hash`, `validate-template-compliance`,
`validate-wave-gate-prerequisite`, `validate-pr-merge-prerequisites`. **`validate-factory-path-
staging` is NOT on this list** (note: `validate-factory-path-root` is a different plugin).
So the two "Cohort A-deferred" validators DO have a real, concrete, already-specced completion
path (S-21.19..S-21.24, all `status: draft` today) that will deliver genuine current-dispatch
blocking once merged — the "SET-BUT-LATENT … activates at S-21.24" framing for those two is
accurate and is a legitimate CLAUDE.md-Rule-3-class deferral (explicit future story anchor).
`validate-factory-path-staging` has no equivalent anchor anywhere in the codebase or specs.

## 5. Is the epic explicitly layered, and do S-25.02/S-25.03 address this?

Yes, E-25 is explicitly a 3-layer holding epic (`.factory/stories/epics/E-25-validation-integrity.md`).
S-25.02 ("Artifact Sharding Layer 2") and S-25.03 ("Bounded Validator Windows Layer 3") are
about eliminating the ROOT CAUSE of fuel/epoch exhaustion (large PostToolUse-artifact scans) via
shard-cap bounding and bounded-window reads — an orthogonal problem. Neither story's scope
(per E-25's own "Three-Layer Architecture" section) touches the PreToolUse/PostToolUse trigger
gap for `validate-factory-path-staging`, or proposes converting it to PostToolUse, or proposes a
new PostToolUse companion validator. **No currently-scoped story closes this specific gap.**

## Summary of what is actually true at S-25.01/PR #807 merge

| Validator | Marker write live? | Current-dispatch block live? | Future path exists? |
|---|---|---|---|
| `validate-pr-merge-prerequisites` | No (PreToolUse) | No (gated on S-21.24) | **Yes** — S-21.19..S-21.24 chain (draft, scoped) |
| `validate-wave-gate-prerequisite` | No (PreToolUse) | No (gated on S-21.24) | **Yes** — same chain |
| `validate-factory-path-staging` | No (PreToolUse, permanently — it is a PreToolUse-native gate) | No, and **never will be** per ADR-047's own "on_error=continue…never blocks" admission | **No** — not in ADR-039 §D2's six-validator list; not addressed by S-25.02/S-25.03; no story exists |

"Layer-1 effective fail-closed count at S-25.01 merge: ONE" (ADR-047, BC-1.18.004, S-25.01
narrative/AC-016, all repeating the same figure) should read **ZERO** — none of the three Cohort
A validators produce any enforcement effect (marker-write, next-advance gate, or current-dispatch
block) different from the fail-open baseline as of this PR.

## Routing

1. **Does NOT block PR #807's CI-green merge.** The diff's code is spec-correct
   (AC-005/BC-1.18.001 INV4) and matches the story's own accurate EC-009 row. No implementer
   fix is warranted or possible here — there is no code bug.
2. **pr-manager**: soften/correct the PR #807 description's "effective-now / live-enforced"
   language before merge (the pr-reviewer's original, narrower ask) — replace with: "Layer 1
   delivers the INDETERMINATE classification, event emission, durable-marker mechanism, and
   next-advance gate, fully unit/integration-tested. Zero of the three Cohort A
   `failure_policy=fail-closed` assignments produce a different runtime effect than fail-open
   today: the two `^Agent$` validators await S-21.24's current-dispatch-block wiring;
   `validate-factory-path-staging`'s PreToolUse nature means the PostToolUse-only marker
   mechanism structurally cannot trigger for it and no story currently closes that gap."
3. **architect** (owner of ADR-047): correct the "EFFECTIVE-NOW / Layer-1 effective count: 1"
   claim (lines 374–399, 660–685) to reflect §4 above, and finish the v1.3 "factual fix" that
   was only half-applied — `validate-factory-path-staging` is `PreToolUse`, not `PostToolUse`,
   everywhere ADR-047 says otherwise. This is an in-scope fix per CLAUDE.md Canonical Principle
   Rule 4 (AI-built defects are the AI's responsibility to fix), not a merge blocker for #807.
4. **product-owner** (owner of BC-1.18.004): sibling-sweep the same PostToolUse→PreToolUse
   correction into BC-1.18.004 PC4 (currently still reads "Governs `.factory/` write-path
   PostToolUse hooks on Edit/Write/MultiEdit/Bash artifacts" — both the event type and the tool
   pattern are wrong; ADR-047 already corrected the tool pattern but this never propagated here,
   a POLICY-9-class propagation gap). Re-word "EFFECTIVE-NOW" partition language to match §4.
5. **story-writer** (owner of S-25.01): reconcile the headline narrative (lines 193–198) and
   AC-016 (lines 943–968) with the story's own already-correct EC-009 row; drop or redefine
   "Layer-1 effective fail-closed count: 1."
6. **Orchestrator/human decision needed** (new work, not yet scoped anywhere): whether to open a
   follow-up story to actually close the gap for `validate-factory-path-staging` (e.g. add a
   PostToolUse companion check, or extend the ADR-039 exhaustion-leg current-dispatch mechanism
   to cover it despite `on_error=continue`), or to explicitly accept it as a permanent
   architectural limitation of the Bash-PreToolUse `git add` staging guard and document that
   acceptance. Per CLAUDE.md Rule 3, this cannot be silently parked in the tech-debt register
   without a concrete future story anchor — none currently exists, so this needs an explicit
   human/orchestrator decision, not a default deferral.
