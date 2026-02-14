use anyhow::Result;
use serde::{Deserialize, Serialize};
use reqwest::Client;

#[derive(Debug, Clone)]
pub struct ApiClient {
    client: Client,
    provider: String,
    api_key: String,
}

#[derive(Debug, Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<OpenAIMessage>,
    temperature: f32,
}

#[derive(Debug, Serialize)]
struct OpenAIMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct OpenAIResponse {
    choices: Vec<OpenAIChoice>,
}

#[derive(Debug, Deserialize)]
struct OpenAIChoice {
    message: OpenAIMessage_,
}

#[derive(Debug, Deserialize)]
struct OpenAIMessage_ {
    content: String,
}

impl ApiClient {
    pub fn new(provider: String, api_key: String) -> Self {
        Self {
            client: Client::new(),
            provider,
            api_key,
        }
    }

    /// Call external LLM API with the full prompt
    pub async fn generate(&self, prompt: &str) -> Result<String> {
        match self.provider.as_str() {
            "openai" => self.call_openai(prompt).await,
            "anthropic" => self.call_anthropic(prompt).await,
            _ => Err(anyhow::anyhow!("Unsupported provider: {}", self.provider)),
        }
    }

    async fn call_openai(&self, prompt: &str) -> Result<String> {
        let request = OpenAIRequest {
            model: "gpt-4o-mini".to_string(),
            messages: vec![OpenAIMessage {
                role: "user".to_string(),
                content: prompt.to_string(),
            }],
            temperature: 0.7,
        };

        let response = self.client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await?;

        let data: OpenAIResponse = response.json().await?;
        Ok(data.choices[0].message.content.clone())
    }

    async fn call_anthropic(&self, prompt: &str) -> Result<String> {
        // Placeholder for Anthropic API
        // Similar structure to OpenAI
        Err(anyhow::anyhow!("Anthropic integration not yet implemented"))
    }
}
