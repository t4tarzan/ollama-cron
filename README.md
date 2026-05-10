# ollama-cron

> Cron, but for AI. Schedule prompts to run anywhere — local Ollama, Claude, GPT — with output to file, webhook, or email.

**Status:** v0.1 — ready to use.

**Sovereignty:** sovereign-by-construction. Works with local Ollama by default. BYO endpoint, BYO key.

---

## What this is

Self-hosters and power users want scheduled AI tasks: a daily news brief from their RSS, a weekly summary of a watched repo, an hourly check on a feed. n8n is too heavy. Cron is too primitive (no LLM call). ollama-cron sits between: cron syntax, OpenAI-compatible call, output to file/webhook.

## What this isn't

- Not an agent platform
- Not a workflow builder
- Not n8n
- Scheduled prompts only

See [PRD-v1.md](./PRD-v1.md) for the full anti-scope definition.

---

## Install

### From source

**Prerequisites:**
- [Rust](https://rustup.rs/) 1.75+

```bash
git clone https://github.com/sovereign-shovels/ollama-cron.git
cd ollama-cron

# Build
cargo build --release

# The binary is at target/release/ollama-cron
```

---

## Usage

### Initialize config

```bash
ollama-cron init
# Creates ~/.config/ollama-cron/config.toml with sample jobs
```

### Edit config

```toml
endpoint = "http://localhost:11434/v1/chat/completions"
model = "llama3.2"

[[job]]
name = "daily-brief"
schedule = "0 8 * * *"
prompt = "Give me a 3-bullet summary of today's AI news. Date: {{date}}"
output = "file:///tmp/daily-brief.txt"

[[job]]
name = "hourly-check"
schedule = "0 * * * *"
prompt = "What time is it?"
output = "https://hooks.example.com/endpoint"
```

**Schedule format:** Standard cron (`min hour day month weekday`)

**Variables:** `{{date}}`, `{{datetime}}`

### List jobs

```bash
ollama-cron list
```

### Run a job immediately

```bash
ollama-cron run daily-brief
```

### Start the daemon

```bash
ollama-cron daemon
```

Runs continuously, executing jobs on their schedules.

### Output sinks

- **File:** `output = "file:///path/to/output.txt"` or `output = "/path/to/output.txt"`
- **Webhook:** `output = "https://hooks.example.com/endpoint"`

### Using cloud providers

```toml
endpoint = "https://api.anthropic.com/v1/messages"
api_key_env_var = "ANTHROPIC_API_KEY"
model = "claude-3-5-sonnet-20241022"
```

Any OpenAI-compatible endpoint works (Ollama, OpenAI, Anthropic, OpenRouter, etc.).

---

## Why this exists

Nobody owns this exact niche. n8n is heavier; cron is more primitive; this sits between.

See [PRD-v1.md](./PRD-v1.md) for the full problem statement and rationale.

## What's next

- **v0.5:** Web UI for schedule management, job history, conditional re-runs, templates
- **v1.0:** Multi-step pipelines, job dependencies, notification routing

See [PRD-v1.md](./PRD-v1.md) for the full roadmap.

---

## License

Apache 2.0. See [LICENSE](./LICENSE).

## Part of sovereign-shovels

This repo is part of the [sovereign-shovels](https://github.com/sovereign-shovels) portfolio of small, focused, sovereign-by-construction AI utilities.

Other shovels: claude-vault, bulbul-studio, saaras-tray, claude-prompts, ollama-cron, mcp-forge, sarvam-pdf, agent-console, sarvam-meet, obsidian-llm, llm-diff, claude-bridge, claude-radio, sarvam-cast.
