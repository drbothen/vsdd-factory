---
pass: 10
verdict: NOT-CLEAN
reviewed_head: 5370db80e0b7360f5c884f4adf68371fe426ba30
reviewed_heads_additional: "diff base origin/develop @ 700b4dd3; fix/nested-factory-path-derivation @ 9afc3226"
factory_artifacts_head: cbff0801262170bcd296afd811ff3cc0a8ec352a
novelty: 0.78
previous_review: "cycles/v1.0-brownfield-backfill/S-21.07/adversary-pass-9.md"
---

## Summary

Pass-10 fresh-context adversarial review. **10 findings: B2 / H4 / M3 / L1 / NIT0** (D-967 correction: F-S2107-P10-010 added; MEDIUM 2→3; total 9→10). Trajectory `47→18→25→25→24→20→16→8→10`. Streak: **0/3** (BC-5.39.001 reset — NOT-CLEAN).

**Reviewed SHAs (stated explicitly):**

| Tree | SHA |
|---|---|
| `feature/S-21.07-validate-cross-site-correspondence` | **`5370db80e0b7360f5c884f4adf68371fe426ba30`** |
| diff base `origin/develop` | **`700b4dd32251bc9ce40dc59bed8cc7441a9afcb0`** |
| `factory-artifacts` | **`cbff0801262170bcd296afd811ff3cc0a8ec352a`** |

**Tooling disclosure (POLICY 22):** Results relayed to state-manager through team-lead for persistent recording per POLICY 22 relay-chain discipline. State-manager re-executed all mechanical claims with own literal shell and captured stdout; results recorded in burst-log D-966 Dim-2. Where re-execution agreed with relayed claims, this file records the relayed findings faithfully. Where re-execution disagreed, discrepancies are noted explicitly in the burst-log.

---

## Part A — Findings

### BLOCKER

#### F-S2107-P10-001 — BLOCKER — POLICY 15 + TD-VSDD-059. The ratified v1.4.22 ATTESTATION-LOCATION GATE is vacuous. Its pre-check runs in the factory-artifacts worktree, which contains zero `*.rs`/`*.bats` files by construction — pre-check always empty — always INAPPLICABLE — attestation check unreachable for any factory-artifacts commit.

**Location:** `.factory/policies.yaml` POLICY 15 ATTESTATION-LOCATION GATE (v1.4.22; ADR-040 §Decision 6); the `git diff --name-only HEAD^1 HEAD -- '*.rs' '*.bats'` pre-check running in the factory-artifacts worktree context; `.factory/policies.yaml` verification_steps; ADR-040. Compounding defect: `RED_GATE_LOG=$(find "$FACTORY_ROOT" -name red-gate-log.md | head -1)` matches 14 factory-artifacts candidates and resolves to a different cycle; the S-21.07 governing log lives in the code repo at `crates/hook-plugins/validate-cross-site-correspondence/docs/red-gate-log.md`.

**Clause violated:** POLICY 15 per-guard mutant-verification mandate — a gate that can never fire (empty domain for the conditional trigger) is trivially vacuous; TD-VSDD-059 (closure of F-S2107-P9-003 claims POLICY 15 gate now satisfiable; pre-check domain voiding contradicts that claim).

**Evidence — literal shell:**

```
$ git -C .factory ls-tree -r --name-only HEAD | grep -cE '\.(rs|bats)$'
0
```

Zero `*.rs` or `*.bats` files in factory-artifacts worktree. The conditional pre-check `git diff --name-only HEAD^1 HEAD -- '*.rs' '*.bats'` therefore returns EMPTY for every factory-artifacts commit, making the gate INAPPLICABLE on every commit where it could fire.

```
$ git -C .factory ls-tree -r --name-only HEAD | wc -l
    3742
```

3742 total files in factory-artifacts — all markdown, yaml, json, toml, sh. Zero assertion-site files.

```
$ find .factory -name red-gate-log.md | wc -l
      14
$ find .factory -name red-gate-log.md
.factory/cycles/v1.0-feature-plugin-async-semantics-pass-1/red-gate-log.md
.factory/cycles/v1.0-feature-engine-discipline-pass-1/implementation/red-gate-log.md
.factory/cycles/v1.0-feature-engine-discipline-pass-1/S-12.03/red-gate-log.md
.factory/cycles/v1.0-brownfield-backfill/S-21.04/implementation/red-gate-log.md
[... 10 more; none is the S-21.07 file at crates/hook-plugins/.../docs/red-gate-log.md]
```

14 candidates found; none is the S-21.07 governing log (which lives in the code repo, not factory-artifacts). `find "$FACTORY_ROOT" -name red-gate-log.md | head -1` resolves to a different cycle's log.

**Why it matters.** The ATTESTATION-LOCATION GATE was the subject of two passes of adversarial attention (P8-003 → P9-003), an ADR (ADR-040), and human ratification (D-965). All of that work resolved an impossibility (HEAD-SHA circularity) but landed on a domain that is structurally empty: assertion-site commits live on code branches (`feature/S-21.07`), not on factory-artifacts. The gate's trigger condition (`*.rs`/`*.bats` changed in factory-artifacts`) can never be true, so the obligation can never be verified, and the gate cannot produce a non-INAPPLICABLE result for any factory-artifacts commit. This is a gate-domain-mismatch class finding: the gate fires in the wrong repo context, and the red-gate-log lookup resolves to the wrong artifact. The D-965 ratification cemented this into policy.

**Routing.** architect: redesign ADR-040 §Decision 6 so the gate fires in the correct context (code repo, not factory-artifacts) or specify a cross-repo pre-check mechanism; ADR-040 v1.2 required; policies.yaml v1.4.23 amendment follows; human re-ratification required (F-003 applies). State-manager: add Blocking Issues entry (P0-class; live registry).

---

#### F-S2107-P10-002 — BLOCKER — POLICY 15 — `[regression]`. Attestation sections for `67ffbdcc` and `38c70f9e` were added retroactively in docs-only commit `5370db80`, not at the commits themselves. POLICY 15 requires existence "at that commit" with same-commit bundling. The violation at those two commits is permanent — history is immutable.

**Location:** `crates/hook-plugins/validate-cross-site-correspondence/docs/red-gate-log.md` — the three `### Pass-9 assertion-site attestation (SHA)` headings added at `5370db80`; `5370db80` commit `67ffbdcc` (the fix commit); `38c70f9e` (the test-writer commit). Against POLICY 15 ATTESTATION-LOCATION GATE v1.4.20+v1.4.22 "same-commit bundling" requirement.

**Clause violated:** POLICY 15 — "a fix wave that adds or strengthens any bats assertion site MUST NOT be pushed until the matching red-gate-log.md attestation section exists at that commit"; TD-VSDD-053 same-commit bundling.

**Evidence — literal shell — per-commit heading counts:**

```
$ RG="crates/hook-plugins/validate-cross-site-correspondence/docs/red-gate-log.md"
$ for sha in 37022ecc 67ffbdcc 38c70f9e 5370db80; do
    count=$(git show "${sha}:${RG}" 2>/dev/null | grep -cE '^### .*assertion-site attestation' || echo 0)
    echo "${sha}: assertion-site-attestation-headings=${count}"
  done
37022ecc: assertion-site-attestation-headings=0
67ffbdcc: assertion-site-attestation-headings=0
38c70f9e: assertion-site-attestation-headings=0
5370db80: assertion-site-attestation-headings=3
```

Zero attestation headings at `67ffbdcc` (the assertion-site commit that closed P8-006/P8-007/P8-013) and `38c70f9e` (the test-writer commit that added bypass-vector tests for P9-001). Three headings were added later at `5370db80`:

```
$ git show 5370db80:${RG} | grep -nE '^### .*assertion-site attestation'
1618:### Pass-9 assertion-site attestation (37022ecc5398514744b72660da47bfb2964abb55)
1655:### Pass-9 assertion-site attestation (67ffbdccda5302a4e1fbffd8b2f2b8bdd0aed3ce)
1682:### Pass-9 assertion-site attestation (38c70f9e3cebb57e6de6686588b4ecbc66c88195)
```

Retroactive attestation at `5370db80` for three prior commits; POLICY 15 "at that commit" requirement not met at `67ffbdcc` and `38c70f9e`. History is immutable — these violations cannot be remediated by future commits.

Sub-clause: `5370db80` itself uses a `stability-record` form in a third heading while the finding ID (F-S2107-P9-003) belongs to the attestation class. The section heading text uses the `attestation` word where v1.4.22 §Decision 6 specifies that stability-record form MUST NOT use `attestation`. The exact heading text resolves to using "attestation" for all three, which is the active obligation form under v1.4.22 — but this means `5370db80` records attestation obligations that belong at `67ffbdcc` and `38c70f9e` respectively.

**Why it matters.** Retroactive attestation cannot be distinguished from properly-timed attestation unless you compare per-commit presence. The gate predicate `grep -cE '^### .*assertion-site attestation ($PARENT)'` as written in v1.4.22 checks the *current tree*, not the *tree at the assertion-site commit*. An adversary running the gate at HEAD (`5370db80`) sees 3 headings and concludes the gate passes — but two of those headings were not present when the obligation was incurred. POLICY 15's "at that commit" requirement is unreachable via the v1.4.22 gate predicate, reinforcing F-001's domain-mismatch finding.

**Routing.** architect: address in the ADR-040 v1.2 redesign (F-001 route). Record as permanent historical violation per TD-VSDD-059; no retroactive remediation possible; the D-965 ratification record must note this. Do NOT retro-edit history.

---

### HIGH

#### F-S2107-P10-003 — HIGH — POLICY 22 + TD-VSDD-059. ADR-040 §Decision 6's justifying premise is false. It describes `5370db80` as a "test-writer commit… stability entry with no assertion-site files changed" producing a false positive. `5370db80` is `docs(S-21.07): add POLICY 15 attestation sections…` — a docs commit that changed `red-gate-log.md` only. The INAPPLICABLE branch it introduced exempts the very commit class used to backfill the attestations. D-965 ratification of policies.yaml v1.4.22 is procured-on-mischaracterization; re-ratification is required after the ADR-040 redesign.

**Location:** `specs/architecture/decisions/ADR-040-policy-15-attestation-gate-parent-sha-predicate.md` — §Decision 6 context paragraph describing `5370db80`; D-965 ratification decision row in STATE.md and decision-log.md; policies.yaml v1.4.22 ATTESTATION-LOCATION GATE.

**Clause violated:** POLICY 22 `subagent_report_fidelity_literal_shell` — load-bearing premises presented for human ratification MUST carry literal-shell backing; POLICY 4 semantic anchoring integrity; TD-VSDD-059 paper-fix detection.

**Evidence — literal shell:**

```
$ git show --no-patch --format="%H %s" 5370db80
5370db80e0b7360f5c884f4adf68371fe426ba30 docs(S-21.07): add POLICY 15 attestation sections for pass-8, pass-9, and docs commits (F-S2107-P9-003)
$ git diff --name-only 5370db80^1..5370db80
crates/hook-plugins/validate-cross-site-correspondence/docs/red-gate-log.md
```

`5370db80` changed exactly one file: `red-gate-log.md`. This is a docs file (not a `*.rs` or `*.bats` file), so the v1.4.22 pre-check (`git diff --name-only HEAD^1 HEAD -- '*.rs' '*.bats'`) returns EMPTY at `5370db80` — which is the INAPPLICABLE path. But `5370db80` is the commit that *added the attestation sections themselves*, not a stability-record commit. ADR-040 §Decision 6 describes it as a "stability entry with no assertion-site files changed" — technically accurate (no `*.rs`/`*.bats` changed) but materially misleading: this is the attestation-provision commit, and the INAPPLICABLE branch was introduced specifically to exempt it, making the gate unable to verify the obligation at the very moment the attestation was being provided.

**Why it matters.** Human ratification decisions depend on the accuracy of the premises presented. D-965 ratified the gate amendment based on the ADR-040 §Decision 6 framing. If the framing incorrectly characterizes the false-positive class, the ratification is not meaningfully informed. Here, the "false positive" characterization of P9-003 may itself be partially incorrect: whether P9-003 was a false positive depends on whether `5370db80` was the correct commit at which to satisfy the gate. Since the gate fires in the wrong context (F-001), the entire premise deserves re-examination.

**Routing.** architect: correct ADR-040 §Decision 6 §Context narrative with accurate per-commit characterization; route to human re-ratification after ADR-040 v1.2 redesign closes F-001. State-manager: record D-965 ratification status as "PROCURED-ON-MISCHARACTERIZATION — pending architect re-review and human re-ratification" in Drift Items.

---

#### F-S2107-P10-004 — HIGH — TD-VSDD-059 + POLICY 11 analog. BC-5.39.010 (v1.16/v1.17) claims in present perfect that the fuel cap "has been raised to 20,000,000 … satisfiable at HEAD". At `5370db80` both `InvokeLimits::default` and `RegistryDefaults::default` are `fuel_cap: 10_000_000`. Because `fuel_consumed` is clamped to the cap and exhaustion traps at 10M, the `≤12,000,000` margin-gate assertion is unconditionally true whenever a plugin completes — a tautological gate, not merely an early one.

**Location:** `specs/behavioral-contracts/ss-05/BC-5.39.010.md` — §Gate Spec production-scale fuel-margin gate assertion ("fuel_consumed ≤ 12,000,000 (60% of 20M cap)") and its normative claim that "the fuel cap has been raised to 20,000,000" in v1.17; against `crates/factory-dispatcher/src/registry.rs` `RegistryDefaults::default()` and `crates/factory-dispatcher/src/invoke.rs` `InvokeLimits::default()`.

**Clause violated:** POLICY 5 SDK-GROUNDING MANDATE + v1.3.6 HEAD-REPRODUCIBILITY (normative claim about external code that does not hold at HEAD); POLICY 11 no_test_tautologies analog; TD-VSDD-059 (self-disclosure of fix is not authoritative; the BC claims fix is implemented; the code shows it is not).

**Evidence — literal shell at `5370db80` HEAD:**

```
$ grep -n "fuel_cap" crates/factory-dispatcher/src/registry.rs | grep -v "test\|//\s"
crates/factory-dispatcher/src/registry.rs:187:            fuel_cap: 10_000_000,
$ grep -n "fuel_cap" crates/factory-dispatcher/src/invoke.rs | grep -v "test\|//\s"  
crates/factory-dispatcher/src/invoke.rs:271:            fuel_cap: 10_000_000,
```

Both `RegistryDefaults::default()` and `InvokeLimits::default()` have `fuel_cap: 10_000_000`. The cap has NOT been raised to 20M at HEAD. The fix branch `fix/fuel-cap-raise-20m` (`7cbb9232`) exists but is release-gated and not merged. The margin gate `≤12,000,000` therefore fires only if `fuel_consumed` exceeds 12M — which is structurally impossible against a 10M cap. Every plugin run against a 10M cap will satisfy `fuel_consumed ≤ 12,000,000` (and indeed ≤ 10,000,000) by definition. The gate is tautological at HEAD.

**Why it matters.** The margin gate exists to provide advance warning before fuel exhaustion. At 10M cap, the `≤12M` assertion provides zero warning — it is structurally equivalent to `true`. The BC claims the gate is substantive; it is not at the code state it claims to govern.

**Routing.** product-owner: correct BC-5.39.010 present-perfect claim — "has been raised to 20M" MUST become "WILL be raised to 20M at release of `fix/fuel-cap-raise-20m`; this BC describes the post-release state; gate is currently tautological against the 10M cap in `develop`"; margin threshold conditioned on the released cap, not the current cap.

---

#### F-S2107-P10-005 — HIGH — POLICY 4 + POLICY 22. STATE.md and burst-log assert "ADR-042 v1.2 ratified"; ADR-042 frontmatter is `status: proposed` with no `ratified:` field. Similarly ADR-041 is `status: proposed`. ADR-041's ALLOCATOR-CEILING gate is codified into POLICY 16 and executed as a blocking pre-allocation gate in live bursts — a proposed ADR governing a live blocking gate.

**Location:** `.factory/STATE.md` — Current Phase cell "ADR-042 v1.2 ratified"; Active Branches section; decisions log D-964 sub-clause (b) "ADR-042 v1.2 ratified"; `burst-log.md` D-964 Block 4 "ADR-042 v1.2 ratified"; against `specs/architecture/decisions/ADR-042-*.md` frontmatter `status: proposed` (no `ratified:` field); and similarly `specs/architecture/decisions/ADR-041-*.md` frontmatter `status: proposed`.

**Clause violated:** POLICY 4 semantic anchoring integrity (audit cell claims "ratified" for an artifact whose frontmatter says "proposed"); POLICY 22 relay-chain fidelity; POLICY 5 HEAD-reproducibility.

**Evidence — literal shell:**

```
$ grep -E "^(status|ratified):" .factory/specs/architecture/decisions/ADR-041-*.md
.../ADR-041-*.md:status: proposed
$ grep -E "^(status|ratified):" .factory/specs/architecture/decisions/ADR-042-*.md
.../ADR-042-*.md:status: proposed
```

Both ADR-041 and ADR-042 have `status: proposed` with no `ratified:` field. STATE.md and burst-log record them as "ratified" based on narrative description in D-961/D-964, but this narrative was never applied to the actual ADR frontmatter fields. Recording a proposed ADR as ratified in the pipeline SoT defeats the human-ratification gate — future agents reading STATE.md will not know to seek human ratification.

**Why it matters.** ADR-041's ALLOCATOR-CEILING gate (POLICY 16) is already blocking live bursts using a gate whose authoritative ADR remains `proposed`. Any future re-opening of the ADR for amendment requires its status to accurately reflect its lifecycle state. The D-965 ratification of ADR-040 demonstrates this correctly (ADR-040 was updated to `status: active`); the same procedure was not applied to ADR-041 or ADR-042.

**Routing.** architect: update ADR-041 frontmatter (`status: proposed → active`; `ratified: 2026-08-07` per D-961); update ADR-042 frontmatter (`status: proposed → active`; `ratified: 2026-08-08` per D-964). State-manager: correct "ratified" assertions in STATE.md and burst-log after architect updates frontmatter.

---

#### F-S2107-P10-006 — HIGH — POLICY 13 BOUNDARY-POLARITY MANDATE. ADR-042 §Decision 1's excluded-region row 4 dismisses "Other O(n)-in-input plugins (lessons.md, STATE.md validators)" because "their budgets are independent; ADR-039 Phase 3 per-plugin calibration covers them" — but §Decision 2 is titled "Raise is **global** (`InvokeLimits::default()`), **not per-plugin**" and no per-plugin `fuel_cap` field exists. The mandate requires a mutant proving the excluded region's polarity; none is supplied. Empirical corroboration: the D-965 burst itself hit `plugin timed out` on four validators writing burst-log.md and STATE.md.

**Location:** `specs/architecture/decisions/ADR-042-*.md` — §Decision 1 excluded-region row 4; §Decision 2 raise-scope definition; against `hooks-registry.toml` (absence of per-plugin `fuel_cap` fields).

**Clause violated:** POLICY 13 BOUNDARY-POLARITY MANDATE — whenever a gate's domain is narrowed by an exclusion, the burst MUST record the false-positive class (harmful content of opposite polarity occupying the excluded region) and a mutant proving the answer; POLICY 13 FAIL-CLOSED-IMPLICATION-DIRECTION.

**Evidence — literal shell (own measurement of affected file sizes at HEAD):**

```
$ wc -c .factory/cycles/v1.0-brownfield-backfill/burst-log.md \
        .factory/cycles/v1.0-brownfield-backfill/decision-log.md \
        .factory/cycles/v1.0-brownfield-backfill/lessons.md \
        .factory/specs/behavioral-contracts/BC-INDEX.md \
        .factory/stories/STORY-INDEX.md \
        .factory/specs/architecture/ARCH-INDEX.md
 1784060 burst-log.md
 1862908 decision-log.md
 1266513 lessons.md
  577860 BC-INDEX.md
  508006 STORY-INDEX.md
  327690 ARCH-INDEX.md
```

**[D-966 precision note CORRECTED by D-967]** ADR-042's `725,832 bytes` re-exhaustion figure derives from the **adapter-class model** (`fuel = 29,452 + 27.514 × payload_bytes`; R² = 0.9999999, `legacy-bash-adapter.wasm`, 78 refs per D-964(c)) — NOT from the `validate-cross-site-correspondence` cross-site model as incorrectly stated in the D-966 relay. Literal-shell arithmetic (D-967 independent derivation, two models):

- Adapter model max: `(20,000,000 − 29,452) / 27.514 = 725,832` ✓ — matches ADR-042 §Decision 1
- Cross-site model max: `(20,000,000 − 2,585,970) / 53.18 = 327,454` — different threshold; does NOT match

Applying the adapter-class model to current cycle-artifact sizes (D-967 measurement; file_bytes + ~51,200-byte `last_assistant_message` overhead per ADR-042 §Context worst-case payload construction):

| artifact | file bytes | fuel (adapter model) | vs 20M |
|---|---|---|---|
| decision-log.md | 1,870,808 | 52.91M | EXHAUSTS ~2.6× |
| burst-log.md | 1,800,373 | 50.97M | EXHAUSTS ~2.5× |
| lessons.md | 1,272,854 | 36.46M | EXHAUSTS ~1.8× |
| BC-INDEX.md | 577,860 | 17.34M | ok (~87%) |
| STORY-INDEX.md | 508,006 | 15.42M | ok (~77%) |
| ARCH-INDEX.md | 327,690 | 10.45M | ok at 20M; **exceeds old 10M cap** |

Corroboration: ARCH-INDEX computed fuel 10,454,231 matches ADR-042's measured 10,406,058 to within 0.46%, independently validating the adapter-class model. **rc.24 alone does not remediate the exhaustion class** — decision-log.md, burst-log.md, and lessons.md already exhaust 20M by 1.8–2.6×; size budgets and compaction for the three cycle artifacts are the load-bearing fix. This confirms the human's decision to defer the release.

**Genuine residual caveat (per ADR-042 §Context platform-wide note):** The four validators that actually timed out at D-965 (`validate-factory-path-root`, `validate-input-hash`, `validate-template-compliance`, plus one additional) are NOT the adapter class — each has different WASM bytecode and per-byte cost coefficients; their exact fuel figures remain unmeasured; §Decision 5 / S-21.13 defers per-plugin calibration. The corrected claim: under the adapter-class model (highest measured event count, 838 events), the three cycle artifacts exhaust 20M by 1.8–2.6×, confirming the excluded region (row 4) is NOT safe under a global cap.

This finding rests on the internal ADR self-contradiction (global raise declared alongside per-plugin exclusion claims) plus the observed timeouts at the D-965 burst (four validator plugins timed out on burst-log.md and STATE.md writes), with the corrected arithmetic corroborating the finding rather than dismissing it.

The self-contradiction: §Decision 2 states the raise is global (single default value); §Decision 1 claims other plugins have "independent budgets." If the raise is truly global, all plugins share the same 20M budget (after release) — there are no independent budgets. The excluded region (row 4) cannot be independently calibrated under a global-only raise mechanism. The mandate requires a mutant showing the excluded region can produce a harmful outcome; none is supplied.

**Routing.** architect: adjudicate the self-contradiction in ADR-042 §Decision 1 vs §Decision 2; either (a) introduce per-plugin `fuel_cap` fields (making §Decision 1 row 4 accurate) or (b) retract the independent-budget claim and scope the excluded-region analysis to the global cap; supply the mandatory BOUNDARY-POLARITY mutant; route to human re-ratification of ADR-042 as a material amendment.

---

### MEDIUM

#### F-S2107-P10-007 — MEDIUM — Canonical Principle Rule 3 — `[process-gap]`. ADR-042 §Decision 3 class (b) records "implementer dispatched by team-lead" but no branch carries the fix; `fix/fuel-exhaustion-fail-loud` is recorded ABANDONED. `main.rs` `extract_reason_from_outcome` still matches `PluginResult::Timeout { .. }`, discarding `cause`, so fuel exhaustion and epoch timeout emit identical `block_reason`. Class (a) is anchored; class (b) has no anchor. An unverified "dispatched" status reads as handled when it is not.

**Location:** `specs/architecture/decisions/ADR-042-*.md` §Decision 3 class (b) implementation status annotation; `crates/factory-dispatcher/src/main.rs` `extract_reason_from_outcome` function; STATE.md Active Branches `fix/fuel-exhaustion-fail-loud` row "ABANDONED".

**Clause violated:** Canonical Principle Rule 3 — tech-debt-register entry requires all three: explicit human direction, concrete future dependency, and specific future story/wave anchor; STATUS-DISPATCHED annotation without an anchor branch is semantically a deferred finding; Rule 4 (AI-built defects are AI's responsibility to fix).

**Why it matters.** Class (a) ("update `invoke.rs` to extract `cause` from `PluginResult::Timeout`") is anchored at `fix/fuel-cap-raise-20m` `7cbb9232`. Class (b) ("update `extract_reason_from_outcome` in `main.rs`") was "dispatched" without producing a branch. The absence of a branch means the fix was dispatched but the result was not committed. Agents and humans reading ADR-042 §Decision 3 see both classes as handled; the fuel-vs-epoch disambiguation at `main.rs` is not handled.

**Routing.** implementer: implement `extract_reason_from_outcome` fix on a new branch (`fix/fuel-exhaustion-fail-loud-v2` or equivalent); bundle with or follow `fix/fuel-cap-raise-20m`; attach to S-21.13 or a dedicated story. Architect: annotate ADR-042 §Decision 3 class (b) as "PENDING implementation" rather than "dispatched."

---

#### F-S2107-P10-008 — MEDIUM — TD-VSDD-091 + POLICY 5 stable-anchor. Line-number pins: ADR-040 (§Context, ~line 255) citing "line 294", ADR-042 (~line 96) citing "BC-INDEX line 1464", and **ARCH-INDEX.md row for ADR-042** citing "~415KB". The ARCH-INDEX instance is a live registry row so POLICY 5's historical carve-out does not reach it. Currently accurate, which is why it will decay silently.

**Location:** `specs/architecture/decisions/ADR-040-*.md` §Context body (citing line 294 of `policies.yaml`); `specs/architecture/decisions/ADR-042-*.md` §Empirical Measurements (citing "BC-INDEX line 1464"); `specs/architecture/ARCH-INDEX.md` ADR-042 row body-table cell (citing "~415KB"); against TD-VSDD-091 (narrative spec content MUST cite function names + behavioral anchors, NOT line numbers).

**Clause violated:** TD-VSDD-091 stable-anchor prohibition (line numbers decay on subsequent diffs); POLICY 5 stable-anchor sub-clause.

**Why it matters.** The ADR-040 `line 294` cite decays on every policies.yaml amendment. The ADR-042 `line 1464` cite decays on every BC-INDEX structural change. The ARCH-INDEX `~415KB` is a snapshot figure that will diverge as BC-INDEX grows. TD-VSDD-091 explicitly lists "file.rs:NNN line numbers" as forbidden, with an exception for "justified citations (Red Gate test tables, AC source-of-truth tables, pass-report changelogs)." None of these three are in the excepted class.

**Routing.** architect: replace line-number cites in ADR-040 and ADR-042 with section-anchor forms (e.g., "policies.yaml POLICY 15 ATTESTATION-LOCATION GATE bullet" instead of "line 294"; "BC-INDEX body-table row for BC-5.39.010" instead of "line 1464"). State-manager: correct ARCH-INDEX ADR-042 row to use a structural anchor (e.g., "BC-INDEX body-table 2026-08-09 row count 1,985; grows with registration traffic") instead of snapshot byte count.

---

### LOW

#### F-S2107-P10-009 — LOW — POLICY 14 leg-3 / POLICY 17. BC-5.39.010 body changelog has a `1.15-erratum` row (D-963) with no corresponding `modified[]` frontmatter entry. Legs 1/2/4/5 pass. LOW because the `-erratum` suffix has no registered parity convention — codify the convention rather than patch ad hoc.

**Location:** `specs/behavioral-contracts/ss-05/BC-5.39.010.md` — body § Changelog `1.15-erratum` row; against `modified:` frontmatter array which has entries for v1.15, v1.16, v1.17 but no `1.15-erratum` entry.

**Clause violated:** POLICY 14 leg 3 (`frontmatter_parity_full_file_type_scope` — every `modified[]` entry must have a matching body Changelog row and vice versa); POLICY 17 (version increment without frontmatter parity).

**Evidence — literal shell:**

```
$ python3 -c "
import re
with open('.factory/specs/behavioral-contracts/ss-05/BC-5.39.010.md','r') as f:
    content = f.read()
fm_match = re.match(r'^---\n(.*?)---\n', content, re.DOTALL)
fm = fm_match.group(1)
mod_match = re.search(r'^modified:\n(.*?)(?=^[a-z])', fm, re.MULTILINE|re.DOTALL)
print(mod_match.group(0)[:500])
"
modified:
  - "2026-07-30"
  - "2026-07-30 (v1.1)"
  [... v1.2 through v1.14 ...]
  - "2026-08-08 (v1.15)"
  - "2026-08-08 (v1.16)"
  - "2026-08-08 (v1.17)"
```

`modified[]` has v1.15, v1.16, v1.17 but no `1.15-erratum` entry. Body changelog carries `1.15-erratum` as a versioned row per D-963, creating a leg-3 parity gap.

**Why it matters.** LOW because the `-erratum` suffix is an informal convention with no registered parity rule. The correct fix is to codify whether `-erratum` rows require `modified[]` parity (they should, to maintain auditability) rather than ad-hoc patching.

**Routing.** product-owner: add `"2026-08-08 (v1.15-erratum)"` to `modified[]` OR establish and document that `-erratum` suffix rows are exempt from leg-3 parity; whichever choice is made, codify it in POLICY 14 or an ADR note so future `-erratum` rows are handled consistently.

---

#### F-S2107-P10-010 — MEDIUM — POLICY 22 + `[process-gap]`. Orchestrator relay introduced wrong model attribution into a permanent record. The D-966 precision note added to F-S2107-P10-006 stated that ADR-042's `725,832 bytes` re-exhaustion figure is specific to `validate-cross-site-correspondence`'s cost model (`2,585,970 + 53.18 × bytes`). This attribution is wrong. The correct provenance is the **adapter-class model** (`fuel = 29,452 + 27.514 × payload_bytes`). `(20,000,000 − 29,452) / 27.514 = 725,832`; the cross-site model gives `(20,000,000 − 2,585,970) / 53.18 = 327,454` — a different threshold. The erroneous note also dismissed size-based reasoning as "apples-to-oranges," incorrectly weakening F-006 in the D-966 permanent record.

**Location:** `cycles/v1.0-brownfield-backfill/S-21.07/adversary-pass-10.md` F-S2107-P10-006 body "Precision note" paragraph (added by D-966 state-manager dispatch); `cycles/v1.0-brownfield-backfill/decision-log.md` D-966 block codification language.

**Clause violated:** POLICY 22 — a correction issued under a verification policy is itself load-bearing material subject to that policy. The enforcer is not exempt. The orchestrator invoked POLICY 22 to discipline a subagent's relay and then issued a wrong model attribution without performing a one-line division. This demonstrates that the PROPOSED POLICY 22 ratification-channel extension (Codification 2, D-966) is insufficiently scoped: an orchestrator *correction* carries the same load-bearing character as an original claim and requires literal-shell backing before entering a permanent record.

**Structural class:** F-S2107-P10-003 (false premise reaching ratification, one rung higher) one rung down the chain — F-003 was an adversary finding relayed to a human; F-010 is an orchestrator correction relayed to the state-manager. The relay-channel verification gap existed at the orchestrator→state-manager level at the time of D-966, structurally identical to the adversary→human channel gap that D-966 codification 2 PROPOSED to close.

**Severity rationale:** MEDIUM (not HIGH). Caught and corrected within the same cycle (D-967) before any downstream decision relied on the wrong attribution. The finding logic in F-006 — structural self-contradiction in ADR-042 §Decision 1 vs §Decision 2 — remains valid regardless of which model produced `725,832`. The corrected analysis using the adapter model STRENGTHENS F-006.

**Fix:** D-967 correction burst — F-006 precision note corrected; STATE.md v7.04→v7.05; trajectory count 9→10 swept at all sites.

**Routing.** State-manager self-closes in-burst (D-967). No additional routing required.

---

## Part B — Observations

**O-S2107-P10-01 — Codification 3 (team-lead): Burst-log Block 8 SHA-patch circular regress.** Each SHA-patch updating Block 8 to cite "actual HEAD" creates a new HEAD, leaving Block 8 stale by one — infinite regress terminated only by fiat. The D-965 burst produced three commits (`7540c669` → `5d2902f5` → `cbff0801`). This is structurally identical to the D-912 HEAD-SHA circularity that ADR-040 §Decision 2 cured for POLICY 15 via PARENT-SHA. Architectural observation routed to architect per CLAUDE.md Agent Routing Table; not a finding because the current per-burst SHA-patch approach is functioning as designed (terminated by convention). Codification candidate per team-lead.

**O-S2107-P10-02 — Codification 4 (team-lead): Nested `.factory/.factory` path reproduction confirmed.** A live `.factory/.factory/logs/` directory exists containing `dispatcher-internal-2026-07-27.jsonl` and `dispatcher-internal-2026-08-06.jsonl`. This confirms `fix/nested-factory-path-derivation` (F-S2107-P8-016 + P9-008) is actively reproducing — the fix is on `fix/nested-factory-path-derivation` (pushed at `9afc3226`) but not released. This is a PLAUSIBLE BUT UNCONFIRMED mechanism for the three unexplained dispatcher-log deletions currently in Drift Items: inconsistent `factory_dir` derivation would write logs to one resolved path while another path is expected, potentially creating confusion about which is the "real" log file and leading to restoration cycles. Do NOT record this as established causation; record as a plausible hypothesis for root-cause investigation.

---

## Part C — Analysis

**POLICY 22 relay discipline.** These findings were relayed through the team-lead for state-manager recording. Per POLICY 22, state-manager re-executed all mechanical gate claims independently and recorded first-hand stdout in burst-log D-966 Dim-2. For F-001: COUNT=0 confirmed. For F-002: per-commit counts 0/0/0/3 confirmed. For F-003: commit message and diff confirmed. For F-004: fuel_cap values confirmed. For F-005: ADR frontmatter status fields confirmed. For F-009: modified[] entries confirmed. My re-execution agreed with all relayed mechanical claims; no discrepancies found.

**D-967 correction — F-S2107-P10-006 model attribution error.** The D-966 precision note appended to F-006 by the orchestrator relay incorrectly attributed ADR-042's `725,832 bytes` figure to the `validate-cross-site-correspondence` cross-site model. The correct provenance is the adapter-class model (`fuel = 29,452 + 27.514 × payload_bytes`). The error demonstrates a POLICY 22 gap at the orchestrator→state-manager channel (F-S2107-P10-010). F-006 body corrected in D-967; finding count advanced 9→10; trajectory corrected 9→10.

**Gates re-verified CLEAN (my own re-execution).** POLICY 16 ALLOCATOR-CEILING: `PASS: global max D-965 < D-9000 ceiling; Next allocation: D-966`. POLICY 14 leg-4 four-index: BC v4.55 / VP v2.76 / STORY v4.291 / ARCH v3.52 — frontmatter confirmed from live files. POLICY 21 no-new-.sh: `git diff --name-status origin/develop...5370db80 -- '*.sh'` returns `M plugins/vsdd-factory/tests/run-all.sh` only (modified, not new; pre-existing grandfathered file). D-963/964/965 h2 allocation records: present at decision-log.md lines 15219/15272/15316.

**On codifications 1 and 2.** Both META-LEVEL-25 and the ratification-channel gap are PROPOSED codifications, not ratified. Codification 1 (META-LEVEL-25: literal-shell-attested vacuous gate) extends D-449(a)'s gate-execution requirement to include negative-control verification — a gate whose PASS is always trivially achievable (empty domain, tautological threshold) is non-evidential even if executed. Codification 2 (POLICY 22 ratification-channel extension) addresses the gap whereby POLICY 22's relay-chain discipline did not cover material presented to a human for ratification. Both require negative-control demonstration before presentation for ratification — this is the new discipline applying to itself on first outing.

**Dominant pattern.** Six of nine findings involve a mechanism that appears to function (the gate runs, the BC has a margin assertion, the ADR records ratification) but whose functional premise is vacuous (empty domain, tautological threshold, status field not updated, gate fires in wrong context). This is the next layer of the attestation-migration pattern observed at pass-9: predicates are now asserting the correct *form* but the *domain* or *threshold* is vacuous. F-001 is the sharpest instance: the gate fires in a context where it can never be non-trivially true.

**On the streak.** Streak remains **0/3** — this pass is NOT-CLEAN, reset to 0/3. Trajectory `47→18→25→25→24→20→16→8→10` (D-967 correction: 9→10). Full trajectory: 9 adversary passes, 0 CLEAN verdicts. Findings by severity: **BLOCKER 2, HIGH 4, MEDIUM 3, LOW 1, NITPICK 0 = 10** (D-967 correction: MEDIUM 2→3; total 9→10; F-S2107-P10-010 added).

**Verdict: NOT-CLEAN.**
