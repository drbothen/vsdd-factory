---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-04-30T00:00:00Z
phase: 2
inputs:
  - .factory/stories/S-8.04-native-port-update-wave-state-on-merge.md
  - .factory/stories/epics/E-8-native-wasm-migration.md
  - .factory/stories/STORY-INDEX.md
input-hash: 5015917
traces_to: prd.md
pass: 1
previous_review: null
target: S-8.04 v1.0
target_file: .factory/stories/S-8.04-native-port-update-wave-state-on-merge.md
verdict: SUBSTANTIVE
clock: 0_of_3 → 0_of_3
findings_total: 17
findings_high: 7
findings_med: 6
findings_low: 3
findings_nit: 1
---

# Adversarial Review: S-8.04 v1.0 (Pass 1) — HIGHEST RISK STORY

## Finding ID Convention

`F-S804-P1-NNN` — story-scoped pass-1 sequence.

## Part B — New Findings (or all findings for pass 1)

### HIGH

#### F-S804-P1-001: CRITICAL — host::write_file does NOT exist in vsdd-hook-sdk; D-6 Option A trigger REQUIRED

- **Severity:** HIGH
- **Category:** interface-gaps
- **Location:** AC-003 / T-4 / Library & Framework Requirements
- **Description:** Empirically verified against `crates/hook-sdk/src/ffi.rs` and `crates/hook-sdk/src/host.rs`: the SDK exposes `host::read_file` and `host::emit_event` but does NOT expose `host::write_file`. The story assumes `host::write_file` is available for wave-state.yaml mutation. This is the D-6 Option A trigger: the SDK must be extended with a `write_file` host function before this story can be implemented. S-8.04 is BLOCKED pending SDK extension PR.
- **Evidence:** `crates/hook-sdk/src/host.rs` — no `write_file` symbol. `crates/hook-sdk/src/ffi.rs` — FFI bindings list does not include `__host_write_file`. Verified against codebase; not a documentation gap.
- **Proposed Fix:** Create SDK extension story (new story or sub-task) to add `host::write_file` to vsdd-hook-sdk. Mark S-8.04 BLOCKED with explicit prerequisite dependency. Update AC-003 and T-4 to reference the new SDK function once added.

#### F-S804-P1-002: Build target wasm32-wasi wrong; SDK mandates wasm32-wasip1

- **Severity:** HIGH
- **Category:** spec-fidelity
- **Location:** T-2 / Library & Framework Requirements build target
- **Description:** The story specifies `wasm32-wasi` as the build target. The SDK and project architecture mandate `wasm32-wasip1` (WASI Preview 1). `wasm32-wasi` is a deprecated alias; using it may produce warnings or errors with recent Rust toolchains.
- **Evidence:** SDK Cargo.toml and architecture documents specify `wasm32-wasip1`. `wasm32-wasi` deprecated in Rust 1.78+.
- **Proposed Fix:** Replace all `wasm32-wasi` references with `wasm32-wasip1` throughout the story.

#### F-S804-P1-003: Sibling propagation — wasm32-wasi error likely present in S-8.01..S-8.09

- **Severity:** HIGH
- **Category:** spec-fidelity
- **Location:** All Tier 1 siblings
- **Description:** The `wasm32-wasi` vs `wasm32-wasip1` error identified in F-S804-P1-002 is likely present across all 9 Tier 1 stories (S-8.01..S-8.09). A systematic fix burst is required across all siblings.
- **Proposed Fix:** Apply `wasm32-wasi` → `wasm32-wasip1` fix uniformly to all Tier 1 story files in a single systematic burst.

#### F-S804-P1-004: Capability tokens file_read/file_write do not exist in TOML schema

- **Severity:** HIGH
- **Category:** interface-gaps
- **Location:** T-5 / AC-002 capability declaration
- **Description:** The story specifies capability tokens `file_read` and `file_write` in the registry entry. The correct TOML schema uses `[hooks.capabilities.read_file]` with `path_allow=[...]` (and the analogous `write_file` block once added to SDK). The `file_read`/`file_write` flat tokens do not exist in the TOML capability schema.
- **Evidence:** Existing hooks-registry.toml entries use `[hooks.capabilities.read_file] path_allow=[".factory/..."]` structured form, not flat `file_read = true` tokens.
- **Proposed Fix:** Replace `file_read`/`file_write` with the correct structured form: `[hooks.capabilities.read_file] path_allow=[".factory/wave-state.yaml"]` and `[hooks.capabilities.write_file] path_allow=[".factory/wave-state.yaml"]` (latter pending SDK extension).

#### F-S804-P1-005: BC-7.03.083-086 unverifiable (BC files not at expected paths; S-8.00 audit prerequisite)

- **Severity:** HIGH
- **Category:** spec-fidelity
- **Location:** behavioral_contracts frontmatter
- **Description:** BC-7.03.083 through BC-7.03.086 are referenced but cannot be verified (BC files are not at expected paths; S-8.00 BC-anchor verification audit has not completed). The story cannot be validated against its behavioral contracts until the BC files are accessible and the S-8.00 audit runs.
- **Proposed Fix:** Note this as a deferred verification item; the BC paths should be confirmed as part of the S-8.00 audit. Add a [process-gap] disclosure.

#### F-S804-P1-006: serde_yaml officially DEPRECATED by dtolnay (2024); needs explicit alt-crate decision

- **Severity:** HIGH
- **Category:** spec-fidelity
- **Location:** Library & Framework Requirements — serde_yaml dependency
- **Description:** `serde_yaml` was officially deprecated by its maintainer (dtolnay) in 2024. Using it as a new workspace dependency introduces a maintenance liability. The project must explicitly decide on an alternative (e.g., `serde-yaml-ng`, `figment`, or manual YAML parsing) before authorizing the dependency addition.
- **Evidence:** dtolnay/serde-yaml: README states "This library is no longer maintained."
- **Proposed Fix:** Add a decision point: select serde_yaml alternative, update Library table to the chosen crate, and note the deprecation decision in the story changelog.

#### F-S804-P1-007: YAML semantic trichotomy not pinned (None vs missing-key vs "not_started" string)

- **Severity:** HIGH
- **Category:** ambiguous-language
- **Location:** AC-004 / EC-002 YAML deserialization semantics
- **Description:** The wave-state.yaml schema has three distinct states for a field that S-8.04 reads: (1) key absent, (2) key present with YAML `null`/`~`, (3) key present with string `"not_started"`. The story does not specify how serde deserialization handles each case, nor which is authoritative. The WASM port can silently produce wrong behavior for case (1) vs (2).
- **Proposed Fix:** Explicitly pin the YAML trichotomy: define which serde type (`Option<String>`, `Option<Option<String>>`, etc.) handles the three cases and what behavior each produces.

### MEDIUM

#### F-S804-P1-008: Race condition with S-8.03 on shared SubagentStop event NOT modeled

- **Severity:** MEDIUM
- **Category:** concurrency
- **Location:** Goal / AC list
- **Description:** S-8.03 (track-agent-stop) and S-8.04 (update-wave-state-on-merge) both fire on the SubagentStop event for pr-manager. Per SS-01 architecture, parallel-tier execution is possible. The story does not model what happens if S-8.03 and S-8.04 execute concurrently on the same event. This is particularly relevant for wave-state.yaml mutation in S-8.04.
- **Proposed Fix:** Add an EC for concurrent SubagentStop handling, or document that the SS-01 dispatcher serializes same-event hook execution (if true).

#### F-S804-P1-009: TOCTOU + non-atomic write on wave-state.yaml

- **Severity:** MEDIUM
- **Category:** concurrency
- **Location:** AC-003 write semantics
- **Description:** AC-003 specifies: read wave-state.yaml → modify → write wave-state.yaml. This is a TOCTOU (time-of-check-to-time-of-use) pattern. If another hook or process modifies wave-state.yaml between read and write, the update will silently clobber the intermediate state. No locking, atomic-rename, or CAS mechanism is specified.
- **Proposed Fix:** Specify a concurrency safety mechanism (e.g., atomic rename via temp file, advisory lock, or single-writer contract). At minimum, document the concurrency assumption explicitly.

#### F-S804-P1-010: Bash regex ERE alternation precedence ambiguity in STEP_COMPLETE pattern

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** AC-004 / T-3 pattern specification
- **Description:** The bash regex `STEP_COMPLETE: step=8.*status=ok|merged|squash.*merge` has POSIX ERE alternation precedence ambiguity. The `|` binds at alternation level, meaning the regex is parsed as `(STEP_COMPLETE: step=8.*status=ok)|(merged)|(squash.*merge)`, not `STEP_COMPLETE: step=8.*(ok|merged|squash.*merge)`. The bash hook may have a latent pattern bug. The WASM port must either faithfully replicate the (possibly buggy) bash behavior or fix it — the story must explicitly choose.
- **Proposed Fix:** Add AC or T note: either (a) replicate bash ERE alternation verbatim including potential precedence bug, or (b) fix the precedence with explicit grouping `(ok|merged|squash.*merge)` and document the intentional deviation.

#### F-S804-P1-011: EC-003 "emit event with merged count unchanged" inverted vs bash source

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** EC-003 duplicate-merge handling
- **Description:** EC-003 states "still emit event with merged count unchanged" for the duplicate-merge case. The bash source for the duplicate-merge path does NOT emit an event — it exits silently without calling bin/emit-event. The spec inverts the bash behavior.
- **Evidence:** Bash source: duplicate-merge path → `exit 0` with no emit call.
- **Proposed Fix:** Correct EC-003 to specify: duplicate-merge detected → exit 0 silently (no event emitted), matching bash source.

#### F-S804-P1-012: T-9 prescribes removing python3 from binary_allow but python3 is not there

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** T-9
- **Description:** T-9 says "remove python3 from binary_allow." The actual registry entry for update-wave-state-on-merge does not list python3 in binary_allow. This is either a copy-paste error from another story or reflects that the bash hook previously used python3 as a soft dependency that was already removed. Attempting to delete a non-existent entry is a no-op but suggests the story was not verified against the actual registry state.
- **Proposed Fix:** Remove T-9's python3 removal instruction, or verify that the registry currently has python3 and update accordingly.

#### F-S804-P1-013: [hooks.capabilities.read_file] path_allow scope missing from migration plan

- **Severity:** MEDIUM
- **Category:** missing-edge-cases
- **Location:** T-5 / capability migration plan
- **Description:** The capability migration plan specifies that read_file capability will be used, but does not specify the `path_allow` scope. For .factory/wave-state.yaml, the path_allow must be set to the exact path. Without an explicit path_allow, the capability either grants over-broad access or is silently rejected.
- **Proposed Fix:** Add `[hooks.capabilities.read_file] path_allow=[".factory/wave-state.yaml"]` to the registry migration specification in T-5.

### LOW

#### F-S804-P1-014: Stretch-Anchor Disclosure repeats S-8.03's text verbatim

- **Severity:** LOW
- **Category:** spec-fidelity
- **Location:** Stretch-Anchor Disclosure
- **Description:** The Stretch-Anchor Disclosure text is copied verbatim from S-8.03. It should be tailored to update-wave-state-on-merge's specific behaviors (wave-state mutation, merge detection, BC anchors).
- **Proposed Fix:** Rewrite disclosure with story-specific context.

#### F-S804-P1-015: input-hash "5015917" shared across siblings without per-story derivation

- **Severity:** LOW
- **Category:** spec-fidelity
- **Location:** Frontmatter input-hash
- **Description:** All Tier 1 siblings share the same input-hash (5015917). The convention comment suggests it should be the E-8 epic content hash, which is common to all siblings — this is correct by convention — but the comment text should explicitly state "E-8 epic content hash" for clarity.
- **Proposed Fix:** Add clarifying comment: "E-8 epic content hash — shared across Tier 1 siblings per input-hash convention."

#### F-S804-P1-016: Token Budget "200K for Sonnet" may be Opus assumption

- **Severity:** LOW
- **Category:** spec-fidelity
- **Location:** Token Budget
- **Description:** The Token Budget states "200K context window for Sonnet." Sonnet 3.5 has a 200K window; however, if the adversary runs on Opus (100K), the budget estimate is wrong. The actual model for spec review should be confirmed.
- **Proposed Fix:** Clarify which model the Token Budget targets, or provide two estimates.

### NIT

#### F-S804-P1-017: AC-004 host::emit_event unqualified path

- **Severity:** NIT
- **Category:** ambiguous-language
- **Location:** AC-004
- **Description:** AC-004 references `host::emit_event` without the fully-qualified path `vsdd_hook_sdk::host::emit_event`. Minor consistency issue with how the SDK function is cited.
- **Proposed Fix:** Use the fully-qualified path in all SDK function references.

---

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 7 |
| MEDIUM | 6 |
| LOW | 3 |
| NIT | 1 |

**Overall Assessment:** block
**Convergence:** findings remain — iterate
**Readiness:** BLOCKED — requires SDK extension PR (host::write_file) before implementation; fix burst for spec issues before pass-2

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 1 |
| **New findings** | 17 |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 17/17 = 1.00 |
| **Median severity** | HIGH |
| **Trajectory** | 17 |
| **Verdict** | FINDINGS_REMAIN |
