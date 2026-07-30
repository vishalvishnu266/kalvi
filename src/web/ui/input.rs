use super::Render;

#[derive(Default, Clone)]
pub struct Input {
    pub label: Option<String>,
    pub value: Option<String>,
    pub placeholder: Option<String>,
    pub type_: Option<String>,
    pub name: Option<String>,
}

impl Input {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
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

    pub fn type_(mut self, type_: impl Into<String>) -> Self {
        self.type_ = Some(type_.into());
        self
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
}

impl Render for Input {
    fn render(&self) -> String {
        let mut attrs = Vec::new();
        if let Some(ref l) = self.label {
            attrs.push(format!("label=\"{}\"", l));
        }
        if let Some(ref v) = self.value {
            attrs.push(format!("value=\"{}\"", v));
        }
        if let Some(ref p) = self.placeholder {
            attrs.push(format!("placeholder=\"{}\"", p));
        }
        if let Some(ref t) = self.type_ {
            attrs.push(format!("type=\"{}\"", t));
        }
        if let Some(ref n) = self.name {
            attrs.push(format!("name=\"{}\"", n));
        }

        format!("<ui-input {}></ui-input>", attrs.join(" "))
    }
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.render())
    }
}
