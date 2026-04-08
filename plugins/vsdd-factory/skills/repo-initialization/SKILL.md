---
name: repo-initialization
description: >
  Interactive repository creation flow. Orchestrator gathers requirements
  from human, devops-engineer creates repo with standard configuration.
---

> **Delegation Reference:** This skill describes work the orchestrator delegates
> to specialist agents via sessions_spawn. Each step names the target agent.
> The orchestrator does NOT execute these steps directly.

# Repository Initialization: Interactive Creation Flow

## When This Skill Runs

When the orchestrator detects that no repository exists for a new project:
- Human provides a product description but no repo path
- Human provides a path that doesn't exist or has no `.git/`
- Human explicitly requests repo creation

## Prerequisites

- `gh` CLI installed and authenticated (`gh auth status`)
- Human has GitHub org/user access for repo creation
- Network access to GitHub API

## Workflow

### Step 1: Orchestrator Gathers Requirements (Interactive)

The orchestrator asks the human:

1. **GitHub organization?**
   - Suggest based on `gh api user/orgs --jq '.[].login'`
   - Default: personal account

2. **Repository name?**
   - Suggest based on product description (e.g., "mdcheck" for "markdown link checker")
   - Validate: lowercase, hyphens, no special chars

3. **Visibility?**
   - Default: private
   - Options: private, public, internal (org only)

4. **Default branch name?**
   - Default: develop
   - VSDD standard is `develop` (not `main`)

### Step 2: Architect Recommends Repo Strategy

After the business-analyst produces the L2 Domain Spec and the architect
begins system design, the architect analyzes the product structure:

#### Signals Favoring MULTI-REPO

| Signal | Weight | Example |
|--------|--------|---------|
| Multiple deployment targets | HIGH | API server + frontend + SDK |
| Different tech stacks | HIGH | Rust backend + Next.js frontend |
| Independent release cycles | MEDIUM | API v2 ships before frontend catches up |
| Team boundaries | MEDIUM | Backend team vs frontend team |
| Shared contract layer | MEDIUM | Types/schemas consumed by multiple services |
| Service-oriented architecture | HIGH | Microservices, API gateway pattern |

#### Signals Favoring SINGLE-REPO

| Signal | Weight | Example |
|--------|--------|---------|
| Single deployment target | HIGH | CLI tool, single binary |
| Single tech stack | MEDIUM | Pure Rust, pure TypeScript |
| Tight coupling between components | HIGH | Shared memory, function calls |
| Single team | LOW | One developer or small team |
| Simple product | HIGH | Library, CLI utility |

Human confirms or overrides. If multi-repo, devops-engineer creates all repos.

### Step 3: DevOps-Engineer Creates Repo

The devops-engineer executes the repo creation protocol:

**WORKSPACE ISOLATION GUARD (BLOCKING):** Before ANY repo or git commands,
verify you are NOT operating inside the dark-factory engine directory:

```bash
CWD=$(pwd)
if [[ "$CWD" == *"dark-factory"* ]]; then
  echo "FATAL: Running in dark-factory engine directory ($CWD). Refusing to proceed."
  echo "Fix: orchestrator must set cwd to the resolved project path in sessions_spawn."
  exit 1
fi
```

1. **Create GitHub repo:**
   ```bash
   gh repo create ORG/REPO --private --description "PRODUCT_DESCRIPTION"
   ```

2. **Clone locally (into the target path, never into dark-factory):**
   ```bash
   git clone git@github.com:ORG/REPO.git TARGET_PATH
   cd TARGET_PATH
   ```

3. **Configure git:**
   ```bash
   git config rerere.enabled true
   git config rerere.autoupdate true
   ```

4. **Create initial structure:**
   ```bash
   mkdir -p .factory .worktrees
   ```
   Create `.gitignore`:
   ```
   .factory/
   .factory-project/
   .worktrees/
   .env
   .env.local
   *.bak
   ```
   Create minimal `README.md` placeholder.

5. **Initial commit and push:**
   ```bash
   git add -A
   git commit -m "chore: initialize project"
   git branch -M develop
   git push -u origin develop
   ```

6. **Configure branch protection:**
   ```bash
   gh api repos/ORG/REPO/branches/develop/protection -X PUT -f \
     required_status_checks='{"strict":true,"contexts":[]}' \
     required_pull_request_reviews='{"required_approving_review_count":0}' \
     enforce_admins=false \
     restrictions=null
   ```

7. **Create merge-config.yaml:**
   Copy `templates/merge-config-template.yaml` to `.factory/merge-config.yaml`
   and customize for this project.

8. **Set up .factory/ as git worktree on factory-artifacts orphan branch:**

   **Pre-check:** Verify git remote is the target project (NOT dark-factory):
   ```bash
   REMOTE_URL=$(git remote get-url origin)
   if [[ "$REMOTE_URL" == *"dark-factory"* ]]; then
     echo "FATAL: git remote points to dark-factory. Wrong repo."
     exit 1
   fi
   ```

   ```bash
   git checkout --orphan factory-artifacts
   git rm -rf .
   git commit --allow-empty -m "chore: initialize factory artifacts branch"
   git push origin factory-artifacts
   git checkout develop
   git worktree add .factory factory-artifacts
   ```
   **Verify worktree is valid (BLOCKING — do not proceed if this fails):**
   ```bash
   git -C .factory rev-parse --git-dir  # Must succeed
   git -C .factory branch --show-current  # Must show "factory-artifacts"
   # Verify .git file points to THIS repo, not the engine
   GITDIR=$(cat .factory/.git | sed 's/gitdir: //')
   [[ "$GITDIR" != *"dark-factory"* ]] || { echo "FATAL: .factory/.git points to engine"; exit 1; }
   ```

9. **Report to orchestrator:**
   ```
   "Repository ORG/REPO created at TARGET_PATH.
    Branch: develop (protected). Git rerere enabled.
    merge-config.yaml created with autonomy level 4.
    factory-artifacts orphan branch created. .factory/ mounted as worktree."
   ```

### Step 4: CI/CD Pipeline Setup

After the repo is created and initial structure committed, the orchestrator
gathers CI/CD preferences and the devops-engineer creates the pipeline.

#### Orchestrator CI/CD Questions

The orchestrator asks the human (in addition to repo questions above):

1. **CI provider?**
   - GitHub Actions (default)
   - GitLab CI
   - Other (provide config template)

2. **Target platforms?** (auto-detected from tech stack)
   - Rust: linux-x64, macos-arm64, windows-x64
   - Node.js: linux-x64, macos-arm64
   - Python: linux-x64, macos-arm64

3. **Package registries?** (auto-detected from tech stack)
   - Rust: crates.io
   - Node.js: npm
   - Python: PyPI
   - Docker: GHCR (GitHub Container Registry)
   - None

4. **Deploy target?**
   - None (library/CLI -- default)
   - Kubernetes (Helm chart)
   - Fly.io
   - Vercel
   - Cloudflare Workers
   - AWS Lambda

#### DevOps-Engineer Creates CI/CD Workflows

The devops-engineer creates `.github/workflows/`:

1. **`ci.yml`** -- CI pipeline:
   ```yaml
   name: CI
   on:
     push: { branches: [feature/*, fix/*] }
     pull_request: { branches: [develop] }
   jobs:
     lint:
       # clippy / eslint / ruff (based on tech stack)
     test:
       # cargo test / npm test / pytest
       # Coverage report (uploaded as artifact)
     build:
       # cargo build --release / npm run build
       # Matrix: [platform targets from init]
   ```

2. **`release.yml`** -- Release pipeline:
   ```yaml
   name: Release
   on:
     push: { tags: ['v*'] }
   jobs:
     build:
       # Build release binaries (matrix: platforms)
       # Run tests (final verification)
     publish:
       # Upload binaries to GitHub Release
       # Publish to registries (crates.io / npm / PyPI)
       # Push Docker image (if configured)
   permissions:
     contents: write
   ```

3. **`security.yml`** -- Security audit:
   ```yaml
   name: Security Audit
   on:
     schedule: [{ cron: '0 6 * * 1' }]  # Weekly Monday 6am
     pull_request: { branches: [develop] }
   jobs:
     audit:
       # cargo audit / npm audit
       # Semgrep with auto rules
       # License check (cargo deny / license-checker)
     report:
       # Post findings as PR comment (if PR trigger)
       # Create issue (if schedule trigger + findings)
   ```

#### Branch Protection Update (with CI checks)

After CI workflows are committed, update branch protection to require CI
status checks:

```bash
gh api repos/ORG/REPO/branches/develop/protection -X PUT \
  --input - <<EOF
{
  "required_status_checks": {
    "strict": true,
    "contexts": ["CI / lint", "CI / test", "CI / build"]
  },
  "required_pull_request_reviews": {
    "required_approving_review_count": 0
  },
  "enforce_admins": false,
  "restrictions": null
}
EOF
```

### Step 5: Multi-Repo Initialization (if applicable)

When architect's architecture output includes `deployment_topology: multi-service`
and human confirms multi-repo:

#### 5a: Create .factory-project/ worktree in the primary repo

The primary repo holds project-level coordination artifacts. Set up an orphan
branch + worktree for `.factory-project/`, just like `.factory/`:

**Pre-check:** Verify git remote is the target project (NOT dark-factory):
```bash
REMOTE_URL=$(git remote get-url origin)
[[ "$REMOTE_URL" != *"dark-factory"* ]] || { echo "FATAL: git remote points to dark-factory"; exit 1; }
```

```bash
git checkout --orphan factory-project-artifacts
git rm -rf .
git commit --allow-empty -m "chore: initialize factory-project artifacts branch"
git push origin factory-project-artifacts
git checkout develop
git worktree add .factory-project factory-project-artifacts
```

Verify:
```bash
git -C .factory-project rev-parse --git-dir  # Must succeed
git -C .factory-project branch --show-current  # Must show "factory-project-artifacts"
# Verify .git file points to THIS repo, not the engine
GITDIR=$(cat .factory-project/.git | sed 's/gitdir: //')
[[ "$GITDIR" != *"dark-factory"* ]] || { echo "FATAL: .factory-project/.git points to engine"; exit 1; }
```

Update `.gitignore` to include `.factory-project/`.

#### 5b: Create per-service repos

For each service identified by the architect:
1. Create via `gh repo create ORG/REPO-NAME --private`
2. Clone into `./repos/REPO-NAME`
3. Init structure (develop branch, branch protection, git rerere)
4. Set up `.factory/` worktree on `factory-artifacts` orphan branch (same as Step 3 item 8)
5. Branch protection on each

#### 5c: Generate project.yaml

Create `project.yaml` in the primary repo from the architect's service map:
```yaml
project:
  name: PRODUCT_NAME
  primary_repo: ORG/PRODUCT_NAME
repos:
  - name: product-api
    path: ./repos/product-api
    tech: rust
    role: service
  - name: product-frontend
    path: ./repos/product-frontend
    tech: typescript
    role: service
  - name: product-sdk
    path: ./repos/product-sdk
    tech: typescript
    role: generated
dependency_graph:
  product-api: []
  product-frontend: [product-api]
  product-sdk: [product-api]
```

#### 5d: Initialize .factory-project/ structure

Spawn state-manager to create the project-level directory structure:
```
.factory-project/
  STATE.md              — project-level pipeline state
  cost/                 — cross-repo cost tracking
  integration/          — cross-repo integration gate results
  specs/                — unified L1 brief, L2 domain spec, L3 PRD (project-level)
  wave-plans/           — repo-wave ordering from dependency graph
```

The unified Phase 1 specs from `.factory/specs/` are moved to `.factory-project/specs/`
since they represent the product as a whole, not a single service.

### Step 6: DX Engineer Environment Setup (DF-027)

After repo is created, orchestrator spawns dx-engineer for environment setup:

1. **Install direnv** (if not present):
   ```bash
   brew install direnv   # macOS
   # Or: sudo apt install direnv  # Linux
   ```

2. **Create .envrc** (committed):
   ```bash
   echo 'dotenv .env' > .envrc
   git add .envrc
   ```

3. **Create .env.example** (committed, empty at init):
   ```bash
   cat > .env.example << 'EOF'
   # ============================================================
   # PRODUCT RUNTIME -- keys your product needs to function
   # Populated when DTU assessment identifies external services
   # ============================================================
   # (empty at init -- filled during Phase 1 DTU assessment)

   # ============================================================
   # RELEASE -- added before first release (Phase 6+)
   # ============================================================
   # CARGO_REGISTRY_TOKEN=     # crates.io publish
   # NPM_TOKEN=                # npm publish
   # DOCKER_PASSWORD=           # Docker Hub / GHCR push
   EOF
   git add .env.example
   ```

   **Note:** Factory LLM keys (ANTHROPIC_API_KEY, OPENAI_API_KEY, etc.) are NOT
   in the product repo's .env. They live in the dark-factory repo's .env.

4. **Update .gitignore** to include:
   ```
   .env
   .env.local
   ```

5. **Run direnv allow:**
   ```bash
   direnv allow .
   ```

6. **Install mcporter** (MCP CLI for sub-agent access):
   ```bash
   npm install -g mcporter
   clawhub install mcporter
   ```

7. **Configure MCP servers via mcporter:**
   ```bash
   mcporter config add perplexity --transport stdio \
     --command "npx" --args "-y @anthropic/perplexity-mcp"
   mcporter config add context7 --transport stdio \
     --command "npx" --args "-y @anthropic/context7-mcp"
   mcporter config add tally --transport stdio \
     --command "npx" --args "-y tally-mcp"
   mcporter config add playwright --transport stdio \
     --command "npx" --args "-y @anthropic/playwright-mcp"
   ```

8. **LLM health check:**
   Verify all 3 model families are reachable via LiteLLM proxy.
   Block if any model is unavailable.

9. **MCP preflight:**
    Verify mcporter can reach Perplexity, Context7, Tally, Playwright.
    Spawn test  to confirm sub-agent MCP access works.

## Output

- GitHub repository created with develop branch and branch protection
- Local clone at TARGET_PATH
- `.gitignore` with `.factory/`, `.worktrees/`, `.env`, `.env.local` excluded
- `.factory/merge-config.yaml` from template
- `.envrc` created and direnv allowed
- `.env.example` created (empty at init, populated incrementally)
- `.factory/` set up as worktree on factory-artifacts branch
- mcporter installed and MCP servers configured
- LLM health check passed (all 3 model families reachable)
- Git rerere enabled
- `.github/workflows/ci.yml` -- lint, test, build on PR
- `.github/workflows/release.yml` -- build + publish on tag
- `.github/workflows/security.yml` -- weekly audit + PR check
- Branch protection requiring CI status checks
- Orchestrator notified of completion

### Excalidraw MCP (UI Products)

For UI/full-stack products:
```bash
mcporter config add excalidraw --url https://mcp.excalidraw.com
```
