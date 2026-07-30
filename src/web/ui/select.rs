use super::Render;

#[derive(Default, Clone)]
pub struct SelectOption {
    pub label: String,
    pub value: String,
    pub selected: bool,
}

#[derive(Default, Clone)]
pub struct Select {
    pub label: Option<String>,
    pub name: Option<String>,
    pub value: Option<String>,
    pub options: Vec<SelectOption>,
    pub placeholder: Option<String>,
}

impl Select {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn option(mut self, label: impl Into<String>, value: impl Into<String>, selected: bool) -> Self {
        self.options.push(SelectOption {
            label: label.into(),
            value: value.into(),
            selected,
        });
        self
    }
}

impl Render for Select {
    fn render(&self) -> String {
        let mut attrs = Vec::new();
        if let Some(ref l) = self.label {
            attrs.push(format!("label=\"{}\"", l));
        }
        if let Some(ref n) = self.name {
            attrs.push(format!("name=\"{}\"", n));
        }
        if let Some(ref v) = self.value {
            attrs.push(format!("value=\"{}\"", v));
        }
        if let Some(ref p) = self.placeholder {
            attrs.push(format!("placeholder=\"{}\"", p));
        }

        let options_json = serde_json::to_string(&self.options).unwrap_or_else(|_| "[]".to_string());
        attrs.push(format!("options='{}'", options_json.replace("'", "&#39;")));

        format!("<ui-select {}></ui-select>", attrs.join(" "))
    }
}

impl std::fmt::Display for Select {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.render())
    }
}
