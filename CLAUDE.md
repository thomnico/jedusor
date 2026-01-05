<?xml version="1.0" encoding="UTF-8"?>
<claude-documentation>
  <meta>
    <purpose>Guidance for Claude Code (claude.ai/code) when working with this repository</purpose>
  </meta>

  <project-overview>
    <description>
      <strong>Jedusor</strong> is a stylus-first AI interaction layer for reMarkable tablets.
      Write questions, annotations, or commands by hand—the AI responds directly on the e-ink display.
    </description>

    <interaction-modes>
      <mode name="journal">Blank page conversation (magical diary experience)</mode>
      <mode name="document">AI assistant overlaid on PDFs, responds to margin annotations</mode>
      <mode name="research" status="future">Multi-document context, cross-reference questions</mode>
    </interaction-modes>
  </project-overview>

  <architecture>
    <diagram type="ascii">
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
    </diagram>

    <core-components>
      <component name="Input Handler" path="src/input/">
        Wacom stylus strokes + gesture detection (circle, underline, lasso)
      </component>
      <component name="Stroke Engine" path="src/stroke/">
        Stroke collection, gesture recognition
      </component>
      <component name="Recognition Service" path="src/recognition/">
        Handwriting to text (Google Input Tools API)
      </component>
      <component name="PDF Module" path="src/pdf/">
        Document loading, rendering, text extraction (lopdf + mupdf)
      </component>
      <component name="Context Manager" path="src/context/">
        Combines annotations + document content for LLM
      </component>
      <component name="LLM Client" path="src/llm/">
        Claude API with persona management
      </component>
      <component name="Renderer" path="src/render/">
        E-ink optimized display with streaming animation
      </component>
      <component name="App Context" path="src/app/">
        Mode switching, state, event loop
      </component>
      <component name="Platform Abstraction" path="src/platform/">
        Trait-based I/O abstraction for device/simulator parity
      </component>
    </core-components>
  </architecture>

  <technology-stack>
    <language msrv="1.80+">Rust</language>
    <framework version="0.7.x" url="https://github.com/canselcik/libremarkable">libremarkable</framework>
    <pdf-handling>
      <library>lopdf (text extraction)</library>
      <library>mupdf (rendering)</library>
    </pdf-handling>
    <target>armv7-unknown-linux-gnueabihf (reMarkable 1/2)</target>
    <build-tool>cross (recommended) or official reMarkable toolchain</build-tool>
  </technology-stack>

  <build-commands>
    <simulator platform="macOS/Desktop">
      <command purpose="run-simulator">cargo run --no-default-features --features simulator</command>
      <command purpose="build-simulator">cargo build --no-default-features --features simulator</command>
      <command purpose="test-simulator">cargo test --no-default-features --features simulator</command>
    </simulator>

    <device platform="reMarkable">
      <setup>
        <command purpose="install-cross">cargo install cross</command>
      </setup>
      <build>
        <command purpose="debug" target="armv7-unknown-linux-gnueabihf">cross build --target armv7-unknown-linux-gnueabihf</command>
        <command purpose="release" target="armv7-unknown-linux-gnueabihf">cross build --release --target armv7-unknown-linux-gnueabihf</command>
        <command purpose="static-link" target="armv7-unknown-linux-musleabihf">cross build --release --target armv7-unknown-linux-musleabihf</command>
      </build>
      <test>
        <command purpose="all-tests">cargo test --no-default-features</command>
        <command purpose="single-test">cargo test test_name --no-default-features</command>
      </test>
      <lint>
        <command purpose="check">cargo check</command>
        <command purpose="lint">cargo clippy --target armv7-unknown-linux-gnueabihf</command>
      </lint>
    </device>
  </build-commands>

  <platform-parity-rule criticality="CRITICAL">
    <principle>
      The macOS simulator and reMarkable device version MUST ALWAYS be kept in sync.
      They are the SAME APPLICATION with different I/O backends, not separate implementations.
    </principle>

    <requirements>
      <requirement name="feature-parity" priority="non-negotiable">
        Every feature must work on both platforms:
        - If it works on device but not simulator → Fix simulator immediately
        - If it works on simulator but not device → Fix device immediately
        - No "device-only" or "simulator-only" features (except I/O layer)
      </requirement>

      <requirement name="shared-codebase" priority="non-negotiable">
        Maximum code sharing (~95% achieved):
        - Business logic: 100% shared (in EventHandler)
        - Gesture detection: 100% shared (GestureDetector)
        - Recognition: 100% shared (GoogleRecognizer)
        - Rendering logic: 100% shared (via Display trait)
        - Only I/O layer differs (libremarkable vs minifb)
      </requirement>

      <requirement name="development-workflow" priority="mandatory">
        <step>1. Implement feature with shared abstractions (traits)</step>
        <step>2. Test thoroughly on macOS simulator</step>
        <step>3. Deploy to device and verify identical behavior</step>
        <step>4. If behavior differs, update both to match</step>
        <step>5. Commit only when both platforms work identically</step>
      </requirement>

      <requirement name="rationale">
        Why this matters:
        - Simulator is primary development environment (10x faster iteration)
        - Device deployment is slow (cross-compile + SSH = 30+ seconds)
        - Contributors without devices depend on simulator accuracy
        - Platform drift makes simulator useless for validation
      </requirement>
    </requirements>

    <implementation-guidelines>
      <guideline>Use #[cfg(feature = "device")] and #[cfg(feature = "simulator")] ONLY for I/O</guideline>
      <guideline>Share all logic via traits: Display, InputSource, Platform</guideline>
      <guideline>Test on simulator first, then verify on device</guideline>
      <guideline>If you break parity, you break the development workflow</guideline>
    </implementation-guidelines>
  </platform-parity-rule>

  <deployment target="reMarkable">
    <connection>
      <ip>10.11.99.1 (USB connection)</ip>
    </connection>
    <commands>
      <command purpose="copy-binary">scp target/armv7-unknown-linux-gnueabihf/release/jedusor root@10.11.99.1:</command>
      <command purpose="run-ssh">ssh root@10.11.99.1 ./jedusor</command>
      <command purpose="one-liner">scp target/armv7-unknown-linux-gnueabihf/release/jedusor root@10.11.99.1: &amp;&amp; ssh root@10.11.99.1 ./jedusor</command>
    </commands>
  </deployment>

  <configuration>
    <environment-variables>
      <variable name="ANTHROPIC_API_KEY" required="true">Claude API key</variable>
      <variable name="JEDUSOR_MODEL" default="claude-sonnet-4-20250514">Claude model ID</variable>
      <variable name="JEDUSOR_RECOGNITION" default="google" values="google|local">Handwriting recognition backend</variable>
      <variable name="JEDUSOR_MODE" default="journal" values="journal|document">Startup interaction mode</variable>
    </environment-variables>
  </configuration>

  <implementation-notes>
    <section name="libremarkable-patterns">
      <code language="rust">
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
      </code>
    </section>

    <section name="wacom-digitizer-events">
      <specification>
        <coordinate name="X" range="0-20967"/>
        <coordinate name="Y" range="0-15725"/>
        <pressure range="0-4095"/>
        <tilt range="-9000 to 9000" axes="X and Y"/>
        <tool-types>
          <tool>BTN_TOOL_PEN</tool>
          <tool>BTN_TOOL_RUBBER</tool>
        </tool-types>
      </specification>
    </section>

    <section name="eink-refresh-modes">
      <modes>
        <mode name="Full (GC16)" use-case="Page clear, final render" speed="~450ms"/>
        <mode name="Partial (DU)" use-case="UI elements, buttons" speed="~120ms"/>
        <mode name="Partial (GC16)" use-case="Text display" speed="~260ms"/>
        <mode name="Partial (A2)" use-case="Real-time strokes" speed="~50ms"/>
      </modes>
    </section>

    <section name="performance-considerations">
      <tip>Always build with --release (debug builds cause 70%+ CPU idle)</tip>
      <tip>Use partial refresh for stroke rendering, full refresh sparingly</tip>
      <tip>Batch UI updates when possible</tip>
      <tip>Release builds achieve 0% idle, 1-2% peak CPU</tip>
    </section>

    <section name="pdf-context-strategy">
      <description>For large documents, use windowed context</description>
      <code language="rust">
enum ContextStrategy {
    CurrentPage,                           // Only current page
    Window { before: usize, after: usize }, // Adjacent pages
    AnnotatedPages,                        // Pages with user annotations
    SemanticSearch { query: String },      // Relevant sections
}
      </code>
    </section>
  </implementation-notes>

  <handwriting-recognition>
    <provider name="Google Input Tools API" implementation="src/recognition/google.rs">
      <api>
        <endpoint>https://inputtools.google.com/request</endpoint>
        <method>POST</method>
        <latency typical="&lt;500ms" timeout="5s"/>
        <accuracy>&gt;90% for legible handwriting</accuracy>
        <languages default="English">Configurable for other languages</languages>
        <retry-logic>3 attempts with exponential backoff</retry-logic>
        <network-required>true</network-required>
      </api>

      <simulator-usage>
        <step>1. Draw text strokes with mouse</step>
        <step>2. Draw a circle gesture to trigger recognition</step>
        <step>3. Recognized text displays with confidence score</step>
        <example>You wrote: Hello (Confidence: 95%)</example>
      </simulator-usage>

      <stroke-format>
        <json-structure>
{
  "ink": [[x1, x2, ...], [y1, y2, ...], [t1, t2, ...]],
  "writing_guide": {"width": 1404, "height": 1872}
}
        </json-structure>
        <components>
          <component name="X coordinates">Wacom space: 0-20967</component>
          <component name="Y coordinates">Wacom space: 0-15725</component>
          <component name="Timestamps">Milliseconds from session start</component>
        </components>
      </stroke-format>

      <response-format>
        <code language="rust">
RecognitionResult {
    text: "Hello",           // Top candidate
    confidence: 0.95,         // 0.0 to 1.0
    alternatives: ["Hollo"]   // Up to 4 alternatives
}
        </code>
      </response-format>
    </provider>
  </handwriting-recognition>

  <project-structure>
    <directory name="jedusor" type="root">
      <file>Cargo.toml</file>
      <file>CLAUDE.xml</file>
      <file>REFACTORING-PLAN.md</file>

      <directory name="docs">
        <file>PRD.md - Product requirements</file>
        <directory name="ADR" description="Architecture decisions">
          <file>000-index.md</file>
          <file>001-programming-language.md</file>
          <file>002-handwriting-recognition.md</file>
          <file>003-llm-provider.md</file>
          <file>004-device-framework.md</file>
          <file>005-response-animation.md</file>
          <file>006-existing-projects-analysis.md</file>
          <file>007-pdf-integration.md</file>
          <file>008-macos-simulator.md</file>
        </directory>
      </directory>

      <directory name="src">
        <file>main.rs - Entry point</file>

        <directory name="app">
          <file>mod.rs - Mode switching, event loops</file>
          <file>event_handler.rs - SHARED business logic (all platforms)</file>
        </directory>

        <directory name="input">
          <file>mod.rs</file>
          <file>wacom.rs - Stylus events</file>
          <file>gesture.rs - Circle, underline, lasso detection</file>
        </directory>

        <directory name="stroke">
          <file>mod.rs - Stroke data structures</file>
        </directory>

        <directory name="recognition">
          <file>mod.rs</file>
          <file>google.rs - Google Input Tools integration</file>
        </directory>

        <directory name="pdf">
          <file>mod.rs</file>
          <file>loader.rs - PDF loading (lopdf)</file>
          <file>renderer.rs - Page rendering (mupdf)</file>
          <file>extractor.rs - Text + position extraction</file>
        </directory>

        <directory name="context">
          <file>mod.rs</file>
          <file>manager.rs - LLM context building</file>
        </directory>

        <directory name="llm">
          <file>mod.rs</file>
          <file>claude.rs - Anthropic API client</file>
          <file>persona.rs - System prompts</file>
        </directory>

        <directory name="render">
          <file>mod.rs</file>
          <file>text.rs - Text layout</file>
          <file>strokes.rs - Stroke rendering</file>
        </directory>

        <directory name="platform" description="I/O abstraction layer">
          <file>mod.rs - Display, InputSource, Platform traits</file>
          <file>device.rs - reMarkable device implementation</file>
          <file>simulator.rs - macOS simulator implementation</file>
        </directory>

        <directory name="simulator" feature="simulator">
          <file>mod.rs</file>
          <file>window.rs - minifb window for macOS</file>
        </directory>
      </directory>

      <directory name="assets">
        <directory name="fonts">
          <file>Caveat-Regular.ttf - Handwriting-style font</file>
        </directory>
      </directory>

      <directory name="tests">
        <description>Integration and unit tests</description>
      </directory>
    </directory>
  </project-structure>

  <references>
    <reference name="libremarkable" url="https://github.com/canselcik/libremarkable">Device framework</reference>
    <reference name="lopdf" url="https://github.com/J-F-Liu/lopdf">PDF text extraction</reference>
    <reference name="mupdf" url="https://mupdf.com/">PDF rendering</reference>
    <reference name="reMarkable Developer SDK" url="https://developer.remarkable.com/documentation/sdk">Official SDK documentation</reference>
    <reference name="awesome-reMarkable" url="https://github.com/reHackable/awesome-reMarkable">Community resources</reference>
    <reference name="reMarkableAI" url="https://github.com/nickian/reMarkableAI" type="similar-project"/>
    <reference name="armrest" url="https://github.com/bkirwi/armrest" type="similar-project"/>
  </references>
</claude-documentation>
