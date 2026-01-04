# Architecture Decision Records

This directory contains the Architecture Decision Records (ADRs) for the Jedusor project.

## Index

| ADR | Title | Status | Summary |
|-----|-------|--------|---------|
| [001](001-programming-language.md) | Programming Language | Accepted | **Rust** selected over C++, Python, Go, JavaScript |
| [002](002-handwriting-recognition.md) | Handwriting Recognition | Accepted | **Google Input Tools API** selected over Tesseract, on-device ML, Mathpix |
| [003](003-llm-provider.md) | LLM Provider | Accepted | **Claude (Anthropic)** selected over GPT-4, local LLM, Gemini |
| [004](004-device-framework.md) | Device Framework | Accepted | **libremarkable** selected over direct evdev, Qt Quick, Toltec |
| [005](005-response-animation.md) | Response Animation | Accepted | **Character/word streaming** selected over fade-in, all-at-once |
| [006](006-existing-projects-analysis.md) | Existing Projects Analysis | Informational | Gap analysis of reMarkableAI, armrest, whiteboard-hypercard, ScribbleGPT |
| [007](007-pdf-integration.md) | PDF Integration | Accepted | **lopdf + mupdf** selected over Poppler, pdf.js, pure pdf-rs |
| [008](008-macos-simulator.md) | macOS Development Simulator | Accepted | **GUI simulator with minifb** selected over terminal-based, web-based |
| [008](008-journal-mode-mvp-implementation.md) | Journal Mode MVP Implementation | Implemented | Documents stroke rendering, gesture detection, recognition integration, deployment |
| [009](009-note-taking-mode.md) | Note-Taking Mode | Proposed | **Split-screen layout** with journal (top 1/4) + writing area (bottom 3/4), gesture-based editing |

## Decision Summary

### Selected Stack

```
┌─────────────────────────────────────────┐
│            Jedusor Stack                │
├─────────────────────────────────────────┤
│  Language:     Rust                     │
│  Framework:    libremarkable 0.7.x      │
│  Simulator:    minifb (macOS/desktop)   │
│  Recognition:  Google Input Tools API   │
│  LLM:          Claude (Anthropic API)   │
│  Animation:    Streaming + Partial GC16 │
│  PDF:          lopdf + mupdf            │
└─────────────────────────────────────────┘
```

### Rejected Alternatives Summary

| Category | Rejected | Primary Reason |
|----------|----------|----------------|
| Language | C++ | No high-level framework, more boilerplate |
| Language | Python | No native device I/O, GC pauses |
| Language | Go | No framebuffer framework, GC pauses |
| Language | JavaScript | No native e-ink control, browser overhead |
| Recognition | Tesseract | Poor handwriting accuracy (OCR focused) |
| Recognition | On-device ML | Development risk, accuracy concerns |
| Recognition | Mathpix | Cost, STEM specialization |
| LLM | GPT-4 | Higher cost, less persona consistency |
| LLM | Local LLM | Hardware limitations (1GB RAM) |
| LLM | Gemini | Weaker creative writing |
| Framework | Qt Quick | No partial refresh control, heavyweight |
| Framework | Direct evdev | Reinventing solved problems |
| Animation | Fade-in | E-ink doesn't support opacity |
| Animation | All-at-once | Loses magical effect |
| PDF | Poppler | Cross-compilation complexity, binary size |
| PDF | pdf.js | Requires WebView, memory hungry |
| PDF | Pure pdf-rs | Insufficient rendering quality |
| Simulator | Terminal-based | No visual feedback, poor UX |
| Simulator | Web-based | Over-engineered, separate codebase |

## ADR Template

When adding new ADRs, use this template:

```markdown
# ADR-NNN: Title

## Status
[Proposed | Accepted | Deprecated | Superseded by ADR-XXX]

## Context
[Why is this decision needed?]

## Decision
[What is the decision and brief rationale?]

## Options Considered
[List all options with pros/cons]

## Consequences
[What are the positive and negative results?]

## Decision Criteria Matrix
[Weighted scoring if applicable]
```

## Future ADRs (Planned)

- ADR-010: Conversation persistence strategy
- ADR-011: Configuration management
- ADR-012: Error handling and offline behavior
- ADR-013: Testing strategy
- ADR-014: Multi-language support
