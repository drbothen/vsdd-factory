---
document_type: fix-burst-plan
wave: 15
date: 2026-05-02
verdict: BLOCKED
producer: orchestrator
---

# W-15 Wave Gate Fix-Burst Plan

**Context:** W-15 wave gate ran 2026-05-02. Verdict: BLOCKED. User chose **Option A — clean up before release**: run a single "wave-15-gate-fixes" fix-burst PR addressing all CRITICAL + HIGH findings before cutting rc.3.

**Branch:** `feature/wave-15-gate-fixes` (create from develop @ 3adfe0b)
**Target PR title:** `[W-15-gate] fix critical+high findings before rc.3 release`
**Acceptance criteria:** Re-run wave gate (implementer + adversary + security-reviewer); verdict must be CONVERGED before next release.

---

## Findings to Address

From adversary review (`adversarial-reviews/wave-15-gate-adversary.md`):
- CRIT-W15-001: Release pipeline does not build 9 of 16 native WASM plugins
- CRIT-W15-002: handoff-validator `on_error=block` dead weight; 3 different block-mode patterns
- CRIT-W15-003: WASI preopened_dir grants unrestricted filesystem access
- CRIT-W15-004: update-wave-state-on-merge regex false positives
- CRIT-W15-005: W-15 closure narrative overstates adapter retirement scope

From security review (`security-reviews/wave-15-gate-security.md`):
- SEC-003: VSDD_SINK_FILE path injection in production dispatcher (HIGH)

Additional HIGH findings (non-blocking but included in fix burst):
- HIGH-W15-002: whitespace counting divergence (chars vs bytes)
- HIGH-W15-004: update-wave-state-on-merge `default = ["standalone"]` inversion
- 2 clippy errors in track-agent-stop (doc_overindented_list_items lib.rs:9 + let_unit_value lib.rs:154)

---

## Prioritized Remediation Steps

### Step 1: CRIT-W15-001 + bats failures — Fix Release Pipeline

**Files to change:**
- `.github/workflows/release.yml`
- `.github/workflows/ci.yml`

**Changes:**
1. Update the WASM plugin build step in both workflows to build all 16 native plugins. Replace per-plugin enumeration with workspace build:
   ```yaml
   - name: Build native WASM hook plugins
     run: |
       cargo build --release --target wasm32-wasip1 --workspace \
         --exclude update-wave-state-on-merge
       cargo build --release --target wasm32-wasip1 \
         -p update-wave-state-on-merge --no-default-features
   ```
   (update-wave-state-on-merge is excluded from the workspace build and built separately with `--no-default-features` to avoid the `standalone` default feature)
2. Stage all 16 `.wasm` outputs into `plugins/vsdd-factory/hook-plugins/`:
   ```yaml
   - name: Stage WASM artifacts
     run: |
       find target/wasm32-wasip1/release -name "*.wasm" -maxdepth 1 \
         -exec cp {} plugins/vsdd-factory/hook-plugins/ \;
   ```
3. Verify the staged count equals 16 (add a check step that fails CI if count != 16).

**Verification:** After fix, `ls plugins/vsdd-factory/hook-plugins/*.wasm | wc -l` must return 16. All 19 bats failures from the implementer review (16 hook plugin failures + 3 regression-v1.0 failures) should resolve.

---

### Step 2: SEC-003 (HIGH) — Gate VSDD_SINK_FILE Behind debug_assertions

**File to change:** `crates/factory-dispatcher/src/main.rs`

**Changes:**
1. Wrap the `VSDD_SINK_FILE` read and file-open logic in `#[cfg(debug_assertions)]`:
   ```rust
   #[cfg(debug_assertions)]
   let sink_file: Option<std::fs::File> = std::env::var("VSDD_SINK_FILE").ok().map(|path| {
       // Reject path traversal
       if path.contains("..") || std::path::Path::new(&path).is_absolute() {
           eprintln!("VSDD_SINK_FILE: rejected unsafe path: {}", path);
           return None;
       }
       std::fs::OpenOptions::new().create(true).append(true).open(&path)
           .map_err(|e| eprintln!("VSDD_SINK_FILE: open failed: {}", e))
           .ok()
   }).flatten();

   #[cfg(not(debug_assertions))]
   let sink_file: Option<std::fs::File> = None;
   ```
2. Ensure the `sink_file` variable is referenced in the same way downstream regardless of cfg gate.
3. Add a `// SECURITY: VSDD_SINK_FILE is debug-only; see SEC-003` comment.

**Verification:** Release build (`cargo build --release`) must not contain any reference to `VSDD_SINK_FILE` in the binary. Verify with `strings target/release/factory-dispatcher | grep VSDD_SINK_FILE` returns empty.

---

### Step 3: CRIT-W15-002 + HIGH-W15-003 — Canonical Block-Mode Pattern

**Files to change:**
- `crates/hook-plugins/handoff-validator/src/lib.rs`
- `crates/hook-plugins/pr-manager-completion-guard/src/lib.rs`
- `crates/hook-plugins/validate-pr-review-posted/src/lib.rs`
- `hooks-registry.toml` (handoff-validator entry)
- `docs/HOST_ABI.md` or equivalent

**Decision to record:** Choose one canonical advisory-block-mode pattern. Recommended: **Option A** — all three plugins emit `{"outcome":"block"}` on stdout (which fires the dispatcher's `plugin_requests_block` gate) and use `on_error=continue` in the registry. The `HookResult::Block` SDK variant addition is deferred to W-16 as a first-class gating mechanism.

**Changes for Option A:**
1. `handoff-validator/src/lib.rs`: Change return from `HookResult::Continue` to writing `{"outcome":"block","reason":"..."}` to stdout when blocking condition is met. Return `HookResult::Continue` always (block signal is via stdout, not return value).
2. `pr-manager-completion-guard/src/lib.rs`: Already emits `{"outcome":"block"}` — verify pattern is canonical; update doc comments to reference the canonical pattern.
3. `validate-pr-review-posted/src/lib.rs`: Change from `HookResult::Block` return to stdout emit pattern.
4. `hooks-registry.toml`: Change handoff-validator `on_error = "block"` to `on_error = "continue"` (since block signal is now via stdout, not panic behavior).
5. Add a section to HOST_ABI.md: "Advisory block-mode pattern: plugins emit `{"outcome":"block","reason":"..."}` to stdout. The dispatcher checks for this before proceeding. `on_error` in the registry controls crash behavior only."

**Record as:** D-NNN "W-15 gate fix: canonical advisory-block-mode pattern chosen — stdout emit, not HookResult::Block; HookResult::Block SDK extension deferred to W-16."

---

### Step 4: CRIT-W15-003 + SEC-001 — Document WASI Preopened_dir vs Capability Boundary

**Files to change:**
- `docs/HOST_ABI.md`
- `specs/architecture/SS-02.md` (if it exists; or equivalent spec)

**Approach (documentation route for v1.0; tightening for v1.1):**

Add a "Filesystem Access Model" section to HOST_ABI.md:
```markdown
## Filesystem Access Model

### WASI preopened directories
All plugins receive WASI preopened directory access to `CLAUDE_PROJECT_DIR` and
`FACTORY_STATE_FILE` parent with `DirPerms::all() | FilePerms::all()`. This means
any plugin can read and write within these directories using native WASI filesystem
calls (`std::fs::read`, `std::fs::write`, etc.) — no capability declaration required.

### host::write_file capability
The `host::write_file` host function provides an ADDITIONAL bounded-write mechanism
with BC-2.02.011 enforcement (max_bytes_per_call, path_allow list). Plugins that
declare `write_file` capability in hooks-registry.toml use this path.

### Relationship
WASI preopened access is the sandbox boundary. host::write_file capability gating
controls only the host function, not native WASI calls. Future releases (v1.1) will
tighten preopens to read-only by default; write access will require explicit capability.
```

**Record as:** D-NNN "W-15 gate fix: WASI preopened_dir vs write_file capability model documented in HOST_ABI.md; capability tightening deferred to v1.1."

---

### Step 5: CRIT-W15-004 — Fix update-wave-state-on-merge Regex

**File to change:** `crates/hook-plugins/update-wave-state-on-merge/src/lib.rs` line 98

**Change:**
```rust
// Before (false positives on "merge" and "squash"):
let re = Regex::new(r"(?i)STEP_COMPLETE: step=8.*status=ok|merge|squash").unwrap();

// After (matches doc comments and bash semantics):
let re = Regex::new(r"(?i)STEP_COMPLETE: step=8.*status=ok|merged|squash.*merge").unwrap();
```

**Add regression tests** covering false-positive strings:
- "fix merge conflict in lib.rs" — must NOT match
- "squash redundant commits" — must NOT match
- "squash strategy considered for release" — must NOT match
- "PR #99 merged" — must match
- "squash-merge S-8.09 into develop" — must match (squash.*merge)

**Verification:** `cargo test -p update-wave-state-on-merge` must pass all new regression tests.

---

### Step 6: HIGH-W15-004 — Invert update-wave-state-on-merge `default = ["standalone"]`

**File to change:** `crates/hook-plugins/update-wave-state-on-merge/Cargo.toml`

**Change:**
```toml
# Before:
[features]
default = ["standalone"]
standalone = []

# After:
[features]
default = []
standalone = []
```

**Impact:** Any build that does NOT explicitly pass `--features standalone` will now use the capability-gated `host::write_file` path (the safer default). The release.yml fix in Step 1 uses `--no-default-features` which is now redundant but harmless (keep it for clarity).

**Verification:** `cargo build --target wasm32-wasip1 -p update-wave-state-on-merge` (no feature flags) must produce a build using the host::write_file path. `cargo build --target wasm32-wasip1 -p update-wave-state-on-merge --features standalone` must produce the standalone variant.

---

### Step 7: CRIT-W15-005 — Qualify W-15 Closure Narrative as "Tier-1-only"

**Files to audit and update:**
1. `STATE.md` — Session Resume Checkpoint line 333: "0 Tier 1 hooks routing through legacy-bash-adapter (W-15 closure achieved)" — already has "Tier 1" qualifier; verify no other STATE.md text implies broader retirement
2. `stories/STORY-INDEX.md` — find any W-15 closure summary row and add "Tier-1-only" qualifier
3. `cycles/v1.0-brownfield-backfill/adversarial-reviews/wave-15-gate-adversary.md` (this file) — already documents the scope correctly
4. S-8.09 demo evidence directory — check for any "0 hooks on legacy-bash-adapter" claim without qualifier
5. Any release notes or cycle manifest that mentions adapter retirement

**Add to STATE.md tech debt section:** TD-014 — "Full Tier 2/3 legacy-bash-adapter retirement. W-15 retired Tier 1 only (12 hooks). Approximately 30+ Tier 2/3 hooks remain on legacy-bash-adapter (e.g., convergence-tracker, validate-bc-title). Calendar-gated to v1.0 GA close."

---

### Step 8: 2 Clippy Errors in track-agent-stop

**File to change:** `crates/hook-plugins/track-agent-stop/src/lib.rs`

**Error 1 — doc_overindented_list_items (lib.rs:9):**
Fix over-indented list items in doc comment. Change from 4-space indent to 2-space or `*` at column 0.

**Error 2 — let_unit_value (lib.rs:154):**
Fix `let () = expr;` pattern. Change to `expr;` (drop the let binding since the type is unit).

**Verification:** `cargo clippy -p track-agent-stop -- -D warnings` must pass with 0 errors.

---

### Step 9: HIGH-W15-002 — Align Whitespace Counting (chars vs bytes)

**Files to change:**
- `crates/hook-plugins/handoff-validator/src/lib.rs`
- `crates/hook-plugins/track-agent-stop/src/lib.rs`

**Decision:** Use `.chars().filter(|c| c.is_whitespace()).count()` (Unicode-aware) as the canonical approach. Update track-agent-stop to match.

**Change in track-agent-stop:**
```rust
// Before (ASCII-only bytes):
let ws_count = input.bytes().filter(|b| *b == b' ' || *b == b'\n' || *b == b'\t').count();

// After (Unicode-aware chars):
let ws_count = input.chars().filter(|c| c.is_whitespace()).count();
```

**Add a doc comment** in both plugins: `// Whitespace counted as Unicode codepoints (is_whitespace()); matches handoff-validator canonical pattern.`

**Verification:** Add a test in both plugins with a Unicode whitespace input (e.g., `"\u{00A0}"` non-breaking space) and verify counts match between plugins.

---

## PR Structure

**Single feature branch:** `feature/wave-15-gate-fixes` from develop @ 3adfe0b
**Single squash-merge PR** into develop after all steps complete
**PR description must include:**
- Link to adversarial review: `.factory/cycles/v1.0-brownfield-backfill/adversarial-reviews/wave-15-gate-adversary.md`
- Link to security review: `.factory/cycles/v1.0-brownfield-backfill/security-reviews/wave-15-gate-security.md`
- Checklist of 9 steps with PR diff pointers

**Post-merge actions:**
1. Re-run wave gate: implementer (full test suite) + adversary (fresh-context diff review) + security-reviewer
2. If CONVERGED: cut release/v1.0.0-rc.3 from develop
3. If further FINDINGS: iterate (another fix-burst)

---

## What to Read for Zero-Context Resume

A fresh-context resume agent needs only these files to pick up the fix-burst:

1. **This file** (you are reading it) — full prioritized remediation plan
2. `.factory/cycles/v1.0-brownfield-backfill/adversarial-reviews/wave-15-gate-adversary.md` — full adversary findings with code locations
3. `.factory/cycles/v1.0-brownfield-backfill/security-reviews/wave-15-gate-security.md` — full security findings with code locations
4. `.factory/STATE.md` Session Resume Checkpoint — branch HEADs and overall status

No prior chat context is required. This plan is self-contained.
