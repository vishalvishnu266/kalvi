use super::Render;

#[derive(Default, Clone)]
pub struct Modal {
    pub title: String,
    pub content: String,
    pub open: bool,
    pub size: Option<String>,
}

impl Modal {
    pub fn new(title: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            content: content.into(),
            ..Default::default()
        }
    }

    pub fn open(mut self) -> Self {
        self.open = true;
        self
    }

    pub fn size(mut self, size: impl Into<String>) -> Self {
        self.size = Some(size.into());
        self
    }
}

impl Render for Modal {
    fn render(&self) -> String {
        let mut attrs = Vec::new();
        attrs.push(format!("label=\"{}\"", self.title));
        
        if self.open {
            attrs.push("open".to_string());
        }
        if let Some(ref s) = self.size {
            attrs.push(format!("size=\"{}\"", s));
        }

        format!("<ui-modal {}>{}</ui-modal>", attrs.join(" "), self.content)
    }
}

impl std::fmt::Display for Modal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.render())
    }
}
