use super::Render;

#[derive(Default, Clone)]
pub struct Toast {
    pub message: String,
    pub variant: Option<String>,
    pub duration: Option<i32>,
}

impl Toast {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            ..Default::default()
        }
    }

    pub fn variant(mut self, variant: impl Into<String>) -> Self {
        self.variant = Some(variant.into());
        self
    }

    pub fn duration(mut self, duration: i32) -> Self {
        self.duration = Some(duration);
        self
    }
}

impl Render for Toast {
    fn render(&self) -> String {
        let mut attrs = Vec::new();
        if let Some(ref v) = self.variant {
            attrs.push(format!("variant=\"{}\"", v));
        }
        if let Some(d) = self.duration {
            attrs.push(format!("duration=\"{}\"", d));
        }

        format!("<ui-toast {}>{}</ui-toast>", attrs.join(" "), self.message)
    }
}

impl std::fmt::Display for Toast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.render())
    }
}
