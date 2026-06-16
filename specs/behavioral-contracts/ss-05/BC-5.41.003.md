---
document_type: behavioral-contract
level: L3
version: "1.8"
status: draft
producer: product-owner
timestamp: 2026-06-14T00:00:00Z
last_amended: "2026-06-15 (v1.8) — fix burst (product-owner): (F-P35-001 MEDIUM) §Changelog section added — was structurally absent (type-parity gap with sibling E-18 BCs per POLICY 17); rows reconstructed from modified[] frontmatter; no skip-marker needed because this BC has a real v1.5 entry (F-P5-002 re-grounding). [Prior: 2026-06-15 (v1.7) — F-P28-001+F-P28-002: (F-P28-001) §Description 'WASM/bash hooks' → 'WASM hooks'; §Architecture Anchors '(or bash equivalent)' hedges removed from both validate-burst-log.wasm and validate-dispatch-advance.wasm anchor lines — both are native WASM plugins under crates/hook-plugins/, no bash equivalent exists (consistent with PC1 WASM-only determination and VP-084 F-P5-004). (F-P28-002) §Traceability DI-025 cell: 'side-channel file' → 'precompact-flush-log (append-only)', consistent with BC-5.41.001 v1.13 F-P22-003 and invariants.md DI-025 canonical terminology. [Prior: 2026-06-14 (v1.6) — F2 pass-6 fix-burst: (E-18) ADR cite convention: v1.5 version token dropped per ADR-026 §BC Traceability Cite Convention (TD-VSDD-091 anti-volatile-pin); stable §Decision anchor adopted (cite-only change). [Prior: 2026-06-14 (v1.5) — F2 pass-5 fix-burst: (F-P5-002 re-grounding) PC1 case (b) and case (c) label corrected: 'write-before-push crash' replaced with 'log corruption or truncation' — the write-before-push-crash justification is mechanically impossible because the append step now occurs AFTER the commit (and on append failure BC-7.07.001 v1.5 resets the commit); Inv1 field-4 ≠ `commit` condition label updated symmetrically; EC-003 stale-entry row updated; ADR cite v1.4→v1.5. [Prior: 2026-06-14 (v1.4) — F2 pass-4 fix-burst: (O-P4-004 MINOR) WASM gate corroboration mechanism corrected: WASM plugins have no exec_subprocess; replaced all `git cat-file -t <SHA>` clauses with FOURTH FIELD (`commit` token) pre-embedded read; (F-P4-002 MAJOR) obsolete `PreCompact flush <N>` / `PreCompact flush 2` commit-message form replaced throughout with canonical `PreCompact flush <cycle>/<step> <ISO-timestamp>`; H1 updated; PC1, PC2, PC4-bats, Inv1, EC-001, EC-003, Canonical Test Vectors, Architecture Anchors all updated; ADR cite v1.3→v1.4. [Prior: 2026-06-14 (v1.3) — F2 pass-3 fix-burst: ADR cite v1.1→v1.3. [Prior: 2026-06-14 (v1.2) — F2 pass-2 fix-burst: (F-P2-003 append-log) H1 enriched with corroboration detail (precompact-flush-log append-log last-line + git cat-file -t). PC1 SHA corroboration updated: last-precompact-flush-sha → precompact-flush-log last line + git cat-file -t; stale-entry case (git cat-file -t returns non-commit → treat as absent → prefix match sufficient) added. Inv 1 updated symmetrically. EC-003 truth table: stale-SHA row added (write-before-push crash → EXEMPT). Architecture Anchors updated: last-precompact-flush-sha → precompact-flush-log append-log. [Prior: 2026-06-14 (v1.1) — F2 pass-1 fix-burst: (F-8) Exemption prefix updated from `^PreCompact flush ` to general `^PreCompact flush ` (per locked convention; BC-7.07.001 commit message now `PreCompact flush <cycle>/<step>` not `PreCompact flush <N>`). PC1 + Inv 1 + Inv 3 updated with new prefix. EC-003 rewritten as concrete HEAD/HEAD^ truth table. Security tightening: exempted commit SHA MUST be corroborated by `.factory/hooks/last-precompact-flush-sha` side-channel file when that file exists, to prevent arbitrary `PreCompact flush` prefix bypass. (DI) TBD-DI replaced with DI-020+DI-025.]"
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
  - "2026-06-15 (v1.8) — fix burst (product-owner): (F-P35-001) §Changelog section added (structural type-parity gap closed; sibling E-18 BCs all carry §Changelog per POLICY 17); rows reconstructed from modified[] frontmatter; no skip-marker needed (real v1.5 entry exists)."
  - "2026-06-15 (v1.7) — F-P28-001+F-P28-002: §Description 'WASM/bash hooks' → 'WASM hooks'; §Architecture Anchors '(or bash equivalent)' removed (native WASM, no bash equivalent per PC1/VP-084); §Traceability DI-025 'side-channel file' → 'precompact-flush-log (append-only)'."
  - "2026-06-14 (v1.6) — F2 pass-6 fix-burst: ADR cite convention: stable §Decision anchor (TD-VSDD-091); cite-only."
  - "2026-06-14 (v1.5) — F2 pass-5 fix-burst: (F-P5-002 re-grounding) PC1 case (b)/(c) + Inv1 + EC-003: 'write-before-push crash' → 'log corruption or truncation'; mechanically impossible write-before-push justification removed; ADR cite v1.4→v1.5."
  - "2026-06-14 (v1.4) — F2 pass-4 fix-burst: (O-P4-004) WASM gate corroboration: git cat-file -t → fourth-field `commit` token read; (F-P4-002) obsolete PreCompact flush <N>/flush 2 → canonical PreCompact flush <cycle>/<step> <ISO-timestamp> throughout; ADR cite v1.3→v1.4."
  - "2026-06-14 (v1.3) — F2 pass-3 fix-burst: ADR cite v1.1→v1.3."
  - "2026-06-14 (v1.2) — F2 pass-2 fix-burst: H1 enriched; PC1+Inv1 side-channel → precompact-flush-log append-log last-line + git cat-file -t; stale-SHA (write-before-push crash → EXEMPT) case added; EC-003 truth table + Architecture Anchors updated."
  - "2026-06-14 (v1.1) — F2 pass-1 fix-burst: prefix updated to general `PreCompact flush ` (removes `wave-<N>` specificity per locked convention); F-8 SHA corroboration against side-channel file; EC-003 HEAD/HEAD^ truth table; TBD-DI replaced with DI-020+DI-025; ADR cite v1.0→v1.1."
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# BC-5.41.003: validate-burst-log and validate-dispatch-advance exempt commits with "PreCompact flush " prefix from MULTI_COMMIT_CHAIN_NOT_ALLOWED (F-8: SHA corroboration via precompact-flush-log append-log last-line FOURTH FIELD pre-embedded `commit` token — WASM reads embedded field, does NOT exec git)

## Description

The `validate-burst-log` and `validate-dispatch-advance` WASM hooks implement the `MULTI_COMMIT_CHAIN_NOT_ALLOWED` detector (TD-VSDD-053), which blocks sequential `factory-artifacts` commits whose subjects both contain sentinel words like "backfill", "Stage 1", "Stage 2". The PreCompact flush hook (BC-7.07.001) produces commits with subject prefix `PreCompact flush <cycle>/<step> <ISO-timestamp>`, which are lifecycle-orthogonal to state-manager bursts (ADR-026 Decision 10). These commits MUST be exempt from the chain detector. Without this exemption, a PreCompact flush commit followed by a legitimate burst commit would trigger a false-positive `MULTI_COMMIT_CHAIN_NOT_ALLOWED` block, making the factory unworkable after any compaction event. This BC is MANDATORY — a factory where PreCompact flush commits block subsequent state-manager bursts is production-blocking.

## Preconditions

1. `validate-burst-log` and `validate-dispatch-advance` hooks are active and operational.
2. A `PreCompact flush <cycle>/<step> <ISO-timestamp>` commit has been made to `factory-artifacts` by `precompact-flush.sh` (BC-7.07.001).
3. A state-manager burst is subsequently attempted (producing a Commit A/B/C/D/E sequence).

## Postconditions

1. **Exemption by prefix match + SHA corroboration (F-8)**: Both `validate-burst-log` and `validate-dispatch-advance` treat any commit whose subject matches the pattern `^PreCompact flush ` as lifecycle-orthogonal, BUT ONLY when the commit's SHA can be corroborated against the LAST LINE of the append-log file `.factory/hooks/precompact-flush-log`. The log line has four space-separated fields: `<ISO-timestamp> <SHA> <cycle>/<step> commit`. The WASM gate reads the SECOND FIELD (the SHA) and the FOURTH FIELD (the object-type token) of the last line. **The WASM gate does NOT exec `git cat-file -t` — it has no exec_subprocess capability.** Instead, it reads the FOURTH FIELD of the log line: (a) if the append-log exists and its last line has FOURTH FIELD == `commit`, the exempted commit's SHA MUST equal the SECOND FIELD SHA (exact match; no partial match). **SHA-mismatch with valid field-4=`commit` is NOT EXEMPT** — this outcome is enforced in-body here (not only in EC-003) because allowing exemption on SHA-mismatch-with-valid-field-4 would permit arbitrary-prefix bypass (an agent authors a commit with `PreCompact flush ` subject while the log records a different SHA; the mismatch is detectable and is treated as suspicious; chain detection applies normally); (b) if the last line's FOURTH FIELD is absent, empty, or not the literal string `commit` (indicating log corruption or truncation), treat the log entry as stale and fall through to case (c); (c) if the append-log is genuinely absent (`test -f` returns false), or the last line is empty, the prefix match alone is sufficient for exemption. This prevents an arbitrary agent from authoring a commit with a `PreCompact flush ` subject to bypass TD-VSDD-053 without having actually run the flush hook. Exempt commits are excluded from the HEAD/HEAD^ chain comparison that detects `MULTI_COMMIT_CHAIN_NOT_ALLOWED`.

2. **No false-positive block**: After a `PreCompact flush <cycle>/<step> <ISO-timestamp>` commit, a subsequent state-manager burst commit (e.g., `state: advance to phase X`) does NOT trigger `MULTI_COMMIT_CHAIN_NOT_ALLOWED`. The burst dispatch proceeds normally.

3. **Normal chain detection preserved**: The exemption is ONLY for commits matching `^PreCompact flush `. All other commit subject patterns continue to be evaluated for chain violations. The exemption does not weaken the general TD-VSDD-053 enforcement.

4. **Bats test coverage**: A bats integration test verifies the exemption: (1) simulate a `PreCompact flush v1.0-feature-context-durability-E18/S-18.04 2026-06-14T00:00:00Z` commit on factory-artifacts; (2) simulate a subsequent burst commit; (3) verify that `validate-burst-log` and `validate-dispatch-advance` return `block_intent = false` (no `MULTI_COMMIT_CHAIN_NOT_ALLOWED`).

5. **Burst-log entry clarity**: The burst-log entry for the state-manager burst MUST NOT cite the PreCompact commit as Commit A/B/C/D/E. It is not a burst commit. If the burst-log author includes it, that is an INV-019 violation.

## Invariants

1. **Prefix + SHA corroboration, not subject-based inference alone**: The exemption check has three gates: (1) prefix match: `subject.starts_with("PreCompact flush ")` on the raw commit subject string (`git log --format=%s -1 <SHA>`); (2) when `.factory/hooks/precompact-flush-log` exists and its last line is non-empty: parse the last line as four space-separated fields `<ISO-timestamp> <SHA-field> <cycle>/<step> <type-field>`; the WASM gate reads `<SHA-field>` (field 2) and `<type-field>` (field 4) — it does NOT exec `git cat-file -t`; if `<type-field>` == `commit` (literal string), the exempted commit's SHA MUST equal `<SHA-field>` — a SHA-mismatch with valid `<type-field>==commit` is NOT EXEMPT (prevents arbitrary-prefix bypass where an attacker authors a `PreCompact flush ` commit while the log records a different SHA; the valid field-4 means the log is not stale, so the mismatch is a security-relevant discrepancy, not a tolerable stale-entry); (3) if the log is genuinely absent, empty, or `<type-field>` is not the literal `commit` (log corruption or truncation), the prefix match alone is sufficient for exemption. NLP inference, regex over the full commit body, and sentiment analysis are all forbidden.

2. **Both hooks must implement the exemption symmetrically**: `validate-burst-log` and `validate-dispatch-advance` are both co-owners of this exemption. An implementation that exempts only one of the two leaves the other as a source of false-positive blocks. Symmetric implementation is MANDATORY.

3. **The exemption is not a general escape hatch**: Commits with subjects starting with arbitrary text (e.g., "My flush wave-") are NOT exempt. Only the exact prefix `PreCompact flush ` (case-sensitive, as produced by `precompact-flush.sh`) is exempt.

4. **TD-VSDD-053 baseline is unchanged**: The `MULTI_COMMIT_CHAIN_NOT_ALLOWED` rule for "backfill", "Stage 1", "Stage 2" sentinel words is unaffected by this exemption. The exemption adds a conditional skip; it does not remove or weaken the baseline detector.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | `PreCompact flush v1.0-feature-context-durability-E18/S-18.04 2026-06-14T00:00:00Z` followed by burst commit | No block; exemption fires; burst proceeds |
| EC-002 | Two consecutive `PreCompact flush ` commits (rapid double-compaction) | Neither triggers MULTI_COMMIT_CHAIN_NOT_ALLOWED; both are individually exempt |
| EC-003 | HEAD/HEAD^ truth table — concrete testable cases | HEAD=`PreCompact flush v1.0-feature-context-durability-E18/S-18.04 2026-06-14T00:00:00Z` (log last line: `2026-06-14T00:00:00Z <SHA> v1.0-feature-context-durability-E18/S-18.04 commit`; WASM reads field-4 == `commit`); HEAD^=`state: burst-23 Commit D` → EXEMPT (no block). HEAD=`state: burst-24 Commit A`; HEAD^=`PreCompact flush v1.0-feature-context-durability-E18/S-18.04 2026-06-14T00:00:00Z` (SHA corroborated via field-4 == `commit`) → EXEMPT (HEAD^ is exempt; HEAD is a normal burst). HEAD=`stage 1 backfill`; HEAD^=`stage 2 backfill` → BLOCK (no PreCompact commit involved; normal chain detection). HEAD=`PreCompact flush injected`; HEAD^=`state: burst-X`; precompact-flush-log last-line field-2 SHA is a different SHA (mismatch) → NOT EXEMPT (SHA mismatch; treated as suspicious; chain detection applies normally). HEAD=`PreCompact flush v1.0-feature-context-durability-E18/S-18.04 2026-06-14T00:00:00Z`; precompact-flush-log genuinely absent → EXEMPT (file absence mechanically verified; WASM reads no file, prefix match alone sufficient). HEAD=`PreCompact flush v1.0-feature-context-durability-E18/S-18.04 2026-06-14T00:00:00Z`; precompact-flush-log last-line field-4 is NOT `commit` (log corruption or truncation; stale entry) → EXEMPT (stale entry treated as absent; prefix match alone sufficient). |
| EC-004 | Subject starts with "precompact flush wave-" (lowercase) | NOT exempt; prefix match is case-sensitive. Only `PreCompact flush ` (capitalized) as produced by the canonical hook is exempt. |
| EC-005 | validate-burst-log implements exemption; validate-dispatch-advance does not | validate-dispatch-advance fires a false-positive block on the burst dispatch. Specification violation — both must be symmetric. |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| HEAD = `PreCompact flush v1.0-feature-context-durability-E18/S-18.04 2026-06-14T00:00:00Z`; HEAD^ = `state: burst-23 Commit D`; log last line field-4 == `commit` and field-2 SHA matches HEAD | validate-burst-log: Continue (PreCompact HEAD is exempt from being the backfill-chain trigger); validate-dispatch-advance: Continue | happy-path-exempt |
| HEAD = `state: burst-24 Commit A`; HEAD^ = `PreCompact flush v1.0-feature-context-durability-E18/S-18.04 2026-06-14T00:00:00Z`; log last line field-4 == `commit` and field-2 SHA matches HEAD^ | No MULTI_COMMIT_CHAIN violation (HEAD^ is PreCompact-exempt; HEAD is a normal burst commit) | burst-after-precompact |
| HEAD = `stage 1 backfill`; HEAD^ = `stage 2 backfill` | MULTI_COMMIT_CHAIN_NOT_ALLOWED (normal TD-VSDD-053 detection; PreCompact exemption not triggered) | normal-chain-detection-preserved |
| bats: simulate PreCompact flush commit + burst commit sequence | validate-burst-log exits 0 (Continue); validate-dispatch-advance exits 0 (Continue) | bats-integration |

## Related BCs

- BC-7.07.001 — depends on: precompact-flush.sh produces commits with `PreCompact flush ` prefix that this BC exempts
- BC-5.39.001 — sibling: 3-CLEAN convergence protocol; TD-VSDD-053 single-commit-per-burst; this BC defines the PreCompact lifecycle boundary that keeps those protocols intact

## Architecture Anchors

- `plugins/vsdd-factory/hook-plugins/validate-burst-log.wasm` — native WASM plugin (crates/hook-plugins/); must be amended with `PreCompact flush ` prefix exemption + precompact-flush-log last-line FOURTH FIELD (`commit` token) corroboration; WASM does NOT exec git; no bash equivalent exists
- `plugins/vsdd-factory/hook-plugins/validate-dispatch-advance.wasm` — native WASM plugin (crates/hook-plugins/); must be amended symmetrically; same fourth-field corroboration logic; WASM does NOT exec git; no bash equivalent exists
- `.factory/hooks/precompact-flush-log` — append-log written by precompact-flush.sh; each flush appends a 4-field line `<ISO-timestamp> <SHA> <cycle>/<step> commit`; hooks read LAST LINE; corroboration is via FIELD 4 == literal `commit` (pre-embedded by shell at write time, not exec'd at read time) and FIELD 2 SHA match
- ADR-026 §Decision 10 — PreCompact flush lifecycle is distinct from state-manager burst lifecycle; exemption rationale; append-log design

## Story Anchor

S-18.04 (precompact-flush.sh shell hook + registry; includes validate-burst-log + validate-dispatch-advance exemption as mandatory deliverable)

## VP Anchors

- VP-084 — PreCompact Flush Commit Is Lifecycle-Distinct From State-Manager Burst Commit

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-084 | validate-burst-log exempts commits with "PreCompact flush " prefix from MULTI_COMMIT_CHAIN_NOT_ALLOWED detector; validate-dispatch-advance applies same exemption symmetrically | integration |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-032 ("Guarantee lossless context-window transitions via wave-boundary checkpoint and PreCompact flush") per capabilities.md §CAP-032 |
| Capability Anchor Justification | CAP-032 ("Guarantee lossless context-window transitions via wave-boundary checkpoint and PreCompact flush") per capabilities.md §CAP-032 — this BC is a MANDATORY enabler of the PreCompact flush (CAP-032 Part B); without the MULTI_COMMIT_CHAIN exemption, the flush hook would produce commits that block subsequent state-manager bursts, making the flush a production-blocking regression rather than a durability improvement; this BC closes ADR-026 §Decision 10 and F1 regression risk §4.1 R5 |
| L2 Domain Invariants | DI-020 (Wave/phase boundary transitions must not lose load-bearing pipeline state — this exemption is a mandatory enabler: without it, PreCompact flush commits block subsequent bursts, making durability a production regression); DI-025 (PreCompact flush commits are lifecycle-orthogonal to state-manager burst commits — enforced by the exemption rule and SHA corroboration against precompact-flush-log (append-only)) |
| Architecture Module | SS-05 (Pipeline Orchestration) — burst-log and dispatch-advance validation logic is orchestration-layer governance (SS-05 behavioral contract) even though the hook implementations may live in SS-04 WASM or SS-07 bash |
| ADR | ADR-026 §Decision 10 (PreCompact flush lifecycle distinct from state-manager burst lifecycle; validate-burst-log + validate-dispatch-advance must exempt commits with `PreCompact flush ` prefix + SHA corroboration via precompact-flush-log last-line fourth-field `commit` token; WASM reads embedded field, does not exec git; exemption is case-sensitive; stale-entry label is log corruption or truncation, not write-before-push-crash per F-P5-002) |
| Stories | S-18.04 |
| Cycle | v1.0-feature-context-durability-E18 (F2) |
| Feature | issue #173 / E-18 |

## Changelog

| Version | Date | Author | Change |
|---------|------|--------|--------|
| v1.8 | 2026-06-15 | product-owner | (F-P35-001) §Changelog section added — was structurally absent (type-parity gap with sibling E-18 BCs per POLICY 17); rows reconstructed from modified[] frontmatter; no skip-marker needed because this BC received a real behavioral change at v1.5 (F-P5-002 re-grounding). |
| v1.7 | 2026-06-15 | product-owner | (F-P28-001) §Description 'WASM/bash hooks' → 'WASM hooks'; §Architecture Anchors '(or bash equivalent)' hedges removed from both hook anchor lines (native WASM, no bash equivalent per PC1/VP-084). (F-P28-002) §Traceability DI-025 cell: 'side-channel file' → 'precompact-flush-log (append-only)'. |
| v1.6 | 2026-06-14 | product-owner | (E-18) ADR cite convention: v1.5 version token dropped per ADR-026 §BC Traceability Cite Convention (TD-VSDD-091 anti-volatile-pin); stable §Decision anchor adopted (cite-only change). |
| v1.5 | 2026-06-14 | product-owner | (F-P5-002 re-grounding) PC1 case (b)/(c) label corrected: 'write-before-push crash' → 'log corruption or truncation' (mechanically impossible write-before-push justification removed; append occurs AFTER commit); Inv1 field-4 ≠ `commit` label updated symmetrically; EC-003 stale-entry row updated; ADR cite v1.4→v1.5. |
| v1.4 | 2026-06-14 | product-owner | (O-P4-004) WASM gate corroboration: `git cat-file -t <SHA>` → FOURTH FIELD (`commit` token) pre-embedded read (WASM has no exec_subprocess). (F-P4-002) Obsolete `PreCompact flush <N>`/`flush 2` → canonical `PreCompact flush <cycle>/<step> <ISO-timestamp>` throughout; H1, PC1, PC2, PC4-bats, Inv1, EC-001, EC-003, Canonical Test Vectors, Architecture Anchors all updated; ADR cite v1.3→v1.4. |
| v1.3 | 2026-06-14 | product-owner | ADR cite v1.1→v1.3. |
| v1.2 | 2026-06-14 | product-owner | (F-P2-003) H1 enriched with corroboration detail; PC1 SHA corroboration: last-precompact-flush-sha → precompact-flush-log last line + git cat-file -t; stale-entry case added (git cat-file -t non-commit → treat as absent → prefix match sufficient); Inv1 updated symmetrically; EC-003 truth table: stale-SHA row added; Architecture Anchors updated. |
| v1.1 | 2026-06-14 | product-owner | (F-8) Exemption prefix updated to general `^PreCompact flush ` (locked convention); PC1+Inv1+Inv3 updated; EC-003 rewritten as HEAD/HEAD^ truth table; SHA corroboration against precompact-flush-log to prevent arbitrary-prefix bypass; TBD-DI replaced with DI-020+DI-025. |
| v1.0 | 2026-06-14 | product-owner | Initial creation (E-18 context-durability feature). |
