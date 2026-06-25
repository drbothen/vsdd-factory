---
document_type: demo-evidence-report
product: "vsdd-factory dispatcher git_context payload injection"
story_id: S-18.04b-prereq
pipeline_run: "2026-06-24"
demo_type: "cli"
recording_tool: "vhs"
status: complete
---

# Demo Evidence Report — S-18.04b-prereq

## Product: Dispatcher git_context payload injection (ADR-029)
## Story: S-18.04b-prereq — PostToolUse Bash git-commit event injection
## Pipeline Run: 2026-06-24

All recordings invoke the real `target/release/factory-dispatcher` binary with a synthetic
factory-artifacts git repo and the `legacy-bash-adapter.wasm` payload-capture plugin. The
plugin writes the full enriched payload to a capture file; `jq` extracts and verifies the
`git_context` fields. Every demo shows actual dispatcher output, not simulated transcripts.

---

## Per-AC Demo Recordings

| AC | BC/VP Clause | Description | Recording | Format | Status |
|----|-------------|-------------|-----------|--------|--------|
| AC-001 | BC-1.16.001 PC1; VP-093-A | Qualifying PostToolUse Bash git-commit → four-field git_context injected with real HEAD/HEAD^ SHA and subject | [AC-001-qualifying-bash-git-commit-injection.gif](AC-001-qualifying-bash-git-commit-injection.gif) / [.webm](AC-001-qualifying-bash-git-commit-injection.webm) | gif+webm | recorded |
| AC-002 | BC-1.16.001 PC2; VP-093-B | git error (non-git factory dir) → all-empty git_context; dispatcher exits 0 (fail-open) | [AC-002-fail-open-git-error-all-empty.gif](AC-002-fail-open-git-error-all-empty.gif) / [.webm](AC-002-fail-open-git-error-all-empty.webm) | gif+webm | recorded |
| AC-003, AC-012 | BC-1.16.001 PC3; VP-093-C | git push (non-qualifying Bash) → git_context key ABSENT from payload | [AC-003-004-non-qualifying-no-injection.gif](AC-003-004-non-qualifying-no-injection.gif) / [.webm](AC-003-004-non-qualifying-no-injection.webm) | gif+webm | recorded |
| AC-004, AC-008 | BC-1.16.001 PC4; VP-093-D | PostToolUse Edit event → git_context key ABSENT (dispatcher never inspects command) | [AC-003-004-non-qualifying-no-injection.gif](AC-003-004-non-qualifying-no-injection.gif) / [.webm](AC-003-004-non-qualifying-no-injection.webm) | gif+webm | recorded |
| AC-005 | BC-1.16.001 PC5; ADR-029 §Decision 4 | HOST_ABI_VERSION remains 1; source constant + runtime summary confirm no ABI bump | [AC-005-host-abi-version-unchanged.gif](AC-005-host-abi-version-unchanged.gif) / [.webm](AC-005-host-abi-version-unchanged.webm) | gif+webm | recorded |
| AC-006, AC-011 | BC-1.16.001 PC6/INV5; VP-093-E | Initial commit (no HEAD^) → head_parent_subject="" and head_parent_sha="" (empty string, not null) | [AC-006-initial-commit-empty-parent-fields.gif](AC-006-initial-commit-empty-parent-fields.gif) / [.webm](AC-006-initial-commit-empty-parent-fields.webm) | gif+webm | recorded |

---

## Recording Details

### AC-001 / VP-093-A — Qualifying Bash git-commit: four-field injection

**What it shows:**
- Two-commit synthetic factory-artifacts repo (HEAD = "state: burst-02 Commit B", HEAD^ = "state: burst-01 Commit A")
- Qualifying PostToolUse envelope: `tool_name="Bash"`, command contains `git -C .factory commit`
- Dispatcher injects `git_context` with all four fields populated
- `head_sha` and `head_parent_sha` are 40-char hex strings matching real repo state
- Non-tautological: exact field values are verified against the real synthetic repo

**Tape:** [AC-001-qualifying-bash-git-commit-injection.tape](AC-001-qualifying-bash-git-commit-injection.tape)

---

### AC-002 / VP-093-B — Fail-open on git error: all-empty fields

**What it shows:**
- Factory dir is NOT a git repo (no `git init` performed)
- Git commands return non-zero exit; dispatcher logs `warn` and continues
- All four `git_context` fields are `""` (JSON empty string, not JSON null)
- Dispatcher exits 0 (fail-open; pipeline is never blocked by git errors)
- Non-tautological: fields are verified as `""` raw JSON, distinguishing from absence

**Tape:** [AC-002-fail-open-git-error-all-empty.tape](AC-002-fail-open-git-error-all-empty.tape)

---

### AC-003 + AC-004 / VP-093-C + VP-093-D — Non-qualifying events: no injection

**What it shows:**
- Scenario C: PostToolUse Bash with `git -C .factory push` — `git_context` key ABSENT
- Scenario D: PostToolUse Edit event — dispatcher never inspects command; `git_context` key ABSENT
- Positive-coverage sentinel: capture file is non-empty in both cases (plugin WAS invoked)
- Absence is intentional, not a routing failure (demonstrated by non-empty capture file)

**Tape:** [AC-003-004-non-qualifying-no-injection.tape](AC-003-004-non-qualifying-no-injection.tape)

---

### AC-005 / ADR-029 §Decision 4 — HOST_ABI_VERSION unchanged

**What it shows:**
- Source constant: `pub const HOST_ABI_VERSION: u32 = 1;` in `crates/factory-dispatcher/src/lib.rs`
- Dispatcher runtime summary line always contains `host_abi=1`
- `git_context` rides in `payload.extra` (HashMap); no new named `HookPayload` field added

**Tape:** [AC-005-host-abi-version-unchanged.tape](AC-005-host-abi-version-unchanged.tape)

---

### AC-006 + AC-011 / VP-093-E — Initial commit: parent fields are empty strings

**What it shows:**
- Single-commit factory-artifacts repo (no HEAD^)
- `head_subject` and `head_sha` are fully populated (HEAD exists on initial commit)
- `head_parent_subject = ""` and `head_parent_sha = ""` — NOT null, NOT absent
- Raw JSON confirmed as `""` not `null` via jq
- Non-tautological: `head_sha` is verified as 40-char hex, distinguishing from all-empty fail-open

**Tape:** [AC-006-initial-commit-empty-parent-fields.tape](AC-006-initial-commit-empty-parent-fields.tape)

---

## AC Coverage Matrix

| AC | VP Clause | Demo File | Result |
|----|-----------|-----------|--------|
| AC-001 (qualifying injection) | VP-093-A / BC-1.16.001 PC1 | AC-001 | PASS |
| AC-002 (fail-open git error) | VP-093-B / BC-1.16.001 PC2 | AC-002 | PASS |
| AC-003 (no injection: git push) | VP-093-C / BC-1.16.001 PC3 | AC-003-004 | PASS |
| AC-004 (no injection: Edit event) | VP-093-D / BC-1.16.001 PC4 | AC-003-004 | PASS |
| AC-005 (HOST_ABI_VERSION=1) | BC-1.16.001 PC5 / ADR-029 §D4 | AC-005 | PASS |
| AC-006 (four-field completeness) | VP-093-A+E / BC-1.16.001 PC6 | AC-001, AC-006 | PASS |
| AC-008 (trigger: commit not Edit) | BC-1.16.001 INV2 | AC-003-004 (D) | PASS |
| AC-009 (fail-open preserves pipeline) | BC-1.16.001 INV3 | AC-002 | PASS |
| AC-011 (initial commit: "" not null) | VP-093-E / BC-1.16.001 INV5 | AC-006 | PASS |
| AC-012 (git push not qualifying) | BC-1.16.001 EC-004 | AC-003-004 (C) | PASS |
| AC-013 (bats harness via dispatcher) | VP-093 (all tests) | all | PASS (bats VP-093 suite) |

ACs not explicitly demoed here (AC-007, AC-010) are structural/invariant constraints verified
by the bats suite (`vp093-git-context-injection.bats`) and Rust unit tests, rather than
distinct interactive behaviors requiring separate recordings.

---

## Harness Files

| File | Purpose |
|------|---------|
| [run-demo.sh](run-demo.sh) | Main demo harness — scenarios A/B/C/D/E |
| [check-abi.sh](check-abi.sh) | AC-005 helper — shows source constant + runtime host_abi=1 |

---

## Toolchain

| Tool | Version | Status |
|------|---------|--------|
| VHS | 0.11.0 | installed |
| jq | system | installed |
| git | system | installed |
| factory-dispatcher | built from source (feature/S-18.04b-prereq @ 27e56530) | present |
| legacy-bash-adapter.wasm | plugins/vsdd-factory/hook-plugins/ | present |

---

## PR Embedding Snippet

```markdown
## Demo Evidence — S-18.04b-prereq (dispatcher git_context injection)

| Scenario | AC | Recording |
|----------|----|-----------|
| Qualifying git-commit → four-field injection | AC-001/VP-093-A | ![AC-001](docs/demo-evidence/S-18.04b-prereq/AC-001-qualifying-bash-git-commit-injection.gif) |
| git error → all-empty fail-open | AC-002/VP-093-B | ![AC-002](docs/demo-evidence/S-18.04b-prereq/AC-002-fail-open-git-error-all-empty.gif) |
| git push + Edit → no injection | AC-003/004/VP-093-C+D | ![AC-003-004](docs/demo-evidence/S-18.04b-prereq/AC-003-004-non-qualifying-no-injection.gif) |
| HOST_ABI_VERSION = 1 | AC-005/ADR-029 §D4 | ![AC-005](docs/demo-evidence/S-18.04b-prereq/AC-005-host-abi-version-unchanged.gif) |
| Initial commit → parent fields = "" | AC-006+011/VP-093-E | ![AC-006](docs/demo-evidence/S-18.04b-prereq/AC-006-initial-commit-empty-parent-fields.gif) |
```

---

## Notes

- All demos invoke the real `target/release/factory-dispatcher` binary (built from S-18.04b-prereq @ 27e56530).
- The `legacy-bash-adapter.wasm` plugin routes the full enriched payload to a `capture.sh` script; `jq` verifies field values from the captured JSON — not from dispatcher logs.
- No source code or production tests were modified during demo recording.
- POLICY 10 layout: all files are under `docs/demo-evidence/S-18.04b-prereq/` (story-scoped subfolder).
