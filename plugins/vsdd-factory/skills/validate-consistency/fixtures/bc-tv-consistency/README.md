---
document_type: fixture-readme
level: ops
version: "1.0"
status: stable
producer: validate-consistency
phase: skill-fixtures
---

# BC Canonical TV ↔ Emitter Consistency Fixtures

These fixtures are reference inputs for **Check 9** (BC Canonical Test Vector
↔ Emitter field consistency). They are NOT compiled or registered in any
BC index — they exist purely to anchor the predicate's expected behavior
on canonical positive/negative shapes.

## Files

| File | Expected Verdict | Why |
|------|-----------------|-----|
| `flagged_bc_excludes_field.md` | (BC source) | BC's Canonical Test Vectors table marks `trace_id in Entry? = No` for the retry-exhaustion case |
| `flagged_emitter_serializes_field.rs` | FLAG | Production struct declares `pub trace_id: String,` with no `#[serde(skip_serializing_if)]` — emits the field unconditionally, contradicting the BC |
| `clean_bc_excludes_field.md` | (BC source) | Same exclusion declared as in the flagged BC fixture |
| `clean_emitter_uses_option_skip.rs` | CLEAN | Production struct declares `pub trace_id: Option<String>` with `#[serde(skip_serializing_if = "Option::is_none")]` — emission honors the BC |
| `clean_bc_with_skip_directive.md` | (BC source) | Same shape but adds `tv_emitter_check: skip` to BC frontmatter — opts out of the check entirely |

## How the predicate decides

For each BC file with a `## Canonical Test Vectors` section:

1. Parse table column headers for shapes like `<Field> in <Shape>?`, `Field <Field> in <Shape>?`, `Includes <Field>?`, `Emits <Field>?`, `Has <Field>?`.
2. For each row whose answer is `No` / `❌` / `false` / `excluded` / `omitted` / `n/a` / `--`, identify the struct named in the row's first cell.
3. Locate the matching `struct <Name> { ... }` in the production crate (named in BC frontmatter `target_module:` or grepped from the body).
4. Inspect the field declaration:
   - **CLEAN** if the field is missing, OR `Option<T>` with `#[serde(skip_serializing_if = "Option::is_none")]`, OR has `#[serde(skip_serializing)]`, OR is gated by feature flag, OR has any `#[serde(skip_serializing_if = ...)]` predicate.
   - **FLAG** if the field is a non-Option type with no skip attribute, OR `Option<T>` without `skip_serializing_if`, OR `pub <field>: T` with no decoration.
5. Skip BCs whose frontmatter contains `tv_emitter_check: skip` or `tv_emitter_check: manual`.

## Adding new fixtures

Follow the `flagged_` / `clean_` naming convention. Pair each new BC fixture
(`.md`) with its corresponding emitter fixture (`.rs`) so the predicate can
be exercised end-to-end against the pair.
