use super::Render;

#[derive(Default, Clone)]
pub struct AppShell {
    pub brand_name: String,
    pub user_name: Option<String>,
    pub sidebar_content: String,
    pub topbar_content: String,
    pub main_content: String,
}

impl AppShell {
    pub fn new(brand_name: impl Into<String>) -> Self {
        Self {
            brand_name: brand_name.into(),
            ..Default::default()
        }
    }

    pub fn user(mut self, name: impl Into<String>) -> Self {
        self.user_name = Some(name.into());
        self
    }

    pub fn sidebar(mut self, content: impl Into<String>) -> Self {
        self.sidebar_content = content.into();
        self
    }

    pub fn topbar(mut self, content: impl Into<String>) -> Self {
        self.topbar_content = content.into();
        self
    }

    pub fn main(mut self, content: impl Into<String>) -> Self {
        self.main_content = content.into();
        self
    }
}

impl Render for AppShell {
    fn render(&self) -> String {
        let mut attrs = Vec::new();
        attrs.push(format!("brand-name=\"{}\"", self.brand_name));
        if let Some(ref u) = self.user_name {
            attrs.push(format!("user-name=\"{}\"", u));
        }

        format!(
            "<app-shell {}>\n  <div slot=\"sidebar\">{}</div>\n  <div slot=\"topbar\">{}</div>\n  {}\n</app-shell>",
            attrs.join(" "),
            self.sidebar_content,
            self.topbar_content,
            self.main_content
        )
    }
}

impl std::fmt::Display for AppShell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.render())
    }
}
