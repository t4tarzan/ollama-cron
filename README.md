# ollama-cron

> Cron, but for AI. Schedule prompts to run anywhere — local Ollama, Claude, GPT — with output to file, webhook, or email.

**Status:** v0.1 — planning. Not yet released.

**Sovereignty:** sovereign-by-construction. BYO endpoint, BYO key, BYO model.
A local-only configuration is documented and tested.

This is a community project, **not affiliated with Ollama**.
Best-effort community shovel — no SLA, no roadmap commitments.

---

## What this is

Cron, but for AI. Schedule prompts to run anywhere — local Ollama, Claude, GPT — with output to file, webhook, or email.

## What this isn't

Not n8n. Not an agent platform. Not a workflow builder. SCHEDULED PROMPTS ONLY.

## Install

> Coming with v0.1 release.

## Configure

You bring the model. By default `ollama-cron` tries to use a local provider:

- For LLM endpoints: Ollama at `http://localhost:11434`
- For voice endpoints: configurable, see [docs/configure.md]

To use any other provider (Claude, GPT, Hermes, OpenRouter, Sarvam, etc.):

```toml
# ~/.config/ollama-cron/config.toml
[provider]
endpoint = "https://api.your-provider.com/v1"
api_key_env = "YOUR_PROVIDER_KEY"
model = "your-model-name"
```

Anthropic, OpenAI, and Sarvam endpoints all work. Local Ollama, llama.cpp,
LM Studio, and vLLM all work via their OpenAI-compatible endpoints.

## Why this exists

Self-hosters and power users want scheduled AI tasks: a daily news brief from their RSS, a weekly summary of a watched repo, an hourly check on a feed. n8n is too heavy. Cron is too primitive (no LLM call). ollama-cron sits between: cron syntax, OpenAI-compatible call, output to file/webhook/email.

## What's next

See [PRD-v1.md](./PRD-v1.md) for the full v0.1 → v0.5 → v1.0 plan.

## License

Apache 2.0. See [LICENSE](./LICENSE).

## Part of sovereign-shovels

This repo is part of the [sovereign-shovels](https://github.com/sovereign-shovels)
portfolio of small, focused, sovereign-by-construction AI utilities.

Other shovels: claude-vault, bulbul-studio, saaras-tray, claude-prompts,
ollama-cron, mcp-forge, sarvam-pdf, agent-console, sarvam-meet, obsidian-llm,
llm-diff, claude-bridge, claude-radio, sarvam-cast.
