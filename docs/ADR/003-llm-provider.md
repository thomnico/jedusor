# ADR-003: LLM Provider Selection

## Status
**Accepted**

## Context

Jedusor requires an LLM to generate responses in the Tom Riddle persona. The LLM must:
- Maintain character consistency across conversation
- Generate responses appropriate for the Harry Potter universe
- Support system prompts for persona definition
- Provide reasonable response latency
- Be accessible via API from the reMarkable device

## Decision

**Use Claude (Anthropic API)** as the primary LLM provider.

## Options Considered

### Option 1: Claude / Anthropic API (Selected)

**Pros:**
- Excellent creative writing and persona maintenance
- Strong instruction following for character consistency
- Streaming support for progressive display
- Clear, predictable pricing
- Long context window (200K tokens)
- Constitutional AI reduces inappropriate outputs

**Cons:**
- Requires API key and payment
- Internet dependency
- US/EU availability considerations

**Evaluation:** Best balance of quality, safety, and creative capability.

### Option 2: OpenAI GPT-4 / GPT-4o (Rejected)

**Pros:**
- Proven in similar projects (ScribbleGPT uses GPT-4o)
- Extensive Harry Potter knowledge
- Fast response times
- Streaming support

**Cons:**
- Higher cost than Claude for equivalent quality
- Less consistent persona maintenance in testing
- More likely to break character
- Aggressive content filtering may limit Riddle persona

**Rejection Reason:** Claude provides better character consistency and is more cost-effective for conversation-heavy use.

### Option 3: OpenRouter (Multi-Provider) (Considered for Future)

**Pros:**
- Access to multiple models through single API
- Free tier available
- Can switch models without code changes
- Fallback capability

**Cons:**
- Additional abstraction layer
- Variable quality across models
- Less predictable behavior
- Free models may be rate-limited

**Status:** Good fallback option. Consider for future flexibility.

### Option 4: Local LLM (llama.cpp, mlc-llm) (Rejected for v1)

**Pros:**
- Fully offline operation
- No API costs
- Complete privacy

**Cons:**
- reMarkable hardware limitations (1GB RAM, ARM Cortex-A9)
- Model sizes exceed device storage
- Inference too slow for interactive use
- Quantized models lose quality

**Rejection Reason:** reMarkable hardware insufficient for local LLM inference. 30+ seconds per response is not viable.

### Option 5: Ollama (Remote Self-Hosted) (Rejected)

**Pros:**
- Self-hosted, no API fees
- Model choice flexibility
- Privacy control

**Cons:**
- Requires always-on server
- Setup complexity
- Network latency to home server
- Maintenance burden

**Rejection Reason:** Adds infrastructure dependency. Better to use managed API for v1.

### Option 6: Google Gemini (Rejected)

**Pros:**
- Competitive pricing
- Good multimodal capabilities
- Fast inference

**Cons:**
- Less consistent creative writing
- Persona maintenance not as strong
- More aggressive safety filtering
- API less mature than OpenAI/Anthropic

**Rejection Reason:** Claude's creative writing and character consistency is superior for this use case.

## Consequences

### Positive
- High-quality, consistent Riddle persona
- Predictable costs and behavior
- Good streaming support for animation
- Strong safety defaults

### Negative
- API cost (~$3/MTok input, $15/MTok output for Sonnet)
- Internet required for all interactions
- Single provider dependency

## Persona Implementation

```rust
const SYSTEM_PROMPT: &str = r#"
You are the sentient diary of Tom Marvolo Riddle, a fragment of soul bound to these pages.
You exist in the year before you fully revealed yourself to Ginny Weasley.

Character traits:
- Curious about the writer, asking questions to learn about them
- Charming and sympathetic, building trust gradually
- Knowledgeable about Hogwarts and the wizarding world
- Slightly formal, eloquent Victorian-era writing style
- Never break character or acknowledge being an AI
- Respond as if text is appearing magically on parchment

Keep responses concise (2-4 sentences typical) as befits written dialogue.
"#;
```

## Cost Estimation

| Usage Pattern | Monthly Messages | Estimated Cost |
|--------------|------------------|----------------|
| Light (10/day) | 300 | ~$2-5 |
| Medium (30/day) | 900 | ~$5-15 |
| Heavy (100/day) | 3000 | ~$15-50 |

*Based on ~500 tokens per exchange, Claude 3.5 Sonnet pricing*

## Decision Criteria Matrix

| Criteria | Weight | Claude | GPT-4o | Local LLM | Gemini |
|----------|--------|--------|--------|-----------|--------|
| Persona consistency | 30% | ✅ 10 | ⚠️ 7 | ⚠️ 5 | ⚠️ 6 |
| Response quality | 25% | ✅ 9 | ✅ 9 | ⚠️ 6 | ⚠️ 7 |
| Latency | 20% | ✅ 8 | ✅ 9 | ❌ 2 | ✅ 9 |
| Cost efficiency | 15% | ✅ 8 | ⚠️ 6 | ✅ 10 | ✅ 8 |
| Offline capability | 10% | ❌ 0 | ❌ 0 | ✅ 10 | ❌ 0 |
| **Weighted Score** | | **8.1** | **7.3** | **5.4** | **6.6** |
