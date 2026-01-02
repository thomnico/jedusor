# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**Jedusor** is a stylus-first AI interaction layer for reMarkable tablets. Write questions, annotations, or commands by hand—the AI responds directly on the e-ink display.

Three interaction modes:

1. **Journal Mode** - Blank page conversation (magical diary experience)
2. **Document Mode** - AI assistant overlaid on PDFs, responds to margin annotations
3. **Research Mode** - Multi-document context, cross-reference questions (future)

## Architecture

```
┌───────────────────────────────────────────────────────────────────┐
│                       reMarkable Tablet                           │
├───────────────────────────────────────────────────────────────────┤
│                                                                   │
│  ┌─────────────┐    ┌──────────────┐    ┌───────────────┐        │
│  │ Stylus Input │───▶│  Stroke      │───▶│  Handwriting  │        │
│  │ (Wacom)      │    │  Capture     │    │  Recognition  │        │
│  └─────────────┘    └──────────────┘    └───────┬───────┘        │
│                                                  │                │
│  ┌─────────────┐                        ┌───────▼───────┐        │
│  │ PDF Module  │───────────────────────▶│  Context      │        │
│  │ (lopdf +    │  (page text, regions)  │  Manager      │        │
│  │  mupdf)     │                        └───────┬───────┘        │
│  └─────────────┘                                │                │
│                                                  │                │
│  ┌─────────────┐    ┌──────────────┐    ┌───────▼───────┐        │
│  │ E-Ink       │◀───│  Response    │◀───│  LLM API      │        │
│  │ Display     │    │  Renderer    │    │  (Claude)     │        │
│  └─────────────┘    └──────────────┘    └───────────────┘        │
│                                                                   │
└───────────────────────────────────────────────────────────────────┘
```

### Core Components

1. **Input Handler** (`src/input/`) - Wacom stylus strokes + gesture detection (circle, underline, lasso)
2. **Stroke Engine** (`src/stroke/`) - Stroke collection, gesture recognition
3. **Recognition Service** (`src/recognition/`) - Handwriting to text (Google Input Tools API)
4. **PDF Module** (`src/pdf/`) - Document loading, rendering, text extraction (lopdf + mupdf)
5. **Context Manager** (`src/context/`) - Combines annotations + document content for LLM
6. **LLM Client** (`src/llm/`) - Claude API with persona management
7. **Renderer** (`src/render/`) - E-ink optimized display with streaming animation
8. **App Context** (`src/app.rs`) - Mode switching, state, event loop

## Technology Stack

- **Language**: Rust (MSRV 1.80+)
- **Framework**: [libremarkable](https://github.com/canselcik/libremarkable) 0.7.x
- **PDF**: lopdf (text extraction) + mupdf (rendering)
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
- `JEDUSOR_MODEL` - Claude model (default: `claude-sonnet-4-20250514`)
- `JEDUSOR_RECOGNITION` - HWR backend: `google` or `local` (default: `google`)
- `JEDUSOR_MODE` - Startup mode: `journal` or `document` (default: `journal`)

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

| Mode | Use Case | Speed |
|------|----------|-------|
| Full (GC16) | Page clear, final render | ~450ms |
| Partial (DU) | UI elements, buttons | ~120ms |
| Partial (GC16) | Text display | ~260ms |
| Partial (A2) | Real-time strokes | ~50ms |

### Performance Considerations

- Always build with `--release` (debug builds cause 70%+ CPU idle)
- Use partial refresh for stroke rendering, full refresh sparingly
- Batch UI updates when possible
- Release builds achieve 0% idle, 1-2% peak CPU

### PDF Context Strategy

For large documents, use windowed context:

```rust
enum ContextStrategy {
    CurrentPage,                           // Only current page
    Window { before: usize, after: usize }, // Adjacent pages
    AnnotatedPages,                        // Pages with user annotations
    SemanticSearch { query: String },      // Relevant sections
}
```

## Handwriting Recognition

**Google Input Tools API** (default):

- Send stroke coordinates to `inputtools/request` endpoint
- Fast, accurate, works for cursive
- Requires network

Stroke format:

```json
{
  "ink": [[x1, x2, ...], [y1, y2, ...], [t1, t2, ...]],
  "writing_guide": {"width": 200, "height": 60}
}
```

## Project Structure

```
jedusor/
├── Cargo.toml
├── CLAUDE.md
├── docs/
│   ├── PRD.md                # Product requirements
│   └── ADR/                  # Architecture decisions
│       ├── 000-index.md
│       ├── 001-programming-language.md
│       ├── 002-handwriting-recognition.md
│       ├── 003-llm-provider.md
│       ├── 004-device-framework.md
│       ├── 005-response-animation.md
│       ├── 006-existing-projects-analysis.md
│       └── 007-pdf-integration.md
├── src/
│   ├── main.rs               # Entry point
│   ├── app.rs                # Mode switching, state
│   ├── input/
│   │   ├── mod.rs
│   │   ├── wacom.rs          # Stylus events
│   │   └── gesture.rs        # Circle, underline detection
│   ├── stroke/
│   │   ├── mod.rs
│   │   └── collector.rs      # Stroke aggregation
│   ├── recognition/
│   │   ├── mod.rs
│   │   ├── google.rs         # Google Input Tools
│   │   └── traits.rs         # Recognition trait
│   ├── pdf/
│   │   ├── mod.rs
│   │   ├── loader.rs         # PDF loading (lopdf)
│   │   ├── renderer.rs       # Page rendering (mupdf)
│   │   └── extractor.rs      # Text + position extraction
│   ├── context/
│   │   ├── mod.rs
│   │   └── manager.rs        # LLM context building
│   ├── llm/
│   │   ├── mod.rs
│   │   ├── claude.rs         # Anthropic API
│   │   └── persona.rs        # System prompts
│   └── render/
│       ├── mod.rs
│       ├── text.rs           # Text layout
│       ├── animation.rs      # Streaming reveal
│       └── zones.rs          # Response areas (margin/footer)
├── assets/
│   └── fonts/                # Typography
└── tests/
```

## References

- [libremarkable](https://github.com/canselcik/libremarkable) - Device framework
- [lopdf](https://github.com/J-F-Liu/lopdf) - PDF text extraction
- [mupdf](https://mupdf.com/) - PDF rendering
- [reMarkable Developer SDK](https://developer.remarkable.com/documentation/sdk)
- [awesome-reMarkable](https://github.com/reHackable/awesome-reMarkable)
- Similar: [reMarkableAI](https://github.com/nickian/reMarkableAI), [armrest](https://github.com/bkirwi/armrest)
