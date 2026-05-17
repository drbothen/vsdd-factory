---
document_type: adversary-pass-report
level: ops
title: "S-15.09 LOCAL Adversary Cascade — Pass 1"
producer: adversary
timestamp: 2026-05-17T00:00:00Z
phase: per-story-step-4-local-adversary-cascade
story: S-15.09
pass: 1
verdict: HIGH
finding_count: { critical: 1, high: 2, medium: 2, low: 3, nitpick: 2, process_gap: 0 }
streak_3_clean: "0/3"
---

# S-15.09 LOCAL Adversary Cascade — Pass 1

## Part A — Findings

### F-P1-001 — CRITICAL — correctness / spec-implementation drift — banner-wc marker pattern does not match real STATE.md prose

- **Severity:** CRITICAL
- **Category:** correctness / silent inert validator (META-LEVEL-class)
- **Location:** `crates/hook-plugins/validate-state-structure/src/lib.rs:104`
- **Evidence:**
  ```rust
  let marker = " lines (wc-l)";
  ```
  Production STATE.md `/Users/jmagady/Dev/vsdd-factory/.factory/STATE.md` line 26 has many occurrences of `" lines (wc-l;"` and `" lines (wc-l."` variants but **zero** occurrences of the literal pattern `" lines (wc-l)"` (with closing-paren as next byte). Confirmed via `Grep ' lines \(wc-l\)' .factory/STATE.md` → **No matches found**. Orchestrator independently verified: `grep -c 'lines (wc-l)' .factory/STATE.md` = 0; 22 occurrences of `lines (wc-l;` exist.
- **Issue:** The hand-rolled marker scan requires the literal 13-byte sequence `" lines (wc-l)"` including the closing `)`. The real production STATE.md banner uses idiomatic prose forms (`"310 lines (wc-l; net..."`, `"...399 lines at pass-67 Commit E (wc-l));..."`) where `wc-l` is followed by `;`, `.`, or `))`, not the bare `)` the marker requires. Consequence: `extract_banner_line_count` returns `None` on the real STATE.md → `validate_banner_wc` short-circuits via `?` and returns `None` → **no banner_wc violation ever fires against the artifact the hook was built to validate**. This is a META-LEVEL-24-class regression-detector false-green pattern (POL-11 / TD-VSDD-057). The fixtures the implementer hand-crafted (`28 lines (wc-l).`) include the literal `)` and thus the unit/bats tests succeed — but the entire validator is silently inert in production. Closes D-421(c)..D-442(d) in nameplate only.
- **Recommendation:** Replace the literal-string marker with a tolerant pattern. Either (a) scan for `lines (wc-l` then accept any non-digit terminator that follows (`)`, `;`, `.`, `,`), or (b) per real-STATE.md convention, anchor on the LAST `NNN lines (wc-l)` occurrence in the SIZE BUDGET banner block specifically (the canonical trailing claim that drives banner enforcement is the last one in the line-growth tracker — currently `428 lines (wc-l)` per line 26's tail). Add a fixture mirroring real STATE.md prose (multiple `NNN lines (wc-l;` interim entries plus a trailing canonical) and assert validation fires correctly.

### F-P1-002 — HIGH — spec-implementation gap — absent banner produces no banner_wc violation, contradicting EC-014

- **Severity:** HIGH
- **Category:** spec-implementation drift / missing-postcondition
- **Location:** `crates/hook-plugins/validate-state-structure/src/lib.rs:155-170` (`validate_banner_wc`)
- **Evidence:**
  ```rust
  pub fn validate_banner_wc(content: &str) -> Option<Violation> {
      let claimed = extract_banner_line_count(content)?;   // None ⇒ early return None
      ...
  }
  ```
  Doc comment at line 149-151 acknowledges this: "Returns `None` if the banner is absent (caller decides how to handle this case; in the current spec, a missing banner means no banner wc-l violation..."
- **Issue:** BC-5.39.005 EC-014 (`.factory/specs/behavioral-contracts/ss-05/BC-5.39.005.md:136`) requires: "STATE.md is newly created with empty content (no banner, no trajectory-tail) | BlockWithFix: **both line-count and trajectory-tail violations** (no banner line found; no tail found)". Implementation only fires the tail violation (via `validate_trajectory_tail` returning the no-tail case). The doc comment explicitly states the implementation chose the opposite contract — this is a direct, acknowledged spec violation. No fixture or test covers EC-014 (`fail-banner-wc-off-by-one` covers wc-l divergence but not banner-absence).
- **Recommendation:** Change `validate_banner_wc` to return a `Violation` when `extract_banner_line_count` is `None` (description: "no banner line found; STATE.md MUST include a SIZE BUDGET banner with `N lines (wc-l)` claim per D-421(c)..D-442(d)"). Add fixture `fail-no-banner/` + bats test asserting both banner and tail violations are enumerated on empty/headerless STATE.md.

### F-P1-003 — HIGH — operational defect — registry priority collision with validate-artifact-path

- **Severity:** HIGH
- **Category:** policy-violation / operational
- **Location:** `plugins/vsdd-factory/hooks-registry.toml:910` and `:846`
- **Evidence:**
  - Line 846: `validate-artifact-path` … `priority = 150`
  - Line 910: `validate-state-structure` … `priority = 150`
- **Issue:** Two distinct hook entries share `priority = 150`. The story spec T-9 explicitly directed: "Insert after the `validate-burst-log` block (PostToolUse section, priority **one higher than validate-burst-log's assigned priority** — check current value before inserting)". validate-burst-log is at 152 (line 865), validate-index-cite-refresh at 151 (line 884) — the canonical convention is monotone-ascending. The next slot is 153. The chosen value 150 (a) violates the story's own directive, (b) collides with validate-artifact-path (different event family, but still a shared priority within PreToolUse=150 vs PostToolUse=150 — note validate-artifact-path is PreToolUse but the duplicate value is a maintenance-confusion smell), and (c) misorders relative to validate-burst-log and validate-index-cite-refresh which are the natural same-event siblings.
- **Recommendation:** Set `validate-state-structure.priority = 153` per the story spec directive and the monotone-ascending sibling convention.

### F-P1-004 — MEDIUM — semantic-anchoring drift — subsystem narrative name mismatches ARCH-INDEX

- **Severity:** MEDIUM
- **Category:** semantic-anchoring / POLICY 6 violation
- **Location:** `.factory/stories/S-15.09-validate-state-structure-phase-1.md:167` and BC body in `.factory/specs/behavioral-contracts/ss-05/BC-5.39.005.md` traceability
- **Evidence:**
  - Story body line 167: `**Subsystem:** SS-05 (Engine Governance — per-story adversary workflow and discipline gate hooks)`
  - ARCH-INDEX line 247 + 293: `SS-05 Pipeline Orchestration` / `SS-05 | Pipeline Orchestration`
- **Issue:** POLICY 6 (`architecture_is_subsystem_name_source_of_truth`) requires subsystem names cited in BCs/stories match ARCH-INDEX exactly. ARCH-INDEX canonical name for SS-05 is "Pipeline Orchestration", not "Engine Governance". Same drift propagates into the BC's capability-anchor justification ("E-12 Engine Governance") — these are spec-narrative labels that contradict the source of truth.
- **Recommendation:** Either (a) update the story/BC narratives to use "Pipeline Orchestration" as the SS-05 label and rename the capability sub-area to fit, or (b) propose an ARCH-INDEX amendment via architect routing to rename SS-05.

### F-P1-005 — MEDIUM — frontmatter-body coherence gap — story frontmatter `subsystems: []` empty while body asserts SS-05

- **Severity:** MEDIUM
- **Category:** frontmatter-body drift / POLICY 4
- **Location:** `.factory/stories/S-15.09-validate-state-structure-phase-1.md:45`
- **Evidence:**
  ```yaml
  subsystems: []
  ```
  Body line 167 asserts subsystem SS-05.
- **Issue:** Story frontmatter `subsystems:` array is empty while the body BC table and narrative explicitly anchor SS-05. Frontmatter is the machine-readable scope source; emptiness here means downstream tooling (consistency-validator, hook automation) cannot detect the SS-05 anchor. POLICY 4 (semantic_anchoring_integrity) — anchors must be both semantic AND syntactic.
- **Recommendation:** Populate `subsystems: ["SS-05"]` in story frontmatter to match the body assertion.

### F-P1-006 — LOW — weak test assertions in fail-all-three.bats (POLICY 11 adjacent)

- **Severity:** LOW
- **Category:** test-quality / POLICY 11 adjacent
- **Location:** `plugins/vsdd-factory/tests/validate-state-structure/fail-all-three.bats:111-114`
- **Evidence:**
  ```bash
  [[ "$output" == *"D-446"* ]] || [[ "$output" == *"dual-margin"* ]] || [[ "$output" == *"margin"* ]]
  [[ "$output" == *"D-433"* ]] || [[ "$output" == *"trajectory"* ]] || [[ "$output" == *"tail"* ]]
  [[ "$output" == *"25"* ]] || [[ "$output" == *"28"* ]] || [[ "$output" == *"banner"* ]]
  ```
- **Issue:** The disjunctive `||` makes each assertion trivially satisfied by single-token substrings (`"margin"` appears in many unrelated stderr lines; `"tail"` too). A single-violation block message would pass these three assertions and the test would falsely claim "all three violations enumerated". This weakens the AC-6 gate.
- **Recommendation:** Require explicit conjunctive markers for each violation class (e.g., `D-421` or `D-422` for banner_wc, `D-446` for dual-margin, `D-433` for trajectory-tail), AND require the block message to contain the literal `3 violation(s)` count produced by `emit_block` (line 387: `"validate-state-structure: {} violation(s)"`).

### F-P1-007 — LOW — extract_trajectory_tail_line returns FIRST arrow-bearing line; banner block prose could contain `→N` in real STATE.md

- **Severity:** LOW
- **Category:** robustness / structural fragility
- **Location:** `crates/hook-plugins/validate-state-structure/src/lib.rs:270-281`
- **Evidence:**
  ```rust
  for line in content.split('\n') {
      let trimmed = line.trim_end_matches('\r').trim();
      if contains_arrow_digit_sequence(trimmed) {
          return Some(trimmed.to_string());
      }
  }
  ```
  Real STATE.md prose may contain `→N` examples (e.g., trajectory snapshots, prose like `9→9→9→9→9→9` in the line-growth tracker), which would be matched before the actual canonical Convergence Status trajectory tail.
- **Issue:** "First line containing arrow+digit" is not a robust definition of "trajectory-tail line". The canonical trajectory tail in STATE.md is in a specific section (e.g., `## Convergence Status` or a named row). A multi-arrow narrative could trigger as the "tail" with a different count than the intended canonical tail.
- **Recommendation:** Anchor extraction on a section heading (`## Convergence Status` or similar canonical marker) and parse the trajectory tail within that section, OR scan ALL lines and apply the count-4 invariant globally with deduplication. Document the chosen extraction rule explicitly in the BC.

### F-P1-008 — LOW — integration-production-registry.bats — failed extraction does not halt test

- **Severity:** LOW
- **Category:** test gate weakness
- **Location:** `plugins/vsdd-factory/tests/validate-state-structure/integration-production-registry.bats:60-100`
- **Evidence:**
  ```bash
  _write_production_registry() {
      local prod_path_allow=$(awk '...')
      if [ -z "$prod_path_allow" ]; then
          echo "FAIL: could not extract..." >&2
          return 1
      fi
      ...
  }
  ```
  In the `@test` body: `_write_production_registry` is called but its return value is never checked.
- **Issue:** If awk extraction returns empty (e.g., production registry entry removed/renamed), the helper returns 1 but the bats test continues. The heredoc write would produce a malformed TOML, and Scenario A might still pass (exit 0 from a misconfigured registry could happen via fail-open). The "production-registry capability shape regression test" loses its load-bearing property.
- **Recommendation:** Add `_write_production_registry || fail "production registry extraction failed"` (or `|| return 1` with explicit bats failure semantics) before the dispatcher invocation.

### F-P1-009 — NITPICK — count_arrow_digit_matches uses byte-level loop without char-boundary stepping

- **Severity:** NITPICK
- **Category:** correctness/robustness (defensive)
- **Location:** `crates/hook-plugins/validate-state-structure/src/lib.rs:314-342`
- **Issue:** The function iterates byte-by-byte (`i += 1`) and matches the 3-byte arrow sequence on raw bytes. UTF-8 self-synchronization makes a false match impossible in valid UTF-8 (no continuation byte is 0xE2/0x86/0x92 as first byte), but the code does not document this safety property nor assert it. A reader can't tell whether the implementation is correct-by-luck or correct-by-design.
- **Recommendation:** Add a comment justifying byte-level scanning is safe due to UTF-8 first-byte invariants (0xE2 is a leading byte; 0x86 and 0x92 are continuation bytes that never start a code point). Alternatively iterate `s.char_indices()` for explicit correctness.

### F-P1-010 — NITPICK — extract_banner_block conflates "first HTML comment" with "SIZE BUDGET banner"

- **Severity:** NITPICK
- **Category:** robustness
- **Location:** `crates/hook-plugins/validate-state-structure/src/lib.rs:237-255`
- **Issue:** `extract_banner_block` returns the contents of the FIRST `<!-- ... -->` HTML comment block. The spec narrative repeatedly refers to "the SIZE BUDGET banner" specifically. If STATE.md ever grows a header HTML comment (license note, edit warning, table-of-contents comment), the dual-margin check would be applied to the wrong block.
- **Recommendation:** Anchor extraction on a marker phrase like `STATE.md SIZE BUDGET` (which is present in all current fixtures and real STATE.md line 24). Scan for `<!--` ... `STATE.md SIZE BUDGET` ... `-->` as the specific block of interest.

## Part B — Production-Grade Default Audit

- **F-P1-001** is a textbook silent-inert validator (SOUL.md #4 + production-grade default Rule 1 violation): the artifact "ships" but doesn't actually enforce its invariant in production. The fixtures hand-crafted to match the marker are a test-path-only validation — the production-path target STATE.md is structurally different. This is the highest-impact finding; closing the D-421(c)..D-442(d) sub-clauses in nameplate while producing zero enforcement against the real artifact is a META-LEVEL-class regression-detector false-green (POL-11/TD-VSDD-057). Implementer self-disclosure absent — production-grade default Rule 4 applies: adversary independently verified by greping real STATE.md.
- **F-P1-002** is a doc-comment-acknowledged spec deviation. The implementer wrote a comment saying "the current spec says X, but we chose Y" — that is exactly the "pending architect review" anti-pattern Rule 6 forbids, except worse: it's silent in code instead of surfaced for adjudication. Should be fixed in-scope.
- **F-P1-003** registry priority defect: the story spec explicitly directed the value and the implementer chose otherwise. This is a sibling-site sweep gap (TD-VSDD-060) — implementer did not verify the priority value before insertion. Should be fixed in-scope.
- **F-P1-004 / F-P1-005** are sibling drift items. POLICY 6 + production-grade default Rule 6 (mechanical questions answered in-scope) mandate adopting ARCH-INDEX canonical "Pipeline Orchestration" everywhere. F-P1-005 frontmatter is trivially fixable.

## Part C — Self-Application Audit (META-LEVEL)

This report itself uses:
- Specific file:line citations with verbatim code excerpts.
- Verifiable Grep invocations (independently re-run by orchestrator: 22 occurrences of `lines (wc-l;`, 0 of `lines (wc-l)`).
- No pseudocode; every finding has captured-stdout-class evidence (Grep counts, Glob results, Read line numbers).

A residual META-LEVEL concern: F-P1-001 was discoverable only because the adversary independently grepped the real STATE.md against the marker string. A future regression of this kind would survive any number of clean adversary passes if the adversary's prompt did not specifically direct them to verify marker patterns against real artifacts. This argues for a new review-axis codification: **"validators that scan production text-targets must include a fixture that mirrors the production target's prose conventions, not the prose conventions the implementer hand-crafted to make the scanner succeed."** (Process-gap candidate: PG-S-15.09-real-prose-fixture-discipline.)

## Verdict & next step

- Verdict: **HIGH** (1 CRITICAL + 2 HIGH + 2 MEDIUM + 3 LOW + 2 NITPICK)
- Streak: **0/3** (CRITICAL + HIGH findings reset streak)
- Mandatory fix-burst required before adversary pass-2.

## Fix-burst routing (orchestrator routed; complete at time of persistence)

- implementer: F-P1-001, F-P1-002, F-P1-003, F-P1-007, F-P1-009, F-P1-010 — closed at branch HEAD `e8ebbfa8` (2 commits + real-STATE.md integration test; 19/19 S-15.09 bats green; 4-gate PASS)
- test-writer: F-P1-006, F-P1-008 — closed at branch HEAD `164bbbae` (2 commits; conjunctive assertions + helper halt-on-failure)
- story-writer: F-P1-004, F-P1-005 — closed at factory-artifacts `d3031b44` (POLICY 6 narrative alignment + frontmatter populated + POLICY 14/17 tripartite-parity)
- state-manager: persist this report (this commit)
- Adversary pass-2 dispatched after this persistence commit.

## Closure note (real-STATE.md F-P1-001 structural verification)

Implementer's real-STATE.md integration test `test_BC_5_39_005_f_p1_001_real_state_md_banner_wc_passes` reads `/Users/jmagady/Dev/vsdd-factory/.factory/STATE.md` (428 lines per `wc -l`; last `wc-l;` interim claim = 428), tolerant marker `extract_banner_line_count` returns `Some(428)`, `count_newlines` returns `428` — no violation. F-P1-001 closed STRUCTURALLY: the validator now actually parses the real target prose. Tolerant marker accepts `;`, `.`, `,`, `)` terminators per 5 new unit tests.
