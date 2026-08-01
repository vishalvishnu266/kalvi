//! Shared helpers for the DSL documentation pages (`/dsl/layouts`,
//! `/dsl/components`, and any future doc pages).
//!
//! These are deliberately **NOT** in `layout.rs` or `components/` because
//! they're not part of the design system — they're docs-page infrastructure.
//! Keeping them here means:
//!
//! * they can use inline styles freely (docs are exempt from the "no
//!   inline styles" rule that applies to real pages),
//! * they don't pollute the DSL prelude with names like `swatch` or `code`,
//! * we have one place to change the docs look-and-feel.
//!
//! ## Usage
//!
//! ```ignore
//! use crate::pages::doc_helpers::*;
//!
//! example("row_actions()",
//!         "Standard action bar preset.",
//!         row_actions().add(swatch(1)).add(swatch(2)),
//!         r#"row_actions().add(save).add(cancel)"#);
//! ```

use crate::prelude::*;
use crate::core::escape_html;

// ---------------------------------------------------------------------------
// Coloured filler widgets used inside layout/component demos
// ---------------------------------------------------------------------------

/// Small numbered swatch — 60×56 primary-coloured box. Use in Row/Column
/// demos where you want to show layout without picking a real component.
pub fn swatch(n: u32) -> Node {
    Node::raw(format!(
        r#"<div style="min-width:60px;min-height:56px;display:flex;align-items:center;justify-content:center;
                     background:var(--color-primary);color:#fff;font-weight:600;
                     border-radius:8px;padding:8px 14px;">{n}</div>"#
    ))
}

/// Larger labelled swatch — for column/section demos where the label
/// carries meaning (e.g. "Main (flex 3)").
pub fn block(label: &str, height_px: u32) -> Node {
    Node::raw(format!(
        r#"<div style="display:flex;align-items:center;justify-content:center;
                     background:var(--color-primary);color:#fff;font-weight:600;
                     border-radius:8px;padding:16px;height:{h}px;min-width:80px;">{label}</div>"#,
        label = escape_html(label),
        h = height_px,
    ))
}

/// Subdued (dashed-outline) swatch — visually distinguishes "side" content
/// from "main" content in two-column layout demos.
pub fn side_block(label: &str, height_px: u32) -> Node {
    Node::raw(format!(
        r#"<div style="display:flex;align-items:center;justify-content:center;
                     background:var(--color-surface);color:var(--color-text);
                     border:1px dashed var(--color-border);
                     border-radius:8px;padding:16px;height:{h}px;min-width:80px;">{label}</div>"#,
        label = escape_html(label),
        h = height_px,
    ))
}

// ---------------------------------------------------------------------------
// Source-snippet renderer
// ---------------------------------------------------------------------------

/// Render a `<pre><code>` block for a Rust snippet, styled with the
/// design-system tokens so it visually reads as source code.
pub fn code(snippet: &str) -> Node {
    Node::raw(format!(
        r#"<pre style="margin:0;padding:12px 14px;background:var(--color-surface);
                     border:1px solid var(--color-border);border-radius:8px;
                     font-family:ui-monospace,SFMono-Regular,Menlo,monospace;
                     font-size:12px;line-height:1.55;overflow:auto;
                     color:var(--color-text);"><code>{}</code></pre>"#,
        escape_html(snippet),
    ))
}

// ---------------------------------------------------------------------------
// Example card — the standard doc-page tile
// ---------------------------------------------------------------------------

/// A labelled example tile: title + description + rendered demo + source
/// snippet. Every doc page composes a grid of these to show off a
/// primitive/component.
///
/// ```ignore
/// grid().cols_min(MinCol::W320).gap(Gap::Md)
///     .add(example("row_actions()", "Standard bar", demo, code_snippet))
///     .add(example("row()", "Bare row", demo, code_snippet));
/// ```
pub fn example(
    title: &str,
    description: &str,
    demo: impl Component + 'static,
    snippet: &str,
) -> Card {
    card()
        .title(title)
        .subtitle(description)
        .add(column().gap(Gap::Md)
            .add(Node::raw(r#"<div class="lu-text-muted" style="font-weight:600;">Rendered</div>"#))
            .add(demo)
            .add(Node::raw(r#"<div class="lu-text-muted" style="font-weight:600;">Source</div>"#))
            .add(code(snippet)))
}

/// Convenience for building a docs-page "sub-section" — a titled `section()`
/// containing a responsive grid of `example()` tiles. This is the ONLY
/// grouping pattern used by the doc pages, so it's worth having as a
/// single call.
///
/// ```ignore
/// doc_section("Row", "Horizontal flex, wraps by default.", vec![
///     example("row().gap(Gap::Sm)", "…", demo, code),
///     example("row().gap(Gap::Xl)", "…", demo, code),
/// ])
/// ```
pub fn doc_section(title: &str, subtitle: &str, examples: Vec<Card>) -> Section {
    let mut g = grid().cols_min(MinCol::W320).gap(Gap::Md);
    for e in examples { g = g.add(e); }
    section().title(title.to_string()).subtitle(subtitle.to_string()).add(g)
}
