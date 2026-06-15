---
document_type: behavioral-contract
level: L3
version: "1.8"
status: draft
producer: product-owner
timestamp: 2026-06-14T00:00:00Z
last_amended: "2026-06-15 (v1.8) — F2 pass-14 fix-burst: (F-P14-001) PC2 precompact_flush_sha null-permitted rule rewritten: blanket wave>1-null prohibition removed; null now permitted for wave_id=1 OR for wave_id>1 ONLY when precompact-flush-log is genuinely absent (advisory per PC5); null is a HARD BLOCK (PrecompactShaMismatch) when log exists and contains a valid commit SHA. PC2, PC5, EC-006, and test vector wave-n-null-sha-advisory are now mutually coherent. [Prior: 2026-06-14 (v1.7) — F2 pass-13 fix-burst: (F-P13-004) ADR Traceability cell: load-bearing inline `per v1.6` version token removed from §Decision 2 parenthetical; rewritten as non-load-bearing informational cite per POLICY 19 anti-volatile-pin. [Prior: 2026-06-14 (v1.6) — F2 pass-6 fix-burst: (F-P6-001/002) PC5 stale-SHA case (b) label: 'write-before-push crash' → 'log corruption or truncation' (re-grounding per ADR-026 v1.6 F-P5-002; mechanically impossible write-before-push justification removed); wave-gate is a SHELL skill so it MAY exec git, but canonical corroboration is structured field read (FIELD-2 SHA + FIELD-4 `commit` token from last-line of precompact-flush-log); git-exec is a permitted shell-context capability, NOT the sole method. (F-P6-004) PC2 heading and body: 'All 9 required fields' → 'All 9 base required fields present; `epic_status: complete` additionally required on EPIC-COMPLETE wave (must be ABSENT on non-final waves)'; `epic_status` conditional field added to PC2 list; H1 updated to reflect 9-base+epic_status conditionality; EC-012/EC-013 added (non-final+epic_status → UnexpectedEpicStatus; final+epic_status:complete → Continue; final missing epic_status → MissingEpicStatus). (E-18) ADR cite convention: v1.4 version token dropped per ADR-026 §BC Traceability Cite Convention (TD-VSDD-091 anti-volatile-pin); stable §Decision anchors adopted. [Prior: 2026-06-14 (v1.4) — F2 pass-4 fix-burst: (F-P4-003) ADR cite v1.3→v1.4 (cite-only). [Prior: 2026-06-14 (v1.3) — F2 pass-3 fix-burst: (O-P3-002) PC8 added: EPIC-COMPLETE operator surfacing — when EPIC-COMPLETE triggers (all stories terminal), wave-gate announces completion via stdout with concrete message format before exiting. ADR cite v1.1→v1.3. [Prior: 2026-06-14 (v1.2) — F2 pass-2 fix-burst: (F-P2-001) PC2 wave_id field def updated: phantom `current_wave:` field → derived from sprint-state.yaml topo-sort ordinal (product) or STATE.md current_step: (engine). PC3 anti-fabrication cross-check updated: wave_id matches derived value from sprint-state.yaml/current_step: (no stored current_wave: field). (F-12 append-log) PC5 last-precompact-flush-sha side-channel → precompact-flush-log append-log (last line + git cat-file -t validation); stale-SHA-in-log edge case added (write-before-push crash → skip); EC-006 + EC-011 + Architecture Anchors updated to precompact-flush-log. [Prior: 2026-06-14 (v1.1) — F2 pass-1 fix-burst: (F-1) Precondition 3 re-anchored: phantom `current_wave:` field removed; wave identity derives from sprint-state.yaml dependency-order (product pipelines) or STATE.md `current_step:` (engine). (F-2) Precondition 4 downgraded: factory lock held OR absent; `factory_lock_holder` nullable; Postcondition 2 and anti-fabrication cross-check Postcondition 3 updated for null case. EC-010 (factory_lock absent) added. (F-12) precompact_flush_sha now a HARD cross-check against `.factory/hooks/last-precompact-flush-sha` when that file exists; advisory-override for wave>1 null SHA tightened. (DI) TBD-DI replaced with DI-020+DI-021+DI-023.]"
phase: F2
inputs:
  - .factory/feature-delta/issue-173/F1-delta-analysis.md
  - .factory/specs/architecture/decisions/ADR-026-wave-boundary-checkpoint-reset-and-lossless-intra-wave-compaction.md
input-hash: "c2426d5"
traces_to: .factory/specs/prd.md
origin: greenfield
extracted_from: null
subsystem: "SS-05"
capability: "CAP-032"
lifecycle_status: draft
introduced: v1.0-feature-context-durability-E18
modified:
  - "2026-06-15 (v1.8) — F2 pass-14 fix-burst: (F-P14-001) PC2 precompact_flush_sha null-permitted rule rewritten: blanket wave>1-null prohibition removed; null permitted for wave_id=1 OR for wave_id>1 ONLY when precompact-flush-log is genuinely absent (advisory per PC5, no hard block); null is HARD BLOCK (PrecompactShaMismatch) when log exists and contains a valid commit SHA. PC2/PC5/EC-006/test-vector wave-n-null-sha-advisory mutually coherent."
  - "2026-06-14 (v1.7) — F2 pass-13 fix-burst: (F-P13-004) ADR Traceability cell: load-bearing inline `per v1.6` version token removed from §Decision 2 parenthetical; rewritten as non-load-bearing informational cite per POLICY 19 anti-volatile-pin."
  - "2026-06-14 (v1.6) — F2 pass-6 fix-burst: (F-P6-001/002) PC5 stale-SHA label corrected + corroboration mechanism documented; (F-P6-004) H1+PC2 field-count 9→9-base+epic_status conditional; EC-012/EC-013 added; ADR cite convention: stable §Decision anchors (TD-VSDD-091)."
  - "2026-06-14 (v1.4) — F2 pass-4 fix-burst: (F-P4-003) ADR cite v1.3→v1.4 (cite-only)."
  - "2026-06-14 (v1.3) — F2 pass-3 fix-burst: PC8 added (EPIC-COMPLETE stdout surfacing per O-P3-002); ADR cite v1.1→v1.3."
  - "2026-06-14 (v1.2) — F2 pass-2 fix-burst: PC2 wave_id phantom-field → derived value (sprint-state.yaml/current_step:); PC3 cross-check updated to derived value; PC5 side-channel file → precompact-flush-log append-log (last line + git cat-file -t); stale-SHA edge case (write-before-push crash); EC-006+EC-011+Arch Anchors updated."
  - "2026-06-14 (v1.1) — F2 pass-1 fix-burst: Precondition 3 re-anchored (sprint-state.yaml/current_step:; no phantom current_wave:); Precondition 4 lock-held-or-absent; PC2 factory_lock_holder nullable; PC3 cross-check updated for null holder; PC5 precompact_flush_sha hard cross-check against last-precompact-flush-sha side-channel; EC-010 factory_lock absent edge case added; TBD-DI replaced with DI-020+DI-021+DI-023; ADR cite v1.0→v1.1."
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# BC-5.41.001: wave-gate writes verified HANDOFF.md with all 9 base required fields (+epic_status on EPIC-COMPLETE wave) and anti-fabrication cross-checks before declaring wave closed

## Description

When the `wave-gate` skill is invoked to close wave N (where N > 0), it must produce a `HANDOFF.md` artifact on the `factory-artifacts` branch before the wave is considered closed. Every field in `HANDOFF.md` must be cross-checked against an external verifiable source (git or filesystem) — no field may rely solely on in-context memory. Missing or unverifiable fields cause `wave-gate` to block and surface the specific gap to the operator. This BC closes the fabricated-SHA failure class documented in issues #170 and #173.

## Preconditions

1. The `wave-gate` skill is invoked with intent to close wave N (N >= 1).
2. The `factory-artifacts` orphan branch is accessible via `git`.
3. Wave identity is determinable from real substrate: for product pipelines, `sprint-state.yaml` wave-group ordering (derived by `wave-scheduling` skill dependency-order); for the self-referential engine, STATE.md frontmatter `current_step:` field. No phantom `current_wave:` frontmatter field is referenced — this field does not exist.
4. The factory lock is held by the invoking session OR is absent. Lock held: `STATE.md` `factory_lock.holder` matches invoking session identity per BC-5.40.001 PC4. Lock absent: `factory_lock:` block is absent/null in STATE.md — wave-gate may proceed without a lock guard; `factory_lock_holder` field in HANDOFF.md is set to `null`. Both states are valid preconditions for wave-gate invocation.
5. Harness version >= v2.1.105 (for PreCompact/PostCompact event support in downstream hooks).

## Postconditions

1. **HANDOFF.md written**: A `HANDOFF.md` file is written to the `factory-artifacts` branch root (or `.factory/` path on that branch) containing all 9 base required fields specified in ADR-026 §Decision 2 (plus `epic_status: complete` conditionally on the EPIC-COMPLETE wave).

2. **All 9 base required fields present; `epic_status: complete` additionally required on EPIC-COMPLETE wave**: The following 9 base fields are present and non-empty (unless explicitly allowed to be null):
   - `wave_id` — integer derived from sprint-state.yaml dependency-order topo-sort ordinal (product pipelines) OR STATE.md `current_step:` pass number (engine) — there is no `current_wave:` field
   - `last_verified_develop_sha` — 40-character lowercase hex string
   - `active_bcs` — non-empty list of strings (each a BC file path)
   - `next_wave_stories` — list of objects with `{id, status}` keys
   - `open_decisions` — list of objects with `{id, anchor_type, anchor_ref}` keys (may be empty list)
   - `pending_fixes` — list of objects with `{finding_id, pr_or_issue_ref}` keys (may be empty list)
   - `process_gaps` — list (may be empty)
   - `precompact_flush_sha` — 40-char hex OR explicit `null` (null permitted for wave_id=1; null also permitted for wave_id>1 ONLY when `.factory/hooks/precompact-flush-log` is genuinely absent — mechanically verified by `test -f`, not operator-attested — advisory warning only, no hard block per PC5; null is a HARD BLOCK via `PrecompactShaMismatch` when the log file exists and its last line contains a valid commit SHA)
   - `factory_lock_holder` — string matching `factory_lock.holder` in `STATE.md` when lock is held; `null` when `factory_lock:` block is absent or holder is absent/null (lock not held)
   - `epic_status` — string `"complete"`; **CONDITIONAL**: required ONLY on the final/EPIC-COMPLETE wave (when all sprint-state.yaml stories have terminal status AND `next_wave_stories: []`); MUST be ABSENT (not `null`, not any value — genuinely absent) on all non-final waves. Presence of `epic_status` on a non-final wave is a malformed HANDOFF.md (causes `UnexpectedEpicStatus` from the validate-wave-handoff-completeness gate).

3. **Anti-fabrication cross-checks pass**:
   - `wave_id` matches the value computed by wave-handoff from sprint-state.yaml topo-sort ordinal (product) or STATE.md `current_step:` (engine) — no `current_wave:` field exists; cross-check is against the derived value, not a stored field
   - `last_verified_develop_sha` == output of `git rev-parse origin/develop` at handoff time
   - Each path in `active_bcs` resolves to an existing file under `.factory/specs/behavioral-contracts/`
   - Each `id` in `next_wave_stories` exists in `STORY-INDEX.md`
   - `factory_lock_holder` == `STATE.md` `factory_lock.holder` (read from git) when lock is held; null in HANDOFF.md is correct when STATE.md `factory_lock:` block is absent/null

4. **Block on missing/failing fields**: If any required field is absent or any anti-fabrication check fails, `wave-gate` blocks wave close, surfaces the specific failing fields and checks to the operator, and does NOT write a partial `HANDOFF.md`.

5. **`precompact_flush_sha` hard cross-check (F-12)**: `wave-gate` MUST cross-check `precompact_flush_sha` against the append-log file `.factory/hooks/precompact-flush-log` — specifically the LAST LINE of that file. The log line has four space-separated fields: `<ISO-timestamp> <SHA> <cycle>/<step> commit`. The canonical/primary corroboration is a structured field read: wave-gate reads FIELD-2 (the SHA) and FIELD-4 (the `commit` token) from the last line. wave-gate is a SHELL skill and therefore MAY exec `git cat-file -t <SHA>` to independently verify the object type, but this git exec is a permitted shell-context capability, NOT the sole or mandatory method — the FIELD-4 pre-embedded token provides sufficient corroboration (consistent with BC-5.41.003 and DI-025 which specify FIELD-4 as the canonical corroboration for WASM/shell contexts alike):
   - **File exists; last-line FIELD-4 == `commit` and FIELD-2 SHA is valid**: `precompact_flush_sha` MUST equal FIELD-2 of the LAST LINE of `.factory/hooks/precompact-flush-log` (read via literal filesystem read, not operator attestation). A mismatch or a null `precompact_flush_sha` when the log contains a valid commit SHA is a HARD BLOCK — `wave-gate` rejects the handoff with `PrecompactShaMismatch`. The advisory-override for wave>1 null SHA is removed; fabricated or null SHAs that contradict the log MUST block.
   - **SHA in log but FIELD-4 is absent, empty, or not the literal string `commit`** (indicating log corruption or truncation): treat the log entry as stale and skip it; fall through to the genuinely-absent case below.
   - **File genuinely absent** (verified by `test -f .factory/hooks/precompact-flush-log` returning false): `precompact_flush_sha: null` is permitted for wave_id=1. For wave_id > 1 with no flush log: advisory warning only; operator must confirm no PreCompact fired; `wave-gate` surfaces the warning but does not hard-block (the absence is mechanically verified, not attested).
   - **Rationale**: An unchecked null SHA on wave>1 is exploitable — an attacker or hallucinating session could write `precompact_flush_sha: null` to bypass the flush guarantee. The side-channel file is the mechanical ground truth.

6. **Commit to factory-artifacts**: After all checks pass, `wave-gate` commits `HANDOFF.md` to the `factory-artifacts` branch with a commit message: `HANDOFF wave-<N> <ISO-timestamp>`.

7. **validate-wave-handoff-completeness gate passes**: After writing `HANDOFF.md`, the `validate-wave-handoff-completeness` WASM gate (BC-4.14.001) is invoked as a PostToolUse gate and must return `Continue` before the wave is declared closed.

8. **EPIC-COMPLETE operator surfacing**: When `wave-gate` detects EPIC-COMPLETE (all entries in `sprint-state.yaml` have terminal status — merged, withdrawn, or cancelled — and no pending/draft entries remain), before exiting 0 it MUST write the following message to stdout so the operator is explicitly notified:
   ```
   EPIC-COMPLETE: All stories in sprint-state.yaml have reached terminal status.
   Epic <epic-id> is complete. No next-wave handoff required.
   HANDOFF.md written with epic_status: complete. wave-state.yaml NOT written.
   ```
   Where `<epic-id>` is derived from the cycle identifier in `STATE.md` `current_cycle:` field. A silent exit 0 on EPIC-COMPLETE is a specification violation — the operator must receive explicit confirmation that the epic has ended, not merely the absence of an error.

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
| EC-006 | `precompact_flush_sha` is null for wave_id > 1; `.factory/hooks/precompact-flush-log` file genuinely absent (mechanically verified) | Advisory warning only; operator must confirm no PreCompact fired; wave-gate does not hard-block |
| EC-010 | `STATE.md` `factory_lock:` block is absent/null (no lock held) | `factory_lock_holder: null` in HANDOFF.md; wave-gate proceeds without lock validation; this is valid per ADR-025 opt-in model |
| EC-011 | `precompact_flush_sha: null` in HANDOFF.md but `.factory/hooks/precompact-flush-log` last line contains a valid commit SHA (verified by `git cat-file -t`) | HARD BLOCK: `PrecompactShaMismatch`; operator must populate `precompact_flush_sha` with the SHA from the last line of the log |
| EC-007 | HANDOFF.md written but `validate-wave-handoff-completeness` gate fails | Wave close is blocked; operator must correct the failing fields |
| EC-008 | `open_decisions` list is empty | Valid; no open decisions is a legitimate wave-close state |
| EC-009 | `process_gaps` list is non-empty (carry-forward from issue #171) | Valid; gaps are carried forward explicitly; wave closes after other checks pass |
| EC-012 | HANDOFF.md write on non-final wave includes `epic_status: complete` | HARD BLOCK: `UnexpectedEpicStatus`; `epic_status` MUST be absent on non-final waves; wave-gate rejects the handoff |
| EC-013 | HANDOFF.md write on EPIC-COMPLETE final wave (all stories terminal, `next_wave_stories: []`) is missing `epic_status` | HARD BLOCK: `MissingEpicStatus`; `epic_status: complete` is required on EPIC-COMPLETE waves |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| wave-gate close (non-final wave) with all 9 base fields correct + all cross-checks pass; `epic_status` absent | HANDOFF.md committed to factory-artifacts; wave declared closed | happy-path-non-final |
| wave-gate close (EPIC-COMPLETE final wave) with all 9 base fields + `epic_status: complete`; all stories terminal; `next_wave_stories: []` | HANDOFF.md committed; EPIC-COMPLETE stdout announcement; wave declared complete | happy-path-epic-complete |
| wave-gate close; non-final wave; HANDOFF.md includes `epic_status: complete` | Block; `UnexpectedEpicStatus`; no HANDOFF.md committed | unexpected-epic-status |
| wave-gate close; EPIC-COMPLETE wave; HANDOFF.md missing `epic_status` | Block; `MissingEpicStatus`; no HANDOFF.md committed | missing-epic-status |
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
- `.factory/hooks/precompact-flush-log` — append-log written by precompact-flush.sh; wave-gate reads the LAST LINE for `precompact_flush_sha` field; canonical corroboration is FIELD-2 (SHA) + FIELD-4 (`commit` token) from the last line; wave-gate MAY additionally exec `git cat-file -t <SHA>` (shell context permits it) but this is not the sole or mandatory method

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
| ADR | ADR-026 §Decision 2 (HANDOFF.md schema + anti-fabrication cross-checks; wave_id from real substrate; factory_lock_holder nullable; precompact_flush_sha hard cross-check against side-channel file; 9 base required fields + epic_status conditional on EPIC-COMPLETE wave — informational, non-load-bearing), §Decision 9 (wave-1 no-op), §Decision 1 (wave-boundary reset is primary mechanism) |
| Stories | S-18.01 |
| Cycle | v1.0-feature-context-durability-E18 (F2) |
| Feature | issue #173 / E-18 |

## Changelog

| Version | Date | Author | Change |
|---------|------|--------|--------|
| v1.8 | 2026-06-15 | product-owner | (F-P14-001) PC2 precompact_flush_sha null-permitted rule rewritten: blanket wave>1-null prohibition removed; null permitted for wave_id=1 OR for wave_id>1 ONLY when precompact-flush-log is genuinely absent (advisory per PC5, no hard block); null is HARD BLOCK (PrecompactShaMismatch) when log exists and contains a valid commit SHA. PC2, PC5, EC-006, and test vector wave-n-null-sha-advisory are now mutually coherent. |
| v1.7 | 2026-06-14 | product-owner | (F-P13-004) ADR Traceability cell: load-bearing inline `per v1.6` version token removed from §Decision 2 parenthetical; rewritten as non-load-bearing informational cite per POLICY 19 anti-volatile-pin. |
| v1.6 | 2026-06-14 | product-owner | (F-P6-001/002) PC5 stale-SHA label corrected + corroboration mechanism documented; (F-P6-004) H1+PC2 field-count 9→9-base+epic_status conditional; EC-012/EC-013 added; ADR cite convention: stable §Decision anchors (TD-VSDD-091). |
| v1.4 | 2026-06-14 | product-owner | (F-P4-003) ADR cite v1.3→v1.4 (cite-only). |
| v1.3 | 2026-06-14 | product-owner | PC8 added (EPIC-COMPLETE stdout surfacing per O-P3-002); ADR cite v1.1→v1.3. |
| v1.2 | 2026-06-14 | product-owner | PC2 wave_id phantom-field → derived value (sprint-state.yaml/current_step:); PC3 cross-check updated; PC5 side-channel → precompact-flush-log append-log; stale-SHA edge case; EC-006+EC-011+Arch Anchors updated. |
| v1.1 | 2026-06-14 | product-owner | PC3 re-anchored (sprint-state.yaml/current_step:; no phantom current_wave:); PC4 lock-held-or-absent; PC2 factory_lock_holder nullable; PC3 null-holder cross-check; PC5 precompact_flush_sha hard cross-check; EC-010 added; TBD-DI replaced with DI-020+DI-021+DI-023. |
| v1.0 | 2026-06-14 | product-owner | Initial creation (E-18 context-durability feature). |
