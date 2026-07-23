---
document_type: behavioral-contract
level: L3
version: "1.6"
status: active
producer: product-owner
timestamp: 2026-07-13T15:54:21Z
last_amended: "(v1.6) — POL-14 auto-promotion: lifecycle_status draft→active on PR #611 squash-merge 091ce499 (S-19.03 MERGED 2026-07-13T15:54:21Z); BC-INDEX v3.98→v3.99; D-834. [Prior: (v1.5) — E-19 pass-42 F-P42-002+F-P42-003 (product-owner): §Verification Properties VP-097 row property cell mis-anchor → traversal-defense framing per VP-INDEX SoT; duplicate VP-098 row consolidated to single canonical-postcondition row, proof-method integration per VP-INDEX; POLICY 9/POLICY 4. [Prior: (v1.4) — E-19 pass-32 O-P32-01 (product-owner): §Traceability L2 Domain Invariants DI-TBD → none (host-ABI operational) — pass-30 sibling-sweep miss aligned to BC-1.17.001/BC-4.13.001 convention. BC-INDEX bump state-manager's, S-19.03 cite sweep story-writer's, same burst. [Prior: (v1.3) — E-19 pass-30 F-P30-002 sibling-sweep (product-owner): input-hash placeholder retired per POLICY 18 (TD-VSDD-060; compute-input-hash --update; real digest); mechanical metadata fix; spec content unchanged. BC-INDEX bump + story cite sweeps state-manager/story-writer same-burst. [Prior: (v1.2) — E-19 pass-7 F-P7-004 adjudication (product-owner): EC-007 reformulated from 'no ancestor canonicalizes on real filesystem' (untestable dead branch on portable Unix — / always canonicalizes) to 'mock-injectable canonicalize returns Err for every ancestor' (portably testable via unit injection). Testability note added: path_util::resolve_path_for_allowlist MUST accept injectable canonicalize fn parameter; S-19.03 AC-001 negative-control B updated accordingly. BC-INDEX v3.71→v3.72. [Prior: (v1.1) — E-19 pass-3 PO finalization (product-owner): F-P3-004 §VP Anchors + §Verification Properties VP-TBD → VP-097 (traversal defense kani-proof; also anchored BC-2.02.011 EC-001) + VP-098 (allowlisted-absent file NOT_FOUND (-5) + zero CAPABILITY_DENIED false-positives integration). [Prior: (v1.0) — initial creation (product-owner): E-19 pass-2 fix burst Package 2 — host::read_file absent-file semantics: codes::NOT_FOUND (-5) additive error code, HostError::NotFound SDK variant (no #[non_exhaustive] per O-P2-002), rejoin path-allowed resolution via shared path_util module, zero false-positive capability_denied for allowlisted-absent paths (story anchor S-19.03; closes rc.22 smoke FINDING-2 BC leg).]]]]]"
phase: F3
inputs:
  - .factory/stories/S-19.03-warn-pending-wave-gate-file-not-found.md
  - crates/factory-dispatcher/src/host/read_file.rs
  - crates/hook-sdk/src/host.rs
input-hash: "5018c37"
traces_to: .factory/specs/prd.md
origin: greenfield
extracted_from: null
subsystem: "SS-02"
capability: "CAP-009"
lifecycle_status: active
introduced: v1.0-feature-engine-discipline-E19
modified:
  - "2026-07-06 (v1.1)"
  - "2026-07-07 (v1.2)"
  - "2026-07-09 (v1.3)"
  - "2026-07-09 (v1.4)"
  - "2026-07-09 (v1.5)"
  - "2026-07-13 (v1.6)"
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# BC-2.07.001: host::read_file absent-file semantics — codes::NOT_FOUND (-5) additive error code, HostError::NotFound SDK variant, rejoin path-allowed resolution, zero false-positive capability_denied

## Description

Three layers close the false-positive `capability_denied` defect where `read_file.rs::path_allowed()` incorrectly returned `path_not_allowed` for allowlisted paths that pointed to absent (not-yet-existing) files, because `canonicalize()` fails on non-existent targets.

**(a) Additive error code codes::NOT_FOUND = -5 and HostError::NotFound.** The occupied negative integer codes in the dispatcher ABI are: 0 (ok), -1 (CAPABILITY_DENIED), -2 (TIMEOUT), -3 (OUTPUT_TOO_LARGE), -4 (INVALID_ARGUMENT), -99 (INTERNAL_ERROR). Code -5 is the next free compact-negative slot per ADR-025 Decision 13; it is added as `pub const NOT_FOUND: i32 = -5` in the dispatcher's codes module and exported from `hook-sdk`. The `HostError` enum in hook-sdk gains a named `NotFound` variant; `from_code(-5)` maps to `HostError::NotFound`. The existing `Other(i32)` catch-all is preserved so plugins compiled against older SDK versions continue to compile — no `#[non_exhaustive]` attribute is added, per architect O-P2-002 adjudication. HOST_ABI_VERSION remains 1; this is an additive enumeration, not a breaking ABI change (ADR-025 Decision 13).

**(b) Rejoin path-allowed resolution for absent targets via shared path_util module.** `read_file.rs::path_allowed()` MUST use the rejoin algorithm for target paths that do not yet exist: if `resolved.canonicalize()` fails (file absent), walk ancestors until one canonicalizes successfully (the deepest existing ancestor), rejoin the non-existent tail path components back onto the canonical ancestor, producing a synthesized canonical path, then check that this synthesized path `starts_with` an allowed prefix. The algorithm MUST be implemented in a shared host module `crates/factory-dispatcher/src/host/path_util.rs` (function `resolve_path_for_allowlist`) and imported by both `read_file.rs` and `write_file.rs` — no duplication. This mirrors the helper already present in `write_file.rs` at the time of S-19.03 authorship. An allowlisted absent path returns `path_allowed() == true`; a path outside all allowed prefixes (even if the file exists) continues to return `false`.

**(c) read_file on an allowlisted-but-absent path returns NOT_FOUND (-5); dispatcher emits internal.file_not_found.** When `path_allowed()` returns `true` but the file open/read fails because the file does not exist, the dispatcher MUST emit a `internal.file_not_found` event (NOT `internal.capability_denied`) and return `codes::NOT_FOUND (-5)` to the plugin. Zero `internal.capability_denied reason=path_not_allowed` events MUST be emitted for any plugin call on an allowlisted-but-absent path after this fix. Genuine path-not-allowed violations (path outside all allowed prefixes) continue to emit `internal.capability_denied reason=path_not_allowed` unchanged.

## Preconditions

1. A plugin invokes `host::read_file(path, max_bytes, timeout_ms)` where `path` resolves to an allowlisted directory prefix but the file at `path` does not yet exist.
2. The `capabilities.read_file.path_allow` list in the registry entry for the calling plugin covers the directory containing `path`.
3. `crates/factory-dispatcher/src/host/path_util.rs` exports `resolve_path_for_allowlist` (extracted from `write_file.rs`) and is importable by both `read_file.rs` and `write_file.rs`.
4. The hook-sdk version used by the plugin includes the `HostError::NotFound` variant and the exported `NOT_FOUND` constant.

## Postconditions

1. **Absent-file-in-allowlist path resolves to allowed.** `path_allowed()` returns `true` for any path whose synthesized canonical form (deepest-existing-ancestor + rejoined tail) `starts_with` an allowed prefix, even if no file exists at the path.

2. **NOT_FOUND returned for allowlisted-but-absent paths.** A `read_file` call on an allowlisted-but-absent path returns `codes::NOT_FOUND (-5)` to the WASM plugin and emits `internal.file_not_found` on the dispatcher's event stream.

3. **No false-positive capability_denied for allowlisted-absent paths.** After this fix, zero `internal.capability_denied reason=path_not_allowed` events are emitted for `read_file` calls on paths within the plugin's `path_allow` list, regardless of whether the target file exists.

4. **Genuine allowlist violations unaffected.** Paths outside all allowed prefixes still emit `internal.capability_denied reason=path_not_allowed` and return `CAPABILITY_DENIED (-1)` (unchanged behavior).

5. **HostError::NotFound variant is callable from plugin code.** Plugin authors can match `HostError::NotFound` from a `read_file` return value without resorting to the raw integer `-5` or the `Other(i32)` catch-all.

## Invariants

1. **Traversal defense preserved.** The rejoin algorithm cannot escape the allowlist: it canonicalizes the deepest EXISTING ancestor (which is a real filesystem path, not a user-controlled path component), then appends only the remaining non-existent tail. The resulting synthesized path still begins with a real canonical prefix that is verifiable via `starts_with`. An attacker-controlled `..` in the absent tail would resolve to a real ancestor during canonicalization or fail canonicalization entirely — the tail is rejoined verbatim and the `starts_with` check catches any escape attempt.

2. **HOST_ABI_VERSION = 1 unchanged.** Adding code -5 and the `NotFound` variant is additive. Plugins compiled against older hook-sdk versions that use `HostError::Other(i32)` to catch unknown codes continue to compile and run correctly; -5 maps to `Other(-5)` in older plugins until they are recompiled against the new SDK.

3. **codes::NOT_FOUND (-5) is distinct from all other occupied codes.** The slot -5 is verified unoccupied at time of allocation per ADR-025 Decision 13: occupied codes are 0/-1/-2/-3/-4/-99; -5 is the next compact-negative slot.

4. **Fail-open plugin behavior on NOT_FOUND is plugin-specific.** This BC mandates the dispatcher's return code and event; it does not mandate how plugins handle NOT_FOUND. The canonical reference plugin (`warn-pending-wave-gate`) treats NOT_FOUND as "file absent → no pending gate → Continue" (BC-governed by the story-level AC). Other plugins may treat NOT_FOUND differently per their semantics.

5. **path_util module is the sole implementation of resolve_path_for_allowlist.** Neither `read_file.rs` nor `write_file.rs` may contain a local copy; both MUST import from `path_util`. This prevents future divergence in path resolution semantics between the read and write paths.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Path points to a dangling symlink (symlink exists, target absent) | `canonicalize()` on the symlink itself may fail (dangling); the rejoin algorithm walks to the deepest REAL ancestor (the directory containing the symlink, or higher). The synthesized canonical path covers the symlink's location. If allowlisted, returns NOT_FOUND (-5) when the read fails. |
| EC-002 | Path contains absent intermediate directories (e.g., `.factory/nonexistent-dir/file.yaml`) | Ancestor walk descends until it finds the deepest existing ancestor (e.g., `.factory/`). Tail is `.factory/nonexistent-dir/file.yaml` minus the existing prefix. Synthesized path is `<canonical-.factory>/nonexistent-dir/file.yaml`; `starts_with` check applies. |
| EC-003 | Path contains `..` in the absent tail segment (e.g., `.factory/subdir/../file.yaml`) | The absent tail is rejoined verbatim AFTER the canonical ancestor. A `..` in the tail that would escape the allowlist prefix fails the `starts_with` check and returns CAPABILITY_DENIED (-1), not NOT_FOUND. Traversal defense preserved per Invariant 1. |
| EC-004 | Absent path is completely outside all allowed prefixes | `path_allowed()` returns `false` via the normal path (synthesized canonical path does not `starts_with` any allowed prefix). Dispatcher emits `internal.capability_denied reason=path_not_allowed` and returns CAPABILITY_DENIED (-1). Identical to the pre-fix behavior for genuinely disallowed paths. |
| EC-005 | Plugin compiled against old hook-sdk that lacks `HostError::NotFound` | Old plugin receives integer -5 from the host. `HostError::from_code(-5)` maps to `Other(-5)` in the old SDK. Plugin behavior on `Other(-5)` is plugin-defined. No crash; no ABI break. |
| EC-006 | Plugin explicitly checks for `HostError::NotFound` but is running against a dispatcher that predates this fix (returns CAPABILITY_DENIED for absent files) | Plugin receives CAPABILITY_DENIED (-1) where it expected NOT_FOUND (-5). Plugin SHOULD handle this gracefully (both codes indicate it cannot read the file; the distinction is advisory). Behavioral contract for such plugins is out of scope for this BC. |
| EC-007 | `resolve_path_for_allowlist` receives a path where `canonicalize()` returns an error for every ancestor attempted by the walk (e.g., a unit test injects a mock canonicalize function that always returns `Err`; on production Unix filesystems this branch is structurally unreachable because `/` always canonicalizes, making the original precondition "no existing ancestor" a dead branch in portable tests) | Algorithm exhausts all ancestors without finding one that canonicalizes. Returns `false`. Dispatcher emits `internal.capability_denied` with `reason = path_resolution_failed` (not `path_not_allowed`). **Testability:** `path_util::resolve_path_for_allowlist` MUST accept an injectable `canonicalize` function parameter (signature `fn(&Path) -> std::io::Result<PathBuf>`) so unit tests can cover this branch without a real sandboxed filesystem. S-19.03 AC-001 negative-control B MUST inject a mock returning `Err(std::io::Error::from(std::io::ErrorKind::NotFound))` for every ancestor and assert `path_allowed() == false` + `reason = path_resolution_failed`. |

## Canonical Test Vectors

| Input (path, allowlist, file exists) | Expected dispatcher return | Expected event emitted |
|--------------------------------------|---------------------------|----------------------|
| `.factory/wave-state.yaml`, allow=[`.factory/`], file absent | NOT_FOUND (-5) | `internal.file_not_found` |
| `.factory/wave-state.yaml`, allow=[`.factory/`], file present, content ≤ max_bytes | Ok (0) with content | (normal read event) |
| `/etc/passwd`, allow=[`.factory/`], file present | CAPABILITY_DENIED (-1) | `internal.capability_denied reason=path_not_allowed` |
| `.factory/nonexistent-subdir/file.yaml`, allow=[`.factory/`], subdir absent | NOT_FOUND (-5) | `internal.file_not_found` |
| `.factory/../secrets/key`, allow=[`.factory/`], file absent | CAPABILITY_DENIED (-1) | `internal.capability_denied reason=path_not_allowed` |
| `.factory/wave-state.yaml`, no `capabilities.read_file` block in registry | CAPABILITY_DENIED (-1) | `internal.capability_denied reason=capability_missing` |

## Related BCs

- BC-2.02.003 — HostError code mapping (-1=-CapabilityDenied, -2=-Timeout, -3=-OutputTooLarge, -4=-InvalidArgument, Other(i32)=fallback); this BC adds -5=NotFound to that enumeration (additive; BC-2.02.003 extended, not superseded)
- BC-2.02.011 — host::write_file; write_file.rs contains `resolve_path_for_allowlist` that is the source of the extracted `path_util` function; this BC mandates the extraction to a shared module
- BC-1.05.021 — read_file denies when no capability block (unchanged by this BC)
- BC-1.05.022 — read_file reads allowed file (unchanged; this BC adds the absent-file arm to the same code path)
- BC-1.05.023 — read_file rejects path outside allow list (unchanged; genuine violations still emit capability_denied)
- BC-1.17.001 — host::read_prefix (sibling new host fn; inherits same path_allow + rejoin resolution model per this BC)

## Architecture Anchors

- `crates/factory-dispatcher/src/host/read_file.rs` — primary implementation: `path_allowed()` rejoin algorithm, `prepare()` NOT_FOUND arm, `internal.file_not_found` event emission
- `crates/factory-dispatcher/src/host/path_util.rs` — shared `resolve_path_for_allowlist` function (new module; extracted from `write_file.rs`)
- `crates/factory-dispatcher/src/host/write_file.rs` — imports `resolve_path_for_allowlist` from `path_util` (existing logic extracted, not changed)
- `crates/hook-sdk/src/host.rs` — `HostError::NotFound` variant + `from_code(-5)` mapping + exported `NOT_FOUND` constant
- ADR-025 Decision 13 — authoritative for codes::NOT_FOUND = -5 allocation; HOST_ABI_VERSION = 1 unchanged

## Story Anchor

S-19.03 (warn-pending-wave-gate FINDING-2: read_file file_not_found semantics + graceful absent-file handling)

## VP Anchors

- VP-097 — path_util::resolve_path_for_allowlist Traversal Defense — .. Sequences Cannot Resolve Outside Allowlist Prefixes (kani-proof; S-19.03; also anchored BC-2.02.011 EC-001)
- VP-098 — Allowlisted-but-Absent File Returns internal.file_not_found Event and NOT_FOUND (-5); Zero CAPABILITY_DENIED False-Positives (integration; S-19.03)

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-097 | path_util::resolve_path_for_allowlist traversal defense — .. sequences cannot resolve outside allowlist prefixes | kani-proof (S-19.03 AC-001; also anchored BC-2.02.011 EC-001) |
| VP-098 | Allowlisted-but-Absent File Returns internal.file_not_found Event and NOT_FOUND (-5); Zero CAPABILITY_DENIED False-Positives | integration (bats; S-19.03 AC-002; AC-003 grep gate is a static prerequisite check, not a separate VP scope) |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-009 ("Plugin-author SDK ABI surface — hook-sdk callable API") per capabilities.md §CAP-009 |
| Capability Anchor Justification | CAP-009 ("Plugin-author SDK ABI surface") — this BC governs an additive extension to the HostError enum and the corresponding negative code in the SDK ABI; plugin authors observe and match on HostError::NotFound from read_file return values, which is the public SDK surface layer (SS-02) |
| L2 Domain Invariants | none (host-ABI operational invariant, not L2 domain spec) |
| Architecture Module | SS-02 (Hook SDK and Plugin ABI) — HostError::NotFound variant + NOT_FOUND export; SS-01 (Hook Dispatcher Core) — read_file.rs rejoin algorithm + path_util extraction (co-anchored) |
| ADR | ADR-025 Decision 13 (codes::NOT_FOUND = -5 allocation; HOST_ABI_VERSION = 1 unchanged; additive enumeration) |
| Stories | S-19.03 |
| Cycle | v1.0-feature-engine-discipline-E19 (F3) |
| Feature | E-19 — Post-rc.22 Operator Hardening |

## Changelog

| Version | Date | Author | Change |
|---------|------|--------|--------|
| 1.6 | 2026-07-13 | state-manager | POL-14 auto-promotion: lifecycle_status draft→active on PR #611 squash-merge 091ce499 (S-19.03 MERGED 2026-07-13T15:54:21Z); BC-INDEX v3.98→v3.99; D-834. |
| 1.5 | 2026-07-09 | product-owner | F-P42-002: §Verification Properties VP-097 row property cell mis-anchor — replaced Postcondition-1/EC-004 semantics with traversal-defense framing per VP-INDEX SoT (`path_util::resolve_path_for_allowlist traversal defense — .. sequences cannot resolve outside allowlist prefixes`); POLICY 4 semantic anchoring. F-P42-003: duplicate VP-098 row (static grep-gate row) consolidated — single canonical postcondition row per VP-INDEX SoT title; proof-method updated to `integration (bats; S-19.03 AC-002; AC-003 grep gate is a static prerequisite check, not a separate VP scope)`; POLICY 9 VP-INDEX-SoT. |
| 1.4 | 2026-07-09 | product-owner | E-19 pass-32 O-P32-01: §Traceability L2 Domain Invariants DI-TBD → none (host-ABI operational invariant, not L2 domain spec) — pass-30 sibling-sweep miss aligned to BC-1.17.001/BC-4.13.001 convention. Behavioral content unchanged. BC-INDEX bump state-manager same-burst; S-19.03 cite sweep story-writer same-burst. |
| 1.3 | 2026-07-09 | product-owner | E-19 pass-30 F-P30-002 sibling-sweep (TD-VSDD-060): input-hash placeholder retired per POLICY 18 (compute-input-hash --update; real digest); mechanical metadata fix; spec content unchanged. BC-INDEX bump + story cite sweeps state-manager/story-writer same-burst. |
| 1.2 | 2026-07-07 | product-owner | E-19 pass-7 F-P7-004 adjudication: EC-007 reformulated — precondition changed from "no existing ancestor on real filesystem" (unreachable dead branch on portable Unix; `/` always canonicalizes) to "mock-injectable canonicalize returns Err for every ancestor attempted" (portably testable via unit injection). `path_util::resolve_path_for_allowlist` MUST accept an injectable `canonicalize` fn parameter (signature `fn(&Path) -> std::io::Result<PathBuf>`) enabling unit tests to inject a mock returning `Err` for every path. EC-007 expected-behavior column updated with testability note. S-19.03 AC-001 negative-control B ruling for story-writer: inject mock canonicalize returning `Err(...)` for every ancestor; assert `path_allowed() == false` + dispatcher emits `internal.capability_denied reason=path_resolution_failed`. BC-INDEX v3.71→v3.72. |
| 1.1 | 2026-07-06 | product-owner | E-19 pass-3 PO finalization: F-P3-004 — §VP Anchors VP-TBD → VP-097 (path_util::resolve_path_for_allowlist Traversal Defense kani-proof; also anchored BC-2.02.011 EC-001) + VP-098 (Allowlisted-but-Absent File Returns internal.file_not_found Event and NOT_FOUND (-5); Zero CAPABILITY_DENIED False-Positives integration); §Verification Properties three VP-TBD rows → VP-097/VP-098 per property alignment. |
| 1.0 | 2026-07-06 | product-owner | Initial creation. E-19 pass-2 fix burst Package 2. Three-layer absent-file contract: (a) codes::NOT_FOUND = -5 additive (ADR-025 Decision 13; HOST_ABI_VERSION = 1 unchanged); HostError::NotFound variant in hook-sdk; Other(i32) catch-all preserved; no #[non_exhaustive] (O-P2-002). (b) Rejoin path-allowed algorithm via shared path_util::resolve_path_for_allowlist (extracted from write_file.rs; imported by both read_file.rs and write_file.rs). (c) Allowlisted-absent path returns NOT_FOUND (-5) + internal.file_not_found event; zero false-positive capability_denied. Closes rc.22 smoke FINDING-2 BC leg (S-19.03). |
