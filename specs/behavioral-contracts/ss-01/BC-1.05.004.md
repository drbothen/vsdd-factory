---
document_type: behavioral-contract
level: L3
version: "2.3"
status: draft
producer: codebase-analyzer
timestamp: 2026-04-25T00:00:00
last_amended: "2026-08-11 (v2.3)"
phase: 1.4b
inputs: [bc-id-mapping.md, pass-3-behavioral-contracts.md]
input-hash: "127adec"
traces_to: bc-id-mapping.md
origin: brownfield
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts.md:186"
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

# Behavioral Contract BC-1.05.004: exec_subprocess refuses setuid/setgid binaries categorically (Unix)

## Description

If the resolved binary path has a setuid or setgid bit set (Unix), `exec_subprocess` returns CAPABILITY_DENIED regardless of the allow-list contents. This is design Q4 resolution.

**"Resolved binary path" is normative (added v2.0):** "Resolved" means the absolute path as produced by ADR-043 Decision 1 load-time trusted-prefix resolution — NOT a bare basename or CWD-relative path. The function `refuse_setuid` in `exec_subprocess.rs::refuse_setuid` MUST receive this absolute path. Prior to ADR-043 Decision 1 implementation, bare-name registry entries (e.g., `cmd="git"`) cause `refuse_setuid` to call `std::fs::metadata("git")`, which stats the CWD-relative path `./git`. That path is almost always absent, so `fs::metadata` returns `Err` and `refuse_setuid` returns `false` — rendering the setuid gate entirely inert for all production registry entries. ADR-043 Decision 3 explicitly repairs this by threading the resolved path from `binary_allowed` through `run()` to `refuse_setuid`. This repair is NOT incidental — Decision 1 resolves allow-list entries at load time but does NOT change the path received by `refuse_setuid` in `run()`; Decision 3's full-substitution rule explicitly passes the resolved path from `binary_allowed`'s `AllowResult::Allowed(resolved_path)` return through `run()` to `refuse_setuid`, ensuring `refuse_setuid` receives an absolute path on every invocation.

**Current production status (pre-ADR-043):** This BC is NOT satisfied in production for any bare-name `binary_allow` entry. The gate is a compile-time no-op for the actual registry population.

## Preconditions

1. Resolved binary path has setuid OR setgid bit (Unix). "Resolved" means the ADR-043 Decision 1 absolute path, not the raw guest-supplied `cmd` string.
2. Plugin has a valid `Capabilities.exec_subprocess` block and `cmd` has passed the binary_allow check.

## Postconditions

1. Returns CAPABILITY_DENIED.
2. Even if the binary is on `binary_allow`, the call fails.
3. `refuse_setuid` MUST receive the resolved absolute path from ADR-043 Decision 1 load-time resolution. Calling `refuse_setuid` with a bare basename or CWD-relative path is a defect (the setuid gate is silently inert for such inputs on all production registry entries). This postcondition is satisfied only after ADR-043 Decision 1 is implemented; until then, Invariant 1 holds only in theory for absolute-path registry entries, which the current registry does not produce.

## Invariants

1. Setuid/setgid binaries are never executed via exec_subprocess on Unix — provided `refuse_setuid` receives the resolved absolute path (Postcondition 3). Prior to ADR-043 Decision 1, this invariant does NOT hold in production: bare-name entries reach `refuse_setuid` as CWD-relative paths; `fs::metadata` returns Err; the gate returns false and permits execution without checking mode bits.
2. On Windows (`x86_64-pc-windows-msvc`), this invariant is satisfied vacuously. The Windows build compiles `refuse_setuid` as an unconditional no-op (`#[cfg(not(unix))] fn refuse_setuid(_cmd: &str) -> bool { false }`). Windows does not have setuid/setgid semantics; binary spawn safety on Windows relies on BC-1.05.002 allow-list enforcement and OS-level ACLs.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Windows host (`x86_64-pc-windows-msvc`) | Windows does not have setuid/setgid bits. `refuse_setuid` compiles as `#[cfg(not(unix))] fn refuse_setuid(_cmd: &str) -> bool { false }` — always returns false, always permits execution past this gate. No TBD: this is the correct and complete behavior. Binary spawn safety on Windows is enforced solely via BC-1.05.002 (allow-list) and Windows OS-level access controls. ADR-043 Decisions 1–5 are scoped to `#[cfg(unix)]` and do NOT apply on Windows; the dispatcher retains current behavior on Windows (`Command::new(bare_name)` with OS PATH lookup at spawn time). Bare-name `binary_allow` entries continue to work on Windows exactly as today; no operator change is required to ratify ADR-043. Absolute-path entries are permitted on any platform via the `ALREADY-ABSOLUTE-and-exists` outcome but are NOT required on Windows. Windows binary resolution (trusted-prefix variants, `.exe`/`PATHEXT` handling, this EC) is a future architecture work item — see ADR-043 §Downstream Routing. |
| EC-002 | Pre-ADR-043: `cmd = "git"` (bare basename, as in current registry); setuid bit set on `/usr/bin/git` | `refuse_setuid("git")` stats `./git` (CWD-relative); almost certainly ENOENT; returns false; setuid binary executes unguarded. Gate is inert. This is the current production defect closed by ADR-043 Decision 1. |
| EC-003 | Post-ADR-043: `cmd` has been resolved to `/usr/bin/git`; setuid bit set on that path | `refuse_setuid("/usr/bin/git")` stats the real absolute path; mode bits checked correctly; returns true; CAPABILITY_DENIED emitted; execution refused. Invariant 1 satisfied. |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Setuid binary on allow-list (Unix) | DENIED | error |
| TBD | TBD | happy-path |
| TBD | TBD | edge-case |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6b) | | |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD (anchor in Phase 1.5) |
| L2 Domain Invariants | TBD |
| Architecture Module | SS-01 — `crates/factory-dispatcher/src/host/exec_subprocess.rs` (`refuse_setuid`, Unix-only) |
| Stories | TBD (re-anchor in Phase 1.8 from S-N.MM stories) |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `host/exec_subprocess.rs::refuse_setuid` (Unix-only); design Q4 resolution |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-behavioral-contracts.md` line `186` |

#### Evidence Types Used

- guard clause (refuse_setuid)

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

- v2.3 (2026-08-11): AllowResult enum sibling-site sweep per TD-VSDD-060 (product-owner; architect direction). Description §"Resolved binary path" updated: replaced "`binary_allowed`'s `Option<String>` return" with "`binary_allowed`'s `AllowResult::Allowed(resolved_path)` return".
- v2.2 (2026-08-11): Adversary review DO-NOT-RATIFY remediation (product-owner; team-lead direction). (1) Frontmatter: added `last_amended: "2026-08-11 (v2.2)"` per POLICY 14/17 Leg-4; `modified` corrected from `pending-D-NNN-option-c-security-model-adjudication` to `D-{TBD-option-c-security-model-adjudication}` per POLICY 16 sentinel form. (2) HIGH 4 — Description: replaced "ADR-043 Decision 1 incidentally repairs this" with "ADR-043 Decision 3 explicitly repairs this"; clarified that Decision 1 alone does NOT deliver the resolved path to `refuse_setuid` — Decision 3's full-substitution rule in `run()` is the required explicit code change.
- v2.1 (2026-08-11): ADR-043 v1.4 reconciliation — Windows EC-001 corrected (product-owner; team-lead direction). EC-001: removed incorrect "Windows deployments MUST use absolute `binary_allow` entries" requirement. ADR-043 Decisions 1–5 are `#[cfg(unix)]`-scoped; Windows retains current behavior (bare-name `Command::new` + OS PATH); no operator action required; ratifying ADR-043 introduces zero Windows behavior change. Added: absolute-path entries permitted but not required on Windows; future architecture work item handles Windows trusted-prefix paths and `.exe`/PATHEXT handling (ADR-043 §Downstream Routing).
- v2.0 (2026-08-11): Option C security-model adjudication (product-owner). (1) Added normative statement that "resolved binary path" means ADR-043 Decision 1 load-time trusted-prefix-resolved absolute path, not bare basename. (2) Added Postcondition 3: `refuse_setuid` MUST receive resolved absolute path; calling with bare name is a defect; gate is inert pre-ADR-043. (3) Extended Invariant 1 with precondition on resolution; added Invariant 2 (Windows vacuous satisfaction). (4) Resolved EC-001 Windows TBD: Windows setuid semantics absent; `#[cfg(not(unix))]` no-op is correct and complete; Windows deployments require absolute `binary_allow` entries until ADR-043 adds Windows trusted-prefix paths. (5) Added EC-002 (pre-ADR-043 production defect: bare name causes inert gate) and EC-003 (post-ADR-043 correct behavior).
