---
document_type: adversarial-review
level: adversary
version: "1.0"
status: complete
story_id: S-21.07
pass: 1
cycle: v1.0-brownfield-backfill
producer: adversary (3 scoped agents)
timestamp: "2026-08-03T00:00:00Z"
phase: 3
inputs:
  - stories/S-21.07-story-validate-cross-site-correspondence.md
  - specs/behavioral-contracts/ss-05/BC-5.39.010.md
  - policies.yaml
input-hash: "8ba2a75"
traces_to: "BC-5.39.010 v1.2; S-21.07 v1.1"
reviewed_head: "db381c1b"
base: "948f0fb1"
story_version: "1.1"
bc: "BC-5.39.010 v1.2"
verdict: NOT-CLEAN
novelty: high
findings_count: 47
severity_breakdown: "B7/H19/M13/L8"
dedup_pairs:
  - "F-S2107-P1B-011 ↔ F-S2107-P1C-003"
  - "F-S2107-P1B-015 ↔ F-S2107-P1C-013"
  - "F-S2107-P1-008 ↔ F-S2107-P1C-018"
streak: "0/3"
previous_review: null
trajectory_append: 47
model_override: false
model_resolved: "claude-sonnet-4-6"
dispatch_shape: "COMPOSITE — union of three scoped fresh-context reviewers (Scope A/B/C); synchronous dispatch after 5 background failures"
adr033_deviation: "ADR-033 cross-family limitation — composite 3-scope dispatch; all scoped agents ran on Claude, same family as authoring agents; fresh context + information asymmetry intact per scope, cross-family independence absent"
asymmetry_enforcement: "Pass-1: no prior S-21.07 adversary pass. Each scoped agent was fresh-context with no access to other scopes' outputs. Source perimeter: story v1.1 + BC-5.39.010 v1.2 + implementation at db381c1b."
policy22_note: "POLICY 22 SUBAGENT-REPORT-FIDELITY active: orchestrator independently re-derived load-bearing BLOCKER claims via literal shell before transcription; verified items marked [ORCH-VERIFIED]. Finding IDs, severities, policy cites, and anchors preserved exactly as given."
persistence_note: "Persisted in record burst (orchestrator-transcribed from three synchronous scoped adversary subagents). Verbatim from authoritative source per POLICY 22."
---

# Adversary Pass 1 — S-21.07 LOCAL cascade (BC-5.39.001)

**Reviewed HEAD:** `db381c1b` (develop base `948f0fb1`) · **Story** v1.1 · **BC-5.39.010** v1.2
**Verdict:** **NOT-CLEAN** — B7 / H19 / M13 / L8 = 47 findings (pre-dedup across three scopes) · **Streak: 0/3**

---

## DISPATCH-SHAPE DEVIATION

**Record prominently.** Pass-1 was executed as the UNION of three scoped fresh-context reviewers rather than one holistic fresh context, after five consecutive background-dispatch delivery failures (three silent idles, one confirmed `API Error: Connection closed mid-response`). Synchronous dispatch succeeded.

**Consequence:** cross-arm defects spanning two scopes may be under-detected; the orchestrator will weight cross-cutting concerns in pass-2.

**Scope split:**
- **Scope A** = Part-A closures + bats/fixtures + registry/WASM (policies 10, 11, 12, 15, 20, 21, 22)
- **Scope B** = arms A1/A2/B + dispatch + frontmatter (policies 4, 5, 6, 7, 13, 18)
- **Scope C** = arms D/E + lib/main + AC parity + spec parity (policies 1, 2, 3, 6, 8, 9, 14, 16, 17, 19)

Union covers all 22 policies.

---

## Finding ID Convention

Finding IDs: `F-S2107-P1-NNN` (Scope A), `F-S2107-P1B-NNN` (Scope B), `F-S2107-P1C-NNN` (Scope C).

---

## 1. Mandatory Provenance Disclosure

**Orchestrator-transcribed** from three fresh-context adversary subagents (no write access). Per POLICY 22 the orchestrator independently re-derived the load-bearing BLOCKER claims via literal shell before transcription; verified items are marked `[ORCH-VERIFIED]`. Finding IDs, severities, policy cites, and anchors preserved exactly as given. Do not re-classify severities. Do not add findings.

---

## 2. Part A — Closure Verification at `db381c1b`

| ID | Claim | Verdict | Evidence |
|----|-------|---------|----------|
| **A1** | Vacuous bats controls — `_assert_plugin_ran_not_crashed` exists and wired to 33 of 35 tests | **PARTIAL** ⚠ | Guard 1 matches any record bearing the plugin name; `executor.rs::emit_invoked` writes `plugin.invoked` BEFORE invocation — proves only attempted dispatch, not execution. Guard 2 checks only `plugin.crashed`; `plugin.timeout` (`cause: fuel\|epoch`) and `plugin.completed` with non-zero exit are invisible. With `on_error="continue"` a fuel-exhausted plugin satisfies both guards → exit 0 → all 14 exit-0 tests green with arm logic never executed. No mutant proves the helper fires RED. Spawns finding `F-S2107-P1-001`. |
| **A2** | False "accepted per session context" claim | **CLOSED** | No residual acceptance text; sole surviving mention is descriptive and accurate. |
| **A3** | E2 monotonicity polarity — CLOSED on code+fixtures | **CLOSED (unverified count)** | `run_arm_e2` uses `curr < prev` (non-strict, correct). Both boundary tests exist and are non-vacuous (`test_BC_5_39_010_class_e2_equal_dates_permitted`, `..._genuine_decrease_blocks`). All 22 fixtures hold ascending-or-equal dates; 8 carry an explicit equal-date pair. The "exactly three fixtures reverted" count could NOT be verified (no diff access) — recorded as unverified. Spawns spec gap `F-S2107-P1-007`. |

---

## 3. Part B — New Findings

### SCOPE A (verdict B0/H4/M4/L2)

---
**F-S2107-P1-001 — HIGH — POLICY 11+15**
**Location:** `validate-cross-site-correspondence.bats::_assert_plugin_ran_not_crashed`
**Defect:** Helper blind to `plugin.timeout` and non-zero exit; `emit_invoked` precedes invocation so presence guard proves only attempted dispatch. Fail-open on exactly the silent-skip outcome BC-5.39.010 §Gate Spec names as primary risk.
**Fix:** Assert `plugin.completed` and reject `plugin.timeout`.

---
**F-S2107-P1-002 — HIGH — POLICY 15** (D-889 substance, not its literal grep; no `&&{false;}||true` form present)
**Location:** same helper
**Defect:** No mutant-proving vector; only ever demonstrated against the now-deleted `todo!()` stub, so the proof is unreproducible.

---
**F-S2107-P1-003 — HIGH — POLICY 11 + TD-VSDD-059**
**Location:** bats `AC-019 (a)` and `AC-019 (b)`
**Defect:** Byte-equivalent duplicates of `AC-001 CONTROL` / `AC-009 CONTROL`; no cap boundary exercised. The file's own header demands a cap-boundary fixture ("The implementer must add that test") — not added. Changing `BC_INDEX_MAX_BYTES` to 4096 or `usize::MAX` leaves both green.

---
**F-S2107-P1-004 — HIGH — POLICY 11**
**Location:** five Class-D bats tests (`AC-012` ×2, `AC-013 MUTANT`, `AC-013 CONTROL`, `AC-014`)
**Defect:** Assert `exit 0` on an arm BC invariant 6 guarantees never blocks — tautological; a no-op Class D passes all five. Advisory TEXT never asserted. Mitigated by 14 `arm_d.rs` unit tests covering pure-core logic; the INTEGRATION path is unverified.
**Fix:** Assert advisory text via `host::log_warn` records in the internal log.

---
**F-S2107-P1-005 — MEDIUM — POLICY 11**
**Location:** bats `AC-020: registry entry has no fuel_cap field`
**Defect:** `awk` range exits 0 on zero matches and empty `$output` satisfies the negative assertion vacuously; sole gate on the ADR-035 §Decision 5 no-`fuel_cap` ruling.
**Fix:** Assert non-empty extraction first.

---
**F-S2107-P1-006 — MEDIUM — POLICY 15 count fidelity**
**Defect:** Bats header and helper doc-comment say "31 payload tests" twice; actual is 33 (35 total − 2 registry-grep). Reproduces the Class-C count-vs-enumeration shape inside the artifact meant to gate it.

---
**F-S2107-P1-007 — MEDIUM — CLAUDE.md Architectural Authority rule 12**
**Location:** `arm_e.rs::run_arm_e2` doc-comment and `test_..._equal_dates_permitted`
**Defect:** Assert BC-5.39.010 specifies `∀i: date[i] ≤ date[i+1]`, but the BC never states it: PC38 says only "compare lexicographically", postcondition 21 says "ascending", EC-016/EC-017 cover only strict-ascending and genuine decrease, and NO EC covers equal dates. The just-fixed polarity bug is re-derivable from the BC as written.
**Route:** product-owner → BC-5.39.010 v1.3 (state relation explicitly, reword postcondition 21 to "non-decreasing", add equal-dates EC + Canonical Test Vector row).

---
**F-S2107-P1-008 — MEDIUM `[process-gap]` — POLICY 11 class**
**Location:** `.github/workflows/ci.yml` `Stage WASM plugins for run-all.sh`
**Defect:** Lacks the count-floor guard its `cargo-host` sibling has; `run-all.sh` step does not set `CI_REQUIRE_ARTIFACTS`; `_require_artifacts` then `skip`s. Blast radius > 1 suite. Pre-existing sibling-parity gap.
**Route:** devops-engineer. (DEDUP with `F-S2107-P1C-018`.)

---
**F-S2107-P1-009 — LOW — POLICY 11**
**Location:** bats `AC-012: S-, BC-, VP- tokens...`
**Defect:** Byte-equivalent duplicate of the preceding AC-012 test; inflates payload count with no delta.

---
**F-S2107-P1-010 — LOW — accepted-risk ledger entry**
**Defect:** `on_error="continue"` trap/timeout silent-skip is BC-ratified and NOT drift, but nothing produces the exhaustion evidence ADR-035 §Decision 5 requires; closed transitively by fixing F-S2107-P1-001.

---

**REFUTED IN SCOPE A (record as refuted; do not carry forward):** The hypothesis that `on_error="continue"` prevents blocking. `main.rs::extract_block_info` treats `stdout.contains("\"outcome\":\"block\"")` as a block trigger annotated "(any on_error)"; `HookResult::Block` serializes to `{"outcome":"block"}` → dispatcher exit 2 regardless. Registry matches spec. `on_error="continue"` forfeits only the fail-closed leg on trap/fuel-timeout.

---

### SCOPE B (verdict B5/H5/M3/L3)

---
**F-S2107-P1B-001 — BLOCKER — POLICY 13**
**Location:** `arm_a2.rs::extract_story_bc_version_citations` + `extract_version_token_from_table_row`
**Defect:** Admits any line with `|` + bc_id (no `\b` token boundary, no bounding section) and returns the FIRST version token (PC13 mandates LAST). Every S-21.07 Edge-Cases row ends `| BC-5.39.010 EC-0NN |` and many descriptions carry `v`-tokens → ≥9 spurious blocking violations. The gate makes its own governing story spec unwritable; every story documenting BC edge cases in a table is equally bricked.

---
**F-S2107-P1B-002 — BLOCKER — POLICY 13 missed-boundary + content-not-presence**
**Location:** same extractor
**Defect:** Only recognizes `v`-prefixed tokens; the canonical story Behavioral Contracts table writes the version BARE (`| BC-5.39.010 | <title> | 1.2 | ... |`) `[ORCH-VERIFIED: cell3=[ 1.2 ]]` → row silently skipped → the arm's headline coverage absent. Root is BC PC13's own regex `\bv([0-9]+\.[0-9]+)\b`, which cannot match the canonical format — code fix alone insufficient; PC13 must be amended (product-owner) or the table format normalized.

---
**F-S2107-P1B-003 — BLOCKER — POLICY 18**
**Location:** `arm_b.rs::parse_story_index_blockquote_hash`
**Defect:** Requires `line.starts_with("> S-21.07=")` ; PC21 specifies a WITHIN-line search. `[ORCH-VERIFIED: STORY-INDEX.md line 732 is a single "> **E-21 delivery:** …" line with prose prefix and ;-separated hashes; grep -c "^> S-21.07=" → 0]` → B3 leg permanently inert → `(Some(b2), None)` → advisory. Corrupting the blockquote to `deadbee` yields the identical advisory+Continue. Three-way collapses to two-way — S-21.04 pass-30 H02 regression class re-introduced. `[regression]`

---
**F-S2107-P1B-004 — BLOCKER — POLICY 18**
**Location:** `arm_b.rs::run_arm_b2`
**Defect:** Takes `rest.find('=')` on each `> ` line; on line 732 the first `=` sits inside the prose prefix, so `story_id` becomes the whole prefix and entries 2..7 are dropped → zero real catalog↔blockquote pairs compared, PLUS a spurious "orphaned blockquote entry" BLOCK on every STORY-INDEX.md write. Arm is simultaneously false-positive and inert. Its tests use a synthetic `> S-21.07=47a65c9` shape that occurs nowhere in production.

---
**F-S2107-P1B-005 — BLOCKER — BC-5.39.010 PC1**
**Location:** `dispatch.rs::is_bc_file`
**Defect:** Filename test is `f.starts_with("BC-") && f.ends_with(".md")` `[ORCH-VERIFIED]`; `BC-INDEX.md` satisfies both. PC1's basename exclusion and `BC-N.NN.NNN` shape guard are both unimplemented → every BC-INDEX.md write (state-manager Commit D of EVERY fix burst) blocks on both branches.

---
**F-S2107-P1B-006 — HIGH — BC PC5**
**Location:** `arm_a1.rs::extract_bc_index_version`
**Defect:** Returns the FIRST cell yielding a token; `split('|')` also splits escaped pipes, so version-history chains yield the OLDEST element. `[ORCH-VERIFIED: 23 rows; BC-1.13.001 = "v1.3 \| v1.4 \| …" vs current 1.12]` → 23 BCs permanently unwritable; inverse case fails open.

---
**F-S2107-P1B-007 — HIGH — POLICY 13 anchor-uniqueness**
**Location:** same extractor
**Defect:** Gates on `line.contains('|')` not `starts_with('|')`, so 17 frontmatter `change:` changelog lines (which PRECEDE the body table) can win. Sibling `parse_story_index_catalog_hash` correctly uses `starts_with('|')` — TD-VSDD-060 divergence.

---
**F-S2107-P1B-008 — HIGH — POLICY 13**
**Location:** `arm_b.rs::parse_story_index_catalog_hash`
**Defect:** Matches the first `|`-row CONTAINING the story id; catalog rows reference other stories in depends_on/blocks. STORY-INDEX line 669 (S-18.00's row) contains `S-18.01`, so writing S-18.01 reads hash `e5bc551` instead of `1b4ea21` → spurious BLOCK; when the wrong row's hash coincides, fail-open. No `^\| <id> \|` anchor, no match-count==1.

---
**F-S2107-P1B-009 — HIGH — POLICY 13 / PC20**
**Location:** `arm_b.rs::extract_input_hash_token`
**Defect:** Literal needle `"input-hash "` (single space, no `\s+`), single `find` with no retry, and `take_while(is_ascii_alphanumeric)` accepting `[0-9A-Za-z]` unbounded instead of `[0-9a-f]{7,40}`. `[ORCH-VERIFIED: ≥6 non-hex prose tokens in live STORY-INDEX.md — line 147 "convention", 155 "mismatch", 269 "bonus", 369 empty, 390 "updated", 398 "updated"; adversary cited 2, actual ≥6]` → spurious BLOCK or fail-open advisory.

---
**F-S2107-P1B-010 — HIGH — BC PC9/PC16**
**Location:** `dispatch.rs::is_story_file`
**Defect:** Lacks the mandated `^S-[0-9]+\.[0-9]+.*\.md$` basename guard → `.factory/stories/epics/E-21-*.md` dispatches as a story; `extract_story_id_from_path` yields `E-21`; catalog match hits S-21.01's row → spurious BLOCK on every epic write.

---
**F-S2107-P1B-011 — MEDIUM — story AC-019 / PC10/12/15/29/35 + TD-VSDD-059**
**Defect:** Cap divergences. `BC_MAX_BYTES=524_288` is correct but only used for A2 secondary reads, never the primary it was defined for. (DEDUP with `F-S2107-P1C-003`.)

---
**F-S2107-P1B-012 — MEDIUM — BC invariant 9 + TD-VSDD-059**
**Location:** `frontmatter.rs::extract_frontmatter_field`
**Defect:** Binds the slice `&trimmed[1..len-1]` BEFORE evaluating its `is_char_boundary` guard, so the guard cannot prevent the panic it exists for; it is also unconditionally true given the enclosing ASCII-quote branch. Invariant 9 satisfied vacuously; sibling `strip_quotes` equally vacuous. HONEST NEGATIVE RESULT: every byte-index slice in the five SCOPE-B files was traced and NO reachable panic exists — the finding is the dead guard, not a crash.

---
**F-S2107-P1B-013 — MEDIUM — BC PC34**
**Location:** `dispatch.rs::is_frontmatter_parity_target`
**Defect:** VP branch matches `VP-INDEX.md` (same class as F-S2107-P1B-005), omits PC34's `ss-*` and `S-*` constraints, and adds an `epics` clause with no PC34 counterpart. Trigger set simultaneously wider and looser than spec.

---
**F-S2107-P1B-014 — LOW — TD-VSDD-060**
**Location:** `arm_a1.rs::extract_version_token`
**Defect:** Lacks the leading word-boundary check its `arm_a2.rs` sibling has; `rev1.5` / `Nov1.6` yield tokens in A1, rejected in A2.

---
**F-S2107-P1B-015 — LOW — POLICY 7**
**Defect:** Story body BC-table title drops both BC H1 trigger clauses ("after a BC frontmatter bump", "after a story edit"). (DEDUP with `F-S2107-P1C-013`.)

---
**F-S2107-P1B-016 — LOW — SOUL.md #4**
**Location:** `arm_a2.rs::run_arm_a2_for_bc_with_result`
**Defect:** `from_utf8(...).unwrap_or("")` turns a decode failure into `bc_version = ""`, misdirecting the fixer to a version bump; decode error discarded rather than surfaced as its own violation class.

---

**Fail-open table (SCOPE B):** A1 = Y conditional (HIGH); A2 = Y confirmed (BLOCKER); B1 = Y confirmed (BLOCKER); B2 = Y confirmed (BLOCKER). Truncation axis RESOLVED — `read_bounded` hard-errors (`ReadErr::TooLarge` → `OUTPUT_TOO_LARGE`), NO silent truncation; exceeding a cap causes spurious BLOCK, never a silent pass. BC PC4's anti-false-green claim holds.

---

### SCOPE C (verdict B2/H10/M6/L3)

---
**F-S2107-P1C-001 — BLOCKER — POLICY 13 ESCAPE-SCOPE-PARITY / BOUNDARY-POLARITY**
**Location:** `lib.rs::on_post_tool_use`
**Defect:** Performs the primary `host::read_file` at Step 2 BEFORE classification at Step 4, and converts ANY `Err` (including `CapabilityDenied`) into a block. `[ORCH-VERIFIED: Step 2 precedes Step 4; Err(e) => return combine_violations_into_block(...); registry has tool = "^(Edit\|Write\|MultiEdit)$" and NO path trigger field — path_allow is a capability allowlist of only 4 prefixes: .factory/specs/behavioral-contracts/, .factory/specs/verification-properties/, .factory/stories/, .factory/cycles/]` → EVERY Edit/Write/MultiEdit to any path outside those 4 prefixes exits 2: crates/\*\*/\*.rs, CLAUDE.md, Cargo.toml, CHANGELOG.md, hooks-registry.toml, AND .factory/STATE.md, .factory/policies.yaml, .factory/specs/architecture/\*\*, .factory/tech-debt-register.md. The repository becomes unwritable the moment this WASM ships — including the factory's own state file and repair path. Inverts the BC's own dispatch pseudocode (read INSIDE each classified branch; unclassified → Continue) and the sibling precedent `validate-closes-completeness` ("else: Continue (not our file)"; "Read file … On error: Continue + log_warn"). No bats test covers an unclassified or non-`.factory` path — the suite is structurally blind.

---
**F-S2107-P1C-002 — BLOCKER — BC invariant 6 / PC29 / PC33**
**Defect:** Cycle artifacts fall to the `else` branch and get `PRIMARY_READ_MAX_BYTES = 1_048_576` where PC29 + AC-019 specify `2097152`; Err disposition is unconditional BLOCK, so (i) a `lessons.md`/`burst-log.md`/cycle `INDEX.md` over 1 MiB blocks — a live near-term condition given CLAUDE.md's ≤3500 soft/≤4000 hard line budget — and (ii) `HostError::NotFound` on a cycle artifact blocks where PC33 mandates advisory+Continue. Both violate "Class D is advisory-only, never blocking". EXONERATES `run_arm_d` itself: it returns `Vec<Advisory>`, `lib.rs` surfaces it via a separate inline `host::log_warn` loop and returns `Continue` before `violations` is consulted — arm D output can NEVER reach `combine_violations_into_block`.

---
**F-S2107-P1C-003 — HIGH — story AC-019 / PC10/12/29/35**
**Defect:** Three of six caps wrong: BC primary and story primary get 1_048_576/5_000 vs spec 524288/3000; cycle artifact gets 1_048_576/5_000 vs spec 2097152/5000. `[ORCH-VERIFIED: PRIMARY_READ_MAX_BYTES=1_048_576, PRIMARY_READ_TIMEOUT_MS=5_000]` The AC-019 constant test pins only the four BC_INDEX/BC constants, so all three divergences are invisible to `cargo test`. (DEDUP with `F-S2107-P1B-011`.)

---
**F-S2107-P1C-004 — HIGH — POLICY 15 NAME-SET-EQUALITY + TD-VSDD-059**
**Defect:** Story AC-004 names `test_BC_5_39_010_arm_a1_primary_target_capability_denied_blocks`; no such symbol exists. Shipped symbol is `..._capability_denied_contract` `[ORCH-VERIFIED: lib.rs:305]` and its body has ZERO assertions plus a retained `todo!()`-era comment. AC-004 is the fail-closed primary-target invariant (invariant 4) — the most safety-relevant AC in the story — with no Rust assertion at all.

---
**F-S2107-P1C-005 — HIGH — POLICY 11 + invariant 9 + TD-VSDD-059**
**Location:** `lib.rs::test_BC_5_39_010_multibyte_utf8_no_panic`
**Defect:** Builds a payload with only a `file_path`, discards the result, asserts nothing; the multi-byte string exists only in a comment. Under the non-wasm test host no multi-byte byte ever reaches the extractors, so all four `is_char_boundary` guards are unexercised. Invariant 9 has no load-bearing test.

---
**F-S2107-P1C-006 — HIGH — Class-C count shape / POLICY 7**
**Defect:** Four-way count divergence: story `title:` + H1 and STORY-INDEX ×3 say "five-arm"; registry comment says "Six arms"; bats header says "seven arms (A1,A2,B1,B2,D,E1,E2)"; `lib.rs` module doc enumerates 7; BC §Description says "four mechanically-gateable classes". Nothing enumerates five of anything.

---
**F-S2107-P1C-007 — HIGH — POLICY 14 leg 3 / POLICY 17**
**Defect:** BC-5.39.010 v1.2 `modified[]` has only `["2026-07-30", "2026-07-30 (v1.2)"]` but the body Changelog carries three rows (1.2/1.1/1.0) and `last_amended` records a two-deep chain — the v1.1 entry is missing. Note: the hook this story ships checks `modified[]` monotonicity but not `modified[]`↔Changelog row correspondence, so it cannot catch the defect in its own governing BC.

---
**F-S2107-P1C-008 — HIGH — POLICY 14 leg 2 / POLICY 17**
**Defect:** Story S-21.07 has NO `## Changelog` section (40 headings enumerated; file ends at `## File Structure Requirements`). At v1.1 with two `modified[]` entries and a two-deep `last_amended` chain, leg 2 is absent entirely while legs 1/3/4 are present and consistent.

---
**F-S2107-P1C-009 — HIGH — POLICY 14 leg 2+leg 5 / POLICY 15**
**Defect:** BC-INDEX.md frontmatter is `version: "4.43"` but its newest `changelog:` entry is `v4.41` (BC-5.39.010 v1.0→v1.1). No v4.42 or v4.43 BC-INDEX changelog row exists. The BC-5.39.010 body-table cell reads v1.2 (current) while the index's own changelog documents only the v1.0→v1.1 propagation — leg-5 content-correspondence record stale by one BC version, and BC-INDEX has two of its own bumps unrecorded. Sixth-consecutive re-staling class from the S-21.04 cascade. `[regression]`

---
**F-S2107-P1C-010 — HIGH — POLICY 8 / BC-5.39.008 schema discipline**
**Location:** `.factory/policies.yaml` policy id 19 (`adr_version_cite_volatile_pin_prohibition`)
**Defect:** Writes `scope: behavioral-contracts traceability rows` as a BARE SCALAR where all 20 siblings use a flow sequence and the file's own header schema documents `scope: [<string>]`. Field ordering also diverges (`scope` before `enforced_by`). `[ORCH-VERIFIED: yaml.safe_load yields a character array — "b, e, h, a, v, i, o, r, a, l, …" — when the rubric is auto-loaded]` Strict deserializer errors on the whole document; duck-typed consumer character-iterates. Same class as the pass-28 BLOCKER where invalid `\+` escapes made the rubric unparseable for eight passes. Pre-existing, not introduced by S-21.07. Route: state-manager/spec-steward. `[process-gap]`

---
**F-S2107-P1C-011 — HIGH — BC PC31 / POLICY 13**
**Location:** `arm_d.rs::run_arm_d`
**Defect:** Uses `lower.contains("closes:")` where PC31 specifies `^Closes:\s*(.+)$`. Two defects: (i) the BC's `^`-anchored form would match ZERO real burst-log lines because D-444(c) blocks are bold-markdown (`**Closes:** …`), so the CODE is right and PC31 is WRONG — but per CLAUDE.md rule 12 the spec wins, so PC31 must be amended by product-owner; the deviation is recorded nowhere and a future reader reconciling code to PC31 would "fix" the code into total inertness. (ii) the `else if` makes first-match win, so `**Closes:** F-X | **Refs:** B01` never scans `B01`.

---
**F-S2107-P1C-012 — HIGH — BC PC30**
**Location:** `arm_d.rs::extract_scope_limited_region`
**Defect:** Lessons branch matches `L-EDP1-` ANYWHERE in a line (PC30 anchors at line start). Real `lessons.md` prose routinely cross-references sibling lessons inline; any such reference after the last true entry heading — including inside the last entry's own body above its `**Closes:**` line — becomes the scope anchor and truncates the region, so the `Closes:` line the arm exists to scan falls outside scope. Silent false-negative on the primary Class-D artifact. Both unit tests use synthetic fixtures with column-0 anchors only.

---
**F-S2107-P1C-013 — MEDIUM — POLICY 7**
**Defect:** Story body BC-table title drops both BC H1 trigger clauses; BC-INDEX carries the H1 verbatim so the story body is the sole divergent site; the dropped clauses name which PostToolUse event fires each arm. (DEDUP with `F-S2107-P1B-015`.)

---
**F-S2107-P1C-014 — MEDIUM — BC PC37**
**Location:** `arm_e.rs::extract_last_amended_outer_version`
**Defect:** Gates `if len < 17 { return None }`, rejecting spec-valid `"2026-07-30 (v2)"` (15 bytes; PC37's capture permits one-or-more components) → unparseable advisory and E1 comparison SKIPPED on a blocking arm. Byte-walk otherwise clean; no panic path found.

---
**F-S2107-P1C-015 — MEDIUM — PC35–37 / SOUL.md #4**
**Location:** `arm_e.rs::run_arm_e1`
**Defect:** Returns empty violations AND empty advisories when `version:` or `last_amended:` is absent — completely silent; BC sanctions silence only for the no-frontmatter case. Compounding: `extract_frontmatter_field` returns None unless the very first line is exactly `---`, so a UTF-8 BOM or leading blank line silently disables E1, E2, A1 version extraction and A2 `behavioral_contracts` lookup on blocking arms.

---
**F-S2107-P1C-016 — MEDIUM — POLICY 15 / AC-018**
**Location:** `lib.rs::test_BC_5_39_010_combined_a1_and_e1_single_block`
**Defect:** Hand-constructs two `Violation` structs and calls `combine_violations_into_block` directly, testing the FORMATTER not the AGGREGATION; cannot detect cross-arm suppression, which is invariant 7's whole point. Genuine path exists only in bats behind `_require_artifacts` skip.

---
**F-S2107-P1C-017 — MEDIUM**
**Location:** `arm_d.rs::extract_scope_limited_region`
**Defect:** Accumulates `byte_offset += line.len() + 1` at two sites; `str::lines()` strips trailing `\r`, so CRLF content drifts one byte per preceding line. `is_char_boundary` guards prevent a panic but the slice starts mid-line; the fallback degrades to returning the ENTIRE file, defeating the frozen-provenance exclusion PC30 enforces.

---
**F-S2107-P1C-018 — MEDIUM `[process-gap]`**
**Defect:** `run-all.sh` never sets `CI_REQUIRE_ARTIFACTS`, so all 34 payload bats tests can skip-as-pass; the only Rust assertions for AC-004, AC-018-as-specified and AC-019 cap-passing live in that suite. Currently green-for-real (both artifacts present), but a staging regression converts all 34 to silent skips. RETRACTED sub-claim: `run-all.sh` globs `tests/*.bats` so the new suite IS auto-registered — story Task 16 satisfied, no finding there. (DEDUP with `F-S2107-P1-008`.)

---
**F-S2107-P1C-019 — LOW**
**Location:** `arm_e.rs::strip_date_annotation`
**Defect:** No-op on whitespace-free annotations (`"2026-07-30(v1.1)"`); reverse ordering yields a spurious E2 block. Not observed live.

---
**F-S2107-P1C-020 — LOW**
**Location:** `arm_d.rs::run_arm_d`
**Defect:** `.contains("closes:")` false-triggers on `discloses:`/`forecloses:`; advisory-only so noise not breakage.

---
**F-S2107-P1C-021 — LOW pending-intent**
**Location:** `arm_d.rs::is_finding_like` + `EXCLUDED_PREFIXES`
**Defect:** `O-` is not excluded, so `O-P30-001` is flagged. Per D-449(d)(i) observation IDs are out of scope for 4-index changelog Refs, so flagging may be DESIRED; story/BC never state intent.
**Route:** product-owner to confirm `O-` is deliberately non-excluded.

---

## 4. AC Coverage Table (SCOPE C)

| AC | Gate | Name-match |
|----|------|-----------|
| AC-001 | Y | Y |
| AC-002 | Y | Y |
| AC-003 | Y | Y |
| AC-004 | Y | **N** — shipped symbol is `..._capability_denied_contract` not `..._capability_denied_blocks` (F-S2107-P1C-004); body has ZERO assertions |
| AC-005 | Y | Y |
| AC-006 | Y | Y |
| AC-007 | Y | Y |
| AC-008 | Y | Y |
| AC-009 | Y | Y |
| AC-010 | Y | Y |
| AC-011 | Y | Y |
| AC-012 | Y | Y |
| AC-013 | Y | Y |
| AC-014 | Y | Y |
| AC-015 | Y | Y |
| AC-016 | Y | Y |
| AC-017 | Y | Y |
| AC-018 | Y (name) | **N** — tests formatter not aggregation (F-S2107-P1C-016) |
| AC-019 | PARTIAL (3 of 6 caps wrong) | **N** — substance diverges (F-S2107-P1-003, F-S2107-P1C-003, F-S2107-P1C-005) |
| AC-020 | Y | Y |
| AC-021 | PENDING (deliberate operator-cache staging) | — |

**Result:** 19/21 name-matched, 1 name-mismatch (AC-004), 1 deliberate-pending (AC-021). Cardinality alone reads as parity; contents diverge → HIGH per POLICY 15.

---

## 5. Policy Execution Ledger (Union of Three Scopes)

| Policy | Result | Notes |
|--------|--------|-------|
| POLICY 1 | EXECUTED-CLEAN | AC/EC/T/RG IDs each occur once; no T-010-class collision |
| POLICY 2 | NOT-APPLICABLE | No DI in perimeter; BC §Traceability "none — process-automation gate" |
| POLICY 3 | NOT-APPLICABLE | Pre-merge perimeter |
| POLICY 4 | EXECUTED | F-S2107-P1C-011, F-S2107-P1B-013 |
| POLICY 5 | EXECUTED | — |
| POLICY 6 | EXECUTED-CLEAN | ARCH-INDEX: SS-04 Plugin Ecosystem, SS-05 Pipeline Orchestration — story + BC verbatim |
| POLICY 7 | EXECUTED | F-S2107-P1B-015, F-S2107-P1C-013 |
| POLICY 8 | EXECUTED | 1 finding; bcs→body table→21 ACs→Token Budget all present with correct v1.2 pin; title verbatim-ness fails |
| POLICY 9 | EXECUTED-CLEAN | 17-VP deferral (VP-102..VP-118) durably recorded at 4 sites |
| POLICY 10 | EXECUTED | Demos not yet recorded; no misplaced evidence |
| POLICY 11 | EXECUTED | Many findings |
| POLICY 12 | EXECUTED | — |
| POLICY 13 | EXECUTED | All extractors, both directions; NO extractor implements anchor-uniqueness |
| POLICY 14 | EXECUTED | 3 findings |
| POLICY 15 | EXECUTED | Several; note D-889 `&&{false;}\|\|true` trap pattern absent — all guards use valid `\|\| { echo; false; }` form |
| POLICY 16 | EXECUTED-CLEAN | Only D-923 and D-889 cited; both corroborated in policies.yaml `amended_at`/`extended_at` |
| POLICY 17 | EXECUTED | 2 findings by extension |
| POLICY 18 | EXECUTED | Story input-hash three-way equality HOLDS (`8ba2a75` == catalog == blockquote), but GATE's B3 leg is inert (F-S2107-P1B-003) |
| POLICY 19 | EXECUTED | BC ADR cite-form CLEAN (stable `ADR-035 §Decision N`, no version pin); registry data defect F-S2107-P1C-010 |
| POLICY 20 | NOT-APPLICABLE | No release bundle; WASM is registry-referenced so not an orphan |
| POLICY 21 | EXECUTED-CLEAN | Zero new `.sh` |
| POLICY 22 | EXECUTED | All closure claims re-derived from primary sources; two claims explicitly marked unverifiable |

---

## 6. Verified-Clean (pass-2 do not re-litigate)

- No arm's error aborts the others — `violations.extend()` accumulates across A1/A2/B1/E1/E2, satisfying invariant 7.
- No violation downgraded or dropped.
- `combine_violations_into_block` joins with `" | "` and applies NO truncation.
- Zero `println!` in the crate.
- Zero `unwrap()`/`expect()` outside `#[cfg(test)]`.
- `main.rs` is a correct SDK trampoline with `WIRING-EXEMPT` annotation.
- `Cargo.toml` carries no `serde_yaml`, `wasmtime`, `factory-dispatcher`, or `regex`; `crate-type = ["cdylib","rlib"]` correct.
- `read_bounded` hard-errors at cap — no silent truncation.
- `run_arm_e2` `curr < prev` correct.
- `EXCLUDED_PREFIXES` = PC32's 11 prefixes verbatim with no shadowing.

---

## 7. NOT REACHED (drives pass-2 dispatch)

- Test execution (no Bash in adversary): all pass/fail claims derived by reading, nothing run.
- Git history/diff: "three fixtures reverted" count unverifiable.
- Byte sizes of `lessons.md` / `BC-INDEX.md` / `STORY-INDEX.md` vs the 1 MiB / 2 MiB caps unmeasured — F-S2107-P1C-002's >1 MiB precondition is asserted as live risk, not measured; RECOMMEND the fix burst capture `wc -c` for all three.
- ADR-035 body §Decision 1/4/5/6 not read, so `ADR-035 §Decision N` anchor targets unverified.
- `VP-INDEX.md`, `verification-architecture.md`, `verification-coverage-matrix.md` not opened (POLICY 9 arithmetic unverified).
- `decision-log.md` not opened.
- BC-INDEX `changelog:` array not read in full (~30 rows).
- Story spec lines 654–790 (Tasks 9+, Red Gate table, Demo Evidence, Definition of Done) truncated by token cap in SCOPE B, partially covered in SCOPE C.
- Individual bats fixture CONTENTS not inspected — cannot rule out degenerate fixtures triggering early returns.
- Live-artifact E1 blast-radius census not performed: unknown how many existing `.factory/` files have `version:` ≠ `last_amended` outer prefix — RECOMMEND a pre-merge census.

---

## 8. Verdict

**NOT-CLEAN.** Counts B7/H19/M13/L8 = 47 findings pre-dedup (confirmed dedup pairs: F-S2107-P1B-011↔F-S2107-P1C-003; F-S2107-P1B-015↔F-S2107-P1C-013; F-S2107-P1-008↔F-S2107-P1C-018). Streak 0/3. Novelty HIGH — all first-observation classes.

**Systemic root cause (record prominently):** every extractor was written against synthetic `#[cfg(test)]` fixtures rather than production artifact shape, which is why 79 unit tests + 35 bats pass green while all four blocking arms fail open on real files. Second root cause: read-before-classify with a single global cap — fixing that ordering (classify first; read inside the branch with the per-branch cap; `Continue + log_warn` on unclassified/absent, per the `validate-closes-completeness` precedent) closes F-S2107-P1C-001, F-S2107-P1C-002 and F-S2107-P1B-003 together.

**Recommendation for the fix burst:** replace unit fixtures with excerpts copied VERBATIM from `BC-INDEX.md` (rows for BC-1.13.001 / BC-1.15.001 / BC-4.16.001 / BC-5.41.002) and `STORY-INDEX.md` (lines 269, 369, 669, 692, 730, 732) BEFORE any code change, so the Red Gate reproduces the confirmed failures.

---

## Summary

| Severity | Count |
|----------|-------|
| BLOCKER | 7 |
| HIGH | 19 |
| MEDIUM | 13 |
| LOW | 8 |

**Overall Assessment:** block
**Convergence:** findings remain — iterate
**Readiness:** requires revision — two BLOCKER classes render the gate repository-breaking if shipped

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 1 |
| **New findings** | 47 |
| **Duplicate/variant findings** | 3 (confirmed dedup pairs: B-011↔C-003, B-015↔C-013, P1-008↔C-018) |
| **Novelty score** | 47 / (47 + 3) = 0.94 |
| **Median severity** | HIGH |
| **Trajectory** | 47 |
| **Verdict** | FINDINGS_REMAIN |
