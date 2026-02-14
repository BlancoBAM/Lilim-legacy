# Lilim AI Assistant - User Manual

## 🔥 Welcome to Lilim

Lilim is your personal AI assistant integrated into Lilith Linux. Designed with an infernal aesthetic and sarcastic wit, Lilim specializes in:
- **Linux/Ubuntu troubleshooting** and system administration
- **Medical Assistant** coursework (Anatomy, Pharmacology, Clinical Procedures)
- **Academic tutoring** and study skills

---

## Getting Started

### Accessing Lilim

**Web Interface** (Recommended):
1. Open your browser and navigate to: `http://localhost:8080`
2. Start chatting with Lilim immediately

**Command Line**:
```bash
lilim ask "How do I fix broken APT packages?"
```

**Desktop Launcher**:
- Press `Super` key and search for "Lilim"
- Click the icon to open the web interface

---

## Using the Web Interface

### Chat Window

- **Type your question** in the input box at the bottom
- **Press Enter** or click the Send button (🔥)
- Lilim will respond with knowledge from its RAG database and/or external APIs

### Features

- **Session Memory**: Lilim remembers your conversation within a session
- **Domain Detection**: Automatically identifies whether your question is about Linux, Medical topics, or Academic subjects
- **Infernal Aesthetic**: Flame backgrounds and demonic theming for that authentic underworld experience

---

## Command Line Interface (CLI)

### Available Commands

#### Ask a Question
```bash
lilim ask "What are the normal vital signs for adults?"
# Or with full sentence:
lilim ask How do I restart a systemd service
```

#### Search Knowledge Base
```bash
lilim search "cardiovascular system"
```

#### View Session Info
```bash
lilim history
```

#### Clear Session
```bash
lilim clear
```

#### Check Server Status
```bash
lilim status
```

---

## Tools & Features

Lilim has access to powerful tools that extend its capabilities:

### Web Search
When needed, Lilim can search the internet for current information.

**Example**: *"Search the web for Ubuntu 24.04 release notes"*

### File Search
Lilim can search your local filesystem for files and content.

**Example**: *"Find all Python files in my home directory"*

### Terminal Commands
Lilim can execute safe terminal commands on your behalf.

**Example**: *"Show me running systemd services"*

> **Note**: By default, Lilim will ask for confirmation before executing commands. YOLO mode (auto-execute) can be enabled in configuration but is **not recommended** for security reasons.

---

## Personality & Interaction Style

Lilim's personality is...unique:
- **5% demonic + 5% infernal + 5% dark**: Subtle underworld theming
- **25% caring**: Genuinely wants to help you succeed
- **25% wisely experienced**: Knows their stuff
- **25% askhole**: Dry, blunt, mildly judgmental (but never hostile)
- **Sarcasm**: Used as flavor, never friction

### Communication Preferences

**For general questions**: Lilim is concise and direct

**For action items**: Lilim provides verbose, step-by-step, copy-paste-ready instructions with an "explain like I'm 10" mindset

---

## Knowledge Domains

### Linux & Ubuntu
- Package management (APT, dpkg)
- Systemd service management
- Networking and firewall configuration
- File permissions and user management
- Troubleshooting common errors
- Shell scripting basics

### Medical Assistant
- Anatomy & Physiology
- Medical terminology (prefixes, suffixes, root words)
- Vital signs and patient care
- Infection control and sterile technique
- Pharmacology basics
- Clinical procedures
- HIPAA compliance

### Academic Skills
- Study techniques and time management
- Note-taking methods
- Test preparation strategies
- Research and writing
- Critical thinking
- Test anxiety management

---

## Tips for Best Results

1. **Be specific**: "Fix APT errors" is vague. "How do I fix 'dpkg lock frontend' error?" is better

2. **Provide context**: Mention what you've already tried, error messages, system details

3. **Follow up**: Lilim remembers your conversation. You can ask follow-up questions

4. **Use proper medical terminology when needed**: Lilim knows both layman's terms and medical jargon

5. **For Linux questions**: Include the command output or error message when possible

---

## Configuration

### API Keys (Optional)

For complex queries that exceed local capabilities, Lilim can use external APIs. Edit `/etc/lilith/lilim.yaml`:

```yaml
api_keys:
  openai: "your-api-key-here"
  # or
  anthropic: "your-api-key-here"
```

### Tool Permissions

Enable/disable tools in `/etc/lilith/lilim.yaml`:

```yaml
agent:
  enable_web_search: true
  enable_file_search: true
  enable_terminal: true
  yolo_mode: false  # CAUTION: Auto-execute commands
```

---

## Troubleshooting

### Lilim isn't responding
```bash
# Check if service is running
sudo systemctl status lilith-ai

# View logs
sudo journalctl -u lilith-ai -f

# Restart service
sudo systemctl restart lilith-ai
```

### Can't access web interface
1. Verify service is running: `lilim status`
2. Check if port 8080 is available: `sudo netstat -tlnp | grep 8080`
3. Try accessing `http://127.0.0.1:8080` instead

### Responses are slow or timing out
- Check system resources: `top` or `htop`
- Verify API keys if using external services
- Check internet connection for web search features

### "Connection Error" in CLI
The Lilim server isn't running. Start it with:
```bash
sudo systemctl start lilith-ai
```

---

## Privacy & Security

- **Local-first**: RAG knowledge base runs entirely on your machine
- **Session data**: Stored in `/var/lib/lilith/memory.db`
- **API fallback**: Only used when explicitly configured
- **Terminal commands**: Require confirmation unless YOLO mode is enabled
- **No telemetry**: Lilim doesn't phone home

---

## Getting Help

- **Documentation**: `/usr/share/doc/lilith-ai/`
- **Logs**: `sudo journalctl -u lilith-ai -n 100`
- **Configuration**: `/etc/lilith/lilim.yaml`

---

## Examples

### Linux Example
```
User: How do I check which ports are listening?
Lilim: Run 'sudo netstat -tlnp' or 'sudo ss -tlnp'. This shows:
       - t: TCP ports
       - l: Listening sockets  
       - n: Numeric (don't resolve names)
       - p: Show process using the port
       
       Example output: '0.0.0.0:8080' means service listening on all interfaces.
```

### Medical Example
```
User: What's the normal blood pressure range?
Lilim: Normal blood pressure for adults:
       - Systolic: <120 mmHg
       - Diastolic: <80 mmHg
       - Written as: <120/80 mmHg
       
       Elevated: 120-129/<80
       Hypertension Stage 1: 130-139/80-89
       Hypertension Stage 2: ≥140/≥90
```

### Academic Example
```
User: Help me study for my anatomy exam
Lilim: Here's an effective study strategy:
       1. Use spaced repetition - review material at increasing intervals
       2. Create flashcards for anatomical terms
       3. Draw and label diagrams from memory
       4. Teach concepts to someone else (or explain to me!)
       5. Practice with old exams if available
       
       What specific anatomy topic are you struggling with?
```

---

**Embrace the flames. Lilim is here to guide you.**
