---
pass: 10
date: 2026-05-13
producer: adversary
artifacts_reviewed:
  - .factory/specs/architecture/SS-01-hook-dispatcher.md (v1.3)
  - .factory/specs/architecture/SS-02-hook-sdk.md (v1.2)
  - .factory/specs/architecture/decisions/ADR-011-dual-hook-routing-tables.md (changelog 2026-05-13)
  - .factory/specs/architecture/decisions/ADR-004-toml-config.md (v1.2)
  - .factory/specs/architecture/decisions/ADR-015-single-stream-otel-schema.md (v1.10)
  - .factory/specs/architecture/ARCH-INDEX.md (v1.99)
  - .factory/specs/behavioral-contracts/BC-INDEX.md (v2.18)
  - .factory/specs/domain-spec/invariants.md (v1.11)
  - .factory/specs/architecture/SS-09-config-activation.md (line 313)
  - .factory/specs/verification-properties/VP-014.md (lines 45, 107)
  - .factory/specs/domain-spec/business-rules.md (line 85)
  - .factory/specs/prd.md (line 1164)
  - .factory/specs/behavioral-contracts/ss-04/BC-4.04.005.md (line 39)
  - .factory/specs/behavioral-contracts/ss-04/BC-4.05.005.md (line 38)
  - .factory/specs/behavioral-contracts/ss-04/BC-4.07.004.md (line 48)
  - .factory/specs/behavioral-contracts/ss-04/BC-4.08.003.md (line 38)
  - .factory/specs/behavioral-contracts/ss-03/BC-3.04.001.md (line 78)
  - crates/factory-dispatcher/src/registry.rs (verification only)
  - crates/factory-dispatcher/src/host/mod.rs (verification only)
  - crates/factory-dispatcher/src/main.rs (verification only)
  - crates/hook-sdk/src/host.rs (verification only)
verdict: HIGH
findings_count:
  CRITICAL: 0
  HIGH: 1
  MEDIUM: 2
  LOW: 1
  NITPICK: 0
fix_burst: D-462 (renumbered from D-346 per F-CRIT-001 resolution 2026-05-13)
seal_dispatch: D-463 (renumbered from D-347 per F-CRIT-001 resolution 2026-05-13)
engine_baseline: develop@d3ae26a5
nitpick_only_counter_before: 0
nitpick_only_counter_after: 0
trend: "22→11→16→16→12→2→1→4→5→4"
---

# Adversarial Review — Pass 10 (E-10 spec package)

## 1. Closure-Axis Verifications (CC / DD / EE / FF / GG)

**CC — D-15.x decision-number correctness [PASS].** No new mis-citations introduced by D-344.

**DD — trace_id field-name consistency [PASS].** SS-01 + ADR-011 every previously-unannotated occurrence is now annotated or canonicalized.

Literal-shell evidence:
```
$ grep -n 'dispatcher_trace_id' .factory/specs/architecture/SS-01-hook-dispatcher.md \
   | grep -v 'renamed from\|D-344\|DI-017 v1.1'
(zero rows)
```

Lines 38, 59, 91, 132 all carry the `(renamed from dispatcher_trace_id per DI-017 v1.1 / ADR-015 v1.7)` annotation; line 190 is the changelog. F-1 SS-01 closure verified.

**EE — BC version-bump/CHANGELOG accuracy [PASS].** ARCH-INDEX v1.99, SS-01 v1.3, SS-02 v1.2, ADR-004 v1.2 all have D-344 changelog entries dated 2026-05-13.

**FF re-verify — DI-017 propagation [PASS for SS-01 + ADR-011].** All `dispatcher_trace_id` occurrences in SS-01 (lines 38, 59, 91, 132) and ADR-011 (lines 239, 258, 265) are annotated. Column 2 retention of `HostContext.dispatcher_trace_id` in ADR-011 line 239 is semantically correct — verified against `crates/factory-dispatcher/src/host/mod.rs:54` where the Rust struct field name `dispatcher_trace_id: String` is unchanged.

**GG re-verify — schema_version=2 differentiation [PARTIAL FAIL].** See F-1 below. SS-01 lines 50, 80, 144 confirmed `= 2`. ADR-004 lines 44, 96 confirmed `= 2`. ARCH-INDEX line 341 canonical. **BUT ADR-004 line 116 retained `REGISTRY_SCHEMA_VERSION: u32 = 1` in §Source/Origin "Code as-built" reference.**

## 2. New Axes (HH / II)

**HH — Mechanical post-fix verification.** Applied via literal-shell `grep -n`. ARCH-INDEX v1.99 changelog claims "all 5 closure gates returned zero rows post-fix" but this claim is contradicted by:

```
$ grep -n 'REGISTRY_SCHEMA_VERSION.*=\s*1' .factory/specs/architecture/decisions/ADR-004-toml-config.md
116:- **Code as-built:** `crates/factory-dispatcher/src/registry.rs::REGISTRY_SCHEMA_VERSION` (`REGISTRY_SCHEMA_VERSION: u32 = 1`),
```

Net: HH directly disproves D-345 seal's "5 zero-row gates" claim. The fix-burst's closure narrative is the **exact same META-LEVEL-24 partial-fix pattern** pass-9 surfaced. Pass-9 closure proposals listed lines 44 + 96 only (omitted 116); pass-10 D-344 closure executed exactly the pass-9 narrative without re-scoping the file with a fresh grep. This is a S-7.01 partial-fix regression at the fix-burst level + a pass-9 axis-GG scope-narrowness defect that survived into D-344.

**II — Cross-cycle propagation audit.** Stale `REGISTRY_SCHEMA_VERSION = 1` / `schema_version = 1` references remain in: VP-014.md (lines 45, 107), business-rules.md (line 85), prd.md (line 1164), BC-4.04.005/4.05.005/4.07.004/4.08.003 (Preconditions). These are F2-ADR-019 cross-cycle propagation gaps not addressed by D-344. See F-2.

`dispatcher_trace_id` sibling-doc audit: BC-3.04.001 line 78 retains `SS-03 (Observability Sinks)` — a sibling-pattern instance of F-3 (which fixed only SS-01 line 122). Per TD-VSDD-060 sibling-site sweep discipline, F-3 closure was scope-narrow.

## 3. Findings

### F-1 [HIGH] ADR-004 partial-fix regression — line 116 retains `REGISTRY_SCHEMA_VERSION: u32 = 1` despite F-4 closure claim

**File:** `.factory/specs/architecture/decisions/ADR-004-toml-config.md`
**Location:** Line 116.
**Defect:** D-344 fix burst patched lines 44 + 96 only. Line 116 `§Source/Origin → Code as-built` still cites `REGISTRY_SCHEMA_VERSION: u32 = 1`. The actual production code (`crates/factory-dispatcher/src/registry.rs:20`) is `REGISTRY_SCHEMA_VERSION: u32 = 2`. ADR-004 v1.2 changelog narrative (line 123) reads "F-4 cross-cycle schema_version sweep — hooks-registry.toml schema_version corrected to 2 at §Amendment (line 44) and §Status (line 96)" — but the §Source/Origin reference was not touched.

**S-7.01 partial-fix verdict:** HIGH severity. ARCH-INDEX v1.99 changelog asserts "5 closure gates returned zero rows post-fix" — literal grep disproves this. Implementer self-disclosure (D-344 closure narrative) is NOT authoritative.

**Routing:** Architect (re-open D-344, expand to ADR-004 line 116).

**Closure proposal:**
- ADR-004 line 116: `crates/factory-dispatcher/src/registry.rs::REGISTRY_SCHEMA_VERSION (REGISTRY_SCHEMA_VERSION: u32 = 2 post-ADR-019 / F2 cycle 2026-05-07; was = 1 pre-ADR-019)`
- ADR-004 v1.2→v1.3 changelog row citing line 116 added to F-4 closure scope.

**[process-gap]:** Pass-9 F-4 closure proposal enumerated only 2 of 3 sites; pass-10 D-344 executed the narrow proposal without re-greping ADR-004. The D-449(a) literal-shell-execution-evidence rule should be applied **pre-fix** (to determine fix scope) AND **post-fix** (to verify zero-row gate). Pass-9 + pass-10 D-344 demonstrate the same META-LEVEL-24 pattern at the fix-burst-scope-determination layer.

### F-2 [MEDIUM] Cross-cycle propagation gap — 8 stale `REGISTRY_SCHEMA_VERSION/schema_version = 1` references survived D-344 axis II sweep

**Files / Locations:**
- `.factory/specs/verification-properties/VP-014.md` lines 45, 107
- `.factory/specs/domain-spec/business-rules.md` line 85
- `.factory/specs/prd.md` line 1164
- `.factory/specs/behavioral-contracts/ss-04/BC-4.04.005.md` line 39
- `.factory/specs/behavioral-contracts/ss-04/BC-4.05.005.md` line 38
- `.factory/specs/behavioral-contracts/ss-04/BC-4.07.004.md` line 48
- `.factory/specs/behavioral-contracts/ss-04/BC-4.08.003.md` line 38

**Defect:** F2 ADR-019 (2026-05-07) bumped `REGISTRY_SCHEMA_VERSION` 1→2. ARCH-INDEX line 341 is canonical (`= 2`). Domain-spec, PRD, VP, and 4 BC-4.x Preconditions retain `= 1`. These directly contradict POLICY 6 (canonical-name SoT) per the schema-version-spelling parity argument. D-344's "cross-cycle annotation F-4" claim is scope-narrow — only ADR-004 was touched, not the 7+ sibling references that share the same gap.

**Scope adjudication:** ADR-004 was prioritized in D-344 because it surfaced directly in pass-9. But "cross-cycle propagation audit" (pass-10 axis II) requires sweeping ALL files that cite the same canonical value. F2 ADR-019's sibling-sweep was incomplete — D-344 inherited the gap rather than closing it.

**Severity:** MEDIUM (not HIGH) because (a) F-4 was already scoped pending intent verification re: F2 cycle owns these files; (b) ARCH-INDEX canonical row is correct, so the contradiction is detectable. But the production-grade default per CLAUDE.md Canonical Rule 4 says "AI-built defects are the AI's responsibility to fix" — passing the buck to "F2 cycle owns this" is the wrong default. Surface the finding and request scope expansion.

**Routing:** Architect (sibling-sweep fix-burst) → state-manager (changelog rollup).

**Closure proposal:**
- All 8 occurrences: replace `schema_version = 1` / `REGISTRY_SCHEMA_VERSION = 1` with `schema_version = 2` / `REGISTRY_SCHEMA_VERSION = 2 (post-ADR-019 F2 2026-05-07)`.
- BC-4.x Preconditions are particularly load-bearing: those BCs ASSERT the hooks-registry passes schema validation; with the stale `= 1` precondition they assert a contradictory invariant.
- VP-014 line 107: "both currently 1" → "REGISTRY_SCHEMA_VERSION = 2 (post-ADR-019), INTERNAL_EVENT_SCHEMA_VERSION = 1".

**[process-gap]:** D-344 closure narrative claimed "cross-cycle annotation F-4 propagated to E-10 ADR-004 per user direction" — but only the single file mentioned in pass-9 was touched. Cross-cycle propagation discipline at the **fix-burst** layer requires `grep -rn <pattern> .factory/specs/` and patching every hit, not just the file pass-9 enumerated.

### F-3 [MEDIUM] Sibling-pattern F-3 instance — BC-3.04.001 line 78 retains stale "SS-03 (Observability Sinks)" subsystem name

**File:** `.factory/specs/behavioral-contracts/ss-03/BC-3.04.001.md`
**Location:** Line 78 (Architecture Module row).
**Defect:** `| Architecture Module | SS-03 (Observability Sinks) — crates/factory-dispatcher/src/sinks/router.rs |`. Per ADR-015 D-15.1, the canonical subsystem name is "SS-03 (Event Emission (OTel-Aligned))". Pass-9 F-3 fixed SS-01 line 122 but did not sibling-sweep this BC. Per TD-VSDD-060 sibling-site sweep discipline + POLICY 6 (canonical-name SoT), this is a sibling-pattern partial-fix.

**Severity moderation:** MEDIUM (not HIGH) because BC-3.04.001 has `lifecycle_status: deprecated` (superseded by BC-3.04.004). However, deprecated BCs are still in the corpus and still drift-blocking per POLICY 6.

**Routing:** Architect/PO (decide if BC-3.04.001 should be retired-not-merely-deprecated, OR fix the subsystem-name cite + add changelog).

**Closure proposal (if accepted in scope):**
- BC-3.04.001 line 78: `SS-03 (Event Emission (OTel-Aligned)) — historically referenced crates/factory-dispatcher/src/sinks/router.rs (retired per ADR-015 D-15.1; BC-3.04.001 deprecated, superseded by BC-3.04.004)`.
- Add changelog row citing E-10 pass-10 sibling-sweep.

### F-4 [LOW] (pending intent verification) — main.rs lines 280-289 plugin-envelope JSON key `dispatcher_trace_id` may surface in plugin-visible payload

**File:** `crates/factory-dispatcher/src/main.rs`
**Locations:** Lines 280, 288.
**Defect:** Dispatcher injects `"dispatcher_trace_id"` key into the plugin's stdin envelope. This is the **plugin HookPayload** shape (SDK-facing), distinct from the **event wire format** in `events-*.jsonl`. DI-017 v1.1 wire-format exclusivity applies only to the latter. SDK-facing payload field-name is governed separately (SS-02 explicitly notes "SDK API surface was not renamed").

**Why flagged LOW:** the dispatcher-to-plugin envelope is arguably a different interface from the OTel emission wire format. However, an external observer reading SS-02 + DI-017 + BC-1.11.001 might reasonably interpret "the legacy alias `dispatcher_trace_id` MUST NOT appear in serialized output" to apply to the stdin envelope JSON as well, since stdin is also serialized JSON. The distinction is implicit in DI-017 v1.1 wording and could benefit from explicit clarification in DI-017 or ADR-015.

**Severity:** LOW (pending intent verification per S-7.01). The dual surface (SDK API + envelope field-name retained) is consistent with the F-5 closure rationale, but there is no spec artifact that explicitly says "the envelope JSON `dispatcher_trace_id` key is intentionally retained alongside the WIRE rename".

**Routing:** Architect (intent adjudication; add a sentence to DI-017 §wire-format-exclusivity scoping the rule to `events-*.jsonl` and explicitly carving out the plugin envelope).

**Closure proposal (if accepted):**
- DI-017 §wire-format-exclusivity: append "Scope: this rule governs `events-*.jsonl` emission (and equivalent OTel collector exports). The plugin stdin envelope (`HookPayload` JSON) retains `dispatcher_trace_id` as the SDK-facing payload field name, consistent with SS-02 SDK API surface."

## 4. Observations

- **O-1 [process-gap]** D-344 fix-burst closure narrative ("all 5 closure gates returned zero rows post-fix") was not produced by literal-shell evidence per D-449(a). It was narrative-attested. Re-running `grep -n 'REGISTRY_SCHEMA_VERSION.*=\s*1' .factory/specs/architecture/decisions/ADR-004-toml-config.md` returns 1 row. The brownfield cycle's adoption of F5 D-449(a) literal-shell discipline is **structurally incomplete**: the discipline applies post-fix but not to the **fix-burst SCOPE determination**. Recommendation: fix-burst scope-determination must run `grep -rn <pattern> .factory/specs/` PRE-fix to enumerate the patch set, not rely on the prior-pass adversary report's site list (which can itself be scope-narrow).

- **O-2 [info]** SS-09-config-activation.md line 313 cites `REGISTRY_SCHEMA_VERSION = 1` — this is **contextually correct** since it's quoting "The original Cross-Cutting section stated:" with the §"Corrected:" amendment immediately following at line 316. No defect.

- **O-3 [info]** ADR-011 dual-routing-tables.md frontmatter has no `version:` field — ADR convention here is changelog-only. Consistent with other ADRs.

## 5. Novelty Assessment

**Novelty: 6/10.** Pass-10 confirms the same META-LEVEL-24 partial-fix pattern recurred at the brownfield fix-burst layer:

- Pass-8 D-336 had narrative-only "swept N hits" with no grep verification → pass-9 exposed 3 missed renames.
- Pass-9 F-4 closure proposal enumerated only 2 of 3 ADR-004 sites → pass-10 D-344 fix executed exactly the narrow proposal → ADR-004 line 116 survived → pass-10 axis HH grep exposed it.

The pattern is: **closure-proposal enumeration is treated as authoritative scope; pre-fix re-grep is skipped.** This is a NEW class of finding for the brownfield cycle: not just narrative-attestation of post-fix gates (META-LEVEL-24) but **inheriting scope-narrowness from the prior-pass adversary report**.

The 4 findings are mostly partial-fix regressions (F-1) + sibling-sweep gaps (F-2, F-3) + intent-ambiguity (F-4). No novel content defects. Structurally healthy axis-discovery: pass-10's HH + II axes surface the gap the prior 9 passes' axes couldn't.

**ADR-013 / BC-5.39.001 convergence counter:** does NOT advance. Remains 0/3 NITPICK_ONLY.

**Trend:** 22 → 11 → 16 → 16 → 12 → 2 → 1 → 4 → 5 → **4**. Asymptotic floor [4..5] HIGH-MEDIUM findings as previously suspected; convergence remains structurally blocked under narrative-attestation-of-fix-burst-scope at the brownfield layer.

## 6. Verdict

**HIGH** — 1 HIGH + 2 MED + 1 LOW. F-1 is the structurally significant one: a S-7.01 partial-fix regression at the fix-burst scope-determination layer, mechanically disproved by literal-shell. F-2 is cross-cycle propagation scope-narrowness. F-3 is sibling-doc scope-narrowness. F-4 is intent-pending and pending architect adjudication.

## 7. Pass-11 Axes Recommendation

- **HH-2 (refinement of HH):** Pre-fix scope-determination grep — every fix-burst must `grep -rn <pattern> .factory/specs/` BEFORE patching, not rely on the prior-pass adversary's enumerated site list. Document the grep output in the fix-burst commit body.
- **II-2 (refinement of II):** Cross-doc sibling-sweep at fix-burst time — when canonical value changes (subsystem name, schema_version), the fix-burst MUST sweep ALL spec files (`.factory/specs/`) not just the files surfaced by the adversary. Adversary's per-pass scope is bounded by token budget; fix-burst scope is bounded by `grep -r`.
- **JJ (new):** Production-grade-default audit on D-344 attestation — when ARCH-INDEX changelog asserts "all N gates returned zero rows post-fix", pass-N+1 must mechanically verify each N gate, NOT inherit the claim. Implementer self-disclosure is not authoritative (CLAUDE.md Standing Rule 3 §1).
- **FF/GG re-verify (post fix-burst):** Confirm D-346 closes F-1/F-2/F-3 exhaustively (literal-shell `grep -c` evidence per F5 D-449(a), captured in burst record).

## 8. Fix-Burst Proposal Sketch (D-346) — NOT EXECUTED, only proposed

**Architect (primary, F-1/F-2/F-3 fixes):**
- ADR-004-toml-config.md v1.2→v1.3: line 116 `REGISTRY_SCHEMA_VERSION: u32 = 1` → `= 2 (post-ADR-019 F2 2026-05-07)`. Changelog row.
- VP-014.md v[next]: lines 45, 107 update.
- business-rules.md v[next]: line 85 update.
- prd.md v[next]: NFR-MAINT-004 row update.
- BC-4.04.005, BC-4.05.005, BC-4.07.004, BC-4.08.003: Precondition 2 `schema_version = 1` → `schema_version = 2 (post-ADR-019)` + version bump + changelog row each.
- BC-3.04.001 v1.1→v1.2: line 78 subsystem-name canonicalization + changelog.

**Architect (F-4 intent adjudication):**
- DI-017 §wire-format-exclusivity: explicit scope-statement that envelope-payload-JSON is carved out from the rule.

**State-manager (seal D-347):**
- ARCH-INDEX v[next]: changelog row citing D-346 + **literal-shell-execution-evidence** captured in commit body per F5 D-449(a). Specifically: `grep -rn 'REGISTRY_SCHEMA_VERSION.*=\s*1\|hooks-registry.*schema_version\s*=\s*1' .factory/specs/` MUST return zero rows post-fix.
- BC-INDEX cite-refresh.
- Add NEW pass-10 D-346 burst-log entry with **pre-fix scope-grep output** (axis HH-2) embedded.

---

**Pass-10 produced 4 findings (1 HIGH + 2 MED + 1 LOW). NITPICK_ONLY counter stays at 0/3. Convergence requires three consecutive NITPICK_ONLY passes per BC-5.39.001 / ADR-013.**
