---
document_type: adversarial-convergence-record
story: S-4.05
title: Dead Letter Queue Implementation
wave: 13
subsystem: SS-03
final_pass: 48
final_version: v1.45
sealed_commit: ac22a3d
branch: factory-artifacts
convergence_reached: 2026-04-28
convergence_criterion: ADR-013 (3 consecutive NITPICK_ONLY passes)
---

# S-4.05 Spec Convergence Record — 48-Pass Adversarial Run

## Summary

S-4.05 (Dead Letter Queue Implementation) achieved CONVERGENCE_REACHED at pass-48
per ADR-013. This is the longest adversarial run in project history, eclipsing
S-7.03's previous record of 17 passes.

- **Final version:** v1.45
- **Sealed commit:** ac22a3d on factory-artifacts
- **Convergence criterion:** passes 46/47/48 all NITPICK_ONLY; clock 1/3 → 2/3 → 3/3

## Convergence Trajectory

```
Pass 1:  11 findings (HIGH=4, MED=5, LOW=2)
Pass 2:  5
Pass 3:  8
Pass 4:  8
Pass 5:  8
Pass 6:  3
Pass 7:  0  ← first clean pass
Pass 8:  3
Pass 9:  5
Pass 10: 1
Pass 11: 2
Pass 12: 1
Pass 13: 2
Pass 14: 0
Pass 15: 2
Pass 16: 2
Pass 17: 0
Pass 18: 1
Pass 19: 4
Pass 20: 2
Pass 21: 2
Pass 22: 2
Pass 23: 2
Pass 24: 1
Pass 25: 1 HIGH  ← architectural restructure trigger (mkdir-p revocation, signature generalization, UTC handling)
Pass 26: 4
Pass 27: 5
Pass 28: 6
Pass 29: 2
Pass 30: 7
Pass 31: 6  ← F-3101 architectural cycle resolution (highest-leverage fix in run)
Pass 32: 8
Pass 33: 8
Pass 34: 6
Pass 35: 5
Pass 36: 4
Pass 37: 5
Pass 38: 4
Pass 39: 3
Pass 40: 7
Pass 41: 7
Pass 42: 7  ← pass-42 caught 41-pass-old structural gap (DLQ template missing directory prefix)
Pass 43: 8
Pass 44: 5
Pass 45: 5
Pass 46: 3 LOW  ← NITPICK_ONLY; clock 1/3 (no fix burst applied per S-7.03 lesson)
Pass 47: 6 LOW  ← NITPICK_ONLY; clock 2/3
Pass 48: 0      ← NITPICK_ONLY (0 findings); clock 3/3 = CONVERGENCE_REACHED
```

## Carry-Forward LOWs (non-blocking per ADR-013)

| ID | Description |
|----|-------------|
| F-4601 | Task 6b try_into() map_err enrichment |
| F-4602 | AC-009 prune-test absence as TD |
| F-4603 | Task 2b ordering documented at separate site |
| F-4701 | FileSink delegation arity in narrative sentence |
| F-4702 | write_event Result discard let _ |
| F-4703 | Task 5 emission skeleton tail return |

All 6 are mechanical drift with canonical patterns documented elsewhere.
Non-blocking per ADR-013; preserved in spec as v1.0.1+ candidates.

## Major Architectural Decisions Made During This Run

| Pass | Decision |
|------|----------|
| pass-2 | SinkDlqEvent placed in sink-core (not factory-dispatcher) to resolve dependency cycle; boundary adapter at factory-dispatcher |
| pass-3 | DlqWriter threading via dlq_writer: Option&lt;Arc&lt;DlqWriter&gt;&gt; field on each Sink struct |
| pass-3 | new_with_observability constructor variant + worker_loop signature extension |
| pass-5 | DLQ rooting via dlq_root: PathBuf field on DlqWriterConfig matching InternalLog::log_dir via main.rs::resolve_log_dir() |
| pass-7 | UTC clock injection seam via clock_fn: Arc&lt;dyn Fn() -&gt; DateTime&lt;Utc&gt; + Send + Sync&gt; on DlqWriter |
| pass-11 | Mutex&lt;Option&lt;(PathBuf, File, u64)&gt;&gt; cache shape for byte-counted size cap rotation |
| pass-14 | try_send fire-and-forget channel semantics matching SinkErrorEvent precedent |
| pass-25 | per-event retry-exhaustion DLQ loop in post_batch |
| pass-31 | F-3101: full architectural cycle resolution — canonical module boundary documented; dependency graph stabilized |

## Lessons Codified

1. **S-7.03 NITPICK-skip strategy validated again** — 3 consecutive clean passes achieved via
   no-fix-burst posture at pass-46/47/48. Applying fixes to NITPICK_ONLY findings extends
   the run by introducing new surface area.

2. **Fresh-context compounding value confirmed past pass-30** — pass-42 caught a 41-pass-old
   structural gap (DLQ template missing directory prefix). Long runs benefit from periodic
   fresh-context adversarial re-reads.

3. **Architectural cycle resolution at pass-31 was the highest-leverage fix** — F-3101
   stabilized the dependency graph; subsequent passes refined but never re-opened the
   core architecture.

4. **Pass-25/26/27 burst introduced the architectural restructure** (mkdir-p revocation,
   signature generalization, UTC handling); passes 28-37 refined the resulting surface
   area through 9 substantive fix bursts.

## Pass Archive Location

Pass artifacts (1-48) are in the story's spec file at:
`.factory/stories/S-4.05-dead-letter-queue.md` (v1.45, ac22a3d)

Full per-pass adversarial review files are not individually archived for this run
(only pass-1 exists at `wave-13-ss-03-S-4.05-pass-1.md`). The convergence trajectory
above is the canonical record per the content-routing rules in STATE-MANAGER role spec.
