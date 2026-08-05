//! Streaming fragments over Server-Sent Events (SSE).
//!
//! The agent runtime (see the `agent` crate) produces a `Stream<Fragment>`
//! as it executes a user command. This module adapts that stream into an
//! `axum::response::Sse` response whose events carry the exact same
//! `<ui-fragment>` envelopes as a normal HTTP response — the browser
//! runtime doesn't care which transport delivered them.
//!
//! ## Event shape
//!
//! Each SSE event has:
//!   * `event: fragment`
//!   * `data:` = the rendered `<ui-fragment>…</ui-fragment>` string
//!
//! The stream is terminated with a final `event: done` event so the
//! client can detach without waiting for the connection to close.

use axum::response::sse::{Event, KeepAlive, Sse};
use futures_core::Stream;
use futures_util::StreamExt;

use crate::fragment::{Fragment, Render};

/// Wrap a fragment stream into an axum SSE response.
///
/// The returned `Sse` has a 15-second keep-alive so long-running agent
/// turns don't get killed by proxies with idle timeouts.
pub fn fragments_sse<S>(
    stream: S,
) -> Sse<impl Stream<Item = Result<Event, std::convert::Infallible>>>
where
    S: Stream<Item = Fragment> + Send + 'static,
{
    // Map each fragment → an SSE `fragment` event whose data payload is
    // the wire-format envelope. Append a terminal `done` event so the
    // browser can call `es.close()` and know the turn is over.
    let mapped = stream
        .map(|frag| {
            let data = frag.render_string();
            Ok::<_, std::convert::Infallible>(
                Event::default().event("fragment").data(data),
            )
        })
        .chain(futures_util::stream::once(async {
            Ok(Event::default().event("done").data(""))
        }));

    Sse::new(mapped).keep_alive(KeepAlive::new().interval(std::time::Duration::from_secs(15)))
}
