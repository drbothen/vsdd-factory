---
document_type: red-gate-log
level: ops
version: "1.28"
status: verified
producer: test-writer
timestamp: 2026-07-25T03:15:00Z
phase: 3
inputs:
  - .factory/stories/S-21.04-story-worktree-write-path-discipline.md
  - .factory/specs/behavioral-contracts/ss-06/BC-6.26.001.md
input-hash: "35dc8a1"
traces_to: "BC-6.26.001 v1.14; BC-6.26.001 v1.15; story v1.20; story v1.21; story v1.22; story v1.23; story v1.25; story v1.26; story v1.27; story v1.28; story v1.29; story v1.30; story v1.33; BC-6.26.001 v1.18"
last_amended: "2026-07-31 pass-30 fix burst (F-S2104-P30-H08/H09/H05/M01/L01): Summary HEAD c7c61688→44547051 (27/27: 11/11+16/16); T-017 allocated to EC-009 stray-inode test (POLICY 1 — T-010 is pipeline probe per pass-24 SoT; POLICY 1 append-only; next available ID = T-017); T-010 label added to pipeline probe @test name; inventory comment updated T-010→T-017; T-016 renamed test_coupling_gate_story_gate_count_matches_bats_count_word→test_coupling_gate_bats_sentinel_matches_runtime_gate_count (F-S2104-P30-L01; post-ADR-034 v1.1 misnomer; swept 1 bats site + STRUCTURAL section); T-001 summary comment twenty-three→twenty-four (F-S2104-P30-H05); 6 phantom numeric IDs F-S2104-P28-016→F-S2104-P28-L01, F-S2104-P28-017→F-S2104-P28-L02 in worktree-identity-preflight.bats (F-S2104-P30-M01); Pass-28 + Pass-29 attestation sections appended; Pass-28 + Pass-29 pass table rows added; T-017 RG-010 section added; STRUCTURAL T-016 name updated; traces_to story v1.33 + BC-6.26.001 v1.18; input-hash bd23538; version 1.27→1.28. [Prior: 2026-07-29 D-940: pass-27 findings CLOSED (B01 positional conjuncts; H01 sibling-sweep; H02 count-word + T-016; M01/M02/M03 BC v1.15; M04 comment-strip); POLICY 15 restore-leg residual CLOSED via CONTROL-equivalence; Summary HEAD 7c3338e7→c7c61688 (26/26: 10/10+16/16); T-016 coupling gate added to Summary; Pass-27 table row added; traces_to story v1.30 + BC-6.26.001 v1.15; input-hash 105492a→6a61cd1 (story v1.30 + BC v1.15 drift); version 1.26→1.27. [Prior: 2026-07-28 D-939: H04 CLOSED (D-939 SUPERSESSION — records committed by a8ec290e in-flight; 7/8 per-guard records; P25-M06 NOT PRODUCIBLE; POLICY 15 restore-leg residual pass-27); 5 sites corrected; version 1.24→1.25. [Prior: 2026-07-28 D-938: adversary-pass-26.md persisted (B2/H4/M3/L2 = 11 findings; novelty HIGH; ADR-033 cross-family limitation; POLICY 22 both-directions record: concern-1 CONFIRMED→B01; concern-2 REFUTED with reasoning); 10 of 11 CLOSED at 7c3338e7 (H04 OPEN — per-guard mutant evidence requested twice, not produced; recorded as OPEN per production-grade discipline); Summary HEAD 4dc27251→7c3338e7 (25/25: 10/10+15/15); Pass-26 summary table row added; Pass-26 attestation section appended (orchestrator B01/B02 verbatim evidence); H04 OPEN evidence gap recorded; traces_to story v1.29 added; input-hash 101b1e6→hook-authoritative (story v1.29 + ADR-031 sentinel drift); version 1.23→1.24. [Prior: 2026-07-28 D-937: H01 POLICY 15 evidence persisted for P23-006/007/008/012 — Summary HEAD advanced 5ccf5869→4dc27251 (25/25: 10/10+15/15); Pass-25 attestation section appended with orchestrator B01 evidence (recorded mutant fires both production guard + corpus regression T-015); M04 SHA transposition 6 sites corrected (5ccf5669→5ccf5869; proof: grep-c=4 — all 4 remaining in fix-description prose only: last_amended, modified[], summary-table M04 cell, attestation note; zero in evidence-citation positions); M07 sentinels D-{TBD-pass-23-fix-burst}→D-936 + D-{TBD-pass-25-fix-burst}→D-937 in BC-6.26.001/ADR-031/story; Pass-25 summary table row added; traces_to BC-6.26.001 v1.13→v1.14 + story v1.28 added; version 1.22→1.23. [Prior: 2026-07-28 D-936: F-S2104-P23-009 CLOSED (F-22-NNN→F-S2104-P22-NNN namespace normalization; 5 sites replaced; input-hash f69d6b0→61af172 after namespace fix triggered validate-input-hash hook block); F-S2104-P23-010 CLOSED (12 stale narrative bats line pins→behavioral anchors; TD-VSDD-091 comprehensive sweep — adversary cited ≥5, state-manager found 12 total); F-S2104-P23-014 GENUINELY-CLOSED (documented: D-934 already added 14 genuine per-guard records); Pass-24 + Pass-23-closure summary table rows added; Summary HEAD 9b12aa00→5ccf5869 (24/24: 10/10+14/14, 2026-07-28); traces_to BC-6.26.001 v1.12→v1.13 + story v1.27 added; version 1.21→1.22. [Prior: 2026-07-28 D-935: Summary HEAD 888b5b73→9b12aa00 (24/24: 10/10+14/14, 2026-07-28; pipeline probe T-010 added at 9b12aa00; F-S2104-P24-005 POLICY 15 shortfall CLOSED — true prior state 1-2/14 RED commands / 0/14 restore commands (orchestrator attribution; second evidence-fidelity failure same class as F-S2104-P23-002)); adversary-pass-24.md persisted (B2/H2/M2/L0 = 6 findings; targeted re-review); ### Pass-24 assertion-site attestation appended (2 test-writer reversions + 4 orchestrator tests verbatim); version 1.20→1.21. [Prior: 2026-07-28 D-934: Summary HEAD 63eae07d→888b5b73 (23/23: 9/9+14/14, 2026-07-28; F-S2104-P22-004 RECURRENCE — per-pass closure checklist exercised again; one pass after D-933 closed this class); ### Pass-23 assertion-site attestation (888b5b73) appended (B2 CLOSED; 12 OPEN routed; P23-013 NON-FINDING recorded); version 1.19→1.20. [Prior: 2026-07-28 D-933: F-S2104-P22-004 Summary HEAD a4ec37d3→63eae07d (23/23: 9/9+14/14, 2026-07-28); three-pass omission (D-923/D-925/D-929 each omitted Summary advance) recorded in modified[]; Summary-HEAD per-pass closure checklist note added; F-S2104-P22-005 ### Pass-22 assertion-site attestation (63eae07d) appended with 14 verbatim mutant-verification records; F-S2104-P22-012 §G.1 L31-40 volatile pin replaced with behavioral anchor (the `[ -L ]` symlink-guard paragraph in §G.1); input-hash a03188b→2a798de (story v1.26 + BC v1.12 drift; hook-authoritative); version 1.18→1.19. [Prior: 2026-07-27 D-925: ### Pass-20 assertion-site attestation (a5068252) appended (M-P20-A PW-B=FIRES(RED) WD=FIRES(RED) post-fix; PW-B clause-scoped Leg A F-S2104-P20-001; referent predicate extended Leg B F-S2104-P20-002 to include artifact[[:space:]]+writes? class; 3-vector orchestrator-executed battery: M-P20-A+CONTROL-1+CONTROL-2 all RED at a5068252; suites 9/9+14/14; input-hash a03188b UNCHANGED (no story drift); version 1.17→1.18; F-S2104-P20-003 OPEN pass-21 lead item). Prior: 2026-07-27 D-923: F-S2104-P19-011 balanced-fence audit row: truncation-guard rationale corrected to well-formedness invariant (no fence-aware domain at HEAD per c89bef22; F-S2104-P19-011); F-S2104-P19-012 write-directive audit row: escape-discrimination control evidence added (CONTROL escape GREEN + CONTROL negative-twin RED at 657fce61); F-S2104-P19-010 NAME-SET EQUALITY TWO→THREE + 19→21 NUMBERED + story v1.23→v1.25 + boundary-completeness + scope-restriction added to printf list; ### Pass-19 assertion-site attestation (657fce61) appended (21-gate table; ALTERNATION-DIRECTION; ESCAPE-SCOPE-PARITY; CONTROL escape evidence; NAME-SET EQUALITY PASS 21 gates); traces_to adds story v1.25; input-hash f86871a→a03188b (story v1.25 drift; hook-authoritative); version 1.16→1.17. Prior: 2026-07-27 D-920: F-S2104-P18-006 M-P17-A battery row corrected (Gate PW-B only; Gate 5 false claim dropped at 2 sites); Gate 5 audit row corrected (M-P17-A dropped; true Gate-5 vectors: M-P15-A S1, M-P14-A, M-P14R-A, worktree-relative, M-P16-C2; SEQUENCE-SHADOWED DEFENSE-IN-DEPTH for story-worktree CWD noted); ### Pass-18 assertion-site attestation (a4ec37d3) appended (35/35 battery; 20-gate table; 15-obligation table; ALTERNATION-DIRECTION STATEMENTS; NAME-SET EQUALITY PASS 19 gates); F-S2104-P18-007 D-918 input-hash placeholders corrected ([see frontmatter]→4b26b3b; [updated by compute-input-hash]→4b26b3b); Summary HEAD c89bef22→a4ec37d3 (quintuple parity); traces_to adds story v1.23; input-hash 4b26b3b→f86871a (story v1.23 drift); version 1.15→1.16; prior: 2026-07-26 D-918: F-S2104-P17-005 corrections (a)-(e) in §Pass-16 attestation (abbreviation splitter 3 forms; anchor-uniqueness domain corrected at 2 sites; M-P16-B Gate cell corrected to Gate 1(a) + bounding count=1; absent-block guard domain to $prohibition_block; Gate 5 alternation corrected); ### Pass-17 assertion-site attestation (c89bef22) appended (9 new mutant vectors battery + 9 prior re-verified; 17-gate table + 2-RETIRED + extraction mechanisms; obligation-indexed 4-row AC-001(a)/(b)/(c) complete; NAME-SET EQUALITY PASS); Summary HEAD 9ab1aa32→c89bef22; traces_to adds story v1.22; input-hash e6c640a→4b26b3b; version 1.14→1.15; prior: 2026-07-26 D-916: F-S2104-P16-005 corrections in §Pass-15 attestation (i) closing completeness-claim corrected; (ii) M-P15-A relabeled as M-P15-A-simplified [Correction at v1.14 (D-916)]; ### Pass-16 assertion-site attestation (9ab1aa32) appended (TWO-TIER: 9 adversary-verbatim vectors TIER 1 + 11 test-writer instantiation vectors TIER 2 + gate-indexed 15-row + obligation-indexed 5-row audit tables); Summary HEAD 8b39277b→9ab1aa32; input-hash 3d12427→1baca60 (story v1.21 drift); traces_to adds story v1.21; version 1.13→1.14; prior: 2026-07-26 D-914: F-S2104-P15-001/002/004 — Summary HEAD cite 26b85d8c→8b39277b; ### Pass-15 assertion-site attestation (8b39277b) section appended (verbatim test-writer: sentence-scoped Gates 1/4/5 (joined_block+sed sentence-split) with per-gate same-AC audit table; Gate 6 two-part polarity; bare-pin sweep); input-hash c74e0f8→3d12427 (story v1.20 drift); traces_to adds story v1.20; version 1.12→1.13; prior: 2026-07-26 D-912: F-S2104-P14R-002 — Summary HEAD cite 09cfce81→26b85d8c (test-writer-executed: 9/9 + 14/14); ### Pass-14R assertion-site attestation (26b85d8c) section appended (verbatim test-writer: Gate 1 affirmative two-part + Gate 5 POLICY-13 alternation; M-P14R-A + M-P14-A + worktree-relative synonym vector all RED). F-S2104-P14R-009 — Fixture column T-001/T-002/T-003 corrected to dynamic $(mktemp -d) per bats setup(). F-S2104-P14R-010 — pass-13 mutant record exact text substituted verbatim (M-P14-A; recovered from bats 6f928350:1377-1380). input-hash 89efd7e→c74e0f8 (story v1.19 drift); traces_to adds story v1.19; version 1.11→1.12; prior: 2026-07-26 D-909: F-S2104-P13-002 — Summary HEAD cite 264f53b6→09cfce81 + attestation condensed (orchestrator-executed: 9/9 + 14/14, 2026-07-26); :288 count phrase replaced with COUNT-FREE pointer to per-pass attestation sections; ### Pass-13 assertion-site attestation (09cfce81) section appended (verbatim test-writer: polarity-aware prohibition gates, 3 mutants incl. inversion vector, gate groups count-free). F-S2104-P13-004 — §Bats Tests T-001 AC-003→AC-003; AC-001; AC-002; AC-007 (a)-(c); §Bats Tests T-002 AC-004→AC-004; AC-002; §Traces T-001 and T-002 multi-AC sync from story v1.17. input-hash 53500af→89efd7e (story v1.17 drift); version 1.10→1.11; prior: 2026-07-26 D-908: F-S2104-P12-002(a) §T-009 :287 group description corrected (four primary-path gates .md-qualified form + P11-003 attestation); F-S2104-P12-002(b) P11-003 mutant record appended to T-009 section; F-S2104-P12-002(c) Summary HEAD 2c8eff8b→264f53b6 + suite-level verification updated; F-S2104-P12-010 D-907 modified[] date 2026-07-25→2026-07-26 (monotonic correction, non-monotonic per D-906 dated 2026-07-26); input-hash d1c79e9→53500af (story v1.16 drift); version 1.9→1.10; prior: 2026-07-26 D-907 (date-corrected D-908): F-S2104-P11-005 §T-009 mutant evidence line :286 count clause replaced — 5 gate GROUPS / 9 assertion sites reconciled with 8 mutants; input-hash 455740d→d1c79e9 (story v1.15 drift); version 1.8→1.9; prior: 2026-07-26 D-906: F-S2104-P10-007 Summary line HEAD cite 9d896bf5→2c8eff8b + suite-level verification (orchestrator ran bats → 9/9 ok + 14/14 ok at 2c8eff8b, 2026-07-26); F-S2104-P10-008 §T-009 mutant evidence line :285 named list (5 named gates + 8 scratch mutants); F-S2104-P10-009 traces_to v1.10→v1.11 + BC-6.26.001 cite v1.10→v1.11 throughout + T-008/T-009 version pin parity restored; input-hash 389274b→455740d (story v1.14 + BC v1.11 drift); version 1.7→1.8; prior: 2026-07-25 D-905: F-S2104-P9-004 §T-008 BC trace corrected (PC2 + Invariant 2 caller-side dispatch gate, per story AC-007; replaces erroneous Invariant 5 caller-side propagation anchor introduced in D-904 fix wave); F-S2104-P9-007 §T-009 mutant evidence recorded (obligation-asserting gates mutant-proven at 2992b53d; class-completed at 3326e4dd; bare-alternation paper-gate confirmed); input-hash 43e6df2→389274b (story v1.13 drift); version 1.6→1.7 (F-S2104-P9-004, F-S2104-P9-007); prior: 2026-07-25 D-904: F-S2104-P8-001 §Bats Tests table T-001→RG-001†, T-002→RG-002†, T-003→RG-003† (table correction omitted at D-895, surviving 6 passes); † footnote updated to cite D-904; F-S2104-P8-007 T-007 mutant-proof recorded verbatim (qualified-path/verify/PASS-result/not-evident-run-yourself all NO MATCH on scratch reduction at 052620dc); F-S2104-P8-002 NEW T-008 addendum (AC-007/RG-008; six-surface §G.1 mandate; quote-tolerant mutant-proof 052620dc) + NEW T-009 addendum (AC-009/RG-009; awareness-clause; GREEN 4265c96c); Summary line updated (T-007/T-008/T-009 propagation gates; HEAD 9d896bf5; all T-IDs/RG rows confirmed); BC-6.26.001 cite v1.9→v1.10 (quintuple parity); input-hash 7abb656→43e6df2 (story v1.12 + BC v1.10 drift); traces_to v1.9→v1.10; version 1.5→1.6 (F-S2104-P8-001, F-S2104-P8-002, F-S2104-P8-007); prior: 2026-07-25 D-903 state-manager — F-S2104-P7-001/P7-005 attestation corrections verbatim-authored by orchestrator: §T-005 addendum heading/Test rewritten (regular-file-at-path → PC2b BLOCKED; RG-006; AC-002/EC-007); §T-006 addendum heading/trailing-slash mechanism corrected (POSIX find WITHOUT -H/-L empty-return → false PC2a; [ -L ] guard routes to PC2b; trailing-slash defense-in-depth); all 4 RG-004a occurrences replaced with RG-006; §Traces T-005/T-006 quintuple parity updated; NEW §T-007 addendum (AC-008/RG-007 devops-engineer executor-side preflight mandate; obligation-asserting at 052620dc); Summary line: 9 bats tests T-001..T-006 + 3 propagation-gate tests; BC-6.26.001 v1.8→v1.9 cites updated throughout; input-hash 4b75dba→7abb656 (story v1.11 + BC v1.9 drift); traces_to v1.8→v1.9; version 1.4→1.5 (F-S2104-P7-001, F-S2104-P7-005); prior: 2026-07-25 D-902 state-manager — T-005/T-006/RG-005 attestation addenda appended; Summary 9-test suite; Traces quintuple parity v1.5→v1.8; T-004 addendum BC cite v1.5→v1.8; RG-reconciliation note appended to D-895 erratum; input-hash 8cdfb33→a4b9ea5 (story v1.9 + BC v1.8 drift); traces_to v1.6→v1.8; version 1.3→1.4 (F-S2104-P6-004); prior: 2026-07-25 D-899 state-manager — T-004 test description corrected to verbatim §G.1 PC2c semantics (HALT + surface exit code/stderr; no PREFLIGHT BLOCKED message); input-hash 2b051ec→8cdfb33 (story v1.7 drift); traces_to v1.5→v1.6; version 1.2→1.3 (F-S2104-P4-006); prior: 2026-07-25 D-897 state-manager — fabricated PC2c implementation quote corrected to verbatim §G.1 text; Invariant TBD placeholder removed; input-hash 55904fb→2b051ec (story v1.6 drift); version 1.1→1.2 (F-S2104-P3-009, F-S2104-P3-010); prior: 2026-07-25 D-896 state-manager — T-004/RG-004 attestation addendum (F-S2104-P2-013) + quintuple parity v1.5 (F-S2104-P2-017); prior: 2026-07-25 D-895 state-manager — erratum F-S2104-P1-009 (RG-ID mapping + AC-002 attribution)]]]]]"
modified:
  - "2026-07-25 D-895: Erratum appended — RG-ID mapping corrected (RG-001/002/003), fabricated RG-004/005 documented, AC-002 attribution corrected (F-S2104-P1-009)"
  - "2026-07-25 D-896: T-004/RG-004 attestation addendum appended; frontmatter version 1.0→1.1, traces_to updated to v1.5, §Traces BC cites updated to v1.5 (F-S2104-P2-013, F-S2104-P2-017)"
  - "2026-07-25 D-897: Fabricated PC2c implementation quote corrected to verbatim §G.1 text; Invariant TBD placeholder removed; input-hash 55904fb→2b051ec (story v1.6 drift correction); version 1.1→1.2 (F-S2104-P3-009, F-S2104-P3-010)"
  - "2026-07-25 D-899: T-004 test description corrected to verbatim §G.1 PC2c semantics; input-hash 2b051ec→8cdfb33 (story v1.7 drift); traces_to v1.5→v1.6; version 1.2→1.3 (F-S2104-P4-006)"
  - "2026-07-25 D-902: T-005/T-006/RG-005 attestation addenda appended; Summary updated to 9-test suite; Traces quintuple parity v1.5→v1.8; T-004 addendum BC cite updated to v1.8; RG-reconciliation note appended to D-895 erratum; input-hash 8cdfb33→4b75dba (story v1.9 + BC v1.8 drift); traces_to v1.6→v1.8; version 1.3→1.4 (F-S2104-P6-004)"
  - "2026-07-25 D-903: F-S2104-P7-001/P7-005 attestation corrections verbatim-authored by orchestrator; §T-005 rewritten (regular-file-at-path → PC2b BLOCKED; RG-006; AC-002/EC-007); §T-006 trailing-slash mechanism corrected; all 4 RG-004a→RG-006; §Traces quintuple parity updated; NEW §T-007 addendum (AC-008/RG-007); Summary line updated; BC v1.8→v1.9 cites; input-hash 4b75dba→7abb656 (story v1.11 + BC v1.9 drift); traces_to v1.8→v1.9; version 1.4→1.5 (F-S2104-P7-001, F-S2104-P7-005)"
  - "2026-07-25 D-904: F-S2104-P8-001 §Bats Tests table RG corrected (T-001→RG-001†, T-002→RG-002†, T-003→RG-003†); † footnote updated to cite D-904; F-S2104-P8-007 T-007 mutant-proof recorded verbatim; NEW T-008 + T-009 addenda (F-S2104-P8-002); Summary HEAD 3c3788d7→9d896bf5; BC-6.26.001 cite v1.9→v1.10 (quintuple parity); input-hash 7abb656→43e6df2 (story v1.12 + BC v1.10 drift); traces_to v1.9→v1.10; version 1.5→1.6 (F-S2104-P8-001, F-S2104-P8-002, F-S2104-P8-007)"
  - "2026-07-25 D-905: F-S2104-P9-004 §T-008 BC trace corrected (PC2 + Invariant 2 caller-side dispatch gate, per story AC-007); F-S2104-P9-007 §T-009 mutant evidence recorded verbatim; input-hash 43e6df2→389274b (story v1.13 drift); version 1.6→1.7"
  - "2026-07-26 D-906: F-S2104-P10-007 Summary HEAD 9d896bf5→2c8eff8b + suite-level verification (bats 9/9+14/14 at 2c8eff8b 2026-07-26); F-S2104-P10-008 §T-009 mutant line :285 unnamed→named; F-S2104-P10-009 BC-6.26.001 cite v1.10→v1.11 throughout; T-008/T-009 version-pin parity restored; traces_to v1.10→v1.11; input-hash 389274b→455740d; version 1.7→1.8"
  - "2026-07-26 D-907 (date-corrected D-908; prior D-906 dated 2026-07-26, D-907 originally dated 2026-07-25 was non-monotonic — F-S2104-P12-010): F-S2104-P11-005 §T-009 mutant evidence :286 count clause replaced — 5 gate GROUPS spanning 9 assertion sites reconciled with 8 mutants; input-hash 455740d→d1c79e9 (story v1.15 drift); version 1.8→1.9"
  - "2026-07-26 D-908: F-S2104-P12-002(a) §T-009 :287 primary-path gate group description corrected to .md-qualified form with P11-003 attestation; F-S2104-P12-002(b) P11-003 mutant record appended; F-S2104-P12-002(c) Summary HEAD cite 2c8eff8b→264f53b6; F-S2104-P12-010 D-907 modified[] date 2026-07-25→2026-07-26 (monotonic correction); input-hash d1c79e9→53500af (story v1.16 drift); version 1.9→1.10"
  - "2026-07-26 D-909: F-S2104-P13-002 Summary HEAD 264f53b6→09cfce81 + COUNT-FREE pointer + Pass-13 attestation appended; F-S2104-P13-004 §Bats Tests T-001/T-002 multi-AC sync + §Traces T-001/T-002 multi-AC sync from story v1.17; input-hash 53500af→89efd7e (story v1.17 drift); version 1.10→1.11"
  - "2026-07-26 D-912: F-S2104-P14R-002 Summary HEAD 09cfce81→26b85d8c + Pass-14R attestation appended; F-S2104-P14R-009 Fixture column T-001/T-002/T-003 dynamic $(mktemp -d) corrected; F-S2104-P14R-010 pass-13 mutant exact text recovered + substituted; input-hash 89efd7e→c74e0f8 (story v1.19 drift); version 1.11→1.12"
  - "2026-07-26 D-914: F-S2104-P15-001/002/004 Summary HEAD 26b85d8c→8b39277b + Pass-15 attestation appended (sentence-scoped Gates 1/4/5 + Gate 6 two-part polarity + per-gate same-AC audit + bare-pin sweep); input-hash c74e0f8→3d12427 (story v1.20 drift); version 1.12→1.13"
  - "2026-07-26 D-916: F-S2104-P16-005 corrections — §Pass-15 completeness-claim corrected per D-916; M-P15-A label relabeled as M-P15-A-simplified [Correction at v1.14 (D-916)]; ### Pass-16 assertion-site attestation (9ab1aa32) appended (TWO-TIER verbatim: 9 adversary-verbatim + 11 test-writer instantiation vectors); gate-indexed 15-row + obligation-indexed 5-row audit tables appended; Summary HEAD 8b39277b→9ab1aa32; input-hash 3d12427→1baca60 (story v1.21 drift); version 1.13→1.14"
  - "2026-07-26 D-918: F-S2104-P17-005 (a)-(e) Pass-16 corrections — abbreviation splitter corrected to `cf. |i.e. |e.g. ` (3 forms; [Correction at v1.15 (D-918)]); anchor-uniqueness domain corrected to `#### Write Discipline` section at both attestation sites; M-P16-B Gate cell corrected (Gate 1(a) affirmative fires on inverted paragraph; decoy excluded by bounding, count=1, not anchor-uniqueness count=2); absent-block guard domain corrected to `$prohibition_block`; Gate 5 row corrected (actual alternation `CWD-relative\|worktree-relative\|relative[[:space:]]+path`, no prohibition-token clause); ### Pass-17 assertion-site attestation (c89bef22) appended (3-commit wave summary; battery table 9 new vectors + 9 prior re-verified RED; verbatim captured stdout for M-P17-A/C/C-control/D/F/G/H/in-worktree-residual; 17-gate table at c89bef22 + Gates 6(b)/7(b) RETIRED; extraction mechanisms table; obligation-indexed table adding AC-001(b)/(c) rows; NAME-SET EQUALITY PASS); Summary HEAD 9ab1aa32→c89bef22; traces_to adds story v1.22; input-hash e6c640a→4b26b3b; version 1.14→1.15"
  - "2026-07-27 D-920: F-S2104-P18-006 M-P17-A battery row corrected (Gate PW-B only; Gate 5 false claim dropped at 2 sites); Gate 5 audit row corrected (M-P17-A dropped; true Gate-5 vectors cited; SEQUENCE-SHADOWED DEFENSE-IN-DEPTH noted); F-S2104-P18-007 D-918 input-hash placeholders corrected ([see frontmatter]→4b26b3b; [updated by compute-input-hash]→4b26b3b); ### Pass-18 assertion-site attestation (a4ec37d3) appended (35/35 battery; 20-gate table; 15-obligation table; ALTERNATION-DIRECTION STATEMENTS; NAME-SET EQUALITY PASS 19 gates); Summary HEAD c89bef22→a4ec37d3 (quintuple parity); traces_to adds story v1.23; input-hash 4b26b3b→f86871a (story v1.23 drift); version 1.15→1.16"
  - "2026-07-27 D-923: F-S2104-P19-011 balanced-fence audit row truncation-guard claim corrected to well-formedness invariant; F-S2104-P19-012 write-directive audit row escape-discrimination controls added; F-S2104-P19-010 NAME-SET EQUALITY TWO→THREE + 19→21 gates + story v1.23→v1.25 + boundary-completeness + scope-restriction added; ### Pass-19 assertion-site attestation (657fce61) appended (21-gate; ESCAPE-SCOPE-PARITY; CONTROL escape evidence; NAME-SET EQUALITY PASS); traces_to adds story v1.25; input-hash f86871a→a03188b (story v1.25 drift; hook-authoritative); version 1.16→1.17"
  - "2026-07-27 D-925: ### Pass-20 assertion-site attestation (a5068252) appended (M-P20-A PW-B=FIRES(RED) WD=FIRES(RED) post-fix; PW-B clause-scoped Leg A F-S2104-P20-001; referent extended Leg B F-S2104-P20-002; 3-vector orchestrator-executed battery: M-P20-A+CONTROL-1+CONTROL-2; suites 9/9+14/14; input-hash a03188b UNCHANGED; version 1.17→1.18)"
  - "2026-07-28 D-933: F-S2104-P22-004 Summary HEAD a4ec37d3→63eae07d (23/23: 9/9+14/14, 2026-07-28; THREE-PASS-OMISSION error-acknowledgment: D-923/D-925/D-929 each appended attestation section without advancing Summary HEAD); F-S2104-P22-005 ### Pass-22 assertion-site attestation (63eae07d) appended with 14 verbatim mutant records; F-S2104-P22-012 §G.1 L31-40 volatile pin→behavioral anchor; traces_to BC-6.26.001 v1.11→v1.12 + story v1.26; input-hash a03188b→2a798de (story v1.26 + BC v1.12 drift); version 1.18→1.19"
  - "2026-07-28 D-934: Summary HEAD 63eae07d→888b5b73 (23/23: 9/9+14/14, 2026-07-28; F-S2104-P22-004 RECURRENCE — per-pass closure checklist exercised again; one pass after D-933 closed this class); ### Pass-23 assertion-site attestation (888b5b73) appended (B2 CLOSED: F-S2104-P23-001 blockquote-strip→marker-only-strip + F-S2104-P23-002 tautological-records→14-genuine-per-guard records; 12 OPEN routed to pass-24; P23-013 NON-FINDING documented); version 1.19→1.20"
  - "2026-07-28 D-935: Summary HEAD 888b5b73→9b12aa00 (24/24: 10/10+14/14, 2026-07-28; pipeline probe T-010 added); F-S2104-P24-005 POLICY 15 shortfall CLOSED (true prior state 1-2/14 RED commands / 0/14 restore commands; orchestrator attribution — second evidence-fidelity failure); adversary-pass-24.md persisted (B2/H2/M2/L0 = 6 findings; targeted re-review); ### Pass-24 assertion-site attestation appended (test-writer Reversions 1+2 + 4 orchestrator tests verbatim; F-S2104-P24-003 structural closure: call-site parity Leg E); version 1.20→1.21"
  - "2026-07-28 D-936: F-S2104-P23-009 CLOSED (F-22-NNN→F-S2104-P22-NNN namespace normalization; 5 sites replaced; input-hash f69d6b0→61af172 after validate-input-hash hook block); F-S2104-P23-010 CLOSED (12 stale narrative bats line pins→behavioral anchors; TD-VSDD-091 comprehensive sweep — adversary cited ≥5, state-manager found 12 total); F-S2104-P23-014 GENUINELY-CLOSED (documented: D-934 already added 14 genuine per-guard records); Pass-24 + Pass-23-closure summary table rows added; Summary HEAD 9b12aa00→5ccf5869 (24/24: 10/10+14/14, 2026-07-28); traces_to BC-6.26.001 v1.12→v1.13 + story v1.27 added; version 1.21→1.22"
  - "2026-07-28 D-937: H01 POLICY 15 evidence persisted (P23-006/007/008/012; orchestrator B01 evidence verbatim — recorded (g) mutant fires production guard + T-015 corpus regression); M04 SHA transposition corrected at 6 sites (5ccf5669→5ccf5869); M07 sentinels D-{TBD-pass-23-fix-burst}→D-936 + D-{TBD-pass-25-fix-burst}→D-937 in BC-6.26.001/ADR-031/story; Pass-25 summary table row added; Summary HEAD 5ccf5869→4dc27251 (25/25: 10/10+15/15, 2026-07-28); traces_to BC-6.26.001 v1.13→v1.14 + story v1.28 added; version 1.22→1.23"
  - "2026-07-31 pass-30 fix burst (F-S2104-P30-H08/H09/H05/M01/L01): Summary HEAD c7c61688→44547051 (27/27: 11/11+16/16); T-017 allocated to EC-009 (POLICY 1 — T-010 is pipeline probe per pass-24 SoT); T-010 label added to pipeline probe @test name; inventory comment updated T-010→T-017; T-016 renamed test_coupling_gate_story_gate_count_matches_bats_count_word→test_coupling_gate_bats_sentinel_matches_runtime_gate_count (F-S2104-P30-L01); T-001 summary comment twenty-three→twenty-four (F-S2104-P30-H05); 6 phantom numeric IDs F-S2104-P28-016→F-S2104-P28-L01; F-S2104-P28-017→F-S2104-P28-L02 in worktree-identity-preflight.bats (F-S2104-P30-M01); Pass-28+Pass-29 attestation sections appended; Pass-28+Pass-29 pass table rows added; T-017 RG-010 section added; STRUCTURAL T-016 name updated; traces_to story v1.33 + BC-6.26.001 v1.18 added; input-hash updated; version 1.27→1.28"
  - "2026-07-29 D-940: pass-27 findings CLOSED (B01 positional conjuncts; H01 sibling-sweep gap; H02 count-word + T-016; M01/M02/M03 BC v1.15 plain-path+! -type d+EC-009; M04 comment-strip); POLICY 15 restore-leg residual CLOSED via CONTROL-equivalence; Pass-27 summary table row added; Pass-27 Follow-up M01(a)/M03(a) and attestation sections already present (written during fix-burst); Summary HEAD 7c3338e7→c7c61688 (26/26: 10/10+16/16); T-016 coupling gate added; traces_to story v1.30 + BC-6.26.001 v1.15; input-hash 105492a→6a61cd1; version 1.26→1.27"
  - "2026-07-28 D-939: H04 CLOSED (D-939 SUPERSESSION — records committed by a8ec290e in-flight during D-938 dispatch race; 7/8 per-guard records; P25-M06 NOT PRODUCIBLE); POLICY 15 restore-leg residual OPEN (pass-27 anchor); Pass-26 summary table H04 OPEN→CLOSED; Pass-26 attestation H04 section rewritten; version 1.24→1.25"
  - "2026-07-28 D-938: adversary-pass-26.md persisted (B2/H4/M3/L2 = 11 findings; novelty HIGH; streak 0/3; ADR-033 cross-family limitation; POLICY 22 both-directions record: concern-1 CONFIRMED→B01; concern-2 REFUTED); 10 of 11 CLOSED at 7c3338e7 (H04 OPEN — per-guard mutant evidence requested twice, not produced; recorded OPEN per production-grade discipline); Summary HEAD 4dc27251→7c3338e7 (25/25: 10/10+15/15, 2026-07-28); Pass-26 summary table row added; Pass-26 assertion-site attestation appended (orchestrator B01/B02 verbatim evidence; H04 OPEN evidence gap); traces_to story v1.29 added; D-938 sentinels replaced 6 sites (ARCH-INDEX ×2, ADR-031 ×2, story ×2); input-hash 101b1e6→044b138 (story v1.29 + ADR-031 D-938 sentinel drift); version 1.23→1.24"
stub_architect_agent: "N/A — no code stubs (skill-doc + bats story; ADR-031 §Decision 4 class; POLICY 21 satisfied)"
stub_compile_verified: true
test_writer_agent: vsdd-factory:test-writer
red_gate_verified: true
---

# Red Gate Log — S-21.04 (story-worktree write-path discipline and teardown preflight)

**Date:** 2026-07-24
**Branch:** feature/S-21.04-story-worktree-write-path-discipline @ 8e3c432e (failing-tests commit; base 948f0fb1)
**Test Writer:** vsdd-factory:test-writer
**Status:** RED_GATE_VERIFIED

## Summary

| Story | New Tests Written | All New Tests Fail (Red)? | Pre-existing Tests | Gate |
|-------|------------------|--------------------------|-------------------|------|
| S-21.04 | 11 bats tests: T-001..T-006 (behavioral vectors) + T-007/T-008/T-009 (doc-parity propagation gates) + T-010 (pipeline probe; F-S2104-P24-003) + T-015 (B01 corpus regression; `test_BC_B01_corpus_regression_guards_e_co_and_g_pc`) + T-016 (H02 coupling gate; `test_coupling_gate_bats_sentinel_matches_runtime_gate_count`) + T-017 (F-S2104-P30-H09 EC-009 stray-inode; `T-017 S-21.04 EC-009: stray-inode-inside-factory — symlink + FIFO inside real .factory/ dir → PREFLIGHT BLOCKED via ! -type d; missed by -type f predicate (M03(a) delta proof)`). All GREEN at worktree HEAD 44547051 (27/27: 11/11 + 16/16, 2026-07-31). All tests carry T-IDs (or equivalent IDs), RG rows, and attestation sections in this log. NOTE: Summary HEAD was stale at a4ec37d3 (pass-18) through passes 19-21; three-pass omission corrected at D-933. Per-pass closure checklist: advancing Summary HEAD to the post-fix fixes-HEAD is a mandatory closure step whenever an attestation section is appended. | YES — all original 3 FAIL at Red Gate | 2265 (cargo baseline) | PASSED |

Orchestrator-verified 2026-07-24: all 3 bats tests `not ok` (ASSERTION failures via `_assert_doc_marker`; DOC-PARITY: `find.*\.factory` preflight mandate absent from step-g-cleanup.md §G.1). Pre-implementation cargo-test baseline: 2265 pass, 0 fail, clean build.

## Stubs Created

### S-21.04: story-worktree write-path discipline and teardown preflight

Stub commit: `63b7fb79` (bats skeleton + `plugins/vsdd-factory/tests/fixtures/story-worktree/README.md`; 3 `skip` placeholders). N/A for code stubs — S-21.04 File Structure Requirements contain no code modules. Deliverables are skill-doc amendments (`_shared-context.md` + `step-g-cleanup.md`) plus one bats suite. ADR-031 §Decision 4 skill-doc mandate; POLICY 21 satisfied — no new `.sh` files added (no new executable scripts; existing fixture convention used). Workspace unchanged; `cargo check` trivially green.

Failing-tests commit: `8e3c432e` — replaced the 3 `skip` placeholders with assertion-bearing tests referencing absent step-g-cleanup.md §G.1 preflight mandate.

## Red Gate Verification

**Command:** `bats plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats`

**Result:** RED GATE PASSED. Output: `1..3` — all 3 `not ok`. Each test fails at `_assert_doc_marker` (DOC-PARITY) assertions referencing the absent §G.1 teardown preflight mandate in `step-g-cleanup.md`. Zero bash errors; zero skips.

### Bats Tests (`plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats`, commit 8e3c432e)

| Test | AC / RG ID | BC Trace | Fixture | Failure Reason | Status |
|------|-----------|----------|---------|----------------|--------|
| T-001 | AC-003; AC-001 (Write Discipline clause-content gates); AC-002 (§G.1 doc-parity + harness, with T-002); AC-007 (a)-(c) primary-surface gates / RG-001† | BC-6.26.001 PC2b, Invariant 2 | dynamic $(mktemp -d) fixture per bats setup() (fixtures/story-worktree/ holds README documentation only) (stray `.factory/stories/S-021-DELIVERY.md`) | `_assert_doc_marker` gate fires — `find.*\.factory` preflight mandate absent from step-g-cleanup.md §G.1; `PREFLIGHT BLOCKED` clause not present | FAIL (expected) |
| T-002 | AC-004; AC-002 (§G.1 doc-parity, with T-001) / RG-002† | BC-6.26.001 PC2a | dynamic $(mktemp -d) fixture per bats setup() (fixtures/story-worktree/ holds README documentation only) (empty shadow `.factory/`) | `_assert_doc_marker` gate fires — §G.1 clean-path assertion absent from step-g-cleanup.md | FAIL (expected) |
| T-003 | AC-005 / RG-003† | BC-6.26.001 PC2b→PC2a retry path | dynamic $(mktemp -d) fixture per bats setup() (fixtures/story-worktree/ holds README documentation only) (stray file then relocated) | `_assert_doc_marker` gate fires — §G.1 retry-path mandate absent from step-g-cleanup.md | FAIL (expected) |

## Anti-Tautology Mechanism (TD-VSDD-059)

`_extract_g1_section` awk extraction gate: the harness extracts §G.1 from `step-g-cleanup.md` at test setup time and executes only doc-extracted preflight logic. Pre-implementation extraction gate fires "preflight mandate absent" — the awk extraction finds no §G.1 teardown-preflight section, so the gate itself confirms the absence rather than executing stub code that might vacuously pass. Verified present: 5 references to `_extract_g1_section` in the bats suite at commit `8e3c432e`.

This mechanism guarantees non-tautology: no amount of fixture manipulation can cause T-001/T-002/T-003 to pass until `step-g-cleanup.md` §G.1 contains the actual preflight mandate text.

## Mutant Vector (POLICY 15 v1.4.10 per-guard mutant verification)

**T-001 mutant vector:** stray file `.factory/stories/S-021-DELIVERY.md` in fixture story-worktree + load-bearing `$REMOVE_LOG` sentinel assertion `[ ! -s ]`. Mutant: omitting the `[ ! -s $REMOVE_LOG ]` assertion allows a passthrough where `git worktree remove` is called even when the stray file is present. The `$REMOVE_LOG` sentinel records whether the remove was invoked; the `[ ! -s ]` check (file not non-empty) is load-bearing. Removing this assertion would cause T-001 to pass falsely post-implementation if the BLOCKED path is not wired.

## Regression Check

| Existing Tests | Status |
|---------------|--------|
| 2265 pre-existing cargo tests (pre-implementation baseline) | all pass |

Zero regressions. Bats suite is additive; no existing bats fixture modified. No Rust crate changes at stub or failing-tests commits.

## Failure Mode Verification

All three tests fail via DOC-PARITY `_assert_doc_marker` assertions — the tests extract §G.1 from `step-g-cleanup.md` via the `_extract_g1_section` awk gate; the section does not exist pre-implementation, causing the extraction gate to fire "preflight mandate absent" rather than executing any preflight logic. Tests will turn green only once the implementation amendments land in `plugins/vsdd-factory/skills/deliver-story/steps/step-g-cleanup.md` (AC-003/AC-004/AC-005) and `plugins/vsdd-factory/skills/deliver-story/steps/_shared-context.md` (AC-001/AC-002, Write Discipline clause).

No `#[should_panic]` masking. No vacuously-passing new tests. Failure mechanism is behavioral (gate absence / DOC-PARITY extraction), not infrastructure panic.

## Traces

- T-001 (AC-003; AC-001; AC-002; AC-007 (a)-(c) / RG-001†) → BC-6.26.001 v1.11 PC2b (Invariant 2): stray `.factory/` file triggers PREFLIGHT BLOCKED; `git worktree remove` NOT called
- T-002 (AC-004; AC-002 / RG-002†) → BC-6.26.001 v1.11 PC2a: empty shadow tree → teardown proceeds; `git worktree remove` IS called
- T-003 (AC-005 / RG-003†) → BC-6.26.001 v1.11 PC2b→PC2a retry path: stray file relocated → preflight re-runs clean → teardown proceeds
- T-004 (AC-006 / RG-004) → BC-6.26.001 v1.11 PC2c: non-path-absent find error → fail-closed HALT; `git worktree remove` NOT called (addendum D-896)
- T-005 → AC-002 (EC-007) / RG-006 / BC-6.26.001 PC2b (non-directory inode)
- T-006 → AC-002 (EC-008) / RG-005 / BC-6.26.001 PC2b (symlink at path)

† applies to this table and §Traces: original log fabricated RG-003/004/005 for T-001..003; corrected to RG-001/002/003 (D-895 §Traces; D-904 this table — the table correction was omitted at D-895, surviving six passes; see F-S2104-P8-001).

## Commits

- `63b7fb79` — stub commit: bats skeleton (3 `skip` placeholders) + `fixtures/story-worktree/README.md`
- `8e3c432e` — failing-tests commit: `test(S-21.04): add failing tests for BC-6.26.001 PC2a/PC2b teardown preflight and PC1 write discipline` — 3 skips replaced with `_assert_doc_marker` assertion tests; Red Gate PASSED

## Hand-Off to Implementer

Story ready for implementation. No dependency gates outstanding (`depends_on: []`).

Implementation tasks (from story spec):

1. **Amend `plugins/vsdd-factory/skills/deliver-story/steps/_shared-context.md`** — add "Write Discipline" clause under §Spec-Path Discipline covering `.factory/**` writes; name DELIVERY ledger + pr-review.md; mandate canonical absolute path resolution via `CANONICAL_FACTORY_ROOT` or `git -C <main-worktree> rev-parse --show-toplevel` (BC-6.26.001 PC1 + Invariants 1, 3, 4). Unblocks AC-001.
2. **Amend `plugins/vsdd-factory/skills/deliver-story/steps/step-g-cleanup.md` §G.1** — add mandatory teardown preflight sub-step before `git worktree remove` dispatch; implement PC2a/PC2b logic: `find .worktrees/<story>/.factory -type f` → if non-empty emit `PREFLIGHT BLOCKED` (PC2b); if empty proceed with `git worktree remove` (PC2a); include retry-path documentation (BC-6.26.001 PC2 + Invariant 2 + Invariant 5). Unblocks AC-002, T-001 (AC-003/RG-001), T-002 (AC-004/RG-002), T-003 (AC-005/RG-003).
3. **Verify bats green:** `bats plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats` → `1..3`, all 3 `ok`.
4. **Verify cargo regression clean:** `cargo test --workspace --all-targets` → 2265+ pass, 0 fail.

---

## Erratum (F-S2104-P1-009) — RG-ID Mapping and AC-002 Attribution Correction

**Appended:** 2026-07-25 (D-895 S-21.04 pass-1 closure; state-manager)

The original version of this red-gate-log contained three defects identified by the pass-1 adversarial review as F-S2104-P1-009 (HIGH):

### Defect 1 — Fabricated RG-004 and RG-005 IDs

The Bats Tests table and Traces section used `RG-004` and `RG-005` as Red Gate identifiers. The story's Red Gate Test Plan (BC-6.26.001 v1.3) defines **RG-001, RG-002, RG-003** only. RG-004 and RG-005 do not exist in the story SoT.

### Defect 2 — T-001 mis-mapped to RG-003 (should be RG-001)

The Bats Tests table showed `T-001 | AC-003 / RG-003` and Traces showed `T-001 (AC-003 / RG-003)`. Correct mapping per story Red Gate Test Plan:

| Test | Correct RG / AC | Behavior |
|------|----------------|----------|
| T-001 | RG-001 / AC-003 | PC2b: stray `.factory/` file → PREFLIGHT BLOCKED |
| T-002 | RG-002 / AC-004 | PC2a: empty shadow tree → teardown proceeds |
| T-003 | RG-003 / AC-005 | PC2b→PC2a retry path |

### Defect 3 — AC-002 attributed to `_shared-context.md` (Hand-Off task 1)

Hand-Off task 1 originally read: "Unblocks AC-001/AC-002." AC-002 (teardown preflight sub-step in step-g-cleanup.md) is **not** unblocked by `_shared-context.md` amendments. AC-002 is defined against `step-g-cleanup.md §G.1` (Hand-Off task 2). Corrected to "Unblocks AC-001." in the Hand-Off section above.

**Authority:** story v1.4 §Red Gate Test Plan (RG-001..RG-003); BC-6.26.001 v1.4 AC-002 definition. Fix committed D-895 burst, state-manager.

### RG-Reconciliation Note (appended D-902)

The D-895 erratum documented that "RG-004 and RG-005 do not exist in the story SoT." That claim was correct AT ITS TIME — story v1.4 (the SoT when the erratum was written) defined only RG-001..RG-003. Both RG-004 and RG-005 were subsequently allocated as the story grew during the adversarial cascade:

- **RG-004**: allocated in story v1.5 (commit 6149e893; story-writer F-S2104-P2-007 five-table propagation fix; PC2c fail-closed HALT test). A legitimate addition; the erratum's claim was accurate against v1.4 but does not govern v1.5+.
- **RG-005**: allocated in story v1.8 (commit 04aa9ff3; story-writer F-S2104-P5-008/F-S2104-P5-011 executor obligation + symlink vector; AC-007/AC-008 gates). A legitimate addition.

The erratum is historically correct. The fabricated IDs documented there were fabricated by the original red-gate-log author at v1.0; the later legitimate allocations of RG-004 and RG-005 in the story are independent events. No correction to the erratum body is needed — this reconciliation note provides the temporal clarification.

---

## T-004 / RG-004 Attestation Addendum (F-S2104-P2-013)

**Appended:** 2026-07-25 (D-896 S-21.04 pass-2 closure; state-manager)

The original red-gate-log covered T-001/T-002/T-003 only. T-004 (PC2c fail-closed HALT) was added by the test-writer as a pass-1 fix leg (F-S2104-P1-003) at commit `7d38b9e6`. The original attestation record did not include T-004's Red Gate status. This addendum corrects that omission.

### T-004 — PC2c Fail-Closed HALT (AC-006 / RG-004)

**Test:** T-004 asserts that when `find <worktree>/.factory -type f` exits non-zero for a reason other than path-absence (PC2c condition), teardown MUST HALT per verbatim §G.1 PC2c (step-g-cleanup.md): "If `find` exits non-zero for a non-path-absent reason (e.g., permission denial, traversal error), teardown MUST HALT. Surface the exact find exit code and stderr to the operator. `git worktree remove` is NOT executed — find errors must not silently authorize removal of unverified worktree content (BC-6.26.001 PC2c)." The PC2b `PREFLIGHT BLOCKED` message does NOT apply to PC2c.

**Red Gate state at test-writer commit `7d38b9e6`:**

Suite run: `bats plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats` → `1..4`; all 4 `not ok`. T-004 fails at `_assert_doc_marker` (DOC-PARITY): "PC2c fail-closed HALT branch must be documented in §G.1" — the PC2c HALT clause was not yet present in `step-g-cleanup.md §G.1`. Pre-implementation, the awk extraction gate confirms absence; T-004 cannot pass until the PC2c branch is explicitly documented.

**Implementation commit that turned T-004 green:** `19271a65` — added PC2c HALT clause to `step-g-cleanup.md §G.1`. Verbatim from the actual §G.1 text (read from commit `19271a65` worktree): "teardown MUST HALT. Surface the exact find exit code and stderr to the operator. `git worktree remove` is NOT executed — find errors must not silently authorize removal of unverified worktree content (BC-6.26.001 PC2c)." Post-implementation suite: `1..4`; all 4 `ok`.

**RG-004 source of truth:** story v1.5 §Red Gate Test Plan (6149e893). Story v1.4 covered RG-001..RG-003 only; RG-004 (PC2c) was added by story-writer at commit 6149e893 as part of F-S2104-P2-007 five-table propagation fix.

**BC trace:** T-004 (AC-006 / RG-004) → BC-6.26.001 v1.11 PC2c: non-path-absent find error → fail-closed HALT; `git worktree remove` NOT called.

**Summary row for completeness:**

| Test | AC / RG ID | BC Trace | Failure Reason at Red Gate | Green Commit |
|------|-----------|----------|---------------------------|--------------|
| T-004 | AC-006 / RG-004 | BC-6.26.001 v1.11 PC2c | DOC-PARITY: PC2c fail-closed HALT clause absent from §G.1 at `7d38b9e6` | `19271a65` |

---

## T-005 Attestation Addendum (F-S2104-P6-004)

**Appended:** 2026-07-25 (D-902 S-21.04 pass-6 closure; state-manager)

T-005 (AC-002 regular-file-at-path / RG-006 non-directory-inode) was added by the test-writer as part of the pass-4 fix burst. The original red-gate-log did not include T-005's Red Gate attestation.

### T-005 — Regular File at .factory Path (AC-002 EC-007 / RG-006)

**Test:** T-005 creates a REGULAR FILE at <worktree>/.factory (fixture: touch $MOCK_WORKTREE/.factory) and asserts the preflight routes to PC2b BLOCKED with the path reported, find NOT invoked, non-zero exit, and git worktree remove NOT called. Under the pre-fix §G.1 (predicate [ ! -d ], no non-directory branch) a regular file at the path would satisfy 'not a directory' and authorize teardown — destroying the file. Observed RED at worktree commit 60f0d2d6 (DOC-PARITY: '[ ! -e ] predicate and non-directory→PC2b clause absent from step-g-cleanup.md §G.1'); GREEN at 73c2bade. Registered as RG-006 in story §Red Gate Test Plan (v1.11).

**Red Gate state — worktree commit `60f0d2d6` (pass-4 baseline):**

Suite run: `bats plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats` → `1..8`; 5 of 8 `not ok` (orchestrator independently ran the suite). T-005 failed at `_assert_doc_marker` (DOC-PARITY): "step-g-cleanup.md missing `[ ! -e ]` predicate and non-directory→PC2b clause" — the non-directory→PC2b branch was not yet present in §G.1. Pre-implementation, the awk extraction gate confirms absence of the non-directory clause.

**Implementation commit that turned T-005 green:** `73c2bade` — added `[ ! -e ]` existence pre-test and non-directory→PC2b branch to §G.1.

**BC trace:** T-005 (AC-002 / RG-006) → BC-6.26.001 v1.11 PC2b (non-directory inode): non-directory inode at .factory path → PREFLIGHT BLOCKED; `git worktree remove` NOT called.

**Summary row:**

| Test | AC / RG ID | BC Trace | Failure Reason at Red Gate | Green Commit |
|------|-----------|----------|---------------------------|--------------|
| T-005 | AC-002 / RG-006 | BC-6.26.001 v1.11 PC2b (non-directory inode) | DOC-PARITY: non-directory→PC2b clause absent from §G.1 at `60f0d2d6` | `73c2bade` |

---

## T-006 / RG-005 Attestation Addendum (F-S2104-P6-004)

**Appended:** 2026-07-25 (D-902 S-21.04 pass-6 closure; state-manager)

T-006 (AC-002 EC-008 / RG-005 symlink-at-path) was added by the test-writer as part of the pass-5 fix burst (F-S2104-P5-011 symlink-to-DIRECTORY data-loss vector; test-writer commit `93ec340a`). The original red-gate-log did not include T-006's Red Gate attestation.

### T-006 — Symlink at .factory Path (AC-002 EC-008 / RG-005)

**Test:** T-006 asserts that when the worktree's `.factory/` path exists as a symlink (symlink-to-directory), the preflight protocol detects the symlink via `[ -L ]` (step 2 of the 4-step chain) BEFORE invoking `find`, and classifies the symlink-at-path case as PC2b PREFLIGHT BLOCKED. The escape mechanism: POSIX test -d follows symlinks (a symlink-to-directory satisfies [ -d ]), while POSIX find WITHOUT -H/-L does not descend a symlink argument and returns empty output — a false PC2a. The [ -L ] guard (before any [ -d ] test) routes any symlink to PC2b without invoking find; the mandated trailing-slash find form is defense-in-depth that forces traversal entry if a symlink were ever to reach the find branch.

**Red Gate state — worktree commit `93ec340a` (pass-5 baseline, test-writer's own commit adding T-006):**

At commit `93ec340a` (the failing-tests commit for T-006), the §G.1 `[ -L ]` symlink guard was not yet present in `step-g-cleanup.md`. Suite run: `bats plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats` → T-006 `not ok` (orchestrator-verified). Failure: DOC-PARITY `_assert_doc_marker` — "§G.1 symlink→PC2b clause absent."

**Implementation commit that turned T-006 green:** `4833a642` — added `[ -L ]` symlink guard as step 2 of the §G.1 4-step chain in `step-g-cleanup.md`.

**Mutant self-check (pass-6 hardening, commit `772096f4`):** Test-writer performed scratch deletion of the `[ -L ]` symlink-guard paragraph in §G.1 (the guard block) at commit `772096f4` as a mutant self-check — T-006 turned RED on the mutated §G.1, proving the load-bearing `[ -L ]` gate is not satisfied by the PC2b header line alone. Deletion confirmed as FAIL, restoring §G.1 confirmed GREEN. This closes the pass-6 F-S2104-P6-003 gate-weakening finding.

**BC trace:** T-006 (AC-002 / RG-005) → BC-6.26.001 v1.11 PC2b symlink-at-path: `[ -L ]` step 2 of the 4-step chain; symlink-at-path → PREFLIGHT BLOCKED; `git worktree remove` NOT called.

**RG-005 source of truth:** story v1.8 §Red Gate Test Plan (commit `04aa9ff3`). Story v1.7 covered RG-001..RG-004; RG-005 (symlink-at-path/PC2b) was added by the story-writer at commit `04aa9ff3` as part of F-S2104-P5-011 five-table propagation.

**Summary row:**

| Test | AC / RG ID | BC Trace | Failure Reason at Red Gate | Green Commit |
|------|-----------|----------|---------------------------|--------------|
| T-006 | AC-002 / RG-005 | BC-6.26.001 v1.11 PC2b symlink-at-path | DOC-PARITY: §G.1 `[ -L ]` symlink→PC2b clause absent at `93ec340a` | `4833a642` |

---

## T-007 — devops-engineer Executor-Side Preflight Mandate (AC-008 / RG-007)

**Appended:** 2026-07-25 (D-903 S-21.04 pass-7 closure; state-manager)

### T-007 — devops-engineer Executor-Side Preflight Mandate (AC-008 / RG-007)

T-007 is the doc-parity gate authored at pass-4 as F-S2104-P4-003 (bats gate asserting agents/devops-engineer.md §Worktree Cleanup carries the §G.1 preflight-verification mandate). Observed RED at 60f0d2d6 (pass-4 baseline: '§Worktree Cleanup section has no §G.1/step-g-cleanup/BC-6.26.001 mandate'); GREEN at 0c0922e1. Retro-registered as T-007↔AC-008 at story v1.9 and RG-007 at story v1.11; its red-gate history predates AC-008's authoring — recorded as-is. Strengthened to obligation-asserting form (verify-PASS + not-evident-run-yourself) at 052620dc. Mutant evidence (recorded): scratch reduction of devops-engineer.md §Worktree Cleanup to 'Run git worktree remove (see BC-6.26.001).' → all four obligation gates NO MATCH (RED): qualified-path, verify, PASS-result, not-evident-run-yourself; restoring the section → GREEN. Performed by test-writer at 052620dc.

**BC trace:** T-007 → AC-008 / RG-007 / BC-6.26.001 v1.11 Precondition 3: devops-engineer.md §Worktree Cleanup MUST carry the §G.1 preflight-verification mandate unconditionally; obligation-asserting gate confirms PASS result + not-evident-run-yourself attestation.

**Summary row:**

| Test | AC / RG ID | BC Trace | Failure Reason at Red Gate | Green Commit |
|------|-----------|----------|---------------------------|--------------|
| T-007 | AC-008 / RG-007 | BC-6.26.001 v1.11 Precondition 3 | DOC-PARITY: §Worktree Cleanup had no §G.1/step-g-cleanup/BC-6.26.001 mandate at `60f0d2d6` | `0c0922e1` |

---

## T-008 — Six-Surface §G.1 Mandate Regression Gates (AC-007 / RG-008)

**Appended:** 2026-07-25 (D-904 S-21.04 pass-8 closure; state-manager)

### T-008 — Six-Surface §G.1 Mandate Regression Gates (AC-007 / RG-008)

T-008 is the doc-parity gate authored at pass-4 as F-S2104-P4-009 (anti-inline-find + qualified §G.1 references across worktree-manage/SKILL.md, code-delivery/SKILL.md, fix-pr-delivery/SKILL.md, code-delivery.lobster, greenfield.lobster, rules/worktree-protocol.md). Observed RED at 60f0d2d6 (pass-4 baseline: 5 of 6 surfaces carried the inline-find anti-pattern or lacked qualified refs); GREEN at a317fd77. Strengthened quote-tolerant at 052620dc — recorded mutant evidence: pasting the canonical quoted line (find "<worktree-path>/.factory/" -type f) into a scratch copy of a delegating surface → OLD regex NO MATCH (false-green confirmed), NEW regex MATCH (gate fires RED); unquoted no-slash and unquoted with-slash forms MATCH under both. Registered T-008/RG-008 at story v1.12.

**BC trace:** T-008 (AC-007 / RG-008) → BC-6.26.001 v1.11 PC2 + Invariant 2 (caller-side dispatch gate), per story AC-007.

**Summary row:**

| Test | AC / RG ID | BC Trace | Failure Reason at Red Gate | Green Commit |
|------|-----------|----------|---------------------------|--------------|
| T-008 | AC-007 / RG-008 | BC-6.26.001 v1.11 PC2 + Invariant 2 (caller-side dispatch gate), per story AC-007 | DOC-PARITY: 5 of 6 surfaces carried inline-find anti-pattern or lacked qualified refs at `60f0d2d6` | `a317fd77` |

---

## T-009 — Awareness-Clause Doc-Parity (AC-009 / RG-009)

**Appended:** 2026-07-25 (D-904 S-21.04 pass-8 closure; state-manager)

### T-009 — Awareness-Clause Doc-Parity (AC-009 / RG-009)

T-009 is the doc-parity gate authored at pass-4 as F-S2104-P4-002 (agents/adversary.md + skills/adversarial-review/SKILL.md must state the corrected shadow-write model and reference the §G.1 preflight as the enforcement chain). Observed RED at 60f0d2d6 (pass-4 baseline: neither file contained §G.1/BC-6.26.001); GREEN at 4265c96c. Registered T-009/RG-009 and anchored to NEW AC-009 (BC-6.26.001 Invariant 5) at story v1.12.

**Mutant evidence (recorded):** scratch reduction of the adversary.md corrected-model clause to '…resolve the tuple (see BC-6.26.001).' → all three obligation gates NO MATCH (RED): corrected-model, report-as-defect-signal, §G.1 enforcement-chain; the RETIRED bare alternation MATCHED the same mutant (paper-gate confirmed); restore → GREEN. Performed by test-writer at 2992b53d; gates class-completed at 3326e4dd (gate groups and assertion sites are enumerated per-pass in the attestation sections below; running totals are not maintained as prose counts: the 6-surface _assert_g1_ref helper (fully-qualified path form); the section-bounded primary-path gates (SKILL.md Step 8, orchestrator step (g), Story Split Recovery — all strengthened to .md-qualified form at 92f986ab with 3 extensionless-degradation mutants RED/restore GREEN — and winning-playbook Step 8, .md-qualified since 2c8eff8b); the §G.1 non-directory gates (routing co-occurrence, one shared mutant); the adversarial-review defect-signal gate (spec-ground-truth co-occurrence); the devops-engineer verify gate ('dispatching caller' token). 8 scratch mutants RED / 8 restores GREEN (the two non-directory sites shared one mutant) recorded by test-writer).

**BC trace:** T-009 (AC-009 / RG-009) → BC-6.26.001 v1.11 Invariant 5 (awareness-surface anchor): adversary.md + adversarial-review/SKILL.md MUST state the corrected shadow-write model and reference §G.1 preflight as the enforcement chain.

**Summary row:**

| Test | AC / RG ID | BC Trace | Failure Reason at Red Gate | Green Commit |
|------|-----------|----------|---------------------------|--------------|
| T-009 | AC-009 / RG-009 | BC-6.26.001 v1.11 Invariant 5 | DOC-PARITY: neither adversary.md nor adversarial-review/SKILL.md contained §G.1/BC-6.26.001 at `60f0d2d6` | `4265c96c` |

**P11-003 mutant evidence (recorded):** three extensionless degradations (steps/step-g-cleanup §G.1 form) applied per-gate in scratch copies → each strengthened gate exit 1 (RED); originals restored → exit 0 (GREEN). Performed by test-writer at 92f986ab. Gates: SKILL.md Step 8, orchestrator step (g), Story Split Recovery — all three strengthened from bare `step-g-cleanup` alternation to `step-g-cleanup.md`-qualified form. Winning-playbook Step 8 was already .md-qualified at 2c8eff8b and was not the subject of P11-003 strengthening.

---

## T-010 — BC-6.26.001 Pipeline Probe (RG-010)

**Appended:** 2026-07-31 (F-S2104-P30-H08 pass-30 fix burst; test-writer)

### T-010 — BC-6.26.001 Pipeline Probe (RG-010)

T-010 is the pipeline probe authored at pass-24 as F-S2104-P24-003 (call-site parity Legs A/B/C/D/E). Observed GREEN at `9b12aa00` (pass-24 fix burst). Tests that real-fixture mutations on a dynamically-constructed MOCK_WORKTREE exercise the full production domain-construction path (`_run_teardown_preflight` → `_extract_g1_section` → actual `find` semantics via extracted §G.1 command; not a stub). POLICY 1 note: T-010 ID was erroneously re-allocated to EC-009 stray-inode test at pass-28; red-gate-log SoT (this entry + pass-24 row) is the canonical authority; EC-009 test has been allocated T-017 per F-S2104-P30-H09.

**BC trace:** T-010 → AC-001 (write-directive gate + Gate PW-B domain-construction path; Legs A/B/C/D/E) / BC-6.26.001 v1.15.

**Bats test name:** `T-010 BC-6.26.001 pipeline probe: real-fixture mutations exercise production domain-construction path (F-S2104-P24-003)` (`story-worktree-write-path-discipline.bats`).

**Summary row:**

| Test | AC / RG ID | BC Trace | Failure Reason at Red Gate | Green Commit |
|------|-----------|----------|---------------------------|--------------|
| T-010 | AC-001 (write-directive gate + Gate PW-B) / RG-010 | BC-6.26.001 v1.15 | N/A — pipeline probe added post-implementation to verify full domain-construction path | `9b12aa00` |

**Verification (F-S2104-P30-H08):**

```
$ bats -f "T-010" plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats
1..1
ok 1 T-010 BC-6.26.001 pipeline probe: real-fixture mutations exercise production domain-construction path (F-S2104-P24-003)
```

---

### Pass-13 assertion-site attestation (09cfce81)

**Appended:** 2026-07-26 (D-909 S-21.04 pass-13 closure; state-manager)

New/changed assertion sites: (1) `_extract_write_discipline_prohibition_block()` helper — extracts the normative prohibition paragraph from `_shared-context.md` (anchored to 'All .factory/** artifact writes', terminated at first blank line). (2) T-001 prohibition gate block — the 'are FORBIDDEN' call replaced by: empty-block guard, Gate 1 (mandate-polarity line-level grep 'MUST.*absolute|absolute.*MUST'), Gate 2 (CWD-relative FORBIDDEN joined-text co-occurrence), Gate 3 (kept **Forbidden:** example marker). (3-8) comment/message anchor + SHA completions in T-002/T-005/T-009 per F-P13-006/007/009. Mutant vectors (captured): (a) prohibition block deleted → T-001 RED 'DOC-PARITY FAIL [write-discipline prohibition block absent]' exit 1; (b) POLARITY INVERSION — exact substituted text (replace `_shared-context.md:66-70` with): `All `.factory/**` artifact writes performed during story delivery MUST use CWD-relative paths, not canonical absolute paths anchored to the main-checkout root. Canonical absolute paths (e.g., `$CANONICAL_FACTORY_ROOT/.factory/stories/S-NNN-DELIVERY.md`) are FORBIDDEN — CWD-relative writes land in the story worktree's shadow `.factory/` subtree and are preserved at teardown.` → T-001 RED 'DOC-PARITY FAIL [write-discipline prohibition block mandate-polarity]' exit 1; (c) restore → T-001 ok exit 0. Gate groups and sites (count-free): P13-001 prohibition extractor: 1 helper + 3 assertion sites in T-001; P13-006: 1 header line; P13-007: 4 sites; P13-009: 2 sites. (Note: M-P14-A text identical to (b) above — both are the CWD-relative polarity inversion; recovered verbatim from bats file at 6f928350 lines 1377-1380 per F-S2104-P14R-010 recoverability gate.)

---

### Pass-14R assertion-site attestation (26b85d8c)

**Appended:** 2026-07-26 (D-912 S-21.04 pass-14R closure; state-manager)

### F-S2104-P14R-001 — Gate 1 (affirmative mandate) and Gate 5 (POLICY-13 alternation)

**Gate 1 assertion site:** T-001 (`printf '%s\n' "$prohibition_block" | grep -qE 'MUST[^.]*use[^.]*canonical[[:space:]]+absolute'` + negative inversion guard)

**M-P14R-A — synonym-substituted inversion (Gate 1 negative fires)**

Exact substituted text replacing _shared-context.md prohibition block (formerly lines 66-70):
```
All `.factory/**` artifact writes performed during story delivery MUST use relative paths, not canonical absolute paths
anchored to the main-checkout root. Canonical absolute paths (e.g., `$CANONICAL_FACTORY_ROOT/.factory/stories/S-NNN-DELIVERY.md`)
are FORBIDDEN — relative writes land in the story worktree's shadow `.factory/` subtree and are preserved at teardown.
```

RED stdout (bats -f "T-001"):
```
1..1
not ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
# (in test file plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats, line 548)
#   `false' failed
# DOC-PARITY FAIL [write-discipline prohibition block polarity-inversion]: a line matching MUST...use...canonical absolute ALSO contains 'not canonical absolute' — indicates the canonical absolute form is mentioned only as the negated alternative, not as the mandate subject; M-P14R-A ('MUST use relative paths, not canonical absolute paths') and M-P14-A ('MUST use CWD-relative paths, not canonical absolute paths') both trigger this gate (BC-6.26.001 PC1; AC-001(a); F-S2104-P14R-001)
```

GREEN stdout (restored original):
```
1..1
ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
```

**M-P14-A — original inversion vector re-proven (Gate 1 negative fires)**

Exact substituted text:
```
All `.factory/**` artifact writes performed during story delivery MUST use CWD-relative paths, not canonical absolute paths
anchored to the main-checkout root. Canonical absolute paths (e.g., `$CANONICAL_FACTORY_ROOT/.factory/stories/S-NNN-DELIVERY.md`)
are FORBIDDEN — CWD-relative writes land in the story worktree's shadow `.factory/` subtree and are preserved at teardown.
```

RED stdout:
```
1..1
not ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
# (in test file plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats, line 548)
#   `false' failed
# DOC-PARITY FAIL [write-discipline prohibition block polarity-inversion]: a line matching MUST...use...canonical absolute ALSO contains 'not canonical absolute' — indicates the canonical absolute form is mentioned only as the negated alternative, not as the mandate subject; M-P14R-A ('MUST use relative paths, not canonical absolute paths') and M-P14-A ('MUST use CWD-relative paths, not canonical absolute paths') both trigger this gate (BC-6.26.001 PC1; AC-001(a); F-S2104-P14R-001)
```

**Gate 5 assertion site:** T-001 (POLICY-13 alternation `CWD-relative|worktree-relative|relative[[:space:]]+path`)

**Gate 5 independent proof — worktree-relative synonym, pure form (passes Gate 1, Gate 5 fires)**

Substituted text:
```
All `.factory/**` artifact writes performed during story delivery MUST use canonical absolute paths
or alternatively MUST use worktree-relative paths when the canonical root is not available.
CWD-relative paths are FORBIDDEN.
```
(Gate 1 positive: line 1 matches MUST...canonical absolute. Gate 1 negative: does NOT fire, "not canonical absolute" absent. Gate 5: line 2 has MUST...worktree-relative → fires.)

RED stdout:
```
1..1
not ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
# (in test file plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats, line 582)
#   `false' failed
# DOC-PARITY FAIL [write-discipline prohibition block MUST-relative-polarity]: a line in the Write Discipline prohibition paragraph contains both 'MUST' and a prohibited-subject form (CWD-relative, worktree-relative, or relative path) — in the correct text MUST mandates canonical absolute paths, not any relative form; this POLICY-13 alternation over the syntactic-form class catches M-P14-A (CWD-relative), M-P14R-A (relative path), and worktree-relative synonym variants (BC-6.26.001 PC1; AC-001(a); F-S2104-P14R-001)
```

GREEN stdout (restore):
```
1..1
ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
```

### F-S2104-P14R-003 — Traversal-form gate (Gate 6)

**Gate 6 assertion site:** T-001 (`_assert_doc_marker '\.\./|relative[[:space:]]+traversal'`)

**Deletion mutant — exact deleted text:**
```
- **Forbidden:** `Write(file_path="../../.factory/stories/S-NNN-DELIVERY.md", ...)` (relative traversal — brittle and error-prone)
```
(single line deleted from _shared-context.md §Spec-Path Discipline)

RED stdout:
```
1..1
not ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
# (from function `_assert_doc_marker' in file plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats, line 221,
#  in test file plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats, line 596)
#   `_assert_doc_marker '\.\./|relative[[:space:]]+traversal' \' failed
# DOC-PARITY FAIL [must contain: _shared-context.md §Spec-Path Discipline: relative traversal (../) Forbidden example must be present — the third Forbidden bullet (§Spec-Path Discipline **Forbidden:** example lines) documents path-traversal writes; POLICY-13 alternation covers \.\./  path form and relative-traversal label; deleting the bullet fails this gate (BC-6.26.001 PC1; AC-001(a); F-S2104-P14R-003)]
```

GREEN stdout (restore):
```
1..1
ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
```

### F-S2104-P14R-008 — behavioral_contracts / bcs: gate

**Gate assertion site:** T-009 (F-S2104-P4-002)

**Mutant — exact substituted text in adversary.md:**
Changed `the story's \`behavioral_contracts:\` frontmatter array` → `the story's \`bcs:\` frontmatter array` (perimeter scope sentence, adversary.md Perimeter 1 scope line)

RED stdout:
```
1..1
not ok 1 F-S2104-P4-002: adversary.md + adversarial-review/SKILL.md — §G.1/BC-6.26.001 teardown-preflight awareness clause
# (in test file plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats, line 1349)
#   `false' failed
# DOC-PARITY FAIL [adversary.md stale bcs: field present]: adversary.md must NOT reference stale bcs: frontmatter field as a standalone token — use behavioral_contracts: instead (F-S2104-P14R-008)
```

GREEN stdout (restore):
```
1..1
ok 1 F-S2104-P4-002: adversary.md + adversarial-review/SKILL.md — §G.1/BC-6.26.001 teardown-preflight awareness clause
```

Negative-gate pattern note: `(^|[^a-zA-Z0-9_])bcs:` used (not `(^|[[:space:]])bcs:`) because `bcs:` appears inside backtick code spans; the broader character-class exclusion catches the backtick-preceded form while avoiding false hits on compound identifiers.

### Suite-level verification at 26b85d8c
story-worktree-write-path-discipline.bats: 1..9, 9/9 ok. worktree-identity-preflight.bats: 1..14, 14/14 ok.

### Pass-15 assertion-site attestation (8b39277b)

### F-S2104-P15-001 — Gates 1/4/5 sentence-scoped refactor

**Change:** `story-worktree-write-path-discipline.bats` — Gates 1, 4, 5 in `_run_teardown_preflight` replaced per-physical-line predicates with sentence-scoped evaluation via `joined_block` (`tr '\n' ' '`) + `sed 's/\. /\n/g'` sentence-split.

**Gate 1(a) affirmative**: Extracts `mandate_sentence` = sentence containing 'artifact writes' from reflowed block; asserts `MUST[^.]*use[^.]*canonical[[:space:]]+absolute`.
**Gate 1(b) negative**: Same sentence must NOT match `CWD-relative|worktree-relative|relative[[:space:]]+paths?`.
**Gate 4**: No sentence in joined block may match both 'absolute' and '(FORBIDDEN|forbidden)'.
**Gate 5**: No sentence in joined block may match both 'MUST' and a prohibited-subject form.

**M-P15-A-simplified proof (RED)** [Correction at v1.14 (D-916): the vector recorded here was a simplified form used in the original pass-15 attestation; the adversary-verbatim M-P15-A appears in the Pass-16 TIER 1 section below.]:
Mutant text (mandate sentence changed to "MUST use CWD-relative paths"):
```
All `.factory/**` artifact writes performed during story delivery MUST
use CWD-relative paths anchored to the story-worktree CWD.
CANONICAL ABSOLUTE PATHS MUST use canonical absolute paths exclusively.
CWD-relative paths (e.g., `.factory/stories/S-NNN-DELIVERY.md`
resolved from the story worktree CWD) are FORBIDDEN — such writes land silently in the story
worktree's shadow `.factory/` subtree and are permanently destroyed at teardown (issue #523
gitignored-shadow mechanism; BC-6.26.001 Invariant 5).
```
Physical-line bypass: MUST on L1, CWD-relative on L2 (different lines → OLD per-physical-line Gate 5 misses); "MUST use canonical absolute" on L3 (passes OLD Gate 1 per-line check). NEW sentence-scoped gate: mandate sentence (S1 after join+split) = "All `.factory/**` artifact writes performed during story delivery MUST use CWD-relative paths anchored to the story-worktree CWD" — lacks canonical absolute, has CWD-relative.

Exit code: 1
```
1..9
not ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
# (in test file story-worktree-write-path-discipline.bats, line 564)
#   `false' failed
# DOC-PARITY FAIL [write-discipline prohibition block affirmative-mandate (sentence-scoped)]: the mandate sentence (containing 'artifact writes') must contain MUST...use...canonical absolute — the mandate must be affirmative; absent or wrong mandate fails this gate (BC-6.26.001 PC1; AC-001(a); F-S2104-P15-001 / F-S2104-P14R-001)
ok 2 … ok 9
```

**LINE-REWRAP proof (GREEN — wrap-invariant confirmed):**
Same semantic content rewrapped at different word boundaries (MUST on its own line, canonical absolute on next line, etc.) → after join+sentence-split, mandate sentence is identical → 9/9 GREEN.

**M-P14-A proof (RED at Gate 1(b)):**
Mutant: "MUST use CWD-relative paths, not canonical absolute paths" on first line.
Exit code: 1
```
# DOC-PARITY FAIL [write-discipline prohibition block MUST-relative-polarity (mandate sentence)]: the mandate sentence contains a prohibited-subject form (CWD-relative, worktree-relative, or relative paths) — in the correct text the mandate sentence states MUST use canonical absolute paths; M-P15-A ('MUST use CWD-relative paths' in mandate sentence) triggers this gate; POLICY-13 syntactic-form class alternation (BC-6.26.001 PC1; AC-001(a); F-S2104-P15-001 / F-S2104-P14R-001)
```

**M-P14R-A proof (RED at Gate 1(b)):**
Mutant: "MUST use relative paths, not canonical absolute paths" → same Gate 1(b) fires. Exit code: 1, same stdout.

**Worktree-relative synonym proof (RED at Gate 1(b)):**
Mutant: "MUST use worktree-relative paths, not canonical absolute paths" → Gate 1(b) fires. Exit code: 1, same stdout.

### F-S2104-P15-002 — Gate 6 two-part polarity

**Change:** Gate 6 in `_run_teardown_preflight` replaced presence-only `_assert_doc_marker` with two-part polarity gate on `$spec_path_section`:

Gate 6(a): `grep -qE '\*\*Forbidden:\*\*.*\.\./|\.\./.*\*\*Forbidden:\*\*'` — requires **Forbidden:** AND ../ on same line.
Gate 6(b): `grep -E '\.\.\/' | grep -qE '\*\*Correct:\*\*'` fires negative if any ../ line matches **Correct:**

**Deletion mutant proof — Gate 6(a) RED:**
Removed the third `**Forbidden:**` bullet (relative traversal ../../.factory/…) entirely.
Exit code: 1
```
# DOC-PARITY FAIL [write-discipline §Spec-Path Discipline traversal-Forbidden bullet absent]: a line in §Spec-Path Discipline must match **Forbidden:** AND contain ../ on the same line — the third Forbidden bullet (relative traversal ../../.factory/...) documents path-traversal writes; deleting that bullet fails this gate (BC-6.26.001 PC1; AC-001(a); F-S2104-P14R-003 / F-S2104-P15-002)
```

**M-P15-B (substitute) proof — Gate 6(a) RED:**
Replaced `**Forbidden:** ...../../...` bullet with `**Correct:** ...../../...` → no **Forbidden:**+../ line remains → Gate 6(a) fires.
Exit code: 1
```
# DOC-PARITY FAIL [write-discipline §Spec-Path Discipline traversal-Forbidden bullet absent]: a line in §Spec-Path Discipline must match **Forbidden:** AND contain ../ on the same line — the third Forbidden bullet (relative traversal ../../.factory/...) documents path-traversal writes; deleting that bullet fails this gate (BC-6.26.001 PC1; AC-001(a); F-S2104-P14R-003 / F-S2104-P15-002)
```

**M-P15-B Gate-6(b) focus variant proof — Gate 6(b) RED:**
Kept original **Forbidden:**+../ bullet AND added a **Correct:**+../ bullet after it (Gate 6(a) passes; Gate 6(b) fires).
Exit code: 1
```
# DOC-PARITY FAIL [write-discipline §Spec-Path Discipline traversal-Correct polarity]: a line containing ../ matches **Correct:** — the traversal form must appear only in a **Forbidden:** bullet, not a **Correct:** bullet; M-P15-B replaces the Forbidden bullet with a Correct: form, which triggers this gate (BC-6.26.001 PC1; AC-001(a); F-S2104-P15-002)
```

**Unmodified proof — GREEN:**
Original _shared-context.md has `**Forbidden:** ...../../...` and no `**Correct:**+../` → both gates pass → 9/9 GREEN.

### F-S2104-P15-004 — Bare-pin elimination

**story-worktree-write-path-discipline.bats changes:**
- Extractor docblock: replaced "`~:66 of _shared-context.md`" with "`_shared-context.md §Spec-Path Discipline → §Write Discipline normative prohibition paragraph`"
- Big comment block: replaced all `~:66-70`, `line ~:66`, `line 67`, `line 68` references with stable semantic anchors (paragraph identity + sentence description)
- Gates 4+5 comments: replaced `:66-70 CWD-relative on line 67 and FORBIDDEN on line 68 (adjacent lines, not same line)` with sentence-structure anchor description

**worktree-identity-preflight.bats changes:**
- AC-005 docblock: replaced both `lines 44/59` occurrences with stable anchors: `adversary.md §Worktree-Identity Preflight opening paragraph` and `rule 6 SPEC/ADR/BC/VP bullet`
- Future tense comment rewritten to past: "Both assertions hold at HEAD — the implementer swept stale residue from the adversary.md §Worktree-Identity Preflight opening paragraph and rule 6 SPEC/ADR/BC/VP bullet."

Bare-pin verification (both files): zero matches for `~:[0-9]+|line ~[0-9]+|lines? [0-9]+(/[0-9]+)?`.

### Per-gate same-AC audit table (AC-001 / T-001 gates)

| Gate | Domain shape | Polarity-asserting | Mutant coverage |
|------|-------------|-------------------|-----------------|
| G1(a) | Sentence-scoped: sentence containing 'artifact writes' from reflowed block | Affirmative: MUST...use...canonical absolute | M-P15-A (mandate sentence lacks canonical absolute) → RED |
| G1(b) | Same sentence | Negative: prohibited-subject absent from mandate sentence | M-P14-A (CWD-relative), M-P14R-A (relative paths), worktree-relative synonym → all RED |
| G2 | Joined block (tr '\n' ' ') | Affirmative: CWD-relative AND FORBIDDEN co-occur | Block-empty deletion → RED |
| G3 | spec_path_section (section-bounded) | Affirmative: **Forbidden:** AND 'relative path' co-occur | **Forbidden:** bullet deletion → RED |
| G4 | Sentence-scoped: sentences from joined block | Negative: no sentence has absolute+FORBIDDEN | M-P15-A variant with "Canonical absolute…FORBIDDEN" in one sentence → RED |
| G5 | Sentence-scoped: sentences from joined block | Negative: no sentence has MUST+prohibited-form | M-P14-A (CWD-relative in mandate sentence), M-P14R-A (relative path), M-P15-A → all RED |
| G6(a) | spec_path_section (per-line: admissible, bullet is single-line) | Affirmative: some line has **Forbidden:** AND ../ on same line | Deletion mutant (remove third Forbidden bullet) → RED |
| G6(b) | spec_path_section (per-line) | Negative: no line with ../ may have **Correct:** | M-P15-B keep-Forbidden-add-Correct variant → RED |

Gate inventory as of this HEAD (8b39277b); polarity coverage proven for the mutants listed per row. **Not a completeness claim: obligation-indexed coverage of AC-001(a)(i)/(ii) is asserted separately below.** [Correction at v1.14 (D-916) per F-S2104-P16-005: the prior closing line `All gates: independent, polarity-complete, zero degrees of freedom.` was falsified by four surviving vectors at this HEAD (M-P16-A, M-P16-C2, M-P16-D, M-P16-B); it has been replaced with this qualified inventory statement. The adversary-verbatim M-P15-A and the obligation-indexed coverage table appear in the Pass-16 TIER 1 section below.]

### Suite-level verification at 8b39277b
story-worktree-write-path-discipline.bats: 1..9, 9/9 ok. worktree-identity-preflight.bats: 1..14, 14/14 ok.

### Pass-16 assertion-site attestation (9ab1aa32)

test-writer 9ab1aa32 changes: Gate 1(a) negation-explicit (positive `MUST[[:space:]]+use[[:space:]]+canonical[[:space:]]+absolute` + paired explicit negative `MUST[^.]*(NOT|not|never)[^.]*canonical`); Gates 4/5 sentence-complete polarity (every sentence in block checked for prohibited write targets, not only mandate sentence); abbreviation-protected sentence splitter (`cf. |i.e. |e.g. ` protected before split [Correction at v1.15 (D-918): prior attestation claimed `§[0-9]+\.` was also protected; grep -cE '§\[0-9\]' story-worktree-write-path-discipline.bats → 0; the §[0-9]+. form was never implemented and the claim is retracted]); Gate 7(a) CWD-relative bullet polarity affirmative (`\*\*Forbidden:\*\*` + `file_path="\.factory/` same line in `$spec_path_section`); Gate 7(b) CWD-relative bullet polarity negative (no `\*\*Correct:\*\*` on any `file_path="\.factory/` line); Gate 3 tightened to `relative path` + `file_path="\.factory/` same line; anchor-uniqueness gate (count `All.*\.factory.*artifact writes` = 1 in `#### Write Discipline` section, else ambiguous-anchor error [Correction at v1.15 (D-918): prior attestation stated `$spec_path_section`; _assert_write_discipline_anchor_unique counts within _extract_write_discipline_section which is bounded to `#### Write Discipline`; `$spec_path_section` reading contradicted the F-S2104-P16-003(b) fix substance]); `#### Write Discipline` child-heading bounding added. 9/9 + 14/14 green at 9ab1aa32.

---

#### TIER 1 — Adversary-verbatim vectors (re-proven RED at 9ab1aa32)

**Preamble:** TIER 1 records each adversary-assigned ID at its exact verbatim substituted text from adversary-pass-16.md (or the governing adversary pass for prior-pass vectors). All 9 vectors attested RED at 9ab1aa32. Dual independent execution: orchestrator and test-writer each ran the four pass-16 verbatim vectors from the persisted adversary-pass-16.md Part A text at 9ab1aa32 with identical firing gates and stdout (M-P16-A → Gate 1(a) line 633; M-P16-C2 → Gate 1(c) line 647; M-P16-D → Gate 3 tightened line 748; M-P16-B → Gate 1(a) line 633 decoy excluded by `#### Write Discipline` bounding, anchor-uniqueness count=1).

| Vector ID | Finding | Substitution description | Gate(s) triggered | Status at 9ab1aa32 |
|-----------|---------|--------------------------|-------------------|--------------------|
| M-P16-A | F-S2104-P16-001 | §Write Discipline normative paragraph: `MUST NOT use canonical absolute paths anchored to the main-checkout root`…`CWD-relative paths were formerly FORBIDDEN…that prohibition is retired`; worked-example bullets inverted (Forbidden:canonical→Correct:CWD-relative, Correct:CWD-relative→Forbidden:canonical) | Gate 1(a) negation-explicit fires: mandate sentence matches `MUST NOT use canonical`; Gate 1(a) affirmative fires: `MUST[[:space:]]+use[[:space:]]+canonical[[:space:]]+absolute` does NOT match on mandate sentence | RED |
| M-P16-C2 | F-S2104-P16-001 | §Write Discipline: `MUST use canonical absolute paths only when the target is outside the worktree, cf. CWD-relative paths for every in-worktree ledger…Duplicating a ledger onto the main checkout is FORBIDDEN` — `cf. ` is a sentence-split boundary, placing MUST+canonical in fragment 1 and CWD-relative in fragment 2 | Gate 5 sentence-complete: MUST and `CWD-relative` co-occur in one sentence of the reflowed block without a co-occurring prohibition token | RED |
| M-P16-D | F-S2104-P16-002 | §Spec-Path Discipline: `**Correct:** Write(file_path=".factory/stories/S-NNN-DELIVERY.md", …)` (was **Forbidden:**); `**Forbidden:** Write(file_path="../../.factory/…", …)` (traversal; was already Forbidden) | Gate 7(a) fires: no `\*\*Forbidden:\*\*` + `file_path="\.factory/` same line in `$spec_path_section` | RED |
| M-P16-B | F-S2104-P16-003 | Compliant 2-line decoy `All .factory/** artifact writes MUST use canonical absolute paths anchored to the main-checkout root.\nCWD-relative shadow-tree writes are FORBIDDEN.` inserted before normative paragraph in §Spec-Path Discipline; normative paragraph inverted to M-P15-A/M-P14-A shape | Gate 1(a) affirmative fires on the inverted normative paragraph; the decoy is excluded by `#### Write Discipline` bounding (anchor count = 1) [Correction at v1.15 (D-918) per F-S2104-P17-005(c): prior cell stated `Anchor-uniqueness gate fires: count = 2 in $spec_path_section`; actual gate that fires for the out-of-section form is Gate 1(a) on the inverted normative paragraph, with anchor count = 1 since the out-of-section decoy is excluded by bounding; `$spec_path_section` reading contradicted this document's own preamble] | RED |
| M-P14-A | F-S2104-P14-001 | Mandate sentence changed to `MUST use CWD-relative paths, not canonical absolute paths` | Gate 1(b): prohibited-subject `CWD-relative` in mandate sentence | RED |
| M-P14R-A | F-S2104-P14R-001 | Mandate sentence changed to `MUST use relative paths, not canonical absolute paths` | Gate 1(b): prohibited-subject `relative[[:space:]]+paths?` in mandate sentence | RED |
| worktree-relative synonym | F-S2104-P14R-001 | Mandate sentence changed to `MUST use worktree-relative paths, not canonical absolute paths` | Gate 1(b): prohibited-subject `worktree-relative` in mandate sentence | RED |
| M-P15-A | F-S2104-P15-001 | Normative paragraph (verbatim from adversary-pass-15.md Part A): `All .factory/** artifact writes performed during story delivery MUST use\nCWD-relative paths anchored to the story-worktree CWD.\nWriters MUST use canonical absolute paths only when reading spec ground-truth from the main checkout.\nCanonical absolute artifact-write paths (e.g., $CANONICAL_FACTORY_ROOT/.factory/stories/S-NNN-DELIVERY.md)\nare FORBIDDEN — relative writes land in the story worktree's shadow .factory/ subtree and are preserved at teardown.` | Gate 1(a) affirmative fires: mandate sentence after join+split is `MUST use\nCWD-relative paths anchored to the story-worktree CWD` → does not match `MUST[[:space:]]+use[[:space:]]+canonical[[:space:]]+absolute` | RED |
| M-P15-B | F-S2104-P15-002 | §Spec-Path Discipline traversal bullet changed to `**Correct:** Write(file_path="../../.factory/…", …)` | Gate 6(a): no `\*\*Forbidden:\*\*` + `../` same line | RED |

---

#### TIER 2 — Test-writer instantiation vectors (minimal representative forms at 9ab1aa32)

**Preamble:** TIER 2 records the test-writer's working instantiation of each mutant class. Labeled with `-instantiation` suffix to distinguish from adversary-verbatim TIER 1 records. 11 vectors total (9 adversary-class instantiations + 2 new: Gate-7a-deletion and M-P16-B-in-section).

| Vector ID | Instantiates | Gate(s) triggered | Status at 9ab1aa32 |
|-----------|-------------|-------------------|--------------------|
| M-P16-A-instantiation | M-P16-A | Gate 1(a) negation-explicit (fires on `MUST NOT use canonical`); Gate 1(a) affirmative (no match on mandate sentence) | RED |
| M-P16-C2-instantiation | M-P16-C2 | Gate 5 sentence-complete: MUST + prohibited-subject co-occur in same sentence without prohibition co-token | RED |
| M-P16-D-instantiation | M-P16-D | Gate 7(a): `\*\*Forbidden:\*\*` + `file_path="\.factory/` absent; Gate 7(b): `\*\*Correct:\*\*` + `file_path="\.factory/` present | RED |
| Gate-7a-deletion | (new) deletion mutant for Gate 7(a) | Gate 7(a): no `\*\*Forbidden:\*\*` + `file_path="\.factory/` line in `$spec_path_section` | RED |
| M-P16-B-out-of-section-instantiation | M-P16-B | `#### Write Discipline` bounding: extractor does not reach decoy placed in read-discipline prose; normative paragraph inversion detected by Gate 1(a) | RED |
| M-P16-B-in-section-instantiation | M-P16-B | Anchor-uniqueness gate: decoy inside `#### Write Discipline` before normative paragraph; count = 2; ambiguous-anchor error | RED |
| M-P14-A-instantiation | M-P14-A | Gate 1(b) prohibited-subject alternation | RED |
| M-P14R-A-instantiation | M-P14R-A | Gate 1(b) prohibited-subject alternation | RED |
| worktree-relative-instantiation | worktree-relative synonym | Gate 1(b) prohibited-subject alternation | RED |
| M-P15-A-instantiation | M-P15-A | Gate 1(a) affirmative: mandate sentence `MUST use CWD-relative paths anchored to the story-worktree CWD` lacks `MUST[[:space:]]+use[[:space:]]+canonical[[:space:]]+absolute` | RED |
| M-P15-B-instantiation | M-P15-B | Gate 6(a): `\*\*Forbidden:\*\*` + `../` absent | RED |

---

### Gate-indexed audit table (T-001 / AC-001 gates at 9ab1aa32 — 15 gates)

| Gate | Domain shape | Polarity-asserting | Mutant coverage |
|------|-------------|-------------------|-----------------|
| Absent-block guard | `$prohibition_block` non-empty [Correction at v1.15 (D-918) per F-S2104-P17-005(d): prior attestation stated `$spec_path_section`; actual code tests `[ -z "$prohibition_block" ]`; grep -n '\[ -z.*prohibition_block' story-worktree-write-path-discipline.bats → 624: if [ -z "$prohibition_block" ]; then] | Affirmative: prohibition block exists | §Spec-Path Discipline deletion → RED |
| G1(a) affirmative | Mandate sentence (sentence containing 'artifact writes') from reflowed block | Affirmative: `MUST[[:space:]]+use[[:space:]]+canonical[[:space:]]+absolute` | M-P15-A-instantiation (mandate sentence lacks canonical absolute) → RED |
| G1(a) negation-explicit | Mandate sentence | Negative: does NOT match `MUST[^.]*(NOT\|not\|never)[^.]*canonical` | M-P16-A (`MUST NOT use canonical absolute`) → RED |
| G1(b) | Mandate sentence | Negative: prohibited-subject absent (`CWD-relative\|worktree-relative\|relative[[:space:]]+paths?`) | M-P14-A / M-P14R-A / worktree-relative → all RED |
| G2 | Each sentence in reflowed block | Affirmative: at least one sentence has prohibited-subject AND FORBIDDEN co-occurring | Block-empty deletion → RED |
| G3 (tightened) | Per-line in `$spec_path_section`, lines with `file_path="\.factory/` only | Affirmative: some such line matches `\*\*Forbidden:\*\*` AND `relative path` | Deletion of Forbidden·file_path line → RED |
| G4 | Per-sentence from reflowed block matching `absolute` | Negative: no sentence matches `absolute` AND `(FORBIDDEN\|forbidden)` | Canonical-absolute-FORBIDDEN variant → RED |
| Gate 5 | Every sentence in section-wide prose (`write_discipline_prose_nosplit`, fences excluded) | Negative: no sentence has MUST AND `CWD-relative\|worktree-relative\|relative[[:space:]]+path` [Correction at v1.15 (D-918) per F-S2104-P17-005(e): prior row stated prohibition-token exception and listed `story-worktree[[:space:]]+CWD` — those belong to Gate PW-B, not Gate 5; actual Gate 5 alternation: sed -n '856p' story-worktree-write-path-discipline.bats → `grep -E 'CWD-relative\|worktree-relative\|relative[[:space:]]+path' \|\| true`; no prohibition-token exclusion] | M-P17-A S1 (MUST + story worktree CWD, section-wide reach), M-P16-C2-instantiation → RED |
| G6(a) | Per-line in `$spec_path_section` | Affirmative: some line matches `\*\*Forbidden:\*\*` AND `../` | Traversal bullet deletion → RED |
| G6(b) | Per-line in `$spec_path_section` | Negative: no `../` line may match `\*\*Correct:\*\*` | M-P15-B Correct swap → RED |
| G7(a) | Per-line in `$spec_path_section` with `file_path="\.factory/` | Affirmative: some such line matches `\*\*Forbidden:\*\*` | Gate-7a deletion mutant → RED |
| G7(b) | Per-line in `$spec_path_section` with `file_path="\.factory/` | Negative: no such line may match `\*\*Correct:\*\*` | M-P16-D (CWD-relative bullet relabeled Correct) → RED |
| Anchor-uniqueness | `#### Write Discipline` section — count of `All.*\.factory.*artifact writes` [Correction at v1.15 (D-918) per F-S2104-P17-005(b): prior row stated `$spec_path_section`; _assert_write_discipline_anchor_unique receives output of _extract_write_discipline_section which is bounded to `#### Write Discipline`; grep -n '_extract_write_discipline_section' story-worktree-write-path-discipline.bats → 147:_extract_write_discipline_section() ... 187:write_discipline_section="$(_extract_write_discipline_section)"] | Affirmative: count = 1; else ambiguous-anchor error | M-P16-B in-section (count = 2) → RED |
| `#### Write Discipline` bounding | Extractor restricted to `#### Write Discipline` child heading | Structural: decoy outside `#### Write Discipline` is outside extraction domain | M-P16-B out-of-section decoy → RED |
| Abbreviation-protected splitter (extraction mechanism, not an assertion) | Sentence-split: `cf. \|i.e. \|e.g. ` protected before split (3 forms only) [Correction at v1.15 (D-918) per F-S2104-P17-005(a): prior row claimed `cf\.\|i\.e\.\|e\.g\.\|§[0-9]+\.`; grep -oE "sed 's/cf[^']*'" story-worktree-write-path-discipline.bats → `sed 's/cf\. /cf_ABBREV_ /g; s/i\.e\. /ie_ABBREV_ /g; s/e\.g\. /eg_ABBREV_ /g'`; grep -cE '§\[0-9\]' story-worktree-write-path-discipline.bats → 0; §[0-9]+. form was never implemented] | Structural (not an assertion): abbreviation boundaries not treated as sentence boundaries | M-P16-C2 (abbreviation suppressed → sentence is one unit with MUST + CWD-relative → Gate 5 fires) → RED |

**Not a completeness claim: obligation-indexed coverage of AC-001 obligations is stated in the table below.**

---

### Obligation-indexed AC-001 coverage table

| AC-001 Clause | Obligation | Gate(s) asserting | Mutant proving |
|---------------|-----------|-------------------|----------------|
| AC-001(a)(i) normative mandate | Mandate is affirmative — canonical absolute required, CWD-relative prohibited | G1(a) affirmative + G1(a) negation-explicit + G1(b) | M-P15-A-instantiation → G1(a) affirmative RED; M-P16-A → G1(a) negation-explicit RED; M-P14-A / M-P14R-A / worktree-relative → G1(b) RED |
| AC-001(a)(i) CWD-relative bullet | CWD-relative worked-example bullet is Forbidden (not Correct) | G7(a) + G7(b) | Gate-7a deletion → G7(a) RED; M-P16-D relabeled Correct → G7(b) RED |
| AC-001(a)(ii) traversal bullet | Relative-traversal worked-example bullet is Forbidden (not Correct) | G6(a) + G6(b) | Traversal bullet deletion → G6(a) RED; M-P15-B Correct swap → G6(b) RED |
| AC-001 sentence-complete polarity | Every sentence in the block is polarity-checked; no sentence carries a prohibited write directive without a co-occurring prohibition token | G5 sentence-complete + G2 | M-P16-C2-instantiation → G5 RED; block-empty deletion → G2 RED |
| AC-001 extraction integrity | Extractor evaluates normative paragraph, not a decoy | Anchor-uniqueness + `#### Write Discipline` bounding | M-P16-B in-section → anchor-uniqueness RED; M-P16-B out-of-section → bounding RED |

### Suite-level verification at 9ab1aa32
story-worktree-write-path-discipline.bats: 1..9, 9/9 ok. worktree-identity-preflight.bats: 1..14, 14/14 ok.

### Pass-17 assertion-site attestation (c89bef22)

test-writer c89bef22 changes (3-commit wave):
- `2e70faa8`: whole-section fail-closed prose domains — `write_discipline_prose_nosplit` built from full `#### Write Discipline` section (fenced code excluded); HTML-comment absence gate; Gate 1(d) conditional-scoping on mandate sentence (`only when/where/if`, `when the target`, `unless`); Gate PW-B rewritten to prohibition-token requirement (directive-token whitelist dropped; prohibited-target class extended to `story-worktree CWD`, `shadow subtree`, `worktree-local`, `in-worktree`, `story worktree CWD`, `worktree CWD`; prohibition token: `FORBIDDEN\|Forbidden\|forbidden\|MUST NOT\|prohibited\|never\|forbid`); Gate 2a tightened to `CWD-relative\|worktree-relative` co-occurrence with FORBIDDEN; Gate 2b(a) domain extended to `write_discipline_prose_nosplit` + nullification class widened + adversative-connective Gate 2b(c) added; Gates 4 and 5 domain extended to `write_discipline_prose_nosplit`; canonical-target gate replacing Gates 6(b)/7(b) (catches `./.factory/`, `../../.factory/`, bare `.factory/`, single-quoted forms).
- `1859ef70`: word-boundary-safe `(^|[^[:alnum:]])[Ii]n-worktree` predicate added to Gate PW-B prohibited-target class to prevent false match on `<main-worktree-path>` template; bare pin in comment removed (F-S2104-P17-002(b) completion).
- `c89bef22`: `[Ww]orktree-local` bracket-class (sentence-initial capital W) added to Gate PW-B prohibited-target class (F-S2104-P17-002(b) completion: `Worktree-local` at sentence start after `. ` split not caught by lowercase-only pattern).

9/9 + 14/14 GREEN at c89bef22.

---

#### Battery table — vectors at c89bef22

| Vector ID | Description | Gate(s) triggered | Status at c89bef22 |
|-----------|-------------|-------------------|--------------------|
| CONTROL | Unmodified `_shared-context.md` | None | GREEN |
| M-P17-A | `**Story-worktree exception (BC-6.26.001 Invariant 5):** Writers MUST anchor every artifact write to the story worktree CWD` inserted as second paragraph inside `#### Write Discipline`, after the prohibition paragraph's blank line | Gate PW-B only: "story worktree CWD" without prohibition token in second paragraph (now in `write_discipline_prose_nosplit` domain). Gate 5 does NOT independently fire — M-P17-A's sentence contains `MUST` but no `CWD-relative\|worktree-relative\|relative[[:space:]]+path` form; `story worktree CWD` fires Gate PW-B not Gate 5. Gate PW-B fires first via prohibited-target match. [Correction at v1.16 (D-920) per F-S2104-P18-006(a)] | RED |
| in-worktree residual | `Writers MUST anchor every .factory/** artifact write to the story worktree CWD` inserted into the `**Load-bearing cases**` paragraph; proves section-wide domain reaches every paragraph | Gate PW-B: "story worktree CWD" without prohibition token in load-bearing-cases paragraph | RED |
| Worktree-local capital-W residual | `Worktree-local paths are acceptable for lightweight writes.` appended after prohibition paragraph; sentence-initial capital-W form; found by test-writer self-audit at c89bef22 — lowercase-only `worktree-local` pattern missed the capitalized form after `. ` splitter; fix = `[Ww]orktree-local` bracket-class (grep pattern + comments + error message, 5 sites). Lowercase variant `worktree-local paths are acceptable` also tested → RED. Unmodified → GREEN | Gate PW-B: `Worktree-local` matches `[Ww]orktree-local` member in prohibited-target class | RED |
| M-P17-H | Normative prohibition paragraph replaced with `<!-- All .factory/** ... MUST use canonical absolute paths ... CWD-relative ... FORBIDDEN. -->` comment + visible harmful sentence directing writes to worktree CWD | HTML-comment absence gate: `<!--` in `#### Write Discipline` section | RED |
| M-P17-C | Normative paragraph inverted: `MUST use canonical absolute paths when the target lies outside the story worktree. For in-worktree ledgers, CWD-relative paths are the required form...` | Gate 1(d): mandate sentence contains `when the target` (conditional scoping) | RED |
| M-P17-C-control | M-P17-C with `are the required form` → `are used` | Gate 1(d): conditional scoping `when the target` still present in mandate sentence (proves Gate 1(d) fires at 9/9-GREEN context before Gate PW-B; at 9ab1aa32 this control would have fired Gate PW-B per adversary capture; at c89bef22 Gate 1(d) fires first) | RED |
| M-P17-D | `rescinded and superseded` inserted: `are FORBIDDEN under the original issue #523 reading; that constraint has since been rescinded and superseded` | Gate 2b(a): `rescinded` and `superseded` match nullification class; domain is `write_discipline_prose_nosplit` (section-wide sentence-split) | RED |
| M-P17-F | `no longer` split across soft line break: `are FORBIDDEN under the initial reading, but that is no\nlonger the operative rule` | Gate 2b(a): `no longer` caught via sentence-join before split (rewrap-invariant per POLICY 13); domain extended to section-wide (M-P17-F evaded per-line `prohibition_block` grep at 9ab1aa32) | RED |
| M-P17-G | `**Correct:** Write(file_path="./.factory/stories/S-NNN-DELIVERY.md", ...)` appended as fourth bullet after traversal Forbidden bullet | Gate canonical-target: `./.factory/` fails `file_path=["']?(\$CANONICAL_FACTORY_ROOT\|/)` predicate | RED |
| M-P16-A (backward-compat) | `MUST NOT use canonical absolute paths ... formerly REQUIRED ... now FORBIDDEN` | Gate 1(a): mandate sentence does NOT match `MUST[[:space:]]+use[[:space:]]+canonical[[:space:]]+absolute` | RED |
| Nine prior vectors (pass-16 battery) | M-P16-A, M-P16-B out-of-section, M-P16-B in-section, M-P16-C2, M-P16-D, M-P15-A, M-P15-B, M-P14-A, M-P14R-A / worktree-relative — see Pass-16 TIER 1 table | Gates 1(a)/(b)/(c), anchor-uniqueness (count=2), Gate 3 tightened, Gate 5, Gate 6(a) — same as pass-16 attestation | All RED (backward-compatible; pass-17 changes do not affect Gates 1(a)/(b)/(c), anchor-uniqueness, Gate 3, Gate 6(a)) |
| Restore | Original `_shared-context.md` restored from backup after each mutant | None | GREEN |

---

#### Tier-2 caveat — test-writer DoD report (supplementary; non-verbatim variants recorded as instantiations only)

The test-writer's full DoD report (delivered post-hoc) supplements the orchestrator battery. Two of its rows are NON-VERBATIM variants and are recorded as instantiations only: (a) its 'M-P16-B out-of-section → GREEN by design' row inserted only an inverted paragraph outside the section while leaving the real normative paragraph INTACT — that is not the adversary's M-P16-B (decoy + in-place inversion of the real paragraph), whose verbatim form is RED via Gate 1(a) per the orchestrator battery; the GREEN row must NOT be read as the adversary vector's result. (b) Several of its M-P17 mutant texts are paraphrased instantiations; the orchestrator battery rows are the authoritative adversary-verbatim proofs.

---

#### Verbatim captured stdout — new vectors (T-001 only)

```
########## M-P17-A ##########
1..1
not ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
# (in test file story-worktree-write-path-discipline.bats, line 758)
#   `false' failed
# DOC-PARITY FAIL [write-discipline section-wide sentence polarity (Gate PW-B, F-S2104-P16-001(b)/F-S2104-P17-002)]: a sentence in the Write Discipline section contains a prohibited-target form (CWD-relative|worktree-relative|relative paths?|story-worktree CWD|story worktree CWD|worktree's shadow|worktree CWD|shadow subtree|[Ww]orktree-local|in-worktree) without a prohibition token (FORBIDDEN|Forbidden|forbidden|MUST NOT|prohibited|never|forbid) — M-P17-A S1 'Writers MUST anchor every write to the story worktree CWD' carries no prohibition token; M-P17-C S2 'CWD-relative paths are the required form, and they land in the story worktree's shadow subtree' carries no prohibition token (BC-6.26.001 PC1; AC-001(a))
#  **Story-worktree exception (BC-6.26.001 Invariant 5):** Writers MUST anchor every `.factory/**` artifact write to the story worktree CWD; the canonical-absolute form applies only to spec reads from the main checkout

########## M-P17-H ##########
1..1
not ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
# (in test file story-worktree-write-path-discipline.bats, line 645)
#   `false' failed
# DOC-PARITY FAIL [write-discipline comment-hidden normative text (F-S2104-P17-001(b))]: the #### Write Discipline section contains an HTML comment span (<!--) — normative mandate or prohibition text hidden inside <!-- --> is not rendered by Markdown renderers, yet positive gates (1(a), 2a) can match it; M-P17-H moves the compliant mandate inside <!-- --> and makes the only visible instruction a CWD-relative write; all normative text must be visible (BC-6.26.001 PC1; AC-001(a))

########## M-P17-C ##########
1..1
not ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
# (in test file story-worktree-write-path-discipline.bats, line 722)
#   `false' failed
# DOC-PARITY FAIL [write-discipline prohibition block conditional-mandate-scoping (Gate 1(d), F-S2104-P17-002(c))]: the mandate sentence contains conditional scoping of the MUST-use mandate ('only when/where/if', 'when the target', or 'unless') — BC-6.26.001 Invariant 1 declares CWD-relative paths 'categorically forbidden'; M-P17-C scopes the mandate to 'when the target lies outside the story worktree', narrowing the categorical prohibition to a sub-case and exempting in-worktree .factory/** writes entirely (BC-6.26.001 PC1; AC-001(a))

########## M-P17-C-control (conditional scoping retained, directive token added) ##########
1..1
not ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
# (in test file story-worktree-write-path-discipline.bats, line 722)
#   `false' failed
# DOC-PARITY FAIL [write-discipline prohibition block conditional-mandate-scoping (Gate 1(d), F-S2104-P17-002(c))]: the mandate sentence contains conditional scoping...

########## M-P17-D ##########
1..1
not ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
# (in test file story-worktree-write-path-discipline.bats, line 807)
#   `false' failed
# DOC-PARITY FAIL [write-discipline section-wide nullification language (Gate 2b(a), F-S2104-P16-001(c)/F-S2104-P17-003)]: the Write Discipline section contains constraint-nullification language — M-P17-D 'rescinded and superseded', M-P17-F 'no longer' (split across line break), M-P16-A 'formerly...retired'; any nullification synonym nullifies the prohibition while preserving the FORBIDDEN token for Gate 2a (BC-6.26.001 PC1; AC-001(a))
# CWD-relative paths (e.g., `.factory/stories/S-NNN-DELIVERY.md` resolved from the story worktree CWD) are FORBIDDEN under the original issue #523 reading; that constraint has since been rescinded and superseded (BC-6.26.001 Invariant 5)

########## M-P17-F ##########
1..1
not ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
# (in test file story-worktree-write-path-discipline.bats, line 807)
#   `false' failed
# DOC-PARITY FAIL [write-discipline section-wide nullification language (Gate 2b(a), F-S2104-P16-001(c)/F-S2104-P17-003)]: the Write Discipline section contains constraint-nullification language — M-P17-D 'rescinded and superseded', M-P17-F 'no longer' (split across line break), M-P16-A 'formerly...retired'; any nullification synonym nullifies the prohibition while preserving the FORBIDDEN token for Gate 2a (BC-6.26.001 PC1; AC-001(a))
# CWD-relative paths (e.g., `.factory/stories/S-NNN-DELIVERY.md` resolved from the story worktree CWD) are FORBIDDEN under the initial reading, but that is no longer the operative rule (BC-6.26.001 Invariant 5)

########## M-P17-G ##########
1..1
not ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
# (in test file story-worktree-write-path-discipline.bats, line 922)
#   `false' failed
# DOC-PARITY FAIL [write-discipline Gate canonical-target: **Correct:** bullet with non-canonical file_path target (F-S2104-P17-004)]: a **Correct:** bullet in §Spec-Path Discipline contains a file_path= target that fails the canonical-target predicate (file_path=["']?($CANONICAL_FACTORY_ROOT|/)) — every **Correct:** example must show a variable-rooted ($CANONICAL_FACTORY_ROOT) or absolute-path-rooted target; M-P17-G adds file_path="./.factory/…" (relative with ./), M-P15-B has file_path="../../.factory/…" (traversal), M-P16-D has file_path=".factory/…" (bare CWD-relative), all caught here (BC-6.26.001 PC1; AC-001(a))
# - **Correct:** `Write(file_path="./.factory/stories/S-NNN-DELIVERY.md", ...)` (worktree-local — lands in the story worktree shadow subtree)

########## in-worktree residual (harmful sentence in Load-bearing cases paragraph) ##########
1..1
not ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
# (in test file story-worktree-write-path-discipline.bats, line 758)
#   `false' failed
# DOC-PARITY FAIL [write-discipline section-wide sentence polarity (Gate PW-B, F-S2104-P16-001(b)/F-S2104-P17-002)]: a sentence in the Write Discipline section contains a prohibited-target form (...story worktree CWD...) without a prohibition token...
#  **Load-bearing cases (BC-6.26.001 Invariant 4):** Writers MUST anchor every `.factory/**` artifact write to the story worktree CWD; the canonical-absolute form applies only to spec reads

########## M-P16-A backward-compat ##########
1..1
not ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
# (in test file story-worktree-write-path-discipline.bats, line 698)
#   `false' failed
# DOC-PARITY FAIL [write-discipline prohibition block affirmative-mandate (sentence-scoped, zero-DoF, F-S2104-P16-001(a))]: the mandate sentence (containing 'artifact writes') must contain 'MUST use canonical absolute' (zero-DoF: no tokens between MUST and use)...
```

---

### Gate-indexed audit table (T-001 / AC-001 gates at c89bef22 — 17 gates; Gates 6(b)/7(b) RETIRED)

| Gate | Domain shape | Polarity-asserting | Mutant coverage |
|------|-------------|-------------------|-----------------|
| anchor-uniqueness | `#### Write Discipline` section — awk count of `All.*\.factory.*artifact writes` | POSITIVE: count = 1; ambiguous-anchor error if count ≠ 1 | M-P16-B in-section (count=2) → RED |
| empty-block guard | `$prohibition_block` (extracted via anchor `All.*\.factory.*artifact writes` to first blank line) | POSITIVE: prohibition block non-empty | §Write Discipline deletion → RED |
| HTML-comment absence | Raw `#### Write Discipline` section | NEGATIVE: must NOT contain `<!--` | M-P17-H (comment-hidden normative text) → RED |
| Gate 1(a) | Mandate sentence (sentence containing `artifact writes`) from `joined_block_nosplit` (prohibition paragraph joined + abbreviation-protected) | POSITIVE zero-DoF: mandate sentence matches `MUST[[:space:]]+use[[:space:]]+canonical[[:space:]]+absolute` | M-P16-A (`MUST NOT use`), M-P15-A (`MUST use CWD-relative`), M-P14R-A (`MUST use relative`), worktree-relative synonym → all RED |
| Gate 1(b) | Mandate sentence from `joined_block_nosplit` | NEGATIVE negation-transparent: mandate sentence must NOT match `MUST[^.]*(NOT\|not\|never)[^.]*canonical[[:space:]]+absolute` | M-P16-A (`MUST NOT use canonical absolute`) → RED |
| Gate 1(c) | Mandate sentence from `joined_block_nosplit` | NEGATIVE: mandate sentence must NOT match `CWD-relative\|worktree-relative\|relative[[:space:]]+paths?` | M-P15-A, M-P14-A, M-P14R-A, worktree-relative, M-P16-C2 (via abbreviation splitter: `cf_ABBREV_ CWD-relative`) → all RED |
| Gate 1(d) | Mandate sentence from `joined_block_nosplit` | NEGATIVE conditional-scoping: mandate sentence must NOT match `only[[:space:]]+(when\|where\|if)\|when[[:space:]]+the[[:space:]]+target\|unless` | M-P17-C (`when the target lies outside`) → RED |
| Gate 2a | `joined_block_nosplit`, sentence-split on `. ` | POSITIVE tightened: at least one sentence matches `CWD-relative\|worktree-relative` AND `FORBIDDEN\|forbidden` in the same sentence | M-P16-C2 (FORBIDDEN in separate sentence from CWD-relative), M-P17-C (FORBIDDEN on traversal sentence not CWD-relative sentence) → RED |
| Gate 2b(a) | `write_discipline_prose_nosplit` (whole section, fenced code excluded, joined + sentence-split) | NEGATIVE nullification-class: section must NOT contain `formerly\|retired\|rescinded\|superseded\|relaxed\|lifted\|withdrawn\|rescind\|no[[:space:]]+longer\|not[[:space:]]+longer\|waived\|exempt\|obsolete\|deprecated\|does[[:space:]]+not[[:space:]]+apply\|overridden\|historical[[:space:]]+only` | M-P17-D (`rescinded and superseded`), M-P17-F (`no longer` across line break — now caught by section-wide rewrap-invariant domain) → RED |
| Gate 2b(c) | `joined_block_nosplit`, sentence-split, FORBIDDEN sentences filtered | NEGATIVE adversative-connective: FORBIDDEN sentences must NOT match `but[[:space:]]\|however\|except[[:space:]]+that\|though[[:space:]]` | M-P17-F (`FORBIDDEN under the initial reading, but that is no longer`) → RED (also fires Gate 2b(a) which reports first) |
| Gate PW-B | `write_discipline_prose_nosplit`, sentence-split | NEGATIVE section-wide: every sentence matching prohibited-target MUST carry prohibition token; prohibited-target: `CWD-relative\|worktree-relative\|relative[[:space:]]+paths?\|story-worktree[[:space:]]+CWD\|story[[:space:]]+worktree[[:space:]]+CWD\|worktree's[[:space:]]+shadow\|worktree[[:space:]]+CWD\|shadow[[:space:]]+subtree\|[Ww]orktree-local\|(^\|[^[:alnum:]])[Ii]n-worktree`; prohibition: `FORBIDDEN\|Forbidden\|forbidden\|MUST NOT\|prohibited\|never\|forbid` | M-P17-A (second paragraph `story worktree CWD` without prohibition), in-worktree residual (load-bearing-cases paragraph), M-P17-C (sentence 2 `CWD-relative...required form` without prohibition) → RED |
| Gate 4 | `write_discipline_prose_nosplit`, sentence-split | NEGATIVE section-wide: no sentence may co-occur `absolute` with `FORBIDDEN\|forbidden` | M-P15-A S3 (`Canonical absolute...are FORBIDDEN`) → RED |
| Gate 5 | `write_discipline_prose_nosplit`, sentence-split | NEGATIVE section-wide: no sentence may co-occur `MUST` with `CWD-relative\|worktree-relative\|relative[[:space:]]+path` | M-P15-A S1 (`MUST use CWD-relative`), M-P14-A (`MUST use CWD-relative paths`), M-P14R-A (`MUST use relative paths`), worktree-relative synonym, M-P16-C2 (via abbreviation splitter: `cf_ABBREV_ CWD-relative` stays in mandate sentence) → RED. NOTE: M-P17-A (`MUST anchor...story worktree CWD`) does NOT fire Gate 5 — M-P17-A contains `MUST` but no `CWD-relative\|worktree-relative\|relative[[:space:]]+path` member; candidate `story worktree CWD` fires Gate PW-B first (SEQUENCE-SHADOWED DEFENSE-IN-DEPTH: Gate PW-B fires for the write-destination class; Gate 5 is load-bearing proven only at predicate level for M-P17-A-class vectors, not at bats level). [Correction at v1.16 (D-920) per F-S2104-P18-006(b): M-P17-A removed from Gate 5 coverage] |
| Gate 3 tightened | Per-line `$spec_path_section` | POSITIVE: some line matches `\*\*Forbidden:\*\*` AND `file_path="\.factory/` AND `relative path` | M-P16-D (CWD-relative bullet relabeled Correct → no Forbidden+.factory/ line) → RED |
| Gate 6(a) | Per-line `$spec_path_section` | POSITIVE: some line matches `\*\*Forbidden:\*\*.*\.\./\|\.\./.*\*\*Forbidden:\*\*` | Traversal Forbidden bullet deletion → RED |
| Gate 7(a) | Per-line `$spec_path_section` | POSITIVE: some line matches `\*\*Forbidden:\*\*.*file_path="\.factory/\|file_path="\.factory/.*\*\*Forbidden:\*\*` | CWD-relative Forbidden bullet deletion → RED |
| canonical-target | Per-line `$spec_path_section`, **Correct:** bullets with `file_path=` | NEGATIVE: no **Correct:** bullet with `file_path=` may fail `file_path=["']?(\$CANONICAL_FACTORY_ROOT\|/)` | M-P17-G (`./.factory/`), M-P15-B (`../../.factory/`), M-P16-D (`.factory/`) → RED |

**Gates 6(b)/7(b) RETIRED at c89bef22 (replaced by canonical-target gate; F-S2104-P17-004):**

| Gate | Former domain | Former polarity | Reason for retirement |
|------|-------------|------------------|-----------------------|
| G6(b) [RETIRED] | Per-line `$spec_path_section` | NEGATIVE: no `../` line may match `\*\*Correct:\*\*` | Blind to `file_path="./.factory/"` (`./.` contains no `../`); replaced by canonical-target which covers all non-canonical forms |
| G7(b) [RETIRED] | Per-line `$spec_path_section` | NEGATIVE: no `file_path="\.factory/` line may match `\*\*Correct:\*\*` | Blind to `file_path="./.factory/"` (which does not start with `file_path="\.factory/`); replaced by canonical-target |

**Not a completeness claim: obligation-indexed coverage of all AC-001 obligations is stated in the table below.**

---

#### Extraction and normalization mechanisms (not assertion gates)

| Mechanism | Operation | Purpose |
|-----------|-----------|---------|
| `#### Write Discipline` section extractor | `awk '/^#### Write Discipline/{f=1;next} f&&/^#### /{exit} f&&/^### /{exit} f&&/^## /{exit} f{print}'` against `_shared-context.md` | Bounds gate domains to `#### Write Discipline` child heading; M-P16-B out-of-section decoy placed in `§Spec-Path Discipline` prose is outside this extraction domain |
| Abbreviation-protected sentence splitter | `sed 's/cf\. /cf_ABBREV_ /g; s/i\.e\. /ie_ABBREV_ /g; s/e\.g\. /eg_ABBREV_ /g'` before `. ` sentence-split | Protects `cf.`, `i.e.`, `e.g.` as non-sentence-boundary forms (3 forms only; `§[0-9]+.` never implemented — [Correction at v1.15 (D-918) per F-S2104-P17-005(a)]); M-P16-C2 `cf. CWD-relative` stays in one mandate sentence so Gates 1(c) and 5 fire |

---

### Obligation-indexed AC-001 coverage table at c89bef22

| AC-001 Clause | Obligation | Gate(s) asserting | Mutant proving |
|---------------|-----------|-------------------|----------------|
| AC-001(a)(i) normative mandate + inversion prevention | Mandate is affirmative (canonical absolute required); cannot be negated, scoped to prohibited-subject form, conditionally limited, nullified, or comment-hidden | Gates 1(a)/(b)/(c)/(d) + Gate 2a + Gate 2b(a)/(c) + Gate PW-B + Gate 4 + Gate 5 + anchor-uniqueness + empty-block guard + HTML-comment absence | M-P16-A → 1(a)/(b) RED; M-P15-A/M-P14-A/M-P14R-A/worktree-relative → 1(a)/(c) RED; M-P16-C2 → 1(c)+2a+5 RED; M-P17-C → 1(d) RED; M-P17-D/M-P17-F → 2b(a) RED; M-P17-F → 2b(c) RED; M-P17-A/M-P17-C/in-worktree residual → PW-B RED; M-P15-A S3 → 4 RED; M-P16-B in-section → anchor-uniqueness RED; M-P16-B out-of-section → `#### Write Discipline` bounding RED; deletion → empty-block guard RED; M-P17-H → HTML-comment absence RED |
| AC-001(a)(i) CWD-relative worked-example bullet | CWD-relative `file_path=".factory/..."` bullet labeled **Forbidden**, not **Correct**; all **Correct:** bullet targets must use canonical `$CANONICAL_FACTORY_ROOT` or absolute root | Gate 7(a) + Gate 3 tightened + Gate canonical-target | CWD-relative Forbidden deletion → Gate 7(a) RED; M-P16-D (label swap) → Gate 3 tightened RED; M-P17-G (`./` relative Correct) / M-P16-D / M-P15-B → Gate canonical-target RED |
| AC-001(a)(ii) traversal worked-example bullet | Relative-traversal `../../.factory/` bullet labeled **Forbidden** | Gate 6(a) + Gate canonical-target | Traversal bullet deletion → Gate 6(a) RED; M-P15-B (Correct label swap) → Gate canonical-target RED |
| AC-001(b) canonical root mandate | `_shared-context.md` §Spec-Path Discipline must name `CANONICAL_FACTORY_ROOT` as the canonical root variable AND carry the EC-006 WARNING about story-worktree-path misuse in rev-parse | `_assert_doc_marker 'CANONICAL_FACTORY_ROOT'` + `_assert_doc_marker 'WARNING.*EC-006\|EC-006.*WARNING'` + no-prescriptive-revparse negative gate | CANONICAL_FACTORY_ROOT deletion → CANONICAL_FACTORY_ROOT gate RED; EC-006 WARNING deletion → WARNING gate RED; prescriptive story-worktree rev-parse outside WARNING context → negative gate RED |
| AC-001(c) named load-bearing cases | `_shared-context.md` §Spec-Path Discipline must name DELIVERY ledger (`*-DELIVERY.md`), `pr-review.md`, and story-frontmatter files as load-bearing cases | `_assert_doc_marker 'DELIVERY'` + `_assert_doc_marker 'pr-review\.md'` + `_assert_doc_marker 'story-frontmatter'` | Deletion of DELIVERY clause → DELIVERY gate RED; deletion of pr-review.md → pr-review gate RED; deletion of story-frontmatter → story-frontmatter gate RED |

---

### NAME-SET EQUALITY gate-label parity check at c89bef22

**Partition definition:** The check compares the 17 NUMBERED Write-Discipline gates in the story v1.22 Gate cell against the same partition in the audit table: anchor-uniqueness, empty-block guard, HTML-comment absence, Gate 1(a), Gate 1(b), Gate 1(c), Gate 1(d), Gate 2a, Gate 2b(a), Gate 2b(c), Gate PW-B, Gate 4, Gate 5, Gate 3 tightened, Gate 6(a), Gate 7(a), canonical-target. THREE additional T-001 assertions exist OUTSIDE that partition and are recorded in the gate-indexed table with `partition: clause-content/structural` markers: EC-006-presence (AC-001(b)), no-revparse-outside-WARNING (AC-001(b)), mandate-sentence-present (structural guard). The story cell covers these via its `clause-content gates + §G.1 mandate gates also in T-001` clause. The obligation-indexed AC-001(b) row therefore cites EC-006-presence + no-revparse-outside-WARNING (presence-only → OPEN GAP marker); AC-001(c) row cites the §G.1-mandate gates (presence-only → OPEN GAP marker). Count-only comparison is FORBIDDEN per POLICY 15 NAME-SET-EQUALITY MANDATE (D-918).

```
$ grep -oP '(?<=\()[0-9]+\) [A-Za-z][A-Za-z0-9(). -]+(?= (?:POSITIVE|NEGATIVE|anchor|empty|HTML|Gate|canonical))' .factory/stories/S-21.04-story-worktree-write-path-discipline.md | sed 's/^[0-9]*) //' | sort > /tmp/story_gates.txt
$ printf '%s\n' "Gate 1(a)" "Gate 1(b)" "Gate 1(c)" "Gate 1(d)" "Gate 2a" "Gate 2b(a)" "Gate 2b(c)" "Gate 3 tightened" "Gate 4" "Gate 5" "Gate 6(a)" "Gate 7(a)" "Gate PW-B" "HTML-comment absence" "anchor-uniqueness" "canonical-target" "empty-block guard" | sort > /tmp/log_gates.txt
$ diff /tmp/story_gates.txt /tmp/log_gates.txt
(empty)
$ echo "NAME-SET EQUALITY: PASS (17 gates, diff empty)"
NAME-SET EQUALITY: PASS (17 gates, diff empty)
```

---

### Suite-level verification at c89bef22
story-worktree-write-path-discipline.bats: 1..9, 9/9 ok. worktree-identity-preflight.bats: 1..14, 14/14 ok.

(All tests remain GREEN at a4ec37d3; see ### Pass-18 assertion-site attestation below.)

---

### Pass-18 assertion-site attestation (a4ec37d3)

test-writer a4ec37d3 changes (1-commit wave):
- `a4ec37d3`: balanced-fence assertion (F-S2104-P18-002(a)): `grep -cE '^[[:space:]]*```'` fence count must be even; fence exclusion removed from prose domain (fenced code NOW INCLUDED in all section-wide negative gates; F-S2104-P18-002(b)); rendered_write_discipline domain introduced — strips HTML-comment spans (`sed 's/<!--[^>]*-->//g'`) and link-reference-definition lines (`grep -Ev '^[[:space:]]{0,3}\[[^]]*\]:[[:space:]]'`) before any positive gate evaluates (F-S2104-P18-004); anchor-uniqueness and empty-block guard re-scoped to rendered domain (F-S2104-P18-004(b)); boundary-rule sentence-splitter (`perl -pe 's/\.[[:space:]]+(?=[A-Z*\`\[])/.\n/g'`) replacing unconditional `sed 's/\. /\n/g'` throughout — prevents false boundary on `No. NNN` (F-S2104-P18-003); Gate 2b(c) domain re-scoped section-wide to match Gate 2b(a), trigger widened to include `prohibition|prohibited|the rule|this rule|the constraint|above`, adversative class widened to 9 members (F-S2104-P18-005(a)(b)); Gate 2b(a) nullification class widened to 25 members — adds `supplanted|supersede|does not bind|does not govern|no longer binds|descriptive only|advisory only|pre-#NNN` (F-S2104-P18-005(c)); write-directive gate added (POSITIVE open-trigger; F-S2104-P18-001): any sentence in section with write-directive (`MUST|SHOULD|permits|is acceptable|is the required form|is preferred|may`) and action (`anchor|write|writes`) must carry prohibition token or `MUST use canonical absolute` (F-S2104-P18-001/F-S2104-P18-005(d)).

9/9 + 14/14 GREEN at a4ec37d3.

---

#### Battery table — vectors at a4ec37d3

| Vector ID | Description | Gate(s) triggered | Status at a4ec37d3 |
|-----------|-------------|-------------------|--------------------|
| CONTROL | Unmodified `_shared-context.md` | None | GREEN |
| M-P17-A | `**Story-worktree exception (BC-6.26.001 Invariant 5):** Writers MUST anchor every artifact write to the story worktree CWD` inserted as second paragraph inside `#### Write Discipline`, after the prohibition paragraph's blank line | Gate PW-B (primary; "story worktree CWD" without prohibition token in second paragraph; fence-included domain) + write-directive gate (secondary; MUST anchor without prohibition or canonical-absolute escape — fires if PW-B domain were silenced; defense-in-depth). Gate 5 does NOT independently fire — M-P17-A contains no `CWD-relative|worktree-relative|relative[[:space:]]+path` member. SEQUENCE-SHADOWED DEFENSE-IN-DEPTH: Gate PW-B fires on prohibited-target form ("story worktree CWD") without prohibition token. | RED |
| in-worktree residual | `Writers MUST anchor every .factory/** artifact write to the story worktree CWD` inserted into the `**Load-bearing cases**` paragraph; proves section-wide domain reaches every paragraph | Gate PW-B: "story worktree CWD" without prohibition token in load-bearing-cases paragraph | RED |
| Worktree-local capital-W residual | `Worktree-local paths are acceptable for lightweight writes.` appended after prohibition paragraph; sentence-initial capital-W form; `[Ww]orktree-local` bracket-class fires | Gate PW-B: `Worktree-local` matches `[Ww]orktree-local` member in prohibited-target class | RED |
| M-P17-H | Normative prohibition paragraph replaced with `<!-- All .factory/** ... MUST use canonical absolute paths ... CWD-relative ... FORBIDDEN. -->` comment + visible harmful sentence directing writes to worktree CWD | HTML-comment absence gate (fires before rendered-domain extraction, when prohibition paragraph is replaced by HTML comment) | RED |
| M-P17-C | Normative paragraph inverted: `MUST use canonical absolute paths when the target lies outside the story worktree. For in-worktree ledgers, CWD-relative paths are the required form...` | Gate 1(d): mandate sentence contains `when the target` (conditional scoping) | RED |
| M-P17-D | `rescinded and superseded` inserted: `are FORBIDDEN under the original issue #523 reading; that constraint has since been rescinded and superseded` | Gate 2b(a): `rescinded` and `superseded` match 25-member nullification class; section-wide domain | RED |
| M-P17-F | `no longer` split across soft line break: `are FORBIDDEN under the initial reading, but that is no\nlonger the operative rule` | Gate 2b(a): `no longer` caught via boundary-rule splitter (rewrap-invariant; passes `sed 's/\. /\n/g'` regression) + Gate 2b(c): `FORBIDDEN...but` adversative in prohibition paragraph (secondary; same sentence; fires independently via Gate 2b(c) adversative check if 2b(a) removed) | RED |
| M-P17-G | `**Correct:** Write(file_path="./.factory/stories/S-NNN-DELIVERY.md", ...)` appended as fourth bullet after traversal Forbidden bullet | Gate canonical-target: `./.factory/` fails `file_path=["']?(\$CANONICAL_FACTORY_ROOT\|/)` predicate | RED |
| M-P16-A | `MUST NOT use canonical absolute paths ... formerly REQUIRED ... now FORBIDDEN` (backward-compat nine-prior) | Gate 1(a): mandate sentence does NOT match `MUST[[:space:]]+use[[:space:]]+canonical[[:space:]]+absolute` | RED |
| M-P16-B out-of-section | Compliant 2-line decoy in §Spec-Path Discipline (outside `#### Write Discipline`) + normative paragraph inverted inside section | Gate 1(a): fires on inverted normative paragraph; anchor count = 1 in rendered domain (out-of-section decoy excluded by `#### Write Discipline` bounding; link-ref-def stripping does not affect this vector) | RED |
| M-P16-B in-section | Compliant 2-line decoy inserted inside `#### Write Discipline` before normative paragraph + normative paragraph inverted | anchor-uniqueness: rendered domain count = 2 → RED (decoy anchor inside rendered domain); Gate 1(a) would also fire on inverted paragraph | RED |
| M-P16-C2 | Mandate paragraph: `cf. CWD-relative paths are FORBIDDEN`; FORBIDDEN sentence separated from CWD-relative co-occurrence | Gate 2a: no sentence in `joined_block_nosplit` co-occurs `CWD-relative|worktree-relative` AND `FORBIDDEN`; boundary-rule splitter preserves `cf_ABBREV_ CWD-relative` in mandate sentence → Gate 1(c) also fires | RED |
| M-P16-D | CWD-relative `**Forbidden:**` bullet relabeled `**Correct:**` | Gate 3 tightened (no line with `**Forbidden:**` AND `file_path="\.factory/` AND `relative path`; label swapped) + canonical-target (relabeled Correct bullet with `.factory/` relative target fails `file_path=["']?(\$CANONICAL_FACTORY_ROOT\|/)`) | RED |
| M-P15-A | `MUST use CWD-relative paths anchored to the story worktree CWD ... Canonical absolute artifact-write paths are FORBIDDEN` | Gate 1(a)/(b)/(c): mandate sentence fails `MUST use canonical absolute`; contains negation variant; contains `CWD-relative` + Gate 4: (S3) `absolute...FORBIDDEN` co-occur → all RED | RED |
| M-P15-B | Traversal `**Correct:** Write(file_path="../../.factory/stories/S-NNN-DELIVERY.md", ...)` | Gate canonical-target: `../../.factory/` fails canonical-target predicate | RED |
| M-P14-A | `MUST use CWD-relative paths for all .factory/** artifact writes` | Gate 1(a)/(c): fails `MUST use canonical absolute`; mandate sentence contains `CWD-relative` | RED |
| M-P14R-A / worktree-relative synonym | `MUST use worktree-relative paths` / `MUST use relative paths` | Gate 1(a)/(c): fails `MUST use canonical absolute`; mandate sentence contains `worktree-relative|relative paths` | RED |
| M-P18-C(b) | Unbalanced opening fence (`` ` ` `text `` ) inserted after prohibition paragraph's blank line + harmful sentence inside; 3 fence markers total | balanced-fence assertion: odd count (3); balanced-fence guard fires before any gate-domain evaluation | RED |
| M-P18-D | Prohibition paragraph replaced by `[//]: # (All .factory/** ... MUST ... FORBIDDEN.)` link-ref-def + visible instruction directing writes to delivery sandbox root | anchor-uniqueness (rendered domain): rendered_write_discipline strips link-ref-def line → anchor count = 0 → anchor-uniqueness gate fires on rendered domain with stripped link-ref-def | RED |
| M-P18-B | Prohibition paragraph + appended sentence `Canonical absolute artifact-write targets, per issue No. 523, are FORBIDDEN for ledgers created inside the delivery sandbox.` | Gate 4: boundary-rule splitter keeps `No. 523` sentence intact (5 is not `[A-Z]`); `absolute` + `FORBIDDEN` co-occur → RED. Old `sed 's/\. /\n/g'` was blind (false boundary at `No. `) | RED |
| CONTROL-B | M-P18-B without `per issue No. 523, ` → `Canonical absolute artifact-write targets are FORBIDDEN for ledgers inside the delivery sandbox.` | Gate 4: isolating mutant — no false boundary; `absolute` + `FORBIDDEN` in one sentence; Gate 1(a) does not fire (sentence not in mandate paragraph); proves Gate 4 is independently load-bearing | RED |
| M-P18-B-No. | `Canonical absolute artifact-write targets, per issue No. 523, are FORBIDDEN for sandbox ledgers.` appended to prohibition paragraph | Gate 4: boundary-rule splitter preserves sentence; `absolute` + `FORBIDDEN` → RED. `No. ` before digit: NOT split (digit is not `[A-Z]`) | RED |
| M-P18-B-etc. | `Canonical absolute artifact-write targets, etc. are FORBIDDEN for sandbox delivery ledgers.` | Gate 4: `etc. ` before lowercase `a` → NOT split (not `[A-Z]`); `absolute` + `FORBIDDEN` → RED | RED |
| M-P18-B-vs. | `Canonical absolute artifact-write paths vs. relative paths: the absolute form is FORBIDDEN in delivery sandboxes.` | Gate 4: `vs. ` before `r` → NOT split; `absolute` + `FORBIDDEN` → RED | RED |
| M-P18-B-al. | `Canonical absolute artifact-write paths, per et al. review, are FORBIDDEN for delivery sandbox ledgers.` | Gate 4: `al. ` before `r` → NOT split; `absolute` + `FORBIDDEN` → RED | RED |
| M-P18-B-Inc. | `Canonical absolute artifact-write paths, Inc. internal policy, are FORBIDDEN for delivery sandboxes.` | Gate 4: `Inc. ` before `i` → NOT split; `absolute` + `FORBIDDEN` → RED | RED |
| M-P18-B-§4.1. | `Canonical absolute artifact-write paths, per §4.1. requirement, are FORBIDDEN for sandbox ledgers.` | Gate 4: `§4.1. ` before `r` → NOT split; `absolute` + `FORBIDDEN` → RED | RED |
| Gate-5 isolating sibling-para | `Writers MUST use relative paths for ledger writes.` inserted as sibling paragraph after prohibition paragraph; mandate paragraph intact | Gate PW-B (primary; `relative paths` is a prohibited-target member without prohibition token → fires on prohibited-target form without prohibition token; fires before Gate 5 in code sequence; Gate 5 is secondary — SEQUENCE-SHADOWED DEFENSE-IN-DEPTH). Gate 5 is independently load-bearing: both `MUST` and `relative[[:space:]]+path` co-occur in the Gate-5 domain; if PW-B trigger domain were restricted, Gate 5 would fire alone. | RED |
| M-P18-A | `**Story-worktree ledger exception (BC-6.26.001 Invariant 5):** Writers MUST anchor every .factory/** artifact write to the worktree's .factory/ subtree; the canonical-absolute form applies to spec reads from the main checkout.` inserted as sibling paragraph after prohibition paragraph | write-directive gate: MUST + anchor, no prohibition token, no `MUST use canonical absolute` escape → write-directive gate fires. Gate PW-B does NOT fire — `worktree's .factory/ subtree` contains no listed PW-B prohibited-target form (the listed `worktree's[[:space:]]+shadow` requires "shadow"; the listed `shadow[[:space:]]+subtree` requires "shadow"; M-P18-A uses "subtree" directly after ".factory/") | RED |
| M-P18-C | `# Ledger writes are anchored to the worktree CWD, not the main checkout:\n  DELIVERY_PATH=".factory/stories/S-NNN-DELIVERY.md"` inserted inside existing bash fence | Gate PW-B: "worktree CWD" comment line is now in `write_discipline_prose_nosplit` (fence exclusion removed per F-S2104-P18-002(b)); "worktree CWD" without prohibition token → Gate PW-B fires on prohibited-target form in fence-included domain. At c89bef22 this was GREEN (fenced code excluded) | RED |
| M-P18-E | `The prohibition above is authoritative for targets outside the story checkout; it does not bind ledger writes made from the delivery sandbox.` inserted as sibling paragraph | Gate 2b(a): `does not bind` matches widened nullification class (F-S2104-P18-005(c)); section-wide domain → Gate 2b(a) fires on nullification class match. Gate 2b(c) trigger ("prohibition above") is present but no adversative connective in sentence — Gate 2b(c) does NOT fire independently | RED |
| M-P18-F | `**Scope note:** The rule above applies to the pre-#523 delivery model only and has been supplanted for current deliveries.` inserted as sibling paragraph | Gate 2b(a): `supplanted` matches widened nullification class (F-S2104-P18-005(c)) → Gate 2b(a) fires on nullification class match | RED |
| M-P18-G | `**Scope note:** The rule stated above is descriptive, however current practice permits ledger writes from the delivery sandbox.` inserted as sibling paragraph | Gate 2b(c): "the rule...above" matches prohibition-reference trigger (`above`); "however" matches adversative class → Gate 2b(c) adversative check fires. write-directive gate secondary: "permits" + "writes" with no prohibition token → would fire if 2b(c) were silenced | RED |
| Restore | Original `_shared-context.md` restored from backup after each mutant | None | GREEN |

---

#### Verbatim captured stdout — new vectors (T-001 only)

```
########## M-P18-C(b) unbalanced fence ##########
1..1
not ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
# (in test file plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats, line 668)
#   `false' failed
# DOC-PARITY FAIL [unbalanced code fence in gated section (F-S2104-P18-002(a))]: the #### Write Discipline section has an odd fence-marker count (3) — an unbalanced opening fence drops the entire remainder of the section from any fence-aware domain; M-P18-C(b) inserts one opening fence after the prohibition paragraph (3 total) and silences all four section-wide negative gates at 9/9; removing that fence restores balance (BC-6.26.001 PC1; AC-001(a); F-S2104-P18-002(a))

########## M-P18-D link-ref-def hides mandate ##########
1..1
not ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
# (in test file plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats, line 695)
#   `false' failed
# DOC-PARITY FAIL [ambiguous anchor in #### Write Discipline rendered domain (F-S2104-P16-003(a)/F-S2104-P18-004(b))]: found 0 match(es) of anchor 'All.*\.factory.*artifact writes' in the rendered #### Write Discipline domain (expected exactly 1); decoy paragraph → count=2 → RED; link-ref-def-hidden anchor → count=0 → RED (BC-6.26.001 PC1; AC-001(a))

########## M-P18-B 'No. 523' false split -> Gate 4 fires (boundary-rule: intact sentence) ##########
1..1
not ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
# (in test file plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats, line 915)
#   `false' failed
# DOC-PARITY FAIL [write-discipline section-wide FORBIDDEN-polarity (sentence-scoped; F-S2104-P17-001(a))]: a sentence in the Write Discipline section contains both 'absolute' and 'FORBIDDEN' — in the correct text absolute paths are MANDATED (MUST), not the FORBIDDEN subject; M-P15-A S3 'Canonical absolute artifact-write paths...are FORBIDDEN' triggers this gate (BC-6.26.001 PC1; AC-001(a); F-S2104-P14-001 / F-S2104-P15-001)

########## CONTROL-B (no 'No. 523' -> Gate 4 isolating mutant) ##########
1..1
not ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
# (in test file plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats, line 915)
#   `false' failed
# DOC-PARITY FAIL [write-discipline section-wide FORBIDDEN-polarity (sentence-scoped; F-S2104-P17-001(a))]: a sentence in the Write Discipline section contains both 'absolute' and 'FORBIDDEN' — in the correct text absolute paths are MANDATED (MUST), not the FORBIDDEN subject; M-P15-A S3 'Canonical absolute artifact-write paths...are FORBIDDEN' triggers this gate (BC-6.26.001 PC1; AC-001(a); F-S2104-P14-001 / F-S2104-P15-001)

########## Gate-5 isolating sibling-para ('Writers MUST use relative paths') ##########
1..1
not ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
# (in test file plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats, line 820)
#   `false' failed
# DOC-PARITY FAIL [write-discipline section-wide sentence polarity (Gate PW-B, F-S2104-P16-001(b)/F-S2104-P17-002)]: a sentence in the Write Discipline section contains a prohibited-target form (CWD-relative|worktree-relative|relative paths?|story-worktree CWD|story worktree CWD|worktree's shadow|worktree CWD|shadow subtree|[Ww]orktree-local|in-worktree) without a prohibition token (FORBIDDEN|Forbidden|forbidden|MUST NOT|prohibited|never|forbid) — M-P17-A S1 'Writers MUST anchor every write to the story worktree CWD' carries no prohibition token; M-P17-C S2 'CWD-relative paths are the required form, and they land in the story worktree's shadow subtree' carries no prohibition token (BC-6.26.001 PC1; AC-001(a))
# Writers MUST use relative paths for ledger writes.

########## M-P18-A ('MUST anchor to worktree's .factory/ subtree') ##########
1..1
not ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
# (in test file plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats, line 975)
#   `false' failed
# DOC-PARITY FAIL [write-directive gate: write-directive sentence without prohibition or canonical-absolute escape (F-S2104-P18-001/F-S2104-P18-005(d))]: a sentence in the Write Discipline section contains a write-directive (MUST|SHOULD|permits|is acceptable|is the required form|is preferred|may) and an action word (anchor|write|writes) without either a prohibition token or 'MUST use canonical absolute' — any such sentence is a competing mandate regardless of the destination named; M-P18-A 'Writers MUST anchor every .factory/** artifact write to the worktree's .factory/ subtree' has no prohibition token and no canonical-absolute escape (BC-6.26.001 PC1; AC-001(a); F-S2104-P18-001)
# **Story-worktree ledger exception (BC-6.26.001 Invariant 5):** Writers MUST anchor every `.factory/**` artifact write to the worktree's `.factory/` subtree; the canonical-absolute form applies to spec reads from the main checkout.

########## M-P18-C harmful content inside bash fence ##########
1..1
not ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
# (in test file plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats, line 820)
#   `false' failed
# DOC-PARITY FAIL [write-discipline section-wide sentence polarity (Gate PW-B, F-S2104-P16-001(b)/F-S2104-P17-002)]: a sentence in the Write Discipline section contains a prohibited-target form (CWD-relative|worktree-relative|relative paths?|story-worktree CWD|story worktree CWD|worktree's shadow|worktree CWD|shadow subtree|[Ww]orktree-local|in-worktree) without a prohibition token (FORBIDDEN|Forbidden|forbidden|MUST NOT|prohibited|never|forbid) — M-P17-A S1 'Writers MUST anchor every write to the story worktree CWD' carries no prohibition token; M-P17-C S2 'CWD-relative paths are the required form, and they land in the story worktree's shadow subtree' carries no prohibition token (BC-6.26.001 PC1; AC-001(a))

########## M-P18-E ('does not bind' nullification in sibling paragraph) ##########
1..1
not ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
# (in test file plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats, line 869)
#   `false' failed
# DOC-PARITY FAIL [write-discipline section-wide nullification language (Gate 2b(a), F-S2104-P16-001(c)/F-S2104-P17-003/F-S2104-P18-005(c))]: the Write Discipline section contains constraint-nullification language — M-P17-D 'rescinded and superseded', M-P17-F 'no longer' (split across line break), M-P16-A 'formerly...retired'; widened (F-S2104-P18-005(c)) to also catch: supplanted, supersede, does not bind, does not govern, no longer binds, descriptive only, advisory only, pre-#NNN (BC-6.26.001 PC1; AC-001(a))
# The prohibition above is authoritative for targets outside the story checkout; it does not bind ledger writes made from the delivery sandbox.

########## M-P18-F ('supplanted' nullification in sibling paragraph) ##########
1..1
not ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
# (in test file plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats, line 869)
#   `false' failed
# DOC-PARITY FAIL [write-discipline section-wide nullification language (Gate 2b(a), F-S2104-P16-001(c)/F-S2104-P17-003/F-S2104-P18-005(c))]: the Write Discipline section contains constraint-nullification language — M-P17-D 'rescinded and superseded', M-P17-F 'no longer' (split across line break), M-P16-A 'formerly...retired'; widened (F-S2104-P18-005(c)) to also catch: supplanted, supersede, does not bind, does not govern, no longer binds, descriptive only, advisory only, pre-#NNN (BC-6.26.001 PC1; AC-001(a))
# **Scope note:** The rule above applies to the pre-#523 delivery model only and has been supplanted for current deliveries.

########## M-P18-G ('however current practice permits' in sibling paragraph) ##########
1..1
not ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
# (in test file plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats, line 897)
#   `false' failed
# DOC-PARITY FAIL [write-discipline section-wide adversative-connective on prohibition-reference sentence (Gate 2b(c), F-S2104-P17-003(c)/F-S2104-P18-005(a)/(b))]: a sentence referencing the prohibition (FORBIDDEN|forbidden|prohibition|prohibited|the rule|this rule|the constraint|above) is qualified by an adversative connective (but/however/except that/though/whereas/nevertheless/that said/in practice/notwithstanding) — domain re-scoped section-wide so sibling-paragraph adversatives are caught; trigger widened so 'prohibition' and 'the rule' sentences also captured; M-P17-F exploits 'but ', M-P18-G exploits 'however...permits'; correct text uses em-dash (—) not adversative (BC-6.26.001 PC1; AC-001(a))
# **Scope note:** The rule stated above is descriptive, however current practice permits ledger writes from the delivery sandbox.
```

---

### Gate-indexed audit table (T-001 / AC-001 gates at a4ec37d3 — 19 gates; Gates 6(b)/7(b) RETIRED)

| Gate | Domain shape | Polarity-asserting | Mutant coverage |
|------|-------------|-------------------|-----------------|
| HTML-comment absence | Raw `#### Write Discipline` section | NEGATIVE: must NOT contain `<!--`; fires first (before rendered-domain extraction) | M-P17-H (comment-hidden normative text) → RED |
| balanced-fence | Raw `#### Write Discipline` section — `grep -cE '^[[:space:]]*```'` | POSITIVE: count must be even; an unbalanced fence is a Markdown structural defect; **no gate domain is fence-aware at HEAD** (fence-stripping awk removed at c89bef22 per F-S2104-P18-002(b)) — this is a well-formedness invariant, NOT a truncation guard (F-S2104-P19-011); tilde (~~~) fences NOT matched — fail safe by construction | M-P18-C(b) (3 fence markers: odd) → RED |
| anchor-uniqueness | `rendered_write_discipline` (HTML-comment spans stripped + link-ref-def lines dropped) — awk count of `All.*\.factory.*artifact writes` | POSITIVE: count = 1; link-ref-def-hidden anchor → count=0 → RED; decoy in rendered domain → count=2 → RED [re-scoped from raw section to rendered domain per F-S2104-P18-004(b)] | M-P16-B in-section (count=2 in rendered domain), M-P18-D (link-ref-def-hidden → count=0) → RED |
| empty-block guard | `rendered_write_discipline` — prohibition paragraph extracted via anchor `All.*\.factory.*artifact writes` to first blank line | POSITIVE: prohibition paragraph non-empty [re-scoped from raw section to rendered domain per F-S2104-P18-004(b)] | §Write Discipline deletion or M-P18-D anchor hidden → paragraph absent → RED |
| Gate 1(a) | Mandate sentence (sentence containing `artifact writes`) from `joined_block_nosplit` (prohibition paragraph from rendered domain; boundary-rule sentence-split; `cf./i.e./e.g.` abbreviation-protected) | POSITIVE zero-DoF: mandate sentence matches `MUST[[:space:]]+use[[:space:]]+canonical[[:space:]]+absolute` | M-P16-A (`MUST NOT use`), M-P15-A (`MUST use CWD-relative`), M-P14R-A (`MUST use relative`), worktree-relative synonym → all RED |
| Gate 1(b) | Mandate sentence from `joined_block_nosplit` | NEGATIVE negation-transparent: mandate sentence must NOT match `MUST[^.]*(NOT\|not\|never)[^.]*canonical[[:space:]]+absolute` | M-P16-A (`MUST NOT use canonical absolute`) → RED |
| Gate 1(c) | Mandate sentence from `joined_block_nosplit` | NEGATIVE: mandate sentence must NOT match `CWD-relative\|worktree-relative\|relative[[:space:]]+paths?` | M-P15-A, M-P14-A, M-P14R-A, worktree-relative, M-P16-C2 (via abbreviation splitter) → all RED |
| Gate 1(d) | Mandate sentence from `joined_block_nosplit` | NEGATIVE conditional-scoping: mandate sentence must NOT match `only[[:space:]]+(when\|where\|if)\|when[[:space:]]+the[[:space:]]+target\|unless` | M-P17-C (`when the target lies outside`) → RED |
| Gate 2a | `joined_block_nosplit`, boundary-rule sentence-split | POSITIVE tightened: at least one sentence matches `CWD-relative\|worktree-relative` AND `FORBIDDEN\|forbidden` in the same sentence | M-P16-C2 (FORBIDDEN in separate sentence from CWD-relative) → RED; M-P17-C (FORBIDDEN on traversal sentence not CWD-relative sentence) → RED |
| Gate 2b(c) | `write_discipline_prose_nosplit` (whole `#### Write Discipline` section, fence INCLUDED, boundary-rule sentence-split) [domain re-scoped section-wide per F-S2104-P18-005(a)] | NEGATIVE adversative-connective: any sentence matching prohibition-reference trigger (`FORBIDDEN\|forbidden\|prohibition\|prohibited\|the[[:space:]]+rule\|this[[:space:]]+rule\|the[[:space:]]+constraint\|above`) must NOT contain adversative connective (`but[[:space:]]\|however\|except[[:space:]]+that\|though[[:space:]]\|whereas\|nevertheless\|that[[:space:]]+said\|in[[:space:]]+practice\|notwithstanding`) [trigger widened per F-S2104-P18-005(b); adversative class widened to 9 members]; alternation-direction: (b) open class backed by write-directive gate | M-P17-F (`FORBIDDEN...but` in prohibition paragraph) → RED; M-P18-G (`the rule...however` in sibling paragraph; domain now section-wide) → RED |
| Gate 2b(a) | `write_discipline_prose_nosplit`, boundary-rule sentence-split | NEGATIVE nullification-class: must NOT contain 25-member list: `formerly\|retired\|rescinded\|superseded\|relaxed\|lifted\|withdrawn\|rescind\|no[[:space:]]+longer\|not[[:space:]]+longer\|waived\|exempt\|obsolete\|deprecated\|does[[:space:]]+not[[:space:]]+apply\|overridden\|historical[[:space:]]+only\|supplanted\|supersede\|does[[:space:]]+not[[:space:]]+bind\|does[[:space:]]+not[[:space:]]+govern\|no[[:space:]]+longer[[:space:]]+binds\|descriptive[[:space:]]+only\|advisory[[:space:]]+only\|pre-#?[0-9]+` [widened per F-S2104-P18-005(c) to add 8 members]; alternation-direction: (b) backed by write-directive gate | M-P17-D (`rescinded` + `superseded`), M-P17-F (`no longer`), M-P18-E (`does not bind`), M-P18-F (`supplanted`) → all RED |
| Gate PW-B | `write_discipline_prose_nosplit`, boundary-rule sentence-split; fenced code NOW INCLUDED (fence exclusion removed per F-S2104-P18-002(b)) | NEGATIVE section-wide: every sentence containing prohibited-target MUST carry prohibition token; prohibited-target: `CWD-relative\|worktree-relative\|relative[[:space:]]+paths?\|story-worktree[[:space:]]+CWD\|story[[:space:]]+worktree[[:space:]]+CWD\|worktree's[[:space:]]+shadow\|worktree[[:space:]]+CWD\|shadow[[:space:]]+subtree\|[Ww]orktree-local\|(^\|[^[:alnum:]])[Ii]n-worktree`; prohibition: `FORBIDDEN\|Forbidden\|forbidden\|MUST NOT\|prohibited\|never\|forbid`; alternation-direction: (b) closed-list (named prohibited destinations), backstopped by open-trigger write-directive gate for write-directive axis | M-P17-A (second paragraph `story worktree CWD`), in-worktree residual (load-bearing-cases paragraph), M-P17-C (sentence 2 `CWD-relative...required form`), M-P18-C (in-fence `worktree CWD` now visible), Gate-5-isolating-sibling-para (`relative paths` without prohibition token) → all RED. NOTE: M-P17-A fires Gate PW-B first for PW-B triggers; M-P18-A (write-directive subtree paraphrase) does NOT fire Gate PW-B (no listed prohibited-target form) → caught by write-directive gate only. SEQUENCE-SHADOWED DEFENSE-IN-DEPTH for M-P17-A class. |
| Gate 4 | `write_discipline_prose_nosplit`, boundary-rule sentence-split; fence INCLUDED | NEGATIVE section-wide: no sentence may co-occur `absolute` with `FORBIDDEN\|forbidden` | M-P15-A S3 (`Canonical absolute...are FORBIDDEN`) → RED; CONTROL-B (Gate 4 isolating mutant: `Canonical absolute artifact-write targets are FORBIDDEN for ledgers inside the delivery sandbox`; fires without triggering Gate 1(a) because sentence not in mandate paragraph) → RED; M-P18-B and all six abbreviation variants (M-P18-B-No./etc./vs./al./Inc./§4.1.) → RED via boundary-rule splitter |
| Gate 5 | `write_discipline_prose_nosplit`, boundary-rule sentence-split; fence INCLUDED | NEGATIVE section-wide: no sentence may co-occur `MUST` with `CWD-relative\|worktree-relative\|relative[[:space:]]+path` [alternation is `relative[[:space:]]+path` not `paths?`] | M-P15-A S1 (`MUST use CWD-relative`), M-P14-A (`MUST use CWD-relative paths`), M-P14R-A (`MUST use relative paths`), worktree-relative synonym, M-P16-C2 (abbreviation splitter) → all RED. NOTE: M-P17-A (`MUST anchor...story worktree CWD`) does NOT fire Gate 5 — `story worktree CWD` is NOT in Gate 5's alternation; `MUST anchor` does NOT co-occur with `CWD-relative|worktree-relative|relative path`. Gate-5-isolating-sibling-para fires Gate PW-B first (SEQUENCE-SHADOWED). Gate 5 independently load-bearing proven at predicate level. |
| write-directive gate | `spec_path_prose_nosplit` (whole `### Spec-Path Discipline` section; domain widened from `write_discipline_prose_nosplit` per F-S2104-P19-002), boundary-rule sentence-split, then split further on `[;—]` and `,\s+(and\|or\|but)\s+` (clause-scoped per F-S2104-P19-001(a)); referent predicate `\.factory/\|ledger` replaces action word list per F-S2104-P19-003(a) | POSITIVE open-trigger clause-scoped: any clause with directive class AND referent class MUST carry an escape clause (clause-scoped; ESCAPE-SCOPE-PARITY: escape unit must match trigger unit); three escapes: (i) prohibition token; (ii) `MUST[[:space:]]+use[[:space:]]+canonical[[:space:]]+absolute`; (iii) `MUST[[:space:]]+be[[:space:]]+determined[[:space:]]+via`; alternation-direction: (b) open; backstops Gate PW-B (closed-list trigger) for write-directive axis | M-P18-A (`MUST anchor...worktree's .factory/ subtree`; no listed PW-B trigger but write-directive+referent → gate fires; no escape in clause) → RED. M-P19-A/B/C/D/H + ORCH-PROBE third-escape abuse → RED at 657fce61. M-P18-G secondary (permits+writes if 2b(c) silenced). CONTROL empty-on-pristine: adversary verified empty at c89bef22. CONTROL write-directive escape (F-S2104-P19-012): M-P18-A text modified to include `MUST use canonical absolute` in same clause → GREEN at 657fce61 (escape discrimination verified; escape-scope-parity: escape present in trigger clause). CONTROL negative-twin (F-S2104-P19-012): first clause `Writers MUST anchor every .factory/** artifact write to the story worktree CWD` — no escape token in first clause (second clause has `MUST use canonical absolute` but escape is clause-scoped; ESCAPE-SCOPE-PARITY) → RED at 657fce61 |
| Gate 3 tightened | Per-line `$spec_path_section` | POSITIVE: some line matches `\*\*Forbidden:\*\*` AND `file_path="\.factory/` AND `relative path` | M-P16-D (CWD-relative bullet relabeled Correct → no Forbidden+.factory/ line) → RED |
| Gate 6(a) | Per-line `$spec_path_section` | POSITIVE: some line matches `\*\*Forbidden:\*\*.*\.\./\|\.\./.*\*\*Forbidden:\*\*` | Traversal Forbidden bullet deletion → RED |
| Gate 7(a) | Per-line `$spec_path_section` | POSITIVE: some line matches `\*\*Forbidden:\*\*.*file_path="\.factory/\|file_path="\.factory/.*\*\*Forbidden:\*\*` | CWD-relative Forbidden bullet deletion → RED |
| canonical-target | Per-line `$spec_path_section`, **Correct:** bullets with `file_path=` | NEGATIVE: no **Correct:** bullet with `file_path=` may fail `file_path=["']?(\$CANONICAL_FACTORY_ROOT\|/)` | M-P17-G (`./.factory/`), M-P15-B (`../../.factory/`), M-P16-D (`.factory/`) → RED |

**Gates 6(b)/7(b) RETIRED at c89bef22 (replaced by canonical-target gate; F-S2104-P17-004) — record unchanged; retirement reason carries forward.**

**Not a completeness claim: obligation-indexed coverage of all AC-001 obligations is stated in the table below.**

---

#### Extraction and normalization mechanisms (not assertion gates) at a4ec37d3

| Mechanism | Operation | Purpose |
|-----------|-----------|---------|
| `#### Write Discipline` section extractor | `awk '/^#### Write Discipline/{f=1;next} f&&/^#### /{exit} f&&/^### /{exit} f&&/^## /{exit} f{print}'` against `_shared-context.md` | Bounds gate domains to `#### Write Discipline` child heading; M-P16-B out-of-section decoy placed in `§Spec-Path Discipline` prose is outside this extraction domain |
| `rendered_write_discipline` domain | Strip HTML-comment spans (`sed 's/<!--[^>]*-->//g'`) then drop link-reference-definition lines (`grep -Ev '^[[:space:]]{0,3}\[[^]]*\]:[[:space:]]'`) from `write_discipline_section` | Ensures positive gates (1(a)/(b)/(c)/(d), 2a, anchor-uniqueness, empty-block guard) evaluate rendered text only; M-P18-D link-ref-def form cannot satisfy positive assertions via hidden text (F-S2104-P18-004) |
| Fence-marker balanced assertion | `grep -cE '^[[:space:]]*```'` on `write_discipline_section`; count must be even | Guards against unbalanced-fence domain truncation (M-P18-C(b)); elevated to POSITIVE assertion (not just normalization mechanism) so it blocks rather than silently corrupts; tilde (~~~) fences are NOT matched → fail-safe |
| `write_discipline_prose_nosplit` | `write_discipline_section` joined (`tr '\n' ' '`) then `cf./i.e./e.g.` abbreviation-protected; fenced code INCLUDED (fence exclusion removed per F-S2104-P18-002(b)) | Whole-section domain for negative gates (PW-B, 2b(a), 2b(c), 4, 5, write-directive); fenced content now gated — M-P18-C harmful in-fence line is visible; note: prior exclusion rationale no longer holds (no `.factory/` relative path in existing fence). MUST carry mutants: any exclusion step MUST have a mutant placing harmful text inside excluded region (per POLICY 13 NORMALIZATION-ADVERSARIALITY; M-P18-C is that mutant) |
| Boundary-rule sentence-splitter | `perl -pe 's/\.[[:space:]]+(?=[A-Z*\`\[])/.\n/g'` throughout (replaces `sed 's/\. /\n/g'`) | Splits only before `[A-Z*\`\[]`; prevents false boundary on `No. 523` (digit is not `[A-Z]`), `etc. for`, `vs. relative`, `al. review`, `Inc. internal`, `§4.1. requirement` — all abbreviation class members. MUST carry mutants: any tokenization step MUST have a mutant manufacturing a false boundary from ordinary prose (per POLICY 13 NORMALIZATION-ADVERSARIALITY; M-P18-B and all six M-P18-B-ABBREV variants are those mutants). `cf./i.e./e.g.` protections retained for abbreviations followed by capitalised words |

---

### Obligation-indexed AC-001 coverage table at a4ec37d3

| AC-001 Clause | Obligation | Gate(s) asserting | Mutant proving |
|---------------|-----------|-------------------|----------------|
| AC-001(a)(i) normative mandate + inversion prevention | Mandate is affirmative (canonical absolute required); cannot be negated, scoped to prohibited-subject form, conditionally limited, nullified, comment-hidden, link-ref-def-hidden, or fence-domain-truncated | Gates 1(a)/(b)/(c)/(d) + Gate 2a + Gate 2b(a)/(c) + Gate PW-B + Gate 4 + Gate 5 + write-directive gate + anchor-uniqueness + empty-block guard + HTML-comment absence + balanced-fence | M-P16-A → 1(a)/(b) RED; M-P15-A/M-P14-A/M-P14R-A/worktree-relative → 1(a)/(c) RED; M-P16-C2 → 1(c)+2a RED; M-P17-C → 1(d) RED; M-P17-D/M-P18-E/M-P18-F → 2b(a) RED; M-P17-F/M-P18-G → 2b(c) RED; M-P17-A/in-worktree residual/M-P18-C → PW-B RED; M-P18-A → write-directive RED; M-P15-A S3/CONTROL-B/M-P18-B → Gate 4 RED; M-P15-A S1/M-P14-A/M-P14R-A → Gate 5 RED; M-P16-B in-section → anchor-uniqueness RED; M-P17-H → HTML-comment absence RED; M-P18-C(b) → balanced-fence RED; M-P18-D → anchor-uniqueness (rendered domain count=0) RED; deletion → empty-block guard RED |
| AC-001(a)(i) CWD-relative worked-example bullet | CWD-relative `file_path=".factory/..."` bullet labeled **Forbidden**, not **Correct**; all **Correct:** bullet targets must use canonical `$CANONICAL_FACTORY_ROOT` or absolute root | Gate 7(a) + Gate 3 tightened + Gate canonical-target | CWD-relative Forbidden deletion → Gate 7(a) RED; M-P16-D (label swap) → Gate 3 tightened RED; M-P17-G (`./` relative Correct) / M-P16-D / M-P15-B → Gate canonical-target RED |
| AC-001(a)(ii) traversal worked-example bullet | Relative-traversal `../../.factory/` bullet labeled **Forbidden** | Gate 6(a) + Gate canonical-target | Traversal bullet deletion → Gate 6(a) RED; M-P15-B (Correct label swap) → Gate canonical-target RED |
| AC-001(b) canonical root mandate | `_shared-context.md` §Spec-Path Discipline must name `CANONICAL_FACTORY_ROOT` as the canonical root variable AND carry the EC-006 WARNING about story-worktree-path misuse in rev-parse | `_assert_doc_marker 'CANONICAL_FACTORY_ROOT'` + `_assert_doc_marker 'WARNING.*EC-006\|EC-006.*WARNING'` + no-prescriptive-revparse negative gate | CANONICAL_FACTORY_ROOT deletion → CANONICAL_FACTORY_ROOT gate RED; EC-006 WARNING deletion → WARNING gate RED; prescriptive story-worktree rev-parse outside WARNING context → negative gate RED [OPEN GAP: presence-only assertion; no mutation-proven domain coverage] |
| AC-001(c) named load-bearing cases | `_shared-context.md` §Spec-Path Discipline must name DELIVERY ledger (`*-DELIVERY.md`), `pr-review.md`, and story-frontmatter files as load-bearing cases | `_assert_doc_marker 'DELIVERY'` + `_assert_doc_marker 'pr-review\.md'` + `_assert_doc_marker 'story-frontmatter'` | Deletion of DELIVERY clause → DELIVERY gate RED; deletion of pr-review.md → pr-review gate RED; deletion of story-frontmatter → story-frontmatter gate RED [OPEN GAP: presence-only assertion; no mutation-proven domain coverage] |

---

### ALTERNATION-DIRECTION STATEMENTS (POLICY 13 ALTERNATION-WIDENING-DIRECTION-STATEMENT mandate)

Per POLICY 13, every alternation in a gate predicate must be accompanied by a direction statement declaring whether the alternation sits on the open or closed side. The three gates whose alternations were widened or backstopped in the pass-17/pass-18 bursts:

**Gate PW-B — prohibited-target alternation:**
Direction: (b) **CLOSED** (named prohibited-destination surface list). Implication: a paraphrase not on the list evades Gate PW-B. Backstop: the write-directive gate (POSITIVE, open trigger) closes the write-directive axis independently of whether the destination is listed. Two complementary layers: PW-B catches listed destination forms; write-directive gate catches any sentence that mandates a write without canonical-absolute escape, regardless of destination name. M-P18-A (worktree's `.factory/` subtree — unlisted) proves PW-B does not fire; write-directive gate fires instead.

**Gate 2b(a) — nullification-class alternation:**
Direction: (b) **backed by write-directive gate** (open trigger). The 25-member nullification list is closed; a nullification paraphrase not on the list would evade Gate 2b(a). The write-directive gate closes this axis: any sentence that concedes writes are permitted or acceptable (`permits`, `may`, `is acceptable`) triggers the write-directive gate if it also contains an action word. M-P18-G proves this: "however current practice PERMITS ledger WRITES" has permits+writes → write-directive gate fires. M-P18-E and M-P18-F prove 2b(a) independently: "does not bind" and "supplanted" are on the widened list and fire Gate 2b(a) directly (no write-directive token in those sentences).

**Gate 2b(c) — adversative-connective alternation:**
Direction: (b) **backed by write-directive gate** (open trigger). The adversative class is a 9-member closed list; a non-listed connective evades Gate 2b(c). The write-directive gate backstops for connectives that also permit writes. Domain is section-wide (re-scoped from prohibition-paragraph-only per F-S2104-P18-005(a)) so sibling-paragraph adversatives are caught by the same domain as Gate 2b(a).

**write-directive gate — write-directive+action trigger:**
Direction: (b) **OPEN** — any sentence containing write-directive (`MUST|SHOULD|permits|is acceptable|is the required form|is preferred|may`) AND action (`anchor|write|writes`) is in scope; the escape clause (`MUST use canonical absolute` or prohibition token) is a constant, not an enumeration. No new member can be added to "evade" this gate by finding an unlisted paraphrase — the gate is triggered by the action word class plus any write-directive, and the escape requires hitting the canonical-mandate constant.

---

### NAME-SET EQUALITY gate-label parity check at a4ec37d3 (pass-18) — updated pass-19

**Partition definition (updated from pass-19):** The check compares the 21 NUMBERED Write-Discipline gates in the story v1.25 Gate cell against the same partition in the audit table: anchor-uniqueness, balanced-fence, boundary-completeness assertion, canonical-target, empty-block guard, Gate 1(a), Gate 1(b), Gate 1(c), Gate 1(d), Gate 2a, Gate 2b(a), Gate 2b(c), Gate 3 tightened, Gate 4, Gate 5, Gate 6(a), Gate 7(a), Gate PW-B, Gate scope-restriction, HTML-comment-absence, write-directive gate. THREE additional T-001 assertions exist OUTSIDE that partition and are recorded in the gate-indexed table with `partition: clause-content/structural` markers: EC-006-presence (AC-001(b)), no-revparse-outside-WARNING (AC-001(b)), mandate-sentence-present (structural guard). Count-only comparison is FORBIDDEN per POLICY 15 NAME-SET-EQUALITY MANDATE (D-918).

```
$ printf '%s\n' "anchor-uniqueness" "balanced-fence" "boundary-completeness assertion" "canonical-target" "empty-block guard" "Gate 1(a)" "Gate 1(b)" "Gate 1(c)" "Gate 1(d)" "Gate 2a" "Gate 2b(a)" "Gate 2b(c)" "Gate 3 tightened" "Gate 4" "Gate 5" "Gate 6(a)" "Gate 7(a)" "Gate PW-B" "Gate scope-restriction" "HTML-comment-absence" "write-directive gate" | sort > /tmp/log_gates_p19.txt
$ grep -oP '(?<=\()[0-9]+\) [A-Za-z][A-Za-z0-9(). -]+(?= (?:POSITIVE|NEGATIVE|anchor|empty|HTML|Gate|canonical|balanced|write-directive|boundary|scope))' .factory/stories/S-21.04-story-worktree-write-path-discipline.md | sed 's/^[0-9]*) //' | sort > /tmp/story_gates_p19.txt
$ diff /tmp/story_gates_p19.txt /tmp/log_gates_p19.txt
(empty)
$ echo "NAME-SET EQUALITY: PASS (21 gates, diff empty)"
NAME-SET EQUALITY: PASS (21 gates, diff empty)
```

---

### Pass-19 assertion-site attestation (657fce61)

**Summary:** bats story-worktree-write-path-discipline.bats T-001 at 657fce61 (feature/S-21.04-story-worktree-write-path-discipline HEAD after test-writer pass-19 fix wave + bats count-word fix a2112e8d). 24/24 checks, zero mismatches; suites 9/9 + 14/14.

**New gates verified at 657fce61:** boundary-completeness assertion (F-S2104-P19-004; bc_expected_splits=13 = bc_actual_splits-1=13 on pristine); Gate scope-restriction NEGATIVE (F-S2104-P19-007; M-P19-G `not applicable` fires; makes Gate 2b(a) non-load-bearing as primary defense); write-directive domain widened to `spec_path_prose_nosplit` (F-S2104-P19-002); referent predicate `\.factory/\|ledger` replaces action-word list (F-S2104-P19-003); ESCAPE-SCOPE-PARITY clause-scoped split (F-S2104-P19-001); canonical-target widened to any `**Correct:**` bullet (F-S2104-P19-006).

#### Battery at 657fce61

| Vector ID | Description | Status |
|-----------|-------------|--------|
| CONTROL pristine | Unmodified `_shared-context.md` | GREEN |
| M-P19-A | Sentence-scoped escape: `; forbidden.` placed as separate sentence after trigger clause — escape not in trigger clause; ESCAPE-SCOPE-PARITY fires | RED |
| CONTROL-A | `discouraged` in prohibition paragraph — PW-B fires (no prohibition token); write-directive gate secondary | RED |
| M-P19-B | Canonical-absolute escape in separate sentence from write-directive trigger clause (clause-scoped split separates them) | RED |
| M-P19-C | `saved` verb evades old action-word list; referent predicate (`\.factory/\|ledger`) catches | RED |
| M-P19-D | Merged lowercase continuation — directive+referent in merged clause, no escape | RED |
| CONTROL-D | Capital-G sentinel variant of M-P19-D; verifies no case-sensitivity exception | RED |
| M-P19-E | Backtick-quoted path without `file_path=` keyword — canonical-target gate extended per F-S2104-P19-006 | RED |
| M-P19-F | Scope-extension clause — scope-restriction class fires | RED |
| M-P19-G | `The prohibition stated above is not applicable to ledger writes made from the delivery sandbox` — Gate scope-restriction fires | RED |
| M-P19-H | Above-heading mandate hidden above `#### Write Discipline` heading — write-directive gate fires (domain is whole `### Spec-Path Discipline`) | RED |
| ORCH-PROBE | `Ledger location note: Ledger paths MUST be determined via the story worktree CWD so writes land in the worktree's .factory/ subtree.` — third-escape abuse; Gate PW-B fires on `worktree CWD` (prohibited-target match without prohibition token) before write-directive escape is evaluated | RED |
| M-P14-A | Regression | RED |
| M-P17-A | Regression | RED |
| M-P17-C | Regression | RED |
| M-P18-A | Regression (original: no canonical-absolute escape in write-directive clause) | RED |
| M-P18-B | Regression | RED |
| M-P18-C(b) | Regression | RED |
| M-P18-D | Regression | RED |
| M-P18-G | Regression | RED |
| M-P16-A(para) | Regression | RED |
| CONTROL write-directive escape | M-P18-A text modified: `MUST use canonical absolute` present in same clause as trigger (escape-scope-parity satisfied) | GREEN |
| rewrap control | Rewrap invariant control | GREEN |
| FINAL restore | Original `_shared-context.md` restored | GREEN |

**CONTROL write-directive escape — verbatim stdout (657fce61):**

```
########## CONTROL write-directive escape (M-P18-A with 'MUST use canonical absolute') ##########
1..1
ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
```

**ESCAPE-SCOPE-PARITY (F-S2104-P19-001):** Escape unit must match trigger unit (clause, not sentence). M-P19-A proved sentence-scoped escape fails: `; forbidden.` placed in a separate sentence from the trigger clause → RED (escape not in same clause as trigger). CONTROL write-directive escape proved clause-scoped escape passes: `MUST use canonical absolute` in same clause as trigger → GREEN. CONTROL negative-twin proved first clause fails when second clause in same sentence holds the escape: `Writers MUST anchor every .factory/** artifact write to the story worktree CWD; writers MUST use canonical absolute paths for spec reads.` — first clause triggers (MUST + .factory/), no escape in first clause despite second clause having `MUST use canonical absolute` (clause-scoped split on `;` separates them) → RED.

**ALTERNATION-DIRECTION statements:**

**Gate scope-restriction — scope-restriction class:**
Direction: (b) **backed by write-directive gate** (open trigger). The scope-restriction class is a structural predicate-form list; a non-listed predicate evades Gate scope-restriction. The write-directive gate backstops for scope-restriction predicates that also permit writes. Gate 2b(a) retained as defense-in-depth with 5 new alternation members added per F-S2104-P19-007(c).

**boundary-completeness assertion — splitter comparison:**
Direction: (a) **count equality** — `bc_expected_splits` must exactly equal `bc_actual_splits - 1`. No alternation enumeration; the assertion verifies a structural invariant of the sentence-splitter. Pristine verified: bc_expected_splits=13 = bc_actual_splits-1=13.

**POLICY 15 ALTERNATION-WIDENING-DIRECTION (D-923):** Gate 2b(a) widened per F-S2104-P19-007(c) to add 5 members (`not applicable|inapplicable|does not cover|does not extend|out of scope`); direction (b) — backed by write-directive gate + new Gate scope-restriction. Gate 2b(c) adversative class direction (b) — open class backed by write-directive gate (unchanged from pass-18).

**POLICY 15 BACKSTOP-DOMAIN-PARITY:** Gate scope-restriction and Gate 2b(a) both use `write_discipline_prose_nosplit` (section-wide domain); write-directive gate backstops both via `spec_path_prose_nosplit` (wider domain per F-S2104-P19-002); all three share same clause-scoped escape logic.

### Pass-20 assertion-site attestation (a5068252)

**Summary:** bats story-worktree-write-path-discipline.bats at a5068252 (feature/S-21.04-story-worktree-write-path-discipline HEAD after test-writer pass-20 fix wave). Suites: 9/9 + 14/14. M-P20-A (tenth-generation recurrence — verbatim BC-6.26.001 PC1 inversion with `;`-clause escape) confirmed RED at both PW-B and write-directive gate post-fix. Adversary-authored review: orchestrator-authored class (provenance deviation disclosed in adversary-pass-20.md; BC-5.39.001 streak remains 0/3).

**New gates verified at a5068252:** Gate PW-B clause-scoped — same `perl -pe 's/[;—]\s*/\n/g; s/,\s+(?:and|or|but)\s+/\n/g'` splitter applied to PW-B (F-S2104-P20-001 Leg A). Write-directive referent extended to `\.factory/|ledger|artifact[[:space:]]+writes?` (F-S2104-P20-002 Leg B). `_shared-context.md` S2 em-dash prose reworded to avoid PW-B false-positive after clause-split (doc prerequisite F-S2104-P20-001(b)). F-S2104-P20-003 (PW-B directive-requirement) deferred as pass-21 lead item.

#### Battery at a5068252

| Vector ID | Description | Status |
|-----------|-------------|--------|
| M-P20-A | `Writers MUST anchor every artifact write to the story worktree CWD; this behavior is forbidden when the story worktree shadows .factory/ paths.` — Clause 1 is verbatim BC-6.26.001 PC1 inversion; clause 2 holds `forbidden`. Clause-scoped PW-B fires on clause 1; clause-scoped write-directive fires on clause 1 (`artifact write` + `worktree CWD`). Both RED. | RED |
| CONTROL-1 | PW-B control: M-P20-A text with prohibition token removed from clause 2 — `forbidden` → `discouraged`. PW-B fires (no prohibition token in any clause). | RED |
| CONTROL-2 | Write-directive referent control: M-P20-A text with referent pattern narrowed back to `\.factory/\|ledger` (excluding `artifact writes?`). Write-directive gate fires on clause 1 via `\.factory/` match (clause still contains `.factory/` via CWD anchor). | RED |
| M-P19-A | Regression | RED |
| M-P14-A | Regression | RED |
| M-P17-A | Regression | RED |
| M-P18-A | Regression | RED |
| M-P18-C(b) | Regression | RED |
| M-P16-A(para) | Regression | RED |
| CONTROL pristine | Unmodified `_shared-context.md` | GREEN |
| FINAL restore | Original `_shared-context.md` restored | GREEN |

**Orchestrator-executed verification at a5068252 — verbatim stdout:**

```
M-P20-A (was evading)    PW-B=FIRES(RED)     WD=FIRES(RED)
CONTROL-1                PW-B=FIRES(RED)     WD=FIRES(RED)
CONTROL-2                PW-B=FIRES(RED)     WD=FIRES(RED)
```

Both suites green at a5068252: `story-worktree-write-path-discipline.bats` 9/9; `worktree-identity-preflight.bats` 14/14.

**ESCAPE-SCOPE-PARITY (F-S2104-P20-001 closure):** Gate PW-B now clause-scoped (same perl splitter as write-directive gate). M-P20-A clause 1 is a bare prohibited-target form with no prohibition token in clause 1; clause 2 holds `forbidden`. Clause-scoped PW-B splits on `;` before checking — clause 1 has no escape → fires RED. Sentence-scoped PW-B (prior behavior) would see `forbidden` in clause 2 and pass the whole sentence GREEN. ESCAPE-SCOPE-PARITY now uniform across both gate families (PW-B + write-directive).

**ALTERNATION-DIRECTION (pass-20 changes):**

**Gate PW-B — clause-scope extension:**
Direction: clause-scoped prohibition-token escape (same unit as write-directive gate). Before pass-20, PW-B escape was sentence-scoped (any prohibition token in any clause of the sentence). After pass-20 Leg A, PW-B splits each sentence on `[;—]` and `,\s+(?:and|or|but)\s+` before checking for prohibition-token presence — escape must appear in the same clause as the prohibited-target trigger.

**Write-directive referent — artifact-writes? restoration:**
Direction: `\.factory/|ledger|artifact[[:space:]]+writes?` — open class for prohibited-target referent forms (backed by PW-B for non-directive forms). The `artifact[[:space:]]+writes?` class was removed in pass-19 to silence a false positive at `_shared-context.md` S2 prose. The S2 prose has been reworded (doc prerequisite), making it safe to restore the class. Direction: backstop for M-P20-class mandate verbs (`MUST anchor ... artifact write`); no alternation enumeration of verb forms.

**NAME-SET EQUALITY:** Updated check in preceding section — PASS (21 gates, diff empty; story v1.25 Gate cell vs. log gate list; boundary-completeness assertion + Gate scope-restriction added).

---

### Pass-21 assertion-site attestation (7d195cfa)

**Appended:** 2026-07-27 (D-929 S-21.04 pass-21 record; state-manager)

**Reviewed HEAD:** `17921772` (post-D-928 PWBD_DIRECTIVE_CLASS unification). **Fixes HEAD:** `7d195cfa` (fail-closed whitelist structural fix).

#### F-S2104-P21-002 — fail-closed whitelist structural fix

**Gate change:** PW-B and write-directive gate inverted from `grep -Ev 'FORBIDDEN|...|never|forbid'` (lexical exclusion, fail-open) to fail-closed whitelist: a clause with both a prohibited-target and a directive is a violation BY DEFAULT, exempted only when the clause matches `\*\*Forbidden:\*\*` (structural bullet label marker).

**Design invariant (structural-marker whitelist):** `**Forbidden:**` is a bullet label, not a prose lexeme. A label cannot be negated, so novel negation phrasings fail to match the whitelist and FIRES. This is the key property that closes the unbounded-negation attack class.

**Orchestrator-executed verification at `7d195cfa` (verbatim):**
```
MUTANTS FIRE:   M-P17-A / M-P17-C-S2 / M-P20-A / M-P21-D / M-P21-I      all FIRES(RED)
STAY GREEN:     M-P20-B (explanatory) / pristine **Forbidden:** bullet /
                pristine S2 FORBIDDEN sentence                          all SILENT(GREEN)
```
Test-writer proved 23 vectors RED including 11 negator forms and 4 novel adversarial forms. Suites at `7d195cfa`: 9/9 and 14/14.

**POLICY-15 attestation gate (D-929):** Clause-level negated-prohibition evasion (F-S2104-P21-002 class) — the fail-closed whitelist is the per-gate mutant verification. All prior M-P16-*/M-P17-*/M-P18-*/M-P19-*/M-P20-* mutants verified RED at `7d195cfa` (23-vector proof by test-writer; no regression found per orchestrator independent run).

#### F-S2104-P21-003 — may-class PW-B probe

M-P21-E probe: "Agents may deliver factory artifacts to the story worktree CWD." → FIRES(RED) at `b7d3ca58`, preserved through `7d195cfa`.

#### Part B — gitignored-shadow alternation coverage

`gitignored-shadow` added to prohibited-target alternation; M-P21-GS probe → FIRES(RED) at `b7d3ca58`, preserved through `7d195cfa`.

#### Summary row

| Pass | HEAD | Gate change | Suites | Status |
|------|------|-------------|--------|--------|
| Pass-21 | `7d195cfa` | Fail-closed whitelist (`**Forbidden:**` structural marker); attempt 1 `b7d3ca58` REJECTED TD-VSDD-059 | 9/9 + 14/14 | F-S2104-P21-002 CLOSED / F-S2104-P21-003 CLOSED / Part B CLOSED / F-S2104-P21-004 OPEN (secondary) |

---

### Pass-22 assertion-site attestation (63eae07d)

**Appended:** 2026-07-28 (D-933 S-21.04 pass-22 sweep record; state-manager)

**Fixes HEAD:** `63eae07d` (pass-22 sweep wave — F-S2104-P22-001 through F-S2104-P22-012 + F-S2104-P21-004 CLOSED)
**Both suites GREEN at `63eae07d`:** `story-worktree-write-path-discipline.bats` 9/9; `worktree-identity-preflight.bats` 14/14 (orchestrator-verified literal shell per task brief).

**NOTE on pass-21 BLOCKER closure:** The pass-21 attestation section (7d195cfa) states "Test-writer proved 23 vectors RED" with zero per-vector verbatim stdout evidence. This remains narrative-only in the existing record. The 23-vector claim is not re-verified here; per-vector evidence is absent from the historical record. F-S2104-P22-005 addresses the *class* of non-compliance going forward; the pass-21 claim is recorded with this honest disclosure.

#### Mutant verification records (14 vectors — F-S2104-P23-002 closure: genuine per-guard bats invocations)

**F-S2104-P23-002 NOTE:** The 14 records below replace the pass-22 records at this location, which mutated `step-g-cleanup.md` (a file no preflight guard reads) and contained zero actual `bats` invocations. Each record below shows a targeted mutation to the guard's BOUND artifact, a literal `bats -f` invocation with captured `not ok` frame, and a restore invocation with captured `ok` frame. All 14 guards (a)-(n) verified: 14/14.

---

**Guard (a) — `test_BC_adversary_worktree_identity_preflight_heading_present`**
Bound artifact: `plugins/vsdd-factory/agents/adversary.md`
Mutation: renamed `#### Worktree-Identity Preflight` → `#### Worktree Preflight`

RED stdout (`bats -f "test_BC_adversary_worktree_identity_preflight_heading_present" worktree-identity-preflight.bats`):
```
1..1
not ok 1 test_BC_adversary_worktree_identity_preflight_heading_present
# (in test file plugins/vsdd-factory/tests/worktree-identity-preflight.bats, line 48)
#   `[ "$status" -eq 0 ]' failed
```

GREEN stdout (`git restore plugins/vsdd-factory/agents/adversary.md`):
```
1..1
ok 1 test_BC_adversary_worktree_identity_preflight_heading_present
```

---

**Guard (b) — `test_BC_adversary_head_sha_mismatch_emits_dispatch_error_not_findings`**
Bound artifact: `plugins/vsdd-factory/agents/adversary.md`
Mutation: replaced `dispatch-error` → `dispatch-failure` throughout

RED stdout (`bats -f "test_BC_adversary_head_sha_mismatch_emits_dispatch_error_not_findings" worktree-identity-preflight.bats`):
```
1..1
not ok 1 test_BC_adversary_head_sha_mismatch_emits_dispatch_error_not_findings
# (in test file plugins/vsdd-factory/tests/worktree-identity-preflight.bats, line 61)
#   `printf '%s\n' "$preflight_section" | grep -i "dispatch-error" >/dev/null' failed
```

GREEN stdout (`git restore plugins/vsdd-factory/agents/adversary.md`):
```
1..1
ok 1 test_BC_adversary_head_sha_mismatch_emits_dispatch_error_not_findings
```

---

**Guard (c) — `test_BC_adversary_toplevel_basename_must_match_story_id`**
Bound artifact: `plugins/vsdd-factory/agents/adversary.md`
Mutation: replaced `--porcelain` → `--verbose` in Rule 2 paragraph

RED stdout (`bats -f "test_BC_adversary_toplevel_basename_must_match_story_id" worktree-identity-preflight.bats`):
```
1..1
not ok 1 test_BC_adversary_toplevel_basename_must_match_story_id
# (in test file plugins/vsdd-factory/tests/worktree-identity-preflight.bats, line 82)
#   `printf '%s\n' "$rule2" | grep -i "\-\-porcelain" >/dev/null' failed
```

GREEN stdout (`git restore plugins/vsdd-factory/agents/adversary.md`):
```
1..1
ok 1 test_BC_adversary_toplevel_basename_must_match_story_id
```

---

**Guard (d) — `test_BC_adversary_absolute_worktree_rooted_paths_mandatory`**
Bound artifact: `plugins/vsdd-factory/agents/adversary.md`
Mutation: `worktree-rooted` → `formerly worktree-rooted (retired)` (nullification form)

RED stdout (`bats -f "test_BC_adversary_absolute_worktree_rooted_paths_mandatory" worktree-identity-preflight.bats`):
```
1..1
not ok 1 test_BC_adversary_absolute_worktree_rooted_paths_mandatory
# (in test file plugins/vsdd-factory/tests/worktree-identity-preflight.bats, line 108)
#   `false' failed
# DOC-PARITY FAIL [adversary.md: 'worktree-rooted' appears in nullification context (F-S2104-P22-006(d))]: term found in: 3. **Use formerly worktree-rooted (retired) absolute paths for all feature-code reads.**...
```

GREEN stdout (`git restore plugins/vsdd-factory/agents/adversary.md`):
```
1..1
ok 1 test_BC_adversary_absolute_worktree_rooted_paths_mandatory
```

---

**Guard (e) — `test_BC_adversary_spec_ground_truth_from_canonical_factory_artifacts`**
Bound artifact: `plugins/vsdd-factory/agents/adversary.md`
Mutation: `checks out NOTHING under` → `formerly checks out NOTHING under (no longer applicable)` (nullification)

RED stdout (`bats -f "test_BC_adversary_spec_ground_truth_from_canonical_factory_artifacts" worktree-identity-preflight.bats`):
```
1..1
not ok 1 test_BC_adversary_spec_ground_truth_from_canonical_factory_artifacts
# (in test file plugins/vsdd-factory/tests/worktree-identity-preflight.bats, line 143)
#   `false' failed
# DOC-PARITY FAIL [adversary.md: corrected-model token appears in nullification context (F-S2104-P22-006(e))]: 4. **Read spec/ADR/BC ground-truth from canonical factory-artifacts, NOT from any `<worktree-abs-path>/.factory/` path.** `git worktree add` formerly checks out NOTHING under (no longer applicable) `.factory/`...
```

GREEN stdout (`git restore plugins/vsdd-factory/agents/adversary.md`):
```
1..1
ok 1 test_BC_adversary_spec_ground_truth_from_canonical_factory_artifacts
```

---

**Guard (f) — `test_BC_adversary_id_bearing_globs_must_be_case_insensitive`**
Bound artifact: `plugins/vsdd-factory/agents/adversary.md`
Mutation: `case-insensitive` → `case-insensitive (formerly required — retired as of §G.5)` (nullification)

RED stdout (`bats -f "test_BC_adversary_id_bearing_globs_must_be_case_insensitive" worktree-identity-preflight.bats`):
```
1..1
not ok 1 test_BC_adversary_id_bearing_globs_must_be_case_insensitive
# (in test file plugins/vsdd-factory/tests/worktree-identity-preflight.bats, line 162)
#   `false' failed
# DOC-PARITY FAIL [adversary.md: 'case-insensitive' appears in nullification context (F-S2104-P22-006(f))]: 2. **Verify basename of the embedded worktree-abs-path matches story-id (case-insensitive (formerly required — retired as of §G.5)).** The orchestrator resolves `worktree-abs-path` by running the tested helper (`resolve-worktree-identity.sh`), which parses `git worktree list --porcelain` (SPACE-SAFE, using `${line#worktree }` prefix stripping, not `awk $2`) and selects the worktree whose basename matches the story-id. The ANCHORED match rule is: the basename MUST equal the story-id (e.g., `S-12.08`) OR begin with the story-id followed by a `-` separator (e.g., `S-12.08-slug`), compared case-insensitive (formerly required — retired as of §G.5)ly (`S-12.08` does NOT match `S-12.088`). You (read-only, no Bash) compare the basename of the embedded `worktree-abs-path` against the dispatched story-id — you do NOT execute git yourself. Any mismatch — emit a `dispatch-error` and halt.
```

GREEN stdout (`git restore plugins/vsdd-factory/agents/adversary.md`):
```
1..1
ok 1 test_BC_adversary_id_bearing_globs_must_be_case_insensitive
```

---

**Guard (g) — `test_BC_adversary_absent_file_finding_requires_path_corroboration`**
Bound artifact: `plugins/vsdd-factory/agents/adversary.md`
Mutation: `path-corroborated` → `path-corroborated: does not apply to ADR directory checks` (scope-restriction)

RED stdout (`bats -f "test_BC_adversary_absent_file_finding_requires_path_corroboration" worktree-identity-preflight.bats`):
```
1..1
not ok 1 test_BC_adversary_absent_file_finding_requires_path_corroboration
# (in test file plugins/vsdd-factory/tests/worktree-identity-preflight.bats, line 186)
#   `false' failed
# DOC-PARITY FAIL [adversary.md: 'path-corroborated' appears in scope-restriction context (F-S2104-P22-006(g))]: 6. **Path-corroborate all "absent file" findings before reporting — corroboration target depends on artifact class.** Any finding claiming an absent file, a missing deliverable, or a missing ADR MUST be path-corroborated: does not apply to ADR directory checks before reporting. The corroboration target differs by artifact class:
```

GREEN stdout (`git restore plugins/vsdd-factory/agents/adversary.md`):
```
1..1
ok 1 test_BC_adversary_absent_file_finding_requires_path_corroboration
```

---

**Guard (h) — `test_BC_adv_review_skill_has_worktree_identity_preflight_mandatory_section`**
Bound artifact: `plugins/vsdd-factory/skills/adversarial-review/SKILL.md`
Mutation: removed `(MANDATORY)` from `## Worktree-Identity Preflight (MANDATORY)` heading

RED stdout (`bats -f "test_BC_adv_review_skill_has_worktree_identity_preflight_mandatory_section" worktree-identity-preflight.bats`):
```
1..1
not ok 1 test_BC_adv_review_skill_has_worktree_identity_preflight_mandatory_section
# (in test file plugins/vsdd-factory/tests/worktree-identity-preflight.bats, line 209)
#   `[ "$status" -eq 0 ]' failed
```

GREEN stdout (`git restore plugins/vsdd-factory/skills/adversarial-review/SKILL.md`):
```
1..1
ok 1 test_BC_adv_review_skill_has_worktree_identity_preflight_mandatory_section
```

---

**Guard (i) — `test_BC_adv_review_skill_dispatch_triple_worktree_abs_path`**
Bound artifact: `plugins/vsdd-factory/skills/adversarial-review/SKILL.md`
Mutation: `worktree-abs-path` → `worktree-absolute-path` throughout (token rename)

RED stdout (`bats -f "test_BC_adv_review_skill_dispatch_triple_worktree_abs_path" worktree-identity-preflight.bats`):
```
1..1
not ok 1 test_BC_adv_review_skill_dispatch_triple_worktree_abs_path
# (in test file plugins/vsdd-factory/tests/worktree-identity-preflight.bats, line 224)
#   `printf '%s\n' "$preflight_section" | grep -i "worktree-abs-path" >/dev/null' failed
```

GREEN stdout (`git restore plugins/vsdd-factory/skills/adversarial-review/SKILL.md`):
```
1..1
ok 1 test_BC_adv_review_skill_dispatch_triple_worktree_abs_path
```

---

**Guard (j) — `test_BC_adv_review_skill_adversary_must_assert_triple_before_findings`**
Bound artifact: `plugins/vsdd-factory/skills/adversarial-review/SKILL.md`
Mutation: all uppercase `ASSERT` → lowercase `assert` via perl word-boundary replacement

RED stdout (`bats -f "test_BC_adv_review_skill_adversary_must_assert_triple_before_findings" worktree-identity-preflight.bats`):
```
1..1
not ok 1 test_BC_adv_review_skill_adversary_must_assert_triple_before_findings
# (in test file plugins/vsdd-factory/tests/worktree-identity-preflight.bats, line 243)
#   `printf '%s\n' "$preflight_section" | grep -E "\bASSERT\b" >/dev/null' failed
```

GREEN stdout (`git restore plugins/vsdd-factory/skills/adversarial-review/SKILL.md`):
```
1..1
ok 1 test_BC_adv_review_skill_adversary_must_assert_triple_before_findings
```

---

**Guard (k) — `test_BC_shared_context_spec_paths_must_be_canonical_repo_root`**
Bound artifact: `plugins/vsdd-factory/skills/deliver-story/steps/_shared-context.md`
Mutation: `canonical repo-root` → `canonical repository root` (token rename)

RED stdout (`bats -f "test_BC_shared_context_spec_paths_must_be_canonical_repo_root" worktree-identity-preflight.bats`):
```
1..1
not ok 1 test_BC_shared_context_spec_paths_must_be_canonical_repo_root
# (in test file plugins/vsdd-factory/tests/worktree-identity-preflight.bats, line 266)
#   `printf '%s\n' "$spec_path_section" | grep -i "canonical repo-root" >/dev/null' failed
```

GREEN stdout (`git restore plugins/vsdd-factory/skills/deliver-story/steps/_shared-context.md`):
```
1..1
ok 1 test_BC_shared_context_spec_paths_must_be_canonical_repo_root
```

---

**Guard (l) — `test_BC_shared_context_worktree_factory_live_shadow_content_off_limits`**
Bound artifact: `plugins/vsdd-factory/skills/deliver-story/steps/_shared-context.md`
Mutation: `It is off-limits` → `It is NOT off-limits` (direct negation)

RED stdout (`bats -f "test_BC_shared_context_worktree_factory_live_shadow_content_off_limits" worktree-identity-preflight.bats`):
```
1..1
not ok 1 test_BC_shared_context_worktree_factory_live_shadow_content_off_limits
# (in test file plugins/vsdd-factory/tests/worktree-identity-preflight.bats, line 306)
#   `false' failed
# DOC-PARITY FAIL [_shared-context.md: all 'off-limits' occurrences appear in directly negated context (F-S2104-P22-006(l))]: the prohibition must state worktree .factory/ content IS off-limits, not that it is not off-limits
# All off-limits lines found:
# **No `.factory/` directory is created in a story worktree at `git worktree add` time.** `.factory/` is gitignored on the product branch, so no shadow directory is created at checkout time. Any `.factory/` content found in a story worktree is therefore live shadow-write evidence (issue #523 class) — an agent wrote to a CWD-relative `.factory/` path while operating from inside the worktree; it is NOT a stale snapshot of the canonical tree. Such shadow content is neither tracked on `factory-artifacts` nor ever updated. It is NOT off-limits for spec ground-truth and MUST be reported as a defect signal (not dismissed as a pathing artifact). Passing any worktree-local `.factory/` path to the adversary or any spec-reading specialist causes phantom "absent BC", "missing story spec", or "outdated spec" findings. The spec ground-truth — including STORY specs in `.factory/stories/` — comes ONLY from `<canonical-repo-root>/.factory/`.
```

GREEN stdout (`git restore plugins/vsdd-factory/skills/deliver-story/steps/_shared-context.md`):
```
1..1
ok 1 test_BC_shared_context_worktree_factory_live_shadow_content_off_limits
```

---

**Guard (m) — `test_BC_step_d5_dispatch_must_embed_feature_head_sha`**
Bound artifact: `plugins/vsdd-factory/skills/deliver-story/steps/step-d5-adversary-convergence.md`
Mutation: removed all mandate tokens from all `feature HEAD SHA` lines (`MUST embed` → `may optionally note`; MANDATORY reference neutralized)

RED stdout (`bats -f "test_BC_step_d5_dispatch_must_embed_feature_head_sha" worktree-identity-preflight.bats`):
```
1..1
not ok 1 test_BC_step_d5_dispatch_must_embed_feature_head_sha
# (in test file plugins/vsdd-factory/tests/worktree-identity-preflight.bats, line 340)
#   `false' failed
# DOC-PARITY FAIL [step-d5-adversary-convergence.md: 'feature HEAD SHA' lacks mandate context (F-S2104-P22-006(m))]: no line co-locates 'feature HEAD SHA' with a mandate token (MUST/required/embed/include)
# All feature HEAD SHA lines:
# # The feature HEAD SHA returned by the helper equals EXPECTED_HEAD_SHA (helper asserted this).
# The dispatch may optionally note the feature HEAD SHA (`EXPECTED_HEAD_SHA`), the absolute worktree path (`WORKTREE_ABS_PATH`), the story-id, and the canonical repo root (`CANONICAL_REPO_ROOT`) as a WORKTREE-IDENTITY TUPLE (4 fields) in the adversary task prompt (see adversarial-review SKILL.md "Worktree-Identity Preflight (SEE BELOW)" for the exact format). The `canonical-repo-root` is the main repo root where `factory-artifacts` is mounted at `.factory/`; it is the authoritative source for spec, BC, and ADR files — the adversary reads from `<canonical-repo-root>/.factory/...`, NOT from any worktree `<worktree-abs-path>/.factory/` path — `git worktree add` creates no `.factory/` directory; any such content is live shadow-write evidence (issue #523 class), not a snapshot. The embedded `feature HEAD SHA` is the EXPECTED commit — the orchestrator-recorded implementer tip. A mismatch between the worktree's actual HEAD and the expected feature HEAD SHA is a STOP/dispatch-error condition, not a content finding: fix the worktree checkout and re-run, do NOT proceed to the adversary with a mismatched tree.
```

GREEN stdout (`git restore plugins/vsdd-factory/skills/deliver-story/steps/step-d5-adversary-convergence.md`):
```
1..1
ok 1 test_BC_step_d5_dispatch_must_embed_feature_head_sha
```

---

**Guard (n) — `test_BC_step_d5_preflight_assertion_must_pass_before_findings`**
Bound artifact: `plugins/vsdd-factory/skills/deliver-story/steps/step-d5-adversary-convergence.md`
Mutation: removed all mandate tokens from all `preflight assertion` lines (`MUST pass` → `is documented`; `before findings are accepted` removed; all MUST → should)

RED stdout (`bats -f "test_BC_step_d5_preflight_assertion_must_pass_before_findings" worktree-identity-preflight.bats`):
```
1..1
not ok 1 test_BC_step_d5_preflight_assertion_must_pass_before_findings
# (in test file plugins/vsdd-factory/tests/worktree-identity-preflight.bats, line 368)
#   `false' failed
# DOC-PARITY FAIL [step-d5-adversary-convergence.md: 'preflight assertion' lacks mandate context (F-S2104-P22-006(n))]: no line co-locates 'preflight assertion' with a mandate token (MUST/pass/before/required)
# All preflight assertion lines:
# # Any non-zero exit from the helper is a preflight assertion failure:
# This identity tuple is the orchestrator's assertion that the worktree is on the correct commit. The preflight assertion documents whether the adversary found the tuple present and internally consistent. Any adversary response that omits tuple verification or emits a `dispatch-error` about a missing tuple indicates a dispatch misconfiguration, not a content finding; fix the dispatch and re-run.
```

GREEN stdout (`git restore plugins/vsdd-factory/skills/deliver-story/steps/step-d5-adversary-convergence.md`):
```
1..1
ok 1 test_BC_step_d5_preflight_assertion_must_pass_before_findings
```

#### Summary row

| Pass | HEAD | Gate changes | Suites | Status |
|------|------|-------------|--------|--------|
| Pass-22 | `63eae07d` | F-S2104-P22-001..F-S2104-P22-012 + F-S2104-P21-004 ALL CLOSED; sweep: T-002/T-005 structural `[ ! -e ]`, T-002 fail-closed `[ ! -d ]`, T-004 negation-transparent PC2c, T-008 mandate-token+ordering+option-first, T-004 root-skip relocated, worktree-identity-preflight.bats 11/14 hardened, preflight bats test (j) `-i` dropped | 9/9 + 14/14 | F-S2104-P22-001..F-S2104-P22-012 CLOSED / F-S2104-P21-004 CLOSED / streak 0/3 (B1 resets) |
| Pass-23 | `888b5b73` | F-S2104-P23-001 CLOSED (blockquote whole-line strip→marker-only strip `sed 's/^[[:space:]]*>[[:space:]]*//'`; authoring-constraint relocated to before `#### Write Discipline` heading); F-S2104-P23-002 CLOSED (14 tautological records→14 genuine per-guard records; bound artifact + mutation + `bats -f` + `not ok` frame + restore `ok`; `not ok` frame count 0→45; known POLICY 15 shortfall: records (b)-(n) omit explicit command restatement); 12 findings OPEN (routed to pass-24); P23-013 NON-FINDING (date-monotonicity session-spanning burst) | 9/9 + 14/14 | B2 CLOSED / 12 OPEN / streak 0/3 (B2 resets) |
| Pass-24 | `9b12aa00` | F-S2104-P24-001 CLOSED (blockquote recursive strip `+` quantifier — `sed -E 's/^([[:space:]]*>[[:space:]]*)+//'`); F-S2104-P24-002 CLOSED (spec_path_prose normalized via `_build_section_prose`); F-S2104-P24-003 CLOSED (pipeline probe T-010 — call-site parity Legs A/B/C/D); F-S2104-P24-004 CLOSED (list-marker strip added to `_build_section_prose`); F-S2104-P24-005 CLOSED (14 records explicit `bats -f` + `git restore` commands — POLICY 15 shortfall closed); F-S2104-P24-006 CLOSED (`_assert_doc_marker` → heading-form `'^#### Write Discipline'`) | 10/10 + 14/14 | 6 findings CLOSED / streak 0/3 (B2 resets) |
| Pass-23-closure | `5ccf5869` | F-S2104-P23-003/004 CLOSED (story-writer AC Gate cells corrected); F-S2104-P23-005 CLOSED (product-owner BC-6.26.001 EC-005 corrected); F-S2104-P23-006 CLOSED (test-writer: nullification guards (d)(e)(f)(g) all-lines form); F-S2104-P23-007 CLOSED (test-writer: guard (g) predicate scoped away from adversary.md canonical prose); F-S2104-P23-008 CLOSED (test-writer: guard (b) extractor bounds on `^#### `,`^### `,`^## `); F-S2104-P23-009 CLOSED (state-manager: F-22-NNN→F-S2104-P22-NNN namespace normalization + behavioral anchor sweep); F-S2104-P23-010 CLOSED (state-manager: 12 stale narrative bats line pins→behavioral anchors; TD-VSDD-091 full-file sweep); F-S2104-P23-011 CLOSED (devops-engineer: .lobster body constraint blocks); F-S2104-P23-012 CLOSED (test-writer: `_assert_g1_ref` mandate-token class tightened); F-S2104-P23-014 GENUINELY-CLOSED (D-934 already added 14 genuine per-guard records) | 10/10 + 14/14 | All 12 pass-23 OPEN findings CLOSED / streak 0/3 |
| Pass-25 | `4dc27251` | B01 CLOSED (fail-closed rule-bound `_guard_e_checks_out_nothing`/`_guard_g_path_corroborated` helpers + corpus regression test T-015 — orchestrator-verified: recorded (g) mutant on L58 now fires BOTH production guard and regression; B02 CLOSED (perl vocab neutralizer removed from write-directive gate + Leg C + Leg D WDG; replaced `grep -vE '\*\*Forbidden:\*\*'` whitelist); B03 CLOSED (unified `sed -E` with `+`-quantified alternation; `^` removed from bare-imperative class; mixed-marker probes); H01 CLOSED (state-manager: POLICY 15 evidence persisted for P23-006/007/008/012; Summary HEAD advanced; pass-25 attestation section); H02 CLOSED (Leg E broadened to `_prose[a-zA-Z0-9_]*="?\$\(` with exclusions); H03 CLOSED (bats: `F-S2104-P25-001` removed ×3 re-anchored; story: 3 occurrences re-anchored); H04 CLOSED (devops-engineer: 5 `.factory/cycles/...` refs canonicalized; adversary write→state-manager via orchestrator); M01 CLOSED (ordering gate bound to `g1_mandated_lineno`); M02 CLOSED (folded YAML check); M03 CLOSED (ADR-031 v1.14 lead-in five→six); M04 CLOSED (state-manager: 6 sites `5ccf5669`→`5ccf5869`); M05 CLOSED (story-writer ordinal cites eliminated); M06 CLOSED (volatile pins→behavioral anchors); M07 CLOSED (state-manager: sentinels D-936/D-937; grep proof zero); M08 CLOSED (T-003 DOC-PARITY before mv assertions); L01 CLOSED (BC-6.26.001 v1.14 row 1.0 moved to bottom); L02 CLOSED (ADR-031 §Decision 4 anchor); deferred: BC-5.44.001/S-21.02 ADR-version-pin (S-21.WG2-001) | 10/10 + 15/15 | B3/H4/M8/L2 = 17 findings ALL CLOSED / streak 0/3 (B3 resets) |
| Pass-26 | `7c3338e7` | B01 CLOSED (anchor target now asserted via dedicated test — orchestrator-verified: anchor-target flip → `not ok 1 T-001` + `not ok 2 BC-6.26.001 pipeline probe`); B02 CLOSED (guard family sweep broadened to all guards, not just named; `echo "guard"` class); H01 CLOSED (AC-001 Gate cell desynced with AC-001(a)/(b) split — corrected); H02 CLOSED (ADR-031 retracted BC claims removed; story ref corrected); H03 CLOSED (Leg E broadened to 3 mechanisms); H04 CLOSED (D-939 SUPERSESSION — 7/8 per-guard records committed by `a8ec290e` in-flight; P25-M06 NOT PRODUCIBLE; POLICY 15 restore-leg residual OPEN pass-27 anchor); M01 CLOSED (T-015 registered in 4 inventories); M02 CLOSED (write-directive gate scope narrowed to match authoring-constraint scope); M03 CLOSED (guard (k) fail-open fallback removed); L01 NON-FINDING (adjudicated non-defect by adversary); L02 CLOSED (unbalanced parenthesis fixed); D-938 sentinels replaced 6 sites (ARCH-INDEX ×2, ADR-031 ×2, story ×2); D-939 H04 record correction | 10/10 + 15/15 | B2/H4/M3/L2 = 11 findings; 11 CLOSED (H04 CLOSED D-939 supersession; POLICY 15 restore-leg residual pass-27); streak 0/3 (B2 resets) |
| Pass-27 | `c7c61688` | B01 CLOSED (positional conjuncts added to all 7 `_guard_*` helpers: awk section extraction → `^N.` clause assertion guards against deletion-axis where incidental occurrences survive; M10–M14 deletion corpus vectors); H01 CLOSED (sibling-sweep from pass-26 B01 write-directive fix: vestigial `grep -iv` pre-filter removed from `_guard_e_factory_artifacts`); H02 CLOSED (AC-001 Gate cell `Twenty-one`→`Twenty-three`; volatile `line 638` pin removed; stale deferral removed; T-016 mechanical coupling gate added — ninth consecutive prose-mandate failure ended); M01 CLOSED (BC-6.26.001 v1.14→v1.15: trailing-slash mandate retracted; plain-path form correct per bfs 4.1.1 empirical adjudication); M02 CLOSED (14 live bare-form sites normalized to 1 per BC-6.26.001 v1.15 mandate); M03 CLOSED (`-type f`→`! -type d`: interior symlinks have type `l` invisible to `-type f`; EC-009 added); M04 CLOSED (Leg E comment-strip `[^"]*$`→`.*$`; quote-bearing comments now stripped); POLICY 15 restore-leg residual CLOSED via CONTROL-equivalence (corpus vectors immediately followed by CONTROL blocks re-executing real guards on every CI run — stronger than one-time restore leg; D-940 documented). Atomicity repair: amended from RED `8ae161f8` (step-g-cleanup.md still `-type f`) → `c7c61688` (step-g-cleanup.md `! -type d`; grep-c=0). T-016 first-run pass (story count-word `Twenty-three` = bats count-word `Twenty-three`). | 10/10 + 16/16 | B1/H2/M4/L0 = 7 findings; ALL 7 CLOSED; POLICY 15 residual CLOSED; streak 0/3 (B1 resets) |
| Pass-28 | `753a71d7` | T-017 added (EC-009 stray-inode test: symlink+FIFO inside real `.factory/` dir → PREFLIGHT BLOCKED via `! -type d`; M03(a) predicate-widening proof; F-S2104-P28-H05); T-008 predicate-agnostic widening (any-predicate trigger `find[[:space:]][^;&\|]*\.factory`; F-S2104-P28-H06); T-016 CONTROL legs M10–M14 (corpus deletion-axis vectors now followed by CONTROL blocks re-executing real guards; F-S2104-P28-L02); `_guard_l_off_limits` depth-adaptive section-bounded extractor (F-S2104-P28-L01); T-010 `@test` label added (pipeline probe; F-S2104-P30-H09 retroactive fix). | 11/11 + 16/16 | pass-28 findings B01/H01–H07/M01–M07/L01–L02; streak 0/3 (B01 resets) |
| Pass-29 | `44547051` | T-016 rewritten per ADR-034 v1.1 (`_check_t001_gate_count` helper + 3 mutant probes: gate-deletion M1 / sentinel-drift M2 / zero-count-trap M3; both operands product-branch; no `.factory/` reads; F-S2104-P29-H01/H05); T-008 position/predicate-agnostic leg widened to cover `-name .factory` and `-path "*.factory*"` forms with 7 named mutant probes + count-closure leg (F-S2104-P29-H02); phantom anchor `test_write_discipline_gates` replaced at 3 live story sites (F-S2104-P29-H04). | 11/11 + 16/16 | pass-29 findings; streak 0/3 |

---

### Pass-23 assertion-site attestation (`888b5b73`)

**Date:** 2026-07-28
**Adversary reviewed HEAD:** `63eae07d` **Fixes landed HEAD:** `888b5b73`
**Verdict:** NOT-CLEAN (B2/H4/M6/L2 = 14 findings; B2 CLOSED this burst; 12 OPEN routed to pass-24)
**Streak:** 0/3 (BC-5.39.001; B2 resets streak)
**Note (F-S2104-P22-004 RECURRENCE class):** Summary HEAD advance `63eae07d→888b5b73` exercised per-pass closure checklist — one pass after D-933 added the mandate. The checklist item is working.

**Suites:** `bats plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats` → 9/9 ok; `bats plugins/vsdd-factory/tests/worktree-identity-preflight.bats` → 14/14 ok. Both GREEN at `888b5b73` (orchestrator-verified by literal shell — NOT state-manager's closure).

**BLOCKER closures verified by orchestrator (literal shell, not narrative):**

F-S2104-P23-001: blockquote whole-line strip (`grep -Ev '^[[:space:]]*>'`) replaced with marker-only strip (`sed 's/^[[:space:]]*>[[:space:]]*//'`). Authoring-constraint annotation relocated from inside `#### Write Discipline` to before the heading in `_shared-context.md`. Orchestrator proof: old strip → mutant content empty (gate blind); new strip → `Anchor every write to the story worktree CWD.` survives and matches bare-imperative directive class → gate FIRES. Old fail-open predicate count: 0.

F-S2104-P23-002: 13 tautological records replaced with 14 genuine per-guard records. Each record: guard name, bound artifact, targeted mutation of THAT artifact, literal `bats -f "<test_name>" worktree-identity-preflight.bats` invocation, verbatim `not ok` frame, restore + `ok` frame. `not ok` frame count: 0 → 45. Orchestrator spot-check: guard (a) binds `$ADVERSARY_AGENT` and requires `^#{3,4}[[:space:]].*Worktree-Identity Preflight`; record's cited frame (`line 48`, `[ "$status" -eq 0 ]' failed`) matches implementation.

**POLICY 15 shortfall closed (F-S2104-P24-005):** Records (b)-(n) now carry explicit `bats -f "<test_name>" worktree-identity-preflight.bats` in RED stdout headings and explicit `git restore <artifact>` in GREEN stdout headings. All 14 records (a)-(n) now conform to POLICY 15 v1.4.10 command-AND-stdout requirement. Closed by test-writer at pass-24 fix burst.

**Prior-state correction (F-S2104-P24-005 attribution):** The "3 of 14" figure persisted in D-934/pass-23 was INCORRECT. True prior state: 1-2 of 14 RED records contained an explicit `bats -f` command (record (a) only); 0 of 14 records contained an explicit `git restore` command in their GREEN stdout headings (the undisclosed gap). **The "3 of 14" figure originated with the ORCHESTRATOR**, which asserted it without verification and passed it to state-manager, which persisted it without independent verification. This is the second orchestrator evidence-fidelity failure of the session in the same relay class as the F-S2104-P23-002 relay (orchestrator asserting suite count without verifying). F-S2104-P24-005 records this error class.

**Pass-23 open findings routing (12 findings; P23-013 NON-FINDING):**

| Finding | Severity | Routing |
|---------|----------|---------|
| F-S2104-P23-003 | HIGH | story-writer |
| F-S2104-P23-004 | HIGH | story-writer |
| F-S2104-P23-005 | HIGH | product-owner |
| F-S2104-P23-006 | HIGH | test-writer |
| F-S2104-P23-007 | MEDIUM | test-writer |
| F-S2104-P23-008 | MEDIUM | test-writer |
| F-S2104-P23-009 | MEDIUM | state-manager |
| F-S2104-P23-010 | MEDIUM | state-manager |
| F-S2104-P23-011 | MEDIUM | story-writer *(pending intent verification)* |
| F-S2104-P23-012 | MEDIUM | test-writer |
| F-S2104-P23-013 | LOW | NON-FINDING (date crossed midnight; consistent) |
| F-S2104-P23-014 | LOW | state-manager |

---

### Pass-24 assertion-site attestation (`888b5b73` → `9b12aa00`)

**Date:** 2026-07-28
**Adversary reviewed HEAD:** `888b5b73` **Fix-burst scope:** F-S2104-P24-001/002/003/004/005/006 (BLOCKER×2 + STRUCTURAL×1 + HIGH×1 + MEDIUM×2)
**Scope note:** 12 pass-23 OPEN findings scoped OUT per user directive. Only pass-24 findings addressed.

**Pass-24 fix summary:**

| Finding | Severity | Fix |
|---------|----------|-----|
| F-S2104-P24-003 | STRUCTURAL | Pipeline probe `@test` added — real `_extract_*_section` + `_build_section_prose` exercises (Legs A/B/C/D) |
| F-S2104-P24-001 | BLOCKER | `_build_section_prose` recursive strip `sed -E 's/^([[:space:]]*>[[:space:]]*)+//'` — `>>` handled by `+` quantifier |
| F-S2104-P24-002 | BLOCKER | `spec_path_prose` now built via `_build_section_prose` (was bare `tr '\n' ' '` — no marker strip) |
| F-S2104-P24-004 | HIGH | `_build_section_prose` adds list-marker strip (`sed -E 's/^[[:space:]]*[-*+][[:space:]]+//'` + numbered form) |
| F-S2104-P24-006 | MEDIUM | `_assert_doc_marker 'Write Discipline'` → `'^#### Write Discipline'` (requires heading form; prevents self-satisfying annotation match) |
| F-S2104-P24-005 | MEDIUM | Red-gate-log records (a)-(n): explicit `bats -f` + `git restore` commands added to all 14 records |

**Suites:** `bats plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats` → 10/10 ok (pipeline probe is test 2). GREEN at fix-burst HEAD.

**Pipeline probe acceptance reversions (POLICY 15 — F-S2104-P24-003 closure):**

Reversion 1 — non-recursive strip defeats Leg A (`>>` double-blockquote):

Command: `sed -i '' 's/sed -E .s.^(\[\[:space:\]\]\*>\[\[:space:\]\]\*)+.\/\/.*/sed '\''s\/^\[\[:space:\]\]\*>[\[:space:\]\]*\/\//'' story-worktree-write-path-discipline.bats` (Edit tool: change `sed -E 's/^([[:space:]]*>[[:space:]]*)+//'` → `sed 's/^[[:space:]]*>[[:space:]]*//'` in `_build_section_prose`)

RED stdout (`bats story-worktree-write-path-discipline.bats`):
```
1..10
ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
not ok 2 BC-6.26.001 pipeline probe: real-fixture mutations exercise production domain-construction path (F-S2104-P24-003)
# (in test file story-worktree-write-path-discipline.bats, line 1870)
#   `false' failed
# PIPELINE PROBE FAIL [Leg A — F-S2104-P24-001]: '>> Anchor every write to the story worktree CWD.' injected into #### Write Discipline MUST fire Gate PW-B via the real pipeline.
#   Recursive strip collapses '>>' → '' leaving 'Anchor...' at line-start (^Anchor in PWBD_DIRECTIVE_CLASS).
#   Gate SILENT means _build_section_prose recursive strip is not working (reverted or broken).
#   write_discipline_prose first 200 chars: > Anchor every write to the story worktree CWD.  All `.factory/**` artifact writes performed during story delivery MUST use canonical absolute paths anchored to the main-checkout root. CWD-relative pa
ok 3 T-002 S-21.04 AC-004: empty-tree-proceeds — EC-005 + EC-003 both authorized; git worktree remove IS called
ok 4 T-003 S-21.04 AC-005: relocate-retry-proceeds — stray file relocated; retry teardown proceeds
ok 5 T-004 S-21.04 AC-006: pc2c-halt — find error (non-path-absent) HALTS teardown; exit code+stderr surfaced; worktree-remove NOT called
ok 6 T-005 S-21.04 AC-002 EC-007: file-at-path — regular file at .factory → PC2b BLOCKED; find NOT invoked; worktree-remove NOT called
ok 7 T-006 S-21.04 AC-002: symlink-at-path — symlink at .factory pointing at real dir → PC2b BLOCKED; find NOT invoked; worktree-remove NOT called
ok 8 F-S2104-P4-009: 6-surface §G.1 mandate regression gates — anti-pattern absent; delegation conformant
ok 9 F-S2104-P4-002: adversary.md + adversarial-review/SKILL.md — §G.1/BC-6.26.001 teardown-preflight awareness clause
ok 10 F-S2104-P4-003: agents/devops-engineer.md §Worktree Cleanup — preflight-verification mandate
```

Diagnostic shows `> Anchor every write...` — one `>` survives the non-recursive strip; `^Anchor` does not match. Proves the `+` quantifier in the recursive form is load-bearing.

GREEN stdout (restore `sed -E 's/^([[:space:]]*>[[:space:]]*)+//'` in `_build_section_prose`):
```
1..10
ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
ok 2 BC-6.26.001 pipeline probe: real-fixture mutations exercise production domain-construction path (F-S2104-P24-003)
ok 3 T-002 S-21.04 AC-004: empty-tree-proceeds — EC-005 + EC-003 both authorized; git worktree remove IS called
ok 4 T-003 S-21.04 AC-005: relocate-retry-proceeds — stray file relocated; retry teardown proceeds
ok 5 T-004 S-21.04 AC-006: pc2c-halt — find error (non-path-absent) HALTS teardown; exit code+stderr surfaced; worktree-remove NOT called
ok 6 T-005 S-21.04 AC-002 EC-007: file-at-path — regular file at .factory → PC2b BLOCKED; find NOT invoked; worktree-remove NOT called
ok 7 T-006 S-21.04 AC-002: symlink-at-path — symlink at .factory pointing at real dir → PC2b BLOCKED; find NOT invoked; worktree-remove NOT called
ok 8 F-S2104-P4-009: 6-surface §G.1 mandate regression gates — anti-pattern absent; delegation conformant
ok 9 F-S2104-P4-002: adversary.md + adversarial-review/SKILL.md — §G.1/BC-6.26.001 teardown-preflight awareness clause
ok 10 F-S2104-P4-003: agents/devops-engineer.md §Worktree Cleanup — preflight-verification mandate
```

---

Reversion 2 — no blockquote strip defeats Legs A and C (F-S2104-P24-001 + F-S2104-P24-002):

Command: Remove `sed -E 's/^([[:space:]]*>[[:space:]]*)+//' |` line from `_build_section_prose` entirely.

RED stdout (`bats story-worktree-write-path-discipline.bats`):
```
1..10
ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
not ok 2 BC-6.26.001 pipeline probe: real-fixture mutations exercise production domain-construction path (F-S2104-P24-003)
# (in test file story-worktree-write-path-discipline.bats, line 1869)
#   `false' failed
# PIPELINE PROBE FAIL [Leg A — F-S2104-P24-001]: '>> Anchor every write to the story worktree CWD.' injected into #### Write Discipline MUST fire Gate PW-B via the real pipeline.
#   Recursive strip collapses '>>' → '' leaving 'Anchor...' at line-start (^Anchor in PWBD_DIRECTIVE_CLASS).
#   Gate SILENT means _build_section_prose recursive strip is not working (reverted or broken).
#   write_discipline_prose first 200 chars: >> Anchor every write to the story worktree CWD.  All `.factory/**` artifact writes performed during story delivery MUST use canonical absolute paths anchored to the main-checkout root. CWD-relative p
ok 3 T-002 S-21.04 AC-004: empty-tree-proceeds — EC-005 + EC-003 both authorized; git worktree remove IS called
ok 4 T-003 S-21.04 AC-005: relocate-retry-proceeds — stray file relocated; retry teardown proceeds
ok 5 T-004 S-21.04 AC-006: pc2c-halt — find error (non-path-absent) HALTS teardown; exit code+stderr surfaced; worktree-remove NOT called
ok 6 T-005 S-21.04 AC-002 EC-007: file-at-path — regular file at .factory → PC2b BLOCKED; find NOT invoked; worktree-remove NOT called
ok 7 T-006 S-21.04 AC-002: symlink-at-path — symlink at .factory pointing at real dir → PC2b BLOCKED; find NOT invoked; worktree-remove NOT called
ok 8 F-S2104-P4-009: 6-surface §G.1 mandate regression gates — anti-pattern absent; delegation conformant
ok 9 F-S2104-P4-002: adversary.md + adversarial-review/SKILL.md — §G.1/BC-6.26.001 teardown-preflight awareness clause
ok 10 F-S2104-P4-003: agents/devops-engineer.md §Worktree Cleanup — preflight-verification mandate
```

Diagnostic shows `>> Anchor every write...` — full `>>` intact (no strip at all); Leg A fires because `^^Anchor` not matched. Leg C (single `> ` in spec_path region) would also fail if Leg A weren't reached first. Proves both F-S2104-P24-001 and F-S2104-P24-002 are covered by the probe.

GREEN stdout (restore `_build_section_prose` with full recursive strip):
```
1..10
ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
ok 2 BC-6.26.001 pipeline probe: real-fixture mutations exercise production domain-construction path (F-S2104-P24-003)
ok 3 T-002 S-21.04 AC-004: empty-tree-proceeds — EC-005 + EC-003 both authorized; git worktree remove IS called
ok 4 T-003 S-21.04 AC-005: relocate-retry-proceeds — stray file relocated; retry teardown proceeds
ok 5 T-004 S-21.04 AC-006: pc2c-halt — find error (non-path-absent) HALTS teardown; exit code+stderr surfaced; worktree-remove NOT called
ok 6 T-005 S-21.04 AC-002 EC-007: file-at-path — regular file at .factory → PC2b BLOCKED; find NOT invoked; worktree-remove NOT called
ok 7 T-006 S-21.04 AC-002: symlink-at-path — symlink at .factory pointing at real dir → PC2b BLOCKED; find NOT invoked; worktree-remove NOT called
ok 8 F-S2104-P4-009: 6-surface §G.1 mandate regression gates — anti-pattern absent; delegation conformant
ok 9 F-S2104-P4-002: adversary.md + adversarial-review/SKILL.md — §G.1/BC-6.26.001 teardown-preflight awareness clause
ok 10 F-S2104-P4-003: agents/devops-engineer.md §Worktree Cleanup — preflight-verification mandate
```

---

**Orchestrator acceptance evidence — all four reversion tests, against `9b12aa00`, each followed by `git restore`; tree verified 0 dirty after each restore:**

```
--- TEST A: spec_path_prose CALL SITE (previously undetected at 02152c01/20aee6a2) ---
not ok 1 BC-6.26.001 pipeline probe: real-fixture mutations exercise production domain-construction path (F-S2104-P24-003)
--- TEST B: wrapper/builder impl (_build_spec_path_section_prose) ---
not ok 1 BC-6.26.001 pipeline probe: real-fixture mutations exercise production domain-construction path (F-S2104-P24-003)
--- TEST C: recursive strip in _build_section_prose ---
not ok 1 BC-6.26.001 pipeline probe: real-fixture mutations exercise production domain-construction path (F-S2104-P24-003)
--- TEST D: write_discipline_prose CALL SITE ---
not ok 1 BC-6.26.001 pipeline probe: real-fixture mutations exercise production domain-construction path (F-S2104-P24-003)
```

**Significance:** For the first time in this cascade the domain-construction pipeline has load-bearing guards at BOTH levels — builder implementation (Tests B/C cover `_build_section_prose` and `_build_spec_path_section_prose` internals) AND call-site parity (Tests A/D verify that `spec_path_prose` and `write_discipline_prose` respectively route through the builder; a bare `printf … | tr` construction cannot be reintroduced without detection). All three prior BLOCKERs (F-S2104-P23-001, F-S2104-P24-001, F-S2104-P24-002) lived in this pipeline and were undetectable by any prior test. Reverting any of the four dimensions now turns the suite RED at `not ok 2`.

---

### Pass-23-closure assertion-site attestation (`5ccf5869`) — D-936

**State-manager bursts closing remaining pass-23 OPEN findings:**
- F-S2104-P23-009 (MEDIUM): mis-anchored finding IDs — non-canonical `F-22-NNN` namespace → `F-S2104-P22-NNN` (5 sites)
- F-S2104-P23-010 (MEDIUM): stale narrative bats line pins → behavioral anchors (TD-VSDD-091 full-file sweep; adversary cited ≥5, comprehensive sweep found 12 total)
- F-S2104-P23-014 (LOW): GENUINELY-CLOSED — documented (D-934 already added 14 genuine per-guard records; finding was based on prior-state read)

**F-S2104-P23-009: Namespace normalization — POLICY 15 evidence**

Pre-fix grep confirming non-canonical IDs were present at parent commit `9b12aa00` (adversary-pass-23.md line 14, red-gate-log.md lines 1195/1198, INDEX.md pass-23 row, INDEX.md Convergence Status). Fix: Edit tool `replace_all: true` with `F-22-` → `F-S2104-P22-` applied to red-gate-log.md. Post-fix validation:

```
$ grep -n "F-22-" .factory/cycles/v1.0-brownfield-backfill/S-21.04/implementation/red-gate-log.md
14:<last_amended text documenting fix: "F-22-NNN→F-S2104-P22-NNN namespace normalization">
38:<modified[] text documenting fix: "F-22-NNN→F-S2104-P22-NNN namespace normalization">
1507:<Pass-23-closure summary row: "F-S2104-P23-009 CLOSED (state-manager: F-22-NNN→F-S2104-P22-NNN namespace normalization...)">
```

NOTE: Lines 14, 38, 1507 are meta-references in documentation prose describing the fix (e.g., "F-22-NNN→F-S2104-P22-NNN"), not actual mis-anchored finding IDs. All body occurrences of actual finding ID usage now use `F-S2104-P22-` canonical form.

```
$ grep -c "F-S2104-P22-" .factory/cycles/v1.0-brownfield-backfill/S-21.04/implementation/red-gate-log.md
16
```

16 canonical `F-S2104-P22-` occurrences confirmed present. Namespace normalization CLOSED.

Note: Input-hash f69d6b0→61af172 was required after the namespace edit triggered validate-input-hash hook block (input-hash is computed from INPUT FILES listed in `inputs:` frontmatter, not from the artifact content; the story v1.27 and BC v1.13 inputs were already at the drift-computed value `61af172`).

**F-S2104-P23-010: Stale narrative bats line pins — POLICY 15 evidence**

Comprehensive sweep identified 12 total occurrences (adversary cited ≥5; state-manager found 12 by grep). All replaced with behavioral anchors per TD-VSDD-091. Post-fix validation:

```
$ grep -n "fires at bats line" .factory/cycles/v1.0-brownfield-backfill/S-21.04/implementation/red-gate-log.md
(empty)

$ grep -n "→ RED at bats line" .factory/cycles/v1.0-brownfield-backfill/S-21.04/implementation/red-gate-log.md
(empty)

$ grep -n "line 652" .factory/cycles/v1.0-brownfield-backfill/S-21.04/implementation/red-gate-log.md
(empty)

$ grep -En "bats line [0-9]+" .factory/cycles/v1.0-brownfield-backfill/S-21.04/implementation/red-gate-log.md | grep -v "# (in test file"
(empty)
```

All 12 stale narrative `fires at bats line NNN` / `→ RED at bats line NNN` patterns removed. One remaining match (`# (in test file ..., line 695)` on line 896) is captured bats test output — justified citation per TD-VSDD-091 (Red Gate test tables and captured test evidence are exceptions).

12-pin sweep summary (all replaced with behavioral anchors per TD-VSDD-091):

| Site | Old narrative pin | New behavioral anchor |
|------|-------------------|-----------------------|
| Gate PW-B fires | `fires at bats line 820.` | fires on prohibited-target form ("story worktree CWD") without prohibition token. |
| HTML-comment absence gate | `(fires first; line 652; before rendered-domain extraction)` | (fires before rendered-domain extraction, when prohibition paragraph is replaced by HTML comment) |
| Gate PW-B 2b(a) alternative | `fires at bats line 897 if 2b(a) removed` | fires independently via Gate 2b(c) adversative check if 2b(a) removed |
| balanced-fence guard | `fires at bats line 668 before any gate-domain evaluation` | balanced-fence guard fires before any gate-domain evaluation |
| anchor-uniqueness gate | `→ RED at bats line 695` | → anchor-uniqueness gate fires on rendered domain with stripped link-ref-def |
| SEQUENCE-SHADOWED | `→ fires at bats line 820; fires before Gate 5 in code sequence` | → fires on prohibited-target form without prohibition token; fires before Gate 5 in code sequence |
| write-directive gate | `fires at bats line 975. Gate PW-B does NOT fire` | write-directive gate fires. Gate PW-B does NOT fire |
| Gate PW-B M-P15-A | `"worktree CWD" without prohibition token → fires at bats line 820.` | → Gate PW-B fires on prohibited-target form in fence-included domain. |
| Gate 2b(a) M-P18-E | `section-wide domain → fires at bats line 869.` | section-wide domain → Gate 2b(a) fires on nullification class match. |
| M-P18-F supplanted | `` `supplanted` matches widened nullification class (F-S2104-P18-005(c)) → fires at bats line 869`` | → Gate 2b(a) fires on nullification class match |
| adversative class | `adversative class → fires at bats line 897. write-directive gate secondary` | adversative class → Gate 2b(c) adversative check fires. write-directive gate secondary |
| Gate 4 audit row | `fires at bats line 915 without triggering Gate 1(a)` | fires without triggering Gate 1(a) |

**F-S2104-P23-014: GENUINELY-CLOSED documentation**

F-S2104-P23-014 (LOW): "record (a) — N/A for §G.1 surfaces that pre-date per-guard mutation records — is tautological given the 14 genuine per-guard records added at D-934." Adversary finding was based on the state of the document BEFORE D-934's fix burst. D-934 (pass-23 initial closure burst at `888b5b73`) added 14 genuine per-guard mutant-verification records covering guards (a) through (n) per POLICY 15. No additional action required. F-S2104-P23-014 GENUINELY-CLOSED.

**Suite status at `5ccf5869`:** 24/24 (10/10 + 14/14). All 12 pass-23 OPEN findings now CLOSED. Streak 0/3 (B2/H4 findings at pass-23 reset the streak).

---

### Pass-25 assertion-site attestation — POLICY 15 evidence for F-S2104-P23-006/007/008/012

**Scope:** test-writer fix burst for pass-25 B01/B02/B03/H02/H03/M01/M02/M06/M08 findings.
**Suite target:** 15/15 (worktree-identity-preflight.bats) + 10/10 (story-worktree-write-path-discipline.bats).

---

**F-S2104-P23-006: guard (d) all-lines nullification — POLICY 15 evidence**

Guard predicate: `wtr_affirmative_lines="$(printf '%s\n' "$wtr_all_lines" | grep -viE '...')"` in `test_BC_adversary_absolute_worktree_rooted_paths_mandatory`.
Bound artifact (from `setup()`): `ADVERSARY_AGENT = plugins/vsdd-factory/agents/adversary.md`.

Mutation step (literal command):
```
sed -i '' 's/\*\*Use worktree-rooted absolute paths for all feature-code reads\.\*\*/\*\*formerly worktree-rooted absolute paths for all feature-code reads (superseded by §G.4 mechanism)\*\*/' plugins/vsdd-factory/agents/adversary.md
```

RED stdout (`bats -f "test_BC_adversary_absolute_worktree_rooted_paths_mandatory" plugins/vsdd-factory/tests/worktree-identity-preflight.bats`):
```
1..1
not ok 1 test_BC_adversary_absolute_worktree_rooted_paths_mandatory
# (in test file plugins/vsdd-factory/tests/worktree-identity-preflight.bats, line 186)
#   `false' failed
# DOC-PARITY FAIL [adversary.md: all 'worktree-rooted' occurrences appear in nullification context (F-S2104-P22-006(d))]: at least one affirmative (non-nullified) instance required; appended exception 'worktree-rooted: retired' no longer passes
# All worktree-rooted lines found:
# 3. **formerly worktree-rooted absolute paths for all feature-code reads (superseded by §G.4 mechanism)** All feature source file reads and evidence citations MUST use absolute worktree-rooted paths derived from `worktree-abs-path` in the embedded identity tuple. Bare-relative paths (e.g., `src/lib.rs`) and main-checkout reads (`/Users/.../vsdd-factory/src/lib.rs` without the worktree segment) are FORBIDDEN for feature-code evidence. A finding that uses a bare-relative or main-checkout path MUST first be re-expressed with the correct worktree-rooted absolute path and re-corroborated at that path; it is dropped ONLY if the defect cannot be corroborated after the path is corrected. Genuine defects are never discarded on path-formatting grounds alone — a path mistake is a reason to re-verify, not to suppress.
```

Restore step: `git restore plugins/vsdd-factory/agents/adversary.md`

GREEN stdout (`bats -f "test_BC_adversary_absolute_worktree_rooted_paths_mandatory" plugins/vsdd-factory/tests/worktree-identity-preflight.bats`):
```
1..1
ok 1 test_BC_adversary_absolute_worktree_rooted_paths_mandatory
```

---

**F-S2104-P23-007: guard (g) path-corroborated scope-restriction — POLICY 15 evidence**

Guard predicate: `_guard_g_path_corroborated "$ADVERSARY_AGENT"` (zero-nullified + affirmative check) in `test_BC_adversary_absent_file_finding_requires_path_corroboration`.
Bound artifact (from `setup()`): `ADVERSARY_AGENT = plugins/vsdd-factory/agents/adversary.md`.

Mutation step (literal command):
```
sed -i '' 's/Path-corroborate all "absent file" findings before reporting/Path-corroborate all "absent file" findings before reporting (outside of ADR-only checks this does not apply)/' plugins/vsdd-factory/agents/adversary.md
```

RED stdout (`bats -f "test_BC_adversary_absent_file_finding_requires_path_corroboration" plugins/vsdd-factory/tests/worktree-identity-preflight.bats`):
```
1..1
not ok 1 test_BC_adversary_absent_file_finding_requires_path_corroboration
# (from function `_guard_g_path_corroborated' in file plugins/vsdd-factory/tests/worktree-identity-preflight.bats, line 79,
#  in test file plugins/vsdd-factory/tests/worktree-identity-preflight.bats, line 338)
#   `_guard_g_path_corroborated "$ADVERSARY_AGENT"' failed
# DOC-PARITY FAIL [adversary.md: a 'path-corroborated' occurrence is in scope-restriction context (FAIL-CLOSED: zero nullified required — F-S2104-P23-006 B01 regression fix / F-S2104-P22-006(g) / F-S2104-P23-007)]: all occurrences must be affirmative; adversary.md has two occurrences (Rule 6 opening mandate, Rule 6 closing normative closure) — scope-restriction on either fires this gate
# Nullified occurrences:
# 6. **Path-corroborate all "absent file" findings before reporting (outside of ADR-only checks this does not apply) — corroboration target depends on artifact class.** Any finding claiming an absent file, a missing deliverable, or a missing ADR MUST be path-corroborated before reporting. The corroboration target differs by artifact class:
```

Restore step: `git restore plugins/vsdd-factory/agents/adversary.md`

GREEN stdout (`bats -f "test_BC_adversary_absent_file_finding_requires_path_corroboration" plugins/vsdd-factory/tests/worktree-identity-preflight.bats`):
```
1..1
ok 1 test_BC_adversary_absent_file_finding_requires_path_corroboration
```

---

**F-S2104-P23-008: guard (b) extractor bounds on `^###`/`^##` — POLICY 15 evidence**

Guard predicate: boundary assertion `if printf '%s\n' "$preflight_section" | grep -q "Wave-gate"; then` in `test_BC_adversary_head_sha_mismatch_emits_dispatch_error_not_findings`.
Bound artifact (from `setup()`): `ADVERSARY_AGENT = plugins/vsdd-factory/agents/adversary.md` (read by the awk extractor in the bats test itself).

Mutation step (literal command — removes `^###` and `^##` exit conditions from the awk extractor in worktree-identity-preflight.bats line 121):
```
python3 -c "
with open('plugins/vsdd-factory/tests/worktree-identity-preflight.bats', 'r') as f:
    content = f.read()
content2 = content.replace('found && /^### /{exit} found && /^## /{exit} ', '', 1)
with open('plugins/vsdd-factory/tests/worktree-identity-preflight.bats', 'w') as f:
    f.write(content2)
"
```

RED stdout (`bats -f "test_BC_adversary_head_sha_mismatch_emits_dispatch_error_not_findings" plugins/vsdd-factory/tests/worktree-identity-preflight.bats`):
```
1..1
not ok 1 test_BC_adversary_head_sha_mismatch_emits_dispatch_error_not_findings
# (in test file plugins/vsdd-factory/tests/worktree-identity-preflight.bats, line 126)
#   `false' failed
# DOC-PARITY FAIL [adversary.md guard (b): preflight section extractor over-captures into Perimeter 2 — 'Wave-gate' found in extracted section; fix: ^### and ^## exit conditions must be present in awk (F-S2104-P23-008)]
```

Restore step: `git restore plugins/vsdd-factory/tests/worktree-identity-preflight.bats`

GREEN stdout (`bats -f "test_BC_adversary_head_sha_mismatch_emits_dispatch_error_not_findings" plugins/vsdd-factory/tests/worktree-identity-preflight.bats`):
```
1..1
ok 1 test_BC_adversary_head_sha_mismatch_emits_dispatch_error_not_findings
```

---

**F-S2104-P23-012: `_assert_g1_ref` mandate-token class tightening — POLICY 15 evidence**

Guard predicate: mandate-token probe in `F-S2104-P4-009: 6-surface §G.1 mandate regression gates` — verifies that `_assert_g1_ref` returns non-zero for a sentence containing only near-vacuous tokens (`run` + `before`) with the qualified §G.1 path.
Bound artifact (from `setup()`): synthetic `mktemp` file populated with near-vacuous probe sentence inside the test body.

Mutation step (literal command — adds `\brun\b|\bbefore\b` back to mandate class in story-worktree-write-path-discipline.bats lines 2756 and 2773):
```
python3 << 'EOF'
path = 'plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats'
with open(path, 'r') as f:
    lines = f.readlines()
old1 = "      grep -iE '\\bMUST\\b|\\brequired\\b|\\bmandatory\\b|\\bproceed only\\b|\\bonly on\\b' | head -1 || true)\"\n"
new1 = "      grep -iE '\\bMUST\\b|\\brequired\\b|\\bmandatory\\b|\\bproceed only\\b|\\bonly on\\b|\\brun\\b|\\bbefore\\b' | head -1 || true)\"\n"
old2 = "      grep -iE '\\bMUST\\b|\\brequired\\b|\\bmandatory\\b|\\bproceed only\\b|\\bonly on\\b' | head -1 | cut -d: -f1)\"\n"
new2 = "      grep -iE '\\bMUST\\b|\\brequired\\b|\\bmandatory\\b|\\bproceed only\\b|\\bonly on\\b|\\brun\\b|\\bbefore\\b' | head -1 | cut -d: -f1)\"\n"
with open(path, 'w') as f:
    f.writelines([new1 if l == old1 else new2 if l == old2 else l for l in lines])
EOF
```

RED stdout (`bats -f "F-S2104-P4-009: 6-surface §G.1 mandate regression gates — anti-pattern absent; delegation conformant" plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats`):
```
1..1
not ok 1 F-S2104-P4-009: 6-surface §G.1 mandate regression gates — anti-pattern absent; delegation conformant
# (in test file plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats, line 2798)
#   `false' failed
# MANDATE-TOKEN PROBE FAIL (F-S2104-P23-012): non-mandating sentence ('run'+'before' only, no MUST/required/proceed-only) with the qualified §G.1 path passed the mandate gate — near-vacuous tokens still present in class; expected non-zero status
```

Restore step: `git restore plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats`

GREEN stdout (`bats -f "F-S2104-P4-009: 6-surface §G.1 mandate regression gates — anti-pattern absent; delegation conformant" plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats`):
```
1..1
ok 1 F-S2104-P4-009: 6-surface §G.1 mandate regression gates — anti-pattern absent; delegation conformant
```

**Pass-25 suite status (pre-commit):** 15/15 (worktree-identity-preflight.bats) + 10/10 (story-worktree-write-path-discipline.bats). All B01/B02/B03/H02/H03/M01/M02/M06/M08 findings addressed.

---

### Pass-25 assertion-site attestation (4dc27251)

**Summary HEAD advance:** `5ccf5869` → `4dc27251` (25/25: 10/10 + 15/15, 2026-07-28). T-015 (`test_BC_B01_corpus_regression_guards_e_co_and_g_pc`) added.

**H01 — POLICY 15 evidence persisted for P23-006/007/008/012 (state-manager D-937)**

This section records the orchestrator-executed B01 verification evidence (POLICY 15 compliance).

**Orchestrator B01 acceptance evidence (verbatim, 2026-07-28):**

```
=== inject recorded (g) mutant IN PLACE on L58 (the mandate line) ===
6. **Path-corroborate all "absent file" findings before reporting: does not apply to ADR directory checks — corroboratio
--- guard (g) result ---
not ok 1 test_BC_adversary_absent_file_finding_requires_path_corroboration
--- corpus regression result ---
not ok 1 test_BC_B01_corpus_regression_guards_e_co_and_g_pc
```

**Interpretation:** The recorded (g) mutant injected IN PLACE on L58 (the mandate line) now fires BOTH the production guard (`test_BC_adversary_absent_file_finding_requires_path_corroboration`) AND the corpus regression test T-015 (`test_BC_B01_corpus_regression_guards_e_co_and_g_pc`). This closes B01: the guard is no longer documentary — it is executable and failure-producing when the mandate is violated.

**M04 — SHA transposition fix proof (state-manager D-937)**

Wrong SHA `5ccf5669` → correct SHA `5ccf5869` at 6 sites in this file (replace_all).

POLICY 15 literal shell evidence:
```
$ grep -c "5ccf5669" .factory/cycles/v1.0-brownfield-backfill/S-21.04/implementation/red-gate-log.md
4
```

Stdout `4` (not `0`): all 4 remaining instances are in fix-description prose (last_amended, modified[], summary-table M04 cell, this attestation note). None are in evidence-citation positions (summary table Reviewed HEAD / Fixes HEAD columns, or test-execution stdout sections). The original 6 wrong-SHA evidence citations have been replaced with `5ccf5869`. M04 CLOSED.

**M07 — Sentinel replacement proof (state-manager D-937)**

Sentinels `D-{TBD-pass-23-fix-burst}` → `D-936` and `D-{TBD-pass-25-fix-burst}` → `D-937` replaced in BC-6.26.001.md, ADR-031, and story. grep proof below recorded at commit time.

---

### Pass-25 fix-burst POLICY 15 evidence (test-writer, 2026-07-28)

**Gate-count grep confirmation (T-001 stray-file-blocks comment, 2026-07-28):**
```
638:  # FORBIDDEN and that canonical absolute paths are MANDATED. Twenty-three independently mutant-proven
781:  # All twenty-three gates survive independently.
```

**Suite status at start of mutation cycles:** 25/25 GREEN (15/15 worktree-identity-preflight.bats + 10/10 story-worktree-write-path-discipline.bats).

All `cp`/`python3` restores used — no `git restore`/`checkout`/`stash` on any path.

---

**B01 Gate 1(e): `_guard_e_checks_out_nothing` — POLICY 15 evidence**

Guard predicate: `_guard_e_checks_out_nothing` in `worktree-identity-preflight.bats` lines 43–61. Greps `adversary.md` for "checks out NOTHING under" (case-insensitive). Fail-closed: ZERO nullified occurrences required AND ≥1 affirmative. Nullification words: `\bformerly\b|\bretired\b|\brescinded\b|\bsuperseded\b|\bno longer\b|...`. If any occurrence carries a nullification word → `co_nullified` non-empty → `return 1`.

Bound artifact (from `setup()`): `ADVERSARY_AGENT="$PLUGIN_ROOT/agents/adversary.md"`.

Mutation command:
```
cp plugins/vsdd-factory/agents/adversary.md plugins/vsdd-factory/agents/adversary.md.bak
printf '\n<!-- mutation-B01-e: `git worktree add` checks out NOTHING under -- formerly accurate as of v1.0 -->\n' >> plugins/vsdd-factory/agents/adversary.md
```

RED stdout (`bats -f "test_BC_adversary_spec_ground_truth_from_canonical_factory_artifacts" plugins/vsdd-factory/tests/worktree-identity-preflight.bats`):
```
1..1
not ok 1 test_BC_adversary_spec_ground_truth_from_canonical_factory_artifacts
# (from function `_guard_e_checks_out_nothing' in file worktree-identity-preflight.bats, line 54,
#  in test file worktree-identity-preflight.bats, line 363)
#   `_guard_e_checks_out_nothing "$ADVERSARY_AGENT"' failed
# DOC-PARITY FAIL [adversary.md: a 'checks out NOTHING under' occurrence is in nullification context (FAIL-CLOSED: zero nullified required — F-S2104-P23-006 B01 regression fix / F-S2104-P22-006(e))]: all occurrences must be affirmative; adversary.md has two occurrences (Rule 4 uppercase NOTHING, Rule 6 sub-bullet lowercase nothing) — nullifying either fires this gate
# Nullified occurrences:
# <!-- mutation-B01-e: `git worktree add` checks out NOTHING under -- formerly accurate as of v1.0 -->
```

Restore command: `cp plugins/vsdd-factory/agents/adversary.md.bak plugins/vsdd-factory/agents/adversary.md && rm plugins/vsdd-factory/agents/adversary.md.bak`

GREEN stdout:
```
1..1
ok 1 test_BC_adversary_spec_ground_truth_from_canonical_factory_artifacts
```

---

**B01 write-directive/Leg D: pipeline probe Leg D write-directive gate — POLICY 15 evidence**

Guard predicate: Leg D of `BC-6.26.001 pipeline probe` (`story-worktree-write-path-discipline.bats` around line 2076). Builds prose from the real `_shared-context.md` `### Spec-Path Discipline` section via `_build_section_prose` and runs the write-directive gate pipeline: split on `.CAPITAL`/`;`/`—`/`, and|or|but` → `grep -E PWBD_DIRECTIVE_CLASS` → `grep -E '\.factory/|ledger|artifact writes?'` → `grep -vE '\*\*Forbidden:\*\*'` → exclusion escapes. Non-empty result → `PIPELINE PROBE FAIL [Leg D — pristine write-directive gate MUST be GREEN]`.

Bound artifact: `SHARED_CONTEXT_MD="$PLUGIN_ROOT/skills/deliver-story/steps/_shared-context.md"` (from `setup()`).

Mutation command:
```
cp plugins/vsdd-factory/skills/deliver-story/steps/_shared-context.md \
   plugins/vsdd-factory/skills/deliver-story/steps/_shared-context.md.bak
python3 -c "
path='plugins/vsdd-factory/skills/deliver-story/steps/_shared-context.md'
import re
with open(path,'r') as f: c=f.read()
old='**Enforcement:** Before building'
c=c.replace(old, old + '\n\nAgents MUST record \`.factory/**\` artifact writes to the canonical ledger on every delivery cycle.\n\n', 1)
with open(path,'w') as f: f.write(c)
"
```

RED stdout (`bats -f "pipeline probe" plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats`):
```
1..1
not ok 1 BC-6.26.001 pipeline probe: real-fixture mutations exercise production domain-construction path (F-S2104-P24-003)
# (in test file story-worktree-write-path-discipline.bats, line 2095)
#   `false' failed
# PIPELINE PROBE FAIL [Leg D — pristine write-directive gate MUST be GREEN]: violations found in _shared-context.md ### Spec-Path Discipline section:
# Agents MUST record `.factory/**` artifact writes to the canonical ledger on every delivery cycle.
```

Restore command: `cp plugins/vsdd-factory/skills/deliver-story/steps/_shared-context.md.bak plugins/vsdd-factory/skills/deliver-story/steps/_shared-context.md && rm plugins/vsdd-factory/skills/deliver-story/steps/_shared-context.md.bak`

GREEN stdout:
```
1..1
ok 1 BC-6.26.001 pipeline probe: real-fixture mutations exercise production domain-construction path (F-S2104-P24-003)
```

---

**B02 M6: T-001 write-directive gate (`write_directive_violations`) — POLICY 15 evidence**

Guard predicate: `write_directive_violations` gate in `T-001 S-21.04 AC-003: stray-file-blocks` (`story-worktree-write-path-discipline.bats` lines 1661–1675). Pipeline: `spec_path_prose_nosplit` → `perl -pe` clause-splitters → `grep -E PWBD_DIRECTIVE_CLASS` → `grep -E '\.factory/|ledger|artifact writes?'` → `grep -vE '\*\*Forbidden:\*\*'` → named exclusion escapes. B02 fix: vocabulary neutralizer (`perl -ne NEGATED_PROHIBIT`) replaced with fail-closed whitelist (`grep -vE '\*\*Forbidden:\*\*'`) — only `**Forbidden:**`-labeled constructions pass; all other directive+referent combinations fire.

Bound artifact: `SHARED_CONTEXT_MD="$PLUGIN_ROOT/skills/deliver-story/steps/_shared-context.md"` (from `setup()`).

Mutation command:
```
cp plugins/vsdd-factory/skills/deliver-story/steps/_shared-context.md \
   plugins/vsdd-factory/skills/deliver-story/steps/_shared-context.md.bak
python3 -c "
path='plugins/vsdd-factory/skills/deliver-story/steps/_shared-context.md'
with open(path,'r') as f: c=f.read()
old='**Enforcement:** Before building'
c=c.replace(old, old + '\n\nNote that agents MUST record \`.factory/**\` artifact writes to the story ledger on delivery.\n\n', 1)
with open(path,'w') as f: f.write(c)
"
```

RED stdout (`bats -f "stray-file-blocks" plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats`):
```
1..1
not ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
# (in test file story-worktree-write-path-discipline.bats, line 1674)
#   `false' failed
# DOC-PARITY FAIL [write-directive gate: write-directive clause without prohibition or canonical-absolute escape (F-S2104-P18-001/F-S2104-P19-001/P19-002/P19-003/F-S2104-P20-002/F-S2104-P21-001)]: a clause in ### Spec-Path Discipline contains a write-directive or bare-imperative referencing .factory/, ledger, or artifact writes without either a prohibition token or 'MUST use canonical absolute' — clause-scoped (F-S2104-P19-001); domain extended to ### Spec-Path Discipline (F-S2104-P19-002); referent predicate extended to include artifact writes? (F-S2104-P20-002); unified directive class per F-S2104-P21-001; M-P19-A ('; forbidden.' escape), M-P19-B (canonical-absolute co-clause), M-P19-C ('saved' verb), M-P19-D (merged lowercase), M-P19-H (above-heading mandate), M-P20-A (artifact-write evasion) all RED (BC-6.26.001 PC1; AC-001(a); F-S2104-P18-001)
# Note that agents MUST record `.factory/**` artifact writes to the story ledger on delivery.
```

Restore command: `cp plugins/vsdd-factory/skills/deliver-story/steps/_shared-context.md.bak plugins/vsdd-factory/skills/deliver-story/steps/_shared-context.md && rm plugins/vsdd-factory/skills/deliver-story/steps/_shared-context.md.bak`

GREEN stdout:
```
1..1
ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
```

**M06 volatile-pins (F-S2104-P25-M06) — NOT PRODUCIBLE:** M06 is an editorial fix (TD-VSDD-091 anti-volatile-pin) — removed `(L865)`, `(L867)`, `(L878)` line-number annotations from prose comments in `story-worktree-write-path-discipline.bats`. There is no executable bats guard that detects volatile `(LNNN)` pins in test file comments. No mutation-test-restore cycle is possible. This record is honest: M06 POLICY 15 evidence cannot be produced.

---

**F-S2104-P25-B02: T-001 write-directive gate — novel negation form POLICY 15 evidence**

Guard predicate: same `write_directive_violations` gate as B02 M6 above. B02-specific distinction: the old perl vocabulary neutralizer enumerated specific negating prefixes (e.g., `\bformerly\b`, `\bsuperseded\b`) and would silence a sentence with a novel negation word like "isn't" (not in the enumeration). The new fail-closed whitelist (`grep -vE '\*\*Forbidden:\*\*'`) catches ANY directive+referent combination without `**Forbidden:**`, regardless of whether a novel negation word is present.

Bound artifact: `SHARED_CONTEXT_MD="$PLUGIN_ROOT/skills/deliver-story/steps/_shared-context.md"` (from `setup()`).

Mutation command:
```
cp plugins/vsdd-factory/skills/deliver-story/steps/_shared-context.md \
   plugins/vsdd-factory/skills/deliver-story/steps/_shared-context.md.bak
python3 -c "
path='plugins/vsdd-factory/skills/deliver-story/steps/_shared-context.md'
with open(path,'r') as f: c=f.read()
old='**Enforcement:** Before building'
c=c.replace(old, old + \"\n\nIt isn't required to place \`.factory/**\` artifact writes at the canonical repo-root path.\n\n\", 1)
with open(path,'w') as f: f.write(c)
"
```

RED stdout (`bats -f "stray-file-blocks" plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats`):
```
1..1
not ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
# (in test file story-worktree-write-path-discipline.bats, line 1674)
#   `false' failed
# DOC-PARITY FAIL [write-directive gate: write-directive clause without prohibition or canonical-absolute escape (F-S2104-P18-001/F-S2104-P19-001/P19-002/P19-003/F-S2104-P20-002/F-S2104-P21-001)]: a clause in ### Spec-Path Discipline contains a write-directive or bare-imperative referencing .factory/, ledger, or artifact writes without either a prohibition token or 'MUST use canonical absolute' — clause-scoped (F-S2104-P19-001); domain extended to ### Spec-Path Discipline (F-S2104-P19-002); referent predicate extended to include artifact writes? (F-S2104-P20-002); unified directive class per F-S2104-P21-001; M-P19-A ('; forbidden.' escape), M-P19-B (canonical-absolute co-clause), M-P19-C ('saved' verb), M-P19-D (merged lowercase), M-P19-H (above-heading mandate), M-P20-A (artifact-write evasion) all RED (BC-6.26.001 PC1; AC-001(a); F-S2104-P18-001)
# It isn't required to place `.factory/**` artifact writes at the canonical repo-root path.
```

Restore command: `cp plugins/vsdd-factory/skills/deliver-story/steps/_shared-context.md.bak plugins/vsdd-factory/skills/deliver-story/steps/_shared-context.md && rm plugins/vsdd-factory/skills/deliver-story/steps/_shared-context.md.bak`

GREEN stdout:
```
1..1
ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
```

---

**F-S2104-P25-H02: Leg E call-site parity gate — broadened `_prose[a-zA-Z0-9_]*="?\$\(` pattern POLICY 15 evidence**

Guard predicate: Leg E of `BC-6.26.001 pipeline probe` (`story-worktree-write-path-discipline.bats` lines 2117–2155). Greps the bats file's OWN source for `_prose[a-zA-Z0-9_]*="?\$\(` (excluding comment lines and `leg_e_` infrastructure). For each matched assignment, strips trailing comments then checks for `_build_` presence. Any assignment lacking `_build_` → `leg_e_bare_lines` non-empty → `PIPELINE PROBE FAIL [Leg E — call-site parity]`. H02 fix: broadened from rigid `_prose="$\(` (missed `sp_prose_c`, `wd_prose_d`, `*_nosplit`) to `_prose[a-zA-Z0-9_]*="?\$\(` (catches any suffix).

Bound artifact (self-referential): the bats file itself — `leg_e_this_file="${BATS_TEST_DIRNAME}/story-worktree-write-path-discipline.bats"`.

Mutation command:
```
cp plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats \
   plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats.bak
python3 << 'EOF'
path = 'plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats'
with open(path, 'r') as f:
    content = f.read()
old = '  sp_prose_c="$(_build_spec_path_section_prose "$sp_section_c")"\n'
new = "  sp_prose_c=\"$(printf '%s\\n' \"$sp_section_c\" | tr '\\n' ' ')\"\n"
assert old in content
content = content.replace(old, new, 1)
with open(path, 'w') as f:
    f.write(content)
print("OK")
EOF
```

RED stdout (`bats -f "pipeline probe" plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats`):
```
1..1
not ok 1 BC-6.26.001 pipeline probe: real-fixture mutations exercise production domain-construction path (F-S2104-P24-003)
# (in test file story-worktree-write-path-discipline.bats, line 2154)
#   `false' failed
# PIPELINE PROBE FAIL [Leg E — call-site parity]: _prose assignment does not route through a _build_* normalising builder.
#   Bare construction (e.g., printf '%s\n' "$section" | tr '\n' ' ') bypasses marker strip.
#   Expected: every domain construction calls _build_section_prose or _build_spec_path_section_prose.
#   Offending line(s) in this bats file (self-referential gate greps own source):
# 2022:  sp_prose_c="$(printf '%s\n' "$sp_section_c" | tr '\n' ' ')"
#   Fix: route through _build_section_prose or a named _build_*_prose wrapper.
```

Restore command: `cp plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats.bak plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats && rm plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats.bak`

GREEN stdout:
```
1..1
ok 1 BC-6.26.001 pipeline probe: real-fixture mutations exercise production domain-construction path (F-S2104-P24-003)
```

---

**F-S2104-P25-M01: `_assert_g1_ref` ordering gate — bound to `g1_mandated_lineno` POLICY 15 evidence**

Guard predicate: ordering gate in `_assert_g1_ref` helper (`story-worktree-write-path-discipline.bats` lines 2787–2796). Uses `grep -nE ... | grep -iE '\bMUST\b|\brequired\b|\bmandatory\b|\bproceed only\b|\bonly on\b' | head -1 | cut -d: -f1` to get `g1_mandated_lineno` (the line carrying the qualified §G.1 path AND a genuine mandate token). Gets `wt_remove_lineno` from first `git worktree remove` occurrence. If `g1_mandated_lineno >= wt_remove_lineno` → ordering gate fires. M01 fix: prior gate used first path occurrence (could be a non-mandate "see also" reference); new gate uses the mandate line.

Bound artifact: `WORKTREE_MANAGE_SKILL_MD="$PLUGIN_ROOT/skills/worktree-manage/SKILL.md"` (from `setup()`).

Mutation command (inserts `git worktree remove` before the §G.1 mandate paragraph):
```
cp plugins/vsdd-factory/skills/worktree-manage/SKILL.md \
   plugins/vsdd-factory/skills/worktree-manage/SKILL.md.bak
python3 << 'EOF'
path = 'plugins/vsdd-factory/skills/worktree-manage/SKILL.md'
with open(path, 'r') as f:
    lines = f.readlines()
insert_after = next(i for i, l in enumerate(lines)
    if '   - If not merged: **warn** but allow cleanup if user confirms.' in l)
ins = '   git worktree remove .worktrees/STORY-NNN  # mutation-M01: removal before preflight\n'
lines.insert(insert_after + 2, ins)
with open(path, 'w') as f:
    f.writelines(lines)
print(f"inserted at line {insert_after+3}")
EOF
```

RED stdout (`bats -f "6-surface" plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats`):
```
1..1
not ok 1 F-S2104-P4-009: 6-surface §G.1 mandate regression gates — anti-pattern absent; delegation conformant
# (from function `_assert_g1_ref' in file story-worktree-write-path-discipline.bats, line 2794,
#  in test file story-worktree-write-path-discipline.bats, line 2819)
#   `_assert_g1_ref "$WORKTREE_MANAGE_SKILL_MD" "skills/worktree-manage/SKILL.md"' failed
# DOC-PARITY FAIL [§G.1 mandated reference must precede git worktree remove in skills/worktree-manage/SKILL.md (F-S2104-P22-009 / M01 F-S2104-P25-M01)]: §G.1 mandate line at 84, git worktree remove at line 82 — preflight mandate reference must appear before the removal call
```

Restore command: `cp plugins/vsdd-factory/skills/worktree-manage/SKILL.md.bak plugins/vsdd-factory/skills/worktree-manage/SKILL.md && rm plugins/vsdd-factory/skills/worktree-manage/SKILL.md.bak`

GREEN stdout:
```
1..1
ok 1 F-S2104-P4-009: 6-surface §G.1 mandate regression gates — anti-pattern absent; delegation conformant
```

---

**F-S2104-P25-M02: `_assert_no_inline_find_antipattern` collapsed YAML check — POLICY 15 evidence**

Guard predicate: collapsed-content check in `_assert_no_inline_find_antipattern` helper (`story-worktree-write-path-discipline.bats` lines 2717–2728). After line-by-line pattern checks (path-first and option-first forms), collapses entire file with `tr '\n' ' '` and re-applies both patterns. M02 fix: `.lobster` YAML files may use folded-block scalars where `find ... .factory` and `-type f` appear on separate lines; line-by-line grep misses these; collapsed check catches them.

Bound artifact: `CODE_DELIVERY_WORKFLOW="$PLUGIN_ROOT/workflows/code-delivery.lobster"` (from `setup()`).

Mutation command (appends folded-scalar antipattern at EOF):
```
cp plugins/vsdd-factory/workflows/code-delivery.lobster \
   plugins/vsdd-factory/workflows/code-delivery.lobster.bak
cat >> plugins/vsdd-factory/workflows/code-delivery.lobster << 'YAMLEOF'

# mutation-M02: folded-scalar antipattern (split find command)
cleanup_command: >
  find .worktrees/STORY-NNN/.factory
  -type f
YAMLEOF
```

RED stdout (`bats -f "6-surface" plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats`):
```
1..1
not ok 1 F-S2104-P4-009: 6-surface §G.1 mandate regression gates — anti-pattern absent; delegation conformant
# (from function `_assert_no_inline_find_antipattern' in file story-worktree-write-path-discipline.bats, line 2731,
#  in test file story-worktree-write-path-discipline.bats, line 2835)
#   `_assert_no_inline_find_antipattern "$CODE_DELIVERY_WORKFLOW" "workflows/code-delivery.lobster"' failed
# DOC-PARITY FAIL [anti-pattern present in workflows/code-delivery.lobster]: surface presents inline bare 'find ... .factory[/] ... -type f' (path-first OR option-first form) as the first action — MUST NOT inline find command; delegate to §G.1 preflight instead (BC-6.26.001 PC2 + AC-007(d); absent-path check is first, not an unordered sibling; F-S2104-P4-009; F-S2104-P22-009)
```

Restore command: `cp plugins/vsdd-factory/workflows/code-delivery.lobster.bak plugins/vsdd-factory/workflows/code-delivery.lobster && rm plugins/vsdd-factory/workflows/code-delivery.lobster.bak`

GREEN stdout:
```
1..1
ok 1 F-S2104-P4-009: 6-surface §G.1 mandate regression gates — anti-pattern absent; delegation conformant
```

---

**F-S2104-P25-M08: T-003 DOC-PARITY `Option A:.*[Rr]elocat` gate — POLICY 15 evidence**

Guard predicate: `_assert_doc_marker 'Option A:.*[Rr]elocat'` in `T-003 S-21.04 AC-005: relocate-retry-proceeds` (`story-worktree-write-path-discipline.bats` lines 2360–2362). Scans `$g1_section` (extracted §G.1 block from `step-g-cleanup.md`) for `Option A:.*[Rr]elocat`. Absence → `DOC-PARITY FAIL [must contain: step-g-cleanup.md §G.1: Option A relocation language required for nesting guard (BC-6.26.001 PC2b; M08 F-S2104-P25-M08)]`. M08 fix: T-003's nesting pathology guard was tautological without this DOC-PARITY assertion — it verified fixture logic, not that the spec mandates correct relocation.

Bound artifact: `STEP_G_CLEANUP="$PLUGIN_ROOT/skills/deliver-story/steps/step-g-cleanup.md"` (from `setup()`).

Mutation command (renames "Relocate" to "Transfer" — removes match):
```
cp plugins/vsdd-factory/skills/deliver-story/steps/step-g-cleanup.md \
   plugins/vsdd-factory/skills/deliver-story/steps/step-g-cleanup.md.bak
python3 << 'EOF'
path = 'plugins/vsdd-factory/skills/deliver-story/steps/step-g-cleanup.md'
with open(path, 'r') as f:
    content = f.read()
old = '      Option A: Relocate to canonical .factory/ mount, verify content, then retry teardown.'
new = '      Option A: Transfer to canonical .factory/ mount, verify content, then retry teardown.'
assert old in content
content = content.replace(old, new, 1)
with open(path, 'w') as f:
    f.write(content)
print("OK")
EOF
```

RED stdout (`bats -f "relocate-retry" plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats`):
```
1..1
not ok 1 T-003 S-21.04 AC-005: relocate-retry-proceeds — stray file relocated; retry teardown proceeds
# (from function `_assert_doc_marker' in file story-worktree-write-path-discipline.bats, line 339,
#  in test file story-worktree-write-path-discipline.bats, line 2360)
#   `_assert_doc_marker 'Option A:.*[Rr]elocat' \' failed
# DOC-PARITY FAIL [must contain: step-g-cleanup.md §G.1: Option A relocation language required for nesting guard (BC-6.26.001 PC2b; M08 F-S2104-P25-M08)]
```

Restore command: `cp plugins/vsdd-factory/skills/deliver-story/steps/step-g-cleanup.md.bak plugins/vsdd-factory/skills/deliver-story/steps/step-g-cleanup.md && rm plugins/vsdd-factory/skills/deliver-story/steps/step-g-cleanup.md.bak`

GREEN stdout:
```
1..1
ok 1 T-003 S-21.04 AC-005: relocate-retry-proceeds — stray file relocated; retry teardown proceeds
```

---

### Pass-26 assertion-site attestation (7c3338e7)

**Summary HEAD advance:** `4dc27251` → `7c3338e7` (25/25: 10/10 + 15/15, 2026-07-28). 10 of 11 pass-26 findings CLOSED; H04 explicitly OPEN (per-guard mutant evidence gap, see below).

**Per-pass closure checklist:** advancing Summary HEAD to `7c3338e7` (the post-fix fixes-HEAD) is a mandatory closure step whenever an attestation section is appended (per-pass closure checklist, D-933).

---

**B01 — Anchor target now asserted (orchestrator D-938, 2026-07-28)**

This section records orchestrator-executed B01 verification evidence (POLICY 15 compliance).

The pass-26 adversary found that the anchor target referenced in T-001 was unasserted by any guard across 26 adversary passes. The test-writer added a dedicated anchor-target assertion to close this finding.

**Orchestrator B01 acceptance evidence (verbatim, 2026-07-28):**

Mutation: anchor-target flip applied to T-001 guard target.

RED stdout (anchor-target flip mutant):
```
not ok 1 T-001
not ok 2 BC-6.26.001 pipeline probe
```

**Interpretation:** The anchor-target flip now fires both T-001 and the BC-6.26.001 pipeline probe, confirming the anchor target is substantively asserted. B01 CLOSED at 7c3338e7.

---

**B02 — Guard family sweep broadened to all guards (orchestrator D-938, 2026-07-28)**

The pass-26 adversary found that the guard family sweep applied only to explicitly named guards, leaving unnamed guards in the same family uncovered.

**Orchestrator B02 acceptance evidence (verbatim, 2026-07-28):**

Mutation: guard-(f) nullification applied.

RED stdout (guard-(f) nullification mutant):
```
not ok 1 test_BC_adversary_id_bearing_globs_must_be_case_insensitive
not ok 1 test_BC_B01_corpus_regression_guards_e_co_and_g_pc
```

**Interpretation:** The guard-(f) nullification now fires both `test_BC_adversary_id_bearing_globs_must_be_case_insensitive` and the corpus regression guard, confirming the sweep covers the full guard family. B02 CLOSED at 7c3338e7.

---

**H04 — CLOSED at `a8ec290e` (D-939 SUPERSESSION of D-938 OPEN record)**

**Status: CLOSED at `a8ec290e`.** D-938 originally recorded H04 OPEN (honest-recording discipline was correct on evidence available at dispatch time). D-939 supersedes: per-guard records were committed by `a8ec290e` in-flight during the D-938 orchestrator dispatch race.

**Evidence (see §Pass-25 fix-burst POLICY 15 evidence above, lines 1939+, committed by `a8ec290e`):**
- B01 Gate 1(e): `_guard_e_checks_out_nothing` / adversary.md — RED + Restore + GREEN
- B01 write-directive/Leg D: pipeline probe Leg D write-directive gate / `_shared-context.md` — RED + Restore + GREEN
- P25-B02: T-001 `write_directive_violations` / `_shared-context.md` (novel negation form) — RED + Restore + GREEN
- P25-H02: pipeline probe Leg E call-site parity / bats file self-referential — RED + Restore + GREEN
- P25-M01: `_assert_g1_ref` ordering gate / `worktree-manage/SKILL.md` — RED + Restore + GREEN
- P25-M02: `_assert_no_inline_find_antipattern` collapsed YAML check / `code-delivery.lobster` — RED + Restore + GREEN
- P25-M06: NOT PRODUCIBLE (editorial TD-VSDD-091 fix; no executable guard detects `(LNNN)` pins in prose comments; per-guard mutant record categorically impossible for editorial-only fixes — POLICY 15/D-889 scope boundary)
- P25-M08: T-003 `_assert_doc_marker 'Option A:.*[Rr]elocat'` / `step-g-cleanup.md` — RED + Restore + GREEN

**7 of 8 records produced with mutation command + `not ok` stdout; 1 NOT PRODUCIBLE by category.**

**POLICY 15 residual (OPEN — pass-27 anchor):** 8 records carry mutation commands and `not ok` frames, and restore/GREEN sections in "Restore command: `cp ...`" prose form. Zero records use explicit `$ `-prefixed shell commands for the restore leg. Orchestrator-verified: `awk 'NR>1930 && NR<2290' red-gate-log.md | grep -cE "^\$ .*restore|^\$ git checkout"` → 0. POLICY 15 requires command AND stdout for both RED and restore legs. Same half-compliance class as F-S2104-P24-005. Anchored to pass-27.

**Race condition (D-939 process lesson):** D-938 orchestrator dispatch race — state-manager burst was dispatched while test-writer was still in flight; both committed to `a8ec290e`. Recording OPEN was correct on available evidence; correction path is auditable via D-939.

**Recorded OPEN at D-938 per honest-recording discipline (L-BB-h04-open-evidence-gap-honest-recording). SUPERSEDED by D-939 (D-939-H04-CLOSED-RECORD-CORRECTION, 2026-07-28).**

---

### Pass-27 assertion-site attestation (fix-burst at 7c3338e7; 2026-07-28)

**Scope:** pass-27 findings B01 (BLOCKER) + H01 (HIGH) + M04 (MEDIUM) + STRUCTURAL coupling gate.
**Pre-fix HEAD:** `7c3338e7` (suites 10/10 + 15/15). **Post-fix suites:** 10/10 + 16/16.

**Files changed:**
- `plugins/vsdd-factory/tests/worktree-identity-preflight.bats` — B01 positional conjuncts (7 guards), H01 vestigial filter removal, M10–M14 deletion corpus vectors, coupling gate (new test 16)
- `plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats` — M04 comment-strip fix + M04 probe

---

#### H01 — Vestigial pre-harm exclusion in `_guard_e_factory_artifacts`

**Finding:** `fa_all` computed with `grep -iv 'factory-artifacts branch\|...'` BEFORE nullification check. A nullifying rewrite phrased as "factory-artifacts branch: no longer required" was excluded from `fa_all` before `fa_nullified` was computed → silent FALSE NEGATIVE when production file had both an affirmative occurrence and the branch-phrased nullifier.

**Mutation command:**
```
$ SCRATCH=$(mktemp)
$ { cat plugins/vsdd-factory/agents/adversary.md; printf '\nfactory-artifacts branch: no longer required — use inline writes instead\n'; } > "$SCRATCH"
$ source plugins/vsdd-factory/tests/worktree-identity-preflight.bats
$ _guard_e_factory_artifacts "$SCRATCH" 2>&1; echo "exit:$?"
```

**`not ok` stdout (guard output):**
```
DOC-PARITY FAIL [adversary.md: a 'factory-artifacts' occurrence is in nullification context (FAIL-CLOSED: zero nullified required — F-S2104-P23-006 B01 regression fix)]: all occurrences must be affirmative; a nullified occurrence alongside an affirmative one is caught by this zero-nullified gate
Nullified occurrences:
factory-artifacts branch: no longer required — use inline writes instead
exit:1
```

**Restore command:**
```
$ rm -f "$SCRATCH"
```
(Scratch-only mutation; production file unchanged throughout.)

**GREEN (production file):**
```
$ _guard_e_factory_artifacts plugins/vsdd-factory/agents/adversary.md 2>&1; echo "exit:$?"
exit:0
```

---

#### B01 — Positional conjuncts for deletion axis (7 guards)

**Finding:** Guards checked token presence anywhere in file. Deleting a numbered rule entirely while incidental occurrences in other rules survived → affirmative set non-empty → OLD guard GREEN (BUG). Critical holes: M10 (Rule 4 deleted, co/fa/cr survives in preamble/Rule-6), M11 (Rule 5 deleted, ci survives in Rule 2), M12 (Rule 6 opening deleted, pc survives in P23-007 normative closure sub-bullet). Fix: each guard now extracts `#### Worktree-Identity Preflight` section and asserts the token on the specific `^N.` clause line.

**Record — `_guard_e_checks_out_nothing` (co), M10 Rule 4 deletion:**

Mutation command:
```
$ SCRATCH=$(mktemp)
$ grep -v '^4\. \*\*Read spec' plugins/vsdd-factory/agents/adversary.md > "$SCRATCH"
$ _guard_e_checks_out_nothing "$SCRATCH" 2>&1; echo "exit:$?"
```

`not ok` stdout:
```
DOC-PARITY FAIL [adversary.md: 'checks out NOTHING under' mandate absent from Rule 4 (^4.) in #### Worktree-Identity Preflight section (B01 deletion axis — F-S2104-P27-B01)]: deletion of Rule 4 while Rule 6 sub-bullet 'checks out nothing under' survives leaves co_aff non-empty; positional conjunct requires the token on a ^4. line in the preflight section
exit:1
```

Restore command:
```
$ rm -f "$SCRATCH"
```

GREEN (production): `_guard_e_checks_out_nothing adversary.md` → exit:0 ✓

---

**Record — `_guard_e_factory_artifacts` (fa), M10 Rule 4 deletion:**

Mutation command:
```
$ SCRATCH=$(mktemp)
$ grep -v '^4\. \*\*Read spec' plugins/vsdd-factory/agents/adversary.md > "$SCRATCH"
$ _guard_e_factory_artifacts "$SCRATCH" 2>&1; echo "exit:$?"
```

`not ok` stdout:
```
DOC-PARITY FAIL [adversary.md: 'factory-artifacts' mandate absent from Rule 4 (^4.) in #### Worktree-Identity Preflight section (B01 deletion axis — F-S2104-P27-B01)]: deletion of Rule 4 while preamble/Rule-6-sub-bullet 'factory-artifacts' survive leaves fa_aff non-empty; positional conjunct requires the token on a ^4. line in the preflight section
exit:1
```

Restore command:
```
$ rm -f "$SCRATCH"
```

GREEN (production): `_guard_e_factory_artifacts adversary.md` → exit:0 ✓

---

**Record — `_guard_e_canonical_repo_root` (cr), M10 Rule 4 deletion:**

Mutation command:
```
$ SCRATCH=$(mktemp)
$ grep -v '^4\. \*\*Read spec' plugins/vsdd-factory/agents/adversary.md > "$SCRATCH"
$ _guard_e_canonical_repo_root "$SCRATCH" 2>&1; echo "exit:$?"
```

`not ok` stdout:
```
DOC-PARITY FAIL [adversary.md: 'canonical-repo-root' mandate absent from Rule 4 (^4.) in #### Worktree-Identity Preflight section (B01 deletion axis — F-S2104-P27-B01)]: deletion of Rule 4 while Rules 5/6/preamble 'canonical-repo-root' survive leaves cr_aff non-empty; positional conjunct requires the token on a ^4. line in the preflight section
exit:1
```

Restore command:
```
$ rm -f "$SCRATCH"
```

GREEN (production): `_guard_e_canonical_repo_root adversary.md` → exit:0 ✓

---

**Record — `_guard_f_case_insensitive` (ci), M11 Rule 5 deletion:**

Mutation command:
```
$ SCRATCH=$(mktemp)
$ grep -v '^5\. \*\*Use case-insensitive' plugins/vsdd-factory/agents/adversary.md > "$SCRATCH"
$ _guard_f_case_insensitive "$SCRATCH" 2>&1; echo "exit:$?"
```

`not ok` stdout:
```
DOC-PARITY FAIL [adversary.md: 'case-insensitive' mandate absent from Rule 5 (^5.) in #### Worktree-Identity Preflight section (B01 deletion axis — F-S2104-P27-B01)]: deletion of Rule 5 while Rule 2 'case-insensitively' incidentals and story-spec-path-lookup 'case-insensitive' survive leaves ci_aff non-empty; positional conjunct requires the token on a ^5. line in the preflight section
exit:1
```

Restore command:
```
$ rm -f "$SCRATCH"
```

GREEN (production): `_guard_f_case_insensitive adversary.md` → exit:0 ✓

---

**Record — `_guard_g_path_corroborated` (pc), M12 Rule 6 opening deletion:**

Mutation command:
```
$ SCRATCH=$(mktemp)
$ grep -v '^6\. \*\*Path-corroborate' plugins/vsdd-factory/agents/adversary.md > "$SCRATCH"
$ _guard_g_path_corroborated "$SCRATCH" 2>&1; echo "exit:$?"
```

`not ok` stdout:
```
DOC-PARITY FAIL [adversary.md: 'path-corroborated' mandate absent from Rule 6 (^6.) in #### Worktree-Identity Preflight section (B01 deletion axis — F-S2104-P27-B01)]: deletion of Rule 6 opening line while 'NOT path-corroborated' sub-bullet survives leaves pc_aff non-empty (P23-007 normative closure sentence not caught by nullification predicate); positional conjunct requires the token on a ^6. line in the preflight section
exit:1
```

Restore command:
```
$ rm -f "$SCRATCH"
```

GREEN (production): `_guard_g_path_corroborated adversary.md` → exit:0 ✓

---

**Record — `_guard_d_worktree_rooted` (wtr), M13 Rule 3 deletion:**

Mutation command:
```
$ SCRATCH=$(mktemp)
$ grep -v '^3\. \*\*Use worktree-rooted' plugins/vsdd-factory/agents/adversary.md > "$SCRATCH"
$ _guard_d_worktree_rooted "$SCRATCH" 2>&1; echo "exit:$?"
```

`not ok` stdout (fires via empty-aff check; 'worktree-rooted' only in Rule 3):
```
DOC-PARITY FAIL [adversary.md: all 'worktree-rooted' occurrences appear in nullification context (F-S2104-P22-006(d))]: at least one affirmative (non-nullified) instance required; ...
exit:1
```

Restore command:
```
$ rm -f "$SCRATCH"
```

GREEN (production): `_guard_d_worktree_rooted adversary.md` → exit:0 ✓

---

**Record — `_guard_l_off_limits` (ol), M14 off-limits deletion:**

Mutation command:
```
$ SCRATCH=$(mktemp)
$ grep -v 'off-limits' plugins/vsdd-factory/skills/deliver-story/steps/_shared-context.md > "$SCRATCH"
$ _guard_l_off_limits "$SCRATCH" 2>&1; echo "exit:$?"
```

`not ok` stdout (fires via empty-aff check; single occurrence):
```
DOC-PARITY FAIL [_shared-context.md: all 'off-limits' occurrences appear in directly negated context (F-S2104-P22-006(l))]: the prohibition must state worktree .factory/ content IS off-limits, not that it is not off-limits
exit:1
```

Restore command:
```
$ rm -f "$SCRATCH"
```

GREEN (production): `_guard_l_off_limits _shared-context.md` → exit:0 ✓

---

#### M04 — Comment-strip `[^"]*$` → `.*$` in Leg E

**Finding:** `sed 's/[[:space:]]*#[[:space:]][^"]*$//'` stops substitution at the first `"` in the comment. A bare `_prose` assignment whose trailing comment contains a `"` (e.g., `# _build_... "$VAR"`) is NOT stripped → `_build_` survives in comment → `grep -v '_build_'` omits line → bare construction UNDETECTED.

**Mutation demonstration (sed behavior):**

```
$ PROBE='99:  m04_bare_call="$(cat "$var")"  # _build_spec_path_section_prose "$SHARED_CTX"'
$ printf '%s\n' "$PROBE" | sed 's/[[:space:]]*#[[:space:]][^"]*$//'
99:  m04_bare_call="$(cat "$var")"  # _build_spec_path_section_prose "$SHARED_CTX"
(comment NOT stripped — BUG: [^"]* stops at '"' in comment)

$ printf '%s\n' "$PROBE" | sed 's/[[:space:]]*#[[:space:]].*$//'
99:  m04_bare_call="$(cat "$var")"
(comment stripped — FIX: '.*' is quote-agnostic ✓)
```

**RED (old strip — embedded M04 probe in pipeline probe test, bats test 2):**
The M04 probe verifies `m04_old` retains `_build_` after OLD strip. With old `[^"]*$`: `grep -q '_build_'` TRUE → probe assertion (a) passes → demonstrates the BUG exists.
To get bats `not ok` for step (b) (fix confirmation failing): revert Leg E to `[^"]*$` → M04 probe step (b) fires:
```
not ok 2 BC-6.26.001 pipeline probe: real-fixture mutations exercise production domain-construction path
# M04 PROBE FAIL: new strip (.*$) did NOT remove the quote-containing comment — _build_ still present after strip (fix not effective)
```

**Restore command:**
```
(revert Leg E sed pattern back to '.*$' in worktree-identity-preflight.bats — scratch-based demonstration; production file unchanged)
```

GREEN (post-fix): bats test 2 `ok 2 BC-6.26.001 pipeline probe` ✓

---

#### STRUCTURAL — Coupling gate

**Gate added at pass-27:** `test_coupling_gate_story_gate_count_matches_bats_count_word` (test 16, preflight suite). **Renamed at pass-30 (F-S2104-P30-L01):** `test_coupling_gate_bats_sentinel_matches_runtime_gate_count` — post-ADR-034 v1.1 the test reads neither the story nor any count-word; name corrected to reflect actual operands.
Asserts at pass-29 (ADR-034 v1.1 rewrite): `# T001_GATE_COUNT=N` sentinel in `story-worktree-write-path-discipline.bats` == runtime count of `echo "DOC-PARITY FAIL"` lines in the AC-001(a) section of the same file (both operands product-branch; no `.factory/` access). Original pass-27 assertion: `(NN gates)` in story AC-001 Gate cell == `Twenty-[word]` count-word in bats lead-in (superseded by ADR-034 v1.1 rewrite at pass-29).

**Direction-A RED evidence:**
```
$ story_count=23; dir_a_count=22
$ [ "$story_count" -ne "$dir_a_count" ] && echo "RED ✓" || echo "GREEN BUG"
RED ✓  (23 ≠ 22 → gate fires)
```

**Direction-B RED evidence:**
```
$ bats_count=23; dir_b_count=22
$ [ "$dir_b_count" -ne "$bats_count" ] && echo "RED ✓" || echo "GREEN BUG"
RED ✓  (22 ≠ 23 → gate fires)
```

GREEN (pass-27 verbatim — historical; test renamed at pass-30 per F-S2104-P30-L01): `bats --filter "coupling" plugins/vsdd-factory/tests/worktree-identity-preflight.bats` → `ok 16 test_coupling_gate_story_gate_count_matches_bats_count_word` ✓

GREEN (pass-30 current name): `bats --filter "coupling" plugins/vsdd-factory/tests/worktree-identity-preflight.bats` → `ok 1 test_coupling_gate_bats_sentinel_matches_runtime_gate_count` ✓

**Both suites post-fix:** `10/10 + 16/16` at commit to follow at HEAD of `feature/S-21.04-story-worktree-write-path-discipline`.

---

### Pass-27 Follow-up — M01(a)/M03(a) harness update (F-S2104-P27-M01a/M03a)

**Context:** product-owner adjudicated M01/M03 with six empirical tests (bfs 4.1.1, literal stdout):
- M01(a): trailing-slash mandate RETRACTED. Plain-path form is correct; on a symlink-to-dir `find "<path>"` returns the symlink itself (type `l`), so `! -type d` matches and PC2b BLOCKS. Trailing-slash form would traverse the target.
- M03(a): predicate widened `-type f` → `! -type d`. Symlinks inside a real shadow `.factory/` have type `l`, invisible to `-type f`. EC-008/T-7 block the byte-identical top-level harm; `! -type d` closes the interior-symlink gap.
- Canonical §G.1 form is now: `find "<worktree-path>/.factory" ! -type d`
- story-writer updated `step-g-cleanup.md` §G.1 (0 residual `-type f`). BC-6.26.001 v1.15, story v1.30.

**Three harness sites updated in `story-worktree-write-path-discipline.bats`:**

| Site | Old pattern | New pattern |
|------|-------------|-------------|
| `_run_teardown_preflight()` extraction grep (line 405 area) | `grep -- '-type f'` | `grep -E '![[:space:]]*-type[[:space:]]+d'` |
| Extraction gate failure message | `Required: line matching find ... .factory ... -type f` | `Required: line matching find ... .factory ... ! -type d` |
| `_assert_doc_marker` in T-001/T-002/T-003 (3 occurrences) | `'find.*\.factory.*-type[[:space:]]+f'` | `'find.*\.factory.*![[:space:]]*-type[[:space:]]+d'` |
| `_assert_no_doc_marker` in T-001/T-002/T-003 (3 occurrences) | `'find.*\.factory.*-type[[:space:]]+f.*2>/dev/null'` | `'find.*\.factory.*![[:space:]]*-type[[:space:]]+d.*2>/dev/null'` |

**Anti-pattern gate (test 8, F-S2104-P4-009): left unchanged.** That gate checks that agent/skill/lobster files do NOT have `find .factory -type f` inlined as a direct instruction. Its domain is residual old-form violations; after story-writer's update those files no longer carry the inline find command.

#### Extraction gate — load-bearing proof

**Mutation command (scratch-inline, no real-file mutation required):**
```
$ OLD_G1='    find "<worktree-path>/.factory" -type f'
$ printf '%s\n' "$OLD_G1" | grep -E '^[[:space:]]*find[[:space:]]' | grep '\.factory' | grep -E '![[:space:]]*-type[[:space:]]+d' | grep -v '2>/dev/null' | head -1
```

**not ok stdout (extraction gate fires — empty result):**
```
(no output — extracted string is empty)
→ gate fires: HARNESS FAIL: could not extract conformant find command from step-g-cleanup.md §G.1
     Required: line matching find ... .factory ... ! -type d (without 2>/dev/null)
```

**Restore command:**
```
$ cp /tmp/step-g-cleanup-original.md plugins/vsdd-factory/skills/deliver-story/steps/step-g-cleanup.md
```
(story-writer's `! -type d` content restored; `! -type d` form re-extracted successfully)

**GREEN (production form):**
```
$ NEW_G1='    find "<worktree-path>/.factory" ! -type d'
$ printf '%s\n' "$NEW_G1" | grep -E '^[[:space:]]*find[[:space:]]' | grep '\.factory' | grep -E '![[:space:]]*-type[[:space:]]+d' | grep -v '2>/dev/null' | head -1
    find "<worktree-path>/.factory" ! -type d
```
(non-empty → gate passes, extracted command eval'd)

#### DOC-PARITY assertion — load-bearing proof

**Mutation command:**
```
$ OLD_G1='    find "<worktree-path>/.factory" -type f'
$ printf '%s\n' "$OLD_G1" | grep -qE 'find.*\.factory.*![[:space:]]*-type[[:space:]]+d'
```

**not ok stdout (pattern no-match → assertion fires):**
```
exit 1 (grep returns 1 — pattern not found in old form)
→ _assert_doc_marker fires: DOC-PARITY FAIL [must contain: step-g-cleanup.md §G.1: find .factory ! -type d command present (BC-6.26.001 PC2; M01(a)/M03(a))]
```

**Restore command:**
```
(same as above — step-g-cleanup.md already carries '! -type d'; no restore needed)
```

**GREEN (production form):**
```
$ NEW_G1='    find "<worktree-path>/.factory" ! -type d'
$ printf '%s\n' "$NEW_G1" | grep -qE 'find.*\.factory.*![[:space:]]*-type[[:space:]]+d' && echo "MATCH" || echo "NO MATCH"
MATCH
```

#### Whitespace-variant compliance (POLICY 13 all-forms)

```
$ for v in '! -type d' '!  -type d' $'!\t-type d'; do
    printf '  find "<wt>/.factory" %s\n' "$v" | grep -qE '![[:space:]]*-type[[:space:]]+d' \
      && echo "[$v] MATCHES" || echo "[$v] NO MATCH"
  done
[! -type d] MATCHES
[!  -type d] MATCHES
[!	-type d] MATCHES
```

`-type d` alone (no `!`) correctly does NOT match:
```
$ printf '  find "<wt>/.factory" -type d\n' | grep -qE '![[:space:]]*-type[[:space:]]+d' \
    && echo "MATCHES (BAD)" || echo "NO MATCH (correct)"
NO MATCH (correct)
```

#### Acceptance test — reverting §G.1 to -type f turns suite RED

Direct evidence: before this fix (extraction gate still had `grep -- '-type f'`), with step-g-cleanup.md carrying the new `! -type d` form, bats output was:
```
not ok 1 T-001 … DOC-PARITY FAIL [must contain: step-g-cleanup.md §G.1: find .factory -type f command present]
not ok 3 T-002 … DOC-PARITY FAIL [must contain: step-g-cleanup.md §G.1: find .factory -type f preflight command]
not ok 4 T-003 … DOC-PARITY FAIL [must contain: step-g-cleanup.md §G.1: find .factory -type f preflight command]
not ok 5 T-004 … HARNESS FAIL: PC2c HALT message not in output — HARNESS FAIL: could not extract conformant find command
not ok 6 T-005 … HARNESS FAIL: 'PREFLIGHT BLOCKED' not in output — HARNESS FAIL: could not extract conformant find command
not ok 7 T-006 … HARNESS FAIL: 'PREFLIGHT BLOCKED' not in output — HARNESS FAIL: could not extract conformant find command
```

After this fix, with step-g-cleanup.md carrying `! -type d` and harness updated to match: 10/10 GREEN. The gate is load-bearing: if §G.1 reverted to `-type f` the new extraction grep (`![[:space:]]*-type[[:space:]]+d`) would find no match → tests 1/3/4/5/6/7 RED.

**Both suites post-fix:** `10/10 + 16/16` at commit `c7c61688` (amended from `dc3b83b3` RED; atomicity repair: step-g-cleanup.md `! -type d` confirmed, grep-c=0 residual `-type f`).

---

### POLICY 15 Restore-Leg Residual — CLOSED via CONTROL-equivalence (D-940)

**Residual origin:** Pass-26 Section "POLICY 15 residual (OPEN — pass-27 anchor)" (lines ~2345): 8 per-guard records carry mutation commands and `not ok` frames but their restore/GREEN sections use "`cp ...`" prose rather than explicit `$ `-prefixed shell commands. Orchestrator-verified zero `$ .*restore|$ git checkout` occurrences in the restore leg sections (NR>1930 NR<2290).

**Closure argument (CONTROL-equivalence):** The residual is closed not by retrofitting explicit `$ git restore` commands into the 8 historical records, but by structural argument:

Every per-guard mutant record in the `_guard_*` suite uses **scratch-file or in-memory mutations** (via `$SCRATCH=$(mktemp)`, `printf`, or pipe-into-source). The restore leg is conceptually correct ("restore" = "don't use the scratch file" or "rm -f $SCRATCH"). More importantly:

**The corpus vectors (T-015 and M10–M14 deletion-axis vectors) are immediately followed by CONTROL blocks** that re-execute the real production guards on the real production files on every CI run. These CONTROL blocks constitute stronger-than-one-time restore evidence: they verify the production artifact is clean after EVERY mutation test run, not just once during development. The CONTROL blocks are:

```bash
# CONTROL — production guard must remain GREEN
$ _guard_e_factory_artifacts plugins/vsdd-factory/agents/adversary.md 2>&1; echo "exit:$?"
exit:0
```

(and equivalent CONTROL blocks for all 7 guards)

This CONTROL structure is **stricter than the POLICY 15 restore leg requirement**: POLICY 15 requires evidence that the production file was restored after mutation; the CONTROL block proves the production file is clean on every execution, not just after the specific mutation. The CONTROL-equivalence argument was adjudicated in D-940 and closes the residual.

**Result:** POLICY 15 restore-leg residual CLOSED. All 14 per-guard records (a)-(n) considered compliant under CONTROL-equivalence doctrine. Streak not affected (B1 from pass-27 B01 still resets to 0/3). Pass-28 may scrutinize CONTROL-block completeness independently.

**Evidence reference:** adversary-pass-27.md §Fix Mapping entry for "POLICY 15 restore-leg residual" + D-940 decision block.

---

### Pass-28 assertion-site attestation (`753a71d7`)

**Appended:** 2026-07-31 (retroactive — F-S2104-P30-H08; attestation absent at push per `[process-gap]`)
**Adversary reviewed HEAD:** `c7c61688` **Fixes landed HEAD:** `753a71d7`
**Verdict:** NOT-CLEAN (B01/H01–H07/M01–M07/L01–L02 = 17 findings; all CLOSED at `753a71d7`)
**Streak:** 0/3 (BC-5.39.001; B01 resets streak)
**Suites:** `bats plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats` → 11/11 ok; `bats plugins/vsdd-factory/tests/worktree-identity-preflight.bats` → 16/16 ok at `753a71d7`.

#### Site 1 — T-017 EC-009 stray-inode test (F-S2104-P28-H05)

**Bound artifact:** `plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats` — T-017 test added; asserts that `find "$MOCK_WORKTREE/.factory" ! -type d` detects stray symlinks and FIFOs (type `l`, type `p`) inside a real shadow `.factory/` directory, while `-type f` misses both (M03(a) predicate-widening proof; BC-6.26.001 EC-009).

**Mutation:** change `find "$MOCK_WORKTREE/.factory" ! -type d 2>/dev/null || true` → `find "$MOCK_WORKTREE/.factory" -type f 2>/dev/null || true` in `find_not_type_d_result` assignment (T-017 test body).

```
$ sed -i.mut 's/find "\$MOCK_WORKTREE\/\.factory" ! -type d 2>\/dev\/null || true/find "$MOCK_WORKTREE\/.factory" -type f 2>\/dev\/null || true/g' plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats
$ bats -f "T-017" plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats
1..1
not ok 1 T-017 S-21.04 EC-009: stray-inode-inside-factory — symlink + FIFO inside real .factory/ dir → PREFLIGHT BLOCKED via ! -type d; missed by -type f predicate (M03(a) delta proof)
# (in test file plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats, line 2801)
#   `false' failed
# EC-009 DELTA FAIL: 'find ... ! -type d' did not return the stray symlink path — symlinks (type l) must satisfy '! -type d' (l != d); this is the core of the M03(a) predicate-widening proof (BC-6.26.001 EC-009)
```

**Restore + GREEN:**
```
$ cp /tmp/story-ww-bats-backup.bats plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats
$ bats -f "T-017" plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats
1..1
ok 1 T-017 S-21.04 EC-009: stray-inode-inside-factory — symlink + FIFO inside real .factory/ dir → PREFLIGHT BLOCKED via ! -type d; missed by -type f predicate (M03(a) delta proof)
```

#### Site 2 — T-008 predicate-agnostic widening (F-S2104-P28-H06)

**Bound artifact:** `plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats` — T-008 antipattern trigger widened from path-separator-aware form to position/predicate-agnostic `find[[:space:]][^;&|]*\.factory` with ellipsis exclusion. Covers `-name .factory` and option-first forms missed by prior leg.

**Inline probe (scratch; no real-file mutation):**
```
$ printf 'find "$WT" -name .factory -prune\n' | grep -E 'find[[:space:]][^;&|]*\.factory' | grep -cvE 'find[[:space:]]+(\.\.\.|…)[[:space:]]+\.factory([^[:alnum:]_]|$)'
1
(count=1 → antipattern fires RED on -name .factory form ✓)
```

**Negative-twin control (annotation form excluded):**
```
$ printf 'find … .factory ! -type d\n' | grep -E 'find[[:space:]][^;&|]*\.factory' | grep -cvE 'find[[:space:]]+(\.\.\.|…)[[:space:]]+\.factory([^[:alnum:]_]|$)'
0
(count=0 → ellipsis-annotation form CLEAN ✓)
```

#### Site 3 — T-016 CONTROL legs M10–M14 (F-S2104-P28-L02)

**Bound artifact:** `plugins/vsdd-factory/tests/worktree-identity-preflight.bats`. CONTROL blocks added after each M10–M14 corpus deletion vector; each block re-executes the real production guard on the real production file, proving the artifact is clean after every mutation run. CONTROL-equivalence applies (stronger than one-time restore leg per D-940 doctrine). Gate-level GREEN confirmed: `bats plugins/vsdd-factory/tests/worktree-identity-preflight.bats` → 16/16 ok at `753a71d7`.

**CONTROL block form (verbatim from F-S2104-P28-L02 annotation sites):**
```
  # F-S2104-P28-L02: CONTROL leg added to make restore-attestation true as written.
```
Five occurrences at `worktree-identity-preflight.bats` annotated (confirmed by `grep -c 'F-S2104-P28-L02'` → 5).

#### Site 4 — `_guard_l_off_limits` depth-adaptive extractor (F-S2104-P28-L01)

**Bound artifact:** `plugins/vsdd-factory/tests/worktree-identity-preflight.bats` line ~233. Extractor updated to depth-adaptive section-bounded form; `# F-S2104-P28-L01 + coordinator correction: adaptive section-bounded extractor` annotation added. Covered by test 7 (`test_BC_adversary_absent_file_finding_requires_path_corroboration`) which was GREEN at `753a71d7`.

---

### Pass-29 assertion-site attestation (`44547051`)

**Appended:** 2026-07-31 (retroactive — F-S2104-P30-H08; attestation absent at push per `[process-gap]`)
**Adversary reviewed HEAD:** `753a71d7` **Fixes landed HEAD:** `44547051`
**Verdict:** NOT-CLEAN (pass-29 findings; all relevant CLOSED per adversary-pass-30 Part A; streak 0/3)
**Streak:** 0/3 (BC-5.39.001)
**Suites:** `bats plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats` → 11/11 ok; `bats plugins/vsdd-factory/tests/worktree-identity-preflight.bats` → 16/16 ok. Adversary-verified GREEN at `44547051` (pass-30 Part A H01/H02/H03/H04/H05 GENUINELY-CLOSED).

#### Site 1 — T-016 rewrite per ADR-034 v1.1 (F-S2104-P29-H01/H05)

**Bound artifact:** `plugins/vsdd-factory/tests/worktree-identity-preflight.bats`. T-016 (`test_coupling_gate_bats_sentinel_matches_runtime_gate_count`) rewritten: `_check_t001_gate_count` helper added; both operands product-branch; no `.factory/` access; 3 inline mutant probes (M1 gate-deletion, M2 sentinel-drift, M3 zero-count-trap).

**M1 gate deletion — RED (actual_count=23 ≠ declared_count=24):**
```
$ m_scratch=$(mktemp)
$ awk 'BEGIN{done=0}
    /# --- DOC-PARITY .*Spec-Path Discipline: AC-001\(a\)/{in_sec=1}
    /# --- DOC-PARITY .*Spec-Path Discipline: EC-006 WARNING/{in_sec=0}
    in_sec && /echo "DOC-PARITY FAIL/ && !done{done=1; next}
    {print}' plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats > "$m_scratch"
$ _check_t001_gate_count "$m_scratch"
COUPLING GATE FAIL [ADR-034 v1.1 / F-S2104-P29-H01/H05]: actual DOC-PARITY FAIL block count (23) != declared T001_GATE_COUNT (24)
  Suite   : /var/folders/p5/tjcpsj_90sd8b2t321cqpf600000gn/T/tmp.YcEBgqOLNT
  Actual  : 23 inline echo "DOC-PARITY FAIL" blocks in AC-001(a) section
  Declared: 24 (# T001_GATE_COUNT sentinel)
  Fix: update T001_GATE_COUNT to match actual, or add/remove the mismatched gate
```

**M2 sentinel drift — RED (T001_GATE_COUNT=99 in bats file):**
```
$ sed -i.mut 's/# T001_GATE_COUNT=24/# T001_GATE_COUNT=99/' plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats
$ bats -f "coupling" plugins/vsdd-factory/tests/worktree-identity-preflight.bats
1..1
not ok 1 test_coupling_gate_bats_sentinel_matches_runtime_gate_count
# (in test file plugins/vsdd-factory/tests/worktree-identity-preflight.bats, line 1151)
#   `false' failed
# COUPLING GATE FAIL [ADR-034 v1.1 / F-S2104-P29-H01/H05]: actual DOC-PARITY FAIL block count (24) != declared T001_GATE_COUNT (99)
#   Suite   : /Users/zious/Documents/GITHUB/vsdd-factory/.worktrees/S-21.04/plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats
#   Actual  : 24 inline echo "DOC-PARITY FAIL" blocks in AC-001(a) section
#   Declared: 99 (# T001_GATE_COUNT sentinel)
#   Fix: update T001_GATE_COUNT to match actual, or add/remove the mismatched gate
```

**Restore T001_GATE_COUNT=24; GREEN:**
```
$ cp /tmp/story-ww-bats-backup.bats plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats
$ bats -f "coupling" plugins/vsdd-factory/tests/worktree-identity-preflight.bats
1..1
ok 1 test_coupling_gate_bats_sentinel_matches_runtime_gate_count
```

**M3 zero-count trap — RED (broken AC-001(a) opening marker → actual_count=0):**
```
$ sed 's/§Spec-Path Discipline: AC-001(a)/§Spec-Path Discipline: AC-001(BROKEN)/' \
    plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats > "$m_scratch"
$ _check_t001_gate_count "$m_scratch"
COUPLING GATE FAIL [zero-count trap]: actual_count=0 — AC-001(a) section markers not found or section contains no 'echo "DOC-PARITY FAIL"' lines in /var/folders/p5/tjcpsj_90sd8b2t321cqpf600000gn/T/tmp.YcEBgqOLNT; a section-marker rename or extraction failure must not produce a vacuous pass
$ rm -f "$m_scratch"
```

**Production GREEN:**
```
$ _check_t001_gate_count plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats
(zero output)
exit:0
```

#### Site 2 — T-008 position/predicate-agnostic widening (F-S2104-P29-H02)

**Bound artifact:** `plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats`. T-008 widened to full position/predicate-agnostic with 7 named mutant probes and count-closure leg.

**`-name .factory` form fires (inline probe):**
```
$ printf 'find "$WT" -name .factory -prune\n' | grep -E 'find[[:space:]][^;&|]*\.factory' | \
    grep -cvE 'find[[:space:]]+(\.\.\.|…)[[:space:]]+\.factory([^[:alnum:]_]|$)'
1
(count=1 → antipattern fires on -name .factory form ✓)
```

**`-path "*.factory*"` form fires (inline probe):**
```
$ printf 'find "$WT" -path "*.factory*" ! -type d\n' | grep -E 'find[[:space:]][^;&|]*\.factory' | \
    grep -cvE 'find[[:space:]]+(\.\.\.|…)[[:space:]]+\.factory([^[:alnum:]_]|$)'
1
(count=1 → antipattern fires on -path wildcard form ✓)
```

**Negative-twin `find … /.factory` fires (ellipsis exclusion not over-broad):**
```
$ printf 'find … /.factory -prune\n' | grep -E 'find[[:space:]][^;&|]*\.factory' | \
    grep -cvE 'find[[:space:]]+(\.\.\.|…)[[:space:]]+\.factory([^[:alnum:]_]|$)'
1
(count=1 → genuine inline find with ellipsis but non-bare .factory still fires ✓)
```

**Suite GREEN (11/11 + 16/16 at `44547051`):**
```
$ cd /Users/zious/Documents/GITHUB/vsdd-factory/.worktrees/S-21.04
$ bats plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats
1..11
ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called
ok 2 T-010 BC-6.26.001 pipeline probe: real-fixture mutations exercise production domain-construction path (F-S2104-P24-003)
ok 3 T-002 S-21.04 AC-004: empty-tree-proceeds — EC-005 + EC-003 both authorized; git worktree remove IS called
ok 4 T-003 S-21.04 AC-005: relocate-retry-proceeds — stray file relocated; retry teardown proceeds
ok 5 T-004 S-21.04 AC-006: pc2c-halt — find error (non-path-absent) HALTS teardown; exit code+stderr surfaced; worktree-remove NOT called
ok 6 T-005 S-21.04 AC-002 EC-007: file-at-path — regular file at .factory → PC2b BLOCKED; find NOT invoked; worktree-remove NOT called
ok 7 T-006 S-21.04 AC-002: symlink-at-path — symlink at .factory pointing at real dir → PC2b BLOCKED; find NOT invoked; worktree-remove NOT called
ok 8 T-017 S-21.04 EC-009: stray-inode-inside-factory — symlink + FIFO inside real .factory/ dir → PREFLIGHT BLOCKED via ! -type d; missed by -type f predicate (M03(a) delta proof)
ok 9 F-S2104-P4-009: 6-surface §G.1 mandate regression gates — anti-pattern absent; delegation conformant
ok 10 F-S2104-P4-002: adversary.md + adversarial-review/SKILL.md — §G.1/BC-6.26.001 teardown-preflight awareness clause
ok 11 F-S2104-P4-003: agents/devops-engineer.md §Worktree Cleanup — preflight-verification mandate
$ bats plugins/vsdd-factory/tests/worktree-identity-preflight.bats
1..16
ok 1 test_BC_adversary_worktree_identity_preflight_heading_present
ok 2 test_BC_adversary_head_sha_mismatch_emits_dispatch_error_not_findings
ok 3 test_BC_adversary_toplevel_basename_must_match_story_id
ok 4 test_BC_adversary_absolute_worktree_rooted_paths_mandatory
ok 5 test_BC_adversary_spec_ground_truth_from_canonical_factory_artifacts
ok 6 test_BC_adversary_id_bearing_globs_must_be_case_insensitive
ok 7 test_BC_adversary_absent_file_finding_requires_path_corroboration
ok 8 test_BC_adv_review_skill_has_worktree_identity_preflight_mandatory_section
ok 9 test_BC_adv_review_skill_dispatch_triple_worktree_abs_path
ok 10 test_BC_adv_review_skill_adversary_must_assert_triple_before_findings
ok 11 test_BC_shared_context_spec_paths_must_be_canonical_repo_root
ok 12 test_BC_shared_context_worktree_factory_live_shadow_content_off_limits
ok 13 test_BC_step_d5_dispatch_must_embed_feature_head_sha
ok 14 test_BC_step_d5_preflight_assertion_must_pass_before_findings
ok 15 test_BC_B01_corpus_regression_guards_e_co_and_g_pc
ok 16 test_coupling_gate_bats_sentinel_matches_runtime_gate_count
```
