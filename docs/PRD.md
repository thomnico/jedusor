# Product Requirements Document: Jedusor

## Overview

**Product Name**: Jedusor
**Version**: 1.0
**Date**: 2026-01-02

Jedusor recreates the magical Tom Riddle diary experience from Harry Potter on the reMarkable tablet. Users write questions with the stylus, and an AI entity responds as if the journal itself is alive, with text appearing magically on the e-ink display.

## Problem Statement

E-ink tablets excel at distraction-free writing but lack interactive, magical experiences. Existing AI chat interfaces are screen-based and lack the tactile intimacy of writing by hand. There is no native way to have a conversational AI experience that feels like writing in a sentient journal.

## Target Users

- Harry Potter fans with reMarkable tablets
- Writers seeking creative AI interaction through handwriting
- Users wanting a unique, immersive journaling experience
- Developers interested in reMarkable application development

## User Stories

### Core Experience

| ID | Story | Priority |
|----|-------|----------|
| US-01 | As a user, I can write a message with the stylus and see it recognized as text | P0 |
| US-02 | As a user, I see the AI response appear gradually as if being written by invisible ink | P0 |
| US-03 | As a user, the journal remembers our conversation within a session | P0 |
| US-04 | As a user, I can start a new conversation by clearing the page | P1 |
| US-05 | As a user, I can configure the AI's personality/persona | P2 |

### Writing Experience

| ID | Story | Priority |
|----|-------|----------|
| US-10 | As a user, I see my strokes rendered in real-time with low latency | P0 |
| US-11 | As a user, I can erase strokes using the pen's eraser end | P1 |
| US-12 | As a user, handwriting recognition works for cursive and print | P0 |
| US-13 | As a user, I can write anywhere on the page, not just lined areas | P1 |

### Response Display

| ID | Story | Priority |
|----|-------|----------|
| US-20 | As a user, AI responses appear in a handwriting-style font | P0 |
| US-21 | As a user, responses appear with a letter-by-letter animation | P1 |
| US-22 | As a user, responses fade or sink into the page after reading | P2 |
| US-23 | As a user, I can scroll through conversation history | P2 |

### System

| ID | Story | Priority |
|----|-------|----------|
| US-30 | As a user, the app works when I have WiFi connectivity | P0 |
| US-31 | As a user, I receive clear feedback when offline | P1 |
| US-32 | As a user, the app doesn't drain my battery excessively | P0 |
| US-33 | As a user, I can exit the app cleanly with a gesture | P0 |

## Functional Requirements

### FR-1: Stylus Input Capture

- Capture Wacom digitizer events (position, pressure, tilt)
- Render strokes at <16ms latency for natural feel
- Support both pen tip and eraser tool detection
- Detect stroke completion (pen lift) to trigger recognition

### FR-2: Handwriting Recognition

- Convert stroke data to text with >90% accuracy for legible handwriting
- Support multiple languages (English minimum, French P1)
- Process recognition within 500ms of stroke completion
- Handle cursive, print, and mixed writing styles

### FR-3: LLM Integration

- Send recognized text to Claude API
- Maintain conversation context (system prompt + history)
- Implement Tom Riddle persona via system prompt
- Handle API errors gracefully with user feedback
- Respect rate limits and implement retry logic

### FR-4: Response Rendering

- Display AI text in handwriting-style typography
- Animate text appearance (ink materializing effect)
- Support word wrapping within page bounds
- Implement appropriate e-ink refresh strategy

### FR-5: Session Management

- Maintain conversation history during session
- Clear conversation on explicit user action (gesture/button)
- Persist API key securely on device

## Non-Functional Requirements

### Performance

| Metric | Target |
|--------|--------|
| Stroke rendering latency | <16ms |
| Recognition latency | <500ms |
| LLM response start | <2s (network dependent) |
| CPU usage (idle) | <1% |
| CPU usage (active) | <10% |
| Memory footprint | <50MB |

### Reliability

- Graceful degradation when offline
- No data loss on unexpected termination
- Recovery from API failures without crash

### Usability

- No tutorial required for basic use
- Single-hand operation possible
- Works in any screen orientation (portrait default)

### Security

- API key stored in user-readable config only
- No transmission of data beyond recognition + LLM APIs
- No persistent logging of conversation content

## Out of Scope (v1.0)

- Offline LLM inference
- Custom persona creation UI
- Multi-page document support
- Export conversation to PDF/text
- Voice input/output
- reMarkable Paper Pro specific features
- Cloud sync of conversations

## Success Metrics

| Metric | Target |
|--------|--------|
| Recognition accuracy | >90% for legible writing |
| End-to-end response time | <5s (network dependent) |
| Session stability | No crashes in 1-hour sessions |
| Battery impact | <10% per hour of active use |

## Dependencies

| Dependency | Purpose | Risk |
|------------|---------|------|
| libremarkable | Device I/O framework | Medium - community maintained |
| Google Input Tools | Handwriting recognition | Low - stable, undocumented |
| Claude API | LLM responses | Low - commercial SLA |
| reMarkable device | Target hardware | Low - owned by user |

## Timeline

| Phase | Scope |
|-------|-------|
| Alpha | Stroke capture + recognition + basic LLM response |
| Beta | Animation, persona tuning, error handling |
| Release | Polish, performance optimization, documentation |

## Open Questions

1. Should responses fade/disappear like in the movie, or persist?
2. What gesture should clear the conversation?
3. Should we support the reMarkable 1 or focus on rM2 only?
4. How to handle very long AI responses on limited screen?
