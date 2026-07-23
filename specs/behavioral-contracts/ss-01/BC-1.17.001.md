---
document_type: behavioral-contract
level: L3
version: "1.7"
status: active
producer: product-owner
timestamp: 2026-07-06T00:00:00Z
last_amended: "2026-07-15 (v1.7) — D-843 S1906-MERGED-W2-COMPLETE (state-manager): POL-14 auto-promotion — S-19.06 PR #657 9787c056 squash-merged 2026-07-15T14:53:16Z; status+lifecycle_status draft→active; input-hash ebf73ff UNCHANGED. BC-INDEX v4.04→v4.05 (v1.7 cell + active). [Prior: 2026-07-10 (v1.6) — E-19 pass-46 F-P46-001 fix burst (product-owner): frontmatter modified[] array re-sorted version-monotonic (was v1.2→v1.1→v1.3→v1.4→v1.5; POLICY 14 leg-3); no body content change. [Prior: (v1.5) — E-19 pass-30 O-P30-02 (product-owner): §Traceability L2 Domain Invariants TBD → 'none (host-ABI operational invariant, not L2 domain spec)' — aligned to BC-4.13.001 sibling convention. Spec behavioral content unchanged. BC-INDEX bump + story cite sweeps state-manager/story-writer same-burst. [Prior: (v1.4) — E-19 pass-28 F-P28-002 (product-owner): VP-101 proof-method cite aligned to VP-INDEX authoritative classification (integration only; POLICY 9); proptest qualifier removed from §VP Anchors bullet + two §Verification Properties Proof Method cells. Behavioral content unchanged. BC-INDEX bump state-manager same-burst; S-19.06 cite sweep story-writer same-burst. [Prior: (v1.3) — E-19 pass-22 fix burst F-P22-004 BC leg (product-owner): §Architecture Anchors added crates/hook-sdk/src/ffi.rs bullet — raw wire-ABI read_prefix extern declaration (-> i32; wasm32 extern block + host_stubs non-wasm stub), the layer §Description §(a) parenthetical assigns the i32 return to; ground-truth: ffi.rs read_file at lines 25 + 92 confirms file + module structure. Anchoring addition only; behavioral content unchanged. BC-INDEX bump state-manager same-burst. [Prior: (v1.2) — E-19 pass-12 F-P12-002 §(a) layering parenthetical (product-owner): inserted architect-recommended SDK/wire-ABI layering parenthetical after §(a) signature; closes F-P12-002 (BC leg; architect Ruling 1, amendment recommended-not-required, adopted under the production-grade default). [Prior: (v1.1) — E-19 pass-3 PO finalization (product-owner): F-P3-004 §VP Anchors + §Verification Properties VP-TBD → VP-101; F-P3-009 §Description(d) cite ADR-025 Decision 15 (drop phantom-pin parenthetical), §Architecture Anchors drop '(architect authors same-burst)', §Story Anchor updated S-19.06 (W2; depends_on S-19.03); F-P3-016 §Traceability CAP-TBD → CAP-009 with justification, ADR cite updated to ADR-025 Decision 15. [Prior: (v1.0) — initial creation (product-owner): E-19 pass-2 fix burst Package 2 — host::read_prefix bounded partial read: head-c semantics, NEVER OUTPUT_TOO_LARGE, additive FFI entry point, same path_allow + rejoin capability model as read_file (BC-2.07.001), absent file returns NOT_FOUND (-5), read_file all-or-nothing semantics unchanged (story anchor S-19.06; architect decision D-d).]]]]]"
phase: F3
inputs:
  - crates/factory-dispatcher/src/host/read_file.rs
  - crates/hook-sdk/src/host.rs
  - .factory/specs/behavioral-contracts/ss-02/BC-2.07.001.md
input-hash: "6bd89ca"
traces_to: .factory/specs/prd.md
origin: greenfield
extracted_from: null
subsystem: "SS-01"
capability: "CAP-009"
lifecycle_status: active
introduced: v1.0-feature-engine-discipline-E19
modified:
  - "2026-07-06 (v1.1)"
  - "2026-07-07 (v1.2)"
  - "2026-07-08 (v1.3)"
  - "2026-07-08 (v1.4)"
  - "2026-07-09 (v1.5)"
  - "2026-07-10 (v1.6) — E-19 pass-46 F-P46-001 fix burst (product-owner): frontmatter modified[] array re-sorted version-monotonic (was v1.2→v1.1→v1.3→v1.4→v1.5; POLICY 14 leg-3); no body content change."
  - "2026-07-15 (v1.7) — D-843 S1906-MERGED-W2-COMPLETE (state-manager): POL-14 auto-promotion; status+lifecycle_status draft→active; S-19.06 PR #657 9787c056 2026-07-15T14:53:16Z; input-hash ebf73ff UNCHANGED."
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# BC-1.17.001: host::read_prefix — bounded partial read (head-c semantics), NEVER OUTPUT_TOO_LARGE, additive FFI entry point, path_allow + rejoin capability model

## Description

`host::read_prefix` is a NEW host function that returns at most `max_bytes` bytes from the start of a file — equivalent to `head -c max_bytes` — and is guaranteed never to return OUTPUT_TOO_LARGE. It is an additive FFI entry point that does not change `read_file` semantics.

**(a) Signature and semantics.** `read_prefix(path: &str, max_bytes: u32, timeout_ms: u32) -> i32` (Layering: the SDK safe wrapper in crates/hook-sdk/src/host.rs returns `Result<Vec<u8>, HostError>`, mirroring host::read_file; the `-> i32` return belongs exclusively to the raw wire-ABI extern in the hook-sdk ffi module, whose realized shape is the 6-parameter pointer/length form mirroring ffi::read_file. Both layers are required.). The function returns at most `max_bytes` bytes from the start of the file (head-c semantics). If the file's total size is less than `max_bytes`, the full file content is returned. The return is byte-exact with no UTF-8 boundary trimming — the caller is responsible for interpreting partial multi-byte sequences. The function NEVER returns OUTPUT_TOO_LARGE (-3); by construction `max_bytes` IS the output cap, so the response always fits within the requested bound.

**(b) read_file all-or-nothing semantics unchanged.** `host::read_file` retains its existing all-or-nothing semantics: it reads the complete file and returns OUTPUT_TOO_LARGE if the file exceeds `max_bytes`. Plugins that currently rely on `read_file` for TOML/YAML parsing correctness must continue using `read_file` — silent truncation-as-success would corrupt those plugins' parsing (D-d rationale). `read_prefix` is for plugins that explicitly want a bounded prefix (e.g., reading the first N bytes of a log for pattern matching, reading YAML front-matter from a large markdown file).

**(c) Capability model.** `read_prefix` honors the same path allowlist (`path_allow`) and the same rejoin path-allowed resolution algorithm (BC-2.07.001, part b) as `read_file`. A separate capability block `capabilities.read_prefix` MUST appear in the registry entry for any plugin that calls `read_prefix`; absence of this block returns CAPABILITY_DENIED (-1) before any filesystem access. An absent-file path that is within the allowlist returns NOT_FOUND (-5), consistent with BC-2.07.001 part c.

**(d) ABI treatment.** `read_prefix` is an additive FFI entry point in the `vsdd` WASM import namespace. HOST_ABI_VERSION governance for this addition is recorded in ADR-025 Decision 15. Plugins that do not import `read_prefix` are unaffected; the new import is visible only to plugins that declare it.

## Preconditions

1. A plugin author declares `read_prefix` in the `vsdd` import namespace and invokes `host::read_prefix(path, max_bytes, timeout_ms)`.
2. The plugin's registry entry includes a `[hooks.capabilities.read_prefix]` block with a `path_allow` list covering the target directory.
3. `max_bytes` is a positive `u32` (0 is a valid degenerate case — see EC-001).
4. `crates/factory-dispatcher/src/host/path_util.rs` is available (same module used by `read_file` and `write_file`; BC-2.07.001 part b).

## Postconditions

1. **Bounded prefix returned.** On success, the response contains at most `max_bytes` bytes from the file's start. The byte count of the response is `min(file_size, max_bytes)`.

2. **Full content returned for short files.** If the file is smaller than `max_bytes`, the response contains the complete file content — no padding, no truncation marker.

3. **OUTPUT_TOO_LARGE never returned.** The function never emits OUTPUT_TOO_LARGE (-3). The `max_bytes` argument IS the cap; data beyond the cap is simply not read. This is the behavioral contract distinguishing `read_prefix` from `read_file`.

4. **Capability gate enforced before filesystem access.** Absent a `capabilities.read_prefix` block, the dispatcher returns CAPABILITY_DENIED (-1) and does not access the filesystem.

5. **Absent-file returns NOT_FOUND (-5).** A path within the allowlist where the file does not exist returns NOT_FOUND (-5) and emits `internal.file_not_found` (same semantics as BC-2.07.001 part c).

6. **Byte-exact prefix, no trimming.** The response is the raw first `max_bytes` bytes of the file content. No UTF-8 normalization, line-ending conversion, or boundary trimming is applied.

## Invariants

1. **Prefix determinism.** Given the same file content and the same `max_bytes`, `read_prefix` always returns the same byte sequence. No non-deterministic behavior.

2. **`read_file` semantics are immutable.** This BC does not modify the behavior of `read_file`. Existing plugins calling `read_file` observe no behavioral change. The two functions are independent host entry points with different response contracts.

3. **Separate capability block enforces defense-in-depth.** A plugin granted `capabilities.read_file` does NOT automatically receive `capabilities.read_prefix`. The two capabilities are independently declared in the registry. This allows operators to grant partial-read access without granting full-file-read access to large files.

4. **Traversal defense identical to read_file.** The `resolve_path_for_allowlist` function from `path_util.rs` is used for `read_prefix` path validation; the same rejoin algorithm, the same `starts_with` allowlist check, and the same traversal defense applies (BC-2.07.001 Invariant 1).

5. **NOT_FOUND semantics consistent with BC-2.07.001.** An absent-file path that is within the allowlist returns NOT_FOUND (-5) for both `read_file` and `read_prefix`. Plugin code handling NOT_FOUND from either function can use the same error-handling branch.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | `max_bytes = 0` | Returns an empty byte sequence (0 bytes) with exit code 0. The file is not read; the response is a zero-length payload. This is a valid degenerate case — the caller asked for zero bytes. |
| EC-002 | `max_bytes` greater than file size | Returns complete file content (no padding, no OUTPUT_TOO_LARGE). Response length is `file_size` bytes. |
| EC-003 | File is absent and path is within allowlist | Returns NOT_FOUND (-5); emits `internal.file_not_found`. Same as BC-2.07.001 part c. |
| EC-004 | Path is outside all allowed prefixes in `capabilities.read_prefix` | Returns CAPABILITY_DENIED (-1); emits `internal.capability_denied reason=path_not_allowed`. `capabilities.read_prefix` path_allow check is independent from `capabilities.read_file`. |
| EC-005 | Plugin has `capabilities.read_file` but no `capabilities.read_prefix` block | Returns CAPABILITY_DENIED (-1) when `read_prefix` is called. `read_file` capability does not extend to `read_prefix`. |
| EC-006 | Read times out before `max_bytes` are read | Returns TIMEOUT (-2); emits `internal.timeout`. Same timeout semantics as `read_file`. |
| EC-007 | File is a directory (not a regular file) | Returns INTERNAL_ERROR (-99) or equivalent OS-level error; does not return partial content. |
| EC-008 | Plugin compiled against a hook-sdk that does not declare `read_prefix` in the FFI module | Plugin cannot call `read_prefix` (it is simply absent from the import table). No runtime error; the capability is opt-in by declaration. |

## Canonical Test Vectors

| Input (path, max_bytes, file content or state) | Expected return value | Expected response bytes |
|------------------------------------------------|----------------------|------------------------|
| `.factory/wave-state.yaml` (20 bytes content), max_bytes=10, file present, allowlisted | 0 (ok) | First 10 bytes of file |
| `.factory/wave-state.yaml` (5 bytes content), max_bytes=100, file present, allowlisted | 0 (ok) | Full 5-byte content |
| `.factory/wave-state.yaml`, max_bytes=50, file absent, allowlisted | NOT_FOUND (-5) | empty |
| `/etc/passwd`, max_bytes=50, allowlist=[`.factory/`] | CAPABILITY_DENIED (-1) | empty |
| `.factory/wave-state.yaml`, max_bytes=0, file present, allowlisted | 0 (ok) | 0 bytes (empty payload) |
| `.factory/wave-state.yaml`, max_bytes=50, plugin has read_file capability only, no read_prefix capability | CAPABILITY_DENIED (-1) | empty |
| `.factory/wave-state.yaml`, max_bytes=50, timeout_ms=1, file exists but I/O stalls | TIMEOUT (-2) | empty |

## Related BCs

- BC-2.07.001 — host::read_file absent-file semantics; establishes codes::NOT_FOUND (-5), HostError::NotFound, rejoin algorithm, and path_util::resolve_path_for_allowlist that this BC inherits for the read_prefix code path
- BC-2.02.002 — bounded host calls are mandatory (read_file and exec_subprocess REQUIRE timeout_ms and a byte cap); read_prefix follows the same mandatory-bounds discipline
- BC-2.02.003 — HostError code mapping; codes -1/-2/-3/-4/-99 defined; this BC adds -5 via BC-2.07.001 (additive, not new here)
- BC-1.05.022 — read_file reads allowed file (all-or-nothing semantics unchanged; read_prefix is a separate fn)
- BC-1.05.024 — read_file rejects file exceeding max_bytes → OUTPUT_TOO_LARGE; read_prefix is the alternative that NEVER returns OUTPUT_TOO_LARGE

## Architecture Anchors

- `crates/factory-dispatcher/src/host/read_prefix.rs` — new host function implementation; imports `resolve_path_for_allowlist` from `path_util.rs`
- `crates/factory-dispatcher/src/host/path_util.rs` — shared path-allowed resolution (see BC-2.07.001 §Architecture Anchors)
- `crates/hook-sdk/src/host.rs` — new `read_prefix` wrapper callable from WASM plugins; parallel to existing `read_file` wrapper
- `crates/hook-sdk/src/ffi.rs` — raw wire-ABI `read_prefix` extern declaration (`-> i32` return; wasm32 extern block + host_stubs non-wasm stub); the layer §Description §(a) parenthetical assigns the `i32` return to
- `plugins/vsdd-factory/hooks-registry.toml` — `capabilities.read_prefix` capability block schema; separate from `capabilities.read_file`
- ADR-025 Decision 15 — HOST_ABI_VERSION governance for additive `read_prefix` FFI entry point

## Story Anchor

S-19.06 (host::read_prefix bounded partial read; W2; depends_on S-19.03)

## VP Anchors

- VP-101 — host::read_prefix Returns Byte-Exact Prefix of len <= max_bytes; Never OUTPUT_TOO_LARGE; Absent File Returns NOT_FOUND (-5) (integration; S-19.06)

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-101 | read_prefix(path, max_bytes) where file_size > max_bytes returns exactly max_bytes bytes at exit code 0; never OUTPUT_TOO_LARGE | integration (S-19.06) |
| VP-101 | read_prefix(path, max_bytes) where file_size < max_bytes returns full file_size bytes at exit code 0 | integration (S-19.06) |
| VP-101 | read_prefix on absent allowlisted path returns NOT_FOUND (-5); no OUTPUT_TOO_LARGE or CAPABILITY_DENIED | integration (S-19.06; inherits AC from BC-2.07.001) |
| VP-101 | read_prefix without capabilities.read_prefix block returns CAPABILITY_DENIED (-1) | integration (S-19.06) |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-009 |
| Capability Anchor Justification | CAP-009 ('Author and publish WASM hook plugins using the Rust SDK') covers all vsdd::* host function bindings; read_prefix is an additive FFI entry point in the vsdd namespace with a parallel hook-sdk wrapper, extending the host-function set without a new capability class. The separate capabilities.read_prefix gate follows the same SDK declaration model as capabilities.read_file. SS-01 is the implementation subsystem; SS-02 the SDK-surface owner (co-subsystem per CAP-002/CAP-010/CAP-013 precedent). |
| L2 Domain Invariants | none (host-ABI operational invariant, not L2 domain spec) |
| Architecture Module | SS-01 (Hook Dispatcher Core) — read_prefix host function + path_util integration |
| ADR | ADR-025 Decision 15 (HOST_ABI_VERSION governance for additive read_prefix FFI entry point; HOST_ABI_VERSION = 1 unchanged) |
| Stories | S-19.06 |
| Cycle | v1.0-feature-engine-discipline-E19 (F3) |
| Feature | E-19 — Post-rc.22 Operator Hardening |

## Changelog

| Version | Date | Author | Change |
|---------|------|--------|--------|
| 1.7 | 2026-07-15 | state-manager | D-843 POL-14 auto-promotion: S-19.06 PR #657 9787c056 squash-merged 2026-07-15T14:53:16Z; status+lifecycle_status draft→active; input-hash ebf73ff UNCHANGED. |
| 1.6 | 2026-07-10 | product-owner | F-P46-001: frontmatter modified[] re-sorted version-monotonic; no body content change. |
| 1.5 | 2026-07-09 | product-owner | E-19 pass-30 O-P30-02: §Traceability L2 Domain Invariants TBD → 'none (host-ABI operational invariant, not L2 domain spec)' — aligned to BC-4.13.001 sibling convention. Spec behavioral content unchanged. BC-INDEX bump + story cite sweeps state-manager/story-writer same-burst. |
| 1.4 | 2026-07-08 | product-owner | E-19 pass-28 F-P28-002: VP-101 proof-method cite aligned to VP-INDEX authoritative classification (integration only; POLICY 9); proptest qualifier removed from §VP Anchors bullet + two §Verification Properties Proof Method cells (rows: file_size>max_bytes and file_size<max_bytes). VP-INDEX line 459 Proof Method col = `integration`; proptest breakdown line 353 lists VP-059/069/075/080/096 (VP-101 absent). Behavioral content unchanged. BC-INDEX bump state-manager same-burst; S-19.06 cite sweep story-writer same-burst. |
| 1.3 | 2026-07-08 | product-owner | E-19 pass-22 fix burst F-P22-004 BC leg: §Architecture Anchors added crates/hook-sdk/src/ffi.rs — raw wire-ABI read_prefix extern declaration (-> i32 return; wasm32 extern block + host_stubs non-wasm stub), the layer §Description §(a) assigns the i32 return to; sibling ground-truth: ffi.rs read_file at lines 25 + 92 confirms file + module structure. Anchoring addition only; behavioral content unchanged. BC-INDEX bump state-manager same-burst. |
| 1.2 | 2026-07-07 | product-owner | E-19 pass-12 F-P12-002 §(a) layering parenthetical: inserted architect-recommended SDK/wire-ABI layering parenthetical after §(a) signature. Closes F-P12-002 (BC leg; architect Ruling 1, amendment recommended-not-required, adopted under the production-grade default). |
| 1.1 | 2026-07-06 | product-owner | E-19 pass-3 PO finalization: (a) F-P3-004 — §VP Anchors VP-TBD → VP-101 (host::read_prefix Returns Byte-Exact Prefix of len <= max_bytes; Never OUTPUT_TOO_LARGE; Absent File Returns NOT_FOUND (-5)); §Verification Properties four VP-TBD rows → VP-101. (b) F-P3-009 — §Description(d) cite ADR-025 Decision 15 directly (drop phantom-pin parenthetical "cited as ADR-025 with no Decision number to avoid phantom-pin on a Decision number not yet authored"); §Architecture Anchors "ADR-025 (amendment)" → "ADR-025 Decision 15", drop "(architect authors same-burst)"; §Story Anchor updated "story file does not exist at BC authorship time; story-writer authors next leg" → "W2; depends_on S-19.03" (S-19.06 now exists v1.0). (c) F-P3-016 — §Traceability L2 Capability CAP-TBD → CAP-009 with justification; Capability Anchor Justification TBD → full text; ADR "ADR-025 (amendment, no Decision number — architect authors same-burst; ...)" → "ADR-025 Decision 15 (HOST_ABI_VERSION governance for additive read_prefix FFI entry point; HOST_ABI_VERSION = 1 unchanged)". Frontmatter capability: "CAP-TBD" → "CAP-009". |
| 1.0 | 2026-07-06 | product-owner | Initial creation. E-19 pass-2 fix burst Package 2. New host fn read_prefix: (a) head-c semantics, max_bytes cap, NEVER OUTPUT_TOO_LARGE, byte-exact no-trimming, max_bytes=0 valid; (b) read_file all-or-nothing semantics UNCHANGED (silent-truncation-as-success would corrupt TOML/YAML parsers — D-d rationale); (c) separate capabilities.read_prefix block, same path_allow + rejoin model as read_file (BC-2.07.001), absent file returns NOT_FOUND (-5); (d) additive FFI entry point in vsdd namespace; HOST_ABI_VERSION governance in ADR-025 amendment (bare cite, no Decision number). Story anchor S-19.06 (story not yet authored). |
