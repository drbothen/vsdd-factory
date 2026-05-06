# Adversarial Review — Pass 52 (E-9 v1.46 / D-295 sealed at 4e66b14)

## 1. Angle (NEW per TD-VSDD-057)

**Adversarial test-vector-derivation.** Construct concrete input vectors and ask: (a) what does BC assert, (b) what does source actually do, (c) is case in CTV table, (d) is divergence by-design (post-S-9.07 frame) or spec-internal incoherence?

## 2. Findings

### MED-P52-001 — EC-005A `max_output_bytes` boundary semantics ambiguous (`exceeds` prose vs strict `>` source)

**Severity:** MEDIUM **Confidence:** HIGH
**Location:** BC-1.05.036 §Edge Cases EC-005A

EC-005A prose says "exceeds `max_output_bytes`" — ambiguous (`>` vs `>=`). Source at exec_subprocess.rs:278 uses strict `>`. Postcondition 2 says `stdout_bytes ≤ max_output_bytes` (inclusive). Boundary `len == max_output_bytes` is success path per source. CTV table had no row witnessing this boundary.

### LOW-P52-001 — ADR-015 FileSink rewire CTV gap

CTV table witnesses post-rewire spec-frame state but not pre-rewire INTERIM. Acknowledged in P4 INTERIM caveat; formal CTV witness deferred to E-9 Wave 1 implementation.

### LOW-P52-002 — `timeout_ms = u32::MAX` upper-bound undocumented

EC-013A documents lower bound; symmetric upper bound (~49.7 days) undocumented. v1 known limitation; operator allow-list governs.

## 3. Verdict

**SUBSTANTIVE per strict ADR-013** (1 MED + 2 LOW; standard threshold treats any MED as SUBSTANTIVE despite adversary's lenient NITPICK_ONLY classification). ADR-013 clock RESETS to 0_of_3.

## 4. Process-Gap Tagging

No process-gap findings. All 3 are content-level spec-completeness items.

## 5. TD-VSDD Lesson Awareness

Reviewed 057-092. Pass-52 angle (TV-derivation) NEW.

## 6. Convergence Assessment

Adversary's honest call: "v1.46 BC pair is genuinely convergence-clean under TV-derivation lens." But 1 MED finding triggers strict-protocol clock reset per quality-preference standard.
