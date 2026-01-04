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

**4. Layout Manager** (`src/layout/`)
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

```
User writes in writing area
    ↓
Stroke completed (pen lift)
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
    ↓
User strikes through journal text
    ↓
Edit Gesture Detected
    ↓
Show edit overlay
    ↓
Capture replacement text
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
