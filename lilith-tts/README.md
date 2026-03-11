<p align="center">
  <img src="assets/tts-icon.png" alt="Lilith TTS" width="100" />
</p>

<h1 align="center">Lilith TTS</h1>

<p align="center">
  <strong>Pure-Rust Text-to-Speech for Lilith Linux</strong><br/>
  Powered by NeuTTS Nano · CPU inference · Voice cloning
</p>

<p align="center">
  <a href="#features">Features</a> •
  <a href="#installation">Installation</a> •
  <a href="#usage">Usage</a> •
  <a href="#library-api">Library API</a> •
  <a href="#voice-management">Voice Management</a> •
  <a href="#contributing">Contributing</a>
</p>

---

## Features

🔊 **Pure-Rust Inference** — No Python runtime needed, single binary

🎙️ **Voice Cloning** — Clone any voice from a short audio sample

⚡ **CPU-Only** — Runs on any hardware, no GPU required

🎛️ **Panel Integration** — System tray icon with voice selection, speed control, and voice creation

⌨️ **Hotkey Read-Aloud** — `Ctrl+Shift+T` reads highlighted text or clipboard contents

📦 **Standalone** — Can be used independently or as part of the Lilim ecosystem

## System Requirements

- Linux (x86_64 or ARM64)
- `espeak-ng` (phonemization)
- `cmake` (build only)
- `aplay` (playback, part of `alsa-utils`)

## Installation

### From Source

```bash
# Install system dependencies
sudo apt install cmake espeak-ng alsa-utils

# Clone and build
git clone https://github.com/BlancoBAM/Lilith-TTS.git
cd Lilith-TTS
cargo build --release

# Install binary
sudo cp target/release/lilith-tts /usr/bin/
```

### NeuTTS Model

The `neutts-nano` model is automatically downloaded from HuggingFace on first use. To pre-download:

```bash
lilith-tts download
```

## Usage

### CLI

```bash
# Speak text to speakers
lilith-tts speak "Hello from the underworld"

# Save to file
lilith-tts speak "Save this" --output output.wav

# Use a custom voice
lilith-tts speak "Cloned voice" --voice /path/to/voice.npy

# Show model info
lilith-tts info

# Download model
lilith-tts download
```

### Read-Aloud Hotkey

While the Lilim desktop app or TTS panel is running:

1. **Highlight text** anywhere on screen (or copy to clipboard)
2. Press **`Ctrl+Shift+T`**
3. The text is read aloud through your speakers

### Panel Icon

The TTS panel runs as a system tray icon with:

| Feature | Description |
|---------|-------------|
| **Voice Selector** | Switch between saved voices (radio buttons) |
| **Speed Control** | 0.5× to 2.0× playback speed |
| **Create Voice** | Clone a voice from an audio sample |
| **Preview** | Test a voice before saving |

```bash
# Start the panel
lilith-tts-panel
```

## Library API

Use Lilith TTS as a Rust library:

```rust
use lilith_tts::LilithTTS;

// Initialize (downloads model if needed)
let tts = LilithTTS::new(None)?;

// Synthesize to file
tts.speak_to_file("Hello, world!", "output.wav")?;

// Synthesize to bytes (in-memory WAV)
let wav_bytes = tts.speak_to_bytes("In-memory audio")?;

// Voice cloning
let audio = tts.synthesize_with_voice("Cloned speech", "reference.npy")?;

// Get sample rate
println!("Sample rate: {}", tts.sample_rate());
```

## Voice Management

### Pre-loaded Voices

Two standard American English voices are included by default.

### Creating a Custom Voice

1. Click the TTS panel icon → **"Create a new voice"**
2. Click **"Voice Sample"** → browse and select an audio file (.wav, .mp3, .ogg, .flac)
3. Wait for encoding (progress bar shows status)
4. Click **"Preview"** to test the voice
5. Enter a name and click **"Save Voice"**

Voices are stored in `~/.local/share/lilith-tts/voices/`.

### Voice Configuration

Settings are saved to `~/.config/lilith-tts/config.json`:

```json
{
  "current_voice": "Default American (Female)",
  "speed": 1.0,
  "saved_voices": {
    "My Custom Voice": {
      "ref_codes": "/home/user/.local/share/lilith-tts/voices/sample.npy",
      "speed": 1.0
    }
  }
}
```

## Project Structure

```
Lilith-TTS/
├── src/
│   ├── lib.rs          # Core TTS engine wrapper
│   └── main.rs         # CLI binary
├── assets/             # Icons
├── scripts/
│   └── lilith-tts-panel  # GTK panel daemon (Python)
├── Cargo.toml          # Rust dependencies
└── README.md
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

AGPL-3.0 — See [LICENSE](LICENSE) for details.

## Related

- [Lilim](https://github.com/BlancoBAM/Lilim) — The full AI assistant for Lilith Linux
- [NeuTTS](https://github.com/nickovchinnikov/neucodec-tts) — The underlying TTS model
