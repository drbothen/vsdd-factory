---
document_type: lessons
cycle: v1.0-feature-plugin-async-semantics-pass-1
producer: state-manager
version: "1.0"
last_updated: 2026-05-08
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

---

## L-P20-001 [codified]: Literal-vs-class grep discipline in retroactive-sweep

**Source:** pass-20 review / fix-burst-19 sub-burst 2 — F-P20-001 extended prose sweep gap.

When applying L-P19-001 (corpus-wide retroactive sweep for a codified rule), the grep MUST use the SEMANTIC pattern CLASS, not the LITERAL string that triggered the rule.

**Failure mode:** codifying lesson is triggered by a specific syntactic instance (e.g., L-P18-002 caught `at line 152`, singular form). Subsequent retroactive sweep uses the literal grep `at line [0-9]+` and misses sibling pattern instances (`at lines 148-224`, `between lines 575-731`, plural/range forms).

**Rule:** when codifying a lesson, the lesson author MUST document the PATTERN CLASS (e.g., "any prose-form line citation including singular/plural/range forms") in addition to the LITERAL grep. State-managers applying L-P19-001 MUST use the broader class grep.

**Example refined grep for prose-form line citations:**
`\bat lines? [0-9]+(-[0-9]+)?\b|\bbetween lines? [0-9]+ and [0-9]+\b|\b(lines?|line) [0-9]+(-[0-9]+)?\b`

[codified] — fix-burst-19 sub-burst 2.

---

## L-P20-002 [codified]: Index-of-indexes cite-refresh discipline (parent-pointer staleness)

**Source:** pass-20 review / fix-burst-19 sub-burst 2 — F-P20-002 ARCH-INDEX cite stale 10 versions.

ARCH-INDEX body cites the BC-INDEX/VP-INDEX/STORY-INDEX versions explicitly (e.g., "Total BCs: 1947 (per BC-INDEX v1.33)"). When a child index is version-bumped, the parent index cite MUST be refreshed in the same burst.

ARCH-INDEX self-codified this rule at v1.18 (2026-05-07) and reinforced at v1.19. The rule was systematically ignored across 15 consecutive fix-bursts (4 through 18); pass-20 found ARCH-INDEX cite was 10 versions stale (BC-INDEX v1.33 vs current v1.43).

Three consecutive HIGH passes (P18-001 sibling-hook bug, P19-001 codified-not-applied, P20-002 cite-stale-15-fix-bursts) demonstrate that prose codification of cite-refresh discipline is structurally insufficient.

**Recommended enforcement:** hook-based parser that compares ARCH-INDEX body cites to current BC-INDEX/VP-INDEX/STORY-INDEX frontmatter versions; blocks Edit/Write to ARCH-INDEX or any of the three child indexes if cites are stale by >0 versions. Tracked in follow-up story S-15.03 (see below).

[codified] — fix-burst-19 sub-burst 2.

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
