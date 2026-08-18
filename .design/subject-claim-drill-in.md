# Feature: Per-subject claim drill-in

## Summary
Add a read-only `cospan subject <repo> <subject>` command that lists one kan
subject's live claims — the natural drill-in from the P0 dashboard's
subjects-by-namespace summary into the actual claims a subject holds. It extends
the proven shell-and-fold spine with zero new dependencies and is the core fold
the claims browser will reuse in the TUI. It serves `telos/p0-spine`.

## Requirements
- REQ-1: Add a `Claim` struct to `src/substrate.rs` capturing the fields kan
  actually emits (verified against `kan show telos/p0-spine --json`, kan
  0.13.0-beta.1): `cid`, `kind`, `subject`, `author`, `recorded_at` (integer
  microseconds since the epoch), and the kind-dependent payload — `text`
  (present on Decision/Observation/Plan/Result) and `title` (present on Subject),
  both optional.
- REQ-2: Add `pub fn subject_claims(repo: &Path, subject: &str) -> Result<Vec<Claim>, String>`
  that shells `kan show <subject> --json` in `repo` — mirroring the existing
  `kan_status` helper — parses the **top-level** `claims` array (not a nested
  `subjects[].claims`, which the field does not have), and returns the claims
  sorted newest-first by `recorded_at` descending, tie-broken by `cid`.
- REQ-3: `subject_claims` re-derives nothing: it returns exactly the live-claim
  set kan's fold already produced (retracted and trust-excluded claims are
  dropped by kan before cospan sees them), honoring `telos/kan-is-truth`. No
  role-fold or recency logic is reimplemented in cospan.
- REQ-4: Add a `cospan subject <repo> <subject>` subcommand to `src/main.rs`'s
  dispatch that prints one line per claim: the `kind` (padded), a short author
  (the `did:key:` prefix stripped and truncated), a compact UTC timestamp from
  `recorded_at`, and a one-line summary — the first line of `text` where present,
  else the quoted `title`, else a kind label such as `(published to GitTree)`.
- REQ-5: `subject_claims` surfaces genuine failure through its `Err(String)`
  channel — a kan spawn failure or malformed JSON — and the `subject` subcommand
  prints that error and exits non-zero, never panicking. A *nonexistent* subject
  is **not** such a failure: kan's append-only model has no unknown-subject
  error (a subject is its claims), so an empty fold is reported as
  "no live claims — unused, or all claims retracted" and exits zero. Conflating
  the two would fake an error kan does not raise.

## Acceptance Criteria
- [x] AC-1: (covers REQ-1, REQ-2) A new unit test in `src/substrate.rs` parses a
  captured `kan show --json` payload into `Vec<Claim>` and asserts the claim
  count, the parsed kinds, and that the result is ordered newest-first by
  `recorded_at`. `cargo test` stays green.
- [x] AC-2: (covers REQ-3, REQ-4) `cospan subject ~/code/kan-tools/day telos/legible-process`
  (or, if that repo is absent, `cospan subject . telos/p0-spine`) prints that
  subject's folded claims — the done-when from `.dropbox/07-first-steps.md`
  Step 1.
- [x] AC-3: (covers REQ-4) A `Subject` claim in the output shows its title and a
  `Publication` claim shows a kind label; neither is dropped and neither renders
  a blank summary.
- [x] AC-4: (covers REQ-2) The most recently recorded claim prints first;
  reversing two claims' `recorded_at` in a test reverses their output order.
- [x] AC-5: (covers REQ-5) `cospan subject . telos/does-not-exist` exits zero and
  prints the "no live claims" note (kan raises no unknown-subject error); a
  genuine kan failure — e.g. running against a path with no `.kan/` — exits
  non-zero with kan's own error text rather than panicking.

## Architecture
The work lands entirely in the two existing files; no new module is introduced.

`src/substrate.rs` already holds the L2 fold model: `Subject`, `Dashboard`, the
`collect` entry point, and the `kan_status`/`day_status` shell-out helpers with
the shared `str_at` accessor. `subject_claims` is a sibling of `kan_status` and
copies its shape exactly — `Command::new("kan").args(["show", subject,
"--json"]).current_dir(repo)`, non-zero exit mapped to `Err` from stderr,
`serde_json` parse, field extraction via `str_at` plus an integer accessor for
`recorded_at`. This keeps the P0 "shell, don't link" decision intact
(`.dropbox/02-kan-day-integration.md` records the eventual library upgrade as
later work).

The verified payload shape is `{v, subject, subjects:[<name strings>], claims:[
{cid, kind, subject, author, recorded_at, text?, title?, artifacts[]}], trust,
excluded_by_trust}`. cospan reads only the top-level `claims`. Because kan has
already applied its live fold and trust filter, `subject_claims` is a pure
projection — the direct expression of `telos/kan-is-truth` at the fold layer.

`src/main.rs` gains a `subject` arm in the `match args.first()` dispatch beside
`demo`/`watch`/`watch-repo`, and a `subject_cmd(&args[1..])` that takes
`<repo> <subject>`, calls `substrate::subject_claims`, and prints the lines (or
the error to stderr with `std::process::exit(1)`). This is a one-shot read, not
a watch loop, so it does not touch `telos/poll-dont-subscribe`; the live TUI that
polls this fold arrives in Step 2. The usage string is extended to list the new
subcommand.

Ordering is cospan's own choice because kan 0.13.0-beta.1 emits no `rev` field
(the sort key `.dropbox/07-first-steps.md` anticipated); `recorded_at` is the
only monotone key, so newest-first-by-`recorded_at`, tie-broken by `cid`, is the
deterministic order. `recorded_at` is microseconds since the epoch and is
rendered as a compact UTC stamp.

This step is the `generative-build` atom consuming this `design-doc`; its
`out` is the `code-change`, and `atom/adversarial-review` audits the result
against this document next.

## Resolved Questions
- RQ-1: Ordering is newest-first by `recorded_at`, tie-broken by `cid` — no `rev`
  field exists in kan 0.13.0-beta.1, so chronological-descending is the
  deterministic choice and matches "what does this subject say now".
- RQ-2: Every live claim is shown with a kind-aware summary (text, else title,
  else a kind label), rather than filtering to narrative kinds — the drill-in
  should not hide that a subject was titled or published.
- RQ-3: The query is per-subject `kan show <subject> --json`, matching Step 1's
  spec and the existing per-command shell-out; the `kan#181` O(n^2) cost is
  accepted for a single small P0 subject and revisited when the TUI folds many
  subjects per tick.

## Open Questions

_None outstanding — the three architectural choices above were resolved during
design._

## Out of Scope
- The interactive TUI, key handling, and polling loop (Step 2 — ratatui).
- The two-pane subject-list + claim-detail split (Step 3).
- Any claim writing, editing, or comment surface (P1+).
- Linking the `kan` crate as a library instead of shelling out (a later read-path
  upgrade, per `.dropbox/02-kan-day-integration.md`).
- Re-implementing day's role-based fold or day-block parsing; cospan renders
  kan's live claims as-is.
