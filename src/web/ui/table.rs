use super::Render;

#[derive(Default, Clone)]
pub struct TableColumn {
    pub key: String,
    pub header: String,
    pub align: Option<String>,
}

#[derive(Default, Clone)]
pub struct Table {
    pub columns: Vec<TableColumn>,
    pub data: Vec<serde_json::Value>,
    pub empty_message: Option<String>,
}

impl Table {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn column(mut self, key: impl Into<String>, header: impl Into<String>, align: Option<String>) -> Self {
        self.columns.push(TableColumn {
            key: key.into(),
            header: header.into(),
            align,
        });
        self
    }

    pub fn data<T: serde::Serialize>(mut self, data: Vec<T>) -> Self {
        self.data = data.into_iter()
            .map(|item| serde_json::to_value(item).unwrap_or(serde_json::Value::Null))
            .collect();
        self
    }

    pub fn empty_message(mut self, message: impl Into<String>) -> Self {
        self.empty_message = Some(message.into());
        self
    }
}

impl Render for Table {
    fn render(&self) -> String {
        let mut attrs = Vec::new();
        
        let cols_json = serde_json::to_string(&self.columns).unwrap_or_else(|_| "[]".to_string());
        attrs.push(format!("columns='{}'", cols_json.replace("'", "&#39;")));

        let data_json = serde_json::to_string(&self.data).unwrap_or_else(|_| "[]".to_string());
        attrs.push(format!("data='{}'", data_json.replace("'", "&#39;")));

        if let Some(ref m) = self.empty_message {
            attrs.push(format!("empty-message=\"{}\"", m));
        }

        format!("<ui-table {}></ui-table>", attrs.join(" "))
    }
}

impl std::fmt::Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.render())
    }
}
