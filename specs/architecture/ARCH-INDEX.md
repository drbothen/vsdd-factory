---
document_type: architecture-index
level: L3
version: "2.05"
status: accepted
producer: architect
timestamp: 2026-05-14T00:00:00Z
last_amended: 2026-05-14
phase: F5
inputs:
  - .factory/phase-0-ingestion/pass-8-final-synthesis.md
  - .factory/phase-0-ingestion/pass-1-architecture.md
  - .factory/legacy-design-docs/2026-04-24-v1.0-factory-plugin-kit-design.md
  - .factory/phase-0-ingestion/pass-0-inventory.md
  - .factory/phase-0-ingestion/pass-2-domain-model.md
  - .factory/phase-0-ingestion/pass-5-conventions.md
traces_to: phase-1-spec-crystallization
deployment_topology: single-service
changelog:
  - date: 2026-05-14
    change: "v2.05 (2026-05-14; E-10 pass-14 D-470 mandatory HIGH closures + D-471 asymptotic-acceptance seal — F-PASS14-001 closure (compute-input-hash mechanical execution against BC-3.04.001; D-468 false tool-unavailable claim corrected: tool present, hash 5d2b1b3 confirmed; POLICY 18 self-applied); F-PASS14-002 closure (LL-3 strict-form inline stdout at D-466/D-467/D-469 attestation sites; replaced narrative + git-pointer-forwarding; 4-row carve-out list verified; POLICY 15 self-applied). D-471 ratifies E-10 sub-cycle asymptotic-acceptance analogous to F5 D-386 Option C + human direction 2026-05-14: 6 consecutive passes (9-14) at [4-9] band; 5th META layer spawned; remaining 6 findings DEFERRED to S-15.03 PRIORITY-A automation wave; NITPICK_ONLY counter FROZEN at 0/3; resumption gate = S-15.03 PRIORITY-A lint hooks in v1.0-feature-engine-discipline-pass-2. MM gate: global max D-469; D-470+D-471 confirmed next-available. ARCH-INDEX v2.04→v2.05. Refs: D-470, D-471, F-PASS14-001, F-PASS14-002."
  - date: 2026-05-14
    change: "v2.04 (2026-05-14; E-10 pass-13 D-468 fix burst + D-469 seal — closures F-PASS13-001 CRITICAL (D-350→D-466 mechanical replacement 7 files; MM-2 citation-authoring scope), F-PASS13-002 HIGH (NN-2 frontmatter parity E-1 + VP-014), F-PASS13-003 HIGH (LL-3 strict-form retroactive in v2.03 row + D-466/D-467 decision-log rows), F-PASS13-004 MED (BC-3.04.001 input-hash audit), F-PASS13-005 LOW (STATE.md orphan-narrative cleanup). POLICY 13-18 registered (commit b8909832; codifies HH-N/KK-N/LL-N/MM-N/NN-N/OO disciplines from pass-13 §9 combined recommendation). MM gate re-run: global max D-468; D-469 confirmed next-available. LL-3 post-fix verbatim stdout at D-468 burst time preserved in ARCH-INDEX v2.03 row (retroactive annotation) + D-468 commit body. HH-4 STRUCTURAL RESOLUTION carried forward (zero live Observability Sinks production-content violations). NITPICK_ONLY counter 0/3 (CRITICAL resets). Pass-14 dispatch is next — CRITICAL TEST whether POLICY 13-18 gates achieve NITPICK_ONLY or spawn 5th-layer META-class. ARCH-INDEX v2.03→v2.04. Refs: D-468, D-469, POLICY 13-18."
  - date: 2026-05-13
    change: "v2.03 (2026-05-13; E-10 pass-12 D-466 fix burst + D-467 seal — closures F-2 (E-1 epic body Changelog), F-3+F-6 (7 corpus-wide subsystem-name sites HH-4 sweep), F-1 (5 BCs KK-2 body audit-trail rows for invisible D-464 touch), F-5 (KK-2 tripartite parity sync). MM cross-cycle namespace gate INVOKED (D-466 confirmed next-available globally past D-465); NN epic+story+VP frontmatter parity gate INVOKED. HH-4 post-fix verification (LL-3 strict-form; retroactively applied at D-468 fix burst 2026-05-14 per F-PASS13-003): $ grep -rnE 'SS-03[^A-Za-z0-9]*Observability Sinks|SS-03[^|]*Observability Sinks|Observability Sinks subsystem' .factory/specs/ .factory/stories/ | grep -v 'changelog|superseded|SUPERSEDED|adv-cycle|decision-log|burst-log|INDEX.md' → stdout: .factory/specs/dtu-assessment.md:205:| 1.1 | 2026-05-13 | architect | D-466 E-10 pass-12 fix burst F-3+F-6 closure (HH-4 regex-alternation discipline): SS-03 subsystem name `Observability Sinks` → ... (changelog audit-trail row — legitimate carve-out); .factory/specs/behavioral-contracts/ss-03/BC-3.04.001.md:136:| v1.2 | 2026-05-13 | architect | D-462 ... Architecture Module row corrected from stale `SS-03 (Observability Sinks)` ... (changelog audit-trail row — legitimate carve-out); .factory/specs/architecture/SS-03-observability-sinks.md:36:The Observability Sinks subsystem provides the multi-sink event fan-out pipeline (SUPERSEDED-file prose — legitimate carve-out); .factory/stories/epics/E-1-dispatcher-foundation.md:82:| 1.1 | 2026-05-13 | architect | D-464 ... `SS-03 (Observability Sinks)` → ... (changelog audit-trail row — legitimate carve-out). Zero live production-content violations. KK-2 tripartite: all 5 BCs version/last_amended/modified[]/top-changelog-row aligned 2026-05-13. NITPICK_ONLY counter 0/3 (HIGH verdict resets, no advance); pass-13 dispatch is next — CRITICAL TEST of HH-4/KK-2/LL-2/MM/NN discipline efficacy on trend-rebound resolution. ARCH-INDEX v2.02→v2.03. Refs: D-466, D-467."
  - date: 2026-05-13
    change: "v2.02 (2026-05-13; F-CRIT-001 closure — D-NNN renumbering corrigendum): Brownfield D-344→D-460, D-345→D-461, D-346→D-462, D-347→D-463, D-348→D-464, D-349→D-465. Original brownfield assignments collided with F5-cycle D-344..D-349 (assigned 2026-05-07) per POLICY 1 (append_only_numbering). All ARCH-INDEX changelog rows, spec files, BC files, VP files, story files, STATE.md, and brownfield decision-log updated atomically. Pre-renumbering grep: 221 hits across 25 brownfield artifact files. Post-renumbering grep: 0 brownfield hits on D-34[4-9] (F5-cycle D-34[4-9] cites preserved: 29 hits). ARCH-INDEX v2.01→v2.02. Refs: F-CRIT-001, D-460, D-461, D-462, D-463, D-464, D-465."
  - date: 2026-05-13
    change: "v2.01 (2026-05-13; E-10 pass-11 D-464 fix burst + D-465 seal — closures F-1/F-2/F-3/F-4/F-5 (1H+2M+2L). F-1 frontmatter parity 5 BCs (KK discipline); F-2 SS-03-observability-sinks lines 72+148 dispatcher_trace_id annotation; F-3 cross-spec SS-03 canonical-name sweep (E-1 epic + S-4.05 3 sites); F-4 VP-014 formal-proof-only intent + §Test Evidence scope annotation; F-5 VP-014 bad_version harness fix lines 56+62. HH-3 multi-axis pre-fix grep INVOKED (4 predicates: dispatcher_trace_id + SS-03-Observability-Sinks + bad_version arrays + schema_version=1) — captured stdout inline in D-464 commit body per LL discipline (brownfield analog of F5 D-449(a)). HH-3 post-fix grep INVOKED — zero non-excluded rows for all 4 predicates. KK frontmatter parity gate INVOKED — 5 BCs synced same-burst. NITPICK_ONLY counter 0/3 (HIGH verdict resets, no advance); pass-12 dispatch is next. Refs: D-464, D-465. [D-NNN corrigendum 2026-05-13: originally cited D-348+D-349; renumbered to D-464+D-465 per F-CRIT-001 resolution.]"
  - date: 2026-05-13
    change: "v2.00 (2026-05-13; E-10 pass-10 D-462 fix burst + D-463 seal — 11 spec files updated for findings F-1/F-2/F-3/F-4 (ADR-004 v1.3 line 116 + VP-014 1.1 + business-rules BR-14 + prd.md 3 sites + BC-4.04.005/4.05.005/4.07.004/4.08.003 + BC-3.04.001 v1.2 + DI-017 v1.2 + SS-03-observability-sinks 1.2). Literal-shell-execution-evidence per F5 D-449(a) applied: pre-fix HH-2 grep surfaced 3 sites beyond pass-10 §8 enumeration; post-fix II-2 grep returns zero stale REGISTRY_SCHEMA_VERSION = 1 precondition rows (excluding intentional historical-quote and negative-test contexts). NITPICK_ONLY counter 0/3 (HIGH verdict resets, no advance); pass-11 dispatch is next. Refs: D-462, D-463. [D-NNN corrigendum 2026-05-13: originally cited D-346+D-347; renumbered to D-462+D-463 per F-CRIT-001 resolution.]"
  - date: 2026-05-13
    change: "v1.99 (2026-05-13; E-10 pass-9 D-460 fix burst + D-461 seal): Brownfield E-10 pass-9 closure — D-460 architect fix burst (4430483d) closed F-1/F-2/F-3 HIGH + F-4 MED + F-5 LOW. Arch modifications: SS-01-hook-dispatcher.md v1.2→v1.3 (dispatcher_trace_id annotation at lines 39/48/59/60/122/144 + REGISTRY_SCHEMA_VERSION=2 annotation + Observability Sinks annotation); SS-02-hook-sdk.md v1.1→v1.2 (SDK exports `dispatcher_trace_id()` only annotation at lines 53/91/168); ADR-011-dual-hook-routing-tables.md (wire-field correction at line 239 + changelog row); ADR-004-toml-config.md v1.1→v1.2 (lines 44/96 schema_version=2 annotation — cross-cycle F2 ADR-019 propagation per user direction). D-461 seal: input-hash recomputed for SS-01 + SS-02 (replaced [pending-recompute] markers → 39de903). SDK API surface verified via literal-shell read of crates/hook-sdk/src/{ffi,host}.rs — SDK exports `dispatcher_trace_id()` only; WIRE renamed to `trace_id` per DI-017 v1.1; SDK API intentionally unchanged. F5 D-449(a) literal-shell-execution-evidence discipline applied retroactively: all 5 closure gates returned zero rows post-fix. [Cross-cycle annotation F-4: ADR-019 schema_version=2 (F2 cycle) propagated to E-10 ADR-004 per user direction 2026-05-13.] ARCH-INDEX v1.98→v1.99. Refs: E-10-pass-9-F-1, E-10-pass-9-F-2, E-10-pass-9-F-3, E-10-pass-9-F-4, E-10-pass-9-F-5, D-460, D-461. [D-NNN corrigendum 2026-05-13: originally cited D-344+D-345; renumbered to D-460+D-461 per F-CRIT-001 resolution.]"
  - date: 2026-05-13
    change: "v1.98 (2026-05-13; pass-74 fix burst per D-454 + D-404 unconditional): Acknowledges decision range D-389..D-454 (inclusive; literal acknowledgment per D-415(c); sample; see decision-log.md for full range D-389..D-454 per D-441(c)+D-442(c) sample-vs-exhaustive citation policy). Per D-404 unconditional: index acknowledges D-454(a/b/c/d/e) by literal ID (no spec content change in this changelog entry). Refs: ADV-EDP1-P74-CRIT-001, ADV-EDP1-P74-HIGH-001/002/003/004, ADV-EDP1-P74-MED-001/002/003, ADV-EDP1-P74-LOW-001, PG-P74-001/002/003, D-454. [O-P74-001/002/003 trimmed per D-449(d)(i): observations are discovery-tier, not Refs-scope.] META-LEVEL-29 CANDIDATE CONFIRMED; L-EDP1-066 65th-layer 35th-consecutive multi-axis; gate-against-canonical-registry-uses-coarser-granularity + freshness-scope-extension-codified-but-re-execution-evidence-narrative + codification-references-storage-that-doesn't-exist + tri-way-block-label-canonical-form-drift + freshness-temporal-scope-narrower-than-document-edit-window ply (sample; see decision-log.md for full range D-389..D-454 per D-441(c)+D-442(c) sample-vs-exhaustive citation policy). ARCH-INDEX v1.97→v1.98."
  - date: 2026-05-13
    change: "v1.97 (2026-05-13; pass-73 fix burst per D-453 + D-404 unconditional): Acknowledges decision range D-389..D-453 (inclusive; literal acknowledgment per D-415(c); sample; see decision-log.md for full range D-389..D-453 per D-441(c)+D-442(c) sample-vs-exhaustive citation policy). Per D-404 unconditional: index acknowledges D-453(a/b/c/d/e) by literal ID (no spec content change in this changelog entry). Refs: ADV-EDP1-P73-CRIT-001, ADV-EDP1-P73-HIGH-001/002/003/004, ADV-EDP1-P73-MED-001/002/003, ADV-EDP1-P73-LOW-001, PG-P73-001/002/003, D-453. [O-P73-001/002/003 trimmed per D-449(d)(i): observations are discovery-tier, not Refs-scope.] META-LEVEL-28 CANDIDATE CONFIRMED; L-EDP1-065 64th-layer 34th-consecutive multi-axis; meta-rule-codified-with-mechanical-gate-AND-explicit-PRESCRIBED_SITES-enumeration-but-PRESCRIBED_SITES-list-itself-INCOMPLETE-OR-freshness-gate-scope-NARROWER-than-validated-gate-scope-OR-site-class-labels-INFORMAL ply (sample; see decision-log.md for full range D-389..D-453 per D-441(c)+D-442(c) sample-vs-exhaustive citation policy). ARCH-INDEX v1.96→v1.97."
  - date: 2026-05-13
    change: "v1.96 (2026-05-13; pass-72 fix burst per D-452 + D-404 unconditional): Acknowledges decision range D-389..D-452 (inclusive; literal acknowledgment per D-415(c); sample; see decision-log.md for full range D-389..D-452 per D-441(c)+D-442(c) sample-vs-exhaustive citation policy). Per D-404 unconditional: index acknowledges D-452(a/b/c/d/e) by literal ID (no spec content change in this changelog entry). Refs: ADV-EDP1-P72-CRIT-001, ADV-EDP1-P72-HIGH-001/002/003/004, ADV-EDP1-P72-MED-001/002/003, ADV-EDP1-P72-LOW-001, PG-P72-001/002/003, D-452. [O-P72-001/002/003 trimmed per D-449(d)(i): observations are discovery-tier, not Refs-scope.] META-LEVEL-27 CANDIDATE CONFIRMED; L-EDP1-064 63rd-layer 33rd-consecutive multi-axis; literal-shell-derivation-gate-INVOKED-and-captured-stdout-correct-but-OUTPUT-NOT-PROPAGATED-to-all-prescribed-citation-sites-PLUS-snapshot-staleness-when-document-continues-to-be-edited-AND-gate-scope-narrower-than-rule-scope ply (sample; see decision-log.md for full range D-389..D-452 per D-441(c)+D-442(c) sample-vs-exhaustive citation policy). ARCH-INDEX v1.95→v1.96."
  - date: 2026-05-13
    change: "v1.95 (2026-05-13; pass-71 fix burst per D-451 + D-404 unconditional): Acknowledges decision range D-389..D-451 (inclusive; literal acknowledgment per D-415(c); sample; see decision-log.md for full range D-389..D-451 per D-441(c)+D-442(c) sample-vs-exhaustive citation policy). Per D-404 unconditional: index acknowledges D-451(a/b/c/d/e) by literal ID (no spec content change in this changelog entry). Refs: ADV-EDP1-P71-CRIT-001, ADV-EDP1-P71-HIGH-001/002/003/004, ADV-EDP1-P71-MED-001/002/003, ADV-EDP1-P71-LOW-001, PG-P71-001/002/003, D-451. [O-P71-001/002/003 trimmed per D-449(d)(i): observations are discovery-tier, not Refs-scope.] META-LEVEL-26 CANDIDATE CONFIRMED; L-EDP1-063 62nd-layer 32nd-consecutive multi-axis; rule-codification-prescribing-co-mechanical-application-of-literal-shell-to-N-sibling-gates-without-applying-literal-shell-to-meta-recursion-ack-self-reference ply (sample; see decision-log.md for full range D-389..D-451 per D-441(c)+D-442(c) sample-vs-exhaustive citation policy). ARCH-INDEX v1.94→v1.95."
  - date: 2026-05-13
    change: "v1.94 (2026-05-13; pass-70 fix burst per D-450 + D-404 unconditional): Acknowledges decision range D-389..D-450 (inclusive; literal acknowledgment per D-415(c); sample; see decision-log.md for full range D-389..D-450 per D-441(c)+D-442(c) sample-vs-exhaustive citation policy). Per D-404 unconditional: index acknowledges D-450(a/b/c/d/e) by literal ID (no spec content change in this changelog entry). Refs: ADV-EDP1-P70-CRIT-001, ADV-EDP1-P70-HIGH-001/002/003/004, ADV-EDP1-P70-MED-001/002/003, ADV-EDP1-P70-LOW-001, PG-P70-001/002/003, D-450. [O-P70-001/002/003 trimmed per D-449(d)(i): observations are discovery-tier, not Refs-scope.] META-LEVEL-25 CANDIDATE CONFIRMED; L-EDP1-062 61st-layer 31st-consecutive multi-axis; rule-codification-with-literal-shell-execution-on-PRIMARY-rule-without-co-application-of-same-mechanical-rigor-to-SIBLING-rules-codified-in-same-burst ply (sample; see decision-log.md for full range D-389..D-450 per D-441(c)+D-442(c) sample-vs-exhaustive citation policy). ARCH-INDEX v1.93→v1.94. [Corrigendum pass-72 ADV-EDP1-P72-HIGH-004 + D-452(d): prior '62nd-layer' was incorrect — L-EDP1-062 heading is '61st-layer'; retroactively corrected here.]"
  - date: 2026-05-13
    change: "v1.93 (2026-05-13; pass-69 fix burst per D-449 + D-404 unconditional): Acknowledges decision range D-389..D-449 (inclusive; literal acknowledgment per D-415(c); sample; see decision-log.md for full range D-389..D-449 per D-441(c)+D-442(c) sample-vs-exhaustive citation policy). Per D-404 unconditional: index acknowledges D-449(a/b/c/d/e) by literal ID (no spec content change in this changelog entry). Refs: F-P69-CRIT-001, F-P69-HIGH-001/002/003/004, F-P69-MED-001/002/003, F-P69-LOW-001, PG-P69-001/002/003, D-449. [O-P69-001/002/003 trimmed per D-449(d)(i): observations are discovery-tier, not Refs-scope.] META-LEVEL-24 CANDIDATE CONFIRMED; L-EDP1-061 60th-layer 30th-consecutive multi-axis; rule-codification-via-pseudocode-narrative-without-literal-shell-execution-evidence ply (sample; see decision-log.md for full range D-389..D-449 per D-441(c)+D-442(c) sample-vs-exhaustive citation policy). ARCH-INDEX v1.92→v1.93."
  - date: 2026-05-13
    change: "v1.92 (2026-05-13; pass-68 fix burst per D-448 + D-404 unconditional): Acknowledges decision range D-389..D-448 (inclusive; literal acknowledgment per D-415(c); sample; see decision-log.md for full range D-389..D-448 per D-441(c)+D-442(c) sample-vs-exhaustive citation policy). Per D-404 unconditional: index acknowledges D-448(a/b/c/d/e) by literal ID (no spec content change in this changelog entry). Refs: F-P68-CRIT-001, F-P68-HIGH-001/002/003/004, F-P68-MED-001/002/003, F-P68-LOW-001, PG-P68-001/002/003, D-448. [O-P68-001/002/003 trimmed per D-449(d)(i): observations are discovery-tier, not Refs-scope.] META-LEVEL-23 CANDIDATE CONFIRMED; L-EDP1-060 59th-layer 29th-consecutive multi-axis; rule-codification-without-self-application-in-codifying-burst-OWN-newly-created-meta-artifact ply (sample; see decision-log.md for full range D-389..D-448 per D-441(c)+D-442(c) sample-vs-exhaustive citation policy). ARCH-INDEX v1.91→v1.92."
  - date: 2026-05-13
    change: "v1.91 (2026-05-13; pass-67 fix burst per D-447 + D-404 unconditional): Acknowledges decision range D-389..D-447 (inclusive; literal acknowledgment per D-415(c); sample; see decision-log.md for full range D-389..D-447 per D-441(c)+D-442(c) sample-vs-exhaustive citation policy). Per D-404 unconditional: index acknowledges D-447(a/b/c/d/e) by literal ID (no spec content change in this changelog entry). Refs: F-P67-001/002/003/004/005/006/007/008, PG-P67-001/002, D-447. META-LEVEL-22 CANDIDATE CONFIRMED; L-EDP1-059 58th-layer 28th-consecutive multi-axis; own-downstream-citation-scope-extension-gap ply (sample; see decision-log.md for full range D-389..D-447 per D-441(c)+D-442(c) sample-vs-exhaustive citation policy). ARCH-INDEX v1.90→v1.91."
  - date: 2026-05-13
    change: "v1.90 (2026-05-13; pass-66 fix burst per D-446 + D-404 unconditional): Acknowledges decision range D-389..D-446 (inclusive; literal acknowledgment per D-415(c); sample; see decision-log.md for full range D-389..D-446 per D-441(c)+D-442(c) sample-vs-exhaustive citation policy). Per D-404 unconditional: index acknowledges D-446(a/b/c/d/e) by literal ID (no spec content change in this changelog entry). Refs: F-P66-001/002/003/004/006/007/008/009, PG-P66-001/002, D-446. META-LEVEL-21 CANDIDATE CONFIRMED; L-EDP1-058 57th-layer 27th-consecutive multi-axis; rule-codification-without-self-application-in-codifying-burst-OWN-burst-log ply (sample; see decision-log.md for full range D-389..D-446 per D-441(c)+D-442(c) sample-vs-exhaustive citation policy). [F-P66-006/008/009 added to Refs per D-447(a) self-application at pass-67 Commit A — prior entry omitted these; D-414(c) corrigendum.] ARCH-INDEX v1.89→v1.90."
  - date: 2026-05-13
    change: "v1.89 (2026-05-13; pass-65 fix burst per D-445 + D-404 unconditional): Acknowledges decision range D-389..D-445 (inclusive; literal acknowledgment per D-415(c); sample; see decision-log.md for full range D-389..D-445 per D-441(c)+D-442(c) sample-vs-exhaustive citation policy). Per D-404 unconditional: index acknowledges D-445(a/b/c/d/e) by literal ID (no spec content change in this changelog entry). Refs: F-P65-001/002/003/004/005/006/007/008, PG-P65-001, D-445. META-LEVEL-20 CANDIDATE CONFIRMED; L-EDP1-057 56th-layer 26th-consecutive multi-axis; rule-codification-applies-to-primary-but-not-downstream-citation ply (sample; see decision-log.md for full range D-389..D-445 per D-441(c)+D-442(c) sample-vs-exhaustive citation policy). ARCH-INDEX v1.88→v1.89."
  - date: 2026-05-12
    change: "v1.88 (2026-05-12; pass-64 fix burst per D-444 + D-404 unconditional): Acknowledges decision range D-389..D-444 (inclusive; literal acknowledgment per D-415(c); sample; see decision-log.md for full range D-389..D-444 per D-441(c)+D-442(c) sample-vs-exhaustive citation policy). Per D-404 unconditional: index acknowledges D-444(a/b/c/d/e) by literal ID (no spec content change in this changelog entry). Refs: F-P64-001/002/003/004/005, D-444. META-LEVEL-19 CANDIDATE CONFIRMED; L-EDP1-056 55th-layer 25th-consecutive multi-axis; rule-codification-without-automation gap ply (sample; see decision-log.md for full range D-389..D-444 per D-441(c)+D-442(c) sample-vs-exhaustive citation policy). ARCH-INDEX v1.87→v1.88."
  - date: 2026-05-12
    change: "v1.87 (2026-05-12; pass-63 fix burst per D-443 + D-404 unconditional): Acknowledges decision range D-389..D-443 (inclusive; literal acknowledgment per D-415(c); sample; see decision-log.md for full range D-389..D-443 per D-441(c)+D-442(c) sample-vs-exhaustive citation policy). Per D-404 unconditional: index acknowledges D-443(a/b/c/d/e) by literal ID (no spec content change in this changelog entry). Refs: F-P63-001/002/003/004/005/006/007/008/009, D-443. META-LEVEL-18 CANDIDATE CONFIRMED; L-EDP1-055 54th-layer 24th-consecutive multi-axis; rule-verification-grep co-evolution gap ply (sample; see decision-log.md for full range D-389..D-443 per D-441(c)+D-442(c) sample-vs-exhaustive citation policy). ARCH-INDEX v1.86→v1.87."
  - date: 2026-05-12
    change: "v1.86 (2026-05-12; pass-62 fix burst per D-442 + D-404 unconditional): Acknowledges decision range D-389..D-442 (inclusive; literal acknowledgment per D-415(c); sample; see decision-log.md for full range D-389..D-442 per D-441(c)+D-442(c) sample-vs-exhaustive citation policy). Per D-404 unconditional: index acknowledges D-442(a/b/c/d/e) by literal ID (no spec content change in this changelog entry). Refs: F-P62-001/002/003/004/005, D-442. META-LEVEL-17 CANDIDATE CONFIRMED; L-EDP1-054 53rd-layer 23rd-consecutive multi-axis; rule-application-cross-channel ply. ARCH-INDEX v1.85→v1.86."
  - date: 2026-05-12
    change: "v1.85 (2026-05-12; pass-61 fix burst per D-441 + D-404 unconditional): Acknowledges decision range D-389..D-441 (inclusive; literal acknowledgment per D-415(c)). Per D-404 unconditional: index acknowledges D-441(a/b/c/d/e) by literal ID (no spec content change in this changelog entry). Refs: F-P61-001/002/003/004/005/006/007, D-441. META-LEVEL-16 CONFIRMED; L-EDP1-053 52nd-layer 22nd-consecutive multi-axis; content-correct/form-divergent ply. ARCH-INDEX v1.84→v1.85."
  - date: 2026-05-12
    change: "v1.84 (2026-05-12; pass-60 fix burst per D-440 + D-404 unconditional): Acknowledges decision range D-389..D-440 (inclusive; literal acknowledgment per D-415(c)). Per D-404 unconditional: index acknowledges D-440(a/b/c/d/e) by literal ID (no spec content change in this changelog entry). Refs: ADV-EDP1-P60-HIGH-001/002/003/004/MED-001/002/003/LOW-001/002, D-440. META-LEVEL-15 CANDIDATE CONFIRMED; L-EDP1-052 51st-layer 21st-consecutive multi-axis. ARCH-INDEX v1.83→v1.84."
  - date: 2026-05-12
    change: "v1.83 (2026-05-12; pass-59 fix burst per D-439 + D-404 unconditional): Acknowledges decision range D-389..D-439 (inclusive; literal acknowledgment per D-415(c)). Per D-404 unconditional: index acknowledges D-439 by literal ID (no spec content change in this changelog entry). Refs: ADV-EDP1-P59-HIGH-001/002/003/004/MED-001/002/003/LOW-001/002, D-439. ARCH-INDEX v1.82→v1.83."
  - date: 2026-05-12
    change: "v1.82 (2026-05-12; pass-58 fix burst per D-438 + D-404 unconditional): Acknowledges decision range D-389..D-438 (inclusive; literal acknowledgment per D-415(c)). Per D-404 unconditional: index acknowledges D-438 by literal ID (no spec content change in this changelog entry). Refs: ADV-EDP1-P58-HIGH-001/002/003/004/MED-001/002/003/LOW-001, D-438. ARCH-INDEX v1.81→v1.82."
  - date: 2026-05-12
    change: "v1.81 (2026-05-12; pass-57 fix burst per D-437 + D-404 unconditional): Acknowledges decision range D-389..D-437 (inclusive; literal acknowledgment per D-415(c)). Per D-404 unconditional: index acknowledges D-437 by literal ID (no spec content change in this changelog entry). Refs: ADV-EDP1-P57-HIGH-001/002/003/MED-001/002/003/LOW-001/002, D-437. ARCH-INDEX v1.80→v1.81."
  - date: 2026-05-12
    change: "v1.80 (2026-05-12; pass-56 fix burst per D-436 + D-404 unconditional): Acknowledges decision range D-389..D-436 (inclusive; literal acknowledgment per D-415(c)). Per D-404 unconditional: index acknowledges D-436 by literal ID (no spec content change in this changelog entry). Refs: ADV-EDP1-P56-HIGH-001/002/003/004/005/MED-001/002/LOW-001/002, D-436. ARCH-INDEX v1.79→v1.80.
    change: v1.79 (2026-05-12; pass-55 fix burst per D-435 + D-404 unconditional): Acknowledges decision range D-389..D-435 (inclusive; literal acknowledgment per D-415(c)). Per D-404 unconditional: index acknowledges D-435 by literal ID (no spec content change in this changelog entry). Refs: ADV-EDP1-P55-HIGH-001/002/003/004/MED-001/002/LOW-001/002, D-435. ARCH-INDEX v1.78→v1.79."
  - date: 2026-05-12
    change: "v1.78 (2026-05-12; pass-54 fix burst per D-434 + D-404 unconditional): Acknowledges decision range D-389..D-434 (inclusive; literal acknowledgment per D-415(c)). Per D-404 unconditional: index acknowledges D-434 by literal ID (no spec content change in this changelog entry). Refs: ADV-EDP1-P54-HIGH-001/002/003/004/MED-001/002/003/LOW-001, D-434. ARCH-INDEX v1.77→v1.78."
  - date: 2026-05-12
    change: "v1.77 (2026-05-12; pass-53 fix burst per D-433 + D-404 unconditional): Acknowledges decision range D-389..D-433 (inclusive; literal acknowledgment per D-415(c)). Per D-404 unconditional: index acknowledges D-433 by literal ID (no spec content change in this changelog entry). Refs: ADV-EDP1-P53-CRIT-001/HIGH-001/002/003/004/MED-001/002/LOW-001, D-433. ARCH-INDEX v1.76→v1.77."
  - date: 2026-05-12
    change: "v1.76 (2026-05-12; pass-52 fix burst per D-432 + D-404 unconditional): Acknowledges decision range D-389..D-432 (inclusive; literal acknowledgment per D-415(c)). Per D-404 unconditional: index acknowledges D-432 by literal ID (no spec content change in this changelog entry). Refs: F-P52-001/002/003/004/005/006/007, D-432. ARCH-INDEX v1.75→v1.76."
  - date: 2026-05-12
    change: "v1.75 (2026-05-12; pass-51 fix burst per D-431 + D-404 unconditional): Acknowledges decision range D-389..D-431 (inclusive; literal acknowledgment per D-415(c)). Per D-404 unconditional: index acknowledges D-431 by literal ID (no spec content change in this changelog entry). Refs: F-P51-001/002/003/004/005/006/007, D-431. ARCH-INDEX v1.74→v1.75."
  - date: 2026-05-12
    change: "v1.74 (2026-05-12; pass-50 fix burst per D-430 + D-404 unconditional): Acknowledges decision range D-389..D-430 (inclusive; literal acknowledgment per D-415(c)). Per D-404 unconditional: index acknowledges D-430 by literal ID (no spec content change in this changelog entry). Refs: F-P50-001/002/003/004/005/006/007, D-430. ARCH-INDEX v1.73→v1.74."
  - date: 2026-05-12
    change: "v1.73 (2026-05-12; pass-49 fix burst per D-429 + D-404 unconditional): Acknowledges decision range D-389..D-429 (inclusive; literal acknowledgment per D-415(c)). Per D-404 unconditional: index acknowledges D-429 by literal ID (no spec content change in this changelog entry). Refs: F-P49-001/002/003/004/005/006/007/008, D-429. ARCH-INDEX v1.72→v1.73."
  - date: 2026-05-12
    change: "v1.72 (2026-05-12; pass-48 fix burst per D-428 + D-404 unconditional): Acknowledges decision range D-389..D-428 (inclusive; literal acknowledgment per D-415(c)). Per D-404 unconditional: index acknowledges D-428 by literal ID (no spec content change in this changelog entry). Refs: F-P48-001/002/003/004/005/006/007/008, D-428. ARCH-INDEX v1.71→v1.72."
  - date: 2026-05-12
    change: "v1.71 (2026-05-12; pass-47 fix burst per D-427 + D-404 unconditional): Acknowledges decision range D-389..D-427 (inclusive; literal acknowledgment per D-415(c)). Per D-404 unconditional: index acknowledges D-427 by literal ID (no spec content change in this changelog entry). Refs: F-P47-001/002/003/004/005/006/007, D-427. ARCH-INDEX v1.70→v1.71."
  - date: 2026-05-12
    change: "v1.70 (2026-05-12; pass-46 fix burst per D-426 + D-404 unconditional): Acknowledges decision range D-389..D-426 (inclusive; literal acknowledgment per D-415(c)). Per D-404 unconditional: index acknowledges D-426 by literal ID (no spec content change in this changelog entry). Refs: F-P46-001/002/003/004/005/006/007, D-426. ARCH-INDEX v1.69→v1.70."
  - date: 2026-05-12
    change: "v1.69 (2026-05-12; pass-45 fix burst per D-425 + D-404 unconditional): Acknowledges decision range D-389..D-425 (inclusive; literal acknowledgment per D-415(c)). Per D-404 unconditional: index acknowledges D-425 by literal ID (no spec content change in this changelog entry). Refs: F-P45-001/002/003/004/005/006/007/008, D-425. ARCH-INDEX v1.68→v1.69."
  - date: 2026-05-12
    change: "v1.68 (2026-05-12; pass-44 fix burst per D-424 + D-404 unconditional): Acknowledges decision range D-389..D-424 (inclusive; literal acknowledgment per D-415(c)). Per D-404 unconditional: index acknowledges D-424 by literal ID (no spec content change in this changelog entry). Refs: F-P44-001/002/003/004/005/006/007, D-424. ARCH-INDEX v1.67→v1.68."
  - date: 2026-05-12
    change: "v1.67 (2026-05-12; pass-43 fix burst per D-423 + D-404 unconditional): Acknowledges decision range D-389..D-423 (inclusive; literal acknowledgment per D-415(c)). Per D-404 unconditional: index acknowledges D-423 by literal ID (no spec content change in this changelog entry). Refs: F-P43-001/002/003/004/005/006/007/008, D-423."
  - date: 2026-05-12
    change: "v1.66 (2026-05-12; pass-42 fix burst per D-422 + D-404 unconditional): Acknowledges decision range D-389..D-422 (inclusive; literal acknowledgment per D-415(c)). Per D-404 unconditional: index acknowledges D-422 by literal ID (no spec content change in this changelog entry). Refs: F-P42-001/002/003/004/005/006/007, D-422."
  - date: 2026-05-12
    change: "v1.65 (2026-05-12; F-block-ai-attribution-message-file-arm F2+F3 close-out): SS-07 BC count 196→198 (+BC-7.03.094 PostToolUse retroactive HEAD verify, +BC-7.03.095 PreToolUse -F file-read arm). SS-07-hook-bash.md amended to v1.3; SS-04-plugin-ecosystem.md amended to v1.4. Total BCs 1,947→1,949. BC-INDEX cite refreshed v1.83→v1.84 (L-P20-002 cite-refresh discipline)."
  - date: 2026-05-12
    change: "v1.64 (2026-05-12; pass-41 fix burst per D-421 + D-404 unconditional): Acknowledges decision range D-389..D-421 (inclusive; literal acknowledgment per D-415(c)). Per D-404 unconditional: index acknowledges D-421 by literal ID (no spec content change in this changelog entry). Refs: F-P41-001/002/003/004/005/006/007/008, D-421."
  - date: 2026-05-12
    change: "v1.63 (2026-05-12; pass-40 fix burst per D-420 + D-404 unconditional): Acknowledges decision range D-389..D-420 (inclusive; literal acknowledgment per D-415(c)). Per D-404 unconditional: index acknowledges D-420 by literal ID (no spec content change in this changelog entry). Refs: F-P40-001/002/003/004/005/006/007, D-420."
  - date: 2026-05-12
    change: "v1.62 (2026-05-12; pass-39 fix burst per D-419 + D-404 unconditional): Acknowledges decision range D-389..D-419 (inclusive; literal acknowledgment per D-415(c)). Per D-404 unconditional: index acknowledges D-419 by literal ID (no spec content change in this changelog entry). Refs: F-P39-001/002/003/004/005/006/007/008, D-419. **Corrigendum (pass-40 fix burst — D-387 / F-P40-001):** F-P39-004 and F-P39-005 added to Refs per D-420(a) closure-set completeness."
  - date: 2026-05-12
    change: "v1.61 (2026-05-12; pass-38 fix burst per D-418 + D-404 unconditional): Acknowledges decision range D-389..D-418 (inclusive; literal acknowledgment per D-415(c)). Per D-404 unconditional: index acknowledges D-418 by literal ID (no spec content change in this changelog entry). Refs: F-P38-001/002/003/004/005, D-418."
  - date: 2026-05-11
    change: "v1.60 (2026-05-11; pass-37 fix burst per D-417 + D-404 unconditional): Acknowledges decision range D-389..D-417. Per D-404 unconditional: index acknowledges D-417 by literal ID (no spec content change in this changelog entry). Refs: F-P37-001/002/003/004/005, D-417."
  - date: 2026-05-11
    change: "v1.59 (2026-05-11; pass-36 fix burst per D-416 + D-404 unconditional): Acknowledges decision range D-389..D-416. Per D-404 unconditional: index acknowledges D-416 by literal ID (no spec content change in this changelog entry). Refs: F-P36-001/002/003/004/005, D-416."
  - date: 2026-05-11
    change: "v1.58 (2026-05-11; pass-35 fix burst per D-415 + D-404 unconditional): Acknowledges decision range D-389..D-415. Per D-404 unconditional: index acknowledges D-415 by literal ID (no spec content change in this changelog entry). Refs: F-P35-001/002/003/004/005, D-415."
  - date: 2026-05-11
    change: "v1.57 (2026-05-11; pass-34 fix burst per D-414 + D-404 unconditional): Acknowledges decision range D-389..D-414. Per D-404 unconditional: index acknowledges D-414 by literal ID (no spec content change in this changelog entry). Refs: F-P34-001/002, O-P34-001, D-414."
  - date: 2026-05-11
    change: "v1.56 (2026-05-11; pass-33 fix burst per D-413 + D-404 unconditional): Acknowledges decision range D-389..D-413. Per D-404 unconditional: index acknowledges D-413 by literal ID (no spec content change in this changelog entry). Refs: F-P33-001/002/003/004/005/006/PG1, D-413."
  - date: 2026-05-11
    change: "v1.55 (2026-05-11; pass-32 fix burst per D-412 + D-404 unconditional): Acknowledges decision range D-389..D-412. Per D-404 unconditional: index acknowledges D-412 by literal ID (no spec content change in this changelog entry). Refs: F-P32-001/002/003, D-412."
  - date: 2026-05-11
    change: "v1.54 (2026-05-11; pass-31 fix burst per D-411 + D-404 unconditional): Acknowledges decision range D-389..D-411. Closes F-P31-001/002/PG1 ARCH-INDEX instance. Refs: F-P31-001, F-P31-002, F-P31-PG1, D-411."
  - date: 2026-05-11
    change: "v1.53 (2026-05-11; pass-30 fix burst per D-410 + D-404 unconditional): Acknowledges decision range D-389..D-410. Closes F-P30-001/PG1 ARCH-INDEX instance. Refs: F-P30-001, F-P30-PG1, D-410."
  - date: 2026-05-11
    change: "v1.52 (2026-05-11; pass-29 fix burst per D-409 + D-404 unconditional): Acknowledges decision range D-389..D-409. Closes F-P29-001/002/005/006/007/PG1 ARCH-INDEX instance. Refs: F-P29-001, F-P29-002, F-P29-006, D-409."
  - date: 2026-05-11
    change: "v1.51 (2026-05-11; pass-28 fix burst per D-408): Acknowledges decision range D-389..D-408. Closes F-P28-001/002/003 ARCH-INDEX instance + F-P28-PG1. Refs: F-P28-001, F-P28-002, F-P28-003, D-408."
  - date: 2026-05-11
    change: "v1.50 (2026-05-11; pass-27 fix burst per D-407(a) + D-404 unconditional clarification): Acknowledges decision range D-389..D-407. Closes F-P27-001 ARCH-INDEX instance. Refs: F-P27-001, D-404, D-406, D-407."
  - date: 2026-05-11
    change: "v1.49 (2026-05-11; pass-25 fix burst per D-405 + D-404 self-correction): Acknowledges decision range D-389..D-405. Closes F-P25-001 ARCH-INDEX instance. Refs: F-P25-001, D-404, D-405."
  - date: 2026-05-11
    change: "v1.48 (2026-05-11; pass-24 fix burst per D-404): Acknowledges decision range D-389..D-403 (extends v1.47 range from D-389..D-402; closes D-403 gap surfaced by F-P24-001 + F-P24-004). Literal acknowledgment per D-404. Closes F-P24-001 + F-P24-004. Refs: F-P24-001, F-P24-004, D-404."
  - date: 2026-05-11
    change: "v1.47 (2026-05-11; pass-23 fix burst per D-403(a)): Acknowledges cycle-governance decision range D-389..D-402 codified in cycle v1.0-feature-engine-discipline-pass-1 fix bursts pass-19 through pass-22 (closing partial-coverage gap surfaced by F-P23-001, F-P23-008). Refs: F-P23-001, F-P23-008, D-403(a). ARCH-INDEX v1.46→v1.47."
  - date: 2026-05-11
    change: "v1.46 (2026-05-11; pass-22 fix burst cite-refresh): BC-INDEX body cite refreshed v1.64→v1.65 per F-P21-005 cycle-decision sync (BC-INDEX v1.65 bumped in pass-21 fix burst to acknowledge governance decisions D-389..D-400; ARCH-INDEX v1.46 cite-refresh was missed in the pass-21 burst — closed retroactively by F-P22-001). Refs: F-P21-005, L-P20-002, F-P22-001. ARCH-INDEX v1.45→v1.46."
  - date: 2026-05-11
    change: "v1.45 (2026-05-11; F-P8-001 — L-P20-002 cite-refresh discipline): BC-INDEX body cite refreshed v1.63→v1.64 corresponding to BC-INDEX bumped v1.63→v1.64 in F-P7-001 fix burst (pass-7) for BC-7.03.091/092 Capability column TBD→CAP-008. ARCH-INDEX changelog entry was missed in the pass-7 fix burst (first L-P20-002 violation in 16 consecutive clean cite-refresh cycles: v1.48→v1.64 all correctly paired; v1.64 broke the streak). F-P8-001 closes this propagation gap retroactively. ARCH-INDEX v1.44→v1.45."
  - date: 2026-05-09
    change: "v1.44 (2026-05-09; fix-burst-49): BC-INDEX body cite refreshed v1.62→v1.63 (BC-INDEX bumped v1.62→v1.63 in same burst for F-P54-001 Title-cell corpus sweep — 6 drifts patched across BC-1.05.010/BC-2.02.011/BC-2.02.012/BC-4.05.002/BC-4.05.003/BC-5.30.001; 19th L-P28-001 META; L-P20-002 cite-refresh discipline). ARCH-INDEX v1.43→v1.44."
  - date: 2026-05-09
    change: "v1.43 (2026-05-09; fix-burst-48): F-P52-001 sibling-artifact propagation — v1.41 changelog narrative corrected: '48 BCs verified clean' → '53 BCs verified clean' (fix-burst-47 inline-corrected BC-INDEX/lessons.md/STATE.md but not ARCH-INDEX; sibling-not-updated gap per Interpretation B adjudication). ARCH-INDEX v1.42→v1.43."
  - date: 2026-05-09
    change: "v1.42 (2026-05-09; fix-burst-47): BC-INDEX body cite refreshed v1.61→v1.62 (BC-INDEX bumped v1.61→v1.62 in same burst for F-P50-001 count-narrative correction E-7=23→28, Total=48→53; L-P20-002 cite-refresh discipline). ARCH-INDEX v1.41→v1.42."
  - date: 2026-05-09
    change: "v1.41 (2026-05-09; fix-burst-46): BC-INDEX body cite refreshed v1.60→v1.61 (BC-INDEX bumped v1.60→v1.61 in same burst for corpus-wide retroactive sweep E-6/7/9/10/11 — 53 BCs verified clean; L-P20-002 cite-refresh discipline). ARCH-INDEX v1.40→v1.41. [Count corrected 48→53 in fix-burst-48 per F-P52-001 sibling-propagation.]"
  - date: 2026-05-09
    change: "v1.40 (2026-05-09; fix-burst-45): BC-INDEX body cite refreshed v1.59→v1.60 (BC-INDEX bumped v1.59→v1.60 in same burst for F-P49-001 E-3+E-4 retroactive Stories propagation 20 BCs; L-P20-002 cite-refresh discipline). ARCH-INDEX v1.39→v1.40."
  - date: 2026-05-09
    change: "v1.39 (2026-05-09; fix-burst-44): BC-INDEX body cite refreshed v1.58→v1.59 (BC-INDEX bumped v1.58→v1.59 in same burst for F-P48-001 count-narrative correction 25→30 BCs; L-P20-002 cite-refresh discipline). ARCH-INDEX v1.38→v1.39."
  - date: 2026-05-09
    change: "v1.38 (2026-05-09; fix-burst-43): BC-INDEX body cite refreshed v1.57→v1.58 (BC-INDEX bumped v1.57→v1.58 in same burst for F-P47-001 30 BC-INDEX Stories cells E-8 native-port propagation; L-P20-002 cite-refresh discipline). ARCH-INDEX v1.37→v1.38."
  - date: 2026-05-09
    change: "v1.37 (2026-05-09; fix-burst-42): BC-INDEX body cite refreshed v1.56→v1.57 (BC-INDEX bumped v1.56→v1.57 in same burst for F-P45-001 12 BC body Stories rows + 2 BC-INDEX TBD fixes; L-P20-002 cite-refresh discipline). ARCH-INDEX v1.36→v1.37."
  - date: 2026-05-09
    change: "v1.36 (2026-05-09; fix-burst-40): BC-INDEX body cite refreshed v1.55→v1.56 (BC-INDEX bumped v1.55→v1.56 in same burst for F-P42-003 BC-5.39.001 S-14.01 addition; L-P20-002 cite-refresh discipline). ARCH-INDEX v1.35→v1.36."
  - date: 2026-05-09
    change: "v1.35 (2026-05-09; fix-burst-39): BC-INDEX body cite refreshed v1.54→v1.55 (BC-INDEX bumped v1.54→v1.55 in same burst for F-P41-002 Stories cells corpus reconciliation; L-P20-002 cite-refresh discipline). ARCH-INDEX v1.34→v1.35."
  - date: 2026-05-09
    change: "v1.34 (2026-05-09; fix-burst-36): BC-INDEX body cite refreshed v1.53→v1.54 (BC-INDEX bumped v1.53→v1.54 in same burst for F-P37-001 3 BC rows S-10.04 added; L-P20-002 cite-refresh discipline). ARCH-INDEX v1.33→v1.34."
  - date: 2026-05-09
    change: "v1.33 (2026-05-09; fix-burst-35): BC-INDEX body cite refreshed v1.52→v1.53 (BC-INDEX bumped v1.52→v1.53 in same burst for F-P36-001 12 BC body Stories rows S-15.01 propagation; L-P20-002 cite-refresh discipline). ARCH-INDEX v1.32→v1.33."
  - date: 2026-05-09
    change: "v1.32 (2026-05-09; fix-burst-34): BC-INDEX body cite refreshed v1.51→v1.52 (BC-INDEX bumped v1.51→v1.52 in same burst for F-P35-001 resolver-platform BC rows reconciliation; L-P20-002 cite-refresh discipline). ARCH-INDEX v1.31→v1.32."
  - date: 2026-05-09
    change: "v1.31 (2026-05-09; fix-burst-26 sub-burst 2): BC-INDEX body cite refreshed v1.50→v1.51 (BC-INDEX bumped v1.50→v1.51 in same burst for F-P27-007 last_amended backfill + v1.51 changelog; L-P20-002 cite-refresh discipline). VP-INDEX bumped v1.34→v1.35 (F-P27-005 kani-proof harmonization). STORY-INDEX v2.55→v2.56 (56 stories POST-MERGE-STATE annotated). ARCH-INDEX v1.30→v1.31."
  - date: 2026-05-09
    change: "v1.30 (2026-05-09; fix-burst-24 sub-burst 2 state-manager close): BC-INDEX body cite refreshed v1.49→v1.50 (ADR-019 v1.11, BC-3.08.001 v1.13, BC-INDEX bumped v1.49→v1.50 per sub-burst 1 of fix-burst-24; L-P20-002 cite-refresh discipline). ARCH-INDEX v1.29→v1.30."
  - date: 2026-05-09
    change: "v1.29 (2026-05-09; fix-burst-23 sub-burst 3): BC-INDEX body cite refreshed v1.48→v1.49 (BC-7.06.001 v1.11 sibling fix per L-P23-001 + L-P24-002; BC-INDEX bumped v1.48→v1.49 in same burst per L-P20-002 cite-refresh discipline). ARCH-INDEX v1.28→v1.29."
  - date: 2026-05-09
    change: "v1.28 (2026-05-09; fix-burst-22 sub-burst 3): BC-INDEX body cite refreshed v1.47→v1.48 (F-P23-001/002 sub-burst 1 lobster-line-cite broadest sweep; BC-INDEX bumped v1.47→v1.48 in same burst per L-P20-002 cite-refresh discipline). ARCH-INDEX v1.27→v1.28."
  - date: 2026-05-08
    change: "v1.27 (2026-05-08; F-P23-003 fix): BC-INDEX body cite refreshed v1.46→v1.47 (F-P23-003 H1 rebrand for BC-1.07.005/BC-1.07.006; BC-INDEX bumped v1.46→v1.47 in same burst per L-P20-002 cite-refresh discipline). ARCH-INDEX v1.26→v1.27."
  - date: 2026-05-08
    change: "v1.26 (2026-05-08; fix-burst-21 sub-burst 4): BC-INDEX body cite refreshed v1.45→v1.46 (state-manager bumped BC-INDEX from v1.45→v1.46 in same burst per L-P20-002 cite-refresh discipline). ARCH-INDEX v1.25→v1.26."
  - date: 2026-05-08
    change: "v1.25 (2026-05-08; fix-burst-20 sub-burst 2): BC-INDEX body cite refreshed v1.44→v1.45 (state-manager bumped BC-INDEX from v1.44→v1.45 in same burst per L-P20-002 cite-refresh discipline). ARCH-INDEX v1.24→v1.25."
  - date: 2026-05-08
    change: "v1.24 (2026-05-08; fix-burst-19 sub-burst 2): BC-INDEX body cite refreshed v1.43→v1.44 (state-manager bumped BC-INDEX from v1.43→v1.44 in same burst per L-P20-002 cite-refresh discipline). ARCH-INDEX v1.23→v1.24."
  - date: 2026-05-08
    change: "v1.23 (2026-05-08; F-P20-002 cite-refresh): BC-INDEX body cite refreshed v1.33→v1.43 (closes 10-version drift accumulated over 15 fix-bursts). VP-INDEX and STORY-INDEX have no versioned body cites (confirmed: no v-number body references outside changelog). ARCH-INDEX v1.22→v1.23."
  - date: 2026-05-08
    change: "v1.22 (2026-05-08; F5 fix-burst-3 Stage 1): ADR-020 rationale clarification (F-P3-006; no version bump). BC-INDEX cite refreshed v1.33→v1.34. VP-INDEX cite refreshed v1.18→v1.19. STORY-INDEX cite refreshed v2.39→v2.40. ARCH-INDEX v1.21→v1.22."
  - date: 2026-05-08
    change: "v1.21 (2026-05-08; F5 fix-burst-2 Stage 1): ADR-020 line 261 typo fix + last_amended bumped. BC-INDEX cite refreshed v1.32→v1.33. VP-INDEX cite refreshed v1.17→v1.18. STORY-INDEX cite refreshed v2.38→v2.39. ARCH-INDEX v1.20→v1.21."
  - date: 2026-05-08
    change: "v1.20 (2026-05-08; F5 pass-1 path-A close): ADR-020 row added (Dispatcher Latency Budget Classes; SS-01; v1.0; ACCEPTED 2026-05-08). BC-INDEX cite refreshed v1.31→v1.32 (BC-1.14.001 v1.7→v1.8 DI-017 traceability). ARCH-INDEX v1.19→v1.20."
  - date: 2026-05-07
    change: "v1.19 (2026-05-07; F2 CONVERGENCE close): BC-INDEX cite refreshed v1.26→v1.27 (NIT-P10-001 fix; BC-3.08.001 v1.4). F2 CONVERGED for v1.0-feature-plugin-async-semantics-pass-1 (ADR-013 clock 3_of_3; trajectory 19→19→7→6→3→5→4→1→2→1; 10 passes). Per burst-close protocol: any BC-INDEX version bump triggers ARCH-INDEX cite refresh in same burst."
  - date: 2026-05-07
    change: "v1.18 (2026-05-07; NIT-P8-001 fix): BC-INDEX cite refreshed v1.25→v1.26 per pass-7 burst. Recurrence note: this is the 3rd refresh of this cite (pass-5 v1.22→v1.24; pass-6 v1.24→v1.25; pass-8 v1.25→v1.26). Future burst-close protocol enhancement: any BC-INDEX version bump must trigger ARCH-INDEX cite refresh in same burst. ARCH-INDEX v1.17→v1.18."
  - date: 2026-05-07
    change: "F2 pass-6 fix burst close (2026-05-07): ADR-019 row updated to v1.8 (F-P6-005: §Consequences inline 100ms parenthetical removed; cites DI-019 by reference). BC-INDEX version cite updated v1.24→v1.25. ARCH-INDEX v1.16→v1.17. last_amended: 2026-05-07 (async-semantics F2 pass-6 fix: ADR-019 v1.8; F-P6-005 inline literal removed)."
  - date: 2026-05-07
    change: "F2 pass-5 fix burst close (2026-05-07): BC-INDEX version cite on line 116 updated v1.22→v1.24 (F-P5-002). ADR-019 row updated to v1.7 (VP-079 §References parity). ARCH-INDEX v1.15→v1.16. last_amended: 2026-05-07 (async-semantics F2 pass-5 fix: BC-INDEX cite refresh; ADR-019 v1.7)."
  - date: 2026-05-07
    change: "F2 pass-4 fix burst close (2026-05-07): ADR-019 v1.5→v1.6 (§Consequences drain-window line 215 symbolic ASYNC_DRAIN_WINDOW_MS; F-P4-004). ARCH-INDEX v1.14→v1.15. last_amended: 2026-05-07 (async-semantics F2 pass-4 fix burst: ADR-019 v1.6; F-P4-001 BC-INDEX re-tally confirmed in ARCH-INDEX)."
  - date: 2026-05-07
    change: "F2 pass-3 fix burst close + user-correction (2026-05-07): DI-NN placeholder resolved to DI-019 in ADR-019 §Consequences and VP-079 (all references). ADR-019 v1.5 confirmed at close. SS-09-config-activation.md v1.2 (stale text replaced in-place). SS-07-hook-bash.md v1.2 (schema_version 1→2 in-place). last_amended: 2026-05-07 (async-semantics F2 pass-3 fix burst close: DI-019 placeholder resolved; SS-09 v1.2; SS-07 v1.2; ADR-019 v1.5)."
  - date: 2026-05-07
    change: "F2 pass-3 fix burst revision (2026-05-07): Subsystem Registry BC counts re-tallied to authoritative frontmatter subsystem (not directory). BC-7.06.001 (ss-07/ dir, frontmatter SS-01): SS-07 197→196, SS-01 116→117. BC-8.29.001/002/003 + BC-8.30.002 (ss-08/ dir, frontmatter SS-05): SS-08 218→214, SS-05 648→652. Total BCs unchanged at 1,947. Obsolete directory-based footnote removed. ADR-019 v1.4→v1.5 (drain window cites DI-019 as canonical constant source). VP-079 v1.2→v1.3 (Property 5 and drain-window references cite DI-019 as canonical source; DI-019 added to Traceability)."
  - date: 2026-05-07
    change: "F2 pass-2 fix burst close (2026-05-07): ADR-019 v1.2→v1.3 (sync/async classification rationale added to §Consequences for warn-pending-wave-gate/regression-gate SYNC deliberate; track-agent-start/stop/session-learning added to async list; BC-7.06.001 Invariant 6 cross-reference added). last_amended: 2026-05-07 (async-semantics F2 pass-2 fix burst: ADR-019 v1.3 §Consequences sync)."
  - date: 2026-05-07
    change: "F2 pass-1 fix burst (2026-05-07): ADR-019 v1.0→v1.1 (placeholder BC IDs resolved; forward ref BC-9.01.006 added to SS-09 subsystem assignments). SS-09-config-activation.md v1.0→v1.1 (async:true refs removed; schema_version 1→2). SS-07-hook-bash.md v1.0→v1.1 (schema_version 1→2). last_amended: 2026-05-07 (F2 pass-1 fix burst: 3 arch doc amendments, BC-9.01.006 forward ref resolved in ADR-019)."
  - date: 2026-05-07
    change: "F2 async-semantics (2026-05-07): +1 ADR (ADR-019 Plugin Async Semantics at Registry Layer — SS-01, SS-07, SS-09 affected). Supersedes: null. ADR-019 is next-available after ADR-018. SS-01 BC count 115→116 (+BC-1.14.001 dispatcher partition contract). SS-07 BC count 196→197 (+BC-7.06.001 registry schema v2 + CI lint). Total BCs 1943→1945."
  - date: 2026-05-07
    change: "D-362 F2-amendment WASM-plugin Context Resolver platform: +1 ADR (ADR-018 WASM-plugin Context Resolvers — Design and Layering — SS-01, SS-04). SS-01 BC count 114→115 (+BC-1.13.001 resolver-registry loading + pre-dispatch context injection); SS-04 BC count 34→39 (+BC-4.12.001-005 resolver lifecycle/ABI/capabilities/error-isolation/merging). Total BCs 1937→1943. ADR-018 is next-available after ADR-017."
  - date: 2026-05-07
    change: "D-340 F2 spec evolution for engine discipline pass-1: +2 ADRs (ADR-016 artifact path registry SoT — SS-04/SS-06; ADR-017 per-story adversary three-perimeter model — SS-04/SS-05). SS-04 BC count 31→34 (+BC-4.10.001/002 + BC-4.11.001); SS-05 BC count 646→648 (+BC-5.39.001/002); SS-06 BC count 585→586 (+BC-6.22.001). Total BCs 1931→1937. ADR-016 and ADR-017 are next-available after ADR-015."
  - date: 2026-05-06
    change: "D-336 E-10 pass-8 fix-cycle (architect): version bump v1.6 → v1.7. F-2 fix: Cross-Cutting Concerns line 152 — `dispatcher_trace_id` → `trace_id` per DI-017 v1.1 / ADR-015 v1.7 canonicalization. F-3 fix: Schema versioning row — differentiated per-config (hooks-registry=1, observability-config=2 per ADR-015 D-15.1); added SS-03 to subsystem list; cited BC-3.05.004 PC4 migration hint and DI-014. last_updated: 2026-05-06."
  - date: 2026-05-06
    change: "D-333 E-10 pass-6 fix-cycle seal: version bump v1.5 → v1.6. F-1 fix: renumbering-history paragraph (line 96) D-15.4 → D-15.1 — same-document sibling-paragraph drift from D-331's primary fix at SS-03 row (line 85) now fully closed. No BC subsystem count changes."
  - date: 2026-05-06
    change: "D-331 E-10 pass-5 fix-cycle seal: version bump v1.4 → v1.5. F-2 propagation: BC-3.05.004 narrative in Subsystem Registry SS-03 row updated D-15.4 → D-15.1 (OQ-1 resolved in SS-03-event-emission.md). No BC subsystem count changes."
  - date: 2026-05-06
    change: "D-327 E-10 ↔ rc.12 format-alignment seal: version bump v1.3 → v1.4. No BC subsystem count changes (4 BCs amended, not added). Courtesy bump to record rc.12 alignment cycle sealed (audit 119e70e → D-326 7afc64d → D-327). Engine baseline: v1.0.0-rc.12 (4cf59bc on develop)."
  - date: 2026-05-06
    change: "D-324 E-10 pass-3 fix burst seal: version bump v1.2 → v1.3. No substantive ARCH-INDEX changes this burst; BC subsystem counts unchanged (no new BCs in D-322). Courtesy bump to record D-324 seal."
  - date: 2026-05-06
    change: "D-321 E-10 pass-2 fix burst seal: version bump v1.1 → v1.2. SS-02 BC count 25→26 (+BC-2.06.001); SS-04 BC count 30→31 (+BC-4.09.001). Total BCs 1929→1931. See cycles/v1.0-brownfield-backfill/E-10-pass-2.md."
---

# Architecture Index: vsdd-factory

> **D-443(b)(i) + D-444(e)(ii) documentary-historical exemption (per D-414(c)):** Pre-v1.87-cohort changelog entries (ARCH v1.60..v1.86) are documentary-historical-exempt per D-414(c); the sample-vs-exhaustive flag introduced at pass-62 fix burst (D-442(c)) applies forward-only from ARCH v1.87+. Do NOT retroactively annotate or rewrite pre-v1.87 changelog entries.

> **Context Engineering:** This is a lightweight index (~350 tokens). Agents load
> ONLY the section files they need. See the Document Map for per-section consumer
> guidance.

## Document Map

| Section | File | Primary Consumer | Purpose |
|---------|------|-----------------|---------|
| SS-01 Hook Dispatcher Core | SS-01-hook-dispatcher.md | implementer, story-writer | Module catalog, host fns, sandbox model, Rust crates |
| SS-02 Hook SDK and Plugin ABI | SS-02-hook-sdk.md | implementer, plugin authors | SDK API, manifest schema, capability declarations |
| SS-03 Event Emission (OTel-Aligned) | SS-03-event-emission.md | implementer, story-writer | Single-stream FileSink, OTel schema, host enrichment, write-failure cascade. Old file SS-03-observability-sinks.md is SUPERSEDED (see ADR-015). |
| SS-04 Plugin Ecosystem | SS-04-plugin-ecosystem.md | implementer, story-writer | legacy-bash-adapter, capture-commit-activity, Tier E/F lifecycle plugin crates |
| SS-05 Pipeline Orchestration | SS-05-orchestration.md | orchestrator, story-writer | Agents, Lobster workflows, pipeline phase structure |
| SS-06 Skill Catalog | SS-06-skill-catalog.md | story-writer, skill authors | 119 skills, SKILL.md contract, output routing |
| SS-07 Hook Bash Layer | SS-07-hook-bash.md | implementer, formal-verifier | 44 bash hooks, registry bindings, exit code semantics |
| SS-08 Templates and Rules | SS-08-templates-rules.md | story-writer, implementer | 108 templates, 9 rules, template compliance contracts |
| SS-09 Configuration and Activation | SS-09-config-activation.md | implementer, story-writer | hooks.json variants, activation skill, CI platform config |
| SS-10 CLI Tools and Bin | SS-10-cli-tools.md | implementer, story-writer | 12 bin tools, 110 slash-command bindings |

### Future Sections (Deferred)

The following cross-cutting documents were planned but are currently deferred.
The per-subsystem SS-NN files above collectively cover their content:

| Deferred File | Covered By |
|---------------|-----------|
| system-overview.md | ARCH-INDEX §Subsystem Registry + §Component Dependency Map |
| module-decomposition.md | SS-01..SS-10 section files + §Subsystem Registry |
| dependency-graph.md | §Component Dependency Map (Mermaid) in this file |
| api-surface.md | SS-01-hook-dispatcher.md, SS-02-hook-sdk.md |
| verification-architecture.md | VP-INDEX.md + SS-07-hook-bash.md |
| purity-boundary-map.md | SS-01..SS-04 section files (purity notes per module) |
| tooling-selection.md | VP-INDEX §Kani Upgrade Candidates + §Property-Test Upgrade Candidates |
| verification-coverage-matrix.md | VP-INDEX §Full Index (scope column = module mapping) |

## Cross-References

| If you need... | Read these together |
|----------------|-------------------|
| BC renumbering mapping | Subsystem Registry below (SS-NN → BC-S range) |
| Implementation plan for a module | SS-NN section file for that subsystem + §Subsystem Registry |
| Verification plan for a module | VP-INDEX.md + SS-07-hook-bash.md (for hook VPs) |
| Story decomposition input | SS-NN section files for relevant subsystems + §Component Dependency Map |

## Subsystem Registry

> **Source of truth** for subsystem names and IDs. BC frontmatter `subsystem:`,
> BC-INDEX subsystem column, story `subsystems:` fields, and PRD subsystem
> references MUST all use the exact Name from this table.

BC counts are shown by **authoritative subsystem** (BC frontmatter `subsystem:` field), not by directory location.

| SS-ID | Name | Section File | Implementing Modules / Folders | BC-S Prefix | BCs | Phase |
|-------|------|--------------|-------------------------------|-------------|-----|-------|
| SS-01 | Hook Dispatcher Core | SS-01-hook-dispatcher.md | `crates/factory-dispatcher/src/{main,registry,routing,executor,invoke,engine,plugin_loader,payload}.rs` | BC-1 | 117 (116 by directory + 1 reanchored: BC-7.06.001 frontmatter subsystem=SS-01 per F-P1-006) | Phase 1 |
| SS-02 | Hook SDK and Plugin ABI | SS-02-hook-sdk.md | `crates/hook-sdk/`, `crates/hook-sdk-macros/` | BC-2 | 26 (includes 2 D-183 BCs: BC-2.02.011 host::write_file, BC-2.02.012 HookPayload SubagentStop fields; +1 D-219 BC-2.02.013 host::run_subprocess — WITHDRAWN D-224; +1 D-321 BC-2.06.001 SDK semver bump) | Phase 1 |
| SS-03 | Event Emission (OTel-Aligned) | SS-03-event-emission.md | `crates/sink-core/` (FileSink; Router/SinkRegistry deprecated), `crates/sink-file/`, `crates/factory-dispatcher/src/{host/emit_event,internal_log,sinks}.rs` (`sink-otel-grpc/` deprecated Wave 1) | BC-3 | 53 (51 prior + 1 Phase 1b BC-3.05.004 v2 schema validation per ADR-015 D-15.1 (OQ-1 resolved in SS-03-event-emission.md) + 1 F2 pass-1 fix burst BC-3.08.001 async-semantics event catalog) | Phase 1 |
| SS-04 | Plugin Ecosystem | SS-04-plugin-ecosystem.md | `crates/hook-plugins/legacy-bash-adapter/`, `crates/hook-plugins/capture-commit-activity/`, `crates/hook-plugins/capture-pr-activity/` [PLANNED S-3.02], `crates/hook-plugins/block-ai-attribution/` [PLANNED S-3.03], `crates/hook-plugins/session-start-telemetry/` [PLANNED S-5.01], `crates/hook-plugins/session-end-telemetry/` [PLANNED S-5.02], `crates/hook-plugins/worktree-hooks/` [PLANNED S-5.03], `crates/hook-plugins/tool-failure-hooks/` [PLANNED S-5.04], `crates/hook-plugins/validate-per-story-adversary-convergence/` [PLANNED], `crates/hook-plugins/validate-artifact-path/` [PLANNED] | BC-4 | 39 (+1 D-321 BC-4.09.001; +3 D-340 BC-4.10.001/002 + BC-4.11.001; +5 D-362 BC-4.12.001-005 resolver platform) | Phase 1 |
| SS-05 | Pipeline Orchestration | SS-05-orchestration.md | `plugins/vsdd-factory/agents/`, `plugins/vsdd-factory/workflows/*.lobster`, `plugins/vsdd-factory/workflows/phases/` | BC-5 | 652 (648 by directory + 4 reanchored: BC-8.29.001/002/003 + BC-8.30.002 frontmatter subsystem=SS-05; files remain in ss-08/ per POLICY 1) | Phase 1 |
| SS-06 | Skill Catalog | SS-06-skill-catalog.md | `plugins/vsdd-factory/skills/` (119 skills, 581 markdown files) | BC-6 | 586 (+1 D-340 BC-6.22.001 relocate-artifact skill) | Phase 1 |
| SS-07 | Hook Bash Layer | SS-07-hook-bash.md | `plugins/vsdd-factory/hooks/*.sh` (44 scripts), `plugins/vsdd-factory/hooks-registry.toml`, `crates/hook-plugins/block-ai-attribution/` (PreToolUse + PostToolUse arms) | BC-7 | 198 (199 by directory − 1 reanchored: BC-7.06.001 is in ss-07/ but frontmatter subsystem=SS-01 per F-P1-006; +2 F-block-ai-attribution-message-file-arm: BC-7.03.094/095) | Phase 1 |
| SS-08 | Templates and Rules | SS-08-templates-rules.md | `plugins/vsdd-factory/templates/` (108 files), `plugins/vsdd-factory/rules/` (9 files) | BC-8 | 214 (218 by directory − 4 reanchored: BC-8.29.001/002/003 + BC-8.30.002 frontmatter subsystem=SS-05; files remain in ss-08/ per POLICY 1) | Phase 1 |
| SS-09 | Configuration and Activation | SS-09-config-activation.md | `plugins/vsdd-factory/hooks/hooks.json*`, `plugins/vsdd-factory/.claude-plugin/plugin.json`, `ci/platforms.yaml`, `scripts/generate-registry-from-hooks-json.sh` | BC-9 | 6 (+1 F2 pass-1 fix burst BC-9.01.006 envelope-sync invariant) | Phase 1 |
| SS-10 | CLI Tools and Bin | SS-10-cli-tools.md | `plugins/vsdd-factory/bin/` (12 tools), `plugins/vsdd-factory/commands/` (110 files), `scripts/` | BC-10 | 58 | Phase 1 |

**Total BCs: 1,949 (per BC-INDEX v1.84; counts above are by authoritative frontmatter subsystem).** Cross-subsystem file placements (POLICY 1 append-only): BC-7.06.001 in ss-07/ → SS-01 (F-P1-006 reanchor); BC-8.29.001/002/003 + BC-8.30.002 in ss-08/ → SS-05 (historical allocation). The total is invariant under both directory-based and frontmatter-based tallying.

**Renumbering history — BC-1.12.008 → BC-3.05.004 (D-311/D-312):** BC-1.12.008 was originally proposed as an SS-01 routing target in D-311; renumbered to BC-3.05.004 (SS-03) in D-312 corrigendum per POLICY 1 ID-collision rule (BC-3.05.001/002/003 already existed as brownfield BCs authored by codebase-analyzer on 2026-04-25; BC-3.05.004 was the next free slot). Consequence: SS-01 has +4 Phase 1a additions (BC-1.12.001–BC-1.12.004) and +4 Phase 1b additions (BC-1.12.005/006/007/009; no BC-1.12.008 ID exists). SS-03 has +1 Phase 1b addition (BC-3.05.004 v2 schema validation per ADR-015 D-15.1). OQ-W16-012 filed-and-resolved in D-312.

### Subsystem Registry Design Notes

The 10-subsystem layout reflects two first-class groups:

- **SS-01 through SS-04** are **Subsystem A** (Rust compiled artifacts). They share one Cargo workspace, one release binary per platform, and the WASM plugin ABI contract.
- **SS-05 through SS-10** are **Subsystem B** (VSDD orchestration framework). They share one plugin manifest, one marketplace entry, and the Lobster workflow format.

**Deviations from the suggested 10-subsystem template:**

- SS-05 was renamed from "Pipeline Orchestration" to "Pipeline Orchestration" — kept, but agents were moved in (34 agents belong here alongside workflows because agents are the steps' executors; splitting them would orphan the orchestrator).
- SS-06 "Skill Catalog" is the largest single BC surface (585 BCs as of v1.0.0-beta.6; see Subsystem Registry table for current count). It is intentionally a standalone subsystem because skills have independent behavioral contracts per skill (each SKILL.md is a discrete unit of behavior).
- SS-09 "Configuration and Activation" is narrower than the suggested name — it covers only the activation plumbing and CI variant generation, not general config. `hooks-registry.toml` routing lives in SS-07 (Hook Bash Layer) because it is the routing table for that layer.
- SS-10 merges "CLI Tools" and "Bin" because all 12 bin tools are CLI-invocable and the commands/ slash-command bindings are just thin wrappers around skills.

## Component Dependency Map

```mermaid
graph TD
    CC["Claude Code (harness)"]
    SS09["SS-09: Configuration and Activation"]
    SS01["SS-01: Hook Dispatcher Core"]
    SS02["SS-02: Hook SDK and Plugin ABI"]
    SS03["SS-03: Event Emission (OTel-Aligned)"]
    SS04["SS-04: Plugin Ecosystem"]
    SS05["SS-05: Pipeline Orchestration"]
    SS06["SS-06: Skill Catalog"]
    SS07["SS-07: Hook Bash Layer"]
    SS08["SS-08: Templates and Rules"]
    SS10["SS-10: CLI Tools and Bin"]

    CC -->|"reads hooks.json"| SS09
    SS09 -->|"generated variants wire to"| SS01
    SS01 -->|"loads + executes WASM via"| SS02
    SS01 -->|"writes all events to (single-stream)"| SS03
    SS01 -->|"invokes plugins from"| SS04
    SS04 -->|"shells out via exec_subprocess to"| SS07
    SS05 -->|"dispatches sub-agents who call"| SS06
    SS05 -->|"gate writes via"| SS07
    SS06 -->|"renders output from"| SS08
    SS06 -->|"emits events via"| SS10
    SS10 -->|"reads/writes state in"| SS05
```

**Strict dependency direction:** data flows down (CC → SS09 → SS01 → SS02/SS03/SS04 → SS07). The orchestration stack (SS-05/06/07/08/10) is a separate plane. SS-07 sits at the intersection: bash hooks are invoked by both SS-04 (via legacy-bash-adapter) and SS-05/SS-06 (as gates on tool use).

## Cross-Cutting Concerns

| Concern | Owner | Mechanism |
|---------|-------|-----------|
| Observability (single-stream) | SS-03 | `events-YYYY-MM-DD.jsonl` — all lifecycle + domain events (ADR-015). Debug file `dispatcher-internal-*.jsonl` opt-in via `VSDD_DEBUG_LOG=1`; ADR-007 amended. |
| Capability enforcement | SS-01 | Deny-by-default; cap-gated host fns emit denial event + return code |
| Schema versioning | SS-01, SS-09, SS-03 | per-config: `hooks-registry.toml` schema_version=2 (post-ADR-019; v1→v2 hard-error, no compat shim); `observability-config.toml` schema_version=2 (post-ADR-015 D-15.1; v1→v2 hard-errors with migration hint per BC-3.05.004 PC4); other TOML configs schema_version=1; mismatch = hard error per DI-014 |
| Trace correlation | SS-01 | `trace_id` (UUID v4) propagated on every emitted event (renamed from `dispatcher_trace_id` per DI-017 / ADR-015 v1.7) |
| Platform selection | SS-09 | Activation skill copies `hooks.json.<platform>` (ADR-009) |
| Error non-blocking | SS-01 | Registry/payload/engine errors → `internal.dispatcher_error` → exit 0 |
| Bash hook compatibility | SS-04 | `legacy-bash-adapter.wasm` via `exec_subprocess` (ADR-012) |
| Secrets protection | SS-07 | `protect-secrets.sh` PreToolUse gate + env_allow deny-by-default |

## Architecture Decisions

| ID | Decision Summary | Subsystems | File |
|----|-----------------|------------|------|
| ADR-001 | Compiled Rust dispatcher per platform | SS-01, SS-09 | decisions/ADR-001-rust-dispatcher.md |
| ADR-002 | WASM (wasmtime) plugin ABI | SS-01, SS-02, SS-04 | decisions/ADR-002-wasm-plugin-abi.md |
| ADR-003 | WASI preview 1 for v1.0; preview 2 deferred | SS-02, SS-04 | decisions/ADR-003-wasi-preview1.md |
| ADR-004 | TOML for all configuration files | SS-01, SS-09 | decisions/ADR-004-toml-config.md |
| ADR-005 | Multi-sink observability natively in dispatcher — **SUPERSEDED by [ADR-015](decisions/ADR-015-single-stream-otel-schema.md)** | SS-01, SS-03 | decisions/ADR-005-multi-sink-observability.md |
| ADR-006 | HOST_ABI_VERSION as separate semver constant | SS-01, SS-02 | decisions/ADR-006-host-abi-version.md |
| ADR-007 | Always-on dispatcher self-telemetry — **AMENDED by [ADR-015](decisions/ADR-015-single-stream-otel-schema.md)** | SS-01, SS-03 | decisions/ADR-007-always-on-telemetry.md |
| ADR-008 | Parallel-within-tier, sequential-between-tier execution | SS-01 | decisions/ADR-008-parallel-within-tier.md |
| ADR-009 | Activation-skill-driven platform binary selection | SS-09 | decisions/ADR-009-activation-platform-selection.md |
| ADR-010 | StoreData-typed linker for host functions (invoke.rs pattern) | SS-01, SS-02 | decisions/ADR-010-storedata-linker.md |
| ADR-011 | Dual hooks.json + hooks-registry.toml during migration | SS-07, SS-09 | decisions/ADR-011-dual-hook-routing-tables.md |
| ADR-012 | Legacy-bash-adapter as universal current router | SS-04, SS-07 | decisions/ADR-012-legacy-bash-adapter-router.md |
| ADR-013 | Cycle-keyed adversarial review structure | SS-05, SS-06 | decisions/ADR-013-adversarial-review-structure.md |
| ADR-014 | Tier-2 native WASM migration (latency-primary gate + bundle-size advisory + 30MB hard kill-switch) | SS-02, SS-04, SS-07 | decisions/ADR-014-tier-2-native-wasm-migration.md |
| ADR-015 | Single-stream OTel schema + producer-side enrichment — **ACCEPTED 2026-05-04; supersedes ADR-005, amends ADR-007** | SS-01, SS-03 | decisions/ADR-015-single-stream-otel-schema.md |
| ADR-016 | Artifact path registry as single source of truth for `.factory/` canonical paths — **ACCEPTED 2026-05-07; D-340 F2** | SS-04, SS-06 | decisions/ADR-016-artifact-path-registry-sot.md |
| ADR-017 | Per-story adversarial convergence gate — three-perimeter model and WASM hook phasing — **ACCEPTED 2026-05-07; D-340 F2** | SS-04, SS-05 | decisions/ADR-017-per-story-adversary-phasing.md |
| ADR-018 | WASM-plugin Context Resolvers — design and layering for factory-agnostic runtime context injection via sandboxed WASM-plugin resolvers — **ACCEPTED 2026-05-07; D-362 F2-amendment** | SS-01, SS-04 | decisions/ADR-018-wasm-plugin-context-resolvers.md |
| ADR-019 | Plugin Async Semantics Belong at the Registry Layer — hard cut to registry-layer `async: bool` per-plugin field; envelope uniformly synchronous; dispatcher partition (sync_group/async_group); CI lint `on_error=block ⇒ async=false` — **ACCEPTED 2026-05-07; F2 async-semantics; v1.8 (F2 pass-6 fix burst close: §Consequences inline 100ms parenthetical removed; cites DI-019 by reference; F-P6-005 closed)** | SS-01, SS-07, SS-09 | decisions/ADR-019-plugin-async-semantics-at-registry-layer.md |
| ADR-020 | Dispatcher Latency Budget Classes — defines Class A (binary-spawn current model, p95 ≤ 1500ms) and Class B (daemon-mode target, TBD); AC-016 in S-15.01 is anchored to Class A; S-15.02 escalation path for Class B — **ACCEPTED 2026-05-08; F5 pass-1 path-A; v1.0; rationale clarification F-P3-006 last_amended 2026-05-08** | SS-01 | decisions/ADR-020-dispatcher-latency-budget-classes.md |

## Phase 1.4 BC Renumbering Map

This table enables downstream Phase 1.4 migration from `BC-AUDIT-NNN` to `BC-S.SS.NNN`.

| Subsystem | Name | Pass-0 source files | Approx BC-AUDIT range | Target BC-S prefix |
|-----------|------|--------------------|-----------------------|-------------------|
| SS-01 | Hook Dispatcher Core | pass-3-behavioral-contracts.md, pass-3-behavioral-contracts-deep-r1.md | BC-AUDIT-001 – BC-AUDIT-086 | BC-1 |
| SS-02 | Hook SDK and Plugin ABI | pass-3-behavioral-contracts.md (hook-sdk section) | BC-AUDIT-087 – BC-AUDIT-111 | BC-2 |
| SS-03 | Event Emission (OTel-Aligned) | pass-3-behavioral-contracts.md (sink-core, sink-file; sink-otel-grpc deprecated) | BC-AUDIT-112 – BC-AUDIT-161 | BC-3 |
| SS-04 | Plugin Ecosystem | pass-3-behavioral-contracts.md (hook-plugins section) | BC-AUDIT-162 – BC-AUDIT-191 | BC-4 |
| SS-05 | Pipeline Orchestration | pass-3-deep-agents.md, pass-3-deep-workflows.md | BC-AUDIT-787 – BC-AUDIT-1003 | BC-5 |
| SS-06 | Skill Catalog | pass-3-deep-skills-batch-1.md, batch-2.md, batch-3.md | BC-AUDIT-192 – BC-AUDIT-744 | BC-6 |
| SS-07 | Hook Bash Layer | pass-3-deep-hooks.md | BC-AUDIT-1004 – BC-AUDIT-1179 | BC-7 |
| SS-08 | Templates and Rules | pass-3-deep-templates-tools-rules.md (templates + rules) | BC-AUDIT-1180 – BC-AUDIT-1309 | BC-8 |
| SS-09 | Configuration and Activation | pass-3-behavioral-contracts.md (activation, CI config) | BC-AUDIT-744 – BC-AUDIT-763 | BC-9 |
| SS-10 | CLI Tools and Bin | pass-3-deep-templates-tools-rules.md (bin tools + commands) | BC-AUDIT-1310 – BC-AUDIT-1452 | BC-10 |
