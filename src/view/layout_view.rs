pub struct LayoutContext {
    pub title: String,
    pub primary_color: String,
    pub dark_mode: bool,
    pub tenant_slug: Option<String>,
}

impl Default for LayoutContext {
    fn default() -> Self {
        Self {
            title: "Kalvi ERP".to_string(),
            primary_color: "#3b82f6".to_string(), // blue-500
            dark_mode: false,
            tenant_slug: None,
        }
    }
}

pub fn render_layout(ctx: LayoutContext, content: String) -> String {
    let template = include_str!("layout.html");
    template
        .replace("{title}", &ctx.title)
        .replace("{primary_color}", &ctx.primary_color)
        .replace("{dark_mode}", &ctx.dark_mode.to_string())
        .replace("{content}", &content)
}
