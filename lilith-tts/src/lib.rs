//! # Lilith TTS
//!
//! Pure-Rust text-to-speech engine for Lilith Linux, powered by NeuTTS nano.
//!
//! This crate wraps the `neutts` library to provide a simple, high-level API
//! for text-to-speech synthesis using the NeuTTS nano GGUF model with
//! CPU-native inference — no Python, no ONNX Runtime, no GPU required.
//!
//! ## Quick Start
//!
//! ```no_run
//! use lilith_tts::LilithTTS;
//!
//! let tts = LilithTTS::new(None).expect("Failed to load TTS model");
//! tts.speak_to_file("Hello from the flames!", "output.wav")
//!     .expect("Failed to synthesize speech");
//! ```
//!
//! ## Voice Cloning
//!
//! ```no_run
//! use lilith_tts::LilithTTS;
//! use std::path::Path;
//!
//! let tts = LilithTTS::new(None).expect("Failed to load TTS model");
//! let ref_codes = tts.load_voice(Path::new("my_voice.npy"))
//!     .expect("Failed to load voice reference");
//! let audio = tts.synthesize_with_voice("Hello!", &ref_codes, "Reference text.")
//!     .expect("Synthesis failed");
//! tts.write_wav(&audio, Path::new("output.wav")).unwrap();
//! ```

use anyhow::{Context, Result};
use neutts::{download, NeuTTS, SAMPLE_RATE};
use std::path::{Path, PathBuf};

/// Default HuggingFace repository for the NeuTTS nano Q4 model.
pub const DEFAULT_MODEL_REPO: &str = "neuphonic/neutts-nano-q4-gguf";

/// Default language code for espeak-ng phonemization.
pub const DEFAULT_LANGUAGE: &str = "en-us";

/// Configuration for the Lilith TTS engine.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TtsConfig {
    /// HuggingFace model repo ID (e.g. "neuphonic/neutts-nano-q4-gguf").
    /// Used when downloading models via hf-hub.
    pub model_repo: String,

    /// Optional local path to backbone GGUF model file.
    /// If set, skips download and loads from disk.
    pub local_backbone_path: Option<PathBuf>,

    /// Optional local path to decoder safetensors file.
    /// If set along with `local_backbone_path`, loads both from disk.
    pub local_decoder_path: Option<PathBuf>,

    /// espeak-ng language code (e.g. "en-us", "de", "fr-fr").
    pub language: String,

    /// Optional path to a voice reference .npy file for voice cloning.
    /// If set, all synthesis uses this voice by default.
    pub default_voice_ref: Option<PathBuf>,

    /// Optional transcript of the voice reference audio.
    /// Required for voice cloning when `default_voice_ref` is set.
    pub default_voice_transcript: Option<String>,
}

impl Default for TtsConfig {
    fn default() -> Self {
        Self {
            model_repo: DEFAULT_MODEL_REPO.to_string(),
            local_backbone_path: None,
            local_decoder_path: None,
            language: DEFAULT_LANGUAGE.to_string(),
            default_voice_ref: None,
            default_voice_transcript: None,
        }
    }
}

/// High-level TTS engine wrapping NeuTTS for Lilith Linux integration.
pub struct LilithTTS {
    engine: NeuTTS,
    config: TtsConfig,
    /// Cached default voice reference codes (loaded once if configured).
    default_ref_codes: Option<Vec<i32>>,
    default_ref_transcript: String,
}

impl LilithTTS {
    /// Create a new Lilith TTS engine.
    ///
    /// If `config` is `None`, uses defaults (downloads neutts-nano-q4 from HuggingFace).
    ///
    /// ## System Requirements
    /// - `espeak-ng` must be installed (`sudo apt install espeak-ng`)
    pub fn new(config: Option<TtsConfig>) -> Result<Self> {
        let config = config.unwrap_or_default();

        let engine = if let (Some(backbone), Some(decoder)) = (
            config.local_backbone_path.as_ref(),
            config.local_decoder_path.as_ref(),
        ) {
            // Load from local paths
            NeuTTS::load_with_decoder(backbone, decoder, &config.language)
                .context("Failed to load NeuTTS from local model files")?
        } else if let Some(backbone) = config.local_backbone_path.as_ref() {
            // Load backbone from local, decoder from default path
            NeuTTS::load(backbone, &config.language)
                .context("Failed to load NeuTTS from local backbone")?
        } else {
            // Download from HuggingFace hub
            download::load_from_hub(&config.model_repo)
                .context("Failed to download and load NeuTTS model from HuggingFace")?
        };

        // Load default voice reference if configured
        let (default_ref_codes, default_ref_transcript) =
            if let Some(ref voice_path) = config.default_voice_ref {
                let codes = engine
                    .load_ref_codes(voice_path)
                    .context("Failed to load default voice reference codes")?;
                let transcript = config
                    .default_voice_transcript
                    .clone()
                    .unwrap_or_default();
                (Some(codes), transcript)
            } else {
                (None, String::new())
            };

        Ok(Self {
            engine,
            config,
            default_ref_codes,
            default_ref_transcript,
        })
    }

    /// Synthesize text to f32 audio samples using the default voice.
    ///
    /// If no default voice is configured, uses the model's built-in default.
    pub fn synthesize(&self, text: &str) -> Result<Vec<f32>> {
        match &self.default_ref_codes {
            Some(ref_codes) => self
                .engine
                .infer(text, ref_codes, &self.default_ref_transcript)
                .context("TTS inference failed"),
            None => {
                // Use empty ref codes for default voice
                let empty_codes: Vec<i32> = Vec::new();
                self.engine
                    .infer(text, &empty_codes, "")
                    .context("TTS inference failed (no voice reference)")
            }
        }
    }

    /// Synthesize text using a specific voice reference.
    pub fn synthesize_with_voice(
        &self,
        text: &str,
        ref_codes: &[i32],
        ref_transcript: &str,
    ) -> Result<Vec<f32>> {
        self.engine
            .infer(text, ref_codes, ref_transcript)
            .context("TTS inference with custom voice failed")
    }

    /// Load voice reference codes from a .npy file.
    pub fn load_voice(&self, npy_path: &Path) -> Result<Vec<i32>> {
        self.engine
            .load_ref_codes(npy_path)
            .context("Failed to load voice reference codes")
    }

    /// Synthesize text and write directly to a WAV file.
    pub fn speak_to_file(&self, text: &str, output_path: &str) -> Result<()> {
        let audio = self.synthesize(text)?;
        self.engine
            .write_wav(&audio, Path::new(output_path))
            .context("Failed to write WAV file")
    }

    /// Synthesize text and return WAV bytes (in-memory).
    pub fn speak_to_bytes(&self, text: &str) -> Result<Vec<u8>> {
        let audio = self.synthesize(text)?;
        Ok(self.engine.to_wav_bytes(&audio))
    }

    /// Write raw audio samples to a WAV file.
    pub fn write_wav(&self, audio: &[f32], output_path: &Path) -> Result<()> {
        self.engine
            .write_wav(audio, output_path)
            .context("Failed to write WAV file")
    }

    /// Get the audio sample rate (24000 Hz for NeuTTS).
    pub fn sample_rate(&self) -> u32 {
        SAMPLE_RATE
    }

    /// Get the current configuration.
    pub fn config(&self) -> &TtsConfig {
        &self.config
    }
}
