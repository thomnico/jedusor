<?xml version="1.0" encoding="UTF-8"?>
<architecture-decision-record number="010">
  <title>Platform Abstraction Architecture</title>
  <status>Accepted</status>
  <date>2026-01-05</date>

  <context>
    <problem>
      Initial implementation had massive code duplication between device and simulator:
      - app.rs: 581 lines with ~60% duplicated business logic
      - "Circle gesture detected" appeared 3 times (duplicated!)
      - Two separate event loops with ~300 lines each
      - Gesture detection logic duplicated
      - Recognition logic duplicated 3 times
    </problem>

    <impact>
      - Maintenance burden: changes must be made in multiple places
      - Bug risk: fixes applied to one platform but not the other
      - Violates DRY principle
      - Platform parity was difficult to maintain
      - Simulator became unreliable for validation
    </impact>

    <requirement>
      CRITICAL Platform Parity Rule: The macOS simulator and reMarkable device version
      MUST ALWAYS be kept in sync. They are the SAME APPLICATION with different I/O backends.
    </requirement>
  </context>

  <decision>
    <summary>
      Implement trait-based platform abstraction to eliminate ALL code duplication between
      device and simulator, achieving ~95% code sharing.
    </summary>

    <rationale>
      - Business logic should be 100% shared (EventHandler)
      - Only I/O operations differ between platforms
      - Traits provide clean abstraction boundary
      - Enables testing shared logic without device
      - Maintains platform parity automatically
    </rationale>
  </decision>

  <architecture>
    <layer name="Platform Traits" path="src/platform/mod.rs">
      <trait name="Display">
        <purpose>Abstract rendering operations</purpose>
        <methods>
          <method>clear() - Clear display</method>
          <method>draw_line() - Draw line with width</method>
          <method>draw_text() - Render text</method>
          <method>refresh() - Full screen refresh</method>
          <method>refresh_region() - Partial refresh</method>
        </methods>
      </trait>

      <trait name="InputSource">
        <purpose>Abstract input polling</purpose>
        <methods>
          <method>poll_event() - Get next input event</method>
          <method>is_running() - Check if platform active</method>
        </methods>
      </trait>

      <trait name="Platform">
        <purpose>Combine display + input for complete platform</purpose>
        <methods>
          <method>new() - Create platform instance</method>
          <method>display() - Get display reference</method>
          <method>input() - Get input reference</method>
        </methods>
      </trait>
    </layer>

    <layer name="Shared Business Logic" path="src/app/event_handler.rs">
      <component name="EventHandler">
        <purpose>ALL gesture detection, recognition, and response logic (100% shared)</purpose>
        <responsibility>Stroke processing</responsibility>
        <responsibility>Gesture detection (circle, underline, lasso)</responsibility>
        <responsibility>Handwriting recognition triggering</responsibility>
        <responsibility>AI response display</responsibility>
        <responsibility>Context management</responsibility>
      </component>
    </layer>

    <layer name="Platform Implementations">
      <implementation name="Simulator" path="src/platform/simulator.rs">
        <purpose>minifb window with mouse-to-Wacom conversion</purpose>
        <lines>~100 lines (I/O only)</lines>
        <components>
          <component>SimulatorDisplay - Wraps SimulatorWindow</component>
          <component>SimulatorInput - Mouse event polling</component>
          <component>SimulatorPlatform - Combines display + input</component>
        </components>
      </implementation>

      <implementation name="Device" path="src/platform/device.rs">
        <purpose>libremarkable framebuffer adapter</purpose>
        <lines>~100 lines (I/O only)</lines>
        <components>
          <component>DeviceDisplay - Wraps libremarkable framebuffer</component>
          <component>Device event loop uses libremarkable callback</component>
        </components>
      </implementation>
    </layer>

    <layer name="Event Loops" path="src/app/mod.rs">
      <loop name="Simulator">
        <purpose>Polling-based event loop</purpose>
        <lines>~30 lines</lines>
        <responsibilities>
          <item>Create SimulatorPlatform</item>
          <item>Poll events</item>
          <item>Call EventHandler.handle_event()</item>
        </responsibilities>
      </loop>

      <loop name="Device">
        <purpose>Callback-based event loop (libremarkable)</purpose>
        <lines>~40 lines</lines>
        <responsibilities>
          <item>Convert libremarkable events to WacomEvent</item>
          <item>Create DeviceDisplay adapter</item>
          <item>Call EventHandler.handle_event()</item>
        </responsibilities>
      </loop>
    </layer>
  </architecture>

  <results>
    <metrics>
      <metric name="Lines of Code">
        <before>app.rs: 581 lines</before>
        <after>app/mod.rs: 281 lines (300 lines eliminated)</after>
      </metric>

      <metric name="Code Duplication">
        <before>"Circle gesture detected": 3 occurrences</before>
        <after>"Circle gesture detected": 1 occurrence (in EventHandler)</after>
      </metric>

      <metric name="Code Sharing">
        <before>~40% shared, 60% duplicated</before>
        <after>~95% shared, 5% platform-specific (I/O only)</after>
      </metric>

      <metric name="Event Loop Size">
        <before>Device: ~300 lines, Simulator: ~150 lines</before>
        <after>Device: ~40 lines, Simulator: ~30 lines</after>
      </metric>
    </metrics>

    <verification status="successful">
      <test>Simulator builds successfully</test>
      <test>Simulator runs correctly</test>
      <test>Circle gesture recognition works</test>
      <test>All strokes and gestures detected</test>
      <test>All logs show shared code path: jedusor::app::event_handler</test>
      <test>grep "Circle gesture detected" returns 1 result (was 3)</test>
    </verification>
  </results>

  <consequences>
    <positive>
      <item>Zero code duplication in business logic</item>
      <item>Platform parity maintained automatically</item>
      <item>Changes only need to be made once</item>
      <item>Bugs fixed in one place affect both platforms</item>
      <item>Easier to add new features</item>
      <item>Simulator is now reliable for validation</item>
      <item>Contributors can develop without device</item>
      <item>Clean separation of concerns (I/O vs logic)</item>
    </positive>

    <negative>
      <item>Slight increase in abstraction complexity</item>
      <item>Device loop constrained by libremarkable callback API</item>
      <item>RefCell used in simulator for shared window access</item>
    </negative>

    <trade-offs>
      <item>More trait bounds in signatures (acceptable for type safety)</item>
      <item>Device loop still callback-based (required by libremarkable)</item>
    </trade-offs>
  </consequences>

  <implementation-notes>
    <note>
      EventHandler owns WacomHandler, GestureDetector, GoogleRecognizer - all shared state
    </note>
    <note>
      Platform implementations only handle I/O conversion (events to WacomEvent, rendering to display)
    </note>
    <note>
      Device uses DeviceDisplay adapter created inside libremarkable callback
    </note>
    <note>
      Simulator uses Rc&lt;RefCell&lt;SimulatorWindow&gt;&gt; for shared access
    </note>
  </implementation-notes>

  <future-considerations>
    <item>Add Platform::screenshot() method for cross-platform screenshots</item>
    <item>Consider abstracting libremarkable callback into polling interface</item>
    <item>May add Platform::Button for physical button handling</item>
  </future-considerations>

  <related-adrs>
    <related-adr number="008">macOS Simulator - established platform parity requirement</related-adr>
    <related-adr number="004">Device Framework - established libremarkable dependency</related-adr>
  </related-adrs>
</architecture-decision-record>
