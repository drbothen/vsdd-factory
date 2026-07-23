---
document_type: behavioral-contract
level: L3
version: "1.2"
status: draft
producer: "PHASE_1_4_B_BCS_AGENT_9"
timestamp: 2026-04-25T00:00:00
phase: 1a
inputs: [pass-3-deep-hooks.md, pass-3-behavioral-contracts.md, pass-3-behavioral-contracts-deep-r1.md, bc-id-mapping.md]
input-hash: "118ab49"
traces_to: domain-spec/L2-INDEX.md
origin: brownfield
extracted_from: "pass-3-deep-hooks.md:1152"
subsystem: "SS-07"
capability: "TBD"
lifecycle_status: active
introduced: v1.0.0-beta.4
last_amended: "2026-07-22"
modified:
  - "2026-07-22 (v1.2) — spec-catches-up amendment (PR #722, human-authorized 2026-07-22): §Description and §Postconditions amended to document that frontmatter version and top changelog row version are normalized (leading v/V stripped from both operands) before comparison; a genuine numeric mismatch still errors."
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# Behavioral Contract BC-7.04.012: validate-changelog-monotonicity: cross-checks frontmatter version against top changelog row

## Description

validate-changelog-monotonicity: cross-checks frontmatter version against top changelog row. Before comparing, both the frontmatter `version:` value and the top changelog row version are normalized by stripping any leading `v` or `V` prefix. If the normalized values differ, error "Frontmatter version 'X' != top changelog version 'Y'" (where X and Y are the raw, pre-normalization values). A genuine numeric mismatch — where the version numbers differ after normalization — still errors.

**Source category:** Validator hook scripts (validate-* and verify-*).
**Audit ID:** `BC-AUDIT-1103` (extracted from `pass-3-deep-hooks.md` line 1152).
**Hook script:** ``plugins/vsdd-factory/hooks/validate-changelog-monotonicity.sh``.

## Preconditions

1. Trigger: Frontmatter has `version:` field AND top changelog row exists.

## Postconditions

1. Behavior: Both the frontmatter `version:` value and the top changelog row version are normalized by stripping any leading `v` or `V` prefix before comparison. If the normalized values differ (i.e., the numeric parts do not match), error "Frontmatter version 'X' != top changelog version 'Y'". This normalization allows `version: "1.2"` in frontmatter to be consistent with either `1.2` or `v1.2` in the changelog row, provided the numeric part is the same.
2. Exit codes: 2.

## Invariants

1. Hook script identity (script path) and registry binding remain stable across the contract lifetime.
2. Exit-code semantics conform to the dispatcher contract: 0 = allow / advisory, 2 = block, 1 = jq-missing-fail-closed (where applicable).

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | TBD | TBD |

## Canonical Test Vectors

> Golden-file test inputs and expected outputs. Used for regression testing and agent validation.

| Input | Expected Output | Category |
|-------|-----------------|----------|
| TBD | TBD | happy-path |
| TBD edge-case | TBD | edge-case |
| TBD error-case | TBD | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|--------------|
| VP-TBD | TBD — to be assigned during VP synthesis | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD |
| L2 Domain Invariants | TBD |
| Architecture Module | SS-07 (Hook Bash Layer) |
| Stories | TBD (filled by story-writer) |

## Related BCs (Recommended)

- TBD — to be cross-linked during BC graph synthesis.

## Architecture Anchors (Recommended)

- `architecture/ss-07-hook-bash.md` — anchor TBD.

## Story Anchor (Recommended)

TBD — story will be assigned during story-writer phase.

## VP Anchors (Recommended)

- TBD — VP linkage to be added during VP synthesis.

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | ``plugins/vsdd-factory/hooks/validate-changelog-monotonicity.sh`` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |
| **Source Document** | `pass-3-deep-hooks.md` line 1152 |
| **Audit ID** | `BC-AUDIT-1103` |
| **Source Line(s) (within hook)** | 144-148. |

#### Evidence Types Used

- **guard clause**: explicit validation check in the hook script body (regex / substring / glob match).

#### Purity Classification

| Property | Assessment |
|----------|------------|
| **I/O operations** | reads + writes (stdin JSON, stderr diagnostics, optional event emission via `${CLAUDE_PLUGIN_ROOT}/bin/emit-event`) |
| **Global state access** | reads global (env vars: `CLAUDE_PLUGIN_ROOT`, `CLAUDE_PROJECT_DIR`, optionally `VSDD_*`) |
| **Deterministic** | yes — bash hooks are deterministic given identical stdin envelope and filesystem state |
| **Thread safety** | not applicable (subprocess-isolated invocation per hook fire) |
| **Overall classification** | effectful shell |

#### Refactoring Notes

Bash hook scripts are inherently effectful (stdin/stderr, optional event emit, optional state-file reads). Native (Rust) replacement would extract pure parse/decision logic from the I/O shell, exposing a `fn(payload) -> HookResult` contract per BC-7.02.009. Until that port lands, the contract is preserved by the script body verbatim and the registry binding tuple.

## Changelog

| Version | Date | Author | Change |
|---------|------|--------|--------|
| 1.2 | 2026-07-22 | product-owner | Spec-catches-up amendment (PR #722, human-authorized 2026-07-22): §Description and §Postconditions amended to document version normalization — leading v/V stripped from both frontmatter version and top changelog row version before comparison; a genuine numeric mismatch still errors. |
| 1.1 | 2026-04-25 | PHASE_1_4_B_BCS_AGENT_9 | Initial authoring. |
