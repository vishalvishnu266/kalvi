//! # Command registry — the single source of truth for what the
//! copilot AND the launcher can do.
//!
//! One central [`CommandRegistry`] is built once at server startup and
//! shared across handlers. It powers:
//!
//! * The **launcher** (`GET /launcher/search`) — commands appear in the
//!   "Commands" section, ranked by fuzzy match on `label + keywords`.
//! * The **copilot** — `StubAgent` consumes the same registry to route
//!   natural-language turns. Adding a command teaches both surfaces.
//!
//! ## Anatomy of a command
//!
//! ```ignore
//! Command {
//!     id:       "theme.dark",              // stable machine id
//!     label:    "Switch to dark mode",     // primary display text
//!     hint:     "Persists across reloads", // secondary text
//!     icon:     "moon",                    // ui-icon name
//!     keywords: &["dark", "night", "theme"],
//!     handler:  |_ctx| vec![ AgentEvent::SideEffect { .. } ],
//! }
//! ```
//!
//! ## Adding a command
//!
//! ```ignore
//! CommandRegistry::new()
//!     .add(Command::new("theme.dark", "Switch to dark mode")
//!         .icon("moon")
//!         .keywords(&["dark", "night"])
//!         .handler(|_| vec![
//!             AgentEvent::SideEffect { kind: "theme".into(),
//!                                       payload: json!({"theme":"dark"}) }
//!         ]));
//! ```
//!
//! That's the entire surface. No macros, no boilerplate. Grep any
//! `Command::new("…")` call to find every command in the codebase.

use serde_json::Value;
use std::sync::Arc;

use crate::AgentEvent;

/// Free-form context handed to a command handler. Kept as JSON so we
/// can extend it (current URL, tenant, focused island, …) without
/// churning the handler signature.
pub type CommandCtx = Value;

/// A handler is a plain closure. `Arc<dyn Fn ..>` keeps it cheap to
/// clone into route state.
pub type CommandHandler =
    Arc<dyn Fn(&CommandCtx) -> Vec<AgentEvent> + Send + Sync + 'static>;

/// One entry in the registry.
///
/// Fields are `String` (not `&'static str`) so commands can be built
/// from config files or per-tenant data at startup without lifetime
/// juggling.
#[derive(Clone)]
pub struct Command {
    pub id:       String,
    pub label:    String,
    pub hint:     String,
    pub icon:     String,
    pub keywords: Vec<String>,
    pub handler:  CommandHandler,
}

impl std::fmt::Debug for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Command")
            .field("id", &self.id)
            .field("label", &self.label)
            .field("hint", &self.hint)
            .field("icon", &self.icon)
            .field("keywords", &self.keywords)
            .finish_non_exhaustive()
    }
}

impl Command {
    /// Start building a command with the two required fields.
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id:       id.into(),
            label:    label.into(),
            hint:     String::new(),
            icon:     "help".into(),
            keywords: Vec::new(),
            handler:  Arc::new(|_| Vec::new()),
        }
    }
    /// Secondary text — one short line shown under the label.
    pub fn hint(mut self, s: impl Into<String>) -> Self { self.hint = s.into(); self }
    /// `<ui-icon name="…">` name.
    pub fn icon(mut self, s: impl Into<String>) -> Self { self.icon = s.into(); self }
    /// Extra terms that should score highly in fuzzy match. Case-insensitive.
    pub fn keywords(mut self, kws: &[&str]) -> Self {
        self.keywords = kws.iter().map(|s| s.to_ascii_lowercase()).collect();
        self
    }
    /// The function to run when the user picks this command.
    pub fn handler<F>(mut self, f: F) -> Self
    where
        F: Fn(&CommandCtx) -> Vec<AgentEvent> + Send + Sync + 'static,
    {
        self.handler = Arc::new(f);
        self
    }
}

/// The registry itself — a plain ordered vec so command display order
/// is deterministic and grep-able. `Arc<Self>` is `Clone` for cheap
/// state sharing.
#[derive(Debug, Clone, Default)]
pub struct CommandRegistry {
    commands: Vec<Command>,
}

impl CommandRegistry {
    pub fn new() -> Self { Self::default() }

    /// Add a command. Chainable for a fluent server-startup style:
    ///
    /// ```ignore
    /// CommandRegistry::new()
    ///     .add(Command::new("a", "First").icon("home"))
    ///     .add(Command::new("b", "Second"));
    /// ```
    pub fn add(mut self, cmd: Command) -> Self {
        self.commands.push(cmd);
        self
    }

    /// Iterate over every registered command in insertion order.
    pub fn iter(&self) -> impl Iterator<Item = &Command> { self.commands.iter() }

    /// Look up by id — for direct dispatch when the launcher tells us
    /// exactly which command to run.
    pub fn get(&self, id: &str) -> Option<&Command> {
        self.commands.iter().find(|c| c.id == id)
    }

    /// Fuzzy-search — returns commands scored against the query, best
    /// first. The scorer is deliberately simple (substring hits on
    /// label + keywords, with a bonus for prefix matches on any word)
    /// so results are predictable and there's no dep on a fuzzy crate.
    ///
    /// Empty query returns everything in insertion order, unscored.
    pub fn search(&self, query: &str) -> Vec<(u32, &Command)> {
        let q = query.trim().to_ascii_lowercase();
        if q.is_empty() {
            return self.commands.iter().map(|c| (0, c)).collect();
        }
        let mut scored: Vec<(u32, &Command)> = self
            .commands
            .iter()
            .filter_map(|c| score(c, &q).map(|s| (s, c)))
            .collect();
        // Higher score = better match. Ties fall back to insertion order.
        scored.sort_by(|a, b| b.0.cmp(&a.0));
        scored
    }
}

/// Simple scoring: 100 for an exact label match, 50 for a label prefix,
/// 20 for label substring, 30 for exact keyword, 15 for keyword prefix,
/// 5 for keyword substring. Sum of hits. `None` if nothing matches.
///
/// Kept naive so it's easy to reason about and to tune. Swap for a
/// real fuzzy crate later if the miss rate becomes annoying.
fn score(c: &Command, q: &str) -> Option<u32> {
    let label = c.label.to_ascii_lowercase();
    let mut s: u32 = 0;
    if label == q         { s += 100; }
    else if label.starts_with(q) { s += 50; }
    else if label.contains(q)    { s += 20; }
    for kw in &c.keywords {
        if kw == q                { s += 30; }
        else if kw.starts_with(q) { s += 15; }
        else if kw.contains(q)    { s += 5; }
    }
    if s == 0 { None } else { Some(s) }
}
