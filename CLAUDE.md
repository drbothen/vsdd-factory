# CLAUDE.md — vsdd-factory project conventions

This file is auto-loaded by Claude Code at session start. It pins
project-specific conventions that any agent or human operator should
honor.

## Project identity

- **Self-referential:** vsdd-factory IS the project being onboarded.
  Engine and product are the same repository. The "don't use
  dark-factory paths as cwd" rule does NOT apply here. `.factory/`
  writes target this repo intentionally.

## Branching model

- `develop` — active integration branch. Every feature/fix PR targets here.
- `main` — released versions only. Receives one merge per release (the
  `release/v1.0.0-rc.X` branch) plus one bot commit per release (the
  binary bundle).
- `factory-artifacts` — orphan branch mounted as a worktree at `.factory/`.
  Holds spec docs, cycle logs, STATE.md.

## Releases

**Read [`RELEASING.md`](./RELEASING.md) before cutting any release.** It is
the canonical procedure. The release skill (`/vsdd-factory:release`)
defers to it.

Critical invariants:
- Release branches MUST be named `release/v<full-semver>` and MUST target
  `main` (enforced by `.github/workflows/release-branch-guardrail.yml`).
- Release PRs MUST be merged with `--merge` (not `--squash`) to preserve
  develop's commits as ancestors of main.
- Tag the release at main's new tip after the PR merges.

## Commit messages

- No AI attribution. Never include `Co-Authored-By: Claude` or similar
  references in commits.
- Conventional Commits format preferred (`feat:`, `fix:`, `chore:`, etc.)
  but not enforced by hook.

## Testing

- Bats suite: `cd plugins/vsdd-factory/tests && ./run-all.sh`
- Cargo workspace: `cargo test --workspace --all-targets`
- Cargo fmt + clippy: `cargo fmt --check --all && cargo clippy --workspace --all-targets -- -D warnings`
- Ci.yml runs the same on every PR to develop or main.

## Hooks (this project's own dispatcher)

This project ships the dispatcher binary it consumes. After every
release, the local plugin cache at
`/Users/$USER/.claude/plugins/cache/claude-mp/vsdd-factory/<version>/`
contains the marketplace-tarball version of the dispatcher and registry.
Edits to source must be released for hooks to pick them up at the
operator level — develop changes don't affect the cached plugin.

## See also

- [`RELEASING.md`](./RELEASING.md) — canonical release procedure
- [`CHANGELOG.md`](./CHANGELOG.md) — release history
- [`.factory/STATE.md`](./.factory/STATE.md) — pipeline state (live)
- [`.github/workflows/release.yml`](./.github/workflows/release.yml) — release automation
- [`.github/workflows/release-branch-guardrail.yml`](./.github/workflows/release-branch-guardrail.yml) — TD #69 enforcement
