//! `/dsl/layouts` — visual catalogue of every layout primitive & preset.
//!
//! Uses the shared helpers in `pages::doc_helpers` (`swatch`, `block`,
//! `code`, `example`, `doc_section`) so the layout code stays focused on
//! *what* to show, not *how* to render tiles.

use crate::pages::doc_helpers::*;
use crate::prelude::*;

pub fn build() -> Page {
    let mut body = page_shell()
        .add(toolbar()
            .add(breadcrumb()
                .item(Crumb::link("DSL", "/dsl"))
                .item(Crumb::current("Layout guide")))
            .add(spacer())
            .add(button().label("Icon catalogue").variant(Variant::Secondary).icon(Icons::STAR))
            .add(button().label("Components").variant(Variant::Secondary).icon(Icons::GRID)));

    body = body.add(card().add(Node::raw(
        "<p style=\"margin:0\">Every layout primitive and preset in the DSL, \
         with a live demo and the exact Rust code that produced it. Use this \
         page to pick the right building block for the job — and remember: \
         if a preset like <code>toolbar()</code> or <code>two_col()</code> \
         fits, use it instead of composing rows/columns from scratch. \
         Consistency &gt; cleverness.</p>",
    )));

    body = body.add(doc_section("1. Container", "Centered max-width wrapper. Every page's root.", vec![
        example("container().size(ContainerSize::Sm)",
                "640px max — narrow reading width for docs / focused forms.",
                container().size(ContainerSize::Sm).add(block("Sm — 640", 60)),
                "container().size(ContainerSize::Sm).add(...)"),
        example("container().size(ContainerSize::Lg)  ← default",
                "1200px max — the ERP standard used by page_of().",
                container().size(ContainerSize::Lg).add(block("Lg — 1200", 60)),
                "container().size(ContainerSize::Lg).add(...)"),
        example("container().fluid()",
                "Fills the viewport width — dashboards, kanbans, tables.",
                container().fluid().add(block("Fluid — 100%", 60)),
                "container().fluid().add(...)"),
    ]));

    body = body.add(doc_section("2. Row", "Horizontal flex, wraps by default.", vec![
        example("row().gap(Gap::Sm)",
                "Small gap between children.",
                row().gap(Gap::Sm).add(swatch(1)).add(swatch(2)).add(swatch(3)),
                "row().gap(Gap::Sm).add(a).add(b).add(c)"),
        example("row().gap(Gap::Xl)",
                "Extra-large gap.",
                row().gap(Gap::Xl).add(swatch(1)).add(swatch(2)).add(swatch(3)),
                "row().gap(Gap::Xl).add(a).add(b).add(c)"),
        example("row().justify(Justify::Center)",
                "All children clustered in the middle.",
                row().gap(Gap::Md).justify(Justify::Center).add(swatch(1)).add(swatch(2)).add(swatch(3)),
                "row().gap(Gap::Md).justify(Justify::Center)…"),
        example("row().justify(Justify::Between)",
                "Space distributed between siblings.",
                row().gap(Gap::Md).justify(Justify::Between).add(swatch(1)).add(swatch(2)).add(swatch(3)),
                "row().gap(Gap::Md).justify(Justify::Between)…"),
        example("row().align(Align::Center) + mixed heights",
                "Children of different heights share a centre line.",
                row().gap(Gap::Md).align(Align::Center)
                    .add(block("tall", 80)).add(block("short", 40)).add(block("mid", 60)),
                "row().gap(Gap::Md).align(Align::Center)…"),
        example("row().align(Align::Stretch)  ← default",
                "Children stretch to match the tallest sibling.",
                row().gap(Gap::Md).align(Align::Stretch)
                    .add(block("tall", 80)).add(block("short", 40)).add(block("mid", 60)),
                "row().gap(Gap::Md).align(Align::Stretch)…"),
    ]));

    body = body.add(doc_section("3. Column", "Vertical flex, evenly spaced.", vec![
        example("column().gap(Gap::Sm)",
                "Stacked vertically, tight spacing.",
                column().gap(Gap::Sm).add(swatch(1)).add(swatch(2)).add(swatch(3)),
                "column().gap(Gap::Sm).add(a).add(b).add(c)"),
        example("column().gap(Gap::Lg)",
                "Stacked vertically, generous spacing.",
                column().gap(Gap::Lg).add(swatch(1)).add(swatch(2)).add(swatch(3)),
                "column().gap(Gap::Lg).add(a).add(b).add(c)"),
    ]));

    body = body.add(doc_section("4. Grid", "Auto-fit responsive columns, no media queries.", vec![
        example("grid().cols_min(MinCol::W200)",
                "Cards reflow: as many as fit at ≥200 px each.",
                grid().cols_min(MinCol::W200).gap(Gap::Sm)
                    .add(swatch(1)).add(swatch(2)).add(swatch(3))
                    .add(swatch(4)).add(swatch(5)).add(swatch(6)),
                "grid().cols_min(MinCol::W200)\n    .add(a).add(b).add(c)"),
        example("grid().cols(Cols::Three)",
                "Always 3 equal columns (does NOT reflow — use with care).",
                grid().cols(Cols::Three).gap(Gap::Sm)
                    .add(swatch(1)).add(swatch(2)).add(swatch(3))
                    .add(swatch(4)).add(swatch(5)).add(swatch(6)),
                "grid().cols(Cols::Three)\n    .add(a).add(b).add(c)"),
    ]));

    body = body.add(doc_section("5. Spacer, row_actions, Divider", "Utility building blocks.", vec![
        example("row_actions() + spacer()",
                "Standard action bar preset. spacer() pushes trailing items to the right.",
                row_actions().add(swatch(1)).add(swatch(2)).add(spacer()).add(swatch(3)),
                "row_actions()\n    .add(save).add(cancel)\n    .add(spacer())\n    .add(delete)"),
        example("divider().vertical() inside a row",
                "Vertical separator to group buttons visually.",
                row_actions()
                    .add(swatch(1)).add(swatch(2))
                    .add(divider().vertical())
                    .add(swatch(3)),
                "row_actions()\n    .add(a).add(b)\n    .add(divider().vertical())\n    .add(c)"),
        example("divider()  ← horizontal",
                "Full-width 1px separator between block-flow siblings.",
                column().gap(Gap::Sm)
                    .add(block("above", 40))
                    .add(divider())
                    .add(block("below", 40)),
                "column().gap(Gap::Sm)\n    .add(above)\n    .add(divider())\n    .add(below)"),
    ]));

    body = body.add(doc_section("6. Responsive", "Resize your window to see these change.", vec![
        example("row().mobile_stack()",
                "Becomes a column on ≤640 px. Try shrinking the browser.",
                row().gap(Gap::Md).mobile_stack()
                    .add(block("Left", 60)).add(block("Middle", 60)).add(block("Right", 60)),
                "row().gap(Gap::Md).mobile_stack()\n    .add(left).add(middle).add(right)"),
        example("row().hide_at(Breakpoint::Mobile)",
                "The whole row disappears on ≤640 px.",
                row().gap(Gap::Md).hide_at(Breakpoint::Mobile)
                    .add(block("Hidden on mobile", 60)).add(swatch(1)).add(swatch(2)),
                "row().gap(Gap::Md).hide_at(Breakpoint::Mobile)…"),
    ]));

    body = body.add(doc_section("7. Presets — use these!",
        "Convention-over-configuration. Every page should compose from these.", vec![
        example("toolbar()",
                "Standard top toolbar: breadcrumb + spacer() + actions. Auto-stacks on mobile.",
                toolbar()
                    .add(block("breadcrumb", 40))
                    .add(spacer())
                    .add(swatch(1)).add(swatch(2)),
                "toolbar()\n    .add(breadcrumb…)\n    .add(spacer())\n    .add(button().label(\"Export\"))\n    .add(button().label(\"New\"))"),
        example("row_actions()",
                "Standard button bar. row().gap(Md).align(Center) under the hood.",
                row_actions().add(swatch(1)).add(swatch(2)).add(swatch(3)),
                "row_actions()\n    .add(save)\n    .add(cancel)\n    .add(delete)"),
        example("two_col(3, 1)",
                "Standard responsive 2-column body. Stacks on mobile.",
                two_col(3, 1)
                    .add(column().flex(3).min_w(MinW::W320).add(block("Main (flex 3)", 100)))
                    .add(column().flex(1).min_w(MinW::W280).add(side_block("Side (flex 1)", 100))),
                "two_col(3, 1)\n    .add(column().flex(3).min_w(MinW::W320).add(main))\n    .add(column().flex(1).min_w(MinW::W280).add(side))"),
        example("two_col_with(3, 1, main, side)",
                "Same as above but builds the two column slots for you.",
                two_col_with(3, 1, block("Main", 100), side_block("Side", 100)),
                "two_col_with(3, 1, main, side)"),
        example("page_shell() + page_of()",
                "The ONE outer wrapper every page uses. Wraps in container + column(Gap::Lg).",
                column().gap(Gap::Md).add(block("Whatever you put inside page_shell().add(...)", 60)),
                "pub fn build() -> Page {\n    page_of(\"Fees · ERP demo\",\n        page_shell()\n            .add(toolbar()…)\n            .add(kpis)\n            .add(two_col_with(3, 1, main, side))\n    )\n}"),
        example("text_body(title, subtitle)",
                "Standard \"strong + muted line\" text block. Used in timeline items, list items, activity feeds.",
                text_body("Fees paid", "Invoice #INV-1042 · ₹4,500"),
                "timeline_item().icon(Icons::CHECK)\n    .add(text_body(\"Fees paid\", \"Invoice #INV-1042 · ₹4,500\"))"),
    ]));

    page_of("Layout guide · DSL catalogue", body)
}
