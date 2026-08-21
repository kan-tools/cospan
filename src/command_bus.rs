//! L4 — the command bus (write seam).
//!
//! `telos/observe-now-control-later`: cospan ships read-only observation first
//! and leaves a clean seam for control to arrive later. This module is that
//! seam. It defines *what a write into a running session looks like* — and
//! nothing more. There is deliberately **no implementor** in this build; the
//! read paths (the Chat view, the fold) never construct or call a `WriteChannel`.
//!
//! The first implementation is the next feature: the P3 "redirect" verb, whose
//! **primary target is the harness message bus** (Claude Code Remote Control /
//! `SendMessage`), which injects a real structured user turn into a session —
//! higher fidelity than scraped keystrokes and available with no workflow
//! change. Multiplexer `send-keys` and PTY ownership are alternate impls this
//! trait admits later; the read layer (`crate::transcripts`) supplies the
//! display, and a write impl is correlated to it by session id.

use crate::transcripts::SessionHandle;

/// A channel that can deliver operator input into a running agent session.
///
/// Defined as the write seam; **unimplemented in this build**. When the write
/// feature lands, an impl (message bus first) delivers `text` to the session the
/// handle names. Kept object-safe so the eventual command bus can hold a
/// `Box<dyn WriteChannel>` without knowing which harness backs it.
pub trait WriteChannel {
    /// Deliver `text` as an operator turn to `session`. `Err` carries a
    /// human-readable reason the write could not be routed.
    fn send(&self, session: &SessionHandle, text: &str) -> Result<(), String>;
}

#[cfg(test)]
mod tests {
    use super::*;

    // AC-8: the trait exists with the documented `send` signature and is
    // object-safe (the command bus will hold it behind a `dyn`). This test
    // passing is a compile-time guarantee — naming `dyn WriteChannel` and the
    // `send` signature only type-checks if the trait exists as specified. There
    // is intentionally no implementor to construct, so no read/view path can
    // perform a write this build.
    #[test]
    fn write_channel_is_object_safe_and_unimplemented() {
        fn _takes(_c: &dyn WriteChannel) {}
        let _sig: fn(&dyn WriteChannel, &SessionHandle, &str) -> Result<(), String> =
            |c, s, t| c.send(s, t);
    }
}
