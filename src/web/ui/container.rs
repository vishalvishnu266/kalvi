use super::Render;

#[derive(Default, Clone)]
pub struct Container {
    pub children: Vec<String>,
    pub max_width: Option<String>,
    pub centered: bool,
}

impl Container {
    pub fn new() -> Self {
        Self {
            centered: true,
            ..Default::default()
        }
    }

    pub fn max_width(mut self, width: impl Into<String>) -> Self {
        self.max_width = Some(width.into());
        self
    }

    pub fn add<T: Render>(mut self, child: T) -> Self {
        self.children.push(child.render());
        self
    }
}

impl Render for Container {
    fn render(&self) -> String {
        let mut styles = Vec::new();
        if let Some(ref w) = self.max_width {
            styles.push(format!("max-width: {}", w));
        }
        if self.centered {
            styles.push("margin-left: auto".to_string());
            styles.push("margin-right: auto".to_string());
        }

        let style_attr = if styles.is_empty() { String::new() } else { format!(" style=\"{}\"", styles.join("; ")) };

        format!("<div class=\"container\" {}>\n  {}\n</div>", style_attr, self.children.join("\n  "))
    }
}

impl std::fmt::Display for Container {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.render())
    }
}
