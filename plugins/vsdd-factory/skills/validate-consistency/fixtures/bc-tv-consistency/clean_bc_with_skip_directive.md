---
document_type: behavioral-contract
level: ops
bc_id: "BC-FIXTURE-SKIP.001"
version: "1.0"
status: fixture
producer: validate-consistency
target_module: crates/sink-core/src/entry.rs
tv_emitter_check: skip
phase: skill-fixtures
---

# BC-FIXTURE-SKIP.001 — Opt-out via tv_emitter_check directive

> **Fixture only.** Not registered in BC-INDEX. Demonstrates the
> `tv_emitter_check: skip` frontmatter directive that opts a BC out
> of Check 9 entirely.

## Why opt out?

Some BCs intentionally describe field-set scenarios that the emitter is
*allowed* to populate even when the canonical test vector marks them
absent — for example, when a downstream consumer tolerates extra fields.
In those cases the BC author records the intent in frontmatter so future
audit passes don't surface false-positive findings.

## Canonical Test Vectors

| Struct | Scenario | trace_id in Entry? | level in Entry? |
|--------|----------|-------------------:|----------------:|
| LogEntry | tolerates-extra-fields scenario | **No** | Yes |

Even though the row says `trace_id in Entry? = No`, the
`tv_emitter_check: skip` frontmatter directive tells Check 9 to skip
this BC entirely.

## Architecture Module

`crates/sink-core/src/entry.rs`
