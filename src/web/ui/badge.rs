use super::Render;

#[derive(Default, Clone)]
pub struct Badge {
    pub label: String,
    pub variant: Option<String>,
    pub size: Option<String>,
    pub pill: bool,
}

impl Badge {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            ..Default::default()
        }
    }

    pub fn variant(mut self, variant: impl Into<String>) -> Self {
        self.variant = Some(variant.into());
        self
    }

    pub fn size(mut self, size: impl Into<String>) -> Self {
        self.size = Some(size.into());
        self
    }

    pub fn pill(mut self) -> Self {
        self.pill = true;
        self
    }
}

impl Render for Badge {
    fn render(&self) -> String {
        let mut attrs = Vec::new();
        if let Some(ref v) = self.variant {
            attrs.push(format!("variant=\"{}\"", v));
        }
        if let Some(ref s) = self.size {
            attrs.push(format!("size=\"{}\"", s));
        }
        if self.pill {
            attrs.push("pill".to_string());
        }

        let attr_str = if attrs.is_empty() {
            String::new()
        } else {
            format!(" {}", attrs.join(" "))
        };

        format!("<ui-badge{}>{}</ui-badge>", attr_str, self.label)
    }
}

impl std::fmt::Display for Badge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.render())
    }
}
