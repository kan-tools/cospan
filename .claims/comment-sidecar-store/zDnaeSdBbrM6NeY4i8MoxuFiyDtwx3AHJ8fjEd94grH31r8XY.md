---
{
  "v": 3,
  "cid": "bafyreibrhqkg5olmtxhg46s5vt6ktkzx64bcaw5i6zrwsngjyereprw5lm",
  "sig": "3bc7390e1b12b6b68961f2a20e529ab54bd20a76f391497718bf341af1f92f6d78465b5da6e251319c8344858016cd8448d2e800adcabdf379859c2172d4e200",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comment-sidecar-store"
  },
  "kind": "observation",
  "cites": [],
  "rev": "223mtguc4boqp",
  "seq": 0,
  "of": 13,
  "text_len": 195,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlU2RCYnJNNk5lWTRpOE1veHVGaXlEdHd4M0FISjhmakVkOTRnckgzMXI4WFllYWdlbnT2Z3N1YmplY3ShZUxvY2FsdWNvbW1lbnQtc2lkZWNhci1zdG9yZWlhcnRpZmFjdHOBoWZDb21taXR4KDcyYTk4N2IyNDllZTJiNmZmZDE4M2Q3MDI4MTc2OTFmYjIyZjY4ODFpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDgzNWY4N2FkZTQwYmRjOGRmMjJjM2JiZDI3OGQ1N2UwNDBmYWEwOGNjNmU0MTVkNmJkMDM4NGViOWE3ZTUzY2VrcmVjb3JkZWRfYXQbAAZZZpAj0mo="
}
---

design doc .design/comment-sidecar-store.md checked against the live design-doc schema: validation: 9 check(s), 0 failed, 0 warning(s), 0 unchecked, 0 open question(s) [doc 7778:66ce2932a22eb8cd]
***8<***
---
{
  "v": 3,
  "cid": "bafyreidu7t2vhcoqkrzs2uqtatw5l6hnyxw5dkaa52pqk7ee3vlfdtcqmi",
  "sig": "facac017590d1bff80572e1e6f47453aaa900e178119fc9fd3f2b7e1ee3198975b127e342cc0af2a1556ade013bd0c12bad70cf261d75a201770f6a852a69b4e",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comment-sidecar-store"
  },
  "kind": "plan",
  "cites": [
    "bafyreibrhqkg5olmtxhg46s5vt6ktkzx64bcaw5i6zrwsngjyereprw5lm"
  ],
  "rev": "223mtguc4ftfq",
  "seq": 1,
  "of": 13,
  "text_len": 542,
  "content": "p2Rib2R5oWRQbGFuoWR0ZXh0YGVjaXRlc4HYKlglAAFxEiAxPBRuuWydzm56Xaz8qas39wIgW6j2Y2k0ycEiR8bdW2ZhdXRob3KiY2RpZHg5ZGlkOmtleTp6RG5hZVNkQmJyTTZOZVk0aThNb3h1Rml5RHR3eDNBSEo4ZmpFZDk0Z3JIMzFyOFhZZWFnZW509mdzdWJqZWN0oWVMb2NhbHVjb21tZW50LXNpZGVjYXItc3RvcmVpYXJ0aWZhY3RzgaFmQ29tbWl0eCg3MmE5ODdiMjQ5ZWUyYjZmZmQxODNkNzAyODE3NjkxZmIyMmY2ODgxaXdvcmtzcGFjZaFpV29ya3NwYWNleEA4MzVmODdhZGU0MGJkYzhkZjIyYzNiYmQyNzhkNTdlMDQwZmFhMDhjYzZlNDE1ZDZiZDAzODRlYjlhN2U1M2Nla3JlY29yZGVkX2F0GwAGWWaQJeUL"
}
---

comment-sidecar-store design (.design/comment-sidecar-store.md): Give cospan its reason to exist beyond a viewer: an ephemeral, per-file comment sidecar whose comments are pinned to a text fingerprint and re-resolved against the current file on every read, into Anchored / Drifted / Unresolvable. Add `cospan comment add` to drop a comment and `cospan comments` to list each with its live state. This is the headless doc-comment round trip and the start of P1. [validation: 9 check(s), 0 failed, 0 warning(s), 0 unchecked, 0 open question(s)]
***8<***
---
{
  "v": 3,
  "cid": "bafyreidto224o4oepyflavivsur3ymzb4zn3rlve23igkfy3ktqqqm2j44",
  "sig": "0d21cee6cbb8dee775f34e867d8e6c561425a6fb3f5e391f3bcb88aad36ee2711a413a97a4933c169ca985515775f6226eae893303eea39b18116948585f4042",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comment-sidecar-store"
  },
  "kind": "subject",
  "cites": [],
  "rev": "223mtguc4h6e6",
  "seq": 2,
  "of": 13,
  "content": "p2Rib2R5oWdTdWJqZWN0omV0aXRsZXg4RmVhdHVyZTogQ29tbWVudCBzaWRlY2FyIHN0b3JlIHdpdGggbGl2ZSByZS1sb2NhbGl6YXRpb25sc3ViamVjdF9raW5kZElkZWFlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlU2RCYnJNNk5lWTRpOE1veHVGaXlEdHd4M0FISjhmakVkOTRnckgzMXI4WFllYWdlbnT2Z3N1YmplY3ShZUxvY2FsdWNvbW1lbnQtc2lkZWNhci1zdG9yZWlhcnRpZmFjdHOBoWZDb21taXR4KDcyYTk4N2IyNDllZTJiNmZmZDE4M2Q3MDI4MTc2OTFmYjIyZjY4ODFpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDgzNWY4N2FkZTQwYmRjOGRmMjJjM2JiZDI3OGQ1N2UwNDBmYWEwOGNjNmU0MTVkNmJkMDM4NGViOWE3ZTUzY2VrcmVjb3JkZWRfYXQbAAZZZpAmkN8="
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreihrzxg2oa2qblddpvvbjrptlxsiyksoncfbabwflgxamvkqbfjcrq",
  "sig": "9fb29ea9e5d3b739ec58cb3db3e33feeb3efa0f5cbe95e4aa57cfba3f748576a653bacbeb4dea03f5a73f4cbbab8afcc9289225e449a0eb0b00b3019d9252af1",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comment-sidecar-store"
  },
  "kind": "decision",
  "cites": [
    "bafyreidu7t2vhcoqkrzs2uqtatw5l6hnyxw5dkaa52pqk7ee3vlfdtcqmi"
  ],
  "rev": "223mtguc4ngzu",
  "seq": 3,
  "of": 13,
  "text_len": 208,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgdPz1U4nQVHMtUhME7dX47cXt0agA7p8FfITdVlHMUGJmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx1Y29tbWVudC1zaWRlY2FyLXN0b3JlaWFydGlmYWN0c4GhZkNvbW1pdHgoNzJhOTg3YjI0OWVlMmI2ZmZkMTgzZDcwMjgxNzY5MWZiMjJmNjg4MWl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAODM1Zjg3YWRlNDBiZGM4ZGYyMmMzYmJkMjc4ZDU3ZTA0MGZhYTA4Y2M2ZTQxNWQ2YmQwMzg0ZWI5YTdlNTNjZWtyZWNvcmRlZF9hdBsABllmkCmzjA=="
}
---

RQ-1: Sidecar location is the .cospan/comments/<path>.jsonl tree (one JSONL per commented file under a single gitignored directory), not a sibling `<file>.cospan.jsonl` — it keeps working directories clean.
***8<***
---
{
  "v": 3,
  "cid": "bafyreihqf5jr53ayv4zcbqrywst6ofckael7lvme37we7cvklpggdhpgni",
  "sig": "d98dc1f1fc73e457f6d73bf3a82d37d8ccad6473925cfcb94256a2aab35f22b4242c6e03bb46ac287eac50e80408864c756b84241c7b33fbcebffa849128436c",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comment-sidecar-store"
  },
  "kind": "decision",
  "cites": [
    "bafyreidu7t2vhcoqkrzs2uqtatw5l6hnyxw5dkaa52pqk7ee3vlfdtcqmi"
  ],
  "rev": "223mtguc4rh6i",
  "seq": 4,
  "of": 13,
  "text_len": 200,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgdPz1U4nQVHMtUhME7dX47cXt0agA7p8FfITdVlHMUGJmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx1Y29tbWVudC1zaWRlY2FyLXN0b3JlaWFydGlmYWN0c4GhZkNvbW1pdHgoNzJhOTg3YjI0OWVlMmI2ZmZkMTgzZDcwMjgxNzY5MWZiMjJmNjg4MWl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAODM1Zjg3YWRlNDBiZGM4ZGYyMmMzYmJkMjc4ZDU3ZTA0MGZhYTA4Y2M2ZTQxNWQ2YmQwMzg0ZWI5YTdlNTNjZWtyZWNvcmRlZF9hdBsABllmkCu0HQ=="
}
---

RQ-2: The step ships both `cospan comment add` and `cospan comments`, so the headless round trip (drop a comment, edit, re-localize) is exercisable end-to-end rather than requiring hand-written JSONL.
***8<***
---
{
  "v": 3,
  "cid": "bafyreicyzu5zwlw7ulrjalmqszkl32cngwbq2gqtky3b7hexucgo64aqfm",
  "sig": "72ffe7635acff29d7f1b4020ebfb525a0f946e839a14c41b2e1a5674e9b5dcef1e11a18ff007ed8f2eda8ca62b955161deee8560902177d84ad9d9168636eb8a",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comment-sidecar-store"
  },
  "kind": "decision",
  "cites": [
    "bafyreidu7t2vhcoqkrzs2uqtatw5l6hnyxw5dkaa52pqk7ee3vlfdtcqmi"
  ],
  "rev": "223mtguc4vfd7",
  "seq": 5,
  "of": 13,
  "text_len": 234,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgdPz1U4nQVHMtUhME7dX47cXt0agA7p8FfITdVlHMUGJmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx1Y29tbWVudC1zaWRlY2FyLXN0b3JlaWFydGlmYWN0c4GhZkNvbW1pdHgoNzJhOTg3YjI0OWVlMmI2ZmZkMTgzZDcwMjgxNzY5MWZiMjJmNjg4MWl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAODM1Zjg3YWRlNDBiZGM4ZGYyMmMzYmJkMjc4ZDU3ZTA0MGZhYTA4Y2M2ZTQxNWQ2YmQwMzg0ZWI5YTdlNTNjZWtyZWNvcmRlZF9hdBsABllmkC2ssg=="
}
---

RQ-3: Re-localization uses incremental last-seen tracking — the anchor is re-captured and written back on each Anchored/Drifted resolution — rather than always matching against the frozen original, so accumulated drift is avoided.
***8<***
---
{
  "v": 3,
  "cid": "bafyreicdx66ww3bjnfgxpea3joowfy3rabulq6jjbtjxzrrlba4rngksca",
  "sig": "4bddea0f33908428fc931bbe75bd1982d8b284edab84994f35b1bb4411c9f0a466db328817edb7412f3cf4b5289e2bb3b53876f6ed03174cbd4802a96ea7aa6c",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comment-sidecar-store"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223mtgucr6hqi",
  "seq": 6,
  "of": 13,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx1Y29tbWVudC1zaWRlY2FyLXN0b3JlaWFydGlmYWN0c4GhZkNvbW1pdHgoNzJhOTg3YjI0OWVlMmI2ZmZkMTgzZDcwMjgxNzY5MWZiMjJmNjg4MWl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAODM1Zjg3YWRlNDBiZGM4ZGYyMmMzYmJkMjc4ZDU3ZTA0MGZhYTA4Y2M2ZTQxNWQ2YmQwMzg0ZWI5YTdlNTNjZWtyZWNvcmRlZF9hdBsABllmkXI2TQ=="
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreibbfc5plbqtg23mkskm3fjc2dbdwbwkiblkcwws7nql4bqvmae32m",
  "sig": "c1b6dee7c22f70cea8715a29b4c102226af3f6d5678b67cc0f1fe37ebf81c03553b822ac8a5ce7623a79ba18289e228e7e2feb4d959afcd12955159eb6ac0cc1",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comment-sidecar-store"
  },
  "kind": "decision",
  "cites": [
    "bafyreidu7t2vhcoqkrzs2uqtatw5l6hnyxw5dkaa52pqk7ee3vlfdtcqmi"
  ],
  "rev": "223mtgv5wcsig",
  "seq": 7,
  "of": 13,
  "text_len": 547,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgdPz1U4nQVHMtUhME7dX47cXt0agA7p8FfITdVlHMUGJmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx1Y29tbWVudC1zaWRlY2FyLXN0b3JlaWFydGlmYWN0c4GhZkNvbW1pdHgoNzJhOTg3YjI0OWVlMmI2ZmZkMTgzZDcwMjgxNzY5MWZiMjJmNjg4MWl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAODM1Zjg3YWRlNDBiZGM4ZGYyMmMzYmJkMjc4ZDU3ZTA0MGZhYTA4Y2M2ZTQxNWQ2YmQwMzg0ZWI5YTdlNTNjZWtyZWNvcmRlZF9hdBsABllmx8RhZQ=="
}
---

adversarial review of comment-sidecar-store: APPROVE-WITH-FOLLOW-UPS — adversarial review of comment-sidecar-store: honest Anchored/Drifted/Unresolvable round trip verified live and by 5 tests, all 6 ACs met, clippy-clean, .cospan gitignored, no kan writes; follow-ups: panic on comment-add to an empty file (lib.rs:55 via StoredAnchor::capture), base_hash is write-only (design says it short-circuits unchanged files but it is never read), one torn JSONL line bricks the whole sidecar (non-atomic save), and CLI flag-order/body-token mis-parse.
***8<***
---
{
  "v": 3,
  "cid": "bafyreib6jqp2tpgiqvxvzzaopbo4n7x4ha6uxp43lfhvkwe6emyo6wurmy",
  "sig": "4810bdf1786aeb3e7e9a8071a5bd17a53ef826ec25700c6c4fad2517bbdaf4cf40eecc2ae95e5d31daa48723eaf605e5fbde43d447c7a578ec057915e3170aa6",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comment-sidecar-store"
  },
  "kind": "observation",
  "cites": [
    "bafyreibbfc5plbqtg23mkskm3fjc2dbdwbwkiblkcwws7nql4bqvmae32m"
  ],
  "rev": "223mtgv6fipze",
  "seq": 8,
  "of": 13,
  "text_len": 400,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgISi69YYTNrbFSUzZUi0MI7BspAVqFa0vtgvgYVYAm9NmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx1Y29tbWVudC1zaWRlY2FyLXN0b3JlaWFydGlmYWN0c4GhZkNvbW1pdHgoNzJhOTg3YjI0OWVlMmI2ZmZkMTgzZDcwMjgxNzY5MWZiMjJmNjg4MWl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAODM1Zjg3YWRlNDBiZGM4ZGYyMmMzYmJkMjc4ZDU3ZTA0MGZhYTA4Y2M2ZTQxNWQ2YmQwMzg0ZWI5YTdlNTNjZWtyZWNvcmRlZF9hdBsABllmyLdXgg=="
}
---

Panic on comment-add to an empty (zero-line) file: comment_cmd -> StoredAnchor::capture -> Anchor::from_file indexes lines[line+1..after_end] = lines[1..0] and panics 'range start index 1 out of range for slice of length 0' (src/lib.rs:55). Reachable from documented CLI usage; relocalize guards the empty case but from_file does not. Reproduced: 'cospan comment add <empty> --line 1 body' exits 101.
***8<***
---
{
  "v": 3,
  "cid": "bafyreidrldu5bg6wfoqbr674mx4fsp565vdlnruqec5p3zxtvlotvdiwcu",
  "sig": "d104646dc7ce312b608b4900225982652495cfc523dafddc361dcf564b9ff20b174fc9200f02beb1a7809bac14accc6f3f2075d2f4ac4e9c2d0c24b2bc57ae5c",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comment-sidecar-store"
  },
  "kind": "observation",
  "cites": [
    "bafyreibbfc5plbqtg23mkskm3fjc2dbdwbwkiblkcwws7nql4bqvmae32m"
  ],
  "rev": "223mtgv6fmxhb",
  "seq": 9,
  "of": 13,
  "text_len": 523,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgISi69YYTNrbFSUzZUi0MI7BspAVqFa0vtgvgYVYAm9NmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx1Y29tbWVudC1zaWRlY2FyLXN0b3JlaWFydGlmYWN0c4GhZkNvbW1pdHgoNzJhOTg3YjI0OWVlMmI2ZmZkMTgzZDcwMjgxNzY5MWZiMjJmNjg4MWl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAODM1Zjg3YWRlNDBiZGM4ZGYyMmMzYmJkMjc4ZDU3ZTA0MGZhYTA4Y2M2ZTQxNWQ2YmQwMzg0ZWI5YTdlNTNjZWtyZWNvcmRlZF9hdBsABllmyLl1QA=="
}
---

base_hash is write-only: content_hash is computed and stored on capture (src/comments.rs:40) but never read or compared anywhere in non-test code; localize_and_update always calls relocalize. The design/architecture states base_hash is 'used to short-circuit an unchanged file' — that short-circuit is unimplemented, so base_hash is speculative provenance state and the design doc overstates the impl. (Also: DefaultHasher is not guaranteed stable across Rust releases for a persisted hash — latent, moot while unread.)
***8<***
---
{
  "v": 3,
  "cid": "bafyreihzrbj57bdqolj4copzbv34svkbh2ugmu3aofkgbd6tq4elsnysvm",
  "sig": "bbd98f9703548055a6fe75a96e4dbdba5923ddaf945c9fd2e14e5e6168078b810252cc0042ce888734c885bb4f3b70b7b51d53d22e7a7374d6061efc2653013f",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comment-sidecar-store"
  },
  "kind": "observation",
  "cites": [
    "bafyreibbfc5plbqtg23mkskm3fjc2dbdwbwkiblkcwws7nql4bqvmae32m"
  ],
  "rev": "223mtgv6fr2p4",
  "seq": 10,
  "of": 13,
  "text_len": 475,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgISi69YYTNrbFSUzZUi0MI7BspAVqFa0vtgvgYVYAm9NmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx1Y29tbWVudC1zaWRlY2FyLXN0b3JlaWFydGlmYWN0c4GhZkNvbW1pdHgoNzJhOTg3YjI0OWVlMmI2ZmZkMTgzZDcwMjgxNzY5MWZiMjJmNjg4MWl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAODM1Zjg3YWRlNDBiZGM4ZGYyMmMzYmJkMjc4ZDU3ZTA0MGZhYTA4Y2M2ZTQxNWQ2YmQwMzg0ZWI5YTdlNTNjZWtyZWNvcmRlZF9hdBsABllmyLuCNg=="
}
---

One malformed/torn JSONL line bricks the whole sidecar: load() collects into Result<Vec<Comment>> (src/comments.rs:90-100), so a single bad line makes BOTH 'comment add' and 'comments' hard-fail (exit 1) — the good comments become inaccessible. save() is a non-atomic full-file std::fs::write (src/comments.rs:103-113), so a crash/interrupt mid-save produces exactly such a torn trailing line. Disposable state so no durable loss, but feature-breaking per file. Reproduced.
***8<***
---
{
  "v": 3,
  "cid": "bafyreihomp7wqh2a7zmc5kyshkkklzabfbqthddx2n4nsol76adu6z65de",
  "sig": "19bc1bf6ed35995d8eea27cfaf9d3313ed23d9f2fdee185e0997a7e176990e4d4c982cc0b989ed4ca22c2d494d069659b97195e5beef0221f6f06692d83e64be",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comment-sidecar-store"
  },
  "kind": "observation",
  "cites": [
    "bafyreibbfc5plbqtg23mkskm3fjc2dbdwbwkiblkcwws7nql4bqvmae32m"
  ],
  "rev": "223mtgv6qnv2x",
  "seq": 11,
  "of": 13,
  "text_len": 563,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgISi69YYTNrbFSUzZUi0MI7BspAVqFa0vtgvgYVYAm9NmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx1Y29tbWVudC1zaWRlY2FyLXN0b3JlaWFydGlmYWN0c4GhZkNvbW1pdHgoNzJhOTg3YjI0OWVlMmI2ZmZkMTgzZDcwMjgxNzY5MWZiMjJmNjg4MWl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAODM1Zjg3YWRlNDBiZGM4ZGYyMmMzYmJkMjc4ZDU3ZTA0MGZhYTA4Y2M2ZTQxNWQ2YmQwMzg0ZWI5YTdlNTNjZWtyZWNvcmRlZF9hdBsABllmyWnrqQ=="
}
---

CLI parse defects in comment_cmd (src/main.rs:63-121): (1) flag-order dependence — 'cospan comment add --line 1 file body' picks the first non---token as the file, which is the numeric line arg, so it tries to read file '1' and fails; documented order (file first) works. (2) body tokens equal to --line/--ctx are swallowed as flags: 'please check the --ctx handling' stores body 'please check the'. (3) id is 'c_{micros}' with no counter (architecture said 'created_at plus a counter'); collision only on same-microsecond cross-process, effectively impossible.
***8<***
---
{
  "v": 3,
  "cid": "bafyreia3t56is7ie62iftxuqm5u3n3qv3n6k77mygddwud5a25k3b3khxa",
  "sig": "e33c532abf809923bbe02dcd50ada7771b95282fdc0cc973df2c4447d26f364a179d94b4161c5af4d3400bc2e9369531681badba7af52ec0cb2cba65b5e04ec2",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comment-sidecar-store"
  },
  "kind": "observation",
  "cites": [
    "bafyreibbfc5plbqtg23mkskm3fjc2dbdwbwkiblkcwws7nql4bqvmae32m"
  ],
  "rev": "223mtgv6qtn4f",
  "seq": 12,
  "of": 13,
  "text_len": 603,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgISi69YYTNrbFSUzZUi0MI7BspAVqFa0vtgvgYVYAm9NmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx1Y29tbWVudC1zaWRlY2FyLXN0b3JlaWFydGlmYWN0c4GhZkNvbW1pdHgoNzJhOTg3YjI0OWVlMmI2ZmZkMTgzZDcwMjgxNzY5MWZiMjJmNjg4MWl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAODM1Zjg3YWRlNDBiZGM4ZGYyMmMzYmJkMjc4ZDU3ZTA0MGZhYTA4Y2M2ZTQxNWQ2YmQwMzg0ZWI5YTdlNTNjZWtyZWNvcmRlZF9hdBsABllmyWzL1w=="
}
---

Design-level walk risk (REQ-4 / RQ-3, telos/honest-ambiguity tension): localize_and_update re-anchors on ANY Anchored OR Drifted match with no confidence floor beyond the relocalizer's FUZZY_FLOOR=0.60. A Drifted match at 0.60-0.64 re-captures the anchor to the new text, which is then tracked at conf 1.00; over a sequence of small edits the anchor can walk to unrelated text while the tool reports ANCHORED — false certainty. Documented tradeoff, but worth a higher re-anchor floor. AC-5's test only exercises the Anchored branch; the Drifted re-capture branch is unit-untested (verified live only).
