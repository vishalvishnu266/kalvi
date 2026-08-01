//! `/dsl/components` — visual catalogue of every UI component.
//!
//! One page you can bookmark to see every `<ui-*>` component the DSL
//! exposes, in all its meaningful variants, with the exact Rust code
//! that produced each one. Use it for:
//!
//!   * Onboarding — new engineers see the whole kit in five minutes.
//!   * Design review — designers can point at things instead of guessing.
//!   * Regression QA — bump a component, refresh, eyeball everything.
//!
//! Uses the shared `pages::doc_helpers` (`example`, `doc_section`, `code`).
//! Every section groups related components together. Add new components
//! by dropping another `doc_section` in `build()` — no other wiring needed.

use crate::pages::doc_helpers::*;
use crate::prelude::*;

pub fn build() -> Page {
    let mut body = page_shell()
        .add(toolbar()
            .add(breadcrumb()
                .item(Crumb::link("DSL", "/dsl"))
                .item(Crumb::current("Components")))
            .add(spacer())
            .add(button().label("Layout guide").variant(Variant::Secondary).icon(Icons::GRID))
            .add(button().label("Icons").variant(Variant::Secondary).icon(Icons::STAR)));

    body = body.add(card().add(Node::raw(
        "<p style=\"margin:0\">Every UI component in the DSL, with every \
         meaningful variant and the exact Rust code that produced it. \
         When building a new page, first check whether the component you \
         want already exists here — chances are it does. If a variant is \
         missing, prefer adding it here (and to the underlying component) \
         over reinventing it in a page file.</p>",
    )));

    // Body sections are appended one-by-one below via find_and_replace so
    // this file can grow without blowing the create_file size limit.
    body = body.add(section_buttons());
    body = body.add(section_inputs());
    body = body.add(section_selection());
    body = body.add(section_cards_and_badges());
    body = body.add(section_stats_and_progress());
    body = body.add(section_avatars());
    body = body.add(section_lists_and_menus());
    body = body.add(section_navigation());
    body = body.add(section_overlays());
    body = body.add(section_data_display());
    body = body.add(section_forms_and_feedback());

    page_of("Components · DSL catalogue", body)
}

// ---------------------------------------------------------------------------
// Buttons
// ---------------------------------------------------------------------------
fn section_buttons() -> Section {
    doc_section("Buttons", "Variants, sizes, icons, disabled state, and full-width.", vec![
        example("Variants",
                "Primary, Secondary, Ghost, Danger — the four semantic tones.",
                row_actions()
                    .add(button().label("Primary").variant(Variant::Primary))
                    .add(button().label("Secondary").variant(Variant::Secondary))
                    .add(button().label("Ghost").variant(Variant::Ghost))
                    .add(button().label("Danger").variant(Variant::Danger)),
                "button().label(\"Primary\").variant(Variant::Primary)\nbutton().label(\"Secondary\").variant(Variant::Secondary)\nbutton().label(\"Ghost\").variant(Variant::Ghost)\nbutton().label(\"Danger\").variant(Variant::Danger)"),
        example("Sizes",
                "Sm, Md (default), Lg — same variant, three sizes.",
                row_actions()
                    .add(button().label("Small").variant(Variant::Primary).size(Size::Sm))
                    .add(button().label("Medium").variant(Variant::Primary))
                    .add(button().label("Large").variant(Variant::Primary).size(Size::Lg)),
                "button().label(\"Small\").size(Size::Sm)\nbutton().label(\"Medium\")            // default = Md\nbutton().label(\"Large\").size(Size::Lg)"),
        example("With icons",
                "Every button can carry a typed Icons::* constant.",
                row_actions()
                    .add(button().label("Save").variant(Variant::Primary).icon(Icons::CHECK))
                    .add(button().label("Delete").variant(Variant::Danger).icon(Icons::DELETE))
                    .add(button().label("Upload").variant(Variant::Secondary).icon(Icons::UPLOAD))
                    .add(button().label("Filter").variant(Variant::Ghost).icon(Icons::FILTER)),
                "button().label(\"Save\").variant(Variant::Primary).icon(Icons::CHECK)"),
        example("Disabled",
                "Use .disabled() for non-interactive states.",
                row_actions()
                    .add(button().label("Disabled Primary").variant(Variant::Primary).disabled())
                    .add(button().label("Disabled Secondary").variant(Variant::Secondary).disabled()),
                "button().label(\"…\").disabled()"),
        example("Full width",
                "For form footers and mobile CTAs — use inside a column.",
                column().gap(Gap::Sm)
                    .add(button().label("Full Primary").variant(Variant::Primary).full())
                    .add(button().label("Full Secondary").variant(Variant::Secondary).full()),
                "button().label(\"Full Primary\").variant(Variant::Primary).full()"),
    ])
}

// ---------------------------------------------------------------------------
// Inputs
// ---------------------------------------------------------------------------
fn section_inputs() -> Section {
    doc_section("Inputs", "Text, email, number, password, textarea — with hints, placeholders and required flag.", vec![
        example("Text input",
                "Basic label + placeholder.",
                input().label("Full name").name("name").placeholder("e.g. Aarav Kumar"),
                "input().label(\"Full name\").name(\"name\").placeholder(\"e.g. Aarav Kumar\")"),
        example("Required + hint",
                ".required() adds the asterisk, .hint() adds helper text below.",
                input().label("Guardian email").name("email")
                    .kind(InputType::Email).required().hint("We'll email fee reminders here."),
                "input().label(\"Guardian email\").name(\"email\")\n    .kind(InputType::Email).required()\n    .hint(\"We'll email fee reminders here.\")"),
        example("Number",
                "InputType::Number restricts to numeric input on all platforms.",
                input().label("Amount (₹)").name("amount").kind(InputType::Number),
                "input().label(\"Amount (₹)\").name(\"amount\").kind(InputType::Number)"),
        example("Password",
                "InputType::Password masks the value.",
                input().label("Password").name("password").kind(InputType::Password).required(),
                "input().label(\"Password\").kind(InputType::Password)"),
        example("Textarea",
                "For long-form entries. Auto-resizes.",
                input().label("Notes").name("notes").kind(InputType::Textarea).hint("Optional"),
                "input().label(\"Notes\").kind(InputType::Textarea).hint(\"Optional\")"),
    ])
}

// ---------------------------------------------------------------------------
// Selection controls
// ---------------------------------------------------------------------------
fn section_selection() -> Section {
    doc_section("Selection", "Select, combobox, radio group, checkbox, switch, segmented.", vec![
        example("Select",
                "Native-feeling <select> with an options list.",
                select().label("Grade").name("grade").placeholder("Choose…")
                    .option(SelectOption::new("5", "Grade 5"))
                    .option(SelectOption::new("6", "Grade 6"))
                    .option(SelectOption::new("7", "Grade 7")),
                "select().label(\"Grade\").placeholder(\"Choose…\")\n    .option(SelectOption::new(\"5\", \"Grade 5\"))\n    …"),
        example("Combobox",
                "Type-to-search picker for large option lists.",
                combobox().label("Student").name("student").placeholder("Search students…")
                    .option(ComboOption::new("aarav", "Aarav Kumar"))
                    .option(ComboOption::new("meera", "Meera Sharma")),
                "combobox().label(\"Student\").placeholder(\"Search students…\")\n    .option(ComboOption::new(\"aarav\", \"Aarav Kumar\"))\n    …"),
        example("Radio group (horizontal)",
                "Single-choice, layout as a row.",
                radio_group("gender").value("F").horizontal()
                    .option(radio("M", "Male"))
                    .option(radio("F", "Female"))
                    .option(radio("X", "Prefer not to say")),
                "radio_group(\"gender\").value(\"F\").horizontal()\n    .option(radio(\"M\", \"Male\"))\n    .option(radio(\"F\", \"Female\"))\n    .option(radio(\"X\", \"Prefer not to say\"))"),
        example("Checkbox",
                "Boolean opt-in. .required() adds the asterisk.",
                checkbox("I consent to the school's data policy").name("consent").required(),
                "checkbox(\"I consent to the school's data policy\").required()"),
        example("Switch",
                "On/off toggle — use for settings that take effect immediately.",
                switch("Send onboarding email").name("notify").checked(),
                "switch(\"Send onboarding email\").checked()"),
        example("Segmented control",
                "Mutually-exclusive short options; great for status.",
                segmented().value("present")
                    .segment(Segment::new("present", "P"))
                    .segment(Segment::new("late",    "L"))
                    .segment(Segment::new("absent",  "A")),
                "segmented().value(\"present\")\n    .segment(Segment::new(\"present\", \"P\"))\n    .segment(Segment::new(\"late\",    \"L\"))\n    .segment(Segment::new(\"absent\",  \"A\"))"),
    ])
}

// ---------------------------------------------------------------------------
// Cards and badges
// ---------------------------------------------------------------------------
fn section_cards_and_badges() -> Section {
    doc_section("Cards & Badges", "The card is padded by default; flush() removes padding for edge-to-edge media.", vec![
        example("Card (padded, default)",
                "Every card is padded unless you opt out with .flush().",
                card().title("Attendance").subtitle("This week")
                    .add(Node::raw("Body text goes here.")),
                "card().title(\"Attendance\").subtitle(\"This week\")\n    .add(Node::raw(\"Body text goes here.\"))"),
        example("Card with action",
                ".action() places a component in the header's actions slot.",
                card().title("Recent activity")
                    .action(button().label("See all").variant(Variant::Ghost).size(Size::Sm).icon(Icons::CHEVRON_RIGHT))
                    .add(Node::raw("List content…")),
                "card().title(\"Recent activity\")\n    .action(button().label(\"See all\").variant(Variant::Ghost).size(Size::Sm))\n    .add(list)"),
        example("Card (flush)",
                "No padding — use for full-bleed images or embedded tables.",
                card().flush().add(Node::raw(r#"<div style="background:var(--color-primary);color:#fff;padding:16px;text-align:center;">Full-bleed content</div>"#)),
                "card().flush().add(image_or_table)"),
        example("Badge tones",
                "Neutral, Brand, Success, Warning, Danger, Info.",
                row_actions()
                    .add(badge("neutral").tone(Tone::Neutral))
                    .add(badge("brand").tone(Tone::Brand))
                    .add(badge("success").tone(Tone::Success))
                    .add(badge("warning").tone(Tone::Warning))
                    .add(badge("danger").tone(Tone::Danger))
                    .add(badge("info").tone(Tone::Info)),
                "badge(\"success\").tone(Tone::Success)"),
        example("Badge with dot",
                "The .dot() flag adds a small filled circle before the label.",
                row_actions()
                    .add(badge("active").tone(Tone::Success).dot())
                    .add(badge("pending").tone(Tone::Warning).dot())
                    .add(badge("blocked").tone(Tone::Danger).dot()),
                "badge(\"active\").tone(Tone::Success).dot()"),
    ])
}

// ---------------------------------------------------------------------------
// Stats and Progress
// ---------------------------------------------------------------------------
fn section_stats_and_progress() -> Section {
    doc_section("Stats & Progress", "KPI tiles and progress bars for at-a-glance summaries.", vec![
        example("Stat with trend up",
                "Icon + big number + delta with an up-arrow.",
                stat("Total students", "1,248").icon(Icons::USERS).trend(Trend::Up).delta("+2.1%"),
                "stat(\"Total students\", \"1,248\")\n    .icon(Icons::USERS).trend(Trend::Up).delta(\"+2.1%\")"),
        example("Stat with trend down",
                "Same, but with a down-arrow (negative delta usually red).",
                stat("Overdue count", "17").icon(Icons::X).trend(Trend::Down).delta("-3"),
                "stat(\"Overdue count\", \"17\")\n    .icon(Icons::X).trend(Trend::Down).delta(\"-3\")"),
        example("Stat without trend",
                "Just the number + label — for KPIs with no comparison.",
                stat("Roll strength", "42").icon(Icons::CLIPBOARD),
                "stat(\"Roll strength\", \"42\").icon(Icons::CLIPBOARD)"),
        example("Progress tones",
                "Brand, Success, Warning, Danger, Info.",
                column().gap(Gap::Sm)
                    .add(progress(72).label("Syllabus").tone(ProgTone::Success).show_value())
                    .add(progress(45).label("Warning zone").tone(ProgTone::Warning).show_value())
                    .add(progress(18).label("Overdue").tone(ProgTone::Danger).show_value())
                    .add(progress(90).label("Attendance").tone(ProgTone::Info).show_value()),
                "progress(72).label(\"Syllabus\").tone(ProgTone::Success).show_value()"),
    ])
}

// ---------------------------------------------------------------------------
// Avatars
// ---------------------------------------------------------------------------
fn section_avatars() -> Section {
    doc_section("Avatars", "Single avatars in three sizes, plus grouped avatars.", vec![
        example("Avatar sizes",
                "Sm, Md (default), Lg — auto-generates initials from the name.",
                row().gap(Gap::Md).align(Align::Center)
                    .add(avatar("Aarav Kumar").size(AvatarSize::Sm))
                    .add(avatar("Meera Sharma"))
                    .add(avatar("Rohan Patel").size(AvatarSize::Lg)),
                "avatar(\"Aarav Kumar\").size(AvatarSize::Sm)\navatar(\"Meera Sharma\")                 // default = Md\navatar(\"Rohan Patel\").size(AvatarSize::Lg)"),
        example("Avatar group",
                "Overlapping stack for attendee lists / assignees.",
                avatar_group()
                    .add(avatar("Aarav Kumar"))
                    .add(avatar("Meera Sharma"))
                    .add(avatar("Rohan Patel"))
                    .add(avatar("Diya Verma"))
                    .add(avatar("Ishaan Thakur")),
                "avatar_group()\n    .add(avatar(\"Aarav Kumar\"))\n    .add(avatar(\"Meera Sharma\"))\n    …"),
    ])
}

// ---------------------------------------------------------------------------
// Lists and menus
// ---------------------------------------------------------------------------
fn section_lists_and_menus() -> Section {
    doc_section("Lists & Menus", "List items, dropdown menus, command palette.", vec![
        example("List item with avatar + trailing",
                ".leading() takes any component; .trailing() puts one on the right.",
                card().add(
                    list_item("Aarav Kumar")
                        .subtitle("Roll 12 · Grade 5")
                        .leading(avatar("Aarav Kumar").size(AvatarSize::Sm))
                        .trailing(badge("present").tone(Tone::Success).dot())),
                "list_item(\"Aarav Kumar\")\n    .subtitle(\"Roll 12 · Grade 5\")\n    .leading(avatar(\"Aarav Kumar\").size(AvatarSize::Sm))\n    .trailing(badge(\"present\").tone(Tone::Success).dot())"),
        example("Dropdown menu",
                "Trigger + items + optional divider between groups.",
                dropdown_menu().align(MenuAlign::End)
                    .trigger(button().label("Actions").variant(Variant::Secondary).icon(Icons::MORE))
                    .item(MenuItem::action("New announcement"))
                    .item(MenuItem::link("Add student", "/dsl/students"))
                    .divider()
                    .item(MenuItem::action("Export PDF")),
                "dropdown_menu().align(MenuAlign::End)\n    .trigger(button().label(\"Actions\").icon(Icons::MORE))\n    .item(MenuItem::action(\"New announcement\"))\n    .item(MenuItem::link(\"Add student\", \"/dsl/students\"))\n    .divider()\n    .item(MenuItem::action(\"Export PDF\"))"),
        example("Command palette (⌘K style)",
                "Fuzzy-matched action launcher — trigger by any button.",
                command()
                    .item(command_item("Add student"))
                    .item(command_item("Create invoice"))
                    .item(command_item("Mark attendance"))
                    .item(command_item("Export PDF")),
                "command()\n    .item(command_item(\"Add student\"))\n    .item(command_item(\"Create invoice\"))\n    …"),
    ])
}

// ---------------------------------------------------------------------------
// Navigation
// ---------------------------------------------------------------------------
fn section_navigation() -> Section {
    doc_section("Navigation", "Breadcrumb, tab bar, pagination, stepper.", vec![
        example("Breadcrumb",
                "Link + link + current — the standard toolbar breadcrumb.",
                breadcrumb()
                    .item(Crumb::link("Home", "#/"))
                    .item(Crumb::link("Students", "#/students"))
                    .item(Crumb::current("Grade 5")),
                "breadcrumb()\n    .item(Crumb::link(\"Home\", \"#/\"))\n    .item(Crumb::link(\"Students\", \"#/students\"))\n    .item(Crumb::current(\"Grade 5\"))"),
        example("Tab bar",
                "Horizontal tabs with an active state (mark each Tab with .active() as needed).",
                tab_bar()
                    .tab(Tab::new("Overview", "#/overview").active())
                    .tab(Tab::new("Roster",   "#/roster"))
                    .tab(Tab::new("Grades",   "#/grades"))
                    .tab(Tab::new("Fees",     "#/fees")),
                "tab_bar()\n    .tab(Tab::new(\"Overview\", \"#/overview\").active())\n    .tab(Tab::new(\"Roster\",   \"#/roster\"))\n    …"),
        example("Pagination",
                "Standard page-through control for tables / lists. pagination(current_page, total_pages).",
                pagination(3, 12),
                "pagination(3, 12)   // (current_page, total_pages)"),
        example("Stepper (horizontal)",
                "Multi-step workflow with completed / active / upcoming.",
                stepper().orientation(StepperOrientation::Horizontal).current(1)
                    .step("Details").step("Guardians").step("Consent").step("Confirm"),
                "stepper().orientation(StepperOrientation::Horizontal).current(1)\n    .step(\"Details\").step(\"Guardians\").step(\"Consent\").step(\"Confirm\")"),
    ])
}

// ---------------------------------------------------------------------------
// Overlays
// ---------------------------------------------------------------------------
fn section_overlays() -> Section {
    doc_section("Overlays", "Tooltip, modal, drawer, toast host — trigger via JS/user actions.", vec![
        example("Tooltip",
                "Hover the badge to see it. tooltip(text) takes the tooltip text; wrap the target with .add(...).",
                tooltip("Click to view invoice details").placement(Placement::Top)
                    .add(badge("INV-1042").tone(Tone::Brand)),
                "tooltip(\"Click to view invoice details\").placement(Placement::Top)\n    .add(badge(\"INV-1042\").tone(Tone::Brand))"),
        example("Modal (structure only)",
                "Rendered here so you can inspect the DOM. Real usage: open via JS. Footer buttons go in .footer(...).",
                modal().id("demo-modal").title("Confirm delete")
                    .add(Node::raw("<p>Are you sure you want to delete this invoice?</p>"))
                    .footer(button().label("Cancel").variant(Variant::Secondary))
                    .footer(button().label("Delete").variant(Variant::Danger).icon(Icons::DELETE)),
                "modal().id(\"demo-modal\").title(\"Confirm delete\")\n    .add(body)\n    .footer(button().label(\"Cancel\").variant(Variant::Secondary))\n    .footer(button().label(\"Delete\").variant(Variant::Danger).icon(Icons::DELETE))"),
        example("Drawer (structure only)",
                "Slide-in panel. Real usage: open via JS on button click.",
                drawer().id("demo-drawer").title("Add note")
                    .placement(DrawerPlacement::Right).size(DrawerSize::Md)
                    .add(input().label("Reason").placeholder("e.g. Medical leave"))
                    .add(input().label("Details").kind(InputType::Textarea)),
                "drawer().id(\"demo-drawer\").title(\"Add note\")\n    .placement(DrawerPlacement::Right).size(DrawerSize::Md)\n    .add(input()…)"),
        example("Toast host",
                "One toast_host() per page — JS fires notifications into it.",
                toast_host(),
                "toast_host()   // add once at the bottom of every page"),
    ])
}

// ---------------------------------------------------------------------------
// Data display
// ---------------------------------------------------------------------------
fn section_data_display() -> Section {
    doc_section("Data display", "Table (server-rendered), data_table (client-interactive), kanban, timeline.", vec![
        example("Simple table",
                "Static server-rendered — good for reports and printouts. Use .column(key, label) and .column_aligned(...) for centred/right columns. Rows are simple string vectors in column order.",
                {
                    use crate::components::table::Align as TAlign;
                    table()
                        .column("name",  "Name")
                        .column_aligned("grade", "Grade", TAlign::Center)
                        .column_aligned("roll",  "Roll",  TAlign::Right)
                        .row(vec!["Aarav Kumar",  "5", "12"])
                        .row(vec!["Meera Sharma", "5", "13"])
                },
                "table()\n    .column(\"name\",  \"Name\")\n    .column_aligned(\"grade\", \"Grade\", Align::Center)\n    .column_aligned(\"roll\",  \"Roll\",  Align::Right)\n    .row(vec![\"Aarav Kumar\", \"5\", \"12\"])\n    .row(vec![\"Meera Sharma\", \"5\", \"13\"])"),
        example("Data table (interactive)",
                "Client-side sorting / searching / pagination / selection.",
                {
                    let mut t = data_table("demo-dt").searchable().selectable().per_page(3)
                        .col("name",  "Name",  ColOpts::text().sortable())
                        .col("grade", "Grade", ColOpts::text().sortable().center())
                        .col("roll",  "Roll",  ColOpts::text().sortable().right());
                    for (n, g, r) in [("Aarav Kumar", "5", "12"), ("Meera Sharma", "5", "13"), ("Rohan Patel", "5", "14"), ("Diya Verma", "6", "21")] {
                        t = t.row(vec![("name", n.to_string()), ("grade", g.to_string()), ("roll", r.to_string())]);
                    }
                    t
                },
                "data_table(\"demo-dt\").searchable().selectable().per_page(3)\n    .col(\"name\",  \"Name\",  ColOpts::text().sortable())\n    .col(\"grade\", \"Grade\", ColOpts::text().sortable().center())\n    .col(\"roll\",  \"Roll\",  ColOpts::text().sortable().right())\n    .row(vec![…])"),
        example("Kanban",
                "Drag-and-drop columns for workflows.",
                kanban()
                    .column(kanban_column("Applied")
                        .add(kanban_card().id("k1").text("Aarav · Grade 5"))
                        .add(kanban_card().id("k2").text("Meera · Grade 6")))
                    .column(kanban_column("Interview")
                        .add(kanban_card().id("k3").text("Diya · Aug 8")))
                    .column(kanban_column("Enrolled")),
                "kanban()\n    .column(kanban_column(\"Applied\")\n        .add(kanban_card().id(\"k1\").text(\"Aarav · Grade 5\"))\n        …)\n    .column(kanban_column(\"Interview\")…)"),
        example("Timeline",
                "Vertical activity feed with icon-per-event and tone.",
                timeline()
                    .item(timeline_item().icon(Icons::CHECK).tone(TimelineTone::Success).time("10:24 AM")
                        .add(text_body("Fees paid", "Invoice #INV-1042 · ₹4,500")))
                    .item(timeline_item().icon(Icons::EDIT).tone(TimelineTone::Info).time("Yesterday")
                        .add(text_body("Profile updated", "Guardian phone changed"))),
                "timeline()\n    .item(timeline_item().icon(Icons::CHECK).tone(TimelineTone::Success).time(\"10:24 AM\")\n        .add(text_body(\"Fees paid\", \"Invoice #INV-1042 · ₹4,500\")))"),
    ])
}

// ---------------------------------------------------------------------------
// Forms and feedback
// ---------------------------------------------------------------------------
fn section_forms_and_feedback() -> Section {
    doc_section("Forms & Feedback", "Form (with save_cancel), datepicker, daterange, file upload, empty state, skeleton, inline edit.", vec![
        example("Form with save_cancel()",
                "Preset footer: Cancel (secondary) + Save (primary, check icon).",
                form().action("/x").method("post")
                    .add(input().label("Full name").required())
                    .add(input().label("Email").kind(InputType::Email).required())
                    .save_cancel("Save student"),
                "form().action(\"/x\").method(\"post\")\n    .add(input()…)\n    .save_cancel(\"Save student\")"),
        example("Datepicker",
                "Single-date input with a native picker.",
                datepicker().label("Due date").name("due").value("2026-08-15"),
                "datepicker().label(\"Due date\").name(\"due\").value(\"2026-08-15\")"),
        example("Date range",
                "Start + end date, paired. Use .from(iso) / .to(iso) for the two boundaries.",
                date_range().label("Term").from("2026-08-01").to("2026-12-15"),
                "date_range().label(\"Term\").from(\"2026-08-01\").to(\"2026-12-15\")"),
        example("File upload",
                "Drag-and-drop with a click fallback.",
                file_upload().name("attachment").accept("image/*,.pdf").hint("PNG, JPG or PDF, up to 5 MB"),
                "file_upload().name(\"attachment\").accept(\"image/*,.pdf\").hint(\"…\")"),
        example("Inline edit",
                "Click the value to edit in place — great for grids. inline_edit(value) takes the initial value.",
                inline_edit("Aarav Kumar").kind(InlineKind::Text),
                "inline_edit(\"Aarav Kumar\").kind(InlineKind::Text)"),
        example("Empty state",
                "Show when a list / table has no rows yet. empty_state(title) takes the title; use .description(...) for the second line.",
                empty_state("No invoices yet")
                    .icon(Icons::CLIPBOARD)
                    .description("Create your first invoice to see it here.")
                    .action(button().label("New invoice").variant(Variant::Primary).icon(Icons::PLUS)),
                "empty_state(\"No invoices yet\")\n    .icon(Icons::CLIPBOARD)\n    .description(\"Create your first invoice…\")\n    .action(button().label(\"New invoice\").variant(Variant::Primary).icon(Icons::PLUS))"),
        example("Skeleton (loading placeholder)",
                "Show while data is being fetched — prevents layout jump. Shape variants: Line, Rect, Circle. .lines(n) stacks multiple rows.",
                column().gap(Gap::Sm)
                    .add(skeleton().shape(SkeletonShape::Line).lines(2))
                    .add(skeleton().shape(SkeletonShape::Rect).height("60px")),
                "skeleton().shape(SkeletonShape::Line).lines(2)\nskeleton().shape(SkeletonShape::Rect).height(\"60px\")"),
    ])
}
