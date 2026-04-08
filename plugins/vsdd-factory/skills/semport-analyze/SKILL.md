---
name: semport-analyze
description: Semantic code porting — scan a reference codebase, extract behavioral intent, and design a translation strategy to the target language. Supports full ingestion and incremental change-level analysis. For porting existing implementations into Corverax or translating between languages.
argument-hint: "[source-path] [target-language] [--incremental module-name]"
---

# Semport Analyze

Semantic code porting analysis. Scans a reference codebase and produces translation artifacts.

## Templates

Read and follow the output format in:
- `.claude/templates/gene-transfusion-assessment-template.md` — assessment structure

## Input

- `$ARGUMENTS[0]` — path to source codebase, or project name if already in `.reference/` (e.g., `../dark-factory`, `onyx`, `adk-python`)
- `$ARGUMENTS[1]` — target language (e.g., `rust`, `typescript`)
- `$ARGUMENTS[2]` (optional) — `--incremental <module-name>` for change-level analysis

If the argument matches a directory name in `.reference/`, resolve to `.reference/<project>/`. This is the canonical location for ingested codebases — check `.factory/reference-manifest.yaml` for the full inventory.

## Analysis Modes

### Full Ingestion (default)

Run the complete 6-pass analysis protocol from the codebase-analyzer agent. Produces a full semantic model of the source codebase.

Use when:
- First time analyzing a codebase
- Major structural changes since last analysis
- Need complete translation strategy

```
/semport-analyze ../dark-factory rust
```

### Incremental Analysis (`--incremental`)

Analyze only specific modules or changed areas. Builds on existing analysis artifacts without re-scanning the entire codebase.

Use when:
- Source codebase has been updated since last analysis
- Only porting specific modules
- Change-level specs (Augment Code pattern): current behavior → target behavior → invariants → scope boundaries

```
/semport-analyze ../dark-factory rust --incremental workflow-engine
```

**Incremental protocol:**
1. Read existing `.factory/semport/<project>/<project>-pass-6-synthesis.md` for context
2. Scan only the specified module and its direct dependencies
3. Produce a delta analysis: what changed, what's new, what was removed
4. Update existing pass files (append, don't replace)
5. Write delta report to `.factory/semport/<project>/<project>-delta-<module>-<date>.md`

## Phase 1: Semantic Analysis (6-pass protocol)

Use the codebase-analyzer agent's 6-pass protocol:
1. Inventory → structure, dependencies, tech stack
2. Architecture → components, layers, data flow
3. Domain Model → entities, relationships, state machines
4. Behavioral Contracts → preconditions, postconditions, error handling
5. NFR Extraction → performance, security, observability patterns
6. Synthesis → unified knowledge doc with confidence assessment

All outputs write to `.factory/semport/`.

## Phase 2: Target Language Design

For each module identified in Phase 1:

1. Map source constructs to target language idioms
2. Design purity boundaries for target language
3. Identify target-specific enhancements (e.g., Rust ownership for memory safety)
4. Document translation strategy with confidence levels
5. Write to `.factory/semport/<project>/<module>-target-design.md`

### Language Idiom Mappings

| Python → Rust | TypeScript → Rust | Python → TypeScript |
|---|---|---|
| `@dataclass` → `struct` with derives | `class` → `struct` + impl | Dynamic typing → strict annotations |
| `Enum(str)` → `enum` with derives | `interface` → `trait` | `snake_case` → `camelCase` |
| `list[T]` → `Vec<T>` or `&[T]` | `Promise<T>` → `Future<T>` | `dict` → `Record<K,V>` |
| Generator/`yield` → `impl Iterator` | `async/await` → `async/.await` | List comprehension → `.filter().map()` |
| `try/except` → `Result<T,E>` | `try/catch` → `Result<T,E>` | `None` → `null`/`undefined` |
| Decorator → proc macro or wrapper | Optional chaining → `Option<T>` methods | `**kwargs` → options object |

## Validation

After analysis, spawn the `validate-extraction` agent to verify:
- Extracted BCs match actual code behavior
- Domain model aligns with test assertions
- No hallucinated dependencies or phantom modules
- Max 3 refinement iterations

## Output

For full ingestion:
1. `.factory/semport/<project>/<project>-pass-0-inventory.md` through `pass-8-final-synthesis.md`
2. `.factory/semport/<project>/<module>-target-design.md` per module
3. `.factory/semport/<project>/semport-assessment.md` — summary with module inventory, complexity ratings, recommended translation order, effort estimates, risk areas

For incremental:
1. `.factory/semport/<project>/<project>-delta-<module>-<date>.md` — change analysis
2. Updated target design files as needed

## After Analysis

Tell the user:
```
Semport analysis complete:
  Mode: full | incremental (<module>)
  Passes: 6/6 complete
  Modules analyzed: <N>
  Behavioral contracts extracted: <N> (HIGH: <N>, MEDIUM: <N>, LOW: <N>)
  Validation: <PASS|FAIL — N refinement iterations>

Review .factory/semport/ for translation strategies.
Stories with implementation_strategy: gene-transfusion will use these artifacts during Phase 3.
```
