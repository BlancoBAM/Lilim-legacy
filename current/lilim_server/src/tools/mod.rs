use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mod web_search;
mod file_search;
mod terminal;

pub use web_search::WebSearchTool;
pub use file_search::FileSearchTool;
pub use terminal::TerminalTool;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ToolArgs {
    Map(HashMap<String, serde_json::Value>),
}

impl ToolArgs {
    pub fn get_str(&self, key: &str) -> Result<&str> {
        match self {
            ToolArgs::Map(map) => {
                map.get(key)
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| anyhow::anyhow!("Missing or invalid arg: {}", key))
            }
        }
    }

    pub fn get_bool(&self, key: &str, default: bool) -> bool {
        match self {
            ToolArgs::Map(map) => {
                map.get(key)
                    .and_then(|v| v.as_bool())
                    .unwrap_or(default)
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSearchResult {
    pub title: String,
    pub url: String,
    pub snippet: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMatch {
    pub path: String,
    pub matches: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ToolResult {
    WebSearch {
        results: Vec<WebSearchResult>,
        query: String,
    },
    FileSearch {
        matches: Vec<FileMatch>,
        query: String,
    },
    Terminal {
        stdout: String,
        stderr: String,
        exit_code: Option<i32>,
    },
    NeedsConfirmation {
        tool: String,
        args: ToolArgs,
        confirmation_id: String,
    },
}

#[async_trait::async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn requires_confirmation(&self) -> bool;
    async fn execute(&self, args: &ToolArgs) -> Result<ToolResult>;
}
