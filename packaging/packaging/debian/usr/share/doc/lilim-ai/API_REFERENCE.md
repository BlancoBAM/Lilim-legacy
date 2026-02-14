# Lilim API Reference

## Base URL

```
http://localhost:8080
```

---

## Endpoints

### POST /chat

Send a chat query to Lilim and receive a response.

**Request Headers**:
```
Content-Type: application/json
```

**Request Body**:
```json
{
  "text": "How do I fix broken APT packages?",
  "session_id": "session_1234567890",
  "tools_enabled": true,
  "yolo_mode": false
}
```

**Request Parameters**:

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `text` | string | Yes | The user's query/question |
| `session_id` | string | No | Session identifier for conversation memory (auto-generated if omitted) |
| `tools_enabled` | boolean | No | Enable tool execution (web search, file search, terminal). Default: `false` |
| `yolo_mode` | boolean | No | Auto-execute safe commands without confirmation. Default: `false` **CAUTION** |

**Response** (200 OK):
```json
{
  "response": "To fix broken packages in APT, run 'sudo apt --fix-broken install'...",
  "source": "local",
  "domain": "Linux"
}
```

**Response Fields**:

| Field | Type | Description |
|-------|------|-------------|
| `response` | string | Lilim's answer with personality formatting applied |
| `source` | string | Where the answer came from: `"local"`, `"api-openai"`, `"api-anthropic"`, `"local-fallback"` |
| `domain` | string | Detected topic domain: `"Linux"`, `"Medical"`, `"Academic"`, `"Technical"`, `"General"` |

**Error Responses**:

**400 Bad Request**:
```json
{
  "error": "Missing required field: text"
}
```

**500 Internal Server Error**:
```json
{
  "error": "RAG search failed: index not found"
}
```

**503 Service Unavailable**:
```json
{
  "error": "API fallback unavailable and local model not configured"
}
```

---

### GET /health

Check if the Lilim server is running.

**Response** (200 OK):
```json
{
  "status": "ok",
  "version": "0.1.0"
}
```

---

## Usage Examples

### cURL

**Basic query**:
```bash
curl -X POST http://localhost:8080/chat \
  -H "Content-Type: application/json" \
  -d '{"text": "What are normal vital signs?", "session_id": "test123"}'
```

**With tools enabled**:
```bash
curl -X POST http://localhost:8080/chat \
  -H "Content-Type: application/json" \
  -d '{
    "text": "Search the web for Ubuntu 24.04 release notes",
    "session_id": "test123",
    "tools_enabled": true,
    "yolo_mode": false
  }'
```

**Health check**:
```bash
curl http://localhost:8080/health
```

### JavaScript/TypeScript

**Fetch API**:
```javascript
async function askLilim(question) {
  const response = await fetch('http://localhost:8080/chat', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({
      text: question,
      session_id: localStorage.getItem('lilim_session') || 'default',
      tools_enabled: true,
      yolo_mode: false,
    }),
  });

  if (!response.ok) {
    throw new Error(`API error: ${response.statusText}`);
  }

  const data = await response.json();
  return data;
}

// Usage
askLilim('How do I restart a systemd service?')
  .then(data => {
    console.log('Response:', data.response);
    console.log('Source:', data.source);
    console.log('Domain:', data.domain);
  })
  .catch(err => console.error(err));
```

**Axios**:
```javascript
const axios = require('axios');

axios.post('http://localhost:8080/chat', {
  text: 'Explain the cardiovascular system',
  session_id: 'user_alice',
  tools_enabled: false,
})
.then(response => {
  console.log(response.data.response);
})
.catch(error => {
  console.error('Error:', error.response?.data || error.message);
});
```

### Python

**requests library**:
```python
import requests

def ask_lilim(question, session_id='default'):
    url = 'http://localhost:8080/chat'
    payload = {
        'text': question,
        'session_id': session_id,
        'tools_enabled': True,
        'yolo_mode': False
    }
    
    try:
        response = requests.post(url, json=payload)
        response.raise_for_status()
        data = response.json()
        
        print(f"Response: {data['response']}")
        print(f"Source: {data['source']}")
        print(f"Domain: {data['domain']}")
        
        return data
    except requests.exceptions.RequestException as e:
        print(f"Error: {e}")

# Usage
ask_lilim("How do I check disk space in Linux?")
```

### Rust

**reqwest (blocking)**:
```rust
use serde::{Deserialize, Serialize};
use reqwest::blocking::Client;

#[derive(Serialize)]
struct LilimQuery {
    text: String,
    session_id: String,
    tools_enabled: bool,
    yolo_mode: bool,
}

#[derive(Deserialize)]
struct LilimResponse {
    response: String,
    source: String,
    domain: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    
    let query = LilimQuery {
        text: "What is HIPAA?".to_string(),
        session_id: "rust_client".to_string(),
        tools_enabled: false,
        yolo_mode: false,
    };
    
    let resp = client
        .post("http://localhost:8080/chat")
        .json(&query)
        .send()?
        .json::<LilimResponse>()?;
    
    println!("Response: {}", resp.response);
    println!("Source: {}", resp.source);
    println!("Domain: {}", resp.domain);
    
    Ok(())
}
```

---

## Response Source Types

| Source | Description |
|--------|-------------|
| `local` | Answer generated from local RAG database only |
| `api-openai` | Answer from OpenAI API (GPT models) |
| `api-anthropic` | Answer from Anthropic API (Claude models) |
| `local-fallback` | Attempted API call failed, fell back to local |

---

## Domain Classification

Lilim automatically detects the topic domain:

- **Linux**: System administration, package management, troubleshooting
- **Medical**: Anatomy, pharmacology, patient care, terminology
- **Academic**: Study skills, writing, research, test preparation
- **Technical**: Programming, software development, general tech
- **General**: Everything else

Domain detection influences:
- RAG search weighting (prioritizes matching domain content)
- Routing decisions (complex medical questions may use API fallback)
- Response formatting and personality injection

---

## Rate Limiting

Currently, no rate limiting is enforced. For production deployments, consider adding:
- Request rate limiting per IP/session
- Concurrent request limits
- Query complexity throttling

---

## WebSocket Support

**Not currently implemented**. Future plans include WebSocket endpoint for:
- Streaming responses
- Real-time tool execution updates
- Server-sent events for long-running queries

---

## Authentication

**Not currently implemented**. The API operates on localhost without authentication.

For network deployment, implement:
- API key authentication via headers
- Session tokens
- OAuth/OIDC integration

---

## CORS Configuration

Configured domains (can be modified in `/etc/lilith/lilim.yaml`):
```yaml
server:
  cors_origins:
    - "http://localhost:5173"  # Vite dev server
    - "http://localhost:8080"  # Production
```

---

## Error Handling Best Practices

1. **Always check HTTP status codes**
2. **Parse error responses** for details
3. **Implement exponential backoff** for retries
4. **Handle timeout scenarios** (default: 30s)
5. **Validate JSON** before parsing

---

**The API is your gateway to Lilim's knowledge. Use it wisely.**
