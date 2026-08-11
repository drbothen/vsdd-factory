---
document_type: behavioral-contract
level: L3
version: "2.4"
status: draft
producer: codebase-analyzer
timestamp: 2026-04-25T00:00:00
last_amended: "2026-08-11 (v2.4)"
phase: 1.4b
inputs: [bc-id-mapping.md, pass-3-deep-rust-tests.md]
input-hash: "0f4f6a3"
traces_to: bc-id-mapping.md
origin: brownfield
extracted_from: ".factory/phase-0-ingestion/pass-3-deep-rust-tests.md:420"
subsystem: "SS-01"
capability: "CAP-TBD"
lifecycle_status: active
introduced: v1.0.0-beta.4
modified: [D-972]
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# Behavioral Contract BC-1.05.028: factory-dispatcher::host::exec_subprocess::binary_allow_matches_basename — allow-list match is path-independent for the operator; resolved path returned at spawn time

## Description

**v1 behavior (pre-ADR-043):** `binary_allowed("/usr/bin/git", &["git"])` and `binary_allowed("git", &["git"])` both return `true`; `binary_allowed("curl", &["git"])` returns `false`. The current implementation returns `bool`; the allow-list input contains raw registry strings (basenames). Operators declare `binary_allow = ["git"]` regardless of where git is installed.

**v2 behavior (post-ADR-043 Decision 1 + Decision 3, updated for v1.5 `AllowResult` enum):** `binary_allowed` signature changes from `(cmd: &str, allow: &[String]) -> bool` to `(cmd: &str, allow: &[String]) -> AllowResult`. The `allow` input contains dispatcher-resolved absolute paths (e.g., `["/usr/bin/git"]`) or `BINARY_UNRESOLVABLE` sentinel entries, not raw registry basenames — resolution happens at registry-load time before `binary_allowed` is called. `binary_allowed` returns `AllowResult::Allowed(resolved_path)` if the guest `cmd`'s basename matches any resolved entry's basename (or if the full guest `cmd` exactly equals a resolved entry); `AllowResult::BinaryUnresolvable` if the matching entry holds a sentinel (stored for either BINARY-UNRESOLVABLE-ABSOLUTE-MISSING or BINARY-UNRESOLVABLE-NAME-NOT-FOUND load-time outcomes); `AllowResult::Denied` if no entry matches. The resolved path carried by `AllowResult::Allowed` is the value used by `Command::new`. The operator UX is unchanged: `binary_allow = ["git"]` in the registry is still the correct declaration.

The test `binary_allow_matches_basename` in `exec_subprocess.rs::tests::binary_allow_matches_basename` MUST be rewritten to exercise the new `AllowResult` return type with resolved-path allow-list inputs, covering all three variants (`Allowed`, `Denied`, `BinaryUnresolvable`). The existing test passes `["git".to_string()]` (basename) as the allow argument; after ADR-043 the allow argument will contain `["/usr/bin/git".to_string()]` (resolved absolute path).

## Preconditions

1. `binary_allowed(cmd, allow_list)` invoked with various cmd shapes. `allow_list` contains resolved absolute paths (post-ADR-043) or raw basenames (pre-ADR-043).

## Postconditions

1. Match is on file basename of guest `cmd` vs. basename of each resolved allow-list entry (or exact equality of guest `cmd` to a resolved entry).
2. Pre-ADR-043: returns `bool`. Post-ADR-043 v1.5: returns `AllowResult` — `AllowResult::Allowed(resolved_path)` on match, `AllowResult::Denied` on no match, `AllowResult::BinaryUnresolvable` if the matching entry has a sentinel stored at load time.
3. The returned resolved path (not the guest `cmd`) is the value used by `Command::new` in `exec_subprocess.rs::execute_bounded`.

## Invariants

1. **(PENDING ADR-043 Decisions 1+3 implementation — current code satisfies only the first sentence; see Description §v1 behavior for pre-ADR-043 state)** Allow-list is path-independent from the operator's perspective: operators declare `binary_allow = ["git"]` regardless of git's installation location. The dispatcher resolves each entry to an absolute path at registry-load time (ADR-043 Decision 1). At spawn time `binary_allowed` compares the guest cmd's basename against the basename of each resolved entry; only the dispatcher-resolved path is returned and subsequently spawned. A guest supplying `cmd="/tmp/evil/git"` where `binary_allow=["git"]` resolved to `/usr/bin/git` executes `/usr/bin/git`, not the guest-supplied path.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Post-ADR-043: guest `cmd="/usr/bin/git"`, allow contains resolved `["/usr/bin/git"]` | `binary_allowed` returns `AllowResult::Allowed("/usr/bin/git")` — exact match. |
| EC-002 | Post-ADR-043: guest `cmd="git"`, allow contains resolved `["/usr/bin/git"]` | `binary_allowed` returns `AllowResult::Allowed("/usr/bin/git")` — basename `"git"` matches basename of resolved entry `"git"`. |
| EC-003 | Post-ADR-043: guest `cmd="/tmp/evil/git"`, allow contains resolved `["/usr/bin/git"]` | `binary_allowed` returns `AllowResult::Allowed("/usr/bin/git")` — basename match passes; dispatcher path returned, not guest path. |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Pre-ADR-043: `binary_allowed("/usr/bin/git", ["git"])` | `true` | happy-path (current) |
| Pre-ADR-043: `binary_allowed("curl", ["git"])` | `false` | error (current) |
| Post-ADR-043: `binary_allowed("/usr/bin/git", ["/usr/bin/git"])` | `AllowResult::Allowed("/usr/bin/git")` | happy-path (target) |
| Post-ADR-043: `binary_allowed("git", ["/usr/bin/git"])` | `AllowResult::Allowed("/usr/bin/git")` | happy-path (target) |
| Post-ADR-043: `binary_allowed("/tmp/evil/git", ["/usr/bin/git"])` | `AllowResult::Allowed("/usr/bin/git")` | security-control (target) |
| Post-ADR-043: `binary_allowed("curl", ["/usr/bin/git"])` | `AllowResult::Denied` | error (target) |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6b) | | |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD (anchor in Phase 1.5) |
| L2 Domain Invariants | TBD |
| Architecture Module | SS-01 — `crates/factory-dispatcher/src/host/exec_subprocess.rs` |
| Stories | TBD (re-anchor in Phase 1.8 from S-N.MM stories) |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `crates/factory-dispatcher/src/host/exec_subprocess.rs::tests::binary_allow_matches_basename` |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-deep-rust-tests.md` line `420` |

#### Evidence Types Used

- assertion (unit test)

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | TBD (Phase 1.6b will refine) |
| **Global state access** | TBD |
| **Deterministic** | TBD |
| **Thread safety** | TBD |
| **Overall classification** | TBD |

#### Refactoring Notes

(TBD — to be assessed in Phase 1.6b verification properties pass)

## Changelog

- v2.4 (2026-08-11): Canonical test vector rows 3–6 updated to `AllowResult` (product-owner; missed in v2.3 sweep). Post-ADR-043 target column: `Some("/usr/bin/git")` → `AllowResult::Allowed("/usr/bin/git")` (rows 3–5); `None` → `AllowResult::Denied` (row 6).
- v2.3 (2026-08-11): AllowResult enum sibling-site sweep per TD-VSDD-060 (product-owner; architect direction). `binary_allowed` return type updated from `Option<String>` to `AllowResult { Allowed(String), Denied, BinaryUnresolvable }` throughout: Description §v2 behavior, §test note, Postcondition 2, EC-001, EC-002, EC-003. `BinaryUnresolvable` variant documented as third return case (sentinel match).
- v2.2 (2026-08-11): Adversary review DO-NOT-RATIFY remediation (product-owner; team-lead direction). (1) Frontmatter: added `last_amended: "2026-08-11 (v2.2)"` per POLICY 14/17 Leg-4; `modified` corrected from `pending-D-NNN-option-c-security-model-adjudication` to `D-{TBD-option-c-security-model-adjudication}` per POLICY 16 sentinel form. (2) MEDIUM 9 — §Source Evidence: removed `(lines 401–406)` line-number citation per TD-VSDD-091; stable anchor is the function name `crates/factory-dispatcher/src/host/exec_subprocess.rs::tests::binary_allow_matches_basename`.
- v2.1 (2026-08-11): Annotation consistency fix (product-owner; team-lead correction C-3). Added "(PENDING ADR-043 Decisions 1+3 implementation — current code satisfies only the first sentence; see Description §v1 behavior for pre-ADR-043 state)" prefix to Invariant 1. Readers of this active BC must not be able to mistake the intended post-ADR-043 behavior for current production behavior.
- v2.0 (2026-08-11): Option C security-model adjudication (product-owner). (1) Title updated to reflect that path-independence is the operator UX guarantee, not the internal implementation mechanism. (2) Description extended with v2 behavior: `binary_allowed` signature changes from `bool` to `Option<String>` (ADR-043 Decision 3); allow-list input becomes resolved absolute paths after ADR-043 Decision 1 load-time resolution; test `binary_allow_matches_basename` must be rewritten for new signature. (3) Invariant 1 rewritten: operator-perspective path-independence preserved; internal mechanism changes to basename-of-resolved-entry comparison returning resolved path. (4) EC-001/EC-002/EC-003 added for post-ADR-043 scenarios. (5) Canonical test vectors extended with post-ADR-043 `Option<String>` targets alongside pre-ADR-043 current behavior.
