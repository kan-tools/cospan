---
{
  "v": 3,
  "cid": "bafyreiatjb7looziek5s2rtokq7rox3z7tqrpq5pbaku3bmjq4kbi4bfii",
  "sig": "e10f0313a1a3ffb3d79eee75d294abcd47b6590570d7148bf890ffa23c2cb2b7031829120b4d376ef9538e4d95200f3a320827606d85da2778a6a09e7df62323",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "telos/comment-roundtrip"
  },
  "kind": "decision",
  "cites": [],
  "rev": "223mtgucqko7c",
  "seq": 0,
  "of": 6,
  "text_len": 266,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlU2RCYnJNNk5lWTRpOE1veHVGaXlEdHd4M0FISjhmakVkOTRnckgzMXI4WFllYWdlbnT2Z3N1YmplY3ShZUxvY2Fsd3RlbG9zL2NvbW1lbnQtcm91bmR0cmlwaWFydGlmYWN0c4GhZkNvbW1pdHgoNzJhOTg3YjI0OWVlMmI2ZmZkMTgzZDcwMjgxNzY5MWZiMjJmNjg4MWl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAODM1Zjg3YWRlNDBiZGM4ZGYyMmMzYmJkMjc4ZDU3ZTA0MGZhYTA4Y2M2ZTQxNWQ2YmQwMzg0ZWI5YTdlNTNjZWtyZWNvcmRlZF9hdBsABllmkWhQMg=="
}
---

A comment dropped in a cospan sidecar re-localizes as its file changes — the headless doc-comment round trip works, resolving Anchored / Drifted / Unresolvable honestly and never guessing at a lost anchor.

```day-telos
{"witnesses":["code-change","verdict"]}
```

***8<***
---
{
  "v": 3,
  "cid": "bafyreicqqbvpu3w444gjzmp4ahcjzzhjokml4qfos4jp4ze6i7fyejqtma",
  "sig": "113fc1726c586b0afd225b88aab746451e8406c2382f4c1861a7e4bab00d033400c739d6fe5fbf66cdf940342fb4dbb4037c21338457047f6a4c0747bad89097",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "telos/comment-roundtrip"
  },
  "kind": "subject",
  "cites": [],
  "rev": "223mtgucqmc3i",
  "seq": 1,
  "of": 6,
  "content": "p2Rib2R5oWdTdWJqZWN0omV0aXRsZXgeUDE6IHRoZSBkb2MtY29tbWVudCByb3VuZCB0cmlwbHN1YmplY3Rfa2luZGRJZGVhZWNpdGVzgGZhdXRob3KiY2RpZHg5ZGlkOmtleTp6RG5hZVNkQmJyTTZOZVk0aThNb3h1Rml5RHR3eDNBSEo4ZmpFZDk0Z3JIMzFyOFhZZWFnZW509mdzdWJqZWN0oWVMb2NhbHd0ZWxvcy9jb21tZW50LXJvdW5kdHJpcGlhcnRpZmFjdHOBoWZDb21taXR4KDcyYTk4N2IyNDllZTJiNmZmZDE4M2Q3MDI4MTc2OTFmYjIyZjY4ODFpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDgzNWY4N2FkZTQwYmRjOGRmMjJjM2JiZDI3OGQ1N2UwNDBmYWEwOGNjNmU0MTVkNmJkMDM4NGViOWE3ZTUzY2VrcmVjb3JkZWRfYXQbAAZZZpFpH7o="
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreigd3hqvcyqzqapemaometlhwxhkyz6ehmssynl7a3z254xdls4xei",
  "sig": "cb83051901c8fc6652d0d8333581e9f05889384c7adc467e9fd5f2093d274fd25483088833e6999a18477ece72df372df12a7791c72177129a03e4b4ea50ccc6",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "telos/comment-roundtrip"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223mtgucre5ty",
  "seq": 2,
  "of": 6,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx3dGVsb3MvY29tbWVudC1yb3VuZHRyaXBpYXJ0aWZhY3RzgaFmQ29tbWl0eCg3MmE5ODdiMjQ5ZWUyYjZmZmQxODNkNzAyODE3NjkxZmIyMmY2ODgxaXdvcmtzcGFjZaFpV29ya3NwYWNleEA4MzVmODdhZGU0MGJkYzhkZjIyYzNiYmQyNzhkNTdlMDQwZmFhMDhjYzZlNDE1ZDZiZDAzODRlYjlhN2U1M2Nla3JlY29yZGVkX2F0GwAGWWaRdQ7P"
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreiendobp3atl7xpc32bf36vqtg44xs2b3elz5b722wph3schbgc2gi",
  "sig": "f5895e4b27d68475a4b805e746478b48c7f918fbe0e46a96bdc802bf0e1361a21ede78c7ff199e636a7a537a1ac8c030e01ec6a3ed0ad80ff6f9f9d2b6711899",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "telos/comment-roundtrip"
  },
  "kind": "decision",
  "cites": [
    "bafyreigd3hqvcyqzqapemaometlhwxhkyz6ehmssynl7a3z254xdls4xei"
  ],
  "rev": "223mtp2iij2nm",
  "seq": 3,
  "of": 6,
  "text_len": 615,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgw9nhUWIZgB5GAcwk1ntc6sZ8Q7JSw1fwbzrvLjXLlyJmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx3dGVsb3MvY29tbWVudC1yb3VuZHRyaXBpYXJ0aWZhY3RzgaFmQ29tbWl0eChjNmIyZDI4ODY4NTlhOGI3MWY2YTQyODIzODMzZjhlYzViNWUxYjA4aXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWagc54B3"
}
---

The doc-comment round trip is fully writable and agent-reachable: a human browses any file with syntax highlighting and adds, replies, edits, and resolves comments in the TUI; comments re-localize as files change, resolving Anchored / Drifted / Unresolvable honestly and never guessing at a lost anchor; a comment or a file's whole set promotes to a durable kan claim on an explicit human action; and agents read and write the same comments over cospan mcp. The headless re-localization round trip remains the substrate this builds on.

```day-telos
{"witnesses":["code-change","verdict","published-artifact"]}
```

***8<***
---
{
  "v": 3,
  "cid": "bafyreialazqwdbzset4ni3oqbuj5cgxnokl75zatrpwbiori6yxrajxpdy",
  "sig": "3dcb886c99da2d997c963ac9e4e82f0467cdb7d50da969b1746f5a27401245e54db133c2a0037d7fe4054a375b836d166eded83d4b36675906917662c5706c56",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "telos/comment-roundtrip"
  },
  "kind": "decision",
  "cites": [
    "bafyreiendobp3atl7xpc32bf36vqtg44xs2b3elz5b722wph3schbgc2gi"
  ],
  "rev": "223mtphya7vej",
  "seq": 4,
  "of": 6,
  "text_len": 684,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgjRuC/YJr/d4t6CXfqwmbnLy0HZF56H+tWefchHCYWjJmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx3dGVsb3MvY29tbWVudC1yb3VuZHRyaXBpYXJ0aWZhY3RzgaFmQ29tbWl0eChjNmIyZDI4ODY4NTlhOGI3MWY2YTQyODIzODMzZjhlYzViNWUxYjA4aXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWat8YuvT"
}
---

The doc-comment round trip is fully writable and agent-reachable: a human browses any file with syntax highlighting and adds, replies, edits, and resolves comments in the TUI; comments re-localize as files change, resolving Anchored / Drifted / Unresolvable honestly and never guessing at a lost anchor; a comment or a file's whole set promotes to a durable kan claim on an explicit human action; and agents read and write the same comments over cospan mcp. The headless re-localization round trip remains the substrate this builds on.

```day-telos
{"witnesses":["verdict","code-change"],"scope":{"code-change":"src/mcp.rs","verdict":"adversarial review of comment-authoring*"}}
```

***8<***
---
{
  "v": 3,
  "cid": "bafyreibb6uo2u6iojukn2x4ga6bqgnxg4uxwm6tza2qvpfvno6ja2kwyua",
  "sig": "d6899f1857329df1291956b795dc691b7a0a856df49bf3a68a5cb2258038769437ba796df54a3140a6b89b226672c7c77aa141e4a1376f6fc8cd9c9471ec7d72",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "telos/comment-roundtrip"
  },
  "kind": "decision",
  "cites": [
    "bafyreialazqwdbzset4ni3oqbuj5cgxnokl75zatrpwbiori6yxrajxpdy"
  ],
  "rev": "223mtpi2kqsdg",
  "seq": 5,
  "of": 6,
  "text_len": 621,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgCwZhYYcyJPjUbdANE9Ea7XKX/uQTi+wUOij2LxAm7x5mYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx3dGVsb3MvY29tbWVudC1yb3VuZHRyaXBpYXJ0aWZhY3RzgaFmQ29tbWl0eChjNmIyZDI4ODY4NTlhOGI3MWY2YTQyODIzODMzZjhlYzViNWUxYjA4aXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWauBC2C7"
}
---

The doc-comment round trip is fully writable and agent-reachable: a human browses any file with syntax highlighting and adds, replies, edits, and resolves comments in the TUI; comments re-localize as files change, resolving Anchored / Drifted / Unresolvable honestly and never guessing at a lost anchor; a comment or a file's whole set promotes to a durable kan claim on an explicit human action; and agents read and write the same comments over cospan mcp. The headless re-localization round trip remains the substrate this builds on.

```day-telos
{"witnesses":["code-change"],"scope":{"code-change":"src/mcp.rs"}}
```

