# ADR-001: Programming Language Selection

## Status
**Accepted**

## Context

We need to select a programming language for developing Jedusor, a native application for the reMarkable tablet. The application requires:
- Low-latency stylus input handling
- Direct framebuffer access for e-ink display
- Cross-compilation to ARM (armv7-unknown-linux-gnueabihf)
- Efficient memory and CPU usage (battery-powered device)
- HTTP client capabilities for API communication

## Decision

**Use Rust** as the primary programming language.

## Options Considered

### Option 1: Rust (Selected)

**Pros:**
- libremarkable framework exists with full device support
- Zero-cost abstractions for performance-critical code
- Memory safety without garbage collection
- Excellent cross-compilation support via `cross` tool
- Strong async ecosystem (tokio) for API calls
- 0% CPU idle, 1-2% peak with optimizations

**Cons:**
- Steeper learning curve
- Longer compilation times
- Smaller ecosystem than C/C++

**Evaluation:** Best fit for native reMarkable development.

### Option 2: C/C++ (Rejected)

**Pros:**
- Official reMarkable SDK targets C/C++
- Maximum control over hardware
- Qt Quick available for UI
- Mature tooling

**Cons:**
- No high-level framework like libremarkable
- Manual memory management risks
- More boilerplate for HTTP/JSON handling
- Would need to reimplement libremarkable's abstractions

**Rejection Reason:** Requires significantly more low-level code. libremarkable already solved the hard problems in Rust.

### Option 3: Python (Rejected)

**Pros:**
- Rapid development
- Rich ML/API libraries
- rmcl library for cloud access

**Cons:**
- No native framebuffer/input library
- GC pauses affect latency
- Larger runtime footprint
- Complex cross-compilation (need Python interpreter on device)
- Higher CPU/memory usage

**Rejection Reason:** Not suitable for real-time stylus input. No native device I/O support.

### Option 4: Go (Rejected)

**Pros:**
- Simple cross-compilation
- Good HTTP client support
- rMAPI exists for cloud features

**Cons:**
- No framebuffer/input framework
- GC pauses (though smaller than Python)
- Larger binary size
- Would need CGO for device access

**Rejection Reason:** No native device framework. GC pauses problematic for real-time input.

### Option 5: JavaScript/TypeScript + Web (Rejected)

**Pros:**
- Existing Tom Riddle diary implementations (ScribbleGPT, diary.ycmjason.com)
- Rich canvas APIs for stroke capture
- Easy LLM integration

**Cons:**
- Requires browser/webview on device
- No native e-ink refresh control
- Higher latency
- Battery drain from browser runtime
- Not a native reMarkable app

**Rejection Reason:** Web approach loses native e-ink control and adds significant overhead.

## Consequences

### Positive
- Access to libremarkable's proven abstractions
- Optimal performance on constrained hardware
- Type safety prevents runtime errors
- Single binary deployment (no runtime dependencies)

### Negative
- Team must be comfortable with Rust
- Compilation requires ARM toolchain setup
- Some libraries may have fewer examples than C equivalents

## Decision Criteria Matrix

| Criteria | Weight | Rust | C++ | Python | Go | JS/Web |
|----------|--------|------|-----|--------|----|----|
| Native framework exists | 30% | ✅ 10 | ⚠️ 5 | ❌ 0 | ❌ 0 | ❌ 0 |
| Performance | 25% | ✅ 10 | ✅ 10 | ❌ 3 | ⚠️ 7 | ❌ 4 |
| Development speed | 20% | ⚠️ 6 | ❌ 4 | ✅ 10 | ✅ 8 | ✅ 9 |
| Memory safety | 15% | ✅ 10 | ❌ 3 | ✅ 10 | ✅ 10 | ✅ 10 |
| API ecosystem | 10% | ✅ 8 | ⚠️ 6 | ✅ 10 | ✅ 9 | ✅ 10 |
| **Weighted Score** | | **8.5** | **5.9** | **5.4** | **5.3** | **4.9** |
