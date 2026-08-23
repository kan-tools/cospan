# 08 — Mobile frontend (recorded vision)

> **Status: recorded vision, not started.** Floated 2026-08-23 while the comment
> authoring milestone (`comment-authoring-surface`, S4/S5 pending) is in flight.
> This is a *parallel* initiative, deliberately deferred until that milestone
> wraps. Captured here and on the kan subject `mobile-frontend` so it is durable
> and findable, not so it is next.

## The idea

Make cospan host all the data that currently drives the TUI as a **thin local
API**, and expose it over a **secure socket to a web client running on the phone**
— a mobile frontend to all of cospan's functionality (Chat · Comments · Ledger ·
Process, and eventually control).

## Why it is not a rewrite

cospan is already layered **L1 (watch) → L2 (fold) → L3 (render)**, and
`substrate::Fold` is a *presentation-agnostic in-memory model*. The ratatui TUI
is **one** renderer over it. A phone frontend is a **second renderer over the same
fold** — a new face, not a new brain.

```
        .kan/log, files, transcripts            ← substrate (unchanged)
                  │  poll-and-fold loop (exists)
          substrate::Fold  +  comments  +  chat sessions
                  │
        ┌─────────┴──────────── operations core ───────────┐
        │  read: serialize the fold      write: comment CRUD (S1),
        │                                 later spawn/kill/redirect (P3)
        └───────┬───────────────┬───────────────┬───────────┘
          TUI (L3, exists)   cospan mcp (S5)   HTTP+WS API   ← new
                                                    │  secure socket
                                              phone web client   ← new
```

The operations core the **S5 MCP server** needs (read the fold, write a comment)
is the *same* core this API needs. One core, several transports: stdio-MCP for
agents, HTTP/WS for the phone. S5 is a down payment on this, not a detour.

## The three genuinely new pieces

1. **A thin local server.** Run the existing fold loop; serialize the fold to JSON
   on a `GET`; **push** updates over a WebSocket whenever the loop re-folds. Small
   `axum`/`tokio` server — and S5 already pulls `tokio` into the tree, so it is not
   a new dependency shock. Note: `telos/poll-dont-subscribe` governs how cospan
   reads the *substrate* (kan/git); it does not forbid cospan *pushing* to its own
   clients, a channel cospan owns. WS push here is consistent.

2. **The web client.** A responsive PWA rendering the four tabs, touch-first. This
   is the real net-new cost — a **separate non-Rust front-end codebase**
   (React/Svelte/…), genuine surface to build and maintain. cospan could serve the
   static bundle itself.

3. **Secure transport — the crux.** This is a remote channel into a machine
   running autonomous agents; once it can write (and eventually spawn/kill/redirect)
   it is effectively a remote control line. Design stance:
   - **No public-internet exposure.** Use an **overlay network (Tailscale /
     WireGuard)**: device-authenticated, end-to-end encrypted, phone and laptop on
     a private mesh, nothing open to the world. Solves "secure socket to my phone"
     and dodges self-signed-cert-on-mobile pain.
   - **App-level auth too** (token / mTLS) — defense in depth even over the overlay.
   - **The kan signing seed never leaves the laptop.** The phone sends *intents*;
     cospan executes and **signs** on the laptop. The phone is a thin client with no
     identity of its own — so every mobile action still lands as a signed, audited
     kan claim (`kan-is-truth` gives the audit log for free).
   - **Read and write are different risk tiers.** Read-only mobile is low-stakes;
     the control plane (killing/redirecting agents from a pocket) deserves explicit
     auth and likely per-action confirmation.

## Telos fit — and one honest tension

- **`observe-now-control-later`** — this is *the* vehicle for it: read-only API
  first (observe), the write/control channel later. `command_bus::WriteChannel` is
  the reserved seam.
- **`kan-is-truth`** — the API serves projections; writes go through the same
  explicit-action path; the seed stays on the laptop.
- **Tension with `telos/disposable`.** cospan today is a throwaway sidecar you can
  `Ctrl-C`. A long-lived hosted server + a maintained web app + an overlay network
  is materially more infrastructure — this pulls hard against "disposable." Name it
  and decide it deliberately (record with `day telos tension` if pursued), do not
  slide into it.

## Phasing (when we do it)

1. Read-only **localhost** HTTP/WS API over the existing fold (JSON + push-on-refold).
2. Minimal **read-only web client**, served over LAN.
3. **Secure remote** access (overlay network) + app auth.
4. **Comment writes** over the API — the observe → control transition (reuses the
   S5 operations core; seed stays on the laptop).
5. Full **P3 control plane** (spawn/kill/redirect) — the sensitive tier, needs a
   real authz model and per-action confirmation.

## Relationship to current work

Parallel to, and larger than, any slice of the comment authoring milestone (it is a
transport layer plus a whole client). It must not derail S4 (promote-to-kan) or S5
(comment MCP). S5's read/write operations core is the shared foundation; build this
on top of it, later.
