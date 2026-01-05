<?xml version="1.0" encoding="UTF-8"?>
<product-requirements-document>
  <metadata>
    <product-name>Jedusor</product-name>
    <version>1.0</version>
    <date>2026-01-02</date>
  </metadata>

  <overview>
    Jedusor is a stylus-first AI interaction layer for reMarkable tablets. Write questions, annotations, or commands by hand—the AI responds directly on the e-ink display. Works as a blank journal (magical diary mode) or overlaid on PDFs/documents for interactive reading and research.
  </overview>

  <problem-statement>
    Reading and annotating PDFs on e-ink tablets is passive. Users highlight, scribble notes, but can't ask questions about content or get explanations without switching devices. Existing AI chat interfaces require typing and break the pen-and-paper flow. There's no way to have a conversation with your documents using handwriting.
  </problem-statement>

  <vision>
    <tagline>Talk to your papers with a pen.</tagline>
    <interaction-modes>
      <mode name="journal">Blank page conversation (Tom Riddle diary experience)</mode>
      <mode name="document">AI assistant overlaid on PDFs, responds to margin annotations</mode>
      <mode name="research">Multi-document context, cross-reference questions</mode>
    </interaction-modes>
  </vision>

  <target-users>
    <persona name="Researcher">
      <use-case>Annotate papers, ask clarifying questions in margins, get summaries</use-case>
    </persona>
    <persona name="Student">
      <use-case>Interactive textbook reading, explain concepts, quiz me</use-case>
    </persona>
    <persona name="Writer">
      <use-case>Creative brainstorming, character dialogue, world-building</use-case>
    </persona>
    <persona name="Professional">
      <use-case>Annotate contracts/reports, ask "what does this clause mean?"</use-case>
    </persona>
    <persona name="Harry Potter fan">
      <use-case>Magical diary experience</use-case>
    </persona>
  </target-users>

  <user-stories>
    <category name="Core Experience (All Modes)">
      <story id="US-01" priority="P0">As a user, I can write with the stylus and see it recognized as text</story>
      <story id="US-02" priority="P0">As a user, I see the AI response appear on the page</story>
      <story id="US-03" priority="P0">As a user, the conversation context persists within a session</story>
      <story id="US-04" priority="P1">As a user, I can switch between interaction modes</story>
      <story id="US-05" priority="P2">As a user, I can configure the AI persona/behavior</story>
    </category>

    <category name="Journal Mode (Blank Page)">
      <story id="US-10" priority="P0">As a user, I can have a freeform conversation on a blank page</story>
      <story id="US-11" priority="P1">As a user, AI responses appear with a "magical writing" animation</story>
      <story id="US-12" priority="P2">As a user, I can select persona presets (Riddle, tutor, assistant)</story>
      <story id="US-13" priority="P2">As a user, old exchanges fade/compact to make room for new ones</story>
    </category>

    <category name="Document Mode (PDF Interaction)">
      <story id="US-20" priority="P0">As a user, I can load a PDF and write annotations in margins</story>
      <story id="US-21" priority="P0">As a user, I can circle/underline text and write "explain this"</story>
      <story id="US-22" priority="P0">As a user, the AI sees the PDF content as context</story>
      <story id="US-23" priority="P0">As a user, AI responses appear in a designated area (margin/footer)</story>
      <story id="US-24" priority="P1">As a user, I can ask questions about specific pages or sections</story>
      <story id="US-25" priority="P1">As a user, I can request summaries of highlighted sections</story>
      <story id="US-26" priority="P2">As a user, annotations and AI responses are saved with the document</story>
    </category>

    <category name="Research Mode (Multi-Document)">
      <story id="US-30" priority="P2">As a user, I can reference multiple documents in context</story>
      <story id="US-31" priority="P2">As a user, I can ask cross-reference questions</story>
      <story id="US-32" priority="P3">As a user, I can build a knowledge base from my annotations</story>
    </category>

    <category name="Writing Experience">
      <story id="US-40" priority="P0">As a user, strokes render in real-time (&lt;16ms latency)</story>
      <story id="US-41" priority="P1">As a user, I can erase strokes with the pen's eraser end</story>
      <story id="US-42" priority="P0">As a user, handwriting recognition works for cursive and print</story>
      <story id="US-43" priority="P1">As a user, I can draw selection regions (circle, lasso)</story>
    </category>

    <category name="System">
      <story id="US-50" priority="P0">As a user, the app works with WiFi connectivity</story>
      <story id="US-51" priority="P1">As a user, I receive clear feedback when offline</story>
      <story id="US-52" priority="P0">As a user, battery drain is acceptable (&lt;10%/hour active)</story>
      <story id="US-53" priority="P0">As a user, I can exit cleanly with a gesture</story>
    </category>
  </user-stories>

  <functional-requirements>
    <requirement id="FR-1" name="Stylus Input Capture">
      <item>Capture Wacom digitizer events (position, pressure, tilt)</item>
      <item>Render strokes at &lt;16ms latency</item>
      <item>Support pen tip and eraser tool detection</item>
      <item>Detect stroke completion (pen lift) to trigger recognition</item>
      <item>Support gesture detection (circle, underline, lasso)</item>
    </requirement>

    <requirement id="FR-2" name="Handwriting Recognition">
      <item>Convert stroke data to text with &gt;90% accuracy</item>
      <item>Support English and French (P1: more languages)</item>
      <item>Process recognition within 500ms</item>
      <item>Handle cursive, print, and mixed styles</item>
      <item>Recognize command keywords ("explain", "summarize", "translate")</item>
    </requirement>

    <requirement id="FR-3" name="PDF Integration">
      <item>Load and render PDF documents</item>
      <item>Extract text content for LLM context</item>
      <item>Detect spatial relationship between annotation and PDF content</item>
      <item>Support page navigation</item>
      <item>Preserve PDF rendering quality on e-ink</item>
    </requirement>

    <requirement id="FR-4" name="LLM Integration">
      <item>Send recognized text + document context to Claude API</item>
      <item>Support long context (200K tokens for large documents)</item>
      <item>Implement multiple persona modes via system prompts</item>
      <item>Stream responses for progressive display</item>
      <item>Handle API errors gracefully</item>
    </requirement>

    <requirement id="FR-5" name="Response Rendering">
      <item>Display AI text in configurable typography</item>
      <item>Journal mode: handwriting-style font with animation</item>
      <item>Document mode: clean sans-serif in designated zones</item>
      <item>Support word wrapping within bounds</item>
      <item>Implement appropriate e-ink refresh strategy</item>
    </requirement>

    <requirement id="FR-6" name="Context Management">
      <item>Maintain conversation history during session</item>
      <item>Include relevant document sections in context</item>
      <item>Implement smart context windowing for long documents</item>
      <item>Clear/reset conversation on user action</item>
    </requirement>
  </functional-requirements>

  <interaction-patterns>
    <pattern name="Journal Mode">
      <description>
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
      </description>
    </pattern>

    <pattern name="Document Mode">
      <description>
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
      </description>
    </pattern>
  </interaction-patterns>

  <non-functional-requirements>
    <performance>
      <metric name="Stroke rendering latency" target="&lt;16ms"/>
      <metric name="Recognition latency" target="&lt;500ms"/>
      <metric name="PDF page render" target="&lt;1s"/>
      <metric name="LLM response start" target="&lt;2s (network)"/>
      <metric name="CPU usage (idle)" target="&lt;1%"/>
      <metric name="CPU usage (active)" target="&lt;10%"/>
      <metric name="Memory (journal)" target="&lt;50MB"/>
      <metric name="Memory (PDF loaded)" target="&lt;150MB"/>
    </performance>

    <reliability>
      <item>Graceful offline degradation (view PDF, save annotations locally)</item>
      <item>No data loss on unexpected termination</item>
      <item>Recovery from API failures without crash</item>
      <item>PDF rendering stability</item>
    </reliability>

    <security>
      <item>API key stored in user-readable config only</item>
      <item>Document content sent only to configured LLM API</item>
      <item>No persistent logging of document content</item>
      <item>Local annotation storage</item>
    </security>
  </non-functional-requirements>

  <out-of-scope version="1.0">
    <item>Offline LLM inference</item>
    <item>EPUB/other document formats</item>
    <item>Voice input/output</item>
    <item>Handwriting-to-LaTeX conversion</item>
    <item>Cloud sync of annotations</item>
    <item>Multi-user collaboration</item>
    <item>reMarkable Paper Pro color features</item>
  </out-of-scope>

  <phased-delivery>
    <phase number="1" name="Journal Mode (MVP)">
      <deliverable>Blank page conversation</deliverable>
      <deliverable>Basic HWR + LLM integration</deliverable>
      <deliverable>Single persona (assistant)</deliverable>
    </phase>

    <phase number="2" name="Document Mode">
      <deliverable>PDF loading and rendering</deliverable>
      <deliverable>Margin annotation detection</deliverable>
      <deliverable>Document context in LLM</deliverable>
    </phase>

    <phase number="3" name="Enhanced Experience">
      <deliverable>Multiple personas</deliverable>
      <deliverable>Animation effects</deliverable>
      <deliverable>Gesture commands</deliverable>
      <deliverable>Annotation persistence</deliverable>
    </phase>

    <phase number="4" name="Research Mode">
      <deliverable>Multi-document context</deliverable>
      <deliverable>Cross-reference queries</deliverable>
      <deliverable>Knowledge base features</deliverable>
    </phase>

    <phase number="5" name="Note-Taking Mode">
      <description>Split-screen layout: journal (top 1/4) + writing area (bottom 3/4)</description>
      <key-features>
        <feature>Journal Area (Top 1/4): Displays recognized text chronologically</feature>
        <feature>Writing Area (Bottom 3/4): Active handwriting input zone</feature>
        <feature>Auto-Recognition: Triggers after 1.5s of inactivity (no circle gesture needed)</feature>
        <feature>Endpoint Detection: Analyzes spatial and temporal coherence of strokes</feature>
        <feature>Fade Animation: Handwriting smoothly fades into text (600ms dithering transition)</feature>
        <feature>Stroke Removal: Writing area clears automatically after recognition</feature>
        <feature>Gesture-based Editing: Strike through text to edit or replace</feature>
        <feature>Content Detection: Distinguish between text and sketches/diagrams</feature>
        <feature>Persistent Context: Journal maintains conversation history</feature>
      </key-features>
      <use-cases>
        <use-case>Meeting notes with AI assistance</use-case>
        <use-case>Brainstorming with inline editing</use-case>
        <use-case>Personal diary with searchable text</use-case>
        <use-case>Quick notes with gesture-based corrections</use-case>
      </use-cases>
    </phase>
  </phased-delivery>

  <success-metrics>
    <metric name="Recognition accuracy" target="&gt;90% legible writing"/>
    <metric name="End-to-end response" target="&lt;5s (network dependent)"/>
    <metric name="Session stability" target="No crashes in 1-hour sessions"/>
    <metric name="PDF render fidelity" target="Readable at native zoom"/>
    <metric name="Battery impact" target="&lt;10% per hour active"/>
  </success-metrics>

  <development-tools>
    <tool name="macOS Simulator">
      <purpose>
        - Local development and testing without reMarkable hardware
        - Visual validation of stroke capture and gesture detection
        - Rapid iteration on rendering and layout
        - Lower barrier to entry for contributors
      </purpose>

      <implementation>
        - Window-based simulator using minifb crate
        - 1404x1872 display matching reMarkable 2 resolution
        - Mouse input converted to simulated Wacom events
        - Shared rendering pipeline with device code via feature flags
      </implementation>

      <capabilities>
        <capability status="supported">Stroke capture and real-time rendering</capability>
        <capability status="supported">Gesture detection (circle, underline, lasso)</capability>
        <capability status="supported">Handwriting recognition API integration</capability>
        <capability status="supported">LLM interaction and response display</capability>
        <capability status="supported">Text layout and typography testing</capability>
        <capability status="supported">Screenshot capture (press 'S' key to save PNG)</capability>
        <capability status="device-only">E-ink refresh mode behavior</capability>
        <capability status="device-only">Pressure sensitivity (mouse lacks pressure data)</capability>
        <capability status="device-only">ARM performance characteristics</capability>
      </capabilities>

      <platform-parity-requirement criticality="CRITICAL">
        <principle>
          The macOS simulator and reMarkable device version MUST ALWAYS be kept in sync.
          They are the SAME APPLICATION with different platform backends, not separate implementations.
        </principle>

        <requirements>
          <requirement status="mandatory">Feature Parity: Every feature implemented on device must work on simulator</requirement>
          <requirement status="mandatory">Shared Codebase: Maximum code sharing via traits and feature flags</requirement>
          <requirement status="mandatory">Unified Testing: Features validated on simulator before device deployment</requirement>
          <requirement status="mandatory">Consistent Behavior: Logic, gestures, recognition flow identical on both</requirement>
          <requirement status="forbidden">No Drift: Platform-specific implementations only for I/O layer</requirement>
          <requirement status="forbidden">No Duplication: Avoid separate codepaths for same functionality</requirement>
        </requirements>

        <development-workflow>
          <step>1. Implement feature with shared abstractions (traits)</step>
          <step>2. Test thoroughly on macOS simulator</step>
          <step>3. Deploy to device and verify behavior</step>
          <step>4. If device-specific adjustments needed, update simulator to match</step>
        </development-workflow>

        <rationale>
          - Simulator is primary development environment (faster iteration)
          - If simulator diverges, it becomes useless for validation
          - Device deployment is expensive (cross-compile + SSH)
          - Contributors without devices depend on simulator accuracy
        </rationale>
      </platform-parity-requirement>

      <usage>
        <command platform="simulator">cargo run --no-default-features --features simulator</command>
        <command platform="device">cross build --release --target armv7-unknown-linux-gnueabihf</command>
      </usage>

      <limitations>
        <limitation>Simulator validates logic but not e-ink performance</limitation>
        <limitation>Device testing required before production deployment</limitation>
        <limitation>Some device-specific bugs may only appear on hardware</limitation>
      </limitations>

      <reference>See ADR-008: macOS Simulator for architectural decision details</reference>
    </tool>
  </development-tools>

  <installation-deployment>
    <development-deployment method="SSH">
      <description>For testing and development, deploy via SSH</description>
      <steps>
        <step>Build release binary: cargo zigbuild --release --target armv7-unknown-linux-gnueabihf</step>
        <step>Deploy to device: scp target/armv7-unknown-linux-gnueabihf/release/jedusor root@10.11.99.1:/home/root/</step>
        <step>Run via SSH: ssh root@10.11.99.1 ./start-jedusor.sh</step>
      </steps>
    </development-deployment>

    <production-deployment method="Launcher Integration">
      <description>For daily use, integrate with a launcher via Toltec package manager</description>

      <setup>
        <step name="Install Toltec">
          ssh root@10.11.99.1
          wget https://toltec-dev.org/bootstrap
          bash bootstrap
        </step>

        <step name="Install a Launcher">
          <option name="Remux" recommended="true">
            <description>Simple app launcher in reMarkable menu - lightweight, minimal overhead, stable</description>
            <command>opkg install remux &amp;&amp; systemctl enable --now remux</command>
          </option>
          <option name="Oxide">
            <description>Complete desktop environment - more features but heavier resource usage</description>
            <command>opkg install oxide &amp;&amp; systemctl enable --now tarnish</command>
          </option>
        </step>

        <step name="App Discovery">
          - Launchers automatically discover apps in /opt/bin/ or via .draft files
          - Jedusor appears in launcher menu - tap to run
          - Launchers handle stopping xochitl automatically (exclusive mode)
        </step>
      </setup>

      <benefits>
        <benefit>No SSH needed for daily use</benefit>
        <benefit>Tap-to-run from device UI</benefit>
        <benefit>Automatic exclusive mode handling</benefit>
        <benefit>Better user experience</benefit>
      </benefits>

      <reference>See DEPLOY.md for detailed deployment instructions and troubleshooting</reference>
    </production-deployment>
  </installation-deployment>

  <dependencies>
    <dependency name="libremarkable" purpose="Device I/O" risk="Medium - community"/>
    <dependency name="minifb" purpose="macOS/desktop simulator" risk="Low - stable"/>
    <dependency name="Google Input Tools" purpose="HWR" risk="Low - stable"/>
    <dependency name="Claude API" purpose="LLM" risk="Low - commercial"/>
    <dependency name="pdf-rs or similar" purpose="PDF parsing" risk="Low - mature"/>
    <dependency name="reMarkable device" purpose="Hardware" risk="Low"/>
  </dependencies>

  <open-questions>
    <question>How to handle very long PDFs (100+ pages) in context?</question>
    <question>Should AI responses be saved into the PDF or separate sidecar?</question>
    <question>Gesture vocabulary: what triggers AI vs. regular annotation?</question>
    <question>How to visually distinguish user writing from AI responses?</question>
    <question>Support reMarkable 1, rM2, or Paper Pro only?</question>
  </open-questions>
</product-requirements-document>
