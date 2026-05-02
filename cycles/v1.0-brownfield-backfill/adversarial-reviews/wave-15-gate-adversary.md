---
document_type: adversarial-review
review_type: wave-gate
wave: 15
date: 2026-05-02
develop_head: "3adfe0b"
verdict: BLOCKED
finding_counts: {critical: 5, high: 6, medium: 7, low: 7}
producer: adversary
---

# W-15 Wave Gate Adversary Review

**Date:** 2026-05-02
**Develop HEAD:** 3adfe0b
**Scope:** Full wave diff — all 12 Tier 1 native WASM port stories (S-8.00, S-8.01, S-8.02, S-8.03, S-8.04, S-8.05, S-8.06, S-8.07, S-8.08, S-8.09, S-8.10, S-8.30) merged to develop
**Verdict:** BLOCKED
**Blocking findings:** CRIT-W15-001, CRIT-W15-002, CRIT-W15-003, CRIT-W15-004, CRIT-W15-005

---

## CRITICAL Findings

### CRIT-W15-001: Release Pipeline Does Not Build W-15 Hook Plugins

| Field | Value |
|-------|-------|
| **Severity** | CRITICAL |
| **File** | `.github/workflows/release.yml` lines 122-135 |
| **Impact** | Every release will produce LoadFailed for 9 of 16 native WASM plugins |
| **Blocking** | YES |

**Description:**

The release pipeline (`.github/workflows/release.yml`) does NOT build the 9 new hook plugins shipped in W-15. `hooks-registry.toml` references 16 native WASM plugins; only 3 are built by the release workflow. The 9 W-15 plugins (track-agent-start, track-agent-stop, handoff-validator, session-learning, update-wave-state-on-merge, pr-manager-completion-guard, validate-pr-review-posted, regression-gate, worktree-hooks) are absent from the build matrix.

Every downstream consumer who installs a release will encounter runtime `LoadFailed` errors on activation for all 9 missing plugins. This is a ship-blocker: the artifact produced by the release workflow does not match the hook registry manifest.

**Root cause:** release.yml was not updated when W-15 stories added new crates to the workspace. The build step appears to enumerate crates explicitly rather than using `--workspace`.

**Fix:** Update `.github/workflows/release.yml` build step to use `cargo build --release --target wasm32-wasip1 --workspace` (with `--no-default-features` for update-wave-state-on-merge which carries a `["standalone"]` default feature). Stage all 16 plugin `.wasm` outputs into `plugins/vsdd-factory/hook-plugins/`. Mirror same fix in `.github/workflows/ci.yml` to catch regressions on PR.

**New CC recommended:** CC-W15-008 BLOCKING — release pipeline must build all 16 native WASM plugins.

---

### CRIT-W15-002: handoff-validator `on_error=block` Is Dead Weight

| Field | Value |
|-------|-------|
| **Severity** | CRITICAL |
| **Files** | `crates/hook-plugins/handoff-validator/src/lib.rs`, `crates/hook-plugins/pr-manager-completion-guard/src/lib.rs`, `crates/hook-plugins/validate-pr-review-posted/src/lib.rs` |
| **Impact** | Documented blocking contract is unimplemented; three plugins use three different block-mode patterns |
| **Blocking** | YES |

**Description:**

`handoff-validator` is registered in `hooks-registry.toml` with `on_error = "block"`, and its doc comments claim "blocks if plugin panics." This claim is factually wrong: the plugin always returns `HookResult::Continue`. The dispatcher's `plugin_requests_block` gate only fires when stdout contains `{"outcome":"block"}`; it does NOT fire on crash or on `HookResult::Block`. A panicking handoff-validator silently continues execution.

Worse: three sibling plugins that all purport to implement advisory blocking chose three DIFFERENT patterns:
1. handoff-validator: `on_error=block` in registry + `HookResult::Continue` in code (advisory-only, registry field is decorative)
2. pr-manager-completion-guard: emits `{"outcome":"block"}` on stdout (actually fires dispatcher block gate)
3. validate-pr-review-posted: returns `HookResult::Block` variant (SDK variant; dispatcher ignores it)

None of these are canonical. The lack of a canonical pattern means future story authors (Tier 2/3) will pick whichever variant they happen to see first.

**Fix:** Define a canonical advisory-block-mode pattern. One of:
- Option A: All three plugins use `on_error=continue` + emit `{"outcome":"block"}` on stdout (actually blocks via dispatcher gate)
- Option B: Add a real `HookResult::Block` variant to the SDK with dispatcher support, and use it consistently

Sweep all three plugins to apply the chosen canonical pattern. Update HOST_ABI.md and SDK documentation. Record architectural decision as D-NNN.

**Related:** CC-W15-003 elevated to BLOCKING per this finding.

---

### CRIT-W15-003: WASI preopened_dir Grants Unrestricted Filesystem Access

| Field | Value |
|-------|-------|
| **Severity** | CRITICAL |
| **Files** | `crates/factory-dispatcher/src/invoke.rs` (preopened_dir grants), `crates/hook-plugins/session-learning/src/lib.rs` lines 23-25, 92 |
| **Impact** | Capability gating via `[hooks.capabilities.write_file]` is decorative; any plugin bypasses it via std::fs |
| **Blocking** | YES |

**Description:**

WASI preopened directory grants in `invoke.rs` give every plugin `DirPerms::all() | FilePerms::all()` for the preopened paths. This means any plugin can call `std::fs::write` directly — bypassing the `host::write_file` capability system entirely. The capability gating declared in `hooks-registry.toml` under `[hooks.capabilities.write_file]` is purely decorative: it only controls access to the `host::write_file` host function, not to native WASI filesystem access.

`session-learning` demonstrates the bypass concretely: `lib.rs:23-25,92` calls `std::fs::write` successfully despite having NO `write_file` capability declaration in the registry.

This breaks the security contract of BC-2.02.011 (bounded-call mandate) and BC-4.07.001/.002 (capability-gated approach). The spec promises that `write_file` capability is required for filesystem writes; the implementation provides unrestricted write access to all plugins.

**Fix:** Two options:
- Option A (v1.0 recommended): Document in HOST_ABI.md and SS-02 that capability gating governs ONLY host functions, not WASI native filesystem access. WASI preopened paths are the filesystem sandboxing boundary. Update specs to match reality.
- Option B (v1.1): Tighten WASI preopens to read-only (DirPerms::read | FilePerms::read) for most plugins; grant write preopens only to plugins with explicit `write_file` capability.

Document whichever option is chosen as an ADR. Option A is a documentation fix; Option B is a capability tightening that could break existing plugins.

**Related:** CC-W15-004 covers the canonical pattern decision.

---

### CRIT-W15-004: update-wave-state-on-merge Regex Matches False Positives

| Field | Value |
|-------|-------|
| **Severity** | CRITICAL |
| **File** | `crates/hook-plugins/update-wave-state-on-merge/src/lib.rs` line 98 |
| **Impact** | False-positive wave state updates on unrelated PR merges |
| **Blocking** | YES |

**Description:**

The PR title regex in `update-wave-state-on-merge` (lib.rs:98) is:
```
(?i)STEP_COMPLETE: step=8.*status=ok|merge|squash
```

The doc comments describe the intended match as `merged|squash.*merge`. The code as written matches:
- Any string containing the word "merge" (e.g., "fix merge conflict in lib.rs")
- Any string containing the word "squash" (e.g., "squash strategy considered")

These are false positives. A PR titled "address merge conflict from rebase" would trigger a wave state update. A PR titled "squash redundant commits" would trigger a wave state update. The bash implementation used `merged|squash.*merge` (past-tense "merged", compound "squash.*merge") to avoid these classes.

**Fix:** Update lib.rs:98 regex from `merge|squash` to `merged|squash.*merge` to match doc comments and bash semantics. Add regression tests with false-positive strings.

**New CC recommended:** CC-W15-010 HIGH — update-wave-state regex false-positive class.

---

### CRIT-W15-005: "0 Hooks via Legacy-Bash-Adapter" Claim Is Mis-Scoped

| Field | Value |
|-------|-------|
| **Severity** | CRITICAL |
| **Files** | STATE.md, STORY-INDEX.md, S-8.09 demo evidence, W-15 closure narrative |
| **Impact** | W-15 closure narrative overstates adapter retirement scope; 30+ Tier 2/3 hooks still use legacy-bash-adapter |
| **Blocking** | YES |

**Description:**

The W-15 closure narrative in STATE.md (line 333) states: "0 Tier 1 hooks routing through legacy-bash-adapter (W-15 closure achieved)." However, multiple documents drop the "Tier 1" qualifier and imply broader retirement.

In reality: only Tier 1 (12 stories) was retired in W-15. 30+ Tier 2/3 hooks STILL use legacy-bash-adapter (convergence-tracker, validate-bc-title, and others). TD-014 tracks full Tier 2/3 retirement. The adapter is far from fully retired.

Mis-scoped closure claims create false confidence. If a downstream user reads "W-15 adapter retirement complete" they may assume all hooks are native WASM, misconfigure systems, or miss the TD-014 scope.

**Fix:** Audit all W-15 closure narrative locations (STATE.md, STORY-INDEX.md, S-8.09 demo evidence, cycle-manifest, any release notes that reference adapter retirement) and add "Tier-1-only" qualifier to every claim. Create TD-014 entry explicitly naming the 30+ Tier 2/3 hooks that remain on legacy-bash-adapter.

---

## HIGH Findings

### HIGH-W15-001: Plugin Version Drift (1.0.0-rc.1 vs 0.0.1)

| Field | Value |
|-------|-------|
| **Severity** | HIGH |
| **Files** | Multiple `crates/hook-plugins/*/Cargo.toml` |
| **Blocking** | NO |

**Description:**

W-15 stories shipped with inconsistent plugin versions. Some plugins (e.g., session-learning, update-wave-state-on-merge) carry `version = "1.0.0-rc.1"` while others (e.g., track-agent-stop, handoff-validator) carry `version = "0.0.1"`. All plugins shipped in the same wave and should carry a consistent version tag matching the release cadence.

**Fix:** Align all 9 W-15 native plugin crates to `version = "1.0.0-rc.1"` (or whatever the canonical version is for the wave-15 release). Run `cargo check --workspace` to verify no version compatibility issues.

---

### HIGH-W15-002: Whitespace Counting Divergence (chars vs bytes)

| Field | Value |
|-------|-------|
| **Severity** | HIGH |
| **Files** | `crates/hook-plugins/handoff-validator/src/lib.rs`, `crates/hook-plugins/track-agent-stop/src/lib.rs` |
| **Blocking** | NO |

**Description:**

`handoff-validator` uses `.chars().filter(|c| c.is_whitespace()).count()` (Unicode-aware, counts codepoints). `track-agent-stop` uses `.bytes().filter(|b| *b == b' ' || *b == b'\n' || *b == b'\t').count()` (ASCII-only byte counting). The two plugins will produce different counts for the same input containing Unicode whitespace (e.g., non-breaking space U+00A0, em space U+2003).

For inputs from the Claude Code hook system in practice, this divergence is unlikely to matter (all hook payloads are ASCII-safe JSON). However, the divergence creates a maintenance trap: if a future story extends one plugin to handle Unicode payloads, the sibling will silently disagree.

**Fix:** Pick one canonical approach (recommend `.chars().filter(|c| c.is_whitespace())` for Unicode correctness) and align both plugins. Add a doc comment noting the chosen standard.

**New CC recommended:** CC-W15-011 HIGH — chars-vs-bytes whitespace counting must align.

---

### HIGH-W15-003: Block-Mode Pattern Divergence (Elevated from CRIT-W15-002)

| Field | Value |
|-------|-------|
| **Severity** | HIGH (elevated to BLOCKING per CRIT-W15-002) |
| **Blocking** | YES (via CRIT-W15-002) |

**Description:**

Three plugins implement three different advisory block-mode patterns. See CRIT-W15-002 for full detail. This finding is a cross-reference to the CRITICAL finding — the pattern divergence is the symptom; the dead-weight `on_error=block` is the root cause.

---

### HIGH-W15-004: update-wave-state-on-merge `default = ["standalone"]` Should Be Inverted

| Field | Value |
|-------|-------|
| **Severity** | HIGH |
| **File** | `crates/hook-plugins/update-wave-state-on-merge/Cargo.toml` |
| **Blocking** | NO (separate CC-W15-009 BLOCKING recommended) |

**Description:**

`update-wave-state-on-merge` declares `default = ["standalone"]` in its feature flags. The "standalone" feature enables direct file I/O without going through the host::write_file path. This means the plugin defaults to a less-sandboxed execution mode. Fail-safe default would be `default = []` (no standalone feature; use host::write_file capability-gated path as the default).

With `default = ["standalone"]`, anyone who builds the plugin without `--no-default-features` gets the standalone mode silently. The release.yml fix (CRIT-W15-001) must use `--no-default-features` for this plugin — easy to miss in future maintenance.

**Fix:** Invert: `default = []`. The "standalone" feature should require explicit opt-in (`--features standalone`). This makes the safer capability-gated path the default.

**New CC recommended:** CC-W15-009 BLOCKING — standalone default invert.

---

### HIGH-W15-005: Missing env_allow Declarations on Native Plugins

| Field | Value |
|-------|-------|
| **Severity** | HIGH |
| **Files** | `hooks-registry.toml` multiple `[[hooks]]` entries |
| **Blocking** | NO |

**Description:**

Several native WASM plugins read environment variables (e.g., `CLAUDE_PROJECT_DIR`, `FACTORY_STATE_FILE`) but do not declare them in `env_allow` in `hooks-registry.toml`. The dispatcher's WASI engine grants access to environment variables in `env_allow`; without a declaration, plugins that read env vars may silently get empty strings (depending on WASI engine configuration) or may get values if the dispatcher passes through all env vars by default.

If the dispatcher passes env through by default, the missing declarations are a documentation gap. If the dispatcher only passes declared vars, missing declarations are functional bugs.

**Fix:** Audit all 9 W-15 plugins for environment variable reads. Add `env_allow = [...]` declarations to `hooks-registry.toml` for each plugin that reads env vars. Update HOST_ABI.md to clarify whether undeclared env vars are passed through or filtered.

---

### HIGH-W15-006: serde_yaml Deprecation Tech Debt

| Field | Value |
|-------|-------|
| **Severity** | HIGH |
| **Files** | Multiple crates using `serde_yaml` |
| **Blocking** | NO |

**Description:**

`serde_yaml` (dtolnay/serde-yaml) was archived and deprecated in late 2024. Multiple crates in the workspace pull it as a dependency. The crate still works but receives no security or bug fixes. For a v1.0 release targeting production use, depending on an archived crate is a liability.

**Fix:** Evaluate replacement with `serde_yml` (community fork) or `serde_yaml_ng`, or migrate YAML parsing to a maintained alternative (e.g., `marked_yaml`). Register as a tech debt item (TD-NNN) with a v1.1 target. Not a blocker for rc.3 but should not reach GA without a migration plan.

---

## MEDIUM Findings

### MED-W15-001: BC-2.02.012 EC-004 Divergence — `output` Field Absent in HookPayload

| Field | Value |
|-------|-------|
| **Severity** | MEDIUM |

**Description:**

BC-2.02.012 EC-004 references an `output` field in HookPayload for the 3-stage bash chain pattern. The actual `HookPayload` struct (post S-8.30 serde flatten) does not contain an `output` field at the top level. The EC-004 example is stale relative to the implemented struct.

**Fix:** Update BC-2.02.012 EC-004 to reflect the actual HookPayload schema. If the `output` field was intentionally removed, document the removal rationale in the BC.

---

### MED-W15-002: regression-gate write_file No max_bytes_per_call Cap in Registry

| Field | Value |
|-------|-------|
| **Severity** | MEDIUM |

**Description:**

The `regression-gate` plugin uses `host::write_file` and is declared in `hooks-registry.toml` with a `write_file` capability. However, the registry entry does not include a `max_bytes_per_call` cap. BC-2.02.011 mandates bounded calls. The plugin code controls the size of what it writes, but there is no registry-level enforcement.

**Fix:** Add `max_bytes_per_call` to the regression-gate `write_file` capability entry in `hooks-registry.toml`. Set to a reasonable upper bound (e.g., 65536 bytes for a gate report).

---

### MED-W15-003: BC Anchor Verification Not Performed for W-15

| Field | Value |
|-------|-------|
| **Severity** | MEDIUM |

**Description:**

The per-story-delivery process includes a step to verify BC anchors (ensure BCs referenced in story files exist in the BC index and vice versa). There is no evidence in the W-15 delivery record that BC anchor verification was run for any of the 12 W-15 stories. Given the volume (12 stories, ~40+ BC references), silent anchor drift is likely.

**Fix:** Run BC anchor verification tool for all 12 W-15 stories before wave gate PASS. Document results as part of the gate report.

---

### MED-W15-004: "serde flatten" Commit Message Inaccurate

| Field | Value |
|-------|-------|
| **Severity** | MEDIUM |

**Description:**

The S-8.30 commit message references "serde flatten" as the key change. Inspection of the actual `payload.rs` diff shows no `#[serde(flatten)]` attribute was added. The commit message is inaccurate and will mislead future bisect/blame readers.

**Fix:** Note in the wave-15 delivery record that the commit message is inaccurate. Consider a follow-up commit that updates the commit message (if the team uses squash-and-rebase discipline) or adds a correction note in the changelog.

---

### MED-W15-005: host::write_file Error Code -99 Undocumented

| Field | Value |
|-------|-------|
| **Severity** | MEDIUM |

**Description:**

The `host::write_file` host function returns error code `-99` for certain failure modes (e.g., path not allowed). This error code is not documented in HOST_ABI.md. Plugin authors who inspect the return value will encounter an undocumented magic number.

**Fix:** Document error code `-99` (and all other host function error codes) in HOST_ABI.md under the `host::write_file` section.

---

### MED-W15-006: Doc-Test Fragility in SDK Crate

| Field | Value |
|-------|-------|
| **Severity** | MEDIUM |

**Description:**

Several doc-tests in the `vsdd-hook-sdk` crate reference internal implementation details (struct field names, variant names) that could change with future refactors. Doc-tests are run as part of `cargo test` but are not tracked in the 957-test count. If doc-tests start failing silently (e.g., due to `#[doc(hidden)]` attributes), the gate count understates breakage.

**Fix:** Audit SDK doc-tests for fragility. Replace implementation-detail references with stable public API surface. Ensure doc-tests are included in the CI test count.

---

### MED-W15-007: pr-manager-completion-guard Hint Table Strict-Parity Invariant Unenforced in CI

| Field | Value |
|-------|-------|
| **Severity** | MEDIUM |

**Description:**

`pr-manager-completion-guard` contains a hint table that must stay in strict parity with the hint strings emitted by the dispatcher. There is no CI check enforcing this invariant. If a developer adds a new hint to the dispatcher without updating the plugin, the hint table silently goes stale.

**Fix:** Add a CI test (bats or Rust integration test) that verifies hint table parity. Alternatively, generate the hint table from a shared constant so drift is impossible.

---

## LOW Findings / Observations

### LOW-W15-001: STATE.md Size Budget Violated

**Description:** STATE.md exceeds 200-line budget at 360 lines (>92K tokens per token counter). The state-manager SIZE_GATE hook should have fired. Investigate why the hook did not block the last several commits. Prune historical decisions to `decision-log.md` or `session-checkpoints.md` as appropriate.

---

### LOW-W15-002: Duplicate serde_json Dev-Dependencies

**Description:** Multiple plugin crates declare `serde_json` as both a regular dependency and a dev-dependency. The dev-dependency declaration is redundant (regular deps are available in tests). Clean up Cargo.toml files.

---

### LOW-W15-003: track-agent-stop Regex Compiled Per-Call

**Description:** `track-agent-stop` compiles its regex on every invocation rather than using `OnceLock` or `lazy_static`. For a hot-path plugin invoked on every agent stop, this is a measurable overhead. Use `OnceLock<Regex>` for all regex constants.

---

### LOW-W15-004: No Perf Comparison vs Bash for 9 Native Plugins

**Description:** W-15 ships 9 native WASM ports but no performance comparison data (latency, memory) vs the retired bash implementations. For the v1.0 release narrative, at minimum a single data point (e.g., "track-agent-start: 8ms WASM vs 45ms bash") would substantiate the migration value.

---

### LOW-W15-005: Capability env_allow Inconsistency

**Description:** Some plugins declare `env_allow` for env vars they don't read; others omit `env_allow` for vars they do read (see HIGH-W15-005). The inconsistency is a documentation and review burden.

---

### LOW-W15-006: Sibling Test-Runner Naming Differences (kebab vs snake)

**Description:** Integration test files use both kebab-case (`test-wave-state.rs`) and snake_case (`test_wave_state.rs`) naming. Rust convention is snake_case for test files. Standardize to snake_case across all 9 plugin crates.

---

### LOW-W15-007: Rust Edition 2024 vs 1.95 Minimum

**Description:** Some plugin crates declare `edition = "2024"` in Cargo.toml while the workspace minimum Rust version (`rust-version` field or CI matrix) appears to be 1.95. Rust edition 2024 requires Rust 1.85+. If the minimum is actually 1.95 this is fine, but if CI ever tests against older toolchains, builds will fail. Verify and document the minimum supported Rust version.

---

## New Compensating Controls Recommended

| CC ID | Severity | Description |
|-------|----------|-------------|
| CC-W15-008 | BLOCKING | release.yml must build all 16 native WASM plugins; currently builds only 3 |
| CC-W15-009 | BLOCKING | update-wave-state-on-merge `default = ["standalone"]` must be inverted to `default = []` |
| CC-W15-010 | HIGH | update-wave-state-on-merge regex `merge|squash` → `merged|squash.*merge` (false-positive class) |
| CC-W15-011 | HIGH | handoff-validator (chars + Unicode) vs track-agent-stop (bytes + ASCII) whitespace counting must align |

---

## Verdict Summary

**VERDICT: BLOCKED**

Five CRITICAL findings prevent W-15 wave gate PASS:

1. **CRIT-W15-001**: Release pipeline does not build 9 of 16 native WASM plugins — every release is broken at activation
2. **CRIT-W15-002**: `on_error=block` is dead weight; three plugins use three different block-mode patterns; documented contract unimplemented
3. **CRIT-W15-003**: WASI preopened_dir grants unrestricted filesystem access; capability gating is decorative
4. **CRIT-W15-004**: update-wave-state-on-merge regex matches "merge conflict" and "squash strategy" — false positive wave state updates
5. **CRIT-W15-005**: W-15 closure narrative overstates adapter retirement scope; 30+ Tier 2/3 hooks still on legacy-bash-adapter

**Required for re-gate:** All 5 CRITICAL + 1 HIGH security finding (SEC-003) must be resolved. Re-run wave gate (implementer + adversary + security-reviewer) after fix-burst. Verdict must be CONVERGED before cutting rc.3.
