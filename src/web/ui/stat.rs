use super::Render;

#[derive(Default, Clone)]
pub struct Stat {
    pub label: String,
    pub value: String,
    pub delta: Option<String>,
    pub delta_type: Option<String>,
    pub icon: Option<String>,
}

impl Stat {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            ..Default::default()
        }
    }

    pub fn delta(mut self, delta: impl Into<String>, type_: impl Into<String>) -> Self {
        self.delta = Some(delta.into());
        self.delta_type = Some(type_.into());
        self
    }

    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }
}

impl Render for Stat {
    fn render(&self) -> String {
        let mut attrs = Vec::new();
        attrs.push(format!("label=\"{}\"", self.label));
        attrs.push(format!("value=\"{}\"", self.value));
        
        if let Some(ref d) = self.delta {
            attrs.push(format!("delta=\"{}\"", d));
        }
        if let Some(ref dt) = self.delta_type {
            attrs.push(format!("delta-type=\"{}\"", dt));
        }
        if let Some(ref i) = self.icon {
            attrs.push(format!("icon=\"{}\"", i));
        }

        format!("<ui-stat {}></ui-stat>", attrs.join(" "))
    }
}

impl std::fmt::Display for Stat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.render())
    }
}
