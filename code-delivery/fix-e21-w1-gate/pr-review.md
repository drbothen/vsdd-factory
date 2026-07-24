# PR #763 — Fresh-Eyes Final Review (Consolidated)

**Final verdict: APPROVE** (round-2 convergence)
**Round-1 verdict: REQUEST_CHANGES** (BLOCKING resolved by commit cb1b00d3; followed by linter improvement at 37d021a0)

Reviewer: pr-reviewer (fresh-eyes, different-model cognitive diversity). Reviewed the diff, PR description, commits, and CI status only.

One blocking completeness gap: the BC-5.41.004 sweep is claimed complete in the PR checklist but misses a sibling write-site that sets the identical `sprint-state.yaml` story-status field. Everything else (Rust refactor, test additions, security fix, doc changes) is sound.

---

## What I verified (clean)

- **MINOR-1 behavior identity (Rust refactor):** Traced all three original inline option-skip loops (`is_git_add_command`, `contains_factory_path_arg`, `find_factory_class_target`) against the extracted `skip_global_options` helper token-by-token. The only structural change: the original `is_git_add_command` returned `true` immediately when an unknown long-option's peek was `add`/`stage`; the helper instead leaves the index at that token and the caller's subcommand check returns `true` — same result. `-C`/`-c` factory-target capture is now computed for all three callers but discarded (`let (sub_idx, _) = …`) by the two that don't need it: no behavior change. Exhaustion paths (`sub_idx >= tokens.len()` → `break 'outer`) map to the originals. Behavior-preserving.
- **NITPICK-1 log-level consts:** `on_pre_tool_use` match rewrite `0..=log_level::INFO / log_level::WARN / _` is exactly `0..=2 / 3 / _`. Path constants in range patterns are valid Rust.
- **MINOR-2 / NITPICK-4:** `_run_dispatcher` temp-file envelope removes the nested `sed`/`printf` quote-escaping; `STDERR_FILE` → `_RUN_DISPATCHER_STDERR` rename swept across all 8 read sites with a contract comment. No stale references remain.
- **MINOR-3 (T-005b):** Not a trivial stub — three independently load-bearing assertions (STOP token, empty push-log, `UnverifiedNetNegativeDelta` token) with documented per-mutant (M1/M2/M3) rationale and an anti-tautology `--stat` doc-parity marker. (Could not see the `_run_gate` harness body from the diff to confirm `force_rd_fail=1` truly isolates the stat-only path, but the assertion structure is meaningful.)
- **W-SEC-003 (CWE-116):** The `gh pr comment` triage fix uses a quoted heredoc (`<< 'BODY'`, no expansion) into an `mktemp` file passed via `--body-file`, then `rm -f`. Correctly eliminates the shell-metacharacter interpolation surface, no new injection point. Other `--body` occurrence (`security-reviewer.md:129`) uses a fixed literal — correctly out of scope.
- **MINOR-4 / NITPICK-7 / NITPICK-8 / F-WG1-001/002:** Doc/comment additions accurate and consistent with surrounding text.
- **CI:** `validate`, SAST/Semgrep, legacy-bash bats, wave-handoff PASS. `cargo-host` (fmt/clippy/test) and `bats-full-suite` were still **pending** at review time — the 138/138 claim is not yet CI-confirmed.

---

## Findings

### 1. [BLOCKING] BC-5.41.004 sweep incomplete — `wave-gate/SKILL.md:125`

`plugins/vsdd-factory/skills/wave-gate/SKILL.md:125` (Gate 6, State Update) still instructs:

> `- Update sprint-state.yaml: all wave stories → \`completed\``

This writes the **same** `sprint-state.yaml` story-status field the sweep retired to `merged` in `deliver-story/SKILL.md`, `step-g-cleanup.md`, `workflows/phases/per-story-delivery.md`, and `phase-3-tdd-implementation.lobster`. Because wave-gate Gate 6 runs *after* all wave stories merge (each already set to `merged` by its own step-g cleanup), this path would overwrite `merged` back to `completed` — reintroducing exactly the divergence BC-5.41.004 targets. The pre-merge checklist asserts "BC-5.41.004 sweep complete: 4 instructional files"; that claim is false while this 5th write-site stands (TD-VSDD-060 sibling-site sweep).

**Fix:** sweep this line to `merged`. If wave-gate semantics are deliberately out of scope (wave-level batch vs per-story merge), correct the completeness claim and record the rationale rather than leaving a silent divergence.

### 2. [SUGGESTION] BC-5.41.004 status vocabulary — `phase-f3-incremental-stories/steps/step-01-load-story-graph.md:20`

The status enumeration `Story status map (completed / in-progress / planned)` omits the new canonical terminal value `merged`. Same BC-5.41.004 domain; the vocabulary is now inconsistent with the retired `completed` terminal.

### 3. [NIT] PR description drift (no code impact)

(a) The Security Review section describes W-SEC-003 as `gh pr create --body → gh pr create --body-file` with `<(cat <<'EOF')` process substitution, but the actual fix is on `gh pr **comment**` using an `mktemp` temp-file pattern. (b) The BC-5.41.004 findings-table file list names `step-f-pr-lifecycle.md` (which actually received the NITPICK-8 cross-ref) instead of `phase-3-tdd-implementation.lobster` (which actually received the `completed→merged` change).

---

## Recommendation

Blocking on finding #1. The Rust refactor, test additions, and security fix are all sound and behavior-preserving — this is purely a sweep-completeness gap. Sweeping `wave-gate/SKILL.md:125` (and ideally #2) to `merged`, or explicitly re-scoping and correcting the checklist claim, clears the block. Also recommend waiting for the pending `cargo-host` / `bats-full-suite` CI jobs to confirm the 138/138 + fmt/clippy claim before merge.
