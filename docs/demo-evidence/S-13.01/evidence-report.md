---
document_type: demo-evidence-report
story_id: S-13.01
product: vsdd-factory path governance bundle
pipeline_run: "2026-05-06T23:21:00Z"
demo_type: cli
recording_tool: vhs
producer: demo-recorder
status: complete
---

# Demo Evidence Report — S-13.01: Path Governance Bundle

**Story:** S-13.01 — Path Governance Bundle: Registry, WASM Hook, Skill Updates, relocate-artifact
**Behavioral Contracts:** BC-4.11.001 (validate-artifact-path hook), BC-6.22.001 (relocate-artifact skill)
**Evidence directory:** `docs/demo-evidence/S-13.01/`
**Recording tool:** VHS 0.10.0 — terminal session capture
**Font:** FiraCode Nerd Font Mono
**Theme:** Catppuccin Mocha — 1200x600 px

---

## AC Coverage Table

| AC | Demo File | Description | Format | Status | Traces To |
|----|-----------|-------------|--------|--------|-----------|
| AC-001 | [AC-001-load-registry-success.gif](AC-001-load-registry-success.gif) | `parse_registry` loads valid YAML; never panics on arbitrary input; all 6 AC-001 unit tests pass | GIF + WEBM | CAPTURED | BC-4.11.001 precondition 2 + VP-069 |
| AC-002 | [AC-002-canonical-match.gif](AC-002-canonical-match.gif) | `match_path` is pure and deterministic; matches BC, ADR, VP, story, cycle, PRD paths; 11 AC-002 tests pass | GIF + WEBM | CAPTURED | BC-4.11.001 invariant 2 + VP-070 |
| AC-003 | [AC-003-block-on-unregistered.gif](AC-003-block-on-unregistered.gif) | Hook emits `block_with_fix` + `ARTIFACT_PATH_UNREGISTERED` for unregistered `.factory/` path | GIF + WEBM | CAPTURED | BC-4.11.001 postcondition 6 |
| AC-004 | — | Non-`.factory/` paths return `Continue` immediately; no registry lookup. Runtime-only behavior; covered by cargo test suite, no separate VHS demo recorded. | — | SKIPPED | BC-4.11.001 postcondition 7 |
| AC-005 | [AC-005-enforcement-levels.gif](AC-005-enforcement-levels.gif) | `block`/`warn`/`advisory` enforcement levels per registry entry; 4 unit tests pass | GIF + WEBM | CAPTURED | BC-4.11.001 postconditions 3/4/5 |
| AC-006 | [AC-006-graceful-degrade.gif](AC-006-graceful-degrade.gif) | Absent registry and malformed YAML both return `Continue`; 2 graceful-degrade tests pass | GIF + WEBM | CAPTURED | BC-4.11.001 EC-001/EC-002 |
| AC-007 | [AC-007-skill-exists.gif](AC-007-skill-exists.gif) | `relocate-artifact/SKILL.md` exists; frontmatter visible; dry-run and apply modes documented | GIF + WEBM | CAPTURED | BC-6.22.001 postconditions 1-5 |
| AC-008 | [AC-008-009-bats-relocate.gif](AC-008-009-bats-relocate.gif) | `relocate-artifact --apply`: git mv, cross-ref update, decision-log entry; bats 11/11 ok | GIF + WEBM | CAPTURED | BC-6.22.001 postconditions 6-9 |
| AC-009 | [AC-008-009-bats-relocate.gif](AC-008-009-bats-relocate.gif) | `--apply` atomic abort: no moves if detection fails for any artifact; exit non-zero | GIF + WEBM | CAPTURED | BC-6.22.001 invariant 3 |
| AC-010 | — | Delivery sequencing gate: hook NOT registered until relocate-artifact reports 0 violations. Structural/procedural constraint; no runtime artifact to capture. | — | SKIPPED | BC-4.11.001 invariant 7 + BC-6.22.001 invariant 7 |
| AC-011 | — | 9 creation skills each reference `artifact-path-registry.yaml`. Verified by VP-072 bats (28/28 ok, tests 8-16). Structural documentation check; no separate VHS demo. | — | SKIPPED | BC-4.11.001 postcondition 8 |
| AC-012 | — | 5+ agent files contain registry preamble paragraph. Verified by VP-072 bats (28/28 ok, tests 17-21). Structural documentation check; no separate VHS demo. | — | SKIPPED | BC-4.11.001 postcondition 8 |
| AC-013 | [AC-013-vp072-sot.gif](AC-013-vp072-sot.gif) | VP-072 SOT invariant bats harness: 28/28 ok — no skill or hook embeds duplicate path enumeration | GIF + WEBM | CAPTURED | BC-6.22.001 invariant 1 + VP-072 |
| AC-014 | [AC-014-hook-registered.gif](AC-014-hook-registered.gif) | `hooks-registry.toml` contains `validate-artifact-path.wasm` entry for PreToolUse Write/Edit | GIF + WEBM | CAPTURED | BC-4.11.001 precondition 1 |
| AC-015 | [AC-015-no-hardcoded-paths.gif](AC-015-no-hardcoded-paths.gif) | `grep -c behavioral-contracts lib.rs` returns 0: no hardcoded path patterns in source | GIF + WEBM | CAPTURED | BC-4.11.001 invariant 1 |

---

## Cross-Cutting Demo

| Demo | Description | Recording | Status |
|------|-------------|-----------|--------|
| WASM-build | WASM artifact exists at expected path; `cargo build --target wasm32-wasip1` completes successfully | [WASM-build.gif](WASM-build.gif) | CAPTURED |

---

## Coverage Summary

**15 ACs in story. 11 captured. 4 skipped (structural/procedural).**

| Category | Count |
|----------|-------|
| Captured (VHS GIF + WEBM) | 11 ACs + 1 cross-cutting |
| Skipped — structural/documentation properties | 4 (AC-004, AC-010, AC-011, AC-012) |
| Failed recordings | 0 |

### Skipped AC Justifications

- **AC-004** (non-`.factory/` early exit): Observable only via unit test output; AC-004 tests are included in the full `cargo test -p validate-artifact-path` run captured by AC-001 through AC-006 tapes. No separate VHS demo adds signal.
- **AC-010** (delivery sequencing gate): A procedural ordering constraint enforced by task ordering and PR description content. No runtime artifact exists to capture. Evidence is the `"0 violations found"` entry in the cycle decision-log.
- **AC-011** (9 creation skills have registry preamble): A structural grep assertion. VP-072 bats test 8-16 verify this mechanically; those tests are visible in the AC-013 VP-072 recording.
- **AC-012** (5+ agent files have registry preamble): Same as AC-011. VP-072 bats tests 17-21 verify this; captured in AC-013 recording.

---

## Artifact Inventory

| File | Size | Type |
|------|------|------|
| AC-001-load-registry-success.gif | 79K | GIF |
| AC-001-load-registry-success.webm | 77K | WEBM |
| AC-002-canonical-match.gif | 106K | GIF |
| AC-002-canonical-match.webm | 95K | WEBM |
| AC-003-block-on-unregistered.gif | 101K | GIF |
| AC-003-block-on-unregistered.webm | 92K | WEBM |
| AC-005-enforcement-levels.gif | 102K | GIF |
| AC-005-enforcement-levels.webm | 95K | WEBM |
| AC-006-graceful-degrade.gif | 105K | GIF |
| AC-006-graceful-degrade.webm | 96K | WEBM |
| AC-007-skill-exists.gif | 214K | GIF |
| AC-007-skill-exists.webm | 204K | WEBM |
| AC-008-009-bats-relocate.gif | 205K | GIF |
| AC-008-009-bats-relocate.webm | 357K | WEBM |
| AC-013-vp072-sot.gif | 1.8M | GIF |
| AC-013-vp072-sot.webm | 554K | WEBM |
| AC-014-hook-registered.gif | 176K | GIF |
| AC-014-hook-registered.webm | 149K | WEBM |
| AC-015-no-hardcoded-paths.gif | 102K | GIF |
| AC-015-no-hardcoded-paths.webm | 72K | WEBM |
| WASM-build.gif | 156K | GIF |
| WASM-build.webm | 373K | WEBM |
| **Total (22 media files)** | **~5.7M** | — |

---

## Toolchain

| Tool | Version | Status |
|------|---------|--------|
| VHS | 0.10.0 | installed |
| bats | system | installed |
| cargo | 1.x (workspace) | installed |
| Playwright | — | not applicable (CLI product) |

---

## Recording Anomalies

- **`Wait+Line` not functional in VHS 0.10.0 on darwin:** `Wait+Line /\$/` timed out on every attempt (terminal prompt is `%` not `$`; VHS internal matcher also appears unreliable in this version). All tapes use `Sleep`-based timing instead. Commands complete well within the sleep windows given they run compiled binaries.
- **`Output` accepts only relative paths** in VHS 0.10.0. All tapes run from the worktree root `/Users/jmagady/Dev/vsdd-factory/.worktrees/S-13.01/` so relative paths resolve correctly.
- **VHS `Type` string restrictions:** Shell variable interpolation (`$?`) and em dashes (`—`) in `Type` argument strings cause parser errors. AC-015 tape uses a plain ASCII alternative command structure.

---

## PR Embedding Snippet

```markdown
## Demo Evidence (S-13.01)

| AC | Demo |
|----|------|
| AC-001 Registry load | ![AC-001](docs/demo-evidence/S-13.01/AC-001-load-registry-success.gif) |
| AC-002 Canonical match | ![AC-002](docs/demo-evidence/S-13.01/AC-002-canonical-match.gif) |
| AC-003 Block unregistered | ![AC-003](docs/demo-evidence/S-13.01/AC-003-block-on-unregistered.gif) |
| AC-005 Enforcement levels | ![AC-005](docs/demo-evidence/S-13.01/AC-005-enforcement-levels.gif) |
| AC-006 Graceful degrade | ![AC-006](docs/demo-evidence/S-13.01/AC-006-graceful-degrade.gif) |
| AC-007 Skill exists | ![AC-007](docs/demo-evidence/S-13.01/AC-007-skill-exists.gif) |
| AC-008/009 Bats relocate | ![AC-008-009](docs/demo-evidence/S-13.01/AC-008-009-bats-relocate.gif) |
| AC-013 VP-072 SOT | ![AC-013](docs/demo-evidence/S-13.01/AC-013-vp072-sot.gif) |
| AC-014 Hook registered | ![AC-014](docs/demo-evidence/S-13.01/AC-014-hook-registered.gif) |
| AC-015 No hardcoded paths | ![AC-015](docs/demo-evidence/S-13.01/AC-015-no-hardcoded-paths.gif) |
| WASM build | ![WASM](docs/demo-evidence/S-13.01/WASM-build.gif) |
```
