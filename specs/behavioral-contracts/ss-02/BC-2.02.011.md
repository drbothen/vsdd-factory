---
document_type: behavioral-contract
level: L3
version: "1.8"
status: active
producer: product-owner
timestamp: 2026-05-01T00:00:00Z
last_amended: "2026-07-17 (v1.8) — RETROACTIVE-POL-14 (state-manager, D-852): status ready→active; POL-14 applied retroactively at S-19.03 PR #611 merge 2026-07-13T15:54:21Z (D-834); S-19.03 behavioral_contracts: [BC-2.07.001, BC-2.02.011]; corrected in same D-852 burst per F-004; input-hash e650b4b→a7ae268 (inputs drifted). [Prior: 2026-07-10 (v1.7) — E-19 pass-45 F-P45-003 fix burst (product-owner): frontmatter modified[] array re-sorted version-monotonic (was v1.3→v1.4→v1.6→v1.5; POLICY 14 leg-3 violation: last entry v1.5 vs Changelog top v1.6); no body content change. [Prior: (v1.6) — orchestrator pre-pass-43 consistency sweep (product-owner): §Verification Properties VP-097 row added (F-P43-class missing-row; kani-proof; S-19.03 AC-001; co-anchored with BC-2.07.001 §Invariant 1; this BC's EC-001); §VP Anchors TBD placeholder replaced with VP-097 bullet.]]"
phase: 1
inputs:
  - .factory/stories/S-8.10-sdk-extension-write-file.md
  - crates/hook-sdk/src/host.rs
  - crates/factory-dispatcher/src/host/read_file.rs
input-hash: "a2eb900"
traces_to: .factory/specs/domain-spec/capabilities.md
origin: brownfield
extracted_from: ".factory/stories/S-8.10-sdk-extension-write-file.md"
subsystem: "SS-02"
capability: "CAP-022"
lifecycle_status: active
introduced: v1.1
modified:
  - "v1.3 (2026-06-20): S-18.04a-prereq / ADR-028 §Decision 8 — corrected Invariant 3 path-resolution base plugin_root → ctx.cwd (CLAUDE_PROJECT_DIR); corrected Postcondition 5 to match; the prior plugin_root claim was stale since S-8.07's read_file fix"
  - "v1.4 (2026-07-06): E-19 pass-2 F-P2-004 fix burst — Architecture Anchors + Traceability Architecture Module extended with path_util.rs (shared path allowlist-resolution helper extracted at S-19.03)"
  - "v1.5 (2026-07-08): E-19 pass-24 F-P24-001 fix burst — §Traceability Stories + §Story Anchor extended with S-19.03 (path_util.rs extraction of resolve_path_for_allowlist + write_file.rs two-step decomposed pattern adoption; EC-001 traversal-defense anchored by VP-097); §Refactoring Notes annotated as carried out by S-19.03; closes bidirectional-parity gap (story declared BC, BC did not acknowledge story)"
  - "v1.6 (2026-07-09): orchestrator pre-pass-43 consistency sweep — §Verification Properties VP-097 row added (missing-row; kani-proof; S-19.03 AC-001; co-anchored with BC-2.07.001 §Invariant 1; this BC's EC-001); §VP Anchors TBD placeholder replaced with VP-097 bullet"
  - "v1.7 (2026-07-10): E-19 pass-45 F-P45-003 fix burst (product-owner) — frontmatter modified[] array re-sorted version-monotonic (was v1.3→v1.4→v1.6→v1.5; POLICY 14 leg-3 violation: last entry v1.5 vs Changelog top v1.6); no body content change"
  - "v1.8 (2026-07-17): RETROACTIVE-POL-14 (state-manager, D-852) — status ready→active; POL-14 applied retroactively at S-19.03 PR #611 merge 2026-07-13T15:54:21Z (D-834); S-19.03 behavioral_contracts: [BC-2.07.001, BC-2.02.011]; corrected in same D-852 burst per F-004; input-hash e650b4b→a7ae268"
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# BC-2.02.011: host::write_file: bounded write capability with allowlist enforcement

## Description

`host::write_file` writes a guest-supplied byte slice to the filesystem through the dispatcher's bounded host function. The call is gated by a per-plugin `path_allow` allowlist (deny-by-default), bounded by a mandatory `max_bytes` cap, and bounded by a mandatory `timeout_ms` budget. This is the write-side symmetric counterpart to `host::read_file` (BC-2.02.002) and follows the same additive ABI extension pattern (D-6 Option A; HOST_ABI_VERSION stays at 1).

## Preconditions

1. A plugin author calls `host::write_file(path: &str, contents: &[u8], max_bytes: u32, timeout_ms: u32)`.
2. The dispatcher context (`ctx`) carries the plugin's declared capabilities, including an optional `capabilities.write_file.path_allow` allowlist.
3. `path` is a valid UTF-8 string (non-empty); `max_bytes` and `timeout_ms` are both mandatory u32 parameters — the API permits no opt-out of either bound.

## Postconditions

1. **Allowlist denial:** If `path_allowed(path, ctx.path_allow)` is false (path not in allowlist or no `write_file` capability block present) → returns `Err(HostError::CapabilityDenied)`; dispatcher emits `codes::CAPABILITY_DENIED (-1)` at the FFI boundary.
2. **Byte cap exceeded:** If `contents.len() > max_bytes` → returns `Err(HostError::OutputTooLarge)`; dispatcher emits `codes::OUTPUT_TOO_LARGE (-3)` at the FFI boundary; no bytes are written to disk.
3. **Successful write:** If `path_allowed` is true AND `contents.len() <= max_bytes` AND the write completes within `timeout_ms` → returns `Ok(())`; the full byte slice is durably written to `path`.
4. **Timeout:** If the write exceeds `timeout_ms` → returns `Err(HostError::Timeout)`; dispatcher emits `codes::TIMEOUT (-2)` at the FFI boundary.
5. **Path resolution error or no parent directory:** If the resolved path's parent directory does not exist, or if a relative path cannot be resolved within `ctx.cwd` (CLAUDE_PROJECT_DIR) → returns `Err(HostError::Other(-99))`; dispatcher emits `codes::INTERNAL_ERROR (-99)` at the FFI boundary. Mirrors the `ReadErr::Other → INTERNAL_ERROR` pattern in `read_file.rs::prepare`.
6. **ABI catalog update:** After this story merges, `crates/hook-sdk/HOST_ABI.md` lists `write_file` in the host export catalog with: signature, input-pointer protocol, timeout semantics, byte-cap semantics, and safety policy (AC-8).

## Invariants

1. **HOST_ABI_VERSION = 1 unchanged:** Adding `write_file` is an additive ABI extension per D-6 Option A. `crates/hook-sdk/src/lib.rs::HOST_ABI_VERSION` and `crates/factory-dispatcher/src/lib.rs::HOST_ABI_VERSION` both remain `pub const HOST_ABI_VERSION: u32 = 1;`. No bump is permitted in v1.x per D-6 Option B prohibition. Verified cross-crate by BC-2.01.003.
2. **`max_bytes` is mandatory — no opt-out:** `host::write_file` is a bounded host call per BC-2.02.002. Story-writer MUST NOT introduce conditional language that allows eliding the `max_bytes` parameter. The API signature enforces this at the type level.
3. **Path resolution mirrors `resolve_for_read`:** Absolute paths pass through as-is; relative paths are joined with `ctx.cwd` (CLAUDE_PROJECT_DIR), mirroring `read_file.rs::resolve_for_read` (both cwd-rooted as of S-8.07) and matching the production `invoke.rs` write_file path. (Corrected from the stale plugin_root claim per S-18.04a-prereq / ADR-028 §Decision 8.)
4. **FFI input-pointer protocol:** The SDK wrapper passes `(path_ptr, path_len, contents_ptr, contents_len, max_bytes, timeout_ms)` to the dispatcher's `vsdd::write_file` host import. The dispatcher reads guest memory via `read_wasm_bytes` (not an output-pointer protocol like `read_file`). This protocol difference MUST be documented in AC-8 and respected in any FFI declaration.
5. **Error codes are stable — no new codes:** `write_file` returns from the existing set `{0: success, -1: CapabilityDenied, -2: Timeout, -3: OutputTooLarge, -4: InvalidArgument, -99: InternalError}`. No new negative codes are introduced by this story (per Architecture Compliance Rule 4 in S-8.10 v1.1).
6. **Deny-by-default capability model:** Absence of a `capabilities.write_file` block in the plugin registry entry produces `CAPABILITY_DENIED (-1)`. This matches the capability-absent guard in `read_file.rs::prepare`.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Path traversal attempt (e.g. `../../../etc/passwd`) | `CAPABILITY_DENIED (-1)`; `internal.capability_denied` telemetry event emitted |
| EC-002 | Path is not in `write_file.path_allow` list | `CAPABILITY_DENIED (-1)`; same denial path as EC-001 |
| EC-003 | No `write_file` capability block present in plugin registry entry | `CAPABILITY_DENIED (-1)`; allowlist check fails at capability-absent guard |
| EC-004 | `timeout_ms = 0` | Accepted for ABI stability; epoch interruption policy handles actual enforcement (mirrors the `let _ = timeout_ms` comment in `read_file.rs::register`) |
| EC-005 | `contents = &[]` (empty slice) | Write succeeds (0); creates or truncates the file with zero bytes |
| EC-006 | Parent directory of destination path does not exist | `io::Error` propagated → `codes::INTERNAL_ERROR (-99)` |
| EC-007 | Plugin compiled against SDK 0.1.x (no `write_file` import) loaded against dispatcher that exports `write_file` | Plugin loads normally; wasmtime ignores unimported host exports; no error — confirmed ABI-safe per D-6 Option A |
| EC-008 | UTF-8 decoding failure on path bytes | `codes::INTERNAL_ERROR (-99)` or `codes::INVALID_ARGUMENT (-4)` per dispatcher implementation |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `write_file("/allowed/path", b"hello", 1024, 5000)` where path is in `path_allow` | `Ok(())`, file contains `b"hello"` | happy-path |
| `write_file("/denied/path", b"hello", 1024, 5000)` where path not in `path_allow` | `Err(HostError::CapabilityDenied)` | error |
| `write_file("/allowed/path", b"data", 3, 5000)` where `contents.len()=4 > max_bytes=3` | `Err(HostError::OutputTooLarge)` | error |
| `write_file("/allowed/path", b"", 1024, 5000)` (empty contents) | `Ok(())`, file created/truncated to zero bytes | edge-case |
| `write_file("/no-parent/sub/path", b"x", 1024, 5000)` where parent dir absent | `Err(HostError::Other(-99))` | edge-case |
| SDK non-wasm stub: `write_file(path, contents, max, timeout)` | `Err(HostError::CapabilityDenied)` (stub returns `-1` → `HostError::from_code(-1)`) | happy-path (unit) |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-097 | path_util::resolve_path_for_allowlist traversal defense — .. sequences cannot resolve outside allowlist prefixes | kani-proof (S-19.03 AC-001; co-anchored with BC-2.07.001; this BC's EC-001) |
| (TBD — to be assigned in Phase 1.6b) | `path_allowed` is a pure deterministic function testable in isolation | Unit test (pure-core) |
| (TBD) | `max_bytes` enforcement occurs before any filesystem operation | Unit test with assertion on call count |
| (TBD) | HOST_ABI_VERSION = 1 in both crates after story merge | grep assertion (AC-3 in S-8.10) |

## Related BCs

- **BC-2.02.002** (composes with) — mandates bounded host calls; `write_file` extends the bounded-call family by adding a write capability
- **BC-2.01.003** (depends on) — HOST_ABI_VERSION = 1 invariant; this BC must not violate it
- **BC-2.02.003** (depends on) — HostError code mapping; `write_file` reuses the existing error code set without additions

## Architecture Anchors

- `crates/hook-sdk/src/host.rs` (new `pub fn write_file` after the existing `pub fn read_file` declaration in `host.rs`, SDK wrapper)
- `crates/hook-sdk/src/ffi.rs` (new `write_file` extern in `#[cfg(target_arch = "wasm32")]` block + `host_stubs`)
- `crates/factory-dispatcher/src/host/write_file.rs` (new file, dispatcher binding, input-pointer protocol)
- `crates/factory-dispatcher/src/host/path_util.rs` (shared path allowlist-resolution helper; extracted from write_file.rs at S-19.03; implements the rejoin algorithm for resolving both existing and absent file paths against the declared path_allow list; imported by both write_file.rs and read_file.rs)
- `crates/factory-dispatcher/src/host/mod.rs` (registration call in `setup_linker`, `allow_write` test helper)
- `crates/factory-dispatcher/src/registry.rs` (`WriteFileCaps` struct, `write_file: Option<WriteFileCaps>` field)
- `crates/hook-sdk/HOST_ABI.md` (ABI catalog documentation, AC-8)

## Story Anchor

S-8.10 — "SDK extension: host::write_file (D-6 Option A unblocker)" resolves OQ-1 in that story. This BC supersedes the `[OQ-1: BC-2.02.011 pending]` placeholder in S-8.10 v1.1 BC-trace table.

S-19.03 — "warn-pending-wave-gate: file-not-found path_util extraction" adopts this BC's EC-001 traversal-defense (resolve_path_for_allowlist extraction into path_util.rs) and applies the write_file.rs two-step decomposed prepare() pattern. S-19.03 v1.12 declares this BC in its `behavioral_contracts: [BC-2.07.001, BC-2.02.011]` frontmatter and cites EC-001 in AC-001 BC Trace.

## VP Anchors

- VP-097 — path_util::resolve_path_for_allowlist Traversal Defense — .. Sequences Cannot Resolve Outside Allowlist Prefixes (kani-proof; S-19.03; co-anchored with BC-2.07.001 §Invariant 1; this BC's EC-001)

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-022 |
| Capability Anchor Justification | CAP-022 ("Port hook plugins from bash to native WASM") per capabilities.md §CAP-022. `host::write_file` is a direct enabling dependency for porting state-mutating bash hooks (S-8.04, S-8.09) to native WASM; without this SDK function those hooks cannot be ported. |
| L2 Domain Invariants | TBD (Phase 1.5 invariant lift pass) |
| Architecture Module | SS-02 — `crates/hook-sdk/src/host.rs`, `crates/factory-dispatcher/src/host/write_file.rs`, `crates/factory-dispatcher/src/host/path_util.rs` |
| Stories | S-8.10 (implementing story), S-8.04 (consumer: update-wave-state-on-merge), S-8.09 (consumer: regression-gate-adapter-retirement), S-19.03 (path_util.rs extraction of resolve_path_for_allowlist + write_file.rs two-step decomposed pattern adoption; EC-001 traversal-defense anchored by VP-097) |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `crates/hook-sdk/src/host.rs::read_file` (read_file symmetric target); `crates/factory-dispatcher/src/host/read_file.rs` (dispatcher reference implementation); `.factory/stories/S-8.10-sdk-extension-write-file.md` v1.1 (story spec, AC-1 through AC-8) |
| **Confidence** | HIGH — sibling `read_file` implementation confirmed in codebase; story spec pinned all signature details and error code mappings |
| **Extraction Date** | 2026-05-01 |
| **Extracted from** | `.factory/stories/S-8.10-sdk-extension-write-file.md` v1.1 |

#### Evidence Types Used

- type constraint (SDK function signature from story AC-1)
- documentation (S-8.10 Architecture Compliance Rules and Architecture Mapping table)
- sibling implementation (`read_file.rs` — symmetric reference)

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | Yes (filesystem write, FFI call to wasmtime host) |
| **Global state access** | No (all state passed via ctx parameter) |
| **Deterministic** | Yes (given same path, contents, capabilities) |
| **Thread safety** | TBD (Phase 1.6b will refine; wasmtime linker context threading) |
| **Overall classification** | effectful-shell (SDK wrapper + dispatcher binding); pure-core subset: `path_allowed` check |

#### Refactoring Notes

`path_allowed` (pure-core subset) should be extracted as a standalone function to enable unit testing without dispatcher context, matching the pattern in `read_file.rs::path_allowed`. The remaining `register`/`prepare` logic is inherently effectful-shell.

The shared path allowlist-resolution helper (`resolve_path_for_allowlist`) described above was extracted into `crates/factory-dispatcher/src/host/path_util.rs` and the two-step decomposed prepare() pattern (resolve→None→path_resolution_failed; prefix_check→false→path_not_allowed) was adopted in `write_file.rs` by S-19.03 (carried out as part of the warn-pending-wave-gate path_util extraction story; EC-001 traversal-defense anchored by VP-097).

## Changelog

- v1.8 (2026-07-17): RETROACTIVE-POL-14 (state-manager, D-852) — status ready→active; POL-14 applied retroactively at S-19.03 PR #611 merge 2026-07-13T15:54:21Z (D-834); S-19.03 behavioral_contracts: [BC-2.07.001, BC-2.02.011]; corrected in D-852 burst per F-004; input-hash e650b4b→a7ae268.
- v1.7 (2026-07-10): E-19 pass-45 F-P45-003 (product-owner) — frontmatter modified[] array re-sorted version-monotonic (was v1.3→v1.4→v1.6→v1.5; POLICY 14 leg-3 violation: last entry v1.5 vs Changelog top v1.6); no body content change.
- v1.6 (2026-07-09): orchestrator pre-pass-43 consistency sweep (product-owner): §Verification Properties VP-097 row added (F-P43-class missing-row; kani-proof; S-19.03 AC-001; co-anchored with BC-2.07.001 §Invariant 1; this BC's EC-001); §VP Anchors TBD placeholder replaced with VP-097 bullet.
- v1.5 (2026-07-08): E-19 pass-24 F-P24-001 fix burst (product-owner): §Traceability Stories row extended with S-19.03 (path_util.rs extraction of resolve_path_for_allowlist + write_file.rs two-step decomposed pattern adoption; EC-001 traversal-defense anchored by VP-097). §Story Anchor extended with S-19.03 anchor sentence (S-19.03 v1.12 behavioral_contracts: [BC-2.07.001, BC-2.02.011] confirmed as grounding). §Refactoring Notes annotated: resolve_path_for_allowlist extraction into path_util.rs and two-step decomposed prepare() pattern in write_file.rs carried out by S-19.03. Closes F-P24-001 bidirectional-parity drift. BC-INDEX version + Stories cell bump is state-manager same-burst.
- v1.4 (2026-07-06): E-19 pass-2 F-P2-004 fix burst (product-owner): §Architecture Anchors extended with `crates/factory-dispatcher/src/host/path_util.rs` bullet (shared path allowlist-resolution helper; extracted from write_file.rs at S-19.03; implements the rejoin algorithm for resolving both existing and absent file paths against the declared path_allow list; imported by both write_file.rs and read_file.rs). §Traceability Architecture Module field appended with `crates/factory-dispatcher/src/host/path_util.rs`. Closes F-P2-004. BC-INDEX v3.61→v3.62.
- v1.3 (2026-06-20): S-18.04a-prereq / ADR-028 §Decision 8 — corrected Invariant 3 path-resolution base `plugin_root` → `ctx.cwd` (CLAUDE_PROJECT_DIR), restoring parity with `read_file.rs::resolve_for_read` and matching production `invoke.rs`; the prior `plugin_root` claim was stale since S-8.07's read_file fix. Postcondition 5 corrected to match.
- v1.2 (2026-05-08): F-P18-002 prose-form line reference migration — 1 prose ref (`after line 187` in §Architecture Anchors) replaced with stable symbol anchor (after `pub fn read_file` declaration in `host.rs`).
- v1.1 (2026-05-08): TD-VSDD-091 stable-anchor migration sweep (Chunk 2) — 6 body cites migrated. `read_file.rs:92/101-107/73-77/36` and `lib.rs:58/43` and `host.rs:187` replaced with stable function/struct symbol anchors. Refactoring Notes cite also migrated.
