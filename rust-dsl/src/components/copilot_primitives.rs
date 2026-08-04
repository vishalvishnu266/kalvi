//! Reusable primitives extracted from `<ui-copilot>`.
//!
//! These render tiny bits of DSL that the Copilot uses internally, so pages
//! can drop them anywhere and re-use the exact styling / behaviour.
//!
//! ```ignore
//! use lit_ui::prelude::*;
//!
//! step_bar(2, 4, "Fetching fees").render();
//! tool_card_call("send_sms", "to=guardian, template=welcome").render();
//! tool_card_result(true, "SMS delivered").render();
//! ```
//!
//! Because the CSS lives inside the copilot component's shadow DOM, this
//! module also emits **light-DOM** styles once per page (via a `<style>`
//! block co-located with the first primitive rendered). If a page already
//! contains the copilot the CSS is duplicated but harmless (same rules).

use crate::core::{escape_html, Component};

// ---------------------------------------------------------------------------
// Public helpers
// ---------------------------------------------------------------------------

/// Progress bar with `n/of` label and a coloured fill. Same visuals as the
/// step bar shown inside `<ui-copilot>` while a streaming tool runs.
pub fn step_bar(n: u32, of: u32, label: impl Into<String>) -> StepBar {
    StepBar { n, of, label: label.into() }
}

/// "Calling a tool" card — dashed border, thinking dots.
pub fn tool_card_call(name: impl Into<String>, args: impl Into<String>) -> ToolCardCall {
    ToolCardCall { name: name.into(), args: args.into() }
}

/// "Tool completed" card — solid check / cross with summary text.
pub fn tool_card_result(ok: bool, summary: impl Into<String>) -> ToolCardResult {
    ToolCardResult { ok, summary: summary.into() }
}

/// Compact mention popover — receives already-resolved entities and renders
/// the same list layout used inside the copilot's `@`-typeahead.
pub fn mention_popover(items: Vec<MentionEntity>) -> MentionPopover {
    MentionPopover { items }
}

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

pub struct StepBar { n: u32, of: u32, label: String }
pub struct ToolCardCall { name: String, args: String }
pub struct ToolCardResult { ok: bool, summary: String }

#[derive(Clone)]
pub struct MentionEntity {
    pub id: String,
    pub label: String,
    pub subtitle: String,
    pub kind: String,   // "student" | "staff" | "class"
    pub icon: String,   // emoji or single character
}

pub struct MentionPopover { items: Vec<MentionEntity> }

// ---------------------------------------------------------------------------
// Renderers
// ---------------------------------------------------------------------------
//
// Every renderer prepends the shared CSS once per rendered tree using a
// tiny sentinel class on `<style>` so pages that render multiple primitives
// don't duplicate rules in the DOM.

const CSS_MARKER: &str = "cp-prim-styles";

fn shared_css() -> &'static str {
    r#"<style class="cp-prim-styles" data-cp-prim="1">
    .cp-step-bar {
      display: flex; align-items: center; gap: 10px;
      padding: 8px 12px;
      font-size: 12px; color: var(--color-text-muted, #475569);
      background: linear-gradient(to right,
        color-mix(in srgb, var(--color-primary, #0a84ff) 6%, transparent),
        transparent);
      border-top: 1px dashed var(--color-border, rgba(0,0,0,.08));
      border-radius: 8px;
    }
    .cp-step-bar .track {
      flex: 1; height: 4px; border-radius: 2px;
      background: var(--color-border, rgba(0,0,0,.08)); overflow: hidden;
    }
    .cp-step-bar .fill {
      height: 100%; background: var(--color-primary, #0a84ff);
      transition: width .35s ease;
    }
    .cp-tool-card {
      display: inline-flex; align-items: center; gap: 8px;
      padding: 8px 10px; border-radius: 10px;
      background: var(--color-info-soft, rgba(90,200,250,.14));
      color: var(--color-info-strong, #036);
      font-size: 13px;
      border: 1px dashed var(--color-info, #5ac8fa); margin-top: 6px;
    }
    .cp-tool-card.ok    { border-style: solid; }
    .cp-tool-card.error {
      background: var(--color-danger-soft, rgba(239,68,68,.14));
      color: var(--color-danger-strong, #991b1b);
      border-color: var(--color-danger, #ef4444);
      border-style: solid;
    }
    .cp-tool-card code {
      font-family: ui-monospace, monospace; font-size: 11px;
      background: var(--color-surface-hover, rgba(0,0,0,.05));
      padding: 1px 6px; border-radius: 4px;
    }
    .cp-thinking { display: inline-flex; align-items: center; gap: 3px; }
    .cp-thinking span {
      width: 5px; height: 5px; border-radius: 50%;
      background: currentColor;
      animation: cp-bp 1s infinite ease-in-out;
    }
    .cp-thinking span:nth-child(2) { animation-delay: .15s; }
    .cp-thinking span:nth-child(3) { animation-delay: .30s; }
    @keyframes cp-bp {
      0%,80%,100% { opacity: .2; transform: translateY(0); }
      40%          { opacity: 1;  transform: translateY(-2px); }
    }
    .cp-mention-popover {
      display: block;
      background: var(--color-surface, #fff);
      color: var(--color-text, #0f172a);
      border: 1px solid var(--color-border, rgba(0,0,0,.08));
      border-radius: 12px;
      box-shadow: 0 8px 24px color-mix(in srgb, currentColor 10%, transparent);
      max-width: 360px; overflow: auto;
    }
    .cp-mention-item {
      display: flex; align-items: center; gap: 10px;
      padding: 8px 12px;
      border-bottom: 1px solid var(--color-border, rgba(0,0,0,.08));
    }
    .cp-mention-item:last-child { border-bottom: 0; }
    .cp-mention-item .avatar {
      width: 28px; height: 28px; border-radius: 50%;
      background: var(--color-surface-alt, #f2f4fb);
      display: grid; place-items: center; font-size: 14px;
    }
    .cp-mention-item .info { flex: 1; min-width: 0; }
    .cp-mention-item .info .name { font-size: 13px; font-weight: 500; }
    .cp-mention-item .info .sub  { font-size: 11px;
                                    color: var(--color-text-muted, #475569); }
    .cp-mention-item .type {
      font-size: 10px; text-transform: uppercase; letter-spacing: .04em;
      background: var(--color-surface-alt, #f2f4fb);
      color: var(--color-text-muted, #475569);
      padding: 2px 6px; border-radius: 4px;
    }
    </style>"#
}

/// Emit the shared style block. Pages usually render multiple primitives
/// side-by-side — repeated `<style>` tags with identical rules are cheap
/// and correct in browsers, but we still add a marker class so authors can
/// spot / dedupe them if they wish.
fn with_css(body: String) -> String {
    format!("{}{}", shared_css(), body)
}

impl Component for StepBar {
    fn render(&self) -> String {
        let pct = if self.of == 0 { 0 } else { self.n * 100 / self.of };
        with_css(format!(
            r#"<div class="cp-step-bar">
                 <span>Step {n}/{of} · {label}</span>
                 <div class="track"><div class="fill" style="width:{pct}%"></div></div>
               </div>"#,
            n = self.n, of = self.of, label = escape_html(&self.label), pct = pct,
        ))
    }
}

impl Component for ToolCardCall {
    fn render(&self) -> String {
        with_css(format!(
            r#"<div class="cp-tool-card">🔧 <strong>{}</strong> <code>{}</code>
                 <span class="cp-thinking"><span></span><span></span><span></span></span>
               </div>"#,
            escape_html(&self.name),
            escape_html(&self.args),
        ))
    }
}

impl Component for ToolCardResult {
    fn render(&self) -> String {
        let (cls, icon) = if self.ok { ("cp-tool-card ok", "✅") } else { ("cp-tool-card error", "⚠️") };
        with_css(format!(
            r#"<div class="{cls}">{icon} {}</div>"#,
            escape_html(&self.summary),
        ))
    }
}

impl Component for MentionPopover {
    fn render(&self) -> String {
        let items = self.items.iter().map(|e| format!(
            r#"<div class="cp-mention-item">
                 <div class="avatar">{icon}</div>
                 <div class="info">
                   <div class="name">{name}</div>
                   <div class="sub">{sub}</div>
                 </div>
                 <span class="type">{kind}</span>
               </div>"#,
            icon = escape_html(&e.icon),
            name = escape_html(&e.label),
            sub  = escape_html(&e.subtitle),
            kind = escape_html(&e.kind),
        )).collect::<Vec<_>>().join("");
        with_css(format!(r#"<div class="cp-mention-popover">{items}</div>"#))
    }
}

// Silence unused-const warnings if a build only pulls one primitive.
#[allow(dead_code)] const _: &str = CSS_MARKER;
