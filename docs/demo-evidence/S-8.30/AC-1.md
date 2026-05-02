# AC-1: HookPayload struct gains 4 SubagentStop fields

**Story:** S-8.30  
**AC:** AC-1 — HookPayload struct gains 4 SubagentStop fields  
**BC clause:** BC-2.02.012 Postconditions 1-4  
**Source:** `crates/hook-sdk/src/payload.rs` lines 54-118

## Struct definition excerpt

The four new fields are declared after `plugin_config` (insertion point as
specified), inside the `// SubagentStop top-level fields` section:

```rust
// ── SubagentStop top-level fields (BC-2.02.012) ──────────────────────
// Present only on `event_name == "SubagentStop"` envelopes.  All four
// use `#[serde(default)]` so non-SubagentStop envelopes deserialize
// successfully with every field as `None` (BC-2.02.012 Invariant 2).
// JSON `null` also deserializes to `None` — providing jq-`//`-equivalent
// null-as-advance semantics (BC-2.02.012 Invariant 3).
// HOST_ABI_VERSION remains 1; this is an additive extension per D-6
// Option A and D-183 (BC-2.02.012 Invariant 1).

/// Agent type identifier carried by a SubagentStop envelope
/// (e.g. `"product-owner"`, `"pr-reviewer"`).
///
/// Primary arm of the canonical agent identity fallback chain
/// (BC-2.02.012 Postcondition 1 and 5).
#[serde(default)]
pub agent_type: Option<String>,

/// Subagent name carried by a SubagentStop envelope
/// (e.g. `"pr-reviewer-fallback"`).
///
/// Fallback arm of the canonical agent identity chain when
/// `agent_type` is `None` (BC-2.02.012 Postcondition 2 and 5).
#[serde(default)]
pub subagent_name: Option<String>,

/// Last assistant message text carried by a SubagentStop envelope.
///
/// Primary arm of the canonical assistant-message fallback chain
/// (BC-2.02.012 Postcondition 3 and 6).
#[serde(default)]
pub last_assistant_message: Option<String>,

/// Result field carried by a SubagentStop envelope.
///
/// Fallback arm of the canonical assistant-message chain when
/// `last_assistant_message` is `None` (BC-2.02.012 Postcondition 4 and 6).
#[serde(default)]
pub result: Option<String>,
```

## Verification

All four fields:
- Declared `pub`
- Annotated `#[serde(default)]`
- Type `Option<String>`
- Field names match BC-2.02.012 Invariant 4 exactly
- Placed after `plugin_config` field per story spec insertion point

**PASS**
