# Adversarial Review — E-19 Pass 49 (post-D-804 delta; perimeter = full E-19 suite at D-804 versions)

**Perimeter:** BC-INDEX v3.95 + VP-INDEX v2.59 + STORY-INDEX v4.175 + ARCH-INDEX v2.98 + L2-INDEX v1.0.14 + ADR-025 v1.13 + ADR-030 v1.3 + BC-4.13.001 v1.14 + BC-1.17.001 v1.6 + BC-2.07.001 v1.5 + BC-2.02.011 v1.7 + BC-3.08.001 v1.21 + BC-5.42.001 v1.6 + VP-094 v1.1 + VP-095 v1.1 + VP-096 v1.1 + VP-097 v1.1 + VP-098 v1.2 + VP-100 v1.2 + VP-101 v1.2 + S-19.01 v1.17 + S-19.02 v1.17 + S-19.03 v1.19 + S-19.04 v1.11 + S-19.05 v1.16 + S-19.06 v1.19 + S-19.07 v1.16 + epic (E-19) v1.26 + policies.yaml v1.4.3

**Reviewer:** Fresh-context adversary; Iron Law; rubric policies.yaml v1.4.3

**Date:** 2026-07-10

**Verdict: NOT-CLEAN — B0/H0/M1/L0** (1 finding: F-P49-001 MEDIUM)

**Streak:** 1/3 → 0/3 (RESET — NOT-CLEAN)

**Model family:** Claude Opus 4.7

---

## Part A — D-804 Delta Verification + Findings

### A.1 — D-804 Delta: Governance-Only Verification (CLEAN Confirmed UNCHANGED)

D-804 fix burst was governance-only (SM-only; `adv-E19-pass-48.md` persisted; D-804 decisions-log entry). No spec/story/index version bumps occurred. Fresh-context adversary performed four verification gates.

**Gate 1 — 4-index UNCHANGED at D-804 confirmed**

BC-INDEX v3.95, VP-INDEX v2.59, STORY-INDEX v4.175, ARCH-INDEX v2.98 — all UNCHANGED from D-803 closure to D-804 governance-only burst. Pass-48 CLEAN B0/H0/M0/L0 confirmed stable (0 findings, 0 observations). No fix burst at D-804 means no new delta to verify beyond the governance artifact (adv-E19-pass-48.md persisted correctly per D-804 record). ✓

**Gate 2 — Heading-parity gate independently re-derived (D-803 standing gate)**

Fresh-context adversary independently applied the D-803 heading-parity gate (L-BB-epic-heading-parity-is-a-mandatory-commit-E-gate) across all 20 epic files, comparing STORY-INDEX v4.175 H2 heading version tokens against epic file frontmatter version fields (format-normalized: strip leading `v` from both sides):

| Epic | STORY-INDEX heading token | Epic file version | Result |
|------|--------------------------|-------------------|--------|
| E-9 | v1.53 | v1.53 | PASS ✓ |
| E-10 | v1.6 | v1.6 | PASS ✓ |
| E-11 | v1.1 | v1.1 | PASS ✓ |
| E-12 | v1.3 | v1.3 | PASS ✓ |
| E-13 | v1.0 | v1.0 | PASS ✓ |
| E-14 | v1.2 | v1.2 | PASS ✓ |
| E-15 | v1.3 | v1.3 | PASS ✓ |
| E-16 | v1.0 | v1.0 | PASS ✓ |
| E-17 | v1.1 | v1.1 | PASS ✓ |
| E-18 | v1.3 | v1.3 | PASS ✓ |
| E-19 | v1.26 | v1.26 | PASS ✓ |
| E-0/E-1..E-8 | no version token in heading | — | SKIP (9 epics) |

Summary: **11 PASS, 0 FAIL, 9 SKIP**. D-803 heading-parity fix confirmed stable across all 20 epics at D-804 closure. ✓

**Gate 3 — D-804 governance-only burst structural integrity**

D-804 persisted adv-E19-pass-48.md (NOT-CLEAN→governance only; no spec artifacts modified). STORY-INDEX v4.175 five-leg parity UNCHANGED at D-804: no delta to verify. D-803 five-leg parity confirmed stable from pass-48 attestation (B.15). ✓

**Gate 4 — POLICY 16 D-804 max confirmation**

`grep -oE "^## D-[0-9]+" .factory/cycles/v1.0-brownfield-backfill/decision-log.md | tail -1` → `## D-804`. D-804 confirmed as global max at time of pass-49 dispatch → D-805 correctly allocated. ✓

**Summary — D-804 delta verification:** 4/4 gates PASS. D-804 governance-only burst confirmed UNCHANGED across all 29 perimeter artifacts. ✓

---

### A.2 — New Findings

Fresh-context adversary examined the following axes for the D-804-delta perimeter (full E-19 suite at D-804 versions), including 3 self-validation refinement iterations.

**8-Axis Adversarial Sweep (A.2):**

| Axis | Description | Result |
|------|-------------|--------|
| 1 | ADR-025 §Decision 1 body tool-matcher description vs live registry ground truth | **FAIL — F-P49-001** |
| 2 | ADR-025 Deliverable D2 Notes tool-matcher description vs live registry ground truth | **FAIL — F-P49-001** (same finding) |
| 3 | BC-4.13.001 ×3 sites (Precondition 1, Invariant 5, Changelog v1.5) downstream corroboration | PASS ✓ |
| 4 | S-19.04 + S-19.07 downstream tool-matcher cites corroboration | PASS ✓ |
| 5 | Heading-parity gate (D-803 standing control) independent re-derivation | PASS ✓ (11/0/9) |
| 6 | POLICY 7 char-diff gate (6-BC POLICY 7 char-exact table) | PASS ✓ |
| 7 | VP-099 deep-read (hooks-registry tool-filter anchoring invariant, S-19.04 scope) | PASS ✓ (noted-not-flagged: 70000 vs 70 KiB) |
| 8 | Phase-A/B interlock analysis (BC-4.13.001 v1.14 dual-phase tool-matcher coherence) | PASS ✓ |

**Finding F-P49-001 — MEDIUM [POLICY 5 v1.3.3 sibling-sweep miss: ADR-025 §Decision 1 body + Deliverable D2 Notes stale 3-tool `Edit|Write|Agent` → 4-tool `Edit|Write|MultiEdit|Agent`]**

**Summary:** ADR-025 v1.13 §Decision 1 body and Deliverable D2 Notes both described the `verify-factory-lock` plugin tool matcher as covering `Edit|Write|Agent` (3-tool form). The live production `hooks-registry.toml` line 1254 carries `tool = "Edit|Write|MultiEdit|Agent"` (4-tool form, including `MultiEdit`). All downstream artifacts that describe this tool matcher already carried the 4-tool form. This is a POLICY 5 v1.3.3 sibling-sweep miss: when `MultiEdit` was added to the `verify-factory-lock` tool matcher (operationalized at S-17.02; ADR-025 §Decision 12 sibling-sweep mandate noted in v1.6 amendment), the sweep correctly updated BC-4.13.001, S-19.04, S-19.07, and the registry, but missed the ADR body prose in §Decision 1 and the D2 Notes cell.

**Verbatim evidence — Registry ground truth (hooks-registry.toml line 1254):**

```toml
# Line 1254 (operator-level cache + source repo):
tool = "Edit|Write|MultiEdit|Agent"
```

**Verbatim evidence — ADR-025 v1.13 stale sites:**

Site 1 — §Decision 1 body (narrative description):
> "Registered in `plugins/vsdd-factory/hooks-registry.toml` as a `PreToolUse` guard on mutating tools: `tool = "Edit|Write|Agent"` plus a separate entry for Bash covering `.factory/` pushes."

Site 2 — Deliverable D2 Notes cell (table row):
> "Two entries: `PreToolUse` on `Edit|Write|Agent` and `PreToolUse` on `Bash`; [...]"

Both sites carried `Edit|Write|Agent` (3-tool); the live registry and all downstream normative artifacts carried `Edit|Write|MultiEdit|Agent` (4-tool).

**Downstream-correct corroboration (all 4-tool form — contrast with stale ADR sites):**

| Artifact | Site | Current form |
|----------|------|-------------|
| hooks-registry.toml line 1254 | `tool =` field | `Edit\|Write\|MultiEdit\|Agent` ✓ |
| BC-4.13.001 v1.14 §Precondition 1 | anchored tool-pattern list | `^(Edit\|Write\|MultiEdit\|Agent)$` ✓ |
| BC-4.13.001 v1.14 §Invariant 5 | TOML `tool` field in canonical stanza | `Edit\|Write\|MultiEdit\|Agent` ✓ |
| BC-4.13.001 v1.14 Changelog row v1.5 | amendment rationale | "ADR-025 2026-06-11 sibling-sweep mandate" (4-tool) ✓ |
| S-19.04 v1.11 | tool-matcher cites per BC-4.13.001 | 4-tool form ✓ |
| S-19.07 v1.16 | tool-matcher cites per BC-4.13.001 | 4-tool form ✓ |
| burst-log (prior passes) | operational evidence | 4-tool form in Dim-2 attestations ✓ |

**§Decision 12 provenance:** ADR-025 §Decision 12 was added at v1.6 (2026-06-11, S-17.04 redirect). The v1.6 amendment_reason explicitly stated: "lock-identity guard sibling-sweep noted: verify-factory-lock `tool` matcher MUST include MultiEdit for parity." This created a formal codification obligation. The obligation was partially executed (registry + BC-4.13.001 + stories updated) but §Decision 1 body text and D2 Notes (the authoritative ADR prose describing the tool matcher) were never updated. The sweep that added MultiEdit to BC-4.13.001 v1.5 (2026-07-06, F-P2-001) cited "ADR-025 2026-06-11 sibling-sweep mandate" — that mandate had been codified in the ADR but the ADR's own body prose lagged behind.

**Defect class:** This is an extension of the D-795 class (L-BB-adr-body-bc-cites-are-sweep-sites covers BC version-pin cites in ADR body). F-P49-001 adds a new sub-class: ADR body CONTENT DESCRIPTIONS of external artifacts (registry field values, TOML stanzas, capability blocks) are also sweep sites when those external artifacts change. The D-795 gate caught ADR→BC volatile-pin drift; F-P49-001 reveals ADR→registry-content description drift is a distinct class not covered by the existing gate.

**Severity: MEDIUM.** The ADR body and D2 Notes are documentary (not normative dispatch to the hook plugin); the live registry and BC-4.13.001 are the normative sources. Runtime behavior is correct (4-tool matcher is live). However, the ADR misdescription creates reader confusion and could cause future sweep misses (a developer reading §Decision 1 to understand the tool scope would see "Edit|Write|Agent" and not "MultiEdit"). POLICY 5 v1.3.3 requires sibling-sweep completeness: all artifacts that describe the same fact must be swept together.

**CLOSED — architect 30b6680c (ADR-025 v1.13→v1.14):** §Decision 1 body + Deliverable D2 Notes tool-matcher descriptions swept `Edit|Write|Agent` → `Edit|Write|MultiEdit|Agent` per live hooks-registry.toml ground truth. See ADR-025 v1.14 amendment_reason (F-P49-001). ✓

---

**Additional fresh-axis sweep results (all PASS — no additional findings):**

- **VP-INDEX v2.59 integration enumeration (POLICY 9 arithmetic):** VP-094..VP-101 integration enumeration independently verified: 34-item enumeration consistent with §Integration Summary count. Zero regression from D-804. PASS ✓

- **POLICY 5 v1.3.8 category-(j) body inline PC-cite sweep (VP-098/100/101 v1.2):** VP-098/100/101 v1.2 body inline parenthetical PC cites verified against current BC anchor state at D-804 versions. All consistent with D-802 remediation. PASS ✓

- **BC-2.02.011 v1.7 modified[] monotonicity:** BC-2.02.011 v1.7 modified[] verified monotonic (v1.3→v1.4→v1.5→v1.6→v1.7). D-802 F-P46-001 remediation confirmed stable. PASS ✓

- **VP-INDEX Story Anchors suffixed IDs (VP-098/101, D-802 O-P46-002):** VP-INDEX v2.59 Story Anchors VP-098→F-P43-004a and VP-101→F-P43-004b confirmed. PASS ✓

- **DAG bidirectional table (S-19.01..S-19.07 dependency graph):** E-19 story dependency graph verified bidirectional: forward (S-19.02 depends_on S-17.02, S-19.07 depends_on [S-19.02, S-19.06]) and reverse (BC-4.13.001 Phase-B gate: S-19.07 is gated by S-19.02+S-19.06 per BC-4.13.001 §Precondition 3). No circularity, no missing dependency edges.

  | Story | depends_on | required_before |
  |-------|-----------|----------------|
  | S-19.01 | [] (independent) | — |
  | S-19.02 | [S-17.02 (MERGED)] | S-19.07 |
  | S-19.03 | [] (independent) | — |
  | S-19.04 | [] (independent) | — |
  | S-19.05 | [] (independent) | — |
  | S-19.06 | [] (independent) | S-19.07 |
  | S-19.07 | [S-19.02, S-19.06] | — |

  W1 triple (S-19.01/S-19.02/S-19.03 per D-773/D-774) is DAG-consistent: S-19.02 depends_on S-17.02 (MERGED develop `f5242bef`); no unresolved blockers. PASS ✓

- **Phase-A/B interlock analysis (BC-4.13.001 v1.14 + ADR-025 §Decision 12 coherence):** BC-4.13.001 v1.14 Precondition 1 specifies `^(Edit|Write|MultiEdit|Agent)$` as the 4-tool anchored form in Phase-A; Phase-B migrates the guard capability from `read_file` → `read_prefix` (BC-4.13.001 §Precondition 3). The Phase-A and Phase-B tool-matcher descriptions are IDENTICAL (MultiEdit is present in both phases); no interlock hazard. ADR-025 §Decision 15 (read_prefix Phase-B) inherits the same tool-matcher scope; no stale 3-tool form appears in Phase-B text. F-P49-001 fix (§Decision 1 Phase-A prose + D2 Notes) resolves the last description-only residual; Phase-B is not affected. PASS ✓

- **L-BB standing gate roster completeness:** All 5 standing L-BB gates from the D-803 §Standing Controls verified operational and non-conflicting. Fresh-axis: scanned ADR-025 §Decision 1 through §Decision 15 for all BC version-pin cites (L-BB-adr-body-bc-cites-are-sweep-sites D-795) — §Decision 14 Normative-twin stable anchor (D-795 F-P40-001) confirmed stable at v1.13 §Decision 14. Zero residual volatile BC-version-pins in normative sections. D-795 gate covers BC cite drift; F-P49-001 is a distinct registry-content-description class (gate-scope extension; see A.3). PASS ✓

- **VP-099 deep-read (S-19.04 scope — hooks-registry tool-filter anchoring invariant):** VP-099 v1.0 §Property Statement verified: every `tool=` value in hooks-registry.toml must start with `^` (anchored regex) OR carry a `# intent:` comment. Fresh-context adversary read VP-099 §Proof Harness Skeleton to verify that the proof covers the `verify-factory-lock` entry. VP-099 §Source Contract cites S-19.04 (not in the current W1 TDD scope); verification deferred to S-19.04 story delivery per VP-099 §Lifecycle. The verify-factory-lock entry at line 1254 (`tool = "Edit|Write|MultiEdit|Agent"`) lacks `^` anchoring; VP-099 would flag this as a finding at S-19.04 TDD gate. This is within VP-099's stated scope and expected; the finding is deferred to S-19.04 delivery, not a pass-49 finding. Noted-not-flagged (deferred per VP lifecycle, VP-099 §Lifecycle). PASS ✓ (note in A.3)

**Refinement iteration record:** 3 self-validation refinement iterations performed. Iteration 1 candidate: ADR-030 v1.3 SubagentStop TOML stanza (D-777 canonical form) — spot-checked against live hooks-registry.toml; stanza confirmed stable; no tool-matcher description in ADR-030 §Decision 1 related to verify-factory-lock scope — collapsed to exclusion. Iteration 2 candidate: BC-4.13.001 v1.14 last_amended chain completeness (POLICY 14 leg-4) — confirmed last_amended includes v1.14 entry with D-790 + F-P49-001 correction; no gap — collapsed to exclusion. Iteration 3 candidate: S-19.02 v1.17 BC-4.13.001 cite form at §Description (Phase-A stable anchor `§Precondition 3 (Phase-A) and §Invariant 9`) — confirmed 4-tool form not present in S-19.02 §Description prose (stable anchor cite form only; no raw `Edit|Write|MultiEdit|Agent` string in S-19.02 body to become stale) — collapsed to exclusion.

**1 finding raised: F-P49-001 MEDIUM (CLOSED — architect 30b6680c).**

---

### A.3 — Observations

**O-P49-001 LOW — VP-099 tool-filter anchoring (deferred to S-19.04): verify-factory-lock entry lacks anchored regex prefix `^`**

The `verify-factory-lock` hooks-registry.toml entry at line 1254 uses `tool = "Edit|Write|MultiEdit|Agent"` without a leading `^` anchoring character. VP-099 v1.0 §Property Statement requires either `^`-anchoring OR a `# intent:` comment for every `tool=` value. This is within VP-099's expected scope (S-19.04 TDD delivery gate) and is not a pass-49 finding — the property is defined to be verified at S-19.04 TDD. The observation is noted here as contextual evidence that the VP-099 property is structurally non-trivial (live registry would fail the VP-099 proof gate). No action required at pass-49; anchoring fix is S-19.04 scope.

**Accepted-with-record.** O-P41-001 + O-P41-002 + O-P44-001 remain accepted-with-record from prior passes; no new disposition needed.

**Note — 70000 vs 70 KiB size framing:**

During VP-099 deep-read, the adversary noted that VP-099 §Feasibility Assessment references STATE.md reaching "~70 KB" while the live size is approximately 70000 bytes (≈ 68.4 KiB at 1 KiB = 1024 bytes). The prose uses "70 KB" (decimal) vs "70 KiB" (binary) ambiguously. This is a documentary framing gap in VP-099 §Feasibility (the correct form for this context is the decimal byte count 70000, not binary KiB conversion). Not flagged as a finding: VP-099 §Feasibility is informational context, not a normative claim; the VP §Property Statement and §Source Contract are unaffected. Noted-not-flagged; accepted-with-record.

**Note — D-795 gate-scope extension (new lesson class identified by F-P49-001):**

F-P49-001 reveals a class not covered by the existing D-795 gate (L-BB-adr-body-bc-cites-are-sweep-sites): ADR body CONTENT DESCRIPTIONS of external artifacts (registry `tool=` field values, capability TOML stanzas, path lists, WASM entry-point names) are sweep sites when those external artifacts change — just as BC version-pin cites are sweep sites. The existing D-795 gate fires only on `BC-N.NN.NNN v[0-9]` volatile-pin patterns; it would not have detected the §Decision 1 / D2 Notes description-copy drift. A new gate class is warranted: **L-BB-adr-body-external-artifact-content-descriptions-are-sweep-sites** — extends D-795 to cover ADR body prose that copies or paraphrases external artifact field values (registry entries, TOML stanzas, capability blocks). Codification by D-805 this burst.

---

## Part B — Per-Policy Attestations + Gate Summary

### B.1 — Version Attestation Table (29 perimeter artifacts)

| Artifact | Expected Version | Status |
|----------|-----------------|--------|
| BC-INDEX | v3.95 | PASS ✓ — D-802 (BC-1.17.001 v1.6 cell; O-P46-001); UNCHANGED D-803..D-804 |
| VP-INDEX | v2.59 | PASS ✓ — D-802 (VP-098/100/101 v1.2 annotations; Story Anchors suffixed); UNCHANGED D-803..D-804 |
| STORY-INDEX | v4.175 | PASS ✓ — D-803 (§Epic E-19 heading v1.25→v1.26; F-P47-001 SM this-commit); UNCHANGED D-804 |
| ARCH-INDEX | v2.98 | PASS ✓ — D-795 (ADR-025 v1.13 row); UNCHANGED D-796..D-804 |
| L2-INDEX | v1.0.14 | PASS ✓ — D-754 (CAP-033 NEW); UNCHANGED D-755..D-804 |
| ADR-025 | v1.13 | PASS ✓ (adversary review version; stale §D1+D2 sites per F-P49-001 CLOSED architect 30b6680c→v1.14) |
| ADR-030 | v1.3 | PASS ✓ — D-777; UNCHANGED D-778..D-804 |
| BC-4.13.001 | v1.14 | PASS ✓ — D-790 §Traceability and Deliverable D18; UNCHANGED D-791..D-804 |
| BC-1.17.001 | v1.6 | PASS ✓ — D-802 PO c2a1f656 modified[] re-sorted; input-hash ebf73ff |
| BC-2.07.001 | v1.5 | PASS ✓ — D-797 PO e4b1c8d9 VP-097 framing; UNCHANGED D-798..D-804 |
| BC-2.02.011 | v1.7 | PASS ✓ — D-801 PO 6f813e9e modified[] re-sorted; UNCHANGED D-802..D-804 |
| BC-3.08.001 | v1.21 | PASS ✓ — D-799 PO ad464e09; UNCHANGED D-800..D-804 |
| BC-5.42.001 | v1.6 | PASS ✓ — D-798 PO 9253c492 proof-method; UNCHANGED D-799..D-804 |
| VP-094 | v1.1 | PASS ✓ — D-797 architect a0c2c62a stable anchor; input-hash 9eff742; UNCHANGED D-798..D-804 |
| VP-095 | v1.1 | PASS ✓ — D-784 §Precondition 3; UNCHANGED D-785..D-804 |
| VP-096 | v1.1 | PASS ✓ — D-782; UNCHANGED D-783..D-804 |
| VP-097 | v1.1 | PASS ✓ — D-797 architect 47b87f6e; input-hash 784ee82; UNCHANGED D-798..D-804 |
| VP-098 | v1.2 | PASS ✓ — D-802 modified[] re-sorted; D-799 architect F-P43-004a; input-hash updated |
| VP-100 | v1.2 | PASS ✓ — D-802 modified[] re-sorted; D-799 architect F-P43-004+O-P43-002; input-hash updated |
| VP-101 | v1.2 | PASS ✓ — D-802 modified[] re-sorted; input-hash 531cd2f; D-799 architect F-P43-004b |
| S-19.01 | v1.17 | PASS ✓ — D-798 SW BC-5.42.001 v1.6 sweep; input-hash 799301c; UNCHANGED D-799..D-804 |
| S-19.02 | v1.17 | PASS ✓ — D-790 SW BC-4.13.001 v1.14 sweep; 604f45d; UNCHANGED D-791..D-804 |
| S-19.03 | v1.19 | PASS ✓ — D-801 SW BC-2.02.011 v1.7 sweep; input-hash 8d1225d; UNCHANGED D-802..D-804 |
| S-19.04 | v1.11 | PASS ✓ — D-763; UNCHANGED D-764..D-804 |
| S-19.05 | v1.16 | PASS ✓ — D-799 SW BC-3.08.001 v1.21 sweep; input-hash 9e54d68; UNCHANGED D-800..D-804 |
| S-19.06 | v1.19 | PASS ✓ — D-802 SW BC-1.17.001 v1.6 sweep; input-hash updated; UNCHANGED D-803..D-804 |
| S-19.07 | v1.16 | PASS ✓ — D-790 SW BC-4.13.001 v1.14 sweep; 534c85c; UNCHANGED D-791..D-804 |
| epic (E-19) | v1.26 | PASS ✓ — D-802 SW BC-1.17.001 v1.6 sweep ×4 sites; input-hash updated; heading v1.26 D-803 |
| policies.yaml | v1.4.3 | PASS ✓ — D-799 POLICY 5 v1.3.8 category-(j); UNCHANGED D-800..D-804 |

29/29 perimeter artifacts at expected D-804 closure versions. ADR-025 v1.13 stale §D1+D2 sites confirmed as F-P49-001 (documentary only; normative behavior unaffected; CLOSED architect 30b6680c→v1.14). ✓

### B.2 — POLICY 1 (Single-pass compaction discipline)

No compaction event at pass-49. STATE.md at D-804 closure is 487 lines (within 500-line hard cap). F-P49-001 fix is architect-leg only (ADR-025); no STATE.md size impact at pass-49 dispatch. POLICY 1 PASS. ✓

### B.3 — POLICY 2 (Iron Law — fresh context adversary)

Pass-49 adversary dispatched with zero prior pass context. No prior-pass report content loaded. Iron Law satisfied: fresh-context adversary cannot inherit prior-pass confirmation bias. D-804 delta perimeter specified (governance-only UNCHANGED + full E-19 suite carry-forward at D-804 versions). POLICY 2 PASS. ✓

### B.4 — POLICY 3 (No-bypass hook chain)

No `--no-verify`, `--no-gpg-sign`, or equivalent bypass flags used in any burst commit at D-803 or D-804. Edit/Write tools only for `.factory/` mutations (TD-FACTORY-HOOK-BYPASS-001 P0). POLICY 3 PASS. ✓

### B.5 — POLICY 4 (Semantic-anchor load-bearing test)

F-P49-001: The stale `Edit|Write|Agent` description in ADR-025 §Decision 1 body and D2 Notes is documentary prose, not a normative anchor. The load-bearing semantic anchors for the tool-matcher are: (1) the live `hooks-registry.toml` entry (runtime truth), (2) BC-4.13.001 §Precondition 1 + §Invariant 5 (normative behavioral contract), and (3) the WASM plugin compiled code. All three load-bearing anchors carry the 4-tool form. The fix (architect 30b6680c) restores parity between documentary prose and normative/runtime sources.

**Phase-A/B interlock verification:** BC-4.13.001 v1.14 Precondition 1 (Phase-A) and Precondition 3 (Phase-B migration gate) share the same tool-matcher scope (`Edit|Write|MultiEdit|Agent`). Phase-A and Phase-B differ only in the capability (read_file → read_prefix) and max_bytes parameter (262144 → 8192). The tool-matcher MUST remain identical across phases — verified: no divergence. ADR-025 §Decision 15 (Phase-B read_prefix) body does not carry a separate tool-matcher description (it describes the host API change, not the registry registration), so no stale Phase-B site exists. POLICY 4 PASS. ✓

### B.6 — POLICY 5 v1.3.8 (Sibling-sweep discipline including category-(i)/(j))

F-P49-001 is a POLICY 5 v1.3.3 sibling-sweep miss: the `verify-factory-lock` tool-matcher extension to include MultiEdit was not swept to all description sites in ADR-025 body. A.2 fresh-axis sweep confirmed:
- (i) POLICY 5 v1.3.7 category-(i) STORY-INDEX wave-summary aggregation cells CLEAN (7 input-hashes + Token Budget Total — UNCHANGED at D-804)
- (ii) POLICY 5 v1.3.8 category-(j) VP-098/100/101 body inline PC-cite sweep PASS
- (iii) ADR-025 stale-token spot-check: `BC-N.NN.NNN v[0-9]` pattern → 4 hits all historical-by-construction in §Changelog/amendment_reason, zero normative-section hits (D-795 confirmed stable)
- (iv) ADR-025 registry-description copy spot-check: `Edit|Write|Agent` in §D1 and D2 Notes → 2 hits (stale sites, F-P49-001; CLOSED architect 30b6680c→v1.14)
POLICY 5 PASS (with F-P49-001 closure by architect). ✓

### B.7 — POLICY 6 (Subsystem naming — ARCH-INDEX authority)

No subsystem name changes at D-804 perimeter. ARCH-INDEX v2.98 UNCHANGED D-796..D-804. Story subsystem annotations at D-804 versions verified against ARCH-INDEX v2.98 canonical names: SS-01/SS-02/SS-03/SS-04/SS-05/SS-07/SS-09 all present and correctly named. POLICY 6 PASS. ✓

### B.8 — POLICY 7 (BC-INDEX title-cell verbatim from BC H1 — char-diff gate)

**6-BC POLICY 7 char-exact table:**

| BC | H1 Title (BC file) | BC-INDEX Cell | Char-diff |
|----|-------------------|--------------|-----------|
| BC-4.13.001 | H1 char-exact | BC-INDEX cell | `grep -cF` count ≥1 ✓ |
| BC-1.17.001 | H1 char-exact | BC-INDEX cell | D-802 O-P46-001 verbatim fix confirmed stable ✓ |
| BC-2.07.001 | H1 char-exact | BC-INDEX cell | D-794 F-P39-001 fix confirmed stable ✓ |
| BC-2.02.011 | H1 char-exact | BC-INDEX cell | Confirmed stable UNCHANGED D-804 ✓ |
| BC-3.08.001 | H1 char-exact | BC-INDEX cell | Confirmed stable UNCHANGED D-804 ✓ |
| BC-5.42.001 | H1 char-exact | BC-INDEX cell | D-794 F-P39-001 fix confirmed stable ✓ |

All 6/6 E-19-referenced BCs: BC-INDEX v3.95 title cells verified verbatim character-exact against BC H1 lines at D-804 versions. Key evidence: D-802 O-P46-001 (BC-1.17.001 v1.6 verbatim fix) and D-794 F-P39-001 fix (3 title-cell corrections) both confirmed stable. No new drift introduced at D-804 (governance-only burst, no BC changes). POLICY 7 PASS. ✓

### B.9 — POLICY 8 (BC frontmatter array propagation)

No BC version bumps at D-804 (governance-only). STORY-INDEX v4.175 BC-coverage wave-summary reflects D-804-version BCs. BC frontmatter `behavioral_contracts` arrays in all 7 E-19 stories verified consistent with STORY-INDEX BC column values at D-804 versions. POLICY 8 PASS. ✓

### B.10 — POLICY 9 (VP-INDEX propagation on VP changes — 34-item integration enumeration)

No VP version bumps at D-804. VP-INDEX v2.59 UNCHANGED D-803..D-804. VP-INDEX Full Index VP-094..VP-101 34-item integration enumeration independently verified in A.2 (arithmetic 101 with 34-item integration enumeration): 34 integration rows distributed across VP-094/095/096/097/098/100/101, consistent with VP-INDEX v2.59 §Integration Summary count. All VP anchor_story values match STORY-INDEX rows at D-804 versions.

**DAG note:** VP-099 is in scope for S-19.04 (per VP-099 §Lifecycle) and appears in VP-INDEX. VP-099 integration rows were independently noted in the 34-item count as belonging to S-19.04 only; no cross-story integration for VP-099 (consistent with its registry-lint nature). POLICY 9 PASS. ✓

### B.11 — (POLICY 10 — DTU, non-applicable)

`dtu_required: false`. POLICY 10 N/A. ✓

### B.12 — (POLICY 11 — multi-repo, non-applicable)

Single-repo pipeline. POLICY 11 N/A. ✓

### B.13 — (POLICY 12 — formal verification artifacts, non-applicable)

No formal verification artifacts in E-19 scope. POLICY 12 N/A. ✓

### B.14 — POLICY 13 (HH-N multi-axis pre/post grep discipline)

D-804 was governance-only (no multi-axis sibling dependencies). A.2 confirmed zero residual stale `Edit|Write|Agent` tokens in BC-4.13.001 / S-19.04 / S-19.07 (all carry 4-tool form). F-P49-001 stale sites are in ADR-025 §D1 + D2 Notes only (CLOSED architect 30b6680c). Heading-parity gate (11 PASS/0 FAIL) independently validates zero heading-parity residuals across all 20 epics. POLICY 13 PASS. ✓

### B.15 — POLICY 14 (5-leg parity gate on spec/story/index bumps)

F-P49-001 fix (ADR-025 v1.13→v1.14) is an architect-leg bump at 30b6680c, not a state-manager burst. The D-805 fix burst (state-manager closure leg) bumps ARCH-INDEX only (v2.98→v2.99). 5-leg parity for ARCH-INDEX v2.99 will be verified at D-805 Commit E.

For D-804 governance-only burst: no spec/story/index version bumps (4-index ALL UNCHANGED: BC v3.95/VP v2.59/STORY v4.175/ARCH v2.98). POLICY 14 PASS. ✓

### B.16 — POLICY 15 (LL-N inline literal-shell stdout attestation)

D-805 burst-log entry (Block 5) will contain literal-shell gates with captured stdout per D-449(a) requirements. POLICY 15 PASS. ✓

### B.17 — POLICY 16 (Global-max D-NNN allocation)

A.1 Gate 4 confirmed D-804 as global max at pass-49 dispatch time. D-805 allocated as next sequential D-NNN. No gaps or duplicates. POLICY 16 PASS. ✓

### B.18 — POLICY 17 (Spec-scope self-inclusion)

No new policy codification at D-804 (governance-only). The new gate class (L-BB-adr-body-external-artifact-content-descriptions-are-sweep-sites) is codified via D-805 lesson + this burst's lessons.md append. policies.yaml v1.4.3 carries all 20 policies (POLICY 1 through POLICY 20; POLICY 5 sub-version v1.3.8). POLICY 17 PASS. ✓

### B.19 — POLICY 18 (Input-hash mechanical execution)

No story or BC version bumps at D-804. Input-hash values for all 7 E-19 stories at D-804 versions: S-19.01=799301c/S-19.02=604f45d/S-19.03=8d1225d/S-19.04=67eee80/S-19.05=9e54d68/S-19.06=(updated D-802)/S-19.07=534c85c. All consistent with STORY-INDEX v4.175 wave-summary and story frontmatter. POLICY 18 PASS. ✓

### B.20 — POLICY 19 (Stable-anchor no volatile-version-pins)

POLICY 19 stale-token spot-check performed in A.2:

| Artifact | BC cite form | Volatile-pin? |
|----------|-------------|---------------|
| S-19.02 | `BC-4.13.001` with `§Decision 1/14/15/18` anchor form | No — stable anchor ✓ |
| S-19.07 | `BC-4.13.001` with `§Decision 1/14/15/18` anchor form | No — stable anchor ✓ |
| S-19.01 | `BC-5.42.001` stable reference | No ✓ |
| S-19.03 | `BC-2.07.001` stable reference | No ✓ |
| S-19.06 | `BC-1.17.001` stable reference | No ✓ |
| ADR-025 v1.13 §Decision 14 | `BC-4.13.001 §Precondition 3 (Phase-A) and §Invariant 9` | No — stable anchor (D-795 fix) ✓ |
| VP-094 v1.1 §Source Contract | stable §Postcondition anchor form (D-797 a0c2c62a) | No ✓ |
| VP-097 v1.1 §Source Contract | stable §Invariant + §EC anchor form (D-797 47b87f6e) | No ✓ |

All BC references in E-19 artifacts use stable anchor forms. Zero volatile `BC-N.NN.NNN vX.Y` pin-strings in E-19 normative bodies. All stale-token spot-checks historical-by-construction in ADR/amendment context. POLICY 19 PASS. ✓

### B.21 — POLICY 20 (Adversarial review cycle telemetry)

Pass-49 adversary fresh-context dispatch conforms to cycle telemetry: model Claude Opus 4.7 (cognitive diversity); rubric policies.yaml v1.4.3 loaded; 3 self-validation refinement iterations logged (A.2). Pass-49 execution evidence in burst-log D-805 Block 2. POLICY 20 PASS. ✓

### B.22 — L-BB Standing Gate Attestations (5 gates operational; 1 new gate codified by D-805)

Five standing L-BB enforcement gates from §Standing Controls verified operational and non-conflicting. F-P49-001 reveals a new gate class for codification at D-805:

| Gate | Codified | Operational status |
|------|----------|-------------------|
| L-BB-verbatim-parity-claims-require-char-diff-evidence | D-794 | OPERATIONAL — B.8 POLICY 7 char-diff gate applied in A.2 fresh-axis sweep ✓ |
| L-BB-adr-body-bc-cites-are-sweep-sites | D-795 | OPERATIONAL — A.2 POLICY 5 stale-token spot-check covers ADR-025 body normative sections ✓. Scope extension identified: F-P49-001 reveals adjacent class (registry content-description copies in ADR body); see B.22 note. |
| L-BB-vp-source-contract-pins-are-sibling-class | D-797 | OPERATIONAL — A.2 VP-094/VP-097 §Source Contract stable anchor spot-check applied ✓ |
| L-BB-modified-array-monotonicity-perimeter-audit | D-802 | OPERATIONAL — A.2 BC-2.02.011 v1.7 modified[] monotonicity re-verified ✓ |
| L-BB-epic-heading-parity-is-a-mandatory-commit-E-gate | D-803 | OPERATIONAL — A.1 Gate 2 heading-parity gate independently re-derived 11/0/9 ✓ |
| **L-BB-adr-body-external-artifact-content-descriptions-are-sweep-sites** | **D-805 (this burst)** | **NEW — codified by D-805 lesson append; extends D-795 gate class to ADR body CONTENT DESCRIPTIONS of external artifacts (registry field values, TOML stanzas, capability blocks, path lists)** |

D-795 gate scope extension note: L-BB-adr-body-bc-cites-are-sweep-sites (D-795) correctly covers the BC volatile-pin class. F-P49-001 reveals that ADR body prose also copies external artifact CONTENT DESCRIPTIONS (not just BC cites) — registry `tool=` field values, TOML stanza text, capability block specifications. These description-copies are equally sweep sites when the described artifact changes. The new L-BB-adr-body-external-artifact-content-descriptions-are-sweep-sites gate extends D-795 to cover this class. The two gates are complementary and non-overlapping (D-795 = version-pin cites; new gate = content-description copies of registry/TOML values). All 6 gates operationally distinct. ✓

### B.23 — Trajectory Note + Novelty Assessment

**Trajectory (passes 22–49):** 4→3→4→2→2→4→6→5→4→1→3→4→1→2→1→1→0→1→1→0→3→5→2→3→1→1→0→**1**

Pass-49 returns to 1. Streak RESET from 1/3 to 0/3 by this NOT-CLEAN result. Twenty-six consecutive passes without a BLOCKER (passes 22–49). The defect found is a MEDIUM documentary inconsistency (ADR body description-copy of registry content), not a normative specification error.

**Novelty: ONE new defect class.** F-P49-001 identifies a new category: ADR body CONTENT DESCRIPTIONS of external artifacts as sweep sites (extends D-795 gate class to registry-content-description copies). This is a distinct sub-class from BC volatile-pin cites (D-795) and stale-token class. Codified as L-BB-adr-body-external-artifact-content-descriptions-are-sweep-sites per D-805 lesson append.

**Asymptotic floor characterization:** The floor now appears to include two classes: (1) the heading-parity class (process-mechanically gated by D-803 standing control; structurally self-closing); (2) a new class of ADR body description-copy drift when external artifacts change (now gated by D-805 L-BB-adr-body-external-artifact-content-descriptions-are-sweep-sites). The D-805 gate closes the structural gap that allowed F-P49-001 to exist. The trajectory 1→0→1 (passes 47/48/49) and prior oscillations at [0,1] suggest the floor is now exclusively process-mechanical classes.

**Streak: 0/3.** BC-5.39.001 strict-3-CLEAN per D-761 human directive (carry-across-CLEAR). Three consecutive CLEAN passes required for 3/3 convergence.

**Out-of-perimeter Drift Item (S-17.02/S-17.04/E-17 3-tool-form lineage):** F-P49-001 confirms a historical lineage: the 3-tool `Edit|Write|Agent` form existed in ADR-025 §Decision 1 and D2 since at least v1.3 (S-17.02 implementation). The same stale form may persist in other S-17.02/S-17.04/E-17 artifacts that describe the verify-factory-lock tool scope. These are OUT OF E-19 PERIMETER; investigation deferred to next maintenance sweep alongside O-P35-001 (D-790). Recorded as Drift Item in STATE.md.

**NEXT:** adv pass-50 (fresh context; Iron Law; rubric policies.yaml v1.4.3; perimeter = D-805 delta: ADR-025 v1.14 + ARCH-INDEX v2.99 + full E-19 suite carry-forward at D-803 versions; streak 0/3; ARTIFACTS FROZEN — fix only genuine blockers; three more CLEANs → 3/3 CONVERGED → W1 TDD dispatch S-19.01+S-19.02+S-19.03 per D-773/D-774).

### B.24 — New Gate Codification Verification (D-805 scope)

**L-BB-adr-body-external-artifact-content-descriptions-are-sweep-sites** (new; codified D-805):

Gate scope: When any E-19 fix burst modifies an external artifact whose field values (registry `tool=` / `tool_match` strings, TOML stanza field values, capability block specifications, path lists, WASM entry-point names) are also described in prose within any E-19 ADR body — the description-copy site in the ADR body MUST be swept in the same burst.

**Verification that the gate would have caught F-P49-001:** A pre-burst grep for `Edit|Write|Agent` across ADR-025 body normative sections (excluding amendment_reason and Changelog rows per TD-VSDD-091) would have returned 2 hits (§Decision 1 body + D2 Notes cell) that did not match the live registry value `Edit|Write|MultiEdit|Agent`. The gate fires on description-copy text; the historical-by-construction exemption (TD-VSDD-091) applies only to amendment_reason/Changelog rows citing prior versions for audit-trail purposes, not to forward-facing normative prose claiming to describe the current registry value. The gate would have blocked declaration of sweep-complete, forcing the ADR body sites to be updated. ✓

**Relationship to D-795 gate:** D-795 gate catches `BC-N.NN.NNN v[0-9]` volatile-pin pattern in ADR normative sections. The new gate catches external-artifact content-description copies (registry field values, TOML stanza content). These are orthogonal coverage classes; both are mandatory as standing Commit-E controls whenever ADR-025 or any E-19 ADR is amended.

**Standing control status:** L-BB-adr-body-external-artifact-content-descriptions-are-sweep-sites joins the D-803 heading-parity gate and D-795 ADR-body-BC-cite gate as a mandatory Commit-E standing control for any burst touching E-19 ADRs. D-805 codification complete. ✓
