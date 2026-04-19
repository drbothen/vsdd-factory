---
name: formal-verify
description: Run formal hardening — Kani proofs for pure core functions, fuzz testing with cargo-fuzz, mutation testing with cargo-mutants, and security scanning with semgrep. Phase 5 quality gate.
disable-model-invocation: true
allowed-tools: Read, Write, Bash, Glob, Grep
---

# Formal Verification & Hardening

Phase 5 quality gate. Run multiple verification techniques against the codebase.

## Templates

Read and follow the output format in:
- `${CLAUDE_PLUGIN_ROOT}/templates/fuzz-report-template.md` — fuzz testing report
- `${CLAUDE_PLUGIN_ROOT}/templates/security-review-template.md` — security scan findings
- `${CLAUDE_PLUGIN_ROOT}/templates/security-scan-report-template.md` — security scan report
- `${CLAUDE_PLUGIN_ROOT}/templates/verification-gap-analysis-template.md` — verification coverage gaps

## Techniques

### 1. Kani Proofs (Pure Core Functions)

For functions in the pure core (no side effects):

```bash
cargo kani --harness <harness_name>
```

**What to prove:**
- Absence of panics for all valid inputs
- Arithmetic overflow safety
- Array bounds safety
- Invariant preservation across state transitions

**How to write harnesses:**
```rust
#[cfg(kani)]
mod verification {
    use super::*;

    #[kani::proof]
    fn verify_no_panic() {
        let input: u32 = kani::any();
        kani::assume(input < MAX_VALUE);
        let result = function_under_test(input);
        // assert properties
    }
}
```

Write proofs for all CRITICAL and HIGH criticality modules (from module-criticality.md).

### 2. Fuzz Testing (cargo-fuzz)

```bash
cargo fuzz run <target> -- -max_total_time=300
```

**Targets:**
- Parser inputs (any function that accepts `&str` or `&[u8]`)
- Deserialization functions
- State machine transitions
- API request handlers

**Create fuzz targets in `fuzz/fuzz_targets/`:**
```rust
#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(input) = std::str::from_utf8(data) {
        let _ = parse_function(input);
    }
});
```

Run each target for at least 5 minutes. Report any crashes.

### 3. Mutation Testing (cargo-mutants)

```bash
cargo mutants --timeout 60
```

**Goal:** Mutation kill rate ≥ 90%.

Mutations that survive indicate:
- Missing test assertions
- Dead code
- Redundant logic

Report surviving mutants with recommended test additions.

### 4. Security Scanning (semgrep)

```bash
semgrep --config auto --config p/rust-security src/
```

**Focus areas:**
- Command injection (CWE-78)
- Path traversal (CWE-22)
- Unsafe code usage
- Cryptographic misuse
- Hardcoded credentials

Report findings by severity.

## Templates

Use `${CLAUDE_PLUGIN_ROOT}/templates/formal-verification-template.md` for the formal verification report format.

## Output

Write to `.factory/cycles/<current>/formal-verification-report.md`:

```markdown
# Formal Verification Report

## Summary
| Technique | Status | Results |
|-----------|--------|---------|
| Kani proofs | ✅/❌ | <N> harnesses, all pass / <N> failures |
| Fuzz testing | ✅/❌ | <N> targets, <duration>, <crashes> |
| Mutation testing | ✅/❌ | Kill rate: <N>%, <N> survivors |
| Security scan | ✅/❌ | <N> findings (<N> critical) |

## Kani Results
<Per-harness results>

## Fuzz Results
<Per-target results, any crashes>

## Mutation Survivors
<List of surviving mutants with analysis>

## Security Findings
<Semgrep findings by severity>

## Gate: PASS | FAIL
<Criteria: all Kani pass, no fuzz crashes, mutation kill ≥90%, no critical security findings>
```

## Prerequisites

Install verification tools:
```bash
cargo install cargo-kani
cargo install cargo-fuzz
cargo install cargo-mutants
pip install semgrep  # or brew install semgrep
```

If a tool is not installed, report which tools are missing and skip that section. Never fail silently (SOUL.md #4).
