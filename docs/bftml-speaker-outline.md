# BFTML — Speaker Outline & Discussion Guide

---

## Overview

**Structure:** 4 acts across 9 slides  
**Tone:** Thoughtful provocateur. You're not selling — you're diagnosing a real problem that everyone in the room has felt.  
**Audience:** Mixed (some devs, some not). Keep the historical argument accessible. Technical detail lives in the Q&A.

---

## Act 1 — The Problem (Slides 1–3)

### Slide 1: Title
Cold open. Let the slide breathe. No need to explain the acronym yet — the subtitle does the work.

> *"I want to start with a question before I tell you anything about what I'm building."*

Pivot directly to slide 2.

---

### Slide 2: Opening Discussion — *"What's your biggest frustration with building for the web?"*

**This is your most important slide.** Give it time. You want the room talking before you deliver your thesis.

**Prompts to draw people out:**

*For developers in the room:*
- "What's the last time CSS made you feel genuinely insane? What happened?"
- "How many JavaScript frameworks have you adopted and abandoned? Be honest."
- "Have you ever shipped a bug because something failed silently — no error, nothing — and you spent hours hunting it down?"
- "If you could remove one thing from how the web works, what would it be?"

*For non-technical people in the room:*
- "What's the thing you wanted to build on the web that felt way harder than it should have been?"
- "When you've worked with developers, what's the thing they complained about most that you wished you could fix for them?"
- "If you could design the tools from scratch, what would feel more intuitive?"

**What you're listening for:**  
Let them name the symptoms. They'll probably say things like: "CSS is unpredictable," "there are too many frameworks," "nothing tells you when something breaks." Validate each one. Don't jump to solutions yet.

> *"These aren't skill problems. Everyone in this room who's ever touched web development has felt these. And I'd argue they're not even random frustrations — they're symptoms of a single deeper issue."*

Advance to slide 3.

---

### Slide 3: The Pain Points

Walk through the three columns briefly. Don't linger — the audience just named most of this.

> *"HTML has inconsistent rules that even experts trip on. CSS fails invisibly. JavaScript was designed for a specific kind of developer, but everyone on the web is forced to use it. These are real, structural problems."*

> *"But here's the thing — these aren't failures of the people who built them. The people who built them were solving a completely different problem. The problem was: how do we put documents on the internet?"*

---

## Act 2 — The Diagnosis (Slides 4–5)

### Slide 4: History

This is your intellectual moment. Slow down here.

> *"The web's foundations come from documents. Hypertext. The printed page. And for a long time, that was fine — because most websites really did look like print."*

Walk the timeline casually. You're building a sense of inevitability.

> *"But then web apps appeared. And instead of building a new platform for apps, we bolted app behavior onto the document platform. We got JavaScript. We got the DOM API. We got React and Vue and Angular — which are component frameworks built on top of a document model that was never designed to support components."*

> *"And developers got so frustrated that some of them would rather write Rust, compile it to WebAssembly, generate JavaScript bindings, and use that to manipulate the DOM — just to avoid writing JavaScript directly."*

Pause. Let that land.

> *"Something strange is happening."*

---

### Slide 5: The Key Insight

This is the thesis. Deliver it as a revelation, not a lecture.

> *"Here's what I think is actually going on."*

> *"A document is a static hierarchy of content with implicit layout and styling. A component is a reusable, stateful, self-contained unit of behavior. These are not the same kind of thing. They're not even on the same axis."*

> *"No amount of expanding a document platform naturally produces a component platform. The browser has been trying to do this expansion for thirty years, and it's always been awkward, because you can't get there from here."*

> *"Document developers and app developers have been arguing about best practices for decades — seemingly unaware that they're not making the same things."*

---

## Act 3 — The Vision (Slides 6–8)

### Slide 6: BFTML Introduction

Shift tone. You're no longer diagnosing — you're building.

> *"So that's the problem. What if we started from the right place?"*

> *"BFTML — which stands for something I'll let you figure out — is a new set of web standards designed from the component abstraction as the foundation. Not a destination we're slowly, painfully expanding toward. The foundation."*

Point to the code example.

> *"Here's a button in BFTML. You can see that behavior, layout, content, and styling are all separate primitives. You compose them. Nothing is implicit. Nothing is inherited from a document model that predates this concept by decades."*

---

### Slide 7: The Four Pillars

Walk through these at a relaxed pace. Each one is a direct response to a frustration the audience named.

- **Component-first:** "Apps deserve their own standards."
- **Single responsibility:** "Every element does one thing. `box` draws a box. That's it. If it's broken, you know exactly where to look."
- **Consistent syntax:** "All tags follow the same grammar. Two types of errors: syntax errors and tag errors. No silent ambiguity."
- **Explicitness by default:** "If something breaks, the error is visible. If you want a fallback, you define it explicitly. Nothing fails silently."

> *"These aren't new ideas in programming. Single responsibility, explicit over implicit, consistent interfaces — we've known these are good principles for decades. We just haven't applied them to the web's foundations."*

---

### Slide 8: WASM Native

This slide tends to get the most reaction from developers. Play to that.

> *"Remember those developers writing Rust to avoid JavaScript? BFTML gives them what they actually want."*

> *"The runtime is WebAssembly-native. Components can be written in any language that compiles to WASM — Rust, Go, Zig, C, Swift, whatever your community prefers. No JS bridge. No bindings. Direct manipulation of the markup."*

> *"And components are scoped to their subtree. You can't accidentally reach across the whole document the way JavaScript can today. The safety and structure of a framework, without the lock-in."*

> *"And the same model applies at every level of the stack. Primitive elements are WASM. Components built from those primitives are also WASM. There's no interpreted layer sitting in the middle — it's WASM all the way up."*

---

### Slide 10: Dev Tools

This slide lands differently depending on who's in the room — but it tends to resonate with everyone, because everyone has spent time lost in a debugger that speaks a different language than the code they wrote.

> *"Here's something I think is just as important as the runtime itself: you should never have to see WASM in your dev tools."*

> *"All behavior in BFTML routes through signals. So that's what the dev tools shows you — not bytecode, not a framework's internal reconciliation state, not a virtual DOM diff. The signal graph. Which signals exist, what depends on what, how they're connected."*

> *"And when something changes, you can watch the dirty flags propagate in real time. You see which nodes get marked, in what order, which draw commands get recomputed. The story of what happened, told in the same terms you used to write the code."*

> *"The principle is: the layer you think in and the layer you debug in should be the same layer. BFTML makes that possible because signals aren't just an implementation detail — they're the actual abstraction. There's nothing hiding beneath them that you'd ever need to inspect."*

**For the technical audience:**
> *"Compare that to debugging a React app today — you're looking at component re-renders, fiber trees, scheduler internals. None of that maps to anything in your source code. BFTML's dev tools has nothing to hide, because the abstraction doesn't leak."*

**If someone asks whether you can inspect the WASM directly:**
> *"Yes — if you know where to look. You can drill into a given module, view the WASM, even edit it in place. It's there. But it's not visible by default, because for the vast majority of debugging scenarios you'll never need it. It's an escape hatch, not the front door."*

---

### Slide 11: ATProto Component Registry

This slide tends to surprise people — the connection between a UI engine and a decentralized protocol isn't obvious until you explain it, and then it feels inevitable. For the Folk Tech crowd especially, the open infrastructure angle is likely to resonate strongly.

**Set up the problem first:**

> *"Every package registry we use today is centralized. npm is owned by GitHub, which is owned by Microsoft. crates.io is run by the Rust Foundation. PyPI is run by the PSF. These are well-intentioned organizations — but the infrastructure for distributing the building blocks of software lives on servers that someone else controls, under terms of service that can change."*

> *"BFTML takes a different approach."*

**Introduce ATProto:**

> *"ATProto is the protocol underneath Bluesky — an open, federated protocol for social data. The core idea is that your data lives on a personal data server that you control. Applications are built on top of that data, but they don't own it."*

> *"In ATProto, there are two key concepts: the PDS — personal data server — and the AppView. The PDS is where your data lives. The AppView is a service that aggregates data from many PDSes and presents it in a useful form — a social feed, a search index, or in our case, a component registry."*

**Connect to BFTML:**

> *"When you publish a BFTML component, you publish it to your own PDS. It's signed with your identity, it lives on your server, and no platform can take it down. AppViews crawl the network of PDSes and aggregate those components into something that looks and works exactly like npm or crates.io — searchable, versioned, installable."*

> *"The interface is familiar. The infrastructure is open. And because the underlying data is public and portable, anyone can run a competing AppView. There's no single registry to capture, no single company to lobby."*

**For the technically curious:**

> *"ATProto uses a content-addressed record format with cryptographic identity — so component provenance is verifiable. You know exactly who published a component and that it hasn't been tampered with, without trusting a central authority to tell you so."*

---

## Act 4 — Open Discussion (Slide 12)

### Slide 12: Open Floor

---

### Slide 9: The Reactive Model

This slide rewards the technical people in the room without losing the non-technical ones. Lead with the intuition, then go into the mechanics.

> *"Let me show you how the engine actually works — because I think it's one of the more interesting design decisions in BFTML."*

**Phase 1 — Static tree (point to the diagram)**

> *"When your app loads, BFTML stitches all your components together into a single element tree. That tree is built once and lives for the entire lifetime of the app. It never gets rebuilt. There's no reconciliation, no diffing against a virtual copy — the structure is stable."*

**Phase 2 — Dirty flags (point to the state change arrow)**

> *"When state changes — say a counter increments — that change immediately marks the affected node dirty. Not scheduled for the next tick, not batched somewhere. Right then. The node knows it needs to recompute."*

**Phase 3 — Lazy render (point to the render pass bar)**

> *"At render time, the engine traverses the tree. When it hits a clean node, it reuses the cached renderer and moves on. When it hits a dirty node, it recomputes, caches the result, and clears the flag."*

> *"In this example, only one node out of the whole tree pays any render cost. Everything else is cache hits."*

**The punchline — for non-technical audience members:**

> *"The short version: your app only does work for the things that actually changed. Not the whole UI. Not a virtual copy of the whole UI. Just the part that changed."*

**For the technically curious:**

> *"What's being cached is a `Renderer` struct — a fully realized wgpu render pipeline, a GPU-side instance buffer, and bind groups that wire uniforms like screen size and scale factor directly into the shader stages. Rebuilding that on every frame for every node would be genuinely expensive. The dirty flag system isn't just a nice optimization — it's load-bearing. A clean node doesn't recompute anything, it just calls `render()` on what's already assembled on the GPU. A dirty node reconstructs the whole thing and caches it for next time. No virtual DOM, no diffing, no reconciliation — just a pipeline that's either ready or needs to be rebuilt."*

> *"And for anyone wondering whether this pattern is proven — game engines have been using dirty flags for decades. Unity, Unreal, Godot all use variants of this to avoid recomputing transforms, physics state, and render data every frame. The paradigm is battle tested. BFTML is just applying it to UI, which is honestly where it should have been all along."*

---

## Act 4 — Open Discussion (Slide 10)

### Slide 11: Open Floor

Reopen the conversation. You started by asking what frustrates people. Now you flip it.

> *"So here's my question to the room:"*

> *"If the platform got out of your way — if you had these primitives, this runtime, this explicitness by default — what would you build?"*

Give this genuine space. You'll hear:
- People naming specific apps or tools they've always wanted to make
- Developers interested in writing primitives in their preferred language
- Questions about the parser, the compiler, the WASI interface

**Closing note:**

> *"BFTML is early. The vision is clear; the implementation is being built. What I'm looking for is people who've felt this pain and want to help solve it — whether that's writing primitives, designing the language, building the compiler, or just kicking the tires and telling me where it breaks."*

---

## Discussion Prompts — Quick Reference Card

### Opening (warm-up)
- "What's your biggest frustration with web development — or watching web development happen?"
- "How many frameworks have you used, adopted, and abandoned?"
- "Has CSS ever made you genuinely question your life choices?"

### Technical (for dev-heavy moments)
- "What would you write web UIs in if you could use any language?"
- "What's the thing you wish the DOM spec had done differently?"
- "Would you write custom WASM primitives if you could? What would they be?"

### Creative (for non-technical moments)
- "What's the app you keep wishing existed but hasn't been built well yet?"
- "If building software felt as natural as writing — what would that change?"

### Closing
- "What would you build if the platform got out of your way?"
- "Who wants to break things with me?"

---

## Timing Guide (Flexible)

| Section | Target | Notes |
|---|---|---|
| Slides 1–3 (problem + discussion) | 10–15 min | Let discussion breathe. This is the best part. |
| Slides 4–5 (diagnosis) | 5–7 min | Intellectual core. Don't rush. |
| Slides 6–8 (vision + pillars + WASM) | 5–8 min | Move faster here. Q&A will go deeper. |
| Slide 9 (reactive model) | 3–5 min | Let the diagram do the work. |
| Slide 10 (dev tools) | 3–5 min | Strong closer before the open floor — ends on DX, not internals. |
| Slide 11 (ATProto registry) | 4–6 min | Likely to spark discussion — leave room for questions. |
| Slide 12 (open floor) | 10–20 min | Uncapped. This is where interest becomes collaboration. |

*Total: 30–50 minutes depending on discussion depth. Works as a 20-minute lightning version by compressing slides 4–8.*

---

## Handling Tough Questions

**"Why not just use [React / Solid / Leptos / etc.]?"**  
> "Those are component frameworks built on the document model. They're workarounds for the same problem I'm describing. BFTML isn't a framework — it's a different foundation. The difference matters because the foundation shapes everything above it."

**"Why would anyone adopt a new standard?"**  
> "Every standard that exists today was once a new standard. The question isn't whether people will adopt it — it's whether the problem is real enough to justify a new foundation. I think thirty years of awkward expansion is a pretty clear answer."

**"What's the timeline?"**  
> "Early. Vision is clear, implementation is active. That's actually why I'm here — to find the people who want to help shape it."

**"Is this a browser replacement?"**  
> "Not necessarily. It could run in a browser, as a desktop runtime, or natively — depending on where the community takes it. The spec and reference implementation come first."
