---
document_type: governance-note
scope: verification-properties
version: "1.0"
produced_by: spec-steward
timestamp: 2026-07-17T00:00:00Z
adjudicates: "F-006 (LOW) — E-19 W3 consistency audit"
cycle: v1.0-brownfield-backfill
---

# VP Lifecycle Convention — Status Field Semantics and Advancement Rules

## Scope

This governance note codifies the authoritative semantics for the two status-related
frontmatter fields in VP files and VP-INDEX catalog rows. It resolves audit finding
F-006 (LOW) from the E-19 W3 consistency audit, which observed that VP-094..VP-101
have `status: draft` despite their anchoring stories being merged and their proof
harnesses being live in the merged tree.

## Two Distinct Fields — Not Analogous to Each Other

The VP template (DF-030) defines two separate fields with different purposes. They
are not analogous to each other, and the BC `status` lifecycle (POL-14) maps to only
one of them.

### 1. `lifecycle_status` — Registry Applicability

**Valid values:** `active | deprecated | retired | removed | withdrawn`

**Meaning:** Whether the VP is currently applicable to the system and in scope for the
verification program. This is the field analogous to the BC `status` field governed
by POL-14 (BC `draft → active` at story merge).

**Convention:** `lifecycle_status` is set to `active` at VP creation time, when the VP
is first added to VP-INDEX. There is no deferred-activation pattern for VPs — a VP
that is not yet active has no business being in VP-INDEX. Unlike BCs (which live in
`draft` until their story's PR merges), VPs are authored by the architect in direct
response to a story's spec package and are immediately operative as proof obligations.

**Auto-advancement rule:** None required beyond the authoring convention above. A VP
transitions off `active` only via an explicit deprecation, retirement, or withdrawal
decision executed by the formal-verifier or architect. No POL-14 mirror for
`lifecycle_status` is needed — the analogous gate (VP is applicable) is satisfied by
VP-INDEX inclusion itself, not by a separate merge event.

### 2. `status` — Proof Completion State

**Valid values:** `draft | in-development | verified | withdrawn`

**Meaning:** Whether the formal proof for this VP has been run, passed, and locked.

| Value | Meaning |
|-------|---------|
| `draft` | VP authored; proof harness may or may not exist; proof has NOT been formally run and locked |
| `in-development` | Proof harness is actively under construction or first-run iteration |
| `verified` | Proof has formally passed; `verification_lock: true`, `proof_completed_date` and `proof_file_hash` are both set |
| `withdrawn` | VP withdrawn; proof obligation eliminated |

**This field is NOT analogous to BC `status`.** A BC `status: active` means "this
behavioral contract is operative and enforceable." A VP `status: draft` means "the
formal proof has not yet been locked" — it says nothing about whether the VP is in
scope or whether a harness exists. A VP with `status: draft` and
`lifecycle_status: active` is the normal state for every VP in the registry until the
formal-verifier runs the proof and executes the verification-lock ceremony.

**Advancement rule for `status`:** VP `status` advances from `draft` to `verified`
ONLY when ALL THREE of the following are true:
1. A proof run has completed (Kani, proptest, integration, or other declared
   `proof_method`) and passed against the current implementation.
2. `verification_lock: true` has been set in the VP frontmatter.
3. Both `proof_completed_date` and `proof_file_hash` are populated.

This advancement is executed by the formal-verifier agent. Mere presence of a proof
harness skeleton (bats `@test` blocks, Rust `#[test]` functions, Kani stubs) in the
merged tree does NOT advance `status` to `verified`. The harness enables the proof;
it does not constitute the proof passing.

**No POL-14 equivalent for VP `status` is warranted.** Mirroring POL-14 by advancing
VP `status` at story merge would conflate proof-harness-existence with
proof-run-completion, producing a false `verified` signal for properties whose harness
has never been formally exercised by the verification toolchain.

## Ruling on Finding F-006

**Finding:** VP-094..VP-101 have `status: draft` even though anchoring stories are
merged and proof harnesses are live.

**Ruling: NOT A DEFECT. `status: draft` is correct for all 101 VPs.**

All 101 VPs have `verification_lock: false`, `proof_completed_date: null`, and
`proof_file_hash: null`. No formal verification-lock ceremony has been executed for
any VP in this repository. `status: draft` accurately reflects that state.

The audit finding conflates the BC `status` lifecycle (draft → active = BC is
operative) with the VP `status` lifecycle (draft → verified = proof has passed).
These are semantically different events. The operative status for VPs is
`lifecycle_status`, not `status`. VP-094..VP-101 already have `lifecycle_status: active`
— they ARE operative in the registry.

**F-006 is closed with no `status` field changes to VP-094..VP-101.**

## Actual Gap Surfaced by Survey: Missing `lifecycle_status` on VP-081..VP-093

The full-corpus status survey (conducted during F-006 adjudication) reveals that
VP-081 through VP-093 (the E-18 Context Durability VP package, 13 VPs) are entirely
missing the `lifecycle_status` field. These VPs were authored without the DF-030
lifecycle block that all other VPs carry. Their anchoring stories (E-18 wave) are
merged, they are included in VP-INDEX, and they are operative proof obligations.

**Migration action required in the next state-manager burst:**

Add `lifecycle_status: active` to the frontmatter of each VP listed below,
positioned after `proof_file_hash:` and before `introduced:` (consistent with the
ordering in VP-001..VP-080). No other field changes are required.

| VP ID | Anchoring story |
|-------|-----------------|
| VP-081 | S-18.01 / S-18.02 |
| VP-082 | S-18.04a |
| VP-083 | S-18.02 |
| VP-084 | S-18.04b |
| VP-085 | S-18.04a |
| VP-086 | S-18.00 |
| VP-087 | S-18.01 |
| VP-088 | S-18.03 |
| VP-089 | S-18.05 |
| VP-090 | S-18.04a |
| VP-091 | S-18.06 |
| VP-092 | S-18.10 |
| VP-093 | S-18.04b-prereq |

The state-manager burst that adds these 13 fields must also bump VP-INDEX version
and `last_amended` (POLICY 9 + POLICY 14 5-leg parity apply to VP frontmatter
changes).

No changes to `status:`, `proof_completed_date:`, `proof_file_hash:`, or
`verification_lock:` are required for any VP.

## Summary

| Field | Meaning | Advances when | BC analogue |
|-------|---------|--------------|-------------|
| `lifecycle_status` | VP is applicable/in-registry | At VP creation (authoring time) | BC `status: active` (POL-14 spirit) |
| `status` | Formal proof has been locked | proof_completed_date + verification_lock set | None |

Current state of corpus: all 101 VPs have correct `status: draft` (no formal proofs
locked). VP-001..VP-080 and VP-094..VP-101 have correct `lifecycle_status: active`.
VP-081..VP-093 are missing the `lifecycle_status` field — one migration action
required in the next state-manager burst.
