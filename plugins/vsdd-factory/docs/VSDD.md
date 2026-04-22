# Verified Spec-Driven Development (VSDD)

## The Fusion: VDD × TDD × SDD for AI-Native Engineering

VSDD is a unified software engineering methodology that fuses three proven paradigms
into a single AI-orchestrated pipeline:

- **SDD:** Define the contract before writing a single line of implementation
- **TDD:** Tests are written before code. Red → Green → Refactor.
- **VDD:** Subject all surviving code to adversarial refinement until the reviewer
  is forced to hallucinate flaws

### The VSDD Toolchain

| Role | Entity | Model | Function |
|------|--------|-------|----------|
| **The Architect** | Human Developer | N/A | Strategic vision, domain expertise, acceptance authority |
| **The Builder** | Claude agents | Opus/Sonnet | Spec authorship, test generation, code implementation |
| **The Adversary** | GPT-5.4 | Different family | Hyper-critical reviewer. Fresh context every pass. |
| **The Verifier** | Formal tools | Sonnet + local | Kani, cargo-fuzz, cargo-mutants, Semgrep |

---

## Phase 1 — Spec Crystallization

*Nothing gets built until the contract is airtight and the architecture is verification-ready.*

### Step 1a-i: Domain Specification (L2)

Before the behavioral spec is finalized, the business-analyst produces the L2 Domain Spec:
- Domain-level capabilities (CAP-NNN) independent of system architecture
- Domain entities (independent of database schema)
- Domain invariants (DI-NNN) -- business rules, not implementation rules
- Domain-level edge cases (DEC-NNN)
- Assumptions requiring validation (ASM-NNN)
- Risk register (R-NNN)
- Failure modes (FM-NNN)

This level guides architecture but architects, not implementers, read it.

### Step 1a: Behavioral Specification (L3)

The Builder produces the functional contract as L3 Behavioral Contracts (BC-S.SS.NNN):

- **Behavioral Contract:** What each module/function/endpoint MUST do, expressed as
  preconditions, postconditions, and invariants
- **Interface Definition:** Input types, output types, error types. No ambiguity.
  If it's an API → OpenAPI schema. If it's a module → type signature and doc contract.
- **Edge Case Catalog:** Explicitly enumerated boundary conditions, degenerate inputs,
  and failure modes. The Builder is prompted to be exhaustive: "What happens when the
  input is null? Empty? Maximum size? Negative? Unicode? Concurrent?"
- **Non-Functional Requirements:** Performance bounds, memory constraints, security
  considerations baked into the spec itself

### Step 1b: Verification Architecture (L4 Precursor)

Phase 1b creates the Provable Properties Catalog, which becomes the L4
Verification Properties Registry. The architect produces per-file VP-NNN
documents with proof method, feasibility assessment, and harness skeletons.

Before any implementation design is finalized, the Builder produces a Verification
Strategy answering: "What properties must be mathematically provable, and what
architectural constraints does that impose?"

- **Provable Properties Catalog:** Which invariants, safety properties, and correctness
  guarantees must be formally verified — not just tested?
  - "This state machine can never reach an invalid state"
  - "This arithmetic can never overflow"
  - "This parser always terminates"
  - "This access control check is never bypassed"
- **Purity Boundary Map:** Clear architectural separation between the deterministic,
  side-effect-free core (where formal verification operates) and the effectful shell
  (I/O, network, database). This is the most consequential design decision — it dictates
  module boundaries, dependency direction, and state flow.
- **Verification Tooling Selection:** Based on language and properties, select the
  verification stack:
  - Rust → Kani, cargo-fuzz, cargo-mutants, proptest
  - TypeScript → fast-check, Stryker, ESLint security rules
  - Python → Hypothesis, mutmut, Semgrep
  - Distributed systems → TLA+
- **Property Specifications:** Draft the actual formal property definitions alongside
  the behavioral spec (Kani proof harnesses, proptest strategies, TLA+ invariants)

### Step 1c: UX Specification

For products with user interfaces:

- Screen definitions with element IDs, types, states, and validation rules
- Interaction flows with success/error paths
- Responsive breakpoints and layout adjustments
- Accessibility requirements (WCAG 2.1 AA minimum)
- Design system component mappings

### Step 1d: Adversarial Spec Review

The complete spec — behavioral contracts AND verification architecture — is reviewed
by both the human and the Adversary. The Adversary tears into the spec looking for:

- Ambiguous language interpretable multiple ways
- Missing edge cases
- Implicit assumptions not stated
- Contradictions between spec sections
- Properties claimed as "testable only" that should be provable
- Purity boundary violations — logic marked "pure" that depends on external state
- Verification tool mismatches — properties the selected tooling can't actually prove

The spec iterates until the Adversary can't find legitimate holes.

**Quality Gate:** Human approves final spec before Phase 2 begins.

---

## Phase 2 — Story Decomposition

With an airtight spec, break it into implementable units:

- Each story maps to one or more L3 Behavioral Contracts (BC-S.SS.NNN)
- Each Acceptance Criterion (AC-NNN) traces to a BC precondition/postcondition
- Verification Properties (VP-NNN) are NOT embedded in stories -- they reference
  the L4 registry by VP-NNN ID only

- **Epics** group related capabilities (3-7 per PRD)
- **Stories** are single-feature, single-team implementation units
- Each story includes:
  - Acceptance criteria as numbered behavioral assertions (e.g., "AC-001: API returns
    paginated results with next_page token when more than 20 results exist"). BDD
    Given/When/Then format is acceptable but not required — behavioral assertions are
    clearer for AI agents and easier to validate programmatically.
  - Verification properties (which formal proofs apply -- referenced by VP-NNN ID)
  - Architecture component mappings
  - UX screen references (if applicable)
  - Dependency graph (must be acyclic)
  - Input-hash for traceability

**Quality Gate:** Human approves story breakdown. Adversary reviews for gaps.

---

## Phase 3 — Test-First Implementation (The TDD Core)

*Red → Green → Refactor, enforced by AI.*

### Step 3a: Test Suite Generation

The Builder translates the spec directly into executable tests:

- **Unit Tests:** One+ per behavioral contract item. Every postcondition → assertion.
  Every precondition violation → expected error test.
- **Edge Case Tests:** Every Edge Case Catalog item → test
- **Integration Tests:** Module interactions within system context
- **Property-Based Tests:** Invariants asserted across randomized inputs (proptest,
  Hypothesis, fast-check)

### Step 3b: The Red Gate

**ALL tests must FAIL before any implementation begins.** If a test passes without
implementation, the test is suspect — it's either testing the wrong thing or the spec
was wrong. The Builder flags this for human review.

The `red-gate.sh` hook enforces this automatically.

### Step 3c: Minimal Implementation

The Builder writes the MINIMUM code to make each test pass, one at a time:

1. Pick the next failing test
2. Write the smallest implementation that makes it pass
3. Run the full suite — nothing else should break
4. Repeat

### Step 3d: Refactor

After all tests are green, refactor for clarity, performance, and adherence to NFRs.
The test suite is the safety net.

**Quality Gate:** All tests pass. Human reviews for spec "spirit" alignment.

---

## Phase 5 — Adversarial Refinement (The VDD Roast)

*The code survived testing. Now it faces the gauntlet.*

The verified, test-passing codebase — spec, tests, AND implementation — is presented
to the Adversary in a **fresh context window**.

### What the Adversary Reviews

1. **Spec Fidelity:** Does the implementation actually satisfy the spec, or did the
   tests encode a misunderstanding?
2. **Test Quality:** Are tests actually testing what they claim? Tautological tests?
   Tests that mock too aggressively? Tests asserting on implementation details?
3. **Code Quality:** Placeholder comments, generic error handling, inefficient patterns,
   hidden coupling, missing resource cleanup, race conditions
4. **Security Surface:** Input validation gaps, injection vectors, auth assumptions
5. **Spec Gaps:** Implemented behavior not covered by the spec

### Adversary Protocol

- **Negative prompting:** Zero tolerance. No "overall looks good" preamble.
- **Context reset:** Fresh context window on every pass. No relationship drift.
- **Concrete findings:** Every flaw has a specific location and proposed fix.
- **Different model family:** GPT-5.4 or DeepSeek-V3.2. NEVER Claude.

### Feedback Integration Loop

- Spec-level flaws → Return to Phase 1. Update spec, re-review.
- Test-level flaws → Return to Phase 3a. Fix/add tests, verify they fail.
- Implementation flaws → Return to Phase 3d. Refactor, tests must still pass.
- New edge cases → Add to Edge Case Catalog, write failing tests, implement.

This loop continues until convergence (Phase 7).

---

## Phase 6 — Formal Hardening

The verification architecture designed in Phase 1b is now executed against the
battle-tested implementation.

- **Proof Execution:** Kani harnesses, proptest strategies, TLA+ invariants run
  against the implementation. Because architecture was designed for verifiability,
  proofs engage cleanly with the pure core.
- **Fuzz Testing:** Structured fuzzing (cargo-fuzz, AFL++, libFuzzer) layered on
  property-based tests. The deterministic core is an ideal fuzz target.
- **Security Hardening:** Semgrep rules, dependency scanning, SAST/DAST gates.
- **Mutation Testing:** cargo-mutants / Stryker mutate the code to verify the test
  suite catches real bugs. Surviving mutants → test suite gap.
- **Purity Boundary Audit:** Final check that purity boundaries from Phase 1b are
  respected. Side effects that crept into the pure core → flagged and refactored.

All results feed back into Phase 5 if issues are found.

---

## Phase 7 — Convergence (The Exit Signal)

VSDD's hallucination-based termination, extended across all seven dimensions:

| Dimension | Convergence Signal |
|-----------|-------------------|
| **Spec** | Adversary critiques are nitpicks about wording, not missing behavior or verification gaps |
| **Tests** | Adversary can't identify a meaningful untested scenario. Mutation testing confirms high kill rate. |
| **Implementation** | Adversary is forced to invent problems that don't exist in the code |
| **Verification** | All properties pass formal proof. Fuzzers find nothing. Purity boundaries intact. |
| **Holdout** | Mean satisfaction score >= 0.85, all must-pass scenarios >= 0.6, std dev < 0.15 |

**Maximum Viable Refinement** is reached when all five dimensions converge.
The software is **Zero-Slop** — every line traces to a spec requirement, is covered
by a test, has survived adversarial scrutiny, and the critical path is formally proven.

---

## The VSDD Contract Chain

Full traceability across the 4-level specification hierarchy:

```
L1 Product Brief
  → L2 Domain Spec Capability (CAP-NNN)
    → L3 Behavioral Contract (BC-S.SS.NNN)
      → Story Acceptance Criterion (AC-NNN)
        → Test Case (test_BC_S_SS_NNN_xxx)
          → Implementation (src/)
            → L4 Verification Property (VP-NNN)
              → Formal Proof (Kani/proptest/TLA+)
                → Adversarial Review (constraints proven)
```

"Why does this line of code exist?" → trace to L3 Behavioral Contract → L2 Capability → L1 Brief.
"Why is this module a pure function?" → trace to Purity Boundary Map → L4 VP proof feasibility.
"Why does this property need formal proof?" → trace to VP-NNN → BC-S.SS.NNN → CAP-NNN.

---

## AI Orchestration

- **Builder agents** benefit from large context windows and strong code gen (Claude Opus/Sonnet)
- **Adversary** benefits from a DIFFERENT model family (GPT-5.4, DeepSeek-V3.2)
- **Human** is the strategic layer — approves specs, resolves disputes, makes judgment calls
- **TDD Prompt Discipline:** Builder is explicitly instructed: "You are operating under
  strict TDD. Write tests FIRST. Do NOT write implementation code until all tests fail.
  When implementing, write the MINIMUM code to pass each test."

---

## Tooling

- **Project context:** `/vsdd-factory:scaffold-claude-md` — auto-detects and generates a project-specific `CLAUDE.md` with build, test, git, and reference information.
- **Visual companion:** `/vsdd-factory:visual-companion` — browser-based mockups and interactive options during early pipeline stages. Optional, requires Node.js.
- **Systematic debugging:** `/vsdd-factory:systematic-debugging` — 4-phase root cause investigation for any bug, test failure, or unexpected behavior. Enforces investigation before fixes.
- **Writing skills:** `/vsdd-factory:writing-skills` — TDD methodology for creating and maintaining plugin skills. RED-GREEN-REFACTOR applied to process documentation.
