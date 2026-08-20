---
{
  "v": 3,
  "cid": "bafyreiasl54psf4op2fl3voujtbn5dtvywtvfcwz3m22wfcvtw3rfifrqq",
  "sig": "723a95628c4501de32f36f834ba3c3eb96cb21ca925ea30f414b7376e083733b641a23118086730e128634be028bed978432d9ffed2504c5928f85f987f67d59",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "schema-block-summaries"
  },
  "kind": "observation",
  "cites": [],
  "rev": "223mti7zokph4",
  "seq": 0,
  "of": 8,
  "text_len": 196,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlU2RCYnJNNk5lWTRpOE1veHVGaXlEdHd4M0FISjhmakVkOTRnckgzMXI4WFllYWdlbnT2Z3N1YmplY3ShZUxvY2FsdnNjaGVtYS1ibG9jay1zdW1tYXJpZXNpYXJ0aWZhY3RzgaFmQ29tbWl0eChjYTBiMzc3MWYwNWJjZDNkOGQyMjU5MzQ5MWI5MDQ1NDQ1ZDM2Zjg3aXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWXF/SFAt"
}
---

design doc .design/schema-block-summaries.md checked against the live design-doc schema: validation: 9 check(s), 0 failed, 0 warning(s), 0 unchecked, 0 open question(s) [doc 5491:8ef473d9c2861a5c]
***8<***
---
{
  "v": 3,
  "cid": "bafyreibifw3xybqcn3ike7my6ot5xsxz3ihbww3cmijt4sbj7rv6ngkek4",
  "sig": "865f0980d5fc5fe500aac9c4ea261fcc05dbb7b7db015f2dccb67168d4b6c0e763d273161a95de7266206d4076bedacb29e80357bd2f8a5b416589344413f78e",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "schema-block-summaries"
  },
  "kind": "plan",
  "cites": [
    "bafyreiasl54psf4op2fl3voujtbn5dtvywtvfcwz3m22wfcvtw3rfifrqq"
  ],
  "rev": "223mti7zorpv3",
  "seq": 1,
  "of": 8,
  "text_len": 564,
  "content": "p2Rib2R5oWRQbGFuoWR0ZXh0YGVjaXRlc4HYKlglAAFxEiASX3j5F45+ir3V1EzC3o51xadSitnbNasUVZ23EqCxhGZhdXRob3KiY2RpZHg5ZGlkOmtleTp6RG5hZVNkQmJyTTZOZVk0aThNb3h1Rml5RHR3eDNBSEo4ZmpFZDk0Z3JIMzFyOFhZZWFnZW509mdzdWJqZWN0oWVMb2NhbHZzY2hlbWEtYmxvY2stc3VtbWFyaWVzaWFydGlmYWN0c4GhZkNvbW1pdHgoY2EwYjM3NzFmMDViY2QzZDhkMjI1OTM0OTFiOTA0NTQ0NWQzNmY4N2l3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABllxf0vW7Q=="
}
---

schema-block-summaries design (.design/schema-block-summaries.md): Teach the claim-detail block renderer to summarize the two day fenced-block types it currently dumps as raw JSON: `day-bridge` (10 subjects — the most common unsummarized block, whose plan is a nested atom tree) and `day-schema` (the structural rules on `schema/design-doc`). Both become human-readable `block_summary` views, closing the gap left when alpha.7's design named `day-bridge` but never implemented it. [validation: 9 check(s), 0 failed, 0 warning(s), 0 unchecked, 0 open question(s)]
***8<***
---
{
  "v": 3,
  "cid": "bafyreifusnzzb7mjmv7cyn2tj2qpolvoj2ek7mxhup27uqal5i6zy4khra",
  "sig": "bec4f909c8c4f0cb27690a3a71fc43b39603696ce7c70d7b356d87a303bda6ab64d8930d102cb324c52d1f44930b2bb75eac58b805f2456d893cfdf50f0e814b",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "schema-block-summaries"
  },
  "kind": "subject",
  "cites": [],
  "rev": "223mti7zotz67",
  "seq": 2,
  "of": 8,
  "content": "p2Rib2R5oWdTdWJqZWN0omV0aXRsZXgyRmVhdHVyZTogZGF5LWJyaWRnZSBhbmQgZGF5LXNjaGVtYSBibG9jayBzdW1tYXJpZXNsc3ViamVjdF9raW5kZElkZWFlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlU2RCYnJNNk5lWTRpOE1veHVGaXlEdHd4M0FISjhmakVkOTRnckgzMXI4WFllYWdlbnT2Z3N1YmplY3ShZUxvY2FsdnNjaGVtYS1ibG9jay1zdW1tYXJpZXNpYXJ0aWZhY3RzgaFmQ29tbWl0eChjYTBiMzc3MWYwNWJjZDNkOGQyMjU5MzQ5MWI5MDQ1NDQ1ZDM2Zjg3aXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWXF/TPwb"
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreibr7g73ascr524ekjd62c4sewku6ofsbswu5yik2nthe3gaphjnze",
  "sig": "321cd97235254f184986d545777c74a4c360189e3fc3586b1ccb8875f390443865469dcbf263ace1c0d1e2d4fe43411d04ba2d3cae5b7bd0db097c48a4c03b4d",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "schema-block-summaries"
  },
  "kind": "decision",
  "cites": [
    "bafyreibifw3xybqcn3ike7my6ot5xsxz3ihbww3cmijt4sbj7rv6ngkek4"
  ],
  "rev": "223mti7zp2z5y",
  "seq": 3,
  "of": 8,
  "text_len": 216,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgKC23fAYCbtCifZjzp9vK+doOG1tiYhM+SCn8a+aZRFdmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx2c2NoZW1hLWJsb2NrLXN1bW1hcmllc2lhcnRpZmFjdHOBoWZDb21taXR4KGNhMGIzNzcxZjA1YmNkM2Q4ZDIyNTkzNDkxYjkwNDU0NDVkMzZmODdpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZZcX9QfA0="
}
---

RQ-1: `day-schema` is rendered generically (every key via `append_extra_keys`) rather than with a bespoke per-field layout, so the view never falls behind the schema's vocabulary — a new rule appears automatically.
***8<***
---
{
  "v": 3,
  "cid": "bafyreihzqm2zydns7o25663tmztbzfx4k73kxit4e4r5ogbzpnr4e4aytu",
  "sig": "834eb934779077def5eda09676114ed11b6ed984efb3eb5a4ab43850319dd2ad5e326408cec54e10da977b69a1af51527b8b595ebad806ac6bf14980c5647ef8",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "schema-block-summaries"
  },
  "kind": "decision",
  "cites": [
    "bafyreibifw3xybqcn3ike7my6ot5xsxz3ihbww3cmijt4sbj7rv6ngkek4"
  ],
  "rev": "223mti7zpcijl",
  "seq": 4,
  "of": 8,
  "text_len": 200,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgKC23fAYCbtCifZjzp9vK+doOG1tiYhM+SCn8a+aZRFdmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx2c2NoZW1hLWJsb2NrLXN1bW1hcmllc2lhcnRpZmFjdHOBoWZDb21taXR4KGNhMGIzNzcxZjA1YmNkM2Q4ZDIyNTkzNDkxYjkwNDU0NDVkMzZmODdpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZZcX9UOXo="
}
---

RQ-2: `day-bridge`'s plan is flattened with day's `>` / `&` / `|` operators rather than shown as a bullet tree, matching how day itself writes composition and keeping the summary to one line per plan.
***8<***
---
{
  "v": 3,
  "cid": "bafyreied3xy2hx3tv7aofwxjqlwoljhwwxx2vceqjsumwzddcuxihkkece",
  "sig": "108ab7111e8f395f7cff672778979379322a9c50743e0381ed5e1c0db889ec5f797cfc16dae2647e0dc78311ad76ee7a25480b9f28ce77bdb5f4d84bb7158e2b",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "schema-block-summaries"
  },
  "kind": "decision",
  "cites": [
    "bafyreibifw3xybqcn3ike7my6ot5xsxz3ihbww3cmijt4sbj7rv6ngkek4"
  ],
  "rev": "223mti7zpjulg",
  "seq": 5,
  "of": 8,
  "text_len": 148,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgKC23fAYCbtCifZjzp9vK+doOG1tiYhM+SCn8a+aZRFdmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx2c2NoZW1hLWJsb2NrLXN1bW1hcmllc2lhcnRpZmFjdHOBoWZDb21taXR4KGNhMGIzNzcxZjA1YmNkM2Q4ZDIyNTkzNDkxYjkwNDU0NDVkMzZmODdpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZZcX9X6bg="
}
---

RQ-3: This is the "schemas next" cycle sequenced after the path-aggregated tree; it is additive to `block_summary` and does not touch the tree work.
***8<***
---
{
  "v": 3,
  "cid": "bafyreibwzml4fpkzoq53ioczyy5dnc6sd5aqi7oyyjd5nvhh35hgchzw3e",
  "sig": "d9da2e8f19e02c847c9ceeffecf5d9a103ebbd12f22f07944d4d491922ee59c94b49f66dd467a52ba0ae4f16e1848cdd81ca7c75925d568d8ceb58b1f05da895",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "schema-block-summaries"
  },
  "kind": "decision",
  "cites": [
    "bafyreibifw3xybqcn3ike7my6ot5xsxz3ihbww3cmijt4sbj7rv6ngkek4"
  ],
  "rev": "223mtiakypwrd",
  "seq": 6,
  "of": 8,
  "text_len": 888,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgKC23fAYCbtCifZjzp9vK+doOG1tiYhM+SCn8a+aZRFdmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx2c2NoZW1hLWJsb2NrLXN1bW1hcmllc2lhcnRpZmFjdHOBoWZDb21taXR4KGNhMGIzNzcxZjA1YmNkM2Q4ZDIyNTkzNDkxYjkwNDU0NDVkMzZmODdpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZZcaHq8X4="
}
---

adversarial review of schema-block-summaries: SHIP (after one BLOCK-and-fix). Independent Opus review initially BLOCKED: flatten_plan matched invented plan operators par/alt, but day's real Node grammar (src/bridge.rs enum Seq/All/Any, serde rename_all lowercase) serializes seq/all/any — so any bridge using concurrency/alternatives (13 nodes in day's own corpus, incl. the newest version) hit the JSON fallback and leaked raw JSON, the exact failure the feature exists to prevent, masked by tests using fictional keys. Fixed: flatten_plan now maps seq->' > ', all->' & ', any->' | '; AC-2 test rewritten to all/any plus a real corpus-shaped seq+any regression test; design doc REQ-1/AC-2/RQ-2 corrected to match. Re-verified: 60 tests, clippy -D warnings, fmt clean; the real bridge {seq:[design,{any:[build,build]}]} renders 'design > build | build' with no JSON leak. Verdict: SHIP.
***8<***
---
{
  "v": 3,
  "cid": "bafyreiax4ukxbncp7jghfnspypj7n6nulepmrhjvnxqwjs42aiwbl222a4",
  "sig": "8cb8ee8361f30459d1a9778e8ddb4a4eb33331a289b25079e2db7b647e50556f6b343bd5e5bfc2314ca34be49fc401720c6966bb912b900e3a5b3fa65a491ca9",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "schema-block-summaries"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223mtial65suu",
  "seq": 7,
  "of": 8,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx2c2NoZW1hLWJsb2NrLXN1bW1hcmllc2lhcnRpZmFjdHOBoWZDb21taXR4KGNhMGIzNzcxZjA1YmNkM2Q4ZDIyNTkzNDkxYjkwNDU0NDVkMzZmODdpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZZcaJB4uY="
}
---
