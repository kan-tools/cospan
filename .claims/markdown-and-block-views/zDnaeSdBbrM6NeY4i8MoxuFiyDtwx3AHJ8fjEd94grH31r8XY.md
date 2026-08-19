---
{
  "v": 3,
  "cid": "bafyreibp6gyevxrfhp2mpofsnairvoybk5cjhjqcaszv7g7mh6yhpxrvga",
  "sig": "304594ee4ea5baafad6b54dfa65a02a539894315c9230b607d3ce23fc66067e35b032e3e6dca70351f609fe73344b07af609f0f4dd6d89ab6de2acc3ea787b01",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "markdown-and-block-views"
  },
  "kind": "observation",
  "cites": [],
  "rev": "223mth4by7sog",
  "seq": 0,
  "of": 6,
  "text_len": 199,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlU2RCYnJNNk5lWTRpOE1veHVGaXlEdHd4M0FISjhmakVkOTRnckgzMXI4WFllYWdlbnT2Z3N1YmplY3ShZUxvY2FseBhtYXJrZG93bi1hbmQtYmxvY2stdmlld3NpYXJ0aWZhY3RzgaFmQ29tbWl0eChmNWFhNmJiYzFkZDU4MmRkYWE4ZGNmYjM1NjA1NmNiY2M0NWM0Nzg1aXdvcmtzcGFjZaFpV29ya3NwYWNleEA4MzVmODdhZGU0MGJkYzhkZjIyYzNiYmQyNzhkNTdlMDQwZmFhMDhjYzZlNDE1ZDZiZDAzODRlYjlhN2U1M2Nla3JlY29yZGVkX2F0GwAGWWiP4uIT"
}
---

design doc .design/markdown-and-block-views.md checked against the live design-doc schema: validation: 10 check(s), 0 failed, 2 warning(s), 0 unchecked, 0 open question(s) [doc 5849:6ef797ac56e8b858]
***8<***
---
{
  "v": 3,
  "cid": "bafyreifnkyfbqfsygbkrycsxloypjwvdnqif6l5vst6egjkg6rflhvpz7a",
  "sig": "1512786fdfed5946dd207f10f2e8d20d35c6330001a7484c83f760ee27c251867b34ecd9b2d1571d38b40160c40376c0797ade1e303440874661ed321b5ea754",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "markdown-and-block-views"
  },
  "kind": "plan",
  "cites": [
    "bafyreibp6gyevxrfhp2mpofsnairvoybk5cjhjqcaszv7g7mh6yhpxrvga"
  ],
  "rev": "223mth4byfjht",
  "seq": 1,
  "of": 6,
  "text_len": 546,
  "content": "p2Rib2R5oWRQbGFuoWR0ZXh0YGVjaXRlc4HYKlglAAFxEiAv8bBK3iU79Me4smgRGrsBV0STpgIEs1+b7D+wd941MGZhdXRob3KiY2RpZHg5ZGlkOmtleTp6RG5hZVNkQmJyTTZOZVk0aThNb3h1Rml5RHR3eDNBSEo4ZmpFZDk0Z3JIMzFyOFhZZWFnZW509mdzdWJqZWN0oWVMb2NhbHgYbWFya2Rvd24tYW5kLWJsb2NrLXZpZXdzaWFydGlmYWN0c4GhZkNvbW1pdHgoZjVhYTZiYmMxZGQ1ODJkZGFhOGRjZmIzNTYwNTZjYmNjNDVjNDc4NWl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAODM1Zjg3YWRlNDBiZGM4ZGYyMmMzYmJkMjc4ZDU3ZTA0MGZhYTA4Y2M2ZTQxNWQ2YmQwMzg0ZWI5YTdlNTNjZWtyZWNvcmRlZF9hdBsABlloj+W9SA=="
}
---

markdown-and-block-views design (.design/markdown-and-block-views.md): Render a claim's body as markdown in the detail pane — wrapped, with headers, emphasis, lists, inline code, and code fences styled — instead of raw text. And give the fenced structured blocks claims carry a readable view: a human summary for the supported `day-*` / `cospan-comment` types, and a code-formatted markdown view for unsupported ones, so nothing renders as an unreadable wall. [validation: 10 check(s), 0 failed, 2 warning(s), 0 unchecked, 0 open question(s)]
***8<***
---
{
  "v": 3,
  "cid": "bafyreicqltga67sy2cxgfwmfttmkgfczexjgq7nyzce6ca44wnto4mslpq",
  "sig": "6dbff91224e82289e884a3ed918e4d0cc7e1586b3ef7623ca33596ba8ea75f92685a5f708bf0aa26c15f4f126eaa93b8f65287643f07192acc4da56c608bb571",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "markdown-and-block-views"
  },
  "kind": "subject",
  "cites": [],
  "rev": "223mth4byhj7s",
  "seq": 2,
  "of": 6,
  "content": "p2Rib2R5oWdTdWJqZWN0omV0aXRsZXg3RmVhdHVyZTogTWFya2Rvd24gY2xhaW0gYm9kaWVzICsgc3RydWN0dXJlZC1ibG9jayB2aWV3c2xzdWJqZWN0X2tpbmRkSWRlYWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GG1hcmtkb3duLWFuZC1ibG9jay12aWV3c2lhcnRpZmFjdHOBoWZDb21taXR4KGY1YWE2YmJjMWRkNTgyZGRhYThkY2ZiMzU2MDU2Y2JjYzQ1YzQ3ODVpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDgzNWY4N2FkZTQwYmRjOGRmMjJjM2JiZDI3OGQ1N2UwNDBmYWEwOGNjNmU0MTVkNmJkMDM4NGViOWE3ZTUzY2VrcmVjb3JkZWRfYXQbAAZZaI/mvFg="
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreigkf6y64rml2k4w7gwkxosp7uxpkau2tugosip6ckdcfklrpx46jm",
  "sig": "c950bceb889b6f49ff49a7ee2953515a6648b58711514e95cdba3b7206b44d4e1e61ffcfd4ce5bbf86e3718276fa3944af728341ed948f66f5f60d08b7eea0a8",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "markdown-and-block-views"
  },
  "kind": "decision",
  "cites": [
    "bafyreifnkyfbqfsygbkrycsxloypjwvdnqif6l5vst6egjkg6rflhvpz7a"
  ],
  "rev": "223mth4byn6us",
  "seq": 3,
  "of": 6,
  "text_len": 201,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgrVYKGBZYMFUcCldbsPTao2wQXy+1lPxDJUb0SrPV+fhmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GG1hcmtkb3duLWFuZC1ibG9jay12aWV3c2lhcnRpZmFjdHOBoWZDb21taXR4KGY1YWE2YmJjMWRkNTgyZGRhYThkY2ZiMzU2MDU2Y2JjYzQ1YzQ3ODVpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDgzNWY4N2FkZTQwYmRjOGRmMjJjM2JiZDI3OGQ1N2UwNDBmYWEwOGNjNmU0MTVkNmJkMDM4NGViOWE3ZTUzY2VrcmVjb3JkZWRfYXQbAAZZaI/pku0="
}
---

RQ-1: Markdown is parsed with `pulldown-cmark` (already vendored) rather than a hand-rolled parser, and rendered to styled `ratatui` lines — headers/emphasis/ code/lists — with pane-width wrapping.
***8<***
---
{
  "v": 3,
  "cid": "bafyreidlmpsir5rrajbiwywzjrzmgiqi24hguvmexklej7gfioyc3mdk3q",
  "sig": "2b8eb459741441242009140e5d94f909a1808902fd51c7861c355040d3f3864112a8cd9835d2e4ba4ab4d968f8b5e97cd65de3baf6303fbce356cce7063977c6",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "markdown-and-block-views"
  },
  "kind": "decision",
  "cites": [
    "bafyreifnkyfbqfsygbkrycsxloypjwvdnqif6l5vst6egjkg6rflhvpz7a"
  ],
  "rev": "223mth4bysuae",
  "seq": 4,
  "of": 6,
  "text_len": 169,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgrVYKGBZYMFUcCldbsPTao2wQXy+1lPxDJUb0SrPV+fhmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GG1hcmtkb3duLWFuZC1ibG9jay12aWV3c2lhcnRpZmFjdHOBoWZDb21taXR4KGY1YWE2YmJjMWRkNTgyZGRhYThkY2ZiMzU2MDU2Y2JjYzQ1YzQ3ODVpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDgzNWY4N2FkZTQwYmRjOGRmMjJjM2JiZDI3OGQ1N2UwNDBmYWEwOGNjNmU0MTVkNmJkMDM4NGViOWE3ZTUzY2VrcmVjb3JkZWRfYXQbAAZZaI/saF4="
}
---

RQ-2: Supported `day-*`/`cospan-comment` blocks get a human summary; every other fenced block is shown as labeled code, never hidden or dumped as raw JSON in prose flow.
***8<***
---
{
  "v": 3,
  "cid": "bafyreicor6yebk2so3rzkcztat2pn36nmkt2zc2jtotz4nfbjcg5ey76qm",
  "sig": "dc2c4f601c1e1928d0d5182ee41949cae6546a1ecefaea740a7a66afcd3c97b908e83e92b351de735f2f8862e25beacbb0ed2e3044e0a4af78a84cc611c08df8",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "markdown-and-block-views"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223mthbc3sbth",
  "seq": 5,
  "of": 6,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GG1hcmtkb3duLWFuZC1ibG9jay12aWV3c2lhcnRpZmFjdHOBoWZDb21taXR4KGY1YWE2YmJjMWRkNTgyZGRhYThkY2ZiMzU2MDU2Y2JjYzQ1YzQ3ODVpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDgzNWY4N2FkZTQwYmRjOGRmMjJjM2JiZDI3OGQ1N2UwNDBmYWEwOGNjNmU0MTVkNmJkMDM4NGViOWE3ZTUzY2VrcmVjb3JkZWRfYXQbAAZZadAcHsA="
}
---
