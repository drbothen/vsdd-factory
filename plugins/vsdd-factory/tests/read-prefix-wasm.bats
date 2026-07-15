#!/usr/bin/env bats
# read-prefix-wasm.bats — T-009 integration gates for hook-sdk read_prefix
# wrapper callable from a WASM plugin (S-19.06 AC-007).
#
# AC-007 requires three gates for the hook-sdk to correctly expose read_prefix:
#   Gate 1: safe Rust wrapper signature in crates/hook-sdk/src/host.rs
#   Gate 2: raw wire-ABI extern in crates/hook-sdk/src/ffi.rs (3 clauses)
#   Gate 3: read_prefix registered in the dispatcher host dispatch table
#
# This bats suite implements the subset of AC-007 gates that the existing
# bats harness supports: static file analysis via grep/awk on source files.
#
# FIXTURE WASM COMPILE/LINK (Gate 4 — integration):
#   AC-007 also requires "a fixture WASM plugin that imports and calls
#   read_prefix compiles and links successfully."  The current bats suite
#   contains no WASM compilation harness (no pattern in plugins/vsdd-factory/
#   tests/ for compiling a .rs fixture to .wasm and running it under the
#   dispatcher).  This assertion CANNOT be expressed in the current suite.
#   The compile/link gate is therefore listed here as a documented gap:
#     - Pre-implementation: gate cannot pass regardless (todo!() stubs)
#     - Post-implementation: gate must be exercised via a future WASM fixture
#       harness (out of scope for the bats suite at this story)
#
# RED GATE STATUS:
#   T-009a..T-009e (static file checks): PASS at Red Gate because the stubs
#   committed at e422a30e already carry the correct signatures and registrations.
#   These tests provide regression protection post-implementation but do NOT
#   constitute a meaningful Red Gate test for the behavioral contract.
#
#   The load-bearing Red Gate for AC-007 is the unit test for T-001..T-008+T-010
#   (via cargo test on read_prefix.rs), not this bats suite.
#
# VP Trace: VP-101
# Story: S-19.06
# BC: BC-1.17.001 v1.6 §(a) layering parenthetical + §Architecture Anchors
#     + Invariant 2

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../.." && pwd)"
  HOST_RS="$REPO_ROOT/crates/hook-sdk/src/host.rs"
  FFI_RS="$REPO_ROOT/crates/hook-sdk/src/ffi.rs"
  MOD_RS="$REPO_ROOT/crates/factory-dispatcher/src/host/mod.rs"
  READ_PREFIX_RS="$REPO_ROOT/crates/factory-dispatcher/src/host/read_prefix.rs"
  CI_YML="$REPO_ROOT/.github/workflows/ci.yml"
  RELEASE_YML="$REPO_ROOT/.github/workflows/release.yml"
}

# ---------------------------------------------------------------------------
# T-009a  AC-007 Gate 1 — safe wrapper signature in hook-sdk/src/host.rs
#
# BC-1.17.001 v1.6 §(a) layering parenthetical: the safe wrapper returns
# Result<Vec<u8>, HostError> (NOT -> i32).  Gate 1 asserts the FULL signature
# including return type, rejecting any -> i32 wrapper that would violate the
# layering parenthetical.
#
# Red Gate note: PASSES at Red Gate — stub committed with correct signature.
# Provides regression protection: if the signature is changed (e.g. return
# type narrowed to -> i32), this gate fires.
# ---------------------------------------------------------------------------
@test "T-009a AC-007 Gate 1: safe wrapper pub fn read_prefix signature in hook-sdk/src/host.rs" {
  [ -f "$HOST_RS" ] || {
    echo "FAIL: hook-sdk/src/host.rs not found at $HOST_RS"
    false
  }
  if ! grep -qE \
    'pub fn read_prefix\(path: &str, max_bytes: u32, timeout_ms: u32\) -> Result<Vec<u8>, HostError>' \
    "$HOST_RS"; then
    echo "FAIL: full safe wrapper signature not found in $HOST_RS"
    echo "Expected: pub fn read_prefix(path: &str, max_bytes: u32, timeout_ms: u32) -> Result<Vec<u8>, HostError>"
    echo "Searched: $HOST_RS"
    grep -n 'read_prefix' "$HOST_RS" || true
    false
  fi
}

# ---------------------------------------------------------------------------
# T-009b  AC-007 Gate 2 clause (i) — raw extern 6-param shape in ffi.rs
#
# BC-1.17.001 v1.6 §(a): the raw wire-ABI extern in ffi.rs has a 6-parameter
# pointer/length shape mirroring ffi::read_file exactly.  Specifically it must
# have both path_len and out_ptr_out in its parameter list.
#
# Red Gate note: PASSES at Red Gate — stub committed with correct shape.
# ---------------------------------------------------------------------------
@test "T-009b AC-007 Gate 2(i): raw extern pub safe fn read_prefix in hook-sdk/src/ffi.rs" {
  [ -f "$FFI_RS" ] || {
    echo "FAIL: hook-sdk/src/ffi.rs not found at $FFI_RS"
    false
  }
  # Clause (i)-a: pub safe fn read_prefix( present
  if ! grep -qE 'pub safe fn read_prefix\(' "$FFI_RS"; then
    echo "FAIL: 'pub safe fn read_prefix(' not found in $FFI_RS"
    grep -n 'read_prefix' "$FFI_RS" || true
    false
  fi
  # Clause (i)-b: 6-param shape — path_len AND out_ptr_out both present in the
  # read_prefix block.  Count must be 2 (one occurrence each).
  count=$(awk '/pub safe fn read_prefix\(/,/-> i32;/' "$FFI_RS" \
    | grep -cE 'path_len|out_ptr_out' || true)
  if [ "$count" -ne 2 ]; then
    echo "FAIL: expected 2 occurrences of (path_len|out_ptr_out) in read_prefix block in ffi.rs; got $count"
    awk '/pub safe fn read_prefix\(/,/-> i32;/' "$FFI_RS" || true
    false
  fi
}

# ---------------------------------------------------------------------------
# T-009c  AC-007 Gate 2 clause (ii) — vsdd import module attribute in ffi.rs
#
# The extern block must be under #[link(wasm_import_module = "vsdd")] so
# read_prefix lands in the vsdd:: namespace (not the default unnamed namespace).
#
# Red Gate note: PASSES at Red Gate — stub in the existing vsdd block.
# ---------------------------------------------------------------------------
@test "T-009c AC-007 Gate 2(ii): #[link(wasm_import_module = \"vsdd\")] attribute in ffi.rs" {
  [ -f "$FFI_RS" ] || {
    echo "FAIL: hook-sdk/src/ffi.rs not found at $FFI_RS"
    false
  }
  if ! grep -qF '#[link(wasm_import_module = "vsdd")]' "$FFI_RS"; then
    echo 'FAIL: #[link(wasm_import_module = "vsdd")] not found in ffi.rs'
    grep -n 'link\|wasm_import' "$FFI_RS" || true
    false
  fi
}

# ---------------------------------------------------------------------------
# T-009d  AC-007 Gate 2 clause (iii) — read_prefix in BOTH cfg blocks
#
# BC-1.17.001 v1.6 §Architecture Anchors: read_prefix must appear in:
#   (a) the #[cfg(target_arch = "wasm32")] extern block (wasm target)
#   (b) the pub mod host_stubs block (non-wasm stub for cargo test / clippy)
#
# Both blocks are required for the SDK to compile on all targets.
#
# Red Gate note: PASSES at Red Gate — both blocks have the stub committed.
# ---------------------------------------------------------------------------
@test "T-009d AC-007 Gate 2(iii): read_prefix in wasm32 extern block AND host_stubs in ffi.rs" {
  [ -f "$FFI_RS" ] || {
    echo "FAIL: hook-sdk/src/ffi.rs not found at $FFI_RS"
    false
  }
  # Check wasm32 cfg block contains read_prefix.
  if ! awk '/^#\[cfg\(target_arch = "wasm32"\)\]/,/^}/' "$FFI_RS" \
    | grep -q 'fn read_prefix'; then
    echo 'FAIL: read_prefix not found inside #[cfg(target_arch = "wasm32")] block in ffi.rs'
    awk '/^#\[cfg\(target_arch = "wasm32"\)\]/,/^}/' "$FFI_RS" || true
    false
  fi
  # Check host_stubs block contains read_prefix.
  if ! awk '/^pub mod host_stubs \{/,/^\}/' "$FFI_RS" \
    | grep -q 'fn read_prefix'; then
    echo 'FAIL: read_prefix not found inside pub mod host_stubs block in ffi.rs'
    awk '/^pub mod host_stubs \{/,/^\}/' "$FFI_RS" || true
    false
  fi
}

# ---------------------------------------------------------------------------
# T-009e  AC-007 Gate 3 — read_prefix registered in dispatcher dispatch table
#
# The dispatcher must call read_prefix::register(&mut linker) in setup_linker
# so the vsdd::read_prefix host import is available to WASM plugins at runtime.
#
# Red Gate note: PASSES at Red Gate — register() call was added to setup_linker
# in the stub commit at e422a30e.
# ---------------------------------------------------------------------------
@test "T-009e AC-007 Gate 3: read_prefix::register in factory-dispatcher/src/host/mod.rs" {
  [ -f "$MOD_RS" ] || {
    echo "FAIL: factory-dispatcher/src/host/mod.rs not found at $MOD_RS"
    false
  }
  if ! grep -qE 'read_prefix::register' "$MOD_RS"; then
    echo "FAIL: read_prefix::register not found in $MOD_RS"
    echo "Expected a call to read_prefix::register(&mut linker) in setup_linker"
    grep -n 'register' "$MOD_RS" || true
    false
  fi
}

# ---------------------------------------------------------------------------
# T-009f  AC-007 Gate 4 — fixture WASM compile/link
#
# Gate 4 asserts that a fixture WASM plugin which imports and calls
# hook_sdk::host::read_prefix compiles and links for the wasm32-wasip1 target.
# This is the FFI boundary integration gate: it exercises the full call path
# from wasm32 extern→safe wrapper in a real WASM binary, catching link errors
# that static grep checks (T-009a..T-009e) cannot detect.
#
# FIXTURE CRATE: crates/hook-plugins/read-prefix-fixture/
#   Package name: read-prefix-fixture
#   Build target: wasm32-wasip1
#   Convention:   mirrors all other hook-plugin crates under crates/hook-plugins/
#                 (e.g., precompact-flush, regression-gate, validate-burst-log)
#   Minimum structure the implementer must create:
#     crates/hook-plugins/read-prefix-fixture/Cargo.toml
#       [package] name = "read-prefix-fixture"
#       [dependencies] vsdd-hook-sdk = { path = "../../hook-sdk" }
#       [[bin]] name = "read-prefix-fixture" path = "src/main.rs"
#     crates/hook-plugins/read-prefix-fixture/src/main.rs
#       calls vsdd_hook_sdk::host::read_prefix("", 0, 0) in a no-op hook body
#     Cargo.toml (workspace root): add "crates/hook-plugins/read-prefix-fixture"
#       to the workspace members list
#
# RED GATE STATUS: FAILS at Red Gate — the fixture crate does not exist yet.
#   `cargo build -p read-prefix-fixture --target wasm32-wasip1` exits non-zero
#   ("package ID specification ... did not match any packages").
#   Will turn GREEN when the implementer creates the fixture crate.
#
# TIMEOUT: 120 seconds (WASM compile of a minimal crate; network-free).
# ---------------------------------------------------------------------------
@test "T-009f AC-007 Gate 4: fixture WASM read-prefix-fixture builds for wasm32-wasip1" {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../.." && pwd)"
  # This test must run from the repo root so Cargo picks up the workspace.
  cd "$REPO_ROOT"

  # Select platform-compatible timeout command.
  # macOS ships without GNU coreutils `timeout`; gtimeout is the homebrew alias.
  # If neither is available (rare CI case), fall through to plain cargo — the
  # package-not-found error fires in <1 s at Red Gate so no hang risk then.
  local timeout_cmd=""
  if command -v timeout &>/dev/null; then
    timeout_cmd="timeout 120"
  elif command -v gtimeout &>/dev/null; then
    timeout_cmd="gtimeout 120"
  fi

  local output
  local exit_code=0
  # shellcheck disable=SC2086  # word-split intentional: timeout_cmd may be empty
  output=$(${timeout_cmd} cargo build -p read-prefix-fixture --target wasm32-wasip1 2>&1) \
    || exit_code=$?

  if [ "$exit_code" -ne 0 ]; then
    echo "FAIL: cargo build -p read-prefix-fixture --target wasm32-wasip1 exited $exit_code"
    echo "--- cargo output (last 30 lines) ---"
    echo "$output" | tail -30
    echo "---"
    echo "ACTION: implement the fixture crate at crates/hook-plugins/read-prefix-fixture/"
    echo "  See the T-009f comment block above for the minimum structure."
    return 1
  fi
}

# ---------------------------------------------------------------------------
# T-009g  AC-003 static gate — closes described-but-unexecuted gap (F-P1-001)
#
# AC-003 (S-19.06 v1.20) requires that OUTPUT_TOO_LARGE does NOT appear in
# non-comment production code of read_prefix.rs.  The four-stage pipeline:
#
#   Stage 1: awk '/^#\[cfg\(test\)\]/{exit} {print}'
#            Scopes output to the production region only — stops at the test-
#            module boundary, excluding T-004's assert_ne!(codes::OUTPUT_TOO_LARGE,...)
#            and its assertion message string (POSIX awk; replaces BSD-
#            incompatible sed label loop).
#
#   Stage 2: stateful awk block-comment stripper (POSIX portable)
#            Strips /* ... */ block comments spanning multiple lines.
#
#   Stage 3: sed 's://.*::'
#            Strips // line comments and //! doc comments.
#
#   Stage 4: grep -oE 'OUTPUT_TOO_LARGE'
#            Emits the literal match only; empty output → gate passes.
#
# Mutation-liveness check (TD-VSDD-059): injects `codes::OUTPUT_TOO_LARGE`
# into the production region of a temp copy and asserts the gate fires.
# This is embedded as a self-verifying fixture within the test so it is
# permanently load-bearing.
#
# Red Gate note: PASSES at Red Gate — production region contains OUTPUT_TOO_LARGE
#   only in //! doc comments and // inline comments, all stripped by stage 3.
# ---------------------------------------------------------------------------
@test "T-009g AC-003 static gate: OUTPUT_TOO_LARGE absent from non-comment production code in read_prefix.rs" {
  [ -f "$READ_PREFIX_RS" ] || {
    echo "FAIL: read_prefix.rs not found at $READ_PREFIX_RS"
    false
  }

  # --- Mutation-liveness check: inject codes::OUTPUT_TOO_LARGE into the
  # production region of a temp copy and assert the gate fires.
  # The injection appears BEFORE // so sed stage 3 does NOT strip it.
  local mut_file
  mut_file=$(mktemp /tmp/t009g_mutant_XXXXXX.rs)

  # Insert the mutation line immediately before the #[cfg(test)] boundary so
  # stage-1 awk includes it in the production region it processes.
  awk '{
    if (/^#\[cfg\(test\)\]/) {
      print "let _m = codes::OUTPUT_TOO_LARGE; // mutation-liveness injection"
    }
    print
  }' "$READ_PREFIX_RS" > "$mut_file"

  local mutant_output
  mutant_output=$(
    awk '/^#\[cfg\(test\)\]/{exit} {print}' "$mut_file" \
      | awk 'BEGIN{b=0}{if(b){if(sub(/.*\*\//,""))b=0;else next};while(match($0,/\/\*/)){s=substr($0,1,RSTART-1);r=substr($0,RSTART+2);if(match(r,/\*\//)){$0=s substr(r,RSTART+RLENGTH)}else{$0=s;b=1;break}};print}' \
      | sed 's://.*::' \
      | grep -oE 'OUTPUT_TOO_LARGE' || true
  )
  rm -f "$mut_file"

  if [ -z "$mutant_output" ]; then
    echo "FAIL: mutation-liveness check — gate did NOT fire on mutant containing"
    echo "  'let _m = codes::OUTPUT_TOO_LARGE;' in the production region."
    echo "  The gate is not live: it cannot detect the violation it is designed to catch."
    false
  fi
  echo "PASS mutation-liveness: gate output on mutant = '$mutant_output'"

  # --- Primary check: gate must produce empty output on actual read_prefix.rs ---
  local actual_output
  actual_output=$(
    awk '/^#\[cfg\(test\)\]/{exit} {print}' "$READ_PREFIX_RS" \
      | awk 'BEGIN{b=0}{if(b){if(sub(/.*\*\//,""))b=0;else next};while(match($0,/\/\*/)){s=substr($0,1,RSTART-1);r=substr($0,RSTART+2);if(match(r,/\*\//)){$0=s substr(r,RSTART+RLENGTH)}else{$0=s;b=1;break}};print}' \
      | sed 's://.*::' \
      | grep -oE 'OUTPUT_TOO_LARGE' || true
  )

  if [ -n "$actual_output" ]; then
    echo "FAIL: AC-003 violation — OUTPUT_TOO_LARGE found in non-comment production code of read_prefix.rs"
    echo "  Gate output: '$actual_output'"
    echo "  Pipeline: awk test-strip | awk block-comment-strip | sed line-comment-strip | grep OUTPUT_TOO_LARGE"
    echo "  read_prefix MUST NEVER reference OUTPUT_TOO_LARGE in non-comment production code."
    false
  fi
}

# ---------------------------------------------------------------------------
# T-009h  POLICY 20 exclusion presence-gate — closes single-point-defense gap (F-P1-002)
#
# read-prefix-fixture is a test-only fixture crate (S-19.06 AC-007 compile/link
# gate, bats T-009f).  It must NEVER become a release artifact (POLICY 20).
# Defense requires exclusion at TWO layers:
#
#   Layer 1 (--exclude): every wasm32-wasip1 --workspace cargo build must carry
#     `--exclude read-prefix-fixture` so the fixture is never compiled into the
#     release artifact bundle.
#
#   Layer 2 (staging case-skip): every staging loop that iterates over
#     target/wasm32-wasip1/*/\*.wasm must have a `read-prefix-fixture.wasm)`
#     case that continues, so a stale build artifact from a prior caching
#     layer is never promoted to the registry.
#
# Assertions (exact counts — both removal AND un-swept new-build-additions fire):
#   (a) release.yml  --exclude read-prefix-fixture count == 1
#   (b) ci.yml       --exclude read-prefix-fixture count == 3  (lines ~181/~480/~624)
#   (c) ci.yml       staging case-skip count           == 3
#   (d) release.yml  staging case-skip count           == 2
#
# Mutation-liveness check: a temp copy of ci.yml with one --exclude line deleted
# produces a count != 3, confirming the count assertion would fire on removal.
#
# Red Gate note: PASSES at Red Gate — both layers present since the first
#   exclusion commit.
# ---------------------------------------------------------------------------
@test "T-009h POLICY 20 exclusion presence-gate: read-prefix-fixture excluded in all wasm32-wasip1 builds and staging loops" {
  [ -f "$CI_YML" ] || {
    echo "FAIL: ci.yml not found at $CI_YML"
    false
  }
  [ -f "$RELEASE_YML" ] || {
    echo "FAIL: release.yml not found at $RELEASE_YML"
    false
  }

  # --- (a) release.yml: --exclude read-prefix-fixture present (exact count == 1) ---
  local release_excl_count
  release_excl_count=$(grep -c '\-\-exclude read-prefix-fixture' "$RELEASE_YML" || true)
  if [ "$release_excl_count" -ne 1 ]; then
    echo "FAIL: release.yml '--exclude read-prefix-fixture' count: expected 1, got $release_excl_count"
    echo "  The wasm32-wasip1 workspace build in release.yml must carry exactly one"
    echo "  '--exclude read-prefix-fixture' flag (POLICY 20)."
    false
  fi

  # --- (b) ci.yml: --exclude read-prefix-fixture in every wasm32-wasip1 --workspace build (count == 3) ---
  local ci_excl_count
  ci_excl_count=$(grep -c '\-\-exclude read-prefix-fixture' "$CI_YML" || true)
  if [ "$ci_excl_count" -ne 3 ]; then
    echo "FAIL: ci.yml '--exclude read-prefix-fixture' count: expected 3, got $ci_excl_count"
    echo "  All three wasm32-wasip1 --workspace builds (~lines 181/480/624) must"
    echo "  carry '--exclude read-prefix-fixture' (POLICY 20)."
    echo "  If a new --workspace build was added without the exclusion, this gate fires."
    false
  fi

  # --- (c) ci.yml: staging case-skip read-prefix-fixture.wasm) count == 3 ---
  local ci_staging_count
  ci_staging_count=$(grep -c 'read-prefix-fixture\.wasm)' "$CI_YML" || true)
  if [ "$ci_staging_count" -ne 3 ]; then
    echo "FAIL: ci.yml staging case-skip 'read-prefix-fixture.wasm)' count: expected 3, got $ci_staging_count"
    echo "  All three staging loops in ci.yml must skip the fixture artifact (POLICY 20)."
    false
  fi

  # --- (d) release.yml: staging case-skip read-prefix-fixture.wasm) count == 2 ---
  local release_staging_count
  release_staging_count=$(grep -c 'read-prefix-fixture\.wasm)' "$RELEASE_YML" || true)
  if [ "$release_staging_count" -ne 2 ]; then
    echo "FAIL: release.yml staging case-skip 'read-prefix-fixture.wasm)' count: expected 2, got $release_staging_count"
    echo "  Both staging loops in release.yml must skip the fixture artifact (POLICY 20)."
    false
  fi

  # --- Mutation-liveness check: delete one --exclude line from a temp copy of ci.yml ---
  # Assert that the --exclude count drops below 3 (the gate would fire on the mutant).
  local mut_ci
  mut_ci=$(mktemp /tmp/t009h_ci_mutant_XXXXXX.yml)

  # Delete exactly the FIRST occurrence of '--exclude read-prefix-fixture' so the
  # temp copy has count == 2, confirming the == 3 assertion would fail on removal.
  awk 'seen==0 && /--exclude read-prefix-fixture/{seen=1;next}{print}' \
    "$CI_YML" > "$mut_ci"

  local mut_count
  mut_count=$(grep -c '\-\-exclude read-prefix-fixture' "$mut_ci" || true)
  rm -f "$mut_ci"

  if [ "$mut_count" -eq 3 ]; then
    echo "FAIL: mutation-liveness check — deleting one '--exclude read-prefix-fixture' line"
    echo "  still gives count == 3; the gate would NOT fire on removal."
    echo "  Expected mutant count != 3 to confirm gate liveness."
    false
  fi
  echo "PASS mutation-liveness: mutant ci.yml --exclude count = $mut_count (expected != 3)"

  echo "PASS T-009h: release.yml --exclude=$release_excl_count; ci.yml --exclude=$ci_excl_count; ci.yml staging=$ci_staging_count; release.yml staging=$release_staging_count"
}

# ---------------------------------------------------------------------------
# T-009 status summary (post T-009g/T-009h addition)
#
# T-009a..T-009e (static file checks): PASS at Red Gate — stubs at e422a30e
#   already carry the correct signatures and registrations.  These tests are
#   regression guards; they do NOT constitute a meaningful Red Gate.
#
# T-009f (fixture compile/link): FAILS at Red Gate — the fixture crate does
#   not exist.  This IS the Red Gate for AC-007 Gate 4.  It turns GREEN when
#   the implementer creates crates/hook-plugins/read-prefix-fixture/ and
#   registers it in the workspace Cargo.toml.
#
# T-009g (AC-003 static gate): PASSES at Red Gate — production region contains
#   OUTPUT_TOO_LARGE only in // and //! comments, all stripped by stage 3 sed.
#   Includes a self-verifying mutation-liveness fixture (injects bare
#   codes::OUTPUT_TOO_LARGE into the production region of a temp copy and
#   asserts the gate fires).  Closes F-P1-001 described-but-unexecuted gap.
#
# T-009h (POLICY 20 exclusion presence-gate): PASSES at Red Gate — both
#   --exclude flags and staging case-skips are present in ci.yml and
#   release.yml.  Exact-count assertions fire on both removal and un-swept
#   new-build additions.  Includes a self-verifying mutation-liveness check
#   (deletes one --exclude from a temp ci.yml copy, asserts count != 3).
#   Closes F-P1-002 single-point-defense gap.
#
# The load-bearing behavioral Red Gate for AC-007 overall remains the unit
# test suite (T-001..T-008+T-010 in read_prefix.rs), which tests the host-side
# logic.  T-009f closes the FFI-boundary gap: it proves the wasm32 extern
# block compiles and links against the real hook-sdk.
# ---------------------------------------------------------------------------
