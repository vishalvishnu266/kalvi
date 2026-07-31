//! `<ui-command>` (⌘K launcher) + `<ui-command-item>` typed builders.

use crate::core::{escape_html, wrap, Attr, Component};

pub struct CommandItem {
    label: String,
    icon: Option<String>,
    group: Option<String>,
    href: Option<String>,
    action: Option<String>,
}
pub fn command_item(label: impl Into<String>) -> CommandItem {
    CommandItem { label: label.into(), icon: None, group: None, href: None, action: None }
}
impl CommandItem {
    pub fn icon(mut self, name: impl Into<String>)   -> Self { self.icon = Some(name.into()); self }
    pub fn group(mut self, name: impl Into<String>)  -> Self { self.group = Some(name.into()); self }
    pub fn href(mut self, s: impl Into<String>)      -> Self { self.href = Some(s.into()); self }
    pub fn action(mut self, s: impl Into<String>)    -> Self { self.action = Some(s.into()); self }
}
impl Component for CommandItem {
    fn render(&self) -> String {
        let mut attrs = Vec::new();
        if let Some(ref v) = self.icon   { attrs.push(Attr::kv("icon",   v.as_str())); }
        if let Some(ref v) = self.group  { attrs.push(Attr::kv("group",  v.as_str())); }
        if let Some(ref v) = self.href   { attrs.push(Attr::kv("href",   v.as_str())); }
        if let Some(ref v) = self.action { attrs.push(Attr::kv("action", v.as_str())); }
        wrap("ui-command-item", &attrs, &escape_html(&self.label))
    }
}

pub struct Command { placeholder: Option<String>, items: Vec<CommandItem> }
pub fn command() -> Command { Command { placeholder: None, items: Vec::new() } }
impl Command {
    pub fn placeholder(mut self, s: impl Into<String>) -> Self { self.placeholder = Some(s.into()); self }
    pub fn item(mut self, i: CommandItem)              -> Self { self.items.push(i); self }
    pub fn items<I: IntoIterator<Item = CommandItem>>(mut self, iter: I) -> Self {
        self.items.extend(iter); self
    }
}
impl Component for Command {
    fn render(&self) -> String {
        let mut attrs = Vec::new();
        if let Some(ref p) = self.placeholder { attrs.push(Attr::kv("placeholder", p.as_str())); }
        let body: String = self.items.iter().map(|i| i.render()).collect();
        wrap("ui-command", &attrs, &body)
    }
}
