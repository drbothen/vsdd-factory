---
document_type: lessons
cycle: v1.0-feature-plugin-async-semantics-pass-1
producer: state-manager
version: "1.2"
last_updated: 2026-05-09
---

# Lessons Learned — v1.0-feature-plugin-async-semantics-pass-1

> Per S-7.02 process-gap codification policy: every process-gap finding must have either
> a follow-up story (status: draft) or a justified deferral. Entries tagged `[codified]`
> have been fully recorded here with actionable discipline.

---

## F5 pass-18 process-gap findings (2026-05-08)

### L-P18-001 — Sibling-hook predicate sweep mandatory after path-matching changes [codified]

**Source:** F-P18-001 (HIGH)
**Finding:** fix-burst-17 sub-burst 1 (cc5a016b) fixed an absolute-path false-negative in
`validate-stable-anchors`. The same bug existed in sibling plugin `validate-artifact-path`
(`matches_canonical` + `hook_logic` predicates), discovered as F-P18-001 in pass-18.

**Lesson:** When a fix-burst changes any path-matching predicate in a hook plugin, the
implementer MUST grep all other hook-plugin crates for the same predicate pattern before
declaring the fix complete. A single-crate fix is insufficient when multiple crates share
logically-equivalent predicate logic.

**Rule:** "Sibling-hook predicate sweep mandatory after any path-matching predicate change."

**Scope:** Applies to all hook plugin crates under `plugins/` that share predicate
patterns (e.g., `is_spec_target`, `matches_canonical`, `hook_logic`).

**Disposition:** No new story required — codified as discipline rule. Add to fix-burst
checklist template when path-matching changes are involved.

> **Codified pre-L-P26-001 — exempt from retroactive verification block.** L-P26-001 (the rule requiring `**Verified retroactively in fix-burst-N:**` blocks) was not codified until fix-burst-25 sub-burst 3. L-P18-001 predates that mandate and is not subject to retroactive verification. Historical application: sibling-hook predicate sweep was applied in fix-burst-18 (validate-artifact-path `matches_canonical` + `hook_logic` predicates, commit 026272ae).

---

### L-P18-002 — TD-VSDD-091 sweep checklist must include prose-form sweep [codified]

**Source:** F-P18-002 (MEDIUM)
**Finding:** The TD-VSDD-091 6-chunk mass-sweep migrated `<word>.<ext>:NNN` patterns
in spec body text but did not scan for prose-form references ("at line NNN", "on line NNN",
"see line NNN"). Three BCs (BC-1.05.035/036, BC-2.02.011) contained prose-form line
references that were not caught by the automated hook or the chunk sweep.

**Lesson:** TD-VSDD-091 sweeps MUST include a manual prose-form sweep step in addition
to automated hook detection. The hook catches machine-readable `<word>.<ext>:NNN`
patterns; it does NOT catch natural-language prose references to line numbers.

**Rule:** "TD-VSDD-091 sweep checklist must include prose-form sweep:
`grep -r 'at line [0-9]\+\|on line [0-9]\+\|see line [0-9]\+' .factory/specs/`"

**Disposition:** Codified as sweep discipline. Apply at start of every TD-VSDD-091 chunk
sweep before the automated pass.

> **Codified pre-L-P26-001 — exempt from retroactive verification block.** Predates L-P26-001 mandate (fix-burst-25). Historical application: prose-form sweep applied retroactively in fix-burst-19 (BC-7.03.009 v1.3, commit fixing plural/range forms) and fix-burst-20/21/22 extended sweeps.

---

### L-P18-003 — Mass-sweep touching >5 BC/VP files must update BC-INDEX/VP-INDEX with aggregated changelog [codified]

**Source:** F-P18-003 (MEDIUM)
**Finding:** The 6-chunk TD-VSDD-091 mass-sweep touched ~50 BCs and 5 VPs across 7
commits. BC-INDEX and VP-INDEX were not updated with an aggregated changelog entry
summarizing the sweep, leaving index audit trail incomplete.

**Lesson:** Any fix-burst that touches more than 5 BC files OR more than 2 VP files
MUST include an aggregated changelog entry in BC-INDEX and/or VP-INDEX respectively.
Individual file-level changelogs are not sufficient — the index must record the mass
event as a single navigable entry with commit references.

**Rule:** "Fix-burst touching >5 BC/VP files must update BC-INDEX/VP-INDEX with
aggregated changelog entry citing all commits in the sweep."

**Disposition:** Codified as index maintenance policy. Add to state-manager sub-burst
checklist for mass sweeps.

> **Codified pre-L-P26-001 — exempt from retroactive verification block.** Predates L-P26-001 mandate (fix-burst-25). Historical application: BC-INDEX and VP-INDEX aggregated changelog entries have been included in all fix-bursts from fix-burst-18 onward (fix-burst-18 through fix-burst-25).

---

### L-P18-004 — When a TD entry says ENFORCEMENT IMPLEMENTED, any enforcement-related fix must update the TD entry [codified]

**Source:** F-P18-004 (MEDIUM)
**Finding:** TD-031 entry said "ENFORCEMENT IMPLEMENTED fix-burst-16 (bb661eaa)" with
test counts 58/58. Fix-burst-17 added tests (58→62 for validate-stable-anchors;
54→58 for validate-artifact-path) and fixed a production enforcement gap (cc5a016b),
but TD-031 was not updated to reflect these changes.

**Lesson:** When a TD entry carries status "ENFORCEMENT IMPLEMENTED" (or similar
enforcement-complete claim), any subsequent fix-burst that:
  (a) adds enforcement tests, or
  (b) fixes an enforcement gap (false-negative, false-positive, sibling propagation), or
  (c) changes test counts associated with enforcement

MUST update the TD entry in the same fix-burst. The TD register is the canonical
source of enforcement status — a stale entry implies enforcement is weaker than it is.

**Rule:** "When a TD entry's description includes 'ENFORCEMENT IMPLEMENTED',
state-manager sub-burst checklist must include: verify TD entry test counts and
enforcement description match current reality."

**Disposition:** Codified as TD register maintenance policy. Applied retroactively to
TD-031 in this burst (cc5a016b + 8b4f697f recorded; test counts updated; Kani deferral noted).

> **Codified pre-L-P26-001 — exempt from retroactive verification block.** Predates L-P26-001 mandate (fix-burst-25). Historical application: TD-031 updated in fix-burst-17 (cc5a016b, 8b4f697f). All subsequent fix-bursts with enforcement changes have updated TD entries.

---

## F5 pass-19 process-gap findings (2026-05-08)

### L-P19-001 — Lesson backfill discipline [codified]

**Source:** F-P19-001 (process-gap)
**Finding:** L-P18-002 was codified in fix-burst-17 with tag `[codified]` but no retroactive
corpus-wide sweep was executed in the same fix-burst to apply the codified rule. The next
adversary pass (pass-19) found sibling instances of the same pattern (F-P19-001: 18 grep
matches; 6 body refs across 4 files not migrated).

**Lesson:** When a lesson L-NNN is codified (`[codified]` tag), the SAME fix-burst that
codifies the lesson MUST also run a corpus-wide retroactive sweep applying the codified rule.
Codification without retroactive backfill is structurally insufficient — the next adversary
pass will find sibling instances of the same pattern (cf. L-P18-002 codified pass-18,
un-applied retroactively, re-found pass-19 as F-P19-001).

**Rule:** Any state-manager fix-burst that adds an entry to lessons.md MUST include in the
same commit (or in a sub-burst within the same fix-burst) a corpus-wide grep+migration step
for the codified pattern.

**Trigger:** When the orchestrator signals "lesson codified", the state-manager checklist
must include: "Run retroactive corpus sweep for codified pattern before declaring fix-burst
complete."

**Disposition:** Codified as state-manager sub-burst discipline. Applies retroactively: any
future lessons.md append must be accompanied by a corpus sweep in the same fix-burst.

> **Codified pre-L-P26-001 — exempt from retroactive verification block.** Predates L-P26-001 mandate (fix-burst-25). Historical application: corpus sweeps have been run in each codifying fix-burst from fix-burst-19 onward per this rule.

---

### L-P19-002 — Kani harness sync policy [codified]

**Source:** F-P19-002 (process-gap)
**Finding:** Implementation code under `#[cfg(kani)]` in validate-artifact-path had stale
`kani::assume(!path.starts_with(".factory/"))` after the absolute-path fix in fix-burst-17
(8b4f697f). The Kani assumption no longer matched the new behavior — proof was unsound
vs absolute-path matching — but this was not detected until the next adversary pass.

**Lesson:** When implementation code under `#[cfg(kani)]` (inline) or `tests/*kani*.rs`
(external) is changed by a non-Kani fix, the same commit MUST also:
  (a) update the Kani assumption/assertion to match the new behavior, OR
  (b) mark the proof as deferred-fix-pending in the spec artifact (e.g.,
      `lifecycle_status: deferred` in VP frontmatter) AND add a TD entry citing the deferral.

**Failure mode:** Implementation changes ship; Kani assumptions go stale; proof is unsound
but undetected until the next Kani run (which may be never if CI doesn't gate Kani).

**Suggested codification mechanism:** Future POLICY 13 candidate. In the interim, add to
the fix-burst checklist: "If any `#[cfg(kani)]` or `kani_*` test file is adjacent to
changed code, verify Kani assumptions still match the new behavior."

**Disposition:** Codified as implementer discipline. Applied retroactively: fix-burst-18
sub-burst 1 (026272ae) updated VP-070 Proof 2 assumption to exclude both relative and
absolute .factory/ paths. Actual proof execution deferred to CI pending rustc version
upgrade (cargo kani 0.67.0 → rustc 1.93.0-nightly < workspace 1.95).

**Verified retroactively in fix-burst-18 + fix-burst-19:**
- VP-070 (assumption tightened in 026272ae fix-burst-18 sub-burst 1)
- VP-071 (`crates/hook-plugins/validate-per-story-adversary-convergence/src/lib.rs::kani_proofs` — `kani::assume(passes < 3)` matches production threshold check at `lib.rs::hook_result_for` (line 131: `if s.passes_clean < 3 { ... CONVERGENCE_PASSES_INSUFFICIENT ... }`) — audited in pass-21 review, no staleness detected) <!-- F-P21-002: corrected fabricated symbol `passes_clean_to_close` → real production fn `hook_result_for` per POLICY 4 -->
- VP-077 (`crates/factory-dispatcher/src/partition.rs::kani_proofs` — `kani::assume(n <= 4)` is a tractability bound, not a behavior assumption; audited in pass-20 review, no staleness detected)

All three active Kani VPs audit-clean as of fix-burst-19.

> **Codified pre-L-P26-001 — exempt from retroactive verification block.** Predates L-P26-001 mandate (fix-burst-25). Historical application: Kani harness sync applied retroactively in fix-burst-18 sub-burst 1 (026272ae) for VP-070; VP-071 and VP-077 audited clean in passes 21 and 20 respectively.

---

## L-P20-001 [codified]: Literal-vs-class grep discipline in retroactive-sweep

**Source:** pass-20 review / fix-burst-19 sub-burst 2 — F-P20-001 extended prose sweep gap.

When applying L-P19-001 (corpus-wide retroactive sweep for a codified rule), the grep MUST use the SEMANTIC pattern CLASS, not the LITERAL string that triggered the rule.

**Failure mode:** codifying lesson is triggered by a specific syntactic instance (e.g., L-P18-002 caught `at line 152`, singular form). Subsequent retroactive sweep uses the literal grep `at line [0-9]+` and misses sibling pattern instances (`at lines 148-224`, `between lines 575-731`, plural/range forms).

**Rule:** when codifying a lesson, the lesson author MUST document the PATTERN CLASS (e.g., "any prose-form line citation including singular/plural/range forms") in addition to the LITERAL grep. State-managers applying L-P19-001 MUST use the broader class grep.

**Example refined grep for prose-form line citations:**
`\bat lines? [0-9]+(-[0-9]+)?\b|\bbetween lines? [0-9]+ and [0-9]+\b|\b(lines?|line) [0-9]+(-[0-9]+)?\b`

[codified] — fix-burst-19 sub-burst 2.

> **Codified pre-L-P26-001 — exempt from retroactive verification block.** Predates L-P26-001 mandate (fix-burst-25). Historical application: broader class grep applied in fix-burst-20 sub-burst 1 (plural/range forms), fix-burst-21 sub-burst 1 (double-backtick postcondition form), fix-burst-22 sub-burst 1 (cross-subsystem extension).

---

## L-P20-002 [codified]: Index-of-indexes cite-refresh discipline (parent-pointer staleness)

**Source:** pass-20 review / fix-burst-19 sub-burst 2 — F-P20-002 ARCH-INDEX cite stale 10 versions.

ARCH-INDEX body cites the BC-INDEX/VP-INDEX/STORY-INDEX versions explicitly (e.g., "Total BCs: 1947 (per BC-INDEX v1.33)"). When a child index is version-bumped, the parent index cite MUST be refreshed in the same burst.

ARCH-INDEX self-codified this rule at v1.18 (2026-05-07) and reinforced at v1.19. The rule was systematically ignored across 15 consecutive fix-bursts (4 through 18); pass-20 found ARCH-INDEX cite was 10 versions stale (BC-INDEX v1.33 vs current v1.43).

Three consecutive HIGH passes (P18-001 sibling-hook bug, P19-001 codified-not-applied, P20-002 cite-stale-15-fix-bursts) demonstrate that prose codification of cite-refresh discipline is structurally insufficient.

**Recommended enforcement:** hook-based parser that compares ARCH-INDEX body cites to current BC-INDEX/VP-INDEX/STORY-INDEX frontmatter versions; blocks Edit/Write to ARCH-INDEX or any of the three child indexes if cites are stale by >0 versions. Tracked in follow-up story S-15.03 (see below).

[codified] — fix-burst-19 sub-burst 2.

> **Codified pre-L-P26-001 — exempt from retroactive verification block.** Predates L-P26-001 mandate (fix-burst-25). Historical application: ARCH-INDEX cite-refresh applied in fix-burst-19 sub-burst 2 (ARCH-INDEX v1.19); subsequently applied in every fix-burst from fix-burst-20 onward (fix-burst-20: v1.20, fix-burst-21: v1.21, fix-burst-22: v1.28, fix-burst-23: v1.29, fix-burst-24: v1.30, fix-burst-25: ARCH-INDEX not bumped but BC-INDEX/VP-INDEX citations refreshed).

---

## F5 pass-21 process-gap findings (2026-05-08)

### L-P21-001 [codified]: POLICY 4 audit-trail integrity — every cited production symbol MUST be grep-verifiable in the codebase

**Source:** F-P21-002 (process-gap)

When a lesson, audit-trail, or burst-log entry cites a production code symbol (function, struct,
method, constant), the symbol name MUST exist in the codebase at the time of writing. Audit
claims that name fabricated symbols are unauditable post-facto and erode the audit trail's value
as a verification artifact.

**Failure mode (F-P21-002):** L-P19-002 disposition cited `lib.rs::passes_clean_to_close` to
claim VP-071 audit-clean status. Subsequent grep across entire codebase returned 0 matches. The
actual production fn is `hook_result_for`. Future auditor following the citation would (a)
abandon the audit, or (b) propagate the fabrication.

**Rule:** any lesson author or auditor citing a production symbol MUST run
`grep -rn "<symbol_name>" /Users/jmagady/Dev/vsdd-factory/` and confirm at least one match before
commit. The grep MUST be cited inline in the lesson narrative if the symbol is non-obvious, OR
the symbol must be cross-linked to its file:line via a stable anchor (per TD-VSDD-091).

**Suggested codification mechanism:** a `validate-symbol-cite` lint hook could grep for cited
`<file>::<symbol>` patterns and assert the symbol resolves. (Future POLICY 13 candidate;
combinable with S-15.03 hook scope.)

[codified] — fix-burst-20 sub-burst 2.

**Verified retroactively in fix-burst-21 + fix-burst-22:**
- fix-burst-21 sub-burst 2 (`2ea5ee5a`): 7 fabrications corrected (BC-1.07.005, BC-1.07.006, edge-cases.md, domain-events.md, VP-016, VP-043, BC-1.14.001).
- fix-burst-22 sub-burst 2 (`60072605`): F-P23-003 closed — BC-1.07.005 + BC-1.07.006 H1 + BC-INDEX rows 226-227 + VP-043 §Source Contract all rebranded to cite real test fn `loads_generated_registry_from_disk`. Per L-P21-001 + new L-P23-001 (codified below) — ALL cite sites of the same fabricated symbol now patched, not just §Source Evidence.

> **Codified pre-L-P26-001 — exempt from retroactive verification block for the verification block itself.** Verification blocks above pre-date the formal L-P26-001 block format but constitute substantive equivalent evidence of retroactive application.

---

## F5 pass-22 process-gap findings (2026-05-08)

### L-P22-001 [codified]: Five-pass HIGH streak — prose-only codification is structurally non-convergent

**Source:** F5 cycle pattern observation — fix-burst-21 sub-burst 4.

The F5 cycle has produced 5 consecutive HIGH passes (P18-001, P19-001, P20-001/002, P21-001, P22-001) all from the same recurrence-of-codified-lesson-applied-too-narrowly pattern:

- fix-burst-17 codified L-P18-002, applied literally → P19-001 sibling
- fix-burst-18 codified L-P19-001, applied retroactively → P20-001 sibling (singular vs plural)
- fix-burst-19 codified L-P20-001 + L-P20-002, applied within burst → P21-001 sibling (range vs single-line)
- fix-burst-20 applied F-P21-001 fix to 10 BCs + codified L-P21-001/002 → P22-001 88 sibling BCs + P22-002 cycle-anchor fabricated symbols
- fix-burst-21 applied broadest sweep yet (88 BCs + 7 additional fabrications via L-P21-001 retroactive) → pass-23 awaited

Empirical observation: each lesson codifies a slightly broader rule, but the same-burst retroactive application is bounded by the human author's grep query at codification time. The next adversary pass uses a different query class and finds new sibling instances.

**Strategic conclusion:** prose-only codification is structurally insufficient for sibling-class sweep discipline. **MUST implement S-15.03 (mechanical hook enforcement)** to break the recurrence pattern. Continued prose-only iteration has expected value below the cost of the hook implementation.

**Process directive:** if pass-23 also produces HIGH from a sibling-class recurrence, the F5 chain MUST halt and the orchestrator MUST surface the strategic decision (halt-and-implement-S-15.03) to the user before pass-24 dispatch.

[codified] — fix-burst-21 sub-burst 4.

> **Codified pre-L-P26-001 — exempt from retroactive verification block.** Predates L-P26-001 mandate (fix-burst-25). L-P22-001 is a strategic-conclusion lesson (no specific artifacts to patch); its "application" is the mandate to implement S-15.03, which remains the standing recommendation.

---

### L-P21-002 [codified]: Story epic anchor must match epic title and subsystems_affected

**Source:** F-P21-003 (process-gap)

When filing a new story, the `epic:` and `subsystems:` frontmatter MUST be cross-checked against
the epic file's title and `subsystems_affected:` list. The story's scope must align with the
epic's stated purpose.

**Failure mode (F-P21-003):** S-15.03 (index-cite-refresh-hook + lessons retroactive-sweep
verification) was filed under E-15 (Plugin Async Semantics — Registry-Layer Partition). Story
scope and epic scope are unrelated. Subsystems `[SS-04]` was not consistent with E-15's
`subsystems_affected: [SS-01, SS-07, SS-09]`.

**Rule:** state-manager filing a story MUST: (a) read the parent epic file to confirm scope
alignment, (b) verify subsystems is a subset of (or expansion-justified extension of) the epic's
`subsystems_affected:`, (c) write a §Anchor Justification section in the story body citing the
epic alignment per POLICY 5.

[codified] — fix-burst-20 sub-burst 2.

> **Codified pre-L-P26-001 — exempt from retroactive verification block.** Predates L-P26-001 mandate (fix-burst-25). Historical application: S-15.03 re-anchored from E-15 to E-12 in fix-burst-20 per this lesson (F-P21-003 closed).

---

## F5 pass-23 process-gap findings (2026-05-09)

### L-P23-001 [codified]: All-cite-sites discipline — when fixing a fabricated symbol, ALL cite sites of that symbol within the spec corpus MUST be updated in the SAME fix-burst

**Source:** F-P23-003 (process-gap)

> Failure mode (F-P23-003): fix-burst-21 sub-burst 2 corrected the fabricated symbol `every_entry_routes_through_legacy_bash_adapter` in the §Source Evidence cell of BC-1.07.005, but the SAME fabricated symbol remained in the H1 title of BC-1.07.005 AND in BC-INDEX row 226 AND in VP-043 §Source Contract. Three out of four cite sites stayed fabricated.
>
> The fix-burst's grep query was scoped to §Source Evidence patterns; it missed H1 titles, BC-INDEX rows, and cross-spec cites.
>
> Rule: when a fix-burst patches ONE cite site of a fabricated symbol, the same burst MUST run a corpus-wide grep for ALL OTHER cite sites of that same fabricated symbol and patch them uniformly. Pattern:
>
> ```
> grep -rn '<old-fabricated-symbol>' .factory/specs/ .factory/stories/
> ```
>
> All matches in active body (H1 / table cells / cross-spec cites) must be patched in the same commit. Changelog/Amendment historical mentions are exempt (append-only record).
>
> This rule extends L-P21-001 ("every cited symbol must grep-verify") with the corollary: when fixing a fabricated cite, the fix MUST be applied to every cite site, not just the one that triggered the discovery.
>
> Suggested codification mechanism: future POLICY 13 / S-15.03 hook scope.

[codified] — fix-burst-22 sub-burst 3.

> **Codified pre-L-P26-001 — exempt from retroactive verification block.** Predates L-P26-001 mandate (fix-burst-25). Historical application: all-cite-sites discipline applied retroactively to BC-1.07.005/006 + VP-043 in fix-burst-22 sub-burst 2 (60072605), and comprehensive corpus audit in fix-burst-23 sub-burst 1 (3576f1a6) per L-P24-002.

---

### L-P23-002 [codified]: Lobster-line-cite class is multi-variant; the corpus-wide sweep grep MUST cover ALL syntactic variants

**Source:** F-P23-001 + F-P23-002 (process-gap)

> Failure mode (F-P23-001): fix-burst-21 sub-burst 1 swept 88 ss-05 BCs with the description-form `Step \`X\` (line N). ... Source M-K.` pattern. 27 sibling ss-05 BCs used the postcondition-form `Step \`\`<step>\` (line N) — ...` (double-backtick wrapping) and were missed. Fix-burst-22 sub-burst 1 swept 27+ additional with the broader pattern.
>
> Failure mode (F-P23-002): fix-burst-21 was scoped to `ss-05/`. BC-6.04.027 (ss-06) had `red-flag table (line 27)` in active §Description and §Invariants — same drift class, different subsystem. Fix-burst-22 sub-burst 1 extended scope cross-subsystem.
>
> Currently-known syntactic variants of the lobster-line-cite class:
> (a) `lines N-K` range form (swept fix-burst-19/20)
> (b) `Step \`X\` (line N). ... Source N-K.` description form, single-backtick (swept fix-burst-21 sub-burst 1)
> (c) `Step \`\`<step>\` (line N) — ... \`` postcondition form, double-backtick (swept fix-burst-22 sub-burst 1)
> (d) `<table-name> (line N)` cross-subsystem reference (swept fix-burst-22 sub-burst 1)
>
> Rule: when sweeping a class, the grep MUST include ALL known syntactic variants AND extend to ALL subsystem directories. The sweep must be CORPUS-WIDE, not directory-scoped.
>
> Suggested codification: S-15.03 mechanical hook would parse line-cite intent semantically rather than relying on syntactic patterns.

[codified] — fix-burst-22 sub-burst 3.

> **Codified pre-L-P26-001 — exempt from retroactive verification block.** Predates L-P26-001 mandate (fix-burst-25). Historical application: corpus-wide sweep with ALL syntactic variants applied in fix-burst-22 sub-burst 1 (9ebd5c31), covering double-backtick postcondition forms + cross-subsystem scope.

---

## F5 pass-24 process-gap findings (2026-05-09)

### L-P24-001 [codified]: Phase 0 brownfield-ingestion mapping artifacts are exempt from L-P21-001 / L-P23-001 fabricated-symbol sweep

**Source:** F-P24-005 — bc-id-mapping.md carve-out adjudication (fix-burst-23 sub-burst 3).

Files with frontmatter `producer: codebase-analyzer` AND `phase: 1.x` (e.g., `bc-id-mapping.md`, `pass-3-deep-*.md`, `pass-4-domain-*.md`) preserve the original Phase 0 ingestion content as historical audit records. The "description" / "extracted_text" / "raw_content" columns of these tables MUST NOT be patched even when they contain fabricated symbols, because:

1. The fabricated symbols ARE the original ingestion output — patching loses the audit-trail evidence of what was extracted vs. what was real.
2. The MAPPING column (e.g., "Maps to BC-1.07.005") is the integration point; the description column is just the raw ingestion text.
3. Per the brownfield ingestion methodology (`vsdd-factory:codebase-analyzer` agent), these tables are append-only historical records.

**Carve-out scope:**
- `.factory/specs/behavioral-contracts/bc-id-mapping.md` (description column)
- `.factory/phase-0-ingestion/pass-*-deep-*.md` (extracted_text columns)
- Any other artifact with `producer: codebase-analyzer` frontmatter

**Not carved out:** Active-body content in OTHER fields (e.g., the actual BC IDs, the Status, the Subsystem assignment) remains subject to L-P21-001 / L-P23-001 discipline.

**Trigger (fix-burst-23 sub-burst 1):** corpus-wide grep for every historical fabricated symbol found `every_entry_*` matches in `bc-id-mapping.md:349-350`. Sub-burst 1 reported these as inside the historical Phase 0 audit mapping table. Sub-burst 3 adjudicates as CARVE-OUT per this lesson.

[codified] — fix-burst-23 sub-burst 3.

> **Codified pre-L-P26-001 — exempt from retroactive verification block.** Predates L-P26-001 mandate (fix-burst-25). Historical application: carve-out confirmed for bc-id-mapping.md in fix-burst-23 sub-burst 3; carve-out extended to F1-delta-analysis.md and F1-platform-amendment-delta-analysis.md in fix-burst-25 sub-burst 2 (a2c390cd) per L-P25-002.

---

### L-P24-002 [codified]: When a fix-burst codifies a lesson, the SAME burst MUST run a comprehensive corpus-wide grep covering ALL historical instances of the codified pattern class — not just the cluster that triggered the lesson

**Source:** F-P24-002 + F-P24-003 + F-P24-004 — comprehensive corpus audit (fix-burst-23 sub-burst 1).

**Failure mode:** L-P23-001 (all-cite-sites discipline) was codified at fix-burst-22 sub-burst 3 to address the BC-1.07.005/006 cluster. The codifying burst's retroactive sweep covered ONLY the BC-1.07.005/006 + VP-043 cluster (the cluster that triggered the lesson). It did NOT corpus-sweep for previously-fixed fabricated symbols (BC-1.14.001 cluster: `RegistryEntry.async`, `run_tiers`, `spawn_detached`).

Pass-24 then found those previously-fixed fabricated symbols surviving at sibling cite sites (BC-7.06.001, S-15.01, ADR-019, E-15) — exactly the failure mode L-P23-001 was meant to prevent.

**Rule:** when codifying a lesson, the retroactive sweep MUST cover not just the symbol/pattern that triggered the lesson but the COMPLETE HISTORICAL CATALOG of every previously-fixed instance of the same class. Concretely, every fix-burst that codifies a lesson must:

1. Maintain a "historical catalog" of the class members fixed in prior bursts (e.g., the fabricated-symbol set, the prose-form line-cite variants, the index-cite-refresh patterns).
2. Run corpus-wide grep for EACH catalog entry in the same burst as the codification.
3. Patch all active-body matches uniformly.
4. Update the catalog with the codifying instance.

Without this discipline, codification reduces to single-cluster fixes and the recurrence pattern continues at NEW layers (validated empirically across passes P18-P24, 7 consecutive HIGH).

**Suggested codification:** S-15.03 mechanical hook would maintain the historical catalog and enforce the grep at write time.

**Verified retroactively in fix-burst-23 sub-burst 1 (3576f1a6):** comprehensive corpus audit — 11 active-body cite sites of fabricated symbols patched across 4 files:
- ADR-019 v1.9→v1.10 (4 sites: line 107 prose, 116 + 121 pseudocode, 286-287 subsystem assignments)
- BC-7.06.001 v1.10→v1.11 (1 site: line 130 Architecture Anchors)
- S-15.01 v1.20→v1.21 (2 sites: line 494 + 762)
- E-15 v1.1→v1.2 (2 sites: line 241 + 242 — discovered as NEW sibling instance)
- Post-sweep grep: 0 active-body matches for all 10 fabricated symbols

[codified] — fix-burst-23 sub-burst 3.

> **Codified pre-L-P26-001 — exempt from retroactive verification block for the verification block itself.** Verification evidence above pre-dates the formal L-P26-001 block format but constitutes substantive equivalent evidence of retroactive application.

---

## F5 pass-25 process-gap findings (2026-05-09)

### L-P25-001 [codified]: When a story moves to Merged, the story body must be retrofitted (or annotated as superseded-by-implementation) to reflect actual merged code

**Source:** O-P25-002 — S-15.01 post-merge body drift.

**Failure mode (O-P25-002):** S-15.01 was MERGED via PR #106 but frontmatter still says `status: ready` and §Implementation Modules + §Tasks describe pre-merge pseudocode (T-1c modifies `engine.rs::run_event`, T-3d wires `drain_async_tasks()`, etc.) that does NOT match merged code. Every adversary pass after merge finds "fabricated symbols" that are actually frozen pre-merge planning vocabulary.

**Rule:** when a story moves to Merged status, state-manager (or a post-merge story-writer agent) MUST either:
(a) Rewrite §Implementation Modules + §Tasks to reflect the merged code shape, OR
(b) Add a §Post-Merge Status block at the top of these sections noting "POST-MERGE STATE: this section's task descriptions reflect pre-merge planning vocabulary frozen at PR #N merge. Actual merged implementation is at HEAD; symbol references below describe planning intent rather than current code."

Without this discipline, story body sections become a recurrence carrier for sibling-class fabricated-symbol findings. Pass-25 demonstrated 10 active-body fabricated-symbol cite sites surviving in S-15.01 §Implementation Modules + §Tasks tables.

**Suggested codification mechanism:** post-merge hook in S-15.03 scope (e.g., `validate-post-merge-story-retrofit`) that blocks status: merged transitions until the §Post-Merge Status block is added.

[codified] — fix-burst-24 sub-burst 2.

**Verified retroactively in fix-burst-26 sub-burst 2:** 56 merged stories with planning-vocabulary §Tasks (containing `- [ ]` or `1. [ ]` checkboxes) received POST-MERGE-STATE annotation per option (b). Stories S-0.01 through S-8.30 annotated, spanning pre-GitHub-PR merges (2026-04-24) through PR-58 (2026-05-02). F-P27-003 closure.

---

### L-P25-002 [codified]: Phase F1 architect-proposal artifacts (status: draft, producer: architect, phase: F1 OR phase: F1-amendment) are exempt from L-P21-001/L-P23-001 fabricated-symbol sweeps under the same logic as L-P24-001 brownfield Phase 0 carve-out

**Source:** F-P25-007 — F1-delta-analysis.md carve-out adjudication.

**Failure mode (F-P25-007):** F1-delta-analysis.md preserves pre-implementation pseudocode (`run_tiers`, `spawn_detached`, `run_event`, `drain_async_tasks`, etc.) as the canonical Phase F1 architect proposal. Patching these to merged-code symbols would lose the audit-trail evidence of what was originally proposed at design-time vs. what was actually built.

**Rule:** artifacts with frontmatter matching `producer: architect` AND (`phase: F1` OR `phase: F1-amendment`) AND (`status: draft` OR `author: architect`) are CARVED OUT from L-P21-001 / L-P23-001 fabricated-symbol sweeps. The pseudocode within them is "expected fabrication" — downstream implementation specs MUST replace these with merged-code symbols, but the F1/F1-amendment source remains as historical proposal. Note: the `author:` field is an acceptable alternative to `producer:` for architect-authored F1 artifacts; both forms are tolerated under this carve-out.

**Suggested codification:** extend L-P24-001 carve-out scope from `producer: codebase-analyzer AND phase: 1.x` (brownfield Phase 0) to ALSO include `producer: architect AND (phase: F1 OR phase: F1-amendment) AND status: draft` (greenfield Phase F1 + amendment proposals).

Fix-burst-24 sub-burst 1 added an inline HTML comment to F1-delta-analysis.md noting the carve-out per L-P25-002. Fix-burst-25 sub-burst 2 (a2c390cd) extended the carve-out to F1-platform-amendment-delta-analysis.md per F-P26-003.

[codified] — fix-burst-24 sub-burst 2. Updated fix-burst-26 sub-burst 2: predicate expanded to `phase: F1 OR phase: F1-amendment`; `author: architect` field tolerance added. F-P27-004 closure.

**Verified retroactively in fix-burst-25 sub-burst 2 (a2c390cd):** F1-delta-analysis.md and F1-platform-amendment-delta-analysis.md received carve-out HTML comments. F-P26-003 closure.

---

## F5 pass-26 process-gap findings (2026-05-09)

### L-P26-001 [codified]: Every codifying burst MUST run L-P24-002 corpus-sweep on the new lesson at codification time, with explicit verification block in lessons.md

**Source:** F-P26-002, F-P26-003, F-P26-006 (process-gap)

**Failure mode (F-P26-002, F-P26-003, F-P26-006):** Fix-burst-24 sub-burst 2 codified L-P25-001 + L-P25-002 but applied each lesson narrowly to ONE trigger artifact (S-15.01 for L-P25-001; F1-delta-analysis for L-P25-002). Six+ merged stories (F-P26-002) and 2 sibling F1 architect proposals (F-P26-003) were skipped.

**Rule:** when a fix-burst codifies a NEW lesson L-NNN, the SAME burst MUST:
1. Read the lesson's codification text to extract the corpus-sweep query (the predicate that defines the lesson's pattern class).
2. Run the query corpus-wide.
3. Apply the lesson's prescribed action to ALL matches.
4. Append a `**Verified retroactively in fix-burst-N:**` block to the lesson body listing all artifacts patched.

This rule is the SAME-burst sibling of L-P24-002 but stated at the codification-event level rather than at the trigger-cluster level. Without this discipline, codifying bursts produce trigger-only fixes and the recurrence pattern continues at NEW layers.

**Suggested codification mechanism:** state-manager codifying-burst protocol template MUST include "L-P26-001 verification block" as a required step before commit. S-15.03's `validate-lesson-retroactive-sweep` hook would enforce this mechanically.

[codified] — fix-burst-25 sub-burst 3.

**Verified retroactively in fix-burst-26 sub-burst 2:** this very lesson now has its required verification block — meta-self-application closing F-P27-002. Verification blocks also added to L-P25-001 (56 stories annotated), L-P25-002 (F1 carve-out scope expanded), L-P26-002 (migration clause added). Older lessons L-P18-001..L-P24-002 marked "codified pre-L-P26-001 — exempt from retroactive verification block" with historical application notes. F-P27-002 closure.

---

### L-P26-002 [codified]: state-manager PR-merge handler MUST update story frontmatter `status: merged` + `merged_at:` + `merged_in:` + `merge_sha:` as a required step

**Source:** F-P26-004, F-P26-005, O-P26-006 (process-gap)

**Failure mode (F-P26-004, F-P26-005, O-P26-006):** When a PR merges, STATE.md "Merged" bucket is updated but story frontmatter `status:` is left at `draft` or `ready`. Cross-cycle pattern across 5+ stories (S-15.01, S-13.01, S-12.01, S-12.02, S-12.06).

**Rule:** state-manager's PR-merge handler MUST update the story's frontmatter atomically with the STATE.md update:
- `status: merged`
- `merged_at: YYYY-MM-DD`
- `merged_in: PR-NNN`
- `merge_sha: <full-or-short-SHA>`

**Migration clause (added fix-burst-26 sub-burst 2, F-P27-006):** legacy `pr: NN` field found in older story frontmatter MUST be migrated to `merged_in: PR-NN`. Any story carrying a bare `pr:` field is non-conformant with the 4-field schema. Existing stories with `pr:` are governed by the F-P27-001 retroactive sweep executed at fix-burst-26 sub-burst 1 (4c26e809), which migrated 18 stories from legacy `pr: NN` format and backfilled `merged_at` / `merge_sha` for 38 additional stories. New PRs MUST NOT use the `pr:` field.

**Tag:** `[process-gap]` — applies to state-manager agent prompt.

**Suggested enforcement:** lint-hook similar to validate-stable-anchors that fails CI when STATE.md lists a story as merged but frontmatter says draft/ready. (Future POLICY 13 candidate.)

**Pre-GitHub-PR sentinel (added fix-burst-27 sub-burst 2, F-P28-002):** For stories merged before GitHub PR tracking began (pre-PR-6 era; approximately 21 historic stories from S-0.x/S-1.x/S-2.x series), the canonical placeholder is `merged_in: none` and `merge_sha:` MUST hold the actual squash/merge commit SHA. Lint enforcement MUST treat `none` as a valid sentinel value for `merged_in:` and fall back to `merge_sha:` presence as the truth condition for merge-status verification. New PRs MUST NOT use `merged_in: none` — this sentinel is exclusively for pre-PR-6 historic stories.

[codified] — fix-burst-25 sub-burst 3. Updated fix-burst-26 sub-burst 2: migration clause added for legacy `pr:` field. F-P27-006 closure. Updated fix-burst-27 sub-burst 2: `merged_in: none` sentinel added for pre-GitHub-PR historic stories. F-P28-002 closure. Updated fix-burst-28: verification block added per L-P26-001 mandate. F-P29-003 closure.

**Verified retroactively in fix-burst-28 (this burst):** 21 historic stories carry `merged_in: none` per the sentinel rule:
- S-0.01, S-0.02, S-0.03, S-0.04, S-0.05
- S-1.01, S-1.02, S-1.03, S-1.04, S-1.05, S-1.06, S-1.07, S-1.08, S-1.09
- S-2.01, S-2.02, S-2.03, S-2.04, S-2.06, S-2.07, S-2.08

Each has `merge_sha:` populated with the actual commit SHA. The sentinel rule is correctly applied across the historic-merged-story corpus (`grep -rln '^merged_in: none' .factory/stories/` returns exactly 21 files). F-P29-003 closure.

**Verified retroactively in fix-burst-26 sub-burst 1 (4c26e809):** 56 historic merged stories' frontmatter retrofitted to 4-field schema (status: merged + merged_at + merged_in + merge_sha); 18 migrated from legacy `pr: NN`; 38 backfilled missing metadata. F-P27-001 closure.

---

## F5 pass-28 process-gap findings (2026-05-09)

### L-P28-001 [codified]: When a fix-burst rewrites a frontmatter field value, the corpus-wide grep MUST include both index file AND every source-of-truth artifact carrying that field

**Source:** F-P28-001 (HIGH)

**Failure mode (F-P28-001):** F-P27-005 closure ("VP-070/VP-071 proof_method `kani` → `kani-proof`") was applied to VP-INDEX rows only. VP-070.md and VP-071.md source frontmatter still carried `proof_method: kani`. The codifying burst's grep was scoped to the index file, missing the source-of-truth files.

**Rule:** when a fix-burst rewrites a frontmatter field value (e.g., `proof_method`, `subsystem`, `capability`, `status`, `priority`), the corpus-wide grep MUST include:
1. The authoritative index file row (BC-INDEX, VP-INDEX, STORY-INDEX, ARCH-INDEX)
2. Every source-of-truth artifact whose frontmatter carries that field

**Validation:** post-fix `grep -rn '^<field>: <old-value>' .factory/specs/<artifact-class>/` MUST return 0 active matches.

**Sweep query:** for `proof_method` rewrites, source-of-truth = `.factory/specs/verification-properties/VP-*.md`. For `subsystem`, source-of-truth = `.factory/specs/behavioral-contracts/ss-*/BC-*.md`. For `status`, source-of-truth = `.factory/stories/S-*.md`.

**Suggested codification:** extend L-P26-001 corpus-sweep mandate to explicitly enumerate "field-value rewrites" as a class. S-15.03's `validate-index-source-coherence` hook would enforce mechanically.

[codified] — fix-burst-27 sub-burst 2.

**Verified retroactively in fix-burst-27 sub-burst 1 (bc7ae728) and sub-burst 2:**
- VP-070.md:17 + VP-071.md:17 patched in fix-burst-27 sub-burst 1 (commit `bc7ae728`). VP-INDEX rows already correct.
- Audit of other field-value drift classes — none found in spot-check of `subsystem:`, `status:`, `priority:` fields across BC/VP/Story corpus.
