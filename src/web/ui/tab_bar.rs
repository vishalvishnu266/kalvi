use super::Render;

#[derive(Default, Clone)]
pub struct Tab {
    pub label: String,
    pub value: String,
    pub icon: Option<String>,
}

#[derive(Default, Clone)]
pub struct TabBar {
    pub active: Option<String>,
    pub tabs: Vec<Tab>,
}

impl TabBar {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn active(mut self, value: impl Into<String>) -> Self {
        self.active = Some(value.into());
        self
    }

    pub fn tab(mut self, label: impl Into<String>, value: impl Into<String>, icon: Option<String>) -> Self {
        self.tabs.push(Tab {
            label: label.into(),
            value: value.into(),
            icon,
        });
        self
    }
}

impl Render for TabBar {
    fn render(&self) -> String {
        let mut attrs = Vec::new();
        if let Some(ref a) = self.active {
            attrs.push(format!("active=\"{}\"", a));
        }

        let tabs_json = serde_json::to_string(&self.tabs).unwrap_or_else(|_| "[]".to_string());
        attrs.push(format!("tabs='{}'", tabs_json.replace("'", "&#39;")));

        format!("<ui-tab-bar {}></ui-tab-bar>", attrs.join(" "))
    }
}

impl std::fmt::Display for TabBar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.render())
    }
}
