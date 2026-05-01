---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-05-01T00:00:00Z
phase: 1d
inputs:
  - .factory/stories/S-8.10-sdk-extension-write-file.md
  - .factory/specs/architecture/ARCH-INDEX.md
  - .factory/specs/architecture/SS-02-hook-sdk.md
  - .factory/specs/behavioral-contracts/BC-INDEX.md
  - .factory/stories/STORY-INDEX.md
  - .factory/stories/epics/E-8-native-wasm-migration.md
  - crates/hook-sdk/src/host.rs
  - crates/hook-sdk/src/lib.rs
  - crates/hook-sdk/src/ffi.rs
  - crates/hook-sdk/Cargo.toml
  - crates/factory-dispatcher/src/lib.rs
  - crates/factory-dispatcher/src/host/mod.rs
  - crates/factory-dispatcher/src/host/read_file.rs
  - crates/factory-dispatcher/src/host/memory.rs
  - crates/factory-dispatcher/src/registry.rs
input-hash: "ad07f36"
traces_to: .factory/stories/epics/E-8-native-wasm-migration.md
phase: 1d
target: story
target_file: .factory/stories/S-8.10-sdk-extension-write-file.md
pass: p1
verdict: SUBSTANTIVE
clock: 0_of_3
findings_critical: 0
findings_high: 5
findings_medium: 7
findings_low: 5
findings_nit: 1
previous_review: null
---

# Adversarial Review: vsdd-factory S-8.10 (Pass p1)

## Finding ID Convention

Finding IDs use the format: `ADV-<CYCLE>-P<PASS>-<SEV>-<SEQ>`

- `ADV`: Fixed prefix identifying adversarial findings
- `<CYCLE>`: Cycle prefix — `S810` for this story
- `<PASS>`: Two-digit pass number (e.g., `P01`)
- `<SEV>`: Severity abbreviation (`CRIT`, `HIGH`, `MED`, `LOW`)
- `<SEQ>`: Three-digit sequence within the pass (e.g., `001`)

Examples: `ADV-S810-P01-HIGH-001`

## Part B — New Findings (or all findings for pass 1)

Pass-1 fresh-context review of S-8.10 v1.0 (341 lines, status=draft, input-hash ad07f36). NEW story authored to unblock D-6 Option A. 18 findings: 0 CRITICAL / 5 HIGH / 7 MEDIUM / 5 LOW / 1 NIT. Verdict SUBSTANTIVE. Clock 0/3.

Top must-fix: HIGH-005 (BC family mis-anchor BC-2.01.005 -> BC-2.02.011), HIGH-002 (BC-2.02.002 max_bytes contradiction), HIGH-001 (broken adversarial-input paths in frontmatter), HIGH-003 (FFI signature ambiguity), HIGH-004 (AC-5 conditional optionality).

### HIGH

#### ADV-S810-P01-HIGH-001: Broken adversarial-input paths in frontmatter
- **Severity:** HIGH
- **Category:** spec-fidelity
- **Location:** .factory/stories/S-8.10-sdk-extension-write-file.md lines 13-14
- **Description:** Frontmatter `inputs:` cite `.factory/stories/adversarial/adv-s8.04-p2.md` and `.factory/stories/adversarial/adv-s8.09-p2.md` — paths do not exist.
- **Evidence:** Actual paths: `.factory/cycles/v1.0-brownfield-backfill/adv-s8.04-p2.md` and `.factory/cycles/v1.0-brownfield-backfill/adv-s8.09-p2.md`.
- **Proposed Fix:** Update lines 13-14 to correct cycle-scoped paths.

#### ADV-S810-P01-HIGH-002: BC-2.02.002 max_bytes contradiction
- **Severity:** HIGH
- **Category:** contradictions
- **Location:** BC-INDEX:153; S-8.10 AC-1, AC-5(d)
- **Description:** AC-1 signature lacks `max_bytes: u32`. BC-2.02.002 declares "Bounded host calls REQUIRE timeout_ms and a byte cap." `write_file` is a bounded host call by intent. AC-5(d) makes the cap conditional, which violates the BC.
- **Evidence:** BC-INDEX:153 — "Bounded host calls REQUIRE timeout_ms and a byte cap." AC-5(d) — "if a cap parameter is introduced; if omitted for MVP, document the omission."
- **Proposed Fix:** Add `max_bytes: u32` to AC-1 signature; OR amend BC-2.02.002 with a scoped exception.

#### ADV-S810-P01-HIGH-003: FFI signature ambiguity
- **Severity:** HIGH
- **Category:** ambiguous-language
- **Location:** S-8.10 AC-2
- **Description:** AC-2 describes input-pointer protocol `(contents_ptr: u32, contents_len: u32)`, but story claims "symmetric to read_file" (which uses output-pointer protocol). The two protocols are not symmetric.
- **Evidence:** read_file uses output-pointer protocol (caller provides buffer); write_file uses input-pointer protocol (caller provides data). These are distinct FFI calling conventions.
- **Proposed Fix:** Pin exact FFI signature in AC-1: `ffi::write_file(path_ptr, path_len, contents_ptr, contents_len, timeout_ms) -> i32`. Drop "symmetric to read_file" framing.

#### ADV-S810-P01-HIGH-004: AC-5 conditional optionality
- **Severity:** HIGH
- **Category:** ambiguous-language
- **Location:** S-8.10 AC-5(d)
- **Description:** AC-5(d) "if a cap parameter is introduced; if omitted for MVP, document the omission" — AC optionality leaves implementer without a clear contract.
- **Evidence:** AC-5(d) wording creates a fork that is unresolved at spec time.
- **Proposed Fix:** Pin max_bytes ON or OFF jointly with HIGH-002 resolution.

#### ADV-S810-P01-HIGH-005: BC family mis-anchor
- **Severity:** HIGH
- **Category:** semantic-anchoring
- **Location:** S-8.10 OQ-1
- **Description:** OQ-1 proposes BC-2.01.005 — but BC-2.01.x is HookResult/Payload/Version family. Host-shim invariants live in BC-2.02.x (per BC-INDEX:148-169).
- **Evidence:** BC-INDEX:148-169 enumerates BC-2.02.x as the host-shim invariant family. BC-2.01.x covers HookResult/Payload/Version.
- **Proposed Fix:** Rewrite OQ-1 to propose BC-2.02.011 (next slot after BC-2.02.010).

### MEDIUM

#### ADV-S810-P01-MED-001: ARCH-INDEX path not specified
- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** S-8.10 frontmatter inputs
- **Description:** ARCH-INDEX path not specified with line anchor.
- **Evidence:** Other stories use `.factory/specs/architecture/ARCH-INDEX.md:75`.
- **Proposed Fix:** Use `.factory/specs/architecture/ARCH-INDEX.md:75`.

#### ADV-S810-P01-MED-002: hook-sdk-macros peer-dep version conflict
- **Severity:** MEDIUM
- **Category:** missing-edge-cases
- **Location:** S-8.10 AC-7
- **Description:** AC-7 0.1.0->0.2.0 SDK bump conflicts with hook-sdk-macros peer-dep `version = "0.1.0"`. Decision needed on whether macros also bumps.
- **Evidence:** Cargo.toml hook-sdk-macros peer-dep pinned at 0.1.0.
- **Proposed Fix:** Decide whether macros also bumps to 0.2.0; document in AC-7.

#### ADV-S810-P01-MED-003: test_support.rs not listed as modify target
- **Severity:** MEDIUM
- **Category:** verification-gaps
- **Location:** S-8.10 AC-5
- **Description:** AC-5 unit tests in write_file.rs need `test_support::allow_write` helper; story does not list test_support.rs as a modify target.
- **Evidence:** read_file tests use test_support helpers — write_file tests will require analogous helper.
- **Proposed Fix:** Add test_support.rs to the file modification targets.

#### ADV-S810-P01-MED-004: Divergent path traversal return codes
- **Severity:** MEDIUM
- **Category:** contradictions
- **Location:** S-8.10 Architecture Compliance Rule 4
- **Description:** Architecture Compliance Rule 4 says write_file returns -4 on path traversal; read_file pattern returns -1 (CapabilityDenied).
- **Evidence:** read_file dispatcher returns -1 on CapabilityDenied. Rule 4 specifies -4 for write_file.
- **Proposed Fix:** Align return codes; document deliberate divergence if intentional.

#### ADV-S810-P01-MED-005: WriteFileCaps struct fields unspecified
- **Severity:** MEDIUM
- **Category:** missing-interface
- **Location:** S-8.10 AC-5
- **Description:** `WriteFileCaps` struct fields unspecified. Implementer cannot create the struct without guessing field names and types.
- **Evidence:** ReadFileCaps analogously specifies `path_allow: Vec<String>`.
- **Proposed Fix:** Pin `path_allow: Vec<String>` (and max_bytes if HIGH-002 ON) with derive set.

#### ADV-S810-P01-MED-006: AC-3 grep assertion under-specifies
- **Severity:** MEDIUM
- **Category:** verification-gaps
- **Location:** S-8.10 AC-3
- **Description:** AC-3 grep assertion under-specifies — multiple HOST_ABI_VERSION lines exist in the codebase.
- **Evidence:** grep for HOST_ABI_VERSION returns multiple matches across files.
- **Proposed Fix:** Use `grep -F 'pub const HOST_ABI_VERSION: u32 = 1;'` for exact match.

#### ADV-S810-P01-MED-007: AC-6 "no regression" target unenumerated
- **Severity:** MEDIUM
- **Category:** verification-gaps
- **Location:** S-8.10 AC-6
- **Description:** AC-6 "no regression" target unenumerated. Implementer has no list of crates to verify.
- **Evidence:** No crate list provided in AC-6.
- **Proposed Fix:** List specific hook-plugins crates (legacy-bash-adapter, capture-commit-activity, capture-pr-activity, block-ai-attribution, etc.).

### LOW

#### ADV-S810-P01-LOW-001: AC-8 doc path ambiguous
- **Severity:** LOW
- **Category:** spec-fidelity
- **Location:** S-8.10 AC-8
- **Description:** AC-8 doc path is "or" — multiple candidate paths listed.
- **Evidence:** `crates/hook-sdk/HOST_ABI.md` verified exists; `docs/ABI.md` does not.
- **Proposed Fix:** Pin `crates/hook-sdk/HOST_ABI.md`.

#### ADV-S810-P01-LOW-002: input-hash "pending" comment contradicts declared hash
- **Severity:** LOW
- **Category:** spec-fidelity
- **Location:** S-8.10 frontmatter
- **Description:** input-hash `ad07f36` declared real but comment says "pending" — resolve.
- **Evidence:** Frontmatter declares hash but inline comment contradicts.
- **Proposed Fix:** Remove "pending" comment; confirm hash is real.

#### ADV-S810-P01-LOW-003: hook-host-absent disclosure uncross-linked
- **Severity:** LOW
- **Category:** missing-edge-cases
- **Location:** S-8.10 disclosure section
- **Description:** hook-host-absent disclosure correct but uncross-linked to S-8.04/S-8.07/S-8.08/S-8.09 prior findings.
- **Evidence:** Prior stories documented same absence; cross-linking aids convergence tracking.
- **Proposed Fix:** Add cross-links to S-8.04, S-8.07, S-8.08, S-8.09 relevant finding IDs.

#### ADV-S810-P01-LOW-004: Estimate too low
- **Severity:** LOW
- **Category:** missing-edge-cases
- **Location:** S-8.10 estimate
- **Description:** Estimate "S (1 day, 3 pts)" too low for 11 tasks across 2 crates + sibling-update T-11.
- **Evidence:** 11 tasks + 2 crates + sibling sweep = M/5pts minimum.
- **Proposed Fix:** Bump to 5 pts or de-scope T-11.

#### ADV-S810-P01-LOW-005: AC-4(a) conflates FFI return with wrapper Err type
- **Severity:** LOW
- **Category:** ambiguous-language
- **Location:** S-8.10 AC-4(a)
- **Description:** AC-4(a) conflates FFI return (`-1`) with wrapper Err type (`HostError::CapabilityDenied`). Conversion chain not spelled out.
- **Evidence:** FFI returns i32; SDK wrapper converts to Result<(), HostError>.
- **Proposed Fix:** Spell out conversion chain: `ffi return -1 → HostError::CapabilityDenied → Err(HostError::CapabilityDenied)`.

### NIT

#### ADV-S810-P01-NIT-001: BC table trace-cell inconsistency
- **Severity:** NIT
- **Category:** spec-fidelity
- **Location:** S-8.10 BC table placeholder row
- **Description:** BC table placeholder row trace-cell only lists AC-1/AC-2 but ACs 1/2/4/5/7/8 carry OQ-1 annotations.
- **Proposed Fix:** Align trace-cell with all annotated ACs.

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 5 |
| MEDIUM | 7 |
| LOW | 5 |
| NIT | 1 |

**Overall Assessment:** block
**Convergence:** findings remain — iterate
**Readiness:** requires revision

## Open Questions

- **OQ-A1**: PO must adjudicate BC family — BC-2.01 vs BC-2.02 for write_file invariant.
- **OQ-A2**: Is BC-2.02.002 max_bytes mandate scoped to read paths only, or does it cover writes too?
- **OQ-A3**: Should SDK wrapper enforce contents.len() <= max_bytes BEFORE FFI (defense-in-depth) or rely on dispatcher-side?

## Pass-2 Priors (carry forward)

- HIGH-005 fix landed in OQ-1 prose AND Capability Anchor Justification (BC-2.01.005 -> BC-2.02.011)
- HIGH-002 + HIGH-004 resolved jointly via AC-1/AC-5 update OR BC-2.02.002 amendment ADR
- HIGH-003 actual FFI signature pinned in AC-1 (not just AC-8 docs)
- HIGH-001 inputs paths point under `.factory/cycles/v1.0-brownfield-backfill/`
- MED-005 WriteFileCaps struct shape pinned with derive set + fields
- Sibling sweep: S-8.04, S-8.07, S-8.08, S-8.09 + 1 random for BC-2.01 vs BC-2.02 family confusion

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 1 |
| **New findings** | 18 |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 1.0 (18/18) |
| **Median severity** | 3.0 |
| **Trajectory** | 18 |
| **Verdict** | FINDINGS_REMAIN |
