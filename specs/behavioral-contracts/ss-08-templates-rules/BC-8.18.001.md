---
document_type: behavioral-contract
level: L3
version: "1.1"
status: draft
producer: phase-1-4-b-bcs-agent-10
timestamp: 2026-04-25T00:00:00
phase: 1.4b
inputs:
  - .factory/specs/behavioral-contracts/bc-id-mapping.md
  - .factory/phase-0-ingestion/pass-3-deep-templates-tools-rules.md
traces_to: .factory/specs/architecture/ARCH-INDEX.md
origin: brownfield
extracted_from: "plugins/vsdd-factory/templates/verify-sha-currency.sh"
subsystem: SS-08
capability: ""
lifecycle_status: active
introduced: v1.0.0-beta.4
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
audit_id: BC-AUDIT-1929
section: "verify-sha-currency template-distributed hook"
type: hook
---

# Behavioral Contract BC-8.18.001: verify-sha-currency.sh: state-manager burst-hygiene gate (template-distributed; opt-in, NOT registered as a vsdd-factory hook)

## Description

Distributed as a template for operators to copy into their `.factory/hooks/`. Verifies SHAs in `.factory/STATE.md` and `.factory/SESSION-HANDOFF.md` are current (match git HEAD), cross-record SHAs in `wave-state.yaml` agree with STATE.md frontmatter, and active-pass narrative is past-tense. Run before every push to factory-artifacts (Stage 1) and after (Stage 2 verification). Exit 0 = PASS; exit 1 = FAIL. WARN-level issues (tense-flip, fabricated SHA cites) print to stdout but do NOT fail. NOT registered in `hooks-registry.toml` (per CONV-ABS-1 in deep-r1).

## Preconditions

1. The hook is invoked at its template-distributed integration point.
2. Caller has set up the hook in their state-manager wave-gate sequence.

## Postconditions

1. Template file present; CLI form `bash <path> [--project-root P]`; opt-in install path `.factory/hooks/verify-sha-currency.sh`.
2. Hook completes its declared check; failure produces actionable diagnostic.

## Invariants

1. Hook is idempotent and side-effect-free beyond documented checks.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | TBD — derive from source file edge cases | TBD |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Hook invoked with valid inputs | Returns success exit code | happy-path |
| Hook invoked with stale SHA / invalid state | Returns failure exit code with message | error |
| Hook invoked outside template-distributed context | TBD | edge-case |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-001 | TBD — promote acceptance criterion to a structural/lint test | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD |
| L2 Domain Invariants | TBD |
| Architecture Module | SS-08 (Templates and Rules) |
| Stories | TBD |
| Audit ID | BC-AUDIT-1929 |
| Section | verify-sha-currency template-distributed hook |

## Related BCs (Recommended)

- TBD — populate during cross-pass synthesis

## Architecture Anchors (Recommended)

- `architecture/SS-08-templates-rules.md` — TBD

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/templates/verify-sha-currency.sh` |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |

**Source metadata:** `plugins/vsdd-factory/templates/verify-sha-currency.sh` | **Type:** template | **Confidence:** HIGH | **Source line(s):** 1–24

**Used by:** `skills/state-burst/` references the template path; operators copy in.

#### Evidence Types Used

- **assertion**: hook runs an explicit check at invocation time
- **documentation**: behavior described in hook comments / template

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads + writes (hook may produce log output) |
| **Global state access** | reads repository state |
| **Deterministic** | yes (given fixed repo state) |
| **Thread safety** | N/A |
| **Overall classification** | effectful shell |

#### Refactoring Notes

Hook script is opt-in and template-distributed; not a core dispatcher dependency. Suitable for migration to a registered hook plugin in a future cycle.

