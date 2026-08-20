---
{
  "v": 3,
  "cid": "bafyreidlbkxsdh37qtew63tmg3ljxpnotcsnurxrzwqhqr7g6dfwikqeoq",
  "sig": "46694e0c9f1acbcc7d0c80658181ee6f5e40672e75501c664d442f843030d244333b6533060b37c806d7864857b86cdbf0e7bf9755fda10a3bdd69eacefb06f3",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comment-gutter-view"
  },
  "kind": "observation",
  "cites": [],
  "rev": "223mtidzh4sjb",
  "seq": 0,
  "of": 8,
  "text_len": 193,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlU2RCYnJNNk5lWTRpOE1veHVGaXlEdHd4M0FISjhmakVkOTRnckgzMXI4WFllYWdlbnT2Z3N1YmplY3ShZUxvY2Fsc2NvbW1lbnQtZ3V0dGVyLXZpZXdpYXJ0aWZhY3RzgaFmQ29tbWl0eChkZjJjNDY2Y2FhN2NlZTA0MTE0Y2NkZmQyOWNlN2M0ZGE1YmQ3YjU2aXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWXJ+0V2X"
}
---

design doc .design/comment-gutter-view.md checked against the live design-doc schema: validation: 9 check(s), 0 failed, 0 warning(s), 0 unchecked, 0 open question(s) [doc 8403:d6d5922d1e81b087]
***8<***
---
{
  "v": 3,
  "cid": "bafyreie7xhvczub4nc3ndgesvokob2kdjttqmjjtvwby2kyhow2wz6txty",
  "sig": "20bd893d7a52e8febdb06fd67dd31f1b32e2b348b818e00914998a63227304de2e0fd63247b08c648e95c3d904557c5b1269e87609c86a7ebc383fd13a24c28a",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comment-gutter-view"
  },
  "kind": "plan",
  "cites": [
    "bafyreidlbkxsdh37qtew63tmg3ljxpnotcsnurxrzwqhqr7g6dfwikqeoq"
  ],
  "rev": "223mtidzhebrm",
  "seq": 1,
  "of": 8,
  "text_len": 596,
  "content": "p2Rib2R5oWRQbGFuoWR0ZXh0YGVjaXRlc4HYKlglAAFxEiBrCq8hn3+EyW9ubDbWm72umKTaRvHNoHhH5vDLZCoEdGZhdXRob3KiY2RpZHg5ZGlkOmtleTp6RG5hZVNkQmJyTTZOZVk0aThNb3h1Rml5RHR3eDNBSEo4ZmpFZDk0Z3JIMzFyOFhZZWFnZW509mdzdWJqZWN0oWVMb2NhbHNjb21tZW50LWd1dHRlci12aWV3aWFydGlmYWN0c4GhZkNvbW1pdHgoZGYyYzQ2NmNhYTdjZWUwNDExNGNjZGZkMjljZTdjNGRhNWJkN2I1Nml3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABllyftUeiQ=="
}
---

comment-gutter-view design (.design/comment-gutter-view.md): Surface the comment sidecar — which already works headless (`src/comments.rs`, `cospan comment add` / `cospan comments`) — inside the TUI as a fourth view: a picker of commented files, and for the selected file its content with an anchored comment gutter showing each comment's live `Anchored` / `Drifted` / `Unresolvable` state as the file changes. This is `telos/comment-roundtrip` made visible — "the reason cospan exists beyond a nice viewer." [validation: 9 check(s), 0 failed, 0 warning(s), 0 unchecked, 0 open question(s)]
***8<***
---
{
  "v": 3,
  "cid": "bafyreie6qik72hg4hmpf33dccuslketp27f5zkimxhk3mi7higqojshaci",
  "sig": "642ff3826c156c90e5b64042551f2f00bc498c8f491fde1cf6e87da2cf2f8f576396588f055b51efff361e5b7044d0e9737b671886c979292bca28f13e4fc666",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comment-gutter-view"
  },
  "kind": "subject",
  "cites": [],
  "rev": "223mtidzhgq6p",
  "seq": 2,
  "of": 8,
  "content": "p2Rib2R5oWdTdWJqZWN0omV0aXRsZXglRmVhdHVyZTogdGhlIGNvbW1lbnQgZ3V0dGVyIHZpZXcgKFAxKWxzdWJqZWN0X2tpbmRkSWRlYWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxzY29tbWVudC1ndXR0ZXItdmlld2lhcnRpZmFjdHOBoWZDb21taXR4KGRmMmM0NjZjYWE3Y2VlMDQxMTRjY2RmZDI5Y2U3YzRkYTViZDdiNTZpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZZcn7WWC8="
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreidavqtugk6hyqptpqltzx5xy2yekldpndr6fiulslqc5jexrvcqty",
  "sig": "e1f1330ea394e993276f9f9dbbf8265073c405e62c66a2d6d26d94da26999cd017768ea17c8ce0c4e687d89db4fd335be07b0a7fb71f4040de6739c4efb57ebc",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comment-gutter-view"
  },
  "kind": "decision",
  "cites": [
    "bafyreie7xhvczub4nc3ndgesvokob2kdjttqmjjtvwby2kyhow2wz6txty"
  ],
  "rev": "223mtidzhnw3z",
  "seq": 3,
  "of": 8,
  "text_len": 326,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgn7nqLNA8aLbRmJKrlODpQ0znBiUzrYONKwd1tWz6d55mYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxzY29tbWVudC1ndXR0ZXItdmlld2lhcnRpZmFjdHOBoWZDb21taXR4KGRmMmM0NjZjYWE3Y2VlMDQxMTRjY2RmZDI5Y2U3YzRkYTViZDdiNTZpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZZcn7Z79o="
}
---

RQ-1: The view re-localizes and re-anchors (via `localize_and_update` + `save`, gated by the per-file mtime), matching `cospan comments` and the sidecar's last-seen tracking, rather than a pure read from the frozen anchor — so drift does not accumulate as an agent rewrites the file, which is the whole point of the feature.
***8<***
---
{
  "v": 3,
  "cid": "bafyreifjdadwr34a5jlxrkmwix6ehkgexwkftiumewcgyrlgytympnwsxy",
  "sig": "ee13c6fcd5489acf8a5812635f4388de2f93c29fed6b8cbe7dfa526ab4b855ae763857fb241b85eb49a302696dbe8f9d5f8b33a7c9016b7e8e867bcb521df717",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comment-gutter-view"
  },
  "kind": "decision",
  "cites": [
    "bafyreie7xhvczub4nc3ndgesvokob2kdjttqmjjtvwby2kyhow2wz6txty"
  ],
  "rev": "223mtidzhuyop",
  "seq": 4,
  "of": 8,
  "text_len": 225,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgn7nqLNA8aLbRmJKrlODpQ0znBiUzrYONKwd1tWz6d55mYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxzY29tbWVudC1ndXR0ZXItdmlld2lhcnRpZmFjdHOBoWZDb21taXR4KGRmMmM0NjZjYWE3Y2VlMDQxMTRjY2RmZDI5Y2U3YzRkYTViZDdiNTZpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZZcn7deiw="
}
---

RQ-2: File selection for P1 is a picker over files that already have a sidecar (discovered from `.cospan/comments`), not the P2 session picker or an arbitrary file browser; the flagship demo is viewing existing comments live.
***8<***
---
{
  "v": 3,
  "cid": "bafyreihajdpyycsuowziqxa5rkhutkewsevauh3oppjryyxqw3u57kex5e",
  "sig": "38212055f8f3722b594bc9469891dc851bd483229178aafa98cabb6d27dbce566bb8b9fda814d61ab118771c0692ffdd8c7464cf32983c6197d1d29ffda6efea",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comment-gutter-view"
  },
  "kind": "decision",
  "cites": [
    "bafyreie7xhvczub4nc3ndgesvokob2kdjttqmjjtvwby2kyhow2wz6txty"
  ],
  "rev": "223mtidzi4cjg",
  "seq": 5,
  "of": 8,
  "text_len": 147,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgn7nqLNA8aLbRmJKrlODpQ0znBiUzrYONKwd1tWz6d55mYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxzY29tbWVudC1ndXR0ZXItdmlld2lhcnRpZmFjdHOBoWZDb21taXR4KGRmMmM0NjZjYWE3Y2VlMDQxMTRjY2RmZDI5Y2U3YzRkYTViZDdiNTZpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZZcn7hIYU="
}
---

RQ-3: Live re-read uses a second per-file mtime gate inside the single existing tick, not a new watch loop or thread (`telos/poll-dont-subscribe`).
***8<***
---
{
  "v": 3,
  "cid": "bafyreiajnz3q43kv3g3wgp37fjsivtdr52nuifjejs4hi63rmw5ttniziy",
  "sig": "2d3a8c2b9bb47d647c689973b1bfae29da0d8d49f8573ee0108b6741e1b15e9e710a0ca7d2c954870449af4055fa31edf156e20c57e80645d9997b398e452ced",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comment-gutter-view"
  },
  "kind": "decision",
  "cites": [
    "bafyreie7xhvczub4nc3ndgesvokob2kdjttqmjjtvwby2kyhow2wz6txty"
  ],
  "rev": "223mtiewmmzf3",
  "seq": 6,
  "of": 8,
  "text_len": 1312,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgn7nqLNA8aLbRmJKrlODpQ0znBiUzrYONKwd1tWz6d55mYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxzY29tbWVudC1ndXR0ZXItdmlld2lhcnRpZmFjdHOBoWZDb21taXR4KGRmMmM0NjZjYWE3Y2VlMDQxMTRjY2RmZDI5Y2U3YzRkYTViZDdiNTZpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZZcrkpfPY="
}
---

adversarial review of comment-gutter-view: SHIP (after one BLOCK-and-fix). Independent Opus review BLOCKED a None-sentinel collision: comment_mtime=None meant both "force reload" and "mtime unavailable (source deleted)", so selecting a commented file whose source was deleted hit should_refold(None,None)=false and early-returned, leaving the previous file's content + gutter on screen under the wrong title and silently dropping the deleted file's comments — violating REQ-2, REQ-5, telos/honest-ambiguity. Reviewer proved it with a repro. Fixed: added comment_loaded: Option<PathBuf> tracking the actually-loaded path; refresh_comments reloads when comment_loaded != selected OR (same-file AND content changed), so a missing source's None mtime can't masquerade as current; a missing source reads as empty -> all comments Unresolvable -> resolve-by-hand list. Also fixed two secondary findings: collect_sidecars uses file_type() (no symlink follow) to close a cycle stack-overflow; gutter_lines prefers the selected comment's marker for the highlight on overlapping lines. Added regression tests (deleted-source + happy-path load). Re-verified by the same reviewer against its repro: deleted-source path honest, no same-file save storm, no regressions. 69 tests, clippy -D warnings, fmt clean. Verdict: SHIP.
***8<***
---
{
  "v": 3,
  "cid": "bafyreibu2vgcqamk6skvaicdfutwllvcxslgs4djhgyijqos2rp3ptbeyi",
  "sig": "9b3e3cbd1c0c460de204ff45cbcf086a9bc260f9514b277800ce8e634c0665de78edce51e76bd97d23f18c34a6d544659840488dff8d0cfcfc258c6f0f5e3c48",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comment-gutter-view"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223mtiewmudid",
  "seq": 7,
  "of": 8,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxzY29tbWVudC1ndXR0ZXItdmlld2lhcnRpZmFjdHOBoWZDb21taXR4KGRmMmM0NjZjYWE3Y2VlMDQxMTRjY2RmZDI5Y2U3YzRkYTViZDdiNTZpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZZcrktJVc="
}
---
