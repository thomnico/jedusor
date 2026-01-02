# ADR-006: Existing Projects Analysis

## Status
**Informational**

## Context

Before building Jedusor, we surveyed existing projects that combine reMarkable tablets with AI, handwriting recognition, or interactive conversation to understand the landscape and identify gaps.

## Existing Projects Evaluated

### Direct AI + reMarkable Integration

#### 1. reMarkableAI (nickian)
**Repository:** https://github.com/nickian/reMarkableAI

**Approach:** Email-based async workflow
- Write prompt on reMarkable → Convert to text → Email to server
- Server polls IMAP, sends to OpenAI, generates PDF
- PDF uploaded to reMarkable cloud via rMAPI

**Tech Stack:** PHP, OpenAI API, mPDF, rMAPI

**Strengths:**
- Works without modifying tablet software
- Uses existing reMarkable cloud infrastructure

**Limitations:**
- Not real-time (cron-based, 60s polling)
- Requires external IMAP server
- Not interactive—single request/response
- No on-device experience

**Gap:** No live, on-device conversational experience.

---

#### 2. remarkableAI (arturo90)
**Repository:** https://github.com/arturo90/remarkableAI

**Approach:** Gmail-based note analysis
- Fetches notes as PDFs from Gmail
- Processes with EasyOCR/Tesseract/LLaVA
- Extracts tasks, summaries, topics
- Web dashboard for results

**Tech Stack:** Python, FastAPI, EasyOCR, Tesseract, Ollama, LLaVA

**Strengths:**
- Multiple OCR backends including multimodal LLM
- Sophisticated analysis pipeline
- Nice web interface

**Limitations:**
- Batch processing, not interactive
- Web-based interface, not on device
- One-way: notes → analysis, no dialogue

**Gap:** Analysis tool, not conversation interface.

---

#### 3. remarkable-mcp
**Repository:** https://github.com/SamMorrowDrums/remarkable-mcp

**Approach:** MCP server for AI assistants
- Enables Claude/Copilot to read reMarkable documents
- OCR for handwritten content
- Search across library

**Tech Stack:** TypeScript, MCP protocol

**Strengths:**
- AI can access reMarkable content
- Works with multiple AI assistants

**Limitations:**
- AI reads documents, doesn't write back
- Desktop AI experience, not on-device

**Gap:** AI reads your notes but can't respond on the tablet.

---

### Native Application Frameworks

#### 4. armrest
**Repository:** https://github.com/bkirwi/armrest

**Approach:** Rust library built on libremarkable
- Python ML pipeline for training HWR models
- TensorFlow Lite handwriting recognition
- Elm-inspired UI framework

**Tech Stack:** Rust, Python, TensorFlow Lite, libremarkable

**Strengths:**
- Native on-device HWR (offline capable)
- Deep LSTM architecture for recognition
- Custom language model support
- Built on libremarkable

**Limitations:**
- No chat/LLM integration
- Complex training pipeline
- Limited documentation

**Opportunity:** Could use armrest's HWR for offline recognition fallback.

---

#### 5. remarkable-apps (CurtisFenner)
**Repository:** https://github.com/CurtisFenner/remarkable-apps

**Approach:** Lua-based app engine
- Minimal API for framebuffer and pen input
- Uses rm2fb-client for display
- Cross-compiled with arm-linux-gnueabihf-gcc

**Tech Stack:** Lua, C, rm2fb-client

**Strengths:**
- Simple scripting approach
- Lower barrier than Rust/C++

**Limitations:**
- Lua performance constraints
- Minimal ecosystem
- No HWR or AI integration

**Gap:** Engine exists but no AI/chat apps built with it.

---

### Collaborative/Interactive Tools

#### 6. whiteboard-hypercard
**Repository:** https://github.com/fenollp/reMarkable-tools

**Approach:** Real-time collaborative whiteboard
- Live drawing sync via NATS messaging
- Multiple users share canvas
- Modular "card" tools (selection, digitize, image)
- "Digitize" tool converts handwriting via AI

**Tech Stack:** Rust, NATS, toltec

**Strengths:**
- Real-time on-device interaction
- AI digitization built-in
- Proven architecture for networked features

**Limitations:**
- Collaboration focus, not personal AI chat
- Requires server infrastructure

**Opportunity:** Architecture could inspire Jedusor's networking layer.

---

### Web-Based Tom Riddle Implementations

#### 7. ScribbleGPT
**URL:** https://blog.memsranga.com/scribblegpt-building-a-basic-handwriting-driven-chatbot

**Approach:** Web canvas-based diary
- HTML5 Canvas with Signature_Pad
- Google Input Tools API for HWR
- GPT-4o with Riddle persona
- Fade animation for responses

**Tech Stack:** React, Next.js, Google Input Tools, GPT-4o

**Strengths:**
- Proves the concept works
- Elegant fade animation
- Good HWR accuracy

**Limitations:**
- Web-only, not on reMarkable
- No e-ink optimization

---

#### 8. diary.ycmjason.com
**Repository:** https://github.com/ycmjason/diary.ycmjason.com

**Approach:** Web-based Riddle diary
- Canvas handwriting input
- Google IME (reverse-engineered) for recognition
- OpenRouter + Vercel AI SDK

**Tech Stack:** JavaScript/TypeScript, Google IME, OpenRouter

**Strengths:**
- Modern JS implementation
- Multiple LLM backend support
- Live demo available

**Limitations:**
- Browser-based only
- Not optimized for e-ink

---

## Gap Analysis

| Feature | reMarkableAI | armrest | whiteboard | ScribbleGPT | **Jedusor** |
|---------|--------------|---------|------------|-------------|-------------|
| On-device native app | ❌ | ✅ | ✅ | ❌ | ✅ |
| Real-time interaction | ❌ | N/A | ✅ | ✅ | ✅ |
| Handwriting recognition | ✅ (cloud) | ✅ (local) | ✅ | ✅ | ✅ |
| LLM conversation | ✅ | ❌ | ❌ | ✅ | ✅ |
| Tom Riddle persona | ❌ | ❌ | ❌ | ✅ | ✅ |
| E-ink optimized | N/A | ✅ | ✅ | ❌ | ✅ |
| Magical animation | ❌ | ❌ | ❌ | ✅ | ✅ |
| Offline capable | ❌ | ✅ (HWR) | ❌ | ❌ | ❌ (v1) |

## Conclusion

**No existing project combines:**
1. Native on-device reMarkable app
2. Real-time stylus handwriting input
3. LLM-powered conversation
4. Magical diary experience (Tom Riddle persona + appearing text animation)
5. E-ink display optimization

Jedusor fills this gap by building on:
- **libremarkable** (device I/O, like armrest)
- **Google Input Tools** (HWR approach from ScribbleGPT/diary.ycmjason)
- **Claude API** (LLM, superior to GPT for persona consistency)
- **Streaming animation** (inspired by web implementations, adapted for e-ink)

## Reuse Opportunities

| Component | From Project | Consideration |
|-----------|--------------|---------------|
| Offline HWR | armrest | TensorFlow Lite model for v2 offline mode |
| UI patterns | whiteboard-hypercard | Card-based tool architecture |
| Animation | ScribbleGPT | Adapt fade concept for e-ink constraints |
| Recognition API | diary.ycmjason | Google IME reverse-engineering reference |

## References

- [awesome-reMarkable](https://github.com/reHackable/awesome-reMarkable) - Curated project list
- [reMarkableAI](https://github.com/nickian/reMarkableAI) - Email-based AI workflow
- [remarkableAI](https://github.com/arturo90/remarkableAI) - Note analysis system
- [armrest](https://github.com/bkirwi/armrest) - Native HWR library
- [whiteboard-hypercard](https://github.com/fenollp/reMarkable-tools) - Collaborative whiteboard
- [ScribbleGPT](https://blog.memsranga.com/scribblegpt-building-a-basic-handwriting-driven-chatbot) - Web diary
- [diary.ycmjason.com](https://github.com/ycmjason/diary.ycmjason.com) - Web diary
