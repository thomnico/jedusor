# ADR-002: Handwriting Recognition Approach

## Status
**Accepted**

## Context

Jedusor must convert stylus strokes into text for LLM processing. The recognition must:
- Handle cursive, print, and mixed handwriting
- Achieve >90% accuracy for legible writing
- Complete within 500ms of stroke completion
- Work with various writing styles and speeds

## Decision

**Use Google Input Tools API** as the primary recognition backend, with architecture allowing future local alternatives.

## Options Considered

### Option 1: Google Input Tools API (Selected)

**Implementation:** Send stroke coordinates to `https://www.google.com/inputtools/request?ime=handwriting`

**Pros:**
- Proven accuracy (used by ScribbleGPT, diary.ycmjason.com)
- Handles cursive exceptionally well
- Fast response times (<200ms typical)
- No model download required
- Supports multiple languages
- Free (undocumented but stable API)

**Cons:**
- Requires internet connectivity
- Undocumented API (could change)
- Privacy: strokes sent to Google
- No offline fallback

**Evaluation:** Best accuracy-to-complexity ratio. Proven in similar projects.

### Option 2: Tesseract.js / Tesseract OCR (Rejected)

**Pros:**
- Open source, well-documented
- Offline capable
- Multiple language support

**Cons:**
- Designed for printed text, not handwriting
- Poor cursive recognition ("absolute trash for handwriting" - ycmjason)
- Requires image rendering of strokes first
- Higher latency

**Rejection Reason:** Fundamentally unsuited for handwriting recognition. Built for OCR of printed documents.

### Option 3: On-Device ML Model (Rejected for v1)

**Candidates:**
- TensorFlow Lite handwriting model
- Custom trained model
- ONNX Runtime inference

**Pros:**
- Fully offline
- Privacy preserving
- No API dependency

**Cons:**
- Significant development effort
- Model size (10-100MB+)
- Lower accuracy than Google without extensive training
- ARM optimization challenges
- No proven reMarkable implementation

**Rejection Reason:** Too much development risk for v1. Consider for future offline mode.

### Option 4: Mathpix API (Rejected)

**Pros:**
- Excellent for mathematical notation
- Good handwriting support
- Commercial API with SLA

**Cons:**
- Paid service ($10+/month)
- Overkill for text-only recognition
- Optimized for STEM content

**Rejection Reason:** Cost and specialization. Better suited for math-focused apps.

### Option 5: Apple/Google Cloud Vision OCR (Rejected)

**Pros:**
- Commercial-grade reliability
- Well-documented APIs

**Cons:**
- Designed for image OCR, not stroke data
- Requires rendering strokes to image first
- Added latency from image processing
- Paid services

**Rejection Reason:** Stroke-based recognition is more accurate and efficient than image-based for real-time input.

### Option 6: MyScript (Rejected)

**Pros:**
- Industry-leading handwriting recognition
- SDK available
- Supports gestures and shapes

**Cons:**
- Commercial license required (expensive)
- Complex SDK integration
- May require specific platform support

**Rejection Reason:** Licensing cost prohibitive for open-source project.

## Consequences

### Positive
- Rapid implementation with proven approach
- High accuracy from day one
- No ML expertise required
- Simple HTTP integration

### Negative
- Internet dependency for core feature
- Privacy consideration (strokes sent to Google)
- API stability risk (undocumented)
- No offline mode initially

## Mitigation Strategies

1. **API instability:** Abstract recognition behind trait, allowing backend swap
2. **Privacy:** Document data transmission clearly to users
3. **Offline:** Design architecture for future local model integration
4. **Latency:** Implement optimistic UI (show strokes immediately, recognize async)

## API Integration Details

```rust
// Stroke format for Google API
struct StrokeData {
    ink: Vec<[Vec<i32>; 3]>,  // [[x...], [y...], [t...]]
    writing_guide: WritingGuide,
}

struct WritingGuide {
    width: i32,
    height: i32,
}

// Endpoint
POST https://www.google.com/inputtools/request?ime=handwriting&app=mobilesearch&cs=1&oe=UTF-8
```

## Decision Criteria Matrix

| Criteria | Weight | Google API | Tesseract | On-Device ML | Mathpix |
|----------|--------|------------|-----------|--------------|---------|
| Handwriting accuracy | 35% | ✅ 10 | ❌ 2 | ⚠️ 6 | ✅ 9 |
| Implementation effort | 25% | ✅ 9 | ⚠️ 5 | ❌ 2 | ✅ 8 |
| Latency | 20% | ✅ 9 | ⚠️ 5 | ✅ 10 | ⚠️ 7 |
| Offline capability | 10% | ❌ 0 | ✅ 10 | ✅ 10 | ❌ 0 |
| Cost | 10% | ✅ 10 | ✅ 10 | ✅ 10 | ❌ 3 |
| **Weighted Score** | | **8.4** | **4.4** | **5.8** | **7.0** |
