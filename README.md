# DeepSeek CLI

An autonomous coding agent for your terminal, powered by DeepSeek. Give it a task, walk away, come back to completed code.

Not a chat wrapper. An autonomous worker that plans, implements, self-heals, reviews, and optimizes — with human checkpoints between phases.

## Why this exists

Existing tools (OpenCode, Aider, Claude Code) follow the same loop: human types, AI responds, human approves. DeepSeek CLI runs a **state machine** that only pauses when it needs you.

```
User prompt
  → PLAN (generates implementation plan)
  → [wait for approval]
  → EXECUTE (tools: read, write, shell, search)
  → SELF-HEAL (auto-fix failures, retry up to N times)
  → SUGGEST (find edge cases, performance wins, missing tests)
  → [wait for approval]
  → OPTIMIZE (apply improvements)
  → DONE
```

## Install

### From crates.io

```bash
cargo install deepseek-cli
```

### Build from source

```bash
git clone https://github.com/Cancelllls/deepseek-cli
cd deepseek-cli
cargo build --release
# Binary at target/release/deepseek-cli
```

### Configure

Set your API key:

```bash
export DEEPSEEK_API_KEY="sk-..."
```

Or create `~/.config/deepseek-cli/config.toml`:

```toml
api_key = "sk-..."
model = "deepseek-chat"
```

## Usage

### Interactive REPL

```bash
deepseek-cli
```

```
  ⟡  DeepSeek CLI
  Autonomous Code Agent

  ⟡  Model: deepseek-chat  |  Type /help for commands, /exit to quit

  ⟡  Add JWT authentication to the user API

  🧠  Activated skills: api, rust, security

  [STATE]  Planning → Generating implementation plan...

  ┌─ PLAN ─────────────────────────────
  │ 1. Read current auth middleware
  │    File: src/auth.rs
  │    Expected: understand existing flow
  │
  │ 2. Add JWT claim validation
  │    File: src/auth/jwt.rs (new)
  │    Create token generation with expiry
  │
  │ 3. Wire into middleware chain
  │    File: src/main.rs
  │    Add JWT middleware before route handlers
  │
  │ 4. Run tests
  │    Command: cargo test
  │    Expected: all pass including new JWT tests
  └────────────────────────────────────

  ?  Proceed with this plan? [Y/n]

  [STATE]  Awaiting Approval → Executing

  ⚙  Reading src/auth.rs ... OK
  ⚙  Writing src/auth/jwt.rs (1024 bytes) ... OK
  ⚙  Running: cargo test ... OK
  ✓  Execution complete.

  [STATE]  Executing → Reviewing

  ┌─ SUGGESTED IMPROVEMENTS ────────────
  │ BUGS:
  │ 1. Token expiry not validated on refresh path
  │ 2. No revocation list check
  │
  │ OPTIMIZATIONS:
  │ 3. Cache JWKS response for 5 minutes
  │ 4. Use ed25519 instead of RSA for 40% smaller tokens
  │
  │ POLISH:
  │ 5. Add `cargo test auth` to CI workflow
  └────────────────────────────────────

  ?  Apply these improvements? [Y/n]

  [STATE]  Awaiting Optimize Approval → Optimizing
  ✓  Optimizations applied.
  🎯  Task complete.

  ⟡  Type your next task or /exit
```

### One-shot mode

```bash
deepseek-cli -p "fix the race condition in connection pool"
deepseek-cli -p "refactor the error handling to use thiserror"
```

### Fully autonomous

```bash
deepseek-cli --yes "build a REST API for user management"
```

All checkpoints are auto-approved. The agent plans, implements, self-heals, and optimizes without asking.

### Override model

```bash
deepseek-cli --model deepseek-reasoner "audit security vulnerabilities in src/"
```

## Skills

Domain-specific instructions are auto-injected into the system prompt based on what you ask for. No configuration needed.

| Skill | Triggers on |
|-------|------------|
| **rust** | cargo, tokio, serde, trait, ownership, async, ... |
| **python** | django, fastapi, pytest, pydantic, numpy, pandas, ... |
| **react** | component, jsx, tsx, hook, tailwind, nextjs, shadcn, ... |
| **api** | rest, graphql, endpoint, jwt, oauth, middleware, cors, ... |
| **database** | sql, postgres, prisma, migration, index, query, schema, ... |
| **testing** | jest, vitest, playwright, coverage, mock, assert, ... |
| **security** | vulnerability, xss, csrf, sql injection, encrypt, hash, ... |
| **devops** | docker, kubernetes, ci/cd, aws, nginx, monitoring, ... |
| **mobile** | ios, android, react native, flutter, swift, kotlin, ... |
| **performance** | optimize, latency, cache, bundle size, lazy load, profile, ... |

Skill files live in `src/skills/` and are compiled into the binary. To add your own: add a `.md` file, register it in `src/skills.rs`.

## Tools

The agent can:

| Tool | What it does |
|------|-------------|
| `read_file` | Reads a file from the project |
| `write_file` | Creates or overwrites a file (creates parent dirs) |
| `run_command` | Runs a shell command, captures stdout + stderr |
| `search_code` | Greps the codebase via `ripgrep` |
| `list_dir` | Lists directory contents |

Self-healing: if `run_command` fails (non-zero exit), the error output is fed back to DeepSeek for a fix attempt. Configurable retries via `--max-retries` (default 3).

## Architecture

```
src/
├── main.rs       — CLI entry, REPL loop
├── api.rs        — DeepSeek API client with SSE streaming
├── config.rs     — API key from env or config file
├── state.rs      — 8-phase state machine
├── planner.rs    — Generates implementation plans
├── executor.rs   — Tool execution loop with self-healing
├── reviewer.rs   — Post-execution review and optimization
├── render.rs     — Terminal rendering (syntect highlighting)
├── skills.rs     — Domain routing engine (keyword matching)
└── skills/       — 10 embedded domain skill files
```

Dependencies: `reqwest` (HTTP), `tokio` (async), `clap` (CLI), `syntect` (highlighting), `colored` (terminal colors).

## Comparison

| | DeepSeek CLI | OpenCode | CodeWhale | Aider |
|---|---|---|---|---|
| **Paradigm** | Autonomous worker | Chat assistant | Chat + Fleet | Chat + edit |
| **State machine** | Plan→Execute→Self-heal→Suggest→Optimize | None | Fleet lanes | None |
| **Human checkpoints** | Yes, between phases | Every turn | Per-tool | Every edit |
| **Self-healing** | Auto-fix + retry | Manual | Manual | Manual |
| **Skill routing** | Keyword auto-detect | Built-in skills | SKILL.md files | None |
| **Language** | Rust | TypeScript | Rust | Python |

## License

MIT
