# The `lit-ui` framework

A **backend-driven, AI-first web framework**. The browser hosts a thin
web-component shell; the backend authors every visible change and ships
it as HTML fragments over normal requests or SSE. Same protocol for
links, forms, and copilot commands.

Server-rendered HTML fragments applied to named regions of a
custom-element shell — no client-side templating, no client framework
beyond Lit (only used to author the components themselves). Designed to
be wrapped in Capacitor/Flutter later without changing the server.

---

## The mental model in one paragraph

Everything the user sees is either the **shell** (`<ui-app-shell>` with
6 named "islands": `topbar`, `sidebar`, `main`, `copilot`, `toast`,
`modal`) or a **page** inside the `main` island. When the user clicks a
link, submits a form, or asks the copilot to do something, the client
fetches (or streams) `<ui-fragment target="…" action="…">…</ui-fragment>`
envelopes from the server and applies them to the matching island. The
URL bar is kept in sync with `pushState`. **The browser never invents
UI** — every visible pixel came from a server response.

---

## Wire format

```html
<ui-fragment target="main" action="replace">
  <!-- any HTML — normally produced by the rust-dsl -->
</ui-fragment>
```

**Actions**

| action    | semantics                                           |
|-----------|-----------------------------------------------------|
| `replace` | (default) swap the target's children                |
| `append`  | append body as last children                        |
| `prepend` | prepend body as first children                      |
| `remove`  | clear the target's children (body is ignored)       |
| `update`  | set `innerHTML` directly (preserves target element) |

Multiple envelopes may appear in a single response body. SSE sends the
same envelopes, one per `event: fragment`.

**Content negotiation**

- `Accept: text/vnd.ui-fragments+html` → server returns fragments only.
- Anything else → server returns a full HTML document (shell + fragments
  inlined into their slots). Deep-linking to any URL Just Works.

The JS runtime automatically sets the fragment MIME on intercepted
navigation, so this is invisible to page authors.

---

## Repo layout

```
rust-dsl/              # the DSL — dependency-free, framework-agnostic
framework/
├── ui-shell/          # server runtime: Fragment, negotiate, SSE
└── agent/             # Agent trait + AgentEvent (LLM lands later)
lit-components/
├── components/        # web components (ui-button, ui-card, …)
│   ├── ui-app-shell.js     # the 6-slot shell
│   └── ui-fragment.js      # self-applying <ui-fragment>
└── framework/
    └── shell.js       # click/submit interceptor + history API
server/                # Axum demo — routes wired end-to-end
```

---

## Add a new page in ~10 lines

```rust
// server/src/pages/reports.rs
use axum::{http::HeaderMap, response::Response};
use lit_ui::prelude::*;
use ui_shell::{Fragment, Fragments, Target, negotiate};
use crate::shell::chrome;

pub async fn handler(headers: HeaderMap) -> Response {
    let body = card().title("Reports").add(list_item().title("Q1"));
    let frags = Fragments::new().push(Fragment::replace(Target::Main, body));
    negotiate(&headers, frags, chrome)
}
```

Register it in `server/src/main.rs`:

```rust
.route("/reports", get(pages::reports::handler))
```

Add a link somewhere in the sidebar (`server/src/shell.rs`) and you're
done. The link click will swap `main`, update the URL, and the direct
URL is still fully loadable.

---

## The six islands

| target    | typical use                                              |
|-----------|----------------------------------------------------------|
| `main`    | the current page — swapped on every navigation           |
| `copilot` | agent chat pane (rebuilt in milestone 8)                 |
| `topbar`  | breadcrumbs, actions, theme toggle                       |
| `sidebar` | primary nav                                              |
| `toast`   | transient toasts — usually `append`                      |
| `modal`   | dialogs — `replace` to open, `remove` to close           |

The Rust `Target` enum, the DSL `Region` enum, and the client's slot
names all use the same strings. Change one, sweep all three.

---

## Side effects

Client-only actions (dark mode, focus, scroll) piggy-back on the
fragment protocol via a special `__side_effect__` target:

```html
<ui-fragment target="__side_effect__" kind="theme">{"theme":"dark"}</ui-fragment>
```

Handlers are registered once in `lit-components/components/ui-app-shell.js`.
Built-ins today: `theme`, `focus`, `scroll`. Add more by calling
`registerSideEffect('kind', handler)` from any component module.

---

## What's implemented (milestones 1–6) vs coming next

**Done**

- Fragment envelope + wire format
- Content negotiation (full page vs fragments)
- Client interception of `<a>` and `<form>`
- History API sync (`pushState`, `popstate`)
- 6-slot `<ui-app-shell>` + `<ui-fragment>` custom elements
- DSL wrappers (`app_shell`, `fragment`, `Region`)
- Demo pages: `/`, `/dashboard`, `/admin`, `/users`
- SSE helper (`ui_shell::fragments_sse`) ready for the copilot

**Coming (milestones 7–10)**

- `agent` crate: `StubAgent` with a rule-based command table
- New lean `<ui-copilot>` that POSTs to `/agent` and applies streamed fragments
- Mobile polish: bottom-sheet copilot, `view-transition` API animations
- LLM-backed `LlmAgent` behind the same trait

---

## Non-goals (kept out on purpose)

- **No client-side templating.** Fragments are pre-rendered HTML.
- **No custom router in the browser.** `pushState` + fetch is enough.
- **No component-level swaps.** Only whole-island swaps in v1.
- **No JSON APIs.** Wire format is always HTML fragments (with a special
  side-effect envelope for tiny client-only actions).
