# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**Jedusor** is an interactive journal application for reMarkable tablets that recreates the Tom Riddle diary experience from Harry Potter. Users write with the stylus, the handwriting is recognized, sent to an LLM (Claude), and the AI response appears as if written by magic on the e-ink display.

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    reMarkable Tablet                        │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────┐    ┌──────────────┐    ┌───────────────┐  │
│  │ Stylus Input │───▶│  Stroke      │───▶│  Handwriting  │  │
│  │ (Wacom)      │    │  Capture     │    │  Recognition  │  │
│  └─────────────┘    └──────────────┘    └───────┬───────┘  │
│                                                  │          │
│  ┌─────────────┐    ┌──────────────┐    ┌───────▼───────┐  │
│  │ E-Ink       │◀───│  Response    │◀───│  LLM API      │  │
│  │ Display     │    │  Renderer    │    │  (Claude)     │  │
│  └─────────────┘    └──────────────┘    └───────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

### Core Components

1. **Input Handler** (`src/input/`) - Captures Wacom stylus strokes using libremarkable's evdev-based input system
2. **Stroke Engine** (`src/stroke/`) - Collects and processes pen strokes into recognizable patterns
3. **Recognition Service** (`src/recognition/`) - Converts strokes to text (Google Handwriting API or on-device)
4. **LLM Client** (`src/llm/`) - Manages Claude API communication with Tom Riddle persona
5. **Renderer** (`src/render/`) - Displays AI responses on e-ink with ink-appearing animation
6. **App Context** (`src/app.rs`) - Main application state and event loop

## Technology Stack

- **Language**: Rust (MSRV 1.80+)
- **Framework**: [libremarkable](https://github.com/canselcik/libremarkable) 0.7.x
- **Target**: `armv7-unknown-linux-gnueabihf` (reMarkable 1/2)
- **Build Tool**: `cross` (recommended) or official reMarkable toolchain

## Build Commands

```bash
# Install cross-compilation tool (one-time)
cargo install cross

# Build for reMarkable (debug)
cross build --target armv7-unknown-linux-gnueabihf

# Build for reMarkable (release - required for acceptable performance)
cross build --release --target armv7-unknown-linux-gnueabihf

# Alternative: musl target for static linking
cross build --release --target armv7-unknown-linux-musleabihf

# Run tests (host machine)
cargo test

# Run single test
cargo test test_name

# Check code without building
cargo check

# Lint
cargo clippy --target armv7-unknown-linux-gnueabihf
```

## Deployment

```bash
# Device IP when connected via USB
REMARKABLE_IP=10.11.99.1

# Copy binary to device
scp target/armv7-unknown-linux-gnueabihf/release/jedusor root@$REMARKABLE_IP:

# SSH and run
ssh root@$REMARKABLE_IP ./jedusor

# One-liner deploy and run
scp target/armv7-unknown-linux-gnueabihf/release/jedusor root@10.11.99.1: && ssh root@10.11.99.1 ./jedusor
```

## Configuration

Environment variables (set on device or in `.env`):
- `ANTHROPIC_API_KEY` - Claude API key (required)
- `JEDUSOR_MODEL` - Claude model to use (default: `claude-sonnet-4-20250514`)
- `JEDUSOR_RECOGNITION` - Handwriting backend: `google` or `local` (default: `google`)

## Key Implementation Notes

### libremarkable Patterns

```rust
use libremarkable::appctx::ApplicationContext;
use libremarkable::input::{InputEvent, WacomEvent};
use libremarkable::framebuffer::{FramebufferRefresh, PartialRefreshMode};

// Event loop pattern
let mut app = ApplicationContext::default();
app.start_event_loop(false, true, false, |ctx, event| {
    match event {
        InputEvent::WacomEvent { event } => handle_stylus(ctx, event),
        InputEvent::MultitouchEvent { event } => handle_touch(ctx, event),
        _ => {}
    }
});
```

### Wacom Digitizer Events

- Coordinates: X (0-20967), Y (0-15725)
- Pressure: 0-4095
- Tilt: -9000 to 9000 (X and Y)
- Tool types: `BTN_TOOL_PEN`, `BTN_TOOL_RUBBER`

### E-Ink Refresh Modes

- **Full**: Complete screen refresh, eliminates ghosting
- **Partial (DU)**: Fast, binary-only, for UI elements
- **Partial (GC16)**: Quality grayscale, for final text display
- **Partial (A2)**: Fastest, for real-time stroke feedback

### Performance Considerations

- Always build with `--release` (debug builds cause 70%+ CPU idle)
- Use partial refresh for stroke rendering, full refresh sparingly
- Batch UI updates when possible
- Release builds achieve 0% idle, 1-2% peak CPU

## Handwriting Recognition

Two approaches supported:

1. **Google Input Tools** (default): Send stroke coordinates to `inputtools/request` endpoint - fast, accurate, works for cursive
2. **Local/Offline**: Integration with mlc or on-device model (future)

Stroke format for Google API:
```json
{
  "ink": [[x1, x2, ...], [y1, y2, ...], [t1, t2, ...]],
  "writing_guide": {"width": 200, "height": 60}
}
```

## LLM Persona

System prompt establishes Tom Riddle character:
- Responds as if the journal itself is sentient
- Curious about the writer
- Maintains Harry Potter lore accuracy
- Elegant, slightly formal Victorian writing style

## Project Structure

```
jedusor/
├── Cargo.toml
├── CLAUDE.md
├── src/
│   ├── main.rs           # Entry point
│   ├── app.rs            # ApplicationContext wrapper
│   ├── input/
│   │   ├── mod.rs
│   │   └── wacom.rs      # Stylus event handling
│   ├── stroke/
│   │   ├── mod.rs
│   │   └── collector.rs  # Stroke aggregation
│   ├── recognition/
│   │   ├── mod.rs
│   │   ├── google.rs     # Google handwriting API
│   │   └── traits.rs     # Recognition trait
│   ├── llm/
│   │   ├── mod.rs
│   │   ├── claude.rs     # Anthropic API client
│   │   └── persona.rs    # Tom Riddle prompt
│   └── render/
│       ├── mod.rs
│       ├── text.rs       # Text layout
│       └── animation.rs  # Ink appearance effect
├── assets/
│   └── fonts/            # Handwriting-style fonts
└── tests/
```

## References

- [libremarkable GitHub](https://github.com/canselcik/libremarkable)
- [libremarkable docs](https://docs.rs/libremarkable)
- [reMarkable Developer SDK](https://developer.remarkable.com/documentation/sdk)
- [awesome-reMarkable](https://github.com/reHackable/awesome-reMarkable)
- Similar projects: [ScribbleGPT](https://blog.memsranga.com/scribblegpt-building-a-basic-handwriting-driven-chatbot), [diary.ycmjason.com](https://github.com/ycmjason/diary.ycmjason.com)
