use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub jobs: Vec<Job>,
    #[serde(default)]
    pub endpoint: Option<String>,
    #[serde(default)]
    pub api_key_env_var: Option<String>,
    #[serde(default)]
    pub model: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    pub name: String,
    pub schedule: String,
    pub prompt: String,
    #[serde(default)]
    pub endpoint: Option<String>,
    #[serde(default)]
    pub model: Option<String>,
    #[serde(default)]
    pub output: Option<String>,
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        let path = Self::path()?;
        if path.exists() {
            let content = std::fs::read_to_string(&path)?;
            let cfg: Config = toml::from_str(&content)?;
            Ok(cfg)
        } else {
            Ok(Config::default())
        }
    }

    pub fn path() -> anyhow::Result<PathBuf> {
        let base = dirs::config_dir().ok_or_else(|| anyhow::anyhow!("No config dir"))?;
        Ok(base.join("ollama-cron").join("config.toml"))
    }

    pub fn api_key(&self) -> Option<String> {
        self.api_key_env_var.as_ref().and_then(|k| std::env::var(k).ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let cfg = Config::default();
        assert!(cfg.endpoint.is_some());
        assert_eq!(cfg.model.unwrap(), "llama3.2");
        assert!(cfg.jobs.is_empty());
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            jobs: vec![],
            endpoint: Some("http://localhost:11434/v1/chat/completions".into()),
            api_key_env_var: None,
            model: Some("llama3.2".into()),
        }
    }
}
