# Feature: Comment sidecar store with live re-localization

## Summary
Give cospan its reason to exist beyond a viewer: an ephemeral, per-file comment
sidecar whose comments are pinned to a text fingerprint and re-resolved against
the current file on every read, into Anchored / Drifted / Unresolvable. Add
`cospan comment add` to drop a comment and `cospan comments` to list each with
its live state. This is the headless doc-comment round trip and the start of P1.

## Requirements
- REQ-1: A new module (src/comments.rs) defines a serializable `Comment` — id,
  an anchor (target, before, after, line_hint, base_hash), body, author,
  created_at, resolved — persisted as JSONL, one record per line, under a
  gitignored .cospan/comments/ tree keyed by the file's repo-relative path. A
  serde round-trip (`serde` derive) preserves every field.
- REQ-2: `cospan comment add <file> --line N [--ctx C] <body>` fingerprints the
  target line via `Anchor::from_file` (src/lib.rs) on the current file content,
  records the content's `base_hash`, appends the record to that file's sidecar,
  and prints the new comment's id and its (Anchored) localization.
- REQ-3: `cospan comments <file>` lists each stored comment with its live
  localization — the `State` (Anchored/Drifted/Unresolvable), span, and
  confidence returned by `relocalize` (src/lib.rs) against the current file
  content.
- REQ-4: Incremental last-seen tracking: relocalization runs against the stored
  (last-seen) fingerprint; when a comment resolves to an Anchored or Drifted
  span, its anchor is re-captured at the new match and `base_hash` updated to the
  current content, then written back — so a run of small edits does not
  accumulate drift. An Unresolvable comment's anchor is left unchanged, there
  being no reliable position to move it to.
- REQ-5: The sidecar is cospan's only owned, mutable state and is ephemeral by
  default: nothing is written to kan. This is the single exception
  `telos/kan-is-truth` names and the owned state `telos/disposable` allows;
  .cospan/ is gitignored and remains covered by the `no-tracked-junk` witness.
- REQ-6: The path mapping, serialization, and the localize-and-update step are
  pure or take content as an argument, so the whole round trip — add, edit,
  re-localize — is exercised by unit tests with no TTY and no real editor.

## Acceptance Criteria
- [ ] AC-1: (covers REQ-1, REQ-6) A unit test serializes a `Comment` to a JSONL
  line and parses it back, asserting every field (including the nested anchor and
  base_hash) survives the round trip.
- [ ] AC-2: (covers REQ-1) A unit test asserts the sidecar path for a
  repo-relative file path maps to .cospan/comments/<path>.jsonl.
- [ ] AC-3: (covers REQ-2, REQ-3) A unit test adds a comment to some content and,
  re-localizing against the unchanged content, gets one Anchored localization
  whose span is the target line.
- [ ] AC-4: (covers REQ-3, REQ-4) A unit test drives the round trip on in-memory
  content: after a pure line-shift edit the comment re-localizes Anchored at the
  shifted span; after an edit that removes the target it re-localizes
  Unresolvable and the stored anchor is left unchanged.
- [ ] AC-5: (covers REQ-4) A unit test asserts incremental tracking: a
  localize-and-update against a changed file re-captures the anchor (its
  base_hash changes to the new content's) when Anchored/Drifted, and does not
  when Unresolvable.
- [ ] AC-6: (covers REQ-5) `.gitignore` contains `.cospan/`, and
  `scripts/no-tracked-junk.sh` still exits 0 — no sidecar is git-tracked.

## Architecture
The anchoring half already exists: src/lib.rs exposes `Anchor` (target/before/
after/line_hint), `Anchor::from_file`, `relocalize(anchor, new_content) ->
Localization { state, span, confidence }`, and `State`. This step adds only the
storage-and-sharing half from .dropbox/03-comments.md — a new module,
src/comments.rs, exposed as `pub mod comments` from src/lib.rs beside `substrate`
and `tui`.

`Comment` carries a `StoredAnchor { target, before, after, line_hint, base_hash }`
— the lib `Anchor` fields plus the content hash at last capture — so the record
serializes independently of the lib type; `StoredAnchor::as_anchor` yields a lib
`Anchor` for `relocalize`. Records serialize with `serde` derive to one JSON
object per line (JSONL): appending a comment is appending a line, and the format
stays diff-friendly and human-readable. `base_hash` is a `std::hash::Hasher`
digest of the file content (deterministic `DefaultHasher`, no new dependency
beyond serde), used to short-circuit an unchanged file and to record provenance.

The store is a pure path map plus file I/O: `sidecar_path(repo_rel)` returns
.cospan/comments/<repo_rel>.jsonl; `load`/`save` read and write the JSONL; the
directory is created on first write. The .cospan/ tree is added to `.gitignore` —
it is disposable, ephemeral state, exactly the class `scripts/no-tracked-junk.sh`
already forbids from being tracked (`telos/disposable`), and nothing here ever
touches kan (`telos/kan-is-truth`'s sole exception).

The heart is `localize_and_update(comment, current_content) -> Localization`: it
builds the lib `Anchor` from the stored fingerprint, calls `relocalize`, and — on
an Anchored or Drifted result with a span — re-captures the fingerprint from the
current content at that span (`Anchor::from_file`) and updates `base_hash`, so
the next comparison is against the last-seen state rather than the original and
small edits do not accumulate drift (.dropbox/04-relocalizer.md). Unresolvable
leaves the anchor untouched. Taking `current_content` as an argument is what
keeps the whole round trip testable headless (REQ-6).

`src/main.rs` gains two dispatch arms: `comment add` builds a `Comment` (id from
created_at plus a counter, author defaulting to the local human) and appends it;
`comments` loads the file's sidecar, runs `localize_and_update` on each, writes
the updated records back, and prints one line per comment reusing the existing
localization rendering idiom (State tag, span, confidence). The `demo`, `watch`,
`watch-repo`, and `subject` subcommands are untouched.

Persist-to-kan — the opt-in `Observation` claim on `comment/<path-slug>` with a
fenced `cospan-comment` block described in .dropbox/03-comments.md — and the
`cospan mcp` server are explicitly deferred; the durability default is
sidecar-only, already settled.

## Resolved Questions
- RQ-1: Sidecar location is the .cospan/comments/<path>.jsonl tree (one JSONL per
  commented file under a single gitignored directory), not a sibling
  `<file>.cospan.jsonl` — it keeps working directories clean.
- RQ-2: The step ships both `cospan comment add` and `cospan comments`, so the
  headless round trip (drop a comment, edit, re-localize) is exercisable
  end-to-end rather than requiring hand-written JSONL.
- RQ-3: Re-localization uses incremental last-seen tracking — the anchor is
  re-captured and written back on each Anchored/Drifted resolution — rather than
  always matching against the frozen original, so accumulated drift is avoided.

## Open Questions

_None outstanding — the three scope choices above were resolved during design._

## Out of Scope
- Persisting comments to kan (the opt-in `comment/<path>` Observation claims and
  `kan publish` path in .dropbox/03-comments.md); the default is sidecar-only.
- The `cospan mcp` comment server and agent-to-agent comment writes.
- Threads and replies; the record carries the fields but this step lists
  top-level comments only.
- Rendering comments in the TUI gutter (a later editor/comment-view step); this
  step is the headless store + `comments` list.
- Multi-line anchor spans beyond what `Anchor::from_file` already captures.
