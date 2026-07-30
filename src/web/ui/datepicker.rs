use super::Render;

#[derive(Default, Clone)]
pub struct Datepicker {
    pub label: Option<String>,
    pub value: Option<String>,
    pub name: Option<String>,
}

impl Datepicker {
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

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
}

impl Render for Datepicker {
    fn render(&self) -> String {
        let mut attrs = Vec::new();
        if let Some(ref l) = self.label {
            attrs.push(format!("label=\"{}\"", l));
        }
        if let Some(ref v) = self.value {
            attrs.push(format!("value=\"{}\"", v));
        }
        if let Some(ref n) = self.name {
            attrs.push(format!("name=\"{}\"", n));
        }

        format!("<ui-datepicker {}></ui-datepicker>", attrs.join(" "))
    }
}

impl std::fmt::Display for Datepicker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.render())
    }
}
