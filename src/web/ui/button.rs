use super::Render;

#[derive(Default, Clone)]
pub struct Button {
    pub label: String,
    pub variant: Option<String>,
    pub size: Option<String>,
    pub icon: Option<String>,
    pub full: bool,
    pub attr: Vec<(String, String)>,
}

impl Button {
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

    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn full(mut self) -> Self {
        self.full = true;
        self
    }

    pub fn attr(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.attr.push((name.into(), value.into()));
        self
    }
}

impl Render for Button {
    fn render(&self) -> String {
        let mut attrs = Vec::new();
        if let Some(ref v) = self.variant {
            attrs.push(format!("variant=\"{}\"", v));
        }
        if let Some(ref s) = self.size {
            attrs.push(format!("size=\"{}\"", s));
        }
        if let Some(ref i) = self.icon {
            attrs.push(format!("icon=\"{}\"", i));
        }
        if self.full {
            attrs.push("full".to_string());
        }
        for (name, value) in &self.attr {
            attrs.push(format!("{}=\"{}\"", name, value));
        }

        let attr_str = if attrs.is_empty() {
            String::new()
        } else {
            format!(" {}", attrs.join(" "))
        };

        format!("<ui-button{}>{}</ui-button>", attr_str, self.label)
    }
}

impl std::fmt::Display for Button {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.render())
    }
}
