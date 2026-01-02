# ADR-007: PDF Integration Approach

## Status

**Accepted**

## Context

Jedusor's Document Mode requires loading, rendering, and extracting content from PDF documents to enable AI-assisted reading. The solution must:

- Render PDFs legibly on e-ink display
- Extract text for LLM context
- Detect spatial relationships between annotations and PDF content
- Handle large academic papers (100+ pages)
- Work within reMarkable's memory constraints (~1GB RAM)

## Decision

**Use a hybrid approach**: `pdf-rs` (lopdf) for text extraction + `mupdf` bindings for rendering, with page-level context windowing.

## Options Considered

### Option 1: pdf-rs (lopdf) + mupdf (Selected)

**Implementation:**

- `lopdf` for PDF parsing and text extraction (pure Rust)
- `mupdf-rs` bindings for high-quality page rendering
- Extract text per-page, send relevant pages to LLM

**Pros:**

- Pure Rust text extraction (no external dependencies)
- mupdf is battle-tested rendering engine
- Memory-efficient page-at-a-time processing
- Good text extraction quality

**Cons:**

- Two libraries to maintain
- mupdf requires C bindings
- Complex PDFs may have extraction issues

**Evaluation:** Best balance of Rust ecosystem integration and rendering quality.

### Option 2: Poppler (Rejected)

**Pros:**

- Industry-standard PDF rendering
- Excellent text extraction
- Handles complex PDFs well

**Cons:**

- Heavy C++ dependency
- Complex cross-compilation for ARM
- Large binary size
- Overkill for e-ink rendering needs

**Rejection Reason:** Cross-compilation complexity and binary size.

### Option 3: pdf.js via WebView (Rejected)

**Pros:**

- Excellent rendering fidelity
- Well-maintained
- Handles all PDF features

**Cons:**

- Requires JavaScript runtime/WebView
- Massive overhead for native app
- No e-ink optimization
- Memory hungry

**Rejection Reason:** Architectural mismatch with native Rust app.

### Option 4: Pure pdf-rs Rendering (Rejected)

**Pros:**

- Single library, pure Rust
- Simplest integration

**Cons:**

- Limited rendering capabilities
- No image/font embedding support
- Many PDFs won't render correctly

**Rejection Reason:** Insufficient rendering quality for real documents.

### Option 5: Reuse reMarkable's PDF Viewer (Rejected)

**Approach:** Hook into xochitl's PDF rendering

**Pros:**

- Already optimized for device
- Native e-ink refresh handling

**Cons:**

- Undocumented internal APIs
- Would require reverse engineering
- Fragile across firmware updates
- Can't overlay our UI

**Rejection Reason:** Not maintainable, depends on proprietary code.

### Option 6: pdfium (Chromium's PDF Engine) (Considered for Future)

**Pros:**

- High-quality rendering (Chrome/Edge use it)
- C API available
- Handles complex PDFs

**Cons:**

- Large binary (~10MB+)
- Complex build process
- Cross-compilation challenges

**Status:** Good fallback if mupdf proves insufficient.

## Consequences

### Positive

- Rust-native text extraction
- High-quality rendering via mupdf
- Page-level processing fits memory constraints
- Can extract text from specific regions

### Negative

- Two library dependencies
- C bindings for mupdf add build complexity
- Some complex PDFs may not extract cleanly

## Implementation Details

### Architecture

```
┌─────────────────────────────────────────┐
│              PDF Module                 │
├─────────────────────────────────────────┤
│  ┌─────────────┐    ┌────────────────┐  │
│  │   lopdf     │    │    mupdf-rs    │  │
│  │  (extract)  │    │   (render)     │  │
│  └──────┬──────┘    └───────┬────────┘  │
│         │                   │           │
│         ▼                   ▼           │
│  ┌─────────────┐    ┌────────────────┐  │
│  │ Page Text   │    │  Page Bitmap   │  │
│  │ + Positions │    │  (grayscale)   │  │
│  └──────┬──────┘    └───────┬────────┘  │
│         │                   │           │
│         ▼                   ▼           │
│  ┌─────────────────────────────────────┐│
│  │         Context Manager             ││
│  │  (page windowing, region mapping)   ││
│  └─────────────────────────────────────┘│
└─────────────────────────────────────────┘
```

### Text Extraction Strategy

```rust
struct PageContext {
    page_num: usize,
    full_text: String,
    text_blocks: Vec<TextBlock>,
}

struct TextBlock {
    text: String,
    bbox: Rect,  // For spatial annotation mapping
}

// Extract text with position info for annotation correlation
fn extract_page_context(doc: &Document, page: usize) -> PageContext {
    // Use lopdf to get text and positions
    // Map annotation coordinates to nearest text blocks
}
```

### Context Windowing for LLM

For large documents, send focused context:

```rust
enum ContextStrategy {
    // Send only current page
    CurrentPage,
    // Current + adjacent pages
    Window { before: usize, after: usize },
    // Pages user has annotated
    AnnotatedPages,
    // Semantic search for relevant sections
    SemanticSearch { query: String },
}

// Default: 3-page window around current position
const DEFAULT_WINDOW: ContextStrategy = ContextStrategy::Window {
    before: 1,
    after: 1,
};
```

### Annotation-to-Text Mapping

```rust
struct Annotation {
    strokes: Vec<Stroke>,
    bbox: Rect,
    recognized_text: String,
    annotation_type: AnnotationType,
}

enum AnnotationType {
    FreeText,           // Margin note
    CircledRegion,      // User circled PDF content
    Underline,          // User underlined text
    Highlight,          // User highlighted area
}

// Find PDF text near annotation
fn correlate_annotation(ann: &Annotation, page: &PageContext) -> Option<String> {
    page.text_blocks
        .iter()
        .filter(|block| block.bbox.intersects(&ann.bbox))
        .map(|block| block.text.clone())
        .collect()
}
```

### Memory Management

| Document Size | Strategy |
|---------------|----------|
| < 20 pages | Load all text, render on demand |
| 20-100 pages | Cache recent 5 pages rendered |
| > 100 pages | Single page in memory, LRU cache |

### E-Ink Rendering Considerations

- Render to grayscale bitmap (no color)
- Scale to screen DPI (226 for rM2)
- Use GC16 waveform for quality text
- Cache rendered pages to avoid re-rendering

## Decision Criteria Matrix

| Criteria | Weight | lopdf+mupdf | Poppler | pdf.js | Pure pdf-rs |
|----------|--------|-------------|---------|--------|-------------|
| Rendering quality | 30% | ✅ 9 | ✅ 10 | ✅ 10 | ❌ 4 |
| Text extraction | 25% | ✅ 8 | ✅ 9 | ✅ 8 | ⚠️ 6 |
| Build complexity | 20% | ⚠️ 6 | ❌ 3 | ❌ 2 | ✅ 10 |
| Memory efficiency | 15% | ✅ 8 | ⚠️ 6 | ❌ 3 | ✅ 9 |
| Binary size | 10% | ⚠️ 7 | ❌ 4 | ❌ 2 | ✅ 10 |
| **Weighted Score** | | **7.8** | **7.0** | **5.8** | **6.6** |

## Dependencies

```toml
[dependencies]
lopdf = "0.32"
mupdf = "0.4"
```

## References

- [lopdf](https://github.com/J-F-Liu/lopdf) - Pure Rust PDF library
- [mupdf-rs](https://github.com/nickolay/mupdf-rs) - MuPDF Rust bindings
- [MuPDF](https://mupdf.com/) - Lightweight PDF/XPS viewer
