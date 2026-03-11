use anyhow::Result;
use clap::{Parser, Subcommand};
use lilith_tts::{LilithTTS, TtsConfig};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "lilith-tts")]
#[command(about = "🔥 Lilith TTS — Voice synthesis for Lilith Linux")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Path to backbone GGUF model file (skips download if set)
    #[arg(long, global = true)]
    model: Option<PathBuf>,

    /// Path to decoder safetensors file
    #[arg(long, global = true)]
    decoder: Option<PathBuf>,

    /// espeak-ng language code
    #[arg(long, global = true, default_value = "en-us")]
    language: String,

    /// Path to voice reference .npy file for voice cloning
    #[arg(long, global = true)]
    voice: Option<PathBuf>,

    /// Transcript of the voice reference audio
    #[arg(long, global = true)]
    voice_transcript: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Synthesize text to a WAV file
    Speak {
        /// Text to synthesize
        text: String,

        /// Output WAV file path
        #[arg(short, long, default_value = "output.wav")]
        output: String,
    },

    /// Download the TTS model from HuggingFace
    Download {
        /// HuggingFace model repo ID
        #[arg(default_value = "neuphonic/neutts-nano-q4-gguf")]
        model_repo: String,
    },

    /// Show model information
    Info,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Speak { text, output } => {
            let config = TtsConfig {
                local_backbone_path: cli.model,
                local_decoder_path: cli.decoder,
                language: cli.language,
                default_voice_ref: cli.voice,
                default_voice_transcript: cli.voice_transcript,
                ..Default::default()
            };

            eprintln!("🔥 Loading Lilith TTS engine...");
            let tts = LilithTTS::new(Some(config))?;

            eprintln!("🗣️  Synthesizing: \"{}\"", text);
            tts.speak_to_file(&text, &output)?;

            eprintln!("✓ Audio saved to: {}", output);
            eprintln!("  Sample rate: {} Hz", tts.sample_rate());
        }

        Commands::Download { model_repo } => {
            eprintln!("🔥 Downloading model from: {}", model_repo);
            let config = TtsConfig {
                model_repo,
                ..Default::default()
            };
            let _tts = LilithTTS::new(Some(config))?;
            eprintln!("✓ Model downloaded and cached successfully");
        }

        Commands::Info => {
            eprintln!("🔥 Lilith TTS — Voice Synthesis Engine");
            eprintln!();
            eprintln!("  Model:       NeuTTS nano (Q4 GGUF)");
            eprintln!("  Backend:     llama-cpp-2 (GGUF backbone)");
            eprintln!("  Codec:       NeuCodec (pure-Rust decoder)");
            eprintln!("  Sample Rate: 24000 Hz");
            eprintln!("  Channels:    Mono");
            eprintln!("  Bit Depth:   16-bit PCM");
            eprintln!();
            eprintln!("  System Deps: espeak-ng");
            eprintln!("  Install:     sudo apt install espeak-ng");
        }
    }

    Ok(())
}
