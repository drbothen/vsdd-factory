# WASM Rule Audit
**Cycle:** v1.0-feature-plugin-async-semantics-pass-1
**Date:** 2026-05-07
**Rule:** "We are migrating to WASM — any new plugins need to use WASM."
**Auditor:** architect

---

## 1. Audit Scope

Files reviewed (18 total):

| # | File |
|---|------|
| 1 | `.factory/specs/architecture/decisions/ADR-019-plugin-async-semantics-at-registry-layer.md` |
| 2 | `.factory/specs/behavioral-contracts/ss-01/BC-1.14.001.md` |
| 3 | `.factory/specs/behavioral-contracts/ss-07/BC-7.06.001.md` |
| 4 | `.factory/specs/behavioral-contracts/ss-09/BC-9.01.006.md` |
| 5 | `.factory/specs/behavioral-contracts/ss-03/BC-3.08.001.md` |
| 6 | `.factory/specs/behavioral-contracts/ss-01/BC-1.08.001.md` |
| 7 | `.factory/specs/verification-properties/VP-077.md` |
| 8 | `.factory/specs/verification-properties/VP-078.md` |
| 9 | `.factory/specs/verification-properties/VP-079.md` |
| 10 | `.factory/specs/domain-spec/invariants.md` (DI-019 section) |
| 11 | `.factory/specs/behavioral-contracts/ss-01/BC-1.01.001.md` |
| 12 | `.factory/specs/behavioral-contracts/ss-01/BC-1.01.007.md` |
| 13 | `.factory/specs/behavioral-contracts/ss-01/BC-1.08.002.md` |
| 14 | `.factory/specs/behavioral-contracts/ss-04/BC-4.04.004.md` |
| 15 | `.factory/specs/behavioral-contracts/ss-04/BC-4.05.004.md` |
| 16 | `.factory/specs/behavioral-contracts/ss-04/BC-4.07.003.md` |
| 17 | `.factory/specs/behavioral-contracts/ss-04/BC-4.08.002.md` |
| 18 | `.factory/stories/S-15.01-plugin-async-semantics.md` |
| 19 | `.factory/stories/epics/E-15-plugin-async-semantics.md` |
| 20 | `.factory/cycles/v1.0-feature-plugin-async-semantics-pass-1/F-P2-001-mechanism-investigation.md` |

---

## 2. Findings

### VIOLATION — HIGH

**FINDING-001: BC-9.01.006 lines 61, 91 — "pre-commit hook" wording implies Git pre-commit**

- File: `.factory/specs/behavioral-contracts/ss-09/BC-9.01.006.md`
- Line 61: `CI lint fails; commit blocked by pre-commit hook; human must remove the 'async: true' entry`
- Line 91 (EC-002): `Pre-commit hook catches; commit blocked; CI also catches`
- Category: VIOLATION
- Reason: This BC is a session-modified F2 artifact. Both lines describe the Layer 1 enforcement mechanism as "pre-commit hook" without the BC-7.06.001 v1.3 clarification that the actual mechanism is a Claude Code PostToolUse Edit|Write hook — not a Git `.git/hooks/pre-commit` script. BC-7.06.001 v1.3 received exactly this fix; BC-9.01.006 did not receive the parallel correction. The PO had already flagged line 61 as suspect.
- Fix: Mirror BC-7.06.001 v1.3 §Postcondition 7 wording: replace "commit blocked by pre-commit hook" with "edit blocked by Claude Code PostToolUse Edit|Write hook (returning `block_intent = true`, exit code 2); CI also catches". Add a parenthetical note on EC-002 matching ADR-019 §Decision 4 generic-term acknowledgment.

---

**FINDING-002: VP-078 lines 50-51, 300, 441 — "pre-commit hook" wording; line 50 says "bash script or bats test" as enforcement mechanism**

- File: `.factory/specs/verification-properties/VP-078.md`
- Line 50-51: `Pre-commit — a lint hook (bash script or bats test) scans hooks-registry.toml and fails the commit if any entry violates the invariant.`
- Line 300: `Trigger Point: CI on every PR; pre-commit hook when hooks-registry.toml changes.`
- Line 441: `Pre-commit hook — runs before every commit that touches hooks-registry.toml or any hooks/*.sh file.`
- Category: VIOLATION
- Reason: VP-078 was authored this session to verify the invariant that S-15.01 enforces. S-15.01 v1.3 specifies the enforcement plugin as a **native WASM plugin** (`lint-registry-async-invariant.wasm`), not a "bash script". VP-078 line 50-51 still says "bash script or bats test" as the pre-commit mechanism — this contradicts S-15.01 v1.3 AC-007 and the WASM rule. Lines 300 and 441 use "pre-commit hook" without the PostToolUse clarification, inconsistent with BC-7.06.001 v1.3.
- Fix: Line 50-51: change to "Pre-commit (edit-time) — a Claude Code PostToolUse Edit|Write WASM plugin (`lint-registry-async-invariant.wasm`) scans `hooks-registry.toml` and blocks the edit if any entry violates the invariant." Lines 300 and 441: replace "pre-commit hook" with "Claude Code PostToolUse Edit|Write hook (native WASM plugin)" matching BC-7.06.001 v1.3 wording pattern.

---

### TEST_FIXTURE — MEDIUM (ambiguous)

**FINDING-003: VP-078 and VP-079 test fixtures use `legacy-bash-adapter.wasm` + `script_path`**

- Files: `VP-078.md` (lines 150, 191, 216, 236, 331, 353, 375, 401), `VP-079.md` (lines 194, 205, 243, 288, 346, 357, 407, 418)
- Pattern: `plugin = "hook-plugins/legacy-bash-adapter.wasm"` + `script_path = "test-fixtures/exit0.sh"` (or exit2.sh, sleep60.sh, bad.sh, blocking.sh, telemetry.sh)
- Category: TEST_FIXTURE — see recommendation below
- Reason: These are bats test harness fixtures that inject controlled plugin behaviors (immediate exit 0, exit 2, slow exit) to test dispatcher routing logic. They are not "shipped plugins" — they are test infrastructure that exercises the dispatcher's handling of plugin output codes. However, they do introduce new reliance on legacy-bash-adapter in session-new artifacts.

**Recommendation (call made):** Keep as-is with an explicit annotation. Rationale:

1. The test fixtures test the DISPATCHER (the property under verification is dispatcher async-routing correctness), not the plugins themselves. The fixture .sh files are stub behaviors, not governance logic.
2. Requiring WASM test-fixture modules would mean writing Rust crates that just `exit 0` or `sleep` — zero behavioral value over a 2-line bash stub, with heavy compilation overhead per test scenario.
3. The legacy-bash-adapter itself is a WASM plugin. Using it to run a 2-line bash stub in test context is materially different from shipping new governance logic in bash.
4. The bats harness itself (infrastructure) is categorically acceptable per audit boundaries.
5. These fixtures simulate existing plugin behavior. The real production plugin (`lint-registry-async-invariant.wasm`) is native WASM — VP-078 verifies IT, using the fixtures as controllable stand-ins for other plugins in the registry.

**Condition for acceptability:** Add a comment in each VP's fixture section stating: "Test-infrastructure only. These fixtures use `legacy-bash-adapter.wasm` to inject controlled exit codes into the dispatcher under test. This is test infrastructure, not a new shipped plugin. The production plugin verified by this VP is a native WASM plugin per S-15.01 AC-007."

This is an **informational annotation**, not a structural change. Does not affect convergence.

---

### HISTORICAL — ACCEPTABLE

| Finding | File | Lines | Reason Acceptable |
|---------|------|-------|------------------|
| ADR-019 "pre-commit hook" in §Decision 4 | ADR-019 | 133-135 | PO already verified: generic/conceptual term, not Git pre-commit. BC-7.06.001 v1.3 explicitly cites and resolves this. No ADR amendment needed. |
| BC-7.06.001 "pre-commit" references | BC-7.06.001 | 57, 70, 177-189 | All resolved in v1.3 with explicit PostToolUse clarification and WASM migration note. Compliant. |
| S-15.01 amendment log references "bash via legacy-bash-adapter" (v1.1, v1.2 history) | S-15.01 | 830-836 | Changelog entries documenting what was REMOVED. Not live spec. |
| F-P2-001 investigation Option C description | F-P2-001 | passim | Investigation artifact documenting options considered. Option C was REJECTED. Historical record. |
| VP-079 line 461 "Pre-commit hook (optional)" | VP-079 | 461 | Describes an optional trigger, explicitly marked "(optional)". Acceptable as an informational note about when the suite may also run. |
| BC-4.08.002 "pre-commit cleanup" in changelog | BC-4.08.002 | 122 | Changelog entry only; refers to a state-manager cleanup action, not a plugin. |
| E-15 "Porting verify-git-push.sh to native WASM — stays bash per E-8 D-1" | E-15 | 148 | References a deferral decision for an EXISTING bash hook migration — explicitly categorized under migration epics. |
| E-15 "hooks/*.sh or CI pipeline" | E-15 | 243 | Column in an AC matrix describing WHERE existing hooks live; does not specify a new bash plugin. |
| domain-spec/invariants.md legacy-bash-adapter | invariants.md | 124 | DI-016 justification section; documents existing config-isolation invariant for the legacy adapter. Historical. |

---

## 3. Recommendations

### R-001 (HIGH — BC-9.01.006, FINDING-001)
Amend BC-9.01.006 to v1.2 (informational wording fix):
- Line 61: Replace `commit blocked by pre-commit hook` with `edit blocked by Claude Code PostToolUse Edit|Write hook (returning block_intent = true, exit code 2); CI also catches`
- Line 91 (EC-002): Replace `Pre-commit hook catches; commit blocked` with `Claude Code PostToolUse Edit|Write hook catches; edit blocked`
- Add a footnote or inline note matching BC-7.06.001 v1.3 §Amendment §7.06.001-v1.3: "ADR-019 §Decision 4 uses 'pre-commit hook' as a generic conceptual label; the actual mechanism is the PostToolUse hook lifecycle."

### R-002 (HIGH — VP-078, FINDING-002)
Amend VP-078 to v1.8 (wording alignment with S-15.01 v1.3):
- Line 50-51: Replace "bash script or bats test" with "Claude Code PostToolUse Edit|Write WASM plugin (`lint-registry-async-invariant.wasm`)"
- Lines 300, 441: Replace "pre-commit hook" with "Claude Code PostToolUse Edit|Write hook (native WASM plugin)"

### R-003 (MEDIUM — VP-078 + VP-079 test fixtures, FINDING-003)
Add an annotated comment block to the fixture sections in VP-078 and VP-079 clarifying that `legacy-bash-adapter.wasm` usage is test infrastructure, not a new shipped plugin. No structural changes; no re-verification needed.

---

## 4. Open Questions for User

**Q-001 (FINDING-003 test fixtures):** The audit recommends keeping bash test fixtures with an annotation rather than writing native WASM test stubs. Do you agree with this call? The alternative (WASM test modules) would require per-scenario Rust crates compiled to .wasm solely to return exit codes — no behavioral logic, significant build overhead. The production plugin is already native WASM; only the test stubs use legacy-bash-adapter.

**Q-002 (VP-078 annotation vs. amendment):** For R-003, do you prefer a short comment block inline with the fixture TOML, or a dedicated "Test Fixture Rationale" subsection? Either is a minor informational-only change.

---

## 5. Convergence Impact Assessment

**F2 convergence status: NOT affected for any HIGH finding.**

- BC-9.01.006 amendment (R-001): Wording-only clarification, no postcondition logic changes. Precedent: BC-7.06.001 v1.3 was classified as an informational wording fix that did not trigger re-convergence. BC-9.01.006 amendment follows the same pattern — bump to v1.2, log in changelog, no re-convergence pass needed.
- VP-078 amendment (R-002): Aligns description of the enforcement mechanism with what S-15.01 v1.3 AC-007 already specifies. The property being verified is unchanged; only the description of WHERE layer 1 runs is corrected. No proof strategy changes. Bump to v1.8; no re-convergence.
- VP-078/VP-079 test fixture annotation (R-003): Informational addition only. Zero impact on convergence.

**Conclusion:** All three fixes are informational clarifications consistent with already-converged decisions. F2 convergence at pass-10 (commit 3568657) is preserved.

---

## 6. Top 3 Violations Ranked

| Rank | Finding | File | Lines | Severity | Fix Effort |
|------|---------|------|-------|----------|-----------|
| 1 | FINDING-001: BC-9.01.006 "pre-commit hook" — PO-flagged; implies Git hook mechanism for the wrong layer | BC-9.01.006 | 61, 91 | HIGH | Low — 2 line rewrites + footnote |
| 2 | FINDING-002: VP-078 "bash script or bats test" as pre-commit enforcement — contradicts S-15.01 v1.3 WASM directive | VP-078 | 50-51, 300, 441 | HIGH | Low — 3 targeted rewrites |
| 3 | FINDING-003: VP-078 + VP-079 test fixtures using legacy-bash-adapter — ambiguous but recommended acceptable as test infrastructure | VP-078, VP-079 | multiple | MEDIUM | Annotation only |

---

## Summary

| Metric | Count |
|--------|-------|
| Artifacts audited | 20 |
| HIGH violations found | 2 (BC-9.01.006, VP-078) |
| MEDIUM test-fixture questions | 1 (VP-078 + VP-079 combined) |
| HISTORICAL / acceptable hits | 9 distinct items |
| F2 convergence impact | None — all fixes are informational v-bumps |
| Recommended test-fixture path | Keep bash stubs with annotation (not WASM stubs) |
