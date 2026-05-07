---

# Adversarial Review — Pass 1

```yaml
---
document_type: adversarial-review
cycle: v1.0-feature-engine-discipline-pass-1
pass: 1
phase: F5
date: 2026-05-07
producer: adversary
---
```

## ⚠️ Tooling constraint surfaced

My system-prompt tool profile is `read-only` (Read/Grep/Glob only; Write/Edit/Bash denied). The user instructed me to write this review to disk at `/Users/jmagady/Dev/vsdd-factory/.factory/cycles/v1.0-feature-engine-discipline-pass-1/adv-cycle-pass-1.md`, but I cannot. Per my system prompt: "Findings are returned as chat text — the orchestrator persists them via state-manager." I am therefore returning the complete review as chat output and surfacing this constraint as a hard error per the user's instruction. The orchestrator must persist this output.

This dispatch surfaces a real `[process-gap]`: the F5 adversary dispatch flow assumes the adversary can write its own document, which contradicts the read-only profile encoded in `agents/adversary.md` (line 320-323) and the broader information-asymmetry justification (line 326). See finding F-PG-001 below.

---

## Critical Findings

### FINDING [CRITICAL] — Lobster workflow references non-existent skill file `step-d5-adversary-convergence.md`

WHY: `phase-3-tdd-implementation.lobster` line 79 declares the new `adversary-convergence` step uses `skill: "skills/deliver-story/steps/step-d5-adversary-convergence.md"`, but this skill file does not exist on disk.

EVIDENCE:
- `/Users/jmagady/Dev/vsdd-factory/.worktrees/F5-review/plugins/vsdd-factory/workflows/phases/phase-3-tdd-implementation.lobster:77-80`
- Glob of `/Users/jmagady/Dev/vsdd-factory/.worktrees/F5-review/plugins/vsdd-factory/skills/deliver-story/steps/*.md` returns step-a through step-g and `_shared-context.md` only — NO step-d5.

IMPACT: The first time the Lobster workflow is dispatched on a real story after this cycle's stories themselves (which were exempt per D-354), the orchestrator will fail when the dispatcher attempts to load the missing skill. This silently breaks the very gate this cycle was created to enforce. S-12.01 AC-005 was satisfied at the YAML-syntax level (the step exists with correct `depends_on`) but the underlying dispatch target was never authored.

FIX: SPEC + CODE — Author `plugins/vsdd-factory/skills/deliver-story/steps/step-d5-adversary-convergence.md` with the loop procedure from `per-story-delivery.md` Step 4.5 (the loop procedure at lines 161-169 is the source). Either that, or change the Lobster step `type` to inline the procedure. The former preserves the workflow-step-as-skill pattern used by step-a..step-g. POLICY: implicit — workflow integrity.

CONFIDENCE: HIGH

---

### FINDING [CRITICAL] — Three stories anchor to a non-existent ADR file path slug

WHY: S-12.01, S-12.02, and E-12 all declare inputs/anchors as `.factory/specs/architecture/decisions/ADR-017-per-story-adversary-three-perimeter-model.md`, but the actual file is `ADR-017-per-story-adversary-phasing.md`.

EVIDENCE:
- Glob shows the only ADR-017 file: `/Users/jmagady/Dev/vsdd-factory/.factory/specs/architecture/decisions/ADR-017-per-story-adversary-phasing.md`
- `/Users/jmagady/Dev/vsdd-factory/.factory/stories/S-12.01-per-story-adversary-workflow.md:16` cites `ADR-017-per-story-adversary-three-perimeter-model.md`
- `/Users/jmagady/Dev/vsdd-factory/.factory/stories/S-12.02-per-story-adversary-convergence-hook.md:17` cites the same wrong slug
- `/Users/jmagady/Dev/vsdd-factory/.factory/stories/epics/E-12-engine-governance.md:17,21` `traces_to:` and `inputs:` both cite the wrong slug
- `/Users/jmagady/Dev/vsdd-factory/.factory/logs/events-2026-05-06.jsonl:1260` confirms the ADR was authored at the *phasing* path
- `/Users/jmagady/Dev/vsdd-factory/.factory/cycles/v1.0-feature-engine-discipline-pass-1/F1-delta-analysis.md:323` calls the ADR title "Per-Story Adversary Convergence Gate: Workflow Precondition vs. WASM Hook"

IMPACT: Any tool that resolves `inputs:` paths (input-hash drift check, traceability validators, downstream input-hash recomputation in F4 implementer) will fail with file-not-found. Three story specs and one epic are "anchored" to a path that does not resolve. Per `policies.yaml` POLICY 4 (`semantic_anchoring_integrity`, MEDIUM) and adversary.md "Mis-anchoring is NEVER an Observation. It ALWAYS blocks convergence" — this is at minimum HIGH severity, escalated to CRITICAL because it occurs in three sibling artifacts (Partial-Fix Regression Discipline blast radius ≥ 2 → HIGH, plus all three artifacts converged with the same broken anchor → CRITICAL). POLICY: 4 (semantic_anchoring_integrity)

FIX: SPEC — Either rename `ADR-017-per-story-adversary-phasing.md` to `ADR-017-per-story-adversary-three-perimeter-model.md` (and run cross-reference fixup) OR update all four anchor strings (E-12 traces_to + inputs, S-12.01 inputs, S-12.02 inputs) to the actual on-disk slug. Recommend the rename so that the slug matches the ADR's prominent "Three-Perimeter Model" framing in the body.

CONFIDENCE: HIGH

---

### FINDING [CRITICAL] — VP-071 spec describes a `BlockWithFix { hook, reason, recommendation, code }` enum variant that does not exist in the SDK

WHY: VP-071 (v1.1) in its "Proof Harness Skeleton" section declares "`HookResult` enum variants used: `Continue` | `BlockWithFix { hook: String, reason: String, recommendation: String, code: String }` (canonical form per HOST_ABI.md §WASM hooks, line 423)". The SDK at `crates/hook-sdk/src/result.rs` defines only three variants — `Continue`, `Block { reason: String }`, `Error { message: String }`. `block_with_fix(...)` is a constructor that returns `HookResult::Block { reason: <combined string> }`. There is no `BlockWithFix` variant.

EVIDENCE:
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/verification-properties/VP-071.md:222` (declared variant)
- `/Users/jmagady/Dev/vsdd-factory/.worktrees/F5-review/crates/hook-sdk/src/result.rs:18-32` (actual enum: only `Continue`, `Block`, `Error`)
- `/Users/jmagady/Dev/vsdd-factory/.worktrees/F5-review/crates/hook-sdk/src/result.rs:50-61` (`block_with_fix` constructor returns `Block { reason: format!(...) }`)
- VP-071 lines 91, 112, 130, 147 also assert `matches!(result, HookResult::BlockWithFix { .. })` in the kani harness skeleton

IMPACT: The spec contradicts the SDK. The implementation's actual kani harness at `lib.rs:480, 501, 519, 536, 547` uses `matches!(result, HookResult::Block { .. })` — i.e., the implementer corrected the spec drift silently in code. This means: (a) future maintainers reading VP-071 will be misled about the SDK shape; (b) the OQ-9 amendment at VP-071 lines 258-282 claims `block_with_fix` is "canonical Why/Fix/Code form" but BC-4.10.001 PC8 mandates separate `hook`, `reason`, `recommendation`, `code` fields — the SDK collapses these into a single `reason` string with a textual schema. Any downstream consumer that tries to deserialize the JSON looking for separate `hook`/`recommendation`/`code` fields will fail; only `outcome: "block"` and `reason: "..."` are emitted (per `result.rs:18-24` `#[serde(tag = "outcome", rename_all = "snake_case")]`).

FIX: SPEC — Amend VP-071 v1.1 → v1.2 to acknowledge that `HookResult::Block { reason }` is the actual on-the-wire variant, and that `block_with_fix(...)` is a constructor that produces `Block { reason: "BLOCKED by <hook>: <reason>. Fix: <fix>. Code: <code>." }`. Update the kani harness in VP-071 to match the implementation. Update BC-4.10.001 PC8 wording to clarify that the four fields are formatted into the single `reason` string (the implementation's kani Part E at lib.rs:547-554 already verifies this). POLICY: 4 (semantic_anchoring_integrity), 7 (bc_h1_is_title_source_of_truth — affects BC body)

CONFIDENCE: HIGH

---

### FINDING [CRITICAL] — Implementation does not call `host::emit_event` for `hook.block` events as BC-4.10.001 mandates

WHY: BC-4.10.001 PC2/3/4 mandate `HookResult::block_with_fix(...)` AND the broader cycle docs (F1-delta-analysis.md §3 ABI conformance line 92, hooks-registry.toml comment at line 931 "block signal via stdout JSON ... + hook.block event") describe `hook.block` event emission. The convergence hook implementation at `validate-per-story-adversary-convergence/src/lib.rs` and `src/main.rs` never calls `host::emit_event` and the `HookCallbacks` trait does not even declare an `emit_event` method.

EVIDENCE:
- Grep for `emit_event` in `/Users/jmagady/Dev/vsdd-factory/.worktrees/F5-review/crates/hook-plugins/validate-per-story-adversary-convergence/`: only two doc-comment hits (lines 17 and main.rs:16) — zero call sites
- `validate-per-story-adversary-convergence/src/lib.rs:239-258` (`HookCallbacks` trait) declares only `read_file`, `list_stories`, `log_debug`, `log_error` — no `emit_event`
- `validate-per-story-adversary-convergence/src/main.rs:39-81` (`RealCallbacks` impl) does not wire `host::emit_event`
- Compare with sibling `validate-artifact-path/src/lib.rs:411-421` which DOES call `(callbacks.emit_event)("hook.warn", ...)` for the warn path

IMPACT: The "advisory-block-mode" comment in hooks-registry.toml line 931 implies the hook conveys block via stdout + `hook.block` event. The hook conveys block ONLY via `HookResult::Block { reason }` (which the dispatcher serializes to exit code 2 + stderr per result.rs:75). Wave-gate observability tooling that expects a `hook.block` event will never see one for convergence violations. Telemetry analytics (the bc_id=4.10.001 monitoring) will show zero hook.block events even when the gate is firing.

FIX: CODE + SPEC — Add an `emit_event` method to the `HookCallbacks` trait, wire it in `main.rs`, and emit `("hook.block", &[("hook", HOOK_NAME), ("code", &code), ("story", story_id), ...])` immediately before each block return in `hook_logic`. Alternatively, amend BC-4.10.001 to drop the `hook.block` event requirement and only require `HookResult::Block`. Recommend the former — observability is a real value-add and matches the artifact-path hook's pattern. POLICY: implicit — spec/code consistency

CONFIDENCE: HIGH

---

## Important Findings

### FINDING [HIGH] — hooks-registry.toml comment claims "advisory-block-mode" but the implementation uses canonical block_with_fix (returns HookResult::Block, exit code 2)

WHY: `hooks-registry.toml:931-933` reads "Advisory-block-mode: hook returns Continue in all cases; block signal via stdout JSON ..." But the actual implementation at `lib.rs:100-148, 337-447` returns `HookResult::Block { reason }` (which the dispatcher serializes to `exit code 2`, blocking the tool). The OQ-9 resolution (D-349) explicitly retired the advisory-block-mode pattern for new hooks. The comment is stale and contradicts both the implementation and the resolved OQ-9.

EVIDENCE:
- `/Users/jmagady/Dev/vsdd-factory/.worktrees/F5-review/plugins/vsdd-factory/hooks-registry.toml:931-933`
- `/Users/jmagady/Dev/vsdd-factory/.worktrees/F5-review/crates/hook-plugins/validate-per-story-adversary-convergence/src/lib.rs:100-148` (returns `HookResult::Block`)
- `/Users/jmagady/Dev/vsdd-factory/.worktrees/F5-review/crates/hook-sdk/src/result.rs:75` (`Block { .. } => 2`)
- `/Users/jmagady/Dev/vsdd-factory/.factory/cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md:28` D-349 retires advisory-block for new hooks

IMPACT: Future maintainers reading the registry will assume the hook is non-blocking. The misleading comment is exactly the kind of stale doc that survives multiple adversary passes. The same registry block is also still shipping with `on_error = "continue"` (line 933) — which is OK semantically (hook crashes don't block) but doubles down on the "advisory" misframing.

FIX: DOC — Update the comment block at lines 924-933 to: "Canonical-form WASM hook (HOST_ABI block_with_fix). Block signal: HookResult::Block via dispatcher exit code 2 (BC-4.10.001 PC8). on_error = continue means a hook crash does NOT block (ABI v1 default; hook failures degrade gracefully)." Either remove or correct the per-story-adversary-convergence-hook.bats test "structural: source uses block_with_fix and registry sets on_error=continue" (line 156) which conflates the two unrelated concepts — block_with_fix is canonical Why/Fix/Code, while on_error="continue" is hook-crash policy.

CONFIDENCE: HIGH

---

### FINDING [HIGH] — bats test conflates `block_with_fix` (block message format) with `on_error = "continue"` (hook-crash policy)

WHY: `per-story-adversary-convergence-hook.bats:156-168` ("structural: source uses block_with_fix and registry sets on_error=continue (AC-002, BC-4.10.001 PC8)") asserts BOTH that source uses `block_with_fix(` AND that `on_error = "continue"` is set in the registry, calling the latter "advisory-block mode". These are orthogonal: `block_with_fix` formats the block message; `on_error` controls what happens when the hook itself crashes. AC-002 (S-12.02) does not require `on_error = "continue"`; that is a separate operational decision.

EVIDENCE:
- `/Users/jmagady/Dev/vsdd-factory/.worktrees/F5-review/plugins/vsdd-factory/tests/per-story-adversary-convergence-hook.bats:156-168`
- `/Users/jmagady/Dev/vsdd-factory/.factory/stories/S-12.02-per-story-adversary-convergence-hook.md:79-97` (AC-002 mentions advisory block but per OQ-9/D-349 that wording is outdated)
- BC-4.10.001 PC8 mandates `block_with_fix`, not `on_error="continue"`

IMPACT: The conflation in the test reinforces the misframing in the registry comment. If a future operator changes `on_error` to `block` (the safer default for blocking hooks — a hook crash should fail closed), this bats test will fail for an unrelated reason and confuse the diagnosis.

FIX: TEST — Split the test into two: one asserts `block_with_fix(` is in source; the other (if kept) asserts the operator policy on `on_error` separately and references the correct rationale (BC-4.10.002 graceful degrade, NOT BC-4.10.001 PC8). The wording "advisory-block mode" should be deleted because OQ-9 retired that pattern for new hooks.

CONFIDENCE: HIGH

---

### FINDING [HIGH] — `RealCallbacks::list_stories` always returns Err — the production WASM hook can never enumerate stories at runtime

WHY: `validate-per-story-adversary-convergence/src/main.rs:61-70` implements `list_stories` as `Err(IoError("list_stories: story list must be supplied via plugin_config.stories".to_string()))`. This causes `hook_logic` (lib.rs:305-313) to take the Err branch, which logs "graceful degrade — invoked outside wave-gate context or cycle directory absent" and returns `HookResult::Continue`. In production, the hook can never block, regardless of whether stories actually need convergence.

EVIDENCE:
- `/Users/jmagady/Dev/vsdd-factory/.worktrees/F5-review/crates/hook-plugins/validate-per-story-adversary-convergence/src/main.rs:61-70`
- `lib.rs:305-313` consumes the `Err` and returns Continue with the wrong log message ("cycle directory absent" — misleading; the actual cause is "no list_stories impl")
- The doc comment at main.rs:62-69 admits "In production, ... should supply the story list via plugin_config.stories" — but `hook_logic` (lib.rs:295-302) only consults `plugin_config.cycle_id`, not `plugin_config.stories`

IMPACT: BC-4.10.001 PC1 mandates the hook reads each story's state file. The current implementation cannot reach that codepath in WASM. The hook is operationally inert — every wave-gate dispatch on real Claude Code will silently return Continue with the wrong log message. The 30 cargo unit tests use `FakeCallbacks` which DOES implement `list_stories` via injected vec, so the tests pass — but they are testing a path that production never reaches. This is the classic "tests pass for the wrong reason" trap.

FIX: CODE — Implement one of:
1. Read `payload.plugin_config.stories` (a `Vec<String>` field) and surface it through `list_stories`. Requires the wave-gate dispatcher / orchestrator to populate `plugin_config.stories` before SubagentStop.
2. Read a manifest file (e.g., `.factory/cycles/<cycle-id>/wave-stories.json`) via `host::read_file` and parse the story list from there.
3. Walk the cycle directory via a new host capability (would bump HOST_ABI_VERSION — bad).

Recommend option (1) or (2). Per AC-013 (S-12.02) and BC-4.10.001 PC1 this is a delivery defect, not a deferred enhancement. POLICY: implicit — spec/code consistency, no_silent_failures (SOUL.md #4).

CONFIDENCE: HIGH

---

### FINDING [HIGH] — VP-071 kani harness in shipped lib.rs uses `HookResult::Block` but VP-071 spec proof harness uses `HookResult::BlockWithFix`

WHY: VP-071.md lines 91, 112, 130, 147 declare the kani harness as `matches!(result, HookResult::BlockWithFix { .. })`. The actual kani harness inlined at `lib.rs:480, 501, 519, 536` uses `matches!(result, HookResult::Block { .. })`. The spec-vs-code mismatch on the kani harness itself.

EVIDENCE:
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/verification-properties/VP-071.md:91` (`HookResult::BlockWithFix`)
- `/Users/jmagady/Dev/vsdd-factory/.worktrees/F5-review/crates/hook-plugins/validate-per-story-adversary-convergence/src/lib.rs:480` (`HookResult::Block`)

IMPACT: A formal-verifier following VP-071 verbatim would author a kani harness that does not compile (no such variant). The implementation silently corrected this. Without an amendment to VP-071, future re-derivation from spec will break. POLICY: 4, 9 (vp_index_is_vp_catalog_source_of_truth — spec drift).

FIX: SPEC — Same fix as the VP-071 critical finding above. Amend VP-071 to v1.2 with corrected harness skeleton matching the production SDK.

CONFIDENCE: HIGH

---

### FINDING [HIGH] — `validate-artifact-path` `pattern_matches` algorithm rejects valid paths whose placeholder spans a slash

WHY: The pattern matcher at `validate-artifact-path/src/lib.rs:190-252` claims (line 188) "placeholders may span segment boundaries (slashes)". But the algorithm at line 211-219 (consecutive literals must appear immediately) and 220-233 (after-placeholder literals must find a literal "starting from pos+1") implements a single-character-greedy match, not segment-boundary-aware. More critically: at line 247 it requires `pos < path_bytes.len()` (last placeholder must consume ≥1 char), but the loop at line 203 only walks `parts.iter()` once through PARTS — it does NOT verify that placeholder content is non-empty between consecutive literals when the literal is found at offset 0. Consider pattern `.factory/specs/{x}/y.md` matching path `.factory/specs//y.md` (empty placeholder content): line 227 finds `/y.md` at offset 0 in `path[pos+1..]`, succeeding incorrectly with empty `{x}`.

EVIDENCE:
- `/Users/jmagady/Dev/vsdd-factory/.worktrees/F5-review/crates/hook-plugins/validate-artifact-path/src/lib.rs:190-252`
- `tests.rs:540-568` `test_BC_4_11_001_ac002_placeholder_cycle_id_expansion_in_cycle_doc_pattern` only tests the happy path; no empty-placeholder negative test

IMPACT: A path `.factory/cycles//decision-log.md` (double slash, empty cycle-id segment) might match the cycle-decision-log entry. Worse, path `.factory/cycles/foo/bar/baz/decision-log.md` (placeholder consumes `foo/bar/baz`) would also match — but BC-4.11.001 invariant 6 says `{placeholder}` matches "any non-empty path segment or sequence of segments." The "or sequence of segments" makes this ambiguous: are nested directories under cycles/ allowed for cycle-decision-log? The registry would then unintentionally cover wrong paths, and the implementer also has no test for this.

FIX: SPEC + TEST — Decide whether `{placeholder}` is single-segment or multi-segment. Update BC-4.11.001 invariant 6 to be precise. Add `tests.rs` cases:
- `pattern_matches(".factory/cycles//decision-log.md", "...{cycle-id}/decision-log.md")` — assert false (or true, per spec decision)
- `pattern_matches(".factory/cycles/a/b/decision-log.md", "...{cycle-id}/decision-log.md")` — assert false if single-segment

CONFIDENCE: HIGH

---

### FINDING [HIGH] — `relocate-artifact` skill is `allowed-tools: Read, Bash` but BC-6.22.001 invariant 4 mandates "git mv is the only mechanism" — Bash with no fence allows arbitrary commands

WHY: `relocate-artifact/SKILL.md:6` declares `allowed-tools: Read, Bash`. The skill body (line 73) shows `git mv` invocations, but the `Bash` permission is unconstrained. The skill prose in line 73 explicitly invokes `git mv` but the runtime has no enforcement that prevents the skill (or any agent following it) from `cp` + `rm`, `mv`, or anything else.

EVIDENCE:
- `/Users/jmagady/Dev/vsdd-factory/.worktrees/F5-review/plugins/vsdd-factory/skills/relocate-artifact/SKILL.md:6`
- BC-6.22.001 invariant 4: "git mv is the ONLY mechanism for moving files. Direct file copy + delete is prohibited because it breaks `git log --follow`."

IMPACT: An agent invoking this skill could violate BC-6.22.001 invariant 4 and lose `git log --follow` history. There is no hook to detect this. Per the relocate-artifact preflight gate (D-350), the skill ran during S-13.01 Chunk 6 and reported zero violations — but if a future invocation breaks invariant 4, the violation is silent.

FIX: TEST — Add a bats test in `relocate-artifact.bats` that fixtures a misplaced artifact, runs `--apply`, and asserts `git log --follow <new_path>` shows the rename (not a delete + add). Alternatively, add a CI lint that scans skill bash blocks for `cp` + `rm` patterns near `git mv` invocations. POLICY: implicit — spec/code consistency.

CONFIDENCE: MEDIUM (the prose says git mv; the gap is enforcement)

---

### FINDING [HIGH] — Wave-gate Gate 3 narrowing language conflates "scope" with "blocking authority"

WHY: `wave-gate/SKILL.md:80-100` (Gate 3) narrows scope to cross-story/integration only, but does NOT narrow Gate 3's blocking authority over per-story findings. S-12.01 Architecture Compliance Rule 4 (line 277) explicitly says "The narrowing applies to finding scope, not to the gate's ability to block." But the SKILL.md as shipped reads (line 88): "Out of scope for Gate 3: within-story findings (assumed converged)" — this implies Gate 3 will not block on within-story findings even if they leak through. If a within-story regression leaks past Step 4.5 (e.g., Step 4.5 was bypassed by the bootstrap exception), Gate 3 might not catch it.

EVIDENCE:
- `/Users/jmagady/Dev/vsdd-factory/.worktrees/F5-review/plugins/vsdd-factory/skills/wave-gate/SKILL.md:80-100`
- `/Users/jmagady/Dev/vsdd-factory/.factory/stories/S-12.01-per-story-adversary-workflow.md:275-281` (Architecture Compliance Rule 4)

IMPACT: The bootstrap-exception cohort (the 3 stories of THIS cycle, per D-354) did not go through Step 4.5. If Gate 3 strictly enforces the "out of scope: within-story findings" narrowing, those bootstrap stories' within-story defects (which this F5 pass exists to surface) would not be picked up at any wave-gate check either. The narrowing language is too aggressive given the documented bootstrap exception.

FIX: SPEC — Amend `wave-gate/SKILL.md:88` to: "Out of scope for Gate 3 origination: within-story findings should be discovered at Step 4.5. However, Gate 3 retains blocking authority over CRITICAL or HIGH within-story findings that surface at the wave perimeter (e.g., bootstrap-exception stories that pre-date Step 4.5)." Cross-reference D-354 bootstrap exception.

CONFIDENCE: MEDIUM

---

### FINDING [HIGH] — `parse_registry` (signature in story spec) does not exist; the actual function is `load_registry`

WHY: S-13.01 AC-001 says "The `parse_registry` function in `validate-artifact-path/src/lib.rs` never panics on any byte sequence input." The actual function name in `lib.rs:119` is `load_registry`. There is no `parse_registry` symbol.

EVIDENCE:
- `/Users/jmagady/Dev/vsdd-factory/.factory/stories/S-13.01-path-governance-bundle.md:73` (says parse_registry)
- `/Users/jmagady/Dev/vsdd-factory/.factory/stories/S-13.01-path-governance-bundle.md:295` task T-2 also says `parse_registry`
- `/Users/jmagady/Dev/vsdd-factory/.worktrees/F5-review/crates/hook-plugins/validate-artifact-path/src/lib.rs:119` (actual: `load_registry`)
- `tests/proptest_registry_load.rs` is named after the actual function

IMPACT: The story-spec/implementation drift means anyone using grep on the spec to find the implementation will get zero hits, and a future test-writer regenerating tests from the spec might author tests for a non-existent function. Same drift exists for `match_path` in S-13.01 vs the actual `matches_canonical`.

FIX: SPEC — Update S-13.01 AC-001 and T-2 to use the actual function names `load_registry` and `matches_canonical`. POLICY: 4 (semantic_anchoring_integrity).

CONFIDENCE: HIGH

---

### FINDING [HIGH] — VP-070 kani harness signature `match_path(path, registry)` does not exist; production fn is `matches_canonical`

WHY: S-13.01 AC-002 says VP-070 verifies `match_path(path, registry)` is pure. The actual production fn is `matches_canonical`, and the kani harness in `lib.rs:517` correctly calls `matches_canonical`. The spec describes a non-existent symbol.

EVIDENCE:
- `/Users/jmagady/Dev/vsdd-factory/.factory/stories/S-13.01-path-governance-bundle.md:80-86`
- `/Users/jmagady/Dev/vsdd-factory/.worktrees/F5-review/crates/hook-plugins/validate-artifact-path/src/lib.rs:150` (actual fn)

IMPACT: Same as above. Spec-implementation drift on canonical function names. POLICY: 4.

FIX: SPEC — Update S-13.01 AC-002 and VP-070 to use `matches_canonical`. The kani harness is correct, so only the spec needs updating.

CONFIDENCE: HIGH

---

### FINDING [HIGH] — All 30+ unit tests in convergence hook use `catch_unwind` + `assert!(result.is_ok(), ...)` "production not yet implemented" pattern even though production IS implemented

WHY: Tests in `validate-per-story-adversary-convergence/src/lib.rs:740-1599` follow the pattern:
```
let result = std::panic::catch_unwind(...|| hook_logic(&payload, &callbacks));
assert!(result.is_ok(), "... — production function is not yet implemented (AC-NNN)");
if let Ok(hook_result) = result { ... actual assertion ... }
```
The `assert!(result.is_ok(), ...)` line is a Red Gate scaffold that should have been removed once production was implemented (the production fn no longer panics). It now produces misleading failure messages — if `hook_logic` ever DOES panic, the test will print "production function is not yet implemented" even though it has been for weeks.

EVIDENCE: 
- `/Users/jmagady/Dev/vsdd-factory/.worktrees/F5-review/crates/hook-plugins/validate-per-story-adversary-convergence/src/lib.rs:750-767, 781-808, 822-848, 859-876, 898-927, ...` (recurring pattern)

IMPACT: When a future regression causes `hook_logic` to panic, the test failure message will misdirect the engineer ("not yet implemented") instead of pointing to the panic. It also masks the test's real purpose: tests should assert that production behaves correctly, not that it doesn't panic. The catch_unwind wrapping makes every test pass for the wrong reason if the production panic message happens to mention what the assertion expects.

FIX: TEST — Remove the catch_unwind wrapping for tests where the production code is implemented (i.e., all of them). Replace with direct `let hook_result = hook_logic(&payload, &callbacks); assert!(matches!(hook_result, ...), ...);`. Update the inline `// not yet implemented` comments. The Red Gate scaffolding pattern was for the failing-test phase; once the implementer made tests pass, it was supposed to be cleaned up. POLICY: 11 (no_test_tautologies — borderline; the tests do call production, but the redundant catch_unwind layer obscures what's being tested).

CONFIDENCE: HIGH

---

### FINDING [HIGH] — `RealCallbacks::log_debug` actually calls `host::log_info`, contradicting BC-4.10.002 PC3

WHY: BC-4.10.002 PC3 mandates the graceful-degrade path "logs a single advisory message via `host::log_debug(...)`". The implementation at `validate-per-story-adversary-convergence/src/main.rs:74-76` maps `log_debug` to `host::log_info`. The doc comment at lib.rs:251-253 explicitly admits the substitution: "BC-4.10.002 postcondition 3 references `host::log_debug` but the implemented hook uses `host::log_info` for advisory messages".

EVIDENCE:
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-04/BC-4.10.002.md:55-58` PC3 (mandates `host::log_debug`)
- `/Users/jmagady/Dev/vsdd-factory/.worktrees/F5-review/crates/hook-plugins/validate-per-story-adversary-convergence/src/main.rs:74-76`
- `/Users/jmagady/Dev/vsdd-factory/.worktrees/F5-review/crates/hook-plugins/validate-per-story-adversary-convergence/src/lib.rs:251-253` (acknowledged drift)

IMPACT: Telemetry filters keyed on log level (e.g., "show only debug events from this hook") will see info-level events instead. The implementation acknowledges the drift but no spec amendment was made — this is exactly the kind of unilateral spec violation that the cycle is designed to prevent.

FIX: SPEC — Amend BC-4.10.002 PC3 to: "Logs a single advisory message via host::log_info (the SDK's HOST_ABI v1 does not expose log_debug; log_info is the lowest-severity level available)." Update the lib.rs comment to drop the "the implemented hook uses host::log_info instead" wording. POLICY: implicit — spec/code consistency.

CONFIDENCE: HIGH

---

### FINDING [HIGH] — `HookCallbacks::read_file` is `FnOnce` in `validate-artifact-path/lib.rs:315` — the registry can only be read once per hook invocation, but tests inject closures that capture by `move` and clone

WHY: `validate-artifact-path/src/lib.rs:313-325` declares `HookCallbacks<R, E, L>` with `R: FnOnce(&str) -> Result<String, String>`. `FnOnce` means the closure is consumed after one call. The hook only calls `read_file` once for the registry path, so this is fine in production. But `tests.rs:122-153` `run_logic` injects `read_file: move |_path| registry_result.clone()` — the `move` captures `registry_result: Result<String, String>`. Since `read_file` is `FnOnce`, calling `clone()` is redundant (the closure body owns `registry_result`); but more importantly, the `R: FnOnce` bound means `hook_logic` cannot retry the read. The first non-fatal error path (e.g., `host::read_file` returning `Err` due to capability throttling) causes immediate graceful-degrade.

EVIDENCE:
- `/Users/jmagady/Dev/vsdd-factory/.worktrees/F5-review/crates/hook-plugins/validate-artifact-path/src/lib.rs:315` (`R: FnOnce`)
- `/Users/jmagady/Dev/vsdd-factory/.worktrees/F5-review/crates/hook-plugins/validate-artifact-path/src/tests.rs:136`

IMPACT: Less critical than it sounds — `host::read_file` is itself reliable for a static registry — but the `FnOnce` bound prevents future enhancement (e.g., reading multiple files for layered registries). It also makes the `read_file_called` assertion in tests harder to express because each test must instantiate a new closure. AC-004 in tests.rs:732-772 uses `Arc<Mutex<bool>>` to work around this — a smell that indicates the trait bound is too restrictive.

FIX: CODE — Change `R: FnOnce(&str) -> Result<String, String>` to `R: FnMut(&str) -> Result<String, String>`. Negligible production cost; significant testability and future-flex gain. POLICY: implicit — testability.

CONFIDENCE: MEDIUM

---

### FINDING [HIGH] — Convergence-state file path discrepancy: registry has `state-runtime-adversary` pointing to `.factory/adversary-convergence-state.json` (root) but BC-5.39.001 mandates `.factory/cycles/<cycle-id>/<story-id>/adversary-convergence-state.json`

WHY: `artifact-path-registry.yaml:131-134` registers:
```yaml
- artifact_type: state-runtime-adversary
  canonical_path_pattern: ".factory/adversary-convergence-state.json"
  description: Adversary convergence runtime state file
```
But BC-5.39.001 PC2 + invariant 3 mandate the file lives at `.factory/cycles/<cycle-id>/<story-id>/adversary-convergence-state.json` (per-story, under the cycle directory). The registry's pattern would BLOCK the per-story write because `.factory/cycles/v1.0-feature-engine-discipline-pass-1/S-12.01/adversary-convergence-state.json` does not match `.factory/adversary-convergence-state.json`.

EVIDENCE:
- `/Users/jmagady/Dev/vsdd-factory/.worktrees/F5-review/plugins/vsdd-factory/config/artifact-path-registry.yaml:131-134`
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-05/BC-5.39.001.md:55-71` PC2 + line 86-87 invariant 3
- `/Users/jmagady/Dev/vsdd-factory/.worktrees/F5-review/plugins/vsdd-factory/workflows/phases/per-story-delivery.md:141` ("`.factory/cycles/<cycle-id>/<story-id>/adversary-convergence-state.json`")

IMPACT: When the next story runs through Step 4.5 (post-bootstrap), the adversary will attempt to write `.factory/cycles/<cycle-id>/<story-id>/adversary-convergence-state.json`. The validate-artifact-path WASM hook will check the registry, find no matching pattern (the per-story path doesn't match either the root pattern OR any cycle-document/cycle-story-implementation pattern that requires `.md` suffix), and BLOCK the write with `ARTIFACT_PATH_UNREGISTERED`. The per-story adversary loop will fail to persist state on its very first run after the bootstrap cohort. This is an integration defect across S-13.01 (which authored the registry) and S-12.01 (which defined the per-story state path).

FIX: SPEC + CONFIG — Either:
1. Add a registry entry: `artifact_type: state-runtime-adversary-per-story`, pattern `.factory/cycles/{cycle-id}/{story-id}/adversary-convergence-state.json`, enforcement_level: block.
2. Change the existing entry's pattern to `.factory/cycles/{cycle-id}/{story-id}/adversary-convergence-state.json` and remove the root-level legacy entry (no production code currently writes to `.factory/adversary-convergence-state.json` at the root).

Recommend (1) — preserves backward compatibility with any tooling that writes a global state file. Without this fix, the entire S-12.01 / S-12.02 convergence machinery is bricked the moment a non-bootstrap story tries to use it. POLICY: 4, integration.

CONFIDENCE: HIGH

---

### FINDING [HIGH] — Registry pattern `cycle-document` (`.factory/cycles/{cycle-id}/{filename}.md`) overlaps with `cycle-decision-log`, `cycle-index`, `phase-delta-analysis`, `adversarial-review` — first-match-wins ordering matters and is undocumented

WHY: `artifact-path-registry.yaml` declares 5 patterns under `.factory/cycles/{cycle-id}/...`:
- L86: `cycle-decision-log` → `.factory/cycles/{cycle-id}/decision-log.md`
- L91: `cycle-index` → `.factory/cycles/{cycle-id}/INDEX.md`
- L96: `phase-delta-analysis` → `.factory/cycles/{cycle-id}/{phase}-delta-analysis.md`
- L101: `cycle-document` → `.factory/cycles/{cycle-id}/{filename}.md`
- L111: `adversarial-review` → `.factory/cycles/{cycle-id}/adv-{slug}.md`

The `cycle-document` pattern at L101 is a SUPERSET of all the others (the `{filename}` placeholder matches `decision-log`, `INDEX`, `F1-delta-analysis`, `adv-cycle-pass-1`, etc.). Per BC-4.11.001 EC-005 first-match-wins, the order matters: the more-specific entries must precede `cycle-document` for them to take effect. They do, in the YAML — but a future edit reordering entries would silently change classification. This invariant is not documented or tested.

EVIDENCE:
- `/Users/jmagady/Dev/vsdd-factory/.worktrees/F5-review/plugins/vsdd-factory/config/artifact-path-registry.yaml:86-113`
- `/Users/jmagady/Dev/vsdd-factory/.worktrees/F5-review/crates/hook-plugins/validate-artifact-path/src/tests.rs:438-471` `test_BC_4_11_001_ac002_matches_canonical_first_match_wins_for_ambiguous_path` tests the rule but not the registry's specific ordering invariant

IMPACT: A future developer who alphabetizes the registry will silently reclassify all `decision-log.md` writes from `cycle-decision-log` to `cycle-document`, losing the more-specific telemetry classification. Worse, if `cycle-document` were moved BEFORE `adversarial-review`, this very review document would be classified as a generic cycle-document, not as an adversarial-review.

FIX: TEST + DOC — Add a registry-ordering test in `vp-072-sot-invariant.bats` that asserts `cycle-decision-log`, `cycle-index`, `phase-delta-analysis`, `adversarial-review` all appear BEFORE `cycle-document` in the YAML. Add a comment block in `artifact-path-registry.yaml` near the cycles section explaining the first-match-wins ordering invariant. Alternatively, change `cycle-document`'s pattern to `.factory/cycles/{cycle-id}/{filename}.md` with explicit anti-patterns excluded — but Rust pattern syntax doesn't support that, so ordering is the only mechanism.

CONFIDENCE: HIGH

---

### FINDING [HIGH] — F1-delta-analysis VP-071 description says "advisory-block output always emitted on non-cleared gate" but VP-071 v1.1 retired that pattern (OQ-9)

WHY: F1-delta-analysis.md line 342 still describes VP-071 as "advisory-block output always emitted on non-cleared gate" — kani. This is the deprecated pre-OQ-9 wording. The actual VP-071 v1.1 (post-OQ-9) is "Block Invariant" — `block_with_fix` form.

EVIDENCE:
- `/Users/jmagady/Dev/vsdd-factory/.factory/cycles/v1.0-feature-engine-discipline-pass-1/F1-delta-analysis.md:342`
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/verification-properties/VP-071.md:38-78` (v1.1 statement)

IMPACT: Anyone reading F1-delta-analysis as the cycle's authoritative scope summary will think VP-071 still uses the deprecated advisory pattern. The OQ-9 fix (D-349) updated VP-071 itself but did not propagate to F1-delta-analysis. POLICY: 9 (vp_index_is_vp_catalog_source_of_truth — propagation).

FIX: DOC — Update F1-delta-analysis.md line 342 to: "VP-071: validate-per-story-adversary-convergence Block Invariant — kani harness verifies HookResult::Block on non-converged input (canonical block_with_fix form)". Add a footnote or amendment note pointing to D-349 / VP-071 v1.1.

CONFIDENCE: HIGH

---

### FINDING [HIGH] — Story S-12.02 AC-002 still uses the deprecated "advisory block signal" wording after OQ-9 was resolved

WHY: S-12.02 AC-002 (lines 81-93) describes the hook as emitting "advisory block signal (stdout JSON `{"outcome":"block",...}` + `hook.block` event via `host::emit_event`) and returns `HookResult::Continue`". This contradicts the post-OQ-9 BC-4.10.001 PC8 (block_with_fix → HookResult::Block, exit code 2) and the actual implementation. The S-12.02 spec was authored 2026-05-06; OQ-9 was resolved 2026-05-07 (D-349). The story spec was never amended.

EVIDENCE:
- `/Users/jmagady/Dev/vsdd-factory/.factory/stories/S-12.02-per-story-adversary-convergence-hook.md:81-93`
- `/Users/jmagady/Dev/vsdd-factory/.factory/cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md:28` D-349
- S-12.02 lines 322-330 "Architecture Compliance Rules" #1 also says "No bare HookResult::block(): All block-signaling uses the advisory-block-mode pattern"

IMPACT: Story spec is internally inconsistent with the resolved OQ-9 and the BC it cites (BC-4.10.001 PC8 mandates block_with_fix). Any future story-writer reading S-12.02 as a template for a similar hook will adopt the deprecated pattern.

FIX: SPEC — Amend S-12.02 AC-002 and Architecture Compliance Rules #1 to match BC-4.10.001 PC8 and VP-071 v1.1. The "advisory-block-mode" pattern is reserved for the three named v1.0 plugins per HOST_ABI.md. POLICY: 4 (semantic_anchoring_integrity).

CONFIDENCE: HIGH

---

## Medium Findings

### FINDING [MEDIUM] — `relocate-artifact` SKILL.md does not declare a behavioral contract for handling files NOT under `.factory/`

WHY: The skill scans `.factory/` (BC-6.22.001 PC1). What about `plugins/`, `crates/`, or other directories that may also be governed by the registry in future expansions? The current `artifact-path-registry.yaml` only has entries under `.factory/`, so this is latent. But ADR-016 implies the registry could expand. There is no contract for what relocate-artifact does when called against a path outside `.factory/`.

EVIDENCE:
- `/Users/jmagady/Dev/vsdd-factory/.worktrees/F5-review/plugins/vsdd-factory/skills/relocate-artifact/SKILL.md` (no out-of-`.factory/` clause)
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-06/BC-6.22.001.md:48` PC2 only mentions `.factory/`

IMPACT: Future expansion of registry to `plugins/` or `crates/` (e.g., for hook-plugin source layout) would require respec'ing relocate-artifact. Low risk today.

FIX: SPEC — Document explicit scope clause: "relocate-artifact scans only `.factory/`. Files in `plugins/`, `crates/`, etc., are out of scope and ignored even if registry patterns reference them." POLICY: 4.

CONFIDENCE: MEDIUM

---

### FINDING [MEDIUM] — `artifact-path-registry.yaml` `cycle-story-implementation` pattern doesn't cover the per-story `red-gate-log.md` path actually written by Step 3

WHY: `per-story-delivery.md:39` says Step 3 writes `red-gate-log.md` to `.factory/cycles/<cycle-id>/<story-id>/implementation/red-gate-log.md`. The registry has `cycle-story-implementation` pattern `.factory/cycles/{cycle-id}/implementation/{filename}.md` (registry line 105-108) — note the path lacks `<story-id>` and the registered pattern doesn't have a `<story-id>` placeholder. This is a real path mismatch.

EVIDENCE:
- `/Users/jmagady/Dev/vsdd-factory/.worktrees/F5-review/plugins/vsdd-factory/workflows/phases/per-story-delivery.md:39`
- `/Users/jmagady/Dev/vsdd-factory/.worktrees/F5-review/plugins/vsdd-factory/config/artifact-path-registry.yaml:105-108`

IMPACT: When a future story passes Step 3 (post-bootstrap), the test-writer tries to write `.factory/cycles/<cycle-id>/<story-id>/implementation/red-gate-log.md`. The registry pattern at L106 does NOT have a `{story-id}` segment — it's `.factory/cycles/{cycle-id}/implementation/{filename}.md` (cycle-level implementation dir, not per-story). The write would fail with `ARTIFACT_PATH_UNREGISTERED`. Same defect class as the convergence-state-file finding above. Combined with the convergence-state path issue, the path registry as shipped does not allow per-story state files of either type.

FIX: CONFIG — Update the `cycle-story-implementation` entry to: `.factory/cycles/{cycle-id}/{story-id}/implementation/{filename}.md`. Or add a separate `per-story-implementation-log` entry. Same priority as the convergence-state path finding.

CONFIDENCE: HIGH (escalating to HIGH given the same pattern as the convergence-state file finding)

---

### FINDING [MEDIUM] — bats test `vp-072-sot-invariant.bats` only spot-checks 4-5 named skills, doesn't enumerate all `create-*`/`scaffold-*`/`register-*` skills

WHY: The VP-072 bats harness exists, but inspection shows it only checks for named skills. F1-delta-analysis OQ-4 (Section 5) explicitly flagged "Additional creation skills to survey" as a known gap. S-13.01 T-0 mandates the survey but there's no automated enforcement that NEW skills added in future cycles get the registry-read preamble.

EVIDENCE:
- `/Users/jmagady/Dev/vsdd-factory/.worktrees/F5-review/plugins/vsdd-factory/tests/vp-072-sot-invariant.bats:90-100` (named-skill checks)
- `/Users/jmagady/Dev/vsdd-factory/.factory/cycles/v1.0-feature-engine-discipline-pass-1/F1-delta-analysis.md:573-578` OQ-4
- `/Users/jmagady/Dev/vsdd-factory/.factory/stories/S-13.01-path-governance-bundle.md:288` T-0 ("Survey... before implementation proceeds")

IMPACT: A future skill `create-spec-amendment` (hypothetical) added in cycle pass-2 would not be in the bats list and would silently fail to follow the registry-read pattern.

FIX: TEST — Add a programmatic enumeration test that finds all SKILL.md files matching `create-*` / `scaffold-*` / `register-*` glob and asserts each one greps for `artifact-path-registry.yaml`. Documented exemptions go in a sidecar list. POLICY: implicit — coverage drift prevention.

CONFIDENCE: MEDIUM (process gap)

---

### FINDING [MEDIUM] — `validate-artifact-path` graceful-degrade on missing `tool_input.file_path` returns Continue but doesn't emit `hook.warn` event

WHY: BC-4.11.001 EC-006 says missing file_path: "Graceful degrade: hook returns HookResult::Continue. Logs `host::log_warn`. No block on missing data." The implementation at `validate-artifact-path/src/lib.rs:355-365` does call `(callbacks.log)(3, ...)` (level 3 = warn) — good. BUT it does not emit a `hook.warn` event via `emit_event`, while the warn-enforcement-level path at lib.rs:411-421 DOES emit `hook.warn`. Inconsistent observability.

EVIDENCE:
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-04/BC-4.11.001.md:118` EC-006
- `/Users/jmagady/Dev/vsdd-factory/.worktrees/F5-review/crates/hook-plugins/validate-artifact-path/src/lib.rs:355-365` (no event)
- `lib.rs:411-421` (warn level emits event)

IMPACT: Observability gap. A monitoring dashboard counting `hook.warn` events misses payload-malformation cases.

FIX: CODE — Add `(callbacks.emit_event)("hook.warn", &[("hook", "validate-artifact-path"), ("reason", "file_path absent from payload")]);` in the EC-006 branch. POLICY: implicit — observability consistency.

CONFIDENCE: MEDIUM

---

### FINDING [MEDIUM] — `pattern_matches` linear-time complexity but BC-4.11.001 EC-007 mandates ≤200ms for 100 entries — no benchmark or test enforces this

WHY: The pattern matcher does a linear scan over registry entries (lib.rs:157-176). For each entry, `pattern_matches` walks the path. For 100 entries × 64-byte paths, this is fast on modern CPUs — but no benchmark exists to enforce the 200ms ceiling. BC-4.11.001 EC-007 says "WASM execution time MUST remain under 200ms. Binary size constraint: keep under 500 KB."

EVIDENCE:
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-04/BC-4.11.001.md:119` EC-007
- `/Users/jmagady/Dev/vsdd-factory/.worktrees/F5-review/crates/hook-plugins/validate-artifact-path/src/lib.rs:150-176` (no perf assertions)
- No benchmark file at `crates/hook-plugins/validate-artifact-path/benches/` (would not exist; checked via Glob earlier — only `tests/` and `src/`)

IMPACT: The 200ms ceiling is unenforced. A future O(n²) pattern enhancement could silently regress.

FIX: TEST — Add a `criterion` benchmark or a bats test that loads the actual registry, runs `matches_canonical` 1000 times against a fixture path, and asserts mean elapsed < 200ms. POLICY: implicit — performance regression prevention.

CONFIDENCE: MEDIUM

---

### FINDING [MEDIUM] — Adversary's `Three-Perimeter Scope Contract` in `agents/adversary.md` says "story spec file" but never resolves the actual story-spec path pattern

WHY: `adversary.md:40` says the per-story scope includes "the story spec file" but doesn't say where to find it. Stories live under `.factory/stories/S-{story-id}-{slug}.md` per the registry. The adversary running on S-12.01 had to read `S-12.01-per-story-adversary-workflow.md` — but how does it know the slug? The story-id is well-known but the slug is not encoded in any frontmatter the adversary reads.

EVIDENCE:
- `/Users/jmagady/Dev/vsdd-factory/.worktrees/F5-review/plugins/vsdd-factory/agents/adversary.md:40`
- The registry pattern requires `S-{story-id}-{slug}.md` — slug is part of the filename but not directly in frontmatter

IMPACT: A new adversary dispatch needs to know how to find the story spec. If the orchestrator just passes "S-12.01" to the adversary without the slug, the adversary must `Glob` for `.factory/stories/S-12.01-*.md`. This works but is unstated.

FIX: SPEC — Amend `agents/adversary.md` Three-Perimeter Scope Contract to include explicit guidance: "The story spec file is found via `Glob('.factory/stories/S-{story-id}-*.md')`". Or, register a story-spec-by-id lookup pattern in the registry. POLICY: 4.

CONFIDENCE: LOW

---

### FINDING [MEDIUM] — `validate-per-story-adversary-convergence` `graceful_degrade_outside_wave_gate` only matches the literal string `wave-gate-dispatch` — missing fallback for variations

WHY: `lib.rs:200` matches `identity != "wave-gate-dispatch"`. If the orchestrator dispatches the wave-gate skill via `subagent_name = "wave-gate"` (without the `-dispatch` suffix) or `agent_type = "wave-gate-dispatcher"` etc., the hook will graceful-degrade and silently miss firing on the actual wave-gate stop. There is no canonical registry of wave-gate identity strings, so this is fragile.

EVIDENCE:
- `/Users/jmagady/Dev/vsdd-factory/.worktrees/F5-review/crates/hook-plugins/validate-per-story-adversary-convergence/src/lib.rs:198-201`

IMPACT: If the orchestrator uses any agent identity other than the exact string `wave-gate-dispatch`, the hook silently no-ops. The wave gate can be bypassed by accident through identity drift. Combined with the inert `RealCallbacks::list_stories`, the hook is doubly inert in production.

FIX: CODE — Match against a pattern (`identity.starts_with("wave-gate")`) OR canonicalize the wave-gate identity in a shared constant in the SDK. Document the canonical identity string in BC-4.10.001 invariant 1 (currently absent). POLICY: implicit.

CONFIDENCE: MEDIUM

---

### FINDING [MEDIUM] — `relocate-artifact/SKILL.md` declares cross-reference update via `Read` tool but `--apply` requires `Bash` for `git mv` — the skill cannot fix references without a write tool

WHY: `relocate-artifact/SKILL.md:6` declares `allowed-tools: Read, Bash`. To update cross-references in `.md` files (BC-6.22.001 PC6b), the skill must rewrite file contents. Bash can do that via `sed -i`, but the SKILL.md doesn't actually show how. The example output at line 76 just says "Updated 2 reference(s) in .factory/specs/bc/BC-INDEX.md" without specifying the mechanism.

EVIDENCE:
- `/Users/jmagady/Dev/vsdd-factory/.worktrees/F5-review/plugins/vsdd-factory/skills/relocate-artifact/SKILL.md:6,73-83`
- BC-6.22.001 PC6b requires cross-reference updates after `git mv`

IMPACT: The skill is documented at a high level but the `--apply` cross-reference update procedure is underspecified. An agent invoking this for the first time would have to invent the `sed`/`awk` commands. Risk: the cross-reference update could miss a reference (due to encoding differences, escape characters in slugs, etc.) and the validate-artifact-path hook would later block writes targeting the new (correct) path while old references still cite the old path.

FIX: SPEC — Add an explicit step to `relocate-artifact/SKILL.md` showing the cross-reference update mechanism (e.g., `grep -rl <old-path> .factory/ | xargs sed -i '' "s|<old-path>|<new-path>|g"`). Add a post-update verification step that grep finds zero remaining references to old paths.

CONFIDENCE: MEDIUM

---

### FINDING [MEDIUM] — F2 BC reference in F1-delta-analysis says BC-5.NN.001 in VP-071 BC traces; VP-071 still references the placeholder BC-5.NN.002 in line 31 instead of the actual BC IDs

WHY: VP-071.md frontmatter line 30-31 has:
```
bcs:
  - see PO output for actual IDs — state-manager will cross-link in BC-INDEX update
  - BC-5.NN.002 (validate-per-story-adversary-convergence WASM hook behavioral contract)
```
The placeholder `BC-5.NN.002` was supposed to be resolved to the actual BC ID. The actual hook BC is `BC-4.10.001`. The VP frontmatter still cites a non-existent `BC-5.NN.002`.

EVIDENCE:
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/verification-properties/VP-071.md:30-31`

IMPACT: Cross-reference traceability tools that read VP frontmatter will see a placeholder. The Related BCs section at line 199-206 also says "BC-5.NN.002" and "See PO output for actual BC IDs". POLICY: 9 (vp_index_is_vp_catalog_source_of_truth — VP frontmatter must be self-consistent post-spec-evolution).

FIX: SPEC — Update VP-071 frontmatter `bcs:` to `[BC-4.10.001, BC-5.39.001]`. Update Related BCs section accordingly. The state-manager TODO at line 31 was never executed.

CONFIDENCE: HIGH (escalating to HIGH)

---

### FINDING [MEDIUM] — The "policy rubric" mentioned in `agents/orchestrator/orchestrator.md:243` is auto-loaded for adversary dispatch, but my dispatch did not include the rubric loading (process gap)

WHY: orchestrator.md line 243 says "Before every adversary dispatch, read `.factory/policies.yaml` (if it exists) and inject all policies into the adversary's task prompt under a `## Project Policy Rubric` heading." My dispatch task included some text about the policy rubric ("Read .factory/policies.yaml..."), but no canonical "Project Policy Rubric" section was included. The adversary therefore had to load the policies file independently — which I did — but a future dispatch that doesn't include the explicit instruction will silently skip policies.

EVIDENCE:
- `/Users/jmagady/Dev/vsdd-factory/.worktrees/F5-review/plugins/vsdd-factory/agents/orchestrator/orchestrator.md:243-249`
- This dispatch's prompt: contained "Read .factory/policies.yaml" but did not pre-inject the rubric into the prompt body

IMPACT: Policy-rubric injection is not automated. Each dispatch depends on the orchestrator remembering. Per S-12.01's mandate to reconcile orchestrator MANDATORY STEPS, this should be an enforced pre-dispatch step — but it isn't.

FIX: SPEC + CODE — Add a hook (or a wrapper skill) that auto-prepends `policies.yaml` content to any adversary dispatch prompt. Or add a validation hook on adversary task prompts that asserts the rubric section is present. `[process-gap]` POLICY: implicit — policy enforcement consistency.

CONFIDENCE: MEDIUM

---

## Observations

### OBSERVATION [LOW] — F1-delta-analysis is a pre-implementation scoping doc and contains many "provisional" estimates that are now stale

WHY: F1-delta-analysis.md:289-313 has a "Proposed BC IDs" table using placeholder NN values (e.g., `BC-4.NN.001`). The actual IDs (BC-4.10.001, BC-4.11.001, etc.) are recorded in the decision-log but F1 was never updated to reflect them. This is intentional ("F1 was the upstream proposal") but a downstream reader may not realize.

EVIDENCE: `/Users/jmagady/Dev/vsdd-factory/.factory/cycles/v1.0-feature-engine-discipline-pass-1/F1-delta-analysis.md:289-313`

IMPACT: Confusion. Adding a one-line "Final IDs assigned in F2 — see decision-log D-340" footnote at the head of the table would suffice.

FIX: DOC — Optional. If the convention is "F1 docs are immutable upstream proposals", document that convention in the cycle's INDEX.md.

CONFIDENCE: LOW

---

### OBSERVATION [LOW] [process-gap] — F5 adversary dispatch flow assumes adversary writes its own document, contradicting agents/adversary.md `tool_access: read-only`

WHY: The user's dispatch instruction said "the previous F5 pass-1 dispatch failed to write its output". `agents/adversary.md:320-326` explicitly states the adversary's `tool_access: read-only`, and "Findings are returned as chat text — the orchestrator persists them via state-manager." The dispatch flow should not depend on the adversary writing — it should always go through state-manager.

EVIDENCE:
- `/Users/jmagady/Dev/vsdd-factory/.worktrees/F5-review/plugins/vsdd-factory/agents/adversary.md:319-326`
- The dispatch instruction itself contradicts this

IMPACT: The dispatch process has a mismatch with the agent's documented tool profile. Future F5 dispatches will keep hitting the same friction. The orchestrator must learn to dispatch adversary then dispatch state-manager to persist the chat output.

FIX: PROCESS — Update the F5 dispatch playbook (in `phase-5-adversarial-refinement.lobster` or the F5 skill, if it exists) to (a) dispatch adversary, (b) capture chat output, (c) dispatch state-manager with the captured output to write to the canonical adversary-review path. Remove instructions in dispatch prompts that ask the adversary to write directly. `[process-gap]`

CONFIDENCE: HIGH

---

### OBSERVATION [LOW] [process-gap] — Bootstrap-exception cohort (D-354) means F5 pass-1 IS the convergence loop for the 3 cycle stories — but no `.factory/cycles/<cycle>/<story-id>/adversary-convergence-state.json` files exist for them

WHY: D-354 acknowledges the 3 cycle stories were exempt from Step 4.5. This F5 dispatch treats this pass as the "first" convergence loop for them. But no state files exist for any of S-13.01, S-12.01, S-12.02 at the canonical path. The convergence loop's state-tracking is therefore not populated for the bootstrap cohort.

EVIDENCE:
- D-354 in `/Users/jmagady/Dev/vsdd-factory/.factory/cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md:33`
- No files at `/Users/jmagady/Dev/vsdd-factory/.factory/cycles/v1.0-feature-engine-discipline-pass-1/S-12.01/` etc. (verified via Glob early in this pass)

IMPACT: When the next wave-gate dispatch runs, the validate-per-story-adversary-convergence hook (if the `list_stories` issue above is fixed) will find no state files for these stories and return CONVERGENCE_STATE_MISSING. The bootstrap cohort will retroactively block wave-gate dispatch unless explicitly waived.

FIX: SPEC + STATE — Either (a) document a per-story state-file backfill requirement in F7 cycle-close (write state files showing `passes_clean: 3, last_classification: NITPICK_ONLY` for the bootstrap cohort, with an annotation that this F5 pass is the source); or (b) add a registry-level exemption for the bootstrap cycle's stories. `[process-gap]`

CONFIDENCE: MEDIUM

---

### OBSERVATION [LOW] — D-355 says "no open spec contradictions (OQ-9 resolved D-349)" but VP-071 frontmatter still has placeholder BC IDs

WHY: D-355 declares spec entry to F5 with "no open spec contradictions". The VP-071 placeholder BC IDs (finding above) are an open spec inconsistency that should have been caught in F7 of the prior burst.

EVIDENCE: `/Users/jmagady/Dev/vsdd-factory/.factory/cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md:34` D-355

IMPACT: The "no open contradictions" claim was overstated.

FIX: PROCESS — Add a pre-F5 lint check that scans new VP/BC frontmatter for placeholder strings (`BC-S.NN`, `VP-NNN`, `[pending-recompute]`, `TBD`, `to be determined`) and surfaces them as gate failures. `[process-gap]`

CONFIDENCE: MEDIUM

---

### OBSERVATION [LOW] — Many BC files have `input-hash: "[pending-recompute]"` in frontmatter — drift not corrected by state-manager

WHY: All 6 new BC files have `input-hash: "[pending-recompute]"`. F2 was completed (per D-340, D-342) but input-hash was never recomputed. This is a known issue tracked separately, but it's worth noting for this cycle.

EVIDENCE:
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-04/BC-4.10.001.md:12`
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-04/BC-4.10.002.md:12`
- (Same in BC-4.11.001, BC-5.39.001, BC-5.39.002, BC-6.22.001)

IMPACT: The input-hash drift check (orchestrator MANDATORY STEPS line 121) cannot establish a baseline for these BCs.

FIX: SPEC — Run `/vsdd-factory:check-input-drift` and accept the recomputed hashes for the 6 new BCs. `[process-gap]` (recurring pattern from prior cycles).

CONFIDENCE: MEDIUM

---

## Novelty Assessment

This is pass 1 — by definition, all findings are novel. No prior F5 pass exists to compare against. Findings span all three perimeters:

- **Per-story scope:** F-CRIT-3 (VP-071 spec drift), F-CRIT-2 (ADR slug), F-HIGH-2 (bats conflation), F-HIGH-9 (function-name drift in S-13.01), F-HIGH-10 (VP-070 fn name drift), F-HIGH-12 (S-12.02 advisory wording), F-HIGH-13 (RealCallbacks::list_stories inert), F-HIGH-14 (RealCallbacks::log_debug → log_info), F-MED-7 (VP-071 placeholder BC IDs)
- **Cross-story scope:** F-CRIT-1 (Lobster step file missing), F-CRIT-4 (emit_event missing), F-HIGH-13 (state-file path mismatch between S-12.01 schema and S-13.01 registry), F-MED-2 (red-gate-log path mismatch)
- **Cycle-level scope:** F-HIGH-7 (Gate 3 over-narrowing vs bootstrap), F-OBS-2 (process-gap: F5 dispatch flow), F-OBS-3 (process-gap: bootstrap cohort state-file backfill)

Novelty: HIGH — substantial gaps found across all three perimeters. Several CRITICAL findings (broken Lobster dispatch, broken state-file path, orphan SDK variant) would prevent the next cycle from functioning correctly. Pass 2 should re-verify these are fixed.

The cycle delivered the SPEC and the SCAFFOLDING for engine governance; what it did NOT deliver is end-to-end functional integration. The hooks compile and unit-test green, but the production dispatch path is bricked at multiple integration points (list_stories inert, state file path unregistered, lobster skill missing).

---

## Self-Validation Loop (3 iterations)

**Iteration 1 (evidence check):** All 26 findings have file:line evidence. F-CRIT-2 (ADR slug) cross-checked via Glob result confirming only one ADR-017 file exists. F-CRIT-3 (HookResult variant) cross-checked via direct read of result.rs:18-32. F-CRIT-4 (emit_event missing) cross-checked via two grep runs. F-HIGH-13 (list_stories inert) verified by reading both main.rs:61-70 and lib.rs:298-302 to confirm cycle_id is read from plugin_config but stories are not.

**Iteration 2 (actionability check):** Every finding has a concrete fix recommendation with route hint (SPEC/CODE/TEST/DOC). The MEDIUM findings on `pattern_matches` placeholder semantics (F-HIGH-6) suggest two specific test additions. The CRITICAL Lobster fix (F-CRIT-1) names the specific file path to author.

**Iteration 3 (duplication check):** F-CRIT-3 (VP-071 BlockWithFix variant) and F-HIGH-5 (VP-071 kani harness uses Block) are related but distinct: the first is a spec-vs-SDK-shape contradiction; the second is a spec-vs-implementation-harness contradiction. Both should be fixed together via a VP-071 v1.2 amendment but they have different technical nuances and warrant separate findings. Same with F-HIGH-2 (bats wording) and F-HIGH-1 (registry comment wording) — both stem from the post-OQ-9 wording rot, but in different files. Kept separate.

After 3 iterations, all findings stand.

---

PASS_CLASSIFICATION: CRITICAL

---

# Return Summary

(a) **Path of review document:** **NOT WRITTEN** — read-only profile (per `agents/adversary.md:319-326`) prevents Write/Edit. Full review delivered as chat text above. Orchestrator must dispatch state-manager with this output to persist to `/Users/jmagady/Dev/vsdd-factory/.factory/cycles/v1.0-feature-engine-discipline-pass-1/adv-cycle-pass-1.md`. (Registry pattern `adversarial-review` `.factory/cycles/{cycle-id}/adv-{slug}.md` accepts this path.)

(b) **Findings count by severity:**
- CRITICAL: 4 (broken Lobster dispatch; ADR slug drift in 3 specs; VP-071 SDK variant ghost; missing emit_event)
- HIGH: 14 (registry-comment drift, bats conflation, list_stories inert, log_debug→log_info, FnOnce read_file, state-file path mismatch, registry-ordering invariant, F1 advisory wording, S-12.02 advisory wording, function-name drift S-13.01 ×2, kani harness drift, Gate 3 over-narrowing, VP-071 placeholder BC IDs, red-gate-log path mismatch)
- MEDIUM: 6 (relocate-artifact scope clause, vp-072 enumeration coverage, EC-006 missing event, perf benchmark, story-spec lookup, wave-gate identity matching, relocate-artifact cross-ref mechanism, policy-rubric injection)
- LOW (Observations): 5 (4 process-gap-tagged)

Note: Counts overlap because some findings span perimeters. Above counts use unique finding IDs.

(c) **PASS_CLASSIFICATION:** CRITICAL

(d) **Top 3 findings (1-line each):**
1. CRITICAL — Lobster workflow references skill file `step-d5-adversary-convergence.md` that does not exist on disk; first non-bootstrap story will brick at Step 4.5.
2. CRITICAL — `RealCallbacks::list_stories` always returns Err in production (only the test FakeCallbacks works); convergence hook is operationally inert in WASM.
3. CRITICAL — Three story specs anchor `inputs:` to `ADR-017-per-story-adversary-three-perimeter-model.md` but the actual on-disk file is `ADR-017-per-story-adversary-phasing.md`.

(e) **Process-gap findings:**
- OBS-2 [process-gap]: F5 dispatch flow assumes adversary writes its own document, contradicting `agents/adversary.md` read-only profile
- OBS-3 [process-gap]: Bootstrap cohort lacks state files; no F7 backfill protocol
- OBS-5 [process-gap]: Pre-F5 lint should scan for placeholder frontmatter strings (`BC-S.NN`, `[pending-recompute]`)
- OBS-6 [process-gap]: input-hash recomputation not enforced after F2 burst
- F-MED-9 [process-gap]: Policy-rubric injection into adversary dispatch is manual

(f) **Anything I couldn't review:**
- I could not run any tests or invoke the WASM hooks live (read-only profile, no Bash). All test verdicts are derived from source-code reading only — the cycle's claim of "30/30 cargo + 11+1skip bats green" was not independently re-verified.
- I did not fully read `validate-artifact-path/src/tests.rs` past line 800 (it has more tests; I covered a representative slice).
- I did not read all 9 modified creation skills (`create-adr`, `create-architecture`, etc.) end-to-end — I confirmed via grep that they reference `artifact-path-registry.yaml`, but did not verify the preamble paragraph is exactly as F1 §7 specifies.
- I did not read all 6 modified writing-agent files end-to-end (architect, product-owner, business-analyst, story-writer, technical-writer, spec-steward) — only spot-checked adversary.md.
- I did not read `relocate-artifact.bats` or `per-story-adversary-workflow.bats` test files.
- The bats test for `RealCallbacks::list_stories` failure mode was not verified to exist as a test case.
