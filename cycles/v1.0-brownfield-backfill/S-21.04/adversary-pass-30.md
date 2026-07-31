---
document_type: adversarial-review
level: adversary
version: "1.0"
status: complete
producer: vsdd-factory:adversary
timestamp: 2026-07-30T00:00:00Z
phase: 3
inputs:
  - stories/S-21.04-story-worktree-write-path-discipline.md
  - specs/behavioral-contracts/ss-06/BC-6.26.001.md
  - specs/architecture/decisions/ADR-034-ci-gate-product-branch-operand-isolation-and-runtime-derived-counts.md
  - cycles/v1.0-brownfield-backfill/S-21.04/implementation/red-gate-log.md
  - policies.yaml
input-hash: "ae93f6f"
traces_to: "BC-6.26.001 v1.18; story v1.33; ADR-034 v1.1"
pass: 30
verdict: NOT-CLEAN
reviewed_head: "44547051"
fixes_landed_head: "44547051"
novelty: high
previous_review: "cycles/v1.0-brownfield-backfill/S-21.04/adversary-pass-29.md"
findings_count: 16
severity_breakdown: "B0/H9/M5/L2"
streak: "0/3"
trajectory_append: 16
model_override: false
model_resolved: "claude-opus-5"
adr033_deviation: "ADR-033 cross-family limitation — cross-family claim (GPT-5) NOT satisfied; ran on Claude, same family as authoring agents; fresh context + information asymmetry intact, cross-family independence absent"
asymmetry_enforcement: "pass-29 Part A lines 1–98 only (frontmatter + Part A + orchestrator-verified additions); Part B (from ## Part B onward), Fix Mapping, Summary, Novelty, Completeness not read; prior pass files (pass-1 through pass-28) and cycle INDEX.md off-limits; adversary confirmed it opened none"
policy22_note: "POLICY 22 SUBAGENT-REPORT-FIDELITY active: adversary findings reported directly; no relay chain between adversary and state-manager in this pass"
persistence_note: "RECOVERY BURST D-946: adversary output was NOT persisted immediately after pass-30 ran (2026-07-30). Orchestrator pivoted to BC-5.39.010/ADR-035/S-21.07 design arc before Post-Adversary Persistence. File persisted from volatile /tmp/p30.md in D-946 recovery burst. Content is verbatim from authoritative source."
---

# Adversary Pass 30 — S-21.04 LOCAL cascade (BC-5.39.001)

**Reviewed HEAD:** `.worktrees/S-21.04` @ `44547051` (develop `948f0fb1`) · **Story** v1.33 · **BC-6.26.001** v1.18 · **ADR-034** v1.1
**Verdict:** **NOT-CLEAN** — B0 / H9 / M5 / L2 = 16 findings · **Streak: 0/3**

---

## Finding ID Convention

Finding IDs for this cascade use the format: `F-S2104-P<PASS>-<SEV><SEQ>` (project-local convention established at pass-1). Map to template ADV severity abbreviations: B→BLOCKER, H→HIGH, M→MEDIUM, L→LOW. Full format example: `F-S2104-P30-H01`.

## 1. Mandatory Provenance Disclosure

### Files read (complete list, absolute paths)

Factory artifacts (canonical `.factory/` mount):
- `/Users/zious/Documents/GITHUB/vsdd-factory/.factory/policies.yaml` (full, 425 lines, three reads)
- `/Users/zious/Documents/GITHUB/vsdd-factory/.factory/stories/S-21.04-story-worktree-write-path-discipline.md` (full, 299 lines)
- `/Users/zious/Documents/GITHUB/vsdd-factory/.factory/specs/behavioral-contracts/ss-06/BC-6.26.001.md` (full, 378 lines)
- `/Users/zious/Documents/GITHUB/vsdd-factory/.factory/specs/architecture/decisions/ADR-034-ci-gate-product-branch-operand-isolation-and-runtime-derived-counts.md` (full, 372 lines)
- `/Users/zious/Documents/GITHUB/vsdd-factory/.factory/specs/behavioral-contracts/BC-INDEX.md` (grep only: frontmatter changelog line 8, catalog row 2053)
- `/Users/zious/Documents/GITHUB/vsdd-factory/.factory/stories/STORY-INDEX.md` (grep + read lines 715–739)
- `/Users/zious/Documents/GITHUB/vsdd-factory/.factory/specs/architecture/ARCH-INDEX.md` (read 432–447, 533–542)
- `/Users/zious/Documents/GITHUB/vsdd-factory/.factory/specs/verification-properties/VP-INDEX.md` (grep only, `-o`, line 8)
- `/Users/zious/Documents/GITHUB/vsdd-factory/.factory/STATE.md` (**grep `-o` only**, pattern `FABRICATED[^;]{0,120}` → lines 65/137/238; Session Resume Checkpoint **not read**)
- `/Users/zious/Documents/GITHUB/vsdd-factory/.factory/cycles/v1.0-brownfield-backfill/S-21.04/adversary-pass-29.md` — **lines 1–98 only** (frontmatter + Part A + orchestrator-verified additions). Part B (from line 96 `## Part B` onward), Fix Mapping, Summary, Novelty, Completeness **not read**.
- `/Users/zious/Documents/GITHUB/vsdd-factory/.factory/cycles/v1.0-brownfield-backfill/S-21.04/implementation/red-gate-log.md` (frontmatter 1–14; 55–66; 1508–1521; 2740–2765; greps)

Product-branch code (worktree-rooted):
- `/Users/zious/Documents/GITHUB/vsdd-factory/.worktrees/S-21.04/plugins/vsdd-factory/tests/worktree-identity-preflight.bats` (greps + read 1060–1198)
- `/Users/zious/Documents/GITHUB/vsdd-factory/.worktrees/S-21.04/plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats` (greps + reads 636–665, 778–817, 915–1014, 2850–3049)
- `/Users/zious/Documents/GITHUB/vsdd-factory/.worktrees/S-21.04/plugins/vsdd-factory/tests/fixtures/story-worktree/README.md` (full)

**NOT read** (asymmetry compliance): Part B of any prior pass file; `.factory/cycles/v1.0-brownfield-backfill/INDEX.md`; STATE.md Session Resume Checkpoint; `adversary-pass-01..28.md`; `burst-log.md`, `decision-log.md`, `lessons.md` (only `files_with_matches` presence output was returned for a `FABRICATED` grep — no content read).

### Proof-of-read — POLICY rubric (`.factory/policies.yaml` v1.4.18, parses as YAML, 22 policies)

**POLICY 5 `creators_justify_anchors` — 17 `verification_steps`.** Final entry, verbatim:

> "HARNESS-EMITTED LINE-NUMBER CARVE-OUT (D-914 per F-S2104-P15-004 pass-15 bare-pin class): line numbers that appear inside verbatim harness-emitted stdout blocks (e.g., bats `# (in test file ..., line NNN)` annotations, `line NNN` failure markers automatically emitted by the bats test framework into fenced code blocks) are EXEMPT from the stable-anchor prohibition — redacting or replacing them would falsify the verbatim record that POLICY 15 (ll_n_verbatim_stdout_discipline) requires. Authored prose and manual comments remain fully bound: bare line-number pins of the form `line ~NNN`, `~:NNN`, `:NNN`, `lines NNN/MMM` in manually-written comment text, docblocks, or narrative prose are forbidden under TD-VSDD-091. The carve-out is tightly scoped to automatically-emitted harness output only; it authorizes no authored line-number pin. Detection: when auditing for TD-VSDD-091 line-pin violations, strip bats-emitted verbatim-stdout fenced blocks (identified by surrounding `Exit code:` / bats output structure) before grepping for bare line-number patterns in manually-authored text."

**POLICY 13 `hh_n_regex_alternation_predicates` — 13 `verification_steps`.** Final entry, verbatim:

> "ALTERNATION-WIDENING-DIRECTION-STATEMENT MANDATE (D-920 per F-S2104-P18-001/005 BLOCKER/HIGH-class ninth-generation; D-497 parsimony extension): any burst that adds a member to an existing alternation in a gate predicate MUST, in the same burst, explicitly state the DIRECTION of that alternation: is it on the OPEN side (trigger — any sentence in domain) or the CLOSED side (enumeration — only listed forms)? Two cases: (a) if the alternation is the TRIGGER (open side of an implication), new members expand coverage legitimately — state 'direction: (b) open trigger' in the audit row; (b) if the alternation is the ENUMERATION (closed side of an implication), adding members is a paper-fix by construction, because a paraphrase not on the list still evades — in this case the burst MUST ALSO add a complementary OPEN-TRIGGER gate covering the same obligation axis, and the audit row MUST state 'direction: (b) closed enumeration; backstopped by <gate-name> (open trigger)'. A widening that does neither is a paper-fix under TD-VSDD-059 regardless of mutant count, because the added mutants are drawn from the same vocabulary as the predicate and do not probe the unlisted-synonym axis. Detection: for each burst containing `|` additions to an existing bats alternation, verify the story AC Gate cell and red-gate-log audit row contain a direction statement; if absent, flag as POLICY 13 HIGH (alternation-widening-direction-statement absent). The detection is mechanically available: `git diff HEAD~ -- *.bats | grep '^+.*|' | grep -v '^+++' | grep -v '^\+\+\+'` surfaces alternation additions; the corresponding story cell and audit row must be updated in the same burst."

**POLICY 15 `ll_n_verbatim_stdout_discipline` — 9 `verification_steps`.** Final entry, verbatim:

> "BACKSTOP-DOMAIN-PARITY MANDATE (D-920 per F-S2104-P18-005 HIGH-class ninth-generation; D-497 parsimony extension): when a fix predicate specifies a BACKSTOP gate whose purpose is to make another gate's enumeration non-load-bearing (i.e., the backstop gate is the open-trigger complement that closes the ALTERNATION-WIDENING-DIRECTION-STATEMENT mandate's closed-enumeration case), the two gates MUST share a domain, and the red-gate-log audit table MUST show their domains in adjacent rows so a domain divergence is visible on inspection. Where domains legitimately differ (e.g., the backstop gate covers a different syntactic form or a different section scope), the divergence MUST be stated in THREE places in the same burst: (1) in the bats code comment for both gates, (2) in the story Gate cell listing both gates, and (3) in the audit table rows for both gates. A backstop gate that silently operates on a narrower domain than the gate it backstops provides no actual backstop — the attacker exploits the domain gap by placing the harmful text in the unguarded region. Detection: for each pair of gates where gate B is described as 'backstopping' or 'backing' gate A, verify in the audit table that their domain rows are adjacent and their domain descriptions are either identical or explicitly annotated with divergence rationale; if absent, flag as POLICY 15 HIGH (backstop-domain-parity gap). The F-S2104-P18-005 recurrence (Gate 2b(c) paragraph-scoped while Gate 2b(a) was section-wide) is the canonical example: the backstop was designed to make the nullification list non-load-bearing but operated on the prohibition paragraph only while the nullification gate was section-wide — a domain gap of exactly one scope level that admitted three surviving vectors."

Policies executed this pass: 1, 4, 5, 6, 7, 8, 13, 14, 15, 16, 17, 18, 19, 21, 22. Not executed (out of scope / no artifacts): 2 (no DI in scope), 9 (`verification_properties: []`), 10 (no demo-evidence in this perimeter), 11, 12, 20.

---

## 2. Part A — Fix Verification at `44547051`

| ID | Claim | Verdict | Literal evidence |
|----|-------|---------|------------------|
| **H01** | T-016 rewritten per ADR-034 v1.1; `actual_count` = runtime DOC-PARITY FAIL count in AC-001(a); `# T001_GATE_COUNT=24` sentinel; word→int map removed; three mutants | **GENUINELY-CLOSED** | `worktree-identity-preflight.bats:1090-1092` `actual_count="$(awk '/# --- DOC-PARITY .*Spec-Path Discipline: AC-001\(a\)/,/# --- DOC-PARITY .*Spec-Path Discipline: EC-006 WARNING/' "$bats_suite" \| grep -cE '^\s+echo "DOC-PARITY FAIL')"`; `:1106` sentinel extract; `:1096` zero-count trap; mutants `:1162` (M1 gate deletion), `:1176` (M2 sentinel drift), `:1187` (M3 zero-count trap). Independent recount: `grep -n '^\s\+echo "DOC-PARITY FAIL'` on `story-worktree-write-path-discipline.bats` inside markers 640→1751 yields exactly 24 lines (796, 813, 851, 864, 922, 955, 961, 970, 976, 984, 994, 1004, 1117, 1456, 1474, 1502, 1535, 1554, 1579, 1677, 1694, 1709, 1720, 1746); sentinel `story-worktree-write-path-discipline.bats:641` `  # T001_GATE_COUNT=24`. **Count parity 24 == 24 confirmed.** |
| **H02** | T-008 position/predicate-agnostic; ellipsis is the *declared* escape convention; count-closure leg added | **GENUINELY-CLOSED** | `story-worktree-write-path-discipline.bats:2970-2971` trigger `find[[:space:]][^;&\|]*\.factory` with `grep -qvE 'find[[:space:]]+(\.\.\.\|…)[[:space:]]+\.factory([^[:alnum:]_]\|$)'`; `:2933-2934` "The ellipsis convention is now DECLARED"; `:2981-2986` count-closure leg (`sed 's/\\\.factory/ESCAPED/g'` then `grep -qE '\.factory.*\.factory'`); `:2936-2940` negative-twin property; `:2942-2955` BOUNDARY-POLARITY block. |
| **H03** | Four-site retracted-B01 identity/status correction with error-acknowledgment | **GENUINELY-CLOSED** | `STORY-INDEX.md:727` `[CORRECTED H03: FABRICATED identity was "policies.yaml YAML-parse"; real B01 = F-S2104-P28-B01; CLOSED 44547051]`; `VP-INDEX.md:8` `FABRICATED identity by prior state-manager burst at 3d12b780`; `STATE.md:65` `FABRICATED — real B01 = coupling-gate detached-HEAD CI failure CLOSED 44547051]`; `STATE.md:137` same-class; `STATE.md:238`. |
| **H04** | Phantom anchor `test_write_discipline_gates` replaced at three live story sites | **GENUINELY-CLOSED** | `grep -n 'stray-file-blocks' story` → `110`, `215`, `274` (three live sites); real `@test` exists at `story-worktree-write-path-discipline.bats:504` `@test "T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called" {`. Residual `test_write_discipline_gates` hits are only `story:11` (`last_amended`) and `story:47` (`modified[]` v1.32) — historical-by-construction per POLICY 5 v1.3.5 Part A (iii)/(i). |
| **H05** | T-016 no longer reads `.factory/` at all | **GENUINELY-CLOSED** | `grep -n 'fa_wt\|FACTORY_ROOT\|story_count\|bats_word' worktree-identity-preflight.bats` → single hit `:1128` `#   Removed: fa_wt discovery loop, story_file, story_count, word-to-integer map.` (a comment). `:1145` `bats_suite="$PLUGIN_ROOT/tests/story-worktree-write-path-discipline.bats"` — both operands product-branch. |
| **M01** | Pass-28 numeric finding IDs remapped to house form across tests, rules, skills, workflows, `agents/adversary.md` | **PARTIAL** ⚠ | `grep -rn 'F-S2104-P28-0[0-9][0-9]' .worktrees/S-21.04/plugins` → **6 residual numeric IDs in `tests/`**: `worktree-identity-preflight.bats:233` `# F-S2104-P28-016 + coordinator correction: adaptive section-bounded extractor`; `:957`, `:991`, `:1013`, `:1034`, `:1056` `# F-S2104-P28-017: CONTROL leg added to make restore-attestation true as written.` Neither `016` nor `017` exists in the authoritative pass-28 set (B01, H01–H07, M01–M07, L01, L02). See **F-S2104-P30-M01**. |
| **M02** | Story provenance `F-S2104-P28-H03` → `F-S2104-P28-M02` | **GENUINELY-CLOSED** | `story:11` and `story:49` both read `[CORRECTED: was 67eaeea; F-S2104-P28-M02]`; sole residual `P28-H03` is `story:47`, which is the v1.32 record *describing* the routing request (historical). |
| **M03** | Story EC-009 synced to BC (T-010/RG-010 cite + socket/device-node residual) | **GENUINELY-CLOSED** | `story:173` `(Added with BC-6.26.001 v1.15 M03(a). Coverage: bats T-010/RG-010 verify symlink (type l) and FIFO (type p). **Unproven:** socket (type s; requires a bound process) and device node (types b/c; mknod requires root on Linux).)`; mirrors `BC-6.26.001.md:298`. |
| **M04** | T-008 rationale corrected to name whitespace (not codepoint); ASCII-ellipsis sites normalized | **GENUINELY-CLOSED** | `story-worktree-write-path-discipline.bats:2886-2890` `The load-bearing escape is the intervening whitespace gap, not the Unicode codepoint — both the U+2026 form and the ASCII '...' form evade via the same whitespace mechanism (F-S2104-P29-M04 rationale correction).` |
| **M05** | Word→integer map removed entirely (unreachable `"thirty")` arm eliminated) | **GENUINELY-CLOSED** | `grep -n 'thirty\|twenty-\|Twenty-\|word_to_int\|case .*word' worktree-identity-preflight.bats` → **no matches**. |
| **M06** | `BC-INDEX` v4.38 Refs → `F-S2104-P28-H05` only | **GENUINELY-CLOSED** (Refs leg) | `BC-INDEX.md:8` `... 4-index: BC-INDEX v4.38 / VP-INDEX v2.73 / STORY-INDEX v4.273 / ARCH-INDEX v3.37. Refs: F-S2104-P28-H05, D-943.` — but the same entry's attribution prose is defective; see **F-S2104-P30-M03**. |
| **L01** | T-010 delta-proof legs re-described as POSIX `find` semantics; fixture README claim downgraded | **GENUINELY-CLOSED** | `fixtures/story-worktree/README.md:123-128` `T-010's direct find invocations prove POSIX find semantics (that -type f misses symlinks and FIFOs) but not a property of step-g-cleanup.md §G.1 — re-deriving the retired -type f leg by extracting from the doc is infeasible ... The genuinely behavioral evidence against a predicate-reversion doc-mutant is the harness call via _run_teardown_preflight (F-S2104-P29-L01).` |
| **L02** | Story EC-003/EC-004 swept to quoted plain-path form | **GENUINELY-CLOSED** | `story:167` `find ".worktrees/S-021/.factory" ! -type d` returns empty`; `story:168` same form. |

### Orchestrator-established items

| Item | Verdict | Evidence |
|------|---------|----------|
| AC-001 gate count is **24**, consistent across story / sentinel / runtime count | **PARTIAL** ⚠ | Runtime count = 24 ✓; sentinel `T001_GATE_COUNT=24` ✓ (`:641`); story header says `(24 gates` and tail `story gate count is 24` ✓ — **but** the same cell enumerates only `(1)…(23)` and the bats T-001 summary comment still says `twenty-three`. See **F-S2104-P30-H04** and **H05**. |
| Fabricated input-hash corrected (`1acf3c6` → `47a65c9`) | **PARTIAL** ⚠ | `story:20` `input-hash: "47a65c9"` ✓; `STORY-INDEX.md:727` `input-hash 47a65c9` ✓; **but** `STORY-INDEX.md:731` `S-21.04=4be9d21` (three-way gate broken → **H02**) and `story:47` still terminates its provenance chain at `1acf3c6` with no `[CORRECTED: …]` annotation (→ **M02**). |
| Pass-29 corrected Part A table faithfully describes pass-28's actual findings | **VERIFIED, bounded** | Corroborated without reading pass-28 Part B: band shape B01/H01–H07/M01–M07/L01–L02 = 17 rows matches `STORY-INDEX.md:727` `F-S2104-P28-B01/H01-H07/M01-M07/L01-L02`; row content cross-checks against artifacts — P28-M02 ↔ `story:49` `[CORRECTED: was 67eaeea; F-S2104-P28-M02]`; P28-M03 ↔ `story:296` `16 preflight tests total` (16 `@test` confirmed); P28-M04 ↔ `story:151/214/273/282` `M1–M14`; P28-H05 ↔ `BC-6.26.001.md:39` v1.17 EC-009/T-010 row; P28-L01 ↔ `worktree-identity-preflight.bats:233` adaptive extractor; P28-L02 ↔ `:957..1056` CONTROL legs; P28-M07 ↔ `${line#worktree }` idiom. **No row misdescribes.** Caveat: fidelity is established by artifact corroboration, not by reading pass-28 Part B (forbidden). Note the *derivative* defect: two of these rows' canonical IDs remain un-swept in the bats file (→ **M01**), and the BC-INDEX v4.38 prose contradicts the BC's own v1.18 mapping (→ **M03**). |

**Part A retraction (my own disproved hypothesis):** I initially suspected `grep -cE '^\s+echo "DOC-PARITY FAIL'` would count 0 on BSD grep (macOS) because `\s` is a GNU ERE extension, making T-016's zero-count trap the only reachable path. **Retracted** — the suite is reported 16/16 GREEN at this SHA and the trap would have fired otherwise; the mutant M3 (`zero-count trap`) further proves the trap is reachable and distinguishable, so `\s` resolves in the execution environment. No finding.

---

## 3. Part B — New Findings (independently derived)

### HIGH

---
**F-S2104-P30-H01 — Story body BC table pinned v1.17 while BC is v1.18 and the same story's Token Budget says v1.18 (POLICY 8 / POLICY 5 sibling-category (c); sixth consecutive leg re-stale)**

**Confidence:** HIGH.

**Defect.** BC-6.26.001 advanced to v1.18 in the pass-29 burst. `Token Budget` was swept; the `## Behavioral Contracts` table cell — which POLICY 5 v1.3.3 names explicitly as sibling category (c) "parallel BC body tables that cite the primary BC (e.g., Behavioral Contracts Table version cells)" — was not. The story therefore states two different BC versions in one document.

**Literal evidence.**
```
$ grep -n 'v1\.1[5-9]' .factory/stories/S-21.04-...md   (live-body sites only)
159:| BC-6.26.001 | deliver-story step agents MUST write ... | v1.17 | AC-001 (PC1, Invariant 1), ...
187:| BC-6.26.001 v1.18 | ~3,500 |
$ grep -n '^version:' .factory/specs/behavioral-contracts/ss-06/BC-6.26.001.md
4:version: "1.18"
$ grep -n 'BC-6.26.001' .factory/specs/behavioral-contracts/BC-INDEX.md | tail -1
2053:| [BC-6.26.001](ss-06/BC-6.26.001.md) | ... | draft | CAP-036 | S-21.04 | v1.18 |
```
BC-INDEX leg-5 is correct; the story's own body table is the unswept leg. The pass-29 4-index entry asserts closure: `BC-INDEX.md:8` `(v4.39) — ... BC-6.26.001 v1.17→v1.18 body-table updated (Commit C/D); ... POLICY-14 leg-5 parity complete.` — the claim is true for BC-INDEX and false for the story.

**Files.** `.factory/stories/S-21.04-story-worktree-write-path-discipline.md:159`
**Owner.** story-writer (cell), state-manager (Commit C sweep scope).

---
**F-S2104-P30-H02 — STORY-INDEX aggregation blockquote input-hash is stale (`S-21.04=4be9d21`) while frontmatter and catalog row are `47a65c9` — POLICY 18 THREE-WAY-INPUT-HASH-EQUALITY GATE broken [regression of F-S2104-P28-H03]**

**Confidence:** HIGH.

**Defect.** POLICY 18's three-way gate requires equality across (1) story frontmatter `input-hash:`, (2) STORY-INDEX catalog row, (3) STORY-INDEX aggregation blockquote `S-NNN=`. Legs (1) and (2) were advanced to the hook-authoritative `47a65c9` in the pass-29 correction; leg (3) was not. This is the exact defect F-S2104-P19-009 codified the gate for and F-S2104-P28-H03 closed one pass ago (`04b393e`→`d6d6a6a`).

**Literal evidence.**
```
$ grep -n '^input-hash:' .factory/stories/S-21.04-...md
20:input-hash: "47a65c9"
$ sed -n '727p' .factory/stories/STORY-INDEX.md        (catalog row, excerpt)
| S-21.04 | ... | ready | [BC-6.26.001 v1.18] (wave 2; ... input-hash 47a65c9; ... story v1.33; ...
$ sed -n '731p' .factory/stories/STORY-INDEX.md        (aggregation blockquote, excerpt)
> **E-21 delivery:** ... Input-hashes: S-21.01=32aaccc; S-21.02=8bd32e5; S-21.03=59e687e; S-21.04=4be9d21; S-21.05=c9265f0; S-21.06=b807086. All 6 distinct. [Refreshed D-914; values live in story frontmatter]
```
`4be9d21` is the v1.31-era value. The trailing annotation "values live in story frontmatter" does not exempt the leg — POLICY 18 requires literal equality, and `All 6 distinct` remains true while pointing at the wrong artifact state (the precise failure mode the policy text names).

**Files.** `.factory/stories/STORY-INDEX.md:731`
**Owner.** state-manager.

---
**F-S2104-P30-H03 — Story frontmatter `last_amended:` text-prefix is `(v1.31)` while `version:` is `1.33` — POLICY 14 leg-4 / POLICY 17 parity broken across two version bumps**

**Confidence:** HIGH.

**Defect.** POLICY 14 leg-4 (codified D-490) requires the `last_amended:` TEXT PREFIX to match the current version; POLICY 17 extends the mandate to story files explicitly. Two subsequent bumps (v1.32 at pass-29 burst, v1.33 at the H01 correction) added `modified[]` entries but never advanced `last_amended:`. The prefix date is also stale (`2026-07-29` vs the v1.33 entry's `2026-07-30`).

**Literal evidence.**
```
$ grep -n '^version:\|^last_amended:' .factory/stories/S-21.04-...md
6:version: "1.33"
11:last_amended: "2026-07-29 (v1.31) — pass-28 fix-burst — F-S2104-P28-005: T-016 ...
$ grep -n '^  - "2026-07-30 v1.3' .factory/stories/S-21.04-...md
46:  - "2026-07-30 v1.33: pass-29 F-S2104-P29-H01 — AC-001 gate count corrected 23→24 ...
47:  - "2026-07-30 v1.32: pass-29 fix-burst — F-S2104-P29-H04: phantom ...
```
Sibling-sweep check (POLICY 5 v1.3.4): BC-6.26.001 legs are all correct — `version: "1.18"` / `last_amended: "(v1.18) …"` / `modified[]` tail v1.18 / Changelog top row `| 1.18 |`. The defect is story-local, blast radius 1 file.

**Files.** `.factory/stories/S-21.04-story-worktree-write-path-discipline.md:11`
**Owner.** state-manager (frontmatter parity is leg-4 of the quintuple gate).

---
**F-S2104-P30-H04 — AC-001 Gate cell states "24 gates" but enumerates only (1)…(23); the 24th counted gate (`mandate-sentence absent`) is absent from the enumeration — POLICY 15 NAME-SET-EQUALITY (count-match with divergent set contents)**

**Confidence:** HIGH.

**Defect.** The count leg was advanced 23→24 at v1.33, but the enumerated gate list inside the same table cell still contains 23 numbered members. POLICY 15's NAME-SET-EQUALITY MANDATE exists precisely because "two documents can agree on count while enumerating disjoint sets"; here a single cell disagrees with itself. ADR-034 §Context lists the 24 blocks and names the member the story omits: `mandate-sentence absent` (`story-worktree-write-path-discipline.bats:955`).

**Literal evidence.**
```
$ grep -oE '\((1[0-9]|2[0-9]|[1-9])\) [A-Za-z]{2,}' .factory/stories/S-21.04-...md   # line 110 only
(1) HTML  (2) balanced  (3) anchor  (4) empty  (5) boundary
(6) Gate (7) Gate (8) Gate (9) Gate (10) Gate (11) Gate (12) Gate
(13) Gate (14) Gate (15) Gate (16) Gate (17) Gate (18) Gate
(19) write  (20) Gate (21) Gate (22) Gate (23) canonical
                                     ↑ max index = 23
$ sed -n '110p' … | grep -o 'bats T-001 (24 gates'
bats T-001 (24 gates
$ sed -n '110p' … | grep -o 'story gate count is 24'
story gate count is 24
```
ADR-034 §Context (`ADR-034…md:101-106`): "The 24 blocks span: HTML-comment absence, balanced-fence, anchor uniqueness, prohibition-block absent, boundary-completeness, **mandate-sentence absent**, Gates 1(a)/1(b)/1(c)/1(d)/1(e)/1(f), Gate PW-B, Gate 2a, Gates 2b(a)/2b(c), scope-restriction, Gates 3/4/5/6(a)/7(a), write-directive, and canonical-target." The story's item (4) is the `empty-block guard` (= prohibition-block absent); `mandate-sentence absent` has no number.

**Orchestrator attribution note (persisted verbatim from D-946 recovery context):** This finding was caused by the orchestrator. The 23→24 count correction was driven by a mechanical count (`grep -cE`) without verifying the enumeration list within the same cell. The orchestrator drove the count correction; the enumeration gap was not checked. This attribution is recorded as part of the honest record per D-946 instructions.

**Files.** `.factory/stories/S-21.04-story-worktree-write-path-discipline.md:110`
**Owner.** story-writer.

---
**F-S2104-P30-H05 — bats T-001 summary comment still reads "All twenty-three gates survive independently", and the story asserts the opposite as already-done while also carrying a stale forward-looking deferral (TD-VSDD-060 sibling-site gap + false attestation)**

**Confidence:** HIGH.

**Defect (three legs).**
(a) The count-word sweep updated the T-001 *lead-in* comment but not the *summary* comment 143 lines below it. Both are `Twenty-/twenty-` count-word sites in the same comment block; there are exactly two in the file, and only one was swept.
(b) The story asserts the summary **already reads** twenty-four — a false statement of product-branch state (TD-VSDD-059 paper-attestation class; POLICY 22 narrative-not-authoritative class).
(c) The same cell still carries `bats file to be updated by test-writer in same pass-29 burst` — a forward-looking deferral that survived the burst it references. This is the exact class F-S2104-P27-H02 closed ("stale deferral note removed").

**Literal evidence.**
```
$ grep -n 'Twenty-\|twenty-' .worktrees/S-21.04/plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats
643:  # FORBIDDEN and that canonical absolute paths are MANDATED. Twenty-four independently mutant-proven
786:  # All twenty-three gates survive independently.
$ grep -o 'summary reads `All twenty-four gates survive independently`' .factory/stories/S-21.04-...md
summary reads `All twenty-four gates survive independently`
$ grep -on 'bats file to be updated by test-writer in same pass-29 burst' .factory/stories/S-21.04-...md
110:bats file to be updated by test-writer in same pass-29 burst
```
Root cause is upstream: ADR-034's Downstream Routing table (`ADR-034…md:368-371`) enumerates only the story cell and *one* bats comment ("Twenty-three independently mutant-proven gates"), never the summary line — so the sweep-site enumeration was incomplete by construction (see **F-S2104-P30-L02**).

**Files.** `.worktrees/S-21.04/plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats:786`; `.factory/stories/S-21.04-story-worktree-write-path-discipline.md:110`
**Owner.** test-writer (bats line 786), story-writer (story cell legs b/c).

---
**F-S2104-P30-H06 — Four story T-016 inventory descriptions still describe the retired cross-branch / count-word semantics — POLICY 14 SAME-BURST PREDICATE-GATE CELL COUPLING**

**Confidence:** HIGH.

**Defect.** ADR-034 v1.1 removed the story read and the count-word comparison from T-016 entirely. The AC-001 Gate cell received an `ADR-034 v1.1 note`, but all four sibling inventories still tell an implementer that T-016 compares the story's stated gate count with the bats lead-in count-word — a predicate that no longer exists. POLICY 14's coupling mandate requires story cells to use "the final T-N predicate forms". Blast radius 4 sites → HIGH per the partial-fix rubric.

**Literal evidence.**
```
$ grep -n 'count-word' .factory/stories/S-21.04-...md      (live-body sites)
151: ... T-016 `test_coupling_gate_story_gate_count_matches_bats_count_word` — mechanical coupling gate
     enforcing story gate-count parity with bats lead-in count-word (AC-001; F-S2104-P28-H04)
215: 16. [x] ... mechanical coupling gate — asserts the story's stated gate count for AC-001 T-001
     matches the bats lead-in count-word in T-001 ...; enforces the D-922/P19-008(b) same-burst
     coupling mandate as an automated regression; any mismatch turns T-016 RED
274: | T-016 | AC-001 (count-word coupling gate) | ... asserts the story's stated gate count for T-001
     matches the bats lead-in count-word in T-001 ...; any burst changing the gate count without
     updating both surfaces turns this test RED (D-922/P19-008(b); F-S2104-P28-H04)
296: ... T-016 ... added — mechanical count-word coupling gate (AC-001; F-S2104-P28-H04); 16 preflight
```
Ground truth: `worktree-identity-preflight.bats:1127-1128` `# ADR-034 Decision 1: No .factory/ reads — both operands on the product branch. #   Removed: fa_wt discovery loop, story_file, story_count, word-to-integer map.` Story line 274's clause "any burst changing the gate count without updating both surfaces turns this test RED" is now **false**: the story cell is documentation-only.

**Files.** `.factory/stories/S-21.04-story-worktree-write-path-discipline.md:151, 215, 274, 296`
**Owner.** story-writer.

---
**F-S2104-P30-H07 — ARCH-INDEX ADR-034 registry row is labeled "ADR-034 v1.1" but carries v1.0's Decision 2 and Decision 3 content, including the counting surface ADR v1.1 explicitly forbids — POLICY 14 leg-5 content-correspondence failure**

**Confidence:** HIGH.

**Defect.** ADR-034 v1.1's entire purpose was correcting the counting surface. The ARCH-INDEX row still prescribes the wrong surface (`_assert_doc_marker` call count) and the wrong sentinel placement, while asserting `**PROPOSED 2026-07-30; ADR-034 v1.1.**`. Leg 5 is *present* (the row exists, the version token says v1.1) but its content corresponds to the superseded ruling — exactly the "presence, not correspondence" failure mode the prompt flags as re-staled five consecutive times. An implementer reading ARCH-INDEX would implement the forbidden predicate.

**Literal evidence.**
```
$ sed -n '539p' .factory/specs/architecture/ARCH-INDEX.md   (excerpt)
| ADR-034 | ... Decision 2 — `bats_count` replaced with runtime-derived assertion count (grep-count of
`_assert_doc_marker` calls within T-001's body in `story-worktree-write-path-discipline.bats`; fail-loud
when count is zero ...); Decision 3 — `story_count` replaced with product-branch sentinel constant
`T001_GATE_COUNT=N` immediately preceding T-001's first assertion in the bats suite; ...
Downstream routing: story-writer amends S-21.04 AC-001 T-016 operand description; product-owner reviews
BC-6.26.001 for any invariant requiring T-016 to read the story document from factory-artifacts.
**PROPOSED 2026-07-30; ADR-034 v1.1.** | SS-05, SS-06 | decisions/ADR-034-...md |
```
Contradicted by the ADR itself:
```
$ grep -n 'Do NOT count' .factory/specs/architecture/decisions/ADR-034-...md
165:**Do NOT count `_assert_doc_marker` or `_assert_no_doc_marker` calls.** Those helper
$ sed -n '168,169p' ADR-034-...md
A count of `_assert_doc_marker` calls gives 21 (§G.1 + Primary-paths), which is neither the
pre-pass-26 gate count nor the post-pass-26 gate count and must not be used.
$ sed -n '182,184p' ADR-034-...md
on its own line at the head of the AC-001(a) Write Discipline block, immediately
after the `# --- DOC-PARITY §Spec-Path Discipline: AC-001(a)` opening marker and
before the first gate code:
$ sed -n '371p' ADR-034-...md
| `BC-6.26.001` | No changes required — BC does not specify the coupling gate mechanism or gate count | — |
```
The row's routing cell also still assigns product-owner a BC review that ADR v1.1 closed as "No changes required."

**Files.** `.factory/specs/architecture/ARCH-INDEX.md:539`
**Owner.** state-manager (index row), architect (content authority for the summary text).

---
**F-S2104-P30-H08 — Two consecutive gate-hardening waves shipped with no red-gate-log assertion-site attestation, no pass row, and a stale Summary HEAD — POLICY 15 ATTESTATION-LOCATION + SAME-AC GATE AUDIT + NAME-SET-EQUALITY all unsatisfied at `44547051`**

**Confidence:** HIGH.

**Defect.** POLICY 15's ATTESTATION-LOCATION GATE: "a fix wave that adds or strengthens any bats assertion site MUST NOT be pushed until the matching red-gate-log.md attestation section EXISTS at that commit … `grep -c 'assertion-site attestation (<HEAD-SHA>)' red-gate-log.md` → 1 … if absent, flag as POLICY 15 HIGH." Pass-28 (T-008 predicate-agnostic widening; T-010/RG-010 registration) and pass-29 (full T-016 rewrite with 3 new mutant probes; T-008 position-agnostic leg + count-closure leg + 7 named mutant probes) both added and strengthened assertion sites. Neither produced an attestation section. Consequently the pass-29 wave also has **no obligation-indexed AC-001 coverage table** (POLICY 15 SAME-AC GATE AUDIT) and **no name-set-equality diff** for the 23-vs-24 gate label set — which is why F-S2104-P30-H04 survived.

**Literal evidence.**
```
$ grep -n 'assertion-site attestation' red-gate-log.md | tail -3
2285:### Pass-26 assertion-site attestation (7c3338e7)
2355:### Pass-27 assertion-site attestation (fix-burst at 7c3338e7; 2026-07-28)
$ grep -cn '44547051\|eba02788' red-gate-log.md
0
$ grep -on 'Pass-28\|Pass-29' red-gate-log.md
2762:Pass-28                       ← prose only ("Pass-28 may scrutinize CONTROL-block completeness")
$ grep -n '^| Pass-2[0-9]' red-gate-log.md | tail -1
1514:| Pass-27 | `c7c61688` | B01 CLOSED ... | 10/10 + 16/16 | ...
$ sed -n '60p' red-gate-log.md   (excerpt — Summary row)
| S-21.04 | 10 bats tests: ... All GREEN at worktree HEAD c7c61688 (26/26: 10/10 + 16/16, 2026-07-28).
... Per-pass closure checklist: advancing Summary HEAD to the post-fix fixes-HEAD is a mandatory closure
step whenever an attestation section is appended. |
$ grep -c 'RG-010' red-gate-log.md
0
$ grep -n '^version:\|^traces_to:' red-gate-log.md
4:version: "1.27"
13:traces_to: "BC-6.26.001 v1.14; BC-6.26.001 v1.15; story v1.20; ... story v1.30"
```
Sub-legs: (a) no `### Pass-28/29 assertion-site attestation (<SHA>)` section; (b) no Pass-28/Pass-29 summary-table row; (c) Summary HEAD stale at `c7c61688` — the log's own text calls advancing it "a mandatory closure step"; (d) `RG-010`, registered in the story's §Red Gate Test Plan at pass-28, has **zero** occurrences in the red-gate-log, which is the RG source-of-truth; (e) frontmatter `traces_to` stops at story v1.30 / BC v1.15 (current: v1.33 / v1.18). This is a recurrence of the F-S2104-P22-004 class ("three-pass omission … one pass after D-933 closed this class").

**Files.** `.factory/cycles/v1.0-brownfield-backfill/S-21.04/implementation/red-gate-log.md` (frontmatter; `:60`; `:1514` table tail; end-of-file)
**Owner.** test-writer (attestation content), state-manager (log persistence + Summary HEAD + frontmatter parity). `[process-gap]` — POLICY 15's gate is mandated but nothing mechanically blocks the push; two consecutive waves cleared it.

---
**F-S2104-P30-H09 — Test ID `T-010` denotes two different tests: the pipeline probe (allocated pass-24, per red-gate-log SoT) and the EC-009 stray-inode test (allocated pass-28, per story) — POLICY 1 append-only ID reuse**

**Confidence:** HIGH.

**Defect.** POLICY 1: "All VSDD identifiers are never renumbered or reused." `T-010` was allocated at pass-24 (D-935) to the five-leg pipeline probe and is recorded that way in the red-gate-log. At pass-28 the story allocated `T-010` to the new EC-009 stray-inode test and demoted the pipeline probe to an unnumbered row. Both allocations are live in different documents, so `T-010` is now ambiguous in every cross-document cite (`RG-010`, `story:256`, `BC-6.26.001.md:311` T-010 vector row, README EC-coverage table).

**Literal evidence.**
```
$ sed -n '60p' red-gate-log.md   (excerpt)
| S-21.04 | 10 bats tests: T-001..T-006 (behavioral vectors) + T-007/T-008/T-009 (doc-parity
propagation gates) + T-010 (pipeline probe; F-S2104-P24-003) + T-015 ... + T-016 ...
$ sed -n '1510p' red-gate-log.md   (excerpt — Pass-24 row)
| Pass-24 | `9b12aa00` | ... F-S2104-P24-003 CLOSED (pipeline probe T-010 — call-site parity Legs A/B/C/D) ...
$ grep -n 'last_amended' red-gate-log.md | grep -o 'pipeline probe T-010 added at 9b12aa00'
pipeline probe T-010 added at 9b12aa00
$ sed -n '263p;272p' .factory/stories/S-21.04-...md   (excerpts)
| Pipeline Probe | AC-001 (write-directive gate + Gate PW-B domain-construction path; ...) | ... |
| T-010 | AC-002 (EC-009) | ... | `S-21.04 EC-009: stray-inode-inside-factory — symlink + FIFO inside
  real .factory/ dir → PREFLIGHT BLOCKED via ! -type d; missed by -type f predicate (M03(a) delta proof)` |
```
Also note the derived count divergence: red-gate-log `:60` says "10 bats tests" while the suite has 11 `@test` blocks and the story says `11 tests total` (`story:149, 282`) — the eleventh is the test that lost its ID.

**Files.** `.factory/cycles/v1.0-brownfield-backfill/S-21.04/implementation/red-gate-log.md:60, 1510`; `.factory/stories/S-21.04-story-worktree-write-path-discipline.md:263, 272`
**Owner.** orchestrator adjudication (which allocation is canonical), then story-writer + state-manager. POLICY 1 forbids reuse, so the newer allocation should move to `T-011` unless the human rules otherwise.

---

### MEDIUM

---
**F-S2104-P30-M01 — Six residual phantom pass-28 numeric IDs (`F-S2104-P28-016`, `F-S2104-P28-017`) survive in the bats file the M01 sweep claimed to cover [regression / sibling-site gap]**

**Confidence:** HIGH (evidence), MEDIUM (severity: blast radius 1 file).

**Defect.** Pass-29 M01 claimed the numeric→house-form remap covered "tests, rules, skills, workflows, `agents/adversary.md`". Six numeric-form IDs remain in `tests/`, and neither `016` nor `017` exists in the authoritative pass-28 set (`B01, H01–H07, M01–M07, L01, L02` per `STORY-INDEX.md:727`). Canonical targets are derivable from pass-29 Part A: `016` → `F-S2104-P28-L01` (depth-adaptive `_guard_l_off_limits` extractor), `017` → `F-S2104-P28-L02` (M10–M14 CONTROL legs).

**Literal evidence.**
```
$ grep -rn 'F-S2104-P28-0[0-9][0-9]' .worktrees/S-21.04/plugins
tests/worktree-identity-preflight.bats:233:  # F-S2104-P28-016 + coordinator correction: adaptive section-bounded extractor
tests/worktree-identity-preflight.bats:957:  # F-S2104-P28-017: CONTROL leg added to make restore-attestation true as written.
tests/worktree-identity-preflight.bats:991:  # F-S2104-P28-017: CONTROL leg added to make restore-attestation true as written.
tests/worktree-identity-preflight.bats:1013:  # F-S2104-P28-017: CONTROL leg added to make restore-attestation true as written.
tests/worktree-identity-preflight.bats:1034:  # F-S2104-P28-017: CONTROL leg added to make restore-attestation true as written.
tests/worktree-identity-preflight.bats:1056:  # F-S2104-P28-017: CONTROL leg added to make restore-attestation true as written.
```
**Files.** `.worktrees/S-21.04/plugins/vsdd-factory/tests/worktree-identity-preflight.bats:233, 957, 991, 1013, 1034, 1056`
**Owner.** test-writer.

---
**F-S2104-P30-M02 — Story provenance chain terminates at the fabricated input-hash `1acf3c6` with no error-acknowledgment annotation [regression of F-S2104-P28-M02]**

**Confidence:** HIGH.

**Defect.** `1acf3c6` was authored without a `compute-input-hash` invocation and is falsified (`--check` → `47a65c9`). The frontmatter was corrected, but the `modified[]` provenance chain still presents `1acf3c6` as the terminal hash, and the v1.33 entry — the entry that records the correction burst — is silent on input-hash entirely. The repository's established remedy for exactly this is the inline `[CORRECTED: …]` annotation, applied one pass ago for `67eaeea`.

**Literal evidence.**
```
$ sed -n '47p' .factory/stories/S-21.04-...md | grep -o 'input-hash 4be9d21→1acf3c6[^"]*'
input-hash 4be9d21→1acf3c6 (BC-6.26.001 updated by concurrent product-owner agent)
$ sed -n '49p' .factory/stories/S-21.04-...md | grep -o 'input-hash 04b393e→d6d6a6a \[CORRECTED[^]]*\]'
input-hash 04b393e→d6d6a6a [CORRECTED: was 67eaeea; F-S2104-P28-M02]
$ sed -n '46p' .factory/stories/S-21.04-...md | grep -c 'input-hash'
0
$ grep -n '^input-hash:' .factory/stories/S-21.04-...md
20:input-hash: "47a65c9"
```
**Files.** `.factory/stories/S-21.04-story-worktree-write-path-discipline.md:46, 47`
**Owner.** state-manager.

---
**F-S2104-P30-M03 — BC-INDEX v4.38 entry attributes BC-6.26.001 v1.16/v1.17 to the wrong pass-28 findings, contradicting BC-6.26.001 v1.18's authoritative namespace-correction record**

**Confidence:** HIGH (evidence), MEDIUM (severity).

**Defect.** BC v1.18 is the authoritative namespace-correction record: `F-S2104-P28-006` → `H05` (which drove **v1.17**), `F-S2104-P28-009` → `M01` (which drove **v1.16**). The BC-INDEX v4.38 entry instead attributes v1.16 to H05 and v1.17 to H07 — H07 being the fixture-README fix, which changed no BC content. When the BC's namespace was corrected at pass-29, this sibling index cell was not swept (POLICY 5 sibling-sweep; POLICY 22 attribution-fidelity).

**Literal evidence.**
```
$ grep -o '(v4.38)[^[]\{0,600\}' .factory/specs/behavioral-contracts/BC-INDEX.md
(v4.38) — D-943 S-21.04 pass-28 sweep record: BC-5.39.008 v1.5→v1.6 ...; BC-6.26.001 v1.15→v1.17
(product-owner H05 v1.16 EC-009 structural rationale + H07 v1.17 T-010/RG-010 cross-ref; state-manager
Commit C body-table sweep). ... Refs: F-S2104-P28-H05, D-943.
$ sed -n '40p' .factory/specs/behavioral-contracts/ss-06/BC-6.26.001.md   (excerpt)
"2026-07-30 (v1.18) — ... Canonical mappings: `F-S2104-P28-006` → `F-S2104-P28-H05` (EC-009 had no test
or Red Gate row); `F-S2104-P28-009` → `F-S2104-P28-M01` (frontmatter modified-array ordering)."
$ sed -n '38,39p' .factory/specs/behavioral-contracts/ss-06/BC-6.26.001.md   (excerpts)
"2026-07-29 (v1.16) — ... frontmatter modified-array ordering ..."
"2026-07-29 (v1.17) — ... EC-009 test-coverage cross-reference ... T-010 row added ..."
```
**Files.** `.factory/specs/behavioral-contracts/BC-INDEX.md:8` (v4.38 entry)
**Owner.** state-manager.

---
**F-S2104-P30-M04 — Story carries a live load-bearing `ADR-034 v1.1` version pin while `anchored_adrs:` and Token Budget omit ADR-034 entirely [regression of the F-P3-017 ADR-pin class]**

**Confidence:** HIGH (evidence), MEDIUM (severity).

**Defect.** Two coupled anchoring defects. (a) The AC-001 Gate cell's normative escape clause is gated on a version-pinned ADR cite; this cascade already normalized exactly this class in this file (v1.6: "Token Budget ADR-031 v1.3 pin removed; TD-VSDD-060 ADR-031 sweep complete (one active version pin normalized)"). Stable form is `ADR-034 §Decision 1/2/3`. (b) ADR-034 now governs a live story obligation (the gate-count cell's documentation-only status), yet the story's `anchored_adrs:` lists only ADR-031 and the Token Budget prices only ADR-031 — so an implementer loading the story's declared inputs never loads the ADR that redefines the gate.

**Literal evidence.**
```
$ grep -on 'ADR-034[^;)]\{0,60\}' .factory/stories/S-21.04-...md
46:ADR-034 v1.1: this story gate-count cell is now documentation-only — T-016 no longer re   ← modified[] (historical)
110:ADR-034 v1.1 note:** this story gate-count cell is documentation-only — T-016 no longer  ← LIVE Gate cell
$ grep -n '^anchored_adrs:' .factory/stories/S-21.04-...md
40:anchored_adrs: [ADR-031]
$ sed -n '188p' .factory/stories/S-21.04-...md
| ADR-031 (§Decision 4) | ~1,500 |
```
**Files.** `.factory/stories/S-21.04-story-worktree-write-path-discipline.md:40, 110, 188`
**Owner.** story-writer.

---
**F-S2104-P30-M05 — ADR-034 `anchors:`/`subsystems_affected:` name SS-05 and SS-06, neither of which owns the modules the ADR constrains (`plugins/vsdd-factory/tests/`, `.github/workflows/ci.yml`) — POLICY 4 / POLICY 6 mis-anchor**

**Confidence:** MEDIUM.

**Defect.** ARCH-INDEX is the canonical owner registry. SS-05 owns `plugins/vsdd-factory/agents/` + `workflows/`; SS-06 owns `plugins/vsdd-factory/skills/`. ADR-034 rules exclusively on a bats suite under `plugins/vsdd-factory/tests/` and a required job in `.github/workflows/ci.yml` — neither path appears in any SS row. Per the mis-anchoring rubric this cannot be deferred: either ARCH-INDEX must register an owner for `tests/`/CI (the sibling precedent is ADR-030, which anchored SS-10 for `bin/` and SS-07 for the registry), or the ADR's anchors must be corrected.

**Literal evidence.**
```
$ grep -n '^anchors:' -A3 .factory/specs/architecture/decisions/ADR-034-...md
18:anchors:
19:  - SS-05
20:  - SS-06
$ grep -n '^| SS-0[56] |' .factory/specs/architecture/ARCH-INDEX.md | head -2
439:| SS-05 | Pipeline Orchestration | ... | `plugins/vsdd-factory/agents/`, `plugins/vsdd-factory/workflows/*.lobster`, `plugins/vsdd-factory/workflows/phases/` | BC-5 | ...
440:| SS-06 | Skill Catalog | ... | `plugins/vsdd-factory/skills/` (119 skills, 581 markdown files) | BC-6 | ...
$ grep -c 'plugins/vsdd-factory/tests' .factory/specs/architecture/ARCH-INDEX.md   # registry rows 435-444
0
$ sed -n '355,357p' .factory/specs/architecture/decisions/ADR-034-...md
- **`bats-full-suite (linux)` job** in `.github/workflows/ci.yml` — the required CI job
  running `run-all.sh` with `SKIP_SUITES=()` (empty, non-skippable) on every PR to
  `main` or `develop`.
```
**Files.** `.factory/specs/architecture/decisions/ADR-034-…md:18-23`; `.factory/specs/architecture/ARCH-INDEX.md:433-444`
**Owner.** architect (anchor correction or registry extension).

---

### LOW

---
**F-S2104-P30-L01 — `test_coupling_gate_story_gate_count_matches_bats_count_word` is a misnomer post-ADR-034: it reads neither the story nor any count-word (pending intent verification)**

**Confidence:** HIGH (evidence), LOW (severity).

**Defect.** The test's name is a load-bearing anchor cited at five story sites and in the red-gate-log. After the v1.1 rewrite it compares a product-branch sentinel against a runtime DOC-PARITY FAIL count — no story read, no count-word. ADR-034 §Decision 4 states the purpose shift explicitly ("from 'story document agrees with bats prose token' to 'declared constant in bats suite agrees with actual assertion count'") but routed no rename, presumably to avoid breaking cites. Tagged `(pending intent verification)` per the intent-adjudication rule: a rename has real coupling cost and only the orchestrator/human can rule.

**Literal evidence.**
```
$ sed -n '1142p' .worktrees/S-21.04/plugins/vsdd-factory/tests/worktree-identity-preflight.bats
@test "test_coupling_gate_story_gate_count_matches_bats_count_word" {
$ sed -n '1143p;1145p' … 
  # ADR-034 v1.1 (Decisions 1-3): rewritten. No .factory/ access.
  bats_suite="$PLUGIN_ROOT/tests/story-worktree-write-path-discipline.bats"
$ sed -n '214,217p' .factory/specs/architecture/decisions/ADR-034-...md
The gate's purpose shifts from "story document agrees with bats prose token" to
"declared constant in bats suite agrees with actual assertion count."
```
**Files.** `.worktrees/S-21.04/plugins/vsdd-factory/tests/worktree-identity-preflight.bats:1142`
**Owner.** orchestrator (adjudication) → test-writer + story-writer if renamed.

---
**F-S2104-P30-L02 — ADR-034 §Downstream Routing under-enumerates the count-word sweep sites, which is the proximate cause of F-S2104-P30-H05 `[process-gap]`**

**Confidence:** MEDIUM.

**Defect.** The routing table names exactly one bats comment site, so a sweep executed faithfully against the ADR still leaves `story-worktree-write-path-discipline.bats:786` stale. A propagation directive that enumerates sweep sites must enumerate *all* occurrences of the changed token (POLICY 5 sibling-sweep + its literal-shell verification gate: the enumerating agent should have run `grep -n 'Twenty-\|twenty-' <file>` — which returns two lines — before writing the table).

**Literal evidence.**
```
$ sed -n '368,370p' .factory/specs/architecture/decisions/ADR-034-...md
| Story `S-21.04-...md` AC-001 gate count cell | Update from "(twenty-three gates)" to "(twenty-four gates)" ... | story-writer |
| `story-worktree-write-path-discipline.bats` T-001 inline comment | Update "Twenty-three independently mutant-proven gates" to "Twenty-four" (same commit as T-016 rewrite) | test-writer |
$ grep -n 'Twenty-\|twenty-' .worktrees/S-21.04/plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats
643:  # ... Twenty-four independently mutant-proven
786:  # All twenty-three gates survive independently.     ← not routed
```
**Files.** `.factory/specs/architecture/decisions/ADR-034-…md:368-371`
**Owner.** architect. `[process-gap]` — ADR propagation directives are not gated on literal-shell site enumeration.

---

### Self-validation record (3 iterations, per AgenticAKM cap)

- **Iteration 1 (evidence):** dropped a hypothesis that `grep -cE '^\s+…'` is BSD-incompatible (retracted above — GREEN suite falsifies it). Dropped a hypothesis that T-008's third leg false-positives on backtick-wrapped annotation prose (no live site; speculative).
- **Iteration 2 (actionability):** rewrote a vague "T-016 counts diagnostics, not assertions" concern into nothing — on inspection every one of the 24 `echo "DOC-PARITY FAIL"` lines is immediately followed by `false` (verified at `:797`, `:814`, `:923`, `:956`, `:962`, `:971`, `:977`, `:985`, `:995`, `:1005`), so the count is a faithful proxy at HEAD. **Not reported.**
- **Iteration 3 (duplication):** merged a separate MEDIUM about the stale "to be updated by test-writer in same pass-29 burst" clause into **H05** leg (c), same sentence-region as the count-word claim. Merged a separate finding about red-gate-log's "10 bats tests" count into **H09** as derived evidence.

---

## Summary

| Severity | Count |
|----------|-------|
| BLOCKER | 0 |
| HIGH | 9 |
| MEDIUM | 5 |
| LOW | 2 |

**Total findings:** 16 (B0/H9/M5/L2)

**Overall Assessment:** NOT-CLEAN — second consecutive zero-BLOCKER pass; H04 orchestrator-attributed (AC-001 "24 gates" enumerates only (1)…(23)); H08 two-pass red-gate-log attestation gap (`[process-gap]`); H09 T-010 ID collision (POLICY 1 first violation of this class); convergence blocked by persistent same-burst propagation deficit across story body table / STORY-INDEX / ARCH-INDEX / red-gate-log. Implementation surface (T-001/T-008/T-009/T-010/T-016) verified GREEN and structurally sound at this HEAD.

**Convergence:** findings remain — iterate. Streak: 0/3 (zero BLOCKERs; streak advances only on NITPICK-OR-CLEAN verdict). Structural pattern: bookkeeping propagation is the dominant failure class (H01/H02/H03/H05/H07/H08 — 6 of 9 HIGH findings).

**Readiness:** requires revision. Pass-31 NEXT after pass-30 fix burst.

**Pass-29 closure assessment:** 10 GENUINELY-CLOSED · 3 PARTIAL (M01 six residual numeric IDs; AC-001 24-gate count/enumeration parity; fabricated hash provenance chain).

---

## 5. Novelty Assessment

**Novelty: HIGH.**

Reasoning: nine of the sixteen findings target artifact classes that the cascade's recent passes demonstrably did not audit at this HEAD, and three are structurally new axes:

- **H08** (POLICY 15 attestation-location) is a *two-pass-deep* gap: the red-gate-log's last attestation is pass-27 and its Summary HEAD is `c7c61688`, meaning passes 28 and 29 both converged without the artifact POLICY 15 designates as the primary coverage proof. That the log itself contains the sentence "advancing Summary HEAD … is a mandatory closure step" makes it self-falsifying.
- **H09** (T-010 ID collision) is a genuinely new axis — an ID-reuse defect visible only by reading the red-gate-log's pass-24 row against the story's pass-28 §Test Plan. No count-based or version-based check can see it.
- **H07** (ARCH-INDEX row carries v1.0 content under a v1.1 label) is the leg-5 correspondence failure the prompt predicted would recur — and it recurred in the *same burst* that declared "POLICY-14 leg-5 parity complete".
- **H04/H05** are the immediate downstream consequence of pass-29's own H01 fix: the count leg was advanced, the two *set-enumeration* legs (story numbering, bats summary) were not. The count/enumeration divergence is the pattern POLICY 15's NAME-SET-EQUALITY MANDATE was written for, applied for the first time to a single self-contradicting cell rather than two documents.
- **H02** and **M02** are literal regressions of pass-28 closures (H03, M02) one pass later — the recurrence interval is now one pass, not three.

Convergence assessment: the *implementation* surface is stabilizing (T-016, T-008, T-010, fixture README, BC body all verified genuine; suites 11/11 + 16/16 with parity 24 == 24). The failures are concentrated entirely in **propagation and attestation bookkeeping** — story inventories, index cells, provenance chains, and the red-gate-log. This is not spec convergence; it is a systematic same-burst-propagation deficit that no mechanical gate currently covers (`lint_hook: null` on POLICY 14, 15, 18, and 1).

---

## 6. Completeness Statement

**Audited.**
- POLICY 1 (T-ID reuse; phantom finding IDs), 4 (ADR/subsystem anchoring; description-bearing anchor prose), 5 (sibling-sweep categories (a)–(j); historical-by-construction enumeration; stable anchors), 6 (SS-06 = "Skill Catalog" verbatim ✓), 7 (BC H1 ↔ BC-INDEX title verbatim parity ✓; story BC-table Title is permitted editorial abbreviation ✓), 8 (`behavioral_contracts: [BC-6.26.001]` ↔ body table ↔ AC traces ↔ Token Budget count — membership ✓, version cell ✗ → H01), 13 (alternation-widening direction statements present in T-008's new leg ✓), 14 (all five legs, both BC and story), 15 (attestation-location, same-AC obligation audit, name-set equality), 16 (D-943/D-944 exist in the cited-at commits ✓ by index cross-reference), 17 (story-level frontmatter parity → H03), 18 (three-way input-hash equality → H02), 19 (ADR version pins → M04), 21 (no new `.sh`; story is skill-doc + bats only ✓), 22 (attribution fidelity → M03; pass-29 Part A corroboration).
- Full text of the story, BC-6.26.001, ADR-034; T-016 helper + test + 3 mutants; T-008 helper (all five legs + escape + count closure); T-001 AC-001(a) region (gate-by-gate for the 24-block recount); fixture README; red-gate-log frontmatter/Summary/pass-table/tail; BC-INDEX + STORY-INDEX + VP-INDEX + ARCH-INDEX relevant cells.
- Independent mechanical recount of the AC-001(a) DOC-PARITY FAIL blocks (24) and both `@test` inventories (11 main, 16 preflight).

**Explicitly NOT audited.**
- `crates/hook-plugins/validate-policies-schema/` and commit `eba02788` — **out of perimeter per dispatch** (BC-5.39.008 scope, pending human decision). Not read, not counted.
- Part B / Fix Mapping / Summary / Novelty / Completeness of `adversary-pass-29.md`; all of passes 01–28; cycle `INDEX.md`; STATE.md Session Resume Checkpoint — **information-asymmetry perimeter**.
- Full bodies of the implementation surfaces `_shared-context.md`, `step-g-cleanup.md`, `SKILL.md`, `per-story-delivery.md` (×2), `worktree-manage/SKILL.md`, `code-delivery/SKILL.md`, `fix-pr-delivery/SKILL.md`, `code-delivery.lobster`, `greenfield.lobster`, `rules/worktree-protocol.md`, `agents/adversary.md`, `agents/devops-engineer.md`, `skills/adversarial-review/SKILL.md`, `step-d5-adversary-convergence.md` — **not re-read this pass**. Rationale: BC-6.26.001 v1.18 changed no normative predicate (its own changelog: "No normative content changed"), and these surfaces are mechanically gated by T-001/T-007/T-008/T-009 which are GREEN at this SHA. Their prose was audited only where a gate or ADR referenced them. A future pass reading these bodies directly for prose drift outside gate coverage remains unbudgeted here.
- `.github/workflows/ci.yml` — not read; ADR-034's claims about the `bats-full-suite (linux)` job configuration (`SKIP_SUITES=()`, required status) are **taken as asserted, not verified**. A CI-as-Code positive-coverage audit of that job (per the CI-as-Code review axis) was not performed and should be scoped for a future pass — noting that T-016's zero-count trap is the relevant false-green guard and it *is* mutant-proven (M3).
- `bin/compute-input-hash` was not invoked (read-only tool profile); the `47a65c9` value is accepted on the orchestrator's literal `--check` evidence, and only the three-way *equality* was audited.
- Cargo workspace, dispatcher, and all non-S-21.04 stories/BCs/VPs — outside perimeter.
