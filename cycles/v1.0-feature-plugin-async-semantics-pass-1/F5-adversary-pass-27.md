---
document_type: adversary-review
cycle: v1.0-feature-plugin-async-semantics-pass-1
pass: 27
verdict: HIGH
adr_013_clock_after_pass: 0_of_3
producer: adversary
timestamp: 2026-05-09T20:00:00Z
strategic_recommendation: continue-protocol-per-user-directive
---

# F5 Pass-27 Adversary Review

## Verdict

**HIGH** — 10th consecutive HIGH. 3H + 2M + 2L. ADR-013 clock RESETS to 0_of_3.

META-META pattern confirmed: fix-burst-25 sub-burst 3 codified L-P26-001 mandating "verification blocks for new lessons" but did NOT add verification blocks to L-P26-001 or L-P26-002 themselves; 14+ prior lessons also missing them.

## Trajectory

→HIGH(P18..P26)→**HIGH(P27)**

## Findings

### F-P27-001 [HIGH] L-P26-002 retroactive sweep skipped 35+ historic merged stories
- L-P26-002 mandates 4-field schema (status: merged + merged_at + merged_in + merge_sha) for ALL merged stories.
- Verified: 0/8 spot-checked historic merged stories conform. ~35 stories from cycles/v1.0-brownfield-backfill/merged-stories-ledger.md merged 2026-04-26..2026-05-04 missing all merge metadata or use legacy `pr: NN` format.
- Examples: S-7.01:7, S-7.02:7, S-7.03:7, S-6.01:7, S-3.01:7, S-4.01:7, S-5.01:7 (all status: merged but no metadata); S-8.00:9, S-8.10:9, S-8.01:9, S-4.05:9 (use `pr: NN` legacy format).
- **Fix:** corpus-sweep ~35 stories; migrate `pr: NN` → `merged_in: PR-NN`; add missing fields. Append "Verified retroactively" block to L-P26-002.

### F-P27-002 [HIGH] META-META violation: codifying burst didn't add verification blocks to L-P26-001/002 + 14 prior lessons
- L-P26-001 mandates "Append a `**Verified retroactively in fix-burst-N:**` block to the lesson body".
- Lessons WITH verification blocks: L-P19-002, L-P21-001, L-P24-002 (3 of 19+).
- Lessons MISSING verification blocks: L-P18-001/002/003/004, L-P19-001, L-P20-001/002, L-P21-002, L-P22-001, L-P23-001/002, L-P24-001, L-P25-001/002, L-P26-001/002 (16 lessons).
- **Fix:** append verification blocks to all 16 lessons listing artifacts patched at codification time, OR mark each pre-L-P26-001 lesson as "exempt from retroactive verification block (codified pre-L-P26-001)".

### F-P27-003 [HIGH] L-P25-001 retroactive sweep skipped 56 of 62 merged stories
- 6 stories have POST-MERGE-STATE annotation (S-9.00, S-12.01, S-12.02, S-12.06, S-13.01, S-15.01).
- 56 merged stories without annotation, including S-7.01:220, S-8.01:294 (planning-vocabulary §Tasks).
- **Fix:** audit each unannotated merged story; apply L-P25-001 (b) annotation where planning-vocabulary §Tasks exists.

### F-P27-004 [MEDIUM] L-P25-002 carve-out scope drift (F1 vs F1-amendment)
- L-P25-002 narrative says `phase: F1`; carve-out applied to `phase: F1-amendment` artifact.
- **Fix:** update L-P25-002 lessons text to expand predicate.

### F-P27-005 [MEDIUM] VP-INDEX Proof Method drift (kani vs kani-proof)
- VP-INDEX line 113 Breakdown table uses "kani-proof" with count 3.
- VP-070 (line 189) uses "kani"; VP-071 (line 190) uses "kani"; VP-077 (line 196) uses "kani-proof".
- **Fix:** harmonize to "kani-proof" canonical form.

### F-P27-006 [LOW] L-P26-002 doesn't address legacy `pr: NN` migration
- Several stories use `pr: NN` instead of `merged_in: PR-NN`.
- **Fix:** update L-P26-002 to explicitly retire `pr:` field with migration path.

### F-P27-007 [LOW] BC-INDEX changelog drops `last_amended:` recap at v1.46+
- v1.45 and prior have inline `last_amended:` recap; v1.46-v1.50 omit.
- **Fix:** backfill v1.46-v1.50 changelog rows with `last_amended:` recap.

## Notable observations

- F-P26-001 PluginEntry sweep verified complete (0 active-body matches).
- F-P26-007 VP-077 harness PluginPartition struct destructuring verified.
- F-P26-002 partially closed (6 of 62 merged stories annotated; F-P27-003 expands).
- F-P26-003 F1 carve-outs verified.
- F-P26-004 + F-P26-005 frontmatter retrofit verified for 5 stories.
- Index versions confirmed: BC-INDEX v1.50, VP-INDEX v1.34, STORY-INDEX v2.55, ARCH-INDEX v1.30.

## Convergence assessment

Novelty HIGH. 10 consecutive HIGH; META-META pattern self-evident. Per user directive: continue protocol.
