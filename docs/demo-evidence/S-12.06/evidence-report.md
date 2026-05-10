---
story_id: S-12.06
title: Context Injection Contract — HOST_ABI.md Documentation
document_type: demo-evidence-report
recorded_by: demo-recorder
recorded_at: 2026-05-07
bcs:
  - BC-4.12.001
  - BC-4.12.002
  - BC-4.12.003
  - BC-4.12.004
  - BC-4.12.005
  - BC-1.13.001
status: complete
---

# Demo Evidence Report — S-12.06

## Summary

11 demos recorded (10 per-AC + 1 cross-cutting bats suite).
All 10 acceptance criteria have coverage. The bats suite recording confirms
48/48 tests green across all ACs simultaneously. No demos skipped.

## AC Coverage Table

| AC ID | Demo File | Description | Status | BC Trace |
|-------|-----------|-------------|--------|----------|
| AC-001 | [AC-001-section-heading.gif](AC-001-section-heading.gif) | grep showing `## Context Injection Contract` section exists at line 463 | PASS | BC-1.13.001 |
| AC-002 | [AC-002-resolver-lifecycle.gif](AC-002-resolver-lifecycle.gif) | Resolver Lifecycle subsection (load-once, mtime cache invalidation, per-dispatch Store isolation) | PASS | BC-4.12.001 |
| AC-003 | [AC-003-abi-types.gif](AC-003-abi-types.gif) | Resolver ABI Types subsection: ResolverInput/Output types, RESOLVER_ABI_VERSION = 1, OD-5 invariant | PASS | BC-4.12.002 |
| AC-004 | [AC-004-capability.gif](AC-004-capability.gif) | Capability Model subsection: path_allow declarations, deny-by-default, read-only linker, telemetry | PASS | BC-4.12.003 |
| AC-005 | [AC-005-error-isolation.gif](AC-005-error-isolation.gif) | Error and Crash Isolation subsection: resolver.error event, resolver_name/error_kind fields, dispatch continues | PASS | BC-4.12.004 |
| AC-006 | [AC-006-merging.gif](AC-006-merging.gif) | Merging Contract subsection: additive overlay, whole-value replacement, None=key absent, resolver.merge_collision event | PASS | BC-4.12.005 |
| AC-007 | [AC-007-needs-context.gif](AC-007-needs-context.gif) | needs_context Field in hooks-registry.toml: registration mechanism, zero-overhead path | PASS | BC-1.13.001 |
| AC-008 | [AC-008-factory-agnostic.gif](AC-008-factory-agnostic.gif) | bats AC-008 factory-agnostic test: section contains no forbidden vsdd-factory domain terms | PASS | BC-1.13.001 INV1, BC-4.12.002 PC9 |
| AC-009 | [AC-009-cross-references.gif](AC-009-cross-references.gif) | Cross-References subsection: ADR-018 + BC-1.13.001, BC-4.12.001 through BC-4.12.005 (6 BCs) | PASS | BC-4.12.001 through BC-4.12.005 |
| AC-010 | [AC-010-sdk-authoring.gif](AC-010-sdk-authoring.gif) | SDK Authoring Surface subsection: resolver-authoring feature, #[resolver] macro, SDK surface docs | PASS | BC-4.12.002 PC5, PC8 |
| cross-cutting | [bats-suite-green.gif](bats-suite-green.gif) | bats resolver-host-abi-context-injection.bats — 48/48 ok covering all ACs | PASS | BC-4.12.001 through BC-4.12.005, BC-1.13.001 |

## Artifact Inventory

| File | Type | Size |
|------|------|------|
| AC-001-section-heading.tape | VHS script | 720 B |
| AC-001-section-heading.gif | Recording | 143 KB |
| AC-002-resolver-lifecycle.tape | VHS script | 648 B |
| AC-002-resolver-lifecycle.gif | Recording | 43 KB |
| AC-003-abi-types.tape | VHS script | 740 B |
| AC-003-abi-types.gif | Recording | 78 KB |
| AC-004-capability.tape | VHS script | 648 B |
| AC-004-capability.gif | Recording | 43 KB |
| AC-005-error-isolation.tape | VHS script | 652 B |
| AC-005-error-isolation.gif | Recording | 48 KB |
| AC-006-merging.tape | VHS script | 754 B |
| AC-006-merging.gif | Recording | 78 KB |
| AC-007-needs-context.tape | VHS script | 657 B |
| AC-007-needs-context.gif | Recording | 46 KB |
| AC-008-factory-agnostic.tape | VHS script | 665 B |
| AC-008-factory-agnostic.gif | Recording | 49 KB |
| AC-009-cross-references.tape | VHS script | 712 B |
| AC-009-cross-references.gif | Recording | 433 KB |
| AC-010-sdk-authoring.tape | VHS script | 636 B |
| AC-010-sdk-authoring.gif | Recording | 45 KB |
| bats-suite-green.tape | VHS script | 653 B |
| bats-suite-green.gif | Recording | 240 KB |

Total directory: ~1.3 MB

## Behavioral Contract Traceability

### BC-4.12.001 — Resolver Lifecycle

- AC-002: HOST_ABI.md Resolver Lifecycle subsection documents load-once compile at startup with mtime-based cache invalidation and per-dispatch Store isolation
- cross-cutting bats: tests 1–10 in resolver-host-abi-context-injection.bats exercise lifecycle content

### BC-4.12.002 — Resolver ABI Types

- AC-003: HOST_ABI.md Resolver ABI Types subsection documents ResolverInput, ResolverOutput, RESOLVER_ABI_VERSION = 1, and OD-5 invariant
- AC-010: SDK Authoring Surface subsection documents resolver-authoring feature gate and #[resolver] macro
- AC-008: factory-agnostic test confirms no forbidden vsdd-factory terms in the new section (PC9)

### BC-4.12.003 — Capability Model

- AC-004: HOST_ABI.md Capability Model subsection documents path_allow declarations, deny-by-default, CapabilityDenied return code, read-only linker

### BC-4.12.004 — Error and Crash Isolation

- AC-005: HOST_ABI.md Error and Crash Isolation subsection documents resolver.error event, resolver_name/error_kind fields, dispatch-continues-after-failure contract

### BC-4.12.005 — Merging Contract

- AC-006: HOST_ABI.md Merging Contract subsection documents additive overlay, whole-value replacement, None = key absent, resolver.merge_collision event, duplicate context_key = startup error

### BC-1.13.001 — Dispatcher Loads resolvers-registry.toml

- AC-001: Section heading at line 463 confirms Context Injection Contract is present in HOST_ABI.md
- AC-007: needs_context Field subsection documents zero-overhead path and registration mechanism

## Skipped Demos

None. All 10 ACs have coverage.

## Notes

- VHS 0.10.0 used; only `.gif` output (not `.webm`) — VHS 0.10 single-output constraint per story context
- Font: FiraCode Nerd Font Mono (installed at /Users/jmagady/Library/Fonts/)
- All demos show grep/awk read-only queries against the story worktree — no production code modified
- bats-suite-green recording used 20s Sleep to allow bats to complete all 48 tests before hold frame
- AC-008 uses `bats --filter 'AC-008'` to isolate the single factory-agnostic invariant test (test 35)
- AC-009 large GIF (433 KB) due to Cross-References subsection spanning full terminal height with ADR-018 + 6 BC refs
