# Contributing to Lilim

Thank you for your interest in contributing to Lilim! 🔥

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR-USERNAME/Lilim.git`
3. Create a feature branch: `git checkout -b feature/my-feature`
4. Make your changes
5. Test locally
6. Push and open a Pull Request

## Development Setup

```bash
# Clone
git clone https://github.com/BlancoBAM/Lilim.git
cd Lilim

# Desktop UI (Tauri + React)
cd lilim_desktop
npm install
npm run tauri dev

# Open Interpreter backend
cd ../Lilim-v2
pip install -e .
interpreter --profile lilim.py
```

## Code Style

- **Rust**: Follow `rustfmt` defaults
- **TypeScript/React**: ESLint + Prettier
- **Python**: PEP 8

## Reporting Issues

Please include:
- OS version
- Steps to reproduce
- Expected vs actual behavior
- Relevant logs (`journalctl -u lilith-ai`)

## License

By contributing, you agree that your contributions will be licensed under the AGPL-3.0 license.
