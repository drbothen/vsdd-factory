---
document_type: adversary-review
cycle: v1.0-feature-plugin-async-semantics-pass-1
pass: 18
verdict: HIGH
adr_013_clock_after_pass: 0_of_3
producer: adversary
timestamp: 2026-05-08T18:00:00Z
---

# F5 Pass-18 Adversary Review

## Verdict

**HIGH** — 1 HIGH (sibling hook absolute-path bug), 3 MEDIUM (stale prose-form line refs in BC bodies, BC-INDEX/VP-INDEX changelog gap, TD-031 status not updated for cc5a016b), 3 LOW. ADR-013 clock RESETS to **0_of_3**.

## Trajectory

17→15→6→5→0→2→5→1→4→2→2→4→4→5→7→5→4→**HIGH**

## Findings

### F-P18-001 [HIGH] Sibling hook `validate-artifact-path` has identical absolute-path bug; cc5a016b only fixed `validate-stable-anchors`

**Category:** impl, sibling drift, security/correctness, [process-gap]

**Evidence:**
- `/Users/jmagady/Dev/vsdd-factory/crates/hook-plugins/validate-artifact-path/src/lib.rs:153` — `if !path.starts_with(".factory/") { return MatchResult::NoMatch; }`
- `/Users/jmagady/Dev/vsdd-factory/crates/hook-plugins/validate-artifact-path/src/lib.rs:398` — `if !file_path.starts_with(".factory/") { return HookResult::Continue; }`
- `/Users/jmagady/Dev/vsdd-factory/crates/hook-plugins/validate-stable-anchors/src/lib.rs:578-582` — fixed by cc5a016b to also accept `contains("/.factory/specs/")`
- `/Users/jmagady/Dev/vsdd-factory/plugins/vsdd-factory/hooks-registry.toml:840-847` — validate-artifact-path is REGISTERED + ACTIVE (PreToolUse, Write|Edit, priority 150, on_error=continue)

**Impact:** Claude Code regularly sends absolute paths. When an absolute path is sent to validate-artifact-path:
1. `matches_canonical(path, ...)` returns `MatchResult::NoMatch` for ALL artifact-path violations under absolute-path Writes/Edits (silent bypass).
2. `hook_logic` early-returns `HookResult::Continue` before ever loading the registry (silent bypass).

This is the EXACT same bug class that cc5a016b fixed in validate-stable-anchors. The fix burst found-and-fixed-one-but-not-the-sibling.

**Confidence:** HIGH

**Recommended fix:** Apply the same `starts_with(".factory/") || contains("/.factory/")` pattern to both call sites in `validate-artifact-path/src/lib.rs`. Add 2 absolute-path tests mirroring the validate-stable-anchors fix.

---

### F-P18-002 [MEDIUM] Stale prose-form line references survived TD-VSDD-091 sweep in BC-1.05.035 body (Edge Cases EC-007 + Test Vector row 6) and BC-2.02.011 (Architecture Anchors)

**Category:** spec, semantic anchoring (POLICY 4)

**Evidence:**
- BC-1.05.035.md:95 — "Implementer feeds canonical to allow-check **at line 152** but leaves raw `cmd` in `execute_bounded(...)` call **at line 173**"
- BC-1.05.035.md:113 — "allow-check **at line 152** sees canonical resolving … attacker swaps symlink between **line 152** and **line 230** to point at `/tmp/attacker` … `execute_bounded(canonical_path.as_str(), args, ...)` **at line 173** → `Command::new(canonical_path)` **at line 230**"
- BC-1.05.036.md:66 — "BC-1.05.035 Postcondition 1 propagating the canonical path through to `Command::new(...)` **at line 230**"
- BC-2.02.011.md:100 — "(new `pub fn write_file` **after line 187**, SDK wrapper)"

**Impact:** TD-VSDD-091's stated purpose is to prevent line-number drift from corrupting spec correctness. Prose-form references like "at line 152" are exactly this drift vector — the migration-sweep partially completed (handled `file.ext:NNN`) but missed semantically-equivalent prose.

**Confidence:** HIGH

**Recommended fix:** Replace prose `at line NNN` with stable anchors. Either extend the validate-stable-anchors hook regex to optionally catch `\bat line \d+\b` prose, OR add this pattern to the manual sweep checklist (POLICY 4 enforcement extension).

---

### F-P18-003 [MEDIUM] BC-INDEX and VP-INDEX changelogs do not record the 6-chunk TD-VSDD-091 sweep

**Category:** spec, propagation (POLICY 9 for VPs), [process-gap]

**Evidence:**
- BC-INDEX.md top changelog entry is `F5 fix-burst-16` (BC-3.08.001 v1.10→v1.11). BC-INDEX version stayed at v1.41.
- VP-INDEX.md top entry is `F5 fix-burst-16` (VP-079 v1.15→v1.16). VP-INDEX version stayed at v1.27.
- VP-077 frontmatter `version: "v1.11"` (chunk 3 bump). VP-INDEX line 182 row description still says "v1.10: F5 fix-burst-4" — stale.
- ~50+ BC files version-bumped, 5+ VP files version-bumped, none recorded in indexes.

**Impact:** Future readers cannot reconstruct the sweep's scope from the index alone.

**Confidence:** HIGH

**Recommended fix:** Add aggregated changelog entries to BC-INDEX and VP-INDEX. Update VP-077 row description.

---

### F-P18-004 [MEDIUM] TD-031 entry not updated to reflect cc5a016b absolute-path bug fix; status text overstates enforcement validity

**Category:** process, tech-debt-register currency, [process-gap]

**Evidence:**
- tech-debt-register.md TD-031: "ENFORCEMENT IMPLEMENTED 2026-05-08 fix-burst-16 (bb661eaa)" + "58/58 tests pass" — but no mention of the cc5a016b absolute-path bug discovery + fix.
- Test count claimed: 58/58. Actual current count (post-cc5a016b): 62/62.

**Impact:** Tech-debt register undersells the enforcement story; reader inferring hook-readiness sees "ENFORCEMENT IMPLEMENTED" but not that the hook had a critical absolute-path silent-bypass bug fixed only after the chunks landed.

**Confidence:** MEDIUM

**Recommended fix:** Add cc5a016b validation entry to TD-031.

---

### F-P18-005 [LOW] Test asymmetry: `test_is_spec_target_absolute_path_matches` uses concrete user-specific path

**Category:** test, portability

**Evidence:** validate-stable-anchors/src/tests.rs:771 — `assert!(is_spec_target("/Users/jmagady/Dev/vsdd-factory/.factory/specs/foo.md"));`

**Recommended fix:** Replace with abstract path: `/abs/project/.factory/specs/foo.md`.

---

### F-P18-006 [LOW] `scan_spec` SitesFence zone restoration assumes Changelog parent context (over-exemption risk)

**Category:** impl, defensive correctness

**Evidence:** validate-stable-anchors/src/lib.rs:302-310 — when SitesFence closes, `zone = ExemptZone::Changelog;` regardless of parent zone.

**Recommended fix:** Track entry zone when SitesFence is activated; restore on fence close.

---

### F-P18-007 [LOW] `is_spec_target` has no case-folding for `.MD` extension or Windows backslash paths

**Category:** impl, robustness

**Recommended fix:** Defer; document as known-limitation.

---

## Process-gap findings (tagged for codification follow-up)

- **[process-gap]** F-P18-001 — Same-type hook-plugin siblings need a structural pattern check during fix-burst close.
- **[process-gap]** F-P18-002 — Validate-stable-anchors hook regex catches `<word>.<ext>:<digit>` syntax only; `\bat line \d+\b` prose form is structurally identical drift but bypasses the lint.
- **[process-gap]** F-P18-003 — Mass-sweep policy: when a fix-burst touches >5 BC/VP files, the burst MUST update BC-INDEX/VP-INDEX changelog with an aggregated entry.
- **[process-gap]** F-P18-004 — Tech-debt-register currency: when a TD entry says "ENFORCEMENT IMPLEMENTED", any subsequent enforcement bug fix must update the entry.

## Notable observations

- The 6-chunk migration is mostly correct: spot-checked anchors all resolve to real symbols. POLICY 4 semantic-anchoring integrity is maintained for the SYNTACTIC migration scope.
- The hook test count (62/62 post-cc5a016b) and confidence in pure unit-test exercise are strong.
- VP-077 row description in VP-INDEX line 182 reads "v1.10: F5 fix-burst-4" but VP-077 frontmatter is now v1.11.

## Convergence assessment

This pass identifies **1 HIGH (F-P18-001)** confirmed by file:line evidence: the `validate-artifact-path` sibling hook has the identical absolute-path-bypass bug. Plus 3 MEDIUM and 3 LOW. ADR-013 clock RESETS to 0_of_3.

**Novelty:** HIGH. F-P18-001 (sibling hook bug), F-P18-002 (prose-form drift class), F-P18-003 (BC-INDEX/VP-INDEX missing the largest sweep entry), and F-P18-004 (TD register currency) are all novel.
