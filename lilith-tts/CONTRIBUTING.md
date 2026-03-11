# Contributing to Lilith TTS

Thank you for your interest in contributing! 🔊

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR-USERNAME/Lilith-TTS.git`
3. Create a feature branch: `git checkout -b feature/my-feature`
4. Make your changes
5. Run `cargo check` and `cargo test`
6. Push and open a Pull Request

## System Dependencies

```bash
sudo apt install cmake espeak-ng alsa-utils
```

## Building

```bash
cargo build --release
```

## Code Style

- Follow `rustfmt` defaults
- Run `cargo clippy` before submitting

## License

By contributing, you agree that your contributions will be licensed under the AGPL-3.0 license.
