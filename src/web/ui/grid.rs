use super::Render;

#[derive(Default, Clone)]
pub struct Grid {
    pub children: Vec<String>,
    pub cols_default: Option<u8>,
    pub cols_md: Option<u8>,
    pub cols_lg: Option<u8>,
    pub gap: Option<String>,
}

impl Grid {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn cols(mut self, cols: u8) -> Self {
        self.cols_default = Some(cols);
        self
    }

    pub fn md(mut self, cols: u8) -> Self {
        self.cols_md = Some(cols);
        self
    }

    pub fn lg(mut self, cols: u8) -> Self {
        self.cols_lg = Some(cols);
        self
    }

    pub fn gap(mut self, gap: impl Into<String>) -> Self {
        self.gap = Some(gap.into());
        self
    }

    pub fn add<T: Render>(mut self, child: T) -> Self {
        self.children.push(child.render());
        self
    }
}

impl Render for Grid {
    fn render(&self) -> String {
        let mut styles = vec!["display: grid".to_string()];
        
        let cols = self.cols_default.unwrap_or(1);
        styles.push(format!("--grid-cols: {}", cols));
        
        if let Some(md) = self.cols_md {
            styles.push(format!("--grid-cols-md: {}", md));
        }
        if let Some(lg) = self.cols_lg {
            styles.push(format!("--grid-cols-lg: {}", lg));
        }
        
        styles.push(format!("grid-template-columns: repeat(var(--grid-cols), minmax(0, 1fr))"));
        
        if let Some(ref g) = self.gap {
            styles.push(format!("gap: {}", g));
        }

        // We use a helper class in app-shell.css or global.css to handle media queries for these variables
        format!("<div class=\"ui-grid\" style=\"{}\">\n  {}\n</div>", styles.join("; "), self.children.join("\n  "))
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.render())
    }
}
