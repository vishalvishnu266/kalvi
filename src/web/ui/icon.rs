use super::Render;

#[derive(Default, Clone)]
pub struct Icon {
    pub name: String,
    pub size: Option<String>,
    pub color: Option<String>,
}

impl Icon {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }

    pub fn size(mut self, size: impl Into<String>) -> Self {
        self.size = Some(size.into());
        self
    }

    pub fn color(mut self, color: impl Into<String>) -> Self {
        self.color = Some(color.into());
        self
    }
}

impl Render for Icon {
    fn render(&self) -> String {
        let mut attrs = vec![format!("name=\"{}\"", self.name)];
        if let Some(ref s) = self.size {
            attrs.push(format!("size=\"{}\"", s));
        }
        if let Some(ref c) = self.color {
            attrs.push(format!("style=\"color: {};\"", c));
        }

        format!("<ui-icon {}></ui-icon>", attrs.join(" "))
    }
}

impl std::fmt::Display for Icon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.render())
    }
}
