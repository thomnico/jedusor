# ADR-004: Device Framework Selection

## Status
**Accepted**

## Context

Jedusor needs to interact with reMarkable hardware:
- Wacom digitizer for stylus input (coordinates, pressure, tilt)
- E-ink framebuffer for display output
- Partial refresh control for low-latency updates
- Physical buttons and touch input

## Decision

**Use libremarkable** as the device interaction framework.

## Options Considered

### Option 1: libremarkable (Selected)

**Repository:** https://github.com/canselcik/libremarkable

**Pros:**
- Only public framework with native e-ink refresh support
- Complete abstraction of all input devices
- ApplicationContext provides event loop pattern
- Proven performance (0% idle CPU, 1-2% peak)
- Active maintenance (v0.7.0, September 2025)
- reMarkable 2 framebuffer client built-in
- Optional GUI framework included

**Cons:**
- Rust-only (no C bindings for other languages)
- ~29% documentation coverage
- Must recompile for library updates
- Community maintained, not official

**Evaluation:** Best and only viable option for native reMarkable development with refresh control.

### Option 2: Direct evdev + Framebuffer (Rejected)

**Approach:** Use Linux evdev for input, mmap /dev/fb0 for display

**Pros:**
- No external dependencies
- Maximum control
- Works with any language

**Cons:**
- Must reverse-engineer e-ink refresh IOCTLs
- Significant development effort
- libremarkable already solved these problems
- No partial refresh without understanding proprietary interfaces

**Rejection Reason:** Reinventing the wheel. libremarkable provides this layer already.

### Option 3: Qt Quick (Official SDK) (Rejected)

**Pros:**
- Official reMarkable SDK support
- Mature UI framework
- Cross-platform development possible

**Cons:**
- No direct partial refresh control
- Higher resource usage
- Overkill for single-page journal UI
- C++ complexity for simple app
- QML learning curve

**Rejection Reason:** Too heavyweight. Designed for complex apps, not single-purpose tools.

### Option 4: RemarkableFramebuffer (Rejected)

**Repository:** https://github.com/canselcik/RemarkableFramebuffer (precursor to libremarkable)

**Pros:**
- Documentation of hardware internals
- C implementation available

**Cons:**
- Superseded by libremarkable
- Less maintained
- Missing newer device support

**Rejection Reason:** Obsolete. Use libremarkable instead.

### Option 5: Toltec + Existing Apps (Rejected)

**Approach:** Build on Toltec package ecosystem, use existing components

**Pros:**
- Community packages available
- Display server (rm2fb) handles rM2 framebuffer

**Cons:**
- Dependency on external display server
- Less control over refresh timing
- Additional system complexity
- Not suitable for custom app development

**Rejection Reason:** Designed for running existing apps, not building new ones. libremarkable includes rm2fb client.

## Consequences

### Positive
- Battle-tested input handling code
- Proven refresh mode abstractions
- Simple event loop pattern
- Good performance out of box

### Negative
- Tied to Rust ecosystem
- Must track library updates
- Limited documentation requires reading source

## Integration Pattern

```rust
use libremarkable::appctx::ApplicationContext;
use libremarkable::input::{InputEvent, WacomEvent, WacomPen};
use libremarkable::framebuffer::{
    FramebufferDraw, FramebufferRefresh,
    common::{color, mxcfb_rect, DRAWING_QUANT_BIT}
};

fn main() {
    let mut app = ApplicationContext::default();

    // Get framebuffer for drawing
    let fb = app.get_framebuffer_ref();

    // Event loop
    app.start_event_loop(false, true, false, |ctx, event| {
        match event {
            InputEvent::WacomEvent { event } => {
                match event {
                    WacomEvent::Draw { position, pressure, tilt } => {
                        // Render stroke
                    }
                    WacomEvent::InstrumentChange { pen, state } => {
                        // Handle pen/eraser switch
                    }
                    _ => {}
                }
            }
            InputEvent::MultitouchEvent { event } => {
                // Handle gestures
            }
            InputEvent::GPIO { event } => {
                // Handle physical buttons
            }
            _ => {}
        }
    });
}
```

## E-Ink Refresh Modes

| Mode | Use Case | Speed | Quality |
|------|----------|-------|---------|
| Full (GC16) | Page clear, final render | Slow (~450ms) | Best, no ghosting |
| Partial (DU) | UI elements, buttons | Fast (~120ms) | Binary only |
| Partial (GC16) | Text display | Medium (~260ms) | Good grayscale |
| Partial (A2) | Real-time strokes | Fastest (~50ms) | Ghosting, low quality |

## Decision Criteria Matrix

| Criteria | Weight | libremarkable | Direct evdev | Qt Quick | Toltec |
|----------|--------|---------------|--------------|----------|--------|
| E-ink refresh control | 35% | ✅ 10 | ⚠️ 5 | ❌ 3 | ⚠️ 6 |
| Development effort | 25% | ✅ 9 | ❌ 2 | ⚠️ 5 | ⚠️ 6 |
| Performance | 20% | ✅ 10 | ✅ 10 | ⚠️ 6 | ⚠️ 7 |
| Documentation | 10% | ⚠️ 5 | ❌ 2 | ✅ 9 | ⚠️ 6 |
| Maintenance | 10% | ⚠️ 7 | ❌ 3 | ✅ 9 | ⚠️ 6 |
| **Weighted Score** | | **8.9** | **4.4** | **5.1** | **6.3** |
