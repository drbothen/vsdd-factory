# AC-3: Section — What's NOT stable (internal JSONL format, dispatcher invocation args)

**AC statement:** Section: What's NOT stable (internal JSONL format, dispatcher invocation
args).

**Evidence type:** section heading grep + verbatim table snippet

## Verification command

```
grep -n "NOT stable\|not stable" /private/tmp/vsdd-S-5.06/docs/guide/semver-commitment.md
```

**Result:**
```
74:## What's NOT stable in v1.0
```

## Verbatim snippet — unstable surface table (L74-L88)

```markdown
## What's NOT stable in v1.0

The following surfaces are internal implementation details and may change across
minor or patch releases without notice. Do not build integrations that depend on
them.

| Surface | Stability | Notes |
|---------|-----------|-------|
| Internal JSONL format (`dispatcher-internal-*.jsonl` field layout) | Unstable | Field names, nesting depth, and optional fields may change in any release. Use the public OTel / file-sink output for downstream integrations. |
| Dispatcher invocation args (CLI flags passed by Claude Code to the dispatcher binary) | Unstable | The dispatcher is invoked by Claude Code, not by operators directly. Flag names, order, and semantics are internal to the dispatcher–Claude Code contract and may change without notice. |
| `hooks-registry.toml` generator script (`scripts/generate-registry-from-hooks-json.sh`) | Unstable | Retired at 1.0.0 GA. Do not script against it. |
| Skill implementation internals (code inside `plugins/vsdd-factory/skills/`) | Unstable | The skill invocation interface (`/vsdd-factory:<name>`) is stable; what each skill does internally is not. |
| `StoreData` / wasmtime host context struct layout | Unstable | Crates internal to the dispatcher. Not part of the public ABI. |
| `dispatcher-internal-*.jsonl` rotation and naming pattern | Unstable | Daily rotation and the `YYYY-MM-DD` suffix are current behavior; path and naming may change. |
| Rust crate-internal module layout (non-`host::*` SDK modules) | Unstable | Only `vsdd_hook_sdk::host::*` is the stable plugin-author surface. |
| `hooks.json.template` intermediate format | Unstable | The `.template` file is a build artifact used by the activation skill. Operators receive the resolved `hooks.json`; the template format is internal. |
```

## Both required surfaces confirmed

| Required surface | Present? | File line |
|-----------------|---------|-----------|
| Internal JSONL format (`dispatcher-internal-*.jsonl` field layout) | Yes | L81 |
| Dispatcher invocation args (CLI flags) | Yes | L82 |

## Commentary

The section lists 8 unstable surfaces. Each row explains *why* it is unstable and
(where applicable) what the stable alternative is. The "Why these are unstable"
subsection following the table provides narrative context explaining that the internal
JSONL format and dispatcher invocation args are controlled by the dispatcher–Claude Code
interaction and may evolve without notice to plugin authors.
