# RC24 Foundational-Impact Analysis — `v1.0.0-rc.23..develop`

- **Range:** `v1.0.0-rc.23..develop` (51 commits; HEAD `27c56c01`)
- **Method:** actual-diff analysis (`git diff`/`git show` on foundational code paths), not commit-message reading. Read-only; nothing modified.
- **Diffstat headline:** 252 files, +43,225 / −1,580. The overwhelming majority of additions are **new test files, new WASM plugin crates, test fixtures, and demo-evidence artifacts** — not edits to existing core code.

## VERDICT

**FOUNDATIONS PRESERVED (all additive/tuning).** No foundational ADR contract, host ABI, dispatch algorithm, error taxonomy, or registry schema was rewritten. Every foundational touch is either an additive extension (new fields with safe defaults, new plugins, new CI gates) or a tuning-constant bump. The S-21.10 "schema only, no enforcement change" claim for ADR-039 is **independently verified true**.

## Foundational-Piece Classification Table

| Foundational piece | Classification | Evidence |
|---|---|---|
| 1. Hook ABI / host<->plugin calling convention | **UNCHANGED** | `HOST_ABI_VERSION: u32 = 1` unchanged (`crates/factory-dispatcher/src/lib.rs:72`). `git diff --stat` shows **zero changes** to `crates/factory-dispatcher/src/host/` and **zero changes** to `crates/hook-sdk/` (incl. `HOST_ABI.md`). No entrypoint/wire-format edits. |
| 2. Dispatcher core control flow | **ADDITIVE-EXTENSION** | `invoke.rs`/`main.rs`/`executor.rs`: fuel cap const 10M→20M; additive `fuel_cap` telemetry field on `PluginResult::Timeout`; fuel-vs-epoch `block_reason` string split (#774, `62fbcf1a`); `.factory/logs` mount-gate (#738, `651db073`). `plugin_fail_closed(result, on_error: OnError)` signature **unchanged** (`executor.rs:638`). Tier semantics, block_intent/exit-code decision logic, fuel/epoch enforcement code all unchanged. |
| 3. `hooks-registry.toml` schema | **ADDITIVE-EXTENSION** | Only 3 new `[[hooks]]` rows appended (count 73→76). No new required fields on existing rows; no changed trigger-event/tier/on-error meaning. All 3 new rows `on_error = "continue"` (fail-open). |
| 4. Error taxonomy / BC-3.08.001 domain events | **UNCHANGED** | Event constants intact and unedited: `PLUGIN_LOADED / LOAD_FAILED / INVOKED / COMPLETED / TIMEOUT / CRASHED / ABANDONED` (`internal_log.rs`). New `FailurePolicy` enum is a registry type, not an error variant. `fuel_cap` added to `plugin.timeout` event payload (additive field only). |
| 5a. ADR-039 (failure_policy / fail-closed) | **ADDITIVE-EXTENSION (schema-only claim VERIFIED)** | `27c56c01` (#780). See per-ADR section. |
| 5b. ADR-042 (fuel cap) | **ADDITIVE (tuning)** | `62fbcf1a` (#774). `DEFAULT_FUEL_CAP = 20_000_000` single-source const; no semantics change. |
| 5c. ADR-032 (timestamp enforcement) | **ADDITIVE-EXTENSION** | `ae263781` (#742). New fail-open advisory plugin + GitContext 4→7 fields; existing refresh hook registration unchanged. See per-ADR section. |
| 5d. ADR-041 / ADR-026 / ADR-028 (sentinel / wave-boundary / precompact) | **UNCHANGED (no code touched)** | `host/` and precompact/wave-boundary paths show no diff in range. |
| 5e. POLICY 15 attestation gate | **ADDITIVE (CI-only)** | `19cb57e6`/`84a441a0`/`a6a15e1d` (#777/#778/#779). New `crates/policy15-attestation-gate` native binary; **0 references in `hooks-registry.toml`** — CI-only required check, not a runtime dispatch hook. |
| 6. ADR files / foundational spec docs in develop tree | **UNCHANGED** | No `adr*/decision*/architecture*` files under develop tree changed (all `.factory/specs/*` paths in the diff are **test fixtures** under `crates/*/tests/` and `docs/demo-evidence/`). `docs/dispatch-package-authoring.md` unchanged. |
| 7. Public surfaces (skills / slash-commands / agent tool profiles / dispatch-package contract) | **ADDITIVE** | Agent/skill `.md` edits are guidance/prompt content additions; no skill signature, slash-command, or tool-profile contract change. `docs/dispatch-package-authoring.md` (TD #74) unchanged. |

## Per-ADR Judgments

### ADR-039 — validator failure_policy (S-21.10, #780, `27c56c01`) — ADDITIVE-EXTENSION; "schema only" VERIFIED

The S-21.10 claim ("schema only, no enforcement change") is **true**. Evidence from `crates/factory-dispatcher/src/registry.rs`:

- New `FailurePolicy` enum with `#[derive(..., Default)]` and `#[default]` on the **`FailOpen`** variant — the safe, backward-compatible default.
- New `RegistryEntry.failure_policy` field carries `#[serde(default)]`, so registry rows that predate the field parse to `FailOpen`. Existing rows in `hooks-registry.toml` were **not** given a `failure_policy` key (no behavior change for shipped plugins).
- `#[serde(rename_all = "kebab-case")]` deliberately rejects the snake-case `"fail_closed"` form (EC-003 guard) — no silent-accept bypass.
- **Enforcement path untouched:** `plugin_fail_closed(result, on_error: OnError)` still takes `OnError` only and never reads `failure_policy`. A repo-wide search finds `failure_policy` referenced only in `registry.rs` (definition + tests) and in **test-only** helper structs in `partition.rs:138` and `executor.rs:964`. It is read nowhere in production dispatch logic. Enforcement flip is explicitly deferred to S-21.11.

Judgment: additive schema field with fail-open default; zero runtime behavior change.

### ADR-042 — fuel cap (S-21.x, #774, `62fbcf1a`) — ADDITIVE (tuning)

- `pub const DEFAULT_FUEL_CAP: u64 = 20_000_000` introduced in `invoke.rs` as single source of truth; both `InvokeLimits::default()` and `RegistryDefaults::default()` now reference it (previously literal `10_000_000`).
- No change to fuel **enforcement semantics**: `build_engine`, `set_fuel`, and the `Trap::OutOfFuel` → `PluginResult::Timeout{cause: Fuel}` classification are unchanged.
- Companion additive changes: `fuel_cap` field on `PluginResult::Timeout` (telemetry) and a distinct fuel `block_reason` string (`"fail-closed: FUEL_EXHAUSTED: fuel cap of N units exhausted..."`). The **epoch** arm string `"fail-closed: plugin timed out"` is intentionally unchanged (preserves operator runbook matches). This affects the reason *text*, not the block/no-block decision.

Judgment: constant/config tuning + additive telemetry. Not a semantics change.

### ADR-032 — timestamp enforcement (#742, `ae263781`) — ADDITIVE-EXTENSION

- New crate `crates/hook-plugins/verify-state-timestamp-advisory` (PostToolUse, `^Bash$`, priority 159, `on_error = "continue"` — advisory, never blocks).
- `GitContext` extended from 4 to 7 fields (`head_state_timestamp`, `head_parent_state_timestamp`, `state_md_in_commit`) — additive, all using the existing empty-string sentinel convention so the advisory plugin reads timestamps from payload without `exec_subprocess` in the WASM sandbox.
- `verify-state-timestamp-refresh/src/lib.rs` internal detection logic was substantially reworked (~1,136 lines, "payload-targeted" + ADR-032 D4 frontmatter placement). Its **registry registration is unchanged** (PreToolUse, `^(Edit|Write|MultiEdit)$`, `on_error = "continue"` advisory). This is a refinement of an existing fail-open gate's detection logic, not a new blocking contract. **Flag for architect review** (see below).

Judgment: additive advisory plugin + additive payload fields; existing hook stays fail-open/advisory.

### POLICY 15 — attestation-location gate (#777/#778/#779) — ADDITIVE (CI-only)

- New native binary crate `crates/policy15-attestation-gate` (`19cb57e6`), wired as a **CI required-check job** `policy-15-attestation-location` in `.github/workflows/ci.yml` (`84a441a0`), plus an inert-skip fix for empty ranges → `SkippedEmptyRange` exit 0 (`a6a15e1d`).
- **Not** registered in `hooks-registry.toml` (0 references) — it does not participate in the runtime dispatch chain, so it cannot alter any existing gate's runtime behavior. Wiring it as required-check adds a new CI gate only.

Judgment: new additive gate; no change to existing gate behavior.

### Dependency security bumps (S-21.12 #781 `97fb07fa`; #770 `700b4dd3`)

- `wasmtime`/`wasmtime-wasi` 44.0 → **46.0.2** (major bump) + `wasmtime-wasi` 44.0.1→44.0.3 patch; `anyhow` 1.0→1.0.104; `httpmock` 0.7→0.8.3; added a `cargo-deny` CI gate. Motivation: clear 5 RUSTSEC advisories (P0 security).
- Despite the wasmtime **major** version jump, the host-integration code (`host/` dir, HOST_ABI, fuel/epoch engine config) required **no source changes** — confirming the upgrade did not alter the host contract. Correctness rests on the green cargo + bats suites.

## Items Deserving Human / Architect Attention Before Release

1. **wasmtime major bump 44 → 46.0.2 (`97fb07fa`).** A major WASM-runtime version jump with no host-code changes is expected-good but is the single highest-leverage risk surface in the range. Confirm the full `cargo test --workspace --all-targets` + bats suite is green on `develop` HEAD before cutting rc.24 (CI reports green per PR #781). No action beyond that gate.
2. **`verify-state-timestamp-refresh` ~1,136-line internal rework (ADR-032, `ae263781`).** Registration stays fail-open/advisory, so it cannot newly wedge sessions, but the *detection* logic changed materially ("payload-targeted", D4 frontmatter placement). Worth a spot-confirm by the architect that the new detection matches ADR-032's intended AC-021 semantics — this is the only existing-hook internal behavior that changed substantively (everything else is net-new plugins or additive fields).
3. **ADR-039 Phase-1/Phase-2 split is load-bearing.** The `failure_policy` schema now ships fail-open with zero enforcement. That is correct for this release, but it means an operator setting `failure_policy = "fail-closed"` in the registry today gets **no effect** until S-21.11. Ensure release notes / CHANGELOG state that `failure_policy` is parsed-but-inert in rc.24 so operators don't assume it's enforced.
4. **20M fuel cap is a tuning bump, not a fix for large-artifact exhaustion.** Per the ADR-042 rationale in-code and CLAUDE.md, 20M still does not eliminate exhaustion on the largest cycle artifacts; size budgets/compaction remain the remedy. No release blocker; just don't market 20M as "solved."

## Confidence

HIGH. Judgments are grounded in the actual diff of the foundational source files (`invoke.rs`, `registry.rs`, `executor.rs`, `main.rs`, `internal_log.rs`, `lib.rs`, `hooks-registry.toml`, `Cargo.toml`, `.github/workflows/ci.yml`) plus repo-wide reference checks confirming `failure_policy` is unreferenced in enforcement code and `HOST_ABI_VERSION` is unchanged.
