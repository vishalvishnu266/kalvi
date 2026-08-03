# ERP Copilot — Agentic Chat Window

A persistent, mobile-friendly chat window that can:

* Stream tokens like an LLM (server → browser via SSE)
* Run multi-step "agent" tasks with a live progress bar
* Drive the UI: navigate pages, focus fields, fill forms, highlight elements, toggle theme, push toasts
* Survive Turbo page navigations (mounted outside `<turbo-frame id="page">`)

There is **no LLM and no database** wired up. Every scenario is mocked in
`src/web/copilot.rs` so you can build the UX first and swap in a real
orchestrator later without touching the front-end.

---

## Architecture (hybrid: web-component shell + server-rendered fragments)

```
┌────────────────────────────────────────────────────────────────┐
│  <ui-copilot>  (lit-components/components/ui-copilot.js)       │
│   ├── Floating FAB + Drawer (desktop) / Bottom-sheet (mobile)  │
│   ├── SSE reader → dispatcher                                  │
│   ├── Client "tools" registry:                                 │
│   │     navigate | focus | scroll_to | highlight | fill_form   │
│   │     open_modal | toast | set_theme                         │
│   └── Injects server HTML fragments into #messages             │
└────────────────────────────────────────────────────────────────┘
                        ▲            │
                text/event-stream   POST JSON
                        │            ▼
┌────────────────────────────────────────────────────────────────┐
│  /copilot/*  (src/web/copilot.rs) — Axum                       │
│   ├── POST /copilot/session  → {"session_id": "cs-…"}          │
│   ├── GET  /copilot/history/:sid → text/html                    │
│   └── POST /copilot/message → SSE stream                       │
│         events: message_start | token | tool_call | tool_result│
│                 | step | ui_action | message_end | error        │
└────────────────────────────────────────────────────────────────┘
```

### Why hybrid (not "pure Turbo" and not "pure Web Component")?

* **Pure Turbo frames** would unmount the chat on every page navigation.
* **Pure JS** would re-implement rendering that the Rust DSL already owns.

Answer: keep the *shell* in a persistent Lit element; keep the *content*
server-rendered as HTML by the Rust DSL and streamed via SSE. The client
just appends fragments.

---

## Usage

### From any DSL page

`layout::page_of(...)` auto-mounts `<ui-copilot>`. Nothing to do — every
existing page (Dashboard, Students, Fees, etc.) already has the FAB.

### Manual mount

```rust
use lit_ui::prelude::*;

page()
    .title("My page")
    .add(container().add(/* ... */))
    .with_copilot()           // ← inject the FAB
    .render()
```

Custom endpoints (e.g. per-tenant):

```rust
page()
    .with_copilot()
    .copilot_session_url("/web/acme/copilot/session")
    .copilot_stream_url ("/web/acme/copilot/message")
    .copilot_history_url("/web/acme/copilot/history")
    .copilot_label("Acme Copilot")
```

### Try it

Run the server, open any `/dsl/*` page. Click the ✨ FAB (or press
<kbd>Ctrl/Cmd + J</kbd>) and try:

| Prompt (case-insensitive)        | What happens                                             |
| -------------------------------- | -------------------------------------------------------- |
| `open the dashboard`             | Streams reply, then `navigate` to `/dsl/dashboard`       |
| `show students`                  | Navigates to `/dsl/students`                             |
| `fees`                           | Navigates to `/dsl/fees`                                 |
| `enrol a student` / `add student`| **Multi-step**: 3 tool cards, step bar 1/3 → 3/3, toast  |
| `fill the form`                  | Server sends `fill_form` — populates any `<ui-form>`     |
| `highlight` or `focus`           | Pulses + focuses the first input                         |
| `dark mode` / `light mode`       | Toggles `<html data-theme>`                              |
| `help` / `what can you do`       | Renders clickable suggestion chips in the bubble         |
| anything else                    | Word-by-word streamed echo                               |

---

## SSE event schema

The server sends **named** SSE events. Two shapes:

1. **HTML fragment events** — `data:` is server-rendered HTML that the
   client appends verbatim. Keeps rendering server-side (Rust DSL).
   * `message_start` — full bubble skeleton (`<div class="msg msg-assistant" id="m-…">…`).
   * `tool_call` — tool card HTML.
   * `tool_result` — result HTML.

2. **JSON control events** — `data:` is compact JSON the client parses.
   * `token`      `{"id": "m-…", "text": "…"}`
   * `step`       `{"n": 2, "of": 4, "label": "Fetching fees"}`
   * `ui_action`  `{"action": "navigate", "href": "/students/123"}`
   * `message_end`, `error`

### Client-side UI tools

| action        | args                                          | what it does                          |
| ------------- | --------------------------------------------- | ------------------------------------- |
| `navigate`    | `{href}`                                      | `Turbo.visit(href)`                   |
| `focus`       | `{selector}`                                  | `.focus()` on the first match         |
| `scroll_to`   | `{selector}`                                  | Smooth scroll                         |
| `highlight`   | `{selector, ms?}`                             | Pulsing outline for N ms              |
| `fill_form`   | `{selector, values: {name: value, …}}`        | Sets values + fires input/change      |
| `open_modal`  | `{selector}`                                  | `.openModal()` on `<ui-modal>`        |
| `toast`       | `{message, tone?}`                            | Pushes into `<ui-toast-host>`         |
| `set_theme`   | `{theme: "light" \| "dark"}`                  | Sets `<html data-theme>`              |

To add a new tool: add a function to the `TOOLS` map in
`lit-components/components/ui-copilot.js`.

---

## Swapping the mock for a real LLM / orchestrator

Only `src/web/copilot.rs::run_scenario` changes. Replace the intent-matching
`if p.contains(…)` chain with your planner/tool-router. Keep emitting the
same SSE events and the front-end continues to work unchanged.

Typical shape:

```rust
async fn run_scenario(em: &EventEmitter, prompt: &str) -> Result<(), String> {
    let plan = planner::plan(prompt).await?;      // LLM call
    let bubble = em.new_bubble().await;
    for (n, step) in plan.iter().enumerate() {
        em.step(n as u32 + 1, plan.len() as u32, &step.label).await;
        let call = em.tool_call(&step.tool, &step.args_display()).await;
        let result = tools::dispatch(step).await?;
        em.tool_result(&call, result.ok, &result.summary).await;
    }
    em.stream_text(&bubble, "All done.", 25).await;
    em.end(&bubble).await;
    Ok(())
}
```

---

## Files touched / added

**Front-end**
* `lit-components/components/ui-copilot.js` — the web component (new).
* `lit-components/components/lazy.js` — registered `ui-copilot` for lazy load.
* `lit-components/copilot-demo.html` — standalone preview page.

**Rust DSL**
* `rust-dsl/src/components/copilot.rs` — typed builder (new).
* `rust-dsl/src/components/mod.rs` — module registration.
* `rust-dsl/src/lib.rs` — prelude re-export.
* `rust-dsl/src/components/page.rs` — `Page::with_copilot()` and setters.
* `rust-dsl/src/layout.rs` — `page_of()` now auto-mounts the copilot.

**Server**
* `src/web/copilot.rs` — Axum SSE mock backend (new).
* `src/web/mod.rs` — module registration.
* `src/http/routes.rs` — merged `/copilot/*` router.
* `Cargo.toml` — added `tokio-stream` and `futures`.

---

## Mobile behaviour

* Below 720 px viewport the panel becomes a **bottom-sheet** taking 85 vh,
  with a grabber handle and `env(safe-area-inset-*)` respected for iOS.
* On desktop it's a **right-hand drawer** ~440 px wide.
* No user code changes needed — pure CSS media query inside the component.

---

## Keyboard shortcuts

* <kbd>Ctrl</kbd>/<kbd>Cmd</kbd> + <kbd>J</kbd> — toggle Copilot
* <kbd>Enter</kbd> — send · <kbd>Shift</kbd>+<kbd>Enter</kbd> — newline
* <kbd>Esc</kbd> — close panel
