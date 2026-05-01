---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-05-01T00:00:00Z
phase: 2
inputs:
  - .factory/stories/S-8.10-sdk-extension-write-file.md
  - .factory/cycles/v1.0-brownfield-backfill/adv-s8.10-p2.md
  - .factory/specs/architecture/ARCH-INDEX.md
  - .factory/specs/behavioral-contracts/BC-INDEX.md
  - crates/hook-sdk/src/host.rs
  - crates/factory-dispatcher/src/host/read_file.rs
  - crates/factory-dispatcher/src/host/mod.rs
input-hash: "e441e99"
traces_to: prd.md
pass: p3
story_id: "S-8.10"
story_version: "1.1"
story_input_hash: "e441e99"
pass_number: 3
previous_review: adv-s8.10-p2.md
target: story
target_file: .factory/stories/S-8.10-sdk-extension-write-file.md
verdict: NITPICK_ONLY
clock: 2_of_3
findings_critical: 0
findings_high: 0
findings_medium: 0
findings_low: 3
findings_nit: 1
---

# Adversarial Review Pass-3 — S-8.10 v1.1

## Finding ID Convention

Finding IDs use the format: `ADV-S810-P03-<SEV>-<SEQ>`
- `ADV`: Fixed prefix
- `S810`: Story identifier
- `P03`: Two-digit pass number
- `<SEV>`: Severity abbreviation (`HIGH`, `MED`, `LOW`, `NIT`)
- `<SEQ>`: Three-digit sequence

## Part A — Pass-2 Fix Verification

**Pre-condition observed:** Story is still at v1.1 with input-hash `e441e99` (unchanged since pass-2). Change Log contains only the 1.0 → 1.1 entry from the pass-1 fix burst. **No pass-2 fix burst was applied** — the seven pass-2 findings carry forward unmodified, but per S-7.03 SKIP-FIX policy LOW/NIT findings may persist. The two pass-2 MEDs require empirical re-evaluation under fresh-context.

### Pass-1 HIGH closures — re-verified (still hold)

| Pass-1 Finding | Re-verification (file:line) | Status |
|---|---|---|
| HIGH-001 input paths cycles/ prefix | Frontmatter inputs lines 12-15: `.factory/cycles/v1.0-brownfield-backfill/adv-s8.04-p2.md` and `adv-s8.09-p2.md` correct | HOLDS |
| HIGH-002 max_bytes in signature | AC-1 line 144: `write_file(path, contents, max_bytes, timeout_ms) -> Result<(), HostError>` — 4 params confirmed | HOLDS |
| HIGH-003 FFI input-pointer protocol | AC-1 lines 152-158 + Rule 3 line 290 + AC-8 line 236 — input-pointer protocol pinned with `read_wasm_bytes` reference | HOLDS |
| HIGH-004 AC-5 conditional removed | AC-5(d) lines 201-203: "No conditional language — the `max_bytes` cap is mandatory" | HOLDS |
| HIGH-005 BC family BC-2.02.011 | OQ-1 lines 113-121 + BC table line 139 — BC-2.02.011 cited; BC-INDEX:152-161 confirms BC-2.02.x is host-shim ABI family | HOLDS |

All five pass-1 HIGH closures remain intact. No regressions.

### Pass-2 finding adjudication

| Pass-2 ID | Severity (P2) | Status (P3) | Adjudication |
|---|---|---|---|
| MED-001 resolve_for_write helper | MEDIUM | DEMOTE → LOW (P3-LOW-001) | Empirical re-check: `read_file.rs:101-107` `resolve_for_read` is trivial absolute-or-`plugin_root.join` — NO symlink-following semantics exist in the read_file sibling. Pass-2's "documented symlink-following" claim was hallucinated. The asymmetry concern reduces to "should write_file path resolution be specified at all" which is LOW since the read_file precedent is also unspecified. |
| MED-002 EC-006 -99 INTERNAL_ERROR conflict | MEDIUM | CLOSED (no defect) | Empirical re-check: `read_file.rs:92` maps `ReadErr::Other` (the catch-all I/O failure) to `codes::INTERNAL_ERROR (-99)`. EC-006 mapping "no parent dir → -99" is **consistent** with the read_file Rule-4 sibling pattern, not divergent. Pass-2 finding mis-diagnosed. |
| LOW-001 AC-5(b) CLAUDE_PROJECT_DIR rooting | LOW | OPEN — carry (P3-LOW-002) | AC-5(b) line 200 unchanged. Per S-7.03 SKIP-FIX may carry; re-tagged. |
| LOW-002 AC-4(b) "without panic" tautology | LOW | OPEN — carry (P3-LOW-003) | AC-4(b) line 191 unchanged. POLICY 11 still applies. |
| LOW-003 BC-2.02.002 scope | LOW | DROP | Re-read shows the story's BC trace table (line 139) lists only the pending BC-2.02.011 — BC-2.02.002 is *cited* in AC-1 and Rule 4 as a pre-existing invariant, never claimed as governing write_file. No drift. Pass-2 finding spurious. |
| LOW-004 depends_on/blocks completeness | LOW | DROP | Pass-2 conjectured S-8.07/S-8.08 sibling coordination; no evidence in either story would require S-8.10 to *block* them. ABI version stays at 1, so additive export is not a blocker for sibling SDK stories. Pass-2 finding speculative. |
| NIT-001 bcs frontmatter empty | NIT | OPEN — carry (P3-NIT-001) | Frontmatter line 21 still `behavioral_contracts: []` with "pending PO author" comment that does not cite S-7.01 by name. |

Net pass-2 adjudication: **3 carry as LOW/NIT (S-7.03 deferred); 2 dropped (mis-diagnosed); 2 closed (empirical re-check; no defect)**.

## Part B — New Findings (Pass-3)

### LOW

#### ADV-S810-P03-LOW-001: write_file path resolution helper unspecified (demoted from pass-2 MED-001)

- **Severity:** LOW
- **Confidence:** MEDIUM
- **Category:** spec-fidelity
- **Location:** S-8.10 AC-1 (lines 144-159), AC-2 (lines 161-176), Architecture Mapping (lines 247-267)
- **Description:** The story does not specify the path-resolution helper for write_file. The read_file sibling at `crates/factory-dispatcher/src/host/read_file.rs:101-107` uses a trivial `resolve_for_read`. The story's silence is consistent with the sibling's silence in spec, but a future maintainer reading only this story would have no documented contract for whether write_file plugin-roots relative paths the same way.
- **Proposed Fix:** Add one sentence to AC-2: "Path resolution mirrors `resolve_for_read` semantics in `read_file.rs:101` — absolute paths pass through; relative paths are joined with `ctx.plugin_root`."

#### ADV-S810-P03-LOW-002: AC-5(b) test helper does not anchor CLAUDE_PROJECT_DIR rooting (carried from p2 LOW-001)

- **Severity:** LOW
- **Confidence:** MEDIUM
- **Category:** spec-fidelity
- **Location:** S-8.10 AC-5(b) line 200
- **Description:** AC-5(b) describes "Writes allowed file to a tmp dir when path is within `path_allow`" without naming the rooting boundary (CLAUDE_PROJECT_DIR or `ctx.plugin_root`).
- **Proposed Fix:** Replace with "Writes allowed file to a tmp dir set as `ctx.plugin_root`, when path is within `path_allow` (parallel to read_file test at `read_file.rs:163-172`)."

#### ADV-S810-P03-LOW-003: AC-4(b) "without panic" tautology persists (carried from p2 LOW-002)

- **Severity:** LOW
- **Confidence:** HIGH
- **Category:** spec-fidelity (POLICY 11)
- **Location:** S-8.10 AC-4(b) line 191
- **Description:** AC-4(b) reads: "`max_bytes` and `timeout_ms` parameters are accepted without panic (stub behavior)." "Without panic" is a tautology under POLICY 11 (no_test_tautologies); production Rust code is expected not to panic.
- **Proposed Fix:** Replace AC-4(b) with: "calling the wrapper with non-trivial `max_bytes` and `timeout_ms` values (e.g., 4096, 1000) still routes through the stub and yields `Err(HostError::CapabilityDenied)` — the parameters do not short-circuit the FFI call."

### NIT

#### ADV-S810-P03-NIT-001: bcs frontmatter "pending PO author" comment does not cite S-7.01 (carried from p2 NIT-001)

- **Severity:** NIT
- **Confidence:** HIGH
- **Category:** spec-fidelity
- **Location:** S-8.10 frontmatter lines 17-21
- **Description:** The comment block above `behavioral_contracts: []` mentions "pending PO authorship" and "behavioral_contracts MUST be non-empty before status transitions to ready" but does not name the governing rule (Spec-First Gate S-7.01).
- **Proposed Fix:** Add a single line to the frontmatter comment block: `# Gate: S-7.01 Spec-First — bcs MUST be populated before status: ready.`

## Open Questions

| ID | Question | Owner | Status |
|----|----------|-------|--------|
| OQ-A1 | PO must author BC-2.02.011 before story status flips draft → ready | PO | OPEN (carried p1, p2) |
| OQ-A2 | Should AC-2 explicitly cite `resolve_for_write` symmetry to `resolve_for_read`? | story-writer | OPEN (P3-LOW-001) |

OQ-A2 (BC-2.02.002 widening) and OQ-A3 (sibling story coordination) from pass-2 are both **dropped** as adjudicated above (LOW-003 and LOW-004 spurious).

## Pass-4 Priors

- **Story will likely converge:** Pass-3 produced 0 HIGH, 0 MED, 3 LOW, 1 NIT. All LOWs are S-7.03 SKIP-FIX eligible. Per ADR-013 a NITPICK_ONLY pass advances clock 1_of_3 → 2_of_3.
- **OQ-A1 (BC-2.02.011 authorship)** remains the gating blocker for status flip — independent of adversarial convergence.
- **No new MED/HIGH expected in pass-4** unless story is re-edited with new content. Recommend pass-4 verify NITPICK_ONLY hold and advance to 3_of_3 → CONVERGED.

## Verdict

**NITPICK_ONLY** — 0 CRITICAL, 0 HIGH, 0 MED, 3 LOW, 1 NIT. All findings are S-7.03 SKIP-FIX eligible. Two of the three LOWs are pass-2 carry-overs (deferred under SKIP-FIX); one is a demotion of a pass-2 MED after empirical re-check. No content defects requiring fix before clock advance.

**Clock state:** 1_of_3 → **2_of_3** (NITPICK_ONLY advance).

## Trajectory

| Pass | H | M | L | NIT | Total |
|------|---|---|---|-----|-------|
| p1 | 5 | 7 | 5 | 1 | 18 |
| p2 | 0 | 2 | 4 | 1 | 7 |
| p3 | 0 | 0 | 3 | 1 | 4 |

18 → 7 → 4. Continued decay (43% from pass-2, down from 61% pass-1→pass-2). Two pass-2 MEDs adjudicated as mis-diagnosed under empirical re-check; net "real" defect count is 4.

## Novelty Assessment

| Field | Value |
|-------|-------|
| Pass | 3 |
| New findings | 1 (P3-LOW-001 demotion of P2-MED-001) |
| Carried findings | 3 (P3-LOW-002, P3-LOW-003, P3-NIT-001 — S-7.03 SKIP-FIX) |
| Pass-2 dropped/closed | 4 (P2-MED-002, P2-LOW-003, P2-LOW-004 spurious; P2-MED-001 demoted) |
| Novelty score | 0.25 (1 substantive new finding / 4 total) |
| Median severity | LOW |
| Trajectory | 18 → 7 → 4 |
| Verdict | LOW novelty — refinements only; spec is converged |

**Novelty: LOW** — pass-3 produced one demotion of a pass-2 finding (not a new gap) and confirmed two carry-over LOWs and one carry-over NIT. Two pass-2 findings dropped under empirical re-check.

## Summary

| Severity | Count |
|----------|-------|
| CRITICAL | 0 |
| HIGH | 0 |
| MEDIUM | 0 |
| LOW | 3 |
| NIT | 1 |

**Overall Assessment:** nitpick-only — story has converged on substance.

**Convergence:** 2_of_3 NITPICK_ONLY passes accumulated under ADR-013. One more clean pass to declare CONVERGED.

**Readiness:** Story is content-complete pending OQ-A1 (BC-2.02.011 authorship by PO). All adversarial findings are SKIP-FIX eligible per S-7.03. Story may proceed to wave-15 implementation queue once BC-2.02.011 is authored and `behavioral_contracts` frontmatter is populated.

---

**Process notes for orchestrator:**
- BC-2.02.011 PO authorship (OQ-A1) is the only remaining hard blocker for status: draft → ready. This is independent of adversarial convergence — it is a Spec-First Gate (S-7.01) compliance requirement.
- Pass-2 produced two MED findings that pass-3 empirical re-check determined to be mis-diagnosed. Future adversarial passes on SDK-surface stories should require empirical sibling-file inspection before tagging asymmetry-based findings as MEDIUM+.
- [process-gap] Adversarial passes that file MEDIUM findings on "asymmetry with sibling file" should be required to cite the sibling file:line evidence in the finding body.
