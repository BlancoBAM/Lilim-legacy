# Lilim AI Assistant - System Architecture

## Overview

Lilim is an intelligent multi-model routing system that combines local AI models with online API providers to deliver fast, accurate, and context-aware responses.

## Architecture Diagram

```
┌─────────────────────────────────────────────┐
│      Lilim Desktop UI (Tauri + React)       │
│    System Tray • Global Hotkeys • Native   │
└──────────────────┬──────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────┐
│      lilim_server (HTTP API Server)         │
│         Port 8080 • REST Endpoints          │
└──────────────────┬──────────────────────────┘
                   │
      ┌────────────┼────────────┐
      ▼            ▼            ▼
┌──────────┐ ┌──────────┐ ┌──────────────┐
│  Router  │ │   Main   │ │  Online API  │
│  Model   │ │  Qwen3   │ │  Providers   │
│ (0.5B)   │ │  (4B VL) │ │  (Fallback)  │
└──────────┘ └────┬─────┘ └──────────────┘
                  │
         ┌────────┼────────┐
         ▼        ▼        ▼
    ┌──────┐ ┌──────┐ ┌──────┐
    │ RAG  │ │ TTS  │ │Tools │
    │Engine│ │neutts│ │ Exec │
    └──────┘ └──────┘ └──────┘
```

## Component Details

### 1. Router Model
- **Purpose**: Fast query classification (<500ms)
- **Model**: Qwen2-0.5B (quantized)
- **Responsibilities**:
  - Determine query complexity
  - Classify task type (coding, medical, general, etc.)
  - Decide executor (local vs online)
  - Estimate latency

### 2. Main Agentic Model
- **Purpose**: Primary local reasoning engine
- **Model**: Qwen3-VL 4B (with vision)
- **Capabilities**:
  - Multi-turn conversations
  - Tool use (web search, file ops, terminal)
  - Vision understanding
  - RAG integration

### 3. Online Providers
- **Purpose**: Fallback for complex queries
- **Trigger Conditions**:
  - Query complexity > 0.9
  - Estimated local latency > 5s
  - Requires latest information
  - User explicitly requests
- **Providers**: Configurable (OpenAI, Anthropic, custom)

### 4. TTS Engine
- **Model**: neutts-nano-q4-gguf
- **Features**:
  - Offline synthesis
  - Fast generation
  - High quality output
  - Low resource usage

### 5. RAG Engine
- **Vector Index**: fastembed + embedvec
- **Keyword Index**: tantivy
- **Domains**: Linux, Medical, Academic (80+ entries)

## Data Flow

1. **User Input** → Desktop UI or CLI
2. **Router Analysis** → Classify and route
3. **Executor Selection**:
   - Simple queries → Router handles directly
   - Medium complexity → Main local model
   - Complex/time-sensitive → Online provider
4. **RAG Retrieval** (if applicable)
5. **Tool Execution** (if needed)
6. **Response Generation**
7. **Personality Application**
8. **TTS Synthesis** (if requested)
9. **Return to User**

## Routing Decision Logic

```python
def route_query(query, context):
    analysis = router_model.analyze(query)
    
    if analysis.complexity < 0.2:
        return RouterExecutor
    
    if rag_relevance(query) > 0.8:
        return LocalMainModel
    
    estimated_latency = estimate_local_time(analysis)
    if estimated_latency > MAX_LOCAL_LATENCY:
        return OnlineProvider
    
    if analysis.task_type in LOCAL_TASKS:
        return LocalMainModel
    
    return OnlineProvider
```

## Security

- **Sandboxing**: Systemd service runs with strict security policies
- **Command Validation**: Terminal commands require confirmation
- **API Keys**: Environment-based, not stored in code
- **Local-First**: Processing happens locally by default

## Performance

- **Router**: <500ms per query
- **Main Model**: 1-3s for typical queries
- **RAG Retrieval**: <100ms
- **TTS Generation**: ~2s per sentence

## Scalability

- **Concurrent Queries**: Up to 10 simultaneous
- **Model Loading**: On-demand with caching
- **Memory Usage**: ~4GB for all models loaded
- **Disk Space**: ~6GB for models, 1GB for indices
