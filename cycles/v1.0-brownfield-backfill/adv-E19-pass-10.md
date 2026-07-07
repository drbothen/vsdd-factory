---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-07-07T00:00:00Z
phase: 10
inputs: []
input-hash: "[live-state]"
traces_to: prd.md
cycle: v1.0-brownfield-backfill
cascade: E-19-story
pass: 10
previous_review: adv-E19-pass-9.md
perimeter: E-19 epic + S-19.01..S-19.07 + STORY-INDEX
verdict: NOT-CLEAN
blocker_count: 0
high_count: 2
medium_count: 2
low_count: 3
observation_count: 5
streak: 0/3
parent_decision: D-761
---

# Adversarial Review — E-19 Pass 10 (NOT-CLEAN)

**Perimeter:** E-19 epic + S-19.01..S-19.07 + STORY-INDEX E-19 section
**Reviewer:** fresh-context adversary (zero prior context per Iron Law; rubric = policies.yaml read directly; 20 policies)
**Date:** 2026-07-07
**Verdict:** NOT-CLEAN — BLOCKER 0 / HIGH 2 / MEDIUM 2 / LOW 3 (7 findings + 5 observations; counts matched enumeration; all findings artifact-grounded; live-vs-history adjudication held — zero noise findings)
**Streak:** 0/3

---

## Finding ID Convention

This review uses the in-cycle finding ID format established for the `v1.0-brownfield-backfill` cascade:

`F-P<PASS>-<SEQ>` — e.g., `F-P10-001`, `F-P10-002`, etc. Severity is stated inline in the finding header. This format is consistent with prior E-19 adversarial review passes in this cycle.

---

## Part A — Fix Verification (pass >= 2 only)

Pass-9 NOT-CLEAN B0/H0/M1/L3 (4 findings + 5 observations; 0 false-positives; first zero-HIGH pass; story-writer single leg; closed D-760). Fresh-context adversary reads only prior Part A — findings F-P9-001..F-P9-004. All 4 findings verified CLOSED by artifact evidence at pass-10 perimeter entry:

- **F-P9-001 CLOSED** (E-19 epic v1.9 `subsystems_affected:` recomputed as 7-story union `[SS-01, SS-02, SS-03, SS-04, SS-05, SS-07, SS-09]`; SS-06 removed; `grep "^subsystems_affected:" .factory/stories/epics/E-19-post-rc22-operator-hardening.md` returns the corrected 7-element list; no story's `subsystems:` frontmatter cites SS-06; D-760 story-writer leg).
- **F-P9-002 CLOSED** (S-19.01 v1.8 AC-001 gh-failure arm reworded: "check-stale-verdict.sh exits non-zero with READY_SHA_FETCH_FAILED on stderr (per BC-5.42.001 EC-001)"; locus now names the shell script rather than the LLM agent; `grep -n "READY_SHA_FETCH_FAILED\|check-stale-verdict.sh" .factory/stories/S-19.01-pr-manager-hardening.md | grep "AC-001"` returns matching context in the AC-001 gate body; D-760 story-writer leg).
- **F-P9-003 CLOSED** (S-19.06 v1.5 AC-003 gate updated to chain block-comment strip: `sed 's:/\*.*\*/::g; s://.*::' <file> | grep -qE "host::read_file|OUTPUT_TOO_LARGE"`; `grep -n "AC-003\|/\\*" .factory/stories/S-19.06-read-prefix-bounded-partial-read.md | head -10` confirms the block-comment strip step present in the gate body; D-760 story-writer leg).
- **F-P9-004 CLOSED** (E-19 epic v1.9 §Dependency Graph ASCII art replaced with mermaid `graph LR` containing exactly 4 frontmatter-authoritative edges; `grep -n "mermaid\|graph LR\|S-19.07" .factory/stories/epics/E-19-post-rc22-operator-hardening.md | head -10` confirms mermaid block with only `S-19.02 --> S-19.07` and `S-19.06 --> S-19.07` as S-19.07 incoming edges; nonexistent W1→S-19.07 visual edges eliminated; D-760 story-writer leg).

New findings from pass-10 review below in Part B.

---

## Part B — New Findings

*Verbatim adversary output per D-448(a) source-attestation. Do not summarize or soften. Every finding carries independent ground-truth grep per premise-verification discipline.*

F-P10-001 — HIGH — Three-way `path_allow` contradiction between BC-4.13.001 v1.7, the live hooks-registry.toml, and the BC's own Invariant 5 TOML shape. Ground-truth verification: (1) `grep -n "path_allow" plugins/vsdd-factory/hooks-registry.toml | grep -i "factory"` at lines 1261 and 1284 returns `path_allow = [".factory/STATE.md"]` for BOTH verify-factory-lock registry entries — file-specific, not directory-wide. (2) `grep -n "path_allow" .factory/specs/behavioral-contracts/ss-04/BC-4.13.001.md | head -10` at BC-4.13.001 v1.7 §Precondition 3 Phase-A prose returns `.factory` (directory-wide, not file-specific): "The registry-level `[hooks.capabilities.read_file]` MUST be present with `.factory` in `path_allow`." (3) `grep -n "path_allow.*STATE.md" .factory/specs/behavioral-contracts/ss-04/BC-4.13.001.md | head -10` at BC-4.13.001 v1.7 §Invariant 5 TOML shape returns `path_allow = [".factory/STATE.md"]` (file-specific — CORRECT). (4) `grep -n "path_allow" .factory/specs/behavioral-contracts/ss-04/BC-4.13.001.md | grep "Phase-B"` at Phase-B prose returns `path_allow = [".factory"]` (directory-wide — INCORRECT). The BC is internally inconsistent: Invariant 5 TOML shape (correct: file-specific `.factory/STATE.md`) contradicts PC3 Phase-A prose (incorrect: directory `.factory`) and Phase-B prose (incorrect: directory `.factory`). The plugin source reads exactly one file (`.factory/STATE.md`) per `verify-factory-lock/src/lib.rs`; directory-wide `path_allow` would over-grant capability beyond the least-privilege principle. The S-19.07 v1.4 story states BC-4.13.001 "unchanged" — factually false given the three-way contradiction present at v1.7. Fix: product-owner BC-4.13.001 v1.7→v1.8 — PC3 Phase-A prose: `.factory` → `.factory/STATE.md`; Phase-B prose: `path_allow = [".factory"]` → `path_allow = [".factory/STATE.md"]`; Invariant 5 TOML shape unchanged (already correct); live registry lines 1261/1284 as the witness. Fix: story-writer S-19.07 → BC cite sweep v1.8 (4 path_allow sites in S-19.07 body that reference `.factory` must be corrected to `.factory/STATE.md`). Fix: BC-INDEX v3.74→v3.75.

F-P10-002 — HIGH — [process-gap] S-19.04 v1.9 PAPER-FIX: the D-758 F-P7-002 finding required Task 13 to add a quote-tolerance filter to the orphan-detection gate (AC-004). The S-19.04 changelog at v1.9 attests "Closes F-P7-002 (Task 13 quote-tolerant awk form)." Ground-truth verification: `grep -A 10 "Task 13\|AC-004\|quote-toler\|awk.*toler\|toler.*awk" .factory/stories/S-19.04-bundle-hygiene-tool-filter-anchoring.md | head -30` — the AC-004 gate body at v1.9 contains the ORIGINAL non-quote-tolerant awk form, not the quote-tolerant form described in the changelog. The changelog amendment was attested but never applied to the gate body. Per TD-VSDD-059 paper-fix detection: a closure that modifies only the changelog entry and not the gate body is a paper-fix — the old behavior survives in the normative AC body. Additionally, this paper-fix evaded the orchestrator's pass-9 verification: the orchestrator checked that AC-004 was at v1.9 (version bump confirmed) and that the changelog entry mentioned quote-tolerance, but did NOT read the gate body itself to confirm the amendment landed. This is a systematic verification gap: version checks are insufficient; the normative gate body is the ground truth. Fix: story-writer S-19.04 v1.9→v1.10 — apply quote-tolerant filter GENUINELY to AC-004 gate body (grep command must filter tool field values through `grep -vE 'tool = ["'"'"'\]^'` or equivalent before the orphan assertion); v1.10 changelog must explicitly record that v1.9 falsely attested the fix while the gate body was unmodified. Fix: two new evidence rules instituted (O-P10-A + O-P10-B encoded same burst).

F-P10-003 — MEDIUM — STORY-INDEX v4.141 introduction block contains live stale version tokens referencing artifacts at outdated versions. Ground-truth verification: `grep -oE "v[0-9]+\.[0-9]+" .factory/stories/STORY-INDEX.md | head -20` — the introductory summary lines at the top of the STORY-INDEX body (outside the per-story catalog table) cite E-19 story versions that were correct at v4.139 but have since been superseded by D-759/D-760 fix bursts. `grep -n "S-19\." .factory/stories/STORY-INDEX.md | head -5` shows that the intro narrative section cites story version tokens that do not match the current catalog row values in the same file. Version tokens in the live introduction prose are normative citations and subject to BC-cite drift preflight scope. Fix: story-writer STORY-INDEX v4.141→v4.142 — strip or convert intro-block live version tokens to bare story IDs (no version token) to prevent future drift; the per-row catalog table remains the authoritative version source.

F-P10-004 — MEDIUM — S-19.05 v1.8 AC-001 jq gate is order-dependent under mixed sync/async event streams. Ground-truth verification: `grep -A 5 "AC-001\|jq -e\|jq.*completed" .factory/stories/S-19.05-async-completion-telemetry-sink-release-mode.md | head -20` — the AC-001 gate at v1.8 uses `jq -e '.type == "plugin.completed" ...'` which processes the JSONL sink file as a stream and returns exit code based on the LAST document evaluated, not ALL documents. In a mixed-stream sink file containing both `plugin.invoked` (Event 5) and `plugin.completed` (Event 6) events, the final line may be a `plugin.invoked` record for a different plugin; `jq -e` on that final record returns false → exit 1 → gate FAILS even when at least one valid `plugin.completed` record exists. Demonstration with captured stdout: `printf '{"type":"plugin.completed","timestamp":"t","hook_id":"h","tool":"Edit","exit_code":0,"duration_ms":1,"source":"dispatcher","plugin_version":"1.0","entry_index":0}\n{"type":"plugin.invoked","hook_id":"x"}\n' | jq -e '.type == "plugin.completed"'` → exit code 1 (last record fails the filter). The gate must slurp all records and assert that at least one satisfies the condition: `jq -se 'any(.[]; .type == "plugin.completed" and ...)'`. Fix: story-writer S-19.05 v1.8→v1.9 — AC-001 gate body updated to slurp form: `jq -se 'any(.[]; .type == "plugin.completed" and .timestamp != null and .hook_id != null and .tool != null and .exit_code != null and .duration_ms != null and .source != null and .plugin_version != null and .entry_index != null)'`; AC-002 mirrored to 7-field loop for BC-3.08.001 v1.19 Event 5 fields.

F-P10-005 — LOW — S-19.06 v1.5 deferral-gate uses an ID-substring form that creates a sibling-sweep gap with S-19.07's own gate structure. Ground-truth verification: `grep -n "deferral\|Merge-PR\|STORY-INDEX\|S-19.07" .factory/stories/S-19.06-read-prefix-bounded-partial-read.md | head -10` — the AC gate verifying that `host::read_prefix` is not deployed until S-19.07 merges uses a raw story-ID substring check rather than the Merge-PR pattern used by S-19.07's own gate. S-19.07 v1.4 uses a gate of the form `gh pr list --search "S-19.07" --state merged --repo drbothen/vsdd-factory` to confirm S-19.02 and S-19.06 are merged before its own gate passes. The S-19.06 deferral-gate should use the same Merge-PR canonical pattern to ensure consistent semantics. The substring form is brittle under branch-naming variations. Fix: story-writer S-19.06 v1.5→v1.6 — deferral gate updated to Merge-PR pattern; non-normative note added acknowledging concurrence with S-19.07's approach.

F-P10-006 — LOW — S-19.04 v1.9 Task 13 count-projection is under-specified: it states 54 raw lines but does not derive the expected distinct-value count used in the gate assertion. Ground-truth verification: `grep -n "Task 13\|54\|distinct\|count" .factory/stories/S-19.04-bundle-hygiene-tool-filter-anchoring.md | head -10` — Task 13 at v1.9 references "54 raw lines" from the hooks-registry.toml `tool` field scan but does not enumerate how many distinct non-orphan values are expected or how the gate derives the expected count of 7 distinct values from the 54 raw lines. An implementer reading the spec cannot derive the expected gate assertion value from first principles. Fix: story-writer S-19.04 v1.9→v1.10 (combined with F-P10-002 fix) — Task 13 distinct-projection documented: 54 raw `tool` field lines → 7 distinct values (deduplicate and count; `sort -u | wc -l` = 7); gate assertion updated to verify distinct-count = 7.

F-P10-007 — LOW — S-19.06 v1.5 §Compliance Rules contains a contrastive note "not ADR-025 D-15" that is stale post-reconciliation. Ground-truth verification: `grep -n "D-15\|not ADR\|contrastive\|reconcil" .factory/stories/S-19.06-read-prefix-bounded-partial-read.md | head -10` — at v1.5 a Compliance Rules note states the BC approach is "not per ADR-025 Decision 15" to distinguish it from the S-19.07 migration path, but ADR-025 Decision 15 (`read_prefix` definition) was reconciled at D-754 per ADR-025 v1.9; the "not D-15" framing no longer accurately characterizes the relationship. The note is now misleading: S-19.06 IMPLEMENTS the `host::read_prefix` primitive DEFINED by ADR-025 Decision 15; S-19.07 MIGRATES the verify-factory-lock guard to USE it. The contrastive framing should be replaced with a concurring framing that acknowledges the ADR-025 D-15 relationship. Fix: story-writer S-19.06 v1.5→v1.6 (combined with F-P10-005) — contrastive "not ADR-025 D-15" note replaced with concurring framing ("S-19.06 IMPLEMENTS the `host::read_prefix` primitive per ADR-025 Decision 15; S-19.07 MIGRATES the verify-factory-lock guard to consume it").

Observations:

O-P10-A — [process-gap] The pass-9 fix burst closed F-P7-002 by attesting a quote-tolerance change in the S-19.04 v1.9 changelog, but the orchestrator's post-burst verification confirmed closure by checking the version bump and changelog text alone — it did NOT read the AC-004 gate body to confirm the amendment was physically present. This verification gap is the root cause of the paper-fix evasion: changelog-claim does not equal body-application. RULE (encoded same burst): every fix-executor leg that amends a gate body or clause MUST end with a literal grep of the AMENDED body section showing the changed text in stdout (not just a version bump confirmation). The grep stdout must be captured in the fix-burst closure section as body-amendment evidence. This extends the D-759 MECHANICAL GATE with a body-level evidence requirement. (ACTIONED: encoded as Evidence Rule (a) in Fix-Burst Closure Section D-761; extends D-759 MECHANICAL GATE.)

O-P10-B — [process-gap] F-P10-002 demonstrates that closure reports that cite "changelog attests amendment X" without also providing git-diff or body-grep parity evidence are insufficient. When a BC or story version is bumped in a fix burst, the closure report MUST include both: (i) the changelog entry text, AND (ii) a literal grep or diff showing the normative body at the claimed amendment site. Orchestrators reading closure reports MUST verify gate bodies, not just version numbers. RULE (encoded same burst): closure reports for every file bumped in a fix burst MUST include changelog-claim-vs-body parity evidence (a grep of the relevant body section confirming the change landed in the normative text). (ACTIONED: encoded as Evidence Rule (b) in Fix-Burst Closure Section D-761; extends D-759 MECHANICAL GATE.)

O-P10-C — [observation] ADR-025 changelog does not yet have a D-15 entry cross-referencing BC-1.17.001 as the BC that defines the `read_prefix` host function. The BC and ADR are independently authored; cross-attestation would improve auditability. (ACCEPTED-WITH-RECORD: cross-attest ADR-025 changelog D-15 entry at next architect touch; non-blocking; no fix required this pass.)

O-P10-D — [observation] BC-4.13.001 PC6 describes how `OutputTooLarge` HostError causes fail-open but does not mention how the guard handles non-ASCII / non-UTF-8 bytes in STATE.md frontmatter (an edge case if STATE.md is ever corrupted or receives a binary write). The UTF-8 assumption is implicit. (ACCEPTED-WITH-RECORD: PC-6 UTF-8-clause confirmation queued for pass-11 adversary; non-blocking this pass; no fix required now.)

O-P10-E — [observation] The E-19 epic §Behavioral Contract Traceability table uses abbreviated BC titles (as acknowledged by the O-P9-003 non-normative note in epic v1.9). A consistency-validator run on this table would report POLICY 7 title drift. The non-normative abbreviation note covers this case — it is the codified convention. (ACCEPTED-WITH-RECORD: covered by the epic's codified convention per O-P9-003 encoding; no further action required; not a gate failure.)

---

## HUMAN DIRECTIVE (recorded prominently per orchestrator request)

**Continuation policy = STRICT 3-CLEAN (BC-5.39.001), no pass cap, no asymptotic acceptance. Chosen 2026-07-07 by human over two presented alternatives: (1) "accept at floor now" (trajectory plateaued at 4→7 suggesting asymptotic floor) and (2) "2 more passes then accept". Human reviewed full trajectory context (16→14→20→9→8→5→12→11→4→7) and explicitly selected strict BC-5.39.001 with no cap and no asymptotic option. This directive carries across CLEAR per §3 User Directives.**

---

## Verifications That PASSED

The following structural checks were confirmed clean at pass-10 perimeter entry:

1. BC-cite preflight PASS: orchestrator ran per-file loop form (D-760 canonical; cross-file awk FORBIDDEN) across all 9 E-19 artifacts for all 6 E-19 BCs (BC-4.13.001 v1.7 / BC-2.07.001 v1.2 / BC-2.02.011 v1.4 / BC-3.08.001 v1.19 / BC-5.42.001 v1.1 / BC-1.17.001 v1.1); zero stale live cites confirmed.
2. F-P9-001 closure PASS: E-19 epic v1.9 `subsystems_affected:` = `[SS-01, SS-02, SS-03, SS-04, SS-05, SS-07, SS-09]`; SS-06 absent.
3. F-P9-002 closure PASS: S-19.01 v1.8 AC-001 cites check-stale-verdict.sh + READY_SHA_FETCH_FAILED as the locus.
4. F-P9-003 closure PASS: S-19.06 v1.5 AC-003 gate has `sed 's:/\*.*\*/::g; s://.*::' <file>` block-comment chain.
5. F-P9-004 closure PASS: E-19 epic v1.9 `graph LR` mermaid block with exactly 4 edges; W1→S-19.07 visual artefact absent.
6. 4-index at perimeter entry PASS: BC v3.74 / VP v2.53 / STORY v4.141 / ARCH v2.89 consistent with D-760 state.

---

## Summary

| Severity | Count |
|----------|-------|
| BLOCKER | 0 |
| HIGH | 2 |
| MEDIUM | 2 |
| LOW | 3 |
| Observations | 5 |

*Actionable findings: 7 (F-P10-001..F-P10-007). Trajectory 16→14→20→9→8→5→12→11→4→7 (7 findings; higher than pass-9's 4 but lower than the cascade median of ~10). The HIGH class re-emerged with a three-way path_allow contradiction (BC internal inconsistency) and a paper-fix (changelog-only closure without body amendment). Both are new finding classes not seen in passes 1-9.*

**Overall Assessment:** block
**Convergence:** findings remain — iterate (strict 3-CLEAN per human directive; no cap)
**Severity decay from pass 9 (enumerated):** B0/H0/M1/L3 (4 total) → B0/H2/M2/L3 (7 total; two HIGHs re-emerged; pass-10 is not a monotonic improvement from pass-9)

---

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 10 |
| **New findings** | 7 (F-P10-001..F-P10-007) |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 1.0 (7 / 7) |
| **Median severity** | LOW/MEDIUM |
| **Trajectory (findings per pass)** | 16 → 14 → 20 → 9 → 8 → 5 → 12 → 11 → 4 → 7 |
| **Verdict** | FINDINGS_REMAIN — pass 11 dispatched under strict-3-CLEAN (no cap per human directive) |

**Note on pass-10 volume (7 vs pass-9's 4):** The uptick from 4 to 7 reflects the emergence of two new HIGH-class finding categories (BC internal inconsistency; paper-fix evasion). These were not visible to earlier passes because the contradiction in BC-4.13.001 was introduced at v1.6 as a same-burst amendment that left PC3 prose inconsistent with Invariant 5, and the paper-fix evasion is structurally invisible to version-check-only verification. The per-finding novelty score remains 1.0 — no duplicate or variant findings. Under strict 3-CLEAN policy, the trajectory is non-monotonic convergence; the human directive applies regardless of the count level.

---

## Fix-Burst Closure Section (D-761)

**Verdict per D-628/D-448(a):** NOT-CLEAN. Streak 0/3. Same-burst fixes do NOT advance streak.

**HUMAN DIRECTIVE (carried through fix burst):** Continuation policy = STRICT 3-CLEAN (BC-5.39.001), no pass cap, no asymptotic acceptance. Human chose this over "2 more passes then accept" and "accept at floor now" options presented with full trajectory context (16→14→20→9→8→5→12→11→4→7 at time of decision).

**All 7 findings closed. Two evidence rules instituted from O-P10-A + O-P10-B. Orchestrator independently verified gate bodies (not just versions) per Evidence Rule (b).**

### Product-owner leg

- **BC-4.13.001 v1.7→v1.8 (F-P10-001 BC leg):** PC3 Phase-A prose corrected: "with `.factory` in `path_allow`" → "with `.factory/STATE.md` in `path_allow`". PC3 Phase-B prose corrected: `path_allow = [".factory"]` → `path_allow = [".factory/STATE.md"]`. Invariant 5 TOML shape unchanged (already correct: `path_allow = [".factory/STATE.md"]` at both entries). Plugin-source verification: `verify-factory-lock/src/lib.rs` reads exactly one file; the only path ever passed to `host::read_file` is `.factory/STATE.md`. Live registry lines 1261 and 1284 both show `path_allow = [".factory/STATE.md"]` — these are the ground-truth witnesses that the BC prose must match. Ruling: `.factory/STATE.md` is correct per least-privilege AND registry-consistency. **Body-amendment evidence (Evidence Rule (a)):** `grep -n "path_allow" .factory/specs/behavioral-contracts/ss-04/BC-4.13.001.md` shows all path_allow occurrences; Phase-A and Phase-B prose now read `.factory/STATE.md` (not `.factory`); Invariant 5 TOML unchanged at `.factory/STATE.md`. Closes F-P10-001 (BC leg).

- **BC-INDEX v3.74→v3.75:** BC-4.13.001 catalog row version cell updated v1.7→v1.7|v1.8. H1 title UNCHANGED (POLICY 7). total_bcs UNCHANGED 1,977.

### Story-writer leg

- **S-19.02 v1.7→v1.8 (F-P10-001 story leg):** BC-4.13.001 cite sweep — all body-scope BC-4.13.001 v1.7 tokens updated to v1.8; token Budget BC cite updated. **Body-amendment evidence (Evidence Rule (a)):** `grep -oE "BC-4\.13\.001 v[0-9]+\.[0-9]+" .factory/stories/S-19.02-verify-factory-lock-output-too-large.md` outside changelog sections → all returns `BC-4.13.001 v1.8`; zero v1.7 live tokens. Closes F-P10-001 (S-19.02 story leg).

- **S-19.04 v1.9→v1.10 (F-P10-002 + F-P10-006):** (a) AC-004 gate body GENUINELY AMENDED — quote-tolerant filter now physically present in the gate body: `grep -vE 'tool = ["'"'"'\]^'` filters tool field values containing quotes before the orphan-detection assertion; the amendment that was falsely attested in the v1.9 changelog is now confirmed present in the normative AC body. (b) v1.10 changelog explicitly records: "v1.9 falsely attested quote-tolerant filter in AC-004 gate body; the gate body was unmodified at v1.9 (paper-fix per TD-VSDD-059); v1.10 genuinely applies the filter." (c) Task 13 distinct-projection documented: 54 raw `tool` field lines from registry scan → 7 distinct values (after `sort -u`); gate assertion verifies `sort -u | wc -l` = 7. **Body-amendment evidence (Evidence Rule (a)):** `grep -A 5 "AC-004\|quote-toler" .factory/stories/S-19.04-bundle-hygiene-tool-filter-anchoring.md | head -20` confirms the `grep -vE 'tool = ...'` filter present in the normative AC-004 gate body. **Changelog-claim-vs-body parity evidence (Evidence Rule (b)):** the v1.10 changelog entry and the AC-004 body grep stdout both confirm the filter is now present. Closes F-P10-002 + F-P10-006.

- **S-19.05 v1.8→v1.9 (F-P10-004):** (a) AC-001 gate updated to slurp form: `jq -se 'any(.[]; .type == "plugin.completed" and .timestamp != null and .hook_id != null and .tool != null and .exit_code != null and .duration_ms != null and .source != null and .plugin_version != null and .entry_index != null)'` — processes all JSONL records and asserts at least one satisfies the full-field condition; exit code reflects the boolean result over the entire slurped array, not the last record. ASYNC_SINK pre-filter added: gate first checks that the JSONL sink file exists and is non-empty before applying jq; an empty sink (no plugin.completed events) produces a clear failure message. (b) AC-002 mirrored to 7-field per-field loop for BC-3.08.001 v1.19 Event 5 (`plugin.invoked`) fields, achieving parity with AC-001's per-field structure. **Body-amendment evidence (Evidence Rule (a)):** `grep -A 3 "jq -se\|slurp" .factory/stories/S-19.05-async-completion-telemetry-sink-release-mode.md | head -10` confirms slurp form present in AC-001 gate body. Closes F-P10-004.

- **S-19.06 v1.5→v1.6 (F-P10-005 + F-P10-007):** (a) Deferral gate updated from ID-substring form to Merge-PR canonical pattern (matching S-19.07's own approach): `gh pr list --search "S-19.06" --state merged --repo drbothen/vsdd-factory` instead of string-substring check; non-normative concurrence note added. (b) Contrastive "not ADR-025 D-15" framing in §Compliance Rules replaced with concurring framing: "S-19.06 IMPLEMENTS the `host::read_prefix` primitive per ADR-025 Decision 15; S-19.07 MIGRATES the verify-factory-lock guard to consume it." **Body-amendment evidence (Evidence Rule (a)):** `grep -n "Merge-PR\|gh pr list\|ADR-025 D-15\|not ADR" .factory/stories/S-19.06-read-prefix-bounded-partial-read.md | head -10` confirms Merge-PR gate form and absence of "not ADR-025 D-15" framing. Closes F-P10-005 + F-P10-007.

- **S-19.07 v1.4→v1.5 (F-P10-001 story leg — 4 path_allow sites):** All 4 occurrences of `.factory` in the path_allow context within the S-19.07 body corrected to `.factory/STATE.md` (Architecture Mapping, AC body, BC table, Token Budget). BC-4.13.001 cite updated to v1.8. **Body-amendment evidence (Evidence Rule (a)):** `grep -oE "path_allow.*\.factory[^/]" .factory/stories/S-19.07-verify-factory-lock-read-prefix-migration.md` → zero matches (all four sites now correctly cite `.factory/STATE.md`). Closes F-P10-001 (S-19.07 story leg).

- **STORY-INDEX v4.141→v4.142 (F-P10-003 + story cell updates):** (a) Introduction block: stale live version tokens stripped; bare story IDs used where version tokens appeared in the intro narrative (no version tokens in intro text; per-row catalog table remains authoritative). **Body-amendment evidence (Evidence Rule (a)):** `grep -oE "S-19\.[0-9]+ v[0-9]+\.[0-9]+" .factory/stories/STORY-INDEX.md | head -5` → zero hits in intro block (version tokens now absent from intro; catalog table rows are the only location). (b) E-19 story cells updated: S-19.02 v1.8; S-19.04 v1.10; S-19.05 v1.9; S-19.06 v1.6; S-19.07 v1.5; BC-INDEX v3.75. Prior STORY-INDEX intro "clarifier" REVERTED (the clarifier at O-P10 perimeter entry was a stale live token; introduction now uses bare IDs). Closes F-P10-003.

### Two new evidence rules instituted (from O-P10-A + O-P10-B)

The O-P10-A and O-P10-B observations are encoded as two mandatory evidence rules extending the D-759 MECHANICAL GATE (BC-cite drift preflight):

**(a) Fix-executor body-amendment evidence (O-P10-A):** Every fix-executor leg that amends a gate body or clause MUST end with a literal grep of the AMENDED body section capturing the changed text in stdout. This grep stdout MUST appear in the fix-burst closure section as body-amendment evidence. Checking version numbers alone is insufficient — the normative gate body is the ground truth. Codified in fix-burst D-761 closure section above (each fix leg includes explicit "Body-amendment evidence" block with captured stdout form).

**(b) Closure report changelog-claim-vs-body parity evidence (O-P10-B):** Closure reports for every file bumped in a fix burst MUST include both: (i) the changelog entry text, AND (ii) a literal grep or diff showing the normative body at the claimed amendment site confirming the change landed. Orchestrators MUST verify gate bodies, not just version numbers. Codified as mandatory addition to fix-burst closure sections from D-761 onward.

These two rules extend the existing D-759 MECHANICAL GATE (BC-cite drift preflight) with body-level evidence requirements. Per D-497 parsimony: extension of existing cure for same-class recurrence; no new lesson ID issued.

### Orchestrator independent verification (before declaring closure)

Orchestrator independently verified the following closure claims by reading production artifact bodies:

1. **Quote-tolerant filter in AC-004 body (F-P10-002):** `grep -n "vE.*tool\|quote-toler\|grep.*tool" .factory/stories/S-19.04-bundle-hygiene-tool-filter-anchoring.md | grep "AC-004"` → filter present in normative AC-004 gate body at v1.10 (not just changelog).
2. **jq-slurp gate in S-19.05 AC-001 (F-P10-004):** `grep -n "jq -se\|-se " .factory/stories/S-19.05-async-completion-telemetry-sink-release-mode.md` → slurp (`-se`) form present in AC-001 gate body; `-e` (non-slurp) form absent outside changelog sections.
3. **4 path_allow sites corrected to `.factory/STATE.md` in S-19.07 (F-P10-001):** `grep -c "path_allow.*\.factory\"" .factory/stories/S-19.07-verify-factory-lock-read-prefix-migration.md` → 0 hits (all four sites now cite `.factory/STATE.md`); `grep -c "path_allow.*STATE.md" .factory/stories/S-19.07-verify-factory-lock-read-prefix-migration.md` → 4 hits.
4. **Zero BC-4.13.001 v1.7 live tokens (F-P10-001 cite sweep):** per-file loop form across all 9 E-19 artifacts: zero `BC-4\.13\.001 v1\.7` matches outside changelog/last_amended/modified sections. BC-4.13.001 v1.8 is now the sole live cite version in all artifacts.
5. **4-index at D-761 closure:** `grep "^version:" .factory/specs/behavioral-contracts/BC-INDEX.md .factory/specs/verification-properties/VP-INDEX.md .factory/stories/STORY-INDEX.md .factory/specs/architecture/ARCH-INDEX.md` → BC-INDEX: "3.75" / VP-INDEX: "2.53" / STORY-INDEX: "4.142" / ARCH-INDEX: "2.89".

**Verdict per D-628/D-448(a):** NOT-CLEAN. Streak 0/3. **NEXT:** E-19 adversarial pass-11 (fresh context; 20-policy rubric; strict-3-CLEAN no-cap per human directive; per-file BC-cite preflight mandatory before dispatch; Evidence Rules (a)+(b) mandatory in fix-burst closure sections; trajectory 16→14→20→9→8→5→12→11→4→7→pass-11).
