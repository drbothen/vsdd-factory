---
document_type: architecture-decision-record
level: L3
adr_id: ADR-025
version: "1.24"
modified:
  - "2026-06-11 (v1.3)"
  - "2026-06-11 (v1.4)"
  - "2026-06-11 (v1.5)"
  - "2026-06-11 (v1.6)"
  - "2026-07-06 (v1.7)"
  - "2026-07-06 (v1.8)"
  - "2026-07-06 (v1.9)"
  - "2026-07-07 (v1.10)"
  - "2026-07-09 (v1.11)"
  - "2026-07-09 (v1.12)"
  - "2026-07-09 (v1.13)"
  - "2026-07-10 (v1.14)"
  - "2026-07-10 (v1.15)"
  - "2026-07-15 (v1.16)"
  - "2026-07-16 (v1.17)"
  - "2026-07-16 (v1.18)"
  - "2026-07-17 (v1.19)"
  - "2026-07-20 (v1.20)"
  - "2026-07-20 (v1.21)"
  - "2026-07-20 (v1.22)"
  - "2026-07-20 (v1.23)"
  - "2026-07-20 (v1.24)"
status: accepted
date: 2026-06-10
producer: architect
timestamp: 2026-06-10T00:00:00Z
amended: 2026-06-11T00:00:00Z
amendment_reason: "2026-07-20 (v1.24) — F-ADR032-P7R-002 §12.2 supersedes-in-part annotation correction (architect): §12.2 ADR-032 supersedes-in-part annotation (v1.20) corrected — header bumped (v1.20)→(v1.24); payload-neutral definition changed from single-field (sets timestamp: at column 0 only) to two-field (neither timestamp: nor factory_lock: in any new_string; a factory_lock-only Edit skips Steps 4–6 but still runs Step 7); "Continue immediately" → "Continue after skipping the timestamp and lock-expiry checks". ADR-032 amended to v1.12 in lock-step (§Source/Origin + related_adrs cites updated v1.23→v1.24). ARCH-INDEX v3.21→v3.22. Closes F-ADR032-P7R-002 MEDIUM. [Prior: 2026-07-20 (v1.23) — F-ADR032-P6R-004 §12.3 Continue-phrasing parity correction (architect): §12.3 ADR-032 supersedes-in-part annotation (v1.22) first operative sentence corrected — "the guard returns `Continue` immediately" changed to "the guard returns `Continue` after skipping the timestamp and lock-expiry checks". This matches the phrasing corrected in ADR-032 §Source/Origin at v1.9 (F-ADR032-P7-003); the ADR-025 annotation had not been updated in lock-step at that time. Annotation header bumped (v1.22)→(v1.23). ADR-032 §Source/Origin reference updated from v1.22 to v1.23 in lock-step. Closes F-ADR032-P6R-004 LOW. [Prior: 2026-07-20 (v1.22) — F-ADR032-P3-002 §12.3 annotation first-sentence correction (architect): §12.3 ADR-032 supersedes-in-part annotation v1.21 first sentence narrowed. The v1.21 text stated that the guard short-circuits for payloads not setting 'timestamp:' (payload-neutral Edit). This is incorrect: a payload that sets 'factory_lock:' but not 'timestamp:' is NOT payload-neutral — ADR-032 Decision 3 runs Step 7 for such payloads. Correct payload-neutral definition: neither 'timestamp:' NOR 'factory_lock:' set in any new_string. §12.3 annotation header bumped (v1.21)→(v1.22) and first sentence corrected to the two-field condition. ADR-032 §Source/Origin cross-ref updated in lock-step (ADR-032 v1.4). Closes F-ADR032-P3-002 MEDIUM. [Prior: 2026-07-20 (v1.21) — F-ADR032-P2-002 §12.3 annotation scope-split (architect): §12.3 ADR-032 supersedes-in-part annotation corrected — the v1.20 annotation over-claimed that all §12.3 rows remain authoritative for payloads setting 'timestamp: OR factory_lock:'. For a factory_lock-only Edit (!sets_timestamp && sets_factory_lock), ADR-032 Decision 3 skips Steps 4–6 (the timestamp check) and runs only Step 7 (lock-expiry check); the TimestampStale rows do NOT apply to factory_lock-only payloads. Corrected scope: TimestampStale rows gated on sets_timestamp; LockExpiryStale rows gated on (sets_timestamp OR sets_factory_lock). ADR-032 §Source/Origin updated in lock-step (ADR-032 v1.2). Closes F-ADR032-P2-002 MEDIUM. [Prior: 2026-07-20 (v1.20) — ADR-032 forward-reference amendment (architect): §12.2 and §12.3 superseded-in-part by ADR-032 Decision 1 + Decision 3. §12.2 body annotated after the fragment-only \"no-op\" warning paragraph: ADR-032 implements payload-targeted enforcement, NOT the fragment-only approach the warning describes; the warning targets a class of implementations that check the fragment regardless of payload content, and ADR-032 does not fall into that class. §12.3 table annotated: for Edit and MultiEdit tools, ADR-032 Decision 1 adds a payload-scan step before §12.3 checks are reached; if no new_string sets timestamp: at column 0, the guard returns Continue (payload-neutral) before any §12.3 row applies; §12.3 rows for payload-containing Edits and all Write operations remain authoritative; ADR-032 Decision 3 similarly gates the LockExpiryStale rows for payload-neutral Edits. Closes F-002 of ADR-032 adversary pass 1 fix burst. [Prior: 2026-07-17 (v1.19) — W3G-001 envelope-diagnostic ruling + F-005 + W3G-005 adjudication (architect): Decision 20 added — Phase-B approaching-envelope diagnostic policy for read_prefix-based guards. Mechanism: after successful read_prefix(path, max_bytes=262144) + successful frontmatter extraction, compute frontmatter_extent = byte offset of the closing --- delimiter. APPROACHING_ENVELOPE_THRESHOLD = (STATE_MD_PREFIX_BYTES * 75) / 100 = 196608 bytes (75% of 262144; production-consistent with Phase-A Invariant 10 threshold of 200000/262144 = 76.3%). When frontmatter_extent > APPROACHING_ENVELOPE_THRESHOLD emit diagnostic event state_md_approaching_prefix_envelope with fields frontmatter_extent_bytes: u64, prefix_cap_bytes: u64, utilization_pct: u32; observability-only, never blocks, never alters Continue/Block verdict. Envelope-exceeded vs malformed hard-distinguished: (a) bytes_returned == prefix_cap_bytes AND no closing --- found → FrontmatterExceedsEnvelope (new error class); emit state_md_frontmatter_exceeds_envelope (fields: prefix_cap_bytes: u64); fail-open Continue; (b) bytes_returned < prefix_cap_bytes AND no closing --- found → MalformedLockBlock (existing error class; continue unchanged). Distinguishing predicate: only when prefix_cap_bytes bytes were returned could the frontmatter exceed the envelope; fewer bytes means the full file was consumed, so absent delimiter = structurally malformed. W3G-005 adjudication: current_bytes as u32 cast in invoke.rs::setup_host_on_store_data is architecturally correct for WASM32 (linear memory ceiling = u32::MAX = 4,294,967,295 bytes; memory.grow returns None before any overflow is reachable; the INTERNAL_ERROR (-99) path from memory.grow failure precedes the cast site); accepted-with-record; fix: add debug_assert!(current_bytes <= u32::MAX as u64, \"WASM32 memory ceiling ensures this cast is safe; memory.grow None path is reached first\") and explanatory comment at cast site; anchor E-20-ARCH-02 in-scope implementer task. F-005 convention ruling: ARCH-INDEX description cells are current-state summaries (live subsystem registry per CLAUDE.md POLICY 6; the AMENDED trail is the historical record mechanism); ADR-025 row opening sentence updated host::read_file → host::read_prefix (max_bytes=262144) to reflect Phase-B production state (S-19.07 merged 6db4c9fc). D23 added. Closes W3G-001 HIGH, W3G-005 LOW, F-005 LOW. ARCH-INDEX v3.05→v3.06. [Prior: 2026-07-16 (v1.18) — F-P12-001 stale forward-ref closure (architect): §Decision 15 body — two instances of "pending product-owner amendment" forward-reference removed; BC-4.13.001 v1.17 §Precondition 3 Phase-B was completed at commit 92e4e325 (story v1.18 chain); both sites now read "as amended in BC-4.13.001 v1.17". TD-VSDD-060 sibling-sweep: no additional live-body sites beyond the two fixed. Closes F-P12-001 MEDIUM POLICY 4. [Prior: 2026-07-16 (v1.17) — S-19.07 cascade F-P1-001 BLOCKER — read_prefix bound adjudication (architect): §Decision 15 max_bytes corrected 8192→262144. The v1.11 derivation from \"ADR-026 compaction keeps frontmatter <2 KiB\" is premise-false: ADR-026 §Decision 7 is a line-count discipline (≤200/≤500 lines), not a byte bound. Measured STATE.md 2026-07-16: 178,742 bytes total; closing --- at byte 35,175; last_amended field alone 32,648 bytes. At max_bytes=8192, extract_frontmatter receives a prefix with no closing --- delimiter → full-input fallback → MalformedLockBlock → fail-open Continue → guard silently inert; the lock block is ~27 KB beyond the 8192-byte window. Adjudicated bound = 262144 (STATE.md byte envelope: BC-4.13.001 §Precondition 3 Phase-A; BC-5.40.001 §Precondition 6); structural guarantee: any on-envelope STATE.md has its closing --- within the prefix. 65536 rejected: no structural guarantee against last_amended append growth. Root disease (inlined last_amended byte-bloat) anchored to S-15.03 PRIORITY-A structured-changelog migration; ADR-026 line-discipline does not constrain bytes. D18(e) test bullet updated to max_bytes=262144. VP-095 v1.2→v1.3 issued same-burst. BC-4.13.001 Phase-B §Precondition 3 pending product-owner amendment (follow-up leg). [Prior: 2026-07-15 (v1.16) — post-E-19 host ABI adjudication (architect): Decision 16 — read_prefix production path registration gap: read_prefix registered in setup_linker (Linker<HostContext>, test path, host/mod.rs) but absent from setup_host_on_store_data (Linker<StoreData>, production path, invoke.rs); grep -n 'read_prefix' crates/factory-dispatcher/src/invoke.rs → 0 hits; any plugin with vsdd::read_prefix import fails wasmtime link on production path; D19 added (implementer registers read_prefix in setup_host_on_store_data mirroring read_file memory-grow protocol). Decision 17 — two-linker out_ptr=0 protocol boundary: test path (Linker<HostContext>/host/read_file.rs::register) writes data at WASM addr 0 and returns ptr=0; read_owned_bytes ptr==0 guard → Vec::new(); production path (Linker<StoreData>/invoke.rs::setup_host_on_store_data) grows memory, writes at current_bytes > 0, returns real address; SEC-001 CRITICAL accepted-with-record status confirmed appropriate; D20 corrects misleading comments in read_file.rs + read_prefix.rs. Decision 18 — timeout_ms non-enforcement framing: 'enforced via epoch interruption' comment in read_file.rs and read_prefix.rs is technically incorrect (epoch ticks at WASM yield points only; cannot preempt blocking func_wrap host calls executing on dispatcher thread); correct framing: timeout_ms is ABI-stable/forward-reserved; per-host-function timeout is structurally unenforced in current synchronous func_wrap dispatch; store-level epoch deadline (limits.timeout_ms) governs coarse plugin-level time; SEC-003 CWE-833 LOW severity confirmed appropriate (path_allow is operator-configured; normal local-SSD paths never block; operator accepting FIFOs/NFS accepts the risk); D20 corrects both comments. Decision 19 — INVALID_ARGUMENT (-4) not added to read_prefix capability schema: -4 is a marshalling-internal code (bad UTF-8 path, guest ptr out-of-bounds); well-formed SDK calls cannot trigger it; not operator-visible; consistent with read_file convention (read_file also omits -4 from its capability schema); current hooks-registry.toml read_prefix preamble table (0,-1,-2,-5,-99) is correct and complete; no change required. F-WG-002 + F-WG-003 routed to implementer via design-brief-post-e19-host-abi-fixes.md (bare literals + missing timestamp are implementation defects, not architecture decisions). Deliverables D19–D22 added. [Prior: 2026-07-10 (v1.15) — F-P50-001: §12.6 volatile line-cite (line 1181–1182) → stable [hooks.capabilities.read_file]-block anchor per TD-VSDD-091; whole-ADR pointer sweep (1 normative-live site fixed). [Prior: 2026-07-10 (v1.14) — F-P49-001: §Decision 1 + Deliverable D2 tool-matcher descriptions swept Edit|Write|Agent → Edit|Write|MultiEdit|Agent per live hooks-registry.toml ground truth (S-17.04/§Decision 12 sibling-sweep completion; POLICY 5 v1.3.3). [Prior: 2026-07-09 (v1.13) — F-P40-001: §Decision 14 Normative-twin stale BC-4.13.001 v1.4 pin → stable §Precondition 3 (Phase-A) + §Invariant 9 anchor form (POLICY 5 v1.3.5; matches §Decision 15 stable-cite pattern). [Prior: 2026-07-09 (v1.12) — E-19 adv-P33 F-P33-002 (architect): D18 Owner crate / path corrected to three-site form per BC-1.17.001 §Architecture Anchors two-layer structure + S-19.06 §Architecture Mapping. (a) Stale path crates/factory-dispatcher/src/host.rs (new read_prefix function) replaced with crates/factory-dispatcher/src/host/read_prefix.rs (new dispatcher host fn); factory-dispatcher host module uses split-file layout — no monolithic host.rs exists at this path. (b) Stale crates/hook-sdk/src/host.rs (new extern read_prefix declaration) split into two entries reflecting the two-layer structure: crates/hook-sdk/src/host.rs (new safe wrapper, -> Result<Vec<u8>, HostError>) and crates/hook-sdk/src/ffi.rs (new raw wire-ABI extern, -> i32 6-param; wasm32 block + host_stubs). Sweep of §Decision 15 + D18 for other host.rs-as-extern-site claims: none found — remaining host.rs references in amendment_reason are convention-reference text (\"matches host.rs u32/i32 convention\"), not owner-path claims. Closes F-P33-002 MEDIUM. [Prior: 2026-07-09 (v1.11) — E-19 adv-P32 F-P32-001 (architect): §Decision 15 body corrected. (a) Primary consumers paragraph: removed stale claim that post-migration STATE_MD_MAX_BYTES becomes a soft prefix-read bound — Phase-B removes the constant entirely at S-19.07 (BC-4.13.001 Phase-B); replaced with accurate statement that the read_prefix max_bytes call-site argument is 8192 per BC-4.13.001 §Precondition 3 Phase-B and is the sole read bound post-migration. (b) Truncation-example sentence: reframed from Phase-A 262144 cap (which is the host::read_file cap from Decision 14, not a read_prefix argument) to the Phase-B 8192 bound; clarified that 262144 is Phase-A-historical and not the read_prefix call-site argument. Closes F-P32-001 MEDIUM. [Prior: 2026-07-07 (v1.10) — E-19 adv-P11 D18(e) amendment (architect): D18 test bullet (e) corrected — verify-factory-lock plugin replaces read_file call with read_prefix (max_bytes=8192 per BC-4.13.001 Phase-B) and STATE.md frontmatter is parsed correctly from the 8192-byte prefix even when the full file approaches the 262144-byte Phase-A cap (fixture body padded past 8192). Closes F-P11-005. [Prior: 2026-07-06 (v1.9) — E-19 adv-P3 architect reconciliation F-P3-001+F-P3-002 (architect): Decision 15 signature corrected (u64→u32, adds timeout_ms: u32; return type i64→i32; matches host.rs u32/i32 convention + BC-2.02.002 mandatory-timeout-ms discipline). Capability model corrected: separate [hooks.capabilities.read_prefix] block required (BC-1.17.001 Invariant 3; defense-in-depth; capabilities.read_file does NOT extend to read_prefix). D18 updated to match: signature, capability key, CAPABILITY_DENIED path reference corrected read_file.path_allow→read_prefix.path_allow. D18 test bullets (c)/(d) disambiguated: FFI return codes (-5/-1) are host function return values to WASM plugin caller; plugin process exits 0 (Continue) in both error cases (no block intent). [Prior: 2026-07-06 (v1.8) — E-19 VP package POLICY 9 propagation (architect): Decision 15 added — host::read_prefix additive host function allocation (HOST_ABI_VERSION=1 unchanged; never OUTPUT_TOO_LARGE; absent path returns NOT_FOUND (-5); mirrors Decision 13 additive code precedent). D18 added. [Prior: 2026-07-06 (v1.7) — E-19 adv-P1 fix burst (architect): Decision 13 added — host ABI codes::NOT_FOUND = -5 allocation (F-P1-001 BLOCKER closure: -4 collides with INVALID_ARGUMENT; enumeration 0/-1/-2/-3/-4/-99 occupied; -5 next free; HOST_ABI_VERSION=1 unchanged, additive code). Decision 14 added — verify-factory-lock STATE_MD_MAX_BYTES 65536→262144 + frontmatter-only parse (mirrors BC-4.13.001 v1.4 Precondition 3 + Invariant 9; closes rc.22 smoke FINDING-1). F-P1-005 routing reclaim: ADR authored by architect. TD-031 executor.rs line-cite fixed (10 occurrences). [Prior: v1.5→v1.6: [S-17.04 redirect — human approved, adversary pass 1 incorporated] Decision 12 added: `verify-state-timestamp-refresh` WASM PreToolUse guard plugin. Decision 11 Mechanism 2 SUPERSEDED. Mechanism 1 (D10) RETAINED. (a) DECISION 12: new WASM plugin `verify-state-timestamp-refresh` in `crates/hook-plugins/verify-state-timestamp-refresh/`, registered in `hooks-registry.toml` as PreToolUse on Edit|Write|MultiEdit where file_path resolves to `.factory/STATE.md`. Per-tool proposed-content extraction: Write → `tool_input.content` (full file body); Edit → on-disk STATE.md with `tool_input.old_string` replaced by `tool_input.new_string` (first occurrence; `replace_all` honored); MultiEdit → on-disk STATE.md with each `tool_input.edits[]` element applied sequentially. Guard reads on-disk via `host::read_file`; compares time fields: BLOCKS if `timestamp:` not advanced (TimestampStale) OR lock held in proposed content and `factory_lock.expires_at` not advanced (LockExpiryStale). Fails open on parse/IO errors per Decision 7 precedent. (b) CORRECTED FINDINGS (adversary pass 1): `new_content` stale field removed (not a real Claude Code payload field — correct fields are `content`/`old_string`+`new_string`/`edits[]`); `[hooks.capabilities.read_file]` corrected to `path_allow`-only (ReadFileCaps is `#[serde(deny_unknown_fields)]` with only `path_allow: Vec<String>` — `max_bytes`/`timeout_ms` fields do not exist in the struct and would break registry load); explicit priorities added (verify-factory-lock at 142, verify-state-timestamp-refresh at 143 — must run AFTER verify-factory-lock, so 143 > 142); canonical-path rule specified (strip leading `./`, collapse `//`, treat absolute `$CLAUDE_PROJECT_DIR/`-prefixed paths — robust normalization, not fail-open); block message format corrected to real `block_with_fix` segments; robust frontmatter extraction specified. (c) SUPERSESSION: Decision 11 Mechanism 2 (D11/D12-registry/D14) WITHDRAWN. Push-time enforcement dropped. (d) INV-019 CURE: (a) Decision 12 added; (b) D11/D12-registry/D14 withdrawn, D15/D16/D17 added; (c) S-17.04 Re-Scope Directive issued. ARCH-INDEX v2.21→v2.22 pending state-manager codification burst. (e) GEMINI CROSS-FAMILY PASS (adversary pass 2): two semantics gaps closed: (R2) lock-held + proposed expires_at absent OR empty now Blocks LockExpiryStale (previously only byte-identical-to-on-disk triggered the block; absent/empty slipped through); (R4) `..` segment resolution added to canonical-path algorithm (segment-stack pop, fail-open if above-root escape). One clarity note added: `timestamp:` is the sole independently-gated freshness field; `last_amended:` is advanced by state-manager POLICY-14 discipline but not independently gated. Lock-identity guard sibling-sweep noted: verify-factory-lock `tool` matcher MUST include MultiEdit for parity. AC/test delta directive issued (§12.9). INV-019 cure: (a) §12.2 time-field table and §12.3 table updated; (b) §12.7 R6 extended; (c) AC/test delta directive issued. (f) ADVERSARY PASS 5 — factory-lock-parse crate relocation: `factory-lock-parse` is a pure library crate (no `[[bin]]`, no WASM output); placing it under `crates/hook-plugins/` inflated the WASM-plugin floor-count gate's expected count and broke CI. Correct path is `crates/factory-lock-parse/`. Updated in §12.5 and D15. INV-019 cure: (a) §12.5 and D15 path updated; (b) no other deliverables affected; (c) one-sentence rationale added in §12.5. (g) ADVERSARY PASS 7 — P0 WASM env-var dead-code fix: §12.7 R6 step 1 specified stripping `$CLAUDE_PROJECT_DIR/` via `std::env::var("CLAUDE_PROJECT_DIR")`. The WASI sandbox inherits NO env vars (WasiCtxBuilder in `crates/factory-dispatcher/src/invoke.rs` uses preopened_dir only, never .env()/.inherit_env()); `std::env::var` always returns Err in production. Claude Code tools emit ABSOLUTE paths (verified in dispatcher logs); the prior step 1 was dead code — absolute paths never stripped, guard always returned Continue → guard was inert in production. Fix: replace env-var strip with WASM-correct suffix/equality rule: after `./`, `//`, `/./`, `..` normalizations, trigger if normalized path EQUALS `.factory/STATE.md` OR ENDS WITH `/.factory/STATE.md`. No env dependency; no capability required; handles both relative and absolute forms. `host::env`+`env_allow` route explicitly rejected (reintroduces env_allow silent-no-op footgun, ADR-025 v1.3 class). §12.1 trigger description updated; §12.7 R6 rewritten; §12.9 absolute-path bats e2e test mandate added. INV-019 cure: (a) §12.1 and §12.7 R6 rewritten; (b) §12.9 updated with absolute-path e2e mandate; (c) AC/EC delta directive updated." v1.4→v1.5 amendment_reason preserved inline: [S-17.04 adversary F-1701-001] Gate-trigger fix for Decision 11 Mechanism 2 + block-message reconciliation + D12 jq capability sync. (1) TRIGGER CORRECTION: the v1.4 spec stated the gate triggers on `git.*push.*factory-artifacts` in the Bash tool-command string. This is inert on the production push path: post-S-17.01 the state-burst SKILL runs `bash plugins/vsdd-factory/bin/factory-cas-push.sh`, and the real `git push --force-with-lease` is a subprocess inside that helper — PreToolUse never inspects subprocess commands. The gate MUST trigger when `.tool_input.command` contains `factory-cas-push` (the canonical helper the SKILL uses) OR matches `git`+`push`+`factory-artifacts` (belt-and-suspenders for any hand-typed raw push). The check fires at PreToolUse on `bash factory-cas-push.sh`, at which point the burst commit already exists locally (HEAD STATE.md carries this burst's expires_at), so the HEAD-vs-origin comparison is valid. (2) BLOCK MESSAGE RECONCILIATION: the legacy-bash-adapter truncates plugin output to the first line of stdout. The implemented gate must therefore emit a single-line block_pre-form message: 'BLOCKED by verify-lock-renewal: RenewalMissed — factory_lock held but expires_at not refreshed in this burst. Fix: Run: factory-lock-write.sh renew .factory/STATE.md Code: RenewalMissed.' The multi-line verbatim text specified in v1.4 step 6 is unreachable through the legacy-bash-adapter; it is now replaced by this one-liner in the Decision 11 spec. (3) D12 JQ SYNC: D12 `exec_subprocess.binary_allow` must include `\"jq\"` alongside `\"bash\"` and `\"git\"`. The gate script execs `jq` to parse the JSON-envelope STATE.md frontmatter; omitting `jq` from binary_allow causes CapabilityDenied → silent fail-open → gate is inert. This is the fourth instance of the deny-by-default silent-no-op footgun class (vector 4: exec_subprocess binary_allow missing required tool for script internals). v1.3→v1.4 amendment_reason preserved inline: [S-17.04] Automatic heartbeat renewal enforcement wiring. Decision 11 added: two complementary mechanisms close the prose-only PC4 enforcement gap — (1) mandatory executable factory-lock-write.sh renew step in state-burst SKILL before git add/commit (Option A); (2) new verify-lock-renewal.sh PreToolUse bash hook that blocks a held-lock factory-artifacts push when HEAD's expires_at equals origin/factory-artifacts' expires_at (RenewalMissed — renewal not committed in this burst), on_error=continue, async=false, no-op when unlocked or no remote baseline (Option C). Decision 5 vestigial burst-END-only sentence corrected. Deliverables D10–D14 added. BC-5.40.001 PC4 unaffected. v1.2→v1.3 amendment_reason preserved inline: [process-gap] S-17.02 TDD implementation finding — exec_subprocess env_allow omission footgun. Decision 2 / D2 capability block spec was incomplete: exec_subprocess capability block listed only binary_allow = [\"git\"] but omitted env_allow. The dispatcher's exec_subprocess host function calls env_clear() and passes ONLY vars listed in caps.env_allow; without HOME (and GIT_CONFIG_GLOBAL / XDG_CONFIG_HOME) in env_allow, git config user.email cannot read the developer's global gitconfig, returns empty string, plugin hits IdentityResolutionFailed, fails open (Continue), and the lock guard is a silent no-op. This is the THIRD instance of the deny-by-default silent-no-op footgun class (first: read_file block omitted; second: exec_subprocess binary_allow omitted; third: exec_subprocess env_allow omitted). Fix: Decision 2 and D2 canonical registry snippet updated to include env_allow = [\"HOME\", \"GIT_CONFIG_GLOBAL\", \"XDG_CONFIG_HOME\"] on the exec_subprocess capability block. Rationale section updated to name all three footgun vectors explicitly. Process note added.]]]]]]]]"
title: "ADR-025: Single-writer factory lock/lease — prevent concurrent session races on factory-artifacts orphan branch"
traces_to: .factory/specs/architecture/ARCH-INDEX.md
anchors:
  - SS-04
  - SS-05
  - SS-06
  - SS-07
  - issue-170
subsystems_affected:
  - SS-04
  - SS-05
  - SS-06
  - SS-07
supersedes: null
superseded_by: null
decision_status: accepted
human_gate_required: false
human_gate_reason: "All decisions confirmed by human design review 2026-06-10. Research-agent verification returned APPROVE-WITH-FIXES 2026-06-10; all five fixes incorporated in v1.2. No remaining human-gated open questions. D-540 codification recorded by state-manager 2026-06-10. Implementation may proceed."
---

# ADR-025: Single-writer factory lock/lease — prevent concurrent session races on factory-artifacts orphan branch

## Status

**ACCEPTED — human design confirmed 2026-06-10; research-agent verification APPROVE-WITH-FIXES incorporated as v1.2. D-540 codification recorded by state-manager 2026-06-10. Implementation dispatch ready. v1.3 amended 2026-06-11: [process-gap] S-17.02 TDD finding — exec_subprocess env_allow omission footgun; env_allow = ["HOME", "GIT_CONFIG_GLOBAL", "XDG_CONFIG_HOME"] added to D2 canonical registry form. v1.4 amended 2026-06-11: [S-17.04] Decision 11 added — automatic heartbeat renewal enforcement (executable state-burst SKILL step + PreToolUse push gate); Decision 5 vestigial burst-END sentence corrected; Deliverables D10–D14 added. v1.5 amended 2026-06-11: [S-17.04 adversary F-1701-001] Decision 11 gate-trigger correction (trigger must fire on `factory-cas-push` helper, not only raw `git push`; the real push runs as subprocess inside the helper and is invisible to PreToolUse), block-message reconciled to legacy-bash-adapter one-liner form, D12 binary_allow extended with "jq" (fourth deny-by-default silent-no-op vector closed). v1.6 amended 2026-06-11: [S-17.04 redirect — human approved; adversary pass 1 incorporated] Decision 12 added — `verify-state-timestamp-refresh` Rust WASM PreToolUse guard: blocks Edit/Write/MultiEdit to STATE.md when proposed full content (reconstructed for Edit/MultiEdit) does not advance `timestamp:` frontmatter (every write) or `factory_lock.expires_at` (when lock held). Per-tool field extraction: Write→`tool_input.content`; Edit→on-disk+`old_string`/`new_string` reconstruct; MultiEdit→sequential `edits[]` apply. Registry caps corrected: `[hooks.capabilities.read_file]` accepts `path_allow` ONLY (`max_bytes`/`timeout_ms` are not struct fields — adding them breaks registry load). Priorities made explicit: verify-factory-lock=142, verify-state-timestamp-refresh=143 (lock-identity check fires first). Canonical-path normalization rule specified. Block message format corrected to real `block_with_fix` segments. Decision 11 Mechanism 2 (D11/D12-registry/D14) SUPERSEDED. Mechanism 1 (D10) RETAINED. S-17.04 REDIRECTED to v1.2. Gemini cross-family pass (adversary pass 2) incorporated: lock-held+absent/empty `expires_at` now Blocks LockExpiryStale (closes enforcement asymmetry); `..` segment resolution added to canonical-path algorithm; clarity note on `timestamp:` vs `last_amended:` scoping added; lock-identity guard MultiEdit sibling-sweep noted. Adversary pass 5: `factory-lock-parse` crate path corrected to `crates/factory-lock-parse/` (lib-only crate must not live under `crates/hook-plugins/` — WASM floor-count gate scope). Adversary pass 7 (P0): `$CLAUDE_PROJECT_DIR` env-var strip was dead code in WASM sandbox (WasiCtxBuilder inherits no env vars; `std::env::var` always returns Err; absolute `file_path` forms never matched; guard was inert in production). Fixed: §12.7 R6 step 1 replaced with WASM-correct suffix/equality rule — trigger when normalized path equals `.factory/STATE.md` OR ends with `/.factory/STATE.md`. No env dependency. Absolute-path bats e2e test mandate added to §12.9. S-17.04 + rc.21 HELD. v1.7 amended 2026-07-06: [E-19 adv-P1 fix burst — architect] Decision 13 added — host ABI codes::NOT_FOUND = -5 allocation (F-P1-001 BLOCKER closure: -4 collides with INVALID_ARGUMENT; -5 is next free; HOST_ABI_VERSION=1 unchanged, additive code). Decision 14 added — verify-factory-lock STATE_MD_MAX_BYTES 65536→262144 + frontmatter-only parse (mirrors BC-4.13.001 v1.4 Precondition 3 + Invariant 9; closes rc.22 smoke FINDING-1). TD-031 executor.rs line-cite fixed (10 occurrences). F-P1-005 routing reclaim: ADR authored by architect. v1.8 amended 2026-07-06: [E-19 VP package POLICY 9 propagation — architect] Decision 15 added — host::read_prefix additive host function (HOST_ABI_VERSION=1 retained; never OUTPUT_TOO_LARGE; absent path returns NOT_FOUND (-5); mirrors Decision 13 additive precedent). D18 added. v1.9 amended 2026-07-06: [E-19 adv-P3 architect reconciliation — F-P3-001+F-P3-002] Decision 15 signature corrected (u32+timeout_ms: u32, return i32; matches host.rs convention; BC-2.02.002 mandatory-timeout); capability model corrected (separate capabilities.read_prefix per BC-1.17.001 Invariant 3; read_file capability does not extend to read_prefix); D18 updated to match; D18 test bullets (c)/(d) disambiguated (FFI return codes -5/-1 are host function return values; plugin process exits 0 Continue in both error cases). v1.10 amended 2026-07-07: [E-19 adv-P11 F-P11-005] D18 test bullet (e) corrected — verify-factory-lock plugin replaces read_file call with read_prefix (max_bytes=8192 per BC-4.13.001 Phase-B); fixture body padded past 8192 tests correct parsing from 8192-byte prefix when full file approaches 262144-byte Phase-A cap. v1.11 amended 2026-07-09: [E-19 adv-P32 F-P32-001 — architect] §Decision 15 body corrected — (a) Primary consumers paragraph reworded: STATE_MD_MAX_BYTES is removed entirely at S-19.07 (BC-4.13.001 Phase-B); read_prefix max_bytes=8192 per BC-4.13.001 §Precondition 3 Phase-B is the sole post-migration read bound. (b) Truncation-example sentence reframed from Phase-A 262144 cap (Decision 14 host::read_file cap, not a read_prefix argument) to the Phase-B 8192 bound; 262144 explicitly marked Phase-A-historical. Closes F-P32-001 MEDIUM. v1.12 amended 2026-07-09: [E-19 adv-P33 F-P33-002 — architect] D18 Owner crate / path corrected to three-site form per BC-1.17.001 §Architecture Anchors — (a) crates/factory-dispatcher/src/host/read_prefix.rs (new dispatcher host fn; split-file layout); (b) crates/hook-sdk/src/host.rs (new safe wrapper); (c) crates/hook-sdk/src/ffi.rs (new raw wire-ABI extern; wasm32 block + host_stubs). Closes F-P33-002 MEDIUM. v1.13 amended 2026-07-09: [F-P40-001 — architect] §Decision 14 Normative-twin stale BC-4.13.001 v1.4 pin → stable §Precondition 3 (Phase-A) + §Invariant 9 anchor form (POLICY 5 v1.3.5; matches §Decision 15 stable-cite pattern). Closes F-P40-001 MEDIUM. v1.14 amended 2026-07-10: [F-P49-001 — architect] §Decision 1 + Deliverable D2 tool-matcher descriptions swept Edit|Write|Agent → Edit|Write|MultiEdit|Agent per live hooks-registry.toml ground truth (S-17.04/§Decision 12 sibling-sweep completion; POLICY 5 v1.3.3). Closes F-P49-001 MEDIUM. v1.15 amended 2026-07-10: [F-P50-001 — architect] §12.6 volatile line-cite (line 1181–1182) → stable `[hooks.capabilities.read_file]`-block anchor per TD-VSDD-091; whole-ADR pointer sweep (1 normative-live site). Closes F-P50-001 MEDIUM. v1.16 amended 2026-07-15: [post-E-19 host ABI adjudication — architect] Decision 16: `read_prefix` absent from `setup_host_on_store_data` (production path in `invoke.rs`); confirmed 0-hit grep; D19 added — implementer registers `read_prefix` in `setup_host_on_store_data` mirroring `read_file` memory-grow protocol. Decision 17: two-linker `out_ptr=0` protocol boundary documented — test path (`Linker<HostContext>`) writes at addr 0; production path (`Linker<StoreData>`) grows memory and writes at `current_bytes > 0`; SEC-001 CRITICAL accepted-with-record confirmed appropriate; D20 corrects misleading comments. Decision 18: `timeout_ms` non-enforcement framing corrected — epoch interruption cannot preempt blocking `func_wrap` host calls; `timeout_ms` is ABI-forward-reserved; SEC-003 CWE-833 LOW severity confirmed; D20 corrects doc comments in both `read_file.rs` and `read_prefix.rs`. Decision 19: INVALID_ARGUMENT (-4) not added to `hooks-registry.toml` `read_prefix` capability schema — marshalling-internal code, not operator-visible; current preamble table is correct. F-WG-002/F-WG-003 routed to implementer via design brief. Deliverables D19–D22 added. v1.17 amended 2026-07-16: [S-19.07 cascade F-P1-001 BLOCKER — read_prefix bound adjudication — architect] §Decision 15 max_bytes corrected 8192→262144. The v1.11 derivation from "ADR-026 compaction keeps frontmatter <2 KiB" is premise-false: ADR-026 §Decision 7 is a line-count discipline (≤200/≤500 lines), not a byte bound; measured closing `---` at byte 35,175 of 178,742-byte STATE.md (2026-07-16; `last_amended` alone 32,648 bytes); at max_bytes=8192 the guard is silently inert (`extract_frontmatter` full-input fallback → `MalformedLockBlock` → fail-open Continue). Adjudicated bound = 262144 (STATE.md byte envelope per BC-4.13.001 §Precondition 3 Phase-A + BC-5.40.001 §Precondition 6); structural guarantee: on-envelope STATE.md files have closing `---` within the 262144-byte prefix. 65536 rejected (no structural margin against `last_amended` growth). Root disease (inlined `last_amended` byte-bloat) anchored to S-15.03 PRIORITY-A. D18(e) fixture corrected to max_bytes=262144. VP-095 v1.2→v1.3 same-burst. BC-4.13.001 Phase-B §Precondition 3 pending product-owner amendment. v1.18 amended 2026-07-16: [F-P12-001 MEDIUM stale forward-ref closure — architect] §Decision 15 body: two "pending product-owner amendment" forward-references replaced with "as amended in BC-4.13.001 v1.17" (BC-4.13.001 v1.17 completed the amendment at commit 92e4e325, story v1.18 chain). TD-VSDD-060 sibling-sweep: no additional live-body sites. Closes F-P12-001 MEDIUM POLICY 4. v1.19 amended 2026-07-17: [W3G-001 HIGH envelope-diagnostic ruling + W3G-005 LOW + F-005 LOW — architect] Decision 20 added — Phase-B approaching-envelope diagnostic policy. Approaching-envelope threshold 196608 bytes (75% of 262144); event `state_md_approaching_prefix_envelope` (fields: `frontmatter_extent_bytes`, `prefix_cap_bytes`, `utilization_pct`); observability-only. Envelope-exceeded vs malformed hard-distinguished by predicate `bytes_returned == prefix_cap_bytes`: true → `FrontmatterExceedsEnvelope` new error class + `state_md_frontmatter_exceeds_envelope` event + fail-open Continue; false → `MalformedLockBlock` existing path. W3G-005: `current_bytes as u32` accepted-with-record (WASM32 ceiling; `debug_assert!` + comment; E-20-ARCH-02). F-005: ARCH-INDEX description cells are current-state summaries (live registry POLICY 6); ADR-025 row opening sentence updated `host::read_file` → `host::read_prefix (max_bytes=262144)`. D23 added. ARCH-INDEX v3.05→v3.06. Closes W3G-001, W3G-005, F-005. v1.20 amended 2026-07-20: [ADR-032 forward-reference amendment — architect] §12.2 and §12.3 superseded-in-part annotations added for ADR-032 Decision 1+3 payload-targeted enforcement. §12.3 annotation stated §12.3 rows remain authoritative for all Edit/MultiEdit that sets `timestamp:` or `factory_lock:`, and all Write operations. Closes F-002 of ADR-032 adversary pass 1 fix burst. v1.21 amended 2026-07-20: [F-ADR032-P2-002 MEDIUM — architect] §12.3 blockquote annotation corrected — TimestampStale rows gated on `sets_timestamp` only; LockExpiryStale rows gated on `(sets_timestamp OR sets_factory_lock)`; factory_lock-only Edit bypasses TimestampStale rows (Steps 4–6 skipped) but still triggers LockExpiryStale rows (Step 7 runs). ADR-032 §Source/Origin updated in lock-step (ADR-032 v1.2). Closes F-ADR032-P2-002 MEDIUM. v1.22 amended 2026-07-20: [F-ADR032-P3-002 MEDIUM — architect] §12.3 blockquote annotation first sentence corrected — payload-neutral condition narrowed to NEITHER `timestamp:` NOR `factory_lock:` set in any `new_string`. The v1.21 text only required `timestamp:` to be absent; a payload that sets `factory_lock:` but not `timestamp:` is not payload-neutral (ADR-032 Decision 3 still runs Step 7 for such payloads). Annotation header bumped (v1.21)→(v1.22). ADR-032 §Source/Origin updated in lock-step (ADR-032 v1.4). Closes F-ADR032-P3-002 MEDIUM. v1.23 amended 2026-07-20: [F-ADR032-P6R-004 LOW — architect] §12.3 annotation Continue-phrasing parity correction — "the guard returns `Continue` immediately" corrected to "the guard returns `Continue` after skipping the timestamp and lock-expiry checks" (parity with ADR-032 §Source/Origin correction at v1.9 F-ADR032-P7-003). Annotation header bumped (v1.22)→(v1.23). ADR-032 §Source/Origin updated in lock-step (ADR-032 v1.11). Closes F-ADR032-P6R-004 LOW. v1.24 amended 2026-07-20: [F-ADR032-P7R-002 MEDIUM — architect] §12.2 supersedes-in-part annotation corrected — header (v1.20)→(v1.24); payload-neutral definition corrected from single-field (sets `timestamp:` at column 0) to two-field (neither `timestamp:` nor `factory_lock:` in any `new_string`; factory_lock-only Edit skips Steps 4–6 but still runs Step 7); "Continue immediately" → "Continue after skipping the timestamp and lock-expiry checks". ADR-032 amended to v1.12 in lock-step. Closes F-ADR032-P7R-002 MEDIUM.**

This ADR resolves the design for the factory lock/lease primitive requested in issue #170.
Twenty decisions are confirmed. Five research-agent fixes are incorporated in v1.2, one
process-gap spec-drift amendment in v1.3, one enforcement-wiring amendment in v1.4, one
gate-trigger + message + capability correction in v1.5, one Rust WASM guard adoption
with Decision 11 Mechanism 2 supersession plus adversary pass 1 corrections in v1.6
(per-tool payload extraction, registry caps reality, explicit priorities, canonical-path
rule, block-message format, robust time extraction — see amendment_reason above), one
host-ABI additive code + guard read-cap amendment in v1.7, one host-function additive
amendment in v1.8, four post-E-19 host ABI adjudications in v1.16 (Decisions 16–19), one
Phase-B envelope-diagnostic policy in v1.19 (Decision 20), one ADR-032 forward-reference
annotation amendment in v1.20, one §12.3 annotation scope-split amendment in v1.21, and one
§12.3 annotation first-sentence correction in v1.22, one §12.3 Continue-phrasing parity correction in v1.23, and one §12.2 supersedes-in-part two-field definition + Continue-phrasing correction in v1.24.
No further human-gated questions remain.

## Context

### The gap: cross-session single-writer is absent

The factory's within-session single-writer discipline is real and robust: `state-manager`
is the sole `.factory/` writer, runs last in every burst, and commits atomically via the
single-commit burst protocol (TD-VSDD-053). However, this discipline has no equivalent
across independent developer sessions.

Two developers — or two Claude Code sessions belonging to different developers — can run
pipelines against the same repo's `factory-artifacts` orphan branch concurrently. Because
all factory state converges on that one branch, concurrent runs race: one party's
state/spec commits can be silently lost, clobbered, or produce a painful orphan-branch
divergence with no merge base (which requires manual surgery to reconcile, as there is no
common ancestor).

Research cache (`issue-170.md`) confirms the gap with zero relevant grep hits for
`lock|flock|mutex|lease|heartbeat|session_id|exclusive` across `plugins/`. The push path
is a plain `git push origin factory-artifacts` at `skills/state-burst/SKILL.md` (the push
call) with no compare-and-swap. `hooks/verify-git-push.sh` explicitly allows
`factory-artifacts` pushes and `--force-with-lease` with no exclusivity check.

### Design principle: keep the mechanism local and simple

The factory's primary deployment is a single developer — or a small team where turn-based
coordination is natural. The concurrency hazard is not millisecond-scale races but
session-level mistakes: two people both start pipeline work without realizing the other is
active. A simple, local, human-readable cooperative lock — visible in STATE.md, enforced
by a WASM hook, controlled by explicit user commands — is the right fit for this threat
model.

A heavyweight server-side CAS mechanism (git refs, etcd, etc.) imposes verification
prerequisites and infrastructure assumptions that are not warranted for the actual threat
model. That upgrade path is preserved as a future option (see §Decision 9) but is not the
primary design.

### Scope

This ADR scopes the lock to **whole-factory granularity** (per repo's `factory-artifacts`
branch) and **developer-level identity** (`git config user.email`). The documented
tradeoff — that the same developer in two concurrent sessions will NOT self-block — is
accepted. The guard protects Developer A vs Developer B, not self-vs-self.

## Decision

This ADR makes twenty numbered decisions. The original ten (Decisions 1–10) are confirmed
by human review 2026-06-10 and verified by research-agent review 2026-06-10 (v1.2
incorporates five APPROVE-WITH-FIXES corrections). Decisions 11–12 were added in v1.4–v1.6.
Decisions 13–15 were added in v1.7–v1.8. Decisions 16–19 were added in v1.16. Decision 20
was added in v1.19.

### Decision 1: Primary enforcement — native-WASM PreToolUse guard `verify-factory-lock`

We will implement a **new native-WASM hook plugin** `verify-factory-lock` as the primary
enforcement mechanism:

- **New Rust crate:** `crates/hook-plugins/verify-factory-lock/` compiled to
  `plugins/vsdd-factory/hook-plugins/verify-factory-lock.wasm`.
- **Registered in `plugins/vsdd-factory/hooks-registry.toml`** as a `PreToolUse` guard
  on mutating tools: `tool = "Edit|Write|MultiEdit|Agent"` plus a separate entry for Bash covering
  `.factory/` pushes. See Decision 2 (deliverable D2) for the complete and mandatory
  capability block specification.
- **Guard logic:** reads `STATE.md` via `host::read_file`, parses the `factory_lock`
  frontmatter block, checks `holder != current_git_email` and `now <= expires_at`. If
  both conditions are true, returns `block_intent = true` (exit code 2) with the
  actionable refusal message (Decision 4). The production block path runs through
  `plugin_requests_block()` in `executor::plugin_requests_block`, invoked
  at `the sync-group executor dispatch` for sync-group plugins. The guard MUST be sync-group
  (`async = false`) — see Decision 2.
- **Read-only tool calls pass through** unconditionally. Only mutating tool calls (Edit,
  Write, Agent dispatch, and Bash commands that push to `factory-artifacts`) are blocked.

**Host ABI is unchanged.** The guard uses only `host::read_file` and
`host::exec_subprocess` with `binary_allow = ["git"]` — both already present in the
dispatcher host ABI at `HOST_ABI_VERSION = 1` (`hook_sdk::HOST_ABI_VERSION`). The
dispatcher binary (`crates/factory-dispatcher`) requires no changes for this feature.

The guard follows the same pattern as `validate-artifact-path.wasm` (an existing
native-WASM PreToolUse guard using `host::read_file`), confirming the structure is
established and the crate scaffolding is known.

Referencing SS-04 (Plugin Ecosystem) because `verify-factory-lock` is a new WASM plugin
crate in `crates/hook-plugins/`. Referencing SS-05 (Pipeline Orchestration) because
`state-manager` writes the `factory_lock` frontmatter block that the guard reads.
Referencing SS-06 (Skill Catalog) because `/factory-lock` and `/factory-unlock` are new
skills. Referencing SS-07 (Hook Bash Layer) because the guard is registered in
`hooks-registry.toml` alongside existing guards.

### Decision 2: Lock state — `factory_lock` frontmatter block in STATE.md

The authoritative lock state lives in the `factory_lock` block in `STATE.md` frontmatter:

```yaml
factory_lock:
  holder: "developer@example.com"   # git config user.email of the locking session
  locked_at: "2026-06-10T14:00:00Z" # ISO-8601
  expires_at: "2026-06-10T14:45:00Z" # ISO-8601; locked_at + TTL
```

Absent or null `factory_lock` block = unlocked. A malformed block (missing required
fields) is treated as unlocked (fail-open, consistent with Decision 7).

The `factory_lock` block travels on `factory-artifacts`, so any developer fetching the
branch sees the current lock state. Any developer can inspect it with
`cat .factory/STATE.md` or via `/factory-health`. The guard reads it via `host::read_file`
without any network call — the fetch of `factory-artifacts` that happens at burst start
is the synchronization point.

`state-manager` is the sole writer of this block, consistent with its role as the sole
`.factory/` writer (TD-VSDD-053). The `/factory-lock` and `/factory-unlock` skills
(Decision 6) delegate writing to `state-manager`.

**Note:** See Decision 3 and the D2 capability block specification for the env_allow
requirement on `exec_subprocess`. Without `HOME` in `env_allow`, `git config user.email`
cannot read the developer's global gitconfig and identity resolution fails open.

### Decision 3: Session identity — `git config user.email` (developer-level, coarse)

The lock holder identity is the output of `git config user.email`, obtained by the guard
via `host::exec_subprocess` with `binary_allow = ["git"]`.

**Documented tradeoff:** this is developer-level identity, not session-level. The same
developer running two concurrent sessions on two machines shares the same git email and
will NOT be self-blocked. The guard protects Developer A vs Developer B. Self-vs-self
concurrency (one developer, two sessions) is out of scope for this iteration and is
addressed by social coordination and the observability surfacing in `/factory-health`.

This tradeoff is accepted because:
1. The primary risk is two different developers inadvertently running concurrent sessions,
   not one developer deliberately doing so.
2. Composite session identity (hostname + pid + Claude session ID) introduces env-var
   dependencies (`CLAUDE_SESSION_ID`) and complexity that is not warranted by the threat
   model.
3. The blind-push fix (Decision 8) remains active as a safety net for the self-vs-self
   case: concurrent pushes from the same developer are detected rather than silently
   clobbering.

### Decision 4: Block semantics and refusal message

When `factory_lock.holder` is set, `now <= expires_at`, and `holder != current_git_email`:

- The guard returns `block_intent = true` (exit code 2) for mutating tools: Edit, Write,
  Agent dispatch, and Bash commands pushing to `factory-artifacts`. This signals the
  dispatcher's sync-group block path (`the sync-group executor dispatch`,
  `plugin_requests_block` at `executor::plugin_requests_block`).
- Read-only tools (Read, Bash reads, non-mutating tool calls) pass through unconditionally
  so the blocked developer can inspect STATE.md to see who holds the lock and when it
  expires.

The refusal message MUST include all of:
- `holder` — the git email of the current lock holder
- `locked_at` — ISO-8601 timestamp when the lock was acquired
- `expires_at` — ISO-8601 timestamp when the lock auto-expires
- `time_remaining` — human-readable duration (e.g., "37 min remaining")
- `/factory-unlock --force` — the exact command to break-glass force-release the lock

Example refusal output:
```
Factory locked by developer@example.com
  Locked at:  2026-06-10T14:00:00Z
  Expires at: 2026-06-10T14:45:00Z (37 min remaining)

To wait: the lock auto-expires at 14:45 UTC.
To force-release: /factory-unlock --force
```

### Decision 5: Stale-lock escape — TTL auto-expiry AND `/factory-unlock --force`

Both escape paths are required. A lock without escape is a stale-lock footgun in waiting.

**Path A — TTL auto-expiry:**
- Default TTL: **45 minutes** (midpoint of the research-backed 2–5× expected burst
  duration range; expected burst duration ~10 minutes).
- The guard computes `now > expires_at` on every check. An expired lock is treated as
  absent — the check passes and the operation proceeds.
- Heartbeat renewal: `state-manager` updates `expires_at = now + TTL` on every
  `state-burst` completion, extending the lease while the session is active. The renewal
  heartbeat fires on every `state-manager` commit in a burst (Commits A through E), not
  only at burst-close. See Decision 11 for the enforcement mechanism.
- A crashed session that never calls `/factory-unlock` auto-expires after 45 minutes at
  worst.

**Failure mode — long burst TTL self-eviction:**

A single burst longer than the 45-minute TTL (e.g., a 30-pass adversary cascade, a large
batch story delivery, a slow network during multi-file commits) self-evicts the lock
mid-burst: `now > expires_at` becomes true while the burst is still running, allowing
another developer to acquire. This is the long-operation hazard identified in lease
literature (Kleppmann §8 "Leases and Lease-Based Locks"; Kubernetes Lease API design notes;
HashiCorp Vault session TTL guidance).

**Mitigation chosen — mid-burst renewal via explicit `/factory-renew` call:**

`state-manager` MUST call a mid-burst renewal whenever a long-running sub-step (e.g.,
each adversary pass within a cascade) is about to commit. Concretely: at every intermediate
`state-manager` commit within a burst (not only at burst-close), `state-manager` writes
an updated `expires_at = now + TTL` alongside the commit. This resets the TTL clock to 45
minutes from each intermediate write rather than from the original `locked_at`. No
separate background timer process is required — the burst's own commit cadence is the
renewal heartbeat.

**Residual risk — fencing:**

Mid-burst renewal via commit does not provide a fencing token (a monotonically increasing
value that storage can check to reject stale-holder writes). If the TTL expires between
two intermediate commits — possible under extreme network delay or WASM fuel exhaustion
on the renewal commit itself — a second developer could acquire between renewals and both
parties proceed in parallel. This residual risk is **explicitly attributed to the Decision
9 git-ref-CAS future path** as the correctness-class upgrade: git ref CAS with monotonic
object-id chaining provides the fencing token that eliminates this window. Under the
current design (advisory/efficiency-class lock per Kleppmann's distinction — see Decision
7), the residual window is accepted because the threat model is cooperative teams, not
adversarial concurrent writers.

**Path B — `/factory-unlock --force` break-glass:**
- Any developer (not just the holder) may run `/factory-unlock --force` to clear the lock
  immediately.
- Force-release is **loudly audit-logged** via the SS-03 event pipeline as
  `factory.lock.stolen` including: `stolen_by` (git email of the releaser), `stolen_from`
  (git email of the original holder), `holder_locked_at`, `stolen_at`. This event is
  non-blocking but permanent — the audit trail cannot be suppressed.
- Without `--force`, `/factory-unlock` only succeeds if `current_git_email == holder`.
  Attempting `/factory-unlock` as a non-holder without `--force` exits with an error and
  does not modify STATE.md.

### Decision 6: Acquire/release UX — explicit `/factory-lock` and `/factory-unlock` skills

Lock acquisition and release are **explicit user actions**, not automatic. There is no
auto-acquire-on-first-write.

**`/factory-lock` skill — acquire with CAS push (Fix 1: closes TOCTOU acquire-race):**
- Performs a `git fetch origin factory-artifacts` to get the current remote state.
- Reads the current `factory_lock` block from the just-fetched local STATE.md.
- If locked by another developer (unexpired): exits with the refusal message (Decision 4).
- If unlocked or expired: delegates to `state-manager` to write
  `factory_lock = { holder: <my_email>, locked_at: <now>, expires_at: <now + 45m> }`
  into STATE.md frontmatter, commit, and push **using the same fetch-then-CAS primitive
  as Decision 8**:

  ```bash
  git -C .factory fetch origin factory-artifacts
  EXPECTED_SHA=$(git -C .factory rev-parse origin/factory-artifacts)
  git -C .factory push --force-with-lease=factory-artifacts:"${EXPECTED_SHA}" origin factory-artifacts
  ```

  If the push is rejected (non-zero exit, non-fast-forward), the acquire fails: another
  session acquired between our fetch and our push (a TOCTOU acquire-race per CWE-367).
  The skill exits with an actionable error: "Acquire failed — concurrent lock write
  detected. Fetch and retry."

- Emits `factory.lock.acquired` event on success.
- The burst heartbeat path (mid-burst `expires_at` renewal by `state-manager`) is
  invoked automatically inside each burst after the lock is held.

**Residual TOCTOU window (honest statement):** The CAS push closes the primary acquire
race: two developers who both see an unlocked STATE.md and both attempt acquire will have
one succeed and one receive a non-fast-forward rejection. The remaining residual window is
the **exact-simultaneity-before-either-push** scenario: two sessions that complete the
fetch step before either executes the push. In this window the `--force-with-lease` check
is the tiebreaker — one push will land and one will be rejected. The rejected session
retries from the top of the acquire flow. This window is a **TOCTOU acquire-race
(CWE-367)** that is narrowed but not eliminated by the CAS push; it is accepted as
residual because the window is bounded to the fetch→push interval (milliseconds under
normal conditions) and the cooperative threat model does not require zero-window
exclusivity. The git-ref CAS future path (Decision 9) eliminates this window entirely.

**`/factory-unlock` skill:**
- Without `--force`: only the current holder (`current_git_email == holder`) may release.
  Clears `factory_lock` block from STATE.md, commits, pushes. Emits `factory.lock.released`.
- With `--force`: any developer may release. Emits `factory.lock.stolen` audit event
  (Decision 5 Path B). Clears the block, commits, pushes.

**Rationale for explicit acquire:** "the user that locked it" is the correct mental model.
Auto-acquiring on first write would mean a crash before the first write leaves no lock,
and a developer might not know they own it. Explicit acquire makes the session boundary
clear, mirrors the `git stash` / `git commit` UX pattern (deliberate state transitions),
and avoids surprise: the developer knows exactly when they take ownership.

### Decision 7: Crash behavior — `on_error = "continue"` (fail-open)

The `verify-factory-lock` plugin's `on_error` field in `hooks-registry.toml` is set to
`"continue"`.

**Rationale:** fail-open is correct here because this is an **advisory/efficiency-class
lock** (Kleppmann §8: "efficiency" — avoiding unnecessary work by two parties — vs
"correctness" — preventing data corruption that cannot be fixed). Per Kleppmann's
distinction, efficiency-class locks can safely fail open because the consequence of a
missed block is duplicated work or a detected push collision (caught by Decision 8's CAS
push), not silent data corruption. Decision 8's `--force-with-lease` push is the
independent safety net that bounds the worst-case outcome of a guard crash to a detected
conflict rather than a silent clobber.

A crashing lock-checker that blocks all writes (`on_error = "block"`) is the stale-lock
footgun in a different costume: a broken guard permanently wedges the factory until the
plugin is fixed or the registry is manually edited. The cost of a false-positive (blocked
write due to guard crash) exceeds the cost of a false-negative (missed guard due to
crash) for this threat model.

Guard crashes are surfaced as advisory log events via `internal.dispatcher_error` (SS-03)
so developers are aware without being blocked.

Existing precedent: `validate-artifact-path.wasm` also uses `on_error = "continue"` for
the same reason (ADR-016).

### Decision 8: Complementary mitigation — blind-push fix in `state-burst` (secondary, standalone)

The blind push at `skills/state-burst/SKILL.md` (the `git push origin factory-artifacts`
call) MUST be changed to a **fetch-then-CAS push** regardless of the lock primitive:

```bash
git -C .factory fetch origin factory-artifacts
EXPECTED_SHA=$(git -C .factory rev-parse origin/factory-artifacts)
git -C .factory push --force-with-lease=factory-artifacts:"${EXPECTED_SHA}" origin factory-artifacts
```

This is a **separate, complementary mitigation** — not the primary enforcement mechanism.
It is independently deliverable and independently valuable: even without the WASM guard,
converting the blind push to `--force-with-lease` means that a concurrent push from
another developer causes a detected collision (non-zero exit, actionable error) rather
than a silent clobber. It is also the safety net for (a) the self-vs-self case that the
coarse identity (Decision 3) intentionally does not guard, and (b) guard-crash fail-open
scenarios (Decision 7) — if the guard misses a block, the push-layer CAS still rejects
concurrent writes.

This change is confirmed already allowed by `hooks/verify-git-push.sh` (which only blocks
raw `--force`; `--force-with-lease` is permitted).

This same CAS primitive is reused by the `/factory-lock` acquire path (Decision 6) at
zero additional cost.

### Decision 9: Future / Out of Scope — git-ref CAS upgrade path

The git ref `refs/factory-lock/<repo-slug>` compare-and-swap mechanism is a **future
upgrade path**, not a current deliverable.

It is the correct choice if the threat model escalates to high-velocity teams where
explicit `/factory-lock` coordination breaks down, or where session-level identity is
required, or where the residual TOCTOU acquire-race (Decision 6) or mid-burst self-eviction
residual risk (Decision 5) must be fully eliminated. This path also provides the fencing
token (monotonic object-id chain) that the current design lacks.

When this path is pursued, it requires an empirical GitHub.com server-side CAS verification
probe (research flags that GitLab historically did not enforce strict ref CAS; GitHub.com
behavior must be confirmed before relying on it). That probe is **not a blocking step for
the current implementation** because we are not relying on server-side CAS in v1.

### Decision 10: Single-developer behavior — hard invariant, no added human action

Single-developer single-session use of the factory is the primary case and MUST be
unaffected in the following sense: a developer who has run `/factory-lock` to acquire the
lock will see zero friction during normal operation. The guard passes all checks silently.
No additional human actions are required between lock acquisition and release.

Observable changes for a single developer:
- Running `/factory-lock` once at the start of a pipeline session (new deliberate step).
- `Factory lock: HELD by this session (expires <time>)` line in `/factory-health` output.
- Running `/factory-unlock` once at the end of the session (or letting it auto-expire).

The guard adds negligible latency per hook invocation: one `host::read_file` call on
STATE.md (a small local file) plus one `host::exec_subprocess` call to `git config
user.email` plus one timestamp comparison. Both calls are local (no network). The latency
budget constraint is well within ADR-020 Class A (p95 ≤ 1500ms for the hook chain).

A developer who does not run `/factory-lock` is in the same position as today: the guard
reads `factory_lock: null` and passes all checks. The lock is opt-in; absence of a lock
record is treated as unlocked.

### Decision 11: Automatic heartbeat renewal enforcement — executable skill step + PreToolUse push gate

The mid-burst `expires_at` renewal obligation (Decision 5 / BC-5.40.001 PC4) is enforced
by two complementary mechanisms, not by agent-remembered prose alone. Prior to this
decision, `state-manager.md` §"factory_lock Write/Renewal/Clear Obligation" documented the
requirement to call `factory-lock-write.sh renew` before each burst commit, but the
`state-burst` SKILL itself never invoked it. An agent that followed state-manager.md prose
but not the skill step — or ran the skill without loading the obligation section — would
silently miss the renewal, allowing the lock to self-evict mid-burst.

**Mechanism 1 — Executable `state-burst` step (Option A):**

The `state-burst` SKILL (`plugins/vsdd-factory/skills/state-burst/SKILL.md`) MUST include
`factory-lock-write.sh renew .factory/STATE.md` as a mandatory numbered step immediately
before the `git -C .factory add -A` / `git commit` block. The call is unconditional: when
no lock is held (absent `factory_lock:` key), the script exits 0 with "no factory_lock
block present — renew is a no-op" — zero friction on the common case. This converts PC4
from a prose obligation to a mechanically-invocable step executed every time the burst
skill is followed.

The `factory-lock-write.sh` script (`plugins/vsdd-factory/bin/factory-lock-write.sh`,
delivered by S-17.01) already implements the `renew` subcommand with a RenewalMissed
guard, post-renew assertion, and CRLF normalization. No new script is required.

**Mechanism 2 — `verify-lock-renewal.sh` PreToolUse gate (Option C):**

A new bash hook `plugins/vsdd-factory/hooks/verify-lock-renewal.sh`, registered in
`hooks-registry.toml` as `PreToolUse` / Bash / `on_error = "continue"` / `async = false`,
provides fail-closed enforcement at the push boundary. At PreToolUse on any Bash command
that invokes the factory-artifacts push, the gate:

1. Checks whether the Bash tool-input command triggers the push. The gate fires if
   `.tool_input.command` **contains `factory-cas-push`** (the canonical helper that
   `state-burst` SKILL runs — `bash plugins/vsdd-factory/bin/factory-cas-push.sh` — and
   which contains the real `git push --force-with-lease` as a subprocess invisible to
   PreToolUse) **OR** if `.tool_input.command` matches `git`+`push`+`factory-artifacts`
   (belt-and-suspenders for any hand-typed raw push). Both patterns are evaluated in order;
   either match triggers the gate. Any Bash command that matches neither pattern returns
   exit 0 (Continue) immediately — non-push commands add zero overhead.

   **Rationale for `factory-cas-push` as the primary trigger:** the v1.4 spec used only
   `git.*push.*factory-artifacts` as the trigger pattern. That pattern is inert on the
   production push path because `state-burst` post-S-17.01 runs `bash factory-cas-push.sh`,
   and the real `git push --force-with-lease=factory-artifacts:...` is a subprocess inside
   that helper. PreToolUse only inspects the top-level Bash command string, not subprocesses.
   A gate keyed solely on the raw `git push` pattern NEVER fires on the SKILL's canonical
   push path — enforcement is functionally inert. The primary trigger must therefore match
   the helper script name.

2. Reads `factory_lock.holder` and `factory_lock.expires_at` from the local committed HEAD:
   `git -C .factory show HEAD:STATE.md`. At PreToolUse time the commit already exists
   locally (the `git commit` ran before the push Bash command fires), so HEAD reflects the
   staged renew if Mechanism 1 was followed. The check firing at `bash factory-cas-push.sh`
   PreToolUse is valid: the burst commit was already composed at this point, so HEAD
   STATE.md carries this burst's `expires_at`.
3. If `factory_lock.holder` is absent in HEAD (factory unlocked): returns exit 0. No-op.
4. If `origin/factory-artifacts` does not exist (first push to a new branch): returns exit 0.
5. Reads `factory_lock.expires_at` from the remote tip:
   `git -C .factory show origin/factory-artifacts:STATE.md`.
6. If HEAD `expires_at` equals `origin/factory-artifacts` `expires_at` (the value was NOT
   refreshed in this burst's commits): returns exit code 2 (block) with the message:

   ```
   BLOCKED by verify-lock-renewal: RenewalMissed — factory_lock held but expires_at not refreshed in this burst. Fix: Run: factory-lock-write.sh renew .factory/STATE.md Code: RenewalMissed.
   ```

   **Message form rationale:** The gate runs via `legacy-bash-adapter.wasm`, which truncates
   plugin output to the **first line of stdout** before surfacing it as the block message.
   A multi-line message is therefore unreachable — only the first line is shown to the
   developer. The single-line `block_pre`-form above is the correct contract: it names the
   gate, the error code, the human-readable cause, and the fix command on one line. This is
   the same first-line-truncation constraint that governs all bash-adapter gates (e.g.,
   `verify-git-push.sh`).
7. If HEAD `expires_at` differs from remote (renewal was committed): returns exit 0.

**Why PreToolUse, not PostToolUse:**

PostToolUse fires after the push has already executed — it can flag but cannot block.
PreToolUse fires before the push runs, allowing a hard block. This is the same trigger
point as `verify-git-push.sh` (PreToolUse / Bash), which guards `factory-artifacts`
pushes using the same legacy-bash-adapter pattern. The gate mirrors that pattern exactly.

**`on_error = "continue"` rationale:**

Consistent with Decision 7: an efficiency-class lock's guard crash must not wedge the
factory. A broken gate that permanently blocks all pushes is a worse failure mode than
a missed renewal, which is bounded by the TTL auto-expiry (Decision 5 Path A). Fail-open
on crash; the audit trail via `internal.dispatcher_error` (SS-03) surfaces the crash
without blocking the developer.

**`async = false` requirement:**

Same as Decision 2 / Decisions 1 rationale: only sync-group plugins participate in the
`block_intent` aggregation at `executor::execute_sync_group` (ADR-019). An async plugin's block
signal is advisory-only and would silently reduce the gate to telemetry.

**BC-5.40.001 PC4 unaffected:**

This decision implements BC-5.40.001 PC4 ("state-manager MUST refresh
`factory_lock.expires_at = now + 45 minutes` at every intermediate burst commit, atomic
with the commit"). PC4's postcondition text is correct and complete as written; no BC
amendment is required.

**SUPERSESSION NOTE (v1.6):** Decision 11 Mechanism 2 (the `verify-lock-renewal.sh`
PreToolUse bash gate, Deliverables D11 / D12-registry-entry / D14) is superseded by
Decision 12 (the `verify-state-timestamp-refresh` WASM PreToolUse guard). Mechanism 1 (the
`state-burst` SKILL renew step, Deliverable D10) is retained unchanged — it is the
mechanism that *performs* the renewal; the WASM guard is the mechanism that *enforces*
it happened at write-time. The supersession does NOT remove D10 from scope.

Push-time enforcement (a renewal gate on `factory-cas-push.sh`) is dropped entirely.
With freshness guaranteed at write-time by the WASM guard, the committed STATE.md always
carries a current heartbeat by the time it is pushed — `factory-cas-push.sh` needs no
renewal gate and remains a plain CAS push.

The v1.5 Decision 11 body above documents the design rationale for the PreToolUse bash
approach and its four bypass vectors; it is preserved as historical record. Implementers
MUST NOT build the PreToolUse bash gate (D11/D12-registry/D14 withdrawn). Implementers
MUST build the WASM guard per Decision 12.

### Decision 12: `verify-state-timestamp-refresh` Rust WASM PreToolUse guard (v1.6)

The hook SDK exposes exactly three outcomes: `Continue` (exit 0), `Block` (exit 2),
`Error` (exit 1). There is no mutate/write-content outcome. Therefore "update the time on
every STATE.md touch" is implemented as: **state-manager writes the fresh time (Mechanism 1,
D10), and a WASM PreToolUse guard blocks the write if the time was not refreshed.** This is
the exact shape of the existing `verify-factory-lock` plugin and follows the established
VSDD Rust hook pattern precisely.

#### 12.1 Plugin identity and trigger

**Plugin name:** `verify-state-timestamp-refresh`
**Crate:** `crates/hook-plugins/verify-state-timestamp-refresh/`
**Compiled to:** `plugins/vsdd-factory/hook-plugins/verify-state-timestamp-refresh.wasm`
**Event:** `PreToolUse`
**Tools:** `Edit`, `Write`, and `MultiEdit` — all three tools that mutate file content
**Trigger condition:** `tool_input.file_path` in the payload, after applying the
normalization steps in §12.7 R6, either EQUALS `.factory/STATE.md` exactly OR ENDS WITH
`/.factory/STATE.md`. Any payload whose `file_path` does not satisfy either condition
returns `Continue` immediately without reading any file.

**Why suffix/equality, not exact equality alone:** Claude Code's Edit, Write, and
MultiEdit tools emit the `file_path` as an **absolute path** (e.g.
`/Users/alice/project/.factory/STATE.md`) — verified in dispatcher logs across 5,235+
captured PreToolUse events. The WASI sandbox has no access to env vars (see §12.7 R6 for
rationale), so the prior `$CLAUDE_PROJECT_DIR` strip was dead code. The suffix check
`ends_with("/.factory/STATE.md")` handles absolute paths with zero env dependency and
zero capability requirement.

**`tool_input.file_path` is always the correct field name.** Claude Code's Write, Edit, and
MultiEdit tools all use `file_path` (not `path`). The dispatcher log confirms: `file_path`
appears in 5,235 captured PreToolUse events; the field `new_content` appears zero times.
The payload structure is `serde_json::Value` (no `deny_unknown_fields`) — there is no parse
error on an unknown field, it simply returns `None` when accessed. An implementation that
reads `tool_input.new_content` will hit the fail-open branch on every real write and the
guard will be a production no-op. Do NOT use `new_content`.

This trigger is structurally bypass-proof: the hook payload field `file_path` is set by
the Claude Code tool infrastructure, not by user command text. There is no Bash command
string to tokenize, no regex to defeat, and no subprocess whose inner invocations are
invisible. All four bypass vectors that afflicted Decision 11 Mechanism 2 (inert-match,
over-match, newline-injection, env-injection) never arise here — the trigger is a
structured field, not a free-text command string.

**Scope: `.factory/STATE.md` only.** The broader factory-artifacts mutation-protection
stays with the existing `verify-factory-lock` guard (Decision 1/2). This new guard's path
scope is exactly one file: `.factory/STATE.md`. Do not widen it.

**Priority ordering with verify-factory-lock:** Both plugins fire on `Edit|Write` to
`.factory/STATE.md`. `verify-factory-lock` must run first (identity check precedes
timestamp check). Assign explicit priorities in the registry: `verify-factory-lock = 142`,
`verify-state-timestamp-refresh = 143`. Lower numbers fire first; same-priority entries
run in parallel. Without explicit priorities, both entries inherit the default (500) and
run in parallel — the ordering is undefined. The registry currently shows no `priority =`
line in the `verify-factory-lock` entry, meaning it inherits 500. Both entries MUST be
given explicit priorities in D16 to make the ordering well-defined.

#### 12.2 What the guard enforces on every STATE.md write

The guard reads **two sources** and compares them:

1. **Proposed full content:** reconstructed from the tool payload depending on tool type
   (see Proposed-content extraction table below).
2. **Current on-disk content:** `.factory/STATE.md` read via `host::read_file`.

**Proposed-content extraction by tool type:**

| Tool | Payload fields | How to obtain proposed full content |
|------|---------------|--------------------------------------|
| `Write` | `tool_input.content` (full file body) + `tool_input.file_path` | Use `tool_input.content` directly — it is the complete new file content |
| `Edit` | `tool_input.old_string` + `tool_input.new_string` (fragment) + `tool_input.file_path` + optional `tool_input.replace_all` (bool, default false) | Read on-disk content via `host::read_file`; replace first occurrence of `old_string` with `new_string` (or all occurrences if `replace_all` is true) to produce proposed content. If `old_string` is not found in on-disk content → **Continue** (fail-open: the tool itself will reject the edit; not the guard's job to duplicate that check) |
| `MultiEdit` | `tool_input.edits[]` (array of `{old_string, new_string, replace_all?}`) + `tool_input.file_path` | Read on-disk content; apply each element of `edits[]` sequentially in array order, same substitution logic as Edit. If any `old_string` is not found → **Continue** (fail-open; same rationale) |

**Why reconstruction is required (not optional):** Edit and MultiEdit deliver only a
fragment in the payload — there is no full-file field. The guard MUST reconstruct the
full proposed file by applying the edit to the on-disk content. Without reconstruction,
the guard can only check the fragment, which will never contain the `timestamp:` or
`factory_lock.expires_at` lines (those are in the frontmatter, which is typically NOT
the fragment being edited). An implementation that only checks the fragment will always
fail to find the timestamp fields and will silently fail-open on every Edit — making the
guard a no-op for the most common STATE.md mutation path.

> **ADR-032 supersedes-in-part (v1.24):** ADR-032 implements payload-targeted enforcement,
> which is NOT the fragment-only approach the warning above describes. Rather than checking
> the fragment for the timestamp, ADR-032 scans the `new_string` values to determine whether
> the Edit explicitly sets `timestamp:` OR `factory_lock:` at column 0. If YES to either
> field, full reconstruction and comparison (this section) applies; a `factory_lock:`-only
> Edit skips Steps 4–6 (timestamp check) but still runs Step 7 (lock-expiry check). If NO
> to both (payload-neutral — neither field set in any `new_string`), the Edit returns
> `Continue` after skipping the timestamp and lock-expiry checks — no reconstruction, no
> fragment check. The §12.2 warning targets a hypothetical implementation that treats ALL
> Edits as fail-open regardless of payload; ADR-032's payload-targeted approach closes that
> gap for explicit-field Edits while eliminating spurious blocks for body-only Edits.

**Time fields extracted from both sources** (see §12.4 for robust extraction spec):

| Field | Location | Condition checked |
|-------|----------|-------------------|
| `timestamp:` | Top-level frontmatter (between first `---` fences) | Proposed string value MUST differ from on-disk value (every STATE.md write must advance this field) |
| `factory_lock.expires_at` | Nested under `factory_lock:` in frontmatter | **Only when** `factory_lock.holder` is present and non-empty in the proposed content (lock held): proposed value MUST be present, non-empty, AND differ from on-disk value. Absent OR empty proposed `expires_at` while lock is held → Block LockExpiryStale (a held lock with no expiry is not a valid renewal, regardless of whether the on-disk value differs). |

**"Differ"** for `timestamp:` means the extracted string values are not byte-for-byte
identical. The guard does NOT parse values as datetimes — string inequality is sufficient
and avoids ISO-8601 edge-case parsing failures being misused as a bypass. The full datetime
semantics are enforced by `factory-lock-write.sh renew` (Mechanism 1, D10). The guard's
job is to detect "value did not change or is missing", not "value is correctly formatted".

**Clarity note — `timestamp:` vs `last_amended:` scoping:** `timestamp:` is the sole
field independently gated by this guard. `last_amended:` is a freeform human-readable
string advanced by state-manager under the same POLICY 14 write discipline, but it is NOT
independently compared by this guard — its format is not machine-comparable (free text
starting with a date). A STATE.md write that advances `timestamp:` but leaves
`last_amended:` unchanged will pass this guard; state-manager's obligation to update
`last_amended:` is enforced by POLICY 14 SKILL discipline, not by this hook.

**Canonical block message format** (using `HookResult::block_with_fix` from
`crates/hook-sdk/src/result.rs`):

The `block_with_fix` constructor signature is:
```rust
pub fn block_with_fix(hook: &str, reason: impl AsRef<str>, recommendation: impl AsRef<str>, code: &str) -> Self
```
It formats to: `BLOCKED by {hook}: {reason}. Fix: {recommendation}. Code: {code}.`
The `reason` segment MUST be human-readable text WITHOUT the code value embedded in it.

1. **TimestampStale:**
   ```rust
   HookResult::block_with_fix(
       "verify-state-timestamp-refresh",
       "STATE.md timestamp not advanced in this write",
       "Update `timestamp:` to the current UTC time before writing STATE.md",
       "TimestampStale",
   )
   ```
   Output: `BLOCKED by verify-state-timestamp-refresh: STATE.md timestamp not advanced in this write. Fix: Update 'timestamp:' to the current UTC time before writing STATE.md. Code: TimestampStale.`

2. **LockExpiryStale** (only when lock held in proposed content):
   ```rust
   HookResult::block_with_fix(
       "verify-state-timestamp-refresh",
       "factory_lock.expires_at not refreshed in this write while lock is held",
       "Run: factory-lock-write.sh renew .factory/STATE.md before writing STATE.md",
       "LockExpiryStale",
   )
   ```
   Output: `BLOCKED by verify-state-timestamp-refresh: factory_lock.expires_at not refreshed in this write while lock is held. Fix: Run: factory-lock-write.sh renew .factory/STATE.md before writing STATE.md. Code: LockExpiryStale.`

**The `[hook] Code: …` bracket form used in the prior draft of AC-005/006 strings is NOT
what `block_with_fix` produces and MUST NOT appear in the implementation or the AC text.**
The correct emitted form is the `BLOCKED by …` line above. The product-owner must correct
AC-005 and AC-006 strings to match this format (see AC-correction directive, §12.7).

#### 12.3 Fail-open vs fail-closed decisions

> **ADR-032 supersedes-in-part (v1.23):** For `Edit` and `MultiEdit` tools, ADR-032 Decision 1
> adds a payload-scan step that fires BEFORE the §12.3 rows are evaluated. If no `new_string`
> in the payload sets EITHER `timestamp:` OR `factory_lock:` (payload-neutral — neither field),
> the guard returns `Continue` after skipping the timestamp and lock-expiry checks, bypassing all §12.3 rows for that invocation. The §12.3 rows below
> remain authoritative with **split scope by row class**: (a) **TimestampStale rows** — gated on
> `sets_timestamp` (Edit/MultiEdit where at least one `new_string` sets `timestamp:` at column 0;
> all Write operations unchanged); (b) **LockExpiryStale rows** — gated on
> `(sets_timestamp OR sets_factory_lock)` per ADR-032 Decision 3 (Edit/MultiEdit where any
> `new_string` sets `timestamp:` or `factory_lock:`; all Write operations unchanged). A
> factory_lock-only Edit (`sets_timestamp == false`, `sets_factory_lock == true`) bypasses
> TimestampStale rows entirely (ADR-032 Steps 4–6 are skipped) but still triggers
> LockExpiryStale rows (Step 7 runs).

| Situation | Outcome | Rationale |
|-----------|---------|-----------|
| `file_path` does not resolve to `.factory/STATE.md` (after canonical-path normalization per §12.7 R6) | **Continue** immediately (no `host::read_file` called) | Out of scope; non-STATE.md writes are not subject to this guard |
| `tool_input.file_path` field absent or null in payload | **Continue** (fail-open) | Structurally unexpected; guard cannot identify the target file; err on the side of not blocking |
| On-disk STATE.md `host::read_file` fails (`CapabilityDenied`, `Timeout`, `NotFound`, etc.) | **Continue** + `log_warn` | Consistent with Decision 7 and `verify-factory-lock` PC6. A guard that permanently blocks writes on read-failure is the stale-lock footgun in a different costume. Required for first-ever STATE.md creation (file does not exist yet → `host::read_file` returns NotFound → Continue). |
| `Edit` or `MultiEdit`: `old_string` not found in on-disk content | **Continue** (fail-open) | The tool itself will reject the edit; guard's job is timestamp enforcement, not edit-applicability validation |
| On-disk frontmatter unparseable (malformed YAML fences or timestamp field) | **Continue** + `log_warn` | No valid prior value to compare against; consistent with `verify-factory-lock` MalformedLockBlock pattern |
| Proposed content frontmatter unparseable (malformed) | **Continue** + `log_warn` | Guard cannot determine the proposed timestamp; consistent with fail-open error policy |
| `timestamp:` absent in on-disk content (first write ever, or on-disk has no frontmatter) | **Continue** | No prior value to compare against; any write is valid |
| `timestamp:` absent in proposed content (state-manager omitted the field) | **Block: TimestampStale** | Every STATE.md write is required to include `timestamp:`. Absence of the field in the proposed write is itself a timestamp-not-advanced violation. |
| `timestamp:` present in both and byte-identical | **Block: TimestampStale** | Core enforcement: the timestamp was not advanced |
| `timestamp:` present in both and different | Continue (for this check) | Timestamp was advanced; proceed to LockExpiryStale check if applicable |
| No lock held in proposed content (`factory_lock` absent or `factory_lock.holder` absent/empty) | Skip LockExpiryStale check; `TimestampStale` check still applies | Lock is not held; `expires_at` is irrelevant |
| Lock held AND proposed `expires_at` is **absent** (field not present in proposed frontmatter) | **Block: LockExpiryStale** | A held lock with no expiry field is not a valid renewal. Absent is as wrong as stale. |
| Lock held AND proposed `expires_at` is **empty** (field present but value is empty string after extraction) | **Block: LockExpiryStale** | A held lock with an empty expiry is not a valid renewal. Empty is as wrong as stale. |
| Lock held AND proposed `expires_at` is **byte-identical** to on-disk `expires_at` | **Block: LockExpiryStale** | Renewal was not performed; Mechanism 1 (D10) was skipped |
| Lock held AND proposed `expires_at` is present, non-empty, and **different** from on-disk | **Continue** | Renewal was performed; expiry was advanced |
| Guard plugin crashed (`on_error = "continue"`) | **Continue** (fail-open) | Consistent with Decision 7 efficiency-class lock. Crash → advisory `internal.dispatcher_error` record in dispatcher log |

#### 12.4 Robust frontmatter time-field extraction

**Problem:** STATE.md is a YAML-frontmatter document delimited by `---` fences. The
`timestamp:` and `factory_lock.expires_at` fields are the operative time fields. A naive
substring scan (e.g., `lines().find(|l| l.starts_with("timestamp:"))`) can misread:
- A `timestamp:` key inside a nested YAML block that happens to have leading whitespace
- A quoted value: `timestamp: "2026-06-12T00:00:00Z"` — the extracted value would include
  the quotes, causing a false byte-identical comparison if one side is quoted and the other
  is not
- An edge line: `timestamp:   2026-06-12T00:00:00Z` (extra spaces)

**Required extraction algorithm:**

1. **Locate the YAML frontmatter block:** find the first `---` line (line 0 or first
   non-empty line); find the second `---` line; the frontmatter body is the text between
   them. If fewer than two `---` fences exist → unparseable → fail-open (§12.3 row 5).
2. **Extract top-level scalar keys only:** iterate lines in the frontmatter body. A
   top-level key line has the form `^<key>:` with NO leading whitespace (lines with
   leading whitespace are nested keys; skip them for top-level extraction). For a line
   matching `^timestamp:\s*(.+)`, trim whitespace from the capture group, then strip
   surrounding `"` or `'` quote characters (one layer only). The result is the canonical
   timestamp string.
3. **Extract `factory_lock.expires_at`:** use the existing `parse_factory_lock` function
   from `factory-lock-parse` crate (see §12.5) — it already handles the `factory_lock:`
   nested block correctly. Do not re-implement nested YAML parsing.
4. **`last_amended:` field:** this field is a freeform string starting with a date.
   For enforcement purposes, checking only `timestamp:` is sufficient — `last_amended:`
   is human-readable prose, not a machine-comparable value. Do NOT attempt to compare
   `last_amended:` for staleness.

**Key invariant:** the comparison MUST use the same extraction path for both on-disk and
proposed content. If on-disk uses raw-line extraction and proposed uses parsed extraction,
quote normalization differences will cause spurious false-positive blocks. Use the same
`extract_yaml_string_value` function on both sides — it already does quote stripping per
the `factory-lock-parse` implementation.

**Test requirement (D17 addition):** test-writer MUST add a fixture for the quoted
timestamp case:
- on-disk: `timestamp: 2026-06-12T00:00:00Z` (unquoted)
- proposed: `timestamp: "2026-06-12T01:00:00Z"` (quoted)
- Expected: Continue (values differ after normalization, even though one is quoted)

And the false-positive guard:
- on-disk: `timestamp: "2026-06-12T00:00:00Z"` (quoted)
- proposed: `timestamp: "2026-06-12T00:00:00Z"` (same quoted value)
- Expected: Block TimestampStale

#### 12.5 Shared parse logic — no duplication

The guard requires the same `factory_lock` frontmatter parse logic that `verify-factory-lock`
already implements and tests. Rather than duplicating line-by-line scan code in a new crate,
the `parse_factory_lock` function and supporting types (`LockState`, `extract_yaml_string_value`)
from `crates/hook-plugins/verify-factory-lock/src/lib.rs` are extracted to a shared location.

**Decision:** promote `parse_factory_lock`, `LockState`, and `extract_yaml_string_value`
from `verify-factory-lock::lib` to a new workspace-internal crate
`crates/factory-lock-parse/`. Both `verify-factory-lock` and
`verify-state-timestamp-refresh` declare `factory-lock-parse` as a dependency.

**Path rationale:** `factory-lock-parse` is a pure library crate — it has no `[[bin]]`
target and produces no `.wasm` output. It MUST NOT live under `crates/hook-plugins/`
because that directory is the WASM-plugin floor-count gate's scope; a lib-only crate
there inflates the expected WASM count and breaks CI. The correct location is
`crates/factory-lock-parse/`, alongside other non-plugin workspace crates.
The existing `verify-factory-lock` tests continue to pass unmodified — only the import path
changes from `crate::` to `factory_lock_parse::`. This is the production-grade path
(single implementation, single test surface) per CLAUDE.md Rule 1 and the no-duplication
principle. Creating two independent implementations of the same frontmatter scanner violates
this principle; the shared crate is mandatory.

The `timestamp:` field is a top-level YAML scalar key. The guard extracts it using the
same `extract_yaml_string_value` helper already in the shared crate (see §12.4 for the
extraction algorithm). No additional YAML parser is needed.

#### 12.6 Capability block (D16 registry entry)

The guard uses `host::read_file` on `.factory/STATE.md` to read the on-disk content.
It reads the proposed content from the tool payload directly (no host call needed for that).
It does NOT call `host::exec_subprocess`.

**CRITICAL: `ReadFileCaps` struct accepts ONLY `path_allow`.** The dispatcher's
`ReadFileCaps` struct definition in `crates/factory-dispatcher/src/registry.rs` is:

```rust
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ReadFileCaps {
    pub path_allow: Vec<String>,
}
```

The `#[serde(deny_unknown_fields)]` attribute means ANY field not present in the struct
will cause the entire registry file to fail to load. Adding `max_bytes = 65536` or
`timeout_ms = 5000` under `[hooks.capabilities.read_file]` will break the registry load
and render ALL 52 plugins non-operational. The `max_bytes` and `timeout_ms` parameters
exist in the `host::read_file` WASM host ABI call arguments (passed by the WASM code at
call time), but they are NOT registry config fields.

The `HOST_ABI.md` specification shows `read_file(path, max_bytes, timeout_ms)` as call
parameters — they are passed from the plugin code itself, not from the TOML registry.

Required (and complete) capability block for D16:

```toml
[hooks.capabilities.read_file]
path_allow = [".factory/STATE.md"]
```

This is the only permissible form. Compare with the existing `verify-factory-lock` entry's `[hooks.capabilities.read_file]` block (following the `verify-factory-lock` `[[hooks]]` stanza) in `hooks-registry.toml`:

```toml
[hooks.capabilities.read_file]
path_allow = [".factory/STATE.md"]
```

Identical. No `max_bytes`, no `timeout_ms`. This is correct.

The `max_bytes` and `timeout_ms` values are specified in the WASM plugin source code itself
(e.g., `host::read_file(path, 65536, 5000)`), not in the TOML registry. The implementer
MUST NOT add these to the TOML registry entry.

Omitting `path_allow` entirely (or providing an empty list `path_allow = []`) still causes
`CapabilityDenied` → silent fail-open → guard is a no-op. The `path_allow` field must be
present and non-empty.

No `exec_subprocess` capability needed — the guard never shells out to `git` or any other
process. This eliminates the `env_allow` footgun class entirely and makes the registry
entry simpler than `verify-factory-lock`.

#### 12.7 Resolved open questions

**R1 — Scope:** `.factory/STATE.md` only. Triggers on Edit|Write|MultiEdit where
`tool_input.file_path` resolves to `.factory/STATE.md` after canonical-path normalization
(see R6 below). All other file paths return Continue immediately without reading any file.

**R2 — Fail-open/fail-closed:** Documented in §12.3 table. Summary: fail-closed
on the positive stale signal (that is the load-bearing case); fail-open on every error
path (consistent with Decision 7 and `verify-factory-lock` precedent).

**R3 — S-17.04 disposition:** Redirect now. Landing a known-superseded mechanism then
deleting it burns review cycles and ships a defective guard to an rc. The WASM guard
approach is the correct target; the Re-Scope Directive is precise enough for immediate
story-writer dispatch.

**R4 — Force-unlock audit event:** Unchanged. `factory-unlock-decide.sh` continues to
emit decision tokens; the `/factory-unlock` SKILL continues to emit the
`factory.lock.stolen` audit event via `emit-event`. The new WASM guard has no impact on
the unlock path.

**R5 — rc cadence:** The `verify-state-timestamp-refresh.wasm` plugin reaches the
operator cache only at the next rc tag. **rc.21 is HELD** pending S-17.04 and the
associated issue bundle (#128, #130, #169, #176, #170). The WASM guard is S-17.04's
primary deliverable. There is NO pre-rc interim period where the guard is absent but
state-manager is expected to advance timestamps — the guard and the obligation are
co-deployed.

**R6 — WASM-correct path-matching rule (adversary pass 7 P0 rewrite):**

**Root cause of prior failure:** The previous step 1 specified stripping the
`$CLAUDE_PROJECT_DIR/` prefix via `std::env::var("CLAUDE_PROJECT_DIR")`. The WASI
sandbox built by the dispatcher (`WasiCtxBuilder` in `crates/factory-dispatcher/src/invoke.rs`)
uses `preopened_dir` only — it never calls `.env()` or `.inherit_env()`. Therefore
`std::env::var` **always returns `Err` in production**. Claude Code tools emit `file_path`
as an **absolute path** (e.g. `/Users/alice/project/.factory/STATE.md`) — verified across
5,235+ captured PreToolUse events. With step 1 dead, absolute paths never matched the
canonical string `.factory/STATE.md`, and the guard returned `Continue` on every real
Edit/Write/MultiEdit to STATE.md. The guard was inert in production while passing unit
tests only because `#[test]` runs as a native binary where `std::env::set_var` works.

**Why `host::env` + `env_allow` is NOT the fix:** The `host::env` call is capability-gated
via `env_allow`. Omitting a variable name from `env_allow` causes the host function to
silently return nothing — the same deny-by-default silent-no-op footgun class documented
in ADR-025 v1.3 (D-545 class). Depending on `env_allow` recreates the exact dead-branch
failure mode in a new costume. This route is explicitly rejected.

**WASM-correct rule (no env dependency, no additional capability):**

Apply the following normalization steps to the raw `tool_input.file_path` string, then
apply the match test:

1. **Strip a single leading `./`** if present: `"./. factory/STATE.md"` → `".factory/STATE.md"`.
2. **Collapse double slashes**: `".factory//STATE.md"` → `".factory/STATE.md"` (repeat until
   no `//` remains).
3. **Collapse `/./` segments**: `".factory/./STATE.md"` → `".factory/STATE.md"` (repeat until
   no `/./` remains).
4. **Resolve `..` segments (segment-stack algorithm):** split the path on `/`; iterate
   segments left-to-right; push non-empty, non-`.` segments onto a stack; on `..` pop the
   top of the stack (if non-empty — above-root `..` against an empty stack is silently
   discarded); rebuild the path by joining the stack with `/`.

**Match test (after all four normalization steps):**

Trigger the guard if the normalized string EQUALS `".factory/STATE.md"` exactly,
OR ENDS WITH `"/.factory/STATE.md"`.

Do NOT apply any further stripping. The `ends_with` branch handles all absolute-path
forms regardless of the leading prefix — no knowledge of the project root is needed.

**Worked examples:**

| Raw `file_path` | After normalization | Match? | Outcome |
|---|---|---|---|
| `".factory/STATE.md"` | `".factory/STATE.md"` | equals | **triggers guard** |
| `"./.factory/STATE.md"` | `".factory/STATE.md"` (strip `./`) | equals | **triggers guard** |
| `"/Users/alice/project/.factory/STATE.md"` | unchanged (no `./`/`//`/`/./`/`..`) | ends_with `/.factory/STATE.md` | **triggers guard** |
| `"/home/ci/repo/.factory//STATE.md"` | `/home/ci/repo/.factory/STATE.md` | ends_with `/.factory/STATE.md` | **triggers guard** |
| `"/repo/x/../.factory/STATE.md"` | `/repo/.factory/STATE.md` (step 4) | ends_with `/.factory/STATE.md` | **triggers guard** |
| `".factory/STATE.md.bak"` | `".factory/STATE.md.bak"` | no match | Continue |
| `"other/STATE.md"` | `"other/STATE.md"` | no match | Continue |
| `".factory/state.md"` (wrong case) | `".factory/state.md"` | no match | Continue |

**Cross-project false-block analysis:** A suffix match could theoretically match a
DIFFERENT repo's `…/.factory/STATE.md`. This is acceptable: the dispatcher runs
per-project and `host::read_file` reads THIS project's `.factory/STATE.md` via the
`path_allow = [".factory/STATE.md"]` capability (relative path). Blocking a stale write
to any path ending in `/.factory/STATE.md` is fail-toward-enforcement — the worst case is
a spurious block on a hypothetical write to a nested project's STATE.md from within this
project's Claude Code session, which is not a supported usage pattern. This risk is judged
negligible against the certainty of the guard being permanently inert under the prior rule.

**Direction of residual misses is always fail-open (never false-block).** Any path that is
not valid UTF-8, or reduces to the empty string after normalization, MUST Continue
(fail-open). The threat model is a cooperative state-manager; a transient evasion is a
better failure mode than a false-block that wedges the factory.

**Absolute-path bats e2e test is MANDATORY (see §12.9).** Native-env unit tests using
`std::env::set_var` do NOT validate the WASM trigger — the WASI sandbox does not inherit
env vars, so a test that works in the native binary can mask a dead branch in the compiled
WASM. The absolute-path code path MUST be exercised through the actual `wasmtime` runtime
via a bats integration test.

**R7 — Priority ordering (H02 resolution):** Resolved in §12.1. Explicit priority values
are mandated: `verify-factory-lock = 142`, `verify-state-timestamp-refresh = 143`. The D2
registry entry for `verify-factory-lock` (in the D16 spec section) MUST be updated to add
`priority = 142` if it is not already present. The D16 registry entry for
`verify-state-timestamp-refresh` MUST include `priority = 143`. This is a required change
to D2 as well as D16.

**R8 — Block message format (H04 resolution):** Resolved in §12.2. The canonical emitted
form is the `BLOCKED by {hook}: {reason}. Fix: {recommendation}. Code: {code}.` single line
from `HookResult::block_with_fix`. The `[hook] TimestampStale: …` bracket form that
appeared in the AC-005/006 strings is not what `block_with_fix` produces and must be
corrected in the ACs (see §12.8 AC-correction directive).

#### 12.8 AC-correction directive for product-owner (adversary pass 1 findings)

The following ACs in story S-17.04 contain incorrect content that must be corrected before
implementation. Product-owner owns these corrections; this directive is an architect
finding routed to the correct specialist per CLAUDE.md Companion Principle.

**AC-005 (TimestampStale block message string):**
- Current (incorrect): `[verify-state-timestamp-refresh] TimestampStale: STATE.md timestamp not advanced`
- Correct: `BLOCKED by verify-state-timestamp-refresh: STATE.md timestamp not advanced in this write. Fix: Update 'timestamp:' to the current UTC time before writing STATE.md. Code: TimestampStale.`
- Root cause: `block_with_fix` emits `BLOCKED by {hook}: …` format, not `[hook] Code: …` bracket format

**AC-006 (LockExpiryStale block message string):**
- Current (incorrect): `[verify-state-timestamp-refresh] LockExpiryStale: factory_lock.expires_at not refreshed`
- Correct: `BLOCKED by verify-state-timestamp-refresh: factory_lock.expires_at not refreshed in this write while lock is held. Fix: Run: factory-lock-write.sh renew .factory/STATE.md before writing STATE.md. Code: LockExpiryStale.`
- Root cause: same as AC-005

**AC-010 (registry entry for `verify-state-timestamp-refresh`):**
- Current (incorrect): capability block contains `max_bytes = 65536` and `timeout_ms = 5000` fields
- Correct: capability block MUST be `path_allow = [".factory/STATE.md"]` ONLY — no other fields
- Root cause: `ReadFileCaps` is `#[serde(deny_unknown_fields)]` with only `path_allow: Vec<String>`; extra fields break registry load
- Also add: `priority = 143` in the plugin entry (not in the capabilities block — at the entry level)
- Also add: `priority = 142` to the existing `verify-factory-lock` registry entry (amendment to D2 deliverable)

**EC-006 (canonical-path matching rule):**
- Current: absent or underspecified
- Correct (updated per adversary pass 7): add an EC or clarifying note specifying the WASM-correct canonical-path rule from §12.7 R6: strip leading `./`; collapse `//`; collapse `/./`; resolve `..` via segment-stack; then trigger if result EQUALS `".factory/STATE.md"` OR ENDS WITH `"/.factory/STATE.md"`. No env vars read. The `$CLAUDE_PROJECT_DIR` prefix strip is REMOVED — it was dead code in the WASM sandbox.

**New ACs for Write/Edit/MultiEdit coverage (proposed additions — product-owner decides exact AC numbering):**
- AC-NEW-WRITE: When the guard receives a Write tool payload for `.factory/STATE.md` with `tool_input.content` containing an unchanged `timestamp:` → Block TimestampStale
- AC-NEW-EDIT: When the guard receives an Edit tool payload for `.factory/STATE.md` with `tool_input.old_string` + `tool_input.new_string` that, after applying to on-disk content, produces unchanged `timestamp:` → Block TimestampStale
- AC-NEW-MULTIEDIT: When the guard receives a MultiEdit tool payload for `.factory/STATE.md` with `tool_input.edits[]` that, after sequential application, produces unchanged `timestamp:` → Block TimestampStale
- AC-NEW-NOOP-EDIT: When `old_string` is not found in on-disk content (Edit or MultiEdit) → Continue (fail-open; the tool itself will reject it)

#### 12.9 AC/test delta directive — Gemini cross-family pass (adversary pass 2)

This directive is routed to story-writer (ACs) and test-writer (unit + bats coverage).
It supplements §12.8 (adversary pass 1 AC corrections) with the two new semantics gaps
closed in this clarification pass.

**Gap R2 — lock-held + absent/empty `expires_at` → Block:**

Story-writer MUST add two ACs (exact numbering: product-owner assigns):
- AC-NEW-LOCK-ABSENT-EXPIRY: When a lock is held in proposed STATE.md content
  (`factory_lock.holder` present and non-empty) AND the proposed `factory_lock.expires_at`
  field is **absent** (not present in frontmatter) → Block LockExpiryStale.
  Canonical block message: `BLOCKED by verify-state-timestamp-refresh: factory_lock.expires_at not refreshed in this write while lock is held. Fix: Run: factory-lock-write.sh renew .factory/STATE.md before writing STATE.md. Code: LockExpiryStale.`
- AC-NEW-LOCK-EMPTY-EXPIRY: When a lock is held AND the proposed `factory_lock.expires_at`
  field is **present but empty** (value is empty string after extraction) → Block
  LockExpiryStale (same message as above).

Test-writer MUST add two unit test cases to D17 (in addition to the 19 cases already
specified):
- **(t)** Lock held + proposed `expires_at` absent → Block LockExpiryStale
- **(u)** Lock held + proposed `expires_at` present but empty (`expires_at: ""`) → Block
  LockExpiryStale

Test-writer MUST also add one bats integration test: lock-held + absent-expires path →
`exit 2` with canonical LockExpiryStale message.

**Gap R4 — `..` traversal paths trigger the guard (not fail-open):**

Story-writer MUST update EC-006 (or the canonical-path AC) to state: "Paths containing
`..` segments are resolved using segment-stack algorithm (split on `/`, pop on `..`,
discard leading/trailing empty segments) before comparison. A path that resolves to
`.factory/STATE.md` after `..` resolution triggers the guard identically to a direct
path. A path that cannot be resolved (non-UTF-8, empty result) → Continue (fail-open)."

Test-writer MUST add two unit test cases:
- **(v)** `file_path = "foo/../.factory/STATE.md"` → resolves to `.factory/STATE.md` →
  guard triggers (same as canonical path; if timestamp stale → Block)
- **(w)** `file_path = "../../.factory/STATE.md"` → resolves to `.factory/STATE.md`
  (above-root `..` segments silently discarded) → guard triggers

**Gap R7 (adversary pass 7 P0) — WASM-correct path trigger; absolute-path e2e test:**

Story-writer MUST update EC-006 to reflect the suffix/equality rule:
"The guard triggers when, after normalization steps (strip leading `./`; collapse `//`;
collapse `/./`; resolve `..` via segment-stack), the path EQUALS `.factory/STATE.md` OR
ENDS WITH `/.factory/STATE.md`. No env vars are read. The `$CLAUDE_PROJECT_DIR`-based
prefix strip is removed — it was dead code in the WASM sandbox."

Test-writer MUST add the following bats integration test (MANDATORY — existing unit tests
do NOT cover this because `std::env::set_var` works in native binaries but the WASI
sandbox inherits no env vars):
- **(e2e-abs)** Send a real PreToolUse payload with `file_path` set to an ABSOLUTE path
  ending in `/.factory/STATE.md` (e.g. `/tmp/test-project/.factory/STATE.md`) and stale
  `timestamp:` content through the actual WASM runtime (wasmtime, not native test binary).
  Assert exit code 2 (Block) with canonical TimestampStale message. This test MUST fail
  before the fix (demonstrating the inert guard) and pass after.

**Why native-env unit tests are insufficient:** `std::env::set_var` in a `#[test]`
function sets the var in the native OS process, where `std::env::var` works normally.
The compiled WASM plugin runs in a wasmtime WASI sandbox with no inherited env vars.
A unit test that calls `std::env::set_var("CLAUDE_PROJECT_DIR", …)` gives false
confidence — the test passes in native, the WASM guard silently does nothing in production.
The bats e2e test exercises the actual binary path through the real sandbox.

Implementer directive (separate from story/test-writer above):
- Remove all `std::env::var("CLAUDE_PROJECT_DIR")` calls from `verify-state-timestamp-refresh`
- Replace the prefix-strip step with: after normalization (strip `./`; collapse `//`,
  `/./`; resolve `..`), check `normalized == ".factory/STATE.md" || normalized.ends_with("/.factory/STATE.md")`
- Do NOT add any `env_allow` entry or `host::env` call — capability-free is the correct path
- The D16 registry entry and D15 crate are NOT changed by this fix (no new capabilities needed)

**Lock-identity guard sibling-sweep note (R1-adjacent, implementer in-scope):**

The existing `verify-factory-lock` registry entry has `tool = "Edit|Write|Agent"` and
omits `MultiEdit`. A MultiEdit to `.factory/STATE.md` therefore bypasses the lock-identity
check while being subject to the timestamp-refresh guard. For parity, the implementer MUST
update the `verify-factory-lock` `tool` matcher to `"Edit|Write|MultiEdit|Agent"` as an
in-scope sibling-sweep when adding the `verify-state-timestamp-refresh` registry entry (D16).
This is governed by ADR-025 Decision 1/2 (the lock-identity guard must cover all file-mutating
tools targeting factory-artifacts). No new story required — it is a one-line registry edit
within the same D16 scope.

### Decision 13 — Host ABI NOT_FOUND return code (-5)

`codes::NOT_FOUND = -5` is allocated in the factory-dispatcher `host` module's `codes`
namespace.

**Collision analysis — occupied codes (HOST_ABI_VERSION = 1):**

| Code | Named constant | Meaning |
|------|----------------|---------|
| `0` | `OK` | Success |
| `-1` | `CAPABILITY_DENIED` | Caller lacks required capability block |
| `-2` | `TIMEOUT` | Plugin fuel or wall-time exhausted |
| `-3` | `OUTPUT_TOO_LARGE` | Content exceeds `max_bytes` |
| `-4` | `INVALID_ARGUMENT` | Malformed argument (already occupied — F-P1-001 BLOCKER averted) |
| `-99` | `INTERNAL_ERROR` | Dispatcher-internal fault |

`-5` is the next free code in the compact negative sequence.

**Purpose:** distinguish "file does not exist" (`NOT_FOUND = -5`) from "path rejected by
capability policy" (`CAPABILITY_DENIED = -1`) in the host `read_file` implementation,
closing the `path_allowed()` / `canonicalize()` conflation identified as rc.22 smoke
FINDING-2 (S-19.03). Previously, an absent file returned `CAPABILITY_DENIED -1`, causing
plugins to misclassify a missing-file condition as a policy violation.

**HOST_ABI_VERSION:** remains `1`. Additive code addition; no existing call-site semantics
changed. Callers treating all non-OK `read_file` results as fail-open (Continue) — including
`verify-factory-lock` and `verify-state-timestamp-refresh` — require no update.

**Closes:** F-P1-001 (BLOCKER). **Cites:** S-19.03.

### Decision 14 — verify-factory-lock read-cap 262144 + frontmatter-only parse

Two coordinated changes to the `verify-factory-lock` and `verify-state-timestamp-refresh`
WASM plugins and the shared `factory-lock-parse` crate:

**`STATE_MD_MAX_BYTES` 65536 → 262144**

The prior 64 KiB cap silently truncated on-disk STATE.md during `host::read_file` calls.
Observed STATE.md size at rc.22 smoke: ~90 KB / ~466 lines. Truncation caused the guard to
parse an incomplete frontmatter block and fail-open, bypassing enforcement silently (rc.22
smoke FINDING-1).

Sizing rationale:
- Observed: ~90 KB
- Worst-case bound: 500-line hard cap × ~400 bytes/line ≤ 200 KiB
- 262144 bytes (256 KiB): ≥28% headroom above worst-case bound
- 512 KiB considered and rejected: unnecessary given frontmatter-only parse (see below)

**Frontmatter-only parse (abort after second `---`)**

Guards need only the frontmatter block of STATE.md. The `factory-lock-parse` crate functions
`parse_factory_lock` and `extract_yaml_string_value` MUST abort scanning after encountering
the second `---` line (the closing frontmatter fence). No content beyond that delimiter is
read or evaluated. This eliminates body-size scaling entirely.

**Normative twin:** BC-4.13.001 §Precondition 3 (Phase-A) and §Invariant 9. Any STATE.md structural
change that repositions the closing frontmatter `---` delimiter MUST be coordinated with
this decision.

**Function/module anchors (TD-VSDD-091):** `factory_lock_parse::parse_factory_lock`,
`factory_lock_parse::extract_yaml_string_value`, constant `STATE_MD_MAX_BYTES` in
`crates/hook-plugins/verify-factory-lock/src/lib.rs` and
`crates/hook-plugins/verify-state-timestamp-refresh/src/lib.rs`. No file:line pins.

**Closes:** rc.22 smoke FINDING-1. **Cites:** F-P1-005.

### Decision 15 — Host ABI `read_prefix` additive function

`host::read_prefix` is allocated as an additive host function in the factory-dispatcher host
ABI.

**Signature:** `read_prefix(path: &str, max_bytes: u32, timeout_ms: u32) -> i32`

**Return semantics:**

| Return value | Meaning |
|---|---|
| `> 0` (byte count written to output buffer) | Success; bytes ≤ `max_bytes` |
| `0` | Success; file is empty |
| `NOT_FOUND (-5)` | Path does not exist (per Decision 13) |
| `CAPABILITY_DENIED (-1)` | Path not covered by `read_prefix.path_allow` capability block |
| `INTERNAL_ERROR (-99)` | Dispatcher-internal I/O fault |

**Contract vs `host::read_file`:** `read_file` returns `OUTPUT_TOO_LARGE (-3)` when the
file's byte-length exceeds `max_bytes`, causing WASM plugins to fail-open (Continue).
`read_prefix` NEVER returns `OUTPUT_TOO_LARGE` — if the file exceeds `max_bytes`, the
function truncates the output to exactly `max_bytes` bytes and returns the truncated byte
count. Callers are responsible for detecting truncation when the full-file guarantee
matters; for frontmatter-only parse, the Phase-B `read_prefix` callers
(`verify-factory-lock` and `verify-state-timestamp-refresh`) supply max_bytes=262144 per
§Decision 15 v1.17 adjudicated bound (BC-4.13.001 §Precondition 3 Phase-B as amended in BC-4.13.001 v1.17) — equal to the established STATE.md byte envelope
(BC-4.13.001 §Precondition 3 Phase-A; BC-5.40.001 §Precondition 6), ensuring the `---`
closing delimiter of any on-envelope STATE.md file necessarily falls within the prefix.
**Retraction of 8192 bound (v1.17):** the v1.11 claim that ADR-026 compaction keeps
STATE.md frontmatter under 2 KiB is premise-false. ADR-026 §Decision 7 is a line-count
discipline (≤200 lines post-compaction / ≤500 lines during active cycle), not a byte
bound. The inlined `last_amended` changelog convention (same disease class as the
S-15.03 PRIORITY-A structured-changelog migration deferral) causes frontmatter to
currently occupy ~35,175 bytes (measured 2026-07-16: STATE.md total 178,742 bytes;
closing `---` at byte 35,175; `last_amended` field alone 32,648 bytes). At max_bytes=8192,
`extract_frontmatter` receives a prefix containing no closing `---` delimiter, triggers
its full-input fallback, and `parse_factory_lock` returns `MalformedLockBlock` →
fail-open Continue → guard silently inert. The lock block itself is ~27,000 bytes beyond
the 8192-byte window. 65536 bytes covers the 2026-07-16 measurement but offers no
structural guarantee as `last_amended` grows with each burst. **Root disease anchor:**
frontmatter byte-bloat from inlined `last_amended` changelog strings is anchored to
S-15.03 PRIORITY-A structured-changelog migration; ADR-026 line-discipline does not
constrain bytes and is not a valid basis for a byte-prefix bound.

**Capability block:** WASM plugins require a separate `[hooks.capabilities.read_prefix]`
block (with `path_allow`); absence of this block returns `CAPABILITY_DENIED (-1)` before
any filesystem access. A plugin holding `[hooks.capabilities.read_file]` does NOT
automatically receive `read_prefix` access — the two capabilities are independently
declared in the registry, allowing operators to grant bounded-prefix-read access
independently from full-file-read access (BC-1.17.001 Invariant 3). `timeout_ms` is
passed as a call argument in WASM plugin source code; no TOML timeout field is needed.

**HOST_ABI_VERSION:** remains `1`. Additive function addition; no existing call-site
semantics are changed. Callers that use only `read_file` require no update.

**Primary consumers:** `verify-factory-lock` and `verify-state-timestamp-refresh` WASM
plugins (Decision 1 and Decision 12). Replacing their `host::read_file` calls with
`host::read_prefix` (max_bytes=262144 per adjudicated §Decision 15 v1.17 bound;
BC-4.13.001 §Precondition 3 Phase-B as amended in BC-4.13.001 v1.17) eliminates
all `OUTPUT_TOO_LARGE` risk on large STATE.md files. `STATE_MD_MAX_BYTES` is removed in
its entirety at S-19.07 (BC-4.13.001 Phase-B); the 262144 call-site argument in the
`read_prefix` invocation is the sole read bound post-migration.

**Mirrors:** Decision 13 additive precedent — `codes::NOT_FOUND = -5` was allocated
without a HOST_ABI_VERSION bump because existing callers were unaffected. `read_prefix`
follows the same additive-extension policy: new function, no changed behavior, version
unchanged.

**Cites:** VP-101, VP-095, S-19.01, S-19.03.

### Decision 16 — Host ABI `read_prefix` production path registration gap

`read_prefix` is registered in `setup_linker` (`Linker<HostContext>`, the unit-test
invocation path, `crates/factory-dispatcher/src/host/mod.rs`) but is **absent from
`setup_host_on_store_data`** (`Linker<StoreData>`, the production dispatch path,
`crates/factory-dispatcher/src/invoke.rs`).

**SDK-grounding evidence (literal grep, 2026-07-15):**

```
$ grep -n "read_prefix" crates/factory-dispatcher/src/invoke.rs
(no output — 0 hits confirmed)
```

`proxy_host_imports` in `invoke.rs` ignores the `_host_linker_reference` argument and
calls `setup_host_on_store_data` directly. The `host/mod.rs::setup_linker` registration
of `read_prefix` (added S-19.06) does not propagate to `setup_host_on_store_data`, which
was not updated in S-19.06. Any plugin compiled with a `vsdd::read_prefix` import will
fail at plugin instantiation time on the production dispatch path with a wasmtime link
error (missing import). No operator-visible `read_prefix` call is currently functional.

**Decision:** add `read_prefix` to `setup_host_on_store_data` in
`crates/factory-dispatcher/src/invoke.rs`. The production implementation MUST follow the
memory-grow protocol established for `read_file` in the same function: grow WASM linear
memory by `ceil(body_len / 65536)` pages, write body at `current_bytes` (the prior memory
end, always `> 0` for non-empty body), and return the real write offset via `out_ptr_out`.
Capability enforcement gates on `ctx.capabilities.read_prefix.path_allow`.
For an empty body (file exists, zero bytes) the implementation writes `ptr=0, len=0` —
the same as the `read_file` production path for empty files — which the hook-sdk
`read_owned_bytes` ptr==0 guard handles correctly.

**HOST_ABI_VERSION:** remains `1`. This is a production-path implementation fill for a
function that Decision 15 already allocated; no ABI wire shape changes.

**Deliverable:** D19. **Cites:** Decision 15, S-19.06.

### Decision 17 — Host ABI two-linker `out_ptr=0` protocol boundary

The factory dispatcher has two distinct linker and invocation paths that share the
`vsdd::read_file` (and `vsdd::read_prefix`) import name but differ in their
memory-write protocol:

**Test path** (`Linker<HostContext>`, built by `setup_linker` in `host/mod.rs`):
`host/read_file.rs::register()` uses `write_wasm_bytes(&mut caller, out_ptr, ...)` where
`out_ptr` is the value returned by `read_file::prepare()`, which returns `Ok((bytes, 0))`
— always 0. Data is written to WASM address 0; `out_ptr_out` is set to 0. The hook-sdk
`read_owned_bytes(0, ...)` triggers the `if ptr == 0 || len == 0` guard and returns
`Vec::new()`. This path is for dispatcher-internal unit tests; it is not the production
dispatch path.

**Production path** (`Linker<StoreData>`, built by `setup_host_on_store_data` in
`invoke.rs`): the `read_file` implementation grows WASM linear memory, writes body at
`current_bytes` (the prior memory end, always `> 0` after growth for non-empty files),
and returns the real address via `out_ptr_out`. For non-empty files, `out_ptr_out` is
always `> 0` and `read_owned_bytes` reads data correctly.

**SDK-grounding evidence (literal grep, 2026-07-15):**

```
$ grep -n "proxy_host_imports\|_host_linker_reference" crates/factory-dispatcher/src/invoke.rs | head -4
(proxy_host_imports ignores _host_linker_reference and calls setup_host_on_store_data)

$ grep -n "if ptr == 0 || len == 0" crates/hook-sdk/src/host.rs
(null-pointer guard confirmed in read_owned_bytes)
```

The SEC-001 CRITICAL finding in the S-19.06 PR review correctly identified the test-path
`out_ptr=0` behavior. The accepted-with-record status is appropriate: the test-path write
at address 0 is intentional for unit-test use; the production path always returns a real
non-zero address for non-empty files. This two-path duality is architectural design, not
a defect.

**Documentation fix required (Deliverable D20, partial):** the comment in `read_file.rs`
on the `prepare()` return form is potentially misleading to implementers who might conflate
the two paths. D20 corrects the comment to distinguish the test-path constant-0 return
from the production-path memory-grow protocol.

**HOST_ABI_VERSION:** unchanged. No behavioral change; documentation only.
**Deliverable:** D20 (partial). **Cites:** S-19.06 SEC-001.

### Decision 18 — `timeout_ms` non-enforcement protocol boundary

`read_file` and `read_prefix` accept a `timeout_ms: u32` parameter per their 6-parameter
wire ABI (BC-2.02.002 mandatory-timeout-ms discipline) but drop it:

```rust
let _ = timeout_ms; // accepted for ABI stability; enforced via epoch interruption
```

The comment "enforced via epoch interruption" is technically incorrect and is retracted by
this decision. Epoch interruption (`EpochTicker` in `engine.rs`) fires at WASM yield
points — bytecode-level control-flow transfers within the guest module. A synchronous
`func_wrap` host closure executes on the dispatcher thread in native Rust; no WASM yield
point exists during its execution. Once `std::fs::File::open` or `read_to_end` begins
inside the closure, no epoch tick can interrupt it.

The per-plugin store-level deadline (`store.set_epoch_deadline(timeout_ms_to_epochs(
limits.timeout_ms))` in `invoke_plugin`) provides a coarse plugin-level time budget that
fires when the WASM guest next executes a yield point after the host function returns. It
does not enforce per-host-function `timeout_ms`.

**Decision:** `timeout_ms` in `read_file` and `read_prefix` is ABI-forward-reserved — the
parameter slot exists for a potential future architecture using async host functions
(which could be interrupted at await points). The current synchronous `func_wrap`
implementation drops it. This is a known and accepted protocol limitation.

**SEC-003 severity (CWE-833):** a plugin calling `read_file` or `read_prefix` on a path
that blocks indefinitely (e.g., a FIFO with no writer, an unresponsive NFS mount) holds
the dispatcher thread permanently, preventing subsequent plugin dispatches for that
session. This is classified LOW severity because:

1. `path_allow` contents are operator-configured, not user-controlled. Normal local-SSD
   paths never block.
2. An operator who places FIFOs or NFS mounts in `path_allow` accepts the associated
   risk; this is a configuration choice, not an exploitable condition.
3. No external user input can influence `path_allow` contents at runtime.

The SEC-003 LOW classification from the S-19.06 PR review is confirmed. No escalation
to MEDIUM or CRITICAL is warranted.

**Deliverable D20** corrects the misleading comment in both
`crates/factory-dispatcher/src/host/read_file.rs` and
`crates/factory-dispatcher/src/host/read_prefix.rs`. The corrected form: "accepted for
ABI forward-compatibility; per-host-function timeout is structurally unenforced in the
current synchronous func_wrap dispatch path; the store-level epoch deadline governs
coarse plugin-level time."

**HOST_ABI_VERSION:** unchanged. No behavioral change; documentation correction only.
**Deliverable:** D20. **Cites:** S-19.06 EC-006, SEC-003.

### Decision 19 — INVALID_ARGUMENT (-4) absent from `read_prefix` capability schema table

`codes::INVALID_ARGUMENT = -4` is returned by host functions only when guest-side
marshalling fails: a path argument that is not valid UTF-8, or a guest `out_ptr_out` /
`out_len_out` pointer that resolves outside the WASM memory bounds. Well-formed SDK
calls (`crates/hook-sdk/src/host.rs`) construct paths from Rust `&str` (always valid
UTF-8) and pass stack-allocated `u32` output slots (always in WASM memory bounds).
Plugin authors using the hook-sdk cannot trigger -4 through correct usage.

`-4` is not listed in the `read_file` capability schema in `hooks-registry.toml` or in
`crates/hook-sdk/src/ffi.rs`. The `read_prefix` preamble added in S-19.06 (AC-007) also
omits `-4`. This is consistent treatment across both host read functions.

**SDK-grounding evidence (literal grep, 2026-07-15):**

```
$ grep -n "INVALID_ARGUMENT" crates/factory-dispatcher/src/host/mod.rs | head -3
(INVALID_ARGUMENT = -4 named constant confirmed in codes module; comment confirms -5
is next free after -4, per Decision 13)
```

**Decision:** do NOT add `INVALID_ARGUMENT (-4)` to the `[hooks.capabilities.read_prefix]`
error code table in `hooks-registry.toml`. The preamble documents operator-visible
outcomes — codes that plugin logic should handle. `-4` is a dispatcher-internal
marshalling code unreachable via correct SDK usage. Adding it would mislead operators
into treating it as a recoverable runtime condition requiring explicit handling.
The current preamble table (0, -1, -2, -5, -99) is complete and correct.

No deliverable required. **Cites:** S-19.06 EC-006, `host/mod.rs::codes` module.

### Decision 20 — Phase-B envelope diagnostic policy for `read_prefix`-based guards

Phase-B migrated `verify-factory-lock` and `verify-state-timestamp-refresh` from
`host::read_file` to `host::read_prefix`. The Phase-A approaching-cap soft-warn
(BC-4.13.001 Invariant 10: `bytes_read > 200000` fires `state_md_approaching_cap`) was
defined only for hooks that read STATE.md "in full" via `host::read_file`. With
`read_prefix`, the host function never returns `OUTPUT_TOO_LARGE` — it silently truncates
at `max_bytes=262144`. If STATE.md frontmatter grows beyond 262144 bytes, the closing
`---` falls outside the prefix, `extract_frontmatter` fails, and the guard routes to
fail-open Continue with **zero diagnostic signal** (clean EC-003/MalformedLockBlock path,
indistinguishable from genuine structural malformation). This is the W3G-001 gap.

#### 20.1 Approaching-envelope diagnostic

**Computation:** After a successful `read_prefix` call that returns bytes and
`extract_frontmatter` locates a valid closing `---` delimiter, the implementation MUST
compute:

```
frontmatter_extent = byte offset of the first byte of the closing --- delimiter sequence
```

This value is already available from the `extract_frontmatter` return value
(`delimiter_start_offset` per BC-4.13.001 Invariant 9 verification note).

**Threshold constant:** `APPROACHING_ENVELOPE_THRESHOLD = (STATE_MD_PREFIX_BYTES * 75) / 100`

At `STATE_MD_PREFIX_BYTES = 262144`, this equals **196608 bytes**. This is production-
consistent with Phase-A Invariant 10's 200000-byte threshold (Phase-A: 200000/262144 =
76.3%; Phase-B rounds down to 75% for a compile-time-computable constant).

**Diagnostic event:** When `frontmatter_extent > APPROACHING_ENVELOPE_THRESHOLD`, the
guard MUST emit a diagnostic event of type `state_md_approaching_prefix_envelope` with
three fields:

- `frontmatter_extent_bytes: u64` — the `delimiter_start_offset` value
- `prefix_cap_bytes: u64` — the `STATE_MD_PREFIX_BYTES` constant (262144)
- `utilization_pct: u32` — `((frontmatter_extent * 100) / prefix_cap_bytes) as u32`
  (integer division; truncation acceptable for an observability metric)

This event is observability-only. It NEVER triggers a block, NEVER alters the
`Continue`/`Block` verdict, and requires ZERO new registry entries. It is emitted
via the same `host::log_warn` mechanism used for `MalformedLockBlock` (advisory-only
log channel per BC-4.13.001 PC4).

**Why `state_md_approaching_prefix_envelope` rather than `state_md_approaching_cap`:**
Phase-A's `state_md_approaching_cap` measured total bytes returned by `read_file`
against the read cap (a cap that, if reached, causes `OUTPUT_TOO_LARGE`). Phase-B
measures frontmatter extent against the prefix ceiling after a read that never fails.
The semantics differ: one is a read-cap approach, the other is a parse-boundary approach.
Reusing the same event name would conflate two distinct signals under an ambiguous label;
distinct names allow operators to distinguish the Phase-A and Phase-B diagnostic paths
unambiguously in the dispatcher log.

#### 20.2 Envelope-exceeded vs malformed hard distinction

**Background:** In Phase-B, `extract_frontmatter` receives at most `prefix_cap_bytes`
bytes. When it cannot find a closing `---` delimiter, there are two distinct root causes:

| Scenario | `bytes_returned` | No closing `---` | Root cause |
|----------|---------|---------|------------|
| **Malformed** | < `prefix_cap_bytes` | true | The full file was consumed; no delimiter exists → structurally broken STATE.md |
| **Envelope-exceeded** | == `prefix_cap_bytes` | true | The file may extend beyond the prefix; the delimiter might exist beyond `prefix_cap_bytes` → frontmatter has grown past the established STATE.md byte envelope |

These two cases require a hard-coded distinguishing predicate at the `extract_frontmatter`
call site:

```
let bytes_returned = prefix_returned.len() as u64;
let delimiter_found = extract_frontmatter(&prefix_returned).is_ok();

if !delimiter_found {
    if bytes_returned == prefix_cap_bytes {
        // FrontmatterExceedsEnvelope: file not fully consumed; delimiter may be beyond cap
        emit state_md_frontmatter_exceeds_envelope { prefix_cap_bytes }
        return Continue  // fail-open, same policy as MalformedLockBlock
    } else {
        // MalformedLockBlock: full file consumed; delimiter genuinely absent
        log_warn MalformedLockBlock
        return Continue  // existing behavior unchanged
    }
}
```

**`FrontmatterExceedsEnvelope` — new error class:**

When `bytes_returned == prefix_cap_bytes` AND no closing `---` is found within the prefix:
- **Emit diagnostic event** `state_md_frontmatter_exceeds_envelope` with field
  `prefix_cap_bytes: u64` (262144). This is a one-shot LOUD warn (higher diagnostic
  priority than the approaching-envelope event — it means the guard is already silent).
- **Route to fail-open Continue** (same policy as `MalformedLockBlock` per Decision 7).
- **Do NOT emit `MalformedLockBlock`** for this case — the event name must reflect the
  correct root cause to enable operator diagnosis.

**`MalformedLockBlock` — existing behavior unchanged:**

When `bytes_returned < prefix_cap_bytes` AND no closing `---` is found:
- Emit `log_warn MalformedLockBlock` (existing behavior; no change).
- Return Continue (existing behavior; no change).

**Why `bytes_returned == prefix_cap_bytes` is the correct predicate:**
`read_prefix` returns at most `max_bytes` bytes. If it returns exactly `max_bytes`, the
original file is guaranteed to be at least `max_bytes` long — the prefix was truncated.
If it returns fewer, the full file fit within the prefix — there is no data beyond. A
missing delimiter in a fully-consumed file is structural malformation. A missing delimiter
in a truncated file is an envelope exceedance. The predicate is exact and requires no
additional host call.

#### 20.3 W3G-005 adjudication: `current_bytes as u32` in invoke.rs

The `read_prefix` (and `read_file`) production implementation in
`crates/factory-dispatcher/src/invoke.rs::setup_host_on_store_data` computes the
write-back address for the WASM guest as `current_bytes as u32` (where `current_bytes`
is the accumulated linear-memory usage before the `memory.grow` call). W3G-005 observed
that this cast could silently truncate if `current_bytes` exceeded `u32::MAX`.

**Adjudication: accepted-with-record.** The cast is architecturally correct for WASM32:

1. WASM32 linear memory is bounded at 4 GiB = `u32::MAX` bytes by the WASM specification.
   Any address within WASM32 linear memory fits in a `u32`.
2. The `memory.grow(pages)` call immediately preceding the cast returns `None` if the
   growth would exceed the WASM memory ceiling (4 GiB). The implementation routes `None`
   to `INTERNAL_ERROR (-99)` and returns before reaching the `as u32` cast. The E-20-ARCH-02
   story will add a test exercising this exact branch.
3. Therefore `current_bytes as u32` is reached only after a successful `memory.grow` that
   confirmed the new memory fits within WASM32 address space — the cast cannot overflow.

**Required fix (low-impact, E-20-ARCH-02 in-scope):** Add a `debug_assert!` and an
explanatory comment at the cast site in `setup_host_on_store_data`:

```rust
// WASM32 linear memory ceiling (4 GiB = u32::MAX) is enforced by memory.grow returning
// None on overflow (routed to INTERNAL_ERROR -99 above); any successful grow guarantees
// current_bytes fits in u32.
debug_assert!(current_bytes <= u32::MAX as u64, "memory.grow succeeded; WASM32 ceiling ensures this is in range");
let out_ptr = current_bytes as u32;
```

This is a defensive-assert addition, not a structural code change. **Cites:** W3G-005.

#### 20.4 Implementation scope

This Decision defines the diagnostic policy and error classification. Implementation
(adding the event emission and error class to `verify-factory-lock` and
`verify-state-timestamp-refresh`, and the `debug_assert!` to `invoke.rs`) is scoped to
**E-20-ARCH-02** (host-ABI boundary hardening wave per the E-19 architecture post-epic
report), which already covers the `memory.grow`-failure branch testing for both entry
points. The envelope-diagnostic and `debug_assert!` work folds naturally with that story:
both address untested or undiagnosed boundary conditions in the same host-function layer.

**Deliverable D23** covers the implementation.

**HOST_ABI_VERSION:** unchanged. All changes are in plugin logic and diagnostic emission;
no ABI wire shape changes.

**Cites:** BC-4.13.001 Invariant 10, BC-4.13.001 §Precondition 3 Phase-B,
`factory_lock_parse::extract_frontmatter`, E-20-ARCH-02, W3G-001, W3G-005.

## Concrete Deliverables

The following artifacts are required to implement this ADR. Story decomposition MUST
trace to each entry:

| # | Deliverable | Owner crate / path | Notes |
|---|-------------|-------------------|-------|
| D1 | New Rust crate `verify-factory-lock` | `crates/hook-plugins/verify-factory-lock/` → compiled to `plugins/vsdd-factory/hook-plugins/verify-factory-lock.wasm` | Native WASM plugin; uses `host::read_file` + `host::exec_subprocess`; no dispatcher changes; HOST_ABI_VERSION=1 unchanged |
| D2 | Registry entries for `verify-factory-lock` | `plugins/vsdd-factory/hooks-registry.toml` | Two entries: `PreToolUse` on `Edit\|Write\|MultiEdit\|Agent` and `PreToolUse` on `Bash`; `async = false` (REQUIRED — sync-group for block decisions); `on_error = "continue"`; `timeout_ms = 5000`; MUST include BOTH capability blocks: `[hooks.capabilities.read_file] path_allow = [".factory/STATE.md"]` AND `[hooks.capabilities.exec_subprocess] binary_allow = ["git"] env_allow = ["HOME", "GIT_CONFIG_GLOBAL", "XDG_CONFIG_HOME"]`. Omitting ANY sub-field within a capability block causes the relevant host call to return CapabilityDenied → plugin graceful-degrades to Continue → THE LOCK NEVER ENFORCES. Three confirmed footgun vectors: (1) read_file block absent; (2) exec_subprocess binary_allow absent; (3) exec_subprocess env_allow absent — env_clear() strips HOME, git config user.email returns empty, IdentityResolutionFailed → fail-open. All three must be enumerated explicitly. |
| D3 | STATE.md frontmatter schema extension | `factory_lock` block (`holder`, `locked_at`, `expires_at`) | `state-manager` is sole writer; absent block = unlocked; malformed block = unlocked (fail-open) |
| D4 | `/factory-lock` skill | `plugins/vsdd-factory/skills/factory-lock/SKILL.md` | Acquires lock via fetch-then-CAS push (same primitive as D6); emits `factory.lock.acquired`; mid-burst renewal path in state-manager |
| D5 | `/factory-unlock` skill | `plugins/vsdd-factory/skills/factory-unlock/SKILL.md` | Releases lock (holder only without `--force`; any developer with `--force`); emits `factory.lock.released` or `factory.lock.stolen`; delegates write to state-manager |
| D6 | `state-burst` blind-push fix | `plugins/vsdd-factory/skills/state-burst/SKILL.md` | Change blind `git push origin factory-artifacts` to fetch-then-`git push --force-with-lease=factory-artifacts:<sha>`; same primitive reused by D4 acquire |
| D7 | Lock-status surfacing in `factory-health` | `plugins/vsdd-factory/skills/factory-health/SKILL.md` | Show `Factory lock: FREE` / `HELD by this session (expires <time>)` / `HELD by <holder> since <locked_at> (expires <expires_at>)` |
| D8 | Lock-status surfacing in `factory-worktree-health` | `plugins/vsdd-factory/skills/factory-worktree-health/SKILL.md` | Same three-state display as D7 |
| D9 | Bats integration tests | `plugins/vsdd-factory/tests/` | Cover: lock blocked when held by other developer; read passes when locked; TTL expiry treated as unlocked; acquire CAS rejection on concurrent acquire; mid-burst renewal extends TTL; force-release emits audit event; single-developer unlocked path adds zero friction; capability-omitted registry entry graceful-degrades (advisory only) |
| D10 | `state-burst` SKILL renewal step | `plugins/vsdd-factory/skills/state-burst/SKILL.md` | Add mandatory step before `git -C .factory add -A` / `git commit`: `bash plugins/vsdd-factory/bin/factory-lock-write.sh renew .factory/STATE.md`. Annotate as no-op when unlocked. Also add anti-pattern row: "Skipping renew before git add while lock held → `verify-state-timestamp-refresh` WASM guard blocks the subsequent write (LockExpiryStale)." Reuses existing `factory-lock-write.sh renew` subcommand (S-17.01 deliverable). **RETAINED in v1.6.** |
| ~~D11~~ | ~~`verify-lock-renewal.sh` PreToolUse gate~~ | ~~`plugins/vsdd-factory/hooks/verify-lock-renewal.sh`~~ | **WITHDRAWN in v1.6.** Superseded by D16 (`verify-state-timestamp-refresh` WASM guard — enforces freshness at write-time, not at push-time). Do NOT implement. |
| ~~D12-registry~~ | ~~Registry entry for `verify-lock-renewal.sh`~~ | ~~`plugins/vsdd-factory/hooks-registry.toml`~~ | **WITHDRAWN in v1.6.** No `verify-lock-renewal` entry is added to `hooks-registry.toml`. Do NOT implement. |
| D13 | `state-manager.md` obligation amendment | `plugins/vsdd-factory/agents/state-manager.md` | Amendment to existing §"factory_lock Write/Renewal/Clear Obligation": add cross-reference sentence at the end of §Sequencing invariants Invariant 2 pointing to the `state-burst` SKILL renew step (D10) as the executable enforcement mechanism, and noting that `verify-state-timestamp-refresh` (D16) enforces it at the WASM hook layer. **RETAINED in v1.6.** |
| ~~D14~~ | ~~Bats tests for Decision 11 (`verify-lock-renewal.bats`)~~ | ~~`plugins/vsdd-factory/tests/verify-lock-renewal.bats`~~ | **WITHDRAWN in v1.6.** The bash gate no longer exists. Renewal-check logic is tested at D17 (Rust unit tests + bats for `verify-state-timestamp-refresh`). Do NOT create `verify-lock-renewal.bats`. |
| D15 | Shared `factory-lock-parse` crate | `crates/factory-lock-parse/` | New workspace-internal **library** crate (no `[[bin]]`, no WASM output). MUST NOT live under `crates/hook-plugins/` — that directory is the WASM-plugin floor-count gate's scope; a lib-only crate there breaks CI. Promotes `parse_factory_lock`, `LockState`, `extract_yaml_string_value`, `parse_iso8601` from `verify-factory-lock::lib` to this shared crate. `verify-factory-lock` and `verify-state-timestamp-refresh` both depend on it. `verify-factory-lock/src/lib.rs` changes import paths from `crate::` to `factory_lock_parse::` — logic and tests unchanged. All existing `verify-factory-lock` tests continue to pass unmodified. No `serde_yaml`/`serde_norway` (manual line-by-line scan per Architecture Compliance Rule 4). `chrono` as workspace dep. |
| D16 | `verify-state-timestamp-refresh` WASM plugin + registry entry + priority amendment to `verify-factory-lock` entry | `crates/hook-plugins/verify-state-timestamp-refresh/` → `plugins/vsdd-factory/hook-plugins/verify-state-timestamp-refresh.wasm`; registry entry in `plugins/vsdd-factory/hooks-registry.toml`; also add `priority = 142` to existing `verify-factory-lock` entry | New PreToolUse guard. See Decision 12 for full spec. Crate pattern identical to `verify-factory-lock`: `[lib]` with pure `guard_logic(payload, callbacks)` injectable for unit tests + `[[bin]]` WASI entry point. Uses `factory-lock-parse` for `parse_factory_lock` and `extract_yaml_string_value`. Registry entry: `event = "PreToolUse"`, `tool = "Edit\|Write\|MultiEdit"`, `async = false` (REQUIRED per ADR-019), `on_error = "continue"`, `priority = 143`, `timeout_ms = 5000`. Capability block: `[hooks.capabilities.read_file]` with `path_allow = [".factory/STATE.md"]` ONLY — NO `max_bytes`/`timeout_ms` (ReadFileCaps is `#[serde(deny_unknown_fields)]` with only `path_allow: Vec<String>`; extra fields break registry load). No `exec_subprocess` capability needed. `max_bytes` and `timeout_ms` values are passed as arguments in the WASM plugin source code at `host::read_file` call sites, not in TOML. |
| D17 | Rust `#[test]` unit coverage + bats integration tests for `verify-state-timestamp-refresh` | `crates/hook-plugins/verify-state-timestamp-refresh/src/lib.rs`; `plugins/vsdd-factory/tests/verify-state-timestamp-refresh.bats` | Table-driven unit tests via injectable callbacks (matching `verify-factory-lock` test pattern). MUST cover: (a) Write payload, lock held, `factory_lock.expires_at` unchanged → Block LockExpiryStale; (b) Write payload, lock held, `expires_at` advanced → Continue; (c) Write payload, no lock held, `timestamp:` unchanged → Block TimestampStale; (d) Write payload, no lock held, `timestamp:` advanced → Continue; (e) Write payload, proposed content frontmatter unparseable → Continue (fail-open); (f) on-disk `host::read_file` fails (any HostError) → Continue (fail-open); (g) `file_path` not STATE.md (after normalization) → Continue immediately (no read_file called); (h) `timestamp:` absent in on-disk content → Continue; (i) `timestamp:` absent in proposed content → Block TimestampStale; (j) Edit payload, `old_string` found, reconstructed full content has stale `timestamp:` → Block TimestampStale; (k) Edit payload, `old_string` found, reconstructed full content has advanced `timestamp:` → Continue; (l) Edit payload, `old_string` NOT found in on-disk content → Continue (fail-open); (m) Edit payload with `replace_all=true`, all occurrences replaced, reconstructed content has advanced `timestamp:` → Continue; (n) MultiEdit payload, all edits apply, reconstructed content has stale `timestamp:` → Block TimestampStale; (o) MultiEdit payload, first edit's `old_string` not found → Continue (fail-open); (p) quoted `timestamp:` value normalization — on-disk unquoted, proposed quoted but different value → Continue (no false positive); (q) quoted `timestamp:` value normalization — both sides same quoted value → Block TimestampStale; (r) canonical-path normalization — `file_path = "./.factory/STATE.md"` (leading `./`) → triggers guard after strip (same as unadorned path); (s) absolute-path trigger — `file_path = "/Users/alice/project/.factory/STATE.md"` → triggers guard via `ends_with("/.factory/STATE.md")` (WASM-correct; no env var required); NOTE: `std::env::var("CLAUDE_PROJECT_DIR")` MUST NOT appear in the implementation — always returns Err in WASM sandbox. Bats integration tests MUST cover: Write happy path (advanced timestamp → exit 0), Write stale path (unchanged timestamp → exit 2 with `BLOCKED by verify-state-timestamp-refresh` canonical message), Edit happy path (reconstructed content has advanced timestamp → exit 0), non-STATE.md path (`file_path = ".factory/OTHER.md"` → exit 0 without read_file), AND (e2e-abs) absolute `file_path` = `"/abs/project/.factory/STATE.md"` with stale timestamp through actual wasmtime runtime → exit 2 (validates suffix/equality rule in WASM context, not just native binary). |
| D18 | `host::read_prefix` host function | `crates/factory-dispatcher/src/host/read_prefix.rs` (new dispatcher host fn); `crates/hook-sdk/src/host.rs` (new safe wrapper); `crates/hook-sdk/src/ffi.rs` (new raw wire-ABI extern; wasm32 block + host_stubs) | Additive host function per Decision 15. Signature: `read_prefix(path: &str, max_bytes: u32, timeout_ms: u32) -> i32`. Contract: never returns `OUTPUT_TOO_LARGE`; truncates output to `max_bytes` bytes and returns truncated byte count. Returns `NOT_FOUND (-5)` for absent path (per Decision 13); `CAPABILITY_DENIED (-1)` for path not in `read_prefix.path_allow`; `INTERNAL_ERROR (-99)` for I/O fault. Requires a separate `[hooks.capabilities.read_prefix]` capability block per Decision 15 and BC-1.17.001 Invariant 3; absence returns `CAPABILITY_DENIED (-1)` before filesystem access. HOST_ABI_VERSION = 1 unchanged. Bats unit tests MUST cover: (a) prefix ≤ file_length → byte-exact prefix content returned, exit 0; (b) prefix > file_length → entire file content returned without error, exit 0; (c) absent path → FFI return NOT_FOUND (-5); plugin process exits 0 (Continue); (d) path outside `read_prefix.path_allow` → FFI return CAPABILITY_DENIED (-1); plugin process exits 0 (Continue); (e) `verify-factory-lock` plugin replaces `read_file` call with `read_prefix` (max_bytes=262144 per adjudicated §Decision 15 v1.17 bound) and STATE.md frontmatter is parsed correctly from the 262144-byte prefix even when the full STATE.md file reaches the byte-envelope maximum (fixture body padded to 262144 bytes). |
| D19 | `read_prefix` registration in `setup_host_on_store_data` | `crates/factory-dispatcher/src/invoke.rs` | Per Decision 16. Add `read_prefix` host function binding to `setup_host_on_store_data` following the `read_file` memory-grow protocol in the same function: (1) parse capability block `ctx.capabilities.read_prefix.path_allow`; (2) run capability + path checks (same deny-by-default sequence as `read_file`); (3) call `host::read_prefix::prepare(&ctx, &path, max_bytes)` for the actual bounded read; (4) if body is empty write `ptr=0, len=0` and return `codes::OK`; (5) grow WASM memory by `ceil(body.len() / 65536)` pages, write body at `current_bytes` (always `> 0`), write real address to `out_ptr_out` and length to `out_len_out`. Must also pass through correct `CAPABILITY_DENIED`, `NOT_FOUND`, and `INVALID_ARGUMENT` error-code returns matching the `read_file` production implementation. HOST_ABI_VERSION = 1 unchanged. Cargo test MUST verify: (a) `vsdd::read_prefix` import resolves at plugin instantiation (no wasmtime link error); (b) a round-trip read via `setup_host_on_store_data` returns the correct bytes and a non-zero `out_ptr`; (c) capability absence returns `CAPABILITY_DENIED (-1)`. |
| D20 | `timeout_ms` and `out_ptr=0` doc comment corrections | `crates/factory-dispatcher/src/host/read_file.rs`; `crates/factory-dispatcher/src/host/read_prefix.rs` | Per Decisions 17 and 18. (a) In both `read_file.rs` and `read_prefix.rs`, replace the comment `// accepted for ABI stability; enforced via epoch interruption` (or equivalent) with: `// accepted for ABI forward-compatibility; per-host-function timeout is structurally unenforced in the current synchronous func_wrap dispatch path; the store-level epoch deadline governs coarse plugin-level time`. (b) In `read_file.rs`, add a comment distinguishing the `prepare()` return convention used by the test-path `register()` (always returns `out_ptr=0`) from the production memory-grow protocol in `invoke.rs::setup_host_on_store_data` (writes at `current_bytes > 0`). No behavioral changes; documentation corrections only. |
| D21 | Telemetry named-constant promotion — F-WG-002 | `crates/factory-dispatcher/src/internal_log.rs`; `crates/factory-dispatcher/src/host/read_file.rs`; `crates/factory-dispatcher/src/host/read_prefix.rs`; `crates/factory-dispatcher/src/host/emit_event.rs` | Per F-WG-002. Add `pub const INTERNAL_FILE_NOT_FOUND: &str = "internal.file_not_found";` and `pub const PLUGIN_ABANDONED: &str = "plugin.abandoned";` to `internal_log.rs`. Sweep all call sites in `read_file.rs`, `read_prefix.rs`, and `emit_event.rs` that use the bare string literals and replace with the named constants. Existing test assertions that match on the event type string `"internal.file_not_found"` or `"plugin.abandoned"` MUST continue to pass unmodified (the constant value does not change, only the reference form). |
| D22 | `plugin.completed` async `timestamp` field — F-WG-003 | `crates/factory-dispatcher/src/host/emit_event.rs` | Per F-WG-003. In `emit_plugin_completed_async`, add `.with_field("timestamp", ts.as_str())` before the final `ctx.emit_internal(ev)` call, mirroring all sibling event emitters (`emit_plugin_abandoned`, `emit_plugin_timeout_async`, and others in the file). The `ts` variable pattern matches existing code in the same file — capture `let ts = ...` in the same form. Cargo test MUST verify that the emitted `plugin.completed` event contains a `timestamp` field with a non-empty string value matching the BC-3.08.001 wire format. |
| D23 | Phase-B envelope diagnostic — `FrontmatterExceedsEnvelope` error class + `state_md_approaching_prefix_envelope` + `state_md_frontmatter_exceeds_envelope` events | `crates/hook-plugins/verify-factory-lock/src/lib.rs`; `crates/hook-plugins/verify-state-timestamp-refresh/src/lib.rs`; `crates/factory-lock-parse/src/lib.rs` (if `extract_frontmatter` returns `delimiter_start_offset`); `crates/factory-dispatcher/src/invoke.rs` (`debug_assert!` + comment at `current_bytes as u32` cast site) | Per Decision 20. (a) In `verify-factory-lock` and `verify-state-timestamp-refresh`, after a successful `read_prefix` call: (1) if `extract_frontmatter` succeeds AND `delimiter_start_offset > APPROACHING_ENVELOPE_THRESHOLD` (196608), emit `state_md_approaching_prefix_envelope` with `frontmatter_extent_bytes`, `prefix_cap_bytes`, `utilization_pct` via `log_warn`; (2) if `extract_frontmatter` fails AND `bytes_returned == prefix_cap_bytes` (i.e., the prefix was fully consumed), emit `state_md_frontmatter_exceeds_envelope` with `prefix_cap_bytes` and return `Continue` (`FrontmatterExceedsEnvelope` path); (3) if `extract_frontmatter` fails AND `bytes_returned < prefix_cap_bytes`, emit `log_warn MalformedLockBlock` and return `Continue` (existing path; no change). (b) Add constant `APPROACHING_ENVELOPE_THRESHOLD: u64 = (STATE_MD_PREFIX_BYTES as u64 * 75) / 100` to each plugin or to the shared `factory-lock-parse` crate if `delimiter_start_offset` is available there. (c) In `invoke.rs::setup_host_on_store_data`, at the `current_bytes as u32` cast site, add `debug_assert!(current_bytes <= u32::MAX as u64, ...)` and the explanatory comment per Decision 20.3. Cargo unit tests MUST cover: approaching-envelope case (frontmatter_extent > 196608, well-formed → event emitted, Continue); non-approaching case (frontmatter_extent ≤ 196608, well-formed → no event); envelope-exceeded case (prefix fully consumed, no delimiter → state_md_frontmatter_exceeds_envelope emitted, Continue, NOT MalformedLockBlock); malformed case (prefix not fully consumed, no delimiter → MalformedLockBlock, Continue, NOT state_md_frontmatter_exceeds_envelope). Anchor: E-20-ARCH-02. |

## Rationale

### Why native WASM over a new bash hook sibling

The existing `verify-git-push.sh` is a bash hook routed via `legacy-bash-adapter.wasm`.
A new bash sibling (`verify-factory-lock.sh`) is also viable, but a native WASM plugin
is preferred for the guard because:

1. The guard needs structured frontmatter parsing (YAML subset). Doing this robustly in
   bash requires `awk`/`grep` heuristics that are brittle on edge cases (e.g., multiline
   YAML values, quoted strings with colons). A native Rust implementation is precise and
   testable via `cargo test`.
2. The plugin ecosystem already has the exact pattern: `validate-artifact-path.wasm` is a
   native WASM PreToolUse guard using `host::read_file`. The structure is established and
   the crate scaffolding is known.
3. Binary allow-list for `exec_subprocess` is already proven for `["git"]` in
   `capture-commit-activity.wasm` (registry lines ~65–80). No new host capability is needed.

### Why `async = false` is mandatory (not optional)

The dispatcher partitions plugins into sync-group and async-group at execution time
(ADR-019). Only sync-group plugins participate in the block decision aggregated at
`executor::execute_sync_group`. An `async = true` plugin's `block_intent` is ignored for the
PreToolUse gate — it becomes advisory telemetry only (per ADR-019 CI lint invariant:
`on_error=block ⇒ async=false`). For `verify-factory-lock`, blocking is the entire
purpose; `async = true` would silently reduce it to a no-op blocker. `async = false` is
therefore a correctness requirement, not a performance preference.

### Why capability blocks must be enumerated completely (deny-by-default)

The dispatcher enforces capability deny-by-default: a `host::read_file` call without a
matching `[hooks.capabilities.read_file]` block returns `CapabilityDenied`, which causes
the Rust plugin to graceful-degrade to `Continue` (no block, no error — invisible). The
same applies to `exec_subprocess`, and the deny-by-default principle applies equally to
every sub-field within a capability block.

Three confirmed silent-no-op footgun vectors for `verify-factory-lock`:

1. **read_file block absent** — `host::read_file` returns `CapabilityDenied`; plugin
   cannot read STATE.md; graceful-degrades to `Continue`; lock never enforces.
2. **exec_subprocess binary_allow absent (or does not list `"git"`)** — `host::exec_subprocess`
   returns `CapabilityDenied`; plugin cannot invoke `git config user.email`; graceful-degrades
   to `Continue`; lock never enforces.
3. **exec_subprocess env_allow absent (or does not include `"HOME"`)** — the dispatcher's
   `exec_subprocess` host function calls `env_clear()` and passes ONLY the env vars listed
   in `caps.env_allow`. Without `HOME` (and optionally `GIT_CONFIG_GLOBAL` /
   `XDG_CONFIG_HOME`) in `env_allow`, `git config user.email` cannot read the developer's
   global gitconfig → returns empty string → plugin hits `IdentityResolutionFailed` →
   fails open (`Continue`) → lock never enforces. This is the same deny-by-default
   silent-no-op class as vectors 1 and 2, surfaced via the env-isolation axis rather than
   the binary-allow axis. Discovered in S-17.02 TDD implementation; codified as v1.3
   [process-gap] amendment.

An implementer who scaffolds the registry entry from a minimal template and omits any of
these three fields ships a lock plugin that is indistinguishable from a working guard until
a concurrent-session incident reveals it. Enumerating all three sub-fields explicitly in D2
closes all three footguns.

### Why `on_error = "continue"` (fail-open) rather than `on_error = "block"`

This is an advisory/efficiency-class lock (Kleppmann §8 distinction). Fail-open is
correct for efficiency-class locks: a guard crash's worst case is a missed block, bounded
by Decision 8's CAS push (which still rejects the concurrent push at the network layer).
`on_error = "block"` would make a guard crash equivalent to a permanent lock — the factory
is wedged until the plugin is repaired or the registry is manually edited. That failure
mode is worse than the one being guarded against. See Decision 7.

### Why explicit `/factory-lock` acquire rather than auto-on-first-write

Auto-acquire creates an invisible state transition: the developer doesn't know they own
the lock, doesn't know when they acquired it, and has no obvious way to release it. On
crash, they can't tell if they left a lock behind. Explicit acquire makes ownership
visible and intentional, matches the "user that locked it" mental model confirmed by human
review, and makes the release step natural. The TTL auto-expiry (Decision 5 Path A)
handles the crash case without requiring auto-acquire.

### Why `git config user.email` and not a composite session identity

Composite identity (`hostname::pid::CLAUDE_SESSION_ID`) requires `CLAUDE_SESSION_ID` to
be set in the environment, which is Claude-Code-specific. The factory is designed to be
host-agnostic; requiring a Claude-specific env var in the guard logic couples the guard to
Claude Code's session model. `git config user.email` is universally available (the factory
already requires git), human-readable, and sufficient for the intended threat model
(Developer A vs Developer B). The tradeoff (self-vs-self not blocked) is accepted and
documented.

### Why the blind-push fix is a separate deliverable, not the primary mechanism

The blind-push fix is a guard at the push layer: it detects concurrent pushes after the
work is done. The WASM guard is a check before work begins: it blocks mutating operations
before they produce commits. These are complementary layers. The push fix is the safety
net for self-vs-self and guard-crash scenarios; the WASM guard is the proactive block for
cross-developer scenarios. Both are needed and neither subsumes the other.

## Consequences

### Positive

- **Eliminates the primary cross-developer race** (Developer A vs Developer B) by blocking
  all mutating factory operations when another developer holds the lock.
- **Acquire is CAS-protected:** two simultaneous `/factory-lock` attempts produce one
  success and one actionable rejection (non-fast-forward push error), closing the
  primary TOCTOU acquire-race (CWE-367).
- **Zero infrastructure overhead:** the entire mechanism runs locally (STATE.md read,
  git email query, timestamp comparison). No network calls in the guard hot path.
- **Human-readable lock state:** any developer can `cat .factory/STATE.md` to see who
  holds the lock. No opaque remote state.
- **Blind-push fix (Decision 8) delivers immediate standalone value:** concurrent pushes
  are detected rather than silently clobbered, even without the WASM guard.
- **Actionable failure messages (Decision 4):** blocked developer knows exactly who holds
  the lock, when it expires, and how to force-release.
- **Fail-open on guard crash (Decision 7):** a broken guard never wedges the factory; its
  worst case is bounded by Decision 8's CAS push.
- **Single-developer behavior unchanged:** the unlocked happy path adds zero friction
  beyond a one-time `/factory-lock` at session start (Decision 10).

### Negative / Trade-offs

- **Cooperative, not mandatory:** a developer who does not run `/factory-lock` bypasses
  the protection entirely. The lock is advisory in practice. The primary value is surfacing
  and blocking accidental concurrent work by well-intentioned developers — not preventing
  a determined adversary.
- **Residual TOCTOU acquire-race (CWE-367):** the exact-simultaneity window between two
  sessions' fetch and push steps is narrowed to milliseconds by the CAS push but not
  eliminated. See Decision 6 honest statement. Eliminated by the Decision 9 git-ref-CAS
  future path.
- **Long-burst TTL self-eviction residual risk:** bursts significantly longer than 45
  minutes between intermediate commits can self-evict mid-burst even with mid-burst renewal.
  Residual risk attributed to Decision 9 git-ref-CAS fencing path. See Decision 5 failure
  mode subsection.
- **Self-vs-self not protected:** same developer in two sessions shares the same git email
  and will not be blocked by the guard. Mitigated by the blind-push fix (Decision 8) and
  the `factory-health` observability surfacing.
- **45-minute maximum wedge on crash:** a developer whose session crashes without running
  `/factory-unlock` blocks others for up to 45 minutes. Break-glass `/factory-unlock
  --force` is always available. Acceptable for expected team size and session cadence.
- **Guard depends on local factory-artifacts being current:** the guard reads the local
  STATE.md. If a developer has not fetched `factory-artifacts` recently, the guard's view
  of the lock is stale. The burst fetch (Decision 8) and the fetch in `/factory-lock`
  acquire (Decision 6) mitigate this for the write path.
- **Capability footgun at implementation time:** three confirmed silent-no-op vectors —
  (1) read_file block absent, (2) exec_subprocess binary_allow absent, (3) exec_subprocess
  env_allow absent (env_clear() strips HOME; git config user.email returns empty;
  IdentityResolutionFailed → fail-open). All three explicitly documented in D2 and
  Rationale (v1.3). The bats test in D9 MUST cover all three omission cases.

### Status as of v1.6 (amended, 2026-06-12; Gemini cross-family pass incorporated)

Human direction confirmed + adversary passes 1 and 2 incorporated: move enforcement into
the Rust hook system, replacing the PreToolUse bash gate with a WASM guard that inspects
the proposed full write content before it lands on disk. Decision 12 added
(`verify-state-timestamp-refresh` WASM guard with per-tool reconstruct semantics).
Decision 11 Mechanism 2 (D11/D12-registry/D14) withdrawn. Decision 11 Mechanism 1 (D10)
retained. Push-time enforcement dropped. S-17.04 redirected to v1.2.

Adversary pass 1 corrections: payload fields (`new_content` removed → `content`/reconstruct);
registry caps (`path_allow`-only); explicit priorities (142/143); block message format
(`block_with_fix` canonical); robust frontmatter extraction with quote normalization.

Gemini cross-family pass (adversary pass 2) corrections: (R2) lock-held + proposed
`expires_at` absent OR empty now Blocks LockExpiryStale — closes enforcement asymmetry;
(R4) `..` segment-stack resolution added to canonical-path algorithm; clarity note on
`timestamp:` vs `last_amended:` scoping added; `verify-factory-lock` MultiEdit sibling-sweep
noted as D16 in-scope fix. §12.9 AC/test delta directive issued.

ARCH-INDEX v2.21→v2.22 pending (state-manager bump in follow-up codification burst).

### Additional positive consequences of Decision 12 (v1.6)

- **All four bypass vectors eliminated structurally by construction.** The trigger is a
  structured `file_path` field, not a command string — there is nothing to tokenize,
  over-match, inject newlines into, or env-substitute.
- **Stronger invariant than push-time enforcement.** Freshness is guaranteed at the moment
  of the write, not retrospectively at push time. Commit and push inherit freshness.
- **No push-time gate required.** `factory-cas-push.sh` remains a plain CAS push. Renewal
  is guaranteed to be in the commit by the time it reaches the push step.
- **`parse_factory_lock` deduplicated.** Promoting to `factory-lock-parse` crate removes
  the risk of the same frontmatter scanner diverging between two WASM plugins.
- **No `exec_subprocess` needed.** The new guard has a simpler capability surface than
  `verify-factory-lock` — no `git config user.email` call, no `env_allow` footgun class.
- **Timestamp discipline enforced across all STATE.md writes**, not only during a held-lock
  burst. Every write that does not advance `timestamp:` is blocked regardless of lock state.

### Additional negative consequences / trade-offs of Decision 12 (v1.6)

- **rc-cadence gate.** The WASM guard reaches the operator cache only at rc.21 (held
  pending this story). During develop, the guard is absent; Mechanism 1 (D10) alone enforces
  the obligation via SKILL discipline. rc.21 is the co-deployment point.
- **`factory-lock-parse` crate extraction is in-scope.** Refactoring `verify-factory-lock`
  to import from a shared crate is required work in S-17.04. It expands scope slightly but
  prevents an immediate duplication debt.
- **S-17.04 story rework.** The in-flight branch requires a v1.2 rework (Re-Scope Directive
  below). Sunk cost: the 16 bats tests for prior ACs are mostly retained as CLI-contract
  tests for the existing lock helpers; the 12 Red Gate tests for the bash gate are replaced
  by Rust unit tests in the new WASM crate.

### Status as of v1.5 (amended, 2026-06-11)

Human design confirmed. Research-agent verification APPROVE-WITH-FIXES incorporated (v1.2).
v1.3 [process-gap] amendment incorporated: exec_subprocess env_allow footgun closed; D2
canonical registry form updated. v1.4 [S-17.04] amendment incorporated: Decision 11 added
(automatic heartbeat renewal enforcement — executable state-burst SKILL step + PreToolUse
push gate); Decision 5 vestigial burst-end-only sentence corrected; Deliverables D10–D14
added; BC-5.40.001 PC4 confirmed unaffected. v1.5 [S-17.04 adversary F-1701-001] amendment
incorporated: Decision 11 Mechanism 2 gate-trigger corrected (primary trigger is
`factory-cas-push` helper, not raw `git push` — the real push runs as subprocess inside the
helper and is invisible to PreToolUse; v1.4 trigger was functionally inert on the production
SKILL path); block message reconciled to legacy-bash-adapter one-liner form (multi-line
text is truncated to first line by the adapter; single-line `block_pre` form is the correct
contract); D12 `binary_allow` extended to `["bash", "git", "jq"]` (gate script execs `jq`
to parse STATE.md frontmatter; omitting `jq` → CapabilityDenied → silent fail-open → gate
inert — fourth instance of the deny-by-default silent-no-op footgun class). No further
human-gated questions remain. All eleven decisions are final. D-540 codification recorded by
state-manager 2026-06-10. Implementation stories may be dispatched; S-17.04 implements
Decision 11.

### Why Decision 11 Mechanism 2 was the correct v1.5 design and why Decision 12 supersedes it (v1.6)

Decision 11 Mechanism 2 was the right engineering response given the constraint that the
enforcement needed to fire at a specific push event and that only a Bash command string
was available to identify that event. The four bypass vectors (inert-match, over-match,
newline-injection, env-injection) are all properties of parsing an untrusted command string.
Given only a command string, the v1.5 design was the best achievable.

Decision 12 supersedes it because the enforcement point changes entirely: instead of
blocking the push after checking whether a renewal was committed, we block the **write**
itself if the renewal is absent. The PreToolUse Edit/Write `file_path` is a structured
field set by the tool infrastructure — not user text, not a command string. All four bypass
vectors vanish structurally because their precondition (a command string to parse) no longer
exists at the new enforcement point.

This is also the more correct invariant: "STATE.md always carries a current timestamp at the
moment it is written" is stronger than "STATE.md carried a current timestamp by push time."
The guard enforces freshness at the write; the commit and the push inherit freshness from the
write. No separate push-time gate is needed.

**Why a new WASM crate, not a dispatcher subcommand:** The hook SDK's three-outcome contract
(Continue / Block / Error) cannot inject content into a write — it can only allow or block.
The `factory-dispatcher lock cas-push` chokepoint idea (prior draft of v1.6) would have
needed to re-read the write content after the fact (PostToolUse, which cannot block) or
intercept it before (but the write content isn't available at PostToolUse without re-reading
the file). A PreToolUse WASM guard that reconstructs proposed full content from the tool
payload fields (`tool_input.content` for Write; `tool_input.old_string`+`new_string` for
Edit; `tool_input.edits[]` for MultiEdit) and compares against on-disk is the only mechanism
in the hook SDK that can inspect content before it lands on disk.

**Why redirect S-17.04 now, not land-then-supersede:** Landing D11/D12/D14 then deleting
them in a follow-up story violates CLAUDE.md Rule 2 ("ship each cycle production-grade").
The human approved the redirect. The total sunk cost in v1.1 is modest; the permanent
benefit (no four-vector-vulnerable mechanism ever in the codebase) is disproportionate.

## Alternatives Considered

- **Git ref `refs/factory-lock/<repo-slug>` CAS as primary enforcement (v1.0 design):**
  Demoted to Future/Out of Scope (Decision 9). Requires empirical GitHub.com CAS
  verification probe and adds server-side state management complexity not warranted by the
  threat model. Preserved as the correct upgrade path if the threat model escalates, and as
  the fencing-token mechanism that eliminates the residual TOCTOU and self-eviction risks.

- **New bash hook sibling `verify-factory-lock.sh` (via legacy-bash-adapter):** Viable
  but rejected in favor of native WASM. Bash YAML parsing is brittle; a native Rust
  crate is precise and unit-testable. The `validate-artifact-path.wasm` pattern is already
  established and should be followed.

- **Extend `verify-git-push.sh` for lock enforcement:** Rejected. `verify-git-push.sh`
  has a narrow declared scope (block raw force + protected branches). Conflating lock
  semantics widens its scope and testing surface without benefit.

- **Auto-acquire lock on first write:** Rejected. Creates invisible state transitions,
  complicates crash-recovery reasoning, and conflicts with the "user that locked it"
  mental model confirmed by human review. Explicit acquire is correct.

- **`on_error = "block"` for the guard:** Rejected. An efficiency-class lock (Kleppmann §8)
  that permanently wedges the factory on guard crash is a worse failure mode than the one
  it guards against. `on_error = "continue"` with Decision 8's CAS push as safety net is
  correct for this lock class.

- **Composite session identity (hostname + pid + claude-session-id):** Rejected.
  Requires `CLAUDE_SESSION_ID` env var (Claude-Code-specific); couples the guard to
  Claude's session model; insufficient benefit for the actual threat model. Git user.email
  is universal, human-readable, and sufficient (Decision 3).

- **Burst-end-only TTL renewal (v1.1 design):** Rejected in v1.2. Burst-end-only renewal
  allows a burst longer than the TTL to self-evict. Mid-burst renewal at each intermediate
  `state-manager` commit is the production-grade fix (Decision 5).

- **Per-story granularity lock:** Considered but rejected. The race window is not limited
  to story delivery; any `state-manager` write is a potential concurrent write. Whole-factory
  granularity is simpler and conservative.

## Process Note

**[process-gap]:** Capability enumeration completeness must include `env_allow` for any
guard whose subprocess depends on ambient environment configuration. `git config user.email`
reads from the developer's global gitconfig, which requires `HOME` to locate
`~/.gitconfig`. The dispatcher's `env_clear()` strips all ambient env vars before
subprocess execution; only vars listed in `caps.env_allow` are forwarded. Omitting
`env_allow` from the `exec_subprocess` capability block silently breaks identity resolution
via the same deny-by-default path as omitting the capability block itself. This footgun was
discovered during S-17.02 TDD implementation and codified in v1.3. The routing obligation
per ADR-024 Process Note applies: implementer TDD findings that change behavior the ADR's
canonical registry form specifies MUST route an architect ADR amendment in the same burst.

## Source / Origin

- **Issue:** [#170](https://github.com/drbothen/vsdd-factory/issues/170) —
  `feat(state): single-writer factory lock/lease — prevent concurrent developers racing
  the same repo's factory-artifacts state`
- **Research cache:** `.factory/research/issues/issue-170.md` (VALID-NEW, High confidence;
  2026-06-09) — primary research sources: git-scm `git-push` man page on
  `--force-with-lease` CAS semantics; kubernetes.io Lease API; etcd.io lease/lock docs;
  LWN `https://lwn.net/Articles/817905/` (POSIX lock failure modes); Kleppmann §8
  "Leases and Lease-Based Locks" (efficiency-vs-correctness distinction; long-operation
  TTL hazard; fencing token requirement).
- **TOCTOU acquire-race:** CWE-367 (Time-Of-Check Time-Of-Use Race Condition). The
  fetch→check→push window is bounded but not zero; see Decision 6 residual window statement.
- **Blind push confirmed at:** `skills/state-burst/SKILL.md` push call (`git push origin
  factory-artifacts`) — no CAS, no fetch, confirmed by research cache codebase grounding.
- **Push-hook gap confirmed at:** `hooks/verify-git-push.sh` — allows `factory-artifacts`
  pushes and `--force-with-lease` with no exclusivity check, confirmed by research cache.
- **Native WASM guard pattern:** `crates/hook-plugins/validate-artifact-path/` +
  `hook-plugins/validate-artifact-path.wasm` — the closest existing analogue (PreToolUse,
  `host::read_file`, `on_error = "continue"`).
- **`exec_subprocess` binary_allow=["git"] pattern:** `crates/hook-plugins/capture-commit-activity/`
  + registry entry `capture-commit-activity` (hooks-registry.toml lines ~65–80).
- **Host ABI version:** `hook_sdk::HOST_ABI_VERSION` — `HOST_ABI_VERSION: u32 = 1`.
  No change to dispatcher or ABI is required.
- **Block path in dispatcher:** `executor::plugin_requests_block`
  (`plugin_requests_block` function) invoked at `the sync-group executor dispatch` for sync-group
  plugins. `async = false` is required for the guard to participate in this path.
- **Sync/async partition:** ADR-019 (Plugin Async Semantics at Registry Layer) — CI lint
  invariant `on_error=block ⇒ async=false`; async plugins are advisory-only for block
  decisions.
- **Capability deny-by-default:** confirmed against `crates/factory-dispatcher/src/executor.rs`
  and `hooks-registry.toml` registry patterns. Missing capability blocks return
  `CapabilityDenied` → plugin graceful-degrades to `Continue`.
- **Within-session discipline:** `agents/state-manager.md`, TD-VSDD-053 (single-commit
  burst) — provides the within-session model that this ADR extends cross-session.
- **Decision D-540:** codification decision for this ADR in the v1.0-brownfield-backfill
  cycle decision log.
- **ADR cross-references:** ADR-016 (artifact path guard pattern; `on_error = "continue"`
  precedent), ADR-019 (push hook allow-list semantics; sync/async partition), ADR-020
  (Class A latency budget ≤1500ms p95 that governs the new guard's hook budget), ADR-013
  (cycle-keyed adversarial review structure, which this ADR protects from concurrent
  clobbering).
- **Human design review:** 2026-06-10 — all ten decisions confirmed. Primary enforcement
  changed from git-ref CAS to WASM guard; identity simplified to git user.email; acquire
  made explicit; fail-open on crash confirmed.
- **Research-agent verification:** 2026-06-10 — APPROVE-WITH-FIXES. Five fixes incorporated
  in v1.2: (1) acquire-race CAS fix + CWE-367 honest statement; (2) long-burst TTL failure
  mode + mid-burst renewal + fencing residual risk; (3) capability block enumeration in D2;
  (4) async=false sync-group requirement in D2; (5) Kleppmann efficiency-vs-correctness
  framing in Decision 7.
- **v1.3 [process-gap] amendment:** 2026-06-11 — S-17.02 TDD implementation finding.
  `exec_subprocess` capability block spec was missing `env_allow`. The dispatcher's
  `exec_subprocess` host function calls `env_clear()` before spawning the subprocess and
  passes only vars listed in `caps.env_allow`; without `HOME`, `git config user.email`
  returns empty → `IdentityResolutionFailed` → fail-open → silent no-op guard. Third
  instance of the deny-by-default silent-no-op footgun class. Fix: D2 canonical registry
  form updated to `env_allow = ["HOME", "GIT_CONFIG_GLOBAL", "XDG_CONFIG_HOME"]` on the
  `exec_subprocess` block; Rationale section updated to enumerate all three footgun vectors;
  Process note and Consequences bullet updated. ARCH-INDEX v2.19→v2.20. Issue #170, S-17.02.
- **v1.4 [S-17.04] amendment:** 2026-06-11 — enforcement wiring for BC-5.40.001 PC4. Gap:
  `state-burst` SKILL had no call to `factory-lock-write.sh renew` before `git add`/commit
  despite state-manager.md §obligation table requiring it. Decision 11 added: (1) mandatory
  executable `renew` step in `state-burst` SKILL before staging (Mechanism 1 — reuses
  existing `factory-lock-write.sh renew` from S-17.01, no new script); (2) new
  `verify-lock-renewal.sh` PreToolUse bash hook that blocks a held-lock `factory-artifacts`
  push when HEAD `expires_at` equals `origin/factory-artifacts` `expires_at` (RenewalMissed),
  `on_error=continue`, `async=false`, no-op when unlocked or no remote baseline (Mechanism 2).
  Decision 5 vestigial "burst END" sentence corrected to "every commit in a burst, not only
  at burst-close." Deliverables D10–D14 added. BC-5.40.001 PC4 confirmed unaffected.
  ARCH-INDEX v2.20→v2.21 (pending state-manager row update + version bump). S-17.04.
- **v1.5 [S-17.04 adversary F-1701-001] amendment:** 2026-06-11 — gate-trigger fix,
  block-message reconciliation, D12 jq capability sync. (1) Decision 11 Mechanism 2
  trigger: v1.4 specified `git.*push.*factory-artifacts` as the Bash command pattern. This
  is inert on the production push path: post-S-17.01 the state-burst SKILL runs
  `bash plugins/vsdd-factory/bin/factory-cas-push.sh`; the real `git push --force-with-lease`
  is a subprocess inside that helper — PreToolUse never inspects subprocess command strings.
  Corrected trigger: primary pattern is `.tool_input.command` contains `factory-cas-push`;
  secondary pattern `git`+`push`+`factory-artifacts` is retained belt-and-suspenders for
  hand-typed raw pushes. The check-timing analysis is unchanged: at PreToolUse on
  `bash factory-cas-push.sh`, the burst commit already exists locally (HEAD STATE.md carries
  this burst's `expires_at`), so the HEAD-vs-origin comparison is valid. (2) Block message:
  the legacy-bash-adapter truncates output to first line; the multi-line v1.4 message
  was unreachable. Reconciled to one-liner: `BLOCKED by verify-lock-renewal: RenewalMissed —
  factory_lock held but expires_at not refreshed in this burst. Fix: Run:
  factory-lock-write.sh renew .factory/STATE.md Code: RenewalMissed.` (3) D12
  `binary_allow`: extended from `["bash", "git"]` to `["bash", "git", "jq"]`; gate script
  execs `jq` to parse STATE.md JSON envelope; omitting `jq` → CapabilityDenied → silent
  fail-open → gate inert (fourth deny-by-default silent-no-op vector). S-17.04, F-1701-001.
- **v1.6 [S-17.04 redirect — human approved; adversary pass 1 incorporated] amendment:**
  2026-06-12 — WASM hook adoption + adversary pass 1 corrections. Human requirement:
  "make sure the time is updated on the state every time the state is touched — match
  existing patterns — move to a Rust-based hook system." Hook SDK constraint confirmed:
  three outcomes only (Continue/Block/Error); no mutate/rewrite-content outcome; enforcement
  must be Block-on-stale, not inject-timestamp. Decision 12 added: `verify-state-timestamp-refresh`
  new WASM PreToolUse guard crate (`crates/hook-plugins/verify-state-timestamp-refresh/`);
  triggers on Edit|Write|MultiEdit where `tool_input.file_path` resolves to
  `.factory/STATE.md` (canonical-path normalization per §12.7 R6); proposed full content
  reconstructed per tool type: Write→`tool_input.content`; Edit→on-disk+old/new_string
  reconstruct; MultiEdit→sequential `edits[]` apply; blocks TimestampStale /
  LockExpiryStale; fail-open on parse/IO errors per Decision 7 precedent;
  `host::read_file` capability only with `path_allow = [".factory/STATE.md"]` ONLY
  (ReadFileCaps has no `max_bytes`/`timeout_ms` fields — validated against `registry.rs`).
  Explicit priorities added: verify-factory-lock=142, verify-state-timestamp-refresh=143.
  Block messages corrected to real `block_with_fix` format. Robust frontmatter extraction
  with quote normalization specified (§12.4). Shared crate `factory-lock-parse` added
  (D15). D16 = guard crate + registry entry + `verify-factory-lock` priority amendment.
  D17 = Rust unit tests (19 cases) + bats integration tests (4 cases).
  Decision 11 Mechanism 2 (D11/D12-registry/D14) withdrawn. Push-time enforcement dropped.
  `factory-cas-push.sh` unchanged. Decision 11 Mechanism 1 (D10) retained.
  S-17.04 redirected to v1.2. INV-019 cure: (a) Decision 12 added; (b) D11/D12-registry/D14
  withdrawn, D15/D16/D17 added; (c) S-17.04 Re-Scope Directive issued. AC-correction
  directive for product-owner issued (§12.8): AC-005/006 block strings, AC-010 caps,
  EC-006 path rule, new Write/Edit/MultiEdit ACs. ARCH-INDEX v2.21→v2.22 pending
  state-manager codification burst. rc.21 HELD pending S-17.04 + Rust port.
  Gemini cross-family pass (adversary pass 2) incorporated 2026-06-12: (R2) lock-held +
  proposed `expires_at` absent OR empty now Blocks LockExpiryStale — closes enforcement
  asymmetry where `"" != on_disk_nonempty` let absent-expiry slip through; §12.2 and
  §12.3 updated. (R4) `..` segment resolution (segment-stack pop) added to §12.7 R6
  canonical-path algorithm — `foo/../.factory/STATE.md` now triggers the guard; above-root
  `..` silently discarded (fail-open for unresolvable). Optional clarity note added §12.2:
  `timestamp:` is sole gated field; `last_amended:` is POLICY-14 discipline only.
  Lock-identity guard `verify-factory-lock` `tool` matcher sibling-sweep noted in §12.9
  (MultiEdit omission: D16 in-scope fix). §12.9 AC/test delta directive issued.
  INV-019 cure: (a) §12.2+§12.3 updated; (b) §12.7 R6 extended; (c) §12.9 issued.
- **v1.6 [adversary pass 5 — factory-lock-parse relocation] amendment:** 2026-06-12 —
  `factory-lock-parse` crate path corrected from `crates/hook-plugins/factory-lock-parse/`
  to `crates/factory-lock-parse/`. Root cause: `factory-lock-parse` is a pure library
  crate (no `[[bin]]`, no WASM output); placing it under `crates/hook-plugins/` inflated
  the WASM-plugin floor-count gate's expected count and broke CI. Path updated in §12.5
  prose and D15 table row. No behavior change; no BC impact. INV-019 cure: (a) §12.5 and
  D15 path corrected; (b) path rationale note added to §12.5; (c) no other deliverables
  or ACs affected. ARCH-INDEX v2.25→v2.26 pending state-manager codification burst.
- **v1.6 [adversary pass 7 — P0 WASM env-var dead-code fix] amendment:** 2026-06-12 —
  §12.7 R6 step 1 (`$CLAUDE_PROJECT_DIR` prefix strip via `std::env::var`) was dead code
  in the WASI sandbox. Root cause: `WasiCtxBuilder` in `crates/factory-dispatcher/src/invoke.rs`
  uses `preopened_dir` only, never `.env()`/`.inherit_env()`; `std::env::var` always
  returns `Err` at runtime. Claude Code tools emit absolute `file_path` values (verified,
  5,235+ events); absolute paths never matched the canonical relative string; guard always
  returned `Continue` → guard was inert in production. Survived 6 adversary passes because
  native `#[test]` binary inherits host env vars where `std::env::set_var` works. Fix:
  step 1 removed; match rule changed to: after normalization (steps 1–4: strip `./`,
  collapse `//`, collapse `/./`, `..` segment-stack), trigger if normalized path EQUALS
  `".factory/STATE.md"` OR ENDS WITH `"/.factory/STATE.md"`. No env dependency; no new
  capability. `host::env`+`env_allow` route explicitly rejected (reintroduces
  deny-by-default silent-no-op footgun, ADR-025 v1.3 class). §12.1 trigger description
  updated; §12.9 absolute-path bats e2e test mandate added (MANDATORY — native-env unit
  tests do not validate the WASM trigger). INV-019 cure: (a) §12.1 trigger and §12.7 R6
  rewritten; (b) §12.9 updated with absolute-path e2e mandate and implementer directive;
  (c) AC/EC delta directive updated. ARCH-INDEX v2.26→v2.27 pending state-manager
  codification burst. Architect: S-17.04, issue #170.

- **v1.7 (2026-07-06):** E-19 adversarial-pass-1 fix burst. Decision 13: host ABI
  `codes::NOT_FOUND = -5` allocated (F-P1-001 BLOCKER closure — `-4` is `INVALID_ARGUMENT`,
  `-5` is next free; HOST_ABI_VERSION=1 unchanged, additive). Decision 14:
  `verify-factory-lock` + `verify-state-timestamp-refresh` `STATE_MD_MAX_BYTES` bumped
  65536→262144; `factory_lock_parse::parse_factory_lock` / `extract_yaml_string_value`
  abort after second `---` frontmatter fence (mirrors BC-4.13.001 v1.4 PC3+INV9; closes
  rc.22 smoke FINDING-1). TD-031: 10 `executor.rs`/`lib.rs` volatile line-cites replaced
  with function-name anchors per TD-VSDD-091. F-P1-005 routing reclaim: ADR authored by
  architect. Architect: E-19, issue #170.

- **v1.10 (2026-07-07):** E-19 adv-P11 D18(e) amendment (F-P11-005). D18 test bullet (e)
  corrected: `verify-factory-lock` plugin replaces `read_file` call with `read_prefix`
  (max_bytes=8192 per BC-4.13.001 Phase-B) and STATE.md frontmatter is parsed correctly
  from the 8192-byte prefix even when the full file approaches the 262144-byte Phase-A
  cap (fixture body padded past 8192). Closes F-P11-005. Architect: E-19, issue #170.

- **v1.11 (2026-07-09):** E-19 adv-P32 F-P32-001 closure (architect). §Decision 15
  body corrected at two sites: (a) Primary consumers paragraph — removed stale claim that
  post-migration `STATE_MD_MAX_BYTES` becomes "a soft prefix-read bound rather than a hard
  file-size cap"; BC-4.13.001 Phase-B removes the constant entirely at S-19.07; replaced
  with accurate statement that the `read_prefix` max_bytes call-site argument is 8192 per
  BC-4.13.001 §Precondition 3 Phase-B and is the sole read bound post-migration. (b)
  Truncation-example sentence — reframed from Phase-A 262144 cap (the `host::read_file`
  cap from Decision 14, not a `read_prefix` argument) to the Phase-B 8192 bound sufficient
  for any realistic STATE.md frontmatter block; 262144 explicitly labelled Phase-A-historical.
  Closes F-P32-001 MEDIUM. Architect: E-19, issue #170.

- **v1.13 (2026-07-09):** F-P40-001 MEDIUM (E-19 adv-P40). §Decision 14 Normative-twin
  stable-anchor fix — stale version-pin `BC-4.13.001 v1.4 Precondition 3 and Invariant 9`
  replaced with stable anchor form `BC-4.13.001 §Precondition 3 (Phase-A) and §Invariant 9`
  (POLICY 5 v1.3.5; no version token; semantically correct: Phase-A covers max_bytes=262144
  read-cap which is the subject of Decision 14; matches §Decision 15 stable-cite pattern).
  Architect: E-19, issue #170.

- **v1.14 (2026-07-10):** F-P49-001 MEDIUM (E-19 adv-P49). §Decision 1 body prose and
  Deliverable D2 Notes cell tool-matcher descriptions swept Edit|Write|Agent →
  Edit|Write|MultiEdit|Agent per live hooks-registry.toml ground truth (S-17.04/§Decision 12
  sibling-sweep completion; POLICY 5 v1.3.3). Architect: E-19, issue #170.

- **v1.15 (2026-07-10):** F-P50-001 MEDIUM (E-19 adv-P50). §12.6 Capability block volatile
  line-cite `at line 1181–1182 of hooks-registry.toml` replaced with stable anchor form
  `[hooks.capabilities.read_file]` block (following the `verify-factory-lock` `[[hooks]]`
  stanza) per TD-VSDD-091 anti-volatile-line-pin / POLICY 19. Whole-ADR pointer sweep via
  `grep -nE 'line [0-9]+([–-][0-9]+)? of|at line [0-9]+'`: 1 normative-live pointer found
  (§12.6; fixed this amendment); 0 historical-exempt sites (amendment_reason/[Prior:]/Status/
  Changelog rows not in scope). TOML snippet byte-match with live hooks-registry.toml lines
  1260–1261 confirmed: `[hooks.capabilities.read_file]` / `path_allow = [".factory/STATE.md"]`
  is identical. ADR-030 pointer sweep: 0 hits (no matching pattern). Architect: E-19, issue #170.

- **v1.16 (2026-07-15):** post-E-19 host ABI adjudication (human-authorized 2026-07-15).
  Decision 16: `read_prefix` absent from `setup_host_on_store_data` (production dispatch
  path, `invoke.rs`); confirmed via `grep -n "read_prefix" crates/factory-dispatcher/src/invoke.rs`
  returning 0 hits; D19 added requiring implementer to register `read_prefix` in
  `setup_host_on_store_data` following the `read_file` memory-grow protocol.
  Decision 17: two-linker `out_ptr=0` protocol boundary documented — test path
  (`Linker<HostContext>`, `host/read_file.rs::register`) writes at WASM addr 0 and
  returns `ptr=0`; production path (`Linker<StoreData>`, `invoke.rs::setup_host_on_store_data`)
  grows memory and writes at `current_bytes > 0`; SEC-001 CRITICAL accepted-with-record
  status confirmed appropriate; D20 (partial) corrects misleading comment in `read_file.rs`.
  Decision 18: `timeout_ms` non-enforcement framing corrected — epoch interruption fires
  at WASM yield points only and cannot preempt blocking `func_wrap` host calls;
  "enforced via epoch interruption" comment in `read_file.rs` and `read_prefix.rs`
  retracted; `timeout_ms` is ABI-forward-reserved; SEC-003 CWE-833 LOW severity confirmed;
  D20 corrects both comments.
  Decision 19: INVALID_ARGUMENT (-4) not added to `[hooks.capabilities.read_prefix]`
  schema preamble in `hooks-registry.toml`; current table (0,-1,-2,-5,-99) confirmed
  complete and correct.
  F-WG-002: D21 added — `INTERNAL_FILE_NOT_FOUND` and `PLUGIN_ABANDONED` named constants
  added to `internal_log.rs`; bare literal sweep in `read_file.rs`, `read_prefix.rs`,
  `emit_event.rs`.
  F-WG-003: D22 added — `plugin.completed` async path gains `timestamp` field in
  `emit_plugin_completed_async` matching all sibling event emitters.
  Deliverables D19–D22 added. No HOST_ABI_VERSION change. Architect: post-E-19, issue #170.

- **v1.17 (2026-07-16):** S-19.07 cascade F-P1-001 BLOCKER — read_prefix bound adjudication
  (architect). §Decision 15 `max_bytes` corrected 8192→262144. The v1.11 derivation of
  max_bytes=8192 from "ADR-026 compaction keeps frontmatter <2 KiB" is premise-false:
  ADR-026 §Decision 7 is a line-count discipline (≤200/≤500 lines), not a byte bound.
  Measured STATE.md (2026-07-16): 178,742 bytes total; closing `---` at byte 35,175;
  `last_amended` field alone 32,648 bytes. At max_bytes=8192, `extract_frontmatter` sees no
  closing `---` delimiter → full-input fallback → `MalformedLockBlock` → fail-open
  Continue → guard silently inert; the lock block is ~27 KB beyond the 8192-byte window.
  Adjudicated bound = 262144 — the established STATE.md byte envelope (BC-4.13.001
  §Precondition 3 Phase-A; BC-5.40.001 §Precondition 6); structural guarantee: any
  on-envelope STATE.md has its closing `---` within the 262144-byte prefix. 65536 rejected:
  covers 2026-07-16 measurement but provides no structural guarantee against `last_amended`
  append growth. Root disease (inlined `last_amended` changelog causing frontmatter
  byte-bloat) anchored to S-15.03 PRIORITY-A structured-changelog migration; ADR-026
  line-discipline does not constrain bytes and is not a valid derivation basis for a
  byte-prefix bound. D18(e): fixture description corrected to max_bytes=262144, body padded
  to 262144 bytes. VP-095 v1.2→v1.3 issued same-burst. BC-4.13.001 Phase-B §Precondition 3
  follows as product-owner amendment (follow-up leg). Closes F-P1-001 BLOCKER
  (S-19.07 cascade). Architect: S-19.07 cascade, issue #170.
