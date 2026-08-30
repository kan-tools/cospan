---
{
  "v": 3,
  "cid": "bafyreid5yfpkxyllr2dsbhd7nvd3j4ngzusd2kd7ohxq2vtag57cvgbuky",
  "sig": "391ebac7c586f2c36144a10360486146ff50aa2e3d5d559232674d23c06e457414d48e019577794c133422df78fa3f74e9fdaea4451ba2e6f2c6a3d8747f7e7f",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comment-writes-over-api"
  },
  "kind": "observation",
  "cites": [],
  "rev": "223mud6uff4ns",
  "seq": 0,
  "of": 9,
  "text_len": 197,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlU2RCYnJNNk5lWTRpOE1veHVGaXlEdHd4M0FISjhmakVkOTRnckgzMXI4WFllYWdlbnT2Z3N1YmplY3ShZUxvY2Fsd2NvbW1lbnQtd3JpdGVzLW92ZXItYXBpaWFydGlmYWN0c4GhZkNvbW1pdHgoZjNhYTUzOTNlOTcyMmNkYWNkNzk3ZTZkNzVkZjkzZDNlZmEyNWEyMml3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABlpJNLWGWw=="
}
---

design doc .design/comment-writes-over-api.md checked against the live design-doc schema: validation: 9 check(s), 0 failed, 0 warning(s), 0 unchecked, 0 open question(s) [doc 9037:b701abb2af3ef5da]
***8<***
---
{
  "v": 3,
  "cid": "bafyreicjmtmv4hbg7h7vrnfw7kerbnwvvu3qclmck6nf6z6ikk3entgk4i",
  "sig": "c1b869f92ff80821c217a0492a0bafec1a5574836871b3916a193c291a7a39366f4e04c717fc6c1627e42e3b4132a7c4d85797bd00cebb97b784b6c6b0f75419",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comment-writes-over-api"
  },
  "kind": "plan",
  "cites": [
    "bafyreid5yfpkxyllr2dsbhd7nvd3j4ngzusd2kd7ohxq2vtag57cvgbuky"
  ],
  "rev": "223mud6ufrqpv",
  "seq": 1,
  "of": 9,
  "text_len": 655,
  "content": "p2Rib2R5oWRQbGFuoWR0ZXh0YGVjaXRlc4HYKlglAAFxEiB9wV6r4WuOhyCcf21HtPGmzSQ9KH9x7w1WYDd+Kpg0VmZhdXRob3KiY2RpZHg5ZGlkOmtleTp6RG5hZVNkQmJyTTZOZVk0aThNb3h1Rml5RHR3eDNBSEo4ZmpFZDk0Z3JIMzFyOFhZZWFnZW509mdzdWJqZWN0oWVMb2NhbHdjb21tZW50LXdyaXRlcy1vdmVyLWFwaWlhcnRpZmFjdHOBoWZDb21taXR4KGYzYWE1MzkzZTk3MjJjZGFjZDc5N2U2ZDc1ZGY5M2QzZWZhMjVhMjJpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZaSTS72k0="
}
---

comment-writes-over-api design (.design/comment-writes-over-api.md): The first *write* over `cospan serve`: add / reply / resolve comments from the phone, opt-in behind `--allow-writes`. Writes go only to cospan's owned sidecar state (`.cospan/comments/`, the `telos/kan-is-truth` exception) — never to kan and never through the agent-control command bus — so this crosses `observe → control` by the mildest step available, reusing the tested S5 write cores. A `GET /capabilities` probe and minimal write affordances in the Comments tab make it usable from the phone. [validation: 9 check(s), 0 failed, 0 warning(s), 0 unchecked, 0 open question(s)]
***8<***
---
{
  "v": 3,
  "cid": "bafyreihbvmuphbcitbd76jdjpjhzyfzix5kkhkzeadipu3co7twvvhf6ka",
  "sig": "d522f696938d726595bf9d087783f4600bf7e62ad7d0bdfdcf2b483e1c6f806f750b099a547b0c12d823f4a56df2182cc85955367b3231b50235129711ddb6bf",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comment-writes-over-api"
  },
  "kind": "subject",
  "cites": [],
  "rev": "223mud6ufvudi",
  "seq": 2,
  "of": 9,
  "content": "p2Rib2R5oWdTdWJqZWN0omV0aXRsZXhGRmVhdHVyZTogQ29tbWVudCB3cml0ZXMgb3ZlciB0aGUgQVBJIChTbGljZSBDIOKAlCBvYnNlcnZlIOKGkiBjb250cm9sKWxzdWJqZWN0X2tpbmRkSWRlYWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx3Y29tbWVudC13cml0ZXMtb3Zlci1hcGlpYXJ0aWZhY3RzgaFmQ29tbWl0eChmM2FhNTM5M2U5NzIyY2RhY2Q3OTdlNmQ3NWRmOTNkM2VmYTI1YTIyaXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWkk0vejG"
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreigbct44gw55dujn5vqkucbamwppnc2rvou4cb334p2q6abkwl2oky",
  "sig": "f6bc903173e37e98c8a1c5bcb17dfce44b89237b2fffbd7e2db0cceb15fd2f2d072bce240f42a81510232a6aefe8192667d03aff8cb288c5f17241758fbf6ac7",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comment-writes-over-api"
  },
  "kind": "decision",
  "cites": [
    "bafyreicjmtmv4hbg7h7vrnfw7kerbnwvvu3qclmck6nf6z6ikk3entgk4i"
  ],
  "rev": "223mud6ugce65",
  "seq": 3,
  "of": 9,
  "text_len": 237,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgSWTZXhwm+f9YtLb6iRC21a03AS2CV5pfZ8hStkbMyuJmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx3Y29tbWVudC13cml0ZXMtb3Zlci1hcGlpYXJ0aWZhY3RzgaFmQ29tbWl0eChmM2FhNTM5M2U5NzIyY2RhY2Q3OTdlNmQ3NWRmOTNkM2VmYTI1YTIyaXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWkk0xCgV"
}
---

RQ-1: Writes are opt-in via `--allow-writes` (off by default), so `serve` stays read-only unless control is explicitly enabled — the `telos/observe-now-control-later` ordering, made a runtime switch rather than an always-on capability.
***8<***
---
{
  "v": 3,
  "cid": "bafyreifmaeghcjtaksajajnbozpsgpybxym24mfqrcmg2iumf35y4lk7lm",
  "sig": "c5afeaf5a819fa51097731ae42eae20d61e2c1f929fd1b81194d4508b42f55222e3e71e2c5c7d7e95ab159545901f6e02516b0d7e35fd643fa091529a353abe1",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comment-writes-over-api"
  },
  "kind": "decision",
  "cites": [
    "bafyreicjmtmv4hbg7h7vrnfw7kerbnwvvu3qclmck6nf6z6ikk3entgk4i"
  ],
  "rev": "223mud6ugowqt",
  "seq": 4,
  "of": 9,
  "text_len": 250,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgSWTZXhwm+f9YtLb6iRC21a03AS2CV5pfZ8hStkbMyuJmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx3Y29tbWVudC13cml0ZXMtb3Zlci1hcGlpYXJ0aWZhY3RzgaFmQ29tbWl0eChmM2FhNTM5M2U5NzIyY2RhY2Q3OTdlNmQ3NWRmOTNkM2VmYTI1YTIyaXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWkk0ynJs"
}
---

RQ-2: Web writes are attributed `who:"human"` with a configurable id (default `"web"`), not reused `agent_author()` — a human operator's comment is not mislabeled as an agent's. The cores are parameterized so the MCP agent path keeps `who:"agent"`.
***8<***
---
{
  "v": 3,
  "cid": "bafyreiehwof7ma2id7bzzhm6zijx7yjucihyogwefg6hpsoyyadt3bub2e",
  "sig": "8a6e5d84e42587f9542b40e54d038c0e2d9585bff7b198cedd40ad3b22b7b2732f3c80ae1ec10d774454fe0553b12f8baf64b38725292d61b393be6a0be2ea0a",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comment-writes-over-api"
  },
  "kind": "decision",
  "cites": [
    "bafyreicjmtmv4hbg7h7vrnfw7kerbnwvvu3qclmck6nf6z6ikk3entgk4i"
  ],
  "rev": "223mud6uh3ggf",
  "seq": 5,
  "of": 9,
  "text_len": 257,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgSWTZXhwm+f9YtLb6iRC21a03AS2CV5pfZ8hStkbMyuJmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx3Y29tbWVudC13cml0ZXMtb3Zlci1hcGlpYXJ0aWZhY3RzgaFmQ29tbWl0eChmM2FhNTM5M2U5NzIyY2RhY2Q3OTdlNmQ3NWRmOTNkM2VmYTI1YTIyaXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWkk00LEc"
}
---

RQ-3: Three POST endpoints (`/comments`, `/thread`, `/resolve`) mirroring the three S5 write tools 1:1, each beside its GET, rather than a PATCH state-change — the simplest shape consistent with the existing endpoints and the resource grammar Slice A set.
***8<***
---
{
  "v": 3,
  "cid": "bafyreico3w6sgatm3lkznqx74dk7lhhhazjcnykii465ja6eq47lgpypj4",
  "sig": "e470a1d9faa29b30d73e60c685e384ad9fa2674ece1b504d432a2a5a797ae02627d742ebdcaa715959e09810a2faf023c2a9e2dd999827863a8c716bedb9749e",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comment-writes-over-api"
  },
  "kind": "decision",
  "cites": [
    "bafyreicjmtmv4hbg7h7vrnfw7kerbnwvvu3qclmck6nf6z6ikk3entgk4i"
  ],
  "rev": "223mud7i76xlr",
  "seq": 6,
  "of": 9,
  "text_len": 673,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgSWTZXhwm+f9YtLb6iRC21a03AS2CV5pfZ8hStkbMyuJmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx3Y29tbWVudC13cml0ZXMtb3Zlci1hcGlpYXJ0aWZhY3RzgaFmQ29tbWl0eChmM2FhNTM5M2U5NzIyY2RhY2Q3OTdlNmQ3NWRmOTNkM2VmYTI1YTIyaXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWklcUnXB"
}
---

adversarial review of comment-writes-over-api: APPROVE — Independent cold Opus review of the first write path: clean APPROVE. Under direct attack — opt-in gate unmounts POST routes (405/404 writes-off), auth wraps writes (401 no token), guard blocks traversal/absolute/symlink on add+reply+resolve (nothing written outside repo), write mutex held across the whole load-modify-save inside spawn_blocking never across await (40 concurrent adds, zero lost), attribution server-set who:human unforgeable, writes touch only .cospan sidecars (no kan claim, command_bus absent), write UI renders via textContent (script body inert). 213 tests + clippy -D warnings + fmt green.
***8<***
---
{
  "v": 3,
  "cid": "bafyreid7h3ujjqa4z7lrgzfyiezsb4krn657lh353xhcnob4aje25kqpgy",
  "sig": "d12728d81837af60a6b00f8eebb08d2b7b4679299fce85510cdf87b099e1cfa6615014128712439a6be7076be93fbcc90a86be27138bcafbe0662bb156f2a629",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comment-writes-over-api"
  },
  "kind": "observation",
  "cites": [
    "bafyreico3w6sgatm3lkznqx74dk7lhhhazjcnykii465ja6eq47lgpypj4"
  ],
  "rev": "223mud7ihpgec",
  "seq": 7,
  "of": 9,
  "text_len": 497,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgTt29IwJs2tWWwv/g1fWc5wZSJuFIRz3Ug8SHPrM/D09mYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx3Y29tbWVudC13cml0ZXMtb3Zlci1hcGlpYXJ0aWZhY3RzgaFmQ29tbWl0eChmM2FhNTM5M2U5NzIyY2RhY2Q3OTdlNmQ3NWRmOTNkM2VmYTI1YTIyaXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWklc2rDN"
}
---

Follow-up (pre-existing, cross-cutting, not Slice C): the read+write API surfaces core-level errors (guard rejection, unknown id, missing ?file=) as HTTP 200 with a {"error":...} body rather than a 4xx status — a convention inherited from the S5 core / Slice A. Clients must inspect the body, not the status. Consider mapping guard/not-found/bad-arg errors to real 4xx codes across /comments,/thread,/resolve in a later API-semantics pass (touches reads too, so out of scope for a single slice).
***8<***
---
{
  "v": 3,
  "cid": "bafyreigrj3ceykx7ubcjavo577msjtwakerr7idyqjghcrwvtm4ln3om6e",
  "sig": "75cdeebd0ccef7f3c2376bb8c2e51229c7eeb81af45f9ee72eafc0d9f8e2a12a44779344435394c910fa6397590d0d95f360e3273bd88f8b5b85f778ec483200",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comment-writes-over-api"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223mud7mghcax",
  "seq": 8,
  "of": 9,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx3Y29tbWVudC13cml0ZXMtb3Zlci1hcGlpYXJ0aWZhY3RzgaFmQ29tbWl0eChiM2E5YmU2ZGFhMWQzNjVlN2YwNGJhZmVmZjgyNWUzOWU5MmJkYmVmaXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWklkxqBg"
}
---
