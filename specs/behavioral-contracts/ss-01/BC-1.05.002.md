---
document_type: behavioral-contract
level: L3
version: "2.4"
status: draft
producer: codebase-analyzer
timestamp: 2026-04-25T00:00:00
last_amended: "2026-08-11 (v2.4)"
phase: 1.4b
inputs: [bc-id-mapping.md, pass-3-behavioral-contracts.md]
input-hash: "127adec"
traces_to: bc-id-mapping.md
origin: brownfield
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts.md:174"
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

# Behavioral Contract BC-1.05.002: exec_subprocess denies binaries not on allow-list

## Description

When `exec_subprocess` is called with a command whose basename is not in the entry's `binary_allow`, it returns CAPABILITY_DENIED and emits a denial event with `reason = "binary_not_on_allow_list"` and the offending command.

**v2.0 security-model clarification — load-time resolution (ADR-043 Decision 1):** The `binary_allow` list uses basename entries in the registry (operators write `binary_allow = ["git"]`). At registry-load time the dispatcher resolves each basename entry to an absolute path via a hardcoded trusted-prefix list (ADR-043 Decision 1). The in-memory `binary_allow` then contains resolved absolute paths. At spawn time, `binary_allowed` matches the guest-supplied `cmd` (by basename comparison against each resolved entry's basename, or by exact equality) and returns `AllowResult::Allowed(resolved_path)` (ADR-043 Decision 3: `AllowResult` return). The returned resolved path is what `Command::new` receives — the guest `cmd` string is discarded after the allow-list check. This means a guest supplying `cmd="/tmp/evil/git"` where `binary_allow=["git"]` resolved to `/usr/bin/git` will execute `/usr/bin/git`, not `/tmp/evil/git`. This is the intended security behavior: the sandbox controls which binary is spawned, not the guest.

**Current production state (pre-ADR-043):** The current `binary_allowed` function returns `bool` and `execute_bounded` receives the raw guest `cmd`. The security gap is structural: a guest declaring a valid basename can supply any absolute path with that basename and have it executed. Practical exploitability is LOW (all production `cmd` values are compile-time literals, none derived from payload), but structural severity is HIGH (CWE-706). ADR-043 Decision 3 closes this structurally.

## Preconditions

1. Plugin's exec_subprocess capability is declared.
2. The guest-supplied `cmd`'s basename is compared against the resolved allow-list entries. **Deny path:** if no entry's resolved basename matches, Postconditions 1–2 apply (CAPABILITY_DENIED + denial event). **Allow path:** if an entry matches, Postcondition 3 applies (`binary_allowed` returns the resolved path; that path is forwarded to subsequent checks and `Command::new`).

## Postconditions

1. Returns CAPABILITY_DENIED.
2. Emits denial event with `reason = "binary_not_on_allow_list"`, `command = <cmd>`.
3. (v2.0, post-ADR-043) When the allow-list check passes, `Command::new` receives the dispatcher-resolved absolute path, not the guest `cmd`. The guest `cmd` is consumed for basename extraction only and then discarded.

## Invariants

1. **(PENDING ADR-043 Decisions 1+3 implementation — current code violates the second and third sentences; see Description §Current production state for pre-ADR-043 behavior)** Registry entries specify allowed binary basenames; operators declare `binary_allow = ["git"]` independently of where git is installed. At registry-load time (ADR-043 Decision 1), each basename entry is resolved to an absolute path via the trusted-prefix list. At spawn time, the guest `cmd` is matched by basename against the resolved entries; only the dispatcher-resolved path is ever passed to `Command::new`. The guest string cannot substitute a different binary of the same basename at a different location.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Allow-list entry is `"git"` (resolved to `/usr/bin/git`); guest cmd is `"/usr/bin/git"` | Allowed: `binary_allowed` returns `AllowResult::Allowed("/usr/bin/git")` (exact match of guest path to resolved entry). `Command::new("/usr/bin/git")` executes. |
| EC-002 | Allow-list entry is `"git"` (resolved to `/usr/bin/git`); guest cmd is `"/tmp/evil/git"` | Allowed by basename check: basename `"git"` matches resolved entry basename `"git"`; `binary_allowed` returns `AllowResult::Allowed("/usr/bin/git")`. `Command::new("/usr/bin/git")` executes — NOT `Command::new("/tmp/evil/git")`. Guest path is discarded; dispatcher-controlled path is spawned. This is the key security invariant of Option C. |
| EC-003 | Allow-list entry is `"git"` (resolved to `/usr/bin/git`); guest cmd is `"curl"` | DENIED: basename `"curl"` does not match `"git"`; `binary_allowed` returns `AllowResult::Denied`; `emit_denial("binary_not_on_allow_list")`; CAPABILITY_DENIED (-1). |
| EC-004 | Windows host: allow-list entry is `"git"` (basename); plugin calls `exec_subprocess` | ADR-043 Decisions 1–5 are scoped to `#[cfg(unix)]` and do NOT apply on Windows. Windows retains the current (pre-ADR-043) behavior: `Command::new("git")` with OS PATH lookup at spawn time. Bare-name `binary_allow` entries (e.g., `"git"`) continue to work on Windows exactly as today; no dispatcher-side load-time resolution occurs; no operator change is required. Ratifying ADR-043 introduces zero Windows behavior change. Absolute-path entries are permitted on any platform via the `ALREADY-ABSOLUTE-and-exists` outcome but are NOT required on Windows. Windows binary resolution (trusted-prefix variants, `.exe`/`PATHEXT` suffix handling) is a future architecture work item — see ADR-043 §Downstream Routing. |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `binary_allow=["git"]` (resolved to `/usr/bin/git`); cmd=`"curl"` | DENIED, reason=`"binary_not_on_allow_list"` | error |
| `binary_allow=["git"]` (resolved to `/usr/bin/git`); cmd=`"/usr/bin/git"` | `binary_allowed` returns `AllowResult::Allowed("/usr/bin/git")`; `Command::new("/usr/bin/git")` | happy-path |
| `binary_allow=["git"]` (resolved to `/usr/bin/git`); cmd=`"/tmp/evil/git"` | `binary_allowed` returns `AllowResult::Allowed("/usr/bin/git")`; `Command::new("/usr/bin/git")` spawned (not the evil path) | security-control |
| `binary_allow=["git"]` (resolved to `/usr/bin/git`); cmd=`"git"` | `binary_allowed` returns `AllowResult::Allowed("/usr/bin/git")`; `Command::new("/usr/bin/git")` | happy-path |

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
| **Path** | `host/exec_subprocess.rs::binary_allowed` |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-behavioral-contracts.md` line `174` |

#### Evidence Types Used

- guard clause (binary_allowed function)

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

- v2.4 (2026-08-11): AllowResult enum sibling-site sweep per TD-VSDD-060 (product-owner; architect direction). `binary_allowed` return type updated from `Option<String>` to `AllowResult` throughout: Description §v2.0 (`Option<String>` reference), EC-001 (`Some` → `AllowResult::Allowed`), EC-002 (`Some` → `AllowResult::Allowed`), EC-003 (`None` → `AllowResult::Denied`), canonical test vector rows 2–4 (`Some` → `AllowResult::Allowed`).
- v2.3 (2026-08-11): Adversary review DO-NOT-RATIFY remediation (product-owner; team-lead direction). (1) Frontmatter: added `last_amended: "2026-08-11 (v2.3)"` per POLICY 14/17 Leg-4; `modified` corrected from `pending-D-NNN-option-c-security-model-adjudication` to `D-{TBD-option-c-security-model-adjudication}` per POLICY 16 sentinel form. (2) HIGH 7 — Precondition 2 widened: replaced deny-only framing ("does not match any resolved allow-list entry") with full state-space coverage — deny path (no entry matches → PC1–PC2) and allow path (entry matches → Postcondition 3), matching the scope of EC-001, EC-002, and the three allow-path canonical test vectors already in the BC.
- v2.2 (2026-08-11): ADR-043 v1.4 reconciliation — Windows EC-004 corrected (product-owner; team-lead direction). EC-004: removed incorrect "Windows deployments MUST use absolute `binary_allow` entries" requirement. ADR-043 Decisions 1–5 are `#[cfg(unix)]`-scoped; Windows retains current behavior (bare-name `Command::new` + OS PATH); no operator action required; Ratifying ADR-043 introduces zero Windows behavior change. Future architecture work item handles Windows trusted-prefix paths. Absolute-path entries permitted but not required on Windows.
- v2.1 (2026-08-11): Annotation consistency fix (product-owner; team-lead correction C-3). Added "(PENDING ADR-043 Decisions 1+3 implementation — current code violates the second and third sentences; see Description §Current production state for pre-ADR-043 behavior)" prefix to Invariant 1. Readers of this active BC must not be able to mistake the intended post-ADR-043 behavior for current production behavior.
- v2.0 (2026-08-11): Option C security-model adjudication (product-owner). (1) Invariant 1 rewritten: from "basename only, never full path" to two-part statement — operator declares basenames; at spawn time only dispatcher-resolved absolute path is passed to `Command::new`, guest cmd is discarded. (2) Added v2.0 security-model clarification to Description: ADR-043 Decision 1 load-time resolution + Decision 3 `binary_allowed` signature change to `Option<String>`; current production gap (CWE-706, LOW practical exploitability, HIGH structural severity) documented. (3) Precondition 2 updated to "resolved entry" framing. (4) Added Postcondition 3: resolved path used by `Command::new` post-ADR-043. (5) EC-001 updated for resolved-path mechanics; EC-002 added (the key security invariant — evil-path guest cmd produces dispatcher-controlled spawn); EC-003 (deny path); EC-004 (Windows gap). (6) Canonical test vectors updated for new `binary_allowed` `Option<String>` return signature.
