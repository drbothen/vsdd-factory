---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-04-30T00:00:00Z
phase: 2
inputs:
  - .factory/stories/S-8.00-perf-baseline-bc-anchor-verification.md
  - .factory/stories/epics/E-8-native-wasm-migration.md
  - .factory/stories/STORY-INDEX.md
input-hash: 68f3d16
traces_to: prd.md
pass: 3
previous_review: adv-s8.00-p2.md
target: S-8.00 v1.2
target_file: .factory/stories/S-8.00-perf-baseline-bc-anchor-verification.md
verdict: SUBSTANTIVE
clock: 0_of_3 → 0_of_3
findings_total: 6
findings_high: 0
findings_med: 3
findings_low: 1
findings_nit: 2
---

# Adversarial Review: S-8.00 v1.2 (Pass 3)

## Finding ID Convention

Finding IDs use the format: `ADV-<CYCLE>-P<PASS>-<SEV>-<SEQ>`

- `ADV`: Fixed prefix identifying adversarial findings
- `<CYCLE>`: Cycle prefix — `S8P3` for this S-8.00 pass-3 review
- `<PASS>`: Two-digit pass number (`P03`)
- `<SEV>`: Severity abbreviation (`MED`, `LOW`, `NIT`)
- `<SEQ>`: Three-digit sequence within the pass (e.g., `001`)

Examples used here: `F-P3-001` through `F-P3-006` (short form per pass-1/pass-2 convention).

## Executive Summary

Pass-3 fresh-context review of S-8.00 v1.2 (504 lines). All 8 pass-2 findings
verified closed. 6 new findings discovered: 0 HIGH, 3 MED, 1 LOW, 2 NIT.
Verdict: **SUBSTANTIVE** — fix burst required before pass-4. Clock held at
0_of_3 per ADR-013 (MED findings prevent advance).

Trajectory: 14 → 8 → 6 (25% decay from pass-2). Dominant issue class: AC prose
vs JSON schema / Task field drift — partial-fix regressions introduced by the v1.2
fix burst itself (F-P2-004 hyperfine lock and F-P2-007 schema expansion did not
propagate into the AC body text with full precision). This matches the
Fresh-Context Compounding Value pattern seen in E-8 epic precedent (P3→P4,
P5→P6, P7→P8 each surfaced 1+ MED from prior-burst partial-fix regressions).

---

## Part A — Fix Verification (Pass-2 Closure Audit)

All 8 pass-2 findings verified closed in v1.2:

| Finding | Description | Status | Evidence |
|---------|-------------|--------|----------|
| F-P2-001 | AC-3 line 229 double-prefix "E-8 epic E-8 AC-7b" | RESOLVED | Canonical "E-8 epic AC-7b" confirmed; grep residue=0 |
| F-P2-002 | Goal §2 double-prefix residue | RESOLVED | Swept and confirmed clean |
| F-P2-003 | OQ-6 anchor relationship clarification in AC-6 + EC-005 | RESOLVED | "OQ-6 is downstream S-8.09 gate, not S-8.00 deliverable" language present; HTML comment block before Architecture Mapping added |
| F-P2-004 | Measurement method locked to hyperfine (drop "or time builtin") | PARTIALLY_RESOLVED | Hyperfine appears in Library & Framework table + Task A.0 pre-flight; AC-1 body prose still uses vague measurement language. Residue surfaces as F-P3-003 below. |
| F-P2-005 | STORY-INDEX line 166 version v1.0→v1.2 | RESOLVED | State-manager scope; confirmed v1.2 + 504 lines |
| F-P2-006 | STORY-INDEX pass-1/pass-2 closure narrative | RESOLVED | State-manager scope; confirmed |
| F-P2-007 | bundle_size.measured_at duality note in JSON schema block | PARTIALLY_RESOLVED | Duality note present; AC-7 prose still uses legacy `bundle_size_bytes` flat-field form rather than `bundle_size` object. Residue surfaces as F-P3-002 below. |
| F-P2-008 | jq dependency justified via Task A.7 | RESOLVED | jq REQUIRED in Library table; Task A.7 schema validation step references jq |

**Summary:** 6 of 8 cleanly resolved; 2 carry partial-fix regressions into pass-3 as MED findings (F-P2-004 → F-P3-003; F-P2-007 → F-P3-002).

---

## Prior-Probe Results

Five prior-probe probes evaluated at pass-3:

| Probe | Description | Result |
|-------|-------------|--------|
| PRIOR-1 | A.0..A.7 sequencing coherence (A.0 pre-flight new in v1.2) | CLEAN — A.0 pre-flight install verification precedes A.1; sequencing logical; no ordering violations found |
| PRIOR-2 | EC-005 OQ-6 deferral wording precision | CLEAN — "OQ-6 is downstream S-8.09 gate, not S-8.00 deliverable" wording precise; EC-005 correctly defers |
| PRIOR-3 | Full double-prefix grep "E-8 epic E-8" | CLEAN — zero residue confirmed in v1.2 |
| PRIOR-4 | Library & Framework table completeness (hyperfine REQUIRED + jq REQUIRED) | CLEAN — both marked REQUIRED; install commands present |
| PRIOR-5 | behavioral_contracts=[] intentional disclosure | CLEAN — [process-gap] under D-2 Option C present; v1.1 BC candidates registered |

All 5 prior-probes clean. Residue class shifts to: AC prose not updated to match schema field names introduced by F-P2-007 expansion, and AC body not updated to match hyperfine invocation specifics locked by F-P2-004.

---

## Part B — New Findings (Pass-3 Fresh-Context Discovery)

### MEDIUM

#### F-P3-001: AC-7 Uses `du -sh` Instead of `du -sb` — Mismatch with Task A.6 + Schema `*_bytes` Integer Fields

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** AC-7 acceptance criteria body, measurement instruction for `legacy_bash_adapter_wasm_bytes` / `all_hook_plugins_wasm_bytes` / `dispatcher_binary_bytes`
- **Description:** AC-7 instructs the implementer to measure bundle sizes using `du -sh`, which produces human-readable output (e.g., `42K`, `1.1M`). The JSON schema in File Structure Requirements specifies fields named `*_bytes` with integer type. Task A.6 uses `du -sb` (byte-count flag). The human-readable flag `-h` is incompatible with the integer schema fields; `du -sb` is the correct command.
- **Evidence:**
  - AC-7 prose: `du -sh <path>` (human-readable)
  - Task A.6: `du -sb <path>` (byte count)
  - Schema: `legacy_bash_adapter_wasm_bytes`, `all_hook_plugins_wasm_bytes`, `dispatcher_binary_bytes` (integer fields expecting byte values)
  - Mismatch: `-sh` → KiB/MiB strings cannot populate integer byte fields without parsing
- **Proposed Fix:** Replace `du -sh` with `du -sb` in AC-7 body. Ensure all three size measurement references in AC-7 use `-sb`.

---

#### F-P3-002: AC-7 References Legacy `bundle_size_bytes` Flat Field — Schema Uses `bundle_size` Object

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** AC-7 acceptance criteria body, reference to the output schema field for bundle size
- **Description:** AC-7 acceptance criteria body uses the flat field name `bundle_size_bytes` as if it is a top-level key in the output JSON. The v1.2 JSON schema block (added per F-P2-007) defines `bundle_size` as a nested object containing `legacy_bash_adapter_wasm_bytes`, `all_hook_plugins_wasm_bytes`, and `dispatcher_binary_bytes` sub-fields. The AC-7 prose was not updated to reference the canonical nested structure; it still uses the pre-F-P2-007 flat-field name. This is a direct partial-fix regression from the v1.2 fix burst.
- **Evidence:**
  - AC-7 body: references `bundle_size_bytes` (flat field — pre-F-P2-007 legacy)
  - Schema block: `bundle_size: { legacy_bash_adapter_wasm_bytes: ..., all_hook_plugins_wasm_bytes: ..., dispatcher_binary_bytes: ... }` (nested object — added by F-P2-007)
  - Drift: AC references a field name that does not exist in the current schema
- **Proposed Fix:** Update AC-7 prose to reference `bundle_size.legacy_bash_adapter_wasm_bytes`, `bundle_size.all_hook_plugins_wasm_bytes`, and `bundle_size.dispatcher_binary_bytes` (or the `bundle_size` object collectively). Remove legacy `bundle_size_bytes` flat-field language. Cross-check AC-2/AC-3/AC-4/AC-5/AC-8 for any stale `bundle_size_bytes` flat-field residue.

---

#### F-P3-003: AC-1 Measurement-Method Prose Does Not Name `hyperfine` or Warmup/Runs Configuration

- **Severity:** MEDIUM
- **Category:** ambiguous-language
- **Location:** AC-1 acceptance criteria body, measurement method description
- **Description:** F-P2-004 locked the measurement method to hyperfine by adding hyperfine to the Library & Framework REQUIRED table and Task A.0 pre-flight. However, AC-1 itself — the primary acceptance criterion for the measurement protocol — still uses vague prose that does not name `hyperfine`, does not specify `--warmup 3 --runs 10`, and does not confirm that the reported metric is median wall-clock time expressed as integer milliseconds (consistent with `*_ms` schema fields). An implementer reading AC-1 in isolation cannot derive the required invocation. This is a partial-fix regression: F-P2-004 fixed the Library table and Task A.0 but did not propagate the same precision into AC-1 body.
- **Evidence:**
  - AC-1 body: does not name `hyperfine`; does not specify warmup count or run count; does not specify median vs mean or integer ms unit
  - Library table: hyperfine REQUIRED (added by F-P2-004 fix burst) — AC-1 does not reference it
  - Schema: `*_ms` fields imply integer milliseconds; AC-1 does not confirm this unit
  - Regression class: F-P2-004 fixed the structural/task layer but left the AC body unupdated
- **Proposed Fix:** Update AC-1 body to name the benchmarking tool explicitly: "measured using a bats harness wrapping `hyperfine --warmup 3 --runs 10` per hook with a representative fixture input; reports median wall-clock time as integer milliseconds (per `*_ms` schema fields)." This combined fix also closes F-P3-006 (integer-ms unit qualifier).

### LOW

#### F-P3-004: Task A.5 Does Not Name `validate-bc-title.sh` as the Tier-2 Verification Source

- **Severity:** LOW
- **Category:** ambiguous-language
- **Location:** Task A.5, BC-anchor verification sub-task
- **Description:** Task A.5 describes running BC-anchor verification for the 9 Tier 1 hooks but refers to the verification mechanism generically without naming the script. The E-8 epic and other cross-references identify `validate-bc-title.sh` as the Tier-2 per-plugin verification source for BC anchoring. Naming the script explicitly in Task A.5 removes ambiguity for the implementer.
- **Evidence:**
  - Task A.5 body: generic "verification script" reference (no filename)
  - E-8 epic AC-7b: `validate-bc-title.sh` named as Tier-2 source
  - Inconsistency: Task A.5 leaves the implementer to discover the script name independently
- **Proposed Fix:** Update Task A.5 to name `validate-bc-title.sh` as the Tier-2 per-plugin BC-anchor verification source.

### NIT

#### F-P3-005: Task A.0 Install Order Lists `cargo install` Before `apt-get` — Ubuntu-latest Convention Is `apt-get` First

- **Severity:** NIT
- **Category:** ambiguous-language
- **Location:** Task A.0 pre-flight install verification, tool installation sequence
- **Description:** Task A.0 lists the `cargo install hyperfine` command before the `apt-get install` path. On a standard `ubuntu-latest` GitHub Actions runner, `apt-get` is the preferred fast path (pre-packaged binary); `cargo install` is the fallback (compile from source, slow). Listing `cargo` first suggests it is the primary path.
- **Evidence:** Task A.0 current order: `cargo install hyperfine` → `apt-get install hyperfine` (fallback). CI convention: ubuntu-latest provides apt-get as recommended path.
- **Proposed Fix:** Reorder Task A.0 install options: `apt-get install hyperfine` (recommended on ubuntu-latest) first, `cargo install hyperfine` (fallback) second.

#### F-P3-006: AC-1 Missing Integer-Millisecond Unit Qualifier (Subsumed by F-P3-003 Fix)

- **Severity:** NIT
- **Category:** ambiguous-language
- **Location:** AC-1 acceptance criteria body, metric unit specification
- **Description:** AC-1 does not explicitly state that the reported measurement unit is integer milliseconds, even though the JSON schema fields use `*_ms` naming. This is a subset of the broader F-P3-003 issue. The fix for F-P3-003 (adding the full measurement method sentence) subsumes this finding. Recorded separately to ensure the unit qualifier is explicitly included in the fix.
- **Evidence:** See F-P3-003 evidence. `*_ms` schema field naming convention implies integer ms; AC-1 should make this explicit.
- **Proposed Fix:** Subsumed by F-P3-003 fix. Ensure the AC-1 prose update includes "integer milliseconds (per `*_ms` schema fields)."

---

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 0 |
| MEDIUM | 3 |
| LOW | 1 |
| NIT | 2 |

**Overall Assessment:** block — SUBSTANTIVE findings present; fix burst required before pass-4.
**Convergence:** findings remain — iterate. Clock held at 0_of_3 per ADR-013.
**Readiness:** requires revision — story-writer fix burst (6 findings), then state-manager seal.

---

## Cross-Document Consistency Audit

| Check | Documents | Result |
|-------|-----------|--------|
| bundle_size field naming consistent across AC-7 + schema block + Task A.6 | S-8.00 v1.2 internally | FAIL — AC-7 uses `bundle_size_bytes` flat field; schema uses `bundle_size` object; Task A.6 uses `du -sb`. After fix: all three align on `bundle_size.*_bytes` + `du -sb`. |
| hyperfine named consistently in AC-1 + Library table + Task A.0 | S-8.00 v1.2 internally | PARTIAL — Library table + A.0 name hyperfine; AC-1 body does not. Fix scope: AC-1 minimum. |
| OQ-6 deferral language consistent (AC-6 + EC-005) | S-8.00 v1.2 internally | PASS — both reference "OQ-6 downstream S-8.09 gate" consistently |
| double-prefix "E-8 epic E-8" | S-8.00 v1.2 internally | PASS — zero residue confirmed in PRIOR-3 |
| behavioral_contracts=[] [process-gap] disclosure | S-8.00 v1.2 vs STORY-INDEX | PASS — consistent |
| Task A.0..A.7 ordering (A.0 pre-flight first) | S-8.00 v1.2 internally | PASS — A.0 precedes A.1..A.7 |
| v1.1 BC candidates BC-7.00.001 + BC-7.00.002 registered | S-8.00 v1.2 vs STORY-INDEX BCs column | PASS — present in both |

---

## Policy Compliance Sweep

| Policy | Description | Status |
|--------|-------------|--------|
| POLICY 1 | Fresh-context lifecycle review | PASS — full document read; no anchoring to cached state |
| POLICY 2 | Adversary must not propose fixes that introduce new ambiguity | PASS — all suggested fixes name specific commands, field names, units |
| POLICY 3 | State-manager-runs-last (fix burst ordering) | PASS — story-writer fixes first; state-manager seals after |
| POLICY 4 | No finding may be closed without Evidence field | PASS — all findings include evidence |
| POLICY 5 | HIGH findings must include root-cause analysis | N/A — 0 HIGH findings |
| POLICY 6 | Precision: implementer must be able to act on the finding without research | PASS — all findings name specific locations and specific fixes |
| POLICY 7 | Archaeology: probe for regressed old findings | PASS — all 8 pass-2 findings re-probed in Part A |
| POLICY 8 | No finding reported without location + issue + evidence | PASS — all 6 findings include all three |
| POLICY 9 | Same-burst VP/STORY-INDEX updates | N/A — no new VPs triggered |
| POLICY 10 | Convergence clock per ADR-013 | PASS — MED findings at pass-3 hold clock at 0_of_3; no premature advance |
| POLICY 11 | No test tautology in fix suggestions | N/A — no test code suggestions |
| POLICY 12 | BC TV emitter consistency | N/A — behavioral_contracts=[] intentional |

---

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 3 |
| **New findings** | 6 |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 6 / (6 + 0) = 1.0 |
| **Median severity** | MEDIUM (3 MED + 1 LOW + 2 NIT) |
| **Trajectory** | 14→8→6 |
| **Verdict** | FINDINGS_REMAIN — SUBSTANTIVE; fix burst required |

**Novelty note:** All 6 findings are net-new to pass-3 (not duplicates of pass-1 or pass-2 findings). However, 3 MED findings are partial-fix regressions — they were introduced by the v1.2 fix burst itself rather than being pre-existing latent issues. This is the canonical Fresh-Context Compounding Value pattern: each fix burst that expands schema/task precision without simultaneously updating the AC bodies creates new drift that only fresh-context discovery catches. Pattern matches E-8 epic precedent (P3→P4, P5→P6, P7→P8 each surfaced 1+ MED from prior-burst regressions).

---

## Priority Fix Order

**Story-writer scope (required before pass-4):**

1. **F-P3-003 + F-P3-006 [MED+NIT combined]** — AC-1 measurement-method prose. Add full sentence naming hyperfine + warmup 3 + runs 10 + median wall-clock integer ms.
2. **F-P3-002 [MED]** — AC-7 `bundle_size_bytes` → `bundle_size` object reference. Cross-check AC-2/AC-3/AC-4/AC-5/AC-8 for stale `bundle_size_bytes` residue.
3. **F-P3-001 [MED]** — AC-7 `du -sh` → `du -sb`. Verify all `du -s` uses in the document use `-sb`.
4. **F-P3-004 [LOW]** — Task A.5 name `validate-bc-title.sh` explicitly as Tier-2 per-plugin source.
5. **F-P3-005 [NIT]** — Task A.0 reorder: apt-get first, cargo fallback second.

**Required regression sweep after story-writer fix burst:**
- `grep -n "bundle_size_bytes" S-8.00-*.md` — expect zero results
- `grep -n "du -s[^b]" S-8.00-*.md` — expect zero results (all `du -s` uses must be `-sb`)
- `grep -n "hyperfine" S-8.00-*.md` — confirm AC-1 + A.0 + Library table all name hyperfine
- File Structure Requirements `E-8-bash-baseline.json` schema reference — confirm current

**State-manager scope (after story-writer burst):**
- Persist adv-s8.00-p3.md (this document)
- Update STORY-INDEX line 166 narrative: v1.2→v1.3, 504→510 lines, pass-3 closure, D-167 cite
- Update STATE.md Current Phase Steps + Decisions Log D-167

---

## Verdict

**SUBSTANTIVE** — 3 MED findings present. Fix burst required before pass-4.

**Clock: 0_of_3 → 0_of_3** (held; MED findings block advance per ADR-013).

**Trajectory:** 14 → 8 → 6 (25% decay pass-2→pass-3; 57% total decay from pass-1 baseline).

**Pass-4 priors for next adversary:**
- Library table apt-get ordering (low-probability NIT after F-P3-005 fix; confirm clean)
- Forward-reference release bundle directory path (check cross-story coherence with S-8.01..S-8.09 structure once those stories exist)
- Full document re-read with no anchoring to this pass's findings (per POLICY 1)

**Expected at pass-4:** NITPICK_ONLY (residue dropping to LOW/NIT polish level only) → clock advance 0_of_3 → 1_of_3 per ADR-013, assuming all 6 findings resolved cleanly and no new regressions introduced by the fix burst.
