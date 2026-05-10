---
repo: ollama-cron
rank: 5
score: 0.76
sprint: 3
substrate_anchor: Ollama
build_estimate: "2–3 weeks for v0.1"
status: planned
---

# PRD v1.0 — ollama-cron

> **One-liner:** Cron, but for AI. Schedule prompts to run anywhere — local Ollama, Claude, GPT — with output to file, webhook, or email.
>
> **Substrate:** LocalLLaMA / selfhost crowd, also cloud-model users who want sovereign scheduling
> **Launch channels:** r/LocalLLaMA, r/selfhosted, r/homelab, HN
> **Build estimate (v0.1):** 2–3 weeks for v0.1

---

## What problem does this solve

Self-hosters and power users want scheduled AI tasks: a daily news brief from their RSS, a weekly summary of a watched repo, an hourly check on a feed. n8n is too heavy. Cron is too primitive (no LLM call). ollama-cron sits between: cron syntax, OpenAI-compatible call, output to file/webhook/email.

## Why this is a shovel and not a product

Nobody owns this niche. Tiny build. Sovereign by default (Ollama is in the name, but any endpoint works). Scope-evolves into a proper scheduling UI without becoming an agent platform.

---

## v0.1 — what ships

CLI daemon. cron-syntax schedule. Endpoint config (any OpenAI-compatible endpoint). Output sinks: file, webhook, email (SMTP). Variable substitution (date, last-run, env).

### Acceptance criteria for v0.1

A v0.1 release is publishable to GitHub when ALL of these are true:

- [ ] Core functionality described above works on the primary developer machine.
- [ ] At least one local-only configuration is documented and tested (no cloud required).
- [ ] BYO endpoint / BYO key configuration is documented.
- [ ] README explains: what it is, who it's for, how to install, how to configure, what it doesn't do.
- [ ] LICENSE present (Apache 2.0 unless overridden).
- [ ] No hardcoded keys or vendor URLs anywhere.
- [ ] No telemetry / phone-home.
- [ ] At least one passing test for the main code path.
- [ ] CI green.
- [ ] AGENTS.md compliance reviewed.

## v0.5 — first major evolution

Web UI for schedule management. Job history. Conditional re-runs. Templates for common patterns (RSS-summarize, repo-watch, daily-brief).

## v1.0 — fuller scope

Multi-step pipelines. Job dependencies. Notification routing.

---

## Architecture sketch

### Stack

Rust or Go for a single static binary. systemd unit + launchd plist + Windows service for OS integration. SQLite for job state.

### Provider abstraction

The shovel MUST expose a provider abstraction even if v0.1 only uses one
provider. Suggested shape:

```
interface Provider {
  name: string;
  endpoint: URL;
  apiKeyEnvVar: string;
  call(input: ProviderInput): Promise<ProviderOutput>;
}
```

The default config in v0.1 must point to a free, local provider where
applicable, and document how to swap in any other.

### Configuration

Configuration order of precedence (highest to lowest):

1. Command-line flags
2. Environment variables (prefix: `OLLAMA_CRON_*`)
3. User config file (`~/.config/ollama-cron/config.toml` on Linux/Mac, equivalent on Windows)
4. Default config (shipped, but never with secrets)

---

## Anti-scope (do NOT build)

Not an agent platform. Not a workflow builder. Not n8n. Scheduled prompts only. Resist the agent-platform gravity well.

---

## Tombstone risk and mitigation

**Risk:** Very low. Nobody is owning this exact niche. n8n is heavier; cron is more primitive; this sits between.

**Mitigation:** Ship fast (v0.1 in 2–3 weeks for v0.1). Build community early
(launch on r/LocalLLaMA, r/selfhosted, r/homelab, HN). Even if upstream absorbs the feature, accumulated
stars and the community are the audience-build payoff.

**Kill signal:** Scope creep. The day someone PRs 'just add tool calling' is the day the project dies if accepted.

If the kill signal triggers, the maintainer must announce within one week and
either (a) refocus on a remaining gap, (b) merge gracefully into upstream if
they're receptive, or (c) mark the repo as archived with a clear pointer to the
replacement.

---

## Launch plan

### Pre-launch checklist

- [ ] Repo on GitHub at `github.com/sovereign-shovels/ollama-cron`
- [ ] README polished (see template in `_templates/`)
- [ ] At least 3 issues / discussions seeded (real ones, not placeholder)
- [ ] LICENSE, CODE_OF_CONDUCT, CONTRIBUTING present
- [ ] Demo asset (gif, screenshot, or short video — depending on category)
- [ ] First-launch post drafted for primary launch channel

### Day-1 launch

Post to: r/LocalLLaMA, r/selfhosted, r/homelab, HN

Subject template (adjust per channel):
- Show HN: `Show HN: ollama-cron – Cron, but for AI. Schedule prompts to run anywhere — local Ollama, Claude, GPT — with output to file, webhook, or email.`
- Reddit: `[OSS] Cron, but for AI. Schedule prompts to run anywhere — local Ollama, Claude, GPT — with output to file, webhook, or email.` with full post explaining the gap and the build
- Twitter/X: thread leading with the demo gif

### Week-1 follow-up

- Respond to every issue and comment within 24h.
- Ship at least one bugfix release based on launch feedback.
- Cross-post to secondary channels.

### Month-1 review

- Assess star velocity and community formation.
- If kill signal triggered, follow tombstone protocol above.
- If trajectory is healthy, plan v0.5.

---

## Cross-references

- Constitution: [[AGENTS]]
- Public README: [[README]]
- Progress frontmatter: [[progress]]
- Internal knowledge graph: [[knowledge-graph]]
