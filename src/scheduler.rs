use anyhow::Result;
use chrono::Utc;
use cron::Schedule;
use std::str::FromStr;
use std::time::Duration;
use tokio::time::sleep;

use crate::config::{Config, Job};
use crate::runner;

pub async fn run_job(cfg: &Config, job: &Job) -> Result<String> {
    let endpoint = job.endpoint.as_deref().unwrap_or_else(|| {
        cfg.endpoint.as_deref().unwrap_or("http://localhost:11434/v1/chat/completions")
    });
    let model = job.model.as_deref().unwrap_or_else(|| {
        cfg.model.as_deref().unwrap_or("llama3.2")
    });
    let api_key = cfg.api_key();

    let prompt = substitute_vars(&job.prompt);
    let result = runner::run_prompt(endpoint, model, &prompt, api_key.as_deref()).await?;

    if let Some(output) = &job.output {
        runner::write_output(output, &result).await?;
    }

    Ok(result)
}

pub async fn daemon(cfg: Config) -> Result<()> {
    println!("Starting ollama-cron daemon with {} job(s)...", cfg.jobs.len());

    loop {
        let now = Utc::now();
        let mut next_runs = Vec::new();

        for job in &cfg.jobs {
            match Schedule::from_str(&job.schedule) {
                Ok(schedule) => {
                    if let Some(next) = schedule.upcoming(Utc).next() {
                        let dur = (next - now).to_std().unwrap_or(Duration::from_secs(0));
                        next_runs.push((dur, job.clone()));
                    }
                }
                Err(e) => eprintln!("Invalid schedule for job '{}': {}", job.name, e),
            }
        }

        if next_runs.is_empty() {
            sleep(Duration::from_secs(60)).await;
            continue;
        }

        next_runs.sort_by(|a, b| a.0.cmp(&b.0));
        let (wait, job) = &next_runs[0];

        println!(
            "[{}] Next job: '{}' in {:?}",
            now.to_rfc3339(),
            job.name,
            wait
        );

        sleep(*wait).await;

        println!("[{}] Running job: '{}'", Utc::now().to_rfc3339(), job.name);
        match run_job(&cfg, job).await {
            Ok(result) => {
                println!("[{}] Job '{}' completed. Output: {} chars", Utc::now().to_rfc3339(), job.name, result.len());
            }
            Err(e) => {
                eprintln!("[{}] Job '{}' failed: {}", Utc::now().to_rfc3339(), job.name, e);
            }
        }
    }
}

fn substitute_vars(prompt: &str) -> String {
    let now = Utc::now();
    prompt
        .replace("{{date}}", &now.format("%Y-%m-%d").to_string())
        .replace("{{datetime}}", &now.to_rfc3339())
}
