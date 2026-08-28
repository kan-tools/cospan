---
{
  "v": 3,
  "cid": "bafyreidajrg37gunedbhaetwzu7lqmbj6vhqbguvg6pzgctep2gms4muoi",
  "sig": "1d203995ed3f92d7169190673a3b3f54bf2f825ed03271f8e92faded28337fbf656513ce8177a5deee99ca25f1608745a742bde935103883b3aa359e23018994",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "mobile-api-server"
  },
  "kind": "observation",
  "cites": [],
  "rev": "223mtymz3nczk",
  "seq": 0,
  "of": 11,
  "text_len": 191,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlU2RCYnJNNk5lWTRpOE1veHVGaXlEdHd4M0FISjhmakVkOTRnckgzMXI4WFllYWdlbnT2Z3N1YmplY3ShZUxvY2FscW1vYmlsZS1hcGktc2VydmVyaWFydGlmYWN0c4GhZkNvbW1pdHgoMDIyYTM4ODRjNzA1YzNhZTNlY2RlMjE2MDY4OGM3YjJkZDgwNzYzNml3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABln0vhmgNg=="
}
---

design doc .design/mobile-api-server.md checked against the live design-doc schema: validation: 9 check(s), 0 failed, 0 warning(s), 0 unchecked, 0 open question(s) [doc 8620:4fa9e27474192d47]
***8<***
---
{
  "v": 3,
  "cid": "bafyreiawteatremkiotyssugunupsk7lw5esffddxsfmfobpzo75nvs2zy",
  "sig": "34292d259a8e3731d567af35f506861cdefb0952743efa55441f7ef3024f5ca1246509c2810371de61f9795f978a188ac49f280f6b9efc69b68fbe1b2dd7f985",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "mobile-api-server"
  },
  "kind": "plan",
  "cites": [
    "bafyreidajrg37gunedbhaetwzu7lqmbj6vhqbguvg6pzgctep2gms4muoi"
  ],
  "rev": "223mtymz3zvce",
  "seq": 1,
  "of": 11,
  "text_len": 810,
  "content": "p2Rib2R5oWRQbGFuoWR0ZXh0YGVjaXRlc4HYKlglAAFxEiBgTE2/mo0gwnASds0+uDAp9U8AmpU3n5MKZH6MyXGUcmZhdXRob3KiY2RpZHg5ZGlkOmtleTp6RG5hZVNkQmJyTTZOZVk0aThNb3h1Rml5RHR3eDNBSEo4ZmpFZDk0Z3JIMzFyOFhZZWFnZW509mdzdWJqZWN0oWVMb2NhbHFtb2JpbGUtYXBpLXNlcnZlcmlhcnRpZmFjdHOBoWZDb21taXR4KDAyMmEzODg0YzcwNWMzYWUzZWNkZTIxNjA2ODhjN2IyZGQ4MDc2MzZpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZZ9L4f7JQ="
}
---

mobile-api-server design (.design/mobile-api-server.md): Add a `cospan serve` subcommand that runs the existing watch-and-fold loop headless and exposes it over a localhost HTTP/WebSocket API: `GET /fold` returns the serialized `substrate::Fold` (the kan/day projection behind the Ledger and Process tabs), `WS /stream` pushes it on every refold, and `GET /comments` / `GET /thread` reuse the S5 MCP read core. This is Phase 1 of the mobile-frontend vision (`.dropbox/08-mobile-frontend.md`) — the transport down payment. The web client and secure remote transport are later, separate phases. Serves `telos/observe-now-control-later` (read-only first; the write/control channel is `command_bus::WriteChannel`, untouched here). [validation: 9 check(s), 0 failed, 0 warning(s), 0 unchecked, 0 open question(s)]
***8<***
---
{
  "v": 3,
  "cid": "bafyreihzpmvh5unsl5v2h3mkqwfpj2s5gwuxnnpaiife22dkwreuzztngu",
  "sig": "f19e7ed7536c0b50edf66146794d235144c3c31277f307349dceb3ec91fd48cc6a5456aabf5b2010d970a7df5fc3ad63a23695b1a7ce5e2f69b87a86347e570b",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "mobile-api-server"
  },
  "kind": "subject",
  "cites": [],
  "rev": "223mtymz45tno",
  "seq": 2,
  "of": 11,
  "content": "p2Rib2R5oWdTdWJqZWN0omV0aXRsZXhERmVhdHVyZTogUmVhZC1vbmx5IEhUVFAvV1MgQVBJIHNlcnZlciBvdmVyIHRoZSBmb2xkIChtb2JpbGUgUGhhc2UgMSlsc3ViamVjdF9raW5kZElkZWFlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlU2RCYnJNNk5lWTRpOE1veHVGaXlEdHd4M0FISjhmakVkOTRnckgzMXI4WFllYWdlbnT2Z3N1YmplY3ShZUxvY2FscW1vYmlsZS1hcGktc2VydmVyaWFydGlmYWN0c4GhZkNvbW1pdHgoMDIyYTM4ODRjNzA1YzNhZTNlY2RlMjE2MDY4OGM3YjJkZDgwNzYzNml3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABln0viHmDQ=="
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreidehrzymar2mgjmhux4rpigbugsbh2wyxxu73wskxcu25cgqzgptm",
  "sig": "1f59d92352de877315c1a09a80eb932650f830f764590f500772ba9d1325f01730172cbad4b285ef8c37eb3f4bc8098062df39c130acf5607136f672efa6945f",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "mobile-api-server"
  },
  "kind": "decision",
  "cites": [
    "bafyreiawteatremkiotyssugunupsk7lw5esffddxsfmfobpzo75nvs2zy"
  ],
  "rev": "223mtymz4jxar",
  "seq": 3,
  "of": 11,
  "text_len": 351,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgFpkBOJGKQ6eJSoajaPkr67dJIpRjvIrCuC/Lv9bWWs5mYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxxbW9iaWxlLWFwaS1zZXJ2ZXJpYXJ0aWZhY3RzgaFmQ29tbWl0eCgwMjJhMzg4NGM3MDVjM2FlM2VjZGUyMTYwNjg4YzdiMmRkODA3NjM2aXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWfS+J/Rg"
}
---

RQ-1: Phase-1 scope is Fold + Comments — `GET /fold` + `WS /stream` (the kan/day projection behind Ledger and Process) plus the comment reads (`GET /comments`, `GET /thread`) reusing the S5 MCP core. Chat (needs new `Serialize` derives on the `transcripts` types), the web client, and the secure remote transport are later phases, out of this slice.
***8<***
---
{
  "v": 3,
  "cid": "bafyreicef5kxcllfszogadrjxdeysyjupznkvfrg7bqtjj3xluxfn5gkui",
  "sig": "cd250b696cd134b24964b518324ce5d2f24477c1fdc2ef66b392dd8619b3eacf659ee3bb055ab0c836df85c417a19603032f4b6bfc5a5126dd6925c45cfc796c",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "mobile-api-server"
  },
  "kind": "decision",
  "cites": [
    "bafyreiawteatremkiotyssugunupsk7lw5esffddxsfmfobpzo75nvs2zy"
  ],
  "rev": "223mtymz4w2x7",
  "seq": 4,
  "of": 11,
  "text_len": 183,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgFpkBOJGKQ6eJSoajaPkr67dJIpRjvIrCuC/Lv9bWWs5mYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxxbW9iaWxlLWFwaS1zZXJ2ZXJpYXJ0aWZhY3RzgaFmQ29tbWl0eCgwMjJhMzg4NGM3MDVjM2FlM2VjZGUyMTYwNjg4YzdiMmRkODA3NjM2aXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWfS+LgMr"
}
---

RQ-2: The `Fold` is serialized by deriving `Serialize` on `Fold`/`Claim`/the process types (a single source of truth for the shape), not a hand-written `serde_json::Value` projection.
***8<***
---
{
  "v": 3,
  "cid": "bafyreievlpzdw7i7zzwrbc562v6almim66nhtpz6bxnyqnq5g2suv2muba",
  "sig": "e6cad6cbcefc696f051656cb1e04f7ee2ad5114d66fdc6f3b3b23060704ddd951887c601bf80e3659b22b7d2b0d418fbffca5df41bdcbc6a1f131640eb52adc9",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "mobile-api-server"
  },
  "kind": "decision",
  "cites": [
    "bafyreiawteatremkiotyssugunupsk7lw5esffddxsfmfobpzo75nvs2zy"
  ],
  "rev": "223mtymz5c5de",
  "seq": 5,
  "of": 11,
  "text_len": 196,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgFpkBOJGKQ6eJSoajaPkr67dJIpRjvIrCuC/Lv9bWWs5mYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxxbW9iaWxlLWFwaS1zZXJ2ZXJpYXJ0aWZhY3RzgaFmQ29tbWl0eCgwMjJhMzg4NGM3MDVjM2FlM2VjZGUyMTYwNjg4YzdiMmRkODA3NjM2aXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWfS+NAy1"
}
---

RQ-3: `serve` stays disposable: a foreground process bound to 127.0.0.1 with no daemon, no auto-start, and no on-disk state — the `telos/disposable` tension is named and held, not resolved away.
***8<***
---
{
  "v": 3,
  "cid": "bafyreig4hr27jool7tfw2gl73eylnqaot5ky7q7lfbenvikfgmbu7pqbsm",
  "sig": "ac7c4f4ab478c159909d818a1a1ad212d25c1544744a6f56ed18c1a97e7a93d034e9756f1b4efac0ab01886188f7e187c8f1b76972265b3dc5a971f87f85da8e",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "mobile-api-server"
  },
  "kind": "decision",
  "cites": [
    "bafyreiawteatremkiotyssugunupsk7lw5esffddxsfmfobpzo75nvs2zy"
  ],
  "rev": "223mu2drjznke",
  "seq": 6,
  "of": 11,
  "text_len": 362,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgFpkBOJGKQ6eJSoajaPkr67dJIpRjvIrCuC/Lv9bWWs5mYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxxbW9iaWxlLWFwaS1zZXJ2ZXJpYXJ0aWZhY3RzgaFmQ29tbWl0eCg4YTI1ODM4N2ZiMDBmODBjYTE2YzdiZWUzZmFhOWNkMjYyNmZlOTk1aXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWgJu/82a"
}
---

adversarial review of mobile-api-server: APPROVE-WITH-FOLLOW-UPS — Independent cold Opus review: all 8 REQs + 6/6 ACs met and verified (195 tests, both real-kan smokes, clippy/fmt clean, hostile-input probing, lsof-confirmed loopback-only, no disk/kan writes, command_bus untouched, concurrency core sound); three narrow follow-ups do not touch the north star.
***8<***
---
{
  "v": 3,
  "cid": "bafyreibzcrbkeqph72msl2afrl3rzhhffgq6ubcwrprq5kfhfijtlezhpi",
  "sig": "f1b99ac229dada88e3ae9ea026024bd5eb68d717dac95a5ffcaec0c97b33f4df71c6a75fcb72b1fa10931c3e35e9d90f68355820952df398e6c41a7353cf5f39",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "mobile-api-server"
  },
  "kind": "observation",
  "cites": [
    "bafyreig4hr27jool7tfw2gl73eylnqaot5ky7q7lfbenvikfgmbu7pqbsm"
  ],
  "rev": "223mu2drthj3x",
  "seq": 7,
  "of": 11,
  "text_len": 296,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIg3Dx19LnL/MttGX/ZMLbADp9Vj8PrKEjaoUUzA0++AZNmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxxbW9iaWxlLWFwaS1zZXJ2ZXJpYXJ0aWZhY3RzgaFmQ29tbWl0eCg4YTI1ODM4N2ZiMDBmODBjYTE2YzdiZWUzZmFhOWNkMjYyNmZlOTk1aXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWgJvlrvH"
}
---

Follow-up (later, before any remote-exposure phase): the shared S5 path guard (mcp.rs guard) blocks only .. and absolute paths, not a repo-internal symlink pointing outside the repo — now also reachable over HTTP via /comments,/thread. Loopback-only + single local user contains it for Phase 1.
***8<***
---
{
  "v": 3,
  "cid": "bafyreifjrmhagqlnuyjpptnliralop55q6xoaby7q53x4q4dlfibpbrkvm",
  "sig": "7f5660ab5b18c3fea971c96f75017fc8ebead9dd3de448b153cfda839a6b48b05c8271e7c1c0195dea0553813aa67608250d470d1e2c9f96072d720fbc1cd224",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "mobile-api-server"
  },
  "kind": "observation",
  "cites": [
    "bafyreig4hr27jool7tfw2gl73eylnqaot5ky7q7lfbenvikfgmbu7pqbsm"
  ],
  "rev": "223mu2drtu5d5",
  "seq": 8,
  "of": 11,
  "text_len": 198,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIg3Dx19LnL/MttGX/ZMLbADp9Vj8PrKEjaoUUzA0++AZNmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxxbW9iaWxlLWFwaS1zZXJ2ZXJpYXJ0aWZhY3RzgaFmQ29tbWl0eCg4YTI1ODM4N2ZiMDBmODBjYTE2YzdiZWUzZmFhOWNkMjYyNmZlOTk1aXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWgJvnQyw"
}
---

Follow-up (later, before any remote-exposure phase): /stream connections are uncapped — each WS upgrade spawns an unbounded task. Loopback-only mitigates; add a bound before non-loopback exposure.
***8<***
---
{
  "v": 3,
  "cid": "bafyreidaxytbxe2xvbsyelt5idb5fd24vwypnysyxsv7bduzwyacf7wt3u",
  "sig": "969ee1aad5a3da761a5fc24c3c87be2f1c0dfedc6fef7957afc3f3bc1491853c29b902d5b5258c8153622911c72ba461fe2d009e9737dda4ad01c3349c729e1c",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "mobile-api-server"
  },
  "kind": "observation",
  "cites": [
    "bafyreig4hr27jool7tfw2gl73eylnqaot5ky7q7lfbenvikfgmbu7pqbsm"
  ],
  "rev": "223mu2druackq",
  "seq": 9,
  "of": 11,
  "text_len": 316,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIg3Dx19LnL/MttGX/ZMLbADp9Vj8PrKEjaoUUzA0++AZNmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxxbW9iaWxlLWFwaS1zZXJ2ZXJpYXJ0aWZhY3RzgaFmQ29tbWl0eCg4YTI1ODM4N2ZiMDBmODBjYTE2YzdiZWUzZmFhOWNkMjYyNmZlOTk1aXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWgJvoyGj"
}
---

AC-6 gap closed after the review: added an isolated #[ignore]d fold_tick unit test (server.rs) asserting the changed-mtime rebuild+swap of the shared latest-Fold and the broadcast — the literal unit test the AC promised, previously covered only via the WS integration smoke. Additive test only, no behavior change.
***8<***
---
{
  "v": 3,
  "cid": "bafyreiengx6xcyrehisrqh3lpdi2koniedqwkzuhypxps7v6hbyigpe6zy",
  "sig": "19996093011c3b434c4bb8f107fdb2330b6b44b1a8058831a52c74061b4c935b5844c917c9d2b81ac6a1d727287a2ac98594e913a4e17d0d0b30c6d4efe1681e",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "mobile-api-server"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223mu4prkgnbm",
  "seq": 10,
  "of": 11,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxxbW9iaWxlLWFwaS1zZXJ2ZXJpYXJ0aWZhY3RzgaFmQ29tbWl0eChmODQ1NmFiOWYyZTJlM2U0ZTUxN2JiN2YyMzk1NDQwZDg5YTllYzYwaXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWhVvBkyG"
}
---
