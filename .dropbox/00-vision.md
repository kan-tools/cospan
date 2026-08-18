# 00 — Vision

## Where this came from

The seed: *"CLI text-file Google Docs with comments"* — a way to watch AI outputs
update in real time and drop annotations next to the text to discuss back with a
running session. That exists in pieces (autoread editors for the live view;
CriticMarkup as a plaintext comment convention; Claude Artifact comment threads on
the web) but **not as an integrated CLI tool**, and nothing that plugs into the
`kan`/`day` substrate. So we build it.

## What cospan is

A single self-contained TUI that is simultaneously:

1. **A fleet view** — see every agent session / harness you're running, across
   worktrees, in one screen. Backgroundable (tmux, ssh, detach).
2. **A substrate inspector** — browse `kan` claims and watch the `day` process
   flow (current atom(s), bridge, telos, drift) live.
3. **A live editor** — file-type-agnostic, syntax-highlighted, auto-reloading as
   agents rewrite files under you.
4. **A comment surface** — anchored comments in a gutter next to the text they
   reference, that both you and agents can read (and, gated, write) — the round
   trip for talking back to a session mid-flight.

## Guiding principles

- **Poll, don't subscribe.** The substrate has no push. One debounced watch loop
  drives all views. (Non-negotiable; it's how kan/day actually work.)
- **kan is the only source of truth.** Everything cospan renders is a projection
  of the kan log — *except* ephemeral comments, the one bit of state cospan owns.
- **Honest ambiguity over false certainty.** `day` deliberately reports an
  ambiguous process position as a *list of candidates* rather than faking a
  cursor; the comment re-localizer reports `Unresolvable` rather than guessing
  wrong. cospan's UI mirrors this everywhere: show the list, don't invent a
  single answer.
- **Observe now, control later.** Ship read-only first; leave a clean command-bus
  seam for spawn/kill/redirect and claim-writes.
- **Disposable like day.** cospan should be throw-away-able without losing
  anything durable. Comments persist to kan only on an explicit human action.

## Non-goals (for now)

- Not an IDE. In AI-driven dev the agent writes; cospan is for *reading,
  reviewing, navigating, and commenting*. It doesn't need a plugin ecosystem.
- Not a task tracker. `day` deliberately declines "how far along are we"; cospan
  inherits that restraint.
- Not a collaboration server. No CRDT, no realtime multi-human editing. The
  "realtime" is agent-streaming + file autoreload, which is already solved.
