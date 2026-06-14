---
document_type: behavioral-contract
level: L3
version: "1.1"
status: draft
producer: product-owner
timestamp: 2026-06-14T00:00:00Z
last_amended: "2026-06-14 (v1.1) — F2 pass-1 fix-burst: (F-1) Precondition 3 re-anchored: phantom `current_wave:` field removed; wave identity derives from sprint-state.yaml dependency-order (product pipelines) or STATE.md `current_step:` (engine). (F-2) Precondition 4 downgraded: factory lock held OR absent; `factory_lock_holder` nullable; Postcondition 2 and anti-fabrication cross-check Postcondition 3 updated for null case. EC-010 (factory_lock absent) added. (F-12) precompact_flush_sha now a HARD cross-check against `.factory/hooks/last-precompact-flush-sha` when that file exists; advisory-override for wave>1 null SHA tightened. (DI) TBD-DI replaced with DI-020+DI-021+DI-023."
phase: F2
inputs:
  - .factory/feature-delta/issue-173/F1-delta-analysis.md
  - .factory/specs/architecture/decisions/ADR-026-wave-boundary-checkpoint-reset-and-lossless-intra-wave-compaction.md
input-hash: "[to-be-computed-by-state-manager]"
traces_to: .factory/specs/prd.md
origin: greenfield
extracted_from: null
subsystem: "SS-05"
capability: "CAP-032"
lifecycle_status: draft
introduced: v1.0-feature-context-durability-E18
modified:
  - "2026-06-14 (v1.1) — F2 pass-1 fix-burst: Precondition 3 re-anchored (sprint-state.yaml/current_step:; no phantom current_wave:); Precondition 4 lock-held-or-absent; PC2 factory_lock_holder nullable; PC3 cross-check updated for null holder; PC5 precompact_flush_sha hard cross-check against last-precompact-flush-sha side-channel; EC-010 factory_lock absent edge case added; TBD-DI replaced with DI-020+DI-021+DI-023; ADR cite v1.0→v1.1."
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# BC-5.41.001: wave-gate writes verified HANDOFF.md with all 9 required fields and anti-fabrication cross-checks before declaring wave closed

## Description

When the `wave-gate` skill is invoked to close wave N (where N > 0), it must produce a `HANDOFF.md` artifact on the `factory-artifacts` branch before the wave is considered closed. Every field in `HANDOFF.md` must be cross-checked against an external verifiable source (git or filesystem) — no field may rely solely on in-context memory. Missing or unverifiable fields cause `wave-gate` to block and surface the specific gap to the operator. This BC closes the fabricated-SHA failure class documented in issues #170 and #173.

## Preconditions

1. The `wave-gate` skill is invoked with intent to close wave N (N >= 1).
2. The `factory-artifacts` orphan branch is accessible via `git`.
3. Wave identity is determinable from real substrate: for product pipelines, `sprint-state.yaml` wave-group ordering (derived by `wave-scheduling` skill dependency-order); for the self-referential engine, STATE.md frontmatter `current_step:` field. No phantom `current_wave:` frontmatter field is referenced — this field does not exist.
4. The factory lock is held by the invoking session OR is absent. Lock held: `STATE.md` `factory_lock.holder` matches invoking session identity per BC-5.40.001 PC4. Lock absent: `factory_lock:` block is absent/null in STATE.md — wave-gate may proceed without a lock guard; `factory_lock_holder` field in HANDOFF.md is set to `null`. Both states are valid preconditions for wave-gate invocation.
5. Harness version >= v2.1.105 (for PreCompact/PostCompact event support in downstream hooks).

## Postconditions

1. **HANDOFF.md written**: A `HANDOFF.md` file is written to the `factory-artifacts` branch root (or `.factory/` path on that branch) containing all 9 required fields specified in ADR-026 §Decision 2.

2. **All 9 required fields present**: The following fields are present and non-empty (unless explicitly allowed to be null):
   - `wave_id` — integer matching `current_wave` in `STATE.md` frontmatter
   - `last_verified_develop_sha` — 40-character lowercase hex string
   - `active_bcs` — non-empty list of strings (each a BC file path)
   - `next_wave_stories` — list of objects with `{id, status}` keys
   - `open_decisions` — list of objects with `{id, anchor_type, anchor_ref}` keys (may be empty list)
   - `pending_fixes` — list of objects with `{finding_id, pr_or_issue_ref}` keys (may be empty list)
   - `process_gaps` — list (may be empty)
   - `precompact_flush_sha` — 40-char hex OR explicit `null` (null only permitted for wave_id=1)
   - `factory_lock_holder` — string matching `factory_lock.holder` in `STATE.md` when lock is held; `null` when `factory_lock:` block is absent or holder is absent/null (lock not held)

3. **Anti-fabrication cross-checks pass**:
   - `wave_id` == `STATE.md` frontmatter `current_wave:` (read from git, not in-context)
   - `last_verified_develop_sha` == output of `git rev-parse origin/develop` at handoff time
   - Each path in `active_bcs` resolves to an existing file under `.factory/specs/behavioral-contracts/`
   - Each `id` in `next_wave_stories` exists in `STORY-INDEX.md`
   - `factory_lock_holder` == `STATE.md` `factory_lock.holder` (read from git) when lock is held; null in HANDOFF.md is correct when STATE.md `factory_lock:` block is absent/null

4. **Block on missing/failing fields**: If any required field is absent or any anti-fabrication check fails, `wave-gate` blocks wave close, surfaces the specific failing fields and checks to the operator, and does NOT write a partial `HANDOFF.md`.

5. **`precompact_flush_sha` hard cross-check (F-12)**: `wave-gate` MUST cross-check `precompact_flush_sha` against the side-channel file `.factory/hooks/last-precompact-flush-sha`:
   - **File exists**: `precompact_flush_sha` MUST equal the SHA recorded in `.factory/hooks/last-precompact-flush-sha` (read via literal file system read, not operator attestation). A mismatch or a null `precompact_flush_sha` when the file contains a real SHA is a HARD BLOCK — `wave-gate` rejects the handoff with `PrecompactShaMismatch`. The advisory-override for wave>1 null SHA is removed; fabricated or null SHAs that contradict the side-channel file MUST block.
   - **File genuinely absent** (verified by `test -f .factory/hooks/last-precompact-flush-sha` returning false): `precompact_flush_sha: null` is permitted for wave_id=1. For wave_id > 1 with no flush file: advisory warning only; operator must confirm no PreCompact fired; `wave-gate` surfaces the warning but does not hard-block (the absence is mechanically verified, not attested).
   - **Rationale**: An unchecked null SHA on wave>1 is exploitable — an attacker or hallucinating session could write `precompact_flush_sha: null` to bypass the flush guarantee. The side-channel file is the mechanical ground truth.

6. **Commit to factory-artifacts**: After all checks pass, `wave-gate` commits `HANDOFF.md` to the `factory-artifacts` branch with a commit message: `HANDOFF wave-<N> <ISO-timestamp>`.

7. **validate-wave-handoff-completeness gate passes**: After writing `HANDOFF.md`, the `validate-wave-handoff-completeness` WASM gate (BC-4.14.001) is invoked as a PostToolUse gate and must return `Continue` before the wave is declared closed.

## Invariants

1. **No in-context memory assertions**: Every `HANDOFF.md` field value must be derivable from an external source (git, filesystem, index file). A field whose value cannot be cross-checked must be omitted or flagged with an explicit `TBD` + operator action item — not silently populated from in-context reasoning.

2. **Wave cannot be closed without a passing HANDOFF.md**: The wave-gate's "declare wave closed" step is gated behind this BC's Postconditions. A wave-gate invocation that skips HANDOFF.md production is a specification violation.

3. **HANDOFF.md is a singleton per wave boundary**: There is one `HANDOFF.md` on `factory-artifacts` (not wave-scoped files). Each wave close overwrites the previous HANDOFF.md with the new wave's verified state. Git history preserves prior HANDOFF.md versions.

4. **Anti-fabrication is not optional**: The cross-check against `git rev-parse origin/develop` for `last_verified_develop_sha` must execute as a literal shell command, not be inferred from in-context knowledge of the SHA. Same for filesystem existence checks on `active_bcs` paths.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Wave_id = 1 (first wave ever); no prior HANDOFF.md exists | `precompact_flush_sha: null` is valid. All other fields still required. |
| EC-002 | `last_verified_develop_sha` mismatch (`git rev-parse` returns different value than operator supplied) | Block with `AntiShabricationFailed: last_verified_develop_sha mismatch`; surface both values to operator |
| EC-003 | An `active_bcs` path does not resolve to an existing file | Block with `AntiShabricationFailed: BC path not found: <path>`; operator must correct the path |
| EC-004 | A `next_wave_stories` story ID not found in STORY-INDEX.md | Block with `AntiShabricationFailed: story ID not in STORY-INDEX: <id>` |
| EC-005 | `factory_lock_holder` does not match `STATE.md` factory_lock.holder (when lock IS held) | Block; lock mismatch indicates stale handoff data or another holder |
| EC-006 | `precompact_flush_sha` is null for wave_id > 1; `.factory/hooks/last-precompact-flush-sha` file genuinely absent (mechanically verified) | Advisory warning only; operator must confirm no PreCompact fired; wave-gate does not hard-block |
| EC-010 | `STATE.md` `factory_lock:` block is absent/null (no lock held) | `factory_lock_holder: null` in HANDOFF.md; wave-gate proceeds without lock validation; this is valid per ADR-025 opt-in model |
| EC-011 | `precompact_flush_sha: null` in HANDOFF.md but `.factory/hooks/last-precompact-flush-sha` exists with a non-empty SHA | HARD BLOCK: `PrecompactShaMismatch`; operator must populate `precompact_flush_sha` with the SHA from the side-channel file |
| EC-007 | HANDOFF.md written but `validate-wave-handoff-completeness` gate fails | Wave close is blocked; operator must correct the failing fields |
| EC-008 | `open_decisions` list is empty | Valid; no open decisions is a legitimate wave-close state |
| EC-009 | `process_gaps` list is non-empty (carry-forward from issue #171) | Valid; gaps are carried forward explicitly; wave closes after other checks pass |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| wave-gate close with all 9 fields correct + all cross-checks pass | HANDOFF.md committed to factory-artifacts; wave declared closed | happy-path |
| wave-gate close; `last_verified_develop_sha` does not match `git rev-parse origin/develop` | Block; error message names the mismatch; no HANDOFF.md committed | anti-fabrication-sha |
| wave-gate close; `active_bcs` contains path `.factory/specs/behavioral-contracts/BC-99.99.999.md` (non-existent) | Block; error names the missing path; no HANDOFF.md committed | anti-fabrication-fs |
| wave_id = 1; `precompact_flush_sha: null` | Valid; HANDOFF.md accepted | wave-1-null-sha |
| wave_id = 2; `precompact_flush_sha: null` | Advisory warning; operator confirmation required; not a hard block | wave-n-null-sha-advisory |
| wave-gate close; `factory_lock_holder` mismatch | Block; lock mismatch surfaced | lock-holder-mismatch |

## Related BCs

- BC-4.14.001 — depends on: validate-wave-handoff-completeness WASM gate verifies HANDOFF.md completeness post-write
- BC-5.41.002 — sibling: wave-state.yaml is written atomically alongside HANDOFF.md at wave close (same wave-gate invocation)
- BC-5.40.001 — depends on: factory_lock must be held; factory-lock-write.sh renew invoked before HANDOFF.md commit per ADR-025 D11 Mechanism 1
- BC-7.07.001 — composes with: precompact-flush.sh sets `precompact_flush_sha` side-channel file; wave-gate reads it for HANDOFF.md field population

## Architecture Anchors

- `plugins/vsdd-factory/skills/wave-gate/SKILL.md` — wave-gate skill; wave-close step extended with HANDOFF.md production obligation
- `plugins/vsdd-factory/skills/wave-handoff/SKILL.md` — NEW skill that encapsulates HANDOFF.md production logic (S-18.01 deliverable)
- `.factory/hooks/last-precompact-flush-sha` — side-channel file written by precompact-flush.sh; read by wave-gate for `precompact_flush_sha` field

## Story Anchor

S-18.01 (HANDOFF.md schema + wave-handoff skill)

## VP Anchors

- VP-081 — Wave Cannot Close Without Verified Handoff (wave_id > 1): validate-wave-handoff-completeness gate blocks wave-gate close unless HANDOFF.md exists with all required ADR-026 §D2 fields

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-081 | Wave cannot close (wave_id > 1) without verified HANDOFF.md with all required ADR-026 §D2 fields | integration |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-032 ("Guarantee lossless context-window transitions via wave-boundary checkpoint and PreCompact flush") per capabilities.md §CAP-032 |
| Capability Anchor Justification | CAP-032 ("Guarantee lossless context-window transitions via wave-boundary checkpoint and PreCompact flush") per capabilities.md §CAP-032 — this BC specifies the verified HANDOFF.md wave-close checkpoint that is the primary cross-wave continuity mechanism (ADR-026 Decision 1 + Decision 2); it directly closes the fabricated-SHA failure class documented in issues #170 and #173 |
| L2 Domain Invariants | DI-020 (Wave/phase boundary transitions must not lose load-bearing pipeline state — enforced by requiring HANDOFF.md with all 9 fields before wave close); DI-021 (Handoff claims must be cross-checked against verifiable external ground truth — enforced by anti-fabrication cross-checks in PC3, including SHA verification against git and precompact_flush_sha against side-channel file); DI-023 (Wave/phase identity and next-wave story lists derive from real persisted substrate — enforced by wave_id derivation from sprint-state.yaml or STATE.md current_step:, and next_wave_stories from sprint-state.yaml status entries) |
| Architecture Module | SS-05 (Pipeline Orchestration) — wave-gate and wave-handoff skills live in `plugins/vsdd-factory/skills/` |
| ADR | ADR-026 v1.1 Decision 2 (HANDOFF.md schema + anti-fabrication cross-checks; wave_id from real substrate; factory_lock_holder nullable; precompact_flush_sha hard cross-check against side-channel file); Decision 9 (wave-1 no-op); Decision 1 (wave-boundary reset is primary mechanism) |
| Stories | S-18.01 |
| Cycle | v1.0-feature-context-durability-E18 (F2) |
| Feature | issue #173 / E-18 |
