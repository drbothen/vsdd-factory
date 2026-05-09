---
document_type: adversary-review
cycle: v1.0-feature-plugin-async-semantics-pass-1
pass: 26
verdict: HIGH
adr_013_clock_after_pass: 0_of_3
producer: adversary
timestamp: 2026-05-09T18:00:00Z
strategic_recommendation: continue-protocol-per-user-directive
---

# F5 Pass-26 Adversary Review

## Verdict

**HIGH** — 9th consecutive HIGH. 5H + 2M + 7L. ADR-013 clock RESETS to 0_of_3.

## Trajectory

→HIGH(P18..P25)→**HIGH(P26)**

## Findings

### F-P26-001 [HIGH] `PluginEntry` fabricated struct pervades VP-077 (8+ sites), VP-078 (4 sites), S-15.01 (3 sites)
- Real struct: `RegistryEntry` at `crates/factory-dispatcher/src/registry.rs:171`. No `PluginEntry` exists.
- Sites:
  - VP-077: lines 46, 109-110, 146-147, 178, 200-201
  - VP-078: lines 316, 325, 347, 369
  - S-15.01: lines 430-431, 759
- Class: fabricated registry-entry-related struct names (sibling of RegistryEntry.async). Survived F-P22-002, F-P24-002, F-P25-001..004, L-P24-002 corpus sweep.
- **Fix:** Replace ALL `PluginEntry` → `RegistryEntry`. Update L-P24-002 catalog.

### F-P26-002 [HIGH] L-P25-001 only applied to S-15.01 trigger; 6+ merged stories skipped
- L-P25-001 codified at fix-burst-24 sub-burst 2 prescribes POST-MERGE-STATE annotation for merged stories with planning §Tasks.
- Verified merged stories WITHOUT annotation: S-9.00 (PR-91), S-13.01 (PR-97), S-12.01 (PR-98), S-12.02 (PR-99), S-12.06 (PR-105), S-7.03, S-8.01.
- Per L-P24-002, codifying burst MUST corpus-sweep — fix-burst-24 sub-burst 2 patched only S-15.01.
- **Fix:** Audit each merged story for planning-vocabulary §Tasks; apply POST-MERGE-STATE annotation.

### F-P26-003 [HIGH] L-P25-002 only applied to plugin-async-semantics F1-delta; engine-discipline cycle's F1-delta + F1-platform-amendment-delta skipped
- 3 artifacts match `producer: architect, phase: F1, status: draft`:
  1. cycles/v1.0-feature-plugin-async-semantics-pass-1/F1-delta-analysis.md ✓ (carve-out present)
  2. cycles/v1.0-feature-engine-discipline-pass-1/F1-delta-analysis.md ✗ (no carve-out)
  3. cycles/v1.0-feature-engine-discipline-pass-1/F1-platform-amendment-delta-analysis.md ✗ (phase: F1-amendment — adjudication needed)
- **Fix:** Add HTML carve-out to (2). Adjudicate (3) re: phase: F1-amendment scope.

### F-P26-004 [HIGH] S-15.01 frontmatter `status: ready` despite PR #106 merge
- L-P25-001 source narrative explicitly cites this exact failure mode.
- Fix-burst-24 added §Architecture Mapping POST-MERGE annotation but NOT frontmatter status update.
- **Fix:** S-15.01 frontmatter: `status: merged`, `merged_at: 2026-05-08`, `merged_in: PR-106`, `merge_sha: 453eee1`.

### F-P26-005 [HIGH] Four engine-discipline stories merged but frontmatter says `status: draft`
- S-13.01 (PR-97 2c97cb0), S-12.01 (PR-98 2e9b670), S-12.02 (PR-99 e2fd3d4), S-12.06 (PR-105 15432c6)
- Sibling-class to F-P26-004. Process gap: state-manager PR-merge handler does not update frontmatter.
- **Fix:** Update each frontmatter with `status: merged` + `merged_at:` + `merged_in:` + `merge_sha:`.

### F-P26-006 [MEDIUM process-gap] L-P24-002 violated by codification of L-P25-001/002 in fix-burst-24 sub-burst 2
- L-P24-002 mandates corpus-wide grep at codification time; F-P26-002/003/005 prove this was not done.
- **Codify:** Every codifying burst MUST run L-P24-002 corpus-sweep on new lesson at codification time, with verification block in lessons.md.

### F-P26-007 [MEDIUM] VP-077 harness skeleton return-type drift — tuple vs PluginPartition struct
- VP-077:130, 180: `let (sync_g, async_g) = partition_plugins(...)` — production returns `PluginPartition` struct, not tuple.
- **Fix:** Update harness to use struct destructuring `let part = partition_plugins(...); let sync_g = &part.sync_group;`.

## Notable observations

- Fix-burst-24 closure verified for trigger artifact only.
- Index versions: BC-INDEX v1.50, VP-INDEX v1.33, STORY-INDEX v2.54, ARCH-INDEX v1.30 — all current.
- POLICY 9 arithmetic: VP total 79 verified.

## Convergence assessment

9 consecutive HIGH. Each codifying burst codifies a slightly broader rule but applies it narrowly. Each next pass finds NEW sibling-class instances at NEW layers.

Per user directive: continue protocol; no escalation.
