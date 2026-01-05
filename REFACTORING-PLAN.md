# Code Refactoring Plan: Eliminate Simulator/Device Duplication

## Problem

**CRITICAL:** Massive code duplication between device and simulator:
- Event loop logic duplicated (~300 lines each)
- Gesture detection logic duplicated (3x)
- Recognition triggering duplicated
- Text rendering duplicated
- Total: ~581 lines in app.rs, most duplicated

## Root Cause

Simulator and device use different I/O APIs:
- Device: `libremarkable::framebuffer`, `libremarkable::input`
- Simulator: `minifb::Window`, mouse events

This led to separate event loops with duplicated business logic.

## Solution Architecture

### Trait Abstractions (I/O Layer Only)

```rust
// src/platform/mod.rs

/// Display abstraction - platform-specific
pub trait Display {
    fn clear(&mut self);
    fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, width: i32, color: Color);
    fn draw_text(&mut self, text: &str, x: i32, y: i32, size: f32);
    fn refresh(&mut self);
}

/// Input abstraction - platform-specific
pub trait InputSource {
    fn poll_event(&mut self) -> Option<WacomEvent>;
    fn is_running(&self) -> bool;
}

/// Platform abstraction - combines display + input
pub trait Platform {
    type Display: Display;
    type Input: InputSource;

    fn display(&mut self) -> &mut Self::Display;
    fn input(&mut self) -> &mut Self::Input;
}
```

### Shared Event Handler (Business Logic)

```rust
// src/app.rs (shared code)

pub struct EventHandler {
    wacom_handler: WacomHandler,
    gesture_detector: GestureDetector,
    recognizer: GoogleRecognizer,
    runtime: tokio::Runtime,
    all_strokes: Vec<Stroke>,
}

impl EventHandler {
    pub fn new() -> Result<Self> { ... }

    /// Process single event - SHARED between all platforms
    pub fn handle_event<D: Display>(
        &mut self,
        event: WacomEvent,
        display: &mut D,
    ) -> Result<()> {
        // ALL business logic here - no duplication!
        match self.wacom_handler.handle_event(event)? {
            Some(stroke) => {
                match self.gesture_detector.detect(&stroke) {
                    Gesture::Circle { center_x, center_y, .. } => {
                        self.trigger_recognition(display, center_x, center_y)?;
                    }
                    _ => {}
                }
                self.all_strokes.push(stroke);
            }
            None => {
                // Render in-progress stroke
                if let Some(stroke) = self.wacom_handler.current_stroke() {
                    self.render_stroke(display, stroke)?;
                }
            }
        }
        Ok(())
    }
}
```

### Platform Implementations

```rust
// src/platform/device.rs
#[cfg(feature = "device")]
pub struct DevicePlatform {
    display: DeviceDisplay,
    input: DeviceInput,
}

// src/platform/simulator.rs
#[cfg(feature = "simulator")]
pub struct SimulatorPlatform {
    display: SimulatorDisplay,
    input: SimulatorInput,
}
```

### Unified Event Loop

```rust
// src/app.rs - SINGLE event loop for ALL platforms

impl App {
    pub fn run(&mut self) -> Result<()> {
        #[cfg(feature = "device")]
        self.run_platform::<DevicePlatform>()?;

        #[cfg(feature = "simulator")]
        self.run_platform::<SimulatorPlatform>()?;

        Ok(())
    }

    fn run_platform<P: Platform>(&mut self) -> Result<()> {
        let mut platform = P::new()?;
        let mut handler = EventHandler::new()?;

        // SINGLE event loop - no duplication!
        while platform.input().is_running() {
            if let Some(event) = platform.input().poll_event() {
                handler.handle_event(event, platform.display())?;
            }
        }
        Ok(())
    }
}
```

## Implementation Steps

1. ✅ Create `src/platform/mod.rs` with trait definitions
2. ✅ Create `src/platform/device.rs` with device impl
3. ✅ Create `src/platform/simulator.rs` with simulator impl
4. ✅ Extract shared logic to `EventHandler` in `src/app.rs`
5. ✅ Replace both event loops with single `run_platform()`
6. ✅ Delete duplicated code
7. ✅ Test on simulator
8. ✅ Test on device
9. ✅ Verify 100% code sharing

## Expected Results

### Before
- app.rs: 581 lines (massive duplication)
- Shared code: ~40%
- Platform-specific: ~60% (duplicated logic!)

### After
- app.rs: ~200 lines (shared EventHandler only)
- platform/device.rs: ~100 lines (I/O only)
- platform/simulator.rs: ~100 lines (I/O only)
- Shared code: ~95%
- Platform-specific: ~5% (I/O layer only!)

## Verification

```bash
# Duplication check
grep -r "Circle gesture detected" src/

# Should return:
# src/app.rs:1 (shared EventHandler only!)
# NOT 3 times in separate loops!
```

## Documentation Updates

After code refactoring:
1. Convert CLAUDE.md → CLAUDE.xml
2. Convert docs/PRD.md → docs/PRD.xml
3. Convert docs/ADR/*.md → docs/ADR/*.xml
4. Add ADR-010: Platform Abstraction Architecture
