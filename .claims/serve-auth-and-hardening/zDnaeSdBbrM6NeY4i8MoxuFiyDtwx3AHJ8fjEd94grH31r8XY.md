---
{
  "v": 3,
  "cid": "bafyreieklpak5f6rag2ll46nobhxyodojwxpjrl5ex2h3trxazal2pkgoi",
  "sig": "c15359a7b19b0fe8d9db2bbac97481d1ebb7653b244790e248c12a203df9eba577c56e66c8407c5f722c111db1b105b9de01e3c747bb29ec40c1a264443b624c",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "serve-auth-and-hardening"
  },
  "kind": "observation",
  "cites": [],
  "rev": "223mualkvahwl",
  "seq": 0,
  "of": 6,
  "text_len": 199,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlU2RCYnJNNk5lWTRpOE1veHVGaXlEdHd4M0FISjhmakVkOTRnckgzMXI4WFllYWdlbnT2Z3N1YmplY3ShZUxvY2FseBhzZXJ2ZS1hdXRoLWFuZC1oYXJkZW5pbmdpYXJ0aWZhY3RzgaFmQ29tbWl0eChkNzdkZmZkMzkyYjFiYTY0Nzk2NGVlODE0ODE0ODlmZGUzYjczYTBhaXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWjRhszHj"
}
---

design doc .design/serve-auth-and-hardening.md checked against the live design-doc schema: validation: 9 check(s), 0 failed, 0 warning(s), 0 unchecked, 1 open question(s) [doc 10574:1f28f5d3e5ca9825]
***8<***
---
{
  "v": 3,
  "cid": "bafyreif4zclcqgqrf3kbogrgm5yhaajoqwnoxakq2raelnqcxpi5q7fhge",
  "sig": "c0bd4a42124a85c3c6aef9890d4468f65701a27723915d72942248a9ef7c94c206a6e5ee399b0aecfdab6a825f591506bdf975a9285320d6d09d104df398bbd2",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "serve-auth-and-hardening"
  },
  "kind": "plan",
  "cites": [
    "bafyreieklpak5f6rag2ll46nobhxyodojwxpjrl5ex2h3trxazal2pkgoi"
  ],
  "rev": "223mualkvncni",
  "seq": 1,
  "of": 6,
  "text_len": 911,
  "content": "p2Rib2R5oWRQbGFuoWR0ZXh0YGVjaXRlc4HYKlglAAFxEiCKW8Cul9EBtLXzzXBPfDhuTa70xX0l9H3ONwZAvT1GcmZhdXRob3KiY2RpZHg5ZGlkOmtleTp6RG5hZVNkQmJyTTZOZVk0aThNb3h1Rml5RHR3eDNBSEo4ZmpFZDk0Z3JIMzFyOFhZZWFnZW509mdzdWJqZWN0oWVMb2NhbHgYc2VydmUtYXV0aC1hbmQtaGFyZGVuaW5naWFydGlmYWN0c4GhZkNvbW1pdHgoZDc3ZGZmZDM5MmIxYmE2NDc5NjRlZTgxNDgxNDg5ZmRlM2I3M2EwYWl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABlo0Ybmh+A=="
}
---

serve-auth-and-hardening design (.design/serve-auth-and-hardening.md): Now that `tailscale serve` makes `cospan serve` reachable from a phone, the read API needs to be safe to expose. Add app-level authentication (a generated bearer token gating every route, including the `/stream` WebSocket upgrade) and close the two hardening follow-ups recorded on `mobile-api-server`: a symlink-resolving path guard so `/comments`/`/thread` cannot escape the repo via an in-repo symlink, and a configurable cap on concurrent `/stream` connections. This is the honest prerequisite for Slice C (comment *writes* over the API): authenticate and harden the observe channel before anything crosses it to write (`telos/observe-now-control-later`), while keeping `serve` a throwaway localhost-first process with no user store (`telos/disposable`). [validation: 9 check(s), 0 failed, 0 warning(s), 0 unchecked, 1 open question(s)]
***8<***
---
{
  "v": 3,
  "cid": "bafyreifbkiskxd5gcv6mskdbah2sbgvpq4votfrxtrj25nbv2qwyfqycdm",
  "sig": "964e8a4fc187f7412e6f3c719e615763e6be43813da74a6c314b3733d7e0277b7b3c470b2f84ba9031006340b1531d91d216e0662a4abbd27af3825f8d4ab1d9",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "serve-auth-and-hardening"
  },
  "kind": "subject",
  "cites": [],
  "rev": "223mualkvrhzj",
  "seq": 2,
  "of": 6,
  "content": "p2Rib2R5oWdTdWJqZWN0omV0aXRsZXhBRmVhdHVyZTogYGNvc3BhbiBzZXJ2ZWAgYXV0aCArIHJlbW90ZS1jaGFubmVsIGhhcmRlbmluZyAoU2xpY2UgQilsc3ViamVjdF9raW5kZElkZWFlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlU2RCYnJNNk5lWTRpOE1veHVGaXlEdHd4M0FISjhmakVkOTRnckgzMXI4WFllYWdlbnT2Z3N1YmplY3ShZUxvY2FseBhzZXJ2ZS1hdXRoLWFuZC1oYXJkZW5pbmdpYXJ0aWZhY3RzgaFmQ29tbWl0eChkNzdkZmZkMzkyYjFiYTY0Nzk2NGVlODE0ODE0ODlmZGUzYjczYTBhaXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWjRhu7d7"
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreia7v4eae4ekbpgwq3mmiw6hu6pjfpaqpwh74lycwzl7gfnyrrgzha",
  "sig": "318ef1a66ef3b241b2982e5de90a01fc876727010e816b760246b094edd83b1238234f2550668f110e2feafed82818f17a73020c3db0cdddf2b95ddb214945a9",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "serve-auth-and-hardening"
  },
  "kind": "decision",
  "cites": [
    "bafyreif4zclcqgqrf3kbogrgm5yhaajoqwnoxakq2raelnqcxpi5q7fhge"
  ],
  "rev": "223muanm4idjo",
  "seq": 3,
  "of": 6,
  "text_len": 481,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgvMiWKBoRLtQXGiZncHABLoWa64FQ1EBFtgK70dh8pzFmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GHNlcnZlLWF1dGgtYW5kLWhhcmRlbmluZ2lhcnRpZmFjdHOBoWZDb21taXR4KGQ3N2RmZmQzOTJiMWJhNjQ3OTY0ZWU4MTQ4MTQ4OWZkZTNiNzNhMGFpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZaNOQnJYA="
}
---

adversarial review of serve-auth-and-hardening: APPROVE-WITH-FOLLOW-UPS — Independent cold Opus review: auth gate held against an extensive live bypass matrix (every route gated pre-routing incl. WS before 101, constant-time compare correct, symlink guard rejects real escapes incl. symlinked repo root, cap enforces+releases under a flood, nothing touches disk/command_bus). Found one latent fail-open (empty-token) + a Bearer-precedence footgun; both fixed in-round with tests.
***8<***
---
{
  "v": 3,
  "cid": "bafyreibipw6m4ksqvt5fztltensrqfxun46cfzqfmwtk3xd7ba4pgpywsy",
  "sig": "1c94de426e1bf36e7d653f9d32923535f5586caffa961aca362e10bab8af49b914613a4a47eec54f23afc335beed476cc7f0131aa8d119bf81f5abbd224368fd",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "serve-auth-and-hardening"
  },
  "kind": "result",
  "cites": [
    "bafyreia7v4eae4ekbpgwq3mmiw6hu6pjfpaqpwh74lycwzl7gfnyrrgzha"
  ],
  "rev": "223muanmeec4j",
  "seq": 4,
  "of": 6,
  "text_len": 703,
  "content": "p2Rib2R5oWZSZXN1bHShZHRleHRgZWNpdGVzgdgqWCUAAXESIB+vCAJwigvNaG2MRbx6eekrwQfY/+LwK2V/MVuIxNk4ZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlU2RCYnJNNk5lWTRpOE1veHVGaXlEdHd4M0FISjhmakVkOTRnckgzMXI4WFllYWdlbnT2Z3N1YmplY3ShZUxvY2FseBhzZXJ2ZS1hdXRoLWFuZC1oYXJkZW5pbmdpYXJ0aWZhY3RzgaFmQ29tbWl0eChkNzdkZmZkMzkyYjFiYTY0Nzk2NGVlODE0ODE0ODlmZGUzYjczYTBhaXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWjTkpR/b"
}
---

Fixed in-round (both flagged fix-before-Slice-C): (1) fail-open — mint_token returned "" on OS-RNG failure and ct_eq("","")=true would authenticate an empty ?token=; now require_auth rejects any empty configured token and run() refuses to serve with a fatal error. (2) Bearer-precedence footgun — a wrong Authorization header masked a correct ?token=; presented_tokens now collects both header and query and authenticates if ANY matches. Tests added: empty_configured_token_authenticates_nothing, wrong_bearer_does_not_mask_a_correct_query_token. Accepted/negligible (not fixed): query-token %-decode absence (fails closed, hex tokens unaffected) and guard TOCTOU (single-user disposable localhost).
***8<***
---
{
  "v": 3,
  "cid": "bafyreidohxgrgo5lzawa6jnow7uq2bvy22237gjhkfat53on233y5tb4pq",
  "sig": "008fa107bb71477601ea447ef2307e538e83376befd8bccb30a1f49c7eb3a6d86b4169c64a502ea1f431251e4f884425a8d13e5bce23494f8b08d8bb5fb0d67a",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "serve-auth-and-hardening"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223muanurumda",
  "seq": 5,
  "of": 6,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GHNlcnZlLWF1dGgtYW5kLWhhcmRlbmluZ2lhcnRpZmFjdHOBoWZDb21taXR4KGMwZmNiNmNlMDhiZDlmZDZjZWNmYzliMTFmNDM1MGMxMjA1NGYxMDlpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZaNPV9SLI="
}
---
