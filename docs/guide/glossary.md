# Glossary

Terminology used throughout the VSDD factory plugin, alphabetized.

---

**BC (Behavioral Contract)**
A formal, testable specification of what a module or function must do. Expressed as
preconditions, postconditions, and invariants. Numbered as BC-S.SS.NNN (subsystem.section.contract).
Part of the L3 specification level. Once a BC reaches `green` status, it is immutable.

**Brownfield Ingest**
Phase 0 of the VSDD pipeline. Analyzes an existing codebase through a broad-then-converge
protocol (7 broad passes, then iterative deepening until novelty decays). Produces a
semantic understanding that feeds into spec crystallization.

**Convergence**
The state where further review produces no meaningful new findings. Measured quantitatively
across five dimensions (spec, tests, implementation, verification, holdout). The pipeline
is complete when all five dimensions independently report CONVERGED.

**DTU (Digital Twin Universe)**
A clone of a reference implementation used for behavioral equivalence testing. DTU clones
are compared against the new implementation to verify that ported behavior is preserved.

**Factory Artifacts**
The orphan git branch (`factory-artifacts`) that holds all pipeline state, specs, stories,
and evaluation artifacts. Mounted as a worktree at `.factory/` in the target project.
Separate from `main` and `develop`.

**Fresh Context**
A deliberate constraint on adversarial review: the adversary agent starts each review pass
with no memory of prior passes. This prevents relationship drift and ensures each pass
provides genuinely independent evaluation.

**Gene Transfusion**
An implementation strategy where a story is built by porting behavioral intent from an
existing reference implementation (analyzed via brownfield ingest or semport) rather than
writing from scratch. The implementation uses the reference as a guide but adapts to the
target language and architecture.

**Holdout Scenario**
A hidden acceptance test created during story decomposition that the builder and test-writer
agents never see. Evaluated by an independent holdout-evaluator agent (different model,
fresh context) against the running system. Enforces train/test separation.

**Information Asymmetry**
A deliberate design principle where different agents have access to different information.
The adversary cannot see prior reviews. The holdout evaluator cannot see specs, source code,
or implementation notes. This prevents gaming and ensures independent evaluation.

**Iron Law**
A non-negotiable behavioral constraint on a discipline skill, expressed in the form
"NO [verb] [scope] WITHOUT [prerequisite] FIRST." Each of the four discipline skills
(`deliver-story`, `brownfield-ingest`, `adversarial-review`, `wave-gate`) has one Iron Law.

**NITPICK**
The strict-binary novelty classification for a convergence round where findings are cosmetic
or trivial. The counterpart of SUBSTANTIVE. Only the literal token NITPICK counts --
"effectively converged" or "borderline NITPICK" is treated as SUBSTANTIVE.

**Phase Gate**
A quality checkpoint between pipeline phases. Each gate has defined pass/fail criteria.
No phase may begin until the previous phase's gate passes. The orchestrator agent is the
only entity that transitions between phases.

**Red Flags Table**
A table embedded in discipline skills that enumerates the rationalizations agents use to
bypass Iron Laws. Each row maps a tempting thought ("I'll skip the Red Gate for this small
change") to the reality of why it is a protocol violation.

**Red Gate**
The TDD checkpoint in Phase 3 where all tests must fail before any implementation begins.
If a test passes without implementation, the test is suspect. Enforced by the `red-gate.sh`
hook when strict mode is active.

**Semport (Semantic Port)**
The process of extracting behavioral intent from a reference implementation and designing
a translation strategy to a target language. Used when porting existing systems. Distinct
from brownfield ingest, which understands what exists; semport translates it.

**SOUL (Agent Principles)**
The set of governing principles that every agent in the factory follows. Defined in
AGENT-SOUL.md. Includes Spec Supremacy, Verification-First Architecture, Red Before Green,
Adversarial Integrity, Silent Failures Are the Enemy, and Pragmatism Over Ceremony.

**STATE.md**
The single source of truth for pipeline progress. Lives at `.factory/STATE.md`. Tracks the
current phase, mode, completed phases, active wave, and story statuses. Read at the start
of every session and updated at every phase transition.

**Strict Binary Novelty**
The convergence assessment protocol where each deepening round is classified as exactly
SUBSTANTIVE (changes the model) or NITPICK (converged). No intermediate values. Prevents
agents from hedging with "partially substantive" or "mostly converged."

**SUBSTANTIVE**
The strict-binary novelty classification for a convergence round where findings materially
change the understanding. The counterpart of NITPICK. Any round that is not unambiguously
NITPICK is SUBSTANTIVE.

**TDD (Test-Driven Development)**
The discipline of writing tests before implementation. In VSDD, this is enforced by the
Red Gate: all tests must fail before any implementation code is written. The cycle is
Red (failing test) then Green (minimum implementation) then Refactor.

**VP (Verification Property)**
A machine-verifiable property that must hold in the implementation. Numbered as VP-NNN.
Part of the L4 specification level. Types include invariant, precondition, postcondition,
safety, and liveness. Once a VP reaches `green` status, it is immutable -- enforced by
the `protect-vp.sh` hook.

**VSDD (Verified Spec-Driven Development)**
The unified methodology combining SDD, TDD, and VDD into a six-phase pipeline. The spec
is the product; code is disposable. Quality is measured by five-dimensional convergence,
not subjective assessment.

**Wave**
A group of stories scheduled for parallel implementation within a single delivery cycle.
Stories within a wave have no dependencies on each other (only on stories from prior waves).
After all stories in a wave are merged, a wave gate validates the wave before the next
wave begins.

**Wave Gate**
The quality checkpoint after all stories in a wave are merged. Runs six checks: full test
suite, DTU validation, adversarial review of the wave diff, demo evidence completeness,
holdout evaluation, and state update. The wave advances only when all six gates pass.

**Worktree**
A git worktree providing an isolated checkout for a specific purpose. The VSDD plugin uses
two kinds: the permanent `.factory/` worktree (on `factory-artifacts`) for pipeline state,
and temporary `.worktrees/STORY-NNN/` worktrees (on feature branches) for per-story
implementation. Story worktrees are removed after merge.
