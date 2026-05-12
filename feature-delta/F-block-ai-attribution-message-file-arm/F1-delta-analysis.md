---
document_type: feature-delta-analysis
level: F1
feature_id: F-block-ai-attribution-message-file-arm
version: "1.0"
status: draft
producer: architect
timestamp: 2026-05-12T00:00:00Z
phase: F1
intent: enhancement
feature_type: infrastructure
scope: standard
routing: standard-F1-F7
subsystems: ["SS-04", "SS-07"]
---

# F1 Delta Analysis: block-ai-attribution Message-File Arm

**Feature:** Extend the `block-ai-attribution` WASM plugin to close two commit-path
bypass vectors: (Option B) PreToolUse `-F <path>` file-arm and (Option C) PostToolUse
retroactive HEAD verification.

**Gap confirmed:** The existing WASM plugin only inspects `tool_input.command`. A real
commit with `Co-Authored-By: Claude Sonnet 4.6 <noreply@anthropic.com>` passed through
when the attribution text lived in a file rather than the command string.

---

## 1. Intent and Scope Classification

| Dimension | Classification | Rationale |
|-----------|---------------|-----------|
| Intent | `enhancement` | Extends an existing shipped plugin; does not add new product capability |
| Feature type | `infrastructure` | WASM plugin internals + registry capability declarations |
| Scope | `standard` | Two new BCs, one new VP, new PostToolUse event registration, new capability declarations — not trivial |
| Quick-dev eligible? | No | Two distinct behavioral arms, new capability surface (`exec_subprocess`), PostToolUse event registration |

---

## 2. Impact Boundary

### 2a. Files and Crates

| File (absolute path) | Change Type | Rationale |
|----------------------|------------|-----------|
| `/Users/jmagady/Dev/vsdd-factory/crates/hook-plugins/block-ai-attribution/src/lib.rs` | MODIFIED | Add `-F` path parsing arm to `on_hook_logic`; add `on_post_tool_use_logic` for PostToolUse arm; `detect_attribution` already pure — reused as-is |
| `/Users/jmagady/Dev/vsdd-factory/crates/hook-plugins/block-ai-attribution/src/main.rs` | MODIFIED | Wire PostToolUse dispatch path into the WASI entry point |
| `/Users/jmagady/Dev/vsdd-factory/plugins/vsdd-factory/hooks-registry.toml` | MODIFIED | Add PostToolUse entry for block-ai-attribution (exec_subprocess capability); add read_file capability to existing PreToolUse entry with path_allow |
| `/Users/jmagady/Dev/vsdd-factory/crates/hook-plugins/block-ai-attribution/Cargo.toml` | MODIFIED | Confirm vsdd-hook-sdk version pin is current; no new external deps expected |

### 2b. Spec Files Created or Modified

| Spec File (absolute path) | Change Type | Rationale |
|---------------------------|------------|-----------|
| `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-07/BC-7.03.094.md` | NEW | Option C — PostToolUse retroactive HEAD verification behavior |
| `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-07/BC-7.03.095.md` | NEW | Option B — PreToolUse `-F <path>` file-read arm |
| `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-07/BC-7.03.001.md` | MODIFIED (version bump only) | Registry binding description changes: PostToolUse entry added, new capabilities declared — identity BC must reflect updated registry shape |
| `/Users/jmagady/Dev/vsdd-factory/.factory/specs/verification-properties/VP-080.md` | NEW | PostToolUse detect_attribution on HEAD commit body; proptest-based |
| `/Users/jmagady/Dev/vsdd-factory/.factory/specs/architecture/SS-07-hook-bash.md` | MODIFIED | Update modules table: block-ai-attribution now spans PreToolUse + PostToolUse; document new capability registrations |
| `/Users/jmagady/Dev/vsdd-factory/.factory/specs/architecture/SS-04-plugin-ecosystem.md` | MODIFIED | block-ai-attribution crate description updated to note exec_subprocess + read_file capability usage |

### 2c. Indexes (state-manager activity, F2/F3)

| Index | Expected Change |
|-------|----------------|
| BC-INDEX.md | +2 rows (BC-7.03.094, BC-7.03.095); SS-07 count 196 → 198; Total 1947 → 1949 |
| VP-INDEX.md | +1 row (VP-080); total_vps 79 → 80 |
| STORY-INDEX.md | +2 story rows (S-16.01 and S-16.02 under new epic E-16); story_count 93 → 95 |
| ARCH-INDEX.md | SS-07 BC count cite refresh (196 → 198); Total BC count refresh (1947 → 1949) |

### 2d. DEPENDENT (unchanged, in regression blast radius)

| File | Why Listed |
|------|-----------|
| `/Users/jmagady/Dev/vsdd-factory/crates/hook-sdk/src/result.rs` | VP-038 anchor; HookResult variants used by both new arms — exit code contract must keep passing |
| `/Users/jmagady/Dev/vsdd-factory/crates/factory-dispatcher/src/host/exec_subprocess.rs` | PostToolUse arm calls this — no changes needed; capability gating verified |
| `/Users/jmagady/Dev/vsdd-factory/crates/factory-dispatcher/src/host/read_file.rs` | PreToolUse -F arm calls this — no changes needed |
| Bats tests for block-ai-attribution (`plugins/vsdd-factory/tests/block-ai-attribution.bats`) | Must be extended; existing vectors must pass unchanged |

---

## 3. New BC Proposal

The BC-7.03 space currently has 93 files (BC-7.03.001 through BC-7.03.093). The next
available IDs are BC-7.03.094 and BC-7.03.095.

**ID verification:** Filesystem scan confirms BC-7.03.093 is the current ceiling
(verify-git-push identity confirmation, BC-AUDIT-1175). No BC-7.03.094 or BC-7.03.095
exist.

### BC-7.03.094 — PostToolUse retroactive HEAD attribution verification (Option C)

| Field | Value |
|-------|-------|
| Proposed ID | BC-7.03.094 |
| Title | block-ai-attribution: PostToolUse retroactive HEAD commit message verification |
| Subsystem | SS-07 |
| Capability | CAP-008 |
| Lifecycle version | v1.0.0-rc.17 |
| Hook path | `crates/hook-plugins/block-ai-attribution/src/lib.rs` (PostToolUse arm) |
| Preconditions | PostToolUse on Bash tool; `tool_response` indicates successful `git commit` (exit 0) |
| Postconditions | (1) `detect_attribution` on `git log -1 --format=%B HEAD` stdout returns attribution → `HookResult::block_with_fix` with remediation pointing to `git reset --soft HEAD~1`; (2) Clean HEAD → `HookResult::Continue`; (3) Subprocess failure (timeout, non-zero exit, empty output, capability denied) → `HookResult::Continue` |
| Invariants | (1) Never false-positive: any subprocess error yields Continue; (2) Lookalike commands `git commit-tree` and `git commit-graph` MUST NOT trigger (gate on presence of `git commit` in tool_input.command AND successful tool_response); (3) Subprocess timeout ≤ 1000ms (Class A per ADR-020; see OQ-F1-001) |

**Does BC-7.03.002 need a `modified:` bump?** No. BC-7.03.002 ("substring gate on
`git commit`") contracts the PreToolUse command-string filter path in the WASM plugin and
its bash predecessor. The PostToolUse arm is an additive path at a different event type.
The substring gate semantics are unchanged. BC-7.03.002 does not need modification.

### BC-7.03.095 — PreToolUse -F file-read arm (Option B)

| Field | Value |
|-------|-------|
| Proposed ID | BC-7.03.095 |
| Title | block-ai-attribution: PreToolUse `-F <path>` file-read arm |
| Subsystem | SS-07 |
| Capability | CAP-008 |
| Lifecycle version | v1.0.0-rc.17 |
| Hook path | `crates/hook-plugins/block-ai-attribution/src/lib.rs` (PreToolUse -F arm) |
| Preconditions | PreToolUse on Bash tool; command contains `git commit`; command contains `-F <path>`, `--file=<path>`, or `--file <path>` flag |
| Postconditions | (1) File content contains attribution pattern → `HookResult::Block`; (2) File content clean → `HookResult::Continue`; (3) Read failure (capability denied, not found, oversize >65536 bytes) → `HookResult::Continue` |
| Invariants | (1) Fallback to Continue on any read failure — never false-positive on IO error; (2) Path extraction handles all three flag forms: `-F path`, `--file=path`, `--file path`; (3) `path_allow` is narrow scope: `/tmp/**`, `/var/folders/**`, `{project_root}/**`, `**/.git/COMMIT_EDITMSG` (see OQ-F1-002 for human confirmation) |

**Does BC-7.03.002 need a `modified:` bump?** No. BC-7.03.095 is a sibling arm that
activates when `-F` is present. The command-string gate (BC-7.03.002) fires first and
passes; BC-7.03.095 then activates the file-read path. Independent code paths; no
semantic overlap.

---

## 4. VP Proposal

### Does VP-038 need extension?

VP-038 is scoped to SS-02 SDK HookResult exit code stability (unit tests on
`hook-sdk/src/result.rs`). It verifies Continue=0, Block=2, Error=1. This is a
necessary precondition for all hook arms but does not verify detection logic. VP-038
must NOT be modified — it is a frozen ABI stability contract.

### VP-080 — New VP for PostToolUse detection on HEAD content

| Field | Value |
|-------|-------|
| Proposed ID | VP-080 |
| Title | block-ai-attribution PostToolUse arm: detect_attribution correctly identifies all TV-001..011 patterns in git log HEAD output |
| Subsystem | SS-07 (behavioral contract); SS-04 (implementation) |
| Scope | Verify that `detect_attribution(&str) -> Option<Attribution>` correctly detects all 11 canonical test vectors (TV-001..011) when applied to `git log -1 --format=%B HEAD` stdout content. Also verifies the fallback invariant: empty string, whitespace-only, and error-signaling inputs all yield `None` (Continue). |
| Proof method | proptest (property-based) + unit-test |
| Feasibility | Feasible: `detect_attribution` is a pure function with no I/O (`fn detect_attribution(command: &str) -> Option<Attribution>`). Property tests can generate arbitrary strings containing attribution patterns and verify detection. The subprocess call is effectful but the detection function is pure-core and independently testable. Proof harness skeleton: proptest strategy over TV-001..011 pattern strings embedded in arbitrary surrounding text. |
| Anchor BC | BC-7.03.094 postcondition 1 |
| Stories | S-16.01 (PostToolUse arm) |

### VP for Option B (-F arm)?

The `-F` arm introduces no new detection logic — it pipes file contents through the
existing `detect_attribution` pure function already covered by VP-080. The novel behavior
is the fallback-to-Continue on read failure, which is an edge-case test rather than a
formal property. A dedicated VP for the -F arm is not warranted at this time. Cover
Option B in S-16.02 acceptance criteria and integration tests. Revisit if F5 adversary
identifies a formal property gap.

---

## 5. Architecture Impact

### SS-07-hook-bash.md (MODIFIED)

The Modules table lists `block-ai-attribution.sh` and `block-ai-attribution.wasm` as
"PreToolUse gate hooks." After this feature, the WASM plugin is both a PreToolUse gate
and a PostToolUse retroactive verifier. Required changes:
- Update the module table row for block-ai-attribution to note both event types
- Add a note to the Public Interface section about the new PostToolUse registry entry
  and the `exec_subprocess` + `read_file` capability blocks

Change scope: append-style amendments to existing tables; no structural changes.

### SS-04-plugin-ecosystem.md (MODIFIED)

block-ai-attribution is listed as a WASM plugin crate in the Plugin Ecosystem. The
capability surface changes from none (pure command-string scan, no host fn calls) to two
host functions (exec_subprocess for PostToolUse arm, read_file for PreToolUse -F arm).
Required changes:
- Single paragraph update to the block-ai-attribution crate description

Change scope: single paragraph; no structural changes.

### ADR assessment — no new ADR required

Both host functions and capability gating are established patterns:
- ADR-002 (WASM plugin ABI) governs all capability-gated host function usage
- ADR-014 (Tier-2 native WASM migration) — block-ai-attribution is already a Tier 2
  native WASM plugin; no migration considerations

One judgment call that could warrant a Decision Log entry (not a full ADR):

**read_file path_allow scope for the -F arm:**
- Narrow: `["/tmp/**", "/var/folders/**", "{project_root}/**", "**/.git/COMMIT_EDITMSG"]`
- Broad: `["**/*"]` (bounded by max_bytes=65536)

**Recommendation: narrow allowlist.** The block-ai-attribution plugin has `on_error =
"block"` — a security-relevant plugin. It has no legitimate reason to read arbitrary
filesystem paths. The narrow allowlist covers all realistic commit message file locations
(temp files from editors, .git/COMMIT_EDITMSG). Rationale should be documented in
BC-7.03.095 invariants and the registry entry comment. An ADR is not warranted because
this is a configuration choice within the established capability gating pattern (ADR-002).

---

## 6. Epic Placement

### E-3 closed — assessment

E-3 ("WASM Port — High-Value Hooks") shipped its four stories at milestone `1.0.0-rc.1`.
The project is now at rc.16. Adding stories to E-3 would reopen a closed epic and
break POLICY 1 (append-only numbering / forward-only epic lifecycle). The E-3 framing
("WASM Port") does not describe this work — the plugin is already ported.

### Options evaluated

| Option | Assessment |
|--------|-----------|
| Extend E-3 with addendum | Rejected: E-3 closed at rc.1 milestone; reopening violates POLICY 1 forward-only lifecycle |
| Place under E-11 (Tier-3 native WASM migration) | Rejected: E-11 is about migrating additional bash hooks to native WASM; block-ai-attribution is already native |
| Place under E-14 (Engine Discipline pass-2) | Rejected: E-14 is about governance/discipline process gaps, not plugin behavior extensions |
| Open new feature epic E-16 | Recommended |

### Recommendation: Open E-16 — Hook Plugin Capability Extensions

**Justification per STORY-INDEX policy and append-only numbering:**
1. E-15 is the current ceiling epic. Next available ID is E-16. POLICY 1 (append-only)
   requires allocation of the next sequential ID.
2. The conceptual scope — extending shipped WASM plugins with new host function
   capabilities — is coherent and likely to attract additional stories from other plugins.
   E-16 is a better long-term home than a one-off.
3. The two stories share the same parent crate, the same capability surface decisions,
   and will share test infrastructure. Grouping in one epic reduces story-writer overhead.

**Proposed epic:** E-16 — Hook Plugin Capability Extensions
**Milestone:** v1.0.0-rc.17
**Subsystems:** SS-04, SS-07
**Story IDs (allocated by F3 story-writer):** S-16.01 (Option C), S-16.02 (Option B)

---

## 7. Story Scoping

### Recommended split: two stories, C-first per human's recommended sequencing

| Story ID | Title | Option | Estimated Points | Depends On | Sequencing Rationale |
|----------|-------|--------|-----------------|-----------|---------------------|
| S-16.01 | block-ai-attribution: PostToolUse retroactive HEAD verification | C | 5 | S-3.03 (merged; WASM plugin exists) | Path-agnostic safety net — catches -m, heredoc, -F, $EDITOR paths because all roads lead to HEAD. Higher risk-reduction per point. |
| S-16.02 | block-ai-attribution: PreToolUse -F file-read arm | B | 3 | S-16.01 | Faster-feedback optimization layered on top of the safety net. Could run in parallel with S-16.01 architecturally, but C-before-B ensures S-16.01's pure test infrastructure for `detect_attribution` is available when S-16.02 is implemented. |

**Point estimates rationale:**

S-16.01 (5 points): New PostToolUse event registration in hooks-registry.toml, new
exec_subprocess capability declaration, new `on_post_tool_use_logic` function in lib.rs,
new main.rs dispatch path, new bats integration tests for PostToolUse path, new VP-080
proptest harness, BC-7.03.094, and the failure-mode test matrix (subprocess failure →
Continue). Moderate complexity due to exec_subprocess path and multiple failure modes.

S-16.02 (3 points): Path-parsing logic for three `-F` flag forms, vsdd::read_file call,
read_file capability declaration, narrow path_allow configuration, bats tests. Simpler
than S-16.01 because `detect_attribution` is reused unchanged and the failure semantics
(read error → Continue) are straightforward.

**Wave suggestion:** S-16.01 as Wave N (standalone story), S-16.02 as Wave N+1
(single dependent story). Both can be in the same sprint but should be sequential to
enable S-16.01's test infrastructure to be available for S-16.02's bats extension.

---

## 8. Regression Risk Assessment

### Risk level: MEDIUM

Primary regression risk: the PostToolUse arm adds a new code path that fires after every
successful `git commit`. If the fallback-to-Continue guard has a bug, the arm could
produce a spurious Block on a legitimate commit. The `-F` arm regression risk is LOW
because its fallback semantics are strong (any read failure → Continue).

### Concrete regression checklist

| Test Case | BC(s) Covered | Risk Level if Missing |
|-----------|--------------|----------------------|
| `git commit -m "clean message"` → Continue (PreToolUse, existing) | BC-7.03.002, BC-7.03.003, BC-7.03.004 | CRITICAL |
| `git commit -m "Co-Authored-By: Claude..."` → Block (PreToolUse, existing) | BC-7.03.003 | CRITICAL |
| `git commit -m "Generated with Claude Code"` → Block (PreToolUse, existing) | BC-7.03.004 | CRITICAL |
| `git commit -m "noreply@anthropic.com"` → Block (PreToolUse, existing) | BC-7.03.004 | CRITICAL |
| `git commit -m "noreply@openai.com"` → Block (PreToolUse, existing) | BC-7.03.004 | HIGH |
| Non-git bash command → Continue (PreToolUse, existing) | BC-7.03.002 | HIGH |
| `git commit --allow-empty` with attribution in body → Block (existing) | S-3.03 EC-002 | HIGH |
| PostToolUse on clean HEAD → Continue (S-16.01 happy path) | BC-7.03.094 PC-2 | CRITICAL |
| PostToolUse on HEAD containing any TV-001..011 pattern → Block | BC-7.03.094 PC-1 | CRITICAL |
| PostToolUse on HEAD: subprocess failure (git absent, timeout) → Continue | BC-7.03.094 INV-1 | CRITICAL — false-positive prevention |
| PostToolUse on HEAD: subprocess returns empty stdout → Continue | BC-7.03.094 INV-1 | HIGH |
| `git commit-tree` PostToolUse → Continue (lookalike non-trigger) | BC-7.03.094 INV-2 | HIGH |
| `git commit-graph` PostToolUse → Continue (lookalike non-trigger) | BC-7.03.094 INV-2 | HIGH |
| `git commit -F /tmp/msg` with attribution in file → Block (S-16.02) | BC-7.03.095 PC-1 | CRITICAL |
| `git commit --file=/tmp/msg` with attribution → Block (S-16.02) | BC-7.03.095 PC-1 | HIGH |
| `git commit --file /tmp/msg` with attribution → Block (S-16.02) | BC-7.03.095 PC-1 | HIGH |
| `git commit -F /nonexistent` → Continue (S-16.02, missing file) | BC-7.03.095 INV-1 | CRITICAL |
| `git commit -F /tmp/oversized_msg` (>65536 bytes) → Continue (S-16.02) | BC-7.03.095 INV-1 | HIGH |
| Full BC-7.03.001..005 bats suite: all vectors green | BC-7.03.001..005 | CRITICAL |

### Latency note

The PostToolUse arm adds one `exec_subprocess("git", ["log", "-1", "--format=%B", "HEAD"])` call per successful `git commit`. Typical wall-clock time is <50ms on local repos. The recommended 1000ms timeout (OQ-F1-001) keeps the worst-case PostToolUse latency within ADR-020 Class A bounds (p95 ≤ 1500ms for binary-spawn model). PostToolUse is non-blocking for the original tool invocation — a delayed retroactive block does not prevent the commit from landing, but the `git reset --soft HEAD~1` remediation is actionable for the agent.

---

## 9. Open Questions (Human Input Required Before F2/F3)

| ID | Question | Impact | Default if No Answer |
|----|----------|--------|---------------------|
| OQ-F1-001 | PostToolUse subprocess timeout: 1000ms (Class A, ADR-020 p95 ≤ 1500ms) or 2000ms (more headroom for slow git on large repos)? | BC-7.03.094 INV-3 wording; latency note in SS-07 amendment | **1000ms** (Class A compliant; `git log -1` is fast in practice) |
| OQ-F1-002 | `read_file` path_allow scope for -F arm: narrow (`["/tmp/**", "/var/folders/**", "{project_root}/**", "**/.git/COMMIT_EDITMSG"]`) or broad (`["**/*"]`)? | BC-7.03.095 INV-3; registry entry capability declaration | **Narrow** (recommended; block-ai-attribution has on_error=block) |
| OQ-F1-003 | PostToolUse gate: fire on all PostToolUse Bash events containing `git commit` command, or only on successful tool_response (exit_code == 0)? | When subprocess fires; whether failed commits are retroactively checked | **Gate on success (exit_code == 0)**: avoids subprocess overhead on failed commits and prevents double-blocking (PreToolUse already blocked) |
| OQ-F1-004 | Release version target for BC-7.03.094, BC-7.03.095, VP-080: rc.17 or a later milestone? | `introduced:` field in BC/VP frontmatter | **v1.0.0-rc.17** (next release after current rc.16) |

---

## 10. Convergence Hook Implications

F4 implementation for S-16.01 and S-16.02 requires the full per-story-delivery loop
(test-writer → implementer → adversary → pr-manager → devops-engineer).

**F5 adversary pass focus areas:**

1. **exec_subprocess attack surface (S-16.01):** The adversary must probe whether the
   `binary_allow = ["git"]` restriction can be bypassed. Verify `shell_bypass_acknowledged
   = false` is set in the registry entry. Construct a test case where `git log -1` returns
   0-byte stdout (empty repo) — must yield Continue, not Block or Error.

2. **False-positive coverage (S-16.01):** Adversary should simulate: subprocess timeout
   fires during `git log -1`; subprocess returns non-zero exit; `git` binary not in PATH;
   WASM capability denied for exec_subprocess. All four cases must yield Continue.

3. **read_file capability isolation (S-16.02):** Adversary should verify the narrow
   path_allow blocks reads outside the allowlist. Test: `git commit -F /etc/passwd` →
   read_file capability denied → Continue (not Block, not Error).

4. **Purity invariant for VP-080 (both stories):** Adversary should confirm that
   `detect_attribution` remains genuinely pure after S-16.01 changes — no new I/O
   introduced into the pure function body. The proptest strategy is only valid if the
   function has no side effects.

5. **Residual gap (advisory, not a blocker):** After Option C lands, a theoretically
   possible residual bypass exists: an agent could write attribution to `.git/config`
   (user.name/user.email set to AI identity) rather than to the commit message. Neither
   arm catches this. This is out of scope for this feature. Note it in BC-7.03.094 as
   a known residual gap for future consideration.

---

## 11. Files NOT Changed (Regression Baseline)

| File | Why Stable |
|------|-----------|
| `/Users/jmagady/Dev/vsdd-factory/crates/hook-sdk/src/result.rs` | VP-038 ABI contract; frozen |
| `/Users/jmagady/Dev/vsdd-factory/crates/factory-dispatcher/src/host/exec_subprocess.rs` | Host function; no changes needed |
| `/Users/jmagady/Dev/vsdd-factory/crates/factory-dispatcher/src/host/read_file.rs` | Host function; no changes needed |
| `/Users/jmagady/Dev/vsdd-factory/.factory/specs/behavioral-contracts/ss-07/BC-7.03.002.md` through BC-7.03.005.md | Attribution detection BCs; content must not change |
| `/Users/jmagady/Dev/vsdd-factory/plugins/vsdd-factory/hooks/block-ai-attribution.sh` | Bash predecessor; superseded; must not be modified |
| All other WASM plugins in `crates/hook-plugins/` | Zero blast radius |
| `/Users/jmagady/Dev/vsdd-factory/.factory/specs/verification-properties/VP-038.md` | ABI stability VP; must not be modified |

---

## 12. Summary of Key Decisions

| Decision | Rationale |
|----------|-----------|
| C before B (PostToolUse before PreToolUse -F) | C is path-agnostic and closes the largest residual risk surface; B is an optimization |
| Two stories: S-16.01 (5pts) and S-16.02 (3pts) | Clean behavioral separation; C-first sequencing per human recommendation |
| New epic E-16 "Hook Plugin Capability Extensions" | E-3 closed (milestone rc.1); E-16 is next available ID; semantically coherent home for future plugin capability extensions |
| BC-7.03.094 and BC-7.03.095 | Next available IDs after BC-7.03.093; no collision; verified by filesystem scan |
| BC-7.03.002 does NOT get a `modified:` bump | PostToolUse arm is a different event type; command-string gate semantics unchanged |
| VP-038 not extended | ABI stability contract; must remain frozen |
| VP-080 (proptest, PostToolUse detection logic) | `detect_attribution` is pure; property tests are the right tool |
| No VP for -F arm separately | Option B reuses existing detection logic; no new formal property |
| No new ADR | exec_subprocess and read_file are established patterns (ADR-002, ADR-014) |
| read_file path_allow: narrow scope recommended | Security-relevant plugin with on_error=block; document in BC-7.03.095 invariants |
| SS-07-hook-bash.md and SS-04-plugin-ecosystem.md: minor amendments | Registry shape changes must be reflected in arch section files |
