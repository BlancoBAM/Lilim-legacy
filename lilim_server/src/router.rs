use anyhow::Result;
use serde::{Deserialize, Serialize};
use lilim_core::rag::Category;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LilimQuery {
    pub text: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LilimResponse {
    pub response: String,
    pub source: String,  // "local" or "api-{provider}"
    pub domain: String,  // "Medical", "SysAdmin", "General"
}

pub struct QueryAnalyzer;

impl QueryAnalyzer {
    /// Estimate query complexity based on various factors
    pub fn analyze_complexity(query: &str) -> f32 {
        let mut score: f32 = 1.0;

        // Length factor
        let word_count = query.split_whitespace().count();
        if word_count > 50 {
            score += 1.0;
        }
        if word_count > 100 {
            score += 1.0;
        }

        // Complexity keywords
        let complex_keywords = ["explain in detail", "write", "essay", "analyze", "compare", "comprehensive"];
        for keyword in complex_keywords {
            if query.to_lowercase().contains(keyword) {
                score += 1.5;
            }
        }

        // Command/action keywords (simple)
        let simple_keywords = ["how to", "what is", "show me", "list", "check"];
        for keyword in simple_keywords {
            if query.to_lowercase().contains(keyword) {
                score -= 0.5;
            }
        }

        score.max(1.0)
    }

    /// Determine if query should be routed to local or API
    pub fn should_use_local(query: &str, has_rag_context: bool, threshold: f32) -> bool {
        let complexity = Self::analyze_complexity(query);
        
        // Boost local if we have good RAG context
        let effective_complexity = if has_rag_context {
            complexity - 0.5
        } else {
            complexity
        };

        effective_complexity < threshold
    }
}

pub fn category_to_string(cat: &Category) -> String {
    match cat {
        Category::Medical => "Medical".to_string(),
        Category::SysAdmin => "SysAdmin".to_string(),
        Category::General => "General".to_string(),
    }
}
