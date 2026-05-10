#!/usr/bin/env bats
# resolver-integration.bats — End-to-end integration test for the
# WaveContextResolver → validate-per-story-adversary-convergence hook pipeline.
#
# Closes F-P2-001: proves the convergence hook is operationally effective in
# production by exercising the complete producer (WaveContextResolver) →
# consumer (convergence hook) pipeline.
#
# AC-008: unconverged story → dispatcher exits 2 (Block)
# AC-009: all converged → dispatcher exits 0 (Continue)
#
# These tests are RED (skip'd at Step 1). Step 2 (test-writer) fills in the
# test body. Step 3 (implementer) makes them GREEN.
#
# BC traces:
#   BC-1.13.001 postcondition 4 — needs_context = ["wave_context"] triggers
#     WaveContextResolver injection before each hook dispatch.
#   BC-4.10.001 postcondition 1 — hook reads story list from wave_context.stories.
#   BC-4.10.001 postcondition 2 — unconverged story → Block.
#   F-P2-001, F-P2-008 — root cause: hook never received story list; fixed by
#     consuming wave_context.stories produced by WaveContextResolver (S-12.07).

@test "F-P2-001 closure: unconverged story → dispatcher exits 2 (Block)" {
    skip "todo: implement in Step 2 (test-writer) / Step 3 (implementer)"
    # Step 2 will seed:
    #   .factory/STATE.md (current_cycle = "test-cycle-001")
    #   .factory/wave-state.yaml (active wave, stories: ["S-FAKE-001", "S-FAKE-002"])
    #   .factory/cycles/test-cycle-001/S-FAKE-001/adversary-convergence-state.json
    #     (unconverged: passes_clean: 1)
    #   .factory/cycles/test-cycle-001/S-FAKE-002/adversary-convergence-state.json
    #     (converged: passes_clean: 3, last_classification: "NITPICK_ONLY")
    # Then invoke the dispatcher with a synthetic SubagentStop event
    # (agent_type = "wave-gate-dispatch") and assert exit code 2 (Block)
    # with code CONVERGENCE_PASSES_INSUFFICIENT in the output.
}

@test "F-P2-001 closure: all converged → dispatcher exits 0 (Continue)" {
    skip "todo: implement in Step 2 (test-writer) / Step 3 (implementer)"
    # Step 2 will seed:
    #   .factory/STATE.md (current_cycle = "test-cycle-001")
    #   .factory/wave-state.yaml (active wave, stories: ["S-FAKE-001", "S-FAKE-002"])
    #   .factory/cycles/test-cycle-001/S-FAKE-001/adversary-convergence-state.json
    #     (converged: passes_clean: 3, last_classification: "NITPICK_ONLY")
    #   .factory/cycles/test-cycle-001/S-FAKE-002/adversary-convergence-state.json
    #     (converged: passes_clean: 3, last_classification: "NITPICK_ONLY")
    # Then invoke the dispatcher with a synthetic SubagentStop event
    # (agent_type = "wave-gate-dispatch") and assert exit code 0 (Continue).
}
