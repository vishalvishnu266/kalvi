use askama::Template;
use axum::response::{Html, IntoResponse};
use super::*;

#[derive(Template)]
#[template(path = "ui/storybook.html")]
struct StorybookTemplate {
    pub components: Vec<ComponentExample>,
}

struct ComponentExample {
    pub name: String,
    pub description: String,
    pub html: String,
}

pub async fn index() -> impl IntoResponse {
    let components = vec![
        ComponentExample {
            name: "Button".to_string(),
            description: "Standard action button with variants and sizes.".to_string(),
            html: vec![
                Button::new("Primary").variant("primary").render(),
                Button::new("Secondary").variant("secondary").render(),
                Button::new("Ghost").variant("ghost").render(),
                Button::new("Danger").variant("danger").render(),
                Button::new("Small").size("sm").render(),
                Button::new("Large").size("lg").render(),
                Button::new("With Icon").icon("plus").render(),
            ].join(" "),
        },
        ComponentExample {
            name: "Badge".to_string(),
            description: "Status indicators and labels.".to_string(),
            html: vec![
                Badge::new("New").variant("primary").render(),
                Badge::new("Success").variant("success").render(),
                Badge::new("Warning").variant("warning").render(),
                Badge::new("Danger").variant("danger").render(),
                Badge::new("Pill").variant("info").pill().render(),
            ].join(" "),
        },
        ComponentExample {
            name: "Avatar".to_string(),
            description: "User profile pictures or initials.".to_string(),
            html: vec![
                Avatar::new().initials("JD").render(),
                Avatar::new().initials("AS").size("lg").shape("square").render(),
                Avatar::new().src("https://i.pravatar.cc/100").size("xl").render(),
            ].join(" "),
        },
        ComponentExample {
            name: "Responsive Grid".to_string(),
            description: "A grid that changes columns based on screen size (1 on mobile, 2 on tablet, 4 on desktop).".to_string(),
            html: Grid::new()
                .cols(1).md(2).lg(4).gap("20px")
                .add(Stat::new("Revenue", "$12,345").delta("+12%", "up"))
                .add(Stat::new("Orders", "150").delta("+5%", "up"))
                .add(Stat::new("Users", "1,234").delta("-2%", "down"))
                .add(Stat::new("Growth", "8%").delta("+1%", "up"))
                .render(),
        },
        ComponentExample {
            name: "Stat".to_string(),
            description: "Key metrics and statistics.".to_string(),
            html: vec![
                Stat::new("Revenue", "$12,345").delta("+12%", "up").icon("currency-dollar").render(),
                Stat::new("Users", "1,234").delta("-5%", "down").icon("people").render(),
            ].join(" "),
        },
        ComponentExample {
            name: "Select".to_string(),
            description: "Dropdown selection menu.".to_string(),
            html: Select::new()
                .label("Choose a Role")
                .placeholder("Select role...")
                .option("Admin", "admin", false)
                .option("Editor", "editor", true)
                .option("Viewer", "viewer", false)
                .render(),
        },
        ComponentExample {
            name: "Segmented".to_string(),
            description: "Multi-option toggle switch.".to_string(),
            html: Segmented::new()
                .value("list")
                .option("Grid", "grid", Some("grid".to_string()))
                .option("List", "list", Some("list-ul".to_string()))
                .render(),
        },
        ComponentExample {
            name: "Tab Bar".to_string(),
            description: "Navigation tabs.".to_string(),
            html: TabBar::new()
                .active("profile")
                .tab("Home", "home", Some("house".to_string()))
                .tab("Profile", "profile", Some("person".to_string()))
                .tab("Settings", "settings", Some("gear".to_string()))
                .render(),
        },
        ComponentExample {
            name: "List Item".to_string(),
            description: "Generic list row component.".to_string(),
            html: vec![
                ListItem::new("Account Settings").sublabel("Manage your profile").icon("person-gear").render(),
                ListItem::new("Notifications").sublabel("Configure alerts").icon("bell").active().render(),
            ].join(" "),
        },
        ComponentExample {
            name: "Datepicker".to_string(),
            description: "Calendar date selection.".to_string(),
            html: Datepicker::new().label("Birthday").render(),
        },
        ComponentExample {
            name: "Card".to_string(),
            description: "Container for grouping content.".to_string(),
            html: Card::new("This is the card content.")
                .title("Card Title")
                .footer("Card Footer")
                .render(),
        },
        ComponentExample {
            name: "Input".to_string(),
            description: "Standard text input field.".to_string(),
            html: vec![
                Input::new().label("Username").placeholder("Enter your username").render(),
                Input::new().label("Password").type_("password").render(),
            ].join("<br><br>"),
        },
    ];

    let template = StorybookTemplate { components };
    Html(template.render().unwrap())
}
