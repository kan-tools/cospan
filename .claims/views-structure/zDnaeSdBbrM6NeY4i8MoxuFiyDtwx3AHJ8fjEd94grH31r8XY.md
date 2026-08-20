---
{
  "v": 3,
  "cid": "bafyreicznvfnsaumgqnupcklpprsfkokdgqiirjmckopuohfnlr44mgcym",
  "sig": "5dbd9ddb54d2fd1a4eb8c9504015826cb17d4550a83b1fc9e0be9f98782b82ea4c58d08ce01556168be02033b0df7b5ee84b22caf8594894b3016200d1d90047",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "views-structure"
  },
  "kind": "observation",
  "cites": [],
  "rev": "223mtjhxi6r7z",
  "seq": 0,
  "of": 3,
  "text_len": 778,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlU2RCYnJNNk5lWTRpOE1veHVGaXlEdHd4M0FISjhmakVkOTRnckgzMXI4WFllYWdlbnT2Z3N1YmplY3ShZUxvY2Fsb3ZpZXdzLXN0cnVjdHVyZWlhcnRpZmFjdHOBoWZDb21taXR4KGUzNjNlY2ZiNzFlYzc0Y2Q5ZDYxZTgwMmFkYzgwZWRhMzg3NTBlNTdpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZZe3riWM4="
}
---

Footer source finding: cospan's process status belongs in a bottom FOOTER (status bar), not the current top panel, and its source is `day status-line` — the exact text Claude Code renders (configured as the harness statusLine command: `day status-line`). It is CACHE-ONLY: reads a pre-rendered cache written by `day hook session-start`, never kan/git, because Claude Code cancels a status line at 300ms. Cache at .day/statusline and .day/statusline.variants; the variants file carries width/style variants tagged `#day-footer <emoji|plain> <width>` (e.g. emoji 43, plain 57). Not `day status` (terse single-atom report for people) and not `day hook session-start` (advisory session banner). cospan should poll the cache each tick / on mtime and pick the width-matched variant.
***8<***
---
{
  "v": 3,
  "cid": "bafyreiftkay6ulyd3v3ppdsfdk2vxvrf57hevbc4hh37dfumsog63bxhna",
  "sig": "29d958667c4b8c8a9c916b0a697886c7c7cb4ab21ef11170bc8a685ca630026d143bafd7d3dce2b55865c74751b9bcdd09bef9b3f8cac2491f0d8b9a9ff0ac68",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "views-structure"
  },
  "kind": "decision",
  "cites": [],
  "rev": "223mtjhxihlbv",
  "seq": 1,
  "of": 3,
  "text_len": 846,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlU2RCYnJNNk5lWTRpOE1veHVGaXlEdHd4M0FISjhmakVkOTRnckgzMXI4WFllYWdlbnT2Z3N1YmplY3ShZUxvY2Fsb3ZpZXdzLXN0cnVjdHVyZWlhcnRpZmFjdHOCoWZDb21taXR4KGUzNjNlY2ZiNzFlYzc0Y2Q5ZDYxZTgwMmFkYzgwZWRhMzg3NTBlNTehZkZpbGVBdIJ3LmRyb3Bib3gvMDUtdmlld3MtdXgubWR4KGUzNjNlY2ZiNzFlYzc0Y2Q5ZDYxZTgwMmFkYzgwZWRhMzg3NTBlNTdpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZZe3rmxIk="
}
---

Revised UI structure (2026-08-20, recorded in .dropbox/05-views-ux.md): four tabs plus an always-on bottom footer. Tabs: Chat (cross-harness live session BUFFERS — open architecture question, tmux-capture vs PTY-ownership vs transcript files, decide in its own pass before building), Comments (the editor view — evolving to a collapsible file tray with a full FS tree + git state, visual diffs, no bottom strip, comment overflow in a shortcut popup), Ledger (today's Browser / kan claims, renamed), Process (today's Atoms+Telos reimagined: atoms as a drill-down FLOWCHART, teloi as a drill-down list with statement/witnesses/tensions). Footer = day status-line (see the observe). Sequencing: footer first (small, validated, fixes the mis-sourced top panel), then Process reshape, Ledger rename folded in; Chat deferred to a design discussion.
***8<***
---
{
  "v": 3,
  "cid": "bafyreig73n4n7wshibdrwboxg44xasihqyceir7cctvtwm6j7nr32d4x3u",
  "sig": "f7034639bcb8b5d83a83e9d21b4527f9e3c124a95efe9db2e78b6d0182490d3a5da16174c19b0c59c088d5961c8177baa4cef3bf1fb3fcb81a46f5e0bdb7fffe",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "views-structure"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223mtjikix2bl",
  "seq": 2,
  "of": 3,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxvdmlld3Mtc3RydWN0dXJlaWFydGlmYWN0c4GhZkNvbW1pdHgoZTM2M2VjZmI3MWVjNzRjZDlkNjFlODAyYWRjODBlZGEzODc1MGU1N2l3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABll7oO6Ahg=="
}
---
