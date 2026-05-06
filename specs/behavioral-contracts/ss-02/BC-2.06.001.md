---
document_type: behavioral-contract
level: L3
version: "1.4"
status: draft
producer: architect
timestamp: 2026-05-06T00:00:00Z
phase: 1c
inputs:
  - .factory/specs/architecture/decisions/ADR-015-single-stream-otel-schema.md
  - .factory/stories/S-10.05-adr015-wave2-plugin-schema-migration.md
input-hash: "[pending-recompute]"
traces_to: ADR-015-single-stream-otel-schema.md
origin: spec-revision
subsystem: "SS-02"
capability: "CAP-009"
lifecycle_status: active
introduced: v1.1.0
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# Behavioral Contract BC-2.06.001: vsdd-hook-sdk::versioning::wave2_major_semver_bump_for_d_15_3_host_field_precedence — SDK MAJOR version increment signals host-field-precedence semantics change per ADR-015 D-15.3 with migration guidance for plugin authors

## Description

ADR-015 D-15.3 introduces **host-field-precedence** semantics: when a plugin's
emitted event payload includes a field that the dispatcher also stamps as a
Resource attribute or per-event host-stamped field, the host-supplied value
wins unconditionally. Plugin-supplied values for host-owned fields are silently
overridden, and a `vsdd.internal.host_field_override.v1` lifecycle event is
emitted (per BC-1.12.005) to make the override observable.

This behavioral change is ABI-level for plugin authors: a plugin author who
relied on their plugin setting `service.name` or `trace_id` directly will find
those values silently replaced after the Wave 2 SDK ships. This is a
**BREAKING CHANGE** in the SemVer sense — callers cannot remain on the old
major version and get the new dispatcher behavior.

This BC governs the SDK MAJOR semver bump that accompanies the Wave 2 SDK
release: the `vsdd-hook-sdk` crate version MUST be incremented to the next
MAJOR version when D-15.3 host-field-precedence semantics ship in the
dispatcher. The version bump is the PUBLIC SIGNAL to plugin authors that
host-field-precedence is now in effect, per Cargo's SemVer resolver semantics.

## Preconditions

1. The Wave 2 dispatcher ships with ADR-015 D-15.3 host-field-precedence
   enforcement active in `host::emit_event` (i.e., BC-1.12.005 is implemented).
2. The current `vsdd-hook-sdk` `Cargo.toml` is at a MAJOR version N (e.g.,
   `0.X.Y` for pre-1.0 or `N.X.Y` for N ≥ 1).
3. The `CHANGELOG.md` in `crates/hook-sdk/` is present and maintained.

## Postconditions

1. `crates/hook-sdk/Cargo.toml` `version` field is incremented to the next
   MAJOR version (i.e., if current version is `0.X.Y`, new version is
   `1.0.0`; if current version is `N.X.Y` for N ≥ 1, new version is
   `(N+1).0.0`). Minor and patch components are reset to zero per SemVer
   convention.
2. `crates/hook-sdk/CHANGELOG.md` contains a new entry for the MAJOR version
   release that MUST include at minimum:
   - A section header with the new version and release date.
   - A "Breaking Changes" subsection documenting that host-field-precedence
     (ADR-015 D-15.3) is now active — REQUIRED when the public API surface
     removes or changes signatures.
   - An "Added / New API" subsection — REQUIRED when the public API gains new
     constructors, methods, or types (additive, non-breaking). For the Wave 2
     MAJOR release this MUST document:
     - `HookResult::block_with_fix(hook, reason, recommendation, code)` — the
       preferred constructor for agent-actionable block messages introduced in
       v1.0.0-rc.12; formats the canonical
       `"BLOCKED by <hook>: <reason>. Fix: <recommendation>. Code: <code>."`
       single-line shape for `permissionDecisionReason` and telemetry.
     - `HookResult::block(reason)` — retained for backward compatibility; new
       plugin sites SHOULD prefer `block_with_fix`.
   - A "Migration Guide" subsection — REQUIRED when "Breaking Changes" is
     non-empty. Contains the guidance paragraph explaining: "Plugin authors that set
     host-owned Resource fields (`service.name`, `service.namespace`,
     `service.instance.id`, `service.version`, `deployment.environment.name`,
     `host.name`, `host.id`, `os.type`, `process.pid`, `vcs.repository.url.full`, `vcs.repository.name`, `vcs.provider.name`, `vcs.owner.name` (per BC-1.12.003 Postcondition 1's authoritative VCS Resource attribute list. Note that `vcs.ref.head.name`, `vcs.ref.head.revision`, `vcs.ref.head.type` are PER-EVENT identity fields per BC-1.12.004 Postcondition 1, NOT Resource attributes — also host-stamped but in a different category.), `worktree.id`,
     `schema_url`) or per-event identity fields (`trace_id`, `event.id`,
     `event.category`, `event.name`, `span_id`, `parent_span_id`,
     `plugin.invocation_id`, `session.id`, `timestamp`, `event.source`) must
     remove those fields from their plugin payloads. The dispatcher will
     override them and emit a `vsdd.internal.host_field_override.v1` lifecycle
     event for each override."
   - A reference to BC-1.12.005 (host_field_override contract) and
     ADR-015 D-15.3 for the authoritative specification.
3. Plugin crates that pin to the old MAJOR version via `vsdd-hook-sdk = "N"`
   (Cargo `~` or `=` or `^N` constraints) will NOT automatically upgrade to
   the new MAJOR version, per Cargo's SemVer resolver behavior. This is
   intentional: plugin authors MUST opt into the new MAJOR version explicitly
   after reading the migration guidance.
4. The `vsdd-hook-sdk` crate documentation (docs.rs or equivalent) is updated
   to reflect the host-field-precedence semantics in the `emit_event` and
   `emit_pair` API documentation.

## Invariants

1. The SDK MAJOR version is bumped EXACTLY ONCE for the D-15.3 host-field-
   precedence change. There is no intermediate MINOR bump followed by a later
   MAJOR bump for the same feature; the MAJOR bump is the Wave 2 release.
2. For the Wave 2 SDK MAJOR release specifically, the CHANGELOG MAJOR version
   entry is present and contains both a "Breaking Changes" section AND an
   "Added / New API" section before the SDK release tag is created. (Wave 2
   unconditionally has at least one additive API: `HookResult::block_with_fix`
   added in v1.0.0-rc.12.) The "Breaking Changes" section MUST document
   host-field-precedence (ADR-015 D-15.3); the "Added / New API" section MUST
   document `HookResult::block_with_fix` as the preferred constructor for
   blocking plugins introduced in v1.0.0-rc.12. An SDK release without either
   required section is a postcondition violation for Wave 2. Future MAJOR
   releases follow Postcondition 2's conditional rule: "Added / New API" is
   REQUIRED only when the public API gains new constructors, methods, or types.
3. The SemVer increment rule per Cargo convention is: MAJOR bump resets MINOR
   and PATCH to zero. `0.X.Y → 1.0.0` or `N.X.Y → (N+1).0.0`. Any other
   increment formula violates this invariant.

## Risk Notes

- **Re-coupling risk:** If plugin ecosystem pins to `vsdd-hook-sdk = "0"` (pre-1.0
  wildcard) rather than an explicit major constraint, they may or may not receive
  the MAJOR bump depending on how `^` wildcard semantics apply to pre-1.0 crates
  in Cargo. Plugin authors should use `vsdd-hook-sdk = "= N.X.Y"` exact pinning
  or `vsdd-hook-sdk = "N"` major-pinning to control upgrade behavior explicitly.
- **Migration window:** Between Wave 2 SDK release and Wave 3 (shim removal), both
  old-major and new-major versions of the SDK are in active use across the plugin
  ecosystem. The dispatcher must be compatible with plugins compiled against EITHER
  MAJOR version during this window (the WASM ABI boundary is the compatibility
  surface, not Cargo; WASM ABI compatibility is governed by BC-2.01.001–004).

## Related BCs

- BC-1.12.005 — host_field_override contract (the dispatcher-side behavior this
  MAJOR bump signals; plugin authors must consult BC-1.12.005 for the exact
  override semantics)
- BC-2.01.001 — hook-sdk ABI stability (sibling: the WASM ABI is NOT broken by
  this MAJOR bump; the MAJOR bump signals behavioral semantics, not WASM binary
  incompatibility)
- BC-4.09.001 — hook-plugins wave2 reverse-DNS event-name migration (companion:
  the 11 native plugins also migrate in Wave 2; this BC covers the SDK version
  bump; BC-4.09.001 covers the plugin-side event-name migration)
- BC-1.11.003 — emit_pair host helper (sibling: also part of Wave 2 SDK surface;
  ships in the same MAJOR version release)

## Architecture Anchors

- `crates/hook-sdk/Cargo.toml` — `version` field; MAJOR increment site
- `crates/hook-sdk/CHANGELOG.md` — migration guidance publication site
- ADR-015 D-15.3 — host-field-precedence policy decision; this BC formalizes
  the SDK versioning signal for that decision

## Story Anchor

S-10.05 (Wave 2: Plugin schema migration — SDK MAJOR semver bump for
D-15.3 host-field-precedence semantics per AC-008)

## VP Anchors

(TBD — to be assigned after S-10.05 story authoring completes Phase 1c)

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Plugin pinned to old MAJOR via `vsdd-hook-sdk = "0"` | Cargo does NOT auto-upgrade to 1.0.0; plugin author must update `Cargo.toml` explicitly; old-major plugin continues to compile against old SDK and run against new dispatcher (WASM ABI unchanged per BC-2.01.001) |
| EC-002 | Plugin uses `vsdd-hook-sdk = "*"` wildcard | Cargo may or may not upgrade depending on lockfile and registry state; plugin authors using wildcards accept unpredictable upgrade behavior; this is a plugin packaging issue, not a BC violation |
| EC-003 | `crates/hook-sdk/CHANGELOG.md` does not contain "Breaking Changes" before tag | Postcondition violation; tag creation must be blocked until CHANGELOG is updated |
| EC-004 | MAJOR version bumped but dispatcher D-15.3 not yet active | Order invariant violated; the MAJOR bump MUST accompany the dispatcher D-15.3 activation, not precede it; separate the SDK release from the dispatcher release only if both are clearly documented as requiring simultaneous deployment |
| EC-005 | Plugin emits no host-owned fields (well-behaved plugin) | MAJOR version bump is transparent; plugin compiles and runs identically against new SDK; no `vsdd.internal.host_field_override.v1` events emitted for that plugin |
| EC-006 | `crates/hook-sdk/CHANGELOG.md` for the Wave 2 MAJOR release contains "Breaking Changes" but no "Added / New API" section | Postcondition violation: Wave 2 unconditionally requires both sections per Invariant 2 (Wave 2 is known to add `HookResult::block_with_fix`). Release validation MUST flag the missing "Added / New API" section; tag creation must be blocked until both required sections are present. NOTE: For non-Wave-2 MAJOR releases, EC-006 applies only when the release introduces additive APIs per Postcondition 2. |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `cat crates/hook-sdk/Cargo.toml \| grep "^version"` after Wave 2 release | `version = "1.0.0"` (or `(N+1).0.0` per current MAJOR) — MAJOR incremented, MINOR and PATCH = 0 | version-bump-format |
| `grep "Breaking Changes" crates/hook-sdk/CHANGELOG.md` | Returns at least one match for the new MAJOR version entry | changelog-breaking-changes-present |
| `grep "Added\|New API\|block_with_fix" crates/hook-sdk/CHANGELOG.md` | Returns at least one match documenting the `block_with_fix` additive API addition | changelog-added-new-api-present |
| `grep "host.field.precedence\|D-15.3\|host_field_override" crates/hook-sdk/CHANGELOG.md` | Returns at least one match referencing the host-field-precedence change | changelog-host-field-precedence-documented |
| Plugin compiled against old MAJOR version of SDK; deployed against Wave 2 dispatcher | Plugin runs without WASM ABI error; host-field-precedence applies silently; `vsdd.internal.host_field_override.v1` emitted if plugin sets host-owned fields | backward-abi-compat |
| **Misimplementation distinguisher:** MAJOR not bumped; only MINOR bump | `grep "^version" crates/hook-sdk/Cargo.toml` returns `N.X+1.0` instead of `(N+1).0.0`; this violates Postcondition 1 — must be caught by release validation | misimplementation-witness-minor-not-major |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — Phase 1c) | SDK `Cargo.toml` version is incremented by MAJOR at Wave 2 release | integration / CI check: parse `Cargo.toml` version; assert MAJOR > previous MAJOR, MINOR = 0, PATCH = 0 |
| (TBD) | CHANGELOG contains "Breaking Changes" section for new MAJOR version | grep-based CI check: `grep "Breaking Changes" crates/hook-sdk/CHANGELOG.md` returns non-empty |
| (TBD) | Plugin pinned to old MAJOR does not auto-upgrade via Cargo resolver | property-based: cargo resolve with lockfile; assert old MAJOR version retained until explicit update |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-009 |
| Capability Anchor Justification | CAP-009 ("Author and publish WASM hook plugins using the Rust SDK") per capabilities.md §CAP-009. BC-2.06.001 governs the SDK version bump that signals an ABI-level behavioral change to plugin authors — specifically, that host-field-precedence (ADR-015 D-15.3) is now active in the dispatcher. CAP-009 describes the SDK as the interface through which "a third-party plugin author can add a dependency and ship a `.wasm` without touching the dispatcher" (capabilities.md §CAP-009). The MAJOR version bump is the public-facing contract signal within that SDK surface: it is the mechanism by which the `vsdd-hook-sdk` crate communicates breaking behavioral changes to plugin authors per SemVer convention. The CHANGELOG migration guidance (Postcondition 2) is the SDK's documentation surface for the same audience. Both artifacts — the `Cargo.toml` version field and the CHANGELOG — are core outputs of CAP-009's "author and publish" capability. CAP-029 governs the dispatcher's single-stream emission concern (a different surface); this BC is squarely the SDK ABI versioning concern under CAP-009's scope. |
| L2 Domain Invariants | No domain invariants directly enforced. The host-field-precedence semantics this bump signals are governed by BC-1.12.005 (dispatcher side); this BC governs the SDK publication contract. |
| Architecture Module | SS-02 — `crates/hook-sdk/Cargo.toml` (version field); `crates/hook-sdk/CHANGELOG.md` (migration guidance) |
| Stories | S-10.05 (Wave 2: Plugin schema migration — SDK MAJOR semver bump per AC-008) |
| Epic | E-10 (Single-stream OTel-aligned event emission) |
| ADR | ADR-015 D-15.3 (host-field-precedence policy — the behavioral change this MAJOR bump signals) |

### Purity Classification

| Property | Assessment |
|----------|-----------|
| I/O operations | NO — this BC is a PUBLICATION CONSTRAINT on the SDK release artifact (Cargo.toml version field and CHANGELOG.md). It governs static file content, not runtime I/O. |
| Global state access | N/A |
| Deterministic | YES — the version bump and CHANGELOG content are deterministic properties of the release artifact |
| Thread safety | N/A |
| Overall classification | Behavioral invariant on release artifact content (SDK Cargo.toml + CHANGELOG.md); verified by CI checks on file content |

### Token Budget

| Item | Estimate |
|------|---------|
| BC files (this BC) | ~1 BC |
| Story anchor | S-10.05 (Wave 2 plugin schema migration) |
| Subsystem | SS-02 |

## Changelog

| Version | Date | Description |
|---------|------|-------------|
| 1.0 | 2026-05-06 | Initial authoring (D-321; ADR-015 D-15.3 SDK MAJOR semver bump for host-field-precedence Wave 2 release). |
| 1.1 | 2026-05-06 | D-322 — F-14 fix: `vcs.*` wildcard in migration guidance (Postcondition 2) expanded to explicit four VCS Resource fields (`vcs.repository.url.full`, `vcs.repository.name`, `vcs.provider.name`, `vcs.owner.name`) per BC-1.12.003 Postcondition 1; parenthetical note added clarifying `vcs.ref.head.*` fields are per-event identity fields (BC-1.12.004), not Resource attributes. |
| 1.2 | 2026-05-06 | D-325 — F-9 fix: Postcondition 2 four VCS fields split into separate backtick code-spans (`vcs.repository.url.full`, `vcs.repository.name`, `vcs.provider.name`, `vcs.owner.name`) — prior version had all four in a single backtick span. F-7 sweep: L2 Capability cell paraphrase removed — cell now just `CAP-009`. F-14 sweep: Architecture Anchors reviewed; references are to TOML/MD file paths (not code symbols); no stable-anchor disclaimer needed. |
| 1.3 | 2026-05-06 | D-326 (D-4) — rc.12 alignment: CHANGELOG requirement amended to add "Added / New API" subsection alongside "Breaking Changes" and "Migration Guide". Policy: Breaking Changes REQUIRED when API removes/changes signatures; Added/New API REQUIRED when API gains new constructors/methods/types (additive, non-breaking); Migration Guide REQUIRED when Breaking Changes is non-empty. Wave 2 MAJOR release MUST document `HookResult::block_with_fix` in the Added/New API section. Invariant 2 updated to require both sections. EC-006 added. Test vector `changelog-added-new-api-present` added. |
| 1.4 | 2026-05-06 | D-328 — E-10 pass-5 F-4 + F-9 combined. F-4: BC-2.06.001 already used `<code>` placeholder (matches HOST_ABI.md + rc.12 audit) — no format-string change needed here; confirmed aligned. F-9: Invariant 2 scoped explicitly to Wave 2 SDK MAJOR release (Wave 2 unconditionally has `HookResult::block_with_fix`; future MAJOR releases follow PC 2's conditional rule for "Added / New API"). EC-006 updated to match: flags missing "Added / New API" for Wave 2 only; adds NOTE for non-Wave-2 MAJOR releases deferring to PC 2 conditional. Postcondition 2 conditional language unchanged. |
