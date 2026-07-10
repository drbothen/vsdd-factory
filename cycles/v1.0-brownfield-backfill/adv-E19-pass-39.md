# Adversarial Review — E-19 Pass 39 (no-delta confirming pass; perimeter = epic v1.22 + full E-19 suite at D-792 versions; streak 1/3)

**Perimeter:** epic v1.22 + S-19.01 v1.16 / S-19.02 v1.17 / S-19.03 v1.16 / S-19.04 v1.11 / S-19.05 v1.14 / S-19.06 v1.18 / S-19.07 v1.16 + STORY-INDEX v4.169 + VP-INDEX v2.55 + BC-4.13.001 v1.14 + BC-5.42.001 v1.5 + BC-2.07.001 v1.4 + BC-2.02.011 v1.5 + BC-3.08.001 v1.19 + BC-1.17.001 v1.5 + VP-095 v1.1 + VP-096 v1.1 + ADR-025 v1.12 + ADR-030 v1.3 + BC-INDEX v3.88 + ARCH-INDEX v2.97 + policies.yaml v1.4.2

**Reviewer:** fresh-context adversary; Iron Law; rubric policies.yaml v1.4.2

**Date:** 2026-07-09

**Verdict:** NOT-CLEAN — B0/H0/M1/L0 (1 finding, 0 observations)

**Streak:** 1/3 → 0/3 (reset)

**Model family:** Claude Opus 4.7

---

## Part A — Version Attestations + Input-Hash Attestations + Finding

### A.1 — Version + Input-Hash Perimeter Attestation

All perimeter artifact versions attested at D-792 levels (no delta from pass-38; governance-only burst):

| Artifact | Version | Input-hash / Notes |
|----------|---------|-------------------|
| epic (E-19) | v1.22 | a18ea87 — PASS ✓ |
| S-19.01 | v1.16 | d40bd21 — PASS ✓ |
| S-19.02 | v1.17 | 604f45d — PASS ✓ |
| S-19.03 | v1.16 | 8d1225d — PASS ✓ |
| S-19.04 | v1.11 | 67eee80 — PASS ✓ |
| S-19.05 | v1.14 | 9e54d68 — PASS ✓ |
| S-19.06 | v1.18 | 998ac74 — PASS ✓ |
| S-19.07 | v1.16 | 534c85c — PASS ✓ |
| STORY-INDEX | v4.169 | — PASS ✓ |
| VP-INDEX | v2.55 | — PASS ✓ |
| BC-4.13.001 | v1.14 | 58518e8 — PASS ✓ |
| BC-5.42.001 | v1.5 | 4fd18a4 — PASS ✓ |
| BC-2.07.001 | v1.4 | 9d60fc5 — PASS ✓ |
| BC-2.02.011 | v1.5 | — PASS ✓ |
| BC-3.08.001 | v1.19 | — PASS ✓ |
| BC-1.17.001 | v1.5 | 03fa998 — PASS ✓ |
| VP-095.md | v1.1 | ce25941 — PASS ✓ |
| VP-096.md | v1.1 | — PASS ✓ |
| ADR-025 | v1.12 | — PASS ✓ |
| ADR-030 | v1.3 | — PASS ✓ |
| BC-INDEX | v3.88 | — PASS ✓ |
| ARCH-INDEX | v2.97 | — PASS ✓ |
| policies.yaml | v1.4.2 | — PASS ✓ |

All 20+ perimeter version pins attested ✓.

### A.2 — BC-INDEX H1 Verbatim-Parity Table (POLICY 7; byte-exact comparison; 6 E-19 BCs)

POLICY 7 mandates that BC-INDEX Title column entries be verbatim copies of the H1 text after the `# BC-N.NN.NNN: ` prefix. Fresh-context adversary performed H1 extraction and byte-level comparison against BC-INDEX v3.88 catalog rows for all 6 E-19 BCs.

| BC ID | BC File H1 (after prefix) | BC-INDEX v3.88 Title Cell | Match? |
|-------|--------------------------|--------------------------|--------|
| BC-1.17.001 | `host::read_prefix — bounded partial read (head-c semantics), NEVER OUTPUT_TOO_LARGE, additive FFI entry point, path_allow + rejoin capability model` | `host::read_prefix: bounded partial read (head-c semantics), NEVER OUTPUT_TOO_LARGE, additive FFI entry point with path_allow + rejoin capability model` | **DRIFT ✗** (2 drifts: ` — `→`: `; `, path_allow`→` with path_allow`) |
| BC-2.07.001 | `host::read_file absent-file semantics — codes::NOT_FOUND (-5) additive error code, HostError::NotFound SDK variant, rejoin path-allowed resolution, zero false-positive capability_denied` | `host::read_file absent-file semantics: codes::NOT_FOUND (-5) additive error code, HostError::NotFound SDK variant, rejoin path-allowed resolution, zero false-positive capability_denied` | **DRIFT ✗** (1 drift: `semantics — `→`semantics: `) |
| BC-5.42.001 | `pr-manager READY-verdict + merge-strategy enforcement — covered_sha pin, stale-verdict detection, and release-PR squash prevention` | `pr-manager READY-verdict + merge-strategy enforcement: covered_sha pin, stale-verdict detection, and release-PR squash prevention` | **DRIFT ✗** (1 drift: `enforcement — `→`enforcement: `) |
| BC-4.13.001 | `verify-factory-lock WASM PreToolUse guard MUST block mutating tools when a foreign unexpired factory_lock is held, MUST pass all read-only tools unconditionally, MUST fail-open on crash, MUST be registered async=false with both capability blocks enumerated, and MUST treat expired/absent/malformed locks as unlocked` | (same) | **VERBATIM ✓** |
| BC-3.08.001 | `dispatcher async-semantics event types are catalogued and emitted via FileSink — \`plugin.async_block_discarded\`, \`dispatcher.schema_mismatch\`, \`dispatcher.registry_invalid\`, \`plugin.timeout\` (async path), \`plugin.abandoned\`, \`plugin.completed\` (async path)` | (same — em-dash preserved verbatim) | **VERBATIM ✓** |
| BC-2.02.011 | `host::write_file: bounded write capability with allowlist enforcement` | (same) | **VERBATIM ✓** |

**Control check:** BC-3.08.001 index row preserves a mid-title em-dash verbatim, proving no index-wide colon-normalization convention exists. The three drifts on BC-1.17.001/BC-2.07.001/BC-5.42.001 are genuine title-cell copy errors, not a format convention.

**Summary:** 3 of 6 E-19 BCs show POLICY 7 title-cell drift (verbatim ✗). 3 of 6 VERBATIM ✓.

### A.3 — F-P39-001: MEDIUM — POLICY 7 BC-H1-is-title-SoT; BC-INDEX v3.88 title-cell drift on 3 of 6 E-19 BCs

**Finding ID:** F-P39-001
**Severity:** MEDIUM
**Policy:** POLICY 7 (BC-INDEX Title column must be VERBATIM copy of BC file H1 title text after prefix; enforcement target (i): BC-INDEX Title column ↔ BC file H1)
**Perimeter artifact:** BC-INDEX v3.88

**Evidence (byte-exact):**

BC-1.17.001 — 2 drifts:
- Drift 1: separator after `host::read_prefix` is colon-space (`: `) in index vs em-dash-space (` — `) in H1
- Drift 2: `additive FFI entry point with path_allow` in index vs `additive FFI entry point, path_allow` in H1 (word `with` inserted, comma replaced)

BC-2.07.001 — 1 drift:
- `absent-file semantics: codes::NOT_FOUND` in index vs `absent-file semantics — codes::NOT_FOUND` in H1 (colon-space vs em-dash-space after `semantics`)

BC-5.42.001 — 1 drift:
- `merge-strategy enforcement: covered_sha` in index vs `merge-strategy enforcement — covered_sha` in H1 (colon-space vs em-dash-space after `enforcement`)

**Severity rationale:** MEDIUM (POLICY 7 enforcement target (i); 3 BCs affected; character-level byte mismatch between authoritative source (BC file H1) and index; adversary used visual comparison, not literal-shell character-diff — process-gap class flagged separately in root-cause note below).

**POLICY 7 scope adjudication (orchestrator):** POLICY 7's editorial-abbreviation exception (C-P3-001) is scoped EXCLUSIVELY to story-body Behavioral Contracts table title cells. BC-INDEX Title column ↔ H1 requires VERBATIM parity (enforcement target (i)). Any prior pass acceptance of "index colon convention" is superseded — BC-INDEX Title column must be byte-exact to H1. The em-dash present in BC-3.08.001 (a mid-title em-dash preserved verbatim since that row's authoring at D-756) confirms no index-wide colon-normalization convention exists; the 3 drifted titles are copy errors.

**Routing:** state-manager (BC-INDEX rows are state-manager domain; no BC file content changes required).

**Proposed fix:** Update BC-INDEX v3.88→v3.89; correct 3 title cells to verbatim H1 form:
- BC-1.17.001: `: ` → ` — ` AND `with path_allow` → `, path_allow`
- BC-2.07.001: `semantics: ` → `semantics — `
- BC-5.42.001: `enforcement: ` → `enforcement — `

**CLOSED:** Fixed in BC-INDEX v3.89 by this D-794 fix burst (state-manager). See post-edit gate evidence in burst-log Block 5.

**Root-cause note for lesson:** The prior CLEAN pass (pass-38) attested POLICY 7 as "PASS" narratively without performing a literal-shell character-level diff between H1 and index title cells. POLICY 5 v1.3.4/v1.3.5-class pseudocode-attestation miss at POLICY 7 verification axis. Lesson: verbatim-parity claims in adversary reports and index-sync attestations REQUIRE literal-shell character-diff evidence (e.g., extract H1 substring, `fgrep` it against the index row, or `diff` the two strings), not visual comparison. Tag: [process-gap][codified D-794].

---

## Part B — 14-Cell Category-(i) Sweep + Policy Attestations

### B.1 — Category-(i) 14-Cell Aggregation Sweep (POLICY 5 v1.3.7; 14/14 CLEAN)

All 14 category-(i) aggregation cells at D-792 versions verified CLEAN per policies.yaml v1.4.2 POLICY 5 v1.3.7. S-19.06 Token Budget Total now reads `~22,000` (corrected at D-792). Wave-summary Input-hashes parity verified. No new aggregation-cell drift found.

| Cell | Artifact | Value | CLEAN? |
|------|----------|-------|--------|
| 1 | S-19.01 Token Budget Total | ~5,000 | ✓ |
| 2 | S-19.02 Token Budget Total | ~22,000 | ✓ |
| 3 | S-19.03 Token Budget Total | ~8,500 | ✓ |
| 4 | S-19.04 Token Budget Total | ~5,000 | ✓ |
| 5 | S-19.05 Token Budget Total | ~5,500 | ✓ |
| 6 | S-19.06 Token Budget Total | ~22,000 | ✓ (D-792 fix; awk row-sum 22,000) |
| 7 | S-19.07 Token Budget Total | ~6,000 | ✓ |
| 8 | STORY-INDEX wave-summary Input-hashes | S-19.01=d40bd21/S-19.02=604f45d/S-19.03=8d1225d/S-19.04=67eee80/S-19.05=9e54d68/S-19.06=998ac74/S-19.07=534c85c | ✓ (D-791 fix; all 7 match frontmatter) |
| 9 | STORY-INDEX story-count | 143 total (128 file-resident + 15 stub IDs) | ✓ |
| 10 | BC-INDEX total_bcs | 1,977 | ✓ |
| 11 | VP-INDEX total VPs | 102 | ✓ |
| 12 | STORY-INDEX merged_count | 98 | ✓ |
| 13 | ARCH-INDEX subsystem count | 10 | ✓ |
| 14 | BC-INDEX SS-05 BC count | 656 | ✓ |

14/14 CLEAN.

### B.2 — POLICY 8 BC Propagation + EC-Mirror Checks (6 E-19 BCs)

POLICY 8 bidirectional checks for all 6 E-19 BCs: BC frontmatter `bcs:` array ↔ body BC table ↔ Token Budget propagation. All 6 checked at D-792 versions. No drift found. EC-mirror parity (EAC table ↔ BC AC coverage) verified for 4 BCs with EAC entries. ✓

### B.3 — POLICY 9 VP Semantic-Fit Table (8 E-19 VPs)

| VP | Assigned BC | Semantic fit | PASS? |
|----|------------|-------------|-------|
| VP-095 | BC-4.13.001 Precondition 3 | stable §Precondition 3 anchor (D-784 v1.1); exclusive boundary correct | ✓ |
| VP-096 | BC-4.13.001 Invariant 9 | exclusive `0..delimiter_start_offset` boundary form (D-782 v1.1); VP-INDEX classification matches BC | ✓ |
| VP-097 | BC-4.13.001 AC-001 | proptest coverage of 0..delimiter_start_offset exclusive boundary | ✓ |
| VP-098 | BC-4.13.001 AC-002 | integration test against real host::read_prefix path | ✓ |
| VP-099 | BC-4.13.001 AC-003 | property-based test for NEVER OUTPUT_TOO_LARGE invariant | ✓ |
| VP-100 | BC-4.13.001 AC-004 | capabilities::path_allow resolution fuzz | ✓ |
| VP-101 | BC-1.17.001 | integration-only proof method (D-782 aligned; VP-INDEX classification matches BC §VP Anchors) | ✓ |
| VP-102 | BC-2.07.001 | absent-file round-trip integration | ✓ |

All 8 VP semantic-fit checks PASS ✓.

### B.4 — Phase-A / Phase-B Coherence (4-way consistency)

Four-way consistency check across BC-4.13.001 v1.14, ADR-025 v1.12, S-19.02 v1.17, S-19.07 v1.16 for Phase-A / Phase-B migration:

- BC-4.13.001 v1.14 §Precondition 3: dual-anchor S-19.02 (Phase-A: host::read_file max_bytes=262144) + S-19.07 (Phase-B: host::read_prefix migration) ✓
- BC-4.13.001 v1.14 §Traceability: cites `and Deliverable D18` (D-790 fix; ADR-025 has Deliverable D18 at line 1210, not §Decision 18) ✓
- ADR-025 v1.12 §Decision 15: Phase-B migration to host::read_prefix documented; STATE_MD_MAX_BYTES removal noted; max_bytes=8192 Phase-B bound stated (D-786 fix) ✓
- S-19.02 v1.17: cites BC-4.13.001 v1.14 ×18 sites (D-790 sweep); crate path `crates/factory-lock-parse/` (D-783 ruling) ✓

Four-way coherence: PASS ✓.

Three-way consistency check across BC-5.42.001 v1.5, ADR-030 v1.3, S-19.01 v1.16:

- BC-5.42.001 v1.5: WASM path `hook-plugins/pr-manager-completion-guard.wasm` (D-776 v1.3 fix) ✓
- ADR-030 v1.3: SubagentStop canonical TOML stanza: `on_error="continue"`, `priority=920`, no `tool` field (D-777 fix) ✓
- S-19.01 v1.16: `bin/pr-manager-completion-guard.wasm` path (D-775 fix) + canonical invocation signature ✓

Three-way coherence: PASS ✓.

### B.5 — DAG / Wave Partition Check

E-19 DAG wave partition (W1=S-19.01/02/03; W2=S-19.04/05/06; W3=S-19.07) verified acyclic and bidirectional. STORY-INDEX v4.169 wave-schedule section confirms W1→W2→W3 dependency order. S-19.07 physical prerequisite S-19.06 and logical prerequisite S-19.02 both satisfied within W-chain. ✓

### B.6 — EAC ↔ AC Coverage Map (epic v1.22)

Epic v1.22 EAC table (EAC-001..EAC-008) verified against story AC coverage for all 7 E-19 stories. EAC-003 §Negative control B (BC-2.07.001 v1.4 cite — D-787 fix) correct. EAC-008 column split (D-768 fix) correct. All EAC entries trace to at least one AC in a story. ✓

### B.7 — POLICY 19 Sentinel Check (stable-anchor form)

All §Traceability ADR Reference cells in the 6 E-19 BCs verified in stable §Decision N form (no volatile version pins):
- BC-4.13.001 v1.14: `ADR-025 §Decisions 1/14/15 and Deliverable D18` — stable ✓
- BC-1.17.001 v1.5: ADR-025 §Decision 13 — stable ✓
- BC-2.07.001 v1.4: ADR-025 §Decision 13 — stable ✓
- BC-5.42.001 v1.5: ADR-030 §Decision 1 — stable ✓
- BC-3.08.001 v1.19: ADR-013 §Decision 1 — stable ✓
- BC-2.02.011 v1.5: ADR-025 §Decision 13 — stable ✓

POLICY 19 sentinels: all PASS ✓.

### B.8 — Trajectory Note

Trajectory passes 22–39: 4→3→4→2→2→4→6→5→4→1→3→4→1→2→1→1→0→1.

Pass-38 narrative verbatim-attestation root-cause: POLICY 7 attestation at pass-38 was performed via visual inspection rather than literal-shell character-diff. The em-dash (` — `) vs colon-space (`: `) separator distinction is visually subtle in Markdown rendering. Fresh-context adversary at pass-39 performed byte-level character extraction to detect the 4 individual byte sequences. This illustrates the compounding value of fresh-context analysis: pass-38 CLEAN was genuine (0 findings) on all other axes, but this process-gap in verbatim-parity attestation escaped visual inspection. Lesson: POLICY 7 verbatim-parity claims require literal-shell character-diff evidence.

Zero BLOCKER since pass-22 (17 consecutive). Zero HIGH since pass-35 (4 consecutive). This is a MEDIUM-only regression from pass-38 CLEAN — streak reset 1/3 → 0/3; pass-40 NEXT.
