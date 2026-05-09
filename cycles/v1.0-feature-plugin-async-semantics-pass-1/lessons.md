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
