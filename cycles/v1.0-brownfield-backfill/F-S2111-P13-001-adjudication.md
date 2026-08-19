---
document_type: adjudication-memo
level: L3
producer: architect
timestamp: 2026-08-18T00:00:00Z
cycle: v1.0-brownfield-backfill
finding: F-S2111-P13-001
severity: HIGH
status: architect-confirmed; option-a-recommended; pending-human-sign-off
traces_to:
  - .factory/specs/architecture/decisions/ADR-039-validator-failure-policy-resource-exhaustion-fail-closed.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.03.017.md
  - .factory/stories/S-21.11-validator-exhaustion-fail-closed-calibration-and-enforcement.md
---

# F-S2111-P13-001 Split-Boundary Adjudication Memo

## Purpose

This memo records the architect's split-boundary decision for the S-21.11 pre-TDD spec
adversarial cascade, PAUSED by the human after 15 passes on finding F-S2111-P13-001 (HIGH,
architect-CONFIRMED). It is produced during a **scoped architect re-grounding burst**: this
burst amends ADR-039 (§Decision 1/2/3/4, v1.7 → v1.8) and BC-1.03.017 (v1.8 → v1.9) to correct
the fuel-vs-epoch axis mislabeling, and records this adjudication. It explicitly does **not**
rescope the S-21.11 story body and does **not** create the bash-adapter follow-up story — both
are deferred to a later burst after human review of the ADR-039 v1.8 amendment (which, per
POLICY 22, requires human ratification before Phase 3/4 work resumes).

## The confirmed gap (restated)

S-21.11 targets six plugins for the `failure_policy = "fail-closed"` flip under ADR-039's
migration. Five of the six are hosted by the shared `hook-plugins/legacy-bash-adapter.wasm`
shim; only one is a native-WASM plugin. Per ADR-042 §Decision 3 class (b), fuel exhaustion for
a `legacy-bash-adapter.wasm`-hosted plugin (if it occurs at all) happens *before* the WASI
`exec_subprocess` call — the bash script body's execution time is never metered by the WASM
fuel counter. The five bash-adapter plugins' actual resource-exhaustion axis is the
host-enforced wall-clock deadline (`timeout_ms` → `TimeoutCause::Epoch`), a mechanism the
`fuel_cap` calibration procedure prescribed by ADR-039 §Decision 3/4 (as originally drafted)
does not touch. BC-1.03.017's PC8/PC9/PC11 gates, as originally drafted, treated
`fuel_cap ≥ 50_000_000` as the sole calibration proof required before any of the six plugins
could receive `failure_policy = "fail-closed"` — for the five bash-adapter plugins this is
calibration theater: it certifies an axis that was never genuinely at risk while leaving the
axis that IS at risk (`timeout_ms`) completely uncalibrated. Two of the five
(`validate-wave-gate-prerequisite`, `validate-pr-merge-prerequisites`) additionally gate the
`PreToolUse` `^Agent$` event; a false "calibrated" fail-closed flip on either risks a hard,
unconditional block on all future `Agent` tool dispatch — including the dispatches needed to
fix the miscalibration.

## Registry verification (literal-shell confirmed)

```
$ grep -n "^name = \|^plugin = " plugins/vsdd-factory/hooks-registry.toml | grep -A1 "validate-factory-path-root\|validate-input-hash\|validate-template-compliance\|validate-wave-gate-prerequisite\|validate-pr-merge-prerequisites\|validate-cross-site-correspondence"
```

Confirmed entries (adapter type, event, tool, `on_error`, `timeout_ms`):

| Plugin | `plugin =` (WASM binary) | `event` | `tool` | `on_error` | `timeout_ms` |
|--------|---------------------------|---------|--------|-----------|--------------|
| `validate-factory-path-root` | `hook-plugins/legacy-bash-adapter.wasm` | PostToolUse | `^(Edit\|Write\|MultiEdit)$` | block | 5000 |
| `validate-input-hash` | `hook-plugins/legacy-bash-adapter.wasm` | PostToolUse | `^(Edit\|Write\|MultiEdit)$` | block | 10000 |
| `validate-template-compliance` | `hook-plugins/legacy-bash-adapter.wasm` | PostToolUse | `^(Edit\|Write\|MultiEdit)$` | block | 10000 |
| `validate-wave-gate-prerequisite` | `hook-plugins/legacy-bash-adapter.wasm` | **PreToolUse** | **`^Agent$`** | block | 10000 |
| `validate-pr-merge-prerequisites` | `hook-plugins/legacy-bash-adapter.wasm` | **PreToolUse** | **`^Agent$`** | block | 10000 |
| `validate-cross-site-correspondence` | `hook-plugins/validate-cross-site-correspondence.wasm` | PostToolUse | `^(Edit\|Write\|MultiEdit)$` | continue | 8000 |

All five `legacy-bash-adapter.wasm` entries route through `[hooks.config] script_path =
"hooks/<name>.sh"` and `[hooks.capabilities.exec_subprocess] binary_allow = ["bash", ...]`,
confirming they shell out via the adapter's WASI `exec_subprocess` call — the mechanism ADR-042
§Decision 3 class (b) documents as invisible to WASM fuel metering.
`validate-cross-site-correspondence` carries no `[hooks.config] script_path` and no
`exec_subprocess` capability — it is a self-contained native-WASM validator.

## Adjudication: Option A (split) — RECOMMENDED

**IN-SCOPE for S-21.11 (unchanged targeted plugin):**

- `validate-cross-site-correspondence` — native-WASM, `hook-plugins/validate-cross-site-correspondence.wasm`, PostToolUse, `on_error = "continue"`. Its `fuel_cap` genuinely bounds its execution end-to-end; the existing ADR-039 §Decision 3/4 fuel-cap calibration procedure (as amended in this burst only to bifurcate the *scope*, not the fuel formula itself) applies to it without modification.

**MOVE OUT of S-21.11 to a new follow-up story (5 plugins, all `legacy-bash-adapter.wasm`):**

| Plugin | WASM path | Adapter type |
|--------|-----------|--------------|
| `validate-factory-path-root` | `hook-plugins/legacy-bash-adapter.wasm` | legacy-bash-adapter shim (confirmed: `[hooks.config] script_path = "hooks/validate-factory-path-root.sh"`) |
| `validate-input-hash` | `hook-plugins/legacy-bash-adapter.wasm` | legacy-bash-adapter shim (confirmed: `script_path = "hooks/validate-input-hash.sh"`) |
| `validate-template-compliance` | `hook-plugins/legacy-bash-adapter.wasm` | legacy-bash-adapter shim (confirmed: `script_path = "hooks/validate-template-compliance.sh"`) |
| `validate-wave-gate-prerequisite` | `hook-plugins/legacy-bash-adapter.wasm` | legacy-bash-adapter shim (confirmed: `script_path = "hooks/validate-wave-gate-prerequisite.sh"`); **PreToolUse `^Agent$` gate** |
| `validate-pr-merge-prerequisites` | `hook-plugins/legacy-bash-adapter.wasm` | legacy-bash-adapter shim (confirmed: `script_path = "hooks/validate-pr-merge-prerequisites.sh"`); **PreToolUse `^Agent$` gate** |

### Why Option A, not a global pause or a global proceed

- **Rejected: proceed with all six under the original fuel-only gate.** This is the live
  hazard — it would let the two PreToolUse `^Agent$` gates receive `failure_policy =
  "fail-closed"` on a predicate (`fuel_cap ≥ 50M`) that provides zero protection against their
  actual exhaustion axis, risking a hard self-lock on all future `Agent` dispatch.
- **Rejected: pause the entire migration (including the native-WASM plugin).**
  `validate-cross-site-correspondence`'s fuel-axis calibration is genuinely sound under the
  original ADR-039 model — there is no reason to hold it hostage to the bash-adapter
  plugins' distinct problem. Splitting lets Phase 3/4 work proceed for the one plugin that is
  ready while the other five get the correct calibration procedure.
- **Accepted: Option A split.** `validate-cross-site-correspondence` stays in S-21.11 scope
  under the (already-correct-for-it) fuel-axis procedure. The five bash-adapter plugins move to
  a named follow-up story scoped to epoch/`timeout_ms` calibration, per the new ADR-039
  §Decision 4 formula (`timeout_ms ≥ max(measured_p99_ms × 2.0, 30_000)`) established in this
  burst's ADR-039 v1.8 amendment.

## Acceptance criterion for the follow-up story (NOT authored in this burst)

The follow-up story (story-writer authors it after human sign-off; ID not yet allocated) must
cover, at minimum:

1. **Scope:** exactly the five `legacy-bash-adapter.wasm`-hosted plugins listed above.
   `validate-cross-site-correspondence` MUST NOT be included (it stays in S-21.11 scope, or in
   S-21.13 per BC-1.03.017 EC-004 Case A if S-21.11's fuel-axis flip is itself deferred — that
   determination is unaffected by this adjudication).
2. **Calibration procedure:** for each of the five plugins, measure bash subprocess wall-clock
   duration (`time_consumed_ms`) over the ADR-039 §Decision 4 production-scale corpus
   (`lessons.md` ≥4000 lines, `STATE.md` at current live size, `decision-log.md` at current
   live size, the ≥574 KB `a1-production-scale` fixture) and set
   `timeout_ms ≥ max(measured_p99_ms × 2.0, 30_000)` per the ADR-039 v1.8 epoch-axis formula.
3. **Fuel-axis calibration is NOT waived** — these five plugins' adapter-marshaling fuel
   consumption is still real (ADR-042 §Decision 1 measured model:
   `fuel ≈ 29,452 + 27.514 × payload_bytes`) and must still satisfy the existing
   `fuel_cap ≥ 50_000_000` floor. The follow-up story's calibration work is IN ADDITION TO,
   not instead of, fuel-axis calibration.
4. **Self-lock regression test for the two PreToolUse `^Agent$` gates:** a behavioral test
   (not merely a configuration assertion, per ADR-039 §Decision 6 / Envoy #38801 discipline)
   MUST drive an actual `Agent`-tool dispatch through `validate-wave-gate-prerequisite` and
   `validate-pr-merge-prerequisites` with a deliberately-undersized `timeout_ms` and assert the
   dispatcher does NOT silently allow the write while ALSO asserting that a subsequent,
   correctly-calibrated dispatch is NOT blocked — i.e., prove the calibration floor is both
   necessary (undersized → the epoch/timeout hazard is observable) and sufficient (properly
   calibrated → no unconditional self-lock).
5. **PC8/PC9 gate completion:** the story must close BC-1.03.017's now-bifurcated PC8
   TIMEOUT-POSITIVE-CONTROL / TIMEOUT-NEGATIVE-CONTROL assertions and PC9's per-adapter-class
   final-state criterion (both `fuel_cap ≥ 50M` AND `timeout_ms ≥ 30_000` for these five
   plugins) — both added to BC-1.03.017 in this burst (v1.9).
6. **Dependency:** `depends_on` S-21.10 (schema prerequisite, already merged) and, if S-21.11
   ships first with only the native-WASM plugin flipped, `depends_on S-21.11` as well (parallel
   structure to how S-21.13 already depends on both per BC-1.03.017 EC-004).
7. **PC11 migration-window interaction:** because these five plugins currently carry
   `on_error = "block"`, BC-1.03.017's Invariant 7 / PC11 prohibition against a half-state
   (3-arg enforcement-active executor + any `on_error = "block"` targeted plugin left at
   `failure_policy = fail-open`) still applies to them. The follow-up story cannot simply defer
   these five indefinitely once the 3-arg executor ships in S-21.11 — it must either land before
   or atomically with S-21.11's Phase 4 executor flip, or S-21.11's Phase 4 flip must itself be
   held back until the follow-up story's annotations are ready. This ordering question is a
   genuine scope/sequencing decision for story-writer and the orchestrator at story-creation
   time — not resolved by this memo.

## Residual edits required after human sign-off (resume-burst checklist)

The following are explicitly **NOT done** in this burst and must be executed in the resume
burst, after the human reviews and ratifies (or amends) the ADR-039 v1.8 §AMD-001 delta:

- [ ] **Human ratification of ADR-039 v1.8 §AMD-001** (POLICY 22 channel, orchestrator-mediated)
      — required before any Phase 3/4 calibration work proceeds for the five bash-adapter
      plugins under the corrected model. (`validate-cross-site-correspondence`'s fuel-axis work
      is unaffected by this ratification gate — its calibration procedure did not change.)
- [ ] **S-21.11 story body rescope** (story-writer; NOT done in this burst per explicit scoping
      instruction): update the Narrative, AC-007, and Task list to reflect the Option A split —
      remove the five bash-adapter plugins from S-21.11's calibration-and-flip scope; retain
      only `validate-cross-site-correspondence`. Remove the `⚠ BLOCKED (F-S2111-P13-001)`
      blockquote once the rescope lands (it currently sits after the Execute line in the story
      file, added at v1.12 during the pause).
- [ ] **New follow-up story creation** (story-writer; NOT done in this burst): author the story
      covering the five bash-adapter plugins per the acceptance-criterion sketch above. Allocate
      a new story ID (next available in the E-21 series; NOT S-21.13, which is scoped
      exclusively to `validate-cross-site-correspondence`'s O(n) fuel-ceiling fix per
      BC-1.03.017 EC-004 — routing an `on_error = "block"` plugin there would be a mis-route
      per BC-1.03.017's existing EC-004 Case B language).
- [ ] **STORY-INDEX.md update** (state-manager): add the new story's row; update S-21.11's row
      to reflect the narrowed scope (targeted-plugin count, AC count if AC-007 is restructured).
- [ ] **STATE.md update** (state-manager): close out `[F-S2111-P13-001]` in Blocking Issues;
      record the resume decision (D-NNN); un-pause the pipeline (PAUSED → ACTIVE) once the
      resume burst's artifacts are committed; update the Session Resume Checkpoint.
- [ ] **VP-INDEX.md / verification-architecture.md / verification-coverage-matrix.md** — if the
      eventual VP-TBD in BC-1.03.017 is allocated a real VP-NNN ID before or during the resume
      burst, ensure the new epoch-axis PC8/PC9 assertions are reflected in the VP's proof
      harness skeleton (state-manager / architect, per VP-INDEX propagation obligation).
- [ ] **BC-1.03.017 PC10/PC11/EC-004 narrative refinement** (product-owner, if further
      refinement beyond this burst's architectural precondition/PC correction is judged
      necessary) — this burst deliberately did not touch PC10/PC11/EC-004's axes-independence
      or migration-window substance; only Preconditions 2/3, PC8, PC9, and the new Invariant 8
      were amended. If the resume burst's rescope surfaces further BC-body narrative gaps
      (e.g., AC-to-PC mapping in the story), route to product-owner.
- [ ] **hooks-registry.toml stale comment cleanup** (out of scope for this burst; noted for
      completeness): the `validate-cross-site-correspondence` entry's comment block states "the
      per-plugin `fuel_cap` registry field ... is not yet implemented" — this is stale;
      `crates/factory-dispatcher/src/registry.rs` confirms `fuel_cap: Option<u64>` and
      `timeout_ms: Option<u32>` are both already live per-plugin override fields (used by all 76
      `[[hooks]]` entries today via `RegistryEntry::fuel_cap()`/`timeout_ms()` accessor methods
      with `RegistryDefaults` fallback). Neither this burst's amendment nor the follow-up story
      requires a schema change — both fields already exist. The stale comment should be swept
      by whichever future burst next touches that registry block (devops-engineer or
      implementer), not treated as blocking.

## Sign-off gate

This memo, together with ADR-039 v1.8 and BC-1.03.017 v1.9, is the complete architect
re-grounding deliverable for this burst. Per the task's explicit scoping, the pipeline PAUSES
AGAIN here for human review of:

1. The ADR-039 v1.8 §AMD-001 amendment (does it require ratification changes, or does the human
   accept the fuel-vs-epoch bifurcation and the `timeout_ms ≥ max(p99_ms × 2.0, 30_000)`
   formula as proposed?).
2. This memo's Option A split recommendation (in-scope-1 / move-out-5).
3. The residual-edits checklist above, which becomes the resume burst's work order once
   sign-off is given.
