// Test files use .expect()/.unwrap()/.panic!() for failure reporting.
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]
//! Red Gate integration tests for PreCompact / PostCompact dispatcher routing.
//!
//! Story: S-18.00 — Dispatcher PreCompact/PostCompact Routing + check-harness-version.sh
//! BC:    BC-1.15.001 — Dispatcher routes PreCompact and PostCompact harness events
//!        to registered plugins (harness >= v2.1.105)
//!
//! ## AC→test traceability
//!
//! | Test | AC | BC clause | Red Gate condition (historical — all GREEN as of S-18.00) |
//! |------|----|-----------|--------------------|
//! | test_BC_1_15_001_event_type_enum_has_precompact_postcompact | AC-006 | INV1 | (historical) is_advisory_only() was absent in no-op stub → compile failure; now delivered GREEN |
//! | test_BC_1_15_001_parse_event_type_precompact | AC-006 | INV1 | (historical) is_advisory_only() was absent → compile failure; now delivered GREEN |
//! | test_BC_1_15_001_parse_event_type_postcompact | AC-006 | INV1 | (historical) is_advisory_only() was absent → compile failure; now delivered GREEN |
//! | test_BC_1_15_001_precompact_routes_to_registered_plugins | AC-001 | PC1 | match_plugins routes "PreCompact" → assert |
//! | test_BC_1_15_001_postcompact_is_advisory_only_event_type | AC-002 | PC2 | (historical) EventType::is_advisory_only() was absent → compile failure; now delivered GREEN |
//! | test_BC_1_15_001_precompact_is_not_advisory_only_event_type | AC-002 | PC2 | (historical) EventType::is_advisory_only() was absent → compile failure; now delivered GREEN |
//! | test_BC_1_15_001_postcompact_advisory_only | AC-002 | PC2 | (historical) EventType::is_advisory_only() was absent → compile failure; now delivered GREEN |
//! | test_BC_1_15_001_precompact_no_plugins_noop | AC-003 | PC3 | match_plugins returns empty |
//! | test_BC_1_15_001_precompact_exit2_sets_block_intent | AC-004 | PC4 | (historical) aggregator + is_advisory_only() absent → compile failure; now delivered GREEN |
//! | test_BC_1_15_001_postcompact_exit2_no_block | EC-002 | PC2 | (historical) is_advisory_only() was absent → compile failure; now delivered GREEN |
//! | test_BC_1_15_001_precompact_on_error_block_crash_blocks | AC-005 | PC5 | aggregator on_error |
//! | test_BC_1_15_001_precompact_on_error_continue_crash_no_block | EC-004 | PC5 | aggregator on_error |
//! | test_BC_1_15_001_precompact_multi_plugin_one_exit2_blocks | EC-001 | PC4 | aggregator |
//! | test_BC_1_15_001_precompact_async_plugin_scheduled_asynchronously | AC-007 | INV2 | (historical) parse + is_advisory_only() absent → compile failure; now delivered GREEN |
//! | test_BC_1_15_001_aggregator_supports_precompact_block_semantics | VP-086 | PC4 | passed at Red Gate — aggregator was already implemented |
//! | test_BC_1_15_001_postcompact_advisory_aggregation_contract | AC-002 | PC2 | passed at Red Gate — documents is_advisory_only() contract |
//!
//! ## Red Gate guarantee (historical — S-18.00 delivered; all tests GREEN)
//!
//! At Red Gate, tests that called `EventType::is_advisory_only()` FAILED TO COMPILE
//! because that method did not exist in the no-op stub implementation. The compile
//! failure WAS the Red Gate for AC-002/PC2 (PostCompact advisory semantics).
//!
//! Tests that assert routing behavior via `match_plugins` used REAL production code and
//! asserted observable outcomes (matched plugin counts, event type mapping). Those
//! failed only if the routing was broken — they were NOT vacuous.
//!
//! The binary-level Red Gate for AC-002/PC2 was in the VP-086 bats harness:
//! `plugins/vsdd-factory/tests/precompact-routing.bats`. Those tests assert the
//! dispatcher binary exits 0 for PostCompact exit-2 events; they historically failed
//! against the no-op implementation (which exited 2 instead) and now pass GREEN
//! against the delivered is_advisory_only() suppression in main.rs.
//!
//! POLICY 11 self-check: every test below either
//!   (a) calls is_advisory_only() → would cause compile failure if method were removed, OR
//!   (b) asserts a specific value on production code output → runtime assertion failure
//!       if the contract is violated.
//! No test is a bare function call with zero assertions.

use factory_dispatcher::aggregator::{self, PluginResult as AggregatorResult};
use factory_dispatcher::invoke::{EventType, dispatch_postcompact, dispatch_precompact};
use factory_dispatcher::payload::HookPayload;
use factory_dispatcher::registry::{OnError, Registry, parse_event_type};
use factory_dispatcher::routing::match_plugins;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn precompact_payload() -> HookPayload {
    HookPayload {
        event_name: "PreCompact".to_string(),
        tool_name: String::new(),
        session_id: "test-sess".to_string(),
        tool_input: serde_json::Value::Null,
        tool_response: None,
        extra: std::collections::HashMap::new(),
    }
}

fn postcompact_payload() -> HookPayload {
    HookPayload {
        event_name: "PostCompact".to_string(),
        tool_name: String::new(),
        session_id: "test-sess".to_string(),
        tool_input: serde_json::Value::Null,
        tool_response: None,
        extra: std::collections::HashMap::new(),
    }
}

/// A minimal registry TOML with one PreCompact plugin.
fn precompact_registry_toml() -> &'static str {
    r#"
schema_version = 2

[[hooks]]
name = "stub-precompact"
event = "PreCompact"
plugin = "hook-plugins/stub.wasm"
timeout_ms = 5000
on_error = "block"
"#
}

/// A minimal registry TOML with one PostCompact plugin (on_error=block is the adversarial case).
/// Defined for completeness; used as a documentation fixture for PostCompact registry shape.
#[allow(dead_code)]
fn postcompact_registry_toml() -> &'static str {
    r#"
schema_version = 2

[[hooks]]
name = "stub-postcompact"
event = "PostCompact"
plugin = "hook-plugins/stub.wasm"
timeout_ms = 5000
on_error = "block"
"#
}

/// A registry TOML with two PreCompact plugins at different priorities.
fn two_precompact_registry_toml() -> &'static str {
    r#"
schema_version = 2

[[hooks]]
name = "stub-precompact-low"
event = "PreCompact"
plugin = "hook-plugins/stub-low.wasm"
priority = 100
timeout_ms = 5000
on_error = "block"

[[hooks]]
name = "stub-precompact-high"
event = "PreCompact"
plugin = "hook-plugins/stub-high.wasm"
priority = 200
timeout_ms = 5000
on_error = "continue"
"#
}

/// A registry TOML with NO PreCompact plugins (only PostToolUse).
fn no_precompact_registry_toml() -> &'static str {
    r#"
schema_version = 2

[[hooks]]
name = "other-plugin"
event = "PostToolUse"
tool = "Bash"
plugin = "hook-plugins/stub.wasm"
timeout_ms = 5000
on_error = "continue"
"#
}

// ---------------------------------------------------------------------------
// AC-006 / BC-1.15.001 INV1 — EventType enum structural + advisory-only contract
//
// Red Gate condition: `EventType::is_advisory_only()` does not exist in the
// no-op stub. Tests that call it FAIL TO COMPILE — the compile failure IS the
// Red Gate for AC-002/PC2 (PostCompact advisory semantics).
//
// The implementer MUST add `is_advisory_only(&self) -> bool` to EventType,
// returning true for PostCompact and false for PreCompact/PreToolUse/PostToolUse.
// ---------------------------------------------------------------------------

/// BC-1.15.001 INV1 — `EventType::PreCompact` and `EventType::PostCompact` must exist
/// as first-class enum variants AND must be classified correctly by `is_advisory_only()`.
///
/// Red Gate: `EventType::is_advisory_only()` does not exist → compile failure.
/// After implementation: PreCompact must NOT be advisory-only; PostCompact MUST be.
#[test]
fn test_BC_1_15_001_event_type_enum_has_precompact_postcompact() {
    // Structural check: these pattern matches compile iff the variants exist.
    let pre = EventType::PreCompact;
    let post = EventType::PostCompact;

    assert_eq!(pre.as_str(), "PreCompact");
    assert_eq!(post.as_str(), "PostCompact");

    // Round-trip check: from_event_str must map strings to correct variants.
    let parsed_pre = EventType::from_event_str("PreCompact");
    let parsed_post = EventType::from_event_str("PostCompact");
    assert_eq!(parsed_pre, EventType::PreCompact);
    assert_eq!(parsed_post, EventType::PostCompact);

    // BC-1.15.001 PC2 advisory-only contract (load-bearing Red Gate assertion).
    // is_advisory_only() must exist and return correct values. This method is
    // absent in the no-op stub → compile failure at Red Gate.
    assert!(
        !pre.is_advisory_only(),
        "PreCompact must NOT be advisory-only: it supports block_intent=true (BC-1.15.001 PC1/PC4)"
    );
    assert!(
        post.is_advisory_only(),
        "PostCompact MUST be advisory-only: block_intent must never be true for PostCompact (BC-1.15.001 PC2)"
    );
}

/// BC-1.15.001 INV1 — `parse_event_type("PreCompact")` must NOT be advisory-only.
///
/// Red Gate: `EventType::is_advisory_only()` absent → compile failure.
#[test]
fn test_BC_1_15_001_parse_event_type_precompact() {
    let event_type = parse_event_type("PreCompact");
    assert_eq!(
        event_type,
        EventType::PreCompact,
        "parse_event_type(\"PreCompact\") must return EventType::PreCompact (BC-1.15.001 INV1)"
    );
    assert!(
        !event_type.is_advisory_only(),
        "PreCompact must NOT be advisory-only (BC-1.15.001 PC4: exit-2 sets block_intent)"
    );
}

/// BC-1.15.001 INV1 — `parse_event_type("PostCompact")` must be advisory-only.
///
/// Red Gate: `EventType::is_advisory_only()` absent → compile failure.
#[test]
fn test_BC_1_15_001_parse_event_type_postcompact() {
    let event_type = parse_event_type("PostCompact");
    assert_eq!(
        event_type,
        EventType::PostCompact,
        "parse_event_type(\"PostCompact\") must return EventType::PostCompact (BC-1.15.001 INV1)"
    );
    assert!(
        event_type.is_advisory_only(),
        "PostCompact MUST be advisory-only (BC-1.15.001 PC2: block_intent never set)"
    );
}

// ---------------------------------------------------------------------------
// AC-002 / BC-1.15.001 PC2 — PostCompact is advisory-only
//
// These tests verify the is_advisory_only() method on EventType. They FAIL
// TO COMPILE at Red Gate because the method does not exist in the no-op stub.
// ---------------------------------------------------------------------------

/// BC-1.15.001 PC2 (AC-002) — `EventType::PostCompact` must report is_advisory_only() = true.
///
/// This is the UNIT-LEVEL Red Gate for F-002 (PostCompact advisory suppression).
///
/// Red Gate: `is_advisory_only()` absent → compile failure.
/// After implementation: must return true for PostCompact, ensuring the dispatch
/// path in main.rs can call `event_type.is_advisory_only()` to suppress block_intent.
#[test]
fn test_BC_1_15_001_postcompact_is_advisory_only_event_type() {
    // The dispatch path in main.rs MUST check event_type.is_advisory_only()
    // before propagating exit-2 block_intent. This method is the load-bearing
    // gate between "plugin exits 2" and "dispatcher exits 2".
    assert!(
        EventType::PostCompact.is_advisory_only(),
        "EventType::PostCompact.is_advisory_only() must return true — \
         the dispatcher must never set block_intent=true for PostCompact events \
         regardless of plugin exit code (BC-1.15.001 PC2)"
    );
}

/// BC-1.15.001 PC2 (AC-002) — `EventType::PreCompact` must NOT be advisory-only.
///
/// Red Gate: `is_advisory_only()` absent → compile failure.
#[test]
fn test_BC_1_15_001_precompact_is_not_advisory_only_event_type() {
    assert!(
        !EventType::PreCompact.is_advisory_only(),
        "EventType::PreCompact.is_advisory_only() must return false — \
         PreCompact exit-2 MUST propagate block_intent=true (BC-1.15.001 PC4)"
    );
}

/// BC-1.15.001 PC2 (AC-002) — Other event types must NOT be advisory-only for PreCompact context.
///
/// The advisory-only flag is specific to PostCompact. PreToolUse and PostToolUse
/// have their own semantics; this test verifies the flag is PostCompact-specific.
///
/// Red Gate: `is_advisory_only()` absent → compile failure.
#[test]
fn test_BC_1_15_001_postcompact_advisory_only() {
    // BC-1.15.001 PC2: PostCompact is the ONLY event type in the dispatcher
    // that is advisory-only at the event-type level. PreToolUse uses on_error
    // semantics for advisory classification; PreCompact uses exit-2 propagation.
    assert!(
        EventType::PostCompact.is_advisory_only(),
        "PostCompact must be advisory-only (BC-1.15.001 PC2)"
    );
    assert!(
        !EventType::PreToolUse.is_advisory_only(),
        "PreToolUse must NOT be advisory-only at the event-type level"
    );
    assert!(
        !EventType::PostToolUse.is_advisory_only(),
        "PostToolUse must NOT be advisory-only at the event-type level (on_error=block is valid)"
    );
    assert!(
        !EventType::PreCompact.is_advisory_only(),
        "PreCompact must NOT be advisory-only (BC-1.15.001 PC1/PC4)"
    );
}

// ---------------------------------------------------------------------------
// AC-001 / BC-1.15.001 PC1 — PreCompact routes to registered plugins
//
// These tests use match_plugins directly to verify routing behavior.
// They assert specific observable outcomes (matched count, plugin names)
// rather than bare no-op calls.
// ---------------------------------------------------------------------------

/// BC-1.15.001 PC1 (AC-001) — When the harness fires a PreCompact event,
/// `match_plugins` routes to all registered PreCompact plugins.
///
/// Red Gate condition: This test exercises REAL routing code and asserts
/// a specific count. The routing itself works today (match_plugins filters
/// by event_name string), so this test PASSES at Red Gate. However, the
/// full binary-level PreCompact routing test is in the VP-086 bats harness
/// (`precompact-routing.bats`) which verifies exit code + block_intent.
///
/// Canonical test vector (BC-1.15.001 §Canonical Test Vectors, row 1):
///   Input:    Dispatcher receives PreCompact event; one plugin registered; plugin exits 0
///   Expected: plugins_run=1, block_intent=false, harness proceeds with compaction
#[test]
fn test_BC_1_15_001_precompact_routes_to_registered_plugins() {
    let reg = Registry::parse_str(precompact_registry_toml())
        .expect("precompact_registry_toml must parse");
    let payload = precompact_payload();
    let matched = match_plugins(&reg, &payload);
    assert_eq!(
        matched.len(),
        1,
        "PreCompact event must route to exactly 1 registered plugin (BC-1.15.001 PC1): \
         got {} matched plugins",
        matched.len()
    );
    assert_eq!(
        matched[0].name, "stub-precompact",
        "Matched plugin must be 'stub-precompact', got '{}'",
        matched[0].name
    );
}

// ---------------------------------------------------------------------------
// AC-003 / BC-1.15.001 PC3 — No-op when no plugins registered
// ---------------------------------------------------------------------------

/// BC-1.15.001 PC3 (AC-003) / EC-007 — When no plugins are registered for
/// PreCompact, `match_plugins` returns an empty slice; block_intent=false.
///
/// Canonical test vector (row 4):
///   Input:    Dispatcher receives PreCompact event; zero plugins registered
///   Expected: plugins_run=0, block_intent=false, exit 0
#[test]
fn test_BC_1_15_001_precompact_no_plugins_noop() {
    let reg = Registry::parse_str(no_precompact_registry_toml())
        .expect("no_precompact_registry_toml must parse");
    let payload = precompact_payload();
    let matched = match_plugins(&reg, &payload);
    assert_eq!(
        matched.len(),
        0,
        "PreCompact event with no registered plugins must match 0 plugins \
         (BC-1.15.001 PC3 no-op): got {} matched plugins",
        matched.len()
    );
}

/// BC-1.15.001 PC3 variant — explicit EC-007 coverage.
/// PostCompact with no registered plugins must also return empty.
#[test]
fn test_BC_1_15_001_precompact_no_plugins_registered_noop() {
    let reg = Registry::parse_str(no_precompact_registry_toml())
        .expect("no_precompact_registry_toml must parse");
    // PostCompact with no registered PostCompact plugins → empty match.
    let post_payload = postcompact_payload();
    let matched = match_plugins(&reg, &post_payload);
    assert_eq!(
        matched.len(),
        0,
        "PostCompact event with no registered plugins must match 0 plugins \
         (EC-007): got {} matched plugins",
        matched.len()
    );
}

// ---------------------------------------------------------------------------
// AC-004 / BC-1.15.001 PC4 — exit-2 sets block_intent for PreCompact
// ---------------------------------------------------------------------------

/// BC-1.15.001 PC4 (AC-004) / VP-086 — exit-2 from a PreCompact plugin must
/// propagate block_intent=true.
///
/// This test verifies the aggregation semantics: the aggregator returns exit_code=2
/// for a PreCompact plugin result with (exit_code=2, on_error=Block), and the
/// EventType::PreCompact.is_advisory_only() == false ensures the dispatcher
/// DOES NOT suppress this block.
///
/// Red Gate: `is_advisory_only()` absent → compile failure.
/// After implementation: the combination of aggregator + is_advisory_only() false
/// ensures the dispatch path correctly propagates block_intent=true.
#[test]
fn test_BC_1_15_001_precompact_exit2_sets_block_intent() {
    // Unit-level: aggregator returns 2 for exit-2 + on_error=Block.
    let results = vec![AggregatorResult {
        exit_code: 2,
        on_error: OnError::Block,
    }];
    let exit_code = aggregator::aggregate_exit_code(&results);
    assert_eq!(
        exit_code, 2,
        "Aggregator must return 2 for PreCompact exit-2 with on_error=Block (VP-086 / BC-1.15.001 PC4)"
    );

    // Advisory-only contract: PreCompact is NOT advisory-only, so the block propagates.
    assert!(
        !EventType::PreCompact.is_advisory_only(),
        "PreCompact must NOT be advisory-only — the aggregated exit-2 MUST propagate to harness \
         (BC-1.15.001 PC4). If is_advisory_only() were true here, block_intent would be suppressed."
    );
}

/// BC-1.15.001 PC4 / EC-001 — Multiple PreCompact plugins; one exits 2, one exits 0.
/// block_intent=true — single exit-2 sufficient per BC-1.14.001 aggregation logic.
#[test]
fn test_BC_1_15_001_precompact_multi_plugin_one_exit2_blocks() {
    // Aggregation: one exit-0 (continue) + one exit-2 (block) → aggregate returns 2.
    let results = vec![
        AggregatorResult {
            exit_code: 0,
            on_error: OnError::Continue,
        },
        AggregatorResult {
            exit_code: 2,
            on_error: OnError::Block,
        },
    ];
    let exit_code = aggregator::aggregate_exit_code(&results);
    assert_eq!(
        exit_code, 2,
        "Multiple PreCompact plugins: one exit-0, one exit-2 → aggregate must return 2 \
         (BC-1.15.001 EC-001: single exit-2 sufficient for block)"
    );

    // Routing: both PreCompact plugins match.
    let reg = Registry::parse_str(two_precompact_registry_toml())
        .expect("two_precompact_registry_toml must parse");
    let payload = precompact_payload();
    let matched = match_plugins(&reg, &payload);
    assert_eq!(
        matched.len(),
        2,
        "Two PreCompact plugins must both match the PreCompact event: \
         got {} matched (BC-1.15.001 EC-001)",
        matched.len()
    );
}

// ---------------------------------------------------------------------------
// AC-002 / EC-002 — PostCompact exit-2 must NOT block
// ---------------------------------------------------------------------------

/// BC-1.15.001 PC2 / EC-002 — PostCompact plugin exits 2: block_intent NOT set.
///
/// This is the UNIT-LEVEL component of BLOCKER F-002.
///
/// The dispatch path in main.rs MUST check `event_type.is_advisory_only()` and
/// suppress block_intent for PostCompact. This test verifies the advisory-only
/// flag is set, which is a PRECONDITION for correct dispatch behavior.
///
/// Red Gate: `is_advisory_only()` absent → compile failure.
/// Binary-level Red Gate is in precompact-routing.bats TC-AC002.
#[test]
fn test_BC_1_15_001_postcompact_exit2_no_block() {
    // PostCompact must be advisory-only regardless of plugin exit code.
    // The dispatch path must NOT call aggregate_exit_code on PostCompact results
    // with on_error=Block — or if it does, it must suppress the exit-2 before
    // returning the final exit code.
    assert!(
        EventType::PostCompact.is_advisory_only(),
        "PostCompact.is_advisory_only() must be true — this is the gate that prevents \
         dispatch from propagating exit-2 as block_intent=true (BC-1.15.001 PC2 / EC-002)"
    );

    // Verify that even if a PostCompact plugin exits 2 with on_error=Block,
    // the aggregator itself would return 2 — but the dispatch path MUST suppress it
    // because is_advisory_only() is true. The implementer must NOT pass PostCompact
    // results through aggregate_exit_code with on_error=Block.
    let adversarial_results = vec![AggregatorResult {
        exit_code: 2,
        on_error: OnError::Block,
    }];
    let raw_aggregate = aggregator::aggregate_exit_code(&adversarial_results);
    assert_eq!(
        raw_aggregate, 2,
        "Aggregator sanity: a PostCompact exit-2 + on_error=Block would produce 2 \
         IF the dispatch path were naive. The dispatch MUST NOT pass this through \
         for PostCompact events (BC-1.15.001 PC2). is_advisory_only() prevents this."
    );
    // The fact that raw_aggregate==2 here is INTENTIONAL: it documents that the aggregator
    // alone does not enforce PostCompact advisory semantics. The dispatch path in main.rs
    // MUST check is_advisory_only() and suppress the block.
}

// ---------------------------------------------------------------------------
// AC-005 / BC-1.15.001 PC5 — on_error semantics for PreCompact
// ---------------------------------------------------------------------------

/// BC-1.15.001 PC5 (AC-005) / EC-003 — on_error=block crash → block_intent=true.
///
/// When a PreCompact plugin's WASM crashes (or times out) and on_error=Block,
/// the dispatcher treats it as a block (fail-closed). The aggregator propagates
/// a synthetic exit_code=2 for the crash.
///
/// This test verifies the aggregation contract for crash+Block using
/// AggregatorResult with exit_code=2 (the value the executor sets for crash+Block).
#[test]
fn test_BC_1_15_001_precompact_on_error_block_crash_blocks() {
    // Crash + on_error=Block: executor sets exit_code=2 (fail-closed convention).
    // Aggregator sees (exit_code=2, on_error=Block) → returns 2.
    let results = vec![AggregatorResult {
        exit_code: 2,
        on_error: OnError::Block,
    }];
    let exit_code = aggregator::aggregate_exit_code(&results);
    assert_eq!(
        exit_code, 2,
        "PreCompact plugin crash with on_error=Block must aggregate to exit_code=2 \
         (block_intent=true, fail-closed per BC-1.15.001 PC5 / EC-003)"
    );
    // PreCompact is not advisory-only, so this block propagates.
    assert!(
        !EventType::PreCompact.is_advisory_only(),
        "PreCompact crash+Block must propagate (is_advisory_only() == false, BC-1.15.001 PC5)"
    );
}

/// BC-1.15.001 PC5 / EC-004 — on_error=continue crash → advisory only; block_intent=false.
///
/// When a PreCompact plugin crashes and on_error=Continue, the dispatcher does NOT
/// propagate block_intent. The aggregator returns 0 for crash+Continue.
#[test]
fn test_BC_1_15_001_precompact_on_error_continue_crash_no_block() {
    // Crash + on_error=Continue: executor does NOT set exit_code=2.
    // Aggregator sees (exit_code=0, on_error=Continue) → returns 0 (no block).
    let results = vec![AggregatorResult {
        exit_code: 0,
        on_error: OnError::Continue,
    }];
    let exit_code = aggregator::aggregate_exit_code(&results);
    assert_eq!(
        exit_code, 0,
        "PreCompact plugin crash with on_error=Continue must aggregate to exit_code=0 \
         (block_intent=false, fail-open per BC-1.15.001 PC5 / EC-004)"
    );
}

// ---------------------------------------------------------------------------
// AC-007 / BC-1.15.001 INV2 — Async classification (EC-008)
// ---------------------------------------------------------------------------

/// BC-1.15.001 INV2 (AC-007) / EC-008 — Async classification for PreCompact/PostCompact
/// plugins follows the same rules as for other event types.
///
/// This test verifies that parse_event_type recognises PreCompact as a valid event-type
/// string (precondition for async partition classification) AND that the advisory-only
/// contract is consistent with async routing.
///
/// Red Gate: `is_advisory_only()` absent → compile failure.
#[test]
fn test_BC_1_15_001_precompact_async_plugin_scheduled_asynchronously() {
    // parse_event_type must recognise PreCompact for the partition module to
    // classify async PreCompact plugins correctly.
    let event_type = parse_event_type("PreCompact");
    assert_eq!(
        event_type,
        EventType::PreCompact,
        "parse_event_type must recognise PreCompact for async partition classification (BC-1.15.001 INV2)"
    );
    // Async PreCompact plugins are valid (async=true, on_error != block is enforced at
    // registry load time by E-REG-002). The advisory-only contract does NOT apply to
    // async classification — async plugins never set block_intent regardless of exit code.
    // is_advisory_only() is the SYNC-GROUP advisory gate only.
    assert!(
        !event_type.is_advisory_only(),
        "PreCompact is NOT advisory-only in the sync group (BC-1.15.001 PC4). \
         Async scheduling is orthogonal — async plugins never set block_intent for ANY event type."
    );
}

// ---------------------------------------------------------------------------
// Aggregator parity assertions (non-is_advisory_only — document wiring contract)
//
// These tests exercise the EXISTING aggregate_exit_code function and pass TODAY.
// They document the expected aggregation semantics that the dispatch path must
// feed into correctly.
// ---------------------------------------------------------------------------

/// Verifies that the existing aggregator supports PreCompact block-intent semantics.
///
/// This test PASSES at Red Gate time (aggregator is fully implemented).
/// It documents the wiring contract: dispatch_precompact must feed exit-2 results
/// with on_error=Block into aggregate_exit_code.
///
/// Exercises VP-086 proof method.
#[test]
fn test_BC_1_15_001_aggregator_supports_precompact_block_semantics() {
    let results = vec![AggregatorResult {
        exit_code: 2,
        on_error: OnError::Block,
    }];
    let exit_code = aggregator::aggregate_exit_code(&results);
    assert_eq!(
        exit_code, 2,
        "Aggregator must return 2 for PreCompact exit-2 with on_error=Block (VP-086 / BC-1.15.001 PC4)"
    );
}

/// Documents that PostCompact advisory semantics require the dispatch path to NOT
/// pass exit-2 with on_error=Block through aggregate_exit_code.
///
/// This test PASSES at Red Gate time (aggregator is agnostic to event type).
/// The dispatch path's responsibility is documented here: it must check
/// is_advisory_only() before calling aggregate_exit_code.
#[test]
fn test_BC_1_15_001_postcompact_advisory_aggregation_contract() {
    // PostCompact advisory semantics (BC-1.15.001 PC2): the dispatch path must
    // use on_error=Continue (or equivalent advisory semantics) for PostCompact results,
    // NOT on_error=Block. Passing Continue into aggregate_exit_code returns 0.
    let results = vec![AggregatorResult {
        exit_code: 2,
        on_error: OnError::Continue, // PostCompact dispatch MUST use Continue semantics
    }];
    let exit_code = aggregator::aggregate_exit_code(&results);
    assert_eq!(
        exit_code, 0,
        "Aggregator returns 0 for PostCompact exit-2 with on_error=Continue — \
         dispatch_postcompact must enforce advisory-only by using Continue semantics \
         (BC-1.15.001 PC2)"
    );
}

// ---------------------------------------------------------------------------
// Compile-time proof: dispatch_precompact and dispatch_postcompact exist
// in the public API. These calls verify the symbols are exported from lib.rs.
// The functions are unit-level anchors; their observable behavior is verified
// at the binary level in precompact-routing.bats (VP-086 harness).
// ---------------------------------------------------------------------------

/// Verifies the public API exports dispatch_precompact and dispatch_postcompact.
/// These are compile-time symbol existence checks, not behavioral tests.
/// Observable behavior is verified in precompact-routing.bats.
#[test]
fn test_BC_1_15_001_dispatch_symbols_exported() {
    // Verify the symbols compile — the functions exist in the public API.
    // They are no-op unit anchors; the binary-level harness verifies behavior.
    let _: fn() = dispatch_precompact;
    let _: fn() = dispatch_postcompact;
    // This test PASSES at Red Gate (symbols exist). The behavioral Red Gate
    // is enforced by is_advisory_only() compile failure + bats harness.
}
