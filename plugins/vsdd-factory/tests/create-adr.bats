#!/usr/bin/env bats
# create-adr.bats — TDD tests for the create-adr skill (S-6.01)
# Status: FAILING — skill not yet implemented. RED phase of TDD.
#
# Each @test maps directly to the test function names listed in the story spec
# acceptance criteria (AC-1 through AC-8).  The tests call a thin driver script
# at helpers/create-adr-driver.sh which is currently a stub that exits 1.  In
# the GREEN phase the implementer replaces the stub with real logic while the
# test assertions stay unchanged.
#
# CLI surface under test:
#   create-adr-driver.sh --title <title> [--subsystems <SS-NN,...>]
#                        [--supersedes <ADR-NNN>] [--brownfield] [--id <ADR-NNN>]
#
# Environment variables wired in setup():
#   DECISIONS_DIR  — path to fixture decisions/ directory
#   ARCH_INDEX     — path to fixture ARCH-INDEX.md
#   ADR_TEMPLATE   — path to real adr-template.md (read-only reference)
#   VALIDATE_BIN   — path to mock validate-template-compliance.sh

PLUGIN_ROOT="${BATS_TEST_DIRNAME}/.."
DRIVER="${BATS_TEST_DIRNAME}/helpers/create-adr-driver.sh"

# ---------------------------------------------------------------------------
# setup / teardown
# ---------------------------------------------------------------------------

setup() {
  TEST_TMPDIR=$(mktemp -d)
  export TEST_TMPDIR

  # Build fixture decisions/ directory with ADR-001 through ADR-013 stubs.
  DECISIONS_DIR="$TEST_TMPDIR/decisions"
  mkdir -p "$DECISIONS_DIR"
  export DECISIONS_DIR

  _make_adr_stub() {
    local num="$1" slug="$2" ss="$3"
    local id; id=$(printf "ADR-%03d" "$num")
    cat > "$DECISIONS_DIR/${id}-${slug}.md" <<FRONTMATTER
---
document_type: adr
adr_id: ${id}
status: proposed
date: 2026-01-01
subsystems_affected: [${ss}]
supersedes: null
superseded_by: null
---

# ${id}: stub
FRONTMATTER
  }

  _make_adr_stub  1  "rust-dispatcher"                "SS-01, SS-09"
  _make_adr_stub  2  "wasm-plugin-abi"                "SS-01, SS-02, SS-04"
  _make_adr_stub  3  "wasi-preview1"                  "SS-02, SS-04"
  _make_adr_stub  4  "toml-config"                    "SS-01, SS-09"
  _make_adr_stub  5  "multi-sink-observability"       "SS-01, SS-03"
  _make_adr_stub  6  "host-abi-version"               "SS-01, SS-02"
  _make_adr_stub  7  "always-on-telemetry"            "SS-01, SS-03"
  _make_adr_stub  8  "parallel-within-tier"           "SS-01"
  _make_adr_stub  9  "activation-platform-selection"  "SS-09"
  _make_adr_stub 10  "storedata-linker"               "SS-01, SS-02"
  _make_adr_stub 11  "dual-hook-routing-tables"       "SS-07, SS-09"
  _make_adr_stub 12  "legacy-bash-adapter-router"     "SS-04, SS-07"
  _make_adr_stub 13  "adversarial-review-structure"   "SS-05, SS-06"

  # Build fixture ARCH-INDEX.md with the matching Architecture Decisions table.
  ARCH_INDEX="$TEST_TMPDIR/ARCH-INDEX.md"
  export ARCH_INDEX
  cat > "$ARCH_INDEX" <<'ARCHINDEX'
---
document_type: architecture-index
---

# Architecture Index: vsdd-factory (fixture)

## Subsystem Registry

| SS-ID | Name |
|-------|------|
| SS-01 | Hook Dispatcher Core |
| SS-02 | Hook SDK and Plugin ABI |
| SS-03 | Observability Sinks |
| SS-04 | Plugin Ecosystem |
| SS-05 | Pipeline Orchestration |
| SS-06 | Skill Catalog |
| SS-07 | Hook Bash Layer |
| SS-08 | Templates and Rules |
| SS-09 | Configuration and Activation |
| SS-10 | CLI Tools and Bin |

## Architecture Decisions

| ID      | Decision Summary                                             | Subsystems            | File                                               |
|---------|--------------------------------------------------------------|-----------------------|----------------------------------------------------|
| ADR-001 | Compiled Rust dispatcher per platform                        | SS-01, SS-09          | decisions/ADR-001-rust-dispatcher.md               |
| ADR-002 | WASM (wasmtime) plugin ABI                                   | SS-01, SS-02, SS-04   | decisions/ADR-002-wasm-plugin-abi.md               |
| ADR-003 | WASI preview 1 for v1.0; preview 2 deferred                  | SS-02, SS-04          | decisions/ADR-003-wasi-preview1.md                 |
| ADR-004 | TOML for all configuration files                             | SS-01, SS-09          | decisions/ADR-004-toml-config.md                   |
| ADR-005 | Multi-sink observability natively in dispatcher              | SS-01, SS-03          | decisions/ADR-005-multi-sink-observability.md      |
| ADR-006 | HOST_ABI_VERSION as separate semver constant                 | SS-01, SS-02          | decisions/ADR-006-host-abi-version.md              |
| ADR-007 | Always-on dispatcher self-telemetry                          | SS-01, SS-03          | decisions/ADR-007-always-on-telemetry.md           |
| ADR-008 | Parallel-within-tier, sequential-between-tier execution      | SS-01                 | decisions/ADR-008-parallel-within-tier.md          |
| ADR-009 | Activation-skill-driven platform binary selection            | SS-09                 | decisions/ADR-009-activation-platform-selection.md |
| ADR-010 | StoreData-typed linker for host functions (invoke.rs pattern)| SS-01, SS-02          | decisions/ADR-010-storedata-linker.md              |
| ADR-011 | Dual hooks.json + hooks-registry.toml during migration       | SS-07, SS-09          | decisions/ADR-011-dual-hook-routing-tables.md      |
| ADR-012 | Legacy-bash-adapter as universal current router              | SS-04, SS-07          | decisions/ADR-012-legacy-bash-adapter-router.md    |
| ADR-013 | Cycle-keyed adversarial review structure                     | SS-05, SS-06          | decisions/ADR-013-adversarial-review-structure.md  |
ARCHINDEX

  # Point at the real adr-template.md (read-only; tests never write to it).
  ADR_TEMPLATE="$PLUGIN_ROOT/templates/adr-template.md"
  export ADR_TEMPLATE

  # Build a mock validate-template-compliance.sh whose exit code is controlled
  # by MOCK_VALIDATE_EXIT (default 0 = pass).
  VALIDATE_BIN="$TEST_TMPDIR/validate-template-compliance.sh"
  export VALIDATE_BIN
  cat > "$VALIDATE_BIN" <<'VALIDATE'
#!/usr/bin/env bash
# Mock validate-template-compliance.sh for create-adr tests.
# Exits with MOCK_VALIDATE_EXIT (default 0).
exit "${MOCK_VALIDATE_EXIT:-0}"
VALIDATE
  chmod +x "$VALIDATE_BIN"
}

teardown() {
  rm -rf "$TEST_TMPDIR"
}

# ---------------------------------------------------------------------------
# Helper: run the driver with standard env wired in
# ---------------------------------------------------------------------------

_run_driver() {
  run "$DRIVER" "$@"
}

# ---------------------------------------------------------------------------
# AC-1: ID allocation
# ---------------------------------------------------------------------------

@test "AC-1 [BC-6.20.001]: skill allocates next sequential ID with no collision" {
  # With ADR-001..ADR-013 in DECISIONS_DIR, the skill must propose ADR-014.
  # Driver is a stub → exits 1 (RED).
  _run_driver --title "New Decision" --subsystems "SS-06"
  [ "$status" -eq 0 ]
  [[ "$output" == *"ADR-014"* ]]
}

@test "AC-1 [BC-6.20.002]: skill refuses duplicate --id override" {
  # ADR-001 already exists; driver must exit non-zero.
  _run_driver --title "New Decision" --subsystems "SS-06" --id "ADR-001"
  [ "$status" -ne 0 ]
  [[ "$output" == *"already exists"* ]] || [[ "$stderr" == *"already exists"* ]]
}

@test "AC-1 [BC-6.20.003]: skill blocks on filesystem-vs-index mismatch" {
  # Remove ADR-013 from the filesystem but leave it in ARCH-INDEX → mismatch.
  rm "$DECISIONS_DIR/ADR-013-adversarial-review-structure.md"
  _run_driver --title "New Decision" --subsystems "SS-06"
  [ "$status" -ne 0 ]
  # Must report the inconsistency, not just silently fail.
  [[ "$output" == *"inconsisten"* ]] || [[ "$output" == *"mismatch"* ]] || [[ "$output" == *"ADR-013"* ]]
}

# ---------------------------------------------------------------------------
# AC-2: Frontmatter scaffold
# ---------------------------------------------------------------------------

@test "AC-2 [BC-6.20.004]: status is always 'proposed' at creation" {
  _run_driver --title "New Decision" --subsystems "SS-06"
  [ "$status" -eq 0 ]
  # The newly written file must contain status: proposed
  new_file=$(find "$DECISIONS_DIR" -name "ADR-014-*.md" 2>/dev/null | head -1)
  [ -n "$new_file" ]
  grep -q "status: proposed" "$new_file"
}

@test "AC-2 [BC-6.20.004]: date matches today's ISO-8601" {
  today=$(date +%Y-%m-%d)
  _run_driver --title "New Decision" --subsystems "SS-06"
  [ "$status" -eq 0 ]
  new_file=$(find "$DECISIONS_DIR" -name "ADR-014-*.md" 2>/dev/null | head -1)
  [ -n "$new_file" ]
  grep -q "date: ${today}" "$new_file"
}

@test "AC-2 [BC-6.20.005]: subsystems validated against ARCH-INDEX registry" {
  # SS-99 is not in the registry; driver must exit non-zero and name the bad ID.
  _run_driver --title "New Decision" --subsystems "SS-99"
  [ "$status" -ne 0 ]
  [[ "$output" == *"SS-99"* ]] || [[ "$output" == *"invalid"* ]] || [[ "$output" == *"unknown"* ]]
}

@test "AC-2 [BC-6.20.006]: --supersedes ID validated to exist" {
  # ADR-999 does not exist; driver must exit non-zero and mention the missing ID.
  _run_driver --title "New Decision" --subsystems "SS-06" --supersedes "ADR-999"
  [ "$status" -ne 0 ]
  [[ "$output" == *"ADR-999"* ]] || [[ "$output" == *"not found"* ]] || [[ "$output" == *"does not exist"* ]]
}

# ---------------------------------------------------------------------------
# AC-3: Supersession bidirectional patch
# ---------------------------------------------------------------------------

@test "AC-3 [BC-6.20.007]: supersedes patches old ADR superseded_by" {
  _run_driver --title "New Decision" --subsystems "SS-06" --supersedes "ADR-013"
  [ "$status" -eq 0 ]
  # ADR-013 must now have superseded_by: ADR-014
  grep -q "superseded_by: ADR-014" "$DECISIONS_DIR/ADR-013-adversarial-review-structure.md"
}

@test "AC-3 [BC-6.20.007, BC-6.20.012]: atomic rollback when supersession patch fails" {
  # Make ADR-013 read-only so the patch cannot be applied.
  chmod 444 "$DECISIONS_DIR/ADR-013-adversarial-review-structure.md"
  _run_driver --title "New Decision" --subsystems "SS-06" --supersedes "ADR-013"
  [ "$status" -ne 0 ]
  # No new ADR-014 file should have been left behind.
  new_file=$(find "$DECISIONS_DIR" -name "ADR-014-*.md" 2>/dev/null | head -1)
  [ -z "$new_file" ]
  # Must report what it rolled back.
  [[ "$output" == *"rollback"* ]] || [[ "$output" == *"revert"* ]] || [[ "$output" == *"failed"* ]]
}

# ---------------------------------------------------------------------------
# AC-4: ARCH-INDEX insertion
# ---------------------------------------------------------------------------

@test "AC-4 [BC-6.20.008]: row inserted in numeric order" {
  _run_driver --title "New Decision" --subsystems "SS-06"
  [ "$status" -eq 0 ]
  # ADR-014 row must appear after the ADR-013 row in ARCH-INDEX.
  adr013_line=$(grep -n "ADR-013" "$ARCH_INDEX" | head -1 | cut -d: -f1)
  adr014_line=$(grep -n "ADR-014" "$ARCH_INDEX" | head -1 | cut -d: -f1)
  [ -n "$adr013_line" ]
  [ -n "$adr014_line" ]
  [ "$adr014_line" -gt "$adr013_line" ]
}

@test "AC-4 [BC-6.20.008]: row is pipe-aligned with existing rows" {
  _run_driver --title "New Decision" --subsystems "SS-06"
  [ "$status" -eq 0 ]
  # Each data row in the table must start with a pipe character.
  # The inserted ADR-014 row must start with '|'.
  grep -q "^| ADR-014" "$ARCH_INDEX"
}

@test "AC-4 [BC-6.20.008]: slug derivation strips special chars" {
  # Title with special chars: slashes and angle brackets must be stripped from
  # the filename slug.  The title in the ARCH-INDEX row uses the original title.
  _run_driver --title "Decision <with/special> chars" --subsystems "SS-06"
  [ "$status" -eq 0 ]
  # Slug in the filename must not contain '<', '>', or '/'
  new_file=$(find "$DECISIONS_DIR" -name "ADR-014-*.md" 2>/dev/null | head -1)
  [ -n "$new_file" ]
  basename "$new_file" | grep -qv '[<>/]'
  # Original title preserved in ARCH-INDEX row
  grep -q "Decision <with/special> chars" "$ARCH_INDEX"
}

@test "AC-4 [BC-6.20.008]: missing Architecture Decisions section blocks insert" {
  # Remove the section header from ARCH-INDEX.
  grep -v "## Architecture Decisions" "$ARCH_INDEX" > "${ARCH_INDEX}.tmp"
  mv "${ARCH_INDEX}.tmp" "$ARCH_INDEX"
  _run_driver --title "New Decision" --subsystems "SS-06"
  [ "$status" -ne 0 ]
  # No new file should be written.
  new_file=$(find "$DECISIONS_DIR" -name "ADR-014-*.md" 2>/dev/null | head -1)
  [ -z "$new_file" ]
  # Must mention the missing section in the error output.
  [[ "$output" == *"Architecture Decisions"* ]] || [[ "$output" == *"section"* ]] || [[ "$output" == *"ARCH-INDEX"* ]]
}

# ---------------------------------------------------------------------------
# AC-5: Hand-off pattern — no ghost-writing, guidance block on stdout
# ---------------------------------------------------------------------------

@test "AC-5 [BC-6.20.009]: scaffold preserves template placeholder text verbatim" {
  _run_driver --title "New Decision" --subsystems "SS-06"
  [ "$status" -eq 0 ]
  new_file=$(find "$DECISIONS_DIR" -name "ADR-014-*.md" 2>/dev/null | head -1)
  [ -n "$new_file" ]
  # The template contains this exact placeholder; it must appear verbatim.
  grep -q "\[2-5 paragraphs\] Background, forces driving the decision" "$new_file"
}

@test "AC-5 [BC-6.20.009]: stdout guidance block lists all sections to flesh out" {
  _run_driver --title "New Decision" --subsystems "SS-06"
  [ "$status" -eq 0 ]
  [[ "$output" == *"Sections to flesh out"* ]]
  [[ "$output" == *"Context"* ]]
  [[ "$output" == *"Decision"* ]]
  [[ "$output" == *"Rationale"* ]]
  [[ "$output" == *"Consequences"* ]]
  [[ "$output" == *"Alternatives Considered"* ]]
  [[ "$output" == *"Source / Origin"* ]]
}

# ---------------------------------------------------------------------------
# AC-6: Brownfield mode annotation
# ---------------------------------------------------------------------------

@test "AC-6 [BC-6.20.010]: --brownfield flag injects Source/Origin annotation" {
  _run_driver --title "New Decision" --subsystems "SS-06" --brownfield
  [ "$status" -eq 0 ]
  new_file=$(find "$DECISIONS_DIR" -name "ADR-014-*.md" 2>/dev/null | head -1)
  [ -n "$new_file" ]
  grep -q "BROWNFIELD" "$new_file"
}

@test "AC-6 [BC-6.20.010]: --supersedes implies brownfield annotation" {
  _run_driver --title "New Decision" --subsystems "SS-06" --supersedes "ADR-012"
  [ "$status" -eq 0 ]
  new_file=$(find "$DECISIONS_DIR" -name "ADR-014-*.md" 2>/dev/null | head -1)
  [ -n "$new_file" ]
  grep -q "BROWNFIELD" "$new_file"
}

@test "AC-6 [BC-6.20.010]: no flag, no supersedes, no annotation" {
  _run_driver --title "New Decision" --subsystems "SS-06"
  [ "$status" -eq 0 ]
  new_file=$(find "$DECISIONS_DIR" -name "ADR-014-*.md" 2>/dev/null | head -1)
  [ -n "$new_file" ]
  # Must NOT contain the brownfield annotation.
  run grep -c "BROWNFIELD" "$new_file"
  [ "$output" -eq 0 ]
}

# ---------------------------------------------------------------------------
# AC-7: Final validation gate
# ---------------------------------------------------------------------------

@test "AC-7 [BC-6.20.011]: validation pass exits 0 and registers ADR" {
  export MOCK_VALIDATE_EXIT=0
  _run_driver --title "New Decision" --subsystems "SS-06"
  [ "$status" -eq 0 ]
  [[ "$output" == *"Template compliance: PASS"* ]]
  # ARCH-INDEX must contain ADR-014 row.
  grep -q "ADR-014" "$ARCH_INDEX"
}

@test "AC-7 [BC-6.20.011]: validation fail exits non-zero and skips index row" {
  export MOCK_VALIDATE_EXIT=1
  _run_driver --title "New Decision" --subsystems "SS-06"
  [ "$status" -ne 0 ]
  [[ "$output" == *"Template compliance: FAIL"* ]] || [[ "$output" == *"not registered"* ]]
  # ARCH-INDEX must NOT contain ADR-014 row.
  run grep -c "ADR-014" "$ARCH_INDEX"
  [ "$output" -eq 0 ]
}

@test "AC-7 [BC-6.20.011]: validation fail skips supersession patch" {
  export MOCK_VALIDATE_EXIT=1
  _run_driver --title "New Decision" --subsystems "SS-06" --supersedes "ADR-013"
  [ "$status" -ne 0 ]
  # Driver must report the validation failure.
  [[ "$output" == *"Template compliance: FAIL"* ]] || [[ "$output" == *"not registered"* ]] || [[ "$output" == *"FAIL"* ]]
  # ADR-013 must NOT have been patched with superseded_by.
  run grep -c "superseded_by: ADR-014" "$DECISIONS_DIR/ADR-013-adversarial-review-structure.md"
  [ "$output" -eq 0 ]
}

# ---------------------------------------------------------------------------
# AC-8: Atomicity / no partial state
# ---------------------------------------------------------------------------

@test "AC-8 [BC-6.20.012]: file write fail leaves no index row" {
  # Make DECISIONS_DIR read-only so the new file cannot be written.
  chmod 555 "$DECISIONS_DIR"
  _run_driver --title "New Decision" --subsystems "SS-06"
  status_after="$status"
  chmod 755 "$DECISIONS_DIR"
  [ "$status_after" -ne 0 ]
  # ARCH-INDEX must not contain ADR-014.
  run grep -c "ADR-014" "$ARCH_INDEX"
  [ "$output" -eq 0 ]
}

@test "AC-8 [BC-6.20.012]: index insert fail deletes the new file" {
  # Make ARCH-INDEX read-only so the row cannot be inserted.
  chmod 444 "$ARCH_INDEX"
  _run_driver --title "New Decision" --subsystems "SS-06"
  status_after="$status"
  chmod 644 "$ARCH_INDEX"
  [ "$status_after" -ne 0 ]
  # New ADR file must have been cleaned up.
  new_file=$(find "$DECISIONS_DIR" -name "ADR-014-*.md" 2>/dev/null | head -1)
  [ -z "$new_file" ]
}

@test "AC-8 [BC-6.20.012]: supersession patch fail deletes new file and rolls back index" {
  # Make ADR-013 read-only so the supersession patch fails mid-flight.
  chmod 444 "$DECISIONS_DIR/ADR-013-adversarial-review-structure.md"
  _run_driver --title "New Decision" --subsystems "SS-06" --supersedes "ADR-013"
  status_after="$status"
  chmod 644 "$DECISIONS_DIR/ADR-013-adversarial-review-structure.md"
  [ "$status_after" -ne 0 ]
  # New file must not exist.
  new_file=$(find "$DECISIONS_DIR" -name "ADR-014-*.md" 2>/dev/null | head -1)
  [ -z "$new_file" ]
  # ARCH-INDEX must not contain ADR-014.
  run grep -c "ADR-014" "$ARCH_INDEX"
  [ "$output" -eq 0 ]
}

@test "AC-8 [BC-6.20.012]: idempotent re-invocation succeeds after prior failure" {
  # First invocation: fail by making DECISIONS_DIR unwritable.
  chmod 555 "$DECISIONS_DIR"
  _run_driver --title "New Decision" --subsystems "SS-06"
  [ "$status" -ne 0 ]
  chmod 755 "$DECISIONS_DIR"

  # Second invocation with same arguments must succeed cleanly.
  _run_driver --title "New Decision" --subsystems "SS-06"
  [ "$status" -eq 0 ]
  grep -q "ADR-014" "$ARCH_INDEX"
}
