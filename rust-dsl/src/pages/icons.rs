//! `/dsl/icons` — visual catalogue of every icon available in the DSL.
//!
//! Renders every `Icons::*` constant in a responsive grid so you can eyeball
//! the whole set in one screen. Also useful for spotting broken icons:
//! any icon rendered as a bright-red ✕ box is unknown to `ui-icon.js`
//! (see the `HELP_PATH` fallback there).
//!
//! To add a new icon:
//!   1. Add its SVG path to `PATHS` in `lit-components/components/ui-icon.js`.
//!   2. Add a `pub const NAME: IconName = IconName("jsKey");` to
//!      `rust-dsl/src/components/icon.rs`.
//!   3. Add a `(name_str, "NAME")` tuple to the `ICON_CATALOGUE` list below.

use crate::prelude::*;

/// Every icon in the catalogue, grouped by section. `.0` is the underlying
/// JS key (what `<ui-icon name="…">` uses); `.1` is the Rust constant name.
///
/// Keep this in-sync with `Icons::*` — the page won't compile if a name
/// here doesn't exist in the catalogue, thanks to `Icons::NAME` in `.icon()`.
fn sections() -> Vec<(&'static str, Vec<(&'static str, IconName)>)> {
    vec![
        ("Actions", vec![
            ("CHECK",    Icons::CHECK),
            ("X",        Icons::X),
            ("PLUS",     Icons::PLUS),
            ("MINUS",    Icons::MINUS),
            ("EDIT",     Icons::EDIT),
            ("UPLOAD",   Icons::UPLOAD),
            ("DOWNLOAD", Icons::DOWNLOAD),
            ("SAVE",     Icons::SAVE),
            ("DELETE",   Icons::DELETE),
            ("SEARCH",   Icons::SEARCH),
            ("FILTER",   Icons::FILTER),
            ("SETTINGS", Icons::SETTINGS),
            ("REFRESH",  Icons::REFRESH),
            ("MORE",     Icons::MORE),
            ("MENU",     Icons::MENU),
        ]),
        ("Feedback", vec![
            ("INFO",    Icons::INFO),
            ("WARNING", Icons::WARNING),
            ("BELL",    Icons::BELL),
        ]),
        ("Navigation", vec![
            ("HOME",          Icons::HOME),
            ("CHEVRON_UP",    Icons::CHEVRON_UP),
            ("CHEVRON_DOWN",  Icons::CHEVRON_DOWN),
            ("CHEVRON_LEFT",  Icons::CHEVRON_LEFT),
            ("CHEVRON_RIGHT", Icons::CHEVRON_RIGHT),
            ("ARROW_UP",      Icons::ARROW_UP),
            ("ARROW_DOWN",    Icons::ARROW_DOWN),
            ("ARROW_LEFT",    Icons::ARROW_LEFT),
            ("ARROW_RIGHT",   Icons::ARROW_RIGHT),
        ]),
        ("Theme", vec![
            ("SUN",  Icons::SUN),
            ("MOON", Icons::MOON),
        ]),
        ("Domain", vec![
            ("USERS",     Icons::USERS),
            ("STUDENT",   Icons::STUDENT),
            ("CALENDAR",  Icons::CALENDAR),
            ("MESSAGE",   Icons::MESSAGE),
            ("BOOKMARK",  Icons::BOOKMARK),
            ("LIBRARY",   Icons::LIBRARY),
            ("CLIPBOARD", Icons::CLIPBOARD),
            ("CARD",      Icons::CARD),
            ("WALLET",    Icons::WALLET),
            ("CHART",     Icons::CHART),
            ("GRID",      Icons::GRID),
            ("ACTIVITY",  Icons::ACTIVITY),
            ("MAIL",      Icons::MAIL),
            ("CLOCK",     Icons::CLOCK),
            ("FILE",      Icons::FILE),
            ("STAR",      Icons::STAR),
        ]),
        ("Deliberate broken example", vec![
            // Passes a raw string that doesn't exist in PATHS to demonstrate
            // the loud red ✕ fallback. Delete this entry before shipping.
            ("(fallback demo)", IconName("this-icon-does-not-exist")),
        ]),
    ]
}

/// Render one icon tile: 32-px icon + Rust constant name + JS key.
fn tile(rust_name: &str, name: IconName) -> Card {
    card()
        .add(column().align(Align::Center).gap(Gap::Sm)
            .add(icon(name).size(32))
            .add(Node::raw(format!(
                r#"<code style="font-size:12px;font-weight:600">{}</code>"#,
                crate::core::escape_html(rust_name),
            )))
            .add(Node::raw(format!(
                r#"<code class="lu-text-muted">"{}"</code>"#,
                crate::core::escape_html(name.as_str()),
            ))))
}

pub fn build() -> Page {
    let mut body = page_shell()
        .add(toolbar()
            .add(breadcrumb()
                .item(Crumb::link("DSL", "/dsl"))
                .item(Crumb::current("Icons")))
            .add(spacer())
            .add(button().label("Layout guide").variant(Variant::Secondary)
                .icon(Icons::GRID)));

    body = body.add(card()
        .add(Node::raw(
            "<p style=\"margin:0\">Every icon exposed by the DSL is listed below. \
             Use the typed constant (<code>Icons::CHECK</code>) — it autocompletes \
             and won't compile if misspelled. The quoted string underneath is the \
             underlying <code>&lt;ui-icon name=…&gt;</code> key from \
             <code>ui-icon.js</code>. A bright-red ✕ means the icon key is unknown \
             — see the last section for a live demo.</p>",
        )));

    for (heading, items) in sections() {
        let tiles = grid().cols_min(MinCol::W200).gap(Gap::Md);
        let tiles = items.into_iter().fold(tiles, |g, (rn, n)| g.add(tile(rn, n)));
        body = body.add(section().title(heading).add(tiles));
    }

    page_of("Icons · DSL catalogue", body)
}
