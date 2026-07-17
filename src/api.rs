use crate::config::Config;
use anyhow::{Context, Result};
use async_stream::stream;
use futures::Stream;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
}

#[derive(Debug, Deserialize)]
struct StreamChunk {
    choices: Vec<StreamChoice>,
}

#[derive(Debug, Deserialize)]
struct StreamChoice {
    delta: StreamDelta,
    #[allow(dead_code)]
    #[serde(default)]
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct StreamDelta {
    #[serde(default)]
    content: Option<String>,
    #[serde(default)]
    reasoning_content: Option<String>,
}

#[derive(Debug, Clone)]
pub enum StreamEvent {
    Content(String),
    Reasoning(String),
    Done,
    Error(String),
}

pub struct ApiClient {
    config: Config,
    client: reqwest::Client,
}

impl ApiClient {
    pub fn new(config: Config) -> Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(300))
            .build()
            .context("failed to create HTTP client")?;
        Ok(Self { config, client })
    }

    pub fn stream_chat(
        &self,
        messages: Vec<Message>,
    ) -> impl Stream<Item = StreamEvent> + '_ {
        let config = self.config.clone();
        let client = self.client.clone();

        stream! {
            let body = ChatRequest {
                model: config.model.clone(),
                messages,
                stream: true,
                max_tokens: Some(config.max_tokens),
                temperature: Some(config.temperature),
            };

            let url = format!("{}/v1/chat/completions", config.base_url);

            let resp = match client
                .post(&url)
                .header("Authorization", format!("Bearer {}", config.api_key))
                .header("Content-Type", "application/json")
                .json(&body)
                .send()
                .await
            {
                Ok(r) => r,
                Err(e) => {
                    yield StreamEvent::Error(format!("Request failed: {}", e));
                    return;
                }
            };

            if !resp.status().is_success() {
                let status = resp.status();
                let text = resp.text().await.unwrap_or_default();
                yield StreamEvent::Error(format!("API error {}: {}", status, text));
                return;
            }

            use futures::StreamExt;
            let mut stream = resp.bytes_stream();

            while let Some(chunk) = stream.next().await {
                let bytes = match chunk {
                    Ok(b) => b,
                    Err(e) => {
                        yield StreamEvent::Error(format!("Stream error: {}", e));
                        return;
                    }
                };

                let text = String::from_utf8_lossy(&bytes);
                for line in text.lines() {
                    let line = line.trim();
                    if line.is_empty() {
                        continue;
                    }
                    if line == "data: [DONE]" {
                        yield StreamEvent::Done;
                        return;
                    }
                    if let Some(json) = line.strip_prefix("data: ") {
                        match serde_json::from_str::<StreamChunk>(json) {
                            Ok(chunk) => {
                                for choice in &chunk.choices {
                                    if let Some(reasoning) = &choice.delta.reasoning_content {
                                        yield StreamEvent::Reasoning(reasoning.clone());
                                    }
                                    if let Some(content) = &choice.delta.content {
                                        yield StreamEvent::Content(content.clone());
                                    }
                                }
                            }
                            Err(_) => {
                                // ignore parse errors for partial chunks
                            }
                        }
                    }
                }
            }
            yield StreamEvent::Done;
        }
    }

    pub async fn chat(&self, messages: Vec<Message>) -> Result<String> {
        use futures::StreamExt;
        let mut stream = Box::pin(self.stream_chat(messages));
        let mut content = String::new();

        while let Some(event) = stream.next().await {
            match event {
                StreamEvent::Content(text) => {
                    content.push_str(&text);
                }
                StreamEvent::Reasoning(text) => {
                    content.push_str(&text);
                }
                StreamEvent::Done => break,
                StreamEvent::Error(e) => anyhow::bail!(e),
            }
        }
        Ok(content)
    }
}
