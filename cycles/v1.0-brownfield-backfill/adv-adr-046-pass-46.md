# ADR-046 Adversarial Spec-Convergence Review — Pass 46

**Reviewed artifact set (frozen):** ADR-046 v1.17 + BC-4.17.001 v1.20 + BC-5.40.001 v1.18 + BC-7.07.001 v1.34
**Review date:** 2026-08-27
**Verdict:** FINDINGS (2 MED)
**Gate:** BC-5.39.001 3-CLEAN spec-convergence (ADR-046 fix-state-writes)
**Streak effect:** 1/3 → **RESETS to 0/3** (5th reset this session)
**D-chain:** D-1103

## Part A — Finding Set (frozen set: ADR-046 v1.17 + BC-4.17.001 v1.20 + BC-5.40.001 v1.18 + BC-7.07.001 v1.34)

**HIGH (0):** none this pass.

**MEDIUM (2):**

- **F-P46-001 (MED, POLICY 4, byte-range/body-confinement arm-scope reconciliation).**
  BC-4.17.001 Invariant 5's headline — "the hook's body is never read or parsed" — was an
  un-caveated byte-range claim that contradicted the arm-scope reconciliation the pass-39/
  pass-40 (F-P39-001/F-P40-001) fixes already applied to sibling loci (Precondition 4,
  Invariant 7, VP-TBD-8, PC4, Invariant 9): the hook's single `host::read_file` call reads the
  WHOLE file (body included), and PC2's `Renewed` outcome's single composed `host::write_file`
  rewrites the WHOLE file back — "operates only within the frontmatter region" is, for the
  `expires_at` arm, a semantic-region guarantee delegated to `renew_lock_if_holder`/
  `flp::parse_factory_lock`/`rewrite_expires_at`, not a byte-range restriction the hook itself
  enforces by construction. Invariant 5's own "Mirrors BC-4.13.001 Invariant 9" citation
  compounded the defect: it imported BC-4.13.001's byte-range `host::read_prefix`
  (read-only, envelope-bounded) semantics wholesale, without the reader-vs-writer caveat this
  hook's `host::read_file` (whole-file, write-capable) call site requires — BC-4.13.001's guard
  never writes; this hook's PC2 arm does. The pass-40 sweep had explicitly listed Invariant 5 as
  "checked" and "confirmed correct, arm-split since v1.18, not re-broken," but that
  self-attestation recorded only that the locus was inspected, not why it was judged correct —
  it was still carrying the pre-F-P39-001 framing underneath a "checked" label.
  **Mandatory exhaustive byte-range/body-confinement locus audit performed** (in-scope, this
  pass): every Precondition/Postcondition/Invariant/VP/Architecture-Anchor/Edge-Case/
  Description/§SDK-Grounding-Evidence locus making a byte-range/body-confinement/
  "frontmatter-only"/"never read-or-inspect body"/"operate within delimiters"/`read_prefix`-
  vs-`read_file` claim was enumerated and verdicted: Precondition 4, Invariant 7 (including its
  fence-not-located tail), PC4, and VP-TBD-8 all re-confirmed CORRECT and unmodified (arm-split
  intact since v1.18/v1.19, not re-broken); Description's "frontmatter-only transforms" phrase,
  PC1's rewrite-mechanism paragraph, PC2's "Required input" paragraph, PC3a, Invariant 9, Edge
  Cases, Canonical Test Vectors, VP-TBD-7/VP-TBD-9, Architecture Anchors, and §SDK Grounding
  Evidence all confirmed either semantic-region-only claims true under both arms, or unrelated
  to byte-range/body-confinement entirely (path-capability scope, TTL sourcing, soft-warn
  thresholds). **Invariant 5 was the ONLY locus still carrying the un-caveated byte-range
  framing** — the class is now fully drained. Fixed same-burst by product-owner: Invariant 5
  restated with the same arm-split as its siblings — (a) PC1's `timestamp:` scan is
  byte-range-confined to the `extract_frontmatter` slice; (b) PC2's `expires_at` arm is fed the
  FULL content, its frontmatter confinement a semantic-region guarantee, persisted via a
  whole-file write that preserves the body byte-for-byte outside the frontmatter region;
  headline claim corrected from "body never read" to "body never PARSED, inspected, or depended
  on"; the Mirrors-BC-4.13.001-Invariant-9 citation corrected to note the mirror is on
  semantic-region intent, not byte-range read mechanism. No PC/Invariant/EC renumbered
  (append-only numbering preserved — POLICY 1).

- **F-P46-002 (MED, POLICY 4, cross-reference integrity).** ADR-046 cited "BC-5.40.001
  Invariant 2/AC-007" in TWO live-body loci — §Rationale's "Why the TTL must be sourced, not
  reinvented" and §Source/Origin's "Behavioral contracts" bullet — as though AC-007 were a
  normative acceptance-criterion section of BC-5.40.001 itself. A full `## `/`### `
  section-heading sweep of BC-5.40.001 (Description/Preconditions/Postconditions/Invariants/
  Edge Cases/Canonical Test Vectors/Related BCs/Architecture Anchors/Story Anchor/VP
  Anchors/Verification Properties/Traceability/Changelog) confirms BC-5.40.001 has no
  Acceptance Criteria section and no AC-NNN numbering scheme at all — the same fact this ADR's
  own pass-43 remediation (F-P43-002) already established for BC-7.07.001/AC-018, now recurring
  on a sibling artifact this ADR also cites. AC-007 is in fact a STORY-level acceptance
  criterion of `.factory/stories/S-17.01-factory-lock-schema-cas-push.md`
  (`### AC-007 (traces to BC-5.40.001 invariant 2 — default TTL is 45 minutes)`). Additionally,
  §Rationale's parenthetical "MUST NOT be overridden via environment or arguments" was
  presented as a verbatim quote but is not verbatim-present anywhere in this repository — not
  in BC-5.40.001 Invariant 2 (which reads "The TTL value is not configurable by users") and not
  in S-17.01's own AC-007 text ("This constant MUST NOT be configurable by users") — confirmed
  by a repo-wide `grep -rn` sweep of `.factory/` returning only this ADR's own two loci as
  hits; the phrase was fabricated at some point in this ADR's own drafting/revision history, not
  inherited from either source artifact. **Mandatory exhaustive AC-reference audit performed**
  (grep-complete sweep of every `AC-[0-9]+` token across the document body): found exactly two
  other AC-NNN loci in the live body, both AC-018 (Companion Amendment 3's closing sentence and
  the v1.17 Changelog row), both already correctly attributed to S-18.04a by the pass-43 fix and
  requiring no further change. Fixed same-burst by architect: both AC-007 loci re-expressed as
  S-17.01's AC-007, anchored to BC-5.40.001 Invariant 2 rather than implied as the BC's own
  AC-NNN — §Rationale now cites S-17.01's AC-007 alongside a verbatim quote from BC-5.40.001
  Invariant 2 ("The TTL value is not configurable by users.") in place of the fabricated
  quote; §Source/Origin now reads "Invariant 2 (`TTL_SECONDS = 2700` non-configurable; also
  S-17.01's AC-007)" — same underlying meaning preserved (TTL is sourced from one
  non-configurable constant) at both loci, now anchored at the artifact where AC-007 actually
  exists. No Decision content, File-Change Plan, or other Companion Amendment item touched. No
  new Decision added; Decision numbering (1–6) unchanged; Status remains **accepted**.

**LOW (0):** none this pass — no observations of any kind, blocking or non-blocking.

**Zero HIGH findings. 2 MEDIUM findings (both fixed same-burst). VERDICT: FINDINGS. Streak
RESETS 1/3 → 0/3 per BC-5.39.001's literal-3-CLEAN discipline — any BLOCKING finding resets the
streak regardless of severity class.**

## Part B — Verified-Clean Observations (adversary-confirmed, no blocking findings)

Fresh-context adversary review, information-asymmetric per the Iron Law (no visibility into
prior-pass narrative, decision-log, lessons.md, or burst-log — reviewed the frozen artifact set
on its own merits only). Every dimension this gate's 45-pass history has previously found a
defect in was independently re-checked against the current frozen set and confirmed holding,
with zero regression beyond the two findings above:

- **Illustrative-quote verbatim-source-accuracy + sibling-parity check (D-1101, ninth
  discipline):** all three companion BCs' illustrative CAP-031/CAP-032 quotes re-derived and
  confirmed verbatim-correct against `capabilities.md` — no regression, SECOND consecutive
  confirmation (following pass-45).
- **Arm-parity what-vs-how reconciliation + locus-class-extension (D-1096/D-1097 classes,
  sixth+seventh disciplines):** re-derived across every OTHER `extract_frontmatter`-guarantee
  locus (all confirmed correctly arm-split); this pass's own F-P46-001 finding is itself the
  discovery that Invariant 5 had NOT been fully swept into this class at pass-40 — the
  discipline's re-application is what surfaced it, not a failure of the discipline itself.
- **AC-owning-artifact cross-reference discipline (D-1100, eighth discipline extension):**
  re-derived against every AC-NNN citation across the frozen set; this pass's own F-P46-002
  finding is the discovery that the discipline had not yet been swept to BC-5.40.001/AC-007
  (only BC-7.07.001/AC-018 had been checked at pass-43) — again, the discipline surfaced its
  own remaining gap, not a failure of the discipline.
- **ADR §Decision/§N.M anchor correctness (D-1092/F-P35-001 class):** every citation
  independently re-derived from the cited ADR's own section content — all CORRECT.
- **ADR-046 own-Decision-list enumeration/count claims (D-1094/F-P37-001 class):** recounted —
  confirmed 6 numbered decisions, correctly cited "1–6" throughout.
- **Grep-complete cluster-wide `inputs:` completeness (D-1090/D-1100):** re-audited all four
  artifacts' own `inputs:` arrays — zero omissions found.
- **4-leg `modified:`-array head==version parity (D-1089):** all four artifacts' `modified:`
  array heads confirmed matching their own frontmatter `version:` field, prior to this pass's
  edits.
- **Behavioral core (write-composition table, five-outcome table, identity-gating logic,
  event-sourcing struct-variant text):** re-verified clean across all four artifacts — stable
  since pass-27 (20 consecutive passes now, counting this one). **This pass's two findings are
  confined entirely to the provenance/cross-reference and citation-accuracy perimeter — neither
  touches the behavioral core.**
- **Every load-bearing code claim (function names, file paths, constant names) independently
  re-verified against the actual source files:** all accurate.
- **`inputs:` cyclic-hash tangle ([D-1082], 4-artifact):** inspected for correctness (not
  re-chased) prior to this burst's edits.

**Novelty assessment:** both findings this pass are genuinely NEW instances of already-codified
discipline classes (arm-parity/locus-class-extension, sixth+seventh disciplines; AC-owning-
artifact cross-reference, eighth-discipline extension) — not a new tenth discipline. Both are
UNSWEPT SIBLINGS of prior fixes: F-P46-001 is the byte-range class's last un-caveated locus
(Invariant 5, missed by the pass-40 sweep's self-attested-but-unexplained "checked" verdict);
F-P46-002 is the AC-reference class's second instance (BC-5.40.001/AC-007, missed because the
pass-43 fix (F-P43-002) only swept BC-7.07.001/AC-018, the locus the pass-43 finding itself
named, not every AC-NNN reference cluster-wide). Both were closed this burst via exhaustive
class-draining audits (a full byte-range/body-confinement locus enumeration for F-P46-001; a
full `AC-[0-9]+` token-grep-complete sweep for F-P46-002) rather than single-locus spot-fixes —
the same technique that drained the `inputs:`-completeness class at D-1100/pass-43. Per
BC-5.39.001, any BLOCKING finding — regardless of severity or class — resets the streak. This is
the **5th reset this session** (after pass-35, pass-37, pass-39, pass-43), and — like pass-43 —
sits at the provenance/cross-reference/citation-accuracy perimeter rather than the behavioral
core (unlike the pass-39 data-destructive reset).

## Part C — State at Close of Review

ADR-046 **v1.17→v1.18** (F-P46-002 fix, architect). BC-4.17.001 **v1.20→v1.21** (F-P46-001 fix,
product-owner). BC-5.40.001 **v1.18 UNCHANGED** (no finding routed to it this pass). BC-7.07.001
**v1.34 UNCHANGED** (no finding routed to it this pass). BC-5.39.001 3-CLEAN streak: **1/3 →
RESETS to 0/3** (5th reset this session). Gate history to date: 46 passes run against
evolving/frozen sets; 36 genuine findings found and fixed, plus 4 audit-extra stragglers (pass-31,
pass-33) and 1 latent-bracket drain (pass-37, not counted as genuine), 1 ACCEPTED non-blocking
observation (O-P42-001, does not count against the streak), and 1 FIXED non-blocking observation
(O-P44-001, governance-elected fix at zero streak cost); 6 clean passes (34, 36, 38, 41, 42, 45).

**Index reconciliation (state-manager, this burst):** ARCH-INDEX v3.87→v3.88 (ADR-046 row
version cell + pass-44..46 narrative summary appended). BC-INDEX v5.10→v5.11 (BC-4.17.001 row
version-chain cell appended).

**NEXT: fresh pass-47** against the newly-frozen set (ADR-046 v1.18 + BC-4.17.001 v1.21 +
BC-5.40.001 v1.18 + BC-7.07.001 v1.34), starting a new streak at 0/3, applying all nine
now-codified convergence-technique disciplines proactively from the start — with the byte-range/
body-confinement class and the AC-reference class both now confirmed exhaustively drained by
this pass's own audits. The human decision this session remains to CONTINUE looping toward
literal 3-CLEAN (not accept D-386 Option C asymptotic acceptance). S-17.05 TDD implementation
remains gated on convergence.
