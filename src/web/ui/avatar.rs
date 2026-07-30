use super::Render;

#[derive(Default, Clone)]
pub struct Avatar {
    pub src: Option<String>,
    pub initials: Option<String>,
    pub size: Option<String>,
    pub shape: Option<String>,
}

impl Avatar {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn src(mut self, src: impl Into<String>) -> Self {
        self.src = Some(src.into());
        self
    }

    pub fn initials(mut self, initials: impl Into<String>) -> Self {
        self.initials = Some(initials.into());
        self
    }

    pub fn size(mut self, size: impl Into<String>) -> Self {
        self.size = Some(size.into());
        self
    }

    pub fn shape(mut self, shape: impl Into<String>) -> Self {
        self.shape = Some(shape.into());
        self
    }
}

impl Render for Avatar {
    fn render(&self) -> String {
        let mut attrs = Vec::new();
        if let Some(ref s) = self.src {
            attrs.push(format!("src=\"{}\"", s));
        }
        if let Some(ref i) = self.initials {
            attrs.push(format!("initials=\"{}\"", i));
        }
        if let Some(ref s) = self.size {
            attrs.push(format!("size=\"{}\"", s));
        }
        if let Some(ref sh) = self.shape {
            attrs.push(format!("shape=\"{}\"", sh));
        }

        let attr_str = if attrs.is_empty() {
            String::new()
        } else {
            format!(" {}", attrs.join(" "))
        };

        format!("<ui-avatar{}></ui-avatar>", attr_str)
    }
}

impl std::fmt::Display for Avatar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.render())
    }
}
