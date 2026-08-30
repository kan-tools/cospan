---
{
  "v": 3,
  "cid": "bafyreiduvm77jkas4npqtjyye26gjg3nrh33shj2nhjyblvjb5ptbrvva4",
  "sig": "14be96f28602353b2a244dca6de47b14c13e4c604dba0401b8d837e15d57e37d6c0a9402dbfcc17c69e4acb80c6f67a527adb64e3d9e196608cc4c68cc381775",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "web-view-ux-pass"
  },
  "kind": "observation",
  "cites": [],
  "rev": "223muappnrrix",
  "seq": 0,
  "of": 9,
  "text_len": 191,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlU2RCYnJNNk5lWTRpOE1veHVGaXlEdHd4M0FISjhmakVkOTRnckgzMXI4WFllYWdlbnT2Z3N1YmplY3ShZUxvY2FscHdlYi12aWV3LXV4LXBhc3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCg5YjBjYWFhMjU1ZDMyMzE1ZjE0YjNjOWEyZmUwZDgzODNhMDg2OWFiaXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWjVrO91o"
}
---

design doc .design/web-view-ux-pass.md checked against the live design-doc schema: validation: 10 check(s), 0 failed, 1 warning(s), 0 unchecked, 0 open question(s) [doc 9191:2a59e4b522791e5a]
***8<***
---
{
  "v": 3,
  "cid": "bafyreif7tw7wmleem4j2fsb7mk3uh347vlptjm4u26avaff4gq7wehorlm",
  "sig": "928d1a4cbec129b370c1fa90f07137d738d74ebd66be5da8abab8e8ae3fde6da14dc8af76f655f42dc8a356e326b6d536b761b4544d3b02fe4e1591b6e2b05f8",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "web-view-ux-pass"
  },
  "kind": "plan",
  "cites": [
    "bafyreiduvm77jkas4npqtjyye26gjg3nrh33shj2nhjyblvjb5ptbrvva4"
  ],
  "rev": "223muappo6xhd",
  "seq": 1,
  "of": 9,
  "text_len": 846,
  "content": "p2Rib2R5oWRQbGFuoWR0ZXh0YGVjaXRlc4HYKlglAAFxEiB0qz/0qBLjXwmnGCa8ZJttife5HTpp04CuqQ9fMMa1B2ZhdXRob3KiY2RpZHg5ZGlkOmtleTp6RG5hZVNkQmJyTTZOZVk0aThNb3h1Rml5RHR3eDNBSEo4ZmpFZDk0Z3JIMzFyOFhZZWFnZW509mdzdWJqZWN0oWVMb2NhbHB3ZWItdmlldy11eC1wYXNzaWFydGlmYWN0c4GhZkNvbW1pdHgoOWIwY2FhYTI1NWQzMjMxNWYxNGIzYzlhMmZlMGQ4MzgzYTA4NjlhYml3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABlo1a0J1Ng=="
}
---

web-view-ux-pass design (.design/web-view-ux-pass.md): A UX pass on the embedded `cospan serve` web view (`src/web/index.html`, served at `GET /`): a **Comments** tab (files-with-comments → a file's comments → a comment's full thread) over a regularized read API, **claim drill-in** in the Browse tab (tap a claim for its full text, resolved cites, artifacts), and two resilience fixes recorded on `mobile-web-view` (a capped, visibility-aware `/stream` reconnect and no redundant first render). It stays a single self-contained embedded page (`telos/disposable`) and read-only (`telos/observe-now-control-later`); the new server surface is one index endpoint that establishes the resource-collection shape the Phase-2 web client and later write verbs extend. [validation: 10 check(s), 0 failed, 1 warning(s), 0 unchecked, 0 open question(s)]
***8<***
---
{
  "v": 3,
  "cid": "bafyreifzkfksudjfohljlk3iy4cqfequryt2yzkzk3ydphuc3qcdwcbrna",
  "sig": "a556c7edae08f20d5213a31aba3bf64d34f10253b1b20f879ef913eb91c85458560b29c8e546c8387df75cc2be6bd625891f6c395fc00e9733651af934405e4f",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "web-view-ux-pass"
  },
  "kind": "subject",
  "cites": [],
  "rev": "223muappoddp5",
  "seq": 2,
  "of": 9,
  "content": "p2Rib2R5oWdTdWJqZWN0omV0aXRsZXhSRmVhdHVyZTogV2ViIHZpZXcgVVggcGFzcyDigJQgY29tbWVudHMsIGNsYWltIGRyaWxsLWluLCByZXNpbGllbnQgc3RyZWFtIChTbGljZSBBKWxzdWJqZWN0X2tpbmRkSWRlYWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxwd2ViLXZpZXctdXgtcGFzc2lhcnRpZmFjdHOBoWZDb21taXR4KDliMGNhYWEyNTVkMzIzMTVmMTRiM2M5YTJmZTBkODM4M2EwODY5YWJpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZaNWtEpi4="
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreiapcmy7pzarjspo2nrijsh3myacogah2my66ejpmsqoywjp5kgiv4",
  "sig": "619a11231a7f03dede7190fb5dc6722fe2d4808b5da61dbc1bcde16fd4743cd31ee9ff9bf82aa90d0578621f3c7b29ef2ed21f7097206b457cedb0196b9575f2",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "web-view-ux-pass"
  },
  "kind": "decision",
  "cites": [
    "bafyreif7tw7wmleem4j2fsb7mk3uh347vlptjm4u26avaff4gq7wehorlm"
  ],
  "rev": "223muappoqs4z",
  "seq": 3,
  "of": 9,
  "text_len": 349,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgv52/ZiyEZxOiyD9it0Pvn6rfNLOU14FQFLw0P2Id0VtmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxwd2ViLXZpZXctdXgtcGFzc2lhcnRpZmFjdHOBoWZDb21taXR4KDliMGNhYWEyNTVkMzIzMTVmMTRiM2M5YTJmZTBkODM4M2EwODY5YWJpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZaNWtLX+g="
}
---

RQ-1: Comment discovery = the index endpoint, not deriving from the fold's `comment/*` subjects (those are only *promoted* comments — one today) nor a manual path box. `GET /comments` (no `file`) returns the files-with-sidecars index — complete (ephemeral + promoted) and the first instance of the resource-collection grammar future views reuse.
***8<***
---
{
  "v": 3,
  "cid": "bafyreieu3apf5n73ndivjwjhibudveyd5mjitk6h6oz2up62qnc33fbmve",
  "sig": "fe99dcd922c7f71aea343399d1095050042201e66a6b4b79ca77e058b27f090f15bb908b5182ffc89f8f40e7920e9313fadfe29c9bd0df61522a34e6b3c99931",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "web-view-ux-pass"
  },
  "kind": "decision",
  "cites": [
    "bafyreif7tw7wmleem4j2fsb7mk3uh347vlptjm4u26avaff4gq7wehorlm"
  ],
  "rev": "223muappp63a3",
  "seq": 4,
  "of": 9,
  "text_len": 216,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgv52/ZiyEZxOiyD9it0Pvn6rfNLOU14FQFLw0P2Id0VtmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxwd2ViLXZpZXctdXgtcGFzc2lhcnRpZmFjdHOBoWZDb21taXR4KDliMGNhYWEyNTVkMzIzMTVmMTRiM2M5YTJmZTBkODM4M2EwODY5YWJpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZaNWtSBEk="
}
---

RQ-2: Comments depth = files → comments → full thread. The tab drills all the way to a single comment's reply thread via `/thread`, mirroring the two existing endpoints, rather than stopping at the per-file list.
***8<***
---
{
  "v": 3,
  "cid": "bafyreid2pqwdziykvedqeg7gsj2zf3drqyfrctc7o56pl45y2p4ths7f2a",
  "sig": "1ef8a4856ed20b42bf4af3276932fb4d5eabcd5d6cc4cd3549d4741d392e5fcb2cc12c695fdfa46c179ea5e3ac74050d41fe0bd36c841a10bcc986d610b91adb",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "web-view-ux-pass"
  },
  "kind": "decision",
  "cites": [
    "bafyreif7tw7wmleem4j2fsb7mk3uh347vlptjm4u26avaff4gq7wehorlm"
  ],
  "rev": "223muapppldt4",
  "seq": 5,
  "of": 9,
  "text_len": 277,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgv52/ZiyEZxOiyD9it0Pvn6rfNLOU14FQFLw0P2Id0VtmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxwd2ViLXZpZXctdXgtcGFzc2lhcnRpZmFjdHOBoWZDb21taXR4KDliMGNhYWEyNTVkMzIzMTVmMTRiM2M5YTJmZTBkODM4M2EwODY5YWJpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZaNWtYpq0="
}
---

RQ-3: Keep the embedded page, do not start the separate Phase-2 PWA now. A single `include_str!` page holds `telos/disposable` (one binary, no build step); the PWA is revisited when the UX genuinely outgrows one file (offline, push, routing, installability), not for this pass.
***8<***
---
{
  "v": 3,
  "cid": "bafyreierrvo6vutfogrxzguys33gcq7hmvf24oyjuunoya35gys6fkduxu",
  "sig": "6f52ed3162792d4a0237115e65004fe3a3eef4db8f40ff2955c3237a78db9bc6767194ca50c904161eee5f7cd1fca21f35ce241f751a8a6e88a8d08d12176720",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "web-view-ux-pass"
  },
  "kind": "decision",
  "cites": [
    "bafyreif7tw7wmleem4j2fsb7mk3uh347vlptjm4u26avaff4gq7wehorlm"
  ],
  "rev": "223mubqgeoxyb",
  "seq": 6,
  "of": 9,
  "text_len": 548,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgv52/ZiyEZxOiyD9it0Pvn6rfNLOU14FQFLw0P2Id0VtmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxwd2ViLXZpZXctdXgtcGFzc2lhcnRpZmFjdHOBoWZDb21taXR4KDliMGNhYWEyNTVkMzIzMTVmMTRiM2M5YTJmZTBkODM4M2EwODY5YWJpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZaPZiqdzQ="
}
---

adversarial review of web-view-ux-pass: APPROVE-WITH-FOLLOW-UPS — Independent cold Opus review: no XSS (all new comment/claim/cite/file-path render paths via textContent; 12 innerHTML sites all constant empty-clear; served page byte-identical to source, hostile <img onerror>/<script> inert), comment_files safe (no walk cycle, symlink evil->/etc skipped, no escape, cheap counts-only), get_comments overload correct, traversal guard intact, disposable+read-only. One REQ-5 gap (fetch-fallback ordering still double-rendered once) fixed in-round.
***8<***
---
{
  "v": 3,
  "cid": "bafyreihlj3mhxvqznkuijoqrr75kjs33cxs2a7p3nw4zqaffkfycptmjja",
  "sig": "49508675f1cad6b8cf44de20cb13cf3e613030a2d301897cc4c436b0013915871ff03fb812b7eefc040766965dc61ab5a7df01307f0fc8963a1036da213756b3",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "web-view-ux-pass"
  },
  "kind": "result",
  "cites": [
    "bafyreierrvo6vutfogrxzguys33gcq7hmvf24oyjuunoya35gys6fkduxu"
  ],
  "rev": "223mubqgmpa37",
  "seq": 7,
  "of": 9,
  "text_len": 599,
  "content": "p2Rib2R5oWZSZXN1bHShZHRleHRgZWNpdGVzgdgqWCUAAXESIJGNXerSZXGjfJqYlvZhQ+dlS647CaUa7AN9NiXiqHS9ZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlU2RCYnJNNk5lWTRpOE1veHVGaXlEdHd4M0FISjhmakVkOTRnckgzMXI4WFllYWdlbnT2Z3N1YmplY3ShZUxvY2FscHdlYi12aWV3LXV4LXBhc3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCg5YjBjYWFhMjU1ZDMyMzE1ZjE0YjNjOWEyZmUwZDgzODNhMDg2OWFiaXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWj2ZKpem"
}
---

Fixed in-round the one review gap: REQ-5 render-once dedupe only fired when /stream won the startup race; in the fetch-fallback ordering (WS silent >800ms) onmessage re-rendered the connect-snapshot, double-rendering once. Now the fetch bootstrap arms a one-shot skipNextFrame (disarmed after 2s) so exactly the post-fetch connect-snapshot is skipped while a genuine later reconnect snapshot still renders to catch up. Also aligned the visibility-hidden handler to detach onerror alongside onclose (symmetry nit). No architecture change; all green (201 lib + 7 integration, clippy -D warnings, fmt).
***8<***
---
{
  "v": 3,
  "cid": "bafyreifu6kpnfu654rzyl64ugj7jcjajxqctc2o7jgrzugq43pyk7wstri",
  "sig": "4aeb1c6a5dc1ec537dda848807e79b1bd69d7d14bff08860bdf53d43a4ee54dc316d90fb14a74a590b16ddc718926ece08c241f1b971887aaf67002366ad7276",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "web-view-ux-pass"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223mubqktpgsu",
  "seq": 8,
  "of": 9,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxwd2ViLXZpZXctdXgtcGFzc2lhcnRpZmFjdHOBoWZDb21taXR4KGViODdjYjZmZWIyMGY0MmJjMDEyMDA1ZjMxZTQ3M2RkM2U0ODk3OWJpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZaPaGasmI="
}
---
