use super::Render;

#[derive(Default, Clone)]
pub struct Card {
    pub title: Option<String>,
    pub content: String,
    pub footer: Option<String>,
}

impl Card {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            ..Default::default()
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn footer(mut self, footer: impl Into<String>) -> Self {
        self.footer = Some(footer.into());
        self
    }
}

impl Render for Card {
    fn render(&self) -> String {
        let title_html = self.title.as_ref().map(|t| format!("<div slot=\"header\">{}</div>", t)).unwrap_or_default();
        let footer_html = self.footer.as_ref().map(|f| format!("<div slot=\"footer\">{}</div>", f)).unwrap_or_default();
        
        format!(
            "<ui-card>{}{}{}</ui-card>",
            title_html,
            self.content,
            footer_html
        )
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.render())
    }
}
