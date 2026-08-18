# 02 — kan / day integration

Ground truth from reading the source (2026-08-18). Both are Rust, both ship a
stdio MCP server, both are also Claude Code plugins.

## kan — the memory layer

Append-only, signed, content-addressed **claim** log, scoped to a git repo.
"Local reasoning, global coherence." Claims are immutable and identified by their
CID; conflict is read-time information (resolved by a **fold** parameterized by
whom you trust), not a write-time error.

### Storage
- `.kan/log/repo.car` — authoritative append-only signed log (CAR file + Merkle
  Search Tree), collection `tools.kan.claim`. Sibling `HEAD` file = current root
  CID. **Watch `HEAD` for change detection.**
- `.kan/overlay/repo.car` — foreign-authored claims (from clones).
- `.kan/index.sqlite` — disposable SQLite projection (table `claims_v2`:
  `content_cid, rev, author_did, author_agent, origin, subject_key, kind, raw`),
  rebuildable from the log. `raw` is the full re-decodable claim.
- `.claims/` (git-tracked) — the **publish** layer: `kan publish <subject>` writes
  signed claims as Markdown (CID-named) so they travel in `git diff`.

### Data model
- **Claim** = `{ author, workspace(Anchor), subject(SubjectRef), body, cites[],
  artifacts[], recorded_at? }` + signature. Frozen schema; new fields only as
  `Option<T>`.
- **Anchor** (the `workspace` field): `Workspace | Commit(sha) | Blob | FileAt(path,sha)
  | LineRangeAt(path, sha, span)`. ← **`LineRangeAt` is what comment persistence
  uses** (see 03).
- **Subject**: `Local(rkey)` — an author-local string name (e.g. `"login-bug"`),
  or `Anchor(...)`.
- **Body kinds**: narrative (`Observation`, `Plan`, `Decision`, `Blocker`,
  `Resolution`, `Result` — carry `text`), `Subject`, `Status`, `Relation`,
  `Retraction`, `Rejects`, `Publication`, `RoleDeclaration`, `Unknown`.

### Identity
- Per-repo `did:key` signs every claim (secret in `.kan/seed`, or OS keychain via
  `kan identity protect`; `KAN_IDENTITY_FILE=<path>` selects a key).
- **Roles** = distinct agent identities in one repo: `kan identity role add <name>`
  mints `.kan/roles.d/<name>` + a `RoleDeclaration`; write as it via
  `KAN_IDENTITY_FILE`; read with `--trust role:<name>`.
- `KAN_AGENT` env var = the harness tag on `author.agent` (legacy but still the way
  each harness is distinguished; e.g. `claude-code`).
- ADR-75 (planned): agents are derived roles, `HKDF(seed, "kan/v1/agent/"+label)`,
  vouched for by a signed claim from the root identity, scope bounded in the
  attestation. **This is the future substrate for the dispatch hierarchy.**

### How cospan reads it
- **Library (preferred, hot path):** `Workspace::open_read_only(cwd)` +
  `actions::show_all_json / status_json / issues_json / context_json`, or
  `fold::fold(claims, trust) -> FoldedView { classes: Vec<SubjectView> }`. Trust:
  `local_trust`, `trust_from(&[String])`.
- **Shell (isolation):** `kan <verb> --json` — check the `v` field (SCHEMA_VERSION 1).
  `kan show --all --json` = full graph in one process; `kan status --json` = cheap
  manifest.
- **MCP:** `kan mcp` — tools `show/status/issues/context`, resource
  `kan://claims/{subject}`.
- Ordering: always `(rev, cid)`, never `recorded_at`.

## day — the process layer

**Stateless. Stores nothing of its own.** Every durable thing is an ordinary kan
claim; day reads/appends only through kan's CLI. Advisory, never blocking. No
daemon, no event bus — **poll it**.

### The schema *is* kan subject conventions + fenced JSON blocks
- `telos/<slug>` (block `day-telos`) — a desired state up to weak equivalence.
- `atom/<slug>` (block `day-atom`) — a typed process unit:
  `{in:[...], out:[...], next:[...], revisits:[...], done:[...]}`. `next` edges
  form a **DAG**. `revisits` = feedback edges (not an ordering).
- `bridge/<slug>` (block `day-bridge`) — a planned arrangement via a grammar
  (`a > b` sequence, `a & b` concurrent, `a | b` alternatives). So the process
  model is a **DAG of atoms**, not a linear pipeline or state machine.
- `agents/handoff/<thread>` (block `day-handoff-scopes`) — per-session handoff.

### Process position is *inferred*, not stored
An atom is a *candidate* for "current" when its inputs are materially present and
its outputs aren't yet. **Ambiguity is reported (all candidates named), never
resolved.** Bounded by a cycle boundary (last `v*` tag). day refuses "how far
along are we." → cospan must render candidate sets, mirroring this.

### `.day/` render cache
Gitignored, display-only, derived, absence-is-never-an-error. Holds pre-rendered
statusline strings. **Never treat as source of truth.**

### How cospan reads it
- **MCP (`day mcp`):** `session_context` (teloi/atoms/open subjects/drift),
  `stream_list` (the multi-agent observability call over `agents/handoff/*`),
  `doctor`, `next{atom}`, `bridge_check`, `design_check`, `assess_*`. All read-only.
- **CLI:** `day status`, `day stream list`, `day doctor`, `day next <atom>`,
  `day config --json`.

## The dispatch hierarchy — honest status

**Not a day primitive yet.** Options, in order of what's available:

1. **Today:** show `day stream list` — a *flat* registry of concurrent
   sessions/agents. No parent/child.
2. **Constructed tree (cospan builds it):** from per-claim signing identity +
   `KAN_AGENT` harness tag + `cites` provenance + handoff-thread lineage. Edges are
   *inferred* → render unresolved ones as a list (the honest-ambiguity idiom).
3. **Future (forward-compatible):** kan ADR-75 vouching claims (parent vouches for
   child agent key, scope in attestation) folded under kan#117 per-read trust
   frames, surfaced by day's not-yet-built Frames. `TrustBase` generalizes from
   `author→weight` to `claim→weight`.

**Open decision (see 06):** ship the constructed tree in P2, or hold for ADR-75 and
show only the flat registry until then?
