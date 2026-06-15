---
document_type: behavioral-contract
level: L3
version: "1.10"
status: draft
producer: product-owner
timestamp: 2026-06-15T00:00:00Z
last_amended: "2026-06-15 (v1.10) — F2 pass-19 fix-burst: (F-P19-001 MEDIUM) §VP Anchors stale VP title cites corrected: VP-083 bullet updated from obsolete 'Completeness Gate Is No-Op on Wave-1 and HANDOFF.md Absent' to current 'Completeness Gate Is No-Op on Wave-1 or Non-HANDOFF.md Writes' (connective and→or; second condition HANDOFF.md Absent→Non-HANDOFF.md Writes; per VP-083 v1.5 F-P16-003+F-P15-004); VP-081 bullet updated from 'Wave Cannot Close Without Verified Handoff' to 'Wave Cannot Close Without Verified Handoff (wave_id > 1)' (missing qualifier added; per VP-081 v1.5 title). Both VP titles now match their respective VP H1 headings verbatim. [Prior: 2026-06-15 (v1.9) — F2 pass-14 fix-burst: (F-P14-002) Precondition 1 TOML block corrected: bare logical name in `plugin =` replaced with canonical native-WASM shape — `name = \"validate-wave-handoff-completeness\"` + `plugin = \"hook-plugins/validate-wave-handoff-completeness.wasm\"`; matches sibling BCs BC-7.07.001/BC-7.07.002 and the regression-gate canonical shape. (F-P14-003) EC-006 made unconditional: removed ambiguous conditional clause; now reads 'Continue — `null` is a present, syntactically-valid value; the pure-parse gate does NOT perform the null-vs-log anti-fabrication check (that is BC-5.41.001 PC5's responsibility)'. [Prior: 2026-06-14 (v1.8) — F2 pass-10 fix-burst: (F-P10-001 MAJOR) Wave-1 no-op discriminator converted from external-read (sprint-state.yaml / prior HANDOFF.md on factory-artifacts / 'wave context cannot be determined') to PAYLOAD-ONLY form: the gate reads `wave_id` directly from the HANDOFF Write/Edit payload; `is_first_wave = (payload.wave_id == 1)`; wave_id==1 → wave-1 no-op (Continue); wave_id>1 → full validation. CRITICAL EC-010 behavior change: when `wave_id` is ABSENT from the payload → gate FAILS CLOSED (proceeds to full validation → HandoffIncomplete: ['wave_id']), NOT fail-open Continue. All residual 'sprint-state.yaml / prior HANDOFF.md / wave context cannot be determined' framing removed from the gate's own behavior across Description, PC3, PC4/Precondition-4, PC8, Inv3, EC-001..006/010, test vectors, and VP-083/VP-081 Verification Properties table rows. Note retained in Traceability ADR cite that 'the shell wave-handoff skill derives wave_id from real substrate' — this describes the caller, not the WASM gate. BC-4.14.001 v1.7→v1.8. [Prior: 2026-06-14 (v1.7) — F2 pass-8 fix-burst: (F-P8-001 MAJOR) PC2a EPIC-COMPLETE detection conjunct corrected: removed the 'AND all story entries in the parsed content have terminal status OR cannot be determined' conjunct from the WASM gate's discriminator; the WASM gate is PURE-PARSE and PAYLOAD-ONLY — the sole discriminator is `next_wave_stories: []` (empty list) per ADR-026 §Decision 8. The terminal-state judgment lives in the shell wave-gate (BC-5.41.002), NOT the WASM gate. PC2a now reads: if `next_wave_stories: []` → EPIC-COMPLETE branch; if `next_wave_stories` non-empty → non-EPIC-COMPLETE branch. Consistent with PC2a line-72 practical heuristic, Invariant 1 (pure-parse), and ADR-026 §Decision 8. [Prior: 2026-06-14 (v1.6) — F2 pass-6 fix-burst: (F-P6-004 MAJOR) H1 + §Description + PC1 conditional field validation: 'all 9 required fields' → 'all 9 base required fields (+ `epic_status: complete` conditionally required on EPIC-COMPLETE wave)'; PC added: gate detects EPIC-COMPLETE → adds `epic_status` to required set; on non-EPIC-COMPLETE wave, `epic_status` present → `UnexpectedEpicStatus`; EC-012/EC-013/EC-014 added; 3 test vectors added. (E-18) ADR cite convention: v1.5 version token dropped (TD-VSDD-091); stable §Decision anchors adopted. [Prior: 2026-06-14 (v1.5) — F2 pass-5 fix-burst: (F-P5-003 MAJOR) §Description phantom field eliminated: 'when `current_wave = 1`' replaced with canonical first-wave derivation from PC3 ('when pipeline context is the first wave — wave-group position 1 per sprint-state.yaml OR no prior HANDOFF.md on factory-artifacts'); ADR cite v1.4→v1.5. [Prior: 2026-06-14 (v1.4) — F2 pass-4 fix-burst: (F-P4-003) ADR cite v1.3→v1.4 (cite-only). [Prior: 2026-06-14 (v1.3) — F2 pass-3 fix-burst: ADR cite v1.1→v1.3. [Prior: 2026-06-14 (v1.2) — F2 pass-2 fix-burst: (F-P2-002 BLOCKER) EC-002/003/004/005/006 `current_wave = N` load-bearing references replaced with first-wave detection language (wave-group position per sprint-state.yaml OR absence of prior HANDOFF.md on factory-artifacts); test vector `any current_wave` → `any pipeline context`; VP-083 + VP-081 verification property rows updated to first-wave detection semantics — no phantom `current_wave:` field referenced anywhere in body. [Prior: 2026-06-14 (v1.1) — F2 pass-1 fix-burst: (F-1) Precondition 4 re-anchored: `current_wave` field removed; wave-1 no-op now derives from sprint-state.yaml dependency-order wave-group OR STATE.md `current_step:` for engine context. PC3 + PC8 updated to reflect real substrate (no phantom `current_wave:` field). (DI) TBD-DI replaced with DI-020.]"
phase: F2
inputs:
  - .factory/feature-delta/issue-173/F1-delta-analysis.md
  - .factory/specs/architecture/decisions/ADR-026-wave-boundary-checkpoint-reset-and-lossless-intra-wave-compaction.md
input-hash: "c2426d5"
traces_to: .factory/specs/prd.md
origin: greenfield
extracted_from: null
subsystem: "SS-04"
capability: "CAP-032"
lifecycle_status: draft
introduced: v1.0-feature-context-durability-E18
modified:
  - "2026-06-15 (v1.10) — F2 pass-19 fix-burst: (F-P19-001 MEDIUM) §VP Anchors stale VP title cites corrected: VP-083 bullet 'and HANDOFF.md Absent' → 'or Non-HANDOFF.md Writes'; VP-081 bullet missing qualifier '(wave_id > 1)' added. Both titles now match VP H1 headings verbatim."
  - "2026-06-15 (v1.9) — F2 pass-14 fix-burst: (F-P14-002) Precondition 1 TOML block corrected to canonical native-WASM shape: `name = \"validate-wave-handoff-completeness\"` + `plugin = \"hook-plugins/validate-wave-handoff-completeness.wasm\"`; bare logical name removed. (F-P14-003) EC-006 unconditional: ambiguous conditional clause replaced with 'Continue — null is syntactically valid; null-vs-log anti-fabrication check is BC-5.41.001 PC5 responsibility, not the pure-parse gate'."
  - "2026-06-14 (v1.8) — F2 pass-10 fix-burst: (F-P10-001 MAJOR) Wave-1 discriminator → PAYLOAD-ONLY `wave_id`: gate reads `wave_id` from HANDOFF payload; wave_id==1 → no-op; wave_id>1 → validate; wave_id ABSENT → FAILS CLOSED (HandoffIncomplete: ['wave_id']). All sprint-state.yaml/prior HANDOFF.md/factory-artifacts/wave-context-cannot-be-determined framing removed from gate behavior in Description, PC3, PC4, PC8, Inv3, EC-001..006/010, test vectors, VP-083/VP-081 rows."
  - "2026-06-14 (v1.7) — F2 pass-8 fix-burst: (F-P8-001 MAJOR) PC2a EPIC-COMPLETE detection conjunct corrected — removed terminal-status conjunct; WASM gate discriminator is PAYLOAD-ONLY (`next_wave_stories: []`); consistent with PC2a §practical heuristic, Invariant 1, and ADR-026 §Decision 8."
  - "2026-06-14 (v1.6) — F2 pass-6 fix-burst: (F-P6-004) H1+Description+PC conditional epic_status; EC-012/EC-013/EC-014; 3 test vectors; ADR cite convention: stable §Decision anchors (TD-VSDD-091)."
  - "2026-06-14 (v1.5) — F2 pass-5 fix-burst: (F-P5-003) §Description 'current_wave = 1' phantom → canonical first-wave derivation (wave-group position 1 per sprint-state.yaml OR no prior HANDOFF.md on factory-artifacts); ADR cite v1.4→v1.5."
  - "2026-06-14 (v1.4) — F2 pass-4 fix-burst: (F-P4-003) ADR cite v1.3→v1.4 (cite-only)."
  - "2026-06-14 (v1.3) — F2 pass-3 fix-burst: ADR cite v1.1→v1.3."
  - "2026-06-14 (v1.2) — F2 pass-2 fix-burst: EC-002/003/004/005/006 phantom current_wave = N → first-wave detection; test vector + VP-083 + VP-081 rows updated to first-wave detection semantics."
  - "2026-06-14 (v1.1) — F2 pass-1 fix-burst: PC4 + PC3 + PC8 re-anchored to real substrate (sprint-state.yaml wave-group order or STATE.md current_step: for engine context); phantom current_wave: field removed; TBD-DI replaced with DI-020; ADR cite v1.0→v1.1."
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# BC-4.14.001: validate-wave-handoff-completeness WASM gate blocks HandoffIncomplete on PostToolUse HANDOFF.md writes (9 base required fields + epic_status conditional on EPIC-COMPLETE); no-op when payload.wave_id==1; fails closed when wave_id absent; no-op on non-HANDOFF.md writes

## Description

`validate-wave-handoff-completeness` is a native WASM plugin registered as a PostToolUse gate on Write/Edit tool calls that target `HANDOFF.md` on `factory-artifacts`. When fired on a HANDOFF.md write, it validates that all 9 base required fields specified in ADR-026 §Decision 2 are syntactically present, and conditionally validates `epic_status: complete` when EPIC-COMPLETE context is detected. It blocks with `HandoffIncomplete` (listing all failing base fields) if any base field is missing or malformed. It blocks with `MissingEpicStatus` if the final wave lacks `epic_status`. It blocks with `UnexpectedEpicStatus` if `epic_status` is present on a non-final wave (malformed HANDOFF.md). Wave-1 no-op is determined PAYLOAD-ONLY: the gate reads `wave_id` directly from the HANDOFF Write/Edit payload and computes `is_first_wave = (payload.wave_id == 1)`; when `wave_id == 1` the gate returns `Continue` (no-op); when `wave_id > 1` the gate performs full validation; when `wave_id` is ABSENT from the payload the gate FAILS CLOSED (proceeds to full validation, blocking with `HandoffIncomplete: ["wave_id"]`). It is also a strict no-op when the tool call does not target `HANDOFF.md`. No external filesystem or git access is performed to determine wave identity — the gate reads exclusively from the Write/Edit tool call payload. No phantom `current_wave:` field is referenced — that field does not exist on STATE.md. This design follows the established factory WASM gate pattern (ADR-008, ADR-014) — deterministic parse-heavy validation with no filesystem or git side effects. Note: the shell wave-handoff skill (BC-5.41.001/BC-5.41.002) is responsible for deriving `wave_id` from real substrate (sprint-state.yaml wave-group ordering or factory-artifacts HANDOFF.md presence) and embedding it in the HANDOFF.md payload before writing.

## Preconditions

1. The plugin is registered in `hooks-registry.toml` as:
   ```toml
   [[hooks]]
   name = "validate-wave-handoff-completeness"
   event = "PostToolUse"
   plugin = "hook-plugins/validate-wave-handoff-completeness.wasm"
   tool = "Write|Edit"
   on_error = "continue"
   async = false
   timeout_ms = 5000
   ```
2. The plugin WASM binary exists at `plugins/vsdd-factory/hook-plugins/validate-wave-handoff-completeness.wasm`.
3. The invoking tool call is a Write or Edit targeting `HANDOFF.md` on `factory-artifacts` OR targeting a path that matches `*/HANDOFF.md`.
4. The HANDOFF.md payload being written contains a `wave_id` field (integer). The gate reads `wave_id` exclusively from the Write/Edit tool call payload — no external filesystem, sprint-state.yaml, or factory-artifacts access is performed. No phantom `current_wave:` field is referenced — that field does not exist on STATE.md. The shell wave-handoff skill (BC-5.41.001/BC-5.41.002) is responsible for correctly populating `wave_id` in the payload before the write occurs.

## Postconditions

1. **Happy path — all 9 base required fields present and syntactically valid (and epic_status conditionally valid)**: Gate returns `Continue`. Wave close may proceed. No block.

2. **HandoffIncomplete — missing or malformed base field**: Gate returns `block_intent = true`, exit code 2, with a structured block message naming each missing or malformed field:
   ```
   HandoffIncomplete: required fields missing or malformed: [<field1>, <field2>, ...]
   ```
   The block message must name ALL failing fields in a single invocation, not just the first.

2a. **Conditional field validation — `epic_status` (F-P6-004)**: The gate performs conditional validation of the `epic_status` field based on EPIC-COMPLETE context detection:
   - **EPIC-COMPLETE detection**: the gate determines EPIC-COMPLETE context by parsing the HANDOFF.md payload exclusively: if the current HANDOFF payload has `next_wave_stories: []` (empty list) → EPIC-COMPLETE branch (epic_status required); if `next_wave_stories` is non-empty → non-EPIC-COMPLETE branch (epic_status forbidden). The discriminator is PAYLOAD-ONLY. Terminal-state judgment on story entries is NOT performed by the WASM gate — that judgment belongs to the shell wave-gate (BC-5.41.002), which is responsible for payload consistency before the write occurs. No external filesystem or git access is performed.
   - **EPIC-COMPLETE context**: `epic_status` is REQUIRED and MUST equal `complete`. If absent: gate blocks with `MissingEpicStatus` (exit 2). If present but not `complete`: gate blocks with `HandoffIncomplete: epic_status malformed`.
   - **Non-EPIC-COMPLETE context**: `epic_status` MUST be absent from HANDOFF.md. If present (any value): gate blocks with `UnexpectedEpicStatus` (exit 2): `HandoffIncomplete: unexpected field epic_status on non-final wave`.
   - **Practical EPIC-COMPLETE heuristic for WASM**: since the gate is pure-parse with no external filesystem access, it uses the HANDOFF.md payload's `next_wave_stories` field as the discriminator: empty list (`[]`) → EPIC-COMPLETE branch; non-empty list → non-EPIC-COMPLETE branch. This is sufficient because wave-gate (BC-5.41.001) is responsible for ensuring the payload is internally consistent before writing.

3. **No-op rule (wave-1, payload-only)**: The gate reads `wave_id` directly from the HANDOFF Write/Edit payload (not from sprint-state.yaml, factory-artifacts, or any external source). It computes `is_first_wave = (payload.wave_id == 1)`. When `wave_id == 1`, the gate returns `Continue` unconditionally without performing field validation. This prevents friction on the first wave where no prior handoff exists. When `wave_id > 1`, full validation applies. When `wave_id` is ABSENT from the payload, the gate does NOT treat this as wave-1; instead it FAILS CLOSED: proceeds to full validation, which will block with `HandoffIncomplete: ["wave_id"]` (and any other missing required fields).

4. **No-op rule (non-HANDOFF.md write)**: When the PostToolUse event's tool call target path does not match `HANDOFF.md` (case-sensitive), the gate returns `Continue` immediately without reading or parsing any file.

5. **Fuel budget**: Gate completes within `timeout_ms = 5000` on any HANDOFF.md body up to 200 lines (ADR-026 Decision 8 200-line cap). Bodies exceeding 200 lines produce an advisory warning but are still parsed (gate does not hard-fail on length alone).

6. **on_error = "continue"**: A gate crash (WASM panic, fuel exhaustion) results in `Continue` (fail-open). Gate crash is logged to the dispatcher internal log with `plugin.crashed` record.

7. **Field validation scope**: The gate validates field PRESENCE and basic syntactic form only (field key exists; value is non-empty or null only where null is permitted) for all 9 base required fields, plus conditional `epic_status` validation per PC2a. It does NOT perform anti-fabrication cross-checks against git or filesystem — those are performed by the `wave-handoff` skill (BC-5.41.001). Cross-checks require side effects; the WASM gate is pure-parse per ADR-026 §Decision 8.

8. **Wave-1 no-op is unconditional (payload.wave_id == 1)**: When `payload.wave_id == 1`, even if `HANDOFF.md` is written with deliberate content, the gate does not validate it. Validation only activates when `payload.wave_id > 1`. When `wave_id` is ABSENT from the payload, the gate does NOT default to fail-open — it FAILS CLOSED (full validation proceeds, blocking with `HandoffIncomplete: ["wave_id"]` at minimum).

## Invariants

1. **No git or filesystem side effects**: The WASM gate reads only the Write/Edit tool call payload (the content being written). It does NOT exec git, read other files, or access the filesystem. It is a pure-parse WASM function. This is the invariant distinguishing it from the `wave-handoff` skill's anti-fabrication checks.

2. **Block message names ALL failing fields**: A partial block message (naming only the first missing field) is a specification violation. The operator must see the complete failure set in one invocation.

3. **No-op conditions are checked first**: The gate checks the no-op conditions (non-HANDOFF.md target, then `wave_id == 1`) before any further parsing. Wave-1 detection reads `wave_id` from the payload — it is NOT checked against sprint-state.yaml, factory-artifacts, or any external source. A gate that parses HANDOFF.md fully and then returns Continue when `wave_id == 1` is correct in outcome but wastes fuel unnecessarily. Absent `wave_id` is NOT a no-op condition — it proceeds to full validation (fail-closed).

4. **Field ordering in HandoffIncomplete is deterministic**: Fields are listed in the same order as ADR-026 §Decision 2 schema table for reproducibility in test assertions.

5. **200-line HANDOFF.md cap is advisory**: The gate parses bodies > 200 lines without hard failure but emits `plugin.log` warning with `level: warn` and `message: "HANDOFF.md body exceeds 200-line advisory cap"`.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | HANDOFF.md write; payload contains `wave_id: 1` | Continue (wave-1 no-op per payload.wave_id==1); no validation |
| EC-002 | HANDOFF.md write; all 9 fields present; payload contains `wave_id: 2` (not first wave) | Continue; no block |
| EC-003 | HANDOFF.md write; `last_verified_develop_sha` field missing; payload `wave_id: 2` (not first wave) | HandoffIncomplete: `["last_verified_develop_sha"]` |
| EC-004 | HANDOFF.md write; `last_verified_develop_sha` present but empty string; payload `wave_id: 2` (not first wave) | HandoffIncomplete: field present but empty is treated as malformed |
| EC-005 | HANDOFF.md write; 4 fields missing; payload `wave_id: 3` (not first wave) | HandoffIncomplete: names all 4 missing fields in one message |
| EC-006 | HANDOFF.md write; `precompact_flush_sha: null`; payload `wave_id: 2` (not first wave) | Continue — `null` is a present, syntactically-valid value; the pure-parse gate does NOT perform the null-vs-log anti-fabrication check (that is BC-5.41.001 PC5's responsibility) |
| EC-007 | Edit to `STATE.md` (not HANDOFF.md) | Continue (non-HANDOFF.md no-op); gate fires but immediately returns Continue |
| EC-008 | Gate crashes (WASM panic) | fail-open Continue; plugin.crashed log; compaction/wave-close not blocked |
| EC-009 | HANDOFF.md body is 350 lines (over 200-line cap) | Gate parses it; emits advisory warn; validates all fields normally |
| EC-010 | HANDOFF.md write; `wave_id` field is ABSENT from the payload | Gate FAILS CLOSED: proceeds to full validation; blocks with `HandoffIncomplete: ["wave_id"]` (plus any other missing required fields); does NOT return Continue (absent wave_id is NOT treated as wave-1) |
| EC-011 | wave-gate invoked with intent to close wave > 1; no HANDOFF.md exists yet | Gate not triggered (no Write to HANDOFF.md occurred); wave-gate skill is responsible for ensuring HANDOFF.md is written before close — the gate validates the write, not the absence |
| EC-012 | HANDOFF.md write on non-final wave includes `epic_status: complete` | Gate blocks with `UnexpectedEpicStatus` (exit 2); `epic_status` MUST be absent on non-final waves (`next_wave_stories` is non-empty) |
| EC-013 | HANDOFF.md write on EPIC-COMPLETE final wave (`next_wave_stories: []`) is missing `epic_status` | Gate blocks with `MissingEpicStatus` (exit 2); `epic_status: complete` is required on the final wave |
| EC-014 | HANDOFF.md write on EPIC-COMPLETE final wave with all 9 base fields + `epic_status: complete`; `next_wave_stories: []` | Gate returns `Continue`; all base fields valid + conditional epic_status valid |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Write to HANDOFF.md; payload `wave_id: 1` | Continue (wave-1 no-op; payload.wave_id==1) | wave-1-no-op |
| Write to HANDOFF.md; payload `wave_id: 2`; all 9 fields present + well-formed | Continue | happy-path |
| Write to HANDOFF.md; payload `wave_id: 2`; `wave_id` present (required field present; other field missing); e.g. `last_verified_develop_sha` missing | `HandoffIncomplete: ["last_verified_develop_sha"]`; exit 2 | missing-field-single |
| Write to HANDOFF.md; `wave_id` ABSENT from payload | `HandoffIncomplete: ["wave_id"]`; exit 2 (fails-closed; not Continue) | wave-id-absent-fails-closed |
| Write to HANDOFF.md; second wave; 3 fields missing | `HandoffIncomplete: ["<f1>", "<f2>", "<f3>"]`; exit 2 | missing-field-multiple |
| Write to STATE.md; any pipeline context | Continue (non-HANDOFF.md target) | non-target-no-op |
| WASM panic during field parse | Continue; `plugin.crashed` in dispatcher log | crash-fail-open |
| Write to HANDOFF.md; body = 350 lines; all fields present | Continue + advisory warn in plugin.log | over-cap-advisory |
| Write to HANDOFF.md; non-final wave (`next_wave_stories` non-empty); `epic_status: complete` present | `UnexpectedEpicStatus`; exit 2 | non-final-unexpected-epic-status |
| Write to HANDOFF.md; EPIC-COMPLETE final wave (`next_wave_stories: []`); all 9 base fields + `epic_status: complete` | Continue; all fields valid | epic-complete-happy-path |
| Write to HANDOFF.md; EPIC-COMPLETE final wave (`next_wave_stories: []`); all 9 base fields; `epic_status` absent | `MissingEpicStatus`; exit 2 | epic-complete-missing-epic-status |

## Related BCs

- BC-5.41.001 — composes with: this gate validates the HANDOFF.md written by wave-gate/wave-handoff; anti-fabrication cross-checks are the skill's responsibility
- BC-4.13.001 — sibling: verify-factory-lock WASM gate; shares the same WASM plugin pattern (pure-parse, no git side effects, fail-open)
- BC-1.15.001 — depends on: dispatcher routing must handle PostToolUse events for WASM plugins

## Architecture Anchors

- `crates/hook-plugins/validate-wave-handoff-completeness/` — NEW Rust crate; `[[bin]]`-bearing; produces `validate-wave-handoff-completeness.wasm`
- `plugins/vsdd-factory/hooks-registry.toml` — `[[hooks]]` entry for `validate-wave-handoff-completeness`; PostToolUse; Write|Edit; on_error=continue; timeout_ms=5000
- ADR-026 §Decision 8 (WASM for completeness gate; 200-line HANDOFF.md cap) and §Decision 9 (wave-1 no-op rule)

## Story Anchor

S-18.02 (validate-wave-handoff-completeness WASM gate crate + registry)

## VP Anchors

- VP-083 — Completeness Gate Is No-Op on Wave-1 or Non-HANDOFF.md Writes
- VP-081 — Wave Cannot Close Without Verified Handoff (wave_id > 1) (partial anchor — VP-083 covers the no-op leg; VP-081 covers the blocking leg)

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-083 | validate-wave-handoff-completeness returns Continue unconditionally when `payload.wave_id == 1` (payload-only; no filesystem access); also no-op on non-HANDOFF.md writes; when `wave_id` is ABSENT from payload the gate FAILS CLOSED (does NOT return Continue) | unit-test |
| VP-081 | Gate blocks HandoffIncomplete when any required ADR-026 §D2 field is missing on a HANDOFF.md write with `payload.wave_id > 1`; gate also blocks HandoffIncomplete: ["wave_id"] when wave_id is absent from payload (fail-closed) | integration |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-032 ("Guarantee lossless context-window transitions via wave-boundary checkpoint and PreCompact flush") per capabilities.md §CAP-032 |
| Capability Anchor Justification | CAP-032 ("Guarantee lossless context-window transitions via wave-boundary checkpoint and PreCompact flush") per capabilities.md §CAP-032 — this BC specifies the WASM completeness gate that enforces HANDOFF.md integrity at write time, preventing a partial or incomplete handoff artifact from being committed and corrupting the wave-boundary continuity guarantee; it directly enforces ADR-026 Decision 8 and Decision 9 |
| L2 Domain Invariants | DI-020 (Wave/phase boundary transitions must not lose load-bearing pipeline state — enforced by this gate blocking incomplete HANDOFF.md writes) |
| Architecture Module | SS-04 (Plugin Ecosystem) — new WASM crate under `crates/hook-plugins/validate-wave-handoff-completeness/` |
| ADR | ADR-026 §Decision 8 (WASM for completeness gate; deterministic parse-heavy validation; pure-parse; no filesystem or git side effects), §Decision 9 (no-op on wave-1; wave identity determined PAYLOAD-ONLY from `wave_id` field in HANDOFF Write/Edit payload; wave_id==1 → no-op; wave_id absent → fail-closed HandoffIncomplete; note: the shell wave-handoff skill derives wave_id from real substrate — sprint-state.yaml wave-group order or factory-artifacts HANDOFF.md presence — and embeds it in the payload before writing; per F-P10-001) |
| Stories | S-18.02 |
| Cycle | v1.0-feature-context-durability-E18 (F2) |
| Feature | issue #173 / E-18 |

## Changelog

| Version | Date | Author | Change |
|---------|------|--------|--------|
| v1.10 | 2026-06-15 | product-owner | (F-P19-001 MEDIUM) §VP Anchors stale VP title cites corrected: VP-083 bullet updated from obsolete 'Completeness Gate Is No-Op on Wave-1 and HANDOFF.md Absent' to current 'Completeness Gate Is No-Op on Wave-1 or Non-HANDOFF.md Writes' (connective and→or; second condition HANDOFF.md Absent→Non-HANDOFF.md Writes; per VP-083 v1.5 F-P16-003+F-P15-004); VP-081 bullet updated from 'Wave Cannot Close Without Verified Handoff' to 'Wave Cannot Close Without Verified Handoff (wave_id > 1)' (missing qualifier added; per VP-081 current H1). |
| v1.9 | 2026-06-15 | product-owner | (F-P14-002) Precondition 1 TOML block corrected to canonical native-WASM shape: `name = "validate-wave-handoff-completeness"` + `plugin = "hook-plugins/validate-wave-handoff-completeness.wasm"`; bare logical name in `plugin =` without `name =` removed. (F-P14-003) EC-006 made unconditional: ambiguous conditional clause replaced with clear statement that null is syntactically valid and that null-vs-log anti-fabrication check is BC-5.41.001 PC5's responsibility, not the pure-parse gate's. |
| v1.8 | 2026-06-14 | product-owner | (F-P10-001 MAJOR) Wave-1 discriminator → PAYLOAD-ONLY `wave_id`; wave_id==1 → no-op; wave_id absent → FAILS CLOSED. All sprint-state.yaml/factory-artifacts framing removed from gate behavior. |
| v1.7 | 2026-06-14 | product-owner | (F-P8-001 MAJOR) PC2a EPIC-COMPLETE detection conjunct corrected — terminal-status conjunct removed; WASM discriminator is PAYLOAD-ONLY (`next_wave_stories: []`). |
| v1.6 | 2026-06-14 | product-owner | (F-P6-004) H1+Description+PC conditional epic_status; EC-012/EC-013/EC-014; 3 test vectors; ADR cite stable §Decision anchors (TD-VSDD-091). |
| v1.5 | 2026-06-14 | product-owner | (F-P5-003) §Description phantom `current_wave = 1` → canonical first-wave derivation; ADR cite v1.4→v1.5. |
| v1.4 | 2026-06-14 | product-owner | (F-P4-003) ADR cite v1.3→v1.4 (cite-only). |
| v1.3 | 2026-06-14 | product-owner | ADR cite v1.1→v1.3. |
| v1.2 | 2026-06-14 | product-owner | (F-P2-002) EC-002..006 phantom `current_wave = N` → first-wave detection language; VP-083+VP-081 rows updated. |
| v1.1 | 2026-06-14 | product-owner | (F-1) PC4+PC3+PC8 re-anchored to real substrate; phantom `current_wave:` removed; TBD-DI replaced with DI-020; ADR cite v1.0→v1.1. |
| v1.0 | 2026-06-14 | product-owner | Initial creation (E-18 context-durability feature). |
