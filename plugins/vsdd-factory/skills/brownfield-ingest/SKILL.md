---
name: brownfield-ingest
description: Analyze an existing codebase using the broad-then-converge analysis protocol. 6 broad passes, then iterative deepening on every pass until novelty decays to LOW. Produces a complete semantic understanding that feeds into spec crystallization.
argument-hint: "[codebase-path] [--resume]"
---

# Brownfield Ingest

Deep analysis of an existing codebase to extract its behavioral intent, architecture, domain model, and conventions. This is the foundation for rebuilding or extending a system using the VSDD pipeline.

## Templates

Read and follow the output format in:
- `.claude/templates/recovered-architecture-template.md` — recovered architecture format

## Input

- `$ARGUMENTS[0]` — path to codebase OR a Git URL (e.g., `../dark-factory`, `https://github.com/org/repo`)
- `$ARGUMENTS[1]` (optional) — `--resume` to continue from last checkpoint

## When to Use

- Before `/create-brief` — to understand what exists before defining what to build
- Before `/semport-analyze` — to understand behavioral intent before translating
- When inheriting an unfamiliar codebase
- When rebuilding a system from scratch but wanting to preserve existing behavior

## How It Differs from Semport

| | Brownfield Ingest | Semport Analyze |
|---|---|---|
| **Goal** | Understand what exists | Translate to new language |
| **Output** | Knowledge docs, draft BCs, NFR catalog | Translation strategy, target design |
| **Uses** | Feeds create-brief, create-domain-spec, create-prd | Feeds deliver-story (gene-transfusion) |
| **Scope** | Whole codebase or module | Specific modules being ported |

## Step 0: Source Acquisition

Before any analysis, ensure the codebase is available in `.reference/<project>/`:

1. **If input is a Git URL:** Clone to `.reference/<project>/` with `--depth=1`:
   ```bash
   git clone --depth=1 <url> .reference/<project>
   ```
2. **If input is a local path outside `.reference/`:** Copy or move to `.reference/<project>/`.
3. **If input is already in `.reference/`:** No action needed.

After cloning/moving, **update `.factory/reference-manifest.yaml`** with the new entry (URL, commit SHA, date). This manifest is the source of truth for rebuilding `.reference/` on a new system.

All subsequent analysis reads from `.reference/<project>/`.

Create the output directory: `mkdir -p .factory/semport/<project>/`

## Sandbox Considerations

Brownfield ingest agents run inside the Corverax sandbox with a **Bash allowlist**, not full Bash. The settings.json allow rules give agents read-only analysis tools (`find`, `wc`, `cat`, `head`, `tail`, `ls`, `awk`, `xargs`, `sort`, `uniq`, `cut`, `tr`, `du`, `file`, `tree`, `basename`, `dirname`, `realpath`, `diff`, `jq`, `yq`, `tokei`, `cloc`, `scc`, `echo`, `printf`, `command -v`, `which`) plus `git -C <dir>` for git inspection inside `.reference/` and `.worktrees/`. Content search uses the **Grep tool** (not Bash `grep`/`rg`); file pattern listing uses the **Glob tool**.

The `codebase-analyzer` agent definition (`.claude/agents/codebase-analyzer.md`) has the full sandbox patterns and LOC counting recipes baked in. **If you write a new ingest-related agent or skill, document the same patterns there** so it does not bail on a single Bash denial.

The two working Bash patterns:
1. **Standalone with absolute paths:** `find /Users/jmagady/Dev/corverax/.reference/<repo> -name '*.go' -exec wc -l {} +`
2. **Chained with `cd` into a reference dir:** `cd /Users/jmagady/Dev/corverax/.reference/<repo> && find . -name '*.go' -exec wc -l {} +`

Both forms are pre-approved for any path under `.reference/` or `.worktrees/`. See `codebase-analyzer.md` for the complete reference.

## Analysis Protocol: Broad → Converge

The protocol has two phases: a **broad sweep** (Passes 0-6) followed by **convergence deepening** until all passes reach LOW novelty.

Launch the `codebase-analyzer` agent (via the Agent tool, subagent_type: `codebase-analyzer`) for each pass.

**One agent per project — always.** Never combine multiple projects into a single agent. Combined agents exhaust context before completing all projects, producing partial results that must be discarded and re-run. When processing multiple projects in parallel, launch separate agents (up to 10 concurrent to avoid rate limits).

### Phase A: Broad Sweep (Passes 0-6)

Run these sequentially. Each pass builds on prior pass outputs.

#### Pass 0: Inventory
- File tree, dependency graph, tech stack
- File prioritization scoring (entry points → configs → core → API → tests → utils)
- Output: `.factory/semport/<project>/<project>-pass-0-inventory.md`

#### Pass 1: Architecture
- Module boundaries, layers, component relationships
- Deployment topology, cross-cutting concerns
- Mermaid architecture and data flow diagrams
- Output: `.factory/semport/<project>/<project>-pass-1-architecture.md`

#### Pass 2: Domain Model
- Two-sub-pass approach:
  - 2a: Structural (entities, relationships, value objects, enums)
  - 2b: Behavioral (operations, business rules, state machines, events)
- Output: `.factory/semport/<project>/<project>-pass-2-domain-model.md`

#### Pass 3: Behavioral Contracts
- Extract from test files (first-class spec inputs), function signatures, validation logic
- Draft BCs with confidence levels (HIGH/MEDIUM/LOW)
- Output: `.factory/semport/<project>/<project>-pass-3-behavioral-contracts.md`

#### Pass 4: NFR Extraction
- Performance, security, observability, reliability, scalability patterns
- Configuration values encoding NFR decisions
- Output: `.factory/semport/<project>/<project>-pass-4-nfr-catalog.md`

#### Pass 5: Convention Catalog
- Naming, module organization, error handling, test patterns
- Design patterns with locations and consistency assessment
- Output: `.factory/semport/<project>/<project>-pass-5-conventions.md`

#### Pass 6: Synthesis
- Cross-reference all passes for inconsistencies
- Unified knowledge doc with confidence assessment
- Gap report identifying orphaned modules and under-documented subsystems
- Output: `.factory/semport/<project>/<project>-pass-6-synthesis.md`

**Commit after Phase A:** `factory(phase-0): brownfield ingest of <project>`

### Phase B: Convergence Deepening

After the broad sweep, iteratively deepen each pass until novelty decays to nitpicks. This is where the real understanding happens — broad passes are necessarily shallow on a large codebase.

#### Convergence Protocol

1. **Read Pass 6 synthesis** — identify all gaps: orphaned modules, under-documented subsystems, missing entity detail, subsystem-level BCs needing function-level depth.

2. **Run deepening rounds on Passes 2 and 3 first** (domain model and behavioral contracts are highest-value). Each round:
   - Reads all prior round outputs for that pass
   - Targets specific gaps from the synthesis and prior round's remaining-gaps list
   - Writes to `<project>-pass-N-deep-<name>-rM.md` (where M is the round number, omitted for round 1)
   - Includes a **Delta Summary** and **Novelty Assessment** (SUBSTANTIVE or NITPICK)

3. **After Passes 2 and 3 converge**, run deepening rounds on Passes 0, 1, 4, and 5. These benefit from the entity and BC knowledge gained during Pass 2/3 convergence — newly-discovered subsystems reveal missed inventory items, architectural implications, NFR patterns, and conventions not captured in the broad sweep.

4. **Each pass converges independently.** A pass is converged when its deepening round reports `Novelty: NITPICK — findings are refinements, not gaps.`

5. **Convergence bounds:**
   - Minimum 2 deepening rounds per pass before declaring NITPICK
   - **No fixed maximum.** Round budgets are advisory floors, not stop conditions. The protocol stops; the agent never does. If round N is SUBSTANTIVE, round N+1 launches regardless of any "max round" the agent claims. Empirical: Vault Pass 2 needed 62 rounds (R6/R10/R15/R30 each predicted "next is NITPICK" and were wrong — R7 found 55 entities, R31 found 16 from never-cited files, R36 found 69 new aggregates).
   - All passes (0-5) must converge — no pass is exempt
   - Passes 2 and 3 can run in parallel with each other per round
   - Passes 0, 1, 4, and 5 can run in parallel with each other per round
   - **One repo per agent, always.** Never combine repos — causes context exhaustion and partial failures.

6. **Targeted-flag carryover.** Each round's "next candidate scope" / "remaining gaps" must be passed **verbatim** into the next round's prompt. The orchestrator selects targets from the prior round's flags — the agent must not pick its own targets, which causes topic drift and re-coverage of already-explored areas.

7. **Cross-round contradiction handling.** When round N contradicts rounds 1..N-1 (e.g., Infisical R6 vs R1-R5 on AI/MCP gateway), the next round gets an explicit "resolve this contradiction" mandate, not a generic deepening prompt. Verify against source; the most recent round is not automatically right.

8. **Negative-finding catalogue.** Phantom subsystems must be retracted, not silently dropped. Use `CONV-ABS-N` or `BUG-RETRACTED-N` markers (per OpenBao LeaseCountQuota → CONV-ABS-7). Subsequent rounds reference the retraction so the same phantom doesn't reappear.

9. **Commit after each round:** `factory(phase-0): brownfield ingest deepening round N`

#### Round Prompt Template (required fields)

Every deepening round prompt MUST include:

1. **Pass identifier and round number** — explicit, e.g., "Pass 2 deepening round 17"
2. **Source path** — absolute, `/Users/.../.reference/<repo>/`
3. **Output path** — absolute, full filename, restated even if convention-implied (agents have written to wrong paths when path is implied)
4. **Inputs to read** — broad pass file + all prior round files
5. **Carryover targets** — verbatim "remaining gaps" / "next candidate scope" from prior round
6. **Strict-binary protocol restatement** — "binary novelty: SUBSTANTIVE or NITPICK; agent recommendations of 'converged' without literal NITPICK are ignored"
7. **Write-before-final mandate** — "Write the file before your final message" (some agents end without writing otherwise)
8. **Required output sections** — "End with Delta Summary, Novelty Assessment"

Missing any of these regularly produces wasted rounds.

#### Calibration

A run is calibrated when at least one small (<5K LOC, single-purpose) reference library converges in 2-8 rounds genuinely. If a small library needs 20+ rounds, the protocol is over-fitting and the prompts/targets need review. If a large multi-tenant system converges in <10 rounds, the protocol is under-fitting and strict-binary enforcement is failing somewhere. Use the small-library convergence as the calibration anchor.

#### Novelty Decay Assessment

Each deepening round MUST assess novelty decay — this is **strict binary**, not a gradient:

| Assessment | Meaning | Action |
|------------|---------|--------|
| **SUBSTANTIVE** | New entities, subsystems, contracts, relationships, or patterns discovered. Findings change the model. | Another round required |
| **NITPICK** | Findings are refinements, edge cases, wording improvements, or confirmations. Nothing changes the model. | Pass has converged |

**The test:** Would removing this round's findings change how you'd spec the system? If yes → SUBSTANTIVE. If no → NITPICK.

**Strict binary enforcement (CRITICAL).** Only the literal token `NITPICK` in the agent's Novelty Assessment counts as convergence. The orchestrator MUST ignore agent self-declarations like:
- "borderline NITPICK"
- "effectively converged"
- "convergence declared"
- "functionally complete"
- "another round may be needed but probably nitpick"
- "recommend halting"

These phrases mean SUBSTANTIVE for orchestrator purposes. The agent has no authority to declare convergence — only the protocol does. SUBSTANTIVE always triggers another round, no exceptions.

**Predicted-NITPICK is unreliable.** Agents are systematically bad at predicting whether the next round will converge. Never trust a prediction; always run the next round and let it speak for itself.

#### File Naming Convention

```
.factory/semport/<project>/
├── <project>-pass-N-<name>.md              # Broad sweep (Phase A)
├── <project>-pass-N-deep-<name>.md          # Deepening round 1
├── <project>-pass-N-deep-<name>-r2.md       # Deepening round 2
├── <project>-pass-N-deep-<name>-r3.md       # Deepening round 3
├── <project>-coverage-audit.md              # Phase B.5 coverage audit
└── <project>-pass-8-final-synthesis.md      # Phase C final synthesis
```

### Phase B.5: Coverage Audit (mandatory)

After all passes reach NITPICK, run a **deep audit** before final synthesis. **B.5 is not optional, even after exhaustive deepening.** Empirical evidence from the secrets-corpus run: every one of 5 repos showed genuine B.5 blind spots after 19-62 rounds of convergence. Round-driven deepening selects targets from prior-round flags, which means topic drift toward repeatedly-covered areas — entire directories can stay unwalked even when overall round count is high. B.5 is the only check that catches this.

**Method must be grep-driven, not agent-judgment-driven.** Inventory the source tree, grep deep files for references to each package/subsystem, flag any with zero or minimal hits as a blind spot. Coverage matrix as a table (package × pass → covered yes/partial/no). Don't ask the agent "are there gaps" — make it prove coverage with greps.

The 6-pass protocol can miss entire subsystems — directories that get noted as "stubs" or "low priority" in early passes but actually contain substantive implementation. The coverage audit cross-references the source directory tree against analysis artifacts to catch these gaps.

**IMPORTANT: One agent per project. Never combine multiple projects into a single agent invocation.** Combined agents run out of context before completing all projects, producing partial results that are unreliable and must be re-run. When batch-auditing multiple projects, launch separate agents (up to 10 in parallel to avoid rate limits).

Launch a general-purpose agent with this task:

1. **Read the Pass 6 synthesis and all convergence artifacts** to understand what was captured
2. **Scan the source directory tree** (`ls -R` on `.reference/<project>/`) to enumerate all directories and files
3. **Cross-reference** the directory listing against the analysis artifacts — identify directories, modules, or file clusters with zero or surface-only coverage
4. **For each missed subsystem**, read the actual source files and produce:
   - Entity catalog with types
   - Behavioral contracts (BC-AUDIT-NNN format)
   - Integration points with already-documented subsystems
   - Architectural patterns relevant to Corverax
5. **Write** to `.factory/semport/<project>/<project>-coverage-audit.md` (or `-rN.md` for subsequent rounds)
6. **Novelty assessment**: SUBSTANTIVE if gaps found, PASS if coverage is comprehensive

**The test:** Compare the set of source directories against the set of documented subsystems. Any directory with >100 lines of code and no corresponding entity or BC documentation is a gap.

#### Audit Loop

The coverage audit iterates until no substantive gaps remain:

1. Run the audit agent (grep-driven coverage matrix)
2. If gaps found → launch **surgical per-blind-spot mini-rounds** (one targeted file per blind spot) rather than re-running the full audit. Each mini-round file: `<project>-phase-b5-tr-N.md`. After all mini-rounds land, re-run the audit to verify they actually closed the gaps.
3. If **PASS** — no remaining directories with substantive uncovered code → audit is complete

**File naming:**
- Full audit re-runs: `<project>-coverage-audit.md`, `<project>-coverage-audit-r2.md`, ...
- Targeted blind-spot mini-rounds: `<project>-phase-b5-tr-N.md` (one per blind spot)

**Bounds:** No fixed maximum. Same protocol as deepening rounds — strict binary, no agent self-stop. In practice, 1-2 full audit cycles + a batch of targeted mini-rounds is typical.

**Commit after audit:** `factory(phase-0): brownfield ingest coverage audit + targeted fills`

### Phase B.6: Extraction Validation

After the coverage audit passes, launch the `validate-extraction` agent to verify the **accuracy** of what was extracted (coverage audit verified completeness; this verifies correctness).

The agent compares extracted artifacts against actual source code to catch:

1. **Hallucinated dependencies** — entities or relationships claimed in analysis but not present in source
2. **Phantom modules** — modules referenced in architecture docs that don't exist
3. **Inaccurate behavioral contracts** — BCs that describe behavior the code doesn't actually implement
4. **Domain model drift** — entities whose properties or relationships don't match the actual struct/class definitions
5. **Stale test references** — BCs citing test files or assertions that don't exist or test different behavior

#### Protocol

1. Launch a `validate-extraction` agent with access to both:
   - Source code: `.reference/<project>/`
   - Analysis artifacts: `.factory/semport/<project>/`
2. Agent reads the final synthesis + all BC files
3. Agent spot-checks a representative sample (~20-30%) of BCs against actual source
4. Agent verifies entity definitions match actual struct/class fields
5. Agent checks dependency graph edges against actual import statements
6. Reports findings as: CONFIRMED (accurate), INACCURATE (wrong), HALLUCINATED (doesn't exist)

#### Iteration

- If inaccuracies found → fix the analysis artifacts, re-validate
- Maximum 3 refinement iterations
- Write to `.factory/semport/<project>/<project>-extraction-validation.md` (or `-rN.md`)

#### Output

```
Extraction validation: <project>
  BCs sampled: N / M total (X%)
  Confirmed: N
  Inaccurate: N (fixed in iteration K)
  Hallucinated: N (removed)
  Entities verified: N / M
  Dependencies verified: N / M
  Result: PASS | FAIL (after N iterations)
```

**Commit:** `factory(phase-0): brownfield ingest extraction validation`

### Phase C: Final Synthesis

After ALL passes converge, coverage audit passes, AND extraction validation passes, run a final synthesis that incorporates everything:

- Reads all pass files (broad + all deepening rounds)
- Produces the definitive synthesis: complete feature set, bounded context map, complexity ranking, critical design decisions, anti-patterns, spec crystallization recommendations
- Includes a convergence report: rounds per pass, novelty trajectory, total coverage metrics
- Output: `.factory/semport/<project>/<project>-pass-8-deep-synthesis.md`

**Commit:** `factory(phase-0): brownfield ingest final synthesis — all passes converged`

### Phase D: Vision Disposition (deferred)

Brownfield ingest produces a **vision-agnostic** semantic understanding. Once the Corverax vision doc exists, every ingested repo must be re-examined through that lens to decide what to Model / Reimplement / Enhance / Leave Behind. This is **Pass 9** and runs via the `/disposition-pass` skill — not here.

**When to run Phase D:**
- After this brownfield ingest completes (Phase C done)
- After the vision doc exists (post `/create-brief` or major vision update)
- Before `/create-prd`, `/create-architecture`, or `/decompose-stories`

**How:** `/disposition-pass <repo>` for one repo, or `/disposition-pass --all --rollup` for the full corpus.

Phase D is **deferred** because it depends on a vision doc that doesn't exist during initial ingest. When the vision doc materially changes, dispositions become stale and must be re-run. The rollup tracks the vision-doc SHA for staleness detection.

## Resumability

Each pass persists a state checkpoint:
```yaml
pass: <N>
status: complete|partial|failed
files_scanned: <N>
timestamp: <ISO8601>
next_pass: <N+1>
resume_from: <file or module if partial>
```

Use `--resume` to continue from the last completed pass or deepening round.

## Post-Analysis Validation

See Phase B.6 above. Extraction validation is now a formal phase in the pipeline, not an ad-hoc step.

## Output Summary

```
Brownfield ingest complete: <project>
  Phase A: 7 broad passes complete
  Phase B: convergence deepening
    Pass 0 (Inventory):             N rounds → NITPICK
    Pass 1 (Architecture):          N rounds → NITPICK
    Pass 2 (Domain Model):          N rounds → NITPICK
    Pass 3 (Behavioral Contracts):  N rounds → NITPICK
    Pass 4 (NFRs):                  N rounds → NITPICK
    Pass 5 (Conventions):           N rounds → NITPICK
  Phase B.5: coverage audit — PASS (N missed subsystems found and documented)
  Phase B.6: extraction validation — PASS (N BCs sampled, N confirmed, N fixed)
  Phase C: final synthesis complete
  Total files produced: <N>
  Entities extracted: <N>
  Behavioral contracts: <N> (HIGH: <N>, MEDIUM: <N>, LOW: <N>)
  NFR configuration values: <N>
  Patterns cataloged: <N>
  Validation: PASS (N refinement iterations)

Artifacts in .factory/semport/<project>/<project>-pass-*.md
Use these as inputs for /create-brief, /create-domain-spec, and /create-prd.
```

## Source Reference

Source codebases live in `.reference/<project>/` for persistence across sessions. This directory is gitignored — sources are re-downloadable from URLs recorded in `.factory/reference-manifest.yaml`.

Step 0 handles cloning/moving to `.reference/` automatically. All codebase-analyzer agents read from `.reference/<project>/`.

To rebuild `.reference/` on a new machine, read `.factory/reference-manifest.yaml` and clone each entry.

## After Analysis

1. Commit to factory-artifacts:
   ```bash
   cd .factory && git add semport/ && git commit -m "factory(phase-0): brownfield ingest of <project> — converged"
   ```
2. Tell the user which downstream skills can now leverage the analysis.
