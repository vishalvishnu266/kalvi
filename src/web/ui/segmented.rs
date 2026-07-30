use super::Render;

#[derive(Default, Clone)]
pub struct SegmentedOption {
    pub label: String,
    pub value: String,
    pub icon: Option<String>,
}

#[derive(Default, Clone)]
pub struct Segmented {
    pub name: Option<String>,
    pub value: Option<String>,
    pub options: Vec<SegmentedOption>,
}

impl Segmented {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn option(mut self, label: impl Into<String>, value: impl Into<String>, icon: Option<String>) -> Self {
        self.options.push(SegmentedOption {
            label: label.into(),
            value: value.into(),
            icon,
        });
        self
    }
}

impl Render for Segmented {
    fn render(&self) -> String {
        let mut attrs = Vec::new();
        if let Some(ref n) = self.name {
            attrs.push(format!("name=\"{}\"", n));
        }
        if let Some(ref v) = self.value {
            attrs.push(format!("value=\"{}\"", v));
        }

        let options_json = serde_json::to_string(&self.options).unwrap_or_else(|_| "[]".to_string());
        attrs.push(format!("options='{}'", options_json.replace("'", "&#39;")));

        format!("<ui-segmented {}></ui-segmented>", attrs.join(" "))
    }
}

impl std::fmt::Display for Segmented {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.render())
    }
}
