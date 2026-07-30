use super::Render;

#[derive(Default, Clone)]
pub struct Stack {
    pub children: Vec<String>,
    pub gap: Option<String>,
}

impl Stack {
    pub fn new() -> Self {
        Self::default()
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

impl Render for Stack {
    fn render(&self) -> String {
        let style = self.gap.as_ref().map(|g| format!(" style=\"gap: {};\"", g)).unwrap_or_default();
        format!("<div class=\"stack\" {}>\n  {}\n</div>", style, self.children.join("\n  "))
    }
}

#[derive(Default, Clone)]
pub struct Row {
    pub children: Vec<String>,
    pub gap: Option<String>,
    pub align: Option<String>,
}

impl Row {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn gap(mut self, gap: impl Into<String>) -> Self {
        self.gap = Some(gap.into());
        self
    }

    pub fn align(mut self, align: impl Into<String>) -> Self {
        self.align = Some(align.into());
        self
    }

    pub fn add<T: Render>(mut self, child: T) -> Self {
        self.children.push(child.render());
        self
    }
}

impl Render for Row {
    fn render(&self) -> String {
        let mut styles = Vec::new();
        if let Some(ref g) = self.gap { styles.push(format!("gap: {}", g)); }
        if let Some(ref a) = self.align { styles.push(format!("align-items: {}", a)); }
        
        let style_attr = if styles.is_empty() { String::new() } else { format!(" style=\"{}\"", styles.join("; ")) };
        
        format!("<div class=\"row\" {}>\n  {}\n</div>", style_attr, self.children.join("\n  "))
    }
}
