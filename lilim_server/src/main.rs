mod router;
mod personality;
mod api;
mod tools;

use anyhow::Result;
use axum::{
    extract::State,
    routing::post,
    Json, Router,
};
use lilim_core::{RagEngine, ContextBuilder, LilimConfig, MemoryEngine, Message};
use router::{LilimQuery, LilimResponse, QueryAnalyzer, category_to_string};
use personality::{PersonalityFormatter, ResponseType};
use api::ApiClient;
use tools::{ToolArgs, ToolResult, WebSearchTool, FileSearchTool, TerminalTool, Tool};
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::Mutex;
use tracing::info;
use serde::{Deserialize, Serialize};

// Extended query with memory and tools support
#[derive(Debug, Deserialize)]
struct ExtendedQuery {
    text: String,
    #[serde(default)]
    session_id: Option<String>,
    #[serde(default)]
    tools_enabled: bool,
    #[serde(default)]
    yolo_mode: Option<bool>,
}

struct AppState {
    rag: Arc<Mutex<RagEngine>>,
    context_builder: ContextBuilder,
    config: LilimConfig,
    personality: PersonalityFormatter,
    api_client: Option<ApiClient>,
    memory: Arc<Mutex<MemoryEngine>>,
    tools: Arc<Mutex<HashMap<String, Box<dyn Tool>>>>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    info!("Starting Lilim Server with Memory + Tools...");

    // Load config
    let config = LilimConfig::default();

    // Initialize RAG engine
    let rag = RagEngine::new(&config.rag.vector_index, &config.rag.keyword_index)?;
    
    // Initialize memory
    let memory = MemoryEngine::new(config.memory.db_path.to_str().unwrap())?;

    // Initialize components
    let context_builder = ContextBuilder::new();
    let personality = PersonalityFormatter::new(config.personality.responses_yaml_path.to_str().unwrap())?;
    
    // Initialize API client if key exists
    let api_client = if let Some(key) = &config.api_keys.openai {
        Some(ApiClient::new(config.models.fallback_provider.clone(), key.clone()))
    } else {
        None
    };

    // Initialize tools
    let mut tools: HashMap<String, Box<dyn Tool>> = HashMap::new();
    if config.agent.enable_web_search {
        tools.insert("web_search".to_string(), Box::new(WebSearchTool::new()));
    }
    if config.agent.enable_file_search {
        tools.insert("file_search".to_string(), Box::new(FileSearchTool::new()));
    }
    if config.agent.enable_terminal {
        tools.insert("terminal".to_string(), Box::new(TerminalTool::new(config.agent.yolo_mode)));
    }

    let state = Arc::new(AppState {
        rag: Arc::new(Mutex::new(rag)),
        context_builder,
        config,
        personality,
        api_client,
        memory: Arc::new(Mutex::new(memory)),
        tools: Arc::new(Mutex::new(tools)),
    });

    // Build routes
    let app = Router::new()
        .route("/chat", post(chat_handler))
        .with_state(state);

    // Start server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;
    info!("Lilim Server listening on http://127.0.0.1:8080");
    info!("Features: RAG ✓ | Memory ✓ | Tools (web/file/terminal) ✓");
    
    axum::serve(listener, app).await?;

    Ok(())
}

async fn chat_handler(
    State(state): State<Arc<AppState>>,
    Json(query): Json<ExtendedQuery>,
) -> Json<LilimResponse> {
    info!("Received query: {}", query.text);

    let session_id = query.session_id.unwrap_or_else(|| "default".to_string());

    // 1. Load conversation history
    let history = {
        let memory = state.memory.lock().await;
        memory.get_history(&session_id, state.config.memory.max_history_messages)
            .unwrap_or_default()
    };

    info!("Loaded {} previous messages", history.len());

    // 2. Detect domain
    let domain = {
        let rag = state.rag.lock().await;
        rag.detect_domain(&query.text)
    };

    // 3. Perform RAG search
    let rag_results = {
        let mut rag = state.rag.lock().await;
        rag.search(&query.text, Some(domain.clone()), 5).await.unwrap_or_default()
    };

    // 4. Build context prompt with history
    let mut prompt = state.context_builder.build_prompt_with_context(&query.text, &rag_results, &domain);
    
    // Add conversation history to prompt
    if !history.is_empty() {
        let mut history_str = String::from("\n\n## Previous Conversation:\n");
        for msg in history.iter().take(10) {
            history_str.push_str(&format!("{}: {}\n", msg.role, msg.content));
        }
        prompt = format!("{}{}\n\nCurrent query: {}", history_str, prompt, query.text);
    }

    // 5. Routing decision
    let use_local = QueryAnalyzer::should_use_local(
        &query.text,
        !rag_results.is_empty(),
        state.config.routing.complexity_threshold,
    );

    // 6. Generate response
    let (raw_response, source) = if use_local {
        // TODO: Integrate Crane local model
        (format!("Local model response (Crane pending). Context: {} RAG results, {} history msgs", 
            rag_results.len(), history.len()), 
         "local".to_string())
    } else if let Some(ref api) = state.api_client {
        match api.generate(&prompt).await {
            Ok(resp) => (resp, format!("api-{}", state.config.models.fallback_provider)),
            Err(e) => {
                info!("API call failed: {}, falling back to local", e);
                ("API unavailable".to_string(), "local-fallback".to_string())
            }
        }
    } else {
        ("No API configured".to_string(), "local".to_string())
    };

    // 7. Save to memory
    {
        let memory = state.memory.lock().await;
        let conv_id = memory.get_or_create_session(&session_id).unwrap_or(0);
        let _ = memory.save_message(conv_id, "user", &query.text);
        let _ = memory.save_message(conv_id, "assistant", &raw_response);
    }

    // 8. Apply personality formatting
    let formatted_response = state.personality.format_response(&raw_response, ResponseType::Complete);

    Json(LilimResponse {
        response: formatted_response,
        source,
        domain: category_to_string(&domain),
    })
}
