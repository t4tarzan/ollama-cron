use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    stream: bool,
}

#[derive(Debug, Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Debug, Deserialize)]
struct ResponseMessage {
    content: String,
}

pub async fn run_prompt(endpoint: &str, model: &str, prompt: &str, api_key: Option<&str>) -> Result<String> {
    let client = reqwest::Client::new();
    let body = ChatRequest {
        model: model.into(),
        messages: vec![Message {
            role: "user".into(),
            content: prompt.into(),
        }],
        stream: false,
    };

    let mut req = client.post(endpoint).json(&body);
    if let Some(key) = api_key {
        req = req.header("Authorization", format!("Bearer {}", key));
    }

    let response = req.send().await?;
    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        return Err(anyhow::anyhow!("API error ({}): {}", status, text));
    }

    let parsed: ChatResponse = response.json().await?;
    let content = parsed
        .choices
        .into_iter()
        .next()
        .map(|c| c.message.content)
        .unwrap_or_default();

    Ok(content)
}

pub async fn write_output(output: &str, content: &str) -> Result<()> {
    if output.starts_with("file://") || !output.contains("://") {
        let path = output.trim_start_matches("file://");
        tokio::fs::write(path, content).await?;
    } else if output.starts_with("http://") || output.starts_with("https://") {
        let client = reqwest::Client::new();
        client
            .post(output)
            .json(&json!({ "content": content }))
            .send()
            .await?;
    }
    Ok(())
}
