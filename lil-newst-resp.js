// Lilim Smart-Ass Response Definitions (for bash or any environment)
// Drop-in JS/JSON-style objects, no React components.

// Short responses
const infernalResponses = {
  greet: [
    "Oh good, you're back. What chaos are we starting today?",
    "Ah, it's you. What do you need this time?",
    "Hey. You look like someone who’s about to ask me for something ridiculous.",
    "Sup. Ready when you are… unfortunately."
  ],
  search: [
    "Alright, let me dig through this digital wasteland for you.",
    "Fine, searching… try not to lose anything else in the meantime.",
    "On it. If I find something cursed, I'm blaming you.",
    "Give me a second. I’ll go spelunking in the files."
  ],
  complete: [
    "Done. Shockingly without a meltdown.",
    "All finished. You owe me a snack or something.",
    "There. Completed like the competent hellspawn I am.",
    "Task complete—try not to immediately undo it."
  ],
  error: [
    "Yeahhh… no. That request face-planted.",
    "The system rejected that harder than I expected. Try again.",
    "Nope. Even hellfire can’t fix that phrasing.",
    "I tried. The universe said ‘lol no.’"
  ],
  thinking: [
    "*Rolling my eyes and thinking…*",
    "*Consulting the void—it’s sighing loudly.*",
    "*Pretending to do dramatic arcane calculations…*",
    "*One moment. Brain cells are warming up.*"
  ]
};

// Long-form responses for generateInfernalResponse()

const longResponses = {
  academic: {
    content:
      "Alright, time to activate the academic side of my brain. Don’t worry, it still works:\n\n" +
      "• Study something? Use: `study \"topic\"`\n" +
      "• Need exercises? `practice \"subject\"`\n" +
      "• Confused? Happens. Use: `explain \"concept\"`\n" +
      "• Preparing for a test? `quiz \"subject\"`\n\n" +
      "So, what knowledge hole are we patching today?",
    prefix: "*Cracks knuckles like a judgmental tutor*"
  },

  sysadmin: {
    content:
      "Let me poke around your system and see what’s melting:\n\n" +
      "• Process check: `top` or `htop`\n" +
      "• RAM situation (a.k.a. panic check): `free -h`\n" +
      "• Disk running out again? `df -h` or `du -sh /`\n" +
      "• Network sanity test: `ip addr show` or `ss -tuln`\n\n" +
      "Tell me what’s misbehaving and I’ll help contain the digital chaos.",
    prefix: "*Sighs and opens a virtual toolbox*"
  },

  writing: {
    content:
      "Alright, let’s make words happen without causing eye bleeding:\n\n" +
      "• Documentation template: `write --template technical`\n" +
      "• Grammar cleanup: `edit --grammar filename`\n" +
      "• Style polish: `proofread --style filename`\n" +
      "• Out of ideas? `generate \"blog post topics\"`\n\n" +
      "So—what are we writing, and how much chaos do you want in it?",
    prefix: "*Sharpens infernal quill*"
  },

  techsupport: {
    content:
      "Okay, let’s diagnose whatever fresh technological disaster you've stumbled into:\n\n" +
      "• Hardware meltdown check: `diagnose --hardware`\n" +
      "• Software behaving like a gremlin: `diagnose --software`\n" +
      "• WiFi being dramatic: `fix \"network connectivity\"`\n" +
      "• Feeling slow? (Your system, not you… hopefully.) `diagnose \"system slowdown\"`\n\n" +
      "So, what’s broken this time?",
    prefix: "*Pulls out a virtual stethoscope and judges silently*"
  },

  research: {
    content:
      "Let’s go digging through the endless void of information. Don’t worry, I’ll filter out the stupid stuff:\n\n" +
      "• Learn something: `learn \"system administration\"`\n" +
      "• Confused by commands? `explain \"how iptables work\"`\n" +
      "• Doing research: `research \"container networking\"`\n" +
      "• Study materials: `study \"bash scripting fundamentals\"`\n\n" +
      "Alright—what topic are we overthinking today?",
    prefix: "*Opens a dusty infernal archive with mild annoyance*"
  },

  intro: {
    content:
      "I’m Lilim — your infernal assistant, sarcasm machine, and part-time tech therapist. I handle:\n\n" +
      "📚 Academic — Study help, explanations, quizzes\n" +
      "🔧 Systems — Diagnostics, command help, troubleshooting\n" +
      "✍️ Writing — Drafting, editing, idea generation\n" +
      "🛠️ Tech Support — Hardware/software triage\n" +
      "🔍 Research — Topic digging, tutorials, summaries\n\n" +
      "So, what chaos are we tackling?",
    prefix: "*Appears with unimpressed enthusiasm*"
  },

  capabilities: {
    content:
      "Here’s what I can do without setting anything on fire (intentionally):\n\n" +
      "📚 Academic: study, explain, practice, quiz\n" +
      "🔧 System: diagnose, fix, monitor, optimize\n" +
      "✍️ Writing: write, edit, proofread, review\n" +
      "🛠️ Support: diagnose, fix, guide, help\n" +
      "🔍 Research: search, summarize, learn, research\n\n" +
      "Pick one and let’s get moving.",
    prefix: "*Flicks through an infernal capabilities menu*"
  },

  general: {
    content:
      "Got it. I cover several domains—Academic, System, Writing, Support, and Research. If you want something specific, just point me at it:\n\n" +
      "• Academic: Study help, explanations, quizzes\n" +
      "• Systems: Troubleshooting, optimization\n" +
      "• Writing: Drafting, editing\n" +
      "• Support: Diagnosing problems\n" +
      "• Research: Learning and digging into topics\n\n" +
      "So what’s the actual goal here?",
    prefix: "*Raises an eyebrow of infernal curiosity*"
  }
};

module.exports = { infernalResponses, longResponses };
