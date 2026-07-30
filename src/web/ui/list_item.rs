use super::Render;

#[derive(Default, Clone)]
pub struct ListItem {
    pub label: String,
    pub sublabel: Option<String>,
    pub icon: Option<String>,
    pub href: Option<String>,
    pub active: bool,
    pub interactive: bool,
}

impl ListItem {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            interactive: true,
            ..Default::default()
        }
    }

    pub fn sublabel(mut self, sublabel: impl Into<String>) -> Self {
        self.sublabel = Some(sublabel.into());
        self
    }

    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn href(mut self, href: impl Into<String>) -> Self {
        self.href = Some(href.into());
        self
    }

    pub fn active(mut self) -> Self {
        self.active = true;
        self
        }
    
    pub fn static_only(mut self) -> Self {
        self.interactive = false;
        self
    }
}

impl Render for ListItem {
    fn render(&self) -> String {
        let mut attrs = Vec::new();
        attrs.push(format!("label=\"{}\"", self.label));
        
        if let Some(ref s) = self.sublabel {
            attrs.push(format!("sublabel=\"{}\"", s));
        }
        if let Some(ref i) = self.icon {
            attrs.push(format!("icon=\"{}\"", i));
        }
        if let Some(ref h) = self.href {
            attrs.push(format!("href=\"{}\"", h));
        }
        if self.active {
            attrs.push("active".to_string());
        }
        if !self.interactive {
            attrs.push("static".to_string());
        }

        format!("<ui-list-item {}></ui-list-item>", attrs.join(" "))
    }
}

impl std::fmt::Display for ListItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.render())
    }
}
