# F5 Adversary Pass-4 — S-15.01 fix-burst-3 (against f5bed48)

## Sanity probes
All 6 probes PASS. Working tree at f5bed48; partition.rs has (name, event, tool); BC-7.06.001 has String-equality clause; DI-019 has Malformed value handling; VP-079 has full fn names; bats H2 has only canonical signals.

## Pass-3 finding resolutions verified
F-P3-001/003/004/005/006 RESOLVED. F-P3-002 PARTIALLY RESOLVED (H1 updated; H2/H4 doc-comment gaps + stale v1.4 cite — see F-P4-002/003).

## Verdict: MEDIUM
Trajectory: 17 → 15 → 6 → 5 (improving but slowing)

## Counts
H: 0  M: 1  L: 4  NIT: 0

## Findings (NEW)

### F-P4-001 [MEDIUM] VP-077 v1.9 has 4 stale BC-7.06.001 v1.4 cites — propagation gap from F-P3-003 (BC bumped to v1.5)
- Axis: A, F, S-7.01 partial-fix
- Source: BC-7.06.001 v1.5 (per F-P3-003); BC-INDEX line 229 tracks v1.5
- Defect: VP-077 v1.9 retains BC-7.06.001 v1.4 Invariant 7 cite at 4 sites:
  - Line 66 (Property Statement Precondition)
  - Line 113 (Harness 1 doc-comment)
  - Line 150 (Harness 2 doc-comment)
  - Line 204 (Harness 4 doc-comment)
- Fix: Sweep VP-077 v1.9 → v1.10. Update all 4 sites: BC-7.06.001 v1.4 → v1.5. POLICY 1 — preserve historical changelog entries.
- Tag: [content-defect]

### F-P4-002 [LOW] partition.rs:150 cites stale BC-7.06.001 v1.4 — sibling of F-P4-001
- Axis: A, S-7.01
- Source: partition.rs:150 reads "BC-7.06.001 v1.4 Invariant 7"
- Defect: F-P3-002 fix-burst-3 commit f5bed48 updated tuple to (name, event, tool) but did NOT update BC version cite from v1.4 to v1.5. Same change vector as F-P4-001.
- Fix: partition.rs:150 cite BC-7.06.001 v1.4 → v1.5
- Tag: [content-defect]

### F-P4-003 [LOW] partition.rs H2 and H4 doc-comments lack tuple-uniqueness precondition note that VP-077 §F-P2-011 amendment requires
- Axis: A, S-7.01 (pending intent verification)
- Source: VP-077 v1.9 §Amendment lines 416-418 explicitly state "§Proof Harness Skeleton H1, H2 kani::assume block, H4 kani::assume block: ... comment references H1 and BC-7.06.001 v1.4 Invariant 7."
- Defect: H1 has the precondition note (line 149-151); H2 (lines 173-180) and H4 (lines 245-252) have no tuple-uniqueness reference. Implementation strategy uses by-construction uniqueness (format!("plugin-{}", i)) instead of kani::assume — but VP-077 amendment text doesn't acknowledge that.
- Fix: Either (a) add brief precondition-note doc-comments to H2 and H4 mirroring H1's pattern, OR (b) amend VP-077 to relax the H2/H4 amendment text. Recommend (a) for consistency.
- Tag: [content-defect] (pending intent verification)

### F-P4-004 [LOW] DI-019 v1.3 §Malformed value handling clause does not cover VSDD_ASYNC_DRAIN_WINDOW_MS=0 (parses → 0ms drain → catastrophic truncation)
- Axis: A, G
- Source: invariants.md:150 says malformed values "cannot be parsed as a valid u64"; 0 is a valid u64
- Defect: VSDD_ASYNC_DRAIN_WINDOW_MS=0 → effective_drain_window=Duration::from_millis(0) → all async terminal events truncated. Debug-build operator footgun.
- Fix: Either (a) extend §Malformed value handling to acknowledge 0ms truncation behavior, OR (b) clamp value with min(value, 1) in main.rs:308-312
- Tag: [content-defect]

### F-P4-005 [LOW] DI-019 v1.3 §Malformed value handling does not cover absurdly-large values (e.g., 99999999999 → minutes-long debug-build drain)
- Axis: A, G
- Source: same as F-P4-004
- Defect: Pathological values parse as u64; Duration::from_millis is unbounded; CI hang on typo
- Fix: Either (a) extend §Malformed clause to acknowledge no upper-bound clamp, OR (b) clamp value with max(value, 60000) in main.rs:308-312
- Tag: [content-defect]

## ADR-013 clock
- Pass-1: HIGH (0_of_3)
- Pass-2: HIGH (0_of_3)
- Pass-3: MEDIUM (0_of_3)
- Pass-4: MEDIUM (0_of_3)
- Counter: 0_of_3 — does NOT advance (NITPICK_ONLY required)

## Notes

**Convergence trajectory:** 17 → 15 → 6 → 5. Slowing but improving. Pass-5 should be NITPICK_ONLY if fix-burst-4 closes all 5.

**Process-gap codification candidates (NEW):**
1. When a BC version bumps mid-burst, ALL doc citing the BC version must be swept in the same burst (CI grep gate). F-P4-001 + F-P4-002 demonstrate the propagation gap recurring within the SAME burst that authored the BC bump.
2. Spec amendment text prescribing implementation changes to MULTIPLE harnesses must either (a) be mechanically enforced or (b) relaxed when implementation strategy diverges (F-P4-003 — VP-077 §F-P2-011 amendment text exceeded actual implementation scope).

**Recommendation:** Fix-burst-4 should:
- Sweep VP-077 v1.9 → v1.10 (F-P4-001) + partition.rs:150 (F-P4-002) — same change vector
- Add H2/H4 doc-comment precondition notes (F-P4-003 option a)
- Extend DI-019 §Malformed value handling with edge-case ack (F-P4-004 + F-P4-005 option a — documentation, not behavioral change)

Pass-5 has high probability of NITPICK_ONLY → ADR-013 clock 1_of_3.
