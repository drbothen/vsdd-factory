#!/usr/bin/env bats
# E-8-artifacts-gate.bats — AC gate tests for S-8.00 output artifacts
#
# Purpose: Verify that the implementer has produced the two canonical output
# artifacts required by S-8.00 with the correct schema and content shape.
#
# Artifacts under test:
#   .factory/measurements/E-8-bash-baseline.json  (AC-1, AC-2, AC-7)
#   .factory/cycles/v1.0-brownfield-backfill/E-8-bc-anchor-table.md  (AC-4, AC-5, AC-8)
#
# RED GATE: ALL tests MUST FAIL until the implementer runs Task A.4 (write
# E-8-bash-baseline.json) and Task B.5 (write E-8-bc-anchor-table.md).
# The .factory/ paths are gitignored on the feature branch; they are written
# locally in the worktree during implementation and committed to factory-artifacts
# by state-manager at GREEN/seal time.
#
# Prerequisites:
#   - jq >= 1.6
#   - grep, awk (standard)
#
# Run: bats tests/perf/E-8-artifacts-gate.bats
#
# Story: S-8.00 (Wave 15 entry-point pre-work)
# See: .factory/stories/S-8.00-perf-baseline-bc-anchor-verification.md

# ---------------------------------------------------------------------------
# Helper: resolve worktree root regardless of cwd
# ---------------------------------------------------------------------------
setup() {
  WORKTREE_ROOT="$(git rev-parse --show-toplevel)"
  BASELINE_JSON="${WORKTREE_ROOT}/.factory/measurements/E-8-bash-baseline.json"
  BC_ANCHOR_TABLE="${WORKTREE_ROOT}/.factory/cycles/v1.0-brownfield-backfill/E-8-bc-anchor-table.md"
}

# ---------------------------------------------------------------------------
# AC-1 + AC-2: E-8-bash-baseline.json existence and top-level parse
# ---------------------------------------------------------------------------

# AC-1
@test "AC-1+AC-2: .factory/measurements/E-8-bash-baseline.json exists and parses" {
  # Fails until Task A.4 writes the file
  [ -f "$BASELINE_JSON" ]
  # Must be valid JSON
  run jq 'empty' "$BASELINE_JSON"
  [ "$status" -eq 0 ]
}

# AC-1: hooks array has exactly 3 entries with non-null median_ms
# Schema: .hooks is an array of 3 objects each with median_ms field
@test "AC-1: hooks array has exactly 3 entries with non-null median_ms" {
  [ -f "$BASELINE_JSON" ]

  # Array length must be 3
  hook_count=$(jq '.hooks | length' "$BASELINE_JSON")
  [ "$hook_count" -eq 3 ]

  # Every entry must have a non-null numeric median_ms
  null_count=$(jq '[.hooks[] | select(.median_ms == null)] | length' "$BASELINE_JSON")
  [ "$null_count" -eq 0 ]

  # Every median_ms must be a number (not string, not null)
  non_numeric=$(jq '[.hooks[] | select(.median_ms | type != "number")] | length' "$BASELINE_JSON")
  [ "$non_numeric" -eq 0 ]
}

# AC-2: tier2_aggregate_projection.projected_aggregate_ms is computed and non-null
@test "AC-2: tier2_aggregate_projection has projected_aggregate_ms computed (non-null)" {
  [ -f "$BASELINE_JSON" ]

  # Field must exist and be a number
  val=$(jq '.tier2_aggregate_projection.projected_aggregate_ms' "$BASELINE_JSON")
  [ "$val" != "null" ]
  [ -n "$val" ]
  run jq -e '.tier2_aggregate_projection.projected_aggregate_ms | type == "number"' "$BASELINE_JSON"
  [ "$status" -eq 0 ]
}

# AC-2: ac7b_attainable must be a boolean (true or false), not null
@test "AC-2: tier2_aggregate_projection.ac7b_attainable is a boolean (true|false), not null" {
  [ -f "$BASELINE_JSON" ]

  run jq -e '.tier2_aggregate_projection.ac7b_attainable | type == "boolean"' "$BASELINE_JSON"
  [ "$status" -eq 0 ]
}

# AC-7: bundle_size object has all four expected fields populated
# Fields: legacy_bash_adapter_wasm_bytes, all_hook_plugins_wasm_bytes,
#         dispatcher_binary_bytes, measured_at
# Non-null, non-zero allowed only for legacy_bash_adapter_wasm_bytes (may be 0 if absent at baseline)
@test "AC-7: bundle_size has all four byte fields populated (non-null, non-zero allowed for legacy_bash_adapter only)" {
  [ -f "$BASELINE_JSON" ]

  # All three byte fields must be present and non-null
  run jq -e '
    .bundle_size.legacy_bash_adapter_wasm_bytes != null and
    .bundle_size.all_hook_plugins_wasm_bytes != null and
    .bundle_size.dispatcher_binary_bytes != null and
    .bundle_size.measured_at != null
  ' "$BASELINE_JSON"
  [ "$status" -eq 0 ]

  # All byte fields must be numbers (not strings)
  run jq -e '
    (.bundle_size.legacy_bash_adapter_wasm_bytes | type == "number") and
    (.bundle_size.all_hook_plugins_wasm_bytes | type == "number") and
    (.bundle_size.dispatcher_binary_bytes | type == "number")
  ' "$BASELINE_JSON"
  [ "$status" -eq 0 ]
}

# ---------------------------------------------------------------------------
# AC-4: E-8-bc-anchor-table.md existence
# ---------------------------------------------------------------------------

# AC-4
@test "AC-4: .factory/cycles/v1.0-brownfield-backfill/E-8-bc-anchor-table.md exists" {
  # Fails until Task B.5 writes the file
  [ -f "$BC_ANCHOR_TABLE" ]
}

# AC-4: table must contain rows for all 9 Tier 1 hooks
# The 9 hooks (per story AC-4):
#   handoff-validator, pr-manager-completion-guard, track-agent-stop,
#   update-wave-state-on-merge, validate-pr-review-posted, session-learning,
#   warn-pending-wave-gate, track-agent-start, regression-gate
@test "AC-4: BC-anchor table has 9 Tier 1 hook rows (handoff-validator, pr-manager-completion-guard, track-agent-stop, update-wave-state-on-merge, validate-pr-review-posted, session-learning, warn-pending-wave-gate, track-agent-start, regression-gate)" {
  [ -f "$BC_ANCHOR_TABLE" ]

  # Each hook name must appear at least once as a table row entry
  grep -q "handoff-validator"           "$BC_ANCHOR_TABLE"
  grep -q "pr-manager-completion-guard" "$BC_ANCHOR_TABLE"
  grep -q "track-agent-stop"            "$BC_ANCHOR_TABLE"
  grep -q "update-wave-state-on-merge"  "$BC_ANCHOR_TABLE"
  grep -q "validate-pr-review-posted"   "$BC_ANCHOR_TABLE"
  grep -q "session-learning"            "$BC_ANCHOR_TABLE"
  grep -q "warn-pending-wave-gate"      "$BC_ANCHOR_TABLE"
  grep -q "track-agent-start"           "$BC_ANCHOR_TABLE"
  grep -q "regression-gate"             "$BC_ANCHOR_TABLE"

  # Count data rows (pipe-delimited rows that start with | and contain a hook name pattern)
  # Must have exactly 9 hook data rows
  row_count=$(grep -c "^\s*|\s*[a-z].*-.*\.sh\|^\s*|\s*[a-z].*-.*[a-z]\s*|" "$BC_ANCHOR_TABLE" 2>/dev/null || true)
  # Fallback: count rows containing the word "-validator" or "-guard" or "-gate" etc.
  hook_row_count=$(grep -cE "^\s*\|\s*(handoff-validator|pr-manager-completion-guard|track-agent-stop|update-wave-state-on-merge|validate-pr-review-posted|session-learning|warn-pending-wave-gate|track-agent-start|regression-gate)" "$BC_ANCHOR_TABLE" 2>/dev/null || echo "0")
  [ "$hook_row_count" -eq 9 ]
}

# AC-4: no 'TBD' values in the BC ID(s) column
# The implementer must have resolved all BC ID lookups — TBD is a stub marker
@test "AC-4: BC-anchor table has no 'TBD' values in BC ID(s) column" {
  [ -f "$BC_ANCHOR_TABLE" ]

  # Count table rows (starting with |) that still have TBD in the BC ID(s) column
  # Column 2 (0-indexed after first |) holds BC ID(s) per Tasks §B.1 schema:
  #   | Hook | BC ID(s) | BC Title(s) | Spec-Current Y/N | Gap-Found Y/N | Action-Needed |
  tbd_count=$(awk -F'|' '
    /^\s*\|/ {
      col2 = $3
      gsub(/^[ \t]+|[ \t]+$/, "", col2)
      if (col2 == "TBD") count++
    }
    END { print count+0 }
  ' "$BC_ANCHOR_TABLE")
  [ "$tbd_count" -eq 0 ]
}

# ---------------------------------------------------------------------------
# AC-5: every Gap-Found=Y row has Action-Needed starting with 'Draft BC-' or 'OQ-'
# ---------------------------------------------------------------------------

# AC-5
@test "AC-5: every Gap-Found=Y row has Action-Needed starting with 'Draft BC-' OR 'OQ-'" {
  [ -f "$BC_ANCHOR_TABLE" ]

  # Extract rows where column 5 (Gap-Found) = Y
  # Schema column order: Hook | BC ID(s) | BC Title(s) | Spec-Current | Gap-Found | Action-Needed
  # For each such row, column 6 (Action-Needed) must start with "Draft BC-" or "OQ-"
  bad_rows=$(awk -F'|' '
    /^\s*\|/ {
      gap_found = $6
      action_needed = $7
      gsub(/^[ \t]+|[ \t]+$/, "", gap_found)
      gsub(/^[ \t]+|[ \t]+$/, "", action_needed)
      if (gap_found == "Y") {
        if (action_needed !~ /^Draft BC-/ && action_needed !~ /^OQ-/) {
          print NR": gap_found=Y but Action-Needed does not start with Draft BC- or OQ-: ["action_needed"]"
        }
      }
    }
  ' "$BC_ANCHOR_TABLE")
  [ -z "$bad_rows" ]
}

# ---------------------------------------------------------------------------
# AC-8: audit table frontmatter document_type=audit-table and status=ready
# (not stub — the file must be the completed artifact, not a placeholder)
# ---------------------------------------------------------------------------

# AC-8
@test "AC-8: audit table frontmatter document_type=audit-table and status=ready (not stub)" {
  [ -f "$BC_ANCHOR_TABLE" ]

  # Must contain frontmatter with document_type: audit-table
  grep -q "document_type: audit-table" "$BC_ANCHOR_TABLE"

  # Must contain status: ready (not status: stub or status: draft)
  grep -q "^status: ready" "$BC_ANCHOR_TABLE"

  # Must NOT contain status: stub
  run grep -c "^status: stub" "$BC_ANCHOR_TABLE"
  [ "$output" = "0" ]
}
