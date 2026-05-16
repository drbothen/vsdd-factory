# hooks/dim2-gates/ — Dim-2 Gate Template Registry

**Purpose:** Canonical bash template storage for Dim-2 mechanical gate scripts, as prescribed by D-453(e). Each file in this directory is a reusable shell template invoked during fix-burst Dim-2 attestation.

**Codification authority:** D-453(e) (pass-73 fix burst, Commit B). Instantiation of this directory closes ADV-EDP1-P74-HIGH-002 (codification-references-storage-that-doesn't-exist pattern; remediated at pass-74 Commit A). S-15.08 promotes all entries from PLANNED to ACTIVE and adds 6 additional scripts from D-450/451/452 codifications.

**Source vs deployment:** This directory is the SOURCE location for dim2-gate templates within the vsdd-factory plugin source tree. The deployment target per D-453(e) is `.factory/hooks/dim2-gates/` within each factory project worktree. Templates ship to the deployment target via plugin release.

## How to Use

Each script is a standalone author-time tool. During Dim-2 fix-burst attestation, invoke the
relevant script by name, capture its stdout, and paste the literal output (command + exit code
+ stdout) into the burst-log Dim-2 block per D-449(a). This replaces hand-rolled grep commands
and prevents scope-narrowing across attestation cycles (D-453(e) discipline).

Deployment: these scripts ship inside the plugin source tree and are deployed to
`.factory/hooks/dim2-gates/` in each factory project worktree via plugin release.

Scripts are NOT registered in `hooks-registry.toml`. They are not dispatcher plugins.

## Active Template Registry

All entries are `ACTIVE` as of S-15.08 delivery.

| File | Gate | Closes | Example Invocation | Exit Codes |
|------|------|--------|--------------------|-----------|
| `trajectory-tail-cell-grep.sh` | Per-cell line-anchor grep verifying trajectory_tail at each canonical prescribed site | D-454(a) | `trajectory-tail-cell-grep.sh <factory-root> "→9→9→9→9" sites.txt` | 0=PASS, 1=FAIL |
| `freshness-literal-stdout.sh` | Freshness re-execution with literal stdout capture — command + exit code + raw output | D-454(b) | `freshness-literal-stdout.sh "grep -c trajectory_tail .factory/STATE.md"` | 0=PASS, 1=FAIL |
| `block-label-canonical-form.sh` | Tri-way canonical form alignment: verifies all 9 D-444(c) burst-log block labels present | D-454(d) | `block-label-canonical-form.sh burst-log.md` | 0=PASS, 1=FAIL |
| `banner-wc-l.sh` | STATE.md banner "actual N lines" vs wc -l arithmetic + dual-margin (500 - N = margin) | D-450(d) (wc-l) | `banner-wc-l.sh .factory/STATE.md` | 0=PASS, 1=FAIL |
| `propagation-completeness.sh` | Post-derivation propagation-completeness — derived value at ALL prescribed sites | D-452(a) | `cd <factory-root> && propagation-completeness.sh "D-453" sites.txt` | 0=PASS, 1=FAIL |
| `dim7-dispatched-count-sweep.sh` | Dim-7 sibling-sweep across ALL prior burst-log entries for anachronism pattern | D-450(b) | `dim7-dispatched-count-sweep.sh burst-log.md` | 0=PASS, 1=FAIL |
| `dim1-file-count-arithmetic.sh` | Dim-1 headline cardinality vs comma-delimited list count arithmetic gate | D-450(c) | `dim1-file-count-arithmetic.sh burst-log.md` | 0=PASS, 1=FAIL |
| `active-branches-sha-currency.sh` | git rev-parse origin/<branch> currency for all Active Branches table rows | D-450(d) (SHA) | `active-branches-sha-currency.sh . .factory/STATE.md` | 0=PASS, 1=FAIL |
| `decision-log-monotonic-rows.sh` | Decision-log D-NNN ascending-order enforcement (STATE.md + decision-log.md SoT) | D-450(e) | `decision-log-monotonic-rows.sh decision-log.md` | 0=PASS, 1=FAIL |
| `layer-ordinal-dual-direction.sh` | Layer-N dual-direction sweep: positive Nth-layer + negative (N±1)th drift detection | D-452(b) | `layer-ordinal-dual-direction.sh 42 lessons.md` | 0=PASS, 1=FAIL |
| `meta-level-ack-grep.sh` | META-LEVEL-N acknowledgment literal-shell grep with captured cardinality across 4 documents | D-451(a) | `meta-level-ack-grep.sh 24 burst-log.md lessons.md decision-log.md STATE.md` | 0=PASS, 1=FAIL |

## Script Details

### trajectory-tail-cell-grep.sh
- **Args:** `<factory-root> <tail-value> <site-list-file>`
- **Gate:** Per-cell grep (not file-level count) verifying trajectory_tail at each prescribed site
- **Sites file format:** `<relative-file-path>:<anchor-pattern>` (one per line; blank lines skipped)
- **EC-004:** blank lines in sites-file are skipped
- **D-NNN closed:** D-454(a)

### freshness-literal-stdout.sh
- **Args:** `<command-to-rerun>` (quoted single argument)
- **Gate:** Re-executes the command, prints command + exit code + raw stdout verbatim
- **EC-010:** Multi-line stdout is captured and printed in full (no truncation)
- **D-NNN closed:** D-454(b)

### block-label-canonical-form.sh
- **Args:** `<burst-log-path>`
- **Gate:** Verifies all 9 D-444(c) canonical block labels present in the burst-log file
- **Labels checked:** Parent-commit, Adversary verdict, Files touched, Codifications, Dim-2, Dim-5, Dim-6, Dim-7, Closes
- **D-NNN closed:** D-454(d)

### banner-wc-l.sh
- **Args:** `<state-md-path>`
- **Gate:** Extracts "actual N lines" from STATE.md banner, runs wc -l, asserts equality + 500-N margin
- **EC-002:** exits 1 with clear message if no banner pattern found
- **D-NNN closed:** D-450(d) (wc-l sub-clause)

### propagation-completeness.sh
- **Args:** `<derived-value> <prescribed-sites-file>`
- **Gate:** Derived value must appear at ALL prescribed sites (not just the primary site)
- **Sites file format:** `<file-path>:<grep-pattern>` (paths relative to CWD at invocation)
- **EC-006:** exits 1 with specific message if a referenced file does not exist
- **D-NNN closed:** D-452(a)

### dim7-dispatched-count-sweep.sh
- **Args:** `<burst-log-path>`
- **Gate:** Sweeps all burst sections; detects Dim-7 cells referencing future pass numbers (anachronism)
- **EC-009:** exits 0 with message if no Burst sections found
- **D-NNN closed:** D-450(b)

### dim1-file-count-arithmetic.sh
- **Args:** `<burst-log-path>`
- **Gate:** For each `**Files touched (Dim-1): N unique files**` headline, counts the comma-delimited filenames on the following line and asserts N equals the count
- **D-NNN closed:** D-450(c)

### active-branches-sha-currency.sh
- **Args:** `<factory-root> <state-md-path>`
- **Gate:** Runs `git rev-parse origin/<branch>` for each Active Branches table row and compares to STATE.md SHA cell
- **EC-001:** exits 1 with human-readable message if not a git repository
- **Testing:** supports `GIT_TEST_SHA_OVERRIDE_<branch>=<sha>` env vars (branch hyphens → underscores)
- **D-NNN closed:** D-450(d) (SHA sub-clause)

### decision-log-monotonic-rows.sh
- **Args:** `<decision-log-path>`
- **Gate:** Extracts all `| D-NNN` rows via regex `^\| D-[0-9]+[\( ]`; checks strictly non-decreasing integer order
- **EC-003:** exits 0 with "no D-NNN rows found" if file has no matching rows (vacuously monotonic)
- **D-NNN closed:** D-450(e)

### layer-ordinal-dual-direction.sh
- **Args:** `<layer-n> <file1> [file2 ...]`
- **Gate:** Positive sweep for `<N>th-layer` (informational); negative sweep for `<N-1>th-layer` and `<N+1>th-layer` (failure)
- **EC-007:** N=1 skips 0th-layer check; only 2nd-layer is checked as +1 drift
- **D-NNN closed:** D-452(b)

### meta-level-ack-grep.sh
- **Args:** `<meta-level-n> <burst-log> <lessons-md> <decision-log> <state-md>`
- **Gate:** Runs `grep -c "META-LEVEL-<N> CANDIDATE CONFIRMED"` against each of 4 files; exits 0 if total >= 1
- **EC-005:** exits 1 if N < 1 ("META-LEVEL-N must be >= 1")
- **D-NNN closed:** D-451(a)

## Template Creation Cadence

Templates are instantiated in the fix burst that codifies their corresponding D-NNN decision. The D-453(e) pattern prescribes: **codify AND instantiate in the same burst** — no deferred creation. D-454(c) codifies this as a hard rule: `storage-path-without-artifacts` is forbidden; every D-NNN that names a storage path MUST create at least a stub artifact at the same commit.

## Registry Status

| Status | Meaning |
|--------|---------|
| `ACTIVE` | Fully implemented, bats-tested, and in use at Dim-2 attestation |

All 11 scripts are `ACTIVE` as of S-15.08 delivery. The former `PLANNED` status is retired for this initial set.
