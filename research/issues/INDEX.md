# GitHub Issue Validation Sweep — Triage Index

**Date:** 2026-06-09
**Sweep:** 18 open GitHub issues validated against codebase (develop @ `82163b7f`) + external research
**Scope:** 5 research-agent subagents; each issue has a durable per-issue cache at `.factory/research/issues/issue-<N>.md`

---

## Triage Table

| Issue | Title (short) | Verdict | Confidence | Cluster | Cache |
|-------|---------------|---------|------------|---------|-------|
| [#128](issue-128.md) | pr-manager merge doesn't verify remote-branch deletion | VALID-NEW | High | Bug: PR-lifecycle | [issue-128.md](issue-128.md) |
| [#130](issue-130.md) | dispatcher recursive `.factory/.factory/logs/` shadow | VALID-PARTIAL | High | Bug: dispatcher | [issue-130.md](issue-130.md) |
| [#169](issue-169.md) | per-story sub-agents read stale worktree specs | VALID-NEW | High | Worktree-identity | [issue-169.md](issue-169.md) |
| [#176](issue-176.md) | adversarial-review worktree-identity preflight | VALID-NEW | High | Worktree-identity | [issue-176.md](issue-176.md) |
| [#170](issue-170.md) | single-writer factory lock/lease | VALID-NEW | High | State durability/concurrency | [issue-170.md](issue-170.md) |
| [#173](issue-173.md) | wave-boundary checkpoint+reset + PreCompact flush | VALID-NEW | High | State durability/concurrency | [issue-173.md](issue-173.md) |
| [#171](issue-171.md) | revalidate deferred items w/ research-agent | VALID-NEW | High | State durability/concurrency (backlog-hygiene) | [issue-171.md](issue-171.md) |
| [#162](issue-162.md) | orchestrator firefighting / runtime enforcement | VALID-NEW | High | Runtime enforcement | [issue-162.md](issue-162.md) |
| [#133](issue-133.md) | intra-phase adversarial passes (scheduled) | VALID-PARTIAL | High | Runtime enforcement | [issue-133.md](issue-133.md) |
| [#177](issue-177.md) | hollow-demo / false-confidence checker | VALID-PARTIAL | High | Runtime enforcement (review-quality) | [issue-177.md](issue-177.md) |
| [#151](issue-151.md) | drift-resistant citation convention + drift checker | VALID-PARTIAL | High | Consistency/citation | [issue-151.md](issue-151.md) |
| [#131](issue-131.md) | consistency-validator URL/endpoint/path coherence | VALID-NEW | High | Consistency/citation | [issue-131.md](issue-131.md) |
| [#150](issue-150.md) | per-story uncertainty-removal + self-containment gate | VALID-PARTIAL | High | Pre-Phase-3 quality gate | [issue-150.md](issue-150.md) |
| [#129](issue-129.md) | production-grade default + routing into shipped plugin | VALID-NEW | High | Canonicalization | [issue-129.md](issue-129.md) |
| [#172](issue-172.md) | route demo evidence to factory-artifacts | VALID-NEW | High | Demo-evidence routing | [issue-172.md](issue-172.md) |
| [#174](issue-174.md) | CLAUDE.md health-check + threshold compaction | VALID-NEW | High | Doc governance | [issue-174.md](issue-174.md) |
| [#175](issue-175.md) | version-drift guard — block until re-activation | VALID-PARTIAL | High | Activate | [issue-175.md](issue-175.md) |
| [#149](issue-149.md) | OTEL telemetry to reduce agent handwaving | ALREADY-DONE | High | — | [issue-149.md](issue-149.md) |

**Summary:** 17 actionable (VALID-NEW or VALID-PARTIAL) + 1 already-done (#149 → recommend GitHub close).

---

## #149 — Recommend GitHub Close (already shipped)

Issue #149 requests OTEL telemetry to reduce agent handwaving. vsdd-factory already ships a complete OTEL observability stack via `claude-telemetry`, `factory-obs`, and `onboard-observability` skills. The `claude-telemetry` skill wires Claude Code's native OTel export to a Grafana/Loki stack providing exactly the requested out-of-band verification channel. This feature is live in the operator cache at rc.20. **Recommendation: close #149 as "already done."**

---

## Cross-Issue Coupling Notes

- **#169 + #176** — Same root cause: sub-agents read the wrong git tree when the factory worktree is mounted. Fix together in one burst; resolving #169 without #176 (or vice versa) leaves half the attack surface open. Per-issue detail in respective cache files.

- **#170 + #173 + #171** — State-durability cluster. #170 (single-writer lock) is the foundational primitive; #173 (wave-boundary checkpoint+reset + PreCompact flush) builds on it; #171 (revalidate deferred items) composes with #173's wave-handoff hook. Schedule together; #170 gates #173 which gates #171.

- **#162 + #133 + #177** — "Move load-bearing checks from prose into the harness." #162 is the umbrella enforcement story; #133 adds scheduled intra-phase adversarial passes; #177 hardens demo-evidence (also an item-D of #162). Coordinate: #177 ↔ #162-item-D both harden demo-evidence; implementing #177 standalone without #162 creates partial coverage.

- **#151 + #131** — Consistency/citation pair. #151 is PARTIAL because the stable-anchor convention (TD-VSDD-091 + `validate-stable-anchors` policy) already exists — the missing piece is an active *drift-detection checker*. #131 is fully net-new (URL/endpoint/path coherence not currently validated). Natural to ship together; #151's drift checker is infrastructure that #131 can reuse.

- **#129 ↔ #171** — Both touch deferral metadata fields. #129 brings CLAUDE.md's canonical-principle + routing table into the shipped plugin; #171 revalidates deferred items (depends on the deferral metadata spec that #129 would formalize). Sequence: #129 before #171 for clean dependency.

- **#150** — Extends existing `rules/story-completeness.md`. Only AC-oracle + cross-story-handoff dimensions are net-new; the existing gate skeleton reduces implementation risk.

- **#175** — The convention (version field in plugin manifest) exists; the compare/block hook does not. Implementation is primarily a new WASM hook entry in `hooks-registry.toml` + `dispatcher` plumbing.

---

## Recommended Sequencing (advisory — not a committed wave plan)

Cluster bugs first (ship-ready, narrowly scoped), then identity/durability, then enforcement, then consistency/quality, then activation.

### Cluster 1: Bugs (ship-ready)
- **#128** — pr-manager remote-branch deletion verification (low LOC; single-agent skill patch)
- **#130** — dispatcher `.factory/.factory/logs/` shadow (path-construction fix in dispatcher source)

### Cluster 2: Worktree-identity (fix together)
- **#169 + #176** — spec-reader git-tree identity (fix in one burst)

### Cluster 3: State durability / concurrency
- **#170** → **#173** → **#171** (in dependency order; #170 gates #173 gates #171)

### Cluster 4: Runtime enforcement
- **#162** (umbrella) → coordinate **#133** + **#177** as sub-items

### Cluster 5: Consistency / citation
- **#151 + #131** (ship together; shared checker infrastructure)

### Cluster 6: Pre-Phase-3 quality gate
- **#150** (extends existing gate; low integration risk)

### Cluster 7: Canonicalization
- **#129** (sequence before #171)

### Cluster 8: Demo-evidence routing
- **#172** (coordinate with #162/#177 if scheduling overlaps)

### Cluster 9: Doc governance
- **#174** (CLAUDE.md health-check; low risk, low urgency)

### Cluster 10: Activate
- **#175** (version-drift block hook; self-contained WASM plugin)

---

## Per-Issue Research Notes

Full rationale, codebase grounding (file:line cites), external citations (URLs), recommended approach, test strategy, and dependencies are in each per-issue cache file. Do not duplicate here — reference the cache.

---

*Generated by state-manager D-533 burst 2026-06-09. Research performed by research-agent subagents on develop @ `82163b7f`.*
