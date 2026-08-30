---
{
  "v": 3,
  "cid": "bafyreihnudgs5w6c3zdkbnuf3u7i7y3nyu2cnfp357o4yuenilctsxyawa",
  "sig": "06f1c48fda541da91132cdcfebfea71b86394577d6f72cf78c6d6024b6bb34ee1cc3ad4566bfdf7fd97c4ea5968210f0e0e90732c98cc9bcb6c16c08163838ba",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "chat-reads-over-api"
  },
  "kind": "observation",
  "cites": [],
  "rev": "223mudjo6jirk",
  "seq": 0,
  "of": 8,
  "text_len": 194,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlU2RCYnJNNk5lWTRpOE1veHVGaXlEdHd4M0FISjhmakVkOTRnckgzMXI4WFllYWdlbnT2Z3N1YmplY3ShZUxvY2Fsc2NoYXQtcmVhZHMtb3Zlci1hcGlpYXJ0aWZhY3RzgaFmQ29tbWl0eChiZjZjYTM3MDg5ZTE4N2E1ZmIzZTMwYjljZDAwODc3Yjg5YTRhYmIwaXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWkvoR7aM"
}
---

design doc .design/chat-reads-over-api.md checked against the live design-doc schema: validation: 10 check(s), 0 failed, 1 warning(s), 0 unchecked, 0 open question(s) [doc 7776:ee8cfe5506943fe9]
***8<***
---
{
  "v": 3,
  "cid": "bafyreifopedhrhmi7w7dlvuszpy472vuo3ccq2d64xhr7mxua6dmrcp2ze",
  "sig": "bf83ac460e7eeecf7b961cc6b0dab646d7c40f51d28a5162272aff6ed40cede43716ac1220e4d1a06dd27bc57c3ddb758b179cde8c43405146cc13a03022c647",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "chat-reads-over-api"
  },
  "kind": "plan",
  "cites": [
    "bafyreihnudgs5w6c3zdkbnuf3u7i7y3nyu2cnfp357o4yuenilctsxyawa"
  ],
  "rev": "223mudjo6yntn",
  "seq": 1,
  "of": 8,
  "text_len": 816,
  "content": "p2Rib2R5oWRQbGFuoWR0ZXh0YGVjaXRlc4HYKlglAAFxEiDtoM0u28LeRqC2hd0+j+NtxTQmlfvv3cxQjULFOV8AsGZhdXRob3KiY2RpZHg5ZGlkOmtleTp6RG5hZVNkQmJyTTZOZVk0aThNb3h1Rml5RHR3eDNBSEo4ZmpFZDk0Z3JIMzFyOFhZZWFnZW509mdzdWJqZWN0oWVMb2NhbHNjaGF0LXJlYWRzLW92ZXItYXBpaWFydGlmYWN0c4GhZkNvbW1pdHgoYmY2Y2EzNzA4OWUxODdhNWZiM2UzMGI5Y2QwMDg3N2I4OWE0YWJiMGl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABlpL6E9OtA=="
}
---

chat-reads-over-api design (.design/chat-reads-over-api.md): Surface the agent chat/transcript sessions over `cospan serve`, read-only: a `GET /chat` index of the repo's sessions and `GET /chat?session=<id>` for one session's turns, feeding a **Chat** tab in the embedded page. It completes the `B → A → C → Chat` roadmap and the mobile-frontend vision's four read tabs (Chat · Comments · Ledger · Process). Serialization is a hand-built projection that **never emits local `$HOME` file paths** (the `Locator`), honoring `telos/disposable` and the operational rule that cospan does not leak the machine's paths; it stays read-only (`telos/observe-now-control-later` — sending messages to agents is the later control tier). [validation: 10 check(s), 0 failed, 1 warning(s), 0 unchecked, 0 open question(s)]
***8<***
---
{
  "v": 3,
  "cid": "bafyreicvtpixnwobgq77t65xngehnqbmuqjjtkhn6p427garbv2wzhay3m",
  "sig": "a163b2ecfaf07d2147f113bbf75e78fb6e0c52c91b0b374ff239ccadc8eccb780e32b5b2fbb7b8aedbb0f05f1772986adf3a6b25da85906dd6de0464cf989714",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "chat-reads-over-api"
  },
  "kind": "subject",
  "cites": [],
  "rev": "223mudjo75msx",
  "seq": 2,
  "of": 8,
  "content": "p2Rib2R5oWdTdWJqZWN0omV0aXRsZXg2RmVhdHVyZTogQ2hhdCByZWFkcyBvdmVyIHRoZSBBUEkgKENoYXQgdGFiIOKAlCBtb2JpbGUpbHN1YmplY3Rfa2luZGRJZGVhZWNpdGVzgGZhdXRob3KiY2RpZHg5ZGlkOmtleTp6RG5hZVNkQmJyTTZOZVk0aThNb3h1Rml5RHR3eDNBSEo4ZmpFZDk0Z3JIMzFyOFhZZWFnZW509mdzdWJqZWN0oWVMb2NhbHNjaGF0LXJlYWRzLW92ZXItYXBpaWFydGlmYWN0c4GhZkNvbW1pdHgoYmY2Y2EzNzA4OWUxODdhNWZiM2UzMGI5Y2QwMDg3N2I4OWE0YWJiMGl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABlpL6FHKqg=="
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreigdzbprfws75khmsa4fwnxnbro4bwpiefp5wytbiwpondu4qm4wxa",
  "sig": "87bb10d0b38c80c32326ff12f6d094eb663563be2c18011aa31b2058fc434ff9224f81ed122b2c891eeb695925ea06b4721f8c9fc5cfe9a2beabd9b5801b2e79",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "chat-reads-over-api"
  },
  "kind": "decision",
  "cites": [
    "bafyreifopedhrhmi7w7dlvuszpy472vuo3ccq2d64xhr7mxua6dmrcp2ze"
  ],
  "rev": "223mudjo7mrc3",
  "seq": 3,
  "of": 8,
  "text_len": 233,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgrnkGeJ2I/b411pLL8c/qtHbEKGh+5c8fsvQHhsiJ+slmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxzY2hhdC1yZWFkcy1vdmVyLWFwaWlhcnRpZmFjdHOBoWZDb21taXR4KGJmNmNhMzcwODllMTg3YTVmYjNlMzBiOWNkMDA4NzdiODlhNGFiYjBpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZaS+hZXHw="
}
---

RQ-1: All events, kind-tagged. `chat_session` returns every event with its `kind`, and the client collapses `thinking`/`toolcall`/`toolresult` — honest and matching the TUI — rather than the server dropping them to messages-only.
***8<***
---
{
  "v": 3,
  "cid": "bafyreihcud4jiqv546hi6gbvquzjghcwrxzlj7377ysy3hgbtt6vjfm4au",
  "sig": "078021f6b593b4ba67ba37416ad340f37ec748226c4ed5cef6ab7cfdc7fb655909b54bc66dbc3e352d66f96bb6c1ad8aedbb34ec771874d3171976478cb10b62",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "chat-reads-over-api"
  },
  "kind": "decision",
  "cites": [
    "bafyreifopedhrhmi7w7dlvuszpy472vuo3ccq2d64xhr7mxua6dmrcp2ze"
  ],
  "rev": "223mudjoa3w7f",
  "seq": 4,
  "of": 8,
  "text_len": 314,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgrnkGeJ2I/b411pLL8c/qtHbEKGh+5c8fsvQHhsiJ+slmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxzY2hhdC1yZWFkcy1vdmVyLWFwaWlhcnRpZmFjdHOBoWZDb21taXR4KGJmNmNhMzcwODllMTg3YTVmYjNlMzBiOWNkMDA4NzdiODlhNGFiYjBpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZaS+hg8DU="
}
---

RQ-2: Hand-built projection, paths omitted. A `chat_index`/`chat_session` projection (not blanket `#[derive(Serialize)]` on the transcript types) emits an explicit safe field set and never the `Locator`, so a local `$HOME` path cannot leak — worth the small extra code over coupling the wire to internal structs.
***8<***
---
{
  "v": 3,
  "cid": "bafyreigpbhvvoewbtz55cl5dqii4cjn67swnbtl6xms6jnbxhxi4uiss54",
  "sig": "62afc11fa181ababe264dfb2bec6665e9199e0e3040075c00ca8c8cc98754a9648695a6387ea763b01938583f882a80bfe4fda209d2e81f10eea8e1521ba0ab6",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "chat-reads-over-api"
  },
  "kind": "decision",
  "cites": [
    "bafyreifopedhrhmi7w7dlvuszpy472vuo3ccq2d64xhr7mxua6dmrcp2ze"
  ],
  "rev": "223mudjoal24z",
  "seq": 5,
  "of": 8,
  "text_len": 147,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgrnkGeJ2I/b411pLL8c/qtHbEKGh+5c8fsvQHhsiJ+slmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxzY2hhdC1yZWFkcy1vdmVyLWFwaWlhcnRpZmFjdHOBoWZDb21taXR4KGJmNmNhMzcwODllMTg3YTVmYjNlMzBiOWNkMDA4NzdiODlhNGFiYjBpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZaS+hof+Y="
}
---

RQ-3: `GET /chat` resource-collection, read-only. The collection Slice A named; sending a message to an agent is the control tier and out of scope.
***8<***
---
{
  "v": 3,
  "cid": "bafyreifmdbju5utvxm5sywtjpeqth5ism3ezjyygb2wkulpv5dsjgkf3jm",
  "sig": "09ad2c485c11c95f48cd1819fdec73c83ed40f4eefa759646266625be6570ebc2de37db96e7ca0bdbdd5234b4a7f0c5354020777eb59439b54ead2e589cecb6a",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "chat-reads-over-api"
  },
  "kind": "decision",
  "cites": [
    "bafyreifopedhrhmi7w7dlvuszpy472vuo3ccq2d64xhr7mxua6dmrcp2ze"
  ],
  "rev": "223mudkaulcpx",
  "seq": 6,
  "of": 8,
  "text_len": 664,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgrnkGeJ2I/b411pLL8c/qtHbEKGh+5c8fsvQHhsiJ+slmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxzY2hhdC1yZWFkcy1vdmVyLWFwaWlhcnRpZmFjdHOBoWZDb21taXR4KGJmNmNhMzcwODllMTg3YTVmYjNlMzBiOWNkMDA4NzdiODlhNGFiYjBpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZaTA2onro="
}
---

adversarial review of chat-reads-over-api: APPROVE — Independent cold Opus review: clean APPROVE. Load-bearing path/privacy check holds — Locator never referenced in the three projections; live against a real 7-session repo the index + a 1209-event session carry only the safe allow-list, no locator key, no path in any non-text field (a /Users path inside event text is content, not a leak). Read-only (single GET route, POST->405, command_bus untouched, nothing persisted), auth-gated (401 no token), panic-free on unknown id / huge session / undecoded opencode body, XSS-safe (uniform textContent). 207 lib + 11 integration + clippy -D warnings + fmt green.
***8<***
---
{
  "v": 3,
  "cid": "bafyreigm5ol46qybhul7cseeku3dknffihjiuogpqmagjz3pg5kc35bppe",
  "sig": "49e7ddfdcfbb7cd20add68504f7912c793bfc54ffcaf46176d348619b1467af03142ca803c567971dda7c3733c691465d465e7abd3e813f26b72e1bd067c516e",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "chat-reads-over-api"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223mudkem6zna",
  "seq": 7,
  "of": 8,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxzY2hhdC1yZWFkcy1vdmVyLWFwaWlhcnRpZmFjdHOBoWZDb21taXR4KGIxZmY4MDc5ZmMwMzk3NDlkZDZlMDFmN2YxMmUyMjZmMmMyMzhkZjlpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZaTBUiffk="
}
---
