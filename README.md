# Jedusor

Stylus-first AI interaction layer for reMarkable tablets.

## Quick Start

### Prerequisites

- Rust 1.80+ (`rustup install stable`)
- Cross-compilation tool: `cargo install cross`
- reMarkable tablet (rM1 or rM2) connected via USB

### Setup

1. **Clone and configure:**
   ```bash
   git clone <repo-url>
   cd jedusor
   cp .env.template .env
   ```

2. **Add your Claude API key to `.env`:**
   ```bash
   ANTHROPIC_API_KEY=sk-ant-...
   ```

3. **Build for reMarkable:**
   ```bash
   cross build --release --target armv7-unknown-linux-gnueabihf
   ```

4. **Deploy to device:**
   ```bash
   # Device IP when connected via USB
   export REMARKABLE_IP=10.11.99.1

   # Copy and run
   scp target/armv7-unknown-linux-gnueabihf/release/jedusor root@$REMARKABLE_IP:
   ssh root@$REMARKABLE_IP ./jedusor
   ```

## Development

### Build Commands

```bash
# Check code on host machine (without device features)
cargo check --no-default-features

# Run tests on host machine
cargo test --no-default-features

# Build for development (slower runtime)
cross build --target armv7-unknown-linux-gnueabihf

# Build optimized release (required for acceptable performance)
cross build --release --target armv7-unknown-linux-gnueabihf

# Lint
cargo clippy --no-default-features
```

**Note:** Local development on macOS/Windows requires `--no-default-features` to exclude Linux-specific device code. Use `cross` for full builds targeting the reMarkable device.

### Project Structure

```
jedusor/
├── src/
│   ├── main.rs          # Entry point
│   ├── app.rs           # Mode switching, state
│   ├── input/           # Wacom stylus input
│   ├── stroke/          # Stroke collection
│   ├── recognition/     # Handwriting recognition
│   ├── context/         # Conversation context
│   ├── llm/             # Claude API client
│   └── render/          # E-ink rendering
├── docs/                # Documentation
├── Cargo.toml           # Dependencies
└── .env.template        # Configuration template
```

## Configuration

Environment variables (`.env` file):

- `ANTHROPIC_API_KEY` - Claude API key (required)
- `JEDUSOR_MODEL` - Claude model (default: `claude-sonnet-4-20250514`)
- `JEDUSOR_RECOGNITION` - HWR backend: `google` or `local` (default: `google`)
- `JEDUSOR_MODE` - Startup mode: `journal` or `document` (default: `journal`)

## Modes

### Journal Mode (Current)
Blank page conversation - write questions, AI responds on the page.

### Document Mode (Coming Soon)
PDF annotations with AI assistant in margins.

### Research Mode (Future)
Multi-document context and cross-references.

## Resources

- [Product Requirements](docs/PRD.md)
- [Architecture Decisions](docs/ADR/)
- [libremarkable Documentation](https://github.com/canselcik/libremarkable)
- [Claude API Documentation](https://docs.anthropic.com/)

## License

MIT
