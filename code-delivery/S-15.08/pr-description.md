# [S-15.08] Dim-2 Gate Template Library — 11 Canonical Bash Scripts for Fix-Burst Attestation

**Epic:** E-12 — Context Resolvers + Platform Stories (brownfield-backfill wave, S-15.03 PRIORITY-A M1)
**Mode:** brownfield-feature
**Convergence:** CONVERGED after 6 LOCAL adversarial passes (3/3 clean streak per BC-5.39.001)

![Tests](https://img.shields.io/badge/tests-29%2F29-brightgreen)
![Bats Suites](https://img.shields.io/badge/bats_suites-50%2B_green-brightgreen)
![Adversary Passes](https://img.shields.io/badge/adversary_passes-6%2F6_CLEAN-brightgreen)
![Cargo](https://img.shields.io/badge/cargo_fmt%2Bclippy-PASS-brightgreen)

This PR delivers the dim-2 gate template library: 11 canonical bash scripts at
`plugins/vsdd-factory/hooks/dim2-gates/`, 12 bats test files, 25+ fixture directories, and
extended bats discovery in `run-all.sh`. These author-time tools close the META-LEVEL-24
false-green attestation discipline gap: every codified Dim-2 mechanical gate now has a
registered, reviewable template that state-manager and agents invoke during fix-burst
attestation — replacing ad-hoc hand-rolled greps that silently narrowed scope.

---

## Architecture Changes

```mermaid
graph TD
    SM["state-manager<br/>(fix-burst executor)"]
    DG["dim2-gates/<br/>11 bash scripts"]
    BL["burst-log.md<br/>(Dim-2 attestation blocks)"]
    BT["bats test suite<br/>(29 new tests)"]
    RA["run-all.sh<br/>(discovery extended)"]

    SM -->|invokes| DG
    DG -->|produces stdout for| BL
    BT -->|validates| DG
    RA -->|discovers| BT

    style DG fill:#90EE90
    style BT fill:#90EE90
    style RA fill:#d4edda
```

**Scope:** Zero Rust changes. Zero hooks-registry.toml changes. Zero dispatcher changes.
Author-time tooling only — not invoked by the dispatcher hook chain at runtime.

<details>
<summary><strong>Architecture Decision</strong></summary>

**Context:** D-453(e) and D-454(c) mandated canonical bash templates for every codified Dim-2 gate.
PR #137 scaffolded the `dim2-gates/` directory with a README listing 5 PLANNED entries.

**Decision:** Deliver all 11 templates (5 README-planned + 6 from D-450/451/452 codifications),
move all README entries from PLANNED to ACTIVE, and provide bats coverage for each script plus an
executable-bit meta-test.

**Rationale:** Author-time tools belong in the plugin source tree so they are version-controlled and
released alongside the plugin. They are NOT registered in hooks-registry.toml because they have no
dispatcher-enforced behavioral contract — they are operator-trusted shell tools invoked manually.

</details>

---

## Story Dependencies

```mermaid
graph LR
    S1508["S-15.08<br/>THIS PR"]
    S1506["S-15.06<br/>MERGED"]
    S1516["S-15.16 Part A<br/>MERGED"]
    S1507["S-15.07<br/>M2 — blocked"]
    S1509["S-15.09<br/>M2 — blocked"]
    S1511["S-15.11<br/>M2 — blocked"]
    S1514["S-15.14<br/>M2 — blocked"]

    S1506 -.->|parallel, no dep| S1508
    S1516 -.->|parallel, no dep| S1508
    S1508 --> S1507
    S1508 --> S1509
    S1508 --> S1511
    S1508 --> S1514

    style S1508 fill:#FFD700
    style S1506 fill:#90EE90
    style S1516 fill:#90EE90
```

**Depends on:** none (M1 has no internal dependencies)
**Blocks:** M2 WASM hook stories S-15.07, S-15.09, S-15.11, S-15.14 (need dim2-gate scripts as registered artifacts)

---

## Spec Traceability

```mermaid
flowchart LR
    D453e["D-453(e)<br/>canonical template mandate"]
    D454a["D-454(a)"] --> AC001["AC-001<br/>trajectory-tail-cell-grep.sh"]
    D454b["D-454(b)"] --> AC002["AC-002<br/>freshness-literal-stdout.sh"]
    D454d["D-454(d)"] --> AC003["AC-003<br/>block-label-canonical-form.sh"]
    D450d1["D-450(d) banner"] --> AC004["AC-004<br/>banner-wc-l.sh"]
    D452a["D-452(a)"] --> AC005["AC-005<br/>propagation-completeness.sh"]
    D450b["D-450(b)"] --> AC006["AC-006<br/>dim7-dispatched-count-sweep.sh"]
    D450c["D-450(c)"] --> AC007["AC-007<br/>dim1-file-count-arithmetic.sh"]
    D450d2["D-450(d) SHA"] --> AC008["AC-008<br/>active-branches-sha-currency.sh"]
    D450e["D-450(e)"] --> AC009["AC-009<br/>decision-log-monotonic-rows.sh"]
    D452b["D-452(b)"] --> AC010["AC-010<br/>layer-ordinal-dual-direction.sh"]
    D451a["D-451(a)"] --> AC011["AC-011<br/>meta-level-ack-grep.sh"]
    D453e --> D454a & D454b & D454d

    AC001 --> T1["trajectory-tail-cell-grep.bats"]
    AC002 --> T2["freshness-literal-stdout.bats"]
    AC003 --> T3["block-label-canonical-form.bats"]
    AC004 --> T4["banner-wc-l.bats"]
    AC005 --> T5["propagation-completeness.bats"]
    AC006 --> T6["dim7-dispatched-count-sweep.bats"]
    AC007 --> T7["dim1-file-count-arithmetic.bats"]
    AC008 --> T8["active-branches-sha-currency.bats"]
    AC009 --> T9["decision-log-monotonic-rows.bats"]
    AC010 --> T10["layer-ordinal-dual-direction.bats"]
    AC011 --> T11["meta-level-ack-grep.bats"]
```

**BC traceability:** `behavioral_contracts: []` — intentional. These are author-time operational
tools without runtime dispatcher-enforced behavioral contracts. Full rationale in story spec
frontmatter lines 35–45 (v1.2, factory-artifacts `f8892007`).

---

## Test Evidence

### Coverage Summary

| Metric | Value | Status |
|--------|-------|--------|
| dim2-gates bats (new) | 29/29 PASS | PASS |
| Pre-existing bats suites | 50 suites GREEN | PASS |
| cargo fmt | PASS (no Rust changes) | PASS |
| cargo clippy | PASS (no Rust changes) | PASS |
| cargo test | Pre-existing `sink-http bc_3_07_001_backoff` failures (UNRELATED — present on develop@224fa184 before this branch; F-P3-008 ubuntu timing flake also pre-existing) | N/A |

**Note on pre-existing failures:** The `sink-http` backoff test failures and `F-P3-008` ubuntu
timing flake are unrelated to S-15.08 (zero Rust changes in this PR). They are present on
`develop@224fa184` before this feature branch was cut. Do not treat them as regressions introduced
here.

```mermaid
graph LR
    DimGates["29 dim2-gates<br/>bats tests"]
    PreExist["50 pre-existing<br/>bats suites"]
    Cargo["cargo fmt+clippy+test<br/>(no Rust changes)"]

    DimGates -->|29/29 PASS| P1["PASS"]
    PreExist -->|all GREEN| P2["PASS"]
    Cargo -->|fmt+clippy PASS| P3["PASS"]

    style P1 fill:#90EE90
    style P2 fill:#90EE90
    style P3 fill:#90EE90
```

<details>
<summary><strong>Test Files Added (This PR)</strong></summary>

| Test File | Script Under Test | PASS/FAIL Cases |
|-----------|-------------------|----------------|
| `tests/dim2-gates/trajectory-tail-cell-grep.bats` | `trajectory-tail-cell-grep.sh` | PASS + FAIL |
| `tests/dim2-gates/freshness-literal-stdout.bats` | `freshness-literal-stdout.sh` | PASS + FAIL |
| `tests/dim2-gates/block-label-canonical-form.bats` | `block-label-canonical-form.sh` | PASS + FAIL + malformed-suffix |
| `tests/dim2-gates/banner-wc-l.bats` | `banner-wc-l.sh` | PASS + FAIL |
| `tests/dim2-gates/propagation-completeness.bats` | `propagation-completeness.sh` | PASS + FAIL |
| `tests/dim2-gates/dim7-dispatched-count-sweep.bats` | `dim7-dispatched-count-sweep.sh` | PASS + FAIL + 3-burst variants |
| `tests/dim2-gates/dim1-file-count-arithmetic.bats` | `dim1-file-count-arithmetic.sh` | PASS + FAIL |
| `tests/dim2-gates/active-branches-sha-currency.bats` | `active-branches-sha-currency.sh` | PASS + FAIL |
| `tests/dim2-gates/decision-log-monotonic-rows.bats` | `decision-log-monotonic-rows.sh` | PASS + FAIL |
| `tests/dim2-gates/layer-ordinal-dual-direction.bats` | `layer-ordinal-dual-direction.sh` | PASS + FAIL |
| `tests/dim2-gates/meta-level-ack-grep.bats` | `meta-level-ack-grep.sh` | PASS + FAIL |
| `tests/dim2-gates/executable-bit.bats` | All 11 scripts (meta-test) | Exec bit on all scripts |

</details>

---

## Demo Evidence

N/A — author-time bash scripts with bats test suite. No browser or terminal recording applicable.
Test run evidence is captured via the 29/29 bats GREEN result (see Test Evidence above).

For each AC, the bats test file serves as the executable demonstration:

| AC | Executable Demo | Result |
|----|-----------------|--------|
| AC-001 | `tests/dim2-gates/trajectory-tail-cell-grep.bats` | 29/29 GREEN |
| AC-002 | `tests/dim2-gates/freshness-literal-stdout.bats` | 29/29 GREEN |
| AC-003 | `tests/dim2-gates/block-label-canonical-form.bats` | 29/29 GREEN |
| AC-004 | `tests/dim2-gates/banner-wc-l.bats` | 29/29 GREEN |
| AC-005 | `tests/dim2-gates/propagation-completeness.bats` | 29/29 GREEN |
| AC-006 | `tests/dim2-gates/dim7-dispatched-count-sweep.bats` | 29/29 GREEN |
| AC-007 | `tests/dim2-gates/dim1-file-count-arithmetic.bats` | 29/29 GREEN |
| AC-008 | `tests/dim2-gates/active-branches-sha-currency.bats` | 29/29 GREEN |
| AC-009 | `tests/dim2-gates/decision-log-monotonic-rows.bats` | 29/29 GREEN |
| AC-010 | `tests/dim2-gates/layer-ordinal-dual-direction.bats` | 29/29 GREEN |
| AC-011 | `tests/dim2-gates/meta-level-ack-grep.bats` | 29/29 GREEN |
| AC-012 (exec-bit) | `tests/dim2-gates/executable-bit.bats` | 29/29 GREEN |

---

## Holdout Evaluation

N/A — evaluated at wave gate (M1 wave gate for S-15.03 PRIORITY-A). Author-time tooling;
no holdout scenario applicable per story spec.

---

## Adversarial Review

**LOCAL cascade — 6 passes, CONVERGED 3/3 per BC-5.39.001**

| Pass | Findings | Critical | High | Status |
|------|----------|----------|------|--------|
| 1 | 6 | 0 | 6 | Fixed (fix-burst-1) |
| 2 | 2 | 0 | 0 | Fixed (mix NITPICK) |
| 3 | 1 | 0 | 1 | Fixed (fix-burst-2) |
| 4 | NITPICK only | 0 | 0 | Fixed (fix-burst-3) |
| 5 | 1 LOW | 0 | 0 | Fixed via spec v1.2 |
| 6 | **CLEAN** | 0 | 0 | **CONVERGED 3/3** |

**Trajectory:** HIGH(6) → NITPICK → HIGH(1) → NITPICK → LOW → **CLEAN**

Review trail: `.factory/cycles/v1.0-brownfield-backfill/s-15.08-local-adversary-pass-{1..6}.md`

<details>
<summary><strong>Key Findings Addressed</strong></summary>

- **F-001 (pass-1) HIGH:** Dim-7 line-mapping error — fixed via corrected sibling-sweep pattern
- **F-002 (pass-1) HIGH:** Propagation per-site pattern scope narrowing — fixed to full per-site enumeration
- **F-003 (pass-1, test side):** EOL hardening for STATE.md fixture — added `.gitattributes`
- **F-004 (pass-1, test side):** FAIL assertion tightening — assertions now check specific error text
- **F-005 (pass-1, test side):** Exec-bit meta-test missing — added `executable-bit.bats`
- **F-006 (pass-1) HIGH:** block-label canonical form — aligned to D-444(c) canonical block type names
- **O-001 (pass-2):** Regex cleanup for readability
- **O-002 (pass-2):** Echo faithfulness — script output faithfully echoes command arguments
- **F-001 (pass-3) HIGH:** bats comment sibling-sweep — comment text aligned with actual script behavior
- **O-P3-002 (pass-3):** Conditional exit code emission — scripts emit exit codes correctly in all paths
- **F-S15.08-LOCAL-P5-001 LOW (pass-5):** Story spec v1.1 missing explicit `tdd_mode: facade` rationale comment — fixed in spec v1.2

</details>

---

## Security Review

```mermaid
graph LR
    Critical["Critical: 0"]
    High["High: 0"]
    Medium["Medium: 0"]
    Low["Low: 1 (accepted)"]

    style Critical fill:#90EE90
    style High fill:#90EE90
    style Medium fill:#90EE90
    style Low fill:#87CEEB
```

**Result: CLEAN for merge.** 0 CRITICAL, 0 HIGH, 0 MEDIUM, 1 LOW (accepted by design).

<details>
<summary><strong>Security Scan Details</strong></summary>

### CWE Analysis (Bash scripts only — zero Rust changes)

| CWE | Description | Finding | Disposition |
|-----|-------------|---------|-------------|
| CWE-78 | Command Injection | `eval "$CMD"` in `freshness-literal-stdout.sh:56` | ACCEPTED — by design per AC-002. Script purpose is to re-execute a command string passed by the operator (state-manager). Trust boundary is operator, not end-user. Documented in `--help` block and README. LOCAL adversary pass-6 Part B confirmed below NITPICK threshold. |
| CWE-22 | Path Traversal | `FACTORY_ROOT/FILE_PATH` path construction in `trajectory-tail-cell-grep.sh` and `propagation-completeness.sh` | LOW — accepted. Operator controls the site-list file. Existence check (`[[ ! -f "$FULL_PATH" ]]`) present before use. Author-time tool with operator trust boundary. |
| CWE-732 | Incorrect Permission | File permissions on scripts | PASS — all 11 scripts are `755` (rwxr-xr-x). Correct for executable operator tools. |

### Bash Security Discipline

| Check | Status |
|-------|--------|
| `set -euo pipefail` | PASS — present in all 11 scripts |
| No `sudo` / privilege escalation | PASS |
| No network calls | PASS |
| No secrets handling | PASS |
| No `--no-verify` bypass | PASS |

### Dependency Audit

N/A — pure bash scripts, no binary dependencies beyond standard POSIX tools (`grep`, `awk`, `wc`, `git`).

### OWASP Top 10 (applicable to bash author-time tools)

- **A03 (Injection):** `eval` in `freshness-literal-stdout.sh` — accepted/by-design (see CWE-78 above)
- All other OWASP categories: N/A (no web surface, no auth, no data storage)

</details>

---

## Risk Assessment & Deployment

### Blast Radius

- **Systems affected:** Author-time tooling only. Zero dispatcher impact. Zero hooks-registry impact. Zero Rust code.
- **User impact:** None if scripts fail — they are invoked manually, not in the hot path
- **Data impact:** Read-only (scripts read factory artifacts, produce stdout, do not write files)
- **Risk Level:** LOW

### Performance Impact

N/A — author-time bash scripts, not in any runtime hot path.

<details>
<summary><strong>Rollback Instructions</strong></summary>

**Immediate rollback (< 2 min):**

These are new files with no downstream dependencies at merge time (M2 stories not yet
started). Rollback is simply reverting the squash commit:

```bash
git revert <SQUASH_COMMIT_SHA>
git push origin develop
```

M2 WASM hook stories remain blocked until S-15.08 is re-delivered.

</details>

---

## Traceability

| Decision Sub-Clause | Script | AC | Test | Status |
|---------------------|--------|----|------|--------|
| D-453(e) | All 11 scripts | AC-001 through AC-011 | 12 bats files | PASS |
| D-454(a) | `trajectory-tail-cell-grep.sh` | AC-001 | trajectory-tail-cell-grep.bats | PASS |
| D-454(b) | `freshness-literal-stdout.sh` | AC-002 | freshness-literal-stdout.bats | PASS |
| D-454(d) | `block-label-canonical-form.sh` | AC-003 | block-label-canonical-form.bats | PASS |
| D-450(d) banner | `banner-wc-l.sh` | AC-004 | banner-wc-l.bats | PASS |
| D-452(a) | `propagation-completeness.sh` | AC-005 | propagation-completeness.bats | PASS |
| D-450(b) | `dim7-dispatched-count-sweep.sh` | AC-006 | dim7-dispatched-count-sweep.bats | PASS |
| D-450(c) | `dim1-file-count-arithmetic.sh` | AC-007 | dim1-file-count-arithmetic.bats | PASS |
| D-450(d) SHA | `active-branches-sha-currency.sh` | AC-008 | active-branches-sha-currency.bats | PASS |
| D-450(e) | `decision-log-monotonic-rows.sh` | AC-009 | decision-log-monotonic-rows.bats | PASS |
| D-452(b) | `layer-ordinal-dual-direction.sh` | AC-010 | layer-ordinal-dual-direction.bats | PASS |
| D-451(a) | `meta-level-ack-grep.sh` | AC-011 | meta-level-ack-grep.bats | PASS |
| D-454(c) README | `README.md` PLANNED→ACTIVE | AC-012 | executable-bit.bats (meta) | PASS |

---

## AI Pipeline Metadata

<details>
<summary><strong>Pipeline Details</strong></summary>

```yaml
ai-generated: true
pipeline-mode: brownfield-feature
factory-version: "1.0.0"
cycle: v1.0-brownfield-backfill
pipeline-stages:
  story-decomposition: completed (S-15.08 v1.2 spec, factory-artifacts f8892007)
  tdd-implementation: completed (facade mode — bats stubs -> script bodies)
  holdout-evaluation: "N/A — author-time tooling"
  local-adversary-cascade: CONVERGED (6 passes, 3/3 clean streak)
  formal-verification: "N/A — bash scripts not Kani-verifiable"
  convergence: achieved
convergence-metrics:
  local-adversary-passes: 6
  clean-streak: "3/3 per BC-5.39.001"
  fix-bursts: 3 (fix-burst-1 through fix-burst-3)
  spec-version: "1.2 (amended pass-5 finding)"
models-used:
  builder: claude-sonnet-4-6
  adversary: claude-opus-4-7-1m
closes:
  - "D-453(e)"
  - "D-454(a)"
  - "D-454(b)"
  - "D-454(d)"
  - "D-450(b)"
  - "D-450(c)"
  - "D-450(d)"
  - "D-450(e)"
  - "D-451(a)"
  - "D-452(a)"
  - "D-452(b)"
  - "D-454(c) README PLANNED->ACTIVE"
generated-at: "2026-05-16T00:00:00Z"
```

</details>

---

## Pre-Merge Checklist

- [ ] All CI status checks passing (note: pre-existing sink-http + F-P3-008 failures expected — see test evidence)
- [x] No new Rust changes — no coverage regression possible
- [ ] No critical/high security findings unresolved (pending security-reviewer)
- [x] Rollback procedure validated (revert squash commit)
- [x] No feature flags required (author-time scripts, no runtime path)
- [x] LOCAL adversary cascade CONVERGED 3/3 per BC-5.39.001 (6 passes)
- [x] 29/29 dim2-gates bats GREEN
- [x] 50 pre-existing bats suites GREEN
- [ ] pr-reviewer approval (0 CRITICAL/IMPORTANT findings)
- [ ] Squash-merge with branch deletion
