---
document_type: demo-evidence-report
product: "vsdd-factory / postcompact-reanchor.sh (S-18.05)"
pipeline_run: "2026-06-26T00:20:00Z"
demo_type: "cli"
recording_tool: "vhs"
status: complete
story_id: "S-18.05"
---

# Demo Evidence Report — S-18.05

**Story:** S-18.05 — postcompact-reanchor.sh Advisory Hook (PostCompact re-anchor from git-sourced STATE.md)
**BC:** BC-7.07.002 v1.13
**VP:** VP-089 v1.3
**Recording tool:** VHS 0.11.0 + bash shell fixtures (git repos with factory-artifacts branch + origin/develop ref)
**Recorded:** 2026-06-26

---

## Per-AC Demo Recordings

| AC | Description | Recording (.gif) | Recording (.webm) | Tape | Result |
|----|-------------|-----------------|-------------------|------|--------|
| AC-001 | Happy path — readable STATE.md + resolvable origin/develop → re-anchor block to stdout with git-sourced values + exit 0 | [AC-001-happy-path.gif](AC-001-happy-path.gif) | [AC-001-happy-path.webm](AC-001-happy-path.webm) | [AC-001-happy-path.tape](AC-001-happy-path.tape) | PASS |
| AC-002 | JSONL log appended — exactly 6 fields (event, current_cycle, current_step, develop_sha, timestamp, status); no wave_id; develop_sha from git rev-parse | [AC-002-jsonl-log.gif](AC-002-jsonl-log.gif) | [AC-002-jsonl-log.webm](AC-002-jsonl-log.webm) | [AC-002-jsonl-log.tape](AC-002-jsonl-log.tape) | PASS |
| AC-004 | No factory-artifacts write — HEAD before equals HEAD after; hook is read-only on factory-artifacts (Invariant 1) | [AC-004-inv1-no-write.gif](AC-004-inv1-no-write.gif) | [AC-004-inv1-no-write.webm](AC-004-inv1-no-write.webm) | [AC-004-inv1-no-write.tape](AC-004-inv1-no-write.tape) | PASS |
| AC-005 / EC-002 | factory-artifacts unreachable (no .git) → stdout WARN advisory + log status=warn + exit 0 | [AC-005-ec002-unreachable.gif](AC-005-ec002-unreachable.gif) | [AC-005-ec002-unreachable.webm](AC-005-ec002-unreachable.webm) | [AC-005-ec002-unreachable.tape](AC-005-ec002-unreachable.tape) | PASS |
| AC-006 / EC-003 | STATE.md fields absent (no current_cycle/current_step) → context=UNKNOWN + status=warn + exit 0 | [AC-006-ec003-fields-absent.gif](AC-006-ec003-fields-absent.gif) | [AC-006-ec003-fields-absent.webm](AC-006-ec003-fields-absent.webm) | [AC-006-ec003-fields-absent.tape](AC-006-ec003-fields-absent.tape) | PASS |
| F-P5-001 | Degraded SHA path: STATE.md readable but origin/develop ref absent → sha=UNKNOWN + log status=warn + exit 0 (behavioral-bug-fix path) | [FP5001-degraded-sha.gif](FP5001-degraded-sha.gif) | [FP5001-degraded-sha.webm](FP5001-degraded-sha.webm) | [FP5001-degraded-sha.tape](FP5001-degraded-sha.tape) | PASS |
| AC-010 | hooks-registry.toml postcompact-reanchor entry canonical shape: event=PostCompact, on_error=continue, legacy-bash-adapter.wasm, [hooks.capabilities] block | [AC-010-registry-shape.gif](AC-010-registry-shape.gif) | [AC-010-registry-shape.webm](AC-010-registry-shape.webm) | [AC-010-registry-shape.tape](AC-010-registry-shape.tape) | PASS |
| Supporting | Full bats suite 11/11 green — covers AC-001, AC-002, AC-003, AC-004, AC-005, AC-006, AC-008, AC-009, AC-010, F-P5-001, EC-005 | [SUPPORTING-bats-suite.gif](SUPPORTING-bats-suite.gif) | [SUPPORTING-bats-suite.webm](SUPPORTING-bats-suite.webm) | [SUPPORTING-bats-suite.tape](SUPPORTING-bats-suite.tape) | PASS (11/11) |

---

## AC Coverage Map

| AC | BC Trace | Demo Coverage | Notes |
|----|----------|--------------|-------|
| AC-001 | BC-7.07.002 postcondition 1 | AC-001-happy-path + SUPPORTING-bats-suite | Re-anchor block format; git-sourced context + sha |
| AC-002 | BC-7.07.002 postcondition 2 | AC-002-jsonl-log + SUPPORTING-bats-suite | 6-field JSONL log; develop_sha field; no wave_id |
| AC-003 | BC-7.07.002 postcondition 3 | SUPPORTING-bats-suite (test 9) | Cannot block compaction — exit 0 / on_error=continue |
| AC-004 | BC-7.07.002 postcondition 4 + invariant 1 | AC-004-inv1-no-write + SUPPORTING-bats-suite | HEAD unchanged; read-only on factory-artifacts |
| AC-005 | BC-7.07.002 postcondition 5 + EC-002 | AC-005-ec002-unreachable + SUPPORTING-bats-suite | WARN advisory on git failure; exit 0 |
| AC-006 | BC-7.07.002 EC-003 | AC-006-ec003-fields-absent + SUPPORTING-bats-suite | context=UNKNOWN when fields absent |
| AC-007 | BC-7.07.002 EC-005 | SUPPORTING-bats-suite (test 10 + 11) | mkdir fail path + log dir absent path |
| AC-008 | BC-7.07.002 postconditions 5+6 | SUPPORTING-bats-suite (test 6) | Exit 0 on ALL error paths |
| AC-009 | BC-7.07.002 invariant 2 | SUPPORTING-bats-suite (test 7) | Values from git, not env vars or in-context |
| AC-010 | BC-7.07.002 precondition 1 | AC-010-registry-shape + SUPPORTING-bats-suite | hooks-registry.toml canonical shape |
| F-P5-001 | BC-7.07.002 PC1 / Inv2 | FP5001-degraded-sha + SUPPORTING-bats-suite (test 10) | sha=UNKNOWN + status=warn when origin/develop absent |

---

## Fixture Architecture

Each VHS tape calls a self-contained bash script (`/tmp/demo-s1805-*.sh`) that:

1. Creates an isolated git fixture repo (`mktemp -d`) with:
   - A `factory-artifacts` branch containing `.factory/STATE.md` with test-controlled content
   - `refs/remotes/origin/develop` pointing to the develop HEAD SHA (happy-path tests)
   - `.factory/logs/` directory for log writes
2. Runs `postcompact-reanchor.sh` via `printf '{...PostCompact...}' | bash hook.sh` from the fixture repo's CWD
3. Asserts expected stdout, log, and exit-code behavior
4. Cleans up the temp dir

No `GIT_DIR` injection — hook uses CWD-based git discovery (production-equivalent, per ADR-026 §Decision 7 / AC-010).

---

## Toolchain

| Tool | Version | Status |
|------|---------|--------|
| VHS | 0.11.0 | installed (via Homebrew) |
| bash | system | installed |
| bats | 1.13.0 | installed |
| jq | system | installed |
| git | system | installed |
| Playwright | N/A | not needed (CLI product) |

---

## PR Embedding Snippet

```markdown
## Demo Evidence — S-18.05: postcompact-reanchor.sh

| Demo | AC | Result |
|------|----|--------|
| ![AC-001 happy path](docs/demo-evidence/S-18.05/AC-001-happy-path.gif) | AC-001 | PASS |
| ![AC-005 unreachable](docs/demo-evidence/S-18.05/AC-005-ec002-unreachable.gif) | AC-005/EC-002 | PASS |
| ![F-P5-001 degraded SHA](docs/demo-evidence/S-18.05/FP5001-degraded-sha.gif) | F-P5-001 | PASS |
| ![bats suite](docs/demo-evidence/S-18.05/SUPPORTING-bats-suite.gif) | 11/11 | PASS |
```

---

## Notes

- All recordings produced both `.gif` (PR embed) and `.webm` (archival) per VHS best practices
- Terminal font: Menlo (macOS system font — most reliable on darwin-arm64)
- Shell: bash (VHS default) with `PS1='$ '` set for consistent prompt rendering
- `Wait+Screen` used instead of `Wait+Line` — command output completes before the prompt returns, so `Wait+Screen` correctly matches output that remains visible in the terminal buffer
- Demo scripts are in `/tmp/` (not committed) — the tapes are the source of truth
- AC-003, AC-007, AC-008, AC-009 covered by SUPPORTING-bats-suite recording only (no separate VHS tape needed — bats output shows all 11 tests green including these ACs)
