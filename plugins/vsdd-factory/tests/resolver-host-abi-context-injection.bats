#!/usr/bin/env bats
#
# resolver-host-abi-context-injection.bats
#
# S-12.06: Context Injection Contract — HOST_ABI.md failing tests (Step 3)
#
# All tests assert on the EXPECTED post-Step-4 state of HOST_ABI.md.
# Every test MUST FAIL today because the "## Context Injection Contract"
# section does not yet exist — only the placeholder HTML comment from Step 2.
#
# AC traces:
#   AC-001 -> BC-4.12.002 PC9   (section heading + RESOLVER_ABI_VERSION presence)
#   AC-002 -> BC-4.12.001 PC1-5 (resolver lifecycle documented)
#   AC-003 -> BC-4.12.002 PC1-7 (ResolverInput/ResolverOutput type shapes + RESOLVER_ABI_VERSION)
#   AC-004 -> BC-4.12.003 INV1-4 (capability model documented)
#   AC-005 -> BC-4.12.004 PC1-5 (error/crash isolation documented)
#   AC-006 -> BC-4.12.005 PC1-8 (merging contract documented)
#   AC-007 -> BC-1.13.001 PC3-6 (needs_context mechanism documented)
#   AC-008 -> BC-1.13.001 INV1 + BC-4.12.002 PC9 (factory-agnostic vocabulary)
#   AC-009 -> BC-4.12.001 + BC-4.12.002 (ADR-018 and all 6 BCs cross-referenced)
#   AC-010 -> BC-4.12.002 PC5   (#[resolver] macro and resolver-authoring feature)
#   EC-001 -> BC-1.13.001 PC1   (absent-registry-file = zero resolvers, NOT error)
#   EC-004 -> BC-4.12.005 EC-005 (duplicate context_key = startup error)
#

setup() {
    REPO_ROOT="/Users/jmagady/Dev/vsdd-factory/.worktrees/S-12.06"
    HOST_ABI="$REPO_ROOT/crates/hook-sdk/HOST_ABI.md"
}

# Helper: extract only the lines in the "## Context Injection Contract" section.
# Uses awk to capture from the section heading up to (but not including) the
# next "## " heading, or EOF.
# The sed strips any trailing "## " line that awk captures as the stop sentinel.
# Compatible with both GNU awk and BSD awk (macOS).
section_text() {
    awk '/^## Context Injection Contract/{found=1} found{if(/^## / && !/^## Context/) exit; print}' "$HOST_ABI"
}

# =============================================================================
# AC-001 | BC-4.12.002 PC9
# Section heading exists; RESOLVER_ABI_VERSION appears.
# =============================================================================

@test "AC-001a BC-4.12.002 PC9: HOST_ABI.md contains Context Injection Contract heading" {
    # Must be an actual markdown heading line (^## ), not just the text in a comment.
    run grep -c "^## Context Injection Contract\|^## Context-Injection Contract" "$HOST_ABI"
    [ "$status" -eq 0 ]
    [ "$output" -ge 1 ]
}

@test "AC-001b BC-4.12.002 PC9: HOST_ABI.md contains RESOLVER_ABI_VERSION" {
    run grep -c "RESOLVER_ABI_VERSION" "$HOST_ABI"
    [ "$status" -eq 0 ]
    [ "$output" -ge 1 ]
}

@test "AC-001c BC-4.12.002 PC9: RESOLVER_ABI_VERSION value is exactly 1 (not TBD, not v1)" {
    # The constant must carry the value "= 1" (or ": 1" or "1" in a code block).
    # BC-4.12.002 PC4 mandates the value is 1, not a placeholder.
    run grep -c "RESOLVER_ABI_VERSION.*=.*1\|RESOLVER_ABI_VERSION.*:.*1" "$HOST_ABI"
    [ "$status" -eq 0 ]
    [ "$output" -ge 1 ]
}

@test "AC-001d ordering: Context Injection Contract section appears after Block-message convention" {
    # The new section must follow the existing content — specifically after the
    # Block-message convention section.
    local line_block line_ctx
    line_block=$(grep -n "^## Block-message convention" "$HOST_ABI" | head -1 | cut -d: -f1)
    line_ctx=$(grep -n "^## Context Injection Contract\|^## Context-Injection Contract" "$HOST_ABI" | head -1 | cut -d: -f1)
    # Both headings must exist and ctx must follow block.
    [ -n "$line_block" ]
    [ -n "$line_ctx" ]
    [ "$line_ctx" -gt "$line_block" ]
}

# =============================================================================
# AC-002 | BC-4.12.001 PC1-5
# Resolver lifecycle: startup load, Module, mtime, per-dispatch Store.
# =============================================================================

@test "AC-002a BC-4.12.001 PC1: section documents startup-time loading of resolver modules" {
    # BC-4.12.001 PC1: loaded once at startup.
    # Scope to the section so we don't match existing content above the new section.
    local cnt
    cnt=$(section_text | grep -ic "startup\|at startup\|dispatcher startup")
    [ "$cnt" -ge 1 ]
}

@test "AC-002b BC-4.12.001 PC2-3: section documents mtime-based cache invalidation" {
    # BC-4.12.001 PC3 invariant: mtime keying is required.
    local cnt
    cnt=$(section_text | grep -ic "mtime")
    [ "$cnt" -ge 1 ]
}

@test "AC-002c BC-4.12.001 PC2: section documents per-dispatch fresh Store" {
    # BC-4.12.001 PC2: Store is per-invocation; module is shared.
    local cnt
    cnt=$(section_text | grep -ic "Store\|per.dispatch.*store\|fresh.*store")
    [ "$cnt" -ge 1 ]
}

@test "AC-002d BC-4.12.001 PC1: section documents Module compilation at startup (Module keyword)" {
    local cnt
    cnt=$(section_text | grep -ic "Module\|compile.*startup\|startup.*compile")
    [ "$cnt" -ge 1 ]
}

# =============================================================================
# AC-003 | BC-4.12.002 PC1-7
# ResolverInput / ResolverOutput types, RESOLVER_ABI_VERSION, resolve() signature.
# =============================================================================

@test "AC-003a BC-4.12.002 PC2: section documents ResolverInput type" {
    local cnt
    cnt=$(section_text | grep -c "ResolverInput")
    [ "$cnt" -ge 1 ]
}

@test "AC-003b BC-4.12.002 PC3: section documents ResolverOutput type" {
    local cnt
    cnt=$(section_text | grep -c "ResolverOutput")
    [ "$cnt" -ge 1 ]
}

@test "AC-003c BC-4.12.002 PC2: section documents ResolverInput field: event_type" {
    local cnt
    cnt=$(section_text | grep -c "event_type")
    [ "$cnt" -ge 1 ]
}

@test "AC-003d BC-4.12.002 PC2: section documents ResolverInput field: hook_event_name" {
    local cnt
    cnt=$(section_text | grep -c "hook_event_name")
    [ "$cnt" -ge 1 ]
}

@test "AC-003e BC-4.12.002 PC2: section documents ResolverInput field: project_dir" {
    local cnt
    cnt=$(section_text | grep -c "project_dir")
    [ "$cnt" -ge 1 ]
}

@test "AC-003f BC-4.12.002 PC2: section documents ResolverInput field: plugin_config" {
    local cnt
    cnt=$(section_text | grep -c "plugin_config")
    [ "$cnt" -ge 1 ]
}

@test "AC-003g BC-4.12.002 PC1: section documents the resolve() exported function signature" {
    # Exact signature: resolve(input_ptr: i32, input_len: i32) -> i64
    local cnt
    cnt=$(section_text | grep -c "resolve(input_ptr\|resolve(.*i32.*i32.*i64\|input_ptr.*i32")
    [ "$cnt" -ge 1 ]
}

# =============================================================================
# AC-004 | BC-4.12.003 INV1-4
# Capability model: path_allow, deny-by-default, CapabilityDenied, read-only.
# =============================================================================

@test "AC-004a BC-4.12.003 INV1: section documents path_allow capability declarations" {
    local cnt
    cnt=$(section_text | grep -c "path_allow")
    [ "$cnt" -ge 1 ]
}

@test "AC-004b BC-4.12.003 INV1: section documents deny-by-default capability model" {
    local cnt
    cnt=$(section_text | grep -ic "deny.by.default\|default.*deny\|empty.*path_allow\|no reads allowed")
    [ "$cnt" -ge 1 ]
}

@test "AC-004c BC-4.12.003 PC2: section documents CapabilityDenied return code" {
    local cnt
    cnt=$(section_text | grep -c "CapabilityDenied")
    [ "$cnt" -ge 1 ]
}

@test "AC-004d BC-4.12.003 INV2: section documents that write_file is absent from resolver linker" {
    # The read-only resolver model: write_file not available to resolvers.
    local cnt
    cnt=$(section_text | grep -ic "write_file.*not available\|absent.*resolver\|read.only\|resolvers.*read.only\|write_file.*not.*resolver\|resolvers.*cannot.*write")
    [ "$cnt" -ge 1 ]
}

@test "AC-004e BC-4.12.003 PC4: section documents that host::log is always available" {
    local cnt
    cnt=$(section_text | grep -ic "host::log\|log.*always\|always.*available.*log\|log.*no restriction\|without.*restriction")
    [ "$cnt" -ge 1 ]
}

# =============================================================================
# AC-005 | BC-4.12.004 PC1-5
# Error and crash isolation: resolver.error, resolver_name, error_kind,
# failed key absent, dispatch continues.
# =============================================================================

@test "AC-005a BC-4.12.004 PC2: section documents resolver.error event" {
    local cnt
    cnt=$(section_text | grep -c "resolver\.error")
    [ "$cnt" -ge 1 ]
}

@test "AC-005b BC-4.12.004 PC2: section documents resolver_name field in resolver.error" {
    local cnt
    cnt=$(section_text | grep -c "resolver_name")
    [ "$cnt" -ge 1 ]
}

@test "AC-005c BC-4.12.004 PC2: section documents error_kind field in resolver.error" {
    local cnt
    cnt=$(section_text | grep -c "error_kind")
    [ "$cnt" -ge 1 ]
}

@test "AC-005d BC-4.12.004 PC1+PC4: section documents that resolver crash does NOT propagate to dispatcher" {
    local cnt
    cnt=$(section_text | grep -ic "crash.*not.*propagate\|does not propagate\|MUST NOT propagate\|isolation\|panic\|trap")
    [ "$cnt" -ge 1 ]
}

@test "AC-005e BC-4.12.004 PC4: section documents that dispatch continues after resolver failure" {
    local cnt
    cnt=$(section_text | grep -ic "dispatch.*continues\|continues.*dispatch\|proceeds.*without\|hook.*proceeds\|dispatch proceeds")
    [ "$cnt" -ge 1 ]
}

# =============================================================================
# AC-006 | BC-4.12.005 PC1-8
# Merging contract: additive, whole-value replacement, value:None = absent,
# resolver.merge_collision, duplicate context_key = startup error.
# =============================================================================

@test "AC-006a BC-4.12.005 PC1: section documents additive overlay merge semantics" {
    local cnt
    cnt=$(section_text | grep -ic "additive\|overlay\|additive overlay")
    [ "$cnt" -ge 1 ]
}

@test "AC-006b BC-4.12.005 PC7: section documents whole-value replacement (no deep merge)" {
    local cnt
    cnt=$(section_text | grep -ic "whole.*replace\|whole-value\|no deep merge\|not.*deep merge\|replaces.*wholesale")
    [ "$cnt" -ge 1 ]
}

@test "AC-006c BC-4.12.005 PC2: section documents value:None means key is absent" {
    local cnt
    cnt=$(section_text | grep -ic "value.*None\|None.*absent\|key.*absent\|absent.*key\|not written.*plugin_config")
    [ "$cnt" -ge 1 ]
}

@test "AC-006d BC-4.12.005 PC5: section documents resolver.merge_collision event" {
    local cnt
    cnt=$(section_text | grep -c "merge_collision\|resolver\.merge_collision")
    [ "$cnt" -ge 1 ]
}

@test "AC-006e BC-4.12.005 PC6: section documents duplicate context_key is a startup error (EC-004)" {
    # EC-004 / BC-4.12.005 PC6: duplicate context_key at registry load = fail-loud startup error.
    local cnt
    cnt=$(section_text | grep -ic "duplicate.*context_key\|duplicate.*name\|startup error\|registry.load error\|load.*error.*duplicate")
    [ "$cnt" -ge 1 ]
}

# =============================================================================
# AC-007 | BC-1.13.001 PC3-6
# needs_context mechanism: declaration, dispatcher invokes only declared resolvers,
# zero-overhead path for empty, resolver.not_found for unknown.
# =============================================================================

@test "AC-007a BC-1.13.001 PC3: section documents needs_context field in hooks-registry.toml" {
    local cnt
    cnt=$(section_text | grep -c "needs_context")
    [ "$cnt" -ge 1 ]
}

@test "AC-007b BC-1.13.001 PC6: section documents resolver.not_found for unknown resolver names" {
    local cnt
    cnt=$(section_text | grep -c "resolver\.not_found\|not_found")
    [ "$cnt" -ge 1 ]
}

@test "AC-007c BC-1.13.001 PC3: section documents zero-overhead path when needs_context is empty" {
    local cnt
    cnt=$(section_text | grep -ic "zero.*overhead\|overhead.*zero\|zero.cost\|cost.*zero\|empty.*skip\|skip.*resolver.*invocation")
    [ "$cnt" -ge 1 ]
}

# =============================================================================
# EC-001 | BC-1.13.001 PC1 Critical Constraint
# Absent resolvers-registry.toml = zero resolvers, NOT a startup error.
# =============================================================================

@test "EC-001 BC-1.13.001 PC1: section documents absent registry file = zero resolvers (NOT error)" {
    # This is the PC1 Critical Constraint — the highest-priority backward-compat invariant.
    # The section MUST state that an absent resolvers-registry.toml is not a startup error.
    local cnt
    cnt=$(section_text | grep -ic "absent.*zero resolver\|zero resolver.*absent\|NOT.*startup error\|MUST NOT.*error\|not.*error.*absent\|absent.*not.*error")
    [ "$cnt" -ge 1 ]
}

# =============================================================================
# AC-008 | BC-1.13.001 INV1 + BC-4.12.002 PC9
# Factory-agnostic vocabulary: the new section must contain ZERO vsdd-factory
# domain-specific terms. This is the load-bearing test for the section's
# existence at the platform layer rather than in a vsdd-specific doc.
# =============================================================================

@test "AC-008 factory-agnostic: section contains no forbidden vsdd-factory domain terms" {
    # Extract just the Context Injection Contract section, then grep for forbidden terms.
    # grep -iE returns exit 0 if any match found; we assert it returns 1 (no matches).
    # Forbidden: wave, wave_context, WaveContext, wave-state, STATE.md, story_id, cycle_id,
    # story (as standalone word), cycle (as standalone word), article, companion, PR
    # (as standalone), review (as standalone in this context).
    #
    # Note: "wave" as a substring of other words (e.g., "wasmtime") would false-positive,
    # so we use word-boundary-aware patterns with \b or precise alternations.
    local section
    section="$(section_text)"

    # If the section is empty (step 4 not done), this grep returns non-zero because
    # there is nothing to search — we still want a clear FAIL, so we check section
    # is non-empty first.  An empty section is itself a failure (AC-001 catches it).
    # Here we run the forbidden-terms grep on whatever text exists; if the section
    # is non-empty and contains forbidden terms, the test fails.  If it is empty,
    # the section_text() produces no output and the forbidden-terms grep returns 1
    # (no match), which would incorrectly pass.  Guard with an explicit non-empty check.
    [ -n "$section" ]

    # Now assert NO forbidden terms appear.
    echo "$section" | grep -qiE '\bwave\b|\bwave_context\b|\bWaveContext\b|\bwave-state\b|STATE\.md|\bstory_id\b|\bcycle_id\b|\bstory\b|\bcycle\b'
    # grep found a match — that is a failure. We invert: the test passes only if
    # the grep exits non-zero (no forbidden terms found).
    local rc=$?
    [ "$rc" -ne 0 ]
}

# =============================================================================
# AC-009 | BC-4.12.001 + BC-4.12.002
# ADR-018 and all 6 BCs cross-referenced explicitly by canonical ID.
# =============================================================================

@test "AC-009a cross-ref: section references ADR-018" {
    local cnt
    cnt=$(section_text | grep -c "ADR-018")
    [ "$cnt" -ge 1 ]
}

@test "AC-009b cross-ref: section references BC-1.13.001" {
    local cnt
    cnt=$(section_text | grep -c "BC-1\.13\.001")
    [ "$cnt" -ge 1 ]
}

@test "AC-009c cross-ref: section references BC-4.12.001" {
    local cnt
    cnt=$(section_text | grep -c "BC-4\.12\.001")
    [ "$cnt" -ge 1 ]
}

@test "AC-009d cross-ref: section references BC-4.12.002" {
    local cnt
    cnt=$(section_text | grep -c "BC-4\.12\.002")
    [ "$cnt" -ge 1 ]
}

@test "AC-009e cross-ref: section references BC-4.12.003" {
    local cnt
    cnt=$(section_text | grep -c "BC-4\.12\.003")
    [ "$cnt" -ge 1 ]
}

@test "AC-009f cross-ref: section references BC-4.12.004" {
    local cnt
    cnt=$(section_text | grep -c "BC-4\.12\.004")
    [ "$cnt" -ge 1 ]
}

@test "AC-009g cross-ref: section references BC-4.12.005" {
    local cnt
    cnt=$(section_text | grep -c "BC-4\.12\.005")
    [ "$cnt" -ge 1 ]
}

# =============================================================================
# AC-010 | BC-4.12.002 PC5
# #[resolver] macro, resolve_impl function name, resolver-authoring feature flag.
# =============================================================================

@test "AC-010a BC-4.12.002 PC5: section documents the #[resolver] proc-macro" {
    local cnt
    cnt=$(section_text | grep -c "#\[resolver\]")
    [ "$cnt" -ge 1 ]
}

@test "AC-010b BC-4.12.002 PC5: section documents resolve_impl as the required user function name" {
    local cnt
    cnt=$(section_text | grep -c "resolve_impl")
    [ "$cnt" -ge 1 ]
}

@test "AC-010c BC-4.12.002 PC8: section documents resolver-authoring feature flag" {
    local cnt
    cnt=$(section_text | grep -c "resolver-authoring")
    [ "$cnt" -ge 1 ]
}

# =============================================================================
# Section sanity: length is reasonable (100–400 lines).
# Catches absurd over/under (empty placeholder vs. runaway prose).
# =============================================================================

@test "sanity: Context Injection Contract section is between 100 and 400 lines" {
    local line_count
    line_count=$(section_text | wc -l | tr -d ' ')
    [ "$line_count" -ge 100 ]
    [ "$line_count" -le 400 ]
}

# =============================================================================
# Additional: resolvers-registry.toml documented as a separate file
# from hooks-registry.toml (BC-1.13.001 INV7 / ADR-018 OD-2).
# =============================================================================

@test "resolvers-registry.toml: section documents separate registry file from hooks-registry.toml" {
    local cnt
    cnt=$(section_text | grep -c "resolvers-registry\.toml")
    [ "$cnt" -ge 1 ]
}

@test "resolvers-registry.toml: section states registry is DISTINCT from hooks-registry.toml" {
    # OD-2: separate file with different schema and lifecycle role.
    local cnt
    cnt=$(section_text | grep -ic "distinct\|separate.*file\|different.*file\|separate.*registry\|distinct.*file\|not.*hooks-registry")
    [ "$cnt" -ge 1 ]
}
