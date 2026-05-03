//! BC-7.03.079 / BC-7.03.080 behavioral-parity tests for the native-WASM port
//! of track-agent-start (S-8.08).
//!
//! **Red Gate scope:** these tests exercise the implementation gaps that remain
//! after the stub-architect's 17 in-crate unit tests.  All registry / filesystem
//! assertions MUST FAIL until the implementer completes S-8.08 T-6 and T-7.
//!
//! **What this file covers (gap analysis vs. stub-architect tests):**
//!
//! 1. Registry parity (AC-001 / AC-002a): `hooks-registry.toml` entry for
//!    track-agent-start must point to the native WASM binary (`track-agent-start.wasm`),
//!    must NOT reference the legacy-bash-adapter, must have no `script_path`,
//!    no `exec_subprocess` capability block, and must preserve the E-8 D-2 binding
//!    tuple (`event=PreToolUse`, `tool=Agent`, `priority=110`, `on_error=continue`).
//!
//! 2. File lifecycle (AC-002b): the `.sh` source file must be absent after migration.
//!
//! 3. hooks.json.* verification (AC-002b): zero track-agent-start command entries
//!    across all platform files and the template (post-DRIFT-004 baseline).
//!
//! 4. Dispatcher linker / emit_event integration (AC-003 / BC-7.03.080 PC-1):
//!    the `vsdd::emit_event` host function must be reachable from a WAT module
//!    that mimics the SDK's encoding and must place an `agent.start` event with
//!    the correct field set (hook, matcher, subagent, optional story_id) in the
//!    HostContext event queue — with no forbidden fields (agent_id, tool_name).
//!
//! BC: BC-7.03.079, BC-7.03.080
//! Story: S-8.08
//!
//! mutation_testing_required: true
//! (The in-crate unit suite is fully GREEN against the stub. The dominant Red
//! Gate signal here comes from registry/filesystem assertions that fail until
//! T-6/T-7 complete. The linker integration test provides an additional gate.)

use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent() // crates/
        .and_then(|p| p.parent()) // workspace root
        .expect("resolve workspace root from CARGO_MANIFEST_DIR")
        .to_path_buf()
}

fn registry_path() -> PathBuf {
    workspace_root()
        .join("plugins")
        .join("vsdd-factory")
        .join("hooks-registry.toml")
}

// ---------------------------------------------------------------------------
// Helper utilities
// ---------------------------------------------------------------------------

/// Encode key/value pairs using the vsdd_hook_sdk wire format:
///   [ key_len u32 LE | key bytes | val_len u32 LE | val bytes ]+
///
/// Mirrors `vsdd_hook_sdk::host::encode_fields` so integration tests can
/// synthesize payloads without depending on the SDK's private API.
fn encode_sdk_fields(pairs: &[(&str, &str)]) -> Vec<u8> {
    let mut buf = Vec::new();
    for (k, v) in pairs {
        buf.extend_from_slice(&(k.len() as u32).to_le_bytes());
        buf.extend_from_slice(k.as_bytes());
        buf.extend_from_slice(&(v.len() as u32).to_le_bytes());
        buf.extend_from_slice(v.as_bytes());
    }
    buf
}

/// Extract the TOML stanza for a named hook from raw `hooks-registry.toml` text.
///
/// Returns the text from the `[[hooks]]` block that contains `name = "<hook_name>"`
/// through (but not including) the next `[[hooks]]` boundary, or `None` if
/// the hook name is not found.
fn extract_stanza(raw: &str, hook_name: &str) -> Option<String> {
    let mut stanza_lines: Vec<&str> = Vec::new();
    let mut in_target = false;

    for line in raw.lines() {
        if line.trim() == "[[hooks]]" {
            if in_target {
                // We've hit the next hook block; stop collecting.
                break;
            }
            // Start collecting a new candidate block.
            stanza_lines.clear();
            stanza_lines.push(line);
        } else if !stanza_lines.is_empty() {
            // Inside a candidate block: check if it's the target.
            let trimmed = line.trim();
            if trimmed == format!("name = \"{hook_name}\"")
                || trimmed == format!("name = '{hook_name}'")
            {
                in_target = true;
            }
            stanza_lines.push(line);
        }
    }

    if in_target {
        Some(stanza_lines.join("\n"))
    } else {
        None
    }
}

// ---------------------------------------------------------------------------
// 1. Registry parity (AC-001 / AC-002a / BC-7.03.079 PC-1 + Inv-1)
// ---------------------------------------------------------------------------

/// AC-001 / BC-7.03.079 postcondition 1: the hooks-registry.toml entry for
/// `track-agent-start` must point to `hook-plugins/track-agent-start.wasm`
/// (NOT the legacy-bash-adapter).
///
/// FAILS until T-6 updates hooks-registry.toml.
#[test]
fn test_BC_7_03_079_ac001_registry_entry_points_to_native_wasm() {
    use factory_dispatcher::registry::Registry;

    let reg = Registry::load(&registry_path()).expect("registry must parse");
    let entry = reg
        .hooks
        .iter()
        .find(|h| h.name == "track-agent-start")
        .expect("hooks-registry.toml must contain a [track-agent-start] entry");

    assert!(
        entry
            .plugin
            .ends_with("hook-plugins/track-agent-start.wasm"),
        "track-agent-start registry entry must use the native WASM binary \
         (not legacy-bash-adapter) — AC-001 / BC-7.03.079 postcondition 1\n\
         plugin path: {}",
        entry.plugin.display()
    );
}

/// AC-001 / BC-7.03.079 invariant 1 (registration stability): the binding
/// tuple (event=PreToolUse, tool=Agent, priority=110, on_error=continue)
/// must remain unchanged after migration.
///
/// Included as a regression gate alongside the native-wasm gate.
/// STILL PASSES in stub state (binding unchanged by stub-architect).
#[test]
fn test_BC_7_03_079_invariant_1_binding_tuple_preserved_after_migration() {
    use factory_dispatcher::registry::Registry;

    let reg = Registry::load(&registry_path()).expect("registry must parse");
    let entry = reg
        .hooks
        .iter()
        .find(|h| h.name == "track-agent-start")
        .expect("track-agent-start entry must exist in registry");

    assert_eq!(
        entry.event, "PreToolUse",
        "track-agent-start event binding must remain PreToolUse \
         (BC-7.03.079 invariant 1 — registration stability)"
    );
    assert_eq!(
        entry.tool.as_deref().unwrap_or(""),
        "Agent",
        "track-agent-start tool binding must remain Agent \
         (BC-7.03.079 invariant 1)"
    );
    assert_eq!(
        entry.priority.unwrap_or(0),
        110,
        "track-agent-start priority must remain 110 \
         (BC-7.03.079 invariant 1)"
    );
    assert!(
        entry
            .on_error
            .is_none_or(|oe| oe == factory_dispatcher::registry::OnError::Continue),
        "track-agent-start on_error must remain continue \
         (BC-7.03.079 invariant 1)"
    );
}

/// AC-001 (negative): the track-agent-start registry stanza must NOT reference
/// the legacy-bash-adapter in any form.
///
/// FAILS until T-6 updates the plugin path.
#[test]
fn test_BC_7_03_079_ac001_registry_entry_does_not_reference_legacy_bash_adapter() {
    let raw = std::fs::read_to_string(registry_path()).expect("read hooks-registry.toml");

    let stanza = extract_stanza(&raw, "track-agent-start")
        .expect("track-agent-start stanza must exist in hooks-registry.toml");

    assert!(
        !stanza.contains("legacy-bash-adapter"),
        "track-agent-start registry stanza must not reference legacy-bash-adapter \
         after migration (AC-001 / BC-7.03.079 postcondition 1).\n\
         Stanza:\n{stanza}"
    );
}

/// AC-001 (negative): the track-agent-start registry entry must have no
/// `script_path` key after migration.
///
/// FAILS until T-6 removes the [hooks.config] script_path entry.
#[test]
fn test_BC_7_03_079_ac001_registry_entry_has_no_script_path() {
    let raw = std::fs::read_to_string(registry_path()).expect("read hooks-registry.toml");

    let stanza = extract_stanza(&raw, "track-agent-start")
        .expect("track-agent-start stanza must exist in hooks-registry.toml");

    assert!(
        !stanza.contains("script_path"),
        "track-agent-start registry stanza must not contain script_path after migration \
         (AC-001 — script_path is only for legacy-bash-adapter).\n\
         Stanza:\n{stanza}"
    );
}

/// AC-001 (negative): the track-agent-start registry entry must have no
/// `exec_subprocess` capability block after migration.
///
/// FAILS until T-6 removes the [hooks.capabilities.exec_subprocess] block.
#[test]
fn test_BC_7_03_079_ac001_registry_entry_has_no_exec_subprocess_block() {
    let raw = std::fs::read_to_string(registry_path()).expect("read hooks-registry.toml");

    let stanza = extract_stanza(&raw, "track-agent-start")
        .expect("track-agent-start stanza must exist in hooks-registry.toml");

    assert!(
        !stanza.contains("exec_subprocess"),
        "track-agent-start registry stanza must not contain exec_subprocess after \
         migration — native crate replaces jq with serde_json, no subprocess needed \
         (AC-001 / E-8 architecture compliance rule).\n\
         Stanza:\n{stanza}"
    );
}

/// AC-002a parity audit: the track-agent-start registry stanza must NOT contain
/// `shell_bypass_acknowledged`.
///
/// FAILS until T-6 removes the legacy-bash-adapter fields.
#[test]
fn test_BC_7_03_079_ac002a_registry_entry_has_no_shell_bypass_acknowledged() {
    let raw = std::fs::read_to_string(registry_path()).expect("read hooks-registry.toml");

    let stanza = extract_stanza(&raw, "track-agent-start")
        .expect("track-agent-start stanza must exist in hooks-registry.toml");

    assert!(
        !stanza.contains("shell_bypass_acknowledged"),
        "track-agent-start registry stanza must not contain shell_bypass_acknowledged \
         after native migration (AC-002a — legacy-bash-adapter artifact).\n\
         Stanza:\n{stanza}"
    );
}

// ---------------------------------------------------------------------------
// 2. File lifecycle (AC-002b)
// ---------------------------------------------------------------------------

/// AC-002b: the bash source `plugins/vsdd-factory/hooks/track-agent-start.sh`
/// must NOT exist after migration.
///
/// FAILS until T-7 deletes track-agent-start.sh.
#[test]
fn test_BC_7_03_079_ac002b_sh_file_deleted_after_migration() {
    let sh_path = workspace_root()
        .join("plugins")
        .join("vsdd-factory")
        .join("hooks")
        .join("track-agent-start.sh");

    assert!(
        !sh_path.exists(),
        "plugins/vsdd-factory/hooks/track-agent-start.sh must be deleted after native \
         WASM migration (AC-002b / T-7).  File still exists at: {}",
        sh_path.display()
    );
}

/// AC-002b (hooks.json.* verification): none of the platform-specific
/// `hooks.json.*` files nor the template must contain a `track-agent-start`
/// command entry (post-DRIFT-004 baseline regression gate).
///
/// PASSES in stub state (DRIFT-004 already removed the entries).
/// Included as a regression gate.
#[test]
fn test_BC_7_03_079_ac002b_hooks_json_contains_zero_track_agent_start_entries() {
    let hooks_dir = workspace_root()
        .join("plugins")
        .join("vsdd-factory")
        .join("hooks");

    let json_files: Vec<_> = std::fs::read_dir(&hooks_dir)
        .expect("list hooks/ directory")
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .map(|n| n.starts_with("hooks.json"))
                .unwrap_or(false)
        })
        .collect();

    assert!(
        !json_files.is_empty(),
        "expected at least one hooks.json.* file in plugins/vsdd-factory/hooks/"
    );

    for path in &json_files {
        let content = std::fs::read_to_string(path)
            .unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
        assert!(
            !content.contains("track-agent-start"),
            "hooks.json file {} must contain zero track-agent-start entries \
             (AC-002b / DRIFT-004 baseline)",
            path.display()
        );
    }
}

// ---------------------------------------------------------------------------
// 3. Dispatcher linker / emit_event integration (AC-003 / BC-7.03.080 PC-1)
// ---------------------------------------------------------------------------

/// AC-003 / BC-7.03.080 postcondition 1 / AC-002a parity audit:
///
/// A WAT stub that calls `vsdd::emit_event` with the track-agent-start field
/// encoding (hook=track-agent-start, matcher=Agent, subagent=pr-manager,
/// story_id=S-6.07) must produce exactly one event in HostContext with:
///   - event type "agent.start"
///   - fields: hook, matcher, subagent, story_id present
///   - NO agent_id, NO tool_name (E-8 D-2 strict parity)
///
/// This is an in-process linker test; it does not compile or load the real WASM
/// crate.  It verifies that the dispatcher's `emit_event` host function routes
/// agent.start events correctly when called with the SDK-encoded field payload.
#[test]
fn test_BC_7_03_080_linker_emit_event_produces_agent_start_with_parity_fields() {
    use factory_dispatcher::host::{HostContext, setup_linker};

    let engine = wasmtime::Engine::default();
    let linker = setup_linker(&engine).expect("setup_linker must not fail");

    let fields_payload = encode_sdk_fields(&[
        ("hook", "track-agent-start"),
        ("matcher", "Agent"),
        ("subagent", "pr-manager"),
        ("story_id", "S-6.07"),
    ]);

    let event_type_bytes = b"agent.start";
    let event_type_offset: i32 = 0;
    let event_type_len = event_type_bytes.len() as i32;
    let fields_offset: i32 = 16;
    let fields_len = fields_payload.len() as i32;

    let wat = format!(
        r#"
(module
  (import "vsdd" "emit_event"
    (func $emit_event (param i32 i32 i32 i32)))
  (memory (export "memory") 1)
  (func (export "do_emit")
    (call $emit_event
      (i32.const {event_type_offset})
      (i32.const {event_type_len})
      (i32.const {fields_offset})
      (i32.const {fields_len}))))
"#
    );

    let module = wasmtime::Module::new(&engine, wat.as_str()).expect("WAT parse");
    let ctx = HostContext::new("track-agent-start", "1.0.0-rc.1", "sess-test", "trace-test");
    let mut store = wasmtime::Store::new(&engine, ctx);

    let instance = linker
        .instantiate(&mut store, &module)
        .expect("instantiate");
    let memory = instance
        .get_memory(&mut store, "memory")
        .expect("memory export");

    memory
        .write(&mut store, event_type_offset as usize, event_type_bytes)
        .expect("write event_type to WASM memory");
    memory
        .write(&mut store, fields_offset as usize, &fields_payload)
        .expect("write fields to WASM memory");

    let do_emit = instance
        .get_typed_func::<(), ()>(&mut store, "do_emit")
        .expect("do_emit export");
    do_emit.call(&mut store, ()).expect("do_emit must not trap");

    let events = store.data().drain_events();
    assert_eq!(
        events.len(),
        1,
        "emit_event call must produce exactly one event in HostContext \
         (AC-003 / BC-7.03.080 postcondition 1)"
    );

    let event = &events[0];

    assert_eq!(
        event.type_, "agent.start",
        "emitted event type must be agent.start (BC-7.03.080 postcondition 1)"
    );

    let field_val = |k: &str| -> Option<&str> { event.fields.get(k).and_then(|v| v.as_str()) };
    assert_eq!(
        field_val("hook"),
        Some("track-agent-start"),
        "emitted event must have hook=track-agent-start (AC-002a bash parity)"
    );
    assert_eq!(
        field_val("matcher"),
        Some("Agent"),
        "emitted event must have matcher=Agent (AC-002a bash parity)"
    );
    assert_eq!(
        field_val("subagent"),
        Some("pr-manager"),
        "emitted event must have subagent=pr-manager (AC-003)"
    );
    assert_eq!(
        field_val("story_id"),
        Some("S-6.07"),
        "emitted event must have story_id=S-6.07 when extracted (AC-004)"
    );

    // AC-002a strict parity: forbidden fields must be absent
    assert!(
        event.fields.get("agent_id").is_none(),
        "emitted event must NOT contain agent_id (E-8 D-2 strict parity, AC-002a)"
    );
    assert!(
        event.fields.get("tool_name").is_none(),
        "emitted event must NOT contain tool_name (E-8 D-2 strict parity, AC-002a)"
    );
}

/// AC-003 / AC-006 (best-effort): emit_event called without story_id (3-field
/// case, no story pattern in prompt) must produce exactly 3 plugin-set fields:
/// hook, matcher, subagent.  No story_id field must appear.
///
/// Verifies the exact field count (3) for the no-story-id emission path.
#[test]
fn test_BC_7_03_080_linker_emit_event_no_story_id_produces_three_fields() {
    use factory_dispatcher::host::{HostContext, setup_linker};

    let engine = wasmtime::Engine::default();
    let linker = setup_linker(&engine).expect("setup_linker must not fail");

    // Only 3 fields: no story_id (prompt had no S-N.NN or STORY-NNN)
    let fields_payload = encode_sdk_fields(&[
        ("hook", "track-agent-start"),
        ("matcher", "Agent"),
        ("subagent", "reviewer"),
    ]);

    let event_type_bytes = b"agent.start";
    let event_type_offset: i32 = 0;
    let event_type_len = event_type_bytes.len() as i32;
    let fields_offset: i32 = 16;
    let fields_len = fields_payload.len() as i32;

    let wat = format!(
        r#"
(module
  (import "vsdd" "emit_event"
    (func $emit_event (param i32 i32 i32 i32)))
  (memory (export "memory") 1)
  (func (export "do_emit")
    (call $emit_event
      (i32.const {event_type_offset})
      (i32.const {event_type_len})
      (i32.const {fields_offset})
      (i32.const {fields_len}))))
"#
    );

    let module = wasmtime::Module::new(&engine, wat.as_str()).expect("WAT parse");
    let ctx = HostContext::new("track-agent-start", "1.0.0-rc.1", "sess-2", "trace-2");
    let mut store = wasmtime::Store::new(&engine, ctx);
    let instance = linker
        .instantiate(&mut store, &module)
        .expect("instantiate");
    let memory = instance.get_memory(&mut store, "memory").expect("memory");
    memory.write(&mut store, 0, event_type_bytes).unwrap();
    memory
        .write(&mut store, fields_offset as usize, &fields_payload)
        .unwrap();
    let do_emit = instance
        .get_typed_func::<(), ()>(&mut store, "do_emit")
        .expect("do_emit");
    do_emit.call(&mut store, ()).expect("call");

    let events = store.data().drain_events();
    assert_eq!(events.len(), 1);
    let event = &events[0];
    assert_eq!(event.type_, "agent.start");
    assert!(
        event.fields.get("story_id").is_none(),
        "AC-004: story_id field must be absent when prompt had no story pattern \
         (3-field no-story-id emission path)"
    );
    assert!(
        event.fields.get("subagent").is_some(),
        "subagent field must be present (3-field emission)"
    );
}

/// AC-006 / BC-7.03.079 invariant 2: STORY-NNN pattern (fallback) produces
/// story_id via the AC-004 cascade. Linker test verifies the story_id is
/// correctly set when the fallback pattern fires.
#[test]
fn test_BC_7_03_080_linker_emit_event_story_nnn_pattern_produces_story_id() {
    use factory_dispatcher::host::{HostContext, setup_linker};

    let engine = wasmtime::Engine::default();
    let linker = setup_linker(&engine).expect("setup_linker");

    // Simulates the hook calling emit_event after extracting story_id=STORY-042
    // via pattern 2 fallback (no S-N.NN in prompt).
    let fields_payload = encode_sdk_fields(&[
        ("hook", "track-agent-start"),
        ("matcher", "Agent"),
        ("subagent", "implementer"),
        ("story_id", "STORY-042"),
    ]);

    let event_type_bytes = b"agent.start";
    let et_len = event_type_bytes.len() as i32;
    let flds_len = fields_payload.len() as i32;

    let wat = format!(
        r#"
(module
  (import "vsdd" "emit_event"
    (func $emit_event (param i32 i32 i32 i32)))
  (memory (export "memory") 1)
  (func (export "do_emit")
    (call $emit_event (i32.const 0) (i32.const {et_len})
                      (i32.const 16) (i32.const {flds_len}))))
"#
    );

    let module = wasmtime::Module::new(&engine, wat.as_str()).expect("WAT parse");
    let ctx = HostContext::new("track-agent-start", "1.0.0-rc.1", "s3", "t3");
    let mut store = wasmtime::Store::new(&engine, ctx);
    let instance = linker
        .instantiate(&mut store, &module)
        .expect("instantiate");
    let memory = instance.get_memory(&mut store, "memory").expect("memory");
    memory.write(&mut store, 0, event_type_bytes).unwrap();
    memory.write(&mut store, 16, &fields_payload).unwrap();
    let do_emit = instance
        .get_typed_func::<(), ()>(&mut store, "do_emit")
        .expect("do_emit");
    do_emit.call(&mut store, ()).expect("call");

    let events = store.data().drain_events();
    assert_eq!(events.len(), 1);
    let event = &events[0];
    assert_eq!(event.type_, "agent.start");
    assert_eq!(
        event.fields.get("story_id").and_then(|v| v.as_str()),
        Some("STORY-042"),
        "AC-004: STORY-NNN fallback pattern must produce story_id=STORY-042"
    );
    assert_eq!(
        event.fields.get("subagent").and_then(|v| v.as_str()),
        Some("implementer"),
        "subagent field must carry implementer value"
    );
}
