# ADR-009: Note-Taking Mode with Gesture-Based Editing

**Date**: 2026-01-04
**Status**: Proposed
**Epic**: note-taking-mode (future)

## Context

Journal Mode (Phase 1) provides a blank-page AI conversation experience, but lacks structured note-taking capabilities. Users need a way to:
- Keep persistent notes while writing
- Edit recognized text without starting over
- Distinguish between text and drawings
- Maintain conversation context visually on screen

This ADR proposes a Note-Taking Mode that combines persistent text display with gesture-based editing.

## Decision

Implement a split-screen note-taking interface with gesture-based text editing:

### Screen Layout

```
┌─────────────────────────────────────┐
│  JOURNAL AREA (Top 1/4 - 468px)    │
│  ─────────────────────────────────  │
│  Recognized Text:                   │
│  "What is the capital of France?"   │
│  "Paris is the capital..."          │
│  "Tell me more about Paris"         │
├─────────────────────────────────────┤
│                                     │
│  WRITING AREA (Bottom 3/4)          │
│                                     │
│  [Active handwriting input zone]    │
│  [User draws/writes here]           │
│                                     │
│                                     │
│                                     │
└─────────────────────────────────────┘
```

### Core Features

**1. Automatic Content Detection**
- **Text Recognition**: Strokes processed through Google Input Tools API
- **Drawing Detection**: Strokes that fail text recognition treated as drawings
- **Hybrid Support**: Allow both text and sketches in the same session

**2. Journal Display (Top 1/4)**
- Fixed area: 1404 x 468px (top quarter of 1872px height)
- Auto-scrolling text display
- Chronological order (newest at bottom)
- Read-only display with gesture editing enabled

**3. Writing Area (Bottom 3/4)**
- Active input zone: 1404 x 1404px
- Real-time stroke rendering
- Clears after recognition or drawing saved
- Full gesture support (circle, underline, strike-through)

**4. Gesture-Based Text Editing**

**Strike-Through Detection:**
- Horizontal stroke crossing existing text
- Triggers edit mode for that text segment
- Shows overlay with original text
- Prompts for replacement text

**Implementation:**
```rust
pub enum EditGesture {
    StrikeThrough {
        affected_text: String,
        text_bounds: Rect,
        stroke: Stroke,
    },
    Replace {
        original: String,
        replacement: String,
        position: usize,
    },
}

impl GestureDetector {
    pub fn detect_edit_gesture(&self, stroke: &Stroke, journal_text: &[(String, Rect)]) -> Option<EditGesture> {
        // Check if stroke intersects any text bounds
        // Verify horizontal linearity (like underline detection)
        // Return StrikeThrough gesture with affected text
    }
}
```

**5. Text vs. Drawing Classification**

```rust
pub struct ContentClassifier {
    recognition_threshold: f32,
}

impl ContentClassifier {
    pub async fn classify(&self, strokes: &[Stroke]) -> ContentType {
        match recognize(strokes).await {
            Ok(result) if result.confidence > self.recognition_threshold => {
                ContentType::Text(result.text)
            }
            _ => ContentType::Drawing(strokes.to_vec())
        }
    }
}

pub enum ContentType {
    Text(String),
    Drawing(Vec<Stroke>),
}
```

## Architecture

### Component Breakdown

**1. Journal Manager** (`src/journal/`)
```rust
pub struct Journal {
    entries: Vec<JournalEntry>,
    display_area: Rect,
    scroll_offset: i32,
}

pub struct JournalEntry {
    content: String,
    timestamp: SystemTime,
    bounds: Rect,  // For gesture detection
    editable: bool,
}

impl Journal {
    pub fn add_entry(&mut self, text: String) {
        // Add to entries
        // Auto-scroll if needed
        // Render to display area
    }

    pub fn find_entry_at(&self, point: Point) -> Option<&JournalEntry> {
        // For strike-through detection
    }

    pub fn replace_entry(&mut self, index: usize, new_text: String) {
        // For editing
    }
}
```

**2. Content Classifier** (`src/input/classifier.rs`)
```rust
pub struct ContentClassifier {
    recognizer: Arc<dyn HandwritingRecognizer>,
    min_text_confidence: f32,
    min_stroke_count: usize,
}

impl ContentClassifier {
    pub async fn classify(&self, strokes: &[Stroke]) -> ContentType {
        // Attempt recognition
        // Check confidence threshold
        // Return Text or Drawing
    }
}
```

**3. Edit Gesture Detector** (`src/input/edit_gesture.rs`)
```rust
pub struct EditGestureDetector {
    strike_linearity_threshold: f32,
}

impl EditGestureDetector {
    pub fn detect(&self, stroke: &Stroke, journal: &Journal) -> Option<EditGesture> {
        // Check if stroke is horizontal
        // Check intersection with journal text
        // Return StrikeThrough gesture
    }
}
```

**4. End-of-Writing Detector** (`src/input/writing_detector.rs`)

Automatically detects when the user has finished writing to trigger recognition without explicit gestures.

```rust
pub struct WritingDetector {
    /// Time since last stroke to consider writing finished
    idle_timeout: Duration,
    /// Timestamp of last stroke
    last_stroke_time: Option<Instant>,
    /// Timer handle for timeout
    timer: Option<Timer>,
}

impl WritingDetector {
    pub fn new(idle_timeout: Duration) -> Self {
        Self {
            idle_timeout,
            last_stroke_time: None,
            timer: None,
        }
    }

    /// Called when a new stroke is added
    pub fn on_stroke_added(&mut self) {
        self.last_stroke_time = Some(Instant::now());
        self.reset_timer();
    }

    /// Check if writing has ended (timeout exceeded)
    pub fn has_writing_ended(&self) -> bool {
        if let Some(last_time) = self.last_stroke_time {
            Instant::now().duration_since(last_time) >= self.idle_timeout
        } else {
            false
        }
    }

    /// Reset the idle timer
    fn reset_timer(&mut self) {
        // Cancel existing timer
        // Start new timer for idle_timeout duration
    }
}

/// Analyzes stroke endpoints to determine writing completion
pub struct EndpointDetector {
    /// Minimum distance between start and end to consider complete
    min_closure_distance: f32,
}

impl EndpointDetector {
    /// Detect if a stroke sequence represents a complete thought/word
    pub fn is_complete_segment(&self, strokes: &[Stroke]) -> bool {
        if strokes.is_empty() {
            return false;
        }

        // Check if strokes form a cohesive group
        let spatial_coherence = self.analyze_spatial_coherence(strokes);
        let temporal_coherence = self.analyze_temporal_coherence(strokes);

        spatial_coherence && temporal_coherence
    }

    /// Check if strokes are spatially close (same word/phrase)
    fn analyze_spatial_coherence(&self, strokes: &[Stroke]) -> bool {
        // Calculate bounding box of all strokes
        let bbox = calculate_bounding_box(strokes);

        // Check if strokes are within reasonable horizontal span
        // Typical word width: 200-800px
        let width = bbox.width();
        width > 50.0 && width < 1000.0
    }

    /// Check if strokes have reasonable timing (not too spread out)
    fn analyze_temporal_coherence(&self, strokes: &[Stroke]) -> bool {
        if strokes.len() < 2 {
            return true;
        }

        // Check gaps between consecutive strokes
        for window in strokes.windows(2) {
            let gap = window[1].start_time - window[0].end_time;
            // Gap > 2 seconds suggests separate writing segments
            if gap > Duration::from_secs(2) {
                return false;
            }
        }
        true
    }
}
```

**5. Layout Manager** (`src/layout/`)
```rust
pub struct SplitLayout {
    journal_area: Rect,    // Top 1/4
    writing_area: Rect,    // Bottom 3/4
}

impl SplitLayout {
    pub fn new(screen_width: u32, screen_height: u32) -> Self {
        let journal_height = screen_height / 4;
        Self {
            journal_area: Rect::new(0, 0, screen_width, journal_height),
            writing_area: Rect::new(0, journal_height, screen_width, screen_height - journal_height),
        }
    }

    pub fn contains_journal(&self, point: Point) -> bool {
        self.journal_area.contains(point)
    }
}
```

## Event Flow

### Normal Writing Flow (Timer-Based)

```
User writes in writing area
    ↓
Stroke completed (pen lift)
    ↓
WritingDetector.on_stroke_added()
    ↓
Reset idle timer (1.5 seconds)
    ↓
User continues writing... (timer resets with each stroke)
    ↓
User stops writing (no new strokes)
    ↓
Idle timeout reached (1.5 seconds since last stroke)
    ↓
EndpointDetector.is_complete_segment()
    ↓
[If complete] Trigger recognition
    ↓
Content Classifier: Text or Drawing?
    ↓
┌─────────────┬──────────────────┐
│ Text        │ Drawing          │
│ - Recognize │ - Save strokes   │
│ - Add to    │ - Add to drawing │
│   journal   │   collection     │
│ - Clear     │ - Clear writing  │
│   writing   │   area           │
│   area      │                  │
└─────────────┴──────────────────┘
```

### Edit Flow (Strike-Through)

```
User strikes through journal text
    ↓
Edit Gesture Detected
    ↓
Show edit overlay
    ↓
Capture replacement text (with timer detection)
    ↓
Update journal entry
    ↓
Refresh journal display
```

## Technical Decisions

### 1. Journal Storage

**Decision**: In-memory with optional persistence

**Rationale**:
- Session-based notes don't require immediate disk writes
- Simplifies implementation for MVP
- Can add SQLite persistence later

```rust
pub struct JournalStorage {
    entries: Vec<JournalEntry>,
    persistence: Option<Box<dyn PersistenceBackend>>,
}
```

### 2. Text Rendering in Journal

**Decision**: Use libremarkable's `draw_text` with line wrapping

**Rationale**:
- Proven rendering for Journal Mode
- Handles e-ink refresh efficiently
- Supports bounding box calculation for gestures

**Implementation**:
```rust
impl Journal {
    pub fn render(&self, fb: &mut Framebuffer) {
        for (y_offset, entry) in self.visible_entries() {
            let bounds = text_renderer.draw_text(
                fb,
                &entry.content,
                50,  // x margin
                self.display_area.y + y_offset,
                40.0,  // font size
            );
            // Store bounds for gesture detection
        }
    }
}
```

### 3. Strike-Through Detection

**Decision**: Extend existing gesture detector with text intersection

**Rationale**:
- Reuse gesture detection infrastructure
- Similar to underline detection (horizontal linearity)
- Add intersection check with journal text bounds

**Algorithm**:
```rust
fn is_strike_through(stroke: &Stroke, text_bounds: &Rect) -> bool {
    // 1. Check horizontal linearity (like underline)
    let linearity = calculate_horizontal_linearity(stroke);
    if linearity < LINEARITY_THRESHOLD {
        return false;
    }

    // 2. Check if stroke intersects text bounds
    let stroke_bounds = stroke.bounding_box();
    if !stroke_bounds.intersects(text_bounds) {
        return false;
    }

    // 3. Check if stroke crosses through (not just touches)
    let cross_points = count_intersections(stroke, text_bounds);
    cross_points >= 2  // Enter and exit
}
```

### 4. Edit Mode UI

**Decision**: Overlay with simple prompt

**Rationale**:
- E-ink requires minimal UI changes
- Clear visual feedback
- Reuse existing text input flow

**Flow**:
```
Strike-through detected
    ↓
Highlight affected text
    ↓
Show prompt: "Replace with:"
    ↓
User writes replacement in writing area
    ↓
Circle gesture or timeout triggers replacement
    ↓
Update journal, clear overlay
```

### 5. End-of-Writing Detection

**Decision**: Timer-based detection with endpoint analysis

**Rationale**:
- Eliminates need for explicit gestures (circle) for every text entry
- More natural note-taking flow
- Balances responsiveness with avoiding premature recognition

**Parameters**:
```rust
const IDLE_TIMEOUT: Duration = Duration::from_millis(1500);  // 1.5 seconds
const MAX_STROKE_GAP: Duration = Duration::from_secs(2);     // 2 seconds
const MIN_SPATIAL_WIDTH: f32 = 50.0;    // Minimum stroke width to recognize
const MAX_SPATIAL_WIDTH: f32 = 1000.0;  // Maximum word width
```

**Algorithm**:
1. **Timer Reset**: Each stroke resets the idle timer
2. **Timeout Trigger**: After 1.5s of inactivity, check for completion
3. **Endpoint Analysis**:
   - Spatial coherence: Are strokes close together? (same word/phrase)
   - Temporal coherence: Are stroke gaps reasonable? (<2s between strokes)
4. **Recognition Trigger**: If complete segment detected, recognize and add to journal

**Trade-offs**:

| Aspect | Short Timeout (0.5s) | Medium Timeout (1.5s) | Long Timeout (3.0s) |
|--------|---------------------|----------------------|-------------------|
| Responsiveness | Fast (500ms) | Balanced (1500ms) | Slow (3000ms) |
| False triggers | High | Medium | Low |
| User experience | Jarring | Natural | Sluggish |
| Battery impact | Higher (more API calls) | Moderate | Lower |

**Selected**: 1.5 seconds (medium timeout) for balanced UX

**Alternative Approaches Considered**:

1. **Gesture-only** (Circle to trigger)
   - Pro: Explicit control, no false triggers
   - Con: Tedious for every entry, breaks flow

2. **Fixed delay per stroke** (recognize after every 3 strokes)
   - Pro: Predictable behavior
   - Con: Breaks multi-stroke words, arbitrary

3. **Machine learning-based** (predict completion)
   - Pro: Adaptive to user patterns
   - Con: Complexity, training data required, latency

**Fallback Mechanisms**:
- User can still trigger recognition manually with circle gesture
- Configurable timeout in settings (0.5s - 3.0s range)
- Disable auto-recognition mode for drawing-heavy sessions

## Performance Considerations

**Journal Scrolling**:
- Partial refresh for scroll updates
- Full refresh only on text add/edit
- Cache rendered text bounds

**Content Classification**:
- Async recognition (non-blocking)
- Timeout for drawing classification (500ms)
- Cache results per stroke set

**Memory Management**:
- Limit journal entries (e.g., 100 most recent)
- Archive older entries to disk
- Clear writing area strokes after recognition

## User Experience

### Workflow Example

**Meeting Notes:**
```
User writes: "Q: Budget for 2025?"
→ Appears in journal

User writes: "A: $2.5M approved"
→ Appears in journal

User strikes through "$2.5M"
→ Edit mode activates

User writes: "$3.2M"
→ Journal updates to "A: $3.2M approved"

User draws org chart diagram
→ Stays in writing area as sketch
```

### Advantages

1. **Persistent Context**: All notes visible at top
2. **Natural Editing**: Strike-through gesture familiar from paper
3. **Flexible Input**: Mix text and drawings
4. **E-ink Optimized**: Minimal screen updates

### Limitations

1. **Limited Journal Space**: Top 1/4 holds ~8-10 lines
2. **Scrolling Required**: For longer sessions
3. **Text-Only Journal**: Drawings not displayed in journal
4. **Single Edit Target**: Can't strike through multiple entries simultaneously

## Open Questions

1. **Should drawings appear in journal as thumbnails?**
   - Pro: Complete note history
   - Con: Reduces text space, complex rendering

2. **How to handle multi-line strike-through?**
   - Option A: Highlight all crossed lines
   - Option B: Only nearest line

3. **Persistence strategy?**
   - Auto-save every N entries?
   - Manual save gesture?
   - Session-based files?

4. **AI Integration in Note-Taking Mode?**
   - Should AI respond to journal text?
   - Separate mode or toggle?
   - How to distinguish questions vs. notes?

## Success Metrics

| Metric | Target |
|--------|--------|
| Journal text capacity | 8-10 lines visible |
| Strike-through accuracy | >85% detection rate |
| Edit gesture latency | <300ms to activate |
| Text classification accuracy | >90% for legible writing |
| Drawing classification recall | >95% (avoid misclassifying) |
| Session stability | No crashes in 30-min note sessions |

## Implementation Plan

### Milestone 1: Split Layout
- Implement SplitLayout with fixed areas
- Render journal and writing zones
- Basic stroke capture in writing area

### Milestone 2: Content Classification
- Integrate ContentClassifier
- Text → Journal, Drawing → Collection
- Clear writing area after classification

### Milestone 3: Gesture Editing
- Implement EditGestureDetector
- Strike-through detection
- Edit overlay UI

### Milestone 4: Journal Management
- Scrolling for long sessions
- Entry replacement
- Optional persistence

## References

- ADR-002: Handwriting Recognition (Google Input Tools)
- ADR-004: Device Framework (libremarkable)
- ADR-008: Journal Mode MVP Implementation
- PRD Phase 5: Note-Taking Mode
- UX Pattern: Paper-based strike-through editing

## Next Steps

1. Create proof-of-concept split layout
2. Test strike-through detection accuracy
3. Evaluate text vs. drawing classification performance
4. Design edit mode UI mockups
5. Gather user feedback on workflow
