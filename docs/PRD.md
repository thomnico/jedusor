# Product Requirements Document: Jedusor

## Overview

**Product Name**: Jedusor
**Version**: 1.0
**Date**: 2026-01-02

Jedusor is a stylus-first AI interaction layer for reMarkable tablets. Write questions, annotations, or commands by hand—the AI responds directly on the e-ink display. Works as a blank journal (magical diary mode) or overlaid on PDFs/documents for interactive reading and research.

## Problem Statement

Reading and annotating PDFs on e-ink tablets is passive. Users highlight, scribble notes, but can't ask questions about content or get explanations without switching devices. Existing AI chat interfaces require typing and break the pen-and-paper flow. There's no way to have a conversation with your documents using handwriting.

## Vision

**"Talk to your papers with a pen."**

Three interaction modes:
1. **Journal Mode** - Blank page conversation (Tom Riddle diary experience)
2. **Document Mode** - AI assistant overlaid on PDFs, responds to margin annotations
3. **Research Mode** - Multi-document context, cross-reference questions

## Target Users

| Persona | Use Case |
|---------|----------|
| Researcher | Annotate papers, ask clarifying questions in margins, get summaries |
| Student | Interactive textbook reading, explain concepts, quiz me |
| Writer | Creative brainstorming, character dialogue, world-building |
| Professional | Annotate contracts/reports, ask "what does this clause mean?" |
| Harry Potter fan | Magical diary experience |

## User Stories

### Core Experience (All Modes)

| ID | Story | Priority |
|----|-------|----------|
| US-01 | As a user, I can write with the stylus and see it recognized as text | P0 |
| US-02 | As a user, I see the AI response appear on the page | P0 |
| US-03 | As a user, the conversation context persists within a session | P0 |
| US-04 | As a user, I can switch between interaction modes | P1 |
| US-05 | As a user, I can configure the AI persona/behavior | P2 |

### Journal Mode (Blank Page)

| ID | Story | Priority |
|----|-------|----------|
| US-10 | As a user, I can have a freeform conversation on a blank page | P0 |
| US-11 | As a user, AI responses appear with a "magical writing" animation | P1 |
| US-12 | As a user, I can select persona presets (Riddle, tutor, assistant) | P2 |
| US-13 | As a user, old exchanges fade/compact to make room for new ones | P2 |

### Document Mode (PDF Interaction)

| ID | Story | Priority |
|----|-------|----------|
| US-20 | As a user, I can load a PDF and write annotations in margins | P0 |
| US-21 | As a user, I can circle/underline text and write "explain this" | P0 |
| US-22 | As a user, the AI sees the PDF content as context | P0 |
| US-23 | As a user, AI responses appear in a designated area (margin/footer) | P0 |
| US-24 | As a user, I can ask questions about specific pages or sections | P1 |
| US-25 | As a user, I can request summaries of highlighted sections | P1 |
| US-26 | As a user, annotations and AI responses are saved with the document | P2 |

### Research Mode (Multi-Document)

| ID | Story | Priority |
|----|-------|----------|
| US-30 | As a user, I can reference multiple documents in context | P2 |
| US-31 | As a user, I can ask cross-reference questions | P2 |
| US-32 | As a user, I can build a knowledge base from my annotations | P3 |

### Writing Experience

| ID | Story | Priority |
|----|-------|----------|
| US-40 | As a user, strokes render in real-time (<16ms latency) | P0 |
| US-41 | As a user, I can erase strokes with the pen's eraser end | P1 |
| US-42 | As a user, handwriting recognition works for cursive and print | P0 |
| US-43 | As a user, I can draw selection regions (circle, lasso) | P1 |

### System

| ID | Story | Priority |
|----|-------|----------|
| US-50 | As a user, the app works with WiFi connectivity | P0 |
| US-51 | As a user, I receive clear feedback when offline | P1 |
| US-52 | As a user, battery drain is acceptable (<10%/hour active) | P0 |
| US-53 | As a user, I can exit cleanly with a gesture | P0 |

## Functional Requirements

### FR-1: Stylus Input Capture

- Capture Wacom digitizer events (position, pressure, tilt)
- Render strokes at <16ms latency
- Support pen tip and eraser tool detection
- Detect stroke completion (pen lift) to trigger recognition
- Support gesture detection (circle, underline, lasso)

### FR-2: Handwriting Recognition

- Convert stroke data to text with >90% accuracy
- Support English and French (P1: more languages)
- Process recognition within 500ms
- Handle cursive, print, and mixed styles
- Recognize command keywords ("explain", "summarize", "translate")

### FR-3: PDF Integration

- Load and render PDF documents
- Extract text content for LLM context
- Detect spatial relationship between annotation and PDF content
- Support page navigation
- Preserve PDF rendering quality on e-ink

### FR-4: LLM Integration

- Send recognized text + document context to Claude API
- Support long context (200K tokens for large documents)
- Implement multiple persona modes via system prompts
- Stream responses for progressive display
- Handle API errors gracefully

### FR-5: Response Rendering

- Display AI text in configurable typography
- Journal mode: handwriting-style font with animation
- Document mode: clean sans-serif in designated zones
- Support word wrapping within bounds
- Implement appropriate e-ink refresh strategy

### FR-6: Context Management

- Maintain conversation history during session
- Include relevant document sections in context
- Implement smart context windowing for long documents
- Clear/reset conversation on user action

## Interaction Patterns

### Journal Mode
```
┌─────────────────────────────────────┐
│                                     │
│   User writes: "What is love?"      │
│                                     │
│   ─────────────────────────────     │
│                                     │
│   AI responds: "A curious           │
│   question. Love is perhaps         │
│   the most powerful magic..."       │
│                                     │
│   User writes: "Tell me more"       │
│                                     │
└─────────────────────────────────────┘
```

### Document Mode
```
┌─────────────────────────────────────┐
│  PDF Content          │  Margin     │
│  ─────────────────    │             │
│  "The transformer     │  [User      │
│  architecture uses    │  circles    │
│  (self-attention)     │  "attention"│
│  mechanisms to..."    │  writes:    │
│                       │  "explain?"]│
│                       │             │
│                       │  AI: "Self- │
│                       │  attention  │
│                       │  allows..." │
├───────────────────────┴─────────────┤
│  [Page navigation]  [Mode toggle]   │
└─────────────────────────────────────┘
```

## Non-Functional Requirements

### Performance

| Metric | Target |
|--------|--------|
| Stroke rendering latency | <16ms |
| Recognition latency | <500ms |
| PDF page render | <1s |
| LLM response start | <2s (network) |
| CPU usage (idle) | <1% |
| CPU usage (active) | <10% |
| Memory (journal) | <50MB |
| Memory (PDF loaded) | <150MB |

### Reliability

- Graceful offline degradation (view PDF, save annotations locally)
- No data loss on unexpected termination
- Recovery from API failures without crash
- PDF rendering stability

### Security

- API key stored in user-readable config only
- Document content sent only to configured LLM API
- No persistent logging of document content
- Local annotation storage

## Out of Scope (v1.0)

- Offline LLM inference
- EPUB/other document formats
- Voice input/output
- Handwriting-to-LaTeX conversion
- Cloud sync of annotations
- Multi-user collaboration
- reMarkable Paper Pro color features

## Phased Delivery

### Phase 1: Journal Mode (MVP)
- Blank page conversation
- Basic HWR + LLM integration
- Single persona (assistant)

### Phase 2: Document Mode
- PDF loading and rendering
- Margin annotation detection
- Document context in LLM

### Phase 3: Enhanced Experience
- Multiple personas
- Animation effects
- Gesture commands
- Annotation persistence

### Phase 4: Research Mode
- Multi-document context
- Cross-reference queries
- Knowledge base features

### Phase 5: Note-Taking Mode
- Split-screen layout: journal (top 1/4) + writing area (bottom 3/4)
- Automatic text vs. drawing detection
- Persistent journal of recognized text
- Strike-through gesture for text editing/replacement
- Timer-based end-of-writing detection (1.5s idle timeout)
- Endpoint analysis for complete segment detection
- Inline text modification without leaving the page
- Conversation history displayed in journal area
- Real-time text recognition and display

**Key Features:**
- **Journal Area (Top 1/4)**: Displays recognized text chronologically
- **Writing Area (Bottom 3/4)**: Active handwriting input zone
- **Auto-Recognition**: Triggers after 1.5s of inactivity (no circle gesture needed)
- **Endpoint Detection**: Analyzes spatial and temporal coherence of strokes
- **Fade Animation**: Handwriting smoothly fades into text (600ms dithering transition)
- **Stroke Removal**: Writing area clears automatically after recognition
- **Gesture-based Editing**: Strike through text to edit or replace
- **Content Detection**: Distinguish between text and sketches/diagrams
- **Persistent Context**: Journal maintains conversation history

**Use Cases:**
- Meeting notes with AI assistance
- Brainstorming with inline editing
- Personal diary with searchable text
- Quick notes with gesture-based corrections

## Success Metrics

| Metric | Target |
|--------|--------|
| Recognition accuracy | >90% legible writing |
| End-to-end response | <5s (network dependent) |
| Session stability | No crashes in 1-hour sessions |
| PDF render fidelity | Readable at native zoom |
| Battery impact | <10% per hour active |

## Development Tools

### macOS Simulator

To accelerate development and enable testing without constant device deployment, Jedusor includes a GUI-based simulator for macOS (and other desktop platforms).

**Purpose:**
- Local development and testing without reMarkable hardware
- Visual validation of stroke capture and gesture detection
- Rapid iteration on rendering and layout
- Lower barrier to entry for contributors

**Implementation:**
- Window-based simulator using `minifb` crate
- 1404x1872 display matching reMarkable 2 resolution
- Mouse input converted to simulated Wacom events
- Shared rendering pipeline with device code via feature flags

**Capabilities:**
- ✅ Stroke capture and real-time rendering
- ✅ Gesture detection (circle, underline, lasso)
- ✅ Handwriting recognition API integration
- ✅ LLM interaction and response display
- ✅ Text layout and typography testing
- ✅ Screenshot capture (press 'S' key to save PNG)
- ❌ E-ink refresh mode behavior (device-only)
- ❌ Pressure sensitivity (mouse lacks pressure data)
- ❌ ARM performance characteristics

**Architecture:**
```rust
// Feature flags enable simulator or device builds
#[cfg(feature = "device")]
use libremarkable::*;  // Real device

#[cfg(feature = "simulator")]
use simulator::*;  // macOS/desktop simulator

// Shared abstractions work on both
trait InputDevice { ... }
trait DisplayDevice { ... }
```

**⚠️ CRITICAL: Platform Parity Requirement**

The macOS simulator and reMarkable device version **MUST ALWAYS be kept in sync**. They are the **same application** with different platform backends, not separate implementations.

**Requirements:**
- ✅ **Feature Parity**: Every feature implemented on device must work on simulator
- ✅ **Shared Codebase**: Maximum code sharing via traits and feature flags
- ✅ **Unified Testing**: Features validated on simulator before device deployment
- ✅ **Consistent Behavior**: Logic, gestures, recognition flow identical on both
- ❌ **No Drift**: Platform-specific implementations only for I/O layer
- ❌ **No Duplication**: Avoid separate codepaths for same functionality

**Development Workflow:**
1. Implement feature with shared abstractions (traits)
2. Test thoroughly on macOS simulator
3. Deploy to device and verify behavior
4. If device-specific adjustments needed, update simulator to match

**Why This Matters:**
- Simulator is primary development environment (faster iteration)
- If simulator diverges, it becomes useless for validation
- Device deployment is expensive (cross-compile + SSH)
- Contributors without devices depend on simulator accuracy

**Usage:**
```bash
# Run simulator on macOS
cargo run --no-default-features --features simulator

# Run on device (cross-compiled)
cross build --release --target armv7-unknown-linux-gnueabihf
```

**Limitations:**
- Simulator validates logic but not e-ink performance
- Device testing required before production deployment
- Some device-specific bugs may only appear on hardware

See [ADR-008: macOS Simulator](ADR/008-macos-simulator.md) for architectural decision details.

## Installation & Deployment

### Development Deployment (SSH)

For testing and development, deploy via SSH:

```bash
# Build release binary
cargo zigbuild --release --target armv7-unknown-linux-gnueabihf

# Deploy to device
scp target/armv7-unknown-linux-gnueabihf/release/jedusor root@10.11.99.1:/home/root/

# Run via SSH
ssh root@10.11.99.1 ./start-jedusor.sh
```

### Production Deployment (Launcher Integration)

For daily use, integrate with a launcher via Toltec package manager:

**1. Install Toltec** (one-time setup):
```bash
ssh root@10.11.99.1
wget https://toltec-dev.org/bootstrap
bash bootstrap
```

**2. Install a Launcher** (choose one):

- **Remux** (Recommended - lightweight):
  - Simple app launcher in reMarkable menu
  - Minimal overhead, stable
  - `opkg install remux && systemctl enable --now remux`

- **Oxide** (Advanced - full desktop):
  - Complete desktop environment
  - More features but heavier resource usage
  - `opkg install oxide && systemctl enable --now tarnish`

**3. App Discovery**:
- Launchers automatically discover apps in `/opt/bin/` or via `.draft` files
- Jedusor appears in launcher menu - tap to run
- Launchers handle stopping xochitl automatically (exclusive mode)

**Benefits**:
- No SSH needed for daily use
- Tap-to-run from device UI
- Automatic exclusive mode handling
- Better user experience

See [DEPLOY.md](../DEPLOY.md) for detailed deployment instructions and troubleshooting.

## Dependencies

| Dependency | Purpose | Risk |
|------------|---------|------|
| libremarkable | Device I/O | Medium - community |
| minifb | macOS/desktop simulator | Low - stable |
| Google Input Tools | HWR | Low - stable |
| Claude API | LLM | Low - commercial |
| pdf-rs or similar | PDF parsing | Low - mature |
| reMarkable device | Hardware | Low |

## Open Questions

1. How to handle very long PDFs (100+ pages) in context?
2. Should AI responses be saved into the PDF or separate sidecar?
3. Gesture vocabulary: what triggers AI vs. regular annotation?
4. How to visually distinguish user writing from AI responses?
5. Support reMarkable 1, rM2, or Paper Pro only?
