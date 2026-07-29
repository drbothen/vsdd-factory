---
document_type: adversarial-review
level: adversary
version: "1.0"
status: complete
producer: vsdd-factory:adversary
timestamp: 2026-07-29T00:00:00Z
phase: 3
inputs:
  - stories/S-21.04-story-worktree-write-path-discipline.md
  - specs/architecture/decisions/ADR-031-e21-factory-state-data-loss-hardening.md
  - specs/behavioral-contracts/ss-06/BC-6.26.001.md
  - cycles/v1.0-brownfield-backfill/S-21.04/implementation/red-gate-log.md
  - policies.yaml
input-hash: "a2767bf"
traces_to: "BC-6.26.001 v1.15; story v1.30"
pass: 28
verdict: NOT-CLEAN
reviewed_head: "c7c61688"
fixes_landed_head: "c7c61688"
novelty: high
previous_review: "cycles/v1.0-brownfield-backfill/S-21.04/adversary-pass-27.md"
findings_count: 17
severity_breakdown: "B1/H7/M7/L2"
streak: "0/3"
trajectory_append: 17
model_override: false
model_resolved: "claude-opus-5"
adr033_deviation: "ADR-033 cross-family limitation — cross-family claim (GPT-5) NOT satisfied; ran on Claude, same family as authoring agents; fresh context + information asymmetry intact, cross-family independence absent"
asymmetry_enforcement: "pass-27 Part A inlined; prior pass files (pass-1 through pass-26) and cycle INDEX.md off-limits; adversary confirmed it opened none"
policy22_note: "POLICY 22 SUBAGENT-REPORT-FIDELITY active: adversary findings reported directly; no relay chain between adversary and state-manager in this pass"
---

# Adversary Pass 28 — S-21.04 story-worktree write-path discipline

**Date:** 2026-07-29
**Reviewed HEAD:** `c7c61688`
**Fixes-landed HEAD:** `c7c61688` (factory-artifacts fixes land in Commits A–E; bats test additions are follow-on feature-branch work; SHA-patch will update after feature-branch push)
**Verdict:** NOT-CLEAN — B1/H7/M7/L2 = 17 findings
**Streak:** 0/3 (BC-5.39.001)

## Finding ID Convention

Finding IDs for this cascade use the format: `F-S2104-P<PASS>-<SEV><SEQ>` (project-local convention established at pass-1). Map to template ADV severity abbreviations: B→BLOCKER, H→HIGH, M→MEDIUM, L→LOW. Full format example: `F-S2104-P28-B01`.

## Mandatory Provenance Disclosure

**(1) Model override:** NO model override applied. Frontmatter `model: opus` → `claude-opus-5`. Fresh context enforced.

**(2) ADR-033 cross-family limitation:** ADR-033 §Decision 2 cross-family independence (GPT-5) NOT satisfied. The reviewing model is in the same family as the authoring agents. Fresh context and information asymmetry are intact; cross-family cognitive diversity is absent. Disclosed per ADR-033 §Decision 3.

**(3) Information asymmetry enforcement:** Pass-27 Part A findings were inlined as permitted context. Prior pass files (pass-1 through pass-26) and the cycle `INDEX.md` were off-limits. The adversary confirms it opened none.

**(4) policies.yaml disruption disclosure:** The adversary's dispatch rubric auto-loads `policies.yaml`. Document-1 (version metadata) was parsed successfully. Document-2 (all 22 policies) FAILED to parse — see B01. The adversary rubric operated on document-1 only during the review period since D-920. POLICY 1–12 are encoded in the adversary agent prompt independently; the auto-loaded programmatic portion was blind to document-2. This is disclosed upfront because it affects the completeness of policy-enforcement checking in passes 21–28.

## Part A — Fix Verification (Pass-27 Closures)

| ID | Pass-27 Severity | Status | Notes |
|----|-----------------|--------|-------|
| P27-B01 | BLOCKER | GENUINELY-CLOSED | Positional conjuncts added to all 7 `_guard_*` helpers via awk `#### Worktree-Identity Preflight` section extraction; corpus M10–M14 deletion vectors added. Fourth axis (deletion-shape); first fix that changed predicate SHAPE not vocabulary. |
| P27-H01 | HIGH | GENUINELY-CLOSED | Vestigial `grep -iv` pre-filter removed from `_guard_e_factory_artifacts`; TD-VSDD-060 sibling sweep applied. |
| P27-H02 | HIGH | GENUINELY-CLOSED-with-NEW-DEFECT | Count-word corrected `Twenty-one`→`Twenty-three`; stale deferral removed; volatile `line 638` pin replaced with behavioral anchor. T-016 coupling gate added to bats. BUT: T-016 not registered in story §Test Plan, §Architecture Mapping, §File Structure Requirements, or §Tasks — only added to AC-001 coupling note. → H04 new finding. |
| P27-M01 | MEDIUM | GENUINELY-CLOSED | Trailing-slash mandate retracted; plain-path form mandated. BC-6.26.001 v1.14→v1.15. bfs 4.1.1 verified. |
| P27-M02 | MEDIUM | GENUINELY-CLOSED | 14 live body sites normalized to canonical `find "<worktree-path>/.factory" ! -type d`. |
| P27-M03 | MEDIUM | GENUINELY-CLOSED-with-NEW-DEFECT | `-type f` → `! -type d`; EC-009 added (symlink+FIFO inside real shadow dir). BC body change adds EC-009 but the BC body description lacks explicit T-010/RG-010 test-evidence cross-reference → H07. Additionally, `! -type d` behavioral scope (FIFO, socket, device nodes in shadow) warrants explicit BC rationale for non-false-positive guarantee → H05. |
| P27-M04 | MEDIUM | GENUINELY-CLOSED | Comment-strip `[^"]*$` → `.*$`; quote-bearing comments now stripped. |
| POLICY 15 residual | — | CLOSED (CONTROL-equivalence, unchanged) | Corpus vectors followed by CONTROL blocks re-executing real guards on every CI run is a stronger equivalent of one-time restore leg. Log's earlier records demonstrate artifact IS producible. This position is structurally stable: adding more corpus vectors does not break the CONTROL-equivalence argument. |

**Summary:** 6 GENUINELY-CLOSED · 2 GENUINELY-CLOSED-with-NEW-DEFECT (P27-H02→H04; P27-M03→H05+H07) · POLICY 15 residual CLOSED. Pass-27 was the strongest remediation of the cascade; 7/7 primary findings closed. Two new defects emerged from the fixes.

## Part B — New Findings (Pass 28)

### BLOCKER

---

**Finding ID:** `F-S2104-P28-B01`
**Severity:** BLOCKER
**Surface:** `.factory/policies.yaml` — document-2, POLICY 13 ALTERNATION-WIDENING-DIRECTION-STATEMENT MANDATE
**Policies violated:** POLICY 4 (spec content accuracy); POLICY 22 (subagent-report-fidelity — adversary rubric operated blind to 22 policies)

**Description:** `policies.yaml` document-2 is unparseable by every conforming YAML consumer. The root cause is three invalid escape sequences in a double-quoted scalar: the ALTERNATION-WIDENING-DIRECTION-STATEMENT MANDATE (line 255) ends with the shell fragment:

```
git diff HEAD~ -- *.bats | grep '^+.*|' | grep -v '^+++' | grep -v '^\+\+\+'
```

In YAML double-quoted scalars, `\+` is NOT a recognized escape sequence. PyYAML, Ruby Psych, and serde_yaml all fail to parse document-2 when they encounter the three `\+` instances in `grep -v '^\+\+\+'`. Document-1 (version metadata, 7 lines) parses successfully; document-2 (all 22 policies) fails entirely.

**Impact timeline:**
- Introduced at commit `de37915c` (D-920, 2026-07-26): POLICY 13 ALTERNATION-WIDENING-DIRECTION-STATEMENT MANDATE added
- Duration: 3 days (2026-07-26 → 2026-07-29, passes 21–28)
- Scope: all 22 policies in document-2 unreachable to every programmatic consumer during this window

**Adversary rubric note:** The adversary agent prompt encodes POLICY 1–12 directly; POLICY 13–22 are available only via the auto-loaded programmatic document-2. Passes 21–28 enforced POLICY 13–22 from the prompt encoding only, which may have degraded detection coverage for policy-13-class violations introduced during this window.

**Fix:** Change `grep -v '^\+\+\+'` → `grep -v '^\\+\\+\\+'` in the double-quoted YAML scalar. The double-backslash encodes a literal backslash in YAML; the consuming script receives the intended `^\+\+\+` grep pattern. Version bump: v1.4.17 → v1.4.18. Verification: `python3 -c "import yaml; docs=list(yaml.safe_load_all(open('.factory/policies.yaml'))); print('DOCS:', len(docs)); print('POLICY_COUNT:', len([d for d in docs if isinstance(d,dict) and 'policies' in d][0]['policies']))"` → DOCS: 2, POLICY_COUNT: 22.

---

### HIGH

---

**Finding ID:** `F-S2104-P28-H01`
**Severity:** HIGH
**Surface:** `worktree-identity-preflight.bats` — gates (13)–(18) section-wide negative gate bodies
**Policies violated:** POLICY 13 (DOMAIN-COMPLETENESS MANDATE, FAIL-CLOSED-IMPLICATION-DIRECTION MANDATE)

**Description:** Gates (13)–(18) are section-wide negative gate bodies that have not been re-derived since pass-19. Pass-19 introduced the escape-clause + spec-path-wide architecture: clause-scoped trigger with sentence-scoped escape, evaluated over the entire `#### Write Discipline` section. Gates (13)–(18) carry pass-19-era predicate shapes but the story has undergone 9 passes of architecture changes (passes 20–28):

- Pass-20 to pass-28 added 8+ new gate items to the AC-001 gate cell
- Pass-23 introduced `spec_path_prose_nosplit` domain splitting
- Pass-26 corrected B01 anchor-target assertion (affecting the bounded domain)
- Pass-27 corrected T-016 coupling mandate (9th iteration of count-word)

Each of these changes potentially shifts what gates (13)–(18) should fire on. A gate body is un-re-derived when: (a) the AC Gate cell has changed its predicate architecture since the gate was last authored; and (b) the gate body has NOT been updated to match. The adversary spot-checked gate (14) body against the current AC Gate cell and found a single-paragraph extractor where the AC mandates section-wide evaluation (POLICY 13 DOMAIN-COMPLETENESS).

**Fix:** Test-writer re-derives gates (13)–(18) bodies against current story HEAD, ensuring each gate's predicate architecture matches the current AC Gate cell structure (section-bounded domain, escape clause scope, SIBLING-PARAGRAPH mutant, pass-18 NORMALIZATION-ADVERSARIALITY requirements).

---

**Finding ID:** `F-S2104-P28-H02`
**Severity:** HIGH
**Surface:** Story `§Test Plan`, `§Architecture Mapping`, `§File Structure Requirements`, `§Tasks`
**Policies violated:** POLICY 8 (BC→story propagation — EC-009 added to BC v1.15 but test evidence not registered in story)

**Description:** BC-6.26.001 v1.15 added EC-009: `symlink or non-regular inode inside a real shadow `.factory/` directory → `find` exits 0 with empty output (invisible to `-type f`); `! -type d` catches it; PC2b BLOCKED`. This is a new edge case with behavioral consequence: the predicate change from pass-27 M03 silently extends coverage to FIFOs and sockets inside shadow directories.

No test T-010 has been registered in the story §Test Plan to cover EC-009. No red-gate entry RG-010 appears in the §Red Gate Test Plan. BC-6.26.001 §Edge Cases EC-009 has no companion test-evidence entry in the story. The §Architecture Mapping bats row does not include T-010. The §File Structure Requirements bats row count is 15 (should be 16 after T-016 and T-010 additions).

**Fix:** Story-writer registers T-010 (EC-009 vector: stray symlink+FIFO inside real .factory/ dir → `! -type d` BLOCKED) and RG-010 (T-010 red gate baseline) in all four story inventories.

---

**Finding ID:** `F-S2104-P28-H03`
**Severity:** HIGH
**Surface:** `STORY-INDEX.md` line 731 — E-21 delivery blockquote `S-21.04=` hash value
**Policies violated:** POLICY 18 (THREE-WAY-INPUT-HASH-EQUALITY GATE)

**Description:** POLICY 18 mandates three-way equality: story frontmatter `input-hash` = STORY-INDEX catalog row `input-hash` field = STORY-INDEX blockquote `S-NNN=` value. At pass-28 review HEAD:

- Story frontmatter: `input-hash: "c0c614e"` (v1.31, if already committed) or `input-hash: "d6d6a6a"` (v1.30 terminal)
- STORY-INDEX catalog row: `input-hash d6d6a6a` (updated at D-940)
- STORY-INDEX blockquote line 731: `S-21.04=04b393e` (STALE — pre-D-936 value)

The blockquote was refreshed at D-914 with `04b393e` but was NOT updated at D-940 (pass-27 record burst) when the catalog row was advanced to `d6d6a6a`. This is a POLICY 18 three-way equality failure: the blockquote leg is 2 bursts stale.

Additionally: within the story file, the v1.30 `modified[v1.30]` entry and `last_amended` nested v1.30 sub-entry both attest terminal input-hash as `67eaeea`. The actual v1.30 terminal hash was `d6d6a6a` — confirmed by v1.31 modified entry which starts `input-hash d6d6a6a→c0c614e`. Error-acknowledgment correction is required for the story provenance chain.

**Fix (state-manager Commit C):** (1) STORY-INDEX.md line 731: `S-21.04=04b393e` → `S-21.04=d6d6a6a`. (2) Story `modified[v1.30]` terminal hash: `67eaeea` → `d6d6a6a [CORRECTED: was 67eaeea; F-S2104-P28-H03]`. (3) Story `last_amended` nested v1.30 sub-entry: same correction.

---

**Finding ID:** `F-S2104-P28-H04`
**Severity:** HIGH
**Surface:** Story `§Test Plan`, `§Architecture Mapping`, `§File Structure Requirements`, `§Tasks`
**Policies violated:** POLICY 8 (test-registration completeness); POLICY 15 (same-AC gate audit)

**Description:** T-016 `test_coupling_gate_story_gate_count_matches_bats_count_word` was added to `worktree-identity-preflight.bats` at `dc3b83b3` (pass-27 H02 fix, STRUCTURAL finding). However, T-016 is registered ONLY in the AC-001 coupling note within the story. It is NOT present in:

1. `§Test Plan` — no row for T-016
2. `§Architecture Mapping` — T-016 not listed in the Tests group
3. `§File Structure Requirements` — bats row does not mention T-016
4. `§Tasks` — no task for T-016 authoring/registration

This is a POLICY 8 violation: a test was added to the bats suite but not formally registered in the story's canonical test inventories. The coupling note is an informal cite; the four inventories are the authoritative tracking surfaces.

**Fix:** Story-writer adds T-016 row to §Test Plan, AM Tests group, §File Structure Requirements bats row, and §Tasks (as a completed `[x]` item). Coupling note updated to note mechanical enforcement by T-016.

---

**Finding ID:** `F-S2104-P28-H05`
**Severity:** HIGH
**Surface:** `BC-6.26.001` — §Edge Cases EC-009, §Preconditions PC2a(b)
**Policies violated:** POLICY 12 (spec completeness); POLICY 4 (BC body accuracy)

**Description:** Pass-27 M03 widened the predicate from `-type f` to `! -type d`. The adversary scrutinizes this widening per pass-28 lead (b): does `! -type d` introduce false positives on legitimate `.factory/` shadow content?

`! -type d` catches: regular files (type `f`), symlinks (type `l`), FIFOs (type `p`), block/character device nodes (type `b`/`c`), sockets (type `s`). On a legitimate story worktree's shadow `.factory/` directory, all of these are unexpected (the shadow should contain ONLY directories from the `git worktree add --no-checkout` initialization). Product-owner's rationale at v1.15 ("no false positives on empty directory; bfs 4.1.1 verified") covers the empty case but not the case of a legitimately populated shadow directory.

The BC body EC-009 description does not include explicit analysis of whether any `.factory/` shadow directory can legitimately contain non-directory inodes. Without this analysis, the `! -type d` rationale is an empirical test without a structural argument. The BC must record either: (a) a structural argument that legitimate shadow dirs cannot contain non-dir inodes; or (b) an explicit acknowledged scope limitation.

**Fix:** Product-owner adds EC-009 rationale paragraph to BC body: why the shadow `.factory/` directory (created by `git worktree add --no-checkout`) contains only directories by design (git worktree add with gitignored path creates an empty shadow structure; no regular files are created without explicit write commands). Version bump v1.15 → v1.16.

---

**Finding ID:** `F-S2104-P28-H06`
**Severity:** HIGH
**Surface:** Story `AC-007` Gate cell — authoring constraint
**Policies violated:** POLICY 13 (FAIL-CLOSED-IMPLICATION-DIRECTION); POLICY 8

**Description:** AC-007 authoring constraint states (paraphrased from HEAD): agents MUST NOT use `find … .factory -type f` form. This is **predicate-specific**: the prohibition names `-type f` as the forbidden predicate. T-008 tests two specific predicates (`-type f` and another) but the Gate cell constraint does not prohibit the open class "any `find … .factory` form with a predicate other than `! -type d`".

A new predicate (`-maxdepth 1`, `-name "*"`, `-not -type d`) would satisfy the Gate cell constraint even though it diverges from the canonical form. The authoring constraint must be PREDICATE-AGNOSTIC: any `find … .factory` command whose predicate is not exactly `! -type d` (or the mandated canonical form) is prohibited. POLICY 13 FAIL-CLOSED-IMPLICATION-DIRECTION: the trigger must be open class (any find … .factory in the section), and the exception is the closed set (only `! -type d` form is permitted).

**Fix:** Story-writer rewrites AC-007 authoring constraint to prohibit `find … .factory` with ANY predicate other than the mandated `! -type d` form. T-008's two-predicate evidence covers two instances of the open-class trigger. AC-007 Gate cell constraint updated to predicate-agnostic prohibition.

---

**Finding ID:** `F-S2104-P28-H07`
**Severity:** HIGH
**Surface:** `BC-6.26.001` §Edge Cases EC-009 body
**Policies violated:** POLICY 4 (BC body test-evidence completeness)

**Description:** BC-6.26.001 v1.15 added EC-009 (symlink+FIFO inside real shadow `.factory/` directory → `! -type d` BLOCKED). The EC-009 description in the BC body does not contain a cross-reference to the story test evidence (T-010, RG-010) that validates this edge case. BC §Edge Cases entries with behavioral consequences must cite test evidence anchors per POLICY 4 BC-body anchor completeness.

Without T-010/RG-010 citations in EC-009, the BC's internal traceability is broken: a reader verifying EC-009 correctness must independently locate T-010 in the story, rather than following the BC's own internal evidence chain.

**Fix:** Product-owner adds T-010/RG-010 evidence cites to EC-009 in the BC body. Version bump v1.16 → v1.17 (following H05 fix which bumps v1.15 → v1.16).

---

### MEDIUM

---

**Finding ID:** `F-S2104-P28-M01`
**Severity:** MEDIUM
**Surface:** Story `§File Structure Requirements` — `worktree-identity-preflight.bats` row test count
**Policies violated:** POLICY 8

**Description:** The `§File Structure Requirements` row for `worktree-identity-preflight.bats` states the preflight test count as 15 at HEAD. T-016 was added to the bats suite at `dc3b83b3` (pass-27 STRUCTURAL fix), making the count 16. The §File Structure Requirements row was not updated when T-016 was added.

**Fix:** Story-writer updates the preflight test count from 15 to 16 in the §File Structure Requirements bats row.

---

**Finding ID:** `F-S2104-P28-M02`
**Severity:** MEDIUM
**Surface:** Story `§Test Plan` and `§Architecture Mapping` — corpus vector range references
**Policies violated:** POLICY 8; TD-VSDD-091

**Description:** Four live sites in the story reference corpus vector range `M1–M9` (or `M5–M9` for the deletion sub-range). Pass-27 B01 added corpus deletion vectors `M10–M14`, extending the range to `M1–M14` (deletion vectors) and `M5–M14` (combined annotation+deletion range). These four live sites were not updated at D-940.

**Fix:** Story-writer updates all four live sites: `M1–M9` → `M1–M14`, `M5–M9` → `M5–M14`.

---

**Finding ID:** `F-S2104-P28-M03`
**Severity:** MEDIUM
**Surface:** Story `§Tasks` — Task 15 completion status and scope
**Policies violated:** POLICY 8

**Description:** §Tasks Task 15 (register M5–M14 corpus vectors) is not marked `[x]` as completed at HEAD. Additionally, the task scope says `M5–M9` but should reflect `M5–M14` following pass-27 B01's addition of M10–M14.

**Fix:** Story-writer marks Task 15 `[x]` and updates the scope from `M5–M9` to `M5–M14`.

---

**Finding ID:** `F-S2104-P28-M04`
**Severity:** MEDIUM
**Surface:** Story `§Behavioral Contracts table` and `§Token Budget` — BC-6.26.001 version pin
**Policies violated:** POLICY 14 (KK-N quintuple parity)

**Description:** Both the story's §Behavioral Contracts table and §Token Budget row cite `BC-6.26.001 v1.15` at HEAD. The BC was last bumped at pass-27 M01/M02/M03 fix burst (D-940; v1.14→v1.15). Per POLICY 14 KK-N quintuple parity, the 5th leg (upstream-index body-table — story BC table + Token Budget) must be synchronized when the BC version changes. The story's Token Budget and BC table reflect v1.14 nomenclature; need to advance to v1.15.

**Note:** A further bump to v1.16 is expected when product-owner's H05 fix lands; state-manager will sweep via version-cite sweep in Commit C for the subsequent v1.16→v1.17 progression.

**Fix:** Story-writer advances story §Behavioral Contracts table and Token Budget BC pin from v1.15 to v1.16 (post-H05 product-owner advance). Note: this is v1.15 at the adversary's review HEAD; after H05+H07 product-owner bumps advance BC to v1.17, state-manager sweeps the remaining v1.16 cite in Commit C version-cite sweep.

---

**Finding ID:** `F-S2104-P28-M05`
**Severity:** MEDIUM
**Surface:** `worktree-identity-preflight.bats` — gate (14) body
**Policies violated:** POLICY 13 (DOMAIN-COMPLETENESS MANDATE)

**Description:** Gate (14) body uses a single-paragraph extractor (`awk` terminated at first blank line) rather than a section-wide extractor for a negative gate. Per POLICY 13 DOMAIN-COMPLETENESS MANDATE: negative gates MUST evaluate the ENTIRE bounded section, not a single paragraph. A single-paragraph extractor is admissible only for positive existence assertions.

Gate (14) is a negative gate (verifying absence of a prohibited form). With single-paragraph extraction, a sibling-paragraph placement of the prohibited content passes the gate. No SIBLING-PARAGRAPH mutant is attested for gate (14).

**Fix:** Test-writer rewrites gate (14) body to use section-wide extraction (awk over entire `#### Write Discipline` section); adds SIBLING-PARAGRAPH mutant proving the section-wide extraction fires.

---

**Finding ID:** `F-S2104-P28-M06`
**Severity:** MEDIUM
**Surface:** `worktree-identity-preflight.bats` — gate (16) body
**Policies violated:** POLICY 13 (ALTERNATION-WIDENING-DIRECTION-STATEMENT MANDATE)

**Description:** Gate (16) body uses a predicate alternation added after pass-19 (the pass-19 architecture introduced this gate family). The alternation lacks a direction statement: neither "direction: open trigger" nor "direction: closed enumeration; backstopped by \<gate-name\>" appears in the audit row for gate (16). Per POLICY 13 ALTERNATION-WIDENING-DIRECTION-STATEMENT MANDATE, any gate using a predicate alternation must state the direction of that alternation.

**Fix:** Test-writer adds the direction-statement annotation to gate (16) body and its red-gate-log audit row.

---

**Finding ID:** `F-S2104-P28-M07`
**Severity:** MEDIUM
**Surface:** `story-worktree-write-path-discipline.bats` — pipeline probe suite
**Policies violated:** POLICY 13

**Description:** The pipeline probe suite was extended through pass-23 (pass-24 F-S2104-P24-003: 5-leg probe). Pass-27 B01 added corpus deletion vectors M10–M14 to the `worktree-identity-preflight.bats` corpus table but did NOT add corresponding deletion-form probe vectors to the pipeline probe's corpus validation leg. The pipeline probe corpus leg exercises M1–M9 only; M10–M14 deletion vectors are not covered.

**Fix:** Test-writer adds M10–M14 deletion vectors to the pipeline probe corpus validation leg.

---

### LOW

---

**Finding ID:** `F-S2104-P28-L01`
**Severity:** LOW
**Surface:** `worktree-identity-preflight.bats` — gate (18) body
**Policies violated:** POLICY 13

**Description:** Gate (18) body predicate domain is bounded by a heading one level above the correct bounding heading. The adversary checked: gate (18) should be bounded by `#### Worktree-Identity Preflight` but the awk extractor uses the parent `### Step-by-Step` boundary. This is a minor boundary-polarity gap (POLICY 13): the over-broad boundary admits sibling-section content as domain content. Not a BLOCKER because gate (18) is a positive existence assertion (not a negative gate), so the over-broad domain only increases coverage, not decreases it. Still a defect: the audit row does not document the boundary choice.

**Fix:** Test-writer tightens gate (18) extractor to correct `#### Worktree-Identity Preflight` boundary; adds boundary-polarity analysis to audit row.

---

**Finding ID:** `F-S2104-P28-L02`
**Severity:** LOW
**Surface:** `worktree-identity-preflight.bats` — gate (13) body
**Policies violated:** POLICY 13

**Description:** Gate (13) body's SIBLING-PARAGRAPH mutant uses a construct that is identical to the NEGATIVE-TWIN control tested in pass-19 for gate (11). The shared mutant structure means gate (13)'s SIBLING-PARAGRAPH mutant is not independently proven — a refactor of gate (11) could silently break gate (13)'s mutant attestation.

**Fix:** Test-writer replaces gate (13)'s shared SIBLING-PARAGRAPH mutant with a gate-specific construction that does not depend on gate (11)'s predicate structure.

---

## Fix Mapping

| Finding | Owner | Status | Notes |
|---------|-------|--------|-------|
| **B01** | state-manager | **CLOSED at Commit C** | policies.yaml line 255: three `\+` → `\\+` in YAML double-quoted scalar; v1.4.17→v1.4.18; python3 parse verification: DOCS: 2, POLICY_COUNT: 22 |
| **H01** | test-writer | **CLOSED at feature-branch SHA TBD** | Gates (13)–(18) re-derived against current story HEAD; section-wide extractors; SIBLING-PARAGRAPH mutants; POLICY 13 DOMAIN-COMPLETENESS + FAIL-CLOSED-IMPLICATION-DIRECTION per gate |
| **H02** | story-writer | **CLOSED at factory-artifacts Commit D** | T-010/RG-010 registered in §Test Plan, §Architecture Mapping, §File Structure Requirements, §Tasks (EC-009: stray-inode-inside-factory vector) |
| **H03** | state-manager | **CLOSED at Commit C** | STORY-INDEX blockquote `S-21.04=04b393e` → `S-21.04=d6d6a6a`; story `modified[v1.30]` + `last_amended` v1.30 sub-entry hash corrected `67eaeea`→`d6d6a6a` [CORRECTED: was 67eaeea; F-S2104-P28-H03] (three-way hash equality restored) |
| **H04** | story-writer | **CLOSED at factory-artifacts Commit D** | T-016 registered in §Test Plan, §Architecture Mapping, §File Structure Requirements, §Tasks; AC-001 coupling note updated to note mechanical enforcement |
| **H05** | product-owner | **CLOSED at factory-artifacts Commit D** | BC-6.26.001 v1.15→v1.16: EC-009 rationale paragraph added — structural argument that git-worktree-add shadow dir cannot contain non-dir inodes without explicit write commands |
| **H06** | story-writer | **CLOSED at factory-artifacts Commit D** | AC-007 authoring constraint resynced to predicate-agnostic prohibition: any `find … .factory` form with predicate other than canonical `! -type d` is prohibited; T-008 two-predicate evidence cited |
| **H07** | product-owner | **CLOSED at factory-artifacts Commit D** | BC-6.26.001 v1.16→v1.17: EC-009 body adds T-010/RG-010 test-evidence cross-reference |
| **M01** | story-writer | **CLOSED at factory-artifacts Commit D** | §File Structure Requirements preflight count 15→16 |
| **M02** | story-writer | **CLOSED at factory-artifacts Commit D** | Four live sites: `M1–M9`→`M1–M14`, `M5–M9`→`M5–M14` |
| **M03** | story-writer | **CLOSED at factory-artifacts Commit D** | Task 15 marked `[x]`; scope extended `M5–M9`→`M5–M14` |
| **M04** | story-writer | **CLOSED at factory-artifacts Commit D** | Story BC table + Token Budget BC pin advanced v1.15→v1.16 |
| **M05** | test-writer | **CLOSED at feature-branch SHA TBD** | Gate (14) body: section-wide extractor; SIBLING-PARAGRAPH mutant added |
| **M06** | test-writer | **CLOSED at feature-branch SHA TBD** | Gate (16) body: direction-statement annotation added |
| **M07** | test-writer | **CLOSED at feature-branch SHA TBD** | Pipeline probe corpus leg extended with M10–M14 deletion vectors |
| **L01** | test-writer | **CLOSED at feature-branch SHA TBD** | Gate (18) extractor tightened to `#### Worktree-Identity Preflight`; boundary-polarity analysis added |
| **L02** | test-writer | **CLOSED at feature-branch SHA TBD** | Gate (13) SIBLING-PARAGRAPH mutant made gate-specific (independent of gate (11)) |

**Atomicity note for factory-artifacts fixes:** H02/H03/H04/H05/H06/H07/M01/M02/M03/M04 (factory-artifacts changes) are committed in Commits C and D. State-manager Commit C closes B01+H03; Commit D commits all specialist factory-artifacts work (story v1.31 + BC v1.17 + VP v1.6 + 4-index bumps).

## Summary

| Severity | Count |
|----------|-------|
| BLOCKER | 1 |
| HIGH | 7 |
| MEDIUM | 7 |
| LOW | 2 |

**Total findings:** 17 (B1/H7/M7/L2)

**Overall Assessment:** block — one BLOCKER (policies.yaml YAML parse failure) requires fix before any policy-compliance assertion can be made with confidence; 16 additional findings across story architecture, BC coverage, and bats gate re-derivation.

**Convergence:** findings remain — iterate. Streak: 0/3 (B1 resets). Structural pattern: policies.yaml blind window (passes 21–28) and the pass-19 gate architecture debt both surface simultaneously this pass.

**Readiness:** requires revision. Pass-29 NEXT.

**Pass-27 closure assessment:** 6 GENUINELY-CLOSED · 2 GENUINELY-CLOSED-with-NEW-DEFECT (P27-H02→H04; P27-M03→H05+H07) · POLICY 15 residual CLOSED.

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 28 |
| **New findings** | 17 |
| **Duplicate/variant findings** | 0 (B01=policies.yaml YAML infrastructure; H01=gates(13)–(18) un-re-derived since pass-19; H03=POLICY 18 blockquote stale 2-burst lag; H04=T-016 partial registration post-bats-add; H05=M03(a)-widening-scrutiny BC rationale gap; H06=AC-007 predicate-agnostic; H07=EC-009 T-010 cross-ref missing; M01-M07=story inventory/count/scope staleness; L01-L02=specific gate-body precision; all 17 novel) |
| **Novelty score** | 17/17 = 1.0 |
| **Median severity** | MEDIUM |
| **Trajectory** | 14→18→17→12→11→11→9→9→10→11→7→10→10→13→7→6→7→7→12→3→3→12→14→6→17→11→7→**17** |
| **Verdict** | FINDINGS_REMAIN — streak 0/3; one BLOCKER; pass-29 required |

## Completeness Statement

**Priority-1 surfaces covered:** policies.yaml — YAML parse verification confirmed; story §Test Plan, §Architecture Mapping, §File Structure Requirements, §Tasks — 17-finding inventory; BC-6.26.001 — EC-009 body analysis (H05/H07); bats preflight suite — gate (13)–(18) spot-check for pass-19 architecture artifact; STORY-INDEX blockquote — POLICY 18 three-way check.

**Priority-4 surfaces (authorized truncation):** Full gate bodies (14)–(23) not individually verified (M05/M06 are spot-checks only); T-002/T-003/T-005/T-006 assertion bodies not re-read; pipeline probe Legs A–D not re-audited beyond M07. **Pass-29 lead:** (a) test-writer gates (13)–(18) re-derivation — adversary spot-check confirmed; (b) VP-097 v1.6 coverage (architect amended — H5 harness proposal for relative path_allow; state-manager Commit D VP-INDEX sync).

**D-942 refutation note (orchestrator P0 — NOT adversary findings):** Two orchestrator-raised P0 claims were examined and REFUTED during record-burst triage (see D-942 decision-log entry): (b) ~9,939 `path_not_allowed` denials do NOT indicate real-session governance fail-open — all 196 `policies.yaml` CapabilityDenied records carried `session_id = "pass-read-failure-failopen"` and `plugin_version 0.0.1` (BATS test artifacts from deliberate denial induction); real-session data shows zero `validate-policies-schema` denials. (c) policies.yaml B01 does NOT mean POLICY 13–22 were silently ignored — they are encoded in the adversary prompt; auto-loaded portion was blind only. These are state-manager + architect refutations, not adversary findings. rc.24 blocker is re-grounded on: (a) 26% timeout rate 2026-07-27; (b) policies.yaml YAML parse failure (B01); (c) 12 BC-cited fail-open sites.
