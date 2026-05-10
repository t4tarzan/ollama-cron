use anyhow::Result;
use clap::{Parser, Subcommand};
use std::fs;

mod config;
mod runner;
mod scheduler;

use config::Config;

#[derive(Parser)]
#[command(name = "ollama-cron")]
#[command(about = "Cron, but for AI. Schedule prompts to run anywhere.")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a job immediately
    Run {
        /// Job name
        name: String,
    },
    /// Start the scheduling daemon
    Daemon,
    /// List configured jobs
    List,
    /// Create a sample config file
    Init,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let cfg = Config::load()?;

    match cli.command {
        Commands::Run { name } => {
            let job = cfg
                .jobs
                .iter()
                .find(|j| j.name == name)
                .ok_or_else(|| anyhow::anyhow!("Job '{}' not found", name))?;

            println!("Running job '{}'...", name);
            let result = scheduler::run_job(&cfg, job).await?;
            println!("\n{}", result);
        }

        Commands::Daemon => {
            scheduler::daemon(cfg).await?;
        }

        Commands::List => {
            if cfg.jobs.is_empty() {
                println!("No jobs configured.");
            } else {
                for job in &cfg.jobs {
                    println!(
                        "{} | {} | {}",
                        job.name, job.schedule, job.prompt
                    );
                }
            }
        }

        Commands::Init => {
            let path = Config::path()?;
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }

            let sample = r#"endpoint = "http://localhost:11434/v1/chat/completions"
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
output = "file:///tmp/hourly-check.txt"
"#;

            fs::write(&path, sample)?;
            println!("Created sample config at: {}", path.display());
        }
    }

    Ok(())
}
