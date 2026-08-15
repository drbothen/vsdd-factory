# Demo Evidence — S-21.07: validate-cross-site-correspondence

Story: `S-21.07-validate-cross-site-correspondence`
Governing BC: `BC-5.39.010`
Gate type: PostToolUse WASM governance hook run under the factory-dispatcher (CLI/terminal
product — no browser UI). Recorded with VHS as a terminal-cast, per the demo-recorder
CLI-product convention.

This directory holds a single, concise (≈23s) VHS recording that illustrates the gate's
three core dispositions against **real** dispatcher + WASM output — not a mock or a
narrated transcript. It is illustrative evidence, not exhaustive AC coverage: the
exhaustive per-AC coverage mapping (28 ACs across bats integration tests and Rust unit
tests) lives in the story's Test Plan
(`.factory/stories/S-21.07-validate-cross-site-correspondence.md`, "## Test Plan"
section) and in `plugins/vsdd-factory/tests/validate-cross-site-correspondence.bats`.

## Files

| File | Description |
|------|-------------|
| `AC-DISPOSITIONS-block-indeterminate-continue.gif` | Recording, GIF (for PR embed) |
| `AC-DISPOSITIONS-block-indeterminate-continue.webm` | Recording, WebM (for archival) |
| `AC-DISPOSITIONS-block-indeterminate-continue.tape` | VHS script source |
| `run-scenario.sh` | Reproducible harness the recording drives — invokes the real `factory-dispatcher` binary against the real compiled `validate-cross-site-correspondence.wasm` plugin, mirroring the exact invocation pattern used by `plugins/vsdd-factory/tests/validate-cross-site-correspondence.bats` (PostToolUse envelope on stdin, `hooks-registry.toml` capability grant, `VSDD_LOG_DIR` internal JSONL log for advisory-level telemetry) |
| `scenarios/indeterminate-index-corrupt/` | Checked-in fixture (well-formed UTF-8) for the INDETERMINATE scenario; `run-scenario.sh` appends an invalid UTF-8 byte sequence to a *copy* of `BC-INDEX.md` at run time (see below) — the committed fixture itself is not corrupted |

## Scenarios demonstrated

### 1. BLOCK — AC-001 / BC-5.39.010 §PC2b

**Fixture:** `plugins/vsdd-factory/tests/fixtures/validate-cross-site-correspondence/a1-index-ahead-of-primary/`
(the same fixture used by bats test `T-P6B (PC2b): BC-5.39.010 index-newer-than-primary
blocks with prescribed text`).

BC-INDEX.md's body-table row cites `v1.11` for BC-5.39.010, but the BC file's own
frontmatter `version:` is `"1.10"` — the index is *newer* than the primary artifact it
describes. Per §PC2b this is anomalous (no legitimate burst-ordering explains the index
advancing ahead of the file it cites), so the gate BLOCKs (exit code 2) with the
POLICY 14/18 prescribed message, surfaced via the dispatcher's `block_reason=` stderr
field.

```
exit code: 2
block_reason="BLOCKED by validate-cross-site-correspondence: [1] validate-cross-site-correspondence
[Class A Arm1]: BC-INDEX.md body-table row for BC-5.39.010 cites v1.11 but frontmatter
version: is "1.10" — index is newer than primary. This is anomalous: the index cannot
legitimately advance ahead of the BC it cites. Verify no index row was updated
out-of-burst or under the wrong BC path. Update per POLICY 14 leg 5. Fix: review and fix
all cross-site correspondence issues listed above, then retry the write.
Code: POLICY 14/18."
```

### 2. INDETERMINATE advisory — AC-028 / BC-5.39.010 §PC15b / §PC26 (v1.22 / ADV-RECON11-001)

**Fixture:** `scenarios/indeterminate-index-corrupt/` (checked in as well-formed UTF-8;
`run-scenario.sh` corrupts a copy of `BC-INDEX.md` at run time with an invalid UTF-8
byte sequence, `\xff\xfe`, appended after the otherwise-valid content — simulating a
mis-encoded save of the index file).

The primary target (`BC-5.39.010.md`) decodes fine and its `version:` matches the last
known-good index row (`1.6`); the primary-target read path (§PC15a / §PC25) is not
exercised here. The *secondary* read target, `BC-INDEX.md`, succeeds as a raw byte read
(`host::read_file` returns `Ok(bytes)`) but fails `std::str::from_utf8` decode inside
`extract_bc_index_version_state`. This is the `BcIndexVersionState::IndexUnreadable`
state (added v1.22): the row/hash state for the BC is genuinely **indeterminate**, not
confirmed-absent, so the gate MUST NOT fall through to the `RowAbsent` BLOCK path (which
would misreport index corruption as a dropped BC registration). Disposition is a
*distinct* advisory (emitted via `host::log_warn`, visible in the dispatcher's internal
JSONL log as a `plugin.log` record with `"level":"warn"`) plus **Continue** (exit 0).

```
exit code: 0
internal log advisory (host::log_warn) record:
"message":"validate-cross-site-correspondence: BC-INDEX.md failed UTF-8 decode — row/hash
state for 'BC-5.39.010' is INDETERMINATE, not confirmed-absent. Fix: verify the index
file's encoding and re-save as UTF-8."
```

Note this is a genuinely distinct code path from Scenario 3 below — both scenarios exit
0 and produce no `block_reason`, but only this scenario emits the `IndexUnreadable`
advisory record in the internal log. AC-028 is Rust-unit-tested (not bats-integration
tested — see `arm_a1.rs` / `arm_b.rs`); this demo is the first fixture-driven,
real-dispatcher-invoked reproduction of the scenario.

### 3. CONTINUE (pass) — well-formed, consistent artifacts

**Fixture:** `plugins/vsdd-factory/tests/fixtures/validate-cross-site-correspondence/a1-current-index/`
(the A1 control fixture also used by several bats CONTROL tests).

BC-INDEX.md's row cites `v1.6`, matching the BC file's frontmatter `version: "1.6"`
exactly. No violation, no advisory — the gate is silent and the write proceeds normally.

```
exit code: 0
no block, no advisory — write proceeds normally
```

## How to reproduce

```bash
# From the repo/worktree root:
cargo build -p factory-dispatcher
cargo build --release --target wasm32-wasip1 -p validate-cross-site-correspondence
cp target/wasm32-wasip1/release/validate_cross_site_correspondence.wasm \
   plugins/vsdd-factory/hook-plugins/validate-cross-site-correspondence.wasm

./docs/demo-evidence/S-21.07/run-scenario.sh block            # AC-001 / PC2b
./docs/demo-evidence/S-21.07/run-scenario.sh indeterminate    # AC-028 / PC15b / PC26
./docs/demo-evidence/S-21.07/run-scenario.sh continue         # clean pass
./docs/demo-evidence/S-21.07/run-scenario.sh all              # all three, in sequence
```

To re-record the tape:

```bash
vhs docs/demo-evidence/S-21.07/AC-DISPOSITIONS-block-indeterminate-continue.tape
```

**Tooling note:** the VHS release installed in this environment (0.11.0) does not
reliably support the `Wait` / `Wait+Line` tape commands — the wait would hang and abort
the recording even though the awaited text was already visible on screen (verified via
`Screenshot` probes during authoring). The tape therefore uses fixed `Sleep` durations
instead of `Wait+Line`; the underlying `run-scenario.sh` invocations complete in well
under 0.5s each (measured with `/usr/bin/time`), so the sleeps are a generous safety
margin, not a guess. If re-recording on a newer VHS release, `Wait+Line` may be
substituted back in for tighter timing.
