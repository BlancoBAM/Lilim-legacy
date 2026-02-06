use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LilimConfig {
    pub models: ModelConfig,
    pub api_keys: ApiKeys,
    pub rag: RagConfig,
    pub routing: RoutingConfig,
    pub personality: PersonalityConfig,
    pub agent: AgentConfig,
    pub memory: MemoryConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ModelConfig {
    pub local_model_path: PathBuf,
    pub fallback_provider: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApiKeys {
    #[serde(default)]
    pub openai: Option<String>,
    #[serde(default)]
    pub anthropic: Option<String>,
    #[serde(default)]
    pub gemini: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RagConfig {
    pub vector_index: PathBuf,
    pub keyword_index: PathBuf,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RoutingConfig {
    pub complexity_threshold: f32,
    pub max_local_context_length: usize,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PersonalityConfig {
    pub responses_yaml_path: PathBuf,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AgentConfig {
    pub yolo_mode: bool,
    pub max_tool_calls_per_query: usize,
    pub allowed_search_paths: Vec<String>,
    pub enable_web_search: bool,
    pub enable_file_search: bool,
    pub enable_terminal: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MemoryConfig {
    pub db_path: PathBuf,
    pub max_history_messages: usize,
    pub cleanup_days: i64,
}

impl LilimConfig {
    pub fn from_file(path: &str) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: LilimConfig = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn default() -> Self {
        Self {
            models: ModelConfig {
                local_model_path: PathBuf::from("/home/blanco/models/Qwen2.5-3B-Instruct"),
                fallback_provider: "openai".to_string(),
            },
            api_keys: ApiKeys {
                openai: None,
                anthropic: None,
                gemini: None,
            },
            rag: RagConfig {
                vector_index: PathBuf::from("/home/blanco/Lilim/current/lilith_rag_builder/rag_ready/rag_index"),
                keyword_index: PathBuf::from("/home/blanco/Lilim/current/lilith_rag_builder/rag_ready/keyword_index"),
            },
            routing: RoutingConfig {
                complexity_threshold: 3.0,
                max_local_context_length: 2048,
            },
            personality: PersonalityConfig {
                responses_yaml_path: PathBuf::from("/home/blanco/Lilim/current/lilim-responses.yaml"),
            },
            agent: AgentConfig {
                yolo_mode: false,
                max_tool_calls_per_query: 5,
                allowed_search_paths: vec!["/home/blanco".to_string()],
                enable_web_search: true,
                enable_file_search: true,
                enable_terminal: true,
            },
            memory: MemoryConfig {
                db_path: PathBuf::from("/home/blanco/.local/share/lilim/conversations.db"),
                max_history_messages: 20,
                cleanup_days: 90,
            },
        }
    }
}
