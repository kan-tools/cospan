# 06 — Roadmap, open decisions, naming

## Phasing (observe now, control later)

| phase | ships | notes |
|-------|-------|-------|
| **P0** | ✅ *first slice shipped*: `cospan watch-repo <path>` — poll `.kan/log/HEAD`, fold `kan status --json` + `day status` into a dashboard (process position · agents/handoff sessions · claims-by-subject). Shells to the binaries; plain-text render. | proves the L1→L2→render spine against a real `.kan/log`. TODO next: per-subject claim drill-in, then move to ratatui + link kan as a library. |
| **P1** | ✅ *shipped* (2026-08): the Comments tab — a collapsible file tree + syntax-highlighted (syntect/onig, ~150 langs incl. Lean) code pane, interactive authoring (add/reply/edit/delete/resolve), the sidecar store + re-localizer, promote-to-kan (`p`/`P` → `comment/<file>` claim), and `cospan mcp` **read** (`list_comments`/`get_thread`). Editor uses syntect, not tree-sitter. | the core doc-comment round trip — DONE |
| **P2** | ✅ `cospan mcp` **write** shipped (S5: `add_comment`/`reply`/`resolve`, agent identity from `KAN_AGENT`) + agent-to-agent comments. ⬜ still ahead: multi-worktree session picker + harness view + **constructed** agent hierarchy. | flat `stream_list` fallback if hierarchy held |
| **P3** | control plane over the command bus (spawn/kill/redirect); kan-mirrored comments; swap constructed hierarchy for ADR-75 vouching claims when kan ships them | the "control later" half |

### Recommended immediate next step
Wire the re-localizer into a **P0 watch-and-fold loop** against a real `.kan/log`
— point cospan at the `day` build itself and watch the process flow move. That's
the dogfooding target (using cospan to watch cospan/day get built), and it
exercises the whole L1→L2→render spine with the smallest surface.

## Open decisions (need a human call)

1. **Dispatch hierarchy in P2** — ship the *inferred* tree (authorship + `KAN_AGENT`
   + `cites` + handoff lineage, approximate), or hold the hierarchy view and show
   only the flat `stream_list` registry until kan#117/ADR-75 land? Product call:
   useful-but-approximate now vs. correct-but-flat until the substrate is ready.
2. **Sidecar location/format** — `<file>.cospan.jsonl` beside the file, or a
   `.cospan/comments/<path>.jsonl` tree? (Tree keeps working dirs clean; sibling is
   more discoverable.)
3. **Comment durability default** — confirmed **sidecar-only** default with an
   explicit human "persist to kan" shortcut. (Settled 2026-08-18; recorded here for
   provenance.)
4. **Editor scope** — read-only reviewer for P1, or minimal editing? Leaning
   read-first (AI writes; human reviews/comments).

## Naming lineage

Family is category theory (`kan` = **Kan extension**; `day` presumably for the
process/temporal layer). Siblings should stay in the family.

- **`cospan`** — the tool. `X → A ← Y`: open pieces joined along a shared interface
  — matches `day`'s atom in/out composition and the tool's "glue many into one
  apex view."
- **`lan`** — Left Kan extension. Reserved on crates.io (v0.0.1). *"It fills in what
  you left unsaid. ;)"*
- **`yoneda`** — the Yoneda lemma. Reserved on crates.io (v0.0.1). *"You are what
  you relate to. ;)"*

Both `lan` and `yoneda` are parked placeholders for future kan-tools uses.

## Provenance

This corpus consolidates a design conversation (2026-08-18) that started from
"vim is a pain, what's the modern CLI editor" → Helix → "actually I want CLI
Google-Docs-with-comments for AI sessions" → this. The kan/day facts throughout
come from reading the real source in `~/code/kan-tools/{kan,day}`; treat any that
name a specific file/flag as verify-before-relying, since those repos move fast.
