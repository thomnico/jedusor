# ADR-008: macOS Development Simulator

## Status
**Accepted**

## Context

Jedusor targets the reMarkable tablet (ARM Linux with e-ink display and Wacom digitizer). The `libremarkable` framework provides device access but only works on Linux with specific hardware. This creates development friction:

- Developers need physical reMarkable devices for testing
- Cross-compilation to ARM slows iteration cycles
- E-ink display behavior can't be tested locally
- Gesture detection requires device deployment
- Full write-deploy-test cycle takes 30+ seconds

We need a way to develop and test core functionality (stroke capture, gesture detection, rendering) on macOS without a physical device.

## Decision

**Implement a GUI-based simulator using `minifb`** that mimics the reMarkable's display and input system on macOS.

## Options Considered

### Option 1: Terminal-Based Simulator (Rejected)

**Approach:** ASCII art visualization of strokes in terminal.

**Pros:**
- Zero GUI dependencies
- Works over SSH
- Minimal code (~50 lines)
- Fast to implement

**Cons:**
- No visual feedback for gestures
- Can't test rendering quality
- Limited stroke resolution
- Poor developer experience
- Can't validate e-ink refresh behavior

**Rejection Reason:** Too limited for realistic testing. Developers need visual feedback to validate gesture detection and rendering.

### Option 2: GUI Simulator with minifb (Selected)

**Approach:** Window-based simulator with mouse input converted to Wacom events.

**Pros:**
- Visual stroke rendering matches device output
- Mouse simulates stylus with realistic coordinates
- Can test gesture detection algorithms visually
- Validates rendering pipeline before deployment
- 1404x1872 window matches reMarkable 2 resolution
- Cross-platform (works on macOS, Linux, Windows)
- Fast iteration: immediate feedback without deployment
- `minifb` is lightweight (~15KB, no native dependencies)

**Cons:**
- Additional dependency in Cargo.toml
- ~200-300 lines of simulator code
- Mouse input lacks pressure sensitivity
- Can't test actual e-ink refresh modes
- Doesn't validate ARM cross-compilation

**Evaluation:** Best balance of realism and development speed. Visual feedback critical for gesture tuning.

### Option 3: Web-Based Simulator (Rejected)

**Approach:** Canvas-based web app with WebSocket bridge to Rust backend.

**Pros:**
- Rich debugging UI possible
- Could record/replay strokes
- Shareable (team can test via URL)
- Browser DevTools for debugging

**Cons:**
- Complex architecture (Rust backend + web frontend)
- WebSocket serialization overhead
- Doesn't test actual rendering code
- Two codebases to maintain
- Network latency affects testing

**Rejection Reason:** Over-engineered for local development. Adds complexity without testing actual rendering pipeline.

## Implementation Details

### Simulator Architecture

```rust
// Feature flag structure
#[cfg(feature = "device")]
use libremarkable::*;  // Real device

#[cfg(feature = "simulator")]
use simulator::*;  // macOS simulator

// Shared trait for both
trait InputDevice {
    fn poll_events(&mut self) -> Vec<InputEvent>;
}

trait DisplayDevice {
    fn draw_stroke(&mut self, stroke: &Stroke);
    fn refresh(&mut self, mode: RefreshMode);
}
```

### Coordinate Mapping

- **reMarkable 2 digitizer:** 20967 x 15725 (Wacom coordinates)
- **reMarkable 2 display:** 1404 x 1872 pixels
- **Simulator window:** 1404 x 1872 (matches display)
- **Mouse → Wacom conversion:** Scale mouse coordinates to Wacom range

### Event Simulation

```rust
// Mouse down → Wacom ToolDown
MouseEvent::Press(x, y) => WacomEvent::ToolDown {
    tool: Tool::Pen,
    x: scale_to_wacom_x(x),
    y: scale_to_wacom_y(y),
    pressure: 2048,  // Fixed mid-range pressure
}
```

### Screenshot Capture

The simulator includes built-in screenshot functionality for debugging and documentation:

- **Activation:** Press 'S' key
- **Output:** PNG file with timestamp (`jedusor-screenshot-{timestamp}.png`)
- **Format:** 1404x1872 RGB image matching display resolution
- **Use cases:**
  - Visual verification of rendering
  - Documentation of gesture detection
  - Bug reports with visual evidence
  - Tutorial screenshots

Implementation uses the `image` crate to convert the u32 framebuffer to RGB8 format and save as PNG.

### Testing Scope

**Can Test:**
- ✅ Stroke capture and collection
- ✅ Gesture detection (circle, underline, lasso)
- ✅ Text rendering and layout
- ✅ Response zone positioning
- ✅ Handwriting recognition API calls
- ✅ LLM integration
- ✅ Full interaction pipeline
- ✅ Screenshot capture for debugging (press 'S' key)

**Cannot Test:**
- ❌ E-ink refresh modes (GC16, DU, A2)
- ❌ Actual ARM performance
- ❌ Pressure sensitivity behavior
- ❌ Tilt detection
- ❌ Eraser tool
- ❌ Battery impact

## Consequences

### Positive

- **Faster iteration:** Test changes in seconds instead of minutes
- **Lower barrier to entry:** Contributors don't need reMarkable devices
- **Visual debugging:** See strokes and gestures in real-time
- **Continuous testing:** Run tests locally without device deployment
- **Parallel development:** Multiple developers can work simultaneously
- **Gesture tuning:** Visually validate circle/underline detection

### Negative

- **Dependency added:** `minifb = "0.27"` in Cargo.toml
- **Code divergence risk:** Simulator behavior may drift from device
- **Pressure blind spot:** Can't test pressure-dependent features
- **E-ink unknowns:** Refresh mode performance issues found only on device
- **Maintenance burden:** Simulator code needs updates as device code evolves
- **False confidence:** Tests may pass in simulator but fail on device

### Mitigation Strategies

- Use feature flags to share core logic: `#[cfg(any(feature = "device", feature = "simulator"))]`
- Document simulator limitations clearly in README
- Require device testing before merging critical features
- Add integration tests that run on both simulator and device
- Keep simulator code minimal (< 300 lines)

## Decision Criteria Matrix

| Criteria | Weight | Terminal | minifb GUI | Web Simulator |
|----------|--------|----------|------------|---------------|
| Visual feedback | 30% | ❌ 2 | ✅ 10 | ✅ 10 |
| Implementation effort | 25% | ✅ 10 | ✅ 8 | ❌ 3 |
| Testing realism | 20% | ❌ 3 | ✅ 9 | ⚠️ 6 |
| Iteration speed | 15% | ✅ 10 | ✅ 10 | ⚠️ 7 |
| Maintenance cost | 10% | ✅ 10 | ✅ 8 | ❌ 4 |
| **Weighted Score** | | **6.0** | **9.1** | **6.6** |

## References

- [minifb crate](https://crates.io/crates/minifb) - Cross-platform window/framebuffer library
- [libremarkable coordinates](https://github.com/canselcik/libremarkable) - Wacom digitizer specs
- reMarkable 2 display: 1404x1872 @ 226 DPI
- Wacom digitizer: 20967x15725 coordinate space

## Related ADRs

- ADR-001: Programming Language Selection (Rust enables feature flags for simulator)
- ADR-004: Device Framework (libremarkable traits can be abstracted for simulator)
