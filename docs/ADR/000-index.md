<?xml version="1.0" encoding="UTF-8"?>
<architecture-decision-records>
  <description>
    This directory contains the Architecture Decision Records (ADRs) for the Jedusor project.
  </description>

  <index>
    <adr number="001" title="Programming Language" status="Accepted" file="001-programming-language.xml">
      <summary>Rust selected over C++, Python, Go, JavaScript</summary>
    </adr>
    <adr number="002" title="Handwriting Recognition" status="Accepted" file="002-handwriting-recognition.xml">
      <summary>Google Input Tools API selected over Tesseract, on-device ML, Mathpix</summary>
    </adr>
    <adr number="003" title="LLM Provider" status="Accepted" file="003-llm-provider.xml">
      <summary>Claude (Anthropic) selected over GPT-4, local LLM, Gemini</summary>
    </adr>
    <adr number="004" title="Device Framework" status="Accepted" file="004-device-framework.xml">
      <summary>libremarkable selected over direct evdev, Qt Quick, Toltec</summary>
    </adr>
    <adr number="005" title="Response Animation" status="Accepted" file="005-response-animation.xml">
      <summary>Character/word streaming selected over fade-in, all-at-once</summary>
    </adr>
    <adr number="006" title="Existing Projects Analysis" status="Informational" file="006-existing-projects-analysis.xml">
      <summary>Gap analysis of reMarkableAI, armrest, whiteboard-hypercard, ScribbleGPT</summary>
    </adr>
    <adr number="007" title="PDF Integration" status="Accepted" file="007-pdf-integration.xml">
      <summary>lopdf + mupdf selected over Poppler, pdf.js, pure pdf-rs</summary>
    </adr>
    <adr number="008a" title="macOS Development Simulator" status="Accepted" file="008-macos-simulator.xml">
      <summary>GUI simulator with minifb selected over terminal-based, web-based</summary>
    </adr>
    <adr number="008b" title="Journal Mode MVP Implementation" status="Implemented" file="008-journal-mode-mvp-implementation.xml">
      <summary>Documents stroke rendering, gesture detection, recognition integration, deployment</summary>
    </adr>
    <adr number="009" title="Note-Taking Mode" status="Proposed" file="009-note-taking-mode.xml">
      <summary>Split-screen layout with journal (top 1/4) + writing area (bottom 3/4), gesture-based editing</summary>
    </adr>
    <adr number="010" title="Platform Abstraction Architecture" status="Accepted" file="010-platform-abstraction.xml">
      <summary>Trait-based abstraction eliminates device/simulator code duplication, achieves 95% code sharing</summary>
    </adr>
  </index>

  <selected-stack>
    <component name="Language">Rust</component>
    <component name="Framework">libremarkable 0.7.x</component>
    <component name="Simulator">minifb (macOS/desktop)</component>
    <component name="Recognition">Google Input Tools API</component>
    <component name="LLM">Claude (Anthropic API)</component>
    <component name="Animation">Streaming + Partial GC16</component>
    <component name="PDF">lopdf + mupdf</component>
    <component name="Platform Abstraction">Display, InputSource, Platform traits</component>
  </selected-stack>

  <rejected-alternatives>
    <rejection category="Language" option="C++" reason="No high-level framework, more boilerplate"/>
    <rejection category="Language" option="Python" reason="No native device I/O, GC pauses"/>
    <rejection category="Language" option="Go" reason="No framebuffer framework, GC pauses"/>
    <rejection category="Language" option="JavaScript" reason="No native e-ink control, browser overhead"/>
    <rejection category="Recognition" option="Tesseract" reason="Poor handwriting accuracy (OCR focused)"/>
    <rejection category="Recognition" option="On-device ML" reason="Development risk, accuracy concerns"/>
    <rejection category="Recognition" option="Mathpix" reason="Cost, STEM specialization"/>
    <rejection category="LLM" option="GPT-4" reason="Higher cost, less persona consistency"/>
    <rejection category="LLM" option="Local LLM" reason="Hardware limitations (1GB RAM)"/>
    <rejection category="LLM" option="Gemini" reason="Weaker creative writing"/>
    <rejection category="Framework" option="Qt Quick" reason="No partial refresh control, heavyweight"/>
    <rejection category="Framework" option="Direct evdev" reason="Reinventing solved problems"/>
    <rejection category="Animation" option="Fade-in" reason="E-ink doesn't support opacity"/>
    <rejection category="Animation" option="All-at-once" reason="Loses magical effect"/>
    <rejection category="PDF" option="Poppler" reason="Cross-compilation complexity, binary size"/>
    <rejection category="PDF" option="pdf.js" reason="Requires WebView, memory hungry"/>
    <rejection category="PDF" option="Pure pdf-rs" reason="Insufficient rendering quality"/>
    <rejection category="Simulator" option="Terminal-based" reason="No visual feedback, poor UX"/>
    <rejection category="Simulator" option="Web-based" reason="Over-engineered, separate codebase"/>
  </rejected-alternatives>

  <adr-template>
    <section name="Status">[Proposed | Accepted | Deprecated | Superseded by ADR-XXX]</section>
    <section name="Context">[Why is this decision needed?]</section>
    <section name="Decision">[What is the decision and brief rationale?]</section>
    <section name="Options Considered">[List all options with pros/cons]</section>
    <section name="Consequences">[What are the positive and negative results?]</section>
    <section name="Decision Criteria Matrix">[Weighted scoring if applicable]</section>
  </adr-template>

  <future-adrs>
    <planned-adr>ADR-011: Configuration management</planned-adr>
    <planned-adr>ADR-012: Error handling and offline behavior</planned-adr>
    <planned-adr>ADR-013: Testing strategy</planned-adr>
    <planned-adr>ADR-014: Multi-language support</planned-adr>
  </future-adrs>
</architecture-decision-records>
