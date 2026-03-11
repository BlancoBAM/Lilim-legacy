"""
Lilim Memory Manager — Rowboat-Inspired Persistent Knowledge Graph

Maintains a Markdown vault of facts, decisions, preferences, and conversation
summaries. Loaded before each conversation for persistent context. Updated
after each conversation with extracted knowledge.

Vault layout:
    ~/.local/share/lilim/memory/
    ├── people/
    │   └── user.md           # User profile, preferences, learned facts
    ├── facts/
    │   └── *.md              # Extracted facts (system info, preferences)
    ├── decisions/
    │   └── *.md              # Key decisions the user made
    ├── sessions/
    │   └── YYYY-MM-DD_HH-MM.md  # Conversation summaries
    └── index.md              # Master index with backlinks
"""

import datetime
import json
import os
import re
from pathlib import Path
from typing import Optional


# ── Default vault path ────────────────────────────────────
DEFAULT_VAULT = os.path.expanduser("~/.local/share/lilim/memory")


class MemoryManager:
    """Persistent knowledge graph stored as Markdown files."""

    def __init__(self, vault_path: Optional[str] = None):
        self.vault = Path(vault_path or DEFAULT_VAULT)
        self._ensure_vault()

    # ── Vault initialization ──────────────────────────────

    def _ensure_vault(self):
        """Create vault directories if they don't exist."""
        for subdir in ["people", "facts", "decisions", "sessions"]:
            (self.vault / subdir).mkdir(parents=True, exist_ok=True)

        # Create user profile if missing
        user_profile = self.vault / "people" / "user.md"
        if not user_profile.exists():
            user_profile.write_text(
                "# User Profile\n\n"
                "## Preferences\n\n"
                "- *(Lilim will learn your preferences over time)*\n\n"
                "## Known Facts\n\n"
                "- Running Lilith Linux\n"
            )

        # Create index if missing
        index = self.vault / "index.md"
        if not index.exists():
            index.write_text(
                "# Lilim Memory Index\n\n"
                "This vault contains Lilim's persistent memory.\n\n"
                "## Sections\n"
                "- [[people/]] — People and user profiles\n"
                "- [[facts/]] — Learned facts\n"
                "- [[decisions/]] — Key decisions\n"
                "- [[sessions/]] — Conversation summaries\n"
            )

    # ── Load context for a conversation ───────────────────

    def load_context(self, query: str = "", max_notes: int = 5, max_chars: int = 2000) -> str:
        """Load relevant memory context for the current conversation.

        Searches the vault for notes related to the query and returns
        a formatted context block to inject into the system message.

        Args:
            query: The user's current message (for relevance matching)
            max_notes: Maximum number of notes to include
            max_chars: Maximum total characters of context

        Returns:
            Formatted context string for system message injection
        """
        relevant = []

        # Always include user profile
        user_profile = self.vault / "people" / "user.md"
        if user_profile.exists():
            content = user_profile.read_text()
            relevant.append(("User Profile", content))

        # Search facts and decisions by keyword relevance
        if query:
            keywords = self._extract_keywords(query)
            for subdir in ["facts", "decisions"]:
                dir_path = self.vault / subdir
                if dir_path.exists():
                    for note in sorted(dir_path.glob("*.md"), key=lambda p: p.stat().st_mtime, reverse=True):
                        content = note.read_text()
                        score = self._relevance_score(content, keywords)
                        if score > 0:
                            relevant.append((note.stem, content, score))

        # Sort by relevance (after user profile which has no score)
        scored = [(name, content) for name, content, *score in relevant]
        scored.sort(key=lambda x: x[1] if len(x) > 2 else 0, reverse=True)

        # Recent sessions (last 3)
        sessions_dir = self.vault / "sessions"
        if sessions_dir.exists():
            recent_sessions = sorted(sessions_dir.glob("*.md"), key=lambda p: p.stat().st_mtime, reverse=True)[:3]
            for session in recent_sessions:
                content = session.read_text()
                scored.append((f"Session: {session.stem}", content))

        # Build context block
        if not scored:
            return ""

        context_parts = []
        total_chars = 0

        for name, content in scored[:max_notes]:
            # Truncate long notes
            if total_chars + len(content) > max_chars:
                remaining = max_chars - total_chars
                if remaining > 100:
                    content = content[:remaining] + "..."
                else:
                    break

            context_parts.append(f"### {name}\n{content.strip()}")
            total_chars += len(content)

        if not context_parts:
            return ""

        return (
            "\n## Your Memory (persistent context from past conversations)\n"
            + "\n\n".join(context_parts)
            + "\n"
        )

    # ── Save knowledge after a conversation ───────────────

    def save_session_summary(self, messages: list[dict], summary: str = ""):
        """Save a conversation summary to the sessions directory.

        Args:
            messages: List of conversation messages
            summary: Optional pre-generated summary
        """
        timestamp = datetime.datetime.now().strftime("%Y-%m-%d_%H-%M")
        session_file = self.vault / "sessions" / f"{timestamp}.md"

        if not summary:
            # Generate a basic summary from messages
            summary = self._generate_basic_summary(messages)

        content = (
            f"# Session — {timestamp}\n\n"
            f"{summary}\n\n"
            f"## Messages ({len(messages)} total)\n"
        )

        # Include last few messages as context
        for msg in messages[-6:]:
            role = msg.get("role", "unknown")
            text = msg.get("content", "")[:200]
            content += f"- **{role}**: {text}\n"

        session_file.write_text(content)

    def save_fact(self, fact: str, category: str = "general"):
        """Save a learned fact to the vault.

        Args:
            fact: The fact to save
            category: Category name for the fact file
        """
        fact_file = self.vault / "facts" / f"{category}.md"

        if fact_file.exists():
            existing = fact_file.read_text()
            # Don't duplicate
            if fact in existing:
                return
            content = existing.rstrip() + f"\n- {fact}\n"
        else:
            content = f"# Facts: {category.title()}\n\n- {fact}\n"

        fact_file.write_text(content)

    def save_decision(self, decision: str, context: str = ""):
        """Save a user decision to the vault."""
        timestamp = datetime.datetime.now().strftime("%Y-%m-%d_%H-%M")
        slug = re.sub(r'[^a-z0-9]+', '-', decision.lower()[:50]).strip('-')
        dec_file = self.vault / "decisions" / f"{timestamp}_{slug}.md"

        content = (
            f"# Decision: {decision[:100]}\n\n"
            f"**Date**: {timestamp}\n\n"
        )
        if context:
            content += f"**Context**: {context}\n\n"
        content += f"**Decision**: {decision}\n"

        dec_file.write_text(content)

    def update_user_profile(self, key: str, value: str):
        """Update a fact in the user profile."""
        profile = self.vault / "people" / "user.md"
        content = profile.read_text() if profile.exists() else "# User Profile\n\n"

        # Append under Known Facts
        if "## Known Facts" in content:
            content = content.rstrip() + f"\n- {key}: {value}\n"
        else:
            content += f"\n## Known Facts\n\n- {key}: {value}\n"

        profile.write_text(content)

    # ── Extract knowledge from conversation ───────────────

    def extract_and_save(self, messages: list[dict], llm_fn=None):
        """Extract knowledge from a conversation and save to vault.

        Args:
            messages: Conversation messages
            llm_fn: Optional function to call an LLM for extraction.
                     Signature: llm_fn(prompt: str) -> str
        """
        if llm_fn:
            # Use LLM to extract structured knowledge
            conversation_text = "\n".join(
                f"{m.get('role', 'unknown')}: {m.get('content', '')}"
                for m in messages[-10:]
            )

            extraction_prompt = f"""Analyze the following conversation and extract key information.
Return a JSON object with these keys (omit any that have no data):

{{
  "facts": ["list of facts learned about the user or system"],
  "decisions": ["list of decisions the user made"],
  "preferences": ["list of user preferences discovered"],
  "summary": "One-sentence summary of what was discussed"
}}

Conversation:
{conversation_text}

JSON:"""

            try:
                result = llm_fn(extraction_prompt)
                # Parse JSON from response
                json_match = re.search(r'\{[^{}]*\}', result, re.DOTALL)
                if json_match:
                    data = json.loads(json_match.group())

                    for fact in data.get("facts", []):
                        self.save_fact(fact)

                    for decision in data.get("decisions", []):
                        self.save_decision(decision)

                    for pref in data.get("preferences", []):
                        self.update_user_profile("Preference", pref)

                    summary = data.get("summary", "")
                    self.save_session_summary(messages, summary)
                    return
            except (json.JSONDecodeError, Exception):
                pass

        # Fallback: basic summary without LLM
        self.save_session_summary(messages)

    # ── Internal helpers ──────────────────────────────────

    def _extract_keywords(self, text: str) -> list[str]:
        """Extract keywords from text for search."""
        # Remove common stop words and extract meaningful terms
        stop_words = {
            "the", "a", "an", "is", "are", "was", "were", "be", "been",
            "being", "have", "has", "had", "do", "does", "did", "will",
            "would", "could", "should", "may", "might", "can", "shall",
            "to", "of", "in", "for", "on", "with", "at", "by", "from",
            "it", "this", "that", "its", "my", "your", "i", "me", "you",
            "what", "how", "when", "where", "why", "who", "which",
            "and", "or", "but", "not", "if", "then", "so", "as",
        }
        words = re.findall(r'\b[a-zA-Z]{3,}\b', text.lower())
        return [w for w in words if w not in stop_words]

    def _relevance_score(self, content: str, keywords: list[str]) -> int:
        """Score a note's relevance to the given keywords."""
        content_lower = content.lower()
        return sum(1 for kw in keywords if kw in content_lower)

    def _generate_basic_summary(self, messages: list[dict]) -> str:
        """Generate a basic summary without an LLM."""
        user_msgs = [m.get("content", "") for m in messages if m.get("role") == "user"]
        if not user_msgs:
            return "Empty conversation"

        topics = ", ".join(msg[:60] for msg in user_msgs[:3])
        return f"Discussed: {topics}"
