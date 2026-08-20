---
{
  "v": 3,
  "cid": "bafyreig7d5fsgbmvtasimbzg3y5g6dvywwc42rzmktvs5nkotuv6uiaee4",
  "sig": "47b2d49fdbd3198a3bd5b43304baf099645447260e21b96933b59170bff3312542f86b1a7911c9e7a99450a17b0ef033a9e7e0b61d5007594609d5765294b870",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "witness-and-block-vocabulary"
  },
  "kind": "observation",
  "cites": [],
  "rev": "223mtibjeg55q",
  "seq": 0,
  "of": 8,
  "text_len": 202,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlU2RCYnJNNk5lWTRpOE1veHVGaXlEdHd4M0FISjhmakVkOTRnckgzMXI4WFllYWdlbnT2Z3N1YmplY3ShZUxvY2FseBx3aXRuZXNzLWFuZC1ibG9jay12b2NhYnVsYXJ5aWFydGlmYWN0c4GhZkNvbW1pdHgoY2U2NDhjOTYxZTQ1NmUwODliMDRjNjQ1YjNjMDYyOWY0YzZmMGQ2Y2l3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABllx3qYMBw=="
}
---

design doc .design/witness-and-block-vocabulary.md checked against the live design-doc schema: validation: 9 check(s), 0 failed, 0 warning(s), 0 unchecked, 0 open question(s) [doc 6142:c87b9a54225bc7ff]
***8<***
---
{
  "v": 3,
  "cid": "bafyreihucwx2dqlg7es57qnp3635coob254q5r35jawajvhz6p5xhgpqci",
  "sig": "87646a22584acb257c2f1792cb5b02ffea73703e659e5c1a48c4ec247d24d11c70139e3a82a2eb520d4a26bb489e4c222fa94587c23732363c58073511c61453",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "witness-and-block-vocabulary"
  },
  "kind": "plan",
  "cites": [
    "bafyreig7d5fsgbmvtasimbzg3y5g6dvywwc42rzmktvs5nkotuv6uiaee4"
  ],
  "rev": "223mtibjemvw5",
  "seq": 1,
  "of": 8,
  "text_len": 537,
  "content": "p2Rib2R5oWRQbGFuoWR0ZXh0YGVjaXRlc4HYKlglAAFxEiDfH0sjBZWYJIYHJt46bw64tYXNRyxU6y61Tp0r6iAEJ2ZhdXRob3KiY2RpZHg5ZGlkOmtleTp6RG5hZVNkQmJyTTZOZVk0aThNb3h1Rml5RHR3eDNBSEo4ZmpFZDk0Z3JIMzFyOFhZZWFnZW509mdzdWJqZWN0oWVMb2NhbHgcd2l0bmVzcy1hbmQtYmxvY2stdm9jYWJ1bGFyeWlhcnRpZmFjdHOBoWZDb21taXR4KGNlNjQ4Yzk2MWU0NTZlMDg5YjA0YzY0NWIzYzA2MjlmNGM2ZjBkNmNpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZZcd6pbxo="
}
---

witness-and-block-vocabulary design (.design/witness-and-block-vocabulary.md): Make cospan render day's real, full block vocabulary as observed across the `day`, `kan`, `cospan`, and `mingus` logs. Fix a `day-witness` rendering bug (a single-probe block like `{"command":"cargo test"}` renders `command: ?`), handle the untagged probe union, and add summaries for the two day block types cospan currently dumps as raw JSON: `day-docs` and `day-injection`. [validation: 9 check(s), 0 failed, 0 warning(s), 0 unchecked, 0 open question(s)]
***8<***
---
{
  "v": 3,
  "cid": "bafyreibjgouayr7cexwz33yzp4blywkafo6vrp7byctvt3pzeocxktolpu",
  "sig": "bfab8a980dac3c2f2008388a9b3622e5dfce489d20a9850db9a9d0f3e08e8be7297345e2ce8c25da68481eb7d8d9444521e8160bc54963dce727f18e8ed6e246",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "witness-and-block-vocabulary"
  },
  "kind": "subject",
  "cites": [],
  "rev": "223mtibjep7zo",
  "seq": 2,
  "of": 8,
  "content": "p2Rib2R5oWdTdWJqZWN0omV0aXRsZXg+RmVhdHVyZTogcm9idXN0IGRheS13aXRuZXNzIHJlbmRlcmluZyArIGRheS1kb2NzL2RheS1pbmplY3Rpb25sc3ViamVjdF9raW5kZElkZWFlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlU2RCYnJNNk5lWTRpOE1veHVGaXlEdHd4M0FISjhmakVkOTRnckgzMXI4WFllYWdlbnT2Z3N1YmplY3ShZUxvY2FseBx3aXRuZXNzLWFuZC1ibG9jay12b2NhYnVsYXJ5aWFydGlmYWN0c4GhZkNvbW1pdHgoY2U2NDhjOTYxZTQ1NmUwODliMDRjNjQ1YjNjMDYyOWY0YzZmMGQ2Y2l3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABllx3qqXjw=="
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreid6vlf7l2yh3mrc6zaal4pfwbatrem4oxriht6mpup5p55c2iacxa",
  "sig": "de8c449ccfb681b77059a3307cc478056f71cadc2f22c4103716e6a005e08b212e519087a742f0a082ad21962edde7e04a4fbd111d437ad948facfea70b0e490",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "witness-and-block-vocabulary"
  },
  "kind": "decision",
  "cites": [
    "bafyreihucwx2dqlg7es57qnp3635coob254q5r35jawajvhz6p5xhgpqci"
  ],
  "rev": "223mtibjew3s4",
  "seq": 3,
  "of": 8,
  "text_len": 286,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIg9BWvocFm+SXfwa/ft9E5wdd5Dsd9SCwE1Pnz+3OZ8BJmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx4HHdpdG5lc3MtYW5kLWJsb2NrLXZvY2FidWxhcnlpYXJ0aWZhY3RzgaFmQ29tbWl0eChjZTY0OGM5NjFlNDU2ZTA4OWIwNGM2NDViM2MwNjI5ZjRjNmYwZDZjaXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWXHergaX"
}
---

RQ-1: `day-witness` is disambiguated by shape-sniffing (top-level keys all in `PROBE_KINDS` → a lone probe; otherwise a name→probe map), because day's fence carries no discriminator. The ambiguity itself is reported upstream as a `day` repo issue rather than worked around silently.
***8<***
---
{
  "v": 3,
  "cid": "bafyreiaf6lgg5is7gm2adwizsh6rkrh4yashkp6lo7s7myywedmwyij2pa",
  "sig": "4d023f2164a5660eacebfab9bf3ab04704556709bbbc6bf1dc4798175c3bed0923f2f2c93a7a99da20598dbe27f5bbef69538b3d2c52a5b7db62880df25d9078",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "witness-and-block-vocabulary"
  },
  "kind": "decision",
  "cites": [
    "bafyreihucwx2dqlg7es57qnp3635coob254q5r35jawajvhz6p5xhgpqci"
  ],
  "rev": "223mtibjf55m5",
  "seq": 4,
  "of": 8,
  "text_len": 220,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIg9BWvocFm+SXfwa/ft9E5wdd5Dsd9SCwE1Pnz+3OZ8BJmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx4HHdpdG5lc3MtYW5kLWJsb2NrLXZvY2FidWxhcnlpYXJ0aWZhY3RzgaFmQ29tbWl0eChjZTY0OGM5NjFlNDU2ZTA4OWIwNGM2NDViM2MwNjI5ZjRjNmYwZDZjaXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWXHesY3X"
}
---

RQ-2: `day-docs`/`day-injection` are rendered generically (every field via `append_extra_keys`) rather than with bespoke layouts, so the view never falls behind day's evolving keys — matching the `day-schema` decision.
***8<***
---
{
  "v": 3,
  "cid": "bafyreibskcbq3a6riypiywj4og5r7y2rxmaysbcx6tqfug6zs6zwdkxpgu",
  "sig": "5c187ffd81c3529880be61561976ea8c798ee8fe933a5d706f428416175e1aaf15e2b75fc2588abd4158cfd744369218a69f257f4705237692632ff341194d54",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "witness-and-block-vocabulary"
  },
  "kind": "decision",
  "cites": [
    "bafyreihucwx2dqlg7es57qnp3635coob254q5r35jawajvhz6p5xhgpqci"
  ],
  "rev": "223mtibjfe6qk",
  "seq": 5,
  "of": 8,
  "text_len": 171,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIg9BWvocFm+SXfwa/ft9E5wdd5Dsd9SCwE1Pnz+3OZ8BJmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx4HHdpdG5lc3MtYW5kLWJsb2NrLXZvY2FidWxhcnlpYXJ0aWZhY3RzgaFmQ29tbWl0eChjZTY0OGM5NjFlNDU2ZTA4OWIwNGM2NDViM2MwNjI5ZjRjNmYwZDZjaXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWXHetRJm"
}
---

RQ-3: This is the reconciliation cycle sequenced after the schema-block summaries; the day-side format inconsistencies are handed off as `day` repo issues, not fixed here.
***8<***
---
{
  "v": 3,
  "cid": "bafyreicealr6wuus3eh2327xgzj7wqq4qcszyz3okvuagmjljgzdas5jtu",
  "sig": "75917bbf18cb8322473101f5a59714b5d87f3dcc82b50bf3477fc5196e9b3f5f1dda9bacd52b79898e0d3434ebbd6742421adcb1409de835a6c9c191a4f45260",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "witness-and-block-vocabulary"
  },
  "kind": "decision",
  "cites": [
    "bafyreihucwx2dqlg7es57qnp3635coob254q5r35jawajvhz6p5xhgpqci"
  ],
  "rev": "223mtibwh5sht",
  "seq": 6,
  "of": 8,
  "text_len": 1015,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIg9BWvocFm+SXfwa/ft9E5wdd5Dsd9SCwE1Pnz+3OZ8BJmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx4HHdpdG5lc3MtYW5kLWJsb2NrLXZvY2FidWxhcnlpYXJ0aWZhY3RzgaFmQ29tbWl0eChjZTY0OGM5NjFlNDU2ZTA4OWIwNGM2NDViM2MwNjI5ZjRjNmYwZDZjaXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWXH40dq0"
}
---

adversarial review of witness-and-block-vocabulary: SHIP. Independent Opus review extracted all 41 day-witness/day-docs/day-injection blocks across day, kan, cospan, mingus and ran the new logic against them: zero anomalies (no ?, no empty renders, no dangling key:, no (inline)). Confirmed the fix: lone {command:"cargo test"} -> 'command: cargo test' (was 'command: ?'); schema map -> 'name: <probe-kind>' with {material,record} -> 'material+record'; day-docs/day-injection render every field incl. numerics. is_probe collision (a witness literally named path/command/... ) does not occur in the corpus and is documented + filed upstream (kan-tools/day#237). Scope clean: other arms + None fallback untouched; tests assert exact content. One follow-up applied post-review: day added an 'every' probe kind (seen as a map value); added it to PROBE_KINDS + a lone-every test so a bare {every:{...}} probe names its kind rather than being mis-bucketed. Re-gate: 63 tests, clippy -D warnings, fmt clean. Verdict: SHIP.
***8<***
---
{
  "v": 3,
  "cid": "bafyreibynhn7ctriyxkfsputc33sti5zexk4jmovrpgcaoerbuefhsnzfe",
  "sig": "7edb5f75f731d951ebfaf6c6384b32e4a475cee56efa3d7b29875d5cd915a29318635af4e604d150005adacd342139162ad438cbc1d6ae090aedaded78a37bc3",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "witness-and-block-vocabulary"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223mtibwhfbwc",
  "seq": 7,
  "of": 8,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx4HHdpdG5lc3MtYW5kLWJsb2NrLXZvY2FidWxhcnlpYXJ0aWZhY3RzgaFmQ29tbWl0eChjZTY0OGM5NjFlNDU2ZTA4OWIwNGM2NDViM2MwNjI5ZjRjNmYwZDZjaXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWXH41Z8U"
}
---
