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
