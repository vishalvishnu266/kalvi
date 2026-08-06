//! Storybook content. Every story is a `Box<dyn Component>` built from
//! the `lit_ui` DSL and nothing else — no local wrapper types, no
//! `format!()` HTML strings, no `Node::raw`.
//!
//! Add a story: push another `story(name, component)` into any section.
//! The `/` route picks it up automatically.

use lit_ui::core::Component;
use lit_ui::prelude::*;

/// A single story: a short name + a fully-configured primitive.
pub type Story = (&'static str, Box<dyn Component>);

/// A group of related stories rendered under a shared heading.
pub struct Section {
    pub title:   &'static str,
    pub stories: Vec<Story>,
}

/// The complete storybook. Sections render in this order.
pub fn all_sections() -> Vec<Section> {
    vec![
        Section { title: "Button",       stories: button_stories()       },
        Section { title: "Input",        stories: input_stories()        },
        Section { title: "Select",       stories: select_stories()       },
        Section { title: "Checkbox",     stories: checkbox_stories()     },
        Section { title: "Radio",        stories: radio_stories()        },
        Section { title: "Switch",       stories: switch_stories()       },
        Section { title: "Datepicker",   stories: datepicker_stories()   },
        Section { title: "Date range",   stories: daterange_stories()    },
        Section { title: "Icon",         stories: icon_stories()         },
        Section { title: "Badge",        stories: badge_stories()        },
        Section { title: "Avatar",       stories: avatar_stories()       },
        Section { title: "Tooltip",      stories: tooltip_stories()      },
        Section { title: "Skeleton",     stories: skeleton_stories()     },
        Section { title: "Slider",       stories: slider_stories()       },
        Section { title: "Progress",     stories: progress_stories()     },
        Section { title: "Color swatch", stories: color_swatch_stories() },
        Section { title: "Theme toggle", stories: theme_toggle_stories() },
        Section { title: "Heading",      stories: heading_stories()      },
        Section { title: "Layout",       stories: layout_stories()       },
    ]
}

// ── helpers ─────────────────────────────────────────────────────────────

fn story<C: Component + 'static>(name: &'static str, c: C) -> Story { (name, Box::new(c)) }

// ── sections ────────────────────────────────────────────────────────────

fn button_stories() -> Vec<Story> {
    vec![
        story("default", button().label("Save")),
        story("variants", cluster_row()
            .add(button().label("Primary"))
            .add(button().label("Secondary").variant(Variant::Secondary))
            .add(button().label("Ghost").variant(Variant::Ghost))
            .add(button().label("Danger").variant(Variant::Danger))),
        story("sizes", cluster_row()
            .add(button().label("Small").size(Size::Sm))
            .add(button().label("Medium").size(Size::Md))
            .add(button().label("Large").size(Size::Lg))),
        story("with icon", cluster_row()
            .add(button().label("Save").icon(Icons::CHECK))
            .add(button().label("Delete").icon(Icons::DELETE).variant(Variant::Danger))
            .add(button().label("Upload").icon(Icons::UPLOAD).variant(Variant::Secondary))),
        story("states", cluster_row()
            .add(button().label("Normal"))
            .add(button().label("Disabled").disabled())
            .add(button().label("Full width").full())),
        story("submit / reset", cluster_row()
            .add(button().label("Submit").submit())
            .add(button().label("Reset").reset().variant(Variant::Ghost))),
    ]
}

fn input_stories() -> Vec<Story> {
    vec![
        story("default",    input().label("Full name").placeholder("Aarav")),
        story("required",   input().label("Email").kind(InputType::Email).required()),
        story("password",   input().label("Password").kind(InputType::Password)),
        story("with hint",  input().label("Handle").hint("3–20 chars, a–z 0–9 _")),
        story("with error", input().label("Handle").error("Required")),
        story("with icons", input().label("Search")
            .icon_leading(Icons::SEARCH).icon_trailing(Icons::X)),
    ]
}

fn select_stories() -> Vec<Story> {
    vec![
        story("default", select().label("Fruit")
            .option(SelectOption::new("a", "Apples"))
            .option(SelectOption::new("b", "Bananas"))
            .option(SelectOption::new("c", "Cherries"))),
        story("searchable + clearable", select()
            .label("Country").placeholder("Choose one").searchable().clearable()
            .option(SelectOption::new("in", "India"))
            .option(SelectOption::new("us", "United States"))
            .option(SelectOption::new("uk", "United Kingdom"))),
        story("multi + allow-new", select()
            .label("Tags").placeholder("Type to add").multiple().allow_new()),
        story("with error", select().label("Priority").error("Please pick one")
            .option(SelectOption::new("hi", "High"))
            .option(SelectOption::new("lo", "Low"))),
    ]
}

fn checkbox_stories() -> Vec<Story> {
    vec![
        story("default",       checkbox("I agree")),
        story("checked",       checkbox("Enable telemetry").checked()),
        story("indeterminate", checkbox("Select all").indeterminate()),
        story("disabled",      checkbox("Locked").disabled().checked()),
        story("with error",    checkbox("Terms").error("Required to continue")),
    ]
}

fn radio_stories() -> Vec<Story> {
    vec![
        story("horizontal", radio_group("fruit").value("a").horizontal()
            .option(radio("a", "Apples"))
            .option(radio("b", "Bananas"))
            .option(radio("c", "Cherries"))),
        story("vertical", radio_group("plan").value("pro")
            .option(radio("free",   "Free tier"))
            .option(radio("pro",    "Pro"))
            .option(radio("teams",  "Teams"))
            .option(radio("enterp", "Enterprise").disabled())),
        story("with error", radio_group("x").error("Pick one")
            .option(radio("y", "Yes"))
            .option(radio("n", "No"))),
    ]
}

fn switch_stories() -> Vec<Story> {
    vec![
        story("default",  switch("Notifications")),
        story("checked",  switch("Marketing e-mails").checked()),
        story("disabled", switch("Locked").disabled().checked()),
    ]
}

fn datepicker_stories() -> Vec<Story> {
    vec![
        story("default",    datepicker().label("Date of birth")),
        story("prefilled",  datepicker().label("Joined").value("2024-01-15")),
        story("with error", datepicker().label("Start").error("Required")),
    ]
}

fn daterange_stories() -> Vec<Story> {
    vec![
        story("default", date_range().label("Report period")),
        story("filled",  date_range().label("Q1").from("2024-01-01").to("2024-03-31")),
    ]
}

fn icon_stories() -> Vec<Story> {
    vec![
        story("common actions", cluster_row()
            .add(icon(Icons::CHECK)).add(icon(Icons::X)).add(icon(Icons::PLUS))
            .add(icon(Icons::EDIT)).add(icon(Icons::DELETE)).add(icon(Icons::SEARCH))
            .add(icon(Icons::FILTER)).add(icon(Icons::SETTINGS)).add(icon(Icons::REFRESH))),
        story("navigation", cluster_row()
            .add(icon(Icons::HOME)).add(icon(Icons::CHEVRON_LEFT))
            .add(icon(Icons::CHEVRON_RIGHT)).add(icon(Icons::ARROW_UP))
            .add(icon(Icons::ARROW_DOWN))),
        story("sizes", cluster_row()
            .add(icon(Icons::STAR).size(12)).add(icon(Icons::STAR).size(18))
            .add(icon(Icons::STAR).size(24)).add(icon(Icons::STAR).size(36))),
    ]
}

fn badge_stories() -> Vec<Story> {
    vec![
        story("tones", cluster_row()
            .add(badge("Neutral"))
            .add(badge("Brand").tone(Tone::Brand))
            .add(badge("Success").tone(Tone::Success))
            .add(badge("Warning").tone(Tone::Warning))
            .add(badge("Danger").tone(Tone::Danger))
            .add(badge("Info").tone(Tone::Info))),
        story("with dot", cluster_row()
            .add(badge("Live").tone(Tone::Success).dot())
            .add(badge("Draft").dot())
            .add(badge("Overdue").tone(Tone::Danger).dot())),
    ]
}

fn avatar_stories() -> Vec<Story> {
    vec![
        story("sizes", cluster_row()
            .add(avatar("Ada Lovelace").size(AvatarSize::Sm))
            .add(avatar("Ada Lovelace").size(AvatarSize::Md))
            .add(avatar("Ada Lovelace").size(AvatarSize::Lg))
            .add(avatar("Ada Lovelace").size(AvatarSize::Xl))),
        story("initials from name", cluster_row()
            .add(avatar("Grace Hopper"))
            .add(avatar("Alan Turing"))
            .add(avatar("Kernighan"))),
    ]
}

fn tooltip_stories() -> Vec<Story> {
    vec![
        story("placements", cluster_row()
            .add(tooltip("Top").add(button().label("Top")))
            .add(tooltip("Bottom").placement(Placement::Bottom).add(button().label("Bottom")))
            .add(tooltip("Left").placement(Placement::Left).add(button().label("Left")))
            .add(tooltip("Right").placement(Placement::Right).add(button().label("Right")))),
    ]
}

fn skeleton_stories() -> Vec<Story> {
    vec![
        story("line",       skeleton().width("240px")),
        story("rect",       skeleton().shape(SkeletonShape::Rect).width("240px").height("120px")),
        story("circle",     skeleton().shape(SkeletonShape::Circle).width("48px").height("48px")),
        story("multi-line", skeleton().lines(3).width("360px")),
    ]
}

fn slider_stories() -> Vec<Story> {
    vec![
        story("default",              slider().value(42).show_value()),
        story("custom bounds + step", slider().min(-20).max(20).step(2).value(0).show_value()),
        story("disabled",             slider().value(30).show_value().disabled()),
    ]
}

fn progress_stories() -> Vec<Story> {
    vec![
        story("tones", stack().gap(Gap::Sm)
            .add(progress().value(25))
            .add(progress().value(50).tone(ProgressTone::Brand))
            .add(progress().value(75).tone(ProgressTone::Success))
            .add(progress().value(60).tone(ProgressTone::Warning))
            .add(progress().value(80).tone(ProgressTone::Danger))),
        story("with label",    progress().value(65).tone(ProgressTone::Brand).label("Uploading photo")),
        story("indeterminate", progress().indeterminate().tone(ProgressTone::Brand)),
    ]
}

fn color_swatch_stories() -> Vec<Story> {
    vec![
        story("palette", cluster_row()
            .add(color_swatch("#ef4444")).add(color_swatch("#f97316"))
            .add(color_swatch("#eab308")).add(color_swatch("#22c55e"))
            .add(color_swatch("#06b6d4")).add(color_swatch("#3b82f6"))
            .add(color_swatch("#8b5cf6")).add(color_swatch("#ec4899"))),
        story("sizes", cluster_row()
            .add(color_swatch("#4f46e5").size(SwatchSize::Sm))
            .add(color_swatch("#4f46e5").size(SwatchSize::Md))
            .add(color_swatch("#4f46e5").size(SwatchSize::Lg))),
        story("selectable + selected", cluster_row()
            .add(color_swatch("#ef4444").selectable())
            .add(color_swatch("#22c55e").selectable().selected())
            .add(color_swatch("#3b82f6").selectable())),
    ]
}

fn theme_toggle_stories() -> Vec<Story> {
    vec![
        story("default", theme_toggle()),
    ]
}

fn heading_stories() -> Vec<Story> {
    vec![
        story("levels", stack().gap(Gap::Sm)
            .add(heading("H1 – page title").h1())
            .add(heading("H2 – section title").h2())
            .add(heading("H3 – subheading").h3())
            .add(heading("H4 – EYEBROW").h4())),
        story("tones", stack().gap(Gap::Sm)
            .add(heading("Default").h3())
            .add(heading("Brand").h3().tone(HeadingTone::Brand))
            .add(heading("Muted").h3().tone(HeadingTone::Muted))
            .add(heading("Success").h3().tone(HeadingTone::Success))
            .add(heading("Warning").h3().tone(HeadingTone::Warning))
            .add(heading("Danger").h3().tone(HeadingTone::Danger))),
    ]
}

fn layout_stories() -> Vec<Story> {
    // Small helper closure so each layout story reads clearly.
    let cell = |n: u32| badge(format!("{n}")).tone(Tone::Brand);
    vec![
        story("columns 1:2:1", columns().ratios("1 2 1").gap(Gap::Sm)
            .add(cell(1)).add(cell(2)).add(cell(3))),
        story("columns 4 equal", columns().cols(4).gap(Gap::Sm)
            .add(cell(1)).add(cell(2)).add(cell(3)).add(cell(4))),
        story("stack (vertical)", stack().gap(Gap::Sm)
            .add(cell(1)).add(cell(2)).add(cell(3))),
        story("grid auto-fit min 180", grid().set_min_col(180).gap(Gap::Sm)
            .add(cell(1)).add(cell(2)).add(cell(3))
            .add(cell(4)).add(cell(5)).add(cell(6))),
        story("cluster (wraps)", cluster_row()
            .add(cell(1)).add(cell(2)).add(cell(3))
            .add(cell(4)).add(cell(5)).add(cell(6))),
    ]
}
