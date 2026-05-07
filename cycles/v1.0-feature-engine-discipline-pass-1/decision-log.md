---
document_type: cycle-decision-log
producer: state-manager
cycle: v1.0-feature-engine-discipline-pass-1
version: "1.0"
---

# v1.0-feature-engine-discipline-pass-1 Cycle Decision Log

Seed decisions recorded at cycle open (2026-05-06). All subsequent decisions will be
appended here per POLICY 1 (append-only, immutable IDs).

| ID | Decision | Rationale | Phase | Date | Made By |
|----|----------|-----------|-------|------|---------|
| D-336 | Cycle opened with two-cluster scope: (a) per-story adversarial convergence loop (documented in orchestrator MANDATORY STEPS but unimplemented in `per-story-delivery.md`), and (b) artifact path governance (WASM hook + path registry + relocation skill). Scope was user-decided after a `feature-deltas/` path-invention error during F1 dispatch surfaced the path-validation need. | Two governance gaps independently identified; co-locating in one cycle reduces release overhead and shares WASM toolchain work. Path-invention error was the triggering event that confirmed the need for path governance enforcement. | F1 | 2026-05-06 | user + orchestrator |
| D-337 | WASM-only for new hooks introduced by this cycle. Tier E Bash→WASM migration is in flight; new hooks MUST NOT introduce additional Bash hook debt. Any hook authored in this cycle must be a WASM binary. | Reduces long-term maintenance burden and aligns with Tier E migration trajectory. Bash hook debt is the root cause being addressed; adding more Bash hooks would worsen the problem. | F1 | 2026-05-06 | user |
| D-338 | Cycle name `v1.0-feature-engine-discipline-pass-1` chosen with `pass-1` suffix anticipating future engine-discipline cycles. Naming anticipates that engine governance improvements will recur across multiple cycles as the engine matures. | Pattern established by brownfield-backfill wave numbering (Wave 1..N). Engine discipline work is ongoing; `pass-1` suffix reserves the naming namespace for continuations. | F1 | 2026-05-06 | user + state-manager |
| D-339 | F1 architect agent recommended 3-story decomposition + 2 epics: S-A (per-story adversary workflow + agent docs), S-B (per-story adversary WASM hook), S-C (artifact path governance bundle). Epics: Engine Governance (S-A + S-B) and Artifact Integrity (S-C). F2 to confirm story boundaries and assign epic IDs. | 3-story split aligns with natural subsystem boundaries: agent-document changes (S-A) are low-blast-radius and shippable independently; WASM hook (S-B) requires Rust implementation; path governance (S-C) is a cross-cutting bundle. | F1 | 2026-05-06 | architect (F1 output) |
