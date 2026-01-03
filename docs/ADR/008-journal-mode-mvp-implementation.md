# ADR-008: Journal Mode MVP Implementation

**Date**: 2026-01-03
**Status**: Implemented
**Epic**: journal-mode

## Context

Journal Mode MVP is the first working implementation of Jedusor on reMarkable hardware. This ADR documents the implementation approach, key technical decisions, and critical fixes made during development.

## Implementation Status

### ✅ Completed Features

1. **Real-time Stroke Rendering**
   - Wacom digitizer input capture (position, pressure, tilt)
   - Stroke rendering with <50ms latency using DU waveform mode
   - Point-by-point accumulation in WacomHandler
   - Bounding box calculation for efficient partial refreshes

2. **Gesture Detection**
   - Circle detection (circularity + closure tolerance)
   - Underline detection (horizontal linearity)
   - Lasso detection (future use)
   - Gesture triggering on stroke completion

3. **Handwriting Recognition**
   - Google Input Tools API integration
   - JSON tuple-based response parsing
   - Stroke-to-ink coordinate conversion
   - API query parameters for handwriting mode

4. **User Interface**
   - Welcome screen with instructions
   - Text rendering using libremarkable draw_text API
   - E-ink optimized refresh modes (DU for strokes, GC16 for text)

5. **System Controls**
   - Middle button exit
   - Power button exit
   - Exclusive mode (xochitl stopped) requirement

### 🚧 Pending Features

1. **AI Integration** (US-02)
   - Claude API client with streaming
   - Response animation
   - Context management

2. **Advanced Gestures** (US-43)
   - Eraser support
   - Selection regions (lasso)

## Critical Technical Fixes

### Fix #1: Stroke Rendering Architecture

**Problem**: Strokes showed "0 points" during rendering, causing no visible ink on screen.

**Root Cause**: Architecture split stroke data between two locations:
- `WacomHandler` accumulated points internally in its own `current_stroke`
- `app.rs` created an empty `Stroke::new()` that never received points

**Solution**: Added `current_stroke()` method to WacomHandler to expose in-progress stroke:

```rust
// In wacom.rs
pub fn current_stroke(&self) -> Option<&Stroke> {
    self.current_stroke.as_ref()
}

// In app.rs
if let Some(stroke) = wacom_handler.current_stroke() {
    // Now has actual points for rendering
    stroke_renderer.draw_stroke(fb, stroke, color::BLACK);
}
```

**Result**: Real-time stroke rendering now works, showing 1, 2, 3... N points as user draws.

### Fix #2: Gesture Detection Missing Path

**Problem**: No gestures were ever detected despite gesture detector code existing.

**Root Cause**: Gesture detection only happened in `WacomEvent::Draw` handler, but strokes complete via `InstrumentChange` events (pen lift). The `InstrumentChange` handler pushed strokes to collection without gesture checking.

**Solution**: Duplicated gesture detection logic in `InstrumentChange` handler:

```rust
libremarkable::input::WacomEvent::InstrumentChange { .. } => {
    if let Ok(Some(stroke)) = wacom_handler.handle_event(WacomEvent::ToolUp) {
        // Added gesture detection here
        match gesture_detector.detect(&stroke) {
            Gesture::Circle { center_x, center_y, .. } => {
                // Trigger recognition and show response
            }
            Gesture::None => {
                debug!("Regular stroke (no gesture)");
            }
            // ... other gestures
        }
        all_strokes.push(stroke);
    }
}
```

**Result**: Circle gestures now properly detected when pen lifts after drawing circle.

### Fix #3: Exclusive Mode Requirement

**Problem**: Running alongside xochitl caused input conflicts and screen corruption.

**Root Cause**: Both Jedusor and xochitl competing for framebuffer and input events.

**Solution**: Updated all launcher scripts to stop xochitl before running Jedusor:

```bash
systemctl stop xochitl
./jedusor
systemctl start xochitl
```

**Result**: Stable input capture and rendering with exclusive framebuffer access.

### Fix #4: Google Input Tools API Integration

**Problem**: API returned `["INPUT_METHOD_NOT_SPECIFIED"]` error.

**Root Cause**: Missing query parameters in API URL.

**Solution**: Added required query parameters:

```rust
api_url: "https://inputtools.google.com/request?ime=handwriting&app=mobilesearch&cs=1&oe=UTF-8"
```

Also updated response parsing from object to tuple format:
```rust
#[derive(Debug, Deserialize)]
struct RecognitionResponse(
    String,  // "SUCCESS" or error status
    Vec<ResponseResult>,
);
```

**Result**: Handwriting recognition successfully converts strokes to text.

## Architecture Decisions

### Event Flow

```
libremarkable WacomEvent::Draw
    ↓
Convert to WacomEvent::ToolDown/ToolMove
    ↓
WacomHandler.handle_event() - accumulates points
    ↓
WacomHandler.current_stroke() - expose for rendering
    ↓
StrokeRenderer.draw_stroke() - real-time display
    ↓
[On pen lift] WacomEvent::InstrumentChange
    ↓
WacomHandler.handle_event(ToolUp) - returns completed stroke
    ↓
GestureDetector.detect() - check for circle/underline/lasso
    ↓
[If Circle] Trigger handwriting recognition
    ↓
GoogleRecognizer.recognize() - API call
    ↓
TextRenderer.draw_text() - show result
```

### E-Ink Refresh Strategy

| Operation | Waveform | Speed | Use Case |
|-----------|----------|-------|----------|
| Stroke rendering | DU | ~50ms | Real-time ink |
| Text display | GC16 | ~260ms | Recognition results |
| Full clear | GC16 | ~450ms | Startup screen |

**Rationale**: DU mode provides fastest refresh for responsive stroke rendering. GC16 provides better quality for text display with acceptable latency.

### Coordinate Systems

**Wacom Digitizer**:
- X: 0-20967
- Y: 0-15725
- Pressure: 0-4095

**Screen Display** (reMarkable 1):
- Width: 1404px
- Height: 1872px
- 226 DPI

No coordinate scaling needed - libremarkable handles conversion internally.

## Performance Characteristics

**Measured on reMarkable 1**:

- Stroke latency: <50ms (acceptable for handwriting)
- CPU usage (release build): 0-2% idle, 15-25% during drawing
- Memory: ~8MB RSS
- Screen refresh: Partial refresh in ~50-120ms depending on region size

**Note**: Debug builds show 70%+ CPU idle due to logging overhead. Always use release builds for testing.

## Deployment Workflow

```bash
# Build (macOS with cargo-zigbuild)
cargo zigbuild --release --target armv7-unknown-linux-gnueabihf

# Deploy
scp target/armv7-unknown-linux-gnueabihf/release/jedusor root@<IP>:/home/root/

# Run (exclusive mode)
ssh root@<IP> ./start-jedusor.sh
```

## Lessons Learned

1. **Architecture Split**: When one component accumulates state (WacomHandler), ensure other components can access it for different purposes (rendering). Adding accessor methods is cleaner than duplicating state.

2. **Event Path Testing**: Test all code paths that handle events. Gesture detection was implemented but unreachable because strokes completed via a different event type.

3. **E-Ink Constraints**: Exclusive framebuffer access is non-negotiable. Attempting to share with xochitl causes unpredictable behavior.

4. **API Integration**: Always check API documentation for required query parameters. Error messages may be generic but root cause is often missing configuration.

5. **Build Configuration**: Debug builds with extensive logging cause severe performance degradation on embedded devices. Always test with release builds.

## Next Steps

1. Implement Claude API client (ADR-003)
2. Add streaming response animation (ADR-005)
3. Build conversation context manager
4. Add eraser tool support
5. Implement undo/redo for strokes

## References

- ADR-001: Programming Language (Rust)
- ADR-002: Handwriting Recognition (Google Input Tools)
- ADR-003: LLM Provider (Claude)
- ADR-004: Device Framework (libremarkable)
- ADR-005: Response Animation
