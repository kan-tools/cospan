---
{
  "v": 3,
  "cid": "bafyreib6c3zrovinknsulamfui3dvpivjvr5r63lx73pb6b5ix7hrtkn44",
  "sig": "527a3a3b1dfb548a82c7435527c92645e3011a6b9c635b89d595a89defd9c97222a0f0b7a14624ffa218e72bbdb92c3a7c591003acf628f187c7f3a7ca68353e",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comment-threads"
  },
  "kind": "observation",
  "cites": [],
  "rev": "223mtiktmmpfq",
  "seq": 0,
  "of": 8,
  "text_len": 189,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlU2RCYnJNNk5lWTRpOE1veHVGaXlEdHd4M0FISjhmakVkOTRnckgzMXI4WFllYWdlbnT2Z3N1YmplY3ShZUxvY2Fsb2NvbW1lbnQtdGhyZWFkc2lhcnRpZmFjdHOBoWZDb21taXR4KGU5YTFjYmEwYThhYmVjZjE3NzU5NGRlZWY4ZmFmN2EwMDdlZWYzMTlpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZZdDMpVQ8="
}
---

design doc .design/comment-threads.md checked against the live design-doc schema: validation: 9 check(s), 0 failed, 1 warning(s), 0 unchecked, 0 open question(s) [doc 5498:74e0e2f42dd97c04]
***8<***
---
{
  "v": 3,
  "cid": "bafyreihrmnmnls4x7d7mz7pyhihdk6rfbykfufkeqsiyd5tnxaiukbgbfe",
  "sig": "5d24f0711075c9546233dbff0bfbe115c8aeaf3543735efe11deffefa8b8c3f85dcaf5128bb7e3fad7610719f3965265be5886e3be073f2e40adc12f3ae26836",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comment-threads"
  },
  "kind": "plan",
  "cites": [
    "bafyreib6c3zrovinknsulamfui3dvpivjvr5r63lx73pb6b5ix7hrtkn44"
  ],
  "rev": "223mtiktmubqn",
  "seq": 1,
  "of": 8,
  "text_len": 569,
  "content": "p2Rib2R5oWRQbGFuoWR0ZXh0YGVjaXRlc4HYKlglAAFxEiA+FvMXVQ1TZUWBhaI2Or0VTWPY+2u/9vD4PUX+eM1N52ZhdXRob3KiY2RpZHg5ZGlkOmtleTp6RG5hZVNkQmJyTTZOZVk0aThNb3h1Rml5RHR3eDNBSEo4ZmpFZDk0Z3JIMzFyOFhZZWFnZW509mdzdWJqZWN0oWVMb2NhbG9jb21tZW50LXRocmVhZHNpYXJ0aWZhY3RzgaFmQ29tbWl0eChlOWExY2JhMGE4YWJlY2YxNzc1OTRkZWVmOGZhZjdhMDA3ZWVmMzE5aXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWXQzLR5u"
}
---

comment-threads design (.design/comment-threads.md): Give sidecar comments a reply thread and a resolve action. A comment gains a `thread: Vec<Reply>` (the field the `Comment` doc comment already anticipates), the CLI gains `cospan comment reply` and `cospan comment resolve`, and the TUI Comments view renders the selected comment's full thread — root plus indented, attributed replies, with a resolved marker. Writes stay on the CLI/MCP path; the TUI still only reads and re-anchors. [validation: 9 check(s), 0 failed, 1 warning(s), 0 unchecked, 0 open question(s)]
***8<***
---
{
  "v": 3,
  "cid": "bafyreigqg3zhsw724k4si6xho4qzxd4r35z7dllzte2cdwwohejj5luepu",
  "sig": "8b4470308d383893ed1e7e1eb1479ad2dcf609e3604b948330a30b83e796f3c13b164bac0d1bbeea3be59fa931eea3f0544b5660907e42432a04e74f0b5e315d",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comment-threads"
  },
  "kind": "subject",
  "cites": [],
  "rev": "223mtiktmwukf",
  "seq": 2,
  "of": 8,
  "content": "p2Rib2R5oWdTdWJqZWN0omV0aXRsZXgqRmVhdHVyZTogY29tbWVudCB0aHJlYWRzIChyZXBseSArIHJlc29sdmUpbHN1YmplY3Rfa2luZGRJZGVhZWNpdGVzgGZhdXRob3KiY2RpZHg5ZGlkOmtleTp6RG5hZVNkQmJyTTZOZVk0aThNb3h1Rml5RHR3eDNBSEo4ZmpFZDk0Z3JIMzFyOFhZZWFnZW509mdzdWJqZWN0oWVMb2NhbG9jb21tZW50LXRocmVhZHNpYXJ0aWZhY3RzgaFmQ29tbWl0eChlOWExY2JhMGE4YWJlY2YxNzc1OTRkZWVmOGZhZjdhMDA3ZWVmMzE5aXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWXQzLmmo"
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreic2rhucs3elqymfowzbfidgvkaconof26zcyx4hgzpsrwui4dvwdi",
  "sig": "c51b12802e27e88145bc29c4de4f24eee7464c97f0135d9517f15479c4807b501298dba5fb842c076624082f2ed8940d0622126d685b787546c1815170f2148e",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comment-threads"
  },
  "kind": "decision",
  "cites": [
    "bafyreihrmnmnls4x7d7mz7pyhihdk6rfbykfufkeqsiyd5tnxaiukbgbfe"
  ],
  "rev": "223mtiktn6fpz",
  "seq": 3,
  "of": 8,
  "text_len": 230,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIg8WNY1cuX+P7M/fg6DjV6JQ4UWhVEhJGB9m24EUUEwSlmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxvY29tbWVudC10aHJlYWRzaWFydGlmYWN0c4GhZkNvbW1pdHgoZTlhMWNiYTBhOGFiZWNmMTc3NTk0ZGVlZjhmYWY3YTAwN2VlZjMxOWl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABll0MzIuVw=="
}
---

RQ-1: Replies are authored only via the CLI (and later MCP), never from the TUI, keeping the observation view read-only; the TUI displays threads. This matches how `comment add` already works and `telos/observe-now-control-later`.
***8<***
---
{
  "v": 3,
  "cid": "bafyreie6pfems75how4nlmxo337yog36szlobq3rf6bwzzogfz6cgftdou",
  "sig": "6835d8086a4cc1f18bf84009ee31b8ebbf5c77ac1b661c1bfd2d27ede3649cb740067285aed25b0b64406388987d984945c4a7e73e34a8976e9a46e760099576",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comment-threads"
  },
  "kind": "decision",
  "cites": [
    "bafyreihrmnmnls4x7d7mz7pyhihdk6rfbykfufkeqsiyd5tnxaiukbgbfe"
  ],
  "rev": "223mtiktngc6t",
  "seq": 4,
  "of": 8,
  "text_len": 145,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIg8WNY1cuX+P7M/fg6DjV6JQ4UWhVEhJGB9m24EUUEwSlmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxvY29tbWVudC10aHJlYWRzaWFydGlmYWN0c4GhZkNvbW1pdHgoZTlhMWNiYTBhOGFiZWNmMTc3NTk0ZGVlZjhmYWY3YTAwN2VlZjMxOWl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABll0MzYgMA=="
}
---

RQ-2: Back-compat is handled by `#[serde(default)]` on the new field rather than a schema version bump — a pre-threads sidecar loads unchanged.
***8<***
---
{
  "v": 3,
  "cid": "bafyreiazuysykxmxd52iwrwxcucr6yef4rjbqjjzuv2i6gmb26nvvjngfm",
  "sig": "a43a48330f67f0e37ab5b55ebef30925baf446790de593bd4e2e18e64247035e3847a2ecb8fc4bda9270540e3967d118fcaf2037ae42341b165dc45a52527bf2",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comment-threads"
  },
  "kind": "decision",
  "cites": [
    "bafyreihrmnmnls4x7d7mz7pyhihdk6rfbykfufkeqsiyd5tnxaiukbgbfe"
  ],
  "rev": "223mtiktnnx4e",
  "seq": 5,
  "of": 8,
  "text_len": 153,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIg8WNY1cuX+P7M/fg6DjV6JQ4UWhVEhJGB9m24EUUEwSlmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxvY29tbWVudC10aHJlYWRzaWFydGlmYWN0c4GhZkNvbW1pdHgoZTlhMWNiYTBhOGFiZWNmMTc3NTk0ZGVlZjhmYWY3YTAwN2VlZjMxOWl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABll0Mznz1A=="
}
---

RQ-3: `resolve` toggles the existing `resolved` flag on (no un-resolve in this cut); the resolved state renders but does not hide or reorder the comment.
***8<***
---
{
  "v": 3,
  "cid": "bafyreigu7dlb7zv6utrsrmglipoznzqb6ug7qa6tkn2ubd326yaglvemou",
  "sig": "414225423974a3979a907d5da27f6c4d20e0c6879ffff72a51c2548962c2286d7ebd5423c795c1f1bb9a16c72d85e64e6a6042b9b54743821c52874baa34b8b7",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comment-threads"
  },
  "kind": "decision",
  "cites": [
    "bafyreihrmnmnls4x7d7mz7pyhihdk6rfbykfufkeqsiyd5tnxaiukbgbfe"
  ],
  "rev": "223mtilb77riv",
  "seq": 6,
  "of": 8,
  "text_len": 963,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIg8WNY1cuX+P7M/fg6DjV6JQ4UWhVEhJGB9m24EUUEwSlmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxvY29tbWVudC10aHJlYWRzaWFydGlmYWN0c4GhZkNvbW1pdHgoZTlhMWNiYTBhOGFiZWNmMTc3NTk0ZGVlZjhmYWY3YTAwN2VlZjMxOWl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABll0TlLddA=="
}
---

adversarial review of comment-threads: SHIP. Independent Opus review, all green (75 tests, clippy -D warnings, fmt). serde back-compat is bidirectional: thread has #[serde(default)] and no skip_serializing_if (save always writes it) and no deny_unknown_fields (old builds read new records); a pre-threads sidecar line loads with an empty thread (tested). CLI arg parsing correct on every probe: reply with 2 args -> empty body guard fires; resolve rejects 3+ args; unknown id exits(1) BEFORE save so an unchanged file is never rewritten. add_reply/resolve match first id, empty slice -> false no panic. thread_lines is a pure render (root + attributed indented replies + [resolved]); the TUI never authors a reply (read-only view held, observe-now-control-later); nothing touches kan. Verified end-to-end via CLI: two replies + resolve -> list shows (2 replies) [resolved]; unknown id errors. 4 tests map to AC-1..4; no stale Comment constructions. Verdict: SHIP.
***8<***
---
{
  "v": 3,
  "cid": "bafyreifikgykqwes3iq2h4uhy6f2qwb7vwc7omnwgidh5xln4rwclum3ne",
  "sig": "08ce9a3e398d190a288667280c5c4fafc8ec86725bc3cb7bc026553583ff6c58162f84026f8db9afb361f81129c2d2a9ebc0e7c4ca797d7ce6d863a97765e644",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comment-threads"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223mtilb7hjqn",
  "seq": 7,
  "of": 8,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxvY29tbWVudC10aHJlYWRzaWFydGlmYWN0c4GhZkNvbW1pdHgoZTlhMWNiYTBhOGFiZWNmMTc3NTk0ZGVlZjhmYWY3YTAwN2VlZjMxOWl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABll0Tla+aw=="
}
---
