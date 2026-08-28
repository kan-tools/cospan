---
{
  "v": 3,
  "cid": "bafyreiehfs7jms5ury4ivbdypwxrjsc2gwkg6ukvv2g64r4sfuhspdxvfe",
  "sig": "65b8c3ed3ba7804c57d151d51936038634128c113ebc0185f157bbd391471ab333966fc4e64f9b0d808872ae04fe6d1ad3fa4c30bedb12e57b7ba4aaa723a64e",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "mobile-web-view"
  },
  "kind": "decision",
  "cites": [
    "bafyreiawteatremkiotyssugunupsk7lw5esffddxsfmfobpzo75nvs2zy"
  ],
  "rev": "223mu5narxik7",
  "seq": 0,
  "of": 6,
  "text_len": 959,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgFpkBOJGKQ6eJSoajaPkr67dJIpRjvIrCuC/Lv9bWWs5mYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxvbW9iaWxlLXdlYi12aWV3aWFydGlmYWN0c4GhZkNvbW1pdHgoNmRkYjI5YjhjNmQ0MmM4ZDBjN2E4N2M4ZWZjMDk3NTA2Mzg1ZjA4MWl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABloczX65mA=="
}
---

Interim human view for the mobile read-API, served at GET / by cospan serve. Decision: ship a single self-contained HTML page embedded in the binary via include_str! (no separate codebase, no build step) rather than start the full Phase-2 web/PWA client now — this keeps telos/disposable intact (one throwaway binary) while giving the read API a human UX today over the existing loopback/Tailscale transport. Scope: a tabbed mobile-first view (Now = day process position/atoms; Teloi = teloi + held tensions; Browse = namespace-grouped subject->claims accordion with filter), a pure client of GET /fold + WS /stream, live-updating on refold. Rendering is textContent/createElement only (no innerHTML with log data) so kan claim/subject/tension text cannot inject markup. Explicitly interim: superseded by the Phase-2 web client (mobile-frontend .dropbox/08) and a fuller UX pass; the /comments + /thread endpoints exist but are not yet surfaced in this cut.
***8<***
---
{
  "v": 3,
  "cid": "bafyreifrxjv2vsdmgj7l5aaeharzdqkobf6ezbv3ruhlbpumr3blmsef7i",
  "sig": "d63bb2295df2f756d94911f2fe3c6acf9a69ac9e4cd877a02b132fecf44ab4b52d0b91ee3e72c19fb429fc885781ed65b0d607eb19e9f7d57c92dd543fe25189",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "mobile-web-view"
  },
  "kind": "subject",
  "cites": [],
  "rev": "223mu5nas3khh",
  "seq": 1,
  "of": 6,
  "content": "p2Rib2R5oWdTdWJqZWN0omV0aXRsZXhDSW50ZXJpbSBlbWJlZGRlZCBodW1hbiB2aWV3IChHRVQgLykgYWhlYWQgb2YgdGhlIFBoYXNlLTIgd2ViIGNsaWVudGxzdWJqZWN0X2tpbmRkSWRlYWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxvbW9iaWxlLXdlYi12aWV3aWFydGlmYWN0c4GhZkNvbW1pdHgoNmRkYjI5YjhjNmQ0MmM4ZDBjN2E4N2M4ZWZjMDk3NTA2Mzg1ZjA4MWl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABloczYDBRA=="
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreidmebbbexsbxett33tfqctnlsqq37pfc5dgredwruxdvjvbwf4ml4",
  "sig": "266c130fc6677e6da41a3fc027618f3ff436fb93c65df0794d63a775819084463bca94b740b270e4d5468e8bb0bd315f56d1cf57d9fe04722da90a0db1a939da",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "mobile-web-view"
  },
  "kind": "decision",
  "cites": [
    "bafyreiehfs7jms5ury4ivbdypwxrjsc2gwkg6ukvv2g64r4sfuhspdxvfe"
  ],
  "rev": "223mu5nhrxfgf",
  "seq": 2,
  "of": 6,
  "text_len": 496,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIghyy+lku0jjiKhHh9rxTIWjWUb1FVro3uR5ItDyeO9SlmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxvbW9iaWxlLXdlYi12aWV3aWFydGlmYWN0c4GhZkNvbW1pdHgoNmRkYjI5YjhjNmQ0MmM4ZDBjN2E4N2M4ZWZjMDk3NTA2Mzg1ZjA4MWl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABloc236tFA=="
}
---

adversarial review of mobile-web-view: APPROVE-WITH-FOLLOW-UPS — Independent cold Opus review of the embedded GET / web UI: XSS airtight (all ~15 fold-data render paths via textContent/createTextNode, only innerHTML='' clears, verified inert against live <img onerror>/<script> payloads), disposable (one include_str! page, no external requests), read-only (static const handler, loopback + WriteChannel untouched); build/tests/clippy/fmt green. Two cosmetic follow-ups deferred to the UX pass.
***8<***
---
{
  "v": 3,
  "cid": "bafyreig67dnflh3es5jlj3x27akqjwai44f4xidtv7o5bksffiprjqjkhq",
  "sig": "7acb69fb08b871351eb4d022458c12d85970721a72843b08b0c4586a0f41fed738f734bed95a8fe2b0f2e035f4286e74bc118119bfba97611687b01f69ca1c4b",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "mobile-web-view"
  },
  "kind": "observation",
  "cites": [
    "bafyreidmebbbexsbxett33tfqctnlsqq37pfc5dgredwruxdvjvbwf4ml4"
  ],
  "rev": "223mu5nhyno7x",
  "seq": 3,
  "of": 6,
  "text_len": 234,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgbCBCEl5BuSc97mWAptXKEN/eUXRmiQdo0uOqahsXjF9mYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxvbW9iaWxlLXdlYi12aWV3aWFydGlmYWN0c4GhZkNvbW1pdHgoNmRkYjI5YjhjNmQ0MmM4ZDBjN2E4N2M4ZWZjMDk3NTA2Mzg1ZjA4MWl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABloc2+nQTw=="
}
---

Follow-up (UX pass): the /stream reconnect loop retries every 1.5s with no backoff cap and no hidden-tab/online gating — a mild mobile-battery cost while the server is down. Add capped exponential backoff + pause on document.hidden.
***8<***
---
{
  "v": 3,
  "cid": "bafyreiebbu6r4pkakvk7fqhdxowyty44rf2dv2yenh7kiglbunxp5izh5q",
  "sig": "47f4219e8edc832281fe0cc1d3929c54bb4c5338aad9b3b84054949d1939f66b5e49dea65152022bd498d8032a3d3a84419e71ed65657ec543ce8c513726e149",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "mobile-web-view"
  },
  "kind": "observation",
  "cites": [
    "bafyreidmebbbexsbxett33tfqctnlsqq37pfc5dgredwruxdvjvbwf4ml4"
  ],
  "rev": "223mu5nhz24dp",
  "seq": 4,
  "of": 6,
  "text_len": 211,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgbCBCEl5BuSc97mWAptXKEN/eUXRmiQdo0uOqahsXjF9mYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxvbW9iaWxlLXdlYi12aWV3aWFydGlmYWN0c4GhZkNvbW1pdHgoNmRkYjI5YjhjNmQ0MmM4ZDBjN2E4N2M4ZWZjMDk3NTA2Mzg1ZjA4MWl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABloc2/AIxQ=="
}
---

Follow-up (UX pass): on load the page fetches /fold and then the WS delivers the same snapshot on connect, causing one redundant initial render (harmless). Could gate the first render on whichever arrives first.
***8<***
---
{
  "v": 3,
  "cid": "bafyreifa7wpi4nns7b7oivvqkbf6x523k2n3wsgd5bc65hq4ylbbd57chu",
  "sig": "63aff3c8d0e0fc03c1487fa3336a8c011aae74189e35c86fd16b5f5d53e8bf0d75d9c4f8b9da89520f7d33d4f6faf275d6cd25905619be9bf021bd53139987ca",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "mobile-web-view"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223mu6iafc4cj",
  "seq": 5,
  "of": 6,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxvbW9iaWxlLXdlYi12aWV3aWFydGlmYWN0c4GhZkNvbW1pdHgoMjQ5NGM4ZDRmYjY5OTU3NDcwYTNmNmQ2MTdlNTNhNzRiNDRiY2I3MWl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABlojjLQInA=="
}
---
