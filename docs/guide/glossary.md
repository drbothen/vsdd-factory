# Glossary

Terminology used throughout the VSDD factory plugin, alphabetized.

---

**AC (Acceptance Criterion)**
A testable condition within a story that must be satisfied for the story to be complete. Numbered as AC-NNN within each story file (local scope — AC-001 in STORY-001 is unrelated to AC-001 in STORY-002). Every AC must trace to a BC precondition or postcondition.

**AD (Architecture Decision)**
A recorded architecture decision with rationale, alternatives considered, and consequences. Numbered as AD-NNN. Lifecycle-scoped. Stored in ARCH-INDEX.md.

**ADV Finding (Adversarial Finding)**
A defect found by the adversary agent during adversarial review. Numbered as `ADV-<CYCLE>-P<PASS>-<SEV>-<SEQ>` (e.g., `ADV-P1CONV-P03-CRIT-001`). Cycle-scoped — IDs reset per convergence cycle. Severity codes: CRIT, HIGH, MED, LOW.

**Anchor Justification**
Requirement that agents creating anchors (BC↔capability, story↔subsystem, VP↔story) must explicitly state and justify their choice citing the source-of-truth document. Prevents force-fitting to "next available" IDs. Enforced by policy `creators_justify_anchors`.

**Append-Only Numbering**
Governance policy that all VSDD identifiers (BC, CAP, VP, EC, DI, STORY, HS) are never renumbered or reused. When an artifact is removed, the old ID stays in indexes with `status: retired`. Filename slugs are immutable even when titles change. Enforced by policy `append_only_numbering`.

**ASM (Assumption)**
An assumption requiring validation, identified during domain specification. Numbered as ASM-NNN. Lifecycle-scoped. Registry: L2-INDEX.md.

**BC (Behavioral Contract)**
A formal, testable specification of what a module or function must do. Expressed as preconditions, postconditions, and invariants. Numbered as BC-S.SS.NNN (section.subsection.contract). Part of the L3 specification level. Once a BC reaches `green` status, it is immutable — enforced by `protect-bc.sh`. Hook-validated by `validate-bc-title.sh` (H1 ↔ INDEX sync), `validate-story-bc-sync.sh` (story ↔ BC sync).

**Brownfield Ingest**
Phase 0 of the VSDD pipeline. Analyzes an existing codebase through a broad-then-converge protocol (7 broad passes, then iterative deepening until novelty decays). Produces a semantic understanding that feeds into spec crystallization.

**CAP (Domain Capability)**
A capability identified during domain specification. Numbered as CAP-NNN. Lifecycle-scoped. Registry: L2-INDEX.md. Referenced by BCs (`capability:` frontmatter), epics, and stories.

**CMP (UI Component)**
A UI component within a screen specification. Numbered as CMP-NNN. Local scope — meaningful only within its parent SCR file.

**Compact State**
The process of extracting historical content (burst logs, adversary passes, session checkpoints, lessons) from STATE.md into cycle-scoped files, keeping STATE.md under 200 lines. Invoked via `/vsdd-factory:compact-state`.

**Convergence**
The state where further review produces no meaningful new findings. Measured quantitatively across seven dimensions (spec, tests, implementation, verification, visual, performance, documentation). The pipeline is complete when all seven dimensions independently report CONVERGED.

**Convergence Index**
A composite metric: `CI(i) = (Novelty * (1 - AvgSimilarity) * (6 - MedianSeverity)) / Cost`. Converged when CI < 0.3 and declining for 3+ consecutive iterations.

**Convergence Tracker**
A deterministic hook (`convergence-tracker.sh`) that enforces convergence rules at write-time: trajectory monotonicity, minimum 3 clean passes for CONVERGENCE_REACHED, novelty score ≤ 0.15, and zero-findings first pass warning. Does not replace the full DF-009 convergence-tracker plugin (which handles semantic similarity and hallucination fingerprinting).

**Convergence Trajectory**
The monotonically decreasing sequence of finding counts across adversarial passes (e.g., 29→24→21→7→4→3→2→0). If findings increase between passes, this is a regression that blocks further convergence.

**CR (Code Review Finding)**
A finding from code review. Numbered as CR-NNN. Cycle-scoped.

**DEC (Domain Edge Case)**
An edge case identified during domain specification. Numbered as DEC-NNN. Lifecycle-scoped. Registry: L2-INDEX.md.

**DI (Domain Invariant)**
A domain-level business rule or constraint. Numbered as DI-NNN. Lifecycle-scoped. Every DI must be enforced by at least one BC (policy `lift_invariants_to_bcs`). Registry: L2-INDEX.md.

**DTU (Digital Twin Universe)**
A clone of a reference implementation used for behavioral equivalence testing. DTU clones are compared against the new implementation to verify that ported behavior is preserved.

**E-xxx-NNN (Error Taxonomy Entry)**
An error code in the error taxonomy. Format: `E-` + subsystem abbreviation + `-` + three-digit number (e.g., `E-NET-001`). Lifecycle-scoped. Registry: error-taxonomy.md.

**EAC (Epic Acceptance Criterion)**
An acceptance criterion within an epic. Numbered as EAC-NNN. Local scope — within epic file.

**EC (Edge Case)**
An edge case within a behavioral contract. Numbered as EC-NNN. Local scope — EC-001 in one BC is unrelated to EC-001 in another.

**ELM (UI Element)**
A UI element within a screen specification. Numbered as ELM-NNN. Local scope — within parent SCR file.

**Epic (EPIC-NNN)**
A group of related stories that deliver a coherent product capability. Epics map to L2 domain capabilities (CAP-NNN) and subsystems (SS-NN). Lifecycle-scoped.

**EVAL (Holdout Evaluation Result)**
The result of evaluating a holdout scenario. Format: `EVAL-HS-NNN-P<pass>`. Cycle-scoped. Registry: EVAL-INDEX.md.

**Factory Artifacts**
The orphan git branch (`factory-artifacts`) that holds all pipeline state, specs, stories, and evaluation artifacts. Mounted as a worktree at `.factory/` in the target project. Separate from `main` and `develop`.

**Fix PR (FIX-P[N]-NNN)**
A pull request created to address an adversarial finding, hardening issue, or convergence gap. The phase prefix indicates origin: P5 = adversarial refinement, P6 = formal hardening, P7 = convergence. Lifecycle-scoped.

**FM (Failure Mode)**
A failure mode identified during domain specification. Numbered as FM-NNN. Lifecycle-scoped. Registry: L2-INDEX.md.

**Fresh Context**
A deliberate constraint on adversarial review: the adversary agent starts each review pass with no memory of prior passes. This prevents relationship drift and ensures each pass provides genuinely independent evaluation.

**GAP Register (GAP-NNN)**
A deferred requirement that cannot be implemented in the current cycle. Gap entries are tracked in traceability matrices with justification (minimum 10 characters) and a resolution target version. Lifecycle-scoped — gaps persist until resolved.

**Gene Transfusion**
An implementation strategy where a story is built by porting behavioral intent from an existing reference implementation (analyzed via brownfield ingest or semport) rather than writing from scratch. The implementation uses the reference as a guide but adapts to the target language and architecture.

**Governance Policy**
A top-level integrity rule that prevents a specific class of drift. Enforced by multiple agents and validated by consistency-validator criteria. The 9 policies are: `append_only_numbering`, `lift_invariants_to_bcs`, `state_manager_runs_last`, `semantic_anchoring_integrity`, `creators_justify_anchors`, `architecture_is_subsystem_name_source_of_truth`, `bc_h1_is_title_source_of_truth`, `bc_array_changes_propagate_to_body_and_acs`, `vp_index_is_vp_catalog_source_of_truth`.

**Holdout Scenario (HS-NNN)**
A hidden acceptance test created during story decomposition that the builder and test-writer agents never see. Evaluated by an independent holdout-evaluator agent (different model, fresh context) against the running system. Enforces train/test separation. Lifecycle-scoped. Registry: HS-INDEX.md.

**ID Scope**
The persistence boundary of a VSDD identifier. Lifecycle-scoped IDs (BC, VP, STORY, SS) are append-only and never reused across cycles. Cycle-scoped IDs (ADV findings, WHS) reset per convergence cycle. Local IDs (EC, AC, CMP) are scoped to a parent artifact.

**Information Asymmetry**
A deliberate design principle where different agents have access to different information. The adversary cannot see prior reviews. The holdout evaluator cannot see specs, source code, or implementation notes. This prevents gaming and ensures independent evaluation.

**INT (UI Interaction)**
A UI interaction within a screen specification. Numbered as INT-NNN. Local scope — within parent SCR file.

**Integration Surface Taxonomy**
The six universal categories used in DTU assessment to classify external system dependencies by data flow direction and business role: inbound data sources, outbound operations, identity & access, persistence & state, observability & export, enrichment & lookup.

**Invariant Lifting**
The requirement that every domain invariant (DI-NNN) declared in `domain-spec/invariants.md` must be cited by at least one behavioral contract's Traceability L2 Invariants field. Orphan invariants (declared but no BC enforces them) are drift findings. Enforced by policy `lift_invariants_to_bcs`.

**Iron Law**
A non-negotiable behavioral constraint on a critical pipeline skill, expressed in the form "NO [action] WITHOUT [prerequisite] FIRST." The 9 Iron Laws are:
1. deliver-story: NO IMPLEMENTATION WITHOUT RED GATE VERIFICATION FIRST
2. adversarial-review: NO APPROVAL WITHOUT FRESH-CONTEXT REVIEW FIRST
3. brownfield-ingest: NO ROUND COMPLETION WITHOUT HONEST CONVERGENCE CHECK FIRST
4. wave-gate: NO WAVE ADVANCE WITHOUT ALL SIX GATES PASSING FIRST
5. decompose-stories: NO STORY WITHOUT BC TRACEABILITY FIRST
6. holdout-eval: NO HOLDOUT EVALUATION WITHOUT INFORMATION ASYMMETRY FIRST
7. convergence-check: NO RELEASE WITHOUT ALL SEVEN DIMENSIONS CONVERGED
8. formal-verify: NO HARDENING SIGN-OFF WITHOUT ALL PROOF HARNESSES PASSING
9. create-architecture: NO ARCHITECTURE WITHOUT VERIFICATION FEASIBILITY ASSESSMENT

**NFR (Non-Functional Requirement)**
A non-functional requirement (performance, security, reliability, etc.). Numbered as NFR-NNN. Lifecycle-scoped. Registry: prd.md.

**NITPICK**
The strict-binary novelty classification for a convergence round where findings are cosmetic or trivial. The counterpart of SUBSTANTIVE. Only the literal token NITPICK counts — "effectively converged" or "borderline NITPICK" is treated as SUBSTANTIVE.

**Novelty Assessment**
A mandatory section in every adversarial review file. Contains 7 required fields: Pass number, New findings count, Duplicate/variant count, Novelty score, Median severity, Trajectory, and Verdict. Enforced by `validate-novelty-assessment.sh` hook.

**Novelty Score**
The fraction of genuinely new findings in an adversarial pass: `Novelty(i) = N(i) / (N(i) + D(i))` where N = new findings and D = duplicate/variant findings. Converged when < 0.15 for 2+ consecutive passes (85%+ are duplicates).

**Phase Gate**
A quality checkpoint between pipeline phases. Each gate has defined pass/fail criteria. No phase may begin until the previous phase's gate passes. The orchestrator agent is the only entity that transitions between phases.

**R (Risk)**
A risk identified during domain specification. Numbered as R-NNN. Lifecycle-scoped. Registry: L2-INDEX.md.

**Red Flags Table**
A table embedded in discipline skills that enumerates the rationalizations agents use to bypass Iron Laws. Each row maps a tempting thought ("I'll skip the Red Gate for this small change") to the reality of why it is a protocol violation. 80 Red Flag entries across 9 skills.

**Red Gate**
The TDD checkpoint in Phase 3 where all tests must fail before any implementation begins. If a test passes without implementation, the test is suspect. Enforced by the `red-gate.sh` hook when strict mode is active.

**SCR (UX Screen)**
A UX screen specification. Numbered as SCR-NNN. Lifecycle-scoped. Registry: UX-INDEX.md.

**SEC (Security Finding)**
A finding from security review. Numbered as SEC-NNN. Cycle-scoped.

**Semantic Anchoring**
The principle that every anchor in the spec system (BC↔capability, story↔subsystem, VP↔story, traceability descriptions) must be semantically correct, not merely syntactically valid. An anchor must make sense if you read both source and target. Mis-anchoring always blocks convergence.

**Semport (Semantic Port)**
The process of extracting behavioral intent from a reference implementation and designing a translation strategy to a target language. Used when porting existing systems. Distinct from brownfield ingest, which understands what exists; semport translates it.

**Single Source of Truth**
The rule that every metric (BC count, story count, VP count, wave assignment) has one authoritative source document. All other documents cite the authoritative source — they do not independently re-derive the value.

**SOUL (Agent Principles)**
The set of governing principles that every agent in the factory follows. Defined in AGENT-SOUL.md. Includes Spec Supremacy, Verification-First Architecture, Red Before Green, Adversarial Integrity, Silent Failures Are the Enemy, and Pragmatism Over Ceremony.

**SS (Architecture Subsystem)**
An architecture subsystem. Numbered as SS-NN (two digits). Lifecycle-scoped. Registry: ARCH-INDEX.md. Hook-validated by `validate-subsystem-names.sh`.

**STATE.md**
The single source of truth for pipeline progress. Lives at `.factory/STATE.md`. Tracks the current phase, mode, completed phases, active wave, and story statuses. Read at the start of every session and updated at every phase transition. Must stay under 200 lines — historical content belongs in cycle files. Enforced by `validate-state-size.sh` hook.

**STORY (STORY-NNN)**
An implementation story that delivers a specific increment of functionality. Each story traces to at least one BC. Numbered as STORY-NNN (three digits). Lifecycle-scoped. Registry: STORY-INDEX.md. Hook-validated by `validate-story-bc-sync.sh` and `validate-template-compliance.sh`.

**Strict Binary Novelty**
The convergence assessment protocol where each deepening round is classified as exactly SUBSTANTIVE (changes the model) or NITPICK (converged). No intermediate values. Prevents agents from hedging with "partially substantive" or "mostly converged."

**SUBSTANTIVE**
The strict-binary novelty classification for a convergence round where findings materially change the understanding. The counterpart of NITPICK. Any round that is not unambiguously NITPICK is SUBSTANTIVE.

**TD (Tech Debt)**
A technical debt register entry. Numbered as TD-NNN. Lifecycle-scoped. Registry: tech-debt-register.md.

**TDD (Test-Driven Development)**
The discipline of writing tests before implementation. In VSDD, this is enforced by the Red Gate: all tests must fail before any implementation code is written. The cycle is Red (failing test) then Green (minimum implementation) then Refactor.

**Trajectory Monotonicity**
The requirement that adversarial finding counts must decrease or stay flat across passes. An increase is a regression — the convergence loop stops and investigates the root cause before continuing. Enforced by `convergence-tracker.sh` hook.

**VP (Verification Property)**
A machine-verifiable property that must hold in the implementation. Numbered as VP-NNN. Part of the L4 specification level. Types include invariant, precondition, postcondition, safety, and liveness. Once a VP reaches `green` status, it is immutable — enforced by `protect-vp.sh`. Hook-validated by `validate-vp-consistency.sh`.

**VP-INDEX Source of Truth**
Governance policy that VP-INDEX.md is the authoritative enumeration of verification properties. Any change to VP-INDEX (additions, retirements, module reassignments, tool changes, phase changes) must propagate to `verification-architecture.md` and `verification-coverage-matrix.md` in the same burst. Enforced by policy `vp_index_is_vp_catalog_source_of_truth`.

**VSDD (Verified Spec-Driven Development)**
The unified methodology combining SDD, TDD, and VDD into an eight-phase pipeline (0-7). The spec is the product; code is disposable. Quality is measured by seven-dimensional convergence, not subjective assessment.

**Wave**
A group of stories scheduled for parallel implementation within a single delivery cycle. Stories within a wave have no dependencies on each other (only on stories from prior waves). After all stories in a wave are merged, a wave gate validates the wave before the next wave begins.

**Wave Gate**
The quality checkpoint after all stories in a wave are merged. Runs six checks: full test suite, DTU validation, adversarial review of the wave diff, demo evidence completeness, holdout evaluation, and state update. The wave advances only when all six gates pass. Iron Law: NO WAVE ADVANCE WITHOUT ALL SIX GATES PASSING FIRST.

**Wave Holdout Scenario (WHS-W[N]-NNN)**
A holdout scenario scoped to a specific wave within a convergence cycle. Unlike lifecycle-scoped HS-NNN scenarios, WHS IDs reset per cycle. Used for per-wave integration verification.

**Worktree**
A git worktree providing an isolated checkout for a specific purpose. The VSDD plugin uses two kinds: the permanent `.factory/` worktree (on `factory-artifacts`) for pipeline state, and temporary `.worktrees/STORY-NNN/` worktrees (on feature branches) for per-story implementation. Story worktrees are removed after merge.

**Zero-Findings Halt**
A convergence protocol rule: if the adversary reports zero findings on its first pass of non-trivial work, this is suspicious — not convergence. The adversary is re-dispatched with an explicit instruction to justify zero findings by citing concrete code evidence for each review category. Enforced by `convergence-tracker.sh` hook (warning).
