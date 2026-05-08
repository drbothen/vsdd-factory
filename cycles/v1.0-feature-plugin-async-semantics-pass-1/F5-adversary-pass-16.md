# F5 Adversary Pass-16 — S-15.01 fix-burst chain (against 7f2914b6)

## Sanity probes — All confirmed
## Pass-15 finding resolutions verified — F-P15-001..007 + O-P15-001/003 ALL RESOLVED

## Verdict: HIGH (substantive findings; meta-pattern surfaced)
Trajectory: 17→15→6→5→0→2→5→1→4→2→2→4→4→5→7→5

## Counts
H: 2  M: 2  L: 1  + 3 observations

## META-OBSERVATION
Fix-burst-14 codified TD-031 (post-EC-012 line drift TD) but ITSELF introduced 5 NEW line-drift defects within the same fix burst. TD-031 surface-only codification has no enforcement. Pattern recurs.

## Findings (NEW)

### F-P16-001 [HIGH] VP-079 v1.14 SITES line citations stale (4 of 5); bats Scenario 6 mutation harness will produce INFRA-ERROR not detection
- VP-079.md:84-87, 465-469, 526-530 cite stale lines (drift 1, 5, 7, 7 lines)
- Actual main.rs SITES: 133, 143, 167, 423, 434 — cited 133, 142, 162, 416, 427
- bats:213-214, 285-286 multi-line ranges 142,147 + 162,167 — actual 143,150 + 167,174
- Bats sed mutation will cause unbalanced parens → cargo build fail → caught=2 INFRA-ERROR (skip, not detect)
- Fix: stable anchors (TD-031) OR refresh all 4+2 cites

### F-P16-002 [HIGH] BC-3.08.001 v1.9 amendment cites wrong line (289 → actual 300)
- Line 289 of emit_event.rs is doc-comment; actual emit_plugin_timeout_async with_session_id call is line 300
- Same amendment line 201 cites reserved_fields_rejected at line 348 — actual line 359
- Drift introduced WITHIN fix-burst-14 (the burst that codified TD-031)
- Fix: BC-3.08.001 v1.9→v1.10 line cite refresh OR stable anchors

### F-P16-003 [MEDIUM] S-15.01 ACs don't propagate BC-3.08.001 v1.9 §Common Fields
- AC-011/012/013/014 mandatory-field lists missing session_id
- "all 6 mandatory fields" stale (now 7 per BC v1.9)
- T-3e implementer guidance also stale
- Fix: S-15.01 v1.18→v1.19 amend AC text 6→7 + add session_id (5 sites)

### F-P16-004 [MEDIUM] VP-079 + S-15.01 + bats cite stale line range "581-700" for Scenario 8 (actual 600-732); §Scenario 7 cites "475-565" (actual 486-605); §Scenario 8b prose absent
- Triple-document drift class — exact pattern TD-031 codified to prevent
- Bats Scenario 8b (added in fix-burst-14 per F-P15-002) has no §Scenario 8b spec block in VP-079
- Wildcard test (Rust + bats) lacks AC-018 traceability
- Fix: stable anchors + add §Scenario 8b prose + AC-018 enumerate S8b/wildcard

### F-P16-005 [LOW] BC-3.08.001 v1.9 §Common Fields paragraph contradicts Events 1+4 wire examples
- §Common Fields says plugin_name "omitted from examples for readability" but Events 1+4 examples include it
- Logical inconsistency
- Fix: differentiate plugin-context (Events 1+4) vs dispatcher-startup (Events 2+3) omissions

### O-P16-001 [process-gap] TD-031 codification surface-only — no enforcement; same fix-burst that codified it introduced 5 violations
- Recommend escalating TD-031 P2→P1 + drafting enforcement (lint-hook OR PR template)
- Decision pending user adjudication

### O-P16-002 [stale] VP-079 v1.14 §Scenario 8 prose cites BC-3.08.001 v1.8 (current v1.9) — within-fix-burst staleness

### O-P16-003 [inputs] VP-079 v1.14 frontmatter inputs: missing BC-7.06.001.md (bcs: cites it; input-hash won't recompute on BC-7 changes)

## ADR-013 clock
- Pass-15: HIGH (RESET 0)
- Pass-16: HIGH
- Counter: 0_of_3

## Strategic note
This is the 16th pass. User directive (after pass-14): "continue with A until we fix everything." Fix-burst-15 will close all pass-16 findings. After fix-burst-15 + pass-17, if substantive defects continue surfacing, strategic re-adjudication may be warranted given pass-16's meta-observation that TD-031 surface-only codification fails without enforcement.
