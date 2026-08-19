---
document_type: adjudication-memo
level: L3
producer: architect
timestamp: 2026-08-19T00:00:00Z
cycle: v1.0-brownfield-backfill
finding: F-S2111V2-P1-001
severity: BLOCKER
status: mechanism-chosen; option-b; ADR-039-§AMD-003-filed; pending-human-ratification
traces_to:
  - .factory/specs/architecture/decisions/ADR-039-validator-failure-policy-resource-exhaustion-fail-closed.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.03.017.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.03.018.md
  - .factory/stories/S-21.11-validator-exhaustion-fail-closed-calibration-and-enforcement.md
---

# F-S2111V2-P1-001 Mechanism-Adjudication Memo

## Purpose

This memo records the architect's mechanism decision for S-21.11 v2.0 adversarial pass-1's
BLOCKER finding F-S2111V2-P1-001: the fail-closed enforcement model this ADR/BC/story chain
claims for `legacy-bash-adapter.wasm`-hosted validator gates is contradicted by the
dispatcher's actual `PluginResult` outcome-classification code. It documents the chosen design,
the rejected alternatives, the F-005 in-scope ruling, the F-004 correction, and the precise
directive for product-owner (BC-1.03.017, BC-1.03.018) and story-writer (S-21.11) to execute
**after** the human ratifies the new ADR-039 §AMD-003. This burst amends ADR-039 (§Decision 1
correction + new §AMD-003, v1.10 → v1.11) and does **not** edit BC-1.03.017, BC-1.03.018, or the
S-21.11 story body — those edits are product-owner's and story-writer's domain and are
enumerated below as a directive for a follow-up burst after ratification.

## The confirmed gap (restated, verified against live source)

1. `crates/factory-dispatcher/src/host/exec_subprocess.rs::run` / `execute_bounded`: a wall-clock
   overrun in the `Instant`-based poll loop calls `child.kill()` and returns `Err(codes::TIMEOUT)`
   — confirmed at the `Ok(None) => { if Instant::now() >= deadline { ... return Err(codes::TIMEOUT); } }`
   arm.
2. `crates/hook-plugins/legacy-bash-adapter/src/lib.rs::run_bash_via_host`: calls
   `host::exec_subprocess(...)` and maps ANY `Err` from it — including the `TIMEOUT` code above —
   to `Err(format!("{e:?}"))`, a plain string, losing the structured error code.
3. `adapter_logic`'s `run_bash` closure: on `Err(e)`, returns
   `HookResult::error(format!("legacy-bash-adapter: bash subprocess failed ({e}) — verify ..."))`
   — confirmed at the `Err(e) => { return HookResult::error(...) }` arm.
4. `crates/hook-sdk/src/result.rs::HookResult::exit_code`: `Error { .. } => 1`.
5. `crates/factory-dispatcher/src/invoke.rs::classify_trap`: a WASI `_start` exit via
   `std::process::exit(1)` (which is how the `#[hook]` macro's exit-code mapping is realized)
   surfaces as `Err(I32Exit(1))` from `start_export.call`, classified at the
   `if let Some(exit) = err.downcast_ref::<I32Exit>()` arm as
   `PluginResult::Ok { exit_code: exit.0, .. }` — i.e. `Ok { exit_code: 1, .. }`. It is **never**
   `PluginResult::Timeout { .. }`: `PluginResult::Timeout { cause: TimeoutCause::Epoch }` is
   produced ONLY by `Trap::Interrupt` (wasmtime's guest-epoch interruption), which — per
   wasmtime's own documented behavior, cited in ADR-039 §Decision 4's existing v1.9
   "Mechanism precision" paragraph — cannot fire while the guest is blocked inside a synchronous
   host call such as `exec_subprocess`. The guest (the adapter's own Rust code) instead returns
   cleanly with `exit_code = 1` once its `Err` handling completes.
6. `crates/factory-dispatcher/src/executor.rs::execute_tiers`: `plugin_requests_block` matches
   stdout `"outcome":"block"` (only true for well-formed `HookResult::Block` JSON);
   `plugin_fail_closed` matches `PluginResult::Crashed { .. } | PluginResult::Timeout { .. }`
   only. Neither catches `PluginResult::Ok { exit_code: 1, .. }` — so `block_intent` stays
   `false` and the dispatch is ALLOWED.

**Net:** a bash-adapter gate's subprocess timeout — and, per F-005, ANY error path inside
`adapter_logic` that returns `HookResult::Error` — surfaces as `PluginResult::Ok` and bypasses
fail-closed, even for the two `on_error = "block"` PreToolUse `^Agent$` gates
(`validate-wave-gate-prerequisite`, `validate-pr-merge-prerequisites`). "Wire `timeout_ms` into
`exec_subprocess`" (the AMD-002 leg, S-21.11 AC-013) does NOT by itself achieve fail-closed
enforcement — it only ensures the kill fires at the correct wall-clock time; it does nothing
about what happens to the resulting outcome at the `plugin_fail_closed` decision point.

ADR-039 §Decision 1's own v1.8 amendment text additionally contained an internal contradiction:
it named `TimeoutCause::Epoch` as the signal reflecting a bash script's actual running time for
`legacy-bash-adapter.wasm`-hosted plugins, which is incompatible with §Decision 4's own v1.9
"Mechanism precision" correction (already in the document) establishing that the real
enforcement point is `exec_subprocess.rs::run()`'s non-trapping poll+kill loop, not wasmtime
epoch interruption. This burst corrects §Decision 1 to align with §Decision 4 and traces the
consequence through to the `PluginResult::Ok { exit_code: 1 }` fail-open hole.

## Mechanism options evaluated

**(a) Adapter-side remap.** `legacy-bash-adapter::adapter_logic` maps a subprocess timeout
(and/or a generic host/script error) to an explicit `{"outcome":"block", ...}` stdout, or to a
distinct signal the executor treats as blockable, instead of `HookResult::Error`.

- *For:* localizes the fix to the one plugin the finding was raised against; no executor change.
- *Against:* fixes only `legacy-bash-adapter` — any future or existing native-WASM plugin that
  legitimately returns `HookResult::Error` under `on_error = "block"` (e.g. a malformed-config
  error, a deserialization failure caught and reported cleanly rather than panicking) would
  remain unprotected by the identical gap. It also repurposes `Block` — semantically "the
  validation ran and found a violation" — to also mean "the plugin could not produce a verdict,"
  conflating two outcomes `HookResult`'s three-variant design (`Continue`/`Block`/`Error`)
  deliberately keeps distinct. **Rejected as the primary mechanism.**

**(b) Executor-side generalization.** Extend `plugin_fail_closed` (or its replacement) so that,
for `on_error = Block`, ANY `PluginResult::Ok` outcome with `exit_code != 0` is treated as
fail-closed, in addition to the existing `Crashed`/`Timeout` cases.

- *For:* a single, general fix at the aggregation layer (`execute_tiers`'s block-decision loop)
  that applies uniformly to every current and future plugin, regardless of adapter class. It
  matches the plain-English contract `on_error = "block"` already claims
  (`hook-sdk/src/result.rs::HookResult::Error`'s own doc comment: "operators set
  `on_error = "block"` ... if they want plugin failures to be hard stops"). Requires no new
  `PluginResult` variant, so no downstream consumer (`emit_lifecycle`'s JSONL telemetry, the
  `TierExecutionSummary` aggregation, or any existing test matching on `PluginResult`'s shape)
  needs to change beyond the one function ADR-039 already governs. It is a strict superset of
  the current rule: `exit_code == 0` (Continue) still never blocks; `exit_code == 2` (Block) is
  already, independently, caught by `plugin_requests_block`'s stdout check regardless of
  `on_error`; this rule newly catches `exit_code == 1` (Error) and, as a defensive side effect,
  any non-compliant plugin that exits with some other nonzero code without emitting parseable
  `outcome:block` JSON.
- *Against:* is coarser than option (c) — it cannot distinguish "the plugin explicitly declined
  to produce a verdict" from "the plugin's `main()` returned an unanticipated nonzero status."
  Judged acceptable: under `on_error = "block"`, the operator's stated intent is that ANY plugin
  failure is a hard stop, so the coarser rule is exactly what was asked for.
- **CHOSEN.**

**(c) New `PluginResult` variant.** Introduce a distinct outcome (e.g. `PluginResult::PluginError`)
separate from `Ok`, so the crash-vs-clean-failure distinction is explicit at the type level.

- *For:* more precise; would let telemetry and future logic distinguish "plugin ran and
  declined" from "plugin ran and succeeded but exited nonzero for an unrelated reason."
- *Against:* touches the `PluginResult` enum's shape, which is `Serialize`/`Deserialize` and
  consumed by `emit_lifecycle`'s JSONL event construction, multiple existing unit and
  integration tests (`invoke.rs`, `executor.rs`), and `TierExecutionSummary`'s aggregation logic
  — materially larger blast radius for a decision landing inside S-21.11's already-large
  (32-point) unified scope. The marginal type-safety benefit does not, in the architect's
  judgment, justify that blast radius to close this BLOCKER. Remains available as a future
  refactor if a later story finds `exit_code`-sniffing insufficiently precise.
- **Rejected for this ADR's scope.**

## Rationale summary

Option (b) is chosen because it is the smallest-blast-radius fix that (1) closes the specific
BLOCKER (bash-adapter timeout bypassing fail-closed) even after AC-013's AMD-002 wiring fix
lands, (2) closes the same class of gap for every OTHER `on_error = "block"` plugin in the
registry (not just the five §Decision 2 bash-adapter plugins), present or future, and (3)
requires touching exactly one function (`plugin_fail_closed`) and its call site, rather than the
`PluginResult` enum's shape and every consumer of it.

## F-005 scope ruling

**F-005 (generic `HookResult::Error` fail-open path, not only the timeout-specific F-001 leg) is
ruled IN SCOPE for S-21.11.** It is not deferred to a separate story.

Justification against the project's canonical production-grade-default principle (CLAUDE.md
Rule 3: a tech-debt deferral requires ALL of explicit human direction, a concrete future
dependency, and a named future story/wave anchor):

- No human has directed this be deferred.
- There is no future dependency that makes F-005 separable from F-001 — both are fixed by the
  identical one-line predicate change to `plugin_fail_closed`, requiring no additional research,
  no additional ratification beyond this same §AMD-003, and no work outside S-21.11's
  already-declared subsystem (SS-01, `crates/factory-dispatcher/src/executor.rs`).
- Splitting them would require touching `plugin_fail_closed` twice for the identical root cause
  — a worse engineering outcome than fixing the general case once, and precisely the kind of
  "fix now vs. defer for no structural reason" case Rule 3 exists to prevent.

**Directive:** story-writer folds F-005's coverage into S-21.11 AC-013's scope (or an adjacent
new AC in the same story, e.g. AC-013b) rather than filing a follow-up story. See the BC/AC
directive below for the specific test cases required.

## F-004 correction (BC-1.03.018 EC-005 tier premise)

**Finding:** BC-1.03.018 EC-005 states: "Both named gates dispatched in the SAME tier, override
set to `all`, both would have blocked." This premise is false per live
`plugins/vsdd-factory/hooks-registry.toml`:

```
[[hooks]]
name = "validate-pr-merge-prerequisites"
priority = 120
...
[[hooks]]
name = "validate-wave-gate-prerequisite"
priority = 130
```

`crates/factory-dispatcher/src/routing.rs::group_by_priority` sorts matched entries by
`(priority, original_index)` and starts a NEW tier every time the priority value changes:

```rust
indexed.sort_by_key(|(i, p, _)| (*p, *i));
let mut tiers: Vec<Vec<&RegistryEntry>> = Vec::new();
let mut current_priority: Option<u32> = None;
for (_, p, entry) in indexed {
    if Some(p) != current_priority {
        tiers.push(Vec::new());
        current_priority = Some(p);
    }
    tiers.last_mut().unwrap().push(entry);
}
```

Priority 120 and priority 130 are distinct values, so `validate-pr-merge-prerequisites` and
`validate-wave-gate-prerequisite` are placed in two DIFFERENT, sequential tiers — never the same
tier.

**However, EC-005's stated conclusion is still correct, for a different reason than given.**
`execute_tiers` (`crates/factory-dispatcher/src/executor.rs`) loops over every tier
unconditionally:

```rust
for tier in tiers {
    let tier_outcomes = execute_tier(&inputs, tier).await;
    for outcome in &tier_outcomes {
        if plugin_requests_block(&outcome.result) || plugin_fail_closed(...) {
            block_intent = true;
        }
    }
    all_outcomes.extend(tier_outcomes);
}
```

There is no early return or `break` on a block-producing tier — every tier runs, and every
outcome (from every tier) is folded into `block_intent`. So even though the two named gates are
in different tiers, BOTH are always invoked and BOTH outcomes are always evaluated within a
single `execute_tiers` call for one dispatch. EC-005's "two separate `break_glass.activated`
events, per-gate audit granularity preserved" conclusion holds — it just does not depend on, and
is not evidenced by, the two gates sharing a tier.

**Directive:** product-owner corrects BC-1.03.018 EC-005's Description column (both the table row
at the EC-005 entry and the parallel Canonical Test Vectors row) from "Both named gates
dispatched in the SAME tier" to "Both named gates dispatched (in separate `execute_tiers` tiers —
priority 130 vs 120 — evaluated unconditionally within the same dispatch via
`execute_tiers`'s no-early-return tier loop)". No PC, Invariant, or Precondition content changes
— this is a Description/Edge-Case-narrative correction only; PC3, PC5, PC6, and Invariant 6 are
unaffected because none of them assert or depend on tier co-membership.

## Directive for product-owner (BC-1.03.017, BC-1.03.018)

**Do not apply until ADR-039 §AMD-003 is ratified by the human (POLICY 22).** Once ratified:

1. **BC-1.03.017 — new postcondition (or PC1/PC3/PC4/PC10 amendment).** Add a new postcondition,
   e.g. **PC13**, asserting the §AMD-003 rule: "`on_error = OnError::Block` with
   `PluginResult::Ok { exit_code, .. }` where `exit_code != 0` MUST produce a block (exit 2),
   regardless of `failure_policy`." This is a THIRD axis alongside PC1-PC9's `failure_policy`
   (exhaustion) coverage and PC4/PC5/PC10's `on_error`-vs-crash coverage — it is the missing
   `on_error`-vs-clean-nonzero-exit case. Cite `crates/factory-dispatcher/src/executor.rs::plugin_fail_closed`
   and `crates/hook-sdk/src/result.rs::HookResult::exit_code` as Architecture Anchors. Include a
   Canonical Test Vector for `on_error = Block` + `PluginResult::Ok { exit_code: 1 }` →
   `block_intent = true`, and a negative control for `on_error = Continue` +
   `PluginResult::Ok { exit_code: 1 }` → `block_intent` unaffected by this rule (existing
   fail-open-on-crash-when-continue semantics preserved).
2. **BC-1.03.017 — Precondition/Invariant addition.** Add an Invariant stating the §AMD-003 rule
   is a strict superset of the pre-existing `Crashed | Timeout` rule (does not remove any
   existing block path; only adds `Ok { exit_code != 0 }` under `on_error = Block`).
3. **BC-1.03.017 — Traceability row.** Add ADR-039 §AMD-003 citation alongside the existing
   §AMD-001/§AMD-002 citations.
4. **BC-1.03.018 — EC-005 correction.** Apply the F-004 correction above verbatim (Description
   column + Canonical Test Vectors row). No PC/Precondition/Invariant renumbering.
5. **Traceability propagation.** Per POLICY 8/9, sweep any BC-INDEX.md row and Token Budget
   entries affected by BC-1.03.017's new PC.

## Directive for story-writer (S-21.11)

**Do not apply until ADR-039 §AMD-003 is ratified.** Once ratified:

1. **Extend AC-013** (or add AC-013b in the same story) to cover the §AMD-003 rule: a test —
   e.g. `test_on_error_block_fails_closed_on_plugin_error_exit_code` — asserting that a
   synthetic `PluginOutcome` with `on_error = OnError::Block` and
   `result = PluginResult::Ok { exit_code: 1, .. }` produces `block_intent = true` from
   `execute_tiers`'s (or the extended `plugin_fail_closed`'s) decision, AND a companion test for
   the calibrated end-to-end path: a real dispatch driving `legacy-bash-adapter.wasm` against a
   fixture bash script that exceeds the registry's `timeout_ms` (post-AC-013 wiring) MUST
   produce `block_intent = true` / exit code 2 for a gate registered `on_error = "block"` — this
   is the F-001 leg closing end-to-end, not just at the unit level.
2. **F-005 coverage** — add a sibling test asserting a NON-timeout `HookResult::Error` path
   (e.g. missing `script_path`, or a bash exit code other than 0/2) ALSO produces
   `block_intent = true` under `on_error = "block"`, closing the general case, not only the
   timeout-specific one.
3. **Task ordering.** This §AMD-003 leg depends on AC-013 (AMD-002 wiring) for its END-TO-END
   test to be meaningful (a `timeout_ms` overrun must actually be enforced before its downstream
   block decision can be exercised realistically) but does NOT depend on it for the UNIT-level
   `plugin_fail_closed`/`execute_tiers` test (item 1's first assertion), which can land
   independently. Sequence: the `plugin_fail_closed` extension (item 1's first assertion) MAY
   land before or concurrently with AC-013; the end-to-end fixture test (item 1's second
   assertion) MUST land after AC-013.
4. **Update the Behavioral Contracts frontmatter array** to cite BC-1.03.017's new version once
   product-owner lands the PC13 addition, per POLICY 8 propagation (body BC table + ACs + Token
   Budget).
5. **BC-1.03.018 EC-005 citation** — if S-21.11's body quotes or paraphrases BC-1.03.018 EC-005's
   "same tier" language anywhere (e.g. in a task description or an AC's edge-case note), sweep it
   to match the corrected wording once product-owner lands the F-004 fix.

## What this burst did NOT do

- Did not edit `BC-1.03.017.md`, `BC-1.03.018.md`, or the S-21.11 story file — those are
  product-owner's and story-writer's domain, to execute after human ratification per the
  directives above.
- Did not implement the `plugin_fail_closed` code change — architect scope for this burst is
  spec-only; implementation is a Phase 4 implementer task within S-21.11, gated on ratification.
- Did not correct the `hooks-registry.toml` header-comment drift (35 vs 37 documented
  `legacy-bash-adapter` entries) noted by AMD-002 v1.10 — that remains flagged for a future
  maintenance sweep, unrelated to this finding.

## F-008 process-gap note (for state-manager codification, not resolved in this burst)

Adversarial pass-1 additionally surfaced a `[process-gap]`: spec-review did not trace the
`PluginResult::Timeout { cause: TimeoutCause::Epoch }` variant construction site before ADR-039
§Decision 1's v1.8 amendment asserted it as the bash-adapter timeout signal. Any future ADR/BC
prose that names a specific `PluginResult` (or similarly-shaped enum) variant as the outcome of a
described code path should be required to cite the variant's actual construction site
(`classify_trap`, in this case) as part of the same edit, not merely the variant's name. This is
a documentation-discipline gap, not a code defect; routed here for state-manager to codify as a
D-NNN / L-EDP1-NNN entry in a later burst, per the routing table (this is not architect's file to
edit).
