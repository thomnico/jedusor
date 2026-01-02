# ADR-005: Response Animation Strategy

## Status
**Accepted**

## Context

The magical diary experience requires AI responses to appear as if being written by invisible hand or materializing like magical ink. This animation must:
- Work within e-ink display constraints
- Not cause excessive ghosting
- Balance visual effect with readability
- Be performant on limited hardware

## Decision

**Implement character-by-character reveal with streaming** using Partial GC16 refresh, combined with periodic full refresh for ghosting cleanup.

## Options Considered

### Option 1: Character-by-Character Streaming (Selected)

**Implementation:**
- Stream LLM response tokens
- Render each character as received
- Use Partial GC16 refresh per character/word
- Full refresh after complete response

**Pros:**
- Natural "being written" appearance
- Leverages Claude's streaming API
- User sees response progressively
- Matches magical diary concept

**Cons:**
- Many partial refreshes accumulate ghosting
- Requires careful refresh batching
- More complex implementation

**Evaluation:** Best matches the magical writing effect.

### Option 2: Word-by-Word Reveal (Alternative Selected)

**Implementation:**
- Buffer streaming tokens until word boundary
- Render complete words
- Partial refresh per word

**Pros:**
- Fewer refreshes than character-by-character
- Still provides progressive reveal
- Easier to implement than per-character
- Better ghosting management

**Cons:**
- Less smooth than character reveal
- Still accumulates some ghosting

**Status:** Implement as default, character-by-character as option.

### Option 3: Fade-In Effect (Rejected)

**Implementation:**
- Render text at low opacity
- Gradually increase opacity through multiple refreshes

**Pros:**
- Elegant visual effect
- Matches movie "ink appearing" look

**Cons:**
- E-ink doesn't support true opacity
- Would require many full refreshes
- Extremely slow (seconds per sentence)
- Heavy ghosting

**Rejection Reason:** E-ink technology doesn't support smooth opacity transitions. Would require impractical number of refreshes.

### Option 4: All-at-Once with Delay (Rejected)

**Implementation:**
- Wait for complete LLM response
- Render entire response at once
- Single refresh

**Pros:**
- Simplest implementation
- Cleanest display (one refresh)
- No ghosting accumulation

**Cons:**
- Loses magical "being written" effect
- User waits with no feedback
- Feels like normal chat, not magical diary

**Rejection Reason:** Destroys the core magical experience. No differentiation from normal chat.

### Option 5: Line-by-Line Typewriter (Rejected)

**Implementation:**
- Complete each line before showing
- Reveal line by line from top

**Pros:**
- Moderate refresh count
- Clean line rendering

**Cons:**
- Less magical than character reveal
- Doesn't match "writing" metaphor
- Feels mechanical

**Rejection Reason:** Doesn't capture the handwritten feel.

### Option 6: Ink Pooling Effect (Rejected for v1)

**Implementation:**
- Characters appear with "ink blob" that settles into letter shape
- Multiple refresh stages per character

**Pros:**
- Highly magical visual effect
- Unique and memorable

**Cons:**
- Extremely complex implementation
- Would need custom font rendering
- Too many refreshes
- Performance concerns

**Status:** Interesting for future exploration, too complex for v1.

## Consequences

### Positive
- Authentic magical writing experience
- Progressive feedback keeps user engaged
- Leverages streaming for perceived speed

### Negative
- Ghosting requires periodic cleanup refresh
- More complex than instant display
- Battery impact from refresh frequency

## Implementation Details

### Refresh Strategy

```rust
enum RefreshStrategy {
    // During streaming - fast, accumulates ghosting
    StreamingPartial,
    // After response complete - clean up ghosting
    FinalFull,
}

struct ResponseRenderer {
    char_count: usize,
    refresh_every_n_chars: usize,  // Batch chars between refreshes
    ghosting_threshold: usize,     // Full refresh after N partials
}

impl ResponseRenderer {
    fn render_token(&mut self, token: &str, fb: &mut Framebuffer) {
        for ch in token.chars() {
            self.draw_character(ch, fb);
            self.char_count += 1;

            if self.char_count % self.refresh_every_n_chars == 0 {
                fb.partial_refresh(
                    &self.dirty_rect,
                    PartialRefreshMode::Async,
                    waveform_mode::WAVEFORM_MODE_GC16,
                );
            }
        }

        // Periodic full refresh to clear ghosting
        if self.partial_refresh_count > self.ghosting_threshold {
            fb.full_refresh();
            self.partial_refresh_count = 0;
        }
    }
}
```

### Timing Parameters

| Parameter | Value | Rationale |
|-----------|-------|-----------|
| Chars per partial refresh | 3-5 | Balance speed vs refresh count |
| Partial refreshes before full | 20-30 | Ghosting accumulation limit |
| Inter-character delay | 30-50ms | Natural writing speed feel |
| Final full refresh delay | 200ms | Let user finish reading |

### Font Considerations

- Use handwriting-style font (e.g., "Shadows Into Light", "Amatic SC")
- Font must render well at e-ink resolution
- Consider anti-aliasing impact on ghosting

## Decision Criteria Matrix

| Criteria | Weight | Char Stream | Word Stream | Fade-In | All-at-Once |
|----------|--------|-------------|-------------|---------|-------------|
| Magical effect | 35% | ✅ 10 | ✅ 8 | ✅ 9 | ❌ 2 |
| Technical feasibility | 25% | ⚠️ 7 | ✅ 9 | ❌ 2 | ✅ 10 |
| Performance | 20% | ⚠️ 6 | ⚠️ 7 | ❌ 2 | ✅ 10 |
| User experience | 20% | ✅ 9 | ✅ 8 | ⚠️ 5 | ⚠️ 5 |
| **Weighted Score** | | **8.2** | **8.0** | **4.6** | **5.9** |
