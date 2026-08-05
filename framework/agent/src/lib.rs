//! # agent — the copilot's brain
//!
//! This crate defines the contract between the copilot pane in the
//! browser and whatever is running commands on the server. It is
//! deliberately abstract: **the framework does not care whether the
//! implementation is a hand-written rule table, a hosted LLM, or a
//! local model.**
//!
//! ## Contract
//!
//! An [`Agent`] receives a [`Turn`] (the user's utterance + context) and
//! returns a `Stream<AgentEvent>`. Each [`AgentEvent`] is either:
//!
//! * A [`Fragment`] to apply to the shell (chat bubble, form, table,
//!   toast, or a *navigation* fragment that replaces `main`).
//! * A [`ToolCall`] / [`ToolResult`] pair for audit + progress cards.
//! * A [`SideEffect`] the browser should perform locally (dark mode
//!   toggle, focus a field, etc.) — encoded as a fragment targeting the
//!   special `__side_effect__` island the client interprets.
//!
//! The server route (`POST /agent`) simply pipes this stream through
//! [`ui_shell::fragments_sse`] and the browser applies each envelope as
//! it lands.
//!
//! ## Milestone status
//!
//! * **Step 3 (this file):** trait + event types only. No implementations.
//! * **Step 7:** [`StubAgent`] with a fixed command table ("go to X",
//!   "add user", "show users", "enable dark mode"). Proves the pipeline.
//! * **Later:** `LlmAgent` implementing the same trait — swap-in only.

pub mod stub;
pub use stub::StubAgent;

use async_trait::async_trait;
use futures_core::Stream;
use serde::{Deserialize, Serialize};
use std::pin::Pin;

use ui_shell::Fragment;

/// A single interaction with the agent.
///
/// The `context` map is intentionally free-form JSON so callers can
/// attach the current URL, the focused island, the current tenant, etc.
/// without churning this crate every time the shell learns a new bit
/// of state.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Turn {
    /// The raw utterance from the user's composer input.
    pub utterance: String,
    /// Free-form JSON context (current URL, focused island, tenant, …).
    #[serde(default)]
    pub context: serde_json::Value,
}

/// A tool the agent decides to run mid-turn.
///
/// The stub agent maps these 1:1 to internal helpers; a future LLM
/// agent would emit them from a function-calling API.
#[derive(Debug, Clone, Serialize)]
pub struct ToolCall {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub args: serde_json::Value,
}

/// The outcome of a [`ToolCall`]. `ok=false` signals the copilot pane
/// should render the result as a failure card.
#[derive(Debug, Clone, Serialize)]
pub struct ToolResult {
    pub id: String,
    pub ok: bool,
    #[serde(default)]
    pub data: serde_json::Value,
}

/// Anything the agent can emit during a turn.
///
/// Each variant is delivered to the browser wrapped in a `Fragment`
/// envelope so a single applier pipeline handles the lot. Rich types
/// exist here (rather than everything being `Fragment`) so the server
/// side can audit / persist a structured trace.
#[derive(Debug, Clone)]
pub enum AgentEvent {
    /// Apply a pre-built fragment to the shell (chat bubble, form,
    /// navigation, toast, …).
    Fragment(Fragment),
    /// Announce that a tool is about to run — the copilot renders a
    /// progress card.
    ToolCall(ToolCall),
    /// Announce a tool's outcome — the copilot updates the card.
    ToolResult(ToolResult),
    /// Ask the browser to perform a client-only action (dark mode
    /// toggle, focus, etc.). Rendered as a fragment targeting the
    /// special `__side_effect__` island.
    SideEffect { kind: String, payload: serde_json::Value },
}

/// A boxed stream of [`AgentEvent`]s. Boxed for object-safety of
/// [`Agent::handle`].
pub type EventStream = Pin<Box<dyn Stream<Item = AgentEvent> + Send>>;

/// The trait every copilot backend implements.
///
/// Kept dyn-safe (no generics on the method, single async return).
#[async_trait]
pub trait Agent: Send + Sync {
    /// Handle one turn. Implementations should return promptly with a
    /// stream — heavy lifting happens inside the stream so the browser
    /// can render intermediate frames.
    async fn handle(&self, turn: Turn) -> EventStream;
}
