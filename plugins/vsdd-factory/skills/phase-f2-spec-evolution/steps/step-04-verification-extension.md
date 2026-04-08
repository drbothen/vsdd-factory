# Step 4: Verification Property Extension

Spawn `formal-verifier` agent to define new verification properties for new requirements.

## Inputs

- PRD delta from Step 2
- Architecture delta from Step 3 (if applicable)
- Current verification architecture at `.factory/specs/verification-architecture.md`

## Actions

1. Spawn `formal-verifier` agent
2. Define new verification properties for new requirements (continue VP-ID sequence)
3. Update existing verification properties if requirements changed
4. Determine which new Kani proofs, proptest strategies, or fuzz targets are needed
5. Write to `.factory/phase-f2-spec-evolution/verification-delta.md`
6. Update `.factory/specs/verification-architecture.md`

## Outputs

- `.factory/phase-f2-spec-evolution/verification-delta.md`
- Updated `.factory/specs/verification-architecture.md`

## Completion Criteria

- New verification properties continue the VP-ID sequence
- Each new VP has a corresponding proof strategy (Kani, proptest, fuzz, or manual)
- Updated VPs reference both old and new behavior
