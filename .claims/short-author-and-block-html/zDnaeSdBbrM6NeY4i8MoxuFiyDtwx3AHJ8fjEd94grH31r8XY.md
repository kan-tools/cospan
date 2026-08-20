---
{
  "v": 3,
  "cid": "bafyreiewq22dhndcj3tik7ifqlfgqwrwhy6ezf44wbeqwo4vbmiyycrkce",
  "sig": "f069da46b8dd7eff7089bd46ebe2bd0df51a69c343a8dc36484608dff1052f9216abbb5c19067d84fc2d34445980b5587c775cb23bee98b1734a4c660e57dbec",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "short-author-and-block-html"
  },
  "kind": "observation",
  "cites": [],
  "rev": "223mtih4ieplq",
  "seq": 0,
  "of": 7,
  "text_len": 201,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlU2RCYnJNNk5lWTRpOE1veHVGaXlEdHd4M0FISjhmakVkOTRnckgzMXI4WFllYWdlbnT2Z3N1YmplY3ShZUxvY2FseBtzaG9ydC1hdXRob3ItYW5kLWJsb2NrLWh0bWxpYXJ0aWZhY3RzgaFmQ29tbWl0eChlNmZkZjBiZjI1MWM1NmE1YjcyMWUwYWJmMTQ2NGFjYmUwNzJlMmY3aXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWXNE5VXM"
}
---

design doc .design/short-author-and-block-html.md checked against the live design-doc schema: validation: 9 check(s), 0 failed, 0 warning(s), 0 unchecked, 0 open question(s) [doc 3394:5d4e4dbfd740b667]
***8<***
---
{
  "v": 3,
  "cid": "bafyreibbwsaxaokorsgwbxffgzqijnzqctiwh7kmk5zkwd3khmhdrzaf4q",
  "sig": "80cbf3b19237aae2efab5f4ec3b9ee8ffe80430c6dbc75051a024eff06e76fe4711a3164e584f464514acdff07f320f67cd1aebcef93a5f576328df47b05909e",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "short-author-and-block-html"
  },
  "kind": "plan",
  "cites": [
    "bafyreiewq22dhndcj3tik7ifqlfgqwrwhy6ezf44wbeqwo4vbmiyycrkce"
  ],
  "rev": "223mtih4imzmq",
  "seq": 1,
  "of": 7,
  "text_len": 425,
  "content": "p2Rib2R5oWRQbGFuoWR0ZXh0YGVjaXRlc4HYKlglAAFxEiCWhrQztGJO5oV9BYLKaFo2PjxMl5ywSQs7lQsRjAoqEWZhdXRob3KiY2RpZHg5ZGlkOmtleTp6RG5hZVNkQmJyTTZOZVk0aThNb3h1Rml5RHR3eDNBSEo4ZmpFZDk0Z3JIMzFyOFhZZWFnZW509mdzdWJqZWN0oWVMb2NhbHgbc2hvcnQtYXV0aG9yLWFuZC1ibG9jay1odG1saWFydGlmYWN0c4GhZkNvbW1pdHgoZTZmZGYwYmYyNTFjNTZhNWI3MjFlMGFiZjE0NjRhY2JlMDcyZTJmN2l3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABllzROl94A=="
}
---

short-author-and-block-html design (.design/short-author-and-block-html.md): Two low-severity display cleanups deferred from earlier reviews: `short_author` front-truncation collides once a log has multiple signers, and block-level HTML in a markdown claim body collapses its lines onto one row. Both are contained fixes with regression tests. [validation: 9 check(s), 0 failed, 0 warning(s), 0 unchecked, 0 open question(s)]
***8<***
---
{
  "v": 3,
  "cid": "bafyreiaoiwxn5ff67zyowl5im3id47occbc5v3x4a2bp6rhlwxtkr5clqe",
  "sig": "ad8dc25058d8e682db009c3ab7b176ebea4abfa886c540c452c724109c3d913f351902915aea6920b4e6ced745e3017fb3100a0bd44f655ea886eeb9e60d9b97",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "short-author-and-block-html"
  },
  "kind": "subject",
  "cites": [],
  "rev": "223mtih4ipv5r",
  "seq": 2,
  "of": 7,
  "content": "p2Rib2R5oWdTdWJqZWN0omV0aXRsZXg6RmVhdHVyZTogc2hvcnQtYXV0aG9yIGNvbGxpc2lvbiArIGJsb2NrLUhUTUwgbmV3bGluZSBmaXhlc2xzdWJqZWN0X2tpbmRkSWRlYWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx4G3Nob3J0LWF1dGhvci1hbmQtYmxvY2staHRtbGlhcnRpZmFjdHOBoWZDb21taXR4KGU2ZmRmMGJmMjUxYzU2YTViNzIxZTBhYmYxNDY0YWNiZTA3MmUyZjdpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZZc0Tq62g="
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreiartr6k2qsekb5dsp6rx5e54o544k66pztv6uiswzaktm7xqwipc4",
  "sig": "f3102ff1b44ff8927d6b7af9bd403695f4d603d0c847fd0d4bd5063a1f2ebb9a514c579e1e86af6f3aeea9781bf38c9b80ce138ae97beb057dfa72c227d46df7",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "short-author-and-block-html"
  },
  "kind": "decision",
  "cites": [
    "bafyreibbwsaxaokorsgwbxffgzqijnzqctiwh7kmk5zkwd3khmhdrzaf4q"
  ],
  "rev": "223mtih4iykqg",
  "seq": 3,
  "of": 7,
  "text_len": 226,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgIbSBcDlOjI1g3KU2YIS3MBTRY/1MV3KrD2o7DjjkBeRmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx4G3Nob3J0LWF1dGhvci1hbmQtYmxvY2staHRtbGlhcnRpZmFjdHOBoWZDb21taXR4KGU2ZmRmMGJmMjUxYzU2YTViNzIxZTBhYmYxNDY0YWNiZTA3MmUyZjdpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZZc0TvQmI="
}
---

RQ-1: `short_author` uses head+tail rather than more leading chars or a hash, matching the familiar address-abbreviation idiom while keeping the readable `did:key` prefix; exact widths (6/4) are a display choice, easily tuned.
***8<***
---
{
  "v": 3,
  "cid": "bafyreic4pvgpv73w73o2m2ymdav34i7yt6pfcv7mrldni3svqwxpqpetq4",
  "sig": "7a3a179fee29959db19638e725ad1cf7bd14e5e7fb1ba4e1993ead64389fb4eb653aca56a8d047320b7162188297dbe534b655fc1d05fc110befd1c60cbd1afb",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "short-author-and-block-html"
  },
  "kind": "decision",
  "cites": [
    "bafyreibbwsaxaokorsgwbxffgzqijnzqctiwh7kmk5zkwd3khmhdrzaf4q"
  ],
  "rev": "223mtih4jbvog",
  "seq": 4,
  "of": 7,
  "text_len": 122,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgIbSBcDlOjI1g3KU2YIS3MBTRY/1MV3KrD2o7DjjkBeRmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx4G3Nob3J0LWF1dGhvci1hbmQtYmxvY2staHRtbGlhcnRpZmFjdHOBoWZDb21taXR4KGU2ZmRmMGJmMjUxYzU2YTViNzIxZTBhYmYxNDY0YWNiZTA3MmUyZjdpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZZc0Tz7hc="
}
---

RQ-2: The renderer is corrected rather than pre-joining HTML events, so a genuine single-line HTML fragment is unaffected.
***8<***
---
{
  "v": 3,
  "cid": "bafyreiezv2div5x7jxmqqisb6go4mzv4xxcn7itmewd2unfqo5e5qsxcwy",
  "sig": "3aaa0be24087764f58d9ea3ddd22ee150fdc73fedc9a6040528c590986e84cf368263538544de6f60f38f797842a6bef07c66b802faed66b62e8800d3a66b431",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "short-author-and-block-html"
  },
  "kind": "decision",
  "cites": [
    "bafyreibbwsaxaokorsgwbxffgzqijnzqctiwh7kmk5zkwd3khmhdrzaf4q"
  ],
  "rev": "223mtihdwercj",
  "seq": 5,
  "of": 7,
  "text_len": 819,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgIbSBcDlOjI1g3KU2YIS3MBTRY/1MV3KrD2o7DjjkBeRmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx4G3Nob3J0LWF1dGhvci1hbmQtYmxvY2staHRtbGlhcnRpZmFjdHOBoWZDb21taXR4KGU2ZmRmMGJmMjUxYzU2YTViNzIxZTBhYmYxNDY0YWNiZTA3MmUyZjdpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZZc1PFXKU="
}
---

adversarial review of short-author-and-block-html: SHIP. Independent Opus review, no breaking input found. short_author: boundary correct (>11 abbreviates, else whole), no scalar-split/panic (slices a Vec<char>, guarded by len>=12), empty author safe, {:<12} pads by char count so the multibyte … aligns; collision fixed for the tested head-share case; the visible … is a net honest-ambiguity gain over the silent take(8). markdown Event::Html: verified pulldown-cmark 0.13.4 emits one Html event per line each ending \n; keeping the trailing \n flushes so lines stay separate with no spurious empties; single-line/inline HTML and the alpha.7 <tag>-preservation via InlineHtml are unregressed. 71 tests, clippy -D warnings, fmt clean; no stale assertions on the old 8-char width or old HTML behavior. Verdict: SHIP.
***8<***
---
{
  "v": 3,
  "cid": "bafyreifumg2ld6gr4ns5mc34jnz5a3kcklvnqwoon44gefuwkazbwdm5r4",
  "sig": "38a24d568ed6d6b3f690f0ca2219e506dc2982645b0cf274b19a02753530571215837f7b90e2fc79eb1443db654df8d17ddd193f9eb85806c8f4d59c86b10edb",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "short-author-and-block-html"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223mtihdwmcdo",
  "seq": 6,
  "of": 7,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx4G3Nob3J0LWF1dGhvci1hbmQtYmxvY2staHRtbGlhcnRpZmFjdHOBoWZDb21taXR4KGU2ZmRmMGJmMjUxYzU2YTViNzIxZTBhYmYxNDY0YWNiZTA3MmUyZjdpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZZc1PJIMw="
}
---
