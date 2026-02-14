use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize)]
pub struct LilimResponses {
    #[serde(rename = "infernalResponses")]
    pub infernal: InfernalResponses,
    #[serde(rename = "longResponses")]
    pub long: HashMap<String, ResponseTemplate>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InfernalResponses {
    pub greet: Vec<String>,
    pub search: Vec<String>,
    pub complete: Vec<String>,
    pub error: Vec<String>,
    pub thinking: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ResponseTemplate {
    pub prefix: String,
    pub content: String,
}

pub struct PersonalityFormatter {
    responses: LilimResponses,
}

impl PersonalityFormatter {
    pub fn new(yaml_path: &str) -> Result<Self> {
        let content = std::fs::read_to_string(yaml_path)?;
        let responses: LilimResponses = serde_yaml::from_str(&content)?;
        Ok(Self { responses })
    }

    /// Wrap the LLM response with Lilim's personality
    pub fn format_response(&self, raw_response: &str, response_type: ResponseType) -> String {
        match response_type {
            ResponseType::Complete => {
                let prefix = self.random_from(&self.responses.infernal.complete);
                format!("{}\n\n{}", prefix, raw_response)
            }
            ResponseType::Search => {
                let prefix = self.random_from(&self.responses.infernal.search);
                format!("{}\n\n{}", prefix, raw_response)
            }
            ResponseType::Error => {
                let prefix = self.random_from(&self.responses.infernal.error);
                format!("{}\n\n{}", prefix, raw_response)
            }
            ResponseType::Greet => {
                self.random_from(&self.responses.infernal.greet).to_string()
            }
            ResponseType::Plain => raw_response.to_string(),
        }
    }

    fn random_from<'a>(&self, options: &'a [String]) -> &'a str {
        use std::collections::hash_map::RandomState;
        use std::hash::{BuildHasher, Hash, Hasher};
        
        let hasher = RandomState::new();
        let mut hasher = hasher.build_hasher();
        std::time::SystemTime::now().hash(&mut hasher);
        let index = (hasher.finish() as usize) % options.len();
        &options[index]
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ResponseType {
    Complete,
    Search,
    Error,
    Greet,
    Plain,
}
