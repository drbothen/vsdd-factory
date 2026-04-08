# Conventions: [PROJECT_NAME]

> Extracted by Dark Factory Phase 0c
> Source codebase: `[CODEBASE_PATH]`
> Date: [GENERATION_DATE]
> Files sampled: [N]

---

## Naming Conventions

### Functions and Methods

| Convention | Pattern | Frequency | Examples |
|-----------|---------|-----------|----------|
| [dominant pattern] | [snake_case / camelCase / etc.] | [N/M files] | `[file:line]`, `[file:line]`, `[file:line]` |
| [exception pattern] | [pattern] | [N/M files] | `[file:line]` |

### Types, Structs, and Classes

| Convention | Pattern | Frequency | Examples |
|-----------|---------|-----------|----------|
| [dominant pattern] | [PascalCase / etc.] | [N/M files] | `[file:line]`, `[file:line]`, `[file:line]` |

### Modules and Files

| Convention | Pattern | Frequency | Examples |
|-----------|---------|-----------|----------|
| [dominant pattern] | [snake_case / kebab-case / etc.] | [N/M files] | `[path]`, `[path]`, `[path]` |

### Constants

| Convention | Pattern | Frequency | Examples |
|-----------|---------|-----------|----------|
| [dominant pattern] | [SCREAMING_SNAKE / etc.] | [N/M files] | `[file:line]`, `[file:line]`, `[file:line]` |

### Test Functions

| Convention | Pattern | Frequency | Examples |
|-----------|---------|-----------|----------|
| [dominant pattern] | [test_descriptive_name / etc.] | [N/M test files] | `[file:line]`, `[file:line]`, `[file:line]` |

---

## Error Handling

### Dominant Pattern

- **Approach:** [structured error types / string errors / anyhow / thiserror / exceptions]
- **Propagation:** [? operator / .map_err() / try-catch / manual match]
- **Error type location:** [per-module error.rs / central errors.rs / inline]
- **Evidence:** `[file:line]`, `[file:line]`, `[file:line]`

### Error Type Structure

```
[Example error enum or class from the codebase, with file:line citation]
```

### Anti-Patterns Detected

| Anti-Pattern | Occurrences | Files | Severity |
|-------------|-------------|-------|----------|
| `unwrap()` in non-test code | [N] | `[file:line]`, ... | [high / medium / low] |
| Swallowed errors (empty catch) | [N] | `[file:line]`, ... | [high / medium / low] |
| String-only errors | [N] | `[file:line]`, ... | [high / medium / low] |

---

## Testing Conventions

### Framework and Runner

| Property | Value | Evidence |
|----------|-------|----------|
| Test framework | [cargo test / pytest / jest / etc.] | `[config file]` |
| Test runner | [default / nextest / etc.] | `[config file or script]` |
| Coverage tool | [llvm-cov / tarpaulin / coverage.py / c8 / none] | `[config file]` |

### Organization

| Property | Value | Evidence |
|----------|-------|----------|
| Test location | [co-located #[cfg(test)] / separate tests/ / both] | `[examples]` |
| Integration tests | [tests/ directory / separate project / none] | `[path]` |
| Test data/fixtures | [fixtures/ dir / inline / builder pattern / none] | `[path]` |

### Naming Convention

```
[Example test function signature with file:line citation]
```

**Pattern:** [test_module_behavior_condition / test_noun_verb / it("should...")]

### Assertion Style

- **Library:** [assert! / assert_eq! / expect() / chai / none]
- **Custom matchers:** [yes -- describe / no]
- **Snapshot tests:** [insta / jest snapshots / none]
- **Property tests:** [proptest / hypothesis / fast-check / none]

### Mock Strategy

- **Library:** [mockall / unittest.mock / jest.mock / none]
- **Pattern:** [trait-based mocks / monkey-patching / dependency injection / none]
- **Evidence:** `[file:line]`, `[file:line]`

---

## Code Organization

### Module Structure

- **Pattern:** [feature folders / layer folders / flat / mixed]
- **Barrel files:** [lib.rs re-exports / index.ts / __init__.py / none]
- **Description:** [how modules are organized and why]
- **Evidence:** [directory listing showing pattern]

### File Size Norms

| Metric | Value |
|--------|-------|
| Median file size | [N lines] |
| Largest file | `[path]` ([N lines]) |
| Smallest file | `[path]` ([N lines]) |
| Files over 500 lines | [N] |

### Import Ordering

- **Convention:** [std -> external -> internal / alphabetical / unordered]
- **Blank line separators:** [yes between groups / no]
- **Evidence:** `[file:line]`, `[file:line]`, `[file:line]`

---

## Documentation Style

### Doc Comments

| Property | Value | Evidence |
|----------|-------|----------|
| Format | [/// / /** / docstring / JSDoc / none] | `[file:line]` |
| Frequency | [all pub items / some / rare / none] | [N documented / M total pub items] |
| Content pattern | [one-liner / full with examples / mixed] | `[file:line]` |

### Inline Comments

| Property | Value |
|----------|-------|
| Frequency | [frequent (>1 per 10 lines) / moderate / sparse / none] |
| Style | [explain why / explain what / TODO markers / mixed] |

### README Files

| Property | Value |
|----------|-------|
| Top-level README | [yes / no] |
| Per-module READMEs | [yes / no] |
| Auto-generated docs | [rustdoc / typedoc / sphinx / none] |

---

## Summary for Agents

### MUST Follow (Dominant Conventions)

1. [Most important naming convention]
2. [Error handling pattern to use]
3. [Test naming and organization pattern]
4. [Import ordering convention]
5. [Documentation pattern]

### MUST AVOID (Detected Anti-Patterns)

1. [Anti-pattern 1 and why]
2. [Anti-pattern 2 and why]

### Style Ambiguities (Ask Human)

1. [Area where conventions are inconsistent -- human should clarify]

---

## Enforceable Rules (Machine-Readable)

Structured rules that the `brownfield-discipline` plugin can parse for automated
convention checking. Each rule has a pattern, scope, and severity.

```yaml
enforceable_rules:
  - id: CONV-001
    description: "[e.g., Functions must use snake_case naming]"
    pattern: "[regex or glob pattern to detect violations]"
    scope: "[file glob, e.g., 'src/**/*.rs']"
    severity: "[error | warning | info]"
    evidence: "[file:line citations from convention extraction]"

  - id: CONV-002
    description: "[e.g., Error types must use thiserror derive]"
    pattern: "[detection pattern]"
    scope: "[file glob]"
    severity: "[error | warning | info]"
    evidence: "[file:line citations]"

  # Add one rule per dominant convention from the sections above.
  # Only dominant patterns (not exceptions) become enforceable rules.
  # Anti-patterns from the Error Handling section become error-severity rules.
```

> **Plugin integration:** The `brownfield-discipline` plugin (Section 2 of this story)
> reads this YAML block from `.factory/phase-0-ingestion/conventions.md` for advisory
> pattern checking. Rules are advisory-only (warnings, never blocking), consistent
> with the plugin's non-blocking design.
